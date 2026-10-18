# One resume invocation action — second increment, unexecuted

The previous six files remain byte-identical to `QUINT-EXECUTION-PINS.json`. Their historical draft comments remain untouched. Root's actual first-increment results are in `checks/quint-prefix-01..04`: both typechecks exited0; the one-transition TypeScript simulation reached its witness4/4 with two-state traces; the separate escrow/reserve tests passed2. This source task read the captured run/test stdout. Those results concern the first abstract increment only and provide no Rust/compiler correspondence.

The second increment adds `continuation_lifecycle.qnt` and a separate test file. It selectively imports only six **pure declarations/types/constructors** from the first module: Specimen, Escrow, Reserve, FinancialFrame, initialFrame, applyStart. No original state variable or action is imported. There is no candidate Moriarty code, artifact, evaluator or compiler import.

## State and one new transition

`PendingState` is the abstract financial/control record **c**. It carries complete balances/program state, duty presence/amount, site, captures and committed logical work. It contains no invocation-attempt counter or output effects. `Observation.last` records one invocation's attempted work/status/segment/effects/source indices; `attemptedTotal` is a separate outside-c running sum including the prefix. It is not a stored cap or an admission mechanism.

`resume(event)` is the only new business action. It uses pure `applyInvocation`, branching over three typed event constructors. Its finite normal event domains are Delivery(false/true), Approval(41/43/51) and Settlement(false/true), selected by specimen/site. Invalid or adversarial events are outside this domain; no security invariants or negative/adversarial tests are defined. `initCase` projects the already checked pure prefix calculation into c; it does not introduce another financial transition.

False observations are real invocations: `last` and `attemptedTotal` change, but the next c is the exact input record (`observe(o, o.c, ...)`). They emit no segment, effects or replacement continuation. The ordinary quote51 suffix rejection also retains that exact c with attempted6 and no segment. This is identity at the **model representation** level; the model has no byte serializer and makes no claim about Rust JSON byte identity.

| Frozen oracle invocation | Current result modeled | Attempted / committed after |
|---|---|---|
| Escrow Delivery(false) | Same site1/captures10/duty10, no segment |2 /5 |
| Escrow Delivery(true) | escrow10→0,bob0→10; duty present-zero; site0/captures[]; ordered transfer/discharge |6 /11 |
| Reserve Approval(41) | Same site1/held42; quote discarded, no segment |4 /11 |
| Reserve Approval(43 or51) | allocated42; reserve58,target42; duty5; basis78 or86; site2/captures[2,42,quote] |15 /26 |
| Reserve Settlement(false) | Same site2/c, no segment |2 /26 |
| Reserve Settlement(true), captured quote43 | Fee2 reserve→treasury then Emit43; duty5 remains; site0/captures[] |10 /36 |
| Reserve Settlement(true), captured quote51 | Commercial `requires quote<=50` rejection at original ordinal9; same c, no Fee/Emit or segment |6 /26 |

Nine tests use literal normal oracle boundary records and ordered effect/step expectations. They also cover escrow false→true attempted13/committed11, reserve false→true→false→true attempted42/committed36, and quote51 accepted→commercial refusal attempted32/committed26 (11+15+6). The finite simulations initialize at the first pending boundary and take at most four resume invocations; completed site0 has no enabled resume. There is no blanket stutter action. A rejected quote51 state may observe another ordinary invocation, but no new recovery mechanism is invented.

## Adapter additions and limits

The original `ADAPTER.md` numeric/asset/absence/identity limits still apply. Quint mathematical ints are not Rust UInt128 or field elements. No overflow, malformed boundary, authority, once-only consumption, authenticity or adversarial-witness relation is modeled. c at completed site0 is an abstract post boundary, **not an emitted Rust Continuation**; `newContinuation` is false there. Positive duty5 remains an ordinary residual financial fact, not settlement.

The prefix C projection drops its historical attempted/effect fields. Each resume replaces the invocation effects, never folds them into c. Effect identities/parties/assets are supplied by the frozen specimen context:

| Segment | Emitted source steps | Ordered effects → original source ordinal |
|---|---|---|
| Escrow resume true |[2,3,4] |TransferAmount→3;DutyValue→4. Await at2 has no effect. |
| Reserve approval true |[3,4,5,6,7] |StateValue allocated→4;TransferAmount→5;DutyValue→6;StateValue basis→7. Await at3 has no effect. |
| Reserve settlement true |[8,9,10,11] |FeeAmount→10;EmitQuote→11. Await8/Requires9 have no effect. |
| False observations |[] |No emitted segment/effects. |
| Commercial suffix rejection |[] |No emitted segment; errorSteps[9] maps to `mori::core::requires`. |

Model transition positions never replace original source ordinals. No source statement interleaving, immutable Rust wire encoding, SDK constraints, ledger phase or compiler preservation is claimed. All event/capture values in this increment are normal frozen cases. No security judgment is supplied.

## Proposed exact checks

`PROPOSED-LIFECYCLE-CHECKS.json` records exact argv/cwd; `LIFECYCLE-INPUTS.json` pins the new files and preserved original six. The same locally discovered Node/Quint JS entrypoints are used, with explicit TypeScript evaluation. Execution remains subject to fresh Opus classification/root dispatch. No command below was executed by this source task. Stop at the first failure; do not add another model action before actual typecheck **and** run succeed.

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js typecheck continuation_lifecycle.qnt
```

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js run continuation_lifecycle.qnt --main continuation_lifecycle --backend=typescript --max-steps=4 --max-samples=32 --seed=20261016 --witnesses resumeObserved falseObservationReached completionReached businessRejectionReached --verbosity=2
```

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js typecheck continuation_lifecycle_test.qnt
```

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js test continuation_lifecycle_test.qnt --main continuation_lifecycle_test --backend=typescript --match '^(escrowFalseTest|escrowTrueTest|escrowFalseThenTrueTest|reserveFalseApprovalTest|reserveTrueApprovalTest|reserveFalseSettlementTest|reserveTrueSettlementTest|reserveFalseThenTrueTrajectoryTest|reserveBusinessRejectionTest)$' --max-samples=1 --seed=20261016 --verbosity=2
```

The four positive witnesses are reachability targets only; sampling may miss a target. A later run must report actual witness counts/trace lengths and the selected test count without interpreting sampling as exhaustive or as Rust correspondence. There is no Quint verify/Rust backend/native command proposal. Original source/command/pin files and product repository source are unchanged by this task. All U/M/MC/SP/G exits remain open.
