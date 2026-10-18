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
