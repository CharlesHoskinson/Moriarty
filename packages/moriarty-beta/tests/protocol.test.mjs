import test from 'node:test';
import assert from 'node:assert/strict';
import { buildSync } from 'esbuild';
import { spawn } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { starterSource, starterScenario } from '../src/starter.ts';
import {transferSource,repaySource} from './fixtures.mjs';

async function protocol() { return import('../src/protocol.ts'); }
test('LSP framing accepts split Unicode frames and rejects excess before body admission', async () => {
  const { FrameDecoder, encodeFrame } = await protocol();
  const frames = []; const decoder = new FrameDecoder(value => frames.push(value));
  const data = encodeFrame({ text: '😀' });
  for (const byte of data) decoder.push(Buffer.from([byte]));
  assert.deepEqual(frames, [{ text: '😀' }]);
  assert.throws(() => new FrameDecoder(()=>{}).push(Buffer.from('Content-Length: 262145\r\n\r\n')), /body|limit/i);
  assert.throws(() => new FrameDecoder(()=>{}).push(Buffer.alloc(8193, 65)), /header|limit/i);
  assert.throws(() => new FrameDecoder(()=>{}).push(Buffer.from('Content-Length: 2\r\nContent-Length: 2\r\n\r\n{}')), /length|header/i);
  assert.throws(() => encodeFrame({text:'x'.repeat(524288)}), /response|limit/i);
});
function client(kind) {
  const dir = mkdtempSync(join(tmpdir(), 'mori-protocol-'));
  const cli = process.env.MORIARTY_BETA_PROTOCOL_CLI;
  if(!cli) buildSync({ stdin:{contents:`import {runLsp,runMcp} from ${JSON.stringify(fileURLToPath(new URL('../src/servers.ts',import.meta.url)))}; ${kind==='lsp'?'runLsp':'runMcp'}();`,resolveDir:process.cwd()},outfile:join(dir,'server.mjs'),bundle:true,platform:'node',format:'esm',logLevel:'silent'});
  const child=spawn(process.execPath,cli ? [cli,kind] : [join(dir,'server.mjs')],{stdio:['pipe','pipe','pipe']});
  let buffer=Buffer.alloc(0), id=0, stderr=''; const replies=new Map(), notices=[];
  child.stderr.on('data',chunk=>stderr+=chunk); child.stdout.on('data',chunk=>{
    buffer=Buffer.concat([buffer,chunk]);
    while(true) {
      let body;
      if(kind==='lsp') { const end=buffer.indexOf('\r\n\r\n'); if(end<0)break; const n=Number(/Content-Length: (\d+)/i.exec(buffer.subarray(0,end).toString())?.[1]); assert.ok(Number.isInteger(n),'stdout must be pure framed JSON'); if(buffer.length<end+4+n)break; body=buffer.subarray(end+4,end+4+n); buffer=buffer.subarray(end+4+n); }
      else {const end=buffer.indexOf('\n');if(end<0)break;body=buffer.subarray(0,end);buffer=buffer.subarray(end+1);}
      const value=JSON.parse(body.toString()); if(value.id!==undefined && value.id!==null) { replies.get(value.id)?.(value);replies.delete(value.id); }else notices.push(value);
    }
  });
  const send=value=>{const body=JSON.stringify(value);child.stdin.write(kind==='lsp'?`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`:body+'\n');};
  return {child,notices,send,request(method,params){ const requestId=++id; return new Promise((resolve,reject)=>{const timer=setTimeout(()=>reject(new Error(`timeout ${method}: ${stderr}`)),4000); replies.set(requestId,value=>{clearTimeout(timer);resolve(value);});send({jsonrpc:'2.0',id:requestId,method,params});}); },async stop(){if(child.exitCode===null && child.signalCode===null) {child.kill();await new Promise(resolve=>child.once('close',resolve));}rmSync(dir,{recursive:true,force:true});}};
}
const source='profile "moriarty-beta/1";\nagreement Demo {\n // 😀\n const amount = 1;\n const total = amount + 1;\n}';
test('real LSP full sync publishes UTF16 diagnostics, honors versions, navigates and formats', async () => {
  const c=client('lsp');try {
    const init=await c.request('initialize',{capabilities:{}});assert.equal(init.result.capabilities.textDocumentSync,1);assert.equal(init.result.capabilities.positionEncoding,'utf-16');
    c.send({jsonrpc:'2.0',method:'initialized',params:{}});
    const uri='file:///demo.mori';c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri,languageId:'moriarty',version:1,text:source}}});
    const symbols=await c.request('textDocument/documentSymbol',{textDocument:{uri}});assert.ok(symbols.result.some(s=>s.name==='amount'));
    const definition=await c.request('textDocument/definition',{textDocument:{uri},position:{line:4,character:16}});assert.equal(definition.result.range.start.line,3);
    const hover=await c.request('textDocument/hover',{textDocument:{uri},position:{line:4,character:16}});assert.match(hover.result.contents.value,/amount/);
    const fmt=await c.request('textDocument/formatting',{textDocument:{uri},options:{tabSize:2,insertSpaces:true}});assert.equal(fmt.result.length,1);
    c.send({jsonrpc:'2.0',method:'textDocument/didChange',params:{textDocument:{uri,version:2},contentChanges:[{text:'profile "moriarty-beta/1"; agreement Demo { // 😀\n const x = missing; }'}]}});
    c.send({jsonrpc:'2.0',method:'textDocument/didChange',params:{textDocument:{uri,version:1},contentChanges:[{text:source}]}});
    const badFmt=await c.request('textDocument/formatting',{textDocument:{uri},options:{}});assert.deepEqual(badFmt.result,[]);
    const diagnostic=c.notices.filter(n=>n.method==='textDocument/publishDiagnostics').at(-1);assert.equal(diagnostic.params.version,2);assert.ok(diagnostic.params.diagnostics.length>0);
    const partial=await c.request('textDocument/completion',{textDocument:{uri},position:{line:1,character:5}});assert.ok(partial.result.items.some(i=>i.label==='const'));
    c.send({jsonrpc:'2.0',method:'textDocument/didClose',params:{textDocument:{uri}}});
    await c.request('textDocument/documentSymbol',{textDocument:{uri}});assert.deepEqual(c.notices.at(-1).params.diagnostics,[]);
    assert.equal((await c.request('shutdown',null)).result,null);c.send({jsonrpc:'2.0',method:'exit'});
  }finally{await c.stop();}
});
test('real MCP initialization and tools use bounded closed text arguments', async () => {
  const c=client('mcp');try {
    const init=await c.request('initialize',{protocolVersion:'2025-06-18',capabilities:{},clientInfo:{name:'test',version:'1'}});assert.equal(init.result.protocolVersion,'2025-06-18');
    c.send({jsonrpc:'2.0',method:'notifications/initialized'});
    const list=await c.request('tools/list',{});assert.deepEqual(list.result.tools.map(t=>t.name),['check','inspect','expand','preview']);
    for(const [text,name,operation,amount] of [[transferSource,'Payment','transfer','1000'],[repaySource,'Repayment','repay','3000']]){
      const reply=await c.request('tools/call',{name:'inspect',arguments:{source:text}});assert.equal(reply.result.isError,false);
      const terms=reply.result.structuredContent.intents.find(i=>i.name===name).terms;
      assert.equal(terms.operation.name,operation);assert.equal(terms.operation.args[operation==='transfer'?'value':'amount'].atoms,amount);assert.equal(terms.valid.args.to.value,'10');assert.equal(terms.signer.authenticated,false);
      assert.deepEqual(JSON.parse(reply.result.content[0].text),reply.result.structuredContent);
    }
    for(const name of ['check','inspect']) {const reply=await c.request('tools/call',{name,arguments:{source}});assert.equal(reply.result.isError,false);assert.equal(JSON.parse(reply.result.content[0].text).status,'AuthoringChecked');}
    for(const name of ['expand','preview']) {const reply=await c.request('tools/call',{name,arguments:{source,action:'missing',scenario:'{}'}});assert.ok(reply.result.content[0].text.includes('diagnostics'));}
    for(const args of [{source,path:'/etc/passwd'},{source:{ast:true}},{source:'x'.repeat(65537)}]) {assert.equal((await c.request('tools/call',{name:'check',arguments:args})).error.code,-32602);}
    assert.equal((await c.request('tools/call',{name:'shell',arguments:{source}})).error.code,-32602);
  }finally{await c.stop();}
});

test('LSP hover range covers the reference and lexical completion excludes comments and strings', async () => {
  const c=client('lsp');try {
    await c.request('initialize',{});const uri='file:///lexical.mori';
    const text='profile "moriarty-beta/1";\nagreement Demo {\n const amount = 1;\n const total = amount + 1;\n // profile const\n const title = "profile";\n}';
    c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri,version:1,text}}});
    const hover=await c.request('textDocument/hover',{textDocument:{uri},position:{line:3,character:17}});assert.equal(hover.result.range.start.line,3);
    for(const position of [{line:4,character:11},{line:5,character:19}]) {const r=await c.request('textDocument/completion',{textDocument:{uri},position});assert.deepEqual(r.result.items,[]);}
  }finally{await c.stop();}
});
test('new over-budget Full source invalidates old analysis; diagnostic locations count UTF16', async () => {
  const c=client('lsp');try {
    await c.request('initialize',{});const uri='file:///unicode.mori';
    const text='profile "moriarty-beta/1";\r\nagreement Demo { const title = "😀"; const x = missing; }';
    c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri,version:1,text}}});
    await c.request('textDocument/documentSymbol',{textDocument:{uri}});
    const diag=c.notices.at(-1).params.diagnostics.find(d=>d.message.includes('missing'));
    assert.equal(diag.range.start.line,1);assert.equal(diag.range.start.character,text.split('\r\n')[1].indexOf('missing'));
    c.send({jsonrpc:'2.0',method:'textDocument/didChange',params:{textDocument:{uri,version:2},contentChanges:[{text:source}]}});
    assert.ok((await c.request('textDocument/documentSymbol',{textDocument:{uri}})).result.length);
    c.send({jsonrpc:'2.0',method:'textDocument/didChange',params:{textDocument:{uri,version:3},contentChanges:[{text:'x'.repeat(65537)}]}});
    assert.deepEqual((await c.request('textDocument/documentSymbol',{textDocument:{uri}})).result,[]);
    const notice=c.notices.at(-1);assert.equal(notice.params.version,3);assert.ok(notice.params.diagnostics.length);
    for(const method of ['textDocument/completion','textDocument/hover','textDocument/definition','textDocument/formatting']){
      const reply=await c.request(method,{textDocument:{uri},position:{line:99,character:99},options:{}});
      assert.equal(reply.error,undefined);assert.deepEqual(reply.result,method.endsWith('completion')?{isIncomplete:false,items:[]}:method.endsWith('formatting')?[]:null);
    }
    c.send({jsonrpc:'2.0',method:'textDocument/didChange',params:{textDocument:{uri,version:4},contentChanges:[{text:source}]}});
    assert.ok((await c.request('textDocument/documentSymbol',{textDocument:{uri}})).result.length);
  }finally{await c.stop();}
});
test('LSP uses identical CR, CRLF and LF positions for navigation and comment completion',async()=>{
 const c=client('lsp');try{
  await c.request('initialize',{});
  for(const [i,eol] of ['\r','\r\n','\n'].entries()){
   const uri=`file:///newline-${i}.mori`,text=['profile "moriarty-beta/1";','agreement Demo {',' const amount = 1;',' const total = amount + 1;','}'].join(eol);
   c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri,version:1,text}}});
   const symbols=await c.request('textDocument/documentSymbol',{textDocument:{uri}});assert.ok(symbols.result.some(s=>s.name==='amount'&&s.range.start.line===2));
   const def=await c.request('textDocument/definition',{textDocument:{uri},position:{line:3,character:16}});assert.equal(def.error,undefined);assert.equal(def.result.range.start.line,2);
   const comment=text.replace(' const total',' // comment'+eol+' const total');
   c.send({jsonrpc:'2.0',method:'textDocument/didChange',params:{textDocument:{uri,version:2},contentChanges:[{text:comment}]}});
   const completion=await c.request('textDocument/completion',{textDocument:{uri},position:{line:4,character:3}});assert.equal(completion.error,undefined);assert.ok(completion.result.items.some(x=>x.label==='const'));
  }
 }finally{await c.stop();}
});
test('document count admits at most32; duplicate opens do not replace current source', async () => {
  const c=client('lsp');try {
    await c.request('initialize',{});
    for(let i=0;i<33;i++) c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri:`file:///count${i}.mori`,version:1,text:source}}});
    assert.ok((await c.request('textDocument/documentSymbol',{textDocument:{uri:'file:///count31.mori'}})).result.length);
    assert.deepEqual((await c.request('textDocument/documentSymbol',{textDocument:{uri:'file:///count32.mori'}})).result,[]);
    c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri:'file:///count0.mori',version:100,text:'incomplete'}}});
    assert.ok((await c.request('textDocument/documentSymbol',{textDocument:{uri:'file:///count0.mori'}})).result.length);
    c.send({jsonrpc:'2.0',method:'textDocument/didClose',params:{textDocument:{uri:'file:///count0.mori'}}});
    c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri:'file:///count32.mori',version:1,text:source}}});
    assert.ok((await c.request('textDocument/documentSymbol',{textDocument:{uri:'file:///count32.mori'}})).result.length);
  }finally{await c.stop();}
});
test('real MCP expands and previews S0 through shared library with retained unqualified premises', async () => {
  const c=client('mcp');try {
    await c.request('initialize',{protocolVersion:'2025-06-18',capabilities:{},clientInfo:{name:'test',version:'1'}});c.send({jsonrpc:'2.0',method:'notifications/initialized'});
    const args={source:starterSource,action:'pay',scenario:JSON.stringify(starterScenario)};
    const expansion=(await c.request('tools/call',{name:'expand',arguments:args})).result;assert.equal(expansion.isError,false);assert.equal(expansion.structuredContent.status,'Expanded');assert.match(expansion.structuredContent.source6,/moriarty-financial-agreement-source\/6/);
    const preview=(await c.request('tools/call',{name:'preview',arguments:args})).result;assert.equal(preview.isError,false);assert.equal(preview.structuredContent.status,'PreparedUnqualified');
    const result=preview.structuredContent.result;assert.equal(result.candidate.candidatePost.balances[0].amount,'8990');assert.ok(result.candidate.requiredPremises.length);assert.ok(result.unverifiedBindings.length);
  }finally{await c.stop();}
});

test('aggregate document bytes are bounded independently of document count', async () => {
  const c=client('lsp');try {
    await c.request('initialize',{});const large='/*'+'.'.repeat(65000)+'*/\n'+source;
    for(let i=0;i<17;i++) c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri:`file:///large${i}.mori`,version:1,text:large}}});
    assert.ok((await c.request('textDocument/documentSymbol',{textDocument:{uri:'file:///large15.mori'}})).result.length);
    assert.deepEqual((await c.request('textDocument/documentSymbol',{textDocument:{uri:'file:///large16.mori'}})).result,[]);
  }finally{await c.stop();}
});
test('framing violations close real stdio processes with no nonprotocol stdout', async () => {
  for(const kind of ['lsp','mcp']) {
    const c=client(kind);try {
      const ended=new Promise(resolve=>c.child.once('close',(code)=>resolve(code)));
      c.child.stdin.write(kind==='lsp'?'Content-Length: 262145\r\n\r\n':Buffer.alloc(262145,65));
      assert.equal(await Promise.race([ended,new Promise((_,reject)=>{const timer=setTimeout(()=>reject(new Error('oversize did not close')),4000);timer.unref();})]),1);
      assert.deepEqual(c.notices,[]);
    }finally{await c.stop();}
  }
});
test('invalid JSON-RPC envelopes receive Invalid Request, not internal errors', async () => {
  const c=client('mcp');try {
    c.send([]);
    await c.request('initialize',{protocolVersion:'2025-06-18',capabilities:{},clientInfo:{name:'test',version:'1'}});
    // Responses with null IDs are notifications to this test client; the service must reply.
    assert.ok(c.notices.some(n=>n.error?.code===-32600));
  }finally{await c.stop();}
});

test('hover and definition distinguish record keys from real source references', async () => {
  const c=client('lsp');try {
    await c.request('initialize',{});const uri='file:///references.mori';const record=' const config = { amount: amount };';
    const text='profile "moriarty-beta/1";\nagreement Demo {\n const amount = 1;\n'+record+'\n}';
    c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri,version:1,text}}});
    for(const method of ['textDocument/hover','textDocument/definition']) {
      assert.equal((await c.request(method,{textDocument:{uri},position:{line:3,character:record.indexOf('amount')+2}})).result,null);
      assert.ok((await c.request(method,{textDocument:{uri},position:{line:3,character:record.lastIndexOf('amount')+2}})).result);
    }
  }finally{await c.stop();}
});
test('LSP quantity hover describes exact atoms rather than an identity claim',async()=>{
 const c=client('lsp');try{
  await c.request('initialize',{});const uri='file:///quantity.mori';
  c.send({jsonrpc:'2.0',method:'textDocument/didOpen',params:{textDocument:{uri,version:1,text:transferSource}}});
  const line=transferSource.split('\n').findIndex(x=>x.includes('const price'));
  const character=transferSource.split('\n')[line].indexOf('price')+1;
  const h=await c.request('textDocument/hover',{textDocument:{uri},position:{line,character}});
  assert.match(h.result.contents.value,/Qty<USD>.*1000 atoms/);assert.ok(!h.result.contents.value.includes('identity metadata'));
 }finally{await c.stop();}
});
