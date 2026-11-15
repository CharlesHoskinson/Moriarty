import SherlockCodec.AuditCmd
import SherlockExec.Consistency
import SherlockExec.Spec
import SherlockExec.Tests

/-! # Audit of SherlockExec

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns SherlockExec` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns SherlockExec

/-- info: 'SherlockExec.consistent' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.consistent

/-- info: 'SherlockExec.CORE_003_no_step_after_outcome' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.CORE_003_no_step_after_outcome

/-- info: 'SherlockExec.ERS_001_fields' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.ERS_001_fields

/-- info: 'SherlockExec.ERS_010_fields' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.ERS_010_fields

/-- info: 'SherlockExec.EKT_001_sixteen_rules' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.EKT_001_sixteen_rules

/-- info: 'SherlockExec.EKT_002_air_bucket' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.EKT_002_air_bucket

/-- info: 'SherlockExec.EKT_004_mixed_outside_bucket' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.EKT_004_mixed_outside_bucket

/-- info: 'SherlockExec.EUI_010_rule_gap' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.EUI_010_rule_gap

/-- info: 'SherlockExec.MYC_CHIP_004_statement_length' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.MYC_CHIP_004_statement_length

/-- info: 'SherlockExec.measured_ers005' does not depend on any axioms -/
#guard_msgs in
#print axioms SherlockExec.measured_ers005

/-- info: 'SherlockExec.apply_of_enabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.apply_of_enabled

/-- info: 'SherlockExec.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_init

/-- info: 'SherlockExec.inv_load' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_load

/-- info: 'SherlockExec.inv_admit' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_admit

/-- info: 'SherlockExec.inv_tape' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_tape

/-- info: 'SherlockExec.inv_execute' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_execute

/-- info: 'SherlockExec.inv_build' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_build

/-- info: 'SherlockExec.inv_prove' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_prove

/-- info: 'SherlockExec.inv_recurse' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_recurse

/-- info: 'SherlockExec.inv_send' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_send

/-- info: 'SherlockExec.inv_newCall' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_newCall

/-- info: 'SherlockExec.apply_disabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.apply_disabled

/-- info: 'SherlockExec.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_apply

/-- info: 'SherlockExec.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_stepOp

/-- info: 'SherlockExec.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_runOps

/-- info: 'SherlockExec.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.inv_run

/-- info: 'SherlockExec.apply_ukey' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.apply_ukey

/-- info: 'SherlockExec.runOps_ukey' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.runOps_ukey

/-- info: 'SherlockExec.EUI_003_program_is_data' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.EUI_003_program_is_data

/-- info: 'SherlockExec.EUI_001_one_family' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EUI_001_one_family

/-- info: 'SherlockExec.ERS_006_prog_d' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_006_prog_d

/-- info: 'SherlockExec.ERS_015_app_key_under_root' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_015_app_key_under_root

/-- info: 'SherlockExec.EKT_004_no_mixed_hash' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EKT_004_no_mixed_hash

/-- info: 'SherlockExec.EUI_004_rsl_two_outcomes' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EUI_004_rsl_two_outcomes

/-- info: 'SherlockExec.EUI_005_one_outcome_kind' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EUI_005_one_outcome_kind

/-- info: 'SherlockExec.EKT_011_alu_canonical' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EKT_011_alu_canonical

/-- info: 'SherlockExec.ERS_005_native_comm_and_W' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_005_native_comm_and_W

/-- info: 'SherlockExec.ERS_003_public_rows' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_003_public_rows

/-- info: 'SherlockExec.EUI_008_static_steps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EUI_008_static_steps

/-- info: 'SherlockExec.ERS_009_rejected_rsl_no_statement' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_009_rejected_rsl_no_statement

/-- info: 'SherlockExec.ERS_010_at_most_one_statement' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_010_at_most_one_statement

/-- info: 'SherlockExec.ERS_007_input_binds_tape' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_007_input_binds_tape

/-- info: 'SherlockExec.ERS_008_halted_payload' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_008_halted_payload

/-- info: 'SherlockExec.EUI_007_no_proof_without_tape' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EUI_007_no_proof_without_tape

/-- info: 'SherlockExec.EKT_007_kit_relation' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.EKT_007_kit_relation

/-- info: 'SherlockExec.ERS_012_bound_into_recursion' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_012_bound_into_recursion

/-- info: 'SherlockExec.ERS_002_private_stays' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.ERS_002_private_stays

/-- info: 'SherlockExec.ERS_011_box_independent' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.ERS_011_box_independent

/-- info: 'SherlockExec.MYC_KEY_004_prog_d_injective' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms SherlockExec.MYC_KEY_004_prog_d_injective

/-- info: 'SherlockExec.MYC_KEY_003_prog_d_ignores_ukey' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.MYC_KEY_003_prog_d_ignores_ukey

/-- info: 'SherlockExec.pin_missing_field' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_missing_field

/-- info: 'SherlockExec.pin_wrong_type' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_wrong_type

/-- info: 'SherlockExec.pin_field_15_outside' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_field_15_outside

/-- info: 'SherlockExec.pin_admit' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_admit

/-- info: 'SherlockExec.pin_app_key' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_app_key

/-- info: 'SherlockExec.pin_version' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_version

/-- info: 'SherlockExec.pin_mixed_hash' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_mixed_hash

/-- info: 'SherlockExec.pin_mixed_interp' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_mixed_interp

/-- info: 'SherlockExec.pin_sha_only' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_sha_only

/-- info: 'SherlockExec.pin_tape_selector' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_tape_selector

/-- info: 'SherlockExec.pin_tape_noncanonical' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_tape_noncanonical

/-- info: 'SherlockExec.pin_tape_missing' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_tape_missing

/-- info: 'SherlockExec.pin_tape_leftover' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_tape_leftover

/-- info: 'SherlockExec.pin_halt' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_halt

/-- info: 'SherlockExec.pin_static' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_static

/-- info: 'SherlockExec.pin_alu' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_alu

/-- info: 'SherlockExec.pin_rsl_no_exhausted' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_rsl_no_exhausted

/-- info: 'SherlockExec.pin_interp_exhausted' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_interp_exhausted

/-- info: 'SherlockExec.pin_rsl_reject' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_rsl_reject

/-- info: 'SherlockExec.pin_interp_reject' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_interp_reject

/-- info: 'SherlockExec.pin_statement' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_statement

/-- info: 'SherlockExec.pin_kit' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_kit

/-- info: 'SherlockExec.pin_unbound' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_unbound

/-- info: 'SherlockExec.pin_bound' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_bound

/-- info: 'SherlockExec.pin_private_out' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_private_out

/-- info: 'SherlockExec.pin_private_owned' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_private_owned

/-- info: 'SherlockExec.pin_publics_out' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_publics_out

/-- info: 'SherlockExec.pin_publics_need_stmt' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_publics_need_stmt

/-- info: 'SherlockExec.pin_two_families' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_two_families

/-- info: 'SherlockExec.pin_refuse_code' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_refuse_code

/-- info: 'SherlockExec.pin_refuse_flag' depends on axioms: [propext] -/
#guard_msgs in
#print axioms SherlockExec.pin_refuse_flag
