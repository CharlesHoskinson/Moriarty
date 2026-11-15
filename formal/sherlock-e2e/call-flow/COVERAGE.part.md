| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |
|---|---|---|---|---|---|---|
| EUF-001 | A | call-flow | MODELED | `EUF_001_order`, `Op.step` enabled only for stage + 1 | pending (Formal-Quint) | stages 1..11 in order |
| EUF-002 | A | call-flow | MODELED | `EUF_002_one_deploy_per_program`, `Inv.deployedFirst` | pending (Formal-Quint) | state part: deployment before any per-call stage; a second call starts after deploy. The artifact table itself is not a transition |
| EUF-005 | A | call-flow | MODELED | `EUF_005_no_tx_after_stop`, `Inv.stoppedNoTx`, `Inv.txAfterSubmit` | pending (Formal-Quint) | failedStage names the stage |
| EUF-006 | A | call-flow | MODELED | `EUF_006_native_refusal_no_prove`, `Inv.rejected` | pending (Formal-Quint) | stop at execute |
| EUF-007 | A | call-flow | MODELED | `EUF_007_prove_needs_profile`, `Inv.proved`, pin `pin_stale_profile` | pending (Formal-Quint) | prove stops with the key-set pin when the profile is stale |
| EUF-008 | A | call-flow | MODELED | `EUF_008_one_claim`, `Inv.oneClaim`, pin `pin_two_boxes` | pending (Formal-Quint) | claimA = claimB = claim |
| EUF-009 | A | call-flow | MODELED | `EUF_009_one_wrap_key`, `Inv.oneKey`, pin `pin_second_program` | pending (Formal-Quint) | a different key is refused (vkRefused) |
| EUF-010 | A | call-flow | MODELED | `EUF_010_outcome_after_readback`, `Inv.outcomeOk`, pin `pin_readback_differs` | pending (Formal-Quint) | outcome 1 equal, 2 differs |
| EUF-011 | A | call-flow | SPEC-CHECKED | Consistency `EUF_011_ok`, `euf011_today_open` | n/a | no state machine: a deployment property (no agent tooling on the user box). Today open: the EUF-011 status names t7_chain, t17.sh and t23.py on the box bus |
| EUF-012 | A | call-flow | MODELED | `EUF_012_pin_stops`, `Inv.pinNamed`, pin `pin_zkir_pin`, Consistency `EUF_012_ledger_ok` | pending (Formal-Quint) | lowest differing pin named |
| EUF-013 | A | call-flow | MODELED | `EUF_013_complete_joined`, `Inv.completeOk`, pins `pin_complete`, `pin_incomplete`, `pin_mixed` | pending (Formal-Quint) | complete only with every eligible call joined under one key set |
| EUF-014 | A | call-flow | MODELED | `EUF_014_one_tx_per_claim`, `Inv.oneTx`, pins `pin_reconcile`, `pin_no_resend` | pending (Formal-Quint) | reconcile by tx hash; resend refused |
| LPC-014 | completion | completion | SPEC-CHECKED | Consistency `LPC_014_ok` | n/a | no state machine: a loop-process condition |
| LPC-017 | completion | completion | SPEC-CHECKED | Consistency `LPC_017_ok` | n/a | no state machine: a document-update condition |
| MYC-ACC-004 | reused | call-flow | SPEC-CHECKED | Consistency `MYC_ACC_004_ok`, `EUF_012_ledger_ok` | n/a | no state machine: transcript version 2.3 is a pin value (onchain-runtime/src/transcript.rs:89-90 per the sentence) |
| MYC-KEY-002 | reused | call-flow | MODELED | `EUF_009_one_wrap_key`, `Inv.oneKey` | pending (Formal-Quint) | every deployment stores universalVk |
| MYC-SEAL-015 | reused | call-flow | MODELED | `EUF_009_one_wrap_key`, `Inv.oneKey` | pending (Formal-Quint) | one key `vk` as param, with no program input |
