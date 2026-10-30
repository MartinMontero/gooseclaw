# RECIPE.md — GooseClaw dependency graph

The persistent graph of what depends on what, and how each subsystem **proves** it works.
Read this before any work. A session's plan is a subset of this file: the subsystems being
touched, in dependency order, with proof lines carried forward.

Proof rules: every proof is a checkable criterion ending in a number, a filename, or a named
output — never a status message. `gate.ps1` runs every proof line below.

**Canon location:** the assembled canon is `specification.md` at the **repo root** —
**committed** since the homecoming PR (2026-09-07, ratified by Martin incl. the §3.2
dual-era MCP constraint). The pre-homecoming parent-directory copy (`..\specification.md`)
is superseded; `gate.ps1` reads the committed copy. There is no `DECISIONS.md`;
proofs target the canon directly.

| Subsystem | Needs | Proof |
|---|---|---|
| Foundation spec (assembled canon) | — | `specification.md` (repo root, committed) exists and contains `fiduciary agent runtime` and `AGPL-3.0-or-later` — 2/2 |
| Seed gate files (94f0658) | Foundation spec (assembled canon) | `CLAUDE.md`, `.github/workflows/ci.yml`, `scripts/vendor_gate.py` exist; CLAUDE.md contains `TIER 1` — 4/4 |
| Seven-crate design (spec) | Seed gate files (94f0658) | CLAUDE.md names all seven crates (`gooseclaw`, `-channels`, `-skills`, `-nostr`, `-knowledge`, `-payments`, `-safety`) — 7/7 |
| Rule-9 gate (core-owned) | Seven-crate design (spec) | CLAUDE.md contains `Rule 9` and `go-ahead`; canon §4.1 contains `core-owned` and `one enforcement point` — 4/4 |
| ACP contract (Approve mode, protocolVersion 1) | Rule-9 gate (core-owned) | canon §3.1 contains `protocolVersion = 1` and `Approve` — 2/2 |
| MCP-vault federation (Alfred sole writer) | ACP contract (Approve mode, protocolVersion 1) | canon §3.2 contains `Single-writer` and `system-of-record` — 2/2 |
| MCP-vault federation client (Phase 2: expectedVersion + offline semantics) | MCP-vault federation (Alfred sole writer) | crates/gooseclaw-knowledge `cargo test` summed `N passed` = **6** (rrf_math_is_exact + fuse_prefers_docs_ranked_in_both_lists + 4 new: conflict_human_edit_wins, offline_writes_mute, create_only_null_semantics, ttl_marks_old_cache_entries_expired); source carries `CONFLICT` and `stale` semantics — 2/2 |
| Nostr channel red-team suite and relay seam (Phase 3) | ACP contract (Approve mode, protocolVersion 1) | crates/gooseclaw-channels `cargo test` summed `N passed` = **18** (4 baseline + 14 new: injection_pattern, exfil_url, allowlisted_host, trifecta, oversized, chunking x2, redteam_table, approval_defenses, relay drain x2, relay chunking, relay signer-offline, relay oversized); `moderate()` and `chunk_text()` present in moderation.rs — 2/2 |
| Persistence (libSQL/FTS5 + native vector) | MCP-vault federation (Alfred sole writer) | canon contains `libSQL`, `FTS5`, `native vector` — 3/3 |
| Nostr channel layer | ACP contract (Approve mode, protocolVersion 1) | canon contains `NIP-17`, `NIP-44`, `NIP-46` — 3/3 |
| Channel priority (RULED 2026-09-01) | Nostr channel layer | canon §7.2(b) contains `Signal`, `White Noise`, `marmot-protocol/mdk` — 3/3 |
| Payments (CDK/Lightning) | Nostr channel layer | canon contains `Cashu`, `P2PK`, `NIP-57` — 3/3 |
| Skillsmith scope (RULED 2026-09-01) | Seven-crate design (spec) | canon §7.2(a) contains `Skillsmith`, `ELv2`, `2026-09-01` — 3/3 |
| Vendor denylist enforcement (Meta/OpenAI/xAI) | Seed gate files (94f0658) | `python scripts/vendor_gate.py --self-test` exits 0; ci.yml wires `--self-test`; vendor_gate.py `EXCEPTIONS` map empty; ci.yml carries 0 `PIN_RESOLVE_IN_PHASE_0` placeholders and pins actions/checkout to full SHA `3d3c42e5...` — 4/4 |
| Graph-engineering convention | Foundation spec (assembled canon) | `RECIPE.md` and `gate.ps1` exist; CLAUDE.md contains `Project graph`; `docs/audit/` exists — 4/4 |

## Parallel tracks

After the serial spine (Foundation spec → Seed gate files → Seven-crate design → Rule-9 gate
→ ACP contract), these tracks share **zero** dependencies and may proceed in parallel:

- **Track A:** MCP-vault federation → Persistence
- **Track B:** Nostr channel layer → Channel priority; Nostr channel layer → Payments
- **Track C:** Skillsmith scope (off Seven-crate design)
- **Track D:** Vendor denylist enforcement (off Seed gate files)

## DRIFT RULE

Every proof line is re-verified by any PR that touches its subsystem. A drifted proof gets
**flagged in the PR body, never silently edited**. If a subsystem's reality has legitimately
changed, the PR must say so explicitly and update the proof line in the same PR, with the
old and new proof quoted. A proof that cannot be run (missing artifact, missing tool) is a
FAIL, not a skip.

## Delegation rule

When a real build gate lands (Phase 0: `cargo check`, `cargo test`, clippy), proof lines
**call it** rather than re-implementing it — the same way the vendor-denylist proof already
delegates to `vendor_gate.py --self-test` instead of duplicating the matcher.

## Proof-line change log (DRIFT RULE)

**2026-10-27 — Phase 1 session (changes approved by the operator):**

1. Seed gate files proof. OLD: "CLAUDE.md, ci.yml, vendor_gate.py exist" (repo root).
   NEW: "CLAUDE.md, .github/workflows/ci.yml, scripts/vendor_gate.py exist".
   WHY: operator-approved move of CI files into canonical locations.
2. Vendor denylist enforcement proof. OLD: "ci.yml carries exactly 9
   PIN_RESOLVE_IN_PHASE_0 placeholders". NEW: "ci.yml carries 0 placeholders and pins
   actions/checkout to full SHA 3d3c42e5aac5ba805825da76410c181273ba90b1 (v7.0.1)".
   WHY: Phase-0 pin-resolution duty executed; placeholders were designed to be resolved.
   Resolutions recorded in docs/pin-register/08_PIN_REGISTER-2026-10-27.md.
3. gate.ps1 check implementations updated to match both new proof lines.

**2026-10-29 — Phase 2 session (federation client, expectedVersion + offline semantics):**

4. NEW proof row added: "MCP-vault federation client (Phase 2: expectedVersion + offline
   semantics)". OLD: no federation-client row existed (Phase 1 delivered only the RRF
   seam). NEW: `cargo test -p gooseclaw-knowledge` summed `N passed` = **6** (RRF's 2 +
   4 new named proofs: conflict_human_edit_wins, offline_writes_mute,
   create_only_null_semantics, ttl_marks_old_cache_entries_expired).
   WHY: spec §8's acceptance gate is executable now — the conflict test (human edit wins)
   and the Alfred-offline degradation test (mute/stale/revalidate) are real tests, not
   claims. gate.ps1 gained a matching `Test-FederationClient` check block.

**2026-10-29 — Phase 3 session (Nostr channel red-team suite):**

5. NEW proof row added: "Nostr channel red-team suite and relay seam (Phase 3)". OLD: no channel
   red-team row existed. NEW: `cargo test -p gooseclaw-channels` summed `N passed` = **18**
   (4 baseline + 14 new adversarial/transport tests: injection_pattern, exfil_url,
   allowlisted_host, trifecta, oversized, chunking x2, redteam_table, approval_defenses,
   relay drain x2, relay chunking, relay signer-offline, relay oversized).
   WHY: spec §8 requires the red-team/prompt-injection suite to pass — Unicode smuggling,
   crafted-DM injection, trifecta exfil, approval spoof/replay — and the suite now exists
   as executable tests, not documentation. gate.ps1 gained a matching `Test-RedteamSuite`
   check block.
