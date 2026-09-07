/** Generate restricted kernels and source-bound metadata, never proving keys. */
import {readFileSync,mkdirSync,writeFileSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {lowerCompact,compactSnapshotHarness} from '../src/lower-compact.ts';
import {canonicalEncode,sha256} from '../src/codec.ts';
const here=fileURLToPath(new URL('.',import.meta.url));
const output=resolve(process.argv[2]??join(here,'generated'));
const bounds=readFileSync(join(here,'../spec/bounds.json'));
const programs=[];
for(const name of ['loan','swap']){
 const source=readFileSync(join(here,`../spec/examples/${name}.mori`)),mapping=lowerCompact(source,bounds),directory=join(output,name);mkdirSync(directory,{recursive:true});
 const files={'kernel.compact':mapping.source,'harness.compact':compactSnapshotHarness(mapping),'metadata.json':canonicalEncode(mapping.metadata),'bound-program.json':canonicalEncode(mapping.bound)};
 const artifacts=[];for(const [file,bytes] of Object.entries(files)){writeFileSync(join(directory,file),bytes);artifacts.push({file,sha256:sha256(bytes),utf8Bytes:Buffer.byteLength(bytes)});}
 programs.push({name,sourceHash:mapping.metadata.sourceHash,programHash:mapping.metadata.programHash,compactSourceHash:mapping.metadata.compactSourceHash,textTable:mapping.metadata.textTable,actions:mapping.metadata.actions.map(a=>({name:a.name,circuit:a.circuit,hints:a.hints.length,effects:a.effects.length})),artifacts});
}
const receipt={schemaVersion:'moriarty-compact-materialization/1',scope:'Restricted Core arithmetic/guard/write/effect-operand kernels and test-only snapshots; no financial settlement, PCD or full compiler correspondence',compiler:'0.31.1',language:'0.23.0',runtime:'0.16.0',programs};
writeFileSync(join(output,'materialization.json'),JSON.stringify(receipt,null,2)+'\n');console.log(JSON.stringify(receipt,null,2));
