// vault-server.mjs — STRICT MCP 2026-07-28 ("modern era") server, hand-rolled to spec.
// No initialize handshake. Per-request _meta. server/discover mandatory.
// Vault semantics per GooseClaw canon §3.2: reads free; writes single-writer
// with optimistic concurrency (read-version / write-if-unchanged).
// Logs every wire line verbatim to wire-mcp.ndjson for the probe transcript.

import fs from "node:fs";
import path from "node:path";
import readline from "node:readline";
import url from "node:url";

const HERE = path.dirname(url.fileURLToPath(import.meta.url));
const VAULT = path.join(HERE, "vault");
const WIRE = path.join(HERE, "wire-mcp.ndjson");
const PROTO = "2026-07-28";
const META = "io.modelcontextprotocol";

fs.mkdirSync(VAULT, { recursive: true });
// Seed: a vault note at version 1 (sole-writer state).
const seedPath = path.join(VAULT, "day1.md");
if (!fs.existsSync(seedPath)) fs.writeFileSync(seedPath, "canon probe note v1\n");
const metaFile = path.join(VAULT, ".versions.json");
let versions = fs.existsSync(metaFile)
  ? JSON.parse(fs.readFileSync(metaFile, "utf8"))
  : { "day1.md": 1 };
const saveVersions = () => fs.writeFileSync(metaFile, JSON.stringify(versions));

const wlog = (dir, line) =>
  fs.appendFileSync(WIRE, JSON.stringify({ t: new Date().toISOString(), dir, line: JSON.parse(line) }) + "\n");

let stripResultType = false; // legacy replies omit resultType
const send = (obj) => {
  if (stripResultType && obj.result) delete obj.result.resultType;
  const line = JSON.stringify(obj);
  wlog("server->client", line);
  process.stdout.write(line + "\n");
};

const err = (id, code, message, data) => send({ jsonrpc: "2.0", id, error: { code, message, ...(data ? { data } : {}) } });
const ok = (id, result) => send({ jsonrpc: "2.0", id, result });

function checkMeta(id, params) {
  const v = params?._meta?.[`${META}/protocolVersion`];
  if (!v) {
    err(id, -32022, "UnsupportedProtocolVersion: request missing _meta protocolVersion", { supported: [PROTO] });
    return false;
  }
  if (v !== PROTO) {
    err(id, -32022, `UnsupportedProtocolVersion: '${v}'`, { supported: [PROTO] });
    return false;
  }
  return true;
}

// Legacy (2025-11-25) tools/call: same semantics, legacy result shape (no resultType).
function callToolLegacy(id, params) {
  stripResultType = true;
  try { callTool(id, params); } finally { stripResultType = false; }
}

const TOOLS = [
  {
    name: "vault_read",
    description: "Read a vault note. Returns content and its current version (sole-writer optimistic concurrency token).",
    inputSchema: { type: "object", properties: { path: { type: "string" } }, required: ["path"] },
  },
  {
    name: "vault_write",
    description: "Write a vault note IF UNCHANGED since expectedVersion (single-writer optimistic concurrency; a human edit wins conflicts).",
    inputSchema: {
      type: "object",
      properties: {
        path: { type: "string" },
        text: { type: "string" },
        expectedVersion: { type: "integer" },
      },
      required: ["path", "text", "expectedVersion"],
    },
  },
];

function callTool(id, params) {
  const { name, arguments: args = {} } = params ?? {};
  const p = String(args.path ?? "");
  if (p.includes("..") || path.isAbsolute(p)) {
    return ok(id, { resultType: "complete", isError: true, content: [{ type: "text", text: "PATH CONFINEMENT VIOLATION" }] });
  }
  const file = path.join(VAULT, p);
  if (name === "vault_read") {
    if (!fs.existsSync(file))
      return ok(id, { resultType: "complete", isError: true, content: [{ type: "text", text: `NOT FOUND: ${p}` }] });
    const text = fs.readFileSync(file, "utf8");
    const version = versions[p] ?? 1;
    return ok(id, {
      resultType: "complete",
      content: [{ type: "text", text: `path=${p}\nversion=${version}\n---\n${text}` }],
      structuredContent: { path: p, version, text },
    });
  }
  if (name === "vault_write") {
    const current = versions[p] ?? (fs.existsSync(file) ? 1 : 0);
    if (args.expectedVersion !== current) {
      // Optimistic concurrency conflict: the other writer (the "human edit") wins.
      return ok(id, {
        resultType: "complete",
        isError: true,
        content: [{ type: "text", text: `CONFLICT: ${p} is at version ${current}, expectedVersion=${args.expectedVersion}. Other writer wins; re-read and retry.` }],
        structuredContent: { conflict: true, currentVersion: current },
      });
    }
    fs.writeFileSync(file, String(args.text));
    versions[p] = current + 1;
    saveVersions();
    return ok(id, {
      resultType: "complete",
      content: [{ type: "text", text: `WROTE ${p} at version ${versions[p]}` }],
      structuredContent: { path: p, version: versions[p] },
    });
  }
  return err(id, -32602, `Unknown tool: ${name}`);
}

// PROBE B: dual-era mode. Legacy clients (goose 1.48.0 speaks 2025-11-25 with
// initialize) are served per the legacy revision; modern clients get 2026-07-28.
// This mirrors exactly what spec versioning.mdx prescribes for dual-era servers.
let legacyMode = false;

const rl = readline.createInterface({ input: process.stdin });
rl.on("line", (line) => {
  if (!line.trim()) return;
  wlog("client->server", line);
  let msg;
  try { msg = JSON.parse(line); } catch { return err(null, -32700, "Parse error"); }
  const { id, method, params } = msg;
  if (id === undefined || id === null) return; // notification: modern era has none we must handle

  if (method === "server/discover") {
    return ok(id, {
      resultType: "complete",
      supportedVersions: [PROTO],
      capabilities: { tools: {} },
      _meta: { [`${META}/serverInfo`]: { name: "alfred-vault-strict", version: "0.1.0" } },
      instructions: "Path-confined vault. Reads free. Writes require expectedVersion (write-if-unchanged).",
      ttlMs: 3600000,
      cacheScope: "public",
    });
  }
  if (method === "initialize") {
    // Dual-era: answer the legacy handshake and serve the negotiated legacy revision.
    legacyMode = true;
    return ok(id, {
      protocolVersion: "2025-11-25",
      capabilities: { tools: {} },
      serverInfo: { name: "alfred-vault-dual", version: "0.1.0" },
      instructions: "Path-confined vault. Reads free. Writes require expectedVersion (write-if-unchanged).",
    });
  }
  if (method === "ping") {
    if (legacyMode) return ok(id, {}); // ping exists in legacy revisions
    return err(id, -32601, "Method not found: ping (removed in 2026-07-28)");
  }
  const modern = params?._meta?.[`${META}/protocolVersion`] !== undefined;
  if (method === "tools/list") {
    if (modern) {
      if (!checkMeta(id, params)) return;
      return ok(id, { resultType: "complete", tools: TOOLS, ttlMs: 300000, cacheScope: "public" });
    }
    if (legacyMode) return ok(id, { tools: TOOLS });
    return err(id, -32022, "UnsupportedProtocolVersion: request missing _meta protocolVersion", { supported: [PROTO] });
  }
  if (method === "tools/call") {
    if (modern) {
      if (!checkMeta(id, params)) return;
      return callTool(id, params);
    }
    if (legacyMode) return callToolLegacy(id, params);
    return err(id, -32022, "UnsupportedProtocolVersion: request missing _meta protocolVersion", { supported: [PROTO] });
  }
  return err(id, -32601, `Method not found: ${method}`);
});
rl.on("close", () => process.exit(0));
