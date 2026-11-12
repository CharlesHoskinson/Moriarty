/-
Axiom audit. A classic file (no `module` header), so that the proof bodies of the imported
ArkLib modules are loaded. Run with `lake env lean MCA/Axioms.lean`.
-/
import MCA

-- ArkLib results that phase 2 builds on (survey check).
#print axioms CodingTheory.ReedSolomon.rs_mcaError_le_in_johnson_range
#print axioms CodingTheory.ReedSolomon.rs_mcaError_le_johnsonExceptionCount
#print axioms ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype
#print axioms ProximityGap.mcaError_powers_le_of_exactPowerAgreement

-- Main theorems of this project (phase 1: they show `sorryAx`).
#print axioms MCA.mca_curve_johnson
#print axioms MCA.mca_fold_johnson
#print axioms MCA.hasMCA_curve_four_thirds
#print axioms MCA.layer_mca_bits
