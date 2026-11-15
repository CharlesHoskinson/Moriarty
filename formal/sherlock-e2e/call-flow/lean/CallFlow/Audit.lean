import SherlockCodec.AuditCmd
import CallFlow.Consistency
import CallFlow.Spec
import CallFlow.Tests

/-! # Audit of CallFlow

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns CallFlow` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns CallFlow

/-- info: 'CallFlow.Consistency.consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.Consistency.consistent

/-- info: 'CallFlow.Consistency.euf011_today_open' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.Consistency.euf011_today_open

/-- info: 'CallFlow.lowestPin_range' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.lowestPin_range

/-- info: 'CallFlow.apply_en' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CallFlow.apply_en

/-- info: 'CallFlow.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_init

/-- info: 'CallFlow.inv_stepAt' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_stepAt

/-- info: 'CallFlow.inv_step' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_step

/-- info: 'CallFlow.inv_setupBox' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CallFlow.inv_setupBox

/-- info: 'CallFlow.inv_rekey' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CallFlow.inv_rekey

/-- info: 'CallFlow.inv_newCall' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_newCall

/-- info: 'CallFlow.inv_checkPins' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_checkPins

/-- info: 'CallFlow.inv_prove' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_prove

/-- info: 'CallFlow.inv_deploySecond' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_deploySecond

/-- info: 'CallFlow.inv_lost' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_lost

/-- info: 'CallFlow.inv_reconcile' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_reconcile

/-- info: 'CallFlow.inv_report' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CallFlow.inv_report

/-- info: 'CallFlow.apply_disabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CallFlow.apply_disabled

/-- info: 'CallFlow.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_apply

/-- info: 'CallFlow.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_stepOp

/-- info: 'CallFlow.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_runOps

/-- info: 'CallFlow.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.inv_run

/-- info: 'CallFlow.EUF_005_no_tx_after_stop' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_005_no_tx_after_stop

/-- info: 'CallFlow.EUF_006_native_refusal_no_prove' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_006_native_refusal_no_prove

/-- info: 'CallFlow.EUF_007_prove_needs_profile' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_007_prove_needs_profile

/-- info: 'CallFlow.EUF_008_one_claim' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_008_one_claim

/-- info: 'CallFlow.EUF_009_one_wrap_key' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_009_one_wrap_key

/-- info: 'CallFlow.EUF_002_one_deploy_per_program' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_002_one_deploy_per_program

/-- info: 'CallFlow.EUF_014_one_tx_per_claim' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_014_one_tx_per_claim

/-- info: 'CallFlow.EUF_013_complete_joined' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_013_complete_joined

/-- info: 'CallFlow.EUF_012_pin_stops' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_012_pin_stops

/-- info: 'CallFlow.EUF_010_outcome_after_readback' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms CallFlow.EUF_010_outcome_after_readback

/-- info: 'CallFlow.EUF_001_order' depends on axioms: [propext] -/
#guard_msgs in
#print axioms CallFlow.EUF_001_order

/-- info: 'CallFlow.pin_full' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_full

/-- info: 'CallFlow.pin_order' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_order

/-- info: 'CallFlow.pin_second_call' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_second_call

/-- info: 'CallFlow.pin_wrap_fails' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_wrap_fails

/-- info: 'CallFlow.pin_check_fails' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_check_fails

/-- info: 'CallFlow.pin_native_refusal' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_native_refusal

/-- info: 'CallFlow.pin_stale_profile' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_stale_profile

/-- info: 'CallFlow.pin_setup_again' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_setup_again

/-- info: 'CallFlow.pin_two_boxes' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_two_boxes

/-- info: 'CallFlow.pin_second_program' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_second_program

/-- info: 'CallFlow.pin_readback_differs' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_readback_differs

/-- info: 'CallFlow.pin_zkir_pin' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_zkir_pin

/-- info: 'CallFlow.pin_complete' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_complete

/-- info: 'CallFlow.pin_incomplete' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_incomplete

/-- info: 'CallFlow.pin_mixed' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_mixed

/-- info: 'CallFlow.pin_reconcile' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_reconcile

/-- info: 'CallFlow.pin_no_resend' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_no_resend

/-- info: 'CallFlow.pin_no_step_while_unknown' does not depend on any axioms -/
#guard_msgs in
#print axioms CallFlow.pin_no_step_while_unknown
