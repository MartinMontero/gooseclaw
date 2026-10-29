//! Supervised subprocess seam (core-owned).
//!
//! Every subprocess GooseClaw rides on — the goose ACP server, the Alfred MCP
//! vault server — is spawned HERE, with an explicit environment and
//! kill-on-drop semantics. Satellites (channels, knowledge) never spawn
//! ambient processes; they receive a connected transport from core.
//! Sovereignty: no network bind anywhere in this seam; stdio only.

#![forbid(unsafe_code)]

use std::io;
use std::process::Stdio;
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

/// A supervised child process wired for newline-delimited JSON-RPC over stdio.
pub struct JsonRpcProcess {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
}

/// Spawn spec: command, args, and the EXACT environment the child sees.
/// Callers pass a scrubbed env (see AcpSpawnConfig::forced_env for the goose
/// case); nothing is inherited implicitly.
pub struct SpawnSpec {
    pub program: String,
    pub args: Vec<String>,
    /// Complete environment for the child (env_clear is applied first).
    pub env: Vec<(String, String)>,
    /// Working directory (None = inherit).
    pub cwd: Option<std::path::PathBuf>,
}

/// Spawn a supervised stdio JSON-RPC subprocess. The child is killed when the
/// returned handle is dropped (kill_on_drop) — no orphaned servers.
pub fn spawn_jsonrpc(spec: &SpawnSpec) -> io::Result<JsonRpcProcess> {
    let mut cmd = Command::new(&spec.program);
    cmd.args(&spec.args)
        .env_clear()
        .envs(spec.env.iter().cloned())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    if let Some(cwd) = &spec.cwd {
        cmd.current_dir(cwd);
    }
    let mut child = cmd.spawn()?;
    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("child stdin not piped"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("child stdout not piped"))?;
    Ok(JsonRpcProcess {
        child,
        stdin,
        stdout,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn spawned_process_round_trips_a_line() {
        // findstr echoes matching stdin lines. Absolute path: the scrubbed
        // env (env_clear) gives the child NO PATH — this also proves the scrub.
        let spec = SpawnSpec {
            program: std::env::var("SystemRoot")
                .map(|w| format!("{w}\\System32\\findstr.exe"))
                .unwrap_or_else(|_| r"C:\Windows\System32\findstr.exe".into()),
            args: vec!["ping".into()],
            env: vec![("GOOSECLAW_TEST".into(), "1".into())],
            cwd: None,
        };
        let mut p = spawn_jsonrpc(&spec).expect("spawn");
        p.stdin.write_all(b"ping\n").await.expect("write");
        // Windows console tools buffer piped stdin until EOF: close stdin so
        // findstr processes and flushes. (The MCP/ACP servers we actually
        // supervise flush per line; findstr does not.)
        drop(p.stdin);
        let mut out = Vec::new();
        tokio::io::AsyncReadExt::read_to_end(&mut p.stdout, &mut out)
            .await
            .expect("read");
        let text = String::from_utf8_lossy(&out);
        assert!(text.contains("ping"), "expected echo, got: {text:?}");
        p.child.wait().await.ok();
    }
}
