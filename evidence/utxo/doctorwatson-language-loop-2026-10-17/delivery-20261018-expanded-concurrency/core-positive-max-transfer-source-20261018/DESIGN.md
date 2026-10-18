# Max rebate public emission regression

This is an unapplied source candidate for the existing Rust `moriarty-beta/1` → raw Source/6 transfer emitter → actual experimental TypeScript Source/6/Core/5 consumer. Root confirmed this route after source inspection showed that Core/1 has no `max`, Qty or `atoms` syntax. No production behavior or language rule changes.

Pinned implementation base: e4d7a2b5709caa18a94f5256d03a0302364b8235. The requested task output directory retains `core-positive` in its name as provenance, not a Core/1 or backend Artifact coverage claim. Local main and checkout HEAD are not the source basis.

The new program combines `max`, Qty subtraction, left Scalar×Qty, `atoms` and positive fee. The dedicated regression invokes the actual reviewed Rust binary twice: public JSON checking, then quiet explicit Source6 expansion. It saves every byte and process outcome and passes the raw decoded expansion stdout directly to both existing TS APIs. It saves both complete returned objects or errors before product-result assertions. It compares the complete Source6 AST and separately frozen financial/effect observations. It captures the whole Rust check report but only asserts source/action/support/diagnostic facts; it does not invent an independent span/report-byte oracle.

Use a new fixture namespace, copied independent oracle freeze, and a dedicated new regression script. This keeps earlier custody/fixtures unchanged and avoids a shared helper refactor. Reusing only the old direct process-capture pattern keeps this test on the actual consumer. A Rust-only text snapshot would miss real TS formation/preparation; a hand-built AST would skip the emitter. Neither is used.

The oracle custodian receives only `inputs/max-rebate.mori` and `inputs/max-rebate.scenario.json` plus action `pay_rebate`. Authored assertions, outputs and derived quantities are excluded. The custodian freezes independent expectations before root execution. The new fixture copy must preserve the complete frozen packet bytes.

Falsifiable predicate: for this new supported source program, public Rust checking and raw emission reach the actual Source6 parser/lowerer/preparation APIs, with the complete emitted Source6 semantic AST and ordered financial proposal/projection matching independently frozen expectations. All actual results remain captured. Full wire whitespace, full Rust CheckReport fields/spans, whole Core Program/post, Target/Artifact, measured work/post-work, qualification, security/native/proof/ledger results remain unasserted.

No new runtime, provider, SDK, native, resource or financial allocation is consumed by authoring. Root owns any later build/execution. No new accrued-repayment support or security test is included. All U0–U7, M1–M17, MC/SP/G, full Artifact/four-golden and original staged gaps retain their dispositions.
