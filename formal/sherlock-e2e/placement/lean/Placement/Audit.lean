import SherlockCodec.AuditCmd
import Placement.Consistency
import Placement.Spec
import Placement.Tests

/-! # Audit of Placement

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns Placement` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns Placement

/-- info: 'Placement.consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms Placement.consistent

/-- info: 'Placement.HLP_006_refuses_edit' does not depend on any axioms -/
#guard_msgs in
#print axioms Placement.HLP_006_refuses_edit

/-- info: 'Placement.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.inv_init

/-- info: 'Placement.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.inv_apply

/-- info: 'Placement.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.inv_stepOp

/-- info: 'Placement.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.inv_runOps

/-- info: 'Placement.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.inv_run

/-- info: 'Placement.HLP_001_kept_after_ledger_check' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.HLP_001_kept_after_ledger_check

/-- info: 'Placement.HLP_002_no_submit_of_refused' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.HLP_002_no_submit_of_refused

/-- info: 'Placement.HLP_003_four_public_words' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.HLP_003_four_public_words

/-- info: 'Placement.HLP_004_no_private_part' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.HLP_004_no_private_part

/-- info: 'Placement.HLP_005_case_A_only' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.HLP_005_case_A_only

/-- info: 'Placement.HLP_007_no_sign_without_wallet' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.HLP_007_no_sign_without_wallet

/-- info: 'Placement.HLP_008_binding_before_verify' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.HLP_008_binding_before_verify

/-- info: 'Placement.PRD_002_one_slot_per_wrap' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Placement.PRD_002_one_slot_per_wrap

/-- info: 'Placement.pin_not_owned' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_not_owned

/-- info: 'Placement.pin_honest' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_honest

/-- info: 'Placement.pin_cv_differs' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_cv_differs

/-- info: 'Placement.pin_sha_differs' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_sha_differs

/-- info: 'Placement.pin_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_refused

/-- info: 'Placement.pin_no_wallet' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_no_wallet

/-- info: 'Placement.pin_wallet' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_wallet

/-- info: 'Placement.pin_helper_slot' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_helper_slot

/-- info: 'Placement.pin_no_slot' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_no_slot

/-- info: 'Placement.pin_one_slot' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_one_slot

/-- info: 'Placement.pin_export' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_export

/-- info: 'Placement.pin_send_helper' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_send_helper

/-- info: 'Placement.pin_send_owned' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_send_owned

/-- info: 'Placement.pin_refuse_code' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_refuse_code

/-- info: 'Placement.pin_refuse_disabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_refuse_disabled

/-- info: 'Placement.pin_refuse_kind' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Placement.pin_refuse_kind
