# Executable financial expressions

The separate financial expression runtime now has fresh independent GPT-6 Astra
and Grok 4.6 scoped approvals. The [result](scoped-result-01.json) binds candidate01,
both raw reviews and the fresh root regression run. All 42 candidate file hashes
match. Root reran 324 old/new profile tests and strict TypeScript checking;
both passed. GPT-6 independently exercised the public API and vault arithmetic.
Grok reviewed the supplied source statically.

`createFinancialExpressionContractV1(schemaCanonicalJSON).evaluate(requestCanonicalJSON)`
executes all 48 constructors. The eight additions support amount and share
construction, tagged variants, option projection, explicit integer conversion,
scalar extraction and conditional selection. UInt256 and dimensional arithmetic
preserve asset identities and check intermediate overflow. Both conditional arms
are checked statically; execution enters only the selected arm.

The unchanged 56-node vault conversion body runs against twenty independent
arithmetic cases. Its positive-supply and positive-valuation conditions are
explicit guards outside that body and consume additional work. Typed empty
financial-state fixtures do not establish valid financial states.

This version uses the financial specification's limit of 256 named record,
enum and variant types, alongside simultaneous bounds on the entire schema.
The original forty-constructor profile keeps its existing total-declaration
limit. Variant families require distinct tags and payload types. The original
root fixture that violated this rule and its failed result remain preserved;
the corrected valid fixture checks runtime rejection of a wrong variant tag.

The original [candidate description](README.md) remains frozen and records the
pre-review state. This result supplies its subsequent review outcome. Full
`.mori` authoring, all 38 financial transitions, signing, K correspondence,
recursive proofs, financial conformance and Midnight acceptance remain open.
This implementation completes a pure-expression component, not a sprint.
