# Native owner intent Task 1 report

Scope: strict envelope, authorization types and host FENS verification.
Task base `3ab8a691a1a81d1be7d38d7589991816b031df8e`. Author for the completion:
Claude Opus 5.5 (`claude-opus-5-5`), continuing the interrupted author's
uncommitted bytes without restarting them. Independent check pending:
GPT-6.1 Sol HIGH. Task 1 is a component gate only. Delivery stays
incomplete until Tasks 2–4 and their audits are done.

## Qualification (strict, reference-only)

These checks run ordinary host/reference code only. They are not native constraints, MockProver,
SRS, keygen, proving or verification, and none of those ran. A passing host
FENS check shows only that the supplied attachment satisfies the response
equation under the caller's reconstructed message. It does not establish
account-label authority, ledger ownership, deployed identity, history or time.
The JSON/SHA work is host reconstruction and is not computed natively. The
independent QA fixtures are off-circuit affine checks. They are not native
evidence or financial authorization. No signer, custody path or wallet code
was added.

## Changes

- `crates/mori-backend/src/authorization.rs` (new):
  - `INTENT_PROFILE`, `PREPARED_PROFILE`, `MAX_SIGNED_ASSETS = 16` and `MAX_ENVELOPE_BYTES = 65_536`.
  - `SignedPolicy`, `ExpectedAuthority`, `UnsignedPayload`, `SignedIntent`, `AuthorizationType{UInt,Native}`, `AuthorizationBinding`, `FinancialValue`, `AuthorizationValue` and `PreparedIntent`, with the spec's exact field sets.
  - Validated structs reject unknown fields and run `validate()` on direct and nested serde decoding as well.
  - The public `from_json` entry points run Core's bounded duplicate-key parser before schema decoding.
  - Labels are 1..1024 UTF-8 bytes with no C0/C1 controls.
  - Digests and field widths are strict.
  - Limits must be strictly ascending by full `AssetId`, unique, at most 16 and use the matching beneficiary.
  - The payload and the wrapper canonical forms are each capped at 65,536 bytes, and the prepared report at 1 MiB.
  - `predecessor_hash` keeps the `["moriarty-predecessor-label/1", label]` framing.
- `crates/mori-backend/src/schnorr.rs` (new), a minimal strict FENS-JubJub host verifier:
  - Exact lowercase affine32LE input. Field integers must be below p and are never reduced.
  - Points are checked for curve membership, prime-order subgroup and non-identity, with no cofactor clearing.
  - The response must be below r and is not reduced.
  - Challenge: `transient_hash([FENS, Rx, Ry, PKx, PKy, m])` reduced wide into the Jubjub scalar field.
  - Response equation: `s·G == R + c·PK`.
- `crates/mori-backend/src/lib.rs`: `pub mod authorization; pub mod schnorr;` only.
- `crates/mori-backend/Cargo.toml` and `Cargo.lock`: adds a direct `midnight-curves = "=0.3.1"` edge to the existing locked package. No new resolution.
- `crates/mori-backend/third-party/jubjub-fens/`: `LICENSE-MIT`, `LICENSE-APACHE` and `NOTICE.md`, with Sig Network attribution and frozen input hashes.
- `crates/mori-backend/tests/authorization.rs` (new), 18 host tests:
  - Golden payload bytes and hash, plus golden signature acceptance and one-bit refusal.
  - The second QA vector (key13/nonce19, message 1) and the alternative golden attachment (nonce23).
  - Wrong challenge tag and order.
  - Identity, off-curve, pure-torsion and mixed key/R points.
  - Responses `0`, `1` and `r-1` decode but do not verify. `r`, `r+1` and `s+r` refuse.
  - Values `p` and `p-1`, malformed hex, decimals and labels.
  - Omitted, extra, duplicate, reordered and 17 limits.
  - The 65,536 and 65,537 canonical wrapper boundary.
  - Exact schemas for the descriptor, policy, authority, wrapper and prepared report.
  - Direct serde decoding and manual edits still go through semantic validation.

This session's repair: the last failing test (`prepared_value`) decoded golden
decimal metadata with `serde_json::from_value::<Fr>`. That uses `Fr`'s binary
serde form and failed with "failed to fill whole buffer". It now uses a
test-only `canonical_decimal_field`:
- digits only, with no sign and no leading zero;
- exact 256-bit accumulation that refuses on overflow;
- `Fr::from_le_bytes`, so a value at or above p refuses and is never reduced.

A new test, `golden_decimal_metadata_matches_canonical_field_bytes`, checks the
conversion against the golden data's own hex values (`message_decimal` →
`message_hex_le32`, and `public_key.x/y` → the `public_key_hex` halves), and
checks that p refuses. Production source bytes did not change in this session.
The test closure was hand-formatted because rustfmt skips over-width macro lines.

## API handoff to Task 2

- `authorization::{INTENT_PROFILE, PREPARED_PROFILE, MAX_SIGNED_ASSETS, MAX_ENVELOPE_BYTES}`
- `UnsignedPayload::{from_json, validate, hash, predecessor_hash, validate_assets}`
- `ExpectedAuthority::{from_json, validate}`
- `SignedIntent::{from_json, validate}`
- `PreparedIntent::{from_json, validate}`
- `SignedPolicy::validate`
- `AuthorizationType`, `AuthorizationBinding`, `FinancialValue` and `AuthorizationValue`
- `validate_label`, `validate_asset`, `decode_digest`, `digest_limbs`, and the re-exported `AssetLimit`
- `schnorr::{FENS_DOMAIN, decode_key, decode_signature, SignatureWitness{r,response}, verify_attachment, decode_native_hex, native_hex}`

`UnsignedPayload::validate_assets(&[AssetId])` is the accepted trusted-source
set comparison:
- It refuses more than 16 inputs first, then invalid labels, then aliases and duplicates.
- It then compares exact sorted full AssetIds against the intrinsically sorted limits.
- Candidate limits are never sorted.

Intrinsic validation cannot establish that the source set is complete. Task 2
must call this with the actual trusted source assets. There is intentionally no
intrinsic `asset.network == payload.network` check. Both are bound to the
actual trusted source in Task 2.

## Captures

All captures are under
`/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/native-owner-task1/`.
The earlier ones are preserved unchanged:

| Capture | Result |
| --- | --- |
| `red.log`, `red-expanded.log` | Red: E0432, the specified `authorization`/`schnorr` API was missing |
| `red-network-contract.log` | Red: 11/13 (asset-set and wrapper limit) |
| `green-attempt1.log` | lib 6/6; host 12/13 (wrapper sizing) |
| `green-attempt2.log` | host 16/17 (`prepared_value` decimal decoding) |
| `green-opus-attempt3.log` | After the decimal repair: lib 6/6, host 18/18 |
| `fmt-diff-opus-attempt1.log` | Red: rustfmt diff in the new helper |
| `fmt-diff-opus-attempt2.log` | rustfmt `--check` exit 0; `git diff --check` exit 0; no trailing whitespace in the new files |
| `cargo-fmt-opus.log` | `cargo fmt -p mori-backend --check` exit 0 |
| `green-opus-final.log` | Final bytes: lib 6/6, host 18/18 |
| `digests-opus.log` | Fixture, license and notice digest confirmation |

Test command, with exit 0 on the final bytes:
`RUSTUP_TOOLCHAIN=1.99.0 CARGO_TARGET_DIR=/home/charl/Moriarty/.worktrees/readiness-20261010/target cargo test -p mori-backend --locked --offline --lib --test authorization`

Digest confirmation:
- The spec matches `a4a01236…b142` and the golden vector matches `9c035c8e…85f0`. Neither has changed since the task base.
- LICENSE-MIT `2230c498…2af1` and LICENSE-APACHE `cfc7749b…3d30` match the NOTICE table.
- All five NOTICE hashes appear in `jubjub-frozen-inputs-v2/manifest.json` (`3037c1a2…c76c`).
- The QA fixture SHA256s match the test comments: second vector `cf6cc127…64bd` and alternative attachment `fefeaab6…cb00`.
- `SECOND_KEY`, `SECOND_SIG` and `ALT` are byte-present in those fixtures.

## Remaining concerns

- `validate_assets` completeness depends on the caller. It is safe only when Task 2 passes the actual trusted source set.
- The 65,536 wrapper boundary test uses the second-vector signature as filler. It checks canonical size admission, not signature validity for that payload.
- `PreparedIntent::validate` checks that the report is internally consistent: its profile, `payload_hash` equals the computed hash, `context_hash` equals `relation_hash`, NativeHex values are canonical and the size is within 1 MiB. It does not recompute the context or projections from source. That belongs to trusted preparation in Task 2.
- The NOTICE records that the upstream signature files were untracked at the observed HEAD. Provenance rests on the frozen runtime manifest, not a git pin.
- The full workspace test suite, clippy, and native, compiler and CLI integration were not run, as the scope required.
