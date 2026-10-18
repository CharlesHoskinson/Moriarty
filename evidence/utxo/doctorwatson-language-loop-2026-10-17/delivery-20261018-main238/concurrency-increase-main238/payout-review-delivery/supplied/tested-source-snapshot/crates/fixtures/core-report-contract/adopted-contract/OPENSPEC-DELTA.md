# PROPOSED OpenSpec delta: accepted normal Core/1 report observability

Status: **PROPOSED / UNADOPTED / UNRUN**. Candidate placement after adoption: a bounded reporting subsection in `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md`, referencing the proposed normal wire contract. The local NR labels below identify clauses in this proposal; they do not add a competing top-level roadmap, change existing requirement counts or score coverage.

## ADDED Requirements (proposal text)

### NR-01 — Complete accepted report and exact step identity

WHEN an ordinary Rust Core/1 action consisting of supported Let, Requires, Next, Ensures and Emit statements completes normally, the public execution report SHALL include exact semantics/action, complete pre/post snapshots, one Step per source statement in order, and complete ordered effects, including the zero-statement action boundary. Each Step SHALL include its zero-based numeric source statement index and exact admitted IR statement with numeric ExprIds referencing that Program's expression array.

#### Scenario: Normal five-statement Counter

WHEN the stated Counter run action starts at declared count UInt0 with input amount UInt7 and no balance/liability rows, the report SHALL retain indexes0..4 and statement expression roots2,5,6,9,10 against the stated eleven-node public expression table. Source-to-IR numbering is an explicit compared observation, not inferred from a matching final count.

#### Scenario: Normal empty action

WHEN an otherwise valid selected Core/1 action with an empty body completes normally, the report SHALL retain its semantics and action name, serialize `steps:[]`, set `post` equal to the complete normalized `pre`, and serialize `effects:[]`. No final Step snapshot is needed because there is no Step.

### NR-02 — Present typed result, including Let

WHEN any of the five scoped statement kinds completes as part of an accepted normal Core/1 action, its Step SHALL serialize a present outer value member containing the evaluated typed result. Let SHALL report the captured RHS value; Requires and Ensures SHALL report typed Bool true; Next SHALL report its RHS value before assignment; Emit SHALL report its emitted typed value. The outer Some is represented by the typed object, with UInt decimal strings and Bool JSON booleans. None/null or omitted value is not the selected representation for those normal completed kinds.

#### Scenario: Observable local capture

WHEN Counter evaluates `let total = count + amount` from count0 and amount7, Step0 SHALL contain statement.value ExprId2 and outer Step.value typed UInt7, while its complete snapshot remains S0 and its effect delta remains empty. `total` SHALL remain a local capture rather than a new state row.

### NR-03 — Complete post-statement snapshot at each position

WHILE an accepted normal report enumerates the scoped action's Steps, each Step.snapshot SHALL be the complete snapshot after that statement, retaining changed and preserved declared state, balances and liabilities in the existing canonical order. Next SHALL update state immediately for subsequent statements; earlier let captures SHALL retain their earlier captured values. Execution.pre SHALL be the complete normalized opening snapshot and Execution.post SHALL equal the last Step.snapshot for N>0; for N=0 it SHALL equal Execution.pre.

#### Scenario: Position-sensitive Next and Ensures

WHEN Counter reaches Next at index2, Step2 SHALL report the count0-to-count7 State effect and snapshot S7. Ensures at index3 SHALL read count7 and total7, report Bool true, retain S7 and add no effect. Emit at index4 SHALL retain S7 while reporting its event value.

### NR-04 — Per-statement effect deltas and cumulative execution effects

WHEN a normal accepted report is serialized, Step.effects SHALL contain only that statement's new ordered effects, and Execution.effects SHALL equal their ordered concatenation. The derived prefix after a Step SHALL be its preceding prefix followed by that Step's delta; no additional prefix field is required. Array order and repeated effects SHALL remain significant. For N=0 the ordered concatenation SHALL be the empty array, matching the defined prefix P[-1]=[].

#### Scenario: State update followed by event

WHEN Counter completes normally, Step2.effects SHALL be [State(count0->count7)], Step3.effects SHALL be [], and Step4.effects SHALL be [Emit(count,UInt7)]. Their derived prefixes SHALL respectively be [State], [State] and [State,Emit], and the final execution list SHALL be [State,Emit]. This scenario distinguishes a Step delta from a cumulative prefix.

### NR-05 — Accepted metered/ordinary execution agreement

WHERE a positive normal invocation selects the existing `moriarty-reference-logical-cost/1` profile with a sufficient local cap, its complete Execution SHALL equal the same normal invocation without metering. The metered public envelope SHALL add the profile and canonical cap/used strings without adding per-step work fields or changing Step result/snapshot/effect observations.

#### Scenario: Counter normal cost16

WHEN the stated Counter normally runs at cap16, the five independently specified increments4,4,2,4,2 SHALL total used16 and the envelope SHALL contain cap:"16", used:"16" and the unchanged complete normal Execution. This logical schedule measures reference primitive interpretation only.

### NR-06 — Public normal process/serialization contract

WHEN ordinary public `mori simulate` successfully prints either scoped normal accepted envelope, it SHALL return exit0 and print the one complete JSON report followed by the final newline, with no ordinary status text on stderr. The existing bounded pretty public exporter is retained; full semantic JSON equality and frozen public byte compatibility SHALL be checked separately. JSON object member order SHALL not be mistaken for execution order; statement/effect/snapshot-row arrays retain the order prescribed by the contract.

#### Scenario: Normal Transfer compatibility

WHEN the existing independently specified normal Transfer specimen is observed through the same public path, its full literal execution and existing accepted envelope bytes SHALL remain unchanged, including its Requires and Transfer Step values, all preserved observer/duty rows, ordered effect list and final newline. This compatibility observation does not expand the five-kind statement-result proposal to full financial-family acceptance.

## Minimal next independent acceptance predicates — none observed

1. Adopt or explicitly revise the Let.value and Step.delta/Execution.concatenation design clauses through the existing consequential decision reviews. Record candidate identity and retained dissent. Do not claim the inspected implementation chose the contract on behalf of users.
2. Have the separate custodian freeze a complete normal Counter input/IR/execution/output/work expectation from the adopted contract before its acceptance run. The custodian declares source/implementation exposure, arithmetic derivation and byte-encoding dependencies. The freeze includes every ExprId, exact statement, typed Step.value (especially Let), complete snapshot, step delta, derived prefix and total work16. It must not call/import the evaluator, parser or formatter under comparison to generate the expectation.
3. Compare the real public ordinary and positive metered Counter paths against that complete freeze: all fields, ordered arrays, process exit0, complete stdout/newline and empty ordinary stderr. An equality between two implementation wrappers alone does not establish this predicate. Exact byte comparison uses a separately frozen complete expected public string, not the candidate exporter as its oracle.
4. Freeze and compare the ordinary normal empty-action boundary separately from the five-step specimen: complete normalized pre retained as post, exact selected semantics/action, no Steps and empty effects. Preserve and compare the existing complete normal Transfer oracle through its actual public consumer, retaining all untouched balances/duties and existing small-report bytes. Preserve that oracle's historical provenance and the separate new Counter freeze; do not overwrite either with observed product output.
5. Record implementation, source inspection, actual local observation and acceptance separately, with exact candidate/profile/consumer/oracle identity, command, ExitCode, stdout/stderr identity and scope. All execution is subject to its existing authority; this source proposal authorizes none. Keep the original EARS/roadmap/target/financial conformance exits open.

No new test harness, framework, all-financial-statement expansion or target qualification is part of this delta. All existing U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24 and incorporated requirements remain retained. This document contains only normal-positive report clauses; other behavior is outside this proposal.
