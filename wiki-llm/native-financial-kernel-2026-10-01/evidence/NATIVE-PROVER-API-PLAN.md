# Pinned full-kernel prover/verifier API plan (source only)

No files in adapter/kernel/repository changed. No fetch, build, check, setup, SRS or proof. Current delivery remains the native adapter source check. k17/114250 is a model, not measured proof fit. Four Compact attempts and old R3 debt remain consumed; this plan grants no future execution authority.

## Actual inspected sources

ZKIR root `/home/charl/.cargo/git/checkouts/midnight-zkir-d3b0dbbf065d6ece/e82d81d`, official full revision e82d81d25aabcc5f5092e2bd559083487f577914. `zkir/src/ir.rs:73-103` implements Zkir::check/prove: preprocess, copy actual pis and pi_skips, call native stdlib prove, return `(Proof,Vec<Fr>,Vec<Option<usize>>)`. `zkir/src/ir_vm.rs:230` preprocess is crate-private; starts public vector with binding, optional communication commitment, handles actual IR Impact at598 with skipped zero-padding counts. Do not assume IR declaration count equals scalar count.

Ledger root `/home/charl/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/9a8777c`, official full revision9a8777c4d035fc7f38ae286bcf5f8656668efd9f. `transient-crypto/src/proofs.rs:745-775` ProofPreimage::prove::<IrSource> resolves keys, loads tagged IR/PK/VK, calls trait prove, performs native self-verification, returns ONLY `(Proof,pi_skips)`. It intentionally discards actual pis. `:554` VerifierKey::verify calls stdlib verify with actual Fr statement; never mock_verify. `:112-117` ParamsVerifier::read is public; ParamsProver::as_verifier is crate-private. `:136` Proof is public Vec<u8> wrapper. `:671` Resolver resolves explicit KeyLocation to ProvingKeyMaterial.

`ledger/src/prove.rs:252-291` inserts native Op::Noop from pi_skips into guaranteed/fallible programs; `:370` overwrites binding with final native contract-call binding. `ledger/src/verify.rs:1956` derives statement `[binding_input, communication_commitment]` plus each actual native operation.field_repr. `:1972-2011` binding hash covers contract address, entrypoint, costs/effects, guaranteed count and transaction binding commitment. Local binding0 proof is not this contract call.

Registry `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`: `midnight-zk-stdlib-2.3.5/src/lib.rs:2017` exact single verifier delegates to BlstPLONK; `src/utils/plonk_api.rs:154` verifies transcript EOF using assert_empty before acceptance. `midnight-proofs-0.8.2/src/transcript/mod.rs:140` checks remainingbytes. Native final verification and raw proof EOF are enforced by that path; serialized container EOF is still caller responsibility.

## Smallest concrete design

One external fixture-proof executable with modes prove and verify; exact separately reviewed source/lock hashes. Tokio/current-thread or existing approved async executor drives APIs; RNG is public-fixture ephemeral cryptographic OsRng, never wallet keys. Existing external target only, locked exact graph; added runtime/executor/rand direct dependencies must be reconciled with existing lock before resource review, not unlocked implicit selection.

Resolver implementation stores exactly reviewed local key-location, reads only explicit external PK/VK/tagged IR paths, checks frozen hashes, returns `Some(ProvingKeyMaterial{prover_key,verifier_key,ir_source})` for matching location and `None` otherwise. The `.zkir` is raw JSON; `ProofPreimage::prove` loads TAGGED IR, so first IrSource::load(raw), then official tagged_serialize(ir) into resolver material. CLI-generated PK/VK are already tagged. Never pass raw JSON as tagged resolver IR. Tagged decoded preimage/key cursors must be fully consumed; raw proof uses Proof(bytes).

Parameter provider must be synchronous/no-network and pinned to the already separately authorized verified k17 parameter file. Implement ParamsProverProvider::get_params(k): reject k!=17, read verified file and ParamsProver::read. Independent verifier uses public ParamsVerifier::read on that same serialized parameter file, not inaccessible as_verifier. Parameter acquisition/ceremony trust/setup remain separately gated and unperformed.

Call exact `preimage.prove::<IrSource>(OsRng, &params, &resolver).await?` once, preserve returned pi_skips and raw Proof.0. Native self-verification inside this call is real, but must not be the sole result verifier. Freeze proof SHA/length and separate native statement bytes before launching independent verify mode; no second proof for controls.

### Concrete statement extraction gap

The high-level API discards actual statement. IrSource::preprocess is private. It is incorrect to claim a ready independent verifier by inventing a flat statement or assuming no skipped branches. Two source-derived alternatives:

A. Preserve the required high-level call; reconstruct native statement using exact native Op objects and returned pi_skips, following ledger prove Noop insertion and verify public_inputs. Existing runtime public trace can be converted into native operations through matching onchain-runtime serde representation (or exact ledger ContractCallPrototype consumer). Start `[preimage.binding_input,preimage.communications_commitment.0]`; apply skip vector to complete Op sequence exactly; native Op.field_repr supplies widths. Assert skip/program iterators exhausted, not guessed byte widths. Use native Noop with n from skip, including trailing skippedblocks. The exact native Op type/import/serde consumer must be reviewed and checked against real source-check result before implementing this path; no public statement is presently emitted by current adapter.

B. Simpler source and stronger actual-statement retention: invoke public trait `Zkir::prove` on loaded IrSource/PK/preimage, which returns actual `(proof,pis,pi_skips)` at ir.rs103, and explicitly perform the SAME VerifierKey::verify as the high-level wrapper. This creates exactly one proof and avoids private preprocess or guessed statements. It changes the requested high-level API and needs explicit design vote; do not secretly substitute it. If selecting B, load PK with `IrSource::load_prover_key_from_tagged`, load VK with tagged_deserialize, pass ParamsProverProvider and preserve actual returned pis. This is the minimum reliable standalone consumer on inspected surfaces.

## Independent verifier and controls

Separate verifier process receives frozen VK, verified parameter file, raw proof and canonical tagged Fr statement vector. Strict tagged-deserialize statement/VK cursor EOF; never infer statement from mutable receipt accepted flags. Check manifest hashes and exact expected length; call `vk.verify(&params_verifier,&Proof(raw),pis.into_iter())`. Raw proof EOF is checked by pinned native verifier; still test appended raw byte and appended serialized container byte separately.

Preserve original good statement/proof unchanged. Re-run independent verification on good first, then mutate each public Fr in a separate copied vector by +1 (canonical field addition), remove/append instance field, flip proof byte, truncate half, append raw proof byte. All must refuse through actual decoding/verify errors, not host success booleans. Repeat original good after controls. Mutated instance length is a native relation-length control; wrong key file/hash and container trailingbytes may be host/decoder refusals and must be labelled separately. No second proof or setup.

Public message/signature/head/effect-transcript negatives remain actual source-check fixture controls; after full proof, a retained proof against changed bound public transcript/binding/communication must refuse. Do not claim direct public mutation of signature/message inputs if committed/private rather than exposed; identify actual public statement indices from native output. External effect map summaries, key_location and constructor identity are not direct IR public-proof predicates. Local proof of binding0 does not settle native escrow, validate caller transaction binding, authenticate chain head or enforce durable cross-instance replay.

## Compiler correspondence boundary

Current Compact kernel is a manually authored fixed specialization of one retained production beta/Core5 projection. It is not actual Moriarty compiler lowering or a generic proof/property/intent/transition/history correspondence theorem. The next proof consumer covers exact retained pay.zkir and runtime preimage only. Integrating the actual beta/Core5 producer as a supported compiler lowering/consumer, with independent correspondence checks, is a later scoped implementation step required before calling this a full language native pipeline. Production financial beta compilation and chain acceptance remain open regardless local proof success.

## Readiness and gate

Current API design is grounded but not fully implementable as requested without choosing statement path A or B. Adapter source-check should run first under its own reviewed scope, retaining actual skips. Then fresh Astra/Grok source/resource votes must bind exact consumer, locked dependencies/build allocation, verified SRS bytes/trust, keygen/proof limits and separate verifier controls. No hidden OnDemand parameter provider, fifth Compact compile, full upstream suite or native phase retry. Preview ledger8 remains incompatible with ledger9 source/runtime and requires separate route evidence; local proof does not close whole goal.
