// X12: GP-2 discharge protocol (kit "GP-2 hook", ARCHITECTURE.md items 1 to 7) with the AD-20
// input-consumption path (owner 2026-11-12: a producer's Halted value is the consumer's input,
// checked by the input link at the seal; the mid-run consume rule stays specified, so the chain
// part of a DL is exercised as a node relation with runs whose deps list is given).
//
// Objects (all node statements are modelled by their fields only):
//   SET roots `sets` (verified), sub-root statements SUB(R, x) `subs`, discharge lists `dls`,
//   segment statements SEGS (fixed: produced by verified segment proofs, NODE-L2 assumed).
// Hash: an injective registry `pre: id -> {tag, args}` with ids from IDBASE up; raw values,
//   sentinels and ids are disjoint ranges. The prover may hash anything (`hashAny`); an opening of
//   x as form F succeeds iff pre(x) has tag F (the injectivity premise). Domain separation is the
//   tag; mutant `ignoreTag` drops it.
// Guards read only statement fields and openings. Ghost data (the run table, DL element lists,
//   the seal log) is read only by invariants.
//
// Abstractions: value digests are raw ints (VALUE DIGEST assumed canonical and injective);
//   input_d = H(IN, val) (fuel and stack limit omitted, INIT range-checks them); premises are
//   ADM sets only (VIEW behaves the same: a premise-only head); FOREIGN is one registered claim;
//   one segmentation per run (n_steps = h_hi).

module ccTypes {
  type Pre = { tag: str, args: List[int] }
  type Elem = { root: int, claim: int }
  type DL = { chain: int, cov: int, link: int, prem: int, ghostElems: List[Elem], ghostLinkRoot: int }
  type Seg = { rid: int, prog: int, input_d: int, deps_d: int, outcome_d: int, h_lo: int, h_hi: int, terminal: bool }
  type Run = { prog: int, inv: int, status: int, outv: int, nsteps: int, deps: List[int] }
  type SealLog = { claim: int, elems: List[Elem], link: int, cov: int }
  type St = {
    pre: int -> Pre, ids: Pre -> int, nextId: int,
    sets: Set[int], subs: Set[(int, int)], dls: Set[DL], seals: Set[SealLog],
    segs: List[Seg], claimRun: int -> int, runClaim: int -> int,
    admOf: int -> int, admRootOf: int -> int, fclaim: int, lastDl: DL
  }
  // Mutant flags: each is a plausible circuit bug named after the kit's mutant list.
  type Mut = {
    noTerminal: bool,       // claim_consume_unsealed_segment: seal skips h_lo == 0 and terminal
    ignoreTag: bool,        // claim_descend_wrong_domain: an opening ignores the domain word
    headOfJoin: bool,       // claim_head_of_join: a JOIN root yields a "head claim"
    subUnrelated: bool,     // claim_sub_root_unrelated: a DL-STEP takes any x under a verified R
    linkAnyStatus: bool,    // claim_input_link_rejected_producer / claim_rejected_as_value
    noValueTie: bool,       // claim_skip_value_opening: witnessed val_d not re-hashed to input_d
    valueMismatch: bool,    // claim_input_link_value_mismatch: outcome' only checked Halted
    nstepsForged: bool,     // claim_nsteps_forged_in_set_leaf
    plainSealNonEmpty: bool, // claim_plain_seal_with_nonempty_chain (= claim_seal_undischarged_deps)
    noChainCheck: bool,     // claim_drop_dependency: SEAL-WITH-DEPS skips chain equality
    premiseInChain: bool,   // claim_premise_in_chain: ADM head accepted as chain or link element
    accum: str,             // "chain" (spec), "sorted" (claim_chain_reordered), "set" (claim_chain_dedup_one_step_for_two)
    coverSkipsLater: bool,  // claim_wrong_set_root: later elements do not join their root into the cover
    linkNoCover: bool,      // probe of the spec text: input-link element does not join R into the cover
    sealDepsEmpty: bool,    // probe: SEAL-WITH-DEPS accepts a premise-only DL (no chain, no link)
  }
  pure val NOMUT: Mut = {
    noTerminal: false, ignoreTag: false, headOfJoin: false, subUnrelated: false,
    linkAnyStatus: false, noValueTie: false, valueMismatch: false, nstepsForged: false,
    plainSealNonEmpty: false, noChainCheck: false, premiseInChain: false, accum: "chain",
    coverSkipsLater: false, linkNoCover: false, sealDepsEmpty: false,
  }
}

module claimConsumption {
  import ccTypes.*
  const MUT: Mut

  // ---------------- ranges and constants ----------------
  pure val IDBASE = 1000
  pure val EMPTY_DEPS = 900
  pure val NO_INPUT_LINK = 901
  pure val NO_PREMISE = 902
  pure val ABSENT = 903
  pure val FRESH = -1          // hash of a preimage nobody registered: equals no id
  pure val HALTED = 1
  pure val REJECTED = 2
  pure val RUNNING = 4
  pure val PA = 11
  pure val PB = 12
  pure val PC = 13
  pure val PROGS = Set(PA, PB, PC)
  pure val VALS = 1.to(12)     // raw values used by the prover's witnesses
  pure val DEPTH = 6           // decode depth bound (traces are short)
  pure val RUN_TAGS = Set("LEAF", "DEP", "FOREIGN")
  pure val HEAD_TAGS = Set("LEAF", "DEP", "FOREIGN", "ADMS", "VIEW")
  pure val ARITY = Map("LEAF" -> 2, "DEP" -> 4, "JOIN" -> 2, "FOREIGN" -> 1, "ADMS" -> 1,
                       "CLAIM" -> 3, "IN" -> 1, "O" -> 3, "ADM" -> 2, "DEPS" -> 2)
  pure val ABSENT_DL: DL = { chain: EMPTY_DEPS, cov: ABSENT, link: NO_INPUT_LINK, prem: NO_PREMISE,
                             ghostElems: [], ghostLinkRoot: ABSENT }

  // The genuine runs (ghost truth). deps lists run indices consumed mid-run (node relation only;
  // under AD-20 every real run has deps []).
  //  0 A: producer, Halted 7.            1 B: input 7 = A's value (honest input link).
  //  2 R: Rejected with value 8.         3 C: input 8 = R's value (no Halted producer).
  //  4 E: input 3, matches no producer.  5 D: deps [A, A] (repetition).
  //  6 F: deps [A, R] (order, Rejected as metadata), input 2 = D's value (link D).
  //  7 G: input 7 (second consumer of A).
  pure val RUNS: List[Run] = [
    { prog: PA, inv: 5, status: HALTED, outv: 7, nsteps: 3, deps: [] },
    { prog: PB, inv: 7, status: HALTED, outv: 9, nsteps: 4, deps: [] },
    { prog: PA, inv: 6, status: REJECTED, outv: 8, nsteps: 2, deps: [] },
    { prog: PB, inv: 8, status: HALTED, outv: 9, nsteps: 4, deps: [] },
    { prog: PB, inv: 3, status: HALTED, outv: 10, nsteps: 4, deps: [] },
    { prog: PC, inv: 1, status: HALTED, outv: 2, nsteps: 5, deps: [0, 0] },
    { prog: PC, inv: 2, status: HALTED, outv: 4, nsteps: 5, deps: [0, 2] },
    { prog: PC, inv: 7, status: HALTED, outv: 11, nsteps: 5, deps: [] },
  ]
  pure val NRUNS = RUNS.length()

  // ---------------- hash registry ----------------
  pure def hashOf(s: St, p: Pre): int = if (s.ids.keys().contains(p)) s.ids.get(p) else FRESH
  pure def reg(s: St, p: Pre): { st: St, id: int } =
    if (s.ids.keys().contains(p)) { st: s, id: s.ids.get(p) }
    else { st: { ...s, pre: s.pre.put(s.nextId, p), ids: s.ids.put(p, s.nextId), nextId: s.nextId + 1 },
           id: s.nextId }
  // Opening of x as a form with tag t: the node hashes the offered preimage and compares with x.
  // Injective hash: only pre(x) passes. With ignoreTag the domain word is not hashed, so any
  // preimage of the same arity whose arguments equal pre(x)'s passes.
  pure def opens(s: St, x: int, t: str): bool =
    s.pre.keys().contains(x) and (s.pre.get(x).tag == t or
      (MUT.ignoreTag and ARITY.keys().contains(s.pre.get(x).tag) and ARITY.get(s.pre.get(x).tag) == ARITY.get(t)))
  pure def argsOf(s: St, x: int): List[int] = s.pre.get(x).args

  // Head opening: tag allowed for a chain or link element.
  pure def headTagOk(t: str): bool =
    RUN_TAGS.contains(t) or (MUT.premiseInChain and Set("ADMS", "VIEW").contains(t))
      or (MUT.headOfJoin and t == "JOIN")
  // Some allowed tag t with opens(s, x, t); returns the head claim (args[0]) or FRESH.
  pure def headOf(s: St, x: int): int =
    if (s.pre.keys().contains(x) and Set("LEAF", "DEP", "FOREIGN", "ADMS", "VIEW", "JOIN").exists(t => headTagOk(t) and opens(s, x, t)))
      argsOf(s, x)[0]
    else FRESH

  // ---------------- dependency accumulator ----------------
  pure def sortedIns(l: List[int], c: int): List[int] =
    l.select(y => y <= c).concat([c]).concat(l.select(y => y > c))
  pure def accStep(s: St, ch: int, c: int): { st: St, id: int } =
    if (MUT.accum == "chain") reg(s, { tag: "DEPS", args: [ch, c] })
    else {
      val prev = if (ch == EMPTY_DEPS) [] else argsOf(s, ch)
      val nl = if (MUT.accum == "set" and prev.select(y => y == c).length() > 0) prev else sortedIns(prev, c)
      reg(s, { tag: "DEPSM", args: nl })
    }

  // ---------------- decode (ghost, invariants only) ----------------
  pure def children(s: St, x: int): Set[int] =
    if (not(s.pre.keys().contains(x))) Set()
    else if (s.pre.get(x).tag == "JOIN") Set(argsOf(s, x)[0], argsOf(s, x)[1])
    else if (s.pre.get(x).tag == "DEP") Set(argsOf(s, x)[2])
    else Set()
  pure def subterms(s: St, r: int): Set[int] =
    range(0, DEPTH).foldl(Set(r), (acc, _i) => acc.union(acc.map(x => children(s, x)).flatten()))
  pure def isHead(s: St, x: int): bool = s.pre.keys().contains(x) and HEAD_TAGS.contains(s.pre.get(x).tag)
  pure def members(s: St, r: int): Set[int] =
    subterms(s, r).filter(x => isHead(s, x)).map(x => argsOf(s, x)[0])

  // ---------------- init ----------------
  pure def regAll(s: St, ps: List[Pre]): St = ps.foldl(s, (a, p) => reg(a, p).st)
  pure def inputD(s: St, v: int): int = hashOf(s, { tag: "IN", args: [v] })
  pure def outcomeD(s: St, st_: int, v: int): int = hashOf(s, { tag: "O", args: [st_, 0, v] })
  pure def claimOfRun(s: St, i: int): int =
    hashOf(s, { tag: "CLAIM", args: [RUNS[i].prog, inputD(s, RUNS[i].inv), outcomeD(s, RUNS[i].status, RUNS[i].outv)] })
  // deps_d of run i under the accumulator in force (the CORE side of item 2).
  pure def depsChain(s: St, i: int): { st: St, id: int } =
    RUNS[i].deps.foldl({ st: s, id: EMPTY_DEPS }, (a, j) => accStep(a.st, a.id, claimOfRun(a.st, j)))

  pure val EMPTY_ST: St = {
    pre: Map(), ids: Map(), nextId: IDBASE, sets: Set(), subs: Set(), dls: Set(), seals: Set(),
    segs: [], claimRun: Map(), runClaim: Map(), admOf: Map(), admRootOf: Map(), fclaim: FRESH,
    lastDl: ABSENT_DL,
  }
  pure val INIT_ST: St = {
    // digests of every genuine run
    val s1 = range(0, NRUNS).foldl(EMPTY_ST, (a, i) => regAll(a, [
      { tag: "IN", args: [RUNS[i].inv] },
      { tag: "O", args: [RUNS[i].status, 0, RUNS[i].outv] }]))
    val s2 = range(0, NRUNS).foldl(s1, (a, i) => reg(a, { tag: "CLAIM",
      args: [RUNS[i].prog, inputD(a, RUNS[i].inv), outcomeD(a, RUNS[i].status, RUNS[i].outv)] }).st)
    val s3 = { ...s2,
      claimRun: range(0, NRUNS).foldl(Map(), (m, i) => m.put(claimOfRun(s2, i), i)),
      runClaim: range(0, NRUNS).foldl(Map(), (m, i) => m.put(i, claimOfRun(s2, i))) }
    // deps chains
    val s4 = range(0, NRUNS).foldl(s3, (a, i) => depsChain(a, i).st)
    // admissions (ADMIT-WORK gives a verified ADM set per admitted program) and one foreign claim
    val s5 = PROGS.fold(s4, (a, p) => {
      val r1 = reg(a, { tag: "ADM", args: [1, p] })
      val r2 = reg(r1.st, { tag: "ADMS", args: [r1.id] })
      { ...r2.st, admOf: r2.st.admOf.put(p, r1.id), admRootOf: r2.st.admRootOf.put(p, r2.id),
        sets: r2.st.sets.union(Set(r2.id)), subs: r2.st.subs.union(Set((r2.id, r2.id))) }
    })
    val f1 = reg(s5, { tag: "FCLAIM", args: [77] })
    val f2 = reg(f1.st, { tag: "FOREIGN", args: [f1.id] })
    val s6 = { ...f2.st, fclaim: f1.id, sets: f2.st.sets.union(Set(f2.id)), subs: f2.st.subs.union(Set((f2.id, f2.id))) }
    // segment statements: one terminal genesis segment per run, plus two that must never seal:
    // a non-terminal genesis segment of A and a terminal tail segment of A (h_lo = 2) from a
    // state that is not A's genesis, ending Halted 99.
    val s7 = regAll(s6, [{ tag: "O", args: [RUNNING, 0, 42] }, { tag: "O", args: [HALTED, 0, 99] }])
    val genuineSegs = range(0, NRUNS).foldl([], (l, i) => l.append({
      rid: i, prog: RUNS[i].prog, input_d: inputD(s7, RUNS[i].inv), deps_d: depsChain(s7, i).id,
      outcome_d: outcomeD(s7, RUNS[i].status, RUNS[i].outv), h_lo: 0, h_hi: RUNS[i].nsteps, terminal: true }))
    val badSegs = [
      { rid: -1, prog: PA, input_d: inputD(s7, 5), deps_d: EMPTY_DEPS, outcome_d: outcomeD(s7, RUNNING, 42),
        h_lo: 0, h_hi: 1, terminal: false },
      { rid: -2, prog: PA, input_d: inputD(s7, 5), deps_d: EMPTY_DEPS, outcome_d: outcomeD(s7, HALTED, 99),
        h_lo: 2, h_hi: 3, terminal: true },
    ]
    { ...s7, segs: genuineSegs.concat(badSegs) }
  }

  var st: St
  action init = st' = INIT_ST

  // ---------------- node relations (guards read statement fields and openings only) ----------------
  // Premise check of a seal: x opens as ADMS(adm_d) with adm_d = H(ADM, 1, prog_d) of the segment.
  pure def premOk(s: St, x: int, prog: int): bool =
    opens(s, x, "ADMS") and argsOf(s, x)[0] == hashOf(s, { tag: "ADM", args: [1, prog] })
  pure def segSealable(sg: Seg): bool = MUT.noTerminal or (sg.h_lo == 0 and sg.terminal)

  pure def addSet(s: St, r: int): St = { ...s, sets: s.sets.union(Set(r)), subs: s.subs.union(Set((r, r))) }

  // SEAL: SEG -> LEAF(claim_d, n_steps). Premise in slot 1 (a SUB statement).
  pure def canSeal(s: St, k: int, psub: (int, int), n: int): bool = {
    val sg = s.segs[k]
    all {
      segSealable(sg),
      MUT.plainSealNonEmpty or sg.deps_d == EMPTY_DEPS,
      s.subs.contains(psub), premOk(s, psub._2, sg.prog),
      MUT.nstepsForged or n == sg.h_hi,
    }
  }
  pure def applySeal(s: St, k: int, n: int): St = {
    val sg = s.segs[k]
    val c = reg(s, { tag: "CLAIM", args: [sg.prog, sg.input_d, sg.outcome_d] })
    val l = reg(c.st, { tag: "LEAF", args: [c.id, n] })
    addSet(l.st, l.id)
  }
  action seal(k: int, psub: (int, int), n: int): bool = all {
    canSeal(st, k, psub, n), st' = applySeal(st, k, n) }

  // JOIN: SET + SET -> SET.
  action join(l: int, r: int): bool = all {
    st.sets.contains(l), st.sets.contains(r),
    val j = reg(st, { tag: "JOIN", args: [l, r] })
    st' = addSet(j.st, j.id),
  }

  // DESCEND: SUB(R, x) and an opening of x as JOIN (side 0 or 1) or DEP (cover) -> SUB(R, child).
  pure def descendChild(s: St, x: int, t: str, side: int): int =
    if (t == "JOIN") argsOf(s, x)[side] else argsOf(s, x)[2]
  action descend(sb: (int, int), t: str, side: int): bool = all {
    st.subs.contains(sb), Set("JOIN", "DEP").contains(t), opens(st, sb._2, t),
    st' = { ...st, subs: st.subs.union(Set((sb._1, descendChild(st, sb._2, t, side)))) },
  }

  // A SUB offered to a DL-STEP: derived, or (mutant) any x under a verified root.
  pure def subOk(s: St, sb: (int, int)): bool =
    s.subs.contains(sb) or (MUT.subUnrelated and s.sets.contains(sb._1) and s.pre.keys().contains(sb._2))

  pure def isAbsent(d: DL): bool = d.cov == ABSENT and d.chain == EMPTY_DEPS and d.link == NO_INPUT_LINK and d.prem == NO_PREMISE
  pure def dlKnown(s: St, d: DL): bool = isAbsent(d) or s.dls.contains(d)
  pure def addDl(s: St, d: DL): St = { ...s, dls: s.dls.union(Set(d)), lastDl: d }

  // cover update: cov' = R if absent else JOIN(cov, R); mutant coverSkipsLater keeps cov.
  pure def coverStep(s: St, cov: int, r: int, joinIt: bool): { st: St, id: int } =
    if (not(joinIt)) { st: s, id: cov }
    else if (cov == ABSENT) { st: s, id: r }
    else if (MUT.coverSkipsLater) { st: s, id: cov }
    else reg(s, { tag: "JOIN", args: [cov, r] })

  // DL-STEP, premise element: first element of a list (from the absent DL only).
  action dlPremise(sb: (int, int)): bool = all {
    st.subs.contains(sb),
    st' = addDl(st, { ...ABSENT_DL, prem: sb._2 }),
  }

  // DL-STEP, chain element.
  pure def canChain(s: St, d: DL, sb: (int, int)): bool =
    dlKnown(s, d) and subOk(s, sb) and headOf(s, sb._2) != FRESH
  pure def applyChain(s: St, d: DL, sb: (int, int)): St = {
    val c = headOf(s, sb._2)
    val ch = accStep(s, d.chain, c)
    val cv = coverStep(ch.st, d.cov, sb._1, true)
    addDl(cv.st, { ...d, chain: ch.id, cov: cv.id,
                   ghostElems: d.ghostElems.append({ root: sb._1, claim: c }) })
  }
  action dlChain(d: DL, sb: (int, int)): bool = all { canChain(st, d, sb), st' = applyChain(st, d, sb) }

  // DL-STEP, input-link element: at most one, before any chain element.
  pure def canLink(s: St, d: DL, sb: (int, int)): bool =
    dlKnown(s, d) and d.link == NO_INPUT_LINK and d.chain == EMPTY_DEPS and subOk(s, sb) and headOf(s, sb._2) != FRESH
  pure def applyLink(s: St, d: DL, sb: (int, int)): St = {
    val c = headOf(s, sb._2)
    val cv = coverStep(s, d.cov, sb._1, not(MUT.linkNoCover))
    addDl(cv.st, { ...d, link: c, cov: cv.id, ghostLinkRoot: sb._1 })
  }
  action dlLink(d: DL, sb: (int, int)): bool = all { canLink(st, d, sb), st' = applyLink(st, d, sb) }

  // SEAL-WITH-DEPS: SEG + DL -> DEP(claim_d, n_steps, cover_root, in_link).
  // Witnesses: wv (the consumer's val_d behind input_d) and the producer claim preimage, which is
  // pre(link) by injectivity (any other offered preimage fails the re-hash).
  pure def linkOk(s: St, sg: Seg, link: int, wv: int): bool =
    if (link == NO_INPUT_LINK) true
    else all {
      MUT.noValueTie or hashOf(s, { tag: "IN", args: [wv] }) == sg.input_d,
      opens(s, link, "CLAIM"),
      val outc = argsOf(s, link)[2]
      if (MUT.linkAnyStatus)
        s.pre.keys().contains(outc) and s.pre.get(outc).tag == "O" and argsOf(s, outc)[2] == wv
      else if (MUT.valueMismatch)
        s.pre.keys().contains(outc) and s.pre.get(outc).tag == "O" and argsOf(s, outc)[0] == HALTED
      else outc == hashOf(s, { tag: "O", args: [HALTED, 0, wv] }),
    }
  pure def canSealDeps(s: St, k: int, d: DL, wv: int, n: int): bool = {
    val sg = s.segs[k]
    all {
      s.dls.contains(d),
      segSealable(sg),
      d.prem != NO_PREMISE, premOk(s, d.prem, sg.prog),
      MUT.sealDepsEmpty or d.cov != ABSENT,
      MUT.noChainCheck or d.chain == sg.deps_d,
      linkOk(s, sg, d.link, wv),
      MUT.nstepsForged or n == sg.h_hi,
    }
  }
  pure def applySealDeps(s: St, k: int, d: DL, n: int): St = {
    val sg = s.segs[k]
    val c = reg(s, { tag: "CLAIM", args: [sg.prog, sg.input_d, sg.outcome_d] })
    val r = reg(c.st, { tag: "DEP", args: [c.id, n, d.cov, d.link] })
    val s2 = addSet(r.st, r.id)
    { ...s2, seals: s2.seals.union(Set({ claim: c.id, elems: d.ghostElems, link: d.link, cov: d.cov })) }
  }
  action sealDeps(k: int, d: DL, wv: int, n: int): bool = all {
    canSealDeps(st, k, d, wv, n), st' = applySealDeps(st, k, d, n) }

  // The prover hashes anything (raw values and known ids as arguments).
  action hashAny(t: str, a0: int, a1: int, a2: int, a3: int): bool = all {
    ARITY.keys().contains(t),
    st' = reg(st, { tag: t, args: [a0, a1, a2, a3].slice(0, ARITY.get(t)) }).st,
  }

  // ---------------- step ----------------
  action step = {
    nondet k = 0.to(st.segs.length() - 1).oneOf()
    nondet sb = st.subs.oneOf()
    nondet psub = st.subs.oneOf()
    nondet d = st.dls.union(Set(ABSENT_DL)).oneOf()
    nondet l = st.sets.oneOf()
    nondet r = st.sets.oneOf()
    nondet t = Set("JOIN", "DEP").oneOf()
    nondet side = Set(0, 1).oneOf()
    nondet wv = VALS.oneOf()
    nondet nd = Set(1, 3, 4, 5, 9).oneOf()
    nondet ht = ARITY.keys().oneOf()
    val pool = st.pre.keys().union(VALS)
    nondet a0 = pool.oneOf()
    nondet a1 = pool.oneOf()
    nondet a2 = pool.oneOf()
    nondet a3 = pool.oneOf()
    // honest n_steps unless the mutant lets the prover choose
    val n = if (MUT.nstepsForged) nd else st.segs[k].h_hi
    any {
      seal(k, psub, n),
      seal(k, (st.admRootOf.get(st.segs[k].prog), st.admRootOf.get(st.segs[k].prog)), n),
      join(l, r),
      descend(sb, t, side),
      dlPremise((st.admRootOf.get(st.segs[k].prog), st.admRootOf.get(st.segs[k].prog))),
      dlPremise(psub),
      dlChain(d, sb),
      dlLink(d, sb),
      sealDeps(k, d, wv, n),
      sealDeps(k, st.lastDl, wv, n),
      hashAny(ht, a0, a1, a2, a3),
    }
  }

  // ---------------- invariants (ghost: the run table and the decode) ----------------
  pure def isRunClaim(s: St, c: int): bool = s.claimRun.keys().contains(c)
  pure def runOf(s: St, c: int): Run = RUNS[s.claimRun.get(c)]
  pure def depsClaims(s: St, c: int): List[int] = runOf(s, c).deps.foldl([], (l, j) => l.append(s.runClaim.get(j)))
  pure def admClaims(s: St): Set[int] = PROGS.map(p => s.admOf.get(p))
  pure def verifiedSubterms(s: St): Set[int] = s.sets.map(r => subterms(s, r)).flatten()
  pure def tagIs(s: St, x: int, t: str): bool = s.pre.keys().contains(x) and s.pre.get(x).tag == t

  // SegmentNeverConsumed (and the member half of GP2-L1): every run head under a verified root is
  // the claim of a genuine run from Init to a terminal state, with that run's n_steps; FOREIGN
  // heads are registered; ADM heads are admissions.
  val SegmentNeverConsumed: bool = verifiedSubterms(st).forall(y =>
    if (tagIs(st, y, "LEAF") or tagIs(st, y, "DEP"))
      isRunClaim(st, argsOf(st, y)[0]) and argsOf(st, y)[1] == runOf(st, argsOf(st, y)[0]).nsteps
    else if (tagIs(st, y, "FOREIGN")) argsOf(st, y)[0] == st.fclaim
    else if (tagIs(st, y, "ADMS")) admClaims(st).contains(argsOf(st, y)[0])
    else true)

  // GP2-L2: every SUB(R, x) has R verified and x a sub-term of R's grammar term.
  val SubRootSound: bool = st.subs.forall(sb => st.sets.contains(sb._1) and subterms(st, sb._1).contains(sb._2))

  // DepsDischargedIffVerifiedSet: every DL element (chain or link) is the head claim of a verified
  // SET, a run or foreign claim (never a premise), and its root is joined into the DL's cover; and
  // in every verified DEP the run's dependencies and its input link are members of the cover.
  pure def elemOk(s: St, cov: int, e: Elem): bool = all {
    s.sets.contains(e.root), members(s, e.root).contains(e.claim),
    isRunClaim(s, e.claim) or e.claim == s.fclaim,
    cov != ABSENT, subterms(s, cov).contains(e.root),
  }
  val DepsDischargedIffVerifiedSet: bool = all {
    st.dls.forall(d => all {
      d.ghostElems.indices().forall(i => elemOk(st, d.cov, d.ghostElems[i])),
      d.link == NO_INPUT_LINK or elemOk(st, d.cov, { root: d.ghostLinkRoot, claim: d.link }),
    }),
    verifiedSubterms(st).forall(y => tagIs(st, y, "DEP") implies {
      val c = argsOf(st, y)[0]
      val cov = argsOf(st, y)[2]
      val link = argsOf(st, y)[3]
      isRunClaim(st, c) implies all {
        depsClaims(st, c).indices().forall(i => members(st, cov).contains(depsClaims(st, c)[i])),
        link == NO_INPUT_LINK or members(st, cov).contains(link),
      }
    }),
  }

  // ChainOrderAndRepetitionExact (GP2-L3): the discharged head claims of every seal equal the
  // consumer's consumed claims, with order and repetition.
  val ChainOrderAndRepetitionExact: bool = st.seals.forall(lg =>
    isRunClaim(st, lg.claim) implies lg.elems.foldl([], (l, e) => l.append(e.claim)) == depsClaims(st, lg.claim))

  // EmptyDepsOnlyBySeal: a LEAF is a run with no dependency; a DEP has a dependency or a link.
  val EmptyDepsOnlyBySeal: bool = verifiedSubterms(st).forall(y =>
    if (tagIs(st, y, "LEAF")) (isRunClaim(st, argsOf(st, y)[0]) implies runOf(st, argsOf(st, y)[0]).deps == [])
    else if (tagIs(st, y, "DEP")) argsOf(st, y)[2] != ABSENT
      and (isRunClaim(st, argsOf(st, y)[0]) implies
        (runOf(st, argsOf(st, y)[0]).deps != [] or argsOf(st, y)[3] != NO_INPUT_LINK))
    else true)

  // InputLinkHaltedOnly (the AD-20 soundness statement): every verified DEP with an input link
  // names a genuine run that Halted with exactly the consumer's input value.
  val InputLinkHaltedOnly: bool = verifiedSubterms(st).forall(y =>
    (tagIs(st, y, "DEP") and argsOf(st, y)[3] != NO_INPUT_LINK) implies {
      val c = argsOf(st, y)[0]
      val link = argsOf(st, y)[3]
      isRunClaim(st, link) and isRunClaim(st, c) and runOf(st, link).status == HALTED
        and runOf(st, link).outv == runOf(st, c).inv
    })

  val soundAll: bool = SegmentNeverConsumed and SubRootSound and DepsDischargedIffVerifiedSet
    and ChainOrderAndRepetitionExact and EmptyDepsOnlyBySeal and InputLinkHaltedOnly

  // ---------------- test helpers (ids of honest objects in the current state) ----------------
  def cl(i: int): int = st.runClaim.get(i)
  def leafOf(i: int): int = hashOf(st, { tag: "LEAF", args: [cl(i), RUNS[i].nsteps] })
  def joinOf(l: int, r: int): int = hashOf(st, { tag: "JOIN", args: [l, r] })
  def admSub(p: int): (int, int) = (st.admRootOf.get(p), st.admRootOf.get(p))
  def selfSub(r: int): (int, int) = (r, r)
  // the DEP root of run i sealed with the last DL
  def depOf(i: int): int = hashOf(st, { tag: "DEP", args: [cl(i), RUNS[i].nsteps, st.lastDl.cov, st.lastDl.link] })
  action sealRun(i: int): bool = seal(i, admSub(RUNS[i].prog), RUNS[i].nsteps)
  def membersOf(r: int): Set[int] = members(st, r)

  // ---------------- non-vacuity (negated reach*: must be VIOLATED in the sound instance) ----------------
  pure def depWithLink(s: St, consumer: int, producer: int): bool =
    s.sets.exists(r => subterms(s, r).exists(y => tagIs(s, y, "DEP")
      and argsOf(s, y)[0] == s.runClaim.get(consumer) and argsOf(s, y)[3] == s.runClaim.get(producer)))
  val reachBConsumesA: bool = not(depWithLink(st, 1, 0))
  val reachJoin: bool = not(st.sets.exists(r => tagIs(st, r, "JOIN")))
  val reachDescend: bool = not(st.subs.exists(sb => sb._1 != sb._2))
  val reachChainSeal: bool = not(st.seals.exists(lg => lg.elems.length() > 0))
}

module cc_sound { import ccTypes.* import claimConsumption(MUT = NOMUT).* }
module cc_noTerminal { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, noTerminal: true }).* }
module cc_ignoreTag { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, ignoreTag: true }).* }
module cc_headOfJoin { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, headOfJoin: true }).* }
module cc_subUnrelated { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, subUnrelated: true }).* }
module cc_linkAnyStatus { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, linkAnyStatus: true }).* }
module cc_noValueTie { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, noValueTie: true }).* }
module cc_valueMismatch { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, valueMismatch: true }).* }
module cc_nstepsForged { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, nstepsForged: true }).* }
module cc_plainSealNonEmpty { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, plainSealNonEmpty: true }).* }
module cc_noChainCheck { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, noChainCheck: true }).* }
module cc_premiseInChain { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, premiseInChain: true }).* }
module cc_accumSorted { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, accum: "sorted" }).* }
module cc_accumSet { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, accum: "set" }).* }
module cc_coverSkipsLater { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, coverSkipsLater: true }).* }
module cc_linkNoCover { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, linkNoCover: true }).* }
module cc_sealDepsEmpty { import ccTypes.* import claimConsumption(MUT = { ...NOMUT, sealDepsEmpty: true }).* }
