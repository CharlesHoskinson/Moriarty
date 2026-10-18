# Partial escrow release: ordinary Core regression source

This unapplied candidate adds one real `mori_core::parse` -> unmetered `simulate` regression at immutable source `64c6dedff1d99b33c23832330d7911a5fad9ed59`. It covers a partial payout, a distinct source Fee, explicit duty discharge, retained unpaid duty and unchanged observer/reserve balances and ordinary state. Root alone integrates; no Cargo, Node, model, SDK, ledger, proof, provider or shared Git/worktree mutation occurred.

`INPUT-ONLY.json` and `INPUT-FREEZE.json` fixed the complete source and opening Scenario before the separate oracle read assertions. Program SHA256 is `d1319f22563072bb397dc31b90f467a9d147360d3160aa7ada8f2ba5371b8a08`; Scenario SHA256 is `9b0b8ae229d56b385afa664c532aaa8878cb60b33b5b9fb6bf313e1e19814156`. Neither input changed afterward.

The separate oracle then froze complete semantic expectations before root checks or author-output exposure. Its expected file SHA256 is `00633e388be889b54dbd19522c4f446fb39dd55020020b23357144e16acf7ae4`, and FREEZE.sha256 SHA256 is `367b32a2089c210edd4830c4f2274d73ccf2c97d3132323f22ebc723efe54959`. Sixteen freeze members were checked; the full small immutable oracle tree is copied under the dedicated fixture's independent directory, preserving the source/inputs, normative premises, prior-exposure disclosure and provenance. No independent cryptographic/provider authentication or actual-result approval is inferred from hashes.

## Actual consumer and assertions

The source uses existing Core/1 Requires, Let, Transfer, Fee, Discharge, Next, Ensures and Emit. `SOURCE-PINS.json` binds exact Git blobs and SHA256 values for the parser/runtime/IR, public exports and relevant build inputs. In particular, existing validation preserves external duties for discharge-only programs; no source-level liability read or new declaration semantics are invented.

The test compares full independent pre/post snapshots, every one of eleven ordered complete snapshot prefixes, every ordered per-step delta effect vector and every ordered cumulative vector, then all six complete effects. It also ties source/input bytes to oracle custody and checks that the borrowed Scenario remains unchanged. Step.value, ExprId and Statement IR have no independently qualified expectation here.

The frozen normal arithmetic pays CASH41, charges a separate CASH3 Fee and explicitly reduces merchant-due97 to56 only at Discharge. Escrow ends CASH93, merchant52 and treasury8. State paid becomes50 and fees7; marker88 and epoch6 remain. All nine balance rows, four state fields and both duties remain in every complete prefix, including all BONUS balances, CASH observer23/reserve19 and unrelated warranty8. This is local supplied-state accounting, not evidence of escrow custody or delivery.

## Patch and root command

`INTEGRATION.patch` contains the full new test `crates/mori-core/tests/utxo_escrow_partial_release.rs`, complete fixtures under `crates/fixtures/utxo-escrow-partial-release/`, and the immutable independent oracle subtree. No production API, dependency or feature changes are proposed. `CANDIDATE-FILES.json` gives every exact file/hash.

`PROPOSED-NORMAL-COMMAND.json` supplies the exact direct installed1.99 Cargo argv and replaced environment, selecting only `--test utxo_escrow_partial_release` and the exact function `partial_escrow_release_preserves_residual_duty_and_observers`, frozen dependencies, one job/thread and no default features. Root must choose its own integration cwd, existing bounded resources and output directory. The optional `MORI_UTXO_PATTERN_EXECUTION_OUTPUT` captures full actual Execution JSON by create_new/write/sync before semantic assertions; unset means no capture-file write. It creates no directory and overwrites no receipt.

Cargo would compile/link ordinary test dependencies and may execute registry build scripts/procedural macros; this lane does not run it or certify those transitive bodies. The selected normal API uses in-memory parsing, scenario normalization and arithmetic. No optional Midnight compatibility feature, CLI/provider/native/proof/model lane is selected. Formatting, compilation, patch application and all tests remain unrun.

## Scope retained

`RETAINED-SCOPE.json` retains all twenty original identities and unchanged open acceptance. Original148requirements/292scenarios, legacy /5-/6 predicates, F03 AccrualFirst, complete U/M/MC/SP/G, financial/composition/target obligations and all private/PCD requirements remain distinct. This ordinary example supplies no security judgment, ledger atomicity, native/target acceptance or parent closure.
