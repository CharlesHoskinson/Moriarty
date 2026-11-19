import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {mkdtempSync, readFileSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {replayNativeReceipt} from './replay.mjs';
import {buildAcceptance} from '../src/build-acceptance.mjs';
import {readContinuation,checkContinuation,checkO9Balances} from '../src/continuation.mjs';
import {NativeLedger} from '../test/native-ledger.mjs';

const requirement = process.argv[2];
assert.ok(['II.4','IV.4'].includes(requirement),'expected II.4 or IV.4');
const root = fileURLToPath(new URL('../../../',import.meta.url));
const directory = join(root,'deliverables/competitor-report/IV.4/native');
const modules = process.env.MORIARTY_ORDER_MODULES ?? join(root,'tools/sherlock-sdk/node_modules');
const validator = process.env.MORIARTY_ORDER_VALIDATOR ?? join(root,'target/debug/examples/validate_call');
const json = path=>JSON.parse(readFileSync(path));
const hex = bytes=>Buffer.from(bytes).toString('hex');
const records = json(join(directory,'receipts.json'));
const {build,deployment,enrollment,actors,clients,controls} = json(join(directory,'readback.json'));
const expected = json(join(root,'packages/moriarty-orders/test/fixtures/o9.json'));
const trial = [
  ['setup-night-reward'],['setup-dust-registration'],['AB-deploy'],['AB-mint'],['C-deploy'],['C-mint'],
  ['deploy-staged-order'],['fund-staged-100A','fund','funded'],['trade-100A-for-200B','first','waiting'],
  ['enable-owner-event','enable','waiting'],['resume-wins','resume','resumed'],
  ['refund-loses-after-resume','refund','resumed',false],['resume-replay','resume','resumed',false],
  ['refund-wins','refund','refunded'],['resume-loses-after-refund','resume','refunded',false],
  ['refund-replay','refund','refunded',false],['spend-recovered-B',null,'spent'],
];
assert.deepEqual(records.map(r=>r.label),trial.map(r=>r[0]));
assert.equal(deployment.network,'undeployed');
const runtime = await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
const L = await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
for (const [name,seed] of [['owner',31],['recipient',33],['executor',35]]) {
  const vk = L.signatureVerifyingKey(L.signingKeyFromBip340(new Uint8Array(32).fill(seed)));
  assert.equal(actors[name],L.addressFromKey(vk));
}
const output = mkdtempSync(join(tmpdir(),'moriarty-stage-check-'));
try {
  const pins = buildAcceptance(output,{skipZk:true,policy:'staged'});
  for (const k of ['programHash','sourceSha256','kernelSha256','upstreamArithmeticSha256','arithmeticSha256',
    'includes','custodySha256','compilerVersion','zkir']) assert.deepEqual(pins[k],build[k],k);
  assert.equal(deployment.programHash,pins.programHash);
  assert.deepEqual(deployment.verifiers,build.verifiers);
  const compiled = await import(pathToFileURL(join(output,'contract/index.js')));
  const instance = {address:deployment.address,contract:{generated:compiled,instance:new compiled.Contract({})}};
  const replays = records.map(r=>replayNativeReceipt(validator,directory,r));
  const snapshots = [];
  for (const [i,row] of records.entries()) {
    const [label,operation,stage,accepted = true] = trial[i];
    const parent = label==='refund-wins'?9:i-1;
    if (parent>=0) assert.equal(row.beforeState.sha256,records[parent].afterBlockStateSha256,label);
    assert.equal(replays[i].outcome,accepted?'success':'failure',label);
    if (!accepted) assert.equal(row.beforeState.sha256,row.afterState.sha256,label);
    if (operation) {
      assert.equal(replays[i].contractCalls.length,1);
      const [call] = replays[i].contractCalls;
      assert.equal(call.address,deployment.address);
      assert.equal(call.entryPoint,operation);
      assert.equal(call.proofVersion,'V3');
      assert.equal(call.verifierKeySHA256,deployment.verifiers[operation]);
    }
    if (!stage) continue;
    const bytes = readFileSync(join(directory,row.afterState.file));
    const state = L.LedgerState.deserialize(bytes);
    const head = readContinuation({network:deployment.network,index:a=>state.index(a)},runtime,compiled,deployment);
    const {terms} = head;
    assert.equal(runtime.decodeUserAddress(terms.account.bytes),actors.owner);
    assert.equal(runtime.decodeUserAddress(terms.executor.bytes),actors.executor);
    const assets = [terms.assetA,terms.assetB,terms.assetC].map(hex);
    assert.equal(new Set(assets).size,3);
    const wallets = Object.fromEntries(Object.entries(actors).map(([name,address])=>[name,assets.map(asset=>
      Number([...state.utxo.filter(address)].filter(c=>c.type===asset).reduce((n,c)=>n+c.value,0n)))]));
    const escrow = new Map([...state.index(deployment.address).balance].map(([token,amount])=>[token.raw,amount]));
    const balances = {...wallets,custody:assets.map(asset=>Number(escrow.get(asset)??0n))};
    checkO9Balances(balances,expected[stage]);
    const phases = {funded:[0n,0n,0n,100n,0n],waiting:[1n,1n,0n,0n,200n],
      resumed:[2n,2n,1n,0n,0n],refunded:[2n,2n,2n,0n,0n],spent:[2n,2n,2n,0n,0n]};
    assert.deepEqual([head.current.f2,head.revision,head.terminal,head.current.f0,head.current.f1],phases[stage]);
    assert.equal(head.remaining+head.revision,3n);
    if (i>=8) {
      assert.equal(hex(head.pending.asset),assets[1]);
      assert.equal(head.pending.amount,200n);
      assert.equal(head.pending.budget,1n);
      assert.equal(hex(head.pending.custody.bytes),hex(runtime.encodeContractAddress(deployment.address)));
      assert.equal(hex(head.pending.program),pins.programHash);
      assert.equal(hex(head.pending.eventId),hex(terms.eventId));
      assert.equal(hex(head.pending.orderId),hex(terms.orderId));
      assert.notEqual(hex(head.pending.predecessor),'00'.repeat(32));
      if (i>8) checkContinuation(snapshots[8].head.pending,head.pending);
    }
    assert.equal(head.eventEnabled,i>=9);
    if (i===10) assert.ok(BigInt(row.time)<terms.epoch+1400n);
    if (i===13) assert.equal(BigInt(row.time),terms.epoch+1400n);
    snapshots[i] = {bytes,state,head,balances};
  }

  // Start fresh wallet processes on the actual retained ledger bytes. Each process
  // verifies the native wallet enrollment and the five installed operation keys.
  function restarted(i,action) {
    const input = {head:hex(snapshots[i].bytes),modules,directory:output,deployment,enrollment,request:action};
    const child = spawnSync(process.execPath,[fileURLToPath(new URL('../test/staged-client.mjs',import.meta.url))],
      {input:JSON.stringify(input),encoding:'utf8',timeout:30000});
    assert.equal(child.status,0,child.stderr);
    return JSON.parse(child.stdout);
  }
  const prior = restarted(9,2);
  assert.ok(prior.signature);
  for (const [index,action,terminal] of [[10,3,1],[13,2,2]]) {
    const read = restarted(index,action);
    assert.equal(read.revision,2);
    assert.equal(read.stage,2);
    assert.equal(read.terminal,terminal);
    assert.equal(read.signature,undefined);
  }
  assert.ok(clients.length>=5);
  assert.equal(new Set(clients.map(c=>c.pid)).size,clients.length);
  for (const [label,row,terminal] of [['restarted-after-resume',10,1],['restarted-after-refund',13,2]]) {
    const client = clients.find(c=>c.label===label);
    assert.equal(client.ledgerSha256,records[row].afterBlockStateSha256);
    assert.equal(client.terminal,terminal);
    assert.equal(client.exit,0);
  }

  // Reconstruct the argument-bound rejection controls from the accepted waiting
  // state against the freshly compiled Compact execution path.
  const signature = r=>({announcement:{x:BigInt(r.announcement.x),y:BigInt(r.announcement.y)},response:BigInt(r.response)});
  async function refused(i,op,args,time,pattern) {
    const h = {L,runtime,state:snapshots[i].state,time:Number(time)};
    await assert.rejects(()=>NativeLedger.prototype.prepareCall.call(h,instance,op,args),pattern);
  }
  const initial = snapshots[7].head, waiting = snapshots[8].head.pending;
  const epoch = initial.terms.epoch, first = signature(restarted(7,0).signature);
  await refused(7,'first',[199n,1199n,first],epoch+1199n,/first minimum/);
  await refused(7,'first',[200n,1200n,first],epoch+1200n,/FIRST_EXPIRED/);
  const resume = signature(prior.signature);
  await refused(8,'resume',[waiting,300n,1199n,resume],epoch+1199n,/CONDITION_FALSE/);
  await refused(9,'resume',[waiting,299n,1399n,resume],epoch+1399n,/second minimum/);
  await refused(9,'resume',[waiting,300n,1400n,resume],epoch+1400n,/SECOND_EXPIRED/);
  const mutations = Object.keys(waiting);
  for (const key of mutations) {
    const changed = structuredClone(waiting);
    changed[key] = typeof waiting[key]==='bigint'?waiting[key]+1n:key==='custody'?{bytes:new Uint8Array(32).fill(8)}:new Uint8Array(32).fill(8);
    await refused(9,'resume',[changed,300n,1399n,resume],epoch+1399n,/CONTINUATION_CAPTURE/);
  }
  assert.deepEqual(controls,['first minimum','at first expiry','condition false','second minimum',...mutations.map(k=>'altered '+k),'at second expiry']);
  console.log(JSON.stringify({requirement,receipts:records.length,programHash:pins.programHash,controls,
    fundedRaceRoot:records[9].afterBlockStateSha256,replays,
    readbacks:trial.flatMap(([label,,stage],i)=>stage?[{label,stage,balances:snapshots[i].balances}]:[]),
    clientRestarts:clients.filter(c=>c.label.startsWith('restarted')).map(c=>({label:c.label,terminal:c.terminal,root:c.ledgerSha256}))},null,2));
} finally { rmSync(output,{recursive:true,force:true}); }
