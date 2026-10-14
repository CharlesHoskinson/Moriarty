# FOREMAN_REPORT

Worker: Grok 4.6. Task: finish the interrupted binding/reserve correction. Scope: generated Compact wrappers and compiled-runtime tests only.

## Result

The timeout partial kept R1 constructor digest, R2 reserve equality, and R3 shared validation. Root verify then showed generate CLI success twice and a silent build CLI. `node build.mjs --skip-zk` exited 0 with empty stdout and created no artifacts. Runtime import failed `ERR_MODULE_NOT_FOUND`.

`isDirectRun()` defaulted `metaUrl` to `generate.mjs` `import.meta.url`. Direct build compared the wrong module and skipped compilation. The author source now requires an explicit caller URL. Direct `node build.mjs` enters the CLI. Import of `build.mjs` does not compile.

Two generate runs wrote wrappers equal to owned `loan.compact` and `swap.compact`. Fresh skip-zk compile wrote artifacts under `state/build/binding-completion/20260908T084719Z`. Compiler outputs exist. Fake compiler counters are not compile evidence.

The compiled-runtime suite reports 24 passed tests and 0 failed tests. Independent GPT-6 Astra result review is required. SP05, ledger, Preview, proof, wallet, full BNF, K, and sprint acceptance remain open.

## Defect

Build imported `isDirectRun` from `generate.mjs`. The default `metaUrl=import.meta.url` was evaluated in `generate.mjs`. Direct `node build.mjs` returned false for the CLI guard. Exit 0 with empty stdout did not prove a build.

Late-swap tests first reused pre-close balances against zero kernel reserves. That failed with `ENTRY_RESERVE_A_MISMATCH` before `epoch is closed`. Tests now carry post-close 0/0 balances from actual effect deltas.

## Fix

`isDirectRun(metaUrl, argv1)` throws without an explicit caller module URL. `generate.mjs` and `build.mjs` pass `import.meta.url`. Constructors assign the pinned program digest and reject the 0x7f-repeat-32 value. Duplicate binding program digests must match metadata. Swap and close require both entry balances to equal kernel reserve fields and check post-effect continuity. Shared validation hashes source, metadata, bound programs, kernels and arithmetic. Build compares owned wrappers to fresh generation before the compiler.

## Tests

Historical receipts stay in `test-evidence.json`. Old missing-artifact RED, the 10-pass/5-fail diagnostic, the 17-pass completion, empty `root-compile3.stdout`, and the 22-pass/2-fail late-swap run are distinct.

Current green run: 24 tests, 24 pass, 0 fail, exit 0. stdout sha256 `a014dbb613c4f37ae760ee9b0013bbb8c528989077900aaacb8ca87f738641f7`. Artifacts `MORIARTY_CUSTODY_ARTIFACTS=/home/charl/.local/state/moriarty/sp05-kernel-custody-20260908/build/binding-completion/20260908T084719Z`.

Direct CLI regression: `node build.mjs` without `--skip-zk` exits nonzero. Import of `build.mjs` writes `imported-without-cli` and does not compile.

## Reproducibility

- loan wrapper sha256 `c1485f2915cedf173b858dfdbfdb9cf135915e109dea18bf00c8d237f302759e`
- swap wrapper sha256 `29b4be0dafcb6013577f67d0446622be7cf9b8ef0bb3fc1fb81a32801a879a56`
- arithmetic sha256 `b2969d0bc346fa93d88546578c9f9d29f1b74d7bc3f88f2e04709fa0932d9415`
- loan kernel sha256 `77e44b28b191b6c7ca904f92356f0da006a13839ff241393e0cf6088959cf363`
- swap kernel sha256 `090b9e2ecf5a0850bee0d46570ca9edb3ac374743e2ed29235913c6e1bb58741`
- compile stdout sha256 `c1fb25412e0258964367e576b4c152bd1d1ee4ceafa3e0b6cad7e937e3995f8d`
- compile exit 0, 630 stdout bytes, 0 stderr bytes, 2 skip-zk compiler invocations

## Secrets

Role secrets are generated in memory for each test. They are not in bindings, evidence, or this report. Constructor addresses are synthetic. They are not wallet custody keys.

## Open

Independent GPT-6 Astra must review this result. Merge status is not product acceptance.
