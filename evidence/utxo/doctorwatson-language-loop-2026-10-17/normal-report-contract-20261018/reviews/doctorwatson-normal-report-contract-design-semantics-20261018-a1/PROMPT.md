Fresh independent normalpositive DESIGN reviewer semantics, Gemini4ArgonHIGH. Return complete6fieldreview token doctorwatson-normal-report-contract-design-semantics-20261018-a1-whole-review, verdict approve-design-for-source-preparation / changes-required / abstain. Do not view otherreviews. No tools/commands/Mori/evaluator/parser/compiler/model/SDK/private/config/caches/wallet/keys/network. Primary3wholeproposaldocs and independentlyfrozenfacts/derivation INLINE. This is not security/rejection/hostile/resource/admissionwork. No actualexecutionorsemanticcertificationapproved. Need substantiveindependentvote on NORMALpublicwirecontract, not rubberstampcurrentimplementation.

Existingindependent custodian derivedfullCounterstate/prefixes/ExprIds/effects/work16 but no independentpublicLet Step.valuecontract: exactunknown /execution/steps/0/value. SourceauthorSolthen proposedpresenttypedLetRHS toexposecapturedvalue, sameaspresentimplementation (source-inspected/honestexposure, NOTblindedoracle). Step.effects is perstatementdelta whileExecution.effects orderedconcatenation, not cumulativeStepfields. NormalTransfergolden finalsoleeffectcannotdistinguishdelta vs cumulative. Decidewhethertheseobservableguarantees sensiblepubliccontract, retaincompatibility andallfinancial/accepted semantics, no newruntime/securitypolicy. NR01-06EARS proposal childofexistingUNI18/20/21/24, notnewtoproadmap. Rootrequestedemptyactionclarification nowexplicitN0 steps[],post=pre,effects[],nosyntheticStep; finalsnapshotruleN>0. Evaluate completeness/wiretypes/expressionIDs/snapshotorder/normalacceptedvalues/position-sensitiveensures/letcapture/meteredagreement with allpreservedstate, emptyboundary and exactbytevsJSONseparation.

If approving thisconsequentialdesign, allowonlynormativedocs/sourcepreparation and independentcustodian fullpositiveexpectedCounter/emptyactiongoldens freeze fromADOPTEDcontract beforeanynewconsumerexecution. Neverreplace oldindependentfreeze/originalfailures/gaps withactualoutputs. Authorimplementationexposure declared; desiredcontractnotestablished merelybecausecodecurrentlyreturnsSome. Custodiancanadoptexplicitclauseasnewnormativespec but mustdeclarepremisesnotretroactivepreimplementationgolden. ActualpublicCLI/IR/fullfields/allprefixes/bytes/exit/workresult remainsseparatelyrequired. No originalU/M/MC/SP/G/financial/composition/proof/targetexitclosed or nativeexecutionauthority; sourceproposaldoesnotclaim languagecomplete. filesReadonly INLINE label honestly;don't invent unrelatedAPI/claims.

### proposal/PROPOSED-CONTRACT.md
# PROPOSED normal Core/1 execution-report wire contract

Status: **PROPOSED, UNADOPTED, SOURCE-ONLY, UNRUN**. This text supplies a concrete design choice for review. Existing serialization code is an observation, not the independent normative authority for this proposal. No EARS clause, implementation result or parent acceptance exit is closed.

The narrow decision is to make the evaluated result of an accepted `Let` observable as a present typed `Step.value`. This matches source-inspected code, but must be adopted as a public contract before an independent custodian treats that field as a normative expectation. The proposal author has inspected implementation source and does not claim to be that blinded custodian.

## Scope and public envelope

The normative clauses below apply to accepted, ordinary finite Rust `language "moriarty/1"` / `moriarty-core/1` execution reports for `Let`, `Requires`, `Next`, `Ensures` and `Emit`. They describe reporting of the selected action's semantics; they do not redefine the source type system, expression arithmetic, other financial statements, staged/Core2 reports or target compilation.

For a successfully printed ordinary public `mori simulate` result, the envelope SHALL contain exactly `status:"accepted"`, `qualification:"local-reference-execution"` and `execution`. `execution` SHALL contain `semantics`, `action`, `pre`, `post`, `steps` and `effects`, with semantics exactly `moriarty-core/1` and action the selected action name.

An accepted metered invocation SHALL preserve that complete `execution` and qualification, adding only the existing logical-cost `profile` and `logical:{cap,used}` envelope fields. `cap` and `used` are canonical decimal strings. This proposal introduces no per-step work field. For successfully printed normal results the process result SHALL be exit0; ordinary simulate has no status text on stderr. Existing pretty JSON/export behavior and one final newline remain compatibility predicates; no new serializer is proposed. JSON object member order is not execution semantics. Exact existing public bytes must be frozen and checked separately rather than inferred from a library's field declaration order.

## Step identity, typed values and snapshots

For an accepted action with N statements, where N is any nonnegative count, `steps` SHALL have N entries in source statement order. Entry i SHALL have numeric zero-based `index:i` and an exact serialized copy of statement i from the admitted public IR. An expression reference in `statement` is the numeric zero-based index in that same Program's `expressions` array. It is not the evaluated value, a source offset, the step index or a new per-statement numbering. IDs belong to the exact source/IR candidate; this contract promises no numbering stability across source edits.

Each Step SHALL serialize all five members: `index`, `statement`, `value`, `snapshot`, `effects`. The outer `Step.value` SHALL be a typed value object for each accepted statement in this scope. A UInt is `{"type":"UInt","value":"7"}` with canonical decimal string units; a Bool is `{"type":"Bool","value":true}` or the corresponding false value. The outer Option representation is Some -> that object and None -> JSON null. **None/null and omission are not the chosen report representation for any of these five accepted statement kinds.** This is a serialization requirement, not a new input-decoder contract for other consumers.

`statement.value`, where present, is an ExprId JSON number; `Step.value` is a typed object. In particular, `Let{name:"total",value:2}` and `Step.value:{type:"UInt",value:"7"}` have different meanings and SHALL NOT be conflated.

Let S[-1] be `execution.pre`, and let S[i] be the state after statement i. Every `steps[i].snapshot` SHALL equal the complete S[i], not a before-image, field delta or local-variable environment. A Snapshot has exactly `state`, `balances`, `liabilities`; it has no `inputs` or local-let bindings. All affected and preserved rows SHALL remain present. Existing canonical snapshot ordering is retained: balances by exact `(asset,account)`, with asset components `(network,issuer,reference,scale)`, and liabilities by name. State is a map of declared state names to typed values. `execution.pre` is the complete normalized starting snapshot, including declared initial values when the supplied state map selects them; `execution.post` SHALL equal the final step snapshot when N>0. When N=0, `execution.steps` SHALL be `[]`, `execution.post` SHALL equal `execution.pre`, and `execution.effects` SHALL be `[]`. There is no Step record for an empty action; the per-Step typed-value rules have no instance to apply to. A captured `let` value is available to later statements but does not appear as a new state field.

## Exact accepted statement rules

Let Eval(e, i) denote the typed semantic value of expression e immediately before statement i, using the then-current state, action inputs and earlier let captures. This is mathematical notation in the specification, not a call to the implementation under review.

| Statement i | Required outer Step.value | Required S[i] | Required Step.effects (new effects of this statement only) |
|---|---|---|---|
| `Let{name,value:e}` | Eval(e,i), captured under name for later statements | S[i-1] | `[]` |
| `Requires{predicate:e}` | typed Bool true on accepted execution | S[i-1] | `[]` |
| `Next{state:x,value:e}` | after = Eval(e,i), before updating x | S[i-1] with x replaced by after; every other state/balance/liability preserved | exactly one `State{name:x,before:S[i-1].state[x],after}` |
| `Ensures{predicate:e}` | typed Bool true on accepted execution, at its written position | S[i-1] | `[]` |
| `Emit{name,value:e}` | v = Eval(e,i) | S[i-1] | exactly one `Emit{name,value:v}` |

Let evaluation/capture is observable through Step.value even though its snapshot/effects are unchanged. Next is an immediate update: subsequent expressions referring to x read after. An earlier let capture retains the value captured earlier. Emit records an event without modifying Snapshot.

**Step.effects is a statement-local delta.** Define a derived cumulative prefix P[-1]=[] and P[i]=P[i-1] followed by `steps[i].effects`, preserving array order and duplicate effects. The prefix is not an extra wire field. `execution.effects` SHALL equal P[N-1], the ordered concatenation of every step delta. At N=0 this is P[-1]=[], so the empty concatenation has the explicitly defined empty effect list. It SHALL NOT be reconstructed only from final state or substituted with a set. An unchanged-snapshot step can still add an Emit effect. This distinction is necessary even though the existing normal Transfer example's sole effect occurs in its final step.

## Normal empty-action boundary

This proposed normal report contract includes an otherwise valid Core/1 action with an empty body. The selected action still has its own action name and a complete normalized pre snapshot. The result SHALL retain `semantics:"moriarty-core/1"`, that selected action name, `steps:[]`, `post:pre` and `effects:[]` inside the normal accepted envelope. There is no invented final-step snapshot, statement index, ExprId or Step.value. This is a total definition of the N=0 boundary, not a new statement kind. Parser/validator/normal serialization source permits that shape; no empty-action invocation or expected JSON golden was produced here.

## Concrete normal Counter illustration

Use this existing normal public specimen with action input amount UInt7 and empty supplied state map, balances and liabilities:

```mori
language "moriarty/1";
agreement Counter {
  state count: UInt = 0;
  action run(amount: UInt) {
    let total = count + amount;
    requires total <= 100;
    next count = total;
    ensures count == total;
    emit "count"(count);
  }
}
```

The following is a manual design illustration, not a generated/frozen expected JSON artifact or an observed test result. The IDs are source-grounded against the public IR/parser convention; independent source-to-IR correspondence still requires its own expectation. The initializer is a StateDecl literal, not an expression node in the action expression array.

| ExprId | Public expression | Typed value at its enclosing statement |
|---|---|---|
| 0 | Name count in Let | UInt0 |
| 1 | Name amount | UInt7 |
| 2 | Add(0,1) | UInt7 |
| 3 | Name total in Requires | UInt7 |
| 4 | Literal UInt100 | UInt100 |
| 5 | Le(3,4) | Bool true |
| 6 | Name total in Next | UInt7 |
| 7 | Name count in Ensures | UInt7 |
| 8 | Name total in Ensures | UInt7 |
| 9 | Eq(7,8) | Bool true |
| 10 | Name count in Emit | UInt7 |

Let S0 be `{state:{count:UInt0},balances:[],liabilities:[]}` and S7 the same complete snapshot with count UInt7. Let C be `State{name:"count",before:UInt0,after:UInt7}`, and E be `Emit{name:"count",value:UInt7}`. UInt/Bool shorthand here expands to the typed objects defined above.

| index | Exact statement fields | Step.value | snapshot | Step.effects | derived prefix P[i] | proposed normal work increment |
|---|---|---|---|---|---|---|
| 0 | `kind:Let,name:total,value:2` | UInt7 | S0 | [] | [] | 1 statement + 3 expression nodes = 4 |
| 1 | `kind:Requires,predicate:5` | Bool true | S0 | [] | [] | 1 + 3 = 4 |
| 2 | `kind:Next,state:count,value:6` | UInt7 | S7 | [C] | [C] | 1 + 1 = 2 |
| 3 | `kind:Ensures,predicate:9` | Bool true | S7 | [] | [C] | 1 + 3 = 4 |
| 4 | `kind:Emit,name:count,value:10` | UInt7 | S7 | [E] | [C,E] | 1 + 1 = 2 |

Thus pre=S0, post=S7, execution.effects=[C,E]. Neither total nor amount is inserted into Snapshot.state. The five increments sum to logical used16 under the existing `moriarty-reference-logical-cost/1` schedule: one statement dispatch plus each uncached expression node, with a fresh cache per statement. A positive metered run at cap16 therefore has cap:"16", used:"16" if the implementation conforms. The contract does not add those increments to the ordinary unmetered wire. This arithmetic illustration is not native cost, fees or a run receipt.

## Consistency with the existing normal Transfer oracle

`crates/mori/tests/core_test_report.rs` records an existing literal normal Transfer expectation. It has Requires predicate2 with typed Bool true, pre snapshot and no new effects, followed by Transfer amount-expression3 with typed UInt25, full post snapshot and a one-element Transfer effect delta. Execution.effects is that same one-element list. This is consistent with the shared identity/snapshot/delta/concatenation rules above, while Transfer's own statement semantics are outside the five-kind normative table.

Its complete preserved observations remain: exact TEST asset `(local,independent-report-oracle,TEST,scale0)`, alice100/bob0 -> alice75/bob25, sixteen observer balances42, incoming_duty11, and unchanged empty state. Nothing here replaces or recomputes its recorded oracle. Its sole final-step effect cannot independently resolve delta-versus-prefix ambiguity, and it contains no Let, so it cannot settle the new Let.value clause.

## Adoption and evidence boundary

Adoption needs the existing consequential design reviews, followed by a separately frozen complete normal Counter oracle from a custodian using the adopted contract and independent semantic/arithmetic reasoning. The proposal's author/source inspection cannot be relabelled independent behavior evidence. No new golden was generated or frozen here. A public acceptance run, if separately authorized, must compare all fields and exact ExitCode/I/O against that freeze; matching final count alone is insufficient.

This is a narrow report-observability child of existing UNI-018/020/021/024 obligations, not a new delivery framework or full grammar/financial-family certification. U0–U7, M1–M17, every MC/SP/G exit, all original fixture/family membership and prior evidence remain unchanged. Local reporting supplies no new target qualification. This proposal makes no security or failure-path judgment and supplies no execution authority.

### proposal/OPENSPEC-DELTA.md
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

### proposal/SOURCE-NOTE.md
# Normal report-wire proposal: source and evidence note

Author: GPT-6.1 Sol high, nonsecurity source proposal. Date: 2026-10-18. Proposal only; no adoption, design vote, independent-oracle qualification or actual behavioral result.

Only files in this outside-Git proposal directory were written. Source checkout inspected read-only: `/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017`; root-assigned current branch `doctorwatson-language/cli-native-feature-20261018`, assigned base `2c2b9182` and retained report tree `61bc8233eb3c0ce421ac5a80e4112eed952925a6`. Git was not queried or modified. Exact inspected file identities below are the actual byte pins for this note; no uncommitted tree identity is invented.

Develop skill/AGENTS/product contract were restored and read-only development CLI status exited0, retaining historical SP01 stale binding/candidate/accounting/resource blockers. No Cargo/rustc/rustfmt, Mori/parser/evaluator/model/SDK/VM, proof/key/wallet, private/cache/configuration/provider or network execution occurred. Existing public Rust types/parser/normal runtime/exporter source was read to ground the proposal. No pending custodian files or newly authored security tests were read. No expected JSON golden was generated, exported, executed or frozen.

## Source observations — not normative approval or run results

- `ir.rs:254` Step declares index, statement, Option<Value>, snapshot and effects. There is no skip_serializing_if on value. Value has type/content tags and UInt serializes as a decimal string. `ir.rs:263` Execution declares the complete execution members. The frozen v1 decode structures retain their own contracts; this proposal does not alter decoding.
- `runtime.rs:719` Let evaluates/captures a value and returns Some(v); Next returns Some(after), accepted predicates return Some(true), and Emit returns Some(v). The proposed Let rule agrees with inspected code but is presented as an explicit design choice, not as an independently established normative requirement.
- `runtime.rs:875..919` normal simulate starts a fresh effects vector per statement, appends it to all_effects, stores the fresh delta in Step.effects and stores post-statement snapshot. This corrects any informal reading of Step.effects as cumulative. The proposed cumulative prefix is derived, not a new wire field.
- `parser.rs:79` state initializers are direct literals. `parser.rs:258` assigns ExprId by expression-array insertion; expr creates child nodes before binary parents. The Counter illustration manually transcribes that public convention. It supplies no independent parser-correspondence result.
- `README.md` documents let capture/immediate next, the existing logical profile and Counter work16. `logical_cost.rs` wraps the same execution schema with CostReport. `runtime.rs:395` creates a fresh evaluation cache and charges uncached node visits. The proposed work table is ordinary arithmetic under that published schedule, not an observed execution.
- `simulate.rs:120/139` has the accepted metered and ordinary public envelope; `core_io.rs:247` exports then println adds final newline. Exporter source is `json.rs:117`. No product exporter was called to construct expected bytes here.
- `core_test_report.rs:109` contains the existing full normal Transfer literal oracle; its recorded freeze is preserved. Its Step values/effects/snapshots are consistency references, not proof of a Let rule or effect-prefix interpretation. `logical_meter.rs` contains the Counter source; its shared-evaluator equality is not treated as independent full normal-report evidence.
- Existing UNI-020 requires independently derived frozen observations; UNI-021 separates evidence/acceptance; UNI-024 retains the full report/public journey obligations. This proposal closes none.

## Design decision for root review

Recommended proposal: accepted Let reports Some(captured typed RHS); all five scoped normal completed kinds report a typed result; Step.effects remains a delta and Execution.effects is its ordered concatenation. The primary reason is a useful unambiguous per-statement observation that distinguishes expression reference from evaluated capture, while preserving current public shapes. This recommendation is not adopted. A different Let.value choice would need an explicit wire/version/compatibility decision before a new oracle is frozen; do not silently fit expectations to existing implementation or pending custodian output.

## Revision 2 — explicit zero-statement normal boundary

This revision remains PROPOSED / UNADOPTED / UNRUN. Root identified the missing N=0 boundary in the first draft; the three outside-Git proposal documents are updated together. The selected design includes otherwise valid empty actions rather than constraining N>=1: steps=[], post=pre, effects=[], with normal semantics/action identity retained. The final-snapshot clause is conditional on N>0, and the cumulative-effect definition explicitly uses its empty P[-1] identity at N=0. No Step or Step.value exists for an empty body.

Source-static check: `parser.rs:135..160` initializes an empty statements vector, may consume the closing brace without a statement iteration, and records that action. `runtime.rs:217..233` checks upper input/statement bounds and iterates statements without a nonempty-body condition. `runtime.rs:880..918` clones normalized pre into current, initializes empty Steps/effects, iterates zero times for an empty body, and returns current as post. `ir.rs:263` represents Steps/effects as Vec fields. These source facts support the proposed boundary; they are not an observed execution or an independent oracle. Read-only development CLI status was refreshed and retained the same historical blockers.

No runtime, tests, expected-golden generation, checkout/Git changes, security case/judgment or native operation occurred for this revision. Existing unobserved contract acceptance remains unobserved; separately reported CLI checks do not close this proposal's independent predicates.

First-draft document identities retained for the root's already-copied proposed evidence:

- `PROPOSED-CONTRACT.md` first-draft SHA256: `c5eccede7fb1a249c3d5108816bf0e7157af741ca31cf58a2f26b9289bc31bab`.
- `OPENSPEC-DELTA.md` first-draft SHA256: `8de857ad20a5567f8e98f6038282584d872c99272658ddd4515fb938b5e91a7d`.
- `SOURCE-NOTE.md` first-draft SHA256: `4019c4240ab9597487d500ec1378f205f7857867a04fde99a645e3e46a8144fd`.

Refreshed empty-action source identities:

- `crates/mori-core/src/parser.rs` SHA256 `587e8b024ae16d513c5e6d6ef4929e4140bbfc7d2a716b5d38781645cb7c7275`; unchanged from the first source note.
- `crates/mori-core/src/runtime.rs` SHA256 `eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930`; unchanged from the first source note.
- `crates/mori-core/src/ir.rs` SHA256 `0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d`; unchanged from the first source note.

## Inspected public source identities

| Source path | SHA256 |
|---|---|
| `crates/mori-core/src/ir.rs` | `0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d` |
| `crates/mori-core/src/parser.rs` | `587e8b024ae16d513c5e6d6ef4929e4140bbfc7d2a716b5d38781645cb7c7275` |
| `crates/mori-core/src/runtime.rs` | `eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930` |
| `crates/mori-core/src/json.rs` | `faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183` |
| `crates/mori-core/src/logical_cost.rs` | `8c1b38da8cbefb5f3bb83e89fc1fb471410e3886fc50b74e2af8c0d9dc9636cc` |
| `crates/mori-core/README.md` | `0d91facda4bcf06e545af2f9ced7c72b2b76220a9b53eb46a5a4caea66ebc4dc` |
| `crates/mori/src/commands/simulate.rs` | `53b1c0cb0bbaa94cf890890c37e1a282535e5e8bbcf29dafaaef498faed9ac11` |
| `crates/mori/src/commands/core_io.rs` | `6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594` |
| `crates/mori/tests/logical_meter.rs` | `c4c6de685036d8c9f36ccc01910a9f5b1c4948e70f0193ec942b1fbd65baa129` |
| `crates/mori/tests/core_test_report.rs` | `74e021ed57129647354f3ec226d5d3acdee314091d9ed77870b67c3b76ac2387` |
| `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md` | `66f85468058830690bd8e0d21e4f76a291dd99c572f1f5224cbf4b49b8de67b1` |

All full original scopes/exits and prior evidence remain retained. Source observations here are not compiler/test success, independent semantic correctness, native cost/correspondence or Midnight target qualification.

Revision2 companion document identities:

- `PROPOSED-CONTRACT.md` SHA256 `e858fa259e01c8cdaafb2e139c2b92f076c70789e87843463ce9aaf26caaa224`.
- `OPENSPEC-DELTA.md` SHA256 `f6bca6929a360bf13fd4eb1e10a711f830fef3e19e13f241cb5ed7781226e456`.

### independent-positive-facts/DERIVATION.md
# Independent Counter positive facts and remaining wire gap

Status: **specified-only; no complete whole-output golden or integration test supplied**. This packet concerns only the existing Core/1 Counter, normal amount7 Scenario and the unmetered/cap16/capmax positive calls. It leaves the current twelve-test compatibility proposal and its frozen input/oracle packet unchanged. It authors no refusal, hostile/security case, resource policy, build or execution admission.

Required Moriarty development startup/status was restored. Only the source/scenario already frozen in the prior independent custody packet, the public Core reference, public IR wire type declarations and independently authored existing Core/1 oracle data supplied the derivation. No evaluator/parser/compiler implementation, changed candidate/helper, actual product output, provider/private/cache/config/key material, runtime/build/Mori/model/SDK/proof/wallet/network operation or Git mutation was read/run/performed. A read of unlabelled compatibility fixture values did not establish independent provenance; those values were deliberately excluded as authority. The independent Core/2 staged Let convention was not promoted to a Core/1 rule.

`counter.mori` is the exact212-byte source from the prior packet, SHA-256 `8e89db83018345a771224e3b20218eb06eb905562d4d3bc5f239b3aff41f07b0`. `amount7.scenario.json` is the exact prior raw Scenario. `INPUT-AND-PREMISES.json` records their identities and the permitted basis files; `FREEZE.sha256` binds every packet member except itself. The prior packet's freeze SHA-256 remains `481f6d56f12e87730ae7b05324a7563ef1ea311800e9177f88f1f1b1ca81e29a`.

## Semantic derivation

The empty Scenario state selects declared initial count UInt0. Input amount is UInt7. No asset, balance or liability is declared or supplied. `let total = count + amount` captures UInt7 without updating state; requires7≤100 is true. `next count = total` updates count0→7 immediately. The following ensures sees count7 and captured total7, so equality is true. The event emits current count UInt7.

All five complete snapshots are therefore count0, count0, count7, count7, count7, each with the same empty balance and liability arrays. Per-step effects are empty, empty, State count0→7, empty, Emit count UInt7. Full effects are exactly that State followed by that Emit. The local name total is not a Snapshot state field. Pre is count0 and post is count7; there are no unlisted preserved financial rows or state variables. Monetary gross/fees/net/reserve/dust/duties are inapplicable to this source, rather than evidence of funded zero-cost settlement.

The public Core reference explicitly gives the same Counter's primitive logical work as4+4+2+4+2=16. Independently counting one statement dispatch plus each distinct expression node gives the same total: Add/name/name, comparison/name/literal, one name, equality/name/name, one name. Fresh caches per statement prevent previous evaluations from reducing those charges. Cap16 and the u128 maximum cap both permit exactly16 logical units. This is a source/reference schedule prediction, never an observed meter, wall-time, allocation, DUST, service, native or ledger cost.

## Proposed concrete IDs and established Step fields

Under the lexical fresh-node/postorder hand-allocation premise already disclosed by independent Core/1 oracles, nodes0..10 are count, amount, Add(0,1), total, UInt100, Le(3,4), total, count, total, Eq(7,8), count. The direct state initializer is not an expression-arena node. Thus statements reference2/5/6/9/10. Repeated identifier occurrences receive fresh nodes. This is an independently hand-lowered representation premise for this source, not a universal parser specification or an observation of the production parser. Actual parser correspondence remains an eventual consumer predicate; a mismatch must not rewrite these proposed IDs.

The public `Step` wire type declares `value: Option<Value>`; it does not define the per-kind attachment rule. The independently authored [bound replay oracle](basis/bound-replay-oracle.md), original JSON SHA-256 `30aa690ee081987728a310e32ac93020dcf7c7d5a890044e348dd3e21fd326f3`, supplies successful Requires and Ensures Booltrue, Next's evaluated UInt and Emit's evaluated UInt. The independent normal report freeze `3911ef8990bbcdf067f641bcae032f48828b2035ddae55f8687969ecc0cb2395` independently supplies Requires Booltrue as well. Neither has a Let Step. Their values support the four concrete remaining Step.value fields in `POSITIVE-FACTS.json`, under the same disclosed per-kind representation premise.

**Exact remaining hole:** `/steps/0/value` in Execution, hence `/execution/steps/0/value` in each intended simulate envelope. Semantically the captured Let value is certainly UInt7. The permitted independent public Core/1 contracts inspected do not determine whether a successful Let publishes Some(UInt7) or None/null. The Option type alone cannot choose. A fixture without established independent provenance, the current direct evaluator call, a helper output or the first actual CLI output cannot fill that gap while preserving independence.

`POSITIVE-FACTS.json` deliberately omits that field from its custom `execution_known_fields` projection and explicitly records the hole. That omission is **not** an expected omitted serialized member, an expected null, an instruction to ignore the field in a full equality check, or a complete accepted Execution. The JSON file is an explanatory facts schema, not an executable whole-output test oracle. No guessed UInt7/null alternative is frozen as a selected output. This is an observability/specification gap; it predicts no evaluator rejection.

## Envelope and byte limits

The documented ordinary output has status accepted, qualification `local-reference-execution` and execution. Metered output additionally has profile `moriarty-reference-logical-cost/1` and literal logical `{cap:"16",used:"16"}` or `{cap:"340282366920938463463374607431768211455",used:"16"}`. The execution semantics, prefix snapshots and effects are the same for all three calls. Expected positive exit0 and empty stderr follow the unchanged existing normal CLI test premises; they remain unobserved for the current candidate.

Because the embedded execution contains the unresolved Let field, **no complete literal accepted execution, whole simulate stdout file, raw stdout digest or standalone positive test is supplied**. Supplying a projection test and calling it full equality would weaken the requested predicate. A separate ordinary source increment must specify the missing Core/1 attachment convention independently before a new whole-output golden is created and frozen. Existing shared-evaluator compatibility tests may retain their original limited scope; this packet does not silently replace or upgrade their oracle.

All original fourteen staged draft gaps, representation/admission limitations, failed-history records and full U0–U7/M1–M17/MC/SP/G, ACTUS/DeFi, staged/composition/private/recursive/native/target/release obligations remain unchanged. No source-state arithmetic, packet hash or future local positive CLI comparison can complete the original language goal. Internal review/custody records remain separate from permissionless public developer use.

### independent-positive-facts/POSITIVE-FACTS.json
{
  "format":"moriarty-independent-counter-positive-facts/1",
  "status":"specified-only; complete semantic/prefix/effect/work facts with an explicitly incomplete wire projection",
  "not_a_complete_Execution_or_simulate_output_golden":true,
  "input":{"source":"counter.mori","scenario":"amount7.scenario.json","action":"run","language":"moriarty/1","semantics":"moriarty-core/1"},
  "arithmetic":{"initial_count":"0","amount":"7","captured_total":"7","requires_total_at_most100":true,"next_count":"7","ensures_count_equals_captured_total":true,"emitted_count":"7"},
  "representation_premise":"Hand allocation follows fresh lexical/postorder expression nodes and direct literal state initializer outside the expression arena, consistent with the independently hand-lowered existing Core1 oracles. This supplies proposed concrete ExprIds for this source, not a universal parser specification or proof. No parser/evaluator/compiler implementation or output was read to obtain them.",
  "expression_arena":[
    {"id":0,"kind":"Name","name":"count"},
    {"id":1,"kind":"Name","name":"amount"},
    {"id":2,"kind":"Binary","op":"Add","lhs":0,"rhs":1},
    {"id":3,"kind":"Name","name":"total"},
    {"id":4,"kind":"Literal","value":{"type":"UInt","value":"100"}},
    {"id":5,"kind":"Binary","op":"Le","lhs":3,"rhs":4},
    {"id":6,"kind":"Name","name":"total"},
    {"id":7,"kind":"Name","name":"count"},
    {"id":8,"kind":"Name","name":"total"},
    {"id":9,"kind":"Binary","op":"Eq","lhs":7,"rhs":8},
    {"id":10,"kind":"Name","name":"count"}
  ],
  "execution_known_fields":{
    "semantics":"moriarty-core/1",
    "action":"run",
    "pre":{"state":{"count":{"type":"UInt","value":"0"}},"balances":[],"liabilities":[]},
    "post":{"state":{"count":{"type":"UInt","value":"7"}},"balances":[],"liabilities":[]},
    "steps":[
      {
        "index":0,
        "statement":{"kind":"Let","name":"total","value":2},
        "snapshot":{"state":{"count":{"type":"UInt","value":"0"}},"balances":[],"liabilities":[]},
        "effects":[]
      },
      {
        "index":1,
        "statement":{"kind":"Requires","predicate":5},
        "value":{"type":"Bool","value":true},
        "snapshot":{"state":{"count":{"type":"UInt","value":"0"}},"balances":[],"liabilities":[]},
        "effects":[]
      },
      {
        "index":2,
        "statement":{"kind":"Next","state":"count","value":6},
        "value":{"type":"UInt","value":"7"},
        "snapshot":{"state":{"count":{"type":"UInt","value":"7"}},"balances":[],"liabilities":[]},
        "effects":[{"kind":"State","name":"count","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"7"}}]
      },
      {
        "index":3,
        "statement":{"kind":"Ensures","predicate":9},
        "value":{"type":"Bool","value":true},
        "snapshot":{"state":{"count":{"type":"UInt","value":"7"}},"balances":[],"liabilities":[]},
        "effects":[]
      },
      {
        "index":4,
        "statement":{"kind":"Emit","name":"count","value":10},
        "value":{"type":"UInt","value":"7"},
        "snapshot":{"state":{"count":{"type":"UInt","value":"7"}},"balances":[],"liabilities":[]},
        "effects":[{"kind":"Emit","name":"count","value":{"type":"UInt","value":"7"}}]
      }
    ],
    "effects":[
      {"kind":"State","name":"count","before":{"type":"UInt","value":"0"},"after":{"type":"UInt","value":"7"}},
      {"kind":"Emit","name":"count","value":{"type":"UInt","value":"7"}}
    ]
  },
  "unresolved_wire_fields":[
    {
      "execution_pointer":"/steps/0/value",
      "all_simulate_envelope_pointers":["/execution/steps/0/value"],
      "known_semantic_value":{"type":"UInt","value":"7"},
      "wire_type":"Option<Value>",
      "unknown":"Whether the successful Let Step exports Some(captured UInt7) or None/null. No permitted independent public Core1 contract inspected supplies the per-kind attachment rule for Let.",
      "projection_omission":"This field is deliberately absent from execution_known_fields. Absence is a gap marker in this custom facts document, NOT an expected omitted JSON member, an expected null, a complete serde Execution value or a wildcard-matching test contract.",
      "not_evaluator_rejection":true
    }
  ],
  "simulate_envelope_known_fields":{
    "unmetered":{"status":"accepted","qualification":"local-reference-execution","execution_reference":"execution_known_fields; missing pointer must first be independently specified","ordinary_top_level_field_count":3},
    "cap16":{"status":"accepted","qualification":"local-reference-execution","profile":"moriarty-reference-logical-cost/1","logical":{"cap":"16","used":"16"},"execution_reference":"the same execution_known_fields; missing pointer remains unresolved"},
    "capmax":{"status":"accepted","qualification":"local-reference-execution","profile":"moriarty-reference-logical-cost/1","logical":{"cap":"340282366920938463463374607431768211455","used":"16"},"execution_reference":"the same execution_known_fields; missing pointer remains unresolved"}
  },
  "work":{
    "profile":"moriarty-reference-logical-cost/1",
    "statements":[
      {"index":0,"statement":"let total = count + amount","expression_ids":[2,0,1],"statement_charge":"1","expression_charges":"3","units":"4","cumulative":"4"},
      {"index":1,"statement":"requires total <= 100","expression_ids":[5,3,4],"statement_charge":"1","expression_charges":"3","units":"4","cumulative":"8"},
      {"index":2,"statement":"next count = total","expression_ids":[6],"statement_charge":"1","expression_charges":"1","units":"2","cumulative":"10"},
      {"index":3,"statement":"ensures count == total","expression_ids":[9,7,8],"statement_charge":"1","expression_charges":"3","units":"4","cumulative":"14"},
      {"index":4,"statement":"emit count(count)","expression_ids":[10],"statement_charge":"1","expression_charges":"1","units":"2","cumulative":"16"}
    ],
    "total":"16","counter_cap16":"16","counter_capmax":"340282366920938463463374607431768211455",
    "cache":"fresh per statement; all expression nodes within a statement are distinct; no dynamic loops or short-circuit assumptions",
    "expression_id_lists":"node membership only; their listed order does not certify a charge visitation order",
    "unmeasured":["parsing","validation","normalization","allocation","wall time","DUST","service fees","external work","native circuit/proof/ledger cost"]
  },
  "financial_scope":{"declared_assets":[],"balances":[],"liabilities":[],"transfer_or_fee_or_duty_effects":[],"gross_debit":"inapplicable; no financial asset or payment in source","fees":"inapplicable; no declared fee asset","net_receipts":"inapplicable; no financial recipient","reserve_or_dust_or_residual_duties":"inapplicable; no financial contract rows; no funded settlement is inferred"},
  "intended_positive_calls":[
    {"cap":null,"expected_exit":0,"expected_stderr":"empty","full_stdout_golden":"NOT_AVAILABLE_PENDING_Let_Step.value_CONTRACT"},
    {"cap":"16","expected_exit":0,"expected_stderr":"empty","full_stdout_golden":"NOT_AVAILABLE_PENDING_Let_Step.value_CONTRACT"},
    {"cap":"340282366920938463463374607431768211455","expected_exit":0,"expected_stderr":"empty","full_stdout_golden":"NOT_AVAILABLE_PENDING_Let_Step.value_CONTRACT"}
  ],
  "acceptance":"No whole-output golden or positive integration test is supplied. Filling the missing pointer from parser/evaluator/current CLI output would violate independence. Existing12 compatibility tests and prior oracle packet remain unchanged. All original stage and roadmap gaps remain open."
}

### independent-positive-facts/basis/core1-public-wire-types.txt
/// `v1` wire, so JSON can never yield `Await`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Statement {
    Await {
        label: String,
        binders: Vec<InputDecl>,
        guard: ExprId,
    },
    Let {
        name: String,
        value: ExprId,
    },
    Requires {
        predicate: ExprId,
    },
    Ensures {
        predicate: ExprId,
    },
    Next {
        state: String,
        value: ExprId,
    },
    Emit {
        name: String,
        value: ExprId,
    },
    Transfer {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Fee {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Liability {
        name: String,
        asset: String,
        debtor: String,
        creditor: String,
        amount: ExprId,
    },
    Discharge {
        name: String,
        amount: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub name: String,
    pub inputs: Vec<InputDecl>,
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub language: String,
    pub semantics: String,
    pub name: String,
    pub states: Vec<StateDecl>,
    pub assets: Vec<AssetDecl>,
    pub expressions: Vec<Expr>,
    pub actions: Vec<Action>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub account: String,
    pub asset: AssetId,
    pub amount: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Liability {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub amount: UInt,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub state: BTreeMap<String, Value>,
    pub inputs: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub state: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Effect {
    State {
        name: String,
        before: Value,
        after: Value,
    },
    Transfer {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Fee {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Liability {
        name: String,
        asset: AssetId,
        debtor: String,
        creditor: String,
        before: UInt,
        after: UInt,
    },
    Emit {
        name: String,
        value: Value,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub index: u16,
    pub statement: Statement,
    pub value: Option<Value>,
    pub snapshot: Snapshot,
    pub effects: Vec<Effect>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub semantics: String,
    pub action: String,
    pub pre: Snapshot,
    pub post: Snapshot,
    pub steps: Vec<Step>,
    pub effects: Vec<Effect>,
}
/// Frozen `moriarty-core/1` decode wire: the baseline (fe01bc8b) derives,
/// moved verbatim. Type names are kept so serde's `expecting` text, unknown
/// variant lists and error messages stay byte-identical. There is no Await.
pub mod v1 {
    use super::{AssetDecl, Effect, Expr, ExprId, InputDecl, Snapshot, StateDecl, Value};

Fullfrozenpublicpacket /home/charl/moriarty-c2-opus55-restored-20261017/normal-report-contract-cycle/design-packet MANIFESTsha 689409e3bc2e5bf0cc9614557871aee6258934e61e4f9b868bccdcea777a3be2
