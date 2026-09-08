# PROBE REPORT — MCP-over-ACP re-validation against MCP spec 2026-07-28

**Date:** 2026-09-07 (as-of date for all VERIFIED-LIVE claims)
**Probe dir:** `docs/audit/2026-09-07-mcp-acp-revalidation/probe/`
**Rule 9:** honored. Nothing committed, pushed, or published. Local processes and local files only. [EXECUTED]

## 0. Versions and pins

| Claim | Value | Label |
|---|---|---|
| goose binary probed | **1.48.0** (`C:\Users\User\.local\bin\goose.exe`), `goose acp` present | EXECUTED |
| stage-goose-sidecar pin | 1.43.0 — **FLAGGED: differs from the 1.48.0 probed here.** Canon `[08R]` independently pins goose v1.48.0 (2026-08-27, re-pin pending ratification), so the sidecar pin is the outlier | CANON / FLAGGED |
| ACP wire protocolVersion | **1** (goose confirmed `protocolVersion: 1` in `initialize` result) | EXECUTED |
| MCP spec current revision | **2026-07-28** (directory and changelog live on `modelcontextprotocol/specification@main`) | VERIFIED-LIVE |
| `@modelcontextprotocol/sdk` latest | **1.30.0** (published 2026-07-27); `LATEST_PROTOCOL_VERSION = '2025-11-25'` — **the current TS SDK cannot build a 2026-07-28 server** | VERIFIED-LIVE |
| goose 1.48.0 MCP client era | **Legacy-era: sends `initialize` with `protocolVersion: "2025-11-25"`**, capabilities `roots`/`sampling`/`elicitation` | EXECUTED (wire, §4.1) |

## 1. SPEC DELTA — what 2026-07-28 changed for a client calling a server

Primary sources (all fetched 2026-09-07, quoted verbatim):
- `github.com/modelcontextprotocol/specification@main:docs/specification/2026-07-28/changelog.mdx`
- `.../basic/transports/stdio.mdx`, `.../server/discover.mdx`, `.../basic/versioning.mdx`, `.../server/tools.mdx`

### 1.1 The breaking changes [VERIFIED-LIVE, quoted from changelog.mdx]

> "1. Remove protocol-level sessions and the `Mcp-Session-Id` header from the Streamable HTTP transport. … Servers that need cross-call state use explicit, server-minted handles passed as ordinary tool arguments (SEP-2567)."

> "2. Make MCP stateless: remove the `initialize`/`notifications/initialized` handshake. Every request now carries its protocol version and client capabilities in `_meta` (`io.modelcontextprotocol/protocolVersion`, `io.modelcontextprotocol/clientCapabilities`). … Version mismatches return `UnsupportedProtocolVersionError` (SEP-2575)."

> "3. Add `server/discover`: servers MUST implement this RPC to advertise their supported protocol versions, capabilities, and identity. Clients MAY call it before any other request for up-front version selection, or use it as a backward-compatibility probe on STDIO (SEP-2575)."

Also breaking for consumers: `ping`, `logging/setLevel`, `notifications/roots/list_changed` removed (item 5); Tasks moved to extension `io.modelcontextprotocol/tasks` (item 6, SEP-2663); MRTR `InputRequiredResult` replaces server-initiated requests (item 7, SEP-2322); **all results carry required `resultType`** with legacy-omission treated as `"complete"` (item 8); SSE resumability/`Last-Event-ID` removed (item 9). Minor: `ttlMs`/`cacheScope` **required** on `tools/list` etc. via `CacheableResult` (SEP-2549); `Mcp-Method`/`Mcp-Name` headers required on HTTP; `UnsupportedProtocolVersion` = `-32022`. Deprecated: Roots, Sampling, Logging (SEP-2577, 12-month window).

The canon's `[08R]` re-validation note ("no `initialize`/`Mcp-Session-Id`; per-request `_meta`; `server/discover`; Tasks-as-extension (SEP-2663); Roots/Sampling/Logging deprecated") is **accurate against the shipped changelog**. [CANON verified against VERIFIED-LIVE source]

### 1.2 What a consumer MUST adapt to (stdio, which is the GooseClaw shape)

From `stdio.mdx` + `versioning.mdx` [VERIFIED-LIVE, quoted]:

- No handshake. "All request metadata for the stdio transport is carried inline in the JSON-RPC message body. … There is no header layer."
- "Servers **MUST** implement it [`server/discover`]."
- The compatibility matrix (`versioning.mdx`, "Backward Compatibility with Initialization-Based Versions") — **this is where the canon assumption lives or dies**:

> "| Legacy   | Modern   | Fails. stdio: the server rejects `initialize` with a JSON-RPC error; the exact code is implementation-defined (`initialize` is an unknown method and the request also lacks the required `_meta` fields). … **Legacy clients have no fall-forward mechanism.** |"
> "| Legacy   | Dual-era | Works. The server answers `initialize` and serves the client according to the negotiated legacy revision. |"

So: against a **modern-only** 2026-07-28 server, a legacy client breaks on the first RPC. Against a **dual-era** server, a legacy client works unchanged. The federation question reduces to: **which era is goose 1.48.0's MCP client?** — answered on the wire below.

## 2. PROBE DESIGN [EXECUTED]

Because the current TS SDK (1.30.0) tops out at 2025-11-25 (§0), the server was hand-rolled directly to the spec wire (newline-delimited JSON-RPC over stdio — the stdio.mdx framing) in `probe/vault-server.mjs`. It implements the canon's §3.2 semantics: path confinement, reads free, writes only via tool, **single-writer optimistic concurrency** (`expectedVersion` = read-version / write-if-unchanged; conflict → other writer wins). It logs every wire line verbatim to `wire-mcp.ndjson`. Two configurations:

- **Probe A — strict modern:** no `initialize` (unknown method, `-32601`); every request requires `_meta.io.modelcontextprotocol/protocolVersion: "2026-07-28"` else `UnsupportedProtocolVersionError -32022`; `server/discover`; `resultType`/`ttlMs`/`cacheScope` on results.
- **Probe B — dual-era:** same server also answers `initialize` and serves 2025-11-25 (legacy result shapes), exactly as `versioning.mdx` prescribes for dual-era servers.

Driver: `probe/acp-probe.mjs` spawns `goose acp` (real subprocess), speaks ACP wire v1 over stdio: `initialize {protocolVersion: 1}` → `session/new` with the vault server as a stdio MCP server → three `session/prompt`s (vault_read; vault_write with current `expectedVersion`; vault_write with stale `expectedVersion`). Approve mode forced via `GOOSE_MODE=approve`; every `session/request_permission` answered `allow_once` by the probe harness (standing in for the human). Telemetry disabled (`GOOSE_TELEMETRY_ENABLED=false`) per sovereignty rule. Full ACP wire logged to `wire-acp.ndjson`. LLM: host-configured `kimi_code`/`k3` (Moonshot — not on the Meta/OpenAI/xAI denylist).

## 3. VERDICT

**The canon's MCP-over-ACP assumption BREAKS at exactly one seam: the protocol-era handshake.** Everything else — the ACP contract, forced Approve, the §3.2 call shape (list tools → vault read → vault write with sole-writer optimistic concurrency), human-in-the-loop permission per call — **WORKS**, but only on the **legacy revision (2025-11-25)**, because goose 1.48.0's MCP client is legacy-era.

Precisely:

1. **BREAKS** — goose 1.48.0 over ACP → strict 2026-07-28 server. Break location: the **first RPC**, `initialize`. Spec section: `specification/2026-07-28/basic/versioning.mdx`, "Backward Compatibility with Initialization-Based Versions", matrix row *Legacy client → Modern server = Fails*; reinforced by `basic/transports/stdio.mdx` ("Backward Compatibility") and `changelog.mdx` item 2. Observed vs expected in §4.1.
2. **WORKS** — goose 1.48.0 over ACP → dual-era server (what Alfred must therefore be). Full canon call shape executed; transcripts in §4.2.

Consequence for the canon: §3.2's federation contract is sound, but it must **pin Alfred's MCP server to dual-era operation** (answer `initialize`, serve 2025-11-25; additionally implement `server/discover` + modern mode for future clients) **until goose ships a 2026-07-28-capable MCP client**, at which point the constraint can be relaxed. Note the ecosystem lag: even the official TS SDK (1.30.0, latest) cannot yet build a modern-only server, so a dual-era Alfred is currently the *only* buildable conformant option on the SDK anyway. The fix as a ready diff is in §5.

## 4. EVIDENCE TRANSCRIPTS [EXECUTED]

### 4.1 Probe A — strict modern server → BREAKS

ACP `session/new` result `_meta.extensionResults` (verbatim from `wire-acp-probeA-strict.ndjson`):

```json
{"name":"alfred-vault","success":false,"error":"process quit before initialization: stderr = "}
```

MCP wire (verbatim, `wire-mcp-probeA-strict.ndjson`) — the entire conversation:

```
client->server {"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-11-25",
  "capabilities":{"extensions":{},"roots":{},"sampling":{},"elicitation":{}},
  "clientInfo":{"name":"gooseclaw-acp-probe","version":"0.1.0"}}}
server->client {"jsonrpc":"2.0","id":0,"error":{"code":-32601,
  "message":"Method not found: initialize (removed in 2026-07-28; use server/discover + per-request _meta)"}}
```

- **Expected** (if the assumption held against the current spec): goose sends `server/discover` or any request carrying `_meta.io.modelcontextprotocol/protocolVersion: "2026-07-28"`, per `changelog.mdx` items 2–3.
- **Observed**: goose sends legacy `initialize` with `protocolVersion: "2025-11-25"` and legacy-era capabilities (`roots`, `sampling`, `elicitation` — all deprecated/removed in 2026-07-28). The conformant modern server rejects it; goose marks the extension failed and the session proceeds **without the vault tools** (the model then honestly reports `vault_read`/`vault_write` unavailable; no fabrication — see probe console log).

### 4.2 Probe B — dual-era server → WORKS

MCP wire (verbatim, `wire-mcp.ndjson`, timestamps UTC):

```
22:30:41.527 client->server initialize {protocolVersion:"2025-11-25", capabilities:{extensions,roots,sampling,elicitation}, ...}
22:30:41.528 server->client result {protocolVersion:"2025-11-25", capabilities:{tools:{}}, serverInfo:{name:"alfred-vault-dual",version:"0.1.0"}, ...}
22:30:41.529 client->server notifications/initialized
22:30:41.707 client->server tools/list {_meta:{agent-session-id:"20260907_10", progressToken:0}}
22:30:41.708 server->client result {tools:[vault_read, vault_write]}
22:30:47.533 client->server tools/call vault_read {path:"day1.md"}
22:30:47.534 server->client result {content:[...version=1...], structuredContent:{path:"day1.md",version:1,text:"canon probe note v1\n"}}
22:30:59.743 client->server tools/call vault_write {path:"day1.md", text:"canon probe note v2 (written by goose over ACP)\n", expectedVersion:1}
22:30:59.744 server->client result {content:["WROTE day1.md at version 2"], structuredContent:{path:"day1.md",version:2}}
22:31:11.648 client->server tools/call vault_write {path:"day1.md", text:"stale overwrite attempt\n", expectedVersion:1}   // STALE
22:31:11.649 server->client result {isError:true, content:["CONFLICT: day1.md is at version 2, expectedVersion=1. Other writer wins; re-read and retry."], structuredContent:{conflict:true,currentVersion:2}}
```

On-disk vault state after the run: `day1.md` = "canon probe note v2 (written by goose over ACP)", `.versions.json` = `{"day1.md":2}` — the stale write did not land. [EXECUTED]

ACP wire (verbatim, `wire-acp.ndjson`) — the canon §3.1 contract holding:

```
initialize      -> result {protocolVersion: 1, agentInfo:{name:"goose",version:"1.48.0"}}
session/new     -> result {sessionId:"20260907_10", modes.currentModeId:"approve",
                           availableModes:[auto, approve, smart_approve, chat],
                           _meta.extensionResults:[..., {"name":"alfred-vault", success:true} implied by tool calls]}
22:30:47.528 session/request_permission {toolCall:{title:"alfred-vault: vault read · day1.md"}, options:[allow_always, allow_once, reject_once, reject_always]}
22:30:47.529 <- {outcome:{outcome:"selected", optionId:"allow_once"}}
22:30:59.741 session/request_permission {toolCall:{title:"alfred-vault: vault write · day1.md", rawInput:{...,expectedVersion:1}}}
22:30:59.742 <- allow_once
22:31:11.647 session/request_permission {toolCall:{title:"alfred-vault: vault write · day1.md", rawInput:{expectedVersion:1,...}}}
22:31:11.647 <- allow_once
```

Every tool call — read and both writes — was gated by a `session/request_permission` round-trip **before** the MCP `tools/call` hit the wire (compare timestamps: permission 22:30:47.528–.529 precedes MCP call 22:30:47.533). Forced Approve mode is real and observable at the ACP seam. [EXECUTED]

### 4.3 Canon §3.1 rows re-verified in passing

| §3.1 row | Result | Label |
|---|---|---|
| Spawn `goose acp` as supervised subprocess, stdio/JSON-RPC | Confirmed | EXECUTED |
| Wire `protocolVersion = 1` stable | Confirmed (agent answered 1) | EXECUTED |
| Force Approve (override Auto default) | Confirmed: `GOOSE_MODE=approve` → `currentModeId:"approve"`; per-call permission requests | EXECUTED |
| Approve-mode caveat: no upstream hard lock against mid-session `/mode` change | Consistent — `session/set_mode` exists (`availableModes` advertised); GooseClaw must keep enforcing at its own gate. Not adversarially tested here | REPORTED (canon) / UNVERIFIED (this probe) |

## 5. READY DIFF — specification.md §3.2 fix (for Martin's ratification; not applied)

```diff
 ### 3.2 Alfred ⇄ GooseClaw — the MCP-vault contract (Option C)
 
 CANON `[04 §3] [03]`:
 
 - Alfred (target repo `github.com/MartinMontero/Alfred`, AGPL-3.0) exposes its path-confined, Zod-validated MCP tools as GooseClaw's **sovereign memory substrate**; Alfred is system-of-record, GooseClaw holds only ephemeral state + cache.
 - **Reads** free within the path root. **Writes go only through Alfred's MCP tools** — Alfred is the single path-traversal enforcement point; GooseClaw never writes the vault on disk directly.
 - **Single-writer with optimistic concurrency** (read-version / write-if-unchanged): **a human editing in Alfred always wins** conflicts.
 - *To specify (Phase 2):* cache staleness bounds; reconnect/conflict semantics when Alfred is offline `[04 §3]`.
-- **Re-validation note [08R]:** MCP spec **2026-07-28 shipped FINAL** with the stateless redesign (no `initialize`/`Mcp-Session-Id`; per-request `_meta`; `server/discover`) and Tasks-as-extension (SEP-2663); Roots/Sampling/Logging deprecated. The MCP-vault contract and any MCP-over-ACP assumptions **must be re-validated against spec 2026-07-28 before Phase 2**.
+- **Re-validation EXECUTED 2026-09-07** (`docs/audit/2026-09-07-mcp-acp-revalidation/`): the [08R] description of spec **2026-07-28** is **confirmed accurate** against the shipped changelog. Probe verdict against goose **1.48.0**:
+  - **BREAKS (modern-only server):** goose 1.48.0's MCP client is **legacy-era** — it opens with `initialize` / `protocolVersion: "2025-11-25"` (capabilities `roots`/`sampling`/`elicitation`). A strict 2026-07-28 server rejects `initialize` (`-32601`, unknown method) and the extension fails to load. Spec locus: `basic/versioning.mdx` compatibility matrix, *Legacy client → Modern server = Fails*; "legacy clients have no fall-forward mechanism."
+  - **WORKS (dual-era server):** the full §3.2 call shape executed live over ACP v1 in forced Approve mode — `tools/list`, `vault_read`, `vault_write` with `expectedVersion` (v1→v2), and a stale write rejected with CONFLICT (human-edit-wins). Every tool call was preceded by an ACP `session/request_permission` round-trip.
+  - **RULED constraint:** Alfred's MCP server **MUST be dual-era** — answer `initialize` and serve **2025-11-25** for goose, AND implement `server/discover` + modern per-request-`_meta` mode for 2026-07-28 clients — until goose ships a 2026-07-28-capable MCP client (re-check each goose bump per [08R] pin rules). Note: `@modelcontextprotocol/sdk` latest (1.30.0, 2026-07-27) tops out at 2025-11-25, so dual-era is currently the only SDK-buildable conformant target regardless.
+  - Side note: stage-goose-sidecar pin **1.43.0 ≠ probed 1.48.0** — flagged per pin rules; sidecar pin needs re-verification before Phase 2.
```

## 6. What was NOT verified [UNVERIFIED]

- goose's HTTP/WS MCP transports (`streamable-http` path with `Mcp-Method`/`Mcp-Name` headers) — probe used stdio only, matching §3.2's local-vault shape.
- `subscriptions/listen`, MRTR `input_required`, Tasks extension — not exercised; the §3.2 call shape doesn't need them.
- Mid-session mode-change adversarial test (§3.1 caveat) — out of scope, recorded as still-open.
- Behavior of goose 1.43.0 (the sidecar pin) — not run; if the sidecar must stay on 1.43.0, this probe should be re-run against it before Phase 2.

## 7. Artifacts

- `probe/vault-server.mjs` — dual-era/strict MCP vault server (spec-conformant wire, hand-rolled; SDK 1.30.0 present in `node_modules` but incapable of 2026-07-28)
- `probe/acp-probe.mjs` — ACP v1 driver harness (Approve mode, permission auto-approve-as-human)
- `probe/wire-acp.ndjson`, `probe/wire-mcp.ndjson` — Probe B transcripts (WORKS)
- `probe/wire-acp-probeA-strict.ndjson`, `probe/wire-mcp-probeA-strict.ndjson` — Probe A transcripts (BREAKS)
- `probe/vault/` — post-run vault state (`day1.md` v2, conflict rejected)
