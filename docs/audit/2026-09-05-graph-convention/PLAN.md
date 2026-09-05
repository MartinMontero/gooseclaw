# PLAN.md — Graph-engineering convention install, Step 1.5 redo (2026-09-05)

Session plan per the Project-graph rule: a subset of RECIPE.md — the subsystems touched, in
dependency order, with proof lines carried forward. This session touches exactly one
subsystem: **Graph-engineering convention** (needs: Foundation spec). All other rows are
mapped and re-proved by the gate, not built.

## Amendments applied (vs the wrong-substrate attempt)

- **(a) No DECISIONS.md.** Rulings live in the canon (`specification.md`); RECIPE.md proof
  lines target the canon directly. No second register.
- **(b) CLAUDE.md exists in this repo** (seed file, 94f0658). The `## Project graph`
  section is **added**, nothing replaced — single insertion before TIER 1.
- **(c) AUDIT.md / PLAN.md fold into `docs/audit/2026-09-05-graph-convention/`** — never
  the repo root.

## Files

| # | File | Change | Purpose |
|---|---|---|---|
| 1 | `docs/audit/2026-09-05-graph-convention/AUDIT.md` | NEW | Research findings + drift flags (RPI: Research) |
| 2 | `docs/audit/2026-09-05-graph-convention/PLAN.md` | NEW | This file (RPI: Plan) |
| 3 | `RECIPE.md` | NEW (root) | Persistent dependency graph: 13 subsystems, `\| Subsystem \| Needs \| Proof \|`, parallel tracks, DRIFT RULE, delegation rule |
| 4 | `gate.ps1` | NEW (root) | Definition-of-done gate: runs every RECIPE.md proof, PASS/FAIL per subsystem, `X/Y subsystems PASS`, exit 0 iff all pass, `-SelfTest` mode |
| 5 | `CLAUDE.md` | EDIT (additive) | `## Project graph` section inserted before TIER 1; zero other changes |

The PR body is written at `gh pr create` time from a temp file — it is not a repo artifact.

## RECIPE.md subsystem order (dependency spine first)

1. Foundation spec (assembled canon, `..\specification.md`) — needs nothing
2. Seed gate files (94f0658) — needs 1
3. Seven-crate design — needs 2
4. Rule-9 gate (core-owned) — needs 3
5. ACP contract (Approve mode, protocolVersion 1) — needs 4
6. MCP-vault federation (Alfred sole writer) — needs 5
7. Persistence (libSQL/FTS5 + native vector) — needs 6
8. Nostr channel layer — needs 5
9. Channel priority (RULED 2026-09-01) — needs 8
10. Payments (CDK/Lightning) — needs 8
11. Skillsmith scope (RULED 2026-09-01) — needs 3
12. Vendor denylist enforcement — needs 2
13. Graph-engineering convention — needs 1

Parallel tracks after ACP contract: {6→7} ∥ {8→9, 8→10} ∥ {11} ∥ {12}.

## gate.ps1 design

- Windows PowerShell 5.1, ASCII-only, no `&&`, no POSIX.
- Parses the RECIPE.md table; every subsystem row must have a check implemented in
  gate.ps1 — an unmapped row FAILs ("no gate check implemented"), so the gate cannot
  silently pass a growing graph.
- **Delegates to the real vendor gate**: the denylist proof runs
  `python vendor_gate.py --self-test` and requires exit 0, instead of duplicating the
  matcher. Same delegation rule will apply when `cargo check`/`cargo test` land in Phase 0.
- Reads the canon from `..\specification.md` (deliberately uncommitted). A missing canon
  FAILs the Foundation-spec row — it does not skip.
- `-SelfTest`: copies RECIPE.md to a temp file, plants a canary row whose proof requires an
  absent string, runs the gate against the copy, confirms the canary FAILs and the exit
  code is 1, deletes the temp copy, prints the captured evidence, exits 0 only if the
  failure fired. The real RECIPE.md is never modified.

## Anti-false-green evidence plan

1. `powershell -File gate.ps1 -SelfTest` on a **temp copy of the whole tree** (repo +
   canon mirrored one level up) → canary FAIL fires, evidence printed.
2. `powershell -File gate.ps1` on the temp copy → clean run, `13/13 subsystems PASS`,
   exit 0.
3. `python vendor_gate.py --self-test` standalone → exit 0.
4. All captured into the PR body.

## Stop line

Class A standing for this session: branch, commit, push, and PR opening are authorized
when the gate is green. **Stop before merge.** No exceptions to the vendor gate; no merge,
tag, or release without Martin's explicit GO.
