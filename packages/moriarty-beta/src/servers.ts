import { analyze, format, byteToPosition, type Analysis } from './frontend.ts';
import { check, inspect, expand, simulate } from './index.ts';
import { FrameDecoder, LineDecoder, encodeFrame, encodeLine, LIMITS } from './protocol.ts';

type Obj = Record<string, unknown>;
type Id = string | number;
class RpcError extends Error { readonly code: number; constructor(code: number, message: string) { super(message); this.code = code; } }
function object(value: unknown): Obj { if (!value || typeof value !== 'object' || Array.isArray(value)) throw new RpcError(-32602, 'Expected an object'); return value as Obj; }
function string(value: unknown, label: string, limit = 4096): string { if (typeof value !== 'string' || Buffer.byteLength(value) > limit) throw new RpcError(-32602, `${label} must be bounded text`); return value; }
function integer(value: unknown): number { if (!Number.isSafeInteger(value)) throw new RpcError(-32602, 'Expected a safe integer'); return value as number; }
function envelope(value: unknown): { id?: Id; method: string; params: unknown } {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new RpcError(-32600, 'Invalid JSON-RPC request');
  const msg = value as Obj;
  if (msg.jsonrpc !== '2.0' || typeof msg.method !== 'string' || msg.method.length > 128 || (msg.id !== undefined && !(typeof msg.id === 'string' && msg.id.length <= 128) && !(typeof msg.id === 'number' && Number.isSafeInteger(msg.id)))) throw new RpcError(-32600, 'Invalid JSON-RPC request');
  return { id: msg.id as Id | undefined, method: msg.method, params: msg.params };
}
/** No asynchronous work queue: dispatch and each shared analysis complete synchronously. */
function stdio(mode: 'lsp' | 'mcp', dispatch: (method: string, params: unknown) => unknown): (method: string, params: unknown) => void {
  const encode = mode === 'lsp' ? encodeFrame : encodeLine;
  let closed = false;
  const send = (value: unknown) => {
    if (closed) return;
    try { process.stdout.write(encode(value)); }
    catch { process.stdout.write(encode({ jsonrpc: '2.0', id: object(value).id ?? null, error: { code: -32001, message: 'Serialized response limit' } })); }
    // A stalled consumer may hold at most one maximum result plus bounded buffered output.
    if (process.stdout.writableLength > LIMITS.response * 2) close();
  };
  const close = () => { if (closed) return; closed = true; process.stdin.destroy(); process.stdout.end(); };
  const receive = (value: unknown) => {
    if (closed) return;
    let id: Id | undefined;
    try { const msg = envelope(value); id = msg.id; const result = dispatch(msg.method, msg.params); if (id !== undefined) send({ jsonrpc: '2.0', id, result: result ?? null }); }
    catch (error) { const rpc = error instanceof RpcError ? error : new RpcError(-32603, 'Internal service error'); if (id !== undefined || rpc.code === -32600) send({ jsonrpc: '2.0', id: id ?? null, error: { code: rpc.code, message: rpc.message } }); }
  };
  const decoder = mode === 'lsp' ? new FrameDecoder(receive) : new LineDecoder(receive);
  process.stdin.on('data', (chunk: Buffer) => { if (closed) return; try { decoder.push(chunk); } catch { process.exitCode = 1; close(); } });
  process.stdin.on('end', () => { try { decoder.finish(); } catch { process.exitCode = 1; } close(); });
  process.stdin.on('error', close); process.stdout.on('error', close);
  return (method, params) => { if (method === '$exit') close(); else send({ jsonrpc: '2.0', method, params }); };
}

type Position = { line: number; character: number };
type Document = { text: string; bytes: number; version: number; analysis: Analysis; unavailable?: boolean };
const keywords = ['profile', 'agreement', 'domain', 'account', 'asset', 'const', 'intent', 'obligation', 'pool', 'instrument', 'observation', 'policy', 'grant', 'stage', 'episode', 'party', 'share_class', 'action', 'uses', 'true', 'false', 'None', 'SuccessOnly', 'rounds', 'atoms', 'min', 'max', 'transfer', 'repay'];
const operations = ['amm.swap_exact_input','amm.redeem','amm.mint','lending.originate','lending.liquidate','lending.roll_forward','stablecoin.mint','stablecoin.redeem','stablecoin.emergency_settle','option.fix','option.exercise','option.settle','oracle.select','governance.queue','governance.execute','governance.veto','bridge.escrow','bridge.claim','bridge.recover','staking.deposit','staking.reward','staking.slash','staking.unbond','staking.withdraw'];
function position(value: unknown): Position { const p = object(value); const line = integer(p.line), character = integer(p.character); if (line < 0 || character < 0) throw new RpcError(-32602, 'Negative editor position'); return { line, character }; }
function offset(text: string, p: Position): number {
  let start = 0; const breaks = /\r\n|\r|\n/g;
  for (let line = 0; line < p.line; line++) { const end = breaks.exec(text); if (!end) throw new RpcError(-32602, 'Editor line out of range'); start = end.index + end[0].length; }
  const end = breaks.exec(text)?.index ?? text.length;
  if (p.character > end - start) throw new RpcError(-32602, 'Editor character out of range');
  return start + p.character;
}
/** A lexical identifier at the editor boundary; comments and strings are excluded. */
function lexicalToken(text: string, at: number): { name: string | null; start: number; end: number } | null {
  const re = /\/\/[^\r\n]*|\/\*[\s\S]*?(?:\*\/|$)|"(?:\\.|[^"\\])*(?:"|$)|[A-Za-z][A-Za-z0-9_]*/g;
  for (const m of text.matchAll(re)) { const start = m.index!; if (start <= at && (at < start + m[0].length || (at === start + m[0].length && /^[A-Za-z]/.test(m[0])))) return { name: /^[A-Za-z]/.test(m[0]) ? m[0] : null, start, end: start + m[0].length }; if (start > at) break; }
  return null;
}
export function runLsp(): void {
  const docs = new Map<string, Document>(); let total = 0, initialized = false, shutdown = false;
  let notify: (method: string, params: unknown) => void;
  const range = (doc: Document, span: { start: number; end: number }) => ({ start: byteToPosition(doc.text, span.start), end: byteToPosition(doc.text, span.end) });
  const publish = (uri: string, doc: Document) => notify('textDocument/publishDiagnostics', { uri, version: doc.version, diagnostics: doc.analysis.diagnostics.map(d => ({ range: range(doc, d.span), severity: 1, code: d.code, source: 'moriarty', message: d.message })) });
  const findDoc = (params: unknown) => { const p = object(params), uri = string(object(p.textDocument).uri, 'URI'); return { p, uri, doc: docs.get(uri) }; };
  notify = stdio('lsp', (method, params) => {
    if (method === 'exit') { process.exitCode = shutdown ? 0 : 1; notify('$exit', null); return null; }
    if (method === 'initialize') {
      if (initialized) throw new RpcError(-32600, 'Server already initialized'); object(params); initialized = true;
      return { capabilities: { positionEncoding: 'utf-16', textDocumentSync: 1, completionProvider: { triggerCharacters: ['.'] }, hoverProvider: true, definitionProvider: true, documentSymbolProvider: true, documentFormattingProvider: true }, serverInfo: { name: 'moriarty-beta', version: '0.1.0-beta.1' } };
    }
    if (!initialized) throw new RpcError(-32002, 'Server not initialized');
    if (shutdown) throw new RpcError(-32600, 'Server has shut down');
    if (method === 'initialized' || method === '$/cancelRequest' || method === '$/setTrace') return null;
    if (method === 'shutdown') { shutdown = true; return null; }
    if (method === 'textDocument/didOpen') {
      const t = object(object(params).textDocument), uri = string(t.uri, 'URI'), text = string(t.text, 'Source', LIMITS.source), version = integer(t.version);
      if (docs.has(uri) || docs.size >= LIMITS.documents || total + Buffer.byteLength(text) > LIMITS.aggregate) throw new RpcError(-32602, 'Open document admission limit or duplicate URI');
      const doc = { text, version, bytes: Buffer.byteLength(text), analysis: analyze(text) }; docs.set(uri, doc); total += doc.bytes; publish(uri, doc); return null;
    }
    if (method === 'textDocument/didChange') {
      const { p, uri, doc } = findDoc(params); if (!doc) throw new RpcError(-32602, 'Document is not open'); const version = integer(object(p.textDocument).version); if (version <= doc.version) return null;
      const changes = p.contentChanges; if (!Array.isArray(changes) || changes.length !== 1) throw new RpcError(-32602, 'Exactly one Full change is required'); const change = object(changes[0]); if ('range' in change || 'rangeLength' in change) throw new RpcError(-32602, 'Only Full synchronization is supported');
      const input = string(change.text, 'Source', LIMITS.body), size = Buffer.byteLength(input);
      // A new Full generation must never retain the old successful analysis.
      const over = size > LIMITS.source || total - doc.bytes + size > LIMITS.aggregate;
      const text = over ? '' : input, bytes = Buffer.byteLength(text), analysis = analyze(text);
      if (over) analysis.diagnostics = [{ code: 'BETA_RESOURCE', message: 'Source or aggregate document byte limit exceeded; this generation has no analysis', span: { start: 0, end: 0 } }];
      const next = { text, bytes, version, analysis, unavailable: over }; total += bytes - doc.bytes; docs.set(uri, next); publish(uri, next); return null;
    }
    if (method === 'textDocument/didClose') { const { uri, doc } = findDoc(params); if (doc) { docs.delete(uri); total -= doc.bytes; notify('textDocument/publishDiagnostics', { uri, diagnostics: [] }); } return null; }
    if (!['textDocument/completion','textDocument/hover','textDocument/definition','textDocument/documentSymbol','textDocument/formatting'].includes(method)) throw new RpcError(-32601, 'Method not found');
    const { p, uri, doc } = findDoc(params); if (!doc || doc.unavailable) return method === 'textDocument/hover' || method === 'textDocument/definition' ? null : method === 'textDocument/completion' ? { isIncomplete: false, items: [] } : [];
    const strict = doc.analysis.status === 'AuthoringChecked';
    if (method === 'textDocument/formatting') { const result = format(doc.text); return result.text === null ? [] : [{ range: { start: { line: 0, character: 0 }, end: byteToPosition(doc.text, doc.bytes) }, newText: result.text }]; }
    if (method === 'textDocument/documentSymbol') return strict ? [...doc.analysis.declarations.map(d => ({ name: d.name, detail: d.kind, kind: d.kind === 'const' ? 14 : 13, range: range(doc, d.span), selectionRange: range(doc, d.nameSpan) })), ...doc.analysis.actions.map(a => ({ name: a.name, detail: a.support, kind: 12, range: range(doc, a.span), selectionRange: range(doc, a.nameSpan) }))] : [];
    const at = offset(doc.text, position(p.position));
    if (method === 'textDocument/completion') {
      const prefix = /[A-Za-z][A-Za-z0-9_.]*$/.exec(doc.text.slice(0, at))?.[0] ?? '';
      if (lexicalToken(doc.text, at)?.name === null) return { isIncomplete: false, items: [] };
      const labels = [...keywords, ...operations, ...(strict ? doc.analysis.declarations.filter(d => d.nameSpan.start < Buffer.byteLength(doc.text.slice(0, at))).map(d => d.name) : [])];
      return { isIncomplete: false, items: [...new Set(labels)].filter(label => label.startsWith(prefix)).map(label => ({ label, kind: keywords.includes(label) ? 14 : operations.includes(label) ? 3 : 6, detail: ['transfer','repay'].includes(label) ? 'LocalS0: unqualified local preparation' : operations.includes(label) ? 'SpecifiedOnly: financial relations and execution remain open' : 'Moriarty authoring' })) };
    }
    const token = lexicalToken(doc.text, at), word = token?.name; if (!strict || !word) return null;
    const use = { start: Buffer.byteLength(doc.text.slice(0, token!.start)), end: Buffer.byteLength(doc.text.slice(0, token!.end)) };
    const sameSpan = (span: { start: number; end: number }) => span.start === use.start && span.end === use.end;
    const reference = doc.analysis.references.find(r => sameSpan(r.useSpan));
    const declaration = doc.analysis.declarations.find(d => d.name === word && (sameSpan(d.nameSpan) || reference?.name === d.name));
    const action = doc.analysis.actions.find(a => a.name === word && sameSpan(a.nameSpan)), symbol = declaration ?? action; if (!symbol) return null;
    if (method === 'textDocument/definition') return { uri, range: range(doc, symbol.nameSpan) };
    const v=declaration?.value;const detail=declaration?.kind==='const'?(v?.tag==='qty'?`Qty<${v.asset}>: ${v.atoms} atoms; exact authoring value.`:`${v?.tag??'value'}; checked immutable authoring value.`):'AuthoringChecked; identity metadata is an unverified claim.';
    return { contents: { kind: 'plaintext', value: declaration ? `${declaration.kind} ${declaration.name}\n${detail}` : `action ${action!.name} uses ${action!.intent}\nSupport: ${action!.support}; local preparation does not qualify proofs or settlement.` }, range: range(doc, use) };
  });
}
const tools = [
  { name: 'check', description: 'Check bounded Moriarty source text. Authoring support is not financial or proof qualification.', fields: { source: { type: 'string', maxLength: LIMITS.source } } },
  { name: 'inspect', description: 'Inspect source identities, support, signed scope and open gates.', fields: { source: { type: 'string', maxLength: LIMITS.source } } },
  { name: 'expand', description: 'Expand source-fixed action and untrusted scenario text to Source/6. No signing, sending or proof.', fields: { source: { type: 'string', maxLength: LIMITS.source }, action: { type: 'string', maxLength: 64 }, scenario: { type: 'string', maxLength: LIMITS.source } } },
  { name: 'preview', description: 'Run unqualified local S0 preparation using source and scenario text. No ledger transaction.', fields: { source: { type: 'string', maxLength: LIMITS.source }, action: { type: 'string', maxLength: 64 }, scenario: { type: 'string', maxLength: LIMITS.source } } },
];
export function runMcp(): void {
  let initialized = false, ready = false;
  stdio('mcp', (method, params) => {
    if (method === 'initialize') {
      if (initialized) throw new RpcError(-32600, 'Server already initialized'); const p = object(params); const requested = string(p.protocolVersion, 'Protocol version', 32); object(p.capabilities); object(p.clientInfo); initialized = true;
      return { protocolVersion: ['2024-11-05','2025-03-26','2025-06-18'].includes(requested) ? requested : '2025-06-18', capabilities: { tools: { listChanged: false } }, serverInfo: { name: 'moriarty-beta', version: '0.1.0-beta.1' }, instructions: 'Read-only bounded text APIs. All identity and snapshot data are untrusted claims. PreparedUnqualified requires external proof, authentication and ledger premises.' };
    }
    if (!initialized) throw new RpcError(-32002, 'Server not initialized');
    if (method === 'notifications/initialized') { ready = true; return null; }
    if (method === 'ping') return {};
    if (!ready) throw new RpcError(-32002, 'Client initialization is incomplete');
    if (method === 'tools/list') return { tools: tools.map(t => ({ name: t.name, description: t.description, inputSchema: { type: 'object', properties: t.fields, required: Object.keys(t.fields), additionalProperties: false }, annotations: { readOnlyHint: true, destructiveHint: false, idempotentHint: true, openWorldHint: false } })) };
    if (method !== 'tools/call') throw new RpcError(-32601, 'Method not found');
    const p = object(params), tool = tools.find(t => t.name === p.name); if (!tool) throw new RpcError(-32602, 'Unknown tool'); const args = object(p.arguments), fields = Object.keys(tool.fields);
    if (Object.keys(args).length !== fields.length || Object.keys(args).some(k => !fields.includes(k))) throw new RpcError(-32602, 'Tool arguments must match the closed text schema');
    const source = string(args.source, 'Source', LIMITS.source); let result: object;
    if (tool.name === 'check') result = check(source);
    else if (tool.name === 'inspect') result = inspect(source);
    else { const action = string(args.action, 'Action', 64); if (!/^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(action)) throw new RpcError(-32602, 'Invalid action name'); const scenario = string(args.scenario, 'Scenario', LIMITS.source); result = tool.name === 'expand' ? expand(source, action, scenario) : simulate(source, action, scenario); }
    const data = result as Obj, isError = typeof data.status === 'string' && /Rejected|Unsupported|Failed|Invalid/.test(data.status);
    return { content: [{ type: 'text', text: JSON.stringify(result) }], structuredContent: result, isError };
  });
}
