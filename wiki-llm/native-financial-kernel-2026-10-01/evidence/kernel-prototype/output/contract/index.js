import * as __compactRuntime from '@midnight-ntwrk/compact-runtime';
__compactRuntime.checkRuntimeVersion('0.20.0');

const _descriptor_0 = new __compactRuntime.CompactTypeBytes(1278);

const _descriptor_1 = __compactRuntime.CompactTypeSecp256k1Scalar;

class _Secp256k1EcdsaSignature_0 {
  alignment() {
    return _descriptor_1.alignment().concat(_descriptor_1.alignment());
  }
  fromValue(value_0) {
    return {
      r: _descriptor_1.fromValue(value_0),
      s: _descriptor_1.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_1.toValue(value_0.r).concat(_descriptor_1.toValue(value_0.s));
  }
}

const _descriptor_2 = new _Secp256k1EcdsaSignature_0();

const _descriptor_3 = __compactRuntime.CompactTypeSecp256k1Point;

const _descriptor_4 = new __compactRuntime.CompactTypeBytes(32);

const _descriptor_5 = new __compactRuntime.CompactTypeUnsignedInteger(340282366920938463463374607431768211455n, 16);

const _descriptor_6 = __compactRuntime.CompactTypeBoolean;

class _UserAddress_0 {
  alignment() {
    return _descriptor_4.alignment();
  }
  fromValue(value_0) {
    return {
      bytes: _descriptor_4.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_4.toValue(value_0.bytes);
  }
}

const _descriptor_7 = new _UserAddress_0();

const _descriptor_8 = __compactRuntime.CompactTypeSecp256k1Base;

const _descriptor_9 = new __compactRuntime.CompactTypeVector(16, _descriptor_4);

class _Either_0 {
  alignment() {
    return _descriptor_6.alignment().concat(_descriptor_4.alignment().concat(_descriptor_4.alignment()));
  }
  fromValue(value_0) {
    return {
      is_left: _descriptor_6.fromValue(value_0),
      left: _descriptor_4.fromValue(value_0),
      right: _descriptor_4.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_6.toValue(value_0.is_left).concat(_descriptor_4.toValue(value_0.left).concat(_descriptor_4.toValue(value_0.right)));
  }
}

const _descriptor_10 = new _Either_0();

class _ContractAddress_0 {
  alignment() {
    return _descriptor_4.alignment();
  }
  fromValue(value_0) {
    return {
      bytes: _descriptor_4.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_4.toValue(value_0.bytes);
  }
}

const _descriptor_11 = new _ContractAddress_0();

class _Either_1 {
  alignment() {
    return _descriptor_6.alignment().concat(_descriptor_11.alignment().concat(_descriptor_7.alignment()));
  }
  fromValue(value_0) {
    return {
      is_left: _descriptor_6.fromValue(value_0),
      left: _descriptor_11.fromValue(value_0),
      right: _descriptor_7.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_6.toValue(value_0.is_left).concat(_descriptor_11.toValue(value_0.left).concat(_descriptor_7.toValue(value_0.right)));
  }
}

const _descriptor_12 = new _Either_1();

const _descriptor_13 = new __compactRuntime.CompactTypeUnsignedInteger(18446744073709551615n, 8);

class _Maybe_0 {
  alignment() {
    return _descriptor_6.alignment().concat(_descriptor_12.alignment());
  }
  fromValue(value_0) {
    return {
      is_some: _descriptor_6.fromValue(value_0),
      value: _descriptor_12.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_6.toValue(value_0.is_some).concat(_descriptor_12.toValue(value_0.value));
  }
}

const _descriptor_14 = new _Maybe_0();

const _descriptor_15 = new __compactRuntime.CompactTypeUnsignedInteger(255n, 1);

const _descriptor_16 = new __compactRuntime.CompactTypeUnsignedInteger(4294967295n, 4);

export class Contract {
  witnesses;
  constructor(...args_0) {
    if (args_0.length !== 1) {
      throw new __compactRuntime.CompactError(`Contract constructor: expected 1 argument, received ${args_0.length}`);
    }
    const witnesses_0 = args_0[0];
    if (typeof(witnesses_0) !== 'object') {
      throw new __compactRuntime.CompactError('first (witnesses) argument to Contract constructor is not an object');
    }
    this.witnesses = witnesses_0;
    this.circuits = {
      async signatureIsCanonical(context, ...args_1) {
        return { result: pureCircuits.signatureIsCanonical(...args_1), context };
      },
      async verifyOwner(context, ...args_1) {
        return { result: pureCircuits.verifyOwner(...args_1), context };
      },
      pay: async (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`pay: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const message_0 = args_1[1];
        const sig_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.callContext.currentQueryContext != undefined)) {
          __compactRuntime.typeError('pay',
                                     'argument 1 (as invoked from Typescript)',
                                     'fixed-transfer.compact line 59 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(message_0.buffer instanceof ArrayBuffer && message_0.BYTES_PER_ELEMENT === 1 && message_0.length === 1278)) {
          __compactRuntime.typeError('pay',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'fixed-transfer.compact line 59 char 1',
                                     'Bytes<1278>',
                                     message_0)
        }
        if (!(typeof(sig_0) === 'object' && typeof(sig_0.r) === 'bigint' && sig_0.r >= 0 && sig_0.r <= __compactRuntime.MAX_SECP256K1_SCALAR && typeof(sig_0.s) === 'bigint' && sig_0.s >= 0 && sig_0.s <= __compactRuntime.MAX_SECP256K1_SCALAR)) {
          __compactRuntime.typeError('pay',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'fixed-transfer.compact line 59 char 1',
                                     'struct Secp256k1EcdsaSignature<r: Secp256k1Scalar, s: Secp256k1Scalar>',
                                     sig_0)
        }
        const context = __compactRuntime.copyCircuitContext(contextOrig_0);
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(message_0).concat(_descriptor_2.toValue(sig_0)),
            alignment: _descriptor_0.alignment().concat(_descriptor_2.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = await this._pay_0(context,
                                           partialProofData,
                                           message_0,
                                           sig_0);
        partialProofData.output = { value: _descriptor_4.toValue(result_0), alignment: _descriptor_4.alignment() };
        __compactRuntime.finalizeCallProofData(context, partialProofData);
        return { result: result_0, context: context, gasCost: context.callContext.currentGasCost };
      }
    };
    this.impureCircuits = { pay: this.circuits.pay };
    this.provableCircuits = { pay: this.circuits.pay };
  }
  async initialState(...args_0) {
    if (args_0.length !== 9) {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 9 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const constructorContext_0 = args_0[0];
    const pk_0 = args_0[1];
    const color_0 = args_0[2];
    const recipient_0 = args_0[3];
    const fee_0 = args_0[4];
    const funding_0 = args_0[5];
    const allowance_0 = args_0[6];
    const work_0 = args_0[7];
    const round_0 = args_0[8];
    if (typeof(constructorContext_0) !== 'object') {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 'constructorContext' in argument 1 (as invoked from Typescript) to be an object`);
    }
    if (!('initialZswapLocalState' in constructorContext_0)) {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 'initialZswapLocalState' in argument 1 (as invoked from Typescript)`);
    }
    if (typeof(constructorContext_0.initialZswapLocalState) !== 'object') {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 'initialZswapLocalState' in argument 1 (as invoked from Typescript) to be an object`);
    }
    if (!(__compactRuntime.isValidSecp256k1Point(pk_0))) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 1 (argument 2 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'Secp256k1Point',
                                 pk_0)
    }
    if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 2 (argument 3 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'Bytes<32>',
                                 color_0)
    }
    if (!(typeof(recipient_0) === 'object' && recipient_0.bytes.buffer instanceof ArrayBuffer && recipient_0.bytes.BYTES_PER_ELEMENT === 1 && recipient_0.bytes.length === 32)) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 3 (argument 4 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'struct UserAddress<bytes: Bytes<32>>',
                                 recipient_0)
    }
    if (!(typeof(fee_0) === 'object' && fee_0.bytes.buffer instanceof ArrayBuffer && fee_0.bytes.BYTES_PER_ELEMENT === 1 && fee_0.bytes.length === 32)) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 4 (argument 5 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'struct UserAddress<bytes: Bytes<32>>',
                                 fee_0)
    }
    if (!(typeof(funding_0) === 'bigint' && funding_0 >= 0n && funding_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 5 (argument 6 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 funding_0)
    }
    if (!(typeof(allowance_0) === 'bigint' && allowance_0 >= 0n && allowance_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 6 (argument 7 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 allowance_0)
    }
    if (!(typeof(work_0) === 'bigint' && work_0 >= 0n && work_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 7 (argument 8 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 work_0)
    }
    if (!(typeof(round_0) === 'bigint' && round_0 >= 0n && round_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('Contract state constructor',
                                 'argument 8 (argument 9 as invoked from Typescript)',
                                 'fixed-transfer.compact line 46 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 round_0)
    }
    const state_0 = new __compactRuntime.ContractState();
    let stateValue_0 = __compactRuntime.StateValue.newArray();
    let stateValue_2 = __compactRuntime.StateValue.newArray();
    stateValue_2 = stateValue_2.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_2 = stateValue_2.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_2 = stateValue_2.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_2 = stateValue_2.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_2 = stateValue_2.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_2 = stateValue_2.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_0 = stateValue_0.arrayPush(stateValue_2);
    let stateValue_1 = __compactRuntime.StateValue.newArray();
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_1 = stateValue_1.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_0 = stateValue_0.arrayPush(stateValue_1);
    state_0.data = new __compactRuntime.ChargedState(stateValue_0);
    state_0.setOperation('pay', new __compactRuntime.ContractOperation());
    const context = __compactRuntime.createCircuitContext({circuitId: 'constructor', contractAddress: __compactRuntime.dummyContractAddress(), coinPublicKeyOrZswapState: constructorContext_0.initialZswapLocalState.coinPublicKey, contractState: state_0.data, privateState: constructorContext_0.initialPrivateState});
    const partialProofData = {
      input: { value: [], alignment: [] },
      output: undefined,
      publicTranscript: [],
      privateTranscriptOutputs: []
    };
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(0n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_3.toValue(({x: 0n, y: 0n, identity: true})),
                                                                                              alignment: _descriptor_3.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(1n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(new Uint8Array(32)),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(2n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(new Uint8Array(32)),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(3n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_6.toValue(false),
                                                                                              alignment: _descriptor_6.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(4n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(new Uint8Array(32)),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(5n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(0n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(1n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(2n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(3n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(4n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(5n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(6n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(7n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(8n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(new Uint8Array(32)),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(9n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_7.toValue({ bytes: new Uint8Array(32) }),
                                                                                              alignment: _descriptor_7.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(10n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_7.toValue({ bytes: new Uint8Array(32) }),
                                                                                              alignment: _descriptor_7.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(11n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(12n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(13n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(14n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    this._configuredKey_0(pk_0);
    __compactRuntime.assert(!this._equal_0(recipient_0.bytes, fee_0.bytes),
                            'ALIASED_DESTINATIONS');
    __compactRuntime.assert(this._equal_1(color_0,
                                          Uint8Array.from([161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n,
                                                           161n],
                                                          Number)),
                            'FIXTURE_ASSET_MAPPING');
    __compactRuntime.assert(this._equal_2(recipient_0.bytes,
                                          Uint8Array.from([2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n,
                                                           2n],
                                                          Number))
                            &&
                            this._equal_3(fee_0.bytes,
                                          Uint8Array.from([3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n,
                                                           3n],
                                                          Number)),
                            'FIXTURE_DESTINATION_MAPPING');
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(0n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_3.toValue(pk_0),
                                                                                              alignment: _descriptor_3.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(8n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(color_0),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(9n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_7.toValue(recipient_0),
                                                                                              alignment: _descriptor_7.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(10n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_7.toValue(fee_0),
                                                                                              alignment: _descriptor_7.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_0 = Uint8Array.from([251n,
                                   80n,
                                   123n,
                                   251n,
                                   132n,
                                   138n,
                                   136n,
                                   89n,
                                   165n,
                                   92n,
                                   2n,
                                   15n,
                                   175n,
                                   65n,
                                   14n,
                                   82n,
                                   249n,
                                   228n,
                                   12n,
                                   0n,
                                   252n,
                                   57n,
                                   56n,
                                   169n,
                                   14n,
                                   163n,
                                   199n,
                                   99n,
                                   225n,
                                   207n,
                                   29n,
                                   108n],
                                  Number);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(1n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(tmp_0),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_1 = Uint8Array.from([250n,
                                   188n,
                                   153n,
                                   200n,
                                   101n,
                                   8n,
                                   20n,
                                   164n,
                                   131n,
                                   234n,
                                   106n,
                                   245n,
                                   23n,
                                   43n,
                                   89n,
                                   229n,
                                   170n,
                                   210n,
                                   199n,
                                   111n,
                                   198n,
                                   77n,
                                   140n,
                                   38n,
                                   19n,
                                   155n,
                                   131n,
                                   70n,
                                   150n,
                                   204n,
                                   128n,
                                   53n],
                                  Number);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(2n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(tmp_1),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(3n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_6.toValue(false),
                                                                                              alignment: _descriptor_6.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_2 = Uint8Array.from([158n,
                                   70n,
                                   1n,
                                   201n,
                                   175n,
                                   148n,
                                   16n,
                                   34n,
                                   8n,
                                   189n,
                                   107n,
                                   222n,
                                   166n,
                                   105n,
                                   60n,
                                   191n,
                                   90n,
                                   150n,
                                   56n,
                                   246n,
                                   214n,
                                   185n,
                                   250n,
                                   113n,
                                   110n,
                                   174n,
                                   84n,
                                   4n,
                                   67n,
                                   204n,
                                   48n,
                                   228n],
                                  Number);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(4n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(tmp_2),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_3 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(5n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_3),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(0n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(round_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(1n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(funding_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_4 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(2n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_4),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_5 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(3n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_5),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(4n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(work_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_6 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(5n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_6),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(6n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(allowance_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_7 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(7n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_7),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_8 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(11n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_8),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_9 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(12n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_9),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_10 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(13n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_10),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_11 = 0n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(14n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_11),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    state_0.data = new __compactRuntime.ChargedState(context.callContext.currentQueryContext.state.state);
    return {
      currentContractState: state_0,
      currentPrivateState: context.callContext.currentPrivateState,
      currentZswapLocalState: context.callContext.currentZswapLocalState
    }
  }
  _left_0(value_0) {
    return { is_left: true, left: value_0, right: new Uint8Array(32) };
  }
  _right_0(value_0) {
    return { is_left: false, left: { bytes: new Uint8Array(32) }, right: value_0 };
  }
  async _sendUnshielded_0(context,
                          partialProofData,
                          color_0,
                          amount_0,
                          recipient_0)
  {
    const tmp_0 = this._left_0(color_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(7n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_10.toValue(tmp_0),
                                                                                              alignment: _descriptor_10.alignment() }).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(amount_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { swap: { n: 0 } },
                                       'neg',
                                       { branch: { skip: 4 } },
                                       { dup: { n: 2 } },
                                       { dup: { n: 2 } },
                                       { idx: { cached: true,
                                                pushPath: false,
                                                path: [ { tag: 'stack' }] } },
                                       'add',
                                       { ins: { cached: true, n: 2 } },
                                       { swap: { n: 0 } }]);
    const tmp_1 = this._left_0(color_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(8n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell(__compactRuntime.alignedConcat(
                                                                                              { value: _descriptor_10.toValue(tmp_1),
                                                                                                alignment: _descriptor_10.alignment() },
                                                                                              { value: _descriptor_12.toValue(recipient_0),
                                                                                                alignment: _descriptor_12.alignment() }
                                                                                            )).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(amount_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { swap: { n: 0 } },
                                       'neg',
                                       { branch: { skip: 4 } },
                                       { dup: { n: 2 } },
                                       { dup: { n: 2 } },
                                       { idx: { cached: true,
                                                pushPath: false,
                                                path: [ { tag: 'stack' }] } },
                                       'add',
                                       { ins: { cached: true, n: 2 } },
                                       { swap: { n: 0 } }]);
    if (recipient_0.is_left
        &&
        this._equal_4(recipient_0.left.bytes,
                      _descriptor_11.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                 partialProofData,
                                                                                 [
                                                                                  { dup: { n: 2 } },
                                                                                  { idx: { cached: true,
                                                                                           pushPath: false,
                                                                                           path: [
                                                                                                  { tag: 'value',
                                                                                                    value: { value: _descriptor_15.toValue(0n),
                                                                                                             alignment: _descriptor_15.alignment() } }] } },
                                                                                  { popeq: { cached: true,
                                                                                             result: undefined } }]).value).bytes))
    {
      const tmp_2 = this._left_0(color_0);
      __compactRuntime.queryLedgerState(context,
                                        partialProofData,
                                        [
                                         { swap: { n: 0 } },
                                         { idx: { cached: true,
                                                  pushPath: true,
                                                  path: [
                                                         { tag: 'value',
                                                           value: { value: _descriptor_15.toValue(6n),
                                                                    alignment: _descriptor_15.alignment() } }] } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_10.toValue(tmp_2),
                                                                                                alignment: _descriptor_10.alignment() }).encode() } },
                                         { dup: { n: 1 } },
                                         { dup: { n: 1 } },
                                         'member',
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(amount_0),
                                                                                                alignment: _descriptor_5.alignment() }).encode() } },
                                         { swap: { n: 0 } },
                                         'neg',
                                         { branch: { skip: 4 } },
                                         { dup: { n: 2 } },
                                         { dup: { n: 2 } },
                                         { idx: { cached: true,
                                                  pushPath: false,
                                                  path: [ { tag: 'stack' }] } },
                                         'add',
                                         { ins: { cached: true, n: 2 } },
                                         { swap: { n: 0 } }]);
    }
    return [];
  }
  async _unshieldedBalance_0(context, partialProofData, color_0) {
    const tmp_0 = this._left_0(color_0);
    return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                     partialProofData,
                                                                     [
                                                                      { dup: { n: 2 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_15.toValue(5n),
                                                                                                 alignment: _descriptor_15.alignment() } }] } },
                                                                      { dup: { n: 0 } },
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_10.toValue(tmp_0),
                                                                                                                             alignment: _descriptor_10.alignment() }).encode() } },
                                                                      'member',
                                                                      { branch: { skip: 3 } },
                                                                      'pop',
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                                                             alignment: _descriptor_5.alignment() }).encode() } },
                                                                      { jmp: { skip: 1 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_10.toValue(tmp_0),
                                                                                                 alignment: _descriptor_10.alignment() } }] } },
                                                                      { popeq: { cached: true,
                                                                                 result: undefined } }]).value);
  }
  async _unshieldedBalanceLt_0(context, partialProofData, color_0, amount_0) {
    const tmp_0 = this._left_0(color_0);
    return _descriptor_6.fromValue(__compactRuntime.queryLedgerState(context,
                                                                     partialProofData,
                                                                     [
                                                                      { dup: { n: 2 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_15.toValue(5n),
                                                                                                 alignment: _descriptor_15.alignment() } }] } },
                                                                      { dup: { n: 0 } },
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_10.toValue(tmp_0),
                                                                                                                             alignment: _descriptor_10.alignment() }).encode() } },
                                                                      'member',
                                                                      { branch: { skip: 3 } },
                                                                      'pop',
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(0n),
                                                                                                                             alignment: _descriptor_5.alignment() }).encode() } },
                                                                      { jmp: { skip: 1 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_10.toValue(tmp_0),
                                                                                                 alignment: _descriptor_10.alignment() } }] } },
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(amount_0),
                                                                                                                             alignment: _descriptor_5.alignment() }).encode() } },
                                                                      'lt',
                                                                      { popeq: { cached: true,
                                                                                 result: undefined } }]).value);
  }
  async _unshieldedBalanceGte_0(context, partialProofData, color_0, amount_0) {
    return !await this._unshieldedBalanceLt_0(context,
                                              partialProofData,
                                              color_0,
                                              amount_0);
  }
  _reverseBytes32_0(b_0) {
    const v_0 = Array.from(b_0, BigInt);
    return Uint8Array.from([v_0[31],
                            v_0[30],
                            v_0[29],
                            v_0[28],
                            v_0[27],
                            v_0[26],
                            v_0[25],
                            v_0[24],
                            v_0[23],
                            v_0[22],
                            v_0[21],
                            v_0[20],
                            v_0[19],
                            v_0[18],
                            v_0[17],
                            v_0[16],
                            v_0[15],
                            v_0[14],
                            v_0[13],
                            v_0[12],
                            v_0[11],
                            v_0[10],
                            v_0[9],
                            v_0[8],
                            v_0[7],
                            v_0[6],
                            v_0[5],
                            v_0[4],
                            v_0[3],
                            v_0[2],
                            v_0[1],
                            v_0[0]],
                           Number);
  }
  _secp256k1EcdsaVerify_0(msgHash_0, sig_0, pk_0) {
    __compactRuntime.assert(!this._equal_5(pk_0,
                                           ({x: 0n, y: 0n, identity: true})),
                            'Secp256k1Point identity is not a permitted secp256k1EcdsaVerify verification key');
    const z_0 = __compactRuntime.convertBytesToField(115792089237316195423570985008687907852837564279074904382605163141518161494336n,
                                                     32,
                                                     this._reverseBytes32_0(msgHash_0),
                                                     'Secp256k1Scalar',
                                                     '<standard library>');
    const __compact_pattern_tmp3_0 = sig_0;
    const r_0 = __compact_pattern_tmp3_0.r;
    const s_0 = __compact_pattern_tmp3_0.s;
    const w_0 = this._inv_0(s_0);
    const u1_0 = __compactRuntime.secp256k1ScalarMul(z_0, w_0);
    const u2_0 = __compactRuntime.secp256k1ScalarMul(r_0, w_0);
    const point_0 = this._ecAdd_0(this._ecMulGenerator_0(u1_0),
                                  this._ecMul_0(pk_0, u2_0));
    return __compactRuntime.convertBytesToField(115792089237316195423570985008687907852837564279074904382605163141518161494336n,
                                                32,
                                                __compactRuntime.convertBigintToBytes(32,
                                                                                      this._secp256k1PointX_0(point_0),
                                                                                      '<standard library>'),
                                                'Secp256k1Scalar',
                                                '<standard library>')
           ===
           r_0;
  }
  _persistentHash_0(value_0) {
    const result_0 = __compactRuntime.persistentHash(_descriptor_0, value_0);
    return result_0;
  }
  _persistentHash_1(value_0) {
    const result_0 = __compactRuntime.persistentHash(_descriptor_9, value_0);
    return result_0;
  }
  _inv_0(s_0) {
    const result_0 = __compactRuntime.secp256k1ScalarInv(s_0);
    return result_0;
  }
  _secp256k1PointX_0(pt_0) {
    const result_0 = __compactRuntime.secp256k1PointX(pt_0);
    return result_0;
  }
  _secp256k1PointY_0(pt_0) {
    const result_0 = __compactRuntime.secp256k1PointY(pt_0);
    return result_0;
  }
  _ecAdd_0(a_0, b_0) {
    const result_0 = __compactRuntime.secp256k1Add(a_0, b_0);
    return result_0;
  }
  _ecMul_0(a_0, b_0) {
    const result_0 = __compactRuntime.secp256k1Mul(a_0, b_0);
    return result_0;
  }
  _ecMulGenerator_0(b_0) {
    const result_0 = __compactRuntime.secp256k1MulGenerator(b_0);
    return result_0;
  }
  _configuredKey_0(pk_0) {
    __compactRuntime.assert(!this._equal_6(pk_0,
                                           ({x: 0n, y: 0n, identity: true})),
                            'IDENTITY_KEY');
    __compactRuntime.assert(this._equal_7(__compactRuntime.convertBigintToBytes(32,
                                                                                this._secp256k1PointX_0(pk_0),
                                                                                'fixed-transfer.compact line 6 char 10'),
                                          Uint8Array.from([171n,
                                                           40n,
                                                           63n,
                                                           143n,
                                                           116n,
                                                           250n,
                                                           8n,
                                                           170n,
                                                           157n,
                                                           219n,
                                                           54n,
                                                           181n,
                                                           48n,
                                                           124n,
                                                           233n,
                                                           40n,
                                                           217n,
                                                           232n,
                                                           241n,
                                                           246n,
                                                           13n,
                                                           177n,
                                                           180n,
                                                           52n,
                                                           88n,
                                                           32n,
                                                           182n,
                                                           227n,
                                                           42n,
                                                           200n,
                                                           115n,
                                                           108n],
                                                          Number)),
                            'SOURCE_SEC1_X');
    __compactRuntime.assert(this._equal_8(__compactRuntime.convertBigintToBytes(32,
                                                                                this._secp256k1PointY_0(pk_0),
                                                                                'fixed-transfer.compact line 7 char 10'),
                                          Uint8Array.from([92n,
                                                           113n,
                                                           182n,
                                                           219n,
                                                           243n,
                                                           70n,
                                                           189n,
                                                           130n,
                                                           102n,
                                                           27n,
                                                           76n,
                                                           68n,
                                                           72n,
                                                           95n,
                                                           89n,
                                                           11n,
                                                           125n,
                                                           9n,
                                                           142n,
                                                           156n,
                                                           5n,
                                                           197n,
                                                           21n,
                                                           148n,
                                                           15n,
                                                           110n,
                                                           55n,
                                                           176n,
                                                           147n,
                                                           105n,
                                                           52n,
                                                           61n],
                                                          Number)),
                            'SOURCE_SEC1_Y_PARITY');
    return [];
  }
  _canonicalSignature_0(sig_0) {
    const sb_0 = __compactRuntime.convertBigintToBytes(32,
                                                       sig_0.s,
                                                       'fixed-transfer.compact line 10 char 24');
    const hi_0 = __compactRuntime.convertBytesToUint(340282366920938463463374607431768211455n,
                                                     16,
                                                     ((e, i) => e.slice(i, i+16))(sb_0,
                                                                                  Number(16n)),
                                                     'Uint<0..340282366920938463463374607431768211456>',
                                                     'fixed-transfer.compact line 11 char 24');
    const lo_0 = __compactRuntime.convertBytesToUint(340282366920938463463374607431768211455n,
                                                     16,
                                                     ((e, i) => e.slice(i, i+16))(sb_0,
                                                                                  Number(0n)),
                                                     'Uint<0..340282366920938463463374607431768211456>',
                                                     'fixed-transfer.compact line 12 char 24');
    __compactRuntime.assert(hi_0 < 170141183460469231731687303715884105727n
                            ||
                            hi_0 === 170141183460469231731687303715884105727n
                            &&
                            lo_0 <= 124072173638108635037164174234284138656n,
                            'HIGH_S');
    __compactRuntime.assert(sig_0.r !== 0n && sig_0.s !== 0n, 'ZERO_SIGNATURE');
    return [];
  }
  _signatureIsCanonical_0(sig_0) {
    this._canonicalSignature_0(sig_0); return [];
  }
  _verifyOwner_0(message_0, sig_0, pk_0) {
    this._configuredKey_0(pk_0);
    this._canonicalSignature_0(sig_0);
    __compactRuntime.assert(this._equal_9(message_0,
                                          Uint8Array.from([109n,
                                                           105n,
                                                           100n,
                                                           110n,
                                                           105n,
                                                           103n,
                                                           104n,
                                                           116n,
                                                           95n,
                                                           115n,
                                                           105n,
                                                           103n,
                                                           110n,
                                                           101n,
                                                           100n,
                                                           95n,
                                                           109n,
                                                           101n,
                                                           115n,
                                                           115n,
                                                           97n,
                                                           103n,
                                                           101n,
                                                           58n,
                                                           49n,
                                                           50n,
                                                           52n,
                                                           57n,
                                                           58n,
                                                           109n,
                                                           111n,
                                                           114n,
                                                           105n,
                                                           97n,
                                                           114n,
                                                           116n,
                                                           121n,
                                                           45n,
                                                           115n,
                                                           105n,
                                                           103n,
                                                           110n,
                                                           101n,
                                                           100n,
                                                           45n,
                                                           105n,
                                                           110n,
                                                           116n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           47n,
                                                           49n,
                                                           0n,
                                                           0n,
                                                           0n,
                                                           4n,
                                                           196n,
                                                           123n,
                                                           34n,
                                                           97n,
                                                           99n,
                                                           116n,
                                                           105n,
                                                           111n,
                                                           110n,
                                                           78n,
                                                           97n,
                                                           109n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           112n,
                                                           97n,
                                                           121n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           97n,
                                                           103n,
                                                           114n,
                                                           101n,
                                                           101n,
                                                           109n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           73n,
                                                           100n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           73n,
                                                           110n,
                                                           118n,
                                                           111n,
                                                           105n,
                                                           99n,
                                                           101n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           97n,
                                                           115n,
                                                           115n,
                                                           101n,
                                                           116n,
                                                           34n,
                                                           58n,
                                                           123n,
                                                           34n,
                                                           105n,
                                                           100n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           65n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           114n,
                                                           101n,
                                                           112n,
                                                           114n,
                                                           101n,
                                                           115n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           97n,
                                                           116n,
                                                           105n,
                                                           111n,
                                                           110n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           99n,
                                                           97n,
                                                           110n,
                                                           111n,
                                                           110n,
                                                           105n,
                                                           99n,
                                                           97n,
                                                           108n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           99n,
                                                           97n,
                                                           108n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           50n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           121n,
                                                           109n,
                                                           98n,
                                                           111n,
                                                           108n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           85n,
                                                           83n,
                                                           68n,
                                                           34n,
                                                           125n,
                                                           44n,
                                                           34n,
                                                           97n,
                                                           117n,
                                                           116n,
                                                           104n,
                                                           111n,
                                                           114n,
                                                           105n,
                                                           110n,
                                                           103n,
                                                           80n,
                                                           114n,
                                                           111n,
                                                           102n,
                                                           105n,
                                                           108n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           109n,
                                                           111n,
                                                           114n,
                                                           105n,
                                                           97n,
                                                           114n,
                                                           116n,
                                                           121n,
                                                           45n,
                                                           98n,
                                                           101n,
                                                           116n,
                                                           97n,
                                                           47n,
                                                           49n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           99n,
                                                           111n,
                                                           114n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           109n,
                                                           111n,
                                                           114n,
                                                           105n,
                                                           97n,
                                                           114n,
                                                           116n,
                                                           121n,
                                                           45n,
                                                           99n,
                                                           111n,
                                                           114n,
                                                           101n,
                                                           47n,
                                                           53n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           100n,
                                                           111n,
                                                           109n,
                                                           97n,
                                                           105n,
                                                           110n,
                                                           34n,
                                                           58n,
                                                           123n,
                                                           34n,
                                                           99n,
                                                           104n,
                                                           97n,
                                                           105n,
                                                           110n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           109n,
                                                           105n,
                                                           100n,
                                                           110n,
                                                           105n,
                                                           103n,
                                                           104n,
                                                           116n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           105n,
                                                           100n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           77n,
                                                           105n,
                                                           100n,
                                                           110n,
                                                           105n,
                                                           103n,
                                                           104n,
                                                           116n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           110n,
                                                           101n,
                                                           116n,
                                                           119n,
                                                           111n,
                                                           114n,
                                                           107n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           112n,
                                                           114n,
                                                           101n,
                                                           118n,
                                                           105n,
                                                           101n,
                                                           119n,
                                                           34n,
                                                           125n,
                                                           44n,
                                                           34n,
                                                           105n,
                                                           110n,
                                                           116n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           34n,
                                                           58n,
                                                           123n,
                                                           34n,
                                                           97n,
                                                           100n,
                                                           118n,
                                                           101n,
                                                           114n,
                                                           116n,
                                                           105n,
                                                           115n,
                                                           101n,
                                                           100n,
                                                           80n,
                                                           111n,
                                                           108n,
                                                           105n,
                                                           99n,
                                                           121n,
                                                           68n,
                                                           105n,
                                                           103n,
                                                           101n,
                                                           115n,
                                                           116n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           112n,
                                                           111n,
                                                           108n,
                                                           105n,
                                                           99n,
                                                           121n,
                                                           49n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           97n,
                                                           100n,
                                                           118n,
                                                           101n,
                                                           114n,
                                                           116n,
                                                           105n,
                                                           115n,
                                                           101n,
                                                           100n,
                                                           83n,
                                                           111n,
                                                           117n,
                                                           114n,
                                                           99n,
                                                           101n,
                                                           72n,
                                                           97n,
                                                           115n,
                                                           104n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           115n,
                                                           114n,
                                                           99n,
                                                           49n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           100n,
                                                           101n,
                                                           108n,
                                                           101n,
                                                           103n,
                                                           97n,
                                                           116n,
                                                           105n,
                                                           111n,
                                                           110n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           110n,
                                                           111n,
                                                           110n,
                                                           101n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           100n,
                                                           105n,
                                                           115n,
                                                           99n,
                                                           108n,
                                                           111n,
                                                           115n,
                                                           117n,
                                                           114n,
                                                           101n,
                                                           115n,
                                                           34n,
                                                           58n,
                                                           91n,
                                                           93n,
                                                           44n,
                                                           34n,
                                                           102n,
                                                           97n,
                                                           105n,
                                                           108n,
                                                           117n,
                                                           114n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           115n,
                                                           117n,
                                                           99n,
                                                           99n,
                                                           101n,
                                                           115n,
                                                           115n,
                                                           95n,
                                                           111n,
                                                           110n,
                                                           108n,
                                                           121n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           102n,
                                                           101n,
                                                           101n,
                                                           67n,
                                                           97n,
                                                           112n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           49n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           103n,
                                                           114n,
                                                           111n,
                                                           115n,
                                                           115n,
                                                           67n,
                                                           97n,
                                                           112n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           49n,
                                                           48n,
                                                           49n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           110n,
                                                           101n,
                                                           116n,
                                                           70n,
                                                           108n,
                                                           111n,
                                                           111n,
                                                           114n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           49n,
                                                           48n,
                                                           48n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           110n,
                                                           111n,
                                                           110n,
                                                           99n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           110n,
                                                           49n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           110n,
                                                           111n,
                                                           116n,
                                                           65n,
                                                           102n,
                                                           116n,
                                                           101n,
                                                           114n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           49n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           110n,
                                                           111n,
                                                           116n,
                                                           66n,
                                                           101n,
                                                           102n,
                                                           111n,
                                                           114n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           111n,
                                                           98n,
                                                           115n,
                                                           101n,
                                                           114n,
                                                           118n,
                                                           97n,
                                                           116n,
                                                           105n,
                                                           111n,
                                                           110n,
                                                           115n,
                                                           34n,
                                                           58n,
                                                           91n,
                                                           93n,
                                                           44n,
                                                           34n,
                                                           111n,
                                                           112n,
                                                           101n,
                                                           114n,
                                                           97n,
                                                           116n,
                                                           105n,
                                                           111n,
                                                           110n,
                                                           34n,
                                                           58n,
                                                           123n,
                                                           34n,
                                                           97n,
                                                           109n,
                                                           111n,
                                                           117n,
                                                           110n,
                                                           116n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           49n,
                                                           48n,
                                                           48n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           102n,
                                                           101n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           49n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           102n,
                                                           101n,
                                                           101n,
                                                           82n,
                                                           101n,
                                                           99n,
                                                           105n,
                                                           112n,
                                                           105n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           70n,
                                                           101n,
                                                           101n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           102n,
                                                           114n,
                                                           111n,
                                                           109n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           79n,
                                                           119n,
                                                           110n,
                                                           101n,
                                                           114n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           107n,
                                                           105n,
                                                           110n,
                                                           100n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           84n,
                                                           114n,
                                                           97n,
                                                           110n,
                                                           115n,
                                                           102n,
                                                           101n,
                                                           114n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           114n,
                                                           101n,
                                                           99n,
                                                           105n,
                                                           112n,
                                                           105n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           82n,
                                                           101n,
                                                           99n,
                                                           105n,
                                                           112n,
                                                           105n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           34n,
                                                           125n,
                                                           44n,
                                                           34n,
                                                           112n,
                                                           114n,
                                                           101n,
                                                           72n,
                                                           101n,
                                                           97n,
                                                           100n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           104n,
                                                           48n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           114n,
                                                           101n,
                                                           99n,
                                                           111n,
                                                           118n,
                                                           101n,
                                                           114n,
                                                           121n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           110n,
                                                           111n,
                                                           110n,
                                                           101n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           114n,
                                                           101n,
                                                           116n,
                                                           97n,
                                                           105n,
                                                           110n,
                                                           101n,
                                                           100n,
                                                           68n,
                                                           117n,
                                                           116n,
                                                           105n,
                                                           101n,
                                                           115n,
                                                           34n,
                                                           58n,
                                                           91n,
                                                           93n,
                                                           44n,
                                                           34n,
                                                           114n,
                                                           101n,
                                                           116n,
                                                           97n,
                                                           105n,
                                                           110n,
                                                           101n,
                                                           100n,
                                                           69n,
                                                           102n,
                                                           102n,
                                                           101n,
                                                           99n,
                                                           116n,
                                                           115n,
                                                           34n,
                                                           58n,
                                                           91n,
                                                           93n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           105n,
                                                           103n,
                                                           110n,
                                                           101n,
                                                           114n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           79n,
                                                           119n,
                                                           110n,
                                                           101n,
                                                           114n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           118n,
                                                           101n,
                                                           114n,
                                                           115n,
                                                           105n,
                                                           111n,
                                                           110n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           109n,
                                                           111n,
                                                           114n,
                                                           105n,
                                                           97n,
                                                           114n,
                                                           116n,
                                                           121n,
                                                           45n,
                                                           105n,
                                                           110n,
                                                           116n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           47n,
                                                           51n,
                                                           34n,
                                                           125n,
                                                           44n,
                                                           34n,
                                                           111n,
                                                           119n,
                                                           110n,
                                                           101n,
                                                           114n,
                                                           80n,
                                                           114n,
                                                           111n,
                                                           103n,
                                                           114n,
                                                           97n,
                                                           109n,
                                                           83n,
                                                           104n,
                                                           97n,
                                                           50n,
                                                           53n,
                                                           54n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           102n,
                                                           97n,
                                                           98n,
                                                           99n,
                                                           57n,
                                                           57n,
                                                           99n,
                                                           56n,
                                                           54n,
                                                           53n,
                                                           48n,
                                                           56n,
                                                           49n,
                                                           52n,
                                                           97n,
                                                           52n,
                                                           56n,
                                                           51n,
                                                           101n,
                                                           97n,
                                                           54n,
                                                           97n,
                                                           102n,
                                                           53n,
                                                           49n,
                                                           55n,
                                                           50n,
                                                           98n,
                                                           53n,
                                                           57n,
                                                           101n,
                                                           53n,
                                                           97n,
                                                           97n,
                                                           100n,
                                                           50n,
                                                           99n,
                                                           55n,
                                                           54n,
                                                           102n,
                                                           99n,
                                                           54n,
                                                           52n,
                                                           100n,
                                                           56n,
                                                           99n,
                                                           50n,
                                                           54n,
                                                           49n,
                                                           51n,
                                                           57n,
                                                           98n,
                                                           56n,
                                                           51n,
                                                           52n,
                                                           54n,
                                                           57n,
                                                           54n,
                                                           99n,
                                                           99n,
                                                           56n,
                                                           48n,
                                                           51n,
                                                           53n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           112n,
                                                           114n,
                                                           111n,
                                                           102n,
                                                           105n,
                                                           108n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           109n,
                                                           111n,
                                                           114n,
                                                           105n,
                                                           97n,
                                                           114n,
                                                           116n,
                                                           121n,
                                                           45n,
                                                           115n,
                                                           105n,
                                                           103n,
                                                           110n,
                                                           101n,
                                                           100n,
                                                           45n,
                                                           105n,
                                                           110n,
                                                           116n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           47n,
                                                           49n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           101n,
                                                           108n,
                                                           101n,
                                                           99n,
                                                           116n,
                                                           101n,
                                                           100n,
                                                           65n,
                                                           99n,
                                                           116n,
                                                           105n,
                                                           111n,
                                                           110n,
                                                           73n,
                                                           100n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           84n,
                                                           114n,
                                                           97n,
                                                           110n,
                                                           115n,
                                                           102n,
                                                           101n,
                                                           114n,
                                                           76n,
                                                           105n,
                                                           116n,
                                                           101n,
                                                           114n,
                                                           97n,
                                                           108n,
                                                           70n,
                                                           101n,
                                                           101n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           105n,
                                                           103n,
                                                           110n,
                                                           97n,
                                                           116n,
                                                           117n,
                                                           114n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           123n,
                                                           34n,
                                                           102n,
                                                           114n,
                                                           97n,
                                                           109n,
                                                           105n,
                                                           110n,
                                                           103n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           109n,
                                                           105n,
                                                           100n,
                                                           110n,
                                                           105n,
                                                           103n,
                                                           104n,
                                                           116n,
                                                           45n,
                                                           115n,
                                                           105n,
                                                           103n,
                                                           110n,
                                                           45n,
                                                           100n,
                                                           97n,
                                                           116n,
                                                           97n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           107n,
                                                           101n,
                                                           121n,
                                                           82n,
                                                           101n,
                                                           102n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           107n,
                                                           101n,
                                                           121n,
                                                           49n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           112n,
                                                           117n,
                                                           98n,
                                                           108n,
                                                           105n,
                                                           99n,
                                                           75n,
                                                           101n,
                                                           121n,
                                                           72n,
                                                           101n,
                                                           120n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           48n,
                                                           50n,
                                                           54n,
                                                           99n,
                                                           55n,
                                                           51n,
                                                           99n,
                                                           56n,
                                                           50n,
                                                           97n,
                                                           101n,
                                                           51n,
                                                           98n,
                                                           54n,
                                                           50n,
                                                           48n,
                                                           53n,
                                                           56n,
                                                           51n,
                                                           52n,
                                                           98n,
                                                           52n,
                                                           98n,
                                                           49n,
                                                           48n,
                                                           100n,
                                                           102n,
                                                           54n,
                                                           102n,
                                                           49n,
                                                           101n,
                                                           56n,
                                                           100n,
                                                           57n,
                                                           50n,
                                                           56n,
                                                           101n,
                                                           57n,
                                                           55n,
                                                           99n,
                                                           51n,
                                                           48n,
                                                           98n,
                                                           53n,
                                                           51n,
                                                           54n,
                                                           100n,
                                                           98n,
                                                           57n,
                                                           100n,
                                                           97n,
                                                           97n,
                                                           48n,
                                                           56n,
                                                           102n,
                                                           97n,
                                                           55n,
                                                           52n,
                                                           56n,
                                                           102n,
                                                           51n,
                                                           102n,
                                                           50n,
                                                           56n,
                                                           97n,
                                                           98n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           99n,
                                                           104n,
                                                           101n,
                                                           109n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           101n,
                                                           99n,
                                                           100n,
                                                           115n,
                                                           97n,
                                                           95n,
                                                           115n,
                                                           101n,
                                                           99n,
                                                           112n,
                                                           50n,
                                                           53n,
                                                           54n,
                                                           107n,
                                                           49n,
                                                           95n,
                                                           115n,
                                                           104n,
                                                           97n,
                                                           50n,
                                                           53n,
                                                           54n,
                                                           34n,
                                                           125n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           111n,
                                                           117n,
                                                           114n,
                                                           99n,
                                                           101n,
                                                           80n,
                                                           114n,
                                                           111n,
                                                           102n,
                                                           105n,
                                                           108n,
                                                           101n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           109n,
                                                           111n,
                                                           114n,
                                                           105n,
                                                           97n,
                                                           114n,
                                                           116n,
                                                           121n,
                                                           45n,
                                                           102n,
                                                           105n,
                                                           110n,
                                                           97n,
                                                           110n,
                                                           99n,
                                                           105n,
                                                           97n,
                                                           108n,
                                                           45n,
                                                           97n,
                                                           103n,
                                                           114n,
                                                           101n,
                                                           101n,
                                                           109n,
                                                           101n,
                                                           110n,
                                                           116n,
                                                           45n,
                                                           115n,
                                                           111n,
                                                           117n,
                                                           114n,
                                                           99n,
                                                           101n,
                                                           47n,
                                                           54n,
                                                           34n,
                                                           44n,
                                                           34n,
                                                           115n,
                                                           111n,
                                                           117n,
                                                           114n,
                                                           99n,
                                                           101n,
                                                           83n,
                                                           104n,
                                                           97n,
                                                           50n,
                                                           53n,
                                                           54n,
                                                           34n,
                                                           58n,
                                                           34n,
                                                           102n,
                                                           98n,
                                                           53n,
                                                           48n,
                                                           55n,
                                                           98n,
                                                           102n,
                                                           98n,
                                                           56n,
                                                           52n,
                                                           56n,
                                                           97n,
                                                           56n,
                                                           56n,
                                                           53n,
                                                           57n,
                                                           97n,
                                                           53n,
                                                           53n,
                                                           99n,
                                                           48n,
                                                           50n,
                                                           48n,
                                                           102n,
                                                           97n,
                                                           102n,
                                                           52n,
                                                           49n,
                                                           48n,
                                                           101n,
                                                           53n,
                                                           50n,
                                                           102n,
                                                           57n,
                                                           101n,
                                                           52n,
                                                           48n,
                                                           99n,
                                                           48n,
                                                           48n,
                                                           102n,
                                                           99n,
                                                           51n,
                                                           57n,
                                                           51n,
                                                           56n,
                                                           97n,
                                                           57n,
                                                           48n,
                                                           101n,
                                                           97n,
                                                           51n,
                                                           99n,
                                                           55n,
                                                           54n,
                                                           51n,
                                                           101n,
                                                           49n,
                                                           99n,
                                                           102n,
                                                           49n,
                                                           100n,
                                                           54n,
                                                           99n,
                                                           34n,
                                                           125n],
                                                          Number)),
                            'FRAME_SOURCE_MISMATCH');
    const digest_0 = this._persistentHash_0(message_0);
    __compactRuntime.assert(this._equal_10(digest_0,
                                           Uint8Array.from([229n,
                                                            158n,
                                                            106n,
                                                            77n,
                                                            29n,
                                                            72n,
                                                            3n,
                                                            135n,
                                                            194n,
                                                            6n,
                                                            8n,
                                                            134n,
                                                            99n,
                                                            246n,
                                                            97n,
                                                            29n,
                                                            41n,
                                                            223n,
                                                            249n,
                                                            141n,
                                                            26n,
                                                            201n,
                                                            157n,
                                                            173n,
                                                            254n,
                                                            44n,
                                                            88n,
                                                            43n,
                                                            39n,
                                                            204n,
                                                            26n,
                                                            72n],
                                                           Number)),
                            'FRAME_SHA256_MISMATCH');
    __compactRuntime.assert(this._secp256k1EcdsaVerify_0(digest_0, sig_0, pk_0),
                            'BAD_SIGNATURE');
    return digest_0;
  }
  async _pay_0(context, partialProofData, message_0, sig_0) {
    const signedDigest_0 = this._verifyOwner_0(message_0,
                                               sig_0,
                                               _descriptor_3.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                         partialProofData,
                                                                                                         [
                                                                                                          { dup: { n: 0 } },
                                                                                                          { idx: { cached: false,
                                                                                                                   pushPath: false,
                                                                                                                   path: [
                                                                                                                          { tag: 'value',
                                                                                                                            value: { value: _descriptor_15.toValue(0n),
                                                                                                                                     alignment: _descriptor_15.alignment() } },
                                                                                                                          { tag: 'value',
                                                                                                                            value: { value: _descriptor_15.toValue(0n),
                                                                                                                                     alignment: _descriptor_15.alignment() } }] } },
                                                                                                          { popeq: { cached: false,
                                                                                                                     result: undefined } }]).value));
    __compactRuntime.assert(!_descriptor_6.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                       partialProofData,
                                                                                       [
                                                                                        { dup: { n: 0 } },
                                                                                        { idx: { cached: false,
                                                                                                 pushPath: false,
                                                                                                 path: [
                                                                                                        { tag: 'value',
                                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                                        { tag: 'value',
                                                                                                          value: { value: _descriptor_15.toValue(3n),
                                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                                        { popeq: { cached: false,
                                                                                                   result: undefined } }]).value),
                            'NONCE_CONSUMED');
    __compactRuntime.assert(this._equal_11(_descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                     partialProofData,
                                                                                                     [
                                                                                                      { dup: { n: 0 } },
                                                                                                      { idx: { cached: false,
                                                                                                               pushPath: false,
                                                                                                               path: [
                                                                                                                      { tag: 'value',
                                                                                                                        value: { value: _descriptor_15.toValue(0n),
                                                                                                                                 alignment: _descriptor_15.alignment() } },
                                                                                                                      { tag: 'value',
                                                                                                                        value: { value: _descriptor_15.toValue(4n),
                                                                                                                                 alignment: _descriptor_15.alignment() } }] } },
                                                                                                      { popeq: { cached: false,
                                                                                                                 result: undefined } }]).value),
                                           Uint8Array.from([158n,
                                                            70n,
                                                            1n,
                                                            201n,
                                                            175n,
                                                            148n,
                                                            16n,
                                                            34n,
                                                            8n,
                                                            189n,
                                                            107n,
                                                            222n,
                                                            166n,
                                                            105n,
                                                            60n,
                                                            191n,
                                                            90n,
                                                            150n,
                                                            56n,
                                                            246n,
                                                            214n,
                                                            185n,
                                                            250n,
                                                            113n,
                                                            110n,
                                                            174n,
                                                            84n,
                                                            4n,
                                                            67n,
                                                            204n,
                                                            48n,
                                                            228n],
                                                           Number))
                            &&
                            _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                      partialProofData,
                                                                                      [
                                                                                       { dup: { n: 0 } },
                                                                                       { idx: { cached: false,
                                                                                                pushPath: false,
                                                                                                path: [
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(0n),
                                                                                                                  alignment: _descriptor_15.alignment() } },
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(5n),
                                                                                                                  alignment: _descriptor_15.alignment() } }] } },
                                                                                       { popeq: { cached: false,
                                                                                                  result: undefined } }]).value)
                            ===
                            0n,
                            'STALE_HEAD');
    let t_0, t_1;
    __compactRuntime.assert((t_1 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                             partialProofData,
                                                                                             [
                                                                                              { dup: { n: 0 } },
                                                                                              { idx: { cached: false,
                                                                                                       pushPath: false,
                                                                                                       path: [
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(1n),
                                                                                                                         alignment: _descriptor_15.alignment() } },
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(0n),
                                                                                                                         alignment: _descriptor_15.alignment() } }] } },
                                                                                              { popeq: { cached: false,
                                                                                                         result: undefined } }]).value),
                             t_1 >= 0n)
                            &&
                            (t_0 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                             partialProofData,
                                                                                             [
                                                                                              { dup: { n: 0 } },
                                                                                              { idx: { cached: false,
                                                                                                       pushPath: false,
                                                                                                       path: [
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(1n),
                                                                                                                         alignment: _descriptor_15.alignment() } },
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(0n),
                                                                                                                         alignment: _descriptor_15.alignment() } }] } },
                                                                                              { popeq: { cached: false,
                                                                                                         result: undefined } }]).value),
                             t_0 <= 10n),
                            'TRUSTED_ROUND_WINDOW');
    let t_2;
    __compactRuntime.assert((t_2 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                             partialProofData,
                                                                                             [
                                                                                              { dup: { n: 0 } },
                                                                                              { idx: { cached: false,
                                                                                                       pushPath: false,
                                                                                                       path: [
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(1n),
                                                                                                                         alignment: _descriptor_15.alignment() } },
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(1n),
                                                                                                                         alignment: _descriptor_15.alignment() } }] } },
                                                                                              { popeq: { cached: false,
                                                                                                         result: undefined } }]).value),
                             t_2 >= 1010n),
                            'OWNER_UNDERFLOW');
    let t_3;
    __compactRuntime.assert((t_3 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                             partialProofData,
                                                                                             [
                                                                                              { dup: { n: 0 } },
                                                                                              { idx: { cached: false,
                                                                                                       pushPath: false,
                                                                                                       path: [
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(1n),
                                                                                                                         alignment: _descriptor_15.alignment() } },
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(6n),
                                                                                                                         alignment: _descriptor_15.alignment() } }] } },
                                                                                              { popeq: { cached: false,
                                                                                                         result: undefined } }]).value),
                             t_3 >= 1010n),
                            'ALLOWANCE_UNDERFLOW');
    let t_4;
    __compactRuntime.assert((t_4 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                             partialProofData,
                                                                                             [
                                                                                              { dup: { n: 0 } },
                                                                                              { idx: { cached: false,
                                                                                                       pushPath: false,
                                                                                                       path: [
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(1n),
                                                                                                                         alignment: _descriptor_15.alignment() } },
                                                                                                              { tag: 'value',
                                                                                                                value: { value: _descriptor_15.toValue(4n),
                                                                                                                         alignment: _descriptor_15.alignment() } }] } },
                                                                                              { popeq: { cached: false,
                                                                                                         result: undefined } }]).value),
                             t_4 >= 1n),
                            'WORK_EXHAUSTED');
    const workTotal_0 = ((t1) => {
                          if (t1 > 340282366920938463463374607431768211455n) {
                            throw new __compactRuntime.CompactError('fixed-transfer.compact line 65 char 31: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
                          }
                          return t1;
                        })(_descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                     partialProofData,
                                                                                     [
                                                                                      { dup: { n: 0 } },
                                                                                      { idx: { cached: false,
                                                                                               pushPath: false,
                                                                                               path: [
                                                                                                      { tag: 'value',
                                                                                                        value: { value: _descriptor_15.toValue(1n),
                                                                                                                 alignment: _descriptor_15.alignment() } },
                                                                                                      { tag: 'value',
                                                                                                        value: { value: _descriptor_15.toValue(4n),
                                                                                                                 alignment: _descriptor_15.alignment() } }] } },
                                                                                      { popeq: { cached: false,
                                                                                                 result: undefined } }]).value)
                           +
                           _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                     partialProofData,
                                                                                     [
                                                                                      { dup: { n: 0 } },
                                                                                      { idx: { cached: false,
                                                                                               pushPath: false,
                                                                                               path: [
                                                                                                      { tag: 'value',
                                                                                                        value: { value: _descriptor_15.toValue(1n),
                                                                                                                 alignment: _descriptor_15.alignment() } },
                                                                                                      { tag: 'value',
                                                                                                        value: { value: _descriptor_15.toValue(5n),
                                                                                                                 alignment: _descriptor_15.alignment() } }] } },
                                                                                      { popeq: { cached: false,
                                                                                                 result: undefined } }]).value));
    const allowanceTotal_0 = ((t1) => {
                               if (t1 > 340282366920938463463374607431768211455n) {
                                 throw new __compactRuntime.CompactError('fixed-transfer.compact line 66 char 36: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
                               }
                               return t1;
                             })(_descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                          partialProofData,
                                                                                          [
                                                                                           { dup: { n: 0 } },
                                                                                           { idx: { cached: false,
                                                                                                    pushPath: false,
                                                                                                    path: [
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(6n),
                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                           { popeq: { cached: false,
                                                                                                      result: undefined } }]).value)
                                +
                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                          partialProofData,
                                                                                          [
                                                                                           { dup: { n: 0 } },
                                                                                           { idx: { cached: false,
                                                                                                    pushPath: false,
                                                                                                    path: [
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(7n),
                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                           { popeq: { cached: false,
                                                                                                      result: undefined } }]).value));
    __compactRuntime.assert(workTotal_0
                            >=
                            _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                      partialProofData,
                                                                                      [
                                                                                       { dup: { n: 0 } },
                                                                                       { idx: { cached: false,
                                                                                                pushPath: false,
                                                                                                path: [
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(1n),
                                                                                                                  alignment: _descriptor_15.alignment() } },
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(4n),
                                                                                                                  alignment: _descriptor_15.alignment() } }] } },
                                                                                       { popeq: { cached: false,
                                                                                                  result: undefined } }]).value)
                            &&
                            allowanceTotal_0
                            >=
                            _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                      partialProofData,
                                                                                      [
                                                                                       { dup: { n: 0 } },
                                                                                       { idx: { cached: false,
                                                                                                pushPath: false,
                                                                                                path: [
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(1n),
                                                                                                                  alignment: _descriptor_15.alignment() } },
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(6n),
                                                                                                                  alignment: _descriptor_15.alignment() } }] } },
                                                                                       { popeq: { cached: false,
                                                                                                  result: undefined } }]).value),
                            'TOTAL_RANGE');
    const entry_0 = await this._unshieldedBalance_0(context,
                                                    partialProofData,
                                                    _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                              partialProofData,
                                                                                                              [
                                                                                                               { dup: { n: 0 } },
                                                                                                               { idx: { cached: false,
                                                                                                                        pushPath: false,
                                                                                                                        path: [
                                                                                                                               { tag: 'value',
                                                                                                                                 value: { value: _descriptor_15.toValue(1n),
                                                                                                                                          alignment: _descriptor_15.alignment() } },
                                                                                                                               { tag: 'value',
                                                                                                                                 value: { value: _descriptor_15.toValue(8n),
                                                                                                                                          alignment: _descriptor_15.alignment() } }] } },
                                                                                                               { popeq: { cached: false,
                                                                                                                          result: undefined } }]).value));
    __compactRuntime.assert(entry_0
                            ===
                            _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                      partialProofData,
                                                                                      [
                                                                                       { dup: { n: 0 } },
                                                                                       { idx: { cached: false,
                                                                                                pushPath: false,
                                                                                                path: [
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(1n),
                                                                                                                  alignment: _descriptor_15.alignment() } },
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(1n),
                                                                                                                  alignment: _descriptor_15.alignment() } }] } },
                                                                                       { popeq: { cached: false,
                                                                                                  result: undefined } }]).value),
                            'ESCROW_BALANCE_MISMATCH');
    __compactRuntime.assert(await this._unshieldedBalanceGte_0(context,
                                                               partialProofData,
                                                               _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                         partialProofData,
                                                                                                                         [
                                                                                                                          { dup: { n: 0 } },
                                                                                                                          { idx: { cached: false,
                                                                                                                                   pushPath: false,
                                                                                                                                   path: [
                                                                                                                                          { tag: 'value',
                                                                                                                                            value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                     alignment: _descriptor_15.alignment() } },
                                                                                                                                          { tag: 'value',
                                                                                                                                            value: { value: _descriptor_15.toValue(8n),
                                                                                                                                                     alignment: _descriptor_15.alignment() } }] } },
                                                                                                                          { popeq: { cached: false,
                                                                                                                                     result: undefined } }]).value),
                                                               1010n),
                            'ESCROW_UNDERFUNDED');
    const recipientAfter_0 = ((t1) => {
                               if (t1 > 340282366920938463463374607431768211455n) {
                                 throw new __compactRuntime.CompactError('fixed-transfer.compact line 71 char 36: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
                               }
                               return t1;
                             })(_descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                          partialProofData,
                                                                                          [
                                                                                           { dup: { n: 0 } },
                                                                                           { idx: { cached: false,
                                                                                                    pushPath: false,
                                                                                                    path: [
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(2n),
                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                           { popeq: { cached: false,
                                                                                                      result: undefined } }]).value)
                                +
                                1000n);
    const feeAfter_0 = ((t1) => {
                         if (t1 > 340282366920938463463374607431768211455n) {
                           throw new __compactRuntime.CompactError('fixed-transfer.compact line 72 char 30: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
                         }
                         return t1;
                       })(_descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                    partialProofData,
                                                                                    [
                                                                                     { dup: { n: 0 } },
                                                                                     { idx: { cached: false,
                                                                                              pushPath: false,
                                                                                              path: [
                                                                                                     { tag: 'value',
                                                                                                       value: { value: _descriptor_15.toValue(1n),
                                                                                                                alignment: _descriptor_15.alignment() } },
                                                                                                     { tag: 'value',
                                                                                                       value: { value: _descriptor_15.toValue(3n),
                                                                                                                alignment: _descriptor_15.alignment() } }] } },
                                                                                     { popeq: { cached: false,
                                                                                                result: undefined } }]).value)
                          +
                          10n);
    const spentAfter_0 = ((t1) => {
                           if (t1 > 340282366920938463463374607431768211455n) {
                             throw new __compactRuntime.CompactError('fixed-transfer.compact line 73 char 32: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
                           }
                           return t1;
                         })(_descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                      partialProofData,
                                                                                      [
                                                                                       { dup: { n: 0 } },
                                                                                       { idx: { cached: false,
                                                                                                pushPath: false,
                                                                                                path: [
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(1n),
                                                                                                                  alignment: _descriptor_15.alignment() } },
                                                                                                       { tag: 'value',
                                                                                                         value: { value: _descriptor_15.toValue(7n),
                                                                                                                  alignment: _descriptor_15.alignment() } }] } },
                                                                                       { popeq: { cached: false,
                                                                                                  result: undefined } }]).value)
                            +
                            1010n);
    const workSpentAfter_0 = ((t1) => {
                               if (t1 > 340282366920938463463374607431768211455n) {
                                 throw new __compactRuntime.CompactError('fixed-transfer.compact line 74 char 36: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
                               }
                               return t1;
                             })(_descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                          partialProofData,
                                                                                          [
                                                                                           { dup: { n: 0 } },
                                                                                           { idx: { cached: false,
                                                                                                    pushPath: false,
                                                                                                    path: [
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                           { tag: 'value',
                                                                                                             value: { value: _descriptor_15.toValue(5n),
                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                           { popeq: { cached: false,
                                                                                                      result: undefined } }]).value)
                                +
                                1n);
    await this._sendUnshielded_0(context,
                                 partialProofData,
                                 _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                           partialProofData,
                                                                                           [
                                                                                            { dup: { n: 0 } },
                                                                                            { idx: { cached: false,
                                                                                                     pushPath: false,
                                                                                                     path: [
                                                                                                            { tag: 'value',
                                                                                                              value: { value: _descriptor_15.toValue(1n),
                                                                                                                       alignment: _descriptor_15.alignment() } },
                                                                                                            { tag: 'value',
                                                                                                              value: { value: _descriptor_15.toValue(8n),
                                                                                                                       alignment: _descriptor_15.alignment() } }] } },
                                                                                            { popeq: { cached: false,
                                                                                                       result: undefined } }]).value),
                                 1000n,
                                 this._right_0(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                         partialProofData,
                                                                                                         [
                                                                                                          { dup: { n: 0 } },
                                                                                                          { idx: { cached: false,
                                                                                                                   pushPath: false,
                                                                                                                   path: [
                                                                                                                          { tag: 'value',
                                                                                                                            value: { value: _descriptor_15.toValue(1n),
                                                                                                                                     alignment: _descriptor_15.alignment() } },
                                                                                                                          { tag: 'value',
                                                                                                                            value: { value: _descriptor_15.toValue(9n),
                                                                                                                                     alignment: _descriptor_15.alignment() } }] } },
                                                                                                          { popeq: { cached: false,
                                                                                                                     result: undefined } }]).value)));
    await this._sendUnshielded_0(context,
                                 partialProofData,
                                 _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                           partialProofData,
                                                                                           [
                                                                                            { dup: { n: 0 } },
                                                                                            { idx: { cached: false,
                                                                                                     pushPath: false,
                                                                                                     path: [
                                                                                                            { tag: 'value',
                                                                                                              value: { value: _descriptor_15.toValue(1n),
                                                                                                                       alignment: _descriptor_15.alignment() } },
                                                                                                            { tag: 'value',
                                                                                                              value: { value: _descriptor_15.toValue(8n),
                                                                                                                       alignment: _descriptor_15.alignment() } }] } },
                                                                                            { popeq: { cached: false,
                                                                                                       result: undefined } }]).value),
                                 10n,
                                 this._right_0(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                         partialProofData,
                                                                                                         [
                                                                                                          { dup: { n: 0 } },
                                                                                                          { idx: { cached: false,
                                                                                                                   pushPath: false,
                                                                                                                   path: [
                                                                                                                          { tag: 'value',
                                                                                                                            value: { value: _descriptor_15.toValue(1n),
                                                                                                                                     alignment: _descriptor_15.alignment() } },
                                                                                                                          { tag: 'value',
                                                                                                                            value: { value: _descriptor_15.toValue(10n),
                                                                                                                                     alignment: _descriptor_15.alignment() } }] } },
                                                                                                          { popeq: { cached: false,
                                                                                                                     result: undefined } }]).value)));
    let t_5;
    const tmp_0 = (t_5 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                   partialProofData,
                                                                                   [
                                                                                    { dup: { n: 0 } },
                                                                                    { idx: { cached: false,
                                                                                             pushPath: false,
                                                                                             path: [
                                                                                                    { tag: 'value',
                                                                                                      value: { value: _descriptor_15.toValue(1n),
                                                                                                               alignment: _descriptor_15.alignment() } },
                                                                                                    { tag: 'value',
                                                                                                      value: { value: _descriptor_15.toValue(1n),
                                                                                                               alignment: _descriptor_15.alignment() } }] } },
                                                                                    { popeq: { cached: false,
                                                                                               result: undefined } }]).value),
                   (__compactRuntime.assert(t_5 >= 1010n,
                                            'result of subtraction would be negative'),
                    t_5 - 1010n));
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(1n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(2n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(recipientAfter_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(3n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(feeAfter_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    let t_6;
    const tmp_1 = (t_6 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                   partialProofData,
                                                                                   [
                                                                                    { dup: { n: 0 } },
                                                                                    { idx: { cached: false,
                                                                                             pushPath: false,
                                                                                             path: [
                                                                                                    { tag: 'value',
                                                                                                      value: { value: _descriptor_15.toValue(1n),
                                                                                                               alignment: _descriptor_15.alignment() } },
                                                                                                    { tag: 'value',
                                                                                                      value: { value: _descriptor_15.toValue(6n),
                                                                                                               alignment: _descriptor_15.alignment() } }] } },
                                                                                    { popeq: { cached: false,
                                                                                               result: undefined } }]).value),
                   (__compactRuntime.assert(t_6 >= 1010n,
                                            'result of subtraction would be negative'),
                    t_6 - 1010n));
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(6n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_1),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(7n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(spentAfter_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    let t_7;
    const tmp_2 = (t_7 = _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                   partialProofData,
                                                                                   [
                                                                                    { dup: { n: 0 } },
                                                                                    { idx: { cached: false,
                                                                                             pushPath: false,
                                                                                             path: [
                                                                                                    { tag: 'value',
                                                                                                      value: { value: _descriptor_15.toValue(1n),
                                                                                                               alignment: _descriptor_15.alignment() } },
                                                                                                    { tag: 'value',
                                                                                                      value: { value: _descriptor_15.toValue(4n),
                                                                                                               alignment: _descriptor_15.alignment() } }] } },
                                                                                    { popeq: { cached: false,
                                                                                               result: undefined } }]).value),
                   (__compactRuntime.assert(t_7 >= 1n,
                                            'result of subtraction would be negative'),
                    t_7 - 1n));
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(4n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_2),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(5n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(workSpentAfter_0),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_3 = 1010n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(11n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_3),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_4 = 1000n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(12n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_4),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_5 = 10n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(13n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_5),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_6 = 1010n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(1n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(14n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_6),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(3n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_6.toValue(true),
                                                                                              alignment: _descriptor_6.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_7 = 1n;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(5n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_5.toValue(tmp_7),
                                                                                              alignment: _descriptor_5.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    const tmp_8 = this._persistentHash_1([_descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                    partialProofData,
                                                                                                    [
                                                                                                     { dup: { n: 0 } },
                                                                                                     { idx: { cached: false,
                                                                                                              pushPath: false,
                                                                                                              path: [
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(0n),
                                                                                                                                alignment: _descriptor_15.alignment() } },
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(4n),
                                                                                                                                alignment: _descriptor_15.alignment() } }] } },
                                                                                                     { popeq: { cached: false,
                                                                                                                result: undefined } }]).value),
                                          signedDigest_0,
                                          _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                    partialProofData,
                                                                                                    [
                                                                                                     { dup: { n: 0 } },
                                                                                                     { idx: { cached: false,
                                                                                                              pushPath: false,
                                                                                                              path: [
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(0n),
                                                                                                                                alignment: _descriptor_15.alignment() } },
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(1n),
                                                                                                                                alignment: _descriptor_15.alignment() } }] } },
                                                                                                     { popeq: { cached: false,
                                                                                                                result: undefined } }]).value),
                                          _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                    partialProofData,
                                                                                                    [
                                                                                                     { dup: { n: 0 } },
                                                                                                     { idx: { cached: false,
                                                                                                              pushPath: false,
                                                                                                              path: [
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(0n),
                                                                                                                                alignment: _descriptor_15.alignment() } },
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(2n),
                                                                                                                                alignment: _descriptor_15.alignment() } }] } },
                                                                                                     { popeq: { cached: false,
                                                                                                                result: undefined } }]).value),
                                          _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                    partialProofData,
                                                                                                    [
                                                                                                     { dup: { n: 0 } },
                                                                                                     { idx: { cached: false,
                                                                                                              pushPath: false,
                                                                                                              path: [
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(1n),
                                                                                                                                alignment: _descriptor_15.alignment() } },
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(8n),
                                                                                                                                alignment: _descriptor_15.alignment() } }] } },
                                                                                                     { popeq: { cached: false,
                                                                                                                result: undefined } }]).value),
                                          _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                    partialProofData,
                                                                                                    [
                                                                                                     { dup: { n: 0 } },
                                                                                                     { idx: { cached: false,
                                                                                                              pushPath: false,
                                                                                                              path: [
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(1n),
                                                                                                                                alignment: _descriptor_15.alignment() } },
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(9n),
                                                                                                                                alignment: _descriptor_15.alignment() } }] } },
                                                                                                     { popeq: { cached: false,
                                                                                                                result: undefined } }]).value).bytes,
                                          _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                    partialProofData,
                                                                                                    [
                                                                                                     { dup: { n: 0 } },
                                                                                                     { idx: { cached: false,
                                                                                                              pushPath: false,
                                                                                                              path: [
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(1n),
                                                                                                                                alignment: _descriptor_15.alignment() } },
                                                                                                                     { tag: 'value',
                                                                                                                       value: { value: _descriptor_15.toValue(10n),
                                                                                                                                alignment: _descriptor_15.alignment() } }] } },
                                                                                                     { popeq: { cached: false,
                                                                                                                result: undefined } }]).value).bytes,
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 146'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(2n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 172'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(3n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 202'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(6n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 226'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(7n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 258'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(4n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 286'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(5n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 313'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(0n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(5n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 336'),
                                          __compactRuntime.convertBigintToBytes(32,
                                                                                _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                                          partialProofData,
                                                                                                                                          [
                                                                                                                                           { dup: { n: 0 } },
                                                                                                                                           { idx: { cached: false,
                                                                                                                                                    pushPath: false,
                                                                                                                                                    path: [
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(1n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } },
                                                                                                                                                           { tag: 'value',
                                                                                                                                                             value: { value: _descriptor_15.toValue(0n),
                                                                                                                                                                      alignment: _descriptor_15.alignment() } }] } },
                                                                                                                                           { popeq: { cached: false,
                                                                                                                                                      result: undefined } }]).value),
                                                                                'fixed-transfer.compact line 82 char 358')]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { idx: { cached: false,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_15.toValue(0n),
                                                                  alignment: _descriptor_15.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_15.toValue(4n),
                                                                                              alignment: _descriptor_15.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_4.toValue(tmp_8),
                                                                                              alignment: _descriptor_4.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } },
                                       { ins: { cached: true, n: 1 } }]);
    return _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                     partialProofData,
                                                                     [
                                                                      { dup: { n: 0 } },
                                                                      { idx: { cached: false,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_15.toValue(0n),
                                                                                                 alignment: _descriptor_15.alignment() } },
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_15.toValue(4n),
                                                                                                 alignment: _descriptor_15.alignment() } }] } },
                                                                      { popeq: { cached: false,
                                                                                 result: undefined } }]).value);
  }
  _equal_0(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_1(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_2(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_3(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_4(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_5(x0, y0) {
    if (x0.identity) { return y0.identity; }
    if (y0.identity || x0.x != y0.x || x0.y != y0.y) {
      return false;
    }
    return true;
  }
  _equal_6(x0, y0) {
    if (x0.identity) { return y0.identity; }
    if (y0.identity || x0.x != y0.x || x0.y != y0.y) {
      return false;
    }
    return true;
  }
  _equal_7(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_8(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_9(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_10(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
  _equal_11(x0, y0) {
    if (!x0.every((x, i) => y0[i] === x)) { return false; }
    return true;
  }
}
export function ledger(stateOrChargedState) {
  const state = stateOrChargedState instanceof __compactRuntime.StateValue ? stateOrChargedState : stateOrChargedState.state;
  const chargedState = stateOrChargedState instanceof __compactRuntime.StateValue ? new __compactRuntime.ChargedState(stateOrChargedState) : stateOrChargedState;
  const context = {
    callContext: { currentQueryContext: new __compactRuntime.QueryContext(chargedState, __compactRuntime.dummyContractAddress()), currentGasCost: __compactRuntime.emptyRunningCost() },
    costModel: __compactRuntime.CostModel.initialCostModel()
  };
  const partialProofData = {
    input: { value: [], alignment: [] },
    output: undefined,
    publicTranscript: [],
    privateTranscriptOutputs: []
  };
  return {
    get ownerKey() {
      return _descriptor_3.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get sourceDigest() {
      return _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get ownerProgramDigest() {
      return _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(2n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get usedNonce() {
      return _descriptor_6.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(3n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get head() {
      return _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(4n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get revision() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(5n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get trustedRound() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(0n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get ownerBalance() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get recipientBalance() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(2n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get feeBalance() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(3n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get workRemaining() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(4n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get workSpent() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(5n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get allowanceRemaining() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(6n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get allowanceSpent() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(7n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get assetColor() {
      return _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(8n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get recipientAddress() {
      return _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(9n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get feeAddress() {
      return _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(10n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get lastDebit() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(11n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get lastRecipientCredit() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(12n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get lastFeeCredit() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(13n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    },
    get lastAllowanceUse() {
      return _descriptor_5.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(1n),
                                                                                                   alignment: _descriptor_15.alignment() } },
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_15.toValue(14n),
                                                                                                   alignment: _descriptor_15.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    }
  };
}
const _emptyContext = {
  callContext: { currentQueryContext: new __compactRuntime.QueryContext(new __compactRuntime.ContractState().data, __compactRuntime.dummyContractAddress()), currentGasCost: __compactRuntime.emptyRunningCost() }
};
const _dummyContract = new Contract({ });
export const pureCircuits = {
  signatureIsCanonical: (...args_0) => {
    if (args_0.length !== 1) {
      throw new __compactRuntime.CompactError(`signatureIsCanonical: expected 1 argument (as invoked from Typescript), received ${args_0.length}`);
    }
    const sig_0 = args_0[0];
    if (!(typeof(sig_0) === 'object' && typeof(sig_0.r) === 'bigint' && sig_0.r >= 0 && sig_0.r <= __compactRuntime.MAX_SECP256K1_SCALAR && typeof(sig_0.s) === 'bigint' && sig_0.s >= 0 && sig_0.s <= __compactRuntime.MAX_SECP256K1_SCALAR)) {
      __compactRuntime.typeError('signatureIsCanonical',
                                 'argument 1',
                                 'fixed-transfer.compact line 16 char 1',
                                 'struct Secp256k1EcdsaSignature<r: Secp256k1Scalar, s: Secp256k1Scalar>',
                                 sig_0)
    }
    return _dummyContract._signatureIsCanonical_0(sig_0);
  },
  verifyOwner: (...args_0) => {
    if (args_0.length !== 3) {
      throw new __compactRuntime.CompactError(`verifyOwner: expected 3 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const message_0 = args_0[0];
    const sig_0 = args_0[1];
    const pk_0 = args_0[2];
    if (!(message_0.buffer instanceof ArrayBuffer && message_0.BYTES_PER_ELEMENT === 1 && message_0.length === 1278)) {
      __compactRuntime.typeError('verifyOwner',
                                 'argument 1',
                                 'fixed-transfer.compact line 17 char 1',
                                 'Bytes<1278>',
                                 message_0)
    }
    if (!(typeof(sig_0) === 'object' && typeof(sig_0.r) === 'bigint' && sig_0.r >= 0 && sig_0.r <= __compactRuntime.MAX_SECP256K1_SCALAR && typeof(sig_0.s) === 'bigint' && sig_0.s >= 0 && sig_0.s <= __compactRuntime.MAX_SECP256K1_SCALAR)) {
      __compactRuntime.typeError('verifyOwner',
                                 'argument 2',
                                 'fixed-transfer.compact line 17 char 1',
                                 'struct Secp256k1EcdsaSignature<r: Secp256k1Scalar, s: Secp256k1Scalar>',
                                 sig_0)
    }
    if (!(__compactRuntime.isValidSecp256k1Point(pk_0))) {
      __compactRuntime.typeError('verifyOwner',
                                 'argument 3',
                                 'fixed-transfer.compact line 17 char 1',
                                 'Secp256k1Point',
                                 pk_0)
    }
    return _dummyContract._verifyOwner_0(message_0, sig_0, pk_0);
  }
};
export const expectedVk = {};

export const circuitSignatures = {
  'signatureIsCanonical': {pure: true, provable: false, argumentTypes: [{tag: 'Struct', name: 'Secp256k1EcdsaSignature', elements: [{name: 'r', type: {tag: 'Secp256k1Scalar'}}, {name: 's', type: {tag: 'Secp256k1Scalar'}}]}], resultType: {tag: 'Tuple', types: []}},
  'verifyOwner': {pure: true, provable: false, argumentTypes: [{tag: 'Bytes', length: 1278}, {tag: 'Struct', name: 'Secp256k1EcdsaSignature', elements: [{name: 'r', type: {tag: 'Secp256k1Scalar'}}, {name: 's', type: {tag: 'Secp256k1Scalar'}}]}, {tag: 'Secp256k1Point'}], resultType: {tag: 'Bytes', length: 32}},
  'pay': {pure: false, provable: true, argumentTypes: [{tag: 'Bytes', length: 1278}, {tag: 'Struct', name: 'Secp256k1EcdsaSignature', elements: [{name: 'r', type: {tag: 'Secp256k1Scalar'}}, {name: 's', type: {tag: 'Secp256k1Scalar'}}]}], resultType: {tag: 'Bytes', length: 32}},
};

export const declaredInterfaces = {};

//# sourceMappingURL=index.js.map
