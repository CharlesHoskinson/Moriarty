import SherlockCodec.AuditCmd
import Governance.Consistency
import Governance.Props
import Governance.Spec
import Governance.Steps
import Governance.Tests

/-! # Audit of Governance

`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,
Classical.choice, Quot.sound), and `#audit_ns Governance` pinned at `[]`. -/

/-- info: [] -/
#guard_msgs in
#audit_ns Governance

/-- info: 'Governance.consistent' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.consistent

/-- info: 'Governance.CRR_002_refuses_unsealed' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.CRR_002_refuses_unsealed

/-- info: 'Governance.CRR_005_refuses_era_key' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.CRR_005_refuses_era_key

/-- info: 'Governance.EGV_008_refuses_short_rtl' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.EGV_008_refuses_short_rtl

/-- info: 'Governance.EGV_010_refuses_late' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.EGV_010_refuses_late

/-- info: 'Governance.measured_ELD_006' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.measured_ELD_006

/-- info: 'Governance.inv_init' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.inv_init

/-- info: 'Governance.ite_t' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.ite_t

/-- info: 'Governance.ite_f' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.ite_f

/-- info: 'Governance.invA_of_core' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.invA_of_core

/-- info: 'Governance.invA_refuse' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_refuse

/-- info: 'Governance.invA_rotate' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_rotate

/-- info: 'Governance.invA_cancelRotation' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_cancelRotation

/-- info: 'Governance.invA_attest' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_attest

/-- info: 'Governance.invA_vkChange' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_vkChange

/-- info: 'Governance.invA_report' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_report

/-- info: 'Governance.invA_addEraRow' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_addEraRow

/-- info: 'Governance.invA_rekeyStart' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_rekeyStart

/-- info: 'Governance.invA_useAttested' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_useAttested

/-- info: 'Governance.invA_useClaim' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_useClaim

/-- info: 'Governance.invA_acceptCall' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invA_acceptCall

/-- info: 'Governance.setEra_eq' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.setEra_eq

/-- info: 'Governance.era_eq' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.era_eq

/-- info: 'Governance.invA_tick' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_tick

/-- info: 'Governance.invA_deploy' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_deploy

/-- info: 'Governance.invA_premise' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_premise

/-- info: 'Governance.invA_announce' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_announce

/-- info: 'Governance.invA_doActivate' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_doActivate

/-- info: 'Governance.invA_activate' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_activate

/-- info: 'Governance.invA_cancel' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_cancel

/-- info: 'Governance.invA_endPart' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_endPart

/-- info: 'Governance.invA_soundnessEnd' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_soundnessEnd

/-- info: 'Governance.invB_of_bcore' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_of_bcore

/-- info: 'Governance.bcore_setEra' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.bcore_setEra

/-- info: 'Governance.bcore_doActivate' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.bcore_doActivate

/-- info: 'Governance.invB_soundnessEnd' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_soundnessEnd

/-- info: 'Governance.invB_tick' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_tick

/-- info: 'Governance.invB_activate' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_activate

/-- info: 'Governance.invB_cancel' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_cancel

/-- info: 'Governance.invB_rotate' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_rotate

/-- info: 'Governance.invB_cancelRotation' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_cancelRotation

/-- info: 'Governance.invB_attest' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_attest

/-- info: 'Governance.invB_vkChange' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_vkChange

/-- info: 'Governance.invB_report' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_report

/-- info: 'Governance.invB_acceptCall' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_acceptCall

/-- info: 'Governance.invB_useClaim' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_useClaim

/-- info: 'Governance.invB_useAttested' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_useAttested

/-- info: 'Governance.invB_announce' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_announce

/-- info: 'Governance.invB_deploy' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invB_deploy

/-- info: 'Governance.invB_premise' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_premise

/-- info: 'Governance.invB_addEraRow' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.invB_addEraRow

/-- info: 'Governance.invB_rekeyStart' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invB_rekeyStart

/-- info: 'Governance.invB_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invB_apply

/-- info: 'Governance.invA_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.invA_apply

/-- info: 'Governance.inv_apply' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.inv_apply

/-- info: 'Governance.inv_step' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.inv_step

/-- info: 'Governance.inv_runOps' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.inv_runOps

/-- info: 'Governance.inv_run' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.inv_run

/-- info: 'Governance.apply_en' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.apply_en

/-- info: 'Governance.apply_dis' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.apply_dis

/-- info: 'Governance.setEra_current' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.setEra_current

/-- info: 'Governance.era_setEra' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.era_setEra

/-- info: 'Governance.GOV_007_ids_in_order' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_007_ids_in_order

/-- info: 'Governance.GOV_003_one_current' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_003_one_current

/-- info: 'Governance.GOV_002_one_pending' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_002_one_pending

/-- info: 'Governance.ERT_002_table_gate_first' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.ERT_002_table_gate_first

/-- info: 'Governance.EGV_005_rekey_gate' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.EGV_005_rekey_gate

/-- info: 'Governance.GOV_001_quorum_announce' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.GOV_001_quorum_announce

/-- info: 'Governance.GOV_001_quorum_cancel' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.GOV_001_quorum_cancel

/-- info: 'Governance.GOV_001_quorum_soundnessEnd' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.GOV_001_quorum_soundnessEnd

/-- info: 'Governance.GOV_001_quorum_rotate' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.GOV_001_quorum_rotate

/-- info: 'Governance.GOV_002_announce' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_002_announce

/-- info: 'Governance.GOV_004_too_early' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_004_too_early

/-- info: 'Governance.ERT_009_grid' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.ERT_009_grid

/-- info: 'Governance.ERT_003_keyset' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.ERT_003_keyset

/-- info: 'Governance.ERT_003_ledger' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.ERT_003_ledger

/-- info: 'Governance.ELD_002_ledger_gate' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.ELD_002_ledger_gate

/-- info: 'Governance.GOV_003_activate_at_start' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_003_activate_at_start

/-- info: 'Governance.GOV_007_cancel' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_007_cancel

/-- info: 'Governance.GOV_007_cancel_late' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_007_cancel_late

/-- info: 'Governance.GOV_005_rotation_frame' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.GOV_005_rotation_frame

/-- info: 'Governance.GOV_005_rotation_effect' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.GOV_005_rotation_effect

/-- info: 'Governance.GOV_011_cancel_rotation' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_011_cancel_rotation

/-- info: 'Governance.GOV_008_attest_delay' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_008_attest_delay

/-- info: 'Governance.GOV_010_nonce' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in
#print axioms Governance.GOV_010_nonce

/-- info: 'Governance.EGV_007_vk_change' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.EGV_007_vk_change

/-- info: 'Governance.EGV_003_path_evidence' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.EGV_003_path_evidence

/-- info: 'Governance.EGV_002_table_readback' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.EGV_002_table_readback

/-- info: 'Governance.EGV_002_ledger_readback' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.EGV_002_ledger_readback

/-- info: 'Governance.ELD_004_refuse_call' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.ELD_004_refuse_call

/-- info: 'Governance.ELD_007_fallback_no_check' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.ELD_007_fallback_no_check

/-- info: 'Governance.CRR_004_unsafe_era' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.CRR_004_unsafe_era

/-- info: 'Governance.CRR_006_check_at_use' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.CRR_006_check_at_use

/-- info: 'Governance.CRR_007_use_writes_own' depends on axioms: [Quot.sound] -/
#guard_msgs in
#print axioms Governance.CRR_007_use_writes_own

/-- info: 'Governance.EGV_009_premise_once' depends on axioms: [propext] -/
#guard_msgs in
#print axioms Governance.EGV_009_premise_once

/-- info: 'Governance.pin_quorum' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_quorum

/-- info: 'Governance.pin_table_first' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_table_first

/-- info: 'Governance.pin_too_early' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_too_early

/-- info: 'Governance.pin_announce' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_announce

/-- info: 'Governance.pin_grid' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_grid

/-- info: 'Governance.pin_ledger_not_deployed' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_ledger_not_deployed

/-- info: 'Governance.pin_threshold0' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_threshold0

/-- info: 'Governance.pin_threshold2' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_threshold2

/-- info: 'Governance.pin_activate_early' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_activate_early

/-- info: 'Governance.pin_activate' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_activate

/-- info: 'Governance.pin_keyset_reuse' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_keyset_reuse

/-- info: 'Governance.pin_ledger_reuse' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_ledger_reuse

/-- info: 'Governance.pin_handover' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_handover

/-- info: 'Governance.pin_before_handover' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_before_handover

/-- info: 'Governance.pin_end_once' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_end_once

/-- info: 'Governance.pin_cancel' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_cancel

/-- info: 'Governance.pin_cancel_holds' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_cancel_holds

/-- info: 'Governance.pin_cancel_late' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_cancel_late

/-- info: 'Governance.pin_soundness' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_soundness

/-- info: 'Governance.pin_soundness_due' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_soundness_due

/-- info: 'Governance.pin_soundness_none' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_soundness_none

/-- info: 'Governance.pin_eld_refuse' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_eld_refuse

/-- info: 'Governance.pin_eld_cancelled' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_eld_cancelled

/-- info: 'Governance.pin_eld_accept' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_eld_accept

/-- info: 'Governance.pin_fallback' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_fallback

/-- info: 'Governance.pin_unsafe' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_unsafe

/-- info: 'Governance.pin_use' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_use

/-- info: 'Governance.pin_rekey_refused' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_rekey_refused

/-- info: 'Governance.pin_rekey_row' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_rekey_row

/-- info: 'Governance.pin_rekey_fallback' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_rekey_fallback

/-- info: 'Governance.pin_rekey_a1' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_rekey_a1

/-- info: 'Governance.pin_premise' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_premise

/-- info: 'Governance.pin_vk_change' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_vk_change

/-- info: 'Governance.pin_path_evidence' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_path_evidence

/-- info: 'Governance.pin_rotate' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_rotate

/-- info: 'Governance.pin_cancel_rotation' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_cancel_rotation

/-- info: 'Governance.pin_no_rotation' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_no_rotation

/-- info: 'Governance.pin_attest' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_attest

/-- info: 'Governance.pin_nonce' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_nonce

/-- info: 'Governance.pin_refuse_code' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_refuse_code

/-- info: 'Governance.pin_refuse_tick' does not depend on any axioms -/
#guard_msgs in
#print axioms Governance.pin_refuse_tick
