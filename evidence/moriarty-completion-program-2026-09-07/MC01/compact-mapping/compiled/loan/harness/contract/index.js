import * as __compactRuntime from '@midnight-ntwrk/compact-runtime';
__compactRuntime.checkRuntimeVersion('0.16.0');

const _descriptor_0 = new __compactRuntime.CompactTypeUnsignedInteger(340282366920938463463374607431768211455n, 16);

class _KernelState_0 {
  alignment() {
    return _descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment()))))))));
  }
  fromValue(value_0) {
    return {
      f0: _descriptor_0.fromValue(value_0),
      f1: _descriptor_0.fromValue(value_0),
      f2: _descriptor_0.fromValue(value_0),
      f3: _descriptor_0.fromValue(value_0),
      f4: _descriptor_0.fromValue(value_0),
      f5: _descriptor_0.fromValue(value_0),
      f6: _descriptor_0.fromValue(value_0),
      f7: _descriptor_0.fromValue(value_0),
      f8: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.f0).concat(_descriptor_0.toValue(value_0.f1).concat(_descriptor_0.toValue(value_0.f2).concat(_descriptor_0.toValue(value_0.f3).concat(_descriptor_0.toValue(value_0.f4).concat(_descriptor_0.toValue(value_0.f5).concat(_descriptor_0.toValue(value_0.f6).concat(_descriptor_0.toValue(value_0.f7).concat(_descriptor_0.toValue(value_0.f8)))))))));
  }
}

const _descriptor_1 = new _KernelState_0();

const _descriptor_2 = new __compactRuntime.CompactTypeUnsignedInteger(4294967295n, 4);

class _Effect1_0_0 {
  alignment() {
    return _descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_0.alignment())));
  }
  fromValue(value_0) {
    return {
      v0: _descriptor_2.fromValue(value_0),
      v1: _descriptor_2.fromValue(value_0),
      v2: _descriptor_2.fromValue(value_0),
      v3: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_2.toValue(value_0.v0).concat(_descriptor_2.toValue(value_0.v1).concat(_descriptor_2.toValue(value_0.v2).concat(_descriptor_0.toValue(value_0.v3))));
  }
}

const _descriptor_3 = new _Effect1_0_0();

class _Effect1_1_0 {
  alignment() {
    return _descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_0.alignment().concat(_descriptor_2.alignment())))));
  }
  fromValue(value_0) {
    return {
      v0: _descriptor_2.fromValue(value_0),
      v1: _descriptor_2.fromValue(value_0),
      v2: _descriptor_2.fromValue(value_0),
      v3: _descriptor_2.fromValue(value_0),
      v4: _descriptor_0.fromValue(value_0),
      v5: _descriptor_2.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_2.toValue(value_0.v0).concat(_descriptor_2.toValue(value_0.v1).concat(_descriptor_2.toValue(value_0.v2).concat(_descriptor_2.toValue(value_0.v3).concat(_descriptor_0.toValue(value_0.v4).concat(_descriptor_2.toValue(value_0.v5))))));
  }
}

const _descriptor_4 = new _Effect1_1_0();

class _Effect1_2_0 {
  alignment() {
    return _descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_0.alignment().concat(_descriptor_2.alignment())))));
  }
  fromValue(value_0) {
    return {
      v0: _descriptor_2.fromValue(value_0),
      v1: _descriptor_2.fromValue(value_0),
      v2: _descriptor_2.fromValue(value_0),
      v3: _descriptor_2.fromValue(value_0),
      v4: _descriptor_0.fromValue(value_0),
      v5: _descriptor_2.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_2.toValue(value_0.v0).concat(_descriptor_2.toValue(value_0.v1).concat(_descriptor_2.toValue(value_0.v2).concat(_descriptor_2.toValue(value_0.v3).concat(_descriptor_0.toValue(value_0.v4).concat(_descriptor_2.toValue(value_0.v5))))));
  }
}

const _descriptor_5 = new _Effect1_2_0();

class _Result1_0 {
  alignment() {
    return _descriptor_1.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_3.alignment().concat(_descriptor_4.alignment().concat(_descriptor_5.alignment())))));
  }
  fromValue(value_0) {
    return {
      after: _descriptor_1.fromValue(value_0),
      remaining: _descriptor_0.fromValue(value_0),
      revision: _descriptor_0.fromValue(value_0),
      effect0: _descriptor_3.fromValue(value_0),
      effect1: _descriptor_4.fromValue(value_0),
      effect2: _descriptor_5.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_1.toValue(value_0.after).concat(_descriptor_0.toValue(value_0.remaining).concat(_descriptor_0.toValue(value_0.revision).concat(_descriptor_3.toValue(value_0.effect0).concat(_descriptor_4.toValue(value_0.effect1).concat(_descriptor_5.toValue(value_0.effect2))))));
  }
}

const _descriptor_6 = new _Result1_0();

class _Arguments1_0 {
  alignment() {
    return _descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_0.alignment()));
  }
  fromValue(value_0) {
    return {
      a0: _descriptor_2.fromValue(value_0),
      a1: _descriptor_2.fromValue(value_0),
      a2: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_2.toValue(value_0.a0).concat(_descriptor_2.toValue(value_0.a1).concat(_descriptor_0.toValue(value_0.a2)));
  }
}

const _descriptor_7 = new _Arguments1_0();

class _KernelObservations_0 {
  alignment() {
    return _descriptor_0.alignment();
  }
  fromValue(value_0) {
    return {
      o0: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.o0);
  }
}

const _descriptor_8 = new _KernelObservations_0();

const _descriptor_9 = new __compactRuntime.CompactTypeUnsignedInteger(1n, 1);

class _Hints1_0 {
  alignment() {
    return _descriptor_9.alignment();
  }
  fromValue(value_0) {
    return {
      unused: _descriptor_9.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_9.toValue(value_0.unused);
  }
}

const _descriptor_10 = new _Hints1_0();

class _Effect0_0_0 {
  alignment() {
    return _descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_0.alignment()))));
  }
  fromValue(value_0) {
    return {
      v0: _descriptor_2.fromValue(value_0),
      v1: _descriptor_2.fromValue(value_0),
      v2: _descriptor_2.fromValue(value_0),
      v3: _descriptor_2.fromValue(value_0),
      v4: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_2.toValue(value_0.v0).concat(_descriptor_2.toValue(value_0.v1).concat(_descriptor_2.toValue(value_0.v2).concat(_descriptor_2.toValue(value_0.v3).concat(_descriptor_0.toValue(value_0.v4)))));
  }
}

const _descriptor_11 = new _Effect0_0_0();

class _Effect0_1_0 {
  alignment() {
    return _descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_2.alignment().concat(_descriptor_0.alignment()))));
  }
  fromValue(value_0) {
    return {
      v0: _descriptor_2.fromValue(value_0),
      v1: _descriptor_2.fromValue(value_0),
      v2: _descriptor_2.fromValue(value_0),
      v3: _descriptor_2.fromValue(value_0),
      v4: _descriptor_0.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_2.toValue(value_0.v0).concat(_descriptor_2.toValue(value_0.v1).concat(_descriptor_2.toValue(value_0.v2).concat(_descriptor_2.toValue(value_0.v3).concat(_descriptor_0.toValue(value_0.v4)))));
  }
}

const _descriptor_12 = new _Effect0_1_0();

class _Result0_0 {
  alignment() {
    return _descriptor_1.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_11.alignment().concat(_descriptor_12.alignment()))));
  }
  fromValue(value_0) {
    return {
      after: _descriptor_1.fromValue(value_0),
      remaining: _descriptor_0.fromValue(value_0),
      revision: _descriptor_0.fromValue(value_0),
      effect0: _descriptor_11.fromValue(value_0),
      effect1: _descriptor_12.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_1.toValue(value_0.after).concat(_descriptor_0.toValue(value_0.remaining).concat(_descriptor_0.toValue(value_0.revision).concat(_descriptor_11.toValue(value_0.effect0).concat(_descriptor_12.toValue(value_0.effect1)))));
  }
}

const _descriptor_13 = new _Result0_0();

class _Arguments0_0 {
  alignment() {
    return _descriptor_2.alignment();
  }
  fromValue(value_0) {
    return {
      a0: _descriptor_2.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_2.toValue(value_0.a0);
  }
}

const _descriptor_14 = new _Arguments0_0();

const _descriptor_15 = new __compactRuntime.CompactTypeUnsignedInteger(18446744073709551615n, 8);

class _MulHint_0 {
  alignment() {
    return _descriptor_15.alignment().concat(_descriptor_15.alignment().concat(_descriptor_15.alignment().concat(_descriptor_15.alignment().concat(_descriptor_15.alignment().concat(_descriptor_15.alignment())))));
  }
  fromValue(value_0) {
    return {
      aLo: _descriptor_15.fromValue(value_0),
      aHi: _descriptor_15.fromValue(value_0),
      bLo: _descriptor_15.fromValue(value_0),
      bHi: _descriptor_15.fromValue(value_0),
      lo: _descriptor_15.fromValue(value_0),
      carry: _descriptor_15.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_15.toValue(value_0.aLo).concat(_descriptor_15.toValue(value_0.aHi).concat(_descriptor_15.toValue(value_0.bLo).concat(_descriptor_15.toValue(value_0.bHi).concat(_descriptor_15.toValue(value_0.lo).concat(_descriptor_15.toValue(value_0.carry))))));
  }
}

const _descriptor_16 = new _MulHint_0();

class _DivisionHint_0 {
  alignment() {
    return _descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_16.alignment()));
  }
  fromValue(value_0) {
    return {
      q: _descriptor_0.fromValue(value_0),
      r: _descriptor_0.fromValue(value_0),
      product: _descriptor_16.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_0.toValue(value_0.q).concat(_descriptor_0.toValue(value_0.r).concat(_descriptor_16.toValue(value_0.product)));
  }
}

const _descriptor_17 = new _DivisionHint_0();

class _Hints0_0 {
  alignment() {
    return _descriptor_16.alignment().concat(_descriptor_16.alignment().concat(_descriptor_16.alignment().concat(_descriptor_17.alignment())));
  }
  fromValue(value_0) {
    return {
      h0: _descriptor_16.fromValue(value_0),
      h1: _descriptor_16.fromValue(value_0),
      h2: _descriptor_16.fromValue(value_0),
      h3: _descriptor_17.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_16.toValue(value_0.h0).concat(_descriptor_16.toValue(value_0.h1).concat(_descriptor_16.toValue(value_0.h2).concat(_descriptor_17.toValue(value_0.h3))));
  }
}

const _descriptor_18 = new _Hints0_0();

const _descriptor_19 = __compactRuntime.CompactTypeBoolean;

const _descriptor_20 = new __compactRuntime.CompactTypeBytes(32);

class _Either_0 {
  alignment() {
    return _descriptor_19.alignment().concat(_descriptor_20.alignment().concat(_descriptor_20.alignment()));
  }
  fromValue(value_0) {
    return {
      is_left: _descriptor_19.fromValue(value_0),
      left: _descriptor_20.fromValue(value_0),
      right: _descriptor_20.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_19.toValue(value_0.is_left).concat(_descriptor_20.toValue(value_0.left).concat(_descriptor_20.toValue(value_0.right)));
  }
}

const _descriptor_21 = new _Either_0();

class _ContractAddress_0 {
  alignment() {
    return _descriptor_20.alignment();
  }
  fromValue(value_0) {
    return {
      bytes: _descriptor_20.fromValue(value_0)
    }
  }
  toValue(value_0) {
    return _descriptor_20.toValue(value_0.bytes);
  }
}

const _descriptor_22 = new _ContractAddress_0();

const _descriptor_23 = new __compactRuntime.CompactTypeUnsignedInteger(255n, 1);

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
      checkedAdd(context, ...args_1) {
        return { result: pureCircuits.checkedAdd(...args_1), context };
      },
      checkedSub(context, ...args_1) {
        return { result: pureCircuits.checkedSub(...args_1), context };
      },
      checkedMul(context, ...args_1) {
        return { result: pureCircuits.checkedMul(...args_1), context };
      },
      checkedDiv(context, ...args_1) {
        return { result: pureCircuits.checkedDiv(...args_1), context };
      },
      transition0(context, ...args_1) {
        return { result: pureCircuits.transition0(...args_1), context };
      },
      transition1(context, ...args_1) {
        return { result: pureCircuits.transition1(...args_1), context };
      },
      record0: (...args_1) => {
        if (args_1.length !== 7) {
          throw new __compactRuntime.CompactError(`record0: expected 7 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const before_0 = args_1[1];
        const args_2 = args_1[2];
        const observations_0 = args_1[3];
        const remaining_0 = args_1[4];
        const revision_0 = args_1[5];
        const hints_0 = args_1[6];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('record0',
                                     'argument 1 (as invoked from Typescript)',
                                     'harness.compact line 4 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(before_0) === 'object' && typeof(before_0.f0) === 'bigint' && before_0.f0 >= 0n && before_0.f0 <= 340282366920938463463374607431768211455n && typeof(before_0.f1) === 'bigint' && before_0.f1 >= 0n && before_0.f1 <= 340282366920938463463374607431768211455n && typeof(before_0.f2) === 'bigint' && before_0.f2 >= 0n && before_0.f2 <= 340282366920938463463374607431768211455n && typeof(before_0.f3) === 'bigint' && before_0.f3 >= 0n && before_0.f3 <= 340282366920938463463374607431768211455n && typeof(before_0.f4) === 'bigint' && before_0.f4 >= 0n && before_0.f4 <= 340282366920938463463374607431768211455n && typeof(before_0.f5) === 'bigint' && before_0.f5 >= 0n && before_0.f5 <= 340282366920938463463374607431768211455n && typeof(before_0.f6) === 'bigint' && before_0.f6 >= 0n && before_0.f6 <= 340282366920938463463374607431768211455n && typeof(before_0.f7) === 'bigint' && before_0.f7 >= 0n && before_0.f7 <= 340282366920938463463374607431768211455n && typeof(before_0.f8) === 'bigint' && before_0.f8 >= 0n && before_0.f8 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record0',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'harness.compact line 4 char 1',
                                     'struct KernelState<f0: Uint<0..340282366920938463463374607431768211456>, f1: Uint<0..340282366920938463463374607431768211456>, f2: Uint<0..340282366920938463463374607431768211456>, f3: Uint<0..340282366920938463463374607431768211456>, f4: Uint<0..340282366920938463463374607431768211456>, f5: Uint<0..340282366920938463463374607431768211456>, f6: Uint<0..340282366920938463463374607431768211456>, f7: Uint<0..340282366920938463463374607431768211456>, f8: Uint<0..340282366920938463463374607431768211456>>',
                                     before_0)
        }
        if (!(typeof(args_2) === 'object' && typeof(args_2.a0) === 'bigint' && args_2.a0 >= 0n && args_2.a0 <= 4294967295n)) {
          __compactRuntime.typeError('record0',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'harness.compact line 4 char 1',
                                     'struct Arguments0<a0: Uint<0..4294967296>>',
                                     args_2)
        }
        if (!(typeof(observations_0) === 'object' && typeof(observations_0.o0) === 'bigint' && observations_0.o0 >= 0n && observations_0.o0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record0',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'harness.compact line 4 char 1',
                                     'struct KernelObservations<o0: Uint<0..340282366920938463463374607431768211456>>',
                                     observations_0)
        }
        if (!(typeof(remaining_0) === 'bigint' && remaining_0 >= 0n && remaining_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record0',
                                     'argument 4 (argument 5 as invoked from Typescript)',
                                     'harness.compact line 4 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     remaining_0)
        }
        if (!(typeof(revision_0) === 'bigint' && revision_0 >= 0n && revision_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record0',
                                     'argument 5 (argument 6 as invoked from Typescript)',
                                     'harness.compact line 4 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     revision_0)
        }
        if (!(typeof(hints_0) === 'object' && typeof(hints_0.h0) === 'object' && typeof(hints_0.h0.aLo) === 'bigint' && hints_0.h0.aLo >= 0n && hints_0.h0.aLo <= 18446744073709551615n && typeof(hints_0.h0.aHi) === 'bigint' && hints_0.h0.aHi >= 0n && hints_0.h0.aHi <= 18446744073709551615n && typeof(hints_0.h0.bLo) === 'bigint' && hints_0.h0.bLo >= 0n && hints_0.h0.bLo <= 18446744073709551615n && typeof(hints_0.h0.bHi) === 'bigint' && hints_0.h0.bHi >= 0n && hints_0.h0.bHi <= 18446744073709551615n && typeof(hints_0.h0.lo) === 'bigint' && hints_0.h0.lo >= 0n && hints_0.h0.lo <= 18446744073709551615n && typeof(hints_0.h0.carry) === 'bigint' && hints_0.h0.carry >= 0n && hints_0.h0.carry <= 18446744073709551615n && typeof(hints_0.h1) === 'object' && typeof(hints_0.h1.aLo) === 'bigint' && hints_0.h1.aLo >= 0n && hints_0.h1.aLo <= 18446744073709551615n && typeof(hints_0.h1.aHi) === 'bigint' && hints_0.h1.aHi >= 0n && hints_0.h1.aHi <= 18446744073709551615n && typeof(hints_0.h1.bLo) === 'bigint' && hints_0.h1.bLo >= 0n && hints_0.h1.bLo <= 18446744073709551615n && typeof(hints_0.h1.bHi) === 'bigint' && hints_0.h1.bHi >= 0n && hints_0.h1.bHi <= 18446744073709551615n && typeof(hints_0.h1.lo) === 'bigint' && hints_0.h1.lo >= 0n && hints_0.h1.lo <= 18446744073709551615n && typeof(hints_0.h1.carry) === 'bigint' && hints_0.h1.carry >= 0n && hints_0.h1.carry <= 18446744073709551615n && typeof(hints_0.h2) === 'object' && typeof(hints_0.h2.aLo) === 'bigint' && hints_0.h2.aLo >= 0n && hints_0.h2.aLo <= 18446744073709551615n && typeof(hints_0.h2.aHi) === 'bigint' && hints_0.h2.aHi >= 0n && hints_0.h2.aHi <= 18446744073709551615n && typeof(hints_0.h2.bLo) === 'bigint' && hints_0.h2.bLo >= 0n && hints_0.h2.bLo <= 18446744073709551615n && typeof(hints_0.h2.bHi) === 'bigint' && hints_0.h2.bHi >= 0n && hints_0.h2.bHi <= 18446744073709551615n && typeof(hints_0.h2.lo) === 'bigint' && hints_0.h2.lo >= 0n && hints_0.h2.lo <= 18446744073709551615n && typeof(hints_0.h2.carry) === 'bigint' && hints_0.h2.carry >= 0n && hints_0.h2.carry <= 18446744073709551615n && typeof(hints_0.h3) === 'object' && typeof(hints_0.h3.q) === 'bigint' && hints_0.h3.q >= 0n && hints_0.h3.q <= 340282366920938463463374607431768211455n && typeof(hints_0.h3.r) === 'bigint' && hints_0.h3.r >= 0n && hints_0.h3.r <= 340282366920938463463374607431768211455n && typeof(hints_0.h3.product) === 'object' && typeof(hints_0.h3.product.aLo) === 'bigint' && hints_0.h3.product.aLo >= 0n && hints_0.h3.product.aLo <= 18446744073709551615n && typeof(hints_0.h3.product.aHi) === 'bigint' && hints_0.h3.product.aHi >= 0n && hints_0.h3.product.aHi <= 18446744073709551615n && typeof(hints_0.h3.product.bLo) === 'bigint' && hints_0.h3.product.bLo >= 0n && hints_0.h3.product.bLo <= 18446744073709551615n && typeof(hints_0.h3.product.bHi) === 'bigint' && hints_0.h3.product.bHi >= 0n && hints_0.h3.product.bHi <= 18446744073709551615n && typeof(hints_0.h3.product.lo) === 'bigint' && hints_0.h3.product.lo >= 0n && hints_0.h3.product.lo <= 18446744073709551615n && typeof(hints_0.h3.product.carry) === 'bigint' && hints_0.h3.product.carry >= 0n && hints_0.h3.product.carry <= 18446744073709551615n)) {
          __compactRuntime.typeError('record0',
                                     'argument 6 (argument 7 as invoked from Typescript)',
                                     'harness.compact line 4 char 1',
                                     'struct Hints0<h0: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>, h1: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>, h2: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>, h3: struct DivisionHint<q: Uint<0..340282366920938463463374607431768211456>, r: Uint<0..340282366920938463463374607431768211456>, product: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>>>',
                                     hints_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_1.toValue(before_0).concat(_descriptor_14.toValue(args_2).concat(_descriptor_8.toValue(observations_0).concat(_descriptor_0.toValue(remaining_0).concat(_descriptor_0.toValue(revision_0).concat(_descriptor_18.toValue(hints_0)))))),
            alignment: _descriptor_1.alignment().concat(_descriptor_14.alignment().concat(_descriptor_8.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_18.alignment())))))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._record0_0(context,
                                         partialProofData,
                                         before_0,
                                         args_2,
                                         observations_0,
                                         remaining_0,
                                         revision_0,
                                         hints_0);
        partialProofData.output = { value: [], alignment: [] };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      },
      record1: (...args_1) => {
        if (args_1.length !== 7) {
          throw new __compactRuntime.CompactError(`record1: expected 7 arguments (as invoked from Typescript), received ${args_1.length}`);
        }
        const contextOrig_0 = args_1[0];
        const before_0 = args_1[1];
        const args_2 = args_1[2];
        const observations_0 = args_1[3];
        const remaining_0 = args_1[4];
        const revision_0 = args_1[5];
        const hints_0 = args_1[6];
        if (!(typeof(contextOrig_0) === 'object' && contextOrig_0.currentQueryContext != undefined)) {
          __compactRuntime.typeError('record1',
                                     'argument 1 (as invoked from Typescript)',
                                     'harness.compact line 6 char 1',
                                     'CircuitContext',
                                     contextOrig_0)
        }
        if (!(typeof(before_0) === 'object' && typeof(before_0.f0) === 'bigint' && before_0.f0 >= 0n && before_0.f0 <= 340282366920938463463374607431768211455n && typeof(before_0.f1) === 'bigint' && before_0.f1 >= 0n && before_0.f1 <= 340282366920938463463374607431768211455n && typeof(before_0.f2) === 'bigint' && before_0.f2 >= 0n && before_0.f2 <= 340282366920938463463374607431768211455n && typeof(before_0.f3) === 'bigint' && before_0.f3 >= 0n && before_0.f3 <= 340282366920938463463374607431768211455n && typeof(before_0.f4) === 'bigint' && before_0.f4 >= 0n && before_0.f4 <= 340282366920938463463374607431768211455n && typeof(before_0.f5) === 'bigint' && before_0.f5 >= 0n && before_0.f5 <= 340282366920938463463374607431768211455n && typeof(before_0.f6) === 'bigint' && before_0.f6 >= 0n && before_0.f6 <= 340282366920938463463374607431768211455n && typeof(before_0.f7) === 'bigint' && before_0.f7 >= 0n && before_0.f7 <= 340282366920938463463374607431768211455n && typeof(before_0.f8) === 'bigint' && before_0.f8 >= 0n && before_0.f8 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record1',
                                     'argument 1 (argument 2 as invoked from Typescript)',
                                     'harness.compact line 6 char 1',
                                     'struct KernelState<f0: Uint<0..340282366920938463463374607431768211456>, f1: Uint<0..340282366920938463463374607431768211456>, f2: Uint<0..340282366920938463463374607431768211456>, f3: Uint<0..340282366920938463463374607431768211456>, f4: Uint<0..340282366920938463463374607431768211456>, f5: Uint<0..340282366920938463463374607431768211456>, f6: Uint<0..340282366920938463463374607431768211456>, f7: Uint<0..340282366920938463463374607431768211456>, f8: Uint<0..340282366920938463463374607431768211456>>',
                                     before_0)
        }
        if (!(typeof(args_2) === 'object' && typeof(args_2.a0) === 'bigint' && args_2.a0 >= 0n && args_2.a0 <= 4294967295n && typeof(args_2.a1) === 'bigint' && args_2.a1 >= 0n && args_2.a1 <= 4294967295n && typeof(args_2.a2) === 'bigint' && args_2.a2 >= 0n && args_2.a2 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record1',
                                     'argument 2 (argument 3 as invoked from Typescript)',
                                     'harness.compact line 6 char 1',
                                     'struct Arguments1<a0: Uint<0..4294967296>, a1: Uint<0..4294967296>, a2: Uint<0..340282366920938463463374607431768211456>>',
                                     args_2)
        }
        if (!(typeof(observations_0) === 'object' && typeof(observations_0.o0) === 'bigint' && observations_0.o0 >= 0n && observations_0.o0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record1',
                                     'argument 3 (argument 4 as invoked from Typescript)',
                                     'harness.compact line 6 char 1',
                                     'struct KernelObservations<o0: Uint<0..340282366920938463463374607431768211456>>',
                                     observations_0)
        }
        if (!(typeof(remaining_0) === 'bigint' && remaining_0 >= 0n && remaining_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record1',
                                     'argument 4 (argument 5 as invoked from Typescript)',
                                     'harness.compact line 6 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     remaining_0)
        }
        if (!(typeof(revision_0) === 'bigint' && revision_0 >= 0n && revision_0 <= 340282366920938463463374607431768211455n)) {
          __compactRuntime.typeError('record1',
                                     'argument 5 (argument 6 as invoked from Typescript)',
                                     'harness.compact line 6 char 1',
                                     'Uint<0..340282366920938463463374607431768211456>',
                                     revision_0)
        }
        if (!(typeof(hints_0) === 'object' && typeof(hints_0.unused) === 'bigint' && hints_0.unused >= 0n && hints_0.unused <= 1n)) {
          __compactRuntime.typeError('record1',
                                     'argument 6 (argument 7 as invoked from Typescript)',
                                     'harness.compact line 6 char 1',
                                     'struct Hints1<unused: Uint<0..2>>',
                                     hints_0)
        }
        const context = { ...contextOrig_0, gasCost: __compactRuntime.emptyRunningCost() };
        const partialProofData = {
          input: {
            value: _descriptor_1.toValue(before_0).concat(_descriptor_7.toValue(args_2).concat(_descriptor_8.toValue(observations_0).concat(_descriptor_0.toValue(remaining_0).concat(_descriptor_0.toValue(revision_0).concat(_descriptor_10.toValue(hints_0)))))),
            alignment: _descriptor_1.alignment().concat(_descriptor_7.alignment().concat(_descriptor_8.alignment().concat(_descriptor_0.alignment().concat(_descriptor_0.alignment().concat(_descriptor_10.alignment())))))
          },
          output: undefined,
          publicTranscript: [],
          privateTranscriptOutputs: []
        };
        const result_0 = this._record1_0(context,
                                         partialProofData,
                                         before_0,
                                         args_2,
                                         observations_0,
                                         remaining_0,
                                         revision_0,
                                         hints_0);
        partialProofData.output = { value: [], alignment: [] };
        return { result: result_0, context: context, proofData: partialProofData, gasCost: context.gasCost };
      }
    };
    this.impureCircuits = {
      record0: this.circuits.record0,
      record1: this.circuits.record1
    };
    this.provableCircuits = {
      record0: this.circuits.record0,
      record1: this.circuits.record1
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
    stateValue_0 = stateValue_0.arrayPush(__compactRuntime.StateValue.newNull());
    stateValue_0 = stateValue_0.arrayPush(__compactRuntime.StateValue.newNull());
    state_0.data = new __compactRuntime.ChargedState(stateValue_0);
    state_0.setOperation('record0', new __compactRuntime.ContractOperation());
    state_0.setOperation('record1', new __compactRuntime.ContractOperation());
    const context = __compactRuntime.createCircuitContext(__compactRuntime.dummyContractAddress(), constructorContext_0.initialZswapLocalState.coinPublicKey, state_0.data, constructorContext_0.initialPrivateState);
    const partialProofData = {
      input: { value: [], alignment: [] },
      output: undefined,
      publicTranscript: [],
      privateTranscriptOutputs: []
    };
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_23.toValue(0n),
                                                                                              alignment: _descriptor_23.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_13.toValue({ after: { f0: 0n, f1: 0n, f2: 0n, f3: 0n, f4: 0n, f5: 0n, f6: 0n, f7: 0n, f8: 0n }, remaining: 0n, revision: 0n, effect0: { v0: 0n, v1: 0n, v2: 0n, v3: 0n, v4: 0n }, effect1: { v0: 0n, v1: 0n, v2: 0n, v3: 0n, v4: 0n } }),
                                                                                              alignment: _descriptor_13.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } }]);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_23.toValue(1n),
                                                                                              alignment: _descriptor_23.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_6.toValue({ after: { f0: 0n, f1: 0n, f2: 0n, f3: 0n, f4: 0n, f5: 0n, f6: 0n, f7: 0n, f8: 0n }, remaining: 0n, revision: 0n, effect0: { v0: 0n, v1: 0n, v2: 0n, v3: 0n }, effect1: { v0: 0n, v1: 0n, v2: 0n, v3: 0n, v4: 0n, v5: 0n }, effect2: { v0: 0n, v1: 0n, v2: 0n, v3: 0n, v4: 0n, v5: 0n } }),
                                                                                              alignment: _descriptor_6.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } }]);
    state_0.data = new __compactRuntime.ChargedState(context.currentQueryContext.state.state);
    return {
      currentContractState: state_0,
      currentPrivateState: context.currentPrivateState,
      currentZswapLocalState: context.currentZswapLocalState
    }
  }
  _checkedAdd_0(a_0, b_0) {
    return ((t1) => {
             if (t1 > 340282366920938463463374607431768211455n) {
               throw new __compactRuntime.CompactError('arithmetic.compact line 14 char 10: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
             }
             return t1;
           })(a_0 + b_0);
  }
  _checkedSub_0(a_0, b_0) {
    __compactRuntime.assert(a_0 >= b_0, 'UINT_UNDERFLOW');
    __compactRuntime.assert(a_0 >= b_0,
                            'result of subtraction would be negative');
    return a_0 - b_0;
  }
  _checkedMul_0(a_0, b_0, h_0) {
    __compactRuntime.assert(this._equal_0(a_0,
                                          h_0.aHi * 18446744073709551616n
                                          +
                                          h_0.aLo),
                            'MUL_A_SPLIT');
    __compactRuntime.assert(this._equal_1(b_0,
                                          h_0.bHi * 18446744073709551616n
                                          +
                                          h_0.bLo),
                            'MUL_B_SPLIT');
    __compactRuntime.assert(this._equal_2(h_0.aLo * h_0.bLo,
                                          h_0.carry * 18446744073709551616n
                                          +
                                          h_0.lo),
                            'MUL_PRODUCT_SPLIT');
    __compactRuntime.assert(this._equal_3(h_0.aHi * h_0.bHi, 0n),
                            'UINT_OVERFLOW_HIGH');
    const middle_0 = h_0.aLo * h_0.bHi + h_0.aHi * h_0.bLo + h_0.carry;
    __compactRuntime.assert(middle_0 < 18446744073709551616n,
                            'UINT_OVERFLOW_MIDDLE');
    return ((t1) => {
             if (t1 > 340282366920938463463374607431768211455n) {
               throw new __compactRuntime.CompactError('arithmetic.compact line 29 char 10: cast from Field or Uint value to smaller Uint value failed: ' + t1 + ' is greater than 340282366920938463463374607431768211455');
             }
             return t1;
           })(middle_0 * 18446744073709551616n + h_0.lo);
  }
  _checkedDiv_0(n_0, d_0, q_0, r_0, h_0) {
    __compactRuntime.assert(d_0 > 0n, 'DIVISION_BY_ZERO');
    __compactRuntime.assert(r_0 < d_0, 'DIVISION_REMAINDER');
    __compactRuntime.assert(this._equal_4(this._checkedAdd_0(this._checkedMul_0(d_0,
                                                                                q_0,
                                                                                h_0),
                                                             r_0),
                                          n_0),
                            'DIVISION_EQUATION');
    return q_0;
  }
  _transition0_0(before_0,
                 args_0,
                 observations_0,
                 remaining_0,
                 revision_0,
                 hints_0)
  {
    __compactRuntime.assert(this._equal_5(this._checkedAdd_0(revision_0,
                                                             remaining_0),
                                          2n),
                            'LIFETIME_INVARIANT');
    __compactRuntime.assert(remaining_0 > 0n, 'LIFETIME_EXHAUSTED');
    let t_0;
    __compactRuntime.assert((t_0 = observations_0.o0, t_0 < 2000000000n),
                            'HORIZON_EXPIRED');
    const e0_0 = observations_0.o0;
    const e1_0 = 2000000000n;
    const e2_0 = e0_0 < e1_0;
    __compactRuntime.assert(e2_0, 'horizon expired');
    const e3_0 = args_0.a0;
    const e4_0 = 2n;
    const e5_0 = this._equal_6(e3_0, e4_0);
    __compactRuntime.assert(e5_0, 'borrower authority required');
    const e6_0 = before_0.f7;
    const e7_0 = 0n;
    const e8_0 = this._equal_7(e6_0, e7_0);
    __compactRuntime.assert(e8_0, 'period already accrued');
    const e9_0 = before_0.f0;
    const e10_0 = 8n;
    const e11_0 = this._checkedMul_0(e9_0, e10_0, hints_0.h0);
    const e12_0 = 31n;
    const e13_0 = this._checkedMul_0(e11_0, e12_0, hints_0.h1);
    const e14_0 = 100n;
    const e15_0 = 365n;
    const e16_0 = this._checkedMul_0(e14_0, e15_0, hints_0.h2);
    const e17_0 = e13_0;
    const e18_0 = e16_0;
    const e19_0 = this._checkedDiv_0(e17_0,
                                     e18_0,
                                     hints_0.h3.q,
                                     hints_0.h3.r,
                                     hints_0.h3.product);
    const e20_0 = e19_0;
    const e21_0 = 33972602n;
    const e22_0 = this._equal_8(e20_0, e21_0);
    __compactRuntime.assert(e22_0, 'independent interest value mismatch');
    const e23_0 = 500000000n;
    const e24_0 = e19_0;
    const e25_0 = before_0.f0;
    const e26_0 = 500000000n;
    const e27_0 = this._checkedSub_0(e25_0, e26_0);
    const e28_0 = e27_0;
    const e29_0 = 4500000000n;
    const e30_0 = this._equal_9(e28_0, e29_0);
    __compactRuntime.assert(e30_0, 'outstanding notional mismatch');
    const e31_0 = 1n;
    const e32_0 = 4n;
    const e33_0 = 2n;
    const e34_0 = 5n;
    const e35_0 = 1n;
    const e36_0 = e23_0;
    const e37_0 = 3n;
    const e38_0 = 2n;
    const e39_0 = 5n;
    const e40_0 = 1n;
    const e41_0 = e24_0;
    return { after:
               { f0: e27_0,
                 f1: e23_0,
                 f2: e24_0,
                 f3: before_0.f3,
                 f4: before_0.f4,
                 f5: before_0.f5,
                 f6: before_0.f6,
                 f7: e31_0,
                 f8: before_0.f8 },
             remaining: this._checkedSub_0(remaining_0, 1n),
             revision: this._checkedAdd_0(revision_0, 1n),
             effect0: { v0: e32_0, v1: e33_0, v2: e34_0, v3: e35_0, v4: e36_0 },
             effect1: { v0: e37_0, v1: e38_0, v2: e39_0, v3: e40_0, v4: e41_0 } };
  }
  _transition1_0(before_0,
                 args_0,
                 observations_0,
                 remaining_0,
                 revision_0,
                 hints_0)
  {
    __compactRuntime.assert(this._equal_10(this._checkedAdd_0(revision_0,
                                                              remaining_0),
                                           2n),
                            'LIFETIME_INVARIANT');
    __compactRuntime.assert(remaining_0 > 0n, 'LIFETIME_EXHAUSTED');
    let t_0;
    __compactRuntime.assert((t_0 = observations_0.o0, t_0 < 2000000000n),
                            'HORIZON_EXPIRED');
    const e0_0 = observations_0.o0;
    const e1_0 = 2000000000n;
    const e2_0 = e0_0 < e1_0;
    __compactRuntime.assert(e2_0, 'horizon expired');
    const e3_0 = args_0.a0;
    const e4_0 = 2n;
    const e5_0 = this._equal_11(e3_0, e4_0);
    __compactRuntime.assert(e5_0, 'borrower authority required');
    const e6_0 = before_0.f7;
    const e7_0 = 1n;
    const e8_0 = this._equal_12(e6_0, e7_0);
    __compactRuntime.assert(e8_0, 'dues are not ready');
    const e9_0 = args_0.a1;
    const e10_0 = 0n;
    const e11_0 = this._equal_13(e9_0, e10_0);
    __compactRuntime.assert(e11_0, 'settlement asset mismatch');
    const e12_0 = args_0.a2;
    const e13_0 = before_0.f1;
    const e14_0 = before_0.f2;
    const e15_0 = this._checkedAdd_0(e13_0, e14_0);
    const e16_0 = this._equal_14(e12_0, e15_0);
    __compactRuntime.assert(e16_0, 'settlement amount mismatch');
    const e17_0 = args_0.a2;
    const e18_0 = 533972602n;
    const e19_0 = this._equal_15(e17_0, e18_0);
    __compactRuntime.assert(e19_0, 'independent total value mismatch');
    const e20_0 = before_0.f5;
    const e21_0 = args_0.a2;
    const e22_0 = e20_0 >= e21_0;
    __compactRuntime.assert(e22_0, 'insufficient synthetic balance');
    const e23_0 = before_0.f5;
    const e24_0 = args_0.a2;
    const e25_0 = this._checkedSub_0(e23_0, e24_0);
    const e26_0 = before_0.f6;
    const e27_0 = args_0.a2;
    const e28_0 = this._checkedAdd_0(e26_0, e27_0);
    const e29_0 = before_0.f3;
    const e30_0 = before_0.f1;
    const e31_0 = this._checkedAdd_0(e29_0, e30_0);
    const e32_0 = before_0.f4;
    const e33_0 = before_0.f2;
    const e34_0 = this._checkedAdd_0(e32_0, e33_0);
    const e35_0 = args_0.a1;
    const e36_0 = 2n;
    const e37_0 = 5n;
    const e38_0 = args_0.a2;
    const e39_0 = 4n;
    const e40_0 = 2n;
    const e41_0 = 5n;
    const e42_0 = 1n;
    const e43_0 = before_0.f1;
    const e44_0 = args_0.a1;
    const e45_0 = 3n;
    const e46_0 = 2n;
    const e47_0 = 5n;
    const e48_0 = 1n;
    const e49_0 = before_0.f2;
    const e50_0 = args_0.a1;
    const e51_0 = 0n;
    const e52_0 = 0n;
    const e53_0 = 2n;
    const e54_0 = 1n;
    const e55_0 = before_0.f0;
    const e56_0 = 4500000000n;
    const e57_0 = this._equal_16(e55_0, e56_0);
    __compactRuntime.assert(e57_0,
                            'episode closure cannot discharge remaining notional');
    return { after:
               { f0: before_0.f0,
                 f1: e51_0,
                 f2: e52_0,
                 f3: e31_0,
                 f4: e34_0,
                 f5: e25_0,
                 f6: e28_0,
                 f7: e53_0,
                 f8: e54_0 },
             remaining: this._checkedSub_0(remaining_0, 1n),
             revision: this._checkedAdd_0(revision_0, 1n),
             effect0: { v0: e35_0, v1: e36_0, v2: e37_0, v3: e38_0 },
             effect1:
               { v0: e39_0,
                 v1: e40_0,
                 v2: e41_0,
                 v3: e42_0,
                 v4: e43_0,
                 v5: e44_0 },
             effect2:
               { v0: e45_0,
                 v1: e46_0,
                 v2: e47_0,
                 v3: e48_0,
                 v4: e49_0,
                 v5: e50_0 } };
  }
  _record0_0(context,
             partialProofData,
             before_0,
             args_0,
             observations_0,
             remaining_0,
             revision_0,
             hints_0)
  {
    const tmp_0 = this._transition0_0(before_0,
                                      args_0,
                                      observations_0,
                                      remaining_0,
                                      revision_0,
                                      hints_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_23.toValue(0n),
                                                                                              alignment: _descriptor_23.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_13.toValue(tmp_0),
                                                                                              alignment: _descriptor_13.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } }]);
    return [];
  }
  _record1_0(context,
             partialProofData,
             before_0,
             args_0,
             observations_0,
             remaining_0,
             revision_0,
             hints_0)
  {
    const tmp_0 = this._transition1_0(before_0,
                                      args_0,
                                      observations_0,
                                      remaining_0,
                                      revision_0,
                                      hints_0);
    __compactRuntime.queryLedgerState(context,
                                      partialProofData,
                                      [
                                       { push: { storage: false,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_23.toValue(1n),
                                                                                              alignment: _descriptor_23.alignment() }).encode() } },
                                       { push: { storage: true,
                                                 value: __compactRuntime.StateValue.newCell({ value: _descriptor_6.toValue(tmp_0),
                                                                                              alignment: _descriptor_6.alignment() }).encode() } },
                                       { ins: { cached: false, n: 1 } }]);
    return [];
  }
  _equal_0(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_1(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_2(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_3(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_4(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_5(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_6(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_7(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_8(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_9(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_10(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_11(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_12(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_13(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_14(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_15(x0, y0) {
    if (x0 !== y0) { return false; }
    return true;
  }
  _equal_16(x0, y0) {
    if (x0 !== y0) { return false; }
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
    get snapshot0() {
      return _descriptor_13.fromValue(__compactRuntime.queryLedgerState(context,
                                                                        partialProofData,
                                                                        [
                                                                         { dup: { n: 0 } },
                                                                         { idx: { cached: false,
                                                                                  pushPath: false,
                                                                                  path: [
                                                                                         { tag: 'value',
                                                                                           value: { value: _descriptor_23.toValue(0n),
                                                                                                    alignment: _descriptor_23.alignment() } }] } },
                                                                         { popeq: { cached: false,
                                                                                    result: undefined } }]).value);
    },
    get snapshot1() {
      return _descriptor_6.fromValue(__compactRuntime.queryLedgerState(context,
                                                                       partialProofData,
                                                                       [
                                                                        { dup: { n: 0 } },
                                                                        { idx: { cached: false,
                                                                                 pushPath: false,
                                                                                 path: [
                                                                                        { tag: 'value',
                                                                                          value: { value: _descriptor_23.toValue(1n),
                                                                                                   alignment: _descriptor_23.alignment() } }] } },
                                                                        { popeq: { cached: false,
                                                                                   result: undefined } }]).value);
    }
  };
}
const _emptyContext = {
  currentQueryContext: new __compactRuntime.QueryContext(new __compactRuntime.ContractState().data, __compactRuntime.dummyContractAddress())
};
const _dummyContract = new Contract({ });
export const pureCircuits = {
  checkedAdd: (...args_0) => {
    if (args_0.length !== 2) {
      throw new __compactRuntime.CompactError(`checkedAdd: expected 2 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const a_0 = args_0[0];
    const b_0 = args_0[1];
    if (!(typeof(a_0) === 'bigint' && a_0 >= 0n && a_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedAdd',
                                 'argument 1',
                                 'arithmetic.compact line 13 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 a_0)
    }
    if (!(typeof(b_0) === 'bigint' && b_0 >= 0n && b_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedAdd',
                                 'argument 2',
                                 'arithmetic.compact line 13 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 b_0)
    }
    return _dummyContract._checkedAdd_0(a_0, b_0);
  },
  checkedSub: (...args_0) => {
    if (args_0.length !== 2) {
      throw new __compactRuntime.CompactError(`checkedSub: expected 2 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const a_0 = args_0[0];
    const b_0 = args_0[1];
    if (!(typeof(a_0) === 'bigint' && a_0 >= 0n && a_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedSub',
                                 'argument 1',
                                 'arithmetic.compact line 17 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 a_0)
    }
    if (!(typeof(b_0) === 'bigint' && b_0 >= 0n && b_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedSub',
                                 'argument 2',
                                 'arithmetic.compact line 17 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 b_0)
    }
    return _dummyContract._checkedSub_0(a_0, b_0);
  },
  checkedMul: (...args_0) => {
    if (args_0.length !== 3) {
      throw new __compactRuntime.CompactError(`checkedMul: expected 3 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const a_0 = args_0[0];
    const b_0 = args_0[1];
    const h_0 = args_0[2];
    if (!(typeof(a_0) === 'bigint' && a_0 >= 0n && a_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedMul',
                                 'argument 1',
                                 'arithmetic.compact line 22 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 a_0)
    }
    if (!(typeof(b_0) === 'bigint' && b_0 >= 0n && b_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedMul',
                                 'argument 2',
                                 'arithmetic.compact line 22 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 b_0)
    }
    if (!(typeof(h_0) === 'object' && typeof(h_0.aLo) === 'bigint' && h_0.aLo >= 0n && h_0.aLo <= 18446744073709551615n && typeof(h_0.aHi) === 'bigint' && h_0.aHi >= 0n && h_0.aHi <= 18446744073709551615n && typeof(h_0.bLo) === 'bigint' && h_0.bLo >= 0n && h_0.bLo <= 18446744073709551615n && typeof(h_0.bHi) === 'bigint' && h_0.bHi >= 0n && h_0.bHi <= 18446744073709551615n && typeof(h_0.lo) === 'bigint' && h_0.lo >= 0n && h_0.lo <= 18446744073709551615n && typeof(h_0.carry) === 'bigint' && h_0.carry >= 0n && h_0.carry <= 18446744073709551615n)) {
      __compactRuntime.typeError('checkedMul',
                                 'argument 3',
                                 'arithmetic.compact line 22 char 1',
                                 'struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>',
                                 h_0)
    }
    return _dummyContract._checkedMul_0(a_0, b_0, h_0);
  },
  checkedDiv: (...args_0) => {
    if (args_0.length !== 5) {
      throw new __compactRuntime.CompactError(`checkedDiv: expected 5 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const n_0 = args_0[0];
    const d_0 = args_0[1];
    const q_0 = args_0[2];
    const r_0 = args_0[3];
    const h_0 = args_0[4];
    if (!(typeof(n_0) === 'bigint' && n_0 >= 0n && n_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedDiv',
                                 'argument 1',
                                 'arithmetic.compact line 32 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 n_0)
    }
    if (!(typeof(d_0) === 'bigint' && d_0 >= 0n && d_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedDiv',
                                 'argument 2',
                                 'arithmetic.compact line 32 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 d_0)
    }
    if (!(typeof(q_0) === 'bigint' && q_0 >= 0n && q_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedDiv',
                                 'argument 3',
                                 'arithmetic.compact line 32 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 q_0)
    }
    if (!(typeof(r_0) === 'bigint' && r_0 >= 0n && r_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('checkedDiv',
                                 'argument 4',
                                 'arithmetic.compact line 32 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 r_0)
    }
    if (!(typeof(h_0) === 'object' && typeof(h_0.aLo) === 'bigint' && h_0.aLo >= 0n && h_0.aLo <= 18446744073709551615n && typeof(h_0.aHi) === 'bigint' && h_0.aHi >= 0n && h_0.aHi <= 18446744073709551615n && typeof(h_0.bLo) === 'bigint' && h_0.bLo >= 0n && h_0.bLo <= 18446744073709551615n && typeof(h_0.bHi) === 'bigint' && h_0.bHi >= 0n && h_0.bHi <= 18446744073709551615n && typeof(h_0.lo) === 'bigint' && h_0.lo >= 0n && h_0.lo <= 18446744073709551615n && typeof(h_0.carry) === 'bigint' && h_0.carry >= 0n && h_0.carry <= 18446744073709551615n)) {
      __compactRuntime.typeError('checkedDiv',
                                 'argument 5',
                                 'arithmetic.compact line 32 char 1',
                                 'struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>',
                                 h_0)
    }
    return _dummyContract._checkedDiv_0(n_0, d_0, q_0, r_0, h_0);
  },
  transition0: (...args_0) => {
    if (args_0.length !== 6) {
      throw new __compactRuntime.CompactError(`transition0: expected 6 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const before_0 = args_0[0];
    const args_1 = args_0[1];
    const observations_0 = args_0[2];
    const remaining_0 = args_0[3];
    const revision_0 = args_0[4];
    const hints_0 = args_0[5];
    if (!(typeof(before_0) === 'object' && typeof(before_0.f0) === 'bigint' && before_0.f0 >= 0n && before_0.f0 <= 340282366920938463463374607431768211455n && typeof(before_0.f1) === 'bigint' && before_0.f1 >= 0n && before_0.f1 <= 340282366920938463463374607431768211455n && typeof(before_0.f2) === 'bigint' && before_0.f2 >= 0n && before_0.f2 <= 340282366920938463463374607431768211455n && typeof(before_0.f3) === 'bigint' && before_0.f3 >= 0n && before_0.f3 <= 340282366920938463463374607431768211455n && typeof(before_0.f4) === 'bigint' && before_0.f4 >= 0n && before_0.f4 <= 340282366920938463463374607431768211455n && typeof(before_0.f5) === 'bigint' && before_0.f5 >= 0n && before_0.f5 <= 340282366920938463463374607431768211455n && typeof(before_0.f6) === 'bigint' && before_0.f6 >= 0n && before_0.f6 <= 340282366920938463463374607431768211455n && typeof(before_0.f7) === 'bigint' && before_0.f7 >= 0n && before_0.f7 <= 340282366920938463463374607431768211455n && typeof(before_0.f8) === 'bigint' && before_0.f8 >= 0n && before_0.f8 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition0',
                                 'argument 1',
                                 'kernel.compact line 13 char 1',
                                 'struct KernelState<f0: Uint<0..340282366920938463463374607431768211456>, f1: Uint<0..340282366920938463463374607431768211456>, f2: Uint<0..340282366920938463463374607431768211456>, f3: Uint<0..340282366920938463463374607431768211456>, f4: Uint<0..340282366920938463463374607431768211456>, f5: Uint<0..340282366920938463463374607431768211456>, f6: Uint<0..340282366920938463463374607431768211456>, f7: Uint<0..340282366920938463463374607431768211456>, f8: Uint<0..340282366920938463463374607431768211456>>',
                                 before_0)
    }
    if (!(typeof(args_1) === 'object' && typeof(args_1.a0) === 'bigint' && args_1.a0 >= 0n && args_1.a0 <= 4294967295n)) {
      __compactRuntime.typeError('transition0',
                                 'argument 2',
                                 'kernel.compact line 13 char 1',
                                 'struct Arguments0<a0: Uint<0..4294967296>>',
                                 args_1)
    }
    if (!(typeof(observations_0) === 'object' && typeof(observations_0.o0) === 'bigint' && observations_0.o0 >= 0n && observations_0.o0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition0',
                                 'argument 3',
                                 'kernel.compact line 13 char 1',
                                 'struct KernelObservations<o0: Uint<0..340282366920938463463374607431768211456>>',
                                 observations_0)
    }
    if (!(typeof(remaining_0) === 'bigint' && remaining_0 >= 0n && remaining_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition0',
                                 'argument 4',
                                 'kernel.compact line 13 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 remaining_0)
    }
    if (!(typeof(revision_0) === 'bigint' && revision_0 >= 0n && revision_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition0',
                                 'argument 5',
                                 'kernel.compact line 13 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 revision_0)
    }
    if (!(typeof(hints_0) === 'object' && typeof(hints_0.h0) === 'object' && typeof(hints_0.h0.aLo) === 'bigint' && hints_0.h0.aLo >= 0n && hints_0.h0.aLo <= 18446744073709551615n && typeof(hints_0.h0.aHi) === 'bigint' && hints_0.h0.aHi >= 0n && hints_0.h0.aHi <= 18446744073709551615n && typeof(hints_0.h0.bLo) === 'bigint' && hints_0.h0.bLo >= 0n && hints_0.h0.bLo <= 18446744073709551615n && typeof(hints_0.h0.bHi) === 'bigint' && hints_0.h0.bHi >= 0n && hints_0.h0.bHi <= 18446744073709551615n && typeof(hints_0.h0.lo) === 'bigint' && hints_0.h0.lo >= 0n && hints_0.h0.lo <= 18446744073709551615n && typeof(hints_0.h0.carry) === 'bigint' && hints_0.h0.carry >= 0n && hints_0.h0.carry <= 18446744073709551615n && typeof(hints_0.h1) === 'object' && typeof(hints_0.h1.aLo) === 'bigint' && hints_0.h1.aLo >= 0n && hints_0.h1.aLo <= 18446744073709551615n && typeof(hints_0.h1.aHi) === 'bigint' && hints_0.h1.aHi >= 0n && hints_0.h1.aHi <= 18446744073709551615n && typeof(hints_0.h1.bLo) === 'bigint' && hints_0.h1.bLo >= 0n && hints_0.h1.bLo <= 18446744073709551615n && typeof(hints_0.h1.bHi) === 'bigint' && hints_0.h1.bHi >= 0n && hints_0.h1.bHi <= 18446744073709551615n && typeof(hints_0.h1.lo) === 'bigint' && hints_0.h1.lo >= 0n && hints_0.h1.lo <= 18446744073709551615n && typeof(hints_0.h1.carry) === 'bigint' && hints_0.h1.carry >= 0n && hints_0.h1.carry <= 18446744073709551615n && typeof(hints_0.h2) === 'object' && typeof(hints_0.h2.aLo) === 'bigint' && hints_0.h2.aLo >= 0n && hints_0.h2.aLo <= 18446744073709551615n && typeof(hints_0.h2.aHi) === 'bigint' && hints_0.h2.aHi >= 0n && hints_0.h2.aHi <= 18446744073709551615n && typeof(hints_0.h2.bLo) === 'bigint' && hints_0.h2.bLo >= 0n && hints_0.h2.bLo <= 18446744073709551615n && typeof(hints_0.h2.bHi) === 'bigint' && hints_0.h2.bHi >= 0n && hints_0.h2.bHi <= 18446744073709551615n && typeof(hints_0.h2.lo) === 'bigint' && hints_0.h2.lo >= 0n && hints_0.h2.lo <= 18446744073709551615n && typeof(hints_0.h2.carry) === 'bigint' && hints_0.h2.carry >= 0n && hints_0.h2.carry <= 18446744073709551615n && typeof(hints_0.h3) === 'object' && typeof(hints_0.h3.q) === 'bigint' && hints_0.h3.q >= 0n && hints_0.h3.q <= 340282366920938463463374607431768211455n && typeof(hints_0.h3.r) === 'bigint' && hints_0.h3.r >= 0n && hints_0.h3.r <= 340282366920938463463374607431768211455n && typeof(hints_0.h3.product) === 'object' && typeof(hints_0.h3.product.aLo) === 'bigint' && hints_0.h3.product.aLo >= 0n && hints_0.h3.product.aLo <= 18446744073709551615n && typeof(hints_0.h3.product.aHi) === 'bigint' && hints_0.h3.product.aHi >= 0n && hints_0.h3.product.aHi <= 18446744073709551615n && typeof(hints_0.h3.product.bLo) === 'bigint' && hints_0.h3.product.bLo >= 0n && hints_0.h3.product.bLo <= 18446744073709551615n && typeof(hints_0.h3.product.bHi) === 'bigint' && hints_0.h3.product.bHi >= 0n && hints_0.h3.product.bHi <= 18446744073709551615n && typeof(hints_0.h3.product.lo) === 'bigint' && hints_0.h3.product.lo >= 0n && hints_0.h3.product.lo <= 18446744073709551615n && typeof(hints_0.h3.product.carry) === 'bigint' && hints_0.h3.product.carry >= 0n && hints_0.h3.product.carry <= 18446744073709551615n)) {
      __compactRuntime.typeError('transition0',
                                 'argument 6',
                                 'kernel.compact line 13 char 1',
                                 'struct Hints0<h0: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>, h1: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>, h2: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>, h3: struct DivisionHint<q: Uint<0..340282366920938463463374607431768211456>, r: Uint<0..340282366920938463463374607431768211456>, product: struct MulHint<aLo: Uint<0..18446744073709551616>, aHi: Uint<0..18446744073709551616>, bLo: Uint<0..18446744073709551616>, bHi: Uint<0..18446744073709551616>, lo: Uint<0..18446744073709551616>, carry: Uint<0..18446744073709551616>>>>',
                                 hints_0)
    }
    return _dummyContract._transition0_0(before_0,
                                         args_1,
                                         observations_0,
                                         remaining_0,
                                         revision_0,
                                         hints_0);
  },
  transition1: (...args_0) => {
    if (args_0.length !== 6) {
      throw new __compactRuntime.CompactError(`transition1: expected 6 arguments (as invoked from Typescript), received ${args_0.length}`);
    }
    const before_0 = args_0[0];
    const args_1 = args_0[1];
    const observations_0 = args_0[2];
    const remaining_0 = args_0[3];
    const revision_0 = args_0[4];
    const hints_0 = args_0[5];
    if (!(typeof(before_0) === 'object' && typeof(before_0.f0) === 'bigint' && before_0.f0 >= 0n && before_0.f0 <= 340282366920938463463374607431768211455n && typeof(before_0.f1) === 'bigint' && before_0.f1 >= 0n && before_0.f1 <= 340282366920938463463374607431768211455n && typeof(before_0.f2) === 'bigint' && before_0.f2 >= 0n && before_0.f2 <= 340282366920938463463374607431768211455n && typeof(before_0.f3) === 'bigint' && before_0.f3 >= 0n && before_0.f3 <= 340282366920938463463374607431768211455n && typeof(before_0.f4) === 'bigint' && before_0.f4 >= 0n && before_0.f4 <= 340282366920938463463374607431768211455n && typeof(before_0.f5) === 'bigint' && before_0.f5 >= 0n && before_0.f5 <= 340282366920938463463374607431768211455n && typeof(before_0.f6) === 'bigint' && before_0.f6 >= 0n && before_0.f6 <= 340282366920938463463374607431768211455n && typeof(before_0.f7) === 'bigint' && before_0.f7 >= 0n && before_0.f7 <= 340282366920938463463374607431768211455n && typeof(before_0.f8) === 'bigint' && before_0.f8 >= 0n && before_0.f8 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition1',
                                 'argument 1',
                                 'kernel.compact line 72 char 1',
                                 'struct KernelState<f0: Uint<0..340282366920938463463374607431768211456>, f1: Uint<0..340282366920938463463374607431768211456>, f2: Uint<0..340282366920938463463374607431768211456>, f3: Uint<0..340282366920938463463374607431768211456>, f4: Uint<0..340282366920938463463374607431768211456>, f5: Uint<0..340282366920938463463374607431768211456>, f6: Uint<0..340282366920938463463374607431768211456>, f7: Uint<0..340282366920938463463374607431768211456>, f8: Uint<0..340282366920938463463374607431768211456>>',
                                 before_0)
    }
    if (!(typeof(args_1) === 'object' && typeof(args_1.a0) === 'bigint' && args_1.a0 >= 0n && args_1.a0 <= 4294967295n && typeof(args_1.a1) === 'bigint' && args_1.a1 >= 0n && args_1.a1 <= 4294967295n && typeof(args_1.a2) === 'bigint' && args_1.a2 >= 0n && args_1.a2 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition1',
                                 'argument 2',
                                 'kernel.compact line 72 char 1',
                                 'struct Arguments1<a0: Uint<0..4294967296>, a1: Uint<0..4294967296>, a2: Uint<0..340282366920938463463374607431768211456>>',
                                 args_1)
    }
    if (!(typeof(observations_0) === 'object' && typeof(observations_0.o0) === 'bigint' && observations_0.o0 >= 0n && observations_0.o0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition1',
                                 'argument 3',
                                 'kernel.compact line 72 char 1',
                                 'struct KernelObservations<o0: Uint<0..340282366920938463463374607431768211456>>',
                                 observations_0)
    }
    if (!(typeof(remaining_0) === 'bigint' && remaining_0 >= 0n && remaining_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition1',
                                 'argument 4',
                                 'kernel.compact line 72 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 remaining_0)
    }
    if (!(typeof(revision_0) === 'bigint' && revision_0 >= 0n && revision_0 <= 340282366920938463463374607431768211455n)) {
      __compactRuntime.typeError('transition1',
                                 'argument 5',
                                 'kernel.compact line 72 char 1',
                                 'Uint<0..340282366920938463463374607431768211456>',
                                 revision_0)
    }
    if (!(typeof(hints_0) === 'object' && typeof(hints_0.unused) === 'bigint' && hints_0.unused >= 0n && hints_0.unused <= 1n)) {
      __compactRuntime.typeError('transition1',
                                 'argument 6',
                                 'kernel.compact line 72 char 1',
                                 'struct Hints1<unused: Uint<0..2>>',
                                 hints_0)
    }
    return _dummyContract._transition1_0(before_0,
                                         args_1,
                                         observations_0,
                                         remaining_0,
                                         revision_0,
                                         hints_0);
  }
};
export const contractReferenceLocations =
  { tag: 'publicLedgerArray', indices: { } };
//# sourceMappingURL=index.js.map
