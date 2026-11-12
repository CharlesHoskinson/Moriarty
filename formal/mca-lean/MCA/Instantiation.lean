/-
OD-19 (b): the bit bound at the frozen `L` and internal parameters. Phase 1 stub.

STATEMENT.md section 7 pins the frozen parameters (log degree, log inverse rate, challenge field
size, batch width `w`, target bits). Until then this file states only the generic shape.

Known convention gaps that this file must close (see SURVEY.md, "Findings"):
* the calculator plugs the rate `d / n` into BCHKS25's `ρ`, which is `(d - 1) / n`;
* the calculator sets `η = √ρ / (2m)` (agreement `(7/6) √ρ` at `m = 3`, the Thm. 1.5 convention),
  while Thm. 4.6 uses `m = ⌈√ρ / η⌉` (agreement `(4/3) √ρ` at `m = 3`);
* ArkLib's curve bound has the factor `2 ℓ` where the calculator has `w - 1`.
-/
import MCA.Statements

namespace MCA

variable {F : Type} [Field F] [Fintype F] [DecidableEq F]
variable {ι : Type} [Fintype ι] [DecidableEq ι]

/-- The parameters of one layer, as `calculator.rs` takes them. -/
structure LayerParams where
  logDegree : ℕ
  logInvRate : ℕ
  /-- `log2` of a lower bound on the challenge field size. -/
  fieldBits : ℕ
  /-- The number of batched words `w` (`w = 2` for a fold). -/
  width : ℕ
  /-- The bits that the calculator claims for this term. -/
  targetBits : ℕ

/-- Generic shape of the instantiation: at the layer's parameters, agreement `(4/3) √ρ`, the MCA
error of `Gen(w; ·)` is at most `2^-targetBits`. The numeric side condition `hnum` is a closed
inequality between numerals that the concrete instances discharge by `norm_num`; it is not the
MCA bound. -/
theorem layer_mca_bits (p : LayerParams) (dom : ι ↪ F)
    (hn : Fintype.card ι = 2 ^ (p.logDegree + p.logInvRate))
    (hq : (2 : ℝ) ^ p.fieldBits ≤ Fintype.card F) (hw : 1 ≤ p.width) (hld : 1 ≤ p.logDegree)
    (hnum : ((p.width : ℝ) - 1) *
        bchksA (2 ^ (p.logDegree + p.logInvRate))
          (reducedRate (2 ^ (p.logDegree + p.logInvRate)) (2 ^ p.logDegree))
          (1 - 4 / 3 * Real.sqrt
            (reducedRate (2 ^ (p.logDegree + p.logInvRate)) (2 ^ p.logDegree))) 3
        ≤ (2 : ℝ) ^ ((p.fieldBits : ℤ) - p.targetBits)) :
    HasMCA (rsCode dom (2 ^ p.logDegree) : Set (ι → F)) (powGen F p.width)
      (1 - 4 / 3 * Real.sqrt
        (reducedRate (2 ^ (p.logDegree + p.logInvRate)) (2 ^ p.logDegree)))
      ((2 : ℝ) ^ (-(p.targetBits : ℤ))) := by
  sorry

-- TODO(STATEMENT.md §7): `L_layer_mca_bits` and `internal_layer_mca_bits` at the frozen
-- parameters, each by `layer_mca_bits` with `hnum` closed by `norm_num`.

end MCA
