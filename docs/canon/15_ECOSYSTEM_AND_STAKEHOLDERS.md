# 15 — Ecosystem & Stakeholders

Who and what GooseClaw depends on, integrates with, or answers to. Keeps a fresh assistant from having to reconstruct the landscape.

## Upstream / foundations
- **Block, Inc. → Agentic AI Foundation (AAIF, under the Linux Foundation).** Stewards **goose** (migrated from `block/goose` to `aaif-goose/goose`, Dec 2025; Apache-2.0). Also the funder of the **Block Goose Grant** ($100K/12mo) — GooseClaw's primary funding target. AAIF co-launched with Anthropic's MCP and OpenAI's AGENTS.md as inaugural projects. *Relationship:* GooseClaw is a downstream consumer of goose over ACP and an on-thesis grant candidate; consider contributing the Nostr channel upstream.
- **Zed Industries + JetBrains.** Created and co-maintain **ACP** (Agent Client Protocol); co-launched the ACP Registry. *Relationship:* ACP is GooseClaw's stable engine seam.
- **Anthropic.** Stewards **MCP**; provider of Claude (a **permitted** model provider). *Relationship:* MCP is the Alfred federation seam; Claude Code Desktop is the build executor.
- **nearai.** Publishes **IronClaw** (`nearai/ironclaw`, MIT OR Apache-2.0). *Relationship:* GooseClaw steals IronClaw's audited security *designs* (safety pipeline, Wasmtime sandbox) with attribution — no fork. Watch the v1→v2/"Reborn" transition (large LOC churn).

## The Builder OS triad (Martin's own ecosystem)
- **Alfred** — the memory system-of-record. Local-first Nostr-native PKM (Tauri 2 + SolidJS, AGPL-3.0), embeds goose over ACP, exposes a path-confined MCP vault. *Relationship:* GooseClaw federates with Alfred over MCP (Option C); Alfred is the deployment-sidecar host (recommended primary packaging). **As of 2026-07-17, Alfred's updater-keypair signing ceremony is complete and escrowed and a full signed-release dry run passed** — a proven signing/escrow pattern GooseClaw inherits (it still needs its own ceremony).
- **WCJBT (wecanjustbuildthings.dev)** — the catalog/architect. ~1,300 entries, three-layer vendor-exclusion engine, Syft + OSV-Scanner supply chain (Trivy banned). *Relationship:* source of the CI vendor-gate pattern; receives GooseClaw's verified-learnings as NIP-32 labels.
- **Holmes** — the research/detective brain of the WCJBT triad (zero-hallucination research, confidence scoring). *Relationship:* methodological sibling; the evidence-over-assertion posture.
- **AOS (And Other Stuff)** — the freedom-tech collective Martin operates through; stewards Nostr ecosystem projects (diVine, Zapstore, Cashu, White Noise). *Relationship:* GooseClaw's organizational home and the framing for the Goose Grant; distribution via **Zapstore**.

## Protocol/tooling dependencies (maintainers)
- **rust-nostr** (`nostr` / `nostr-sdk`) — the Nostr implementation. ALPHA.
- **Cashu / CDK** (`cashubtc/cdk`) — eCash. ALPHA.
- **Bytecode Alliance** — Wasmtime (skill sandbox).
- **Turso / libSQL** — embedded persistence (FTS5 + native vector).
- **Anchore** — Syft + Grype (SBOM + vuln scan). **Google** — OSV-Scanner (permitted).
- **smith-horn/skillsmith** — optional skill discovery. Elastic-2.0 (license caveat; keep optional).

## Provider policy at a glance
- **Excluded (hard denylist):** Meta, OpenAI, xAI — as dependencies **and** model providers.
- **Permitted:** Google, Mistral, Ollama, MiniMax, Anthropic; open-weight models.
- **Unresolved edge:** Llama is open-weight **and** Meta-origin — the March grant draft's model list omits it, implying Meta-origin wins. Needs a ruling before the setup wizard ships (runtime filtering, layer 2/3 — not the CI gate). See `16_RISK_REGISTER.md`.

## Funders (GooseClaw)
- **Primary:** Block Goose Grant (see `PROPOSAL_goose_grant.md` / `GOOSE_GRANT_APPLICATION.md`).
- *(GooseClaw-only workspace — Martin's other-initiative funders, e.g. HRF/Mozilla/OpenSats/Vancouver Foundation, live in the separate `aos-portfolio-proposals/` workspace.)*
