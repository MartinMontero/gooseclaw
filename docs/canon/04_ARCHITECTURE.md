# 04 — Architecture

## 1. Workspace (Rust) — seven crates
AGPL-3.0-or-later. A 7th `safety` crate is split from `core` because the IronClaw read shows safety is substantial enough to isolate.

1. **`gooseclaw` (core).** Spawns and supervises a goose ACP server (`goose acp` / `goose serve`) via the `agent-client-protocol` crate (+ tokio process-spawn + MCP-over-ACP); forces goose **Approve** mode (overriding its **Auto** default); **owns the single Rule-9 gate** (Constitution 12) wrapping every consequential tool call; owns the scheduler and the draft-and-surface routines engine; owns process-tree lifecycle + kill-on-exit reaping; **owns durable session state** for crash recovery.
2. **`gooseclaw-channels`.** Channel adapters; **Nostr (NIP-17) first**. Renders/transports the ✅/❌ approval but **calls** the core gate — it does not own the decision. Enforces the inbound sender allowlist (Constitution 11). Message chunking from day one (Discord 2000, Telegram 4096, Nostr DM relay-dependent ~8 KB); careful SSE/nested-event parsing of goose's events.
3. **`gooseclaw-skills`.** Wasmtime component-model sandbox; **Trusted vs Installed** trust tiers; SKILL.md (YAML frontmatter + markdown); gating → scoring → attenuation, plus v2-style **capability leases**; supply-chain gate (Syft/OSV/Grype + Skill-Card); a defined **WIT world** for the skill ABI. Optional Skillsmith MCP discovery (self-hosted registry).
4. **`gooseclaw-nostr`.** NIP-44/17/59/46/57/61/32/56 via **rust-nostr** (`nostr` / `nostr-sdk`) — Rust, not Alfred's JS `nostr-tools`; parity is at the **protocol/NIP level**, not the library. Relay client; bunker (NIP-46) custody with fail-closed behavior.
5. **`gooseclaw-knowledge`.** Federation client to Alfred's MCP vault (system-of-record); thin local cache + working index; RRF hybrid search over libSQL/FTS5 + vector.
6. **`gooseclaw-payments`.** Lightning + Cashu (CDK; P2PK NUT-11) + NIP-57/61; per-spend approval + caps; **calls** the same core gate.
7. **`gooseclaw-safety`.** The IronClaw-derived safety pipeline (see §4): injection sanitizer + policy engine + leak detector + redaction + host-boundary credential injection.

## 2. goose ⇄ GooseClaw (the ACP contract)
Spawn `goose acp` as a supervised subprocess; drive via `agent-client-protocol` over stdio/JSON-RPC; negotiate `protocolVersion = 1`; force **Approve** mode. Pin goose to a verified release; Step-0 verify the ACP surface before any version bump. goose 2.0 is being built **behind** ACP, so the seam is the stable contract. **Note:** goose ACP providers do not support session `resume`/`fork` upstream — GooseClaw owns session state and replays context on restart.

## 3. Alfred ⇄ GooseClaw (the MCP-vault contract — Option C)
- Alfred exposes its path-confined, Zod-validated MCP tools as GooseClaw's **sovereign memory substrate**; Alfred is system-of-record, GooseClaw holds only ephemeral state + cache.
- **Reads** free within the path root; **writes go through Alfred's MCP tools** (Alfred is the single path-traversal enforcement point). GooseClaw never writes the vault on disk directly.
- **Single-writer** with optimistic concurrency (read-version / write-if-unchanged) so a human editing in Alfred always wins conflicts. *To specify:* cache staleness bounds and reconnect/conflict semantics when Alfred is offline.
- Federation target repo: `github.com/MartinMontero/Alfred` (AGPL-3.0).

## 4. IronClaw steals (must be re-derived against a pinned real commit before porting)
Read from `nearai/ironclaw` **main**, license **MIT OR Apache-2.0** (so selective port into AGPL with a NOTICE is legally clean). **Correction from QA:** the source spec's provenance is unreliable — see the warnings inline and `10_QA_HISTORY.md`.

- **`gooseclaw-safety` ← `crates/ironclaw_safety`.** Two trait seams: `InjectionScanner::scan_injection(content) -> Vec<InjectionWarning>` (sanitize untrusted text **before** it reaches the model; modules `sanitizer`, `prompt_validation`) and `LeakScanner::scan_leaks(content) -> LeakScanResult` (scan output **before** it crosses a persistence boundary; module `leak_detector`). Plus a **policy engine** (`Policy`/`PolicyRule`/`PolicyAction`/`Severity`), **redaction**/`display_redaction` (byte-capped), byte-capped **provider-payload validation**, and **credential detection**. *(Verified present on main.)*
- **Host-boundary credential injection ← `ironclaw_host_api` + `ironclaw_secrets` + `ironclaw_network`.** Secrets stored **AES-256-GCM** in the system keychain (`ironclaw_secrets`); injected **only at egress** (`ironclaw_network`: egress/url_target/policy/resolver) to allowlisted URL targets; the host API mediates every action with a decision + audit log (`ironclaw_host_api`: decision/dispatch/audit/http). Model and skills never see raw secrets. *(Verified present on main.)*
- **Capabilities ← `ironclaw_capabilities`.** Capability `requests` + `obligations` + host-side enforcement — the deny-by-default model skills request against. *(Verified present.)*
- **Skills trust model ← `ironclaw_skills`.** **Trusted** (user-placed, full tool access) vs **Installed** (registry/external, read-only tools); SKILL.md interoperable with Anthropic/goose skills; gating → selector (scoring) → attenuation (trust-based tool ceiling). IronClaw's v2 is migrating this to a Python/Monty orchestrator with **capability leases** — adopt the *lease* concept (time/scope-bounded grants) in Rust; **re-implement selection clean** (see `03_DECISION_LOG.md`). *(Verified.)*
- **WASM sandbox ← `ironclaw_wasm`.** Wasmtime **component model** + a resource limiter (memory/CPU/time). **CORRECTION:** `ironclaw_wasm` **exists**, but the source spec's `ironclaw_wasm_sandbox_core` and `ironclaw_wasm_limiter` **do not exist on any branch** — do not reference them. IronClaw pins Wasmtime **43.0.2** on main (43.0.1 was a stale staging value).
- **RRF hybrid search ← `ironclaw_memory` (+ `ironclaw_embeddings`).** Full-text + vector fused via Reciprocal Rank Fusion for `gooseclaw-knowledge`. IronClaw ships `crates/ironclaw_hooks_libsql` for exactly the embedded case — evidence for the libSQL decision.

**Provenance caveats (from QA):** main has **~27 workspace crates, not ~78** as the source spec claimed; the source spec's commit `4c82051` is **unverifiable** (latest release was 0.29.1 / 2026-06-04 / 556dfd0). Pin branch **and** commit explicitly, and confirm every crate/module path resolves during the Phase-0 sparse clone before porting anything.
