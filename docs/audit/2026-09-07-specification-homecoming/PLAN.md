# PLAN — canon/specification-homecoming (2026-09-07)

Subset of `RECIPE.md` touched by this session, in dependency order, proofs carried forward.

## Trigger

Martin ratified the amended canon (2026-09-07), including the §3.2 dual-era Alfred MCP
constraint from `docs/audit/2026-09-07-mcp-acp-revalidation/REPORT.md` §5, and ordered the
specification-homecoming PR package refreshed against `main` = `1013fdb` — prepared only,
no push without explicit GO.

## Pre-existing state (verified)

- No branch, file, or doc named "homecoming" existed locally or on origin before this
  session (searched branches, stashes, working tree, canon). The package was constructed
  fresh on branch `canon/specification-homecoming` from `1013fdb`.
- `main` moved `94f0658 → 1013fdb` (merge of PR #1, graph-engineering convention). Fetched
  2026-09-07.

## Subsystems touched and their proofs

| Subsystem | Change | Proof (carried forward) |
|---|---|---|
| Foundation spec (assembled canon) | Canon comes home: `specification.md` committed at repo root; proof line updated (old/new quoted in PR_BODY.md) | `specification.md` (repo root, committed) exists and contains `fiduciary agent runtime` and `AGPL-3.0-or-later` — 2/2 |
| MCP-vault federation (Alfred sole writer) | §3.2 ratified amendment applied (dual-era constraint; re-validation EXECUTED 2026-09-07) | canon §3.2 contains `Single-writer` and `system-of-record` — 2/2 (tokens preserved through the amendment) |
| Graph-engineering convention | `gate.ps1` spec path `..\specification.md` → `specification.md`; RECIPE.md canon-location paragraph; CLAUDE.md canon bullet | `RECIPE.md` and `gate.ps1` exist; CLAUDE.md contains `Project graph`; `docs/audit/` exists — 4/4 |
| Vendor denylist enforcement | untouched; re-verified by gate | `python vendor_gate.py --self-test` exits 0; ... — 4/4 |

## Evidence packaged

`docs/audit/2026-09-07-mcp-acp-revalidation/` (probe report, ACP/MCP wire transcripts,
probe sources, post-run vault state; `node_modules` excluded via `.gitignore`).

## Rule 9

Local commits on `canon/specification-homecoming` are part of the ordered preparation.
Push is explicitly gated on Martin's GO. No remote creation, no publish.
