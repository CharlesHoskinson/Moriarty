# CallFlow oracle: requirement table

Package `formal/sherlock-e2e/call-flow/lean`, build `lake build --wfail` (odin), 0 sorry. Wire names and op codes: `../ORACLE.md`.

| ID | sentence (verbatim from inv.json) | Lean name(s) | state |
|---|---|---|---|
| EUF-001 | When a user runs one end-to-end call, the Sherlock toolchain SHALL run the stages in this order: compile, setup, plan, deploy, tape, execute, prove, wrap, ledger check, submit and read-back. | `EUF_001_order`, `Op.step` enabled only for stage + 1 | MODELED (replay pending) |
| EUF-002 | The Sherlock toolchain SHALL give each stage only the artifacts and the private state references that the stage dependency table of `design.md` names for the stage. | `EUF_002_one_deploy_per_program`, `Inv.deployedFirst` | MODELED (replay pending) |
| EUF-005 | If a stage before submit fails, then the Sherlock toolchain SHALL send no transaction for the call and record the stage name and the error. | `EUF_005_no_tx_after_stop`, `Inv.stoppedNoTx`, `Inv.txAfterSubmit` | MODELED (replay pending) |
| EUF-006 | If the execute stage rejects a Compact call, then the Sherlock toolchain SHALL start no prove stage and no submit stage for the call. | `EUF_006_native_refusal_no_prove`, `Inv.rejected` | MODELED (replay pending) |
| EUF-007 | While a box has no profile for the installed universal key set, the Sherlock toolchain SHALL refuse a prove stage on the box. | `EUF_007_prove_needs_profile`, `Inv.proved`, pin `pin_stale_profile` | MODELED (replay pending) |
| EUF-008 | When two boxes that the user owns prove the same call, the Sherlock prover SHALL give byte-equal `claim_pub` values. | `EUF_008_one_claim`, `Inv.oneClaim`, pin `pin_two_boxes` | MODELED (replay pending) |
| EUF-009 | When a developer deploys a second program, the Sherlock toolchain SHALL use the wrap verifier key of the first program, byte for byte. | `EUF_009_one_wrap_key`, `Inv.oneKey`, pin `pin_second_program` | MODELED (replay pending) |
| EUF-010 | When a settled call is read back, the Sherlock toolchain SHALL compare `runs[claim_pub]` with `outcome_pub` and the outcome with the native transcript of the call. | `EUF_010_outcome_after_readback`, `Inv.outcomeOk`, pin `pin_readback_differs` | MODELED (replay pending) |
| EUF-011 | The Sherlock toolchain SHALL run the flow of EUF-001 with no DoctorWatson, herdr, box bus or agent tool on the user's box. | Consistency `EUF_011_ok`, `euf011_today_open` | SPEC-CHECKED |
| EUF-012 | If the `compactc` version, the ZKIR version, the machine version, the universal key-set identity or the ledger version of an artifact differs from the pin of the run, then the Sherlock toolchain SHALL stop the call and name the differing pin. | `EUF_012_pin_stops`, `Inv.pinNamed`, pin `pin_zkir_pin`, Consistency `EUF_012_ledger_ok` | MODELED (replay pending) |
| EUF-013 | When the prototype is reported complete, the Sherlock toolchain SHALL show, for one universal key set, the flow of EUF-001 on every suite call that is not refused-as-native or n/a. | `EUF_013_complete_joined`, `Inv.completeOk`, pins `pin_complete`, `pin_incomplete`, `pin_mixed` | MODELED (replay pending) |
| EUF-014 | If a submitted call has no read-back, then the Sherlock toolchain SHALL reconcile the call by its transaction hash and send no second transaction for the same `claim_pub`. | `EUF_014_one_tx_per_claim`, `Inv.oneTx`, pins `pin_reconcile`, `pin_no_resend` | MODELED (replay pending) |
| LPC-014 | When every row is DONE with a lead verdict or waits on the owner, the tested-design loop SHALL present the interface freeze to the owner. | Consistency `LPC_014_ok` | SPEC-CHECKED |
| LPC-017 | When an owner ruling changes a formula, the tested-design loop SHALL update FINAL-DESIGN.md and ARCHITECTURE.md to the ruled form. | Consistency `LPC_017_ok` | SPEC-CHECKED |
| MYC-ACC-004 | The Mycroft host shall set the transcript version to the ledger's current one (2.3, `onchain-runtime/src/transcript.rs:89-90`). | Consistency `MYC_ACC_004_ok`, `EUF_012_ledger_ok` | SPEC-CHECKED |
| MYC-KEY-002 | When a compiled contract is deployed, the Mycroft host shall store the one universal wrap key as the verifier key of every entry. | `EUF_009_one_wrap_key`, `Inv.oneKey` | MODELED (replay pending) |
| MYC-SEAL-015 | The universal seal shall use one verifier key for every contract, entry and run, generated with no program, contract or entry as input (X6). | `EUF_009_one_wrap_key`, `Inv.oneKey` | MODELED (replay pending) |
