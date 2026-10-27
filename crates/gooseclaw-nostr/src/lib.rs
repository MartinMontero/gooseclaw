//! gooseclaw-nostr — Nostr protocol layer.
//!
//! Phase 1: custody types and the fail-closed seam only — NO network code,
//! no rust-nostr dependency yet (rust-nostr is ALPHA: nostr 0.45.5 /
//! nostr-sdk 0.45.4 as of 2026-10-27; API breakage expected, so the
//! dependency lands behind this seam in the channels phase).
//!
//! NIP coverage targets (github.com/nostr-protocol/nips @ master, 2026-10-27):
//! NIP-17 private DMs (NIP-04 is DEPRECATED in favor of NIP-17 — never
//! implement NIP-04), NIP-44 versioned encryption, NIP-59 gift wrap,
//! NIP-46 remote signing (bunker), NIP-57 zaps, NIP-61 nutzaps,
//! NIP-32 labeling, NIP-56 reporting.

#![forbid(unsafe_code)]

use std::fmt;

/// A Nostr public key (32-byte x-only secp256k1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NostrPubKey(pub [u8; 32]);

/// Custody errors. The governing rule (CLAUDE.md Tier 1 #6): NIP-46 remote
/// signing only — never a raw nsec on this host — and bunker offline means
/// FAIL CLOSED. There is deliberately no local-key variant to fall back to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustodyError {
    /// The NIP-46 bunker is unreachable or refused. Operation aborted.
    BunkerOffline,
    /// The bunker returned a malformed signature.
    MalformedSignature,
}

impl fmt::Display for CustodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustodyError::BunkerOffline => write!(f, "NIP-46 bunker offline — failing closed"),
            CustodyError::MalformedSignature => write!(f, "bunker returned a malformed signature"),
        }
    }
}

impl std::error::Error for CustodyError {}

/// Handle to a NIP-46 remote signer (bunker). Phase 1: every operation fails
/// closed — there is no network client yet, and no code path that could ever
/// touch a local secret key.
#[derive(Debug, Clone)]
pub struct BunkerHandle {
    /// bunker:// or nostrconnect:// URI. Stored, never dialed in Phase 1.
    uri: String,
}

impl BunkerHandle {
    pub fn new(uri: impl Into<String>) -> Self {
        Self { uri: uri.into() }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Sign an event via the bunker. Phase 1: always fails closed.
    pub fn sign_event(&self, _unsigned_event_json: &[u8]) -> Result<[u8; 64], CustodyError> {
        Err(CustodyError::BunkerOffline)
    }
}

/// The NIPs this crate is chartered to implement.
pub const NIP_TARGETS: &[u16] = &[17, 32, 44, 46, 56, 57, 59, 61];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bunker_offline_fails_closed() {
        let b = BunkerHandle::new("bunker://example?relay=wss://relay.example");
        assert_eq!(b.sign_event(b"{}"), Err(CustodyError::BunkerOffline));
    }

    #[test]
    fn nip04_is_not_a_target() {
        assert!(
            !NIP_TARGETS.contains(&4),
            "NIP-04 is deprecated; use NIP-17"
        );
        for nip in [17, 44, 46, 57, 59, 61] {
            assert!(NIP_TARGETS.contains(&nip));
        }
    }
}
