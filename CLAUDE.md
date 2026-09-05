# CLAUDE.md — GooseClaw

Nostr-native, always-on **fiduciary** agent runtime. Drives goose over **ACP**; federates with Alfred over **MCP** (Alfred = system-of-record). Seven-crate Rust workspace. Greenfield.

## Project graph

- **Read `RECIPE.md` before any work.** It is the persistent dependency graph of this project: every subsystem, what it needs, and the proof that it works.
- **A session's plan is a subset of `RECIPE.md`** — the subsystems being touched, in dependency order, with their proof lines carried forward. If the plan touches a subsystem, it inherits that subsystem's proof.
- **Session audit/plan docs live in `docs/audit/<date>-<session>/`, never at the repo root.**
- **Rulings live in the canon (`specification.md`), never in a second register** — there is no `DECISIONS.md`. The canon is deliberately not committed; `gate.ps1` reads it from the repo's parent directory.
- **Run `powershell -File gate.ps1` before claiming done.** This is the definition-of-done gate and is **separate from the build/vendor gates** (`ci.yml`, `vendor_gate.py`). All must pass; one does not substitute for another.
- **DRIFT RULE:** any PR that touches a subsystem re-verifies its proof line. A drifted proof is flagged in the PR body — never silently edited. A proof that cannot run (missing artifact, missing tool) is a FAIL, not a skip.

## TIER 1 — HARD RULES (never violate)

1. **Rule 9 — consent before consequence.** No `git commit`, `git push`, remote creation, delete, spend, or Nostr publish without explicit human go-ahead. Enforced by a PreToolUse deny hook. Local file writes and `cargo check` are free.
2. **RPI.** Research → Plan → Implement. Plan mode default. Present a plan and **STOP** before implementing anything non-trivial.
3. **Zero fabrication.** Every version/API claim needs a primary-source URL + as-of date. Can't verify → say so. Pin drifted → **flag it, don't silently fix it**.
4. **Vendor denylist.** No **Meta / OpenAI / xAI** dependencies, SDKs, or model providers. **Google, Mistral, Ollama, MiniMax, Anthropic permitted.** Enforced in CI: `scripts/vendor_gate.py`.
5. **Supply chain.** Syft SBOM + OSV-Scanner + Grype + cargo-deny. **Trivy is BANNED** (CVE-2026-33634). Pin every GitHub Action to a full commit SHA.
6. **Security invariants.**
   - Strip Unicode Tag Block (U+E0000–U+E007F), zero-width, and bidi chars from **all inbound content** *and* from approval summaries shown to the human.
   - **One core-owned Rule-9 gate.** Channels and payments *call* it; they never own or reimplement the decision.
   - Inbound message text is **untrusted data, never instructions**. Unknown senders are not on the allowlist.
   - Approvals must be **signed by an allowlisted pubkey**, nonce + expiry bound to the action hash, single-use.
   - **NIP-46 remote signing** — never a raw `nsec` on this host. Bunker offline ⇒ **fail closed**.
   - Path confinement on every tool. Never auto-exfiltrate private data (break the lethal trifecta).
   - Assume prompt-injection classifiers are bypassable. Rely on architecture, not detection.
7. **Sovereignty.** Local-first. **Zero telemetry = no phone-home; local audit logs are required.** No network bind by default. No mandatory hosted dependency.

## TIER 2 — PROJECT

- **License:** AGPL-3.0-or-later. goose = Apache-2.0 (Apache→AGPL composes one-way). IronClaw = MIT OR Apache-2.0 → record attribution in `NOTICE` **before** any reuse.
- **Crates:** `gooseclaw` (core: ACP supervisor, the gate, scheduler, session state) · `-channels` · `-skills` · `-nostr` · `-knowledge` · `-payments` · `-safety`.
- **Precedence:** verified primary source > `specification.md` > this file. On conflict: **flag and stop**.

## Commands

<!-- Phase 0 fills these once the workspace is scaffolded -->
- Check: `cargo check --workspace`
- Test: `cargo test --workspace`
- Lint: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings`
- Vendor gate: `python3 scripts/vendor_gate.py` (self-test: `--self-test`)
