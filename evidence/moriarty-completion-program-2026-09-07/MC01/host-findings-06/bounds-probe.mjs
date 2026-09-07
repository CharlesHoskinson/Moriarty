import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const root=process.argv[2];
const {compile,parse,check,elaborate}=await import(pathToFileURL(root+'/experiments/moriarty-language/src/frontend.ts'));
const source=readFileSync(root+'/experiments/moriarty-language/spec/examples/loan.moriarty');
const bounds=JSON.parse(readFileSync(root+'/experiments/moriarty-language/spec/bounds.json'));
bounds.domainRegistry={invalid:'unregistered'};
const results=[];
try {const c=compile(source,JSON.stringify(bounds));results.push({probe:'changed-domain-registry',accepted:true,programHash:c.bound.programHash,boundsHash:c.bound.manifest.bounds.boundsHash});}catch(e){results.push({probe:'changed-domain-registry',accepted:false,code:e.code});}
for(const [name,api] of [['parse',parse],['check',check],['elaborate',elaborate]])try {const r=api(source,JSON.stringify({schemaVersion:'moriarty-bounds/1',semanticProfile:'moriarty-bounded-atomic/1'}));results.push({probe:'incomplete-bounds',api:name,result:r.code});}catch(e){results.push({probe:'incomplete-bounds',api:name,thrown:e.constructor.name,message:e.message});}
console.log(JSON.stringify(results,null,2));
