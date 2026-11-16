// aluArith: the ALU kit constraint system at a reduced width (task 2.1, spec
// `sherlock-chip-alu` ALU-001 to ALU-004; FD ARCHITECTURE "Kit: ALU", Verification).
//
// Written from the spec and the constraint list of the AIR, not from the Rust trace generator.
// The AIR (openvm branch kit-alu, crates/sherlock-leaf/src/alu.rs) works over BabyBear with
// 16-bit message limbs (8 per u128 lane) and 8-bit sub-limbs ("bytes") for the multiply, because
// one 16 x 16-bit limb product (about 2^32) is above p (about 2^31). This model keeps the same
// equations at a reduced width:
//
//   real AIR                       model
//   p = 2013265921                 P = 13
//   limb 16 bits, 8 limbs          limb 2 bits (base 4), 2 limbs: a word is 4 bits (W = 16)
//   byte 8 bits, 16 bytes          "byte" 1 bit, 4 bits per word
//   2^128                          W = 16
//   MUL carries ranged to 16 bits  MUL carries ranged to 2 bits (max 3)
//   -1/2^16 mod p = 30720 < 2^16   -1/4 mod P = 3 < 4 (a non-boolean top carry fits a limb in both)
//
// In both, every honest equation has both sides below P as integers (no wrap), and P is small
// enough here that every weakened constraint set admits a field-wrapped or non-boolean witness.
//
// Constraints of one active provider row (selector s_op one-hot, op = sum k s_k):
//   R  link:   x_i = xb_2i + 2 xb_2i+1, the same for y and z; every bit ranged (the in-chip range
//              check of x, y, z, ALU-002)
//   A  chain:  A_i + B_i + c_(i-1) - C_i - 4 c_i = 0 with (A, B, C) = (x, y, z) for ADD,
//              (z, y, x) for SUB, (w, y, x) for LT; c_i boolean; w ranged
//   F  flag:   ADD, SUB: flag = c_top; LT, EQ: flag = 0; LT: z = (c_top, 0)
//   E  EQ:     z_0 (1 - z_0) = 0 (LEAD PICK ALU-EQBOOL); z_0 (x_i - y_i) = 0 for each i
//              (forward); sum_i (x_i - y_i) inv_i + z_0 - 1 = 0 (reverse); z_1 = 0
//   M  MUL:    schoolbook over bits: conv_k + cr_(k-1) = ob_k + 2 cr_k (k = 0..6),
//              conv_7 + cr_6 = ob_7; ob = (z bits, h bits); cr ranged; flag = 0 implies h = 0;
//              flag = 1 implies sum h != 0 (inverse witness)
//   N  op:     the op word of the message is sum k s_k
//
// Each weakened variant removes one constraint group, the same one as the weak bit of the AIR of
// the same name. The model checks:
//   uniqueOk:   for every op in the list and every ranged x, y, the correct constraint set
//               admits exactly one (z, flag), and it is the spec result (ALU-001, ALU-003)
//   rangeOk:    an out-of-range or field-wrapped operand or result limb is refused (ALU-002)
//   opListOk:   an op outside the list is refused (ALU-004)
//   mutant_*:   each weakened variant admits the named forged witness that the correct set refuses
//
// Check: quint test --main aluArithTest --match "^(unique|range|op|overflow|mutant|weak|eqBool)" aluArith.qnt
// (every run must pass).

module aluArith {
  pure val P = 13
  pure val BASE = 4
  pure val W = 16
  pure val LIMBS = 0.to(BASE - 1)
  pure val FIELD = 0.to(P - 1)
  pure val WORDS = 0.to(W - 1)

  pure val ADD = 1
  pure val LT = 2
  pure val SUB = 3
  pure val MUL = 4
  pure val EQ = 5
  pure val OPS = Set(ADD, LT, SUB, MUL, EQ)

  // Weak bits (the constraint group each one removes; names as in the AIR).
  pure val W_FLAG_LINK = "flag_link"       // ADD/SUB flag = c_top
  pure val W_CARRY_BOOL = "carry_bool"     // c_i boolean below the top
  pure val W_TOP_BOOL = "top_bool"         // c_top boolean (the LT boolean)
  pure val W_X_RANGE = "x_range"           // x link to ranged bits
  pure val W_Z_RANGE = "z_range"           // z link to ranged bits
  pure val W_MUL_FIELD = "mul_field"       // schoolbook replaced by one field equation
  pure val W_MUL_HIGH = "mul_high"         // flag = 0 implies high half zero
  pure val W_MUL_CARRY = "mul_carry"       // MUL carry range
  pure val W_EQ_REV = "eq_rev"             // EQ reverse implication
  pure val W_OP_LINK = "op_link"           // op word = sum k s_k
  pure val W_EQ_BOOL = "eq_bool"           // EQ z_0 boolean (LEAD PICK ALU-EQBOOL)

  pure def md(a: int): int = ((a % P) + P) % P
  pure def finv(a: int): int = FIELD.filter(i => md(a * i) == 1).fold(0, (s, i) => i)
  pure def bit(v: int, k: int): int = (v / (2 ^ k)) % 2
  pure def on(weak: Set[str], g: str): bool = not(weak.contains(g))

  // Spec result of (op, x, y) on 4-bit words: (z, flag).
  pure def spec(op: int, x: int, y: int): (int, int) =
    if (op == ADD) (if (x + y >= W) (x + y - W, 1) else (x + y, 0))
    else if (op == SUB) (if (x >= y) (x - y, 0) else (x - y + W, 1))
    else if (op == MUL) ((x * y) % W, if (x * y >= W) 1 else 0)
    else if (op == LT) (if (x < y) (1, 0) else (0, 0))
    else (if (x == y) (1, 0) else (0, 0))

  pure def ob(zb: int, hb: int, k: int): int = if (k < 4) bit(zb, k) else bit(hb, k - 4)
  pure def conv(xb: int, yb: int, k: int): int =
    0.to(3).fold(0, (s, i) => if (k - i >= 0 and k - i <= 3) s + bit(xb, i) * bit(yb, k - i) else s)

  // The bit vectors (as a 4-bit word) that the link R allows for limbs (l0, l1): with the link,
  // only the ranged reconstruction; without it, any bits.
  pure def bitsOf(linked: bool, l0: int, l1: int): Set[int] =
    if (not(linked)) WORDS
    else if (LIMBS.contains(l0) and LIMBS.contains(l1)) Set(l0 + BASE * l1)
    else Set()

  // Chain A for limbs (a, b, c) = (A, B, C): returns (ok, c_top) under the boolean bits.
  pure def chain(weak: Set[str], a0: int, a1: int, b0: int, b1: int, k0: int, k1: int): (bool, int) =
    val i4 = finv(BASE)
    val c0 = md((a0 + b0 - k0) * i4)
    val c1 = md((a1 + b1 + c0 - k1) * i4)
    val ok0 = (not(on(weak, W_CARRY_BOOL)) or c0 <= 1)
    val ok1 = (not(on(weak, W_TOP_BOOL)) or c1 <= 1)
    (ok0 and ok1, c1)

  // Exists a witness for selector `sel` on message limbs (x, y, z, flag)?
  pure def admitsSel(weak: Set[str], sel: int, x0: int, x1: int, y0: int, y1: int, z0: int, z1: int, flag: int): bool =
    val xbs = bitsOf(on(weak, W_X_RANGE), x0, x1)
    val ybs = bitsOf(true, y0, y1)
    val zbs = bitsOf(on(weak, W_Z_RANGE), z0, z1)
    val linkOk = xbs.size() > 0 and ybs.size() > 0 and zbs.size() > 0
    linkOk and (flag == 0 or flag == 1) and (
      if (sel == ADD or sel == SUB) {
        val r = if (sel == ADD) chain(weak, x0, x1, y0, y1, z0, z1) else chain(weak, z0, z1, y0, y1, x0, x1)
        r._1 and (not(on(weak, W_FLAG_LINK)) or flag == r._2)
      } else if (sel == LT) {
        WORDS.exists(w => {
          val r = chain(weak, w % BASE, w / BASE, y0, y1, x0, x1)
          r._1 and flag == 0 and z0 == r._2 and z1 == 0
        })
      } else if (sel == EQ) {
        val d0 = md(x0 - y0)
        val d1 = md(x1 - y1)
        val fwd = md(z0 * d0) == 0 and md(z1 * 0) == 0 and md(z0 * d1) == 0
        // reverse: exists inv with d0 inv0 + d1 inv1 + z0 - 1 = 0 (one inverse nonzero suffices)
        val rev = FIELD.exists(i => md(d0 * i + z0 - 1) == 0 or md(d1 * i + z0 - 1) == 0)
        val zbool = md(z0 * (1 - z0)) == 0
        fwd and (not(on(weak, W_EQ_REV)) or rev) and (not(on(weak, W_EQ_BOOL)) or zbool) and z1 == 0 and flag == 0
      } else if (sel == MUL) {
        xbs.exists(xb => ybs.exists(yb => zbs.exists(zb => WORDS.exists(hb => {
          val i2 = finv(2)
          // carries by field division; ranged to 2 bits unless weakened
          val cr = range(0, 7).foldl([], (acc, k) => {
            val prev = if (k == 0) 0 else acc[k - 1]
            acc.append(md((conv(xb, yb, k) + prev - ob(zb, hb, k)) * i2))
          })
          val school = md(conv(xb, yb, 7) + cr[6] - ob(zb, hb, 7)) == 0 and
            (not(on(weak, W_MUL_CARRY)) or 0.to(6).forall(k => cr[k] <= 3))
          val xr = md(x0 + BASE * x1)
          val yr = md(y0 + BASE * y1)
          val fieldEq = md(xr * yr - (z0 + BASE * z1) - W * hb) == 0
          val prodOk = if (on(weak, W_MUL_FIELD)) school else fieldEq
          val hsum = 0.to(3).fold(0, (s, k) => s + bit(hb, k))
          val highOk = (not(on(weak, W_MUL_HIGH)) or flag == 1 or hb == 0) and (flag == 0 or md(hsum) != 0)
          prodOk and highOk
        }))))
      } else false
    )

  // The provider admits the message (op, flag, x, y, z): some one-hot selector row provides it.
  pure def admits(weak: Set[str], op: int, x0: int, x1: int, y0: int, y1: int, z0: int, z1: int, flag: int): bool =
    OPS.exists(sel => (op == sel or not(on(weak, W_OP_LINK))) and admitsSel(weak, sel, x0, x1, y0, y1, z0, z1, flag))

  pure def admitsW(weak: Set[str], op: int, x: int, y: int, z: int, flag: int): bool =
    admits(weak, op, x % BASE, x / BASE, y % BASE, y / BASE, z % BASE, z / BASE, flag)
}

module aluArithTest {
  import aluArith.*

  pure val NONE: Set[str] = Set()

  // ALU-001 / ALU-003: exactly the spec (z, flag) for every op and every ranged x, y.
  pure def isUnique(op: int): bool =
    WORDS.forall(x => WORDS.forall(y =>
      WORDS.forall(z => 0.to(1).forall(f =>
        admitsW(NONE, op, x, y, z, f) == ((z, f) == spec(op, x, y))))))

  run uniqueAdd = assert(isUnique(ADD))
  run uniqueSub = assert(isUnique(SUB))
  run uniqueLt = assert(isUnique(LT))
  run uniqueEq = assert(isUnique(EQ))
  run uniqueMul = assert(isUnique(MUL))

  // ALU-002: a field-wrapped or out-of-range limb of x or z is refused for every op.
  run rangeOk = assert(OPS.forall(op => Set(4, 5, P - 5, P - 1).forall(bad =>
    WORDS.forall(z => 0.to(1).forall(f =>
      not(admits(NONE, op, bad, 0, 2, 0, z % BASE, z / BASE, f)) and
      not(admits(NONE, op, 1, 0, 2, 0, bad, 0, f)))))))

  // ALU-004: an op outside the list (0, 6, 7) is refused.
  run opListOk = assert(Set(0, 6, 7).forall(op => WORDS.forall(x => WORDS.forall(z =>
    0.to(1).forall(f => not(admitsW(NONE, op, x, 2, z, f)))))))

  // ALU-003 completeness: honest overflow is provable with flag 1.
  run overflowProvable = assert(admitsW(NONE, ADD, 15, 1, 0, 1) and admitsW(NONE, MUL, 4, 4, 0, 1)
    and admitsW(NONE, SUB, 0, 1, 15, 1))

  // Mutants: the weakened set admits the forged witness; the correct set refuses it.
  pure def caught(g: str, op: int, x0: int, x1: int, y0: int, y1: int, z0: int, z1: int, f: int): bool =
    admits(Set(g), op, x0, x1, y0, y1, z0, z1, f) and not(admits(NONE, op, x0, x1, y0, y1, z0, z1, f))

  // alu_second_result_for_same_inputs: ADD 1 + 2 = 3 also with flag 1.
  run mutant_second_result = assert(caught(W_FLAG_LINK, ADD, 1, 0, 2, 0, 3, 0, 1))
  // alu_carry_forged: a non-boolean low carry gives a wrong ADD result (some z != spec).
  run mutant_carry_forged = assert(WORDS.exists(z => 0.to(1).exists(f =>
    (z, f) != spec(ADD, 1, 2) and caught(W_CARRY_BOOL, ADD, 1, 0, 2, 0, z % BASE, z / BASE, f))))
  // alu_borrow_forged: the same for SUB.
  run mutant_borrow_forged = assert(WORDS.exists(x => WORDS.exists(z => 0.to(1).exists(f =>
    (z, f) != spec(SUB, x, 1) and caught(W_CARRY_BOOL, SUB, x % BASE, x / BASE, 1, 0, z % BASE, z / BASE, f)))))
  // alu_operand_range_missing: x limb 0 = p - 3 (field-wrapped, the reduced form of p - 5), y limb 0 = 3:
  // the field sum wraps to 0 and the AIR without the link admits it.
  run mutant_operand_range = assert(LIMBS.exists(z0 => LIMBS.exists(z1 => caught(W_X_RANGE, ADD, P - 3, 0, 3, 0, z0, z1, 0))))
  // alu_lt_field_wrap: x = p - 1 (limb 0), y = 2 claims x < y.
  run mutant_lt_field_wrap = assert(caught(W_X_RANGE, LT, P - 1, 0, 2, 0, 1, 0, 0))
  // alu_output_range_missing: an unranged z limb with the right integer sum.
  run mutant_output_range = assert(FIELD.exists(z0 => FIELD.exists(z1 =>
    not(LIMBS.contains(z0) and LIMBS.contains(z1)) and caught(W_Z_RANGE, ADD, 1, 0, 2, 0, z0, z1, 0))))
  // alu_lt_boolean_removed: a non-boolean LT result.
  run mutant_lt_boolean = assert(LIMBS.exists(z0 => z0 > 1 and WORDS.exists(x => WORDS.exists(y =>
    caught(W_TOP_BOOL, LT, x % BASE, x / BASE, y % BASE, y / BASE, z0, 0, 0)))))
  // alu_mul_field_residue: 5 * 5 = 25; the residue 25 mod 13 = 12 claimed with flag 0.
  run mutant_mul_field_residue = assert(caught(W_MUL_FIELD, MUL, 1, 1, 1, 1, 0, 3, 0))
  // alu_mul_high_half_dropped: 4 * 4 = 16 with z = 0, flag 0.
  run mutant_mul_high_dropped = assert(caught(W_MUL_HIGH, MUL, 0, 1, 0, 1, 0, 0, 0))
  // alu_mul_c1_overflow: unranged carries, the product 1 * 1 = 1 offered as 1 + p = 14 with flag 0.
  run mutant_mul_carry = assert(caught(W_MUL_CARRY, MUL, 1, 0, 1, 0, 2, 3, 0))
  // alu_eq_reverse_implication_removed: 7 = 7 with z = 0.
  run mutant_eq_rev = assert(caught(W_EQ_REV, EQ, 3, 1, 3, 1, 0, 0, 0))
  // bus_key_namespace_alias (ALU case): op 6 provided by an ADD row.
  run mutant_op_alias = assert(caught(W_OP_LINK, 6, 0, 0, 0, 0, 0, 0, 0))

  // Each weakened variant breaks uniqueness somewhere in the domain (not only at the named witness).
  pure def isBroken(g: str, op: int): bool =
    not(WORDS.forall(x => WORDS.forall(y => WORDS.forall(z => 0.to(1).forall(f =>
      admitsW(Set(g), op, x, y, z, f) == ((z, f) == spec(op, x, y)))))))
  // ALU-EQBOOL: in a prime field the EQ boolean is implied by the two implications, so removing it
  // admits no new witness (no mutant exists for it; it is there for the Lean proof of O4).
  run eqBoolRedundant = assert(WORDS.forall(x => WORDS.forall(y => FIELD.forall(z0 =>
    admits(Set(W_EQ_BOOL), EQ, x % BASE, x / BASE, y % BASE, y / BASE, z0, 0, 0) ==
    admits(NONE, EQ, x % BASE, x / BASE, y % BASE, y / BASE, z0, 0, 0)))))

  run weakBreaksUniqueness = assert(isBroken(W_FLAG_LINK, ADD) and isBroken(W_CARRY_BOOL, ADD) and isBroken(W_CARRY_BOOL, SUB)
    and isBroken(W_TOP_BOOL, LT) and isBroken(W_EQ_REV, EQ) and isBroken(W_MUL_HIGH, MUL) and isBroken(W_MUL_FIELD, MUL)
    and isBroken(W_MUL_CARRY, MUL))
}
