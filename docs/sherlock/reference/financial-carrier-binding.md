# Financial carrier source boundary

The publicly nameable `mori_core::sherlock::helper_machine::shared::fin_carrier` component provides checked source bindings and financial snapshot carriers. It resolves strict source names, distinguishes state rows from name IDs, checks plan arithmetic and action identity, and preserves typed state, asset, balance and liability rows through Core snapshot conversion. Mutation helpers validate the source/snapshot binding and return typed errors for invalid state, transfer and liability updates. The public staged-source producer below is wired; financial owner admission, genesis and execution are not.

`FinBoundSource`, `FinAdmittedExtras` and `FinSnapshot` keep their construction fields private. Their identity is structural over the producing names, states, assets, actions and capacities; it is not a program digest, authentication result, complete ProgramView validator or public finance admission. `apply_set_state`, `apply_transfer` and `apply_liability_add` retain their literal reviewed checks and atomic refusal behavior.

At the earlier carrier-only cut, two fresh independent Muse source/design/security/API opinions approved this sealed prerequisite, and Root adopted only that source prerequisite. Sixteen source-exposed carrier tests passed in that historical qualification. The normal Core build passed with 116 exact dependency inputs; the real full cfg(test) build passed with 186. The test binary listed 409 tests, listed the exact sixteen carrier tests, then ran only those sixteen: 16 passed, 0 failed and 393 filtered out. These observed results are separate from the source opinions and do not establish financial owner-step, admission, genesis or public finance behavior. The normal and test builds retained 50 and 29 warnings respectively; full diagnostics remain in the result archive.

The direct indexes require their proven bounds and sealed production construction; the owner's old prose saying there are no direct indexes is inaccurate. The tests do not separately assert from-Core liability overcapacity; some refusal preservation checks are grouped, and a fixed fixture helper ignores its `rows` argument. These residuals remain disclosed without changing the approved source.

Full financial lowering remains required: all twenty XFin transitions, financial scalar entry, recorded source/ref/env provenance, selection and frame invariants, public invocation, genesis, stored admission capacities, continuations, canonical wire/digests, independent schedules and complete financial outcomes. This component does not close the fourteen whole-finance findings or all twelve original counterexample predicates. Native constraints, proof, target acceptance and EARS closure remain open.

The linked source archive records exact source and independent review provenance. It contains no compiled artifact or private oracle data. These project review records are not developer compile, prove or deployment permissions.

See [source qualifications](../../results/sherlock-financial-carrier-unit02-source-20261020/SOURCE-QUALIFICATION.json), [review A](../../results/sherlock-financial-carrier-unit02-source-20261020/A-FINAL-RESPONSE.txt) and [review B](../../results/sherlock-financial-carrier-unit02-source-20261020/B-FINAL-RESPONSE.txt).


## Interpret the observed carrier result at its producing boundary

Use the [actual five-operation result](../../results/sherlock-financial-carrier-unit02-source-20261020/actual/ROOT-RESULT.json) when checking execution, and the source qualifications above when checking the component contract. A listed test is not an executed test: the qualification ran16 carrier tests from409 listed members and left393 filtered out. It built the real normal and cfg(test) Core cuts; it did not execute a financial owner invocation.

`FinAdmittedExtras` and `FinSnapshot` are now publicly nameable checked carriers. Neither their names nor their tests establish admission or attach financial state to the actual admitted owner image. Keep public source/typed input, owner admission, financial genesis, dispatch/apply, recorded effects, continuations and ledger settlement as separate links. The next owner consumer must supply those links rather than presenting the existing carrier test as the complete path. Full20XFin, all14findings and native/target/EARS duties remain open.


## Public staged source preparation — observed October 20

`shared::owner_host::prepare_fin_staged_source` accepts source text, `&LayoutSchemaC` and `&FinCapacitiesC`; `prepare_fin_staged_program` accepts an already sealed `stage::StagedProgram`. Both functions and the necessary carrier types are also re-exported through public `shared`. The real producing path is `stage::load` followed by `FinPreparedStagedSource::prepare`. Its fields remain private; source/plan/action/input/statement/expression accessors are read-only. The complete staged source, stage plan and forwarded hashes are retained, while its single `FinPlan` is derived from the stage plan. No caller-authored replacement plan enters the prepared wrapper.

The `/2` binder interns distinct spellings once. A legal state/action or asset/action shared spelling can share `NameId`; typed state/asset rows and ordered action bindings distinguish roles. Program/state, program/asset and program/action equality are also supported. Genuine same-kind/state-asset collisions remain stage errors. Binding checks name rows, then name bytes, then state and asset capacities; stage parse/profile/type/validation failures precede binding.

[The actual eight-operation result](../../results/sherlock-financial-staged-source-producer-20261020/README.md) records normal Core117 dependencies, full real cfg(test)188,422 listed tests,9 new internal passes/413filtered and2 genuine external public-API passes/0filtered. The old16 tests were not replayed. Compiler warnings24/27 and complete diagnostics are retained; zero compiler errors were observed. These results establish this finite source-preparation/API increment, not financial execution or a whole-suite pass.

Public visibility also exposes checked standalone carrier helpers. A caller can construct a standalone checked `FinAdmittedExtras`; it must never be treated as admitted financial state. `same_source` compares retained source/plan/hashes/extras, including financial capacities, but ignores sufficient layout thresholds that are not retained. It is not complete preparation, target, configuration, admission or cryptographic identity. Static `let`/name linkage is not recorded runtime environment provenance. The full-finance and native/EARS gaps above remain open.

See [prepare staged financial source](../how-to/prepare-staged-financial-source.md). Historical source comments and raw preexecution UNRUN labels are retained verbatim and qualified in the new result archive; current visibility and observed11 test results are stated separately.
