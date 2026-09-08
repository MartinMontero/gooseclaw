# 17 — Roadmap

A time-based, milestone-oriented view of the build, mapped to the Block Goose Grant's 12-month / quarterly structure. The engineering detail lives in `05_BUILD_PLAN.md` (phase gates) and `06_SDLC_DEVOPS_DEPLOYMENT.md`; this is the calendar view for planning and grant reporting. Pacing is indicative — the grant is milestone-based, so pace to real capacity.

## Now (pre-Phase-0 — concept → ready)
- **Status:** concept-stage, no repo. Mission Control knowledge base assembled; foundation spec QA'd across two passes; repo-seed (`CLAUDE.md`, CI, vendor gate) pre-authored.
- **Blockers to close first:** the license decision (R7), and confirming the two remaining open decisions (Skillsmith scope, channel priority — `09_OPEN_DECISIONS.md`).
- **Exit:** hand `12_CLAUDE_CODE_KICKOFF_PROMPT.md` to Claude Code Desktop.

## Q1 (Months 1–3) — Foundation
Phases **0–1**. Verify all pins; scaffold the seven-crate workspace; drive goose over ACP; prove the `prompt → tool call → ActionRequired → authenticated human ✅ → resume` consent loop; CI green with the vendor + supply-chain gates; setup wizard; Nostr identity + channel gateway (NIP-17 + local web UI).
- **Milestone gate:** a user installs GooseClaw, generates a Nostr identity, and interacts with their goose-powered agent over encrypted NIP-17 DMs and a local web UI, with the core consent gate enforced.

## Q2 (Months 4–6) — Sovereign memory + safe skills
Phases **2 + 5 (partial)**. Alfred MCP-vault federation (read-free / write-through, single-writer concurrency, RRF hybrid search); Wasmtime skill sandbox with a defined WIT world, trust tiers, capability leases; port the `gooseclaw-safety` pipeline (attributed); supply-chain gating for community skills.
- **Milestone gate:** the agent reads/writes sovereign memory through Alfred, runs a sandboxed community skill under explicit capability grants, and passes the prompt-injection red-team suite.

## Q3 (Months 7–9) — Multi-channel, payments, proactive heartbeat
Phases **3 + 4 + 6 (core)**. Multi-channel router with the shared authenticated approval gate + inbound allowlist + NIP-56 moderation handling; Lightning + Cashu (NUT-11 P2PK) with per-spend approval + caps; approval-gated Nostr publishing; the draft-and-surface **heartbeat** (no unattended consequential action).
- **Milestone gate:** an always-on agent that communicates across channels, processes Lightning/Cashu payments under caps, and proactively drafts scheduled work for approval.

## Q4 (Months 10–12) — Hardening, packaging, release
Phase **6 (complete)** + release. Security audit (injection/leak/sandbox-escape/privilege-escalation) + red-team suite + agent-behavior eval harness; 30+ day continuous-operation resource management; signed binaries + Tauri-sidecar (Alfred) + rootless-container + Start9/Umbrel packaging; full docs; **v1.0 release candidate** via Zapstore; contribute the Nostr channel upstream to goose.
- **Milestone gate:** **GooseClaw v1.0** — production-ready, security-audited, documented; installable in minutes; extensible by any freedom-tech developer.

## Cross-references
- Phase acceptance criteria: `05_BUILD_PLAN.md`. Grant milestone text: `GOOSE_GRANT_APPLICATION.md` (Page 4). Risks to the schedule: `16_RISK_REGISTER.md` (R1–R4 most schedule-relevant).
