# 01 — Charter

## What GooseClaw is
A Nostr-native, **always-on fiduciary agent runtime**. It drives Block/AAIF **goose** over the **Agent Client Protocol (ACP)**, federates with **Alfred** over Alfred's path-confined MCP vault, and reuses **IronClaw**'s security/sandbox *designs* — without forking IronClaw or embedding anything autonomous inside Alfred.

It is a **fiduciary steward, not an autonomous agent**. Its loop is **Observe → Draft → Surface → Execute**, and it **halts at the moment of execution** to await explicit human consent. It has no interests of its own; it extends the user's capability and never displaces the user's authority.

## The Builder OS triad (and where GooseClaw sits)
- **WCJBT** (wecanjustbuildthings.dev) — *the Architect*: decides what to build; the catalog + vendor-exclusion engine.
- **Holmes** — *the Detective*: establishes what is true; evidence, confidence scoring, zero-hallucination research.
- **Alfred** — *the Memory / system-of-record*: the sovereign, local-first Markdown vault; specs, decisions, history that compound across sessions.
- **goose** — *the Hands*: the capability engine that executes tasks; no independent memory or judgment.
- **GooseClaw** — *the Body & Judgment*: the always-on connective tissue that reads Alfred's memory, commands goose's hands, and enforces the human-safety rules. Holds only ephemeral state; Alfred stays the system-of-record.

The relationship: **Memory is not Hands.** GooseClaw never runs inside Alfred and never turns Alfred into an agent; they communicate only across Alfred's typed MCP boundary.

## Who it is for
Sovereignty-focused builders, collectives, and non-profits — and, first in priority, **marginalized and targeted communities** for whom metadata protection and structural safety are not features but requirements. The design premise: *the people most exposed are the ones this serves*, so safety is the unbypassable default rather than a setting.

Product framing note (shared with Alfred): the identity is **Augmented Intelligence** — augmenting human agency and will, "Human in the Lead" — not "AI that does things for you."

## Ethos (the non-negotiable posture)
Liberation technology; sovereignty; **local-first**; **zero telemetry** (no outbound phone-home — distinct from *no logs*, see `07_THREAT_MODEL.md`); **no public network bind by default**; **no mandatory hosted dependency**; **no vendor lock-in**. Source stays **AGPL-3.0-or-later**; network-served instances offer their source (AGPL §13). The vendor line holds: **Meta, OpenAI, xAI are hard-excluded**; Google/Mistral/Ollama/MiniMax/Anthropic are permitted.

## Strategy
1. **Prove the seam first.** The entire foundation rises or falls on driving a goose ACP subprocess with the Rule-9 gate in the loop (Phase 1). Everything else depends on it.
2. **Steal, don't fork.** Lift IronClaw's audited, permissively-licensed security *mechanisms* (safety pipeline, Wasmtime sandbox) with attribution; author the *governance* (Rule-9 gate, trust model, policy) clean, because that governance encodes GooseClaw's values and must be fully auditable.
3. **Engine-agnostic at the ACP boundary.** Route to goose by default; keep the option to route other ACP agents later. goose 2.0 is being built *behind* ACP, so the seam is the stable contract, not goose's internals.
4. **Differentiate on the parts no fork gives you:** the Nostr identity/DM/custody layer and the Lightning/Cashu payments layer.
5. **Fund via the commons, not enclosure.** Pursue the Goose Grant; consider contributing the Nostr channel upstream to goose as an MCP extension so the commons benefits regardless of GooseClaw's own trajectory.

## Funding / commons angle
The **Block Goose Grant** (up to **$100,000 / 12 months**, rolling, milestone-based, quarterly reviews) lists focus areas including *self-flying capabilities (autonomous background operations)* and *self-improving agents* — a direct match for an always-on, Rule-9-gated runtime. GooseClaw's AAIF/Linux-Foundation alignment (goose, not IronClaw, as the base) is part of what makes it an on-thesis candidate. Full application in `PROPOSAL_goose_grant.md`. [Verified — grant parameters confirmed against Block's Goose Grant program; see `08_PIN_REGISTER.md`.]
