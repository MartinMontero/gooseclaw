//! Durable session state (core-owned).
//!
//! goose ACP has no session resume across processes; GooseClaw owns the
//! durable record. Phase 1: append-only JSONL on local disk. No telemetry,
//! no network — sovereignty invariant (CLAUDE.md Tier 1 #7).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionEventKind {
    SessionCreated,
    PromptSent,
    ToolCallRequested,
    GateDecision,
    TurnComplete,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub at_unix: u64,
    pub kind: SessionEventKind,
    /// Sanitized, redacted detail. Never raw secrets, never unstripped text.
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub session_id: String,
    pub created_at_unix: u64,
    pub events: Vec<SessionEvent>,
}

/// Append-only JSONL store rooted at a local directory.
pub struct JsonlSessionStore {
    root: PathBuf,
}

impl JsonlSessionStore {
    pub fn new(root: impl Into<PathBuf>) -> std::io::Result<Self> {
        let root = root.into();
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    fn path_for(&self, session_id: &str) -> PathBuf {
        // session_id is core-generated (hex/uuid); never attacker input.
        self.root.join(format!("{session_id}.jsonl"))
    }

    pub fn append(&self, session_id: &str, event: &SessionEvent) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path_for(session_id))?;
        let mut line = serde_json::to_string(event)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        line.push('\n');
        f.write_all(line.as_bytes())
    }

    pub fn load(&self, session_id: &str) -> std::io::Result<Vec<SessionEvent>> {
        let path = self.path_for(session_id);
        if !Path::new(&path).exists() {
            return Ok(Vec::new());
        }
        let f = BufReader::new(std::fs::File::open(path)?);
        let mut out = Vec::new();
        for line in f.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            out.push(
                serde_json::from_str(&line)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
            );
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jsonl_roundtrip() {
        let dir = std::env::temp_dir().join(format!("gooseclaw-sess-{}", std::process::id()));
        let store = JsonlSessionStore::new(&dir).unwrap();
        let ev = SessionEvent {
            at_unix: 1,
            kind: SessionEventKind::SessionCreated,
            detail: "created".into(),
        };
        store.append("abc123", &ev).unwrap();
        store.append("abc123", &ev).unwrap();
        let loaded = store.load("abc123").unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].kind, SessionEventKind::SessionCreated);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
