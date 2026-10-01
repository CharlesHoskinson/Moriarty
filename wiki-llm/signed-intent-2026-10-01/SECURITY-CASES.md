# Signed intent /1 — security cases (S1)

Scope: signature-protocol evidence only. A valid signature proves possession of a key. It is not account authority,
snapshot membership, a valid transition, a native proof or ledger acceptance (`keyAuthority: Unverified`,
`ledger_accepted: false`, `authority_valid: null` in every result below).

## Layers and exact commands

| Layer | File | Command | Result |
|---|---|---|---|
| Independent oracle (pure Python, no Rust/TS encoder, no private key) | `experiments/midnight-crypto/reference-intent.py` | `python3 experiments/midnight-crypto/reference-intent.py check` | `match`, 20 golden, 433 corpus |
| Rust vs oracle, real native verifier | `experiments/midnight-crypto/tests/intent_reference.rs` | `cargo test --offline --locked --jobs 2 --test intent_reference` | 9 passed |
| Test-only throwaway-key signer | `experiments/midnight-crypto/examples/intent-fixture.rs` | built example `intent-fixture <scheme> <framing> < draft.json` | public `{statement,signatureHex}` only |
| Node real consumer | `packages/moriarty-beta/tests/auth-adversarial.test.mjs` | `MORIARTY_REQUIRE_NATIVE=1 MORIARTY_CRYPTO_BINARY=<built moriarty-midnight-crypto> node --test tests/auth-adversarial.test.mjs` | 7 passed |

The Node suite finds the helper next to the binary (`MORIARTY_INTENT_FIXTURE` overrides) and falls back to
`$CARGO_TARGET_DIR` or `experiments/midnight-crypto/target`. Without binaries it skips, unless `MORIARTY_REQUIRE_NATIVE=1`,
which makes a missing binary fail. The final receipt must be run with that variable set.

## Counts

- Hand-written fixtures: 5 (transfer with fee, transfer zero fee, repay interest-only, repay partial, repay full), each
  with hand-derived `expected_scope` and `expected_economics`. Golden vectors: 5 x 2 schemes x 2 framings = 20.
- Rust leaf matrix (retained, 868 leaves): 572 bound, 276 constrained-const, 20 derived-hash, 0 unclassified; 1724 native
  calls; raw mutation outcomes: 484 rejected by hash, 100 native `false`, 284 otherwise rejected; none verify.
- Node leaf matrix: the same walker semantics gives 868 leaves (8 x 44 + 12 x 43) over 20 real Rust-signed artifacts. Every
  one-leaf mutation is rejected and never reaches Core: 808 `BETA_SIGNATURE_SOURCE_MISMATCH`, the rest
  `BETA_SIGNATURE_SCHEMA` (scheme/framing/key shape) or `SignatureRejected` (key leaf that is still a valid point). The
  50/10 and 48/12 splits seen across runs come from the random throwaway key. Plus 20 signature-hex flips, all `SignatureRejected`.
- Node positive path: 20 artifacts, expected Debit/Credit/SetObligation effects and payer post balance compared with the
  hand-written economics; the Node-prepared statement equals the Rust helper's statement.
- Source rebinds (network, representation, symbol, scale, key, nonce, window, policy digest, agreement, domain id, asset
  id, signer account, whitespace/newline) over all fixtures: at least 220 attempts, each rejected with its manual code.
- Unsigned scenario: per artifact, round past the window, insufficient balance and consumed replay give `SignedCoreRejected`
  with a valid signature; a larger balance gives `SignedPreparedUnqualified` with a different post state. The scenario is
  not signed, so a signature is portable across scenarios and only Core decides.
- Hostile JSON (4 artifacts x 35 cases): duplicate keys, `__proto__`/`constructor`, extra/missing fields, non-object
  roots, BOM, trailing comma/garbage, uppercase/0x/short/long/numeric hex, number-typed strings, deep nesting, oversize.
- Unknown vs false: malformed input throws (unknown); a checked bad signature (r/s flips, other-fixture signature, swapped
  framing, other key, ECDSA high-S twin) returns `SignatureRejected` with `local: null`.

## Red tests retained (`/home/charl/research/moriarty-signed-intent-2026-10-01/`)

- `S1-root-reference-check.txt`: 7 passed, 2 failed, caused by the two defects below.
- `S1-node-red.txt`: missing required binary fails all 8 tests; a lying verifier shim (`false` rewritten to `true`) fails
  the unknown/false test and the leaf matrix.

## Defects found

1. Oracle: the Python BIP340 verifier challenged with the raw message. The native `k256` trait `Verifier::verify` (used by
   `midnight-base-crypto`) hashes the message with SHA-256 first and uses that digest as the BIP340 message. Fixed in the
   reference, not in production. For both schemes the raw/wallet framing bytes are the bytes before this internal hash.
2. ECDSA SEC1: native `k256` accepted a prefix-`05` alias of a compressed key; the corpus expected rejection. Root fixed
   `src/intent.rs` to require canonical prefix `02`/`03` before the native decode. Node surfaces this as
   `BETA_SIGNATURE_SCHEMA` (unknown, not false), and the suite asserts it.

## Limits and qualifications

- One-leaf mutations only, plus fixed hostile/rebind lists. Not exhaustive and no fuzzing. Multi-leaf mutations are not covered.
- The oracle verifier is pure Python (not constant-time) and holds no private keys. The helper uses throwaway in-memory
  keys, writes nothing and is not a production signing command.
- Node transport negatives (timeout, output bounds, invalid UTF-8, duplicate/unknown native fields) live in
  `auth-process.test.mjs` and are not repeated.
- `BETA_CRYPTO_RESPONSE` is returned for malformed artifact shapes (extra/missing top-level or signature fields), which
  is a misleading code for caller input. The suite locks current behaviour; a rename would be a wire-code change.
- Root subsequently made the auth/atomic test defaults portable and classified malformed artifact shape as BETA_SIGNATURE_SCHEMA (a caller judgment), reserving BETA_CRYPTO_RESPONSE for native protocol failures.
- Not established: key authority, snapshot membership, head extension, native proof or ledger acceptance. Root Astra/Grok
  audits are pending, and this file is not an approval.
