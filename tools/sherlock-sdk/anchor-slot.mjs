// T12-SLOT15 (LEAD PICK): the host side of the anchor at top-level slot 15 (MYC-SEAL-005 to 008
// production profile) for SherlockLedger. compactc 0.36.0 cannot place a field at top-level index
// 15 (a ledger of more than 15 fields is nested: a 16th field compiles to a two-key `idx`), so the
// host builds it:
//   - deploy: the top-level state is a 16-child array, the compactc fields at their native
//     indices 0 to 3, Null at 4 to 14 and the anchor cell C_L at 15 (ledger 9.1.0.1 allows arrays
//     of up to 16 children, onchain-state state.rs; the runtime's `arrayPush` helper stops at 15,
//     so the array is built by one `push` op);
//   - call: the prologue `dup 0; idx [0]; popeq` of every entry reads slot 15: the one key of the
//     first uncached `idx` becomes 15. Every other transcript op is unchanged (MEASURED: the
//     compactc template differs only in the key word, lane 6; the wrap pins lane 6 to 15).
// Wiring into t10a-ledger-run.mjs (deploy and call) is T23 work; until then that runner builds
// slot-0 calls, which the slot-15 wrap key refuses.
export const ANCHOR_SLOT = 15;
export const TOP_LEVEL = 16;

/** The Field cell and the u8 key in the runtime's `AlignedValue` form. */
export function codec(rt) {
  const u8 = new rt.CompactTypeUnsignedInteger(255n, 1);
  const fld = rt.CompactTypeField;
  return {
    cell: x => ({tag: 'cell', content: {value: fld.toValue(x), alignment: fld.alignment()}}),
    key: n => ({tag: 'value', value: {value: u8.toValue(BigInt(n)), alignment: u8.alignment()}}),
    keyCell: n => ({tag: 'cell', content: {value: u8.toValue(BigInt(n)), alignment: u8.alignment()}}),
    field: v => fld.fromValue([...v]),
  };
}

/** The SherlockLedger deploy state (EncodedStateValue): `anchor`, `runs`, `verified_roots`,
 * `reg_root` at 0 to 3 (the constructor's values; `anchor` stays at 0, sealed and unread by the
 * wrap), Null at 4 to 14, the anchor cell at 15. */
export function ledgerDeployState(rt, anchor, regRoot) {
  if (anchor === 0n) throw new Error('zero anchor');
  const c = codec(rt);
  const content = [c.cell(anchor), {tag: 'map', content: new Map()}, {tag: 'map', content: new Map()}, c.cell(regRoot)];
  while (content.length < ANCHOR_SLOT) content.push({tag: 'null'});
  content.push(c.cell(anchor));
  return {tag: 'array', content};
}

/** The deploy state as a runtime `StateValue` (one `push` op on an empty stack). */
export function ledgerDeployStateValue(rt, ocrt, anchor, regRoot) {
  const st = new ocrt.VmStack();
  st.push(ocrt.StateValue.newNull(), true);
  const r = ocrt.runProgram(st, [{push: {storage: true, value: ledgerDeployState(rt, anchor, regRoot)}}], ocrt.CostModel.initialCostModel());
  return r.stack.get(1);
}

/** Rewrite the prologue of one entry's transcript ops: the first op must be `dup 0`, the second
 * an uncached `idx` with the one key 0 (the compactc read of field 0); its key becomes 15. Any
 * other shape is refused. */
export function prologueToSlot15(rt, ops) {
  const c = codec(rt);
  const [d, ix] = ops;
  const isKey0 = k => k?.tag === 'value' && k.value.value.length === 1 && [...k.value.value[0]].every(b => b === 0);
  if (!(d?.dup?.n === 0 && ix?.idx && ix.idx.cached === false && ix.idx.pushPath === false
        && ix.idx.path.length === 1 && isKey0(ix.idx.path[0]) && 'popeq' in (ops[2] ?? {}))) {
    throw new Error('not the SherlockLedger prologue: dup 0; idx [0] uncached; popeq');
  }
  return [d, {idx: {...ix.idx, path: [c.key(ANCHOR_SLOT)]}}, ...ops.slice(2)];
}

// T23 /check at slot 15 (Preview rehearsal 2026-11-15). The proof server's /check runs the stock compactc zkir of
// acceptRun, whose prologue impact is [0x50, 1, 1, 0x00]: the idx key is the constant 0 at public transcript input 4
// (call-vector lane 6). That zkir therefore refuses every slot-15 transcript ("Public transcript input mismatch for
// input 4"). The runner sends /check the slot-0 preimage of the same circuit run (same randomness, same TTL) and
// uses its skips for the slot-15 call. That is sound only when the two preimages differ in input 4 alone (0 -> 15)
// and the two partitioned programs differ in the op-1 idx key alone, with equal gas: then a stock check of the
// slot-0 preimage is the check of the slot-15 preimage against the key-15 zkir. This function refuses any other
// difference. (All acceptRun impacts have the constant guard 1, so the skips are all empty either way;
// paddedCheckResult refuses any other skips.)
export const PROLOGUE_KEY_INPUT = 4;
const canon = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? `${v}n` : v instanceof Uint8Array ? [...v] : v);
const PREIMAGE_FIELDS = ['tag', 'inputs', 'privateTranscript', 'publicTranscriptOutputs', 'bindingInput', 'communicationsCommitment', 'keyLocation'];

/** pre0 and pre15: decoded preimages (e6-padding decodePreimage) of the slot-0 and slot-15 builds; g0 and g15: their
 * guaranteed transcripts. Throws unless the only difference is the prologue key (input 4: 0 -> 15; op 1 idx path). */
export function assertSlot15CheckEquivalent(pre0, pre15, g0, g15) {
  for (const f of PREIMAGE_FIELDS) if (canon(pre0[f]) !== canon(pre15[f])) throw new Error(`slot-15 check: preimage field ${f} differs`);
  const a = pre0.publicTranscriptInputs, b = pre15.publicTranscriptInputs;
  if (a.length !== b.length) throw new Error(`slot-15 check: ${a.length} vs ${b.length} public transcript inputs`);
  const diff = a.map((x, i) => x === b[i] ? null : i).filter(i => i !== null);
  if (canon(diff) !== canon([PROLOGUE_KEY_INPUT]) || a[PROLOGUE_KEY_INPUT] !== 0n || b[PROLOGUE_KEY_INPUT] !== BigInt(ANCHOR_SLOT)) {
    throw new Error(`slot-15 check: public transcript inputs differ at [${diff}], not only input ${PROLOGUE_KEY_INPUT} 0 -> ${ANCHOR_SLOT}`);
  }
  const p = g0.program, q = g15.program;
  if (p.length !== q.length) throw new Error(`slot-15 check: ${p.length} vs ${q.length} ops`);
  const opDiff = p.map((op, i) => canon(op) === canon(q[i]) ? null : i).filter(i => i !== null);
  const pathless = op => canon({...op?.idx, path: null});
  if (canon(opDiff) !== canon([1]) || !p[1]?.idx || pathless(p[1]) !== pathless(q[1])) {
    throw new Error(`slot-15 check: programs differ at ops [${opDiff}], not only the op-1 idx key`);
  }
  if (canon(g0.gas) !== canon(g15.gas)) throw new Error('slot-15 check: gas differs');
  return {input: PROLOGUE_KEY_INPUT, from: 0n, to: BigInt(ANCHOR_SLOT)};
}
