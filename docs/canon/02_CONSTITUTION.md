# 02 — Constitution (non-negotiables)

The ten articles are ratified. Articles 11–13 are **security amendments** added after the two QA passes (see `10_QA_HISTORY.md`); they are binding and must be reflected in the spec before the Phase-1 spike.

## Ratified articles

1. **Memory is not hands.** Alfred remains the single-user sovereign memory layer; GooseClaw never runs inside Alfred and never turns Alfred into an agent. They communicate only across Alfred's typed MCP boundary.
2. **Consent before consequence (Rule 9).** No write, send, spend, commit, push, or destructive action executes without explicit human go-ahead. Proactivity may only *draft and surface*.
3. **Sovereign by construction.** Local-first, zero telemetry, no public bind by default, no mandatory hosted dependency. Any external service must be self-hostable or optional.
4. **goose is the engine; ACP is the seam.** GooseClaw drives goose over ACP and stays engine-agnostic at that boundary; no in-process coupling to a specific agent core.
5. **Secrets never reach skills.** Credentials are stored encrypted, injected only at the host/network egress boundary to allowlisted endpoints, scanned on the way out, and never exposed to model or tool code.
6. **Untrusted code is sandboxed by default.** Community skills run in a Wasmtime component-model sandbox under deny-by-default capabilities; trust tiers gate tool authority.
7. **Vendor line holds.** Meta, OpenAI, xAI are excluded at config/CI level; this is enforced, not advisory.
8. **Nostr-native identity, metadata-protective by default.** NIP-17 DMs and NIP-46 custody are defaults, not options — because the people most exposed are the ones this serves.
9. **AGPL and the commons.** Source stays AGPL-3.0-or-later; network-served instances offer their source (AGPL §13); distribution is Nostr-native (Zapstore).
10. **Evidence over assertion.** Zero fabrication; primary sources with URLs; stop and ask rather than infer; no unauthorized scope decisions.

## Security amendments (binding; added post-QA)

11. **Inbound is untrusted; talk is gated.** All inbound message content (Nostr DMs and any channel) is treated as untrusted **data**, never as instructions. Unknown senders are **not** on the allowlist by default: they get no tool exposure and no vault reads until a human approves them. This closes the lethal-trifecta exposure of an always-on agent that reads a private vault and can reach external egress.
12. **One gate, owned by the core.** The Rule-9 approval gate is a **single, core-owned, non-bypassable primitive**. Channels and payments *call* it; they never own or reimplement the decision. There is exactly one enforcement point.
13. **Approvals are authenticated.** A ✅/❌ approval is only honored if it is **signed by an allowlisted approver pubkey**, carries a **nonce and expiry bound to the specific action's hash**, and is **single-use**. Relay-injected, replayed, or reordered approvals are rejected. Bunker-offline (NIP-46) means the agent **fails closed** — it goes mute rather than acting unsigned.
