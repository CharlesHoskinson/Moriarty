// -*- mode: quint; -*-
// FP-8 KeyBind Soundness & Completeness Specification
//
// Source-file correspondence:
//   qa/gemini-quint/fp8-keybind/keybind.qnt      <-> lane-src/keybind.rs
//                                                <-> lane-src/proof_shape_sends_excerpt.rs
//                                                <-> lane-src/pvs_sends_excerpt.rs
//                                                <-> lane-src/driver_fp8_keyset_excerpt.rs
//   qa/gemini-quint/fp8-keybind/keybind_test.qnt <-> lane-src/driver_negatives.rs

module keybind {

  // =========================================================================
  // 1. Types & Constants
  // =========================================================================

  // Digest representation: 0 is unset; positive integers represent digests.
  type VkCommit = {
    cached: int,
    pre: int,
  }

  pure val UNSET_VK: VkCommit = { cached: 0, pre: 0 }

  // Public values of a node layer (Role::Leaf, N1, N2, N3):
  // flag, 5 VkCommit slots (app, leaf, n1, n2, n3), and depth.
  type NodePvs = {
    flag: int,
    app: VkCommit,
    leaf: VkCommit,
    n1: VkCommit,
    n2: VkCommit,
    n3: VkCommit,
    depth: int,
  }

  pure val ZERO_PVS: NodePvs = {
    flag: 0,
    app: UNSET_VK,
    leaf: UNSET_VK,
    n1: UNSET_VK,
    n2: UNSET_VK,
    n3: UNSET_VK,
    depth: 0,
  }

  // Public values of Root (layer L): exposes (app, leaf, n1) = 48 words.
  type RootPvs = {
    app: VkCommit,
    leaf: VkCommit,
    n1: VkCommit,
  }

  // Roles (a compile-time constant of each KeyBind AIR)
  type Role =
    | RoleLeaf
    | RoleN1
    | RoleN2
    | RoleN3
    | RoleRoot({ n2: VkCommit, n3: VkCommit })

  // Mutants for mutation testing of KeyBindAir + L_Fr
  type Mutant =
    | MutNone
    | MutNoUnsetCheck
    | MutNoCcReceive
    | MutRootNoN2Pin
    | MutRootNoPrehashPin
    | MutN3NoCopyN2
    | MutSelectorFree
    | MutLfrNoPin
    | MutN3NoCopyN3

  // Key universe: 5 honest node/app keys + 1 honest Root key + 3 adversarial keys
  // Each key has:
  //   - id: unique identifier
  //   - dagId: identifier of its constraint DAG (committed in parent's cached trace)
  //   - preId: identifier of its VK pre-hash (sent on PreHashBus)
  //
  // Honest keys:
  //   KEY_A  (1): honest app key A (dagId = 1, preId = 1)
  //   KEY_LF (2): honest Leaf verifier key (dagId = 2, preId = 2)
  //   KEY_K1 (3): honest N1 verifier key (dagId = 3, preId = 3)
  //   KEY_K2 (4): honest N2 verifier key (dagId = 4, preId = 4)
  //   KEY_K3 (5): honest N3 verifier key (dagId = 5, preId = 5)
  //   KEY_L  (6): honest Root (L) verifier key (dagId = 6, preId = 6)
  //
  // Adversarial / alternate keys (same shape, unconstrained PVs):
  //   KEY_B  (7): alternate app / node key (dagId = 7, preId = 7)
  //   KEY_X  (8): adversarial key sharing pre-hash with K3 but with adversarial DAG (dagId = 8, preId = 5)
  //   KEY_Y  (9): adversarial key sharing DAG with K3 but with adversarial pre-hash (dagId = 5, preId = 9)
  pure val KEY_A: int = 1
  pure val KEY_LF: int = 2
  pure val KEY_K1: int = 3
  pure val KEY_K2: int = 4
  pure val KEY_K3: int = 5
  pure val KEY_L: int = 6
  pure val KEY_B: int = 7
  pure val KEY_X: int = 8
  pure val KEY_Y: int = 9

  pure val HONEST_KEYS: Set[int] = Set(KEY_A, KEY_LF, KEY_K1, KEY_K2, KEY_K3, KEY_L)
  pure val APP_KEYS: Set[int] = Set(KEY_A, KEY_B)
  pure val ADV_NODE_KEYS: Set[int] = Set(KEY_B, KEY_X, KEY_Y)

  // Layer parameter IDs (identifying the parent verifier sub-circuit parameters)
  pure val LP_LEAF: int = 1
  pure val LP_INTERNAL: int = 2
  pure val LP_ROOT: int = 3

  // Extract the DAG ID of a key
  pure def keyDagId(k: int): int =
    if (k == KEY_Y) 5 else k

  // Extract the pre-hash ID of a key
  pure def keyPreId(k: int): int =
    if (k == KEY_X) 5 else k

  // Injective pre-hash table and lookup function from preId to digest integer
  pure val PRE_TABLE: int -> int = Map(
    1 -> 101, 2 -> 102, 3 -> 103, 4 -> 104, 5 -> 105,
    6 -> 106, 7 -> 107, 8 -> 108, 9 -> 109
  )

  pure def preHash(k: int): int =
    100 + keyPreId(k)

  // Injective commitment table and lookup function from (key's dagId, layer_params) to digest integer
  pure val COMMIT_TABLE: (int, int) -> int = Map(
    (1, LP_LEAF) -> 1001, (7, LP_LEAF) -> 1007,
    (2, LP_INTERNAL) -> 2002, (3, LP_INTERNAL) -> 2003,
    (4, LP_INTERNAL) -> 2004, (5, LP_INTERNAL) -> 2005,
    (7, LP_INTERNAL) -> 2007, (8, LP_INTERNAL) -> 2008,
    (4, LP_ROOT) -> 3004, (5, LP_ROOT) -> 3005,
    (7, LP_ROOT) -> 3007, (8, LP_ROOT) -> 3008
  )

  pure def dagCommit(lp: int, childKey: int): int =
    1000 * lp + keyDagId(childKey)

  // VkCommit of childKey in a parent layer with layer_params lp.
  // Because (keyDagId(k), keyPreId(k)) is unique for every k in 1..9,
  // vkCommitIn(lp, k) is strictly injective over the entire key universe.
  pure def vkCommitIn(lp: int, childKey: int): VkCommit = {
    cached: dagCommit(lp, childKey),
    pre: preHash(childKey),
  }

  // Canonical honest VkCommits
  pure val HONEST_APP_A: VkCommit = vkCommitIn(LP_LEAF, KEY_A)
  pure val HONEST_APP_B: VkCommit = vkCommitIn(LP_LEAF, KEY_B)
  pure val HONEST_LEAF: VkCommit  = vkCommitIn(LP_INTERNAL, KEY_LF)
  pure val HONEST_N1: VkCommit    = vkCommitIn(LP_INTERNAL, KEY_K1)
  pure val HONEST_N2: VkCommit    = vkCommitIn(LP_INTERNAL, KEY_K2)
  pure val HONEST_N3: VkCommit    = vkCommitIn(LP_INTERNAL, KEY_K3)
  pure val HONEST_L_K3_COMMIT: int = dagCommit(LP_ROOT, KEY_K3)

  // =========================================================================
  // 2. Bus Messages & Prover Witness Columns
  // =========================================================================

  // True values sent on PublicValuesBus, CachedCommitBus, and PreHashBus
  // by VerifierSubCircuit (ProofShape + PVs modules) for one child proof.
  type ChildBusSends = {
    has_pvs: bool,       // true iff child proof has public values at KB_AIR_ID = 0
    pvs: NodePvs,        // true public values of the child proof
    has_cc: bool,        // true iff child proof has a cached trace at CE_AIR_ID = 1
    cc: int,             // true cached commitment of the child's cached trace
    pre: int,            // true pre-hash of the child's verifying key
  }

  // Prover-chosen witness columns for a single valid row of KeyBindAir
  type RowWitness = {
    p: NodePvs,          // prover columns C_P..C_P+NUM_PVS
    s2: int,             // selector C_S2
    s31: int,            // selector C_S31
    s3x: int,            // selector C_S3X
    dinv: int,           // inverse column C_DINV (in BabyBear, inverse of p.depth - 1)
    free_pre: int,       // unconstrained column used only by MutRootNoPrehashPin
  }

  // =========================================================================
  // 3. KeyBindAir Constraints (eval)
  // =========================================================================

  pure def isBool(x: int): bool = (x == 0 or x == 1)

  // In BabyBear, `(depth - 1) * dinv == 1` has a solution `dinv` iff `depth != 1`,
  // and that solution is unique: `dinv = (depth - 1)^{-1} != 0`.
  // In our integer representation, we represent the field inverse of non-zero `x = depth - 1`
  // canonically by `dinv == x` (note that for depth == 2, x == 1 and dinv == 1).
  pure def isFieldInv(x: int, dinv: int): bool =
    (x != 0 and dinv == x)

  // Evaluate KeyBindAir constraints + permutation bus matching on one valid row
  pure def evalRow(
    role: Role,
    bus: ChildBusSends,
    w: RowWitness,
    ownPv: NodePvs,
    rootPv: RootPvs,
    mut: Mutant
  ): bool = {
    val hasChildPvs = match role {
      | RoleLeaf => false
      | _ => true
    }

    // 1. PublicValuesBus permutation check (KB_AIR_ID = 0):
    // RoleLeaf performs 0 receives; if an app sends PVs, the bus is unbalanced (fails closed).
    // Non-leaf roles receive all NUM_PVS words, binding w.p == bus.pvs.
    val pvBusOk =
      if (hasChildPvs)
        (bus.has_pvs and w.p == bus.pvs)
      else
        not(bus.has_pvs)

    // 2. CachedCommitBus permutation check (CE_AIR_ID = 1, cached_idx = 0, global_cached_idx = 0):
    // RoleLeaf performs 0 receives; if an app sends a cached commit, the bus is unbalanced.
    val expectedCc = match role {
      | RoleLeaf => 0
      | RoleN1 => w.p.app.cached
      | RoleN2 => w.p.leaf.cached
      | RoleN3 => w.s2 * w.p.n1.cached + w.s31 * w.p.n2.cached + w.s3x * w.p.n3.cached
      | RoleRoot(_) => w.s31 * w.p.n2.cached + w.s3x * w.p.n3.cached
    }
    val ccBusOk =
      if (not(hasChildPvs))
        not(bus.has_cc)
      else if (mut == MutNoCcReceive)
        true
      else
        (bus.has_cc and expectedCc == bus.cc)

    // 3. PreHashBus permutation check:
    val expectedPre = match role {
      | RoleLeaf => ownPv.app.pre
      | RoleN1 => ownPv.leaf.pre
      | RoleN2 => ownPv.n1.pre
      | RoleN3 => w.s2 * ownPv.n2.pre + (w.s31 + w.s3x) * ownPv.n3.pre
      | RoleRoot(pins) =>
          if (mut == MutRootNoPrehashPin) w.free_pre else pins.n3.pre
    }
    val preBusOk = (expectedPre == bus.pre)

    // 4. Role-specific polynomial constraints (lane-src/keybind.rs lines 271-368):
    val roleOk = match role {
      | RoleLeaf => and {
          ownPv.flag == 0,
          ownPv.depth == 0,
          ownPv.leaf == UNSET_VK,
          ownPv.n1 == UNSET_VK,
          ownPv.n2 == UNSET_VK,
          ownPv.n3 == UNSET_VK,
        }

      | RoleN1 => and {
          w.p.flag == 0,
          w.p.depth == 0,
          if (mut == MutNoUnsetCheck) true else and {
            w.p.leaf == UNSET_VK,
            w.p.n1 == UNSET_VK,
            w.p.n2 == UNSET_VK,
            w.p.n3 == UNSET_VK,
          },
          w.p.app == ownPv.app,
          ownPv.flag == 1,
          ownPv.depth == 0,
          ownPv.n1 == UNSET_VK,
          ownPv.n2 == UNSET_VK,
          ownPv.n3 == UNSET_VK,
        }

      | RoleN2 => and {
          w.p.flag == 1,
          w.p.depth == 0,
          if (mut == MutNoUnsetCheck) true else and {
            w.p.n1 == UNSET_VK,
            w.p.n2 == UNSET_VK,
            w.p.n3 == UNSET_VK,
          },
          w.p.app == ownPv.app,
          w.p.leaf == ownPv.leaf,
          ownPv.flag == 2,
          ownPv.depth == 0,
          ownPv.n2 == UNSET_VK,
          ownPv.n3 == UNSET_VK,
        }

      | RoleN3 => and {
          isBool(w.s2),
          isBool(w.s31),
          isBool(w.s3x),
          w.s2 + w.s31 + w.s3x == 1,
          // s2: child is N2 (flag 2, depth 0, n2 and n3 unset)
          (w.s2 == 1) implies and {
            w.p.flag == 2,
            w.p.depth == 0,
            if (mut == MutNoUnsetCheck) true else (w.p.n2 == UNSET_VK and w.p.n3 == UNSET_VK),
          },
          // s31: child is N3 at depth 1 (n3 unset)
          (w.s31 == 1) implies and {
            w.p.flag == 3,
            if (mut == MutSelectorFree) true else (w.p.depth == 1),
            if (mut == MutNoUnsetCheck) true else (w.p.n3 == UNSET_VK),
          },
          // s3x: child is N3 at depth != 1
          (w.s3x == 1) implies and {
            w.p.flag == 3,
            if (mut == MutSelectorFree) true else isFieldInv(w.p.depth - 1, w.dinv),
          },
          // Copies of slots below this layer
          w.p.app == ownPv.app,
          w.p.leaf == ownPv.leaf,
          w.p.n1 == ownPv.n1,
          if (mut == MutN3NoCopyN2) true else ((w.s31 + w.s3x == 1) implies (w.p.n2 == ownPv.n2)),
          if (mut == MutN3NoCopyN3) true else ((w.s3x == 1) implies (w.p.n3 == ownPv.n3)),
          // Own: flag 3; depth 1 over N2, child depth + 1 over N3; n3 unset at depth 1
          ownPv.flag == 3,
          (w.s2 == 1) implies (ownPv.depth == 1),
          (w.s31 + w.s3x == 1) implies (ownPv.depth == w.p.depth + 1),
          (w.s2 == 1) implies (ownPv.n3 == UNSET_VK),
        }

      | RoleRoot(pins) => and {
          isBool(w.s2),
          isBool(w.s31),
          isBool(w.s3x),
          w.s2 + w.s31 + w.s3x == 1,
          w.s2 == 0,
          // s31: child is N3 at depth 1 (n3 unset)
          (w.s31 == 1) implies and {
            w.p.flag == 3,
            if (mut == MutSelectorFree) true else (w.p.depth == 1),
            if (mut == MutNoUnsetCheck) true else (w.p.n3 == UNSET_VK),
          },
          // s3x: child is N3 at depth != 1
          (w.s3x == 1) implies and {
            w.p.flag == 3,
            if (mut == MutSelectorFree) true else isFieldInv(w.p.depth - 1, w.dinv),
          },
          // Universal keys: N2 always, N3 once set (s3x)
          if (mut == MutRootNoN2Pin) true else (w.p.n2 == pins.n2),
          (w.s3x == 1) implies (w.p.n3 == pins.n3),
          // Expose app, leaf, n1
          w.p.app == rootPv.app,
          w.p.leaf == rootPv.leaf,
          w.p.n1 == rootPv.n1,
        }
    }

    and { pvBusOk, ccBusOk, preBusOk, roleOk }
  }

  // Honest witness constructor (mirrors keybind::trace in lane-src/keybind.rs lines 378-449)
  pure def honestRowWitness(role: Role, bus: ChildBusSends): RowWitness = {
    val cp = bus.pvs
    val isLeaf = match role { | RoleLeaf => true | _ => false }
    if (isLeaf) {
      { p: ZERO_PVS, s2: 0, s31: 0, s3x: 0, dinv: 0, free_pre: bus.pre }
    } else if (cp.flag == 2) {
      { p: cp, s2: 1, s31: 0, s3x: 0, dinv: 0, free_pre: bus.pre }
    } else if (cp.flag == 3 and cp.depth == 1) {
      { p: cp, s2: 0, s31: 1, s3x: 0, dinv: 0, free_pre: bus.pre }
    } else if (cp.flag == 3 and cp.depth >= 2) {
      { p: cp, s2: 0, s31: 0, s3x: 1, dinv: cp.depth - 1, free_pre: bus.pre }
    } else {
      { p: cp, s2: 0, s31: 0, s3x: 0, dinv: 0, free_pre: bus.pre }
    }
  }

  // Honest public values constructor (mirrors keybind::trace in lane-src/keybind.rs)
  pure def honestNodePvs(role: Role, p0: NodePvs, childCommit: VkCommit): NodePvs =
    match role {
      | RoleLeaf => { ...ZERO_PVS, flag: 0, app: childCommit, depth: 0 }
      | RoleN1   => { ...p0, flag: 1, leaf: childCommit }
      | RoleN2   => { ...p0, flag: 2, n1: childCommit }
      | RoleN3   =>
          if (p0.flag == 2)
            { ...p0, flag: 3, n2: childCommit, depth: 1 }
          else if (p0.flag == 3 and p0.depth == 1)
            { ...p0, flag: 3, n3: childCommit, depth: p0.depth + 1 }
          else
            { ...p0, flag: 3, depth: p0.depth + 1 }
      | RoleRoot(_) => ZERO_PVS
    }

  pure def honestRootPvs(p0: NodePvs): RootPvs = {
    app: p0.app,
    leaf: p0.leaf,
    n1: p0.n1,
  }

  // =========================================================================
  // 4. Proof Tree Representation (Arity 1 or 2, Depth up to 7)
  // =========================================================================

  type ProofNode = {
    key: int,            // Verifying key of this node
    arity: int,          // 1 or 2 child proofs
    pvs: NodePvs,        // Public values of this node
    w0: RowWitness,      // KeyBind row 0 witness
    w1: RowWitness,      // KeyBind row 1 witness (used when arity == 2)
  }

  pure def roleOfKey(k: int): Role =
    if (k == KEY_LF) RoleLeaf
    else if (k == KEY_K1) RoleN1
    else if (k == KEY_K2) RoleN2
    else if (k == KEY_K3) RoleN3
    else RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })

  pure def lpOfKey(k: int): int =
    if (k == KEY_LF) LP_LEAF
    else if (k == KEY_L) LP_ROOT
    else LP_INTERNAL

  // Bus messages sent when a Leaf layer verifies an app proof
  pure def appBusSends(appKey: int, hasPvs: bool, hasCc: bool): ChildBusSends = {
    has_pvs: hasPvs,
    pvs: ZERO_PVS,
    has_cc: hasCc,
    cc: 0,
    pre: preHash(appKey),
  }

  // Bus messages sent when a parent layer verifies `childNode`, which itself verified
  // a grandchild with key `grandchildKey` in `childNode`'s cached trace.
  pure def nodeBusSends(childNode: ProofNode, grandchildKey: int): ChildBusSends = {
    has_pvs: true,
    pvs: childNode.pvs,
    has_cc: true,
    cc: dagCommit(lpOfKey(childNode.key), grandchildKey),
    pre: preHash(childNode.key),
  }

  // Check whether `node` is accepted given bus sends `s0, s1` from its children.
  // Adversarial keys (KEY_B, KEY_X, KEY_Y) have free AIR constraints and accept arbitrary PVs.
  pure def nodeAccepted(
    node: ProofNode,
    s0: ChildBusSends,
    s1: ChildBusSends,
    rootPv: RootPvs,
    mut: Mutant
  ): bool = {
    if (ADV_NODE_KEYS.contains(node.key)) {
      true
    } else {
      val role = roleOfKey(node.key)
      val ok0 = evalRow(role, s0, node.w0, node.pvs, rootPv, mut)
      val ok1 = if (node.arity == 2) evalRow(role, s1, node.w1, node.pvs, rootPv, mut) else true
      and { node.arity >= 1, node.arity <= 2, ok0, ok1 }
    }
  }

  // Complete proof tree (depth up to 7: app -> leaf -> N1 -> N2 -> N3d1 -> N3d2 -> N3d3 -> L)
  // Supports binary branching at N1 (two leaf proofs over app0Key, app1Key) and at N3d2.
  type TreeChain = {
    app0Key: int,
    app1Key: int,
    leaf0: ProofNode,
    leaf1: ProofNode,
    n1Node: ProofNode,
    n2Node: ProofNode,
    n3Depth: int,          // 1, 2, or 3 N3 layers
    n3d1: ProofNode,
    n3d2: ProofNode,
    n3d2Row1FromD1: bool,  // true: row 1 of n3d2 is n3d1; false: mixed with n3d2
    n3d3: ProofNode,
    rootNode: ProofNode,
    rootPv: RootPvs,
  }

  // Construct an honest proof chain for `appKey` with `n3Levels` (1, 2, or 3) N3 layers
  pure def buildHonestTree(appKey: int, n3Levels: int): TreeChain = {
    val appCommit = vkCommitIn(LP_LEAF, appKey)
    val sApp = appBusSends(appKey, false, false)

    val lfPvs = honestNodePvs(RoleLeaf, ZERO_PVS, appCommit)
    val lfW = honestRowWitness(RoleLeaf, sApp)
    val lfNode: ProofNode = {
      key: KEY_LF, arity: 1, pvs: lfPvs, w0: lfW, w1: lfW
    }

    val sLf = nodeBusSends(lfNode, appKey)
    val n1Pvs = honestNodePvs(RoleN1, lfPvs, HONEST_LEAF)
    val n1W = honestRowWitness(RoleN1, sLf)
    val n1Node: ProofNode = {
      key: KEY_K1, arity: 2, pvs: n1Pvs, w0: n1W, w1: n1W
    }

    val sN1 = nodeBusSends(n1Node, KEY_LF)
    val n2Pvs = honestNodePvs(RoleN2, n1Pvs, HONEST_N1)
    val n2W = honestRowWitness(RoleN2, sN1)
    val n2Node: ProofNode = {
      key: KEY_K2, arity: 1, pvs: n2Pvs, w0: n2W, w1: n2W
    }

    val sN2 = nodeBusSends(n2Node, KEY_K1)
    val n3d1Pvs = honestNodePvs(RoleN3, n2Pvs, HONEST_N2)
    val n3d1W = honestRowWitness(RoleN3, sN2)
    val n3d1: ProofNode = {
      key: KEY_K3, arity: 1, pvs: n3d1Pvs, w0: n3d1W, w1: n3d1W
    }

    val sN3d1 = nodeBusSends(n3d1, KEY_K2)
    val n3d2Pvs = honestNodePvs(RoleN3, n3d1Pvs, HONEST_N3)
    val n3d2W = honestRowWitness(RoleN3, sN3d1)
    val n3d2: ProofNode = {
      key: KEY_K3, arity: 2, pvs: n3d2Pvs, w0: n3d2W, w1: n3d2W
    }

    val sN3d2 = nodeBusSends(n3d2, KEY_K3)
    val n3d3Pvs = honestNodePvs(RoleN3, n3d2Pvs, HONEST_N3)
    val n3d3W = honestRowWitness(RoleN3, sN3d2)
    val n3d3: ProofNode = {
      key: KEY_K3, arity: 1, pvs: n3d3Pvs, w0: n3d3W, w1: n3d3W
    }

    val topN3 = if (n3Levels == 1) n3d1 else if (n3Levels == 2) n3d2 else n3d3
    val topChildKey = if (n3Levels == 1) KEY_K2 else KEY_K3
    val sTop = nodeBusSends(topN3, topChildKey)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    val rPv = honestRootPvs(topN3.pvs)
    val rootNode: ProofNode = {
      key: KEY_L, arity: 1, pvs: ZERO_PVS, w0: rootW, w1: rootW
    }

    {
      app0Key: appKey,
      app1Key: appKey,
      leaf0: lfNode,
      leaf1: lfNode,
      n1Node: n1Node,
      n2Node: n2Node,
      n3Depth: n3Levels,
      n3d1: n3d1,
      n3d2: n3d2,
      n3d2Row1FromD1: true,
      n3d3: n3d3,
      rootNode: rootNode,
      rootPv: rPv,
    }
  }

  // Evaluate whether an entire proof tree `t` is accepted under mutant `mut`
  pure def treeAccepted(t: TreeChain, mut: Mutant): bool = {
    val sApp0 = appBusSends(t.app0Key, false, false)
    val sApp1 = appBusSends(t.app1Key, false, false)

    val lf0Ok = nodeAccepted(t.leaf0, sApp0, sApp0, t.rootPv, mut)
    val lf1Ok = nodeAccepted(t.leaf1, sApp1, sApp1, t.rootPv, mut)

    // In a multi-child verifier layer, the single cached trace commits one child DAG;
    // if arity == 2, both children are verified against the same cached trace (dagId).
    val sLf0 = nodeBusSends(t.leaf0, t.app0Key)
    val sLf1 = nodeBusSends(t.leaf1, t.app1Key)
    val n1SameChildDag =
      if (t.n1Node.arity == 2) keyDagId(t.leaf0.key) == keyDagId(t.leaf1.key) else true
    val n1Ok = n1SameChildDag and nodeAccepted(t.n1Node, sLf0, sLf1, t.rootPv, mut)

    val sN1 = nodeBusSends(t.n1Node, t.leaf0.key)
    val n2Ok = nodeAccepted(t.n2Node, sN1, sN1, t.rootPv, mut)

    val sN2 = nodeBusSends(t.n2Node, t.n1Node.key)
    val n3d1Ok = nodeAccepted(t.n3d1, sN2, sN2, t.rootPv, mut)

    val sN3d1 = nodeBusSends(t.n3d1, t.n2Node.key)
    val sRow1For2 = if (t.n3d2Row1FromD1) sN3d1 else nodeBusSends(t.n3d2, t.n3d1.key)
    val n3d2SameChildDag =
      if (t.n3d2.arity == 2 and not(t.n3d2Row1FromD1))
        keyDagId(t.n3d1.key) == keyDagId(t.n3d2.key)
      else true
    val n3d2Ok =
      if (t.n3Depth >= 2)
        n3d2SameChildDag and nodeAccepted(t.n3d2, sN3d1, sRow1For2, t.rootPv, mut)
      else true

    val sN3d2 = nodeBusSends(t.n3d2, t.n3d1.key)
    val n3d3Ok =
      if (t.n3Depth >= 3)
        nodeAccepted(t.n3d3, sN3d2, sN3d2, t.rootPv, mut)
      else true

    val topN3 = if (t.n3Depth == 1) t.n3d1 else if (t.n3Depth == 2) t.n3d2 else t.n3d3
    val topChildKey =
      if (t.n3Depth == 1) t.n2Node.key
      else if (t.n3Depth == 2) t.n3d1.key
      else t.n3d2.key
    val sTop = nodeBusSends(topN3, topChildKey)

    // L_Fr pins the cached commitment of L (which commits topN3's DAG in LP_ROOT) to K3's DAG
    val lfrPinOk =
      if (mut == MutLfrNoPin) true
      else (dagCommit(LP_ROOT, topN3.key) == HONEST_L_K3_COMMIT)

    val rootOk =
      (t.rootNode.key == KEY_L) and
      lfrPinOk and
      nodeAccepted(t.rootNode, sTop, sTop, t.rootPv, mut)

    and { lf0Ok, lf1Ok, n1Ok, n2Ok, n3d1Ok, n3d2Ok, n3d3Ok, rootOk }
  }

  // =========================================================================
  // 5. Named Mutant Predicates & Properties (BIND, COMPLETE, Hygiene)
  // =========================================================================

  pure def mut_no_unset_check(t: TreeChain): bool = treeAccepted(t, MutNoUnsetCheck)
  pure def mut_no_cc_receive(t: TreeChain): bool = treeAccepted(t, MutNoCcReceive)
  pure def mut_root_no_n2_pin(t: TreeChain): bool = treeAccepted(t, MutRootNoN2Pin)
  pure def mut_root_no_prehash_pin(t: TreeChain): bool = treeAccepted(t, MutRootNoPrehashPin)
  pure def mut_n3_no_copy_n2(t: TreeChain): bool = treeAccepted(t, MutN3NoCopyN2)
  pure def mut_selector_free(t: TreeChain): bool = treeAccepted(t, MutSelectorFree)
  pure def mut_lfr_no_pin(t: TreeChain): bool = treeAccepted(t, MutLfrNoPin)
  pure def mut_n3_no_copy_n3(t: TreeChain): bool = treeAccepted(t, MutN3NoCopyN3)

  // The BIND property:
  // Whenever L (Root, with pins K2, K3 and L_Fr pin on K3) accepts:
  // 1. Every N3 position and N2 position along every path uses the honest key (KEY_K3, KEY_K2).
  // 2. All leaves prove the same app key, and if N1/Leaf are honest keys, the 48 exposed words
  //    (app, leaf, n1) in rootPv are the exact (dagCommit, preHash) of the keys used on every path.
  pure def bindHoldsOnTree(t: TreeChain): bool = and {
    t.n3d1.key == KEY_K3,
    (t.n3Depth >= 2) implies (t.n3d2.key == KEY_K3 and t.n3d2Row1FromD1),
    (t.n3Depth >= 3) implies (t.n3d3.key == KEY_K3),
    t.n2Node.key == KEY_K2,
    t.rootPv.n1 == vkCommitIn(lpOfKey(t.n2Node.key), t.n1Node.key),
    (t.n1Node.key == KEY_K1) implies and {
      t.rootPv.leaf == vkCommitIn(lpOfKey(t.n1Node.key), t.leaf0.key),
      (t.n1Node.arity == 2) implies (t.leaf1.key == t.leaf0.key),
      (t.leaf0.key == KEY_LF) implies and {
        t.rootPv.app == vkCommitIn(lpOfKey(t.leaf0.key), t.app0Key),
        (t.n1Node.arity == 2 and t.leaf1.key == KEY_LF) implies (t.app1Key == t.app0Key),
      },
    },
  }

  // Slot hygiene property: no child proof in an accepted tree has non-zero slots above its layer
  pure def slotHygieneOnTree(t: TreeChain): bool = and {
    t.leaf0.pvs.leaf == UNSET_VK and t.leaf0.pvs.n1 == UNSET_VK and t.leaf0.pvs.n2 == UNSET_VK and t.leaf0.pvs.n3 == UNSET_VK,
    t.n1Node.pvs.n1 == UNSET_VK and t.n1Node.pvs.n2 == UNSET_VK and t.n1Node.pvs.n3 == UNSET_VK,
    t.n2Node.pvs.n2 == UNSET_VK and t.n2Node.pvs.n3 == UNSET_VK,
    t.n3d1.pvs.n3 == UNSET_VK,
  }

  // Selector consistency property: selectors and dinv match child depth at N3 and Root
  pure def rowSelectorConsistent(w: RowWitness): bool = and {
    (w.s2 == 1) implies (w.p.flag == 2 and w.p.depth == 0),
    (w.s31 == 1) implies (w.p.flag == 3 and w.p.depth == 1),
    (w.s3x == 1) implies (w.p.flag == 3 and isFieldInv(w.p.depth - 1, w.dinv)),
  }

  pure def selectorConsistencyOnTree(t: TreeChain): bool = and {
    rowSelectorConsistent(t.n3d1.w0),
    (t.n3Depth >= 2) implies (rowSelectorConsistent(t.n3d2.w0) and rowSelectorConsistent(t.n3d2.w1)),
    (t.n3Depth >= 3) implies rowSelectorConsistent(t.n3d3.w0),
    rowSelectorConsistent(t.rootNode.w0),
  }

  // =========================================================================
  // 6. Concrete Counterexamples Catching Every Mutant
  // =========================================================================

  // Counterexample 1: catches mut_no_unset_check
  // Per-program Leaf key KEY_B pre-sets upper slot n2 in its PVs; N1 accepts under mut_no_unset_check
  // and refuses under MutNone.
  pure def cexMutNoUnsetCheck: TreeChain = {
    val base = buildHonestTree(KEY_A, 1)
    val dirtyLeafPvs = {
      ...base.leaf0.pvs,
      n2: vkCommitIn(LP_INTERNAL, KEY_B),
    }
    val dirtyLeaf = { ...base.leaf0, key: KEY_B, pvs: dirtyLeafPvs }
    val sDirtyLf = nodeBusSends(dirtyLeaf, KEY_A)
    val n1Pvs = honestNodePvs(RoleN1, dirtyLeafPvs, vkCommitIn(LP_INTERNAL, KEY_B))
    val n1CleanPvs = { ...n1Pvs, n2: UNSET_VK }
    val n1W = honestRowWitness(RoleN1, sDirtyLf)
    val n1Node = { ...base.n1Node, pvs: n1CleanPvs, w0: n1W, w1: n1W }
    val sN1 = nodeBusSends(n1Node, KEY_B)
    val n2Pvs = honestNodePvs(RoleN2, n1CleanPvs, HONEST_N1)
    val n2W = honestRowWitness(RoleN2, sN1)
    val n2Node = { ...base.n2Node, pvs: n2Pvs, w0: n2W, w1: n2W }
    val sN2 = nodeBusSends(n2Node, KEY_K1)
    val n3d1Pvs = honestNodePvs(RoleN3, n2Pvs, HONEST_N2)
    val n3d1W = honestRowWitness(RoleN3, sN2)
    val n3d1 = { ...base.n3d1, pvs: n3d1Pvs, w0: n3d1W, w1: n3d1W }
    val sTop = nodeBusSends(n3d1, KEY_K2)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    val rootNode = { ...base.rootNode, w0: rootW, w1: rootW }
    {
      ...base,
      leaf0: dirtyLeaf,
      leaf1: dirtyLeaf,
      n1Node: n1Node,
      n2Node: n2Node,
      n3d1: n3d1,
      rootNode: rootNode,
      rootPv: honestRootPvs(n3d1Pvs),
    }
  }

  // Counterexample 2: catches mut_no_cc_receive (violates BIND)
  // Leaf proves app KEY_A, but claims cached half of HONEST_APP_B in its PVs;
  // without CachedCommitBus receive in N1, the tampered app cached commit reaches Root!
  pure def cexMutNoCcReceive: TreeChain = {
    val base = buildHonestTree(KEY_A, 1)
    val forgedApp = { cached: HONEST_APP_B.cached, pre: HONEST_APP_A.pre }
    val lfPvs = { ...base.leaf0.pvs, app: forgedApp }
    val lfNode = { ...base.leaf0, pvs: lfPvs }
    val sLf = nodeBusSends(lfNode, KEY_A)
    val n1Pvs = honestNodePvs(RoleN1, lfPvs, HONEST_LEAF)
    val n1W = honestRowWitness(RoleN1, sLf)
    val n1Node = { ...base.n1Node, pvs: n1Pvs, w0: n1W, w1: n1W }
    val sN1 = nodeBusSends(n1Node, KEY_LF)
    val n2Pvs = honestNodePvs(RoleN2, n1Pvs, HONEST_N1)
    val n2W = honestRowWitness(RoleN2, sN1)
    val n2Node = { ...base.n2Node, pvs: n2Pvs, w0: n2W, w1: n2W }
    val sN2 = nodeBusSends(n2Node, KEY_K1)
    val n3d1Pvs = honestNodePvs(RoleN3, n2Pvs, HONEST_N2)
    val n3d1W = honestRowWitness(RoleN3, sN2)
    val n3d1 = { ...base.n3d1, pvs: n3d1Pvs, w0: n3d1W, w1: n3d1W }
    val sTop = nodeBusSends(n3d1, KEY_K2)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    {
      ...base,
      leaf0: lfNode,
      leaf1: lfNode,
      n1Node: n1Node,
      n2Node: n2Node,
      n3d1: n3d1,
      rootNode: { ...base.rootNode, w0: rootW, w1: rootW },
      rootPv: honestRootPvs(n3d1Pvs),
    }
  }

  // Counterexample 3: catches mut_root_no_n2_pin (violates BIND)
  // Adversarial key KEY_B is used at N2 position (forging app = HONEST_APP_B while proving KEY_A);
  // N3(d1) records KEY_B in slot n2, and Root accepts when N2 pin is missing.
  pure def cexMutRootNoN2Pin: TreeChain = {
    val base = buildHonestTree(KEY_A, 1)
    val advN2Pvs = { ...base.n2Node.pvs, app: HONEST_APP_B }
    val advN2 = { ...base.n2Node, key: KEY_B, pvs: advN2Pvs }
    val sN2 = nodeBusSends(advN2, KEY_K1)
    val n3d1Pvs = honestNodePvs(RoleN3, advN2Pvs, vkCommitIn(LP_INTERNAL, KEY_B))
    val n3d1W = honestRowWitness(RoleN3, sN2)
    val n3d1 = { ...base.n3d1, pvs: n3d1Pvs, w0: n3d1W, w1: n3d1W }
    val sTop = nodeBusSends(n3d1, KEY_B)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    {
      ...base,
      n2Node: advN2,
      n3d1: n3d1,
      rootNode: { ...base.rootNode, w0: rootW, w1: rootW },
      rootPv: honestRootPvs(n3d1Pvs),
    }
  }

  // Counterexample 4: catches mut_root_no_prehash_pin (violates BIND)
  // Top N3 node uses adversarial key KEY_Y (same DAG as K3 so L_Fr pin passes, but adversarial
  // pre-hash); Root puts KEY_Y's pre-hash into the free column and accepts forged app = HONEST_APP_B.
  pure def cexMutRootNoPrehashPin: TreeChain = {
    val base = buildHonestTree(KEY_A, 1)
    val forgedN3Pvs = { ...base.n3d1.pvs, app: HONEST_APP_B }
    val advN3 = { ...base.n3d1, key: KEY_Y, pvs: forgedN3Pvs }
    val sTop = nodeBusSends(advN3, KEY_K2)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    {
      ...base,
      n3d1: advN3,
      rootNode: { ...base.rootNode, w0: rootW, w1: rootW },
      rootPv: honestRootPvs(forgedN3Pvs),
    }
  }

  // Counterexample 5: catches mut_n3_no_copy_n2 (violates BIND)
  // At depth 2, N2 uses adversarial key KEY_B (forging app = HONEST_APP_B);
  // N3(d1) records KEY_B in n2, and N3(d2) overwrites its own n2 slot with HONEST_N2!
  pure def cexMutN3NoCopyN2: TreeChain = {
    val base = buildHonestTree(KEY_A, 2)
    val advN2Pvs = { ...base.n2Node.pvs, app: HONEST_APP_B }
    val advN2 = { ...base.n2Node, key: KEY_B, pvs: advN2Pvs }
    val sN2 = nodeBusSends(advN2, KEY_K1)
    val n3d1Pvs = honestNodePvs(RoleN3, advN2Pvs, vkCommitIn(LP_INTERNAL, KEY_B))
    val n3d1W = honestRowWitness(RoleN3, sN2)
    val n3d1 = { ...base.n3d1, pvs: n3d1Pvs, w0: n3d1W, w1: n3d1W }
    val sN3d1 = nodeBusSends(n3d1, KEY_B)
    val n3d2RawPvs = honestNodePvs(RoleN3, n3d1Pvs, HONEST_N3)
    // Prover overwrites n2 in n3d2.pvs with HONEST_N2 so Root's pin check passes!
    val n3d2ForgedPvs = { ...n3d2RawPvs, n2: HONEST_N2 }
    val n3d2W = honestRowWitness(RoleN3, sN3d1)
    val n3d2 = { ...base.n3d2, pvs: n3d2ForgedPvs, w0: n3d2W, w1: n3d2W }
    val sTop = nodeBusSends(n3d2, KEY_K3)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    {
      ...base,
      n2Node: advN2,
      n3d1: n3d1,
      n3d2: n3d2,
      rootNode: { ...base.rootNode, w0: rootW, w1: rootW },
      rootPv: honestRootPvs(n3d2ForgedPvs),
    }
  }

  // Counterexample 6: catches mut_selector_free
  // Prover chooses dinv = 0 with s3x = 1 at Root on a depth-2 tree (or s31/s3x depth mismatch).
  pure def cexMutSelectorFree: TreeChain = {
    val base = buildHonestTree(KEY_A, 2)
    val badRootW = { ...base.rootNode.w0, dinv: 0 }
    {
      ...base,
      rootNode: { ...base.rootNode, w0: badRootW, w1: badRootW },
    }
  }

  // Counterexample 7: catches mut_lfr_no_pin (violates BIND)
  // Top N3 node uses adversarial key KEY_X (same pre-hash as K3 so Root's PreHashBus pin passes,
  // but adversarial DAG so L_Fr pin is needed to reject it); forges app = HONEST_APP_B.
  pure def cexMutLfrNoPin: TreeChain = {
    val base = buildHonestTree(KEY_A, 1)
    val forgedN3Pvs = { ...base.n3d1.pvs, app: HONEST_APP_B }
    val advN3 = { ...base.n3d1, key: KEY_X, pvs: forgedN3Pvs }
    val sTop = nodeBusSends(advN3, KEY_K2)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    {
      ...base,
      n3d1: advN3,
      rootNode: { ...base.rootNode, w0: rootW, w1: rootW },
      rootPv: honestRootPvs(forgedN3Pvs),
    }
  }

  // Counterexample 8: catches mut_n3_no_copy_n3 (violates BIND)
  // At depth 3, N3(d1) uses adversarial key KEY_B (forging app = HONEST_APP_B);
  // N3(d2) records KEY_B in slot n3, and N3(d3) (with s3x = 1) overwrites its own n3 slot with HONEST_N3!
  pure def cexMutN3NoCopyN3: TreeChain = {
    val base = buildHonestTree(KEY_A, 3)
    val advN3d1Pvs = { ...base.n3d1.pvs, app: HONEST_APP_B }
    val advN3d1 = { ...base.n3d1, key: KEY_B, pvs: advN3d1Pvs }
    val sN3d1 = nodeBusSends(advN3d1, KEY_K2)
    val n3d2Pvs = honestNodePvs(RoleN3, advN3d1Pvs, vkCommitIn(LP_INTERNAL, KEY_B))
    val n3d2W = honestRowWitness(RoleN3, sN3d1)
    val n3d2 = { ...base.n3d2, pvs: n3d2Pvs, w0: n3d2W, w1: n3d2W }
    val sN3d2 = nodeBusSends(n3d2, KEY_B)
    val n3d3RawPvs = honestNodePvs(RoleN3, n3d2Pvs, HONEST_N3)
    val n3d3ForgedPvs = { ...n3d3RawPvs, n3: HONEST_N3 }
    val n3d3W = honestRowWitness(RoleN3, sN3d2)
    val n3d3 = { ...base.n3d3, pvs: n3d3ForgedPvs, w0: n3d3W, w1: n3d3W }
    val sTop = nodeBusSends(n3d3, KEY_K3)
    val rootRole = RoleRoot({ n2: HONEST_N2, n3: HONEST_N3 })
    val rootW = honestRowWitness(rootRole, sTop)
    {
      ...base,
      n3d1: advN3d1,
      n3d2: n3d2,
      n3d3: n3d3,
      rootNode: { ...base.rootNode, w0: rootW, w1: rootW },
      rootPv: honestRootPvs(n3d3ForgedPvs),
    }
  }

  // =========================================================================
  // 7. State Machine Variables, Invariants, Witnesses & Actions
  // =========================================================================

  var currentTree: TreeChain
  var isHonestStep: bool
  var stepTag: str

  // Primary Safety Invariant: BIND
  val BIND: bool =
    treeAccepted(currentTree, MutNone) implies bindHoldsOnTree(currentTree)

  // Primary Completeness Invariant: COMPLETE
  val COMPLETE: bool =
    isHonestStep implies (treeAccepted(currentTree, MutNone) and bindHoldsOnTree(currentTree))

  // Combined Soundness Invariant (BIND + Slot Hygiene + Selector Consistency)
  val SOUND_ALL: bool =
    treeAccepted(currentTree, MutNone) implies and {
      bindHoldsOnTree(currentTree),
      slotHygieneOnTree(currentTree),
      selectorConsistencyOnTree(currentTree),
    }

  // Mutant Invariants (each FAILS on its respective mutant and PASSES on MutNone):
  val inv_mut_no_unset_check: bool =
    mut_no_unset_check(currentTree) implies slotHygieneOnTree(currentTree)
  val inv_mut_no_cc_receive: bool =
    mut_no_cc_receive(currentTree) implies bindHoldsOnTree(currentTree)
  val inv_mut_root_no_n2_pin: bool =
    mut_root_no_n2_pin(currentTree) implies bindHoldsOnTree(currentTree)
  val inv_mut_root_no_prehash_pin: bool =
    mut_root_no_prehash_pin(currentTree) implies bindHoldsOnTree(currentTree)
  val inv_mut_n3_no_copy_n2: bool =
    mut_n3_no_copy_n2(currentTree) implies bindHoldsOnTree(currentTree)
  val inv_mut_selector_free: bool =
    mut_selector_free(currentTree) implies selectorConsistencyOnTree(currentTree)
  val inv_mut_lfr_no_pin: bool =
    mut_lfr_no_pin(currentTree) implies bindHoldsOnTree(currentTree)
  val inv_mut_n3_no_copy_n3: bool =
    mut_n3_no_copy_n3(currentTree) implies bindHoldsOnTree(currentTree)

  // Witnesses for simulation coverage
  val witnessedHonestAccept: bool =
    (stepTag != "init") and isHonestStep and treeAccepted(currentTree, MutNone)

  val witnessedAdversarialRefused: bool =
    not(isHonestStep) and not(treeAccepted(currentTree, MutNone))

  val witnessedDepth3Honest: bool =
    isHonestStep and (currentTree.n3Depth == 3) and treeAccepted(currentTree, MutNone)

  // Actions
  action init: bool = all {
    currentTree' = buildHonestTree(KEY_A, 1),
    isHonestStep' = true,
    stepTag' = "init",
  }

  action stepHonest(appKey: int, n3Levels: int): bool = all {
    APP_KEYS.contains(appKey),
    n3Levels >= 1 and n3Levels <= 3,
    currentTree' = buildHonestTree(appKey, n3Levels),
    isHonestStep' = true,
    stepTag' = "honest",
  }

  action stepMixedAppKeys(n3Levels: int): bool = {
    val base = buildHonestTree(KEY_A, n3Levels)
    val bTree = buildHonestTree(KEY_B, n3Levels)
    all {
      n3Levels >= 1 and n3Levels <= 3,
      currentTree' = { ...base, app1Key: KEY_B, leaf1: bTree.leaf0 },
      isHonestStep' = false,
      stepTag' = "mixed_app",
    }
  }

  action stepTamperExposedApp(appKey: int, n3Levels: int): bool = {
    val base = buildHonestTree(appKey, n3Levels)
    val otherApp = if (appKey == KEY_A) HONEST_APP_B else HONEST_APP_A
    all {
      APP_KEYS.contains(appKey),
      n3Levels >= 1 and n3Levels <= 3,
      currentTree' = { ...base, rootPv: { ...base.rootPv, app: otherApp } },
      isHonestStep' = false,
      stepTag' = "tamper_exposed",
    }
  }

  action stepMixedDepthsN3: bool = {
    val base = buildHonestTree(KEY_A, 2)
    val sD2 = nodeBusSends(base.n3d2, KEY_K3)
    val wRow1 = honestRowWitness(RoleN3, sD2)
    all {
      currentTree' = {
        ...base,
        n3d2: { ...base.n3d2, w1: wRow1 },
        n3d2Row1FromD1: false,
      },
      isHonestStep' = false,
      stepTag' = "mixed_depths",
    }
  }

  action stepCexMutant(cexId: int): bool = {
    val chosen =
      if (cexId == 1) cexMutNoUnsetCheck
      else if (cexId == 2) cexMutNoCcReceive
      else if (cexId == 3) cexMutRootNoN2Pin
      else if (cexId == 4) cexMutRootNoPrehashPin
      else if (cexId == 5) cexMutN3NoCopyN2
      else if (cexId == 6) cexMutSelectorFree
      else if (cexId == 7) cexMutLfrNoPin
      else cexMutN3NoCopyN3
    all {
      cexId >= 1 and cexId <= 8,
      currentTree' = chosen,
      isHonestStep' = false,
      stepTag' = "cex_mutant",
    }
  }

  action step: bool = {
    nondet appKey = APP_KEYS.oneOf()
    nondet n3Levels = Set(1, 2, 3).oneOf()
    nondet cexId = Set(1, 2, 3, 4, 5, 6, 7, 8).oneOf()
    any {
      stepHonest(appKey, n3Levels),
      stepMixedAppKeys(n3Levels),
      stepTamperExposedApp(appKey, n3Levels),
      stepMixedDepthsN3,
      stepCexMutant(cexId),
    }
  }

}
