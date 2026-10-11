# Target-bound public reference replay design

Status: design for the approved M6/U0/U2 language slice, 2026-10-11. Feature logic is not implemented by this packet. The sole writer waits for root's independent expectation/design review clearance before implementation. Author routing is GPT-6.1 Sol HIGH; fresh independent GPT-6.1 Sol HIGH and Grok 4.6 HIGH assess the exact candidate. No new user permission flow, resource allocation or external protocol is introduced.

Basis: commit `31b392b972ecb054b8385019b1184f0b2ead80b7`, tree `89e9f5d00ed648dc243d6dbe249d6c041ad110e8`. This refines the local replay portion of [the approved October 10 design](2026-10-10-fenrir-integration-design.md). All original scope, exits, failures, resource charges and retained evidence remain in force.

## Deliverable and inspected facts

A developer generates a versioned public replay sidecar through `mori simulate --trace --target`, then independently consumes it through `mori replay` with separately trusted source/action/Scenario/Target. Signed mode also requires separately trusted ExpectedAuthority, logical `at` and an actual signature attachment. The consumer reconstructs the actual pipeline and reports the first differing step and deterministic field path. Rehashing a self-consistent false record never supplies authority.

Repository observations at the basis:

- `crates/mori-core/src/ir.rs`: Step has index/statement/value/snapshot/effects; Execution has semantics/action/pre/post/steps/effects. UInt is checked u128 serialized as canonical decimal strings. Zero liabilities remain present.
- `crates/mori-core/src/runtime.rs`: simulate validates and executes on a clone; refusal returns Error without partial Execution. Existing replay compares identity/pre, ordered steps, complete effects and post, but identifies no nested field.
- `crates/mori-core/src/json.rs`: decoded duplicate keys/trailing data refuse; reader bounds are 1,048,576 input bytes, depth32 and65,536 nodes. ASCII-safe pretty export reserves its final newline and uses the same reader. `intent::canonical` sorts object keys and preserves array order but itself supplies no bound.
- `crates/mori-backend/src/signed_relation.rs`: shared reconstruction parses raw source, compiles against actual Target, selects its circuit, checks source-wide registered duties, simulates, reconstructs financial/public values and signed payload/context. `accept` validates the exact separately supplied wrapper and strict FENS attachment without constructing a private witness. `accept_intent` additionally constructs a transient witness for the ordinary relation checker.
- `crates/mori-backend/src/proving.rs`: private Claim is exactly ProofBundle without proof_hex. Its canonical bytes bind native revision3; verification independently reconstructs them before parameters. Existing preparation constructs those bytes without reading parameters, then proving/verification reads SRS separately.
- `crates/mori/src/commands/simulate.rs` and `test.rs`: actual source/scenario callers exist; ordinary simulate prints its established execution wrapper and test checks separately supplied complete effects/post. Neither consumes a bound sidecar today.

External source observations, read-only: `/home/charl/projectfenrir` HEAD `ec8da3be532f29e5e92ff7b1804b7dff0bc89b15`, `research/DESIGN-v4.md`, and `/home/charl/minocrab` HEAD `9f4d6a62abeb882fc2987d43cd6e0ba0cfb279cd`, `crates/fenrir-contracts/src/settlement.rs`, confirm the distinction between desired typed STF replay and currently exposed Group A fraud methods (double-nullifier/omitted-forced plus conflict paths). Mutable MinoCrab work is not a dependency pin. No complete typed statement-step challenge/opening interface is established. The existing reduced Anchor circuit-set and SHA/depth/timing gaps in the approved design remain. This sidecar invents no fraud transport, leaf encoding, ledger consumer or deployment identity.

## Approaches

1. Separate bounded sidecar plus a focused backend consumer (selected): reuse reconstruction and unchanged v3 Claim framing, add detailed Core comparison and actual CLI dispatch. It preserves current programs, signatures and proofs while supplying trustworthy local replay.
2. Extend Step/Execution with target/work fields: easy to inspect, but changes execution_hash and native/signing contracts. Rejected for this slice.
3. Standalone script validating record hashes: small, but cannot establish trusted pipeline correspondence or the production CLI consumer. Rejected.

No new crate/runtime framework is needed. Core owns execution comparison; backend owns source-to-target/authorization identities; CLI owns bounded file inputs and output.

## Compatibility boundary

Program, Step, Execution, Target, prepared intent, signed payload/wrapper, artifact/ABI/relation/proof/binding revision3 serialization and meanings remain byte-identical. Existing `mori_core::replay` signature, guard behavior, Error codes/messages/step and ordinary CLI simulate/test/intent/verify-intent/prove/verify remain unchanged. Detailed diagnostics use a separate API.

Extract existing private Claim into a public typed `proving::TransitionClaim` and a single crate-private constructor. This is an API refactor of existing bytes, with no field addition, removal, rename, default or ordering/framing change. Native prepare and replay share that constructor. Claim validation shares current ProofBundle v3 rules. Native verify still compares the full independently reconstructed Claim and derives its own native statement/VK; the sidecar never replaces that path.

Source hashing remains exact raw UTF-8 bytes; Program hashing remains canonical full parsed Program; Scenario hashing remains its admitted supplied arrays/maps before snapshot normalization; Target hashing remains exact canonical trusted Target including signed_policy and limits. Semantically equal source spellings have distinct raw hashes; reordered Scenario arrays can produce equal normalized execution but different Scenario hashes. No normalization changes these existing identities.

The signed relation_hash remains the existing context_hash, not the new selected-ZKIR hash. Current signing bytes and 18-element context preimage remain exact. Claim binding remains SHA256(C([BINDING_VERSION, Claim])), interpreted as the full LE256 integer reduced modulo Fr p by `Fr(lo) + 2^128 * Fr(hi)`. This is distinct from paired context/hash limbs; it is not redefined by the sidecar hash.

## Exact public sidecar schema

New `mori_backend::replay` types use strict serde decoding and Core's bounded parser. Every listed field is required; unknown/missing fields refuse. Nullable fields require explicit null when absent through `deserialize_with = Option::deserialize`; missing is not equivalent to null. Derived counts/positions are bounded JSON integers; UInt uses existing string serialization. Record decoding explicitly checks the presence of nested `execution.steps[].value` before typed decoding: Core Step currently allows omission of its Option value, and that existing ordinary decoder remains unchanged.

```rust
pub const RECORD_FORMAT: &str = "moriarty-reference-replay/1";
pub struct ReplayRecord { pub body: ReplayBody, pub record_hash: String }
pub struct ReplayBody {
    pub format: String,                       // exact RECORD_FORMAT
    pub language: String,                     // exact moriarty/1
    pub semantics: String,                    // exact moriarty-core/1
    pub network: String,                      // reconstructed trusted Target
    pub anchor: String,                       // reconstructed trusted Target
    pub program_name: String,                 // actual parsed Program.name
    pub program_address: Option<String>,      // signed Target policy; null unsigned
    pub artifact_format: String,              // exact moriarty-zkir-artifact/3
    pub zkir_hash: String,                     // selected actual circuit.zkir bytes
    pub authority_hash: Option<String>,       // trusted ExpectedAuthority; null unsigned
    pub input_hash: String,
    pub trace_hash: String,
    pub source_step_count: u32,
    pub claim: proving::TransitionClaim,
    pub execution: mori_core::Execution,
}
```

TransitionClaim contains EXACT existing Claim fields/types: `abi:String`, `relation:String`, `binding:String`, `profile:String`, `registry:Vec<LiabilityIdentity>`, `format:String`, `source_hash:String`, `program_hash:String`, `target_hash:String`, `scenario_hash:String`, `execution_hash:String`, `action:String`, `circuit_k:u8`, `authorization_mode:String`, `context_hash:String`, `financial_bindings:Vec<PublicBinding>`, `authorization_bindings:Vec<AuthorizationBinding>`, `public_values:Vec<String>`, `authorization:Option<ClaimAuthorization>`. Its public declaration order preserves the current Claim. ClaimAuthorization remains `{payload_hash,message_hex,public_key_hex,at}` with at:UInt. Unsigned authorization is required explicit null; signed mode requires the current object. Existing descriptor nulls/positions and canonical native-field/key encodings retain their current validation.

Source/action/Scenario/Target identities and the full registry are available through Claim; no embedded raw source, Scenario, Target or authority is treated as a replay root. Program address is available only in signed_policy today; unsigned records use null and name the actual source program. Null never claims an authenticated deployment. Signed authority identity commits all actual ExpectedAuthority fields, including nonce/predecessor/validity/caps; `claim.authorization.at` binds the separately supplied logical time. Record formation is possible only after signed attachment acceptance; no signature-valid/native-valid/status flags are in the schema.

Execution supplies full normalized pre/post, every ordered statement/value/snapshot/effects and complete ordered effects, including untouched balances, unchanged states, untouched duties and present-zero duties. Snapshot arrays retain existing canonical runtime ordering. Claim registry/financial slots bind the source-fixed duty frame independently. Absence and present-zero remain distinct; no sparse duty projection is added.

Public Claim values include already public financial and authorization statement values. Never serialize private transcript/witness vectors, private point memory, attachment signature_hex/R/s, SRS, proving/verifying keys, credentials or private state. Transient reconstruction Fr values are the existing public statement only. Signed replay uses `accept`, never `witness`, and exports no attachment. An alternative valid same-payload attachment produces the same public record; an invalid attachment refuses even against a matching record.

## Canonical hashes and bounds

Let C be unchanged `mori_core::intent::canonical`, SHA be SHA256, and Digest be exactly64 lowercase hex digits. All object keys sort recursively; arrays stay ordered; UTF-8 serde_json escaping is exact, without Unicode normalization, whitespace or final newline. Do not hash the pretty/escaped display output.

Every new canonical JSON hash uses a helper that first admits its value through Core export/parse and admits C(value) through Core parse, then hashes those exact C bytes. This bounds publication and computation without changing existing digest helper behavior or existing v3 preimages. Inputs are already bounded/typed; raw source is bounded by the unchanged parser to65,536 bytes. Selected ZKIR is produced under trusted Target's existing artifact limits; its raw-byte hash is not a JSON hash. No record truncation or partial trace export is allowed.

Exact new preimages:

- `zkir_hash = SHA(selected_circuit.zkir.as_bytes())`.
- Signed `authority_hash = SHA(C(ExpectedAuthority))`; unsigned authority_hash is null.
- `input_hash = SHA(C(["moriarty-reference-input/1", claim.source_hash, claim.program_hash, claim.action, claim.scenario_hash, claim.target_hash, authority_hash, at]))`, where at is `claim.authorization.at` in its UInt string encoding for signed mode, null unsigned.
- `trace_hash = SHA(C(["moriarty-reference-trace/1", execution]))`.
- `record_hash = SHA(C([RECORD_FORMAT, body]))`. Body includes trace/input hashes, static identities, Claim and unchanged Execution. It does not include record_hash, so no cycle exists.
- Claim source/program/Scenario/Target/Execution/context hashes and public statement remain the existing hashes computed by the actual pipeline; no extra domain tags are inserted into them.

Record read caps: input<=1,048,576 bytes, depth<=32, nodes<=65,536, strict typed schema and digest/native encodings. Record generation validates final ASCII-safe pretty export including final newline through the same profile before returning/writing it. Final record size may refuse an otherwise admitted local execution; rejection emits no proposed financial effects. No 64 KB transaction, fraud-k, replay-time or maximum-work certificate follows from these bounds.

`source_step_count` is exactly the selected source action's statement count and completed Execution.steps.len(), <=1,024, recomputed by replay. It counts each statement once, including Let/Requires/Ensures; it does not count expression visits, compiler instructions, allocations, curve work or elapsed time. It is an explicitly limited reference measure. No logical-work meter or native-bound Execution field is added.

## API, guard order and first divergence

```rust
// Core: new APIs; existing replay stays unchanged.
pub struct ReplayDifference { pub step: Option<u16>, pub field: String }
pub fn compare_execution(actual: &Execution, candidate: &Execution)
    -> Result<(), ReplayDifference>;
pub fn replay_detailed(program: &Program, action: &str, scenario: &Scenario,
                       candidate: &Execution) -> Result<(), DetailedReplayError>;
pub enum DetailedReplayError { Execution(Error), Divergence(ReplayDifference) }

// Backend: no ProofOptions or params path; signed mode has no fallback.
pub enum ReplayAuthorization<'a> {
    Unsigned,
    Signed { authority: &'a ExpectedAuthority, at: UInt, signature: &'a SignedIntent },
}
pub fn generate(source: &str, action: &str, scenario: &Scenario, target: &Target,
                authorization: ReplayAuthorization<'_>) -> Result<ReplayRecord, Error>;
pub fn replay(source: &str, action: &str, scenario: &Scenario, target: &Target,
              authorization: ReplayAuthorization<'_>, candidate: &ReplayRecord)
    -> Result<(), ReplayError>;
pub enum ReplayError { Input(Error), Divergence(ReplayDifference) }
impl ReplayRecord { pub fn from_json(text: &str) -> Result<Self, Error>; }
```

Backend Error remains existing backend error; detailed failure schema is separately serialized as a tagged enum. `from_json` performs bounded parser/schema/version/digest/native validation; it does not replay, recompute execution/message/trace/record digests or use candidate data to construct roots. Shape validation must not enforce digest equality before ordered semantic comparison. Direct typed API calls also apply the same profile/schema validation before comparison, so decoding is not the only bound.

Guard order is normative:

1. CLI validates complete argument groups and bounds each input read. Sidecar bounded parsing/schema/version/encoding precedes candidate traversal. Source parse or malformed input refusal contains no record/Execution/effects.
2. Enforce actual Target mode against authorization mode. Signed mode validates actual wrapper, then existing trusted reconstruct/authority/time/source-wide admission/caps and strict attachment equation through `accept`. Unsigned mode uses reconstruct(None). Target compile limits, scenario registry, arithmetic and execution refusals retain current pipeline order. No parameters or private witness are constructed. Loading actual ZKIR to obtain circuit_k is permitted and has no SRS operation.
3. Construct the independent actual record with shared Claim construction. Compare static/trusted identity fields in this order: format, language, semantics, network, anchor, program_name, program_address, artifact_format, zkir_hash, authority_hash, input_hash, then Claim abi/relation/binding/profile/registry/format/source_hash/program_hash/target_hash/scenario_hash/action/circuit_k/authorization_mode/context_hash/financial_bindings/authorization_bindings; signed Claim authorization payload_hash/public_key_hex/at are identity fields. Each failure is step:null and exact JSON Pointer under `/body`.
4. Compare Execution in this order: semantics, action, pre, then steps at ascending ordinal. Within each step compare index, statement, value, snapshot, effects in that explicit order. For nested objects visit decoded keys in Rust lexical order; for arrays compare elements ascending, then length. A missing/extra step differs at its first absent ordinal with field `/steps/N`; both changes and later mutations cannot hide an earlier step. Step means ordinal, not candidate index. First differing step field is returned immediately.
5. After steps, compare complete effects then final post with step:null. Complete effects compare order/multiplicity/value/identity; post includes all fields and untouched/present-zero duties.
6. Compare derived Claim execution_hash/public_values and signed authorization.message_hex, then source_step_count, trace_hash, and record_hash. Check hashes against independently computed actual values, not merely internal candidate consistency. A digest changed together with trace content cannot mask the earlier semantic difference. A digest-only mutation reports its own path.

Core paths are RFC6901 JSON Pointers: `~` escapes as `~0`, `/` as `~1`; root empty string, missing child its child path, array length path is the array path. Backend prefixes Execution paths with `/body/execution`; examples: `/body/execution/steps/1/effects/0/amount`, `/body/execution/steps/2/snapshot/balances/0/amount`, `/body/execution/post/liabilities/2/amount`. No unstable Debug formatting or HashMap iteration defines field order. A nested object missing/extra key differs at the first lexical union key. Null is legitimate only for declared nullable fields such as Step.value and signed/unsigned record/Claim options; other nulls refuse schema decoding. Successful replay returns no candidate-derived effects or native claims.

## Actual CLI workflow

Keep ordinary `mori simulate FILE --action NAME --scenario FILE` output/behavior unchanged. Add `--trace` flag and `--target FILE` with required pairing. Trace generation outputs ReplayRecord JSON directly so stdout redirection produces a valid input record. With signed Target require the complete `--profile moriarty-jubjub-owner-intent/1 --authority FILE --at N --signature FILE` group; explicit signed profile through unsigned Target and partial groups refuse. Supplying those new options without --trace refuses.

Add `mori replay FILE --action NAME --scenario FILE --target FILE --record FILE`, plus the same signed group when applicable. All five trusted inputs are separate arguments; none are loaded from the candidate. Replay success prints `{status:"accepted",qualification:"local-reference-replay",record_hash:<reconstructed digest>}` and exits0. Divergence prints `{status:"rejected",qualification:"local-reference-replay",error:{kind:"divergence",step:<ordinal-or-null>,field:<pointer>}}` and exits1. Reconstruction failure prints rejected error and exits1, with no Execution/effects. File/clap/schema errors use existing diagnostics and nonzero exit. Perform full bounded export before stdout emission.

```bash
mori simulate payment.mori --action settle --scenario payment.scenario.json \
  --trace --target payment.target.json --profile moriarty-jubjub-owner-intent/1 \
  --authority payment.authority.json --at 15 --signature payment.signed.json > payment.replay.json
mori replay payment.mori --action settle --scenario payment.scenario.json \
  --target payment.target.json --record payment.replay.json \
  --profile moriarty-jubjub-owner-intent/1 --authority payment.authority.json \
  --at 15 --signature payment.signed.json
```

These filenames describe actual public test inputs assembled from `deliverables/native-owner-intent-2026-10-11/inputs/` or existing public example inputs; CLI tests write fixture copies in a temporary directory. No signer/proof generation occurs. Ordinary `mori test` remains compatible; CLI integration tests call both production commands.

## Decisive checks and narrow claims

Independent expected outcomes are supplied separately under `.moriarty-dev/runtime/bound-replay-20261011/independent-expected-outcomes.md` and `independent-oracle-cases.json`; implementation consumes their public oracle, not author-generated expectations. Frozen SHA256s are `8e906fb45edb96c6df74d9700d88dc24e872adac028d968c6690d797d9c70c96` for the Markdown and `30aa690ee081987728a310e32ac93020dcf7c7d5a890044e348dd3e21fd326f3` for JSON. Contrasting hand-lowered expression IDs are an explicit parser assumption to check; independent arithmetic/effect/snapshot expectations remain the oracle. Honest payment ends with alice28/bob80/treasury2, paid70, duties payment30/reserve5/zero0 present, ordered Transfer70/Fee2/Liability50->30/State0->70/Emit70; caps72/2/68/35. A contrasting independently authored fee-first collection program uses the same parser/compiler/reconstruction/CLI, with complete independently fixed step snapshots. No fixture-name dispatch is permitted.

Test source whitespace identity, action, supplied Scenario including reorder, full Target/limits/network/Anchor/policy/address, trusted authority and at changes. Tamper each early statement/index/value/snapshot/effect; add later mutations and recompute every new hash and Claim execution_hash to prove the first semantic divergence survives. Reorder/drop/add steps/effects; change final post, untouched balances/states/duties and present-zero duty presence. Change ZKIR/context/Claim descriptors/public values/authorization identities and record hashes. Reject malformed UTF-8/JSON, duplicate decoded keys, unknown/missing fields, forbidden nulls, noncanonical digests/numbers/native encodings, trailing data and input/depth/node/export bounds. Signed alternative valid same-payload attachment accepts; wrong/missing/invalid/cross-payload attachment refuses. Every source/requires/late-ensures/arithmetic/target/authorization failure returns no proposed effects.

Compatibility tests compare existing golden prepared/signing/context values and exact Program/Step/Execution canonical bytes with the unchanged baseline. Independently reconstruct the published payment ProofBundle's Claim by removing only proof_hex and compare its full canonical bytes, old execution_hash and old binding preimage with the new shared Claim, without native verification or parameters. Existing signatures/preparation/artifact/ABI snapshots retain exact identities.

Only local canonical target-bound reference replay is delivered. This does not authenticate ledger state, account/key ownership, current time, nonce consumption, native verification, history/recursion/private handoff, source/5, full U1/U2/U0, full M6 fraud game, transaction64KB, DUST or devnet qualification. The genuine selected local signed native result already recorded is retained; this work neither reruns nor generalizes it. The implementation documentation corrects stale nonblocking wording at NATIVE-CORE lines242–244/291–292 and Quint README lines281–282 to distinguish that genuine selected result from still-open broader qualification.

No SRS reads, keygen, proving, native verification, MockProver, ignored/native tests, budget reuse, dependencies/tools installation or upgrade, network, push or merge. Use Rust1.99 locked offline and the assigned shared target directory. Only the assigned worktree is writable; Fenrir, original/signed-QA checkouts, public native receipts and retained history stay untouched.
