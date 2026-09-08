# 13 — NIP Compliance Matrix

Every Nostr Improvement Possibility GooseClaw depends on, how it is used, and its verified status. Kind numbers and audit facts verified 2026-07-06 against github.com/nostr-protocol/nips (see `08_PIN_REGISTER.md`). Protocol-level compliance is the bar — the Rust implementation (`rust-nostr`) need not match Alfred's JS library, only the wire protocol.

| NIP | Purpose | Kinds | GooseClaw use | Status |
|---|---|---|---|---|
| **NIP-01** | Basic protocol, events, relays | 0 (metadata), 1 (note) | Foundation; profile + note publishing (approval-gated) | Verified |
| **NIP-05** | DNS-based identifiers | — | Human-readable agent identity in the setup wizard | Verified |
| **NIP-17** | Private DMs | 14 (chat), 13 (seal), 1059 (gift wrap) | **Primary channel.** Encrypted DMs to/from the agent; the default comms surface | Verified |
| **NIP-44** | Versioned encryption (v2) | — | Encryption layer under NIP-17. **Cure53-audited (Dec 2023)** | Verified |
| **NIP-59** | Gift wrap | 1059 | Metadata-protective wrapping of sealed DMs (hides sender/timing) | Verified |
| **NIP-46** | Remote signing (bunker) | 24133 | **Key custody.** No raw `nsec` on the always-on host; every sign is a policy-gated request. **Fail-closed when bunker offline.** Caveat: still uses NIP-04 encryption (issue #1095) | Verified w/ caveat |
| **NIP-57** | Lightning zaps | 9734 (request), 9735 (receipt) | Payments layer; approval-gated + capped | Verified |
| **NIP-61** | Nutzaps (Cashu over Nostr) | 9321, 10019 | eCash payments (P2PK); approval-gated + capped | Verified |
| **NIP-32** | Labeling | 1985 | Publishing verified-learnings back to the WCJBT catalog as labels; content tagging | Verified |
| **NIP-56** | Reporting | 1984 | **Moderation/abuse handling** for the public agent surface (Phase 3). *Handler design still to be authored.* | Verified (spec); design open |
| **NIP-04** | Legacy encrypted DMs | 4 | **Deprecated — read-only legacy only.** Marked "unrecommended" in the NIPs repo. Not used for outbound | Verified (deprecated) |
| **NIP-89** | Application handlers | 31990 | *(Optional/future)* advertise GooseClaw's supported job kinds for discovery | Verified (spec) |
| **NIP-90** | Data Vending Machines | 5000–5999 / 6000–6999 | *(Optional/future)* expose capabilities as a DVM. **Do not base the business case on DVM revenue — demand is weak** | Verified (spec); demand weak |

## Notes
- **Inbound trust (Constitution 11):** every inbound NIP-17 DM is untrusted **data**. Signature verification + Unicode-tag/zero-width/bidi stripping happen before content reaches the model. Unknown senders are quarantined (no vault reads, no tool exposure) until human-approved.
- **Approval authentication (Constitution 13):** a ✅/❌ reply is honored only if signed by an allowlisted approver pubkey, with a nonce + expiry bound to the action hash, single-use — defeats relay-injected/replayed approvals.
- **Metadata residual:** NIP-17/59 hide DM content and sender, but an always-on agent's relay connections remain an IP fingerprint. Tor/mixnet for relay I/O is a `07_THREAT_MODEL.md` open item.
- **Library churn:** `rust-nostr` is ALPHA (breaking API changes expected) — pin exact versions at Phase 0.
