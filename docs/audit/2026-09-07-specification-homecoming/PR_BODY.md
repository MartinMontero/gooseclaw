# PR: Bring the canon home — specification.md (ratified 2026-09-07, incl. §3.2 dual-era MCP constraint)

**Base:** `main` @ `1013fdb` · **Branch:** `canon/specification-homecoming`
**Status:** prepared locally; NOT pushed (awaiting Martin's explicit GO per Rule 9).

## What this PR does

1. **Homes the canon.** `specification.md` moves from the uncommitted parent-directory
   register to a committed file at the repo root. Ratified by Martin 2026-09-07.
2. **Carries the ratified §3.2 amendment** — the MCP-over-ACP re-validation outcome and
   the **dual-era Alfred MCP constraint** (evidence:
   `docs/audit/2026-09-07-mcp-acp-revalidation/`):
   - goose 1.48.0's MCP client is legacy-era (opens `initialize` / `2025-11-25`); a strict
     2026-07-28 server **rejects it** (spec `basic/versioning.mdx` matrix: Legacy → Modern
     = Fails). **BREAKS**, wire transcript in the audit dir.
   - Against a **dual-era** server the full §3.2 call shape **WORKS** over ACP v1 in forced
     Approve mode: `tools/list`, `vault_read`, `vault_write(expectedVersion)` v1→v2, stale
     write CONFLICT (human-edit-wins); every call gated by `session/request_permission`.
   - **RULED:** Alfred's MCP server MUST be dual-era until goose ships a 2026-07-28-capable
     MCP client. (`@modelcontextprotocol/sdk` latest 1.30.0 tops out at 2025-11-25 —
     dual-era is currently the only SDK-buildable conformant target.)
3. **DRIFT RULE disclosures** (reality legitimately changed; proofs updated in this PR,
   old → new quoted):

   - Foundation spec proof —
     OLD: `` `..\specification.md` exists and contains `fiduciary agent runtime` and `AGPL-3.0-or-later` — 2/2 ``
     NEW: `` `specification.md` (repo root, committed) exists and contains `fiduciary agent runtime` and `AGPL-3.0-or-later` — 2/2 ``
   - `gate.ps1` spec path —
     OLD: `$Spec = Join-Path $Root '..\specification.md'   # assembled canon - deliberately NOT committed`
     NEW: `$Spec = Join-Path $Root 'specification.md'   # assembled canon - committed at repo root (homecoming 2026-09-07)`
   - RECIPE.md "Canon location" paragraph and CLAUDE.md canon bullet updated to match
     (old text quoted "deliberately never committed / reads it from the repo's parent
     directory"; new text records the committed home and the superseded parent copy).

4. **Flags (not silently fixed), per pin rules:**
   - stage-goose-sidecar pin **1.43.0 ≠ probed/ratified goose 1.48.0** — sidecar pin needs
     re-verification before Phase 2.
   - Canon open item **O2** ("MCP-vault contract re-validation … OPEN `[08R]`") is now
     satisfied by the 2026-09-07 probe; recommend a follow-up ruling flipping O2 to CLOSED
     (not edited here — outside the ratified diff).
   - The parent-directory copy `..\specification.md` still exists post-amendment and is
     **superseded** by this PR; recommend Martin archive/delete it after merge (deletion
     not performed — Rule 9).

## Gate evidence

`powershell -File gate.ps1` → see PLAN.md / session log (all subsystems PASS;
`gate.ps1 -SelfTest` canary PASS; `vendor_gate.py --self-test` exit 0).

## Package contents

- `specification.md` (new, committed canon, ratified text)
- `gate.ps1`, `RECIPE.md`, `CLAUDE.md` (graph updates per DRIFT RULE)
- `.gitignore` (excludes probe `node_modules`)
- `docs/audit/2026-09-07-mcp-acp-revalidation/` (probe report + wire transcripts + sources)
- `docs/audit/2026-09-07-specification-homecoming/` (this plan + PR body)
