//! gooseclaw-payments — Lightning + Cashu P2PK (NUT-11) + NIP-57/61.
//!
//! Phase 1: spend-request types, caps, and the gate seam. No rails wired.
//! AMENDMENT 2 (approved 2026-10-27): every spend CALLS the core gate via
//! GateCaller. Caps are a pre-gate filter, not an approval path — an
//! under-cap spend still requires a signed, single-use human approval.

#![forbid(unsafe_code)]

use gooseclaw::gate::{ActionRequest, Approval, Decision, GateCaller, GateError};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rail {
    Lightning,
    /// Cashu P2PK-locked ecash (NUT-11), e.g. NIP-61 nutzaps.
    CashuP2pk,
    /// NIP-57 lightning zap.
    Zap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpendRequest {
    pub amount_msat: u64,
    pub rail: Rail,
    pub destination: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy)]
pub struct SpendingCaps {
    pub per_spend_msat: u64,
    pub daily_msat: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpendError {
    OverCap,
    Gate(GateError),
}

impl fmt::Display for SpendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpendError::OverCap => write!(f, "spend exceeds configured cap"),
            SpendError::Gate(e) => write!(f, "gate: {e}"),
        }
    }
}
impl std::error::Error for SpendError {}

/// Payments facade. Holds NO approval authority — only caps + the gate call.
pub struct PaymentGate<G: GateCaller> {
    pub caps: SpendingCaps,
    pub gate: G,
}

impl<G: GateCaller> PaymentGate<G> {
    /// Pre-gate cap check, then delegate the decision to the core gate.
    pub fn request_spend(
        &mut self,
        req: SpendRequest,
        approval: &Approval,
    ) -> Result<Decision, SpendError> {
        if req.amount_msat > self.caps.per_spend_msat {
            return Err(SpendError::OverCap);
        }
        let action = ActionRequest {
            actor: format!("payments:{:?}", req.rail).to_lowercase(),
            summary: format!(
                "spend {} msat to {} ({})",
                req.amount_msat, req.destination, req.reason
            ),
            payload: format!("{}|{}|{}", req.amount_msat, req.destination, req.reason).into_bytes(),
        };
        self.gate
            .submit_and_decide(action, approval)
            .map_err(SpendError::Gate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gooseclaw::Rule9Gate;

    fn req(amount: u64) -> SpendRequest {
        SpendRequest {
            amount_msat: amount,
            rail: Rail::CashuP2pk,
            destination: "npub1...".into(),
            reason: "test".into(),
        }
    }

    fn dummy_approval() -> Approval {
        Approval {
            pubkey: [0u8; 32],
            nonce: [0u8; 32],
            expires_at_unix: u64::MAX,
            signature: [0u8; 64],
        }
    }

    #[test]
    fn over_cap_spend_never_reaches_the_gate() {
        let mut pg = PaymentGate {
            caps: SpendingCaps {
                per_spend_msat: 1000,
                daily_msat: 10_000,
            },
            gate: Rule9Gate::new([]),
        };
        assert_eq!(
            pg.request_spend(req(1001), &dummy_approval()),
            Err(SpendError::OverCap)
        );
        // and the gate saw nothing
        assert!(pg.gate.audit_log().is_empty());
    }

    #[test]
    fn under_cap_spend_still_needs_a_real_approval() {
        let mut pg = PaymentGate {
            caps: SpendingCaps {
                per_spend_msat: 1000,
                daily_msat: 10_000,
            },
            gate: Rule9Gate::new([]), // no allowlisted signer
        };
        assert!(matches!(
            pg.request_spend(req(1000), &dummy_approval()),
            Err(SpendError::Gate(GateError::UnknownSigner))
        ));
    }
}
