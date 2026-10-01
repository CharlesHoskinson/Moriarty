# Independent Grok 4.6 high exact-candidate audit

Signed-intent runner candidate, 2026-10-01.

## Auditor identity and terminal status

| Field | Value |
| --- | --- |
| Requested | exact `grok-4.6` high |
| Returned session identity | Grok 4.6 (host presentation: "Grok 4.6 released by xAI") |
| Effort | high |
| Nested agents | none |
| Other-reviewer verdicts read | none |
| Author/source-vote reuse | none |
| Terminal status | completed; this file is the only write |
| Written at | 2026-10-01T04:09:17Z |

This is a full exact-candidate source and actual-result audit. It is not a diff-only review and not an ingestion of another auditor's verdict.

Startup applied: Moriarty `AGENTS.md`, checked-in `plugins/moriarty-dev/skills/develop/SKILL.md`, and `cli.py --repo . status --json`. Develop status at audit time: capability `SP01.6 loan-swap-subset`; `pendingTransactions` empty; loan-swap implementation remains blocked on stale SP01 admission inputs. That block does not cover this signed-intent review.

## Candidate identity

| Field | Observed |
| --- | --- |
| Cwd | `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` |
| Branch | `feat/signed-intent-runner-20261001` |
| HEAD | `12358d3738b7e094ed0914cb7b423c46dba0005b` |
| Base | `2905cb6da0bf0ccfdca28b0f22478e0eda2420e3` |
| Merge-base vs stated base | identical to stated base |
| Worktree | clean (0 porcelain lines) |
| Tip subject | Implement native signed intent consumer and local atomic precursor |
| Manifest | `wiki-llm/signed-intent-2026-10-01/candidate-manifest.json` |
| Manifest SHA-256 | `4a23dcb7307a92ecba724e2d63be41ab55cf236040116c576e80295400869d58` |
| Manifest bytes | 30598 |
| Declared files | 231 |
| Files present and SHA-matched | 231 / 231 |
| Missing | 0 |
| Mismatch | 0 |
| Profile | `moriarty-signed-intent-candidate/1` |
| Declared scope | closed signed protocol / native consumer / local atomic precursor / DevEx / numerical proof; financial Preview gates open |

Exact SHA check was computed over the live tree against the frozen manifest map. Every declared path hashed to the declared digest.

Independent hashes of the principal implementation files in this tree:

| SHA-256 | Path |
| --- | --- |
| `8db3b6bddc6d6493e6d518d8cb46126efbe1ebc4dc5c782ce680a5b3fd84e6db` | `experiments/midnight-crypto/src/intent.rs` |
| `f72e902b336582ded4b720efbfa362e30039d6f79c07d4b1165ef8317b692de4` | `experiments/midnight-crypto/src/financial_transfer.rs` |
| `90394ae6cb35b3b11653ba0d981c0f283e131ba30ccbbca2d72792872f935e7b` | `packages/moriarty-beta/src/auth.ts` |
| `c719045a7d374cd87e8f29ad5b82648ec76ab73c7270e2b955d4c9265d9db537` | `packages/moriarty-beta/src/atomic.ts` |

Numerical-proof pin set in `evidence/native-run-receipt.json` matches the current files for `src/financial_transfer.rs`, `tests/financial_transfer.rs`, `src/lib.rs`, `Cargo.toml`, `Cargo.lock`, and `fixtures/moriarty.json`.

## Method

Reviewed the live implementation, public API, CLI, pack/examples, protocol docs, tests, and numerical receipts. Cheap reruns used the existing external target and the stated helper:

- binary `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto` (SHA-256 `0b5fdf85c342f85a39364127f9e47c69bc890f679489a27ebc2a00a101e39c0d`, 2832552 bytes)
- helper `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/examples/intent-fixture`
- `MORIARTY_REQUIRE_NATIVE=1`
- `CARGO_TARGET_DIR=/home/charl/research/moriarty-crypto-2026-09-30/target`
- Cargo `--jobs 2`
- host Node `v24.21.0`, rustc `1.98.1`

Not performed, per instruction: source edits, commits, native proof setup, ignored k10 rerun, SRS generation, Preview, network, wallet.

`--all-features` was not selected. That suite would rerun the existing tiny k6 smoke.

## Source review

### Native Rust signature codec

`experiments/midnight-crypto/src/intent.rs` plus `codec.rs`, `lib.rs`, `main.rs`.

Closed JSON profile `moriarty-signed-intent/1`. Duplicate keys, JSON numbers, unknown fields, and noncanonical decimals reject before any `signature_valid` boolean. Exact object key sets are enforced at every layer. TEXT is nonempty Unicode scalars, ≤1024 UTF-8 bytes, no C0/DEL. IDs match `[A-Za-z][A-Za-z0-9_]{0,63}`. Rounds use UInt128. Money uses ≤2^127−1. Scale ≤18.

Projection P is the closed owner-term subset. `ownerProgramSha256` is SHA-256 of `moriarty-owner-program/1\0 || u32BE(len(P)) || canonical(P)`. Signing frame F is `moriarty-signed-intent/1\0 || u32BE(len(C)) || canonical(C)`, cap 16384. Raw signing message is F. Wallet framing is `midnight_signed_message:<len(F)>:` || F. Framing is signed metadata. This session recomputed `SHA256(F) == frame_sha256` on the three packed artifacts.

BIP340 and ECDSA go through the pinned Midnight `k256` APIs. Those APIs hash the framed message with SHA-256 internally. ECDSA keys require canonical SEC1 prefix `02` or `03` before native deserialize. Malformed keys/signatures return `IntentRejected` with `signature_valid: null` and process exit 2. Well-formed wrong signatures return `IntentSignatureChecked` with `signature_valid: false` and exit 1. Success is exit 0 with `qualification: "signature-protocol-only"` and `authority_valid`, `snapshot_membership_valid`, `transition_valid` all JSON null, `ledger_accepted: false`.

Old experimental `frame`/`verify` commands remain on the same binary and keep domain `moriarty-midnight-auth-experiment/1`.

No production signing command exists. `examples/intent-fixture.rs` is test-only: in-memory OsRng keys, public `{statement,signatureHex}` only, secrets never printed.

### Beta consumer

`packages/moriarty-beta/src/auth.ts`, CLI in `src/cli.ts`, types exported from `src/index.ts`.

`prepareOwnerIntent` / `verifyAndPrepare` derive owner terms from the selected Source/6 AST plus the actually referenced beta domain/asset entities (chain, network, representation, scale, symbol). `keyRef` comes from the selected source intent. The entire artifact statement must equal that derivation, including SHA-256 of the exact source bytes. Scenario changes may alter Core preparation only.

Trusted executable path is absolute deployment configuration. Spawn is `spawn(binaryPath, [command], {shell:false})`. No PATH lookup, no shell, no runtime build. Bounds: stdin 65536, stdout 262144, stderr 8192, timeout 1..30000 ms default 10000. Native responses are one UTF-8 JSON line, no BOM, closed key set, `qualification` must be `signature-protocol-only`, authority fields must stay null. Hex strings use a wider internal parser; ordinary JSON stays at 1024.

Transport/protocol failure throws `BETA_CRYPTO_*` and leaves signature validity unknown. Native `IntentRejected` becomes `BETA_SIGNATURE_SCHEMA` (caller input). Checked-false is `SignatureRejected` with `local: null`. Valid signature plus Core rejection is `SignedCoreRejected` with `signature_valid: true`. Local success is `SignedPreparedUnqualified`. No result is named Accepted, Authenticated, or LedgerCommitted.

Public TypeScript types name `SignedIntentStatement`, frame bytes, and `signature_valid: boolean`. Package `engines.node` is `>=24`. This host ran Node v24.21.0. Packed declaration client in `tests/distribution.test.mjs` compiles those fields under `tsc --strict --module NodeNext --target ES2023`. Package typecheck (`tsc --noEmit`) exited 0 in this session.

### Local atomic stipulated registry

`packages/moriarty-beta/src/atomic.ts`.

In-memory single-writer store. Scenario is rebuilt from store state. Native verify runs outside the mutex. Commit is copy-on-write under a promise-chain mutex shared with `revoke`. Commit order: repeat receipt, authority re-resolve, digest CAS, Core re-prepare, derived successor head, single root swap. Faults before swap discard the draft. Inflight cap 32, receipt cap 4096.

Replay key is `["domain","signer","nonce"]`. Same frame is `AlreadyCommitted`. Different statement with the same nonce is stale or Core replay, never a second consume. Revocation racing an in-flight settle is `AUTH_REVOKED`. Stale `expectedStateDigest` is refused before native spawn unless a receipt already exists (ack-loss retry).

Every result carries `qualification: "local-stipulation"`, `ledger_accepted: false`, `authority_valid: null`. Registry is an explicit local stipulation. `revoke` has no authenticated revoker.

### ASCII human review

`packages/moriarty-beta/src/intent-display.ts`.

`--review` prints every signed leaf, claimed versus computed source SHA-256, recomputed frame SHA-256, atoms with asset id/scale, and an explicit not-established list. Default remains JSON (`asciiJson`). `--review` and `--json` are mutually exclusive. Control, bidi, and non-ASCII scalars are escaped. CLI holds no keys.

### Three public packed examples

Under `packages/moriarty-beta/examples/signed-intent/`:

| Example | Operation | Scheme | Framing | signature.json SHA-256 |
| --- | --- | --- | --- | --- |
| `transfer-schnorr-raw` | Transfer 1000+10 | BIP340 | raw | `c959c5f0fff7489edd1787e68fb452b9a335ee6b1164ad3329fc4e151e3e3b12` |
| `transfer-ecdsa-wallet` | Transfer 1000+10 | ECDSA | midnight-sign-data | `116c238a8b18638dc282bc8efb323926ee440fe506aa2eaff0a37d50bf02969f` |
| `repay-ecdsa-raw` | Repay 3000 accrual-first | ECDSA | raw | `a8c52b3b6a8357a75ab22e35cc8f199cc5a7a6e6d99f9c339ec67291b97d0e8c` |

Independent native `intent-verify` on all three returned exit 0, `signature_valid: true`, `qualification: signature-protocol-only`, `ledger_accepted: false`. Raw signing message equals the frame. Wallet message is the documented prefix plus frame. Complete effect/post expectations in `expected.json` match the vector economics (transfer debit 1010 / owner 8990; repay 3000 accrual-first). These examples use the unsigned scenario `post_head`; they are verify-intent consumers, not `LocalSettlementStore` commits.

### Bounded numerical KZG precursor

`experiments/midnight-crypto/src/financial_transfer.rs` and `tests/financial_transfer.rs`.

28 public numerical fields, 8 slacks, UInt128 bit-range, signed-127 on amount/fee/gross/caps/floor, amount ≥ 1 via slack. Identities, signatures, hashes, replay, and head are outside the circuit. Host fixture adapter checks ordered three-account Core/5 transfer layout before numbers enter the circuit. The live proving test is `#[ignore]` with reason `native setup/proving held for independently reviewed resource proposal`. Default all-features therefore keeps it ignored.

`Cargo.lock` contains both `midnight-proofs 0.8.0` / `midnight-curves 0.3.0` (git `0ededef`, native-proof feature) and `midnight-proofs 0.7.3` / `midnight-curves 0.2.1` (pulled by ledger pin `9f9842eb`). Compatibility of 0.8 proof artifacts with the 0.7.3 ledger verifier remains open. That gap is documented in the candidate and is outside this precursor's acceptance.

## Numbered findings

### F1 — Severity: nit

File: `packages/moriarty-beta/src/json.ts` (`parseJsonWithinLimits`)

The byte-limit check uses the caller-supplied `byteLimit`, including 262144 for native responses, while the error string is always `JSON exceeds 65536 bytes`. Enforcement is the numeric limit. The message is wrong for the native-response parser.

### F2 — Severity: nit

File: `packages/moriarty-beta/src/atomic.ts` (`prepareIntent`)

The call into `prepareOwnerIntent` type-asserts `scheme` as `'schnorr_bip340'` and `framing` as `'raw'`. Runtime still passes the caller values. `midnight-sign-data` settlement is tested. The assertion is a TypeScript convenience only.

### F3 — Severity: documented coverage limit

File: `packages/moriarty-beta/tests/atomic.test.mjs`

The local store's successful-signature path is exercised with ephemeral ECDSA. Schnorr appears as a key-mismatch case. `verifyAndPrepare` accepts both schemes. This is recorded in `LOCAL-SETTLEMENT.md`. It does not weaken the codec.

No blocker, high, or medium source defects were found in the closed signed protocol, the beta consumer, the local store, the review/CLI surface, the packed examples, or the numerical precursor as scoped.

## Actual-result evidence

### Frozen receipts (candidate files)

Beta (`evidence/final-beta-tests.txt`):

```
ℹ tests 187
ℹ pass 187
ℹ fail 0
ℹ skipped 0
```

`MORIARTY_REQUIRE_NATIVE=1` is required by the adversarial/pack tests. The receipt includes packed install outside checkout with empty PATH, copied verifier, `SIGNED-INTENT.md` in the tarball, typed API client, and negative cases.

Rust all-features (`evidence/final-rust-tests.txt`), counted from the per-harness summaries:

| Harness | Pass | Ignored |
| --- | --- | --- |
| lib/main unit | 0 | 0 |
| authority | 1 | 0 |
| cli | 2 | 0 |
| financial_transfer | 7 | 1 (`native_transfer_proof_bounded_k10`) |
| intent | 4 | 0 |
| intent_reference | 9 | 0 |
| ledger | 1 | 0 |
| native_proof (k6 smoke) | 1 | 0 |
| signatures | 6 | 0 |
| state | 1 | 0 |
| **Total** | **32** | **1** |

The ignored test is the new numerical proof. The k6 smoke reran and passed.

Numerical (`evidence/native-run.log`, `evidence/native-run-receipt.json`):

- command: `cargo test --offline --locked --features native-proof --test financial_transfer native_transfer_proof_bounded_k10 -- --ignored --exact --nocapture`
- log: `Numerical precursor real KZG proof bytes=10924 k=10; pairing and strict EOF checked`
- test source asserts all 28 public-input +1 mutations, first-byte corruption, half truncation, and one appended byte
- receipt `exit_code` 0, elapsed 0.90 s, sampled peak RSS 106070016, Cargo/Rayon 2, wall 600 s, 8 GiB address-space cap
- qualification in receipt: local numerical proof only

Typecheck (`evidence/final-typecheck.txt`): `tsc --noEmit` succeeded.

### Independent reruns this session

| Check | Result |
| --- | --- |
| Manifest SHA and 231 file digests | match |
| Numerical pin set vs current files | match |
| `packages/moriarty-beta` `tsc --noEmit` | exit 0 |
| Native verify of 3 packed artifacts | exit 0, `signature_valid: true` |
| Uppercase / `0x` / all-zero Schnorr signature | exit 2, `signature_valid: null`, code `signature` |
| Single-nibble flip of a valid Schnorr signature | exit 1, `signature_valid: false` |
| Extra field / JSON number / duplicate key | exit 2, unknown validity |
| ECDSA SEC1 prefix `05` | exit 2, code `key` |
| amount+fee overflow past 2^127−1 | exit 2, code `range`, `gross overflow` |
| `grossCap` = 2^127 | exit 2, `integer out of range` |
| zero-fee transfer with recipient = feeRecipient | exit 2, alias rejected |
| repay `feeCap` = 1 | exit 2, `invalid repayment bounds` |
| swapped framing on a valid Schnorr artifact | exit 1, `signature_valid: false` |
| `cargo test --jobs 2 --test intent --test intent_reference` (no native-proof feature) | 13 passed, 0 failed, 0 ignored |
| Node `auth.test.mjs` + `auth-process.test.mjs` + `auth-adversarial.test.mjs` with `MORIARTY_REQUIRE_NATIVE=1` | 20 passed, 0 skipped |
| Node `atomic.test.mjs` with the same binary | 68 passed, 0 skipped |

Adversarial leaf split this session: 868 leaves, 808 `BETA_SIGNATURE_SOURCE_MISMATCH`, 47 `BETA_SIGNATURE_SCHEMA`, 13 `SignatureRejected`. The frozen receipt recorded 51/9. That split moves with the throwaway key; `SECURITY-CASES.md` already records it. No mutated leaf produced a prepared result.

Python oracle corpus in the independent Rust rerun: 433 entries across the documented families, including unicode (NFC/NFD/astral/U+2028), overflow, numeric keys, and scalar-text. Golden vectors: 20.

This session did not rerun npm pack or the ignored k10 proof.

## Required checks

| Check | Disposition |
| --- | --- |
| Native output trusted configuration only | Absolute `binaryPath`, `shell:false`, no PATH lookup. Relative path `cargo` throws `BETA_CRYPTO_BINARY_PATH`. Missing binary throws `BETA_CRYPTO_BINARY_UNAVAILABLE`. |
| Malformed/unknown vs checked false | Native exit 2 + `signature_valid: null` for malformed; exit 1 + boolean false for well-formed wrong; beta throws versus `SignatureRejected`. Independently confirmed. |
| Source identity / canonical framing / prehash | Source SHA-256 of exact bytes; owner projection domain-separated from the signing frame; raw vs wallet prefix; SHA-256(F) matches `frame_sha256`; signing a digest or hex text is a native-false control in `intent_reference.rs`. |
| Money ranges + crypto | 2^127−1 money, UInt128 rounds, scale 0..18, amount > 0, pairwise distinct transfer endpoints including zero fee, repay feeCap=netFloor=0. Schnorr 32 / compressed ECDSA 33 / 64-byte signatures, lowercase hex, SEC1 `02`/`03`. |
| API types Node 24 | `engines.node >=24`; host Node v24.21.0; explicit statement/frame/`signature_valid` types; typecheck exit 0. `@types/node` in the lockfile is 26.6.3 (DefinitelyTyped numbering). |
| Authority / replay / race / CAS / revocation / failure atomic | Independently reran 68 atomic tests, including R1/R1b/R2/R2b/R3/R7/R11–R14 and all write-step faults. |
| Exact effect/post | Hand economics in vectors, packed `expected.json`, and atomic fixtures. Transfer 1000+10 → debit 1010, owner 8990. |
| Package install outside checkout | Frozen 187 includes the packed-install test with empty PATH and copied verifier. Test source matches that contract. This session verified the three artifacts natively and did not rerun `npm pack`. |
| Constraints / state-version compatibility | Numerical precursor uses proofs 0.8 / curves 0.3. Ledger pin still carries proofs 0.7.3 / curves 0.2.1. Compatibility is open. `state`/`ledger` tests remain privileged local checks with their original limits. |

## Preserved dissent and seat attribution

S1 binary dissent is preserved. `PROTOCOL.md` records that G1/G2 selected strict closed JSON and that S1 recommended a fixed binary envelope. `plans/S1.md` still specifies `OwnerIntentEnvelope/1` as that alternative. The implemented profile is JSON. The dissent remains on disk and is not treated as a defect of the chosen profile.

Sonnet weekly-limit incomplete seat: `evidence/S3-implement-receipt.json` records seat `S3`, requested model `claude-sonnet-5-5`, effort high, `api_error_status` 429, `process_exit_code` 1, result `You've hit your weekly limit · resets Oct 4, 12pm (America/Denver)`. Display, tests, and walkthrough completion after that stop are attributed to root and G2 in `DEVEX-RESULT.md`. This audit does not attribute that completion to Sonnet.

S1 first implementation: `evidence/S1-implement-receipt.json` `terminal_reason: max_turns`. A later finish receipt completed the S1 implementation scope. Original failing red tests remain in evidence.

## Open product gates

These remain open. This candidate does not close them.

- Authenticated account key lifecycle and owner binding
- Authentic ledger state, snapshot membership, and head
- Compiler / native complete financial proof correspondence
- Full mandatory financial, property, intent, transition, and history proofs
- Atomic native financial ledger acceptance
- Preview transfer and repayment finality
- Live wallet `signData` compatibility
- Compact secp ECDSA symbol on the installed compiler
- proofs 0.8 artifact verification on the ledger 0.7.3 verifier
- Persistence / crash recovery of the local store
- Self-certifying account identity versus the explicit local registry

The local key registry and the k10 numerical circuit are precursors with the exclusions recorded in `NATIVE-RESULT.md` and `LOCAL-SETTLEMENT.md`. No wallet identity, seed, or existing contract state was used to manufacture evidence. No Preview financial transaction is shown.

## Verdict

**approve-scoped**

Approve the frozen candidate `12358d3738b7e094ed0914cb7b423c46dba0005b` for the declared closed scope: versioned native Rust signature codec, actual beta consumer, local atomic stipulated registry, ASCII human review, three public packed examples, and the bounded numerical KZG precursor with its recorded exclusions.

Do not treat this as financial, authenticated-owner-state, PCD, or Preview acceptance.

Nits F1–F2 are non-blocking. F3 is documented coverage. Open gates above stay open.
