# Bounded liability-read statement model

Repository observation and model experiment, October 14, 2026. This new model describes the Core source semantics of `liability_present("d")` and `liability_amount("d")`, with ordered statement evaluation. Its executable checks are Quint model evidence. They are not Rust test evidence, a refinement proof, native constraint qualification, PCD evidence or ledger acceptance.

## Source correspondence

| Model definition | Inspected Core source | Covered behavior |
| --- | --- | --- |
| `Present`, `Amount`, `Read` | `crates/mori-core/src/parser.rs::Parser::expr`; `ir.rs::Expr`; `runtime.rs::eval_cached` | Two distinct typed read leaves, absent false/zero and present-zero true/zero |
| `evalTick`, `evaluate` | `runtime.rs::evaluate`, `eval_cached` | Left-to-right eager expression traversal; first-visit ExprId caching within one statement; new cache at each statement |
| `Value`, `binaryDefined` | `runtime.rs::eval_cached` | Checked UInt128 addition, subtraction and multiplication; typed equality, Boolean and/or, unary not |
| `applyStatement` | `runtime.rs::simulate` | Let captures, state writes, requires/ensures, event payloads, liability accrual/discharge; operands evaluated before their own effects |
| `validPre`, `begin`, `refuseAdmission` | `runtime.rs::liability_identities`, `scenario_snapshot` | Fixed declared asset/debtor/creditor identity abstracted to an opaque identity token; hostile incoming replacement refuses before step 0 |
| `refuse`, `commit`, `refusalIsAtomic` | `runtime.rs::simulate` | Working snapshot is a clone; error exposes no committed execution or effects; successful action returns final snapshot/effects |
| Separate literal controls for captured/fresh/zero/wide reads | `crates/mori-core/tests/liability_reads.rs`; `runtime.rs::eval_cached` and `simulate` | The Rust test source supplies captured 7, fresh 4, tombstone zero and exact wide read expectations; these files were inspected, not executed in this experiment |
| Self-read-before-effect model control | `liability_reads_test.qnt::selfReadBeforeEffectTest`; `runtime.rs::simulate` liability/discharge arms | The model test supplies the literal 7→14 doubling control; the runtime evaluates the operand before checked accrual. The doubling case is not attributed to the Rust Core test file |

The native consumer design was also inspected at `.moriarty-dev/runtime/language-wave1-20261014/native-duty-read-consumer-design.md` and `.json`. Its storage, transcript, signing, one-payer, and constraint obligations remain separate. No native producer/compiler correspondence is claimed by this table.

## Explicit abstraction and bounds

There is one sequential action evaluator with shared local state, no actors or messages. An action statement is atomic. Internal DFS evaluation uses a pure function, so intermediate expression states cannot interleave with duty writes. The expression arena admits at most eight nodes, dependencies strictly precede their parent, and evaluation has 32 internal ticks. Traversal stops on its first error. The `model-fuel` error is excluded by the sampled safety invariant; it is an abstraction failure rather than a Core diagnostic.

There are two opaque declared duty names, 0 and 1, with fixed identity tokens 10 and 11. The full asset and debtor/creditor tuple is abstracted to one token. A pre-populated map represents absence with `present=false, amount=0, identity=0`; actual Core uses an omitted row. Present-zero rows stay present. Duplicate wire rows cannot be represented by this map and are outside this experiment.

Actions have at most ten statements, state names `x` and `y`, and distinct let names separate from those state names. The simulator ranges over 20 fixed case templates; this is not all Core programs within the bounds. UInt arithmetic uses the actual `u128::MAX`, with explicit samples at `2^64` and `2^127`. The finite template corpus and bounded statement count constrain generated values; arbitrary integers are not nondeterministic inputs.

The model covers UInt and Bool only, not assets, balances, transfer/fee financial projections, other Core value types, parser rejection/type checking, unknown duty strings, duplicate source declarations, source byte limits, events' encoded byte limits or all supported binary comparisons. It records committed event payloads and effect counts, not a native effect/operation transcript. Let names cannot be rebound in this profile. The correspondence is a reviewed modeling claim to inspect, not mechanically checked refinement.

## Predicates and controls

`freshReads` compares every successful statement's read observations to that statement's prefix snapshot. `capturedLets` preserves the originally captured local value across duty writes. `fixedIdentity` keeps every admitted current duty source-bound; its exception identifies an admission refusal that retains the hostile input unchanged. `refusalIsAtomic` keeps the committed snapshot, event list and effect count at the predecessor until success. `safety` combines these and excludes fuel exhaustion.

Tests use literal expected outputs and failure outcomes, including a write or event before a discarded overflowing let. Direct and sampled successful controls exercise unary `not`, UInt equality (including wide values) and Boolean equality with both true and false results. Test-only faults inject a stale read, recapture a let, change a duty identity, leak effects after refusal, and independently leak committed event payloads; each must violate its corresponding predicate. Those actions are absent from the production `step`. Invalid successful execution is disabled, while refusal is a separate observable terminal transition. Terminal actions are disabled: the `all { false, state' = state }` branch supplies the assignment required by Quint's effect type checker but cannot fire; it provides no stutter escape.

Every major production transition has a positive witness: admission, seven statement kinds, runtime refusal, admission refusal, and commit. The ordinary suite tests both committed and refused terminal states with `step.fail()`.

## Recorded execution and limits

Quint 0.32.0, TypeScript backend, seed 1010, model typecheck, 40 test definitions (100 samples for each of three nondeterministic tests), and 1,000 simulation traces with at most 12 transitions per trace were executed. Final results, exact command records, earlier failures and candidate hashes are in the task's `duty-read-model/round3/` runtime reports; the original frozen candidate and earlier evidence remain in `duty-read-model/`. All Quint commands held `/tmp/moriarty-language-wave1-build.lock`.

Observed result: all 40 model test definitions passed; no safety violation was observed in the sampled traces, and every major transition witness was reached. This is bounded sampled evidence. Neither `quint verify` nor exhaustive model checking was run. No Cargo build, Rust test, native VM/query/check, proof, key operation, installation, deployment or network execution was performed. Existing Rust counts and existing transition/G1/G2/G3 models supply no evidence for this new model.

Refresh this model and its literal controls when the cited Core read/evaluation/identity/statement behavior changes. Preserve the explicit abstraction limits and re-run the separate model checks before renewing this correspondence claim.
