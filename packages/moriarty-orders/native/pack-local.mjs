// Stage the offline archive from the scored ledger histories and the corresponding
// native builds. All inputs stay local. Run with a populated pinned SDK installation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {dirname,join,relative,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';

const pkg = fileURLToPath(new URL('../',import.meta.url));
const repo = resolve(pkg,'../..');
const [sdk,build,verifier,notices,output] = process.argv.slice(2).map(p=>resolve(p));
assert.ok(output,'usage: node native/pack-local.mjs MODULES O1_BUILD STRICT_VERIFIER NOTICES OUTPUT');
const metadata = JSON.parse(readFileSync(join(pkg,'package.json')));
const manifest = JSON.parse(readFileSync(join(pkg,'replay/manifest.json')));
const built = JSON.parse(readFileSync(join(build,'acceptance-build.json')));
const sha = bytes=>createHash('sha256').update(bytes).digest('hex');
const digest = p=>sha(readFileSync(p));
const checkoutSources = {};
const packageSources = {};
function checkoutFile(path) {
  checkoutSources[relative(repo,path)] = digest(path);
}
assert.equal(built.programHash,manifest.program);
assert.deepEqual(built.verifiers,manifest.verifiers);
const work = mkdtempSync(join(tmpdir(),'moriarty-orders-pack-'));
function install(source,path) {
  const destination = join(work,path);
  mkdirSync(dirname(destination),{recursive:true});
  cpSync(source,destination,{recursive:true,dereference:true});
}
try {
  for (const file of ['package.json','README.md','replay/manifest.json','replay/cli.mjs','native/replay.mjs',
    'src/lifecycle.mjs','src/acceptance.mjs','src/accepted-state.mjs','src/authorization.mjs','orders/o1.mori'])
    {
      install(join(pkg,file),file);
      checkoutFile(join(pkg,file));
      packageSources[file] = digest(join(pkg,file));
    }
  install(join(repo,'LICENSE'),'LICENSE');
  install(verifier,'native/validate_call');
  const attributions = JSON.parse(readFileSync(join(notices,'index.json')));
  assert.ok(attributions.components.some(c=>c.name==='midnight-ledger-v9'));
  install(notices,'notices');
  packageSources['notices/index.json'] = digest(join(notices,'index.json'));
  for (const path of ['Cargo.lock','Cargo.toml','rust-toolchain.toml','LICENSE','crates/mori-recursion/Cargo.toml',
    'crates/mori-recursion/examples/validate_call.rs','experiments/moriarty-language/src/lower-compact.ts',
    'experiments/moriarty-language/compact/arithmetic.compact','experiments/moriarty-language/spec/bounds.json',
    'tools/sherlock-sdk/package.json','tools/sherlock-sdk/package-lock.json',
    'packages/moriarty-orders/contracts/acceptance.compact','packages/moriarty-orders/src/build-acceptance.mjs',
    'packages/moriarty-orders/native/pack-local.mjs','packages/moriarty-orders/native/collect-notices.py',
    ...readdirSync(join(pkg,'native/upstream-notices')).map(file=>'packages/moriarty-orders/native/upstream-notices/'+file)])
    checkoutFile(join(repo,path));
  // Keep the compiler's public state decoder and sources for proving-key regeneration.
  for (const file of ['index.js','index.js.map','index.d.ts']) install(join(build,'contract',file),join('generated',file));
  for (const file of ['arithmetic.compact','acceptance.compact','kernel.compact']) install(join(build,'source',file),join('source',file));
  for (const [op,id] of Object.entries(manifest.verifiers)) {
    const path = join(build,'keys',op+'.verifier');
    assert.equal(digest(path),id);
    install(path,join('verifiers',op+'.verifier'));
  }
  assert.equal(digest(join(work,'orders/o1.mori')),manifest.source);
  assert.equal(digest(join(work,'source/kernel.compact')),manifest.kernel);
  for (const folder of ['generated','source','verifiers','native']) {
    for (const file of readdirSync(join(work,folder))) packageSources[join(folder,file)] = digest(join(work,folder,file));
  }
  packageSources.LICENSE = digest(join(work,'LICENSE'));
  const sourceCommit = execFileSync('git',['rev-parse','HEAD'],{cwd:repo,encoding:'utf8'}).trim();
  const checkoutSourceFiles = Object.fromEntries(Object.entries(checkoutSources).sort());
  // Fail if a modified or untracked input would be attributed to the base commit.
  for (const [path,expected] of Object.entries(checkoutSourceFiles)) {
    assert.equal(sha(execFileSync('git',['show',sourceCommit+':'+path],{cwd:repo})),expected,path);
  }
  const packageSourceFiles = Object.fromEntries(Object.entries(packageSources).sort());
  const revisions = new Set();
  for (const [sample,expected] of Object.entries(manifest.samples)) {
    const inputs = join(repo,'deliverables/competitor-report/III.1/native',sample);
    const checks = JSON.parse(readFileSync(join(inputs,'checks.json')));
    revisions.add(checks.revision);
    for (const [file,id] of Object.entries(expected.sha256)) assert.equal(digest(join(inputs,file)),id);
    const rows = JSON.parse(readFileSync(join(inputs,'receipts.json')));
    const files = new Set([...Object.keys(expected.sha256),...rows.flatMap(row=>[row.beforeState.file,row.transaction.file,row.afterState.file])]);
    for (const file of files) install(join(inputs,file),join('replay',sample,file));
  }
  assert.equal(revisions.size,1);
  const dependencies = {};
  function dependency(name,from) {
    let directory = from;
    while (!existsSync(join(directory,name,'package.json'))) {
      if (directory===sdk) throw Error('unresolved dependency: '+name);
      directory = dirname(directory);
      if (!directory.startsWith(sdk)) directory = sdk;
    }
    const source = join(directory,name);
    const info = JSON.parse(readFileSync(join(source,'package.json')));
    if (dependencies[name]) {
      assert.equal(dependencies[name],info.version,'different transitive versions for '+name);
      return;
    }
    dependencies[name] = info.version;
    install(source,join('node_modules',name));
    for (const child of Object.keys(info.dependencies??{})) dependency(child,join(source,'node_modules'));
  }
  for (const [name,version] of Object.entries(metadata.dependencies)) {
    dependency(name,sdk);
    assert.equal(dependencies[name],version);
  }
  mkdirSync(output,{recursive:true});
  const [packed] = JSON.parse(execFileSync('npm',['pack','--offline','--ignore-scripts','--json','--pack-destination',output],
    {cwd:work,encoding:'utf8',env:{...process.env,npm_config_cache:join(work,'.cache')}}));
  for (const name of Object.keys(dependencies)) assert.ok(packed.bundled.includes(name),'missing bundled '+name);
  const release = {schema:'moriarty-orders/local-release/1',package:metadata.name,version:metadata.version,
    file:packed.filename,size:packed.size,sha256:digest(join(output,packed.filename)),integrity:packed.integrity,
    sampleBaseRevision:[...revisions][0],sourceCommit,checkoutSourceFiles,packageSourceFiles,
    checkoutSourceTreeSha256:sha(JSON.stringify(checkoutSourceFiles)),packageSourceTreeSha256:sha(JSON.stringify(packageSourceFiles)),
    noticesSha256:packageSources['notices/index.json'],program:manifest.program,verifiers:manifest.verifiers,dependencies,
    platform:'linux-arm64-glibc',nativeVerifierSha256:digest(verifier),generatedInterfaceSha256:digest(join(build,'contract/index.js')),
    operations:Object.keys(manifest.samples),publicLocation:null,publication:'pending',outsideReproduction:'deferred',
    localReplay:['npm install --offline --ignore-scripts --no-audit --no-fund /absolute/path/'+packed.filename,
      './node_modules/.bin/moriarty-orders replay o1-fill','./node_modules/.bin/moriarty-orders replay cancel-unfilled']};
  writeFileSync(join(output,'release.json'),JSON.stringify(release,null,2)+'\n');
  console.log(JSON.stringify(release));
} finally {rmSync(work,{recursive:true,force:true});}
