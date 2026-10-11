# Native owner intent implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development task-by-task. Each task uses a fresh implementer, an independent scoped spec/quality review, and the final fresh Sol HIGH/Grok HIGH whole-source/result pair.

**Goal:** Make the actual local financial compiler and intent/proof CLI constrain strict JubJub owner signatures and checked financial caps in the same circuit.

**Architecture:** Keep Core UInt/Bool semantics and the reference interpreter. Stage the full source-fixed financial layout, hash its exact context, append the backend authorization ABI, and constrain MOI1/FENS and caps using those financial wires. Every consumer reconstructs the complete statement and separately supplied authority; no candidate VK, host success flag or parallel evaluator supplies trust.

**Tech Stack:** Rust1.99, existing ledger04c9 ZKIR3.0.0-rc.2, locked midnight-curves0.3.1/Poseidon primitives, existing Core canonical JSON, quint-llm-kit QA.

## Global constraints

- Normative contract: `docs/superpowers/specs/2026-10-11-native-owner-intent-design.md`, exact SHA256 `a4a012366c8d202b3766fab00b5933f97b30fea522c07cae28c465678a1fb142`. Golden laboratory data: `examples/core/native-owner-intent/golden-vector.json`, exact SHA256 `9c035c8e42e2b378e8ed2b3388767d3a8d60ac7691aafd3d45dc7066eeb285f0`. Do not substitute a roundtrip for that independently reproduced vector.
- Both fresh design votes APPROVE this exact contract: Sol vote609ad465 and Grok terminal session01a12af9-b06c-7883-b78e-d33131c6d2fc/requestcbe13545-f564-4732-8f24-62aa904d5072, returnedgrok-4.6-build, requestedHIGH. Design adoption is not implementation/native approval.
- Basis merged PR26 `3748fe8278287a3729c17c5ddc245f669e21e847`, tree85ad41f4d0a2ff2c3f9576bba9a70a95c7f42b9c. Work only in `/home/charl/Moriarty/.worktrees/native-intents-20261011`; preserve primary user checkout and prior reviewed proof worktree.
- Core remains `moriarty/1`, `moriarty-core/1`; legacy host `moriarty-owner-intent/2` BIP340/ECDSA remains separately scoped. Add Native only to the backend authorization descriptor type.
- Use `moriarty-jubjub-owner-intent/1`; explicit unsigned/signed discrimination; all artifact/ABI/relation/proof/binding surfaces revision3 together. Signed-policy absence omitted from Target serialization; explicit null refused. Reject stale backend2 rather than reinterpret it.
- Source-wide single owner/beneficiary, at most16 exact distinct AssetIds and16 duties; refuse any other transfer/fee payer/debtor even in unselected actions before proportional allocation. All original financial slots, ordered effects, untouched/present-zero duties remain bound.
- Canonical bytes, exact strict schemas,18-element context staging, twoLE128 SHA bridges, auth positions N+11+4A, and MOI1 fields13+2N+6A follow the normative spec verbatim. MOI1=0x31494f4d, Bool1/UInt2, FENS=0x46454e53; no outer list prefix.
- Native statement is existential over private R/s; host signed acceptance separately checks the exact supplied attachment. Claim excludes attachment/R/s and commits reconstructed public statement. No claim of native commitment to attached bytes, ledger ownership/time/history, or JSON/SHA computation.
- Checked gross/fee/net/full closing-duty caps constrain actual effect/frame wires; refunds do not erase gross, owner fees reduce net once, and discharge is not payment or consent.
- Frozen external input bytes and notices are runtime `jubjub-frozen-inputs-v2/manifest.json`; own only the minimal strict verifier subset with MIT/Apache notices and Sig Network attribution. External Fenrir repository remains read-only; no absolute external dependency or whole application vendor.
- Use installed Rust1.99 with `RUSTUP_TOOLCHAIN=1.99.0`, locked offline dependencies and persistent `CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target`. Never invoke an absent toolchain or download fallback. The interrupted author tooling download/partial install is immutable failure evidence, not authority.
- No native/SRS decoding/keygen/prove/verify/MockProver/ignored-test experiment in Tasks1–3. Native starts only after actual integrated/max-shape measurements and BOTH new exact resource votes. All prior allocations consumed. No retry/k-search/resource increase without a distinct approved decision.
- One delivery capability through review; no planner/framework, Lean stream, exhaustive Quint verify, Fenrir implementation or broader library displacement. Full U0–U7/M1–M17 remains active after this slice.

### Task 1: Strict envelope, authorization types and host FENS verification

**Files:** Create `crates/mori-backend/src/authorization.rs`, `crates/mori-backend/src/schnorr.rs`, `crates/mori-backend/tests/authorization.rs`; modify backend `lib.rs` only for exports and manifests only for already locked direct curve dependencies. Add minimal license/provenance notices under `crates/mori-backend/third-party/jubjub-fens/`. Preserve the frozen golden vector. No compiler/proof/CLI behavior change yet.

**Interfaces:** These names and types are the handoff to Task2. Full field sets/serialized types are exactly the normative spec; no permissive/default/alternate schema.

```rust
// authorization.rs
pub const INTENT_PROFILE: &str = "moriarty-jubjub-owner-intent/1";
pub const MAX_SIGNED_ASSETS: usize = 16;
// SignedPolicy{owner,beneficiary,program_address}; ExpectedAuthority;
// UnsignedPayload; SignedIntent{payload,signature_hex}; AuthorizationType{UInt,Native};
// AuthorizationBinding{position,role,name,ty,constant,asset};
// PreparedIntent with exactly the specified payload/hash/message/projections/review.
// All structs derive strict serde decoding and existing UInt/AssetLimit fields.
impl UnsignedPayload {
    pub fn from_json(text: &str) -> Result<Self, crate::Error>;
    pub fn validate(&self) -> Result<(), crate::Error>;
    pub fn hash(&self) -> Result<[u8; 32], crate::Error>;
}
impl ExpectedAuthority {
    pub fn from_json(text: &str) -> Result<Self, crate::Error>;
    pub fn validate(&self) -> Result<(), crate::Error>;
}
impl SignedIntent {
    pub fn from_json(text: &str) -> Result<Self, crate::Error>;
}
// Strict labels/digests/canonical UInt/native fields and full sorted exact assets.
// These helpers have no Target/compiler/native dependency.

// schnorr.rs: exact canonical affine32LE encodings and FENS scheme.
pub fn decode_key(hex: &str) -> Result<midnight_curves::JubjubSubgroup, crate::Error>;
pub struct SignatureWitness {
    pub r: midnight_curves::JubjubSubgroup,
    pub response: midnight_transient_crypto::curve::Fr,
}
pub fn decode_signature(hex: &str) -> Result<SignatureWitness, crate::Error>;
pub fn verify_attachment(key_hex: &str, signature_hex: &str,
    message: midnight_transient_crypto::curve::Fr) -> Result<(), crate::Error>;
```

- [ ] Write failing ordinary tests using the independently frozen key/signature/message and exact payload. Start with real calls such as the following; retain red output, including compile failure solely from the missing specified API when necessary.

```rust
let fixture: serde_json::Value = serde_json::from_str(include_str!(
    "../../../examples/core/native-owner-intent/golden-vector.json")).unwrap();
let payload = mori_backend::authorization::UnsignedPayload::from_json(
    fixture["payload_canonical_utf8"].as_str().unwrap()).unwrap();
assert_eq!(mori_core::intent::hex(&payload.hash().unwrap()),
    fixture["payload_hash"].as_str().unwrap());
assert!(mori_backend::authorization::UnsignedPayload::from_json(
    "{\"profile\":null}").is_err());
// Strict attached verification receives independently frozen message/key/R/s,
// then a one-bit signature mutation must reject, never just hash roundtrip.
```

- [ ] Implement the exact structs and bounded parser-before-schema decoding. Decode labels by exact Unicode scalar/control/UTF8 rules; validate digest/field widths and canonicality; reject null/default/unknown/duplicate/trailing values; canonical wrapper and payload each <=65536. Limits sorted by full AssetId, unique exact identity, matching beneficiary,16 ceiling. Retain exact predecessor-label domain framing.
- [ ] Implement minimal strict host decoding/reference verification: canonical field bytes below p, curve/subgroup/nonidentity key/R, response below r without reduction, FENS-first challenge, response equation. Only use the inspected scheme; generic pinned Schnorr is different. Frozen input hashes/notices identify copied/adapted source; no production key custody or wallet signer. Fixed QA signing utilities belong in tests/examples only.
- [ ] Add real positive and hostile host tests: golden and a second independently specified key/message, modified message/key/signature/tag/order, identity/offcurve/pure torsion/mixed PK/R, s0/1/r-1 range versus r/r+1/s+r, malformed hex/decimal/labels, omitted/extra/duplicate/reordered limits,65536 wrapper boundary. Range-valid scalar alone is not full signature acceptance. Distinguish host/reference checks from native constraints.
- [ ] Run `RUSTUP_TOOLCHAIN=1.99.0 CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target cargo test -p mori-backend --locked --offline --lib --test authorization`. Run affected fmt/diff checks, retain failures and green logs. Commit with a scoped report and handoff API names. Task1 is a component gate; delivery remains incomplete until Task2–4.

### Task 2: Integrated compiler, full statement and production intent/proof callers

**Files:** Modify `crates/mori-backend/src/lib.rs`, `proving.rs`, affected compiler/proof tests and Task1 authorization module; create `crates/mori-backend/src/signed_relation.rs` and focused integrated tests as useful. Modify production `crates/mori/src/commands/{intent,verify_intent,proof_io,prove,verify}.rs`, CLI tests under `crates/mori/tests/`. Add exact independent payment source/scenario/caps examples under `examples/core/native-owner-intent/`.

**Interfaces:** Use Task1 types unchanged. Add Target.signed_policy:Option<SignedPolicy> with omission when absent and null refusal. Separate compiler authorization descriptors keep financial PublicBinding.ty unchanged. Preserve old unsigned APIs while explicitly refusing a signed Target through them; production signed APIs are:

```rust
pub fn prepare_intent(source: &str, action: &str, scenario: &mori_core::Scenario,
    target: &crate::Target, authority: &ExpectedAuthority, at: mori_core::UInt)
    -> Result<PreparedIntent, crate::Error>;
pub fn verify_intent(source: &str, action: &str, scenario: &mori_core::Scenario,
    target: &crate::Target, authority: &ExpectedAuthority, at: mori_core::UInt,
    signed: &SignedIntent) -> Result<mori_core::Execution, crate::Error>;
// proving.rs
pub fn prove_signed(source: &str, action: &str, scenario: &mori_core::Scenario,
    target: &crate::Target, authority: &ExpectedAuthority, at: mori_core::UInt,
    signed: &SignedIntent, options: &ProofOptions)
    -> Result<(ProofBundle, ProofReceipt), crate::Error>;
pub fn verify_signed(source: &str, action: &str, scenario: &mori_core::Scenario,
    target: &crate::Target, authority: &ExpectedAuthority, at: mori_core::UInt,
    signed: &SignedIntent, bundle: &ProofBundle, options: &ProofOptions)
    -> Result<ProofReceipt, crate::Error>;
```

`prepare_intent` and `verify_intent` use ordinary compiled relation/reference checks without SRS. Proving prepares the exact typed private inputs; verification reconstructs public values/circuit/VK itself. Internal shared preparation avoids duplicate simulation/framing logic. Only prove needs private witness construction; native verifier must not trust candidate layout or an externally supplied VK.

- [ ] Write failing real compiler tests with a signed Target and unchanged golden source. Assert staged18-element context identity4d84b3a3…, all28financial roles/values preserved,15authorization positions and five flattened typed private inputs, exact75-field message, direct actual financial operands and unconditional public Impact. Add a second payer/debtor in an unexecuted action and assert compile refusal before allocation. Add17asset/duty refusal and exact full registry tests.
- [ ] Stage financial layouts/static frames with placeholder context cells, derive the exact projection excluding constant values, compute context, then set positions0/1 constants. Use the same v3 staging for unsigned mode. Add strict revision3 discrimination/decoding to artifact/ABI/relation/bundle/binding and key locations. Existing unsigned source behavior remains callable under its explicit mode; old backend2 must refuse.
- [ ] Append the exact N+11+4A authorization inputs with backend Native versus UInt assignment. Keep original financial operand handles to form MOI1 directly; source-fixed tags/counts/full asset digest constants. Compute/publish message, equality-bind typed private PK coordinates to public PK, use typed subgroup PK/R assignment and canonical response roundtrip, emit unconditional strict FENS Assert. No supplied message/soft flag/communication commitment.
- [ ] Constrain checked financial sums on actual ordered effect wires and every closing owner duty. Use existing checked UInt addition/subtraction and exact inequalities; accumulate complete nonnegative sums before net subtraction. Source-wide applicability is separate admission. Pure state actions retain authorization and zero-compatible caps. Never derive limits or current time from candidate authority.
- [ ] Implement preparation/reconstruction with exact payload/Target/authority/domain/scenario/raw-source/context hashes, projections and caps. Verify strict attachment under reconstructed message before SRS. Populate exact v3 Claim and public values; exclude R/s and attachment. Preserve native existential scope and complete Execution receipt. Check output/profile/length/admission before parameter access.
- [ ] Wire existing CLI commands to these APIs: signed intent preparation receives explicit named profile, trusted Target/authority/at; signed verify-intent/prove/verify additionally receive attachment. Legacy scheme/framing/public-key arguments remain required together on the host profile; signed mode refuses legacy or partial groups. `prove` and `verify` require all signed arguments with a signed Target and no unsigned fallback. Review output clearly names signed message/domains/caps/local qualification, excludes developer approval controls and wallet custody.
- [ ] Add ordinary production tests with an EMPTY/NONEXISTENT params directory: honest signed inputs pass host/relation preparation to the specific missing-parameters diagnostic; malformed/downgrade/nonowner/invalid signature/cap/time/authority inputs refuse earlier. No parameter file, keygen or genuine verification. Private signature controls use ordinary checker only and are labeled as such.
- [ ] Use independently authored runtime `signed-reference-expectations/payment.mori`/scenario/expected.json as the contrasting source acceptance case: alice28/bob80/treasury2, paid70, duties30/5/present0, ordered Transfer70/Fee2/Liability50→30/State0→70/Emit70. Gross72/fee2/net68/duty35 exact caps accept; fresh signed caps71/1/69/34 each refuse. No author-generated success snapshot replaces those expectations.
- [ ] Run focused backend/core/CLI ordinary suites with locked offline Rust1.99, preserving red/green failures. Check actual CLI compile/intent/verify-intent and no publication on refusal, Unicode/FIFO/exclusive-file behavior and stale versions. Commit source/tests/examples plus scoped report. Do not run ignored/native experiments or claim native support demonstrated yet.

### Task 3: Independent QA, Quint and exact integrated shape measurements

**Files:** Create `experiments/moriarty-language/formal/quint/language/signed_transition.qnt`, `_test.qnt`; update QA README and `docs/NATIVE-CORE.md`; add only necessary ordinary hostile/shape tests in existing backend/CLI test files. No production protocol change or dependency expansion.

- [ ] Build a finite shared-state model from the adopted requirements using quint-modeling/quint-lang: exact asset/payer/debtor admission, presence/amount duty membership, checked gross/fees/credit/debit/net/closing duties, signed interval/independent policy, atomic accepted transition/refusal. Cryptography is an explicit external premise bound to the whole candidate; it does not replace the circuit or qualify M8.
- [ ] Separate pure guard/update/property functions and tests; witnesses for accepted payment, state-only/zero-finance, selftransfer, partial/full/present-zero duties, every cap refusal, source/domain/time/authority mismatch, overflow and16/17 admission. Preserve actual UInt128 bounds using the existing TypeScript evaluator route. No blanket stutter to hide refusal/deadlock. Typecheck/test/run after increments;10000 samples/12steps/seed1011 and all named witnesses, no Quint verify/Apalache.
- [ ] Run full ordinary workspace and existing beta compatibility checks once after all production changes. Keep signature-only beta native terminology separate from transition proofs. If actual checks fail, diagnose and repair the responsible component with its author and scoped checks; retain failures.
- [ ] Measure exact integrated ZKIR/IR-model rows/k, instruction/public/field/replay counts, artifact/layout/IR hashes, source/scenario/Target/authority/payload/signature bindings for golden payment, independently authored payment, signed observation, zero/full16registry and max16assets/16duties combined shape. No SRS decoding/keygen/prove/MockProver. Refuse unsupported k>19 rather than silently shrink/drop slots or change spec. Record native signature basis measurements separately from whole-circuit costs.
- [ ] Freeze candidate and exact expected native controls. Commit model/tests/docs with a scoped ordinary report. Independent scoped spec/quality review must approve; source/native/ledger qualification is still open until Task4.

### Task 4: Bounded real native result and full delivery review

**Files:** Root-owned immutable runtime resource proposal/driver/captures; after actual execution add public fixed-QA evidence and scoped result document under `docs/evidence/` and `experiments/moriarty-language/evidence/`. Publish no user wallet, credential or private handoff material.

- [ ] Select one actual measured integrated signed financial circuit and exact pinned cached parameter file. Freeze driver, binary, source/Target/scenario/authority/payload/attached signature/proof statement and all resource ceilings/counts. If a parameter is missing, obtain a distinct bounded resource decision for that acquisition; do not silently download or try another k.
- [ ] Define honest genuine proof, producer self verify and fresh consumer-derived VK verification with exact complete effect readback. Define unchanged-proof altered financial/message/PK/time public-statement controls with correctly reconstructed metadata and independently valid replacement signed inputs when necessary so native refusal is reached. Separate early host cap/signature/metadata refusal from actual native verification. Raw-byte append/bitflip remain real verifier negatives.
- [ ] If forced hostile-point/canonicality proof attempts are included, enumerate and allocate them explicitly; use identical raw witness format with an honest positive. Only identified synthesis constraint refusal or produced-proof native rejection counts; arbitrary panic/prover error fails. Ordinary checker refusal never becomes native evidence.
- [ ] BOTH selected distinct resource reviewers must approve exact candidate/config before new native/SRS/keygen/prove/verifier work. Dispatch one bounded eligible experiment under existing admission, process-group timeout/memory/no-retry rules; record actual calls, pins, exits and complete Execution. Allocation is consumed regardless of result.
- [ ] Freeze final full candidate and actual result, obtain both fresh GPT6.1SolHIGH/Grok4.6HIGH whole-source/result audits independently. Changed bytes need current full audits. Missing/cancelled/max-turn reports do not approve; preserve failures and finish the same live reviewer process/session when possible.
- [ ] Publish reviewed scope and merge under existing user authorization. Keep actual ledger identity/ownership/history/nonce/time, phases/recovery, recursive private history, fraud replay/64KB transaction qualification, DUST/sponsorship, live deploy/call, full U0–U7/M1–M17 gates open until separately demonstrated. Continue the next whole-roadmap capability.
