import {readFileSync, openSync, closeSync, writeFileSync, fsyncSync, renameSync, unlinkSync, linkSync, lstatSync, constants} from 'node:fs';
import {createHash, randomUUID} from 'node:crypto';
import {join, isAbsolute, dirname} from 'node:path';
import {pathToFileURL} from 'node:url';

export const PINNED_NM = '/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';
// Direct provider entry bytes are pinned; this is not a transitive supply-chain attestation.
const PINS = {
  "midnight-js-network-id": {"version": "4.1.1", "entry": "dist/index.mjs", "sha256": "c4c035dc49196098de80ec81b42a27632901da60f3e2a0bafa1876847f2db503"},
  "midnight-js-node-zk-config-provider": {
    "version": "4.1.1",
    "entry": "dist/index.mjs",
    "sha256": "0698e57a324a2f8e85f0593d4feb1166c46fce93b7f6be1f65e348e72d12c942"
  },
  "midnight-js-level-private-state-provider": {
    "version": "4.1.1",
    "entry": "dist/index.mjs",
    "sha256": "fd724431222804df8c643ee17f8c2d71f26d6c71fa9e5bf259369083a57c5ec1"
  },
  "midnight-js-indexer-public-data-provider": {
    "version": "4.1.1",
    "entry": "dist/index.mjs",
    "sha256": "23a1883b796fbea037817ae0a7e0f5ed3c072a7d30974154a8de917893cd7f2f"
  },
  "midnight-js-http-client-proof-provider": {
    "version": "4.1.1",
    "entry": "dist/index.mjs",
    "sha256": "f412603b4d2b262f864d9aa09eba0195a64de3ec258e2afe0e78f7c26354bc3c"
  }
};
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
export async function loadFinancialSdk() {
  const result = {};
  for (const [name, pin] of Object.entries(PINS)) {
    const root = join(PINNED_NM, '@midnight-ntwrk', name);
    const pkg = JSON.parse(readFileSync(join(root, 'package.json'), 'utf8'));
    if (pkg.version !== pin.version || digest(readFileSync(join(root, pin.entry))) !== pin.sha256) throw Error('financial SDK pin mismatch: ' + name);
    Object.assign(result, await import(pathToFileURL(join(root, pin.entry)).href));
  }
  return result;
}
function amount(value, label) {
  if (typeof value !== 'bigint' || value < 0n) throw Error('invalid ' + label);
  return value;
}
function integer(value, label) {
  if (!Number.isSafeInteger(value) || value <= 0) throw Error('invalid ' + label);
  return value;
}
function asset(value) {
  if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) throw Error('invalid native asset');
  return value;
}
function nativeTransaction(tx, ledger) {
  if (!(tx instanceof ledger.Transaction) || Object.getPrototypeOf(tx) !== ledger.Transaction.prototype) throw Error('native Transaction required');
  if (tx.rewards !== undefined || tx.guaranteedOffer !== undefined || (tx.fallibleOffer?.size ?? 0) !== 0) throw Error('shielded/transient/reward transaction unsupported');
  if (!(tx.intents instanceof Map)) throw Error('native intents required');
  for (const [, intent] of tx.intents) {
    if (!(intent instanceof ledger.Intent)) throw Error('native Intent required');
    if (intent.dustActions?.registrations.length) throw Error('DUST registration authorization unsupported; use an already registered wallet');
  }
  return tx;
}
function parts(recipe, ledger) {
  if (!recipe || recipe.type !== 'UNBOUND_TRANSACTION' || Object.keys(recipe).some(k => !['type', 'baseTransaction', 'balancingTransaction'].includes(k))) throw Error('unsupported financial recipe; expected UNBOUND_TRANSACTION');
  const txs = [recipe.baseTransaction, ...(recipe.balancingTransaction === undefined ? [] : [recipe.balancingTransaction])].map(tx => nativeTransaction(tx, ledger));
  ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', txs[0].serialize());
  if (txs[1]) ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', txs[1].serialize());
  return txs;
}
function semanticBytes(txs) {
  return Buffer.from(txs.map(tx => tx.eraseProofs()).reduce((left, right) => left.merge(right)).serialize());
}
function inspect(txs, ledger, payer, verify) {
  const gross = Object.create(null), seen = new Set();
  let dustFee = 0n;
  for (const tx of txs) {
    for (const [segment, intent] of tx.intents) {
      for (const offer of [intent.guaranteedUnshieldedOffer, intent.fallibleUnshieldedOffer]) {
        if (offer === undefined) continue;
        if (!(offer instanceof ledger.UnshieldedOffer)) throw Error('native UnshieldedOffer required');
        if (verify && offer.signatures.length !== offer.inputs.length) throw Error('input signature count mismatch');
        for (const [index, input] of offer.inputs.entries()) {
          const key = input.intentHash + ':' + input.outputNo;
          if (seen.has(key)) throw Error('duplicate consumed input');
          seen.add(key);
          if (input.owner !== payer) throw Error('input owner differs from selected payer');
          const type = asset(input.type);
          gross[type] = (gross[type] ?? 0n) + amount(input.value, 'gross input');
          if (verify && ledger.verifySignature(input.owner, intent.signatureData(segment), offer.signatures[index]) !== true) throw Error('input signature verification failed');
        }
      }
      if (intent.dustActions !== undefined) {
        if (!(intent.dustActions instanceof ledger.DustActions)) throw Error('native DustActions required');
        for (const spend of intent.dustActions.spends) {
          if (!(spend instanceof ledger.DustSpend)) throw Error('native DustSpend required');
          const key = 'dust:' + spend.oldNullifier;
          if (seen.has(key)) throw Error('duplicate DUST spend');
          seen.add(key);
          dustFee += amount(spend.vFee, 'DUST fee');
        }
      }
    }
  }
  return {gross, dustFee};
}

const encode = value => JSON.stringify(value, (_, v) => typeof v === 'bigint' ? v.toString() : v);
function bindingFor({walletContext, networkConfig, limits}) {
  if (typeof limits?.allocationId !== 'string' || !limits.allocationId.trim() || !isAbsolute(limits.reservationStatePath ?? '')) throw Error('reservation allocationId and absolute reservationStatePath required');
  if (typeof networkConfig?.networkId !== 'string' || !networkConfig.networkId) throw Error('reservation networkId required');
  const network = {};
  for (const key of ['networkId', 'node', 'indexer', 'indexerWS', 'proofServer']) {
    if (typeof networkConfig[key] !== 'string' || !networkConfig[key]) throw Error('reservation network binding missing: ' + key);
    network[key] = networkConfig[key];
  }
  const grossByAsset = {};
  if (!limits.grossByAsset || Object.getPrototypeOf(limits.grossByAsset) !== Object.prototype) throw Error('reservation asset limits required');
  for (const key of Object.keys(limits.grossByAsset).sort()) grossByAsset[asset(key)] = amount(limits.grossByAsset[key], 'asset allowance').toString();
  return {allocationId: limits.allocationId, reservationStatePath: limits.reservationStatePath, accountId: walletContext.unshieldedKeystore.getBech32Address().toString(), payer: walletContext.unshieldedKeystore.getPublicKey(), network,
    limits: {deadlineMs: integer(limits.deadlineMs, 'deadlineMs'), submissions: integer(limits.submissions, 'submissions'), dustFee: amount(limits.dustFee, 'dustFee').toString(), grossByAsset}};
}
function syncDirectory(path) { const fd = openSync(dirname(path), constants.O_RDONLY); try { fsyncSync(fd); } finally { closeSync(fd); } }
function safeStateParent(path) {
  for (let dir = dirname(path); ; dir = dirname(dir)) {
    const stat = lstatSync(dir);
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw Error('reservation path has a non-directory/symlink ancestor');
    if (dirname(dir) === dir) break;
  }
}
function reservationLock(path) {
  safeStateParent(path);
  let fd;
  try { fd = openSync(path + '.lock', constants.O_WRONLY | constants.O_CREAT | constants.O_EXCL | constants.O_NOFOLLOW, 0o600); }
  catch { throw Error('reservation lock exists or cannot be acquired; no automatic recovery'); }
  // If any fsync/write fails, leave the lock for explicit recovery.
  try { writeFileSync(fd, encode({pid: process.pid, nonce: randomUUID()})); fsyncSync(fd); } finally { closeSync(fd); }
  syncDirectory(path);
  let poisoned = false;
  return {
    write(record, fresh = false) {
      const temp = path + '.' + randomUUID() + '.tmp';
      try {
        const fd = openSync(temp, constants.O_WRONLY | constants.O_CREAT | constants.O_EXCL | constants.O_NOFOLLOW, 0o600);
        try { writeFileSync(fd, encode(record) + '\n'); fsyncSync(fd); } finally { closeSync(fd); }
        if (fresh) { linkSync(temp, path); unlinkSync(temp); } else renameSync(temp, path);
        syncDirectory(path);
      } catch (error) { poisoned = true; throw error; }
    },
    release() { if (!poisoned) { unlinkSync(path + '.lock'); syncDirectory(path); } },
  };
}
function readReservations(path, binding) {
  let state;
  try {
    const stat = lstatSync(path);
    if (!stat.isFile() || stat.isSymbolicLink() || (stat.mode & 0o077) !== 0 || (process.getuid && stat.uid !== process.getuid())) throw Error('unsafe reservation file');
    const fd = openSync(path, constants.O_RDONLY | constants.O_NOFOLLOW);
    try { state = JSON.parse(readFileSync(fd, 'utf8')); } finally { closeSync(fd); }
  } catch { throw Error('reservation state missing, corrupt, or unsafe; explicit recovery required'); }
  if (encode(state.binding) !== encode(binding)) throw Error('reservation allocation binding mismatch');
  if (Object.keys(state).sort().join(',') !== 'active,binding,identifiers,reservedDustFee,reservedGrossByAsset,reservedSubmissions,schema,stopped' || state.schema !== 'moriarty.financial-reservations/1') throw Error('reservation schema mismatch');
  const decimal = value => typeof value === 'string' && /^(0|[1-9][0-9]*)$/.test(value);
  if (!Number.isSafeInteger(state.reservedSubmissions) || state.reservedSubmissions < 0 || state.reservedSubmissions > binding.limits.submissions || !decimal(state.reservedDustFee) || BigInt(state.reservedDustFee) > BigInt(binding.limits.dustFee)) throw Error('reservation counters corrupt');
  if (!state.reservedGrossByAsset || Object.getPrototypeOf(state.reservedGrossByAsset) !== Object.prototype) throw Error('reservation asset counters corrupt');
  for (const [key, value] of Object.entries(state.reservedGrossByAsset)) if (!(key in binding.limits.grossByAsset) || !decimal(value) || BigInt(value) > BigInt(binding.limits.grossByAsset[key])) throw Error('reservation asset counters corrupt');
  if (typeof state.stopped !== 'boolean' || !Array.isArray(state.identifiers) || state.identifiers.some(id => typeof id !== 'string' || !id)) throw Error('reservation stop/identifiers corrupt');
  if (state.active !== null && (!state.active || Object.keys(state.active).sort().join(',') !== 'owner,phase' || typeof state.active.owner !== 'string' || !['balancing', 'reserved', 'finalized', 'submitting'].includes(state.active.phase))) throw Error('reservation active operation corrupt');
  return state;
}
/** Explicit allocation creation only. Never called by provider construction or recovery. */
export function initializeFinancialReservations(options) {
  const binding = bindingFor(options), path = options.limits.reservationStatePath;
  const lock = reservationLock(path);
  try {
    try { lstatSync(path + '.allocation'); throw Error('reservation allocation already exists; missing state requires recovery'); } catch (error) { if (error.code !== 'ENOENT') throw error; }
    try { lstatSync(path); throw Error('reservation state already exists'); } catch (error) { if (error.code !== 'ENOENT') throw error; }
    const allocationFd = openSync(path + '.allocation', constants.O_WRONLY | constants.O_CREAT | constants.O_EXCL | constants.O_NOFOLLOW, 0o600);
    try { writeFileSync(allocationFd, encode(binding) + '\n'); fsyncSync(allocationFd); } finally { closeSync(allocationFd); }
    syncDirectory(path);
    lock.write({schema: 'moriarty.financial-reservations/1', binding, reservedSubmissions: 0, reservedDustFee: '0', reservedGrossByAsset: {}, identifiers: [], active: null, stopped: false}, true);
  } finally { lock.release(); }
}
function approvedBalance(original, recipe, ledger, payer, ttl, networkId) {
  const base = recipe.baseTransaction;
  const originalSegments = [...original.intents.keys()];
  if (encode([...base.intents.keys()]) !== encode(originalSegments)) throw Error('balanced base changed approved segment identities');
  const normalized = ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', base.serialize());
  const normalizedIntents = normalized.intents;
  // Pinned wallet appends funding/change to each fallible offer, and guaranteed funding to the first intent.
  for (const [segment, expected] of original.intents) {
    const actual = base.intents.get(segment), restored = normalizedIntents.get(segment);
    for (const key of ['guaranteedUnshieldedOffer', 'fallibleUnshieldedOffer']) {
      const before = expected[key], after = actual[key];
      const oldInputs = before?.inputs ?? [], oldOutputs = before?.outputs ?? [], oldSignatures = before?.signatures ?? [];
      const inputs = after?.inputs ?? [], outputs = after?.outputs ?? [], signatures = after?.signatures ?? [];
      // The native intents setter canonicalizes offer order, so preserve multiplicity rather than JS prefix position.
      const additions = (before, after) => {
        const remaining = [...after];
        for (const row of before) {
          const index = remaining.findIndex(value => encode(value) === encode(row));
          if (index < 0) throw Error('balanced offer changed original approved effects');
          remaining.splice(index, 1);
        }
        return remaining;
      };
      const addedInputs = additions(oldInputs, inputs), addedOutputs = additions(oldOutputs, outputs);
      if (encode(signatures) !== encode(oldSignatures)) throw Error('balanced offer changed original approved signatures');
      const imbalanceSegment = key === 'guaranteedUnshieldedOffer' ? 0 : segment;
      const imbalances = new Map([...original.imbalances(imbalanceSegment)].filter(([token, value]) => token.tag === 'unshielded' && value !== 0n).map(([token, value]) => [token.raw, value]));
      if (key === 'guaranteedUnshieldedOffer' && segment !== originalSegments[0] && (addedInputs.length || addedOutputs.length)) throw Error('balanced offer added funding outside approved segment');
      for (const input of addedInputs) if (input.owner !== payer || !imbalances.has(input.type) || imbalances.get(input.type) >= 0n) throw Error('unnecessary or unauthorized payer funding');
      for (const output of addedOutputs) if (output.owner !== ledger.addressFromKey(payer) || !imbalances.has(output.type)) throw Error('unauthorized balance change output');
      restored[key] = before;
    }
    normalizedIntents.set(segment, restored);
  }
  normalized.intents = normalizedIntents;
  // Removing only checked added funding must recover every original byte, including actions/transcripts/TTL/network.
  if (!Buffer.from(normalized.serialize()).equals(Buffer.from(original.serialize()))) throw Error('balance changed original approved actions, transcripts or metadata');
  for (const segment of [...originalSegments, 0]) for (const [token, value] of base.imbalances(segment)) if (token.tag === 'unshielded' && value !== 0n) throw Error('balanced approved unshielded effects are not balanced');
  if (recipe.balancingTransaction !== undefined) {
    const fee = recipe.balancingTransaction;
    let firstFree = 1; while (original.intents.has(firstFree)) firstFree += 1;
    if (fee.intents.size !== 1 || !fee.intents.has(firstFree)) throw Error('unexpected DUST balancing segment');
    const intent = fee.intents.get(firstFree);
    if (intent.actions.length || intent.guaranteedUnshieldedOffer !== undefined || intent.fallibleUnshieldedOffer !== undefined || !intent.dustActions || intent.ttl.getTime() !== Math.floor(ttl.getTime() / 1000) * 1000) throw Error('only DUST balancing actions are authorized');
    const network = fee.toString(true).match(/^StandardTransaction \{ network_id: "([^"]+)"/);
    if (!network || network[1] !== networkId) throw Error('DUST balancing network differs from allocation');
  }
}

/** Fixed unshielded loan/swap provider. Requires an existing funded, DUST-registered wallet.
 * limits.deadlineMs is an absolute epoch deadline. Reservations are cumulative, never refunded.
 * sdk injection is for explicitly inert source tests; omit it for the pinned constructors.
 */
export async function createFinancialProviders({walletContext, networkConfig, zkConfigPath, privateStateConfig, limits, ledger, sdk, onEvent = () => {}} = {}) {
  if (!limits || !ledger?.Transaction || typeof ledger.verifySignature !== 'function') throw Error('limits and native ledger required');
  const deadline = integer(limits.deadlineMs, 'deadlineMs');
  const maxSubmissions = integer(limits.submissions, 'submissions');
  const maxDustFee = amount(limits.dustFee, 'dustFee');
  if (!limits.grossByAsset || Object.getPrototypeOf(limits.grossByAsset) !== Object.prototype) throw Error('grossByAsset allowance required');
  const caps = new Map(Object.entries(limits.grossByAsset).map(([key, value]) => [asset(key), amount(value, 'asset allowance')]));
  if (Date.now() >= deadline) throw Error('financial deadline exceeded');
  if (!walletContext?.wallet || !walletContext.unshieldedKeystore || !walletContext.shieldedSecretKeys || !walletContext.dustSecretKey) throw Error('existing WalletContext required');
  const wallet = walletContext.wallet;
  for (const key of ['balanceUnboundTransaction', 'signRecipe', 'finalizeRecipe', 'submitTransaction', 'stop']) if (typeof wallet[key] !== 'function') throw Error('wallet method missing: ' + key);
  if (!isAbsolute(zkConfigPath ?? '') || !isAbsolute(privateStateConfig?.midnightDbName ?? '') || zkConfigPath === privateStateConfig.midnightDbName) throw Error('distinct absolute proven-asset and private-state locations required');
  if (typeof privateStateConfig.privateStoragePasswordProvider !== 'function' || typeof privateStateConfig.privateStateStoreName !== 'string' || !privateStateConfig.privateStateStoreName) throw Error('private-state store and secret password provider required');
  if (Object.keys(privateStateConfig).some(k => !['midnightDbName', 'privateStateStoreName', 'privateStoragePasswordProvider', 'signingKeyStoreName'].includes(k))) throw Error('unknown private-state option');
  const accountId = walletContext.unshieldedKeystore.getBech32Address().toString();
  const payer = walletContext.unshieldedKeystore.getPublicKey();
  if (typeof accountId !== 'string' || !accountId.trim() || typeof payer !== 'string') throw Error('existing wallet account and verifying key required');
  for (const [key, protocols] of [['indexer', ['http:', 'https:']], ['indexerWS', ['ws:', 'wss:']], ['proofServer', ['http:', 'https:']]]) {
    if (!protocols.includes(new URL(networkConfig?.[key]).protocol)) throw Error('invalid network endpoint: ' + key);
  }
  if (typeof onEvent !== 'function') throw Error('onEvent required');
  let stopped = false, reason, reservedSubmissions = 0, reservedDustFee = 0n, busy = false, cleanupPromise;
  const reservedGross = new Map(), ids = new Set(), issued = new Map(), pending = new Set();
  const binding = bindingFor({walletContext, networkConfig, limits}), statePath = limits.reservationStatePath, owner = randomUUID();
  const selectedNetwork = Object.freeze({...binding.network});
  let currentLock, record;
  function hydrate(state) {
    record = state; reservedSubmissions = state.reservedSubmissions; reservedDustFee = BigInt(state.reservedDustFee);
    reservedGross.clear(); for (const [key, value] of Object.entries(state.reservedGrossByAsset)) reservedGross.set(key, BigInt(value));
    for (const id of state.identifiers) ids.add(id);
  }
  function persist() {
    if (!currentLock) throw Error('reservation lock not held');
    record.reservedSubmissions = reservedSubmissions; record.reservedDustFee = reservedDustFee.toString();
    record.reservedGrossByAsset = Object.fromEntries([...reservedGross].map(([key, value]) => [key, value.toString()]));
    record.identifiers = [...ids]; currentLock.write(record);
  }
  { const lock = reservationLock(statePath); try { hydrate(readReservations(statePath, binding)); } finally { lock.release(); } }
  if (record.stopped || record.active !== null) { stopped = true; reason = 'persisted stop or unresolved financial operation'; }
  function getState() { return {stopped, reason, reservedSubmissions, reservedDustFee, reservedGrossByAsset: Object.fromEntries(reservedGross), identifiers: [...ids], pendingOperations: pending.size}; }
  function stop(why = 'caller stop') {
    stopped = true; reason ??= why;
    if (why === 'cleanup' && record.active === null) return;
    if (currentLock) { record.stopped = true; persist(); return; }
    let lock;
    try { lock = reservationLock(statePath); const state = readReservations(statePath, binding); state.stopped = true; state.identifiers = [...new Set([...state.identifiers, ...ids])]; lock.write(state); }
    catch { /* Existing/poisoned lock itself bars reconstruction; local stop remains latched. */ }
    finally { lock?.release(); }
  }
  function checkDeadline() {
    if (stopped) throw Error('financial provider stopped: ' + reason);
    if (!currentLock) {
      const lock = reservationLock(statePath);
      try {
        const state = readReservations(statePath, binding);
        if (state.stopped || (state.active !== null && state.active.owner !== owner)) { stopped = true; reason = 'persisted stop or unresolved financial operation'; throw Error('financial provider stopped: ' + reason); }
      } finally { lock.release(); }
    }
    if (Date.now() >= deadline) { stop('deadline exceeded'); throw Error('financial deadline exceeded'); }
  }
  async function execute(label, operation) {
    checkDeadline();
    let timer;
    const task = Promise.resolve().then(() => { checkDeadline(); return operation(); });
    pending.add(task);
    task.then(() => pending.delete(task), () => pending.delete(task));
    try {
      const result = await Promise.race([task, new Promise((_, reject) => { timer = setTimeout(() => reject(Error('financial deadline exceeded: ' + label)), Math.min(deadline - Date.now(), 2147483647)); })]);
      checkDeadline();
      return result;
    } catch (error) {
      stop('ambiguous or failed ' + label);
      // Do not expose provider exception text or private transaction/witness data in events.
      try { onEvent({kind: 'stopped', operation: label, identifiers: [...ids], reservationRetained: true, pendingOperations: pending.size}); } catch { /* stop already latched */ }
      throw error;
    } finally { clearTimeout(timer); }
  }
  function guarded(provider) {
    return new Proxy(provider, {get(target, key) {
      const value = Reflect.get(target, key, target);
      if (typeof value !== 'function') return value;
      return (...args) => {
        checkDeadline();
        // Preserve synchronous SDK methods such as setContractAddress.
        if (value.constructor.name !== 'AsyncFunction') {
          try {
            const result = value.apply(target, args);
            return result?.then ? execute(String(key), () => result) : result;
          } catch (error) { stop('provider operation failed: ' + String(key)); throw error; }
        }
        return execute(String(key), () => value.apply(target, args));
      };
    }});
  }
  function checkAllowance({gross, dustFee}) {
    if (reservedSubmissions >= maxSubmissions) throw Error('submission allowance exceeded');
    for (const [type, value] of Object.entries(gross)) if (!caps.has(type) || value + (reservedGross.get(type) ?? 0n) > caps.get(type)) throw Error('gross asset allowance exceeded');
    if (reservedDustFee + dustFee > maxDustFee) throw Error('DUST fee allowance exceeded');
  }
  async function exclusive(kind, operation) {
    checkDeadline();
    if (busy) throw Error('concurrent financial operation unsupported');
    const lock = reservationLock(statePath);
    try {
      hydrate(readReservations(statePath, binding));
      if (record.stopped || (record.active !== null && (kind !== 'submit' || record.active.owner !== owner || record.active.phase !== 'finalized'))) throw Error('persisted stop or unresolved reservation operation');
      currentLock = lock; busy = true;
      try { return await operation(); } catch (error) { stop('financial operation failed'); throw error; }
      finally { busy = false; currentLock = undefined; }
    } finally { lock.release(); }
  }
  const walletProvider = {
    getCoinPublicKey() { checkDeadline(); return walletContext.shieldedSecretKeys.coinPublicKey; },
    getEncryptionPublicKey() { checkDeadline(); return walletContext.shieldedSecretKeys.encryptionPublicKey; },
    balanceTx(tx, ttl = new Date(deadline)) { return exclusive('balance', async () => {
      nativeTransaction(tx, ledger);
      ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', tx.serialize());
      if (!(ttl instanceof Date) || !Number.isFinite(ttl.getTime()) || ttl.getTime() <= Date.now()) throw Error('invalid transaction ttl');
      checkAllowance(inspect([tx], ledger, payer, false));
      const original = ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', tx.serialize());
      record.active = {owner, phase: 'balancing'}; persist();
      const recipe = await execute('balance', () => wallet.balanceUnboundTransaction(tx, {shieldedSecretKeys: walletContext.shieldedSecretKeys, dustSecretKey: walletContext.dustSecretKey}, {ttl, tokenKindsToBalance: ['unshielded', 'dust']}));
      const unsigned = parts(recipe, ledger);
      approvedBalance(original, recipe, ledger, payer, ttl, selectedNetwork.networkId);
      checkAllowance(inspect(unsigned, ledger, payer, false));
      const beforeSign = semanticBytes(unsigned.map(t => t.eraseSignatures()));
      const signed = await execute('sign', () => wallet.signRecipe(recipe, data => { checkDeadline(); return walletContext.unshieldedKeystore.signData(data); }));
      const signedParts = parts(signed, ledger);
      if (!beforeSign.equals(semanticBytes(signedParts.map(t => t.eraseSignatures())))) throw Error('signing changed semantic bytes');
      const budget = inspect(signedParts, ledger, payer, true);
      checkAllowance(budget);
      const approved = semanticBytes(signedParts); // Snapshot before facade can mutate objects.
      checkDeadline();
      reservedSubmissions += 1;
      reservedDustFee += budget.dustFee;
      for (const [type, value] of Object.entries(budget.gross)) reservedGross.set(type, (reservedGross.get(type) ?? 0n) + value);
      for (const part of signedParts) for (const id of part.identifiers()) ids.add(id);
      record.active = {owner, phase: 'reserved'}; persist();
      const finalized = await execute('finalize', () => wallet.finalizeRecipe(signed));
      nativeTransaction(finalized, ledger);
      if (!approved.equals(semanticBytes([finalized]))) throw Error('finalization changed semantic bytes');
      // The pinned native deserializer enforces the finalized marker, independent of JS shapes.
      ledger.Transaction.deserialize('signature', 'proof', 'binding', finalized.serialize());
      const bytes = Buffer.from(finalized.serialize());
      for (const id of finalized.identifiers()) ids.add(id);
      issued.set(digest(bytes), {bytes, used: false});
      record.active = {owner, phase: 'finalized'}; persist();
      return finalized;
    }); },
  };
  const midnightProvider = {submitTx(tx) { return exclusive('submit', async () => {
    nativeTransaction(tx, ledger);
    const bytes = Buffer.from(tx.serialize()), ticket = issued.get(digest(bytes));
    if (!ticket || !ticket.bytes.equals(bytes) || ticket.used) throw Error('transaction was not issued or was already submitted');
    checkDeadline(); ticket.used = true;
    record.active = {owner, phase: 'submitting'}; persist();
    const txIdentifiers = tx.identifiers();
    const txHash = tx.transactionHash();
    // Capture notification even if the facade resolves after our deadline race.
    const id = await execute('submit', async () => {
      const observedId = await wallet.submitTransaction(tx);
      if (typeof observedId !== 'string' || !txIdentifiers.includes(observedId)) throw Error('submission returned an unrelated identifier');
      ids.add(observedId);
      onEvent({kind: 'submitted', txId: observedId, identifiers: txIdentifiers, transactionHash: txHash});
      return observedId;
    });
    record.active = null; persist();
    return id;
  }); }};
  sdk ??= await loadFinancialSdk();
  if (!stopped) checkDeadline();
  if (typeof sdk.getNetworkId !== 'function') throw Error('pinned SDK network accessor required');
  function getExecutionBinding() { return Object.freeze({network: selectedNetwork, payerAddress: ledger.addressFromKey(payer), sdkNetworkId: sdk.getNetworkId()}); }
  const zkConfigProvider = new sdk.NodeZkConfigProvider(zkConfigPath);
  const privateStateProvider = sdk.levelPrivateStateProvider({...privateStateConfig, accountId});
  const publicDataProvider = sdk.indexerPublicDataProvider(selectedNetwork.indexer, selectedNetwork.indexerWS);
  const proofProvider = sdk.httpClientProofProvider(selectedNetwork.proofServer, zkConfigProvider);
  async function cleanup() {
    if (cleanupPromise) return cleanupPromise;
    stop('cleanup');
    cleanupPromise = (async () => {
      let timer;
      try {
        await Promise.race([Promise.resolve().then(() => wallet.stop()), new Promise((_, reject) => { timer = setTimeout(() => reject(Error('wallet cleanup timeout')), 5000); })]);
        // Pinned level provider closes its database per operation. Indexer exposes no dispose API.
        return {walletStopped: true, pendingOperations: pending.size, containmentComplete: false, indexerDisposal: 'not-exposed-by-pinned-sdk'};
      } catch { return {walletStopped: false, pendingOperations: pending.size, containmentComplete: false, indexerDisposal: 'not-exposed-by-pinned-sdk'}; }
      finally { clearTimeout(timer); }
    })();
    return cleanupPromise;
  }
  return {walletProvider, midnightProvider, zkConfigProvider: guarded(zkConfigProvider), privateStateProvider: guarded(privateStateProvider), publicDataProvider: guarded(publicDataProvider), proofProvider: guarded(proofProvider), cleanup, stop, getState, getExecutionBinding, checkDeadline, execute};
}
