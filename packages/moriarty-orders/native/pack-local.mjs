// Stage the offline archive from the scored ledger histories and the corresponding
// native builds. All inputs stay local. Run with a populated pinned SDK installation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {dirname,join,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';

const pkg = fileURLToPath(new URL('../',import.meta.url));
const repo = resolve(pkg,'../..');
const [sdk,build,verifier,output] = process.argv.slice(2).map(p=>resolve(p));
assert.ok(output,'usage: node native/pack-local.mjs MODULES O1_BUILD STRICT_VERIFIER OUTPUT');
const metadata = JSON.parse(readFileSync(join(pkg,'package.json')));
const manifest = JSON.parse(readFileSync(join(pkg,'replay/manifest.json')));
const built = JSON.parse(readFileSync(join(build,'acceptance-build.json')));
const digest = p=>createHash('sha256').update(readFileSync(p)).digest('hex');
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
    install(join(pkg,file),file);
  install(join(repo,'LICENSE'),'LICENSE');
  install(verifier,'native/validate_call');
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
    evidenceRevision:[...revisions][0],program:manifest.program,verifiers:manifest.verifiers,dependencies,
    platform:'linux-arm64-glibc',nativeVerifierSha256:digest(verifier),generatedInterfaceSha256:digest(join(build,'contract/index.js')),
    operations:Object.keys(manifest.samples),publicLocation:null,publication:'pending',outsideReproduction:'deferred',
    localReplay:['npm install --offline --ignore-scripts --no-audit --no-fund /absolute/path/'+packed.filename,
      './node_modules/.bin/moriarty-orders replay o1-fill','./node_modules/.bin/moriarty-orders replay cancel-unfilled']};
  writeFileSync(join(output,'release.json'),JSON.stringify(release,null,2)+'\n');
  console.log(JSON.stringify(release));
} finally {rmSync(work,{recursive:true,force:true});}
