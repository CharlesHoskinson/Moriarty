#!/usr/bin/env node
/** One admitted full Compact build. Import/manifest inspection is inert.
 * Review digests establish byte/candidate bindings, not auditor independence or votes.
 * commandAdapter is explicitly synthetic and can never produce a proven manifest.
 */
import {spawn} from 'node:child_process';
import {readFileSync, lstatSync, readdirSync, mkdirSync, writeFileSync, openSync, closeSync, fsyncSync} from 'node:fs';
import {dirname, join, resolve, relative, isAbsolute, sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import {PIN_PATHS, generateWrappers, validatePinnedInputs, sha256, isDirectRun} from '../custody/generate.mjs';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../..');
const CUSTODY = 'experiments/moriarty-midnight-financial/custody';
const BUILDER = 'experiments/moriarty-midnight-financial/ledger/build-proven.mjs';
const REQUIRED_SOURCES = [...Object.values(PIN_PATHS), BUILDER, `${CUSTODY}/generate.mjs`, `${CUSTODY}/bindings.json`, `${CUSTODY}/loan.compact`, `${CUSTODY}/swap.compact`].sort();
const CIRCUITS = {loan:['initialize','accrue','settle'],swap:['initialize','swap','close']};
const VERSIONS = {compiler:'0.31.1',language:'0.23.0',runtime:'0.16.0',compactWrapper:'0.5.2'};
const HEX = /^[a-f0-9]{64}$/;

function requireThat(value, message) { if (!value) throw new Error(message); }
function positive(value) { return Number.isSafeInteger(value) && value > 0; }
function noSymlinks(path, {missingLeaf=false}={}) {
  const absolute=resolve(path); let current=absolute;
  while(true) {
    const stat=lstatSync(current,{throwIfNoEntry:false});
    if(stat) {
      requireThat(!stat.isSymbolicLink(),`symlink forbidden: ${current}`);
      if(current!==absolute) requireThat(stat.isDirectory(),`not a directory: ${current}`);
    } else requireThat(missingLeaf && current===absolute,`missing parent: ${current}`);
    const parent=dirname(current);if(parent===current)break;current=parent;
  }
  return absolute;
}
function checkedFile(path) {
  noSymlinks(path);requireThat(lstatSync(path).isFile(),`not a regular file: ${path}`);
  return readFileSync(path);
}
function durableExclusive(path, record) {
  noSymlinks(path,{missingLeaf:true});
  const fd=openSync(path,'wx',0o600);
  try {writeFileSync(fd,JSON.stringify(record,null,2)+'\n');fsyncSync(fd);} finally {closeSync(fd);}
  const dirFd=openSync(dirname(path),'r');try {fsyncSync(dirFd);}finally{closeSync(dirFd);}
}
function boundRecord(ref,label) {
  requireThat(ref && isAbsolute(ref.path??'') && HEX.test(ref.sha256??''),`missing ${label} binding`);
  const bytes=checkedFile(ref.path);requireThat(sha256(bytes)===ref.sha256,`${label} digest mismatch`);
  return JSON.parse(bytes.toString('utf8'));
}
export function sourceManifestHash(manifest) {return sha256(JSON.stringify(manifest));}
export function inspectBuildSources() {
  return {schema:'moriarty.financial-build-sources/1',files:REQUIRED_SOURCES.map(path=>({path,sha256:sha256(checkedFile(join(ROOT,path)))}))};
}
function checkSources(manifest) {
  requireThat(manifest?.schema==='moriarty.financial-build-sources/1' && Array.isArray(manifest.files),'source manifest required');
  const observed=inspectBuildSources();
  requireThat(manifest.files.length===observed.files.length,'missing or extra source pin');
  for(let i=0;i<observed.files.length;i++) {
    requireThat(manifest.files[i]?.path===observed.files[i].path && manifest.files[i]?.sha256===observed.files[i].sha256,`stale or missing source pin: ${observed.files[i].path}`);
    requireThat(Object.keys(manifest.files[i]).sort().join(',')==='path,sha256','unknown source pin field');
  }
  requireThat(Object.keys(manifest).sort().join(',')==='files,schema','unknown source manifest field');
}

/** Runs one direct subprocess in its own process group; no shell, retry or install. */
async function realCommand(argv,{cwd,timeoutMs}) {
  return new Promise((resolveResult,reject)=>{
    const child=spawn(argv[0],argv.slice(1),{cwd,detached:true,stdio:['ignore','pipe','pipe']});
    let stdout='',stderr='',bytes=0,failure=null;
    const stop=reason=>{failure??=reason;try{process.kill(-child.pid,'SIGKILL');}catch(error){if(error.code!=='ESRCH')failure+='; process-group kill failed';}};
    const timer=setTimeout(()=>stop('command timeout'),timeoutMs);
    const append=(which,data)=>{bytes+=data.length;if(bytes>1048576){stop('command output limit');return;}if(which==='stdout')stdout+=data;else stderr+=data;};
    child.stdout.on('data',d=>append('stdout',d));child.stderr.on('data',d=>append('stderr',d));
    child.once('error',error=>{clearTimeout(timer);reject(error);});
    child.once('close',(status,signal)=>{clearTimeout(timer);resolveResult({status,signal,stdout,stderr,error:failure});});
  });
}
function hashAssets(assetsPath,kind) {
  const artifacts=[];
  function visit(dir) {
    noSymlinks(dir);requireThat(lstatSync(dir).isDirectory(),'asset directory required');
    for(const name of readdirSync(dir).sort()) {
      const path=join(dir,name);const stat=lstatSync(path);
      requireThat(!stat.isSymbolicLink(),`asset symlink forbidden: ${path}`);
      if(stat.isDirectory())visit(path);
      else {
        requireThat(stat.isFile() && stat.size>0,`empty or nonregular artifact: ${path}`);
        artifacts.push({path:relative(assetsPath,path).split(sep).join('/'),sha256:sha256(checkedFile(path)),bytes:stat.size});
      }
    }
  }
  visit(assetsPath);
  const required=['contract/index.js','compiler/contract-info.json',...CIRCUITS[kind].flatMap(name=>[`keys/${name}.prover`,`keys/${name}.verifier`,`zkir/${name}.zkir`,`zkir/${name}.bzkir`])];
  for(const path of required)requireThat(artifacts.some(a=>a.path===path),`missing required artifact: ${path}`);
  const metadata=JSON.parse(checkedFile(join(assetsPath,'compiler/contract-info.json')));
  for(const [key,name] of [['compiler-version','compiler'],['language-version','language'],['runtime-version','runtime']])requireThat(metadata[key]===VERSIONS[name],`compiler metadata ${key} mismatch`);
  requireThat(Array.isArray(metadata.circuits),'compiler metadata circuits missing');
  const proven=metadata.circuits.filter(c=>c.proof===true && c.pure===false).map(c=>c.name).sort();
  requireThat(JSON.stringify(proven)===JSON.stringify([...CIRCUITS[kind]].sort()),'compiler metadata proof circuits mismatch');
  return artifacts;
}

export async function buildProven(options={}) {
  const {case:kind,sourceManifest,admission,resourceCounters,commandAdapter}=options;
  requireThat(Object.hasOwn(CIRCUITS,kind),'case must be loan or swap');
  requireThat(options.outputDir && isAbsolute(options.outputDir),'absolute outputDir required');
  const outputDir=resolve(options.outputDir);
  requireThat(options.outputDir===outputDir,'outputDir must be normalized');
  requireThat(!lstatSync(outputDir,{throwIfNoEntry:false}),'output already exists');
  noSymlinks(outputDir,{missingLeaf:true});
  checkSources(sourceManifest);
  const candidateHash=sourceManifestHash(sourceManifest);
  requireThat(admission?.allowFullBuild===true && typeof admission.id==='string' && admission.id.length>0,'full-build admission required');
  requireThat(admission.candidateHash===candidateHash,'admission candidate mismatch');
  requireThat(admission.case===kind && admission.outputDir===outputDir,'admission case/output mismatch');
  requireThat(positive(admission.maxCompileMs) && positive(admission.deadlineMs) && admission.deadlineMs>Date.now(),'invalid timeout/deadline');
  requireThat(admission.maxAttempts===1,'one compile attempt required');
  const resource=boundRecord(admission.resourceRecord,'resource');
  requireThat(resource.id===admission.resourceId && resource.sourceCandidateSha===candidateHash,'resource identity/candidate mismatch');
  for(const key of ['case','outputDir','maxCompileMs','deadlineMs','maxAttempts'])requireThat(resource[key]===admission[key],`resource ${key} mismatch`);
  requireThat(isAbsolute(resource.attemptFile??''),'absolute attemptFile required');
  const attemptFile=noSymlinks(resource.attemptFile,{missingLeaf:true});
  requireThat(attemptFile===resource.attemptFile && !attemptFile.startsWith(outputDir+sep),'attempt must persist outside output directory');
  requireThat(!lstatSync(attemptFile,{throwIfNoEntry:false}),'resource attempt already exists');
  requireThat(Array.isArray(admission.reviews) && admission.reviews.length>0,'review byte bindings required');
  for(const ref of admission.reviews) {
    const review=boundRecord(ref,'review');
    requireThat(review.sourceCandidateSha===candidateHash && review.verdict==='APPROVED','review candidate/verdict mismatch');
  }
  requireThat(resourceCounters?.resourceId===resource.id && resourceCounters.maxAttempts===1 && resourceCounters.attempts===0,'resource counters exhausted or mismatched');
  requireThat(commandAdapter===undefined || typeof commandAdapter==='function','invalid commandAdapter');
  const bindings=JSON.parse(checkedFile(join(ROOT,CUSTODY,'bindings.json')));
  for(const [key,name] of [['compilerVersion','compiler'],['languageVersion','language'],['compactRuntimeVersion','runtime'],['compactWrapperVersion','compactWrapper']])requireThat(bindings.toolchain[key]===VERSIONS[name],`toolchain ${key} mismatch`);
  const runtime=JSON.parse(readFileSync(join(bindings.toolchain.runtimeNodeModules,'@midnight-ntwrk/compact-runtime/package.json'),'utf8'));
  requireThat(runtime.version===VERSIONS.runtime,'runtime package version mismatch');
  const pinned=validatePinnedInputs({worktree:ROOT,bindings,readFileSync});
  const generated=generateWrappers({worktree:ROOT,bindings,readFileSync,pinned});
  for(const name of ['loan','swap']) requireThat(checkedFile(join(ROOT,CUSTODY,name+'.compact')).equals(Buffer.from(generated[name+'Source'])),`stale wrapper ${name}`);

  // Atomic reservation precedes any version/compile process and survives every outcome.
  durableExclusive(attemptFile,{schema:'moriarty.financial-build-attempt/1',resourceId:resource.id,admissionId:admission.id,sourceCandidateSha:candidateHash,case:kind,outputDir,attempts:1,startedAt:new Date().toISOString(),synthetic:!!commandAdapter});
  resourceCounters.attempts=1;
  mkdirSync(outputDir,{mode:0o700});
  const assetsPath=join(outputDir,kind),stage=join(outputDir,'stage');
  const receipt={schema:'moriarty.financial-proven-assets/1',status:'started',case:kind,assetsPath,contractModulePath:join(assetsPath,'contract/index.js'),proven:false,inspectedProofAssets:false,proofsGenerated:false,synthetic:!!commandAdapter,sourceManifestHash:candidateHash,admissionId:admission.id,resourceId:resource.id,attemptFile,resourceRecord:admission.resourceRecord,reviews:admission.reviews,versions:{},commands:[],artifacts:[]};
  const run=async argv=>{
    const remaining=admission.deadlineMs-Date.now();requireThat(remaining>0,'build deadline exceeded');
    const timeoutMs=Math.min(admission.maxCompileMs,remaining);
    const result=await (commandAdapter??realCommand)(argv,{cwd:ROOT,timeoutMs});
    const stdout=String(result.stdout??''),stderr=String(result.stderr??'');
    receipt.commands.push({argv,timeoutMs,exit:result.status,signal:result.signal??null,error:result.error??null,stdoutSha256:sha256(stdout),stderrSha256:sha256(stderr),stdout,stderr});
    requireThat(result.status===0 && !result.error,`command exit ${result.status}: ${result.error??stderr}`);
    requireThat(Date.now()<=admission.deadlineMs,'build deadline exceeded');
    return stdout.trim();
  };
  try {
    const compact=bindings.toolchain.compact;
    for(const [args,key,expected] of [[['--version'],'compactWrapper','compact '+VERSIONS.compactWrapper],[['compile','--version'],'compiler',VERSIONS.compiler],[['compile','--language-version'],'language',VERSIONS.language],[['compile','--runtime-version'],'runtime',VERSIONS.runtime]]) {
      const actual=await run([compact,...args]);requireThat(actual===expected,`${key} version mismatch`);receipt.versions[key]=actual;
    }
    mkdirSync(stage);
    writeFileSync(join(stage,'arithmetic.compact'),pinned.arithmetic.bytes,{flag:'wx'});
    writeFileSync(join(stage,'kernel.compact'),pinned[kind+'Kernel'].bytes,{flag:'wx'});
    writeFileSync(join(stage,kind+'.compact'),generated[kind+'Source'],{flag:'wx'});
    await run([compact,'compile','--compact-path',stage,join(stage,kind+'.compact'),assetsPath]);
    receipt.artifacts=hashAssets(assetsPath,kind);
    checkSources(sourceManifest);
    receipt.status=commandAdapter?'source-test-only':'built';
    receipt.proven=!commandAdapter;receipt.inspectedProofAssets=!commandAdapter;
  } catch(error) {
    receipt.status='failed';receipt.error=error.message;throw error;
  } finally {
    receipt.finishedAt=new Date().toISOString();
    durableExclusive(join(outputDir,'build-receipt.json'),receipt);
  }
  return receipt;
}

if(isDirectRun(import.meta.url)) {
  const args=process.argv.slice(2);
  requireThat(args.length===2 && args[0]==='--request','usage: build-proven.mjs --request REQUEST.json');
  const options=JSON.parse(checkedFile(resolve(args[1])).toString('utf8'));
  requireThat(!Object.hasOwn(options,'commandAdapter'),'CLI cannot inject an adapter');
  const result=await buildProven(options);
  process.stdout.write(JSON.stringify(result)+'\n');
}
