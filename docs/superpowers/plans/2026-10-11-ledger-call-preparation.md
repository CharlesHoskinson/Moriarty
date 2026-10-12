# General Ledger Call Preparation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. The user's single-writer instruction selects inline implementation; no second writer or delegated implementation. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Deliver `mori prepare-call` with source-derived actual pinned VM execution, native guaranteed transcripts and unsigned public offers, plus complete scoped readback.

**Architecture:** Add one owned `mori-ledger` crate below the existing CLI, consuming checked Core without changing existing local v3 artifact/signature/proof bytes. Typed public admission, layout, op lowering, custody and actual VM/transcript execution return a fully serialized public bundle. The CLI safely publishes it and remains explicitly local-unproven.

**Tech Stack:** Rust 1.99.0; existing Moriarty Core; native Midnight Git revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`; serde strict owned public schemas; native tagged serialization; SHA256 identity; Quint plain finite custody QA.

## Global Constraints

- Controlling [spec](../specs/2026-10-11-ledger-call-preparation-design.md); fresh independent Sol HIGH/Grok 4.6 HIGH approval before production/test/model implementation.
- Base `d6485f0c0c7495a5b9cf6811b5fb6ed2297bae5a`, tree `f44db2747986c60c74b0cd84213b3d951aa4692c`; one writer in this worktree.
- Moriarty edits only; minocrab/projectfenrir tracked source read-only at full pinned commits from the spec; no mutable absolute-path dependency.
- Preparation profile `moriarty-ledger-call-preparation/1`, ABI `moriarty-ledger-public-abi/1`, prospective relation `moriarty-ledger-call-relation/1` unimplemented.
- Qualification `local-unproven`; one public user payer, native unshielded assets, one contract call, one guaranteed financial phase; no Ckpt/key/IR operations.
- Existing input cap 1,048,576 bytes; Core MAX_TRACE_CELLS 65,536 / MAX_EXPRESSIONS 8,192 / MAX_STATEMENTS 1,024 / MAX_DECLARATIONS 256; native op cap 100,000; aggregate artifacts 16,777,216 bytes. No increased resource authority.
- No proof/keygen/MockProver/SRS body/network/account/funding/transaction job. No wallets/seeds/.env/private keys/witnesses/credentials. Unsigned native type unit signatures do not establish authorization.
- Existing v3 Target/Claim/signature/proof canonical bytes and compiler/prover/verifier behavior unchanged; original allocation consumed and sorted9a877 failure retained.
- No native apply/financial settlement claims. Actual VM state/effects/events are distinct from prospective unapplied user balances/UTXOs. Full-effects controls are mandatory.
- Model cryptography, VM correctness and public-byte binding as abstract premises, then test actual pinned VM/offer/transcript bytes independently. Quint QA uses typecheck/test/run, no verify/Apalache.

## File structure and public interfaces

Create workspace crate `crates/mori-ledger/`:

| File | Responsibility |
|---|---|
| `Cargo.toml`, `src/lib.rs` | Native dependencies; exports and phase-specific errors |
| `src/profile.rs` | Exact profile/deployment/request/public evidence schemas and source-wide support |
| `src/codec.rs` | Existing-size bounded native input, canonical tagged decoding/public bundle encoding |
| `src/layout.rs` | Source-derived typed maps, complete native predecessor→Core projection and full frame |
| `src/ops.rs` | Native read/write/claim/log op templates and source/op/field binding metadata |
| `src/custody.rs` | Actual native spends/outputs/unsigned offer, grouping/change/conservation/native balance preflight |
| `src/prepare.rs` | Actual QueryContext/query/partition/run_transcript/prototype/add_call and independent complete comparison |
| `tests/support/mod.rs` | Public typed local predecessor builders and source-independent arithmetic fixtures |
| `tests/{admission,vm,custody,prepare}.rs` | Actual caller positive/hostile boundaries |

Modify `Cargo.toml`, `Cargo.lock`, `crates/mori/Cargo.toml`, `crates/mori/src/cli.rs`, `crates/mori/src/commands/mod.rs`; create `crates/mori/src/commands/prepare_call.rs`, `crates/mori/tests/prepare_call.rs`. Add public `examples/core/ledger-call/{payment.mori,collection.mori,README.md}` and link that local preparation workflow from `README.md`. Add `experiments/moriarty-language/formal/quint/language/ledger_call_custody.qnt` and `experiments/moriarty-language/formal/quint/language/ledger_call_custody_test.qnt` beside the existing language models; old local signed models remain separately scoped.

Owned interfaces (new types all belong to mori-ledger; native DB alias is `InMemoryDB`):

```rust
pub fn admit_source(program: &mori_core::Program, deployment: &Deployment)
    -> Result<LedgerAbi, Error>;
pub fn decode_native<T: midnight_serialize::Deserializable + midnight_serialize::Tagged
    + midnight_serialize::Serializable>(bytes: &[u8]) -> Result<T, Error>;
pub fn decode_predecessor(program: &mori_core::Program, abi: &LedgerAbi,
    deployment: &Deployment, state: &LedgerState<InMemoryDB>, request: &CallRequest)
    -> Result<AdmittedPredecessor, Error>;
pub fn lower_ops(abi: &LedgerAbi, pre: &AdmittedPredecessor,
    execution: &mori_core::Execution) -> Result<LoweredOps, Error>;
pub fn prepare_offer(pre: &AdmittedPredecessor, execution: &mori_core::Execution)
    -> Result<PreparedOffer, Error>;
pub fn prepare_call(program: &mori_core::Program, deployment: &Deployment,
    predecessor_bytes: &[u8], parameter_bytes: &[u8], request: &CallRequest)
    -> Result<PreparedBundle, Error>;
```

`Deployment`, `CallRequest`, `LedgerAbi` match exact spec fields; deserialize owned JSON with deny_unknown_fields and existing duplicate-aware `mori_core::json::parse`. `AdmittedPredecessor` holds actual full LedgerState/ContractState/explicit CallContext, decoded full Core Scenario, all native UTXO identities/metadata and trusted mappings; no expected post. `LoweredOps` holds `Vec<Op<ResultModeVerify,InMemoryDB>>` and complete source/static/dynamic field metadata. `PreparedOffer` holds native `UnshieldedOffer<(),InMemoryDB>`, grouped claims, effect→optional claim/output index mapping and full prospective projection; zero effect mapping is None, same-role self-payment/change remain separate; no signatures. `PreparedBundle` holds fixed leaf byte arrays/owned public readback and final marker; no generic private-material serialization. Error layers: input/identity/source-profile/semantic/native-invariant/native-VM/native-offer/native-balance/native-partition/unsupported-phase/full-readback/publication.

### Task 1: Strict public admission and pinned offline native dependencies

**Files:** root Cargo manifests/lock; new crate Cargo.toml/lib/profile/codec; `tests/admission.rs`.

**Interfaces:** Produce admit_source/decode_native and owned schema types above. No CLI flags yet.

- [ ] Read installed develop/AGENTS and refresh guarded status. Confirm root's frozen design clearance and exact source HEAD. Do not resume a stale native campaign.
- [ ] Run existing callable light integration baseline before edits with `cargo test --locked --offline -p mori --test check_compile_io` and `cargo test --locked --offline -p mori-backend --test authorization`. Expected: prior check/compile and host authorization cases pass; native proving is not invoked. If dependencies are unavailable, retain exact refusal and stop dependent build.
- [ ] Add failing tests for duplicate/unknown/request post-state/private fields, wrong full tuple, source-wide second-payer unselected action, missing/extra operation and keyed legacy operation. Tests assert exact error layer and zero VM work.
- [ ] Add new dependencies from the spec's exact native pin/versions; keep default proof-verifying feature and no weakened feature. Update lock offline (`cargo generate-lockfile --offline`), inspect changes for unrequested pin/version updates; preserve existing entries or restore/repair unrelated lock drift. Current lock has serialize but lacks ledger/onchain/coin/storage packages: resolving them is required, not presumed complete.
- [ ] Implement canonical tagged decoder; this concrete code controls EOF and noncanonical acceptance:

```rust
pub fn decode_native<T>(bytes: &[u8]) -> Result<T, Error>
where T: midnight_serialize::Deserializable + midnight_serialize::Tagged
    + midnight_serialize::Serializable {
    if bytes.len() > 1_048_576 { return Err(Error::input("input-size")); }
    let value: T = midnight_serialize::tagged_deserialize(bytes)
        .map_err(|_| Error::input("native-decode"))?;
    let mut canonical = Vec::new();
    midnight_serialize::tagged_serialize(&value, &mut canonical)
        .map_err(|_| Error::input("native-encode"))?;
    if canonical != bytes { return Err(Error::input("noncanonical-native")); }
    Ok(value)
}
```

`Error::input(&'static str)` is defined in lib.rs as the owned input error constructor; do not include raw native bytes in errors. Implement exact typed descriptor/request schemas and source-wide all-Statement/all-Expr admission per spec. Add full source/core/ABI/op-set hashes and explicit absent proof/qualification evidence.
- [ ] Run `cargo test --offline -p mori-ledger --test admission`, then `cargo check --locked --offline -p mori-ledger`; expected all cases pass and no missing dependencies. Commit scoped schema/dependency changes only after checks. Record unavailable cached dependency as a blocker, never fetch silently.

### Task 2: Complete typed predecessor/layout and independent local inputs

**Files:** layout.rs; tests/support/mod.rs; admission.rs; example source files.

**Interfaces:** Produce decode_predecessor and AdmittedPredecessor. Consumes admit_source and strict native decoder.

- [ ] Write independent fixtures using public constructors `LedgerState::new`, `ContractState::new`, `ChargedState::new`, `StateValue::{Array,Map,Cell}`, native public Utxo structs/metadata, operation(None,None) for each action. Keep NIGHT supply unchanged by using a distinct local QA unshielded token; explicitly local trust, no funding claim. Produce canonical native bytes and separately trusted descriptor hashes outside production code. Fixed public key encodings are supplied public verifier keys, no generated private wallet.
- [ ] Author payment70 and fee-first43 source with parameter-dependent amounts and all state/duty metadata. Copy expectations from independent diagnosis, not simulate output. Assert source parsing yields all forms, fee-first order differs, and rename agreement/action/asset alias without selecting another lowering.
- [ ] Write failing tests replacing source state defaults with actual paid/collected successor values; present-zero/reserve/inherited addon membership; frozen/extension/unknown UTXO metadata preservation; asset/key/scale substitution; full predecessor hash mismatch and invalid native NIGHT supply.
- [ ] Implement the exact Array states/duties/extensions layout and domain-separated length-framed hash keys from the spec. Decode all states/duties/roles/assets, checked full UTXO sums and source input types. Retain full native frame. Check source-wide mappings and exact operation/balance/maintenance identity.
- [ ] Call actual `predecessor.check_night_balance_invariant()` before native VM/offer work; no correcting or funding it. Derive actual caller/contract balance and set every CallContext field; all explicit trusted block data and empty com_indices. No Scenario post-state input path.
- [ ] Run `cargo test --locked --offline -p mori-ledger --test admission`; expected mutation refusal and positive full-state decoding. Commit typed layout and independent public fixtures.

### Task 3: Plain finite custody model QA

**Files:** `experiments/moriarty-language/formal/quint/language/ledger_call_custody.qnt` and `ledger_call_custody_test.qnt` in that directory.

**Interfaces:** Semantic effects→grouped claims→exact payout+change multiset; no Rust API or native cryptographic certificate.

- [ ] Apply installed quint-llm-kit:quint-modeling from-requirements and quint-lang references after design clearance. Read existing Quint placement and local CLI version without network/install. Use plain finite shared-state model, no framework/delegation.
- [ ] Model independent opening balances/duties/state/ordered semantic effects, selected distinct input IDs, grouped per-asset/recipient claims, sorted indexed output list, change and preservation. State transitions: propose, native-shape-check, complete-compare, reject; acceptance flag means preparation only, no consumed ledger state. Use concrete finite role/token/amount sets; no uninitialized constants. Explicit premises: typed-byte hashing, VM correctness, proof/signatures and external funding are abstract/unverified.
- [ ] Define non-vacuous invariants for per-asset conservation, exact grouped claim amount output, no extra/omitted output, input uniqueness, untouched membership and present-zero duty persistence. Native claim subset and Moriarty full-output equality are separate predicates. Same-role payer-as-recipient is supported; model the required payment and change as separate indexed output occurrences, including equal50/50 multiplicity. Distinct-role alias collision remains unsupported. Semantic zero effects retain order with no claim/output and optional mapping None; zero-only and empty-offer cases preserve other transitions.
- [ ] Concrete positive run witnesses assert payment70 and fee-first43 full results, payer12/Bob20/change68, payer50/change50 as two equal occurrences, zero-only change100, and empty offer with no selected inputs/no positive payouts. Hostile witnesses assert recipient substitution, claim69/output70, missing fee, unclaimed extra output, split aggregate, merged claim/change, duplicate input and value/sum conservation violations. Preserve duplicate output multiplicity in list rather than Set abstraction.
- [ ] Run `quint typecheck experiments/moriarty-language/formal/quint/language/ledger_call_custody.qnt` then `quint run experiments/moriarty-language/formal/quint/language/ledger_call_custody.qnt --main=ledger_call_custody --invariant=preparation_invariants --max-samples=100 --max-steps=10`; run concrete test module using local CLI's documented `quint test` syntax. Expected positive reachability and hostile rejection assertions pass. A deliberately weakened complete comparison must expose extra-output witness. No verify/Apalache. Retain model assumptions and actual outputs as scoped QA. Commit model only after all checks.

### Task 4: Real native op lowering and full query checks

**Files:** ops.rs; tests/vm.rs.

**Interfaces:** Produce lower_ops and LoweredOps. Consume full pre, ABI, actual Core Execution. Dynamic values are host preparation, never constrained proof evidence.

- [ ] Write actual-query tests for state/duty reads/writes, present-zero retention, member/insert claim grouping, real event payload/name/order and strong final stack. Begin with one actual state read/write and one recipient claim so stack/FAB mistakes fail in the real VM.
- [ ] Implement read templates native Dup/Idx/Popeq from actual typed opening values; path writes with native Idx(push_path), Push(storage=true), Ins restore strong state. Implement grouped slot8 accumulator using actual Op and native ClaimedUnshieldedSpendsKey FAB conversion only for positive effects; preserve zero ordered effects and None native mapping without zero claim ops. All eight other Effects maps/sets must remain empty.
- [ ] Implement each semantic effect in order, including Next, Liability, Discharge and Emit; checked UInt/Bool source expressions use Core evaluation. Emit native MIP wrapper with typed public payload; no pretend user balance cells, inc contract input/output or mint.
- [ ] Derive every op scalar field vector from native `FieldRepr`; metadata records source statement/expression, static headers and dynamic field origin. Unit controls compare templates with native real types/conversions. Avoid handwritten opcode/asset/address tags.
- [ ] Execute native `QueryContext::query::<ResultModeVerify>` with explicit full supplied context/cost model and trusted bounded gas; assert full native states/duties/extensions/claims/events. Concrete arithmetic checks:

```rust
assert_eq!(observed.state_uint("paid"), 70);
assert_eq!(observed.duty_amount("payment"), Some(30));
assert_eq!(observed.duty_amount("reserve"), Some(5));
assert_eq!(observed.duty_amount("zero"), Some(0));
assert_eq!(observed.recipient_claim("bob", "QA"), 70);
assert_eq!(observed.recipient_claim("treasury", "QA"), 2);
assert_eq!(observed.event_values("paid"), vec![70]);
```

`observed` is a test-only independently decoded readback facade in support/mod.rs; each method decodes named native map/claim/event through trusted fixture metadata, never calls production simulation or expected post.
- [ ] Mutate missing/extra/reordered op, Popeq read, wrong UInt/Bool alignment, wrong recipient field, event payload and untouched state. Expected actual native query refusal or independent whole-readback mismatch, honestly classified. Run `cargo test --locked --offline -p mori-ledger --test vm`; commit actual op lowering.

### Task 5: Native unsigned offer construction and complete custody

**Files:** custody.rs; tests/custody.rs.

**Interfaces:** Produce prepare_offer; native balance preflight is used after real call/intent assembly in Task6. Consume actual predecessor selected spends, Core effects, trusted mappings.

- [ ] Write native typed input/output tests for both independent financial positives. Check all selected input fields `(value,key,type,intent_hash,output_no)` against full predecessor, same trusted payer, checked sums, all preserved UTXOs/metadata.
- [ ] Implement checked aggregation over `(TokenType,PublicAddress)` for recipients, one aggregate payout output per pair, separate positive payer change per asset, zero-change omission; allow same-role payer recipient while rejecting distinct-role alias collision. Preserve zero semantic effects with no native claims/outputs and None mapping. Allow selected admitted asset F100/no positive payout as change100, and F0/no positive payout as empty offer; reject unknown/unmapped assets. Sort native `UtxoSpend` and `UtxoOutput` with native Ord; grouped payment native-key order then appended change and stable sort retain deterministic separate provenance for equal outputs. Preserve list multiplicity; never dedup output arrays.
- [ ] Construct real `UnshieldedOffer<(),InMemoryDB>`, empty signatures. Native immediate well_formed shape checks belong after parent construction; unit SignatureKind never establishes authority. Compare exact complete outputs/claims and input=output per asset independently of native subset acceptance. Keep effect index→optional group claim→optional sorted payment-output index mapping; zero effects have None, change has separate provenance/index.
- [ ] Concrete expected projections are literals:

```rust
assert_eq!(payment.prospective_balances(),
    [("alice", 28), ("bob", 80), ("treasury", 2)]);
assert_eq!(collection.prospective_balances(),
    [("alice", 57), ("carol", 47), ("spectator", 9), ("treasury", 4)]);
```

Fixture methods explicitly label these prospective/unapplied; actual VM contract balances remain unchanged.
- [ ] Hostile cases: split recipient aggregate vs exact native amount; zero semantic effect creating native zero claim/output; zero-only F100 emits change100, F0 empty offer; Fee+Transfer same recipient; payer12 payment +68 change retained separately (merged80 rejected); payer50 +change50 keeps two equal outputs; equal-value distinct recipient/change outputs; extra unclaimed output; omitted change; duplicate input; wrong spend owner/value/color/prior intent/output index; wrong native sort; nonzero invariant; checked host overflow. Verify input/out arrays, complete preservation, not sums only.
- [ ] Run `cargo test --locked --offline -p mori-ledger --test custody`; expected controls reject and both projections agree with independent literals. Commit custody construction.

### Task 6: Actual native transcript/prototype/preimage integration

**Files:** prepare.rs; tests/prepare.rs; codec/public bundle additions.

**Interfaces:** Produce prepare_call and PreparedBundle. Consume Tasks1/2/4/5 interfaces; no compile/prove/verify API change.

- [ ] Build preparation through full strict inputs. Run actual Core transition, check optional unsigned configured economic limits via existing `mori_core::intent::check_effects`; successful configured checks mean bounds only, no authorization. Lower native ops and compare actual query readback completely.
- [ ] Construct native input AlignedValue in action declaration order, output(), explicit public communication randomness. Use actual runtime communication_commitment. Supply actual verify Ops and explicit context to `partition_transcripts(&[PreTranscript { .. }],parameters)`; accept Some guaranteed/None fallible only. No Ckpt.
- [ ] Run actual guaranteed `run_transcript` from initial context; compare direct query's complete data/effects/events and costs. Test altered transcript gas/effects/ops and parameter-bound changed native partition behavior. Native refusal cannot be relabeled proof evidence.
- [ ] Assemble actual ContractCallPrototype with local operation(None,None), native transcript/FAB framing, empty private outputs, entry-point KeyLocation. Assemble native unsigned unproven Intent with explicit public binding randomness, TTL, no DUST/deploy/update and real offer; use Intent::add_call::<ProofPreimage>.
- [ ] Read actual native preimage public op field vectors and Popeq-only outputs; require empty private transcript. Verify complete public field vectors agree with actual Op::field_repr, no scalar local relation imported. Construct actual `Transaction::from_intents`, call `.balance(None)`, require per-token/phase0 balance. Shape checks call offer.well_formed(1,erased_parent) immediate result only; returned signature closure not executed.
- [ ] Native balance edge controls: legal Core2^127 individual value fails native conversion; multiple individually legal values may overflow intermediate i128 despite finalzero, so actual API must refuse. Do not change Core range or preflight using only final sums.
- [ ] Calculate actual `erased_parent.intent_hash(1)` and prospective sorted output indices; label identities provisional and require reconstruction after final proof/DUST/binding/TTL. Preserve supplied LedgerState exactly and publish no applied global successor. Compare all unchanged native frame hashes.
- [ ] Serialize all fixed leaf bytes and final hashes/evidence status under existing artifact cap, before filesystem publication. Tests assert every unavailable proof/signature/apply/funding/fee/settlement/qualification predicate is not-performed/unestablished. Run `cargo test --locked --offline -p mori-ledger`; expected actual VM/offer/transcript caller controls pass. Commit complete owned preparation API.

### Task 7: Actual strict safe CLI and examples

**Files:** Mori CLI/Cargo/module dispatch; prepare_call.rs; CLI integration tests; public examples/readme; root README.md link.

**Interfaces:** Clap `PrepareCall(prepare_call::Args)` exposes exact command in spec. Consumes prepare_call and PreparedBundle; uses existing source/JSON/display/publication helpers.

- [ ] Add failing public process tests with `env!("CARGO_BIN_EXE_mori")` for both independent positives, payer12/Bob20/change68, two equal payer50/change50 output occurrences, zero-only change100, empty offer, and each admission/mutation layer. Every file path passed explicitly; no source fixture selection in production.
- [ ] Add Args(file,action,deployment,predecessor,parameters,request,out), register enum/dispatch. Bounded-read complete source and native input bytes, strict request/deployment parse, action consistency; fully prepare/serialize before `create_dir(out)`.
- [ ] Publish fixed leaves through existing `proof_io::publish_new`, final preparation.json last; refuse existing root, file/symlink/dangling symlink/directory and raced leaf. Never delete unrelated additions or previous completed leaves. Escape source/action/address/display labels. Keep private fields/bytes out of errors.
- [ ] CLI tests decode emitted tagged transcript/data/offer and independently recompute all SHA256 identities and two full expected results. Require complete artifact marker only after all leaves, no financial successful artifact after arithmetic/late ensures/VM/partition failure.
- [ ] Add race/collision controls at first/middle/final leaf; sentinel contents/links survive, no malformed marker. Source at limit/over-limit/invalid UTF8, JSON duplicate/unknown, native truncated/wrong tag/trailing, oversized and private-material field controls preserve input files.
- [ ] Document actual local public fixture construction and explicit usage/results, absence of VK/proof/funding/settlement, and future reconstruction obligations. Examples show payment70 and fee-first43 general source with variable amounts; no canned source aliases.
- [ ] Run `cargo test --locked --offline -p mori --test prepare_call`; expected actual process caller/full decoded result and safe publication tests pass. Commit CLI and examples.

### Task 8: Frozen result verification and independent review handoff

**Files:** Focused source/tests/docs only; create-once public runtime outputs under `/home/charl/Moriarty/.moriarty-dev/runtime/ledger-call-20261011`.

**Interfaces:** Exact final commit/tree/content hashes, actual public test/model logs, scoped result and unresolved gates for root's fresh independent reviewers.

- [ ] Run `cargo fmt --check`, `cargo test --locked --offline -p mori-ledger`, `cargo test --locked --offline -p mori-core`, focused backend host authorization/liability/replay test targets found in the existing test catalog, and `cargo test --locked --offline -p mori --test prepare_call --test check_compile_io`. Never run native proof test jobs implicitly; inspect selected test bodies/feature wiring first. Expected checks pass; retain any unavailable dependency/refusal honestly.
- [ ] Run the bounded concrete Quint typecheck/test/run once on final model. Do not broaden sampling/resource budgets without a specific failure reason; no cryptographic validity claim from it.
- [ ] Confirm production path source→admission→full predecessor→Core→native ops→actual VM→partition→run_transcript→native prototype/preimage/offer/native balance→public bundle→actual safe CLI publication. Inspect whole final source and ensure no expected-value/name/hash lookup returns success.
- [ ] Recheck old Target/Claim/signature/golden artifact/proof bytes via existing byte-vector tests; old consumed proving jobs never rerun. Confirm native SDK revision and existing crypto versions unchanged, no weakened features or external edits.
- [ ] Freeze commit/tree/each changed source hash plus public results, independent oracle/model premise references and all nineteen controls' failure layers. Return to root for fresh full-source/result Sol HIGH and Grok HIGH exact-candidate reviews. Author self-review does not approve; missing auditors do not vote. Repair within agreed scope, then obtain current exact-byte audits if bytes change.
- [ ] Report only delivered local unsigned unproven preparation, actual VM readback and prospective offer balances. U/M/native proving/deploy/apply/funding/fees/authentic state/settlement/recursion/private/history obligations remain open. No merge or network action is performed by this author handoff; root uses existing majority/merge authority after reviews.

## Design and plan self-review

Coverage: source-wide typed lowering and old-byte preservation Tasks1/2/4; exact native dependencies/strict public trust Tasks1/2; independent finances/full custody Tasks2/3/5; actual VM/phase/transcript/preimage/gas/native balance Tasks4/6; usable strict CLI and safe public artifact Task7; final complete scoped verification and audits Task8. Every spec negative is mapped to native/host/identity/refusal controls rather than a native adversarial-proof claim. No hidden install/proving/network job or fabricated key is included.

The independent custody addendum SHA256 `f2b2dc2b14bd59e3d2d18d6967a5b5a9fbd0bf2d9873328a423a9b1ffe316469` controls grouping/self/zero/multiplicity cases with the amended spec. No implementation, test, native execution or Quint model logic is written by this documentation freeze. Root receives both docs' full hashes and reviews their exact final tree before clearing Tasks1–8. Existing autonomous decision authority removes user permission prompts; it does not substitute for the two fresh substantive design votes or actual implementation checks.
