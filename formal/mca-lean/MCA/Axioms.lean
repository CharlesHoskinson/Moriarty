/-
Axiom audit. A classic file (no `module` header), so that the proof bodies of the imported
ArkLib modules are loaded. Run with `lake env lean MCA/Axioms.lean`. Every line must print only
`propext`, `Classical.choice` and `Quot.sound`.
-/
import MCA

-- ArkLib results that route (a) builds on.
#print axioms ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype
#print axioms Finset.card_mul_sq_sub_card_mul_le_of_inter_card_le

-- Bridges.
#print axioms MCA.rsCode_eq_arklib
#print axioms MCA.mcaBad_iff_isMCA
#print axioms MCA.hasMCA_iff_mcaError_le

-- Route (a).
#print axioms MCA.curve_card_le
#print axioms MCA.vandermonde_mem
#print axioms MCA.cor_8_2

-- PROOF.md integer lemmas and Corollary 9.2.
#print axioms MCA.lemma_9_0
#print axioms MCA.lemma_9_0b
#print axioms MCA.cor_9_2

-- Bit bounds at the frozen rows.
#print axioms MCA.row_bits
#print axioms MCA.row_mcaError_bits
#print axioms MCA.row_subcode_bits
