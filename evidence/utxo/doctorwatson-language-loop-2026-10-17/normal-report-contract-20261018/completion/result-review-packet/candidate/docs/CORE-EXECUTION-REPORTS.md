# Normal Core/1 execution reports

This contract defines the adopted public report design for accepted finite Rust `language "moriarty/1"` / `moriarty-core/1` actions using `Let`, `Requires`, `Next`, `Ensures` and `Emit`, including an otherwise valid empty action. It defines their observations without changing source typing or arithmetic. Other financial statements retain their existing semantics and reports. Core/2 staged reports and target compilation have separate contracts.

Design adoption on 2026-10-18 establishes these requirements. Runtime behavior, source-to-IR correspondence and exact public bytes require separate conformance evidence; this document records no such result. The clauses refine existing [UNI-018/020/021/024](../openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md#requirement-uni-018-profile-and-consumer-admission), with child labels NR-01–06.

## Public envelope and execution identity

For a successfully printed ordinary `mori simulate` result, the public envelope SHALL contain exactly `status: "accepted"`, `qualification: "local-reference-execution"` and `execution`. Execution SHALL contain exactly `semantics`, `action`, `pre`, `post`, `steps` and `effects`. Its semantics SHALL be `moriarty-core/1`, and its action SHALL be the selected action's name.

For an action with N statements, `steps` SHALL contain N entries in source order. Each Step SHALL contain all five members: `index`, `statement`, `value`, `snapshot` and `effects`. Entry i SHALL carry numeric zero-based `index: i` and the exact serialized statement i from the admitted public IR. Each ExprId in that statement SHALL be a numeric zero-based reference to the same Program's `expressions` array. An ExprId identifies an expression, rather than its evaluated value, source offset or Step. Numbering belongs to the exact source/IR candidate; it is not promised to survive source edits. These are NR-01 identity requirements.

Under NR-02, every accepted statement of the five scoped kinds SHALL have a present typed outer `Step.value`. UInt uses a canonical decimal string, for example `{"type":"UInt","value":"7"}`. Bool uses a JSON boolean, for example `{"type":"Bool","value":true}`. The outer Option encodes Some as that typed object and None as JSON null; null or an omitted outer `value` is not the selected representation for these completed statement kinds. This reporting rule does not change input decoding elsewhere.

`statement.value`, where present, is an ExprId number. The outer `Step.value` is the typed evaluation result. For example, `Let { name: "total", value: 2 }` can report outer `value: {"type":"UInt","value":"7"}`. The reference 2 and result 7 have different meanings.

## Sequential values and complete snapshots

Let S[-1] be `execution.pre` and S[i] the complete snapshot after statement i. A Snapshot SHALL contain exactly `state`, `balances` and `liabilities`. It SHALL retain affected and preserved rows, with declared state names mapped to typed values, balances ordered by exact `(asset, account)` and liabilities ordered by name. Asset comparison uses `(network, issuer, reference, scale)`. Inputs and local let captures are not Snapshot state fields.

NR-03 requires `pre` to be the complete normalized starting snapshot, including declared initial values selected by an empty supplied state map. Each `steps[i].snapshot` SHALL equal S[i]. For N>0, `post` SHALL equal the final Step snapshot. A let capture remains available to later statements while retaining its captured value; `Next` updates state immediately for subsequent expressions.

Define Eval(e,i) as the typed semantic value of expression e immediately before statement i, using the then-current state, action inputs and earlier captures. This notation specifies evaluation; it is not a call to an implementation.

| Statement | Required outer Step.value | Complete snapshot after statement | New Step.effects |
| --- | --- | --- | --- |
| `Let { name, value: e }` | Eval(e,i), captured under name | S[i-1] | `[]` |
| `Requires { predicate: e }` | typed Bool true | S[i-1] | `[]` |
| `Next { state: x, value: e }` | after = Eval(e,i), evaluated before assigning x | S[i-1] with only x replaced by after | exactly `[State { name: x, before: S[i-1].state[x], after }]` |
| `Ensures { predicate: e }` | typed Bool true, at its written position | S[i-1] | `[]` |
| `Emit { name, value: e }` | v = Eval(e,i) | S[i-1] | exactly `[Emit { name, value: v }]` |

The typed Let capture is observable even though its snapshot and effects do not change. `Ensures` reads the state at its own source position. Emit records an event while preserving the snapshot.

NR-04 defines `Step.effects` as the ordered effects newly produced by that statement. Define P[-1]=[] and P[i] as P[i-1] followed by `steps[i].effects`. `execution.effects` SHALL equal the ordered concatenation of all Step deltas, preserving repeated effects and array order. P is a derived prefix, not another wire field. Final state alone cannot supply this effect history, and unchanged snapshots can accompany Emit effects.

For N=0, `steps` SHALL be `[]`, `post` SHALL equal the complete normalized `pre`, and `effects` SHALL be `[]`, the empty concatenation P[-1]. Semantics and selected action identity remain present. No synthetic Step, statement index, ExprId or Step.value is created; per-Step rules have no instance for an empty action.

## Complete Counter illustration and logical work

The following exact specimen illustrates the adopted requirements with input amount UInt7 and empty supplied state, balances and liabilities:

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

Its public expression table has the following source-grounded numbering. The declared initial value is a StateDecl literal, not an action expression node. Independent correspondence to an actual parsed Program remains a separate compared observation.

| ExprId | Expression |
| --- | --- |
| 0 | Name count in Let |
| 1 | Name amount |
| 2 | Add(0,1) |
| 3 | Name total in Requires |
| 4 | Literal UInt100 |
| 5 | Le(3,4) |
| 6 | Name total in Next |
| 7 | Name count in Ensures |
| 8 | Name total in Ensures |
| 9 | Eq(7,8) |
| 10 | Name count in Emit |

Here UInt7 and Bool true abbreviate the typed objects above. S0 is `{state:{count:UInt0},balances:[],liabilities:[]}`; S7 differs only by count UInt7. C is `State { name: "count", before: UInt0, after: UInt7 }`, and E is `Emit { name: "count", value: UInt7 }`.

| index | Exact statement fields | Step.value | snapshot | Step.effects | Derived P[i] | Logical units |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | `kind:Let,name:total,value:2` | UInt7 | S0 | [] | [] | 4 |
| 1 | `kind:Requires,predicate:5` | Bool true | S0 | [] | [] | 4 |
| 2 | `kind:Next,state:count,value:6` | UInt7 | S7 | [C] | [C] | 2 |
| 3 | `kind:Ensures,predicate:9` | Bool true | S7 | [] | [C] | 4 |
| 4 | `kind:Emit,name:count,value:10` | UInt7 | S7 | [E] | [C,E] | 2 |

Thus `pre` is S0, `post` is S7 and `execution.effects` is [C,E]. Neither total nor amount becomes a state field. Under the existing `moriarty-reference-logical-cost/1` schedule, each dispatched statement and each uncached expression node costs one logical unit, with a fresh cache per statement. The five increments are `1+3`, `1+3`, `1+1`, `1+3`, `1+1`, totaling16. They describe reference interpretation, not fees or target execution, and introduce no per-Step cost field.

NR-05 requires a positive metered invocation with a sufficient cap to retain exactly the same complete Execution as the corresponding unmetered invocation. Its public envelope SHALL retain status and qualification, adding only `profile: "moriarty-reference-logical-cost/1"` and `logical: { cap, used }`, with canonical decimal strings. For this Counter at cap16, cap SHALL be `"16"` and used SHALL be `"16"`. The example and arithmetic are requirements, not generated goldens or observed executions.

## Public serialization and existing compatibility

NR-06 requires a successfully printed normal accepted result, ordinary or metered, to return exit0 and print one complete JSON report followed by one final newline, with no ordinary status text on stderr. The existing bounded pretty public exporter is retained. Complete semantic JSON equality and independently frozen public-byte compatibility SHALL be checked separately. Object member order is not execution semantics; statement, effect and snapshot-row arrays retain their prescribed ordering. Exact bytes must not be inferred from library field declaration order.

The existing [normal Transfer oracle](../crates/mori/tests/core_test_report.rs) uses the original `mori test` aggregate/report consumer. Its complete accepted-case Execution, under `cases[].actual.Ok`, and original aggregate bytes SHALL remain unchanged. The legacy small-report byte pin is for that 28,093-byte `mori test` aggregate; it is not a byte golden for the ordinary three-field `mori simulate` accepted envelope. Its Requires predicate2 reports Bool true with the pre snapshot and no effects. Its Transfer amount-expression3 reports UInt25 with the complete post snapshot and one new Transfer effect; Execution.effects is the same one-element ordered list. Exact TEST identity is `(local, independent-report-oracle, TEST, scale0)`. Alice100/bob0 becomes alice75/bob25; sixteen observer balances42, incoming_duty11 and empty state remain preserved. The five-kind table does not redefine Transfer's own semantics. This example's single final effect and lack of Let cannot independently establish the new Let result or distinguish delta from prefix reporting. Counter and empty-action `simulate` envelopes require their own independently frozen public-byte expectations.

## Provenance and conformance

The adopted design retains the original unadopted proposal and its review history. Its author had inspected implementation source; that exposure is disclosed and supplies no independent behavior proof. A separate custodian must derive and freeze complete expected inputs, IR, typed results, snapshots, deltas, prefixes, work and public bytes before a conformance run, without using the implementation under comparison to generate expectations. Original freezes and failures remain unchanged.

Implementation, source inspection, actual observations and acceptance remain separate for the exact profile, candidate and consumer under UNI-021. This report contract does not close a parent roadmap exit or broaden an existing financial, staged or target result. Public developers remain subject to the [permissionless product contract](MORIARTY-PRODUCT-CONTRACT.md); internal review and evidence records are not prerequisites for using the language.
