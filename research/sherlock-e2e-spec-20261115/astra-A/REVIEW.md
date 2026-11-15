# Astra-A phase 2 review

Review scope: proposal, the six slice-A capability specs, D-001 through D-027, DDD, the decision index and COVERAGE. The lead changed files during this review. D-023 was read before its cancellation. The final index records that cancellation. DDD, INDEX and COVERAGE appeared during the review and were read before completion.

This review used file reads and `rg`. No test, build, prover, git command or network request ran. Existing test results below retain their source's scope.

## Source names

| Name | Exact path |
|---|---|
| CHANGE | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/openspec/changes/sherlock-e2e-prototype-20261115` |
| COMPILER | `CHANGE/specs/sherlock-e2e-compact-compiler/spec.md` |
| STATEMENT | `CHANGE/specs/sherlock-e2e-rsl-statement/spec.md` |
| KITS | `CHANGE/specs/sherlock-e2e-rsl-kits/spec.md` |
| INTERPRETER | `CHANGE/specs/sherlock-e2e-universal-interpreter/spec.md` |
| SUITE | `CHANGE/specs/sherlock-e2e-compact-suite/spec.md` |
| FLOW | `CHANGE/specs/sherlock-e2e-user-flow/spec.md` |
| DDD | `CHANGE/architecture/DDD.md` |
| COVERAGE | `CHANGE/architecture/COVERAGE.md` |
| INDEX | `CHANGE/architecture/decisions/INDEX.md` |
| MYC | `/home/hoskinson/Projects/Moriarty-wt/mycroft-factory/mycroft` |
| RSL | `/home/hoskinson/.cache/m7-wrap/src/openvm-rekey-chips/crates/sherlock-leaf` |
| COMPLETE | `/home/hoskinson/Projects/Moriarty-wt/spec-audit/openspec/changes/sherlock-completion-20261115` |
| FINAL | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/openspec/changes/sherlock-processor-final-20261111` |
| UI | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/openspec/changes/sherlock-universal-interpreter-design` |
| MOD | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/openspec/changes/sherlock-modular-leaf-20261111` |
| T23 | `/home/hoskinson/Projects/Moriarty-wt/t23-suite/tools/t23-suite/t23.py` |
| MAP | `/home/hoskinson/Projects/Moriarty-wt/t23-args/research/sherlock-t23-args-20261115/MAP.md` |
| ARGS | `/home/hoskinson/Projects/Moriarty-wt/t23-args/tools/t23-suite/accept_args.py` |
| LOOP | `/home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` |
| DECISIONS | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/research/sherlock-final-design-20261111/DECISIONS.md` |
| BUS | `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/research/sherlock-e2e-spec-20261115/bus` |
| WIKI | `/home/hoskinson/Projects/Moriarty/llm-wiki/wiki/log.md` |

All file:line references use these expansions. References to a new requirement identify proposed text, not implemented behavior.

## Check of the adopted draft findings

The following are not reported again as newly discovered gaps. Findings below concern their wording, verification or integration.

| Adopted ID | Check |
|---|---|
| ERS-012 | The binding gap and zero-valued probe are stated correctly. The demanded FP-1 representation needs correction, R-02. |
| ERS-013 | Correctly identifies the debug-string producer and instruction-array consumer. Preserve its round-trip check. |
| ERS-014 | Correctly identifies unchecked JSON and specifies a named refusal. Preserve its negative parser check. |
| ERS-015 | Correctly permits authenticated per-program app keys and supersedes the overbroad MYC-REF-052 clause. |
| EUI-010 | Correctly distinguishes five subset rules, two value rules and the 120-rule target. Its proof check covers too little, R-06. |
| ECC-014 | Correctly distinguishes internal s01 Jubjub values from typed Jubjub inputs. Preserve the open input-lowering work. |
| ECS-012 | Correctly requires the comparison denominator. The denominator text already exists in T23. Correct its status, R-22. |
| ECS-013 | Correctly requires an identity map. Preserve separate scenario-call and ContractCall/vector identities. T23:206 and T23:215 show why they differ. |

## Findings

### R-01 — HIGH — A mandatory mixed-hash refusal narrows the language contract

**File and IDs:** KITS:43, EKT-004. `CHANGE/architecture/decisions/D-024.md:20`, D-024.

**Evidence:** EKT-004 requires refusal whenever a program calls both hashes. D-024 selects that limit as a LEAD PICK for T15. The code does refuse it at `RSL/src/rsl/compile.rs:847`. That proves an implementation limit. It does not authorize a language restriction. CPR-005 at FINAL `specs/sherlock-general-purpose/spec.md:49` prohibits requirements that narrow GP-1 through GP-4. LOOP:102 retains that hard rule. D-024 supplies no alternative proving route.

**Exact change requested:** Remove the unconditional normative requirement to refuse valid mixed-hash programs. Record the refusal as an open implementation gap. Keep a temporary named capacity refusal and its regression test. Require a path that preserves both hashes and the universal key rule, or retain this capability as incomplete. State in D-024 that a lead decision cannot authorize a hard-rule exception.

### R-02 — HIGH — ERS-012 prescribes a statement representation that does not fit FP-1

**File and ID:** STATEMENT:147, ERS-012.

**Evidence:** The requirement tells the leaf to read raw `comm` and W from FP-1 public values that equal PUBLIC rows. FP-1 has 54 BabyBear words with assigned digest/counter fields at `RSL/src/core.rs:44`. The probe has the same 54-word width at `RSL/src/rsl/air.rs:223`. PUBLIC stores each Fr word as 32 byte limbs at `air.rs:199`. Even the a01 `test_all` result has 81 effect words, `MYC/compiler/out/suite.json:4` and `:10`. Raw equality cannot preserve this layout. MAP:81 asks for an authenticated outcome digest derived from PUBLIC words.

**Exact change requested:** Replace the raw-word FP-1 requirement with: “When an R_SL app enters recursion, the app statement adapter SHALL bind its constrained `comm` and ordered W to the authenticated outcome commitment consumed by recursion.” Name the retained FP-1 field or versioned statement adapter after agreement with B. Keep T-RSL-STMT-BIND. Add a changed-last-effect mutation on an entry with more than 54 effect words. Do not require another public lane layout through prose alone.

### R-03 — HIGH — The input test and outcome scenario confuse payloads with digests

**File and IDs:** STATEMENT:82, ERS-007 and its Verification at :84. STATEMENT:101, ERS-008 scenario.

**Evidence:** The ERS-007 test rejects a tape digest that differs from `input_d`. Those values normally differ. MAP:38 and ARGS:232 define `input_d = P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)`, with `val_d = vd(tape)` under MAP:73. ERS-008 says `outcome_d` is `Halted(vd(...))`. MAP:39 and ARGS:238 define `outcome_d` as a tagged digest. `Halted(vd(...))` describes the outcome kind and payload, not that digest's type.

**Exact change requested:** Specify the two digest layers explicitly. Make T-RSL-INPUT mutate tape while retaining the original salt, resource fields and resulting `input_d`. In ERS-008, state the Halted payload and then the exact tagged `outcome_d` encoding. Reuse the domain and value-encoding definitions from the map and B's interface. Add the distinction between the tape salt word and the eight-word input-digest salt.

### R-04 — HIGH — RslStatement has no explicit privacy boundary and its scan is unsound

**File and ID:** STATEMENT:121, ERS-010. `CHANGE/architecture/DDD.md:65`, :134 and :241.

**Evidence:** ERS-010 says to “publish” raw `prog_d`, `input_d`, `outcome_d`, `comm` and W. DDD calls this the “public face” and claims “no private word in it.” MYC-CLM-009, `MYC/spec/specs/claims/spec.md:70`, excludes the raw digests from published objects. ZK-002 at the audit-fixed FINAL zero-knowledge spec:15 excludes plain `prog_d` from the published view. Also, MYC-TAPE-012 at `MYC/spec/specs/witness-tape/spec.md:87` permits native effects to contain disclosed witness/read words. A value scan cannot require every tape word to be absent from W. A public effect can also equal a salt by coincidence.

**Exact change requested:** Name this an owned-host-only `PrivateRslStatement`. Apply the ownership restriction to the whole record for its full lifetime. Use `StatementPublics` as the external projection. Replace the value-absence scan with field-level checks, permitted-disclosure checks and a transport test that refuses an unowned destination. Remove “no private word” and “public face” from the raw-statement DDD rows. Keep the four public digest fields distinct from the original call's W.

### R-05 — HIGH — EUI-009 retires interpreter behavior without a replacement map

**File and IDs:** INTERPRETER:108, EUI-009. `CHANGE/architecture/decisions/D-025.md:19`, D-025.

**Evidence:** The normative clause only establishes precedence for an actual conflict. Its scenario makes a missing processor-final stage enough to classify UIREQ text as history. D-025 goes further and retires the whole set. Absence is not a conflicting requirement. UIREQ-003, 004, 005, 008, 014 and 016 define lexical scope, evaluation order, recursion, matching and resumable execution. They have no explicit behavioral replacement in EUI. Existing host tests include lexical scope and recursion at `crates/mori-core/tests/sherlock_original_ears_host_positive.rs:196`, :257 and :315. Checkpoint tests exist at `crates/mori-core/tests/sherlock_technical_chunks_positive.rs:255`. The UI spec also records adopted nominal refinements at UI `specs/universal-interpreter/spec.md:260`. D-025:47 calls the set a design that was not built without these distinctions.

**Exact change requested:** Restrict EUI-009 to named, demonstrated conflicts. Add an ID-by-ID retention or supersession map for UIREQ and the adopted refinements. Preserve applicable language behavior by reference under existing IDs. Name its host evidence and missing proof evidence. Remove blanket retirement and the “not built” claim. Map every behavior to CORE/RFB or a retained UIREQ check before marking the inventory covered.

### R-06 — HIGH — Interpreter acceptance checks do not establish their declared scope

**File and IDs:** INTERPRETER:57 and :63, EUI-005. INTERPRETER:123, EUI-010 Verification.

**Evidence:** EUI-005 calls `entry x;` Halted. The T6 report records `Rejected code 3` at `research/sherlock-t6-leaf-20261113/RESULTS.md:47`, with the exact source and empty-variable case at :50. `RSL/src/core.rs:536` implements error publication. T6 is therefore not a Halted witness. The EUI-005 check also does not identify an Exhausted proof. EUI-010 requires each of 120 rules, but its Verification asks for one proof per rule family. Two family cases cannot establish 120 constrained transitions.

**Exact change requested:** Change `entry x;` to the Rejected witness. Name the T8b nullary-constructor Halted witness separately. Add an Exhausted proof case and each outcome's tag/payload mutations. For EUI-010, require rule-indexed honest and forged-transition evidence for every enabled rule, with explicit reachability reasons where applicable. Preserve RFB-003/004's failure-on-gap rule.

### R-07 — HIGH — The generic user flow forbids provable rejected runs

**File and IDs:** FLOW:69, EUF-006. STATEMENT:108, ERS-009. `CHANGE/architecture/DDD.md:182` and :333.

**Evidence:** EUF-006 applies to any execution rejection in the whole user flow and forbids proof production. GP-001 requires every outcome kind at `openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:4`. EUI-005 retains Rejected and Exhausted outcomes. MAP:75 limits the no-settlement choice to native-refused T23 calls and explicitly preserves a future generic Rejected claim form. EUI-002 uses the general interpreter as part of the demonstration.

**Exact change requested:** Scope EUF-006 and ERS-009 to the Compact call/T23 admission policy. State that a native-refused Compact call emits no successful Compact call statement and is not settled in this route. Preserve general Sherlock proofs of Rejected and Exhausted outcomes. Qualify the DDD “no statement” language by context. Add one generic Rejected proof case to the cross-family demonstration.

### R-08 — MEDIUM — Witness-independent compiler output is checked with a downstream micro-op test

**File and ID:** COMPILER:43, ECC-004 and Verification at :45.

**Evidence:** ECC-004 requires byte-equal Mycroft program images. `rsl_key_independent_of_tape`, `RSL/tests/rsl.rs:60`, loads already emitted files and compares their lowered `ops`. It invokes neither Mycroft compilation nor image serialization. It also does not compare ABI bytes or app key bytes, although the ECC-004 scenario claims key equality.

**Exact change requested:** Reference MYC-FE-009/010 for deterministic, witness-independent compilation. Add a compiler-boundary check that serializes the same source/configuration with two prepared calls and compares image and ABI bytes. Keep the micro-op test as separate downstream evidence. Mark the compiler serialization check to build until it exists.

### R-09 — MEDIUM — The prog_d rekey test leaves its actual inputs uncontrolled

**File and ID:** STATEMENT:71, ERS-006 Verification and scenario at :74.

**Evidence:** The proposed test requires one digest under two universal key sets. `prog_d` includes `key_root`. MYC-KEY-003 defines the exact inputs at `MYC/spec/specs/keys/spec.md:28`. MYC-KEY-008 at :63 permits a new `prog_d` and anchor after rekey. MYC-CLM-023 at `MYC/spec/specs/claims/spec.md:168` includes the image, plan, key root and machine version. Two contracts can also compile to the same image. Their names alone do not require distinct digests.

**Exact change requested:** Hold image, plan, `key_root` and machine version fixed when testing independence from unrelated universal-key metadata. In a separate case, change a bound input and check the resulting identity and anchor. Change the two-contract scenario to two distinct compiled images. Record the existing MYC-KEY-003/008 ambiguity instead of copying its conflicting test language.

### R-10 — MEDIUM — Kit Verification lines claim integration checks that do not exist

**File and IDs:** KITS:97, EKT-008 Verification. KITS:110, EKT-009 Verification. KITS:136, EKT-011 Verification.

**Evidence:** `rsl_air_count_in_universal_leaf_bucket` at `RSL/tests/rsl.rs:191` loops over call files. It does not call the named Jubjub and ECDSA test programs or compare leaf keys. It checks `6..=9`, not exactly 6 or 9. `rsl_secp_chip_program` at `RSL/tests/rsl_secp.rs:318` creates a handwritten JSON program locally. It does not consume Mycroft test output. `rsl_secp_random_field_ops` checks a noncanonical hint, while EKT-011 requires rejection of an exact ALU result above its modulus. These are different conditions. The relevant implementation is `RSL/src/rsl/compile.rs:192`.

**Exact change requested:** Mark the compiler-to-kit and universal-leaf checks to build. Feed actual emitted Mycroft files to the R_SL adapter and check their final leaf pre-hashes. Check exactly the intended AIR counts. Add direct exact-ALU overflow inputs for EKT-011. Keep existing unit tests as limited evidence, with their actual scope.

### R-11 — MEDIUM — EKT-007 requires refusal of every alternative trace

**File and ID:** KITS:82, EKT-007.

**Evidence:** The trigger is any trace row that differs from “the honest row.” A proof relation accepts valid witnesses, not one canonical host trace. Hint values, auxiliary witnesses and inactive cells need only meet the relation. The cited tests mutate particular semantic cells, for example product and quotient limbs in `RSL/tests/rsl_secp.rs:292`. They do not prove uniqueness of every trace cell. `MOp::Hint` is a free word under constraints at `RSL/src/rsl/compile.rs:162`.

**Exact change requested:** State refusal of a trace that violates the kit relation or claims an incorrect result under fixed admitted inputs. Name the unchanged inputs/key/statement in each mutation scenario. Keep valid alternative witnesses admissible. Use the concrete forged-cell tests to check the relation, rather than requiring trace-byte identity.

### R-12 — MEDIUM — Compiler gate results omit the formal proof boundary

**File and IDs:** COMPILER:17, ECC-002 and :134, ECC-011. `CHANGE/architecture/DDD.md:131`.

**Evidence:** ECC-002 cites LPC-019, but only requires factory gates. `MYC/factory/gates.sh:49` builds Lean and its KRS checks compare a finite reference suite. `MYC/lean/SORRIES.md:13` retains S1 limitations, :14 retains S2's run-to-seal premise and :83 excludes typed secp layout and arithmetic from that model. A warning-free theorem build does not discharge a theorem hypothesis. DDD's invariant “equal to the Lean oracle” has no qualification for this scope. FINAL CF-012 at frontend spec:85 separately requires CF-L1.

**Exact change requested:** Bind ECC-002's acceptance to LPC-019, MYC-LOW-019 and CF-012, which COVERAGE:50 and :54 already retain. Require an assumption report for S1, S2, builtin parameters, wide arguments and typed kits. State which vectors ECC-011 covers. Separate a passed finite conformance gate from full compiler or Rust-to-AIR correctness. Retain CF-L1 as an open obligation unless its mapped theorem and axiom report are supplied.

### R-13 — MEDIUM — The flow's immediate-predecessor restriction prevents required inputs

**File and IDs:** FLOW:10, EUF-001 scenario. FLOW:17, EUF-002. `CHANGE/architecture/DDD.md:228`.

**Evidence:** The scenario permits each stage to read only the previous stage's artifact. EUF-002 independently limits the artifact list. Proving needs the admitted program, keys, profile and private call. Wrapping needs a proof and keyed configuration. Submission needs the deployment identity and wallet. MAP:49 and :85 require a private statement bundle that EUF-002 does not name. ERS-010 also expects a deployment salt from an earlier stage. A failed or repeated call cannot redeploy and rerun all setup by definition.

**Exact change requested:** Replace the linear input restriction with an explicit dependency table. Separate per-program compile/plan/deploy, per-machine setup and per-call tape/prove/settle. Permit each stage's named immutable inputs and private state references. Add the private accept bundle, proof/key/profile identities and public argument projection to the handoff types. Test resume from a deployed program without a second deployment.

### R-14 — MEDIUM — A post-submit failure cannot stop the call before submit

**File and ID:** FLOW:56, EUF-005. `CHANGE/architecture/DDD.md:226` and :230.

**Evidence:** EUF-001 includes submit and read-back. EUF-005 says any stage failure stops before submit. A read-back failure occurs after submission and can occur after inclusion. DDD also permits only a named refusal “with no transaction,” while DDD:211 defines node refusal on a submitted call. The existing recovery context distinguishes these stages and forbids resubmission of an included call at DDD:229.

**Exact change requested:** Restrict “no submission” to failures before submission. Add separate post-submit outcomes for node refusal, unknown submission status and included-but-unread state. Reference C's recovery requirements. Add a lost-response/read-back fault scenario that checks receipt reconciliation and prevents duplicate submission.

### R-15 — MEDIUM — The compile command requires success even for invalid input

**File and IDs:** COMPILER:147, ECC-012 and :160, ECC-013.

**Evidence:** ECC-012 triggers whenever `sherlock compile` runs on a compactc build directory, then requires files and exit zero. ECC-013 requires nonzero and no image for a refused circuit. `Refusal::IrVersion`, `MissingKit` and `EffectTooLong` exist at `MYC/compiler/src/lower.rs:64`. The edited-version scenario satisfies both triggers.

**Exact change requested:** Condition ECC-012 on successful admission of the build and every requested proof circuit. Keep the refusal path under ECC-013 and MYC-FE-011. Define whether a multi-entry failure leaves no outputs or a documented partial result. Test the chosen policy with one accepted and one refused circuit in the same request.

### R-16 — MEDIUM — EUI-007 promises a trace-free stable error that the cited check does not establish

**File and ID:** INTERPRETER:82, EUI-007 and its scenario at :88.

**Evidence:** The cited test calls `compile::build` and receives a `Built` value with `failed`, `RSL/tests/rsl.rs:94`. It checks substring “tape length” for both missing and extra words at :88 and :91. The builder records failure and continues building micro-ops at `RSL/src/rsl/compile.rs:598`, :614 and :901. It does not return the scenario's `inputLeftover` code at that boundary. `prove_call` refuses before AIR/proof construction at `RSL/src/rsl/mod.rs:42`.

**Exact change requested:** Specify the intended boundary: no proof begins after invalid tape admission. Name the stable returned error separately if required. Mark that error normalization to build. Test the public executor response and a proof-start spy. Do not cite an assertion on `Built.failed` as evidence of no intermediate execution representation.

### R-17 — HIGH — Suite completion lacks an executable evidence-completeness predicate

**File and IDs:** SUITE:121, ECS-010. FLOW:160 and :162, EUF-013. Adopted ECS-012 and ECS-013 remain required.

**Evidence:** EUF-013's normative scope requires the joined flow for every eligible call, but its Verification names mismatch zero and T12 gate logs. Those checks do not reject a missing per-call proof, ledger check, receipt or readback. Current `T23:371` supports dry settlement, and `classify` can assign pass after that dry result at T23:448 and :452. Live settlement is a deliberate stop at T23:378. Also, `compare` loops only over supplied result vectors at T23:222. An empty result list reaches “equal” at T23:240. A result-class count or identity label alone cannot authenticate these artifacts.

**Exact change requested:** Add an integration acceptance check over the complete pinned manifest. Require, for each eligible call/vector, final proof/key identity, stock and ledger verification results, included Preview transaction/block, readback and authenticated native comparison. Distinguish simulated comparison from a live settlement pass. Reject empty, omitted, duplicate and extra vectors. Delete each required evidence item in a negative test and require completion to fail. Reference STL-002/004/005 rather than copying their behavior.

### R-18 — HIGH — EUF-013 confuses one universal key set with one program key root

**File and ID:** FLOW:166, EUF-013 scenario. Related STATEMENT:69, ERS-006.

**Evidence:** The scenario requires the whole suite table and receipts to name one `key_root`. Per-program keys sit under a program's `key_root` inside its `prog_d`, `MYC/spec/specs/keys/spec.md:9` and :49. R_SL preprocessing is per program, `RSL/src/rsl/mod.rs:13`. A universal node/L/L_Fr/wrap key set does not require all program app-key menus to have the same root. ERS-015 correctly retains that distinction.

**Exact change requested:** Require one universal key-set identity across the suite. Record the admitted `key_root` and `prog_d` per program/circuit. Compare each proof with its own program admission record. Preserve a shared root only where the planner explicitly constructs one. Do not use equality of program roots as the universal-key acceptance predicate.

### R-19 — MEDIUM — New IDs duplicate existing normative obligations

**File and IDs:** The pairs below. Proposal:21 and D-026 require references instead of restatement.

**Evidence:** The following pairs have two normative homes for the same behavior.

**Exact change requested:** Convert the duplicate behavior to a reference block under the existing ID. Retain only new integration behavior as a new EARS sentence. If meaning changes, identify the exact superseded clause and its decision.

| New requirement | Existing normative home | Required action |
|---|---|---|
| ECC-004 at COMPILER:43 | MYC-FE-010, `MYC/spec/specs/frontend/spec.md:75`, plus MYC-FE-009 at :68 | Reference determinism and witness independence. Add only the missing serialization check. |
| ECC-006 at COMPILER:69 | MYC-BLT-008, `MYC/spec/specs/builtins/spec.md:63` | Reference kit refusal. Keep any new stable external error-code rule separately. |
| ECC-008 at COMPILER:95 | MYC-FE-017, cited by the requirement itself at :105 | Reference the existing unsupported-curve obligation. |
| ERS-004 at STATEMENT:43 | MYC-LOW-009, `MYC/spec/specs/lowering/spec.md:68`, and MYC-TRN-014 | Reference native commitment semantics. |
| ERS-005 at STATEMENT:56 | CF-003, FINAL frontend spec:18, and MYC-LOW-010 | Reference word equality. Keep cross-executor integration checks separately. |
| ERS-006 at STATEMENT:69 | MYC-CLM-023, `MYC/spec/specs/claims/spec.md:168`, and MYC-KEY-003 | Reference the identity formula. Keep the missing R_SL proof binding as the new obligation. |
| EKT-005 at KITS:56 | MYC-BLT-001, `MYC/spec/specs/builtins/spec.md:12` | Amend or refine that existing sponge definition, with explicit supersession if required. |
| EKT-008 at KITS:95 | RSL-006, COMPLETE RSL spec:59, and RSL-005:48 | Reference no-new-AIR and bucket behavior. Keep any new complete interface check separately. |
| EUF-003 at FLOW:30 and EUF-004 at :43 | ZK-001/002 and GP-ZK, FINAL general-purpose spec:4 | Reference the privacy obligations. New transport enforcement can have a separate integration ID. |
| EUF-009 at FLOW:108 | MYC-KEY-002, `MYC/spec/specs/keys/spec.md:21`, and CPR-003 | Reference universal wrap-key identity. Keep a two-deployment integration scenario. |

This is one consistency defect with multiple instances. A “References” line after a new SHALL does not remove the second normative home.

### R-20 — MEDIUM — Decision authority and confirmation exceed their source scope

**File and IDs:** `CHANGE/architecture/decisions/D-017.md:24`, D-017. `CHANGE/architecture/decisions/D-020.md:34`, D-020. `CHANGE/architecture/decisions/D-027.md:35`, D-027.

**Evidence:** WIKI:4107 confirms a listed rekey package. D-017 uses that to declare the exact current code table authoritative and all other lists superseded. Yet EKT-001:14 says secp has no rule in that table and its `R_SL_SECP` name is only a counter. `RSL/src/rsl/compile.rs:818` implements that typed path outside the 16-name table. Owner confirmation of the rekey does not by itself specify an exact later rule taxonomy. D-020's confirmation omits the new format and parser obligations ERS-013/014. D-027's stated goal is all three Jubjub suite calls, but its named outcome tests omit both j02 calls and ECC-014. A passing s01 test cannot close wide input admission.

**Exact change requested:** Separate confirmed rekey scope from the proposed exact machine/rule encoding in D-017. Name the concrete source text and time for each authority claim. Resolve secp's rule identity before calling the table exhaustive. Add ERS-013/014 and their named checks to D-020. Add ECC-014 and the two j02 native/parity/full-chain cases to D-027's confirmation. Keep authority status separate from observed test status.

### R-21 — MEDIUM — D-011 uses transactions where the contract requires atomic contract calls

**File and ID:** `CHANGE/architecture/decisions/D-011.md:42`, D-011.

**Evidence:** The consequence says “At most 2 transactions per call.” ANC-004 at FINAL `specs/sherlock-anchor-chunks/spec.md:25` requires k contract calls in one intent of one transaction. ANC-005/006 preserve atomic failure behavior. Two ledger transactions would not preserve that boundary.

**Exact change requested:** Replace the consequence with “At most two chunk contract calls in one intent of one transaction.” Name ANC-004/008 in Confirmation and add the chunk atomicity check handle. Keep the chunk cap independent of transaction count.

### R-22 — LOW — Status and source citations overstate the cited evidence

**File and IDs:** SUITE:157, ECS-012. STATEMENT:136 and :144, ERS-011. COMPILER:14, ECC-001. KITS:6, EKT-001 Verification. STATEMENT:14, ERS-001 Verification/status.

**Evidence:** T23:494 already prints “mismatch … of … compared after a proof.” Its regression/structured count check can remain open, but the displayed denominator is built. An odin-only run cannot measure ERS-011's two-box equality. Full JSON result hashes include timings at `RSL/src/rsl/mod.rs:76`, so whole-file sha256 equality is the wrong cross-box test. `MYC/factory/REPORT.md:8` counts 106 circuits including a pure export, not 106 compiled proof circuits. EKT-001 calls a rule-table digest `rules_d`, while current per-execution `rules_d` is defined over enabled slot rule lanes in CORE-006 and SRC-001. ERS-001 cites five checked-in fixtures as input to two tests, but `RSL/tests/rsl.rs:18` and `RSL/tests/rsl_exec.rs:56` load `$RSL_CALLS` or a cache directory. The five checked-in files exist. The cited tests do not select that directory by default or validate the schema's full field set.

**Exact change requested:** Label ECS-012 “display built, regression/structured field open.” Label cross-box parity open until both boxes run, then compare canonical comm/W only. Count 105 proof circuits plus one pure export. Distinguish a machine rule-table digest from the per-trace rules digest and name each type. Cite actual fixture locations and avoid claiming fixture-schema validation from a cache-based execution test.

### R-23 — MEDIUM — Legacy decisions and cancelled references are not fully mapped

**File and IDs:** `CHANGE/architecture/decisions/INDEX.md:70` and :72. STATEMENT:92 and :105, FLOW:79 and :118. D-023, D-070 and D-071.

**Evidence:** The index says older decisions are in record legacy columns. The A record set does not map several language-visible owner choices: placement V/V+ (OD-5), Compact field emulation (OD-14), the fixed call-vector layout (OD-7/15), hybrid native entry checks (OD-8/16), and mandatory log reservation (OD-17). Their sources are DECISIONS:19, :21, :22 and :28 through :31. D-023 was cancelled during review, BUS `LOG.md:17` and INDEX:55, but statement/flow requirements still cite it. D-071 in `BUS/decisions-C.md:41` retains the rejected input alternatives. That content must survive the merge. D-025's replacement set also needs the retention map in R-05.

**Exact change requested:** Add an explicit legacy-ID-to-record map for these choices, with existing EARS IDs and named confirmation checks. Extend existing records where that preserves one real decision per record. Do not invent rejected alternatives where the source does not record them. Replace D-023 references with D-070 for deployment granularity and D-071 for input/outcome/refusal policy. Preserve the map's public-input-only and VAL_NONE rejected options. Mark decisions outside the run-form prototype as retained obligations with their scope, rather than silently omitting them.

The completed COVERAGE matrix retains the sealed-form requirements at :52 and :165. That preserves their requirement inventory. It does not supply the missing decision-to-record map claimed by INDEX:72.

### R-24 — MEDIUM — The coverage matrix has stale statuses and incompatible completion scopes

**File and IDs:** `CHANGE/architecture/COVERAGE.md:54`, :55, :112, :129 and :167. Related ECC-002, EUI-010, EUF-011/013 and D-085.

**Evidence:** COVERAGE says `lake build --wfail` is open and factory deliverables are uncommitted. `MYC/factory/REPORT.md:14` records the warning-free build in oracle-equal, and :115 states the deliverables are committed. This review used no git command, so repository status is NOT independently established. The report no longer supports the matrix's status. COVERAGE:129 repeats old T16(c)-(e) statuses. The current LOOP:37 begins with the newer three-box completion result and leaves T16(b) open. COVERAGE:112 says 115 rules remain while :68 counts five subset plus two value rules. That count requires a rule-identity union before stating a remainder. Finally, COVERAGE:167 defers `sherlock compile` until after T15, while ECC-012 requires that command and EUF-011/013 require the public joined flow at completion.

**Exact change requested:** Date each status and distinguish reported evidence from independently checked repository state. Update the latest T16 result while preserving the open subtests. List the seven built rule IDs and compute the uncovered set explicitly. Resolve whether the product CLI is required before T15 or is superseded by the released-tool runbook. Record that scope change in the affected requirement and decision, rather than only in the coverage gap column.

## Finding table

| Finding | Severity | One-line fix |
|---|---|---|
| R-01 | HIGH | Treat mixed-hash refusal as an open implementation gap and preserve GP-1. |
| R-02 | HIGH | Authenticate the outcome commitment through the retained statement schema instead of placing raw W in FP-1. |
| R-03 | HIGH | Separate tape/value/input digests and outcome payload/tagged digest in statements and tests. |
| R-04 | HIGH | Make the raw R_SL statement private to owned hosts and test permitted disclosure and transport. |
| R-05 | HIGH | Map retained interpreter behavior before retiring any UIREQ requirement. |
| R-06 | HIGH | Correct entry-x to Rejected and require outcome- and rule-indexed proof evidence. |
| R-07 | HIGH | Limit the no-proof rejection policy to Compact/T23 calls and preserve generic rejected-run proofs. |
| R-08 | MEDIUM | Check image and ABI bytes at the Mycroft compiler boundary. |
| R-09 | MEDIUM | Control all prog_d inputs in rekey tests and distinguish program names from images. |
| R-10 | MEDIUM | Build the actual Mycroft-to-kit, final leaf-key and exact-ALU checks. |
| R-11 | MEDIUM | Reject invalid relations rather than every trace that differs from a host trace. |
| R-12 | MEDIUM | Keep formal premises and finite conformance scope explicit. |
| R-13 | MEDIUM | Replace the predecessor-only flow with named stage dependencies and retained private state. |
| R-14 | MEDIUM | Separate pre-submit failure from post-submit reconciliation. |
| R-15 | MEDIUM | Require successful compilation only after admission succeeds. |
| R-16 | MEDIUM | Check no-proof admission failure and normalize the public error at its real boundary. |
| R-17 | HIGH | Require complete per-call proof, verification, receipt and comparison evidence for completion. |
| R-18 | HIGH | Use one universal key-set ID with per-program key_root records. |
| R-19 | MEDIUM | Replace duplicated SHALL statements with references or explicit amendments. |
| R-20 | MEDIUM | Scope authority precisely and bind format, secp-rule and wide-Jubjub decisions to their checks. |
| R-21 | MEDIUM | State two chunk contract calls in one transaction, with atomicity checks. |
| R-22 | LOW | Correct denominator, parity, circuit-count, digest and fixture evidence claims. |
| R-23 | MEDIUM | Map legacy decisions and replace cancelled D-023 references. |
| R-24 | MEDIUM | Refresh coverage evidence and reconcile rule counts and pre-T15 product-flow scope. |
