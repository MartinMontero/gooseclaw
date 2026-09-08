# 20 — Personas & Use Cases

## Personas

**P1 — The Targeted Journalist / Activist ("the exposed").** Operates under surveillance pressure; cannot trust hosted tools; metadata leakage is a physical-safety risk. Needs an agent that protects identity (NIP-17, NIP-46), never acts unattended, never exfiltrates, and can be killed instantly. *This persona sets the defaults — safety is unbypassable because of them.*

**P2 — The Sovereign Builder.** Freedom-tech developer. Wants to self-host, extend, and audit everything; rejects vendor lock-in and telemetry. Values the ACP seam (swappable engine), the skills system, and AGPL. Will author skills and contribute upstream.

**P3 — The Collective / Non-profit Operator.** Runs shared infrastructure for a community (mutual aid, independent media, a co-op). Needs auditability, spend caps, and safety guarantees they can explain to their members. Homebase-adjacent.

**P4 — The Cautious Power User.** Wants meaningful automation (scheduled research, triage, drafting) but is unwilling to hand an AI unsupervised authority. The draft-and-surface model is exactly their comfort zone.

## Jobs to be done
- "When I'm away from my desk, help me stay on top of my relays and draft responses I can approve later — without ever posting as me on its own."
- "Run my recurring research and hand me a draft, so I decide what ships."
- "Pay this invoice for me, but never spend beyond a cap or without my yes."
- "Let me run a community-built skill without trusting it with everything."
- "Filter the strangers messaging me so only people I've allowed can reach the agent."

## Use cases / user stories
- **Relay watch → draft:** the heartbeat observes watched relay events (read-only), drafts a reply or note, surfaces it; the user approves; only then it publishes (FR2, FR3, FR9).
- **Scheduled research → draft:** a cron routine runs a research task via goose, writes findings to Alfred through the MCP vault, and surfaces a summary draft (FR1, FR5, FR9).
- **Gated payment:** the agent proposes a Lightning/Cashu payment; the gate shows amount + destination + remaining cap; the user signs an authenticated approval; the payment executes (FR7, FR3, FR8).
- **Sandboxed skill:** the user installs a community skill (Installed tier, read-only tools); it requests a capability lease; the user grants a scoped, time-bounded lease (FR6).
- **Allowlisted DMs:** a stranger DMs the agent; they are quarantined (no tools, no vault); a known contact's DM is processed normally (FR4, FR2).
- **Approved publishing:** the agent drafts a long-form note with NIP-32 labels; the user approves; it publishes (FR3).

## Day in the life (P1)
Morning: the agent has drafted three relay replies overnight and a research summary; nothing was sent. The user reviews on their phone over an encrypted NIP-17 DM, approves two replies and edits the third. Midday: a payment request appears for a $4 tool — the user approves; a second request would exceed the daily cap and is blocked pending an explicit raise. Evening: an unknown pubkey tried to instruct the agent; it was quarantined and logged. The user checks the local audit feed, sees the full trail, and pauses the agent for the weekend with the kill switch.

## Anti-personas (explicitly not served)
- Users wanting a fully autonomous agent that acts without approval.
- Engagement-maximizing or growth-hacking use.
- Surveillance, stalking, or harassment use — excluded by ethos and by `21_MODERATION_AND_ABUSE.md`.
