# 05 — Build Plan

Method: GitHub Spec Kit (constitution → specification → plan → tasks) + goose RPI. Nothing is committed to any repo yet (Rule 9). Each phase gate = **all tests pass + explicit human sign-off.**

## Phases

- **Phase 0 — Step-0 primary-source pins.** Pin + verify every volatile surface before wiring (see `08_PIN_REGISTER.md`): goose release + ACP surface + Approve-mode mechanism; `agent-client-protocol`; MCP (watch the 2026-07-28 RC — stateless redesign, Tasks-as-extension); NIP-17/44/59/46 test vectors; Wasmtime component model; Cashu NUT-11 / CDK; libSQL FTS5 + vector. Sparse-clone `nearai/ironclaw@<pinned-commit>` and confirm every crate/module path resolves. **Also produce: the threat-model doc and the observability/audit-log design** (moved earlier per QA).
  - *Acceptance:* verification table delivered with every drift flagged and sourced; threat model + audit-log design drafted.

- **Phase 1 — ACP runtime spike (highest leverage).** Drive a goose ACP subprocess from Rust; prove `prompt → tool call → ActionRequired → human ✅ → resume` with the **core** Rule-9 gate in the loop, in Approve mode. This validates or kills the foundation.
  - *Acceptance:* e2e consent-loop test passes; localhost-only bind; supervisor restart + session-replay test passes; resource caps (token/CPU/memory) enforced.

- **Phase 2 — Alfred MCP-vault federation.** Read-free / write-through-MCP + single-writer optimistic concurrency against Alfred's MCP vault; define cache staleness + reconnect/conflict semantics.
  - *Acceptance:* conflict test (human edit wins); Alfred-offline degradation test.

- **Phase 3 — Nostr channel + approval gate.** NIP-17 DMs; the non-bypassable ✅/❌ gate (calling core) + chunking as shared primitives; **inbound sender allowlist**; **authenticated approvals**; NIP-56/NIP-32 moderation/abuse handling.
  - *Acceptance:* red-team/prompt-injection suite passes (Unicode tag/zero-width smuggling, indirect injection via crafted DMs, trifecta exfil, approval spoof/replay).

- **Phase 4 — Key custody + wallet gating.** NIP-46 bunker (fail-closed); Cashu P2PK; per-spend approval + caps.
  - *Acceptance:* bunker-offline mute test; spend-cap-under-load test.

- **Phase 5 — Skills sandbox + safety + supply chain.** Wasmtime component model with a defined **WIT world**; port the `gooseclaw-safety` internals (attributed) + author the governance clean; Syft/OSV/Grype + Skill-Card; optional Skillsmith discovery.
  - *Acceptance:* sandbox escape attempt fails; supply-chain gate blocks a known-bad fixture.

- **Phase 6 — Heartbeat as draft-and-surface.** cron/event/webhook triggers → read-only observation → draft → approval. No unattended consequential action. Add the **eval harness** for agent behavior.
  - *Acceptance:* deployment runbook validated on a clean machine; eval harness green.

## Immediate tasks (Phase 0 → start of Phase 1)
1. **Initialize the GooseClaw repo** — AGPL-3.0-or-later; `CLAUDE.md` / `AGENTS.md`; Spec Kit `constitution.md` (from `02`), `specification.md` (from `04`), `plan.md` (this file), `tasks.md`. *(Rule 9: Martin authorizes repo creation/first push; the workspace produces the contents.)*
2. **Confirm the resolved decisions** (persistence = libSQL; steal split) and **close or defer the two remaining open ones** (`09_OPEN_DECISIONS.md`).
3. **Scaffold the Cargo workspace** — seven crate stubs with the module seams from `04` (empty `lib.rs` carrying the documented public surface + the `InjectionScanner` / `LeakScanner` traits). Target: `cargo check` clean.
4. **Phase-1 spike** — minimal Rust binary that spawns `goose acp`, completes the ACP handshake against the pinned goose in Approve mode, and demonstrates one core-gated tool call.
5. **Draft the Goose Grant proposal** in parallel (`PROPOSAL_goose_grant.md`).

## Standing pre-build punch-list
The ten amendments in `10_QA_HISTORY.md` §Amendments must be reflected in the spec before Phase 1. The three Critical ones (inbound allowlist, single core-owned gate, authenticated approvals) are load-bearing for the security model.
