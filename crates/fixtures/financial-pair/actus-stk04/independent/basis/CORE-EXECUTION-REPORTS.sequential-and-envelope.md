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

