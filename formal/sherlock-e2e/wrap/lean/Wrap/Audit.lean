import SherlockCodec.AuditCmd
import Wrap.Consistency
import Wrap.Spec
import Wrap.Tests

/-! # Audit of Wrap

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns Wrap` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns Wrap

/-- info: 'Wrap.consistent' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.consistent

/-- info: 'Wrap.consistent_ad20_b' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.consistent_ad20_b

/-- info: 'Wrap.dg002_forces_k21' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.dg002_forces_k21

/-- info: 'Wrap.wrp003_is_dg002' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.wrp003_is_dg002

/-- info: 'Wrap.lw016_t4_rows' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.lw016_t4_rows

/-- info: 'Wrap.k22_rejected' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.k22_rejected

/-- info: 'Wrap.vk_over_rejected' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.vk_over_rejected

/-- info: 'Wrap.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_init

/-- info: 'Wrap.addBit_range' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.addBit_range

/-- info: 'Wrap.addBit_full' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.addBit_full

/-- info: 'Wrap.inv_askK' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_askK

/-- info: 'Wrap.inv_cost' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_cost

/-- info: 'Wrap.inv_keygen' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_keygen

/-- info: 'Wrap.inv_expect' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_expect

/-- info: 'Wrap.inv_decode' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_decode

/-- info: 'Wrap.inv_prove' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_prove

/-- info: 'Wrap.inv_deploy' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_deploy

/-- info: 'Wrap.inv_gpu' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_gpu

/-- info: 'Wrap.inv_newCall' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_newCall

/-- info: 'Wrap.inv_qualify' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_qualify

/-- info: 'Wrap.apply_disabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Wrap.apply_disabled

/-- info: 'Wrap.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_apply

/-- info: 'Wrap.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_stepOp

/-- info: 'Wrap.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_runOps

/-- info: 'Wrap.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.inv_run

/-- info: 'Wrap.WRP_002_never_k22' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_002_never_k22

/-- info: 'Wrap.WRP_003_row_gate' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_003_row_gate

/-- info: 'Wrap.WRP_004_vk_bound' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_004_vk_bound

/-- info: 'Wrap.WRP_001_one_proof' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_001_one_proof

/-- info: 'Wrap.runOps_testSeed' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Wrap.runOps_testSeed

/-- info: 'Wrap.WRP_006_fresh_blinders' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_006_fresh_blinders

/-- info: 'Wrap.WRP_007_anchor_slot' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_007_anchor_slot

/-- info: 'Wrap.WRP_009_three_files' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_009_three_files

/-- info: 'Wrap.WRP_010_one_key' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_010_one_key

/-- info: 'Wrap.WRP_011_gpu_key' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_011_gpu_key

/-- info: 'Wrap.WRP_012_deploy_qualified' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.WRP_012_deploy_qualified

/-- info: 'Wrap.ANC_016_no_template_in_ordinary_call' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Wrap.ANC_016_no_template_in_ordinary_call

/-- info: 'Wrap.rowGate_derived' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.rowGate_derived

/-- info: 'Wrap.kMax_is_21' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.kMax_is_21

/-- info: 'Wrap.measured_t10m_rows' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.measured_t10m_rows

/-- info: 'Wrap.measured_t4_rows' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.measured_t4_rows

/-- info: 'Wrap.measured_t3_rows' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.measured_t3_rows

/-- info: 'Wrap.measured_t10m_vk' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.measured_t10m_vk

/-- info: 'Wrap.publicInputs_1026' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.publicInputs_1026

/-- info: 'Wrap.measured_d3_weakest' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.measured_d3_weakest

/-- info: 'Wrap.pin_k22' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_k22

/-- info: 'Wrap.pin_k21' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_k21

/-- info: 'Wrap.pin_rows_over' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_rows_over

/-- info: 'Wrap.pin_rows_gate' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_rows_gate

/-- info: 'Wrap.pin_vk_over' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_vk_over

/-- info: 'Wrap.pin_vk_bound' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_vk_bound

/-- info: 'Wrap.pin_keydiff' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_keydiff

/-- info: 'Wrap.pin_slot0' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_slot0

/-- info: 'Wrap.pin_slot3' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_slot3

/-- info: 'Wrap.pin_template' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_template

/-- info: 'Wrap.pin_ordinary' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_ordinary

/-- info: 'Wrap.pin_written' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_written

/-- info: 'Wrap.pin_verify_fail' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_verify_fail

/-- info: 'Wrap.pin_deploy_unqualified' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_deploy_unqualified

/-- info: 'Wrap.pin_deploy_qualified' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_deploy_qualified

/-- info: 'Wrap.pin_deploy_four_bits' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_deploy_four_bits

/-- info: 'Wrap.pin_second_key' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_second_key

/-- info: 'Wrap.pin_gpu_other' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_gpu_other

/-- info: 'Wrap.pin_gpu_same' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_gpu_same

/-- info: 'Wrap.pin_blinders' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_blinders

/-- info: 'Wrap.pin_seeded' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_seeded

/-- info: 'Wrap.pin_refuse_code' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_refuse_code

/-- info: 'Wrap.pin_refuse_disabled' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_refuse_disabled

/-- info: 'Wrap.pin_refuse_vkid0' does not depend on any axioms -/
#guard_msgs in
#print axioms Wrap.pin_refuse_vkid0
