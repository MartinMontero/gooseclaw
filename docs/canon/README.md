# GooseClaw canon — precedence

This directory is the home of the numbered GooseClaw canon, copied verbatim from the
project knowledge base (28-file Mission Control set, QA'd 2026-07-17).

Precedence (highest wins):
1. Verified primary source (with URL + as-of date)
2. This numbered canon — within it, `03_DECISION_LOG.md` leads; later rulings supersede
3. Kickoff prompts and chat transcripts

`specification.md` at repo root is assembled FROM this canon with per-line provenance
and is the governing file `CLAUDE.md` points to. On conflict between specification.md
and CLAUDE.md, specification.md wins; flag the conflict, stop, and surface it — never
resolve silently.

Known supersessions as of 2026-09-03 (flagged, canon text NOT edited):
- `09_OPEN_DECISIONS.md` listed Skillsmith scope + channel priority as OPEN — **resolved in-file 2026-09-08** (ruling 2): both items now record the D0901 rulings verbatim, recommendations kept as superseded.
  Martin DECIDED both 2026-09-01: Skillsmith IN for v1 (external-only ELv2 posture,
  npx-spawned, NOTICE attribution, no BuilderOS-hosted registry without smith-horn's
  written permission); channels Nostr → Signal → White Noise (marmot-protocol/mdk,
  NOT the obsolete whitenoise-rs crate). Recorded in specification.md §10.
- `08_PIN_REGISTER.md` last verified 2026-07-06; refreshed 2026-09-03 — see
  `docs/pin-register/08_PIN_REGISTER-2026-09-03.md`. Drift flagged, not silently fixed.
