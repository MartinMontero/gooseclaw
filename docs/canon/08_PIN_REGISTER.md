# 08 — Pin Register (volatile — re-verify before relying)

Every load-bearing external dependency, its last verified state, and its source. **All of these move.** Re-check against the source before Phase 0 and before any bump; flag drift, don't silently fix. Last full verification: **2026-07-06** (QA pass 2).

| Surface | Pinned / expected | Verified state (as-of 2026-07-06) | Verdict | Source |
|---|---|---|---|---|
| **goose** release | v1.39.0 (source spec) | **v1.41.0** (assets 2026-07-03) | **Re-pin** | github.com/aaif-goose/goose/releases |
| goose ACP entry point | `goose acp` / `goose serve` | Exists; goose is an ACP server; `goose serve` = ACP over HTTP/WS | Confirmed | goose-docs.ai |
| goose permission modes | Approve vs Autonomous default | Modes are **Auto (default), Approve, Chat, Smart Approve**; force Approve | Confirmed | deepwiki.com/block/goose §6.2 |
| goose 2.0 behind ACP | seam stable | goose 2.0 beta re-architects clients behind ACP; goosed being removed | Confirmed | goose-docs.ai/blog/2026/04/08 |
| goose org / license | block/goose | Migrated to **aaif-goose/goose** (AAIF/LF, Dec 2025); **Apache-2.0** | Update URLs | github.com/aaif-goose/goose |
| **`agent-client-protocol`** (Rust) | 1.0.0, wire v1 | **1.0.0** current; protocol version **1** (`ProtocolVersion::V1`) | Confirmed | crates.io/crates/agent-client-protocol |
| ACP governance | Zed-created | Zed Industries (Aug 2025); JetBrains co-maintains; ACP Registry | Confirmed | agentclientprotocol.com |
| **MCP** spec | 2025-11-25 | Current stable = 2025-11-25 | Confirmed | modelcontextprotocol.io |
| MCP upcoming RC | ~2026-07-28 | RC **2026-07-28** in draft; stateless redesign; Tasks → extension | Watch (breaking) | blog.modelcontextprotocol.io |
| **NIP-17 / 44 / 59** | seal 13 / wrap 1059 / chat 14 | Kind numbers exact-correct; NIP-44 v2 **audited by Cure53** (Dec 2023) | Confirmed | github.com/nostr-protocol/nips; paulmillr/nip44 |
| NIP-46 bunker | remote signing | Confirmed; **still uses NIP-04 encryption** (open issue #1095) | Confirmed w/ caveat | github.com/nostr-protocol/nips/issues/1095 |
| NIP-57 / 61 / 32 / 56 | zaps / nutzaps / labels / reports | All confirmed in the NIPs repo | Confirmed | github.com/nostr-protocol/nips |
| NIP-04 | deprecated | Marked "unrecommended: deprecated in favor of NIP-17" | Confirmed | github.com/nostr-protocol/nips |
| **rust-nostr** | `nostr` / `nostr-sdk` | Supports NIP-17/44/46/59/61; **ALPHA — breaking API changes** | Confirmed w/ caveat | github.com/rust-nostr/nostr |
| **Cashu** NUT-11 / CDK | P2PK, `cdk` crate | NUT-11 supported; CDK self-labels **ALPHA "use with caution"** | Confirmed w/ caveat | github.com/cashubtc/cdk; github.com/cashubtc/nuts |
| **Wasmtime** | 43.0.1 (IronClaw) | Current **45.0.0** (2026-05-26); IronClaw main pins **43.0.2**; component model stable | Stale (version) | github.com/bytecodealliance/wasmtime/releases |
| **libSQL** | FTS5 + native vector | Native vector (F32_BLOB, DiskANN) + FTS5; embedded, no server | Confirmed | turso.tech/vector; github.com/tursodatabase/libsql |
| **Skillsmith** | smith-horn/skillsmith | Real; MCP skill-discovery server; local-first SQLite+FTS5; **Elastic-2.0** (not OSI-free) | Confirmed w/ caveat | github.com/smith-horn/skillsmith |
| **Zapstore** | desktop dist | Nostr app store; **desktop/CLI via zapstore-cli**; Android more mature | Confirmed | github.com/zapstore/zapstore-cli |
| **IronClaw** | ~78 crates, 3 wasm crates, commit 4c82051 | **~27 crates**; only `ironclaw_wasm` exists (not the 2 sub-crates); **4c82051 unverifiable** (latest 0.29.1 / 2026-06-04 / 556dfd0); license **MIT OR Apache-2.0** | **Wrong (partial)** — re-derive | github.com/nearai/ironclaw |
| **Block Goose Grant** | $100K / 12mo | Confirmed: $100K over 12 months, rolling, milestone-based, quarterly reviews; focus areas incl. self-flying + self-improving agents | Confirmed | github.com/aaif-goose/goose/discussions/3674 |
| **Trivy** (banned) | — | TeamPCP compromise 2026-03-19, **CVE-2026-33634** (CVSS 9.4); banned. Use Syft + OSV-Scanner + Grype | Banned | wiz.io; github advisories |

## Standing watch-list
1. **goose cadence** — v1.33 → v1.41 in ~10 weeks. Build against the ACP version, not goose internals.
2. **MCP 2026-07-28 RC** — stateless redesign + Tasks-as-extension could break the MCP-over-ACP path.
3. **rust-nostr + CDK ALPHA churn** — pin exact versions; expect breaking changes; budget for it.
4. **IronClaw v1→v2/Reborn transition** — issue #2193 plans deleting ~35k LOC of v1; pin a release, expect churn.
