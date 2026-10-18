# Ordinary Core consumer regression candidate

This unapplied source candidate exercises the real public `mori_core::parse` -> `mori_core::simulate` route at exact revision `e4d7a2b5709caa18a94f5256d03a0302364b8235`. It proposes one normal regression for the existing prefunded bilateral exchange source. Nothing was built, parsed, simulated or tested. Root is the sole integrator.

The expectations are **authored by this same source-preparation agent and are not independently frozen**. They are literal proposed outcomes, not observed evaluator output, an independently qualified oracle or accepted evidence.

## Complete candidate

`candidate/` mirrors four proposed repository paths:

- `crates/mori-core/tests/utxo_pattern_exchange.rs`
- `crates/fixtures/utxo-pattern-exchange/program.mori`
- `crates/fixtures/utxo-pattern-exchange/normal.scenario.json`
- `crates/fixtures/utxo-pattern-exchange/authored.expected.json`

The program and input are byte-identical to the prior source-example bundle. They exchange A10 from escrow-a to trader and B25 from escrow-b to provider, pay a separate A1 fee from fee-payer to treasury, change phase 0 to 1 and revision 4 to 5, and emit UInt10. All balances use complete exact asset identities.

The Rust candidate calls the actual parser on the `.mori` text. It decodes the complete typed Scenario, then calls the public `simulate` API. It never decodes a fabricated Program as a substitute for parsing or replaces simulation with a helper/evaluator. The expected fixture contains all public `Execution` fields: semantics, action, normalized prestate, poststate, all eleven complete Step records and all six ordered effects. Each prefix comparison includes literal expected parsed Statement/ExprId references, value, complete state, all nine balances, both liabilities and that step's ordered effects. The complete Execution equality catches additional/missing public fields, and the input scenario must remain unchanged.

No metering, replay, compiler, target or hostile/security lane is added. The earlier logical count34 remains an authored unexecuted calculation; this regression selects the unmetered public consumer and makes no `workRemaining` claim.

## Actual source route inspected

At the pinned revision, `lib.rs` exports `parse` and `simulate`. `parser.rs` accepts explicit `moriarty/1`, UInt state/input declarations, static exact asset records, requires/ensures, Transfer/Fee, next and emit. It validates the resulting Program and appends expression nodes in source order. State initializers are literal values rather than entries in the expression arena. The expected statement expression references 2/5/8/11/12/13/14/15/18/21/22 were derived by reading that allocation order; the parser was not executed to obtain them.

`runtime.rs::simulate` calls validation, selects the named action, normalizes the scenario and evaluates statements in order. Normalization requires explicit balances for all selected transfer/fee participants, preserves declared exact assets, sorts balances by asset then account, and sorts residual duties by name. It permits supplied external duties when no conflicting source identity is declared. `move_funds` performs checked debit/credit; Transfer/Fee each produce their distinct effect. `next` updates the current state immediately; emit records its typed value. The normal result contains complete prefix snapshots.

The pinned parser/runtime/IR/Core README and family design are byte-identical to the corresponding inputs read for main74a909. Full public source copies and hashes are included in `main-inputs/` and `SOURCE-MANIFEST.json`; the new revision remains the controlling pin. This is a source-level compatibility observation, not executed behavior.

## Affected and preserved finances

| Exact asset reference | Account | Pre | Proposed post |
|---|---|---:|---:|
| A | escrow-a | 10 | 0 |
| A | fee-payer | 5 | 4 |
| A | observer | 9 | 9 |
| A | trader | 3 | 13 |
| A | treasury | 2 | 3 |
| B | escrow-b | 25 | 0 |
| B | observer | 11 | 11 |
| B | provider | 4 | 29 |
| B | trader | 8 | 8 |

A remains total29 and B total48. `service-B` remains B2 owed by provider to auditor; `warranty-A` remains A7 owed by provider to trader. Every one of these fields is literal in every expected prefix. Marker77 remains unchanged. No duty is discharged and local phase1 does not mean debt-free termination. The six effects in order are A transfer10, B transfer25, A Fee1, phase state update, revision state update and exchanged event10.

## Supported and missing constructs

This candidate uses only inspected existing Core/1 public semantics. It does not add grammar, behavior, a model or a feature. Division/constant-product quotient, dynamic account selection, loops, general recursion and Core/1 branch/await statements remain outside this route. Unsupported constructs are missing capabilities rather than expressions replaced by host results; this program does not contain them. Core/2 stage admission is a separate consumer and no Core/2 result is inferred.

Core/1 has no legacy allowance/nonce/operation-history/lifecycle-work fields in this Scenario/Execution wire. The candidate therefore does not claim to compare such fields, translate legacy /5-/6 semantics or satisfy the exact original SB01 scenarios. `TWENTY-FAMILY-SCOPE.json` retains all twenty literal original identities, with every acceptance open. The exchange and fee primitives illustrate related portions of two families only.

## Boundaries and integration

This is ordinary developer-example regression source. Account and escrow labels are supplied local facts, not authenticated ownership or ledger state. No asset-owner authority, signatures, replay protection, cryptographic correctness, adversarial test adequacy or other security judgment is supplied. No compiled claim output, VI/VS consumption, ledger atomicity, actual body, native proof, inclusion/readback/finality, target qualification, universal certificate or PCD acceptance is supplied. All20 and broader obligations remain open.

Root may review and integrate the four candidate paths after the applicable full source audit. The next actual check is the real Cargo integration-test consumer under separately selected authority; this bundle prescribes no execution command. `cargo fmt`, `cargo clippy` and `cargo test` are all UNRUN, including focused compilation. The source may still contain compile/format/result issues. No dependency or feature changes are proposed. Literal fixtures follow the existing financial-pair direct-comparison pattern, with data under crates/fixtures and no module remount or external runtime path.

Only this new assigned directory was written. Git was used read-only to inspect the requested revision; no shared checkout, worktree, branch, index or commit was changed. No build/test/native/SDK/provider/model/wallet/network operation or dispatch occurred. The development skill, checkout/target AGENTS and read-only startup status were read. The installed rust checklist path was unavailable, so the exact target revision's checked-in checklist was read. Historical absolute pointers were not followed.
