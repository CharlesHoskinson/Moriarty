# Installed local call CLI consumer implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Root adoption root-design-adoption-01.json authorizes source implementation of design SHA c1f59c05b9864beefa71d856d210604a0c1a92bc7f701e5f6d8c4308d223b338 only; it grants no native allocation. The root owns review dispatch, native admission, publication and merge; Root-assigned worker sol_installed_parameters_impl owns parameters.rs only; sol_installed_call_quint_qa owns new installed_call_consumer.qnt/test and focused formal README. This author owns all remaining source/CLI/docs.

**Goal:** Implement actual `keygen-call`, `prove-call` and PK-free cold `verify-call` consumers of original-source ZKIR, a private complete-action installed-local capability, the existing SDK Intent prover and the installed-operation Real verifier.

**Architecture:** Extract a private validated-static binding and request-free full-frame admission from the existing relation/layout path, preserving legacy wrappers. Native derivation builds every source action's public operation and selected proving material; cold import repeats derivation before comparing opaque exported keys. A separate installed owner retains the actual admitted operation, while bounded ledger artifact code and three CLI consumers connect the existing raw owner, complete final readback and mandatory Real verification.

**Tech Stack:** Rust 1.99.0, existing Moriarty Core/parser/ledger crates, SDK `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, compatibility `a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1`, existing serde/SHA256/libc/futures/rand, locked offline Cargo, existing Quint source-call QA.

## Global constraints

- "Moriarty is the only implementation target. All U0–U7/M1–M17 remain active."
- "Native execution is not authorized by this document."
- "The old `moriarty-ledger-call-preparation/1` schema, wrappers and installed VK/IR refusal remain."
- New profile: `moriarty-ledger-installed-local/1`; qualification: `local-source-derived-installed-unqualified`; Anchor: `local-unqualified`; phase policy: `guaranteed-only`.
- "Every action is compiled and admitted, even when not selected."
- "The raw intent is expanded exactly once by SDK Intent::prove; processed calls remain inspection projections."
- "No larger-degree substitution."
- "Untrusted PK/VK bytes must not be decoded before this comparison."
- "Cold verification regenerates all public VKs only through keygen_vk; it neither reads PK files nor contacts a proving service."
- "Explicit dependency `proof-verifying` must remain enabled with no default features."
- "No second compiler/framework/nativeharness/providerfallback" (root task constraint).
- "No native k/model/synthesis/SRS read or decode/parameter initialization/keygen/prove/crypto verification, ignored native test, wallet/funding/network/apply may execute under ordinary source permission."
- "Publishing is create-new and final marker last; collision/symlink/partial failure preserves existing files and never reports native success."
- Existing 1,048,576-byte ordinary input and 16,777,216-byte ordinary output envelopes remain; new installed envelopes use explicit caller bounds.
- No fake derived-key capabilities, successful proofs or proof-success objects in ordinary Rust tests. Opaque module-private byte comparisons and compile-fail privacy checks are permitted.
- Every result/log/receipt uses a new runtime path and exclusive creation. Never overwrite failures or historical evidence.

## Frozen context and inspected evidence

Assigned checkout: `/home/charl/Moriarty/.worktrees/installed-call-sol-20261012`. Inspected clean HEAD `078de66b988087a0439f26d298e682fd1371bf88`, tree `e72c77c40baa1d5a66f7cdcecfcc34999bb1a1dd`; source base PR32 merge `b5b47acb51315dba89d8df85599226077b23f5d7`. This draft is outside that checkout. No code or native execution is part of this preparatory turn.

Design read in full: `docs/superpowers/specs/2026-10-12-installed-call-consumer-design.md`, SHA256 `c1f59c05b9864beefa71d856d210604a0c1a92bc7f701e5f6d8c4308d223b338`. Independent oracle read: `/home/charl/Moriarty/.moriarty-dev/runtime/source-proving-20261012/next-installed-consumer-expectations-01.md`, SHA256 `c52e7128771435df825f6b8bbfea55f6f8ccc0ef29e56cbc5603c4b382f1c9bf`. Its `next-installed-consumer-source-hashes-01.json` has SHA256 `0577f9655fb34a250ca4bc89f4940892ce33cb7c1b74e2ee0fc7d1150bcfbb01`. These are acceptance inputs, not votes. Current reviewer conclusions were not read.

All current `mori-ledger/src` modules were inspected, including relation arithmetic/event/process and private source SDK adapter; CLI dispatch, compile/prepare consumers, support/admission/relation/proving tests and pinned backend parameter reader were inspected. SDK interfaces inspected locally include IrSource k/keygen/keygen_vk, ProofPreimage proving/provider/resolver, Intent expansion, public_inputs, ProofMarker Real, ContractState/ContractOperation/LedgerState cloning and tagged deserialization.

Guarded status was refreshed in the assigned checkout: no pending transactions; stale loan binding/accounting/live-state/operational-history blockers remain and grant no native authority. Root's literal reference receipt `/home/charl/Moriarty/.moriarty-dev/runtime/installed-call-20261012/root-literal-reference-01/receipt.json` records actual two-source checks, six complete semantic cases and three refusal cases, not installed/native acceptance.

SDK cost fact: IrSource.k calls `optimal_k`, which may scan 9..=25 via `cost_model`; keygen/keygen_vk call k again. `max_degree` refuses the discovered degree before SRS/keygen, but cannot bound the internal model scan. Every such call/repetition requires later process containment. SDK `VERIFIER_MAX_DEGREE=14` describes embedded verifier parameter public-input support; do not infer relation k<=14 or initialize `PARAMS_VERIFIER` during ordinary checks.

## Files and ownership

| File | Responsibility |
|---|---|
| `crates/mori-ledger/src/profile.rs` | Preserve legacy DTOs; add distinct strict installed descriptor/static metadata and versioned objective work limits |
| `crates/mori-ledger/src/layout.rs` | Private validated static bindings, request-free frame admission, private operation policy, request extension |
| `crates/mori-ledger/src/relation/{mod,compiler}.rs` | Existing compiler consumes validated static view; wrappers preserve source validation and legacy profile |
| `crates/mori-ledger/src/parameters.rs` (new) | Narrow ledger-owned copy of pinned standard table/immutable exact-degree SRS recipe with explicit limits |
| `crates/mori-ledger/src/local_deployment.rs` (new) | Private all-action capabilities, complete source derivation, keyless seed and operations-only local snapshot construction |
| `crates/mori-ledger/src/call_artifacts.rs` (new) | Strict keyset/proven DTOs, bounded canonical codec, immutable read snapshots, preflight, exact opaque comparison and export leaves |
| `crates/mori-ledger/src/{prepare,proving,lib}.rs` | Private installed owner, actual stored operation binding, delegation to existing SDK adapter, exported callable APIs and updated source closure |
| `crates/mori-ledger/Cargo.toml` | Add workspace libc for nonblocking reader; retain explicit proof-verifying |
| `crates/mori/src/commands/{keygen_call,prove_call,verify_call,call_io}.rs` (new) | Real library consumers and bounded CLI file/publication plumbing |
| `crates/mori/src/{cli.rs,commands/mod.rs}` | Expose three commands without changing legacy compile/prepare/prove/verify contracts |
| `crates/mori/Cargo.toml` | Move/add futures and rand workspace dependencies needed by actual async consumers/RNG; retain locked versions |
| `crates/mori-ledger/tests/{admission,proving,relation,mutations}.rs` | Ordinary regression, privacy and literal full-readback checks |
| `crates/mori-ledger/tests/{installed_seed,call_artifacts}.rs` (new) | Safe request-free/strict-envelope tests; no minted key/proof capability |
| `crates/mori/tests/installed_call.rs` (new) | Real command help/missing-input/static-refusal paths; native positives unrun |
| `experiments/moriarty-language/formal/quint/language/{installed_call_consumer,installed_call_consumer_test}.qnt` (QA worker) | Whole-map/cold-purpose necessary checks under external derivation/crypto premises |
| `docs/NATIVE-CALL-PROVING.md`, `examples/core/ledger-call/README.md`, `README.md` | Actual API/CLI scope, native unrun limits; correct stale Transaction::prove naming |

The capabilities may be public opaque types so external library consumers can pass them. All fields/construction helpers are private; no Deserialize, From<manifest>, mutable-map or arbitrary-hash constructor exists. Narrow crate-private accessors connect modules; public access is immutable. No test-only constructor for a derived capability is added.

## Task 0: Adopt exact design and preserve baseline evidence

**Files:** runtime only until root adoption; then inspect repository startup-required `README.md`, `ROADMAP.md`, `WIKI_SCHEMA.md`, `docs/FOOTGUNS.md` and current assignments before edits.

- [ ] Receive root's explicit adoption/resume and any adopted design fixes. Re-read changed design bytes; record the actual adopted SHA in a NEW runtime note. Do not infer adoption from elapsed time or a single vote.
- [ ] Refresh `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`. Retain exact output at a new path.
- [ ] Before edits, run baseline callable checks with `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target`, Rust1.99, `--locked --offline`, preserving each stdout/stderr/exit without replacement:

```text
cargo test --locked --offline -p mori-ledger --lib --tests
cargo test --locked --offline -p mori-ledger --doc
cargo test --locked --offline -p mori --test prepare_call
cargo test --locked --offline -p mori --bin mori commands::prepare_call::tests
cargo test --locked --offline -p mori --bin mori commands::compile_call::tests
cargo check --locked --offline -p mori-ledger -p mori --no-default-features
cargo tree --locked --offline -p mori-ledger -e features
```

Expected: current callable suite passes (root reports 44 ledger ordinary tests and six prepare CLI tests); compile-fail docs pass; publication checks pass; explicit ledger proof-verifying appears. A current failure is preserved/reproduced before repair, not hidden behind historical baseline counts. Never run `--ignored` or the native_proofs target.

- [ ] Use ordinary TDD for behavior work. Run each new safe targeted regression red, implement, then green; retain red/green outputs. Checkpoint commits only when root permits them, never push/merge from this author.

## Task 1: Private static bindings and request-free complete keyless frame

**Files:** profile.rs, layout.rs, relation/mod.rs, relation/compiler.rs; tests/installed_seed.rs and admission.rs.

**Interfaces:**

```rust
// Crate-private; constructor validates supplied source and exact admitted profile.
pub(crate) struct ValidatedStaticBindings {
    source: String,
    program: mori_core::Program,
    abi: crate::layout::LedgerAbi,
    bindings: BindingData,
}
pub(crate) struct BindingData {
    payer: String,
    roles: Vec<crate::profile::RoleBinding>,
    assets: Vec<crate::profile::AssetBinding>,
    duties: Vec<crate::profile::DutyBinding>,
}
pub(crate) enum OperationPolicy<'a> {
    Keyless,
    Installed(&'a crate::local_deployment::PublicLocalDeployment),
}
// AdmittedFrame owns all request-independent predecessor fields and Scenario
// with empty inputs. AdmittedSeed owns a keyless admitted frame plus descriptor.
pub(crate) struct AdmittedSeed { frame: AdmittedFrame, descriptor: crate::profile::Deployment }
pub(crate) fn compile_static_call(binding: &ValidatedStaticBindings, action: &str)
    -> crate::Result<crate::relation::Artifact>;
pub fn admit_local_seed(source: &str, program: &mori_core::Program,
    descriptor: &crate::profile::Deployment, state: &[u8], parameters: &[u8])
    -> crate::Result<AdmittedSeed>;
```

AdmittedSeed is opaque public if returned by the public API; its declaration remains field-private. OperationPolicy and BindingData stay crate-private and are never selected through a public bool/map. Distinct installed DTOs are validated explicitly into the same static view; never rewrite an installed descriptor's profile/qualification to pass legacy APIs.

- [ ] Add failing seed test with a ToggleCounter source, valid complete keyless frame, count=u128::MAX/active=false, no request. Admission succeeds despite impossible selected step; request preparation fails. This proves keygen is independent of transition success.
- [ ] Add failing seed cases for wrong source/program/ABI/parameters/complete state hash, malformed typed state/duty cell, unknown state/duty, NIGHT invariant, role alias, extra/missing operations and any existing VK/IR. Use existing support public keyless fixtures and existing safe IR-only mutation; do not construct native VKs.
- [ ] Extract existing source static checks from layout::admit_source, leaving exact legacy profile checks in the legacy wrapper. Replace compiler's `&Deployment` field with `&BindingData` without changing expression/statement arithmetic or templates. relation::compile retains original source parsing/equality and builds the same IR via compile_static_call.
- [ ] Split decode_predecessor into complete request-independent frame admission and request extension. Move state/duty/UTXO/balance/gas/frame/supply checks intact; extend with selected spends/block context/request inputs only for preparation. No fake request/simulation in seed admission.
- [ ] Keep native state/parameter hashes and ABI commitments independently checked. Bound action name count and duplicate membership before map construction; Ordinal leaves later remove action names from filesystem paths.
- [ ] Run targeted new tests, legacy admission/mutations/relation tests. Preserve identical IR for identical static source; compiler manifest/source closure identities legitimately change with compiler bytes and require requalification.

## Task 2: Strict installed DTOs, objective limits and bounded codecs

**Files:** profile.rs, call_artifacts.rs (new), lib.rs; tests/call_artifacts.rs.

**Interfaces:**

```rust
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeWorkLimits {
    pub profile: String,
    pub max_degree: u8,
    pub max_actions: u64,
    pub max_srs_bytes: u64,
    pub max_prover_key_bytes: u64,
    pub max_verifier_key_bytes: u64,
    pub max_input_bytes: u64,
    pub max_leaf_bytes: u64,
    pub max_artifact_bytes: u64,
    pub max_operations: u64,
    pub max_public_inputs: u64,
}
pub(crate) fn ordinal_leaf(index: usize, suffix: &str) -> String {
    format!("action-{index:06}.{suffix}")
}
pub(crate) fn checked_total(total: usize, next: usize, cap: usize)
    -> crate::Result<usize> {
    total.checked_add(next).filter(|n| *n <= cap)
        .ok_or(crate::Error::input("installed-artifact-size"))
}
```

Use version `moriarty-native-call-work-limits/1`. Validate every numerical bound nonzero, each u64->usize conversion checked, cap+1 checked, and max_actions/max_ops/PI counts checked before corresponding ordinary work. No default limits. New installed descriptor carries the existing seed binding fields plus derived source/static/build/map/native identities, in its distinct strict schema. No public bindings file or independently selectable duplicate static data is introduced.

- [ ] Write red tests for missing/unknown/duplicate JSON fields, wrong profile/qualification/anchor/phase, zero/overflow limits, unknown action, manifest action duplicates/omissions/extras, invalid ordinal paths and aggregate sum overflow.
- [ ] Use Core's duplicate-rejecting JSON parser then serde deny_unknown_fields for every new DTO; do not parse through serde_json alone. Bound source/request/metadata with existing ordinary caps and installed binary/key/proof leaves with explicit new limits.
- [ ] Add bounded writer/read-snapshot code that rejects before appending/reading past cap. New canonical encode/decode uses SDK tagged_serialize/tagged_deserialize, exact EOF and bounded roundtrip equality. Do not call old decode_native on new large installed state or globally raise its cap.
- [ ] Keep imported VK/PK leaves as Vec<u8> in untrusted snapshot DTOs. No key decode/init or trusted capability constructor in artifact parsing. Metadata preflight compares independently compiled source/static/IR/manifest/build/features/standard-parameter identity before native calls.
- [ ] Add exact EOF/tag/truncated/noncanonical safe codec tests using ordinary Vec/state values and wrong tags. Tests never create a fake ProofMarker success object.

## Task 3: Ledger-owned pinned exact-degree SRS reader

**Files:** parameters.rs (new), mori-ledger/Cargo.toml, lib.rs; internal ordinary tests.

**Interfaces:**

```rust
pub(crate) struct PinnedCallParameters {
    degree: u8,
    params: midnight_transient_crypto::proofs::ParamsProver,
    identity: ParameterIdentity,
}
// ParameterIdentity is a strict DTO: SDK pin, standard profile, ordinal degree,
// fixed bls_midnight_2p{degree} name, pinned SHA256 and actual bounded byte count.
```

- [ ] Copy the exact 20-entry SDK04c9c5d pinned table and immutable-reader recipe from existing backend parameters.rs into the narrow ledger module. Do not depend on mori-backend. Adopted implementation retains the existing 128MiB hard inner cap in addition to an explicit caller max_snapshot_bytes; no new default is invented.
- [ ] Reader order: supported degree/table and limits -> explicit path -> O_NONBLOCK open -> opened FD is regular -> bounded immutable snapshot -> expected SHA -> u32 degree header -> ParamsProver::read -> EOF. Store decoded immutable params and identity; exact-degree ParamsProverProvider refuses any other degree. Equal-degree actions share an immutable Arc snapshot.
- [ ] Add source-only safe tests for unsupported degree/missing path/nonregular FIFO or directory, declared-size/over-limit temp file rejection, and bound arithmetic. Refusals must occur before SHA/decode; never open/read old SRS files or initialize params in tests.
- [ ] Typecheck reader/provider. Record SHA/header/EOF/pinned-positive behavior as native unrun, rather than circumventing hashes with a test constructor.

## Task 4: Actual complete native derivation and cold comparison

**Files:** local_deployment.rs, proving.rs, call_artifacts.rs; privacy doctests and opaque comparison unit tests.

**Interfaces:**

```rust
#[derive(Clone)]
pub struct PublicLocalDeployment { snapshot: std::sync::Arc<PublicDeploymentSnapshot> }
#[derive(Clone)]
pub struct DerivedLocalDeployment {
    public: PublicLocalDeployment,
    selected: crate::proving::DerivedCallMaterial,
}
pub async fn derive_local_deployment(seed: &AdmittedSeed, selected_action: &str,
    params_dir: &std::path::Path, limits: &NativeWorkLimits)
    -> crate::Result<DerivedLocalDeployment>;
pub async fn rederive_local_public_deployment(source: &str, program: &mori_core::Program,
    descriptor: &InstalledDeployment, imported: &ImportedPublicKeyset,
    params_dir: &std::path::Path, limits: &NativeWorkLimits)
    -> crate::Result<PublicLocalDeployment>;
pub async fn reopen_local_proving_deployment(source: &str, program: &mori_core::Program,
    descriptor: &InstalledDeployment, imported: &ImportedProvingKeyset,
    selected_action: &str, params_dir: &std::path::Path, limits: &NativeWorkLimits)
    -> crate::Result<DerivedLocalDeployment>;
```

InstalledDeployment/ImportedPublicKeyset/ImportedProvingKeyset are strict untrusted artifact DTO/snapshot types from Tasks1/2. PublicDeploymentSnapshot owns all BTreeMap<String, PublicDerivedCallKey>, static/build identities and per-action degree/SRS identity; no prover material/params. Native-only private constructors convert the freshly compiled Artifact and generated keys into existing source capabilities.

- [ ] Add compile-fail docs for field construction, Deserialize, arbitrary imported map conversion and installed-owner construction. Typecheck actual future API consumers without invoking native functions.
- [ ] Compile ALL source action artifacts and check action/IR/manifest aggregate work first. For each artifact determine degree using actual ir.k, refuse discovered >max_degree/missing table, load exact pinned params once per degree, then native ir.keygen_vk for every public action. Generate selected full ir.keygen pair; its VK must equal separately derived selected public VK. Encode keys under caller writer caps. This explicit duplicate native work is counted in later admission.
- [ ] Refactor only the existing private derived-key construction to reuse a freshly source-admitted Artifact and immutable exact-degree params; do not expose an imported Artifact/key constructor to callers.
- [ ] Cold public path compares ordinary metadata and source artifacts, regenerates all VKs, then exact opaque VK equality. Cold prover additionally regenerates selected PK/VK and compares exact opaque PK/VK before using regenerated material. Reproducibility mismatch refuses; hashes alone do not admit keys.
- [ ] Opaque byte comparison helpers are unit-tested for swapped/truncated/missing/action-mapped VK and PK, including unused action changes. Label them comparison-only; tests must not turn these bytes into derived capabilities.
- [ ] Bind build/source identities to changed profile/layout/parameters/local_deployment/artifact/prepare/proving/CLI/Cargo/lock bytes and actual pinned feature policy. Fingerprints remain declared closure, not independent build soundness evidence.

## Task 5: Operations-only snapshot and exact installed preparation owner

**Files:** local_deployment.rs, layout.rs, prepare.rs, proving.rs; seed/mutation/ordinary frame tests.

**Interfaces:**

```rust
pub struct LocalInstalledSnapshot {
    descriptor: InstalledDeployment,
    predecessor: Vec<u8>,
    parameters: Vec<u8>,
}
pub struct PreparedInstalledCall {
    prepared: crate::prepare::PreparedNativeCall,
    operation: midnight_onchain_state::state::ContractOperation,
    public: PublicLocalDeployment,
}
pub fn build_local_installed_snapshot(seed: AdmittedSeed,
    derived: &DerivedLocalDeployment, limits: &NativeWorkLimits)
    -> crate::Result<LocalInstalledSnapshot>;
pub fn prepare_installed_native_call(source: &str, program: &mori_core::Program,
    descriptor: &InstalledDeployment, predecessor: &[u8], parameters: &[u8],
    request: &crate::profile::CallRequest, public: &PublicLocalDeployment,
    limits: &NativeWorkLimits) -> crate::Result<PreparedInstalledCall>;
pub async fn prove_installed_call(prepared: PreparedInstalledCall,
    derived: DerivedLocalDeployment, rng: rand::rngs::StdRng)
    -> crate::Result<crate::proving::ProvenNativeCall>;
pub fn verify_installed_call(prepared: &PreparedInstalledCall,
    candidate: &crate::proving::ProvenIntent)
    -> crate::Result<crate::proving::CallVerification>;
```

- [ ] Implement operation-map construction solely from capability-owned all-action current VK + exact tagged IR. Actual map names exactly equal source names; every actual operation equals ContractOperation::new(Some(derived VK),Some(IrBuf(exact IR))), with no v2/key omission/IR omission/mixed key.
- [ ] Snapshot replacement clones admitted ledger and its target ContractState, replaces only operations, then inserts that contract back. Never call ChargedState::new on existing data; preserve charged keys/bookkeeping wholesale. Recompute full predecessor/state/operations/descriptor hashes from resulting bytes, recheck NIGHT invariant and full preservation. Keygen seed is revalidated keyless, so existing imported keys cannot be laundered by replacement.
- [ ] Reuse exact frame/state/duty/role/UTXO/gas/context/request/economic checks with private Installed policy. Admit whole map before selecting action, then retain the actual stored selected operation in PreparedInstalledCall. Neither proof bundle nor public function argument chooses operation.
- [ ] Extract existing prepare_call_inner body behind admitted static/frame input so installed preparation reuses current simulation/lowering/VM/partition/offer/raw-owner logic. Preserve processed inspection-only expectation and complete readback.
- [ ] Delegate proving to prove_prepared_call with stored operation and selected material. Delegate verification to verify_proven_call with prepared expected owner, capability-selected action key and stored operation. Require whole final envelope, current V3, exact state/effects/events/gas/PI and final-intent-derived outputs; no mock/preimage/erased marker route.
- [ ] Ordinary tests cover complete keyless seed/frame and pure operations-only replacement helper with empty/keyless maps for bookkeeping preservation only. No positive installed capability is minted. Real derived-map replacement/admission/proof success is native unrun; compile-check actual flow.

## Task 6: Immutable bounded keyset/proven import/export

**Files:** call_artifacts.rs, proving.rs, CLI call_io.rs; artifact/publication tests.

**Interfaces:** `snapshot_public_keyset(directory, limits) -> ImportedPublicKeyset`; `snapshot_proving_keyset(directory, selected_action, limits) -> ImportedProvingKeyset`; `snapshot_proven_call(directory, limits) -> ImportedProvenCall`; `export_local_deployment(snapshot, derived, limits) -> ArtifactLeaves`; `export_proven_call(prepared_metadata, proven, limits) -> ArtifactLeaves`. ArtifactLeaves owns fixed/ordinal leaf BTreeMap and final marker; ImportedProvenCall owns candidate typed intent only after canonical bounded decode, plus strict untrusted readback metadata.

- [ ] Specify fixed leaves `deployment.json`, `predecessor.bin`, `parameters.bin`, `source.mori`, `public-keyset.json`; per-action ordinal `.manifest.json`, `.zkir.bin`, `.vk.bin`; selected PK separate `selected.pk.bin` with `proving-keyset.json`. Public snapshot reader never stats/opens selected.pk.bin or proving-keyset.json; public manifest contains no PK-required leaf. PK-free bundle remains valid.
- [ ] Each marker binds original source/program/static ABI/template/compiler/build/features/toolchain, action name/ordinal/IR/VK/degree/SRS identity and complete source action map; local snapshot binds complete native ledger/parameters/frame commitments. Proving-keyset marker binds selected action + PK independently of public marker.
- [ ] Read each leaf once through opened regular descriptor under individual and aggregate checked limits. Reject marker duplicate names/path traversal/unsupported leaf names/missing public leaves/count conflict before reading. Key resolver/provider uses only immutable generated snapshots, never disk rereads.
- [ ] Proving artifacts contain final-intent.bin, final SDK PI, processed transcript, full readback including typed state/duties/all effects/events/gas/offer and reconstructed UTXO IDs; fixed final `proven-call.json` marker last. Retain every preparation leaf useful for complete inspection with distinct names. Require independently supplied source/request/predecessor/key policy, not values selected by proof marker.
- [ ] Publish directory create-new and each file create_new with fixed safe leaves. Preserve existing collision/regular/symlink/dangling-symlink/directory data; fail without own success marker. Validate all export byte totals before writing; partial failure never returns success.
- [ ] Ordinary tests exercise the same production publication and parsing primitives with harmless opaque bytes, collision at first/middle/final marker, trailing/truncation/digest/schema/path/aggregate failures and absent PK public snapshot. No fake native-success marker or private proving-result constructor is used.

## Task 7: Actual public CLI consumers

**Files:** keygen_call.rs/prove_call.rs/verify_call.rs/call_io.rs, cli.rs, commands/mod.rs, mori/Cargo.toml, tests/installed_call.rs.

- [ ] Add clap required inputs exactly from adopted design:

```text
mori keygen-call SOURCE --seed-deployment D --seed-state S --parameters L --action A --params-dir P --work-limits W --out K
mori prove-call SOURCE --deployment D2 --predecessor S2 --parameters L --request Q --action A --keyset K --params-dir P --work-limits W --out PROVEN
mori verify-call SOURCE --deployment D2 --predecessor S2 --parameters L --request Q --action A --public-keyset K --proof PROVEN --params-dir P --work-limits W
```

- [ ] keygen run chain is source parse -> strict work limits/seed descriptor -> bounded seed/parameters -> admit_local_seed -> block_on derive_local_deployment -> build_local_installed_snapshot -> bounded export -> create-new publication/marker -> local derivation status. No request argument/invented action input/simulation is accepted.
- [ ] prove run chain is independently supplied source/descriptors/request/limits -> immutable proving-keyset preflight -> block_on reopen_local_proving_deployment -> whole-map installed preparation -> caller fresh StdRng from entropy -> block_on prove_installed_call -> actual Real readback -> bounded proven export/publication -> honest local unsigned proof status.
- [ ] verify run chain is independently supplied source/descriptors/request/limits -> immutable public-only keyset preflight -> block_on rederive_local_public_deployment -> expected installed preparation -> bounded canonical typed final intent import -> verify_installed_call -> print complete scoped verification result. No PK files/proving-cache access, alternate backend or provider fallback.
- [ ] Command refusal tests use missing arguments, strict invalid limits, source/action/IR/manifest/build/parameter-profile mismatch before any degree call. If a test reaches native derivation unexpectedly, that is a test defect: stop and change it, never run the operation as a convenience.
- [ ] Compile all production commands, inspect their full call chains and test help/missing/static-refusal/publication behavior. Successful command native positives remain unrun until admission; no unconditional throw or checker-only success substitute is accepted as implementation.

## Task 8: Independent literal/full-effects source checks, formal QA and documentation

**Files:** literal fixtures under mori-ledger/tests/fixtures, relation/proving/mutation tests, existing Quint source_call_proving/test models and README, native guide and example README.

- [ ] Copy exact independent two-action source bytes from oracle/manifest into source test fixtures (MeteredRemittance SHA `b7504aeafba01f8432b9996e5db8ee44fc91a6989871ea2c0886d70d0f182402`; ToggleCounter SHA `af7716f9a4e0a5e0ae034a20d979bc43fa0f27d960dad55d840a26ef77b8bbb9`). Keep existing single-action fixtures historical; do not silently substitute reformatted bytes.
- [ ] Through actual ordinary source-static compile/prepare API compare complete typed post-state and all ordered effects for A1/A2/B1/B2/freeze/reset. Assert request-independent action IR/manifest with same program/static bindings, distinct action relations, exactly settle+freeze/step+reset keyless source maps, extension/unrelated UTXO/duty/frame preservation, repeated-transfer aggregation and empty Toggle offers.
- [ ] A1: running4->11, duty0->7->4, alice/bob/treasury84/19/3, eight effects; A2: running31->34, duty9->12->9, balances92/11/3, eight effects. B1 count31->32/active true->false; B2 count255->256/active true->false, four effects each. Freeze leaves running/duties/balances unchanged and emits false; reset count0/active true/emits UInt0. Reject too-small amount, disabled step and overflow without tentative effects.
- [ ] Retain ordinary event branch0/1/31/32/255/256/maxUInt, Bool, absent/present/zero duty/zero claims, malformed vectors, correlated witness/static path/aggregate/gas/PI/envelope faults. These are ordinary relation/preparation/equality results, never cryptographic proof results.
- [ ] QA worker adds focused installed_call_consumer Quint QA with complete map (selected + unused action), exported claim vs regenerated capability, public-only vs selected-PK cold purpose and refusal for missing/extra/unused substitution. Keep explicit external source compiler/derivation/crypto/parameter premises; imported self-consistent hash is not SourceRecompiled. Add positive and negative witnesses and actual Rust links. Use quint-modeling/lang skill on execution; no bridge-formal claim.
- [ ] Update native guide and README with actual source APIs/commands and explicit native unrun obligations. Correct example `SDK Transaction::prove` to `SDK Intent::prove`. Preserve local unsigned/unqualified scope and complete U0–U7/M1–M17 objective.
- [ ] Run focused ordinary suite once, docs/privacy checks, CLI tests, no-default compile, explicit feature tree and Quint typecheck/test/bounded sampling with new immutable logs. Existing Quint command syntax is reused from experiments/.../language/README.md; run no native driver or ignored test.

## Source completion and handoff

- [ ] Self-review complete public CLI -> library -> SDK call chain, no stub/fallback, complete all-action policy, strict new profile and untouched old refusal/schema; update source closure for every changed production module and locked dependency/features.
- [ ] Record exact final HEAD/tree/diff/source manifests, all current failures and ordinary command exits. Freeze the full candidate and return it to root for fresh independent Sol HIGH + Grok HIGH exact-source audits. Author does not approve its own candidate or merge/push.
- [ ] Hand off native tests as UNRUN: actual source k/fit and every repeated scan, pinned SRS/hash/decode/EOF, all-action VK+selected PK generation/reproducibility, complete derived snapshot, same-key A1/A2/B1/B2 proof, freeze/reset routing, PK-free cold verify, embedded-verifier compatibility, weak-key/trivial-proof substitution, unused-action IR/VK mutation, corrupt/swapped PK, wrong SRS/degree, V3/trailing/proof corruption, unchanged-proof PI/binding/event/gas/claim crypto rejection, hostile-witness real constraints, feature-disabled/mock refusal and process containment.
- [ ] Root obtains current consumed-budget-aware per-phase allocation, process time/memory/thread/disk/output/attempt/containment bounds, exact commands and independent resource decisions before guarded native dispatch. Nothing in this plan grants that authority or claims fit, authenticity, funding, settlement, recursive history or full source soundness.

## Draft self-review

Coverage: static/profile and keyless seed Tasks1–2; pinned parameters/work bounds Tasks2–3; all-action/cold source-key correspondence Task4; operations-only snapshot/actual stored operation/raw SDK/Real verification Task5; strict artifact/canonical/read-once/publication Task6; actual CLI Task7; independent literal complete effects/Quint/docs Task8; native unrun acceptance and full roadmap retained in handoff. Type names and ownership are explicit above; code snippets are interface/helper definitions, not a claim to have implemented them. This plan contains no executable native tests or fabricated positive material. Root adopted binding conditions also require public import to ignore any combined-manifest optional PK entries entirely, lazy outer-state VK wrappers to remain data before comparison, and the SDK 50,000-byte VK inner cap to remain enforced.
