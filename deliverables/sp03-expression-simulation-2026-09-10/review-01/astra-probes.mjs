import {readFileSync,writeFileSync,mkdirSync,rmSync,symlinkSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {spawnSync} from 'node:child_process';
import assert from 'node:assert/strict';
const root=process.cwd(), area=resolve('deliverables/sp03-expression-simulation-2026-09-10/review-01'), dir=join(area,'astra-probe-inputs');mkdirSync(dir);
const prefix=resolve('experiments/moriarty-language'), cli=join(prefix,'src/cli.ts');
const {createExpressionSourceV1}=await import(join(prefix,'src/successor/expression-source-v1.ts'));
const example=join(prefix,'spec/successor/examples/expression-counter'), p='moriarty-expression-source/1';
const schema=example+'.schema.json',source=example+'.mori',snap=example+'.snapshots.json',missing=join(dir,'missing'),cases=[];
const file=(name,value)=>{const f=join(dir,name);writeFileSync(f,value);return f;};
const args=(sc=schema,src=source,sn=snap,pr=p)=>['simulate','--profile',pr,'--schema',sc,'--snapshots',sn,src];
function expect(name,argv,expected,exit=2){const r=spawnSync(process.execPath,[cli,...argv],{encoding:'utf8',timeout:5000});assert.equal(r.error,undefined);assert.equal(r.signal,null);assert.equal(r.status,exit,r.stderr);assert.equal(r.stdout,'');assert.equal(r.stderr,JSON.stringify(expected)+'\n');cases.push({name,exitCode:r.status,stderr:JSON.parse(r.stderr)});}
const fail=(code,input)=>({status:'CliRejected',code,input});
try {
 expect('schema open wins all missing',args(missing,missing,missing),fail('CLI_IO','schema'));
 expect('source read wins missing snapshot even malformed schema',args(file('bad-schema','{}'),missing,missing),fail('CLI_IO','source'));
 expect('snapshot open wins source parse and schema admission',args(file('bad-schema2','{}'),file('bad-source','bad'),missing),fail('CLI_IO','snapshots'));
 expect('source UTF8 wins snapshot open',args(schema,file('bad-utf8',Buffer.from([255])),missing),fail('INVALID_UTF8','source'),1);
 expect('snapshot bound wins UTF8',args(schema,source,file('oversize',Buffer.alloc(2000001,255))),fail('INPUT_BOUND','snapshots'),1);
 expect('profile rejects before nonexistent paths',args(missing,missing,missing,'unsupported'),fail('CLI_PROFILE','arguments'));
 expect('argv shape wins profile validation',[...args(missing,missing,missing,'unsupported'),'extra'],fail('CLI_USAGE','arguments'));
 const fifo=join(dir,'fifo');assert.equal(spawnSync('mkfifo',[fifo]).status,0);
 expect('snapshot FIFO returns promptly',args(schema,source,fifo),fail('CLI_IO','snapshots'));
 const raw=' '.repeat(2000000),bounded=file('exact-bound',raw),api=createExpressionSourceV1(readFileSync(schema,'utf8'));
 expect('exact 2M reaches API canonical rejection',args(schema,source,bounded),api.evaluate(readFileSync(source,'utf8'),raw),1);
 const link=join(dir,'snapshot-link');symlinkSync(snap,link);const r=spawnSync(process.execPath,[cli,...args(schema,source,link)],{encoding:'utf8',timeout:5000});assert.equal(r.status,0,r.stderr);assert.equal(r.stderr,'');const out=JSON.parse(r.stdout);assert.deepEqual(out.pre,{counter:'10',funds:'100'});assert.equal(out.result.post.counter,'12');cases.push({name:'regular snapshot symlink succeeds',exitCode:r.status});
 console.log(JSON.stringify({passed:true,cases},null,2));
} finally {rmSync(dir,{recursive:true,force:true});}
