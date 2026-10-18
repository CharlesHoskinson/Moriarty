import test from 'node:test';import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';import {mkdtempSync,readFileSync,writeFileSync,existsSync,rmSync} from 'node:fs';import {tmpdir} from 'node:os';import {join} from 'node:path';
const cli=new URL('../src/cli.ts',import.meta.url).pathname;
import {transferSource,repaySource} from './fixtures.mjs';
const run=(args)=>spawnSync(process.execPath,[cli,...args],{encoding:'utf8'});
test('init creates self-contained tested project and refuses overwrites',()=>{
 const parent=mkdtempSync(join(tmpdir(),'mori-init-')),dir=join(parent,'invoice');
 try{let r=run(['init',dir]);assert.equal(r.status,0,r.stderr);assert.ok(existsSync(join(dir,'invoice.mori')));
 const before=readFileSync(join(dir,'invoice.mori'),'utf8');r=run(['init',dir]);assert.equal(r.status,1);assert.equal(readFileSync(join(dir,'invoice.mori'),'utf8'),before);
 r=run(['check',join(dir,'invoice.mori'),'--json']);assert.equal(r.status,0,r.stderr);assert.equal(JSON.parse(r.stdout).status,'AuthoringChecked');
 r=run(['simulate',join(dir,'invoice.mori'),'--action','pay','--scenario',join(dir,'scenario.json')]);assert.equal(r.status,0,r.stderr);assert.equal(JSON.parse(r.stdout).status,'PreparedUnqualified');
 r=run(['test',dir]);assert.equal(r.status,0,r.stderr);assert.equal(JSON.parse(r.stdout).status,'TestsPassed');
 writeFileSync(join(dir,'mori.tests.json'),JSON.stringify({profile:'moriarty-beta-tests/1',cases:[{name:'bad',source:'../secret',action:'pay',scenario:'scenario.json',expect:{status:'PreparedUnqualified'}}]}));
 r=run(['test',dir]);assert.equal(r.status,1);assert.match(r.stdout,/BETA_CASE_PATH/);
 }finally{rmSync(parent,{recursive:true,force:true});}
});
test('CLI rejects unknown flags, unsafe fmt and invalid source',()=>{
 const parent=mkdtempSync(join(tmpdir(),'mori-cli-')),file=join(parent,'bad.mori');
 try{writeFileSync(file,'bad');let r=run(['check',file,'--bogus']);assert.equal(r.status,1);assert.match(r.stderr,/Unknown/);
 r=run(['fmt',file,'--write']);assert.equal(r.status,1);assert.equal(readFileSync(file,'utf8'),'bad');
 r=run(['simulate',file,'--action','pay']);assert.equal(r.status,1);assert.match(r.stderr,/scenario/);
 }finally{rmSync(parent,{recursive:true,force:true});}
});
test('actual CLI inspect exposes transfer and repayment signed scope',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-inspect-')),file=join(dir,'request.mori');
 try{for(const [source,name,operation,atoms] of [[transferSource,'Payment','transfer','1000'],[repaySource,'Repayment','repay','3000']]){
  writeFileSync(file,source);const r=run(['inspect',file]);assert.equal(r.status,0,r.stderr);
  const terms=JSON.parse(r.stdout).intents.find(i=>i.name===name).terms;assert.equal(terms.operation.name,operation);assert.equal(terms.operation.args[operation==='transfer'?'value':'amount'].atoms,atoms);assert.equal(terms.valid.args.to.value,'10');assert.equal(terms.signer.authenticated,false);
 }}finally{rmSync(dir,{recursive:true,force:true});}
});
test('check distinguishes readable output from explicit machine JSON',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-check-output-')),file=join(dir,'request.mori');
 try{
  writeFileSync(file,transferSource);
  let r=run(['check',file]);assert.equal(r.status,0);assert.match(r.stdout,/AuthoringChecked/);assert.match(r.stdout,/pay.*LocalS0/);assert.throws(()=>JSON.parse(r.stdout));
  r=run(['check',file,'--json']);assert.equal(JSON.parse(r.stdout).status,'AuthoringChecked');
  writeFileSync(file,'bad');r=run(['check',file]);assert.equal(r.status,1);assert.match(r.stdout,/BETA_/);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test('malformed UTF8 and BOM never get rewritten or silently reinterpreted',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-encoding-')),file=join(dir,'bad.mori');
 try{for(const bytes of [Buffer.concat([Buffer.from(transferSource+'\n//'),Buffer.from([0xff])]),Buffer.concat([Buffer.from([0xef,0xbb,0xbf]),Buffer.from(transferSource)])]){
  writeFileSync(file,bytes);let r=run(['check',file,'--json']);assert.equal(r.status,1);
  r=run(['fmt',file,'--write']);assert.equal(r.status,1);assert.deepEqual(readFileSync(file),bytes);
 }}finally{rmSync(dir,{recursive:true,force:true});}
});
test('case failures explain the first exact mismatch without dumping containers',()=>{
 const parent=mkdtempSync(join(tmpdir(),'mori-case-diff-')),dir=join(parent,'invoice');
 try{assert.equal(run(['init',dir]).status,0);const path=join(dir,'mori.tests.json'),m=JSON.parse(readFileSync(path,'utf8'));
 m.cases[0].expect.post.balances[0].amount='8991';writeFileSync(path,JSON.stringify(m));
 const r=run(['test',dir]);assert.equal(r.status,1);const c=JSON.parse(r.stdout).cases[0];
 assert.deepEqual(c.mismatches,[{pointer:'/post/balances/0/amount',expected:'8991',actual:'8990'}]);
 }finally{rmSync(parent,{recursive:true,force:true});}
});
test('repayment template has independent complete money expectations',()=>{
 const parent=mkdtempSync(join(tmpdir(),'mori-repay-init-')),dir=join(parent,'repay');
 try{let r=run(['init',dir,'--template','repay']);assert.equal(r.status,0,r.stderr);
 r=run(['test',dir]);assert.equal(r.status,0,r.stderr);assert.equal(JSON.parse(r.stdout).status,'TestsPassed');
 const m=JSON.parse(readFileSync(join(dir,'mori.tests.json'),'utf8'));
 assert.equal(m.cases[0].expect.post.obligations[0].principal,'98000');
 assert.equal(m.cases[0].expect.post.balances[0].amount,'197000');
 }finally{rmSync(parent,{recursive:true,force:true});}
});
