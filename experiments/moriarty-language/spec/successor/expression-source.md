# Expression source /1

Implementation candidate; independent implementation/result audits are pending.
The exact source contract is the preserved [proposal03](../../../../deliverables/sp02-expression-source-2026-09-10/PROPOSAL-03.md).
Its D1–D4 choices have both independent design approvals; D5's narrow diagnostic
corrections remain in the final audit scope. The complete [grammar](expression-source-grammar.ebnf)
extends the separate syntax-only profile. No original profile is upgraded.

Developers can write typed expressions in `.mori`, check them, inspect their Core
and evaluate ordinary state updates against a trusted schema and snapshots.
The source factory accepts source text, never caller-supplied Core or an AST.

```javascript
import { createExpressionSourceV1 } from '../../src/successor/expression-source-v1.ts';
const language = createExpressionSourceV1(schemaCanonicalJSON);
language.check(source);                        // static checks and conservative work
language.elaborate(source);                    // checked Core, action and work bound
language.evaluate(source, snapshotCanonicalJSON);
```

The schema and snapshot strings use the exact canonical representation in
[representation.md](representation.md). Snapshot keys are exactly `Pre`, `Args`,
`Obs`, `workInitial`. The trusted schema controls financial field classes and
operation operand schemas; source cannot reclassify them. Each call owns fresh
input/result trees. Schema and each snapshot have independent65536-byte limits;
the snapshot transport has an outer2000000-byte ceiling before JSON parsing.

`ExpressionPrepared` contains local ordinary post-state and typed operation
descriptors. Emitting `Transfer` does not transfer tokens. Direct writes to
financial fields reject. Any rejection publishes neither tentative state nor
descriptors. Check/elaborate perform no reduction and make no snapshot, signing,
proof, oracle, privacy or ledger claim.

Run the retained real-source example from the repository root:

```sh
npm --prefix experiments/moriarty-language run expression-demo
```

The [example](examples/expression-counter.mori) increments an ordinary counter,
retains the financial field, and returns a quote descriptor. The executable tests
also exercise every one of the reviewed40 constructor names, exact indexed types,
dead-branch static errors, literal/collection boundaries, direct emission work,
UTF-8 spans, input ownership and rollback. See the candidate result under the
[delivery record](../../../../deliverables/sp02-expression-source-2026-09-10/).

This is an expression component of SP02. Exactly one source action is supported;
const/state initialization and source-defined schemas remain unimplemented.
Only optional unit/party roster confirmations and name-only `asset A: Asset;`
confirmations are admitted. The trusted schema must meet the explicit source
name-domain rules; identities are never silently renamed. Numeric type metadata
is written directly in source; there is no host type-alias registry.

Full agreement authoring, multiple actions, genesis/constants, source identity
escaping, financial-operation semantics, K/evaluator correspondence, mandatory
proofs, ACTUS/DeFi coverage and Preview acceptance retain their existing SP01–SP12
owners and remain open. The separate funded and atomic APIs keep their meaning.
The later financial8/UInt256 runtime is not selected by this40-constructor source
profile and needs its own reviewed source extension.
