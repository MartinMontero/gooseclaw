# 06 — SDLC / DevOps / Deployment

## Repo structure (Rust workspace)
```
crates/
  gooseclaw            # core: consent loop, lifecycle, supervision, session state
  gooseclaw-channels   # adapters (Nostr first), inbound allowlist, chunking
  gooseclaw-skills     # Wasmtime sandbox, trust tiers, WIT world
  gooseclaw-nostr      # relay I/O, NIP-46 signing, event sanitization
  gooseclaw-knowledge  # Alfred MCP federation, libSQL cache, RRF
  gooseclaw-payments   # Lightning + Cashu, spend caps
  gooseclaw-safety     # ported IronClaw safety pipeline (attributed)
docs/  .github/workflows/  SBOM/
CLAUDE.md  AGENTS.md  constitution.md  specification.md  plan.md  tasks.md
THREAT-MODEL.md  SECURITY.md  NOTICE  ATTRIBUTION.md  CONTRIBUTING.md
```

## Branching / versioning
Trunk-based; **SemVer**; **conventional commits**; GPG-signed tags. Pin goose by release tag; treat the **ACP protocol version** as the stable contract to build against (goose internals churn fast — see `08_PIN_REGISTER.md`).

## CI/CD (GitHub Actions)
- **Vendor-exclusion gate** (the WCJBT pattern): a required job that **fails the build** if any resolved dependency is Meta/OpenAI/xAI-origin, proven by a deliberately-failing test fixture.
- **Pin every Action to a full commit SHA** (direct lesson from the Trivy tag-poisoning attack, CVE-2026-33634) and run a SHA-pin linter.
- **Supply chain:** **Syft** SBOM generation + **OSV-Scanner** + **Grype**; `cargo-deny` for license/advisory gates; lockfile audit. **Trivy is banned** (2026 TeamPCP compromise — see `08_PIN_REGISTER.md`).

## Testing pyramid
Unit → integration (ACP handshake, relay round-trip, bunker sign, MCP vault round-trip) → e2e (full Observe→Draft→Surface→Execute consent loop) → **agent-behavior eval harness** → **red-team / prompt-injection suite** (Unicode tag + zero-width smuggling, indirect injection via crafted Nostr events, lethal-trifecta exfil attempts, tool poisoning, approval spoof/replay). The security suites are release gates, not nice-to-haves.

## Deployment options (analyzed)
- **systemd on a VPS** — simplest supervision (`Restart=on-failure` + watchdog), but a network-exposed always-on agent is the OpenClaw risk profile; only acceptable bound to `127.0.0.1` behind a VPN.
- **rootless Podman** — recommended isolation for headless-server users; the secondary target.
- **Start9 / Umbrel packaging** — aligns with the sovereign user base; good secondary target.
- **Tauri sidecar (Alfred alignment) — RECOMMENDED PRIMARY.** Ship GooseClaw as a goose sidecar inside Alfred's Tauri 2 shell, reusing Alfred's **Windows Job Object kill guard**, born-redacted logging, and path-confinement plumbing. Distribution via **Zapstore** (desktop/CLI path via zapstore-cli).

## Observability (within zero-telemetry)
Local-only structured **audit log** — steal IronClaw's decision/dispatch/audit record model. No network egress. **W3C trace-context** correlation across the ACP boundary (Alfred already does SEP-414-style trace correlation). Metrics for token-spend and sat-spend against caps. "Zero telemetry" = no phone-home, **not** no logs.

## Secrets / keys
NIP-46 bunker on **separate hardware**; the always-on host holds only a disposable client key; no raw `nsec` on disk. Other secrets AES-256-GCM in the system keychain (IronClaw pattern), injected only at egress to allowlisted targets. **Critical operational item:** the Zapstore/updater signing key needs a documented backup/escrow procedure before any signed release (carried over from the Alfred program).

## Update / rollback
Pin goose by release tag; validate against a smoke-test recipe before promoting; keep the previous known-good goose binary for **instant rollback**. Watch the MCP 2026-07-28 RC (stateless redesign + Tasks-as-extension) as a potential breaking change.

## Resource governance
Global caps enforced in core: token/cost ceilings (goose supports `--budget` / `GOOSE_MAX_TURNS`), Wasmtime fuel + memory limits per skill, per-routine quotas, and restart-storm damping in the supervisor.
