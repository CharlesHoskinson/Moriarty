import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join, isAbsolute} from 'node:path';
import {pathToFileURL} from 'node:url';

export const PINNED_NM = '/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';
// Direct provider entry bytes are pinned; this is not a transitive supply-chain attestation.
const PINS = {
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
  function getState() { return {stopped, reason, reservedSubmissions, reservedDustFee, reservedGrossByAsset: Object.fromEntries(reservedGross), identifiers: [...ids], pendingOperations: pending.size}; }
  function stop(why = 'caller stop') { stopped = true; reason ??= why; }
  function checkDeadline() {
    if (stopped) throw Error('financial provider stopped: ' + reason);
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
  async function exclusive(operation) {
    checkDeadline();
    if (busy) throw Error('concurrent financial operation unsupported');
    busy = true;
    try { return await operation(); } catch (error) { stop('financial operation failed'); throw error; } finally { busy = false; }
  }
  const walletProvider = {
    getCoinPublicKey() { checkDeadline(); return walletContext.shieldedSecretKeys.coinPublicKey; },
    getEncryptionPublicKey() { checkDeadline(); return walletContext.shieldedSecretKeys.encryptionPublicKey; },
    balanceTx(tx, ttl = new Date(deadline)) { return exclusive(async () => {
      nativeTransaction(tx, ledger);
      ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', tx.serialize());
      if (!(ttl instanceof Date) || !Number.isFinite(ttl.getTime()) || ttl.getTime() <= Date.now()) throw Error('invalid transaction ttl');
      checkAllowance(inspect([tx], ledger, payer, false));
      const recipe = await execute('balance', () => wallet.balanceUnboundTransaction(tx, {shieldedSecretKeys: walletContext.shieldedSecretKeys, dustSecretKey: walletContext.dustSecretKey}, {ttl, tokenKindsToBalance: ['unshielded', 'dust']}));
      const unsigned = parts(recipe, ledger);
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
      const finalized = await execute('finalize', () => wallet.finalizeRecipe(signed));
      nativeTransaction(finalized, ledger);
      if (!approved.equals(semanticBytes([finalized]))) throw Error('finalization changed semantic bytes');
      // The pinned native deserializer enforces the finalized marker, independent of JS shapes.
      ledger.Transaction.deserialize('signature', 'proof', 'binding', finalized.serialize());
      const bytes = Buffer.from(finalized.serialize());
      for (const id of finalized.identifiers()) ids.add(id);
      issued.set(digest(bytes), {bytes, used: false});
      return finalized;
    }); },
  };
  const midnightProvider = {submitTx(tx) { return exclusive(async () => {
    nativeTransaction(tx, ledger);
    const bytes = Buffer.from(tx.serialize()), ticket = issued.get(digest(bytes));
    if (!ticket || !ticket.bytes.equals(bytes) || ticket.used) throw Error('transaction was not issued or was already submitted');
    checkDeadline(); ticket.used = true;
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
    return id;
  }); }};
  sdk ??= await loadFinancialSdk();
  checkDeadline();
  const zkConfigProvider = new sdk.NodeZkConfigProvider(zkConfigPath);
  const privateStateProvider = sdk.levelPrivateStateProvider({...privateStateConfig, accountId});
  const publicDataProvider = sdk.indexerPublicDataProvider(networkConfig.indexer, networkConfig.indexerWS);
  const proofProvider = sdk.httpClientProofProvider(networkConfig.proofServer, zkConfigProvider);
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
  return {walletProvider, midnightProvider, zkConfigProvider: guarded(zkConfigProvider), privateStateProvider: guarded(privateStateProvider), publicDataProvider: guarded(publicDataProvider), proofProvider: guarded(proofProvider), cleanup, stop, getState, checkDeadline, execute};
}
