# 09 — Open Decisions

Status of the six decisions the foundation spec flagged for Martin. Four are **resolved** (two by Martin's D0901 rulings, 2026-09-01), two are **confirmed**, none remain open.

## Resolved
1. **Persistence → libSQL/FTS5 + native vector; RRF on top.** [resolved] Embedded, single-file, in-process — no server, no ports, no hosted account: the local-first/sovereign posture. Postgres/pgvector rejected as IronClaw's heavier multi-user choice and a sovereignty + supply-chain liability; no hybrid-search capability is lost (RRF is a ranking-fusion formula over FTS5 + the vector index). IronClaw itself ships `ironclaw_hooks_libsql` for this case. *Switch only if multi-tenant server scale ever becomes a hard requirement.*
2. **Steal mechanism → port the security *mechanism*, author the *governance* clean.** [resolved] IronClaw is MIT OR Apache-2.0, so porting into AGPL with a NOTICE is clean. Port the Wasmtime sandbox + the leak-detection/credential-injection/redaction internals of `ironclaw_safety`; author clean the Policy/Severity rules, the Rule-9 gate, the Trusted/Installed model + capability leases, and skill selection (IronClaw is migrating selection to Python anyway).

## Confirmed
3. **Seven-crate split + `gooseclaw-*` naming.** [confirmed] Keep the seven, with one correction from QA: the Rule-9 gate is a **core-owned** primitive, not owned by `gooseclaw-channels` (Constitution 12).
4. **Rust Nostr library → rust-nostr (`nostr` / `nostr-sdk`).** [confirmed] Supports all required NIPs; parity with Alfred is at the **protocol/NIP level**, not the library level. Pin exact versions (ALPHA churn).

## Still open (need ratification)
5. **Skillsmith in scope for v1?** [RESOLVED  Real and useful (MCP skill discovery, local-first), but **Elastic-2.0** (not OSI-free), which sits awkwardly against the AGPL-commons ethos, and it is not essential to the core loop. **Recommendation:** keep it **optional, self-hosted only, deferred past v1** — not a v1 dependency.
6. **Channel priority beyond Nostr?** [RESOLVED  **Recommendation: Signal next** — consistent with the metadata-protective, high-safety "liberation tech" posture (Constitution 8). Discord/Telegram add metadata exposure; if added later, gate behind explicit metadata warnings.

## Decide-by
- #5 and #6 were DECIDED 2026-09-01 (D0901) 
- All three Critical security amendments (inbound allowlist, single core-owned gate, authenticated approvals) are **not** open decisions — they are ratified in `02_CONSTITUTION.md` and must be in the spec before Phase 1.
