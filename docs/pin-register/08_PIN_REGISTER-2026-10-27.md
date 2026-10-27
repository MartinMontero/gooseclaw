# 08 — Pin Register (REFRESHED 2026-10-27 — supersedes as-of dates, preserves prior column for audit)

Re-verified against primary sources on **2026-10-27** (BuilderOS drift-and-refresh sweep, GOOSECLAW lane). Prior full verification: 2026-09-03 (see `08_PIN_REGISTER-2026-09-03.md`), before that 2026-07-06 (QA pass 2).
Rule: drift is **flagged, never silently fixed** — the prior column below is the 2026-09-03 record; the current column is today's primary-source state. **Re-pin decisions are Martin's, recorded in `03_DECISION_LOG.md`.** This refresh changes no pins.

New this refresh: every checkable row is now machine-checkable — `scripts/pin_drift_check.py` resolves `pin-snapshot-2026-10-27.json` against live upstream and exits 1 on any drift (fail-first proven against the 2026-09-03 values: 8/10 DRIFTED, exit 1).

| Surface | Prior state (as-of 2026-09-03) | Verified state (as-of 2026-10-27) | Verdict | Primary source |
|---|---|---|---|---|
| **goose** release | v1.48.0 (2026-08-27) | **v1.56.0 (2026-10-23)** — 8 releases in ~8 weeks; the cadence watch-list item continues | **DRIFT — re-pin (Martin's call)** | github.com/aaif-goose/goose/releases/tag/v1.56.0 |
| goose org / license | aaif-goose/goose; Apache-2.0 | Confirmed (release stream continues under aaif-goose/goose) | Confirmed | github.com/aaif-goose/goose |
| goose ACP entry point / permission modes | `goose acp` stdio; `goose serve` HTTP/WS; 4 modes, default Auto, force Approve; no documented hard lock vs mid-session `/mode` | Not re-verified this sweep (docs surface, not API-resolvable from this environment) | UNVERIFIED (prior stands) | goose-docs.ai |
| **`agent-client-protocol`** (Rust) | Crate 2.0.0 (2026-07-23); wire v1 stable | **Crate now 3.5.0 (2026-10-22); 3.0.0 landed 2026-10-06 — a second major bump in ~10 weeks.** Whether `ProtocolVersion::V1` remains the stable wire version under 3.x was NOT verifiable from this environment | **DRIFT (crate 2.x→3.x); wire version under 3.x UNVERIFIED — check before Phase 0 locks the ACP dependency** | crates.io/crates/agent-client-protocol |
| ACP governance | Zed + JetBrains joint; Registry | Not re-verified this sweep | UNVERIFIED (prior stands) | agentclientprotocol.com |
| **MCP** spec | 2026-07-28 current stable (stateless redesign shipped) | Confirmed: `2026-07-28` remains the latest spec tag; no newer stable | Confirmed | github.com/modelcontextprotocol/specification/tags |
| **NIP-17 / 44 / 59** | kinds exact; NIP-44 v2 Cure53-audited; kind 21059 documented | Confirmed (NIPs repo; no drift observed) | Confirmed | github.com/nostr-protocol/nips |
| NIP-46 bunker | kind 24133 content now NIP-44-encrypted; issue #1095 still open | Confirmed: current 46.md carries nip44 encrypt/decrypt (nip04 retained as legacy) | Confirmed | github.com/nostr-protocol/nips/blob/master/46.md |
| NIP-57 / 61 / 32 / 56 / NIP-04 | present / deprecated | Confirmed present; no drift observed | Confirmed | github.com/nostr-protocol/nips |
| **rust-nostr** (now nostrdevkit/nostr) | NIP-61 NOT implemented; `nostr` 0.45.4, `nostr-sdk` 0.45.2; ALPHA | **`nostr` 0.45.5 (2026-09-11), `nostr-sdk` 0.45.4 (2026-09-19)**; repo nostrdevkit/nostr active, not archived; NIP-61 gap not re-checked | **DRIFT (minor versions); NIP-61 gap UNVERIFIED this sweep** | crates.io/crates/nostr; crates.io/crates/nostr-sdk; github.com/nostrdevkit/nostr |
| **Cashu** NUT-11 / CDK | 0.18.0 (2026-09-02); ALPHA | **0.18.2 (crates.io 2026-10-14)**; ALPHA label not re-checked | **DRIFT (minor)** | crates.io/crates/cdk |
| **Wasmtime** | current 48.0.1 (2026-08-24); IronClaw main pins 47.0.4 | **current v50.0.0 (2026-10-20)**; **IronClaw main still pins 47.0.4** (confirmed in main Cargo.toml) | **DRIFT (upstream); IronClaw pin Confirmed** | github.com/bytecodealliance/wasmtime/releases; github.com/nearai/ironclaw/blob/main/Cargo.toml |
| **libSQL** | FTS5 + native vector confirmed; maintenance-mode banner ("new features are being developed in Turso") | Confirmed: banner still present on libSQL README; maintenance posture unchanged | Confirmed (caveat stands) | github.com/tursodatabase/libsql |
| **Skillsmith** | active; Elastic-2.0 | Repo active (pushed 2026-10-27), not archived; GitHub license detection returns NOASSERTION (Elastic-2.0 is non-SPDX-detectable — no evidence of a license change, but not positively confirmed) | Confirmed w/ caveat (license UNVERIFIED this sweep) | github.com/smith-horn/skillsmith |
| **Zapstore** | desktop/CLI via zapstore-cli | **MAJOR DRIFT — `github.com/zapstore/zapstore-cli` now 404s and is absent from the zapstore org repo list** (org is live; repos: zapstore, zsp, relay, webapp, goapk, defender, hotbox, steroid). Where the CLI went (renamed, folded into `zapstore/zapstore`, or dropped) is UNVERIFIED. **The Zapstore distribution pin (03_DECISION_LOG ratified spine: distribution via Zapstore/zapstore-cli) is BROKEN pending Martin** | **DRIFT (major) — distribution path** | github.com/zapstore (org repo list) |
| **IronClaw** | 1.4.0 (2026-08-27, 4cb47cf); ~68 crates; Reborn completed; MIT OR Apache-2.0 | **Latest release ironclaw-v1.4.1 (2026-09-29, b011eb3)**; wasmtime pin 47.0.4 confirmed in main Cargo.toml; crate count / `ironclaw_wasm_*` layout not re-derived this sweep | **DRIFT (minor release); steals provenance re-derivation still owed before any port** | github.com/nearai/ironclaw/releases |
| **Block Goose Grant** | $100K / 12mo, rolling, milestone, quarterly; block.github.io/goose/grants | Grant discussion **aaif-goose/goose#3674 still OPEN** (titled "goose grant program"). The grants page was NOT reachable from this environment (curl HTTP 000 to both block.github.io and aaif-goose.github.io) — parameters UNVERIFIED live | Confirmed (program exists) / parameters UNVERIFIED | github.com/aaif-goose/goose/discussions/3674 |
| **Trivy** (banned) | CVE-2026-33634, CISA KEV 2026-03-26; ban stands | Not re-verified this sweep; no reason to revisit — ban stands | Confirmed (prior stands) | github.com/advisories (CVE-2026-33634) |
| **White Noise / Marmot** | whitenoise.chat on Marmot; crates at marmot-protocol/mdk; whitenoise-rs obsolete | Confirmed: marmot-protocol/mdk active (pushed 2026-10-27) | Confirmed | github.com/marmot-protocol/mdk |

## Standing watch-list (updated 2026-10-27)
1. **goose cadence** — v1.48 → v1.56 in ~8 weeks; now 8 releases behind the last register state. Build against the ACP wire version, not goose internals. (But see 2.)
2. **ACP crate 3.x** — the wire-version assumption (`ProtocolVersion::V1` stable) was asserted under crate 2.0.0 and is UNVERIFIED under 3.5.0. This is now the highest-priority pin risk: GooseClaw's whole seam is ACP wire v1. Verify before Phase 0.
3. ~~MCP 2026-07-28 RC~~ — shipped; still the latest stable tag (no newer drift this period). The dual-era Alfred MCP constraint (03 §Post-spine closures) remains the ratified mitigation.
4. **rust-nostr (nostrdevkit) + CDK ALPHA churn** — minor bumps this period; NIP-61 nutzap gap identified 2026-09-03 not re-checked.
5. **IronClaw post-Reborn** — 1.4.1 out; re-derive the steals section against a pinned 1.x commit (04 §4 + amendment 4 of 10) before any port. Still owed.
6. **libSQL maintenance mode** — banner unchanged; evaluate before Phase 0 locks the persistence pin.
7. **NEW: Zapstore distribution path broken** — zapstore-cli repo gone; the ratified distribution decision (Zapstore) needs a live target or a re-ruling. Martin's call.

## Unverifiable residue (stated, not worked around)
- ACP wire protocol version under crate 3.x (docs.rs unreachable for method-level check from this environment).
- Where zapstore-cli went (no redirect observed; org repo list simply lacks it).
- Block Goose Grant parameters ($100K/12mo) — grants page unreachable (HTTP 000); discussion #3674 confirmed open only.
- goose permission-mode hard-lock caveat (docs surface, not re-verified).
- Skillsmith license text (GitHub detection: NOASSERTION; Elastic-2.0 is not SPDX-detectable, so this may be detection, not change).
- NIP-61 implementation status in nostrdevkit/nostr (not re-checked).
