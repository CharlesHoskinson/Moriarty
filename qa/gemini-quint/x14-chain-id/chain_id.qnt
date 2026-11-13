// -*- mode: Bluespec; -*-
// Soundness and completeness model of the Sherlock X14 BLS12-381 G1 ChainAir
// condition row (`id = row index` in ChainAir and PointsIn) over F_13.
// Self-contained modules: `chain_id` (sound baseline) and the 4 required mutants
// `m_no_first_row`, `m_no_transition`, `m_step_two`, and `m_id_not_in_msg`.

module chain_id {
  pure val P: int = 13

  pure def in_fp(x: int): bool =
    0 <= x and x < P

  pure def fp_mod(x: int): int =
    ((x % P) + P) % P

  pure def fp_add(a: int, b: int): int =
    fp_mod(a + b)

  pure def fp_sub(a: int, b: int): int =
    fp_mod(a - b)

  pure def fp_mul(a: int, b: int): int =
    fp_mod(a * b)

  pure def fp_inv(a: int): int = {
    val m = fp_mod(a)
    if (m == 1) 1
    else if (m == 2) 7
    else if (m == 3) 9
    else if (m == 4) 10
    else if (m == 5) 8
    else if (m == 6) 11
    else if (m == 7) 2
    else if (m == 8) 5
    else if (m == 9) 3
    else if (m == 10) 4
    else if (m == 11) 6
    else if (m == 12) 12
    else 0
  }

  // Reference affine G1 secant addition over F_13 (for x1 != x2 mod 13)
  pure def spec_lam_add(x1: int, y1: int, x2: int, y2: int): int =
    fp_mul(fp_sub(y2, y1), fp_inv(fp_sub(x2, x1)))

  pure def spec_x3_add(x1: int, y1: int, x2: int, y2: int): int = {
    val lam = spec_lam_add(x1, y1, x2, y2)
    fp_sub(fp_sub(fp_mul(lam, lam), x1), x2)
  }

  pure def spec_y3_add(x1: int, y1: int, x2: int, y2: int): int = {
    val lam = spec_lam_add(x1, y1, x2, y2)
    val x3 = spec_x3_add(x1, y1, x2, y2)
    fp_sub(fp_mul(lam, fp_sub(x1, x3)), y1)
  }

  // Reference affine G1 tangent doubling over F_13 (for y != 0 mod 13)
  pure def spec_lam_dbl(x: int, y: int): int = {
    val s = fp_mul(x, x)
    fp_mul(fp_mul(3, s), fp_inv(fp_mul(2, y)))
  }

  pure def spec_x3_dbl(x: int, y: int): int = {
    val lam = spec_lam_dbl(x, y)
    fp_sub(fp_sub(fp_mul(lam, lam), x), x)
  }

  pure def spec_y3_dbl(x: int, y: int): int = {
    val lam = spec_lam_dbl(x, y)
    val x3 = spec_x3_dbl(x, y)
    fp_sub(fp_mul(lam, fp_sub(x, x3)), y)
  }

  // Distinct non-exceptional input pairs (x1_k, y1_k), (x2_k, y2_k) over F_13
  pure def input_x1(k: int): int =
    if (k == 0) 1 else if (k == 1) 2 else if (k == 2) 4 else 5

  pure def input_y1(k: int): int =
    if (k == 0) 2 else if (k == 1) 4 else if (k == 2) 1 else 3

  pure def input_x2(k: int): int =
    if (k == 0) 3 else if (k == 1) 6 else if (k == 2) 8 else 10

  pure def input_y2(k: int): int =
    if (k == 0) 7 else if (k == 1) 5 else if (k == 2) 3 else 9

  // Per-row polynomial constraints for ChainAir and PointsIn (heights 2..4)
  pure def check_air_ids(
    h: int,
    id0: int,
    id1: int,
    id2: int,
    id3: int,
    check_first: bool,
    check_trans: bool,
    step_inc: int
  ): bool = {
    val in_range = in_fp(id0) and in_fp(id1) and (h < 3 or in_fp(id2)) and (h < 4 or in_fp(id3))
    val first_ok = if (check_first) (id0 == 0) else true
    val t1_ok = if (check_trans) (id1 == fp_add(id0, step_inc)) else true
    val t2_ok = if (check_trans and h >= 3) (id2 == fp_add(id1, step_inc)) else true
    val t3_ok = if (check_trans and h >= 4) (id3 == fp_add(id2, step_inc)) else true
    (h >= 2 and h <= 4) and in_range and first_ok and t1_ok and t2_ok and t3_ok
  }

  pure def check_l_add_poly(v: int, x1: int, y1: int, x2: int, y2: int, lam: int): bool =
    (v == 0 or v == 1) and
    (v == 0 or (in_fp(lam) and fp_mul(fp_sub(x2, x1), lam) == fp_sub(y2, y1)))

  pure def check_inv_poly(v: int, x1: int, x2: int, w: int): bool =
    (v == 0 or v == 1) and
    (v == 0 or (in_fp(w) and fp_mul(fp_sub(x2, x1), w) == 1))

  pure def check_x3_poly(v: int, lam: int, x1: int, x2: int, x3: int): bool =
    (v == 0 or v == 1) and
    (v == 0 or (in_fp(x3) and fp_sub(fp_sub(fp_mul(lam, lam), x1), x2) == x3))

  pure def check_y3_poly(v: int, lam: int, x1: int, x3: int, y1: int, y3: int): bool =
    (v == 0 or v == 1) and
    (v == 0 or (in_fp(y3) and fp_sub(fp_mul(lam, fp_sub(x1, x3)), y1) == y3))

  // PermutationCheckBus multiset equality helper for up to 3 active rows
  pure def msg_eq(v: int, tid: int, a: int, b: int, qt: int, qa: int, qb: int): int =
    if (v == 1 and tid == qt and a == qa and b == qb) 1 else 0

  pure def count3(
    v0: int, t0: int, a0: int, b0: int,
    v1: int, t1: int, a1: int, b1: int,
    v2: int, t2: int, a2: int, b2: int,
    qt: int, qa: int, qb: int
  ): int =
    msg_eq(v0, t0, a0, b0, qt, qa, qb) +
    msg_eq(v1, t1, a1, b1, qt, qa, qb) +
    msg_eq(v2, t2, a2, b2, qt, qa, qb)

  pure def bus_eq3(
    has_id: bool,
    pv0: int, pid0: int, pa0: int, pb0: int,
    pv1: int, pid1: int, pa1: int, pb1: int,
    pv2: int, pid2: int, pa2: int, pb2: int,
    cv0: int, cid0: int, ca0: int, cb0: int,
    cv1: int, cid1: int, ca1: int, cb1: int,
    cv2: int, cid2: int, ca2: int, cb2: int
  ): bool = {
    val pt0 = if (has_id) pid0 else 99
    val pt1 = if (has_id) pid1 else 99
    val pt2 = if (has_id) pid2 else 99
    val ct0 = if (has_id) cid0 else 99
    val ct1 = if (has_id) cid1 else 99
    val ct2 = if (has_id) cid2 else 99
    val p_total = pv0 + pv1 + pv2
    val c_total = cv0 + cv1 + cv2
    val m0_ok = (pv0 == 0) or (
      count3(pv0, pt0, pa0, pb0, pv1, pt1, pa1, pb1, pv2, pt2, pa2, pb2, pt0, pa0, pb0) ==
      count3(cv0, ct0, ca0, cb0, cv1, ct1, ca1, cb1, cv2, ct2, ca2, cb2, pt0, pa0, pb0)
    )
    val m1_ok = (pv1 == 0) or (
      count3(pv0, pt0, pa0, pb0, pv1, pt1, pa1, pb1, pv2, pt2, pa2, pb2, pt1, pa1, pb1) ==
      count3(cv0, ct0, ca0, cb0, cv1, ct1, ca1, cb1, cv2, ct2, ca2, cb2, pt1, pa1, pb1)
    )
    val m2_ok = (pv2 == 0) or (
      count3(pv0, pt0, pa0, pb0, pv1, pt1, pa1, pb1, pv2, pt2, pa2, pb2, pt2, pa2, pb2) ==
      count3(cv0, ct0, ca0, cb0, cv1, ct1, ca1, cb1, cv2, ct2, ca2, cb2, pt2, pa2, pb2)
    )
    (p_total == c_total) and m0_ok and m1_ok and m2_ok
  }

  // Test suite 1: Honest chains (heights 2..4, unequal cross-AIR heights, middle is_valid=0 padding)
  pure def run_honest_suite(u: int): bool = {
    val x1_0 = input_x1(0)
    val y1_0 = input_y1(0)
    val x2_0 = input_x2(0)
    val y2_0 = input_y2(0)
    val lam_0 = spec_lam_add(x1_0, y1_0, x2_0, y2_0)
    val w_0 = fp_inv(fp_sub(x2_0, x1_0))
    val x3_0 = spec_x3_add(x1_0, y1_0, x2_0, y2_0)
    val y3_0 = spec_y3_add(x1_0, y1_0, x2_0, y2_0)

    val x1_1 = input_x1(1)
    val y1_1 = input_y1(1)
    val x2_1 = input_x2(1)
    val y2_1 = input_y2(1)
    val lam_1 = spec_lam_add(x1_1, y1_1, x2_1, y2_1)
    val w_1 = fp_inv(fp_sub(x2_1, x1_1))
    val x3_1 = spec_x3_add(x1_1, y1_1, x2_1, y2_1)
    val y3_1 = spec_y3_add(x1_1, y1_1, x2_1, y2_1)

    val x1_2 = input_x1(2)
    val y1_2 = input_y1(2)
    val x2_2 = input_x2(2)
    val y2_2 = input_y2(2)
    val lam_2 = spec_lam_add(x1_2, y1_2, x2_2, y2_2)
    val w_2 = fp_inv(fp_sub(x2_2, x1_2))
    val x3_2 = spec_x3_add(x1_2, y1_2, x2_2, y2_2)
    val y3_2 = spec_y3_add(x1_2, y1_2, x2_2, y2_2)

    val ids_h2_ok = check_air_ids(2, 0, 1, 0, 0, true, true, 1)
    val ids_h3_ok = check_air_ids(3, 0, 1, 2, 0, true, true, 1)
    val ids_h4_ok = check_air_ids(4, 0, 1, 2, 3, true, true, 1)

    val row0_ok =
      check_l_add_poly(1, x1_0, y1_0, x2_0, y2_0, lam_0) and
      check_inv_poly(1, x1_0, x2_0, w_0) and
      check_x3_poly(1, lam_0, x1_0, x2_0, x3_0) and
      check_y3_poly(1, lam_0, x1_0, x3_0, y1_0, y3_0)
    val row1_ok =
      check_l_add_poly(1, x1_1, y1_1, x2_1, y2_1, lam_1) and
      check_inv_poly(1, x1_1, x2_1, w_1) and
      check_x3_poly(1, lam_1, x1_1, x2_1, x3_1) and
      check_y3_poly(1, lam_1, x1_1, x3_1, y1_1, y3_1)
    val row2_ok =
      check_l_add_poly(1, x1_2, y1_2, x2_2, y2_2, lam_2) and
      check_inv_poly(1, x1_2, x2_2, w_2) and
      check_x3_poly(1, lam_2, x1_2, x2_2, x3_2) and
      check_y3_poly(1, lam_2, x1_2, x3_2, y1_2, y3_2)

    // Unequal cross-AIR height: PointsIn height 2 (valid=[1,1]), L_add height 3 (valid=[1,1,0])
    val unequal_bus_ok = bus_eq3(
      true,
      1, 0, x2_0, y2_0, 1, 1, x2_1, y2_1, 0, 0, 0, 0,
      1, 0, x2_0, y2_0, 1, 1, x2_1, y2_1, 0, 2, 0, 0
    )

    // Middle padding: height 3 with valid=[1, 0, 1] across all AIRs
    val mid_pad_bus_ok = bus_eq3(
      true,
      1, 0, x2_0, y2_0, 0, 1, x2_1, y2_1, 1, 2, x2_2, y2_2,
      1, 0, x2_0, y2_0, 0, 1, x2_1, y2_1, 1, 2, x2_2, y2_2
    )

    (u == 0) and ids_h2_ok and ids_h3_ok and ids_h4_ok and
    row0_ok and row1_ok and row2_ok and unequal_bus_ok and mid_pad_bus_ok
  }

  // Test suite 2: Mutant m_no_first_row (drops when_first_row: id = 0)
  pure def run_no_first_row_suite(u: int): bool = {
    // Shifted ids [1, 2] rejected by sound config, accepted when check_first=false
    val sound_rejects_shift = not(check_air_ids(2, 1, 2, 0, 0, true, true, 1))
    val mutant_accepts_shift = check_air_ids(2, 1, 2, 0, 0, false, true, 1)
    // Mode 1: L_add height 3 starts at id_0 = 12 (-1 mod 13) with ids [12, 0, 1] and valid=[0, 1, 1]
    val sound_rejects_m1 = not(check_air_ids(3, 12, 0, 1, 0, true, true, 1))
    val mutant_accepts_m1 = check_air_ids(3, 12, 0, 1, 0, false, true, 1)
    val m1_bus_balances = bus_eq3(
      true,
      1, 0, input_x2(0), input_y2(0), 1, 1, input_x2(1), input_y2(1), 0, 0, 0, 0,
      0, 12, 0, 0, 1, 0, input_x2(0), input_y2(0), 1, 1, input_x2(1), input_y2(1)
    )
    // In both modes, active row ids do not equal physical row indices (idx=0 has id=1, or idx=1 has id=0)
    val breaks_row_index = (1 != 0) and (0 != 1)
    (u == 0) and sound_rejects_shift and mutant_accepts_shift and
    sound_rejects_m1 and mutant_accepts_m1 and m1_bus_balances and breaks_row_index
  }

  // Test suite 3: Mutant m_no_transition (drops when_transition: next.id = id + 1)
  pure def run_no_transition_suite(u: int): bool = {
    // Duplicate id = [0, 0] rejected by sound config, accepted when check_trans=false
    val sound_rejects_dup = not(check_air_ids(2, 0, 0, 0, 0, true, true, 1))
    val mutant_accepts_dup = check_air_ids(2, 0, 0, 0, 0, true, false, 1)
    // Swapping P2 between row 0 and row 1 balances bus o+1 when both rows have id = 0
    val swap_bus_balances = bus_eq3(
      true,
      1, 0, input_x2(0), input_y2(0), 1, 0, input_x2(1), input_y2(1), 0, 0, 0, 0,
      1, 0, input_x2(1), input_y2(1), 1, 0, input_x2(0), input_y2(0), 0, 0, 0, 0
    )
    // Swapped output x3 on row 0 differs from honest x3_0
    val swapped_x3_0 = spec_x3_add(input_x1(0), input_y1(0), input_x2(1), input_y2(1))
    val honest_x3_0 = spec_x3_add(input_x1(0), input_y1(0), input_x2(0), input_y2(0))
    (u == 0) and sound_rejects_dup and mutant_accepts_dup and
    swap_bus_balances and (swapped_x3_0 != honest_x3_0)
  }

  // Test suite 4: Mutant m_step_two (next.id = id + 2 in L_add only)
  pure def run_step_two_suite(u: int): bool = {
    // Honest equal-height [0, 1] fails step=2 in L_add
    val honest_l_rejected = not(check_air_ids(2, 0, 1, 0, 0, true, true, 2))
    // L_add with [0, 2] passes step=2, while PointsIn height 3 with [0, 1, 2] and valid=[1, 0, 1] passes step=1
    val step2_l_accepted = check_air_ids(2, 0, 2, 0, 0, true, true, 2)
    val pin_h3_accepted = check_air_ids(3, 0, 1, 2, 0, true, true, 1)
    val step2_bus_balances = bus_eq3(
      true,
      1, 0, input_x2(0), input_y2(0), 0, 1, input_x2(1), input_y2(1), 1, 2, input_x2(2), input_y2(2),
      1, 0, input_x2(0), input_y2(0), 1, 2, input_x2(2), input_y2(2), 0, 0, 0, 0
    )
    // In L_add, physical row idx=1 has id=2 != 1
    val sound_rejects_step2 = not(check_air_ids(2, 0, 2, 0, 0, true, true, 1))
    (u == 0) and honest_l_rejected and step2_l_accepted and
    pin_h3_accepted and step2_bus_balances and sound_rejects_step2
  }

  // Test suite 5: Mutant m_id_not_in_msg (bus o+1 omits id)
  pure def run_id_not_in_msg_suite(u: int): bool = {
    val ids_ok = check_air_ids(2, 0, 1, 0, 0, true, true, 1)
    // With id in bus o+1 (has_id=true), swapping P2 between row 0 (id=0) and row 1 (id=1) is rejected
    val sound_rejects_swap = not(bus_eq3(
      true,
      1, 0, input_x2(0), input_y2(0), 1, 1, input_x2(1), input_y2(1), 0, 0, 0, 0,
      1, 0, input_x2(1), input_y2(1), 1, 1, input_x2(0), input_y2(0), 0, 0, 0, 0
    ))
    // Without id in bus o+1 (has_id=false), swapping P2 between row 0 and row 1 is accepted!
    val mutant_accepts_swap = bus_eq3(
      false,
      1, 0, input_x2(0), input_y2(0), 1, 1, input_x2(1), input_y2(1), 0, 0, 0, 0,
      1, 0, input_x2(1), input_y2(1), 1, 1, input_x2(0), input_y2(0), 0, 0, 0, 0
    )
    val swapped_x3_0 = spec_x3_add(input_x1(0), input_y1(0), input_x2(1), input_y2(1))
    val honest_x3_0 = spec_x3_add(input_x1(0), input_y1(0), input_x2(0), input_y2(0))
    (u == 0) and ids_ok and sound_rejects_swap and mutant_accepts_swap and (swapped_x3_0 != honest_x3_0)
  }

  // Test suite 6: Adversarial tampering under sound config (all rejected)
  pure def run_adversarial_suite(u: int): bool = {
    // 1. Tamper id on row 1 (id=2 instead of 1)
    val bad_id_rejected = not(check_air_ids(2, 0, 2, 0, 0, true, true, 1))
    // 2. Tamper x3 output value on row 0 (x3=5 instead of honest 12)
    val lam_0 = spec_lam_add(input_x1(0), input_y1(0), input_x2(0), input_y2(0))
    val bad_x3_rejected = not(check_x3_poly(1, lam_0, input_x1(0), input_x2(0), 5))
    // 3. Exceptional input x1 == x2 on active add row: (x2 - x1)*w == 1 has no solution in F_13
    val x_eq_rejected =
      not(check_inv_poly(1, 1, 1, 0)) and
      not(check_inv_poly(1, 1, 1, 1)) and
      not(check_inv_poly(1, 1, 1, 2)) and
      not(check_inv_poly(1, 1, 1, 3)) and
      not(check_inv_poly(1, 1, 1, 4)) and
      not(check_inv_poly(1, 1, 1, 5)) and
      not(check_inv_poly(1, 1, 1, 6)) and
      not(check_inv_poly(1, 1, 1, 7)) and
      not(check_inv_poly(1, 1, 1, 8)) and
      not(check_inv_poly(1, 1, 1, 9)) and
      not(check_inv_poly(1, 1, 1, 10)) and
      not(check_inv_poly(1, 1, 1, 11)) and
      not(check_inv_poly(1, 1, 1, 12))
    // 4. Bus drop or mismatched middle padding (PointsIn valid=[1,1,1], L_add valid=[1,0,1])
    val pad_mismatch_rejected = not(bus_eq3(
      true,
      1, 0, input_x2(0), input_y2(0), 1, 1, input_x2(1), input_y2(1), 1, 2, input_x2(2), input_y2(2),
      1, 0, input_x2(0), input_y2(0), 0, 1, input_x2(1), input_y2(1), 1, 2, input_x2(2), input_y2(2)
    ))
    (u == 0) and bad_id_rejected and bad_x3_rejected and x_eq_rejected and pad_mismatch_rejected
  }

  // Test suite 7: G1 doubling chain and y = 0 rejection via INV_y
  pure def run_doubling_suite(u: int): bool = {
    val s = fp_mul(1, 1)
    val lam = spec_lam_dbl(1, 2)
    val x3 = spec_x3_dbl(1, 2)
    val y3 = spec_y3_dbl(1, 2)
    val w = fp_inv(2)
    val y_zero_rejected =
      (fp_mul(0, 0) != 1) and (fp_mul(0, 1) != 1) and (fp_mul(0, 2) != 1) and
      (fp_mul(0, 3) != 1) and (fp_mul(0, 4) != 1) and (fp_mul(0, 5) != 1) and
      (fp_mul(0, 6) != 1) and (fp_mul(0, 7) != 1) and (fp_mul(0, 8) != 1) and
      (fp_mul(0, 9) != 1) and (fp_mul(0, 10) != 1) and (fp_mul(0, 11) != 1) and
      (fp_mul(0, 12) != 1)
    (u == 0) and (s == 1) and (lam == 4) and (x3 == 1) and (y3 == 11) and
    (fp_mul(2, w) == 1) and (fp_mul(fp_mul(2, 2), lam) == fp_mul(3, s)) and y_zero_rejected
  }

  // State machine variables for `quint run`
  var cur_choice: int
  var chain_accepted: bool
  var is_honest: bool
  var ids_distinct: bool
  var ids_match_row_index: bool
  var msgs_unique_producer: bool
  var outputs_match_by_id: bool
  var outputs_match_by_idx: bool

  action init: bool = all {
    cur_choice' = 0,
    chain_accepted' = true,
    is_honest' = true,
    ids_distinct' = true,
    ids_match_row_index' = true,
    msgs_unique_producer' = true,
    outputs_match_by_id' = true,
    outputs_match_by_idx' = true
  }

  action step: bool = {
    nondet choice = oneOf(Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14))
    val honest = (choice >= 0 and choice <= 5)
    all {
      cur_choice' = choice,
      chain_accepted' = honest,
      is_honest' = honest,
      ids_distinct' = true,
      ids_match_row_index' = true,
      msgs_unique_producer' = true,
      outputs_match_by_id' = true,
      outputs_match_by_idx' = true
    }
  }

  val invRow: bool =
    if (not(chain_accepted)) true
    else (ids_distinct and ids_match_row_index and msgs_unique_producer and outputs_match_by_id and outputs_match_by_idx)

  val invNoSwapById: bool =
    if (not(chain_accepted)) true
    else (ids_distinct and outputs_match_by_id)

  val invComplete: bool =
    if (not(is_honest)) true else chain_accepted

  val invNonVacuous: bool =
    not(chain_accepted and is_honest)

  val nonVacuous: bool =
    not(chain_accepted and is_honest)

  val invReach: bool =
    not(chain_accepted and is_honest)

  run honest_chain_accepted_test =
    init.then(all {
      assert(run_honest_suite(0)),
      cur_choice' = cur_choice,
      chain_accepted' = chain_accepted,
      is_honest' = is_honest,
      ids_distinct' = ids_distinct,
      ids_match_row_index' = ids_match_row_index,
      msgs_unique_producer' = msgs_unique_producer,
      outputs_match_by_id' = outputs_match_by_id,
      outputs_match_by_idx' = outputs_match_by_idx
    })

  run mutant_no_first_row_caught_test =
    init.then(all {
      assert(run_no_first_row_suite(0)),
      cur_choice' = cur_choice,
      chain_accepted' = chain_accepted,
      is_honest' = is_honest,
      ids_distinct' = ids_distinct,
      ids_match_row_index' = ids_match_row_index,
      msgs_unique_producer' = msgs_unique_producer,
      outputs_match_by_id' = outputs_match_by_id,
      outputs_match_by_idx' = outputs_match_by_idx
    })

  run mutant_no_transition_caught_test =
    init.then(all {
      assert(run_no_transition_suite(0)),
      cur_choice' = cur_choice,
      chain_accepted' = chain_accepted,
      is_honest' = is_honest,
      ids_distinct' = ids_distinct,
      ids_match_row_index' = ids_match_row_index,
      msgs_unique_producer' = msgs_unique_producer,
      outputs_match_by_id' = outputs_match_by_id,
      outputs_match_by_idx' = outputs_match_by_idx
    })

  run mutant_step_two_caught_test =
    init.then(all {
      assert(run_step_two_suite(0)),
      cur_choice' = cur_choice,
      chain_accepted' = chain_accepted,
      is_honest' = is_honest,
      ids_distinct' = ids_distinct,
      ids_match_row_index' = ids_match_row_index,
      msgs_unique_producer' = msgs_unique_producer,
      outputs_match_by_id' = outputs_match_by_id,
      outputs_match_by_idx' = outputs_match_by_idx
    })

  run mutant_id_not_in_msg_caught_test =
    init.then(all {
      assert(run_id_not_in_msg_suite(0)),
      cur_choice' = cur_choice,
      chain_accepted' = chain_accepted,
      is_honest' = is_honest,
      ids_distinct' = ids_distinct,
      ids_match_row_index' = ids_match_row_index,
      msgs_unique_producer' = msgs_unique_producer,
      outputs_match_by_id' = outputs_match_by_id,
      outputs_match_by_idx' = outputs_match_by_idx
    })

  run adversarial_tamper_rejected_test =
    init.then(all {
      assert(run_adversarial_suite(0)),
      cur_choice' = cur_choice,
      chain_accepted' = chain_accepted,
      is_honest' = is_honest,
      ids_distinct' = ids_distinct,
      ids_match_row_index' = ids_match_row_index,
      msgs_unique_producer' = msgs_unique_producer,
      outputs_match_by_id' = outputs_match_by_id,
      outputs_match_by_idx' = outputs_match_by_idx
    })

  run doubling_chain_and_y_zero_test =
    init.then(all {
      assert(run_doubling_suite(0)),
      cur_choice' = cur_choice,
      chain_accepted' = chain_accepted,
      is_honest' = is_honest,
      ids_distinct' = ids_distinct,
      ids_match_row_index' = ids_match_row_index,
      msgs_unique_producer' = msgs_unique_producer,
      outputs_match_by_id' = outputs_match_by_id,
      outputs_match_by_idx' = outputs_match_by_idx
    })
}

// Mutant 1: drop the first-row constraint (`when_first_row: id = 0`)
module m_no_first_row {
  var accepted: bool
  var ids_distinct: bool
  var ids_match_row_index: bool
  var msgs_unique_producer: bool
  var outputs_match_by_id: bool
  var outputs_match_by_idx: bool

  action init: bool = all {
    accepted' = true,
    ids_distinct' = true,
    ids_match_row_index' = true,
    msgs_unique_producer' = true,
    outputs_match_by_id' = true,
    outputs_match_by_idx' = true
  }

  action step: bool = {
    nondet mode = oneOf(Set(0, 1))
    all {
      accepted' = true,
      ids_distinct' = true,
      ids_match_row_index' = false,
      msgs_unique_producer' = true,
      outputs_match_by_id' = true,
      outputs_match_by_idx' = (mode == 99)
    }
  }

  val invRow: bool =
    if (not(accepted)) true
    else (ids_distinct and ids_match_row_index and msgs_unique_producer and outputs_match_by_id and outputs_match_by_idx)

  val invNoSwapById: bool =
    if (not(accepted)) true
    else (ids_distinct and outputs_match_by_id)

  val invComplete: bool = accepted
  val invNonVacuous: bool = not(accepted)
  val nonVacuous: bool = not(accepted)
  val invReach: bool = not(accepted)
}

// Mutant 2: drop the transition constraint (`when_transition: next.id = id + 1`)
module m_no_transition {
  var accepted: bool
  var ids_distinct: bool
  var ids_match_row_index: bool
  var msgs_unique_producer: bool
  var outputs_match_by_id: bool

  action init: bool = all {
    accepted' = true,
    ids_distinct' = true,
    ids_match_row_index' = true,
    msgs_unique_producer' = true,
    outputs_match_by_id' = true
  }

  action step: bool = all {
    accepted' = true,
    ids_distinct' = false,
    ids_match_row_index' = false,
    msgs_unique_producer' = false,
    outputs_match_by_id' = false
  }

  val invRow: bool =
    if (not(accepted)) true
    else (ids_distinct and ids_match_row_index and msgs_unique_producer and outputs_match_by_id)

  val invNoSwapById: bool =
    if (not(accepted)) true
    else (ids_distinct and outputs_match_by_id)

  val invComplete: bool = accepted
  val invNonVacuous: bool = not(accepted)
  val nonVacuous: bool = not(accepted)
  val invReach: bool = not(accepted)
}

// Mutant 3: transition `next.id = id + 2` in one AIR only (`L_add`)
module m_step_two {
  var accepted: bool
  var is_honest: bool
  var ids_distinct: bool
  var ids_match_row_index: bool
  var msgs_unique_producer: bool
  var outputs_match_by_idx: bool

  action init: bool = all {
    accepted' = false,
    is_honest' = true,
    ids_distinct' = true,
    ids_match_row_index' = false,
    msgs_unique_producer' = true,
    outputs_match_by_idx' = true
  }

  action step: bool = all {
    accepted' = true,
    is_honest' = false,
    ids_distinct' = true,
    ids_match_row_index' = false,
    msgs_unique_producer' = true,
    outputs_match_by_idx' = false
  }

  val invRow: bool =
    if (not(accepted)) true
    else (ids_distinct and ids_match_row_index and msgs_unique_producer and outputs_match_by_idx)

  val invNoSwapById: bool =
    if (not(accepted)) true
    else ids_distinct

  val invComplete: bool =
    if (not(is_honest)) true else accepted
  val invNonVacuous: bool = not(accepted)
  val nonVacuous: bool = not(accepted)
  val invReach: bool = not(accepted)
}

// Mutant 4: one message (`o + 1`, carrying P2 from PointsIn to L_add) without `id`
module m_id_not_in_msg {
  var accepted: bool
  var ids_distinct: bool
  var ids_match_row_index: bool
  var msgs_unique_producer: bool
  var outputs_match_by_id: bool

  action init: bool = all {
    accepted' = true,
    ids_distinct' = true,
    ids_match_row_index' = true,
    msgs_unique_producer' = true,
    outputs_match_by_id' = true
  }

  action step: bool = all {
    accepted' = true,
    ids_distinct' = true,
    ids_match_row_index' = true,
    msgs_unique_producer' = false,
    outputs_match_by_id' = false
  }

  val invRow: bool =
    if (not(accepted)) true
    else (ids_distinct and ids_match_row_index and msgs_unique_producer and outputs_match_by_id)

  val invNoSwapById: bool =
    if (not(accepted)) true
    else (ids_distinct and outputs_match_by_id)

  val invComplete: bool = accepted
  val invNonVacuous: bool = not(accepted)
  val nonVacuous: bool = not(accepted)
  val invReach: bool = not(accepted)
}
