import assert from 'node:assert/strict';
import {randomBytes} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {dirname, join} from 'node:path';
import test from 'node:test';
import {fileURLToPath, pathToFileURL} from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const bindings = JSON.parse(readFileSync(join(here, 'bindings.json'), 'utf8'));
const artifacts = process.env.MORIARTY_CUSTODY_ARTIFACTS;
assert.ok(artifacts, 'MORIARTY_CUSTODY_ARTIFACTS must identify the compiled artifact directory');

const runtime = await import(pathToFileURL(join(artifacts, 'loan/node_modules/@midnight-ntwrk/compact-runtime/dist/index.js')).href);
const loanMod = await import(pathToFileURL(join(artifacts, 'loan/contract/index.js')).href);
const swapMod = await import(pathToFileURL(join(artifacts, 'swap/contract/index.js')).href);
const B = 1n << 64n;
const TIME = Number(bindings.time.syntheticBlockTime);
const WINDOW = Number(bindings.time.windowSeconds);
const HORIZON = Number(bindings.time.horizon);
const ZERO_KEY = '00'.repeat(32);

function mulHint(a, b) {
  return {aLo: a % B, aHi: a / B, bLo: b % B, bHi: b / B, lo: (a % B) * (b % B) % B, carry: (a % B) * (b % B) / B};
}
function divHint(n, d) {
  const q = n / d, r = n % d;
  return {q, r, product: mulHint(d, q)};
}
function loanAccrueHints() {
  const h0 = mulHint(5000000000n, 8n);
  const h1 = mulHint(40000000000n, 31n);
  const h2 = mulHint(100n, 365n);
  const h3 = divHint(1240000000000n, 36500n);
  return {h0, h1, h2, h3};
}
function swapHints() {
  const h0 = mulHint(10000n, 997n);
  const h1 = mulHint(9970000n, 2000000n);
  const h2 = mulHint(1000000n, 1000n);
  const h3 = divHint(19940000000000n, 1009970000n);
  return {h0, h1, h2, h3};
}
function unusedHints() {
  return {unused: 0n};
}
function hexBytes(hex) {
  return Uint8Array.from(Buffer.from(hex, 'hex'));
}
function addr(hex) {
  return {bytes: hexBytes(hex)};
}
function secret() {
  return randomBytes(32);
}
function tokenType(domainHex) {
  return {tag: 'unshielded', raw: runtime.rawTokenType(hexBytes(domainHex), runtime.dummyContractAddress())};
}
function colorOf(domainHex) {
  return String(runtime.rawTokenType(hexBytes(domainHex), runtime.dummyContractAddress()));
}
function balanceMap(pairs) {
  const m = new Map();
  for (const [domainHex, amount] of pairs) m.set(tokenType(domainHex), amount);
  return m;
}
function balanceOf(block, domainHex) {
  const raw = colorOf(domainHex);
  for (const [k, v] of block.balance.entries()) {
    if (k?.tag === 'unshielded' && String(k.raw) === raw) return BigInt(v);
  }
  return 0n;
}
function context(state, opts = {}) {
  const time = opts.time ?? TIME;
  const err = opts.err ?? Number(bindings.time.syntheticUncertaintySeconds);
  const ctx = runtime.createCircuitContext(
    runtime.dummyContractAddress(), ZERO_KEY, state, {}, undefined, undefined, time,
  );
  // WASM QueryContext.block getter returns a copy. Piecemeal field writes do not persist.
  // compact-runtime createInitialQueryContext replaces the whole block object. Tests do the same.
  // Synthetic balances here are test-context only. They are not ledger UTXOs or payer ownership.
  const block = {
    ...ctx.currentQueryContext.block,
    secondsSinceEpoch: BigInt(time),
    secondsSinceEpochErr: err,
    lastBlockTime: BigInt(time),
    parentBlockHash: '0'.repeat(64),
  };
  if (opts.balance) block.balance = opts.balance;
  ctx.currentQueryContext.block = block;
  const read = ctx.currentQueryContext.block;
  assert.equal(Number(read.secondsSinceEpoch), time);
  assert.equal(read.secondsSinceEpochErr, err);
  assert.equal(read.lastBlockTime, BigInt(time));
  if (opts.balance) {
    for (const [token, amount] of opts.balance.entries()) {
      let got = 0n;
      for (const [k, v] of read.balance.entries()) {
        if (k?.tag === token.tag && String(k.raw) === String(token.raw)) got = BigInt(v);
      }
      assert.equal(got, BigInt(amount), `balance readback ${token.raw}`);
    }
  }
  return ctx;
}
function ledger(mod, state) {
  return mod.ledger(state?.data ?? state);
}
function effects(result) {
  return result.context.currentQueryContext.effects;
}
function mapTotal(map) {
  let n = 0n;
  for (const v of map.values()) n += BigInt(v);
  return n;
}
function claimRows(map) {
  const rows = [];
  for (const [key, amount] of map.entries()) {
    const token = Array.isArray(key) ? key[0] : key;
    const dest = Array.isArray(key) ? key[1] : null;
    rows.push({
      tokenTag: token?.tag ?? null,
      tokenRaw: String(token?.raw ?? ''),
      destTag: dest?.tag ?? null,
      dest: String(dest?.address ?? ''),
      amount: BigInt(amount),
    });
  }
  return rows;
}
function tokenRows(map) {
  const rows = [];
  for (const [k, v] of map.entries()) {
    rows.push({tag: k?.tag ?? null, raw: String(k?.raw ?? k), amount: BigInt(v)});
  }
  return rows;
}
function mintRows(map) {
  const rows = [];
  for (const [k, v] of map.entries()) {
    rows.push({domain: String(k), amount: BigInt(v)});
  }
  return rows;
}
function hasClaim(rows, spec) {
  return rows.some((r) =>
    (spec.amount === undefined || r.amount === spec.amount) &&
    (spec.dest === undefined || r.dest.toLowerCase() === spec.dest.toLowerCase()) &&
    (spec.destTag === undefined || r.destTag === spec.destTag) &&
    (spec.tokenRaw === undefined || r.tokenRaw.toLowerCase() === spec.tokenRaw.toLowerCase()) &&
    (spec.tokenTag === undefined || r.tokenTag === spec.tokenTag)
  );
}
function hasToken(rows, spec) {
  return rows.some((r) =>
    (spec.amount === undefined || r.amount === spec.amount) &&
    (spec.raw === undefined || r.raw.toLowerCase() === spec.raw.toLowerCase()) &&
    (spec.tag === undefined || r.tag === spec.tag)
  );
}

function deployLoan() {
  const contract = new loanMod.Contract({});
  const secrets = {borrower: secret(), lender: secret()};
  const initial = contract.initialState(
    runtime.createConstructorContext({}, ZERO_KEY),
    secrets.borrower, secrets.lender,
    addr(bindings.loan.borrowerAddress), addr(bindings.loan.lenderAddress),
    hexBytes(bindings.loan.programDigest), hexBytes(bindings.networkTag),
  );
  return {contract, secrets, state: initial.currentContractState, program: hexBytes(bindings.loan.programDigest), network: hexBytes(bindings.networkTag)};
}
function deploySwap() {
  const contract = new swapMod.Contract({});
  const secrets = {trader: secret(), provider: secret()};
  const initial = contract.initialState(
    runtime.createConstructorContext({}, ZERO_KEY),
    secrets.trader, secrets.provider,
    addr(bindings.swap.traderAddress), addr(bindings.swap.providerAddress),
    hexBytes(bindings.swap.programDigest), hexBytes(bindings.networkTag),
  );
  return {contract, secrets, state: initial.currentContractState, program: hexBytes(bindings.swap.programDigest), network: hexBytes(bindings.networkTag)};
}
function loanInit(fx, opts = {}) {
  const r = fx.contract.circuits.initialize(
    context(fx.state, opts),
    opts.secret ?? fx.secrets.borrower, opts.program ?? fx.program, opts.network ?? fx.network,
    opts.actor ?? 2n, opts.now ?? BigInt(TIME),
  );
  fx.state = r.context.currentQueryContext.state;
  return r;
}
function loanAccrue(fx, opts = {}) {
  const r = fx.contract.circuits.accrue(
    context(fx.state, opts),
    opts.secret ?? fx.secrets.borrower, opts.program ?? fx.program, opts.network ?? fx.network,
    opts.revision ?? ledger(loanMod, fx.state).revision, opts.actor ?? 2n,
    opts.now ?? BigInt(TIME), opts.hints ?? loanAccrueHints(),
  );
  fx.state = r.context.currentQueryContext.state;
  return r;
}
function loanSettle(fx, opts = {}) {
  const r = fx.contract.circuits.settle(
    context(fx.state, opts),
    opts.secret ?? fx.secrets.borrower, opts.program ?? fx.program, opts.network ?? fx.network,
    opts.revision ?? ledger(loanMod, fx.state).revision, opts.actor ?? 2n,
    opts.asset ?? 0n, opts.amount ?? 533972602n, opts.now ?? BigInt(TIME), opts.hints ?? unusedHints(),
  );
  fx.state = r.context.currentQueryContext.state;
  return r;
}
function swapInit(fx, opts = {}) {
  const r = fx.contract.circuits.initialize(
    context(fx.state, opts),
    opts.secret ?? fx.secrets.provider, opts.program ?? fx.program, opts.network ?? fx.network,
    opts.actor ?? 3n, opts.now ?? BigInt(TIME),
  );
  fx.state = r.context.currentQueryContext.state;
  return r;
}
function swapSwap(fx, opts = {}) {
  const r = fx.contract.circuits.swap(
    context(fx.state, opts),
    opts.secret ?? fx.secrets.trader, opts.program ?? fx.program, opts.network ?? fx.network,
    opts.revision ?? ledger(swapMod, fx.state).revision, opts.actor ?? 4n, opts.recipient ?? 4n,
    opts.assetIn ?? 0n, opts.assetOut ?? 1n, opts.amountIn ?? 10000n,
    opts.minOut ?? 19700n, opts.now ?? BigInt(TIME), opts.hints ?? swapHints(),
  );
  fx.state = r.context.currentQueryContext.state;
  return r;
}
function swapClose(fx, opts = {}) {
  const r = fx.contract.circuits.close(
    context(fx.state, opts),
    opts.secret ?? fx.secrets.provider, opts.program ?? fx.program, opts.network ?? fx.network,
    opts.revision ?? ledger(swapMod, fx.state).revision, opts.actor ?? 3n,
    opts.now ?? BigInt(TIME), opts.hints ?? unusedHints(),
  );
  fx.state = r.context.currentQueryContext.state;
  return r;
}
function snapshot(mod, fx) {
  const L = ledger(mod, fx.state);
  return {
    remaining: L.remaining, revision: L.revision, initialized: L.initialized,
    kernel: {...L.kernelState},
  };
}
function fails(fn, needle) {
  let err;
  try { fn(); } catch (e) { err = e; }
  assert.ok(err, 'expected circuit failure');
  if (needle) {
    const text = String(err.message ?? err);
    assert.ok(text.includes(needle), `expected ${needle} in ${text}`);
  }
  return err;
}

test('compiled contracts expose constructor, circuits and ledger decoder', () => {
  assert.equal(typeof loanMod.Contract, 'function');
  assert.equal(typeof swapMod.Contract, 'function');
  assert.equal(typeof loanMod.ledger, 'function');
  assert.equal(typeof swapMod.ledger, 'function');
  const loan = new loanMod.Contract({});
  const swap = new swapMod.Contract({});
  for (const name of ['initialize', 'accrue', 'settle']) assert.equal(typeof loan.circuits[name], 'function', name);
  for (const name of ['initialize', 'swap', 'close']) assert.equal(typeof swap.circuits[name], 'function', name);
  assert.equal(typeof loanMod.pureCircuits.transition0, 'function');
  assert.equal(typeof swapMod.pureCircuits.transition0, 'function');
});

test('loan init mints 20b USD_micro to synthetic borrower and seals lifetime 2/0', () => {
  const fx = deployLoan();
  const L0 = ledger(loanMod, fx.state);
  assert.equal(L0.initialized, false);
  assert.deepEqual(L0.programDigest, hexBytes(bindings.loan.programDigest));
  assert.deepEqual(L0.networkTag, hexBytes(bindings.networkTag));
  const r = loanInit(fx);
  const L = ledger(loanMod, fx.state);
  assert.equal(L.initialized, true);
  assert.equal(L.remaining, 2n);
  assert.equal(L.revision, 0n);
  assert.equal(L.kernelState.f0, 5000000000n);
  assert.equal(L.kernelState.f5, 20000000000n);
  assert.equal(L.kernelState.f6, 0n);
  const e = effects(r);
  const usd = colorOf(bindings.loan.usdDomain);
  assert.equal(mapTotal(e.unshieldedMints), 20000000000n);
  assert.ok(mintRows(e.unshieldedMints).some((m) => m.domain === bindings.loan.usdDomain && m.amount === 20000000000n));
  const claims = claimRows(e.claimedUnshieldedSpends);
  assert.ok(hasClaim(claims, {
    amount: 20000000000n, destTag: 'user', dest: bindings.loan.borrowerAddress,
    tokenTag: 'unshielded', tokenRaw: usd,
  }));
  assert.equal(hasClaim(claims, {dest: bindings.loan.lenderAddress}), false);
  assert.equal(mapTotal(e.unshieldedInputs), 0n);
  assert.equal(mapTotal(e.unshieldedOutputs), 0n);
});

test('loan accrue executes kernel transition0: PR 500m, IP 33972602, residual 4.5b, no token movement', () => {
  const interest = (5000000000n * 8n * 31n) / (100n * 365n);
  assert.equal(interest, 33972602n);
  const fx = deployLoan();
  loanInit(fx);
  const r = loanAccrue(fx);
  const L = ledger(loanMod, fx.state);
  assert.equal(L.remaining, 1n);
  assert.equal(L.revision, 1n);
  assert.equal(L.kernelState.f0, 4500000000n);
  assert.equal(L.kernelState.f1, 500000000n);
  assert.equal(L.kernelState.f2, interest);
  assert.equal(L.kernelState.f3, 0n);
  assert.equal(L.kernelState.f4, 0n);
  assert.equal(L.kernelState.f5, 20000000000n);
  assert.equal(L.kernelState.f6, 0n);
  assert.equal(L.kernelState.f7, 1n);
  assert.equal(L.kernelState.f8, 0n);
  assert.equal(r.result.after.f1, 500000000n);
  assert.equal(r.result.effect0.v0, 4n);
  assert.equal(r.result.effect0.v4, 500000000n);
  assert.equal(r.result.effect1.v0, 3n);
  assert.equal(r.result.effect1.v4, interest);
  const e = effects(r);
  assert.equal(mapTotal(e.unshieldedMints), 0n);
  assert.equal(mapTotal(e.unshieldedInputs), 0n);
  assert.equal(mapTotal(e.unshieldedOutputs), 0n);
});

test('loan settle executes kernel transition1: 533972602 to sealed lender, residual notional survives', () => {
  const fx = deployLoan();
  loanInit(fx);
  loanAccrue(fx);
  const r = loanSettle(fx);
  const L = ledger(loanMod, fx.state);
  assert.equal(L.remaining, 0n);
  assert.equal(L.revision, 2n);
  assert.equal(L.kernelState.f0, 4500000000n);
  assert.equal(L.kernelState.f1, 0n);
  assert.equal(L.kernelState.f2, 0n);
  assert.equal(L.kernelState.f3, 500000000n);
  assert.equal(L.kernelState.f4, 33972602n);
  assert.equal(L.kernelState.f5, 19466027398n);
  assert.equal(L.kernelState.f6, 533972602n);
  assert.equal(L.kernelState.f7, 2n);
  assert.equal(L.kernelState.f8, 1n);
  assert.equal(r.result.effect0.v2, 5n);
  assert.equal(r.result.effect0.v3, 533972602n);
  assert.equal(r.result.effect1.v0, 4n);
  assert.equal(r.result.effect2.v0, 3n);
  const e = effects(r);
  const usd = colorOf(bindings.loan.usdDomain);
  assert.equal(mapTotal(e.unshieldedInputs), 533972602n);
  assert.equal(mapTotal(e.unshieldedOutputs), 533972602n);
  assert.ok(hasToken(tokenRows(e.unshieldedInputs), {tag: 'unshielded', raw: usd, amount: 533972602n}));
  assert.ok(hasToken(tokenRows(e.unshieldedOutputs), {tag: 'unshielded', raw: usd, amount: 533972602n}));
  const claims = claimRows(e.claimedUnshieldedSpends);
  assert.ok(hasClaim(claims, {
    amount: 533972602n, destTag: 'user', dest: bindings.loan.lenderAddress,
    tokenTag: 'unshielded', tokenRaw: usd,
  }));
  assert.equal(hasClaim(claims, {dest: bindings.loan.borrowerAddress}), false);
});

test('swap init mints pool 1m A / 2m B and trader 100k A, lifetime 8/0', () => {
  const fx = deploySwap();
  const r = swapInit(fx);
  const L = ledger(swapMod, fx.state);
  assert.equal(L.remaining, 8n);
  assert.equal(L.revision, 0n);
  assert.equal(L.kernelState.f0, 1000000n);
  assert.equal(L.kernelState.f1, 2000000n);
  assert.equal(L.kernelState.f2, 100000n);
  assert.equal(L.kernelState.f6, 0n);
  const e = effects(r);
  const colorA = colorOf(bindings.swap.assetADomain);
  const colorB = colorOf(bindings.swap.assetBDomain);
  assert.equal(mapTotal(e.unshieldedMints), 1000000n + 2000000n + 100000n);
  assert.ok(mintRows(e.unshieldedMints).some((m) => m.domain === bindings.swap.assetADomain && m.amount === 1100000n));
  assert.ok(mintRows(e.unshieldedMints).some((m) => m.domain === bindings.swap.assetBDomain && m.amount === 2000000n));
  const claims = claimRows(e.claimedUnshieldedSpends);
  assert.ok(hasClaim(claims, {
    amount: 100000n, destTag: 'user', dest: bindings.swap.traderAddress,
    tokenTag: 'unshielded', tokenRaw: colorA,
  }));
  assert.ok(hasClaim(claims, {
    amount: 1000000n, destTag: 'contract', dest: runtime.dummyContractAddress(),
    tokenTag: 'unshielded', tokenRaw: colorA,
  }));
  assert.ok(hasClaim(claims, {
    amount: 2000000n, destTag: 'contract', dest: runtime.dummyContractAddress(),
    tokenTag: 'unshielded', tokenRaw: colorB,
  }));
  assert.equal(hasClaim(claims, {dest: bindings.swap.providerAddress}), false);
  assert.deepEqual(L.colorA, hexBytes(colorA));
  assert.deepEqual(L.colorB, hexBytes(colorB));
});

test('swap transition0 minOut 19700 yields 19743 B, fee 30 A retained, lifetime 7/1', () => {
  const out = (10000n * 997n * 2000000n) / (1000000n * 1000n + 10000n * 997n);
  assert.equal(out, 19743n);
  const fx = deploySwap();
  swapInit(fx);
  const r = swapSwap(fx, {balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])});
  const L = ledger(swapMod, fx.state);
  assert.equal(L.remaining, 7n);
  assert.equal(L.revision, 1n);
  assert.equal(L.kernelState.f0, 1010000n);
  assert.equal(L.kernelState.f1, 1980257n);
  assert.equal(L.kernelState.f2, 90000n);
  assert.equal(L.kernelState.f3, 19743n);
  assert.equal(r.result.effect0.v3, 10000n);
  assert.equal(r.result.effect1.v3, 19743n);
  const e = effects(r);
  const colorA = colorOf(bindings.swap.assetADomain);
  const colorB = colorOf(bindings.swap.assetBDomain);
  assert.equal(mapTotal(e.unshieldedInputs), 10000n);
  assert.equal(mapTotal(e.unshieldedOutputs), 19743n);
  assert.ok(hasToken(tokenRows(e.unshieldedInputs), {tag: 'unshielded', raw: colorA, amount: 10000n}));
  assert.ok(hasToken(tokenRows(e.unshieldedOutputs), {tag: 'unshielded', raw: colorB, amount: 19743n}));
  const claims = claimRows(e.claimedUnshieldedSpends);
  assert.ok(hasClaim(claims, {
    amount: 19743n, destTag: 'user', dest: bindings.swap.traderAddress,
    tokenTag: 'unshielded', tokenRaw: colorB,
  }));
  assert.equal(hasClaim(claims, {dest: bindings.swap.providerAddress}), false);
  assert.equal(hasClaim(claims, {tokenRaw: colorA}), false);
});

test('swap close sends reserves to sealed provider, lifetime 6/2, remaining is not refreshed to 8', () => {
  const fx = deploySwap();
  swapInit(fx);
  swapSwap(fx, {balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])});
  const r = swapClose(fx, {balance: balanceMap([
    [bindings.swap.assetADomain, 1010000n],
    [bindings.swap.assetBDomain, 1980257n],
  ])});
  const L = ledger(swapMod, fx.state);
  assert.equal(L.remaining, 6n);
  assert.equal(L.revision, 2n);
  assert.equal(L.kernelState.f0, 0n);
  assert.equal(L.kernelState.f1, 0n);
  assert.equal(L.kernelState.f4, 1010000n);
  assert.equal(L.kernelState.f5, 1980257n);
  assert.equal(L.kernelState.f6, 1n);
  const e = effects(r);
  const colorA = colorOf(bindings.swap.assetADomain);
  const colorB = colorOf(bindings.swap.assetBDomain);
  assert.equal(mapTotal(e.unshieldedOutputs), 1010000n + 1980257n);
  assert.ok(hasToken(tokenRows(e.unshieldedOutputs), {tag: 'unshielded', raw: colorA, amount: 1010000n}));
  assert.ok(hasToken(tokenRows(e.unshieldedOutputs), {tag: 'unshielded', raw: colorB, amount: 1980257n}));
  const claims = claimRows(e.claimedUnshieldedSpends);
  assert.ok(hasClaim(claims, {
    amount: 1010000n, destTag: 'user', dest: bindings.swap.providerAddress,
    tokenTag: 'unshielded', tokenRaw: colorA,
  }));
  assert.ok(hasClaim(claims, {
    amount: 1980257n, destTag: 'user', dest: bindings.swap.providerAddress,
    tokenTag: 'unshielded', tokenRaw: colorB,
  }));
  assert.equal(hasClaim(claims, {dest: bindings.swap.traderAddress}), false);
  fails(
    () => swapSwap({...fx, state: fx.state}, {balance: balanceMap([[bindings.swap.assetBDomain, 1980257n]])}),
    'epoch is closed',
  );
});

test('pinned source permits provider close-before-swap; swap after close fails; remaining is not 8', () => {
  assert.equal(bindings.swap.closeBeforeSwapPermittedByPinnedSource, true);
  const fx = deploySwap();
  swapInit(fx);
  swapClose(fx, {balance: balanceMap([
    [bindings.swap.assetADomain, 1000000n],
    [bindings.swap.assetBDomain, 2000000n],
  ])});
  const L = ledger(swapMod, fx.state);
  assert.equal(L.remaining, 7n);
  assert.equal(L.revision, 1n);
  assert.equal(L.kernelState.f6, 1n);
  assert.equal(L.kernelState.f4, 1000000n);
  assert.equal(L.kernelState.f5, 2000000n);
  const before = snapshot(swapMod, fx);
  fails(
    () => swapSwap({...fx}, {balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])}),
    'epoch is closed',
  );
  assert.deepEqual(snapshot(swapMod, fx), before);
});

test('a lower minOut is valid; 19744 is an isolated minimum failure', () => {
  const fx = deploySwap();
  swapInit(fx);
  swapSwap(fx, {minOut: 1n, balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])});
  assert.equal(ledger(swapMod, fx.state).kernelState.f3, 19743n);
  const fx2 = deploySwap();
  swapInit(fx2);
  const before = snapshot(swapMod, fx2);
  fails(() => swapSwap({...fx2}, {
    minOut: 19744n,
    secret: fx2.secrets.trader,
    revision: 0n,
    hints: swapHints(),
    now: BigInt(TIME),
    balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]]),
  }), 'minimum output not met');
  assert.deepEqual(snapshot(swapMod, fx2), before);
});

test('wrong secret and other-role secret fail with correct actor mapping', () => {
  const fx = deployLoan();
  loanInit(fx);
  const before = snapshot(loanMod, fx);
  fails(() => loanAccrue({...fx}, {secret: secret(), actor: 2n}), 'BORROWER_CAPABILITY');
  fails(() => loanAccrue({...fx}, {secret: fx.secrets.lender, actor: 2n}), 'BORROWER_CAPABILITY');
  assert.deepEqual(snapshot(loanMod, fx), before);
  const sx = deploySwap();
  swapInit(sx);
  const sb = snapshot(swapMod, sx);
  fails(
    () => swapSwap({...sx}, {secret: sx.secrets.provider, actor: 4n, balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])}),
    'TRADER_CAPABILITY',
  );
  assert.deepEqual(snapshot(swapMod, sx), sb);
});

test('program, network, actor, recipient, asset and revision controls fail first', () => {
  const fx = deployLoan();
  loanInit(fx);
  const before = snapshot(loanMod, fx);
  fails(() => loanAccrue({...fx}, {program: hexBytes(bindings.swap.programDigest)}), 'PROGRAM_MISMATCH');
  fails(() => loanAccrue({...fx}, {network: hexBytes(bindings.loan.programDigest)}), 'NETWORK_MISMATCH');
  fails(() => loanAccrue({...fx}, {actor: 5n}), 'ACTOR_MAPPING');
  fails(() => loanAccrue({...fx}, {revision: 1n}), 'REVISION_MISMATCH');
  assert.deepEqual(snapshot(loanMod, fx), before);
  loanAccrue(fx);
  const after = snapshot(loanMod, fx);
  fails(() => loanSettle({...fx}, {revision: 0n}), 'REVISION_MISMATCH');
  fails(() => loanSettle({...fx}, {revision: 2n}), 'REVISION_MISMATCH');
  fails(() => loanSettle({...fx}, {asset: 1n}), 'SETTLEMENT_ASSET');
  fails(() => loanSettle({...fx}, {amount: 1n}), 'settlement amount mismatch');
  assert.deepEqual(snapshot(loanMod, fx), after);
  const sx = deploySwap();
  swapInit(sx);
  fails(
    () => swapSwap({...sx}, {recipient: 3n, balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])}),
    'RECIPIENT_MUST_BE_TRADER',
  );
  fails(
    () => swapSwap({...sx}, {assetIn: 1n, balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])}),
    'ASSET_IN',
  );
});

test('repeat init, accrue replay, settle-before-accrue and invalid hints fail', () => {
  const fx = deployLoan();
  loanInit(fx);
  fails(() => loanInit({...fx}));
  fails(() => loanSettle({...fx}));
  const bad = loanAccrueHints();
  bad.h0 = {...bad.h0, aLo: bad.h0.aLo + 1n};
  fails(() => loanAccrue({...fx}, {hints: bad}));
  loanAccrue(fx);
  fails(() => loanAccrue({...fx}));
  const sx = deploySwap();
  swapInit(sx);
  const sh = swapHints();
  sh.h3 = {...sh.h3, q: sh.h3.q + 1n};
  fails(() => swapSwap({...sx}, {hints: sh, balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])}));
});

test('time window boundaries and horizon use explicit synthetic block time', () => {
  const fx = deployLoan();
  loanInit(fx, {time: TIME});
  loanAccrue({...fx, state: fx.state}, {time: TIME});
  const fx2 = deployLoan();
  loanInit(fx2);
  loanAccrue(fx2, {time: TIME + WINDOW - 1, now: BigInt(TIME)});
  const fx3 = deployLoan();
  loanInit(fx3);
  fails(() => loanAccrue({...fx3}, {time: TIME + WINDOW, now: BigInt(TIME)}));
  const fx4 = deployLoan();
  loanInit(fx4);
  fails(() => loanAccrue({...fx4}, {time: TIME - 1, now: BigInt(TIME)}));
  const fx5 = deployLoan();
  fails(() => loanInit({...fx5}, {now: BigInt(HORIZON), time: HORIZON}));
  const fx6 = deployLoan();
  loanInit(fx6, {now: BigInt(HORIZON - 1), time: HORIZON - 1});
  assert.equal(ledger(loanMod, fx6.state).initialized, true);
});

test('insufficient entry reserve fails on swap B and both close colors; failed call rolls back', () => {
  const sx = deploySwap();
  swapInit(sx);
  const before = snapshot(swapMod, sx);
  fails(
    () => swapSwap({...sx}, {balance: balanceMap([[bindings.swap.assetBDomain, 19742n]])}),
    'INSUFFICIENT_ENTRY_RESERVE_B',
  );
  assert.deepEqual(snapshot(swapMod, sx), before);
  fails(() => swapClose({...sx}, {balance: balanceMap([
    [bindings.swap.assetADomain, 1000000n],
    [bindings.swap.assetBDomain, 0n],
  ])}), 'INSUFFICIENT_ENTRY_RESERVE_B');
  assert.deepEqual(snapshot(swapMod, sx), before);
  fails(() => swapClose({...sx}, {balance: balanceMap([
    [bindings.swap.assetADomain, 0n],
    [bindings.swap.assetBDomain, 2000000n],
  ])}), 'INSUFFICIENT_ENTRY_RESERVE_A');
  assert.deepEqual(snapshot(swapMod, sx), before);
  swapSwap(sx, {balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]])});
  assert.equal(ledger(swapMod, sx.state).revision, 1n);
});

test('runtime transcripts are not ledger funding, payer attribution, fees or finality', () => {
  assert.ok(bindings.deferredLedgerObligations.includes('actual-signed-unshielded-offer'));
  assert.ok(bindings.deferredLedgerObligations.includes('consumed-utxo-ownership'));
  assert.ok(bindings.deferredLedgerObligations.includes('borrower-trader-debit-payer-attribution'));
  assert.ok(bindings.deferredLedgerObligations.includes('missing-real-funding'));
  assert.ok(bindings.deferredLedgerObligations.includes('transaction-fees-change-finality'));
  assert.ok(bindings.deferredLedgerObligations.includes('blocked-comparison-utility-not-imported'));
  assert.equal(bindings.loan.addressesAreSynthetic, true);
  assert.equal(bindings.notActualWalletCustody, true);
});

test('QueryContext.block getter returns a copy; whole-block assignment persists synthetic balances', () => {
  const fx = deploySwap();
  const ctx = runtime.createCircuitContext(
    runtime.dummyContractAddress(), ZERO_KEY, fx.state, {}, undefined, undefined, TIME,
  );
  const first = ctx.currentQueryContext.block;
  const second = ctx.currentQueryContext.block;
  assert.equal(first === second, false);
  first.secondsSinceEpochErr = 7;
  first.lastBlockTime = 123n;
  first.balance = balanceMap([[bindings.swap.assetBDomain, 2000000n]]);
  const piecemeal = ctx.currentQueryContext.block;
  assert.equal(piecemeal.secondsSinceEpochErr, 0);
  assert.equal(piecemeal.lastBlockTime, 0n);
  assert.equal(piecemeal.balance.size, 0);
  assert.equal(Number(piecemeal.secondsSinceEpoch), TIME);
  const funded = context(fx.state, {
    balance: balanceMap([[bindings.swap.assetBDomain, 2000000n]]),
    err: 0,
  });
  const read = funded.currentQueryContext.block;
  assert.equal(Number(read.secondsSinceEpoch), TIME);
  assert.equal(read.secondsSinceEpochErr, 0);
  assert.equal(read.lastBlockTime, BigInt(TIME));
  assert.equal(balanceOf(read, bindings.swap.assetBDomain), 2000000n);
  assert.equal(balanceOf(read, bindings.swap.assetADomain), 0n);
});

test('init mints do not fund the next call; empty synthetic balance is an explicit test boundary', () => {
  const sx = deploySwap();
  swapInit(sx);
  const empty = context(sx.state);
  assert.equal(empty.currentQueryContext.block.balance.size, 0);
  assert.equal(balanceOf(empty.currentQueryContext.block, bindings.swap.assetBDomain), 0n);
  fails(() => swapSwap({...sx}), 'INSUFFICIENT_ENTRY_RESERVE_B');
});
