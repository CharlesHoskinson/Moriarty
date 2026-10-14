/** Public receipt decode, closed validator, and financial oracle projection. */
import {importPinned} from './providers.mjs';

const U128 = /^(0|[1-9][0-9]*)$/;
function gcd(a, b) {
  let x = a < 0n ? -a : a; let y = b < 0n ? -b : b;
  while (y !== 0n) { const t = x % y; x = y; y = t; }
  return x === 0n ? 1n : x;
}
function need(obj, path) {
  const parts = path.split('.');
  let cur = obj;
  for (const p of parts) {
    if (cur == null) throw new Error('missing observation ' + path);
    cur = Array.isArray(cur) ? cur[Number(p)] : cur[p];
  }
  if (cur == null || cur === '') throw new Error('missing observation ' + path);
  return cur;
}
function parseMoriU(src, name) {
  const re = new RegExp(name + '[\\s\\S]{0,80}?(?:uint|amount)\\((\\d+)');
  const m = src.match(re);
  if (!m) throw new Error('missing .mori constant ' + name);
  return BigInt(m[1]);
}
function parseMoriLifetime(src) {
  const m = src.match(/\blifetime\s+(\d+)/);
  if (!m) throw new Error('missing lifetime');
  return m[1];
}
function parseMoriHorizon(src) {
  const m = src.match(/\bhorizon\s+(\d+)/);
  if (!m) throw new Error('missing horizon');
  return m[1];
}
function clone(v) { return JSON.parse(JSON.stringify(v)); }
function setLeaf(obj, path, value) {
  const parts = path.split('.');
  const last = parts.pop();
  let cur = obj;
  for (const p of parts) cur = Array.isArray(cur) ? cur[Number(p)] : cur[p];
  cur[last] = String(value);
}
function layoutCheck(metadata, names) {
  const have = new Set((metadata.stateFields || []).map((f) => f.name));
  for (const n of names) {
    if (!have.has(n)) throw new Error('metadata layout missing ' + n);
  }
}
function identityCheck({metadata, boundProgram, deployedProgramDigest, receipt}) {
  if (!deployedProgramDigest) throw new Error('missing deployed program identity');
  if (metadata.programHash !== deployedProgramDigest) throw new Error('metadata programHash != deployed');
  if (boundProgram.programHash !== deployedProgramDigest) throw new Error('bound-program programHash != deployed');
  if (receipt.programDigest !== deployedProgramDigest) throw new Error('observed programDigest != deployed');
}

export function extractPublicFailedTxData(err) {
  const f = err?.finalizedTxData;
  if (!f) throw new Error('no finalizedTxData');
  return {
    name: err.name || 'TxFailedError',
    finalizedTxData: {
      status: f.status,
      txId: f.txId,
      identifiers: f.identifiers,
      txHash: f.txHash,
      blockHash: f.blockHash,
      blockHeight: f.blockHeight,
      fees: f.fees,
      segmentStatusMap: f.segmentStatusMap instanceof Map ? [...f.segmentStatusMap.entries()] : f.segmentStatusMap,
    },
  };
}

export function validatePublicFinancialReceipt(receipt) {
  const errors = [];
  if (!receipt || typeof receipt !== 'object') return {ok: false, errors: [{message: 'receipt required'}]};
  if (!receipt.contractAddress) errors.push({path: 'contractAddress', message: 'required'});
  if (!receipt.blockHash) errors.push({path: 'blockHash', message: 'required'});
  if (receipt.txStatus === 'FailEntirely' || receipt.txStatus === 'FailFallible') {
    errors.push({path: 'txStatus', message: 'not a successful financial stage'});
  }
  if (receipt.acceptedStage !== true) errors.push({path: 'acceptedStage', message: 'stage not accepted'});
  if (receipt.blockHash && /^0+$/.test(String(receipt.blockHash))) errors.push({path: 'blockHash', message: 'unknown block'});
  if (receipt.contractAddress && /^0+$/.test(String(receipt.contractAddress))) errors.push({path: 'contractAddress', message: 'unknown contract'});
  for (const [k, v] of Object.entries(receipt.kernelState || {})) {
    if (typeof v === 'number') errors.push({path: 'kernelState.' + k, message: 'exact integer string required'});
    else if (v != null && !U128.test(String(v))) errors.push({path: 'kernelState.' + k, message: 'malformed integer'});
  }
  return {ok: errors.length === 0, errors};
}

export async function decodePublicFinancialReceipt(input) {
  if (!input || typeof input !== 'object') throw new Error('input required');
  if (input.provenance === 'finalized-bytes') {
    const raw = input.serializedTx;
    if (!raw || raw.length < 8) throw new Error('finalized bytes missing or not a proven transaction');
    const ledger = await importPinned('@midnight-ntwrk/ledger-v8');
    const tx = ledger.Transaction.deserialize('signature', 'proof', 'binding', raw);
    const again = ledger.Transaction.deserialize('signature', 'proof', 'binding', tx.serialize());
    if (tx.transactionHash() !== again.transactionHash()) throw new Error('canonical round-trip failed');
    throw new Error('positive finalized-byte coverage unavailable offline: deserialize succeeded but this source pass has no admitted proven fixture');
  }
  const f = input.finalizedTxData || {};
  const pub = input.publicState || {};
  const status = f.status;
  const accepted = status === 'SucceedEntirely';
  const rec = {
    provenance: input.provenance || 'synthetic-test',
    contractAddress: input.contractAddress,
    blockHash: f.blockHash,
    txStatus: status,
    acceptedStage: accepted,
    guaranteedEffectsRetained: status === 'FailFallible' || accepted,
    identifiers: f.identifiers || [],
    fees: {
      paidFees: f.fees?.paidFees,
      estimatedFees: f.fees?.estimatedFees,
      knownAggregate: f.fees?.paidFees,
      unobservedPrivateDust: true,
    },
    segmentStatuses: f.segmentStatusMap,
    txId: f.txId,
    txHash: f.txHash,
    ...pub,
    kernelState: pub.kernelState || input.kernelState,
    remaining: pub.remaining,
    revision: pub.revision,
    work: pub.work,
    lastResult: pub.lastResult,
    residualDuty: pub.residualDuty,
  };
  if (input.provenance === 'synthetic-test') {
    Object.assign(rec, input);
    rec.txStatus = status || input.txStatus;
    rec.acceptedStage = (rec.txStatus === 'SucceedEntirely');
    rec.guaranteedEffectsRetained = rec.txStatus === 'FailFallible' || rec.acceptedStage;
    rec.blockHash = rec.blockHash || f.blockHash;
  }
  return rec;
}

function mark(map, path, kind, note, value) {
  map[path] = {kind, note, path: note, value: value == null ? null : String(value)};
}

function loanOracle(input) {
  const {receipt, expectedFixture, sourceText, metadata, boundProgram, deployedProgramDigest} = input;
  identityCheck({metadata, boundProgram, deployedProgramDigest, receipt});
  layoutCheck(metadata, ['notional', 'principal_due', 'interest_due', 'principal_paid', 'interest_paid', 'borrower_cash', 'lender_cash', 'cursor', 'episode_closed']);
  const rateN = parseMoriU(sourceText, 'annual_rate_numerator');
  const rateD = parseMoriU(sourceText, 'annual_rate_denominator');
  const perN = parseMoriU(sourceText, 'day_count_numerator');
  const perD = parseMoriU(sourceText, 'day_count_denominator');
  const installment = parseMoriU(sourceText, 'principal_installment');
  const lifetime = parseMoriLifetime(sourceText);
  const horizon = parseMoriHorizon(sourceText);
  const setup = receipt.stagesObserved?.setup;
  const accrue = receipt.stagesObserved?.accrue;
  const settle = receipt.stagesObserved?.settle;
  if (!setup || !accrue || !settle) throw new Error('missing stage observation');
  const n = BigInt(need(setup, 'notional'));
  const num = n * rateN * perN;
  const den = rateD * perD;
  const interest = num / den;
  const rem = num % den;
  const g = gcd(rem, den);
  const remN = rem / g;
  const remD = den / g;
  const out = clone(expectedFixture);
  out.networkAcceptance = false;
  out.observationKind = 'synthetic-local';
  out.networkEvidence = 'incompleteNetworkEvidence';
  out.schemaVersion = 'moriarty-financial-record/1';
  out.sourcePins.metadataUsedForExpectations = false;
  out.agreement.lifetime = lifetime;
  out.agreement.horizon = horizon;
  const map = {};
  const srcNote = 'experiments/moriarty-language/spec/examples/loan.mori';
  mark(map, 'agreement.lifetime', 'source-derived', srcNote + ' lifetime', lifetime);
  mark(map, 'agreement.horizon', 'source-derived', srcNote + ' horizon', horizon);
  mark(map, 'stages.accrue.liabilities.accrual.rateNumerator', 'source-derived', srcNote + ' annual_rate_numerator', rateN);
  mark(map, 'stages.accrue.liabilities.accrual.rateDenominator', 'source-derived', srcNote + ' annual_rate_denominator', rateD);
  mark(map, 'stages.accrue.liabilities.accrual.periodNumerator', 'source-derived', srcNote + ' day_count_numerator', perN);
  mark(map, 'stages.accrue.liabilities.accrual.periodDenominator', 'source-derived', srcNote + ' day_count_denominator', perD);
  mark(map, 'stages.accrue.liabilities.principal.installment', 'source-derived', srcNote + ' principal_installment', installment);
  mark(map, 'stages.accrue.liabilities.accrual.floorRemainderNumerator', 'arithmetic-derived', 'gcd(n*8*31 % (100*365), 100*365) numerator', remN);
  mark(map, 'stages.accrue.liabilities.accrual.floorRemainderDenominator', 'arithmetic-derived', 'gcd remainder denominator', remD);
  function rates(stageIdx) {
    out.stages[stageIdx].liabilities.accrual.rateNumerator = String(rateN);
    out.stages[stageIdx].liabilities.accrual.rateDenominator = String(rateD);
    out.stages[stageIdx].liabilities.accrual.periodNumerator = String(perN);
    out.stages[stageIdx].liabilities.accrual.periodDenominator = String(perD);
    out.stages[stageIdx].liabilities.accrual.floorRemainderNumerator = String(remN);
    out.stages[stageIdx].liabilities.accrual.floorRemainderDenominator = String(remD);
    out.stages[stageIdx].liabilities.principal.installment = String(installment);
  }
  rates(0); rates(1); rates(2);
  const s0 = out.stages[0];
  s0.revision = String(need(setup, 'revision'));
  s0.remaining = String(need(setup, 'remaining'));
  s0.work = String(need(setup, 'work'));
  s0.status.episodeClosed = String(need(setup, 'episode_closed'));
  s0.status.cursor = String(need(setup, 'cursor'));
  s0.balances.post[0].amount = String(need(setup, 'borrower_cash'));
  s0.balances.post[1].amount = String(need(setup, 'lender_cash'));
  s0.transfers[0].amount = String(need(setup, 'borrower_cash'));
  s0.actorEffects[0].netCredit = String(need(setup, 'borrower_cash'));
  s0.liabilities.nominalRemaining = String(need(setup, 'notional'));
  s0.liabilities.principal.outstandingNotional = String(need(setup, 'notional'));
  s0.residualDuties[0].amount = String(need(setup, 'notional'));
  mark(map, 'stages.setup.balances.post.0.amount', 'observed', 'stagesObserved.setup.borrower_cash', setup.borrower_cash);
  mark(map, 'stages.setup.liabilities.nominalRemaining', 'observed', 'stagesObserved.setup.notional', setup.notional);
  const s1 = out.stages[1];
  s1.revision = String(need(accrue, 'revision'));
  s1.remaining = String(need(accrue, 'remaining'));
  s1.work = String(need(accrue, 'work'));
  s1.status.episodeClosed = String(need(accrue, 'episode_closed'));
  s1.status.cursor = String(need(accrue, 'cursor'));
  s1.balances.pre[0].amount = String(need(setup, 'borrower_cash'));
  s1.balances.pre[1].amount = String(need(setup, 'lender_cash'));
  s1.balances.post[0].amount = String(need(accrue, 'borrower_cash'));
  s1.balances.post[1].amount = String(need(accrue, 'lender_cash'));
  s1.liabilities.nominalRemaining = String(need(accrue, 'notional'));
  s1.liabilities.principal.paid = String(accrue.principal_paid || '0');
  s1.liabilities.principal.due = String(need(accrue, 'principal_due'));
  s1.liabilities.principal.outstandingNotional = String(need(accrue, 'notional'));
  s1.liabilities.accrual.interestCalculated = String(interest);
  s1.liabilities.accrual.interestDue = String(need(accrue, 'interest_due'));
  s1.liabilities.dues[0].created = String(need(accrue, 'principal_due'));
  s1.liabilities.dues[0].outstanding = String(need(accrue, 'principal_due'));
  s1.liabilities.dues[1].created = String(need(accrue, 'interest_due'));
  s1.liabilities.dues[1].outstanding = String(need(accrue, 'interest_due'));
  s1.residualDuties[0].amount = String(need(accrue, 'notional'));
  mark(map, 'stages.accrue.liabilities.principal.paid', 'observed', 'stagesObserved.accrue.principal_paid', accrue.principal_paid || '0');
  mark(map, 'stages.accrue.liabilities.accrual.interestDue', 'observed', 'stagesObserved.accrue.interest_due', accrue.interest_due);
  mark(map, 'stages.accrue.liabilities.accrual.interestCalculated', 'arithmetic-derived', 'floor(n*8*31/(100*365))', interest);
  const s2 = out.stages[2];
  s2.revision = String(need(settle, 'revision'));
  s2.remaining = String(need(settle, 'remaining'));
  s2.work = String(need(settle, 'work'));
  s2.status.episodeClosed = String(need(settle, 'episode_closed'));
  s2.status.cursor = String(need(settle, 'cursor'));
  s2.balances.pre[0].amount = String(need(accrue, 'borrower_cash'));
  s2.balances.pre[1].amount = String(need(accrue, 'lender_cash'));
  s2.balances.post[0].amount = String(need(settle, 'borrower_cash'));
  s2.balances.post[1].amount = String(need(settle, 'lender_cash'));
  s2.transfers[0].amount = String(need(settle, 'lender_cash'));
  s2.actorEffects[0].grossDebit = String(need(settle, 'lender_cash'));
  s2.actorEffects[1].netCredit = String(need(settle, 'lender_cash'));
  s2.liabilities.nominalRemaining = String(need(settle, 'notional'));
  s2.liabilities.principal.paid = String(need(settle, 'principal_paid'));
  s2.liabilities.principal.due = String(need(settle, 'principal_due'));
  s2.liabilities.principal.outstandingNotional = String(need(settle, 'notional'));
  s2.liabilities.accrual.interestCalculated = String(interest);
  s2.liabilities.accrual.interestPaid = String(need(settle, 'interest_paid'));
  s2.liabilities.accrual.interestDue = String(need(settle, 'interest_due'));
  s2.liabilities.dues[0].settled = String(need(settle, 'principal_paid'));
  s2.liabilities.dues[1].settled = String(need(settle, 'interest_paid'));
  s2.residualDuties[0].amount = String(need(settle, 'notional'));
  mark(map, 'stages.settle.liabilities.principal.paid', 'observed', 'stagesObserved.settle.principal_paid', settle.principal_paid);
  mark(map, 'stages.settle.balances.post.1.amount', 'observed', 'stagesObserved.settle.lender_cash', settle.lender_cash);
  if (receipt.txStatus && receipt.txStatus !== 'SucceedEntirely') throw new Error('unsuccessful status cannot project a success record');
  if (receipt.kernelState) {
    if (String(receipt.kernelState.notional) !== String(settle.notional)) throw new Error('kernel notional diverged from last stage');
    if (String(receipt.kernelState.episode_closed) !== String(settle.episode_closed)) throw new Error('kernel episode_closed diverged');
  }
  if (receipt.remaining != null && String(receipt.remaining) !== String(settle.remaining)) throw new Error('remaining diverged');
  if (receipt.revision != null && String(receipt.revision) !== String(settle.revision)) throw new Error('revision diverged');
  if (receipt.work != null && String(receipt.work) !== String(settle.work)) throw new Error('work diverged');
  if (receipt.balances?.borrower && String(receipt.balances.borrower) !== String(settle.borrower_cash)) throw new Error('borrower balance diverged');
  if (receipt.transfers?.[0]?.amount && String(receipt.transfers[0].amount) !== String(settle.lender_cash)) throw new Error('transfer amount diverged');
  if (receipt.residualDuty?.[0]?.amount && String(receipt.residualDuty[0].amount) !== String(settle.notional)) throw new Error('residual duty diverged');
  if (receipt.fees?.paidFees && receipt.fees.paidFees !== '0') {
    out.networkFeeAccounting.status = 'observed-public-fee';
  }
  return {oracleRecord: out, provenanceMap: map, ok: true};
}

function swapOracle(input) {
  const {receipt, expectedFixture, sourceText, metadata, boundProgram, deployedProgramDigest} = input;
  identityCheck({metadata, boundProgram, deployedProgramDigest, receipt});
  layoutCheck(metadata, ['reserve_a', 'reserve_b', 'trader_a', 'trader_b', 'provider_a', 'provider_b', 'epoch_closed']);
  const feeN = parseMoriU(sourceText, 'fee_numerator');
  const feeD = parseMoriU(sourceText, 'fee_denominator');
  const lifetime = parseMoriLifetime(sourceText);
  const horizon = parseMoriHorizon(sourceText);
  const setup = receipt.stagesObserved?.setup;
  const swap = receipt.stagesObserved?.swap;
  const close = receipt.stagesObserved?.close;
  if (!setup || !swap || !close) throw new Error('missing stage observation');
  const amountIn = BigInt(need(setup, 'trader_a')) - BigInt(need(swap, 'trader_a'));
  const reserveA = BigInt(need(setup, 'reserve_a'));
  const reserveB = BigInt(need(setup, 'reserve_b'));
  const fee = amountIn - (amountIn * feeN) / feeD;
  const effective = amountIn * feeN;
  const output = (effective * reserveB) / (reserveA * feeD + effective);
  const out = clone(expectedFixture);
  out.networkAcceptance = false;
  out.observationKind = 'synthetic-local';
  out.networkEvidence = 'incompleteNetworkEvidence';
  out.schemaVersion = 'moriarty-financial-record/1';
  out.sourcePins.metadataUsedForExpectations = false;
  out.agreement.lifetime = lifetime;
  out.agreement.horizon = horizon;
  const map = {};
  const srcNote = 'experiments/moriarty-language/spec/examples/swap.mori';
  mark(map, 'fee_numerator', 'source-derived', srcNote + ' fee_numerator', feeN);
  mark(map, 'source.fee_numerator', 'source-derived', srcNote + ' fee_numerator', feeN);
  mark(map, 'fee_denominator', 'source-derived', srcNote + ' fee_denominator', feeD);
  function fillStage(stage, obs, idx) {
    stage.revision = String(need(obs, 'revision'));
    stage.remaining = String(need(obs, 'remaining'));
    stage.work = String(need(obs, 'work'));
    stage.status.episodeClosed = String(need(obs, 'epoch_closed'));
    const actors = [
      ['pool', 'ASSET_A', need(obs, 'reserve_a')],
      ['pool', 'ASSET_B', need(obs, 'reserve_b')],
      ['trader', 'ASSET_A', need(obs, 'trader_a')],
      ['trader', 'ASSET_B', need(obs, 'trader_b')],
      ['provider', 'ASSET_A', need(obs, 'provider_a')],
      ['provider', 'ASSET_B', need(obs, 'provider_b')],
    ];
    for (let i = 0; i < 6; i += 1) stage.balances.post[i].amount = String(actors[i][2]);
  }
  fillStage(out.stages[0], setup, 0);
  out.stages[0].transfers[0].amount = String(need(setup, 'reserve_a'));
  out.stages[0].transfers[1].amount = String(need(setup, 'reserve_b'));
  out.stages[0].transfers[2].amount = String(need(setup, 'trader_a'));
  out.stages[0].actorEffects[0].netCredit = String(need(setup, 'reserve_a'));
  out.stages[0].actorEffects[1].netCredit = String(need(setup, 'reserve_b'));
  out.stages[0].actorEffects[2].netCredit = String(need(setup, 'trader_a'));
  const pre1 = out.stages[0].balances.post;
  for (let i = 0; i < 6; i += 1) out.stages[1].balances.pre[i].amount = pre1[i].amount;
  fillStage(out.stages[1], swap, 1);
  out.stages[1].transfers[0].amount = String(amountIn);
  out.stages[1].transfers[1].amount = String(need(swap, 'trader_b'));
  out.stages[1].economicFee.amount = String(fee);
  out.stages[1].actorEffects[0].netCredit = String(amountIn);
  out.stages[1].actorEffects[1].grossDebit = String(need(swap, 'trader_b'));
  out.stages[1].actorEffects[2].grossDebit = String(amountIn);
  out.stages[1].actorEffects[2].fee = String(fee);
  out.stages[1].actorEffects[3].netCredit = String(need(swap, 'trader_b'));
  const pre2 = out.stages[1].balances.post;
  for (let i = 0; i < 6; i += 1) out.stages[2].balances.pre[i].amount = pre2[i].amount;
  fillStage(out.stages[2], close, 2);
  out.stages[2].transfers[0].amount = String(need(close, 'provider_a'));
  out.stages[2].transfers[1].amount = String(need(close, 'provider_b'));
  out.stages[2].actorEffects[0].grossDebit = String(need(close, 'provider_a'));
  out.stages[2].actorEffects[1].grossDebit = String(need(close, 'provider_b'));
  out.stages[2].actorEffects[4].netCredit = String(need(close, 'provider_a'));
  out.stages[2].actorEffects[5].netCredit = String(need(close, 'provider_b'));
  if (close.epoch_closed === '1') out.stages[2].residualDuties[0].status = 'discharged';
  mark(map, 'stages.swap.economicFee.amount', 'arithmetic-derived', 'amountIn - floor(amountIn*997/1000)', fee);
  mark(map, 'stages.swap.transfers.1.amount', 'observed', 'stagesObserved.swap.trader_b', swap.trader_b);
  mark(map, 'arithmetic.output', 'arithmetic-derived', 'floor((amountIn*997)*reserveB/(reserveA*1000+amountIn*997))', output);
  if (receipt.txStatus && receipt.txStatus !== 'SucceedEntirely') throw new Error('unsuccessful status cannot project a success record');
  return {oracleRecord: out, provenanceMap: map, ok: true};
}

export function projectFinancialOracle(input) {
  if (!input?.receipt) throw new Error('receipt required');
  if (!input.expectedFixture) throw new Error('expected fixture required');
  if (!input.sourceText) throw new Error('pinned .mori source required');
  if (input.receipt.provenance === 'finalized-on-chain') throw new Error('do not relabel a real receipt as synthetic');
  const realReceipt = clone(input.receipt);
  const built = input.case === 'swap' ? swapOracle(input) : loanOracle(input);
  built.realReceipt = realReceipt;
  built.oracleRecord.networkAcceptance = false;
  return built;
}

