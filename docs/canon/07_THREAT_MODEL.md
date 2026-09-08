# 07 — Threat Model

Scope: an **unattended, always-on, Nostr-connected agent** with read access to a private vault (via Alfred), an LLM in the loop, and outbound reach (Nostr publish/DM/zap, HTTP egress). This is exactly the profile that produced the OpenClaw security crisis; the design's job is to break that class structurally, not to detect it.

## The core danger: the lethal trifecta
An agent that simultaneously (1) reads private data, (2) ingests untrusted content, and (3) can exfiltrate over a channel is exploitable. GooseClaw satisfies all three by nature. Mitigations, in priority:
- **Break leg 3 for untrusted-triggered actions.** No auto-outbound with private data in context. Every consequential egress passes the human gate.
- **Gate leg 2 at the door.** Inbound sender allowlist (Constitution 11); unknown senders get no vault reads and no tool exposure. All inbound text is treated as **data, never instructions**.
- **Assume detection fails.** Prompt-injection classifiers are bypassable (this is the upstream consensus). Rely on architecture — allowlisting, the consent gate, egress control — not on scanning.

## STRIDE

| Threat | Vector (GooseClaw-specific) | Control |
|---|---|---|
| **Spoofing** | Forged Nostr events; a fake "approver" ✅ | Verify event signatures; honor approvals only if signed by an **allowlisted approver pubkey** |
| **Tampering** | Unicode Tag Block (U+E0000–U+E007F), zero-width (U+200B/C), bidi confusables in inbound DMs **and in drafted approval summaries** | Strip tag-block/zero-width/bidi from all inbound content **and** from the plain-English text shown at the gate; validate before wrapping into model-visible markup |
| **Repudiation** | "The agent did X and there's no record" | Append-only local **audit log** (decision/dispatch/audit), no network egress |
| **Information disclosure** | Vault secrets exfiltrated via injected instruction → outbound | Break the exfil leg (above); secrets injected only at egress to allowlisted targets; model/skills never see raw secrets |
| **Denial of service** | Relay event floods; restart-storm; runaway token spend | Relay-message rate limiting; supervisor restart damping; global token/CPU/memory caps |
| **Elevation of privilege** | Malicious community skill escaping the sandbox | Wasmtime component-model deny-by-default + resource limiter; Trusted/Installed trust tiers; capability leases |

## Named risks beyond STRIDE
- **Approval-gate social engineering.** Approval fatigue and ✅-spam. Controls: authenticated, single-use, action-hash-bound approvals (Constitution 13); the gate shows *which tool, what data, what external destination* so the human can catch a trifecta attempt; rate-limit approval prompts.
- **Bunker-offline availability.** NIP-46 bunker down = agent **fails closed** (mute) rather than signing unsigned. Accepted as correct for custody; documented as a design property. Caveat: NIP-46 still depends on NIP-04 encryption (nostr-protocol/nips issue #1095) — the bunker channel crypto is weaker than the NIP-44 DM path.
- **Supply-chain / typosquatting.** Syft/OSV/Grype catch known CVEs, **not** name-confusion on a self-hosted Skillsmith registry. Control: Skill-Card review + trust tiers + human install-time consent; treat registry installs as Installed-tier (read-only) by default.
- **Cashu proof theft.** Local proof storage is bearer value. Control: encrypted at rest; spend caps limit blast radius; P2PK where possible.
- **Metadata leakage despite NIP-17.** An always-on agent's relay connections are a persistent IP fingerprint. Control: consider Tor/mixnet for relay I/O; document the residual risk for at-risk users.

## Reference incidents (why these controls exist)
- **OpenClaw (Jan 2026):** one-click RCE (CVE-2026-25253) from an unauthenticated control UI trusting a query-string gateway URL; ~135k exposed instances; the ClawHub "ClawHavoc" campaign shipped ~342 malicious skills. Lesson: no default network exposure, no implicit-localhost trust, no unvetted skill marketplace.
- **Block "Operation Pale Fire" (Jan 2026):** Block's own red team compromised an employee laptop via prompt injection hidden in **zero-width Unicode** inside a shared goose Recipe; goose now strips zero-width Unicode from Recipe inputs and shows a pre-flight action preview. Lesson: sanitize invisible Unicode everywhere untrusted text enters, including anything rendered to the human at the gate.
- **Trivy (Mar 2026):** the `aquasecurity/trivy-action` tag-poisoning supply-chain attack (CVE-2026-33634). Lesson: pin Actions to commit SHAs; Trivy is banned; use Syft + OSV-Scanner + Grype.
