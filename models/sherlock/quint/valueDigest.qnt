// VALUE DIGEST (task 2.5, specs/sherlock-value-digest, VDG-004, VDG-006; FD/ARCHITECTURE.md
// section 1.11 FP-4 row `vd`, the sherlock-value/2 forms).
//
// A structural value model with two separate physical representations:
//   - `heap`: the write-once DATA arena that CORE appends to (the kit computes each `vd` from the
//     form, the scalar and the `vd` lanes of the child records it reads through the read port);
//   - `rom`: the constants table, built in its own arena (other addresses, other order) by the
//     reference digest function (the PROG premise: constant digests are canonical).
// Digests are modelled by interning: `vt` maps a preimage word list to a fresh digest id, so the
// hash is injective on its domain (the collision-resistance premise). The reference decoding of
// a record is interned separately in `rt` over the decoded structure (form, scalar, the decoded
// children), independent of addresses and of `vd` lanes.
//
// Invariants (VDG-006):
//   VdRelocationInvariant  two records of the heap with the same decoded value have equal vd
//   VdConstDataEqual       a heap record and a constant with the same decoded value have equal vd
//   VdSharingEqual         a pair node over one shared child and a pair node over two distinct
//                          children of the same value have equal vd
//   VdInjective            equal vd implies equal decoded values (O24 under the premise)
//   VdFormsDisjoint        the FP-4 heads (length, tag, version, form, arity) are distinct per form
// Mutants (each is expected to violate the named invariant):
//   vdBusChain       a pair node hashes [vd(x), vd(y)] with no head (vd_bus_rehash_chain)  -> VdInjective
//   vdChildKind      a child may be an ENV record (child_kind_unconstrained)                -> VdInjective
//   vdHostVd         a node's vd is host supplied (host_supplied_vd)                        -> VdRelocationInvariant
//   vdCursorSkip     a fold step may advance the cursor by 2 (fold_cursor_skipped)          -> VdRelocationInvariant
//   vdBoolNonBool    Bool takes b = 2 (bool_nonboolean_selection)                           -> VdRelocationInvariant
//
// Check: quint typecheck valueDigest.qnt; then for each module M and invariant I below
//   quint run --main M --invariant I --max-steps 12 --max-samples 20000 valueDigest.qnt

module valueDigest {
  const MUT: str

  type VRec = { kind: int, form: int, scalar: int, kids: List[int], vd: int, ref: int }
  type VFold = { active: bool, m: int, cur: int, acc: int, kids: List[int], ty: int }

  // forms (FP-4): 0 UInt, 1 Bool, 2 Nil, 3 Cons, 4 Leaf, 5 Branch, 6 constructor; 7 field step
  pure val TAG_V = 13
  pure val KIND_DATA = 1
  pure val KIND_ENV = 3
  pure val MAXLEN = 6

  var heap: List[VRec]
  var rom: List[VRec]
  var vt: (List[int] -> int)
  var rt: (List[int] -> int)
  var fc: VFold

  pure def idOf(t: (List[int] -> int), k: List[int]): int =
    if (t.keys().contains(k)) t.get(k) else t.keys().size() + 1
  pure def intern(t: (List[int] -> int), k: List[int]): (List[int] -> int) =
    if (t.keys().contains(k)) t else t.put(k, t.keys().size() + 1)

  // the FP-4 head: [length with the tag, tag, version 2, form, arity]
  pure def arity(form: int): int = if (form == 3 or form == 5 or form == 7) 2 else if (form == 4) 1 else 0
  pure def plen(form: int): int =
    if (form == 0) 13 else if (form == 1) 6 else if (form == 2) 5
    else if (form == 4) 13 else if (form == 6) 23 else 21
  pure def vhead(form: int, ar: int): List[int] = [plen(form), TAG_V, 2, form, ar]

  // the preimage of a node from its scalar and child digests (the kit's word list)
  pure def preimage(mut: str, form: int, scalar: int, kv: List[int]): List[int] =
    if (mut == "busChain" and (form == 3 or form == 5)) kv
    else if (form == 0 or form == 1) vhead(form, 0).append(scalar)
    else if (form == 2) vhead(form, 0)
    else vhead(form, arity(form)).concat(kv)

  // the reference decoding key: form, the decoded scalar, the decoded children
  pure def refKey(form: int, scalar: int, kr: List[int]): List[int] =
    [form, if (form == 1) (if (scalar != 0) 1 else 0) else scalar].concat(kr)

  pure def isData(h: List[VRec], a: int): bool = h[a].kind == KIND_DATA

  pure val emptyFold = { active: false, m: 0, cur: 0, acc: 0, kids: [], ty: 0 }

  action init = all {
    heap' = [],
    rom' = [],
    vt' = Map(),
    rt' = Map(),
    fc' = emptyFold,
  }

  // ---- heap appends (CORE rules through VALUE DIGEST)
  action appendScalar = {
    nondet form = oneOf(Set(0, 1, 2))
    nondet s0 = oneOf(0.to(if (MUT == "boolNonBool") 2 else 1))
    val s = if (form == 2) 0 else s0
    val k = preimage(MUT, form, s, [])
    val r = refKey(form, s, [])
    all {
      heap.length() < MAXLEN,
      vt' = intern(vt, k),
      rt' = intern(rt, r),
      heap' = heap.append({ kind: KIND_DATA, form: form, scalar: s, kids: [], vd: idOf(vt, k), ref: idOf(rt, r) }),
      rom' = rom, fc' = fc,
    }
  }

  // an ENV record: its vd lane holds some digest of the table (or 0); it is not a value
  action appendEnv = {
    nondet g = oneOf(Set(0).union(vt.keys().map(k => vt.get(k))))
    all {
      heap.length() < MAXLEN,
      heap' = heap.append({ kind: KIND_ENV, form: -1, scalar: 0, kids: [], vd: g, ref: -1 }),
      vt' = vt, rt' = rt, rom' = rom, fc' = fc,
    }
  }

  action appendNode = {
    nondet form = oneOf(Set(3, 4, 5))
    nondet x = oneOf(0.to(MAXLEN - 1))
    nondet y = oneOf(0.to(MAXLEN - 1))
    val kids = if (form == 4) [x] else [x, y]
    val kv = kids.foldl([], (acc, a) => acc.append(heap[a].vd))
    val kr = kids.foldl([], (acc, a) => acc.append(heap[a].ref))
    val k = preimage(MUT, form, 0, kv)
    val r = refKey(form, 0, kr)
    nondet host = oneOf(1.to(vt.keys().size() + 1))
    all {
      heap.length() < MAXLEN,
      kids.foldl(true, (ok, a) => ok and a < heap.length()),
      // the read port: child kind DATA (dropped by vdChildKind)
      MUT == "childKind" or kids.foldl(true, (ok, a) => ok and isData(heap, a)),
      vt' = intern(vt, k),
      rt' = intern(rt, r),
      heap' = heap.append({ kind: KIND_DATA, form: form, scalar: 0, kids: kids,
                            vd: if (MUT == "hostVd") host else idOf(vt, k), ref: idOf(rt, r) }),
      rom' = rom, fc' = fc,
    }
  }

  // ---- the constructor fold (state `fc`): start (m, ty), one step per field, completion at cursor = m
  action foldStart = {
    nondet m = oneOf(0.to(2))
    nondet ty = oneOf(Set(1, 2))
    all {
      not(fc.active),
      fc' = { active: true, m: m, cur: 0, acc: 0, kids: [], ty: ty },
      heap' = heap, rom' = rom, vt' = vt, rt' = rt,
    }
  }

  action foldStep = {
    nondet f = oneOf(0.to(MAXLEN - 1))
    nondet adv = oneOf(if (MUT == "cursorSkip") Set(1, 2) else Set(1))
    val k = vhead(7, 2).concat([fc.acc, heap[f].vd])
    all {
      fc.active,
      fc.cur < fc.m,
      f < heap.length(),
      isData(heap, f),
      vt' = intern(vt, k),
      fc' = { ...fc, cur: fc.cur + adv, acc: idOf(vt, k), kids: fc.kids.append(f) },
      heap' = heap, rom' = rom, rt' = rt,
    }
  }

  action foldDone = {
    val k = [23, TAG_V, 2, 6, fc.m, fc.ty, fc.acc]
    val r = [6, fc.ty].concat(fc.kids.foldl([], (acc, a) => acc.append(heap[a].ref)))
    all {
      fc.active,
      fc.cur == fc.m,
      heap.length() < MAXLEN,
      vt' = intern(vt, k),
      rt' = intern(rt, r),
      heap' = heap.append({ kind: KIND_DATA, form: 6, scalar: fc.ty, kids: fc.kids, vd: idOf(vt, k), ref: idOf(rt, r) }),
      fc' = emptyFold,
      rom' = rom,
    }
  }

  // ---- the constants table: its own arena, built by the reference digest function
  action romScalar = {
    nondet form = oneOf(Set(0, 1, 2))
    nondet s0 = oneOf(0.to(1))
    val s = if (form == 2) 0 else s0
    val k = preimage("none", form, s, [])
    val r = refKey(form, s, [])
    all {
      rom.length() < MAXLEN,
      vt' = intern(vt, k), rt' = intern(rt, r),
      rom' = rom.append({ kind: KIND_DATA, form: form, scalar: s, kids: [], vd: idOf(vt, k), ref: idOf(rt, r) }),
      heap' = heap, fc' = fc,
    }
  }

  action romNode = {
    nondet form = oneOf(Set(3, 4, 5))
    nondet x = oneOf(0.to(MAXLEN - 1))
    nondet y = oneOf(0.to(MAXLEN - 1))
    val kids = if (form == 4) [x] else [x, y]
    val k = preimage("none", form, 0, kids.foldl([], (acc, a) => acc.append(rom[a].vd)))
    val r = refKey(form, 0, kids.foldl([], (acc, a) => acc.append(rom[a].ref)))
    all {
      rom.length() < MAXLEN,
      kids.foldl(true, (ok, a) => ok and a < rom.length()),
      vt' = intern(vt, k), rt' = intern(rt, r),
      rom' = rom.append({ kind: KIND_DATA, form: form, scalar: 0, kids: kids, vd: idOf(vt, k), ref: idOf(rt, r) }),
      heap' = heap, fc' = fc,
    }
  }

  action step = any { appendScalar, appendEnv, appendNode, foldStart, foldStep, foldDone, romScalar, romNode }

  // ---- invariants
  def dataIdx(h: List[VRec]): Set[int] = h.indices().filter(a => h[a].kind == KIND_DATA)

  val VdRelocationInvariant =
    dataIdx(heap).forall(a => dataIdx(heap).forall(b => heap[a].ref == heap[b].ref implies heap[a].vd == heap[b].vd))

  val VdConstDataEqual =
    dataIdx(heap).forall(a => rom.indices().forall(c =>
      (heap[a].ref == rom[c].ref implies heap[a].vd == rom[c].vd) and
      (heap[a].vd == rom[c].vd implies heap[a].ref == rom[c].ref)))

  val VdSharingEqual =
    dataIdx(heap).forall(a => dataIdx(heap).forall(b =>
      (heap[a].form == 3 or heap[a].form == 5) and heap[a].kids[0] == heap[a].kids[1] and
      heap[b].form == heap[a].form and heap[b].kids[0] != heap[b].kids[1] and
      heap[heap[b].kids[0]].ref == heap[heap[b].kids[1]].ref and
      heap[heap[a].kids[0]].ref == heap[heap[b].kids[0]].ref
        implies heap[a].vd == heap[b].vd))

  val VdInjective =
    dataIdx(heap).forall(a => dataIdx(heap).forall(b => heap[a].vd == heap[b].vd implies heap[a].ref == heap[b].ref))

  pure val VdFormsDisjoint =
    0.to(7).forall(f => 0.to(7).forall(g => f != g implies vhead(f, arity(f)) != vhead(g, arity(g))))

  val VdAll = VdRelocationInvariant and VdConstDataEqual and VdSharingEqual and VdInjective and VdFormsDisjoint

  // reachability witnesses (expected to FAIL: such states exist)
  val reachSharedPair = not(dataIdx(heap).exists(a => dataIdx(heap).exists(b =>
    (heap[a].form == 3) and heap[a].kids[0] == heap[a].kids[1] and heap[b].form == 3 and
    heap[b].kids[0] != heap[b].kids[1] and heap[a].vd == heap[b].vd)))
  val reachConstEqualsData = not(dataIdx(heap).exists(a => rom.indices().exists(c => heap[a].form >= 3 and heap[a].vd == rom[c].vd)))
  val reachCtor3 = not(dataIdx(heap).exists(a => heap[a].form == 6 and heap[a].kids.length() == 2))
}

module vdBase { import valueDigest(MUT = "none").* }
module vdBusChain { import valueDigest(MUT = "busChain").* }
module vdChildKind { import valueDigest(MUT = "childKind").* }
module vdHostVd { import valueDigest(MUT = "hostVd").* }
module vdCursorSkip { import valueDigest(MUT = "cursorSkip").* }
module vdBoolNonBool { import valueDigest(MUT = "boolNonBool").* }
