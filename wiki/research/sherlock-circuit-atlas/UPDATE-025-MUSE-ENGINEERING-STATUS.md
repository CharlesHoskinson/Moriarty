---
id: moriarty.research.sherlock.atlas.update025.status
title: UPDATE025 literal Muse engineering snapshot
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T22:13:28.581751+00:00
tags: [moriarty, sherlock, circuit-concept]
sources:
  - src-86b216d3b4eb930a2ae7
---

# UPDATE-025 — internal engineering status

**Date:** 2026-10-20. Internal only, kept apart from the finished visitor copy. The website is UNPUBLISHED and goes live only when the full Sherlock goal is finished and accepted. No site-live, circuit, native, proof, target or EARS qualification is claimed. The finished-future visitor direction does not license any current-result claim below. The layers below are separate engineering facts, not a progress score and not a final circuit validation.

## ScalarConstruct source05: complete source assembly, all tests UNRUN

ScalarConstruct05 is a complete five-file source assembly: the handwritten three-mode emitter (`c6_scalar_construct_step.rs`), the real consumer (`c6_scalar_construct_consumer.rs`), the parent `mod.rs`, and two test-source files (`c6_scalar_construct_step_tests.rs` and `c6_scalar_construct_consumer_raw_tests.rs`). Test source contains 26 test functions in total: 12 positive and 14 raw. All 26 are UNCOMPILED and UNRUN. Test source is delivered author source, not an independent blind oracle and not a native result.

The sole production change against frozen C4 recorded by the source-owner assembly is two comment-only disabled-receipt doc corrections, in the emitter doc block and the consumer module doc; non-comment production bytes are unchanged per the owner inverse (9/9 inverse checks in the owner report; documentation lane ran no Rust, Cargo, SDK, IR or native check). No instruction, gate or API change is inferred from those doc hunks. Recorded API questions in the assembly (accessor versus field style across the two test files, the raw stated count of 11 against a literal 14 `#[test]` functions, raw imports of non-pub consumer items) were recorded, not silently fixed; passive name presence is textual only, not type, borrow or visibility adjudication.

At the assembly-report cut the status was `COMPLETE_SOURCE_ASSEMBLY_UNCOMPILED_UNRUN_NOT_REVIEWED`, and the earlier C4 Reviews A and B were NOT_APPROVED solely on the disabled-case doc wording (MF1), with bounded source/API approval otherwise and neither being a compiler pass. By later Root attribution supplied with this assignment, ScalarConstruct source05 has since received a bounded-only source review approval and exists as draft Moriarty PR 362 at `ecd902c0b51352503c13e8a0279907e975f2d172`, with the 26 tests still UNRUN. That bounded source opinion and draft-PR state are kept separate from compilation, SDK execution, native proof and target acceptance, none of which is claimed.

The receipt distinction depicted in the visitor copy is the corrected source semantics: disabled (`enable == 0`) remains a constrained identity post-state (full pre/post equality for image and helper, plus the already-unconditional META/PROG/CTX/DATA equality); the raw `is_helper_work_max` receipt may export 1 when pre helper work equals max helper work; the disposition lane is not constrained to P3 in the disabled case, no charged step occurs, and the receipt does not alter the identity constraint. The receipt is diagnostic, not the post-state.

## Linked producer: draft source, bounded source opinion, native UNRUN

The linked producer stands as draft Moriarty PR 361 (head `03496bffd944f4b46c9d2997863f996f00c85017`), an exact-4 source set. Its test source totals 14 (11 Unix normal plus 3 ignored native), all UNCOMPILED and UNRUN in the retained receipt; exactly one normal test calls SDK emit / ir.k source. A later fresh Muse Wonder FULL HIGH lane-C opinion on producer revision02 returned APPROVE_SOURCE_WITH_LIMITS, covering first-compile source eligibility only, with same-model-family independence disclosed; all 14 normal/native tests remain UNRUN/uncompiled under that opinion. No native, proof or target acceptance follows from the draft source, the committed source hashes, or that bounded opinion. Original-token, attempt, fingerprint and result binding and any later proof or target observation remain separate layers.

## Cohort05: exact source set verified, nothing dispatched or executed

Cohort05 stands at `ROOT_EXACT_COHORT_SOURCE_VERIFICATION_PASS` (2026-10-20T21:26:07.576067+00:00): exactly 1735 files, 87569159 bytes, exact set and read-only modes verified against its manifest. Dispatch is not approved, native has not been executed, and stopped C13 accounting is unchanged. An exact, hash-verified source cohort is source identity for a set; it is not authentication of execution, not SDK synthesis and not a target result.

## Quint model: directed named tests pass in scope, sampled run incomplete

The Quint workflow model records 64 corrected directed named tests (34 original authored + 24 extension + 5 regression + 1 directed walk) across distinct command scopes; these are not 64 independent formal proofs. The original whole command exited 1 because extra imported declarations failed, and the 34 authored run names verified by the selector report are not relabelled as an all-pass. The directed baseline exited 0, with six requested witnesses reached in one trace in the retained stdout (the 47-step figure is Root task attribution; the max-steps-50 flag alone does not establish length 47). The 10000-sample / max-steps-60 run exited 124 after about 30.03 seconds with both streams empty: incomplete, with no pass, completion or counterexample inferred. The nested-forall history quantifiers at source lines 568-591 exist in source; no timing profile establishes them as the cause of that timeout. Quint is a pure workflow model, not an interpreter proof, and its modelled token/attempt/fingerprint/result binding does not authenticate actual producer execution.

## Lean value equality: 05 actual FAIL, 06 separate actual strict PASS for a bounded claim

Lean05 is an actual isolated value-equality FAIL (exit 1, accepted products 0): parser diagnostics `unexpected mutual` at 32:72 and 148:70, plus the strict `unused simp argument eq_self_iff_true` warning at 89:12. Its prospective source/API and single-operation eligibility approval did not establish parsing or elaboration, and explicitly left those unobserved. Lean06 is a separate candidate (doc comments moved after `mutual` and directly before `valueEq` / `valueSize`, exactly the one unused simp argument removed) with a separate fresh source/API/prospective-operation approval followed by an actual strict single-leaf kernel PASS (exit 0, both streams empty). That PASS covers general structural equality for all represented logical Values plus seven examples only. It is not B55 whole-Host encode, native serializer, SDK, refinement or EARS acceptance, and the original traversal was not accepted. By later Root attribution supplied with this assignment, the Lean06 Clean result is now merged in project-fenrir PR 65 at `a04c305ad9bb0f83b362f0c14449d7cf3ed39089`; the merge extends the claim no further than the bounded PASS above.

## Resource disposition and live reviews: no adoption, no permission

On compile-resource design02, vote A returned REVISE and vote B returned AGREE; the design is not adopted, design03 was reported under preparation, and no resource permission, cap or contract change follows. Current reviews described as Root LIVE elsewhere are not returned approvals. Only the named bounded opinions above are relied on, each within its stated limits.

## Open scope, unchanged

The full 39-selector scope, witness/constraint/export closure, grouping, residual, finance, persistence, recursion and stagenet, SI08, SI17, SI21, SI22 and StepInv as wholes, program membership, P1 provenance and source authentication, native issuance and accounting, refinement and EARS remain open as recorded in the source05 assembly. A public fixture hash, a coherent owner record, exact cohort hashes, finite ordinary model observations, pure kernel results and bounded source/API opinions remain separate layers and are not merged into one qualification. UPDATE-024's five authored fields and its dated status remain unchanged and are not rewritten by this note.