# 10 — QA History (audit trail)

How the GooseClaw foundation was stress-tested. Two formal QA passes plus the file-recovery and source-verification work. This is the durable record of *why* the current spec says what it says.

## Pass 1 — Comprehensive QA + production rebuild
Goal: a full-spectrum stress test of the GooseClaw concept (knowledge base, system design, product/UX, build strategy) benchmarked against the state of the art in always-on agent runtimes, plus a production rebuild (SDLC + a ready Claude Code kickoff prompt).

- **Source retrieval:** the original nine-file package could **not** be retrieved as source files; only derivative NotebookLM decks existed in Drive. Every claim about the package's *internals* was therefore labeled inferential; the *external* facts were all verified against primary sources.
- **Key external verifications:** goose is AAIF/Linux Foundation, Apache-2.0, ACP-native; ACP is a Zed-created open standard; the Nostr NIP kind numbers check out; **OpenClaw's security crisis is real** (CVE-2026-25253 one-click RCE, ~135k exposed instances, the ClawHub "ClawHavoc" malicious-skill campaign); **Trivy's 2026 compromise is real** (CVE-2026-33634) — Martin's ban is well-founded; Letta's sleep-time compute is real.
- **Benchmarking verdicts (steal / avoid / watch):** STEAL goose's scheduler + adversary reviewer + sandbox; STEAL IronClaw's heartbeat + guardian validation + skill trust tiers; STEAL Letta's sleep-time consolidation; AVOID every OpenClaw default (open gateway, `0.0.0.0` bind, unauthenticated control UI, unvetted skills); WATCH ElizaOS plugin architecture and the NIP-90 DVM ecosystem (but do **not** base the business case on DVM revenue — demand is weak).
- **Rebuild output:** a Rust-workspace SDLC, a Tauri-sidecar-under-Alfred deployment recommendation, NIP-46 remote signing, a Syft + OSV-Scanner CI with SHA-pinned Actions, and a ready-to-paste Claude Code kickoff prompt with phase gates.
- **Pass-1 error later corrected:** it reported IronClaw as ~6 crates with a "phantom" wasm crate. This was **reading the staging branch** — see the dispute below.

## Pass 2 — Second-pass verification + stress test
Triggered when two canonical files were recovered (`gooseclaw-project-kickoff-prompt.md`, `gooseclaw-foundation-spec-v1.md`). Goal: re-verify every pin as of 2026-07-06, adjudicate the IronClaw dispute, and stress-test the two recovered files.

- **File recovery:** 2 of 9 canonical files recovered; the decision ledger is reconstructable from the foundation spec; files `01`–`06`, `07_DECISION_LOG.md`, the Claude Code prompt (later also recovered), and the project Instructions remained missing. Drive holds only NotebookLM decks.
- **Pin verification:** almost every external bet verified real and current; **goose v1.39.0 is stale (v1.41.0 shipped 2026-07-03)**; Wasmtime is at 45.x (IronClaw pins 43.0.2); rust-nostr and CDK are ALPHA; Skillsmith is Elastic-2.0.
- **New development surfaced:** Block's **"Operation Pale Fire"** red team compromised an employee laptop via zero-width-Unicode prompt injection in a goose Recipe — directly informing the threat model's Unicode-sanitization requirement (including sanitizing the text shown to the human at the gate).

## The IronClaw dispute — adjudication
- **Who was right:** split decision. The **spec was right** that main has far more than 6 crates and that a wasm crate exists (pass 1 read staging). The **spec was wrong** on specifics: main has **~27 crates, not ~78**; `ironclaw_wasm` exists but **`ironclaw_wasm_sandbox_core` and `ironclaw_wasm_limiter` do not exist on any branch**; Wasmtime is **43.0.2** on main (43.0.1 was staging); commit **`4c82051` is unverifiable** (latest release 0.29.1 / 2026-06-04 / 556dfd0).
- **Confirmed real on main:** `ironclaw_safety`, `ironclaw_skills` (Trusted/Installed tiers), `ironclaw_secrets` (AES-256-GCM), `ironclaw_network` (egress), `ironclaw_capabilities`, `ironclaw_host_api`, `ironclaw_memory`; v2 dual-engine Python/Monty orchestrator with capability leases; license **MIT OR Apache-2.0**.
- **Standing lesson:** pin **branch + commit** explicitly; re-derive the steals section against a real pinned commit before porting.

## Amendments (the standing pre-build punch-list)
These ten must be reflected in the spec before the Phase-1 spike. Severity from the QA.

**Critical**
1. Add an **inbound-DM sender allowlist** + "all inbound text is untrusted data" rule (→ Constitution 11).
2. Resolve gate ownership: a **single core-owned** Rule-9 gate; channels/payments call it (→ Constitution 12).
3. Specify **approval authentication**: allowlisted-pubkey-signed, nonce + expiry bound to the action hash, single-use (→ Constitution 13).

**High**
4. Re-derive the **IronClaw steals** against a pinned real commit; delete the two non-existent wasm crates; fix crate count (~27) and Wasmtime (43.0.2); replace `4c82051` with a verified commit.
5. Re-pin **goose to a verified current release** (v1.41.0) and update the repo path to **aaif-goose/goose**.
6. Add a local **audit-log / observability** design reconciled with "zero telemetry."
7. Add global **resource/cost caps** (token/CPU/memory/wallet).
8. Document the **NIP-46 bunker-offline fail-closed** behavior and the NIP-46→NIP-04 crypto caveat (issue #1095).
9. Add a **session crash/resume** design that does not rely on goose's (absent) ACP resume.

**Housekeeping**
10. Adopt the two resolved decisions in the spec text (**libSQL**; **port-safety/author-governance**) and define the **skill WIT world** in Phase 5.

## Net verdict
The foundation is **sound to proceed to Phase 0/1**, conditional on the ten amendments. Its external technology bets are almost all verified-real and current; the architecture (goose-over-ACP, Option-C federation, steal-don't-fork) is coherent and on-thesis for the Goose Grant. The weaknesses were an unreliable IronClaw provenance note, a stale goose pin, and missing security/operability design — all fixable at Phase 0, none fatal.
