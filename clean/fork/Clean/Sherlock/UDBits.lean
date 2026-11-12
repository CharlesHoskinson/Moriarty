import Clean.Sherlock.FrLog

/-!
Gate G4: the parameter check of a WHIR layer in the unique decoding regime (list size 1), with
every error term as an exact rational and every bit count as a kernel-checked integer fact.

The results whose conditions this file checks (statements as recorded in
`research/sherlock-design-paper-review-20261111/G5-PAPER-MAP.md`, which reads the full papers):

* WHIR (ePrint 2024/1586) Lemma 4.10 and Corollary 4.11 (p.23): mutual correlated agreement of
  `Gen(2; alpha)` in unique decoding, `B* = (1 + rho)/2`, `err* = 2^m' / (rho |F|)` for the code
  `RS[F, L, m']`. Theorem 4.8, unique decoding branch (p.23): the same error at radius below
  `(1 - rho)/2`.
* WHIR Theorem 5.2 (p.34) with list size 1: `eps_fold(i,s) <= d l / |F| + err*` with `d = 3`;
  `eps_shift(i) <= (1 - delta)^t + l (t + 1)/|F|`; `eps_fin <= (1 - delta)^t`; the `gamma` term
  `l (t + s)/|F|` (Claim 5.4, p.35). Hypotheses H3 (`delta_i < 1 - B*`) and H4 (list size 1) hold
  for `delta_i < (1 - rho_i)/2`.
* WHIR Lemma 4.25 (p.31): `s` OOD samples drawn together, error `(l^2/2) (2^m_i / |F|)^s`.
* SWIRL Theorem 4.2.4, unique decoding branch (p.25): `mu` batching of `w` stacked columns,
  `N = (w - 1) |L|`, error `N / |F|`.
* SWIRL Theorem 4.2.1 (p.26): LogUp, with the hypothesis that the total interaction count of one
  proof is below `p`. Its bit count is a calculator figure (D3); the gate checks the hypothesis
  and that the LogUp proof of work is at least the frozen value.

Proof of work enters as the calculator's factor `(floor(p / 2^b) + 1) / p` (one base-`p` digit
ground to `b` zero bits). The radius is `delta_i = (1 - rho_i)/2 - 2^-sigma`, strictly inside the
unique decoding radius as H3 requires.
-/

namespace Clean.Sherlock.Bridge

/-- An error probability `num / den` (exact rational). -/
structure Err where
  num : Nat
  den : Nat
  deriving DecidableEq

def Err.mul (a b : Err) : Err := ⟨a.num * b.num, a.den * b.den⟩

def Err.add (a b : Err) : Err := ⟨a.num * b.den + b.num * a.den, a.den * b.den⟩

def Err.pow (a : Err) (t : Nat) : Err := ⟨a.num ^ t, a.den ^ t⟩

/-- The error is at most `2^-B`: `B` bits of soundness. -/
def Err.ge (e : Err) (B : Nat) : Bool := 2 ^ B * e.num ≤ e.den

/-- The bit count is exactly `B`: `2^-(B+1) < e <= 2^-B`. -/
def Err.reaches (e : Err) (B : Nat) : Prop := 2 ^ B * e.num ≤ e.den ∧ e.den < 2 ^ (B + 1) * e.num

instance (e : Err) (B : Nat) : Decidable (e.reaches B) := by unfold Err.reaches; infer_instance

/-- The size of the challenge field of a BabyBear layer: `|EF| = p^4` (`2^123.6276`). -/
def efSize : Nat := pBB ^ 4

/-- The calculator's proof-of-work factor for `b` bits. -/
def powFac (b : Nat) : Err := ⟨pBB / 2 ^ b + 1, pBB⟩

/-- The declared analysis parameters of one WHIR layer in unique decoding. -/
structure UDSet where
  /-- `log2` of the inverse rate of round 0 (`rho = 2^-logBlowup`). -/
  logBlowup : Nat
  /-- `log2` of the committed polynomial's size (`n_stack + l_skip`). -/
  logDeg : Nat
  /-- Folds per WHIR round (`k_whir`). -/
  kWhir : Nat
  /-- Queries per round; the round count `M` is the length. -/
  queries : List Nat
  /-- Bound on the stacked columns (`w_stack`). -/
  wStack : Nat
  /-- Bound on the total interaction count of one proof (SWIRL Theorem 4.2.1). -/
  interactions : Nat
  /-- The frozen LogUp proof of work. -/
  logupReq : Nat
  /-- Radius slack: `delta_i = (1 - rho_i)/2 - 2^-sigma`. -/
  sigma : Nat
  deriving DecidableEq

/-- The proof-of-work and OOD values of a layer, read from its transcript program. -/
structure PowVec where
  /-- One value per fold, round-major (`M * k` entries). -/
  fold : List Nat
  mu : Nat
  /-- One value per round. -/
  query : List Nat
  logup : Nat
  /-- OOD samples of rounds `1 .. M - 1`. -/
  ood : List Nat
  deriving DecidableEq

namespace UDSet

variable (F : Nat) (u : UDSet) (pw : PowVec)

def rounds : Nat := u.queries.length

/-- `log2 (1 / rho_i)`: the rate falls by `2^(k - 1)` per round. -/
def rateLog (i : Nat) : Nat := u.logBlowup + i * (u.kWhir - 1)

/-- Fold `s` (1-based) of round `i`: `3 / |F| + 2^(m_i - s) / (rho_i |F|)`, times the PoW
factor. `2^(m_i - s) / rho_i = 2^(m + b - s - i)`. -/
def foldErr (i s : Nat) : Err :=
  (Err.mk (3 + 2 ^ (u.logDeg + u.logBlowup - s - i)) F).mul (powFac (pw.fold.getD (i * u.kWhir + s - 1) 0))

/-- All fold terms. -/
def foldErrs : List Err :=
  (List.range u.rounds).flatMap fun i => (List.range u.kWhir).map fun j => foldErr F u pw i (j + 1)

/-- `mu` batching: `(w - 1) 2^(m + b) / |F|`, times the PoW factor. -/
def muErr : Err := (Err.mk ((u.wStack - 1) * 2 ^ (u.logDeg + u.logBlowup)) F).mul (powFac pw.mu)

/-- `1 - delta_i = (1 + rho_i)/2 + 2^-sigma`. -/
def agree (i : Nat) : Err :=
  ⟨(2 ^ u.rateLog i + 1) * 2 ^ u.sigma + 2 ^ (u.rateLog i + 1), 2 ^ (u.rateLog i + 1 + u.sigma)⟩

/-- Queries of round `i`: `(1 - delta_i)^t` times the PoW factor, plus `(t + 1)/|F|`. -/
def queryErr (i : Nat) : Err :=
  (((u.agree i).pow (u.queries.getD i 0)).mul (powFac (pw.query.getD i 0))).add
    ⟨u.queries.getD i 0 + 1, F⟩

def queryErrs : List Err := (List.range u.rounds).map (u.queryErr F pw)

/-- OOD of round `i >= 1` with `s` samples, list size 1: `(1/2) (2^m_i / |F|)^s`. -/
def oodErr (i : Nat) : Err :=
  ⟨2 ^ ((u.logDeg - i * u.kWhir) * pw.ood.getD (i - 1) 0), 2 * F ^ pw.ood.getD (i - 1) 0⟩

def oodErrs : List Err := (List.range (u.rounds - 1)).map fun i => u.oodErr F pw (i + 1)

/-- `gamma` of round `i >= 1`: `(t_(i-1) + s_i) / |F|`. -/
def gammaErr (i : Nat) : Err := ⟨u.queries.getD (i - 1) 0 + pw.ood.getD (i - 1) 0, F⟩

def gammaErrs : List Err := (List.range (u.rounds - 1)).map fun i => u.gammaErr F pw (i + 1)

/-- The radius is non-negative: `2^-sigma <= (1 - rho_i)/2`, i.e.
`2^(e + 1) <= 2^sigma (2^e - 1)` for `e = log2 (1/rho_i)`. -/
def radiusOK : Bool :=
  (List.range u.rounds).all fun i => 2 ^ (u.rateLog i + 1) ≤ 2 ^ u.sigma * (2 ^ u.rateLog i - 1)

/-- The shape conditions: a code rate below 1, at least one fold per round, enough variables for
every fold, the interaction bound below `p` (SWIRL Theorem 4.2.1), the vector lengths, at least
one OOD sample per round. -/
def shapeOK : Bool :=
  1 ≤ u.logBlowup && 1 ≤ u.kWhir && 1 ≤ u.rounds && u.kWhir * u.rounds ≤ u.logDeg &&
    u.interactions < pBB && u.logupReq ≤ pw.logup && u.radiusOK &&
    pw.fold.length == u.kWhir * u.rounds && pw.query.length == u.rounds &&
    pw.ood.length == u.rounds - 1 && pw.ood.all (1 ≤ ·)

/-- The G4 gate in unique decoding: the shape conditions, every grindable term (folding, `mu`,
queries) at least `101` bits and every other term (OOD, `gamma`) at least `100` bits. -/
def gate : Bool :=
  u.shapeOK pw && (u.foldErrs F pw).all (·.ge 101) && (u.muErr F pw).ge 101 &&
    (u.queryErrs F pw).all (·.ge 101) && (u.oodErrs F pw).all (·.ge 100) &&
    (u.gammaErrs F pw).all (·.ge 100)

end UDSet

/-! ## The frozen unique decoding sets (owner decision OD-19) -/

/-- `L` (OD-19): log blowup 3, `n_stack + l_skip = 20`, `k_whir = 4`, queries 98/83/82,
`w_stack <= 48`, the keygen interaction bound `p - 1` (total interactions of one proof below `p`,
`max_interaction_count = BabyBear::ORDER_U32`), LogUp PoW 18. -/
def lUD : UDSet := ⟨3, 20, 4, [98, 83, 82], 48, pBB - 1, 18, 40⟩

/-- The PoW and OOD values of the OD-19 `L` set: folding 0 on all 12 folds, `mu` 6, query 20,
LogUp 18, one OOD sample per round. -/
def lPow : PowVec := ⟨List.replicate 12 0, 6, [20, 20, 20], 18, [1, 1]⟩

/-- Internal layer (OD-19): log blowup 3, `n_stack + l_skip = 19`, queries 97/82/81,
`w_stack <= 512`, the same interaction bound. -/
def intUD : UDSet := ⟨3, 19, 4, [97, 82, 81], 512, pBB - 1, 18, 40⟩

/-- Internal PoW: folding 0, `mu` 9, query 21, LogUp 18, one OOD sample. -/
def intPow : PowVec := ⟨List.replicate 12 0, 9, [21, 21, 21], 18, [1, 1]⟩

theorem lUD_gate : lUD.gate efSize lPow = true := by decide

theorem intUD_gate : intUD.gate efSize intPow = true := by decide

/-- The bit count of every term of the OD-19 `L` set (calculator figures in brackets,
`UD-SIZING.md` section 2): fold `101` (101.6276), `mu` `101` (101.0730), query `101` per round
(101.3466 / 101.1427 / 101.7684), OOD `108`, `112` (108.6276 / 112.6276), `gamma` `116`, `117`
(116.9982). -/
theorem lUD_bits :
    (lUD.foldErr efSize lPow 0 1).reaches 101 ∧ (lUD.muErr efSize lPow).reaches 101 ∧
    (lUD.queryErr efSize lPow 0).reaches 101 ∧ (lUD.queryErr efSize lPow 1).reaches 101 ∧
    (lUD.queryErr efSize lPow 2).reaches 101 ∧ (lUD.oodErr efSize lPow 1).reaches 108 ∧
    (lUD.oodErr efSize lPow 2).reaches 112 ∧ (lUD.gammaErr efSize lPow 1).reaches 116 ∧
    (lUD.gammaErr efSize lPow 2).reaches 117 := by decide

/-- Fold `(0, 1)` is the weakest fold of `L`: every fold term is at least as strong. -/
theorem lUD_fold_min : (lUD.foldErrs efSize lPow).all (·.ge 101) = true ∧
    (lUD.foldErrs efSize lPow).length = 12 := by decide

/-- The bit count of every term of the OD-19 internal set: fold `102` (102.6276), `mu` `101`
(101.6304), query `101` per round (101.5158 / 101.1643 / 101.7705), OOD `109`, `113`
(109.6276), `gamma` `117` (117.0129). -/
theorem intUD_bits :
    (intUD.foldErr efSize intPow 0 1).reaches 102 ∧ (intUD.muErr efSize intPow).reaches 101 ∧
    (intUD.queryErr efSize intPow 0).reaches 101 ∧ (intUD.queryErr efSize intPow 1).reaches 101 ∧
    (intUD.queryErr efSize intPow 2).reaches 101 ∧ (intUD.oodErr efSize intPow 1).reaches 109 ∧
    (intUD.oodErr efSize intPow 2).reaches 113 ∧ (intUD.gammaErr efSize intPow 1).reaches 117 ∧
    (intUD.gammaErr efSize intPow 2).reaches 117 := by decide

/-- `mu` PoW 5 leaves the `mu` term at 100 bits, below the grindable gate. -/
theorem lUD_mu5_fails : (lUD.muErr efSize { lPow with mu := 5 }).reaches 100 ∧
    lUD.gate efSize { lPow with mu := 5 } = false := by decide

/-- One query fewer in round 0 (97) leaves that term at 100 bits. -/
theorem lUD_q97_fails : (({ lUD with queries := [97, 83, 82] } : UDSet).queryErr efSize lPow 0).reaches 100 ∧
    ({ lUD with queries := [97, 83, 82] } : UDSet).gate efSize lPow = false := by decide

/-- A key with no OOD sample fails the shape conditions. -/
theorem lUD_no_ood_fails : lUD.gate efSize { lPow with ood := [0, 1] } = false := by decide

/-- Two OOD samples per round also pass (OOD `216`, `224` bits). -/
theorem lUD_two_ood : lUD.gate efSize { lPow with ood := [2, 2] } = true ∧
    (lUD.oodErr efSize { lPow with ood := [2, 2] } 1).reaches 216 ∧
    (lUD.oodErr efSize { lPow with ood := [2, 2] } 2).reaches 224 := by decide

end Clean.Sherlock.Bridge
