# GooseClaw — Session Specification (Phase 0 + Phase 1)

Derived from the canon (specification.md, repo root). Session scope ONLY:
Phase 0 (verify) and Phase 1 (ACP spike). Nothing past Phase 1.

## Scope

- **In:** seven-crate AGPL workspace skeleton; documented public surfaces; the
  InjectionScanner/LeakScanner trait seams; the core-owned Rule-9 gate;
  the ACP spike against the installed goose.
- **Out:** Alfred MCP federation (Phase 2), rust-nostr wiring, libSQL binding,
  Wasmtime runtime, payments rails, vendoring IronClaw code (Phase 5),
  any autonomy (Phase 6).

## Contracts under test this session

### goose <-> GooseClaw (ACP) — canon §3.1, drift-corrected 2026-10-27
- Spawn: goose acp as supervised subprocess, stdio/JSON-RPC. (goose serve = HTTP/WS variant.)
- Client crate: agent-client-protocol **=2.0.0** — matches the installed goose 1.48.0
  (its Cargo.lock @ v1.48.0 pins 2.0.0). The kickoff prompt's 1.0.0 pin is SUPERSEDED
  (drift recorded in docs/audit/2026-10-27-phase0-verify/).
- Wire: **protocolVersion = 1** (ProtocolVersion::V1). A v2 wire exists in the crate as
  alpha — not used.
- Approve mode: NO CLI flag on goose acp (verified --help, 1.48.0). Mechanism is the
  **GOOSE_MODE** env var (auto/approve/chat/smart_approve; default auto) — forced to
  approve in a scrubbed spawn env, plus GOOSE_TELEMETRY_ENABLED=false.
  Caveat carried from canon: no documented upstream hard lock against mid-session mode
  change — GooseClaw enforces Approve at its own gate and does not trust the goose setting.
- Session state: GooseClaw owns durable session state (goose ACP has no resume).

### Alfred <-> GooseClaw (MCP vault) — canon §3.2 — Phase 2, NOT BUILT
Dual-era MCP server constraint (ratify pending); single-writer + optimistic concurrency;
human-edit-wins.

## The three Critical amendments (as approved this session)

Recorded in full (old text -> new text -> why -> approval) in the canon Appendix C.
Summary:
1. Inbound sender allowlist enforced in channels; inbound text is untrusted data.
2. Exactly one core-owned Rule-9 gate; channels/payments call it, never own it.
3. Approvals signed by allowlisted pubkey, nonce + expiry bound to action hash, single-use.

## Acceptance (Phase 1)

cargo check/fmt/clippy/test clean; vendor gate green with empty EXCEPTIONS;
CI pins resolved + checksum-verified; ACP handshake at protocolVersion = 1 in forced
Approve mode with one core-gated, authenticated tool-call round-trip, real messages
logged under docs/audit/2026-10-27-phase1-acp-spike/. No commit/push.
