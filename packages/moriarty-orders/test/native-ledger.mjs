// Local SDK driver adapted from input-output-hk/SOTA-DeFiFormal-Moriarty at
// 1ced5fe1e0a83d665184bb496bea01bcb9940449, moriarty/adapters/moriarty_self/native_ledger/ledger.mjs.
// Local test genesis and public test keys; never use this wallet on a public network.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync, symlinkSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';

export const encode = x => JSON.stringify(
    x,
    (_k, v) => typeof v === 'bigint' ? v.toString() :
        v instanceof Set             ? [...v] :
        v instanceof Map             ? [...v] :
                                       v,
    2);
export const hash = b => createHash('sha256').update(b).digest('hex');
export const bytes = n => new Uint8Array(32).fill(n);
const coinId = x => `${x.intentHash}:${x.outputNo}:${x.type}`;
const noProof = {
  lookupKey: async () => { throw Error('Unexpected proving key query in bootstrap'); },
  prove: async () => {
    throw Error('Unexpected proof required in bootstrap');
  },
  check: async () => {
    throw Error('Unexpected proof check in bootstrap');
  }
};
export class NativeLedger {
  static async create(options = {}) {
    const h = new NativeLedger();
    h.modules = resolve(options.modules);
    const require = createRequire(join(h.modules, '../package.json'));
    h.load = async name => import(pathToFileURL(require.resolve('@midnight-ntwrk/'+name)));
    h.L = await h.load('midnight-js-protocol/v9');
    h.runtime = await h.load('compact-runtime');
    h.network = 'undeployed';
    h.actors = {};
    h.proofServer = options.proofServer;
    h.retain = options.retain;
    for (const [name, seed] of [['owner', 31], ['recipient', 33], ['executor', 35]]) {
      const key = h.L.signingKeyFromBip340(bytes(seed)), vk = h.L.signatureVerifyingKey(key);
      h.actors[name] = {key, vk, address: h.L.addressFromKey(vk)};
    }
    h.dustKey = h.L.DustSecretKey.fromSeed(bytes(32));
    h.state = h.L.LedgerState.blank(h.network);
    h.time = 0;
    h.records = [];
    h.bootstrap = [];
    h.funding = new Map();
    h.clearReservations();
    const amount = 1000000000000n, {key, vk, address} = h.actors.owner;
    h.state = h.state.testingDistributeNight(address, amount, h.date);
    let rewards = h.L.ClaimRewardsTransaction.new(h.network, amount, vk, '19'.repeat(32), 'Reward');
    rewards = rewards.addSignature(h.L.signData(key, rewards.dataToSign));
    // Rewards have signatures but no proof-bearing components. fromRewards is
    // already bound; native wellFormed validates this variant directly.
    h.bootstrap.push(h.apply(h.L.Transaction.fromRewards(rewards), 'setup-night-reward'));
    h.time = Number(h.params.dust.timeToCapSeconds) + 1000;
    const coin = [...h.state.utxo.filter(address)][0];
    assert.ok(coin);
    let intent = h.L.Intent.new(h.ttl);
    intent.guaranteedUnshieldedOffer = h.L.UnshieldedOffer.new(
        [{...coin, owner: vk}],
        Array.from(
            {length: 4}, () => ({owner: address, type: h.L.nativeToken().raw, value: amount / 4n})),
        []);
    const registration = new h.L.DustRegistration(
        'signature', vk, h.dustKey.publicKey, amount * h.params.dust.nightDustRatio);
    intent.dustActions = new h.L.DustActions('signature', 'pre-proof', h.date, [], [registration]);
    const sig = h.L.signData(key, intent.signatureData(1));
    intent.guaranteedUnshieldedOffer = intent.guaranteedUnshieldedOffer.addSignatures([new h.L.SignatureEnabled(sig)]);
    registration.signature = new h.L.SignatureEnabled(sig);
    intent.dustActions = new h.L.DustActions('signature', 'pre-proof', h.date, [], [registration]);
    const tx = await h.L.Transaction.fromParts(h.network, undefined, undefined, intent)
                   .prove(noProof, h.params.transactionCostModel.runtimeCostModel);
    h.dust = new h.L.DustLocalState(h.params.dust);
    h.bootstrap.push(h.apply(tx.bind(), 'setup-dust-registration'));
    h.time += Number(h.params.dust.timeToCapSeconds) + 1;
    return h;
  }
  get date() {
    return new Date(this.time * 1000);
  }
  get ttl() {
    return new Date((this.time + 3600) * 1000);
  }
  get params() {
    return this.state.parameters;
  }
  strictFlags() {
    const s = new this.L.WellFormedStrictness();
    return Object.fromEntries([
      'enforceBalancing', 'verifyNativeProofs', 'verifyContractProofs', 'enforceLimits',
      'verifySignatures'
    ].map(k => [k, s[k]]));
  }
  snapshot() {
    return {state: this.state.serialize(), dust: this.dust.serialize(), time: this.time};
  }
  restore(s) {
    this.state = this.L.LedgerState.deserialize(s.state);
    this.dust = this.L.DustLocalState.deserialize(s.dust);
    this.time = s.time;
  }
  balance(address, type) {
    return [...this.state.utxo.filter(address)]
        .filter(x => x.type === type)
        .reduce((a, x) => a + x.value, 0n);
  }
  clearReservations() {
    this.reservedCoins = new Set();
    this.reservedDust = new Set();
  }
  reserveFunding(tx) {
    const f = this.funding.get(hash(tx.serialize()));
    assert.ok(f, 'unrecorded funded transaction');
    for (const id of f.coins) this.reservedCoins.add(id);
    this.reservedDust.add(f.dust);
  }
  async splitAsset(actor, type, count, contract, label = 'setup-split-funding') {
    const coins = [...this.state.utxo.filter(actor.address)].filter(c => c.type === type);
    const amount = coins.reduce((a, c) => a + c.value, 0n);
    assert.ok(amount >= BigInt(count));
    const each = amount / BigInt(count),
          outputs = Array.from(
              {length: count}, (_, i) => ({
                                 owner: actor.address,
                                 type,
                                 value: i === count - 1 ? amount - each * BigInt(count - 1) : each
                               }));
    const intent = this.L.Intent.new(this.ttl);
    intent.guaranteedUnshieldedOffer =
        this.L.UnshieldedOffer.new(coins.map(c => ({...c, owner: actor.vk})), outputs, []);
    const tx = await this.fundAndProve(
        this.L.Transaction.fromParts(this.network, undefined, undefined, intent), contract, actor);
    return {tx, record: this.apply(tx, label)};
  }
  apply(tx, label, {allowFailure = false} = {}) {
    const before = this.state.serialize(), strict = new this.L.WellFormedStrictness();
    assert.ok(Object.values(this.strictFlags()).every(Boolean));
    const dustBefore = this.dust?.walletBalance(this.date);
    const paymentTimes = [...(tx.intents?.values() ?? [])]
                             .filter(i => i.dustActions?.spends.length)
                             .map(i => i.dustActions.ctime);
    const paymentBalancesBefore = paymentTimes.map(
        time => ({time: time.valueOf() / 1000, balance: this.dust.walletBalance(time)}));
    const verified = tx.wellFormed(this.state, strict, this.date);
    const context = new this.L.TransactionContext(this.state, {
      secondsSinceEpoch: BigInt(this.time),
      secondsSinceEpochErr: 0,
      parentBlockHash: '00'.repeat(32),
      lastBlockTime: BigInt(this.time)
    });
    let result;
    [this.state, result] = this.state.apply(verified, context);
    if (this.dust) this.dust = this.dust.replayEvents(this.dustKey, result.events);
    const record = {
      label,
      time: this.time,
      status: result.type,
      error: result.error ?? null,
      successfulSegments: result.successfulSegments ?? [],
      transactionHash: tx.rewards ? null : tx.transactionHash(),
      transaction: tx.serialize(),
      beforeStateHash: hash(before),
      afterStateHash: hash(this.state.serialize()),
      requiredFeeSpeck: tx.fees(this.params),
      resources: tx.cost(this.params),
      dustBefore,
      dustAfter: this.dust?.walletBalance(this.date),
      strict: this.strictFlags()
    };
    record.executionBoundary = 'native-ledger-state-apply';
    record.serializedTransactionSha256 = hash(record.transaction);
    record.beforeState = before;
    record.afterState = this.state.serialize();
    record.dustTimeBefore = this.time;
    record.dustTimeAfter = this.time;
    record.dustPayments = [...(tx.intents ?? [])].flatMap(([segment, intent]) => {
      if (!intent.dustActions) return [];
      return intent.dustActions.spends.map(spend => ({
                                             segment,
                                             creationTime: intent.dustActions.ctime.valueOf() / 1000,
                                             vFee: spend.vFee,
                                             oldNullifier: spend.oldNullifier,
                                             newCommitment: spend.newCommitment
                                           }));
    });
    record.dustSpendEvents =
        result.events.filter(e => e.content.tag === 'dustSpendProcessed')
            .map(e => ({source: e.source, content: e.content, serialized: e.serialize()}));
    for (const event of record.dustSpendEvents) {
      assert.equal(event.source.transactionHash, record.transactionHash);
      assert.ok(
          record.dustPayments.some(
              p => p.segment === event.source.physicalSegment &&
                  p.oldNullifier === event.content.nullifier &&
                  p.newCommitment === event.content.commitment && p.vFee === event.content.vFee),
          'accepted DUST event does not match signed payment');
    }
    for (const p of record.dustPayments)
      p.accepted = record.dustSpendEvents.some(
          e => e.source.physicalSegment === p.segment && e.content.nullifier === p.oldNullifier);
    record.nativeFeeSpeck = record.dustSpendEvents.reduce((n, e) => n + e.content.vFee, 0n);
    if ([...(tx.intents?.values() ?? [])].some(i => i.dustActions?.registrations.length))
      record.nativeFeeSpeck = null;
    record.paymentTimeBalances = paymentBalancesBefore.map(
        x => ({...x, after: this.dust.walletBalance(new Date(x.time * 1000))}));
    // Finalize the block's DUST roots before building a fee proof against its successor.
    // Retain the transaction post-state and the finalized parent as separate identities.
    this.state = this.state.postBlockUpdate(this.date);
    record.afterBlockStateSha256 = hash(this.state.serialize());
    this.time += 1;
    this.retain(record);
    this.records.push(record);
    if (!allowFailure) assert.equal(result.type, 'success', encode({label, result}));
    return record;
  }
  async loadContract(directory) {
    directory = resolve(directory);
    if (!existsSync(join(directory, 'node_modules')))
      symlinkSync(this.modules, join(directory, 'node_modules'), 'dir');
    const generated = await import(pathToFileURL(join(directory, 'contract/index.js')));
    const {NodeZkConfigProvider} = await this.load('midnight-js-node-zk-config-provider');
    const {httpClientProvingProvider} = await this.load('midnight-js-http-client-proof-provider');
    const zk = new NodeZkConfigProvider(directory);
    return {
      directory,
      generated,
      instance: new generated.Contract({}),
      zk,
      prover: httpClientProvingProvider(this.proofServer, zk)
    };
  }
  async deploy(contract, args, label) {
    const {L,runtime} = this;
    const context = runtime.createConstructorContext({},'00'.repeat(32));
    const initial = await contract.instance.initialState(context,...args);
    const state = L.ContractState.deserialize(initial.currentContractState.serialize());
    for (const name of Object.keys(contract.instance.provableCircuits)) {
      const operation = state.operation(name);
      operation.verifierKey = await contract.zk.getVerifierKey(name);
      state.setOperation(name,operation);
    }
    state.maintenanceAuthority = new L.ContractMaintenanceAuthority([this.actors.owner.vk],1);
    const deploy = new L.ContractDeploy(state);
    const intent = L.Intent.new(this.ttl).addDeploy(deploy);
    const tx = await this.fundAndProve(
        this.L.Transaction.fromParts(this.network, undefined, undefined, intent), contract);
    const record = this.apply(tx, label);
    return {address: deploy.address, contract, record};
  }
  async prepareCall(instance, circuit, args) {
    const {L,runtime} = this, {address,contract} = instance;
    const ledgerState = this.state.index(address);
    assert.ok(ledgerState);
    const compactState = runtime.ContractState.deserialize(ledgerState.serialize());
    const ctx = runtime.createCircuitContext({circuitId:circuit,contractAddress:address,
      coinPublicKeyOrZswapState:'00'.repeat(32),contractState:compactState,privateState:{},time:this.time});
    const initial = ctx.callContext.currentQueryContext;
    initial.block = {...initial.block,secondsSinceEpoch:BigInt(this.time),secondsSinceEpochErr:0,lastBlockTime:BigInt(this.time)};
    const executed = await contract.instance.provableCircuits[circuit](ctx,...args);
    assert.equal(executed.context.callProofDataTrace.length,1);
    const p = executed.context.callProofDataTrace[0];
    let query = new L.QueryContext(ledgerState.data,address);
    query.block = initial.block;
    query.effects = initial.effects;
    for (const entry of executed.context.callContext.currentQueryContext.comIndices)
      query = query.insertCommitment(...entry);
    const pre = new L.PreTranscript(query,p.publicTranscript);
    const call = new L.PrePartitionContractCall(
        address, circuit, ledgerState.operation(circuit), pre, p.privateTranscriptOutputs, p.input,
        p.output, L.communicationCommitmentRandomness(), circuit);
    return {call, pre, executed, instance, circuit};
  }
  assemble(prepared) {
    const {L} = this;
    let tx = L.Transaction.fromParts(this.network)
                 .addCalls({tag: 'first'}, prepared.map(x => x.call), this.params, this.ttl);
    // Native addCalls partitions the whole batch. Claimed user outputs must be
    // carried in the corresponding native offers, just as in midnight-js.
    for (const [segment, intent] of tx.intents ?? []) {
      for (const [property, field] of [
               ['guaranteedUnshieldedOffer', 'guaranteedTranscript'],
               ['fallibleUnshieldedOffer', 'fallibleTranscript']]) {
        const outputs = [];
        for (const action of intent.actions) {
          const transcript = action[field];
          if (transcript)
            for (const [[type, to], value] of transcript.effects.claimedUnshieldedSpends)
              if (to.tag === 'user' && type.tag !== 'dust')
                outputs.push({type: type.raw, owner: to.address, value});
        }
        if (outputs.length) intent[property] = L.UnshieldedOffer.new([], outputs, []);
      }
      tx.intents = tx.intents.set(segment, intent);
    }
    return tx;
  }
  balanceTokens(tx, payer) {
    const {L} = this;
    const used = new Set();
    for (const [segment, intent] of tx.intents ?? []) {
      for (const [logical, property] of [
               [0, 'guaranteedUnshieldedOffer'], [segment, 'fallibleUnshieldedOffer']]) {
        const inputs = [...(intent[property]?.inputs ?? [])],
              outputs = [...(intent[property]?.outputs ?? [])];
        for (const [type, imbalance] of tx.imbalances(logical, 0n)) {
          if (type.tag === 'dust' || imbalance >= 0n) continue;
          let total = 0n;
          for (const coin of this.state.utxo.filter(payer.address)) {
            const id = coinId(coin);
            if (coin.type !== type.raw || used.has(id) || this.reservedCoins.has(id)) continue;
            inputs.push({...coin, owner: payer.vk});
            used.add(id);
            total += coin.value;
            if (total >= -imbalance) break;
          }
          assert.ok(total >= -imbalance, 'insufficient native token funding');
          if (total + imbalance > 0n)
            outputs.push({owner: payer.address, type: type.raw, value: total + imbalance});
        }
        if (inputs.length || outputs.length)
          intent[property] = L.UnshieldedOffer.new(inputs, outputs, []);
      }
      const sig = L.signData(payer.key, intent.signatureData(segment));
      for (const prop of ['guaranteedUnshieldedOffer', 'fallibleUnshieldedOffer'])
        if (intent[prop]) intent[prop] = intent[prop].addSignatures(intent[prop].inputs.map(() => new L.SignatureEnabled(sig)));
      tx.intents = tx.intents.set(segment, intent);
    }
    return tx;
  }
  async fundAndProve(tx, contract, payer = this.actors.owner) {
    return this.fundBalancedAndProve(this.balanceTokens(tx, payer), contract, () => payer);
  }
  async fundBalancedAndProve(tx, contract, walletForAddress) {
    const {L} = this;
    const proven = await tx.prove(contract.prover, this.params.transactionCostModel.runtimeCostModel);
    let fee = proven.fees(this.params), feeTx, iterations = 0;
    const feeCoin = this.dust.utxos.find(x => !this.reservedDust.has(x.nonce));
    assert.ok(feeCoin, 'no unreserved DUST coin');
    const segment = [...Array(65535)].map((_, i) => i + 1).find(i => !proven.intents?.has(i));
    do {
      assert.ok(++iterations <= 8, 'fee convergence failed');
      const [, spend] = this.dust.spend(this.dustKey, feeCoin, fee, this.date);
      const intent = L.Intent.new(this.ttl);
      intent.dustActions = new L.DustActions('signature', 'pre-proof', this.date, [spend], []);
      feeTx = L.Transaction.fromParts(this.network).addIntent({tag: 'specific', value: segment}, intent);
      // Same proof-erased fee estimation used by the pinned wallet. No mocked
      // proofs are submitted; both the contract and DUST spend are really proved.
      const required = proven.eraseProofs().merge(feeTx.eraseProofs()).fees(this.params);
      if (required > fee) {
        fee = required;
        continue;
      }
      const provenFee =
          await feeTx.prove(contract.prover, this.params.transactionCostModel.runtimeCostModel);
      const merged = proven.merge(provenFee);
      if (walletForAddress) {
        // A versioned contract proof can change the final signature preimage. Sign
        // after proving and fee assembly, while the transaction is still mutable.
        for (const [id, intent] of merged.intents) {
          const message = intent.signatureData(id);
          for (const property of ['guaranteedUnshieldedOffer', 'fallibleUnshieldedOffer']) {
            const offer = intent[property];
            if (!offer?.inputs.length) continue;
            const signatures = offer.inputs.map(coin => {
              const address = L.addressFromKey(coin.owner);
              const wallet = walletForAddress(address);
              assert.equal(wallet?.address, address, 'no signer for final input owner');
              return new L.SignatureEnabled(L.signData(wallet.key, message));
            });
            intent[property] = L.UnshieldedOffer.new(offer.inputs, offer.outputs, signatures);
          }
          merged.intents = merged.intents.set(id, intent);
        }
      }
      const complete = merged.bind();
      const actual = complete.fees(this.params);
      if (actual > fee) {
        fee = actual;
        continue;
      }
      this.lastFee = {
        paidSpeck: fee,
        requiredSpeck: actual,
        iterations,
        policy: 'native proved transaction cost; iterate until fully funded; no wallet margin'
      };
      this.funding.set(hash(complete.serialize()), {
        coins: [...proven.intents.values()]
                   .flatMap(
                       i =>
                           [...(i.guaranteedUnshieldedOffer?.inputs ?? []),
                            ...(i.fallibleUnshieldedOffer?.inputs ?? [])])
                   .map(coinId),
        dust: feeCoin.nonce,
        fee: this.lastFee
      });
      return complete;
    } while (true);
  }
  async call(instance, circuit, args, label, payer = this.actors.owner) {
    const prepared = await this.prepareCall(instance, circuit, args);
    const tx = await this.fundAndProve(this.assemble([prepared]), instance.contract, payer);
    return {tx, record: this.apply(tx, label), prepared};
  }
}
