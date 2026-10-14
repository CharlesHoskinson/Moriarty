import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { calibrateGuards } from '../src/custody-time.mjs';

test('constructed fully pinned transcript and copied origin names cannot admit production', () => {
  const root = mkdtempSync(join(tmpdir(), 'forged-'));
  const pin = (name, body) => {
    const path = join(root, name); writeFileSync(path, body);
    return { path, sha256: createHash('sha256').update(body).digest('hex'), present: true };
  };
  const bindings = {
    genesis: pin('genesis', '{"slotLength":1}'), artifact: pin('artifact', 'compiled-probe'),
    body: pin('body', 'unsigned-body'), readback: pin('readback', '{"scriptContextLower":"1000"}'),
  };
  const log = pin('commands', `genesis ${bindings.genesis.path}\nquery tip\nquery era-history\n`);
  const transcriptPath = join(root, 'transcript.json');
  const queriedSeed = { txId: '1234'.repeat(16), index: 0 };
  const transcript = {
    productionAdmitted: true, networkCommandsRun: true, evidenceClass: 'node-genesis-v3-probe',
    genesisPath: bindings.genesis.path, artifactPath: bindings.artifact.path, bodyPath: bindings.body.path,
    readbackPath: bindings.readback.path, cliPosix: '1000', lucidPosix: '1000', scriptContextLower: '1000',
    originTool: 'executeParityCapture', commandLogPath: log.path, commandLogSha256: log.sha256, queriedSeed,
  };
  writeFileSync(transcriptPath, JSON.stringify(transcript));
  const result = calibrateGuards({
    probeExecuted: true, evidenceClass: 'node-genesis-v3-probe', slotLengthMs: '1000',
    cliPosix: '1000', lucidPosix: '1000', scriptContextLower: '1000', bindings, transcriptPath,
    origin: 'executeParityCapture', queriedSeed,
    measuredBounds: { releaseLower: '40000', partition: '100000', refundInclusive: '159999' },
  });
  assert.equal(result.ok, false);
  assert.equal(result.code, 'CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
});

test('source terminals evaluate substituted time guards and project the fixed Core amount', async () => {
  const { readFileSync } = await import('node:fs');
  const { substituteTimeGuards } = await import('../src/custody-time.mjs');
  const { loadSourceTerminals } = await import('../src/native-financial.mjs');
  const source = readFileSync('/home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time/escrow-custody-v5.mori', 'utf8');
  const bounds = { releaseLower: '40000', partition: '100000', refundInclusive: '159999' };
  const sibling = substituteTimeGuards(source, bounds);
  const terminals = loadSourceTerminals(sibling.text, { bounds });
  assert.equal(terminals.ok, true, terminals.code);
  assert.equal(terminals.terminals.release.args.lower, '40000');
  assert.equal(terminals.terminals.release.args.upper, '99999');
  assert.equal(terminals.terminals.refund.args.lower, '100000');
  assert.equal(terminals.terminals.refund.args.upper, '159999');
  for (const kind of ['release', 'refund', 'cancel']) {
    assert.equal(terminals.terminals[kind].redeemer.amount, 100);
    assert.equal(terminals.terminals[kind].financialPost.work.remaining, '975');
    assert.equal(terminals.terminals[kind].financialPost.work.spent, '49');
    assert.equal(terminals.terminals[kind].financialPost.work.closureReserve, '16');
  }
  assert.equal(terminals.residual.code, 'NONEMPTY_OBLIGATIONS');
});

test('economic consumer refuses absent or invalid amounts and obligation encoder refuses incomplete quantities', async () => {
  const { readFileSync } = await import('node:fs');
  const { loadSourceTerminals, datumFromPost, judgeEconomicAction } = await import('../src/native-financial.mjs');
  const { decodePostedDatum } = await import('../src/native-caller.mjs');
  const source = readFileSync('/home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time/escrow-custody-v5.mori', 'utf8');
  const terminal = loadSourceTerminals(source).terminals.release;
  const hashes = { programHash: '11'.repeat(32), boundsHash: '22'.repeat(32), sourceHash: '33'.repeat(32), schemaHash: '44'.repeat(32), seedHash: '55'.repeat(32) };
  const decoded = decodePostedDatum(datumFromPost(hashes, '66'.repeat(32), terminal).plutus, { headQuantity: 1, continuingCash: 0, payoutCash: 100 });
  for (const amount of [undefined, null, '0100', '-1', '1.0', (2n ** 128n).toString()]) {
    const result = judgeEconomicAction(decoded, 'release', { redeemer: { ...terminal.redeemer, amount } });
    assert.equal(result.ok, false, String(amount));
    assert.ok(result.mismatches.includes('amount'));
  }
  for (const amount of [100, '100']) assert.equal(judgeEconomicAction(decoded, 'release', { redeemer: { ...terminal.redeemer, amount } }).ok, true);
  const incomplete = { ...terminal, financialPost: { ...terminal.financialPost, obligations: [{ principal: '1', accrued: '0', outstanding: '1' }] } };
  assert.equal(datumFromPost(hashes, '66'.repeat(32), incomplete).plutus, null);
});

test('original pair race refuses missing loser receipt, ambiguous winner and wrong complete post', async () => {
  const { readFileSync, chmodSync } = await import('node:fs');
  const { loadSourceTerminals, datumFromPost, expectedFrame } = await import('../src/native-financial.mjs');
  const { decodePostedDatum, runOwnedRace } = await import('../src/native-caller.mjs');
  const root = mkdtempSync(join(tmpdir(), 'ra-'));
  const source = readFileSync('/home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time/escrow-custody-v5.mori', 'utf8');
  const terminals = loadSourceTerminals(source).terminals;
  const hashes = { programHash: '11'.repeat(32), boundsHash: '22'.repeat(32), sourceHash: '33'.repeat(32), schemaHash: '44'.repeat(32), seedHash: '55'.repeat(32) };
  const identity = { ...hashes, instanceId: '66'.repeat(32) };
  const decoded = decodePostedDatum(datumFromPost(hashes, identity.instanceId, terminals.release).plutus, { headQuantity: 1, continuingCash: 0, payoutCash: 100 });
  const bin = join(root, 'cli');
  const log = join(root, 'order');
  const mode = join(root, 'mode');
  writeFileSync(bin, `#!/usr/bin/env node
import { readFileSync, appendFileSync, writeFileSync } from 'node:fs';
const argv=process.argv.slice(2), mode=readFileSync(${JSON.stringify(mode)}, 'utf8');
const tag=argv.includes('submit')?'submit':argv.includes('txid')||argv.includes('query')?'poll':'other';
appendFileSync(${JSON.stringify(log)}, tag+'\\n');
if(argv.includes('--out-file'))writeFileSync(argv[argv.indexOf('--out-file')+1], '{}');
if(argv.includes('submit')&&argv[argv.indexOf('--tx-file')+1].includes('Borrower')&&mode!=='missing') {process.stderr.write('BadInputsUTxO');process.exitCode=1;}
if(argv.includes('txid'))process.stdout.write(argv[argv.indexOf('--tx-file')+1].includes('Lender')?'${'aa'.repeat(32)}':'${'bb'.repeat(32)}');
if(argv.includes('--tx-in')) {
 const ref=argv[argv.indexOf('--tx-in')+1];
 if(ref.startsWith('${'aa'.repeat(32)}')||mode==='ambiguous') {
  const decoded=${JSON.stringify(decoded)};
  if(mode==='wrong')decoded.work.remaining=974;
  if(mode==='identity')decoded.source_hash='${'77'.repeat(32)}';
  process.stdout.write(JSON.stringify({[ref]:{decoded}}));
 }
}
`);
  chmodSync(bin, 0o755);
  const key=join(root,'unit.skey');writeFileSync(key,'UNIT fixture');chmodSync(key,0o600);
  const bodies=['release','cancel'].map(name=>{const p=join(root,name+'.body');writeFileSync(p,'UNIT');return p;});
  for (const [variant, code] of [['good','RACE_UNIT_OBSERVATION'],['missing','RACE_LOSER_RECEIPT_ABSENT'],['ambiguous','RACE_WINNER_AMBIGUOUS'],['wrong','RACE_POST_MISMATCH'],['identity','RACE_POST_MISMATCH']]) {
    writeFileSync(mode,variant);writeFileSync(log,'');
    const ref=(hex)=>({txId:hex.repeat(32),index:0});
    const result=await runOwnedRace({signBin:bin,submitBin:bin,pollBin:bin,logPath:log,networkMagic:764,head:ref('44'),
      left:{actor:'Lender',resource:'release',fee:ref('55'),collateral:ref('66'),expectedFrame:expectedFrame(terminals.release,identity),redeemer:terminals.release.redeemer},
      right:{actor:'Borrower',resource:'cancel',fee:ref('77'),collateral:ref('88'),expectedFrame:expectedFrame(terminals.cancel,identity),redeemer:terminals.cancel.redeemer},
      leftBodyFile:bodies[0],rightBodyFile:bodies[1],signingKeyPath:key,overlap:'release-cancel'});
    assert.equal(result.code,code,variant);
    const events=readFileSync(log,'utf8').trim().split('\n');
    assert.equal(events.filter(x=>x==='submit').length,2);
    assert.ok(events.lastIndexOf('submit')<events.indexOf('poll'));
    assert.equal(result.resubmitted,false);
    assert.equal(result.ledgerExecution,false);
    if(variant==='good')assert.equal(result.postCode,'ECONOMIC_ACTION_MATCHED');
  }
});

test('production capture refuses a caller-supplied compiler before any real command', async () => {
  const { executeParityCapture } = await import('../src/posix-parity-harness.mjs');
  const root=mkdtempSync(join(tmpdir(),'hc-'));
  const review=join(root,'review');writeFileSync(review,'UNIT refusal-only review file');
  const socket=join(root,'socket');writeFileSync(socket,'UNIT non-socket');
  let called=false;
  const result=executeParityCapture({armed:true,cliPath:'/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-cli',
    socketPath:socket,reviewA:review,reviewB:review,compile:()=>{called=true;return {ok:true};}});
  assert.equal(result.code,'COMPILE_CALLBACK_NOT_PRODUCTION');
  assert.equal(result.networkCommandsRun,false);
  assert.equal(called,false);
});
