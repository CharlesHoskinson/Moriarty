/-!
O4 (kit ALU, spec `sherlock-chip-alu` ALU-001 to ALU-003): the ALU AIR of openvm branch `kit-alu`
(`crates/sherlock-leaf/src/alu.rs`) over BabyBear, p = 2013265921, 16-bit message limbs.

A field element is a natural number below `p`; a field equation `a = b` of the AIR is
`a % p = b % p` on such representatives. The AIR's subtraction form `A + B + c - C - 2^16 c' = 0`
is the same field equation as `A + B + c = C + 2^16 c'`.

Proved here (imports only `Init`; `#print axioms` at the end):
* `no_wrap`: a field equation whose two sides are below `p` as integers is an integer equation.
* `chain_sound`: the 8-limb ADD/SUB/LT chain with 16-bit limbs and boolean carries implies
  `A + B = C + 2^128 c_7` over the integers (no field wrap). ADD is `(x, y, z)`, SUB `(z, y, x)`,
  LT `(w, y, x)`.
* `add_spec`, `sub_spec`, `lt_spec`: the integer relation of the op table and a unique `(z, flag)`.
* `mul_column_no_wrap`, `mul_chain_unique`: each MUL column equation has both sides below `p`
  (bytes below 256, carries below 2^16, at most 16 products per column), so it is an integer
  equation, and the output bytes and carries are a function of the column sums: two witnesses with
  the same operands have the same `z` bytes and the same high bytes.
* `mul_flag_unique`: the flag is a function of the high bytes (`flag = 0` forces them to zero,
  `flag = 1` needs an inverse of their sum, which does not exist for the sum 0).
* `eq_spec`: EQ gives `z = [x = y]`, under the premise that `p` is prime (no zero divisors among
  the field representatives). The premise is a hypothesis of the theorem, not an axiom.

* `schoolbook`, `mul_spec`: the byte product is the sum of the column sums, and the column chain
  gives `z = X Y mod 2^128` and the high half `X Y / 2^128`; with `mul_flag_unique`,
  `flag = [X Y >= 2^128]`. Every intermediate integer is below `p` per column, so the multiply has
  no field wrap.

* `chk_small`, `dvd_p`, `no_zero_div`: 2013265921 is prime (trial division to 44,870, checked
  by the kernel with `decide +kernel`, no extra axiom), so the field has no zero divisors, and
  `field_bool`: every `assert_bool` cell is 0 or 1.
* `add_row`, `sub_row`, `lt_row`, `eq_row`, `mul_row` (O4, composed): from the constraints of one
  provider row of each op (selectors substituted, every cell a field representative, the range
  lookups as bounds on the ranged cells) to the op table on the integers. So for each `(op, x, y)`
  the row admits exactly one `(z, flag)`, every equation is an integer equation (no field wrap),
  and the conclusion holds for every field value of the unranged inverse witnesses. `eq_row` uses
  the EQ boolean of LEAD PICK ALU-EQBOOL.

Not covered here: the one-hot selector lemma (selectors boolean, `act` boolean and
`op = sum k s_k` give exactly one op of the list) is checked by `aluArith.qnt` and the
`bus_key_namespace_alias` mutant, not stated in Lean; the Clean `Module` instance form.
-/

set_option linter.unusedVariables false
set_option maxRecDepth 100000

namespace Clean.Sherlock.AluRange

def p : Nat := 2013265921

theorem no_wrap {a b : Nat} (ha : a < p) (hb : b < p) (h : a % p = b % p) : a = b := by
  rwa [Nat.mod_eq_of_lt ha, Nat.mod_eq_of_lt hb] at h

/-- The ADD/SUB/LT chain: `A_i + B_i + c_(i-1) = C_i + 2^16 c_i` in the field for i = 0..7. -/
theorem chain_sound (a0 a1 a2 a3 a4 a5 a6 a7 : Nat) (b0 b1 b2 b3 b4 b5 b6 b7 : Nat) (c0 c1 c2 c3 c4 c5 c6 c7 : Nat) (k0 k1 k2 k3 k4 k5 k6 k7 : Nat)
    (ha0 : a0 < 65536)
    (ha1 : a1 < 65536)
    (ha2 : a2 < 65536)
    (ha3 : a3 < 65536)
    (ha4 : a4 < 65536)
    (ha5 : a5 < 65536)
    (ha6 : a6 < 65536)
    (ha7 : a7 < 65536)
    (hb0 : b0 < 65536)
    (hb1 : b1 < 65536)
    (hb2 : b2 < 65536)
    (hb3 : b3 < 65536)
    (hb4 : b4 < 65536)
    (hb5 : b5 < 65536)
    (hb6 : b6 < 65536)
    (hb7 : b7 < 65536)
    (hc0 : c0 < 65536)
    (hc1 : c1 < 65536)
    (hc2 : c2 < 65536)
    (hc3 : c3 < 65536)
    (hc4 : c4 < 65536)
    (hc5 : c5 < 65536)
    (hc6 : c6 < 65536)
    (hc7 : c7 < 65536)
    (hk0 : k0 ≤ 1)
    (hk1 : k1 ≤ 1)
    (hk2 : k2 ≤ 1)
    (hk3 : k3 ≤ 1)
    (hk4 : k4 ≤ 1)
    (hk5 : k5 ≤ 1)
    (hk6 : k6 ≤ 1)
    (hk7 : k7 ≤ 1)
    (e0 : (a0 + b0) % p = (c0 + 65536 * k0) % p)
    (e1 : (a1 + b1 + k0) % p = (c1 + 65536 * k1) % p)
    (e2 : (a2 + b2 + k1) % p = (c2 + 65536 * k2) % p)
    (e3 : (a3 + b3 + k2) % p = (c3 + 65536 * k3) % p)
    (e4 : (a4 + b4 + k3) % p = (c4 + 65536 * k4) % p)
    (e5 : (a5 + b5 + k4) % p = (c5 + 65536 * k5) % p)
    (e6 : (a6 + b6 + k5) % p = (c6 + 65536 * k6) % p)
    (e7 : (a7 + b7 + k6) % p = (c7 + 65536 * k7) % p)
    : (a0 + 65536 * a1 + 4294967296 * a2 + 281474976710656 * a3 + 18446744073709551616 * a4 + 1208925819614629174706176 * a5 + 79228162514264337593543950336 * a6 + 5192296858534827628530496329220096 * a7) + (b0 + 65536 * b1 + 4294967296 * b2 + 281474976710656 * b3 + 18446744073709551616 * b4 + 1208925819614629174706176 * b5 + 79228162514264337593543950336 * b6 + 5192296858534827628530496329220096 * b7) = (c0 + 65536 * c1 + 4294967296 * c2 + 281474976710656 * c3 + 18446744073709551616 * c4 + 1208925819614629174706176 * c5 + 79228162514264337593543950336 * c6 + 5192296858534827628530496329220096 * c7) + 340282366920938463463374607431768211456 * k7 := by
  have n0 := no_wrap (by unfold p; omega) (by unfold p; omega) e0
  have n1 := no_wrap (by unfold p; omega) (by unfold p; omega) e1
  have n2 := no_wrap (by unfold p; omega) (by unfold p; omega) e2
  have n3 := no_wrap (by unfold p; omega) (by unfold p; omega) e3
  have n4 := no_wrap (by unfold p; omega) (by unfold p; omega) e4
  have n5 := no_wrap (by unfold p; omega) (by unfold p; omega) e5
  have n6 := no_wrap (by unfold p; omega) (by unfold p; omega) e6
  have n7 := no_wrap (by unfold p; omega) (by unfold p; omega) e7
  omega

/-- The value of eight 16-bit limbs is below 2^128, and the limbs are a function of it. -/
theorem limbs_lt (a0 a1 a2 a3 a4 a5 a6 a7 : Nat) (h0 : a0 < 65536) (h1 : a1 < 65536) (h2 : a2 < 65536) (h3 : a3 < 65536) (h4 : a4 < 65536) (h5 : a5 < 65536) (h6 : a6 < 65536) (h7 : a7 < 65536) :
    a0 + 65536 * a1 + 4294967296 * a2 + 281474976710656 * a3 + 18446744073709551616 * a4 + 1208925819614629174706176 * a5 + 79228162514264337593543950336 * a6 + 5192296858534827628530496329220096 * a7 < 340282366920938463463374607431768211456 := by omega

theorem limbs_inj (a0 a1 a2 a3 a4 a5 a6 a7 b0 b1 b2 b3 b4 b5 b6 b7 : Nat)
    (ha0 : a0 < 65536) (ha1 : a1 < 65536) (ha2 : a2 < 65536) (ha3 : a3 < 65536) (ha4 : a4 < 65536) (ha5 : a5 < 65536) (ha6 : a6 < 65536) (ha7 : a7 < 65536)
    (hb0 : b0 < 65536) (hb1 : b1 < 65536) (hb2 : b2 < 65536) (hb3 : b3 < 65536) (hb4 : b4 < 65536) (hb5 : b5 < 65536) (hb6 : b6 < 65536) (hb7 : b7 < 65536)
    (h : a0 + 65536 * a1 + 4294967296 * a2 + 281474976710656 * a3 + 18446744073709551616 * a4 + 1208925819614629174706176 * a5 + 79228162514264337593543950336 * a6 + 5192296858534827628530496329220096 * a7 = b0 + 65536 * b1 + 4294967296 * b2 + 281474976710656 * b3 + 18446744073709551616 * b4 + 1208925819614629174706176 * b5 + 79228162514264337593543950336 * b6 + 5192296858534827628530496329220096 * b7) : a0 = b0 ∧ a1 = b1 ∧ a2 = b2 ∧ a3 = b3 ∧ a4 = b4 ∧ a5 = b5 ∧ a6 = b6 ∧ a7 = b7 := by omega

/-- ADD: `x + y = z + 2^128 flag`, so `(z, flag)` is the spec result and unique. -/
theorem add_spec (X Y Z f : Nat) (hz : Z < 2^128) (hf : f ≤ 1) (h : X + Y = Z + 2^128 * f) :
    Z = (X + Y) % 2^128 ∧ f = (X + Y) / 2^128 := by omega

/-- SUB: the chain `(z, y, x)` gives `z + y = x + 2^128 flag`: `z = x - y mod 2^128` and
`flag = [x < y]` (the borrow). -/
theorem sub_spec (X Y Z f : Nat) (hx : X < 2^128) (hy : Y < 2^128) (hz : Z < 2^128) (hf : f ≤ 1)
    (h : Z + Y = X + 2^128 * f) : (f = 1 ↔ X < Y) ∧ Z = (X + 2^128 - Y) % 2^128 := by omega

/-- LT: the chain `(w, y, x)` with `w` ranged gives `c_7 = [x < y]`; `z = (c_7, 0, ..)`. -/
theorem lt_spec (X Y Wd c : Nat) (hx : X < 2^128) (hy : Y < 2^128) (hw : Wd < 2^128) (hc : c ≤ 1)
    (h : Wd + Y = X + 2^128 * c) : c = 1 ↔ X < Y := by omega

/-- Uniqueness of `(z, flag)` for ADD, SUB and LT: two solutions of the chain relation with the
same operands are equal. -/
theorem chain_unique (S Z Z' f f' : Nat) (hz : Z < 2^128) (hz' : Z' < 2^128) (hf : f ≤ 1) (hf' : f' ≤ 1)
    (h : S = Z + 2^128 * f) (h' : S = Z' + 2^128 * f') : Z = Z' ∧ f = f' := by omega

/-- A MUL column: `col + cr_(k-1) = ob + 256 cr_k` with at most 16 byte products in `col`,
`ob < 256` and carries below 2^16 has both sides below `p`. -/
theorem mul_column_no_wrap (col cin ob cout : Nat) (hcol : col ≤ 16 * 255 * 255) (hin : cin < 65536)
    (hob : ob < 256) (hout : cout < 65536) (e : (col + cin) % p = (ob + 256 * cout) % p) :
    col + cin = ob + 256 * cout :=
  no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem byte_prod_le (a b : Nat) (ha : a < 256) (hb : b < 256) : a * b ≤ 255 * 255 :=
  Nat.mul_le_mul (by omega) (by omega)

/-- The output byte and the carry of a column are a function of the column sum and the carry in. -/
theorem mul_step_unique (s ob cout ob' cout' : Nat) (hob : ob < 256) (hob' : ob' < 256)
    (h : s = ob + 256 * cout) (h' : s = ob' + 256 * cout') : ob = ob' ∧ cout = cout' := by omega

/-- The whole MUL chain: equal column sums give equal output bytes and carries (32 columns, the
last with no carry out). -/
theorem mul_chain_unique (col : Nat → Nat) (ob cr ob' cr' : Nat → Nat)
    (hob : ∀ k, ob k < 256) (hob' : ∀ k, ob' k < 256)
    (h : ∀ k, col k + (if k = 0 then 0 else cr (k - 1)) = ob k + 256 * cr k)
    (h' : ∀ k, col k + (if k = 0 then 0 else cr' (k - 1)) = ob' k + 256 * cr' k) :
    ∀ k, ob k = ob' k ∧ cr k = cr' k := by
  intro k
  induction k with
  | zero =>
    have a := h 0; have b := h' 0; have c := hob 0; have d := hob' 0
    simp at a b
    omega
  | succ n ih =>
    have a := h (n + 1); have b := h' (n + 1); have c := hob (n + 1); have d := hob' (n + 1)
    simp at a b
    omega

/-- MUL flag: `flag = 0` forces each high byte to 0 (`(1 - flag) h_j = 0`); `flag = 1` needs
`S minv = 1` for the high byte sum `S < p`. So the flag is `[S != 0]`. -/
theorem mul_flag_unique (S minv : Nat) (hS : S < p) :
    (S * minv % p = 1 % p → S ≠ 0) := by
  intro h hs
  subst hs
  simp [p] at h

/-- EQ under the premise that `p` has no zero divisors: forward `z (x_i - y_i) = 0`, reverse
`sum_i (x_i - y_i) inv_i + z = 1` give `z = 1 ↔ x = y` (one limb shown; the 8-limb form is the
conjunction over limbs, `d = 0` for every limb iff `x = y` by `limbs_inj`). -/
theorem eq_spec (prime : ∀ a b : Nat, a < p → b < p → a * b % p = 0 → a = 0 ∨ b = 0)
    (z d r : Nat) (hz : z < p) (hd : d < p)
    (fwd : z * d % p = 0) (rev : (r + z) % p = 1 % p) (hr : d = 0 → r % p = 0) :
    (d = 0 → z = 1) ∧ (d ≠ 0 → z = 0) := by
  constructor
  · intro h0
    have := hr h0
    have e : (r + z) % p = z % p := by
      rw [Nat.add_mod, this, Nat.zero_add, Nat.mod_mod]
    rw [e, Nat.mod_eq_of_lt hz] at rev
    simpa [p] using rev
  · intro hne
    rcases prime z d hz hd fwd with h | h
    · exact h
    · exact absurd h hne


/-- The MUL schoolbook over bytes: the product of the byte reconstructions is the sum of the
column sums `col_k = sum_(i+j=k) x_i y_j` times `256^k` (proved by `grind`'s ring normaliser). -/
theorem schoolbook (x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 x10 x11 x12 x13 x14 x15 y0 y1 y2 y3 y4 y5 y6 y7 y8 y9 y10 y11 y12 y13 y14 y15 : Nat) : (1 * x0 + 256 * x1 + 65536 * x2 + 16777216 * x3 + 4294967296 * x4 + 1099511627776 * x5 + 281474976710656 * x6 + 72057594037927936 * x7 + 18446744073709551616 * x8 + 4722366482869645213696 * x9 + 1208925819614629174706176 * x10 + 309485009821345068724781056 * x11 + 79228162514264337593543950336 * x12 + 20282409603651670423947251286016 * x13 + 5192296858534827628530496329220096 * x14 + 1329227995784915872903807060280344576 * x15) * (1 * y0 + 256 * y1 + 65536 * y2 + 16777216 * y3 + 4294967296 * y4 + 1099511627776 * y5 + 281474976710656 * y6 + 72057594037927936 * y7 + 18446744073709551616 * y8 + 4722366482869645213696 * y9 + 1208925819614629174706176 * y10 + 309485009821345068724781056 * y11 + 79228162514264337593543950336 * y12 + 20282409603651670423947251286016 * y13 + 5192296858534827628530496329220096 * y14 + 1329227995784915872903807060280344576 * y15) = 1 * (x0 * y0) + 256 * (x0 * y1 + x1 * y0) + 65536 * (x0 * y2 + x1 * y1 + x2 * y0) + 16777216 * (x0 * y3 + x1 * y2 + x2 * y1 + x3 * y0) + 4294967296 * (x0 * y4 + x1 * y3 + x2 * y2 + x3 * y1 + x4 * y0) + 1099511627776 * (x0 * y5 + x1 * y4 + x2 * y3 + x3 * y2 + x4 * y1 + x5 * y0) + 281474976710656 * (x0 * y6 + x1 * y5 + x2 * y4 + x3 * y3 + x4 * y2 + x5 * y1 + x6 * y0) + 72057594037927936 * (x0 * y7 + x1 * y6 + x2 * y5 + x3 * y4 + x4 * y3 + x5 * y2 + x6 * y1 + x7 * y0) + 18446744073709551616 * (x0 * y8 + x1 * y7 + x2 * y6 + x3 * y5 + x4 * y4 + x5 * y3 + x6 * y2 + x7 * y1 + x8 * y0) + 4722366482869645213696 * (x0 * y9 + x1 * y8 + x2 * y7 + x3 * y6 + x4 * y5 + x5 * y4 + x6 * y3 + x7 * y2 + x8 * y1 + x9 * y0) + 1208925819614629174706176 * (x0 * y10 + x1 * y9 + x2 * y8 + x3 * y7 + x4 * y6 + x5 * y5 + x6 * y4 + x7 * y3 + x8 * y2 + x9 * y1 + x10 * y0) + 309485009821345068724781056 * (x0 * y11 + x1 * y10 + x2 * y9 + x3 * y8 + x4 * y7 + x5 * y6 + x6 * y5 + x7 * y4 + x8 * y3 + x9 * y2 + x10 * y1 + x11 * y0) + 79228162514264337593543950336 * (x0 * y12 + x1 * y11 + x2 * y10 + x3 * y9 + x4 * y8 + x5 * y7 + x6 * y6 + x7 * y5 + x8 * y4 + x9 * y3 + x10 * y2 + x11 * y1 + x12 * y0) + 20282409603651670423947251286016 * (x0 * y13 + x1 * y12 + x2 * y11 + x3 * y10 + x4 * y9 + x5 * y8 + x6 * y7 + x7 * y6 + x8 * y5 + x9 * y4 + x10 * y3 + x11 * y2 + x12 * y1 + x13 * y0) + 5192296858534827628530496329220096 * (x0 * y14 + x1 * y13 + x2 * y12 + x3 * y11 + x4 * y10 + x5 * y9 + x6 * y8 + x7 * y7 + x8 * y6 + x9 * y5 + x10 * y4 + x11 * y3 + x12 * y2 + x13 * y1 + x14 * y0) + 1329227995784915872903807060280344576 * (x0 * y15 + x1 * y14 + x2 * y13 + x3 * y12 + x4 * y11 + x5 * y10 + x6 * y9 + x7 * y8 + x8 * y7 + x9 * y6 + x10 * y5 + x11 * y4 + x12 * y3 + x13 * y2 + x14 * y1 + x15 * y0) + 340282366920938463463374607431768211456 * (x1 * y15 + x2 * y14 + x3 * y13 + x4 * y12 + x5 * y11 + x6 * y10 + x7 * y9 + x8 * y8 + x9 * y7 + x10 * y6 + x11 * y5 + x12 * y4 + x13 * y3 + x14 * y2 + x15 * y1) + 87112285931760246646623899502532662132736 * (x2 * y15 + x3 * y14 + x4 * y13 + x5 * y12 + x6 * y11 + x7 * y10 + x8 * y9 + x9 * y8 + x10 * y7 + x11 * y6 + x12 * y5 + x13 * y4 + x14 * y3 + x15 * y2) + 22300745198530623141535718272648361505980416 * (x3 * y15 + x4 * y14 + x5 * y13 + x6 * y12 + x7 * y11 + x8 * y10 + x9 * y9 + x10 * y8 + x11 * y7 + x12 * y6 + x13 * y5 + x14 * y4 + x15 * y3) + 5708990770823839524233143877797980545530986496 * (x4 * y15 + x5 * y14 + x6 * y13 + x7 * y12 + x8 * y11 + x9 * y10 + x10 * y9 + x11 * y8 + x12 * y7 + x13 * y6 + x14 * y5 + x15 * y4) + 1461501637330902918203684832716283019655932542976 * (x5 * y15 + x6 * y14 + x7 * y13 + x8 * y12 + x9 * y11 + x10 * y10 + x11 * y9 + x12 * y8 + x13 * y7 + x14 * y6 + x15 * y5) + 374144419156711147060143317175368453031918731001856 * (x6 * y15 + x7 * y14 + x8 * y13 + x9 * y12 + x10 * y11 + x11 * y10 + x12 * y9 + x13 * y8 + x14 * y7 + x15 * y6) + 95780971304118053647396689196894323976171195136475136 * (x7 * y15 + x8 * y14 + x9 * y13 + x10 * y12 + x11 * y11 + x12 * y10 + x13 * y9 + x14 * y8 + x15 * y7) + 24519928653854221733733552434404946937899825954937634816 * (x8 * y15 + x9 * y14 + x10 * y13 + x11 * y12 + x12 * y11 + x13 * y10 + x14 * y9 + x15 * y8) + 6277101735386680763835789423207666416102355444464034512896 * (x9 * y15 + x10 * y14 + x11 * y13 + x12 * y12 + x13 * y11 + x14 * y10 + x15 * y9) + 1606938044258990275541962092341162602522202993782792835301376 * (x10 * y15 + x11 * y14 + x12 * y13 + x13 * y12 + x14 * y11 + x15 * y10) + 411376139330301510538742295639337626245683966408394965837152256 * (x11 * y15 + x12 * y14 + x13 * y13 + x14 * y12 + x15 * y11) + 105312291668557186697918027683670432318895095400549111254310977536 * (x12 * y15 + x13 * y14 + x14 * y13 + x15 * y12) + 26959946667150639794667015087019630673637144422540572481103610249216 * (x13 * y15 + x14 * y14 + x15 * y13) + 6901746346790563787434755862277025452451108972170386555162524223799296 * (x14 * y15 + x15 * y14) + 1766847064778384329583297500742918515827483896875618958121606201292619776 * (x15 * y15) := by
  grind

/-- MUL: with the columns `col_k` (atoms here; `schoolbook` gives `P = sum_k 256^k col_k`), the
integer column chain gives `P = Z + 2^128 H` for the z bytes `Z` and the high bytes `H`, so
`Z = P mod 2^128` and `H = P / 2^128`; the flag is `[H != 0]` (`mul_flag_unique`), so
`flag = [P >= 2^128]`. -/
theorem mul_spec (P : Nat) (c0 c1 c2 c3 c4 c5 c6 c7 c8 c9 c10 c11 c12 c13 c14 c15 c16 c17 c18 c19 c20 c21 c22 c23 c24 c25 c26 c27 c28 c29 c30 : Nat) (o0 o1 o2 o3 o4 o5 o6 o7 o8 o9 o10 o11 o12 o13 o14 o15 o16 o17 o18 o19 o20 o21 o22 o23 o24 o25 o26 o27 o28 o29 o30 o31 : Nat) (r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14 r15 r16 r17 r18 r19 r20 r21 r22 r23 r24 r25 r26 r27 r28 r29 r30 : Nat)
    (hP : P = 1 * c0 + 256 * c1 + 65536 * c2 + 16777216 * c3 + 4294967296 * c4 + 1099511627776 * c5 + 281474976710656 * c6 + 72057594037927936 * c7 + 18446744073709551616 * c8 + 4722366482869645213696 * c9 + 1208925819614629174706176 * c10 + 309485009821345068724781056 * c11 + 79228162514264337593543950336 * c12 + 20282409603651670423947251286016 * c13 + 5192296858534827628530496329220096 * c14 + 1329227995784915872903807060280344576 * c15 + 340282366920938463463374607431768211456 * c16 + 87112285931760246646623899502532662132736 * c17 + 22300745198530623141535718272648361505980416 * c18 + 5708990770823839524233143877797980545530986496 * c19 + 1461501637330902918203684832716283019655932542976 * c20 + 374144419156711147060143317175368453031918731001856 * c21 + 95780971304118053647396689196894323976171195136475136 * c22 + 24519928653854221733733552434404946937899825954937634816 * c23 + 6277101735386680763835789423207666416102355444464034512896 * c24 + 1606938044258990275541962092341162602522202993782792835301376 * c25 + 411376139330301510538742295639337626245683966408394965837152256 * c26 + 105312291668557186697918027683670432318895095400549111254310977536 * c27 + 26959946667150639794667015087019630673637144422540572481103610249216 * c28 + 6901746346790563787434755862277025452451108972170386555162524223799296 * c29 + 1766847064778384329583297500742918515827483896875618958121606201292619776 * c30)
        (ho0 : o0 < 256)
    (ho1 : o1 < 256)
    (ho2 : o2 < 256)
    (ho3 : o3 < 256)
    (ho4 : o4 < 256)
    (ho5 : o5 < 256)
    (ho6 : o6 < 256)
    (ho7 : o7 < 256)
    (ho8 : o8 < 256)
    (ho9 : o9 < 256)
    (ho10 : o10 < 256)
    (ho11 : o11 < 256)
    (ho12 : o12 < 256)
    (ho13 : o13 < 256)
    (ho14 : o14 < 256)
    (ho15 : o15 < 256)
    (ho16 : o16 < 256)
    (ho17 : o17 < 256)
    (ho18 : o18 < 256)
    (ho19 : o19 < 256)
    (ho20 : o20 < 256)
    (ho21 : o21 < 256)
    (ho22 : o22 < 256)
    (ho23 : o23 < 256)
    (ho24 : o24 < 256)
    (ho25 : o25 < 256)
    (ho26 : o26 < 256)
    (ho27 : o27 < 256)
    (ho28 : o28 < 256)
    (ho29 : o29 < 256)
    (ho30 : o30 < 256)
    (ho31 : o31 < 256)
    (e0 : c0 = o0 + 256 * r0)
    (e1 : c1 + r0 = o1 + 256 * r1)
    (e2 : c2 + r1 = o2 + 256 * r2)
    (e3 : c3 + r2 = o3 + 256 * r3)
    (e4 : c4 + r3 = o4 + 256 * r4)
    (e5 : c5 + r4 = o5 + 256 * r5)
    (e6 : c6 + r5 = o6 + 256 * r6)
    (e7 : c7 + r6 = o7 + 256 * r7)
    (e8 : c8 + r7 = o8 + 256 * r8)
    (e9 : c9 + r8 = o9 + 256 * r9)
    (e10 : c10 + r9 = o10 + 256 * r10)
    (e11 : c11 + r10 = o11 + 256 * r11)
    (e12 : c12 + r11 = o12 + 256 * r12)
    (e13 : c13 + r12 = o13 + 256 * r13)
    (e14 : c14 + r13 = o14 + 256 * r14)
    (e15 : c15 + r14 = o15 + 256 * r15)
    (e16 : c16 + r15 = o16 + 256 * r16)
    (e17 : c17 + r16 = o17 + 256 * r17)
    (e18 : c18 + r17 = o18 + 256 * r18)
    (e19 : c19 + r18 = o19 + 256 * r19)
    (e20 : c20 + r19 = o20 + 256 * r20)
    (e21 : c21 + r20 = o21 + 256 * r21)
    (e22 : c22 + r21 = o22 + 256 * r22)
    (e23 : c23 + r22 = o23 + 256 * r23)
    (e24 : c24 + r23 = o24 + 256 * r24)
    (e25 : c25 + r24 = o25 + 256 * r25)
    (e26 : c26 + r25 = o26 + 256 * r26)
    (e27 : c27 + r26 = o27 + 256 * r27)
    (e28 : c28 + r27 = o28 + 256 * r28)
    (e29 : c29 + r28 = o29 + 256 * r29)
    (e30 : c30 + r29 = o30 + 256 * r30)
    (e31 : 0 + r30 = o31) :
    (1 * o0 + 256 * o1 + 65536 * o2 + 16777216 * o3 + 4294967296 * o4 + 1099511627776 * o5 + 281474976710656 * o6 + 72057594037927936 * o7 + 18446744073709551616 * o8 + 4722366482869645213696 * o9 + 1208925819614629174706176 * o10 + 309485009821345068724781056 * o11 + 79228162514264337593543950336 * o12 + 20282409603651670423947251286016 * o13 + 5192296858534827628530496329220096 * o14 + 1329227995784915872903807060280344576 * o15) = P % 2^128 ∧ (1 * o16 + 256 * o17 + 65536 * o18 + 16777216 * o19 + 4294967296 * o20 + 1099511627776 * o21 + 281474976710656 * o22 + 72057594037927936 * o23 + 18446744073709551616 * o24 + 4722366482869645213696 * o25 + 1208925819614629174706176 * o26 + 309485009821345068724781056 * o27 + 79228162514264337593543950336 * o28 + 20282409603651670423947251286016 * o29 + 5192296858534827628530496329220096 * o30 + 1329227995784915872903807060280344576 * o31) = P / 2^128 := by
  omega

/-! ### BabyBear is prime (trial division up to 44,870, checked by the kernel) -/

def chkFrom (a : Nat) : Nat → Bool
  | 0 => true
  | n + 1 => p % (a + n) != 0 && chkFrom a n

theorem chkFrom_sound (a : Nat) : ∀ n, chkFrom a n = true → ∀ d, a ≤ d → d < a + n → p % d ≠ 0 := by
  intro n
  induction n with
  | zero => intro _ d h1 h2; omega
  | succ n ih =>
    intro h d h1 h2
    simp only [chkFrom, Bool.and_eq_true, bne_iff_ne, ne_eq] at h
    by_cases hd : d = a + n
    · subst hd; exact h.1
    · exact ih h.2 d h1 (by omega)

theorem chk_small : chkFrom 2 44869 = true := by decide +kernel

theorem dvd_p (d : Nat) (h : d ∣ p) : d = 1 ∨ d = p := by
  obtain ⟨e, he⟩ := h
  have hp : p = 2013265921 := rfl
  have nsd := chkFrom_sound 2 44869 chk_small
  by_cases d0 : d = 0
  · subst d0; simp [hp] at he
  by_cases d1 : d = 1
  · exact Or.inl d1
  by_cases e1 : e = 1
  · subst e1; right; omega
  have e0 : e ≠ 0 := by intro h0; subst h0; simp [hp] at he
  exfalso
  by_cases hd : d < 44871
  · exact nsd d (by omega) (by omega) (by rw [he]; simp)
  · by_cases he' : e < 44871
    · exact nsd e (by omega) (by omega) (by rw [he, Nat.mul_comm]; simp)
    · have : 44871 * 44871 ≤ d * e := Nat.mul_le_mul (by omega) (by omega)
      omega

/-- `p` is prime: no zero divisors among the representatives. -/
theorem no_zero_div (a b : Nat) (ha : a < p) (hb : b < p) (h : a * b % p = 0) : a = 0 ∨ b = 0 := by
  by_cases a0 : a = 0
  · exact Or.inl a0
  right
  have hcop : Nat.Coprime p a := by
    rcases dvd_p (Nat.gcd p a) (Nat.gcd_dvd_left p a) with g | g
    · exact g
    · exfalso
      have := Nat.le_of_dvd (by omega) (Nat.gcd_dvd_right p a)
      omega
  exact Nat.eq_zero_of_dvd_of_lt (hcop.dvd_of_dvd_mul_left (Nat.dvd_of_mod_eq_zero h)) hb

/-- `assert_bool(c)`: `c (c - 1) = 0` in the field, i.e. `c c = c`, gives `c` in {0, 1}. -/
theorem field_bool (c : Nat) (hc : c < p) (h : c * c % p = c % p) : c ≤ 1 := by
  have h2 : (c * c - c) % p = 0 := Nat.sub_mod_eq_zero_of_mod_eq h
  rw [show c * c - c = c * (c - 1) by rw [Nat.mul_sub_one]] at h2
  rcases no_zero_div c (c - 1) hc (by omega) h2 with h | h <;> omega

/-- The limb-to-byte link of the AIR: a limb equal in the field to two ranged bytes is that
integer, so it is below 2^16. -/
theorem byte_link (x b0 b1 : Nat) (hx : x < p) (h0 : b0 < 256) (h1 : b1 < 256)
    (h : x % p = (b0 + 256 * b1) % p) : x = b0 + 256 * b1 :=
  no_wrap hx (by unfold p; omega) h

theorem link_lt (x b0 b1 : Nat) (h0 : b0 < 256) (h1 : b1 < 256) (h : x = b0 + 256 * b1) :
    x < 65536 := by omega

theorem zero_of_mod (z : Nat) (hz : z < p) (h : z % p = 0) : z = 0 := by
  rw [Nat.mod_eq_of_lt hz] at h; exact h

theorem eq_of_one_mul (z x y : Nat) (hz : z = 1) (hx : x < p) (hy : y < p)
    (h : z * x % p = z * y % p) : x = y := by
  subst hz; simp only [Nat.one_mul] at h; exact no_wrap hx hy h

theorem rev_zero (s : Nat) (h : (s + 0) % p = (s + 1) % p) : False := by
  unfold p at h; omega

theorem high_zero (f o : Nat) (h : (1 + p - f) * o % p = 0) (ho : o < 256) : f = 0 → o = 0 := by
  intro h0; subst h0; unfold p at h; omega

theorem mf_flag (f S minv : Nat) (h : f * (S * minv) % p = f % p) : f = 1 → S ≠ 0 := by
  intro h1 hS; subst h1; subst hS; unfold p at h; simp at h

/-! ### Composed per-op theorems: a full AIR row of each op to the spec result

Each theorem takes the constraints of one provider row with that op's selector set (the one-hot
selectors substituted), every cell a field representative below `p`, and the range lookups as
bounds on the ranged cells (bytes below 256, `w` and MUL carries below 2^16). The conclusion is
the op table of `alu.rs` on the integers, so `(z, flag)` is unique. -/

set_option maxHeartbeats 4000000 in
/-- ADD row: `z = x + y mod 2^128`, `flag = [x + y >= 2^128]`. -/
theorem add_row
    (x0 x1 x2 x3 x4 x5 x6 x7 : Nat) (xb0 xb1 xb2 xb3 xb4 xb5 xb6 xb7 xb8 xb9 xb10 xb11 xb12 xb13 xb14 xb15 : Nat)
    (hx0 : x0 < p)
    (hx1 : x1 < p)
    (hx2 : x2 < p)
    (hx3 : x3 < p)
    (hx4 : x4 < p)
    (hx5 : x5 < p)
    (hx6 : x6 < p)
    (hx7 : x7 < p)
    (hxb0 : xb0 < 256)
    (hxb1 : xb1 < 256)
    (hxb2 : xb2 < 256)
    (hxb3 : xb3 < 256)
    (hxb4 : xb4 < 256)
    (hxb5 : xb5 < 256)
    (hxb6 : xb6 < 256)
    (hxb7 : xb7 < 256)
    (hxb8 : xb8 < 256)
    (hxb9 : xb9 < 256)
    (hxb10 : xb10 < 256)
    (hxb11 : xb11 < 256)
    (hxb12 : xb12 < 256)
    (hxb13 : xb13 < 256)
    (hxb14 : xb14 < 256)
    (hxb15 : xb15 < 256)
    (lx0 : x0 % p = (xb0 + 256 * xb1) % p)
    (lx1 : x1 % p = (xb2 + 256 * xb3) % p)
    (lx2 : x2 % p = (xb4 + 256 * xb5) % p)
    (lx3 : x3 % p = (xb6 + 256 * xb7) % p)
    (lx4 : x4 % p = (xb8 + 256 * xb9) % p)
    (lx5 : x5 % p = (xb10 + 256 * xb11) % p)
    (lx6 : x6 % p = (xb12 + 256 * xb13) % p)
    (lx7 : x7 % p = (xb14 + 256 * xb15) % p)
    (y0 y1 y2 y3 y4 y5 y6 y7 : Nat) (yb0 yb1 yb2 yb3 yb4 yb5 yb6 yb7 yb8 yb9 yb10 yb11 yb12 yb13 yb14 yb15 : Nat)
    (hy0 : y0 < p)
    (hy1 : y1 < p)
    (hy2 : y2 < p)
    (hy3 : y3 < p)
    (hy4 : y4 < p)
    (hy5 : y5 < p)
    (hy6 : y6 < p)
    (hy7 : y7 < p)
    (hyb0 : yb0 < 256)
    (hyb1 : yb1 < 256)
    (hyb2 : yb2 < 256)
    (hyb3 : yb3 < 256)
    (hyb4 : yb4 < 256)
    (hyb5 : yb5 < 256)
    (hyb6 : yb6 < 256)
    (hyb7 : yb7 < 256)
    (hyb8 : yb8 < 256)
    (hyb9 : yb9 < 256)
    (hyb10 : yb10 < 256)
    (hyb11 : yb11 < 256)
    (hyb12 : yb12 < 256)
    (hyb13 : yb13 < 256)
    (hyb14 : yb14 < 256)
    (hyb15 : yb15 < 256)
    (ly0 : y0 % p = (yb0 + 256 * yb1) % p)
    (ly1 : y1 % p = (yb2 + 256 * yb3) % p)
    (ly2 : y2 % p = (yb4 + 256 * yb5) % p)
    (ly3 : y3 % p = (yb6 + 256 * yb7) % p)
    (ly4 : y4 % p = (yb8 + 256 * yb9) % p)
    (ly5 : y5 % p = (yb10 + 256 * yb11) % p)
    (ly6 : y6 % p = (yb12 + 256 * yb13) % p)
    (ly7 : y7 % p = (yb14 + 256 * yb15) % p)
    (z0 z1 z2 z3 z4 z5 z6 z7 : Nat) (zb0 zb1 zb2 zb3 zb4 zb5 zb6 zb7 zb8 zb9 zb10 zb11 zb12 zb13 zb14 zb15 : Nat)
    (hz0 : z0 < p)
    (hz1 : z1 < p)
    (hz2 : z2 < p)
    (hz3 : z3 < p)
    (hz4 : z4 < p)
    (hz5 : z5 < p)
    (hz6 : z6 < p)
    (hz7 : z7 < p)
    (hzb0 : zb0 < 256)
    (hzb1 : zb1 < 256)
    (hzb2 : zb2 < 256)
    (hzb3 : zb3 < 256)
    (hzb4 : zb4 < 256)
    (hzb5 : zb5 < 256)
    (hzb6 : zb6 < 256)
    (hzb7 : zb7 < 256)
    (hzb8 : zb8 < 256)
    (hzb9 : zb9 < 256)
    (hzb10 : zb10 < 256)
    (hzb11 : zb11 < 256)
    (hzb12 : zb12 < 256)
    (hzb13 : zb13 < 256)
    (hzb14 : zb14 < 256)
    (hzb15 : zb15 < 256)
    (lz0 : z0 % p = (zb0 + 256 * zb1) % p)
    (lz1 : z1 % p = (zb2 + 256 * zb3) % p)
    (lz2 : z2 % p = (zb4 + 256 * zb5) % p)
    (lz3 : z3 % p = (zb6 + 256 * zb7) % p)
    (lz4 : z4 % p = (zb8 + 256 * zb9) % p)
    (lz5 : z5 % p = (zb10 + 256 * zb11) % p)
    (lz6 : z6 % p = (zb12 + 256 * zb13) % p)
    (lz7 : z7 % p = (zb14 + 256 * zb15) % p)
    (c0 c1 c2 c3 c4 c5 c6 c7 : Nat)
    (hc0 : c0 < p)
    (hc1 : c1 < p)
    (hc2 : c2 < p)
    (hc3 : c3 < p)
    (hc4 : c4 < p)
    (hc5 : c5 < p)
    (hc6 : c6 < p)
    (hc7 : c7 < p)
    (bc0 : c0 * c0 % p = c0 % p)
    (bc1 : c1 * c1 % p = c1 % p)
    (bc2 : c2 * c2 % p = c2 % p)
    (bc3 : c3 * c3 % p = c3 % p)
    (bc4 : c4 * c4 % p = c4 % p)
    (bc5 : c5 * c5 % p = c5 % p)
    (bc6 : c6 * c6 % p = c6 % p)
    (bc7 : c7 * c7 % p = c7 % p)
    (e0 : (x0 + y0) % p = (z0 + 65536 * c0) % p)
    (e1 : (x1 + y1 + c0) % p = (z1 + 65536 * c1) % p)
    (e2 : (x2 + y2 + c1) % p = (z2 + 65536 * c2) % p)
    (e3 : (x3 + y3 + c2) % p = (z3 + 65536 * c3) % p)
    (e4 : (x4 + y4 + c3) % p = (z4 + 65536 * c4) % p)
    (e5 : (x5 + y5 + c4) % p = (z5 + 65536 * c5) % p)
    (e6 : (x6 + y6 + c5) % p = (z6 + 65536 * c6) % p)
    (e7 : (x7 + y7 + c6) % p = (z7 + 65536 * c7) % p)
    (f : Nat) (hf : f < p) (fl : f % p = c7 % p) :
    (z0 + 65536 * z1 + 4294967296 * z2 + 281474976710656 * z3 + 18446744073709551616 * z4 + 1208925819614629174706176 * z5 + 79228162514264337593543950336 * z6 + 5192296858534827628530496329220096 * z7) = ((x0 + 65536 * x1 + 4294967296 * x2 + 281474976710656 * x3 + 18446744073709551616 * x4 + 1208925819614629174706176 * x5 + 79228162514264337593543950336 * x6 + 5192296858534827628530496329220096 * x7) + (y0 + 65536 * y1 + 4294967296 * y2 + 281474976710656 * y3 + 18446744073709551616 * y4 + 1208925819614629174706176 * y5 + 79228162514264337593543950336 * y6 + 5192296858534827628530496329220096 * y7)) % 2^128 ∧ f = ((x0 + 65536 * x1 + 4294967296 * x2 + 281474976710656 * x3 + 18446744073709551616 * x4 + 1208925819614629174706176 * x5 + 79228162514264337593543950336 * x6 + 5192296858534827628530496329220096 * x7) + (y0 + 65536 * y1 + 4294967296 * y2 + 281474976710656 * y3 + 18446744073709551616 * y4 + 1208925819614629174706176 * y5 + 79228162514264337593543950336 * y6 + 5192296858534827628530496329220096 * y7)) / 2^128 := by
  have kx0 := byte_link x0 xb0 xb1 hx0 hxb0 hxb1 lx0
  have kx1 := byte_link x1 xb2 xb3 hx1 hxb2 hxb3 lx1
  have kx2 := byte_link x2 xb4 xb5 hx2 hxb4 hxb5 lx2
  have kx3 := byte_link x3 xb6 xb7 hx3 hxb6 hxb7 lx3
  have kx4 := byte_link x4 xb8 xb9 hx4 hxb8 hxb9 lx4
  have kx5 := byte_link x5 xb10 xb11 hx5 hxb10 hxb11 lx5
  have kx6 := byte_link x6 xb12 xb13 hx6 hxb12 hxb13 lx6
  have kx7 := byte_link x7 xb14 xb15 hx7 hxb14 hxb15 lx7
  have gx0 := link_lt x0 xb0 xb1 hxb0 hxb1 kx0
  have gx1 := link_lt x1 xb2 xb3 hxb2 hxb3 kx1
  have gx2 := link_lt x2 xb4 xb5 hxb4 hxb5 kx2
  have gx3 := link_lt x3 xb6 xb7 hxb6 hxb7 kx3
  have gx4 := link_lt x4 xb8 xb9 hxb8 hxb9 kx4
  have gx5 := link_lt x5 xb10 xb11 hxb10 hxb11 kx5
  have gx6 := link_lt x6 xb12 xb13 hxb12 hxb13 kx6
  have gx7 := link_lt x7 xb14 xb15 hxb14 hxb15 kx7
  have ky0 := byte_link y0 yb0 yb1 hy0 hyb0 hyb1 ly0
  have ky1 := byte_link y1 yb2 yb3 hy1 hyb2 hyb3 ly1
  have ky2 := byte_link y2 yb4 yb5 hy2 hyb4 hyb5 ly2
  have ky3 := byte_link y3 yb6 yb7 hy3 hyb6 hyb7 ly3
  have ky4 := byte_link y4 yb8 yb9 hy4 hyb8 hyb9 ly4
  have ky5 := byte_link y5 yb10 yb11 hy5 hyb10 hyb11 ly5
  have ky6 := byte_link y6 yb12 yb13 hy6 hyb12 hyb13 ly6
  have ky7 := byte_link y7 yb14 yb15 hy7 hyb14 hyb15 ly7
  have gy0 := link_lt y0 yb0 yb1 hyb0 hyb1 ky0
  have gy1 := link_lt y1 yb2 yb3 hyb2 hyb3 ky1
  have gy2 := link_lt y2 yb4 yb5 hyb4 hyb5 ky2
  have gy3 := link_lt y3 yb6 yb7 hyb6 hyb7 ky3
  have gy4 := link_lt y4 yb8 yb9 hyb8 hyb9 ky4
  have gy5 := link_lt y5 yb10 yb11 hyb10 hyb11 ky5
  have gy6 := link_lt y6 yb12 yb13 hyb12 hyb13 ky6
  have gy7 := link_lt y7 yb14 yb15 hyb14 hyb15 ky7
  have kz0 := byte_link z0 zb0 zb1 hz0 hzb0 hzb1 lz0
  have kz1 := byte_link z1 zb2 zb3 hz1 hzb2 hzb3 lz1
  have kz2 := byte_link z2 zb4 zb5 hz2 hzb4 hzb5 lz2
  have kz3 := byte_link z3 zb6 zb7 hz3 hzb6 hzb7 lz3
  have kz4 := byte_link z4 zb8 zb9 hz4 hzb8 hzb9 lz4
  have kz5 := byte_link z5 zb10 zb11 hz5 hzb10 hzb11 lz5
  have kz6 := byte_link z6 zb12 zb13 hz6 hzb12 hzb13 lz6
  have kz7 := byte_link z7 zb14 zb15 hz7 hzb14 hzb15 lz7
  have gz0 := link_lt z0 zb0 zb1 hzb0 hzb1 kz0
  have gz1 := link_lt z1 zb2 zb3 hzb2 hzb3 kz1
  have gz2 := link_lt z2 zb4 zb5 hzb4 hzb5 kz2
  have gz3 := link_lt z3 zb6 zb7 hzb6 hzb7 kz3
  have gz4 := link_lt z4 zb8 zb9 hzb8 hzb9 kz4
  have gz5 := link_lt z5 zb10 zb11 hzb10 hzb11 kz5
  have gz6 := link_lt z6 zb12 zb13 hzb12 hzb13 kz6
  have gz7 := link_lt z7 zb14 zb15 hzb14 hzb15 kz7
  have kc0 := field_bool c0 hc0 bc0
  have kc1 := field_bool c1 hc1 bc1
  have kc2 := field_bool c2 hc2 bc2
  have kc3 := field_bool c3 hc3 bc3
  have kc4 := field_bool c4 hc4 bc4
  have kc5 := field_bool c5 hc5 bc5
  have kc6 := field_bool c6 hc6 bc6
  have kc7 := field_bool c7 hc7 bc7
  have ch := chain_sound x0 x1 x2 x3 x4 x5 x6 x7 y0 y1 y2 y3 y4 y5 y6 y7 z0 z1 z2 z3 z4 z5 z6 z7 c0 c1 c2 c3 c4 c5 c6 c7 gx0 gx1 gx2 gx3 gx4 gx5 gx6 gx7 gy0 gy1 gy2 gy3 gy4 gy5 gy6 gy7 gz0 gz1 gz2 gz3 gz4 gz5 gz6 gz7 kc0 kc1 kc2 kc3 kc4 kc5 kc6 kc7 e0 e1 e2 e3 e4 e5 e6 e7
  have nf := no_wrap hf hc7 fl
  clear e0 e1 e2 e3 e4 e5 e6 e7 bc0 bc1 bc2 bc3 bc4 bc5 bc6 bc7 fl lx0 lx1 lx2 lx3 lx4 lx5 lx6 lx7 ly0 ly1 ly2 ly3 ly4 ly5 ly6 ly7 lz0 lz1 lz2 lz3 lz4 lz5 lz6 lz7
  omega

set_option maxHeartbeats 4000000 in
/-- SUB row (chain `(z, y, x)`): `z = x - y mod 2^128`, `flag = [x < y]`. -/
theorem sub_row
    (x0 x1 x2 x3 x4 x5 x6 x7 : Nat) (xb0 xb1 xb2 xb3 xb4 xb5 xb6 xb7 xb8 xb9 xb10 xb11 xb12 xb13 xb14 xb15 : Nat)
    (hx0 : x0 < p)
    (hx1 : x1 < p)
    (hx2 : x2 < p)
    (hx3 : x3 < p)
    (hx4 : x4 < p)
    (hx5 : x5 < p)
    (hx6 : x6 < p)
    (hx7 : x7 < p)
    (hxb0 : xb0 < 256)
    (hxb1 : xb1 < 256)
    (hxb2 : xb2 < 256)
    (hxb3 : xb3 < 256)
    (hxb4 : xb4 < 256)
    (hxb5 : xb5 < 256)
    (hxb6 : xb6 < 256)
    (hxb7 : xb7 < 256)
    (hxb8 : xb8 < 256)
    (hxb9 : xb9 < 256)
    (hxb10 : xb10 < 256)
    (hxb11 : xb11 < 256)
    (hxb12 : xb12 < 256)
    (hxb13 : xb13 < 256)
    (hxb14 : xb14 < 256)
    (hxb15 : xb15 < 256)
    (lx0 : x0 % p = (xb0 + 256 * xb1) % p)
    (lx1 : x1 % p = (xb2 + 256 * xb3) % p)
    (lx2 : x2 % p = (xb4 + 256 * xb5) % p)
    (lx3 : x3 % p = (xb6 + 256 * xb7) % p)
    (lx4 : x4 % p = (xb8 + 256 * xb9) % p)
    (lx5 : x5 % p = (xb10 + 256 * xb11) % p)
    (lx6 : x6 % p = (xb12 + 256 * xb13) % p)
    (lx7 : x7 % p = (xb14 + 256 * xb15) % p)
    (y0 y1 y2 y3 y4 y5 y6 y7 : Nat) (yb0 yb1 yb2 yb3 yb4 yb5 yb6 yb7 yb8 yb9 yb10 yb11 yb12 yb13 yb14 yb15 : Nat)
    (hy0 : y0 < p)
    (hy1 : y1 < p)
    (hy2 : y2 < p)
    (hy3 : y3 < p)
    (hy4 : y4 < p)
    (hy5 : y5 < p)
    (hy6 : y6 < p)
    (hy7 : y7 < p)
    (hyb0 : yb0 < 256)
    (hyb1 : yb1 < 256)
    (hyb2 : yb2 < 256)
    (hyb3 : yb3 < 256)
    (hyb4 : yb4 < 256)
    (hyb5 : yb5 < 256)
    (hyb6 : yb6 < 256)
    (hyb7 : yb7 < 256)
    (hyb8 : yb8 < 256)
    (hyb9 : yb9 < 256)
    (hyb10 : yb10 < 256)
    (hyb11 : yb11 < 256)
    (hyb12 : yb12 < 256)
    (hyb13 : yb13 < 256)
    (hyb14 : yb14 < 256)
    (hyb15 : yb15 < 256)
    (ly0 : y0 % p = (yb0 + 256 * yb1) % p)
    (ly1 : y1 % p = (yb2 + 256 * yb3) % p)
    (ly2 : y2 % p = (yb4 + 256 * yb5) % p)
    (ly3 : y3 % p = (yb6 + 256 * yb7) % p)
    (ly4 : y4 % p = (yb8 + 256 * yb9) % p)
    (ly5 : y5 % p = (yb10 + 256 * yb11) % p)
    (ly6 : y6 % p = (yb12 + 256 * yb13) % p)
    (ly7 : y7 % p = (yb14 + 256 * yb15) % p)
    (z0 z1 z2 z3 z4 z5 z6 z7 : Nat) (zb0 zb1 zb2 zb3 zb4 zb5 zb6 zb7 zb8 zb9 zb10 zb11 zb12 zb13 zb14 zb15 : Nat)
    (hz0 : z0 < p)
    (hz1 : z1 < p)
    (hz2 : z2 < p)
    (hz3 : z3 < p)
    (hz4 : z4 < p)
    (hz5 : z5 < p)
    (hz6 : z6 < p)
    (hz7 : z7 < p)
    (hzb0 : zb0 < 256)
    (hzb1 : zb1 < 256)
    (hzb2 : zb2 < 256)
    (hzb3 : zb3 < 256)
    (hzb4 : zb4 < 256)
    (hzb5 : zb5 < 256)
    (hzb6 : zb6 < 256)
    (hzb7 : zb7 < 256)
    (hzb8 : zb8 < 256)
    (hzb9 : zb9 < 256)
    (hzb10 : zb10 < 256)
    (hzb11 : zb11 < 256)
    (hzb12 : zb12 < 256)
    (hzb13 : zb13 < 256)
    (hzb14 : zb14 < 256)
    (hzb15 : zb15 < 256)
    (lz0 : z0 % p = (zb0 + 256 * zb1) % p)
    (lz1 : z1 % p = (zb2 + 256 * zb3) % p)
    (lz2 : z2 % p = (zb4 + 256 * zb5) % p)
    (lz3 : z3 % p = (zb6 + 256 * zb7) % p)
    (lz4 : z4 % p = (zb8 + 256 * zb9) % p)
    (lz5 : z5 % p = (zb10 + 256 * zb11) % p)
    (lz6 : z6 % p = (zb12 + 256 * zb13) % p)
    (lz7 : z7 % p = (zb14 + 256 * zb15) % p)
    (c0 c1 c2 c3 c4 c5 c6 c7 : Nat)
    (hc0 : c0 < p)
    (hc1 : c1 < p)
    (hc2 : c2 < p)
    (hc3 : c3 < p)
    (hc4 : c4 < p)
    (hc5 : c5 < p)
    (hc6 : c6 < p)
    (hc7 : c7 < p)
    (bc0 : c0 * c0 % p = c0 % p)
    (bc1 : c1 * c1 % p = c1 % p)
    (bc2 : c2 * c2 % p = c2 % p)
    (bc3 : c3 * c3 % p = c3 % p)
    (bc4 : c4 * c4 % p = c4 % p)
    (bc5 : c5 * c5 % p = c5 % p)
    (bc6 : c6 * c6 % p = c6 % p)
    (bc7 : c7 * c7 % p = c7 % p)
    (e0 : (z0 + y0) % p = (x0 + 65536 * c0) % p)
    (e1 : (z1 + y1 + c0) % p = (x1 + 65536 * c1) % p)
    (e2 : (z2 + y2 + c1) % p = (x2 + 65536 * c2) % p)
    (e3 : (z3 + y3 + c2) % p = (x3 + 65536 * c3) % p)
    (e4 : (z4 + y4 + c3) % p = (x4 + 65536 * c4) % p)
    (e5 : (z5 + y5 + c4) % p = (x5 + 65536 * c5) % p)
    (e6 : (z6 + y6 + c5) % p = (x6 + 65536 * c6) % p)
    (e7 : (z7 + y7 + c6) % p = (x7 + 65536 * c7) % p)
    (f : Nat) (hf : f < p) (fl : f % p = c7 % p) :
    (z0 + 65536 * z1 + 4294967296 * z2 + 281474976710656 * z3 + 18446744073709551616 * z4 + 1208925819614629174706176 * z5 + 79228162514264337593543950336 * z6 + 5192296858534827628530496329220096 * z7) = ((x0 + 65536 * x1 + 4294967296 * x2 + 281474976710656 * x3 + 18446744073709551616 * x4 + 1208925819614629174706176 * x5 + 79228162514264337593543950336 * x6 + 5192296858534827628530496329220096 * x7) + 2^128 - (y0 + 65536 * y1 + 4294967296 * y2 + 281474976710656 * y3 + 18446744073709551616 * y4 + 1208925819614629174706176 * y5 + 79228162514264337593543950336 * y6 + 5192296858534827628530496329220096 * y7)) % 2^128 ∧ (f = 1 ↔ (x0 + 65536 * x1 + 4294967296 * x2 + 281474976710656 * x3 + 18446744073709551616 * x4 + 1208925819614629174706176 * x5 + 79228162514264337593543950336 * x6 + 5192296858534827628530496329220096 * x7) < (y0 + 65536 * y1 + 4294967296 * y2 + 281474976710656 * y3 + 18446744073709551616 * y4 + 1208925819614629174706176 * y5 + 79228162514264337593543950336 * y6 + 5192296858534827628530496329220096 * y7)) ∧ f ≤ 1 := by
  have kx0 := byte_link x0 xb0 xb1 hx0 hxb0 hxb1 lx0
  have kx1 := byte_link x1 xb2 xb3 hx1 hxb2 hxb3 lx1
  have kx2 := byte_link x2 xb4 xb5 hx2 hxb4 hxb5 lx2
  have kx3 := byte_link x3 xb6 xb7 hx3 hxb6 hxb7 lx3
  have kx4 := byte_link x4 xb8 xb9 hx4 hxb8 hxb9 lx4
  have kx5 := byte_link x5 xb10 xb11 hx5 hxb10 hxb11 lx5
  have kx6 := byte_link x6 xb12 xb13 hx6 hxb12 hxb13 lx6
  have kx7 := byte_link x7 xb14 xb15 hx7 hxb14 hxb15 lx7
  have gx0 := link_lt x0 xb0 xb1 hxb0 hxb1 kx0
  have gx1 := link_lt x1 xb2 xb3 hxb2 hxb3 kx1
  have gx2 := link_lt x2 xb4 xb5 hxb4 hxb5 kx2
  have gx3 := link_lt x3 xb6 xb7 hxb6 hxb7 kx3
  have gx4 := link_lt x4 xb8 xb9 hxb8 hxb9 kx4
  have gx5 := link_lt x5 xb10 xb11 hxb10 hxb11 kx5
  have gx6 := link_lt x6 xb12 xb13 hxb12 hxb13 kx6
  have gx7 := link_lt x7 xb14 xb15 hxb14 hxb15 kx7
  have ky0 := byte_link y0 yb0 yb1 hy0 hyb0 hyb1 ly0
  have ky1 := byte_link y1 yb2 yb3 hy1 hyb2 hyb3 ly1
  have ky2 := byte_link y2 yb4 yb5 hy2 hyb4 hyb5 ly2
  have ky3 := byte_link y3 yb6 yb7 hy3 hyb6 hyb7 ly3
  have ky4 := byte_link y4 yb8 yb9 hy4 hyb8 hyb9 ly4
  have ky5 := byte_link y5 yb10 yb11 hy5 hyb10 hyb11 ly5
  have ky6 := byte_link y6 yb12 yb13 hy6 hyb12 hyb13 ly6
  have ky7 := byte_link y7 yb14 yb15 hy7 hyb14 hyb15 ly7
  have gy0 := link_lt y0 yb0 yb1 hyb0 hyb1 ky0
  have gy1 := link_lt y1 yb2 yb3 hyb2 hyb3 ky1
  have gy2 := link_lt y2 yb4 yb5 hyb4 hyb5 ky2
  have gy3 := link_lt y3 yb6 yb7 hyb6 hyb7 ky3
  have gy4 := link_lt y4 yb8 yb9 hyb8 hyb9 ky4
  have gy5 := link_lt y5 yb10 yb11 hyb10 hyb11 ky5
  have gy6 := link_lt y6 yb12 yb13 hyb12 hyb13 ky6
  have gy7 := link_lt y7 yb14 yb15 hyb14 hyb15 ky7
  have kz0 := byte_link z0 zb0 zb1 hz0 hzb0 hzb1 lz0
  have kz1 := byte_link z1 zb2 zb3 hz1 hzb2 hzb3 lz1
  have kz2 := byte_link z2 zb4 zb5 hz2 hzb4 hzb5 lz2
  have kz3 := byte_link z3 zb6 zb7 hz3 hzb6 hzb7 lz3
  have kz4 := byte_link z4 zb8 zb9 hz4 hzb8 hzb9 lz4
  have kz5 := byte_link z5 zb10 zb11 hz5 hzb10 hzb11 lz5
  have kz6 := byte_link z6 zb12 zb13 hz6 hzb12 hzb13 lz6
  have kz7 := byte_link z7 zb14 zb15 hz7 hzb14 hzb15 lz7
  have gz0 := link_lt z0 zb0 zb1 hzb0 hzb1 kz0
  have gz1 := link_lt z1 zb2 zb3 hzb2 hzb3 kz1
  have gz2 := link_lt z2 zb4 zb5 hzb4 hzb5 kz2
  have gz3 := link_lt z3 zb6 zb7 hzb6 hzb7 kz3
  have gz4 := link_lt z4 zb8 zb9 hzb8 hzb9 kz4
  have gz5 := link_lt z5 zb10 zb11 hzb10 hzb11 kz5
  have gz6 := link_lt z6 zb12 zb13 hzb12 hzb13 kz6
  have gz7 := link_lt z7 zb14 zb15 hzb14 hzb15 kz7
  have kc0 := field_bool c0 hc0 bc0
  have kc1 := field_bool c1 hc1 bc1
  have kc2 := field_bool c2 hc2 bc2
  have kc3 := field_bool c3 hc3 bc3
  have kc4 := field_bool c4 hc4 bc4
  have kc5 := field_bool c5 hc5 bc5
  have kc6 := field_bool c6 hc6 bc6
  have kc7 := field_bool c7 hc7 bc7
  have ch := chain_sound z0 z1 z2 z3 z4 z5 z6 z7 y0 y1 y2 y3 y4 y5 y6 y7 x0 x1 x2 x3 x4 x5 x6 x7 c0 c1 c2 c3 c4 c5 c6 c7 gz0 gz1 gz2 gz3 gz4 gz5 gz6 gz7 gy0 gy1 gy2 gy3 gy4 gy5 gy6 gy7 gx0 gx1 gx2 gx3 gx4 gx5 gx6 gx7 kc0 kc1 kc2 kc3 kc4 kc5 kc6 kc7 e0 e1 e2 e3 e4 e5 e6 e7
  have nf := no_wrap hf hc7 fl
  clear e0 e1 e2 e3 e4 e5 e6 e7 bc0 bc1 bc2 bc3 bc4 bc5 bc6 bc7 fl lx0 lx1 lx2 lx3 lx4 lx5 lx6 lx7 ly0 ly1 ly2 ly3 ly4 ly5 ly6 ly7 lz0 lz1 lz2 lz3 lz4 lz5 lz6 lz7
  omega

set_option maxHeartbeats 4000000 in
/-- LT row (chain `(w, y, x)`, `w` ranged): `z = [x < y]`, `flag = 0`. -/
theorem lt_row
    (x0 x1 x2 x3 x4 x5 x6 x7 : Nat) (xb0 xb1 xb2 xb3 xb4 xb5 xb6 xb7 xb8 xb9 xb10 xb11 xb12 xb13 xb14 xb15 : Nat)
    (hx0 : x0 < p)
    (hx1 : x1 < p)
    (hx2 : x2 < p)
    (hx3 : x3 < p)
    (hx4 : x4 < p)
    (hx5 : x5 < p)
    (hx6 : x6 < p)
    (hx7 : x7 < p)
    (hxb0 : xb0 < 256)
    (hxb1 : xb1 < 256)
    (hxb2 : xb2 < 256)
    (hxb3 : xb3 < 256)
    (hxb4 : xb4 < 256)
    (hxb5 : xb5 < 256)
    (hxb6 : xb6 < 256)
    (hxb7 : xb7 < 256)
    (hxb8 : xb8 < 256)
    (hxb9 : xb9 < 256)
    (hxb10 : xb10 < 256)
    (hxb11 : xb11 < 256)
    (hxb12 : xb12 < 256)
    (hxb13 : xb13 < 256)
    (hxb14 : xb14 < 256)
    (hxb15 : xb15 < 256)
    (lx0 : x0 % p = (xb0 + 256 * xb1) % p)
    (lx1 : x1 % p = (xb2 + 256 * xb3) % p)
    (lx2 : x2 % p = (xb4 + 256 * xb5) % p)
    (lx3 : x3 % p = (xb6 + 256 * xb7) % p)
    (lx4 : x4 % p = (xb8 + 256 * xb9) % p)
    (lx5 : x5 % p = (xb10 + 256 * xb11) % p)
    (lx6 : x6 % p = (xb12 + 256 * xb13) % p)
    (lx7 : x7 % p = (xb14 + 256 * xb15) % p)
    (y0 y1 y2 y3 y4 y5 y6 y7 : Nat) (yb0 yb1 yb2 yb3 yb4 yb5 yb6 yb7 yb8 yb9 yb10 yb11 yb12 yb13 yb14 yb15 : Nat)
    (hy0 : y0 < p)
    (hy1 : y1 < p)
    (hy2 : y2 < p)
    (hy3 : y3 < p)
    (hy4 : y4 < p)
    (hy5 : y5 < p)
    (hy6 : y6 < p)
    (hy7 : y7 < p)
    (hyb0 : yb0 < 256)
    (hyb1 : yb1 < 256)
    (hyb2 : yb2 < 256)
    (hyb3 : yb3 < 256)
    (hyb4 : yb4 < 256)
    (hyb5 : yb5 < 256)
    (hyb6 : yb6 < 256)
    (hyb7 : yb7 < 256)
    (hyb8 : yb8 < 256)
    (hyb9 : yb9 < 256)
    (hyb10 : yb10 < 256)
    (hyb11 : yb11 < 256)
    (hyb12 : yb12 < 256)
    (hyb13 : yb13 < 256)
    (hyb14 : yb14 < 256)
    (hyb15 : yb15 < 256)
    (ly0 : y0 % p = (yb0 + 256 * yb1) % p)
    (ly1 : y1 % p = (yb2 + 256 * yb3) % p)
    (ly2 : y2 % p = (yb4 + 256 * yb5) % p)
    (ly3 : y3 % p = (yb6 + 256 * yb7) % p)
    (ly4 : y4 % p = (yb8 + 256 * yb9) % p)
    (ly5 : y5 % p = (yb10 + 256 * yb11) % p)
    (ly6 : y6 % p = (yb12 + 256 * yb13) % p)
    (ly7 : y7 % p = (yb14 + 256 * yb15) % p)
    (z0 z1 z2 z3 z4 z5 z6 z7 : Nat) (zb0 zb1 zb2 zb3 zb4 zb5 zb6 zb7 zb8 zb9 zb10 zb11 zb12 zb13 zb14 zb15 : Nat)
    (hz0 : z0 < p)
    (hz1 : z1 < p)
    (hz2 : z2 < p)
    (hz3 : z3 < p)
    (hz4 : z4 < p)
    (hz5 : z5 < p)
    (hz6 : z6 < p)
    (hz7 : z7 < p)
    (hzb0 : zb0 < 256)
    (hzb1 : zb1 < 256)
    (hzb2 : zb2 < 256)
    (hzb3 : zb3 < 256)
    (hzb4 : zb4 < 256)
    (hzb5 : zb5 < 256)
    (hzb6 : zb6 < 256)
    (hzb7 : zb7 < 256)
    (hzb8 : zb8 < 256)
    (hzb9 : zb9 < 256)
    (hzb10 : zb10 < 256)
    (hzb11 : zb11 < 256)
    (hzb12 : zb12 < 256)
    (hzb13 : zb13 < 256)
    (hzb14 : zb14 < 256)
    (hzb15 : zb15 < 256)
    (lz0 : z0 % p = (zb0 + 256 * zb1) % p)
    (lz1 : z1 % p = (zb2 + 256 * zb3) % p)
    (lz2 : z2 % p = (zb4 + 256 * zb5) % p)
    (lz3 : z3 % p = (zb6 + 256 * zb7) % p)
    (lz4 : z4 % p = (zb8 + 256 * zb9) % p)
    (lz5 : z5 % p = (zb10 + 256 * zb11) % p)
    (lz6 : z6 % p = (zb12 + 256 * zb13) % p)
    (lz7 : z7 % p = (zb14 + 256 * zb15) % p)
    (w0 w1 w2 w3 w4 w5 w6 w7 : Nat)
    (hw0 : w0 < 65536)
    (hw1 : w1 < 65536)
    (hw2 : w2 < 65536)
    (hw3 : w3 < 65536)
    (hw4 : w4 < 65536)
    (hw5 : w5 < 65536)
    (hw6 : w6 < 65536)
    (hw7 : w7 < 65536)
    (c0 c1 c2 c3 c4 c5 c6 c7 : Nat)
    (hc0 : c0 < p)
    (hc1 : c1 < p)
    (hc2 : c2 < p)
    (hc3 : c3 < p)
    (hc4 : c4 < p)
    (hc5 : c5 < p)
    (hc6 : c6 < p)
    (hc7 : c7 < p)
    (bc0 : c0 * c0 % p = c0 % p)
    (bc1 : c1 * c1 % p = c1 % p)
    (bc2 : c2 * c2 % p = c2 % p)
    (bc3 : c3 * c3 % p = c3 % p)
    (bc4 : c4 * c4 % p = c4 % p)
    (bc5 : c5 * c5 % p = c5 % p)
    (bc6 : c6 * c6 % p = c6 % p)
    (bc7 : c7 * c7 % p = c7 % p)
    (e0 : (w0 + y0) % p = (x0 + 65536 * c0) % p)
    (e1 : (w1 + y1 + c0) % p = (x1 + 65536 * c1) % p)
    (e2 : (w2 + y2 + c1) % p = (x2 + 65536 * c2) % p)
    (e3 : (w3 + y3 + c2) % p = (x3 + 65536 * c3) % p)
    (e4 : (w4 + y4 + c3) % p = (x4 + 65536 * c4) % p)
    (e5 : (w5 + y5 + c4) % p = (x5 + 65536 * c5) % p)
    (e6 : (w6 + y6 + c5) % p = (x6 + 65536 * c6) % p)
    (e7 : (w7 + y7 + c6) % p = (x7 + 65536 * c7) % p)
    (tz0 : z0 % p = c7 % p)
    (lzz1 : z1 % p = 0)
    (lzz2 : z2 % p = 0)
    (lzz3 : z3 % p = 0)
    (lzz4 : z4 % p = 0)
    (lzz5 : z5 % p = 0)
    (lzz6 : z6 % p = 0)
    (lzz7 : z7 % p = 0)
    (f : Nat) (hf : f < p) (fl : f % p = 0) :
    ((z0 + 65536 * z1 + 4294967296 * z2 + 281474976710656 * z3 + 18446744073709551616 * z4 + 1208925819614629174706176 * z5 + 79228162514264337593543950336 * z6 + 5192296858534827628530496329220096 * z7) = 1 ↔ (x0 + 65536 * x1 + 4294967296 * x2 + 281474976710656 * x3 + 18446744073709551616 * x4 + 1208925819614629174706176 * x5 + 79228162514264337593543950336 * x6 + 5192296858534827628530496329220096 * x7) < (y0 + 65536 * y1 + 4294967296 * y2 + 281474976710656 * y3 + 18446744073709551616 * y4 + 1208925819614629174706176 * y5 + 79228162514264337593543950336 * y6 + 5192296858534827628530496329220096 * y7)) ∧ (z0 + 65536 * z1 + 4294967296 * z2 + 281474976710656 * z3 + 18446744073709551616 * z4 + 1208925819614629174706176 * z5 + 79228162514264337593543950336 * z6 + 5192296858534827628530496329220096 * z7) ≤ 1 ∧ f = 0 := by
  have kx0 := byte_link x0 xb0 xb1 hx0 hxb0 hxb1 lx0
  have kx1 := byte_link x1 xb2 xb3 hx1 hxb2 hxb3 lx1
  have kx2 := byte_link x2 xb4 xb5 hx2 hxb4 hxb5 lx2
  have kx3 := byte_link x3 xb6 xb7 hx3 hxb6 hxb7 lx3
  have kx4 := byte_link x4 xb8 xb9 hx4 hxb8 hxb9 lx4
  have kx5 := byte_link x5 xb10 xb11 hx5 hxb10 hxb11 lx5
  have kx6 := byte_link x6 xb12 xb13 hx6 hxb12 hxb13 lx6
  have kx7 := byte_link x7 xb14 xb15 hx7 hxb14 hxb15 lx7
  have gx0 := link_lt x0 xb0 xb1 hxb0 hxb1 kx0
  have gx1 := link_lt x1 xb2 xb3 hxb2 hxb3 kx1
  have gx2 := link_lt x2 xb4 xb5 hxb4 hxb5 kx2
  have gx3 := link_lt x3 xb6 xb7 hxb6 hxb7 kx3
  have gx4 := link_lt x4 xb8 xb9 hxb8 hxb9 kx4
  have gx5 := link_lt x5 xb10 xb11 hxb10 hxb11 kx5
  have gx6 := link_lt x6 xb12 xb13 hxb12 hxb13 kx6
  have gx7 := link_lt x7 xb14 xb15 hxb14 hxb15 kx7
  have ky0 := byte_link y0 yb0 yb1 hy0 hyb0 hyb1 ly0
  have ky1 := byte_link y1 yb2 yb3 hy1 hyb2 hyb3 ly1
  have ky2 := byte_link y2 yb4 yb5 hy2 hyb4 hyb5 ly2
  have ky3 := byte_link y3 yb6 yb7 hy3 hyb6 hyb7 ly3
  have ky4 := byte_link y4 yb8 yb9 hy4 hyb8 hyb9 ly4
  have ky5 := byte_link y5 yb10 yb11 hy5 hyb10 hyb11 ly5
  have ky6 := byte_link y6 yb12 yb13 hy6 hyb12 hyb13 ly6
  have ky7 := byte_link y7 yb14 yb15 hy7 hyb14 hyb15 ly7
  have gy0 := link_lt y0 yb0 yb1 hyb0 hyb1 ky0
  have gy1 := link_lt y1 yb2 yb3 hyb2 hyb3 ky1
  have gy2 := link_lt y2 yb4 yb5 hyb4 hyb5 ky2
  have gy3 := link_lt y3 yb6 yb7 hyb6 hyb7 ky3
  have gy4 := link_lt y4 yb8 yb9 hyb8 hyb9 ky4
  have gy5 := link_lt y5 yb10 yb11 hyb10 hyb11 ky5
  have gy6 := link_lt y6 yb12 yb13 hyb12 hyb13 ky6
  have gy7 := link_lt y7 yb14 yb15 hyb14 hyb15 ky7
  have kz0 := byte_link z0 zb0 zb1 hz0 hzb0 hzb1 lz0
  have kz1 := byte_link z1 zb2 zb3 hz1 hzb2 hzb3 lz1
  have kz2 := byte_link z2 zb4 zb5 hz2 hzb4 hzb5 lz2
  have kz3 := byte_link z3 zb6 zb7 hz3 hzb6 hzb7 lz3
  have kz4 := byte_link z4 zb8 zb9 hz4 hzb8 hzb9 lz4
  have kz5 := byte_link z5 zb10 zb11 hz5 hzb10 hzb11 lz5
  have kz6 := byte_link z6 zb12 zb13 hz6 hzb12 hzb13 lz6
  have kz7 := byte_link z7 zb14 zb15 hz7 hzb14 hzb15 lz7
  have gz0 := link_lt z0 zb0 zb1 hzb0 hzb1 kz0
  have gz1 := link_lt z1 zb2 zb3 hzb2 hzb3 kz1
  have gz2 := link_lt z2 zb4 zb5 hzb4 hzb5 kz2
  have gz3 := link_lt z3 zb6 zb7 hzb6 hzb7 kz3
  have gz4 := link_lt z4 zb8 zb9 hzb8 hzb9 kz4
  have gz5 := link_lt z5 zb10 zb11 hzb10 hzb11 kz5
  have gz6 := link_lt z6 zb12 zb13 hzb12 hzb13 kz6
  have gz7 := link_lt z7 zb14 zb15 hzb14 hzb15 kz7
  have kc0 := field_bool c0 hc0 bc0
  have kc1 := field_bool c1 hc1 bc1
  have kc2 := field_bool c2 hc2 bc2
  have kc3 := field_bool c3 hc3 bc3
  have kc4 := field_bool c4 hc4 bc4
  have kc5 := field_bool c5 hc5 bc5
  have kc6 := field_bool c6 hc6 bc6
  have kc7 := field_bool c7 hc7 bc7
  have ch := chain_sound w0 w1 w2 w3 w4 w5 w6 w7 y0 y1 y2 y3 y4 y5 y6 y7 x0 x1 x2 x3 x4 x5 x6 x7 c0 c1 c2 c3 c4 c5 c6 c7 hw0 hw1 hw2 hw3 hw4 hw5 hw6 hw7 gy0 gy1 gy2 gy3 gy4 gy5 gy6 gy7 gx0 gx1 gx2 gx3 gx4 gx5 gx6 gx7 kc0 kc1 kc2 kc3 kc4 kc5 kc6 kc7 e0 e1 e2 e3 e4 e5 e6 e7
  have nz0 := no_wrap hz0 hc7 tz0
  have nzz1 := zero_of_mod z1 hz1 lzz1
  have nzz2 := zero_of_mod z2 hz2 lzz2
  have nzz3 := zero_of_mod z3 hz3 lzz3
  have nzz4 := zero_of_mod z4 hz4 lzz4
  have nzz5 := zero_of_mod z5 hz5 lzz5
  have nzz6 := zero_of_mod z6 hz6 lzz6
  have nzz7 := zero_of_mod z7 hz7 lzz7
  have nf := zero_of_mod f hf fl
  clear e0 e1 e2 e3 e4 e5 e6 e7 bc0 bc1 bc2 bc3 bc4 bc5 bc6 bc7 fl tz0 lzz1 lzz2 lzz3 lzz4 lzz5 lzz6 lzz7 lx0 lx1 lx2 lx3 lx4 lx5 lx6 lx7 ly0 ly1 ly2 ly3 ly4 ly5 ly6 ly7 lz0 lz1 lz2 lz3 lz4 lz5 lz6 lz7
  omega

set_option maxHeartbeats 4000000 in
/-- EQ row (with the boolean of LEAD PICK ALU-EQBOOL): `z = [x = y]`, `flag = 0`, for every field value of the inverses `v`. -/
theorem eq_row
    (x0 x1 x2 x3 x4 x5 x6 x7 : Nat) (xb0 xb1 xb2 xb3 xb4 xb5 xb6 xb7 xb8 xb9 xb10 xb11 xb12 xb13 xb14 xb15 : Nat)
    (hx0 : x0 < p)
    (hx1 : x1 < p)
    (hx2 : x2 < p)
    (hx3 : x3 < p)
    (hx4 : x4 < p)
    (hx5 : x5 < p)
    (hx6 : x6 < p)
    (hx7 : x7 < p)
    (hxb0 : xb0 < 256)
    (hxb1 : xb1 < 256)
    (hxb2 : xb2 < 256)
    (hxb3 : xb3 < 256)
    (hxb4 : xb4 < 256)
    (hxb5 : xb5 < 256)
    (hxb6 : xb6 < 256)
    (hxb7 : xb7 < 256)
    (hxb8 : xb8 < 256)
    (hxb9 : xb9 < 256)
    (hxb10 : xb10 < 256)
    (hxb11 : xb11 < 256)
    (hxb12 : xb12 < 256)
    (hxb13 : xb13 < 256)
    (hxb14 : xb14 < 256)
    (hxb15 : xb15 < 256)
    (lx0 : x0 % p = (xb0 + 256 * xb1) % p)
    (lx1 : x1 % p = (xb2 + 256 * xb3) % p)
    (lx2 : x2 % p = (xb4 + 256 * xb5) % p)
    (lx3 : x3 % p = (xb6 + 256 * xb7) % p)
    (lx4 : x4 % p = (xb8 + 256 * xb9) % p)
    (lx5 : x5 % p = (xb10 + 256 * xb11) % p)
    (lx6 : x6 % p = (xb12 + 256 * xb13) % p)
    (lx7 : x7 % p = (xb14 + 256 * xb15) % p)
    (y0 y1 y2 y3 y4 y5 y6 y7 : Nat) (yb0 yb1 yb2 yb3 yb4 yb5 yb6 yb7 yb8 yb9 yb10 yb11 yb12 yb13 yb14 yb15 : Nat)
    (hy0 : y0 < p)
    (hy1 : y1 < p)
    (hy2 : y2 < p)
    (hy3 : y3 < p)
    (hy4 : y4 < p)
    (hy5 : y5 < p)
    (hy6 : y6 < p)
    (hy7 : y7 < p)
    (hyb0 : yb0 < 256)
    (hyb1 : yb1 < 256)
    (hyb2 : yb2 < 256)
    (hyb3 : yb3 < 256)
    (hyb4 : yb4 < 256)
    (hyb5 : yb5 < 256)
    (hyb6 : yb6 < 256)
    (hyb7 : yb7 < 256)
    (hyb8 : yb8 < 256)
    (hyb9 : yb9 < 256)
    (hyb10 : yb10 < 256)
    (hyb11 : yb11 < 256)
    (hyb12 : yb12 < 256)
    (hyb13 : yb13 < 256)
    (hyb14 : yb14 < 256)
    (hyb15 : yb15 < 256)
    (ly0 : y0 % p = (yb0 + 256 * yb1) % p)
    (ly1 : y1 % p = (yb2 + 256 * yb3) % p)
    (ly2 : y2 % p = (yb4 + 256 * yb5) % p)
    (ly3 : y3 % p = (yb6 + 256 * yb7) % p)
    (ly4 : y4 % p = (yb8 + 256 * yb9) % p)
    (ly5 : y5 % p = (yb10 + 256 * yb11) % p)
    (ly6 : y6 % p = (yb12 + 256 * yb13) % p)
    (ly7 : y7 % p = (yb14 + 256 * yb15) % p)
    (z0 z1 z2 z3 z4 z5 z6 z7 : Nat) (zb0 zb1 zb2 zb3 zb4 zb5 zb6 zb7 zb8 zb9 zb10 zb11 zb12 zb13 zb14 zb15 : Nat)
    (hz0 : z0 < p)
    (hz1 : z1 < p)
    (hz2 : z2 < p)
    (hz3 : z3 < p)
    (hz4 : z4 < p)
    (hz5 : z5 < p)
    (hz6 : z6 < p)
    (hz7 : z7 < p)
    (hzb0 : zb0 < 256)
    (hzb1 : zb1 < 256)
    (hzb2 : zb2 < 256)
    (hzb3 : zb3 < 256)
    (hzb4 : zb4 < 256)
    (hzb5 : zb5 < 256)
    (hzb6 : zb6 < 256)
    (hzb7 : zb7 < 256)
    (hzb8 : zb8 < 256)
    (hzb9 : zb9 < 256)
    (hzb10 : zb10 < 256)
    (hzb11 : zb11 < 256)
    (hzb12 : zb12 < 256)
    (hzb13 : zb13 < 256)
    (hzb14 : zb14 < 256)
    (hzb15 : zb15 < 256)
    (lz0 : z0 % p = (zb0 + 256 * zb1) % p)
    (lz1 : z1 % p = (zb2 + 256 * zb3) % p)
    (lz2 : z2 % p = (zb4 + 256 * zb5) % p)
    (lz3 : z3 % p = (zb6 + 256 * zb7) % p)
    (lz4 : z4 % p = (zb8 + 256 * zb9) % p)
    (lz5 : z5 % p = (zb10 + 256 * zb11) % p)
    (lz6 : z6 % p = (zb12 + 256 * zb13) % p)
    (lz7 : z7 % p = (zb14 + 256 * zb15) % p)
    (v0 v1 v2 v3 v4 v5 v6 v7 : Nat)
    (bz : z0 * z0 % p = z0 % p)
    (fw0 : z0 * x0 % p = z0 * y0 % p)
    (fw1 : z0 * x1 % p = z0 * y1 % p)
    (fw2 : z0 * x2 % p = z0 * y2 % p)
    (fw3 : z0 * x3 % p = z0 * y3 % p)
    (fw4 : z0 * x4 % p = z0 * y4 % p)
    (fw5 : z0 * x5 % p = z0 * y5 % p)
    (fw6 : z0 * x6 % p = z0 * y6 % p)
    (fw7 : z0 * x7 % p = z0 * y7 % p)
    (rv : (x0 * v0 + x1 * v1 + x2 * v2 + x3 * v3 + x4 * v4 + x5 * v5 + x6 * v6 + x7 * v7 + z0) % p = (y0 * v0 + y1 * v1 + y2 * v2 + y3 * v3 + y4 * v4 + y5 * v5 + y6 * v6 + y7 * v7 + 1) % p)
    (lzz1 : z1 % p = 0)
    (lzz2 : z2 % p = 0)
    (lzz3 : z3 % p = 0)
    (lzz4 : z4 % p = 0)
    (lzz5 : z5 % p = 0)
    (lzz6 : z6 % p = 0)
    (lzz7 : z7 % p = 0)
    (f : Nat) (hf : f < p) (fl : f % p = 0) :
    ((z0 + 65536 * z1 + 4294967296 * z2 + 281474976710656 * z3 + 18446744073709551616 * z4 + 1208925819614629174706176 * z5 + 79228162514264337593543950336 * z6 + 5192296858534827628530496329220096 * z7) = 1 ↔ (x0 + 65536 * x1 + 4294967296 * x2 + 281474976710656 * x3 + 18446744073709551616 * x4 + 1208925819614629174706176 * x5 + 79228162514264337593543950336 * x6 + 5192296858534827628530496329220096 * x7) = (y0 + 65536 * y1 + 4294967296 * y2 + 281474976710656 * y3 + 18446744073709551616 * y4 + 1208925819614629174706176 * y5 + 79228162514264337593543950336 * y6 + 5192296858534827628530496329220096 * y7)) ∧ (z0 + 65536 * z1 + 4294967296 * z2 + 281474976710656 * z3 + 18446744073709551616 * z4 + 1208925819614629174706176 * z5 + 79228162514264337593543950336 * z6 + 5192296858534827628530496329220096 * z7) ≤ 1 ∧ f = 0 := by
  have kx0 := byte_link x0 xb0 xb1 hx0 hxb0 hxb1 lx0
  have kx1 := byte_link x1 xb2 xb3 hx1 hxb2 hxb3 lx1
  have kx2 := byte_link x2 xb4 xb5 hx2 hxb4 hxb5 lx2
  have kx3 := byte_link x3 xb6 xb7 hx3 hxb6 hxb7 lx3
  have kx4 := byte_link x4 xb8 xb9 hx4 hxb8 hxb9 lx4
  have kx5 := byte_link x5 xb10 xb11 hx5 hxb10 hxb11 lx5
  have kx6 := byte_link x6 xb12 xb13 hx6 hxb12 hxb13 lx6
  have kx7 := byte_link x7 xb14 xb15 hx7 hxb14 hxb15 lx7
  have gx0 := link_lt x0 xb0 xb1 hxb0 hxb1 kx0
  have gx1 := link_lt x1 xb2 xb3 hxb2 hxb3 kx1
  have gx2 := link_lt x2 xb4 xb5 hxb4 hxb5 kx2
  have gx3 := link_lt x3 xb6 xb7 hxb6 hxb7 kx3
  have gx4 := link_lt x4 xb8 xb9 hxb8 hxb9 kx4
  have gx5 := link_lt x5 xb10 xb11 hxb10 hxb11 kx5
  have gx6 := link_lt x6 xb12 xb13 hxb12 hxb13 kx6
  have gx7 := link_lt x7 xb14 xb15 hxb14 hxb15 kx7
  have ky0 := byte_link y0 yb0 yb1 hy0 hyb0 hyb1 ly0
  have ky1 := byte_link y1 yb2 yb3 hy1 hyb2 hyb3 ly1
  have ky2 := byte_link y2 yb4 yb5 hy2 hyb4 hyb5 ly2
  have ky3 := byte_link y3 yb6 yb7 hy3 hyb6 hyb7 ly3
  have ky4 := byte_link y4 yb8 yb9 hy4 hyb8 hyb9 ly4
  have ky5 := byte_link y5 yb10 yb11 hy5 hyb10 hyb11 ly5
  have ky6 := byte_link y6 yb12 yb13 hy6 hyb12 hyb13 ly6
  have ky7 := byte_link y7 yb14 yb15 hy7 hyb14 hyb15 ly7
  have gy0 := link_lt y0 yb0 yb1 hyb0 hyb1 ky0
  have gy1 := link_lt y1 yb2 yb3 hyb2 hyb3 ky1
  have gy2 := link_lt y2 yb4 yb5 hyb4 hyb5 ky2
  have gy3 := link_lt y3 yb6 yb7 hyb6 hyb7 ky3
  have gy4 := link_lt y4 yb8 yb9 hyb8 hyb9 ky4
  have gy5 := link_lt y5 yb10 yb11 hyb10 hyb11 ky5
  have gy6 := link_lt y6 yb12 yb13 hyb12 hyb13 ky6
  have gy7 := link_lt y7 yb14 yb15 hyb14 hyb15 ky7
  have kz0 := byte_link z0 zb0 zb1 hz0 hzb0 hzb1 lz0
  have kz1 := byte_link z1 zb2 zb3 hz1 hzb2 hzb3 lz1
  have kz2 := byte_link z2 zb4 zb5 hz2 hzb4 hzb5 lz2
  have kz3 := byte_link z3 zb6 zb7 hz3 hzb6 hzb7 lz3
  have kz4 := byte_link z4 zb8 zb9 hz4 hzb8 hzb9 lz4
  have kz5 := byte_link z5 zb10 zb11 hz5 hzb10 hzb11 lz5
  have kz6 := byte_link z6 zb12 zb13 hz6 hzb12 hzb13 lz6
  have kz7 := byte_link z7 zb14 zb15 hz7 hzb14 hzb15 lz7
  have gz0 := link_lt z0 zb0 zb1 hzb0 hzb1 kz0
  have gz1 := link_lt z1 zb2 zb3 hzb2 hzb3 kz1
  have gz2 := link_lt z2 zb4 zb5 hzb4 hzb5 kz2
  have gz3 := link_lt z3 zb6 zb7 hzb6 hzb7 kz3
  have gz4 := link_lt z4 zb8 zb9 hzb8 hzb9 kz4
  have gz5 := link_lt z5 zb10 zb11 hzb10 hzb11 kz5
  have gz6 := link_lt z6 zb12 zb13 hzb12 hzb13 kz6
  have gz7 := link_lt z7 zb14 zb15 hzb14 hzb15 kz7
  have kz := field_bool z0 hz0 bz
  have nzz1 := zero_of_mod z1 hz1 lzz1
  have nzz2 := zero_of_mod z2 hz2 lzz2
  have nzz3 := zero_of_mod z3 hz3 lzz3
  have nzz4 := zero_of_mod z4 hz4 lzz4
  have nzz5 := zero_of_mod z5 hz5 lzz5
  have nzz6 := zero_of_mod z6 hz6 lzz6
  have nzz7 := zero_of_mod z7 hz7 lzz7
  have nf := zero_of_mod f hf fl
  clear bz fl lzz1 lzz2 lzz3 lzz4 lzz5 lzz6 lzz7 lx0 lx1 lx2 lx3 lx4 lx5 lx6 lx7 ly0 ly1 ly2 ly3 ly4 ly5 ly6 ly7 lz0 lz1 lz2 lz3 lz4 lz5 lz6 lz7
  rcases (show z0 = 0 ∨ z0 = 1 by omega) with h0 | h1
  · have hne : ¬(x0 = y0 ∧ x1 = y1 ∧ x2 = y2 ∧ x3 = y3 ∧ x4 = y4 ∧ x5 = y5 ∧ x6 = y6 ∧ x7 = y7) := by
      intro hq
      obtain ⟨q0, q1, q2, q3, q4, q5, q6, q7⟩ := hq
      subst q0; subst q1; subst q2; subst q3; subst q4; subst q5; subst q6; subst q7; subst h0
      exact rev_zero _ rv
    clear rv fw0 fw1 fw2 fw3 fw4 fw5 fw6 fw7
    omega
  · have m0 := eq_of_one_mul z0 x0 y0 h1 hx0 hy0 fw0
    have m1 := eq_of_one_mul z0 x1 y1 h1 hx1 hy1 fw1
    have m2 := eq_of_one_mul z0 x2 y2 h1 hx2 hy2 fw2
    have m3 := eq_of_one_mul z0 x3 y3 h1 hx3 hy3 fw3
    have m4 := eq_of_one_mul z0 x4 y4 h1 hx4 hy4 fw4
    have m5 := eq_of_one_mul z0 x5 y5 h1 hx5 hy5 fw5
    have m6 := eq_of_one_mul z0 x6 y6 h1 hx6 hy6 fw6
    have m7 := eq_of_one_mul z0 x7 y7 h1 hx7 hy7 fw7
    clear rv fw0 fw1 fw2 fw3 fw4 fw5 fw6 fw7
    omega

theorem mul_col0 (a0 b0 : Nat) (ha0 : a0 < 256) (hb0 : b0 < 256) (r0 o0 : Nat) (hr0 : r0 < 65536) (ho : o0 < 256) (e : (a0 * b0) % p = (o0 + 256 * r0) % p) :
    a0 * b0 = o0 + 256 * r0 := by
  have q0_0 := byte_prod_le a0 b0 ha0 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col1 (a0 a1 b0 b1 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (r0 r1 o1 : Nat) (hr0 : r0 < 65536) (hr1 : r1 < 65536) (ho : o1 < 256) (e : (a0 * b1 + a1 * b0 + r0) % p = (o1 + 256 * r1) % p) :
    a0 * b1 + a1 * b0 + r0 = o1 + 256 * r1 := by
  have q0_1 := byte_prod_le a0 b1 ha0 hb1
  have q1_0 := byte_prod_le a1 b0 ha1 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col2 (a0 a1 a2 b0 b1 b2 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (r1 r2 o2 : Nat) (hr1 : r1 < 65536) (hr2 : r2 < 65536) (ho : o2 < 256) (e : (a0 * b2 + a1 * b1 + a2 * b0 + r1) % p = (o2 + 256 * r2) % p) :
    a0 * b2 + a1 * b1 + a2 * b0 + r1 = o2 + 256 * r2 := by
  have q0_2 := byte_prod_le a0 b2 ha0 hb2
  have q1_1 := byte_prod_le a1 b1 ha1 hb1
  have q2_0 := byte_prod_le a2 b0 ha2 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col3 (a0 a1 a2 a3 b0 b1 b2 b3 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (r2 r3 o3 : Nat) (hr2 : r2 < 65536) (hr3 : r3 < 65536) (ho : o3 < 256) (e : (a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0 + r2) % p = (o3 + 256 * r3) % p) :
    a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0 + r2 = o3 + 256 * r3 := by
  have q0_3 := byte_prod_le a0 b3 ha0 hb3
  have q1_2 := byte_prod_le a1 b2 ha1 hb2
  have q2_1 := byte_prod_le a2 b1 ha2 hb1
  have q3_0 := byte_prod_le a3 b0 ha3 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col4 (a0 a1 a2 a3 a4 b0 b1 b2 b3 b4 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (r3 r4 o4 : Nat) (hr3 : r3 < 65536) (hr4 : r4 < 65536) (ho : o4 < 256) (e : (a0 * b4 + a1 * b3 + a2 * b2 + a3 * b1 + a4 * b0 + r3) % p = (o4 + 256 * r4) % p) :
    a0 * b4 + a1 * b3 + a2 * b2 + a3 * b1 + a4 * b0 + r3 = o4 + 256 * r4 := by
  have q0_4 := byte_prod_le a0 b4 ha0 hb4
  have q1_3 := byte_prod_le a1 b3 ha1 hb3
  have q2_2 := byte_prod_le a2 b2 ha2 hb2
  have q3_1 := byte_prod_le a3 b1 ha3 hb1
  have q4_0 := byte_prod_le a4 b0 ha4 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col5 (a0 a1 a2 a3 a4 a5 b0 b1 b2 b3 b4 b5 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (r4 r5 o5 : Nat) (hr4 : r4 < 65536) (hr5 : r5 < 65536) (ho : o5 < 256) (e : (a0 * b5 + a1 * b4 + a2 * b3 + a3 * b2 + a4 * b1 + a5 * b0 + r4) % p = (o5 + 256 * r5) % p) :
    a0 * b5 + a1 * b4 + a2 * b3 + a3 * b2 + a4 * b1 + a5 * b0 + r4 = o5 + 256 * r5 := by
  have q0_5 := byte_prod_le a0 b5 ha0 hb5
  have q1_4 := byte_prod_le a1 b4 ha1 hb4
  have q2_3 := byte_prod_le a2 b3 ha2 hb3
  have q3_2 := byte_prod_le a3 b2 ha3 hb2
  have q4_1 := byte_prod_le a4 b1 ha4 hb1
  have q5_0 := byte_prod_le a5 b0 ha5 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col6 (a0 a1 a2 a3 a4 a5 a6 b0 b1 b2 b3 b4 b5 b6 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (r5 r6 o6 : Nat) (hr5 : r5 < 65536) (hr6 : r6 < 65536) (ho : o6 < 256) (e : (a0 * b6 + a1 * b5 + a2 * b4 + a3 * b3 + a4 * b2 + a5 * b1 + a6 * b0 + r5) % p = (o6 + 256 * r6) % p) :
    a0 * b6 + a1 * b5 + a2 * b4 + a3 * b3 + a4 * b2 + a5 * b1 + a6 * b0 + r5 = o6 + 256 * r6 := by
  have q0_6 := byte_prod_le a0 b6 ha0 hb6
  have q1_5 := byte_prod_le a1 b5 ha1 hb5
  have q2_4 := byte_prod_le a2 b4 ha2 hb4
  have q3_3 := byte_prod_le a3 b3 ha3 hb3
  have q4_2 := byte_prod_le a4 b2 ha4 hb2
  have q5_1 := byte_prod_le a5 b1 ha5 hb1
  have q6_0 := byte_prod_le a6 b0 ha6 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col7 (a0 a1 a2 a3 a4 a5 a6 a7 b0 b1 b2 b3 b4 b5 b6 b7 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (r6 r7 o7 : Nat) (hr6 : r6 < 65536) (hr7 : r7 < 65536) (ho : o7 < 256) (e : (a0 * b7 + a1 * b6 + a2 * b5 + a3 * b4 + a4 * b3 + a5 * b2 + a6 * b1 + a7 * b0 + r6) % p = (o7 + 256 * r7) % p) :
    a0 * b7 + a1 * b6 + a2 * b5 + a3 * b4 + a4 * b3 + a5 * b2 + a6 * b1 + a7 * b0 + r6 = o7 + 256 * r7 := by
  have q0_7 := byte_prod_le a0 b7 ha0 hb7
  have q1_6 := byte_prod_le a1 b6 ha1 hb6
  have q2_5 := byte_prod_le a2 b5 ha2 hb5
  have q3_4 := byte_prod_le a3 b4 ha3 hb4
  have q4_3 := byte_prod_le a4 b3 ha4 hb3
  have q5_2 := byte_prod_le a5 b2 ha5 hb2
  have q6_1 := byte_prod_le a6 b1 ha6 hb1
  have q7_0 := byte_prod_le a7 b0 ha7 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col8 (a0 a1 a2 a3 a4 a5 a6 a7 a8 b0 b1 b2 b3 b4 b5 b6 b7 b8 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (r7 r8 o8 : Nat) (hr7 : r7 < 65536) (hr8 : r8 < 65536) (ho : o8 < 256) (e : (a0 * b8 + a1 * b7 + a2 * b6 + a3 * b5 + a4 * b4 + a5 * b3 + a6 * b2 + a7 * b1 + a8 * b0 + r7) % p = (o8 + 256 * r8) % p) :
    a0 * b8 + a1 * b7 + a2 * b6 + a3 * b5 + a4 * b4 + a5 * b3 + a6 * b2 + a7 * b1 + a8 * b0 + r7 = o8 + 256 * r8 := by
  have q0_8 := byte_prod_le a0 b8 ha0 hb8
  have q1_7 := byte_prod_le a1 b7 ha1 hb7
  have q2_6 := byte_prod_le a2 b6 ha2 hb6
  have q3_5 := byte_prod_le a3 b5 ha3 hb5
  have q4_4 := byte_prod_le a4 b4 ha4 hb4
  have q5_3 := byte_prod_le a5 b3 ha5 hb3
  have q6_2 := byte_prod_le a6 b2 ha6 hb2
  have q7_1 := byte_prod_le a7 b1 ha7 hb1
  have q8_0 := byte_prod_le a8 b0 ha8 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col9 (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (r8 r9 o9 : Nat) (hr8 : r8 < 65536) (hr9 : r9 < 65536) (ho : o9 < 256) (e : (a0 * b9 + a1 * b8 + a2 * b7 + a3 * b6 + a4 * b5 + a5 * b4 + a6 * b3 + a7 * b2 + a8 * b1 + a9 * b0 + r8) % p = (o9 + 256 * r9) % p) :
    a0 * b9 + a1 * b8 + a2 * b7 + a3 * b6 + a4 * b5 + a5 * b4 + a6 * b3 + a7 * b2 + a8 * b1 + a9 * b0 + r8 = o9 + 256 * r9 := by
  have q0_9 := byte_prod_le a0 b9 ha0 hb9
  have q1_8 := byte_prod_le a1 b8 ha1 hb8
  have q2_7 := byte_prod_le a2 b7 ha2 hb7
  have q3_6 := byte_prod_le a3 b6 ha3 hb6
  have q4_5 := byte_prod_le a4 b5 ha4 hb5
  have q5_4 := byte_prod_le a5 b4 ha5 hb4
  have q6_3 := byte_prod_le a6 b3 ha6 hb3
  have q7_2 := byte_prod_le a7 b2 ha7 hb2
  have q8_1 := byte_prod_le a8 b1 ha8 hb1
  have q9_0 := byte_prod_le a9 b0 ha9 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col10 (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (r9 r10 o10 : Nat) (hr9 : r9 < 65536) (hr10 : r10 < 65536) (ho : o10 < 256) (e : (a0 * b10 + a1 * b9 + a2 * b8 + a3 * b7 + a4 * b6 + a5 * b5 + a6 * b4 + a7 * b3 + a8 * b2 + a9 * b1 + a10 * b0 + r9) % p = (o10 + 256 * r10) % p) :
    a0 * b10 + a1 * b9 + a2 * b8 + a3 * b7 + a4 * b6 + a5 * b5 + a6 * b4 + a7 * b3 + a8 * b2 + a9 * b1 + a10 * b0 + r9 = o10 + 256 * r10 := by
  have q0_10 := byte_prod_le a0 b10 ha0 hb10
  have q1_9 := byte_prod_le a1 b9 ha1 hb9
  have q2_8 := byte_prod_le a2 b8 ha2 hb8
  have q3_7 := byte_prod_le a3 b7 ha3 hb7
  have q4_6 := byte_prod_le a4 b6 ha4 hb6
  have q5_5 := byte_prod_le a5 b5 ha5 hb5
  have q6_4 := byte_prod_le a6 b4 ha6 hb4
  have q7_3 := byte_prod_le a7 b3 ha7 hb3
  have q8_2 := byte_prod_le a8 b2 ha8 hb2
  have q9_1 := byte_prod_le a9 b1 ha9 hb1
  have q10_0 := byte_prod_le a10 b0 ha10 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col11 (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (r10 r11 o11 : Nat) (hr10 : r10 < 65536) (hr11 : r11 < 65536) (ho : o11 < 256) (e : (a0 * b11 + a1 * b10 + a2 * b9 + a3 * b8 + a4 * b7 + a5 * b6 + a6 * b5 + a7 * b4 + a8 * b3 + a9 * b2 + a10 * b1 + a11 * b0 + r10) % p = (o11 + 256 * r11) % p) :
    a0 * b11 + a1 * b10 + a2 * b9 + a3 * b8 + a4 * b7 + a5 * b6 + a6 * b5 + a7 * b4 + a8 * b3 + a9 * b2 + a10 * b1 + a11 * b0 + r10 = o11 + 256 * r11 := by
  have q0_11 := byte_prod_le a0 b11 ha0 hb11
  have q1_10 := byte_prod_le a1 b10 ha1 hb10
  have q2_9 := byte_prod_le a2 b9 ha2 hb9
  have q3_8 := byte_prod_le a3 b8 ha3 hb8
  have q4_7 := byte_prod_le a4 b7 ha4 hb7
  have q5_6 := byte_prod_le a5 b6 ha5 hb6
  have q6_5 := byte_prod_le a6 b5 ha6 hb5
  have q7_4 := byte_prod_le a7 b4 ha7 hb4
  have q8_3 := byte_prod_le a8 b3 ha8 hb3
  have q9_2 := byte_prod_le a9 b2 ha9 hb2
  have q10_1 := byte_prod_le a10 b1 ha10 hb1
  have q11_0 := byte_prod_le a11 b0 ha11 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col12 (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (r11 r12 o12 : Nat) (hr11 : r11 < 65536) (hr12 : r12 < 65536) (ho : o12 < 256) (e : (a0 * b12 + a1 * b11 + a2 * b10 + a3 * b9 + a4 * b8 + a5 * b7 + a6 * b6 + a7 * b5 + a8 * b4 + a9 * b3 + a10 * b2 + a11 * b1 + a12 * b0 + r11) % p = (o12 + 256 * r12) % p) :
    a0 * b12 + a1 * b11 + a2 * b10 + a3 * b9 + a4 * b8 + a5 * b7 + a6 * b6 + a7 * b5 + a8 * b4 + a9 * b3 + a10 * b2 + a11 * b1 + a12 * b0 + r11 = o12 + 256 * r12 := by
  have q0_12 := byte_prod_le a0 b12 ha0 hb12
  have q1_11 := byte_prod_le a1 b11 ha1 hb11
  have q2_10 := byte_prod_le a2 b10 ha2 hb10
  have q3_9 := byte_prod_le a3 b9 ha3 hb9
  have q4_8 := byte_prod_le a4 b8 ha4 hb8
  have q5_7 := byte_prod_le a5 b7 ha5 hb7
  have q6_6 := byte_prod_le a6 b6 ha6 hb6
  have q7_5 := byte_prod_le a7 b5 ha7 hb5
  have q8_4 := byte_prod_le a8 b4 ha8 hb4
  have q9_3 := byte_prod_le a9 b3 ha9 hb3
  have q10_2 := byte_prod_le a10 b2 ha10 hb2
  have q11_1 := byte_prod_le a11 b1 ha11 hb1
  have q12_0 := byte_prod_le a12 b0 ha12 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col13 (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (r12 r13 o13 : Nat) (hr12 : r12 < 65536) (hr13 : r13 < 65536) (ho : o13 < 256) (e : (a0 * b13 + a1 * b12 + a2 * b11 + a3 * b10 + a4 * b9 + a5 * b8 + a6 * b7 + a7 * b6 + a8 * b5 + a9 * b4 + a10 * b3 + a11 * b2 + a12 * b1 + a13 * b0 + r12) % p = (o13 + 256 * r13) % p) :
    a0 * b13 + a1 * b12 + a2 * b11 + a3 * b10 + a4 * b9 + a5 * b8 + a6 * b7 + a7 * b6 + a8 * b5 + a9 * b4 + a10 * b3 + a11 * b2 + a12 * b1 + a13 * b0 + r12 = o13 + 256 * r13 := by
  have q0_13 := byte_prod_le a0 b13 ha0 hb13
  have q1_12 := byte_prod_le a1 b12 ha1 hb12
  have q2_11 := byte_prod_le a2 b11 ha2 hb11
  have q3_10 := byte_prod_le a3 b10 ha3 hb10
  have q4_9 := byte_prod_le a4 b9 ha4 hb9
  have q5_8 := byte_prod_le a5 b8 ha5 hb8
  have q6_7 := byte_prod_le a6 b7 ha6 hb7
  have q7_6 := byte_prod_le a7 b6 ha7 hb6
  have q8_5 := byte_prod_le a8 b5 ha8 hb5
  have q9_4 := byte_prod_le a9 b4 ha9 hb4
  have q10_3 := byte_prod_le a10 b3 ha10 hb3
  have q11_2 := byte_prod_le a11 b2 ha11 hb2
  have q12_1 := byte_prod_le a12 b1 ha12 hb1
  have q13_0 := byte_prod_le a13 b0 ha13 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col14 (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (r13 r14 o14 : Nat) (hr13 : r13 < 65536) (hr14 : r14 < 65536) (ho : o14 < 256) (e : (a0 * b14 + a1 * b13 + a2 * b12 + a3 * b11 + a4 * b10 + a5 * b9 + a6 * b8 + a7 * b7 + a8 * b6 + a9 * b5 + a10 * b4 + a11 * b3 + a12 * b2 + a13 * b1 + a14 * b0 + r13) % p = (o14 + 256 * r14) % p) :
    a0 * b14 + a1 * b13 + a2 * b12 + a3 * b11 + a4 * b10 + a5 * b9 + a6 * b8 + a7 * b7 + a8 * b6 + a9 * b5 + a10 * b4 + a11 * b3 + a12 * b2 + a13 * b1 + a14 * b0 + r13 = o14 + 256 * r14 := by
  have q0_14 := byte_prod_le a0 b14 ha0 hb14
  have q1_13 := byte_prod_le a1 b13 ha1 hb13
  have q2_12 := byte_prod_le a2 b12 ha2 hb12
  have q3_11 := byte_prod_le a3 b11 ha3 hb11
  have q4_10 := byte_prod_le a4 b10 ha4 hb10
  have q5_9 := byte_prod_le a5 b9 ha5 hb9
  have q6_8 := byte_prod_le a6 b8 ha6 hb8
  have q7_7 := byte_prod_le a7 b7 ha7 hb7
  have q8_6 := byte_prod_le a8 b6 ha8 hb6
  have q9_5 := byte_prod_le a9 b5 ha9 hb5
  have q10_4 := byte_prod_le a10 b4 ha10 hb4
  have q11_3 := byte_prod_le a11 b3 ha11 hb3
  have q12_2 := byte_prod_le a12 b2 ha12 hb2
  have q13_1 := byte_prod_le a13 b1 ha13 hb1
  have q14_0 := byte_prod_le a14 b0 ha14 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col15 (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha0 : a0 < 256) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb0 : b0 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r14 r15 o15 : Nat) (hr14 : r14 < 65536) (hr15 : r15 < 65536) (ho : o15 < 256) (e : (a0 * b15 + a1 * b14 + a2 * b13 + a3 * b12 + a4 * b11 + a5 * b10 + a6 * b9 + a7 * b8 + a8 * b7 + a9 * b6 + a10 * b5 + a11 * b4 + a12 * b3 + a13 * b2 + a14 * b1 + a15 * b0 + r14) % p = (o15 + 256 * r15) % p) :
    a0 * b15 + a1 * b14 + a2 * b13 + a3 * b12 + a4 * b11 + a5 * b10 + a6 * b9 + a7 * b8 + a8 * b7 + a9 * b6 + a10 * b5 + a11 * b4 + a12 * b3 + a13 * b2 + a14 * b1 + a15 * b0 + r14 = o15 + 256 * r15 := by
  have q0_15 := byte_prod_le a0 b15 ha0 hb15
  have q1_14 := byte_prod_le a1 b14 ha1 hb14
  have q2_13 := byte_prod_le a2 b13 ha2 hb13
  have q3_12 := byte_prod_le a3 b12 ha3 hb12
  have q4_11 := byte_prod_le a4 b11 ha4 hb11
  have q5_10 := byte_prod_le a5 b10 ha5 hb10
  have q6_9 := byte_prod_le a6 b9 ha6 hb9
  have q7_8 := byte_prod_le a7 b8 ha7 hb8
  have q8_7 := byte_prod_le a8 b7 ha8 hb7
  have q9_6 := byte_prod_le a9 b6 ha9 hb6
  have q10_5 := byte_prod_le a10 b5 ha10 hb5
  have q11_4 := byte_prod_le a11 b4 ha11 hb4
  have q12_3 := byte_prod_le a12 b3 ha12 hb3
  have q13_2 := byte_prod_le a13 b2 ha13 hb2
  have q14_1 := byte_prod_le a14 b1 ha14 hb1
  have q15_0 := byte_prod_le a15 b0 ha15 hb0
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col16 (a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha1 : a1 < 256) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb1 : b1 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r15 r16 o16 : Nat) (hr15 : r15 < 65536) (hr16 : r16 < 65536) (ho : o16 < 256) (e : (a1 * b15 + a2 * b14 + a3 * b13 + a4 * b12 + a5 * b11 + a6 * b10 + a7 * b9 + a8 * b8 + a9 * b7 + a10 * b6 + a11 * b5 + a12 * b4 + a13 * b3 + a14 * b2 + a15 * b1 + r15) % p = (o16 + 256 * r16) % p) :
    a1 * b15 + a2 * b14 + a3 * b13 + a4 * b12 + a5 * b11 + a6 * b10 + a7 * b9 + a8 * b8 + a9 * b7 + a10 * b6 + a11 * b5 + a12 * b4 + a13 * b3 + a14 * b2 + a15 * b1 + r15 = o16 + 256 * r16 := by
  have q1_15 := byte_prod_le a1 b15 ha1 hb15
  have q2_14 := byte_prod_le a2 b14 ha2 hb14
  have q3_13 := byte_prod_le a3 b13 ha3 hb13
  have q4_12 := byte_prod_le a4 b12 ha4 hb12
  have q5_11 := byte_prod_le a5 b11 ha5 hb11
  have q6_10 := byte_prod_le a6 b10 ha6 hb10
  have q7_9 := byte_prod_le a7 b9 ha7 hb9
  have q8_8 := byte_prod_le a8 b8 ha8 hb8
  have q9_7 := byte_prod_le a9 b7 ha9 hb7
  have q10_6 := byte_prod_le a10 b6 ha10 hb6
  have q11_5 := byte_prod_le a11 b5 ha11 hb5
  have q12_4 := byte_prod_le a12 b4 ha12 hb4
  have q13_3 := byte_prod_le a13 b3 ha13 hb3
  have q14_2 := byte_prod_le a14 b2 ha14 hb2
  have q15_1 := byte_prod_le a15 b1 ha15 hb1
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col17 (a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha2 : a2 < 256) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb2 : b2 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r16 r17 o17 : Nat) (hr16 : r16 < 65536) (hr17 : r17 < 65536) (ho : o17 < 256) (e : (a2 * b15 + a3 * b14 + a4 * b13 + a5 * b12 + a6 * b11 + a7 * b10 + a8 * b9 + a9 * b8 + a10 * b7 + a11 * b6 + a12 * b5 + a13 * b4 + a14 * b3 + a15 * b2 + r16) % p = (o17 + 256 * r17) % p) :
    a2 * b15 + a3 * b14 + a4 * b13 + a5 * b12 + a6 * b11 + a7 * b10 + a8 * b9 + a9 * b8 + a10 * b7 + a11 * b6 + a12 * b5 + a13 * b4 + a14 * b3 + a15 * b2 + r16 = o17 + 256 * r17 := by
  have q2_15 := byte_prod_le a2 b15 ha2 hb15
  have q3_14 := byte_prod_le a3 b14 ha3 hb14
  have q4_13 := byte_prod_le a4 b13 ha4 hb13
  have q5_12 := byte_prod_le a5 b12 ha5 hb12
  have q6_11 := byte_prod_le a6 b11 ha6 hb11
  have q7_10 := byte_prod_le a7 b10 ha7 hb10
  have q8_9 := byte_prod_le a8 b9 ha8 hb9
  have q9_8 := byte_prod_le a9 b8 ha9 hb8
  have q10_7 := byte_prod_le a10 b7 ha10 hb7
  have q11_6 := byte_prod_le a11 b6 ha11 hb6
  have q12_5 := byte_prod_le a12 b5 ha12 hb5
  have q13_4 := byte_prod_le a13 b4 ha13 hb4
  have q14_3 := byte_prod_le a14 b3 ha14 hb3
  have q15_2 := byte_prod_le a15 b2 ha15 hb2
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col18 (a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha3 : a3 < 256) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb3 : b3 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r17 r18 o18 : Nat) (hr17 : r17 < 65536) (hr18 : r18 < 65536) (ho : o18 < 256) (e : (a3 * b15 + a4 * b14 + a5 * b13 + a6 * b12 + a7 * b11 + a8 * b10 + a9 * b9 + a10 * b8 + a11 * b7 + a12 * b6 + a13 * b5 + a14 * b4 + a15 * b3 + r17) % p = (o18 + 256 * r18) % p) :
    a3 * b15 + a4 * b14 + a5 * b13 + a6 * b12 + a7 * b11 + a8 * b10 + a9 * b9 + a10 * b8 + a11 * b7 + a12 * b6 + a13 * b5 + a14 * b4 + a15 * b3 + r17 = o18 + 256 * r18 := by
  have q3_15 := byte_prod_le a3 b15 ha3 hb15
  have q4_14 := byte_prod_le a4 b14 ha4 hb14
  have q5_13 := byte_prod_le a5 b13 ha5 hb13
  have q6_12 := byte_prod_le a6 b12 ha6 hb12
  have q7_11 := byte_prod_le a7 b11 ha7 hb11
  have q8_10 := byte_prod_le a8 b10 ha8 hb10
  have q9_9 := byte_prod_le a9 b9 ha9 hb9
  have q10_8 := byte_prod_le a10 b8 ha10 hb8
  have q11_7 := byte_prod_le a11 b7 ha11 hb7
  have q12_6 := byte_prod_le a12 b6 ha12 hb6
  have q13_5 := byte_prod_le a13 b5 ha13 hb5
  have q14_4 := byte_prod_le a14 b4 ha14 hb4
  have q15_3 := byte_prod_le a15 b3 ha15 hb3
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col19 (a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha4 : a4 < 256) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb4 : b4 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r18 r19 o19 : Nat) (hr18 : r18 < 65536) (hr19 : r19 < 65536) (ho : o19 < 256) (e : (a4 * b15 + a5 * b14 + a6 * b13 + a7 * b12 + a8 * b11 + a9 * b10 + a10 * b9 + a11 * b8 + a12 * b7 + a13 * b6 + a14 * b5 + a15 * b4 + r18) % p = (o19 + 256 * r19) % p) :
    a4 * b15 + a5 * b14 + a6 * b13 + a7 * b12 + a8 * b11 + a9 * b10 + a10 * b9 + a11 * b8 + a12 * b7 + a13 * b6 + a14 * b5 + a15 * b4 + r18 = o19 + 256 * r19 := by
  have q4_15 := byte_prod_le a4 b15 ha4 hb15
  have q5_14 := byte_prod_le a5 b14 ha5 hb14
  have q6_13 := byte_prod_le a6 b13 ha6 hb13
  have q7_12 := byte_prod_le a7 b12 ha7 hb12
  have q8_11 := byte_prod_le a8 b11 ha8 hb11
  have q9_10 := byte_prod_le a9 b10 ha9 hb10
  have q10_9 := byte_prod_le a10 b9 ha10 hb9
  have q11_8 := byte_prod_le a11 b8 ha11 hb8
  have q12_7 := byte_prod_le a12 b7 ha12 hb7
  have q13_6 := byte_prod_le a13 b6 ha13 hb6
  have q14_5 := byte_prod_le a14 b5 ha14 hb5
  have q15_4 := byte_prod_le a15 b4 ha15 hb4
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col20 (a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha5 : a5 < 256) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb5 : b5 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r19 r20 o20 : Nat) (hr19 : r19 < 65536) (hr20 : r20 < 65536) (ho : o20 < 256) (e : (a5 * b15 + a6 * b14 + a7 * b13 + a8 * b12 + a9 * b11 + a10 * b10 + a11 * b9 + a12 * b8 + a13 * b7 + a14 * b6 + a15 * b5 + r19) % p = (o20 + 256 * r20) % p) :
    a5 * b15 + a6 * b14 + a7 * b13 + a8 * b12 + a9 * b11 + a10 * b10 + a11 * b9 + a12 * b8 + a13 * b7 + a14 * b6 + a15 * b5 + r19 = o20 + 256 * r20 := by
  have q5_15 := byte_prod_le a5 b15 ha5 hb15
  have q6_14 := byte_prod_le a6 b14 ha6 hb14
  have q7_13 := byte_prod_le a7 b13 ha7 hb13
  have q8_12 := byte_prod_le a8 b12 ha8 hb12
  have q9_11 := byte_prod_le a9 b11 ha9 hb11
  have q10_10 := byte_prod_le a10 b10 ha10 hb10
  have q11_9 := byte_prod_le a11 b9 ha11 hb9
  have q12_8 := byte_prod_le a12 b8 ha12 hb8
  have q13_7 := byte_prod_le a13 b7 ha13 hb7
  have q14_6 := byte_prod_le a14 b6 ha14 hb6
  have q15_5 := byte_prod_le a15 b5 ha15 hb5
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col21 (a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha6 : a6 < 256) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb6 : b6 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r20 r21 o21 : Nat) (hr20 : r20 < 65536) (hr21 : r21 < 65536) (ho : o21 < 256) (e : (a6 * b15 + a7 * b14 + a8 * b13 + a9 * b12 + a10 * b11 + a11 * b10 + a12 * b9 + a13 * b8 + a14 * b7 + a15 * b6 + r20) % p = (o21 + 256 * r21) % p) :
    a6 * b15 + a7 * b14 + a8 * b13 + a9 * b12 + a10 * b11 + a11 * b10 + a12 * b9 + a13 * b8 + a14 * b7 + a15 * b6 + r20 = o21 + 256 * r21 := by
  have q6_15 := byte_prod_le a6 b15 ha6 hb15
  have q7_14 := byte_prod_le a7 b14 ha7 hb14
  have q8_13 := byte_prod_le a8 b13 ha8 hb13
  have q9_12 := byte_prod_le a9 b12 ha9 hb12
  have q10_11 := byte_prod_le a10 b11 ha10 hb11
  have q11_10 := byte_prod_le a11 b10 ha11 hb10
  have q12_9 := byte_prod_le a12 b9 ha12 hb9
  have q13_8 := byte_prod_le a13 b8 ha13 hb8
  have q14_7 := byte_prod_le a14 b7 ha14 hb7
  have q15_6 := byte_prod_le a15 b6 ha15 hb6
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col22 (a7 a8 a9 a10 a11 a12 a13 a14 a15 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha7 : a7 < 256) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb7 : b7 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r21 r22 o22 : Nat) (hr21 : r21 < 65536) (hr22 : r22 < 65536) (ho : o22 < 256) (e : (a7 * b15 + a8 * b14 + a9 * b13 + a10 * b12 + a11 * b11 + a12 * b10 + a13 * b9 + a14 * b8 + a15 * b7 + r21) % p = (o22 + 256 * r22) % p) :
    a7 * b15 + a8 * b14 + a9 * b13 + a10 * b12 + a11 * b11 + a12 * b10 + a13 * b9 + a14 * b8 + a15 * b7 + r21 = o22 + 256 * r22 := by
  have q7_15 := byte_prod_le a7 b15 ha7 hb15
  have q8_14 := byte_prod_le a8 b14 ha8 hb14
  have q9_13 := byte_prod_le a9 b13 ha9 hb13
  have q10_12 := byte_prod_le a10 b12 ha10 hb12
  have q11_11 := byte_prod_le a11 b11 ha11 hb11
  have q12_10 := byte_prod_le a12 b10 ha12 hb10
  have q13_9 := byte_prod_le a13 b9 ha13 hb9
  have q14_8 := byte_prod_le a14 b8 ha14 hb8
  have q15_7 := byte_prod_le a15 b7 ha15 hb7
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col23 (a8 a9 a10 a11 a12 a13 a14 a15 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha8 : a8 < 256) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb8 : b8 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r22 r23 o23 : Nat) (hr22 : r22 < 65536) (hr23 : r23 < 65536) (ho : o23 < 256) (e : (a8 * b15 + a9 * b14 + a10 * b13 + a11 * b12 + a12 * b11 + a13 * b10 + a14 * b9 + a15 * b8 + r22) % p = (o23 + 256 * r23) % p) :
    a8 * b15 + a9 * b14 + a10 * b13 + a11 * b12 + a12 * b11 + a13 * b10 + a14 * b9 + a15 * b8 + r22 = o23 + 256 * r23 := by
  have q8_15 := byte_prod_le a8 b15 ha8 hb15
  have q9_14 := byte_prod_le a9 b14 ha9 hb14
  have q10_13 := byte_prod_le a10 b13 ha10 hb13
  have q11_12 := byte_prod_le a11 b12 ha11 hb12
  have q12_11 := byte_prod_le a12 b11 ha12 hb11
  have q13_10 := byte_prod_le a13 b10 ha13 hb10
  have q14_9 := byte_prod_le a14 b9 ha14 hb9
  have q15_8 := byte_prod_le a15 b8 ha15 hb8
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col24 (a9 a10 a11 a12 a13 a14 a15 b9 b10 b11 b12 b13 b14 b15 : Nat) (ha9 : a9 < 256) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb9 : b9 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r23 r24 o24 : Nat) (hr23 : r23 < 65536) (hr24 : r24 < 65536) (ho : o24 < 256) (e : (a9 * b15 + a10 * b14 + a11 * b13 + a12 * b12 + a13 * b11 + a14 * b10 + a15 * b9 + r23) % p = (o24 + 256 * r24) % p) :
    a9 * b15 + a10 * b14 + a11 * b13 + a12 * b12 + a13 * b11 + a14 * b10 + a15 * b9 + r23 = o24 + 256 * r24 := by
  have q9_15 := byte_prod_le a9 b15 ha9 hb15
  have q10_14 := byte_prod_le a10 b14 ha10 hb14
  have q11_13 := byte_prod_le a11 b13 ha11 hb13
  have q12_12 := byte_prod_le a12 b12 ha12 hb12
  have q13_11 := byte_prod_le a13 b11 ha13 hb11
  have q14_10 := byte_prod_le a14 b10 ha14 hb10
  have q15_9 := byte_prod_le a15 b9 ha15 hb9
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col25 (a10 a11 a12 a13 a14 a15 b10 b11 b12 b13 b14 b15 : Nat) (ha10 : a10 < 256) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb10 : b10 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r24 r25 o25 : Nat) (hr24 : r24 < 65536) (hr25 : r25 < 65536) (ho : o25 < 256) (e : (a10 * b15 + a11 * b14 + a12 * b13 + a13 * b12 + a14 * b11 + a15 * b10 + r24) % p = (o25 + 256 * r25) % p) :
    a10 * b15 + a11 * b14 + a12 * b13 + a13 * b12 + a14 * b11 + a15 * b10 + r24 = o25 + 256 * r25 := by
  have q10_15 := byte_prod_le a10 b15 ha10 hb15
  have q11_14 := byte_prod_le a11 b14 ha11 hb14
  have q12_13 := byte_prod_le a12 b13 ha12 hb13
  have q13_12 := byte_prod_le a13 b12 ha13 hb12
  have q14_11 := byte_prod_le a14 b11 ha14 hb11
  have q15_10 := byte_prod_le a15 b10 ha15 hb10
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col26 (a11 a12 a13 a14 a15 b11 b12 b13 b14 b15 : Nat) (ha11 : a11 < 256) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb11 : b11 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r25 r26 o26 : Nat) (hr25 : r25 < 65536) (hr26 : r26 < 65536) (ho : o26 < 256) (e : (a11 * b15 + a12 * b14 + a13 * b13 + a14 * b12 + a15 * b11 + r25) % p = (o26 + 256 * r26) % p) :
    a11 * b15 + a12 * b14 + a13 * b13 + a14 * b12 + a15 * b11 + r25 = o26 + 256 * r26 := by
  have q11_15 := byte_prod_le a11 b15 ha11 hb15
  have q12_14 := byte_prod_le a12 b14 ha12 hb14
  have q13_13 := byte_prod_le a13 b13 ha13 hb13
  have q14_12 := byte_prod_le a14 b12 ha14 hb12
  have q15_11 := byte_prod_le a15 b11 ha15 hb11
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col27 (a12 a13 a14 a15 b12 b13 b14 b15 : Nat) (ha12 : a12 < 256) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb12 : b12 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r26 r27 o27 : Nat) (hr26 : r26 < 65536) (hr27 : r27 < 65536) (ho : o27 < 256) (e : (a12 * b15 + a13 * b14 + a14 * b13 + a15 * b12 + r26) % p = (o27 + 256 * r27) % p) :
    a12 * b15 + a13 * b14 + a14 * b13 + a15 * b12 + r26 = o27 + 256 * r27 := by
  have q12_15 := byte_prod_le a12 b15 ha12 hb15
  have q13_14 := byte_prod_le a13 b14 ha13 hb14
  have q14_13 := byte_prod_le a14 b13 ha14 hb13
  have q15_12 := byte_prod_le a15 b12 ha15 hb12
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col28 (a13 a14 a15 b13 b14 b15 : Nat) (ha13 : a13 < 256) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb13 : b13 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r27 r28 o28 : Nat) (hr27 : r27 < 65536) (hr28 : r28 < 65536) (ho : o28 < 256) (e : (a13 * b15 + a14 * b14 + a15 * b13 + r27) % p = (o28 + 256 * r28) % p) :
    a13 * b15 + a14 * b14 + a15 * b13 + r27 = o28 + 256 * r28 := by
  have q13_15 := byte_prod_le a13 b15 ha13 hb15
  have q14_14 := byte_prod_le a14 b14 ha14 hb14
  have q15_13 := byte_prod_le a15 b13 ha15 hb13
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col29 (a14 a15 b14 b15 : Nat) (ha14 : a14 < 256) (ha15 : a15 < 256) (hb14 : b14 < 256) (hb15 : b15 < 256) (r28 r29 o29 : Nat) (hr28 : r28 < 65536) (hr29 : r29 < 65536) (ho : o29 < 256) (e : (a14 * b15 + a15 * b14 + r28) % p = (o29 + 256 * r29) % p) :
    a14 * b15 + a15 * b14 + r28 = o29 + 256 * r29 := by
  have q14_15 := byte_prod_le a14 b15 ha14 hb15
  have q15_14 := byte_prod_le a15 b14 ha15 hb14
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col30 (a15 b15 : Nat) (ha15 : a15 < 256) (hb15 : b15 < 256) (r29 r30 o30 : Nat) (hr29 : r29 < 65536) (hr30 : r30 < 65536) (ho : o30 < 256) (e : (a15 * b15 + r29) % p = (o30 + 256 * r30) % p) :
    a15 * b15 + r29 = o30 + 256 * r30 := by
  have q15_15 := byte_prod_le a15 b15 ha15 hb15
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

theorem mul_col31 (r30 o31 : Nat) (hr30 : r30 < 65536) (ho : o31 < 256) (e : (0 + r30) % p = (o31) % p) :
    0 + r30 = o31 := by
  exact no_wrap (by unfold p; omega) (by unfold p; omega) e

set_option maxHeartbeats 4000000 in
/-- MUL row: `z = x y mod 2^128` and `flag = [x y >= 2^128]` (operands as their ranged bytes; the limb links are `byte_link`), for every field value of `minv`. -/
theorem mul_row
    (a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 : Nat) (b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 : Nat) (o0 o1 o2 o3 o4 o5 o6 o7 o8 o9 o10 o11 o12 o13 o14 o15 o16 o17 o18 o19 o20 o21 o22 o23 o24 o25 o26 o27 o28 o29 o30 o31 : Nat) (r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14 r15 r16 r17 r18 r19 r20 r21 r22 r23 r24 r25 r26 r27 r28 r29 r30 : Nat) (f minv : Nat)
    (ha0 : a0 < 256)
    (ha1 : a1 < 256)
    (ha2 : a2 < 256)
    (ha3 : a3 < 256)
    (ha4 : a4 < 256)
    (ha5 : a5 < 256)
    (ha6 : a6 < 256)
    (ha7 : a7 < 256)
    (ha8 : a8 < 256)
    (ha9 : a9 < 256)
    (ha10 : a10 < 256)
    (ha11 : a11 < 256)
    (ha12 : a12 < 256)
    (ha13 : a13 < 256)
    (ha14 : a14 < 256)
    (ha15 : a15 < 256)
    (hb0 : b0 < 256)
    (hb1 : b1 < 256)
    (hb2 : b2 < 256)
    (hb3 : b3 < 256)
    (hb4 : b4 < 256)
    (hb5 : b5 < 256)
    (hb6 : b6 < 256)
    (hb7 : b7 < 256)
    (hb8 : b8 < 256)
    (hb9 : b9 < 256)
    (hb10 : b10 < 256)
    (hb11 : b11 < 256)
    (hb12 : b12 < 256)
    (hb13 : b13 < 256)
    (hb14 : b14 < 256)
    (hb15 : b15 < 256)
    (ho0 : o0 < 256)
    (ho1 : o1 < 256)
    (ho2 : o2 < 256)
    (ho3 : o3 < 256)
    (ho4 : o4 < 256)
    (ho5 : o5 < 256)
    (ho6 : o6 < 256)
    (ho7 : o7 < 256)
    (ho8 : o8 < 256)
    (ho9 : o9 < 256)
    (ho10 : o10 < 256)
    (ho11 : o11 < 256)
    (ho12 : o12 < 256)
    (ho13 : o13 < 256)
    (ho14 : o14 < 256)
    (ho15 : o15 < 256)
    (ho16 : o16 < 256)
    (ho17 : o17 < 256)
    (ho18 : o18 < 256)
    (ho19 : o19 < 256)
    (ho20 : o20 < 256)
    (ho21 : o21 < 256)
    (ho22 : o22 < 256)
    (ho23 : o23 < 256)
    (ho24 : o24 < 256)
    (ho25 : o25 < 256)
    (ho26 : o26 < 256)
    (ho27 : o27 < 256)
    (ho28 : o28 < 256)
    (ho29 : o29 < 256)
    (ho30 : o30 < 256)
    (ho31 : o31 < 256)
    (hr0 : r0 < 65536)
    (hr1 : r1 < 65536)
    (hr2 : r2 < 65536)
    (hr3 : r3 < 65536)
    (hr4 : r4 < 65536)
    (hr5 : r5 < 65536)
    (hr6 : r6 < 65536)
    (hr7 : r7 < 65536)
    (hr8 : r8 < 65536)
    (hr9 : r9 < 65536)
    (hr10 : r10 < 65536)
    (hr11 : r11 < 65536)
    (hr12 : r12 < 65536)
    (hr13 : r13 < 65536)
    (hr14 : r14 < 65536)
    (hr15 : r15 < 65536)
    (hr16 : r16 < 65536)
    (hr17 : r17 < 65536)
    (hr18 : r18 < 65536)
    (hr19 : r19 < 65536)
    (hr20 : r20 < 65536)
    (hr21 : r21 < 65536)
    (hr22 : r22 < 65536)
    (hr23 : r23 < 65536)
    (hr24 : r24 < 65536)
    (hr25 : r25 < 65536)
    (hr26 : r26 < 65536)
    (hr27 : r27 < 65536)
    (hr28 : r28 < 65536)
    (hr29 : r29 < 65536)
    (hr30 : r30 < 65536)
    (e0 : (a0 * b0) % p = (o0 + 256 * r0) % p)
    (e1 : (a0 * b1 + a1 * b0 + r0) % p = (o1 + 256 * r1) % p)
    (e2 : (a0 * b2 + a1 * b1 + a2 * b0 + r1) % p = (o2 + 256 * r2) % p)
    (e3 : (a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0 + r2) % p = (o3 + 256 * r3) % p)
    (e4 : (a0 * b4 + a1 * b3 + a2 * b2 + a3 * b1 + a4 * b0 + r3) % p = (o4 + 256 * r4) % p)
    (e5 : (a0 * b5 + a1 * b4 + a2 * b3 + a3 * b2 + a4 * b1 + a5 * b0 + r4) % p = (o5 + 256 * r5) % p)
    (e6 : (a0 * b6 + a1 * b5 + a2 * b4 + a3 * b3 + a4 * b2 + a5 * b1 + a6 * b0 + r5) % p = (o6 + 256 * r6) % p)
    (e7 : (a0 * b7 + a1 * b6 + a2 * b5 + a3 * b4 + a4 * b3 + a5 * b2 + a6 * b1 + a7 * b0 + r6) % p = (o7 + 256 * r7) % p)
    (e8 : (a0 * b8 + a1 * b7 + a2 * b6 + a3 * b5 + a4 * b4 + a5 * b3 + a6 * b2 + a7 * b1 + a8 * b0 + r7) % p = (o8 + 256 * r8) % p)
    (e9 : (a0 * b9 + a1 * b8 + a2 * b7 + a3 * b6 + a4 * b5 + a5 * b4 + a6 * b3 + a7 * b2 + a8 * b1 + a9 * b0 + r8) % p = (o9 + 256 * r9) % p)
    (e10 : (a0 * b10 + a1 * b9 + a2 * b8 + a3 * b7 + a4 * b6 + a5 * b5 + a6 * b4 + a7 * b3 + a8 * b2 + a9 * b1 + a10 * b0 + r9) % p = (o10 + 256 * r10) % p)
    (e11 : (a0 * b11 + a1 * b10 + a2 * b9 + a3 * b8 + a4 * b7 + a5 * b6 + a6 * b5 + a7 * b4 + a8 * b3 + a9 * b2 + a10 * b1 + a11 * b0 + r10) % p = (o11 + 256 * r11) % p)
    (e12 : (a0 * b12 + a1 * b11 + a2 * b10 + a3 * b9 + a4 * b8 + a5 * b7 + a6 * b6 + a7 * b5 + a8 * b4 + a9 * b3 + a10 * b2 + a11 * b1 + a12 * b0 + r11) % p = (o12 + 256 * r12) % p)
    (e13 : (a0 * b13 + a1 * b12 + a2 * b11 + a3 * b10 + a4 * b9 + a5 * b8 + a6 * b7 + a7 * b6 + a8 * b5 + a9 * b4 + a10 * b3 + a11 * b2 + a12 * b1 + a13 * b0 + r12) % p = (o13 + 256 * r13) % p)
    (e14 : (a0 * b14 + a1 * b13 + a2 * b12 + a3 * b11 + a4 * b10 + a5 * b9 + a6 * b8 + a7 * b7 + a8 * b6 + a9 * b5 + a10 * b4 + a11 * b3 + a12 * b2 + a13 * b1 + a14 * b0 + r13) % p = (o14 + 256 * r14) % p)
    (e15 : (a0 * b15 + a1 * b14 + a2 * b13 + a3 * b12 + a4 * b11 + a5 * b10 + a6 * b9 + a7 * b8 + a8 * b7 + a9 * b6 + a10 * b5 + a11 * b4 + a12 * b3 + a13 * b2 + a14 * b1 + a15 * b0 + r14) % p = (o15 + 256 * r15) % p)
    (e16 : (a1 * b15 + a2 * b14 + a3 * b13 + a4 * b12 + a5 * b11 + a6 * b10 + a7 * b9 + a8 * b8 + a9 * b7 + a10 * b6 + a11 * b5 + a12 * b4 + a13 * b3 + a14 * b2 + a15 * b1 + r15) % p = (o16 + 256 * r16) % p)
    (e17 : (a2 * b15 + a3 * b14 + a4 * b13 + a5 * b12 + a6 * b11 + a7 * b10 + a8 * b9 + a9 * b8 + a10 * b7 + a11 * b6 + a12 * b5 + a13 * b4 + a14 * b3 + a15 * b2 + r16) % p = (o17 + 256 * r17) % p)
    (e18 : (a3 * b15 + a4 * b14 + a5 * b13 + a6 * b12 + a7 * b11 + a8 * b10 + a9 * b9 + a10 * b8 + a11 * b7 + a12 * b6 + a13 * b5 + a14 * b4 + a15 * b3 + r17) % p = (o18 + 256 * r18) % p)
    (e19 : (a4 * b15 + a5 * b14 + a6 * b13 + a7 * b12 + a8 * b11 + a9 * b10 + a10 * b9 + a11 * b8 + a12 * b7 + a13 * b6 + a14 * b5 + a15 * b4 + r18) % p = (o19 + 256 * r19) % p)
    (e20 : (a5 * b15 + a6 * b14 + a7 * b13 + a8 * b12 + a9 * b11 + a10 * b10 + a11 * b9 + a12 * b8 + a13 * b7 + a14 * b6 + a15 * b5 + r19) % p = (o20 + 256 * r20) % p)
    (e21 : (a6 * b15 + a7 * b14 + a8 * b13 + a9 * b12 + a10 * b11 + a11 * b10 + a12 * b9 + a13 * b8 + a14 * b7 + a15 * b6 + r20) % p = (o21 + 256 * r21) % p)
    (e22 : (a7 * b15 + a8 * b14 + a9 * b13 + a10 * b12 + a11 * b11 + a12 * b10 + a13 * b9 + a14 * b8 + a15 * b7 + r21) % p = (o22 + 256 * r22) % p)
    (e23 : (a8 * b15 + a9 * b14 + a10 * b13 + a11 * b12 + a12 * b11 + a13 * b10 + a14 * b9 + a15 * b8 + r22) % p = (o23 + 256 * r23) % p)
    (e24 : (a9 * b15 + a10 * b14 + a11 * b13 + a12 * b12 + a13 * b11 + a14 * b10 + a15 * b9 + r23) % p = (o24 + 256 * r24) % p)
    (e25 : (a10 * b15 + a11 * b14 + a12 * b13 + a13 * b12 + a14 * b11 + a15 * b10 + r24) % p = (o25 + 256 * r25) % p)
    (e26 : (a11 * b15 + a12 * b14 + a13 * b13 + a14 * b12 + a15 * b11 + r25) % p = (o26 + 256 * r26) % p)
    (e27 : (a12 * b15 + a13 * b14 + a14 * b13 + a15 * b12 + r26) % p = (o27 + 256 * r27) % p)
    (e28 : (a13 * b15 + a14 * b14 + a15 * b13 + r27) % p = (o28 + 256 * r28) % p)
    (e29 : (a14 * b15 + a15 * b14 + r28) % p = (o29 + 256 * r29) % p)
    (e30 : (a15 * b15 + r29) % p = (o30 + 256 * r30) % p)
    (e31 : (0 + r30) % p = (o31) % p)
    (hf : f < p) (bf : f * f % p = f % p)
    (hz16 : (1 + p - f) * o16 % p = 0)
    (hz17 : (1 + p - f) * o17 % p = 0)
    (hz18 : (1 + p - f) * o18 % p = 0)
    (hz19 : (1 + p - f) * o19 % p = 0)
    (hz20 : (1 + p - f) * o20 % p = 0)
    (hz21 : (1 + p - f) * o21 % p = 0)
    (hz22 : (1 + p - f) * o22 % p = 0)
    (hz23 : (1 + p - f) * o23 % p = 0)
    (hz24 : (1 + p - f) * o24 % p = 0)
    (hz25 : (1 + p - f) * o25 % p = 0)
    (hz26 : (1 + p - f) * o26 % p = 0)
    (hz27 : (1 + p - f) * o27 % p = 0)
    (hz28 : (1 + p - f) * o28 % p = 0)
    (hz29 : (1 + p - f) * o29 % p = 0)
    (hz30 : (1 + p - f) * o30 % p = 0)
    (hz31 : (1 + p - f) * o31 % p = 0)
    (hm : f * ((o16 + o17 + o18 + o19 + o20 + o21 + o22 + o23 + o24 + o25 + o26 + o27 + o28 + o29 + o30 + o31) * minv) % p = f % p) :
    (o0 + 256 * o1 + 65536 * o2 + 16777216 * o3 + 4294967296 * o4 + 1099511627776 * o5 + 281474976710656 * o6 + 72057594037927936 * o7 + 18446744073709551616 * o8 + 4722366482869645213696 * o9 + 1208925819614629174706176 * o10 + 309485009821345068724781056 * o11 + 79228162514264337593543950336 * o12 + 20282409603651670423947251286016 * o13 + 5192296858534827628530496329220096 * o14 + 1329227995784915872903807060280344576 * o15) = ((1 * a0 + 256 * a1 + 65536 * a2 + 16777216 * a3 + 4294967296 * a4 + 1099511627776 * a5 + 281474976710656 * a6 + 72057594037927936 * a7 + 18446744073709551616 * a8 + 4722366482869645213696 * a9 + 1208925819614629174706176 * a10 + 309485009821345068724781056 * a11 + 79228162514264337593543950336 * a12 + 20282409603651670423947251286016 * a13 + 5192296858534827628530496329220096 * a14 + 1329227995784915872903807060280344576 * a15) * (1 * b0 + 256 * b1 + 65536 * b2 + 16777216 * b3 + 4294967296 * b4 + 1099511627776 * b5 + 281474976710656 * b6 + 72057594037927936 * b7 + 18446744073709551616 * b8 + 4722366482869645213696 * b9 + 1208925819614629174706176 * b10 + 309485009821345068724781056 * b11 + 79228162514264337593543950336 * b12 + 20282409603651670423947251286016 * b13 + 5192296858534827628530496329220096 * b14 + 1329227995784915872903807060280344576 * b15)) % 2^128 ∧ (f = 1 ↔ (1 * a0 + 256 * a1 + 65536 * a2 + 16777216 * a3 + 4294967296 * a4 + 1099511627776 * a5 + 281474976710656 * a6 + 72057594037927936 * a7 + 18446744073709551616 * a8 + 4722366482869645213696 * a9 + 1208925819614629174706176 * a10 + 309485009821345068724781056 * a11 + 79228162514264337593543950336 * a12 + 20282409603651670423947251286016 * a13 + 5192296858534827628530496329220096 * a14 + 1329227995784915872903807060280344576 * a15) * (1 * b0 + 256 * b1 + 65536 * b2 + 16777216 * b3 + 4294967296 * b4 + 1099511627776 * b5 + 281474976710656 * b6 + 72057594037927936 * b7 + 18446744073709551616 * b8 + 4722366482869645213696 * b9 + 1208925819614629174706176 * b10 + 309485009821345068724781056 * b11 + 79228162514264337593543950336 * b12 + 20282409603651670423947251286016 * b13 + 5192296858534827628530496329220096 * b14 + 1329227995784915872903807060280344576 * b15) ≥ 2^128) := by
  have n0 := mul_col0 a0 b0 ha0 hb0 r0 o0 hr0 ho0 e0
  have n1 := mul_col1 a0 a1 b0 b1 ha0 ha1 hb0 hb1 r0 r1 o1 hr0 hr1 ho1 e1
  have n2 := mul_col2 a0 a1 a2 b0 b1 b2 ha0 ha1 ha2 hb0 hb1 hb2 r1 r2 o2 hr1 hr2 ho2 e2
  have n3 := mul_col3 a0 a1 a2 a3 b0 b1 b2 b3 ha0 ha1 ha2 ha3 hb0 hb1 hb2 hb3 r2 r3 o3 hr2 hr3 ho3 e3
  have n4 := mul_col4 a0 a1 a2 a3 a4 b0 b1 b2 b3 b4 ha0 ha1 ha2 ha3 ha4 hb0 hb1 hb2 hb3 hb4 r3 r4 o4 hr3 hr4 ho4 e4
  have n5 := mul_col5 a0 a1 a2 a3 a4 a5 b0 b1 b2 b3 b4 b5 ha0 ha1 ha2 ha3 ha4 ha5 hb0 hb1 hb2 hb3 hb4 hb5 r4 r5 o5 hr4 hr5 ho5 e5
  have n6 := mul_col6 a0 a1 a2 a3 a4 a5 a6 b0 b1 b2 b3 b4 b5 b6 ha0 ha1 ha2 ha3 ha4 ha5 ha6 hb0 hb1 hb2 hb3 hb4 hb5 hb6 r5 r6 o6 hr5 hr6 ho6 e6
  have n7 := mul_col7 a0 a1 a2 a3 a4 a5 a6 a7 b0 b1 b2 b3 b4 b5 b6 b7 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 r6 r7 o7 hr6 hr7 ho7 e7
  have n8 := mul_col8 a0 a1 a2 a3 a4 a5 a6 a7 a8 b0 b1 b2 b3 b4 b5 b6 b7 b8 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 r7 r8 o8 hr7 hr8 ho8 e8
  have n9 := mul_col9 a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 r8 r9 o9 hr8 hr9 ho9 e9
  have n10 := mul_col10 a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 r9 r10 o10 hr9 hr10 ho10 e10
  have n11 := mul_col11 a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 r10 r11 o11 hr10 hr11 ho11 e11
  have n12 := mul_col12 a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 r11 r12 o12 hr11 hr12 ho12 e12
  have n13 := mul_col13 a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 r12 r13 o13 hr12 hr13 ho13 e13
  have n14 := mul_col14 a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 r13 r14 o14 hr13 hr14 ho14 e14
  have n15 := mul_col15 a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha0 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb0 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r14 r15 o15 hr14 hr15 ho15 e15
  have n16 := mul_col16 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha1 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb1 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r15 r16 o16 hr15 hr16 ho16 e16
  have n17 := mul_col17 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha2 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb2 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r16 r17 o17 hr16 hr17 ho17 e17
  have n18 := mul_col18 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha3 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb3 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r17 r18 o18 hr17 hr18 ho18 e18
  have n19 := mul_col19 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha4 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb4 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r18 r19 o19 hr18 hr19 ho19 e19
  have n20 := mul_col20 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha5 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb5 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r19 r20 o20 hr19 hr20 ho20 e20
  have n21 := mul_col21 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha6 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb6 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r20 r21 o21 hr20 hr21 ho21 e21
  have n22 := mul_col22 a7 a8 a9 a10 a11 a12 a13 a14 a15 b7 b8 b9 b10 b11 b12 b13 b14 b15 ha7 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb7 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r21 r22 o22 hr21 hr22 ho22 e22
  have n23 := mul_col23 a8 a9 a10 a11 a12 a13 a14 a15 b8 b9 b10 b11 b12 b13 b14 b15 ha8 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb8 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r22 r23 o23 hr22 hr23 ho23 e23
  have n24 := mul_col24 a9 a10 a11 a12 a13 a14 a15 b9 b10 b11 b12 b13 b14 b15 ha9 ha10 ha11 ha12 ha13 ha14 ha15 hb9 hb10 hb11 hb12 hb13 hb14 hb15 r23 r24 o24 hr23 hr24 ho24 e24
  have n25 := mul_col25 a10 a11 a12 a13 a14 a15 b10 b11 b12 b13 b14 b15 ha10 ha11 ha12 ha13 ha14 ha15 hb10 hb11 hb12 hb13 hb14 hb15 r24 r25 o25 hr24 hr25 ho25 e25
  have n26 := mul_col26 a11 a12 a13 a14 a15 b11 b12 b13 b14 b15 ha11 ha12 ha13 ha14 ha15 hb11 hb12 hb13 hb14 hb15 r25 r26 o26 hr25 hr26 ho26 e26
  have n27 := mul_col27 a12 a13 a14 a15 b12 b13 b14 b15 ha12 ha13 ha14 ha15 hb12 hb13 hb14 hb15 r26 r27 o27 hr26 hr27 ho27 e27
  have n28 := mul_col28 a13 a14 a15 b13 b14 b15 ha13 ha14 ha15 hb13 hb14 hb15 r27 r28 o28 hr27 hr28 ho28 e28
  have n29 := mul_col29 a14 a15 b14 b15 ha14 ha15 hb14 hb15 r28 r29 o29 hr28 hr29 ho29 e29
  have n30 := mul_col30 a15 b15 ha15 hb15 r29 r30 o30 hr29 hr30 ho30 e30
  have n31 := mul_col31   r30 o31 hr30 ho31 e31
  have sb := schoolbook a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15
  have ms := mul_spec ((1 * a0 + 256 * a1 + 65536 * a2 + 16777216 * a3 + 4294967296 * a4 + 1099511627776 * a5 + 281474976710656 * a6 + 72057594037927936 * a7 + 18446744073709551616 * a8 + 4722366482869645213696 * a9 + 1208925819614629174706176 * a10 + 309485009821345068724781056 * a11 + 79228162514264337593543950336 * a12 + 20282409603651670423947251286016 * a13 + 5192296858534827628530496329220096 * a14 + 1329227995784915872903807060280344576 * a15) * (1 * b0 + 256 * b1 + 65536 * b2 + 16777216 * b3 + 4294967296 * b4 + 1099511627776 * b5 + 281474976710656 * b6 + 72057594037927936 * b7 + 18446744073709551616 * b8 + 4722366482869645213696 * b9 + 1208925819614629174706176 * b10 + 309485009821345068724781056 * b11 + 79228162514264337593543950336 * b12 + 20282409603651670423947251286016 * b13 + 5192296858534827628530496329220096 * b14 + 1329227995784915872903807060280344576 * b15)) (a0 * b0) (a0 * b1 + a1 * b0) (a0 * b2 + a1 * b1 + a2 * b0) (a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0) (a0 * b4 + a1 * b3 + a2 * b2 + a3 * b1 + a4 * b0) (a0 * b5 + a1 * b4 + a2 * b3 + a3 * b2 + a4 * b1 + a5 * b0) (a0 * b6 + a1 * b5 + a2 * b4 + a3 * b3 + a4 * b2 + a5 * b1 + a6 * b0) (a0 * b7 + a1 * b6 + a2 * b5 + a3 * b4 + a4 * b3 + a5 * b2 + a6 * b1 + a7 * b0) (a0 * b8 + a1 * b7 + a2 * b6 + a3 * b5 + a4 * b4 + a5 * b3 + a6 * b2 + a7 * b1 + a8 * b0) (a0 * b9 + a1 * b8 + a2 * b7 + a3 * b6 + a4 * b5 + a5 * b4 + a6 * b3 + a7 * b2 + a8 * b1 + a9 * b0) (a0 * b10 + a1 * b9 + a2 * b8 + a3 * b7 + a4 * b6 + a5 * b5 + a6 * b4 + a7 * b3 + a8 * b2 + a9 * b1 + a10 * b0) (a0 * b11 + a1 * b10 + a2 * b9 + a3 * b8 + a4 * b7 + a5 * b6 + a6 * b5 + a7 * b4 + a8 * b3 + a9 * b2 + a10 * b1 + a11 * b0) (a0 * b12 + a1 * b11 + a2 * b10 + a3 * b9 + a4 * b8 + a5 * b7 + a6 * b6 + a7 * b5 + a8 * b4 + a9 * b3 + a10 * b2 + a11 * b1 + a12 * b0) (a0 * b13 + a1 * b12 + a2 * b11 + a3 * b10 + a4 * b9 + a5 * b8 + a6 * b7 + a7 * b6 + a8 * b5 + a9 * b4 + a10 * b3 + a11 * b2 + a12 * b1 + a13 * b0) (a0 * b14 + a1 * b13 + a2 * b12 + a3 * b11 + a4 * b10 + a5 * b9 + a6 * b8 + a7 * b7 + a8 * b6 + a9 * b5 + a10 * b4 + a11 * b3 + a12 * b2 + a13 * b1 + a14 * b0) (a0 * b15 + a1 * b14 + a2 * b13 + a3 * b12 + a4 * b11 + a5 * b10 + a6 * b9 + a7 * b8 + a8 * b7 + a9 * b6 + a10 * b5 + a11 * b4 + a12 * b3 + a13 * b2 + a14 * b1 + a15 * b0) (a1 * b15 + a2 * b14 + a3 * b13 + a4 * b12 + a5 * b11 + a6 * b10 + a7 * b9 + a8 * b8 + a9 * b7 + a10 * b6 + a11 * b5 + a12 * b4 + a13 * b3 + a14 * b2 + a15 * b1) (a2 * b15 + a3 * b14 + a4 * b13 + a5 * b12 + a6 * b11 + a7 * b10 + a8 * b9 + a9 * b8 + a10 * b7 + a11 * b6 + a12 * b5 + a13 * b4 + a14 * b3 + a15 * b2) (a3 * b15 + a4 * b14 + a5 * b13 + a6 * b12 + a7 * b11 + a8 * b10 + a9 * b9 + a10 * b8 + a11 * b7 + a12 * b6 + a13 * b5 + a14 * b4 + a15 * b3) (a4 * b15 + a5 * b14 + a6 * b13 + a7 * b12 + a8 * b11 + a9 * b10 + a10 * b9 + a11 * b8 + a12 * b7 + a13 * b6 + a14 * b5 + a15 * b4) (a5 * b15 + a6 * b14 + a7 * b13 + a8 * b12 + a9 * b11 + a10 * b10 + a11 * b9 + a12 * b8 + a13 * b7 + a14 * b6 + a15 * b5) (a6 * b15 + a7 * b14 + a8 * b13 + a9 * b12 + a10 * b11 + a11 * b10 + a12 * b9 + a13 * b8 + a14 * b7 + a15 * b6) (a7 * b15 + a8 * b14 + a9 * b13 + a10 * b12 + a11 * b11 + a12 * b10 + a13 * b9 + a14 * b8 + a15 * b7) (a8 * b15 + a9 * b14 + a10 * b13 + a11 * b12 + a12 * b11 + a13 * b10 + a14 * b9 + a15 * b8) (a9 * b15 + a10 * b14 + a11 * b13 + a12 * b12 + a13 * b11 + a14 * b10 + a15 * b9) (a10 * b15 + a11 * b14 + a12 * b13 + a13 * b12 + a14 * b11 + a15 * b10) (a11 * b15 + a12 * b14 + a13 * b13 + a14 * b12 + a15 * b11) (a12 * b15 + a13 * b14 + a14 * b13 + a15 * b12) (a13 * b15 + a14 * b14 + a15 * b13) (a14 * b15 + a15 * b14) (a15 * b15) o0 o1 o2 o3 o4 o5 o6 o7 o8 o9 o10 o11 o12 o13 o14 o15 o16 o17 o18 o19 o20 o21 o22 o23 o24 o25 o26 o27 o28 o29 o30 o31 r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14 r15 r16 r17 r18 r19 r20 r21 r22 r23 r24 r25 r26 r27 r28 r29 r30 sb ho0 ho1 ho2 ho3 ho4 ho5 ho6 ho7 ho8 ho9 ho10 ho11 ho12 ho13 ho14 ho15 ho16 ho17 ho18 ho19 ho20 ho21 ho22 ho23 ho24 ho25 ho26 ho27 ho28 ho29 ho30 ho31 n0 n1 n2 n3 n4 n5 n6 n7 n8 n9 n10 n11 n12 n13 n14 n15 n16 n17 n18 n19 n20 n21 n22 n23 n24 n25 n26 n27 n28 n29 n30 n31
  have kf := field_bool f hf bf
  have zz16 := high_zero f o16 hz16 ho16
  have zz17 := high_zero f o17 hz17 ho17
  have zz18 := high_zero f o18 hz18 ho18
  have zz19 := high_zero f o19 hz19 ho19
  have zz20 := high_zero f o20 hz20 ho20
  have zz21 := high_zero f o21 hz21 ho21
  have zz22 := high_zero f o22 hz22 ho22
  have zz23 := high_zero f o23 hz23 ho23
  have zz24 := high_zero f o24 hz24 ho24
  have zz25 := high_zero f o25 hz25 ho25
  have zz26 := high_zero f o26 hz26 ho26
  have zz27 := high_zero f o27 hz27 ho27
  have zz28 := high_zero f o28 hz28 ho28
  have zz29 := high_zero f o29 hz29 ho29
  have zz30 := high_zero f o30 hz30 ho30
  have zz31 := high_zero f o31 hz31 ho31
  have mz := mf_flag f _ minv hm
  clear sb bf hm e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 e10 e11 e12 e13 e14 e15 e16 e17 e18 e19 e20 e21 e22 e23 e24 e25 e26 e27 e28 e29 e30 e31 n0 n1 n2 n3 n4 n5 n6 n7 n8 n9 n10 n11 n12 n13 n14 n15 n16 n17 n18 n19 n20 n21 n22 n23 n24 n25 n26 n27 n28 n29 n30 n31 hz16 hz17 hz18 hz19 hz20 hz21 hz22 hz23 hz24 hz25 hz26 hz27 hz28 hz29 hz30 hz31
  generalize (1 * a0 + 256 * a1 + 65536 * a2 + 16777216 * a3 + 4294967296 * a4 + 1099511627776 * a5 + 281474976710656 * a6 + 72057594037927936 * a7 + 18446744073709551616 * a8 + 4722366482869645213696 * a9 + 1208925819614629174706176 * a10 + 309485009821345068724781056 * a11 + 79228162514264337593543950336 * a12 + 20282409603651670423947251286016 * a13 + 5192296858534827628530496329220096 * a14 + 1329227995784915872903807060280344576 * a15) * (1 * b0 + 256 * b1 + 65536 * b2 + 16777216 * b3 + 4294967296 * b4 + 1099511627776 * b5 + 281474976710656 * b6 + 72057594037927936 * b7 + 18446744073709551616 * b8 + 4722366482869645213696 * b9 + 1208925819614629174706176 * b10 + 309485009821345068724781056 * b11 + 79228162514264337593543950336 * b12 + 20282409603651670423947251286016 * b13 + 5192296858534827628530496329220096 * b14 + 1329227995784915872903807060280344576 * b15) = P at ms ⊢
  rcases (show f = 0 ∨ f = 1 by omega) with h0 | h1
  · have y16 := zz16 h0
    have y17 := zz17 h0
    have y18 := zz18 h0
    have y19 := zz19 h0
    have y20 := zz20 h0
    have y21 := zz21 h0
    have y22 := zz22 h0
    have y23 := zz23 h0
    have y24 := zz24 h0
    have y25 := zz25 h0
    have y26 := zz26 h0
    have y27 := zz27 h0
    have y28 := zz28 h0
    have y29 := zz29 h0
    have y30 := zz30 h0
    have y31 := zz31 h0
    subst h0
    obtain ⟨m1, m2⟩ := ms
    refine ⟨by omega, ?_⟩
    constructor
    · intro h; omega
    · intro h; omega
  · have y := mz h1
    subst h1
    obtain ⟨m1, m2⟩ := ms
    refine ⟨by omega, ?_⟩
    constructor
    · intro h; omega
    · intro h; omega

end Clean.Sherlock.AluRange

#print axioms Clean.Sherlock.AluRange.chain_sound
#print axioms Clean.Sherlock.AluRange.add_spec
#print axioms Clean.Sherlock.AluRange.sub_spec
#print axioms Clean.Sherlock.AluRange.lt_spec
#print axioms Clean.Sherlock.AluRange.chain_unique
#print axioms Clean.Sherlock.AluRange.limbs_inj
#print axioms Clean.Sherlock.AluRange.mul_column_no_wrap
#print axioms Clean.Sherlock.AluRange.mul_chain_unique
#print axioms Clean.Sherlock.AluRange.mul_flag_unique
#print axioms Clean.Sherlock.AluRange.eq_spec
#print axioms Clean.Sherlock.AluRange.schoolbook
#print axioms Clean.Sherlock.AluRange.mul_spec
#print axioms Clean.Sherlock.AluRange.no_zero_div
#print axioms Clean.Sherlock.AluRange.field_bool
#print axioms Clean.Sherlock.AluRange.add_row
#print axioms Clean.Sherlock.AluRange.sub_row
#print axioms Clean.Sherlock.AluRange.lt_row
#print axioms Clean.Sherlock.AluRange.eq_row
#print axioms Clean.Sherlock.AluRange.mul_row
