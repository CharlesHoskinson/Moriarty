import SherlockCodec.AuditCmd
import NetworkOps.Consistency
import NetworkOps.Spec
import NetworkOps.Tests

/-! # Audit of NetworkOps

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns NetworkOps` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns NetworkOps

/-- info: 'NetworkOps.consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.consistent

/-- info: 'NetworkOps.OBS_004_missing_control_fails' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.OBS_004_missing_control_fails

/-- info: 'NetworkOps.OBS_003_forbidden_fails' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.OBS_003_forbidden_fails

/-- info: 'NetworkOps.OBS_003_clean_passes' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.OBS_003_clean_passes

/-- info: 'NetworkOps.OBS_009_sum_of_measured' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.OBS_009_sum_of_measured

/-- info: 'NetworkOps.OBS_005_lines' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.OBS_005_lines

/-- info: 'NetworkOps.FRC_001_marker' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.FRC_001_marker

/-- info: 'NetworkOps.FRC_011_LPC_015_agree' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.FRC_011_LPC_015_agree

/-- info: 'NetworkOps.FRC_011_holds_other_branch' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.FRC_011_holds_other_branch

/-- info: 'NetworkOps.FRC_012_no_call_twice' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.FRC_012_no_call_twice

/-- info: 'NetworkOps.FRC_012_scenario' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.FRC_012_scenario

/-- info: 'NetworkOps.WAL_009_refuses' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.WAL_009_refuses

/-- info: 'NetworkOps.SCL_009_stops' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.SCL_009_stops

/-- info: 'NetworkOps.loop_consistent' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.loop_consistent

/-- info: 'NetworkOps.LPC_008_with_PVW_010' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.LPC_008_with_PVW_010

/-- info: 'NetworkOps.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_init

/-- info: 'NetworkOps.inv_start' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_start

/-- info: 'NetworkOps.inv_chain' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_chain

/-- info: 'NetworkOps.inv_state' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_state

/-- info: 'NetworkOps.inv_modes' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_modes

/-- info: 'NetworkOps.inv_addr' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_addr

/-- info: 'NetworkOps.inv_lock' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.inv_lock

/-- info: 'NetworkOps.inv_deploy' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_deploy

/-- info: 'NetworkOps.inv_confirm' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_confirm

/-- info: 'NetworkOps.inv_prove' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_prove

/-- info: 'NetworkOps.inv_rerun' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_rerun

/-- info: 'NetworkOps.inv_balance' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_balance

/-- info: 'NetworkOps.inv_submit' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_submit

/-- info: 'NetworkOps.inv_outcome' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_outcome

/-- info: 'NetworkOps.inv_resolve' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_resolve

/-- info: 'NetworkOps.inv_readback' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_readback

/-- info: 'NetworkOps.inv_refusals' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_refusals

/-- info: 'NetworkOps.inv_log' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_log

/-- info: 'NetworkOps.inv_finish' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_finish

/-- info: 'NetworkOps.inv_resync' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_resync

/-- info: 'NetworkOps.inv_batch' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_batch

/-- info: 'NetworkOps.apply_disabled' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.apply_disabled

/-- info: 'NetworkOps.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_apply

/-- info: 'NetworkOps.inv_stepOp' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_stepOp

/-- info: 'NetworkOps.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_runOps

/-- info: 'NetworkOps.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.inv_run

/-- info: 'NetworkOps.PVW_001_no_wallet_on_other_chain' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.PVW_001_no_wallet_on_other_chain

/-- info: 'NetworkOps.PVW_002_two_chains' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.PVW_002_two_chains

/-- info: 'NetworkOps.SCL_007_ttl' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.SCL_007_ttl

/-- info: 'NetworkOps.SCL_003_dry_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.SCL_003_dry_run

/-- info: 'NetworkOps.WAL_004_lock' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.WAL_004_lock

/-- info: 'NetworkOps.FRC_002_one_deploy' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.FRC_002_one_deploy

/-- info: 'NetworkOps.PVW_013_final_gate' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.PVW_013_final_gate

/-- info: 'NetworkOps.PVW_010_final_on_gpu' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.PVW_010_final_on_gpu

/-- info: 'NetworkOps.FRC_004_attempts' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.FRC_004_attempts

/-- info: 'NetworkOps.FRC_006_refusals_need_readback' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.FRC_006_refusals_need_readback

/-- info: 'NetworkOps.PVW_006_four_controls' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.PVW_006_four_controls

/-- info: 'NetworkOps.SCL_002_exit_codes' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.SCL_002_exit_codes

/-- info: 'NetworkOps.PVW_012_failed_control_fails' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.PVW_012_failed_control_fails

/-- info: 'NetworkOps.FRC_005_no_reuse_of_refused' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.FRC_005_no_reuse_of_refused

/-- info: 'NetworkOps.FRC_003_reuse' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.FRC_003_reuse

/-- info: 'NetworkOps.FRC_014_hash_checked' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.FRC_014_hash_checked

/-- info: 'NetworkOps.FRC_013_no_submit_while_unknown' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.FRC_013_no_submit_while_unknown

/-- info: 'NetworkOps.WAL_002_redactions' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms NetworkOps.WAL_002_redactions

/-- info: 'NetworkOps.PVW_011_validium_refuses' depends on axioms: [propext] -/
#guard_msgs in
#print axioms NetworkOps.PVW_011_validium_refuses

/-- info: 'NetworkOps.ttlMax_derived' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.ttlMax_derived

/-- info: 'NetworkOps.reuse_and_attempts' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.reuse_and_attempts

/-- info: 'NetworkOps.pin_pass' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_pass

/-- info: 'NetworkOps.pin_ttl0' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_ttl0

/-- info: 'NetworkOps.pin_ttl_bound' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_ttl_bound

/-- info: 'NetworkOps.pin_ttl_ok' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_ttl_ok

/-- info: 'NetworkOps.pin_chain' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_chain

/-- info: 'NetworkOps.pin_devnet' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_devnet

/-- info: 'NetworkOps.pin_state' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_state

/-- info: 'NetworkOps.pin_modes' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_modes

/-- info: 'NetworkOps.pin_addr' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_addr

/-- info: 'NetworkOps.pin_gate' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_gate

/-- info: 'NetworkOps.pin_deploy_resume' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_deploy_resume

/-- info: 'NetworkOps.pin_final_cpu' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_final_cpu

/-- info: 'NetworkOps.pin_not_owned' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_not_owned

/-- info: 'NetworkOps.pin_attempts' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_attempts

/-- info: 'NetworkOps.pin_other_error' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_other_error

/-- info: 'NetworkOps.pin_resync' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_resync

/-- info: 'NetworkOps.pin_offhost' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_offhost

/-- info: 'NetworkOps.pin_r2' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_r2

/-- info: 'NetworkOps.pin_noverdict' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_noverdict

/-- info: 'NetworkOps.pin_ctrl_timeout' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_ctrl_timeout

/-- info: 'NetworkOps.pin_timeout' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_timeout

/-- info: 'NetworkOps.pin_resolve' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_resolve

/-- info: 'NetworkOps.pin_refused' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_refused

/-- info: 'NetworkOps.pin_reuse' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_reuse

/-- info: 'NetworkOps.pin_no_reuse_300' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_no_reuse_300

/-- info: 'NetworkOps.pin_no_reuse_hash' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_no_reuse_hash

/-- info: 'NetworkOps.pin_no_readback' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_no_readback

/-- info: 'NetworkOps.pin_exit1' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_exit1

/-- info: 'NetworkOps.pin_dry' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_dry

/-- info: 'NetworkOps.pin_redact' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_redact

/-- info: 'NetworkOps.pin_lock' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_lock

/-- info: 'NetworkOps.pin_validium' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_validium

/-- info: 'NetworkOps.pin_not_op' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_not_op

/-- info: 'NetworkOps.pin_disabled' does not depend on any axioms -/
#guard_msgs in
#print axioms NetworkOps.pin_disabled
