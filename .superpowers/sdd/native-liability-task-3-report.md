# Native liability Task 3 author report

Status: **DONE within the delegated QA/model/docs/ordinary measurement author boundary**. Native resource decisions, the eligible actual proof experiment, fresh full Task 3/result/final Sol 6.1 HIGH and Grok 4.6 HIGH reviews, publication and merge remain owed to the controller. No author approval or roadmap completion is claimed.

Checkout `/home/charl/Moriarty/.worktrees/native-proofs-20261011`, branch `feat/moriarty-native-liabilities-20261011`; author base `4cd299cdb6b2743e520e98627c20662143b51f79`. The controller freezes the later full candidate independently. This report hashes authored payload files; its own hash and exact commit/tree are obtained from Git after commit, avoiding a self-referential digest.

## Startup, scope and authority

Loaded installed `moriarty-dev:develop`, AGENTS.md, scoped Task 3 requirements, exact Task 1/2 reports and controller-independent Quint outcomes. Applied installed `quint-modeling` **from requirements**, `quint-lang`, TDD and verification guidance. The already approved whole-record, shared-local-state, atomic-stage design supplied state grain and identity assumptions; no new design vote or framework was created. No whole plan or inherited old queue was loaded. Current Sol 6.1 HIGH / Grok 4.6 HIGH routing supersedes Astra. No subagents were spawned.

Refreshed guarded status: no pending transactions; old SP01 admission/accounting/history gaps remain unchanged and block dependent campaign dispatch. Authorized source/ordinary QA proceeded without a campaign or network action. Final raw status is `task-3-guarded-status.log`.

Owned files: the two new `liability_transition*.qnt` files, language QA README, `docs/NATIVE-CORE.md`, one isolated upstream shape-measurement test in `crates/mori-backend/tests/proving.rs`, and this report. Core/backend production files, manifests, Target schema, dependency pins and parameters remain unchanged. Existing independently evaluated settlement/zero examples and manifests remain unchanged. No external Fenrir files, shared checkpoint or plugin infrastructure were changed.

## Model intake and correspondence

Authoritative intake: `/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/quint-independent-expectations.md`. One stage operates on complete immutable local registry identity and presence/amount state. Communication/time/ledger actors and failures are outside the state shape; invalid stages have an observable refusal counter rather than intermediate publication. Registry name labels are integers abstracting exact static source names; exact asset network/issuer/reference/scale and debtor/creditor are record fields. Serialization, hash implementation and lexical sorting are not modelled.

Highest-priority requirements map as follows:

- Unique exact source registry with identical repeated declarations and maximum16: `sourceAdmitted`, `contextAdmitted`, `exactClosure` correspond to Task 1 `liabilities::registry`, actual registry resource metadata and full descriptors.
- Canonical absence versus occupied present-zero, UInt128 range and persistent prior-present slots: `canonicalFrame` and `persistence` correspond to Boolean/UInt opening/closing wires and complete frames.
- Sequential checked accrual/discharge and exact ordered effect-bearing statement identity: `canOperation`, `canStage`, `orderedEffects`, `descriptorSteps` correspond to compiler sequential wires and Task 2 `public_values` source-step/operation/full-frame reconstruction.
- Unchanged untouched duties and atomic refusal: `untouched`, `refusalAtomic`, guarded `refuse` preserve the full financial duty map and previously published effects on refusal.

Pure guards/updates are separate from thin actions. `canStage` checks every intermediate operation on scratch state; `applyStage` only publishes after the whole guard holds. Discharge never implies transfer or erases a slot. Empty duty operations represent an action without duty effects, including ordinary transfers; balance/fee/state/event correctness is deliberately outside this model and covered separately by Rust checks. Source and scenario admission are local context restrictions, not evidence of actual authenticated predecessor completeness.

Default sampling uses exactly2 slots, deltas0..4, both source operations and explicit context/order faults. Deterministic tests additionally cover0/16-slot state and admission17. Amounts use the exact mathematical UInt128 maximum `340282366920938463463374607431768211455`, with actual maximum, maximum+1, intermediate overflow and underflow controls.

No model cryptographic truth flag was introduced. Hash collision resistance, source-to-constraint correspondence, public input publication, native cryptography, signed authority, authenticated real predecessor/full duty completeness, ledger phases/unique consumption/history, recovery and private recursion remain external premises or open work. Sampled QA is not exhaustive verification or native proof evidence.

## Executable model checks and retained failures

Evidence directory for every log/artifact below:
`/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011`.

Quint0.32.0 is installed. Incremental type scaffold typechecked. Its first default-Rust run failed because the exact UInt128 literal exceeds the evaluator's i64 limit (`task-3-quint-types-run.log`); no limit was weakened. Repeating with `--backend typescript` succeeded (`task-3-quint-types-typescript-run.log`). Scaffold-only step was temporary; final step performs actual accepted/refused stages.

Test-first ordered-progress control initially encountered syntax/list-operator setup errors, retained in `task-3-quint-red.log`, `task-3-quint-red-behavior.log` and `task-3-quint-red-behavior-2.log`. After those were corrected, the control executed and failed its amount/effect expectation because the stub did not implement ordered progress (`task-3-quint-red-behavior-3.log`). Implemented the guard/update stage, typechecked and ran100 samples/5 steps (`task-3-quint-logic-typecheck.log`, `task-3-quint-logic-run.log`). Initial expanded suite passed23 tests; adding empty-registry and invalid-delta controls produced final25.

Final exact new commands (paths relative to checkout):

```sh
quint typecheck experiments/moriarty-language/formal/quint/language/liability_transition_test.qnt
quint test experiments/moriarty-language/formal/quint/language/liability_transition_test.qnt --main liability_transition_test --backend typescript --match 'Test$' --seed 1010
quint run experiments/moriarty-language/formal/quint/language/liability_transition.qnt --backend typescript --invariant safety --witnesses accruedWitness dischargedWitness partialWitness fullWitness zeroCreationWitness refusedWitness --seed 1010 --max-samples 10000 --max-steps 12 --verbosity 1
```

Exact expanded argument vectors/exit codes for all five model typechecks and deterministic suites are preserved in `task-3-quint-commands.json`. Final new logs `task-3-liability_transition-typecheck.log` and `task-3-liability_transition-tests.log`: exit0, **25 passed**. Four unchanged historical model suites: acceptance30, lifecycle11, batch18, native_transition5 (**64 passed**), each typecheck exit0. Historical model/results files were not rewritten. Their current rerun is compatibility evidence, not new revision2 native evidence.

`task-3-quint-simulation.log`: exit0, no `safety` violation at exact10000 samples/12 steps/seed1010 (hex0x3f2), default `init`/`step`, TypeScript evaluator. All action-based witnesses have nonzero counts: accrual9927, discharge9569, partial8890, full1565, absent-zero creation1778, refusal9985 of10000 traces. Predicates require accepted/refused action history and are not already true at init. No `quint verify`/Apalache was run.

## Exact upstream circuit measurements

Sole author Cargo ownership; persistent target `/home/charl/Moriarty/.worktrees/readiness-20261010/target`.

```sh
env MORIARTY_LIABILITY_MEASUREMENT_DIR=/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/task-3-measurements cargo test -p mori-backend --locked --offline --test proving liability_fixture_shapes_without_parameters_or_proving --target-dir /home/charl/Moriarty/.worktrees/readiness-20261010/target -- --nocapture
```

`task-3-measurements.log`: **1 passed**, exit0. Actual committed width1 zero-lifecycle and width2 settlement sources were compiled without modifications. The width16 program is independently authored in the test:16 actual immutable duty declarations, alternating input/zero accrual, distinct debtors/creditors; a3-operation progress action and an untouched observation action all use complete16 frames. It is not padding or a dropped-duty surrogate. Optional explicit output env writes exact source/canonical target/artifact copies; absent env creates no output files.

No parameter provider, keygen, prover or verifier is invoked by the measurement. Only `compile`, pinned upstream `IrSource::load`, `ir.k()`, `ir.model().k()` and `model.rows()` are used.

| Exact fixture/action | C | Optimal/model k | Rows | Instructions | Public inputs | Effects | Source steps |
|---|---:|---:|---:|---:|---:|---:|---:|
| zero-lifecycle/close | 1 | 10 / 10 | 724 | 132 | 28 | 6 | 7 |
| settlement/seed | 2 | 10 / 10 | 605 | 97 | 21 | 2 | 2 |
| settlement/repay | 2 | 11 / 11 | 1059 | 137 | 28 | 5 | 6 |
| settlement/observe | 2 | 9 / 9 | 328 | 60 | 13 | 1 | 1 |
| complete-sixteen/originate | 16 | 12 / 12 | 3568 | 636 | 131 | 16 | 16 |
| complete-sixteen/progress | 16 | 11 / 11 | 1773 | 393 | 79 | 3 | 3 |
| complete-sixteen/observe | 16 | 11 / 11 | 1352 | 333 | 67 | 1 | 1 |

All closing/opening duty descriptors have actual C entries, including untouched actions. All measured circuits are within existing k19 engineering ceiling; this fact does not authorize native work, establish proving feasibility or measure replay/transaction cost.

Exact target: `Target::ledger9("local", "native-liability-qa")`, pinned Ledger revision04c9c5d9bcebb8d4427d8589fb54d58a55599c14 / ZKIR3.0.0-rc.2; canonical target SHA256 `0a79a6226f66efb8c3c6d0a08291a41d2705073c01805fef2d2b36d3cd642aec`.

- `zero-lifecycle` source `9c9bbec60457dccd8125c45a21b23da41fd094be15347edde1815f7e3f81068f`; canonical artifact `54c1e82711ed5b237951e8a1961b0bf870c47094f1201763ef029304c260340d`.
- `settlement` source `cf653de549a82ac9535d868605d71c1df29d4bf3c0d6df4cf6a1a894e619ccb2`; canonical artifact `3562cbe752a0a3e21eef1122ddefbb4f01f6de81cafd9795af23eaeeb88d4cb4`.
- `complete-sixteen` source `3b575b28031f9a139e6386d23755ee1af239db19a05e3755b5ae2577acc69630`; canonical artifact `af0811ae1458b33443510e075d94f64d0697e2e3d585e0d4cb3f6db2857fbf97`.

Every action's exact IR digest, full liability/effect frame digest, balance frame count, instructions/public positions, action and steps are in `task-3-measurements/measurements.json` and the raw log. Exact authored16 source is `task-3-measurements/complete-sixteen.mori`, with canonical artifact and Target alongside it. These runtime files are retained locally, not native authority receipts. Committed source/scenario/manifest digests and all authored payload hashes are also in `task-3-authored-candidate.json`.

## Full ordinary and beta compatibility

```sh
cargo test --workspace --locked --offline --target-dir /home/charl/Moriarty/.worktrees/readiness-20261010/target
```

`task-3-workspace.log`: exit0, **225 passed,0 failed,3 existing ignored**,29 test/doc-test result groups, no Rust warnings. Includes actual compiler complete-frame/honest-preimage checks, ordinary parameter-absence refusals, CLI authored examples/manifest/strict revision checks and existing legacy workspace suites. Ignored/native tests were never run. Existing corrupted dummy-header parameter rejection tests use fabricated test files; no actual SRS was accessed.

Actual beta signature helper state was inspected first. Shared Rust workspace target contained neither verifier nor intent-fixture; retained actual missing-path result in `task-3-tools.log`. Located already built signature-only binaries under `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/`, preserved their real byte hashes, and did not build/download new helpers. These signature tests are explicitly permitted compatibility scope and do not execute native transition proving.

```sh
env MORIARTY_CRYPTO_BINARY=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto MORIARTY_INTENT_FIXTURE=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/examples/intent-fixture MORIARTY_REQUIRE_NATIVE=1 npm test
env MORIARTY_CRYPTO_BINARY=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto MORIARTY_INTENT_FIXTURE=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/examples/intent-fixture MORIARTY_REQUIRE_NATIVE=1 npm run typecheck
```

Run from `packages/moriarty-beta`: `task-3-beta-test-initial.log`, **193 passed,0 failed,0 skipped**, exit0; `task-3-beta-typecheck.log`, exit0. The suite runs native signature adapters/adversarial20-artifact868-leaf controls, local preparation/atomic store, source-built LSP/MCP protocol helpers, readable intent CLI and packed fixture checks; prepack itself builds the beta bundle. `MORIARTY_BETA_PROTOCOL_CLI` was unset, so protocol tests used their existing source-built server mode. No separate product transport or proof campaign was added.

Node v24.21.0, npm11.19.0. Signature binary SHA256 `42fa598d8b54334650f60bc13428bff3d79bb240a5bb0c203fc84684b5a30339`; intent-fixture SHA256 `e084f2ebc9dc74542e0e84c7d7607c11343bd8121021d11bedec33fa8a26c70c`. Fresh counts above are not inherited from prior PR25/Task2 logs. All initial failures and setup omissions remain retained.

Static `rustfmt --edition 2024 --check crates/mori-backend/tests/proving.rs` and `git diff --check`: exit0. No production source repair was needed.

## Authored payload hashes and remaining work

Final evidence log SHA256 values are preserved in `task-3-authored-candidate.json` together with the payload/source/scenario hashes.

- `crates/mori-backend/tests/proving.rs`: `213d17e41617af7b73ab7d52f75408fd4a83a21b1ed4d10b03479379973ac727`.
- `experiments/moriarty-language/formal/quint/language/liability_transition.qnt`: `54844f47829f1d716028c3028747aebd2281c382df403728d066a4973caa6e04`.
- `experiments/moriarty-language/formal/quint/language/liability_transition_test.qnt`: `351748fc85a08deaca5f9addfeb2a0677269abeffbc915fae520954ca0465394`.
- `experiments/moriarty-language/formal/quint/language/README.md`: `5630cc33727351f1714c81965417a38c52be849d9d7d74f6db56815150c010dc`.
- `docs/NATIVE-CORE.md`: `f54e65e0b555da4b9126ae4f8a37a448104dd6bd05b16ca10339a0f76773e85a`.

No real SRS access, key generation, genuine transition proof/proof verification, ignored-test execution, download/network call, new resource dispatch or ledger transaction occurred. The prior native payment allocation remains consumed. Root must obtain both selected separate votes for an exact measured native liability positive and specified fresh-key/changed-statement controls, preserve authority/accounting/resource bounds, and dispatch only the admitted experiment. Missing votes/shape drift block it. Current source approval cannot replace actual-result/full-candidate audits.

Current docs distinguish artifact/ABI/relation/proof/binding2 from historical1 evidence, supported source-registry semantics from broader reference semantics, and model/runtime correspondence from authenticated acceptance. Signed authority/caps/consent, actual predecessor/full real duty completeness, ledger phases/unique consumption/history, recovery races/coordinator, recursive/private history, live Preview financial settlement and full U0–U7/M1–M17/roadmap exits remain open. No release gate is closed by these ordinary results.

After the clean author commit, Cargo and owned source files are released to root. The current full Task3/result review and roadmap remain active.
