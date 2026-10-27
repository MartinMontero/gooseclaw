# Phase 1 — ACP spike evidence — 2026-10-27

Spike binary: crates/gooseclaw/src/bin/acp_spike.rs (agent-client-protocol =2.0.0,
matching goose 1.48.0's own Cargo.lock; wire protocolVersion = 1).
Raw wire log: acp-messages.log (every stdin/stdout/stderr line, direction-tagged).
Session record: acp-spike.jsonl... see session file in this directory.

## Run result — EXIT 0 (full round-trip)

1. Spawn: goose acp, scrubbed env, GOOSE_MODE=approve, GOOSE_TELEMETRY_ENABLED=false (forced; overridable by nothing).
2. initialize -> response protocol_version = ProtocolVersion(1); agent = goose 1.48.0.
3. session/new -> SessionId("20261027_9").
4. session/prompt: "Use the shell tool to run exactly this command and nothing else: echo gooseclaw-acp-spike-ok".
5. goose requested permission (Approve mode honored — Auto would not have asked):
   session/request_permission for toolCall "shell · echo gooseclaw-acp-spike-ok",
   options: allow_always / allow_once / reject_once / reject_always.
6. Core gate (crates/gooseclaw/src/gate.rs): action submitted; simulated AUTHENTICATED
   human approval = ed25519 signature by the allowlisted test key over
   action_hash || nonce || expiry, single-use; gate.decide -> Approved (1 audit record).
7. Responded with option allow_once (never allow_always).
8. Prompt completed: stop_reason = EndTurn.
9. SPIKE-SUMMARY permission_requests=1 audit_events=1.

## Notes
- Provider contingency did NOT trigger: goose had a working provider path on this machine
  (desktop install present); nothing was installed.
- Zero permission requests would have been flagged as Approve-not-honored; we observed 1.
- Known limits (Phase 2+): allow_once chosen over allow_always by construction; the
  goose-internal mode could still be changed mid-session — GooseClaw never trusts the
  goose setting (canon 3.1 caveat carried).
