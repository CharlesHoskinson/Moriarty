# Executable Moriarty expression source

Developers can now write all forty Core expression constructors through real
`.mori` source, check the program, inspect its Core and evaluate local ordinary
state updates. Fresh GPT-6 Astra and Grok 4.6 reviews pass corrected candidate02
and its D5 diagnostic boundary. The [result](scoped-source-result-02.json) binds
both reviews and the fresh root run of 506 passing language tests.

The public API is `createExpressionSourceV1(schemaCanonicalJSON)` with
`elaborate(source)`, `check(source)` and `evaluate(source, snapshotCanonicalJSON)`.
The schema is trusted immutable text; evaluation accepts source and snapshots,
never caller-supplied Core or mutable input objects. The separate parser and
formatter require the explicit `moriarty-expression-source/1` header.

The actual source demo increments an ordinary counter from 10 to 12, preserves
its financial field and returns a typed quote descriptor. Run
`npm --prefix experiments/moriarty-language run expression-demo` from the repo.
Emit describes an operation; this component does not execute a financial action.
Failed guards, ensures and capacity checks publish no tentative state or effects.

The original candidate reported a later field-name error before an earlier
source error in `access_field`. Correction02 preserves arity priority, lowers
the first operand once, then validates field metadata. The original nine failing
cases, corrected 36-case diagnostic matrix, archived source and original reviews
remain intact. GPT-6 independently reproduced both versions and verified exact
UTF-8 spans, paths, work and successful behavior. Grok reviewed the correction
statically; its original pass does not erase the first review's finding.

All forty constructors are expressible within this profile's explicit source
and identifier bounds. Arbitrarily deep serialized Core metadata does not imply
an equivalent source spelling. The current factory supports one action against
a supplied schema; complete source-defined schemas, multiple actions, constants,
genesis and identity escaping remain SP02 work. The original syntax/0 and funded
APIs retain their behavior. Grammar is in
[expression-source-grammar.ebnf](../../experiments/moriarty-language/spec/successor/expression-source-grammar.ebnf).

The future 48-constructor source design is a separate proposal. Full financial
transitions, signing, K correspondence, proofs, ACTUS/DeFi conformance and Midnight
acceptance remain open. This result completes the expression-source component,
not SP02 or the full language.
