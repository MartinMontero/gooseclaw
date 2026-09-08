# 03 — Decision Log (canonical)

The decision system-of-record. Reconstructs and supersedes the original `07_DECISION_LOG.md` (unrecovered), merging the ratified ledger from the foundation spec with the resolutions in the Claude Code kickoff prompt and the ten amendments from `10_QA_HISTORY.md`. Labels: **[ratified]** decided; **[resolved]** an ex-open decision now closed; **[amended]** changed by QA.

## Ratified spine

| Axis | Decision |
|---|---|
| Foundation | **[ratified]** goose driven over **ACP** (Rust crate `agent-client-protocol`, wire version 1). **NOT** a fork of IronClaw; **NOT** embedded in Alfred. **[amended]** Pin goose to a **verified current release** at Phase 0 (v1.41.0 as of 2026-07-03, superseding the v1.39.0 in the source spec); repo is **aaif-goose/goose** (AAIF/Linux Foundation), Apache-2.0. |
| Topology | **[ratified]** **Option C federated.** Alfred = sovereign memory substrate via its path-confined MCP vault; GooseClaw = the always-on body, a separate process tree. |
| Autonomy | **[ratified]** **Rule-9 human-in-the-loop by default.** Heartbeat/routines = **draft-and-surface-for-approval**, never unattended consequential action. |
| Approval gate | **[amended]** A **single, core-owned, non-bypassable** primitive (Constitution 12). Channels/payments call it. Approvals are **authenticated**: allowlisted-pubkey-signed, nonce + expiry bound to the action hash, single-use (Constitution 13). |
| Inbound trust | **[amended]** **Sender allowlist by default** (Constitution 11). Unknown senders quarantined; all inbound text is untrusted data. |
| DMs | **[ratified]** **NIP-17** private DMs (seal kind 13 + gift-wrap kind 1059, chat kind 14), **NIP-44** encryption, **NIP-59** wrap. NIP-04 = deprecated, read-only legacy. |
| Key custody | **[ratified]** **NIP-46** remote signing (bunker); no hot `nsec` in the always-on process. **[amended]** Fail-closed when bunker offline; document the NIP-46→NIP-04 crypto caveat (nostr-protocol/nips issue #1095). |
| Payments | **[ratified]** **Lightning + Cashu** (P2PK proofs, NUT-11, via Rust CDK) + **NIP-57/NIP-61** zaps. Explicit-approval-per-spend + per-period caps by default. |
| Skills | **[ratified]** **Wasmtime component-model** sandbox + **Trusted/Installed** trust tiers + supply-chain gate (Syft SBOM / OSV-Scanner / Grype + Skill-Card). **[amended]** Define the skill **WIT world / ABI** in Phase 5. Skillsmith optional (see open decisions). |
| Memory / persistence | **[resolved]** **libSQL/FTS5 + native vector**, RRF implemented on top. Embedded, single-file, in-process — no server, no ports, no hosted account. Postgres/pgvector rejected (IronClaw's heavier multi-user choice; a sovereignty + supply-chain liability). |
| Steal mechanism | **[resolved]** **Port the security *mechanism* (attributed + scanned); author the *governance* clean.** Port/vendor (Phase 5, with go-ahead): the Wasmtime sandbox and the leak-detection / credential-injection / redaction *internals* of `ironclaw_safety`, under AGPL with a NOTICE citing `nearai/ironclaw@<pinned-commit>`, run through the Syft/OSV/Grype gate. Author clean: the Policy/Severity rules, the Rule-9 gate, the Trusted/Installed model + capability leases, and skill selection. |
| Resources / cost | **[amended]** Global caps required: token/cost ceilings, Wasmtime fuel/memory limits, per-routine quotas — not just per-spend wallet caps. |
| Observability | **[amended]** Local-only structured **audit log** (steal IronClaw's decision/dispatch/audit record model); reconciled with "zero telemetry" = no network egress, not no logs. |
| Session recovery | **[amended]** GooseClaw owns durable session state and replays context; it does **not** rely on goose ACP session resume (not supported upstream). |
| License / dist | **[ratified]** **AGPL-3.0-or-later**; distribution via **Zapstore** (desktop/CLI path via zapstore-cli). |
| Provider policy | **[ratified]** Config-level denylist excluding **only Meta, OpenAI, xAI**; Google/Mistral/Ollama/MiniMax/Anthropic permitted. |
| IronClaw provenance | **[amended]** Re-derive the entire steals section against a **pinned, real** main-branch commit before any port. The source spec's provenance is unreliable (see `04_ARCHITECTURE.md` §4 and `10_QA_HISTORY.md`). |
| Method | **[ratified]** GitHub **Spec Kit** (constitution → specification → plan → tasks) + goose **RPI**; **Rule 9** (announce + go-ahead before any commit/push/destructive action). |
| Ethos | **[ratified]** Liberation tech; sovereignty; local-first; zero telemetry; no vendor lock-in. |

## Still open (need ratification — see `09_OPEN_DECISIONS.md`)
- **Skillsmith in scope for v1?** (optional self-hosted MCP discovery; Elastic-2.0 license caveat).
- **Channel priority beyond Nostr?** (recommendation: Signal next).
