# Independent native adapter expectations — specified before implementation review

Date: 2026-09-30 America/Denver. Requested reviewer route: GPT-6 Astra medium; no independently verifiable returned model/effort metadata is exposed in this delegated context. This is an independent expected-outcome specification, not an adapter source verdict or execution approval. The author adapter was not inspected. No build, runtime invocation, setup, SRS fetch, proof or network acquisition was performed. No prior reviewer verdict is used as evidence.

## Frozen scope and inspected sources

Target is the single pay invocation of the existing fixed kernel: Compact SHA256 `a3205e5bed3aabcbf06e7293be3384724498b7fbf1ae15e5c62a4c36486199db`, pay.zkir SHA256 `c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae`. Source JSON declares IR3.1, communications commitment=true,42 native-scalar message declarations followed by two Secp256k1-scalar signature declarations, and two native-scalar outputs. Do not assume this means44 serialized Fr elements: foreign scalars have their own official field representation.

NEXT-NATIVE-PROPOSAL.md pins midnight-zkir3.1.0-rc.1 commit `e82d81d25aabcc5f5092e2bd559083487f577914`, ledger commit `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`, midnight-proofs0.8.2. Its caps are proposals, not approved commands. All entries presently covered by native-route-source/manifest.json match their recorded hashes; manifest SHA256 is `8d39b08a5cb6e77a1391f35c34ec93203ad2b2de46a01dde4c217d2674e7374e`. Some later source snapshots are outside that manifest; do not imply manifest coverage for them.

Inspected official source snapshots transient-crypto/src/proofs.rs, ledger-wasm contract/transcript, onchain-runtime transcript, ZKIR public API/tests/Cargo metadata. Also inspected the already available exact ledger9 checkout under `/home/charl/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/9a8777c`: onchain-runtime-wasm/src/primitives.rs:227, ledger/src/construct.rs:515, ledger/src/prove.rs and ledger/src/verify.rs:1945. Inspected installed runtime0.20 proof-data.d.ts, circuit-context.js and onchain-runtime-v4.d.ts plus generated kernel JavaScript and fixed fixture. The standalone pinned ZKIR implementation internals beyond the supplied snapshots were not independently audited here. Its exact native check behavior must be observed, not inferred from a different ZKIR revision.

## Required extraction and conversion

Obtain actual `CallProofData` from the successful `contract.circuits.pay` result's `context.callProofDataTrace`. Require the expected single pay call and fixture contract identity. Retain original input, output, ordered publicTranscript, privateTranscriptOutputs, initial/final query contexts and effect projection. Do not manufacture these from desired post-state or accepted booleans.

The official runtime exports `proofDataIntoSerializedPreimage(input, output, publicTranscript, privateTranscriptOutputs, key_location)`. For this local check, prefer that tagged binary bridge over hand-written numeric/JSON packing. The exact pinned implementation:

- Parses `AlignedValue` inputs/output and `Vec<Op<ResultModeVerify>>`; invokes `ensure_ops_valid`.
- Uses `ValueReprAlignedValue(input).field_vec()` for direct IR inputs.
- Concatenates each private transcript output's `value_only_field_repr`, preserving order.
- Concatenates every public Op's `field_repr` for public_transcript_inputs. Do not include only push values or only ledger reads; opcode and structural fields matter.
- For every `Op::Popeq`, concatenates `result.value_only_field_repr` into public_transcript_outputs. Do not fold Popeq results into the wrong vector or add alignment metadata ad hoc.
- Sets binding_input=0, communication randomness=0 and communications_commitment=Some((transient_hash([0] + input.value_only_field_repr + output.value_only_field_repr),0)). This is an explicit public local fixture, not production randomness.
- Retains explicit key_location; omitted location defaults to dummy, which is not acceptable identity discipline for this adapter.

Decode using exact pinned tagged ProofPreimage serialization and reject truncation, tag/version mismatch and unconsumed trailing bytes. Compare adapter vectors and communication commitment against the official serializer, not only a self-generated JSON round trip. Preserve bytes/alignment exactly: Bytes1278 message and two Secp256k1 scalars have different encodings; Bytes32 return value is not a single integer. No modulo coercion, narrowing, reordering or guessed endianness may silently repair malformed input.

## Good invocation acceptance

The good control must use the existing public signature, exact1278-byte prefixed frame (SHA256 `e59e6a4d1d480387c206088663f6611d29dff98d1ac99dadfe2c582b27cc1a48`), configured key and actual fixture constructor/prestate. It must produce owner/recipient/fee8990/1000/10, allowance8990/1010, work9/1, consumed nonce, revision1, the exact returned head, two native destination claims1000/10 and output debit1010 with no extra financial effect.

Run `ProofPreimage::check(&exact_ir)` / matching IrSource check only after reviewed build/execution allocation. Required outcome is successful native witness-generation/relation check with retained skip vector and explicit input/transcript lengths. Passing a host fixture comparison or JavaScript runtime is insufficient. Report check failure as evidence of an adapter/toolchain/relation gap; do not drop constraints, disable communications commitment, substitute a smaller IR, or switch versions to force success.

## Independent failure matrix

All rows below are specified-only until separately executed. Each control starts from a fresh copy of the frozen good artifact. Record mutation location, old/new digest, refusal layer, native error and whether native check was actually reached. A host refusal must not be counted as a native-constraint refusal.

| Mutation | Required adapter outcome and evidence layer |
|---|---|
| One message byte changed, same signature | Refuse. For a direct decoded-preimage control, update communication commitment consistently so failure demonstrates the fixed message/signature relation rather than only stale commitment. Also test the stale-commitment variant separately. |
| Signature r changed to another valid scalar; s stays canonical | Refuse internally when checking otherwise coherent vectors/commitment. Tests the ECDSA relation rather than JavaScript shape validation. |
| s→n−s, zero r/s, or scalar outside range | Refuse. Low-S/nonzero are kernel predicates; out-of-range may properly fail typed decoding first. Report these layers separately. |
| Read result for initial head or revision changed, with relevant Popeq/Op representation kept coherent | Refuse internally at stale-head/revision or transcript consistency. A modified external snapshot alone, never projected into preimage, is only a host mismatch test. |
| Consumed nonce, insufficient allowance/work, underfunded or mismatched escrow read | Refuse through the corresponding kernel predicate using coherent native read vectors. No financial acceptance from a runtime exception alone. |
| Amount/destination/color in a financial write or sendUnshielded Op transcript changed | Refuse internal transcript consistency when it conflicts with the fixed relation. Map the selected Op and field to actual native effect, do not mutate a random scalar and call that an effect test. |
| Effect summary changed outside serialized Op transcript | Host projection must refuse; local binding0 check may remain valid because this summary is not itself in ProofPreimage. Do not claim native detection of bytes it never receives. |
| Returned head/output changed | Refuse communication commitment/output consistency when commitment is derived from mutated output and native kernel computes the original output. Also reject host output/result mismatch. |
| Communication commitment changed alone, randomness changed alone, or commitment omitted | Refuse for this commitment-enabled IR. Recomputing a coherent commitment with different allowed public randomness can define a valid alternative local statement; frozen fixture policy must reject it at host boundary if randomness0 is required. |
| Input/output alignment, lengths, scalar widths or ordering changed | Refuse via canonical typed conversion or frozen schema checks; semantic native failure where applicable. Do not assume all alternate alignment descriptions necessarily change flattened fields. |
| Public inputs/outputs truncated, appended, reordered, Popeq result omitted; private witness stream fabricated | Refuse. Retain precise boundary/error; never pad missing values or ignore extras. A panic is a failed adapter robustness check, not a clean successful negative. |
| key_location changed or points to another IR/key set | Host/resolver refusal. check(&ir) does not resolve the string and cannot prove key identity. |
| binding_input changed alone | Host local-fixture refusal if nonzero; do not expect relation failure because this input is deliberately arbitrary and bound into the eventual public statement. |
| IR/source/runtime/dependency digest changed | Host identity refusal before check. An accepted proof for another circuit is irrelevant. |

Add an unmutated good control after mutation processing to detect accidental shared-state contamination. Native checks should retain typed errors and stop on inconsistent conversion; no setup/proving is necessary to resolve these projection questions.

## Public statement and later ledger correspondence

`ProofPreimage::check` returns skips; it does not generate a proof or verifier acceptance. Retain `Vec<Option<usize>>` exactly. Public statement layout is not safely reconstructed by concatenating every original vector: skip handling in the matching prover changes the emitted transcript, inserts Noop runs and updates gas. Preserve official preprocessing/proving statement output and independently compare it with the matching final consumer.

Pinned ledger `ContractCallExt::construct_proof` takes guaranteed transcript Ops followed by fallible Ops, the same field representations and Popeq results. It initially sets binding_input0, but the ledger proving consumer overwrites it after transcript/skip/cost processing. `ContractCall::public_inputs` is the binding input, communication commitment, then final guaranteed and fallible Op field representations.

The ledger binding input hashes the versioned domain tag, contract address, entry point, guaranteed gas/effects, optional fallible gas/effects, guaranteed instruction count and transaction binding commitment. It uses the first31 SHA256 bytes interpreted little-endian in Fr. Use the official consumer; do not recreate it by informal JSON hashing. The local helper's zero binding is sufficient only for the explicitly scoped local relation fixture. It does not bind a real transaction, contract instance, state root, fees or effects summary. Later standalone verifier tests must mutate the exact returned public statement/proof; changing preimage.binding_input and generating a fresh proof is not a negative verification test.

The input/output communication commitment commits the actual signature inputs and kernel output under the specified randomness. It does not authenticate constructor authority or chain state. The beta intent still lacks instance custody binding; head/round/key/account/asset mappings remain fixture assumptions. Same-snapshot durable exclusion, real escrow funding/recipient settlement and Preview ledger8 versus ledger9 compatibility remain open after any successful local check or local proof.

## Evidence required to review adapter result

Provide exact source/Cargo.lock/toolchain hashes and explicit key_location; exact runtime extraction command; original public ProofData and serialized tagged preimage hashes; official conversion comparison; input/output/transcript counts; named mutations with separate host/native outcomes; good native check result and skip vector. Resource caps and no-network behavior must be enforced by the separately approved execution path. This document authorizes no build, new acquisition, SRS/setup, key generation, proof, wallet debit or chain dispatch.
