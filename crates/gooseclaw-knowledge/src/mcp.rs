//! Minimal MCP client for the Alfred vault server.
//!
//! Era handling per spec 2026-07-28 (basic/transports/stdio, verified
//! 2026-10-29): probe with server/discover FIRST; a DiscoverResult or a
//! recognized modern error (-32022) marks a dual-era server; ANY other error
//! or a timeout marks a legacy server -> initialize handshake. The fallback is
//! deliberately NOT keyed to one specific error code.
//!
//! Alfred today serves the legacy era only (2025-11-25 via initialize), so
//! after a successful probe this client always completes the legacy
//! handshake. Framing: newline-delimited JSON-RPC 2.0 over stdio.

#![forbid(unsafe_code)]

use serde_json::{Value, json};
use std::fmt;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

pub const LEGACY_VERSION: &str = "2025-11-25";
pub const MODERN_META_KEY: &str = "io.modelcontextprotocol/protocolVersion";
pub const MODERN_PROBE_VERSION: &str = "2026-07-28";
pub const UNSUPPORTED_PROTOCOL_VERSION: i64 = -32022;

const PROBE_TIMEOUT: Duration = Duration::from_secs(5);
const CALL_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug)]
pub enum McpError {
    Transport(String),
    Protocol(String),
    Timeout,
    /// The server returned a JSON-RPC error object (code + message preserved).
    Remote {
        code: i64,
        message: String,
    },
    /// The server is dual-era/modern but does not serve the legacy version we need.
    Unsupported {
        requested: String,
        supported: Vec<String>,
    },
}

impl fmt::Display for McpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            McpError::Transport(e) => write!(f, "transport: {e}"),
            McpError::Protocol(e) => write!(f, "protocol: {e}"),
            McpError::Timeout => write!(f, "timeout"),
            McpError::Remote { code, message } => write!(f, "remote error {code}: {message}"),
            McpError::Unsupported {
                requested,
                supported,
            } => {
                write!(
                    f,
                    "server does not serve {requested} (supported: {supported:?})"
                )
            }
        }
    }
}
impl std::error::Error for McpError {}

/// How the server identified itself during the era probe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerEra {
    /// Answered server/discover (or -32022): dual-era/modern-aware.
    DualEraAware { supported: Vec<String> },
    /// Anything else: legacy, initialize handshake required.
    Legacy,
}

/// A connected MCP session (post-handshake), generic over the byte streams so
/// tests run in-process (tokio::io::duplex) and production uses a supervised
/// subprocess (gooseclaw::supervise).
pub struct McpConnection<R, W> {
    reader: BufReader<R>,
    writer: W,
    next_id: u64,
    pub era: ServerEra,
    /// The protocol version negotiated with the server.
    pub negotiated_version: String,
}

impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> McpConnection<R, W> {
    /// Era probe + handshake. Returns a ready session or an error.
    pub async fn connect(reader: R, writer: W) -> Result<Self, McpError> {
        let mut conn = Self {
            reader: BufReader::new(reader),
            writer,
            next_id: 0,
            era: ServerEra::Legacy,
            negotiated_version: String::new(),
        };

        // 1. Era probe: server/discover with a modern _meta.
        let probe = json!({
            "_meta": {
                MODERN_META_KEY: MODERN_PROBE_VERSION,
                "io.modelcontextprotocol/clientInfo": { "name": "gooseclaw-knowledge", "version": env!("CARGO_PKG_VERSION") },
                "io.modelcontextprotocol/clientCapabilities": {},
            }
        });
        match conn
            .roundtrip("server/discover", probe, PROBE_TIMEOUT)
            .await
        {
            Ok(result) => {
                let supported = result
                    .get("supportedVersions")
                    .and_then(Value::as_array)
                    .map(|a| {
                        a.iter()
                            .filter_map(Value::as_str)
                            .map(String::from)
                            .collect()
                    })
                    .unwrap_or_default();
                conn.era = ServerEra::DualEraAware { supported };
            }
            Err(McpError::Remote { code, .. }) if code == UNSUPPORTED_PROTOCOL_VERSION => {
                // Recognized modern error: dual-era server, unserved version.
                conn.era = ServerEra::DualEraAware { supported: vec![] };
            }
            Err(_) => {
                // Any other error or timeout: legacy server (spec rule).
                conn.era = ServerEra::Legacy;
            }
        }

        // 2. Legacy handshake (the only era Alfred serves today).
        let init = conn
            .roundtrip(
                "initialize",
                json!({
                    "protocolVersion": LEGACY_VERSION,
                    "capabilities": {},
                    "clientInfo": { "name": "gooseclaw-knowledge", "version": env!("CARGO_PKG_VERSION") },
                }),
                CALL_TIMEOUT,
            )
            .await?;
        let negotiated = init
            .get("protocolVersion")
            .and_then(Value::as_str)
            .ok_or_else(|| McpError::Protocol("initialize: no protocolVersion in result".into()))?;
        if negotiated != LEGACY_VERSION {
            return Err(McpError::Unsupported {
                requested: LEGACY_VERSION.into(),
                supported: vec![negotiated.to_string()],
            });
        }
        conn.negotiated_version = negotiated.to_string();
        conn.notify("notifications/initialized", json!({})).await?;
        Ok(conn)
    }

    async fn send_value(&mut self, v: Value) -> Result<(), McpError> {
        let mut line = serde_json::to_string(&v).map_err(|e| McpError::Protocol(e.to_string()))?;
        line.push('\n');
        self.writer
            .write_all(line.as_bytes())
            .await
            .map_err(|e| McpError::Transport(e.to_string()))
    }

    async fn notify(&mut self, method: &str, params: Value) -> Result<(), McpError> {
        self.send_value(json!({ "jsonrpc": "2.0", "method": method, "params": params }))
            .await
    }

    /// One request/response round-trip; lines with other ids (notifications)
    /// are skipped. Times out per call.
    pub async fn roundtrip(
        &mut self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value, McpError> {
        self.next_id += 1;
        let id = self.next_id;
        self.send_value(json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }))
            .await?;
        let fut = async {
            loop {
                let mut line = String::new();
                let n = self
                    .reader
                    .read_line(&mut line)
                    .await
                    .map_err(|e| McpError::Transport(e.to_string()))?;
                if n == 0 {
                    return Err(McpError::Transport("eof (server closed stdio)".into()));
                }
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let msg: Value =
                    serde_json::from_str(trimmed).map_err(|e| McpError::Protocol(e.to_string()))?;
                if msg.get("id").and_then(Value::as_u64) != Some(id) {
                    continue; // notification or out-of-order; skip
                }
                if let Some(err) = msg.get("error") {
                    return Err(McpError::Remote {
                        code: err.get("code").and_then(Value::as_i64).unwrap_or(0),
                        message: err
                            .get("message")
                            .and_then(Value::as_str)
                            .unwrap_or("error")
                            .to_string(),
                    });
                }
                return Ok(msg.get("result").cloned().unwrap_or(Value::Null));
            }
        };
        tokio::time::timeout(timeout, fut)
            .await
            .map_err(|_| McpError::Timeout)?
    }

    /// Call a tool; returns (text_body, is_error).
    pub async fn call_tool(
        &mut self,
        name: &str,
        arguments: Value,
    ) -> Result<(String, bool), McpError> {
        let result = self
            .roundtrip(
                "tools/call",
                json!({ "name": name, "arguments": arguments }),
                CALL_TIMEOUT,
            )
            .await?;
        let is_error = result
            .get("isError")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let text = result
            .get("content")
            .and_then(Value::as_array)
            .and_then(|a| a.first())
            .and_then(|c| c.get("text"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        Ok((text, is_error))
    }
}
