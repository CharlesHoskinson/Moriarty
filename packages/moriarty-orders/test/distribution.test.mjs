import test from 'node:test';
import assert from 'node:assert/strict';
import {execFileSync,spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {mkdtempSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath} from 'node:url';

const releaseDirectory = fileURLToPath(new URL('../../../deliverables/competitor-report/III.6/',import.meta.url));
test('install the pinned archive offline, run both native lifecycles, then exercise a broken installation', () => {
  const release = JSON.parse(readFileSync(join(releaseDirectory,'release.json')));
  const archive = join(releaseDirectory,release.file);
  assert.equal(createHash('sha256').update(readFileSync(archive)).digest('hex'),release.sha256);
  const cwd = mkdtempSync(join(tmpdir(),'order-consumer-'));
  try {
    const env = {PATH:process.env.PATH,HOME:cwd,npm_config_cache:join(cwd,'empty-cache')};
    const options = {cwd,env,encoding:'utf8'};
    const start = Date.now();
    execFileSync('npm',['install','--offline','--ignore-scripts','--no-audit','--no-fund','--no-package-lock',archive],options);
    const cli = join(cwd,'node_modules/.bin/moriarty-orders');
    const pkg = join(cwd,'node_modules/@moriarty-lang/orders');
    const notices = JSON.parse(readFileSync(join(pkg,'notices/index.json')));
    for (const id of ['midnight-ledger-v9','midnight-ledger-wasm-v9','midnight-onchain-runtime-wasm','blst','@midnight-ntwrk/compact-runtime']) {
      assert.ok(notices.components.some(component=>component.name===id && component.texts.length),id);
    }
    for (const component of notices.components) {
      for (const text of component.texts) {
        assert.equal(createHash('sha256').update(readFileSync(join(pkg,'notices',text.file))).digest('hex'),text.sha256);
      }
    }
    for (const [name,id] of Object.entries(release.packageSourceFiles)) {
      assert.equal(createHash('sha256').update(readFileSync(join(pkg,name))).digest('hex'),id);
    }
    const exports = JSON.parse(execFileSync(process.execPath,['--input-type=module','-e',
      'import * as api from "@moriarty-lang/orders"; console.log(JSON.stringify(Object.keys(api)))'],options));
    for (const name of ['fundOrder','acceptFill','cancelOrder','readAcceptedOrder','signOrder','issueEnrollment']) assert.ok(exports.includes(name),name);
    const operations = ['o1-fill','cancel-unfilled'].map(op=>JSON.parse(execFileSync(cli,['replay',op],options)));
    assert.deepEqual(operations.map(o=>o.operation),['o1-fill','cancel-unfilled']);
    assert.deepEqual(operations[0].finalBalances,[[0,0],[100,999800],[0,200],[0,0]]);
    assert.deepEqual(operations[1].finalBalances,[[100,0],[0,1000000],[0,0],[0,0]]);
    assert.ok(operations[1].receipts.find(r=>r.label==='spend-refund' && r.outcome==='success'));
    assert.notEqual(spawnSync(cli,['replay','unknown'],options).status,0);
    const firstState=join(pkg,'replay/o1-fill/00-beforeState.bin');
    const damaged=readFileSync(firstState); damaged[0]^=1; writeFileSync(firstState,damaged);
    assert.notEqual(spawnSync(cli,['replay','o1-fill'],options).status,0);
    rmSync(join(pkg,'generated/index.js'));
    assert.notEqual(spawnSync(cli,['replay','cancel-unfilled'],options).status,0);
    const result = {schema:'moriarty-orders/clean-install/1',platform:process.platform,architecture:process.arch,
      node:process.version,npm:execFileSync('npm',['--version'],options).trim(),elapsedMs:Date.now()-start,sha256:release.sha256,
      package:release.package,version:release.version,offline:true,emptyCache:true,operations};
    if (process.env.MORIARTY_ORDER_INSTALL_RECEIPT) writeFileSync(process.env.MORIARTY_ORDER_INSTALL_RECEIPT,JSON.stringify(result,null,2)+'\n');
  } finally {
    rmSync(cwd,{recursive:true,force:true});
  }
});
