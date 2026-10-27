//! gooseclaw-skills — Wasmtime component-model skill sandbox.
//!
//! Phase 1: trust tiers, capability leases, and the sandbox trait seam only.
//! The Wasmtime dependency lands when the first real skill executes
//! (IronClaw main pins wasmtime 43.0.2; current upstream is v50.0.0 as of
//! 2026-10-20 — re-pin against primary source at implementation time per
//! DRIFT RULE).
//!
//! Design derived from IronClaw's crates/lanes/ironclaw_wasm,
//! ironclaw_wasm_limiter, and ironclaw_sandbox (MIT OR Apache-2.0 — see
//! NOTICE). Note: wasm_sandbox_core is a MODULE inside IronClaw's
//! ironclaw_wasm crate, not a standalone crate.

#![forbid(unsafe_code)]

/// Trust tier of an installed skill (IronClaw Trusted/Installed model).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustTier {
    /// Shipped with GooseClaw, reviewed — still capability-leased.
    Trusted,
    /// Third-party. Sandbox + deny-by-default capabilities.
    Installed,
}

/// A capability that MAY be leased to a skill. Deny-by-default: a skill
/// holds exactly the capabilities it was leased, nothing more.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    HttpOut,
    ReadFs,
    WriteFs,
    NostrPublish,
    Spend,
}

/// A time-bounded grant of one capability to one skill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityLease {
    pub skill: String,
    pub capability: Capability,
    /// Unix expiry; the sandbox re-checks on every host call.
    pub expires_at_unix: u64,
}

/// Deny-by-default capability lookup over a lease set.
pub struct LeaseBook {
    leases: Vec<CapabilityLease>,
}

impl LeaseBook {
    pub fn new(leases: Vec<CapabilityLease>) -> Self {
        Self { leases }
    }

    /// True only if an unexpired lease for (skill, capability) exists.
    pub fn allows(&self, skill: &str, capability: Capability, now_unix: u64) -> bool {
        self.leases
            .iter()
            .any(|l| l.skill == skill && l.capability == capability && l.expires_at_unix > now_unix)
    }
}

/// The sandbox seam. Implementations: a Wasmtime component-model runtime
/// with a store limiter (fuel/epoch interruption + memory caps).
/// Phase 1: trait only.
pub trait WasmSandbox: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Run one skill invocation. Every host call the guest makes is checked
    /// against the LeaseBook.
    fn invoke(&mut self, skill: &str, tier: TrustTier, input: &str) -> Result<String, Self::Error>;
}

/// The WIT world every GooseClaw skill targets (documented; codegen lands
/// with the Wasmtime dependency):
///
/// ```text
/// package gooseclaw:skill@0.1.0;
/// interface host {
///     audit-log: func(message: string);
///     // Every other host capability is leased and checked per call.
/// }
/// world gooseclaw-skill {
///     import host;
///     export run: func(input: string) -> result<string, string>;
/// }
/// ```
pub const WIT_WORLD: &str = "gooseclaw:skill@0.1.0/gooseclaw-skill";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leases_are_deny_by_default_and_expiry_bound() {
        let book = LeaseBook::new(vec![CapabilityLease {
            skill: "webfetch".into(),
            capability: Capability::HttpOut,
            expires_at_unix: 1000,
        }]);
        assert!(book.allows("webfetch", Capability::HttpOut, 999));
        assert!(
            !book.allows("webfetch", Capability::HttpOut, 1000),
            "expired"
        );
        assert!(
            !book.allows("webfetch", Capability::Spend, 999),
            "never leased"
        );
        assert!(
            !book.allows("other", Capability::HttpOut, 999),
            "other skill"
        );
    }
}
