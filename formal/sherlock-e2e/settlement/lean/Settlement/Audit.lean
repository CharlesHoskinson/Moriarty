import SherlockCodec.AuditCmd
import Settlement.Consistency
import Settlement.Spec
import Settlement.Tests

/-! # Audit of Settlement

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns Settlement` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns Settlement

/-- info: 'Settlement.Consistency.consistent' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.Consistency.consistent

/-- info: 'Settlement.Consistency.SLC_007_changed_effect' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.Consistency.SLC_007_changed_effect

/-- info: 'Settlement.Consistency.SLC_001_matches_oracle' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.Consistency.SLC_001_matches_oracle

/-- info: 'Settlement.Consistency.LDG_008_once' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.Consistency.LDG_008_once

/-- info: 'Settlement.Consistency.finding_bat011_boundary' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.Consistency.finding_bat011_boundary

/-- info: 'Settlement.Consistency.finding_bat010_boundary' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.Consistency.finding_bat010_boundary

/-- info: 'Settlement.Consistency.finding_mid002_read_entries' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.Consistency.finding_mid002_read_entries

/-- info: 'Settlement.Consistency.finding_anc012_formula' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.Consistency.finding_anc012_formula

/-- info: 'Settlement.Consistency.ERT_004_consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.Consistency.ERT_004_consistent

/-- info: 'Settlement.Consistency.GOV_006_consistent' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.Consistency.GOV_006_consistent

/-- info: 'Settlement.b2i_range' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.b2i_range

/-- info: 'Settlement.b2i_true' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.b2i_true

/-- info: 'Settlement.b2i_false' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.b2i_false

/-- info: 'Settlement.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_init

/-- info: 'Settlement.chunks_le' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.chunks_le

/-- info: 'Settlement.runOf_nonneg' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.runOf_nonneg

/-- info: 'Settlement.collBad_root' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.collBad_root

/-- info: 'Settlement.inv_deployNew' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_deployNew

/-- info: 'Settlement.inv_deploy' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_deploy

/-- info: 'Settlement.inv_buildArgs' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.inv_buildArgs

/-- info: 'Settlement.inv_settle' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_settle

/-- info: 'Settlement.inv_acceptRun' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_acceptRun

/-- info: 'Settlement.inv_tick' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_tick

/-- info: 'Settlement.inv_job' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_job

/-- info: 'Settlement.inv_close' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_close

/-- info: 'Settlement.inv_announce' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.inv_announce

/-- info: 'Settlement.inv_startRun' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.inv_startRun

/-- info: 'Settlement.inv_intent' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.inv_intent

/-- info: 'Settlement.inv_plan' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_plan

/-- info: 'Settlement.inv_addCollForm' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.inv_addCollForm

/-- info: 'Settlement.inv_replay' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_replay

/-- info: 'Settlement.inv_freeze' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.inv_freeze

/-- info: 'Settlement.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_apply

/-- info: 'Settlement.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_stepOp

/-- info: 'Settlement.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_runOps

/-- info: 'Settlement.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.inv_run

/-- info: 'Settlement.SLC_001_004_deploy_shape' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.SLC_001_004_deploy_shape

/-- info: 'Settlement.SLC_002_anchor_slot' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.SLC_002_anchor_slot

/-- info: 'Settlement.SLC_002_anchor_constructor_only' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.SLC_002_anchor_constructor_only

/-- info: 'Settlement.BAT_002_operator_no_acceptRun' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.BAT_002_operator_no_acceptRun

/-- info: 'Settlement.SLC_010_native_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.SLC_010_native_refused

/-- info: 'Settlement.SLC_012_unbound_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.SLC_012_unbound_refused

/-- info: 'Settlement.SLC_005_other_program' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.SLC_005_other_program

/-- info: 'Settlement.LDG_002_at_most_once' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.LDG_002_at_most_once

/-- info: 'Settlement.STL_004_runs_set' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.STL_004_runs_set

/-- info: 'Settlement.BAT_011_era_refusal' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.BAT_011_era_refusal

/-- info: 'Settlement.BAT_006_default_ttl' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.BAT_006_default_ttl

/-- info: 'Settlement.BAT_010_close_before_era' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.BAT_010_close_before_era

/-- info: 'Settlement.LDG_006_one_write_per_epoch' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.LDG_006_one_write_per_epoch

/-- info: 'Settlement.BAT_005_cap' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Settlement.BAT_005_cap

/-- info: 'Settlement.anc012_call_words' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.anc012_call_words

/-- info: 'Settlement.measured_largest_calls' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.measured_largest_calls

/-- info: 'Settlement.kmax_default_ttl' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.kmax_default_ttl

/-- info: 'Settlement.kmax_long_ttl' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.kmax_long_ttl

/-- info: 'Settlement.kmax_short_ttl' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.kmax_short_ttl

/-- info: 'Settlement.bat005_pins' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.bat005_pins

/-- info: 'Settlement.epoch_cap_depth20' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.epoch_cap_depth20

/-- info: 'Settlement.bat003_rates' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.bat003_rates

/-- info: 'Settlement.bat004_counts' does not depend on any axioms -/
#guard_msgs in
#print axioms Settlement.bat004_counts

/-- info: 'Settlement.pin_deploy' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_deploy

/-- info: 'Settlement.pin_settle' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_settle

/-- info: 'Settlement.pin_replay' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_replay

/-- info: 'Settlement.pin_other_program' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_other_program

/-- info: 'Settlement.pin_both_node' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_both_node

/-- info: 'Settlement.pin_mismatch' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_mismatch

/-- info: 'Settlement.pin_no_args' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_no_args

/-- info: 'Settlement.pin_operator' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_operator

/-- info: 'Settlement.pin_native' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_native

/-- info: 'Settlement.pin_unbound' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_unbound

/-- info: 'Settlement.pin_stl005' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_stl005

/-- info: 'Settlement.pin_epoch' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_epoch

/-- info: 'Settlement.pin_no_overrun' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_no_overrun

/-- info: 'Settlement.pin_no_late_job' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_no_late_job

/-- info: 'Settlement.pin_no_early_close' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_no_early_close

/-- info: 'Settlement.pin_empty_epoch' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_empty_epoch

/-- info: 'Settlement.pin_no_form' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_no_form

/-- info: 'Settlement.pin_dup_root' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_dup_root

/-- info: 'Settlement.pin_registry' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_registry

/-- info: 'Settlement.pin_era_close' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_era_close

/-- info: 'Settlement.pin_era_run' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_era_run

/-- info: 'Settlement.pin_ttl' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_ttl

/-- info: 'Settlement.pin_plan' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_plan

/-- info: 'Settlement.pin_short_ttl' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_short_ttl

/-- info: 'Settlement.pin_reuse' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_reuse

/-- info: 'Settlement.pin_freeze_open' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_freeze_open

/-- info: 'Settlement.pin_refuse_code' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_refuse_code

/-- info: 'Settlement.pin_refuse_claim4' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Settlement.pin_refuse_claim4
