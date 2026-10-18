# mori-k2-driver (UNADOPTED, source only, never built or run)

A general source-to-Groth16 path for admitted `moriarty/1` programs. It compiles source with the
actual R1 compiler (`mori_r1cs::compile_source`, which runs Core's parser and validator), selects
the `SettlementHashV2` adapter, and then sets up, proves, verifies and exports the artefacts the
Cardano C2/S2 consumers read. Nothing here has been compiled, tested or executed.

## Layout

The crate expects this execution layout (the same relative layout R1 used):

```
<root>/moriarty   Core70 copy (Cargo.toml, crates/mori-core, mori-lexer, mori-diagnostics, mori-span, rust-toolchain.toml)
<root>/r1cs       the reviewed R1 candidate rust/ directory (source/mori-r1cs, fixtures, expectations, EXPECTED-PACKET*)
<root>/k2         this crate
```

The R1 compiler must be the reviewed bounded-XOR revision. With the frozen R1 `blake2b.rs`, every
SettlementHashV2 synthesis is predicted to exceed 8 GiB (review finding E-01).

## Commands

```
mori-k2 compile --source FILE --selector FILE
mori-k2 setup   --source FILE --selector FILE --out-record FILE --out-proving-key FILE
mori-k2 prove   --source FILE --request FILE --setup-record FILE --proving-key FILE --out-proof FILE
mori-k2 verify  --setup-record FILE --proof FILE [--source FILE]
```

- `compile` and `verify` write no file and draw no randomness. `verify` never generates a key.
- `setup` and `prove` draw only from `rand_core::OsRng`. Outputs are created with `create_new`
  and never overwritten. The proving key gets mode 0600.
- Exit status 0 means success, 1 a refusal, 2 a usage error. There is no retry, fallback or
  chaining between commands, and no network code.

Example inputs are in `requests/`. They are verbatim transcriptions of the R1 fixtures P02
(invoice `partial`) and P04 (reserve `allocate`); `tests/codec_tier.rs` a08 checks the
transcription.

## Inputs

- Source: UTF-8, at most 256 KiB, any profile Core routes. `moriarty/2` is refused by the
  compiler at lowering (`StagedNotLowered`).
- Selector `moriarty-k2-relation-selector/1`: `language` `moriarty/1`, `semantics`
  `moriarty-core/1`, `adapter` `SettlementHashV2`, `cap` `NotBound`, `accounting`
  `GrossDebitAndFees`, `frame` `TouchedOnly`, an `action`, and optional
  `expectedSourceSha256` / `expectedProgramHash` pins. Any other value is refused.
- Request `moriarty-k2-prove-request/1`: a selector, a `transition` proposal
  (`{"kind":"CoreReference","scenario":…}` or
  `{"kind":"Records","inputs":…,"pre":…,"post":…,"effects":…}`), and the ABI v2 `context`
  premises. The VK digest is derived from the VK; a request may only restate it.

Every JSON document goes through Core's strict parser (duplicate keys, trailing data, depth 32,
65,536 nodes, 1 MiB) and then `deny_unknown_fields` types.

## Outputs

- Setup record `moriarty-k2-groth16-setup/1`: public. It holds the relation descriptor (every
  identity of the compiled relation), the 488-byte VK and its BLAKE2b-256, and the proving-key
  SHA-256 and size. Trust profile `single-party-test-setup/1`.
- Proving key: arkworks compressed `ProvingKey<Bls12_381>`. Custody material, not a public export.
- Proof export `moriarty-k2-groth16-proof/1`: public. It holds the statement bytes and digest,
  `x1`/`x2`, the 192-byte proof, the decoded context, the Cardano `EnvV2` values, the catalogue
  action entry digests, the domain `/2` intent target hash and the declared anchor.

C2 `Issue { vk, proof }` takes `verifyingKeyHex` from the record and `proofHex` from the export.
`EnvV2` needs an `expiry` from the transaction builder. The driver produces no Plutus Data/CBOR.

## Tests

| Target | Tier | Needs |
|---|---|---|
| `codec_tier` | A, pure | nothing beyond an ordinary build |
| `compile_tier` | B, compile and synthesis, no keys | the reviewed bounded-XOR compiler and the R1 resource envelope |
| `crypto_tier` | C, setup/prove/verify | `--features crypto-authority -- --ignored --test-threads=1` and separate crypto authority |

Expected outcomes, grades and lineage are in the task's `EXPECTATIONS.json`. No ordinary test
establishes arbitrary-witness soundness, setup honesty or ledger acceptance.

## What a passing proof means

Under setup honesty and Groth16 knowledge soundness, a valid proof shows that the statement's
constant header (tag, family, predicate digest, ABI v2 shape digest, body length) and its body
satisfy the compiled `moriarty-core/1` relation. Context bytes are bound but not authenticated.
That this VK came from the named matrices is declared by the setup record. No verifier can check
it. The result is one layer of the claim path. It is not ledger admission, settlement or Preprod.
