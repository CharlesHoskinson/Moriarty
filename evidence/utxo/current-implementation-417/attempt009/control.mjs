/**
 * Red control for Q2, Q1, Q4 against the live exported consumers.
 * Unit context only. No node, keygen, fund, mint, sign, submit, or query.
 */
import { createHash } from 'node:crypto';
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const ATTEMPT = "/home/charl/moriarty-eutxo-validation-2026-10-12/t1-gpt61-implementation-417/attempt009";
const SRC = '/home/charl/Moriarty/.worktrees/eutxo-candidate-219/experiments/moriarty-eutxo/src';
const WORD64_MAX = 18446744073709551615n;
const UINT128_MAX = 340282366920938463463374607431768211455n;
const QUANTITIES = {
  enterprise: '13500000000000000',
  base: '27000000000000000',
  safe: '9007199254740991',
  safePlus1: '9007199254740992',
  safePlus2: '9007199254740993',
  digits15: '999999999999999',
  digits16: '1000000000000000',
  word64Max: WORD64_MAX.toString(),
  word64MaxPlus: (WORD64_MAX + 1n).toString(),
  uint128Max: UINT128_MAX.toString(),
  uint128MaxPlus: (UINT128_MAX + 1n).toString(),
};

const failures = [];
function check(id, ok, detail) {
  const row = { id, ok: ok === true, detail: detail ?? null };
  if (!row.ok) failures.push(row);
  return row;
}
function digitsOf(value) {
  if (value == null) return null;
  if (typeof value === 'bigint') return value.toString();
  if (typeof value === 'number') return Number.isFinite(value) ? String(value) : null;
  if (typeof value === 'string') return value;
  return null;
}
function isNullQuantityFrame(decoded) {
  if (!decoded || decoded.ok === false) return false;
  if (decoded.ordinary_locked == null) return true;
  if (decoded.balances && decoded.balances.Escrow == null) return true;
  return false;
}
function utf8(text) {
  return { bytes: Buffer.from(text, 'utf8').toString('hex') };
}
function datum(quantity, workSpent = '1') {
  const bytes = 'cd'.repeat(32);
  const int = (value) => ({ int: value });
  const balance = (party, asset, amount) => ({ fields: [utf8(party), utf8(asset), int(amount)] });
  return {
    fields: [
      { bytes }, { bytes }, { bytes }, { bytes }, { bytes },
      utf8('unit-network'), utf8('unit-deployment'), { bytes },
      int('1'), int('2'), int(quantity), int('4'),
      { fields: [
        utf8('moriarty-financial-lifecycle-state/1'),
        { list: [balance('Escrow', 'Cash', quantity)] },
        { list: [] }, { list: [] }, { list: [] }, { list: [] }, { list: [] }, { list: [] },
        { fields: [int('10'), int(workSpent), int('2')] },
      ] },
    ],
  };
}
function txOuts(argv) {
  const outs = [];
  if (!Array.isArray(argv)) return outs;
  for (let i = 0; i < argv.length; i += 1) {
    if (argv[i] === '--tx-out') outs.push(argv[i + 1]);
  }
  return outs;
}
function expectCoin(room, input, each) {
  const amount = BigInt(input);
  const roleSum = each * 3n;
  const change = amount - roleSum - 1n;
  return room
    && room.ok === true
    && room.code === 'FUNDING_FEE_ROOM_OK'
    && digitsOf(room.eachText ?? room.each) === each.toString()
    && digitsOf(room.roleSumText ?? room.roleSum) === roleSum.toString()
    && digitsOf(room.feeText ?? room.feeLovelace) === '1'
    && digitsOf(room.changeRoomText ?? room.changeRoom) === change.toString()
    && each * 3n + 1n + change === amount
    && room.ledgerFunding !== true
    && room.fundedInNodeInitialFunds !== true;
}

const pins = ['native-bootstrap.mjs', 'native-producer.mjs', 'native-caller.mjs', 'native-financial.mjs'].map((name) => {
  const path = join(SRC, name);
  const bytes = readFileSync(path);
  return { path, bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') };
});
writeFileSync(join(ATTEMPT, 'source-pins-before.json'), `${JSON.stringify(pins, null, 2)}\n`);

const bootstrap = await import(pathToFileURL(join(SRC, 'native-bootstrap.mjs')).href);
const producer = await import(pathToFileURL(join(SRC, 'native-producer.mjs')).href);
const caller = await import(pathToFileURL(join(SRC, 'native-caller.mjs')).href);
const financial = await import(pathToFileURL(join(SRC, 'native-financial.mjs')).href);

const addrDir = join(ATTEMPT, 'unit-addrs');
mkdirSync(addrDir, { recursive: true });
const addresses = {
  funder: join(addrDir, 'funder.addr'),
  lender: join(addrDir, 'lender.addr'),
  borrower: join(addrDir, 'borrower.addr'),
  issuer: join(addrDir, 'issuer.addr'),
};
writeFileSync(addresses.funder, 'addr_test1unitfunder\n');
writeFileSync(addresses.lender, 'addr_test1unitlender\n');
writeFileSync(addresses.borrower, 'addr_test1unitborrower\n');
writeFileSync(addresses.issuer, 'addr_test1unitissuer\n');
const beforeNames = readdirSync(ATTEMPT).sort();
const plan = {
  magic: 764,
  cli: 'cardano-cli',
  paths: {
    addrOuts: [addresses.funder],
    roles: [
      { name: 'lender', addr: addresses.lender },
      { name: 'borrower', addr: addresses.borrower },
      { name: 'issuer', addr: addresses.issuer },
    ],
    socketPath: join(ATTEMPT, 'unit-not-a-socket.sock'),
    fundingBody: join(ATTEMPT, 'funding-body-not-executed.json'),
  },
};
const tx = 'a1'.repeat(32);
const rows = [];

function fundingFor(digits) {
  const stdout = `{"${tx}#0":{"address":"addr_test1unit","value":{"lovelace":${digits}}}}`;
  const parsed = bootstrap.parseQueryUtxoText(stdout);
  const roomText = bootstrap.assessFundingConservation({
    inputLovelace: parsed.lovelace,
    lovelaceText: parsed.lovelaceText,
    feeLovelace: 1,
    roleCount: 3,
  });
  const roomString = bootstrap.assessFundingConservation({
    inputLovelace: digits,
    feeLovelace: 1,
    roleCount: 3,
  });
  const argv = bootstrap.resolveBootstrapArgv({ id: 'funding-build' }, plan, { 'query-funder-utxo': parsed });
  return { parsed, roomText, roomString, argv };
}

const enterprise = fundingFor(QUANTITIES.enterprise);
rows.push(check('q2-parser-text-enterprise', enterprise.parsed.lovelaceText === QUANTITIES.enterprise && enterprise.parsed.lovelace == null, {
  lovelaceText: enterprise.parsed.lovelaceText,
  lovelace: enterprise.parsed.lovelace,
}));
rows.push(check('q2-enterprise-conservation', expectCoin(enterprise.roomText, QUANTITIES.enterprise, 1350000000000000n), {
  code: enterprise.roomText.code,
  reason: enterprise.roomText.reason || null,
  each: digitsOf(enterprise.roomText.eachText ?? enterprise.roomText.each),
  change: digitsOf(enterprise.roomText.changeRoomText ?? enterprise.roomText.changeRoom),
}));
rows.push(check('q2-enterprise-string-input', expectCoin(enterprise.roomString, QUANTITIES.enterprise, 1350000000000000n), {
  code: enterprise.roomString.code,
  reason: enterprise.roomString.reason || null,
}));
const enterpriseOuts = txOuts(enterprise.argv);
rows.push(check('q2-enterprise-argv', Array.isArray(enterprise.argv)
  && enterprise.argv.includes('--testnet-magic')
  && enterprise.argv.includes('764')
  && enterpriseOuts.length === 3
  && enterpriseOuts.every((item) => item.endsWith('+1350000000000000'))
  && new Set(enterpriseOuts.map((item) => item.split('+')[0])).size === 3
  && !enterprise.argv.some((item) => String(item).includes('13500000000000000')), {
  argvIsArray: Array.isArray(enterprise.argv),
  code: enterprise.argv && enterprise.argv.code,
  reason: enterprise.argv && enterprise.argv.reason,
  outs: enterpriseOuts,
}));

const base = fundingFor(QUANTITIES.base);
rows.push(check('q2-base-conservation', expectCoin(base.roomText, QUANTITIES.base, 2700000000000000n), {
  code: base.roomText.code,
  reason: base.roomText.reason || null,
  each: digitsOf(base.roomText.eachText ?? base.roomText.each),
}));
rows.push(check('q2-base-argv', txOuts(base.argv).length === 3 && txOuts(base.argv).every((item) => item.endsWith('+2700000000000000')), {
  code: base.argv && base.argv.code,
  reason: base.argv && base.argv.reason,
  outs: txOuts(base.argv),
}));

const safeRoom = bootstrap.assessFundingConservation({ inputLovelace: 9007199254740991, feeLovelace: 1, roleCount: 3 });
rows.push(check('q2-safe-number', safeRoom.ok === true && digitsOf(safeRoom.eachText ?? safeRoom.each) === (9007199254740991n / 10n).toString(), {
  code: safeRoom.code,
  each: digitsOf(safeRoom.eachText ?? safeRoom.each),
  eachType: typeof safeRoom.each,
}));

const word = fundingFor(QUANTITIES.word64Max);
rows.push(check('q2-word64-max', word.roomText.ok === true && Array.isArray(word.argv) && txOuts(word.argv).every((item) => item.endsWith(`+${(WORD64_MAX / 10n).toString()}`)), {
  code: word.roomText.code,
  reason: word.roomText.reason || null,
  argvCode: word.argv && word.argv.code,
}));

function refusedCoin(label, input) {
  const room = bootstrap.assessFundingConservation({ lovelaceText: input.text, inputLovelace: input.mirror, feeLovelace: input.fee ?? 1, roleCount: 3 });
  rows.push(check(label, room.ok === false && room.code === input.code && (!input.reason || room.reason === input.reason) && room.reason !== 'role-output-zero', {
    code: room.code,
    reason: room.reason || null,
  }));
}
refusedCoin('q2-word64-plus', { text: QUANTITIES.word64MaxPlus, code: 'LEDGER_COIN_REFUSED', reason: 'above-word64' });
refusedCoin('q2-uint128', { text: QUANTITIES.uint128Max, code: 'LEDGER_COIN_REFUSED', reason: 'above-word64' });
refusedCoin('q2-negative', { text: '-1', code: 'LEDGER_COIN_REFUSED', reason: 'negative' });
refusedCoin('q2-leading-zero', { text: '013500000000000000', code: 'LEDGER_COIN_REFUSED', reason: 'noncanonical' });
refusedCoin('q2-decimal', { text: '1.5', code: 'LEDGER_COIN_REFUSED', reason: 'noncanonical' });
refusedCoin('q2-unsafe-number', { mirror: 9007199254740992, code: 'LEDGER_COIN_REFUSED', reason: 'unsafe-number' });
refusedCoin('q2-nonfinite', { mirror: Number.POSITIVE_INFINITY, code: 'LEDGER_COIN_REFUSED', reason: 'nonfinite' });
refusedCoin('q2-mirror-mismatch', { text: QUANTITIES.enterprise, mirror: 1, code: 'LEDGER_COIN_REFUSED', reason: 'mirror-mismatch' });
const matched = bootstrap.assessFundingConservation({
  lovelaceText: QUANTITIES.digits15,
  inputLovelace: 999999999999999,
  feeLovelace: 1,
  roleCount: 3,
});
rows.push(check('q2-mirror-match-15', matched.ok === true && digitsOf(matched.eachText ?? matched.each) === (999999999999999n / 10n).toString(), {
  code: matched.code,
  reason: matched.reason || null,
}));
const hungry = bootstrap.assessFundingConservation({ inputLovelace: 9000000, feeLovelace: 8000000, roleCount: 3 });
rows.push(check('q2-fee-room', hungry.ok === false && hungry.code === 'CONSERVATION_FAILED' && hungry.reason === 'fee-does-not-fit', {
  code: hungry.code,
  reason: hungry.reason || null,
}));
const afterNames = readdirSync(ATTEMPT).sort();
rows.push(check('q2-no-synthetic-keys', beforeNames.join() === afterNames.filter((name) => name !== 'source-pins-before.json').join() || afterNames.includes('unit-addrs'), {
  before: beforeNames,
  after: afterNames,
}));

function expectPreserved(label, quantity, workSpent = '1') {
  const decoded = caller.decodePostedDatum(datum(quantity, workSpent));
  const escrow = decoded && decoded.balances ? decoded.balances.Escrow : null;
  rows.push(check(label, decoded && decoded.ok !== false && !isNullQuantityFrame(decoded)
    && digitsOf(decoded.ordinary_locked) === String(quantity)
    && digitsOf(escrow) === String(quantity)
    && decoded.work && digitsOf(decoded.work.spent) === String(workSpent)
    && Array.isArray(decoded.obligations)
    && Array.isArray(decoded.usedTransferIds)
    && Array.isArray(decoded.usedAllocationIds)
    && Array.isArray(decoded.usedOriginationIds)
    && Array.isArray(decoded.usedAccrualIds), {
    ordinary: digitsOf(decoded && decoded.ordinary_locked),
    escrow: digitsOf(escrow),
    code: decoded && decoded.code,
    nullFrame: isNullQuantityFrame(decoded),
  }));
  return decoded;
}
expectPreserved('q1-enterprise', QUANTITIES.enterprise);
expectPreserved('q1-base', QUANTITIES.base);
expectPreserved('q1-safe-string', QUANTITIES.safe);
expectPreserved('q1-safe-plus1-string', QUANTITIES.safePlus1);
expectPreserved('q1-safe-plus2-string', QUANTITIES.safePlus2);
expectPreserved('q1-digits15', QUANTITIES.digits15);
expectPreserved('q1-digits16', QUANTITIES.digits16);
expectPreserved('q1-word64-max', QUANTITIES.word64Max);
expectPreserved('q1-word64-plus-kernel', QUANTITIES.word64MaxPlus);
expectPreserved('q1-uint128-max', QUANTITIES.uint128Max);
const low = expectPreserved('q1-low-cash', 100, 49);
rows.push(check('q1-low-strict-number', low && low.ordinary_locked === 100 && low.work.spent === 49 && low.balances.Escrow === 100, {
  ordinaryType: typeof (low && low.ordinary_locked),
  spentType: typeof (low && low.work && low.work.spent),
  ordinary: low && low.ordinary_locked,
  spent: low && low.work && low.work.spent,
}));
const numericSafe = caller.decodePostedDatum(datum(9007199254740991));
rows.push(check('q1-safe-number-literal', numericSafe && digitsOf(numericSafe.ordinary_locked) === QUANTITIES.safe && !isNullQuantityFrame(numericSafe), {
  ordinary: digitsOf(numericSafe && numericSafe.ordinary_locked),
}));

function expectKernelRefusal(label, quantity, reason) {
  const decoded = caller.decodePostedDatum(datum(quantity));
  rows.push(check(label, decoded && decoded.ok === false && decoded.code === 'KERNEL_AMOUNT_REFUSED' && decoded.reason === reason && !isNullQuantityFrame(decoded), {
    code: decoded && decoded.code,
    reason: decoded && decoded.reason,
    nullFrame: isNullQuantityFrame(decoded),
    ordinary: decoded && decoded.ordinary_locked,
  }));
}
expectKernelRefusal('q1-uint128-plus', QUANTITIES.uint128MaxPlus, 'above-uint128');
expectKernelRefusal('q1-negative', '-1', 'negative');
expectKernelRefusal('q1-leading-zero', '0100', 'noncanonical');
expectKernelRefusal('q1-decimal', '100.0', 'noncanonical');
expectKernelRefusal('q1-unsafe-number', 9007199254740992, 'unsafe-number');
const nonfinite = caller.decodePostedDatum(datum(Number.NaN));
rows.push(check('q1-nonfinite', nonfinite && nonfinite.ok === false && nonfinite.code === 'KERNEL_AMOUNT_REFUSED' && nonfinite.reason === 'nonfinite' && !isNullQuantityFrame(nonfinite), {
  code: nonfinite && nonfinite.code,
  reason: nonfinite && nonfinite.reason,
}));
const wideCoin = caller.decodePostedDatum(datum(100), { continuingCash: QUANTITIES.word64Max });
rows.push(check('q1-ledger-coin-word64', wideCoin && wideCoin.ok !== false && digitsOf(wideCoin.continuingCash) === QUANTITIES.word64Max && wideCoin.ordinary_locked === 100, {
  code: wideCoin && wideCoin.code,
  continuing: digitsOf(wideCoin && wideCoin.continuingCash),
}));
const overCoin = caller.decodePostedDatum(datum(100), { continuingCash: QUANTITIES.word64MaxPlus });
rows.push(check('q1-ledger-coin-above-word64', overCoin && overCoin.ok === false && overCoin.code === 'WORD64_COIN_REFUSED' && overCoin.reason === 'above-word64' && !isNullQuantityFrame(overCoin), {
  code: overCoin && overCoin.code,
  reason: overCoin && overCoin.reason,
  nullFrame: isNullQuantityFrame(overCoin),
}));
const compared = financial.compareFinancialFrame(
  { balances: { Escrow: '100', Borrower: 0, Lender: 0 }, allowances: {}, obligations: [], work: { spent: '49' }, usedTransferIds: [], continuingCash: 0, payoutCash: '100', ordinary_locked: '0' },
  { balances: { Escrow: 100, Borrower: 0, Lender: 0 }, allowances: {}, obligations: [], work: { spent: 49 }, usedTransferIds: [], continuingCash: '0', payoutCash: 100, ordinary_locked: 0 },
);
rows.push(check('q1-canonical-low-compare', compared.ok === true && compared.code === 'FINANCIAL_FRAME_MATCHED', {
  code: compared.code,
  mismatches: compared.mismatches || null,
}));
const different = financial.compareFinancialFrame(
  { balances: {}, allowances: {}, obligations: [], work: { spent: '0' }, continuingCash: 100, payoutCash: 0 },
  { balances: {}, allowances: {}, obligations: [], work: { spent: '49' }, continuingCash: 0, payoutCash: 100 },
);
rows.push(check('q1-canonical-still-mismatches', different.ok === false && different.code === 'READBACK_FINANCIAL_MISMATCH', {
  code: different.code,
}));

function expectTx(label, lovelace, ok, extra = {}) {
  const txOut = producer.txOutValue({ address: 'addr_test1unit', lovelace });
  const pass = ok
    ? txOut.ok === true && txOut.code === 'TX_OUT_VALUE' && txOut.txOut === `addr_test1unit+${extra.digits ?? lovelace}`
    : txOut.ok === false && txOut.code === 'TX_OUT_LOVELACE_REFUSED' && (!extra.reason || txOut.reason === extra.reason);
  rows.push(check(label, pass, { code: txOut.code, reason: txOut.reason || null, txOut: txOut.txOut || null }));
}
expectTx('q4-tx-enterprise', QUANTITIES.enterprise, true);
expectTx('q4-tx-base', QUANTITIES.base, true);
expectTx('q4-tx-safe-string', QUANTITIES.safe, true);
expectTx('q4-tx-digits16', QUANTITIES.digits16, true);
expectTx('q4-tx-word64', QUANTITIES.word64Max, true);
expectTx('q4-tx-word64-plus', QUANTITIES.word64MaxPlus, false, { reason: 'above-word64' });
expectTx('q4-tx-uint128', QUANTITIES.uint128Max, false, { reason: 'above-word64' });
expectTx('q4-tx-safe-number', 9007199254740991, true, { digits: QUANTITIES.safe });
expectTx('q4-tx-low-number', 2000000, true, { digits: '2000000' });
expectTx('q4-tx-unsafe-number', 9007199254740992, false, { reason: 'unsafe-number' });
expectTx('q4-tx-negative', '-1', false, { reason: 'negative' });
expectTx('q4-tx-leading-zero', '02000000', false, { reason: 'noncanonical' });
expectTx('q4-tx-decimal', '1.5', false, { reason: 'noncanonical' });
expectTx('q4-tx-nonfinite', Number.NaN, false, { reason: 'nonfinite' });

function expectMin(label, digits, ok, reason) {
  const parsed = producer.parseMinRequiredUtxo(`Coin ${digits}\n`);
  const got = parsed.lovelaceText ?? digitsOf(parsed.lovelace);
  const pass = ok
    ? parsed.ok === true && parsed.code === 'MIN_REQUIRED_UTXO_PARSED' && got === digits && parsed.reason !== 'not-safe-integer'
    : parsed.ok === false && parsed.code === 'MIN_REQUIRED_UTXO_SHOW_MISMATCH' && parsed.reason === reason && parsed.reason !== 'not-safe-integer';
  rows.push(check(label, pass, { code: parsed.code, reason: parsed.reason || null, lovelace: got }));
  if (ok && digits.length < 16) {
    rows.push(check(`${label}-safe-number`, parsed.lovelace === Number(digits), { lovelace: parsed.lovelace, type: typeof parsed.lovelace }));
  }
}
expectMin('q4-min-enterprise', QUANTITIES.enterprise, true);
expectMin('q4-min-base', QUANTITIES.base, true);
expectMin('q4-min-safe', QUANTITIES.safe, true);
expectMin('q4-min-digits15', QUANTITIES.digits15, true);
expectMin('q4-min-digits16', QUANTITIES.digits16, true);
expectMin('q4-min-word64', QUANTITIES.word64Max, true);
expectMin('q4-min-word64-plus', QUANTITIES.word64MaxPlus, false, 'above-word64');
expectMin('q4-min-uint128', QUANTITIES.uint128Max, false, 'above-word64');
expectMin('q4-min-leading-zero', '01', false, 'noncanonical');
const small = producer.parseMinRequiredUtxo('Coin 1021470\n');
rows.push(check('q4-min-captured-shape', small.ok === true && small.lovelace === 1021470 && small.minAdaMeasured === false, {
  code: small.code,
  lovelace: small.lovelace,
}));
const planned = producer.planOutputMinimums({
  cli: 'cardano-cli',
  protocolParamsFile: join(ATTEMPT, 'protocol-not-read.json'),
  outputs: [{ address: 'addr_test1unit', lovelace: QUANTITIES.enterprise }],
});
const plannedArgv = planned.steps && planned.steps[0] && planned.steps[0].argv;
rows.push(check('q4-downstream-min-ada', planned.blocker == null && Array.isArray(plannedArgv) && plannedArgv.includes(`addr_test1unit+${QUANTITIES.enterprise}`) && plannedArgv.includes('calculate-min-required-utxo'), {
  blocker: planned.blocker,
  argv: plannedArgv,
}));

const summary = {
  status: failures.length === 0 ? 'green' : 'red',
  failureCount: failures.length,
  checkCount: rows.length,
  failures,
  unitOnly: true,
  ledgerQuery: false,
  signed: false,
  submitted: false,
};
writeFileSync(join(ATTEMPT, 'results.json'), `${JSON.stringify(summary, null, 2)}\n`);
console.log(JSON.stringify({ status: summary.status, failureCount: summary.failureCount, checkCount: summary.checkCount, ids: failures.map((item) => item.id) }, null, 2));
process.exit(failures.length === 0 ? 0 : 1);
