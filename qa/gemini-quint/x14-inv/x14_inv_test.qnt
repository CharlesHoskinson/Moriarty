// -*- mode: Bluespec; -*-
// Deterministic unit tests for the X14 G1 addition-chain INV condition row.
// Run with:
//   quint test qa/gemini-quint/x14-inv/x14_inv_test.qnt --main=x14_inv_test

module x14_inv_test {
  import x14_inv.* from "./x14_inv"

  // 1. Completeness: honest distinct non-opposite points (P1 != +/-P2) are accepted
  //    and satisfy the refinement invariant invRow.
  run testHonestDistinctAccepts =
    init.then(all {
      assert(chainAcceptedWith(st, CFG_SOUND)),
      assert(invRowHolds(st, CFG_SOUND)),
      st' = st
    })

  // 2. Completeness on padding rows: a chain with row 0 valid (is_valid = 1)
  //    and row 1 padded (is_valid = 0, even with x1 == x2 on the padded row)
  //    is accepted and satisfies invRow.
  run testHonestPaddedAccepts =
    init.then(all { st' = TEST_PADDED_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_SOUND)),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  // 3. Soundness on P1 = P2 (doubling): any forged lambda and any w on x1 == x2
  //    is rejected by the sound INV row and bus o+10.
  run testSamePointRejectedBySound =
    init.then(all { st' = TEST_SAME_PT_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  // 4. Soundness on P1 = -P2 (x1 == x2, y1 != y2): rejected by both L_add and INV.
  run testOppositePointRejectedBySound =
    init.then(all { st' = TEST_OPP_PT_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  // 5. Mutant m_no_inv_row: without o+10 send/INV row, P1 = P2 with forged lambda = 7
  //    is accepted and violates invRow.
  run testMutantNoInvRowViolatesInvRow =
    init.then(all { st' = TEST_SAME_PT_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_NO_INV_ROW)),
          assert(not(invRowHolds(st, CFG_NO_INV_ROW))),
          st' = st
        })

  // 6. Mutant m_no_recv: when INV does not receive on o+10, prover sets INV to
  //    unrelated (x1=1, x2=2, w=1) while forging P1 = P2 in L_add; accepted, violates invRow.
  run testMutantNoRecvViolatesInvRow =
    init.then(all { st' = TEST_NO_RECV_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_NO_RECV)),
          assert(not(invRowHolds(st, CFG_NO_RECV))),
          st' = st
        })

  // 7. Mutant m_wrong_den: when INV checks w*(x2 + x1) = 1, P1 = P2 with x1 = x2 = 3
  //    has x2 + x1 = 6 invertible mod 13 (w = 11), so forged lambda = 7 is accepted
  //    and violates invRow.
  run testMutantWrongDenViolatesInvRow =
    init.then(all { st' = TEST_WRONG_DEN_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_WRONG_DEN)),
          assert(not(invRowHolds(st, CFG_WRONG_DEN))),
          st' = st
        })

  // 8. Mutant m_no_id: dropping id from o+10 allows swapping INV rows 0 and 1
  //    while the sound specification rejects the swapped rows.
  run testMutantNoIdAcceptsSwappedInvRows =
    init.then(all { st' = TEST_SWAPPED_ID_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(chainAcceptedWith(st, CFG_NO_ID)),
          st' = st
        })

  // 9. Mutant m_inv_valid_free: when INV receives with multiplicity 1 instead of
  //    is_valid, an honest padded trace (row 1 is_valid = 0) fails bus balance.
  run testMutantInvValidFreeRejectsPaddedChain =
    init.then(all { st' = TEST_INV_VALID_FREE_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_SOUND)),
          assert(not(chainAcceptedWith(st, CFG_INV_VALID_FREE))),
          st' = st
        })

  // 10. Soundness against dropping or duplicating a bus message: any single dropped
  //     or duplicated bus message breaks multiset balance and is rejected.
  run testDropOrDupMessageRejectedBySound =
    init.then(all { st' = dropOrDupState(HONEST_DEFAULT_STATE, CFG_SOUND, 5, -1) })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          st' = dropOrDupState(HONEST_DEFAULT_STATE, CFG_SOUND, 5, 1)
        })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          st' = st
        })
}
