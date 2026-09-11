import assert from 'node:assert/strict';
import {mkdtempSync,readFileSync,writeFileSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {spawnSync} from 'node:child_process';
const root=process.argv[2], prefix=join(root,'experiments/moriarty-language');
const {canonical}=await import(join(prefix,'src/successor/expression-wire-v1.ts'));
const {createExpressionSourceV1}=await import(join(prefix,'src/successor/expression-source-v1.ts'));
const {createFinancialExpressionSourceV1}=await import(join(prefix,'src/successor/financial-expression-source-v1.ts'));
const directory=mkdtempSync(join(tmpdir(),'moriarty-sp03-root-'));let serial=0;const cases=[];
const file=text=>{const p=join(directory,String(serial++));writeFileSync(p,text);return p;};
const invoke=(profile,schema,source,snapshots)=>{
 const files=[file(schema),file(source),file(snapshots)];
 const args=[join(prefix,'src/cli.ts'),'simulate','--profile',profile,'--schema',files[0],'--snapshots',files[2],files[1]];
 const result=spawnSync(process.execPath,args,{encoding:'utf8',timeout:10000,maxBuffer:2_000_000});
 assert.equal(result.error,undefined);assert.equal(result.signal,null);
 for(let i=0;i<3;i++)assert.equal(readFileSync(files[i],'utf8'),[schema,source,snapshots][i]);
 return result;
};
try{
 const fp='moriarty-financial-expression-source/1',op='moriarty-expression-source/1';
 const example=join(prefix,'spec/successor/examples/financial-vault-quote');
 const source=readFileSync(example+'.mori','utf8'),schema=readFileSync(example+'.schema.json','utf8'),snaps=JSON.parse(readFileSync(example+'.snapshots.json','utf8'));
 const success=invoke(fp,schema,source,canonical(snaps));assert.equal(success.status,0,success.stderr);assert.equal(success.stderr,'');
 const output=JSON.parse(success.stdout);
 assert.equal(output.judgmentResult,'SourceSimulated');assert.equal(output.sourceProfile,fp);assert.deepEqual(output.pre,{last:'0'});assert.equal(output.initialWork,'1000');
 assert.deepEqual(output.result,{status:'ExpressionPrepared',post:{last:'1'},descriptors:[{operation:'Notice',fields:{allocation:'1',offered:'4'}}],workRemaining:'959'});cases.push('financial exact complete source simulation');
 for(const [label,body,snapshot,code,work] of [
  ['selected None',source,{...snaps,Args:{...snaps.Args,useSupplied:true}},'OPTION_NONE','19'],
  ['rollback after write and emit',source.replace('ensures post.last == selected;','ensures false;'),snaps,'ENSURES_FAILED','39'],
  ['zero work',source,{...snaps,workInitial:'0'},'WORK_EXHAUSTED','0']]){
  const raw=canonical(snapshot);const reference=createFinancialExpressionSourceV1(schema).evaluate(body,raw);
  const r=invoke(fp,schema,body,raw);assert.equal(r.status,1);assert.equal(r.stdout,'');const actual=JSON.parse(r.stderr);
  assert.deepEqual(actual,reference);assert.equal(actual.status,'Rejected');assert.equal(actual.code,code);assert.equal(actual.workUsed,work);assert.equal('post' in actual,false);assert.equal('descriptors' in actual,false);cases.push(label);
 }
 const type=['Collection',['Text'],'40'];const bigSchema={units:[],assets:[],vaults:[],parties:[],recordTypes:{},enumTypes:{},fields:{payload:{type,writeClass:'ordinary'}},args:{incoming:type},observations:{},operations:{}};
 const bigSource=`profile "${op}"; agreement Example { action step(incoming: Collection<Text,40>) { next.payload=incoming; } }`;
 const big={Pre:{payload:Array(40).fill('x'.repeat(1024))},Args:{incoming:Array(40).fill('y'.repeat(1024))},Obs:{},workInitial:'20'};
 assert.ok(Buffer.byteLength(canonical(big))>65536);
 const b=invoke(op,canonical(bigSchema),bigSource,canonical(big));assert.equal(b.status,0,b.stderr);const bo=JSON.parse(b.stdout);
 assert.deepEqual(bo.pre,big.Pre);assert.deepEqual(bo.result.post,{payload:big.Args.incoming});assert.equal(bo.result.workRemaining,'18');
 cases.push('valid aggregate snapshot above65536');
 for(const [label,schemaText,sourceText,snapshotText,code] of [
  ['snapshot before parse',schema,'invalid source','{}','INPUT_SCHEMA'],
  ['BOM snapshot',schema,source,'\uFEFF'+canonical(snaps),'INPUT_SCHEMA'],
  ['snapshot numeric value',schema,source,JSON.stringify({...snaps,workInitial:1000}),'INPUT_SCHEMA']]){
  const api=createFinancialExpressionSourceV1(schemaText).evaluate(sourceText,snapshotText);
  const r=invoke(fp,schemaText,sourceText,snapshotText);assert.equal(r.status,1);assert.equal(r.stdout,'');assert.deepEqual(JSON.parse(r.stderr),api);assert.equal(api.code,code);cases.push(label);
 }
 console.log(JSON.stringify({passed:true,cases},null,2));
}finally{rmSync(directory,{recursive:true,force:true});}
