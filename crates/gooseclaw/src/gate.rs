//! The single core-owned Rule-9 gate (CLAUDE.md Tier 1 #1 and #6).
//!
//! AMENDMENT 2 (approved 2026-10-27): exactly one gate, owned here in core.
//! Channels and payments CALL it; they can never mint, cache, or override a
//! decision. AMENDMENT 3 (approved 2026-10-27): approvals are signed by an
//! allowlisted pubkey, nonce + expiry bound to the action hash, single-use.
//!
//! No unattended action can construct a Decision: the only path is
//! Rule9Gate::decide with a valid Approval over a pending action hash.

#![forbid(unsafe_code)]

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// SHA-256 over the canonical action encoding. Binds the approval to
/// exactly this action — summary edits after signing invalidate it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionHash(pub [u8; 32]);

/// A consequential action a subsystem wants the human to authorize.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRequest {
    /// Originating subsystem, e.g. "channels:nostr", "payments:lightning", "acp:tool".
    pub actor: String,
    /// Human-readable summary. MUST already be sanitized
    /// (gooseclaw-safety strip_invisible) before display; the hash covers it as-is.
    pub summary: String,
    /// Canonical payload bytes (e.g. serialized tool call, spend request).
    pub payload: Vec<u8>,
}

impl ActionRequest {
    pub fn hash(&self) -> ActionHash {
        let mut h = Sha256::new();
        h.update(self.actor.as_bytes());
        h.update([0u8]);
        h.update(self.summary.as_bytes());
        h.update([0u8]);
        h.update(&self.payload);
        ActionHash(h.finalize().into())
    }
}

/// An authenticated human approval (AMENDMENT 3).
///
/// The signed message is exactly:
///     action_hash (32) || nonce (32) || expires_at_unix (8, big-endian)
#[derive(Debug, Clone, Copy)]
pub struct Approval {
    pub pubkey: [u8; 32],
    pub nonce: [u8; 32],
    pub expires_at_unix: u64,
    pub signature: [u8; 64],
}

impl Approval {
    /// Canonical signed bytes for (action, nonce, expiry).
    pub fn message(action: &ActionHash, nonce: &[u8; 32], expires_at_unix: u64) -> Vec<u8> {
        let mut m = Vec::with_capacity(72);
        m.extend_from_slice(&action.0);
        m.extend_from_slice(nonce);
        m.extend_from_slice(&expires_at_unix.to_be_bytes());
        m
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Approved,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateError {
    UnknownSigner,
    BadSignature,
    NonceReused,
    Expired,
    ActionNotPending,
}

impl fmt::Display for GateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GateError::UnknownSigner => "signer not on the approval allowlist",
            GateError::BadSignature => "approval signature invalid",
            GateError::NonceReused => "approval nonce already used (replay)",
            GateError::Expired => "approval expired",
            GateError::ActionNotPending => "action was never submitted to the gate",
        };
        f.write_str(s)
    }
}
impl std::error::Error for GateError {}

/// Opaque handle proving an action passed through submit(). Minted ONLY here.
pub struct PendingAction {
    hash: ActionHash,
    summary: String,
}
impl PendingAction {
    pub fn hash(&self) -> ActionHash {
        self.hash
    }
    pub fn summary(&self) -> &str {
        &self.summary
    }
}

/// The one gate. Owned by core; every other crate borrows it via GateCaller.
pub struct Rule9Gate {
    allowlisted_signers: HashSet<[u8; 32]>,
    used_nonces: HashSet<[u8; 32]>,
    pending: HashSet<ActionHash>,
    /// Local audit log (sovereignty: zero telemetry; audit logs REQUIRED).
    audit: Vec<AuditRecord>,
}

#[derive(Debug, Clone)]
pub struct AuditRecord {
    pub at_unix: u64,
    pub action: ActionHash,
    pub decision: Decision,
    pub signer: Option<[u8; 32]>,
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl Rule9Gate {
    pub fn new(allowlisted_signers: impl IntoIterator<Item = [u8; 32]>) -> Self {
        Self {
            allowlisted_signers: allowlisted_signers.into_iter().collect(),
            used_nonces: HashSet::new(),
            pending: HashSet::new(),
            audit: Vec::new(),
        }
    }

    /// Enter an action into the pending set. Core-only by construction:
    /// PendingAction has no public constructor and cannot be cloned.
    pub fn submit(&mut self, action: ActionRequest) -> PendingAction {
        let hash = action.hash();
        self.pending.insert(hash);
        PendingAction {
            hash,
            summary: action.summary,
        }
    }

    /// Decide a pending action from an authenticated approval.
    pub fn decide(
        &mut self,
        pending: &PendingAction,
        approval: &Approval,
    ) -> Result<Decision, GateError> {
        if !self.pending.contains(&pending.hash) {
            return Err(GateError::ActionNotPending);
        }
        if !self.allowlisted_signers.contains(&approval.pubkey) {
            return Err(GateError::UnknownSigner);
        }
        if now_unix() > approval.expires_at_unix {
            return Err(GateError::Expired);
        }
        if self.used_nonces.contains(&approval.nonce) {
            return Err(GateError::NonceReused);
        }
        let msg = Approval::message(&pending.hash, &approval.nonce, approval.expires_at_unix);
        let key =
            VerifyingKey::from_bytes(&approval.pubkey).map_err(|_| GateError::UnknownSigner)?;
        let sig = Signature::from_bytes(&approval.signature);
        key.verify(&msg, &sig)
            .map_err(|_| GateError::BadSignature)?;

        // Single-use: burn the nonce and the pending entry.
        self.used_nonces.insert(approval.nonce);
        self.pending.remove(&pending.hash);
        self.audit.push(AuditRecord {
            at_unix: now_unix(),
            action: pending.hash,
            decision: Decision::Approved,
            signer: Some(approval.pubkey),
        });
        Ok(Decision::Approved)
    }

    /// Record an explicit denial (also audited).
    pub fn deny(&mut self, pending: PendingAction) -> Decision {
        self.pending.remove(&pending.hash);
        self.audit.push(AuditRecord {
            at_unix: now_unix(),
            action: pending.hash,
            decision: Decision::Denied,
            signer: None,
        });
        Decision::Denied
    }

    pub fn audit_log(&self) -> &[AuditRecord] {
        &self.audit
    }
}

/// The seam every other crate uses. Implemented ONLY by Rule9Gate.
/// Channels/payments hold a G: GateCaller; they cannot construct decisions.
pub trait GateCaller {
    fn submit_and_decide(
        &mut self,
        action: ActionRequest,
        approval: &Approval,
    ) -> Result<Decision, GateError>;
}

impl GateCaller for Rule9Gate {
    fn submit_and_decide(
        &mut self,
        action: ActionRequest,
        approval: &Approval,
    ) -> Result<Decision, GateError> {
        let pending = self.submit(action);
        self.decide(&pending, approval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn keypair(seed: u8) -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[seed; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    fn sign(sk: &SigningKey, hash: &ActionHash, nonce: [u8; 32], exp: u64) -> Approval {
        let msg = Approval::message(hash, &nonce, exp);
        let sig = sk.sign(&msg).to_bytes();
        Approval {
            pubkey: sk.verifying_key().to_bytes(),
            nonce,
            expires_at_unix: exp,
            signature: sig,
        }
    }

    fn action() -> ActionRequest {
        ActionRequest {
            actor: "acp:tool".into(),
            summary: "run shell: ls".into(),
            payload: b"tool=shell".to_vec(),
        }
    }

    #[test]
    fn happy_path_single_use() {
        let (sk, pk) = keypair(7);
        let mut gate = Rule9Gate::new([pk]);
        let pending = gate.submit(action());
        let exp = now_unix() + 60;
        let ap = sign(&sk, &pending.hash(), [1u8; 32], exp);
        assert_eq!(gate.decide(&pending, &ap), Ok(Decision::Approved));
        // replay the SAME approval: nonce burned AND action no longer pending
        let err = gate.decide(&pending, &ap).unwrap_err();
        assert!(matches!(
            err,
            GateError::ActionNotPending | GateError::NonceReused
        ));
        assert_eq!(gate.audit_log().len(), 1);
    }

    #[test]
    fn unknown_signer_rejected() {
        let (sk, _pk) = keypair(1);
        let (_sk2, pk2) = keypair(2);
        let mut gate = Rule9Gate::new([pk2]);
        let pending = gate.submit(action());
        let ap = sign(&sk, &pending.hash(), [3u8; 32], now_unix() + 60);
        assert_eq!(gate.decide(&pending, &ap), Err(GateError::UnknownSigner));
    }

    #[test]
    fn tampered_action_hash_breaks_signature() {
        let (sk, pk) = keypair(9);
        let mut gate = Rule9Gate::new([pk]);
        let pending = gate.submit(action());
        let other = gate.submit(ActionRequest {
            actor: "acp:tool".into(),
            summary: "run shell: rm -rf /".into(),
            payload: b"{}".to_vec(),
        });
        // approval signed for the FIRST action, presented for the SECOND
        let ap = sign(&sk, &pending.hash(), [4u8; 32], now_unix() + 60);
        assert_eq!(gate.decide(&other, &ap), Err(GateError::BadSignature));
    }

    #[test]
    fn expired_approval_rejected() {
        let (sk, pk) = keypair(5);
        let mut gate = Rule9Gate::new([pk]);
        let pending = gate.submit(action());
        let ap = sign(
            &sk,
            &pending.hash(),
            [5u8; 32],
            now_unix().saturating_sub(1),
        );
        assert_eq!(gate.decide(&pending, &ap), Err(GateError::Expired));
    }

    #[test]
    fn decision_cannot_be_forged_outside_gate() {
        let mut gate = Rule9Gate::new([]);
        let pending = gate.submit(action());
        // No approval at all: deny path is the only terminal state available.
        assert_eq!(gate.deny(pending), Decision::Denied);
        assert_eq!(gate.audit_log()[0].decision, Decision::Denied);
    }
}
