//! Federation acceptance tests (PR-B), against an in-process fake Alfred MCP
//! server speaking the dual-era + expectedVersion wire contract. The live
//! end-to-end proofs against the REAL Alfred server are in
//! docs/audit/2026-10-29-phase2-federation/ (not CI — CI stays hermetic).

use gooseclaw_knowledge::client::{ConnectFuture, KnowledgeError, VaultClient, WriteOutcome};
use gooseclaw_knowledge::mcp::ServerEra;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, DuplexStream, ReadHalf, WriteHalf};
use tokio::task::JoinHandle;

type R = ReadHalf<DuplexStream>;
type W = WriteHalf<DuplexStream>;

#[derive(Default)]
struct FakeVault {
    notes: HashMap<String, (String, u64)>,
    ops: Vec<String>,
}

fn spawn_fake(vault: Arc<Mutex<FakeVault>>) -> (DuplexStream, JoinHandle<()>) {
    let (client_io, server_io) = tokio::io::duplex(1 << 16);
    let (r, mut w) = tokio::io::split(server_io);
    let handle = tokio::spawn(async move {
        let mut lines = BufReader::new(r).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let Ok(msg) = serde_json::from_str::<Value>(&line) else {
                continue;
            };
            let Some(id) = msg.get("id").cloned() else {
                continue;
            }; // notification
            let method = msg.get("method").and_then(Value::as_str).unwrap_or("");
            let resp = match method {
                "server/discover" => json!({"jsonrpc":"2.0","id":id,"result":{
                    "resultType":"complete","supportedVersions":["2025-11-25"],
                    "capabilities":{"tools":{}},"ttlMs":3600000,"cacheScope":"public"}}),
                "initialize" => json!({"jsonrpc":"2.0","id":id,"result":{
                    "protocolVersion":"2025-11-25","capabilities":{"tools":{}},
                    "serverInfo":{"name":"fake-alfred","version":"0.0.0"}}}),
                "tools/call" => {
                    let p = msg.get("params").cloned().unwrap_or(json!({}));
                    let name = p.get("name").and_then(Value::as_str).unwrap_or("");
                    let args = p.get("arguments").cloned().unwrap_or(json!({}));
                    let mut v = vault.lock().unwrap();
                    match name {
                        "vault_read" => {
                            let path = args["path"].as_str().unwrap_or("").to_string();
                            v.ops.push(format!("read {path}"));
                            match v.notes.get(&path) {
                                Some((content, ver)) => json!({"jsonrpc":"2.0","id":id,"result":{
                                    "content":[{"type":"text","text":serde_json::to_string(&json!({
                                        "path":path,"content":content,
                                        "expectedVersion":format!("v{ver}")})).unwrap()}]}}),
                                None => json!({"jsonrpc":"2.0","id":id,"result":{
                                    "content":[{"type":"text","text":"Error: not found"}],"isError":true}}),
                            }
                        }
                        "vault_write" => {
                            let path = args["path"].as_str().unwrap_or("").to_string();
                            let content = args["content"].as_str().unwrap_or("").to_string();
                            let expected =
                                args.get("expectedVersion").cloned().unwrap_or(Value::Null);
                            v.ops.push(format!("write {path}"));
                            let current = v.notes.get(&path).map(|(_, n)| format!("v{n}"));
                            let expected_s = match &expected {
                                Value::Null => None,
                                Value::String(s) => Some(s.clone()),
                                _ => None,
                            };
                            if current == expected_s {
                                let n = v.notes.get(&path).map(|(_, n)| n + 1).unwrap_or(1);
                                v.notes.insert(path.clone(), (content, n));
                                json!({"jsonrpc":"2.0","id":id,"result":{"content":[{"type":"text",
                                    "text":serde_json::to_string(&json!({"ok":true,"path":path,
                                    "version":format!("v{n}")})).unwrap()}]}})
                            } else {
                                json!({"jsonrpc":"2.0","id":id,"result":{"content":[{"type":"text",
                                    "text":serde_json::to_string(&json!({"ok":false,"error":"CONFLICT",
                                    "path":path,"currentVersion":current})).unwrap()}],"isError":true}})
                            }
                        }
                        _ => {
                            json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"unknown tool"}})
                        }
                    }
                }
                _ => continue,
            };
            if w.write_all(serde_json::to_string(&resp).unwrap().as_bytes())
                .await
                .is_err()
            {
                return;
            }
            if w.write_all(b"\n").await.is_err() {
                return;
            }
        }
    });
    (client_io, handle)
}

struct Rig {
    vault: Arc<Mutex<FakeVault>>,
    servers: Arc<Mutex<Vec<JoinHandle<()>>>>,
    down: Arc<std::sync::atomic::AtomicBool>,
}

impl Rig {
    fn new() -> (Self, VaultClient<R, W, impl FnMut() -> ConnectFuture<R, W>>) {
        let vault = Arc::new(Mutex::new(FakeVault::default()));
        let servers: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));
        let down = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let v2 = vault.clone();
        let s2 = servers.clone();
        let d2 = down.clone();
        let factory = move || -> ConnectFuture<R, W> {
            let v3 = v2.clone();
            let s3 = s2.clone();
            let d3 = d2.clone();
            Box::pin(async move {
                if d3.load(std::sync::atomic::Ordering::SeqCst) {
                    return Err("alfred is down".to_string());
                }
                let (io, h) = spawn_fake(v3);
                s3.lock().unwrap().push(h);
                let (r, w) = tokio::io::split(io);
                Ok((r, w))
            })
        };
        (
            Rig {
                vault,
                servers,
                down,
            },
            VaultClient::new(factory),
        )
    }

    fn seed(&self, path: &str, content: &str) {
        self.vault
            .lock()
            .unwrap()
            .notes
            .insert(path.into(), (content.into(), 1));
    }
    /// The human edits the vault directly (Alfred UI) — out-of-band.
    fn human_edit(&self, path: &str, content: &str) {
        let mut v = self.vault.lock().unwrap();
        let n = v.notes.get(path).map(|(_, n)| *n).unwrap_or(0);
        v.notes.insert(path.into(), (content.into(), n + 1));
    }
    fn ops(&self) -> Vec<String> {
        self.vault.lock().unwrap().ops.clone()
    }
    /// Alfred goes down: abort every live server task AND refuse new connections.
    fn kill_all(&self) {
        self.down.store(true, std::sync::atomic::Ordering::SeqCst);
        for h in self.servers.lock().unwrap().drain(..) {
            h.abort();
        }
    }
    /// Alfred restarts: new connections accepted again (state persisted).
    fn restart(&self) {
        self.down.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

#[tokio::test]
async fn era_probe_and_legacy_handshake() {
    let (_rig, mut client) = Rig::new();
    let r = client.read("hot.md").await; // hot.md missing is fine; handshake must succeed first
    // seed and read to force a full round trip
    _rig.seed("hot.md", "# hot");
    let res = client.read("hot.md").await.unwrap();
    assert_eq!(res.content, "# hot");
    assert!(!res.stale);
    let _ = r;
}

#[tokio::test]
async fn conflict_human_edit_wins() {
    let (rig, mut client) = Rig::new();
    rig.seed("note.md", "v1 agent copy");
    let first = client.read("note.md").await.unwrap();
    assert_eq!(first.expected_version.as_deref(), Some("v1"));

    // Human edits in Alfred (out-of-band) -> v2
    rig.human_edit("note.md", "v2 human edit");

    // Agent writes with the stale v1 token -> CONFLICT; vault keeps v2.
    let outcome = client.write("note.md", "v3 agent clobber").await.unwrap();
    match outcome {
        WriteOutcome::Conflict { fresh } => {
            assert_eq!(fresh.content, "v2 human edit");
            assert_eq!(fresh.expected_version.as_deref(), Some("v2"));
        }
        other => panic!("expected Conflict, got {other:?}"),
    }
    // No silent overwrite: the vault still holds the human edit...
    assert_eq!(
        rig.vault.lock().unwrap().notes["note.md"].0,
        "v2 human edit"
    );
    // ...and the client's re-read (cache was discarded) shows v2.
    let reread = client.read("note.md").await.unwrap();
    assert_eq!(reread.content, "v2 human edit");
    assert!(!reread.stale);
}

#[tokio::test]
async fn offline_mute_and_stale_reads() {
    let (rig, mut client) = Rig::new();
    rig.seed("cached.md", "cached content");
    let r = client.read("cached.md").await.unwrap();
    assert!(!r.stale);

    rig.kill_all(); // Alfred down
    tokio::task::yield_now().await;

    // Write while offline: MUTED, never queued.
    let w = client.write("cached.md", "offline write").await;
    assert!(matches!(w, Err(KnowledgeError::Muted)), "got {w:?}");
    assert!(client.is_offline());

    // Cached read comes back flagged stale.
    let stale_read = client.read("cached.md").await.unwrap();
    assert!(stale_read.stale);
    assert_eq!(stale_read.content, "cached content");

    // Uncached read while offline: refused.
    let miss = client.read("never-read.md").await;
    assert!(matches!(miss, Err(KnowledgeError::Offline)), "got {miss:?}");

    // The vault is untouched by the muted write.
    assert_eq!(
        rig.vault.lock().unwrap().notes["cached.md"].0,
        "cached content"
    );
}

#[tokio::test]
async fn reconnect_revalidates_before_write() {
    let (rig, mut client) = Rig::new();
    rig.seed("n.md", "v1");
    client.read("n.md").await.unwrap();

    rig.kill_all();
    tokio::task::yield_now().await;
    assert!(matches!(
        client.write("n.md", "x").await,
        Err(KnowledgeError::Muted)
    ));

    // Human edits while we are offline; then Alfred restarts.
    rig.human_edit("n.md", "v2 human-while-offline");
    rig.restart();

    // First write after reconnect must revalidate (a read precedes the write on the wire).
    let before = rig.ops().len();
    let outcome = client.write("n.md", "v3 after-reconnect").await.unwrap();
    let ops = rig.ops();
    let new_ops = &ops[before.min(ops.len())..];
    assert!(
        new_ops.iter().any(|o| o.starts_with("read")),
        "revalidation read missing: {new_ops:?}"
    );
    match outcome {
        WriteOutcome::Written { version } => {
            // Revalidation picked up the human's v2; our write landed on top as v3.
            assert_eq!(version, "v3");
        }
        other => panic!("expected Written after revalidation, got {other:?}"),
    }
    assert_eq!(
        rig.vault.lock().unwrap().notes["n.md"].0,
        "v3 after-reconnect"
    );
}

#[tokio::test]
async fn ttl_serves_fresh_reads_from_cache() {
    let (rig, mut client) = Rig::new();
    rig.seed("t.md", "ttl test");
    client.read("t.md").await.unwrap();
    let reads_before = rig.ops().iter().filter(|o| o.starts_with("read")).count();
    let second = client.read("t.md").await.unwrap(); // within TTL -> cache
    assert!(!second.stale);
    let reads_after = rig.ops().iter().filter(|o| o.starts_with("read")).count();
    assert_eq!(
        reads_before, reads_after,
        "second read hit the server (TTL cache failed)"
    );
}

#[tokio::test]
async fn era_is_detected_as_dual_era_aware() {
    // Connect directly to inspect the negotiated era.
    let vault = Arc::new(Mutex::new(FakeVault::default()));
    let (io, _h) = spawn_fake(vault);
    let (r, w) = tokio::io::split(io);
    let conn = gooseclaw_knowledge::mcp::McpConnection::connect(r, w)
        .await
        .unwrap();
    assert_eq!(conn.negotiated_version, "2025-11-25");
    assert!(matches!(conn.era, ServerEra::DualEraAware { .. }));
}
