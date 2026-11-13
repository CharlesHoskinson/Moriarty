// -*- mode: Bluespec; -*-
// Deterministic unit tests for the X14 G1 addition-chain INV condition row.
// Run with:
//   quint test qa/gemini-quint/x14-inv/inv_test.qnt --main=inv_test

module inv_test {
  import inv.* from "./inv"

  run testHonestDistinctAccepts =
    init.then(all {
      assert(chainAcceptedWith(st, CFG_SOUND)),
      assert(invRowHolds(st, CFG_SOUND)),
      st' = st
    })

  run testHonestPaddedAccepts =
    init.then(all { st' = TEST_PADDED_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_SOUND)),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  run testSamePointRejectedBySound =
    init.then(all { st' = TEST_SAME_PT_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  run testOppositePointRejectedBySound =
    init.then(all { st' = TEST_OPP_PT_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  run testMutantNoInvRowViolatesInvRow =
    init.then(all { st' = TEST_SAME_PT_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_NO_INV_ROW)),
          assert(not(invRowHolds(st, CFG_NO_INV_ROW))),
          st' = st
        })

  run testMutantNoRecvViolatesInvRow =
    init.then(all { st' = TEST_NO_RECV_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_NO_RECV)),
          assert(not(invRowHolds(st, CFG_NO_RECV))),
          st' = st
        })

  run testMutantWrongDenViolatesInvRow =
    init.then(all { st' = TEST_WRONG_DEN_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_WRONG_DEN)),
          assert(not(invRowHolds(st, CFG_WRONG_DEN))),
          st' = st
        })

  run testMutantNoIdAcceptsSwappedInvRows =
    init.then(all { st' = TEST_SWAPPED_ID_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(chainAcceptedWith(st, CFG_NO_ID)),
          st' = st
        })

  run testMutantInvValidFreeRejectsPaddedChain =
    init.then(all { st' = TEST_INV_VALID_FREE_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_SOUND)),
          assert(not(chainAcceptedWith(st, CFG_INV_VALID_FREE))),
          st' = st
        })

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
