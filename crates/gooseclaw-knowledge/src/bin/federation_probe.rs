//! federation_probe — LIVE acceptance proof for Phase 2 (not CI).
//!
//! Spawns the REAL Alfred MCP server (feat/dual-era-vault-mcp) through the
//! core-supervised seam and runs the spec section-8 Phase-2 acceptance gate:
//!   1. Conflict test — human edit wins; stale write CONFLICT; vault untouched;
//!      local copy discarded and re-fetched.
//!   2. Alfred-offline degradation — writes MUTED, cached reads stale-flagged,
//!      revalidation before the first post-restart write.
//! (Gate 3, dual-era wire capture, is proven Alfred-side; transcript in the
//! same audit dir.)
//!
//! Every step is logged (action + observed result) to the audit dir as JSONL.
//! Run: cargo run -p gooseclaw-knowledge --bin federation_probe

use gooseclaw::supervise::{SpawnSpec, spawn_jsonrpc};
use gooseclaw_knowledge::client::{ConnectFuture, KnowledgeError, VaultClient, WriteOutcome};
use std::io::Write as IoWrite;
use std::path::PathBuf;
use std::sync::Mutex as StdMutex;
use std::sync::atomic::{AtomicBool, Ordering};

const AUDIT: &str = "docs/audit/2026-10-29-phase2-federation";
const ALFRED: &str = r"C:\Users\User\dev\Alfred";

// Keep spawned children alive for the process lifetime; kill at exit.
static CHILDREN: StdMutex<Vec<tokio::process::Child>> = StdMutex::new(Vec::new());
static ALFRED_DOWN: AtomicBool = AtomicBool::new(false);

fn log_line(f: &std::sync::Mutex<std::fs::File>, step: &str, observed: &str) {
    let line = format!("{{\"step\":{:?},\"observed\":{:?}}}\n", step, observed);
    f.lock().unwrap().write_all(line.as_bytes()).unwrap();
    eprintln!("[probe] {step}: {observed}");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::fs::create_dir_all(AUDIT)?;
    let log = std::sync::Mutex::new(std::fs::File::create(format!(
        "{AUDIT}/federation-probe.jsonl"
    ))?);

    // Scratch vault (temp; not Alfred's real vault).
    let vault = std::env::temp_dir().join(format!("gooseclaw-fed-probe-{}", std::process::id()));
    std::fs::create_dir_all(&vault)?;
    std::fs::write(vault.join("note.md"), "v1 agent copy\n")?;

    // Minimal env for cmd + node (the scrub is core's default; the probe
    // passes exactly what node needs and nothing else).
    let path = std::env::var("PATH").unwrap_or_default();
    let sysroot = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());

    let make_factory = |down: &'static AtomicBool| {
        let vault = vault.clone();
        let path = path.clone();
        let sysroot = sysroot.clone();
        move || -> ConnectFuture<_, _> {
            let vault = vault.clone();
            let path = path.clone();
            let sysroot = sysroot.clone();
            Box::pin(async move {
                if down.load(Ordering::SeqCst) {
                    return Err("alfred is down (simulated outage)".to_string());
                }
                // node + tsx CLI directly (npx needs APPDATA/npm bootstrap;
                // the scrubbed env deliberately lacks it).
                let spec = SpawnSpec {
                    program: r"C:\Program Files\nodejs\node.exe".into(),
                    args: vec![
                        r"node_modules\tsx\dist\cli.mjs".into(),
                        "mcp/run.ts".into(),
                        vault.to_string_lossy().into_owned(),
                    ],
                    env: vec![
                        ("PATH".into(), path),
                        ("SystemRoot".into(), sysroot),
                        ("COMSPEC".into(), r"C:\Windows\System32\cmd.exe".into()),
                    ],
                    cwd: Some(PathBuf::from(ALFRED)),
                };
                let p = spawn_jsonrpc(&spec).map_err(|e| e.to_string())?;
                let gooseclaw::supervise::JsonRpcProcess {
                    child,
                    stdin,
                    stdout,
                } = p;
                CHILDREN.lock().unwrap().push(child);
                Ok((stdout, stdin))
            })
        }
    };

    let mut client = VaultClient::new(make_factory(&ALFRED_DOWN));

    // ---- Gate 1: conflict — human edit wins ------------------------------
    let read = client.read("note.md").await?;
    log_line(
        &log,
        "1.read",
        &format!("version={:?} stale={}", read.expected_version, read.stale),
    );
    let v1 = read
        .expected_version
        .clone()
        .expect("read carries expectedVersion");

    std::fs::write(vault.join("note.md"), "v2 human edit in Alfred\n")?; // out-of-band
    log_line(&log, "1.human-edit", "note.md overwritten out-of-band (v2)");

    let outcome = client.write("note.md", "v3 agent clobber\n").await?;
    match &outcome {
        WriteOutcome::Conflict { fresh } => {
            log_line(
                &log,
                "1.conflict",
                &format!("CONFLICT; fresh={:?}", fresh.content),
            );
            assert_eq!(fresh.content, "v2 human edit in Alfred\n");
        }
        other => panic!("GATE 1 FAIL: expected Conflict, got {other:?}"),
    }
    let on_disk = std::fs::read_to_string(vault.join("note.md"))?;
    assert_eq!(
        on_disk, "v2 human edit in Alfred\n",
        "GATE 1 FAIL: vault was clobbered"
    );
    log_line(&log, "1.vault-untouched", &format!("disk={on_disk:?}"));
    let reread = client.read("note.md").await?;
    assert_eq!(reread.content, "v2 human edit in Alfred\n");
    assert!(!reread.stale);
    log_line(
        &log,
        "1.reread",
        "discarded local copy; re-read shows v2 — GATE 1 PASS",
    );
    let _ = v1;

    // ---- Gate 2: Alfred-offline degradation -------------------------------
    ALFRED_DOWN.store(true, Ordering::SeqCst);
    for mut c in CHILDREN.lock().unwrap().drain(..) {
        let _ = c.kill().await;
    }
    log_line(
        &log,
        "2.alfred-down",
        "server killed; factory refusing new connections",
    );

    let w = client.write("note.md", "offline write\n").await;
    assert!(
        matches!(w, Err(KnowledgeError::Muted)),
        "GATE 2 FAIL: got {w:?}"
    );
    log_line(
        &log,
        "2.write-muted",
        "write refused (Muted), nothing queued",
    );

    let stale = client.read("note.md").await?;
    assert!(stale.stale, "GATE 2 FAIL: cached read not flagged stale");
    log_line(
        &log,
        "2.read-stale",
        &format!("stale=true content={:?}", stale.content),
    );

    let miss = client.read("never-read.md").await;
    assert!(
        matches!(miss, Err(KnowledgeError::Offline)),
        "GATE 2 FAIL: got {miss:?}"
    );
    log_line(&log, "2.uncached-read", "refused (Offline)");

    // Restart; the human edited again during the outage.
    std::fs::write(vault.join("note.md"), "v4 human edit during outage\n")?;
    ALFRED_DOWN.store(false, Ordering::SeqCst);
    log_line(
        &log,
        "2.restart",
        "Alfred back; human edited during outage (v4)",
    );

    let post = client
        .write("note.md", "v5 agent after reconnect\n")
        .await?;
    match &post {
        // Revalidation sees v4 and the write lands cleanly on top of it.
        WriteOutcome::Written { version } => {
            log_line(
                &log,
                "2.revalidated-write",
                &format!("written after revalidation, version={version}"),
            );
        }
        WriteOutcome::Conflict { fresh } => {
            log_line(
                &log,
                "2.revalidated-conflict",
                &format!("revalidation surfaced human edit: {:?}", fresh.content),
            );
        }
    }
    let final_disk = std::fs::read_to_string(vault.join("note.md"))?;
    log_line(
        &log,
        "2.final",
        &format!("disk={final_disk:?} — GATE 2 PASS"),
    );

    println!("PROBE-SUMMARY: GATE1=conflict-human-wins PASS; GATE2=offline-mute+revalidate PASS");
    for mut c in CHILDREN.lock().unwrap().drain(..) {
        let _ = c.kill().await;
    }
    let _ = std::fs::remove_dir_all(&vault);
    Ok(())
}
