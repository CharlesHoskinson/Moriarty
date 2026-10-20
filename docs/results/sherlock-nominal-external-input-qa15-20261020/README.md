# Nominal external-input handler QA — 2026-10-20

The public test consumer has **14 retained QA4 passes plus one new QA5 test 12 pass**. QA5 compiled the changed consumer, listed all 15 tests, and ran only `nominal_owner_and_genesis_refusals_preserve_payload`. The other 14 bodies are byte-identical and were not replayed. This is not 15 new passes.

| Actual cut | Observation |
|---|---|
| QA3 | Compile failed with five E0277 errors requiring `CompileFailure: Debug`; list and run remained unrun. |
| QA4 | Compile and list passed; runtime 14 passed and test 12 failed on its StepCap locator expectation. Its later genesis and layout assertions were not reached. |
| QA5 | Compile 0.851650222 s; list 15 in 0.003845450 s; exact test 12 in 0.010816806 s, with 1 passed and 14 filtered. |

QA5 compile stdout was empty. Its full 4,341-byte stderr retains four warnings and no errors; runtime stderr was empty. Root retained raw child results and full streams before post checks and acceptance. All 484 static pins, four fixed library inventories and generated hash/mode pins remained stable. The exact source is 44,367 bytes, SHA256 `5ca0dadc8807a9eb7bdc1da3d9ea410338c5e313dfe6bccbb404704128ddbf82`. Its `SOURCE_ONLY_UNRUN` header and dated source-freeze statuses are immutable history from before execution; the later actual receipts qualify only the scope stated here.

The consumer imports five real Args, handler, reader, publisher and display modules, plus two existing public fixtures. All seven match the checked public checkout. It reuses the qualified full normal Core 189/117 library `dca9…`; only the external consumer uses `cfg(test)`. This leaf rebuilt no Core library and ran no Cargo, SDK, native action or complete backend CLI binary. The current checkout differs from the frozen Core basis in `shared.rs`, `shared/fin_carrier.rs` and `shared/owner_host.rs`. There is no whole-current-public-tree build claim, and compiled artifacts are not distributed.

Test 12 checks StepCap's explicit field locator, the direct Tooling genesis fuel refusal and the typed layout mismatch. The designated source owner derived those expectations from the guards frozen before execution. A fresh nonauthor review approved the source/API and exact ordinary operations with limits. Prior reviews missed the Debug bound and locator contract; their full reports and the resulting failures are preserved. A source opinion is not a test result or independent semantic oracle.

Test 3 is shared-parser propagation; tests 13/15 are current shared-producer equivalence. The 23-key ProgramView object plus selected joins is not independent equality of all 23 fields. Synthesized-entry body errors, partial-prefix HostViolation and serializer failure remain unexercised. Full CLI, global H1, native proof/target, independent semantics and EARS/financial acceptance remain separate. No original requirement or refinement is closed by this package.

`actual/qa03`, `actual/qa04` and `actual/qa05` preserve selected COMMAND files, results, complete streams, acceptance records, dep-info and pin checks. QA3 partial dep-info is explicitly failed and unqualified. Binary hashes are provenance only; ELF, rlib and rmeta payloads are excluded. `history` retains source cuts, hunks and inverses. `source-inputs` contains the direct public inputs and selected complete public guard bodies, with exact excerpt origins. `reviews` contains public final responses and safe identity/terminal projections. Configured Muse identity and the returned `muse-wonder-1.0-contributor` facet remain distinct. Private provider logs/prompts, wallets, SDK payloads and protected oracles are excluded.

Raw commands retain their original absolute paths and limits. Their portable copies are audit records, not additional executed commands. Reproduction needs matching ordinary toolchain/dependency artifacts supplied separately and an explicit scoped operation selection; no runner or binary is installed by this package. See the [inspection guide](../../sherlock/how-to/inspect-nominal-external-input-qa15.md).
