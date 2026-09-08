import * as __compactRuntime from '@midnight-ntwrk/compact-runtime';
__compactRuntime.checkRuntimeVersion('0.16.0');

const _descriptor_0 = new __compactRuntime.CompactTypeBytes(32);

const _descriptor_1 = new __compactRuntime.CompactTypeUnsignedInteger(340282366920938463463374607431768211455n, 16);

class _ShieldedCoinInfo_0 {
  alignment() {
    return _descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_1.alignment()));
  }
  fromValue(value_0) {
    return {
      nonce: _descriptor_0.fromValue(value_0),
      color: _descriptor_0.fromValue(value_0),
      value: _descriptor_1.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.nonce).concat(_descriptor_0.toValue(value_0.color).concat(_descriptor_1.toValue(value_0.value)));
  }
}

const _descriptor_2 = new _ShieldedCoinInfo_0();

class _ZswapCoinPublicKey_0 {
  alignment() {
    return _descriptor_0.alignment();
  }
  fromValue(value_0) {
    return {
      bytes: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.bytes);
  }
}

const _descriptor_3 = new _ZswapCoinPublicKey_0();

const _descriptor_4 = __compactRuntime.CompactTypeBoolean;

class _Maybe_0 {
  alignment() {
    return _descriptor_4.alignment().concat(_descriptor_2.alignment());
  }
  fromValue(value_0) {
    return {
      is_some: _descriptor_4.fromValue(value_0),
      value: _descriptor_2.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_4.toValue(value_0.is_some).concat(_descriptor_2.toValue(value_0.value));
  }
}

const _descriptor_5 = new _Maybe_0();

class _ShieldedSendResult_0 {
  alignment() {
    return _descriptor_5.alignment().concat(_descriptor_2.alignment());
  }
  fromValue(value_0) {
    return {
      change: _descriptor_5.fromValue(value_0),
      sent: _descriptor_2.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_5.toValue(value_0.change).concat(_descriptor_2.toValue(value_0.sent));
  }
}

const _descriptor_6 = new _ShieldedSendResult_0();

class _ContractAddress_0 {
  alignment() {
    return _descriptor_0.alignment();
  }
  fromValue(value_0) {
    return {
      bytes: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.bytes);
  }
}

const _descriptor_7 = new _ContractAddress_0();

const _descriptor_8 = new __compactRuntime.CompactTypeUnsignedInteger(18446744073709551615n, 8);

class _QualifiedShieldedCoinInfo_0 {
  alignment() {
    return _descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_1.alignment().concat(_descriptor_8.alignment())));
  }
  fromValue(value_0) {
    return {
      nonce: _descriptor_0.fromValue(value_0),
      color: _descriptor_0.fromValue(value_0),
      value: _descriptor_1.fromValue(value_0),
      mt_index: _descriptor_8.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.nonce).concat(_descriptor_0.toValue(value_0.color).concat(_descriptor_1.toValue(value_0.value).concat(_descriptor_8.toValue(value_0.mt_index))));
  }
}

const _descriptor_9 = new _QualifiedShieldedCoinInfo_0();

class _UserAddress_0 {
  alignment() {
    return _descriptor_0.alignment();
  }
  fromValue(value_0) {
    return {
      bytes: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.bytes);
  }
}

const _descriptor_10 = new _UserAddress_0();

class _Either_0 {
  alignment() {
    return _descriptor_4.alignment().concat(_descriptor_3.alignment().concat(_descriptor_7.alignment()));
  }
  fromValue(value_0) {
    return {
      is_left: _descriptor_4.fromValue(value_0),
      left: _descriptor_3.fromValue(value_0),
      right: _descriptor_7.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_4.toValue(value_0.is_left).concat(_descriptor_3.toValue(value_0.left).concat(_descriptor_7.toValue(value_0.right)));
  }
}

const _descriptor_11 = new _Either_0();

const _descriptor_12 = __compactRuntime.CompactTypeField;

const _descriptor_13 = new __compactRuntime.CompactTypeVector(2, _descriptor_0);

const _descriptor_14 = new __compactRuntime.CompactTypeVector(2, _descriptor_12);

const _descriptor_15 = new __compactRuntime.CompactTypeBytes(21);

class _CoinPreimage_0 {
  alignment() {
    return _descriptor_15.alignment().concat(_descriptor_2.alignment().concat(_descriptor_4.alignment().concat(_descriptor_0.alignment())));
  }
  fromValue(value_0) {
    return {
      domain_sep: _descriptor_15.fromValue(value_0),
      info: _descriptor_2.fromValue(value_0),
      dataType: _descriptor_4.fromValue(value_0),
      data: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_15.toValue(value_0.domain_sep).concat(_descriptor_2.toValue(value_0.info).concat(_descriptor_4.toValue(value_0.dataType).concat(_descriptor_0.toValue(value_0.data))));
  }
}

const _descriptor_16 = new _CoinPreimage_0();

class _Either_1 {
  alignment() {
    return _descriptor_4.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment()));
  }
  fromValue(value_0) {
    return {
      is_left: _descriptor_4.fromValue(value_0),
      left: _descriptor_0.fromValue(value_0),
      right: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_4.toValue(value_0.is_left).concat(_descriptor_0.toValue(value_0.left).concat(_descriptor_0.toValue(value_0.right)));
  }
}

const _descriptor_17 = new _Either_1();

class _Either_2 {
  alignment() {
    return _descriptor_4.alignment().concat(_descriptor_7.alignment().concat(_descriptor_10.alignment()));
  }
  fromValue(value_0) {
    return {
      is_left: _descriptor_4.fromValue(value_0),
      left: _descriptor_7.fromValue(value_0),
      right: _descriptor_10.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_4.toValue(value_0.is_left).concat(_descriptor_7.toValue(value_0.left).concat(_descriptor_10.toValue(value_0.right)));
  }
}

const _descriptor_18 = new _Either_2();

const _descriptor_19 = new __compactRuntime.CompactTypeUnsignedInteger(255n, 1);

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
      mintUnshieldedToSelfTest: (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`mintUnshieldedToSelfTest: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const domainSep_0 = args_1[1];
        const amount_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('mintUnshieldedToSelfTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 18 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(domainSep_0.buffer instanceof ArrayBuffer && domainSep_0.BYTES_PER_ELEMENT === 1 && domainSep_0.length === 32)) {
          __compactRuntime.typeError('mintUnshieldedToSelfTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 18 char 1',
                                     'Bytes<32>',
                                     domainSep_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 18446744073709551615n)) {
          __compactRuntime.typeError('mintUnshieldedToSelfTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 18 char 1',
                                     'Uint<0..18446744073709551616>',
                                     amount_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(domainSep_0).concat(_descriptor_8.toValue(amount_0)),
            alignment: _descriptor_0.alignment().concat(_descriptor_8.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._mintUnshieldedToSelfTest_0(context,
                                                          partialProofData,
                                                          domainSep_0,
                                                          amount_0);
        partialProofData.output = { value: _descriptor_0.toValue(result_0), alignment: _descriptor_0.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      mintUnshieldedToContractTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`mintUnshieldedToContractTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const domainSep_0 = args_1[1];
        const address_0 = args_1[2];
        const amount_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('mintUnshieldedToContractTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 22 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(domainSep_0.buffer instanceof ArrayBuffer && domainSep_0.BYTES_PER_ELEMENT === 1 && domainSep_0.length === 32)) {
          __compactRuntime.typeError('mintUnshieldedToContractTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 22 char 1',
                                     'Bytes<32>',
                                     domainSep_0)
        }
        if (!(typeof(address_0) === 'object' && address_0.bytes.buffer instanceof ArrayBuffer && address_0.bytes.BYTES_PER_ELEMENT === 1 && address_0.bytes.length === 32)) {
          __compactRuntime.typeError('mintUnshieldedToContractTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 22 char 1',
                                     'struct ContractAddress<bytes: Bytes<32>>',
                                     address_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 18446744073709551615n)) {
          __compactRuntime.typeError('mintUnshieldedToContractTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 22 char 1',
                                     'Uint<0..18446744073709551616>',
                                     amount_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(domainSep_0).concat(_descriptor_7.toValue(address_0).concat(_descriptor_8.toValue(amount_0))),
            alignment: _descriptor_0.alignment().concat(_descriptor_7.alignment().concat(_descriptor_8.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._mintUnshieldedToContractTest_0(context,
                                                              partialProofData,
                                                              domainSep_0,
                                                              address_0,
                                                              amount_0);
        partialProofData.output = { value: _descriptor_0.toValue(result_0), alignment: _descriptor_0.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      mintUnshieldedToUserTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`mintUnshieldedToUserTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const domainSep_0 = args_1[1];
        const address_0 = args_1[2];
        const amount_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('mintUnshieldedToUserTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 26 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(domainSep_0.buffer instanceof ArrayBuffer && domainSep_0.BYTES_PER_ELEMENT === 1 && domainSep_0.length === 32)) {
          __compactRuntime.typeError('mintUnshieldedToUserTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 26 char 1',
                                     'Bytes<32>',
                                     domainSep_0)
        }
        if (!(typeof(address_0) === 'object' && address_0.bytes.buffer instanceof ArrayBuffer && address_0.bytes.BYTES_PER_ELEMENT === 1 && address_0.bytes.length === 32)) {
          __compactRuntime.typeError('mintUnshieldedToUserTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 26 char 1',
                                     'struct UserAddress<bytes: Bytes<32>>',
                                     address_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 18446744073709551615n)) {
          __compactRuntime.typeError('mintUnshieldedToUserTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 26 char 1',
                                     'Uint<0..18446744073709551616>',
                                     amount_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(domainSep_0).concat(_descriptor_10.toValue(address_0).concat(_descriptor_8.toValue(amount_0))),
            alignment: _descriptor_0.alignment().concat(_descriptor_10.alignment().concat(_descriptor_8.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._mintUnshieldedToUserTest_0(context,
                                                          partialProofData,
                                                          domainSep_0,
                                                          address_0,
                                                          amount_0);
        partialProofData.output = { value: _descriptor_0.toValue(result_0), alignment: _descriptor_0.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendUnshieldedToSelfTest: (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`sendUnshieldedToSelfTest: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const color_0 = args_1[1];
        const amount_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendUnshieldedToSelfTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 30 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
          __compactRuntime.typeError('sendUnshieldedToSelfTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 30 char 1',
                                     'Bytes<32>',
                                     color_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendUnshieldedToSelfTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 30 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     amount_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(color_0).concat(_descriptor_1.toValue(amount_0)),
            alignment: _descriptor_0.alignment().concat(_descriptor_1.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendUnshieldedToSelfTest_0(context,
                                                          partialProofData,
                                                          color_0,
                                                          amount_0);
        partialProofData.output = { value: [], alignment: [] };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendUnshieldedToContractTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`sendUnshieldedToContractTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const color_0 = args_1[1];
        const amount_0 = args_1[2];
        const address_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendUnshieldedToContractTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 34 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
          __compactRuntime.typeError('sendUnshieldedToContractTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 34 char 1',
                                     'Bytes<32>',
                                     color_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendUnshieldedToContractTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 34 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     amount_0)
        }
        if (!(typeof(address_0) === 'object' && address_0.bytes.buffer instanceof ArrayBuffer && address_0.bytes.BYTES_PER_ELEMENT === 1 && address_0.bytes.length === 32)) {
          __compactRuntime.typeError('sendUnshieldedToContractTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 34 char 1',
                                     'struct ContractAddress<bytes: Bytes<32>>',
                                     address_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(color_0).concat(_descriptor_1.toValue(amount_0).concat(_descriptor_7.toValue(address_0))),
            alignment: _descriptor_0.alignment().concat(_descriptor_1.alignment().concat(_descriptor_7.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendUnshieldedToContractTest_0(context,
                                                              partialProofData,
                                                              color_0,
                                                              amount_0,
                                                              address_0);
        partialProofData.output = { value: [], alignment: [] };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendUnshieldedToUserTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`sendUnshieldedToUserTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const color_0 = args_1[1];
        const amount_0 = args_1[2];
        const address_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendUnshieldedToUserTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 38 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
          __compactRuntime.typeError('sendUnshieldedToUserTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 38 char 1',
                                     'Bytes<32>',
                                     color_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendUnshieldedToUserTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 38 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     amount_0)
        }
        if (!(typeof(address_0) === 'object' && address_0.bytes.buffer instanceof ArrayBuffer && address_0.bytes.BYTES_PER_ELEMENT === 1 && address_0.bytes.length === 32)) {
          __compactRuntime.typeError('sendUnshieldedToUserTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 38 char 1',
                                     'struct UserAddress<bytes: Bytes<32>>',
                                     address_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(color_0).concat(_descriptor_1.toValue(amount_0).concat(_descriptor_10.toValue(address_0))),
            alignment: _descriptor_0.alignment().concat(_descriptor_1.alignment().concat(_descriptor_10.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendUnshieldedToUserTest_0(context,
                                                          partialProofData,
                                                          color_0,
                                                          amount_0,
                                                          address_0);
        partialProofData.output = { value: [], alignment: [] };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      receiveUnshieldedTest: (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`receiveUnshieldedTest: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const color_0 = args_1[1];
        const amount_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('receiveUnshieldedTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 42 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
          __compactRuntime.typeError('receiveUnshieldedTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 42 char 1',
                                     'Bytes<32>',
                                     color_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('receiveUnshieldedTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 42 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     amount_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(color_0).concat(_descriptor_1.toValue(amount_0)),
            alignment: _descriptor_0.alignment().concat(_descriptor_1.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._receiveUnshieldedTest_0(context,
                                                       partialProofData,
                                                       color_0,
                                                       amount_0);
        partialProofData.output = { value: [], alignment: [] };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      getUnshieldedBalanceTest: (...args_1) => {
        if (args_1.length !== 2) {
          throw new __compactRuntime.CompactError(`getUnshieldedBalanceTest: expected 2 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const color_0 = args_1[1];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('getUnshieldedBalanceTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 46 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
          __compactRuntime.typeError('getUnshieldedBalanceTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 46 char 1',
                                     'Bytes<32>',
                                     color_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(color_0),
            alignment: _descriptor_0.alignment()
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._getUnshieldedBalanceTest_0(context,
                                                          partialProofData,
                                                          color_0);
        partialProofData.output = { value: _descriptor_1.toValue(result_0), alignment: _descriptor_1.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      getUnshieldedBalanceGtTest: (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`getUnshieldedBalanceGtTest: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const color_0 = args_1[1];
        const amount_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('getUnshieldedBalanceGtTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 50 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
          __compactRuntime.typeError('getUnshieldedBalanceGtTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 50 char 1',
                                     'Bytes<32>',
                                     color_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('getUnshieldedBalanceGtTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 50 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     amount_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(color_0).concat(_descriptor_1.toValue(amount_0)),
            alignment: _descriptor_0.alignment().concat(_descriptor_1.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._getUnshieldedBalanceGtTest_0(context,
                                                            partialProofData,
                                                            color_0,
                                                            amount_0);
        partialProofData.output = { value: _descriptor_4.toValue(result_0), alignment: _descriptor_4.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      getUnshieldedBalanceLtTest: (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`getUnshieldedBalanceLtTest: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const color_0 = args_1[1];
        const amount_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('getUnshieldedBalanceLtTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 54 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(color_0.buffer instanceof ArrayBuffer && color_0.BYTES_PER_ELEMENT === 1 && color_0.length === 32)) {
          __compactRuntime.typeError('getUnshieldedBalanceLtTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 54 char 1',
                                     'Bytes<32>',
                                     color_0)
        }
        if (!(typeof(amount_0) === 'bigint' && amount_0 >= 0n && amount_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('getUnshieldedBalanceLtTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 54 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     amount_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(color_0).concat(_descriptor_1.toValue(amount_0)),
            alignment: _descriptor_0.alignment().concat(_descriptor_1.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._getUnshieldedBalanceLtTest_0(context,
                                                            partialProofData,
                                                            color_0,
                                                            amount_0);
        partialProofData.output = { value: _descriptor_4.toValue(result_0), alignment: _descriptor_4.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      mintShieldedToSelfTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`mintShieldedToSelfTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const domainSep_0 = args_1[1];
        const value_0 = args_1[2];
        const nonce_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('mintShieldedToSelfTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 60 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(domainSep_0.buffer instanceof ArrayBuffer && domainSep_0.BYTES_PER_ELEMENT === 1 && domainSep_0.length === 32)) {
          __compactRuntime.typeError('mintShieldedToSelfTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 60 char 1',
                                     'Bytes<32>',
                                     domainSep_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 18446744073709551615n)) {
          __compactRuntime.typeError('mintShieldedToSelfTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 60 char 1',
                                     'Uint<0..18446744073709551616>',
                                     value_0)
        }
        if (!(nonce_0.buffer instanceof ArrayBuffer && nonce_0.BYTES_PER_ELEMENT === 1 && nonce_0.length === 32)) {
          __compactRuntime.typeError('mintShieldedToSelfTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 60 char 1',
                                     'Bytes<32>',
                                     nonce_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(domainSep_0).concat(_descriptor_8.toValue(value_0).concat(_descriptor_0.toValue(nonce_0))),
            alignment: _descriptor_0.alignment().concat(_descriptor_8.alignment().concat(_descriptor_0.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._mintShieldedToSelfTest_0(context,
                                                        partialProofData,
                                                        domainSep_0,
                                                        value_0,
                                                        nonce_0);
        partialProofData.output = { value: _descriptor_2.toValue(result_0), alignment: _descriptor_2.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      mintShieldedToContractTest: (...args_1) => {
        if (args_1.length !== 5) {
          throw new __compactRuntime.CompactError(`mintShieldedToContractTest: expected 5 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const domainSep_0 = args_1[1];
        const value_0 = args_1[2];
        const nonce_0 = args_1[3];
        const address_0 = args_1[4];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('mintShieldedToContractTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 64 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(domainSep_0.buffer instanceof ArrayBuffer && domainSep_0.BYTES_PER_ELEMENT === 1 && domainSep_0.length === 32)) {
          __compactRuntime.typeError('mintShieldedToContractTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 64 char 1',
                                     'Bytes<32>',
                                     domainSep_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 18446744073709551615n)) {
          __compactRuntime.typeError('mintShieldedToContractTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 64 char 1',
                                     'Uint<0..18446744073709551616>',
                                     value_0)
        }
        if (!(nonce_0.buffer instanceof ArrayBuffer && nonce_0.BYTES_PER_ELEMENT === 1 && nonce_0.length === 32)) {
          __compactRuntime.typeError('mintShieldedToContractTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 64 char 1',
                                     'Bytes<32>',
                                     nonce_0)
        }
        if (!(typeof(address_0) === 'object' && address_0.bytes.buffer instanceof ArrayBuffer && address_0.bytes.BYTES_PER_ELEMENT === 1 && address_0.bytes.length === 32)) {
          __compactRuntime.typeError('mintShieldedToContractTest',
                                     'argument 4 (argument 5 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 64 char 1',
                                     'struct ContractAddress<bytes: Bytes<32>>',
                                     address_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(domainSep_0).concat(_descriptor_8.toValue(value_0).concat(_descriptor_0.toValue(nonce_0).concat(_descriptor_7.toValue(address_0)))),
            alignment: _descriptor_0.alignment().concat(_descriptor_8.alignment().concat(_descriptor_0.alignment().concat(_descriptor_7.alignment())))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._mintShieldedToContractTest_0(context,
                                                            partialProofData,
                                                            domainSep_0,
                                                            value_0,
                                                            nonce_0,
                                                            address_0);
        partialProofData.output = { value: _descriptor_2.toValue(result_0), alignment: _descriptor_2.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      mintShieldedToUserTest: (...args_1) => {
        if (args_1.length !== 5) {
          throw new __compactRuntime.CompactError(`mintShieldedToUserTest: expected 5 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const domainSep_0 = args_1[1];
        const value_0 = args_1[2];
        const nonce_0 = args_1[3];
        const publicKey_0 = args_1[4];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('mintShieldedToUserTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 68 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(domainSep_0.buffer instanceof ArrayBuffer && domainSep_0.BYTES_PER_ELEMENT === 1 && domainSep_0.length === 32)) {
          __compactRuntime.typeError('mintShieldedToUserTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 68 char 1',
                                     'Bytes<32>',
                                     domainSep_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 18446744073709551615n)) {
          __compactRuntime.typeError('mintShieldedToUserTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 68 char 1',
                                     'Uint<0..18446744073709551616>',
                                     value_0)
        }
        if (!(nonce_0.buffer instanceof ArrayBuffer && nonce_0.BYTES_PER_ELEMENT === 1 && nonce_0.length === 32)) {
          __compactRuntime.typeError('mintShieldedToUserTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 68 char 1',
                                     'Bytes<32>',
                                     nonce_0)
        }
        if (!(typeof(publicKey_0) === 'object' && publicKey_0.bytes.buffer instanceof ArrayBuffer && publicKey_0.bytes.BYTES_PER_ELEMENT === 1 && publicKey_0.bytes.length === 32)) {
          __compactRuntime.typeError('mintShieldedToUserTest',
                                     'argument 4 (argument 5 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 68 char 1',
                                     'struct ZswapCoinPublicKey<bytes: Bytes<32>>',
                                     publicKey_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_0.toValue(domainSep_0).concat(_descriptor_8.toValue(value_0).concat(_descriptor_0.toValue(nonce_0).concat(_descriptor_3.toValue(publicKey_0)))),
            alignment: _descriptor_0.alignment().concat(_descriptor_8.alignment().concat(_descriptor_0.alignment().concat(_descriptor_3.alignment())))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._mintShieldedToUserTest_0(context,
                                                        partialProofData,
                                                        domainSep_0,
                                                        value_0,
                                                        nonce_0,
                                                        publicKey_0);
        partialProofData.output = { value: _descriptor_2.toValue(result_0), alignment: _descriptor_2.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      receiveShieldedTest: (...args_1) => {
        if (args_1.length !== 2) {
          throw new __compactRuntime.CompactError(`receiveShieldedTest: expected 2 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const coin_0 = args_1[1];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('receiveShieldedTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 72 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(coin_0) === 'object' && coin_0.nonce.buffer instanceof ArrayBuffer && coin_0.nonce.BYTES_PER_ELEMENT === 1 && coin_0.nonce.length === 32 && coin_0.color.buffer instanceof ArrayBuffer && coin_0.color.BYTES_PER_ELEMENT === 1 && coin_0.color.length === 32 && typeof(coin_0.value) === 'bigint' && coin_0.value >= 0n && coin_0.value <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('receiveShieldedTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 72 char 1',
                                     'struct ShieldedCoinInfo<nonce: Bytes<32>, color: Bytes<32>, value: Uint<0..340282366920938463463374607431768211456>>',
                                     coin_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_2.toValue(coin_0),
            alignment: _descriptor_2.alignment()
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._receiveShieldedTest_0(context,
                                                     partialProofData,
                                                     coin_0);
        partialProofData.output = { value: [], alignment: [] };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendShieldedToSelfTest: (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`sendShieldedToSelfTest: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const input_0 = args_1[1];
        const value_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendShieldedToSelfTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 76 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(input_0) === 'object' && input_0.nonce.buffer instanceof ArrayBuffer && input_0.nonce.BYTES_PER_ELEMENT === 1 && input_0.nonce.length === 32 && input_0.color.buffer instanceof ArrayBuffer && input_0.color.BYTES_PER_ELEMENT === 1 && input_0.color.length === 32 && typeof(input_0.value) === 'bigint' && input_0.value >= 0n && input_0.value <= 340282366920938463463374607431768211455n && typeof(input_0.mt_index) === 'bigint' && input_0.mt_index >= 0n && input_0.mt_index <= 18446744073709551615n)) {
          __compactRuntime.typeError('sendShieldedToSelfTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 76 char 1',
                                     'struct QualifiedShieldedCoinInfo<nonce: Bytes<32>, color: Bytes<32>, value: Uint<0..340282366920938463463374607431768211456>, mt_index: Uint<0..18446744073709551616>>',
                                     input_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendShieldedToSelfTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 76 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     value_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_9.toValue(input_0).concat(_descriptor_1.toValue(value_0)),
            alignment: _descriptor_9.alignment().concat(_descriptor_1.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendShieldedToSelfTest_0(context,
                                                        partialProofData,
                                                        input_0,
                                                        value_0);
        partialProofData.output = { value: _descriptor_6.toValue(result_0), alignment: _descriptor_6.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendShieldedToContractTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`sendShieldedToContractTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const input_0 = args_1[1];
        const address_0 = args_1[2];
        const value_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendShieldedToContractTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 80 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(input_0) === 'object' && input_0.nonce.buffer instanceof ArrayBuffer && input_0.nonce.BYTES_PER_ELEMENT === 1 && input_0.nonce.length === 32 && input_0.color.buffer instanceof ArrayBuffer && input_0.color.BYTES_PER_ELEMENT === 1 && input_0.color.length === 32 && typeof(input_0.value) === 'bigint' && input_0.value >= 0n && input_0.value <= 340282366920938463463374607431768211455n && typeof(input_0.mt_index) === 'bigint' && input_0.mt_index >= 0n && input_0.mt_index <= 18446744073709551615n)) {
          __compactRuntime.typeError('sendShieldedToContractTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 80 char 1',
                                     'struct QualifiedShieldedCoinInfo<nonce: Bytes<32>, color: Bytes<32>, value: Uint<0..340282366920938463463374607431768211456>, mt_index: Uint<0..18446744073709551616>>',
                                     input_0)
        }
        if (!(typeof(address_0) === 'object' && address_0.bytes.buffer instanceof ArrayBuffer && address_0.bytes.BYTES_PER_ELEMENT === 1 && address_0.bytes.length === 32)) {
          __compactRuntime.typeError('sendShieldedToContractTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 80 char 1',
                                     'struct ContractAddress<bytes: Bytes<32>>',
                                     address_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendShieldedToContractTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 80 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     value_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_9.toValue(input_0).concat(_descriptor_7.toValue(address_0).concat(_descriptor_1.toValue(value_0))),
            alignment: _descriptor_9.alignment().concat(_descriptor_7.alignment().concat(_descriptor_1.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendShieldedToContractTest_0(context,
                                                            partialProofData,
                                                            input_0,
                                                            address_0,
                                                            value_0);
        partialProofData.output = { value: _descriptor_6.toValue(result_0), alignment: _descriptor_6.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendShieldedToUserTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`sendShieldedToUserTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const input_0 = args_1[1];
        const publicKey_0 = args_1[2];
        const value_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendShieldedToUserTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 84 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(input_0) === 'object' && input_0.nonce.buffer instanceof ArrayBuffer && input_0.nonce.BYTES_PER_ELEMENT === 1 && input_0.nonce.length === 32 && input_0.color.buffer instanceof ArrayBuffer && input_0.color.BYTES_PER_ELEMENT === 1 && input_0.color.length === 32 && typeof(input_0.value) === 'bigint' && input_0.value >= 0n && input_0.value <= 340282366920938463463374607431768211455n && typeof(input_0.mt_index) === 'bigint' && input_0.mt_index >= 0n && input_0.mt_index <= 18446744073709551615n)) {
          __compactRuntime.typeError('sendShieldedToUserTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 84 char 1',
                                     'struct QualifiedShieldedCoinInfo<nonce: Bytes<32>, color: Bytes<32>, value: Uint<0..340282366920938463463374607431768211456>, mt_index: Uint<0..18446744073709551616>>',
                                     input_0)
        }
        if (!(typeof(publicKey_0) === 'object' && publicKey_0.bytes.buffer instanceof ArrayBuffer && publicKey_0.bytes.BYTES_PER_ELEMENT === 1 && publicKey_0.bytes.length === 32)) {
          __compactRuntime.typeError('sendShieldedToUserTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 84 char 1',
                                     'struct ZswapCoinPublicKey<bytes: Bytes<32>>',
                                     publicKey_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendShieldedToUserTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 84 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     value_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_9.toValue(input_0).concat(_descriptor_3.toValue(publicKey_0).concat(_descriptor_1.toValue(value_0))),
            alignment: _descriptor_9.alignment().concat(_descriptor_3.alignment().concat(_descriptor_1.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendShieldedToUserTest_0(context,
                                                        partialProofData,
                                                        input_0,
                                                        publicKey_0,
                                                        value_0);
        partialProofData.output = { value: _descriptor_6.toValue(result_0), alignment: _descriptor_6.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendImmediateShieldedToSelfTest: (...args_1) => {
        if (args_1.length !== 3) {
          throw new __compactRuntime.CompactError(`sendImmediateShieldedToSelfTest: expected 3 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const input_0 = args_1[1];
        const value_0 = args_1[2];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendImmediateShieldedToSelfTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 88 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(input_0) === 'object' && input_0.nonce.buffer instanceof ArrayBuffer && input_0.nonce.BYTES_PER_ELEMENT === 1 && input_0.nonce.length === 32 && input_0.color.buffer instanceof ArrayBuffer && input_0.color.BYTES_PER_ELEMENT === 1 && input_0.color.length === 32 && typeof(input_0.value) === 'bigint' && input_0.value >= 0n && input_0.value <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendImmediateShieldedToSelfTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 88 char 1',
                                     'struct ShieldedCoinInfo<nonce: Bytes<32>, color: Bytes<32>, value: Uint<0..340282366920938463463374607431768211456>>',
                                     input_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendImmediateShieldedToSelfTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 88 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     value_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_2.toValue(input_0).concat(_descriptor_1.toValue(value_0)),
            alignment: _descriptor_2.alignment().concat(_descriptor_1.alignment())
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendImmediateShieldedToSelfTest_0(context,
                                                                 partialProofData,
                                                                 input_0,
                                                                 value_0);
        partialProofData.output = { value: _descriptor_6.toValue(result_0), alignment: _descriptor_6.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendImmediateShieldedToContractTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`sendImmediateShieldedToContractTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const input_0 = args_1[1];
        const address_0 = args_1[2];
        const value_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendImmediateShieldedToContractTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 92 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(input_0) === 'object' && input_0.nonce.buffer instanceof ArrayBuffer && input_0.nonce.BYTES_PER_ELEMENT === 1 && input_0.nonce.length === 32 && input_0.color.buffer instanceof ArrayBuffer && input_0.color.BYTES_PER_ELEMENT === 1 && input_0.color.length === 32 && typeof(input_0.value) === 'bigint' && input_0.value >= 0n && input_0.value <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendImmediateShieldedToContractTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 92 char 1',
                                     'struct ShieldedCoinInfo<nonce: Bytes<32>, color: Bytes<32>, value: Uint<0..340282366920938463463374607431768211456>>',
                                     input_0)
        }
        if (!(typeof(address_0) === 'object' && address_0.bytes.buffer instanceof ArrayBuffer && address_0.bytes.BYTES_PER_ELEMENT === 1 && address_0.bytes.length === 32)) {
          __compactRuntime.typeError('sendImmediateShieldedToContractTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 92 char 1',
                                     'struct ContractAddress<bytes: Bytes<32>>',
                                     address_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendImmediateShieldedToContractTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 92 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     value_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_2.toValue(input_0).concat(_descriptor_7.toValue(address_0).concat(_descriptor_1.toValue(value_0))),
            alignment: _descriptor_2.alignment().concat(_descriptor_7.alignment().concat(_descriptor_1.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendImmediateShieldedToContractTest_0(context,
                                                                     partialProofData,
                                                                     input_0,
                                                                     address_0,
                                                                     value_0);
        partialProofData.output = { value: _descriptor_6.toValue(result_0), alignment: _descriptor_6.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      sendImmediateShieldedToUserTest: (...args_1) => {
        if (args_1.length !== 4) {
          throw new __compactRuntime.CompactError(`sendImmediateShieldedToUserTest: expected 4 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const input_0 = args_1[1];
        const publicKey_0 = args_1[2];
        const value_0 = args_1[3];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('sendImmediateShieldedToUserTest',
                                     'argument 1 (as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 96 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(input_0) === 'object' && input_0.nonce.buffer instanceof ArrayBuffer && input_0.nonce.BYTES_PER_ELEMENT === 1 && input_0.nonce.length === 32 && input_0.color.buffer instanceof ArrayBuffer && input_0.color.BYTES_PER_ELEMENT === 1 && input_0.color.length === 32 && typeof(input_0.value) === 'bigint' && input_0.value >= 0n && input_0.value <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendImmediateShieldedToUserTest',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 96 char 1',
                                     'struct ShieldedCoinInfo<nonce: Bytes<32>, color: Bytes<32>, value: Uint<0..340282366920938463463374607431768211456>>',
                                     input_0)
        }
        if (!(typeof(publicKey_0) === 'object' && publicKey_0.bytes.buffer instanceof ArrayBuffer && publicKey_0.bytes.BYTES_PER_ELEMENT === 1 && publicKey_0.bytes.length === 32)) {
          __compactRuntime.typeError('sendImmediateShieldedToUserTest',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 96 char 1',
                                     'struct ZswapCoinPublicKey<bytes: Bytes<32>>',
                                     publicKey_0)
        }
        if (!(typeof(value_0) === 'bigint' && value_0 >= 0n && value_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('sendImmediateShieldedToUserTest',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'upstream-token-interfaces.compact line 96 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     value_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_2.toValue(input_0).concat(_descriptor_3.toValue(publicKey_0).concat(_descriptor_1.toValue(value_0))),
            alignment: _descriptor_2.alignment().concat(_descriptor_3.alignment().concat(_descriptor_1.alignment()))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._sendImmediateShieldedToUserTest_0(context,
                                                                 partialProofData,
                                                                 input_0,
                                                                 publicKey_0,
                                                                 value_0);
        partialProofData.output = { value: _descriptor_6.toValue(result_0), alignment: _descriptor_6.alignment() };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      }
    };
    this.impureCircuits = {
      mintUnshieldedToSelfTest: this.circuits.mintUnshieldedToSelfTest,
      mintUnshieldedToContractTest: this.circuits.mintUnshieldedToContractTest,
      mintUnshieldedToUserTest: this.circuits.mintUnshieldedToUserTest,
      sendUnshieldedToSelfTest: this.circuits.sendUnshieldedToSelfTest,
      sendUnshieldedToContractTest: this.circuits.sendUnshieldedToContractTest,
      sendUnshieldedToUserTest: this.circuits.sendUnshieldedToUserTest,
      receiveUnshieldedTest: this.circuits.receiveUnshieldedTest,
      getUnshieldedBalanceTest: this.circuits.getUnshieldedBalanceTest,
      getUnshieldedBalanceGtTest: this.circuits.getUnshieldedBalanceGtTest,
      getUnshieldedBalanceLtTest: this.circuits.getUnshieldedBalanceLtTest,
      mintShieldedToSelfTest: this.circuits.mintShieldedToSelfTest,
      mintShieldedToContractTest: this.circuits.mintShieldedToContractTest,
      mintShieldedToUserTest: this.circuits.mintShieldedToUserTest,
      receiveShieldedTest: this.circuits.receiveShieldedTest,
      sendShieldedToSelfTest: this.circuits.sendShieldedToSelfTest,
      sendShieldedToContractTest: this.circuits.sendShieldedToContractTest,
      sendShieldedToUserTest: this.circuits.sendShieldedToUserTest,
      sendImmediateShieldedToSelfTest: this.circuits.sendImmediateShieldedToSelfTest,
      sendImmediateShieldedToContractTest: this.circuits.sendImmediateShieldedToContractTest,
      sendImmediateShieldedToUserTest: this.circuits.sendImmediateShieldedToUserTest
    };
    this.provableCircuits = {
      mintUnshieldedToSelfTest: this.circuits.mintUnshieldedToSelfTest,
      mintUnshieldedToContractTest: this.circuits.mintUnshieldedToContractTest,
      mintUnshieldedToUserTest: this.circuits.mintUnshieldedToUserTest,
      sendUnshieldedToSelfTest: this.circuits.sendUnshieldedToSelfTest,
      sendUnshieldedToContractTest: this.circuits.sendUnshieldedToContractTest,
      sendUnshieldedToUserTest: this.circuits.sendUnshieldedToUserTest,
      receiveUnshieldedTest: this.circuits.receiveUnshieldedTest,
      getUnshieldedBalanceTest: this.circuits.getUnshieldedBalanceTest,
      getUnshieldedBalanceGtTest: this.circuits.getUnshieldedBalanceGtTest,
      getUnshieldedBalanceLtTest: this.circuits.getUnshieldedBalanceLtTest,
      mintShieldedToSelfTest: this.circuits.mintShieldedToSelfTest,
      mintShieldedToContractTest: this.circuits.mintShieldedToContractTest,
      mintShieldedToUserTest: this.circuits.mintShieldedToUserTest,
      receiveShieldedTest: this.circuits.receiveShieldedTest,
      sendShieldedToSelfTest: this.circuits.sendShieldedToSelfTest,
      sendShieldedToContractTest: this.circuits.sendShieldedToContractTest,
      sendShieldedToUserTest: this.circuits.sendShieldedToUserTest,
      sendImmediateShieldedToSelfTest: this.circuits.sendImmediateShieldedToSelfTest,
      sendImmediateShieldedToContractTest: this.circuits.sendImmediateShieldedToContractTest,
      sendImmediateShieldedToUserTest: this.circuits.sendImmediateShieldedToUserTest
    };
  }
  initialState(...args_0) {
    if (args_0.length !== 1) {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 1 argument (as invoked from Typescript), received ${args_0.length}`);
    }
    const constructorContext_0 = args_0[0];
    if (typeof(constructorContext_0) !== 'object') {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 'constructorContext' in argument 1 (as invoked from Typescript) to be an object`);
    }
    if (!('initialZswapLocalState' in constructorContext_0)) {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 'initialZswapLocalState' in argument 1 (as invoked from Typescript)`);
    }
    if (typeof(constructorContext_0.initialZswapLocalState) !== 'object') {
      throw new __compactRuntime.CompactError(`Contract state constructor: expected 'initialZswapLocalState' in argument 1 (as invoked from Typescript) to be an object`);
    }
    const state_0 = new __compactRuntime.ContractState();
    let stateValue_0 = __compactRuntime.StateValue.newArray();
    state_0.data = new __compactRuntime.ChargedState(stateValue_0);
    state_0.setOperation('mintUnshieldedToSelfTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('mintUnshieldedToContractTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('mintUnshieldedToUserTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendUnshieldedToSelfTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendUnshieldedToContractTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendUnshieldedToUserTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('receiveUnshieldedTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('getUnshieldedBalanceTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('getUnshieldedBalanceGtTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('getUnshieldedBalanceLtTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('mintShieldedToSelfTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('mintShieldedToContractTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('mintShieldedToUserTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('receiveShieldedTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendShieldedToSelfTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendShieldedToContractTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendShieldedToUserTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendImmediateShieldedToSelfTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendImmediateShieldedToContractTest', new __compactRuntime.ContractOperation());
    state_0.setOperation('sendImmediateShieldedToUserTest', new __compactRuntime.ContractOperation());
    const context = __compactRuntime.createCircuitContext(__compactRuntime.dummyContractAddress(), constructorContext_0.initialZswapLocalState.coinPublicKey, state_0.data, constructorContext_0.initialPrivateState);
    const partialProofData = {
      input: { value: [], alignment: [] },
      output: undefined,
      publicTranscript: [],
      privateTranscriptOutputs: []
    };
    state_0.data = new __compactRuntime.ChargedState(context.currentQueryContext.state.state);
    return {
      currentContractState: state_0,
      currentPrivateState: context.currentPrivateState,
      currentZswapLocalState: context.currentZswapLocalState
    }
  }
  _some_0(value_0) { return { is_some: true, value: value_0 }; }
  _none_0() {
    return { is_some: false,
             value:
               { nonce: new Uint8Array(32), color: new Uint8Array(32), value: 0n } };
  }
  _left_0(value_0) {
    return { is_left: true, left: value_0, right: { bytes: new Uint8Array(32) } };
  }
  _left_1(value_0) {
    return { is_left: true, left: value_0, right: new Uint8Array(32) };
  }
  _left_2(value_0) {
    return { is_left: true, left: value_0, right: { bytes: new Uint8Array(32) } };
  }
  _right_0(value_0) {
    return { is_left: false, left: { bytes: new Uint8Array(32) }, right: value_0 };
  }
  _right_1(value_0) {
    return { is_left: false, left: { bytes: new Uint8Array(32) }, right: value_0 };
  }
  _tokenType_0(domain_sep_0, contractAddress_0) {
    return this._persistentCommit_0([domain_sep_0, contractAddress_0.bytes],
                                    new Uint8Array([109, 105, 100, 110, 105, 103, 104, 116, 58, 100, 101, 114, 105, 118, 101, 95, 116, 111, 107, 101, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
  }
  _mintShieldedToken_0(context,
                       partialProofData,
                       domain_sep_0,
                       value_0,
                       nonce_0,
                       recipient_0)
  {
    const coin_0 = { nonce: nonce_0,
                     color:
                       this._tokenType_0(domain_sep_0,
                                         _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                   partialProofData,
                                                                                                   [
                                                                                                    { dup: { n: 2 } },
                                                                                                    { idx: { cached: true,
                                                                                                             pushPath: false,
                                                                                                             path: [
                                                                                                                    { tag: 'value',
                                                                                                                      value: { value: _descriptor_19.toValue(0n),
                                                                                                                               alignment: _descriptor_19.alignment() } }] } },
                                                                                                    { popeq: { cached: true,
                                                                                                               result: undefined } }]).value)),
                     value: value_0 };
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(4n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(domain_sep_0),
                                                                                              alignment: _descriptor_0.alignment() }).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_8.toValue(value_0),
                                                                                              alignment: _descriptor_8.alignment() }).encode() } },
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
    this._createZswapOutput_0(context, partialProofData, coin_0, recipient_0);
    const cm_0 = this._coinCommitment_0(coin_0, recipient_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(2n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(cm_0),
                                                                                              alignment: _descriptor_0.alignment() }).encode() } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newNull().encode() } },
                                       { ins: { cached: true, n: 2 } },
                                       { swap: { n: 0 } }]);
    if (!recipient_0.is_left
        &&
        this._equal_0(recipient_0.right.bytes,
                      _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                partialProofData,
                                                                                [
                                                                                 { dup: { n: 2 } },
                                                                                 { idx: { cached: true,
                                                                                          pushPath: false,
                                                                                          path: [
                                                                                                 { tag: 'value',
                                                                                                   value: { value: _descriptor_19.toValue(0n),
                                                                                                            alignment: _descriptor_19.alignment() } }] } },
                                                                                 { popeq: { cached: true,
                                                                                            result: undefined } }]).value).bytes))
    {
      __compactRuntime.queryLedgerState(context,
                                        partialProofData,
                                        [
                                         { swap: { n: 0 } },
                                         { idx: { cached: true,
                                                  pushPath: true,
                                                  path: [
                                                         { tag: 'value',
                                                           value: { value: _descriptor_19.toValue(1n),
                                                                    alignment: _descriptor_19.alignment() } }] } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(cm_0),
                                                                                                alignment: _descriptor_0.alignment() }).encode() } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newNull().encode() } },
                                         { ins: { cached: true, n: 2 } },
                                         { swap: { n: 0 } }]);
    }
    return coin_0;
  }
  _receiveShielded_0(context, partialProofData, coin_0) {
    const recipient_0 = this._right_1(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                partialProofData,
                                                                                                [
                                                                                                 { dup: { n: 2 } },
                                                                                                 { idx: { cached: true,
                                                                                                          pushPath: false,
                                                                                                          path: [
                                                                                                                 { tag: 'value',
                                                                                                                   value: { value: _descriptor_19.toValue(0n),
                                                                                                                            alignment: _descriptor_19.alignment() } }] } },
                                                                                                 { popeq: { cached: true,
                                                                                                            result: undefined } }]).value));
    this._createZswapOutput_0(context, partialProofData, coin_0, recipient_0);
    const tmp_0 = this._coinCommitment_0(coin_0, recipient_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(1n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(tmp_0),
                                                                                              alignment: _descriptor_0.alignment() }).encode() } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newNull().encode() } },
                                       { ins: { cached: true, n: 2 } },
                                       { swap: { n: 0 } }]);
    return [];
  }
  _sendShielded_0(context, partialProofData, input_0, recipient_0, value_0) {
    const selfAddr_0 = _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                 partialProofData,
                                                                                 [
                                                                                  { dup: { n: 2 } },
                                                                                  { idx: { cached: true,
                                                                                           pushPath: false,
                                                                                           path: [
                                                                                                  { tag: 'value',
                                                                                                    value: { value: _descriptor_19.toValue(0n),
                                                                                                             alignment: _descriptor_19.alignment() } }] } },
                                                                                  { popeq: { cached: true,
                                                                                             result: undefined } }]).value);
    this._createZswapInput_0(context, partialProofData, input_0);
    const tmp_0 = this._coinNullifier_0(this._downcastQualifiedCoin_0(input_0),
                                        selfAddr_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(0n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(tmp_0),
                                                                                              alignment: _descriptor_0.alignment() }).encode() } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newNull().encode() } },
                                       { ins: { cached: true, n: 2 } },
                                       { swap: { n: 0 } }]);
    let t_0;
    const change_0 = (t_0 = input_0.value,
                      (__compactRuntime.assert(t_0 >= value_0,
                                               'result of subtraction would be negative'),
                       t_0 - value_0));
    const output_0 = { nonce:
                         this._upgradeFromTransient_0(this._transientHash_0([__compactRuntime.convertBytesToField(28,
                                                                                                                  new Uint8Array([109, 105, 100, 110, 105, 103, 104, 116, 58, 107, 101, 114, 110, 101, 108, 58, 110, 111, 110, 99, 101, 95, 101, 118, 111, 108, 118, 101]),
                                                                                                                  '<standard library>'),
                                                                             this._degradeToTransient_0(input_0.nonce)])),
                       color: input_0.color,
                       value: value_0 };
    this._createZswapOutput_0(context, partialProofData, output_0, recipient_0);
    const tmp_1 = this._coinCommitment_0(output_0, recipient_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(2n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(tmp_1),
                                                                                              alignment: _descriptor_0.alignment() }).encode() } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newNull().encode() } },
                                       { ins: { cached: true, n: 2 } },
                                       { swap: { n: 0 } }]);
    if (!recipient_0.is_left
        &&
        this._equal_1(recipient_0.right.bytes, selfAddr_0.bytes))
    {
      const tmp_2 = this._coinCommitment_0(output_0, recipient_0);
      __compactRuntime.queryLedgerState(context,
                                        partialProofData,
                                        [
                                         { swap: { n: 0 } },
                                         { idx: { cached: true,
                                                  pushPath: true,
                                                  path: [
                                                         { tag: 'value',
                                                           value: { value: _descriptor_19.toValue(1n),
                                                                    alignment: _descriptor_19.alignment() } }] } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(tmp_2),
                                                                                                alignment: _descriptor_0.alignment() }).encode() } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newNull().encode() } },
                                         { ins: { cached: true, n: 2 } },
                                         { swap: { n: 0 } }]);
    }
    if (this._equal_2(change_0, 0n)) {
      return { change: this._none_0(), sent: output_0 };
    } else {
      const changeCoin_0 = { nonce:
                               this._upgradeFromTransient_0(this._transientHash_0([__compactRuntime.convertBytesToField(30,
                                                                                                                        new Uint8Array([109, 105, 100, 110, 105, 103, 104, 116, 58, 107, 101, 114, 110, 101, 108, 58, 110, 111, 110, 99, 101, 95, 101, 118, 111, 108, 118, 101, 47, 50]),
                                                                                                                        '<standard library>'),
                                                                                   this._degradeToTransient_0(input_0.nonce)])),
                             color: input_0.color,
                             value: change_0 };
      this._createZswapOutput_0(context,
                                partialProofData,
                                changeCoin_0,
                                this._right_1(selfAddr_0));
      const cm_0 = this._coinCommitment_0(changeCoin_0,
                                          this._right_1(selfAddr_0));
      __compactRuntime.queryLedgerState(context,
                                        partialProofData,
                                        [
                                         { swap: { n: 0 } },
                                         { idx: { cached: true,
                                                  pushPath: true,
                                                  path: [
                                                         { tag: 'value',
                                                           value: { value: _descriptor_19.toValue(2n),
                                                                    alignment: _descriptor_19.alignment() } }] } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(cm_0),
                                                                                                alignment: _descriptor_0.alignment() }).encode() } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newNull().encode() } },
                                         { ins: { cached: true, n: 2 } },
                                         { swap: { n: 0 } }]);
      __compactRuntime.queryLedgerState(context,
                                        partialProofData,
                                        [
                                         { swap: { n: 0 } },
                                         { idx: { cached: true,
                                                  pushPath: true,
                                                  path: [
                                                         { tag: 'value',
                                                           value: { value: _descriptor_19.toValue(1n),
                                                                    alignment: _descriptor_19.alignment() } }] } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(cm_0),
                                                                                                alignment: _descriptor_0.alignment() }).encode() } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newNull().encode() } },
                                         { ins: { cached: true, n: 2 } },
                                         { swap: { n: 0 } }]);
      return { change: this._some_0(changeCoin_0), sent: output_0 };
    }
  }
  _sendImmediateShielded_0(context, partialProofData, input_0, target_0, value_0)
  {
    return this._sendShielded_0(context,
                                partialProofData,
                                this._upcastQualifiedCoin_0(input_0),
                                target_0,
                                value_0);
  }
  _downcastQualifiedCoin_0(coin_0) {
    return { nonce: coin_0.nonce, color: coin_0.color, value: coin_0.value };
  }
  _upcastQualifiedCoin_0(coin_0) {
    return { nonce: coin_0.nonce,
             color: coin_0.color,
             value: coin_0.value,
             mt_index: 0n };
  }
  _coinCommitment_0(coin_0, recipient_0) {
    return this._persistentHash_0({ domain_sep:
                                      new Uint8Array([109, 105, 100, 110, 105, 103, 104, 116, 58, 122, 115, 119, 97, 112, 45, 99, 99, 91, 118, 49, 93]),
                                    info: coin_0,
                                    dataType: recipient_0.is_left,
                                    data:
                                      recipient_0.is_left ?
                                      recipient_0.left.bytes :
                                      recipient_0.right.bytes });
  }
  _coinNullifier_0(coin_0, addr_0) {
    return this._persistentHash_0({ domain_sep:
                                      new Uint8Array([109, 105, 100, 110, 105, 103, 104, 116, 58, 122, 115, 119, 97, 112, 45, 99, 110, 91, 118, 49, 93]),
                                    info: coin_0,
                                    dataType: false,
                                    data: addr_0.bytes });
  }
  _mintUnshieldedToken_0(context,
                         partialProofData,
                         domainSep_0,
                         amount_0,
                         recipient_0)
  {
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(5n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_0.toValue(domainSep_0),
                                                                                              alignment: _descriptor_0.alignment() }).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_8.toValue(amount_0),
                                                                                              alignment: _descriptor_8.alignment() }).encode() } },
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
    const color_0 = this._tokenType_0(domainSep_0,
                                      _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                partialProofData,
                                                                                                [
                                                                                                 { dup: { n: 2 } },
                                                                                                 { idx: { cached: true,
                                                                                                          pushPath: false,
                                                                                                          path: [
                                                                                                                 { tag: 'value',
                                                                                                                   value: { value: _descriptor_19.toValue(0n),
                                                                                                                            alignment: _descriptor_19.alignment() } }] } },
                                                                                                 { popeq: { cached: true,
                                                                                                            result: undefined } }]).value));
    const tmp_0 = this._left_1(color_0);
    const tmp_1 = amount_0;
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(8n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell(__compactRuntime.alignedConcat(
                                                                                              { value: _descriptor_17.toValue(tmp_0),
                                                                                                alignment: _descriptor_17.alignment() },
                                                                                              { value: _descriptor_18.toValue(recipient_0),
                                                                                                alignment: _descriptor_18.alignment() }
                                                                                            )).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(tmp_1),
                                                                                              alignment: _descriptor_1.alignment() }).encode() } },
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
        this._equal_3(recipient_0.left.bytes,
                      _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                partialProofData,
                                                                                [
                                                                                 { dup: { n: 2 } },
                                                                                 { idx: { cached: true,
                                                                                          pushPath: false,
                                                                                          path: [
                                                                                                 { tag: 'value',
                                                                                                   value: { value: _descriptor_19.toValue(0n),
                                                                                                            alignment: _descriptor_19.alignment() } }] } },
                                                                                 { popeq: { cached: true,
                                                                                            result: undefined } }]).value).bytes))
    {
      const tmp_2 = this._left_1(color_0);
      const tmp_3 = amount_0;
      __compactRuntime.queryLedgerState(context,
                                        partialProofData,
                                        [
                                         { swap: { n: 0 } },
                                         { idx: { cached: true,
                                                  pushPath: true,
                                                  path: [
                                                         { tag: 'value',
                                                           value: { value: _descriptor_19.toValue(6n),
                                                                    alignment: _descriptor_19.alignment() } }] } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_17.toValue(tmp_2),
                                                                                                alignment: _descriptor_17.alignment() }).encode() } },
                                         { dup: { n: 1 } },
                                         { dup: { n: 1 } },
                                         'member',
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(tmp_3),
                                                                                                alignment: _descriptor_1.alignment() }).encode() } },
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
    return color_0;
  }
  _sendUnshielded_0(context, partialProofData, color_0, amount_0, recipient_0) {
    const tmp_0 = this._left_1(color_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(7n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_17.toValue(tmp_0),
                                                                                              alignment: _descriptor_17.alignment() }).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(amount_0),
                                                                                              alignment: _descriptor_1.alignment() }).encode() } },
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
    const tmp_1 = this._left_1(color_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(8n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell(__compactRuntime.alignedConcat(
                                                                                              { value: _descriptor_17.toValue(tmp_1),
                                                                                                alignment: _descriptor_17.alignment() },
                                                                                              { value: _descriptor_18.toValue(recipient_0),
                                                                                                alignment: _descriptor_18.alignment() }
                                                                                            )).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(amount_0),
                                                                                              alignment: _descriptor_1.alignment() }).encode() } },
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
                      _descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                partialProofData,
                                                                                [
                                                                                 { dup: { n: 2 } },
                                                                                 { idx: { cached: true,
                                                                                          pushPath: false,
                                                                                          path: [
                                                                                                 { tag: 'value',
                                                                                                   value: { value: _descriptor_19.toValue(0n),
                                                                                                            alignment: _descriptor_19.alignment() } }] } },
                                                                                 { popeq: { cached: true,
                                                                                            result: undefined } }]).value).bytes))
    {
      const tmp_2 = this._left_1(color_0);
      __compactRuntime.queryLedgerState(context,
                                        partialProofData,
                                        [
                                         { swap: { n: 0 } },
                                         { idx: { cached: true,
                                                  pushPath: true,
                                                  path: [
                                                         { tag: 'value',
                                                           value: { value: _descriptor_19.toValue(6n),
                                                                    alignment: _descriptor_19.alignment() } }] } },
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_17.toValue(tmp_2),
                                                                                                alignment: _descriptor_17.alignment() }).encode() } },
                                         { dup: { n: 1 } },
                                         { dup: { n: 1 } },
                                         'member',
                                         { push: { storage: false,
                                                   value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(amount_0),
                                                                                                alignment: _descriptor_1.alignment() }).encode() } },
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
  _receiveUnshielded_0(context, partialProofData, color_0, amount_0) {
    const tmp_0 = this._left_1(color_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { swap: { n: 0 } },
                                       { idx: { cached: true,
                                                pushPath: true,
                                                path: [
                                                       { tag: 'value',
                                                         value: { value: _descriptor_19.toValue(6n),
                                                                  alignment: _descriptor_19.alignment() } }] } },
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_17.toValue(tmp_0),
                                                                                              alignment: _descriptor_17.alignment() }).encode() } },
                                       { dup: { n: 1 } },
                                       { dup: { n: 1 } },
                                       'member',
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(amount_0),
                                                                                              alignment: _descriptor_1.alignment() }).encode() } },
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
    return [];
  }
  _unshieldedBalance_0(context, partialProofData, color_0) {
    const tmp_0 = this._left_1(color_0);
    return _descriptor_1.fromValue(__compactRuntime.queryLedgerState(context,
                                                                     partialProofData,
                                                                     [
                                                                      { dup: { n: 2 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_19.toValue(5n),
                                                                                                 alignment: _descriptor_19.alignment() } }] } },
                                                                      { dup: { n: 0 } },
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_17.toValue(tmp_0),
                                                                                                                             alignment: _descriptor_17.alignment() }).encode() } },
                                                                      'member',
                                                                      { branch: { skip: 3 } },
                                                                      'pop',
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(0n),
                                                                                                                             alignment: _descriptor_1.alignment() }).encode() } },
                                                                      { jmp: { skip: 1 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_17.toValue(tmp_0),
                                                                                                 alignment: _descriptor_17.alignment() } }] } },
                                                                      { popeq: { cached: true,
                                                                                 result: undefined } }]).value);
  }
  _unshieldedBalanceLt_0(context, partialProofData, color_0, amount_0) {
    const tmp_0 = this._left_1(color_0);
    return _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                     partialProofData,
                                                                     [
                                                                      { dup: { n: 2 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_19.toValue(5n),
                                                                                                 alignment: _descriptor_19.alignment() } }] } },
                                                                      { dup: { n: 0 } },
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_17.toValue(tmp_0),
                                                                                                                             alignment: _descriptor_17.alignment() }).encode() } },
                                                                      'member',
                                                                      { branch: { skip: 3 } },
                                                                      'pop',
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(0n),
                                                                                                                             alignment: _descriptor_1.alignment() }).encode() } },
                                                                      { jmp: { skip: 1 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_17.toValue(tmp_0),
                                                                                                 alignment: _descriptor_17.alignment() } }] } },
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(amount_0),
                                                                                                                             alignment: _descriptor_1.alignment() }).encode() } },
                                                                      'lt',
                                                                      { popeq: { cached: true,
                                                                                 result: undefined } }]).value);
  }
  _unshieldedBalanceGt_0(context, partialProofData, color_0, amount_0) {
    const tmp_0 = this._left_1(color_0);
    return _descriptor_4.fromValue(__compactRuntime.queryLedgerState(context,
                                                                     partialProofData,
                                                                     [
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(amount_0),
                                                                                                                             alignment: _descriptor_1.alignment() }).encode() } },
                                                                      { dup: { n: 3 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_19.toValue(5n),
                                                                                                 alignment: _descriptor_19.alignment() } }] } },
                                                                      { dup: { n: 0 } },
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_17.toValue(tmp_0),
                                                                                                                             alignment: _descriptor_17.alignment() }).encode() } },
                                                                      'member',
                                                                      { branch: { skip: 3 } },
                                                                      'pop',
                                                                      { push: { storage: false,
                                                                                value: __compactRuntime.StateValue.newCell({ value: _descriptor_1.toValue(0n),
                                                                                                                             alignment: _descriptor_1.alignment() }).encode() } },
                                                                      { jmp: { skip: 1 } },
                                                                      { idx: { cached: true,
                                                                               pushPath: false,
                                                                               path: [
                                                                                      { tag: 'value',
                                                                                        value: { value: _descriptor_17.toValue(tmp_0),
                                                                                                 alignment: _descriptor_17.alignment() } }] } },
                                                                      'lt',
                                                                      { popeq: { cached: true,
                                                                                 result: undefined } }]).value);
  }
  _transientHash_0(value_0) {
    const result_0 = __compactRuntime.transientHash(_descriptor_14, value_0);
    return result_0;
  }
  _persistentHash_0(value_0) {
    const result_0 = __compactRuntime.persistentHash(_descriptor_16, value_0);
    return result_0;
  }
  _persistentCommit_0(value_0, rand_0) {
    const result_0 = __compactRuntime.persistentCommit(_descriptor_13,
                                                       value_0,
                                                       rand_0);
    return result_0;
  }
  _degradeToTransient_0(x_0) {
    const result_0 = __compactRuntime.degradeToTransient(x_0);
    return result_0;
  }
  _upgradeFromTransient_0(x_0) {
    const result_0 = __compactRuntime.upgradeFromTransient(x_0);
    return result_0;
  }
  _createZswapInput_0(context, partialProofData, coin_0) {
    const result_0 = __compactRuntime.createZswapInput(context, coin_0);
    partialProofData.privateTranscriptOutputs.push({
      value: [],
      alignment: []
    });
    return result_0;
  }
  _createZswapOutput_0(context, partialProofData, coin_0, recipient_0) {
    const result_0 = __compactRuntime.createZswapOutput(context,
                                                        coin_0,
                                                        recipient_0);
    partialProofData.privateTranscriptOutputs.push({
      value: [],
      alignment: []
    });
    return result_0;
  }
  _mintUnshieldedToSelfTest_0(context, partialProofData, domainSep_0, amount_0)
  {
    return this._mintUnshieldedToken_0(context,
                                       partialProofData,
                                       domainSep_0,
                                       amount_0,
                                       this._left_0(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                              partialProofData,
                                                                                                              [
                                                                                                               { dup: { n: 2 } },
                                                                                                               { idx: { cached: true,
                                                                                                                        pushPath: false,
                                                                                                                        path: [
                                                                                                                               { tag: 'value',
                                                                                                                                 value: { value: _descriptor_19.toValue(0n),
                                                                                                                                          alignment: _descriptor_19.alignment() } }] } },
                                                                                                               { popeq: { cached: true,
                                                                                                                          result: undefined } }]).value)));
  }
  _mintUnshieldedToContractTest_0(context,
                                  partialProofData,
                                  domainSep_0,
                                  address_0,
                                  amount_0)
  {
    return this._mintUnshieldedToken_0(context,
                                       partialProofData,
                                       domainSep_0,
                                       amount_0,
                                       this._left_0(address_0));
  }
  _mintUnshieldedToUserTest_0(context,
                              partialProofData,
                              domainSep_0,
                              address_0,
                              amount_0)
  {
    return this._mintUnshieldedToken_0(context,
                                       partialProofData,
                                       domainSep_0,
                                       amount_0,
                                       this._right_0(address_0));
  }
  _sendUnshieldedToSelfTest_0(context, partialProofData, color_0, amount_0) {
    this._sendUnshielded_0(context,
                           partialProofData,
                           color_0,
                           amount_0,
                           this._left_0(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                  partialProofData,
                                                                                                  [
                                                                                                   { dup: { n: 2 } },
                                                                                                   { idx: { cached: true,
                                                                                                            pushPath: false,
                                                                                                            path: [
                                                                                                                   { tag: 'value',
                                                                                                                     value: { value: _descriptor_19.toValue(0n),
                                                                                                                              alignment: _descriptor_19.alignment() } }] } },
                                                                                                   { popeq: { cached: true,
                                                                                                              result: undefined } }]).value)));
    return [];
  }
  _sendUnshieldedToContractTest_0(context,
                                  partialProofData,
                                  color_0,
                                  amount_0,
                                  address_0)
  {
    this._sendUnshielded_0(context,
                           partialProofData,
                           color_0,
                           amount_0,
                           this._left_0(address_0));
    return [];
  }
  _sendUnshieldedToUserTest_0(context,
                              partialProofData,
                              color_0,
                              amount_0,
                              address_0)
  {
    this._sendUnshielded_0(context,
                           partialProofData,
                           color_0,
                           amount_0,
                           this._right_0(address_0));
    return [];
  }
  _receiveUnshieldedTest_0(context, partialProofData, color_0, amount_0) {
    this._receiveUnshielded_0(context, partialProofData, color_0, amount_0);
    return [];
  }
  _getUnshieldedBalanceTest_0(context, partialProofData, color_0) {
    return this._unshieldedBalance_0(context, partialProofData, color_0);
  }
  _getUnshieldedBalanceGtTest_0(context, partialProofData, color_0, amount_0) {
    return this._unshieldedBalanceGt_0(context,
                                       partialProofData,
                                       color_0,
                                       amount_0);
  }
  _getUnshieldedBalanceLtTest_0(context, partialProofData, color_0, amount_0) {
    return this._unshieldedBalanceLt_0(context,
                                       partialProofData,
                                       color_0,
                                       amount_0);
  }
  _mintShieldedToSelfTest_0(context,
                            partialProofData,
                            domainSep_0,
                            value_0,
                            nonce_0)
  {
    return this._mintShieldedToken_0(context,
                                     partialProofData,
                                     domainSep_0,
                                     value_0,
                                     nonce_0,
                                     this._right_1(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                             partialProofData,
                                                                                                             [
                                                                                                              { dup: { n: 2 } },
                                                                                                              { idx: { cached: true,
                                                                                                                       pushPath: false,
                                                                                                                       path: [
                                                                                                                              { tag: 'value',
                                                                                                                                value: { value: _descriptor_19.toValue(0n),
                                                                                                                                         alignment: _descriptor_19.alignment() } }] } },
                                                                                                              { popeq: { cached: true,
                                                                                                                         result: undefined } }]).value)));
  }
  _mintShieldedToContractTest_0(context,
                                partialProofData,
                                domainSep_0,
                                value_0,
                                nonce_0,
                                address_0)
  {
    return this._mintShieldedToken_0(context,
                                     partialProofData,
                                     domainSep_0,
                                     value_0,
                                     nonce_0,
                                     this._right_1(address_0));
  }
  _mintShieldedToUserTest_0(context,
                            partialProofData,
                            domainSep_0,
                            value_0,
                            nonce_0,
                            publicKey_0)
  {
    return this._mintShieldedToken_0(context,
                                     partialProofData,
                                     domainSep_0,
                                     value_0,
                                     nonce_0,
                                     this._left_2(publicKey_0));
  }
  _receiveShieldedTest_0(context, partialProofData, coin_0) {
    this._receiveShielded_0(context, partialProofData, coin_0); return [];
  }
  _sendShieldedToSelfTest_0(context, partialProofData, input_0, value_0) {
    return this._sendShielded_0(context,
                                partialProofData,
                                input_0,
                                this._right_1(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                        partialProofData,
                                                                                                        [
                                                                                                         { dup: { n: 2 } },
                                                                                                         { idx: { cached: true,
                                                                                                                  pushPath: false,
                                                                                                                  path: [
                                                                                                                         { tag: 'value',
                                                                                                                           value: { value: _descriptor_19.toValue(0n),
                                                                                                                                    alignment: _descriptor_19.alignment() } }] } },
                                                                                                         { popeq: { cached: true,
                                                                                                                    result: undefined } }]).value)),
                                value_0);
  }
  _sendShieldedToContractTest_0(context,
                                partialProofData,
                                input_0,
                                address_0,
                                value_0)
  {
    return this._sendShielded_0(context,
                                partialProofData,
                                input_0,
                                this._right_1(address_0),
                                value_0);
  }
  _sendShieldedToUserTest_0(context,
                            partialProofData,
                            input_0,
                            publicKey_0,
                            value_0)
  {
    return this._sendShielded_0(context,
                                partialProofData,
                                input_0,
                                this._left_2(publicKey_0),
                                value_0);
  }
  _sendImmediateShieldedToSelfTest_0(context, partialProofData, input_0, value_0)
  {
    return this._sendImmediateShielded_0(context,
                                         partialProofData,
                                         input_0,
                                         this._right_1(_descriptor_7.fromValue(__compactRuntime.queryLedgerState(context,
                                                                                                                 partialProofData,
                                                                                                                 [
                                                                                                                  { dup: { n: 2 } },
                                                                                                                  { idx: { cached: true,
                                                                                                                           pushPath: false,
                                                                                                                           path: [
                                                                                                                                  { tag: 'value',
                                                                                                                                    value: { value: _descriptor_19.toValue(0n),
                                                                                                                                             alignment: _descriptor_19.alignment() } }] } },
                                                                                                                  { popeq: { cached: true,
                                                                                                                             result: undefined } }]).value)),
                                         value_0);
  }
  _sendImmediateShieldedToContractTest_0(context,
                                         partialProofData,
                                         input_0,
                                         address_0,
                                         value_0)
  {
    return this._sendImmediateShielded_0(context,
                                         partialProofData,
                                         input_0,
                                         this._right_1(address_0),
                                         value_0);
  }
  _sendImmediateShieldedToUserTest_0(context,
                                     partialProofData,
                                     input_0,
                                     publicKey_0,
                                     value_0)
  {
    return this._sendImmediateShielded_0(context,
                                         partialProofData,
                                         input_0,
                                         this._left_2(publicKey_0),
                                         value_0);
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
    if (x0 !== y0) { return false; }
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
}
export function ledger(stateOrChargedState) {
  const state = stateOrChargedState instanceof __compactRuntime.StateValue ? stateOrChargedState : stateOrChargedState.state;
  const chargedState = stateOrChargedState instanceof __compactRuntime.StateValue ? new __compactRuntime.ChargedState(stateOrChargedState) : stateOrChargedState;
  const context = {
    currentQueryContext: new __compactRuntime.QueryContext(chargedState, __compactRuntime.dummyContractAddress()),
    costModel: __compactRuntime.CostModel.initialCostModel()
  };
  const partialProofData = {
    input: { value: [], alignment: [] },
    output: undefined,
    publicTranscript: [],
    privateTranscriptOutputs: []
  };
  return {
  };
}
const _emptyContext = {
  currentQueryContext: new __compactRuntime.QueryContext(new __compactRuntime.ContractState().data, __compactRuntime.dummyContractAddress())
};
const _dummyContract = new Contract({ });
export const pureCircuits = {};
export const contractReferenceLocations =
  { tag: 'publicLedgerArray', indices: { } };
//# sourceMappingURL=index.js.map
