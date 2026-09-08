# 19 — Product Requirements Document (PRD)

## Problem
There is no always-on agent runtime that speaks freedom-tech protocols (Nostr, Lightning) natively, and the always-on agents that exist are structurally unsafe for the people most exposed — the OpenClaw class shipped with default network exposure, no consent gate, and unvetted skill marketplaces. People who need automation but cannot surrender sovereignty or authority have nothing to run. GooseClaw exists to give them an always-on agent that augments their agency without ever acting unattended.

## Goals
Sovereign, local-first, always-on agent runtime; human-in-the-loop by construction (Observe → Draft → Surface → Execute, halting at consequence); Nostr-native identity/DM/custody; safe-by-construction (breaks the lethal trifecta by default); a goose *extension over ACP*, not a fork; AGPL commons.

## Non-goals
Not autonomous; not a goose fork; not embedded in Alfred; not a hosted SaaS with per-user COGS; not engagement-maximizing; not a general chatbot. It does not try to win on hosted convenience or DVM revenue.

## Users
See `20_PERSONAS_AND_USE_CASES.md`. Primary: at-risk journalists/activists; sovereign builders; collective/non-profit operators.

## Functional requirements
- **FR1** Drive goose over ACP as a supervised subprocess, forced to Approve mode.
- **FR2** Nostr NIP-17 DM channel (NIP-44 encryption, NIP-59 wrap) as the first channel.
- **FR3** A single, core-owned, non-bypassable Rule-9 consent gate wrapping every consequential action.
- **FR4** Inbound sender allowlist; unknown senders quarantined; all inbound text treated as untrusted data.
- **FR5** Federate with Alfred's MCP vault (reads free in path root; writes through Alfred; single-writer concurrency).
- **FR6** Wasmtime component-model skill sandbox with deny-by-default capabilities, trust tiers, capability leases, and a defined WIT world.
- **FR7** Lightning + Cashu payments with per-spend approval and per-period caps.
- **FR8** NIP-46 remote key custody (no raw nsec on host); authenticated, single-use approvals; fail-closed when bunker offline.
- **FR9** Draft-and-surface heartbeat (cron/event/webhook → read-only observe → draft → approval); no unattended consequential action.
- **FR10** Local, append-only audit log (no network egress).
- **FR11** Multi-channel router (Nostr, CLI, local web UI) with per-channel/user session isolation and message chunking.
- **FR12** Setup wizard: key generation/import, provider selection (denylist-filtered), relay config, approval policy, caps, allowlist.

## Non-functional requirements
- **Security:** per `07_THREAT_MODEL.md` — injection sanitization, egress control, sandbox isolation.
- **Sovereignty:** local-first; zero telemetry (no phone-home); no network bind by default; no mandatory hosted dependency.
- **Reliability:** 30+ day continuous operation on consumer hardware; crash recovery via owned session state (goose ACP has no resume).
- **Resource governance:** global token/CPU/memory caps; restart-storm damping.
- **Observability:** local structured audit log; W3C trace correlation across the ACP seam.
- **Portability:** Tauri sidecar (primary), rootless container, Start9/Umbrel, localhost systemd.
- **Licensing:** AGPL-3.0-or-later (see `03_DECISION_LOG.md`; the open license question is R7 in `16_RISK_REGISTER.md`).

## MVP
**Phases 0–3** = MVP: ACP spike + core consent gate + Alfred federation + Nostr channel with allowlist and authenticated approvals. Payments (Phase 4), skills sandbox (Phase 5), and heartbeat (Phase 6) are post-MVP. Rationale: the MVP proves the thesis — a sovereign always-on agent that drafts and waits — with the smallest safe surface.

## Success metrics
Measurement is constrained by zero telemetry, so metrics are **proxy or opt-in**, never silently collected. The measurement tension is explicit and unavoidable.
- **North star:** at-risk users safely running an always-on agent (not directly measurable; proxied by community signal, self-reported deployments, relay-visible NIP-89 handler announcements if the user opts to advertise).
- **Adoption (proxy):** Zapstore installs, GitHub stars/forks, self-reported deployments.
- **Commons health:** external skills authored; the Nostr channel contributed upstream to goose and merged; downstream forks/adaptations.
- **Reliability:** uptime in self-reported/CI soak testing; crash-recovery success rate.
- **Security:** red-team suite pass rate (`06_SDLC_DEVOPS_DEPLOYMENT.md` testing pyramid); external audit findings closed.
- **Delivery:** Goose Grant quarterly milestones met (`GOOSE_GRANT_APPLICATION.md`).

## Product acceptance criteria
Per-phase gates in `05_BUILD_PLAN.md` are the acceptance criteria; each gate = all tests pass + explicit human sign-off.
