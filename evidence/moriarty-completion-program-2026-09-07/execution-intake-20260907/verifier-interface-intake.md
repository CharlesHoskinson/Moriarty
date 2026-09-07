# Moriarty verifier interface source intake

source-inspected; direct-adapter interface-blocked under inspected pins; Preview compatibility unproven.

Bounded read-only retained local source inspection; no implementation, audit verdict, build, proof, network, native launch or Preview dispatch. Only /tmp packet files written.

## Main finding

A byte/tag adapter alone cannot implement the inspected native-to-ledger acceptance predicate: transcript hash, carried-accumulator discharge, application decider, VK version/architecture serialization, and ledger PI/effect binding are distinct boundaries. Direct reuse is source-incompatible under these pins; wrapper feasibility and Preview acceptance remain unknown.

## Source observations

- **O1 (source fact):** Native IvcVerifier::verify takes &IvcInstance<T> and &[u8]. It checks canonical vk_repr and T::decider, prepares the proof using CircuitTranscript<PoseidonState<F>>, requires transcript EOF, accumulates proof_acc with instance.acc, then checks the combined KZG pairing.
- **O2 (source fact):** Native prove_step returns raw Vec<u8>. IVC PI order is vk_repr, application format_public_input(state), AssignedAccumulator::as_public_input(acc); accumulator PI concatenates lhs and rhs MSM encodings. This is not the contract-call PI schema.
- **O3 (source fact):** IvcVerifier ctx/vk/params_verifier and IvcInstance vk_repr/state/acc are pub(crate). Instance exposes only state(). No public retained-verifier/instance deserialization interface appears in the inspected IVC module. resume_from requires full state, proof bytes, and Accumulator; it is not a persisted successor handoff format.
- **O4 (source fact):** Underlying native MidnightVK has read/write: architecture bytes, k u8, public input count u32 little-endian, then PLONK VK using requested SerdeFormat. This does not make private IvcVerifier.vk exportable through its public API.
- **O5 (source fact):** Inspected ledger Proof wraps Vec<u8> with proof[v5] tag. VerifierKey is verifier-key[v6], serialized as a length-prefixed vector of Processed MidnightVK bytes; declared VK payload above 50000 bytes rejects. VerifierKey::verify(params,proof,statement Iterator<Fr>) calls stdlib verify with TranscriptHash = blake2b_simd::State, DummyRelation, and no committed instance. It does not call the IVC decider or consume/discharge an IVC accumulator argument.
- **O6 (source fact):** Ledger ProofMarker::proof_verify uses the operation VK and ProofVersioned::V2; real verification requires proof-verifying and a non-mock mode. Default ledger features include proof-verifying, but a no-feature build returns Ok from proof_verify. The actual binary configuration must be pinned in any probe.
- **O7 (source fact):** ContractCall::public_inputs is binding_input(parent binding commitment), communication commitment, guaranteed transcript operation fields, then fallible transcript operation fields. Binding input includes contract address, entry point, gas and effects. Directly replacing these with IVC PI cannot establish contract-call acceptance/correspondence.
- **O8 (repository observation):** Native aggregation requires proofs 0.8.0/circuits 7.0.0 and R3 uses truncated-challenges. Retained ledger-8 source requires proofs ^0.7.0/circuits ^6.2.0. Preview client lock pins ledger-v8 8.1.0, and retained proof-server receipt pins 8.1.0 image digest. No exact Rust commit to deployed Preview node provenance was established by this inspection.
- **O9 (repository observation):** R3 retained evidence reports recursive VK row exhaustion at k17 and no recursive proof. Harness planned raw step-N.proof, full PI via field to_repr concatenation, and 54 u64 state limbs in little-endian bytes, but no retained VK export. A positive retained-proof probe cannot run from this failed R3 output.
- **O10 (source fact):** Compact compiler documents per-circuit .zkir/.bzkir and .prover/.verifier outputs for exported ledger-touching circuits. Its source recursion ban is not evidence of absence of backend recursion. No direct Compact IVC verifier entry point was identified in this bounded retained-source inspection.
- **O11 (inference):** A byte/tag adapter alone cannot implement the inspected native-to-ledger acceptance predicate: transcript hash, carried-accumulator discharge, application decider, VK version/architecture serialization, and ledger PI/effect binding are distinct boundaries. Direct reuse is source-incompatible under these pins; wrapper feasibility and Preview acceptance remain unknown.

## Pins and source locators

- https://github.com/midnightntwrk/midnight-zk.git — main — `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` (git rev-parse and retained R3 runner constant).
- https://github.com/midnightntwrk/midnight-ledger.git — ledger-8 — `a8ab82ba2124c36f92795c683e70bd888bc1d1fb` (git rev-parse; not asserted to equal Preview node build).
- https://github.com/LFDT-Minokawa/compact.git — main (retained receipt) — `11e7ec5abeecb99297c4faa74d30ef9adc7b51f3` (git rev-parse and retained receipt).
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/Cargo.toml`: dependencies/features; SHA256 `4e8382b6fad723cc538dbad5bee90e69fa7440577fc69373a31260366ed6fece`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/verifier.rs`: 27-91; SHA256 `8a8034299533228612cca9054427cc086d51deb7c2d6bdeb17748b1ec9284bd5`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/prover.rs`: 36-164; SHA256 `a47a6a44cbf127f41a99974183fa5f215d47f5b9b6f9c370b64521f331d5e4a2`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/circuit.rs`: 30-50;128-135; SHA256 `0d3231519552a051f04de9d810a1bda3d0b731323adb84f3aef9afa8ac4c3f3f`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/setup.rs`: 24-60; SHA256 `8edf56b4b1e64ee754d484bac12207b7633f26073800efc1e368d4eb30d6614e`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/zk_stdlib/src/interface.rs`: 97-153;565-590; SHA256 `f679793a7ab046a01bca3612247bb7b14c8c56273e3056f84baa51b8980d89e5`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/circuits/src/verifier/accumulator.rs`: 210-218; SHA256 `23e511ddbd2b01e227b7abe51e0e3d562b3acafa2a971d93e9aab44919bdcb87`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-ledger/transient-crypto/src/proofs.rs`: 64;109-135;377-438;501-566; SHA256 `06a6d770b72cfedac6e2cad81fccbf905eec9f370e1e915422b2aa3f423e6462`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-ledger/transient-crypto/Cargo.toml`: 31-32; SHA256 `43bf1bec220a8d24115b4a3c185c15c4b6a9db943f91d8675c75a7644e620b8f`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-ledger/ledger/src/structure.rs`: 410-490; SHA256 `eaa0e25fad3b1f1db0af0980be46a6754571ff2812b3d6380ac2aac647291c49`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-ledger/ledger/src/verify.rs`: 1854-1918; SHA256 `fe77c458885c09200f8f491b74583840a875bf78a8c2488b70fab01138ece5d1`.
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-ledger/ledger/Cargo.toml`: 10-18; SHA256 `b7441412ef52dc7148387ab784051d01f65d12d14ef1fddcebd274ae7be1906a`.
- `/home/charl/Moriarty/repos/LFDT-Minokawa/compact/compiler/compact-reference-proto.mdx`: 1697;3360-3380; SHA256 `cc55e9aafcb0ab8bd3d940761119a2572710c8b6a5a17543aac198f055530a16`.
- `/home/charl/Moriarty/experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs`: 470-516;582-604; SHA256 `ad8faaa473028f84ec0103f41d71882fd3d00a4aa95928ddfeb43a347ff74d32`.
- `/home/charl/Moriarty/experiments/moriarty-native-ivc-r3/run-native.py`: 14-15; SHA256 `66d6fa01cee40f7b0d84d866b4dd07744abc88b322ce7aa404206199f5b04bbb`.
- `/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/package-lock.json`: 564-569; SHA256 `4edec87652262deed0ed9a8812335fde4b616cc6af266a8b4602593efd3bcb40`.
- `/home/charl/Moriarty/experiments/moriarty-midnight-network/hello-world/docker-compose.yml`: 77-85; SHA256 `11216c654c86c86eaf8dfe7f3cc10306df4633d3c0b86848578198e2ee308799`.
- `/home/charl/Moriarty/evidence/moriarty-native-ivc-r3-2026-09-07/README.md`: native failure scope; SHA256 `d619f5df3b0f8d94b6a2e9ed8c3676e163d24d3a625ca020c4ac55dc10eea004`.
- `/home/charl/Moriarty/evidence/moriarty-native-ivc-r3-2026-09-07/srs-receipt.json`: all; SHA256 `71dc565f127fc5fbd29edf64f6e22b132501dec02b8e0bc94d54102768de238e`.
- `/home/charl/Moriarty/evidence/moriarty-midnight-network-2026-09-07/local-receipts.json`: proofServer image pin; SHA256 `b14f4691ec91cc3692618c4a0cfbf718a5a9b6261e3831d720e22466bf7e3fd5`.
- `/home/charl/Moriarty/raw/pcd-midnight-recursion-2026-09-06/repository-inspection.receipt.json`: retained upstream pins; SHA256 `3e6d35115a223d666203c9d16006d5f6079070d97175507c2a11b054e64bb271`.
- `/home/charl/Moriarty/openspec/MORIARTY-COMPLETION-PROGRAM.md`: Cross-package empirical decisions; SHA256 `d0b724dda63e13aa917147194bb94f305bb433b9056a7e93f9c14e5193fba1f4`.

## Unknowns

- Exact Preview node source/build/features and its relation to Rust ledger-8 checkout and ledger-v8 8.1.0 WASM.
- Checked wrapper realization and resource cost, including final pairing/decider verification inside the target acceptance relation.
- Native VK export, exact processed byte compatibility across proofs/circuits versions, and canonical accumulator persistence.
- Actual SRS binary presence and consistency with target verifier parameters; retained receipt only was inspected.
- Current installed Compact generated artifacts and ledger-v8 declarations were absent at guessed local paths; no exhaustive filesystem search performed.
- No multi-parent join or isolated witness handoff checked.
- No positive proof or rejection-control verifier run executed.

## Smallest decisive test — specified only

Can an authenticated full native IVC decider be enforced by the exact Preview-compatible contract-call verifier, with ledger-derived statement and effects?

Prerequisites:

- Pin the exact Preview-compatible ledger verifier build and map it to retained source; prove actual proof-verifying configuration, no mock mode.
- Review a checked Compact/ZKIR decider wrapper or a fully justified version-alignment design; alignment alone does not remove transcript/accumulator/PI gaps.
- Add reviewed public VK/instance/accumulator persistence with canonical decoding and EOF checks; retain SRS digest and all public context.
- Obtain MC03 retained positive proof under separately authorized bounded execution; failed original R3 supplies none.

Smallest probe:

- First, decode one canonical exported VK with the target VerifierKey::init and validate PI length/format; decoder success is only format evidence.
- Then verify one valid retained native final statement through the reviewed wrapper and the real local ledger ContractCall path using ledger-computed public_inputs, exact operation VK and actual effects.
- Require rejection of absent/truncated/altered/appended proof, wrong VK, changed state/domain/program/predecessor/effect binding, and a carried accumulator or decider violation that an ordinary outer-proof check alone would miss.
- Only after local real-verifier predicates pass, use the minimal authorized Preview positive case and bind exact deployment/VK/code/effects to the same lineage.

Stop conditions:

- No matching pinned target verifier or decider enforcement interface: record interface-blocked and stop direct adapter.
- Any positive or required rejection failure stops the probe; no native rerun or budget escalation follows automatically.

A host-computed verification bit, successful deserialization, ordinary native PLONK verification, mock mode, or local-only settlement cannot discharge native IVC acceptance on Preview.

None supplied by this packet; MC03/MC04 commands, budgets, and independent review gates remain required.

This source-only packet is not an independent acceptance audit and does not approve MC03/MC04 execution.
