# Sign and verify an owner intent locally

The `intent` command prepares exact owner terms for external signing. The
`verify-intent` command checks the external signature with the native Rust binary,
then runs the existing local financial preparation. A successful result is
`SignedPreparedUnqualified`. Key authority, authenticated state, native financial
proof and Midnight ledger acceptance remain open.

## Install the package and verifier

Use Node 24 or later. In `packages/moriarty-beta`:

```sh
npm ci
npm run build
npm pack
```

Install the resulting archive into a separate project:

```sh
mkdir /tmp/mori-signed-demo
cd /tmp/mori-signed-demo
npm install /absolute/path/to/moriarty-lang-beta-0.1.0-beta.1.tgz
./node_modules/.bin/mori --help
```

This is a local archive install; no npm registry release is claimed. The package
contains its JavaScript consumer and examples. Install or copy a separately built
native verifier for your platform, and pass its absolute path explicitly. The CLI
never searches PATH for it, invokes Cargo, downloads it or starts a shell.

To build the verifier from a checkout, use Rust/Cargo and the pinned lockfile:

```sh
cargo build --locked --manifest-path experiments/midnight-crypto/Cargo.toml
```

Use `--offline` when the pinned Git and registry dependencies are already cached.
A missing cache prevents an offline build. Copy `target/debug/moriarty-midnight-crypto`
(or the file under your configured Cargo target directory) to a trusted absolute
path. Copying a binary requires compatible OS, architecture and runtime libraries.
The ephemeral `intent-fixture` example is a test tool and is not a production
signing interface.

## Verify a ready public example

The package includes three complete examples:

| Directory under `examples/signed-intent` | Operation | Scheme | Framing |
| --- | --- | --- | --- |
| `transfer-schnorr-raw` | Transfer with fee | BIP340 Schnorr | raw |
| `transfer-ecdsa-wallet` | Transfer with fee | secp256k1 ECDSA/SHA256 | midnight-sign-data |
| `repay-ecdsa-raw` | AccrualFirst partial repayment | secp256k1 ECDSA/SHA256 | raw |

Each contains exact source bytes, scenario, a public signature artifact and
complete independent effect/post expectations. The public keys are throwaway test
identities. They do not authorize the source's symbolic `Owner` or `Payer` account.
For the first example, from the installed project:

```sh
./node_modules/.bin/mori test node_modules/@moriarty-lang/beta/examples/signed-intent/transfer-schnorr-raw
./node_modules/.bin/mori verify-intent node_modules/@moriarty-lang/beta/examples/signed-intent/transfer-schnorr-raw/program.mori --action pay --scenario node_modules/@moriarty-lang/beta/examples/signed-intent/transfer-schnorr-raw/scenario.json --signature node_modules/@moriarty-lang/beta/examples/signed-intent/transfer-schnorr-raw/signature.json --crypto-binary /absolute/path/to/moriarty-midnight-crypto --review
```

For the second example use `transfer-ecdsa-wallet` and action `pay`; for repayment
use `repay-ecdsa-raw` and action `repay_loan`. Omit `--review` for JSON, or use
`--json` explicitly. `--review` and `--json` are mutually exclusive. Review text
and JSON output escape non-ASCII characters for reliable terminal inspection;
JSON decoding preserves the original Unicode values. Review shows integer atoms,
asset identity/representation/scale, symbolic signer/key reference, supplied key,
source hash, nonce, head, window, caps and the operation.

## Prepare your own external signature

Select a real public key using the exact raw encoding: Schnorr is 32 bytes,
ECDSA is a compressed 33-byte secp256k1 key. Supply lowercase hexadecimal without
`0x`. A source `key: "key1"` is an opaque reference, not a public key or account
ownership proof. The public key and framing choice are signed terms.

```sh
./node_modules/.bin/mori intent /absolute/path/program.mori --action pay --scenario /absolute/path/scenario.json --scheme schnorr_bip340 --public-key YOUR_64_LOWERCASE_HEX_CHARACTERS --framing raw --crypto-binary /absolute/path/to/moriarty-midnight-crypto --json > intent.json
```

`OwnerIntentPrepared` returns the full statement, `frame_hex`, `frame_sha256` and
`signing_message_hex`. It proves no signature possession. Inspect the statement
and a `--review` rendering before signing. Decode `signing_message_hex` into bytes;
these are the exact bytes the selected native signature API receives. Do not sign
the hexadecimal text or `frame_sha256`. For example, exporting those bytes:

```sh
node --input-type=module -e "import{readFileSync,writeFileSync}from'node:fs';const r=JSON.parse(readFileSync('intent.json','utf8'));writeFileSync('signing-message.bin',Buffer.from(r.signing_message_hex,'hex'));"
```

Both pinned native APIs apply SHA256 internally to these bytes. BIP340 signs that
32-byte SHA256 result using its BIP340 algorithm; ECDSA signs the SHA256 result
using secp256k1. A signer that explicitly accepts a prehash must use SHA256 of the
exact signing-message bytes, while a compatible ordinary-message API hashes once
internally. Avoid hashing twice. Live wallet compatibility is unperformed.

For `raw`, the message is the entire tagged canonical intent frame. For
`midnight-sign-data`, it is `midnight_signed_message:<frame byte length>:` followed
by the entire frame. That prefix is already present in `signing_message_hex`.
A wallet API that adds the same prefix must receive decoded `frame_hex` as its
input and produce exactly the selected signing message; do not prefix twice.
There is no alternate-framing fallback. Native verification is the check of
compatibility. A wallet signing unrelated bytes cannot satisfy this protocol.

Create `signature.json` with exactly the returned `statement` and a
`signatureHex` containing the external 64-byte raw signature as 128 lowercase hex
characters. DER ECDSA signatures need conversion to raw `r || s` outside this CLI;
use the native API's accepted low-S encoding. Keep private keys in your signing
system; no secret-key file is an input to Moriarty.

```json
{"statement":{"profile":"moriarty-signed-intent/1","...":"replace with the full returned statement"},"signatureHex":"replace with 128 lowercase hex characters"}
```

The illustration above is not a valid closed statement. Copy the full actual
object without adding `...`, changing fields or dropping null/empty fields.
Then verify with the same source/action and a local scenario:

```sh
./node_modules/.bin/mori verify-intent /absolute/path/program.mori --action pay --scenario /absolute/path/scenario.json --signature /absolute/path/signature.json --crypto-binary /absolute/path/to/moriarty-midnight-crypto --json
```

## Read the result and boundaries

| Result | Exit | Meaning |
| --- | --- | --- |
| `OwnerIntentPrepared` | 0 | Native signable bytes prepared; possession unchecked |
| `SignedPreparedUnqualified` | 0 | Native signature valid and local Core candidate prepared |
| `SignatureRejected` | 1 | Well-formed signature checked false; local preparation skipped |
| `SignedCoreRejected` | 1 | Signature valid; local financial/state/history judgment rejected |
| Formation/source/schema rejection | 1 | No successful signed preparation |
| Binary, timeout, transport/response failure | 2 | Signature validity unknown; no fallback acceptance |

The signed statement binds exact source SHA256, agreement/action, chain/network
claims, asset representation/scale/symbol, signer/key reference, signature metadata
and owner operation/bounds. The owner-program hash identifies an owner-term
projection, not compiled ZKIR or a compiler proof. Advertised source/policy labels
remain opaque claims. Editing comments or whitespace changes exact source SHA256
and requires a new signature. Formatting never rewrites commitments.

The scenario, balances, allowance/work counters, proposed effects, post-state and
repayment creditor are not owner-signed state. The creditor comes from the local
obligation cell. A changed scenario may preserve signature validity and yield a
new candidate or Core rejection. Local round-window and replay checks do not
establish current chain time or global replay state. A source network string
`preview` is a claim, not a Preview connection.

Quantities are exact decimal atom strings. Scale is 0..18; local S0 money fields
fit 2^127−1 and transport counters fit UInt128. Fees count toward gross debit and
net goals. Repayment pays accrued interest before principal. Unsupported financial
families remain SpecifiedOnly. No floating point, implicit conversion, full ACTUS
coverage or general native financial proof is provided by this signature flow.

All four required premises and unverified bindings remain in signed results.
The source account ID has no authenticated address/public-key mapping.
`keyAuthority` remains `Unverified`, state remains `LocalStipulationOnly`, native
proof is `NotChecked`, ledger is `NotSubmitted`, and `ledger_accepted` is false.
Signature verification does not establish snapshot membership, head extension,
atomic ledger compare-and-consume, proof-carrying financial acceptance or Preview
settlement. No production acceptance result is named Accepted.

Each example also has `expected.json` with exactly `{effects,post}`. These complete
expectations are derived from the independent vector economics and explicit
fixture state, then compared with the actual consumer output. They are not copied
from that output.
