## Execution and verification

These unchecked steps refine the [single implementation register](../consolidated-language-kernel/tasks.md). The [mapping](design.md#report-task-mapping) gives report dependencies. The report gives II.1, II.5 and II.6 no product prerequisites. Their factory registrations use the case selector from step 2.2. Step 16.1 reads all in-scope results. Each section names one accountable role unless a step names another. All paths are repository-relative.

Use `scripts/sherlock-next` when the factory selects implementation work. Use the [current contract](../../../models/doctorwatson/factory/CONTRACT.md). Run heavy work through `scripts/doctorwatson heavy --id ID -- COMMAND`. Read the trace after failure. Retry with `--hypothesis TEXT`. Shrink the case after two similar failures. Check the selected predicate before the inner-loop code or test commit. Run `scripts/doctorwatson stop-rule` afterward. Bring the trace and hypothesis to the owner on STOP. T3 promotion uses Preview and its one-hour cold review. Keep these tools outside the public package.

**Command conventions.** The npm scripts below exist in the current manifest. The factory interfaces exist. All report-specific runners, test commands and predicate invocations below are **planned**. Use the registered ID for `ID`. Only a registration step assigns it. Do not pass a report ID directly to DoctorWatson unless registered under that ID.

- Each section's **runner** is the exact planned command from the root. For a heavy run before registration, use the measured wrapper above with that command.
- Each **register** instruction adds the section's runner to `qa/sherlock/predicates.yaml`. Record its ID in the section receipt. Declare tier, dependencies, inputs and budget. Set `heavy: true` for heavy checks. Verify the definition with `scripts/doctorwatson validate`. Run `scripts/doctorwatson check ID` to verify the result. Use the report dependency graph for the registered checks. The I.2 baseline stays runnable before later checks exist. The factory dispatches heavy rows through its measured path.
- Each test-first step freezes independent inputs and expected effects in its named test or fixture. Run the section's test command before changing behavior. Retain the failing assertion, or an existing pass for a delivered child. A missing file, tool or credential does not demonstrate a behavioral rejection.
- A **receipt** is the exact JSON output named in the section. Record the test and runner commands, revision, environment and outcomes there. For native cases, include transaction identities, authority, expected and accepted state, fee effects and limitations. Redact private inputs. Retain enough public data to replay each check.

## 1. I.1 Reproduce the assessed baseline

Accountable role — language maintainer. Existing commands from the root:

```sh
npm --prefix experiments/moriarty-language run demo
npm --prefix experiments/moriarty-language run loan-lifecycle-demo
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-language run build
```

- [x] 1.1 Preserve the [original log](https://github.com/input-output-hk/SOTA-DeFiFormal-Moriarty/blob/1ced5fe1e0a83d665184bb496bea01bcb9940449/moriarty/competitors/moriarty/evidence/maturity-036b16efcce7496a/reproduction-2.log) at `deliverables/competitor-report/I.1/original.log`. Run the documented demos and suite from a complete clean checkout of the assessed SHA. Write `deliverables/competitor-report/I.1/baseline.json`. Pass when it binds the original digest, all 21 failing names, tool versions, dependency inputs, commands and actual exits.
- [x] 1.2 Reproduce the failing public import in `experiments/moriarty-language/tests/authority-fee-binding.test.mjs`. Check with `node --test experiments/moriarty-language/tests/authority-fee-binding.test.mjs`. Repair the Source/6 import/export contract in `experiments/moriarty-language/src/successor/financial-agreement-source-v6.ts` and `experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts`. Pass when the public evaluator runs the original fee and net-goal assertions.
- [x] 1.3 Repair the documented setup in `experiments/moriarty-language/README.md`. Rerun the commands above on the clean delivery revision with the required fixtures and compiler/runtime dependencies. Write `deliverables/competitor-report/I.1/result.json`. Pass when both demos and the intended suite succeed. Link each earlier failure to a verified resolution or a justified scope exclusion. Keep unresolved failures blocking.

## 2. I.2 Automate the baseline after I.1

Accountable role — factory maintainer. Planned runner: `python3 scripts/qa/order-report.py I.2`. Test: `python3 -m unittest discover -s scripts/qa/tests -p test_order_report.py`.

- [x] 2.1 Write `scripts/qa/tests/test_order_report.py`. Freeze expectations for missing coverage, omitted suites, false success and a deliberate source, kernel or ledger regression. Pass when the harness detects the seeded defect and records the affected check.
- [x] 2.2 Implement `scripts/qa/order-report.py` with a case selector. Register I.2. Write `deliverables/competitor-report/I.2/result.json`. Pass when build, tests and both demos repeat on the named revision. Include scoped coverage for source, kernel and ledger files with metric, denominator and exclusions. Retain clean and deliberately failing runs. Add later behaviors to this coverage as their runners land.

## 3. II.1 Connect signed orders to native acceptance

Accountable role — ledger maintainer. Planned runner: `python3 scripts/qa/order-report.py II.1`. Test: `node --test packages/moriarty-orders/test/acceptance.test.mjs`.

- [ ] 3.1 Write `packages/moriarty-orders/test/acceptance.test.mjs` and `packages/moriarty-orders/test/fixtures/acceptance.json`. Freeze signed-field, current-state and complete funded-effect expectations. Pass when the tests distinguish a permitted transition from forged owner, domain substitution, stale state and duplicate use.
- [ ] 3.2 Implement signed bindings in `packages/moriarty-orders/src/authorization.mjs`. Bind key/account, order, assets, amounts, recipient, program and chain. Pass when each changed binding fails independently.
- [ ] 3.3 Implement authenticated state access in `packages/moriarty-orders/src/accepted-state.mjs`. Pass when a stale predecessor fails despite a caller's freshness assertion. Two requests against one consumed authority cannot both succeed.
- [ ] 3.4 Implement native atomic consumption in `packages/moriarty-orders/src/acceptance.mjs` and `packages/moriarty-orders/contracts/acceptance.compact`. Document trust and upgrades in `packages/moriarty-orders/TRUST.md`. Bind the generated kernel and ZKIRv3 verifier identity. Implement the runner case in `scripts/qa/order-report.py`. Register II.1. Write `deliverables/competitor-report/II.1/result.json`. Pass when the native cases reconcile funding, state, authority and all order effects.

## 4. II.2 Fund canonical O1 after II.1

Accountable role — order-library maintainer. Planned runner: `python3 scripts/qa/order-report.py II.2`. Test: `node --test packages/moriarty-orders/test/o1.test.mjs`.

- [ ] 4.1 Write `packages/moriarty-orders/test/o1.test.mjs` and `packages/moriarty-orders/test/fixtures/o1.json`. Freeze independent account and asset deltas for both permitted outputs and every II.2 rejection. Pass when partial input or an altered recipient fails even with sufficient aggregate output.
- [ ] 4.2 Deliver `packages/moriarty-orders/orders/o1.mori` through the II.1 adapter. Extend `scripts/qa/order-report.py`. Register II.2. Write `deliverables/competitor-report/II.2/result.json`. Pass when one frozen implementation executes the full spec matrix against funded custody. Retain the independent expectations, complete ledger reconciliation and assumptions for coverage review.

## 5. II.3 Support cancellation and recovery after II.1

Accountable role — custody maintainer. Planned runner: `python3 scripts/qa/order-report.py II.3`. Test: `node --test packages/moriarty-orders/test/recovery.test.mjs`.

- [ ] 5.1 Write `packages/moriarty-orders/test/recovery.test.mjs`. Separate unfilled, partially filled and expired cases. Include both fill/cancel orderings, two orders, extra deposits, fresh funding after revocation and recovery beyond the execution horizon. Pass when the fixtures distinguish each owner's recoverable balance and preserved transfers.
- [ ] 5.2 Implement `packages/moriarty-orders/contracts/recovery.compact`. Document surplus and upgrade rules in `packages/moriarty-orders/TRUST.md`. Extend `scripts/qa/order-report.py`. Register II.3. Write `deliverables/competitor-report/II.3/result.json`. Pass when accepted-state checks preserve completed transfers and prove spendable refunds in all permitted cases. Use a partial-fill policy for partial recovery. Preserve O1's one-fill rule.

## 6. II.4 Serialize continuations after II.1

Accountable role — continuation maintainer. Planned runner: `python3 scripts/qa/order-report.py II.4`. Test: `node --test packages/moriarty-orders/test/continuation.test.mjs`.

- [ ] 6.1 Write `packages/moriarty-orders/test/continuation.test.mjs`. Freeze rejected replay, substituted custody, changed event and edited-cap cases. Include both resume/refund orders with process termination and restart. Pass when the expected ledger admits only one terminal action.
- [ ] 6.2 Implement continuation identity in `packages/moriarty-orders/src/continuation.mjs`. Pass when any changed predecessor, event, custody or resource cap fails verification.
- [ ] 6.3 Persist consumed state in `packages/moriarty-orders/contracts/continuation.compact`. Document storage trust in `packages/moriarty-orders/TRUST.md`. Pass when a restarted client reads the accepted consumption and cannot spend again.
- [ ] 6.4 Bind resume and refund to that state in `packages/moriarty-orders/src/continuation.mjs`. Extend `scripts/qa/order-report.py`. Register II.4. Write `deliverables/competitor-report/II.4/result.json`. Pass when each funded race produces one accepted terminal outcome across restart.

## 7. II.5 Authenticate named observations

Accountable role — oracle maintainer. Planned runner: `python3 scripts/qa/order-report.py II.5`. Test: `node --test packages/moriarty-orders/test/feed.test.mjs`.

- [ ] 7.1 Write `packages/moriarty-orders/test/feed.test.mjs`. Cover valid updates, substitution, timestamp bounds, equality, outage and a changed condition. Pass when a caller's authenticity flag cannot satisfy the positive case.
- [ ] 7.2 Implement `packages/moriarty-orders/contracts/feed.compact` and `packages/moriarty-orders/src/feed.mjs`. Record controller, update policy, time source and outage handling in `packages/moriarty-orders/TRUST.md`. Extend `scripts/qa/order-report.py`. Register II.5. Write `deliverables/competitor-report/II.5/result.json`. Pass when the adapter rechecks the authenticated observation at execution and all boundary cases match. Preserve the economic correctness assumption.

## 8. II.6 Check financial invariants

Accountable role — formal methods maintainer. Planned runner: `python3 scripts/qa/order-report.py II.6`. Test: `node --test packages/moriarty-orders/test/invariants.test.mjs`.

- [x] 8.1 Freeze statements and independent boundary cases in `packages/moriarty-orders/test/invariants.test.mjs` and `deliverables/competitor-report/II.6/obligations.md`. Bind each to the bounded-atomic profile, generated kernel and ledger premises. Pass when conservation, cumulative budget and owner authority have named domains, assumptions and failure cases.
- [x] 8.2 Supply checked results in `clean/fork/Clean/Sherlock/OrderInvariants.lean`. Extend `scripts/qa/order-report.py`. Register II.6. Write `deliverables/competitor-report/II.6/result.json`. Pass when an independent checker can rerun the named results and inspect their premises. Retain exact analysis commands separately from finite-test commands. List open compiler, custody and acceptance correspondence obligations.

## 9. III.1 Measure complete operations after II.2 and II.3

Accountable role — measurement maintainer. Planned runner: `python3 scripts/qa/order-report.py III.1`. Test: `node --test packages/moriarty-orders/test/measurement.test.mjs`.

- [ ] 9.1 Write `packages/moriarty-orders/test/measurement.test.mjs` and `deliverables/competitor-report/III.1/workloads.json`. Pin both profile workloads, transaction inventory, fee scenario, native resource fields and completion checks before measurement. Pass when omitted setup, recovery, fees, fields or finality invalidate a sample.
- [ ] 9.2 Implement `scripts/qa/measure-order-lifecycle.mjs`. Extend `scripts/qa/order-report.py`. Register III.1 as heavy. Write `deliverables/competitor-report/III.1/result.json`. Pass when serial samples reconcile every required transaction for both operations and reproduce the phase sums. Retain raw receipts, setup, finality and exclusions. Emit the pinned assessment workflow input at `deliverables/competitor-report/III.1/assessment-input.json`. Retain both O10 observations as unavailable with reasons. Supply the frozen inputs and raw receipts for independent review.

## 10. III.5 Exercise executor fallback after II.2 and II.3

Accountable role — application maintainer. Planned runner: `python3 scripts/qa/order-report.py III.5`. Test: `node --test packages/moriarty-orders/test/executors.test.mjs`.

- [ ] 10.1 Write `packages/moriarty-orders/test/executors.test.mjs`. Exercise two distinct client authorizations, an unauthorized client and stopped operators. Pass when the cases distinguish separate grants from two sessions sharing one credential.
- [ ] 10.2 Implement `packages/moriarty-orders/src/executors.mjs`. Document selection, fees, exclusivity, censorship, open participation and fallback in `packages/moriarty-orders/EXECUTORS.md`. Extend `scripts/qa/order-report.py`. Register III.5. Write `deliverables/competitor-report/III.5/result.json`. Pass when funded fill, fallback, policy refusal and owner recovery match ledger readback. Exercise public language use without service enrollment.

## 11. III.6 Release and replay after I.2, II.1, II.2 and II.3

Accountable role — release maintainer. Planned runner: `python3 scripts/qa/order-report.py III.6`. Test: `node --test packages/moriarty-orders/test/distribution.test.mjs`.

- [ ] 11.1 Write `packages/moriarty-orders/test/distribution.test.mjs`. Test fresh installation outside the checkout with only released inputs. Pass when an omitted dependency or missing lifecycle command fails the distribution check.
- [ ] 11.2 Publish the versioned O1 lifecycle and replay package from `packages/moriarty-orders/package.json`. Deliver public commands in `packages/moriarty-orders/README.md`. Package public replay inputs and expected readbacks in `packages/moriarty-orders/replay/manifest.json`. Record the published locations and digests in `deliverables/competitor-report/III.6/release.json`. Pass when immutable release and package identities resolve to the recorded revision, bytes, operations, trust assumptions and gaps.
- [ ] 11.3 Replay the released instructions as a maintainer on a clean setup. Extend `scripts/qa/order-report.py`. Register III.6. Write `deliverables/competitor-report/III.6/result.json`. Pass when the clean replay reproduces the released ledger effects. Retain revision, package identity, commands, environment, setup time, receipts and outcome. Preserve failed attempts. Mark outside reproduction deferred.

## 12. IV.2 Fund O5 after II.2 and II.5

Accountable role — order-library maintainer. Planned runner: `python3 scripts/qa/order-report.py IV.2`. Test: `node --test packages/moriarty-orders/test/o5.test.mjs`.

- [ ] 12.1 Write `packages/moriarty-orders/test/o5.test.mjs`. Freeze permitted delivery and all IV.2 rejection cases. Pass when an earlier true condition cannot authorize a later false one.
- [ ] 12.2 Deliver `packages/moriarty-orders/orders/o5.mori` through funded custody and the II.5 adapter. Extend `scripts/qa/order-report.py`. Register IV.2. Write `deliverables/competitor-report/IV.2/result.json`. Pass when accepted ledger effects match every case at execution. Retain the actual feed, time and inclusion assumptions for independent review.

## 13. IV.3 Fund O8 after II.1 and II.2

Accountable role — ledger maintainer. Planned runner: `python3 scripts/qa/order-report.py IV.3`. Test: `node --test packages/moriarty-orders/test/o8.test.mjs`.

- [ ] 13.1 Write `packages/moriarty-orders/test/o8.test.mjs`. Freeze all three owner grants and the complete asset matrix. Pass when altered or missing authority and an unfunded third leg require zero order transfers.
- [ ] 13.2 Deliver `packages/moriarty-orders/orders/o8.mori` and `packages/moriarty-orders/contracts/atomic-exchange.compact`. Extend `scripts/qa/order-report.py`. Register IV.3. Write `deliverables/competitor-report/IV.3/result.json`. Pass when all three separately authorized legs settle atomically and each failure preserves order balances. Record custody funding, separate fee funding, resource bounds and multiple-payer limitations. A backend refusal leaves delivery open.

## 14. IV.4 Fund O9 after II.3 and II.4

Accountable role — custody maintainer. Planned runner: `python3 scripts/qa/order-report.py IV.4`. Test: `node --test packages/moriarty-orders/test/o9.test.mjs`.

- [ ] 14.1 Write `packages/moriarty-orders/test/o9.test.mjs`. Freeze first-trade balances, second-stage conditions, both race outcomes and restart cases. Pass when neither terminal action can reverse the first trade.
- [ ] 14.2 Deliver `packages/moriarty-orders/orders/o9.mori` and `packages/moriarty-orders/contracts/staged-exchange.compact`. Extend `scripts/qa/order-report.py`. Register IV.4. Write `deliverables/competitor-report/IV.4/result.json`. Pass when the native ledger records exactly one continuation or refund per trial. Retain a spend of the owner's recovered B.

## 15. IV.6 Decide O10 after II.2 and II.3

Accountable role — integration maintainer. Planned check: `python3 scripts/qa/order-report.py IV.6`.

- [ ] 15.1 Write `deliverables/competitor-report/IV.6/decision.md`. Add its decision check to `scripts/qa/order-report.py`. Pass when the cited integration evidence justifies deferral or requires a validated two-chain design. For a design, record proof, custody, finality, reorganization, timeout and late-release/refund cases. Keep O10 coverage and cost open pending implementation, native execution and independent assessment.

## 16. Reconcile delivery and assessment

Accountable role — evidence maintainer. Planned check: `python3 scripts/qa/order-report.py reconcile`.

- [ ] 16.1 Reconcile the retained records in `deliverables/competitor-report/index.json`. Add the index check to `scripts/qa/order-report.py`. List the evidence supplied for independent assessment, including the O1, cost and O5 packages. Pass when all 16 report IDs resolve to their scoped outcomes, evidence identities, limitations and assessment status. Record actual independent findings or a pending review for each new assessed claim. Keep pending claims open. Record III.6 as partial and IV.5 as deferred. External reproduction and a partner pilot are outside the closure checks.

IV.5 has no implementation or preparation step. Its original dependencies, funded success and failure cases, and downstream evidence obligations remain in the [design mapping](design.md#report-task-mapping).
