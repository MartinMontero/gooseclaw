# Phase 0 verification — 2026-10-27

Method: installed-binary inspection + primary-source checks (gh api, crates.io API,
raw.githubusercontent). Nothing committed or pushed. Sparse study-clone of nearai/ironclaw
at b011eb3932a0e1ab8bbe833893599ca9ae4925c5 under reference/ironclaw (read-only).

## Result table

| surface | expected (kickoff) | actual (verified) | source | status |
|---|---|---|---|---|
| goose installed | v1.41.0 | 1.48.0 | goose --version | DRIFT |
| goose upstream | v1.41.0 | v1.56.0 (2026-10-23) | block/goose releases | DRIFT |
| ACP entry | goose acp vs serve | goose acp (stdio); goose serve (HTTP/WS) | goose --help 1.48.0 | OK |
| Approve mechanism | unknown flag | GOOSE_MODE env (auto/approve/chat/smart_approve; default auto) | block/goose config-files.md @ v1.48.0 | OK |
| ACP crate | 1.0.0 | 3.5.0 latest; goose 1.48.0 uses 2.0.0 | crates.io; goose Cargo.lock | DRIFT -> pin =2.0.0 |
| ACP wire | protocolVersion = 1 | ProtocolVersion::V1 = 1; V2 alpha exists | crate 2.0.0 source | OK |
| MCP | stable 2025-11-25; RC 2026-07-28 | 2026-07-28 is FULL release | modelcontextprotocol releases | DRIFT |
| Wasmtime | upstream 45.x | v50.0.0 | wasmtime releases | DRIFT |
| NIPs | 17/44/59/46/57/61 (+32/56) | all present; NIP-04 deprecated in favor of NIP-17 | nostr-protocol/nips README | OK |
| rust-nostr | ALPHA | ALPHA confirmed; nostr 0.45.5 / nostr-sdk 0.45.4; repo moved rust-nostr/nostr -> nostrdevkit/nostr | crates.io; GitHub API | OK |
| Cashu | NUT-11 P2PK; CDK ALPHA | NUT-11 present; cdk 0.18.2 | cashubtc/nuts; crates.io | OK |
| libSQL | FTS5 + native vector, embedded | libsql 0.10.0; vector + FTS documented | crates.io; docs.turso.tech | OK |
| IronClaw pin | real commit | b011eb39 = ironclaw-v1.4.1 (2026-09-29); MIT OR Apache-2.0 | nearai/ironclaw | OK (re-pinned) |
| IronClaw layout | ~27 crates; flat crates/; wasm_limiter missing | ~74 lib crates; grouped dirs; ironclaw_wasm_limiter EXISTS; wasm_sandbox_core is a module of ironclaw_wasm; ironclaw_embeddings absent | git tree @ pin | DRIFT (prompt wrong) |
| Safety traits | InjectionScanner/LeakScanner | confirmed at crates/substrates/ironclaw_safety/src/lib.rs:67-80 | pinned source | OK |
| Alfred | MartinMontero/Alfred AGPL-3.0 | confirmed | GitHub API | OK |
| goose 2.0 behind ACP | claim | no primary source found | — | UNVERIFIED |

## Pin resolutions
See docs/pin-register/08_PIN_REGISTER-2026-10-27.md (5 CI pins + toolchain pins, all with
source URLs and as-of dates).

## Housekeeping
37 scratch verification files at repo root deleted 2026-10-27 with operator approval.
reference/ironclaw retained (study-only).
