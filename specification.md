# GooseClaw — specification.md

**Status:** DRAFT for ratification by Martin (project owner). Assembled 2026-09-03 from the numbered canon (`00`–`10`, `19`, `20`) plus the refreshed pin register `[08R]`. **Amended 2026-09-08:** §3.2 dual-era MCP-server constraint applied per the Choice-B probe (`REPORT.md` §5, probe executed 2026-09-07); §9 O2 marked ANSWERED with residuals; **O2 CLOSED 2026-09-09** by Martin's ruling (recorded in `03_DECISION_LOG.md`).
**Provenance convention:** every design claim is tagged **CANON** with an inline source tag — `[01]`…`[20]` = the numbered canon files, `[08R]` = `08_PIN_REGISTER-refreshed-2026-09-03.md`, `[BC]` = `BUILDEROS-CONSTITUTION-GooseClaw.md`, `[VG]` = `vendor_gate.py`, `[CI]` = `ci.yml`, `[CL]` = `CLAUDE.md`, `[D0901]` = Martin's decisions of 2026-09-01 (recorded here; `09_OPEN_DECISIONS.md` not yet updated — see §10) — or labeled **PROPOSED** (not yet canon; requires ratification). Nothing else is asserted.
**Rules in force:** zero fabrication; conflicts flagged in §10, never silently resolved; drift in pins flagged, not silently fixed `[00] [08R]`.

---

## 0. Purpose & fiduciary scope

**Verdict:** GooseClaw is a Nostr-native, always-on **fiduciary agent runtime** — a fiduciary steward, **not an autonomous agent**. CANON `[01] [19]`.

| Element | Statement | Source |
|---|---|---|
| Loop | **Observe → Draft → Surface → Execute**; the runtime **halts at the moment of execution** to await explicit human consent | CANON `[01]` |
| Authority | No interests of its own; extends user capability, never displaces user authority. Proactivity may only *draft and surface* | CANON `[01] [02 §2]` |
| What it drives | Block/AAIF **goose** over **ACP** (goose is an external dependency, never forked); federates with **Alfred** over Alfred's path-confined MCP vault; reuses **IronClaw** security/sandbox *designs* without forking | CANON `[01] [BC]` |
| State posture | Holds only ephemeral state + cache; **Alfred stays the system-of-record**. GooseClaw never runs inside Alfred and never turns Alfred into an agent | CANON `[01] [02 §1] [04 §3]` |
| Who it serves first | Marginalized and targeted communities; safety is the unbypassable default, not a setting ("the people most exposed are the ones this serves") | CANON `[01] [20 P1]` |
| Identity framing | Augmented Intelligence — "Human in the Lead" | CANON `[01]` |
| Non-goals | Not autonomous; not a goose fork; not embedded in Alfred; not a hosted SaaS; not engagement-maximizing; not a general chatbot | CANON `[19]` |
| License / distribution | AGPL-3.0-or-later; network-served instances offer source (AGPL §13); distribution Nostr-native via Zapstore | CANON `[02 §9] [03]` |

---

## 1. Governing constitution (13 articles — summary; `02_CONSTITUTION.md` is the binding text)

CANON `[02]` (articles 1–10 ratified; 11–13 binding security amendments added post-QA `[10]`).

| # | Article (compressed) | # | Article (compressed) |
|---|---|---|---|
| 1 | Memory is not hands — Alfred is sovereign memory; typed MCP boundary only | 8 | Nostr-native identity, metadata-protective by default (NIP-17 DMs, NIP-46 custody) |
| 2 | Consent before consequence (Rule 9) — no write/send/spend/commit/push/destructive act without explicit human go-ahead | 9 | AGPL-3.0-or-later; commons; Zapstore distribution |
| 3 | Sovereign by construction — local-first, zero telemetry, no public bind by default, no mandatory hosted dependency | 10 | Evidence over assertion — zero fabrication; primary sources; stop and ask |
| 4 | goose is the engine; ACP is the seam — engine-agnostic at the boundary | 11 | **Inbound is untrusted; talk is gated** — sender allowlist default; unknown senders get no tools, no vault reads |
| 5 | Secrets never reach skills — encrypted at rest, injected only at egress to allowlisted endpoints, scanned outbound | 12 | **One gate, owned by the core** — single, non-bypassable Rule-9 primitive; channels/payments *call* it |
| 6 | Untrusted code sandboxed by default — Wasmtime component model, deny-by-default, trust tiers | 13 | **Approvals are authenticated** — allowlisted-pubkey-signed, nonce + expiry bound to action hash, single-use; NIP-46 bunker-offline ⇒ fail closed |
| 7 | Vendor line holds — Meta/OpenAI/xAI excluded at config/CI level; enforced, not advisory | | |

---

## 2. Architecture — seven crates and dependency direction

CANON `[04 §1] [06] [09 §3] [CL]`. Seven-crate Rust workspace, AGPL-3.0-or-later; `gooseclaw-safety` split from core because the IronClaw read showed safety is substantial enough to isolate `[04 §1]`.

| Crate | Owns | Source |
|---|---|---|
| `gooseclaw` (core) | Spawns/supervises the goose ACP subprocess; forces **Approve** mode; **owns the single Rule-9 gate**; scheduler + draft-and-surface routines engine; process-tree lifecycle + kill-on-exit reaping; **durable session state**; global resource caps | `[04 §1] [06]` |
| `gooseclaw-channels` | Channel adapters (Nostr/NIP-17 first); renders/transports the ✅/❌ approval but **calls** the core gate; enforces the inbound sender allowlist; message chunking from day one | `[04 §1] [02 §11–12]` |
| `gooseclaw-skills` | Wasmtime component-model sandbox; Trusted/Installed tiers; SKILL.md; gating → scoring → attenuation + capability leases; supply-chain gate; defined WIT world (Phase 5) | `[04 §1] [03] [10 A10]` |
| `gooseclaw-nostr` | NIP-44/17/59/46/57/61/32/56 via rust-nostr (repo moved → `nostrdevkit/nostr` `[08R]`); relay client; NIP-46 bunker custody, fail-closed | `[04 §1] [08R]` |
| `gooseclaw-knowledge` | Federation client to Alfred's MCP vault; thin local cache + working index; RRF hybrid search over libSQL/FTS5 + native vector | `[04 §1] [03]` |
| `gooseclaw-payments` | Lightning + Cashu (CDK; P2PK NUT-11) + NIP-57/61; per-spend approval + per-period caps; **calls** the core gate | `[04 §1] [03]` |
| `gooseclaw-safety` | IronClaw-derived safety pipeline: injection sanitizer + policy engine + leak detector + redaction + host-boundary credential injection | `[04 §1, §4]` |

**Dependency direction (the Rule-9 star):** CANON: `-channels` and `-payments` **call into** the core gate; the decision is never owned or reimplemented outside core — exactly one enforcement point `[02 §12] [BC]`. `-safety` internals are consumed by the pipeline that wraps model I/O and egress `[04 §4]`. `-knowledge` is the only crate that speaks to Alfred, across the typed MCP boundary `[01] [04 §3]`.
PROPOSED (ratification requested): the call graph is acyclic with core at the center — satellites depend on the core gate API; core never imports channel/payment transport implementations. The canon fixes the gate direction explicitly but does not spell out the remaining edges.

---

## 3. Contracts

### 3.1 goose ⇄ GooseClaw — the ACP contract

CANON `[04 §2] [03]`, drift-corrected `[08R]`:

| Term | Value | Source |
|---|---|---|
| Spawn | `goose acp` (or `goose serve` for ACP over HTTP/WS) as a supervised subprocess; stdio/JSON-RPC | `[04 §2] [08R]` |
| Client crate | `agent-client-protocol` — **crate 2.0.0** (2026-07-23); **wire `protocolVersion = 1` remains the stable protocol** (V2 only behind an unstable feature flag) | `[08R]` |
| Permission mode | Force **Approve** (overriding goose's **Auto** default; modes: Auto/Approve/Chat/Smart Approve) | `[04 §2] [08R]` |
| Approve-mode caveat | **No documented upstream hard lock** against a mid-session `/mode` change — GooseClaw enforces Approve at its own gate and does not trust the goose setting | `[08R]` (UNVERIFIED upstream; caveat recorded) |
| Session state | **GooseClaw owns durable session state and replays context on restart** — goose ACP has no `resume`/`fork` upstream | `[04 §2] [03] [10 A9]` |
| Seam stability | goose 2.0 is being built behind ACP; build against the ACP wire version, not goose internals | `[04 §2] [08R]` |
| Pin | **Pin goose to a verified current release at Phase 0 — v1.48.0 (2026-08-27) as of 2026-09-03**; repo `github.com/aaif-goose/goose`, Apache-2.0; re-pin pending Martin's ratification | `[08R]` |

### 3.2 Alfred ⇄ GooseClaw — the MCP-vault contract (Option C)

CANON `[04 §3] [03]`:

- Alfred (target repo `github.com/MartinMontero/Alfred`, AGPL-3.0) exposes its path-confined, Zod-validated MCP tools as GooseClaw's **sovereign memory substrate**; Alfred is system-of-record, GooseClaw holds only ephemeral state + cache.
- **Reads** free within the path root. **Writes go only through Alfred's MCP tools** — Alfred is the single path-traversal enforcement point; GooseClaw never writes the vault on disk directly.
- **Single-writer with optimistic concurrency** (read-version / write-if-unchanged): **a human editing in Alfred always wins** conflicts.
- *To specify (Phase 2):* cache staleness bounds; reconnect/conflict semantics when Alfred is offline `[04 §3]`.
- **Re-validation EXECUTED 2026-09-07** (Choice-B probe; evidence: `REPORT.md` §4 wire transcripts, probe dir `docs/audit/2026-09-07-mcp-acp-revalidation/` — probe-EXECUTED, carried here as REPORTED evidence pending independent replay; amendment applied to this draft **2026-09-08**): the [08R] description of spec **2026-07-28** is **confirmed accurate** against the shipped changelog. Probe verdict against goose **1.48.0**:
  - **BREAKS (modern-only server):** goose 1.48.0's MCP client is **legacy-era** — it opens with `initialize` / `protocolVersion: "2025-11-25"` (capabilities `roots`/`sampling`/`elicitation`). A strict 2026-07-28 server rejects `initialize` (`-32601`, unknown method) and the extension fails to load. Spec locus: `basic/versioning.mdx` compatibility matrix, *Legacy client → Modern server = Fails*; "legacy clients have no fall-forward mechanism."
  - **WORKS (dual-era server):** the full §3.2 call shape executed live over ACP v1 in forced Approve mode — `tools/list`, `vault_read`, `vault_write` with `expectedVersion` (v1→v2), and a stale write rejected with CONFLICT (human-edit-wins). Every tool call was preceded by an ACP `session/request_permission` round-trip.
  - **PROPOSED constraint (pending ratification — the probe text says RULED; it becomes RULED on Martin's word):** Alfred's MCP server **MUST be dual-era** — answer `initialize` and serve **2025-11-25** for goose, AND implement `server/discover` + modern per-request-`_meta` mode for 2026-07-28 clients — until goose ships a 2026-07-28-capable MCP client (re-check each goose bump per [08R] pin rules). Note: `@modelcontextprotocol/sdk` latest (1.30.0, 2026-07-27) tops out at 2025-11-25, so dual-era is currently the only SDK-buildable conformant target regardless.
  - Side note: stage-goose-sidecar pin **1.43.0 ≠ probed 1.48.0** — flagged per pin rules; sidecar pin needs re-verification before Phase 2.

### 3.3 Channel contract

CANON `[04 §1] [02 §11, §13] [03] [07]`:

- First channel: **Nostr NIP-17 private DMs** (seal kind 13 + gift-wrap kind 1059, chat kind 14; NIP-44 encryption; NIP-59 wrap — NIP-59 also documents ephemeral gift-wrap kind 21059 `[08R]`). NIP-04 = deprecated, read-only legacy.
- Channels **render/transport** the ✅/❌ approval; the **decision belongs to the core gate** (§4.1).
- Inbound sender allowlist enforced in the channel layer; unknown senders quarantined — no tool exposure, no vault reads.
- Message chunking from day one (Discord 2000, Telegram 4096, Nostr DM relay-dependent ~8 KB); careful SSE/nested-event parsing of goose's events `[04 §1]`.
- Channel roadmap: **Nostr → Signal → White Noise** `[D0901]` (§7.2). PROPOSED: draft FRs for Signal and White Noise adapters (current FR list `[19]` names only Nostr/CLI/local-web-UI).

### 3.4 Payments contract

CANON `[03] [04 §1] [07]`:

- **Lightning + Cashu** — Cashu **P2PK proofs (NUT-11) via the Rust CDK** (CDK self-labels ALPHA "use with caution"; latest 0.18.0, 2026-09-02, breaking mint-config migration `[08R]`); NIP-57 zaps / NIP-61 nutzaps.
- **Explicit approval per spend + per-period caps, by default**; every spend **calls the core Rule-9 gate** (§4.1).
- Local Cashu proofs are bearer value: encrypted at rest; caps limit blast radius; P2PK where possible `[07]`.
- **NIP-61 implementation path is OPEN** — NIP-61 (nutzaps) is **not implemented** in `nostrdevkit/nostr` `[08R]`. See §9.

---

## 4. Security model

Scope assumption, CANON `[07]`: an unattended, always-on, Nostr-connected agent with private-vault reads, an LLM in the loop, and outbound reach — the OpenClaw risk profile. The design breaks that class **structurally**, not by detection: prompt-injection classifiers are assumed bypassable; rely on architecture (allowlisting, consent gate, egress control).

### 4.1 The Rule-9 gate (singular, core-owned, PreToolUse)

CANON `[02 §12] [03] [BC] [CL]`:

- **Exactly one enforcement point**, owned by `gooseclaw` core; **channels and payments CALL it** — they never own or reimplement the decision.
- Enforced as a **PreToolUse deny hook** wrapping every consequential tool call — mechanism, not prompt discipline `[BC] [CL]`.
- Covers every write, send, spend, commit, push, or destructive action `[02 §2]`.
- The gate shows the human **which tool, what data, what external destination**, so a lethal-trifecta attempt is visible at the point of consent; approval prompts are rate-limited (anti-fatigue/✅-spam) `[07]`.

### 4.2 Inbound trust & consent (Nostr identity/consent)

CANON `[02 §11, §13] [03] [07]`:

- **Inbound sender allowlist by default.** Unknown senders quarantined: no tool exposure, no vault reads, until a human approves them. All inbound text is **data, never instructions**.
- **Authenticated approvals:** a ✅/❌ is honored only if **signed by an allowlisted approver pubkey**, carries a **nonce + expiry bound to the specific action's hash**, and is **single-use**. Relay-injected, replayed, or reordered approvals are rejected.
- **Key custody:** NIP-46 remote signing (bunker, on separate hardware); no hot `nsec` in the always-on process; host holds only a disposable client key `[03] [06]`.
- **Fail-closed:** bunker offline ⇒ the agent goes **mute** rather than acting unsigned `[02 §13] [03] [07] [10 A8]`.
- **Crypto caveat (transitional):** NIP-46 kind 24133 content is **now NIP-44-encrypted** (`nip44_*` methods; `nip04_*` retained as legacy); tracker issue nostr-protocol/nips#1095 remains OPEN. The old "NIP-46 depends on NIP-04" caveat in `[03]/[07]` is reworded as **transitional, not structural** `[08R]`.

### 4.3 Sanitization

CANON `[07] [CL] [10]` (Pale Fire lesson): strip Unicode Tag Block (U+E0000–U+E007F), zero-width (U+200B/C), and bidi confusables from **all inbound content** *and* from the plain-English approval summaries shown at the gate; validate before wrapping into model-visible markup.

### 4.4 Sandbox & skills

CANON `[02 §6] [03] [04 §1, §4] [07]`: Wasmtime component-model sandbox, deny-by-default capabilities; Trusted (user-placed, full tools) vs Installed (registry/external, read-only) tiers; capability leases (time/scope-bounded grants); gating → scoring → attenuation; WIT world defined in Phase 5; registry installs default to Installed tier with human install-time consent.

### 4.5 Secrets

CANON `[02 §5] [04 §4] [06]`: credentials stored AES-256-GCM in the system keychain (IronClaw pattern); injected **only at the host/network egress boundary** to allowlisted URL targets; scanned outbound; model and skill code never see raw secrets.

### 4.6 Lethal-trifecta mitigations (priority order)

CANON `[07]`: (1) break the exfil leg — no auto-outbound with private data in context; every consequential egress passes the human gate; (2) gate ingestion at the door — sender allowlist, untrusted-data rule; (3) assume detection fails. Plus: relay rate-limiting, supervisor restart damping, global token/CPU/memory caps (DoS); path confinement on every tool `[CL]`; metadata residual risk (persistent relay IP fingerprint) documented for at-risk users, Tor/mixnet considered `[07]`.

---

## 5. Observability & audit

CANON `[03] [06] [07] [10 A6]`:

| Element | Statement |
|---|---|
| Audit log | **Local-only, append-only, structured** audit log (decision/dispatch/audit record model stolen from IronClaw `ironclaw_host_api`); **zero network egress** |
| Born-redacted | Logging is **born-redacted** (redaction at write time, byte-capped), reusing the Alfred-side plumbing pattern via the Tauri-sidecar deployment `[06]`; leak-scan before any persistence boundary `[04 §4]` |
| Zero telemetry defined | "Zero telemetry" = **no phone-home**, *not* no logs — the local audit log is required `[06] [07]` |
| Correlation | W3C trace-context across the ACP boundary (Alfred already does SEP-414-style correlation) |
| Metrics | Token-spend and sat-spend against caps; all local |
| Repudiation control | The append-only log answers "the agent did X and there's no record" `[07]` |
| Product metrics | Proxy or opt-in only, never silently collected (Zapstore installs, self-reported deployments, opt-in NIP-89 announcements) `[19]` |

---

## 6. Supply chain & vendor denylist

### 6.1 Policy

CANON `[01] [02 §7] [03] [BC] [VG]`: **Meta, OpenAI, xAI are hard-excluded — direct AND transitive** — as dependencies, SDKs, and model providers. **Google permitted**; also Mistral, Ollama, MiniMax, Anthropic. Enforced at config/CI level — enforced, not advisory.

### 6.2 Enforcement

CANON `[VG] [CI] [06] [12]`:

- **`vendor_gate.py`** = layer 1 (build-time dependency graph, via `cargo metadata`; crate-name, org-URL, and author-domain matchers; stdlib-only by design). Layers 2 (runtime provider config) and 3 (setup-wizard model filtering) are application code.
- Wired as a **required CI check** — never optional/allow-failure — with a deliberately-failing self-test fixture proving the matcher fires before the real scan can pass `[CI]`.
- `EXCEPTIONS` map is **deliberately empty**: denied-org infrastructure crates (e.g. `zstd`, `rocksdb` from `github.com/facebook/`) may appear transitively; if the gate fires, **stop and ask** — an exception is a Rule-9-class human decision with rationale + date. Never add exceptions to make CI green `[12] [VG]`.
- Supply-chain stack: **Syft SBOM + OSV-Scanner + Grype + cargo-deny**; lockfile audit. **Trivy is BANNED** (CVE-2026-33634, TeamPCP tag-poisoning, CVSS 9.4, CISA KEV 2026-03-26) `[06] [08R]`. Every GitHub Action pinned to a **full commit SHA**; the committed `ci.yml` carries 9 deliberately-invalid `PIN_RESOLVE_IN_PHASE_0` placeholders so CI fails loudly rather than run on a fabricated hash — preserve this property until Phase 0 resolves them against primary sources `[CI] [BC] [12]`.

### 6.3 Known divergence (system weakness §7.1)

CANON `[BC]`: the `vendor_gate.py` seed list is **maintained independently of Holmes's `policy.rs` and WCJBT's vendor-exclusion engine, with no conformance test** — the three denylist implementations can silently drift apart.
**PROPOSED (conformance-test strategy, not yet canon):** add a CI job that extracts the three seed lists (vendor_gate.py / Holmes `policy.rs` / WCJBT engine), diffs them, and fails on divergence; plus a shared golden-fixture corpus (must-catch / must-pass packages, extending the existing self-test fixture) run against all three matchers.

---

## 7. Decisions record

### 7.1 Ratified spine (canonical, from `03_DECISION_LOG.md`)

CANON `[03]` — compressed; the decision log is the system-of-record.

| Axis | Decision |
|---|---|
| Foundation | goose over ACP (wire v1); NOT an IronClaw fork; NOT embedded in Alfred; pin verified current goose release at Phase 0 (now v1.48.0 `[08R]`) |
| Topology | Option C federated: Alfred = memory substrate; GooseClaw = separate always-on process tree |
| Autonomy | Rule-9 human-in-the-loop by default; routines = draft-and-surface only |
| Approval gate | Single, core-owned, non-bypassable; authenticated approvals (Constitution 12/13) |
| Inbound trust | Sender allowlist by default; inbound text = untrusted data (Constitution 11) |
| DMs | NIP-17 (13/1059/14) + NIP-44 + NIP-59; NIP-04 legacy read-only |
| Key custody | NIP-46 bunker; fail-closed offline; crypto caveat now transitional `[08R]` |
| Payments | Lightning + Cashu (P2PK/NUT-11/CDK) + NIP-57/61; per-spend approval + per-period caps |
| Skills | Wasmtime component model + Trusted/Installed tiers + supply-chain gate; WIT world in Phase 5; Skillsmith posture per §7.2 `[D0901]` |
| Persistence | **libSQL/FTS5 + native vector**, RRF on top; Postgres/pgvector rejected (sovereignty + supply-chain liability). Standing caveat: libSQL is in **maintenance mode** — new features move to Turso; evaluate before Phase 0 locks the pin `[08R]` |
| Steal mechanism | **Port the security mechanism (attributed + scanned); author the governance clean** |
| Resources | Global caps: token/cost ceilings, Wasmtime fuel/memory, per-routine quotas |
| Observability | Local-only audit log; zero telemetry = no egress, not no logs |
| Session recovery | GooseClaw owns durable session state; no reliance on goose ACP resume |
| License/dist | AGPL-3.0-or-later; Zapstore (desktop/CLI via zapstore-cli) |
| Provider policy | Denylist only Meta/OpenAI/xAI; Google/Mistral/Ollama/MiniMax/Anthropic permitted |
| IronClaw provenance | Re-derive steals against a pinned real commit before any port |
| Method | GitHub Spec Kit + goose RPI; Rule 9 before any commit/push/destructive action |
| Ethos | Liberation tech; sovereignty; local-first; zero telemetry; no vendor lock-in |

### 7.2 Decided by Martin, 2026-09-01 (recorded; do not re-ask)

`[D0901]`, White Noise pin row CANON `[08R]`:

**(a) Skillsmith is IN for v1, ships day 1 — external-only ELv2 posture.**
- End-users spawn Skillsmith **locally via `npx`** as an **external MCP server** — "connected to, never bundled." Nothing of Skillsmith is vendored into the AGPL tree (Elastic-2.0 license boundary preserved).
- **Tripwire:** no BuilderOS-hosted Skillsmith registry/service for third parties without smith-horn's written permission.
- NOTICE attribution carried.
- Supersedes: `09_OPEN_DECISIONS.md` #5 (was open; recommendation had been "optional, deferred past v1") — see §10 C3.

**(b) Channel priority: Nostr → Signal → White Noise.**
- White Noise = whitenoise.chat, built on the **Marmot Protocol (MLS over Nostr)**; integrate via **`marmot-protocol/mdk` crates — NOT the obsolete `whitenoise-rs`** (its own README declares it obsolete) `[08R]`.
- Supersedes: `09_OPEN_DECISIONS.md` #6 (was open) — see §10 C3.

### 7.3 QA amendments traceability (the ten, `10_QA_HISTORY.md §Amendments` — all reflected)

CANON `[10]`:

| # | Severity | Amendment | Reflected in |
|---|---|---|---|
| 1 | Critical | Inbound-DM sender allowlist; inbound text = untrusted data | §1 (art. 11), §3.3, §4.2 |
| 2 | Critical | Single core-owned Rule-9 gate; channels/payments call it | §1 (art. 12), §2, §4.1 |
| 3 | Critical | Authenticated approvals (pubkey-signed, nonce+expiry bound to action hash, single-use) | §1 (art. 13), §4.2 |
| 4 | High | Re-derive IronClaw steals vs pinned real commit; drop phantom wasm crates; fix crate count/Wasmtime | §7.1, §9, Appendix B; current drift facts in `[08R]` |
| 5 | High | Re-pin goose to verified current release; repo aaif-goose/goose | §3.1 (`v1.48.0 as of 2026-09-03` `[08R]`) |
| 6 | High | Local audit-log / observability design reconciled with zero telemetry | §5 |
| 7 | High | Global resource/cost caps (token/CPU/memory/wallet) | §4.6, §7.1 |
| 8 | High | NIP-46 bunker-offline fail-closed + crypto caveat (now transitional `[08R]`) | §4.2 |
| 9 | High | Session crash/resume design independent of goose ACP resume | §3.1 |
| 10 | Housekeeping | Adopt libSQL + port-mechanism/author-governance decisions; define skill WIT world in Phase 5 | §2, §7.1, §8 |

---

## 8. Phases, Phase-0 exit criteria, first milestone

CANON `[05] [12] [19]`. Method: GitHub Spec Kit (constitution → specification → plan → tasks) + goose RPI. Every phase gate = **all tests pass + explicit human sign-off**.

| Phase | Scope | Acceptance gate |
|---|---|---|
| 0 | Step-0 primary-source pins (goose/ACP/MCP/NIPs/Wasmtime/Cashu/libSQL); sparse-clone IronClaw @ pinned commit, confirm every crate/module path; produce threat-model doc + audit-log design | Verification table delivered, every drift flagged + sourced; threat model + audit-log design drafted |
| 1 | **ACP runtime spike** — drive goose ACP subprocess with the core Rule-9 gate in the loop, Approve mode | e2e consent loop passes; localhost-only bind; supervisor restart + session-replay test; resource caps enforced |
| 2 | Alfred MCP-vault federation (read-free / write-through-MCP; single-writer optimistic concurrency) — **after MCP 2026-07-28 re-validation `[08R]`** | Conflict test (human edit wins); Alfred-offline degradation test |
| 3 | Nostr channel + approval gate: NIP-17 DMs, core-called ✅/❌ gate, chunking, allowlist, authenticated approvals, NIP-56/32 moderation | Red-team/prompt-injection suite passes (Unicode smuggling, crafted-DM injection, trifecta exfil, approval spoof/replay) |
| 4 | Key custody + wallet gating: NIP-46 bunker (fail-closed), Cashu P2PK, per-spend approval + caps | Bunker-offline mute test; spend-cap-under-load test |
| 5 | Skills sandbox + safety port + supply chain; WIT world; Skillsmith external-MCP integration `[D0901]` | Sandbox escape fails; supply-chain gate blocks known-bad fixture |
| 6 | Heartbeat as draft-and-surface + eval harness | Runbook validated on clean machine; eval harness green |

**Phase 0 exit criteria** CANON `[05 §Phase 0] [12]`: (1) every volatile surface pinned and verified against primary sources with as-of dates, drifts flagged not fixed (including the 9 `PIN_RESOLVE_IN_PHASE_0` CI placeholders, resolved with checksum verification); (2) IronClaw sparse-clone path-resolution confirmed against a pinned real commit; (3) threat-model doc and observability/audit-log design drafted; (4) repo scaffolding ready (seven crate stubs, `cargo check` clean) with pre-authored `CLAUDE.md`/`ci.yml`/`vendor_gate.py` preserved, not regenerated.

**First milestone** CANON `[05 Phase 1] [12]`: **the ACP supervisor + the core-owned Rule-9 gate ONLY** — spawn `goose acp`, complete the handshake in Approve mode, demonstrate one core-gated authenticated tool-call round-trip (`prompt → tool call → ActionRequired → human ✅ → resume`). **No channels, no payments, no Nostr publishing** in this milestone. MVP = Phases 0–3 `[19]`.

---

## 9. Open questions

| # | Question | Status / source |
|---|---|---|
| O1 | **NIP-61 nutzaps implementation path** — not implemented in `nostrdevkit/nostr` `[08R]`. PROPOSED: evaluate alternatives (patch/upstream contribution, alternate library, or defer nutzaps) **before Phase 4** | OPEN `[08R]` |
| O2 | **MCP-vault contract re-validation** against the shipped stateless MCP spec 2026-07-28 (and MCP-over-ACP assumptions) **before Phase 2** | **CLOSED 2026-09-09** by Martin's ruling (recorded in `03_DECISION_LOG.md`, Post-spine closures); answered 2026-09-07 (Choice-B probe, `REPORT.md` — REPORTED): dual-era constraint ratified into §3.2. **Residuals (standing duties, not open items):** re-check each goose bump; sidecar pin 1.43.0 ≠ probed 1.48.0; HTTP/WS transports + mid-session `/mode` adversarial test unverified |
| O3 | **libSQL maintenance-mode risk** — new features move to Turso (Rust rewrite, beta); evaluate before Phase 0 locks the persistence pin | OPEN (standing caveat) `[08R]` |
| O4 | **IronClaw steals re-derivation** against a pinned **1.x** commit post-"Reborn" (1.4.0, 2026-08-27, commit 4cb47cf; ~68 crates; `ironclaw_wasm_limiter` now exists; `ironclaw_wasm_sandbox_core` still absent; Wasmtime pinned 47.0.4) | OPEN — required before any port `[08R] [10 A4]` |
| O5 | MCP-vault cache staleness bounds + reconnect/conflict semantics when Alfred is offline | To specify, Phase 2 `[04 §3]` |
| O6 | Skill **WIT world / ABI** definition | Phase 5 `[03] [10 A10]` |
| O7 | Vendor-gate conformance test across vendor_gate.py / Holmes policy.rs / WCJBT engine | PROPOSED strategy in §6.3 — needs ratification `[BC]` |
| O8 | FRs for Signal and White Noise adapters (post-`[D0901]` channel priority) | PROPOSED — §3.3 |
| O9 | Approve-mode hard lock: no documented upstream mechanism to prevent mid-session `/mode` change; GooseClaw enforces at its own gate. Whether upstream adds a hard lock | UNVERIFIED upstream `[08R]` |
| O10 | Whether `ironclaw_wasm_sandbox_core` exists on any non-main IronClaw branch | UNVERIFIABLE from current tooling `[08R]` |

---

## 10. Conflicts (flagged, unresolved)

Per constitution: conflicts are listed, not resolved. None block ratification of this draft; each names its resolution owner.

| # | Conflict | Between | State |
|---|---|---|---|
| C1 | **Pin-register drift.** Old register `[08]` pins goose v1.41.0, `agent-client-protocol` 1.0.0, MCP 2025-11-25 stable, Wasmtime 45.0.0/IronClaw-43.0.2, IronClaw ~27 crates — all superseded by primary-source re-verification (goose v1.48.0; crate 2.0.0/wire v1; MCP 2026-07-28 FINAL; Wasmtime 48.0.1/IronClaw-47.0.4; IronClaw ~68 crates) | `[08]` (2026-07-06) vs `[08R]` (2026-09-03) | FLAGGED — this spec follows `[08R]`; re-pin of the register awaits Martin's ratification |
| C2 | **NIP-46 crypto caveat.** `[03]`/`[07]` say "NIP-46 still depends on NIP-04 (issue #1095)"; kind 24133 is now NIP-44-encrypted, issue #1095 still open | `[03] [07]` vs `[08R]` | FLAGGED — spec adopts the transitional rewording (§4.2); `[03]/[07]` text stale |
| C3 | **Open-decisions staleness.** `09_OPEN_DECISIONS.md` still lists Skillsmith scope (#5) and channel priority (#6) as open; both were DECIDED by Martin 2026-09-01 (§7.2) | `[09]` vs `[D0901]` | FLAGGED — substance resolved by the decisions; `09` file not yet updated (ratification PR's job) |
| C4 | **IronClaw wasm-crate existence.** `[04 §4]`/`[10 A4]` (correctly, as of 2026-07-06) say `ironclaw_wasm_limiter` does not exist on any branch; it **now exists** on main post-Reborn. `ironclaw_wasm_sandbox_core` still 404 on main | `[04 §4] [10]` vs `[08R]` | FLAGGED — the standing rule (re-derive steals vs a pinned 1.x commit before any port) covers both directions of drift |
| C5 | **Denylist vs transitive infra crates.** Policy blocks denied-org URLs outright; `zstd`/`rocksdb` (`github.com/facebook/`) may appear transitively with no AI surface. `EXCEPTIONS` is deliberately empty | `[02 §7] [VG]` policy vs real dependency trees `[12]` | FLAGGED by design — resolution is a Rule-9-class human decision per occurrence; do not auto-exempt |
| C6 | **Stale rust-nostr URL in gate fixture.** `vendor_gate.py`'s self-test fixture cites `github.com/rust-nostr/nostr` as a real-dep URL; the repo moved to `nostrdevkit/nostr` | `[VG]` fixture vs `[08R]` | FLAGGED — cosmetic (matcher unaffected); fix at Phase-0 pin resolution |
| C7 | **Kickoff expects ACP crate 1.0.0 / goose v1.41.0.** `[12]` Step-0 expectations predate the refresh | `[12]` vs `[08R]` | FLAGGED — treat `[12]` expectations as superseded by `[08R]` at execution time |

---

## Appendix A — Provenance map

| Spec section | Source file(s) |
|---|---|
| 0 Purpose & fiduciary scope | `01_CHARTER.md`; `19_PRD.md`; `20_PERSONAS_AND_USE_CASES.md`; `02_CONSTITUTION.md` §1–2; `04_ARCHITECTURE.md` §3; `BUILDEROS-CONSTITUTION-GooseClaw.md`; `03_DECISION_LOG.md` |
| 1 Constitution summary | `02_CONSTITUTION.md` (verbatim-compressed); `10_QA_HISTORY.md` |
| 2 Architecture / crates | `04_ARCHITECTURE.md` §1; `06_SDLC_DEVOPS_DEPLOYMENT.md`; `09_OPEN_DECISIONS.md` §3; `CLAUDE.md`; `08_PIN_REGISTER-refreshed-2026-09-03.md` [08R] |
| 3.1 ACP contract | `04_ARCHITECTURE.md` §2; `03_DECISION_LOG.md`; [08R]; `10_QA_HISTORY.md` A9 |
| 3.2 MCP-vault contract | `04_ARCHITECTURE.md` §3; `03_DECISION_LOG.md`; `01_CHARTER.md`; [08R] (MCP 2026-07-28) |
| 3.3 Channel contract | `04_ARCHITECTURE.md` §1; `02_CONSTITUTION.md` §11/13; `03_DECISION_LOG.md`; `07_THREAT_MODEL.md`; [08R] (kind 21059); [D0901] |
| 3.4 Payments contract | `03_DECISION_LOG.md`; `04_ARCHITECTURE.md` §1; `07_THREAT_MODEL.md`; [08R] (CDK 0.18.0; NIP-61 gap) |
| 4 Security model | `07_THREAT_MODEL.md`; `02_CONSTITUTION.md` §5–6, §11–13; `03_DECISION_LOG.md`; `BUILDEROS-CONSTITUTION-GooseClaw.md`; `CLAUDE.md`; `06_SDLC_DEVOPS_DEPLOYMENT.md`; `10_QA_HISTORY.md` A1–A3, A8; [08R] (NIP-46 rewording) |
| 5 Observability & audit | `06_SDLC_DEVOPS_DEPLOYMENT.md`; `03_DECISION_LOG.md`; `07_THREAT_MODEL.md`; `04_ARCHITECTURE.md` §4; `10_QA_HISTORY.md` A6; `19_PRD.md` |
| 6 Supply chain & vendor denylist | `vendor_gate.py`; `ci.yml`; `02_CONSTITUTION.md` §7; `01_CHARTER.md`; `03_DECISION_LOG.md`; `06_SDLC_DEVOPS_DEPLOYMENT.md`; `12_CLAUDE_CODE_KICKOFF_PROMPT.md`; `BUILDEROS-CONSTITUTION-GooseClaw.md` (§7.1 divergence); [08R] (Trivy KEV) |
| 7 Decisions record | `03_DECISION_LOG.md` (spine); [D0901] via kickoff record + `08_PIN_REGISTER-refreshed-2026-09-03.md` (White Noise row); `10_QA_HISTORY.md` §Amendments (traceability); `09_OPEN_DECISIONS.md` (superseded rows) |
| 8 Phases / Phase-0 exit / first milestone | `05_BUILD_PLAN.md`; `12_CLAUDE_CODE_KICKOFF_PROMPT.md`; `19_PRD.md`; `ci.yml`; [08R] |
| 9 Open questions | [08R]; `04_ARCHITECTURE.md` §3; `03_DECISION_LOG.md`; `10_QA_HISTORY.md` A4/A10; `BUILDEROS-CONSTITUTION-GooseClaw.md` |
| 10 Conflicts | `08_PIN_REGISTER.md` vs [08R]; `03/07` vs [08R]; `09` vs [D0901]; `04 §4`/`10` vs [08R]; `vendor_gate.py`/`12`; `12` vs [08R] |
| Appendix B | `08_PIN_REGISTER.md`; `08_PIN_REGISTER-refreshed-2026-09-03.md`; `12_CLAUDE_CODE_KICKOFF_PROMPT.md` |

Note: `13`–`18`, `21`, the `SOURCE_*` files, the proposal files, `11`, and `PROJECT_INSTRUCTIONS.md` were consulted for context; where they overlap the numbered canon, the numbered canon leads `[00]`.

## Appendix B — Pin register pointer

The pin register is the volatile-facts system-of-record: **`08_PIN_REGISTER.md`** (last full canon verification 2026-07-06) **superseded in as-of dates by `08_PIN_REGISTER-refreshed-2026-09-03.md` [08R]** (primary-source re-verification, 2026-09-03). Rules `[08] [08R] [12]`: re-verify every pin against its primary source before Phase 0 and before any bump; **flag drift, never silently fix**; every pin carries a source URL + as-of date; a pin resolved without a primary source is a fabricated hash `[BC]`. Key current values `[08R]`: goose **v1.48.0** (2026-08-27, re-pin pending Martin's ratification); `agent-client-protocol` crate **2.0.0**, wire protocol **1**; MCP spec **2026-07-28 FINAL** (stateless; re-validate before Phase 2); rust-nostr → **`nostrdevkit/nostr`** (`nostr` 0.45.4 / `nostr-sdk` 0.45.2, ALPHA; **NIP-61 not implemented**); CDK **0.18.0** (ALPHA); Wasmtime current **48.0.1**, IronClaw main pins **47.0.4**; IronClaw **1.4.0** (2026-08-27, commit 4cb47cf, ~68 crates, MIT OR Apache-2.0); libSQL confirmed + **maintenance-mode caveat**; White Noise via **`marmot-protocol/mdk`**; Trivy banned (CVE-2026-33634, CISA KEV). **IronClaw attribution requirement: NOTICE citing `nearai/ironclaw@<pinned-commit>` is required BEFORE any reuse** (MIT OR Apache-2.0 lineage into AGPL) `[03] [12] [CL] [BC]`; steals provenance must be re-derived against a pinned 1.x commit before any port `[08R] [10 A4]`.

---

## Coverage checklist (mandatory minimum content)

| # | Required element | Present | Where |
|---|---|---|---|
| 1 | Fiduciary steward, not autonomous; Observe → Draft → Surface → Execute, halts at execution | ✅ | §0, §1 (art. 2) |
| 2 | Seven crates + dependency direction | ✅ | §2 |
| 3 | Singular core-owned Rule-9 gate as PreToolUse deny hook; channels/payments call it; one enforcement point | ✅ | §4.1, §1 (art. 12) |
| 4 | ACP contract: force Approve, protocolVersion 1, GooseClaw-owned durable session state | ✅ | §3.1 |
| 5 | MCP-vault: writes only via Alfred (system of record), single-writer optimistic concurrency (human edit wins) | ✅ | §3.2 |
| 6 | Payments: per-spend approval + per-period caps, Cashu P2PK (NUT-11) via CDK | ✅ | §3.4 |
| 7 | Nostr identity/consent: inbound sender allowlist, NIP-46 fail-closed when bunker offline | ✅ | §4.2, §3.3 |
| 8 | Born-redacted telemetry: local-only append-only audit log, zero network egress | ✅ | §5 |
| 9 | Vendor denylist (Meta/OpenAI/xAI excluded direct+transitive, Google permitted), vendor_gate.py, §7.1 divergence + PROPOSED conformance-test strategy | ✅ | §6 |
| 10 | IronClaw attribution: NOTICE required BEFORE any reuse; MIT OR Apache-2.0 lineage | ✅ | Appendix B, §7.1 |
| 11 | Phase 0 exit criteria | ✅ | §8 |
| 12 | First milestone = ACP supervisor + Rule-9 gate ONLY (no channels/payments/Nostr publishing) | ✅ | §8 |
| 13 | The two 2026-09-01 decisions (Skillsmith IN for v1 external-only ELv2 + tripwire; Nostr → Signal → White Noise via mdk) | ✅ | §7.2 |
| 14 | All ten QA amendments reflected | ✅ | §7.3 (traceability table) |
