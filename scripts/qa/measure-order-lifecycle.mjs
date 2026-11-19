// Summarize serial local-ledger executions only after checking the signed bytes,
// strict replay, actual DUST debit and frozen operation inventory.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {replayNativeReceipt} from '../../packages/moriarty-orders/native/replay.mjs';
import {requireCommittedGenerator} from '../../packages/moriarty-orders/native/record.mjs';

const root = fileURLToPath(new URL('../../',import.meta.url));
const report = join(root,'deliverables/competitor-report/III.1');
const read = file => JSON.parse(readFileSync(file));
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const recipe = read(join(report, 'recipe.json'));
const fields = Object.keys(recipe.resourceUnits);
const strictFlags = ['enforceBalancing','enforceLimits','verifyContractProofs','verifyNativeProofs','verifySignatures'];
function uint(value) {
  assert.ok(typeof value === 'string' && /^(0|[1-9][0-9]*)$/.test(value), 'noncanonical unsigned integer');
  return BigInt(value);
}
export function checkInventory(operation, actual) {
  assert.deepEqual(actual, [...operation.setup,...operation.operation,...operation.recovery]);
}
export function measureTransaction(row) {
  assert.equal(row.status,'success');
  for (const flag of strictFlags) assert.equal(row.strict[flag],true);
  assert.deepEqual(Object.keys(row.resources).sort(), [...fields].sort());
  const resources = Object.fromEntries(fields.map(k => [k,uint(row.resources[k])]));
  assert.equal(row.dustTimeAfter,row.time);
  assert.equal(row.dustTimeBefore,row.time);
  const fee = uint(row.nativeFeeSpeck);
  assert.ok(fee >= uint(row.requiredFeeSpeck));
  assert.equal(uint(row.dustBefore)-uint(row.dustAfter),fee);
  assert.ok(row.dustPayments.length > 0);
  assert.equal(row.dustPayments.length,row.dustSpendEvents.length);
  const events = new Map();
  for (const event of row.dustSpendEvents) {
    assert.equal(event.source.transactionHash,row.transactionHash);
    assert.equal(event.content.tag,'dustSpendProcessed');
    assert.ok(Object.keys(event.serialized).length > 0);
    assert.ok(!events.has(event.content.nullifier));
    events.set(event.content.nullifier,event);
  }
  let paid = 0n;
  for (const payment of row.dustPayments) {
    assert.equal(payment.creationTime,row.time);
    assert.equal(payment.accepted,true);
    const event = events.get(payment.oldNullifier);
    assert.ok(event);
    events.delete(payment.oldNullifier);
    assert.equal(event.source.physicalSegment,payment.segment);
    assert.equal(event.content.commitment,payment.newCommitment);
    assert.equal(event.content.vFee,payment.vFee);
    paid += uint(payment.vFee);
  }
  assert.equal(paid,fee);
  return {transaction_id:row.transactionHash,resources,fee};
}
function sum(rows) {
  const resources = Object.fromEntries(fields.map(k => [k,0n]));
  let fee_paid = 0n;
  for (const tx of rows) {
    for (const key of fields) resources[key] += tx.resources[key];
    fee_paid += tx.fee;
  }
  return {transaction_count:rows.length,resources,fee_paid};
}
export async function measureSample(directory, operation, validator) {
  const profilePath = join(report,recipe.profile.file);
  assert.equal(hash(readFileSync(profilePath)),recipe.profile.sha256);
  const profile = read(profilePath).operations.find(op=>op.id===operation);
  const spec = recipe.operations[operation];
  assert.ok(profile && spec);
  const receipts = read(join(directory,'receipts.json'));
  const semantic = read(join(directory,'checks.json'));
  assert.equal(semantic.operation,operation);
  assert.deepEqual(Object.keys(semantic.results).sort(),[...profile.required_checks].sort());
  for (const result of Object.values(semantic.results)) assert.equal(result,true);
  assert.equal(semantic.protocolFee,0);
  assert.deepEqual(semantic.finalBalances,operation === 'o1-fill' ?
    [[0,0],[100,999800],[0,200],[0,0]] : [[100,0],[0,1000000],[0,0],[0,0]]);
  // The sample names the clean committed code and the host that measured it.
  const revision = requireCommittedGenerator(semantic.generator);
  assert.ok(semantic.executionId && semantic.generator.platform && semantic.generator.architecture);
  const scored = receipts.filter(r=>!r.label.startsWith('control-') && !r.label.startsWith('setup-'));
  checkInventory(spec,scored.map(r=>r.label));
  assert.deepEqual(receipts.filter(r=>r.label.startsWith('setup-')).map(r=>r.label),
    ['setup-night-reward','setup-dust-registration']);
  for (const row of receipts) replayNativeReceipt(validator,directory,row);
  const modules = process.env.MORIARTY_ORDER_MODULES ?? join(root,'tools/sherlock-sdk/node_modules');
  const ledger = await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
  const parameters = ledger.LedgerParameters.deserialize(readFileSync(join(directory,'parameters.bin')));
  const codec = await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
  const setup = scored.find(row=>row.label==='fund-order');
  const filled = scored.at(-1);
  const view = state => {
    const contract = state.index(semantic.deployment.address);
    assert.ok(contract);
    // The recorded genesis fixes the three fixture wallet keys. Read token types
    // from the scored fund and mint transactions, independently of the recorded matrix.
    const fund = ledger.Transaction.deserialize('signature','proof','binding',readFileSync(join(directory,setup.transaction.file)));
    const types = [...fund.intents.values()].flatMap(i=>[...(i.guaranteedUnshieldedOffer?.inputs??[]),
      ...(i.fallibleUnshieldedOffer?.inputs??[])]).map(c=>c.type);
    const [a] = new Set(types);
    const executor = ledger.addressFromKey(ledger.signatureVerifyingKey(ledger.signingKeyFromBip340(new Uint8Array(32).fill(35))));
    const initial = ledger.LedgerState.deserialize(readFileSync(join(directory,setup.afterState.file)));
    const [b] = [...initial.utxo.filter(executor)].map(c=>c.type).filter(type=>type!==a);
    assert.ok(a && b);
    const balance = codec.ContractState.deserialize(contract.serialize()).balance;
    return [31,35,33,null].map(seed=>[a,b].map(type=>{
      if (seed===null) return Number([...balance].find(([asset])=>asset.tag==='unshielded' && asset.raw===type)?.[1]??0n);
      const key = ledger.signatureVerifyingKey(ledger.signingKeyFromBip340(new Uint8Array(32).fill(seed)));
      return Number([...state.utxo.filter(ledger.addressFromKey(key))].filter(c=>c.type===type).reduce((n,c)=>n+c.value,0n));
    }));
  };
  assert.deepEqual(view(ledger.LedgerState.deserialize(readFileSync(join(directory,setup.afterState.file)))),semantic.initialBalances);
  assert.deepEqual(view(ledger.LedgerState.deserialize(readFileSync(join(directory,filled.afterState.file)))),semantic.finalBalances);
  let wallet = new ledger.DustLocalState(parameters.dust);
  const dustKey = ledger.DustSecretKey.fromSeed(new Uint8Array(32).fill(32));
  let previous;
  for (const row of receipts) {
    if (previous) assert.equal(previous.afterBlockStateSha256,row.beforeState.sha256,row.label);
    previous = row;
    const state = ledger.LedgerState.deserialize(readFileSync(join(directory,row.beforeState.file)));
    const wire = readFileSync(join(directory,row.transaction.file));
    const reward = row.label === 'setup-night-reward';
    const tx = ledger.Transaction.deserialize('signature',reward ? 'pre-proof' : 'proof','binding',wire);
    if (reward) assert.ok(tx.rewards, 'proof-free envelope must be a native reward claim');
    const time = new Date(row.time*1000);
    const before = wallet.walletBalance(time);
    const verified = tx.wellFormed(state,new ledger.WellFormedStrictness(),time);
    const context = new ledger.TransactionContext(state,{secondsSinceEpoch:BigInt(row.time),
      secondsSinceEpochErr:0,lastBlockTime:BigInt(row.time),parentBlockHash:'00'.repeat(32)});
    const [after,applied] = state.apply(verified,context);
    assert.equal(hash(after.serialize()),row.afterState.sha256);
    wallet = wallet.replayEvents(dustKey,applied.events);
    if (!scored.includes(row)) continue;
    assert.equal(before,uint(row.dustBefore));
    assert.equal(wallet.walletBalance(time),uint(row.dustAfter));
    assert.equal(tx.transactionHash(),row.transactionHash);
    assert.equal(tx.fees(parameters),uint(row.requiredFeeSpeck));
    for (const [name,value] of Object.entries(tx.cost(parameters))) assert.equal(value,uint(row.resources[name]));
    const spends = applied.events.filter(event=>event.content.tag==='dustSpendProcessed');
    assert.equal(spends.length,row.dustSpendEvents.length);
    for (const [index,event] of spends.entries()) {
      assert.deepEqual([...event.serialize()],Object.values(row.dustSpendEvents[index].serialized));
    }
  }
  const transactions = scored.map(row => ({...measureTransaction(row), label:row.label, signed:row.transaction}));
  const phases = Object.fromEntries(['setup','operation','recovery'].map(p=>
    [p,sum(transactions.filter(tx=>spec[p].includes(tx.label)))]));
  return {operation,executionId:semantic.executionId,revision,host:{platform:semantic.generator.platform,
    architecture:semantic.generator.architecture,node:semantic.generator.node},boundary:profile.boundary,
    finality:recipe.finality,feePolicy:recipe.feePolicy,phases,total:sum(transactions),transactions};
}
if (import.meta.url === pathToFileURL(resolve(process.argv[1] ?? '')).href) {
  const [directory,operation,validator] = process.argv.slice(2);
  if (!validator) throw Error('usage: node measure-order-lifecycle.mjs EVIDENCE OPERATION VALIDATOR');
  console.log(JSON.stringify(await measureSample(directory,operation,validator),
    (_,v)=>typeof v==='bigint' ? JSON.rawJSON(String(v)) : v,2));
}
