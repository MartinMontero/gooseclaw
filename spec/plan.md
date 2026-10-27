# GooseClaw — Phase 1 Plan (EXECUTING)

Approved 2026-10-27 by the operator, as presented in the Plan checkpoint.

## Order of work (dependency order per RECIPE.md)

1. Workspace skeleton: Cargo.toml, 7 crates, deny.toml, LICENSE (AGPL-3.0), NOTICE, AGENTS.md.  [DONE]
2. CI: move ci.yml -> .github/workflows/ci.yml, vendor_gate.py -> scripts/ (approved move);
   resolve 9 PIN placeholders; add checksum verification (syft/grype/osv checksum files
   verified live against release assets 2026-10-27).  [DONE]
3. Crate stubs with documented public surfaces + unit proofs.  [DONE]
4. Spec-Kit docs (this set) + canon Appendix C (amendments) + audit + pin register.
5. gate.ps1 / RECIPE.md proof lines updated per DRIFT RULE (locations + pin count changed
   by the approved move + pin resolution; old -> new quoted in the audit report).
6. ACP spike: crates/gooseclaw/src/bin/acp_spike.rs.  [BUILT]
7. Provider contingency: no GOOSE_PROVIDER configured; Ollama present (0.40.7) but ZERO
   models pulled and NOT configured for goose. Per the approved contingency: DO NOT
   install; run handshake leg; stop and report on the prompt leg if it cannot proceed.

## Risks

- Prompt leg may fail without a provider -> exit code 2, reported, NOT fixed by installing.
- agent-client-protocol 2.0.0 API used from its own shipped example
  (examples/yolo_one_shot_client.rs) — no invented API.
