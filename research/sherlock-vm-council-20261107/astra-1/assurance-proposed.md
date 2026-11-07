# Proposed assurance clauses for the consensus draft

Author: astra-1. These are proposed requirements and acceptance checks. They are not implemented claims. Source base: `8fd34ed794eda7186c32495f715609ec54a9a297`.

## The assurance argument

For a supported profile, let `R(P, s, s', e)` denote the specified step relation. Let `Enc` relate complete semantic states to circuit carriers satisfying the declared internal invariant. Let `C` denote the accepted constraint relation, including the public statement. External boundaries require the declared canonical decoder or projection. Internal microsteps may retain unreachable rows when `StepInv` permits them. [openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md:A4]

The required local argument has these parts:

1. Admission and genesis establish the profile's invariant and an `Enc` relation to its initial carriers.
2. For every allowed pre-state and every witness, `C` implies a semantic successor under `R`, invariant preservation and the exact public statement. Hidden helper steps use the declared profile projection.
3. Every valid supported step within declared bounds has a satisfying witness. Refusals identify unsupported or insufficient-capacity inputs separately from semantic rejection.
4. The composed leaf checks fragment premises on the same carriers. Its dispatch, canonicality and transition fragments cannot refer to disconnected copies.
5. Recursive composition proves the ordered sequence under authorized keys. It preserves the program/profile identity, continuous state and bounded work.
6. Final acceptance authenticates genesis and discharges every carried cryptographic obligation.
7. The target adapter binds that accepted result to the selected network, predecessor state and actual effects. Unique consumption remains a separate ledger property.

These are obligations, not asserted theorems. Existing requirements already separate code identity, arbitrary-witness semantics, final discharge and present entitlement. [openspec/changes/sherlock-universal-interpreter-design/security-requirements.md:UISEC-001,006,009–012; docs/MORIARTY-BACKEND-REQUIREMENTS.md]

A formal proof must name the exact definitions and premises it covers. Differential tests provide finite correspondence evidence on their declared domain. Positive witnesses and invalid mutations support the argument but do not prove its universal quantifiers. [proposal following UISEC-012]

## EARS and executable acceptance

| ID | Form and requirement | Acceptance check |
| --- | --- | --- |
| ASR-001 | Ubiquitous: The assurance reporter SHALL record evidence class, dependency fingerprints, semantic profile, tested bounds and explicit remaining assumptions. | Reject missing fields. Preserve historical artifacts. Reject a report that labels a finite model sample as a theorem or ledger acceptance. |
| ASR-002 | Event-driven: When a relevant dependency changes, the qualification checker SHALL invalidate dependent evidence until revalidated. | Change each declared dependency independently. Check stale classification. An unrelated file change may retain evidence only through a complete declared dependency closure. |
| ASR-003 | Unwanted behavior: If mutation execution fails before constraint verification, the checker SHALL report that failure without constraint-rejection credit. | Inject baseline failure, decoder refusal, synthesis error, panic and timeout. Require distinct outcomes. Require zero such failures for a campaign claiming complete rejection of its selected cases. |
| ASR-004 | Event-driven: When a witness-mutation campaign closes a constraint obligation, the reporter SHALL identify the reached verifier layer and exact mutant. | Require a valid baseline and a deliberately missing-constraint control. Preserve seed, case count, sampled/exhaustive status, mutation site and allowlist identity. |
| ASR-005 | Event-driven: When a trace comparison closes a semantic obligation, the comparator SHALL check complete ordered observations through a declared projection. | Mutate an intermediate preserved field while retaining the final result. Mutate both producer traces identically. Require the independent oracle to distinguish both faults. |
| ASR-006 | State-driven: While recursive evidence has an undischarged obligation, the verifier SHALL refuse final-valid classification. | Alter the carried accumulator, skip the decider, substitute a relation/key and supply a wrong predecessor instance. Require rejection by the intended cryptographic boundary. |
| ASR-007 | Event-driven: When a target result qualifies a candidate, the checker SHALL match its call and effects to that candidate's artifact manifest. | Substitute a valid receipt from another statement, contract, chain instance or runtime. Reject each mismatch. Do not select the candidate through file modification time. |
| ASR-008 | Optional: Where a circuit optimization is enabled, qualification SHALL retain the same specified semantics and acceptance obligations. | Compare declared traces against the baseline. Re-run affected adversarial cases and native/target compatibility checks. Report measured resource changes separately. |
| ASR-009 | Event-driven: When a release claims a formal property, its report SHALL identify a checkable proof artifact and its exact theorem, assumptions and source closure. | Remove or change the artifact or a premise. Reject the formal claim. Do not reject a correctly labeled experimental result merely because it has no theorem. |
| ASR-010 | Event-driven: When a release admits a native rule, its inventory SHALL identify evidence for initialization, invariant preservation and every fragment composition premise. | Alter a lane assumed canonical by one fragment. Check that the composed leaf rejects it or that authenticated reachability excludes it through a checked invariant argument. |
| ASR-011 | Event-driven: When a profile, compiler, proof system, key or ledger adapter changes, qualification SHALL verify the affected cross-layer interfaces again. | Use an incompatible leaf schema, proof tag, key tag and target era. Require refusal before submission. |
| ASR-012 | Ubiquitous: A release report SHALL distinguish implemented behavior, executed evidence, proved properties and open obligations. | A report generated from a task plan must leave unexecuted and unproved claims open. A lower-level pass cannot close a higher-level gate. |

These IDs extend the six suggested IDs in `position.md`: the table here is proposed draft text, not a revision to the frozen independent position. The chair should choose one final ID mapping and update every traceability row consistently.

## Evidence gaps that motivate these clauses

- The status view counts retained pass flags and merges old entries under a newly written HEAD. It does not revalidate source closures. [models/doctorwatson/factory/status.py:34-47,77-112]
- The mutation harness maps synthesis errors to `Rejected`, counts panics as rejections and lacks a zero-panic pass assertion. [crates/mori-backend/src/qa_mutation.rs:458-486,640-650; crates/mori-backend/tests/soundness_mutation.rs:227-245]
- The pairwise comparator has a useful sensitivity test: equal corrupted traces match each other but fail the independent oracle. [models/sherlock/qa/test_paired_projection.py:95-102]
- The current model package compares retained F03 results while reporting no new producer execution or refinement. [models/sherlock/qa/test_model_package.py:24-42]
- Dispatch explicitly leaves IMAGE/HELPER canonicality and recursion composition to paired fragments and callers. [crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:34-38]
- The IVC test uses `ToyLeaf`. Its binding and step count are unconstrained there. That test cannot close real C6 semantics. [crates/mori-recursion/tests/sherlock_ivc.rs:1-7,68-72]
- The node adds height and steps as field values and scales only the previous-proof contribution at genesis before folding carried and leaf accumulators. The no-wrap and genesis-carried-accumulator arguments must be explicit. These observations do not establish a forgery. [crates/mori-recursion/src/ivc.rs:308-368]
- A8 identifies any call at its address. A10 chooses the newest statement file. A single explicit candidate manifest would remove this ambiguity. [scripts/qa/check-a8.sh:7-42; scripts/qa/check-a10.sh:7-24]

## Milestone exit rule

A demonstration may proceed with stated unproved obligations when it claims only its observed scope. A named supported release must close its declared semantic, constraint, recursive and target obligations. Full Sherlock acceptance must retain the original financial and language obligations or explicitly amend them through an owner decision. A council vote alone is neither runtime evidence nor a proof artifact. [proposal preserving AGENTS.md, UISEC-012 and docs/MORIARTY-PRODUCT-CONTRACT.md]
