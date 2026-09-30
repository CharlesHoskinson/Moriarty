// Tiny local stdio clients for the installed Moriarty CLI. No editor settings are changed.
import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";

const cli = new URL("../../node_modules/@moriarty-lang/beta/dist/cli.js", import.meta.url);
const source = readFileSync(new URL("../servicing.mori", import.meta.url), "utf8");
const invalid = readFileSync(new URL("../invalid-window.mori", import.meta.url), "utf8");
const scenario = readFileSync(new URL("../scenarios/installment.json", import.meta.url), "utf8");

function start(args) {
  return spawn(process.execPath, [cli.pathname, ...args], { stdio: ["pipe", "pipe", "pipe"] });
}

function frames(child) {
  let buffer = Buffer.alloc(0);
  const notes = [];
  const responses = [];
  const waiters = [];
  const deliver = (body) => {
    const waiter = waiters.shift();
    if (waiter) waiter(body);
    else responses.push(body);
  };
  child.stdout.on("data", (chunk) => {
    buffer = Buffer.concat([buffer, chunk]);
    while (true) {
      const headerEnd = buffer.indexOf("\r\n\r\n");
      if (headerEnd < 0) return;
      const header = buffer.subarray(0, headerEnd).toString("ascii");
      const match = /content-length:\s*(\d+)/i.exec(header);
      if (!match) throw new Error(`bad lsp header ${header}`);
      const length = Number(match[1]);
      const start = headerEnd + 4;
      if (buffer.length < start + length) return;
      const body = JSON.parse(buffer.subarray(start, start + length).toString("utf8"));
      buffer = buffer.subarray(start + length);
      if (body.id === undefined) notes.push(body);
      else deliver(body);
    }
  });
  return {
    notes,
    send(value) {
      const body = Buffer.from(JSON.stringify(value));
      child.stdin.write(`Content-Length: ${body.length}\r\n\r\n`);
      child.stdin.write(body);
    },
    next() {
      const queued = responses.shift();
      if (queued) return Promise.resolve(queued);
      return new Promise((resolve) => waiters.push(resolve));
    },
  };
}

function lines(child) {
  let text = "";
  const queue = [];
  const waiters = [];
  child.stdout.on("data", (chunk) => {
    text += chunk.toString("utf8");
    while (true) {
      const nl = text.indexOf("\n");
      if (nl < 0) return;
      const line = text.slice(0, nl);
      text = text.slice(nl + 1);
      if (!line) continue;
      const message = JSON.parse(line);
      const waiter = waiters.shift();
      if (waiter) waiter(message);
      else queue.push(message);
    }
  });
  return {
    send(value) {
      child.stdin.write(JSON.stringify(value) + "\n");
    },
    next() {
      const queued = queue.shift();
      if (queued) return Promise.resolve(queued);
      return new Promise((resolve) => waiters.push(resolve));
    },
  };
}

const report = { mcp: {}, lsp: {} };

const mcp = start(["mcp"]);
const mcpIo = lines(mcp);
mcpIo.send({
  jsonrpc: "2.0",
  id: 1,
  method: "initialize",
  params: {
    protocolVersion: "2025-06-18",
    capabilities: {},
    clientInfo: { name: "g2-local-probe", version: "0" },
  },
});
const init = await mcpIo.next();
report.mcp.initialize = {
  protocolVersion: init.result?.protocolVersion,
  server: init.result?.serverInfo,
  toolCapability: init.result?.capabilities?.tools ?? null,
};
mcpIo.send({ jsonrpc: "2.0", id: 2, method: "tools/list" });
const early = await mcpIo.next();
report.mcp.beforeInitialized = early.error ?? null;
mcpIo.send({ jsonrpc: "2.0", method: "notifications/initialized" });
mcpIo.send({ jsonrpc: "2.0", id: 3, method: "tools/list" });
const listed = await mcpIo.next();
report.mcp.tools = listed.result?.tools?.map((tool) => tool.name) ?? listed;
mcpIo.send({
  jsonrpc: "2.0",
  id: 4,
  method: "tools/call",
  params: { name: "check", arguments: { source } },
});
const checked = await mcpIo.next();
const checkBody = JSON.parse(checked.result.content[0].text);
report.mcp.check = {
  isError: checked.result.isError,
  status: checkBody.status,
  actions: checkBody.actions?.map((action) => ({ name: action.name, support: action.support })),
};
mcpIo.send({
  jsonrpc: "2.0",
  id: 5,
  method: "tools/call",
  params: {
    name: "preview",
    arguments: { source, action: "service_installment", scenario },
  },
});
const preview = await mcpIo.next();
const previewBody = JSON.parse(preview.result.content[0].text);
report.mcp.preview = {
  isError: preview.result.isError,
  status: previewBody.status,
  qualification: previewBody.qualification ?? null,
  premises: previewBody.result?.candidate?.requiredPremises ?? null,
  bindings: previewBody.result?.unverifiedBindings ?? null,
  obligation: previewBody.result?.candidate?.candidatePost?.obligations?.[0] ?? null,
};
mcpIo.send({
  jsonrpc: "2.0",
  id: 6,
  method: "tools/call",
  params: {
    name: "preview",
    arguments: { source, action: "roll", scenario },
  },
});
const roll = await mcpIo.next();
const rollBody = JSON.parse(roll.result.content[0].text);
report.mcp.roll = {
  isError: roll.result.isError,
  status: rollBody.status,
  code: rollBody.diagnostics?.[0]?.code ?? null,
  publishedEffects: rollBody.publishedEffects,
};
mcp.stdin.end();
await new Promise((resolve) => mcp.on("close", resolve));

const lsp = start(["lsp"]);
const lspIo = frames(lsp);
const uri = "file:///tmp/moriarty-g2-servicing.mori";
lspIo.send({
  jsonrpc: "2.0",
  id: 1,
  method: "initialize",
  params: {
    processId: process.pid,
    rootUri: null,
    capabilities: {},
    clientInfo: { name: "g2-local-probe", version: "0" },
  },
});
const lspInit = await lspIo.next();
report.lsp.encoding = lspInit.result?.capabilities?.positionEncoding ?? null;
report.lsp.sync = lspInit.result?.capabilities?.textDocumentSync ?? null;
lspIo.send({ jsonrpc: "2.0", method: "initialized", params: {} });
lspIo.send({
  jsonrpc: "2.0",
  method: "textDocument/didOpen",
  params: { textDocument: { uri, languageId: "moriarty", version: 1, text: source } },
});
lspIo.send({
  jsonrpc: "2.0",
  id: 2,
  method: "textDocument/documentSymbol",
  params: { textDocument: { uri } },
});
const symbols = await lspIo.next();
report.lsp.symbols = symbols.result?.map((symbol) => ({ name: symbol.name, detail: symbol.detail })) ?? symbols;
const principal = source.split("\n").findIndex((line) => line.includes("const principal:"));
lspIo.send({
  jsonrpc: "2.0",
  id: 3,
  method: "textDocument/hover",
  params: { textDocument: { uri }, position: { line: principal, character: 8 } },
});
const hover = await lspIo.next();
report.lsp.hover = hover.result?.contents?.value ?? hover.result;
lspIo.send({
  jsonrpc: "2.0",
  id: 4,
  method: "textDocument/definition",
  params: { textDocument: { uri }, position: { line: principal, character: 8 } },
});
const definition = await lspIo.next();
report.lsp.definitionLine = definition.result?.range?.start?.line ?? definition;
const invalidUri = "file:///tmp/moriarty-g2-invalid-window.mori";
lspIo.send({
  jsonrpc: "2.0",
  method: "textDocument/didOpen",
  params: { textDocument: { uri: invalidUri, languageId: "moriarty", version: 1, text: invalid } },
});
const diagnostic = await new Promise((resolve) => {
  const timer = setTimeout(() => resolve(null), 1000);
  const existing = lspIo.notes.find((note) => note.method === "textDocument/publishDiagnostics" && note.params?.uri === invalidUri);
  if (existing) {
    clearTimeout(timer);
    resolve(existing);
    return;
  }
  const check = setInterval(() => {
    const found = lspIo.notes.find((note) => note.method === "textDocument/publishDiagnostics" && note.params?.uri === invalidUri);
    if (found) {
      clearInterval(check);
      clearTimeout(timer);
      resolve(found);
    }
  }, 10);
});
report.lsp.invalidDiagnostics = diagnostic?.params?.diagnostics?.map((item) => ({
  code: item.code,
  message: item.message,
  line: item.range?.start?.line,
  character: item.range?.start?.character,
})) ?? diagnostic;
lspIo.send({ jsonrpc: "2.0", id: 5, method: "shutdown" });
await lspIo.next();
lspIo.send({ jsonrpc: "2.0", method: "exit" });
lsp.stdin.end();
await new Promise((resolve) => lsp.on("close", resolve));
report.lsp.exitCode = lsp.exitCode;

process.stdout.write(JSON.stringify(report, null, 2) + "\n");
