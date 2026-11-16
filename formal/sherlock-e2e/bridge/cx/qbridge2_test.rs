
/// Q-bridge-2 counterexample (Formal-Quint, not committed to openvm): read literally, ANC-017 and
/// CLB-005 give the lock that chunk j READS the operand `(pack(C_(j+1)), j)`, the same as the lock
/// it WRITES. The FP-3 code reads `x_(j-1) = lock_x(.., C_j, j-1)` and writes `x_j`. A chunk whose
/// `a_in` is the literal-reading operand is refused; the honest chunk is accepted.
#[test]
fn qbridge2_literal_read_lock_operand_is_refused() {
    let h = honest("QB2-honest-2of3", Case::Chunk { j: 2, k: 3 }, 31, 300);
    assert!(verdict(&h).accepted(), "control: honest chunk accepted");
    let (a_in, a_out) = locks_of(&h, 3, 2);
    let rt = &h.words[iface::RUN_TAG_WORD..iface::RUN_TAG_WORD + 8];
    let (_li, lo) = fp3::links_of(&h.eff);
    let a_in_literal = iface::lock_x(h.ds, rt, 3, lo, 2);
    assert_ne!(a_in_literal, a_in, "the literal read operand differs from x_(j-1)");
    assert_eq!(a_in_literal, a_out, "the literal read operand is the chunk's own write lock x_j");
    let c = build("QB2-literal-read-lock-2of3", Case::Chunk { j: 2, k: 3 }, 31, 300, 7, (Some(a_in_literal), Some(a_out)), |_| {});
    let v = verdict(&c);
    println!("QB2 honest: {}", verdict(&h).show());
    println!("QB2 literal read lock (pack(C_(j+1)), j): {}", v.show());
    assert!(!v.accepted(), "the literal-reading read lock is refused by the FP-3 code");
}
