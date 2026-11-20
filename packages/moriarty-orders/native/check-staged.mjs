import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {mkdtempSync, readFileSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {replayNativeReceipt} from './replay.mjs';
import {rerunRefusals} from './refusals.mjs';
import {buildAcceptance} from '../src/build-acceptance.mjs';
import {readContinuation,checkContinuation,checkO9Balances,verifyStagedEnrollment} from '../src/continuation.mjs';

const requirement = process.argv[2];
assert.ok(['II.4','IV.4'].includes(requirement),'expected II.4 or IV.4');
const root = fileURLToPath(new URL('../../../',import.meta.url));
const directory = process.argv[3] ?? join(root,'deliverables/competitor-report/IV.4/native');
const modules = process.env.MORIARTY_ORDER_MODULES ?? join(root,'tools/sherlock-sdk/node_modules');
const validator = process.env.MORIARTY_ORDER_VALIDATOR ?? join(root,'target/debug/examples/validate_call');
const json = path=>JSON.parse(readFileSync(path));
const hex = bytes=>Buffer.from(bytes).toString('hex');
const sha = bytes=>createHash('sha256').update(bytes).digest('hex');
const records = json(join(directory,'receipts.json'));
const {build,deployment,enrollment,actors,clients} = json(join(directory,'readback.json'));
const expected = json(join(root,'packages/moriarty-orders/test/fixtures/o9.json'));
// [label, operation, stage, accepted, parent]
const trial = [
  ['setup-night-reward'],['setup-dust-registration'],['AB-deploy'],['AB-mint'],['C-deploy'],['C-mint'],
  ['deploy-staged-order'],['fund-staged-100A','fund','funded'],['trade-100A-for-200B','first','waiting'],
  ['enable-owner-event','enable','waiting'],
  ['resume-wins','resume','resumed',true,9],['resume-replay-before-expiry','resume','resumed',false],
  ['refund-loses-after-resume','refund','resumed',false],
  ['refund-wins','refund','refunded',true,9],['resume-loses-after-refund','resume','refunded',false],
  ['refund-replay','refund','refunded',false],['spend-recovered-B',null,'spent'],
  ['withdraw-unfilled-100A','withdraw','withdrawn',true,7],['spend-withdrawn-A',null,'withdrawnSpent'],
];
assert.deepEqual(records.map(r=>r.label),trial.map(r=>r[0]));
assert.equal(deployment.network,'undeployed');
const runtime = await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
const L = await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
for (const [name,seed] of [['owner',31],['recipient',33],['executor',35]]) {
  const vk = L.signatureVerifyingKey(L.signingKeyFromBip340(new Uint8Array(32).fill(seed)));
  assert.equal(actors[name],L.addressFromKey(vk));
}
// A refused terminal action loses only to consumption: the ledger read the consumed stage.
const consumed = /mismatch between expected \(<\[[^\]]*, 01\]: [^>]*>\) and actual \(<\[[^\]]*, 02\]: [^>]*>\) read/;
const output = mkdtempSync(join(tmpdir(),'moriarty-stage-check-'));
try {
  const pins = buildAcceptance(output,{skipZk:true,policy:'staged'});
  for (const k of ['programHash','sourceSha256','kernelSha256','upstreamArithmeticSha256','arithmeticSha256',
    'includes','custodySha256','compilerVersion','zkir']) assert.deepEqual(pins[k],build[k],k);
  assert.equal(deployment.programHash,pins.programHash);
  assert.deepEqual(deployment.verifiers,build.verifiers);
  assert.deepEqual(Object.keys(deployment.verifiers).sort(),['enable','first','fund','refund','resume','withdraw']);
  const compiled = await import(pathToFileURL(join(output,'contract/index.js')));
  const replays = records.map(r=>replayNativeReceipt(validator,directory,r));
  const snapshots = [];
  for (const [i,row] of records.entries()) {
    const [label,operation,stage,accepted = true,parent = i-1] = trial[i];
    if (parent>=0) assert.equal(row.beforeState.sha256,records[parent].afterBlockStateSha256,label);
    assert.equal(replays[i].outcome,accepted?'success':'failure',label);
    if (!accepted) {
      assert.equal(row.beforeState.sha256,row.afterState.sha256,label);
      // A byte-identical refund replay stops at intent replay protection before any state read.
      assert.match(String(row.error),label==='refund-replay'?/replay protection has been violated: IntentAlreadyExists/:consumed,label);
    }
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
    verifyStagedEnrollment(L,runtime,deployment,head,enrollment);
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
    const phases = {funded:[0n,0n,0n,100n,0n],waiting:[1n,1n,0n,0n,200n],resumed:[2n,2n,1n,0n,0n],
      refunded:[2n,2n,2n,0n,0n],spent:[2n,2n,2n,0n,0n],withdrawn:[2n,1n,3n,0n,0n],withdrawnSpent:[2n,1n,3n,0n,0n]};
    assert.deepEqual([head.current.f2,head.revision,head.terminal,head.current.f0,head.current.f1],phases[stage],label);
    assert.equal(head.remaining+head.revision,3n);
    if (i>=8 && i<=16) {
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
    assert.equal(head.eventEnabled,i>=9 && i<=16);
    if ([10,11].includes(i)) assert.ok(BigInt(row.time)<terms.epoch+1400n,label+' precedes the second deadline');
    if (i===13) assert.equal(BigInt(row.time),terms.epoch+1400n);
    if (i===17) assert.equal(BigInt(row.time),terms.epoch+1200n);
    snapshots[i] = {bytes,state,head,balances};
  }

  // Restarted processes resubmitted the journals two earlier processes wrote against the
  // shared waiting predecessor. The journals are retained and digest-bound.
  const raceRoot = records[9].afterBlockStateSha256;
  const submitted = clients.filter(c=>c.mode==='resubmit');
  assert.deepEqual(submitted.map(c=>c.submitted.label),['resume-wins','resume-replay-before-expiry','refund-loses-after-resume',
    'refund-wins','resume-loses-after-refund','refund-replay']);
  for (const c of submitted) {
    assert.equal(c.preparedAgainstSha256,raceRoot);
    const file = join(directory,'journals',(c.action==='resume'?'resume':'refund')+'-client.json');
    assert.equal(sha(readFileSync(file)),c.journalSha256);
    assert.equal(c.submitted.transactionHash,records.find(r=>r.label===c.submitted.label).transactionHash);
  }
  const writers = clients.filter(c=>c.mode==='journal');
  assert.deepEqual(writers.map(c=>c.label),['resume-client','refund-client']);
  assert.equal(new Set(clients.map(c=>c.pid)).size,clients.length,'each client is a separate process');

  // Start fresh honest wallet processes on the retained ledger bytes. After either
  // terminal action they read the consumed continuation and decline to sign.
  function restarted(i,action) {
    const input = {head:hex(snapshots[i].bytes),modules,directory:output,deployment,enrollment,request:action};
    const child = spawnSync(process.execPath,[fileURLToPath(new URL('../test/staged-client.mjs',import.meta.url))],
      {input:JSON.stringify(input),encoding:'utf8',timeout:60000});
    assert.equal(child.status,0,child.stderr);
    return JSON.parse(child.stdout);
  }
  assert.ok(restarted(9,2).signature);
  for (const [index,action,terminal] of [[10,3,1],[13,2,2],[17,0,3]]) {
    const read = restarted(index,action);
    assert.equal(read.terminal,terminal);
    assert.equal(read.signature,undefined);
  }
  for (const [label,row,terminal] of [['restarted-after-resume',10,1],['restarted-after-refund',13,2]]) {
    const client = clients.find(c=>c.label===label);
    assert.equal(client.ledgerSha256,records[row].afterBlockStateSha256);
    assert.equal(client.terminal,terminal);
    assert.equal(client.exit,0);
  }

  // Re-execute every argument-bound refusal against its recorded accepted state.
  const refusals = json(join(directory,'refusals.json'));
  const mutations = Object.keys(snapshots[8].head.pending);
  assert.deepEqual(refusals.map(r=>r.label),['first minimum','at first expiry','condition false','second minimum',
    ...mutations.map(k=>'altered '+k),'at second expiry','withdraw before first deadline','first after withdrawal']);
  const rerun = await rerunRefusals({L,runtime,compiled,directory,rows:refusals});
  assert.match(rerun.find(r=>r.label==='condition false').error,/condition false/);
  console.log(JSON.stringify({requirement,receipts:records.length,programHash:pins.programHash,
    evidenceClasses:{ledgerApplied:replays.map(r=>r.label),compiledCircuitRefusals:rerun},
    fundedRaceRoot:raceRoot,replays,
    readbacks:trial.flatMap(([label,,stage],i)=>stage?[{label,stage,balances:snapshots[i].balances}]:[]),
    clientRestarts:clients.filter(c=>c.mode==='resubmit' || c.label.startsWith('restarted'))
      .map(c=>({label:c.label,pid:c.pid,mode:c.mode,submitted:c.submitted??null,root:c.ledgerSha256}))},null,2));
} finally { rmSync(output,{recursive:true,force:true}); }
