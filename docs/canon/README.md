# docs/canon — governing canon, home in the repo

**Precedence (per the BuilderOS spec, as stated in Martin's canon-homecoming
tasking 2026-09-07 — the BuilderOS spec text itself is not on this disk;
label: REPORTED):** **verified primary source > component canon > system
spec.** Within its lane, this repo's own canon governs.

## Governing canon — in-repo

| File | Role |
|---|---|
| `specification.md` | GooseClaw specification — DRAFT-ratified 2026-09-07 (homecoming commit `53bebaf` on `canon/specification-homecoming`); assembled from the numbered canon with inline provenance tags |
| `vendor_gate.py` | Compiled vendor gate (`[VG]`) |
| `ci.yml` | CI gate (`[CI]`) |
| `CLAUDE.md` | Standing orders (`[CL]`) |
| `RECIPE.md` + `gate.ps1` | Dependency graph + definition-of-done gate (graph convention, PR #1) |

## Governing canon — still Kimi-side (ABSENT from disk, homecoming blocked on export)

`specification.md` cites these by tag; they govern but live only in the
Kimi project's knowledge files. **None were found on this disk
(2026-09-07, full-text search) — stated, not reconstructed.** When Martin
exports them, they land in this directory verbatim:

- `00`–`10`, `19`, `20` — the numbered canon set (incl.
  `02_CONSTITUTION.md` — the binding 13-article text, `03_DECISION_LOG.md`,
  `07_THREAT_MODEL.md`, `09_OPEN_DECISIONS.md`, `10_QA_HISTORY.md`)
- `BUILDEROS-CONSTITUTION-GooseClaw.md` (`[BC]`)
- `08_PIN_REGISTER.md` and `08_PIN_REGISTER-refreshed-2026-09-03.md` (`[08R]`)

**Known drift waiting on that export (spec §10 C3, flagged not fixed):**
`09_OPEN_DECISIONS.md` still lists Skillsmith scope (#5) and channel
priority (#6) as open; both were DECIDED by Martin 2026-09-01
(`[D0901]`, spec §7.2). Substance is resolved; the file update happens
when the set comes home.

## Reference material (stays put; not governing)

`docs/audit/*` (session audits, plans, probe evidence).

## The vocabulary (D-0901 era note)

This repo's audit docs already use the five system states (EXECUTED /
VERIFIED-LIVE / CANON / REPORTED / UNVERIFIED — build-workflow
vocabulary per Alfred ADR-0005). Holmes's D-15 (2026-09-01 ruling)
retires `[DIRECTIONAL]`/`[NEEDS-CAVEAT]` Holmes-side; if the numbered
canon set carries those markers when exported, migrate on landing with
the same mapping: `[DIRECTIONAL]` → REPORTED; `[NEEDS-CAVEAT]` → the
state its sourcing earns, caveat preserved verbatim.
