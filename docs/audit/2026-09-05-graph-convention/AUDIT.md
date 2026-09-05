# AUDIT.md — Graph-engineering convention install, Step 1.5 redo (2026-09-05)

Labels: EXECUTED (ran it) / VERIFIED-LIVE (ran against the real artifact) / CANON (canonical
source) / REPORTED (asserted by Martin, not independently verified) / UNVERIFIED (could not check).

## Substrate (the redo)

The previous attempt ran against a **bare canon-drop directory** — no git repo, no seed files —
and its task-premise drift flags were correct for that substrate. This run targets the real
seed repo. The previous attempt's outputs are quarantined at
`C:\Users\User\dev\gooseclaw-preexisting\` (nothing deleted, nothing shipped).

VERIFIED-LIVE (EXECUTED: `git clone`, `git log`, `git ls-tree -r --name-only HEAD`):

| Artifact | Present | Notes |
|---|---|---|
| Repo `github.com/MartinMontero/gooseclaw` | yes | cloned; main = `94f0658` "Add files via upload" |
| Tree @ 94f0658 | exactly 3 files | `CLAUDE.md`, `ci.yml`, `vendor_gate.py` — matches the expected tree; verified before any work |
| `CLAUDE.md` | yes | CANON `[CL]` — TIER 1 hard rules, TIER 2 project, Commands |
| `ci.yml` | yes | CANON `[CI]` — vendor-gate required check, 9 `PIN_RESOLVE_IN_PHASE_0` placeholders |
| `vendor_gate.py` | yes | CANON `[VG]` — stdlib-only, `--self-test` fixture, `EXCEPTIONS` empty |
| Assembled canon `specification.md` | yes (outside repo) | CANON — see drift flag 3; **not committed, by design** |

## String-level verification of proof targets (EXECUTED: Select-String, all 34 tokens)

All 34 proof tokens HIT — canon (`fiduciary agent runtime`, `AGPL-3.0-or-later`,
`protocolVersion = 1`, `Approve`, `Single-writer`, `system-of-record`, `libSQL`, `FTS5`,
`native vector`, `NIP-17`, `NIP-44`, `NIP-46`, `Skillsmith`, `ELv2`, `2026-09-01`, `Signal`,
`White Noise`, `marmot-protocol/mdk`, `Cashu`, `P2PK`, `NIP-57`, `core-owned`,
`one enforcement point`); CLAUDE.md (`Rule 9`, `go-ahead`, `TIER 1`, seven crate names);
ci.yml (`--self-test`); vendor_gate.py (`EXCEPTIONS: dict[str, str] = {}`).

`PIN_RESOLVE_IN_PHASE_0` occurrences in ci.yml: **9** (EXECUTED, regex count) — **matches**
canon §6.2 exactly. No drift.

## Rulings: upgraded from REPORTED to CANON

In the wrong-substrate attempt, Skillsmith scope and channel priority were REPORTED-only
(image-only canon PDFs, no decision log in tree). In this substrate they are **CANON-verified**
against `specification.md` §7.2 `[D0901]` (EXECUTED: Select-String):

- **Skillsmith IN for v1**, external-only ELv2 posture via `npx` as an external MCP server;
  tripwire: no BuilderOS-hosted registry without smith-horn's written permission.
- **Channel priority Nostr → Signal → White Noise**, White Noise via `marmot-protocol/mdk`
  (not the obsolete `whitenoise-rs`).

Per amendment (a) there is **no DECISIONS.md** — proofs target the canon directly.

## Drift flags (surfaced, not silently fixed)

1. **`scripts/vendor_gate.py` path mismatch.** ci.yml runs `python3 scripts/vendor_gate.py`
   and CLAUDE.md Commands references `scripts/vendor_gate.py`, but the committed file sits at
   repo **root** (`vendor_gate.py`). FLAGGED — not moved: relocating the seed file is a
   Phase-0 scaffolding decision for Martin. `gate.ps1` delegates to the file where it is
   committed. (When CI reaches a scaffolded workspace the checkout action placeholders fail
   first anyway — see flag 4.)
2. **Wrong-substrate predecessor.** Prior attempt's AUDIT/PLAN/DECISIONS/RECIPE/gate/PR_BODY
   and its own CLAUDE.md are quarantined outside the repo; none of it ships. Its substance
   was re-derived here against the real artifacts, with amendments (a)–(c) applied.
3. **Canon path.** The brief named `C:\Users\User\dev\specification.md`; it was absent there.
   Byte-identical copies (`fc /b`: no differences, 34,975 bytes) existed in `Downloads\` and
   `Desktop\kc phase 0 results\Holmes\`; the Downloads copy was placed at the stated path.
   The canon stays **outside the repo** and is never committed; `gate.ps1` reads
   `..\specification.md`.
4. **GitHub CI cannot go green yet — by design.** The 9 `PIN_RESOLVE_IN_PHASE_0`
   placeholders are deliberately invalid so CI fails loudly rather than run on a fabricated
   hash (canon §6.2 `[CI]`). "Green" for this PR = the local definition-of-done gate
   (evidence in the PR body). Resolving the pins is Phase-0 work with primary-source
   checksum verification.
5. **Canon C6 (pre-existing, cosmetic):** vendor_gate.py's self-test fixture cites
   `github.com/rust-nostr/nostr`; the repo moved to `nostrdevkit/nostr` `[08R]`. Matcher
   unaffected; canon schedules the fix at Phase-0 pin resolution. Not touched here.

## Dependency evidence (CANON — from canon text, not guesses)

- Canon §2: `-channels` and `-payments` **call into** the core gate; exactly one enforcement
  point `[02 §12] [BC]` → the Rule-9 gate row sits under the seven-crate design, and the ACP
  contract under the gate (gate is enforced PreToolUse around every consequential call).
- Canon §3.2: writes go only through Alfred's MCP tools; single-writer optimistic
  concurrency → persistence rides the federation seam.
- Canon §3.3/§3.4: NIP-57/61 are Nostr protocols → payments need the Nostr layer.
- Canon §7.2: both 2026-09-01 rulings are recorded with provenance `[D0901]` → channel
  priority and Skillsmith rows target the canon, not a second register.

## Tooling (VERIFIED-LIVE)

- Windows PowerShell 5.1.26100.9278 — gate.ps1 targets this: ASCII-only, no `&&`, no POSIX.
- Python 3.13.14 (`python` and `python3` both resolve) — runs `vendor_gate.py --self-test`.
- git 2.54.0; gh 2.98.0 authenticated as MartinMontero (scopes: repo, workflow).

## Denylist

No Meta/OpenAI/xAI dependency, import, or endpoint introduced — direct or transitive.
Deliverables are markdown plus one dependency-free PowerShell script. The vendor gate's own
self-test passes (evidence in the PR body); its `EXCEPTIONS` map remains deliberately empty.
