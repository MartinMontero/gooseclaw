# 16 — Risk Register

Project, execution, and dependency risks — **distinct from `07_THREAT_MODEL.md`** (which covers security/adversarial risk). Likelihood (L) and Impact (I): H/M/L. Owner is Martin unless noted. Review at each phase gate.

| # | Risk | L | I | Mitigation | Trigger to act |
|---|---|---|---|---|---|
| R1 | **goose's fast cadence breaks the ACP contract** (v1.33→v1.41 in ~10 weeks) | M | H | Build against the **ACP protocol version**, not goose internals; pin goose by tag; smoke-test recipe before promoting; keep last-good binary for rollback | Any goose minor bump before a phase gate |
| R2 | **ALPHA dependencies churn** (rust-nostr, Cashu CDK both self-label ALPHA/"use with caution") | H | M | Pin exact versions; wrap behind internal traits so a breaking change is contained to one crate; budget rework time | A breaking release lands mid-phase |
| R3 | **Solo-founder capacity vs. six-phase scope** | M | H | The 12-month grant gives slack; phase gates keep scope honest; defer non-core (Skillsmith, extra channels); recruit contributors if funded | Slipping a phase gate by >3 weeks |
| R4 | **Funding dependency** — build pace assumes the Goose Grant | M | M | Grant is milestone-based (partial funding still useful); the build is viable unfunded at a slower pace; commons/upstream angle is fallback value | Grant declined or delayed past Q1 |
| R5 | **IronClaw v1→v2 "Reborn" transition** deletes/relocates the crates being ported | M | M | Pin a **specific real commit** for the steal; port (not live-dependency) so upstream churn can't break the build; re-derive against a verified commit before porting | Porting begins (Phase 5) |
| R6 | **MCP 2026-07-28 RC is breaking** (stateless redesign, Tasks→extension) | M | M | Treat as a watch item; validate the MCP-over-ACP path against the RC before adopting; stay on stable 2025-11-25 until validated | RC finalizes |
| R7 | **License decision unresolved** (AGPL-3.0 vs the March draft's Apache/MIT) blocks a coherent grant submission and repo `LICENSE` | H | M | Decide before submitting the grant and before repo init; `GOOSE_GRANT_APPLICATION.md` uses AGPL-3.0 pending confirmation | Grant submission or repo init |
| R8 | **Llama / open-weights denylist ambiguity** — "exclude Meta" vs "permit open-weights" conflict | M | L | Rule needed for runtime provider filtering (layer 2/3); March draft implies Meta-origin wins | Before the setup wizard ships |
| R9 | **NIP-46 bunker is a single point of failure** for an always-on daemon | M | M | Fail-closed is by-design (correct for custody); document it; optionally add a scoped local signer for non-consequential reads only | Phase 4 |
| R10 | **Zapstore desktop distribution is less mature than its Android path** | L | M | Validate the zapstore-cli desktop flow early; keep signed-binary + container fallbacks; inherit Alfred's proven signing/escrow ceremony | Phase 6 packaging |
| R11 | **DVM monetization assumed** despite weak demand | L | M | Do **not** put DVM revenue in any plan/proposal; use NIP-89/90 for capability exposure only | Any monetization planning |
| R12 | **Supply-chain typosquat** on a self-hosted Skillsmith registry (Syft/OSV catch CVEs, not name-confusion) | L | M | Skill-Card review + trust tiers + install-time human consent; treat registry installs as Installed-tier (read-only) | If Skillsmith enters scope |
| R13 | **Vendor-gate false positive** on infra-from-a-denied-org (e.g. `zstd`/`rocksdb`, both `github.com/facebook/`) blocks CI | M | L | `EXCEPTIONS` map exists (empty by default); granting one is a documented human decision with rationale + date — never auto-added to go green | Gate fires in Phase 0 |

## Standing note
Risks R1, R2, R5, R6 all reduce to the same discipline: **pin against stable contracts (ACP protocol, a real IronClaw commit, MCP stable) and wrap volatile dependencies behind internal seams** so upstream churn is contained. R7 and R8 are **decisions Martin owes**, not risks to mitigate — see `09_OPEN_DECISIONS.md`.
