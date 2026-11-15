/* =============================================================================
 * Sherlock Proving Co-Processor — QA Lane `pathc-xcheck`
 * Soundness model for T7c Path C (`X` check in `L_Fr`, wrap `X` pin dropped,
 * `StmtAir::eval_root`, `KeyPathAir`, `RootSealAir`, and slim `Stmt`/`Call` AIRs).
 *
 * Models the four-stage verification chain AS BUILT in `lane-src/` (plus the
 * T7c lead-accepted wrap change dropping `decode.rs:366-368`):
 *   Contract/Target <- Wrap (`sherlock-wrap/decode.rs`)
 *                   <- L_Fr (`examples/t7_chain.rs:740-775`, `sherlock-lfr/lverify.rs`)
 *                   <- L    (`sherlock-lfr/stmt.rs`, `sherlock-lfr/iface_l.rs`)
 *                   <- Child (N3 full-tree aggregator or Path C single-leaf layer)
 *
 * Modules in this file:
 *   - `pathc_xcheck_base`                          (parametric types, verifiers & state machine)
 *   - `pathc_xcheck`                               (SOUND T7c instance: L_Fr checks X, wrap drops X)
 *   - `pathc_xcheck_m_lfr_no_x_check`              (mutant: L_Fr omits X == cached_in[0])
 *   - `pathc_xcheck_m_lfr_x_check_low_limbs_only`  (mutant: L_Fr checks only low limb of X)
 *   - `pathc_xcheck_m_pc_free`                     (mutant: pc not boolean / not tied to child & seal)
 *   - `pathc_xcheck_m_dag_leaf_no_x`               (mutant: DOM_KEYDAG omits enc(X))
 *   - `pathc_xcheck_m_dag_leaf_no_hpre`            (mutant: DOM_KEYDAG omits child vk_pre_hash hp)
 *   - `pathc_xcheck_m_hpre_free`                   (mutant: Path C omits pc * lpv::H_PRE == 0)
 *   - `pathc_xcheck_m_keypath_no_root`             (mutant: KeyPathAir KR_BUS not tied to folded root)
 *   - `pathc_xcheck_m_seal_no_keyroot`             (mutant: RootSealAir DOM_PROG ignores KR_BUS key_root)
 *   - `pathc_xcheck_m_wrap_drops_x_and_lfr_drops_x`(mutant: both wrap and L_Fr drop X check)
 *   - `pathc_xcheck_m_lfr_no_l_vk_pin`             (bonus Q1 mutant: L_Fr/wrap omit L vk pin)
 *   - `pathc_xcheck_m_wrap_ignores_slim_call`      (bonus Q5 mutant: wrap ignores slim Call PUB tie)
 *   - `pathc_xcheck_as_built_wrap`                 (diagnostic Q3 instance: decode.rs:368 X == sh.x kept)
 * ============================================================================= */

module pathc_xcheck_base {
  // ---------------------------------------------------------------------------
  // 1. Types
  // ---------------------------------------------------------------------------

  // Small-scale representation of `X` (`iface::LIMBS = 9`, `LIMB_BITS = 30` in Rust):
  // 2 limbs of 2 bits (`LIMB_RADIX = 4`), BabyBear field analog `BB_MOD = 7`,
  // BN254 Fr scalar field analog `FR_MOD = 11`.
  // Preserves both Rust inequalities `2^30 < BB_P` (`4 < 7`) and `r < (2^30)^9` (`11 < 16`).
  type Limbs2 = {
    l0: int,
    l1: int,
  }

  // Child proving keys in the finite universe:
  // - KeyN3:               full-tree N3 key (dagX = 9, vkPre = 1, 90 PVs)
  // - KeyLeafAdmitted:     admitted Path C set-0 leaf key (dagX = 4, vkPre = 2, 62 PVs)
  // - KeyLeafRogueDag:     unadmitted leaf key with different DAG (dagX = 5, vkPre = 2, 62 PVs)
  // - KeyLeafRogueLowLimb: unadmitted leaf key colliding on low limb 0 (dagX = 8, 8 % 4 == 0, vkPre = 2, 62 PVs)
  // - KeyLeafRoguePreHash: unadmitted leaf key sharing DAG 4 with forged vk_pre_hash (dagX = 4, vkPre = 3, 62 PVs)
  type ChildKeyKind =
    | KeyN3
    | KeyLeafAdmitted
    | KeyLeafRogueDag
    | KeyLeafRogueLowLimb
    | KeyLeafRoguePreHash

  // Outer L key used by the prover:
  // - HonestLKey: universal L key compiled into the L_Fr verifier tape
  // - RogueLKey:  alternate L' key with altered AIR constraints and distinct pre_hash
  type LKeyKind =
    | HonestLKey
    | RogueLKey

  // Constraint configuration flags.
  type Cfg = {
    lfrCheckX: bool,
    lfrCheckXAllLimbs: bool,
    lfrPinsLVk: bool,
    wrapPinX: bool,
    wrapCanonX: bool,
    pcConstrained: bool,
    dagLeafHasX: bool,
    dagLeafHasHpre: bool,
    hpreZeroRequired: bool,
    keypathChecksRoot: bool,
    sealChecksKeyroot: bool,
    wrapBindsSlimWords: bool,
  }

  // Complete witness across Child -> L -> L_Fr -> Wrap.
  type ChainWit = {
    childKey: ChildKeyKind,
    leafHPre: int,
    childOutcome: int,
    lKey: LKeyKind,
    lCachedCommit: int,
    pcStmt: int,
    pcSeal: int,
    xLimbs: Limbs2,
    hpWit: int,
    stmtKeyRoot: int,
    kpSib: int,
    kpBit: int,
    kpSentRoot: int,
    sealOpenKr: int,
    sealProgD: int,
    slimCallOutcome: int,
    slimPostProveTamper: bool,
    runTagWord: int,
    lfrPubPreHash: int,
    wrapClaimedProgD: int,
    wrapClaimedOutcome: int,
  }

  // State machine record: current witness, verifier acceptance bit, and step marker.
  type ChainState = {
    wit: ChainWit,
    acc: bool,
    stepped: bool,
  }

  // ---------------------------------------------------------------------------
  // 2. Constants & Domain Sets
  // ---------------------------------------------------------------------------

  pure val LIMB_RADIX: int = 4
  pure val BB_MOD: int = 7
  pure val FR_MOD: int = 11

  pure val X_LEAF_ADMITTED: int = 4
  pure val X_LEAF_ROGUE_DAG: int = 5
  pure val X_LEAF_ROGUE_LOW0: int = 8
  pure val X_N3: int = 9

  pure val X_LEAF_ADMITTED_LIMBS: Limbs2 = { l0: 0, l1: 1 }
  pure val X_LEAF_CARRY_LIMBS: Limbs2 = { l0: 4, l1: 0 }
  pure val X_LEAF_MODWRAP_LIMBS: Limbs2 = { l0: 3, l1: 3 }
  pure val X_LEAF_ROGUE_DAG_LIMBS: Limbs2 = { l0: 1, l1: 1 }
  pure val X_LEAF_ROGUE_LOW0_LIMBS: Limbs2 = { l0: 0, l1: 2 }
  pure val X_N3_LIMBS: Limbs2 = { l0: 1, l1: 2 }

  pure val PRE_N3: int = 1
  pure val PRE_LEAF_ADMITTED: int = 2
  pure val PRE_LEAF_ROGUE: int = 3

  pure val PRE_APP_HONEST: int = 1
  pure val CC_APP_HONEST: int = 1
  pure val ENT_INTERNAL_SET: int = 0

  pure val TAG_FULL_TREE: int = 0
  pure val TAG_PATH_C: int = 1

  pure val PV_COUNT_N3: int = 90
  pure val PV_COUNT_LEAF: int = 62

  pure val L_VK_HONEST: int = 1
  pure val L_VK_ROGUE: int = 2

  pure val IMAGE_HONEST: int = 1
  pure val PLAN_HONEST: int = 1
  pure val MV_HONEST: int = 1
  pure val R_P_HONEST: int = 1

  pure val HONEST_OUTCOME: int = 1
  pure val FORGED_OUTCOME: int = 2

  pure val ALL_CHILD_KEYS: Set[ChildKeyKind] = Set(
    KeyN3,
    KeyLeafAdmitted,
    KeyLeafRogueDag,
    KeyLeafRogueLowLimb,
    KeyLeafRoguePreHash
  )

  pure val ALL_L_KEYS: Set[LKeyKind] = Set(HonestLKey, RogueLKey)
  pure val ALL_CACHED_COMMITS: Set[int] = Set(X_LEAF_ADMITTED, X_LEAF_ROGUE_DAG, X_LEAF_ROGUE_LOW0, X_N3)
  pure val ALL_PC_VALS: Set[int] = Set(0, 1, 2)
  pure val ALL_HPRE_VALS: Set[int] = Set(0, 1)
  pure val ALL_HP_WITS: Set[int] = Set(PRE_N3, PRE_LEAF_ADMITTED, PRE_LEAF_ROGUE)
  pure val ALL_OUTCOMES: Set[int] = Set(HONEST_OUTCOME, FORGED_OUTCOME)
  pure val ALL_BB_WORDS: Set[int] = 0.to(6)

  pure val INTERESTING_X_LIMBS: Set[Limbs2] = Set(
    X_LEAF_ADMITTED_LIMBS,
    X_LEAF_CARRY_LIMBS,
    X_LEAF_MODWRAP_LIMBS,
    X_LEAF_ROGUE_DAG_LIMBS,
    X_LEAF_ROGUE_LOW0_LIMBS,
    X_N3_LIMBS
  )

  // Named configurations (mirroring the concrete module instances below).
  pure val CFG_SOUND: Cfg = {
    lfrCheckX: true,
    lfrCheckXAllLimbs: true,
    lfrPinsLVk: true,
    wrapPinX: false,
    wrapCanonX: false,
    pcConstrained: true,
    dagLeafHasX: true,
    dagLeafHasHpre: true,
    hpreZeroRequired: true,
    keypathChecksRoot: true,
    sealChecksKeyroot: true,
    wrapBindsSlimWords: true,
  }

  pure val CFG_AS_BUILT_WRAP: Cfg = { ...CFG_SOUND, wrapPinX: true, wrapCanonX: true }
  pure val CFG_SOUND_WITH_WRAP_CANON: Cfg = { ...CFG_SOUND, wrapCanonX: true }

  pure val CFG_M_LFR_NO_X_CHECK: Cfg = { ...CFG_SOUND, lfrCheckX: false }
  pure val CFG_M_LFR_X_CHECK_LOW_LIMBS_ONLY: Cfg = { ...CFG_SOUND, lfrCheckXAllLimbs: false }
  pure val CFG_M_PC_FREE: Cfg = { ...CFG_SOUND, pcConstrained: false }
  pure val CFG_M_DAG_LEAF_NO_X: Cfg = { ...CFG_SOUND, dagLeafHasX: false }
  pure val CFG_M_DAG_LEAF_NO_HPRE: Cfg = { ...CFG_SOUND, dagLeafHasHpre: false }
  pure val CFG_M_HPRE_FREE: Cfg = { ...CFG_SOUND, hpreZeroRequired: false }
  pure val CFG_M_KEYPATH_NO_ROOT: Cfg = { ...CFG_SOUND, keypathChecksRoot: false }
  pure val CFG_M_SEAL_NO_KEYROOT: Cfg = { ...CFG_SOUND, sealChecksKeyroot: false }
  pure val CFG_M_WRAP_DROPS_X_AND_LFR_DROPS_X: Cfg = { ...CFG_SOUND, wrapPinX: false, lfrCheckX: false }
  pure val CFG_M_LFR_NO_L_VK_PIN: Cfg = { ...CFG_SOUND, lfrPinsLVk: false }
  pure val CFG_M_WRAP_IGNORES_SLIM_CALL: Cfg = { ...CFG_SOUND, wrapBindsSlimWords: false }

  // ---------------------------------------------------------------------------
  // 3. Pure Arithmetic, Hash (Domain-Separated Injective P2), & Key Helpers
  // ---------------------------------------------------------------------------

  pure def inBabyBear(v: int): bool =
    v >= 0 and v < BB_MOD

  pure def inLimbRange(v: int): bool =
    v >= 0 and v < LIMB_RADIX

  pure def packInt(xl: Limbs2): int =
    xl.l0 + xl.l1 * LIMB_RADIX

  // `lfr_gen_any` (`examples/t7_chain.rs:760-770`): Horner pack in Fr (`mod FR_MOD`)
  // with NO `< 2^30` limb check and NO `< r` (`Canon`) check.
  pure def packFr(xl: Limbs2): int =
    packInt(xl) % FR_MOD

  // `decode.rs:342, 367` (`range < 2^30` and `dec` with `Canon < r`).
  pure def isCanonLimbs(xl: Limbs2): bool =
    inLimbRange(xl.l0) and inLimbRange(xl.l1) and packInt(xl) < FR_MOD

  // Domain-separated injective Poseidon2 models:
  // Prefix 1xxxx: DOM_KEYDAG (`stmt.rs:1108-1121, 1276-1281`)
  pure def hashKeyDag(
    cfg: Cfg,
    ent: int,
    pathTag: int,
    xl: Limbs2,
    hp: int,
    preApp: int,
    ccApp: int
  ): int = {
    val effL0 = if (cfg.dagLeafHasX) xl.l0 else 0
    val effL1 = if (cfg.dagLeafHasX) xl.l1 else 0
    val effHp = if (cfg.dagLeafHasHpre) hp else 0
    10000 + (((((ent * 2 + pathTag) * 8 + effL0) * 8 + effL1) * 4 + effHp) * 2 + preApp) * 2 + ccApp
  }

  // Prefix 2xxxx: DOM_KEYTRIPLE (full-tree chip-set leaf in `key_root`)
  pure def hashKeyTriple(ent: int, tripleId: int): int =
    20000 + ent * 10 + tripleId

  // Prefix 3xxxxxxxxx: DOM_KEYNODE (`stmt.rs:1340`, 1-level Merkle node)
  pure def hashMerkle2(leftLeaf: int, rightLeaf: int): int =
    3000000000 + leftLeaf * 100000 + rightLeaf

  // Prefix 4xxxxxxxxxxx: DOM_PROG (`stmt.rs:1450`, `P2(DOM_PROG, image_d, plan_d, key_root, mv)`)
  pure def hashProg(imgD: int, planD: int, kr: int, mv: int): int =
    400000000000 + kr * 100 + (imgD * 4 + planD) * 2 + mv

  // Prefix 5xxxxxxxxxxxx: DOM_PROGC (`stmt.rs:1439`, `P2(DOM_PROGC, r_P, prog_d)`)
  pure def hashProgCommit(rP: int, progD: int): int =
    5000000000000 + progD * 10 + rP

  pure val SIB_N3_TRIPLE: int = hashKeyTriple(1, 1)

  pure def honestDagLeaf(cfg: Cfg): int =
    hashKeyDag(
      cfg,
      ENT_INTERNAL_SET,
      TAG_PATH_C,
      X_LEAF_ADMITTED_LIMBS,
      PRE_LEAF_ADMITTED,
      PRE_APP_HONEST,
      CC_APP_HONEST
    )

  pure def honestKeyRoot(cfg: Cfg): int =
    hashMerkle2(honestDagLeaf(cfg), SIB_N3_TRIPLE)

  pure def honestProgD(cfg: Cfg): int =
    hashProg(IMAGE_HONEST, PLAN_HONEST, honestKeyRoot(cfg), MV_HONEST)

  // Helper for Q4: a custom program whose `key_root` commits a specific limb vector `xl`
  // for `KeyLeafAdmitted` (`PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST`).
  pure def customKeyRootForLimbs(cfg: Cfg, xl: Limbs2): int = {
    val leaf = hashKeyDag(
      cfg,
      ENT_INTERNAL_SET,
      TAG_PATH_C,
      xl,
      PRE_LEAF_ADMITTED,
      PRE_APP_HONEST,
      CC_APP_HONEST
    )
    hashMerkle2(leaf, SIB_N3_TRIPLE)
  }

  pure def customProgForLimbs(cfg: Cfg, xl: Limbs2): int =
    hashProg(IMAGE_HONEST, PLAN_HONEST, customKeyRootForLimbs(cfg, xl), MV_HONEST)

  // Child key metadata accessors.
  pure def childKeyDagX(ck: ChildKeyKind): int =
    match ck {
      | KeyN3 => X_N3
      | KeyLeafAdmitted => X_LEAF_ADMITTED
      | KeyLeafRogueDag => X_LEAF_ROGUE_DAG
      | KeyLeafRogueLowLimb => X_LEAF_ROGUE_LOW0
      | KeyLeafRoguePreHash => X_LEAF_ADMITTED
    }

  pure def childKeyVkPre(ck: ChildKeyKind): int =
    match ck {
      | KeyN3 => PRE_N3
      | KeyLeafAdmitted => PRE_LEAF_ADMITTED
      | KeyLeafRogueDag => PRE_LEAF_ADMITTED
      | KeyLeafRogueLowLimb => PRE_LEAF_ADMITTED
      | KeyLeafRoguePreHash => PRE_LEAF_ROGUE
    }

  pure def childPvCount(ck: ChildKeyKind): int =
    match ck {
      | KeyN3 => PV_COUNT_N3
      | _ => PV_COUNT_LEAF
    }

  pure def childKeyEnt(ck: ChildKeyKind): int =
    match ck {
      | _ => ENT_INTERNAL_SET
    }

  pure def childKeyPreApp(ck: ChildKeyKind): int =
    match ck {
      | _ => PRE_APP_HONEST
    }

  pure def childKeyCcApp(ck: ChildKeyKind): int =
    match ck {
      | _ => CC_APP_HONEST
    }

  pure def lKeyVkPre(lk: LKeyKind): int =
    match lk {
      | HonestLKey => L_VK_HONEST
      | RogueLKey => L_VK_ROGUE
    }

  // ---------------------------------------------------------------------------
  // 4. Stage-by-Stage Verifier Constraints (Child -> L -> L_Fr -> Wrap)
  // ---------------------------------------------------------------------------

  // Child STARK proof validity:
  // - `KeyN3` verifies the full segment tree from step 0 (`h_lo = 0`) and leaf-0 `DOM_PROG`,
  //   so it can only prove `HONEST_OUTCOME` for `honestProgD(cfg)`.
  // - `KeyLeafAdmitted` from `leafHPre == 0` enforces initial VM state and proves `HONEST_OUTCOME`,
  //   whereas a non-initial shard (`leafHPre > 0`) starts from unconstrained mid-run state
  //   and can validly reach `FORGED_OUTCOME`.
  // - Unadmitted/corrupted leaf keys (`KeyLeafRogueDag`, `KeyLeafRogueLowLimb`, `KeyLeafRoguePreHash`)
  //   can prove `FORGED_OUTCOME` even from `leafHPre == 0`.
  pure def childProofValid(cfg: Cfg, w: ChainWit): bool =
    match w.childKey {
      | KeyN3 =>
          w.childOutcome == HONEST_OUTCOME and w.sealProgD == honestProgD(cfg)
      | KeyLeafAdmitted =>
          if (w.leafHPre == 0)
            w.childOutcome == HONEST_OUTCOME
          else
            (w.leafHPre > 0 and (w.childOutcome == HONEST_OUTCOME or w.childOutcome == FORGED_OUTCOME))
      | _ =>
          w.leafHPre >= 0 and (w.childOutcome == HONEST_OUTCOME or w.childOutcome == FORGED_OUTCOME)
    }

  // `KeyPathAir` (`stmt.rs:1314-1342`): 1-level Merkle fold of `dagLeaf` with `kpSib`.
  pure def foldKeyPath1(dagLeaf: int, sib: int, sideBit: int): int =
    if (sideBit == 0)
      hashMerkle2(dagLeaf, sib)
    else
      hashMerkle2(sib, dagLeaf)

  // `L` STARK verifier (`StmtAir::eval_root`, `KeyPathAir`, `RootSealAir`, slim `IfaceAir`s,
  // `VerifierSubCircuit` in `child_meta` mode, and Fiat-Shamir PoW transcript).
  pure def acceptsL(cfg: Cfg, w: ChainWit): bool = {
    // Public value words live in BabyBear (`0 .. BB_MOD - 1`).
    val bbWordsOk = and {
      inBabyBear(w.xLimbs.l0),
      inBabyBear(w.xLimbs.l1),
      inBabyBear(w.runTagWord),
    }
    // Fiat-Shamir transcript (`lverify.rs:335-340`, `t7_chain.rs:1377-1387`):
    // any post-prove edit to a public word (even on a slim zero-column AIR) fails PoW/challenges.
    val transcriptOk = not(w.slimPostProveTamper)

    match w.lKey {
      | RogueLKey =>
          // A rogue L' key has adversary-chosen AIR constraints; L' alone accepts if public words
          // are BabyBear elements and its own transcript is intact.
          bbWordsOk and transcriptOk
      | HonestLKey => {
          val npcStmt = 1 - w.pcStmt

          // 1. VerifierSubCircuit (`child_meta` mode): verifies child proof against the DAG
          // committed in L's cached trace (`w.lCachedCommit`), WITHOUT seeing `pv[X]`.
          val childSubCircuitOk = and {
            childProofValid(cfg, w),
            w.lCachedCommit == childKeyDagX(w.childKey),
          }

          // 2. `pc` boolean check (`stmt.rs:1183`), `public_values_bus` balance (`stmt.rs:1201-1213`),
          // and `SEAL_BUS` tie (`stmt.rs:1284, 1422`).
          val pcAndPvBusOk =
            if (cfg.pcConstrained)
              and {
                w.pcStmt == 0 or w.pcStmt == 1,
                w.pcSeal == w.pcStmt,
                (w.pcStmt == 0 and childPvCount(w.childKey) == PV_COUNT_N3)
                  or (w.pcStmt == 1 and childPvCount(w.childKey) == PV_COUNT_LEAF),
              }
            else
              (w.pcStmt >= 0 and w.pcStmt <= 2 and (w.pcSeal == 0 or w.pcSeal == 1))

          // 3. `pre_hash_bus` LogUp balance (`stmt.rs:1226-1233`):
          // child sends `vk_pre_hash`, StmtAir receives `npc * n3_pre + pc * hp`.
          val preHashBusOk =
            childKeyVkPre(w.childKey) == npcStmt * PRE_N3 + w.pcStmt * w.hpWit

          // 4. Path C initial-shard check `pc * lw(lpv::H_PRE) == 0` (`stmt.rs:1268-1270`).
          val hpreZeroOk =
            if (cfg.hpreZeroRequired)
              w.pcStmt * w.leafHPre == 0
            else
              true

          // 5. Full-tree `X` constant check `npc * (pv[X] - x_n3) == 0` (`stmt.rs:1273-1275`)
          // and full-tree `key_root` envelope copy (`stmt.rs:1262`).
          val fullTreeXAndKrOk = and {
            npcStmt * (w.xLimbs.l0 - X_N3_LIMBS.l0) == 0,
            npcStmt * (w.xLimbs.l1 - X_N3_LIMBS.l1) == 0,
            npcStmt * (w.stmtKeyRoot - honestKeyRoot(cfg)) == 0,
          }

          // 6. `KD_LEAF` + `KeyPathAir` + `KR_BUS` (`stmt.rs:1276-1281, 1314-1342, 1451`).
          // Telescoping `sum(rcv - en) == rcv_0 in {0, 1}` in KeyPathAir forces KR_BUS send
          // multiplicity in `{0, 1}` matching RootSealAir's `pcv = w.pcSeal`.
          val dagLeaf = hashKeyDag(
            cfg,
            childKeyEnt(w.childKey),
            TAG_PATH_C,
            w.xLimbs,
            w.hpWit,
            childKeyPreApp(w.childKey),
            childKeyCcApp(w.childKey)
          )
          val foldedKr = foldKeyPath1(dagLeaf, w.kpSib, w.kpBit)
          val keyPathAndSealOk =
            if (w.pcSeal == 0)
              true
            else if (w.pcSeal == 1)
              and {
                w.kpBit == 0 or w.kpBit == 1,
                if (cfg.keypathChecksRoot) w.kpSentRoot == foldedKr else true,
                // KR_BUS balance (`stmt.rs:1341, 1451`): RS_KR (`stmtKeyRoot`) == kpSentRoot
                w.stmtKeyRoot == w.kpSentRoot,
                // RootSealAir `DOM_PROG` opening (`stmt.rs:1446-1450`)
                w.sealProgD == hashProg(
                  IMAGE_HONEST,
                  PLAN_HONEST,
                  if (cfg.sealChecksKeyroot) w.stmtKeyRoot else w.sealOpenKr,
                  MV_HONEST
                ),
              }
            else
              false

          and {
            bbWordsOk,
            transcriptOk,
            childSubCircuitOk,
            pcAndPvBusOk,
            preHashBusOk,
            hpreZeroOk,
            fullTreeXAndKrOk,
            keyPathAndSealOk,
          }
        }
    }
  }

  // `L_Fr` verifier tape (`examples/t7_chain.rs:740-775`, `sherlock-lfr/lverify.rs`).
  pure def acceptsLfr(cfg: Cfg, w: ChainWit): bool = {
    val lProofOk = acceptsL(cfg, w)

    // `lverify.rs:292-293, 861-956, 1621`: L_Fr tape unrolls HonestLKey's constraint DAGs
    // and exposes L's `vk.pre_hash` as `pw[0]`.
    val lVkOk =
      if (cfg.lfrPinsLVk)
        w.lKey == HonestLKey and w.lfrPubPreHash == L_VK_HONEST
      else
        w.lfrPubPreHash == lKeyVkPre(w.lKey)

    // `t7_chain.rs:760-770`: `acc = sum_t words[X_WORD + t] * 2^(30 t) mod r`; `assert_eq(acc, cached_in[0])`.
    val xCheckOk =
      if (not(cfg.lfrCheckX))
        true
      else if (cfg.lfrCheckXAllLimbs)
        packFr(w.xLimbs) == w.lCachedCommit
      else
        (w.xLimbs.l0 % LIMB_RADIX) == (w.lCachedCommit % LIMB_RADIX)

    lProofOk and lVkOk and xCheckOk
  }

  // Wrap circuit (`sherlock-wrap/decode.rs`) + contract statement target.
  pure def acceptsWrap(cfg: Cfg, w: ChainWit): bool = {
    val lfrOk = acceptsLfr(cfg, w)

    // `decode.rs:328`: `a.assert_eq_c(&pw[0], sh.pre_hash, "pre-hash")`
    val preHashPinOk =
      if (cfg.lfrPinsLVk)
        w.lfrPubPreHash == L_VK_HONEST
      else
        true

    // `decode.rs:342-346`: unpack range checks (`< BB_P` on head words; optional `< 2^30` + `Canon` on X).
    val unpackOk = and {
      inBabyBear(w.xLimbs.l0),
      inBabyBear(w.xLimbs.l1),
      inBabyBear(w.runTagWord),
      if (cfg.wrapCanonX) isCanonLimbs(w.xLimbs) else true,
      if (cfg.wrapPinX) (isCanonLimbs(w.xLimbs) and w.xLimbs == X_N3_LIMBS) else true,
    }

    // `lverify.rs:1848-1878` (`gen_ops_check`) + `decode.rs:531-616, 868-897`:
    // `C_P = P2(DOM_PROGC, r_P, sealProgD)` binds the program digest in `claim_pub`,
    // and `PUB` entries tie the slim `Call` lanes (`slimCallOutcome`) to `R`'s `childOutcome`.
    val progCommitTieOk =
      hashProgCommit(R_P_HONEST, w.sealProgD) == hashProgCommit(R_P_HONEST, w.wrapClaimedProgD)

    val slimCallDecodeOk =
      if (cfg.wrapBindsSlimWords)
        (w.slimCallOutcome == w.childOutcome and w.wrapClaimedOutcome == w.slimCallOutcome)
      else
        w.wrapClaimedOutcome == w.slimCallOutcome

    lfrOk and preHashPinOk and unpackOk and progCommitTieOk and slimCallDecodeOk
  }

  // Full verification chain acceptance (`acc` in the state machine: contract checks a proof
  // claiming `(wrapClaimedProgD, wrapClaimedOutcome)` for the deployed `honestProgD(cfg)`).
  pure def acceptsChain(cfg: Cfg, w: ChainWit): bool =
    acceptsWrap(cfg, w) and w.wrapClaimedProgD == honestProgD(cfg)

  // Variant used in Q4 to test any caller-specified program digest `targetProgD`
  // (e.g., a maliciously deployed `key_root` containing non-canonical limbs of `X`).
  pure def acceptsChainForProg(cfg: Cfg, w: ChainWit, targetProgD: int): bool =
    acceptsWrap(cfg, w) and w.wrapClaimedProgD == targetProgD

  // ---------------------------------------------------------------------------
  // 5. Reference Semantics (`refinesChain`) & Honest Witnesses
  // ---------------------------------------------------------------------------

  // Reference semantics: the wrap-decoded statement `(wrapClaimedProgD, wrapClaimedOutcome)`
  // is a true execution outcome (`HONEST_OUTCOME`) of `honestProgD(cfg)` from step 0 (`h_lo = 0`),
  // genuinely proved under `HonestLKey` and either:
  //   - full tree: `KeyN3` with cached commitment `X_N3`, or
  //   - path C:    `KeyLeafAdmitted` from `leafHPre == 0` with cached commitment `X_LEAF_ADMITTED`.
  pure def refinesChain(cfg: Cfg, w: ChainWit): bool = and {
    w.wrapClaimedProgD == honestProgD(cfg),
    w.wrapClaimedOutcome == HONEST_OUTCOME,
    w.lKey == HonestLKey,
    or {
      and {
        w.childKey == KeyN3,
        w.lCachedCommit == X_N3,
      },
      and {
        w.childKey == KeyLeafAdmitted,
        w.leafHPre == 0,
        w.lCachedCommit == X_LEAF_ADMITTED,
      },
    },
  }

  // Honest Path C witness (`pc = 1`, single-leaf execution from step 0 under `KeyLeafAdmitted`).
  pure def honestPathCWit(cfg: Cfg): ChainWit = {
    val kr = honestKeyRoot(cfg)
    val pd = honestProgD(cfg)
    {
      childKey: KeyLeafAdmitted,
      leafHPre: 0,
      childOutcome: HONEST_OUTCOME,
      lKey: HonestLKey,
      lCachedCommit: X_LEAF_ADMITTED,
      pcStmt: 1,
      pcSeal: 1,
      xLimbs: X_LEAF_ADMITTED_LIMBS,
      hpWit: PRE_LEAF_ADMITTED,
      stmtKeyRoot: kr,
      kpSib: SIB_N3_TRIPLE,
      kpBit: 0,
      kpSentRoot: kr,
      sealOpenKr: kr,
      sealProgD: pd,
      slimCallOutcome: HONEST_OUTCOME,
      slimPostProveTamper: false,
      runTagWord: 0,
      lfrPubPreHash: L_VK_HONEST,
      wrapClaimedProgD: pd,
      wrapClaimedOutcome: HONEST_OUTCOME,
    }
  }

  // Honest Full-Tree witness (`pc = 0`, N3 proof under `KeyN3` with `X = X_N3`).
  pure def honestFullTreeWit(cfg: Cfg): ChainWit = {
    val kr = honestKeyRoot(cfg)
    val pd = honestProgD(cfg)
    {
      childKey: KeyN3,
      leafHPre: 0,
      childOutcome: HONEST_OUTCOME,
      lKey: HonestLKey,
      lCachedCommit: X_N3,
      pcStmt: 0,
      pcSeal: 0,
      xLimbs: X_N3_LIMBS,
      hpWit: 0,
      stmtKeyRoot: kr,
      kpSib: 0,
      kpBit: 0,
      kpSentRoot: 0,
      sealOpenKr: kr,
      sealProgD: pd,
      slimCallOutcome: HONEST_OUTCOME,
      slimPostProveTamper: false,
      runTagWord: 0,
      lfrPubPreHash: L_VK_HONEST,
      wrapClaimedProgD: pd,
      wrapClaimedOutcome: HONEST_OUTCOME,
    }
  }

  pure def isHonestPathC(cfg: Cfg, w: ChainWit): bool =
    w == honestPathCWit(cfg)

  pure def isHonestFullTree(cfg: Cfg, w: ChainWit): bool =
    w == honestFullTreeWit(cfg)

  pure def buildState(cfg: Cfg, w: ChainWit, didStep: bool): ChainState = {
    wit: w,
    acc: acceptsChain(cfg, w),
    stepped: didStep,
  }

  // ---------------------------------------------------------------------------
  // 6. Module Parameters (`const`), State Variable, Invariants & Actions
  // ---------------------------------------------------------------------------

  const LFR_CHECK_X: bool
  const LFR_CHECK_X_ALL_LIMBS: bool
  const LFR_PINS_L_VK: bool
  const WRAP_PIN_X: bool
  const WRAP_CANON_X: bool
  const PC_CONSTRAINED: bool
  const DAG_LEAF_HAS_X: bool
  const DAG_LEAF_HAS_HPRE: bool
  const HPRE_ZERO_REQUIRED: bool
  const KEYPATH_CHECKS_ROOT: bool
  const SEAL_CHECKS_KEYROOT: bool
  const WRAP_BINDS_SLIM_WORDS: bool

  val activeCfg: Cfg = {
    lfrCheckX: LFR_CHECK_X,
    lfrCheckXAllLimbs: LFR_CHECK_X_ALL_LIMBS,
    lfrPinsLVk: LFR_PINS_L_VK,
    wrapPinX: WRAP_PIN_X,
    wrapCanonX: WRAP_CANON_X,
    pcConstrained: PC_CONSTRAINED,
    dagLeafHasX: DAG_LEAF_HAS_X,
    dagLeafHasHpre: DAG_LEAF_HAS_HPRE,
    hpreZeroRequired: HPRE_ZERO_REQUIRED,
    keypathChecksRoot: KEYPATH_CHECKS_ROOT,
    sealChecksKeyroot: SEAL_CHECKS_KEYROOT,
    wrapBindsSlimWords: WRAP_BINDS_SLIM_WORDS,
  }

  var st: ChainState

  // ---------------------------------------------------------------------------
  // Safety Invariant (`invRefine`) & Non-Vacuity Reachability Invariants
  // ---------------------------------------------------------------------------

  // Primary soundness invariant: every accepted chain witness refines the reference semantics.
  // Must HOLD (exit 0) in `pathc_xcheck`, and be VIOLATED (exit 1) in every mutant `pathc_xcheck_m_*`.
  val invRefine: bool =
    st.acc implies refinesChain(activeCfg, st.wit)

  // Negated reachability invariants that MUST BE VIOLATED (exit 1) in `pathc_xcheck`:
  val reachHonestPathC: bool =
    not(st.stepped and st.acc and isHonestPathC(activeCfg, st.wit))

  val reachHonestFullTree: bool =
    not(st.stepped and st.acc and isHonestFullTree(activeCfg, st.wit))

  val reachAcceptedUnreadRunTag: bool =
    not(st.stepped and st.acc and st.wit.runTagWord != 0 and refinesChain(activeCfg, st.wit))

  val reachRejectedTamper: bool =
    not(st.stepped and not(st.acc))

  // Negated bad-tamper reachability invariant: MUST NOT BE VIOLATED (exit 0) in `pathc_xcheck`,
  // and MUST BE VIOLATED (exit 1) in every mutant `pathc_xcheck_m_*`.
  val reachAcceptedBadTamper: bool =
    not(st.acc and not(refinesChain(activeCfg, st.wit)))

  // Positive witness predicates (for `--witnesses` if desired).
  val witHonestPathC: bool =
    st.stepped and st.acc and isHonestPathC(activeCfg, st.wit)

  val witHonestFullTree: bool =
    st.stepped and st.acc and isHonestFullTree(activeCfg, st.wit)

  // ---------------------------------------------------------------------------
  // Actions
  // ---------------------------------------------------------------------------

  action init: bool = all {
    st' = buildState(activeCfg, honestPathCWit(activeCfg), false),
  }

  action submitWitness(cand: ChainWit): bool = all {
    st' = buildState(activeCfg, cand, true),
  }

  // Nondeterministic adversary exploration across honest witnesses, focused tamper families,
  // and unconstrained combinations of adversary-controlled witness fields.
  action step: bool = {
    val basePathC = honestPathCWit(activeCfg)
    val baseFull = honestFullTreeWit(activeCfg)
    val hKr = honestKeyRoot(activeCfg)
    val hPd = honestProgD(activeCfg)
    any {
      // 1. Honest Path C chain
      submitWitness(basePathC),

      // 2. Honest Full-Tree (N3) chain
      submitWitness(baseFull),

      // 3. Unread `run_tag` (`81..89`) variation before L proving (Q3 / Q5)
      {
        nondet rt = Set(1, 2, 3, 4, 5, 6).oneOf()
        submitWitness({ ...basePathC, runTagWord: rt })
      },

      // 4. Rogue child DAG + mismatched X vs L cached commitment (Q1 / lfr_no_x_check /
      //    lfr_x_check_low_limbs_only / wrap_drops_x_and_lfr_drops_x / dag_leaf_no_x)
      {
        nondet ck = Set(KeyLeafRogueDag, KeyLeafRogueLowLimb).oneOf()
        nondet xl = Set(X_LEAF_ADMITTED_LIMBS, X_LEAF_ROGUE_DAG_LIMBS, X_LEAF_ROGUE_LOW0_LIMBS).oneOf()
        nondet oc = ALL_OUTCOMES.oneOf()
        val cc = childKeyDagX(ck)
        val leaf = hashKeyDag(activeCfg, ENT_INTERNAL_SET, TAG_PATH_C, xl, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
        val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
        submitWitness({
          ...basePathC,
          childKey: ck,
          childOutcome: oc,
          lCachedCommit: cc,
          xLimbs: xl,
          stmtKeyRoot: fkr,
          kpSentRoot: fkr,
          slimCallOutcome: oc,
          wrapClaimedOutcome: oc,
        })
      },

      // 5. Rogue child vk_pre_hash sharing admitted DAG X = 4 (Q6 / dag_leaf_no_hpre)
      {
        nondet oc = ALL_OUTCOMES.oneOf()
        val leaf = hashKeyDag(activeCfg, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_ADMITTED_LIMBS, PRE_LEAF_ROGUE, PRE_APP_HONEST, CC_APP_HONEST)
        val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
        submitWitness({
          ...basePathC,
          childKey: KeyLeafRoguePreHash,
          childOutcome: oc,
          lCachedCommit: X_LEAF_ADMITTED,
          xLimbs: X_LEAF_ADMITTED_LIMBS,
          hpWit: PRE_LEAF_ROGUE,
          stmtKeyRoot: fkr,
          kpSentRoot: fkr,
          slimCallOutcome: oc,
          wrapClaimedOutcome: oc,
        })
      },

      // 6. Non-initial leaf shard `leafHPre > 0` on Path C (Q6 / hpre_free)
      {
        nondet hpre = Set(1, 2).oneOf()
        nondet oc = ALL_OUTCOMES.oneOf()
        submitWitness({
          ...basePathC,
          leafHPre: hpre,
          childOutcome: oc,
          slimCallOutcome: oc,
          wrapClaimedOutcome: oc,
        })
      },

      // 7. Cross-branch / non-boolean `pc` tampers (Q2 / pc_free)
      {
        nondet ck = Set(KeyN3, KeyLeafAdmitted, KeyLeafRogueDag).oneOf()
        nondet pcv = ALL_PC_VALS.oneOf()
        nondet pcs = Set(0, 1).oneOf()
        nondet xl = Set(X_N3_LIMBS, X_LEAF_ADMITTED_LIMBS, X_LEAF_ROGUE_DAG_LIMBS).oneOf()
        nondet hp = ALL_HP_WITS.oneOf()
        nondet oc = ALL_OUTCOMES.oneOf()
        submitWitness({
          ...basePathC,
          childKey: ck,
          childOutcome: oc,
          lCachedCommit: childKeyDagX(ck),
          pcStmt: pcv,
          pcSeal: pcs,
          xLimbs: xl,
          hpWit: hp,
          stmtKeyRoot: hKr,
          kpSentRoot: if (pcs == 1) hKr else 0,
          sealOpenKr: hKr,
          sealProgD: hPd,
          slimCallOutcome: oc,
          wrapClaimedOutcome: oc,
        })
      },

      // 8. KeyPathAir / RootSealAir `key_root` decoupling tampers (keypath_no_root / seal_no_keyroot)
      {
        nondet useHonestSentKr = Set(true, false).oneOf()
        nondet oc = ALL_OUTCOMES.oneOf()
        val rogueLeaf = hashKeyDag(activeCfg, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_ROGUE_DAG_LIMBS, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
        val rogueKr = foldKeyPath1(rogueLeaf, SIB_N3_TRIPLE, 0)
        val sentKr = if (useHonestSentKr) hKr else rogueKr
        submitWitness({
          ...basePathC,
          childKey: KeyLeafRogueDag,
          childOutcome: oc,
          lCachedCommit: X_LEAF_ROGUE_DAG,
          xLimbs: X_LEAF_ROGUE_DAG_LIMBS,
          stmtKeyRoot: sentKr,
          kpSentRoot: sentKr,
          sealOpenKr: hKr,
          sealProgD: hPd,
          slimCallOutcome: oc,
          wrapClaimedOutcome: oc,
        })
      },

      // 9. Non-canonical limb vectors for X = 4 (Q4: carry-overflow {4,0} and modulus-wrap {3,3})
      {
        nondet xl = Set(X_LEAF_CARRY_LIMBS, X_LEAF_MODWRAP_LIMBS).oneOf()
        nondet bBit = Set(0, 1).oneOf()
        nondet useFoldedKr = Set(false, true).oneOf()
        val leaf = hashKeyDag(activeCfg, ENT_INTERNAL_SET, TAG_PATH_C, xl, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
        val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, bBit)
        val krPick = if (useFoldedKr) fkr else hKr
        submitWitness({
          ...basePathC,
          xLimbs: xl,
          kpBit: bBit,
          stmtKeyRoot: krPick,
          kpSentRoot: krPick,
        })
      },

      // 10. Rogue L' outer key tamper (Q1 / lfr_no_l_vk_pin)
      {
        nondet ph = Set(L_VK_HONEST, L_VK_ROGUE).oneOf()
        nondet oc = ALL_OUTCOMES.oneOf()
        submitWitness({
          ...basePathC,
          lKey: RogueLKey,
          childOutcome: oc,
          slimCallOutcome: oc,
          lfrPubPreHash: ph,
          wrapClaimedOutcome: oc,
        })
      },

      // 11. Slim `Stmt`/`Call` interface AIR pre-prove and post-prove tampers (Q5 / wrap_ignores_slim_call)
      {
        nondet tamperPost = Set(true, false).oneOf()
        nondet slimOc = ALL_OUTCOMES.oneOf()
        nondet wrapOc = ALL_OUTCOMES.oneOf()
        submitWitness({
          ...basePathC,
          slimCallOutcome: slimOc,
          slimPostProveTamper: tamperPost,
          wrapClaimedOutcome: wrapOc,
        })
      },

      // 12. General wide nondeterministic prover draw across all layers
      {
        nondet ck = ALL_CHILD_KEYS.oneOf()
        nondet hpre = ALL_HPRE_VALS.oneOf()
        nondet oc = ALL_OUTCOMES.oneOf()
        nondet lk = ALL_L_KEYS.oneOf()
        nondet cc = ALL_CACHED_COMMITS.oneOf()
        nondet pcv = ALL_PC_VALS.oneOf()
        nondet pcs = Set(0, 1).oneOf()
        nondet xl = INTERESTING_X_LIMBS.oneOf()
        nondet hp = ALL_HP_WITS.oneOf()
        nondet bBit = Set(0, 1).oneOf()
        nondet tamperPost = Set(false, true).oneOf()
        nondet slimOc = ALL_OUTCOMES.oneOf()
        nondet rt = Set(0, 1, 6).oneOf()
        nondet ph = Set(L_VK_HONEST, L_VK_ROGUE).oneOf()
        nondet useFoldedKr = Set(false, true).oneOf()
        val leaf = hashKeyDag(activeCfg, ENT_INTERNAL_SET, TAG_PATH_C, xl, hp, PRE_APP_HONEST, CC_APP_HONEST)
        val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, bBit)
        val krChoice = if (useFoldedKr) fkr else hKr
        submitWitness({
          childKey: ck,
          leafHPre: hpre,
          childOutcome: oc,
          lKey: lk,
          lCachedCommit: cc,
          pcStmt: pcv,
          pcSeal: pcs,
          xLimbs: xl,
          hpWit: hp,
          stmtKeyRoot: krChoice,
          kpSib: SIB_N3_TRIPLE,
          kpBit: bBit,
          kpSentRoot: krChoice,
          sealOpenKr: hKr,
          sealProgD: hPd,
          slimCallOutcome: slimOc,
          slimPostProveTamper: tamperPost,
          runTagWord: rt,
          lfrPubPreHash: ph,
          wrapClaimedProgD: hPd,
          wrapClaimedOutcome: slimOc,
        })
      },
    }
  }
}

/* =============================================================================
 * Concrete Sound Module & Mutant Instances
 * ============================================================================= */

// SOUND T7c design: L_Fr checks `X == cached_in[0]` across all limbs; wrap drops `X == sh.x`
// (and even drops `< 2^30` / `Canon` on `X`).
module pathc_xcheck {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 1: `lfr_no_x_check`
module pathc_xcheck_m_lfr_no_x_check {
  import pathc_xcheck_base(
    LFR_CHECK_X = false,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 2: `lfr_x_check_low_limbs_only`
module pathc_xcheck_m_lfr_x_check_low_limbs_only {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = false,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 3: `pc_free`
module pathc_xcheck_m_pc_free {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = false,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 4: `dag_leaf_no_x`
module pathc_xcheck_m_dag_leaf_no_x {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = false,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 5: `dag_leaf_no_hpre`
module pathc_xcheck_m_dag_leaf_no_hpre {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = false,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 6: `hpre_free`
module pathc_xcheck_m_hpre_free {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = false,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 7: `keypath_no_root`
module pathc_xcheck_m_keypath_no_root {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = false,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 8: `seal_no_keyroot`
module pathc_xcheck_m_seal_no_keyroot {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = false,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Mandatory Mutant 9: `wrap_drops_x_and_lfr_drops_x`
module pathc_xcheck_m_wrap_drops_x_and_lfr_drops_x {
  import pathc_xcheck_base(
    LFR_CHECK_X = false,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Bonus Mutant 10 (Q1): `lfr_no_l_vk_pin` (L_Fr and wrap fail to pin L's vk / constraints)
module pathc_xcheck_m_lfr_no_l_vk_pin {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = false,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}

// Bonus Mutant 11 (Q5): `wrap_ignores_slim_call` (wrap omits PUB tie on slim Call AIR words)
module pathc_xcheck_m_wrap_ignores_slim_call {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = false,
    WRAP_CANON_X = false,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = false
  ).*
}

// Diagnostic Instance 12 (Q3): AS-BUILT wrap (`sherlock-wrap/decode.rs:366-368` still pins `X == sh.x = X_N3`).
// Sound (`invRefine` holds, exit 0), accepts Full Tree, but REJECTS honest Path C (`reachHonestPathC` holds, exit 0).
module pathc_xcheck_as_built_wrap {
  import pathc_xcheck_base(
    LFR_CHECK_X = true,
    LFR_CHECK_X_ALL_LIMBS = true,
    LFR_PINS_L_VK = true,
    WRAP_PIN_X = true,
    WRAP_CANON_X = true,
    PC_CONSTRAINED = true,
    DAG_LEAF_HAS_X = true,
    DAG_LEAF_HAS_HPRE = true,
    HPRE_ZERO_REQUIRED = true,
    KEYPATH_CHECKS_ROOT = true,
    SEAL_CHECKS_KEYROOT = true,
    WRAP_BINDS_SLIM_WORDS = true
  ).*
}
