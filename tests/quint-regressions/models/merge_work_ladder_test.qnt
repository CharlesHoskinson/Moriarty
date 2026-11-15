// -*- mode: Bluespec; -*-
// Deterministic unit tests, mutant attack witnesses, honest-witness checks,
// and exhaustive pure tamper-domain checks for `merge_work_ladder.qnt`.

module merge_work_ladder_test {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = SOUND_CFG) as sound_m from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_W_LEFT) as m_w_left from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_NO_PRE_LINK) as m_no_pre_link from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_NO_H_LINK) as m_no_h_link from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_NO_PROG_LINK) as m_no_prog_link from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_H_WRAP) as m_h_wrap from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_WHI_NO_RANGE) as m_whi_no_range from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_MERGE_NO_CONTIG) as m_merge_no_contig from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_STEPV_ONCE) as m_stepv_once from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_STEPV_WF_FREE) as m_stepv_wf_free from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_CORE_SEND_ROW0_ONLY) as m_core_send_row0_only from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_ROW_UNBOUND) as m_row_unbound from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_HALF_SPLIT_OFF) as m_half_split_off from "./merge_work_ladder"
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_KEY_CHECK_NONE) as m_key_check_none from "./merge_work_ladder"

  pure val U7: int = 7

  // ==================================================================
  // 1. Sound baseline & honest witness checks across sound + all mutants
  // ==================================================================

  run soundHonestTest =
    sound_m::init.expect(and {
      sound_m::acc,
      sound_m::invRefine,
      sound_m::invUniversalKeys,
      refinesWitness(U7, sound_m::wit),
    })

  // Every mutant that preserves the honest right-end 8-row layout MUST still accept
  // the honest witness AND have it refine (`PROMPT.md:18`, non-vacuity of mutants).
  run honestPreservedMutantsTest =
    sound_m::init.expect(and {
      evalAccepted(U7, { ...SOUND_CFG, pre_link: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, h_link: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, prog_link: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, carry_bool: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, whi_bounded: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, merge_contig: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, stepv_wf_pinned: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, core_send_all_rows: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, all_rows_bound: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, strict_half_split: false }, honestWitness(U7)),
      evalAccepted(U7, { ...SOUND_CFG, parent_checks_vkc: false }, honestWitness(U7)),
      refinesWitness(U7, honestWitness(U7)),
    })

  run mNoPreLinkHonestTest =
    m_no_pre_link::init.expect(m_no_pre_link::acc and m_no_pre_link::invRefine)

  run mNoHLinkHonestTest =
    m_no_h_link::init.expect(m_no_h_link::acc and m_no_h_link::invRefine)

  run mNoProgLinkHonestTest =
    m_no_prog_link::init.expect(m_no_prog_link::acc and m_no_prog_link::invRefine)

  run mHWrapHonestTest =
    m_h_wrap::init.expect(m_h_wrap::acc and m_h_wrap::invRefine)

  run mWhiNoRangeHonestTest =
    m_whi_no_range::init.expect(m_whi_no_range::acc and m_whi_no_range::invRefine)

  run mMergeNoContigHonestTest =
    m_merge_no_contig::init.expect(m_merge_no_contig::acc and m_merge_no_contig::invRefine)

  run mStepvWfFreeHonestTest =
    m_stepv_wf_free::init.expect(m_stepv_wf_free::acc and m_stepv_wf_free::invRefine)

  run mCoreSendRow0OnlyHonestTest =
    m_core_send_row0_only::init.expect(m_core_send_row0_only::acc and m_core_send_row0_only::invRefine)

  run mRowUnboundHonestTest =
    m_row_unbound::init.expect(m_row_unbound::acc and m_row_unbound::invRefine)

  run mHalfSplitOffHonestTest =
    m_half_split_off::init.expect(m_half_split_off::acc and m_half_split_off::invRefine)

  run mKeyCheckNoneHonestTest =
    m_key_check_none::init.expect(m_key_check_none::acc and m_key_check_none::invRefine)

  // Two mutants intentionally change the honest witness layout itself (`PROMPT.md:18`):
  // - `w_left` moves W's link from right of c1 to left of c0 (so right-end honest witness is rejected).
  // - `stepv_once` changes StmtAir's STEP_PV_BUS receive multiplicity from 8 to 1 (so 8-row CORE != 1).
  run layoutChangingMutantsDocumentedTest =
    m_w_left::init
      .expect(not(m_w_left::acc) and m_w_left::invRefine)
      .then(all {
        m_w_left::assignWitness(honestWitness(U7)),
        assert(not(evalAccepted(U7, { ...SOUND_CFG, stepv_mult_8: false }, honestWitness(U7)))),
      })
      .expect(not(m_w_left::acc))

  // ==================================================================
  // 2. Concrete real attacks catching every mutant (all 13 mutants)
  // ==================================================================

  // Attack 1 (`w_left`): W = [0, 1) prepended at the left of c0 = [1, 3), c1 = [3, 5).
  // Accepted by `m_w_left`, rejected by `sound_m`, violates SHP-003 right-end refinement.
  pure val attackWLeftWitness: Witness = {
    val base_w = honestWitness(U7)
    val c0 = mkN3SegProof(ProgA, In1, 1, 3, true)
    val c1 = mkN3SegProof(ProgA, In1, 3, 5, true)
    val sw = mkStepWords(ProgA, 100, 0, 1)
    val ww: WorkWitness = {
      ...base_w.work,
      c0: c0,
      c1: c1,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      whi0: limb2OfInt(1),
      wcry0: { lo: 0, hi: 0 },
      pv_seg: mkTrueSeg(ProgA, In1, 0, 5, true),
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgA,
        true_pre: 100,
        true_steps: 1,
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mWLeftAttackTest =
    m_w_left::init
      .then(m_w_left::assignWitness(attackWLeftWitness))
      .expect(and {
        m_w_left::acc,
        not(m_w_left::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackWLeftWitness)),
      })

  // Attack 2 (`no_pre_link`): c0=[0,2), c1=[2,4) on In1 (100->104), W=[4,5) on In2 (204->205).
  // Splices In1's initial state 100 with In2's final state 205 under one `prog_d`.
  pure val attackNoPreLinkWitness: Witness = {
    val base_w = honestWitness(U7)
    val sw = mkStepWords(ProgA, 204, 4, 1)
    val ww: WorkWitness = {
      ...base_w.work,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      pv_seg: { ...base_w.work.pv_seg, post_d: 205 },
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgA,
        true_pre: 204,
        true_steps: 1,
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mNoPreLinkAttackTest =
    m_no_pre_link::init
      .then(m_no_pre_link::assignWitness(attackNoPreLinkWitness))
      .expect(and {
        m_no_pre_link::acc,
        not(m_no_pre_link::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackNoPreLinkWitness)),
      })

  // Attack 3 (`no_h_link`): c0=[0,2), c1=[2,4) (100->104), W executes 104->105 at forged h_pre=8 (`INIT` off).
  // Forges output height `h_hi = 9` for a 5-step execution.
  pure val attackNoHLinkWitness: Witness = {
    val base_w = honestWitness(U7)
    val sw = mkStepWords(ProgA, 104, 8, 1)
    val ww: WorkWitness = {
      ...base_w.work,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      whi0: limb2OfInt(9),
      pv_seg: { ...base_w.work.pv_seg, h_hi: limb2OfInt(9) },
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgA,
        true_pre: 104,
        true_steps: 1,
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mNoHLinkAttackTest =
    m_no_h_link::init
      .then(m_no_h_link::assignWitness(attackNoHLinkWitness))
      .expect(and {
        m_no_h_link::acc,
        not(m_no_h_link::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackNoHLinkWitness)),
      })

  // Attack 4 (`no_prog_link`): c0=[0,2), c1=[2,4) on ProgA (100->104), W runs 1 step of ProgB (104->114).
  // Forges output `(ProgA, 100, 114, 0, 5)`.
  pure val attackNoProgLinkWitness: Witness = {
    val base_w = honestWitness(U7)
    val sw = mkStepWords(ProgB, 104, 4, 1)
    val ww: WorkWitness = {
      ...base_w.work,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      pv_seg: { ...base_w.work.pv_seg, post_d: 114 },
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgB,
        true_pre: 104,
        true_steps: 1,
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mNoProgLinkAttackTest =
    m_no_prog_link::init
      .then(m_no_prog_link::assignWitness(attackNoProgLinkWitness))
      .expect(and {
        m_no_prog_link::acc,
        not(m_no_prog_link::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackNoProgLinkWitness)),
      })

  // Attack 5 (`h_wrap`): c0=[0,200), c1=[200,255), W at h_pre=[15,15]=255, steps=1.
  // With wcry=[1,1], wraps 256 -> whi=[0,0] (`h_hi = 0`).
  pure val attackHWrapWitness: Witness = {
    val base_w = honestWitness(U7)
    val c0 = mkN3SegProof(ProgA, In1, 0, 200, true)
    val c1 = mkN3SegProof(ProgA, In1, 200, 255, true)
    val sw = mkStepWords(ProgA, 355, 255, 1)
    val ww: WorkWitness = {
      ...base_w.work,
      c0: c0,
      c1: c1,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      whi0: { lo: 0, hi: 0 },
      wcry0: { lo: 1, hi: 1 },
      pv_seg: { ...c0.env, post_d: 356, h_hi: { lo: 0, hi: 0 } },
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgA,
        true_pre: 355,
        true_steps: 1,
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mHWrapAttackTest =
    m_h_wrap::init
      .then(m_h_wrap::assignWitness(attackHWrapWitness))
      .expect(and {
        m_h_wrap::acc,
        not(m_h_wrap::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackHWrapWitness)),
      })

  // Attack 6A (`whi_no_range` — REAL DEFECT in `stmt.rs:1049-1055`):
  // On the honest 5-step run (h_pre=[4,0], steps=1), boolean carry `wcry=[1,0]` underflows limb 0
  // mod 257 to `whi = [246, 1]` (`246 + 16*1 = 262 ≡ 5 mod 257`), forging `H_HI` all the way to `L`!
  pure val attackWhiFalseCarryWitness: Witness = {
    val base_w = honestWitness(U7)
    val bad_whi: Limb2 = { lo: 246, hi: 1 }
    val ww: WorkWitness = {
      ...base_w.work,
      whi0: bad_whi,
      wcry0: { lo: 1, hi: 0 },
      pv_seg: { ...base_w.work.pv_seg, h_hi: bad_whi },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  // Attack 6B (`whi_no_range` — REAL DEFECT wrap bypass in `stmt.rs:1049-1055`):
  // At h_pre=[15,15]=255, steps=1 (sum=256, wraps 2^8), setting `wcry=[0,0]` and `whi=[16,15]`
  // satisfies `assert_bool(wcry)` and `assert_zero(wcry[last])`, completely bypassing wrap detection!
  pure val attackWhiWrapBypassWitness: Witness = {
    val base_w = honestWitness(U7)
    val c0 = mkN3SegProof(ProgA, In1, 0, 200, true)
    val c1 = mkN3SegProof(ProgA, In1, 200, 255, true)
    val sw = mkStepWords(ProgA, 355, 255, 1)
    val bad_whi: Limb2 = { lo: 16, hi: 15 }
    val ww: WorkWitness = {
      ...base_w.work,
      c0: c0,
      c1: c1,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      whi0: bad_whi,
      wcry0: { lo: 0, hi: 0 },
      pv_seg: { ...c0.env, post_d: 356, h_hi: bad_whi },
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgA,
        true_pre: 355,
        true_steps: 1,
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mWhiNoRangeFalseCarryAttackTest =
    m_whi_no_range::init
      .then(m_whi_no_range::assignWitness(attackWhiFalseCarryWitness))
      .expect(and {
        m_whi_no_range::acc,
        not(m_whi_no_range::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackWhiFalseCarryWitness)),
      })

  run mWhiNoRangeWrapBypassAttackTest =
    m_whi_no_range::init
      .then(m_whi_no_range::assignWitness(attackWhiWrapBypassWitness))
      .expect(and {
        m_whi_no_range::acc,
        not(m_whi_no_range::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackWhiWrapBypassWitness)),
      })

  // Attack 7 (`merge_no_contig`): c0=[0,2) on In1 (100->102), c1=[2,4) on In2 (202->204), W=[4,5) on In2 (204->205).
  pure val attackMergeNoContigWitness: Witness = {
    val base_w = honestWitness(U7)
    val c1 = mkN3SegProof(ProgA, In2, 2, 4, true)
    val sw = mkStepWords(ProgA, 204, 4, 1)
    val ww: WorkWitness = {
      ...base_w.work,
      c1: c1,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      pv_seg: { ...base_w.work.pv_seg, post_d: 205 },
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgA,
        true_pre: 204,
        true_steps: 1,
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mMergeNoContigAttackTest =
    m_merge_no_contig::init
      .then(m_merge_no_contig::assignWitness(attackMergeNoContigWitness))
      .expect(and {
        m_merge_no_contig::acc,
        not(m_merge_no_contig::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackMergeNoContigWitness)),
      })

  // Attack 8 (`stepv_once`): 1-row CORE (`core_h = 1`) skips row S=4 (`bs`) and sends forged `post_d = 999` once.
  pure val attackStepvOnceWitness: Witness = {
    val base_w = honestWitness(U7)
    val forged_sw = { ...mkStepWords(ProgA, 104, 4, 1), post_d: 999 }
    val ww: WorkWitness = {
      ...base_w.work,
      wk_rows: List(forged_sw, ZERO_STEP_WORDS),
      pv_seg: { ...base_w.work.pv_seg, post_d: 999 },
      core: {
        ...base_w.work.core,
        core_h: 1,
        pvk_rows: List(forged_sw),
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mStepvOnceAttackTest =
    m_stepv_once::init
      .then(m_stepv_once::assignWitness(attackStepvOnceWitness))
      .expect(and {
        m_stepv_once::acc,
        not(m_stepv_once::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackStepvOnceWitness)),
      })

  // Attack 9 (`stepv_wf_free`): `wf = [0, 1]`, row 1 receives the 8 honest CORE sends, row 0 links `post_d = 999`.
  pure val attackStepvWfFreeWitness: Witness = {
    val base_w = honestWitness(U7)
    val honest_sw = mkStepWords(ProgA, 104, 4, 1)
    val forged_sw = { ...honest_sw, post_d: 999 }
    val ww: WorkWitness = {
      ...base_w.work,
      wk_rows: List(forged_sw, honest_sw),
      wf_rows: List(0, 1),
      whi1: limb2OfInt(5),
      pv_seg: { ...base_w.work.pv_seg, post_d: 999 },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mStepvWfFreeAttackTest =
    m_stepv_wf_free::init
      .then(m_stepv_wf_free::assignWitness(attackStepvWfFreeWitness))
      .expect(and {
        m_stepv_wf_free::acc,
        not(m_stepv_wf_free::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackStepvWfFreeWitness)),
      })

  // Attack 10 (`core_send_row0_only`): CORE row 0 has `post_d = 999` while row 4 has true `post_d = 105`.
  pure val attackCoreSendRow0OnlyWitness: Witness = {
    val base_w = honestWitness(U7)
    val honest_sw = mkStepWords(ProgA, 104, 4, 1)
    val forged_sw = { ...honest_sw, post_d: 999 }
    val ww: WorkWitness = {
      ...base_w.work,
      wk_rows: List(forged_sw, ZERO_STEP_WORDS),
      pv_seg: { ...base_w.work.pv_seg, post_d: 999 },
      core: {
        ...base_w.work.core,
        pvk_rows: repeatStepWords(honest_sw, CORE_ROWS).replaceAt(0, forged_sw),
      },
    }
    { ...base_w, target: TargetRootL, work: ww }
  }

  run mCoreSendRow0OnlyAttackTest =
    m_core_send_row0_only::init
      .then(m_core_send_row0_only::assignWitness(attackCoreSendRow0OnlyWitness))
      .expect(and {
        m_core_send_row0_only::acc,
        not(m_core_send_row0_only::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackCoreSendRow0OnlyWitness)),
      })

  // Attack 11 (`row_unbound`): Row 4 replaced by `vkcPair(K_N3, K_FAKE)`, laundering `SrcFake` into `L`.
  pure val attackRowUnboundWitness: Witness = {
    val base_w = honestWitness(U7)
    val utbl = canonicalUKeys(U7)
    val rogue_vkc = vkcPair(K_N3, K_FAKE)
    val rogue_tbl = utbl.replaceAt(4, rogue_vkc)
    val rg0: NodeRung = {
      parent_k: K_N3,
      child_k: K_FAKE,
      tbl: rogue_tbl,
      sel_idx: 4,
      pv_vkc: rogue_vkc,
    }
    {
      ...base_w,
      target: TargetRootL,
      root: {
        ...base_w.root,
        src: SrcFake,
        fake_claimed_vkc: vkcPair(K_FAKE, K_FAKE),
        rungs: List(rg0),
      },
    }
  }

  run mRowUnboundAttackTest =
    m_row_unbound::init
      .then(m_row_unbound::assignWitness(attackRowUnboundWitness))
      .expect(and {
        m_row_unbound::acc,
        not(m_row_unbound::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackRowUnboundWitness)),
      })

  // Attack 12 (`half_split_off`): `Kind::Node` (`K_N2`) selects `sel_idx = UROWS = 7` (`foreign[0]`, FV key).
  // Accepted at the Node layer under `m_half_split_off` (presenting a foreign row as a node key),
  // AND still refused one layer up by its parent (`N3`) when `parent_checks_vkc` is on.
  pure val attackHalfSplitOffWitness: Witness = {
    val base_w = honestWitness(U7)
    val utbl = canonicalUKeys(U7)
    {
      ...base_w,
      target: TargetNode,
      node: {
        ...base_w.node,
        sel_idx: U7,
        pv_vkc: utbl.nth(U7),
      },
    }
  }

  run mHalfSplitOffAttackTest = {
    val utbl = canonicalUKeys(U7)
    val parent_chain_over_bad_node: Witness = {
      ...honestWitness(U7),
      target: TargetRootL,
      root: {
        ...honestWitness(U7).root,
        src: SrcPlainN1,
        rungs: List(
          { parent_k: K_N2, child_k: K_N1, tbl: utbl, sel_idx: U7, pv_vkc: utbl.nth(U7) },
          { parent_k: K_N3, child_k: K_N2, tbl: utbl, sel_idx: 1, pv_vkc: utbl.nth(1) }
        ),
      },
    }
    m_half_split_off::init
      .then(m_half_split_off::assignWitness(attackHalfSplitOffWitness))
      .expect(and {
        m_half_split_off::acc,
        not(m_half_split_off::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackHalfSplitOffWitness)),
        // Parent N3 still refuses the foreign-claiming N2 proof one layer up (`stmt.rs:845-848`):
        not(evalAccepted(U7, { ...SOUND_CFG, strict_half_split: false }, parent_chain_over_bad_node)),
      })
  }

  // Attack 13 (`key_check_none`): `SrcFake` -> `K_N2` (claiming row 0) -> `K_N3` (claiming row 1) -> `L`.
  // Even though `L` checks `top.actual_vkc == top.claimed_vkc`, dropping internal child-key checks
  // allows `K_N2` and `K_N3` to launder `SrcFake` into a genuine `vkcPair(K_N3, K_N2)` proof!
  pure val attackKeyCheckNoneWitness: Witness = {
    val base_w = honestWitness(U7)
    val utbl = canonicalUKeys(U7)
    {
      ...base_w,
      target: TargetRootL,
      root: {
        ...base_w.root,
        src: SrcFake,
        fake_claimed_vkc: utbl.nth(0),
        rungs: List(
          { parent_k: K_N2, child_k: K_FAKE, tbl: utbl, sel_idx: 0, pv_vkc: utbl.nth(0) },
          { parent_k: K_N3, child_k: K_N2, tbl: utbl, sel_idx: 1, pv_vkc: utbl.nth(1) }
        ),
      },
    }
  }

  run mKeyCheckNoneAttackTest =
    m_key_check_none::init
      .then(m_key_check_none::assignWitness(attackKeyCheckNoneWitness))
      .expect(and {
        m_key_check_none::acc,
        not(m_key_check_none::invRefine),
        not(evalAccepted(U7, SOUND_CFG, attackKeyCheckNoneWitness)),
      })

  // ==================================================================
  // 3. EXHAUSTIVE pure checks over finite attack domains (`PROMPT.md:19`)
  // ==================================================================

  // Q1 Exhaustive Check A: Segment stitching (drop, reorder, duplicate, input splice, program splice,
  // wrong W position, output post_d / h_hi forgery).
  pure val exhaustiveQ1StitchingHolds: bool = {
    val base_ww = honestWorkWitnessFor(U7, ProgA)
    val seg_choices = Set((0, 2), (2, 4), (4, 5))
    val w_choices = Set((0, 1), (2, 2), (4, 1), (8, 1))
    Set(false, true).forall(slot1_on =>
      Set(ProgA, ProgB).forall(p0 =>
        Set(ProgA, ProgB).forall(p1 =>
          Set(ProgA, ProgB).forall(pw =>
            Set(In1, In2).forall(inp0 =>
              Set(In1, In2).forall(inp1 =>
                Set(In1, In2).forall(inpw =>
                  seg_choices.forall(s0 =>
                    seg_choices.forall(s1 =>
                      w_choices.forall(wv =>
                        Set(false, true).forall(forge_post => {
                          val c0 = mkN3SegProof(p0, inp0, s0._1, s0._2, true)
                          val c1 = mkN3SegProof(p1, inp1, s1._1, s1._2, true)
                          val wh = wv._1
                          val wsteps = wv._2
                          val wpre = stepN(pw, initStateOf(inpw), wh)
                          val sw = mkStepWords(pw, wpre, wh, wsteps)
                          val out_post = if (forge_post) 999 else sw.post_d
                          val whi_v = wh + wsteps
                          val cand: WorkWitness = {
                            ...base_ww,
                            c0: c0,
                            c1: c1,
                            slot1_on: slot1_on,
                            wk_rows: List(sw, ZERO_STEP_WORDS),
                            whi0: limb2OfInt(whi_v),
                            wcry0: { lo: (wh % LIMB_BASE + wsteps) / LIMB_BASE, hi: 0 },
                            pv_seg: {
                              tag: TagSeg,
                              prog_d: c0.env.prog_d,
                              pre_d: c0.env.pre_d,
                              post_d: out_post,
                              h_lo: c0.env.h_lo,
                              h_hi: limb2OfInt(whi_v),
                              keys_d_ok: true,
                            },
                            core: {
                              core_h: CORE_ROWS,
                              pvk_rows: repeatStepWords(sw, CORE_ROWS),
                              true_prog: pw,
                              true_pre: wpre,
                              true_steps: wsteps,
                            },
                          }
                          evalWorkLayer(U7, SOUND_CFG, cand) implies refinesWorkLayer(U7, cand)
                        })
                      )
                    )
                  )
                )
              )
            )
          )
        )
      )
    )
  }

  run exhaustiveQ1StitchingTest =
    sound_m::init.expect(exhaustiveQ1StitchingHolds)

  // Q1 Exhaustive Check B: Limb carry equations WITH 4-bit range check (`SOUND_CFG`) vs WITHOUT (`whi_no_range`).
  pure val exhaustiveQ1LimbDomainHolds: bool = {
    val wh_set = Set(0, 4, 15, 240, 254, 255)
    val steps_set = Set(1, 2, 4)
    val whi_lo_set = Set(0, 1, 5, 6, 15, 16, 246)
    val whi_hi_set = Set(0, 1, 15, 16)
    val wcry_lo_set = Set(0, 1, 2, 16)
    val wcry_hi_set = Set(0, 1)

    wh_set.forall(wh =>
      steps_set.forall(st =>
        whi_lo_set.forall(wlo =>
          whi_hi_set.forall(whi_h =>
            wcry_lo_set.forall(clo =>
              wcry_hi_set.forall(chi => {
                val hp = limb2OfInt(wh)
                val out_l: Limb2 = { lo: wlo, hi: whi_h }
                val cry_l: Limb2 = { lo: clo, hi: chi }
                evalLimbAdd(SOUND_CFG, hp, st, out_l, cry_l) implies and {
                  wh + st <= MAX_H,
                  isCanonLimb2(out_l),
                  limb2Val(out_l) == wh + st,
                }
              })
            )
          )
        )
      )
    )
  }

  run exhaustiveQ1LimbDomainTest =
    sound_m::init.expect(exhaustiveQ1LimbDomainHolds)

  // Q2 Exhaustive Check: All `STEP_PV_BUS` tampers (one row forged, rows disagreeing, <8 rows sending,
  // `wf` not on row 0, a second receive across 4 StmtAir rows and 16 CoreAir rows).
  pure val exhaustiveQ2StepPvBusHolds: bool = {
    val base_ww = honestWorkWitnessFor(U7, ProgA)
    val h_sw = mkStepWords(ProgA, 104, 4, 1)
    val f_sw = { ...h_sw, post_d: 999 }
    val msg_set = Set(h_sw, f_sw)
    val core_heights = Set(1, 4, 8, 16)
    val wf2_pats = Set(List(0, 0), List(1, 0), List(0, 1), List(1, 1))
    val wf4_pats = Set(List(1, 0, 0, 0), List(0, 1, 0, 0), List(1, 0, 1, 0), List(1, 1, 0, 0))

    val h2_ok = core_heights.forall(ch =>
      msg_set.forall(m0 =>
        msg_set.forall(m3 =>
          msg_set.forall(m4 =>
            wf2_pats.forall(wfp =>
              msg_set.forall(wk0 =>
                Set(ZERO_STEP_WORDS, h_sw, f_sw).forall(wk1 => {
                  val base_rows = repeatStepWords(h_sw, ch)
                  val r0 = base_rows.replaceAt(0, m0)
                  val r3 = if (ch > 3) r0.replaceAt(3, m3) else r0
                  val r4 = if (ch > 4) r3.replaceAt(4, m4) else r3
                  val whi1_c = if (wk1 == ZERO_STEP_WORDS) { lo: 0, hi: 0 } else limb2OfInt(5)
                  val cand: WorkWitness = {
                    ...base_ww,
                    stmt_h: 2,
                    wk_rows: List(wk0, wk1),
                    wf_rows: wfp,
                    whi1: whi1_c,
                    pv_seg: { ...base_ww.pv_seg, post_d: wk0.post_d },
                    core: { ...base_ww.core, core_h: ch, pvk_rows: r4 },
                  }
                  evalWorkLayer(U7, SOUND_CFG, cand) implies refinesWorkLayer(U7, cand)
                })
              )
            )
          )
        )
      )
    )

    val h4_ok = Set(8, 16).forall(ch =>
      wf4_pats.forall(wfp =>
        msg_set.forall(wk0 =>
          msg_set.forall(wk2 => {
            val c_rows =
              if (ch == 16) repeatStepWords(wk0, 8).concat(repeatStepWords(wk2, 8))
              else repeatStepWords(h_sw, 8)
            val cand: WorkWitness = {
              ...base_ww,
              stmt_h: 4,
              wk_rows: List(wk0, ZERO_STEP_WORDS, wk2, ZERO_STEP_WORDS),
              wf_rows: wfp,
              pv_seg: { ...base_ww.pv_seg, post_d: wk0.post_d },
              core: { ...base_ww.core, core_h: ch, pvk_rows: c_rows },
            }
            evalWorkLayer(U7, SOUND_CFG, cand) implies refinesWorkLayer(U7, cand)
          })
        )
      )
    )

    h2_ok and h4_ok
  }

  run exhaustiveQ2StepPvBusTest =
    sound_m::init.expect(exhaustiveQ2StepPvBusHolds)

  // Q3 Exhaustive Check A: Every one of the 14 `UKeys` rows (7 node + 7 foreign/padded) is bound into `keys_d`,
  // half split strictly separates Node (`0..6`) from Foreign/Work (`7..13`), and any wrong/zero/foreign key
  // commitment is refused at its parent.
  pure val exhaustiveQ3KeyRowsAndHalfSplitHolds: bool = {
    val utbl = canonicalUKeys(U7)
    val base_w = honestWitness(U7)
    val alt_vkcs = Set(vkcPair(K_N3, K_FAKE), vkcPair(K_NW, K_N3), VKC_ZERO)

    // (a) No row of the 14 rows can be altered without changing `keys_d` under SOUND_CFG:
    val every_row_bound = 0.to(2 * U7 - 1).forall(idx =>
      alt_vkcs.forall(av =>
        (av != utbl.nth(idx)) implies not(sameKeysD(U7, SOUND_CFG, utbl.replaceAt(idx, av), utbl))
      )
    )

    // (b) Standalone Node accepts ONLY `0 <= sel < UROWS` and refuses foreign/out-of-table vkcs:
    val node_half_enforced = 0.to(2 * U7 - 1).forall(s_idx =>
      Set(utbl.nth(s_idx), vkcPair(K_FAKE, K_FAKE)).forall(claimed => {
        val nw: NodeWitness = {
          ...base_w.node,
          sel_idx: s_idx,
          pv_vkc: claimed,
        }
        evalNodeLayer(U7, SOUND_CFG, nw) implies and {
          s_idx < U7,
          claimed == utbl.nth(s_idx),
          refinesNodeLayer(U7, nw),
        }
      })
    )

    // (c) Any wrong key row claimed by MERGE-WORK (including `foreign[0]` FV row 7 or zero rows 9..13)
    // is refused one layer up by `NW` (and across the ladder to `L`):
    val work_wrong_row_refused_by_parent = 0.to(2 * U7 - 1).forall(s_idx => {
      val ww: WorkWitness = {
        ...base_w.work,
        sel_idx: s_idx,
        pv_vkc: utbl.nth(s_idx),
      }
      val w_root: Witness = { ...base_w, target: TargetRootL, work: ww }
      evalRootLayer(U7, SOUND_CFG, w_root) implies (s_idx == U7 + 1)
    })

    every_row_bound and node_half_enforced and work_wrong_row_refused_by_parent
  }

  run exhaustiveQ3KeyRowsAndHalfSplitTest =
    sound_m::init.expect(exhaustiveQ3KeyRowsAndHalfSplitHolds)

  // Q3 Exhaustive Check B: Uniqueness of the 3-rung ladder `MERGE-WORK -> NW -> NW2 -> N3` into `N3` / `L`,
  // and complete rejection of `NW` or `NW2` presented directly to `L`, `SrcFv`, and `SrcFake`.
  pure val exhaustiveQ3UniqueLadderHolds: bool = {
    val utbl = canonicalUKeys(U7)
    val base_w = honestWitness(U7)
    val node_keys = Set(K_N2, K_N3, K_NW, K_NW2)
    val all_keys = Set(K_N1, K_N2, K_FV, K_N3, K_WORK, K_NW, K_NW2, K_FAKE)
    val node_sels = 0.to(U7 - 1)

    // Helper: build a candidate NodeRung on `canonicalUKeys(7)` (since `L` forces `sameKeysD`,
    // which under `SOUND_CFG` forces `tbl == utbl` as proven in Q3 Check A).
    // 0-rung: MERGE-WORK, FV, PlainN1, Fake presented directly to L -> ALL refused!
    val len0_refused = Set(SrcWork, SrcPlainN1, SrcFv, SrcFake).forall(src =>
      not(evalRootLayer(U7, SOUND_CFG, {
        ...base_w,
        target: TargetRootL,
        root: { ...base_w.root, src: src, rungs: List() },
      }))
    )

    // 1-rung: exhaustive over ALL `src`, `parent_k`, `child_k`, `sel_idx` (4 * 4 * 8 * 7 = 896 cases) -> ALL refused!
    val len1_refused = Set(SrcWork, SrcPlainN1, SrcFv, SrcFake).forall(src =>
      node_keys.forall(pk0 =>
        all_keys.forall(ck0 =>
          node_sels.forall(s0 => {
            val rg0: NodeRung = { parent_k: pk0, child_k: ck0, tbl: utbl, sel_idx: s0, pv_vkc: utbl.nth(s0) }
            not(evalRootLayer(U7, SOUND_CFG, {
              ...base_w,
              target: TargetRootL,
              root: { ...base_w.root, src: src, rungs: List(rg0) },
            }))
          })
        )
      )
    )

    // 2-rung: exhaustive over `(pk, sel)` on `SrcWork`, `SrcFv`, `SrcFake`:
    // (Note `VerifierSubCircuit` forces `ck0 == cur.actual_vkc.pre` and `ck1 == pk0` for any accepting run;
    // we also quantify `ck0 == base_pre` and `ck1 == pk0` plus mismatched `ck` check).
    val len2_work_fv_fake_refused = Set(SrcWork, SrcFv, SrcFake).forall(src => {
      val b_pre = match src {
        | SrcWork => K_WORK
        | SrcFv => K_FV
        | _ => K_FAKE
      }
      node_keys.forall(pk0 =>
        node_sels.forall(s0 =>
          node_keys.forall(pk1 =>
            node_sels.forall(s1 => {
              val rg0: NodeRung = { parent_k: pk0, child_k: b_pre, tbl: utbl, sel_idx: s0, pv_vkc: utbl.nth(s0) }
              val rg1: NodeRung = { parent_k: pk1, child_k: pk0, tbl: utbl, sel_idx: s1, pv_vkc: utbl.nth(s1) }
              not(evalRootLayer(U7, SOUND_CFG, {
                ...base_w,
                target: TargetRootL,
                root: { ...base_w.root, src: src, rungs: List(rg0, rg1) },
              }))
            })
          )
        )
      )
    })

    // 3-rung from `SrcWork` (and `SrcFake`, `SrcFv`): exhaustive over all `(pk0, s0, pk1, s1, pk2, s2)`
    // where each rung selects one of the 7 node rows (plus checking `pk` mismatch against `utbl[s].pre`).
    // Proves `NW (row 4) -> NW2 (row 5) -> N3 (row 6)` is the UNIQUE 3-rung route from `MERGE-WORK` into `N3` / `L`!
    val len3_unique_work_route = node_sels.forall(s0 =>
      node_sels.forall(s1 =>
        node_sels.forall(s2 =>
          node_keys.forall(pk2 => {
            val pk0 = utbl.nth(s0).pre
            val pk1 = utbl.nth(s1).pre
            val rg0: NodeRung = { parent_k: pk0, child_k: K_WORK, tbl: utbl, sel_idx: s0, pv_vkc: utbl.nth(s0) }
            val rg1: NodeRung = { parent_k: pk1, child_k: pk0, tbl: utbl, sel_idx: s1, pv_vkc: utbl.nth(s1) }
            val rg2: NodeRung = { parent_k: pk2, child_k: pk1, tbl: utbl, sel_idx: s2, pv_vkc: utbl.nth(s2) }
            val cand_w: Witness = {
              ...base_w,
              target: TargetRootL,
              root: { ...base_w.root, src: SrcWork, rungs: List(rg0, rg1, rg2) },
            }
            val cand_fake: Witness = {
              ...base_w,
              target: TargetRootL,
              root: {
                ...base_w.root,
                src: SrcFake,
                fake_claimed_vkc: vkcPair(K_FAKE, K_FAKE),
                rungs: List({ ...rg0, child_k: K_FAKE }, rg1, rg2),
              },
            }
            and {
              evalRootLayer(U7, SOUND_CFG, cand_w) iff (s0 == 4 and s1 == 5 and s2 == 6 and pk2 == K_N3),
              not(evalRootLayer(U7, SOUND_CFG, cand_fake)),
            }
          })
        )
      )
    )

    len0_refused and len1_refused and len2_work_fv_fake_refused and len3_unique_work_route
  }

  run exhaustiveQ3UniqueLadderTest =
    sound_m::init.expect(exhaustiveQ3UniqueLadderHolds)

  // Q4 Exhaustive Check: One universal `keys_d` across both programs (`ProgA`, `ProgB`)
  // and both routes (`RoutePlain`, `RouteWork`), with accepted end-to-end `L` witnesses for all four!
  pure val exhaustiveQ4UniversalKeysHolds: bool = {
    val progs = Set(ProgA, ProgB)
    val routes = Set(RoutePlain, RouteWork)
    val base_w = honestWitness(U7)
    val all_tables_equal = progs.forall(p1 =>
      routes.forall(r1 =>
        progs.forall(p2 =>
          routes.forall(r2 =>
            ukeysForProgramAndRoute(U7, p1, r1) == ukeysForProgramAndRoute(U7, p2, r2)
          )
        )
      )
    )
    val progA_work_ok = evalRootLayer(U7, SOUND_CFG, base_w) and refinesRootLayer(U7, base_w)
    val progB_work_w = { ...base_w, work: honestWorkWitnessFor(U7, ProgB) }
    val progB_work_ok = evalRootLayer(U7, SOUND_CFG, progB_work_w) and refinesRootLayer(U7, progB_work_w)
    val plain_w = {
      ...base_w,
      root: { ...base_w.root, src: SrcPlainN1, rungs: honestPlainLadderRungs(U7) },
    }
    val plain_ok = evalRootLayer(U7, SOUND_CFG, plain_w) and refinesRootLayer(U7, plain_w)
    all_tables_equal and progA_work_ok and progB_work_ok and plain_ok
  }

  run exhaustiveQ4UniversalKeysTest =
    sound_m::init.expect(exhaustiveQ4UniversalKeysHolds)
}
