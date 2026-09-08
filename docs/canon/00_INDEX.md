# 00 — Index & Navigation

**GooseClaw Mission Control.** The single source of truth for building GooseClaw and for the adjacent AOS / freedom-tech proposal work. Concept-stage; no repo yet. As-of 2026-07-17.

## How to read this workspace
Precedence, highest first: **verified primary source → the numbered canon (`03_DECISION_LOG.md` leads) → the kickoff prompts (`11`/`12`)**. The numbered canon supersedes the `SOURCE_*` documents wherever they differ. On any conflict, flag and stop. All external pins are volatile — re-verify against `08_PIN_REGISTER.md`'s sources before relying on them.

## File map

The knowledge base has four layers: **governance**, **canon** (the living decisions/design), **reference** (the connective research), and **operating prompts** — plus preserved **source** documents and the GooseClaw **proposal**.

### Governance
| File | What it is | Read it when |
|---|---|---|
| `PROJECT_INSTRUCTIONS.md` | Role, hard rules, operating conventions | Always loaded; governs behavior |
| `00_INDEX.md` | This map | Orienting |

### Canon (living — the what/why/how)
| File | What it is | Read it when |
|---|---|---|
| `01_CHARTER.md` | Mission, triad, users, ethos, strategy, funding | Understanding *what* and *why* |
| `02_CONSTITUTION.md` | Non-negotiables (10 + 3 security amendments) | Any values/architecture question |
| `03_DECISION_LOG.md` | Ratified + amended decisions (canonical) | Any "what did we decide" question |
| `04_ARCHITECTURE.md` | Crates, ACP+MCP contracts, topology, IronClaw steals | Any build/design question |
| `05_BUILD_PLAN.md` | Phases 0–6, immediate tasks | Planning/sequencing |
| `06_SDLC_DEVOPS_DEPLOYMENT.md` | Repo, CI/CD, supply chain, testing, deploy, observability | DevOps/release questions |
| `07_THREAT_MODEL.md` | STRIDE, lethal trifecta, approval-gate auth, Pale Fire lessons | Any security-relevant ruling |
| `08_PIN_REGISTER.md` | Every version pin, as-of date, source URL | Before trusting any version claim |
| `09_OPEN_DECISIONS.md` | What still needs ratification | Deciding next steps |
| `10_QA_HISTORY.md` | The two-pass audit trail + the 10 amendments | Understanding how we got here |

### Reference (the connective research)
| File | What it is | Read it when |
|---|---|---|
| `13_NIP_COMPLIANCE_MATRIX.md` | Every Nostr NIP used, kinds, status, source | Any Nostr protocol question |
| `14_COMPETITIVE_LANDSCAPE.md` | Benchmarking vs goose/OpenClaw/IronClaw/ElizaOS/Letta/DVMs; steal/avoid/watch | Positioning; what to borrow/avoid |
| `15_ECOSYSTEM_AND_STAKEHOLDERS.md` | Who's who: AAIF, nearai, Zed, the triad, funders, providers | Any "who/what is X" question |
| `16_RISK_REGISTER.md` | Project/execution/dependency risks (not security) | Planning; gate reviews |
| `17_ROADMAP.md` | Time-based quarterly view mapped to grant milestones | Scheduling; grant reporting |
| `18_GLOSSARY.md` | Definitions of every term, protocol, concept | Any unfamiliar term |
| `21_MODERATION_AND_ABUSE.md` | Inbound authorization, NIP-56 handling, abuse-via-agent exclusion | Public-surface / abuse questions |

### Product
| File | What it is | Read it when |
|---|---|---|
| `19_PRD.md` | Problem, goals, FR/NFR, MVP scope, success metrics | Any "what must it do / is X in scope" question |
| `20_PERSONAS_AND_USE_CASES.md` | The four personas, jobs-to-be-done, user stories, anti-personas | Product/UX decisions; who it's for |

### Operating prompts
| File | What it is | Read it when |
|---|---|---|
| `11_PROJECT_KICKOFF_PROMPT.md` | First-message prompt for a fresh reviewer assistant | Starting a review session |
| `12_CLAUDE_CODE_KICKOFF_PROMPT.md` | Phase 0→1 build prompt for Claude Code Desktop | Handing off to the executor |

### Source documents (preserved verbatim for provenance)
| File | What it is |
|---|---|
| `SOURCE_foundation-spec-v1.md` | The original foundation spec (Path B+), as uploaded |
| `SOURCE_project-kickoff-prompt.md` | The original architect/reviewer kickoff, as uploaded |
| `SOURCE_claude-code-kickoff-prompt.md` | The original Claude Code kickoff, as uploaded |

*These are the raw inputs. Where they differ from the numbered canon, the **numbered canon wins** (it carries the QA amendments); the sources are kept for provenance and audit.*

### Proposal (GooseClaw funding)
| File | What it is |
|---|---|
| `PROPOSAL_goose_grant.md` | Block Goose Grant — status, facts, corrections, open items |
| `GOOSE_GRANT_APPLICATION.md` | The full six-page submittable application text |

**Scope note:** Mission Control is **GooseClaw-only**. The adjacent AOS/portfolio proposals (AOS nostr.com Vancouver, Homebase→HRF, Homebase→Mozilla, Basecamp YVR) live in a **separate workspace** (`aos-portfolio-proposals/`), not here.

**Repo seed (not project knowledge — these go in the GooseClaw repo):** `CLAUDE.md`, `.github/workflows/ci.yml`, and `scripts/vendor_gate.py` are pre-authored in `gooseclaw-repo-seed/`. Claude Code reads them as canonical and must not regenerate them; Phase 0 only resolves their 9 pin placeholders and fills the Commands block. Everything else (workspace, crate stubs, AGENTS.md, deny.toml, LICENSE, Spec Kit docs) is generated in Phase 0.

## Status snapshot (2026-07-17)
- **Foundation:** sound to proceed to Phase 0/1, conditional on the 10 amendments in `10_QA_HISTORY.md` §Amendments.
- **Two gating open decisions resolved:** persistence = **libSQL**; steal mechanism = **port the security *mechanism*, author the *governance* clean** (see `03_DECISION_LOG.md`).
- **Remaining open decisions:** Skillsmith scope; channel priority beyond Nostr (see `09_OPEN_DECISIONS.md`).
- **Critical pre-build items:** inbound-DM sender allowlist; single core-owned Rule-9 gate; approval-message authentication (see `07_THREAT_MODEL.md`).
- **Canonical files recovered:** the two kickoff prompts + the foundation spec. Files `01`–`06` and `07_DECISION_LOG.md` from the original nine-file package were **not** recoverable as source; their content is reconstructed here from the foundation spec and the two QA passes. Google Drive holds only derivative NotebookLM decks.
- **Proposal scope:** GooseClaw's own funding (Block Goose Grant) lives here; the four separate-initiative proposals were split into `aos-portfolio-proposals/` on 2026-07-17.
- **Knowledge-base completeness (2026-07-17):** 28 files — governance (2), canon (10), product (2: PRD, personas), reference (7: NIP matrix, competitive, ecosystem, risk, roadmap, glossary, moderation), operating prompts (2), preserved sources (3), and the GooseClaw proposal (2). This is the complete context set; the repo-seed files and portfolio proposals live outside this workspace by design. *(File numbers are stable identifiers, not reading order — read by the layer groupings above. `_archive_parallel_session/` holds superseded duplicates from a parallel drafting session, kept for reference; do not upload it.)*
