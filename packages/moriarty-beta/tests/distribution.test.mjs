import test from 'node:test';import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';import {mkdtempSync,rmSync,readFileSync,writeFileSync} from 'node:fs';import {tmpdir} from 'node:os';import {join} from 'node:path';
const root=new URL('..',import.meta.url).pathname;
test('packed install outside checkout supports CLI, side-effect-free public API and declaration imports',()=>{
 const temp=mkdtempSync(join(tmpdir(),'mori-packed-'));
 try{
  const receipt=JSON.parse(execFileSync('npm',['pack','--json','--pack-destination',temp],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}))[0];
  assert.ok(receipt.files.some(x=>x.path==='dist/cli.js'));assert.ok(receipt.files.some(x=>x.path.endsWith('src/index.d.ts')));
  assert.ok(!receipt.files.some(x=>x.path.includes('/node_modules/')||x.path.includes('.foreman')));
  execFileSync('npm',['install','--prefix',temp,join(temp,receipt.filename),'--ignore-scripts','--no-audit','--no-fund'],{encoding:'utf8'});
  const cli=join(temp,'node_modules/@moriarty-lang/beta/dist/cli.js'),project=join(temp,'invoice');
  assert.deepEqual(readFileSync(join(temp,'node_modules/@moriarty-lang/beta/editor/vscode/server/cli.js')),readFileSync(cli),'IDE server must have the exact current distributed CLI bytes');
  const run=(args)=>execFileSync(process.execPath,[cli,...args],{cwd:temp,encoding:'utf8',env:{...process.env,PATH:''}});
  assert.equal(JSON.parse(run(['init',project])).status,'Initialized');const file=join(project,'invoice.mori'),scenario=join(project,'scenario.json');
  assert.equal(JSON.parse(run(['check',file,'--json'])).status,'AuthoringChecked');
  assert.ok(run(['fmt',file]).includes('agreement Invoice'));const inspection=JSON.parse(run(['inspect',file]));assert.equal(inspection.identities.length,5);assert.equal(inspection.intents[0].terms.gross_cap.atoms,'1010');assert.equal(inspection.intents[0].terms.operation.args.value.atoms,'1000');
  assert.equal(JSON.parse(run(['expand',file,'--action','pay','--scenario',scenario])).status,'Expanded');
  assert.equal(JSON.parse(run(['simulate',file,'--action','pay','--scenario',scenario])).status,'PreparedUnqualified');
  assert.equal(JSON.parse(run(['test',project])).status,'TestsPassed');
  writeFileSync(join(temp,'client.mjs'),`import {check,simulate} from '@moriarty-lang/beta';import{readFileSync}from'node:fs';const s=readFileSync(${JSON.stringify(file)},'utf8'),j=readFileSync(${JSON.stringify(scenario)},'utf8');console.log(JSON.stringify({check:check(s).status,local:simulate(s,'pay',j).status}));`);
  assert.deepEqual(JSON.parse(execFileSync(process.execPath,[join(temp,'client.mjs')],{cwd:temp,encoding:'utf8',env:{...process.env,PATH:''}})),{check:'AuthoringChecked',local:'PreparedUnqualified'});
  writeFileSync(join(temp,'package.json'),'{"type":"module"}');
  writeFileSync(join(temp,'client.ts'),`import {check,simulate,prepareOwnerIntent,verifyAndPrepare,type SignedIntentStatement} from '@moriarty-lang/beta';check('source');const r=simulate('source','action','{}');if(r.status==='PreparedUnqualified'&&'result'in r&&r.result.status==='PreparedUnqualified'){const p:string=r.result.candidate.candidatePost.head;console.log(p);}async function signingClient(){const p=await prepareOwnerIntent('source','pay','{}',{scheme:'schnorr_bip340',publicKeyHex:'key',framing:'raw'},{binaryPath:'/binary'});const terms:SignedIntentStatement=p.statement;const frame:string=p.frame_hex;const network:string=terms.domain.network;const v=await verifyAndPrepare('source','pay','{}','{}',{binaryPath:'/binary'});const valid:boolean=v.signature.signature_valid;const signer:string=v.signature.statement.intent.signer;return {frame,network,valid,signer};}`);
  execFileSync(process.execPath,[join(root,'node_modules/typescript/bin/tsc'),'--noEmit','--strict','--module','NodeNext','--moduleResolution','NodeNext','--target','ES2023',join(temp,'client.ts')],{cwd:temp,encoding:'utf8'});
 }finally{rmSync(temp,{recursive:true,force:true});}
});
