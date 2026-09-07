/** Full source-derived records only. This runner makes no evaluation/proof claim. */
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { compile, canonicalEncode, hashDomain, sha256 } from './frontend.ts';
const languageRoot=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const repoRoot=resolve(languageRoot,'../..');
const outputRoot=process.argv[2]??resolve(repoRoot,'evidence/moriarty-completion-program-2026-09-07/MC01/profile-04/materialized');
const bounds=readFileSync(resolve(languageRoot,'spec/bounds.json'));
const summaries=[];
for(const name of ['loan','swap']) {
  const sourcePath=resolve(languageRoot,`spec/examples/${name}.mori`);
  const sourceBytes=readFileSync(sourcePath);
  const result=compile(sourceBytes,bounds);
  const directory=resolve(outputRoot,name);mkdirSync(directory,{recursive:true});
  const records={'source-ast':result.source,'typed-program':result.typed,'bound-program':result.bound,'semantic-manifest':result.bound.manifest};
  const artifacts=[];
  for(const [kind,value] of Object.entries(records)){const encoded=canonicalEncode(value);const path=resolve(directory,`${kind}.json`);writeFileSync(path,encoded);artifacts.push({kind,path:path.slice(repoRoot.length+1),sha256:sha256(encoded),utf8Bytes:new TextEncoder().encode(encoded).length});}
  const summary={name,sourcePath:sourcePath.slice(repoRoot.length+1),sourceHash:result.source.sourceHash,sourceUtf8Bytes:Number(result.source.sourceUtf8Bytes),programHash:result.bound.programHash,boundsHash:result.bound.manifest.bounds.boundsHash,claimRoot:hashDomain('MORIARTY-CLAIMS-bounded-atomic/1',result.bound.manifest.requiredClaims),annotations:result.typed.annotations.length,actionResources:result.bound.manifest.core.actions.map(a=>({action:a.name,...a.resourceCounts})),metrics:result.metrics,artifacts};
  writeFileSync(resolve(directory,'metrics.json'),JSON.stringify(summary,null,2)+'\n');summaries.push(summary);
}
const vectorDirectory=resolve(outputRoot,'vectors');mkdirSync(vectorDirectory,{recursive:true});
const vectorSource=`  agreement SpanVector profile "moriarty-bounded-atomic/1" {
  lifetime 2;
  horizon 2000000000;
  state closed: UInt128 = uint(0);
  observation now: UInt128;
  status episode closed_when closed == uint(1);
  status agreement no_remaining_notional;
  action run(actor: Text) {
    guard text("é😀") == text("é😀"), "scalar";
    let nested = ((uint(1)));
    let divided = floor_div((uint(8) + uint(2)), ((uint(3))));
  }
}  \n`;
const vectorResult=compile(vectorSource,bounds);
writeFileSync(resolve(vectorDirectory,'nested-parentheses.mori'),vectorSource);
writeFileSync(resolve(vectorDirectory,'source-ast.json'),canonicalEncode(vectorResult.source));
writeFileSync(resolve(vectorDirectory,'typed-program.json'),canonicalEncode(vectorResult.typed));
writeFileSync(resolve(vectorDirectory,'bound-program.json'),canonicalEncode(vectorResult.bound));
const vectorAction=vectorResult.source.declarations.find(d=>d.tag==='ActionDecl');
if(!vectorAction||vectorAction.tag!=='ActionDecl')throw new Error('vector action missing');
const sourceSlice=(span:{startByte:string;endByte:string})=>new TextDecoder().decode(new TextEncoder().encode(vectorSource).slice(Number(span.startByte),Number(span.endByte)));
const nested=vectorAction.statements[1];
if(nested.tag!=='Let'||nested.expression.tag!=='Literal')throw new Error('vector nested expression missing');
const vectorReceipt={sourceHash:vectorResult.source.sourceHash,programHash:vectorResult.bound.programHash,sourceUtf8Bytes:vectorResult.source.sourceUtf8Bytes,rootSpan:vectorResult.source.span,nestedExpression:{span:nested.expression.span,sourceSlice:sourceSlice(nested.expression.span)},nestedLiteral:{span:nested.expression.literal.span,sourceSlice:sourceSlice(nested.expression.literal.span)},annotations:vectorResult.typed.annotations,metrics:vectorResult.metrics,canonicalVectors:[{input:{z:'é😀\u0000\n',a:true},canonical:canonicalEncode({z:'é😀\u0000\n',a:true})}]};
writeFileSync(resolve(vectorDirectory,'vectors.json'),JSON.stringify(vectorReceipt,null,2)+'\n');
const receipt={schemaVersion:'moriarty-frontend-materialization/1',status:'experimental-source-derived-materialization',scope:'Complete SourceAST, TypedProgram, Core and BoundProgram from each exact source; no parser/evaluator correspondence, MC01 admission, native proof, PCD or protocol conformance claim.',runtime:process.version,command:'node experiments/moriarty-language/src/materialize.ts',nodeCounting:'Every decoded JSON value counts once; object keys are not nodes. Keys are included in text-size maxima. Root depth is zero.',limitations:['Frontend diagnostics have not yet been admitted against the closed staged diagnostic protocol.','The canonical value codec requires a caller-supplied closed-schema validator when receiving arbitrary wire records; compile itself only accepts source bytes and constructs closed typed records.','No evaluation, proof, native backend, compiler correspondence, ACTUS or DeFi conformance is established by these records.'],programs:summaries,vectors:{path:resolve(vectorDirectory,'vectors.json').slice(repoRoot.length+1),sourceHash:vectorResult.source.sourceHash,programHash:vectorResult.bound.programHash}};
writeFileSync(resolve(outputRoot,'receipt.json'),JSON.stringify(receipt,null,2)+'\n');
console.log(JSON.stringify(receipt,null,2));
