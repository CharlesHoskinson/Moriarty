# Native call proving source adapter

`mori-ledger` now has source APIs for the pinned SDK's actual call proving and verification route. They are authored and compiled source, with ordinary preparation checks. Native cost selection, key generation, proving and cryptographic verification have not run for this candidate. Circuit fit and compatible trusted parameters remain unknown.

This is a component of U1/U2. The complete U0–U7/M1–M17 language objective remains open. This adapter does not establish financial settlement, authenticated state, transaction authorization, unique consumption, full source-to-constraint correspondence or recursive history/private composition.

## Preparation and the raw owner

`prepare::prepare_native_call` recompiles the selected source action and uses the existing production preparation path. Its private `PreparedNativeCall` owns the raw unsigned `Intent<(), ProofPreimageMarker, PedersenRandomness, InMemoryDB>`. `raw_intent()` returns an immutable reference. `bundle()` exposes ordinary readback and artifacts; neither accessor permits mutation of the owned intent or expected projection.

Checked preparation publishes `raw-intent.bin` separately from `raw-call.bin` and `processed-call.bin`. The processed call remains an inspection projection. The SDK inserts Noops and adjusts gas when proving the raw intent, so a processed call must not be passed through proving again. Existing `local-unproven` and `local-relation-checked-unproven` preparation qualifications remain unchanged. No preparation result asserts native proof verification.

## Derived material and actual SDK APIs

`proving::generate_call_keys(source, program, deployment, action, params, limits)` recompiles original source and validated static bindings before invoking actual `IrSource::k` and `IrSource::keygen`. It does not accept a caller-built `relation::Artifact`, arbitrary key bytes or a caller-selected expected hash. It requires an already initialized `ParamsProver` object of exactly the circuit degree and explicit caller-selected degree and serialized-key work bounds. There are no default PK/SRS size envelopes and no file reader or parameter download.

The private `DerivedCallMaterial` owns source IR, manifest, PK/VK bytes and an immutable parameter snapshot. Cloning it shares that snapshot so separately prepared A/B calls can reuse one generated key with fresh RNGs; actual proving work still needs its own resources. `SourceCallResolver` only resolves the selected action from that once-generated snapshot. It never dispatches Midnight built-in names or fetches network material. `SourceCallProver` implements the actual SDK `ProvingProvider`; check pins the action and source relation, prove requires the final binding overwrite and unchanged activity, and then calls `ProofPreimage::prove::<IrSource>`. `split` forks cryptographic RNG streams and shares the immutable source/key/parameter snapshot.

A `PublicDerivedCallKey` is available only from generated material. It has private fields and no deserializer, imported-bundle constructor or arbitrary expected-key constructor. It contains public IR/manifest/VK bytes and no PK or prover parameters. Its exported bytes cannot recreate this capability or certify a trusted durable import. The declared source fingerprint includes preparation, lowering, layout, custody, profile, codec, provider, Core, lexer, Cargo and lockfile bytes; the relation manifest separately pins its compiler modules. This fingerprint is not a certificate for the complete Rust build tuple, correct keygen execution or authenticated parameters.

The capability records actual SDK derivation from source **with supplied initialized parameters**. It does not authenticate SRS provenance or trusted setup. Independently trusted compatible SRS and independently checked exact key derivation remain native qualification preconditions. A caller's self-consistent digest is not such evidence.

## Proving and independent verification

`prove_prepared_call(prepared, material, operation, rng)` consumes the raw owner through public SDK `Intent::prove`. That is the sole raw proving route. Before native work it requires the exact derived source identity and supplied operation's current VK and IR, with no legacy VK. `PublicDerivedCallKey::local_operation()` constructs such a local SDK operation object; it neither installs an operation nor authenticates a ledger policy.

The final consumer compares the whole unsigned intent against the independently checked processed expectation, including offers, TTL, parent randomness, action count/order and exact transcripts. It rejects extra calls, deployments and maintenance actions. It requires V3 proof, exact gas/effects/version and SDK public inputs, replays the returned processed transcript on the admitted predecessor, and compares full semantic and charged state, effects, events and measured gas. Final prospective UTXO identities are recomputed from the final intent hash.

`verify_proven_call(independently_prepared, public_key, operation, candidate_intent)` requires the same exact source and operation identities, validates the complete final result, then calls `<ProofMarker as ProofKind<InMemoryDB>>::proof_verify` in `ProofVerificationMode::Real`. The ledger dependency explicitly enables `proof-verifying`, including when Moriarty is built without default features. No mock verifier or preimage/erased marker supplies verification success. The public key contains no PK, prover parameters or prover cache.

The result is a local unsigned call-proof predicate under supplied parameters and a supplied operation. It records remaining SRS/derivation, source correspondence, installed-state, authorization, replay, fees, apply, settlement and history predicates. Full transaction well-formedness and ledger acceptance are separate APIs and remain open here.

## Current blockers

The legacy deployment profile deliberately rejects any installed VK or IR in `layout::decode_predecessor`. Safe CLI key generation/proving/verification against installed predecessor state requires a separately reviewed versioned profile, complete action-map policy, installed key authorization and source/VK/parameter correspondence. This adapter does not weaken the old refusal, relabel forged state, or invent that policy. There is no `prove-call`, `verify-call` or `keygen-call` CLI success stub.

Durable key export/reopen/import, cold verifier artifact admission, authenticated deployment and operation lookup remain dependent on that design. Native model degree/fit, exact offline SRS provenance/compatibility, real PK/VK/SRS sizes, numerical process budgets and the admitted positive/hostile matrix are also unresolved. Source-only compilation or ordinary relation checks cannot establish these predicates. The candidate remains unmerged pending complete fresh independent audits and any later native resource decision.
