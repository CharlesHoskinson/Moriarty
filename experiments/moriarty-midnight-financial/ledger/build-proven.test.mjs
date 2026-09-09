import {test} from 'node:test';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import {mkdtemp, mkdir, writeFile, readFile, symlink, rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {writeFileSync} from 'node:fs';
import {buildProven, inspectBuildSources, sourceManifestHash} from './build-proven.mjs';

async function fixture(t, kind = 'loan') {
  const dir = await mkdtemp(join(tmpdir(), 'moriarty-build-source-test-'));
  t.after(() => rm(dir, {recursive:true, force:true}));
  const sourceManifest = inspectBuildSources();
  const outputDir = join(dir, 'fresh');
  const calls = [];
  const options = {
    case:kind, outputDir, sourceManifest,
    admission:{id:'test-only-admission',resourceId:'test-only-resource',candidateHash:sourceManifestHash(sourceManifest),case:kind,outputDir,allowFullBuild:true,reviewed:true,maxCompileMs:1000,deadlineMs:Date.now()+30000,maxAttempts:1},
    resourceCounters:{resourceId:'test-only-resource',attempts:0,maxAttempts:1},
    commandAdapter: async (argv, context) => {
      calls.push({argv,context});
      const flags = argv.slice(1).join(' ');
      const versions = {'--version':'compact 0.5.2','compile --version':'0.31.1','compile --language-version':'0.23.0','compile --runtime-version':'0.16.0'};
      if (versions[flags]) return {status:0,stdout:versions[flags],stderr:''};
      assert.equal(argv[1], 'compile');
      assert.equal(argv.includes('--skip-zk'),false);
      const assets = argv.at(-1);
      const names = kind==='loan'?['initialize','accrue','settle']:['initialize','swap','close'];
      for (const sub of ['contract','compiler','keys','zkir']) await mkdir(join(assets,sub),{recursive:true});
      await writeFile(join(assets,'contract/index.js'),'// inert test artifact, not a real contract\n');
      await writeFile(join(assets,'compiler/contract-info.json'),JSON.stringify({'compiler-version':'0.31.1','language-version':'0.23.0','runtime-version':'0.16.0',circuits:names.map(name=>({name,pure:false,proof:true}))}));
      for(const name of names) for(const ext of ['prover','verifier','zkir','bzkir']) await writeFile(join(assets,['prover','verifier'].includes(ext)?'keys':'zkir',name+'.'+ext),'inert-'+name+'-'+ext);
      return {status:0,stdout:'inert compilation only',stderr:''};
    },
  };
  const hash=b=>createHash('sha256').update(b).digest('hex');
  const record={id:options.admission.resourceId,sourceCandidateSha:options.admission.candidateHash,case:kind,outputDir,maxCompileMs:1000,deadlineMs:options.admission.deadlineMs,maxAttempts:1,attemptFile:join(dir,'resource-attempt.json')};
  const rp=join(dir,'resource.json');const rb=JSON.stringify(record);await writeFile(rp,rb);
  const reviewPath=join(dir,'review.json');const reviewBytes=JSON.stringify({sourceCandidateSha:options.admission.candidateHash,verdict:'APPROVED',scope:'inert source-test receipt only'});await writeFile(reviewPath,reviewBytes);
  options.admission.resourceRecord={path:rp,sha256:hash(rb)};
  options.admission.reviews=[{path:reviewPath,sha256:hash(reviewBytes)}];
  return {dir, options, calls};
}

test('loan complete inert artifacts produce inspected hashes, never a proven build claim',async t=>{
  const {options,calls}=await fixture(t);
  const result=await buildProven(options);
  assert.equal(result.status,'source-test-only'); assert.equal(result.proven,false);
  assert.equal(result.inspectedProofAssets,false); assert.equal(result.proofsGenerated,false);
  assert.match(result.proofScope,/compiler key artifacts only; no transaction proof/);
  assert.equal(result.case,'loan'); assert.equal(options.resourceCounters.attempts,1);
  assert.equal(calls.filter(x=>x.argv.includes('--compact-path')).length,1);
  assert.equal(result.artifacts.length,14); assert(result.artifacts.every(x=>/^[a-f0-9]{64}$/.test(x.sha256)));
  assert.equal(result.contractModulePath,join(options.outputDir,'loan/contract/index.js'));
  const receipt=JSON.parse(await readFile(join(options.outputDir,'build-receipt.json')));
  assert.equal(receipt.sourceManifestHash,options.admission.candidateHash);
  assert.equal(receipt.commands.length,5);
});
test('swap chooses exact exported circuits',async t=>{
 const {options}=await fixture(t,'swap'); const r=await buildProven(options);
 assert(r.artifacts.some(x=>x.path==='keys/close.verifier'));
 assert(!r.artifacts.some(x=>x.path.includes('settle')));
});
for(const [label, mutate] of [
 ['absent admission',o=>{delete o.admission;}],
 ['wrong case',o=>{o.case='other';}],
 ['wrong candidate',o=>{o.admission.candidateHash='0'.repeat(64);}],
 ['wrong resource',o=>{o.resourceCounters.resourceId='other';}],
 ['consumed allowance',o=>{o.resourceCounters.attempts=1;}],
 ['nonfinite timeout',o=>{o.admission.maxCompileMs=Infinity;}],
 ['expired deadline',o=>{o.admission.deadlineMs=1;}],
 ['missing source pin',o=>{o.sourceManifest.files.pop();o.admission.candidateHash=sourceManifestHash(o.sourceManifest);}],
 ['stale source pin',o=>{o.sourceManifest.files[0].sha256='0'.repeat(64);o.admission.candidateHash=sourceManifestHash(o.sourceManifest);}],
 ['different output',o=>{o.admission.outputDir+='_other';}],
]) test(label+' rejects before compiler and allocation',async t=>{
 const {options,calls}=await fixture(t);mutate(options);await assert.rejects(buildProven(options));assert.equal(calls.length,0);
});
test('existing output is preserved and rejects',async t=>{
 const {options,calls}=await fixture(t);await mkdir(options.outputDir);await writeFile(join(options.outputDir,'keep'),'keep');
 await assert.rejects(buildProven(options),/exist/);assert.equal(calls.length,0);assert.equal(await readFile(join(options.outputDir,'keep'),'utf8'),'keep');
});
test('symlink ancestor rejects before compiler',async t=>{
 const {options,dir,calls}=await fixture(t);await symlink(dir,join(dir,'link')); options.outputDir=join(dir,'link/fresh');options.admission.outputDir=options.outputDir;
 await assert.rejects(buildProven(options),/symlink/);assert.equal(calls.length,0);
});
test('compiler failure preserves charged attempt, partial outputs and diagnostic receipt; no retry',async t=>{
 const {options,calls}=await fixture(t); const adapter=options.commandAdapter;
 options.commandAdapter=async(a,c)=>{if(a.includes('--compact-path')){calls.push({argv:a});await mkdir(a.at(-1));await writeFile(join(a.at(-1),'partial'),'partial');return {status:2,stdout:'',stderr:'inert failure'};}return adapter(a,c);};
 await assert.rejects(buildProven(options),/exit/);assert.equal(options.resourceCounters.attempts,1);
 assert.equal(await readFile(join(options.outputDir,'loan/partial'),'utf8'),'partial');
 assert.equal(JSON.parse(await readFile(join(options.outputDir,'build-receipt.json'))).status,'failed');
 assert.equal(calls.filter(x=>x.argv.includes('--compact-path')).length,1);
});
test('missing proving key rejects despite compiler success',async t=>{
 const {options}=await fixture(t);const adapter=options.commandAdapter;
 options.commandAdapter=async(a,c)=>{const r=await adapter(a,c);if(a.includes('--compact-path'))await rm(join(a.at(-1),'keys/settle.prover'));return r;};
 await assert.rejects(buildProven(options),/missing.*settle.prover/);
});
test('wrong metadata version rejects despite compiler success',async t=>{
 const {options}=await fixture(t);const adapter=options.commandAdapter;
 options.commandAdapter=async(a,c)=>{const r=await adapter(a,c);if(a.includes('--compact-path')){const p=join(a.at(-1),'compiler/contract-info.json');const d=JSON.parse(await readFile(p));d['runtime-version']='other';await writeFile(p,JSON.stringify(d));}return r;};
 await assert.rejects(buildProven(options),/metadata/);
});
test('generated output symlink is rejected, never hashed through',async t=>{
 const {options,dir}=await fixture(t);const adapter=options.commandAdapter;await writeFile(join(dir,'outside'),'outside');
 options.commandAdapter=async(a,c)=>{const r=await adapter(a,c);if(a.includes('--compact-path'))await symlink(join(dir,'outside'),join(a.at(-1),'escape'));return r;};
 await assert.rejects(buildProven(options),/symlink/);
});

test('consumed on-disk attempt refuses reset counters on later invocation',async t=>{
 const {options,calls,dir}=await fixture(t);await buildProven(options);options.resourceCounters.attempts=0;
 await rm(options.outputDir,{recursive:true});const before=calls.length;
 await assert.rejects(buildProven(options),/attempt|exist/);assert.equal(calls.length,before);
 assert.equal(JSON.parse(await readFile(join(dir,'resource-attempt.json'))).resourceId,'test-only-resource');
});
test('review digest and resource bytes must match immutable admission bindings',async t=>{
 const {options,calls}=await fixture(t);await writeFile(options.admission.reviews[0].path,'{}');
 await assert.rejects(buildProven(options),/review|digest/);assert.equal(calls.length,0);
});

test('source commitment ignores object insertion order, retaining the exact ordered source list',()=>{
 const manifest=inspectBuildSources();
 const reordered={files:manifest.files.map(f=>({sha256:f.sha256,path:f.path})),schema:manifest.schema};
 assert.equal(sourceManifestHash(reordered),sourceManifestHash(manifest));
 assert.notEqual(sourceManifestHash({...manifest,files:[...manifest.files].reverse()}),sourceManifestHash(manifest));
});
test('attempt path equal to output rejects before consuming the allowance',async t=>{
 const {options,calls}=await fixture(t);
 const ref=options.admission.resourceRecord,record=JSON.parse(await readFile(ref.path));
 record.attemptFile=options.outputDir;
 const bytes=JSON.stringify(record);await writeFile(ref.path,bytes);
 ref.sha256=createHash('sha256').update(bytes).digest('hex');
 await assert.rejects(buildProven(options),/attempt must persist outside/);
 assert.equal(options.resourceCounters.attempts,0);assert.equal(calls.length,0);
 await assert.rejects(readFile(options.outputDir),{code:'ENOENT'});
});
test('post-charge output creation failure retains a diagnostic outside output',async t=>{
 const {options,calls,dir}=await fixture(t);let attempts=0;
 Object.defineProperty(options.resourceCounters,'attempts',{get:()=>attempts,set:value=>{
   attempts=value;writeFileSync(options.outputDir,'raced output, preserve it');
 }});
 await assert.rejects(buildProven(options),error=>{
   assert.match(error.message,/EEXIST/);assert.match(error.message,/receipt persistence failed/);return true;
 });
 assert.equal(attempts,1);assert.equal(calls.length,0);
 const receipt=JSON.parse(await readFile(join(dir,'resource-attempt.json.failure.json')));
 assert.equal(receipt.status,'failed');assert.match(receipt.error,/EEXIST/);
 assert.match(receipt.receiptWriteError,/not a directory|ENOTDIR/);
 assert.equal(await readFile(options.outputDir,'utf8'),'raced output, preserve it');
});
test('receipt write failure preserves the original compiler failure and outside diagnostic',async t=>{
 const {options,dir}=await fixture(t),adapter=options.commandAdapter;
 options.commandAdapter=async(a,c)=>{
  if(!a.includes('--compact-path'))return adapter(a,c);
  await mkdir(join(options.outputDir,'build-receipt.json'));
  return {status:23,stderr:'original compiler failure'};
 };
 await assert.rejects(buildProven(options),error=>{
  assert.match(error.message,/original compiler failure/);assert.match(error.message,/receipt persistence failed/);return true;
 });
 const receipt=JSON.parse(await readFile(join(dir,'resource-attempt.json.failure.json')));
 assert.equal(receipt.status,'failed');assert.match(receipt.error,/original compiler failure/);
 assert.match(receipt.receiptWriteError,/EEXIST/);
});
