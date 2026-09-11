import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,readFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawnSync} from 'node:child_process';
const root=process.argv[2];
const cli=join(root,'experiments/moriarty-language/src/cli.ts');
const dir=mkdtempSync(join(tmpdir(),'moriarty-bom-repro-'));
const results=[];
try {
 const bytes=Uint8Array.from([0xef,0xbb,0xbf,0x78]);
 const withTrue=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(bytes);
 const withFalse=new TextDecoder('utf-8',{fatal:true,ignoreBOM:false}).decode(bytes);
 assert.equal(withTrue,'\uFEFFx'); assert.equal(withFalse,'x');
 results.push({case:'Node TextDecoder direct',node:process.version,ignoreBOMtrue:Array.from(withTrue,c=>c.codePointAt(0)),ignoreBOMfalse:Array.from(withFalse,c=>c.codePointAt(0))});
 for(const financial of [false,true]){
  const profile=financial?'moriarty-financial-expression-source/1':'moriarty-expression-source/1';
  const source=`profile "${profile}"; agreement Demo { action step() { requires true; } }`;
  const schema={units:[],assets:[],vaults:[],parties:[],recordTypes:{},enumTypes:{},fields:{},args:{},observations:{},operations:{}};
  if(financial)schema.variantTypes={};
  const {canonical}=await import(join(root,'experiments/moriarty-language/src/successor/expression-wire-v1.ts'));
  const factory=financial?(await import(join(root,'experiments/moriarty-language/src/successor/financial-expression-source-v1.ts'))).createFinancialExpressionSourceV1:(await import(join(root,'experiments/moriarty-language/src/successor/expression-source-v1.ts'))).createExpressionSourceV1;
  const rawSchema=canonical(schema);
  for(const bomInput of ['source','schema']){
   const sourceText=(bomInput==='source'?'\uFEFF':'')+source;
   const schemaText=(bomInput==='schema'?'\uFEFF':'')+rawSchema;
   const sf=join(dir,'source.mori'),jf=join(dir,'schema.json');
   writeFileSync(sf,sourceText);writeFileSync(jf,schemaText);
   const expected=factory(schemaText).check(sourceText);
   const r=spawnSync(process.execPath,[cli,'check','--profile',profile,'--schema',jf,sf],{encoding:'utf8',timeout:5000});
   assert.equal(r.status,1);assert.equal(r.stdout,'');assert.deepEqual(JSON.parse(r.stderr),expected);
   assert.equal(expected.code,bomInput==='source'?'UNEXPECTED_CHAR':'INPUT_SCHEMA');
   assert.equal(readFileSync(sf,'utf8'),sourceText);assert.equal(readFileSync(jf,'utf8'),schemaText);
   results.push({case:profile+' BOM '+bomInput,exit:r.status,result:expected});
   if(bomInput==='source'){
    const f=spawnSync(process.execPath,[cli,'format','--profile',profile,sf],{encoding:'utf8',timeout:5000});
    assert.equal(f.status,1);assert.equal(f.stdout,'');assert.equal(JSON.parse(f.stderr).code,'UNEXPECTED_CHAR');
    results.push({case:profile+' BOM format',exit:f.status,result:JSON.parse(f.stderr)});
   }
  }
 }
 console.log(JSON.stringify({passed:true,conclusion:'ignoreBOM:true retains U+FEFF; false strips it. Current CLI matches API for BOM source/schema in both profiles. Proposed change would violate its retention contract.',results},null,2));
} finally {rmSync(dir,{recursive:true,force:true});}
