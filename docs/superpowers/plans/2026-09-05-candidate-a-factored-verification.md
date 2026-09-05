# Candidate A Factored Verification Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Test whether explicit argument sharing and joint raw/effect computation make a semantics-preserving Candidate A authority pilot tractable, while retaining every full-A5 obligation.

**Architecture:** Add isolated verification modules under `specs/quint/s02/factored_verification/`. Keep frozen common/A modules and both accepted lifecycle sources unchanged. A joint evaluator shares one actual pre-reduction and full transaction result; mechanically derived variants preserve all other declarations, guards, histories and test bodies.

**Tech Stack:** Quint 0.32.0, pinned Rust evaluator, offline Apalache 0.56.1/build70cdaf4, Python standard library for derivation and immutable command receipts.

Classification: recommendation/specification, **specified-only**. Quint code is unimplemented and not typechecked. Read-only plan validation assembled a scratch closure and ran `quint parse` on32 modules; all returned exit0. Python blocks passed AST parsing; derivation pins and exact boundary-body roundtrip passed in memory. These are syntax/derivation checks, not behavioral or model-checking evidence. Root review and task dispatch are prerequisites to implementation. This plan does not authorize commits, unrelated edits, additional heap-only attempts, or an A5 completion claim.

## Global constraints

- Authoritative scope: approved completion XML phase A5; adopted Candidate A authority design and S02 model-comparison design; verification-command addendum at a314549. Preserve E00.2 and source anchor `d14cfea98a1c9213e5ef5f12c1a088f4e966083d`.
- First obtain root's accepted A2/A3 closure hashes. Pin all original source bytes before derivation. Source movement stops the task; do not silently regenerate against a different baseline.
- Both signing profiles, every mandatory scenario and adverse control remain required. A pilot is an explicitly partial verification unit, not a substitute for those workloads.
- Complete maps: sixteen nodes, six Core accounts, five optional choices; exact integers and complete ledger, registry, parent, signing, attempt and evidence records. No hash-only state, fabricated expected table, trusted validity cache, reduced effect list, history deletion or compressed case.
- Preparation, signing, proposal, verification, commit and rejection remain separate observable actions. Cancellation remains identity/no Core projection/empty effects. Supplied timeout input retains complete rollback, diagnostics and original minimumTime. Refusal completion is not financial completion.
- Public extraction must compare the entire supplied result to actual evaluation, including rejected results. Diagnostic precedence must stay unchanged for malformed state/program/input even when the supplied result is forged.
- EvidenceValid remains a symbolic premise. PreparedAttempt.actor is metadata, not authenticated sender; use wrong signed.signer or actual Core chooser/depositor for authenticated-actor controls.
- No fairness assumptions are added. Genuine route terminals disable step; no blanket stutter. Disable checker deadlock reporting only with a separately checked nonterminal-enabled predicate.
- First failed expensive experiment ends that experiment series. Archive the failure and require a reviewed changed hypothesis before another expensive run. OOM/timeout/translation failure is incomplete verification, not an architecture counterexample.
- Only new files listed here may be implemented. Parent owns admission, evidence integration, main/wiki/DB edits and commits. This draft changes only this plan and its ignored report.

## Evidence and changed hypothesis

Repository observation: prior failure commit `93fce82250cf6f0f68114b4d2b5c373cc9b7390d` records a 93,413,860-byte flattened input and terminal InlinePass heap failures at 4/8 GiB before state exploration. The original input remained at a temporary path, not in that original archive; later digest comparison confirms the retained bytes still match. Do not rewrite that provenance.

Experiment observation: `fcd28f25d2faa68fe6ba8fa802096334ae21a745`, `evidence/s02-candidate-a-completion/a5/nullary-control/README.md`, records a tiny non-Candidate-A nullary-binding control: agreement passed depth2; deliberately false invariant yielded a state1 counterexample/exit12. It is neither Candidate A verification nor a performance comparison.

Recommendation/hypothesis H1: replace repeated actual raw evaluation and repeated pre-reduction inside the adapter with one pure joint computation; bind complex boundary arguments to local nullary values before parameter substitution. The old log already reports removal of irrelevant operators, so dead-code pruning is NOT the changed hypothesis. Existing local vals already survive Quint compilation; H1 can fail if backend inlining still duplicates them. Generated sizes and terminal checker behavior, not source line count, decide the pilot.

### Preservation obligations before performance claims

For every representable program/state/input/time, prove by branch inspection that `computeJointF(...).evaluation == computeTransaction(...)`. For every computed raw result, prove joint extraction equals original public extraction on that raw; for every supplied result, public factored extraction equals original extraction, preserving diagnostic-before-mismatch precedence. The non-recomputed accepted pre-reduction is equal because the identical pure reducer has identical program and timed original state; this requires no reachability premise. Original accepted computation cannot have a failing pre-reduction.

For every representable authority request/operation/plan/display, adaptation equality follows from that lemma and exact cancellation/invalid-operation/time0 branches. No policy validity or honest-input precondition is allowed in this statement. Boundary clones preserve each guard and rejection reason in original evaluation order; the extra bindings are identities. Undefined original operations outside guarded domains are not newly claimed total.

State relation R is literal equality of complete observable state, scenario/profile/cursor, full history and all instrumentation fields between original and factored harnesses. Initializers correspond exactly; each enabled original action has the same enabled factored action and equal next state, and conversely. Auxiliary joint values are pure, not new transitions or mutable caches. There is no stuttering quotient. Corpus agreement supports the implementation but is not a universal proof of these lemmas. Root must review the branch argument and exact generated diff.

## Ownership and review units

Pre-Task1 bootstrap (sole snapshot owner; A4 producer only consumes the admitted store) creates `scripts/derive_s02_candidate_a_factored.py` and `scripts/run_s02_candidate_a_factoring_pilot.py` using their complete blocks below. Do not derive models or launch pilot commands during bootstrap. Task1 creates `candidate_a_joint_f.qnt` and `candidate_a_joint_f_test.qnt` in the isolated directory.
Task2 uses the bootstrapped derivation script and creates `candidate_a_factoring_equivalence_test.qnt`, and mechanically derived `*_f.qnt` files plus `derivation.json` in that directory. The derivation covers all 26 current Candidate A modules except unchanged types/programs; its exact manifest fixes the file and run inventory.
Task3 creates two pilot modules and a pilot test module, and uses the bootstrapped recorder. Receipts go in a fresh ignored `.superpowers/sdd/a5-factoring-<stage>/`; no existing directory is overwritten.
Each task ends at an independent root review gate. Do not execute the next unit until admitted.

### Task 1: Joint evaluator and public untrusted-result boundary

**Interfaces:** Original `ATransactionEvaluation`, `AEffectExtraction`, `AAuthorityAdaptation` remain the public result types. New `JointF`, `computeJointF`, `computeTransactionF`, `extractCommittedEffectsF`, `adaptAuthorityF` have the full signatures below.

- [ ] Write the complete test file below first.
- [ ] Add the complete kernel below with exactly one initial RED difference: replace `if (supplied != actual) EffectResultMismatchA else joint.extraction` with `joint.extraction` only in `extractCommittedEffectsF`.
- [ ] Capture immutable full transitive source/tool closure, recursive typecheck, then `quint test specs/quint/s02/factored_verification/candidate_a_joint_f_test.qnt --backend=rust --seed=42 --match suppliedResultRejectedTest`. Require terminal typecheck exit0 and an actual assertion failure. A parse error or nondiagnostic timeout is not RED. Keep every command to terminal.
- [ ] Freeze that RED closure and restore the exact comparison line. Run recursive typecheck and all six tests using `--match '.*Test'`; require six passing, no exclusions. Capture full original/factored source bytes before and after, raw stdout/stderr, CLI/backend hashes, command/cwd/seed, elapsed time and terminal exit. Use a fresh stage directory, never rewrite RED.
- [ ] Root reviews exact branch/diagnostic/effect-order preservation and the compiling RED before Task2.

```quint
module candidate_a_joint_f_test {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import execution.* from "../execution"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core.* from "../candidate_a_core"
  import candidate_a_projection.* from "../candidate_a_projection"
  import candidate_a_cases as C from "../candidate_a_cases"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"
  import candidate_a_authority_swap_fixtures as S from "../candidate_a_authority_swap_fixtures"
  import candidate_a_joint_f.* from "./candidate_a_joint_f"

  pure def equalityF(r: C::DiagnosticRequest): bool = {
    val actual = computeTransaction(r.program, r.before, r.input, r.now)
    val changed = computeJointF(r.program, r.before, r.input, r.now)
    actual == changed.evaluation and match actual {
      | TransactionComputedA(raw) => changed.extraction == extractCommittedEffects(r.program, r.before, r.input, r.now, raw)
          and extractCommittedEffectsF(r.program, r.before, r.input, r.now, raw) == changed.extraction
      | OutsideModelDomainA => changed.extraction == EffectOutsideDomainA
      | TransactionReductionBoundFailureA => changed.extraction == EffectReductionBoundFailureA
      | TransactionInputPreconditionFailureA => changed.extraction == EffectInputPreconditionFailureA
    }
  }
  run completeCorpusTest = assert(C::CASE_REQUESTS.indices().forall(i => equalityF(C::CASE_REQUESTS.nth(i).request)))
  run suppliedResultRejectedTest = assert(C::CASE_REQUESTS.indices().forall(i => {
    val r = C::CASE_REQUESTS.nth(i).request
    match computeTransaction(r.program, r.before, r.input, r.now) {
      | TransactionComputedA(raw) => Set(
          {...raw, accepted: not(raw.accepted)},
          {...raw, reductions: raw.reductions + 1},
          {...raw, state: {...raw.state, minimumTime: if (raw.state.minimumTime == Time100) Time101 else Time100}},
          {...raw, error: CoreErrorCode("forged")},
          {...raw, payments: raw.payments.append({source: aliceA, recipient: Mallory, asset: TokenA, quantity: 1})}
        ).forall(bad => extractCommittedEffectsF(r.program, r.before, r.input, r.now, bad) == EffectResultMismatchA)
      | _ => false
    }
  }))
  run orderedEffectsLiteralTest = {
    val r: C::DiagnosticRequest = {program: C::preDepositFixture(1), before: C::funded(N3, 1),
      input: C::depositInput(5), now: Time2}
    val result = computeJointF(r.program, r.before, r.input, r.now)
    assert(result.extraction == EffectsExtractedA(List(
      {source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 1},
      {source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 5},
      {source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 5})))
  }
  run malformedBeforeMismatchTest = {
    val r: C::DiagnosticRequest = {program: S::programS,
      before: {...S::beforeS(0).state, accounts: Map(), choices: Map()}, input: C::depositInput(10), now: Time1}
    val fake: ATransactionResult = {accepted: true, state: S::beforeS(1).state, error: NoCoreError,
      payments: List(), warnings: List(), reductions: 0}
    assert(equalityF(r) and extractCommittedEffectsF(r.program, r.before, r.input, r.now, fake) == EffectOutsideDomainA)
  }
  run timeoutRollbackLiteralTest = assert(Set(Time100, Time101).forall(t => {
    val before = S::beforeS(2)
    val r: AAuthorityRequest = {before: before, input: C::choiceInput(Bob, 1), now: t}
    val joint = computeJointF(before.program, before.state, r.input, t)
    joint == {evaluation: TransactionComputedA({accepted: false, state: before.state,
      error: CoreErrorCode("contract_closed"), payments: List(), warnings: List(), reductions: 0}),
      extraction: EffectsExtractedA(List())}
  }))
  run authorityCorpusTest = assert(S::SCENARIOS_S.forall(s =>
    Set(FundingOneAttempt, FundingTwoAttempt, DispositionAttempt).forall(id => {
      val op = S::plannedS(id, s)
      match op.artifactAndCall {
        | AgreementCallA(r) => adaptAuthorityF(r, op.operation, S::planS(id,s), PublicDisplay)
            == adaptAuthorityA(r, op.operation, S::planS(id,s), PublicDisplay)
        | _ => false
      }
    })))
}
```

Corrected kernel:

```quint
module candidate_a_joint_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core.* from "../candidate_a_core"
  import candidate_a_projection.* from "../candidate_a_projection"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"

  type JointF = {evaluation: ATransactionEvaluation, extraction: AEffectExtraction}
  pure def diagnosticF(e: ATransactionEvaluation): JointF = {
    evaluation: e, extraction: match e {
      | OutsideModelDomainA => EffectOutsideDomainA
      | TransactionReductionBoundFailureA => EffectReductionBoundFailureA
      | TransactionInputPreconditionFailureA => EffectInputPreconditionFailureA
      | TransactionComputedA(_) => EffectsExtractedA(List())
    }}
  pure def rejectF(before: AState, error: CoreError): JointF =
    diagnosticF(rolledBackTransaction(before, error))
  pure def acceptF(input: AInput, pre: AReduction, finished: AReduction): JointF = {
    val actual = acceptedTransaction(finished)
    val prefix = pre.payments.foldl(List(), (items, payment) => items.append(paymentTransfer(payment)))
    val withDeposit = match input {
      | NoAInput => prefix
      | PresentAInput(supplied) => match supplied {
          | ChoiceInputA(_) => prefix
          | DepositInputA(deposit) => prefix.append({source: Wallet(deposit.depositor),
              destination: Escrow(deposit.account), asset: deposit.account.asset, quantity: deposit.quantity})
        }
    }
    val suffix = finished.payments.slice(pre.payments.length(), finished.payments.length())
    {evaluation: actual, extraction: EffectsExtractedA(
      suffix.foldl(withDeposit, (items, payment) => items.append(paymentTransfer(payment))))}
  }
  pure def computeJointF(program: AProgram, before: AState, supplied: AInput, now: ATime): JointF = {
    val prog = program
    val original = before
    val input = supplied
    val clock = now
    if (not(validState(prog, original)) or not(validInput(input))) diagnosticF(OutsideModelDomainA)
    else if (timeValue(clock) < timeValue(original.minimumTime))
      rejectF(original, CoreErrorCode("time_before_state"))
    else {
      val timed = {...original, minimumTime: clock}
      val preliminary = reduceToQuiescence(prog, timed)
      match preliminary {
        | QuiescenceOutsideDomainA => diagnosticF(OutsideModelDomainA)
        | ReductionBoundFailureA => diagnosticF(TransactionReductionBoundFailureA)
        | ReductionCompleteA(pre) => {
            val currentNode = prog.nodes.get(pre.state.continuation)
            match input {
              | NoAInput => match currentNode {
                  | CloseA => if (pre.reductions == 0 and pre.payments == List() and pre.warnings == List())
                      rejectF(original, CoreErrorCode("contract_closed"))
                    else acceptF(input, pre, pre)
                  | WhenA(_) => rejectF(original, CoreErrorCode("input_required"))
                  | _ => acceptF(input, pre, pre)
                }
              | PresentAInput(present) => match currentNode {
                  | WhenA(_) => {
                      val application = applyInput(prog, pre.state, present)
                      match application {
                        | InputOutsideDomainA => diagnosticF(OutsideModelDomainA)
                        | InputPreconditionFailureA => diagnosticF(TransactionInputPreconditionFailureA)
                        | InputAppliedA(applied) => if (applied.error != NoCoreError)
                            rejectF(original, applied.error)
                          else {
                            val following = reduceToQuiescence(prog, applied.state)
                            match following {
                              | QuiescenceOutsideDomainA => diagnosticF(OutsideModelDomainA)
                              | ReductionBoundFailureA => diagnosticF(TransactionReductionBoundFailureA)
                              | ReductionCompleteA(post) => {
                                  val finished = {state: post.state,
                                    payments: post.payments.foldl(pre.payments, (items, payment) => items.append(payment)),
                                    warnings: post.warnings.foldl(pre.warnings, (items, warning) => items.append(warning)),
                                    reductions: pre.reductions + post.reductions}
                                  acceptF(input, pre, finished)
                                }
                            }
                          }
                      }
                    }
                  | CloseA => rejectF(original, CoreErrorCode("contract_closed"))
                  | _ => rejectF(original, CoreErrorCode("no_matching_input"))
                }
            }
          }
      }
    }
  }
  pure def computeTransactionF(program: AProgram, before: AState, input: AInput, now: ATime): ATransactionEvaluation = {
    val joint = computeJointF(program, before, input, now)
    joint.evaluation
  }
  pure def extractCommittedEffectsF(program: AProgram, before: AState, input: AInput,
    now: ATime, supplied: ATransactionResult): AEffectExtraction = {
    val joint = computeJointF(program, before, input, now)
    match joint.evaluation {
      | TransactionComputedA(actual) => if (supplied != actual) EffectResultMismatchA else joint.extraction
      | _ => joint.extraction
    }
  }
  pure def adaptAuthorityF(request: AAuthorityRequest, op: Operation,
    plan: AResolvedPlan, display: DisplayProjection): AAuthorityAdaptation = {
    val req = request
    val operation = op
    val resolved = plan
    val shown = display
    if (not(validOperation(operation)) or req.now == Time0) AuthorityInvalidRequestA
    else match operation {
      | OpCancelParent(_) => if (not(validState(req.before.program, req.before.state))
          or req.input != NoAInput) AuthorityInvalidRequestA
        else AuthorityAdaptedA({predecessor: req.before, proposedSuccessor: req.before,
          artifactAndCall: CancellationCallA({before: req.before, now: req.now}), resolvedPlan: resolved,
          transactionTime: timeValue(req.now), input: NoInput, effects: List(), outcome: Cancellation,
          coreProjection: NoCoreProjection, effectEvidence: EvidenceValid, display: shown})
      | _ => {
          val joint = computeJointF(req.before.program, req.before.state, req.input, req.now)
          match joint.evaluation {
            | TransactionComputedA(raw) => match joint.extraction {
                | EffectsExtractedA(effects) => AuthorityAdaptedA({predecessor: req.before,
                    proposedSuccessor: {program: req.before.program, state: raw.state},
                    artifactAndCall: AgreementCallA(req), resolvedPlan: resolved,
                    transactionTime: timeValue(req.now), input: projectInput(req.input), effects: effects,
                    outcome: if (raw.accepted) acceptedOutcomeA(operation) else Rejected(CoreRejected(raw.error)),
                    coreProjection: CoreProjected(projectResult(req.before.program, raw)),
                    effectEvidence: EvidenceValid, display: shown})
                | _ => AuthorityExtractionDiagnosticA(joint.extraction)
              }
            | _ => AuthorityComputationDiagnosticA(joint.evaluation)
          }
        }
    }
  }
}
```

### Task 2: Full corpus-preserving derivation and identity comparisons

- [ ] Add the derivation script below with apply_patch. It prints an apply_patch payload; capture that output and pass it to apply_patch. Do not use shell redirection to create source files. It fails if any target exists. Derivation is a bulk mechanical rewrite, not a new semantic implementation.
- [ ] Review all generated changes: module/import renaming, three delegate bodies, and boundary identity-binding wrappers only. All original type/constructor labels and other declarations remain byte-derived. No fixture literal, test body, invariant or route is dropped.
- [ ] Run `/home/charl/Moriarty/.venv/bin/python scripts/derive_s02_candidate_a_factored.py --check`; require exact equality for every generated file and source manifest. The explicit name/hash inventory includes 28 originals and derives exactly 26, leaving types/programs unchanged. Pending A4 modules are not selected by a glob. Any pinned-byte change stops derivation and requires root review; future corpus expansion remains an explicit A5 gate.
- [ ] Add the complete equivalence test below. Recursively typecheck the equivalence module and every generated test entry; pin complete closures. Require structural compatibility of all original/factored aliases and sum labels. If Quint reports nominal/type incompatibility, stop for review: do not replace full-state equality by a projection.
- [ ] Run every original `candidate_a*_test.qnt` and its derived counterpart with the exact fixed-inventory recorder loop below; require identical ordered run-name inventories and every passing terminal result. The manifest fixes inventory at derivation, including all 17 installment and all 17 swap tests. Capture each command independently; no test matching subset.
- [ ] Run the six equivalence tests with the same Rust/seed/full-match options. They compare every original agreement-harness prefix through its original four/three-record bound, plus every scheduled authority prefix, actual guards, full updates, completed/refused histories and rebound forged evidence. Thus comparison includes the29 stateful records as well as the24 diagnostic requests in the existing53-record corpus, not merely final balances. No original step request is omitted.
- [ ] Run both derived lifecycle original 100-sample commands with their original maxsteps/invariants and every named witness, using the exact adopted A2/A3 commands with only entry module path changed to the generated `_f` file. Record all 25 installment and 24 swap counts; diagnose any zero before conditional1000 escalation. Do not assert these scheduled routes cover arbitrary interleavings.
- [ ] Run the original full Python suite with `/home/charl/Moriarty/.venv/bin/python -m pytest -q` and complete-inventory comparison with `/home/charl/Moriarty/.venv/bin/python scripts/check_s02_candidate_a_correspondence.py --cases .superpowers/sdd/candidate-a-export-stages/final/cases.json --input-root .superpowers/sdd/candidate-a-export-stages/final/inputs --report .superpowers/sdd/a5-factoring-receipts/core-comparison/report.json`. Create that fresh report directory via the recorder, require exit0 and53 records, and include original cases, all14 raw ITFs, Python Core/swap, checker and test sources in the before/after archive pins. No `--allow-subset`; schema1 inputs remain unchanged. A4's integrated independent replay gate remains separate and must be rebound after any affected producer change. Derived tests are not an independent Python checker.
- [ ] Root reviews original/factored corpus inventories, all terminal outcomes, branch lemmas and exact source closure before authorizing Task3 checker launch.

```python
#!/usr/bin/env python3
"""Print an apply_patch input. Never write or overwrite model files."""
import argparse
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "specs/quint/s02"
TARGET = SOURCE / "factored_verification"
PINNED_ORIGINALS = {
    "candidate_a_authority_adapter": "3f3b093f4c718dd08eda38e610de700d0a24138beb82fd7b2b12dcf9d300bda8",
    "candidate_a_authority_adapter_harness": "d7b54410ec0ed2ce8de0b63e5f13dca685f91298730e33342759bf52aad9dd23",
    "candidate_a_authority_adapter_test": "029b2dfd24380f385c961a3145d52528339ee78b9a4d63b9009e7770f2077cf8",
    "candidate_a_authority_boundary": "95ddf75ef32b432acaec5384c89826d0dc12245b80f78fd8a60ff78d0da8164b",
    "candidate_a_authority_boundary_fixtures": "32e68b44ac77f01df74e200cde3c23150b12e40ff772167dc5fa3c79464b52ad",
    "candidate_a_authority_boundary_harness": "26d13e6f34b4e16158dc92d80ccbdf0075d9291121deb1a88aa76de817c39652",
    "candidate_a_authority_boundary_test": "a5c7645697166d8979125f8f46e81ccb6938d589c69165c01bd540b92a9585cd",
    "candidate_a_authority_installment": "f12d91938098d48a313baf7cb5218f84b6bd840da8c518d54f195e0f7fa1e4cd",
    "candidate_a_authority_installment_fixtures": "22d975d6d615e1f8e80c453115ded79fa68f783880a036f73470814221bf1a92",
    "candidate_a_authority_installment_harness": "9a49b2a76d0c9e27a0d06b942b4f6a3338bf54a1e005cbba6d6388e91dcf82ca",
    "candidate_a_authority_installment_test": "b1d9c21c5285105b382525df8df2ebaa41dbf10979f400d975b40500c3f7c3d1",
    "candidate_a_authority_swap": "294633d4213d44075df76085f67b9bd24fab23d07bc863fff8b4f22e79d10024",
    "candidate_a_authority_swap_fixtures": "fb407555584e167ae0fddc3e59fbf6d64ecd79a00d2ad03d02f159da8429ed15",
    "candidate_a_authority_swap_harness": "bb8991e439153af69f3c8ec52df1771786a0ce0d5360f975189958b978555e29",
    "candidate_a_authority_swap_test": "5aed1719310ef472f8fdd5a19e8cc5188a47c7c136f7fc3158bb5a12943636c5",
    "candidate_a_boundary_test": "1bb9ff76c2cf7f6e73e72f3c3076b1ff490e1ed0e9cc90c0bb71d7e7c01ca623",
    "candidate_a_cases": "03da5af65cd4af2fa2d7e23f87ea43d9212f8ac9edef5dfe86e02ee5c79705cb",
    "candidate_a_cases_test": "b1a6165ba7a69b0557d628594bcb754ada8de0b18da1bb15edd10928fc80e239",
    "candidate_a_core": "e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec",
    "candidate_a_core_test": "91482debc44658a6a55eb6f74b63253f715a9372ed19228e43b16b1261c12375",
    "candidate_a_harness": "0651108d66e40e0295f7ef568665257bee59b93300dae1f7dd6aad684ce8429c",
    "candidate_a_installment_harness": "5912655f3f8b35e7f85602099851fea9bdddf6a28493964ddc834e53b098194e",
    "candidate_a_installment_test": "6ce3ed1077f46c5aabbaf4449548adb39b3916935d4021acf76af1ebd2a80f8f",
    "candidate_a_programs": "bf814bce2924cafac5836a171b52a4c9bdba43e083fe7129bcdeadd810c42d5e",
    "candidate_a_projection": "2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c",
    "candidate_a_projection_test": "3a6eac6f132b86003d0973c0f6fd26af938b56a082a7a648de597c4f5a2c96a2",
    "candidate_a_test": "7906400286288d3419ea22b48d424c68f21b5842fa8349ac156df26286077d25",
    "candidate_a_types": "40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b"
}
NAMES = tuple(n for n in PINNED_ORIGINALS if n not in ("candidate_a_types", "candidate_a_programs"))
J_IMPORT = '  import candidate_a_joint_f as J from "./candidate_a_joint_f"\n'
WRAPPERS = {
    "candidate_a_core": (
        "computeTransaction", None,
        "  pure def computeTransaction(program: AProgram, before: AState, supplied: AInput, now: ATime): ATransactionEvaluation =\n"
        "    J::computeTransactionF(program, before, supplied, now)\n"),
    "candidate_a_projection": (
        "extractCommittedEffects", None,
        "  pure def extractCommittedEffects(program: AProgram, before: AState, input: AInput, now: ATime, result: ATransactionResult): AEffectExtraction =\n"
        "    J::extractCommittedEffectsF(program, before, input, now, result)\n"),
    "candidate_a_authority_adapter": (
        "adaptAuthorityA", "authorityObservationMatchesA",
        "  pure def adaptAuthorityA(request: AAuthorityRequest, op: Operation, plan: AResolvedPlan, display: DisplayProjection): AAuthorityAdaptation =\n"
        "    J::adaptAuthorityF(request, op, plan, display)\n"),
}

def digest(data):
    return hashlib.sha256(data).hexdigest()

def replace_body(text, first, following, replacement):
    start = text.index("  pure def " + first + "(")
    end = text.index("  pure def " + following + "(", start) if following else text.rfind("}")
    return text[:start] + replacement + text[end:]

def boundary_bindings(text):
    # Only declaration names change. Bodies, field labels, strings and selectors are untouched.
    pattern = re.compile(r"(?m)^  pure def (\w+)\(([^)]*)\): ([^=]+?)=")
    matches = list(pattern.finditer(text))
    for m in reversed(matches):
        name = m.group(1)
        internal = "a5_body_" + name
        assert internal not in text
        params = []
        for param in m.group(2).split(","):
            param_name, typ = param.strip().split(":", 1)
            assert re.fullmatch(r"[A-Za-z]\w*", param_name)
            assert typ.strip()
            params.append(param_name)
        bindings = "".join("    val shared_" + p + " = " + p + "\n" for p in params)
        args = ", ".join("shared_" + p for p in params)
        wrapper = m.group(0) + " {\n" + bindings + "    " + internal + "(" + args + ")\n  }\n"
        renamed = m.group(0).replace("pure def " + name + "(", "pure def " + internal + "(", 1)
        text = text[:m.start()] + wrapper + renamed + text[m.end():]
    return text

def check_boundary_roundtrip(original, wrapped):
    wrapper = re.compile(
        r"(?m)^  pure def (\w+)\([^)]*\): [^=]+?= \{\n"
        r"(?:    val shared_\w+ = \w+\n)+"
        r"    a5_body_\1\([^\n]*\)\n  }\n")
    restored = wrapper.sub("", wrapped)
    restored = restored.replace("pure def a5_body_", "pure def ")
    assert restored == original, "Wrapper changed an original body, selector, label or literal"

def derived(name):
    text = (SOURCE / (name + ".qnt")).read_text()
    if name in WRAPPERS:
        first, following, replacement = WRAPPERS[name]
        text = replace_body(text, first, following, replacement)
    if name == "candidate_a_authority_boundary":
        original = text
        text = boundary_bindings(text)
        check_boundary_roundtrip(original, text)
    text = re.sub(r"\bmodule\s+" + name + r"\b", "module " + name + "_f", text, count=1)
    def imported(m):
        module, rest, path = m.groups()
        assert path == module, (module, path)
        if module in NAMES:
            return "import " + module + "_f" + rest + ' from "./' + module + '_f"'
        return "import " + module + rest + ' from "../' + path + '"'
    text = re.sub(r'import (\w+)([^\n]*?) from "\./(\w+)"', imported, text)
    if name in WRAPPERS:
        pos = text.index("\n") + 1
        text = text[:pos] + J_IMPORT + text[pos:]
    return text

def patch_add(path, content):
    return "*** Add File: " + str(path.relative_to(ROOT)) + "\n" + "".join("+" + line + "\n" for line in content.splitlines())

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    assert len(NAMES) == 26, NAMES
    for name, wanted in PINNED_ORIGINALS.items():
        assert digest((SOURCE / (name + ".qnt")).read_bytes()) == wanted, "Original pin mismatch: " + name
    records = [{"path": str((SOURCE / (n + ".qnt")).relative_to(ROOT)),
                "sha256": digest((SOURCE / (n + ".qnt")).read_bytes()),
                "runs": re.findall(r"(?m)^\s*run\s+(\w+)\s*=", (SOURCE / (n + ".qnt")).read_text())}
               for n in NAMES]
    generated = {TARGET / (name + "_f.qnt"): derived(name) for name in NAMES}
    generated[TARGET / "derivation.json"] = json.dumps({"originals": records,
        "rule": "module/import renaming; three joint delegates; boundary identity let bindings only"}, indent=2) + "\n"
    if args.check:
        for path, content in generated.items():
            assert path.read_text() == content, str(path)
        return
    assert all(not p.exists() for p in generated), "Refuse to overwrite a derivation"
    print("*** Begin Patch")
    for path, content in generated.items():
        print(patch_add(path, content), end="")
    print("*** End Patch")

if __name__ == "__main__":
    main()
```

```quint
module candidate_a_factoring_equivalence_test {
  import candidate_a_harness as HS from "../candidate_a_harness"
  import candidate_a_harness_f as HSF from "./candidate_a_harness_f"
  import candidate_a_installment_harness as HI from "../candidate_a_installment_harness"
  import candidate_a_installment_harness_f as HIF from "./candidate_a_installment_harness_f"
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"
  import candidate_a_authority_boundary as B from "../candidate_a_authority_boundary"
  import candidate_a_authority_boundary_f as BF from "./candidate_a_authority_boundary_f"
  import candidate_a_authority_installment_fixtures as I0 from "../candidate_a_authority_installment_fixtures"
  import candidate_a_authority_installment as I from "../candidate_a_authority_installment"
  import candidate_a_authority_installment_f as IF from "./candidate_a_authority_installment_f"
  import candidate_a_authority_swap_fixtures as S0 from "../candidate_a_authority_swap_fixtures"
  import candidate_a_authority_swap as S from "../candidate_a_authority_swap"
  import candidate_a_authority_swap_f as SF from "./candidate_a_authority_swap_f"

  pure val swapRequests = Set({input: HS::aliceFunding, now: Time1},
    {input: HS::bobFunding, now: Time2}, {input: HS::settleInput, now: Time2},
    {input: HS::refundInput, now: Time2}, {input: NoAInput, now: Time100},
    {input: HS::settleInput, now: Time100})
  pure val installmentRequests = Set({input: HI::fillOneInput, now: Time2},
    {input: HI::fillTwoInput, now: Time2}, {input: HI::recoveryInput, now: Time2},
    {input: NoAInput, now: Time100}, {input: HI::recoveryInput, now: Time100})
  pure val allSwapPrefixes = List(0,1,2,3).foldl(Set(HS::swapInitial), (states, _) =>
    states.union(states.map(s => swapRequests.filter(r => HS::canRequest(s,r.input,r.now))
      .map(r => HS::applyRequest(s,r.input,r.now))).flatten()))
  pure val allInstallmentPrefixes = List(0,1,2).foldl(Set(HI::installmentInitial), (states, _) =>
    states.union(states.map(s => installmentRequests.filter(r => HI::canRequest(s,r.input,r.now))
      .map(r => HI::applyRequest(s,r.input,r.now))).flatten()))
  run completeSwapTraceCorpusTest = assert(allSwapPrefixes.forall(s =>
    HS::traceSafety(s) and HS::traceSafety(s) == HSF::traceSafety(s)
      and HS::traceTerminal(s) == HSF::traceTerminal(s)
      and HS::traceEnabled(s) == HSF::traceEnabled(s)
      and swapRequests.forall(r =>
        HS::canRequest(s,r.input,r.now) == HSF::canRequest(s,r.input,r.now)
          and (if (HS::canRequest(s,r.input,r.now))
            HS::applyRequest(s,r.input,r.now) == HSF::applyRequest(s,r.input,r.now) else true))))
  run completeInstallmentTraceCorpusTest = assert(allInstallmentPrefixes.forall(s =>
    HI::traceSafety(s) and HI::traceSafety(s) == HIF::traceSafety(s)
      and HI::traceTerminal(s) == HIF::traceTerminal(s)
      and HI::traceEnabled(s) == HIF::traceEnabled(s)
      and installmentRequests.forall(r =>
        HI::canRequest(s,r.input,r.now) == HIF::canRequest(s,r.input,r.now)
          and (if (HI::canRequest(s,r.input,r.now))
            HI::applyRequest(s,r.input,r.now) == HIF::applyRequest(s,r.input,r.now) else true))))

  run installmentPrefixesEqualTest = assert(I0::PROFILES_I.forall(p => I0::SCENARIOS_I.forall(s =>
    0.to(I::routeI(s).length()).forall(n => {
      val original = I::runPrefixI(s,p,n)
      val factored = IF::runPrefixI(s,p,n)
      original == factored and original.ok
        and I::safetyI(original.state) == IF::safetyI(original.state)
        and (if (n == I::routeI(s).length()) true else {
          val cmd = I::routeI(s).nth(n)
          I::canCommandI(original.state,s,p,cmd) == IF::canCommandI(original.state,s,p,cmd)
            and I::applyCommandI(original.state,s,p,cmd) == IF::applyCommandI(original.state,s,p,cmd)
        })
    }))))
  run swapPrefixesEqualTest = assert(S0::PROFILES_S.forall(p => S0::SCENARIOS_S.forall(s =>
    0.to(S::routeS(s).length()).forall(n => {
      val original = S::prefixS(s,p,n)
      val factored = SF::prefixS(s,p,n)
      original == factored and original.ok
        and S::financialTerminalS(original.state) == SF::financialTerminalS(original.state)
        and (if (n == S::routeS(s).length()) true else {
          val cmd = S::routeS(s).nth(n)
          S::canCommandS(original.state,s,p,cmd) == SF::canCommandS(original.state,s,p,cmd)
            and S::applyCommandS(original.state,s,p,cmd) == SF::applyCommandS(original.state,s,p,cmd)
        })
    }))))
  run stalePrefixesEqualTest = assert(S0::PROFILES_S.forall(p =>
    0.to(S::staleRouteS.length()).forall(n =>
      S::executeListS(S::staleScenarioS,p,S::staleRouteS.slice(0,n))
        == SF::executeListS(S::staleScenarioS,p,S::staleRouteS.slice(0,n)))))
  run reboundForgeryEqualTest = assert(S0::PROFILES_S.forall(p => {
    val prefix = S::prefixS(S::staleScenarioS,p,3)
    prefix.ok and match prefix.state.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => {
          val program = {...a.observation.proposedSuccessor.program,
            nodes: a.observation.proposedSuccessor.program.nodes.put(N15,
              PayA({account: aliceA, payee: Mallory, amount: ConstantA(1), continuation: N0}))}
          val changed = {...a, observation: {...a.observation,
            proposedSuccessor: {...a.observation.proposedSuccessor, program: program}}}
          val bad = {...prefix.state, attempts: prefix.state.attempts.put(FundingOneAttempt, ProposedAttempt(changed))}
          val evidence = S::evidenceS(changed,S::staleScenarioS,p)
          val retained = {...bad, attempts: bad.attempts.put(FundingOneAttempt,
            RejectedOperation({attempt: changed, evidence: evidence, observedContext: bad.authority.context,
              reason: UnauthorizedEffect, stage: VerificationBoundary}))}
          not(B::canVerifyAuthorityA(bad,FundingOneAttempt,evidence))
            and not(BF::canVerifyAuthorityA(bad,FundingOneAttempt,evidence))
            and B::applyRejectProposedAuthorityA(bad,FundingOneAttempt,evidence) == retained
            and BF::applyRejectProposedAuthorityA(bad,FundingOneAttempt,evidence) == retained
        }
      | _ => false
    }
  }))
}
```

### Task 3: Actual funding pilot and paired finite offline experiment

**Files:** Create `candidate_a_funding_pilot.qnt`, `candidate_a_funding_pilot_f.qnt`, and `candidate_a_funding_pilot_test.qnt` in the isolated directory. The recorder script shown in the next section is created as Task1 receipt infrastructure and reused here without rewriting old copies.

**State sketch and scope:** One shared execution record, signing profile, honest/forged selector, cursor, full execution-history list, and raw-result list. Alice/Bob/Mallory and both assets remain present; actual funding moves10 TokenA from Alice wallet to her escrow. Raw predecessor minimumTime0 is unchanged; actual funding is at1 and environment remains1. Full nonce0/nonce1 and attempt maps remain intact. No network/fairness model is added. There are four routes: two profiles times honest/fully rebound forged proposal. Honest has five actions; forged has four, ending in exact rejection under the original signed policy. This is not a full swap or financial-terminal pilot.

- [ ] Write the pilot test first, then both complete modules below with apply_patch. The two module bodies differ only in module name and four import destinations. Read those exact five changes as a preservation review.
- [ ] Capture recursive typecheck and `quint test specs/quint/s02/factored_verification/candidate_a_funding_pilot_test.qnt --backend=rust --seed=42 --match allPilotPrefixesTest`; require one passing quantified test covering every prefix of all four routes, equal full states/guards/updates and terminal records.
- [ ] Use the exact sample commands below, then inspect every action/profile witness. Every witness must be nonzero in100 samples; otherwise inspect deterministic test paths before a separately admitted escalation. A sample result is not checker evidence.
- [ ] Compile original and factored `pilotSafety` once each. Compare generated JSON bytes, total JSON objects, let/app object counts and frontend wall/max-RSS. These metrics describe generated input, not post-InlinePass sharing. Archive both generated inputs themselves, including the original, not merely their hashes.
- [ ] Root reviews those metrics before checking. If either compile fails, JSON is invalid, pins move, or the factored input is not smaller, stop and report H1 unresolved; require a revised/reviewed hypothesis for another expensive experiment. Do not rewrite predicates or domain to pass a size gate.
- [ ] Under the admitted paired experiment run the original baseline checker once and the factored checker once with identical bounds/resources. Original baseline failure is retained as a resource outcome, never marked successful. H1 is supported for this pilot only if the factored result is terminal exit0/NoError with actual states0–5 explored and `pilotSafety` checked. If the factored checker fails before exploration, stop; do not raise heap/depth/wall automatically.
- [ ] Only after factored `pilotSafety` passes, compile/check `neverPrepared` on the identical factored source. Require a genuine state1 counterexample and exit12, with a retained trace showing actual preparation. This is a checker nonvacuity control, not the Task1 behavioral RED and not an architecture counterexample. Failure to produce that expected trace blocks pilot admission.
- [ ] Archive raw command outputs, all generated JSON, source/runner archives, tool pins, resource logs, checker logs/SMT artifacts and counterexample files. Report every result separately. Root independently reviews and decides whether H1 justifies a full-workload A5 execution design; passing this task alone cannot close A5.

Original pilot:

```quint
module candidate_a_funding_pilot {
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import authorization.* from "../authorization"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"
  import candidate_a_authority_swap_fixtures as F from "../candidate_a_authority_swap_fixtures"
  import candidate_a_authority_swap as S from "../candidate_a_authority_swap"
  import candidate_a_authority_boundary as B from "../candidate_a_authority_boundary"
  import candidate_a_core as C from "../candidate_a_core"
  type PilotState = {execution: AAuthorityExecution, profile: SigningProfile, forged: bool,
    cursor: int, history: List[AAuthorityExecution], raw: List[ATransactionEvaluation]}
  pure val scenario = {funded: 2, mode: F::SettleS}
  pure def initial(p: SigningProfile, bad: bool): PilotState = {
    execution: F::unsignedS, profile: p, forged: bad, cursor: 0, history: List(), raw: List()}
  pure def pilotLength(p: PilotState): int = if (p.forged) 4 else 5
  pure def command(i: int): S::CommandS =
    if (i == 0) S::PrepareS({id: FundingOneAttempt, principal: Alice})
    else if (i == 1) S::SignS({id: FundingOneAttempt, principal: Alice})
    else if (i == 3) S::VerifyS(FundingOneAttempt) else S::CommitS(FundingOneAttempt)
  pure def observation(obs: AAuthorityObservation, bad: bool): AAuthorityObservation =
    if (not(bad)) obs else {...obs, proposedSuccessor: {...obs.proposedSuccessor,
      program: {...obs.proposedSuccessor.program, nodes: obs.proposedSuccessor.program.nodes.put(N15,
        PayA({account: aliceA, payee: Mallory, amount: ConstantA(1), continuation: N0}))}}}
  pure def ready(p: PilotState): bool =
    if (p.cursor < 0 or p.cursor >= pilotLength(p)) false
    else if (p.cursor == 2) match F::actualS(FundingOneAttempt,scenario) {
      | AuthorityAdaptedA(obs) => canPropose(p.execution,FundingOneAttempt,OpFund,observation(obs,p.forged),Alice)
      | _ => false }
    else if (p.cursor == 3 and p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => B::canRejectProposedAuthorityA(p.execution,FundingOneAttempt,S::evidenceS(a,scenario,p.profile))
      | _ => false }
    else S::canCommandS(p.execution,scenario,p.profile,command(p.cursor))
  pure def nextExecution(p: PilotState): AAuthorityExecution =
    if (p.cursor == 2) match F::actualS(FundingOneAttempt,scenario) {
      | AuthorityAdaptedA(obs) => applyPropose(p.execution,FundingOneAttempt,OpFund,observation(obs,p.forged),Alice)
      | _ => p.execution }
    else if (p.cursor == 3 and p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => B::applyRejectProposedAuthorityA(p.execution,FundingOneAttempt,S::evidenceS(a,scenario,p.profile))
      | _ => p.execution }
    else S::applyCommandS(p.execution,scenario,p.profile,command(p.cursor))
  pure def advance(p: PilotState): PilotState = {
    val after = nextExecution(p)
    val raw = if (p.cursor != 2) p.raw else p.raw.append(C::computeTransaction(F::programS,F::beforeS(0).state,
      PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 10})),Time1))
    {...p, execution: after, cursor: p.cursor + 1, history: p.history.append(after), raw: raw}
  }
  pure def terminalExact(p: PilotState): bool =
    if (p.cursor != pilotLength(p)) true
    else if (p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | RejectedOperation(r) => {
          val registered = p.history.nth(1)
          p.execution == {...registered, attempts: registered.attempts.put(FundingOneAttempt,
            RejectedOperation({attempt: r.attempt, evidence: S::evidenceS(r.attempt,scenario,p.profile),
              observedContext: registered.authority.context, reason: UnauthorizedEffect, stage: VerificationBoundary}))}
            and r.attempt == {id: FundingOneAttempt, actor: Alice, operation: OpFund,
              observation: observation(F::expectedS(FundingOneAttempt,scenario),true), context: registered.authority.context}
            and not(B::canVerifyAuthorityA(p.history.nth(2),FundingOneAttempt,r.evidence))
        }
      | _ => false }
    else p.execution.authority.context.candidate == F::beforeS(1)
      and p.execution.authority.context.ledger == F::ledgerS(1)
      and p.execution.authority.context.registry.get(F::keyS(Alice,0)) ==
        AuthorityConsumed({signed: {policy: F::policyS(FundingOneAttempt,scenario,Alice,p.profile), signer: Alice, token: 0}, revision: 1})
      and match p.execution.attempts.get(FundingOneAttempt) { | ExecutedOperation(_) => true | _ => false }
  pure def safety(p: PilotState): bool = S::safetyS(p.execution)
    and p.cursor == p.history.length() and p.cursor >= 0 and p.cursor <= pilotLength(p)
    and (if (p.cursor < 3) p.raw == List() else p.raw == List(TransactionComputedA({
      accepted: true, state: F::beforeS(1).state, error: NoCoreError, payments: List(), warnings: List(), reductions: 0})))
    and (if (p.cursor == pilotLength(p)) not(ready(p)) else ready(p))
    and terminalExact(p)
    and (if (p.cursor == 0) p.execution == F::unsignedS else p.execution == p.history.nth(p.cursor - 1))
    and (if (p.cursor < 5) p.execution.authority.context.candidate == F::beforeS(0)
      and p.execution.authority.context.ledger == F::ledgerS(0) else true)
  var pilot: PilotState
  action init = {
    nondet profile = F::PROFILES_S.oneOf()
    nondet forged = Set(false,true).oneOf()
    pilot' = initial(profile,forged)
  }
  action step = all { ready(pilot), pilot' = advance(pilot) }
  val pilotSafety = safety(pilot)
  val neverPrepared = pilot.cursor == 0
  val prepared = pilot.cursor >= 1
  val signed = pilot.cursor >= 2
  val proposed = pilot.cursor >= 3
  val verified = not(pilot.forged) and pilot.cursor >= 4
  val committed = not(pilot.forged) and pilot.cursor == 5
  val rejected = pilot.forged and pilot.cursor == 4
  val afterCompleted = pilot.cursor == pilotLength(pilot) and pilot.profile == SignAfterResolve
  val beforeCompleted = pilot.cursor == pilotLength(pilot) and pilot.profile == SignBeforeResolve
}
```

Factored pilot (complete file, not a substituted expected-output table):

```quint
module candidate_a_funding_pilot_f {
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import authorization.* from "../authorization"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"
  import candidate_a_authority_swap_fixtures_f as F from "./candidate_a_authority_swap_fixtures_f"
  import candidate_a_authority_swap_f as S from "./candidate_a_authority_swap_f"
  import candidate_a_authority_boundary_f as B from "./candidate_a_authority_boundary_f"
  import candidate_a_core_f as C from "./candidate_a_core_f"
  type PilotState = {execution: AAuthorityExecution, profile: SigningProfile, forged: bool,
    cursor: int, history: List[AAuthorityExecution], raw: List[ATransactionEvaluation]}
  pure val scenario = {funded: 2, mode: F::SettleS}
  pure def initial(p: SigningProfile, bad: bool): PilotState = {
    execution: F::unsignedS, profile: p, forged: bad, cursor: 0, history: List(), raw: List()}
  pure def pilotLength(p: PilotState): int = if (p.forged) 4 else 5
  pure def command(i: int): S::CommandS =
    if (i == 0) S::PrepareS({id: FundingOneAttempt, principal: Alice})
    else if (i == 1) S::SignS({id: FundingOneAttempt, principal: Alice})
    else if (i == 3) S::VerifyS(FundingOneAttempt) else S::CommitS(FundingOneAttempt)
  pure def observation(obs: AAuthorityObservation, bad: bool): AAuthorityObservation =
    if (not(bad)) obs else {...obs, proposedSuccessor: {...obs.proposedSuccessor,
      program: {...obs.proposedSuccessor.program, nodes: obs.proposedSuccessor.program.nodes.put(N15,
        PayA({account: aliceA, payee: Mallory, amount: ConstantA(1), continuation: N0}))}}}
  pure def ready(p: PilotState): bool =
    if (p.cursor < 0 or p.cursor >= pilotLength(p)) false
    else if (p.cursor == 2) match F::actualS(FundingOneAttempt,scenario) {
      | AuthorityAdaptedA(obs) => canPropose(p.execution,FundingOneAttempt,OpFund,observation(obs,p.forged),Alice)
      | _ => false }
    else if (p.cursor == 3 and p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => B::canRejectProposedAuthorityA(p.execution,FundingOneAttempt,S::evidenceS(a,scenario,p.profile))
      | _ => false }
    else S::canCommandS(p.execution,scenario,p.profile,command(p.cursor))
  pure def nextExecution(p: PilotState): AAuthorityExecution =
    if (p.cursor == 2) match F::actualS(FundingOneAttempt,scenario) {
      | AuthorityAdaptedA(obs) => applyPropose(p.execution,FundingOneAttempt,OpFund,observation(obs,p.forged),Alice)
      | _ => p.execution }
    else if (p.cursor == 3 and p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => B::applyRejectProposedAuthorityA(p.execution,FundingOneAttempt,S::evidenceS(a,scenario,p.profile))
      | _ => p.execution }
    else S::applyCommandS(p.execution,scenario,p.profile,command(p.cursor))
  pure def advance(p: PilotState): PilotState = {
    val after = nextExecution(p)
    val raw = if (p.cursor != 2) p.raw else p.raw.append(C::computeTransaction(F::programS,F::beforeS(0).state,
      PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 10})),Time1))
    {...p, execution: after, cursor: p.cursor + 1, history: p.history.append(after), raw: raw}
  }
  pure def terminalExact(p: PilotState): bool =
    if (p.cursor != pilotLength(p)) true
    else if (p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | RejectedOperation(r) => {
          val registered = p.history.nth(1)
          p.execution == {...registered, attempts: registered.attempts.put(FundingOneAttempt,
            RejectedOperation({attempt: r.attempt, evidence: S::evidenceS(r.attempt,scenario,p.profile),
              observedContext: registered.authority.context, reason: UnauthorizedEffect, stage: VerificationBoundary}))}
            and r.attempt == {id: FundingOneAttempt, actor: Alice, operation: OpFund,
              observation: observation(F::expectedS(FundingOneAttempt,scenario),true), context: registered.authority.context}
            and not(B::canVerifyAuthorityA(p.history.nth(2),FundingOneAttempt,r.evidence))
        }
      | _ => false }
    else p.execution.authority.context.candidate == F::beforeS(1)
      and p.execution.authority.context.ledger == F::ledgerS(1)
      and p.execution.authority.context.registry.get(F::keyS(Alice,0)) ==
        AuthorityConsumed({signed: {policy: F::policyS(FundingOneAttempt,scenario,Alice,p.profile), signer: Alice, token: 0}, revision: 1})
      and match p.execution.attempts.get(FundingOneAttempt) { | ExecutedOperation(_) => true | _ => false }
  pure def safety(p: PilotState): bool = S::safetyS(p.execution)
    and p.cursor == p.history.length() and p.cursor >= 0 and p.cursor <= pilotLength(p)
    and (if (p.cursor < 3) p.raw == List() else p.raw == List(TransactionComputedA({
      accepted: true, state: F::beforeS(1).state, error: NoCoreError, payments: List(), warnings: List(), reductions: 0})))
    and (if (p.cursor == pilotLength(p)) not(ready(p)) else ready(p))
    and terminalExact(p)
    and (if (p.cursor == 0) p.execution == F::unsignedS else p.execution == p.history.nth(p.cursor - 1))
    and (if (p.cursor < 5) p.execution.authority.context.candidate == F::beforeS(0)
      and p.execution.authority.context.ledger == F::ledgerS(0) else true)
  var pilot: PilotState
  action init = {
    nondet profile = F::PROFILES_S.oneOf()
    nondet forged = Set(false,true).oneOf()
    pilot' = initial(profile,forged)
  }
  action step = all { ready(pilot), pilot' = advance(pilot) }
  val pilotSafety = safety(pilot)
  val neverPrepared = pilot.cursor == 0
  val prepared = pilot.cursor >= 1
  val signed = pilot.cursor >= 2
  val proposed = pilot.cursor >= 3
  val verified = not(pilot.forged) and pilot.cursor >= 4
  val committed = not(pilot.forged) and pilot.cursor == 5
  val rejected = pilot.forged and pilot.cursor == 4
  val afterCompleted = pilot.cursor == pilotLength(pilot) and pilot.profile == SignAfterResolve
  val beforeCompleted = pilot.cursor == pilotLength(pilot) and pilot.profile == SignBeforeResolve
}
```

```quint
module candidate_a_funding_pilot_test {
  import policies.* from "../policies"
  import candidate_a_funding_pilot as O from "./candidate_a_funding_pilot"
  import candidate_a_funding_pilot_f as F from "./candidate_a_funding_pilot_f"
  pure def prefixOriginal(profile: SigningProfile, bad: bool, n: int): O::PilotState =
    0.to(4).fold(O::initial(profile,bad), (p, i) => if (i < n) O::advance(p) else p)
  pure def prefixFactored(profile: SigningProfile, bad: bool, n: int): F::PilotState =
    0.to(4).fold(F::initial(profile,bad), (p, i) => if (i < n) F::advance(p) else p)
  run allPilotPrefixesTest = assert(Set(SignAfterResolve,SignBeforeResolve).forall(profile =>
    Set(false,true).forall(bad => 0.to(if (bad) 4 else 5).forall(n => {
      val original = prefixOriginal(profile,bad,n)
      val factored = prefixFactored(profile,bad,n)
      original == factored and O::safety(original) and F::safety(factored)
        and O::ready(original) == F::ready(factored)
        and (if (O::ready(original)) O::advance(original) == F::advance(factored) else true)
    }))))
}
```

### Immutable bounded command recorder

Create `scripts/run_s02_candidate_a_factoring_pilot.py` and the Task2 derivation script as Task1 infrastructure before the first test command. The derivation script is only invoked to produce model patches after Task1 admission. Its explicit source pins also supply the recorder's closed original inventory. The recorder archives all that inventory plus the isolated models and six common imports; it does not include concurrent A4 source files by glob. Record root's actual A2/A3-admitted pre-dispatch base separately from the current sourceCommit.

The shared `.superpowers/sdd/a5-factoring-receipts/tool-store/` retains original runtime bytes once. Its manifest pins actual Node, the installed Quint dist/dependency/package metadata closure, Rust, Java runtime lib/conf/release and launch helpers/native libraries. It validates/reuses the existing3329-file Python snapshot at its exact admitted manifest/archive digests, instead of copying a venv per stage. Bootstrap validates tar member bytes and captures raw Node/Quint/Java/Python/Rust/libc version output. Every command rechecks every current runtime file and both retained archive/manifest digests before and after; receipts bind that shared store separately from semantic source pins. Use explicit Node+CLI argv, controlled loader/path variables, disabled Node compile cache and a fresh Python cache prefix with bytecode writes disabled. No full OS-image claim is made; archive the complete tool-store and referenced Python snapshot with final evidence. Producer consumes this same admitted store without creating a second one.

Each compile has900-second wall and4096-MiB Node heap limits; each checker has600-second wall and4096-MiB JVM heap. Each pilot sample/test/typecheck has1200-second wall. GNU time reports maximum resident memory; this is an observed process-resource metric, not proof of a strict whole-tree memory ceiling. Timeout kills and waits for the launched process group; preserve the real exit and timedOut flag. No command is started twice to obtain a better receipt.

```python
#!/usr/bin/env python3
"""Immutable, finite offline pilot receipts. No service/listener or retry loop."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import signal
import shutil
import re
import subprocess
import sys
import tarfile
import time

ROOT = Path(__file__).resolve().parents[1]
MODEL = ROOT / "specs/quint/s02/factored_verification"
RECEIPTS = ROOT / ".superpowers/sdd/a5-factoring-receipts"
QUINT = Path("/home/charl/.npm-global/bin/quint")
RUST = Path("/home/charl/.quint/rust-evaluator-v0.6.0/quint_evaluator")
AP = Path("/home/charl/.quint/apalache-dist-0.56.1/apalache/bin/apalache-mc")
JAR = AP.parent.parent / "lib/apalache.jar"
JAVA = Path(subprocess.check_output(["which", "java"], text=True).strip()).resolve()
PINNED = {
 str(QUINT.resolve()): "ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501",
 str(RUST): "b2efdeac5713d153e41bf2143b94ed75d888fdd5637f4a5d61a04c695313510a",
 str(AP): "bda52d2dbdbc7f6e95289a69dfe7ddeb162493ddd3501898d33ea7d1da3a8cd7",
 str(JAR): "4753c0ebb2cbb266e2c6ac19ab5ca3827d726cc80fd1fc5d7c1eeb64736cd60b",
}
WITNESSES = ("prepared", "signed", "proposed", "verified", "committed", "rejected",
             "afterCompleted", "beforeCompleted")

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


NODE = Path(shutil.which("node")).resolve()
QUINT_PACKAGE = QUINT.resolve().parents[2]
PYTHON_MANIFEST = ROOT / ".superpowers/sdd/a4-checker-task1-receipts/python-environment.json"
PYTHON_ARCHIVE = PYTHON_MANIFEST.with_suffix(".tar.gz")
PYTHON_MANIFEST_SHA = "cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4"
PYTHON_ARCHIVE_SHA = "7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac"

def controlled_environment(heap):
    env = dict(os.environ)
    for key in ("NODE_PATH", "NODE_COMPILE_CACHE", "PYTHONPATH", "JAVA_TOOL_OPTIONS", "_JAVA_OPTIONS",
                "JDK_JAVA_OPTIONS", "LD_PRELOAD", "LD_LIBRARY_PATH"):
        env.pop(key, None)
    env.update(PATH=os.pathsep.join((str(JAVA.parent), str(NODE.parent), "/usr/bin", "/bin")),
        APALACHE_JAR=str(JAR), JVM_ARGS="-Xmx" + str(heap) + "m",
        JVM_GC_ARGS="-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent",
        NODE_OPTIONS="--max-old-space-size=4096", NODE_DISABLE_COMPILE_CACHE="1",
        PYTHONNOUSERSITE="1", PYTHONDONTWRITEBYTECODE="1")
    return env

def verify_python_snapshot():
    assert sha(PYTHON_MANIFEST) == PYTHON_MANIFEST_SHA
    assert sha(PYTHON_ARCHIVE) == PYTHON_ARCHIVE_SHA
    manifest = json.loads(PYTHON_MANIFEST.read_text())
    assert manifest["archiveSha256"] == PYTHON_ARCHIVE_SHA
    assert Path(manifest["pythonResolved"]) == Path(sys.executable).resolve()
    records = manifest["sourceBytes"]
    expected = {r["archivePath"]: r for r in records}
    assert len(expected) == len(records)
    with tarfile.open(PYTHON_ARCHIVE, "r:gz") as archive:
        actual = {m.name: m for m in archive.getmembers() if m.isfile()}
        assert actual.keys() == expected.keys()
        for name, item in expected.items():
            assert hashlib.sha256(archive.extractfile(actual[name]).read()).hexdigest() == item["sha256"]
            assert sha(item["path"]) == item["sha256"]
    return records

def bootstrap(before_dispatch_base):
    assert re.fullmatch(r"[0-9a-f]{40}", before_dispatch_base)
    assert subprocess.check_output(["git","rev-parse","HEAD"], cwd=ROOT, text=True).strip() == before_dispatch_base
    for path, wanted in PINNED.items():
        assert sha(path) == wanted, "Tool pin mismatch: " + path
    store = RECEIPTS / "tool-store"
    store.mkdir(parents=True, exist_ok=False)
    python_records = verify_python_snapshot()
    paths = {NODE, QUINT.resolve(), RUST, AP, JAR, JAVA, Path(sys.executable).resolve(), Path("/usr/bin/time")}
    paths.update(Path(shutil.which(n)).resolve() for n in ("bash","env","dirname","readlink","mkdir","mktemp","ldd"))
    java_root = JAVA.parent.parent
    assert (java_root/"lib/modules").is_file() and (QUINT_PACKAGE/"package.json").is_file()
    # Conservative installed dependency closure, not unrelated global npm/JDK packages.
    tree_roots = (QUINT_PACKAGE/"dist", QUINT_PACKAGE/"node_modules", java_root/"lib", java_root/"conf")
    tree_members = {}
    for root in tree_roots:
        tree_members[str(root)] = []
        for p in root.rglob("*"):
            assert not(p.is_symlink() and p.is_dir()), "Review external directory dependency: " + str(p)
            if p.is_file():
                paths.add(p.resolve())
                tree_members[str(root)].append(str(p.resolve()))
        tree_members[str(root)] = sorted(set(tree_members[str(root)]))
    paths.update((QUINT_PACKAGE/"package.json", java_root/"release"))
    native = paths.union(Path(r["path"]) for r in python_records if r["path"].endswith(".so"))
    ldd_receipts = []
    for p in sorted(native):
        with p.open("rb") as stream:
            elf = stream.read(4) == b"\x7fELF"
        if not elf:
            continue
        result = subprocess.run(["ldd",str(p)], capture_output=True, timeout=30,
                                env=controlled_environment(4096))
        ldd_receipts.append({"argv":["ldd",str(p)],"exitCode":result.returncode,
            "stdout":result.stdout.decode(errors="replace"),"stderr":result.stderr.decode(errors="replace")})
        for path in re.findall(r"(?:=>\s*)?(/[^\s()]+)", result.stdout.decode(errors="replace")):
            dependency = Path(path)
            if dependency.is_file():
                paths.add(dependency.resolve())
    records = [{"path":str(p), "archivePath":str(p).lstrip("/"), "sha256":sha(p)} for p in sorted(paths)]
    with tarfile.open(store/"runtime.tar.gz","x:gz",dereference=True) as archive:
        for item in records:
            archive.add(item["path"],arcname=item["archivePath"],recursive=False)
    with tarfile.open(store/"runtime.tar.gz","r:gz") as archive:
        members = archive.getmembers()
        assert all(m.isfile() for m in members)
        expected = {item["archivePath"]:item for item in records}
        assert {m.name for m in members} == expected.keys()
        for member in members:
            assert hashlib.sha256(archive.extractfile(member).read()).hexdigest() == expected[member.name]["sha256"]
    versions = []
    for name,argv in (
        ("node",[str(NODE),"--version"]),
        ("quint",[str(NODE),str(QUINT.resolve()),"--version"]),
        ("java",[str(JAVA),"-version"]),
        ("python",[sys.executable,"--version"]),
        ("rust",[str(RUST),"--version"]),
        ("libc",["ldd","--version"])):
        result = subprocess.run(argv,capture_output=True,timeout=30,env=controlled_environment(4096))
        (store/(name+".stdout")).write_bytes(result.stdout)
        (store/(name+".stderr")).write_bytes(result.stderr)
        assert result.returncode == (1 if name == "rust" else 0), "Unexpected version command result: "+name
        versions.append({"name":name,"argv":argv,"exitCode":result.returncode,
            "stdoutSha256":sha(store/(name+".stdout")),"stderrSha256":sha(store/(name+".stderr"))})
    manifest = {"files":records,"treeMembers":tree_members,"archiveSha256":sha(store/"runtime.tar.gz"),
        "reusedPythonManifest":str(PYTHON_MANIFEST),"reusedPythonManifestSha256":PYTHON_MANIFEST_SHA,
        "reusedPythonArchive":str(PYTHON_ARCHIVE),"reusedPythonArchiveSha256":PYTHON_ARCHIVE_SHA,
        "pythonFiles":python_records,"versions":versions,"dynamicLibraryReceipts":ldd_receipts,
        "quintPackage":json.loads((QUINT_PACKAGE/"package.json").read_text()),
        "javaRelease":(java_root/"release").read_text(),
        "scope":"actual launchers, installed Quint dependency tree, Java runtime, native library closure; reused admitted Python source/native closure; not an OS image"}
    (store/"manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
    dispatch={"beforeDispatchCommit":before_dispatch_base,"runtimeManifestSha256":sha(store/"manifest.json"),
        "runtimeArchiveSha256":sha(store/"runtime.tar.gz")}
    (RECEIPTS/"dispatch.json").write_text(json.dumps(dispatch,indent=2)+"\n")
    verify_runtime(before_dispatch_base)
    print(json.dumps(dispatch),flush=True)

def dispatch_base():
    return json.loads((RECEIPTS/"dispatch.json").read_text())["beforeDispatchCommit"]

def verify_runtime(before_dispatch_base):
    dispatch=json.loads((RECEIPTS/"dispatch.json").read_text())
    assert before_dispatch_base == dispatch["beforeDispatchCommit"]
    store=RECEIPTS/"tool-store"
    assert sha(store/"manifest.json") == dispatch["runtimeManifestSha256"]
    manifest=json.loads((store/"manifest.json").read_text())
    for root, expected in manifest["treeMembers"].items():
        assert sorted({str(p.resolve()) for p in Path(root).rglob("*") if p.is_file()}) == expected
    assert sha(store/"runtime.tar.gz") == manifest["archiveSha256"] == dispatch["runtimeArchiveSha256"]
    assert sha(PYTHON_MANIFEST) == PYTHON_MANIFEST_SHA and sha(PYTHON_ARCHIVE) == PYTHON_ARCHIVE_SHA
    for item in manifest["files"] + manifest["pythonFiles"]:
        assert sha(item["path"]) == item["sha256"], "Runtime moved: "+item["path"]
    assert NODE == Path(shutil.which("node")).resolve() and JAVA == Path(shutil.which("java")).resolve()
    for item in manifest["versions"]:
        assert sha(store/(item["name"]+".stdout")) == item["stdoutSha256"]
        assert sha(store/(item["name"]+".stderr")) == item["stderrSha256"]
    return {"dispatchSha256":sha(RECEIPTS/"dispatch.json"), **dispatch}

def record(stage, argv, wall, heap, property_name, depth, compile_output=False, extra=(), *, before_dispatch_base, domain="two-profile actual Alice funding and rebound forgery pilot"):
    runtime_before = verify_runtime(before_dispatch_base)
    requested_argv = list(argv)
    if argv[0] == str(QUINT):
        argv = [str(NODE), str(QUINT.resolve()), *argv[1:]]
    out = RECEIPTS / stage
    out.mkdir(parents=True, exist_ok=False)
    derivation = ROOT / "scripts/derive_s02_candidate_a_factored.py"
    spec = importlib.util.spec_from_file_location("a5_derivation", derivation)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    base = ROOT / "specs/quint/s02"
    sources = [base / (name + ".qnt") for name in module.PINNED_ORIGINALS]
    sources += [base / (name + ".qnt") for name in
                ("effects", "consumption", "observations", "policies", "authorization", "execution")]
    sources += sorted(MODEL.glob("*.qnt"))
    sources += [Path(__file__).resolve(), derivation]
    if (MODEL / "derivation.json").exists():
        sources.append(MODEL / "derivation.json")
    for name, wanted in module.PINNED_ORIGINALS.items():
        assert sha(base / (name + ".qnt")) == wanted, "Original pin mismatch: " + name
    tools = [NODE, QUINT.resolve(), RUST, AP, JAR, JAVA, Path(sys.executable).resolve(), Path("/usr/bin/time")]
    for path, wanted in PINNED.items():
        assert sha(path) == wanted, "Tool pin mismatch: " + path
    pins = [{"path": str(p), "sha256": sha(p)} for p in sources + tools + list(extra)]
    with tarfile.open(out / "source-and-runner.tar.gz", "x:gz") as archive:
        for path in sources + list(extra):
            archive.add(path, arcname=str(path.relative_to(ROOT)), recursive=False)
    metadata = {"argv": argv, "requestedArgv": requested_argv, "beforeDispatchCommit": before_dispatch_base,
      "runtimeSnapshot": runtime_before, "cwd": str(ROOT), "property": property_name, "depth": depth,
      "sourceCommit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
      "pins": pins, "sourceArchiveSha256": sha(out / "source-and-runner.tar.gz"),
      "domains": domain,
      "fairness": "none", "deadlock": "checker deadlock checking disabled by --no-deadlock; pilotSafety requires ready at every nonterminal and disabled step at terminal",
      "limits": {"wallSeconds": wall, "jvmHeapMiB": heap, "nodeHeapMiB": 4096},
      "toolVersions": {"quint": "0.32.0 (pinned earlier raw version receipt)",
        "rust": "v0.6.0 directory label; --version unsupported",
        "apalache": "0.56.1/build70cdaf4 pinned jar"},
      "classification": "pilot only; not full Candidate A A5"}
    (out / "input.json").write_text(json.dumps(metadata, indent=2) + "\n")
    with (out / "java-version.stdout").open("xb") as stdout, (out / "java-version.stderr").open("xb") as stderr:
        version = subprocess.run([str(JAVA), "-version"], stdout=stdout, stderr=stderr, timeout=10)
    assert version.returncode == 0
    output = out / ("input.qnt.json" if compile_output else "stdout.txt")
    wrapped = ["/usr/bin/time", "-v", "-o", str(out / "resources.txt"), *argv]
    env = controlled_environment(heap)
    env["PYTHONPYCACHEPREFIX"] = str(out/"fresh-python-cache")
    started = time.monotonic()
    timed_out = False
    with output.open("xb") as stdout, (out / "stderr.txt").open("xb") as stderr:
        proc = subprocess.Popen(wrapped, cwd=ROOT, env=env, stdout=stdout, stderr=stderr, start_new_session=True)
        try:
            exit_code = proc.wait(timeout=wall)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(proc.pid, signal.SIGTERM)
            try:
                exit_code = proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL)
                exit_code = proc.wait()
    try:
        runtime_after = verify_runtime(before_dispatch_base)
        runtime_unchanged = runtime_after == runtime_before
    except (AssertionError, OSError):
        runtime_unchanged = False
    result = {"exitCode": exit_code, "timedOut": timed_out, "durationSeconds": time.monotonic()-started,
      "stdoutSha256": sha(output), "stderrSha256": sha(out/"stderr.txt"),
      "runtimeUnchanged": runtime_unchanged,
      "sourceAndToolsUnchanged": runtime_unchanged and all(sha(p["path"]) == p["sha256"] for p in pins)}
    if compile_output and exit_code == 0 and not timed_out:
        try:
            document = json.loads(output.read_text())
            counts = {"objects": 0, "letObjects": 0, "appObjects": 0}
            stack = [document]
            while stack:
                item = stack.pop()
                if isinstance(item, dict):
                    counts["objects"] += 1
                    counts["letObjects"] += item.get("kind") == "let"
                    counts["appObjects"] += item.get("kind") == "app"
                    stack.extend(item.values())
                elif isinstance(item, list):
                    stack.extend(item)
            result["generated"] = {"bytes": output.stat().st_size, **counts}
        except (ValueError, MemoryError) as error:
            result["measurementError"] = type(error).__name__
    (out / "result.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"stage": stage, **result}), flush=True)
    if not result["sourceAndToolsUnchanged"]:
        raise SystemExit("Source/tool movement invalidates stage")
    return 124 if timed_out else exit_code

def main():
    if len(sys.argv) == 3 and sys.argv[1] == "bootstrap":
        bootstrap(sys.argv[2])
        return
    mode, variant, prop, before_dispatch_base = sys.argv[1:5]
    def run(*args, **kwargs):
        return record(*args, before_dispatch_base=before_dispatch_base, **kwargs)
    assert variant in ("original", "factored")
    stem = "candidate_a_funding_pilot" + ("_f" if variant == "factored" else "")
    entry = MODEL / (stem + ".qnt")
    stage = mode + "-" + variant + "-" + prop
    if mode == "compile":
        assert prop in ("pilotSafety", "neverPrepared")
        code = run(stage, [str(QUINT), "compile", str(entry), "--main="+stem, "--target=json",
          "--invariant="+prop, "--verbosity=0"], 900, 4096, prop, 5, True)
    elif mode == "check":
        assert prop in ("pilotSafety", "neverPrepared")
        compiled = RECEIPTS / ("compile-"+variant+"-"+prop)
        previous = json.loads((compiled / "result.json").read_text())
        assert previous["exitCode"] == 0 and not previous["timedOut"] and previous["sourceAndToolsUnchanged"]
        assert "measurementError" not in previous
        compiled_metadata = json.loads((compiled / "input.json").read_text())
        assert all(sha(p["path"]) == p["sha256"] for p in compiled_metadata["pins"])
        inp = compiled / "input.qnt.json"
        assert sha(inp) == previous["stdoutSha256"]
        code = run(stage, [str(AP), "--out-dir="+str(RECEIPTS/stage/"apalache"), "check",
          "--init=q::init", "--next=q::step", "--inv=q::inv", "--length=5", "--no-deadlock", str(inp)],
          600, 4096, prop, 5, extra=(inp,))
    elif mode == "sample":
        assert prop == "pilotSafety"
        code = run(stage, [str(QUINT), "run", str(entry), "--backend=rust", "--seed=42",
          "--max-samples=100", "--max-steps=5", "--invariant=pilotSafety", "--witnesses", *WITNESSES,
          "--verbosity=1"], 1200, 4096, prop, 5)
    else:
        raise SystemExit("Unsupported stage; no implicit retry")
    raise SystemExit(code)

if __name__ == "__main__":
    main()
```

Exact pilot commands, in gated order (cwd is this worktree). Each command writes a unique directory and refuses to overwrite it. The following first two lines run once, at the exact root-dispatched base, before Task1 RED; keep that base fixed across later root documentation commits. Every main command accepts that full hash as its fourth argument, and every direct recorder call passes it explicitly:

```sh
A5_DISPATCH_BASE=$(git rev-parse HEAD)
PYTHONDONTWRITEBYTECODE=1 /home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py bootstrap "$A5_DISPATCH_BASE"
```

```sh
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py sample original pilotSafety "$A5_DISPATCH_BASE"
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py sample factored pilotSafety "$A5_DISPATCH_BASE"
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py compile original pilotSafety "$A5_DISPATCH_BASE"
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py compile factored pilotSafety "$A5_DISPATCH_BASE"
```

Pause for root's generated-size review. After admission:

```sh
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py check original pilotSafety "$A5_DISPATCH_BASE"
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py check factored pilotSafety "$A5_DISPATCH_BASE"
```

After the factored safety result has actually passed:

```sh
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py compile factored neverPrepared "$A5_DISPATCH_BASE"
/home/charl/Moriarty/.venv/bin/python scripts/run_s02_candidate_a_factoring_pilot.py check factored neverPrepared "$A5_DISPATCH_BASE"
```

For kernel/equivalence tests and typechecks, call the same recorder directly; there is no unrecorded shell command alias. Example is the exact initial behavioral RED typecheck/test pair. Terminal assertions below require the expected outcomes; an unexpected result stops the sequence:

```python
from scripts.run_s02_candidate_a_factoring_pilot import record, QUINT, MODEL, dispatch_base
from functools import partial
record = partial(record, before_dispatch_base=dispatch_base())
entry = MODEL / "candidate_a_joint_f_test.qnt"
assert record("kernel-red-typecheck", [str(QUINT), "typecheck", str(entry)],
    1200,4096,"typecheck",0,domain="complete original and factored kernel closure") == 0
assert record("kernel-red-test", [str(QUINT),"test",str(entry),"--backend=rust","--seed=42",
    "--match","suppliedResultRejectedTest"],1200,4096,"suppliedResultRejectedTest",0,
    domain="all24 original named requests with full supplied-result mutations") == 1
```

After the one-line correction, use the exact fresh pair:

```python
from scripts.run_s02_candidate_a_factoring_pilot import record, QUINT, MODEL, dispatch_base
from functools import partial
record = partial(record, before_dispatch_base=dispatch_base())
entry = MODEL / "candidate_a_joint_f_test.qnt"
assert record("kernel-green-typecheck",[str(QUINT),"typecheck",str(entry)],
    1200,4096,"typecheck",0,domain="complete original and factored kernel closure") == 0
assert record("kernel-green-tests",[str(QUINT),"test",str(entry),"--backend=rust","--seed=42",
    "--match",".*Test"],1200,4096,"all6kernelTests",0,domain="all kernel tests without exclusions") == 0
```

Task2's exact all-original/all-derived deterministic command loop uses the fixed derivation inventory and records each run separately. Inspect passing names against each record's `runs`; exit0 alone is insufficient if no tests ran.

```python
import json
from scripts.run_s02_candidate_a_factoring_pilot import record, QUINT, MODEL, ROOT, dispatch_base
from functools import partial
record = partial(record, before_dispatch_base=dispatch_base())
inventory = json.loads((MODEL/"derivation.json").read_text())["originals"]
for item in inventory:
    if not item["runs"]:
        continue
    original = ROOT/item["path"]
    factored = MODEL/(original.stem+"_f.qnt")
    for label,entry in (("original",original),("factored",factored)):
        stage = "corpus-"+label+"-"+original.stem
        assert record(stage+"-typecheck",[str(QUINT),"typecheck",str(entry)],1200,4096,
            "typecheck",0,domain="complete original26-module corpus and all derived test entries") == 0
        assert record(stage+"-tests",[str(QUINT),"test",str(entry),"--backend=rust","--seed=42",
            "--match",".*Test"],1200,4096,"allRecordedRunNames",0,
            domain=json.dumps(item["runs"])) == 0
for stem in ("candidate_a_factoring_equivalence_test",):
    # Task2 only. The independently admitted Task3 pilot command is separate.
    entry=MODEL/(stem+".qnt")
    assert record(stem+"-typecheck",[str(QUINT),"typecheck",str(entry)],1200,4096,
        "typecheck",0,domain=stem) == 0
    assert record(stem+"-tests",[str(QUINT),"test",str(entry),"--backend=rust","--seed=42",
        "--match",".*Test"],1200,4096,"allTests",0,domain=stem) == 0
```

Task2 also has a compiling boundary negative control, which must run before the full corrected corpus loop: before the first factored `reboundForgeryEqualTest` run, replace ONLY the generated public `authorityAttemptMatchesA` wrapper with this temporary body:

```quint
  pure def authorityAttemptMatchesA(attempt: AAuthorityAttempt): bool = true
```

Capture recursive typecheck exit0 then the exact `reboundForgeryEqualTest` assertion failure with the recorder under `boundary-red-typecheck`/`boundary-red-test`. The before-resolution profile supplies fully rebound evidence and exposes the missing A fidelity gate; the other profile may retain independent protection. Archive the full RED closure, then restore the exact generated wrapper and require `derive_s02_candidate_a_factored.py --check` plus the complete six-test equivalence GREEN. A surviving or redundant control is investigated, never mislabeled RED.

## Full A5 obligation map: mandatory after pilot, not discharged by it

This is the concrete H1 implementation/pilot plan. It deliberately does not authorize an unreviewed full-workload checker campaign: root must use measured pilot outcome and accepted A4 inventory to admit the next bounded verification manifest. The requirements below remain mandatory; failure of H1 does not remove them. A full-campaign plan must supply exact property operators/domains and terminal receipts for every row before A5 acceptance.

| Required obligation | Frozen source/property anchor and retained finite coverage | Admission evidence still required |
|---|---|---|
| Raw Core semantics, diagnostics, order, rollback, complete programs/maps | `candidate_a_core_test`, `candidate_a_projection_test`, all24 `CASE_REQUESTS`, `coreTraceSafety`, `installmentTraceSafety`, `corpusComplete` | Full original/factored corpus and independent pinned Python comparison; terminal checks of actual trace predicates, including timeout input rollback and pre/deposit/post order. Old93MB failure is unresolved here. |
| Actual A adapter and complete binding, no extra effects | `adapterBindingSafety`, original adapter mutation tests and boundary40-test inventory | Whole observation/call/program/result/effect mutation controls and terminal bounded A-boundary property checks. Pilot only covers Alice funding and one rebound successor mutation. |
| Signed envelopes, exact effect multiplicity, refund/change destinations | Actual `canVerifyAuthorityA`, `canCommitAuthorityA`, common policy clauses/effectOrder, original boundary and A2/A3 negatives | Executed-attempt signed-envelope predicate checked for every required workload and both profiles. No claim from balance conservation alone. |
| Nonnegative balances and per-asset conservation | `installmentSafetyI`, `swapSafetyS`, actual ledgers and six escrow accounts | Complete installment11x2 and swap12x2 ordinary routes plus both-profile stale route, with terminal checker depth at least20/22 and exact action inventory. Samples are supporting evidence only. |
| Replay, nonce, duplicate, cancellation race | A2 both race orders, verified retained stale loser, two fills, fresh cancellation, duplicate tests, `retainedRaceI`, `recoveryPreservesI` | Both contender proposal/verification orders and winner orders retained; full registry/parent revision accounting; replay must not commit. Scheduled route checking is not unrestricted interleaving verification. |
| Residual non-expansion and recovery history | Parent paid+allowance10; fill5/5; rev1/rev2; initial/residual recovery10/5; nonce1 signing against actual cancelled history | Full parent/nonce0 preservation even when escrow0 and allowance10/5; missing cancel/unsigned/oldnonce negatives; finite soleRecoveryAttempt refusal versus financial-terminal distinction. |
| Anchors, implementation/enforcement versions, artifact identity at both boundaries | S02 design finite anchor/version domain0/1; common stale binding checks; A3 stale signing and verified attempt controls | An explicitly reviewed interleaving driver that exposes environment changes between prepare/sign/verify/commit under actual A boundary, without dropping original routes. Existing scheduled A2/A3 drivers do not exhaust this domain. |
| Rejection atomicity and evidence retention | Exact `RejectedOperation` original attempt, supplied evidence, observed context, reason and stage; `transitionExactI`, `nonCommitS` | Full-state rejection invariants and bounded controls, including rollback fields, empty timeout envelope rejection and retained stale loser. No financial terminal claim for refusal. |
| Display/disclosure cannot grant effects; evidence-level honesty | Original common/policy/boundary tests; EvidenceValid/Unavailable/Invalid semantics | Bounded adverse evidence/display/disclosure substitutions. Symbolic evidence premise remains stated; no cryptographic soundness claim. |
| Positive nonvacuity and terminal/deadlock treatment | Every original action witness, both profiles, settlement/refund/deadline/fill/cancel/recovery witnesses | Actual terminal checker reachability traces or equivalent checked witness queries for required endpoints, not only a false invariant at init. All nonterminal guards enabled; no stutter hiding a deadlock. |
| Complete fixed workload inventory | XML A2 I01-I11 and A3 S01-S08 with both profiles; accepted A4 mandatory inventory | Reconcile original26-module corpus plus all newly accepted A4 integrated trace cases/negatives. The explicit generator inventory excludes future A4 modules to prevent races; it does NOT waive their later comparison or coverage. |

Full-workload finite bounds must be declared before checking: participants Alice/Bob/Mallory; assets TokenA/TokenB; installments1/2; nonces0/1; complete original domain tags; times0/1/2/100/101; anchors/implementation/enforcement versions0/1 where the S02 design requires adverse changes; exact quantities and bounded reductions from original types; full policy/parent/evidence/attempt maps. A narrower pilot remains labeled narrower. No fairness or arbitrary successful retrial may erase deliberately retained refusal records.

H1 changes no history recurrence. If history replay remains dominant, incremental recurrence or internal microsteps are a DIFFERENT hypothesis requiring new explicit trace-preservation arguments and independent review. Neither is authorized by this plan.

### A5 requirement disposition

- A5-R01: Tasks1–2 establish branch-review obligations, full original corpus comparison and full-state prefix correspondence; A4 expansion remains a full-campaign gate.
- A5-R02: Task3 recorder declares pilot domains, depth5, properties, fairness none, disabled terminal step, tool pins and finite resources before each command; future manifests must do the same for their larger domains.
- A5-R03: Pilot claim requires actual terminal result plus trace-reaching negative control; each full-campaign property needs its own terminal evidence or an explicitly mapped checked conjunction.
- A5-R04: Every timeout/OOM/unsupported translation is a retained incomplete result. Failure before exploration cannot be described as a semantic counterexample.
- A5-R05: The fixed original derivation retains all bodies/tests/invariants/routes, while the full-obligation map prohibits substituting pilot coverage for complete workloads or deleting pending A4 cases.

## Final review and handoff

- [ ] Self-review code blocks, identifier/type imports, exact field preservation, source pins, test counts, terminal lengths, resource bounds and no-listener arguments.
- [ ] Root independently reviews this plan before any implementation. Code is specified-only until genuine task receipts exist; Python derivation self-checks are not Quint typechecks.
- [ ] After implementation, submit immutable source/RED/GREEN/generated-input/checker archives with separate original versus factored outcomes and the unresolved full-A5 matrix. Do not mark A5 complete from this pilot.
- [ ] Await root's next bounded dispatch; no commit, DB/main/wiki/evidence integration, Council or Foreman action is authorized here.
