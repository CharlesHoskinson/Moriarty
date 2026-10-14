import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { CARDANO_CLI, ENCODING_VKEY_HEX, encodeUnsignedBody, paymentKeyHashOfVkeyCbor } from '../src/native-body.mjs';
import { initialHeadDatum, runNativeCaller } from '../src/native-caller.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const OUT = join('/home/charl/moriarty-eutxo-validation-2026-10-12/t1-f207', 'checks', 'financial-body');
const CLI = CARDANO_CLI;

function ref(n) {
  return { txId: n.toString(16).padStart(64, 'a'), index: n };
}

function run(args) {
  return spawnSync(CLI, args, { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 });
}

test('quoted asset text is refused before the CLI sees it', () => {
  const refused = encodeUnsignedBody({
    head: ref(1),
    fee: ref(3),
    collateral: ref(4),
    requiredSignerHash: 'ab'.repeat(28),
    outputAssets: `"100 ${'ab'.repeat(28)}.43617368"`,
  });
  assert.equal(refused.code, 'QUOTED_ASSET_TEXT');
  assert.equal(refused.ok, false);
});

test('real cardano-cli accepts the cash mint and the full head-open body', async () => {
  mkdirSync(OUT, { recursive: true });
  const ran = await runNativeCaller({
    sourcePath: join(TASK, 'designers/02-escrow-time/escrow-custody-v5.mori'),
    cashDesign: {
      reviewA: join(TASK, 'cash54-approved56/a/RECEIPT.json'),
      reviewB: join(TASK, 'cash54-approved56/b/RECEIPT.json'),
      candidate: join(TASK, 't1-cash-repair-53/CANDIDATE-SHA256.json'),
      sourcePath: join(TASK, 't1-cash-repair-53/prototype/validators/cash_fixture.ak'),
    },
    utxos: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map(ref),
    networkMagic: 764,
    outputLovelace: '2000000',
    feeLovelace: '200000',
    buildBody: true,
    financialBody: true,
    evidenceRoot: OUT,
    receiptRoot: join(OUT, 'receipts'),
    keepReceipt: false,
    cliPath: CLI,
  });
  const mint = ran.stages.mint;
  const body = ran.stages.body;
  const derived = paymentKeyHashOfVkeyCbor(ENCODING_VKEY_HEX);
  assert.equal(derived, 'f5d0373d489e38050f10f8ccbec9c988a775bad527e7b239f257cb96');
  assert.notEqual(derived, 'ab'.repeat(28));
  const mismatch = encodeUnsignedBody({
    head: ref(1),
    fee: ref(3),
    collateral: ref(4),
    requiredSignerHash: 'ab'.repeat(28),
  });
  assert.equal(mismatch.code, 'KEYHASH_MISMATCH');
  assert.equal(mismatch.spawned, false);
  assert.equal(mismatch.derivedPaymentHash, derived);
  assert.equal(mint?.code, 'UNSIGNED_BODY', JSON.stringify({ code: mint?.code, detail: mint?.detail }));
  assert.equal(mint.paymentKeyHash, derived);
  assert.equal(mint.seedOwner, derived);
  assert.equal(mint.holder, derived);
  assert.equal(mint.outputPaymentHash, derived);
  assert.deepEqual(mint.signatories, [derived]);
  assert.equal(body?.code, 'UNSIGNED_BODY', JSON.stringify({ code: body?.code, detail: body?.detail }));
  assert.equal(mint.nativeBudgetMeasured, false);
  assert.equal(body.nativeBudgetMeasured, false);
  assert.equal(body.quantities.protocolMinAda, 'unmeasured');
  assert.equal(mint.cashCanMint, false);
  assert.equal(mint.ledgerFunding, false);
  assert.equal(body.cashSeedReused, false);
  assert.equal(body.headMintsCash, false);
  assert.equal(body.holding, `${mint.txid}#0`);
  assert.notEqual(body.holding, `${ref(1).txId}#1`);
  assert.equal(Object.keys(body.datumFields).length, 13);
  assert.equal(body.datumFields.transport_remaining, 1);
  assert.equal(body.datumFields.lifecycle.work.spent, 0);
  assert.equal(body.datumFields.lifecycle.work.remaining, 1024);
  assert.equal(ran.stages.posts?.code, 'SOURCE_POSTS_EVALUATED', JSON.stringify({ code: ran.stages.posts?.code, detail: ran.stages.posts?.detail }));
  assert.equal(ran.stages.posts.residual.bodyBuilt, false);
  assert.equal(ran.stages.posts.residual.fenceBypass, false);
  assert.equal(ran.stages.posts.residual.code, 'NONEMPTY_OBLIGATIONS');
  assert.equal(ran.stages.spend?.code, 'SPEND_BODIES_ENCODED', JSON.stringify(ran.stages.spend?.code));
  for (const kind of ['release', 'refund', 'cancel']) {
    const spent = ran.stages.spend[kind];
    assert.equal(spent.code, 'UNSIGNED_BODY', JSON.stringify({ kind, code: spent.code, detail: spent.detail }));
    assert.equal(spent.quantities.continuingCash, '0');
    assert.equal(spent.quantities.payoutCash, '100');
    assert.equal(spent.quantities.headQuantity, '1');
    assert.equal(spent.nativeBudgetMeasured, false);
    assert.equal(spent.quantities.protocolMinAda, 'unmeasured');
    assert.equal(spent.signed, false);
    assert.equal(spent.signedIsProof, false);
  }
  const script = ran.stages.headApply.scriptPath;
  const address = run(['address', 'build', '--payment-script-file', script, '--testnet-magic', '42']);
  assert.equal(address.status, 0);
  assert.equal(body.outputAddress, address.stdout.trim());
  const cashScript = ran.stages.cashCompile.scriptPath;
  const policy = run(['hash', 'script', '--script-file', cashScript]);
  assert.equal(policy.status, 0);
  assert.equal(policy.stdout.trim(), mint.cashPolicyId);
  const headPolicy = run(['hash', 'script', '--script-file', script]);
  assert.equal(headPolicy.status, 0);
  assert.equal(headPolicy.stdout.trim(), body.headPolicyId);
  for (const [name, stage] of [['cash-mint', mint], ['head-open', body]]) {
    const file = join(OUT, `${name}.body.json`);
    writeFileSync(file, JSON.stringify({ type: stage.type, description: 'unsigned offline body', cborHex: stage.cborHex }));
    const txid = run(['conway', 'transaction', 'txid', '--tx-body-file', file, '--output-text']);
    assert.equal(txid.status, 0, txid.stderr);
    assert.equal(txid.stdout.trim(), stage.txid);
    const view = run(['debug', 'transaction', 'view', '--tx-body-file', file]);
    assert.equal(view.status, 0, view.stderr);
    writeFileSync(join(OUT, `${name}.view.json`), view.stdout);
    assert.match(view.stdout, /43617368/);
    if (name === 'head-open') {
      assert.match(view.stdout, /48656164/);
      assert.match(view.stdout, /constructor/);
      assert.match(view.stdout, new RegExp(body.headPolicyId));
    }
  }
  const datum = initialHeadDatum(ran.stages.source.hashes, ran.stages.headApply.instanceId);
  assert.equal(datum.plutus.fields.length, 13);
  writeFileSync(join(OUT, 'head-open.datum.json'), JSON.stringify(datum.plutus));
  const summary = {
    mintTxid: mint.txid,
    headTxid: body.txid,
    cashPolicyId: mint.cashPolicyId,
    headPolicyId: body.headPolicyId,
    instanceId: body.instanceId,
    holding: body.holding,
    nativeBudgetMeasured: false,
    protocolMinAda: body.quantities.protocolMinAda,
    offlineProtocolParamsSha256: body.offlineProtocolParamsSha256,
    offlineExecutionUnits: body.offlineExecutionUnits,
    cashCanMint: false,
    ledgerFunding: false,
    ledgerExecution: false,
    cborSha256: {
      mint: createHash('sha256').update(mint.cborHex).digest('hex'),
      head: createHash('sha256').update(body.cborHex).digest('hex'),
    },
  };
  summary.paymentKeyHash = derived;
  summary.requiredSignerWasAb28 = false;
  writeFileSync(join(OUT, 'financial-body-summary.json'), `${JSON.stringify(summary, null, 2)}\n`);
  const probeInput = join(OUT, 'cash-aligned-input.json');
  mkdirSync(OUT, { recursive: true });
  writeFileSync(probeInput, `${JSON.stringify({
    owner: derived,
    oldSigner: 'ab'.repeat(28),
    policyId: mint.cashPolicyId,
    seedTx: ref(1).txId,
    feeTx: ref(3).txId,
    txid: mint.txid,
    scriptFile: ran.stages.cashCompile.scriptPath,
  })}\n`);
  const probe = spawnSync('python3', [join(TASK, 't1-native-consumer-repair-70/probes/cash-aligned.py'), probeInput], {
    encoding: 'utf8',
    maxBuffer: 32 * 1024 * 1024,
  });
  writeFileSync(join(OUT, 'cash-aligned.stdout.txt'), probe.stdout || '');
  writeFileSync(join(OUT, 'cash-aligned.stderr.txt'), probe.stderr || '');
  assert.equal(probe.status, 0, probe.stderr || probe.stdout);
  const evaluated = JSON.parse(probe.stdout);
  assert.equal(evaluated.aligned, 'OUTER_ACCEPT');
  assert.equal(evaluated.oldSeed, 'OUTER_FAIL');
  assert.equal(evaluated.nativeBudgetMeasured, false);
  assert.equal(evaluated.cashCanMint, false);
  assert.equal(evaluated.ledgerExecution, false);
});
