// S-slot leaf statement and slot contract (task 4.2a, SRC-001, SRC-003, SRC-014, SVM-003).
// Design: openspec/changes/sherlock-vm-consensus-20261107/design.md §2.2(3), §2.2(6), §2.3,
// §3.2(3), amended 2026-11-10 by the design review
// (research/sherlock-recursion-design-review-20261110/review.md C2, §5 design edit 3).
//
// What the model checks: the acceptance predicate of one S-slot universal leaf (`accepts`)
// implies that the public boundary (pre, post, rulesD, steps, halted) equals an independent
// reference run of `steps` owner steps from `pre` (`refRules`, `refPost`, `refTerminalLast`).
// The constraints of `accepts` are the slot contract:
//   C1 one-hot:       sum_r sel_j[r] == enable_j for every slot j (a disabled slot selects nothing)
//   C2 selector tie:  sel_j[r] == 1 implies the guard of rule r holds on pre_j
//   C3 enable prefix: enable_0 == 1 and enable_{j+1} * (1 - enable_j) == 0
//   C4 publication:   enable_{j+1} == 1 implies post_j is Running, so a publishing step is the
//                     last enabled slot
//   C5 transition:    enabled: post_j == applyRule(pre_j, rid_j); disabled: post_j == pre_j
//   C6 chain:         post_j == pre_{j+1}; pre == pre_0; post == post_{S-1}
//   C7 rulesD:        rulesD == [rid_0, ..., rid_{S-1}], exactly S entries in slot order, with
//                     rid_j == sum_r sel_j[r] * lane(r); a disabled slot therefore encodes the
//                     reserved id DISABLED == 0, which is not the lane of any admitted rule
//   C8 steps:         steps == number of enabled slots
//   C9 outcome:       the published outcome is read from the constrained post carrier
// rulesD is modeled as the list itself (the Poseidon digest is injective on the domain, a premise).
//
// Witness generation: an honest witness for an arbitrary enable vector (gaps included), plus at
// most one tamper of one field. Random witnesses would almost never satisfy `accepts`, so the
// generator stays near the accepting set; each mutant drops one constraint and must let a
// tampered or gapped witness through, which a soundness invariant then refutes.
//
// Premises (not modeled): dispatch correctness (`coreRule` is Core's selection and the guards of
// the admitted rules are mutually exclusive), digest injectivity, fragment soundness of
// `applyRule`, and the node and decider (F20, F21). This is model evidence for the slot contract
// only; it is not a refinement proof against the Lean `Leaf`.
//
// Check: quint typecheck, then quint run --main <module> --invariant <inv> per module below.
// leafSlotsBase / leafSlotsS1 / leafSlotsS8: every soundness invariant holds; the reach*
// invariants are expected to FAIL (they show that accepting witnesses of each shape exist).
// leafSlots_<mutant>: the named invariant is expected to FAIL (mutant caught).

module leafSlots {
  const S: int
  const MUTANT: str

  // Rule lanes. 0 is reserved for a disabled slot. Rule 3 publishes (sets the outcome Halted).
  pure val RULES: Set[int] = Set(1, 2, 3)
  pure val DISABLED: int = 0
  pure val PUB: int = 3
  pure val TAMPERS: Set[str] = Set("none", "drop_sel", "sel_on_disabled", "wrong_sel",
                                   "disabled_write", "steps", "rules", "outcome")

  type St = { v: int, halted: bool }
  type Slot = { en: bool, sel: int -> int, pre: St, post: St }
  type W = { slots: List[Slot], rulesD: List[int], steps: int, pre: St, post: St, halted: bool }

  // Core selection on a Running state (premise: dispatch is correct and exclusive).
  pure def coreRule(s: St): int = (s.v % 3) + 1
  pure def guardHolds(r: int, s: St): bool = r == coreRule(s)
  pure def applyRule(s: St, r: int): St = { v: s.v + r, halted: r == PUB }
  pure def otherRule(s: St): int = (coreRule(s) % 3) + 1

  pure def sumSel(sl: Slot): int = RULES.fold(0, (a, r) => a + sl.sel.get(r))
  pure def rid(sl: Slot): int = RULES.fold(0, (a, r) => a + sl.sel.get(r) * r)
  pure def bit(b: bool): int = if (b) 1 else 0
  pure def noSel: int -> int = RULES.mapBy(r => 0)
  pure def oneSel(x: int): int -> int = RULES.mapBy(r => if (r == x) 1 else 0)

  // Encoding of one slot in rulesD as the circuit computes it. Mutant disabled_id_collides
  // gives a disabled slot the lane of rule 1 instead of the reserved id.
  pure def encSlot(sl: Slot): int =
    if (MUTANT == "disabled_id_collides" and not(sl.en)) 1 else rid(sl)
  pure def countEn(sls: List[Slot]): int = sls.foldl(0, (n, sl) => n + bit(sl.en))

  // ---------------------------------------------------------------- reference run
  pure def refState(s0: St, k: int): St =
    0.to(S).fold(s0, (s, j) => if (j < k) applyRule(s, coreRule(s)) else s)
  pure def refPost(s0: St, k: int): St = refState(s0, k)
  pure def refRules(s0: St, k: int): List[int] =
    range(0, S).foldl([], (l, j) => l.append(if (j < k) coreRule(refState(s0, j)) else DISABLED))
  // Only the last of the k steps may publish: every step starts from a Running state.
  pure def refTerminalLast(s0: St, k: int): bool =
    0.to(S - 1).forall(j => j >= k or not(refState(s0, j).halted))

  // ---------------------------------------------------------------- the leaf relation
  pure def accepts(w: W): bool = and {
    w.slots.length() == S,
    w.rulesD.length() == S,
    // C1 one-hot: sum(sel) == enable
    MUTANT == "no_onehot" or w.slots.indices().forall(j => sumSel(w.slots[j]) == bit(w.slots[j].en)),
    // C2 selector tie to the pre-state guard
    MUTANT == "no_selector_tie" or w.slots.indices().forall(j =>
      RULES.forall(r => w.slots[j].sel.get(r) == 0 or guardHolds(r, w.slots[j].pre))),
    // C3 enable prefix
    w.slots[0].en,
    MUTANT == "no_enable_prefix" or w.slots.indices().forall(j =>
      j + 1 == S or not(w.slots[j + 1].en) or w.slots[j].en),
    // C4 publication only in the last enabled slot
    MUTANT == "publish_anywhere" or w.slots.indices().forall(j =>
      j + 1 == S or not(w.slots[j + 1].en) or not(w.slots[j].post.halted)),
    // C5 transition, disabled slot identity
    w.slots.indices().forall(j =>
      if (w.slots[j].en) w.slots[j].post == applyRule(w.slots[j].pre, rid(w.slots[j]))
      else (MUTANT == "no_disabled_identity" or w.slots[j].post == w.slots[j].pre)),
    // C6 internal chain and boundary
    w.slots.indices().forall(j => j + 1 == S or w.slots[j].post == w.slots[j + 1].pre),
    w.pre == w.slots[0].pre,
    w.post == w.slots[S - 1].post,
    // C7 rulesD: exactly S slot ids in order
    MUTANT == "rules_unbound" or w.slots.indices().forall(j => w.rulesD[j] == encSlot(w.slots[j])),
    // C8 steps
    MUTANT == "steps_unbound" or w.steps == countEn(w.slots),
    // C9 outcome from the constrained post carrier
    MUTANT == "outcome_unbound" or w.halted == w.post.halted,
  }

  // ---------------------------------------------------------------- witness generator
  pure def mkSlot(s: St, en: bool, j: int, t: int, tk: str): Slot =
    if (j == t and tk == "drop_sel" and en)
      { en: true, sel: noSel, pre: s, post: applyRule(s, 0) }
    else if (j == t and tk == "sel_on_disabled" and not(en))
      { en: false, sel: oneSel(coreRule(s)), pre: s, post: s }
    else if (j == t and tk == "wrong_sel" and en)
      { en: true, sel: oneSel(otherRule(s)), pre: s, post: applyRule(s, otherRule(s)) }
    else if (j == t and tk == "disabled_write" and not(en))
      { en: false, sel: noSel, pre: s, post: { v: s.v + 1, halted: s.halted } }
    else if (en)
      { en: true, sel: oneSel(coreRule(s)), pre: s, post: applyRule(s, coreRule(s)) }
    else
      { en: false, sel: noSel, pre: s, post: s }

  pure def mkSlots(s0: St, ens: List[bool], t: int, tk: str): List[Slot] =
    range(0, S).foldl([], (l, j) =>
      val s = if (j == 0) s0 else l[j - 1].post
      l.append(mkSlot(s, ens[j], j, t, tk)))

  pure def mkWitness(s0: St, ens: List[bool], t: int, tk: str): W =
    val sls = mkSlots(s0, ens, t, tk)
    val enc = sls.foldl([], (l, sl) => l.append(encSlot(sl)))
    val st = countEn(sls)
    val post = sls[S - 1].post
    {
      slots: sls,
      rulesD: if (tk == "rules") enc.replaceAt(t, (enc[t] % 3) + 1) else enc,
      steps: if (tk == "steps") st + 1 else st,
      pre: s0,
      post: post,
      halted: if (tk == "outcome") not(post.halted) else post.halted,
    }

  var w: W
  var acc: bool
  var tamper: str

  action sample = {
    nondet v0 = 0.to(5).oneOf()
    nondet enMap = 0.to(S - 1).setOfMaps(Set(true, false)).oneOf()
    nondet t = 0.to(S - 1).oneOf()
    nondet tk = TAMPERS.oneOf()
    val ens = range(0, S).foldl([], (l, j) => l.append(enMap.get(j)))
    val x = mkWitness({ v: v0, halted: false }, ens, t, tk)
    val honest = mkWitness({ v: v0, halted: false }, ens, t, "none")
    all {
      w' = x,
      acc' = accepts(x),
      // a tamper that leaves the witness unchanged (wrong slot kind) counts as none
      tamper' = if (x == honest) "none" else tk,
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- soundness invariants
  val invStepsRange: bool = acc implies (1 <= w.steps and w.steps <= S)
  val invPrefix: bool = acc implies w.slots.indices().forall(j => w.slots[j].en == (j < w.steps))
  val invOneHot: bool = acc implies w.slots.indices().forall(j => sumSel(w.slots[j]) == bit(w.slots[j].en))
  val invRules: bool = acc implies w.rulesD == refRules(w.pre, w.steps)
  val invPost: bool = acc implies w.post == refPost(w.pre, w.steps)
  val invTerminalLast: bool = acc implies refTerminalLast(w.pre, w.steps)
  val invOutcome: bool = acc implies w.halted == refPost(w.pre, w.steps).halted
  val invSound: bool = and {
    invStepsRange, invPrefix, invOneHot, invRules, invPost, invTerminalLast, invOutcome,
  }
  // Every tamper is refused by the unmutated relation.
  val invTamperRefused: bool = (acc and MUTANT == "none") implies tamper == "none"

  // ---------------------------------------------------------------- non-vacuity (expected to FAIL)
  val reachShort: bool = not(acc and w.steps < S)
  val reachTerminalShort: bool = not(acc and w.halted and w.steps < S)
  val reachFull: bool = not(acc and w.steps == S)
}

module leafSlotsBase { import leafSlots(S = 4, MUTANT = "none").* }
module leafSlotsS1 { import leafSlots(S = 1, MUTANT = "none").* }
module leafSlotsS8 { import leafSlots(S = 8, MUTANT = "none").* }

module leafSlots_no_onehot { import leafSlots(S = 4, MUTANT = "no_onehot").* }
module leafSlots_no_selector_tie { import leafSlots(S = 4, MUTANT = "no_selector_tie").* }
module leafSlots_no_enable_prefix { import leafSlots(S = 4, MUTANT = "no_enable_prefix").* }
module leafSlots_publish_anywhere { import leafSlots(S = 4, MUTANT = "publish_anywhere").* }
module leafSlots_no_disabled_identity { import leafSlots(S = 4, MUTANT = "no_disabled_identity").* }
module leafSlots_disabled_id_collides { import leafSlots(S = 4, MUTANT = "disabled_id_collides").* }
module leafSlots_rules_unbound { import leafSlots(S = 4, MUTANT = "rules_unbound").* }
module leafSlots_steps_unbound { import leafSlots(S = 4, MUTANT = "steps_unbound").* }
module leafSlots_outcome_unbound { import leafSlots(S = 4, MUTANT = "outcome_unbound").* }
