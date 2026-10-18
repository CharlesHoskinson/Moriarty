import { TIME_PREDICATE } from './admit.mjs';

const MAX = (1n << 128n) - 1n;
const hexBytes = h => `#"${h}"`;
const pascal = s => s.charAt(0).toUpperCase() + s.slice(1);
function textLit(s) {
  if (!/^[A-Za-z0-9_.:-]*$/.test(s)) throw Object.assign(new Error('TEXT_LITERAL_UNSUPPORTED'), { code: 'TEXT_LITERAL_UNSUPPORTED' });
  return JSON.stringify(s);
}

function numeric(v) {
  if (v.tag === 'Bool') return v.value ? 'True' : 'False';
  if (v.tag === 'Text') return textLit(v.value);
  return v.value;
}

function firstActionOverflows(m) {
  const initial = Object.fromEntries(m.initialState.map(s => [s.name, BigInt(s.value.value)]));
  for (const ins of m.core.actions[0]?.instructions || []) {
    if (ins.tag !== 'Set') continue;
    const e = ins.expression;
    if (e.tag !== 'Add') continue;
    const parts = [e.left, e.right];
    const state = parts.find(p => p.tag === 'StateRef');
    const lit = parts.find(p => p.tag === 'Literal' && (p.value?.tag === 'UInt128' || p.value?.tag === 'Amount'));
    if (state && lit) {
      if (initial[state.declaration] + BigInt(lit.value.value) > MAX) return true;
    }
  }
  return false;
}

export function lowerAiken(admitted) {
  if (!admitted?.ok) throw Object.assign(new Error(admitted?.code || 'NOT_ADMITTED'), { code: admitted?.code || 'NOT_ADMITTED' });
  const bound = admitted.bound;
  const m = bound.manifest;
  const man = admitted.manifest;
  const stateFields = m.stateSchema.map(s => s.name);
  const initial = Object.fromEntries(m.initialState.map(s => [s.name, s.value.value]));
  const overflowFixture = firstActionOverflows(m);
  const lines = [];
  const emit = s => lines.push(s);

  emit('//// Generated from Moriarty Core. Public atomic experiment; not production/PCD.');
  emit(`//// programHash ${bound.programHash}`);
  emit(`//// sourceHash ${m.sourceHash}`);
  emit(`//// timePredicate ${TIME_PREDICATE}`);
  emit('//// nativePredicate public-single-head-wallet-transfer/1');
  emit('//// genesis: external consumed-seed one-shot mint; this spend validator does not authenticate issuance.');
  emit('use aiken/collection/list');
  emit('use aiken/interval.{Interval, IntervalBound, Finite}');
  emit('use cardano/address.{Address, Credential, Script, VerificationKey}');
  emit('use cardano/assets.{Value, PolicyId, AssetName}');
  emit('use cardano/transaction.{Transaction, Output, OutputReference, Input, InlineDatum, NoDatum, Datum}');
  emit('use cardano/transaction as tx');
  emit('use moriarty/public.{checked_add, checked_sub, checked_mul, checked_div, last_valid_posix, max_u128, in_u128}');
  emit('');
  emit(`pub const program_hash: ByteArray = ${hexBytes(bound.programHash)}`);
  emit(`pub const bounds_hash: ByteArray = ${hexBytes(m.bounds.boundsHash)}`);
  emit(`pub const source_hash: ByteArray = ${hexBytes(m.sourceHash)}`);
  emit(`pub const domain_network: ByteArray = ${textLit(man.domain.network)}`);
  emit(`pub const domain_deployment: ByteArray = ${textLit(man.domain.deployment)}`);
  emit(`pub const thread_policy: PolicyId = ${hexBytes(man.thread.policy)}`);
  emit(`pub const thread_name: AssetName = ${hexBytes(man.thread.name)}`);
  emit(`pub const lifetime: Int = ${m.lifetime}`);
  emit(`pub const horizon: Int = ${m.horizon}`);
  emit('');
  emit('pub type StateValues {');
  emit(stateFields.map(n => `  ${n}: Int,`).join('\n') || '  unused: Int,');
  emit('}');
  emit('');
  emit('pub fn genesis_values() -> StateValues {');
  emit(`  StateValues { ${stateFields.map(n => `${n}: ${initial[n] ?? '0'}`).join(', ')} }`);
  emit('}');
  emit('');
  emit('pub type HeadDatum {');
  emit('  program_hash: ByteArray,');
  emit('  bounds_hash: ByteArray,');
  emit('  source_hash: ByteArray,');
  emit('  domain_network: ByteArray,');
  emit('  domain_deployment: ByteArray,');
  emit('  instance_id: ByteArray,');
  emit('  revision: Int,');
  emit('  remaining: Int,');
  emit('  values: StateValues,');
  emit('}');
  emit('');
  for (const action of m.core.actions) {
    emit(`pub type ${pascal(action.name)}Args {`);
    for (const a of action.arguments) {
      emit(`  ${a.name}: ${a.type.tag === 'Text' ? 'ByteArray' : 'Int'},`);
    }
    emit('}');
    emit('');
  }
  emit('pub type AtomicRedeemer {');
  for (const action of m.core.actions) emit(`  ${pascal(action.name)}(${pascal(action.name)}Args)`);
  emit('}');
  emit('');
  emit('pub fn actor_vkh(name: ByteArray) -> ByteArray {');
  const actors = Object.entries(man.actors);
  actors.forEach(([name, hex], i) => {
    emit(`  ${i === 0 ? 'if' : 'else if'} name == ${textLit(name)} { ${hexBytes(hex)} }`);
  });
  emit('  else { fail }');
  emit('}');
  emit('');
  emit('pub fn actor_address(name: ByteArray) -> Address {');
  emit('  Address { payment_credential: VerificationKey(actor_vkh(name)), stake_credential: None }');
  emit('}');
  emit('');
  const assets = Object.entries(man.assets);
  emit('pub fn asset_policy(asset: ByteArray) -> PolicyId {');
  assets.forEach(([name, spec], i) => {
    emit(`  ${i === 0 ? 'if' : 'else if'} asset == ${textLit(name)} { ${hexBytes(spec.policy)} }`);
  });
  emit('  else { fail }');
  emit('}');
  emit('');
  emit('pub fn asset_name(asset: ByteArray) -> AssetName {');
  assets.forEach(([name, spec], i) => {
    emit(`  ${i === 0 ? 'if' : 'else if'} asset == ${textLit(name)} { ${hexBytes(spec.name)} }`);
  });
  emit('  else { fail }');
  emit('}');
  emit('');
  emit('fn bindings_match(head: HeadDatum) -> Bool {');
  emit('  and {');
  emit('    head.program_hash == program_hash,');
  emit('    head.bounds_hash == bounds_hash,');
  emit('    head.source_hash == source_hash,');
  emit('    head.domain_network == domain_network,');
  emit('    head.domain_deployment == domain_deployment,');
  emit('  }');
  emit('}');
  emit('');
  emit('fn thread_qty(value: Value) -> Int {');
  emit('  assets.quantity_of(value, thread_policy, thread_name)');
  emit('}');
  emit('');
  emit('fn is_script(cred: Credential) -> Bool {');
  emit('  when cred is {');
  emit('    Script(_) -> True');
  emit('    VerificationKey(_) -> False');
  emit('  }');
  emit('}');
  emit('');
  emit('fn take_payout(');
  emit('  outputs: List<Output>,');
  emit('  used: List<Int>,');
  emit('  idx: Int,');
  emit('  target: Address,');
  emit('  policy: PolicyId,');
  emit('  name: AssetName,');
  emit('  qty: Int,');
  emit(') -> List<Int> {');
  emit('  when outputs is {');
  emit('    [] -> fail');
  emit('    [o, ..rest] -> {');
  emit('      if list.has(used, idx) {');
  emit('        take_payout(rest, used, idx + 1, target, policy, name, qty)');
  emit('      } else if o.address == target && assets.quantity_of(o.value, policy, name) == qty {');
  emit('        [idx, ..used]');
  emit('      } else {');
  emit('        take_payout(rest, used, idx + 1, target, policy, name, qty)');
  emit('      }');
  emit('    }');
  emit('  }');
  emit('}');
  emit('');
  emit('fn qty_at_cred_outputs(outputs: List<Output>, cred: Credential, policy: PolicyId, name: AssetName) -> Int {');
  emit('  list.foldl(');
  emit('    outputs,');
  emit('    0,');
  emit('    fn(o, acc) {');
  emit('      if o.address.payment_credential == cred {');
  emit('        acc + assets.quantity_of(o.value, policy, name)');
  emit('      } else {');
  emit('        acc');
  emit('      }');
  emit('    },');
  emit('  )');
  emit('}');
  emit('');
  emit('fn qty_at_cred_inputs(inputs: List<Input>, cred: Credential, policy: PolicyId, name: AssetName) -> Int {');
  emit('  qty_at_cred_outputs(list.map(inputs, fn(i) { i.output }), cred, policy, name)');
  emit('}');
  emit('');
  emit('fn party_net(self: Transaction, vkh: ByteArray, policy: PolicyId, name: AssetName) -> Int {');
  emit('  let cred = VerificationKey(vkh)');
  emit('  qty_at_cred_inputs(self.inputs, cred, policy, name) - qty_at_cred_outputs(self.outputs, cred, policy, name)');
  emit('}');
  emit('');
  emit('fn expect_single_script_head(inputs: List<Input>, own_ref: OutputReference) -> Input {');
  emit('  let scripts = list.filter(inputs, fn(i) { is_script(i.output.address.payment_credential) })');
  emit('  expect [own] = scripts');
  emit('  expect own.output_reference == own_ref');
  emit('  expect thread_qty(own.output.value) == 1');
  emit('  own');
  emit('}');
  emit('');
  emit('fn expect_one_continuation(outputs: List<Output>, script: Credential) -> Output {');
  emit('  let heads =');
  emit('    list.filter(');
  emit('      outputs,');
  emit('      fn(o) { o.address.payment_credential == script && thread_qty(o.value) == 1 },');
  emit('    )');
  emit('  expect [cont] = heads');
  emit('  expect list.length(list.filter(outputs, fn(o) { thread_qty(o.value) > 0 })) == 1');
  emit('  cont');
  emit('}');
  emit('');
  emit('fn head_value_preserved(own: Output, cont: Output) -> Bool {');
  emit('  assets.flatten(own.value) == assets.flatten(cont.value)');
  emit('}');
  emit('');

  function lowerAction(action) {
    const argMap = new Map(action.arguments.map(a => [a.name, `args.${a.name}`]));
    emit(`fn run_${action.name}(head: HeadDatum, redeemer: AtomicRedeemer, now: Int, self: Transaction, own_ref: OutputReference) -> Bool {`);
    emit(`  expect ${pascal(action.name)}(args) = redeemer`);
    emit('  expect bindings_match(head)');
    emit('  expect in_u128(head.revision)');
    emit('  expect in_u128(head.remaining)');
    for (const f of stateFields) emit(`  expect in_u128(head.values.${f})`);
    for (const a of action.arguments) {
      if (a.type.tag !== 'Text') emit(`  expect in_u128(args.${a.name})`);
    }
    emit('  expect head.remaining > 0');
    emit('  expect checked_add(head.revision, head.remaining) == lifetime');
    emit('  expect now < horizon');
    emit('  expect head.revision != 0 || head.values == genesis_values()');
    emit('  expect head.revision != 0 || head.remaining == lifetime');
    const constMap = new Map(m.constants.map(c => [c.name, numeric(c.value)]));
    const state = new Map(stateFields.map(n => [n, `head.values.${n}`]));
    const locals = new Map();
    let n = 0;
    const effects = [];
    function ref(e) {
      if (e.tag === 'Literal') return numeric(e.value);
      if (e.tag === 'Remaining') return 'head.remaining';
      if (e.tag === 'ArgRef') return argMap.get(e.declaration);
      if (e.tag === 'ObservationRef') {
        if (e.declaration !== 'now') throw Object.assign(new Error('OBSERVATION_UNSUPPORTED'), { code: 'OBSERVATION_UNSUPPORTED' });
        return 'now';
      }
      if (e.tag === 'ConstRef') return constMap.get(e.declaration);
      if (e.tag === 'StateRef') return state.get(e.declaration);
      if (e.tag === 'LocalRef') return locals.get(e.local);
      return null;
    }
    function expr(e) {
      const direct = ref(e);
      if (direct !== null && e.tag !== 'Not' && e.tag !== 'FloorDiv' && !e.left) return direct;
      if (e.tag === 'And' || e.tag === 'Or') {
        const left = expr(e.left);
        const mark = lines.length;
        const right = expr(e.right);
        const rightLets = lines.splice(mark);
        const id = `v${n++}`;
        if (e.tag === 'And') {
          emit(`  let ${id} = if ${left} {`);
          for (const line of rightLets) emit(`  ${line}`);
          emit(`    ${right}`);
          emit('  } else { False }');
        } else {
          emit(`  let ${id} = if ${left} { True } else {`);
          for (const line of rightLets) emit(`  ${line}`);
          emit(`    ${right}`);
          emit('  }');
        }
        return id;
      }
      const id = `v${n++}`;
      let body;
      if (e.tag === 'Not') body = `!${expr(e.operand)}`;
      else if (e.tag === 'FloorDiv') body = `checked_div(${expr(e.numerator)}, ${expr(e.denominator)})`;
      else if (e.tag === 'Add') body = `checked_add(${expr(e.left)}, ${expr(e.right)})`;
      else if (e.tag === 'Sub') body = `checked_sub(${expr(e.left)}, ${expr(e.right)})`;
      else if (e.tag === 'Mul') body = `checked_mul(${expr(e.left)}, ${expr(e.right)})`;
      else if (e.tag === 'Eq') body = `${expr(e.left)} == ${expr(e.right)}`;
      else if (e.tag === 'Lt') body = `${expr(e.left)} < ${expr(e.right)}`;
      else if (e.tag === 'Lte') body = `${expr(e.left)} <= ${expr(e.right)}`;
      else if (e.tag === 'Gt') body = `${expr(e.left)} > ${expr(e.right)}`;
      else if (e.tag === 'Gte') body = `${expr(e.left)} >= ${expr(e.right)}`;
      else throw Object.assign(new Error('EXPRESSION_UNSUPPORTED'), { code: 'EXPRESSION_UNSUPPORTED', tag: e.tag });
      emit(`  let ${id} = ${body}`);
      return id;
    }
    for (const ins of action.instructions) {
      if (ins.tag === 'Guard') emit(`  expect ${expr(ins.condition)}`);
      else if (ins.tag === 'Let') locals.set(ins.name, expr(ins.expression));
      else if (ins.tag === 'Set') state.set(ins.field, expr(ins.expression));
      else if (ins.tag === 'Emit') {
        const fields = Object.fromEntries(ins.fields.map(f => [f.label, expr(f.expression)]));
        effects.push({ kind: ins.kind, fields });
      }
    }
    emit('  let own = expect_single_script_head(self.inputs, own_ref)');
    emit('  expect Script(script_hash) = own.output.address.payment_credential');
    emit('  expect list.has(self.extra_signatories, actor_vkh(args.actor))');
    emit('  expect assets.quantity_of(self.mint, thread_policy, thread_name) == 0');
    for (const [assetName] of assets) {
      emit(`  expect assets.quantity_of(self.mint, asset_policy(${textLit(assetName)}), asset_name(${textLit(assetName)})) == 0`);
    }
    emit('  let cont = expect_one_continuation(self.outputs, Script(script_hash))');
    emit('  expect InlineDatum(post_raw) = cont.datum');
    emit('  expect post_raw: HeadDatum = post_raw');
    emit('  let post = HeadDatum {');
    emit('    program_hash: head.program_hash,');
    emit('    bounds_hash: head.bounds_hash,');
    emit('    source_hash: head.source_hash,');
    emit('    domain_network: head.domain_network,');
    emit('    domain_deployment: head.domain_deployment,');
    emit('    instance_id: head.instance_id,');
    emit('    revision: checked_add(head.revision, 1),');
    emit('    remaining: checked_sub(head.remaining, 1),');
    emit(`    values: StateValues { ${stateFields.map(f => `${f}: ${state.get(f)}`).join(', ')} },`);
    emit('  }');
    emit('  expect post_raw == post');
    emit('  expect cont.address == own.output.address');
    emit('  expect head_value_preserved(own.output, cont)');
    emit('  let used: List<Int> = []');
    effects.forEach((eff, i) => {
      if (eff.kind !== 'Transfer' && eff.kind !== 'Fee') {
        throw Object.assign(new Error('EFFECT_KIND_UNSUPPORTED'), { code: 'EFFECT_KIND_UNSUPPORTED' });
      }
      const dest = i === effects.length - 1 ? `_used${i}` : `used${i}`;
      emit(`  expect ${dest} = take_payout(self.outputs, ${i === 0 ? 'used' : `used${i - 1}`}, 0, actor_address(${eff.fields.to}), asset_policy(${eff.fields.asset}), asset_name(${eff.fields.asset}), ${eff.fields.amount})`);
    });
    const nets = new Map();
    const keyOf = (party, asset) => `${party}\0${asset}`;
    for (const [actorName] of actors) {
      for (const [assetName] of assets) nets.set(keyOf(textLit(actorName), textLit(assetName)), { pos: [], neg: [] });
    }
    for (const eff of effects) {
      const fromK = keyOf(eff.fields.from, eff.fields.asset);
      const toK = keyOf(eff.fields.to, eff.fields.asset);
      if (!nets.has(fromK) || !nets.has(toK)) {
        throw Object.assign(new Error('SCRIPT_CUSTODY_UNSUPPORTED'), { code: 'SCRIPT_CUSTODY_UNSUPPORTED' });
      }
      nets.get(fromK).pos.push(eff.fields.amount);
      nets.get(toK).neg.push(eff.fields.amount);
    }
    for (const [actorName] of actors) {
      for (const [assetName] of assets) {
        const cell = nets.get(keyOf(textLit(actorName), textLit(assetName)));
        let expected = '0';
        for (const p of cell.pos) expected = `(${expected} + ${p})`;
        for (const nAmt of cell.neg) expected = `(${expected} - ${nAmt})`;
        emit(`  expect party_net(self, actor_vkh(${textLit(actorName)}), asset_policy(${textLit(assetName)}), asset_name(${textLit(assetName)})) == ${expected}`);
        if (cell.pos.length) {
          emit(`  expect ${expected} <= 0 || list.has(self.extra_signatories, actor_vkh(${textLit(actorName)}))`);
        }
      }
    }
    emit('  True');
    emit('}');
    emit('');
  }

  for (const action of m.core.actions) lowerAction(action);

  emit('validator atomic {');
  emit('  spend(datum: Option<HeadDatum>, redeemer: AtomicRedeemer, own_ref: OutputReference, tx: Transaction) {');
  emit('    expect Some(head) = datum');
  emit('    let now = last_valid_posix(tx.validity_range)');
  emit('    when redeemer is {');
  for (const action of m.core.actions) {
    emit(`      ${pascal(action.name)}(_) -> run_${action.name}(head, redeemer, now, tx, own_ref)`);
  }
  emit('    }');
  emit('  }');
  emit('');
  emit('  else(_) {');
  emit('    fail');
  emit('  }');
  emit('}');
  emit('');

  const tests = overflowFixture
    ? generateOverflowTests(m, man, stateFields, initial)
    : generatePrototypeTests(m, man, stateFields, initial);
  emit(tests.body);

  const validator = lines.join('\n') + '\n';
  return {
    validator,
    sampleDatum: sampleDatum(m, man, bound, stateFields),
    sampleRedeemer: sampleRedeemer(m),
    schemas: dataSchemas(m),
    expectedTests: tests.expectedTests,
    positiveUplcValidation: tests.positiveUplcValidation,
    metadata: {
      programHash: bound.programHash,
      sourceHash: m.sourceHash,
      boundsHash: m.bounds.boundsHash,
      timePredicate: TIME_PREDICATE,
      privateClaims: admitted.unsupportedPrivateClaims,
      production: false,
      nativePredicate: admitted.nativePredicate,
      genesisPrecondition: admitted.genesisPrecondition,
      positiveUplcValidation: tests.positiveUplcValidation,
    },
  };
}

function sampleDatum(m, man, bound, stateFields) {
  const values = Object.fromEntries(m.initialState.map(s => [s.name, s.value.value]));
  return {
    constructor: 0,
    fields: {
      program_hash: bound.programHash,
      bounds_hash: m.bounds.boundsHash,
      source_hash: m.sourceHash,
      domain_network: man.domain.network,
      domain_deployment: man.domain.deployment,
      instance_id: 'instance-1',
      revision: '0',
      remaining: m.lifetime,
      values: Object.fromEntries(stateFields.map(n => [n, values[n] ?? '0'])),
    },
  };
}

function sampleRedeemer(m) {
  const action = m.core.actions[0];
  const args = Object.fromEntries(action.arguments.map(a => {
    if (a.name === 'actor') return [a.name, 'payer'];
    if (a.type.tag === 'Text') return [a.name, 'CASH'];
    return [a.name, '5'];
  }));
  return { constructor: action.name, constructorIndex: 0, fields: args };
}

function dataSchemas(m) {
  return {
    HeadDatum: {
      encoding: 'PlutusData Constr 0',
      fields: [
        'program_hash: bytes32',
        'bounds_hash: bytes32',
        'source_hash: bytes32',
        'domain_network: bytes',
        'domain_deployment: bytes',
        'instance_id: bytes',
        'revision: int',
        'remaining: int',
        `values: Constr 0 { ${m.stateSchema.map(s => `${s.name}: int`).join(', ')} }`,
      ],
    },
    AtomicRedeemer: {
      encoding: 'PlutusData Constr per action ordinal',
      actions: m.core.actions.map((a, i) => ({
        constructor: i,
        name: a.name,
        fields: a.arguments.map(arg => ({ name: arg.name, type: arg.type.tag === 'Text' ? 'bytes' : 'int' })),
      })),
    },
    genesisConstructor: {
      role: 'source-level HeadDatum at revision 0 with manifest.initialState',
      remaining: 'lifetime',
      threadIssuance: 'external consumed-seed one-shot mint; not checked by this spend validator',
    },
  };
}

function headSig(stateFields) {
  return `fn head(revision: Int, remaining: Int, ${stateFields.map(n => `${n}: Int`).join(', ')}, instance_id: ByteArray) -> HeadDatum {`;
}

function headValues(stateFields, override = {}) {
  return `StateValues { ${stateFields.map(n => `${n}: ${override[n] ?? n}`).join(', ')} }`;
}

function sharedTestHelpers(stateFields, initial) {
  const out = [];
  const e = s => out.push(s);
  const script = '#"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"';
  e(headSig(stateFields));
  e('  HeadDatum {');
  e('    program_hash,');
  e('    bounds_hash,');
  e('    source_hash,');
  e('    domain_network,');
  e('    domain_deployment,');
  e('    instance_id,');
  e('    revision,');
  e('    remaining,');
  e(`    values: ${headValues(stateFields)},`);
  e('  }');
  e('}');
  e('');
  e('fn script_addr() -> Address {');
  e(`  Address { payment_credential: Script(${script}), stake_credential: None }`);
  e('}');
  e('');
  e('fn other_script_addr() -> Address {');
  e('  Address { payment_credential: Script(#"ffffffffffffffffffffffffffffffffffffffffffffffffffffffff"), stake_credential: None }');
  e('}');
  e('');
  e('fn valued(addr: Address, with_thread: Bool, cash_qty: Int, datum: Datum) -> Output {');
  e('  let ada = assets.from_lovelace(2_000_000)');
  e('  let threaded = if with_thread { assets.merge(ada, assets.from_asset(thread_policy, thread_name, 1)) } else { ada }');
  e('  let value = if cash_qty > 0 { assets.merge(threaded, assets.from_asset(asset_policy("CASH"), asset_name("CASH"), cash_qty)) } else { threaded }');
  e('  Output { address: addr, value, datum, reference_script: None }');
  e('}');
  e('');
  e('fn finite_range(lower: Int, upper_exclusive: Int) -> Interval {');
  e('  Interval {');
  e('    lower_bound: IntervalBound { bound_type: Finite(lower), is_inclusive: True },');
  e('    upper_bound: IntervalBound { bound_type: Finite(upper_exclusive), is_inclusive: False },');
  e('  }');
  e('}');
  e('');
  e('fn ref_at(i: Int) -> OutputReference {');
  e('  OutputReference { transaction_id: tx.placeholder.id, output_index: i }');
  e('}');
  e('');
  e('fn own_ref() -> OutputReference {');
  e('  ref_at(0)');
  e('}');
  e('');
  e('fn base_tx(inputs: List<Input>, outputs: List<Output>, signers: List<ByteArray>, range: Interval) -> Transaction {');
  e('  Transaction { ..tx.placeholder, inputs, outputs, extra_signatories: signers, validity_range: range }');
  e('}');
  e('');
  return { text: out.join('\n'), initial };
}

function generateOverflowTests(m, man, stateFields, initial) {
  const out = [];
  const e = s => out.push(s);
  const helpers = sharedTestHelpers(stateFields, initial);
  e(helpers.text);
  e('');
  const action = m.core.actions[0];
  const count = initial.count ?? '0';
  e('test overflow_step() fail {');
  e(`  let pre = head(0, lifetime, ${count}, "instance-1")`);
  e(`  let post = head(1, lifetime - 1, ${count}, "instance-1")`);
  e('  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(pre)) }');
  e('  let payer = Input { output_reference: ref_at(1), output: valued(actor_address("payer"), False, 10, NoDatum) }');
  e('  let cont = valued(script_addr(), True, 0, InlineDatum(post))');
  e('  let pay = valued(actor_address("payee"), False, 5, NoDatum)');
  e('  let change = valued(actor_address("payer"), False, 5, NoDatum)');
  e(`  let rdmr = ${pascal(action.name)}(${pascal(action.name)}Args { actor: "payer", ${action.arguments.filter(a => a.name !== 'actor').map(a => `${a.name}: ${a.type.tag === 'Text' ? '"CASH"' : '5'}`).join(', ')} })`);
  e('  let txb = base_tx([own, payer], [cont, pay, change], [actor_vkh("payer")], finite_range(1000, 2000))');
  e('  atomic.spend(Some(pre), rdmr, own_ref(), txb)');
  e('}');
  return {
    body: out.join('\n'),
    positiveUplcValidation: false,
    expectedTests: [{ title: 'overflow_step', status: 'success' }],
  };
}

function generatePrototypeTests(m, man, stateFields, initial) {
  const action = m.core.actions[0];
  const postValues = { ...initial };
  if (postValues.count !== undefined) postValues.count = String(BigInt(postValues.count) + 1n);
  const out = [];
  const e = s => out.push(s);
  e(sharedTestHelpers(stateFields, initial).text);
  e('');
  e('fn positive_tx() -> (Option<HeadDatum>, AtomicRedeemer, OutputReference, Transaction) {');
  e(`  let pre = head(0, lifetime, ${stateFields.map(n => initial[n] ?? '0').join(', ')}, "instance-1")`);
  e(`  let post = head(1, lifetime - 1, ${stateFields.map(n => postValues[n] ?? initial[n] ?? '0').join(', ')}, "instance-1")`);
  e('  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(pre)) }');
  e('  let payer = Input { output_reference: ref_at(1), output: valued(actor_address("payer"), False, 10, NoDatum) }');
  e('  let cont = valued(script_addr(), True, 0, InlineDatum(post))');
  e('  let pay = valued(actor_address("payee"), False, 5, NoDatum)');
  e('  let change = valued(actor_address("payer"), False, 5, NoDatum)');
  e(`  let rdmr = ${pascal(action.name)}(${pascal(action.name)}Args { actor: "payer", ${action.arguments.filter(a => a.name !== 'actor').map(a => `${a.name}: ${a.type.tag === 'Text' ? '"CASH"' : '5'}`).join(', ')} })`);
  e('  let txb = base_tx([own, payer], [cont, pay, change], [actor_vkh("payer")], finite_range(1000, 2000))');
  e('  (Some(pre), rdmr, own_ref(), txb)');
  e('}');
  e('');
  e('test positive_step() {');
  e('  let (d, r, o, t) = positive_tx()');
  e('  atomic.spend(d, r, o, t)');
  e('}');
  e('');
  const failCases = [
    ['missing_continuation', 'let (d, r, o, t) = positive_tx()\n  let t2 = Transaction { ..t, outputs: list.filter(t.outputs, fn(x) { thread_qty(x.value) == 0 }) }\n  atomic.spend(d, r, o, t2)'],
    ['multiple_continuation', 'let (d, r, o, t) = positive_tx()\n  expect [c, p, ch] = t.outputs\n  let t2 = Transaction { ..t, outputs: [c, c, p, ch] }\n  atomic.spend(d, r, o, t2)'],
    ['wrong_token', 'let (d, r, o, t) = positive_tx()\n  let bad = valued(script_addr(), False, 0, InlineDatum(head(1, lifetime - 1, 1, "instance-1")))\n  expect [_, p, ch] = t.outputs\n  atomic.spend(d, r, o, Transaction { ..t, outputs: [bad, p, ch] })'],
    ['wrong_address', 'let (d, r, o, t) = positive_tx()\n  let stolen = valued(actor_address("payee"), True, 0, InlineDatum(head(1, lifetime - 1, 1, "instance-1")))\n  expect [_, p, ch] = t.outputs\n  atomic.spend(d, r, o, Transaction { ..t, outputs: [stolen, p, ch] })'],
    ['wrong_datum', 'let (d, r, o, t) = positive_tx()\n  let mutated = valued(script_addr(), True, 0, InlineDatum(head(1, lifetime - 1, 9, "instance-1")))\n  expect [_, p, ch] = t.outputs\n  atomic.spend(d, r, o, Transaction { ..t, outputs: [mutated, p, ch] })'],
    ['changed_result', 'let (d, r, o, t) = positive_tx()\n  expect Some(pre) = d\n  let mutated = HeadDatum { ..pre, values: StateValues { count: 4 } }\n  atomic.spend(Some(mutated), r, o, t)'],
    ['signer_missing', 'let (d, r, o, t) = positive_tx()\n  atomic.spend(d, r, o, Transaction { ..t, extra_signatories: [] })'],
    ['stale_program_hash', 'let (d, r, o, t) = positive_tx()\n  expect Some(pre) = d\n  let stale = HeadDatum { ..pre, program_hash: #"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" }\n  expect [_, payer] = t.inputs\n  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(stale)) }\n  atomic.spend(Some(stale), r, o, Transaction { ..t, inputs: [own, payer] })'],
    ['conflicting_predecessor', 'let (d, r, o, t) = positive_tx()\n  expect [own, payer] = t.inputs\n  let other_ref = OutputReference { transaction_id: tx.placeholder.id, output_index: 2 }\n  let other = Input { output_reference: other_ref, output: own.output }\n  atomic.spend(d, r, o, Transaction { ..t, inputs: [own, payer, other] })'],
    ['shared_payout', 'let (d, r, o, t) = positive_tx()\n  expect [c, _, ch] = t.outputs\n  atomic.spend(d, r, o, Transaction { ..t, outputs: [c, ch] })'],
    ['unbounded_time', 'let (d, r, o, t) = positive_tx()\n  atomic.spend(d, r, o, Transaction { ..t, validity_range: tx.placeholder.validity_range })'],
    ['wallet_debit_required', 'let (d, r, o, t) = positive_tx()\n  expect [own, _] = t.inputs\n  let sponsor = Input { output_reference: ref_at(1), output: valued(actor_address("payee"), False, 5, NoDatum) }\n  expect [c, p, _] = t.outputs\n  atomic.spend(d, r, o, Transaction { ..t, inputs: [own, sponsor], outputs: [c, p] })'],
    ['cross_script_head', 'let (d, r, o, t) = positive_tx()\n  let other_value = assets.merge(assets.from_lovelace(2_000_000), assets.from_asset(#"ffffffffffffffffffffffffffffffffffffffffffffffffffffffff", #"4f544852", 1))\n  let other = Input { output_reference: ref_at(2), output: Output { address: other_script_addr(), value: other_value, datum: NoDatum, reference_script: None } }\n  expect [own, payer] = t.inputs\n  atomic.spend(d, r, o, Transaction { ..t, inputs: [own, payer, other] })'],
    ['forged_genesis_values', 'let (d, r, o, t) = positive_tx()\n  expect Some(pre) = d\n  let forged = HeadDatum { ..pre, values: StateValues { count: 4 } }\n  let post = HeadDatum { ..forged, revision: 1, remaining: lifetime - 1, values: StateValues { count: 5 } }\n  expect [_, payer] = t.inputs\n  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(forged)) }\n  let cont = valued(script_addr(), True, 0, InlineDatum(post))\n  expect [_, p, ch] = t.outputs\n  atomic.spend(Some(forged), r, o, Transaction { ..t, inputs: [own, payer], outputs: [cont, p, ch] })'],
    ['prestate_out_of_range', 'let (d, r, o, t) = positive_tx()\n  expect Some(pre) = d\n  let bad = HeadDatum { ..pre, values: StateValues { count: -1 } }\n  expect [_, payer] = t.inputs\n  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(bad)) }\n  atomic.spend(Some(bad), r, o, Transaction { ..t, inputs: [own, payer] })'],
    ['argument_out_of_range', `let (d, _r, o, t) = positive_tx()\n  let rdmr = ${pascal(action.name)}(${pascal(action.name)}Args { actor: "payer", ${action.arguments.filter(a => a.name !== 'actor').map(a => `${a.name}: ${a.type.tag === 'Text' ? '"CASH"' : '-1'}`).join(', ')} })\n  atomic.spend(d, rdmr, o, t)`],
  ];
  for (const [name, body] of failCases) {
    e(`test ${name}() fail {`);
    e(`  ${body}`);
    e('}');
    e('');
  }
  e('test arithmetic_boundary_count() fail {');
  e('  let (d, r, o, t) = positive_tx()');
  e('  expect Some(pre) = d');
  e('  let overflowed = HeadDatum { ..pre, revision: 1, remaining: lifetime - 1, values: StateValues { count: max_u128 } }');
  e('  expect [_, payer] = t.inputs');
  e('  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(overflowed)) }');
  e('  let post = HeadDatum { ..overflowed, revision: 2, remaining: lifetime - 2, values: StateValues { count: 0 } }');
  e('  let cont = valued(script_addr(), True, 0, InlineDatum(post))');
  e('  expect [_, p, ch] = t.outputs');
  e('  atomic.spend(Some(overflowed), r, o, Transaction { ..t, inputs: [own, payer], outputs: [cont, p, ch] })');
  e('}');
  e('');
  e('test shortcircuit_or() {');
  e('  let left = True');
  e('  let v = if left { True } else { checked_div(1, 0) > 0 }');
  e('  v');
  e('}');
  e('');
  e('test shortcircuit_and() {');
  e('  let left = False');
  e('  let v = if left { checked_div(1, 0) > 0 } else { False }');
  e('  !v');
  e('}');
  const expectedTests = [
    'positive_step',
    'missing_continuation',
    'multiple_continuation',
    'wrong_token',
    'wrong_address',
    'wrong_datum',
    'changed_result',
    'signer_missing',
    'stale_program_hash',
    'conflicting_predecessor',
    'shared_payout',
    'unbounded_time',
    'wallet_debit_required',
    'cross_script_head',
    'forged_genesis_values',
    'prestate_out_of_range',
    'argument_out_of_range',
    'arithmetic_boundary_count',
    'shortcircuit_or',
    'shortcircuit_and',
  ].map(title => ({ title, status: 'success' }));
  return {
    body: out.join('\n'),
    positiveUplcValidation: true,
    expectedTests,
  };
}

export { MAX };
