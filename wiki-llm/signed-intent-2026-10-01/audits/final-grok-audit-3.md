# Signed-intent candidate actual-result audit (Grok 4.6 high)

**Verdict: approve-scoped**

Scope of approval: closed `moriarty-signed-intent/1` native Rust signature codec; actual beta consumer (`prepareOwnerIntent` / `verifyAndPrepare` / CLI); local in-memory atomic stipulated registry store; ASCII human review; three public packed examples; bounded numerical KZG precursor `moriarty-transfer-numerical/1` at k10. This is a signature-protocol and local-stipulation result. Full financial settlement, authenticated owner/state, PCD, compiler correspondence, and Preview acceptance remain open.

This is a fresh whole-candidate source and actual-result review of `fd9725e0804c4db0866a3cf733ab300cf2c9179b`. It does not reuse an author vote, a prior Grok verdict, or any other reviewer's audit text. A previous unfinished Grok pass on an earlier tree confirmed a native-response classification blocker and was cancelled with no approval. The present candidate includes that repair. The blocker is independently re-checked here as repaired.

No nested agents. No repository source edits, commits, native-proof setup, SRS, Preview, network, or wallet action. External compiler/kernel prototype directories outside the candidate are out of scope.

---

## 1. Reviewer identity and terminal status

| Item | Value |
| --- | --- |
| Requested model | `grok-4.6` at high effort |
| Returned identity | Grok 4.6 released by xAI |
| Session identity | Grok 4.6 released by xAI |
| Requested effort | high |
| Returned effort | high, as requested. This session did not expose a separate numeric effort receipt. |
| Terminal status | completed. Audit file written. Independent cheap typecheck, focused Node tests, native intent-verify, and default-feature Rust intent tests exited 0. Existing full beta/Rust/k10 receipts read. No nested agents. No source edits. No proof rerun. |
| Checkout | `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` |
| Branch | `feat/signed-intent-runner-20261001` |
| Working tree | clean (`git status --porcelain` empty) |
| Helper binary | `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto` (existing target; not rebuilt for proving) |
| Node | v24.21.0 `/usr/local/bin/node` |

Guarded develop CLI `status --json` at start: capability `SP01.6 loan-swap-subset`; `pendingTransactions` empty; loan-swap implementation blocked on stale SP01 binding/candidate, missing current-accounting, unavailable resource live state. This review is independent of that blocked campaign.

---

## 2. Exact hash check

Manifest `wiki-llm/signed-intent-2026-10-01/candidate-manifest.json`:

| Item | Claimed | Actual this session |
| --- | --- | --- |
| Manifest SHA-256 | `73622f068a9ea6ff2af36c8f07f7ae962a90da1f11b67d77f342d2872d6ee89f` | `73622f068a9ea6ff2af36c8f07f7ae962a90da1f11b67d77f342d2872d6ee89f` |
| Manifest bytes | 252 files | 33664 bytes, 252 path entries |
| Candidate HEAD | `fd9725e0804c4db0866a3cf733ab300cf2c9179b` | `fd9725e0804c4db0866a3cf733ab300cf2c9179b` |
| Base | `2905cb6da0bf0ccfdca28b0f22478e0eda2420e3` | `2905cb6da0bf0ccfdca28b0f22478e0eda2420e3` (merge-base equals base) |
| Previous candidate recorded in manifest | `2dfcc57029af302d6bee3f0f3e3936828eac5b2c` | present in manifest; this review is of `fd9725e` |
| File hashes | 252 | 252 matched, 0 missing, 0 mismatched |
| Profile | `moriarty-signed-intent-candidate/1` | confirmed |
| Manifest scope | closed signed protocol/native consumer/local atomic precursor/DevEx/numerical proof; financial Preview gates open | confirmed |

Independently hashed implementation surfaces (all match the freeze):

| Path | SHA-256 |
| --- | --- |
| `experiments/midnight-crypto/src/intent.rs` | `8db3b6bddc6d6493e6d518d8cb46126efbe1ebc4dc5c782ce680a5b3fd84e6db` |
| `experiments/midnight-crypto/src/lib.rs` | `396f0e86dcd72a9e4dfd7684df872b88c9f4415cab42a96779c11e3622a7d041` |
| `experiments/midnight-crypto/src/main.rs` | `466a9b2fed4cbf7ec0c8611d1218b4ab6e15e513fbb6ad3a7e0422ceff4f521d` |
| `experiments/midnight-crypto/src/codec.rs` | `7ce67fb916efe31b9d8a647910bcc4a0511d2b5f9b9e0bda9c5fd375c4285ae2` |
| `experiments/midnight-crypto/src/financial_transfer.rs` | `f72e902b336582ded4b720efbfa362e30039d6f79c07d4b1165ef8317b692de4` |
| `experiments/midnight-crypto/Cargo.toml` | `15dedfcb94a3d8ffe8290e30e4c9858e200b9a10682de3db759f93e402204388` |
| `experiments/midnight-crypto/Cargo.lock` | `53731faef893a205c4c96933845d72253e3b55277b3bdefc6cf7c355d8b46bab` |
| `packages/moriarty-beta/src/auth.ts` | `bbddda1d5fe9e2148d29fb1983f0a842dfe52d2677402265f17acd72019153e0` |
| `packages/moriarty-beta/src/atomic.ts` | `93b666701a3388567fa43e4460c82d4b9dfc271f2d7ceaad605878030223dec4` |
| `packages/moriarty-beta/src/cli.ts` | `35c2c5bd145d201eeac918c8de3ffe196e449cc4cb5f950946621cb41e547b08` |
| `packages/moriarty-beta/src/json.ts` | `7147152fb45f2310cecc470da9b1f95d457162b4002c09bdea4d6bf8b262b93f` |
| `packages/moriarty-beta/src/intent-display.ts` | `8e935906b9a020edec7f684936db76b2d2b59f1e2bfea832ff8fc87dc9448b42` |
| `packages/moriarty-beta/src/index.ts` | `712ae51ad8b0af52a43100709c2d7c66666307a31a1507ea14cdc58b0e1900a0` |
| `packages/moriarty-beta/tests/auth-response-errors.test.mjs` | `d418f599ffbf7526a1645414586fef7acc29db1d9861de8a1daad66811079684` |
| `packages/moriarty-beta/tests/intent-display-error.test.mjs` | `cef119a9f598855087e4f3cec8be34618ce172e9750d3dda8a7b3121cd3c2cde` |
| `packages/moriarty-beta/tests/signed-distribution.test.mjs` | `21610a1eb1071b396ce2628c3f587cca2641f80155555a2a996597a4d636afc7` |

`native-run-receipt.json` pins for the k10 attempt match the same `financial_transfer.rs`, test file, `lib.rs`, `Cargo.toml`, `Cargo.lock`, and `fixtures/moriarty.json` hashes.

Git vs base: 128 paths differ from `2905cb6d`. The 252-file freeze is the review corpus. Manifest `excluded` records derived `node_modules`/`dist`/editor-server output, external Cargo target, and final audit receipts added after freeze.

---

## 3. What was independently inspected

Loaded `AGENTS.md` and checked-in `plugins/moriarty-dev/skills/develop/SKILL.md`, then guarded CLI status. Read the closed protocol, security cases, local-settlement contract, numerical result, Preview-route diagnosis, RESULT, DEVEX attribution, S1 plan (binary-envelope dissent), S3 implementation receipt (weekly-limit stop), native source/resource vote `evidence/native-grok-source.md` (resource conditions only), and the implementation/tests named above. Read the three public example `expected.json` files and `package.json` engines.

Did **not** read Astra audits, prior Grok audit markdown, or `wiki-llm/signed-intent-2026-10-01/audits/*` verdict prose. External `compiler-probe/` and `kernel-prototype/` trees are outside the candidate.

Cheap independent checks this session (`MORIARTY_REQUIRE_NATIVE=1`, `MORIARTY_CRYPTO_BINARY` the existing helper, `CARGO_TARGET_DIR` the existing research target, Cargo `--jobs 2`, `--offline --locked`, default features, no `native-proof` selection of the ignored k10 test):

| Check | Result |
| --- | --- |
| `packages/moriarty-beta` `npm run typecheck` | `tsc --noEmit` clean |
| Focused Node: `auth-response-errors`, `intent-display-error`, `json`, `auth-process` | 10 pass, 0 skip, 0 fail |
| Native `intent-verify` of `examples/signed-intent/transfer-schnorr-raw/signature.json` | exit 0, `status=IntentSignatureChecked`, `signature_valid=true`, `ledger_accepted=false`, `authority_valid=null`, `qualification=signature-protocol-only` |
| All-zero Schnorr `signatureHex` | native exit 2, `IntentRejected`, `signature_valid=null`, code `signature` |
| One-nibble flip of a valid Schnorr signature | native exit 1 (checked false) |
| `cargo test --test intent --test intent_reference` | intent 4 pass; intent_reference 9 pass including helper receipts and closed S1 corpus |

Existing frozen full receipts, independently read:

| Receipt | Observation |
| --- | --- |
| `wiki-llm/signed-intent-2026-10-01/evidence/final-beta-tests-3.txt` | `tests 193`, `pass 193`, `fail 0`, `skipped 0`; packed-install test present |
| `wiki-llm/signed-intent-2026-10-01/evidence/final-rust-tests.txt` | 32 passed, 1 ignored (`native_transfer_proof_bounded_k10`); includes prior tiny k6 native equality smoke plus privileged ledger/state tests |
| `wiki-llm/signed-intent-2026-10-01/evidence/final-typecheck-3.txt` | `tsc --noEmit` clean |
| `wiki-llm/signed-intent-2026-10-01/evidence/native-run.log` | `bytes=10924 k=10`; pairing and strict EOF checked |
| `wiki-llm/signed-intent-2026-10-01/evidence/native-run-receipt.json` | exit 0, 0.90 s, peak sampled group RSS 106070016 bytes, qualification local numerical proof only |

The ignored-test source itself mutates every public instance field (`for i in 0..instances.len()` over 28 fields), then first-byte corruption, half truncation, and one appended byte. The log records that those checks ran. This review did not re-execute the ignored native proof.

---

## 4. Implementation review

### 4.1 Native codec (trusted configuration; unknown vs checked-false)

`intent.rs` is the closed encoder. Build/frame/verify share one validator. Success output is the closed key set with `qualification: "signature-protocol-only"`, `authority_valid/snapshot_membership_valid/transition_valid: null`, `ledger_accepted: false`. Verify adds `signature_valid` boolean. `main.rs` intent commands: well-formed `signature_valid=false` exits 1; `IntentError` prints `IntentRejected` with `signature_valid: null` and exits 2. Independent native runs this session match that split (valid public Schnorr exit 0; all-zero malformed exit 2; nibble-flip exit 1).

Trusted configuration: beta spawn uses an absolute `binaryPath`, `shell: false`, fixed argv `[command]`, no PATH lookup, no runtime build. Bounds: stdin 65536, stdout 262144, stderr 8192, timeout 1..30000 ms (default 10000). Timeout settles on the bound even if a descendant keeps inherited pipes open (`auth.ts` comment and `auth-process` test).

Malformed native **response** JSON (duplicate keys, numbers, unpaired surrogate, depth, oversize string) is remapped in `native()` to `BETA_CRYPTO_RESPONSE`. CLI `main().catch` sets exit 2 for any `LocalError` whose code starts with `BETA_CRYPTO_`. Review mode prints `unable` and `signature validity is unknown, not false`. Caller artifact JSON (`parseBoundedJson` on the signature file) remains `BETA_JSON_*` / `BETA_SIGNATURE_SCHEMA` with CLI exit 1. Four red/green regressions in `auth-response-errors.test.mjs` lock both sides; this session re-ran them.

Native `IntentRejected` (native successfully classified the request as schema/key/signature malformed) is surfaced as `BETA_SIGNATURE_SCHEMA`. That is a reached schema judgment, distinct from a transport/protocol-response failure.

Old experimental `frame`/`verify` commands remain on the binary and still use the historical `Rejected` shape. Closed intent commands are a separate argv path.

### 4.2 Source identity, canonical framing, prehash

Owner terms are derived from the selected Source/6 AST: `actions.find` for the requested action, then `declarations.find(name === selected.intent)`. Domain/asset/keyRef come from that selected intent. Scenario text can change Core preparation; it does not change owner-signing terms. `sourceSha256` is SHA256 of the exact UTF-8 source bytes. `ownerProgramSha256` is `SHA256("moriarty-owner-program/1\0" || u32BE(len(P)) || canonical(P))` via Midnight `persistent_hash`. Frame F is `"moriarty-signed-intent/1\0" || u32BE(len(C)) || C`. Wallet framing prepends `midnight_signed_message:<len(F)>:`. Beta recomputes projection digest, frame tag/length, canonical payload, and signing-message hex against the native receipt and refuses mismatch as `BETA_CRYPTO_RESPONSE`.

Pinned Midnight `vk.verify(message, sig)` hashes the framed message with SHA256 before BIP340/ECDSA. Independent Python `reference-intent.py` applies `message = sha256(message)` before BIP340. SECURITY-CASES records the earlier oracle defect (raw BIP340 over unhashed F) as fixed in the reference. `intent_reference` 9/9 passed this session.

ECDSA compressed keys require SEC1 prefix `02` or `03` before native deserialize. Prefix `05` is a `key` rejection on the closed protocol. Hex is lowercase, exact widths, no `0x`.

### 4.3 Money ranges and crypto

Decimals are canonical unsigned strings. Money fields cap at `2^127-1`; scale at 18; general uint at `2^128-1`. Transfer: pairwise-distinct from/recipient/feeRecipient even at fee 0; amount > 0; amount+fee fits money; `grossCap >= amount+fee`, `feeCap >= fee`, `netFloor <= amount`. Repay: signer = payer; amount > 0; `grossCap >= amount`; `feeCap = netFloor = 0`. `notBefore <= notAfter`. JSON numbers are rejected (`expected string`). Unknown fields, duplicates, and empty policy arrays are closed.

### 4.4 Beta API types (Node 24)

`package.json` engines: `node: ">=24"`. Public types in `auth.ts` are explicit interfaces: `SignedIntentStatement`, `NativeIntentReceipt`, `NativeIntentVerificationReceipt`, `OwnerIntentPreparation`, `SignedLocalPreparation`. `Signing` and `CryptoConfig` are exported. `tsconfig.json` is `strict: true` with `skipLibCheck: true` (residual; see open items). Typecheck receipts and this session's `tsc --noEmit` are clean. `index.ts` re-exports the signed APIs and `LocalSettlementStore` without CLI/network side effects.

### 4.5 Authority, replay, race, CAS, revocation, failure atomicity

`LocalSettlementStore` is an in-memory single-writer stipulation. Qualification is always `local-stipulation`; `ledger_accepted: false`; `authority_valid: null`. Native verify runs outside the mutex. Commit is a promise-chain mutex shared with `revoke`. Commit re-reads, re-resolves the registry, compares the state digest, re-runs Core/5, validates the derived successor head, then copy-on-write assigns one new root. Precedence at commit: exact-repeat receipt, authority, digest, Core. A revocation that lands between prepare and commit reports `AUTH_REVOKED`. Other concurrent mutation reports `ATOMIC_STALE_STATE`. Stale `expectedStateDigest` is refused before spawn unless a matching replay receipt already exists (acknowledgement-loss retry). Fault hook can only abort; return values are ignored. Throws before the final assignment discard the draft (`ATOMIC_COMMIT_FAULT`). Caps: 32 in-flight, 4096 receipts, 64 bindings.

Registry codes: `AUTH_DOMAIN_MISMATCH`, `AUTH_NO_BINDING`, `AUTH_KEY_MISMATCH`, `AUTH_REVOKED`, `AUTH_NOT_YET_VALID`, `AUTH_EXPIRED`. Binding `scheme` is the union `'schnorr_bip340' | 'ecdsa_secp256k1_sha256'`. `prepareIntent` still narrows caller signing fields with TypeScript `as` after a closed `plain()` parse; `signingValue()` in `prepareOwnerIntent` validates at runtime.

Successor head is `SHA256("moriarty-local-successor/1\0" || ...)` over domain, financial effects without `AdvanceHead`, frame digest, and pre-head. `validateSuccessor` requires exactly one trailing `AdvanceHead` from pre-head to that derivation, equal to post-state head.

### 4.6 Exact effect post

Public examples ship independent Core/5 `expected.json` post-states. Transfer-schnorr-raw: Debit 1010 Owner, Credit 1000 Recipient, Credit 10 Fee, UseAllowance 1010, UseReplay, AdvanceHead `h0`→`h1`; Owner post 8990, Recipient 1000, Fee 10, work 9/1. Packed-distribution test compares actual CLI/Core effects against those files after `npm pack` outside the checkout with empty PATH and a copied actual binary. Frozen beta receipt includes that packed test.

### 4.7 ASCII human review

`escapeBody` / `escapeAscii` / `asciiJson` emit printable ASCII plus newline. `renderErrorReview` maps **every** supplementary line through `escapeAscii`. Unit test covers ESC, bidi, newline, tab. Real-native CLI test injects those characters as extra statement keys: `--review` stdout stays ASCII and does not contain the raw key; `--json` preserves the RFC6901 pointer. This session re-ran both.

CLI mismatch extras are pre-escaped before being passed into `renderErrorReview`, which escapes them again. Output remains ASCII. Finding F2.

### 4.8 Numerical precursor

Circuit version `moriarty-transfer-numerical/1`, 28 named public fields (balances, amount/fee/gross/caps/floor, round window, work and allowance counters, four effect amounts). Identities, signatures, hashes, replay, and head are outside the circuit (`fixture_public_values` checks identities in the host fixture). Range: 16-byte little-endian Fq prefix reconstruction; values ≥ 2^128 fail equality. Fields 7..=12 (amount, fee, gross, gross_cap, fee_cap, net_floor) enable `signed_start`, forcing bit 127 of the high byte to 0 (≤ 2^127−1). Slack witnesses cannot override gates. k ≤ 10 asserted before `ParamsKZG::unsafe_setup`. Pins: ledger `9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8`, proofs/curves `0ededef0e605701fc5139ebdcf011b11f3d86ba7`. Resource envelope from the retained native vote and receipt: wall 600 s, group CPU 600 s, RSS/address 8 GiB, jobs/rayon 2, ephemeral unsafe local SRS, one attempt. Receipt: 10924 bytes, k=10, exit 0, 0.90 s, peak sampled RSS 106070016. Sampling interval 0.1 s cannot report a continuous peak.

---

## 5. Numbered findings

Severity: High = protocol/safety hole in the claimed scope. Medium = incorrect classification or missing required check in the claimed scope. Low = residual diagnostic/docs/ergonomics that does not change unknown/false/success.

### F1 — Low — string-limit diagnostic text ignores `stringLimit`

`parseJsonWithinLimits` interpolates the configured `byteLimit` (`JSON exceeds ${byteLimit} bytes`). The string-length error is hardcoded `'JSON string exceeds 1024 bytes'` even when `stringLimit` is 131072 (native-response parser). Native `close` remaps any non-`BETA_CRYPTO_*` `LocalError` to `BETA_CRYPTO_RESPONSE` with `Invalid native JSON (${e.code})`, so the public code for a malformed native string is still `BETA_CRYPTO_RESPONSE` and CLI exit 2. The focused test uses a 131073-byte string and expects that code. Caller `parseBoundedJson` uses stringLimit 1024, where the message is accurate. Residual diagnostic text only.

### F2 — Low — CLI double-escapes mismatch supplementary lines

`cli.ts` builds mismatch extras with `escapeAscii(asciiJson(...))`, then `renderErrorReview` maps every extra through `escapeAscii` again. Backslashes become doubled in that path. Hostile-key review still matches printable ASCII and omits the raw injected key. Safe over-escape.

### F3 — Low — session wiki still records 187 beta tests

`wiki/sessions/signed-intent-2026-10-01.md` says RESULT records `187beta`. RESULT and `final-beta-tests-3.txt` record 193 pass / 0 skip. The session page is a stale index, not a production validator.

No High or Medium findings in the claimed scope.

---

## 6. Checklist (claimed scope)

| Requirement | Result |
| --- | --- |
| Native output trusted configuration only | Pass. Absolute `binaryPath`; no shell/PATH; closed native claims; foreign qualification/authority refused |
| Malformed/unknown vs checked false | Pass. Native exit 2/`signature_valid:null` vs exit 1/`false`; beta remaps malformed native JSON to `BETA_CRYPTO_RESPONSE` exit 2; caller JSON remains judgment exit 1 |
| Source identity / canonical framing / prehash | Pass. Selected-action draft; tagged SHA256; Midnight SHA256-before-BIP340/ECDSA; independent Python oracle |
| All money ranges + crypto | Pass. s127 money, u128 counters, scale 0..18, SEC1 02/03, lowercase hex, closed schemes/framings |
| API types Node 24 | Pass. engines `>=24`; explicit exported interfaces; typecheck clean |
| Authority / replay / race / CAS / revocation / failure atomic | Pass for local stipulation. Mutex CAS, digest, revocation window, copy-on-write, capacity/busy |
| Exact effect post | Pass. Hand-written Core/5 expected posts; packed-install comparison |
| Package install outside checkout | Pass. Frozen 193 includes packed test with copied binary and empty PATH |
| Constraints / state-version compatibility scope | Pass as open. Numerical circuit is a local precursor; proofs 0.8 / ledger 0.7.3 compatibility remains unestablished |

---

## 7. Preserved dissent and seat attribution

S1 (`claude-sonnet-5-5` high) recommended a fixed binary envelope `OwnerIntentEnvelope/1` (`plans/S1.md`). PROTOCOL records G1/G2 selected closed JSON and retains the S1 alternative. The implemented profile is JSON. That dissent remains on disk and is not treated as a defect of the chosen profile.

Sonnet S3 implementation receipt (`evidence/S3-implement-receipt.json`): `terminal_reason=api_error`, `api_error_status=429`, result `You've hit your weekly limit · resets Oct 4, 12pm (America/Denver)`. DEVEX-RESULT records that root and GPT-6.1 Sol G2 finished remaining CLI, docs, examples, and distribution. This audit does not attribute that completion to Sonnet. Unavailable or incomplete seats do not vote.

S1-root-reference-check.txt retains historical FAILED helper/corpus lines. Current cheap `intent_reference` 9/9 pass. The old file is stale evidence of the defects that were later repaired (oracle prehash, helper), not a live failure of `fd9725e`.

---

## 8. Remaining open (outside this approval)

These are scope limits, not reasons to withhold scoped approval:

1. Authenticated owner/key/account lifecycle, snapshot membership, and chain head. Local registry is an explicit stipulation.
2. Full mandatory financial/property/intent/transition/history proofs and atomic native ledger compare-and-consume.
3. Preview transfer/repayment, live wallet `signData`, and any network submission. No wallet, seed, or contract state was used here.
4. Compiler/runtime correspondence: installed Compact 0.31.1 rejects the proposed secp ECDSA symbol; numerical KZG public layout/VK/transcript is not the selected contract verifier. Proofs 0.8 / curves 0.3 vs ledger 0.7.3 / curves 0.2.1 is open.
5. Identities, signatures, hashes, replay, and head are outside the numerical circuit.
6. In-memory store only. Process restart loses receipts/registry.
7. Spawn inherits the caller environment. Timeout SIGKILLs the configured child, not an OS process group; descendants may remain after the bound (documented; settle-on-bound is tested).
8. `skipLibCheck: true`.
9. Old experimental `frame`/`verify` still accept historical encodings (including k256 prefix-05). Closed intent commands reject prefix-05.
10. Platform portability of a copied host binary is unestablished.
11. Multi-leaf mutations and fuzzing are not claimed. One-leaf plus fixed hostile/rebind lists.
12. No result named Accepted, Authenticated, or LedgerCommitted is in the protocol, RESULT, NATIVE-RESULT, LOCAL-SETTLEMENT, or PREVIEW-ROUTE for this stage.

---

## 9. Verdict

**approve-scoped** for candidate `fd9725e0804c4db0866a3cf733ab300cf2c9179b` against freeze `73622f068a9ea6ff2af36c8f07f7ae962a90da1f11b67d77f342d2872d6ee89f` (252/252).

The previous unfinished Grok review's confirmed blocker (malformed native parser `LocalError` classified as caller JSON judgment, CLI exit 1) is repaired: native-response parse failures are `BETA_CRYPTO_RESPONSE` / CLI exit 2 / unknown; caller artifact JSON remains `BETA_JSON_*` / exit 1. Four new regressions and this session's re-run support that split. Renderer extra-line escaping, hostile-key CLI, byte-limit diagnostic interpolation, and Binding scheme/framing unions are present in source and tests.

Three Low findings (string-limit message text, double-escape, stale 187 session line) do not change unknown/false/success or local atomicity.

This approval does not extend to Preview, authenticated owner state, PCD, full financial settlement, or compiler/proof-stack import.
