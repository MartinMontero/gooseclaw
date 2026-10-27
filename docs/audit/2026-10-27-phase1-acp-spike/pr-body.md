## Phase 1 — scaffold + ACP spike (DO NOT MERGE without operator review)

### Acceptance table

| Check | Result |
|---|---|
| cargo check --workspace | clean |
| cargo fmt --all -- --check | clean |
| cargo clippy --workspace --all-targets -- -D warnings | clean |
| cargo test --workspace | 22 unit tests + doctests, all pass |
| scripts/vendor_gate.py --self-test | OK (catches async-openai + zstd-sys fixture) |
| scripts/vendor_gate.py (real graph) | 179 packages, 0 denied, EXCEPTIONS empty |
| gate.ps1 (definition of done) | 13/13 subsystems PASS |
| ACP spike vs installed goose 1.48.0 | EXIT 0 — full core-gated round-trip |
| Commit/push boundary | this branch only; no merge |

### ACP wire log excerpt (full log: docs/audit/2026-10-27-phase1-acp-spike/acp-messages.log)

```
CLIENT->AGENT  initialize              {"protocolVersion":1, ...}
AGENT->CLIENT  result                  {"protocolVersion":1, agent: goose 1.48.0, ...}
CLIENT->AGENT  session/new             -> SessionId("20261027_9")
CLIENT->AGENT  session/prompt          "echo gooseclaw-acp-spike-ok"
AGENT->CLIENT  session/request_permission  "shell · echo gooseclaw-acp-spike-ok"
               options: allow_always / allow_once / reject_once / reject_always
   [core gate: ed25519-signed approval, allowlisted key, nonce+expiry bound to
    action hash, single-use -> Approved; 1 audit record]
CLIENT->AGENT  response: allow_once    (allow_always never selectable)
AGENT->CLIENT  prompt complete: EndTurn
```

Spawn env scrubbed: GOOSE_MODE=approve + GOOSE_TELEMETRY_ENABLED=false forced.
Approve mode honored: the permission request fired (Auto would not ask).

### Drift flags (flagged, not silently fixed)

1. **"Greenfield, no repo yet" was stale** — this repo exists with canon committed since
   the 2026-09-07 homecoming; CLAUDE.md's "canon deliberately not committed" line is
   outdated vs RECIPE.md (RECIPE.md is correct).
2. **goose 1.48.0 ACP advertises loadSession: true** — canon §3.1 says goose ACP has no
   resume/fork; re-verify before relying on either claim.
3. **rust-nostr/nostr now redirects to nostrdevkit/nostr** — canon [08R] was right;
   pin register updated (nostr 0.45.5 / nostr-sdk 0.45.4, ALPHA).

Also recorded in docs/pin-register/08_PIN_REGISTER-2026-10-27.md: ACP crate pin
superseded to =2.0.0 (matches goose 1.48.0 Cargo.lock; kickoff's 1.0.0 stale);
MCP 2026-07-28 is now a FULL release (was RC); wasmtime upstream v50.0.0;
IronClaw study pin b011eb39 (ironclaw-v1.4.1) — ~74 lib crates, grouped dirs,
ironclaw_wasm_limiter EXISTS (kickoff said it didn't); "goose 2.0 behind ACP"
remains UNVERIFIED.

### Stop-line confirmation

Session stopped at Phase 1 acceptance: no Alfred federation (Phase 2), no rust-nostr /
libSQL / wasmtime / payments wiring, no IronClaw vendoring (reference/ clone is
study-only and gitignored), no autonomy (Phase 6), **no merge**. Amendments 1-3 recorded
in canon Appendix C with operator approval 2026-10-27.