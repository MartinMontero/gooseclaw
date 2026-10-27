# Pin register refresh — 2026-10-27 (Phase 0 of the Phase-0/1 session)

Supersedes as-of dates in 08_PIN_REGISTER-2026-09-03.md where newer. Every entry verified
against the primary source on 2026-10-27. Rule: flag drift, never silently fix.

## CI pins (resolved this session — replaces the 9 PIN_RESOLVE_IN_PHASE_0 placeholders)

| Pin | Value | Source | As-of |
|---|---|---|---|
| actions/checkout | commit 3d3c42e5aac5ba805825da76410c181273ba90b1 (v7.0.1; refs/tags/v7 dereferences to the same commit) | api.github.com/repos/actions/checkout/git/ref/tags/v7 | 2026-10-27 |
| Syft | v1.55.0 (2026-10-19); checksums.txt + sigstore verified present | github.com/anchore/syft/releases | 2026-10-27 |
| Grype | v0.121.0 (2026-10-19); checksums.txt + sigstore verified present | github.com/anchore/grype/releases | 2026-10-27 |
| OSV-Scanner | v2.7.0 (2026-10-12); osv-scanner_SHA256SUMS verified present | github.com/google/osv-scanner/releases | 2026-10-27 |
| cargo-deny | 0.20.2 (2026-07-09); installed from source, --locked | github.com/EmbarkStudios/cargo-deny/releases | 2026-10-27 |

## Toolchain pins

| Pin | Value | Source | As-of |
|---|---|---|---|
| goose (installed) | 1.48.0 | goose --version (local) | 2026-10-27 |
| goose (upstream latest) | v1.56.0 (2026-10-23) — upgrade decision deferred to operator | github.com/block/goose/releases | 2026-10-27 |
| agent-client-protocol | =2.0.0 (matches goose 1.48.0 Cargo.lock); latest 3.5.0 (2026-10-22) | crates.io; block/goose Cargo.lock @ v1.48.0 | 2026-10-27 |
| ACP wire | protocolVersion = 1 (ProtocolVersion::V1; V2 = alpha) | crate 2.0.0 source src/role/acp.rs | 2026-10-27 |
| MCP spec | 2025-11-25 stable; 2026-07-28 now FULL release (was RC) | github.com/modelcontextprotocol/modelcontextprotocol/releases | 2026-10-27 |
| rust-nostr | nostr 0.45.5 / nostr-sdk 0.45.4, ALPHA; REPO MOVED: rust-nostr/nostr -> nostrdevkit/nostr | crates.io; api.github.com/repos/rust-nostr/nostr | 2026-10-27 |
| Cashu CDK | 0.18.2 | crates.io/api/v1/crates/cdk | 2026-10-27 |
| libsql | 0.10.0 (2026-10-13) | crates.io/api/v1/crates/libsql | 2026-10-27 |
| wasmtime | upstream v50.0.0 (2026-10-20); IronClaw main pins 43.0.2 | github.com/bytecodealliance/wasmtime/releases | 2026-10-27 |
| IronClaw (study pin) | b011eb3932a0e1ab8bbe833893599ca9ae4925c5 = tag ironclaw-v1.4.1 (2026-09-29); MIT OR Apache-2.0 | github.com/nearai/ironclaw | 2026-10-27 |

## Drift flags raised this session (all recorded; none silently fixed)

1. Kickoff prompt said goose v1.41.0/v1.39.0 — installed 1.48.0; upstream v1.56.0.
2. Kickoff prompt pinned agent-client-protocol 1.0.0 — SUPERSEDED by =2.0.0 (goose lock match).
3. Kickoff prompt called MCP 2026-07-28 an RC — it is now the latest FULL release.
4. Kickoff prompt said wasmtime upstream 45.x — actual v50.0.0.
5. Kickoff prompt said IronClaw has ~27 crates — actual ~74 library crates at pin.
6. Kickoff prompt said ironclaw_wasm_limiter does NOT exist — it EXISTS (crates/lanes/ironclaw_wasm_limiter). ironclaw_wasm_sandbox_core confirmed absent as a crate (it is a MODULE inside ironclaw_wasm). ironclaw_embeddings absent.
7. IronClaw steal-paths are GROUPED (substrates/, domains/, kernel/, contracts/, lanes/), not flat crates/.
8. 'goose 2.0 built behind ACP' — UNVERIFIED (no primary source; canon carries it as [04 A2][08R]; treat as aspiration).
