# Native liability Task 2 implementation report

Status: **DONE** within Task 2 ownership. This is author implementation and ordinary test evidence. Independent source/result approval and Task 3 remain controller-owned.

Checkout: `/home/charl/Moriarty/.worktrees/native-proofs-20261011`.
Branch: `feat/moriarty-native-liabilities-20261011`.
Author base: `4c636b8aad215729f7043fa005f996358aaa220f`.
Compiler author interface: `02ffcd9f70968d94cb9d10b1f398c2cae5ebdd0a`, with independent Task 1 review read before implementation.

## Scope and startup

Loaded installed `moriarty-dev:develop` and the current `AGENTS.md`; refreshed guarded status, including JSON. No pending transactions. Existing old SP01 stale-input/accounting/history stops were preserved; no dependent campaign was dispatched. Applied the fresh Sol 6.1 HIGH and Grok 4.6 HIGH review override; Astra reviews remain historical. Read the scoped brief first, frozen compiler report, independent Sol Task 1 review, and root's independently hand-derived expectations. Did not load the whole implementation plan or old queue. Used TDD, systematic failure inspection and verification guidance within the approved design. Controller retains shared checkpoint ownership.

Owned changes only:

- `crates/mori-backend/src/proving.rs`.
- `crates/mori-backend/tests/proving.rs`.
- `crates/mori/tests/native_proofs.rs`.
- New `examples/core/native-liability/`: two source fixtures, eight supplied scenarios, and six-case `mori-tests.json`.
- This scoped report.

No adjacent CLI consumer changes were necessary. Core grammar, interpreter, source semantics, compiler Task 1 code, Target, dependency pins/manifests, parameters, external Fenrir, maintained product documentation and Task 3 QA were not changed. The two source structures/scenarios were copied from root's independent reference folder as explicitly permitted by the brief. Accepted manifest expectations come from its hand-derived `expected.json`, not from actual simulation output.

## Exact public schema and commitments

`prove`, `verify`, `ProofOptions`, the explicit Target argument, and complete `ProofReceipt.execution` keep their signatures/shapes. `ProofBundle` and the private canonical `Claim` gain these required fields:

```rust
abi: String,                  // moriarty-public-abi/2
relation: String,             // moriarty-zkir-relation/2
binding: String,              // moriarty-native-transition-binding/2
profile: String,              // trusted Target.profile, currently ledger-9
registry: Vec<LiabilityIdentity>, // canonical sorted source-derived identities
```

Those are public members on `ProofBundle`. Registry identities reuse the frozen exact typed compiler `LiabilityIdentity` (name, full AssetId including scale, debtor, creditor); they do not contain caller-selected amounts. The serialized `format` is now `moriarty-native-transition-proof/2`. The remaining fields are unchanged: `source_hash`, `program_hash`, `target_hash`, `scenario_hash`, `execution_hash`, `action`, `circuit_k`, plus `proof_hex` only on the bundle. Unknown fields remain refused.

The binding preimage is canonical serialization of:

```text
("moriarty-native-transition-binding/2", Claim)
```

It preserves SHA256 of raw source bytes, canonical compiled program, exact Target, complete Scenario and complete Execution (including all ordered statements, step snapshots/effects, pre/post and otherwise untouched values), selected action and actual reconstructed circuit k. SHA256 is reduced into the pinned Fr as little-endian low128 + 2^128 * high128; the ordinary binding test explicitly checks both limbs. Existing ABI position/value vectors outside duties remain unchanged. The test's old v1 domain literal is intentionally replaced with v2; historical v1 receipts/evidence were not modified.

Task 1's relation-context circuit constants already bind the exact relation, ABI, ceiling16, canonical complete registry, Program, Target and action. The native statement still consists of the host binding followed by the reconstructed circuit values. The proof's internal key location changes to `moriarty-local-native-transition/2`. This is a namespace revision, not an exported verifier key. Production verifies using a fresh VK from trusted recompiled IR and explicit local parameters; no bundle-provided registry, circuit, public statement, key, status flag or report is accepted as verification authority.

`ProofBundle::from_json` validates exact proof/ABI/relation/binding revisions after bounded duplicate-key-safe decoding. Public `verify` performs that revision check before parsing/recompiling even the source. It then reconstructs and compares every claim field including profile and registry before parameter loading. Ordinary tests fabricate an exact metadata bundle with a one-byte placeholder and stop at missing SRS; that placeholder is never passed to a real native verifier. Old version1 inputs, including programmatically constructed bundles, are refused before parameters. Old source text may still compile/proceed under the explicit v2 boundary.

## Actual trusted reconstruction

Removed all three blanket liability refusals and replaced them with source-derived registry consumption:

1. Source parsing/compilation remains the production trust boundary. The complete all-action sorted source registry validates the selected scenario before reference simulation. Unknown names, duplicate names, or any immutable AssetId/debtor/creditor mismatch refuse with `mori::backend::unsupported`, including untouched duties.
2. Each scenario, predecessor, successor and every ordered step snapshot becomes the entire fixed-size registry vector. An absent name is `(false, 0)`; any supplied exact name, including amount0, is `(true, amount)`. Presence never follows amount positivity. Every descriptor identity/order must match the canonical registry.
3. Execution step count, index, exact source Statement and flattened ordered effects must match the selected source action. Untouched registry slots remain equal at every step, and closing slots equal the final step (or opening slots for an empty action).
4. Opening/closing presence and UInt amount use frozen absolute descriptor positions and roles. Existing `set` validates bounds, role, exact name/asset, value type and one assignment per position; all positions must be filled. Duplicate and out-of-bounds positions refuse.
5. Duty effects distinguish the actual source `Liability` from `Discharge`, compare the trusted descriptor's exact operation/identity, compare actual before/after snapshot membership/amounts with the full core Effect, and check the reference step's evaluated UInt delta using checked addition/subtraction. Discharge requires opening presence and preserves closing presence. Full discharge remains present0; no payment or deletion is implied.
6. Effect descriptors must have the exact source-step sequence of effect-bearing steps, as well as exact count, identity and per-step effect. This prevents identical zero-valued core Effects from hiding swapped operation descriptors. The four duty fields are reconstructed in frozen before-presence/before-amount/after-presence/after-amount order. Non-duty descriptors must not carry duty metadata.

No separate authority evaluator was added. Broader reference execution remains unchanged: wrong creditor and an unknown untouched shadow name still simulate successfully, while the fixed proof profile refuses them before SRS. Semantic zero-guard, absent-discharge and overdischarge failures retain scenario/refusal behavior. Complete receipts and all existing bounded/exclusive publication behavior remain intact.

## Test evidence and retained failures

Persistent evidence root:
`/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011`.

Shared Cargo target:
`/home/charl/Moriarty/.worktrees/readiness-20261010/target`.
The author held sole Cargo ownership throughout implementation and did not spawn agents.

Actual red logs:

- `task-2-red.log`: backend lib 3 passed, 1 failed at unsupported new public binding roles. Integration binary was not reached because the lib failure stopped Cargo.
- `task-2-red-public.log`: registered duties expected `parameters` but received the old blanket `unsupported` refusal (0 passed, 1 failed).
- `task-2-red-schema.log`: stale proof1 was admitted by `from_json` (0 passed, 1 failed).
- `task-2-red-cli.log`: actual CLI liability proving failed at the old blanket refusal rather than missing SRS. For this baseline test, temporarily restored the exact base `proving.rs`, then restored the candidate in a Python `finally` block. No base source changes were committed.

All intermediate attempts retained:

- `task-2-green-attempt-1.log`: initial 4 lib + 8 ordinary proving passed; 1 pre-existing native test remained ignored. One unused Snapshot import warning was removed.
- `task-2-green-attempt-2.log`: 5 lib passed, 1 failed. Swapping two identical zero-valued descriptors was accepted. Root cause was equality of core Effects without a descriptor-step sequence check. Added exact sequence comparison; retained the failing control.
- `task-2-green-attempt-3.log`: 6 lib + 8 ordinary proving passed; 1 native test ignored.
- `task-2-cli-attempt-1.log`: 7 CLI ordinary tests passed, 1 failed solely at the new simulation test's incorrect top-level `post` assertion; actual CLI `simulate` returns the existing `execution` envelope. No product change was needed.
- `task-2-ordinary.log`: repeated that test-only failure because the first text replacement corrected the multiline effects assertion but did not match the formatted one-line post assertion. Inspected the exact current line and applied an asserted exact replacement.
- `task-2-ordinary-attempt-2.log`: full ordinary suites passed (80 passed, 0 failed, 3 ignored).

Final clean command, after adding interpreter-only honest-preimage checks of production-prepared settlement/zero vectors and exact two-limb binding equality:

```text
cargo test -p mori-core -p mori-backend -p mori --locked \
  --target-dir /home/charl/Moriarty/.worktrees/readiness-20261010/target
```

`task-2-green.log`: **80 passed, 0 failed, 3 ignored**, 15 binary/doc-test results, no warnings. The three ignored existing tests were not executed; no new ignored/native tests were added. Totals: core30, backend36, CLI14. `task-2-static.log`: exact three-file rustfmt check and `git diff --check`, both exit0.

Final green log SHA256:
`1539e7ad426859a88ac5cbec42641e641d926bd39999769984514b18ea05b068`.

Coverage includes exact partial/full/zero complete poststates and ordered effects from independent hand expectations; payment10->3, full payment10->present0, untouched reserve present0; zero absent->present0->present0->present2->present0; all-step snapshot names/full tuple/duplicates/presence mutation; exact source operation and step delta; duplicate/unknown/out-of-bounds/wrong-order descriptors; untouched action framing with nonzero, zero and absent duties; all new and retained claim commitments; strict stale/missing/unknown schema; direct version1 verification refusal before invalid source handling. Ordinary `ProofPreimage::check` consumes production-prepared vectors, without parameters or native proving; it establishes honest vector/IR agreement only.

Actual CLI tests invoke the production executable for compile, simulate, test and missing-SRS prove preparation. All existing FIFO, Unicode escaped-receipt bounds, dangling symlink, existing destination, duplicate JSON key and exclusive-file tests pass. Supported partial/full/zero contexts reach missing SRS; unsupported and semantic cases refuse before SRS with no output publication.

Supplemental direct CLI evidence: `task-2-cli.log` and `task-2-cli/commands.json`, `summary.json`, preserved raw stdout/stderr and emitted v2 artifacts. Nine recorded commands passed their expected exits: `mori test` (six manifest cases), compile both sources, simulate three successful scenarios against the independent complete expectations, and three prove preparation attempts stopping at explicit missing SRS with no proof files. Exact binary SHA256 is in `task-2-cli/summary.json`; Target is the existing unchanged `task-1-cli/target.json`.

## Limits and next independent boundary

No real SRS access/download, key generation, native proof/verifier execution, ignored tests, native/resource experiment, network action or guarded financial dispatch occurred. Missing-SRS preparation and ordinary IR/interpreter checks do not demonstrate native soundness, measured proving feasibility, proof roundtrip, ledger acceptance or PCD. The prior payment allocation remains consumed.

Task 3 and controller review must separately cover the exact full candidate, QA/Quint correspondence and native resource measurement scope: distinct C0/C1/C16 registries, zero/nonzero/absent persistence, same-name sequential accrual/discharge, full discharge to present0, mixed payment/fee/duty/state/event steps, and exact circuits/version/profile bindings. Any SRS/keygen/prove/verify experiment requires both selected resource votes on measured exact candidate circuits before dispatch; this report supplies no such authority.

Authenticated real predecessor/full actual duty completeness, signed authority/caps/consent, ledger phases/unique consumption/history, recursive/private history, recovery coordinator/races, live financial Preview settlement and full roadmap acceptance remain open. Source registry closure is an explicit profile restriction, not evidence that off-registry real obligations do not exist. No author approval or whole-roadmap closure is claimed.
