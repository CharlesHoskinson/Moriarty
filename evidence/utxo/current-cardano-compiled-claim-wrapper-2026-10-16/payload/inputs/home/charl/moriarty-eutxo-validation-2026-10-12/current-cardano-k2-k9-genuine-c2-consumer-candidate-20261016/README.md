# K9 genuine C2 consumer (source-only)

Status: SOURCE ONLY. Uncompiled, unexecuted, no test result, no fresh approval. Compilation is UNOBSERVED; do not read any statement here as "the syntax works". This module was authored under the explicit user Gemini/Muse fallback after the original author timed out; the author supplies no independent approval and is not an independent reviewer. A fresh non-author FULL source/effect review is required before materialising any native harness, compiling or running anything.

## What this slice is

The next real consumer after the K8 direct-pairing tests: the UNCHANGED production function `pvc2/claim.issue` (lib/pvc2/claim.ak) called with the genuine public VK/proof and a typed Cardano `Transaction`, with honest expected positive and negative rejections. There is no duplicated or stubbed acceptance logic and no diagnostic helper: the test module contains only direct `claim.issue(...)` calls.

Files:

- `lib/pvc2/k9_consumer_builder.ak` — pure builder returning `(policy, env, tx)` for a future compiled-context adapter. It builds the transaction from the unchanged stdlib testing API `cardano/transaction.placeholder` (transaction.ak lines 207-235: typed placeholder with empty fields), overriding only `outputs` and `mint` by record update. The single claim output uses the typed script address (`cardano/address.from_script`, i.e. `Script(policy_id)` payment credential), an inline envelope datum and no reference script; values are built through the opaque stdlib `Value` API (`assets.from_asset`). No `test_ctx` CBOR decoding, opaque downcast or hermetic wallet/ledger fixture is used or introduced.
- `lib/pvc2/k9_genuine_c2_consumer_test.ak` — seven tests: one positive, three crypto negatives (N1/N2/N3) and three non-crypto guard negatives. Crypto negatives use explicit `!claim.issue(...)` Bool assertions; no test uses a `fail` annotation, because every transaction built here satisfies the issuance `expect` patterns, so each rejection is a Bool `False` from an `and` conjunct rather than an evaluation failure.
- `EXPECTATIONS.json` — expectations fixed before any execution: exact test names, fail flags and the first expected guard for each test.
- `DESIGN-POSITIONS.json` — the author's FOR positions on the consequential clauses, with source justification. The author is the single source author, not a reviewer; these positions grant no authority.

Exact unchanged source closure for a later reviewed runner (proposal only): `lib/pvc2/claim.ak`, `lib/pvc2/core.ak`, `lib/pvc2/catalogue.ak` (imported by core), `lib/pvc2/types.ak`, `lib/pvc2/bytes.ak`, `lib/pvc2/groth16.ak`, plus the already-adopted generated `lib/pvc2/k8_fixture.ak` (5269 bytes, sha256 1b9e2a9a...) and the two new modules above. Dependencies: `aiken-lang/stdlib v2.2.0` (aiken.toml), compiler `v1.1.24`, Plutus v3. Production sources are unchanged; no new language features are needed for the local builder unless a future compiler run observes otherwise (none has run).

## Genuine public inputs (no new VK/proof)

- VK: 488 bytes from the K7 public setup record (sha256 86b588fe...), digest (blake2b-256) `d19d99a4...` (86-record d19 digest); proof: 192 bytes from the K6 actual Rust proof (file sha256 `930198...`), adopted via the K8 fixture (5269 bytes, sha256 `1b9e2a9a...`, actually adopted).
- Positive statement digest `c9ef8ea0...`; N1 body-rederived digest `123f0bad...`; N2 context-rederived digest `621c562a...`; N3 is the positive proof with A and C G1 slices swapped. All come from the frozen public receipts; nothing was generated here.
- Envelope fields (predicate digest, VK digest, claimant VKH `5555..` (28 bytes), settle outref txid `2222..` index 0) are the actual public record values from the proof receipt's `cardano.envV2`. These, and the `a2a2..` settle hash, remain local fixture caller inputs: they are not real Preprod data, not chain balances and not claim-ledger truth.

## Two explicit choices (not facts about the chain or the proof)

- Policy id: the builder's `chosen_policy_id` is the 28-byte settle hash `a2a2..a2` from the actual public export, chosen explicitly as the local policy id for calling the library function in tests. It must NOT be labelled the actual compiled C2 hash. The genuine APPLIED B9 wrapper / hashing step is a separate later slice; a future compiled-context adapter passes the real applied policy id into `build_for_env`.
- Expiry: the Rust export intentionally omits `expiry`. The builder chooses the explicit constant `1893456000000` (2030-01-01, POSIX ms) and documents it in code. Expiry is builder-chosen and unbound: no field of the proved statement binds it (see `core.statement`), `claim.issue` never inspects it, and it is not a ledger assertion.

## Why N1/N2 test the pairing, not the token-name guard

For N1 and N2 the FULL coherent tampered `EnvV2` is built first (only `statement_digest` replaced), and the token name, mint and single output are then recomputed FROM THAT envelope by `build_for_env`. Issuance therefore stays well-formed: singleton mint pair, single claim output, `Script(holder) == policy`, inline datum, correct envelope token name, well-formed envelope and correct VK digest all pass, so the first possible refusal is the pairing equation inside `groth16.verify_digest`. For N3 the positive envelope/transaction are kept unchanged and only the proof's A/C slices are swapped.

## What this does NOT claim

- C2 alone authenticates nothing beyond "a Groth16 proof verifies for this statement digest under this VK" (claim.ak header). Predicate/env binding beyond C2 — recomputing the statement from authenticated facts, the authenticated catalogue, and signatures — is S2's job and is not pretended here. Outref, claimant and predicate are not authenticated by C2 alone, and nothing here is ledger truth.
- No real VI signature, no compiled C2 wrapper, no ExUnit result, no Preprod claim. The user preference for actual Preprod stands: the prototype eventually needs a real current context proof and a qualified transaction; this slice is the actual contract-function consumer only.
- K8 status is unchanged by this slice: K8 source plus its ordinary fixture are actually closed, but the native Aiken pairing remains unrun under a separate runner audit. K9 source preparation neither requires a K8 pass nor reuses any K8 authority; no direct-pairing K8 AUTH exists. Native authority is zero: no setup, proving key, key, wallet, signing, network or ledger action is authorised or performed.
- Trust profile is single-party-test-setup/1: whoever controlled the setup can forge proofs for this relation. Single-party setup controller forgery, fabricated caller context, the unproved VK matrix and unmeasured transitive CPU are all retained limitations.
- All 20 UTxO families, ACTUS 277/18/32, DS01-07, DeFi 72, DA01-24, general financial duties, real recursion, private handoff, bounded multi-parent PCD, universal certificates and real Preprod remain open. No percentage of full compatibility is claimed.

## First blocked boundary

Fresh non-author FULL source/effect review of these five files; then a new bounded native consumer fixture plus compile/run under a separately reviewed runner, with a fresh actual audit. The actual compiled C2 wrapper / genuine applied hash, FX1 and real Preprod are separate later boundaries.
