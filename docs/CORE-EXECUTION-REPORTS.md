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

## Adopted financial observation extension

This section defines the adopted additive observation contract for normally accepted finite Rust `moriarty/1` / `moriarty-core/1` Transfer, Fee, Liability and Discharge statements. Its SHALL clauses are effective design requirements under the [recorded design adoption](../evidence/utxo/doctorwatson-language-loop-2026-10-17/financial-report-adoption-20261018/DESIGN-ADOPTION.json) of 2026-10-18. They do not replace NR-01–06, broaden the earlier five-kind adoption, or change typing, arithmetic, admission or financial transitions. The author inspected implementation source; this design is not independent behavior proof, a golden or an executed result. Core/2, other language profiles and target consumers retain separate contracts.

Current Core/1 has no Mint, Burn or dedicated accrual statement. Liability creates a named duty or adds an explicitly supplied amount. This addition supplies no interest rate, time basis, day-count, compounding or accrual schedule. Mint/burn and dedicated accrual remain missing language/financial obligations where required by the broader product; this report extension neither implements nor removes them.

Let S[-1] be the complete normalized Execution.pre and S[i] the complete snapshot immediately after statement i. Let q be the UInt result of that statement's amount expression evaluated immediately before its transition, using current state, action inputs, earlier captures and current liability reads under existing Core semantics. For Transfer, Fee and Liability, let A be the exact declared AssetId denoted by the statement's asset alias. Discharge uses the existing duty row's AssetId, debtor and creditor in S[i-1]. For each completed financial statement, the Step SHALL have exactly index, statement, value, snapshot and effects. The numeric zero-based index SHALL be its ordinal in the selected action, including preceding nonfinancial statements. The exact admitted statement SHALL retain these members in addition to kind:

| kind | Statement members | Meaning |
| --- | --- | --- |
| Transfer | asset, from, to, amount | asset alias, account strings, amount ExprId |
| Fee | asset, from, to, amount | asset alias, account strings, amount ExprId |
| Liability | name, asset, debtor, creditor, amount | duty name, asset alias, party strings, amount ExprId |
| Discharge | name, amount | existing duty name, amount ExprId |

Statement.amount SHALL remain the numeric zero-based reference into that Program's expressions array. The outer Step.value SHALL instead contain the present typed UInt object with type "UInt" and value q's canonical unsigned decimal string, including zero. It SHALL report the moved, charged, added or discharged amount, respectively; neither the ExprId, a boolean success value, recipient balance nor closing duty amount shall replace it. Null or omission SHALL NOT replace the typed object for a completed statement of these kinds. The following table defines the report projection of existing accepted transitions; it creates no new admission premise:

| Statement | Complete post-statement snapshot relative to S[i-1] | Exact new Step.effects |
| --- | --- | --- |
| Transfer | Existing movement of q in A between the two explicit account rows; preserve all other balances, state and liabilities | one Transfer {asset:A,from,to,amount:q} |
| Fee | Existing movement of q in A between the two explicit account rows; preserve all other balances, state and liabilities | one Fee {asset:A,from,to,amount:q} |
| Liability, row absent | Insert the identified duty with amount q in name order; preserve all other duties, balances and state | one Liability {name,asset:A,debtor,creditor,before:0,after:q} |
| Liability, row present with amount b | Preserve duty identity; replace only its amount with the existing checked sum b+q; preserve other duties, balances and state | one Liability {name,asset:A,debtor,creditor,before:b,after:b+q} |
| Discharge, row present with amount b | Preserve duty identity; replace only its amount with the existing checked difference b-q, retaining zero; preserve other duties, balances and state | one Liability {name,asset:existing asset,debtor:existing debtor,creditor:existing creditor,before:b,after:b-q} |

The brace forms denote exact effect field sets plus kind carrying the named variant. Each effect AssetId SHALL contain network, issuer, reference and scale, rather than the statement asset alias. Effect amount, before and after SHALL be canonical decimal strings without a typed Value wrapper. Discharge SHALL use the Liability effect variant; no new Discharge effect is introduced. The exact statement and complete prior snapshot SHALL retain the distinction among creation, addition and discharge. A Liability effect with before zero alone cannot distinguish absent creation from a present-zero opening duty.

For an already admitted self-transfer or self-fee, the numeric balance SHALL remain unchanged while the amount effect remains present. For an already admitted zero amount, the effect SHALL remain present even where numeric amounts do not change. Zero-amount Liability creation SHALL insert a present-zero duty; zero or full Discharge SHALL retain the existing duty row. These clauses add no admission authority. Discharge SHALL remain a duty reduction; any payment is a separate statement and Step.

Each Step.snapshot SHALL equal S[i] in full, containing exactly state, balances and liabilities. State SHALL retain all declared names and typed values; inputs and let captures SHALL NOT become state fields. Balance rows SHALL retain exactly account, full asset and amount; liability rows exactly name, full asset, debtor, creditor and amount. Affected and preserved rows, including observer balances and incoming duties admitted by Core, SHALL remain present. Balances SHALL be ordered by exact (asset,account), AssetId by (network,issuer,reference,scale), and liabilities by name. Absent and present-zero liabilities SHALL remain distinct. Empty state SHALL be {}, and empty balance/liability collections SHALL be [], not omitted or null. This frame concerns the admitted local snapshot and supplies no authenticated-domain or ledger-acceptance claim.

Step.effects SHALL contain only the statement's new effect, rather than an accumulated prefix. With P[-1]=[] and P[i]=P[i-1] followed by Step[i].effects, Execution.effects SHALL equal the ordered concatenation of all statement deltas, including intervening nonfinancial effects, preserving duplicates and zero/self effects. P remains derived notation, not a wire field. Complete snapshots and effect histories SHALL both be retained; unchanged numeric state does not imply empty effects. For N>0, post SHALL equal the last Step.snapshot. For N=0, the existing complete pre=post, steps:[] and effects:[] requirements remain, without a synthetic Step. Existing Execution/envelope fields and logical-work schedule remain unchanged; no per-Step work member is introduced. Any claimed ordinary/metered agreement SHALL compare the same complete Execution and independently specified work under the existing schedule.

Design adoption, source/IR correspondence, independent expectations, actual runtime observations and acceptance SHALL remain separate. Expectations for complete statements/ExprIds, typed values, snapshots, deltas, prefixes, work and consumer-specific bytes SHALL be independently derived and frozen before authorized runs, without generating them from the implementation under comparison. Existing financial-pair coverage and the normal Transfer aggregate compatibility case SHALL retain their exact scope; no full raw Step.value/statement or ordinary simulate-byte coverage is inferred. Original proposals, freezes, failures, five-kind adoption and pending normal-report cases SHALL remain unchanged. All U0–U7, M1–M17, MC/SP/G, full finance, ACTUS/DeFi and composition obligations remain. This observation contract decides no conservation, soundness, authorization, authentication, proof or security condition and supplies no hostile/security tests; those remain with the designated security owner. Internal review records remain separate from permissionless developer use.

## Provenance and conformance

The adopted design retains the original unadopted proposal and its review history. Its author had inspected implementation source; that exposure is disclosed and supplies no independent behavior proof. A separate custodian must derive and freeze complete expected inputs, IR, typed results, snapshots, deltas, prefixes, work and public bytes before a conformance run, without using the implementation under comparison to generate expectations. Original freezes and failures remain unchanged.

Implementation, source inspection, actual observations and acceptance remain separate for the exact profile, candidate and consumer under UNI-021. This report contract does not close a parent roadmap exit or broaden an existing financial, staged or target result. Public developers remain subject to the [permissionless product contract](MORIARTY-PRODUCT-CONTRACT.md); internal review and evidence records are not prerequisites for using the language.
