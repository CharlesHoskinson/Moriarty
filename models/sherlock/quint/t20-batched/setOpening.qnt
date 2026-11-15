// T20 fix 5: opening a claim from a verified collection root, with the depth inside the proof
// (ARCHITECTURE 1.11 FP-3 fact 3, kit LEDGER CONTRACT, LDG-003 to LDG-005 after alignment).
// `descend` is removed: there is no K_open bound and no contract call per level.
//
// The SET grammar of the GP-2 hook (LEAF, DEP, JOIN, FOREIGN, ADM, VIEW) is built by actions of
// any shape (GP-4). An opening proof is abstracted as "x is reachable from root by JOIN and DEP
// cover links" (Lean GP2-L2 is the assumption). The same relation is the one a Sherlock consumer
// discharges in circuit (GP-2 DESCEND, DL-STEP), so this model also covers the fix 2 Sherlock path.
//
// MUT selects one weakened rule:
//   "none"              the design
//   "descend_k_open"    the stale spec: one descend call per K_OPEN levels, then openClaim
//   "head_any_form"     the proof does not refuse an ADM, VIEW or JOIN head
//   "no_replay_check"   openClaim inserts without the runs member check
//   "unverified_root"   openClaim does not require the root in verified_roots
module setOpening {
  const MUT: str

  type Form = { kind: str, head: int, l: int, r: int }

  pure val CLAIMS = Set(1, 2, 3)          // run claims (claim_pub stand-ins)
  pure val PREMISES = Set(101, 102)       // ADM and VIEW head claims (seal premises, never run claims)
  pure val FCLAIMS = Set(201)             // foreign claims (fclaim_d stand-ins)
  pure val MAX_NODES = 7
  pure val MAXD = 7
  pure val K_OPEN = 2
  pure val NOHEAD = -1

  var forms: int -> Form
  var nid: int
  var verified: Set[int]
  var settled: int -> int                 // ghost: inserts into runs or foreign per key
  var openCalls: int -> int               // ghost: contract calls spent per opened key
  var opened: Set[(int, int, int)]        // ghost: (root, key, depth) of each successful open

  // ---- the SET grammar: one-hash children of a node (JOIN l and r, DEP cover)
  pure def children(f: int -> Form, x: int): Set[int] =
    if (not(f.keys().contains(x))) Set()
    else {
      val n = f.get(x)
      if (n.kind == "JOIN") Set(n.l, n.r)
      else if (n.kind == "DEP") Set(n.l)
      else Set()
    }

  // nodes reachable from root with their depth (every depth reached, so a diamond shows twice)
  pure def reach(f: int -> Form, root: int): Set[(int, int)] =
    range(1, MAXD + 1).foldl(Set((root, 0)), (acc, d) =>
      acc.union(acc.filter(p => p._2 == d - 1).map(p => children(f, p._1).map(c => (c, d))).flatten()))

  pure def hasHead(n: Form): bool = n.kind != "JOIN"
  pure def isClaimHead(n: Form): bool = n.kind == "LEAF" or n.kind == "DEP" or n.kind == "FOREIGN"

  // members(root): every head claim reachable from root (GP-2 hook item 1)
  pure def members(f: int -> Form, root: int): Set[int] =
    reach(f, root).filter(p => f.keys().contains(p._1) and hasHead(f.get(p._1))).map(p => f.get(p._1).head)

  pure def getOr(m: int -> int, k: int, d: int): int = if (m.keys().contains(k)) m.get(k) else d
  pure def ceilDiv(a: int, b: int): int = (a + b - 1) / b

  action init = all {
    forms' = Map(), nid' = 0, verified' = Set(),
    settled' = Map(), openCalls' = Map(), opened' = Set(),
  }

  action build(n: Form): bool = all {
    nid < MAX_NODES,
    forms' = forms.put(nid, n), nid' = nid + 1,
    verified' = verified, settled' = settled, openCalls' = openCalls, opened' = opened,
  }
  action mkLeaf(c: int): bool = build({ kind: "LEAF", head: c, l: NOHEAD, r: NOHEAD })
  action mkForeign(fc: int): bool = build({ kind: "FOREIGN", head: fc, l: NOHEAD, r: NOHEAD })
  action mkPremise(k: str, p: int): bool = build({ kind: k, head: p, l: NOHEAD, r: NOHEAD })
  action mkJoin(a: int, b: int): bool = all { a < nid, b < nid, build({ kind: "JOIN", head: NOHEAD, l: a, r: b }) }
  action mkDep(c: int, cov: int): bool = all { cov < nid, build({ kind: "DEP", head: c, l: cov, r: NOHEAD }) }

  // acceptCollection(reg_pub, set_pub): one root, no claim write (LDG-003)
  action acceptCollection(root: int): bool = all {
    root < nid, not(verified.contains(root)),
    verified' = verified.union(Set(root)),
    forms' = forms, nid' = nid, settled' = settled, openCalls' = openCalls, opened' = opened,
  }

  // acceptRun: the user's own run, keyed by claim_pub in the same runs map (LDG-002)
  action acceptRun(c: int): bool = all {
    getOr(settled, c, 0) == 0,
    settled' = settled.put(c, 1),
    forms' = forms, nid' = nid, verified' = verified, openCalls' = openCalls, opened' = opened,
  }

  // openClaim(set_pub, claim): one call whatever the depth (LDG-004, LDG-005); the stale spec
  // spends one descend call per K_OPEN levels first
  action openClaim(root: int, x: int, d: int): bool = all {
    MUT == "unverified_root" or verified.contains(root),
    reach(forms, root).contains((x, d)),
    forms.keys().contains(x),
    val n = forms.get(x)
    all {
      if (MUT == "head_any_form") true else isClaimHead(n),
      MUT == "no_replay_check" or getOr(settled, n.head, 0) == 0,
      settled' = settled.put(n.head, getOr(settled, n.head, 0) + 1),
      openCalls' = openCalls.put(n.head, if (MUT == "descend_k_open") 1 + ceilDiv(d, K_OPEN) else 1),
      opened' = opened.union(Set((root, n.head, d))),
    },
    forms' = forms, nid' = nid, verified' = verified,
  }

  action step = any {
    nondet c = CLAIMS.oneOf()
    mkLeaf(c),
    nondet fc = FCLAIMS.oneOf()
    mkForeign(fc),
    nondet p = PREMISES.oneOf()
    nondet k = Set("ADM", "VIEW").oneOf()
    mkPremise(k, p),
    nondet a = 0.to(MAX_NODES - 1).oneOf()
    nondet b = 0.to(MAX_NODES - 1).oneOf()
    mkJoin(a, b),
    nondet c = CLAIMS.oneOf()
    nondet cov = 0.to(MAX_NODES - 1).oneOf()
    mkDep(c, cov),
    nondet r = 0.to(MAX_NODES - 1).oneOf()
    acceptCollection(r),
    nondet c = CLAIMS.oneOf()
    acceptRun(c),
    nondet r = 0.to(MAX_NODES - 1).oneOf()
    nondet x = 0.to(MAX_NODES - 1).oneOf()
    nondet d = 0.to(MAXD).oneOf()
    openClaim(r, x, d),
  }

  // ---- invariants
  // ClaimSettledOnceAcrossEntries and DiamondAncestorOpenedAtMostOnce: one insert per key
  val ClaimSettledOnce = settled.keys().forall(k => settled.get(k) <= 1)
  // only LEAF, DEP (into runs) and FOREIGN (into foreign) heads settle; never ADM, VIEW or JOIN
  val OnlyClaimHeadsSettle = settled.keys().forall(k => CLAIMS.contains(k) or FCLAIMS.contains(k))
  // openClaim settles only under a verified root
  val OpenedOnlyFromVerified = opened.forall(o => verified.contains(o._1))
  // LDG-004 after alignment: one contract call per opened claim at any depth (no descend)
  val OneCallAnyDepth = openCalls.keys().forall(k => openCalls.get(k) == 1)
  // every opened key is a member of its root (the proof relation)
  val OpenedAreMembers = opened.forall(o => members(forms, o._1).contains(o._2))

  val Inv = ClaimSettledOnce and OnlyClaimHeadsSettle and OpenedOnlyFromVerified and OneCallAnyDepth and OpenedAreMembers

  // ---- non-vacuity witnesses (each must be reachable: run with --invariant on its negation)
  val WitDeepOpen = opened.exists(o => o._3 >= 3)
  val WitForeignOpen = opened.exists(o => FCLAIMS.contains(o._2))
  val WitDiamond = verified.exists(r => reach(forms, r).exists(p => reach(forms, r).exists(q =>
    p._1 != q._1 and forms.keys().contains(p._1) and forms.keys().contains(q._1)
    and hasHead(forms.get(p._1)) and hasHead(forms.get(q._1)) and forms.get(p._1).head == forms.get(q._1).head)))
  val NotWitDeepOpen = not(WitDeepOpen)
  val NotWitForeignOpen = not(WitForeignOpen)
  val NotWitDiamond = not(WitDiamond)
}
