## PR-B: gooseclaw-knowledge federation client + O5 cache semantics (Phase 2)

**Do not merge without Martin's word (Class B).** Companion: Alfred PR #47 (feat/dual-era-vault-mcp).

### What changed
- gooseclaw::supervise — core-owned supervised stdio JSON-RPC spawn (env_clear + kill_on_drop; satellites never spawn ambient processes).
- gooseclaw-knowledge::mcp — minimal MCP client: era probe per 2026-07-28 stdio rules (server/discover first; -32022 => dual-era-aware; ANY other error/timeout => legacy fallback, not keyed to one code), then legacy initialize at 2025-11-25.
- gooseclaw-knowledge::client — VaultClient: reads via vault_read; writes via vault_write with expectedVersion from the producing read; CONFLICT => local copy discarded + re-fetched, NEVER merged; O5 in code: CACHE_TTL = 60s; offline writes MUTE (no queue, no disk); offline reads cache-only with stale=true; first write after reconnect revalidates against the server.
- Canon: O5 row To-specify -> SPECIFIED (old text quoted in RECIPE.md change log).
- gate.ps1/RECIPE.md extended per DRIFT RULE: new row MCP-vault federation (Phase 2, executed); additive, no existing proof line changed. gate.ps1: 14/14 PASS.

### Acceptance gate (spec §8) — all EXECUTED
| Gate | Result | Evidence |
|---|---|---|
| 1. Conflict: human edit wins | PASS | read v1, out-of-band edit v2, write(v1) -> CONFLICT, disk stayed v2, cache discarded, re-read v2 — docs/audit/2026-10-29-phase2-federation/federation-probe.jsonl |
| 2. Alfred-offline degradation | PASS | write -> Muted; cached read stale=true; uncached read -> Offline; post-restart write revalidates first (live, same file) |
| 3. Dual-era on the wire | PASS | dual-era-wire-transcript.txt: server/discover -> DiscoverResult(supportedVersions ["2025-11-25"]) AND legacy initialize accepted at 2025-11-25, tools/list = 11 tools |
| 4. Workspace gates | PASS | check/fmt/clippy -D warnings/test clean (29 tests incl. 6 federation vs in-process fake server); vendor gate 179 pkgs 0 denied, EXCEPTIONS empty; gate.ps1 14/14 |
| 5. Audit docs | DONE | docs/audit/2026-10-29-phase2-federation/ |

### Claims (labeled)
- VERIFIED-LIVE (2026-10-29): @modelcontextprotocol/sdk latest 1.32.2 tops out at 2025-11-25 — dual-era constraint re-checked, still binding.
- VERIFIED-LIVE (2026-10-29): server/discover + UnsupportedProtocolVersionError (-32022) shapes from spec tag 2026-07-28.
- EXECUTED: live probe ran against the REAL Alfred server (feat/dual-era-vault-mcp) via node + tsx CLI through the core spawn seam.
- REPORTED (honest note): after revalidation, a whole-file agent write lands on top of the human's during-outage edit — fresh-base optimistic concurrency, not a stale clobber.
- UNVERIFIED (unchanged): goose loadSession not relied on; goose sidecar pin untouched (Martin's ruling).

### Stop line honored
No channels/payments/Nostr; no IronClaw vendoring; no loadSession reliance; no merge.