import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync, mkdirSync, writeFileSync, readFileSync, readdirSync, lstatSync, rmSync, symlinkSync} from 'node:fs';
import {join, relative} from 'node:path';
import {tmpdir} from 'node:os';
import {createHash} from 'node:crypto';
import {inspectBuildSources, sourceManifestHash} from './build-proven.mjs';
import {inspectFinancialBuild, loadProvenFinancialContract} from './proven-assets.mjs';
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
function fixture(t, kind = 'loan') {
  const dir = mkdtempSync(join(tmpdir(), 'sp05-assets-source-')); t.after(() => rmSync(dir, {recursive: true, force: true}));
  const assets = join(dir, kind), names = kind === 'loan' ? ['initialize','accrue','settle'] : ['initialize','swap','close'];
  for (const sub of ['compiler','contract','keys','zkir']) mkdirSync(join(assets, sub), {recursive: true});
  writeFileSync(join(assets, 'contract/index.js'), "import * as runtime from '@midnight-ntwrk/compact-runtime';\nexport class Contract {}\nexport function ledger(state) { return state; }\n");
  writeFileSync(join(assets, 'compiler/contract-info.json'), JSON.stringify({'compiler-version':'0.31.1','language-version':'0.23.0','runtime-version':'0.16.0',circuits:names.map(name=>({name,pure:false,proof:true}))}));
  for (const name of names) for (const ext of ['prover','verifier','zkir','bzkir']) writeFileSync(join(assets, ['prover','verifier'].includes(ext)?'keys':'zkir',name+'.'+ext), 'INERT SOURCE TEST '+name+'.'+ext);
  const sourceHash = sourceManifestHash(inspectBuildSources());
  const resourceRecord = {path: join(dir,'resource.json'), sha256:hash('{}')};writeFileSync(resourceRecord.path,'{}');
  const attemptFile=join(dir,'attempt.json');
  writeFileSync(attemptFile, JSON.stringify({schema:'moriarty.financial-build-attempt/1',resourceId:'inert-resource',admissionId:'inert-admission',sourceCandidateSha:sourceHash,case:kind,outputDir:dir,attempts:1,synthetic:true}));
  const compact='/home/charl/.local/bin/compact', stage=join(dir,'stage');
  const commands = [['--version'],['compile','--version'],['compile','--language-version'],['compile','--runtime-version'],['compile','--compact-path',stage,join(stage,kind+'.compact'),assets]].map((args,index)=>{
    const stdout=['compact 0.5.2','0.31.1','0.23.0','0.16.0','inert compiler output'][index];
    return {argv:[compact,...args],timeoutMs:1000,exit:0,signal:null,error:null,stdout,stderr:'',stdoutSha256:hash(stdout),stderrSha256:hash('')};
  });
  const receipt = {schema:'moriarty.financial-proven-assets/1',status:'source-test-only',case:kind,assetsPath:assets,contractModulePath:join(assets,'contract/index.js'),proven:false,inspectedProofAssets:false,proofsGenerated:false,synthetic:true,sourceManifestHash:sourceHash,admissionId:'inert-admission',resourceId:'inert-resource',attemptFile,resourceRecord,reviews:[],versions:{compactWrapper:'compact 0.5.2',compiler:'0.31.1',language:'0.23.0',runtime:'0.16.0'},commands,artifacts:[],finishedAt:new Date().toISOString()};
  const options={case:kind,receiptPath:join(dir,'build-receipt.json'),receiptSha256:'',sourceManifestHash:sourceHash};
  function save({rehash=true}={}) {
    if (rehash) { receipt.artifacts=[]; const walk=d=>{for(const name of readdirSync(d).sort()){const file=join(d,name),stat=lstatSync(file);if(stat.isDirectory())walk(file);else if(!stat.isSymbolicLink())receipt.artifacts.push({path:relative(assets,file),bytes:stat.size,sha256:hash(readFileSync(file))});}};walk(assets); }
    const bytes=JSON.stringify(receipt);writeFileSync(options.receiptPath,bytes);options.receiptSha256=hash(bytes);
  }
  save(); return {dir,assets,receipt,options,save};
}
for (const kind of ['loan','swap']) test(kind+' synthetic layout and inert constructor composition remain nonexecutable', async t=>{
  const f=fixture(t,kind), calls=[];
  const report=await inspectFinancialBuild({...f.options,sourceTestOnly:true,moduleAdapter:async path=>{
    calls.push(['import-inert',path]);return {generatedModule:{Contract:class {},ledger:s=>s},CompiledContract:{make(tag,ctor){calls.push(['make',tag,typeof ctor]);return {pipe(...fns){return fns.reduce((s,f)=>f(s),{});}}},withVacantWitnesses:s=>{calls.push(['vacant']);return s;},withCompiledFileAssets:path=>s=>{calls.push(['assets',path]);return s;}}};
  }});
  assert.equal(report.executable,false);assert.equal(report.status,'source-test-only');
  assert.equal('compiledContract' in report,false);assert.equal('decodeState' in report,false);
  assert.equal(calls[0][1],f.receipt.contractModulePath);assert.deepEqual(calls.at(-1),['assets',f.assets]);
  await assert.rejects(loadProvenFinancialContract(f.options),/synthetic|genuine/);
});
test('synthetic receipts cannot use executable loader adapters or flipped proven flags',async t=>{
  const f=fixture(t);let calls=0;
  await assert.rejects(loadProvenFinancialContract({...f.options,moduleAdapter:()=>{calls++;}}),/adapter|synthetic|genuine/);assert.equal(calls,0);
  Object.assign(f.receipt,{synthetic:false,status:'built',proven:true,inspectedProofAssets:true});f.save();
  await assert.rejects(loadProvenFinancialContract(f.options),/attempt.*synthetic|genuine/);
});
for(const [label,change] of [
 ['receipt binding',f=>{f.options.receiptSha256='0'.repeat(64);}],
 ['stale sources',f=>{f.options.sourceManifestHash='0'.repeat(64);}],
 ['wrong case',f=>{f.options.case='swap';}],
 ['skip-zk command',f=>{f.receipt.commands.at(-1).argv.splice(2,0,'--skip-zk');f.save();}],
 ['missing key',f=>{rmSync(join(f.assets,'keys/initialize.prover'));f.save();}],
 ['wrong metadata',f=>{const p=join(f.assets,'compiler/contract-info.json');const m=JSON.parse(readFileSync(p));m['runtime-version']='0.15.0';writeFileSync(p,JSON.stringify(m));f.save();}],
 ['missing circuit',f=>{const p=join(f.assets,'compiler/contract-info.json');const m=JSON.parse(readFileSync(p));m.circuits.pop();writeFileSync(p,JSON.stringify(m));f.save();}],
 ['unlisted asset',f=>{writeFileSync(join(f.assets,'contract/unlisted.js'),'export const bad=1;');}],
 ['mutated asset',f=>{writeFileSync(join(f.assets,'keys/settle.prover'),'CHANGED');}],
 ['unsafe path',f=>{f.receipt.artifacts[0].path='../escape';f.save({rehash:false});}],
 ['symlink asset',f=>{symlinkSync(f.options.receiptPath,join(f.assets,'contract/link.js'));}],
 ['new package import',f=>{writeFileSync(join(f.assets,'contract/index.js'),"import fs from 'node:fs'; export class Contract {} export function ledger(s){return s}");f.save();}],
 ['lazy import',f=>{writeFileSync(join(f.assets,'contract/index.js'),"export async function ledger(){return import('@midnight-ntwrk/compact-runtime')} export class Contract{}");f.save();}],
]) test(label+' rejects before module adapter',async t=>{
 const f=fixture(t);change(f);let calls=0;
 await assert.rejects(inspectFinancialBuild({...f.options,sourceTestOnly:true,moduleAdapter:()=>{calls++;}}));assert.equal(calls,0);
});
test('source adapter exercises actual pinned protocol CompiledContract constructors without executable result',async t=>{
  const f=fixture(t);
  const {CompiledContract}=await import('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-protocol/dist/compact-js.mjs');
  const report=await inspectFinancialBuild({...f.options,sourceTestOnly:true,moduleAdapter:async()=>({CompiledContract,generatedModule:{Contract:class InertContract{},ledger:state=>state}})});
  assert.equal(report.executable,false);assert.equal(report.artifactCount,14);
  assert.equal('compiledContract' in report,false);
});
test('module adapter mutation is caught by complete post-composition reinspection',async t=>{
  const f=fixture(t);
  const {CompiledContract}=await import('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/midnight-js-protocol/dist/compact-js.mjs');
  await assert.rejects(inspectFinancialBuild({...f.options,sourceTestOnly:true,moduleAdapter:async()=>{
    writeFileSync(join(f.assets,'zkir/settle.bzkir'),'mutated-after-check');
    return {CompiledContract,generatedModule:{Contract:class InertContract{},ledger:state=>state}};
  }}),/asset bytes/);
});
