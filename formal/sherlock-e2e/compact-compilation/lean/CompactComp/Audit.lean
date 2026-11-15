import SherlockCodec.AuditCmd
import CompactComp.Consistency
import CompactComp.Spec
import CompactComp.Tests

/-! # Audit of CompactComp

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns CompactComp` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns CompactComp

/-- info: 'CompactComp.Consistency.consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.Consistency.consistent

/-- info: 'CompactComp.Consistency.cf001_ecc001_conflict' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.Consistency.cf001_ecc001_conflict

/-- info: 'CompactComp.circuitRefusal_range' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.circuitRefusal_range

/-- info: 'CompactComp.ind_range' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.ind_range

/-- info: 'CompactComp.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_init

/-- info: 'CompactComp.setBit_range' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.setBit_range

/-- info: 'CompactComp.inv_admit' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_admit

/-- info: 'CompactComp.inv_finish' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_finish

/-- info: 'CompactComp.inv_serialize' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_serialize

/-- info: 'CompactComp.inv_gate' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_gate

/-- info: 'CompactComp.inv_use' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_use

/-- info: 'CompactComp.inv_newRequest' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_newRequest

/-- info: 'CompactComp.apply_disabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.apply_disabled

/-- info: 'CompactComp.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_apply

/-- info: 'CompactComp.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_stepOp

/-- info: 'CompactComp.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_runOps

/-- info: 'CompactComp.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.inv_run

/-- info: 'CompactComp.ECC_010_image_or_refusal' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_010_image_or_refusal

/-- info: 'CompactComp.ECC_012_exit_zero_writes_all' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_012_exit_zero_writes_all

/-- info: 'CompactComp.ECC_013_refusal_no_image' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_013_refusal_no_image

/-- info: 'CompactComp.ECC_003_one_abi_per_image' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_003_one_abi_per_image

/-- info: 'CompactComp.ECC_004_bytes_witness_free' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_004_bytes_witness_free

/-- info: 'CompactComp.ECC_007_no_tier3' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_007_no_tier3

/-- info: 'CompactComp.ECC_009_deploy_slot_15' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_009_deploy_slot_15

/-- info: 'CompactComp.ECC_014_jubjub_tape_words' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_014_jubjub_tape_words

/-- info: 'CompactComp.ECC_002_settle_needs_gates' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CompactComp.ECC_002_settle_needs_gates

/-- info: 'CompactComp.CF_014_no_kit_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.CF_014_no_kit_refused

/-- info: 'CompactComp.MYC_FE_002_ir_version_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.MYC_FE_002_ir_version_refused

/-- info: 'CompactComp.ECC_001_carrier_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.ECC_001_carrier_refused

/-- info: 'CompactComp.ECC_005_shape_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.ECC_005_shape_refused

/-- info: 'CompactComp.MYC_SEAL_021_slot_clash_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.MYC_SEAL_021_slot_clash_refused

/-- info: 'CompactComp.ECC_014_jubjub_admitted' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.ECC_014_jubjub_admitted

/-- info: 'CompactComp.measured_circuits' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.measured_circuits

/-- info: 'CompactComp.measured_suite_calls' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.measured_suite_calls

/-- info: 'CompactComp.measured_gates' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.measured_gates

/-- info: 'CompactComp.measured_oracle_equal' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.measured_oracle_equal

/-- info: 'CompactComp.shapes_13' does not depend on any axioms -/
#guard_msgs in
#print axioms CompactComp.shapes_13

/-- info: 'CompactComp.pin_compile_ok' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_compile_ok

/-- info: 'CompactComp.pin_ir32' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_ir32

/-- info: 'CompactComp.pin_carrier' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_carrier

/-- info: 'CompactComp.pin_nokit' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_nokit

/-- info: 'CompactComp.pin_shape' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_shape

/-- info: 'CompactComp.pin_clash' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_clash

/-- info: 'CompactComp.pin_point' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_point

/-- info: 'CompactComp.pin_scalar' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_scalar

/-- info: 'CompactComp.pin_secp' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_secp

/-- info: 'CompactComp.pin_keccak' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_keccak

/-- info: 'CompactComp.pin_slot' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_slot

/-- info: 'CompactComp.pin_bytes' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_bytes

/-- info: 'CompactComp.pin_nogates' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_nogates

/-- info: 'CompactComp.pin_gates' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_gates

/-- info: 'CompactComp.pin_mutant' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_mutant

/-- info: 'CompactComp.pin_refuse_empty_finish' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_refuse_empty_finish

/-- info: 'CompactComp.pin_refuse_code' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_refuse_code

/-- info: 'CompactComp.pin_refuse_digit' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CompactComp.pin_refuse_digit
