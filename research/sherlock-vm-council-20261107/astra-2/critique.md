# astra-2 cross-critique

I read all six positions, including my own, after publishing my independent position. Source base: `8fd34ed794eda7186c32495f715609ec54a9a297`. This critique evaluates proposed design obligations. It does not require implementation during the council.

## fable-1

**Agreement.** Keep the reference machine, `StepInv`, in-circuit dispatch and leaf → IVC → decider architecture. A universal leaf with an authenticated family fallback is reasonable. Freeze the representation and qualification configuration. Sources: `fable-1/position.md:Keep`, `docs/sherlock/stagenet-gp-recursion.md:148` and `openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md:A4`.

**Disagreement 1: the proposed statement is incomplete.** K4 and I1 retain five semantic lanes. The actual IVC consumes six semantic lanes plus binding, including `steps`. For S-step leaves, define `steps` as the number of enabled machine transitions. Define the rule lane as either a single rule or an ordered rules commitment under a versioned schema. Keep semantic charges separate because publication is free. Constrain padding, terminality, internal links and the exact count. Sources: `crates/mori-recursion/src/ivc.rs:58`, `:191`, `docs/sherlock/design.md`, “Trace continuity, observer stops and persistent boundaries.”

**Disagreement 2: one-hot dispatch needs complete composition.** Exactly one selected bit does not constrain a missing rule implementation. Unsupported rules must fail closed. Every selected fragment must enforce its effect and preserve `StepInv`. Disabled slots must not change state or add count. Source: ZR06/ZR07 in `docs/MORIARTY-BACKEND-REQUIREMENTS.md` and `c6_step_kit.rs:739` under `crates/mori-backend/src/sherlock_machine/`.

**Disagreement 3: preserve the obligations behind MC03.** Supersede its implementation route only with an explicit requirement-to-successor crosswalk. A pure machine run does not discharge financial episode, duty, genesis or continuation obligations. Sources: `openspec/changes/mc03-native-recursive-proof/design.md`, `docs/MORIARTY-BACKEND-REQUIREMENTS.md:ZR03–ZR11`.

**Disagreement 4: reduce dispatch cost without silently discarding coverage.** A finite exhaustive test is not a theorem, but its checks can still detect defects. Replace expensive redundant checks with a documented per-rule and boundary-case matrix, then add sampled traces. The proposed 24-state sample and two-hour cap need measurement and coverage rationale. Source: `crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs:89`.

**Concession.** One universal leaf should be the default implementation experiment. I do not require every state to be projected into boundary-canonical form after each microstep. `StepInv` is the correct internal contract. Full B-chain requalification should not block unrelated GP implementation. Sources: `security-candidate-b.md:A4` and `qa/sherlock/predicates.yaml`.

## fable-2

**Agreement.** Your seven-lane statement matches the consumer. Retain stock proof primitives and the terminal pairing. Measure U/F and S before selecting production values. Re-capture the contract transcript after the toolchain update. Sources: `fable-2/position.md:K2–K3,I2–I3,I8`, `crates/mori-recursion/src/ivc.rs:191` and `run_decider.rs:247`.

**Disagreement 1: fresh malicious witnesses are essential.** Mutating a public input beside an existing proof tests proof binding. It does not establish that the relation rejects a semantically wrong witness. Require fresh proof attempts under the same authorized key with false outcome, wrong result, bad count and coordinated carrier mutations. The C6 adapter currently pins externally supplied outcome constants rather than deriving them from carrier fields. Sources: `crates/mori-backend/src/sherlock_c6_child.rs:560` and `:598`. Outcome-dependent relation constants also need reconciliation with the one-key-across-programs objective.

**Disagreement 2: exact addition needs bounds.** `steps >= 1` over intended integers is stronger than `steps != 0` in a field. The current IVC implements the latter and has no local range constraint on the sum. Require the bounded-integer obligation in the node or an explicitly proved composed premise. Sources: `crates/mori-recursion/src/ivc.rs:308` and UISEC-006.

**Disagreement 3: result encoding is not a language restriction.** The readback convenience of a nonzero low byte must not become a correctness condition on UInt128 results. Include 0, 256 and boundary values in ledger readback qualification. Use the canonical codec to recover the value, with an explicit encoding contract. Sources: `tools/sherlock-sdk/sherlock-run.compact:18` and UISEC-005.

**Disagreement 4: recovery must preserve live obligations.** Redeployment alone is not rollback for an active financial contract. Separate empty test-deployment replacement from migration of live state, consumed resources and duties. Also retain MC03 requirements when replacing its old route. Sources: ZR10/ZR15 and `openspec/changes/mc03-native-recursive-proof/design.md`.

**Concession.** The A102/A80 endpoint and B rollup are useful regressions, but neither is the GP implementation itself. My milestone order should not force general C6 completion before requalifying a sound, separately named fixed-relation endpoint. Sources: `crates/mori-recursion/src/parent.rs:583` and `qa/sherlock/predicates.yaml`.

## fable-3

**Agreement.** Use the actual factory CLI. Distinguish mutating operations from observational checks. Add executable traceability and target-identity validation. Preserve durable public artifacts and the evaluator's holdout ownership. Sources: `fable-3/position.md:I2,I5–I8`, `models/doctorwatson/factory/CONTRACT.md` and `scripts/qa/check-a8.sh`.

**Disagreement 1: key incompatibility must be measured.** A new toolchain requires requalification and may require regenerated keys. It does not establish that every old key is invalid on ledger 9. Upstream explicitly retains older proof/key verification and identifies specific affected circuit classes. Compare actual relation, key, transcript and target identities. Source: `research/midnight-ledger9-20261107/api/midnightntwrk--midnight-ledger--releases.json`, release `ledger-9.1.0.0`.

**Disagreement 2: abandonment is not general rollback.** Your proposed rollback procedure is acceptable for an empty failed deployment. For live contracts it must preserve state, funds, consumed identifiers, duties and pending events. A maintenance authority is one migration mechanism, not a universal security requirement. Immutable deployments are explicitly allowed by ZR15. Source: `docs/MORIARTY-BACKEND-REQUIREMENTS.md:ZR15`.

**Disagreement 3: keep Mainnet design in scope.** The brief explicitly asks for an implementation and deployment workflow through Mainnet. Name exact additions and authorization boundaries without performing deployments now. A new T4 label is optional if existing commands can express the same gates. Older roadmap prose cannot remove the current council's requested design scope. Sources: `BRIEF.md:Phase 3` and `models/doctorwatson/factory/CONTRACT.md:CLI`.

**Disagreement 4: STOP should identify the stalled work.** A genuine F4/F11 fix may update the factory's recorded progress. Do not design a housekeeping pass merely to reset the timer while A8/U023 remain stalled. Record the responsible operation and changed hypothesis. Source: `AGENTS.md:The loop` and `models/doctorwatson/factory/CONTRACT.md:stop-rule`.

**Concession.** Preview is the relevant public rehearsal target for the released production tuple. Historical stagenet evidence remains useful for its exact runtime, not production compatibility. The scout's RPC record supports the distinction. Sources: `research/midnight-ledger9-20261107/api/preview-rpc-status.json` and `astra-3/observations.md`.

## astra-1

**Agreement.** I verified the mutation-accounting finding: synthesis errors and panics can receive rejection credit, while `assert_sound` does not require zero panics. I also verified A8's broad call selection and A10's newest-file selection. These require explicit early fixes. Sources: `crates/mori-backend/src/qa_mutation.rs:458`, `:640`, `crates/mori-backend/tests/soundness_mutation.rs:227`, `scripts/qa/check-a8.sh` and `check-a10.sh:7`.

**Qualification.** State soundness relative to the admitted invariant. A fragment need not reject every unreachable malformed pre-state if authenticated genesis and every predecessor establish and preserve `StepInv`. The composed acceptance theorem must discharge that premise. Your later `refinements.md` already agrees. Source: `openspec/changes/sherlock-universal-interpreter-design/security-candidate-b.md:A4`.

**Concession.** My initial position underemphasized evidence freshness and actual failure-stage accounting. I adopt your explicit outcomes and candidate manifest, implemented through existing factory records. Formal arguments, cryptographic evidence and finite tests should remain distinct rather than form one total strength ranking. Source: `astra-1/position.md:Assurance strategy` and UISEC-012.

## astra-3

**Agreement.** A8's missing call is deployment-specific. Dispatch's long preprocessing test is not a proving benchmark. Requalify the existing endpoint beside C6 work, preserve every reduced diagnostic's scope and use Preview for production rehearsal. Sources: `astra-3/position.md`, `scripts/qa/check-a8.sh` and `crates/mori-backend/src/sherlock_machine/c6_dispatch_consumer.rs:1`.

**Qualification.** “Test-cost problem until a trace shows otherwise” must not imply that constraint coverage is already established. Performance diagnosis can proceed independently, while the new leaf still needs invariant, binding and adversarial-witness obligations. Source: `host-evidence.md` and `recursion-evidence.md` in this folder.

**Concession.** I accept the two concurrent work fronts, with heavy operations serialized by the existing lock. Keep the current machine baseline and avoid requiring new VM or workflow infrastructure before concrete progress. Source: `models/doctorwatson/factory/CONTRACT.md:heavy`.

## Shared corrections and voting conditions

1. Keep every existing security obligation or map it explicitly to a successor requirement. Supersession must not erase scope.
2. Make leaf schema, `StepInv`, integer progress, final decider, key authorization and exact target binding acceptance requirements.
3. Classify hostile tests by the stage actually reached. Require semantically invalid coordinated witnesses, not only changed proof bytes.
4. Define recovery for live contracts separately from replacement of test deployments.
5. Keep ledger source pins, runtime observations and binary provenance distinct. The scout reports that the deployed runtime's exact source mapping is not publicly reproducible yet. Record that limit rather than claiming a local source build equals the deployed verifier. Source: `wiki/midnight-ledger-9-hardfork.md:Contradictions and gaps`.

I can ACCEPT a design that mandates these obligations and schedules their concrete implementation. I do not require their proof artifacts to exist during this council. I will OBJECT if the final design treats host validation, outer verification or a stale/unbound receipt as sufficient financial acceptance.
