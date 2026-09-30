import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";

const cli = new URL("../node_modules/@moriarty-lang/beta/dist/cli.js", import.meta.url);
const sourcePath = new URL("../bridge-stake/vault-bridge.mori", import.meta.url);
const source = readFileSync(sourcePath, "utf8");
const scenario = readFileSync(new URL("../bridge-stake/scenarios/home-rail.json", import.meta.url), "utf8");

function start(args) {
  const child = spawn(process.execPath, [cli.pathname, ...args], { stdio: ["pipe", "pipe", "pipe"] });
  let stderr = "";
  child.stderr.on("data", (chunk) => { stderr += chunk.toString("utf8"); });
  return { child, stderr: () => stderr };
}

function frame(value) {
  const body = Buffer.from(JSON.stringify(value));
  return Buffer.concat([Buffer.from(`Content-Length: ${body.length}\r\n\r\n`), body]);
}

async function lsp() {
  const { child, stderr } = start(["lsp"]);
  const pending = new Map();
  const notes = [];
  let buf = Buffer.alloc(0);
  child.stdout.on("data", (chunk) => {
    buf = Buffer.concat([buf, chunk]);
    while (true) {
      const headerEnd = buf.indexOf("\r\n\r\n");
      if (headerEnd < 0) return;
      const header = buf.subarray(0, headerEnd).toString("ascii");
      const match = /Content-Length:\s*(\d+)/i.exec(header);
      if (!match) throw new Error(`bad lsp header ${header}`);
      const length = Number(match[1]);
      const startAt = headerEnd + 4;
      if (buf.length < startAt + length) return;
      const body = JSON.parse(buf.subarray(startAt, startAt + length).toString("utf8"));
      buf = buf.subarray(startAt + length);
      if (body.id !== undefined && pending.has(body.id)) pending.get(body.id)(body);
      else notes.push(body);
    }
  });
  const call = (id, method, params) => new Promise((resolve) => {
    pending.set(id, resolve);
    child.stdin.write(frame({ jsonrpc: "2.0", id, method, params }));
  });
  const note = (method, params) => child.stdin.write(frame({ jsonrpc: "2.0", method, params }));
  const init = await call(1, "initialize", { processId: null, rootUri: null, capabilities: { general: { positionEncodings: ["utf-16"] } } });
  note("initialized", {});
  const uri = "file:///tmp/moriarty-g3-vault-bridge.mori";
  note("textDocument/didOpen", { textDocument: { uri, languageId: "moriarty", version: 1, text: source } });
  const symbols = await call(2, "textDocument/documentSymbol", { textDocument: { uri } });
  const actionLine = source.split(/\r\n|\n|\r/).findIndex((line) => line.includes("action escrow_home"));
  const hover = await call(3, "textDocument/hover", { textDocument: { uri }, position: { line: actionLine, character: 10 } });
  const tinyUri = "file:///tmp/moriarty-g3-tiny.mori";
  const tiny = "profile \"moriarty-beta/1\";\nagreement Tiny {\n  const next = bridge.\n}\n";
  note("textDocument/didOpen", { textDocument: { uri: tinyUri, languageId: "moriarty", version: 1, text: tiny } });
  const completion = await call(4, "textDocument/completion", { textDocument: { uri: tinyUri }, position: { line: 2, character: 22 } });
  const broken = tiny.replace("bridge.", "01;");
  note("textDocument/didChange", { textDocument: { uri: tinyUri, version: 2 }, contentChanges: [{ text: broken }] });
  await new Promise((resolve) => setTimeout(resolve, 50));
  const shutdown = await call(5, "shutdown", {});
  note("exit", {});
  await new Promise((resolve) => child.on("exit", resolve));
  const labels = (completion.result?.items ?? []).map((item) => item.label);
  return {
    initialize: init.result?.capabilities ?? init,
    serverInfo: init.result?.serverInfo ?? null,
    symbolCount: Array.isArray(symbols.result) ? symbols.result.length : null,
    symbolSample: Array.isArray(symbols.result) ? symbols.result.slice(0, 4).map((item) => ({ name: item.name, detail: item.detail })) : symbols,
    hover: hover.result,
    bridgeCompletion: labels.filter((label) => label.startsWith("bridge.") || label.startsWith("staking.")),
    diagnostics: notes.filter((item) => item.method === "textDocument/publishDiagnostics").map((item) => ({
      uri: item.params.uri,
      codes: (item.params.diagnostics ?? []).map((diagnostic) => diagnostic.code),
    })),
    shutdown: shutdown.result ?? shutdown,
    stderr: stderr(),
  };
}

async function mcp() {
  const { child, stderr } = start(["mcp"]);
  let buf = "";
  const pending = new Map();
  const notes = [];
  child.stdout.on("data", (chunk) => {
    buf += chunk.toString("utf8");
    let nl;
    while ((nl = buf.indexOf("\n")) >= 0) {
      const line = buf.slice(0, nl);
      buf = buf.slice(nl + 1);
      if (!line) continue;
      const body = JSON.parse(line);
      if (body.id !== undefined && pending.has(body.id)) pending.get(body.id)(body);
      else notes.push(body);
    }
  });
  const call = (id, method, params) => new Promise((resolve) => {
    pending.set(id, resolve);
    child.stdin.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
  });
  const note = (method, params) => child.stdin.write(JSON.stringify({ jsonrpc: "2.0", method, params }) + "\n");
  const init = await call(1, "initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g3-probe", version: "0" } });
  note("notifications/initialized", {});
  const tools = await call(2, "tools/list", {});
  const checked = await call(3, "tools/call", { name: "check", arguments: { source } });
  const preview = await call(4, "tools/call", { name: "preview", arguments: { source, action: "escrow_home", scenario } });
  child.stdin.end();
  await new Promise((resolve) => child.on("exit", resolve));
  const checkBody = JSON.parse(checked.result.content[0].text);
  const previewBody = JSON.parse(preview.result.content[0].text);
  return {
    serverInfo: init.result?.serverInfo ?? null,
    instructions: init.result?.instructions ?? null,
    toolNames: (tools.result?.tools ?? []).map((tool) => tool.name),
    checkStatus: checkBody.status,
    checkActions: (checkBody.actions ?? []).map((action) => action.support),
    preview: { status: previewBody.status, code: previewBody.diagnostics?.[0]?.code ?? null, isError: preview.result.isError, publishedEffects: previewBody.publishedEffects },
    notes,
    stderr: stderr(),
  };
}

const lspResult = await lsp();
const mcpResult = await mcp();
process.stdout.write(JSON.stringify({ lsp: lspResult, mcp: mcpResult }, null, 2) + "\n");
