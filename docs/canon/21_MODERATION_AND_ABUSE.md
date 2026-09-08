# 21 — Moderation & Abuse

The design spec for handling abuse of, and abuse *via*, a public-facing always-on Nostr agent. Flagged as missing in both QA passes (`10_QA_HISTORY.md`) and referenced by the PRD/personas. Phase 3 deliverable.

## The two abuse directions
1. **Abuse *of* the agent** — strangers prompting, flooding, or injecting a public agent (prompt injection, event floods, social-engineering the approval gate). Covered primarily by the security controls in `07_THREAT_MODEL.md`.
2. **Abuse *via* the agent** — someone trying to use GooseClaw itself as a tool for harassment, stalking, spam, or surveillance. Covered here and excluded by ethos (`20_PERSONAS_AND_USE_CASES.md` anti-personas).

## Inbound authorization (the first gate)
- **Sender allowlist by default (Constitution 11).** Unknown pubkeys are quarantined: no vault reads, no tool exposure, no consequential action. Their messages are treated as untrusted data and logged, not executed.
- **Tiers:** *Allowed* (full interaction, still gated by Rule 9) · *Quarantined* (default for unknown; can request the human promote them) · *Blocked* (silently dropped).
- **Rate limits** per sender and per relay to blunt floods (ties to `07_THREAT_MODEL.md` DoS controls).

## NIP-56 report handling
- GooseClaw both **consumes** and can **emit** NIP-56 reports (kind 1984).
- *Consuming:* reports about a pubkey the agent interacts with raise that sender's scrutiny (auto-quarantine on credible reports from allowlisted sources); never auto-block solely on an unverified third-party report — surface to the human.
- *Emitting:* the agent may draft a report (e.g., on receiving abuse) but **publishing it is Rule-9-gated** like any outbound event.
- Report *types* handled: spam, illegal, profanity, impersonation, malware/nudity as defined by NIP-56 — mapped to quarantine/block recommendations, not automatic action.

## NIP-32 labeling
- Used to annotate content the agent has assessed (e.g., labeling a draft's sources, or tagging quarantined content) — see `13_NIP_COMPLIANCE_MATRIX.md`. Labels are informational; they inform the human, they don't gate autonomously.

## Block/mute lists
- Support importing and honoring the user's existing Nostr mute lists (kind 10000) and maintaining a local block list.
- Blocked senders are dropped pre-model — their content never reaches the agent's context.

## Harassment / stalking exclusion (abuse *via*)
- The agent must not be usable to target a person: no mass-DM, no scraping-for-surveillance, no impersonation. These are excluded by the anti-personas and enforced structurally — outbound is Rule-9-gated and capped, and there is no bulk-send primitive.
- Content that would facilitate targeting a person is refused at draft time.

## Liability & operator guidance (public deployments)
- A public agent that strangers can prompt is an operator responsibility. The docs must state: run allowlisted by default; understand that quarantined-but-logged inbound content is retained locally; and that the operator is responsible for what their instance publishes (all publishing is human-approved by design, which is the primary liability control).
- **No unattended consequential action** is the single most important abuse-mitigation property: the agent cannot be tricked into acting because it does not act without a human ✅.

## Data retention for abuse material
- Quarantined/abusive inbound content is written only to the local audit log (no network egress), retained per the data-retention policy (to be authored alongside this in Phase 2/3), and never forwarded.

## Open items
- The exact auto-quarantine thresholds (how many credible reports, from which trust tier) — to be tuned in Phase 3.
- Whether to support a shared community block list (sovereignty vs. convenience trade-off) — defer to post-v1.
