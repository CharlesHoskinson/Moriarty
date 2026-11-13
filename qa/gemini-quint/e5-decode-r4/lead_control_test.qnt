// Lead controls (T13, 2026-11-13) for the round 4 exhaustive check of mut_free_mult.
// The check `not(mut_free_mult(PW_BAD_FREE_MULT, free_mult_attack_wit(i6, i7)))` for all 64 pairs
// means something only if the generator can build an accepted witness once the m-range check is off.
module control_test {
  import decode.* from "./decode"
  pure val NO_MR: Flags = { ...DEFAULT_FLAGS, free_mult: true, check_m_range: false }
  run c_attack_domain_test = {
    val acc = 0.to(B_CAP - 1).fold(Set(), (s, i6) =>
      0.to(B_CAP - 1).fold(s, (s2, i7) =>
        if (constraints_with_flags(NO_MR, PW_BAD_FREE_MULT, free_mult_attack_wit(i6, i7))) s2.union(Set((i6, i7))) else s2))
    assert(q::debug("pairs accepted with free_mult and no m-range", acc) == acc)
  }
  run c_attack_66_test = {
    val a = constraints_with_flags(NO_MR, PW_BAD_FREE_MULT, free_mult_attack_wit(6, 6))
    val b = mut_free_mult(PW_BAD_FREE_MULT, free_mult_attack_wit(6, 6))
    assert(q::debug("(6, 6): free_mult and no m-range, free_mult only", (a, b)) == (a, b))
  }
}
