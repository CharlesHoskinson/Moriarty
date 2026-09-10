# Expression contract /1 runtime

The separate TypeScript runtime executes all forty constructors of the proposed
`moriarty-expression-contract/1`. It admits closed Core, schemas and snapshots,
checks the complete action before reduction, and returns either a typed fragment
value, `ExpressionPrepared`, or a defined rejection with consumed work and original
diagnostic provenance. These implementation bytes still require fresh independent
review.

Run `node deliverables/sp02-expression-runtime-2026-09-10/demo-01.mjs` from the
repository root to observe x:10→11 with workRemaining90, then an ENSURES_FAILED
rejection using ten work units and publishing no tentative state.

The public entry point is
[`createExpressionContractV1`](../../experiments/moriarty-language/src/successor/expression-v1.ts).
The trusted host supplies the schema as canonical JSON text when creating an
evaluator. Its `evaluate(requestCanonicalJSON)` method accepts a closed request:

```text
{contract, source, core, Pre, Args, Obs, workInitial}
```

`contract` must be `moriarty-expression-contract/1`. `workInitial` is a canonical
decimal string. Core, spans, types and values use the exact trees in
representation.md; the request wrapper is an API envelope, not a new Core node
or registered execution profile. Every JSON object is in canonical key order,
and the text has no whitespace or nonminimal escapes. Duplicate keys, JSON
numbers/null, live objects and malformed scalar text reject. The trusted host
must bind the reviewed schema; this factory does not authenticate or register
it. An evaluation request cannot replace the schema or reclassify financial
fields.

The runtime is implemented in three files:

- `expression-wire-v1.ts`: canonical spelling, owned JSON parsing, closed
  structures and finite transport guards.
- `expression-types-v1.ts`: exact type/value trees, schema references and cycles,
  nominal numeric domains, and individual/aggregate size checks.
- `expression-v1.ts`: structural Core admission, complete-action static typing,
  ordered snapshot validation, all forty reductions and rejection results.

Each evaluation parses a fresh owned schema and request. Returned objects cannot
change later calls. No production code imports cases or dispatches by expected
outcome, case identity or source string. The reducer dispatches on the closed
constructor inventory and uses exact BigInt arithmetic.

And/Or short-circuit after exhaustive admission and static checking. A skipped
right operand consumes no work; a selected operand retains its original path
and span. Malformed spans use the candidate03 correction for finding A-1: preserve the
input defect and node path but return valid synthetic `[0,0)` output provenance.

Let binds only after its value succeeds. NextWrite stages one write per ordinary
field. Ensure sees the assembled post view in the final suffix. Emit appends a
typed descriptor and checks its individual and aggregate bounds; it executes no
financial operation. A rejection returns no tentative post-state, locals, writes
or descriptors. Diagnostic work is not a committed ledger or authority charge.

## Actual checks

The focused Node suite passes **188 tests**. It executes every one of the forty
positive/rejection pairs, all nineteen active combined derivations, the explicit
successor of the historical strict-And case, all forty-eight corrected Boolean
fixtures, and additional admission/encoding/boundary controls. The cases that
assume earlier locals or writes establish those frames through actual statement
prefixes. Their tests state the added prefix/wrapper work; the API does not accept
forged initial locals or writes.

The finite-record failure uses the specified zero matrix: its input is 8,455
bytes, so a larger byte representation does not mask the 4,355-node constructed
value failure. Other controls cover the exact 65,536/65,537-byte input boundary,
64/65 value depth, 128/129 descriptors, 256/257 statements, 64/65 record fields,
numeric endpoints, Unicode byte lengths, complete static checking before
snapshot/runtime errors, schema cycles and aliasing isolation.

The complete language regression suite passes **424 tests** and TypeScript
checking passes. Commands, exit codes and full logs are retained beside this
document. Two root-authored admission/isolation controls are included unchanged.

An author test reproduced locale-dependent schema ordering (`A` versus `a` with
different errors). The implementation now uses ASCII ordering, and the test
passes. Root separately checked the corrected behavior; its control does not
claim to have reproduced the earlier failure. The earlier failing-run records
are retained as observations, not counted as passing checks.

## Scope

This is an executable expression contract, not full SP02/SP03 acceptance. The
source parser is not connected to a full forty-constructor elaborator here;
callers supply the explicit Core representation. Pure fragment `ExpressionValue`
results are diagnostic judgments, not a new value constructor or accepted
financial status. Statement lists return the defined `ExpressionPrepared` tree.

The existing funded evaluator, source profile, repayment paths and K definitions
remain byte-for-byte unchanged. No financial-operation relation, signing/history
authentication, privacy/disclosure policy, native proof, K correspondence or
Midnight acceptance is supplied by this runtime. Future numeric or financial
extensions require a distinct reviewed boundary; unsupported tags or overloads
reject under the current closed contract.
