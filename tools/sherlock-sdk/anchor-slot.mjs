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
