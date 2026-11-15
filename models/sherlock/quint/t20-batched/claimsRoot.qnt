// T20 fix 2 (and fix 3, foreign leaves): openClaim is replaced by membership against the collection's
// claims_root. SherlockLedger is written once per epoch (one claims_root) and once per user-paid acceptRun;
// a consumer proves membership inside its own call and keeps its own nullifier keyed by the claim alone.
// The claim list and the paths are off chain (data availability, `da`); a claim is usable only after its
// epoch's collection is accepted (epoch wait).
//
// Merkle soundness is the assumption: a path to an accepted root exists only for a member of that root
// (fixed depth D_C, H2 collision resistance). Run leaves and foreign leaves are separated by the leaf form
// (run leaf = claim_pub, a 3-input hash; foreign leaf = H2(SHK_CLAIM_FGN, fclaim_pub)); the model keeps the
// kind in the leaf and the consumer's entry fixes the kind it accepts.
//
// MUT:
//   "none"                  the design
//   "per_claim_open"        openClaim kept: each use also writes the ledger (audit row 4)
//   "root_keyed_nullifier"  the consumer's nullifier is keyed by (root, claim)
//   "no_root_check"         the consumer does not read rootAccepted (any root is trusted)
//   "unbound_root"          L does not bind claims_root to set_root: a non-member enters the claims tree
//   "no_kind_separation"    (fix 3) a foreign leaf and a run leaf share one form, so a foreign claim
//                           passes as a run claim with the same value
module claimsRoot {
  const MUT: str

  type Leaf = { kind: str, v: int }       // kind "RUN" or "FGN"

  pure val TRUE_RUNS = Set(1, 2, 3)       // run claims that L can open from a verified set_root
  pure val TRUE_FGN = Set(7)              // foreign claims that a registered verifier accepted (GP-3)
  pure val FORGED = 9                     // a value that is no verified claim of any kind
  pure val CONSUMERS = Set(1, 2)
  pure val MAX_EPOCH = 4

  var epoch: int
  var pending: Set[Leaf]                  // members proved in the open epoch (off chain)
  var accepted: Set[int]                  // claim_roots (by epoch number) on chain
  var members: int -> Set[Leaf]           // off-chain claim list of each accepted root
  var da: Set[int]                        // roots whose claim list and paths are available
  var runs: Set[int]                      // acceptRun claims (user-paid)
  var used: Set[(int, str, int, int)]     // consumer nullifiers: (consumer, kind, value, rootOrMinus1)
  var uses: List[(int, Leaf, int)]        // ghost: every successful use (consumer, leaf as used, root or -1)
  var ledgerWrites: int
  var busyEpochs: int
  var userRuns: int

  action init = all {
    epoch' = 0, pending' = Set(), accepted' = Set(), members' = Map(), da' = Set(), runs' = Set(),
    used' = Set(), uses' = List(), ledgerWrites' = 0, busyEpochs' = 0, userRuns' = 0,
  }

  // the operator proves a member (run or foreign claim) in the open epoch
  action proveMember(l: Leaf): bool = all {
    epoch < MAX_EPOCH,
    if (l.kind == "RUN") TRUE_RUNS.contains(l.v) else TRUE_FGN.contains(l.v),
    pending' = pending.union(Set(l)),
    epoch' = epoch, accepted' = accepted, members' = members, da' = da, runs' = runs, used' = used, uses' = uses,
    ledgerWrites' = ledgerWrites, busyEpochs' = busyEpochs, userRuns' = userRuns,
  }

  // epoch close: acceptCollection(reg_pub, claims_root), one write; the claim list is published off chain
  action closeEpoch(publish: bool): bool = all {
    epoch < MAX_EPOCH, pending.size() > 0,
    val ms = if (MUT == "unbound_root") pending.union(Set({ kind: "RUN", v: FORGED })) else pending
    all {
      accepted' = accepted.union(Set(epoch)), members' = members.put(epoch, ms),
      da' = if (publish) da.union(Set(epoch)) else da,
    },
    ledgerWrites' = ledgerWrites + 1, busyEpochs' = busyEpochs + 1,
    pending' = Set(), epoch' = epoch + 1,
    runs' = runs, used' = used, uses' = uses, userRuns' = userRuns,
  }

  action skipEpoch: bool = all {
    epoch < MAX_EPOCH, pending.size() == 0, epoch' = epoch + 1,
    pending' = pending, accepted' = accepted, members' = members, da' = da, runs' = runs, used' = used, uses' = uses,
    ledgerWrites' = ledgerWrites, busyEpochs' = busyEpochs, userRuns' = userRuns,
  }

  action userAcceptRun(c: int): bool = all {
    TRUE_RUNS.contains(c), not(runs.contains(c)),
    runs' = runs.union(Set(c)), ledgerWrites' = ledgerWrites + 1, userRuns' = userRuns + 1,
    epoch' = epoch, pending' = pending, accepted' = accepted, members' = members, da' = da, used' = used, uses' = uses,
    busyEpochs' = busyEpochs,
  }

  // a later publication of a withheld claim list (the operator or a DA store)
  action publish(r: int): bool = all {
    accepted.contains(r), not(da.contains(r)), da' = da.union(Set(r)),
    epoch' = epoch, pending' = pending, accepted' = accepted, members' = members, runs' = runs, used' = used, uses' = uses,
    ledgerWrites' = ledgerWrites, busyEpochs' = busyEpochs, userRuns' = userRuns,
  }

  // the leaf a claim of `kind` and value v has in the tree (fix 3: the kind is in the leaf form)
  pure def leafMatches(mut: str, inTree: Leaf, kind: str, v: int): bool =
    if (mut == "no_kind_separation") inTree.v == v else (inTree.kind == kind and inTree.v == v)

  // a consumer uses a claim of `kind` from root r inside its own call (useRunClaim / useForeignClaim)
  action useFromRoot(k: int, kind: str, v: int, r: int): bool = all {
    MUT == "no_root_check" or accepted.contains(r),
    // the path: from the published list (da), or any path at all when the root is not checked
    if (MUT == "no_root_check") true
    else all { da.contains(r), members.get(r).exists(m => leafMatches(MUT, m, kind, v)) },
    val key = (k, kind, v, if (MUT == "root_keyed_nullifier") r else -1)
    all {
      not(used.contains(key)),
      used' = used.union(Set(key)),
    },
    uses' = uses.append((k, { kind: kind, v: v }, r)),
    ledgerWrites' = ledgerWrites + (if (MUT == "per_claim_open") 1 else 0),
    epoch' = epoch, pending' = pending, accepted' = accepted, members' = members, da' = da, runs' = runs,
    busyEpochs' = busyEpochs, userRuns' = userRuns,
  }

  // a consumer uses a run its own user settled by acceptRun (useSettledRun), same nullifier
  action useFromRuns(k: int, v: int): bool = all {
    runs.contains(v),
    val key = (k, "RUN", v, -1)
    all { not(used.contains(key)), used' = used.union(Set(key)) },
    uses' = uses.append((k, { kind: "RUN", v: v }, -1)),
    ledgerWrites' = ledgerWrites,
    epoch' = epoch, pending' = pending, accepted' = accepted, members' = members, da' = da, runs' = runs,
    busyEpochs' = busyEpochs, userRuns' = userRuns,
  }

  action step = any {
    nondet c = TRUE_RUNS.oneOf()
    proveMember({ kind: "RUN", v: c }),
    nondet f = TRUE_FGN.oneOf()
    proveMember({ kind: "FGN", v: f }),
    nondet p = Set(true, false).oneOf()
    closeEpoch(p),
    skipEpoch,
    nondet c = TRUE_RUNS.oneOf()
    userAcceptRun(c),
    nondet r = 0.to(MAX_EPOCH - 1).oneOf()
    publish(r),
    nondet k = CONSUMERS.oneOf()
    nondet kind = Set("RUN", "FGN").oneOf()
    nondet v = Set(1, 2, 3, 7, 9).oneOf()
    nondet r = 0.to(MAX_EPOCH - 1).oneOf()
    useFromRoot(k, kind, v, r),
    nondet k = CONSUMERS.oneOf()
    nondet c = TRUE_RUNS.oneOf()
    useFromRuns(k, c),
  }

  pure def countUses(us: List[(int, Leaf, int)], k: int, l: Leaf): int =
    us.foldl(0, (n, u) => if (u._1 == k and u._2 == l) n + 1 else n)

  // ---- invariants
  // the owner rule: ledger writes are epochs with members plus user-paid runs, never per claim
  val LedgerWritesBounded = ledgerWrites <= busyEpochs + userRuns
  // a consumer uses only verified claims of the kind it asked for (no forged run claim, no foreign as run)
  val UsedOnlyVerified = uses.indices().forall(i => val u = uses[i]
    if (u._2.kind == "RUN") TRUE_RUNS.contains(u._2.v) else TRUE_FGN.contains(u._2.v))
  // each consumer uses a claim at most once, whatever root or source it came through
  val UsedOncePerConsumer = uses.indices().forall(i => countUses(uses, uses[i]._1, uses[i]._2) <= 1)
  // epoch wait: a use from a root needs that root accepted (the claim's epoch closed)
  val UseAfterEpochClose = uses.indices().forall(i => uses[i]._3 == -1 or accepted.contains(uses[i]._3))

  val Inv = LedgerWritesBounded and UsedOnlyVerified and UsedOncePerConsumer and UseAfterEpochClose

  // ---- non-vacuity witnesses
  // one claim feeds two consumers
  val WitTwoConsumers = uses.indices().exists(i => uses.indices().exists(j => uses[i]._1 != uses[j]._1 and uses[i]._2 == uses[j]._2))
  // a claim is a member of two accepted roots
  val WitTwoRoots = accepted.exists(a => accepted.exists(b => a != b and members.get(a).intersect(members.get(b)).size() > 0))
  // an accepted root whose list is withheld while one of its members is unused (the DA dependency)
  val WitWithheld = accepted.exists(r => not(da.contains(r)))
  // a foreign claim is used
  val WitForeignUse = uses.indices().exists(i => uses[i]._2.kind == "FGN")
  val NotWitTwoConsumers = not(WitTwoConsumers)
  val NotWitTwoRoots = not(WitTwoRoots)
  val NotWitWithheld = not(WitWithheld)
  val NotWitForeignUse = not(WitForeignUse)
}
