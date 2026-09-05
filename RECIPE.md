# RECIPE.md — GooseClaw dependency graph

The persistent graph of what depends on what, and how each subsystem **proves** it works.
Read this before any work. A session's plan is a subset of this file: the subsystems being
touched, in dependency order, with proof lines carried forward.

Proof rules: every proof is a checkable criterion ending in a number, a filename, or a named
output — never a status message. `gate.ps1` runs every proof line below.

**Canon location:** the assembled canon is `specification.md` in the repo's **parent
directory** (`..\specification.md`). It is deliberately **never committed** — it is the
ratification-draft register of rulings, not a repo artifact. There is no `DECISIONS.md`;
proofs target the canon directly.

| Subsystem | Needs | Proof |
|---|---|---|
| Foundation spec (assembled canon) | — | `..\specification.md` exists and contains `fiduciary agent runtime` and `AGPL-3.0-or-later` — 2/2 |
| Seed gate files (94f0658) | Foundation spec (assembled canon) | `CLAUDE.md`, `ci.yml`, `vendor_gate.py` exist; CLAUDE.md contains `TIER 1` — 4/4 |
| Seven-crate design (spec) | Seed gate files (94f0658) | CLAUDE.md names all seven crates (`gooseclaw`, `-channels`, `-skills`, `-nostr`, `-knowledge`, `-payments`, `-safety`) — 7/7 |
| Rule-9 gate (core-owned) | Seven-crate design (spec) | CLAUDE.md contains `Rule 9` and `go-ahead`; canon §4.1 contains `core-owned` and `one enforcement point` — 4/4 |
| ACP contract (Approve mode, protocolVersion 1) | Rule-9 gate (core-owned) | canon §3.1 contains `protocolVersion = 1` and `Approve` — 2/2 |
| MCP-vault federation (Alfred sole writer) | ACP contract (Approve mode, protocolVersion 1) | canon §3.2 contains `Single-writer` and `system-of-record` — 2/2 |
| Persistence (libSQL/FTS5 + native vector) | MCP-vault federation (Alfred sole writer) | canon contains `libSQL`, `FTS5`, `native vector` — 3/3 |
| Nostr channel layer | ACP contract (Approve mode, protocolVersion 1) | canon contains `NIP-17`, `NIP-44`, `NIP-46` — 3/3 |
| Channel priority (RULED 2026-09-01) | Nostr channel layer | canon §7.2(b) contains `Signal`, `White Noise`, `marmot-protocol/mdk` — 3/3 |
| Payments (CDK/Lightning) | Nostr channel layer | canon contains `Cashu`, `P2PK`, `NIP-57` — 3/3 |
| Skillsmith scope (RULED 2026-09-01) | Seven-crate design (spec) | canon §7.2(a) contains `Skillsmith`, `ELv2`, `2026-09-01` — 3/3 |
| Vendor denylist enforcement (Meta/OpenAI/xAI) | Seed gate files (94f0658) | `python vendor_gate.py --self-test` exits 0; ci.yml wires `--self-test`; vendor_gate.py `EXCEPTIONS` map empty; ci.yml carries exactly 9 `PIN_RESOLVE_IN_PHASE_0` placeholders — 4/4 |
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
