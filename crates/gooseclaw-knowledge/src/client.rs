//! The Alfred MCP-vault federation client (Phase 2).
//!
//! Contract (canon 3.2, Option C): Alfred is the system-of-record and SOLE
//! WRITER. GooseClaw reads free within the path root and writes only through
//! Alfred's MCP tools, with expectedVersion from the read that produced the
//! local copy. On CONFLICT the local copy is DISCARDED and re-fetched, never
//! merged (human-edit-wins).
//!
//! O5 — cache staleness + reconnect semantics (specified HERE, 2026-10-29):
//!  - TTL: a cache entry is fresh for CACHE_TTL (60s) after a successful read;
//!    past that it is re-fetched when online.
//!  - Alfred OFFLINE: writes go MUTE — refused with KnowledgeError::Muted,
//!    nothing queued, nothing written to disk by this client. Reads are served
//!    from cache only, always flagged stale=true to the caller.
//!  - RECONNECT: the first write after a reconnect revalidates — the client
//!    re-reads the note and uses the SERVER's current version for the write;
//!    if the human edited while we were away, that edit wins (CONFLICT path).
//!
//! The MCP connection is established through a caller-supplied connect factory
//! (production: gooseclaw::supervise::spawn_jsonrpc — core-owned, never
//! ambient; tests: in-process duplex against a fake server).

#![forbid(unsafe_code)]

use crate::mcp::{McpConnection, McpError};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use tokio::io::{AsyncRead, AsyncWrite};

/// O5 staleness bound: a cached note is fresh for this long after a read.
/// Chosen: 60s — short enough that an always-on agent re-reads often, long
/// enough to absorb bursty read patterns. Documented in spec O5 closure.
pub const CACHE_TTL: Duration = Duration::from_secs(60);

#[derive(Debug)]
pub enum KnowledgeError {
    /// Alfred is unreachable AND the note is not in cache.
    Offline,
    /// A write was attempted while Alfred is offline. Mute = refused, not queued.
    Muted,
    /// A write was attempted without a prior read (no expectedVersion to offer).
    MustReadFirst(String),
    Transport(String),
    Protocol(String),
}

impl fmt::Display for KnowledgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KnowledgeError::Offline => write!(f, "Alfred offline; note not in cache"),
            KnowledgeError::Muted => write!(f, "Alfred offline; write muted (never queued)"),
            KnowledgeError::MustReadFirst(p) => {
                write!(f, "no cached version for {p}; read before write")
            }
            KnowledgeError::Transport(e) => write!(f, "transport: {e}"),
            KnowledgeError::Protocol(e) => write!(f, "protocol: {e}"),
        }
    }
}
impl std::error::Error for KnowledgeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadResult {
    pub path: String,
    pub content: String,
    /// The server's optimistic-concurrency token for this content.
    pub expected_version: Option<String>,
    /// True when served from cache (past TTL or while offline). Callers MUST
    /// surface this; stale data is never silently presented as fresh.
    pub stale: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteOutcome {
    /// Write accepted; carries the new version token.
    Written { version: String },
    /// CONFLICT: our expectedVersion was stale. The local copy was discarded
    /// and re-fetched; this carries the fresh server state (the human's edit).
    Conflict { fresh: ReadResult },
}

#[derive(Debug, Clone)]
struct CacheEntry {
    content: String,
    version: Option<String>,
    fetched_at: Instant,
}

impl CacheEntry {
    fn fresh(&self) -> bool {
        self.fetched_at.elapsed() < CACHE_TTL
    }
}

/// The federation client. Generic over the connection factory so tests inject
/// an in-process server and production injects the core-supervised subprocess.
pub struct VaultClient<R, W, F>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
    F: FnMut() -> crate::client::ConnectFuture<R, W>,
{
    connect: F,
    conn: Option<McpConnection<R, W>>,
    cache: HashMap<String, CacheEntry>,
    offline: bool,
    /// Set when a connection is (re)established after an outage: the next
    /// write must revalidate against the server first (O5).
    needs_revalidation: bool,
}

/// Boxed connect future (the factory may spawn a process).
pub type ConnectFuture<R, W> =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<(R, W), String>> + Send>>;

impl<R, W, F> VaultClient<R, W, F>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
    F: FnMut() -> ConnectFuture<R, W>,
{
    pub fn new(connect: F) -> Self {
        Self {
            connect,
            conn: None,
            cache: HashMap::new(),
            offline: false,
            needs_revalidation: false,
        }
    }

    pub fn is_offline(&self) -> bool {
        self.offline
    }

    /// Ensure a live connection, attempting (re)connect when offline/absent.
    async fn ensure_conn(&mut self) -> Result<&mut McpConnection<R, W>, KnowledgeError> {
        if self.offline || self.conn.is_none() {
            let (r, w) = (self.connect)().await.map_err(KnowledgeError::Transport)?;
            let conn = McpConnection::connect(r, w)
                .await
                .map_err(|e| KnowledgeError::Protocol(e.to_string()))?;
            let was_offline = self.offline;
            self.conn = Some(conn);
            self.offline = false;
            if was_offline {
                self.needs_revalidation = true;
            }
        }
        Ok(self.conn.as_mut().expect("connection established above"))
    }

    fn mark_offline(&mut self) {
        self.offline = true;
        self.conn = None; // the dead connection is dropped (kill_on_drop upstream)
    }

    /// If offline, attempt one lazy reconnect. Success flips us online and
    /// arms needs_revalidation (O5).
    async fn maybe_reconnect(&mut self) {
        if !self.offline {
            return;
        }
        match self.ensure_conn().await {
            Ok(_) => {}
            Err(_) => self.mark_offline(),
        }
    }

    /// Read a note. Online: fresh cache (< TTL) served directly, else fetched.
    /// Offline: cache only, always stale-flagged.
    pub async fn read(&mut self, path: &str) -> Result<ReadResult, KnowledgeError> {
        self.maybe_reconnect().await;
        if !self.offline {
            if let Some(e) = self.cache.get(path)
                && e.fresh()
            {
                return Ok(ReadResult {
                    path: path.into(),
                    content: e.content.clone(),
                    expected_version: e.version.clone(),
                    stale: false,
                });
            }
            match self.read_live(path).await {
                Ok(r) => return Ok(r),
                Err(e) => {
                    self.mark_offline();
                    if let Some(cached) = self.cached_stale(path) {
                        return Ok(cached);
                    }
                    return Err(e);
                }
            }
        }
        self.cached_stale(path).ok_or(KnowledgeError::Offline)
    }

    fn cached_stale(&self, path: &str) -> Option<ReadResult> {
        self.cache.get(path).map(|e| ReadResult {
            path: path.into(),
            content: e.content.clone(),
            expected_version: e.version.clone(),
            stale: true,
        })
    }

    async fn read_live(&mut self, path: &str) -> Result<ReadResult, KnowledgeError> {
        let conn = self.ensure_conn().await?;
        let (text, is_error) = conn
            .call_tool("vault_read", json!({ "path": path }))
            .await
            .map_err(conn_err)?;
        if is_error {
            return Err(KnowledgeError::Protocol(format!(
                "vault_read refused: {text}"
            )));
        }
        let body: Value =
            serde_json::from_str(&text).map_err(|e| KnowledgeError::Protocol(e.to_string()))?;
        let r = ReadResult {
            path: path.into(),
            content: body
                .get("content")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            expected_version: body
                .get("expectedVersion")
                .and_then(Value::as_str)
                .map(String::from),
            stale: false,
        };
        self.cache.insert(
            path.into(),
            CacheEntry {
                content: r.content.clone(),
                version: r.expected_version.clone(),
                fetched_at: Instant::now(),
            },
        );
        Ok(r)
    }

    /// Write through Alfred's vault_write tool with the expectedVersion from
    /// the read that produced the local copy. CONFLICT -> discard + re-fetch.
    pub async fn write(
        &mut self,
        path: &str,
        content: &str,
    ) -> Result<WriteOutcome, KnowledgeError> {
        self.maybe_reconnect().await;
        if self.offline {
            return Err(KnowledgeError::Muted); // MUTE: no queue, no disk
        }
        // O5 reconnect rule: after an outage, revalidate before writing.
        if self.needs_revalidation {
            self.read_live(path).await?; // refreshes cache to the server's truth
            self.needs_revalidation = false;
        }
        let expected = match self.cache.get(path) {
            Some(e) => e.version.clone(),
            None => return Err(KnowledgeError::MustReadFirst(path.into())),
        };
        let result = self.try_write(path, content, expected).await;
        match result {
            Ok(outcome) => Ok(outcome),
            Err(e) => {
                self.mark_offline();
                match e {
                    // The connection died mid-write: the write did not happen
                    // and nothing is queued — that IS a mute.
                    KnowledgeError::Transport(_) => Err(KnowledgeError::Muted),
                    other => Err(other),
                }
            }
        }
    }

    async fn try_write(
        &mut self,
        path: &str,
        content: &str,
        expected: Option<String>,
    ) -> Result<WriteOutcome, KnowledgeError> {
        let conn = self.ensure_conn().await?;
        let (text, is_error) = conn
            .call_tool(
                "vault_write",
                json!({ "path": path, "content": content, "expectedVersion": expected }),
            )
            .await
            .map_err(conn_err)?;
        let body: Value =
            serde_json::from_str(&text).map_err(|e| KnowledgeError::Protocol(e.to_string()))?;
        if is_error {
            let is_conflict = body.get("error").and_then(Value::as_str) == Some("CONFLICT")
                || text.contains("CONFLICT");
            if is_conflict {
                // Human-edit-wins: discard the local copy, re-fetch, never merge.
                self.cache.remove(path);
                let fresh = self.read_live(path).await?;
                return Ok(WriteOutcome::Conflict { fresh });
            }
            return Err(KnowledgeError::Protocol(format!(
                "vault_write refused: {text}"
            )));
        }
        let version = body
            .get("version")
            .and_then(Value::as_str)
            .ok_or_else(|| KnowledgeError::Protocol("write ok but no version returned".into()))?
            .to_string();
        self.cache.insert(
            path.into(),
            CacheEntry {
                content: content.to_string(),
                version: Some(version.clone()),
                fetched_at: Instant::now(),
            },
        );
        Ok(WriteOutcome::Written { version })
    }
}

fn conn_err(e: McpError) -> KnowledgeError {
    match e {
        McpError::Transport(m) => KnowledgeError::Transport(m),
        McpError::Timeout => KnowledgeError::Transport("timeout".into()),
        other => KnowledgeError::Protocol(other.to_string()),
    }
}
