# 14 — Competitive Landscape & Benchmarking

GooseClaw benchmarked against the state of the art in always-on agent runtimes, from the Pass-1 research (see `10_QA_HISTORY.md`). Purpose: surface blind spots ("what don't we know we don't know") and record concrete **steal / avoid / watch** verdicts. All external facts verified against primary sources; see `08_PIN_REGISTER.md`.

## Benchmarking matrix

| System | What it addresses that GooseClaw's spec doesn't | What GooseClaw does better | Verdict |
|---|---|---|---|
| **goose (upstream, AAIF)** | Cron Scheduler + Recipes; subagents; sandbox mode; an **adversary reviewer** that watches for unsafe actions; ACP-over-HTTP; single-binary distribution | Fiduciary consent gate; Nostr-native identity; vendor denylist as a hard CI gate | **STEAL** the scheduler, adversary reviewer, and sandbox mode |
| **OpenClaw** | *(cautionary only)* | Consent gate; no default network exposure; no unvetted skill marketplace; no implicit-localhost trust | **AVOID** its default-open gateway, `0.0.0.0` bind, unauthenticated control UI, and unvetted skills |
| **IronClaw (nearai)** | Heartbeat proactive execution; RBAC; guardian command validation; skill **trust tiers**; sandbox backends; a mature safety pipeline | Nostr-native identity; fiduciary framing; born-redacted telemetry | **STEAL** the heartbeat pattern, guardian validation, trust tiers, and the safety pipeline (per `04_ARCHITECTURE.md` §4) |
| **ElizaOS** | Plugin/character system; multi-agent swarms; deployable-app model; OS distribution | Not token/Web3-coupled; sovereignty focus; safety-by-construction | **WATCH** the plugin extension-point architecture |
| **Letta (MemGPT)** | **Sleep-time compute**; self-editing core-memory blocks; OS-tiered memory | Alfred as a durable external system-of-record | **STEAL** the sleep-time agent for idle-period consolidation (optional, Phase 3+) |
| **Nostr DVM ecosystem** | NIP-90 marketplace; NIP-89 discovery; Rust SDKs for agent discovery/payments | — | **WATCH** NIP-90/89 for capability exposure — but **demand is weak**; do not build the business case on DVM revenue |

## Reference incidents (why the security posture is what it is)
- **OpenClaw (Jan 2026):** one-click RCE (**CVE-2026-25253**, CVSS 8.8) from an unauthenticated control UI trusting a query-string gateway URL; ~135k exposed instances across 82 countries; the ClawHub **"ClawHavoc"** campaign shipped ~342 malicious skills (Atomic macOS Stealer). **Lesson:** no default network exposure, no implicit-localhost trust, no unvetted skill marketplace — GooseClaw's consent gate + allowlist + sandbox negate this class.
- **Block "Operation Pale Fire" (Jan 2026):** Block's own red team compromised an employee laptop via prompt injection hidden in **zero-width Unicode** inside a shared goose Recipe. goose now strips zero-width Unicode from Recipe inputs and shows a pre-flight action preview. **Lesson:** sanitize invisible Unicode everywhere untrusted text enters, *including the summary shown to the human at the gate*.
- **Trivy (Mar 2026):** the `aquasecurity/trivy-action` tag-poisoning supply-chain attack (**CVE-2026-33634**, CVSS 9.4). **Lesson:** pin Actions to commit SHAs; Trivy is banned; use Syft + OSV-Scanner + Grype.

## Adjacent SOTA (landscape awareness only — not dependencies)
Claude Agent SDK / headless `claude -p` patterns and current MCP-security research reinforce the same conclusion: the durable defense is **architectural** (allowlisting, human-in-the-loop gates, breaking the exfiltration leg, container/gVisor/Firecracker sandboxing), not classifier-based — injection "may never be fully solved." LangGraph and Meta/OpenAI/Microsoft agent frameworks are noted for landscape only and are **excluded** as dependencies under the vendor denylist.

## Net positioning
GooseClaw is, uniquely among 2026 always-on agents, **built to avoid the exact failure class** (the lethal trifecta with autonomy left on) that produced the OpenClaw crisis. Its differentiation is the parts no fork gives you: the **Nostr identity/DM/custody layer** and the **Lightning/Cashu payments layer**, wrapped in a fiduciary consent model. Its risk is execution and unspecified engineering, not design philosophy.
