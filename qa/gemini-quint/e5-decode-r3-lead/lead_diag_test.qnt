// Lead diagnostic (2026-11-13): per-case SOUND and COMPLETE over CORPUS, and the three checks of catch_mut_no_m_range_test.
module diag_test {
  import decode.* from "./decode"
  run d_sound_test = {
    val r = 0.to(CORPUS.length() - 1).map(i => (i, is_sound_on(CORPUS.nth(i).pw, CORPUS.nth(i).wit), is_complete_on(CORPUS.nth(i).pw)))
    assert(q::debug("SC", r) == r)
  }
  run d_mrange_test = {
    val a = ref_decode(PW_BAD_NO_M_RANGE).ok
    val b = constraints_ok(PW_BAD_NO_M_RANGE, WIT_BAD_NO_M_RANGE)
    val c = mut_no_m_range(PW_BAD_NO_M_RANGE, WIT_BAD_NO_M_RANGE)
    assert(q::debug("MR ref_ok, constraints_ok, mut_ok", (a, b, c)) == (a, b, c))
  }
  // Lead (T13 repair): is the round 3 free_mult counterexample refused only by the m-range check?
  run d_freemult_test = {
    val a = ref_decode(PW_BAD_FREE_MULT).ok
    val b = constraints_ok(PW_BAD_FREE_MULT, WIT_BAD_FREE_MULT)
    val c = mut_free_mult(PW_BAD_FREE_MULT, WIT_BAD_FREE_MULT)
    val d = constraints_with_flags({ ...DEFAULT_FLAGS, free_mult: true, check_m_range: false }, PW_BAD_FREE_MULT, WIT_BAD_FREE_MULT)
    assert(q::debug("FM ref_ok, constraints_ok, mut_free_mult, free_mult_and_no_m_range", (a, b, c, d)) == (a, b, c, d))
  }
}
