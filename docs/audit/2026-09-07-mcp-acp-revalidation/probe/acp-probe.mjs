// acp-probe.mjs — minimal ACP client (wire protocolVersion = 1) driving `goose acp`.
// Forces Approve mode via GOOSE_MODE=approve; answers session/request_permission
// with allow (evidence of the approval gate). Attaches the strict 2026-07-28
// MCP vault server as a stdio MCP server in session/new.
// Logs every ACP wire line verbatim to wire-acp.ndjson.

import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import readline from "node:readline";
import url from "node:url";

const HERE = path.dirname(url.fileURLToPath(import.meta.url));
const WIRE = path.join(HERE, "wire-acp.ndjson");
const GOOSE = process.env.GOOSE_BIN || "goose";

const prompts = [
  // 1: canon call shape — vault-style READ
  "Use the vault_read tool from the alfred-vault MCP server to read path \"day1.md\". Report back the version number and the content exactly. Do not use any other tools.",
  // 2: canon call shape — vault-style WRITE with sole-writer optimistic concurrency
  null, // filled after read, using observed version
  // 3: conflict probe — stale expectedVersion must lose
  null,
];

const child = spawn(GOOSE, ["acp"], {
  env: {
    ...process.env,
    GOOSE_MODE: "approve",
    GOOSE_TELEMETRY_ENABLED: "false",
  },
  stdio: ["pipe", "pipe", "inherit"],
});

const wlog = (dir, obj) =>
  fs.appendFileSync(WIRE, JSON.stringify({ t: new Date().toISOString(), dir, line: obj }) + "\n");

let nextId = 1;
const pending = new Map();
function request(method, params) {
  const id = nextId++;
  const msg = { jsonrpc: "2.0", id, method, params };
  wlog("client->agent", msg);
  child.stdin.write(JSON.stringify(msg) + "\n");
  return new Promise((resolve, reject) => pending.set(id, { resolve, reject, method }));
}
function respond(id, result) {
  const msg = { jsonrpc: "2.0", id, result };
  wlog("client->agent", msg);
  child.stdin.write(JSON.stringify(msg) + "\n");
}

const rl = readline.createInterface({ input: child.stdout });
let agentText = "";
rl.on("line", (line) => {
  if (!line.trim()) return;
  let msg;
  try { msg = JSON.parse(line); } catch { console.error("UNPARSEABLE:", line); return; }
  wlog("agent->client", msg);

  if (msg.id !== undefined && msg.method) {
    // Agent -> client request
    if (msg.method === "session/request_permission") {
      const opts = msg.params?.options ?? [];
      const allow = opts.find((o) => o.kind === "allow_once") ?? opts.find((o) => o.kind === "allow_always") ?? opts[0];
      console.log(`[PERMISSION] tool=${msg.params?.toolCall?.fields?.title ?? msg.params?.toolCall?.toolCallId} -> approving with ${allow?.optionId ?? "none"}`);
      if (allow) respond(msg.id, { outcome: { outcome: "selected", optionId: allow.optionId } });
      else respond(msg.id, { outcome: { outcome: "cancelled" } });
    } else {
      respond(msg.id, { error: undefined }); // not expected; we advertised no fs/terminal caps
    }
    return;
  }
  if (msg.id !== undefined) {
    // Response to our request
    const p = pending.get(msg.id);
    if (p) {
      pending.delete(msg.id);
      if (msg.error) p.reject(new Error(`${p.method} failed: ${JSON.stringify(msg.error)}`));
      else p.resolve(msg.result);
    }
    return;
  }
  // Notification
  if (msg.method === "session/update") {
    const u = msg.params?.update;
    if (u?.sessionUpdate === "agent_message_chunk" && u.content?.type === "text") {
      agentText += u.content.text;
      process.stdout.write(u.content.text);
    } else if (u?.sessionUpdate === "tool_call") {
      console.log(`\n[TOOL_CALL] ${u.title} (${u.toolCallId}) kind=${u.kind} status=${u.status}`);
      if (u.rawInput) console.log(`  rawInput: ${JSON.stringify(u.rawInput)}`);
    } else if (u?.sessionUpdate === "tool_call_update") {
      const out = u.fields?.rawOutput ?? u.rawOutput;
      console.log(`[TOOL_UPDATE] ${u.toolCallId} status=${u.fields?.status ?? u.status}${out ? " rawOutput=" + JSON.stringify(out).slice(0, 400) : ""}`);
    }
  }
});

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function main() {
  const init = await request("initialize", {
    protocolVersion: 1,
    clientCapabilities: { fs: { readTextFile: false, writeTextFile: false }, terminal: false },
    clientInfo: { name: "gooseclaw-acp-probe", version: "0.1.0" },
  });
  console.log("== initialize ok, agent:", JSON.stringify(init?.agentInfo ?? {}), "protocolVersion:", init?.protocolVersion);

  const sess = await request("session/new", {
    cwd: HERE,
    mcpServers: [
      {
        type: "stdio",
        name: "alfred-vault",
        command: process.execPath,
        args: [path.join(HERE, "vault-server.mjs")],
        env: [],
      },
    ],
  });
  const sessionId = sess.sessionId;
  console.log("== session/new ok:", sessionId);

  // Prompt 1: read
  agentText = "";
  await request("session/prompt", { sessionId, prompt: [{ type: "text", text: prompts[0] }] });
  console.log("\n== P1 (vault_read) complete");
  const m = agentText.match(/version[^\d]*(\d+)/i);
  const observedVersion = m ? parseInt(m[1], 10) : 1;
  console.log(`== observed version from agent: ${observedVersion}`);

  // Prompt 2: write with write-if-unchanged
  prompts[1] = `Use the vault_write tool from the alfred-vault MCP server to write path "day1.md" with text "canon probe note v2 (written by goose over ACP)\\n" and expectedVersion ${observedVersion}. Report the tool result exactly. Do not use any other tools.`;
  agentText = "";
  await request("session/prompt", { sessionId, prompt: [{ type: "text", text: prompts[1] }] });
  console.log("\n== P2 (vault_write, expectedVersion current) complete");

  // Prompt 3: stale write must CONFLICT (human-edit-wins semantics)
  prompts[2] = `Use the vault_write tool from the alfred-vault MCP server to write path "day1.md" with text "stale overwrite attempt\\n" and expectedVersion ${observedVersion}. Report the tool result exactly, including any CONFLICT text. Do not retry and do not use any other tools.`;
  agentText = "";
  await request("session/prompt", { sessionId, prompt: [{ type: "text", text: prompts[2] }] });
  console.log("\n== P3 (vault_write, stale expectedVersion) complete");

  child.stdin.end();
  await sleep(500);
  child.kill();
  process.exit(0);
}

main().catch((e) => {
  console.error("PROBE FAILED:", e.message);
  child.kill();
  process.exit(2);
});

setTimeout(() => { console.error("PROBE TIMEOUT"); child.kill(); process.exit(3); }, 420000);
