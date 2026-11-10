// Canonical genesis Init(prog_d, input_d) and the first-leaf check (SRC-021, council D1 as
// corrected 2026-11-10). Design: openspec/changes/sherlock-vm-consensus-20261107/design.md
// §3.2(5), table "Init(prog_d, input_d)". Reference start state: Machine::new
// (crates/mori-core/src/sherlock/reference.rs:14-27); C6 genesis: genesis()
// (crates/mori-core/src/sherlock/helper_machine/shared/owner_prepare.rs:2760-2800).
//
// What the model checks (all by pure quantification over finite sets, no sampling):
//   T1 exactness: for every statement (prog_d, input_d) of the domain and every witness of the
//      generated set with h_pre == 0, firstLeafAccepts holds iff the statement is admissible
//      (fuel limbs and stack limit canonical, input shape matches the program's input
//      declaration), the witness openings equal the statement digests, and the register file
//      and roots equal Init(prog_d, input_d) field by field.
//   T2 seal: a run is sealed only from a segment with h_lo == 0 and a terminal outcome.
//   T3 mutants: one mutant per Init field drops that field's equality; seven more drop the
//      prog_d opening, the input_d opening, the fuel range, the stack range, the input shape
//      check, the h_pre gate (replaced by a witness flag) and each half of the seal rule. Every
//      mutant must break T1 or T2 (survivors == Set()).
//   The witness set holds: the honest Init image; for each field, Init with that one field
//   changed (the change always differs from the canonical value); the same with a witness
//   genesis flag cleared; and a witness that opens another statement's prog or input together
//   with that statement's own Init image (swapped input, same fuel and stack).
//
// Abstractions (premises, not modeled): a digest is its opening (Poseidon injective on the
// domain); a table root is the tag plus the row content; the canonical value encoding of
// input_d is produced canonically (its well-formedness is a premise on val_d, like PROG
// canonicality on prog_root); the 64-bit limb is scaled to 16 bits; lane widths are collapsed (a None record or a zero cache is [0]).
// Linkage at h_pre > 0 (pre_d == previous post_d) is the node's job (F20/F21) and is not
// modeled here.
//
// Check: quint typecheck models/sherlock/quint/init.qnt
//        quint test --main init models/sherlock/quint/init.qnt

module init {
  // One fuel limb is 64 bits in the design; the model scales it to 2^16 so that LIMB * LIMB fits
  // the evaluator's i64. Canonicality is the same predicate (each limb below LIMB).
  pure val LIMB: int = 65536
  pure val NONE: int = -1                     // no input function
  pure val NIL: int = -1                      // empty stack link, not a valid address
  pure val ENTRY: int = -2                    // reserved virtual entry expression id
  pure val EVAL: int = 0                      // control tag of Eval (c6_phys.rs encode_control)
  pure val RUNNING: int = 0                   // outcome Running = reference Pending
  pure val T_DATA: int = 1
  pure val T_FIELDS: int = 2
  pure val T_ENV: int = 3
  pure val T_SLOTS: int = 4
  pure val EMPTY_DEPS: int = 0
  pure val TY_UINT: int = 0
  pure val TY_BOOL: int = 1
  pure val TY_LIST: int = 2

  // ------------------------------------------------------------------ openings
  // val_d opening: canonical encoding of the run input (data rows 5 lanes, field rows 2 lanes).
  type Val = { present: bool, ty: int, data: List[int], fields: List[int], ref: int }
  type ProgOpen = { root: int, e0: int, inFn: int, inTy: int }
  type InOpen = { v: Val, fuelLo: int, fuelHi: int, stack: int }
  type Stmt = { progD: ProgOpen, inputD: InOpen }
  type St = str -> List[int]
  type W = { hPre: int, genFlag: bool, prog: ProgOpen, inp: InOpen, st: St }
  type Seg = { hLo: int, terminal: bool }

  pure val NO_VAL: Val = { present: false, ty: 0, data: [], fields: [], ref: 0 }
  pure def encUInt(n: int): Val =
    { present: true, ty: TY_UINT, data: [0, n, 0, 0, 0], fields: [], ref: 0 }
  pure def encBool(b: int): Val =
    { present: true, ty: TY_BOOL, data: [1, b, 0, 0, 0], fields: [], ref: 0 }
  // List [a1..ak]: Nil at row 0; for ak down to a1: a U128 row, a field slice
  // [Data(u128 row), Data(tail)], a Cons row [3,0,0,start,2]. Children before parent.
  pure def encList(items: List[int]): Val =
    val k = items.length()
    val acc = range(0, k).foldl({ data: [2, 0, 0, 0, 0], fields: List(), prev: 0 }, (a, j) =>
      val x = items[k - 1 - j]
      val uRow = a.data.length() / 5
      val fStart = a.fields.length() / 2
      { data: a.data.concat([0, x, 0, 0, 0]).concat([3, 0, 0, fStart, 2]),
        fields: a.fields.concat([1, uRow, 1, a.prev]),
        prev: uRow + 1 })
    { present: true, ty: TY_LIST, data: acc.data, fields: acc.fields, ref: acc.prev }

  pure def nData(v: Val): int = v.data.length() / 5
  pure def nFields(v: Val): int = v.fields.length() / 2
  pure def fuelOf(i: InOpen): int = i.fuelLo + LIMB * i.fuelHi

  // ------------------------------------------------------------------ Init
  pure val FIELDS: Set[str] = Set(
    "ctl_tag", "ctl_expr", "ctl_rest", "ctl_origin",
    "outcome", "initial_fuel", "remaining_fuel", "consumed_work", "helper_work",
    "helper_visits", "reference_cursor", "stack_limit", "frame_count", "task_count",
    "env_ptr", "fp", "tp", "top", "task", "helper", "result", "vregs",
    "alloc_data", "alloc_fields", "alloc_env",
    "len_data", "len_fields", "len_env", "len_slots",
    "seg_active", "seg_alloc", "seg_consumed",
    "prog_root", "root_data", "root_fields", "root_env", "root_slots",
    "fin", "deps")

  pure def Init(p: ProgOpen, i: InOpen): St = Map(
    "ctl_tag" -> [EVAL],
    "ctl_expr" -> [if (p.inFn == NONE) p.e0 else ENTRY],
    "ctl_rest" -> [0],
    "ctl_origin" -> [0],
    "outcome" -> [RUNNING],
    "initial_fuel" -> [fuelOf(i)],
    "remaining_fuel" -> [fuelOf(i)],
    "consumed_work" -> [0],
    "helper_work" -> [0],
    "helper_visits" -> [0],
    "reference_cursor" -> [0],
    "stack_limit" -> [i.stack],
    "frame_count" -> [0],
    "task_count" -> [0],
    "env_ptr" -> [0, 0],
    "fp" -> [NIL],
    "tp" -> [NIL],
    "top" -> [0],
    "task" -> [0],
    "helper" -> [0],
    "result" -> [0],
    "vregs" -> [0, 0, 0],
    "alloc_data" -> [nData(i.v)],
    "alloc_fields" -> [nFields(i.v)],
    "alloc_env" -> [0],
    "len_data" -> [nData(i.v)],
    "len_fields" -> [nFields(i.v)],
    "len_env" -> [0],
    "len_slots" -> [0],
    "seg_active" -> [0, 0, 0, 0],
    "seg_alloc" -> [0, 0, 0],
    "seg_consumed" -> [0],
    "prog_root" -> [p.root],
    "root_data" -> [T_DATA].concat(i.v.data),
    "root_fields" -> [T_FIELDS].concat(i.v.fields),
    "root_env" -> [T_ENV],
    "root_slots" -> [T_SLOTS],
    "fin" -> [0],
    "deps" -> [EMPTY_DEPS])

  // ------------------------------------------------------------------ the first-leaf check
  pure def fuelCanonical(i: InOpen): bool =
    0 <= i.fuelLo and i.fuelLo < LIMB and 0 <= i.fuelHi and i.fuelHi < LIMB
  pure def stackCanonical(i: InOpen): bool = 0 <= i.stack and i.stack < LIMB
  // Input shape: a program without an input function takes no input; otherwise the input is
  // present and has the declared parameter type.
  pure def shapeOk(p: ProgOpen, i: InOpen): bool =
    if (p.inFn == NONE) not(i.v.present) else (i.v.present and i.v.ty == p.inTy)

  pure def gate(m: str, w: W): bool = if (m == "gate_by_flag") w.genFlag else w.hPre == 0

  pure def firstLeafAccepts(m: str, w: W, s: Stmt): bool =
    val ini = Init(w.prog, w.inp)
    and {
      m == "no_prog_open" or w.prog == s.progD,
      m == "no_input_open" or w.inp == s.inputD,
      not(gate(m, w)) or and {
        m == "no_fuel_range" or fuelCanonical(w.inp),
        m == "no_stack_range" or stackCanonical(w.inp),
        m == "no_input_shape" or shapeOk(w.prog, w.inp),
        w.st.keys() == FIELDS,
        FIELDS.forall(f => m == f or w.st.get(f) == ini.get(f)),
      },
    }

  pure def admissible(s: Stmt): bool =
    fuelCanonical(s.inputD) and stackCanonical(s.inputD) and shapeOk(s.progD, s.inputD)
  pure def exactSpec(w: W, s: Stmt): bool = and {
    admissible(s), w.prog == s.progD, w.inp == s.inputD,
    w.st == Init(s.progD, s.inputD),
  }

  // ------------------------------------------------------------------ seal rule
  pure def sealOk(m: str, g: Seg): bool =
    (m == "no_seal_h0" or g.hLo == 0) and (m == "no_seal_terminal" or g.terminal)
  pure val SEGS: Set[Seg] = Set(0, 5).map(h => Set(true, false).map(t => { hLo: h, terminal: t })).flatten()
  pure def sealExact(m: str): bool = SEGS.forall(g => sealOk(m, g) == (g.hLo == 0 and g.terminal))

  // ------------------------------------------------------------------ domain
  pure val P_FACT: ProgOpen = { root: 101, e0: 0, inFn: NONE, inTy: 0 }
  pure val P_LIST: ProgOpen = { root: 102, e0: 0, inFn: NONE, inTy: 0 }
  pure val P_UIN: ProgOpen = { root: 103, e0: 0, inFn: 2, inTy: TY_UINT }
  pure val P_LIN: ProgOpen = { root: 104, e0: 0, inFn: 1, inTy: TY_LIST }
  pure val PROGS: Set[ProgOpen] = Set(P_FACT, P_LIST, P_UIN, P_LIN)
  pure val VALS: Set[Val] = Set(NO_VAL, encUInt(5), encBool(1), encList([1, 2, 3]))
  // (lo, hi): zero, small, max u128, non-canonical lo >= 2^64 (aliases fuel 2^64 + 3).
  pure val FUELS: Set[(int, int)] = Set((0, 0), (300, 0), (LIMB - 1, LIMB - 1), (LIMB + 3, 0))
  pure val STACKS: Set[int] = Set(64, LIMB)
  pure val STMTS: Set[Stmt] =
    tuples(PROGS, VALS, FUELS, STACKS).map(((p, v, f, k)) =>
      { progD: p, inputD: { v: v, fuelLo: f._1, fuelHi: f._2, stack: k } })

  pure def bump(x: List[int]): List[int] = if (x.length() == 1) [x[0] + 1] else x.append(7)
  pure def honest(s: Stmt): W =
    { hPre: 0, genFlag: true, prog: s.progD, inp: s.inputD, st: Init(s.progD, s.inputD) }
  pure def tampered(s: Stmt, f: str, flag: bool): W =
    val h = honest(s)
    { ...h, genFlag: flag, st: h.st.set(f, bump(h.st.get(f))) }
  // Another statement's openings with that statement's own Init image.
  pure def swapped(s: Stmt, o: Stmt): W =
    { hPre: 0, genFlag: true, prog: o.progD, inp: o.inputD, st: Init(o.progD, o.inputD) }
  pure def others(s: Stmt): Set[Stmt] = Set(
    { ...s, inputD: { ...s.inputD, v: if (s.inputD.v == encUInt(5)) encUInt(6) else encUInt(5) } },
    { ...s, progD: { ...s.progD, root: s.progD.root + 10 } })
  pure def witnesses(s: Stmt): Set[W] =
    Set(honest(s))
      .union(FIELDS.map(f => tampered(s, f, true)))
      .union(FIELDS.map(f => tampered(s, f, false)))
      .union(others(s).map(o => swapped(s, o)))

  pure def exactAll(m: str): bool =
    STMTS.forall(s => witnesses(s).forall(w => firstLeafAccepts(m, w, s) == exactSpec(w, s)))
  pure def passes(m: str): bool = exactAll(m) and sealExact(m)

  pure val EXTRA: Set[str] = Set("no_prog_open", "no_input_open", "no_fuel_range",
    "no_stack_range", "no_input_shape", "gate_by_flag", "no_seal_h0", "no_seal_terminal")
  pure val MUTANTS: Set[str] = FIELDS.union(EXTRA)
  pure val survivors: Set[str] = MUTANTS.filter(m => passes(m))

  // ------------------------------------------------------------------ tests
  run fieldCountTest = assert(and {
    FIELDS.size() == 39,
    Init(P_FACT, { v: NO_VAL, fuelLo: 0, fuelHi: 0, stack: 64 }).keys() == FIELDS,
  })
  run encodingTest = assert(and {
    encList([1, 2, 3]).data.length() == 35,   // Nil + 3 x (U128 + Cons) = 7 rows
    encList([1, 2, 3]).fields.length() == 12, // 3 slices of 2 refs
    encList([1, 2, 3]).ref == 6,
  })
  run admissibleCasesExistTest = assert(and {
    STMTS.exists(s => admissible(s) and s.progD.inFn == NONE),
    STMTS.exists(s => admissible(s) and s.inputD.v.ty == TY_LIST and s.inputD.v.present),
    STMTS.exists(s => admissible(s) and s.inputD.fuelHi == LIMB - 1),
    STMTS.exists(s => not(admissible(s))),
  })
  run baseExactTest = assert(passes("none"))
  run honestAcceptedTest = assert(STMTS.forall(s => admissible(s) implies firstLeafAccepts("none", honest(s), s)))
  run everyFieldDeviationRefusedTest = assert(
    STMTS.forall(s => FIELDS.forall(f => not(firstLeafAccepts("none", tampered(s, f, true), s)))))
  run swappedInputRefusedTest = assert(
    STMTS.forall(s => others(s).forall(o => not(firstLeafAccepts("none", swapped(s, o), s)))))
  run genesisReuseNotGatedTest = assert(
    // At h_pre > 0 the Init check is off; such a segment cannot seal (h_lo != 0).
    not(sealOk("none", { hLo: 5, terminal: true })))
  run allMutantsCaughtTest = assert(survivors == Set())
  run fieldMutantsBreakExactnessTest = assert(FIELDS.forall(f => not(exactAll(f)) and sealExact(f)))
  run sealMutantsBreakSealTest = assert(and {
    exactAll("no_seal_h0"), not(sealExact("no_seal_h0")),
    exactAll("no_seal_terminal"), not(sealExact("no_seal_terminal")),
  })
  run mutantCountTest = assert(MUTANTS.size() == 39 + 8)
}
