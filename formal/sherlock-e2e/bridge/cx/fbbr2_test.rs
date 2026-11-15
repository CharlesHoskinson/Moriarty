
/// F-B-BR-2 counterexample (slice B formal check, not committed to openvm): the spec text of
/// CLB-005, ANC-002 and ANC-017 gives the lock operand 6 inputs,
/// `x_j = sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, j)`. The code (FP-3, `LOCK_LEN = 7`)
/// absorbs the chain link `pack(C_(j+1))` before `j`. A chunk whose `a_in` and `a_out` are the
/// 6-operand values is refused, and the honest 7-operand chunk is accepted, so the 6-operand text
/// refuses every honest FP-3 chunk.
#[test]
fn fbbr2_six_operand_lock_text_vs_fp3_code() {
    let x6 = |c: &Call, k: u64, j: u64| {
        let rt = &c.words[iface::RUN_TAG_WORD..iface::RUN_TAG_WORD + 8];
        iface::sponge_u(
            iface::U_LOCK,
            &[c.ds[0], sherlock_lfr::fr::pack(rt), Fr::from_u64(k), c.ds[1], c.ds[2], Fr::from_u64(j)],
        )
    };
    let h = honest("FBBR2-honest-2of3", Case::Chunk { j: 2, k: 3 }, 31, 300);
    let vh = verdict(&h);
    println!("FBBR2 honest 7-operand lock: {}", vh.show());
    assert!(vh.accepted(), "control: the honest FP-3 chunk is accepted");
    let (a_in7, a_out7) = locks_of(&h, 3, 2);
    let (a_in6, a_out6) = (x6(&h, 3, 1), x6(&h, 3, 2));
    assert_ne!(a_in6, a_in7, "the link changes x_(j-1)");
    assert_ne!(a_out6, a_out7, "the link changes x_j");
    let c6 = build("FBBR2-six-operand-lock-2of3", Case::Chunk { j: 2, k: 3 }, 31, 300, 7, (Some(a_in6), Some(a_out6)), |_| {});
    let v6 = verdict(&c6);
    println!("FBBR2 6-operand lock (spec text): {}", v6.show());
    assert!(!v6.accepted(), "the 6-operand lock of the spec text is refused by the FP-3 code");
}
