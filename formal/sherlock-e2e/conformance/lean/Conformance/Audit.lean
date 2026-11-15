import SherlockCodec.AuditCmd
import Conformance.Consistency
import Conformance.Spec
import Conformance.Tests

/-! # Audit of Conformance

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns Conformance` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns Conformance

/-- info: 'Conformance.Consistency.consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.Consistency.consistent

/-- info: 'Conformance.Consistency.ecs011_ecs004' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.Consistency.ecs011_ecs004

/-- info: 'Conformance.ind_range' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.ind_range

/-- info: 'Conformance.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_init

/-- info: 'Conformance.inv_start' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_start

/-- info: 'Conformance.inv_nativeRef' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_nativeRef

/-- info: 'Conformance.inv_compile' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_compile

/-- info: 'Conformance.inv_prove' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_prove

/-- info: 'Conformance.inv_verify' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_verify

/-- info: 'Conformance.inv_settle' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_settle

/-- info: 'Conformance.inv_compare' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_compare

/-- info: 'Conformance.inv_report' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_report

/-- info: 'Conformance.apply_disabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Conformance.apply_disabled

/-- info: 'Conformance.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_apply

/-- info: 'Conformance.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_stepOp

/-- info: 'Conformance.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_runOps

/-- info: 'Conformance.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.inv_run

/-- info: 'Conformance.ECS_004_table_sums' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_004_table_sums

/-- info: 'Conformance.ECS_005_blocked_reason' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_005_blocked_reason

/-- info: 'Conformance.ECS_003_stage_order' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_003_stage_order

/-- info: 'Conformance.ECS_007_no_dispatch' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_007_no_dispatch

/-- info: 'Conformance.ECS_008_no_preview' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_008_no_preview

/-- info: 'Conformance.ECS_009_derived_label' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_009_derived_label

/-- info: 'Conformance.ECS_012_denominator' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_012_denominator

/-- info: 'Conformance.ECS_013_one_identity' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_013_one_identity

/-- info: 'Conformance.ECS_014_complete_needs_evidence' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_014_complete_needs_evidence

/-- info: 'Conformance.ECS_010_mismatch_not_done' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Conformance.ECS_010_mismatch_not_done

/-- info: 'Conformance.measured_manifest' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_manifest

/-- info: 'Conformance.measured_native_refs' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_native_refs

/-- info: 'Conformance.measured_dryrun_classes' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_dryrun_classes

/-- info: 'Conformance.measured_blocked_reasons' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_blocked_reasons

/-- info: 'Conformance.measured_kit_blocked' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_kit_blocked

/-- info: 'Conformance.measured_counts' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_counts

/-- info: 'Conformance.measured_dryrun_denominator' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_dryrun_denominator

/-- info: 'Conformance.measured_dryrun_preview' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.measured_dryrun_preview

/-- info: 'Conformance.pin_pass' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_pass

/-- info: 'Conformance.pin_complete' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_complete

/-- info: 'Conformance.pin_na' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_na

/-- info: 'Conformance.pin_rejected_ref' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_rejected_ref

/-- info: 'Conformance.pin_derived' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_derived

/-- info: 'Conformance.pin_compile_mismatch' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_compile_mismatch

/-- info: 'Conformance.pin_kit_absent' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_kit_absent

/-- info: 'Conformance.pin_kit_present' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_kit_present

/-- info: 'Conformance.pin_chipset' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_chipset

/-- info: 'Conformance.pin_no_native' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_no_native

/-- info: 'Conformance.pin_dispatch_off' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_dispatch_off

/-- info: 'Conformance.pin_settle_dry' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_settle_dry

/-- info: 'Conformance.pin_incomplete_report' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_incomplete_report

/-- info: 'Conformance.pin_denominator_dry' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_denominator_dry

/-- info: 'Conformance.pin_denominator' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_denominator

/-- info: 'Conformance.pin_mismatch_not_done' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_mismatch_not_done

/-- info: 'Conformance.pin_duplicate' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_duplicate

/-- info: 'Conformance.pin_refuse_order' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_refuse_order

/-- info: 'Conformance.pin_refuse_kit' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_refuse_kit

/-- info: 'Conformance.pin_preview_off' does not depend on any axioms -/
#guard_msgs in
#print axioms Conformance.pin_preview_off
