//! gooseclaw — core crate.
//!
//! Owns, and no other crate may own:
//!   1. The ACP supervisor (spawn/monitor the goose acp subprocess).
//!   2. THE Rule-9 gate (gate.rs) — the single decision point (AMENDMENT 2).
//!   3. The scheduler (stub — proactivity is Phase 6; no autonomy now).
//!   4. Durable session state (session.rs) — goose ACP has no resume.
//!
//! Contracts:
//!   goose <-> GooseClaw: ACP over stdio, agent-client-protocol =2.0.0,
//!   protocolVersion = 1, GOOSE_MODE=approve forced at spawn.
//!   Alfred <-> GooseClaw: MCP vault federation (Phase 2, not built).

#![forbid(unsafe_code)]

pub mod gate;
pub mod session;

pub use gate::{
    ActionHash, ActionRequest, Approval, AuditRecord, Decision, GateCaller, GateError,
    PendingAction, Rule9Gate,
};
pub use session::{JsonlSessionStore, SessionEvent, SessionEventKind, SessionRecord};

/// Spawn configuration for the supervised goose ACP subprocess.
/// The supervisor builds a SCRUBBED environment: GOOSE_MODE=approve is set
/// unconditionally (CLAUDE.md: do not assume a flag; the mode mechanism is
/// the env var, verified in block/goose config-files.md @ v1.48.0), and
/// GOOSE_TELEMETRY_ENABLED=false is forced (sovereignty: zero telemetry).
pub struct AcpSpawnConfig {
    /// Path to the goose binary (resolved at runtime).
    pub goose_bin: std::path::PathBuf,
    /// Extra env vars allowed through (provider config etc.).
    pub passthrough_env: Vec<(String, String)>,
}

impl AcpSpawnConfig {
    /// The env pairs the supervisor MUST force. Callers may add provider
    /// vars via passthrough_env but can never override these two.
    pub fn forced_env() -> [(String, String); 2] {
        [
            ("GOOSE_MODE".into(), "approve".into()),
            ("GOOSE_TELEMETRY_ENABLED".into(), "false".into()),
        ]
    }
}

/// The scheduler exists so channels/triggers have a seam to hang on, but
/// Phase 1 runs NOTHING unattended (session rule 6; proactivity is Phase 6).
pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Self
    }
    /// Always empty in Phase 1: no autonomous jobs exist yet.
    pub fn pending_jobs(&self) -> usize {
        0
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forced_env_is_approve_and_no_telemetry() {
        let env = AcpSpawnConfig::forced_env();
        assert!(env.contains(&("GOOSE_MODE".to_string(), "approve".to_string())));
        assert!(env.contains(&("GOOSE_TELEMETRY_ENABLED".to_string(), "false".to_string())));
    }

    #[test]
    fn scheduler_runs_nothing_in_phase_1() {
        assert_eq!(Scheduler::new().pending_jobs(), 0);
    }
}
