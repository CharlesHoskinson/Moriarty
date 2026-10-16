# K10 compiled-C2 input adapter (source-only, R2)

## What this is

A DATA-only Python (stdlib) adapter that prepares the exact inputs for executing the **already compiled** APPLIED B9 `c2.c2.mint` minting validator — the real Aiken wrapper whose `Issue` branch calls `claim.issue` — under the pinned Aiken `uplc eval --cbor` interface, with the genuine public Groth16 VK/proof payload and coherent tampered contexts. It replaces the K9 locally chosen `a2a2..` test policy with the compiled artifact's policy everywhere, so the full wrapper (singleton mint/output expects, holder check, token-name guard, VK-digest guard, and the pairing checked last) is what would run. R2 changes only the identity basis (below), the execution-proposal discipline, and documentation; the seven contexts, envelope encoding, token-name derivation and expectations are unchanged from R1.

Nothing here executes a validator. Execution needs a fresh non-author full source/effect audit that AGREEs, ordinary data generation, a fresh actual-input review, and a separate new exact bounded native AUTH (see `EXECUTION-PROPOSAL.json`).

## Files

| File | Role |
|---|---|
| `tools/build_k10_inputs.py` | The adapter: pinned-input verification, independent script-hash recomputation, envelope CBOR/token-name derivation, PlutusV3 ScriptContext term generation, deterministic manifest. |
| `EXPECTATIONS.json` | Source-frozen expectations for the seven tests, the context shape, and the compiled-artifact identity with its hash basis. |
| `EXECUTION-PROPOSAL.json` | The bounded ordinary data step, the exact future native command shape, parser, budgets discipline and isolation; grants no authority. |
| `DESIGN-POSITIONS.json` | The source author's explicit positions (FOR) on every consequential clause, including the new hash basis and authority separation. |

## Generation (ordinary data step, later)

```
python3 tools/build_k10_inputs.py <b9-applied.json> <k8_fixture.ak> \
  BASE/current-cardano-k2-k10-public-inputs-generated-20261016
```

Outputs: `script.hex` (verbatim `c2.c2.mint` compiledCode), `contexts/K10-T*.uplc.txt` (one `(con data (...))` PlutusV3 ScriptContext term each), and `manifest.json`. Inputs are SHA-256-pinned and re-verified after generation; a non-empty output directory is refused. This step is stdlib-only, one attempt, zero retries, with the budgets in `EXECUTION-PROPOSAL.json`; no native AUTH is needed for the pure data step, and no hash match is claimed until root actually runs it and records a public receipt (`generated=false` until then).

## Identity: independently recomputed, tool-declared only as cross-check

The policy `6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1` is now **independently recomputed** by the adapter from the pinned compiledCode; the `hash` field in `b9-applied.json` (echoed in `hash-c2.txt`) is used only as a cross-check, and generation fails closed if the two disagree. The generated manifest records `scriptHashClaimedHex`, `scriptHashIndependentlyRecomputedHex`, `policyIdHex` (the recomputed value) and the classification.

Source basis (exact Pallas 0.35.1 crate sources, checksum-matched per `SOURCE-MANIFEST.json`; snippets are audit citations, not runtime dependencies):

```rust
// pallas-traverse-0.35.1/src/hashes.rs:85-89
impl<const VERSION: usize> ComputeHash<28> for alonzo::PlutusScript<VERSION> {
    fn compute_hash(&self) -> Hash<28> {
        Hasher::<224>::hash_tagged(&self.0, VERSION as u8)
    }
}
// pallas-crypto-0.35.1/src/hash/hasher.rs:73-78 (Hasher<224> = Blake2b::new(28), line 59)
pub fn hash_tagged(bytes: &[u8], tag: u8) -> Hash<28> {
    hasher.input(&[tag]); hasher.input(bytes); hasher.finalize()
}
```

Aiken's side closes the loop: `compiled_code_and_hash` builds `conway::PlutusScript::<3>(program.to_cbor())` (`ast.rs:169-174`) and `Serialize` writes `compiledCode` as the hex of that same script byte slice together with its hash (`ast.rs:179-187`). Therefore, for PlutusV3:

```
recomputed = blake2b-224( 0x03 || compiledCode bytes )     # hashlib.blake2b(..., digest_size=28)
```

Before hashing, the adapter strictly decodes the compiledCode as exactly one definite CBOR byte string (`590acf` = 2767 payload bytes; shortest-form length, exact consumption, no trailing bytes, indefinite forms rejected). That decode is a structural guard only: the inner flat payload is **not** hashed, and the hash adds no second CBOR layer — neither the inner-flat preimage nor a double-CBOR preimage is correct for `hash_tagged(&self.0, 3)` as constructed above.

Audit source pins (SHA-256): pallas-traverse `hashes.rs` `eecfb1f4…6ab`, pallas-crypto `hasher.rs` `80fb9044…37e`, pallas-primitives `lib.rs` `a7c70f25…601`, pallas-primitives `plutus_data.rs` `adfebe4d…24b`, pallas-codec `utils.rs` `7019c76d…98f`, aiken `ast.rs` `d3d9009b…b27`, aiken `eval.rs` `2d63c16d…725` (full pins in `EXPECTATIONS.json` and in the generated manifest).

## Envelope CBOR cross-check

The frozen EnvV2 preimage was cross-checked against the actual Pallas `PlutusData` encoding (`pallas-primitives/src/plutus_data.rs`): `Constr` encodes its CBOR tag then its fields, and `BoundedBytes` is a single definite byte string up to 64 bytes and an indefinite 64-byte-chunk encoding above that. Every frozen envelope field is at most 32 bytes, so the definite path applies and the R1 frozen segment assembly agrees byte-for-byte; no material discrepancy was found and the encoder is unchanged (fields over 64 bytes still fail closed, unreachable here).

## Limits

The context is locally fabricated (K9 `2222../555..` fixtures, builder-chosen unbound 2030 expiry, zero fee, empty inputs) — semantically well-formed for the wrapper, but not a ledger transaction and no basis for fee, conservation, admission, wallet, signing or Preprod claims; it must never be presented as a live Preprod claim. The single-party Class-0 setup could forge proofs and the VK-matrix link is unproved; all broader obligations (20 UTxO families, ACTUS/DeFi and general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, real Preprod) remain open.
