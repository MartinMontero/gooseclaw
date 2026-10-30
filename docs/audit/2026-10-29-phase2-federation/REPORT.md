# Phase 2 — Alfred MCP-vault federation — 2026-10-29

Two repos, two PRs, one logical change each. Class A only; merges are Martin's.

## PR-A (Alfred, feat/dual-era-vault-mcp -> main, PR #47)
- mcp/dual-era.ts: transport shim — server/discover answered with DiscoverResult
  (supportedVersions ["2025-11-25"]); modern per-request _meta at unserved versions ->
  UnsupportedProtocolVersionError (-32022, data {requested, supported}); everything else
  passes through to the SDK (legacy initialize path untouched).
- vault-fs.ts: versionOf (sha256 content token; null when absent).
- server.ts: expectedVersion on every read result; all five write tools require it;
  stale -> CONFLICT, vault untouched (human-edit-wins); null = create-only.
  Tool roster + path confinement + poisoning chokepoint untouched.
- Failure-proof (pre-change behavior, EXECUTED): reads carried no version; blind write
  silently clobbered an out-of-band human edit; server/discover -> -32601.
  Probe: failproof-probe.mts output: expectedVersion=false; isError=false + clobbered;
  -32601.
- After: 53/53 mcp tests (8 new), tsc clean, build:mcp clean.

## PR-B (GooseClaw, phase-2-federation -> main)
- gooseclaw::supervise — core-owned JSON-RPC subprocess spawn (env_clear + kill_on_drop).
- gooseclaw-knowledge::mcp — minimal MCP client; era probe per 2026-07-28 stdio rules
  (server/discover first; -32022 => dual-era; anything else => legacy fallback, NOT
  keyed to one error code); legacy initialize at 2025-11-25.
- gooseclaw-knowledge::client — VaultClient: reads within the vault, writes via tools
  with expectedVersion; CONFLICT => discard + re-fetch, never merge; O5 semantics
  (TTL 60s; offline MUTE; stale-flagged cached reads; revalidate-before-write after
  reconnect).

## Acceptance gate (spec §8) — all EXECUTED live against the real Alfred server
1. Conflict test — PASS (federation-probe.jsonl): read v1, out-of-band human edit v2,
   write(v1) -> CONFLICT, disk stayed v2, cache discarded, re-read v2. No silent overwrite.
2. Alfred-offline — PASS: write -> Muted (no queue/disk), cached read stale=true,
   uncached read -> Offline; restart + human edit during outage -> first write
   revalidated (read precedes write), then landed.
   Note (honest): after revalidation the agent's whole-file write lands ON TOP of the
   human's v4 — that is fresh-base optimistic concurrency, not a stale clobber.
3. Dual-era on the wire — PASS (dual-era-wire-transcript.txt): server/discover answered
   (DiscoverResult, supportedVersions ["2025-11-25"]) AND legacy initialize accepted at
   2025-11-25, tools/list = 11 tools.
4. Workspace gates: check/fmt/clippy -D warnings/test clean (incl. 6 federation tests
   vs in-process fake server); vendor gate 179 packages 0 denied, EXCEPTIONS empty;
   gate.ps1 extended to 14 subsystems.
5. This audit dir. Nothing committed to main; PRs open, not merged.

## Step-0 verifications (all 2026-10-29)
- @modelcontextprotocol/sdk latest 1.32.2; SUPPORTED_PROTOCOL_VERSIONS tops at
  2025-11-25 (types.js:2-4) — dual-era constraint re-checked, still binding. VERIFIED-LIVE.
- server/discover + -32022 shapes from spec tag 2026-07-28 schema + discover.mdx.
  VERIFIED-LIVE.
- stdio era-probe fallback rules from spec 2026-07-28 basic/transports/stdio.mdx.
  VERIFIED-LIVE.
- MCP 2026-07-28 remains the latest FULL release. VERIFIED-LIVE (pin register sweep
  2026-10-27, re-confirmed).
