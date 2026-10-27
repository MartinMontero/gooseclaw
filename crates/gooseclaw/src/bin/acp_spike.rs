//! gooseclaw-acp-spike — Phase 1 ACP spike.
//!
//! Spawns the installed goose (1.48.0) as an ACP agent over stdio with a
//! scrubbed environment (GOOSE_MODE=approve, GOOSE_TELEMETRY_ENABLED=false),
//! completes the handshake at protocolVersion = 1 (ProtocolVersion::V1,
//! agent-client-protocol crate =2.0.0 — matches goose's own Cargo.lock),
//! and demonstrates ONE core-gated tool call end to end:
//!
//!   prompt -> tool call -> session/request_permission (ActionRequired)
//!     -> core Rule-9 gate -> simulated AUTHENTICATED human approve
//!     (ed25519, allowlisted pubkey, nonce+expiry bound to action hash,
//!      single-use) -> permission response -> resume.
//!
//! Every raw ACP line (stdin/stdout/stderr) is logged verbatim to
//! docs/audit/2026-10-27-phase1-acp-spike/acp-messages.log .
//!
//! Exit codes: 0 = full round-trip; 2 = handshake OK but prompt leg failed
//! (e.g. no model provider configured — per session rule, we stop and report
//! rather than install anything); 1 = handshake failed.

use agent_client_protocol::schema::ProtocolVersion;
use agent_client_protocol::schema::v1::{
    ContentBlock, InitializeRequest, NewSessionRequest, PromptRequest, RequestPermissionOutcome,
    RequestPermissionRequest, RequestPermissionResponse, SelectedPermissionOutcome,
    SessionNotification, TextContent,
};
use agent_client_protocol::{AcpAgent, AcpAgentConfig, Agent, Client, ConnectionTo};
use ed25519_dalek::{Signer, SigningKey};
use gooseclaw::gate::{ActionRequest, Approval, Decision, Rule9Gate};
use gooseclaw::session::{JsonlSessionStore, SessionEvent, SessionEventKind};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const AUDIT_DIR: &str = "docs/audit/2026-10-27-phase1-acp-spike";
const LOG_FILE: &str = "docs/audit/2026-10-27-phase1-acp-spike/acp-messages.log";

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

struct RawLog(Mutex<File>);
impl RawLog {
    fn line(&self, dir: &str, text: &str) {
        let mut f = self.0.lock().unwrap();
        let _ = writeln!(f, "[{}] {}", dir, text);
    }
}

struct SpikeState {
    gate: Mutex<Rule9Gate>,
    signer: SigningKey,
    permission_requests: Mutex<u32>,
    sessions: JsonlSessionStore,
    session_id: Mutex<String>,
}

impl SpikeState {
    fn record(&self, kind: SessionEventKind, detail: impl Into<String>) {
        let sid = self.session_id.lock().unwrap().clone();
        let _ = self.sessions.append(
            &sid,
            &SessionEvent {
                at_unix: now_unix(),
                kind,
                detail: detail.into(),
            },
        );
    }
}

/// The simulated-but-authenticated human: signs the ACTION HASH with an
/// allowlisted test key. Same gate API the NIP-46 path will use; no network.
fn human_sign(state: &SpikeState, pending: &gooseclaw::gate::PendingAction) -> Approval {
    let nonce: [u8; 32] = rand_free_nonce();
    let expiry = now_unix() + 120;
    let msg = Approval::message(&pending.hash(), &nonce, expiry);
    let sig = state.signer.sign(&msg).to_bytes();
    Approval {
        pubkey: state.signer.verifying_key().to_bytes(),
        nonce,
        expires_at_unix: expiry,
        signature: sig,
    }
}

/// Deterministic nonce from time+counter — adequate for a single-process
/// spike; production nonces come from the channel's CSPRNG (Phase 2+).
fn rand_free_nonce() -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut h = Sha256::new();
    h.update(nanos.to_le_bytes());
    h.update(std::process::id().to_le_bytes());
    h.finalize().into()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::fs::create_dir_all(AUDIT_DIR)?;
    let log = Arc::new(RawLog(Mutex::new(
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(LOG_FILE)?,
    )));

    // -- The gate, with one allowlisted test signer -------------------------
    let signer = SigningKey::from_bytes(&[0x42u8; 32]); // deterministic TEST key
    let signer_pk = signer.verifying_key().to_bytes();
    let state = Arc::new(SpikeState {
        gate: Mutex::new(Rule9Gate::new([signer_pk])),
        signer,
        permission_requests: Mutex::new(0),
        sessions: JsonlSessionStore::new(AUDIT_DIR)?,
        session_id: Mutex::new("acp-spike".to_string()),
    });

    // -- Spawn config: scrubbed env, Approve forced, no telemetry -----------
    for (k, v) in gooseclaw::AcpSpawnConfig::forced_env() {
        log.line("SPAWN-ENV", &format!("{}={}", k, v));
    }
    let mut cfg = AcpAgentConfig::new(PathBuf::from("goose"));
    cfg = cfg.arg("acp");
    for (k, v) in gooseclaw::AcpSpawnConfig::forced_env() {
        cfg = cfg.env(k, v);
    }
    cfg = cfg.env("NO_COLOR", "1");

    let log2 = log.clone();
    let agent = AcpAgent::new(cfg).with_debug(move |line: &str, dir| {
        let d = match dir {
            agent_client_protocol::LineDirection::Stdin => "CLIENT->AGENT",
            agent_client_protocol::LineDirection::Stdout => "AGENT->CLIENT",
            agent_client_protocol::LineDirection::Stderr => "AGENT-STDERR",
        };
        log2.line(d, line);
    });

    eprintln!("[spike] spawning: goose acp (GOOSE_MODE=approve, telemetry off)");

    let st_notif = state.clone();
    let st_perm = state.clone();
    let st_conn = state.clone();

    let result = Client
        .builder()
        .on_receive_notification(
            async move |notification: SessionNotification, _cx| {
                st_notif.record(
                    SessionEventKind::ToolCallRequested,
                    format!("session/update: {:?}", notification.update),
                );
                Ok(())
            },
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_request(
            async move |request: RequestPermissionRequest, responder, _connection| {
                let st = st_perm.clone();
                    *st.permission_requests.lock().unwrap() += 1;
                    let raw = serde_json::to_string_pretty(&request)
                        .unwrap_or_else(|_| "<unserializable>".into());
                    st.record(
                        SessionEventKind::ToolCallRequested,
                        format!("session/request_permission RAW: {}", raw),
                    );
                    eprintln!("[spike] PERMISSION REQUEST (ActionRequired):");
                    eprintln!("{}", raw);

                    // Build the gated action from the request.
                    let action = ActionRequest {
                        actor: "acp:tool".into(),
                        summary: format!("goose requests tool call: {:?}", request.tool_call),
                        payload: raw.into_bytes(),
                    };
                    let decision = {
                        let mut gate = st.gate.lock().unwrap();
                        let pending = gate.submit(action);
                        // Simulated AUTHENTICATED human: signed, nonce+expiry
                        // bound to the action hash, single-use (Amendment 3).
                        let approval = human_sign(&st, &pending);
                        gate.decide(&pending, &approval)
                    };
                    match decision {
                        Ok(Decision::Approved) => {
                            st.record(SessionEventKind::GateDecision, "APPROVED (signed, single-use)");
                            eprintln!("[spike] gate: APPROVED -> allow_once");
                            // Prefer an allow-once option; fall back to first.
                            let opt = request
                                .options
                                .iter()
                                .find(|o| format!("{:?}", o.kind).contains("AllowOnce"))
                                .or_else(|| request.options.first());
                            match opt {
                                Some(o) => {
                                    let _ = responder.respond(RequestPermissionResponse::new(
                                        RequestPermissionOutcome::Selected(
                                            SelectedPermissionOutcome::new(o.option_id.clone()),
                                        ),
                                    ));
                                }
                                None => {
                                    let _ = responder.respond(RequestPermissionResponse::new(
                                        RequestPermissionOutcome::Cancelled,
                                    ));
                                }
                            }
                        }
                        other => {
                            st.record(
                                SessionEventKind::GateDecision,
                                format!("DENIED/ERROR: {:?}", other),
                            );
                            eprintln!("[spike] gate: {:?} -> cancel", other);
                            let _ = responder.respond(RequestPermissionResponse::new(
                                RequestPermissionOutcome::Cancelled,
                            ));
                        }
                    }
                    Ok(())
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_with(agent, move |connection: ConnectionTo<Agent>| {
            let st = st_conn.clone();
            async move {
                eprintln!("[spike] -> initialize (protocolVersion: 1)");
                let init = connection
                    .send_request(InitializeRequest::new(ProtocolVersion::V1))
                    .block_task()
                    .await?;
                eprintln!(
                    "[spike] <- initialized: protocol_version={:?} agent={:?}",
                    init.protocol_version, init.agent_info
                );

                let session = connection
                    .send_request(NewSessionRequest::new(
                        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                    ))
                    .block_task()
                    .await?;
                *st.session_id.lock().unwrap() = format!("{:?}", session.session_id);
                st.record(SessionEventKind::SessionCreated, format!("{:?}", session.session_id));
                eprintln!("[spike] <- session/new: {:?}", session.session_id);

                let prompt = "Use the shell tool to run exactly this command and nothing else: \
                              echo gooseclaw-acp-spike-ok";
                st.record(SessionEventKind::PromptSent, prompt);
                eprintln!("[spike] -> session/prompt (expecting a gated tool call)");
                let prompt_fut = connection.send_request(PromptRequest::new(
                    session.session_id.clone(),
                    vec![ContentBlock::Text(TextContent::new(prompt.to_string()))],
                ));
                let resp = tokio::time::timeout(
                    Duration::from_secs(180),
                    prompt_fut.block_task(),
                )
                .await;
                match resp {
                    Ok(Ok(pr)) => {
                        st.record(SessionEventKind::TurnComplete, format!("{:?}", pr.stop_reason));
                        eprintln!("[spike] <- prompt complete: {:?}", pr.stop_reason);
                    }
                    Ok(Err(e)) => {
                        st.record(SessionEventKind::Error, format!("prompt error: {e}"));
                        eprintln!("[spike] PROMPT LEG FAILED: {e}");
                        eprintln!("[spike] NOTE: no provider configured? Session rule: stop, do not install.");
                        std::process::exit(2);
                    }
                    Err(_) => {
                        st.record(SessionEventKind::Error, "prompt timeout");
                        eprintln!("[spike] PROMPT LEG TIMEOUT (180s)");
                        std::process::exit(2);
                    }
                }
                Ok(())
            }
        })
        .await;

    match result {
        Ok(()) => {
            let n = *state.permission_requests.lock().unwrap();
            println!(
                "SPIKE-SUMMARY permission_requests={} audit_events={}",
                n,
                state.gate.lock().unwrap().audit_log().len()
            );
            if n == 0 {
                println!(
                    "SPIKE-SUMMARY WARNING: prompt completed with zero permission requests — Approve mode may not have been honored"
                );
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("[spike] HANDSHAKE/TRANSPORT FAILED: {e}");
            std::process::exit(1);
        }
    }
}
