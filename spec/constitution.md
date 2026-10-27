# GooseClaw Constitution (Spec-Kit working copy)

Derived from the binding canon: specification.md §1 (assembles 02_CONSTITUTION.md, ratified
articles 1-10 + binding security amendments 11-13). The canon leads on any conflict.

## The 13 articles (compressed)

1. **Memory is not hands** — Alfred is sovereign memory; typed MCP boundary only.
2. **Consent before consequence (Rule 9)** — no write/send/spend/commit/push/destructive act
   without explicit human go-ahead.
3. **Sovereign by construction** — local-first, zero telemetry, no public bind by default,
   no mandatory hosted dependency.
4. **goose is the engine; ACP is the seam** — engine-agnostic at the boundary.
5. **Secrets never reach skills** — encrypted at rest, injected only at egress to allowlisted
   endpoints, scanned outbound.
6. **Untrusted code sandboxed by default** — Wasmtime component model, deny-by-default,
   trust tiers.
7. **Vendor line holds** — Meta/OpenAI/xAI excluded at config/CI level; enforced, not advisory.
   Google/Mistral/Ollama/MiniMax/Anthropic permitted.
8. **Nostr-native identity** — metadata-protective by default (NIP-17 DMs, NIP-46 custody).
9. **AGPL-3.0-or-later** — commons; Zapstore distribution.
10. **Evidence over assertion** — zero fabrication; primary sources; stop and ask.
11. **Inbound is untrusted; talk is gated** — sender allowlist default; unknown senders get
    no tools, no vault reads. (AMENDMENT 1; re-ratified 2026-10-27.)
12. **One gate, owned by the core** — single, non-bypassable Rule-9 primitive;
    channels/payments CALL it. (AMENDMENT 2; re-ratified 2026-10-27.)
13. **Approvals are authenticated** — allowlisted-pubkey-signed, nonce + expiry bound to the
    action hash, single-use; NIP-46 bunker-offline => fail closed. (AMENDMENT 3; re-ratified
    2026-10-27.)

## Enforcement

- CLAUDE.md Tier 1 mirrors these as session rules.
- scripts/vendor_gate.py + .github/workflows/ci.yml enforce article 7 (and supply chain).
- gate.ps1 + RECIPE.md enforce the proof graph (article 10).
