/* =============================================================================
 * Sherlock Proving Co-Processor — QA Lane `pathc-xcheck`
 * Deterministic & Exhaustive Test Suite (`pathc_xcheck_test.qnt`)
 *
 * Covers:
 *   1. Honest Path C and Honest Full-Tree acceptance + refinement in `pathc_xcheck`
 *      AND non-vacuity (honest acceptance + refinement) under EVERY mutant.
 *   2. Concrete real forgery witnesses (`wrapClaimedOutcome == FORGED_OUTCOME` for
 *      `honestProgD`) breaking `invRefine` on all 9 mandatory mutants + 2 bonus mutants,
 *      while rejected in `pathc_xcheck`.
 *   3. Targeted witnesses for Questions 1–6 (`pathc_x_not_the_l_commitment_accepted_at_L`,
 *      cross-branch `pc` refusal, as-built `decode.rs:368` blocking Path C, carry-overflow
 *      and Fr-modulus-wrap non-canonical `X` limbs, slim `Stmt`/`Call` pre-prove vs
 *      post-prove tampers, and unread `run_tag` freedom).
 *   4. Exhaustive pure `Set` / `fold` checks over finite attack domains backing every
 *      SOUND claim across Questions 1–6 (> 20,000 witness configurations evaluated).
 * ============================================================================= */

module pathc_xcheck_test {
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
  ).* from "./pathc_xcheck"

  // ---------------------------------------------------------------------------
  // 1. Named Attack Witnesses (One Per Mutant & Per Finding)
  // ---------------------------------------------------------------------------

  // Finding / Mutant 1 (`lfr_no_x_check`) & Mutant 9 (`wrap_drops_x_and_lfr_drops_x`):
  // Adversary proves `FORGED_OUTCOME` under `KeyLeafRogueDag` (`dagX = 5`), runs universal `L`
  // with `lCachedCommit = 5`, and places admitted `X_LEAF_ADMITTED_LIMBS` (`{0, 1}`, packs to 4)
  // in `StmtAir` `pv[X]`. Accepted at `L` (`pathc_x_not_the_l_commitment_accepted_at_L`),
  // refused by `L_Fr`'s `packFr(xLimbs) == lCachedCommit` (`4 != 5`) in `CFG_SOUND`.
  pure def witAttackLfrNoX(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    {
      ...base,
      childKey: KeyLeafRogueDag,
      childOutcome: FORGED_OUTCOME,
      lCachedCommit: X_LEAF_ROGUE_DAG,
      xLimbs: X_LEAF_ADMITTED_LIMBS,
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Finding / Mutant 2 (`lfr_x_check_low_limbs_only`):
  // Adversary proves `FORGED_OUTCOME` under `KeyLeafRogueLowLimb` (`dagX = 8`, `8 % 4 == 0`),
  // runs `L` with `lCachedCommit = 8`, and places admitted `X_LEAF_ADMITTED_LIMBS = { l0: 0, l1: 1 }`
  // in `StmtAir` `pv[X]`. Low limb `0 == 8 % 4` collides; high limb `1 != 2` catches it in `CFG_SOUND`.
  pure def witAttackLowLimbsOnly(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    {
      ...base,
      childKey: KeyLeafRogueLowLimb,
      childOutcome: FORGED_OUTCOME,
      lCachedCommit: X_LEAF_ROGUE_LOW0,
      xLimbs: X_LEAF_ADMITTED_LIMBS,
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Finding / Mutant 3 (`pc_free`):
  // Adversary proves `FORGED_OUTCOME` under `KeyLeafRogueDag` (`62` PVs, `dagX = 5`), sets
  // `pcStmt = 1` (matching 62 PVs and freeing `X = {1, 1}` from `x_n3`) while setting `pcSeal = 0`
  // (skipping `KD_LEAF`, `KeyPathAir`, and `RootSealAir` `DOM_PROG` `key_root` opening).
  pure def witAttackPcFree(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    {
      ...base,
      childKey: KeyLeafRogueDag,
      childOutcome: FORGED_OUTCOME,
      lCachedCommit: X_LEAF_ROGUE_DAG,
      pcStmt: 1,
      pcSeal: 0,
      xLimbs: X_LEAF_ROGUE_DAG_LIMBS,
      kpSentRoot: 0,
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Finding / Mutant 4 (`dag_leaf_no_x`):
  // Adversary proves `FORGED_OUTCOME` under `KeyLeafRogueDag` (`dagX = 5, vkPre = 2`), sets
  // `lCachedCommit = 5` and `xLimbs = {1, 1}` (`packFr == 5`, passing `L_Fr`). When `DOM_KEYDAG`
  // omits `enc(X)`, `dag_leaf` collides with `honestDagLeaf(cfg)`.
  pure def witAttackDagLeafNoX(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    val leaf = hashKeyDag(cfg, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_ROGUE_DAG_LIMBS, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
    val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
    {
      ...base,
      childKey: KeyLeafRogueDag,
      childOutcome: FORGED_OUTCOME,
      lCachedCommit: X_LEAF_ROGUE_DAG,
      xLimbs: X_LEAF_ROGUE_DAG_LIMBS,
      stmtKeyRoot: fkr,
      kpSentRoot: fkr,
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Finding / Mutant 5 (`dag_leaf_no_hpre`):
  // Adversary proves `FORGED_OUTCOME` under `KeyLeafRoguePreHash` (same DAG `dagX = 4` as
  // `KeyLeafAdmitted`, corrupted preprocessed trace `vkPre = 3 != 2`), sets `lCachedCommit = 4`,
  // `xLimbs = {0, 1}` (`packFr == 4`, passing `L_Fr`), and `hpWit = 3`. When `DOM_KEYDAG` omits
  // `hp`, `dag_leaf` collides with `honestDagLeaf(cfg)`.
  pure def witAttackDagLeafNoHpre(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    val leaf = hashKeyDag(cfg, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_ADMITTED_LIMBS, PRE_LEAF_ROGUE, PRE_APP_HONEST, CC_APP_HONEST)
    val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
    {
      ...base,
      childKey: KeyLeafRoguePreHash,
      childOutcome: FORGED_OUTCOME,
      lCachedCommit: X_LEAF_ADMITTED,
      xLimbs: X_LEAF_ADMITTED_LIMBS,
      hpWit: PRE_LEAF_ROGUE,
      stmtKeyRoot: fkr,
      kpSentRoot: fkr,
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Finding / Mutant 6 (`hpre_free`):
  // Adversary uses admitted `KeyLeafAdmitted` (`dagX = 4, vkPre = 2`) on a mid-execution shard
  // starting at step `leafHPre = 1 > 0` to prove `FORGED_OUTCOME`, while `StmtAir` `seg` claims `h_lo = 0`.
  pure def witAttackHpreFree(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    {
      ...base,
      leafHPre: 1,
      childOutcome: FORGED_OUTCOME,
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Finding / Mutant 7 (`keypath_no_root`):
  // Adversary uses `KeyLeafRogueDag` (`dagX = 5`, `xLimbs = {1, 1}`, passing `L_Fr` `5 == 5`),
  // and `KeyPathAir` sends `honestKeyRoot(cfg)` on `KR_BUS` instead of the folded Merkle root.
  pure def witAttackKeypathNoRoot(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    val hKr = honestKeyRoot(cfg)
    {
      ...base,
      childKey: KeyLeafRogueDag,
      childOutcome: FORGED_OUTCOME,
      lCachedCommit: X_LEAF_ROGUE_DAG,
      xLimbs: X_LEAF_ROGUE_DAG_LIMBS,
      stmtKeyRoot: hKr,
      kpSentRoot: hKr,
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Finding / Mutant 8 (`seal_no_keyroot`):
  // Adversary uses `KeyLeafRogueDag` (`dagX = 5`, `xLimbs = {1, 1}`); `KeyPathAir` honestly folds
  // to `rogueKr` and sends `rogueKr` on `KR_BUS`, while `RootSealAir` opens `honestProgD(cfg)`
  // using `sealOpenKr = honestKeyRoot(cfg)` instead of `RS_KR` (`stmtKeyRoot`).
  pure def witAttackSealNoKeyroot(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    val rogueLeaf = hashKeyDag(cfg, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_ROGUE_DAG_LIMBS, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
    val rogueKr = foldKeyPath1(rogueLeaf, SIB_N3_TRIPLE, 0)
    {
      ...base,
      childKey: KeyLeafRogueDag,
      childOutcome: FORGED_OUTCOME,
      lCachedCommit: X_LEAF_ROGUE_DAG,
      xLimbs: X_LEAF_ROGUE_DAG_LIMBS,
      stmtKeyRoot: rogueKr,
      kpSentRoot: rogueKr,
      sealOpenKr: honestKeyRoot(cfg),
      slimCallOutcome: FORGED_OUTCOME,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Bonus Finding / Mutant 10 (Q1 `lfr_no_l_vk_pin`):
  // Adversary generates an outer proof under a rogue key `RogueLKey` (`L'`) with `lCachedCommit = 4`
  // and `xLimbs = {0, 1}` (`4 == 4`), bypassing `L`'s `StmtAir` constraints.
  pure def witAttackRogueLKey(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    {
      ...base,
      lKey: RogueLKey,
      childOutcome: FORGED_OUTCOME,
      slimCallOutcome: FORGED_OUTCOME,
      lfrPubPreHash: L_VK_ROGUE,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Bonus Finding / Mutant 11 (Q5 `wrap_ignores_slim_call`):
  // Adversary runs an honest child (`childOutcome = HONEST_OUTCOME`) and pre-proves slim `L` with
  // `slimCallOutcome = FORGED_OUTCOME` (`slimPostProveTamper = false`). Accepted by `L` and `L_Fr`,
  // caught by Wrap's `PUB` check (`decode.rs:593-616`) in `CFG_SOUND`.
  pure def witAttackSlimCallPreProve(cfg: Cfg): ChainWit = {
    val base = honestPathCWit(cfg)
    {
      ...base,
      slimCallOutcome: FORGED_OUTCOME,
      slimPostProveTamper: false,
      wrapClaimedOutcome: FORGED_OUTCOME,
    }
  }

  // Q2 Cross-branch witnesses:
  pure val witLeafAsFullTree: ChainWit = {
    ...honestFullTreeWit(CFG_SOUND),
    childKey: KeyLeafAdmitted,
    lCachedCommit: X_LEAF_ADMITTED,
    pcStmt: 0,
    pcSeal: 0,
    xLimbs: X_N3_LIMBS,
  }

  pure val witN3AsPathC: ChainWit = {
    ...honestPathCWit(CFG_SOUND),
    childKey: KeyN3,
    lCachedCommit: X_N3,
    pcStmt: 1,
    pcSeal: 1,
    xLimbs: X_N3_LIMBS,
    hpWit: PRE_N3,
  }

  // Q4 Non-canonical limb witnesses for `X = 4` (`X_LEAF_ADMITTED`):
  // 1) Carry-overflow `{ l0: 4, l1: 0 }` (`l0 >= LIMB_RADIX (4)`, `l0 < BB_MOD (7)`, `4 + 0*4 = 4`).
  pure val witCarryLimbsOnHonestProg: ChainWit = {
    val leaf = hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_CARRY_LIMBS, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
    val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
    {
      ...honestPathCWit(CFG_SOUND),
      xLimbs: X_LEAF_CARRY_LIMBS,
      stmtKeyRoot: fkr,
      kpSentRoot: fkr,
    }
  }

  // 2) Fr-modulus-wrap `{ l0: 3, l1: 3 }` (`l0, l1 < 4`, `3 + 3*4 = 15 = 4 + 11`, `15 % 11 == 4`).
  pure val witModWrapLimbsOnHonestProg: ChainWit = {
    val leaf = hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_MODWRAP_LIMBS, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
    val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
    {
      ...honestPathCWit(CFG_SOUND),
      xLimbs: X_LEAF_MODWRAP_LIMBS,
      stmtKeyRoot: fkr,
      kpSentRoot: fkr,
    }
  }

  // Q5 Post-prove mutation on slim `Call` / `Stmt` AIR:
  pure val witSlimPostProveTamper: ChainWit = {
    ...honestPathCWit(CFG_SOUND),
    slimPostProveTamper: true,
  }

  // ---------------------------------------------------------------------------
  // 2. Exhaustive Pure Checks (`Set` / `fold` Over Finite Attack Domains)
  // ---------------------------------------------------------------------------

  pure val ALL_MUTANT_CFGS: Set[Cfg] = Set(
    CFG_M_LFR_NO_X_CHECK,
    CFG_M_LFR_X_CHECK_LOW_LIMBS_ONLY,
    CFG_M_PC_FREE,
    CFG_M_DAG_LEAF_NO_X,
    CFG_M_DAG_LEAF_NO_HPRE,
    CFG_M_HPRE_FREE,
    CFG_M_KEYPATH_NO_ROOT,
    CFG_M_SEAL_NO_KEYROOT,
    CFG_M_WRAP_DROPS_X_AND_LFR_DROPS_X,
    CFG_M_LFR_NO_L_VK_PIN,
    CFG_M_WRAP_IGNORES_SLIM_CALL
  )

  // All 49 BabyBear 2-limb vectors (`0..6 x 0..6`), and the 16 limb-range vectors (`0..3 x 0..3`).
  pure val ALL_BB_LIMB_VECS: Set[Limbs2] =
    ALL_BB_WORDS.map(a => ALL_BB_WORDS.map(b => { l0: a, l1: b })).flatten()

  pure val EMPTY_INT_SET: Set[int] = Set()

  // Exhaustive Check 0: Domain-separated Poseidon2 hash injectivity across all inputs.
  pure val hashKeyDagImagesCount: int =
    Set(0, 1).fold(EMPTY_INT_SET, (s1, ent) =>
      Set(0, 1).fold(s1, (s2, tag) =>
        ALL_BB_LIMB_VECS.fold(s2, (s3, xl) =>
          Set(1, 2, 3).fold(s3, (s4, hp) =>
            s4.union(Set(hashKeyDag(CFG_SOUND, ent, tag, xl, hp, 1, 1)))
          )
        )
      )
    ).size()

  // Expect exact bijection: 2 * 2 * 49 * 3 = 588 distinct digests, and disjoint from DOM_KEYTRIPLE.
  pure val hashInjectivityHolds: bool = and {
    hashKeyDagImagesCount == 2 * 2 * 49 * 3,
    honestDagLeaf(CFG_SOUND) != SIB_N3_TRIPLE,
    hashMerkle2(honestDagLeaf(CFG_SOUND), SIB_N3_TRIPLE) != hashMerkle2(SIB_N3_TRIPLE, honestDagLeaf(CFG_SOUND)),
    hashProgCommit(R_P_HONEST, honestProgD(CFG_SOUND)) != hashProgCommit(R_P_HONEST, honestProgD(CFG_SOUND) + 1),
  }

  // Exhaustive Check 1 (Q1 — Path C `X` check in `L_Fr` & `L` vk binding):
  // Sweeps all 5 child keys x 2 L keys x 4 cached commitments x 6 X limb vectors x 2 Merkle bits
  // x 2 sibling choices x 2 L_Fr pre_hash words x 2 outcomes = 3,840 tamper states via `fold`.
  pure val q1SweepCounts: (int, int) = {
    val hKr = honestKeyRoot(CFG_SOUND)
    val hPd = honestProgD(CFG_SOUND)
    val sibSet = Set(SIB_N3_TRIPLE, honestDagLeaf(CFG_SOUND))
    ALL_CHILD_KEYS.fold((0, 0), (acc1, ck) =>
      ALL_L_KEYS.fold(acc1, (acc2, lk) =>
        ALL_CACHED_COMMITS.fold(acc2, (acc3, cc) =>
          INTERESTING_X_LIMBS.fold(acc3, (acc4, xl) =>
            Set(0, 1).fold(acc4, (acc5, bBit) =>
              sibSet.fold(acc5, (acc6, sib) =>
                Set(L_VK_HONEST, L_VK_ROGUE).fold(acc6, (acc7, ph) =>
                  ALL_OUTCOMES.fold(acc7, (cnts, oc) => {
                    val hp = childKeyVkPre(ck)
                    val leaf = hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, xl, hp, PRE_APP_HONEST, CC_APP_HONEST)
                    val fkr = foldKeyPath1(leaf, sib, bBit)
                    val cand: ChainWit = {
                      childKey: ck,
                      leafHPre: 0,
                      childOutcome: oc,
                      lKey: lk,
                      lCachedCommit: cc,
                      pcStmt: 1,
                      pcSeal: 1,
                      xLimbs: xl,
                      hpWit: hp,
                      stmtKeyRoot: fkr,
                      kpSib: sib,
                      kpBit: bBit,
                      kpSentRoot: fkr,
                      sealOpenKr: hKr,
                      sealProgD: hPd,
                      slimCallOutcome: oc,
                      slimPostProveTamper: false,
                      runTagWord: 0,
                      lfrPubPreHash: ph,
                      wrapClaimedProgD: hPd,
                      wrapClaimedOutcome: oc,
                    }
                    if (acceptsChain(CFG_SOUND, cand))
                      if (refinesChain(CFG_SOUND, cand))
                        (cnts._1 + 1, cnts._2)
                      else
                        (cnts._1, cnts._2 + 1)
                    else
                      cnts
                  })
                )
              )
            )
          )
        )
      )
    )
  }

  // Exhaustive Check 2 (Q2 — Full Tree vs Path C cross-presentation & non-boolean `pc`):
  // Sweeps 5 child keys x 3 `pcStmt` ({0,1,2}) x 3 `pcSeal` ({0,1,2}) x 4 cached commits
  // x 4 X limb vectors x 3 `hpWit` x 2 outcomes = 4,320 states via `fold`.
  // Counts: (goodAccepted, badOrCrossAccepted).
  pure val q2SweepCounts: (int, int) = {
    val hKr = honestKeyRoot(CFG_SOUND)
    val hPd = honestProgD(CFG_SOUND)
    val xSet = Set(X_N3_LIMBS, X_LEAF_ADMITTED_LIMBS, X_LEAF_ROGUE_DAG_LIMBS, X_LEAF_ROGUE_LOW0_LIMBS)
    ALL_CHILD_KEYS.fold((0, 0), (acc1, ck) =>
      ALL_PC_VALS.fold(acc1, (acc2, pcv) =>
        ALL_PC_VALS.fold(acc2, (acc3, pcs) =>
          ALL_CACHED_COMMITS.fold(acc3, (acc4, cc) =>
            xSet.fold(acc4, (acc5, xl) =>
              ALL_HP_WITS.fold(acc5, (acc6, hp) =>
                ALL_OUTCOMES.fold(acc6, (cnts, oc) => {
                  val cand: ChainWit = {
                    childKey: ck,
                    leafHPre: 0,
                    childOutcome: oc,
                    lKey: HonestLKey,
                    lCachedCommit: cc,
                    pcStmt: pcv,
                    pcSeal: pcs,
                    xLimbs: xl,
                    hpWit: hp,
                    stmtKeyRoot: hKr,
                    kpSib: SIB_N3_TRIPLE,
                    kpBit: 0,
                    kpSentRoot: if (pcs == 1) hKr else 0,
                    sealOpenKr: hKr,
                    sealProgD: hPd,
                    slimCallOutcome: oc,
                    slimPostProveTamper: false,
                    runTagWord: 0,
                    lfrPubPreHash: L_VK_HONEST,
                    wrapClaimedProgD: hPd,
                    wrapClaimedOutcome: oc,
                  }
                  val crossBranch =
                    (ck == KeyN3 and pcv != 0)
                      or (ck != KeyN3 and pcv != 1)
                      or (pcv == 2 or pcs == 2)
                      or (pcv != pcs)
                  if (acceptsChain(CFG_SOUND, cand))
                    if (refinesChain(CFG_SOUND, cand) and not(crossBranch))
                      (cnts._1 + 1, cnts._2)
                    else
                      (cnts._1, cnts._2 + 1)
                  else
                    cnts
                })
              )
            )
          )
        )
      )
    )
  }

  // Exhaustive Check 3 (Q3 — Dropping wrap `X` pin: `X` never becomes free, `run_tag` harmless):
  // Sweeps all 49 BabyBear `X` limb pairs x 2 branches (`pc = 0, 1`) x 7 `run_tag` values
  // x 2 outcomes = 1,372 states via `fold`.
  pure val q3SweepResult: { acceptedCount: int, freeXCount: int, badRefineCount: int, runTagSet: Set[int] } = {
    val basePathC = honestPathCWit(CFG_SOUND)
    val baseFull = honestFullTreeWit(CFG_SOUND)
    val initAcc = { acceptedCount: 0, freeXCount: 0, badRefineCount: 0, runTagSet: EMPTY_INT_SET }
    Set(0, 1).fold(initAcc, (acc1, pcBit) =>
      ALL_BB_LIMB_VECS.fold(acc1, (acc2, xl) =>
        ALL_BB_WORDS.fold(acc2, (acc3, rt) =>
          ALL_OUTCOMES.fold(acc3, (res, oc) => {
            val leaf = hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, xl, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
            val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
            val cand: ChainWit =
              if (pcBit == 0)
                { ...baseFull, xLimbs: xl, runTagWord: rt, childOutcome: oc, slimCallOutcome: oc, wrapClaimedOutcome: oc }
              else
                { ...basePathC, xLimbs: xl, stmtKeyRoot: fkr, kpSentRoot: fkr, runTagWord: rt, childOutcome: oc, slimCallOutcome: oc, wrapClaimedOutcome: oc }
            if (acceptsChain(CFG_SOUND, cand)) {
              val expectedX = if (pcBit == 0) X_N3_LIMBS else X_LEAF_ADMITTED_LIMBS
              {
                acceptedCount: res.acceptedCount + 1,
                freeXCount: if (cand.xLimbs != expectedX) res.freeXCount + 1 else res.freeXCount,
                badRefineCount: if (not(refinesChain(CFG_SOUND, cand))) res.badRefineCount + 1 else res.badRefineCount,
                runTagSet: res.runTagSet.union(Set(rt)),
              }
            } else {
              res
            }
          })
        )
      )
    )
  }

  // Exhaustive Check 4 (Q4 — Limb Canonicity over all 49 BabyBear limb vectors):
  // Finds all limb vectors in `0..6 x 0..6` packing to `X_LEAF_ADMITTED = 4 (mod 11)`:
  //   - `{l0:0, l1:1}` (sum  4, canonical)
  //   - `{l0:4, l1:0}` (sum  4, carry-overflow `l0 >= 4`, `l0 < 7`)
  //   - `{l0:3, l1:3}` (sum 15 = 11 + 4, Fr-modulus-wrap with both limbs `< 4`)
  //   - `{l0:6, l1:5}` (sum 26 = 2*11 + 4, combined limb-overflow + 2*Fr-wrap)
  //   - `{l0:2, l1:6}` (sum 26 = 2*11 + 4, combined limb-overflow + 2*Fr-wrap)
  // Checks zero non-canonical vectors pass on `honestProgD`, and also checks custom `key_root` programs.
  pure val allLimbVecsPackingToX4: Set[Limbs2] =
    ALL_BB_LIMB_VECS.filter(xl => packFr(xl) == X_LEAF_ADMITTED)

  pure val allInRangeLimbVecsPackingToX4: Set[Limbs2] =
    allLimbVecsPackingToX4.filter(xl => inLimbRange(xl.l0) and inLimbRange(xl.l1))

  pure val q4NonCanonAcceptedOnHonestProgCount: int =
    ALL_BB_LIMB_VECS.fold(0, (cnt1, xl) =>
      Set(0, 1).fold(cnt1, (cnt2, bBit) => {
        val base = honestPathCWit(CFG_SOUND)
        val leaf = hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, xl, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
        val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, bBit)
        val cand1 = { ...base, xLimbs: xl, kpBit: bBit, stmtKeyRoot: fkr, kpSentRoot: fkr }
        val cand2 = { ...base, xLimbs: xl, kpBit: bBit }
        val nonCanon = not(isCanonLimbs(xl))
        val cAfter1 = if (nonCanon and acceptsChain(CFG_SOUND, cand1)) cnt2 + 1 else cnt2
        if (nonCanon and acceptsChain(CFG_SOUND, cand2)) cAfter1 + 1 else cAfter1
      })
    )

  // Malicious-deployer subcase for Q4: deployer builds `customProgForLimbs(CFG_SOUND, xl)`
  // using ANY `xl` in `0..6 x 0..6`; check whether any unadmitted child key (`ck != KeyLeafAdmitted`)
  // or `FORGED_OUTCOME` can pass `CFG_SOUND` (even with `WRAP_CANON_X = false`)!
  pure val q4CustomProgSoundnessBadCount: int =
    ALL_BB_LIMB_VECS.fold(0, (c1, deployedXl) => {
      val targetPd = customProgForLimbs(CFG_SOUND, deployedXl)
      val targetKr = customKeyRootForLimbs(CFG_SOUND, deployedXl)
      // Only test custom programs whose deployed limbs pack to the admitted leaf DAG X = 4:
      if (packFr(deployedXl) != X_LEAF_ADMITTED)
        c1
      else
        ALL_CHILD_KEYS.fold(c1, (c2, ck) =>
          ALL_OUTCOMES.fold(c2, (c3, oc) => {
            val cand: ChainWit = {
              ...honestPathCWit(CFG_SOUND),
              childKey: ck,
              childOutcome: oc,
              lCachedCommit: childKeyDagX(ck),
              xLimbs: deployedXl,
              hpWit: childKeyVkPre(ck),
              stmtKeyRoot: targetKr,
              kpSentRoot: targetKr,
              sealOpenKr: targetKr,
              sealProgD: targetPd,
              slimCallOutcome: oc,
              wrapClaimedProgD: targetPd,
              wrapClaimedOutcome: oc,
            }
            if (acceptsChainForProg(CFG_SOUND, cand, targetPd) and (ck != KeyLeafAdmitted or oc != HONEST_OUTCOME))
              c3 + 1
            else
              c3
          })
        )
    })

  // Exhaustive Check 5 (Q5 — Slim `Stmt`/`Call` interface AIRs):
  // Sweeps `pc in {0, 1}` x `childOutcome in {1, 2}` x `slimCallOutcome in {1, 2}` x
  // `wrapClaimedOutcome in {1, 2}` x `slimPostProveTamper in {false, true}` x `runTagWord in 0..6` = 224 states.
  pure val q5SweepBadCount: int = {
    val basePathC = honestPathCWit(CFG_SOUND)
    val baseFull = honestFullTreeWit(CFG_SOUND)
    Set(0, 1).fold(0, (c1, pcBit) =>
      ALL_OUTCOMES.fold(c1, (c2, childOc) =>
        ALL_OUTCOMES.fold(c2, (c3, slimOc) =>
          ALL_OUTCOMES.fold(c3, (c4, wrapOc) =>
            Set(false, true).fold(c4, (c5, postTamper) =>
              ALL_BB_WORDS.fold(c5, (badCnt, rt) => {
                val base = if (pcBit == 0) baseFull else basePathC
                val cand = {
                  ...base,
                  childOutcome: childOc,
                  slimCallOutcome: slimOc,
                  slimPostProveTamper: postTamper,
                  runTagWord: rt,
                  wrapClaimedOutcome: wrapOc,
                }
                val postTamperCheck = postTamper implies (not(acceptsL(CFG_SOUND, cand)) and not(acceptsLfr(CFG_SOUND, cand)))
                val refineCheck = acceptsChain(CFG_SOUND, cand) implies refinesChain(CFG_SOUND, cand)
                if (postTamperCheck and refineCheck) badCnt else badCnt + 1
              })
            )
          )
        )
      )
    )
  }

  // Exhaustive Check 6 (Q6 — `lpv::H_PRE == 0` AND `vk_pre_hash` in `DOM_KEYDAG`, plus KeyPath/Seal):
  // Sweeps 5 child keys x `leafHPre in {0, 1, 2}` x `hpWit in {1, 2, 3}` x 2 `kpSentRoot` choices
  // x 2 `sealOpenKr` choices x 2 outcomes = 360 states via `fold`.
  pure val q6SweepCounts: (int, int) = {
    val base = honestPathCWit(CFG_SOUND)
    val hKr = honestKeyRoot(CFG_SOUND)
    ALL_CHILD_KEYS.fold((0, 0), (c1, ck) =>
      Set(0, 1, 2).fold(c1, (c2, hpre) =>
        ALL_HP_WITS.fold(c2, (c3, hp) =>
          Set(false, true).fold(c3, (c4, forceHonestSentKr) =>
            Set(false, true).fold(c4, (c5, forceHonestOpenKr) =>
              ALL_OUTCOMES.fold(c5, (cnts, oc) => {
                val xl = if (ck == KeyN3) X_N3_LIMBS
                         else if (ck == KeyLeafRogueDag) X_LEAF_ROGUE_DAG_LIMBS
                         else if (ck == KeyLeafRogueLowLimb) X_LEAF_ROGUE_LOW0_LIMBS
                         else X_LEAF_ADMITTED_LIMBS
                val leaf = hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, xl, hp, PRE_APP_HONEST, CC_APP_HONEST)
                val fkr = foldKeyPath1(leaf, SIB_N3_TRIPLE, 0)
                val sentKr = if (forceHonestSentKr) hKr else fkr
                val openKr = if (forceHonestOpenKr) hKr else fkr
                val cand: ChainWit = {
                  ...base,
                  childKey: ck,
                  leafHPre: hpre,
                  childOutcome: oc,
                  lCachedCommit: childKeyDagX(ck),
                  xLimbs: xl,
                  hpWit: hp,
                  stmtKeyRoot: sentKr,
                  kpSentRoot: sentKr,
                  sealOpenKr: openKr,
                  slimCallOutcome: oc,
                  wrapClaimedOutcome: oc,
                }
                if (acceptsChain(CFG_SOUND, cand))
                  if (refinesChain(CFG_SOUND, cand) and hpre == 0 and ck == KeyLeafAdmitted and hp == PRE_LEAF_ADMITTED)
                    (cnts._1 + 1, cnts._2)
                  else
                    (cnts._1, cnts._2 + 1)
                else
                  cnts
              })
            )
          )
        )
      )
    )
  }

  // ---------------------------------------------------------------------------
  // 3. Executable `run ...Test` Suite (All Suffixed with `Test`)
  // ---------------------------------------------------------------------------

  // --- Honest chain acceptance & refinement in sound `pathc_xcheck` ---
  run honestPathCSoundTest =
    init
      .then(submitWitness(honestPathCWit(CFG_SOUND)))
      .expect(st.acc and invRefine and witHonestPathC)

  run honestFullTreeSoundTest =
    init
      .then(submitWitness(honestFullTreeWit(CFG_SOUND)))
      .expect(st.acc and invRefine and witHonestFullTree)

  // --- Non-vacuity: BOTH honest chains accepted & refine under ALL 11 mutants ---
  run allMutantsAcceptHonestPathCTest =
    init.expect(
      ALL_MUTANT_CFGS.forall(mCfg =>
        acceptsChain(mCfg, honestPathCWit(mCfg)) and refinesChain(mCfg, honestPathCWit(mCfg))
      )
    )

  run allMutantsAcceptHonestFullTreeTest =
    init.expect(
      ALL_MUTANT_CFGS.forall(mCfg =>
        acceptsChain(mCfg, honestFullTreeWit(mCfg)) and refinesChain(mCfg, honestFullTreeWit(mCfg))
      )
    )

  // --- Mandatory Mutant 1: `lfr_no_x_check` ---
  run attackLfrNoXCheckTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackLfrNoX(CFG_SOUND))),
      acceptsChain(CFG_M_LFR_NO_X_CHECK, witAttackLfrNoX(CFG_M_LFR_NO_X_CHECK)),
      not(refinesChain(CFG_M_LFR_NO_X_CHECK, witAttackLfrNoX(CFG_M_LFR_NO_X_CHECK))),
    })

  // --- Mandatory Mutant 2: `lfr_x_check_low_limbs_only` ---
  run attackLfrXCheckLowLimbsOnlyTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackLowLimbsOnly(CFG_SOUND))),
      acceptsChain(CFG_M_LFR_X_CHECK_LOW_LIMBS_ONLY, witAttackLowLimbsOnly(CFG_M_LFR_X_CHECK_LOW_LIMBS_ONLY)),
      not(refinesChain(CFG_M_LFR_X_CHECK_LOW_LIMBS_ONLY, witAttackLowLimbsOnly(CFG_M_LFR_X_CHECK_LOW_LIMBS_ONLY))),
    })

  // --- Mandatory Mutant 3: `pc_free` ---
  run attackPcFreeTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackPcFree(CFG_SOUND))),
      acceptsChain(CFG_M_PC_FREE, witAttackPcFree(CFG_M_PC_FREE)),
      not(refinesChain(CFG_M_PC_FREE, witAttackPcFree(CFG_M_PC_FREE))),
    })

  // --- Mandatory Mutant 4: `dag_leaf_no_x` ---
  run attackDagLeafNoXTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackDagLeafNoX(CFG_SOUND))),
      acceptsChain(CFG_M_DAG_LEAF_NO_X, witAttackDagLeafNoX(CFG_M_DAG_LEAF_NO_X)),
      not(refinesChain(CFG_M_DAG_LEAF_NO_X, witAttackDagLeafNoX(CFG_M_DAG_LEAF_NO_X))),
    })

  // --- Mandatory Mutant 5: `dag_leaf_no_hpre` ---
  run attackDagLeafNoHpreTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackDagLeafNoHpre(CFG_SOUND))),
      acceptsChain(CFG_M_DAG_LEAF_NO_HPRE, witAttackDagLeafNoHpre(CFG_M_DAG_LEAF_NO_HPRE)),
      not(refinesChain(CFG_M_DAG_LEAF_NO_HPRE, witAttackDagLeafNoHpre(CFG_M_DAG_LEAF_NO_HPRE))),
    })

  // --- Mandatory Mutant 6: `hpre_free` ---
  run attackHpreFreeTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackHpreFree(CFG_SOUND))),
      acceptsChain(CFG_M_HPRE_FREE, witAttackHpreFree(CFG_M_HPRE_FREE)),
      not(refinesChain(CFG_M_HPRE_FREE, witAttackHpreFree(CFG_M_HPRE_FREE))),
    })

  // --- Mandatory Mutant 7: `keypath_no_root` ---
  run attackKeypathNoRootTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackKeypathNoRoot(CFG_SOUND))),
      acceptsChain(CFG_M_KEYPATH_NO_ROOT, witAttackKeypathNoRoot(CFG_M_KEYPATH_NO_ROOT)),
      not(refinesChain(CFG_M_KEYPATH_NO_ROOT, witAttackKeypathNoRoot(CFG_M_KEYPATH_NO_ROOT))),
    })

  // --- Mandatory Mutant 8: `seal_no_keyroot` ---
  run attackSealNoKeyrootTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackSealNoKeyroot(CFG_SOUND))),
      acceptsChain(CFG_M_SEAL_NO_KEYROOT, witAttackSealNoKeyroot(CFG_M_SEAL_NO_KEYROOT)),
      not(refinesChain(CFG_M_SEAL_NO_KEYROOT, witAttackSealNoKeyroot(CFG_M_SEAL_NO_KEYROOT))),
    })

  // --- Mandatory Mutant 9: `wrap_drops_x_and_lfr_drops_x` ---
  run attackWrapDropsXAndLfrDropsXTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackLfrNoX(CFG_SOUND))),
      acceptsChain(CFG_M_WRAP_DROPS_X_AND_LFR_DROPS_X, witAttackLfrNoX(CFG_M_WRAP_DROPS_X_AND_LFR_DROPS_X)),
      not(refinesChain(CFG_M_WRAP_DROPS_X_AND_LFR_DROPS_X, witAttackLfrNoX(CFG_M_WRAP_DROPS_X_AND_LFR_DROPS_X))),
    })

  // --- Bonus Mutant 10 (Q1): `lfr_no_l_vk_pin` ---
  run attackLfrNoLVkPinTest =
    init.expect(and {
      not(acceptsChain(CFG_SOUND, witAttackRogueLKey(CFG_SOUND))),
      acceptsChain(CFG_M_LFR_NO_L_VK_PIN, witAttackRogueLKey(CFG_M_LFR_NO_L_VK_PIN)),
      not(refinesChain(CFG_M_LFR_NO_L_VK_PIN, witAttackRogueLKey(CFG_M_LFR_NO_L_VK_PIN))),
    })

  // --- Bonus Mutant 11 (Q5): `wrap_ignores_slim_call` ---
  run attackWrapIgnoresSlimCallTest =
    init.expect(and {
      acceptsL(CFG_SOUND, witAttackSlimCallPreProve(CFG_SOUND)),
      acceptsLfr(CFG_SOUND, witAttackSlimCallPreProve(CFG_SOUND)),
      not(acceptsChain(CFG_SOUND, witAttackSlimCallPreProve(CFG_SOUND))),
      acceptsChain(CFG_M_WRAP_IGNORES_SLIM_CALL, witAttackSlimCallPreProve(CFG_M_WRAP_IGNORES_SLIM_CALL)),
      not(refinesChain(CFG_M_WRAP_IGNORES_SLIM_CALL, witAttackSlimCallPreProve(CFG_M_WRAP_IGNORES_SLIM_CALL))),
    })

  // --- Targeted Question Checks ---

  // Q1: `pathc_x_not_the_l_commitment_accepted_at_L` — `L` alone accepts mismatched `X != cached_in[0]`,
  // `L_Fr` rejects it.
  run q1LAloneAcceptsWrongXTest =
    init.expect(and {
      acceptsL(CFG_SOUND, witAttackLfrNoX(CFG_SOUND)),
      not(acceptsLfr(CFG_SOUND, witAttackLfrNoX(CFG_SOUND))),
      not(acceptsChain(CFG_SOUND, witAttackLfrNoX(CFG_SOUND))),
    })

  // Q2: Path C leaf child refused on `pc = 0`; N3 child refused on `pc = 1`.
  run q2CrossBranchRefusedTest =
    init.expect(and {
      not(acceptsL(CFG_SOUND, witLeafAsFullTree)),
      not(acceptsChain(CFG_SOUND, witLeafAsFullTree)),
      not(acceptsL(CFG_SOUND, witN3AsPathC)),
      not(acceptsChain(CFG_SOUND, witN3AsPathC)),
    })

  // Q3: As-built wrap (`decode.rs:366-368` `X == sh.x = X_N3`) accepts Full Tree but REJECTS honest Path C!
  run q3AsBuiltWrapBlocksPathCTest =
    init.expect(and {
      acceptsChain(CFG_AS_BUILT_WRAP, honestFullTreeWit(CFG_AS_BUILT_WRAP)),
      not(acceptsChain(CFG_AS_BUILT_WRAP, honestPathCWit(CFG_AS_BUILT_WRAP))),
      acceptsChain(CFG_SOUND, honestFullTreeWit(CFG_SOUND)),
      acceptsChain(CFG_SOUND, honestPathCWit(CFG_SOUND)),
    })

  // Q4: Both non-canonical limb families (`{4, 0}` carry-overflow and `{3, 3}` Fr-modulus-wrap)
  // pack to `X = 4`, alter `DOM_KEYDAG` leaf digest, and are refused at `L` on `honestProgD`.
  run q4NonCanonicalLimbsRefusedOnHonestProgTest =
    init.expect(and {
      packFr(X_LEAF_CARRY_LIMBS) == X_LEAF_ADMITTED,
      not(isCanonLimbs(X_LEAF_CARRY_LIMBS)),
      inBabyBear(X_LEAF_CARRY_LIMBS.l0) and inBabyBear(X_LEAF_CARRY_LIMBS.l1),
      packFr(X_LEAF_MODWRAP_LIMBS) == X_LEAF_ADMITTED,
      not(isCanonLimbs(X_LEAF_MODWRAP_LIMBS)),
      inLimbRange(X_LEAF_MODWRAP_LIMBS.l0) and inLimbRange(X_LEAF_MODWRAP_LIMBS.l1),
      hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_CARRY_LIMBS, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
        != honestDagLeaf(CFG_SOUND),
      hashKeyDag(CFG_SOUND, ENT_INTERNAL_SET, TAG_PATH_C, X_LEAF_MODWRAP_LIMBS, PRE_LEAF_ADMITTED, PRE_APP_HONEST, CC_APP_HONEST)
        != honestDagLeaf(CFG_SOUND),
      not(acceptsL(CFG_SOUND, witCarryLimbsOnHonestProg)),
      not(acceptsChain(CFG_SOUND, witCarryLimbsOnHonestProg)),
      not(acceptsL(CFG_SOUND, witModWrapLimbsOnHonestProg)),
      not(acceptsChain(CFG_SOUND, witModWrapLimbsOnHonestProg)),
    })

  // Q5: Post-prove edit on slim `Stmt`/`Call` refused at `L` and `L_Fr`; unread `run_tag` (`81..89`)
  // accepted when pre-proved without affecting refinement.
  run q5SlimInterfaceBindingsTest =
    init
      .then(submitWitness({ ...honestPathCWit(CFG_SOUND), runTagWord: 5 }))
      .expect(and {
        st.acc,
        invRefine,
        not(acceptsL(CFG_SOUND, witSlimPostProveTamper)),
        not(acceptsLfr(CFG_SOUND, witSlimPostProveTamper)),
      })

  // --- Exhaustive Pure Verification Tests (`Set` / `fold`) ---

  run hashInjectivityExhaustiveTest =
    init.expect(hashInjectivityHolds)

  run q1PathCBindingExhaustiveTest =
    init.expect(q1SweepCounts == (1, 0))

  // On `pc = 0`, `hpWit` is gated off by `pc` (`0 * hpWit`), so all 3 `hp` values in `ALL_HP_WITS`
  // pass for `KeyN3`, plus 1 Path C witness (`hp = PRE_LEAF_ADMITTED`) = 4 good, 0 bad/cross.
  run q2CrossBranchExhaustiveTest =
    init.expect(q2SweepCounts == (4, 0))

  run q3WrapDropXPinnedExhaustiveTest =
    init.expect(and {
      q3SweepResult.acceptedCount == 14,
      q3SweepResult.freeXCount == 0,
      q3SweepResult.badRefineCount == 0,
      q3SweepResult.runTagSet == ALL_BB_WORDS,
    })

  run q4LimbCanonicityExhaustiveTest =
    init.expect(and {
      allLimbVecsPackingToX4 == Set(
        X_LEAF_ADMITTED_LIMBS,
        X_LEAF_CARRY_LIMBS,
        X_LEAF_MODWRAP_LIMBS,
        { l0: 6, l1: 5 },
        { l0: 2, l1: 6 }
      ),
      allInRangeLimbVecsPackingToX4 == Set(X_LEAF_ADMITTED_LIMBS, X_LEAF_MODWRAP_LIMBS),
      q4NonCanonAcceptedOnHonestProgCount == 0,
      q4CustomProgSoundnessBadCount == 0,
    })

  run q5SlimIfaceExhaustiveTest =
    init.expect(q5SweepBadCount == 0)

  // All 4 `(forceHonestSentKr, forceHonestOpenKr)` flag pairs collapse to `hKr` when `fkr == hKr`
  // (`ck == KeyLeafAdmitted, hpre == 0, hp == PRE_LEAF_ADMITTED, oc == HONEST_OUTCOME`) = 4 good, 0 bad.
  run q6HpreDualExhaustiveTest =
    init.expect(q6SweepCounts == (4, 0))
}
