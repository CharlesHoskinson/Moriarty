#!/usr/bin/env node
/** Reproducible loan/swap Compact custody wrappers. No wallet SDK, no network. */
import {createHash} from 'node:crypto';
import {mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {dirname, join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));

export const PIN_PATHS = {
  loanSource: 'experiments/moriarty-language/spec/examples/loan.mori',
  swapSource: 'experiments/moriarty-language/spec/examples/swap.mori',
  arithmetic: 'experiments/moriarty-language/compact/arithmetic.compact',
  loanKernel: 'experiments/moriarty-language/compact/generated/loan/kernel.compact',
  swapKernel: 'experiments/moriarty-language/compact/generated/swap/kernel.compact',
  loanMetadata: 'experiments/moriarty-language/compact/generated/loan/metadata.json',
  swapMetadata: 'experiments/moriarty-language/compact/generated/swap/metadata.json',
  loanBoundProgram: 'experiments/moriarty-language/compact/generated/loan/bound-program.json',
  swapBoundProgram: 'experiments/moriarty-language/compact/generated/swap/bound-program.json',
};

export function sha256(buf) {
  return createHash('sha256').update(asBuffer(buf)).digest('hex');
}

export function asBuffer(value) {
  if (Buffer.isBuffer(value)) return value;
  if (value instanceof Uint8Array) return Buffer.from(value);
  if (typeof value === 'string') return Buffer.from(value);
  throw new Error('expected bytes');
}

export function isDirectRun(metaUrl, argv1 = process.argv[1]) {
  if (typeof metaUrl !== 'string' || metaUrl.length === 0) {
    throw new Error('isDirectRun requires explicit caller module identity');
  }
  if (!argv1) return false;
  return resolve(argv1) === fileURLToPath(metaUrl);
}

function pad32(ascii) {
  const b = Buffer.alloc(32);
  Buffer.from(ascii, 'utf8').copy(b);
  return b;
}
function assertPad(ascii, hex) {
  const got = pad32(ascii).toString('hex');
  if (got !== hex) throw new Error(`pad32(${ascii})=${got} != ${hex}`);
}
function bytesLiteral(hex) {
  if (!/^[0-9a-f]{64}$/.test(hex)) throw new Error(`expected 32-byte hex, got ${hex}`);
  const nums = [];
  for (let i = 0; i < 64; i += 2) nums.push(String(parseInt(hex.slice(i, i + 2), 16)));
  return `Bytes[${nums.join(', ')}]`;
}
function textId(metadata, text) {
  const row = metadata.textTable.find((t) => t.text === text);
  if (!row) throw new Error(`missing text ${text}`);
  return Number(row.id);
}
function fieldOf(metadata, name) {
  const row = metadata.stateFields.find((f) => f.name === name);
  if (!row) throw new Error(`missing field ${name}`);
  return row.field;
}
function initialByName(bound) {
  return Object.fromEntries(bound.manifest.initialState.map((x) => [x.name, x.value.value]));
}
function constByName(bound) {
  return Object.fromEntries(bound.manifest.constants.map((x) => [x.name, x.value.value]));
}
function kernelStateLiteral(metadata, bound) {
  const init = initialByName(bound);
  return `KernelState { ${metadata.stateFields.map((f) => `${f.field}: ${init[f.name]}`).join(', ')} }`;
}

export function validatePinnedInputs({worktree, bindings, readFileSync: rf}) {
  assertPad(bindings.networkTagAscii, bindings.networkTag);
  assertPad(bindings.loan.borrowerDomainAscii, bindings.loan.borrowerDomain);
  assertPad(bindings.loan.lenderDomainAscii, bindings.loan.lenderDomain);
  assertPad(bindings.loan.usdDomainAscii, bindings.loan.usdDomain);
  assertPad(bindings.swap.traderDomainAscii, bindings.swap.traderDomain);
  assertPad(bindings.swap.providerDomainAscii, bindings.swap.providerDomain);
  assertPad(bindings.swap.assetADomainAscii, bindings.swap.assetADomain);
  assertPad(bindings.swap.assetBDomainAscii, bindings.swap.assetBDomain);

  function pin(key) {
    const rel = PIN_PATHS[key];
    const abs = join(worktree, rel);
    const bytes = asBuffer(rf(abs));
    const digest = sha256(bytes);
    const expected = bindings.digests[key];
    if (digest !== expected) throw new Error(`stale input ${rel}: ${digest} != ${expected}`);
    return {
      abs, rel, bytes, digest,
      json: rel.endsWith('.json') ? JSON.parse(bytes.toString('utf8')) : null,
    };
  }

  const loanSource = pin('loanSource');
  const swapSource = pin('swapSource');
  const arithmetic = pin('arithmetic');
  const loanKernel = pin('loanKernel');
  const swapKernel = pin('swapKernel');
  const loanMetaPin = pin('loanMetadata');
  const swapMetaPin = pin('swapMetadata');
  const loanBoundPin = pin('loanBoundProgram');
  const swapBoundPin = pin('swapBoundProgram');
  const loanMeta = loanMetaPin.json;
  const swapMeta = swapMetaPin.json;
  const loanBound = loanBoundPin.json;
  const swapBound = swapBoundPin.json;

  if (loanMeta.programHash !== bindings.digests.loanProgram) throw new Error('loan program hash');
  if (swapMeta.programHash !== bindings.digests.swapProgram) throw new Error('swap program hash');
  if (bindings.loan.programDigest !== bindings.digests.loanProgram) throw new Error('loan programDigest != digests.loanProgram');
  if (bindings.swap.programDigest !== bindings.digests.swapProgram) throw new Error('swap programDigest != digests.swapProgram');
  if (bindings.loan.programDigest !== loanMeta.programHash) throw new Error('loan programDigest != metadata.programHash');
  if (bindings.swap.programDigest !== swapMeta.programHash) throw new Error('swap programDigest != metadata.programHash');
  if (loanMeta.sourceHash !== bindings.digests.loanSource) throw new Error('loan source hash');
  if (swapMeta.sourceHash !== bindings.digests.swapSource) throw new Error('swap source hash');
  if (loanMeta.sourceHash !== loanSource.digest) throw new Error('loan metadata.sourceHash != loan.mori bytes');
  if (swapMeta.sourceHash !== swapSource.digest) throw new Error('swap metadata.sourceHash != swap.mori bytes');
  if (loanMeta.compactSourceHash !== bindings.digests.loanKernel) throw new Error('loan kernel hash');
  if (swapMeta.compactSourceHash !== bindings.digests.swapKernel) throw new Error('swap kernel hash');
  if (loanBound.programHash !== bindings.digests.loanProgram) throw new Error('loan bound program hash');
  if (swapBound.programHash !== bindings.digests.swapProgram) throw new Error('swap bound program hash');
  if (loanBound.manifest.profile !== bindings.profile) throw new Error('loan profile');
  if (swapBound.manifest.profile !== bindings.profile) throw new Error('swap profile');

  return {
    loanSource, swapSource, arithmetic, loanKernel, swapKernel,
    loanMeta, swapMeta, loanBound, swapBound,
    loanMetaPin, swapMetaPin, loanBoundPin, swapBoundPin,
  };
}

function header(kind, meta, bindings) {
  return `pragma language_version >= 0.23 && <= 0.23;
import CompactStandardLibrary;
include "kernel";
// generated custody wrapper kind=${kind}
// profile=${bindings.profile}
// sourceHash=${meta.sourceHash}
// programHash=${meta.programHash}
// compactSourceHash=${meta.compactSourceHash}
// metadataHash=${kind === 'loan' ? bindings.digests.loanMetadata : bindings.digests.swapMetadata}
`;
}

function timeCircuits() {
  return `
circuit capabilityHash(domain: Bytes<32>, network: Bytes<32>, program: Bytes<32>, secret: Bytes<32>): Bytes<32> {
  return persistentHash<Vector<4, Bytes<32>>>([domain, network, program, secret]);
}

circuit constrainNow(now: Uint<64>): Uint<128> {
  const publicNow: Uint<64> = disclose(now);
  const maxNow: Uint<64> = 18446744073709551315;
  assert(publicNow <= maxNow, "UINT64_WINDOW_OVERFLOW");
  const windowEnd: Uint<64> = ((publicNow as Uint<128>) + 300) as Uint<64>;
  assert(blockTimeGte(publicNow), "BLOCK_TIME_TOO_EARLY");
  assert(blockTimeLt(windowEnd), "BLOCK_TIME_TOO_LATE");
  assert(publicNow < 2000000000, "HORIZON_EXPIRED");
  return publicNow as Uint<128>;
}
`;
}

function envelope(program, network, revision) {
  return `  assert(${program} == programDigest, "PROGRAM_MISMATCH");
  assert(${network} == networkTag, "NETWORK_MISMATCH");
  assert(${revision} == revision, "REVISION_MISMATCH");
  assert(initialized, "NOT_INITIALIZED");`;
}

function emitLoan(bindings, pinned) {
  const m = pinned.loanMeta;
  const bound = pinned.loanBound;
  const c = constByName(bound);
  const borrower = textId(m, 'borrower');
  const lender = textId(m, 'lender');
  const usdAsset = textId(m, 'USD_TEST_ASSET');
  const usdMicro = textId(m, 'USD_micro');
  const duePr = textId(m, 'lam01:period1:PR');
  const dueIp = textId(m, 'lam01:period1:IP');
  const principalDue = c.principal_installment;
  const interestDue = c.expected_interest;
  const totalDue = c.expected_total_due;
  const residual = c.expected_outstanding_notional;
  const pinnedProgram = bytesLiteral(m.programHash);
  return `${header('loan', m, bindings)}
export ledger programDigest: Bytes<32>;
export ledger networkTag: Bytes<32>;
export ledger borrowerCapability: Bytes<32>;
export ledger lenderCapability: Bytes<32>;
export ledger borrowerAddress: UserAddress;
export ledger lenderAddress: UserAddress;
export ledger usdDomain: Bytes<32>;
export ledger usdColor: Bytes<32>;
export ledger initialized: Boolean;
export ledger remaining: Uint<128>;
export ledger revision: Uint<128>;
export ledger kernelState: KernelState;
export ledger lastAccrue: Result0;
export ledger lastSettle: Result1;
${timeCircuits()}
constructor(borrowerSecret: Bytes<32>, lenderSecret: Bytes<32>, borrowerPayout: UserAddress, lenderPayout: UserAddress, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>) {
  const pinnedProgram: Bytes<32> = ${pinnedProgram};
  const publicProgram: Bytes<32> = disclose(expectedProgram);
  assert(publicProgram == pinnedProgram, "PROGRAM_MISMATCH");
  programDigest = pinnedProgram;
  networkTag = disclose(expectedNetwork);
  borrowerAddress = disclose(borrowerPayout);
  lenderAddress = disclose(lenderPayout);
  borrowerCapability = disclose(capabilityHash(pad(32, "${bindings.loan.borrowerDomainAscii}"), expectedNetwork, pinnedProgram, borrowerSecret));
  lenderCapability = disclose(capabilityHash(pad(32, "${bindings.loan.lenderDomainAscii}"), expectedNetwork, pinnedProgram, lenderSecret));
  usdDomain = pad(32, "${bindings.loan.usdDomainAscii}");
  initialized = false;
  remaining = 0;
  revision = 0;
}

export circuit initialize(borrowerSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedActor: Uint<32>, now: Uint<64>): [] {
  assert(expectedProgram == programDigest, "PROGRAM_MISMATCH");
  assert(expectedNetwork == networkTag, "NETWORK_MISMATCH");
  assert(capabilityHash(pad(32, "${bindings.loan.borrowerDomainAscii}"), networkTag, programDigest, borrowerSecret) == borrowerCapability, "BORROWER_CAPABILITY");
  assert(expectedActor == ${borrower}, "ACTOR_MAPPING");
  assert(!initialized, "ALREADY_INITIALIZED");
  constrainNow(now);
  kernelState = ${kernelStateLiteral(m, bound)};
  remaining = ${bound.manifest.lifetime};
  revision = 0;
  usdColor = mintUnshieldedToken(usdDomain, ${bindings.loan.mintToBorrower}, right<ContractAddress, UserAddress>(borrowerAddress));
  initialized = true;
}

export circuit accrue(borrowerSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedRevision: Uint<128>, expectedActor: Uint<32>, now: Uint<64>, hints: Hints0): Result0 {
${envelope('expectedProgram', 'expectedNetwork', 'expectedRevision')}
  assert(capabilityHash(pad(32, "${bindings.loan.borrowerDomainAscii}"), networkTag, programDigest, borrowerSecret) == borrowerCapability, "BORROWER_CAPABILITY");
  assert(expectedActor == ${borrower}, "ACTOR_MAPPING");
  const t: Uint<128> = constrainNow(now);
  const result: Result0 = disclose(transition0(kernelState, Arguments0 { a0: expectedActor }, KernelObservations { o0: t }, remaining, revision, hints));
  assert(result.effect0.v0 == ${duePr}, "DUE_ID_PR");
  assert(result.effect0.v1 == ${borrower}, "DUE_PR_DEBTOR");
  assert(result.effect0.v2 == ${lender}, "DUE_PR_CREDITOR");
  assert(result.effect0.v3 == ${usdMicro}, "DUE_PR_DENOMINATION");
  assert(result.effect0.v4 == ${principalDue}, "DUE_PR_AMOUNT");
  assert(result.effect1.v0 == ${dueIp}, "DUE_ID_IP");
  assert(result.effect1.v1 == ${borrower}, "DUE_IP_DEBTOR");
  assert(result.effect1.v2 == ${lender}, "DUE_IP_CREDITOR");
  assert(result.effect1.v3 == ${usdMicro}, "DUE_IP_DENOMINATION");
  assert(result.effect1.v4 == ${interestDue}, "DUE_IP_AMOUNT");
  assert(result.after.${fieldOf(m, 'notional')} == ${residual}, "RESIDUAL_NOTIONAL");
  kernelState = result.after;
  remaining = result.remaining;
  revision = result.revision;
  lastAccrue = result;
  return result;
}

export circuit settle(borrowerSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedRevision: Uint<128>, expectedActor: Uint<32>, settlementAsset: Uint<32>, amountDue: Uint<128>, now: Uint<64>, hints: Hints1): Result1 {
${envelope('expectedProgram', 'expectedNetwork', 'expectedRevision')}
  assert(capabilityHash(pad(32, "${bindings.loan.borrowerDomainAscii}"), networkTag, programDigest, borrowerSecret) == borrowerCapability, "BORROWER_CAPABILITY");
  assert(expectedActor == ${borrower}, "ACTOR_MAPPING");
  assert(settlementAsset == ${usdAsset}, "SETTLEMENT_ASSET");
  const t: Uint<128> = constrainNow(now);
  const result: Result1 = disclose(transition1(kernelState, Arguments1 { a0: expectedActor, a1: settlementAsset, a2: amountDue }, KernelObservations { o0: t }, remaining, revision, hints));
  assert(result.effect0.v0 == ${usdAsset}, "TRANSFER_ASSET");
  assert(result.effect0.v1 == ${borrower}, "TRANSFER_FROM");
  assert(result.effect0.v2 == ${lender}, "TRANSFER_TO");
  assert(result.effect0.v3 == ${totalDue}, "TRANSFER_AMOUNT");
  assert(result.effect1.v0 == ${duePr}, "SETTLED_PR_ID");
  assert(result.effect1.v1 == ${borrower}, "SETTLED_PR_DEBTOR");
  assert(result.effect1.v2 == ${lender}, "SETTLED_PR_CREDITOR");
  assert(result.effect1.v3 == ${usdMicro}, "SETTLED_PR_DENOMINATION");
  assert(result.effect1.v4 == ${principalDue}, "SETTLED_PR_AMOUNT");
  assert(result.effect1.v5 == ${usdAsset}, "SETTLED_PR_ASSET");
  assert(result.effect2.v0 == ${dueIp}, "SETTLED_IP_ID");
  assert(result.effect2.v1 == ${borrower}, "SETTLED_IP_DEBTOR");
  assert(result.effect2.v2 == ${lender}, "SETTLED_IP_CREDITOR");
  assert(result.effect2.v3 == ${usdMicro}, "SETTLED_IP_DENOMINATION");
  assert(result.effect2.v4 == ${interestDue}, "SETTLED_IP_AMOUNT");
  assert(result.effect2.v5 == ${usdAsset}, "SETTLED_IP_ASSET");
  assert(result.after.${fieldOf(m, 'notional')} == ${residual}, "RESIDUAL_NOTIONAL");
  const entry: Uint<128> = unshieldedBalance(usdColor);
  const paid: Uint<128> = result.effect0.v3;
  assert(checkedAdd(entry, paid) >= paid, "SETTLE_IN_CALL_DELTA");
  receiveUnshielded(usdColor, paid);
  sendUnshielded(usdColor, paid, right<ContractAddress, UserAddress>(lenderAddress));
  kernelState = result.after;
  remaining = result.remaining;
  revision = result.revision;
  lastSettle = result;
  return result;
}
`;
}

function emitSwap(bindings, pinned) {
  const m = pinned.swapMeta;
  const bound = pinned.swapBound;
  const c = constByName(bound);
  const trader = textId(m, 'trader');
  const provider = textId(m, 'provider');
  const pool = textId(m, 'pool');
  const assetA = textId(m, 'ASSET_A');
  const assetB = textId(m, 'ASSET_B');
  const expectedOut = c.expected_output;
  const expectedRA = c.expected_reserve_a;
  const expectedRB = c.expected_reserve_b;
  const reserveA = fieldOf(m, 'reserve_a');
  const reserveB = fieldOf(m, 'reserve_b');
  const pinnedProgram = bytesLiteral(m.programHash);
  return `${header('swap', m, bindings)}
export ledger programDigest: Bytes<32>;
export ledger networkTag: Bytes<32>;
export ledger traderCapability: Bytes<32>;
export ledger providerCapability: Bytes<32>;
export ledger traderAddress: UserAddress;
export ledger providerAddress: UserAddress;
export ledger assetADomain: Bytes<32>;
export ledger assetBDomain: Bytes<32>;
export ledger colorA: Bytes<32>;
export ledger colorB: Bytes<32>;
export ledger initialized: Boolean;
export ledger remaining: Uint<128>;
export ledger revision: Uint<128>;
export ledger kernelState: KernelState;
export ledger lastSwap: Result0;
export ledger lastClose: Result1;
${timeCircuits()}
constructor(traderSecret: Bytes<32>, providerSecret: Bytes<32>, traderPayout: UserAddress, providerPayout: UserAddress, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>) {
  const pinnedProgram: Bytes<32> = ${pinnedProgram};
  const publicProgram: Bytes<32> = disclose(expectedProgram);
  assert(publicProgram == pinnedProgram, "PROGRAM_MISMATCH");
  programDigest = pinnedProgram;
  networkTag = disclose(expectedNetwork);
  traderAddress = disclose(traderPayout);
  providerAddress = disclose(providerPayout);
  traderCapability = disclose(capabilityHash(pad(32, "${bindings.swap.traderDomainAscii}"), expectedNetwork, pinnedProgram, traderSecret));
  providerCapability = disclose(capabilityHash(pad(32, "${bindings.swap.providerDomainAscii}"), expectedNetwork, pinnedProgram, providerSecret));
  assetADomain = pad(32, "${bindings.swap.assetADomainAscii}");
  assetBDomain = pad(32, "${bindings.swap.assetBDomainAscii}");
  initialized = false;
  remaining = 0;
  revision = 0;
}

export circuit initialize(providerSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedActor: Uint<32>, now: Uint<64>): [] {
  assert(expectedProgram == programDigest, "PROGRAM_MISMATCH");
  assert(expectedNetwork == networkTag, "NETWORK_MISMATCH");
  assert(capabilityHash(pad(32, "${bindings.swap.providerDomainAscii}"), networkTag, programDigest, providerSecret) == providerCapability, "PROVIDER_CAPABILITY");
  assert(expectedActor == ${provider}, "ACTOR_MAPPING");
  assert(!initialized, "ALREADY_INITIALIZED");
  constrainNow(now);
  kernelState = ${kernelStateLiteral(m, bound)};
  remaining = ${bound.manifest.lifetime};
  revision = 0;
  colorA = mintUnshieldedToken(assetADomain, ${bindings.swap.poolA}, left<ContractAddress, UserAddress>(kernel.self()));
  colorB = mintUnshieldedToken(assetBDomain, ${bindings.swap.poolB}, left<ContractAddress, UserAddress>(kernel.self()));
  mintUnshieldedToken(assetADomain, ${bindings.swap.traderA}, right<ContractAddress, UserAddress>(traderAddress));
  initialized = true;
}

export circuit swap(traderSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedRevision: Uint<128>, expectedActor: Uint<32>, recipient: Uint<32>, assetIn: Uint<32>, assetOut: Uint<32>, amountIn: Uint<128>, minOut: Uint<128>, now: Uint<64>, hints: Hints0): Result0 {
${envelope('expectedProgram', 'expectedNetwork', 'expectedRevision')}
  assert(capabilityHash(pad(32, "${bindings.swap.traderDomainAscii}"), networkTag, programDigest, traderSecret) == traderCapability, "TRADER_CAPABILITY");
  assert(expectedActor == ${trader}, "ACTOR_MAPPING");
  assert(recipient == ${trader}, "RECIPIENT_MUST_BE_TRADER");
  assert(assetIn == ${assetA}, "ASSET_IN");
  assert(assetOut == ${assetB}, "ASSET_OUT");
  const entryA: Uint<128> = unshieldedBalance(colorA);
  const entryB: Uint<128> = unshieldedBalance(colorB);
  assert(entryA == kernelState.${reserveA}, "ENTRY_RESERVE_A_MISMATCH");
  assert(entryB == kernelState.${reserveB}, "ENTRY_RESERVE_B_MISMATCH");
  const t: Uint<128> = constrainNow(now);
  const result: Result0 = disclose(transition0(kernelState, Arguments0 { a0: expectedActor, a1: recipient, a2: assetIn, a3: assetOut, a4: amountIn, a5: minOut }, KernelObservations { o0: t }, remaining, revision, hints));
  assert(result.effect0.v0 == ${assetA}, "SWAP_IN_ASSET");
  assert(result.effect0.v1 == ${trader}, "SWAP_IN_FROM");
  assert(result.effect0.v2 == ${pool}, "SWAP_IN_TO");
  assert(result.effect0.v3 == amountIn, "SWAP_IN_AMOUNT");
  assert(result.effect1.v0 == ${assetB}, "SWAP_OUT_ASSET");
  assert(result.effect1.v1 == ${pool}, "SWAP_OUT_FROM");
  assert(result.effect1.v2 == ${trader}, "SWAP_OUT_TO");
  assert(result.effect1.v3 == ${expectedOut}, "SWAP_OUT_AMOUNT");
  assert(result.after.${reserveA} == ${expectedRA}, "RESERVE_A");
  assert(result.after.${reserveB} == ${expectedRB}, "RESERVE_B");
  assert(unshieldedBalanceGte(colorB, result.effect1.v3), "INSUFFICIENT_ENTRY_RESERVE_B");
  assert(checkedAdd(entryA, result.effect0.v3) >= result.effect0.v3, "SWAP_IN_CALL_DELTA");
  assert(entryB >= result.effect1.v3, "SWAP_ENTRY_B_DELTA");
  assert(checkedAdd(entryA, result.effect0.v3) == result.after.${reserveA}, "RESERVE_A_EFFECT_DELTA");
  assert(checkedAdd(result.after.${reserveB}, result.effect1.v3) == entryB, "RESERVE_B_EFFECT_DELTA");
  receiveUnshielded(colorA, result.effect0.v3);
  sendUnshielded(colorB, result.effect1.v3, right<ContractAddress, UserAddress>(traderAddress));
  kernelState = result.after;
  remaining = result.remaining;
  revision = result.revision;
  lastSwap = result;
  return result;
}

export circuit close(providerSecret: Bytes<32>, expectedProgram: Bytes<32>, expectedNetwork: Bytes<32>, expectedRevision: Uint<128>, expectedActor: Uint<32>, now: Uint<64>, hints: Hints1): Result1 {
${envelope('expectedProgram', 'expectedNetwork', 'expectedRevision')}
  assert(capabilityHash(pad(32, "${bindings.swap.providerDomainAscii}"), networkTag, programDigest, providerSecret) == providerCapability, "PROVIDER_CAPABILITY");
  assert(expectedActor == ${provider}, "ACTOR_MAPPING");
  const entryA: Uint<128> = unshieldedBalance(colorA);
  const entryB: Uint<128> = unshieldedBalance(colorB);
  assert(entryA == kernelState.${reserveA}, "ENTRY_RESERVE_A_MISMATCH");
  assert(entryB == kernelState.${reserveB}, "ENTRY_RESERVE_B_MISMATCH");
  const t: Uint<128> = constrainNow(now);
  const result: Result1 = disclose(transition1(kernelState, Arguments1 { a0: expectedActor }, KernelObservations { o0: t }, remaining, revision, hints));
  assert(result.effect0.v0 == ${assetA}, "CLOSE_A_ASSET");
  assert(result.effect0.v1 == ${pool}, "CLOSE_A_FROM");
  assert(result.effect0.v2 == ${provider}, "CLOSE_A_TO");
  assert(result.effect1.v0 == ${assetB}, "CLOSE_B_ASSET");
  assert(result.effect1.v1 == ${pool}, "CLOSE_B_FROM");
  assert(result.effect1.v2 == ${provider}, "CLOSE_B_TO");
  assert(unshieldedBalanceGte(colorA, result.effect0.v3), "INSUFFICIENT_ENTRY_RESERVE_A");
  assert(unshieldedBalanceGte(colorB, result.effect1.v3), "INSUFFICIENT_ENTRY_RESERVE_B");
  assert(entryA == result.effect0.v3, "CLOSE_SURPLUS_A");
  assert(entryB == result.effect1.v3, "CLOSE_SURPLUS_B");
  assert(result.after.${reserveA} == 0, "CLOSE_RESERVE_A_ZERO");
  assert(result.after.${reserveB} == 0, "CLOSE_RESERVE_B_ZERO");
  assert(checkedAdd(result.after.${reserveA}, result.effect0.v3) == entryA, "CLOSE_A_EFFECT_DELTA");
  assert(checkedAdd(result.after.${reserveB}, result.effect1.v3) == entryB, "CLOSE_B_EFFECT_DELTA");
  sendUnshielded(colorA, result.effect0.v3, right<ContractAddress, UserAddress>(providerAddress));
  sendUnshielded(colorB, result.effect1.v3, right<ContractAddress, UserAddress>(providerAddress));
  kernelState = result.after;
  remaining = result.remaining;
  revision = result.revision;
  lastClose = result;
  return result;
}
`;
}

export function generateWrappers({worktree, bindings, readFileSync: rf, pinned}) {
  const p = pinned ?? validatePinnedInputs({worktree, bindings, readFileSync: rf});
  return {
    loanSource: emitLoan(bindings, p),
    swapSource: emitSwap(bindings, p),
    pinned: p,
  };
}

export function runGenerateCli({
  argv = process.argv,
  cwd = process.cwd(),
  env = process.env,
  writeFileSync: wf = writeFileSync,
  mkdirSync: mk = mkdirSync,
  readFileSync: rf = readFileSync,
} = {}) {
  const i = argv.indexOf('--output-dir');
  if (i < 0 || !argv[i + 1]) throw new Error('missing --output-dir');
  const outputDir = resolve(argv[i + 1]);
  const bindings = JSON.parse(asBuffer(rf(join(here, 'bindings.json'))).toString('utf8'));
  const worktree = resolve(here, '../../..');
  const generated = generateWrappers({worktree, bindings, readFileSync: rf});
  mk(outputDir, {recursive: true});
  wf(join(outputDir, 'loan.compact'), generated.loanSource);
  wf(join(outputDir, 'swap.compact'), generated.swapSource);
  const files = {
    'loan.compact': {bytes: Buffer.byteLength(generated.loanSource), sha256: sha256(generated.loanSource)},
    'swap.compact': {bytes: Buffer.byteLength(generated.swapSource), sha256: sha256(generated.swapSource)},
  };
  const report = JSON.stringify({
    status: 'generated',
    argv,
    cwd,
    envNames: Object.keys(env).sort(),
    outputDir,
    files,
    pinned: {
      arithmetic: generated.pinned.arithmetic.digest,
      loanKernel: generated.pinned.loanKernel.digest,
      swapKernel: generated.pinned.swapKernel.digest,
      loanSource: generated.pinned.loanSource.digest,
      swapSource: generated.pinned.swapSource.digest,
    },
  }) + '\n';
  process.stdout.write(report);
  return {outputDir, files, report};
}

if (isDirectRun(import.meta.url)) {
  runGenerateCli();
}
