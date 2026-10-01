# Independent native prover expectations

Status: specified only; independent source inspection, not execution approval or result review. Scope: expectations before authoring the next native financial prover consumer. Current delivery item remains native adapter source-check and resource review. No fetch, build, source-check, SRS load, key generation or proof was run for this report. No other auditor decisions were read.

## Provenance and boundaries

Repository observation: startup status in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` reported branch `feat/native-financial-kernel-20261001`, unresolved operational/admission/accounting evidence and no pending transactions. Installed Moriarty develop skill and repository AGENTS were read. Routing files absent from this worktree were read at the main `/home/charl/Moriarty` checkout. This report does not clear those stops.

Source facts below refer to these locally inspected official sources:

- **Z**: `/home/charl/.cargo/git/checkouts/midnight-zkir-d3b0dbbf065d6ece/e82d81d`, full HEAD `e82d81d25aabcc5f5092e2bd559083487f577914`, official repository `https://github.com/midnightntwrk/midnight-zkir`.
- **L**: `/home/charl/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/9a8777c`, full HEAD `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`, official repository `https://github.com/midnightntwrk/midnight-ledger`.
- **S**: `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5`.
- **P**: `/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.2`.
- Current adapter dependency evidence: `native-adapter-successor-v3/Cargo.toml` pins Z and patches ledger crypto/serialization/storage crates to L. Its `Cargo.lock:1618` resolves stdlib 2.3.5, checksum `01ec8ff46d47734090358ac4dbfb444f2bfeede7cecd792225fbd9333dcf7758`; lock also resolves proofs 0.8.2 and circuits 7.2.4. A future consumer must freeze its own resolved dependency identity; these facts do not establish its eventual build identity.

## Production API and public statement

Source fact: `L/transient-crypto/src/proofs.rs:745` implements `ProofPreimage::prove::<IrSource>(rng, params, resolver).await`. It resolves the exact `key_location`, fails if absent, loads tagged IR, VK and PK, calls `ir.prove`, then verifies with that VK using the **relation-returned** `pis` and parameters for the VK's k. It returns only `(Proof, pi_skips)`. The high-level wrapper discards `pis`; it cannot honestly be described as returning an exported public statement.

Source fact: `Z/zkir/src/ir.rs:73–103` implements public `Zkir::prove` for `IrSource`. It obtains parameters for the initialized PK k, preprocesses the supplied preimage, takes `preproc.pis` and `preproc.pi_skips`, calls actual `midnight_zk_stdlib::prove`, and returns `(Proof, Vec<Fr>, Vec<Option<usize>>)`. `check` only preprocesses; `prove_unchecked` is explicitly intended for testing and is unsuitable as the normal route.

Independent assessment of proposed option B: a consumer may explicitly use the public `Zkir::prove` route to retain its native-produced `(proof, pis, skips)` while reproducing the high-level wrapper's exact resolution/loading and VK self-verification. This avoids generating a second proof or guessing statement widths. It must be identified as this lower public production API, not falsely reported as a call to `ProofPreimage::prove`. Required invariant: same immutable resolver material, same preimage, same PK/IR, native proof, and explicit `VerifierKey::verify` with the same resolved VK and returned `pis`; any failure aborts output publication. This is a source-derived proposed implementation contract, not design or execution approval.

If the high-level wrapper is retained instead, exported statement reconstruction requires an explicit independently checked derivation. Do not label a host reconstruction “relation-returned.” Do not expose private witness data merely to recover pis.

Source fact: `Z/zkir/src/ir_vm.rs:230–267,589–620,862–899` puts binding input first, adds communication commitment only when the IR enables it, then appends Impact fields. Active Impact blocks consume and compare public transcript input fields; inactive Impact blocks append exactly `count` zeros and emit `Some(count)`. Active blocks emit `None`, which does **not** contain their width. Unconsumed transcript data and an incorrect communications commitment fail preprocessing. Private witness outputs and public call results are not a license to concatenate all input arrays into the statement.

Expected invariants:

- Serialize the native `pis` in exact order, canonical field encoding, explicit count and format version; preserve all zeros. Persist exact proof bytes and exact tagged VK bytes, with identities and public statement digest.
- Record skip entries and whether statement bytes are relation-produced or reconstructed. Never infer `None` widths from the skip list alone; use exact IR Impact widths and the ledger transcript representation if reconstructing.
- Check count and element equality between any ledger/host reconstruction and relation-produced pis. A hash comparison alone is insufficient to diagnose ordering or padding errors, though hashes are useful artifact identities.
- No host-generated boolean flags, MockProver result, `check` result or `prove_unchecked` substitutes for a real proof. Runtime logging must not expose preimages/private witnesses on failure.

## Independent verifier and failures

Expected implementation: a separate process receives only immutable proof, canonical public statement, expected VK and trusted verifier parameters/identity metadata. It must read those bytes anew, strictly decode the artifacts, and directly call `VerifierKey::verify(&params_verifier, &proof, pis.into_iter())`. It must not regenerate the witness, keygen, call the prover, or accept the producer's success flag. Verify the statement supplied by the caller against an independently selected expected VK/statement identity; do not trust an attacker-supplied manifest as its own authority.

Source fact: `L/transient-crypto/src/proofs.rs:554–569` maps `Fr` to native fields and calls `S/src/lib.rs:2017–2040` with `DummyRelation` and Blake2b transcript hashing. Stdlib checks public-input count before actual PLONK verification. `ParamsVerifier::read` is available (`proofs.rs:110–129`); the ledger uses static `PARAMS_VERIFIER` derived from published k14 material. Parameters derived from the approved k17 source can support standalone verification, but any ledger compatibility claim also needs the ledger's actual parameter route demonstrated.

All cases below are specified, not observed:

| Case | Required observable result |
| --- | --- |
| Untouched native proof, exact serialized pis, exact VK | Separate process returns verified success |
| One canonical field changed (including binding and commitment positions where present), order changed, count shortened/extended | Verification rejects; original proof is reused |
| Proof bit corruption in parsed content; malformed curve/scalar encoding; empty/truncated proof | Decoder or native verification rejects cleanly |
| One byte, several bytes, zero bytes or another proof appended to raw proof | Full native verifier rejects trailing data |
| Bytes appended outside tagged proof/statement/VK envelope | Strict artifact decoder rejects leftover bytes |
| Wrong VK/IR/PK/SRS identity, wrong key location, absent resolver entry | Explicit failure before reporting a valid expected proof |
| Missing local SRS or checksum mismatch | Failure without download, retry acquisition or generated setup fallback |

“Clean” means bounded failure/nonzero consumer exit with a precise stage, no panic-based pass, no success record, and no acceptance of mutation names as expected-error evidence. Negative tests alter actual artifact bytes and invoke the same verifier process. Restore originals between cases. Finite mutations are regression evidence, not a proof that every malformed byte string rejects.

### EOF finding and principled strictness

Source fact correcting an anticipated issue: the proofs 0.8.2 lower-level `plonk::prepare` is a parser/PCS preparation primitive and does not alone establish EOF. However, the **actual pinned full route** reaches `S/src/utils/plonk_api.rs:132–159`, where `BlstPLONK::verify` constructs a transcript, calls `prepare`, then calls `transcript.assert_empty()` **before** PCS verification. `P/src/transcript/mod.rs:140–150` implements this by comparing cursor position with total buffer length. The locally inspected stdlib 2.3.3 source also includes this check. Therefore “VerifierKey::verify necessarily ignores trailing proof bytes” is unsupported for these inspected versions. No runtime trailing-byte result was obtained here.

Required strictness criterion: successful verification must consume precisely the complete proof grammar for that VK and protocol, plus strict complete consumption of every outer artifact encoding. Use the existing full verifier route and demonstrate the suffix failures. If the eventual lock resolves a different implementation, re-inspect its complete call chain; if EOF is absent, use a reviewed verifier/parser integration that checks the actual consumed cursor after the same grammar (or a formally justified grammar-derived byte count for the exact VK/protocol). Comparing length to one generated “golden” proof, filtering only known suffixes, or asserting a proof digest from the same untrusted package is not a general parsing criterion. Do not silently patch a dependency or count a lower-level verification call as equivalent.

## Published k17 parameters and identity

Source fact: `L/base-crypto/src/data_provider.rs:167–170` pins `bls_midnight_2p17` SHA-256 to `4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74`. `get_local` hashes the entire file before returning it (`:280–307`); `FetchMode::Synchronous` makes `get_file` fail on a cache miss (`:488–505`). Explicit `fetch` remains possible, so merely selecting this mode is not a blanket network guarantee. The consumer must contain no fetch/HTTP fallback and run within the existing no-network policy.

The preserved `native-route-source/srs-k17-head.json` is a HEAD receipt for `https://srs.midnight.network/bls_midnight_2p17`, status 200, content length 25,166,212 bytes and multipart ETag. It does not establish downloaded content integrity. Require exact published SHA-256, receipt/source provenance and k identity before parameter use. `ParamsProver::read` uses `RawBytesUnchecked` (`L/transient-crypto/src/proofs.rs:91–98`), so untrusted arbitrary bytes must not reach it before integrity validation. Read verified immutable bytes or avoid time-of-check/time-of-use substitution.

Qualification: matching the official digest establishes identity with the pinned published setup, not an independent setup ceremony audit or proof that toxic waste is absent. No fresh deterministic/local setup may replace published parameters while claiming ledger-compatible evidence. Record the trust assumption explicitly. A k17 limit is a resource bound, not a prediction that this financial circuit fits.

Expected resolver controls: allowlist the exact key location; bind it to immutable IR, tagged PK and VK hashes; freeze compiler/kernel artifact and dependency identities; fail mismatched formats, trailing serialized data, wrong k or unknown location. Hash identity alone does not establish that unrelated keys match the intended relation. Preserve actual keygen provenance and require real same-VK verification, then independently compare the VK against the intended contract operation. A resolver which hands both arbitrary keys to the producer and verifier does not establish intended-circuit identity. CSPRNG use, bounded wall time/memory/output and original failure evidence remain required under the existing resource decision, which this report does not amend.

## Actual ledger consumer and binding-zero limitation

Source facts: `L/ledger/src/construct.rs:515–574` constructs aligned input and private transcript encodings, public operation field representations and Popeq results, and initializes binding input to zero as a **placeholder**. `L/ledger/src/prove.rs:250–385` calls provider check, inserts Noop padding for skipped impacts, adjusts transcript costs and supplies `Some(intermediate_call.binding_input(binding_commitment))` to `ProvingProvider::prove`. A provider that ignores this override cannot claim the ledger path.

`L/ledger/src/verify.rs:1932–2012` obtains the verification statement from the finalized call: binding input, communication commitment, guaranteed transcript operation fields, then fallible transcript operation fields. The binding hash includes contract address, entry point, transcript effects/costs, guaranteed instruction count and parent binding commitment. `L/ledger/src/structure.rs:435–506` can compile to unconditional success when `proof-verifying` is disabled; when enabled, V3 selects the operation's v3 VK and calls native verification with static parameters, except the explicitly configured mock mode. Require actual feature/consumer-path evidence, not dependency presence.

Expected scope label for a local binding-zero run: a standalone proof of this exact relation and serialized host statement, subject to its tested predicate. It does not establish finalized ledger-call binding, contract operation VK registration, transaction acceptance, private handoff, recursive PCD, complete financial semantics or Preview settlement. To extend that claim later, honor the ledger overwrite, compare finalized ledger public inputs, select real verification mode with `proof-verifying`, and obtain actual ledger evidence. No hidden MockProver, calibrated mock verification, disabled verifier feature or placeholder provider may enter the claimed path.

## Expected output before any result claim

The future consumer should produce an immutable public artifact bundle containing proof bytes, tagged VK, exact canonical pis, skips, artifact/IR/key/dependency/SRS identities, route and parameter trust labels, and bounded execution metadata. Producer and independent verifier outputs must identify the same bytes. Result records must distinguish preparation/source-check, actual native proving, same-process native verification, independent process verification, negative cases, and any separately demonstrated ledger acceptance. Unrun cases stay specified-only. This report neither authorizes their dispatch nor concludes the wider goal.
