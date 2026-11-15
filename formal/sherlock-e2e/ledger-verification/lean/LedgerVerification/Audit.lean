import SherlockCodec.AuditCmd
import LedgerVerification.Consistency
import LedgerVerification.Spec
import LedgerVerification.Tests

/-! # Audit of LedgerVerification

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns LedgerVerification` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns LedgerVerification

/-- info: 'LedgerVerification.Consistency.consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.Consistency.consistent

/-- info: 'LedgerVerification.Consistency.MID_004_needs_preview' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.Consistency.MID_004_needs_preview

/-- info: 'LedgerVerification.Consistency.MID_006_needs_pin' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.Consistency.MID_006_needs_pin

/-- info: 'LedgerVerification.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_init

/-- info: 'LedgerVerification.inv_manifest' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_manifest

/-- info: 'LedgerVerification.inv_gate' depends on axioms: [propext] -/
#guard_msgs in
#print axioms LedgerVerification.inv_gate

/-- info: 'LedgerVerification.inv_loadKey' depends on axioms: [propext] -/
#guard_msgs in
#print axioms LedgerVerification.inv_loadKey

/-- info: 'LedgerVerification.inv_check' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_check

/-- info: 'LedgerVerification.inv_buildCall' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_buildCall

/-- info: 'LedgerVerification.inv_native' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_native

/-- info: 'LedgerVerification.inv_submit' depends on axioms: [propext] -/
#guard_msgs in
#print axioms LedgerVerification.inv_submit

/-- info: 'LedgerVerification.inv_connect' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_connect

/-- info: 'LedgerVerification.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_apply

/-- info: 'LedgerVerification.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_stepOp

/-- info: 'LedgerVerification.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_runOps

/-- info: 'LedgerVerification.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.inv_run

/-- info: 'LedgerVerification.LVR_002_gate' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_002_gate

/-- info: 'LedgerVerification.LVR_003_no_g4_key' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_003_no_g4_key

/-- info: 'LedgerVerification.LVR_004_ten_controls' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_004_ten_controls

/-- info: 'LedgerVerification.LVR_005_exit_status' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_005_exit_status

/-- info: 'LedgerVerification.LVR_006_native_before_submit' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_006_native_before_submit

/-- info: 'LedgerVerification.LVR_007_refused_not_submitted' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_007_refused_not_submitted

/-- info: 'LedgerVerification.LVR_007_step' depends on axioms: [propext] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_007_step

/-- info: 'LedgerVerification.LVR_008_genesis_stable' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms LedgerVerification.LVR_008_genesis_stable

/-- info: 'LedgerVerification.measured_vk_bytes' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.measured_vk_bytes

/-- info: 'LedgerVerification.mid003_public_inputs' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.mid003_public_inputs

/-- info: 'LedgerVerification.measured_tx_bytes' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.measured_tx_bytes

/-- info: 'LedgerVerification.lvr004_lanes' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.lvr004_lanes

/-- info: 'LedgerVerification.pin_gate_pass' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_gate_pass

/-- info: 'LedgerVerification.pin_gate_override' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_gate_override

/-- info: 'LedgerVerification.pin_gate_stale' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_gate_stale

/-- info: 'LedgerVerification.pin_g4' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_g4

/-- info: 'LedgerVerification.pin_no_key_no_check' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_no_key_no_check

/-- info: 'LedgerVerification.pin_honest' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_honest

/-- info: 'LedgerVerification.pin_lane_passed' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_lane_passed

/-- info: 'LedgerVerification.pin_flip_passed' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_flip_passed

/-- info: 'LedgerVerification.pin_honest_refused' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_honest_refused

/-- info: 'LedgerVerification.pin_unpinned' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_unpinned

/-- info: 'LedgerVerification.pin_submit_before_native' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_submit_before_native

/-- info: 'LedgerVerification.pin_submit' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_submit

/-- info: 'LedgerVerification.pin_native_refused' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_native_refused

/-- info: 'LedgerVerification.pin_reprove' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_reprove

/-- info: 'LedgerVerification.pin_first_connect' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_first_connect

/-- info: 'LedgerVerification.pin_other_chain' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_other_chain

/-- info: 'LedgerVerification.pin_same_chain' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_same_chain

/-- info: 'LedgerVerification.pin_refuse_code' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_refuse_code

/-- info: 'LedgerVerification.pin_refuse_genesis0' does not depend on any axioms -/
#guard_msgs in
#print axioms LedgerVerification.pin_refuse_genesis0
