# Native financial kernel iteration

This iteration follows signed-intent integration merged to `main` in PR13, commit `033e9a90465d4c06b787dedb5010e4e18715e030`. It tests the next missing predicate: whether the exact financial runtime invocation is accepted by the matching native ZKIR relation and can subsequently produce a verified native proof.

## Current evidence

| Predicate | Status | Evidence and limit |
|---|---|---|
| Production beta verifies actual Midnight signatures | Locally implemented, published | Previous signed-intent dossier / PR13. |
| Financial kernel SHA256/ECDSA, low S, state/head, nonce and full transfer/fee effects | Local source and runtime reviewed | External kernel prototype, two scoped independent audits. Fixed public fixture, manually funded escrow; constructor and instance trust unresolved. |
| Actual official runtime proof-data serialization | Locally observed | Current actual production handoff produced byte-identical frozen native preimage; no proof statement fabricated. |
| Native source check | Locally observed, scoped result reviewed | [Result](evidence/ADAPTER-V3-RESULT.json): actual native relation accepted good and refused five forged preimages. No proof. |
| Production signed Beta to native ledger preparation | Locally observed, narrow result reviewed | [Actual v5 result](V5-ACTUAL-PREPARATION.md): handoff and actual Rust preparation succeeded; complete preimage and eight contexts matched, actual IR preprocessing passed. No keys/proof/application. |
| Full financial native proof | Open | Exact IR row model k17/114250 rows. No SRS body, key generation or proof performed. Model size is not proof success. |
| Actual ledger financial acceptance | Open | Current keyless preimage has provisional binding0. Finalized ledger binding, actual proof, strict validation/application, fees and authenticated custody remain required. |
| Preview transfer and repayment | Incompatible matching published formats, open capability | Live Preview reports ledger8.1.2; matching published release supports older proof/operation/IR encodings. Conditional on live release correspondence; no live build attestation or observed candidate rejection. No transaction submitted. |

## Design choices

Use the official runtime converter and pinned Rust relation implementation. Preserve the actual public transcript, communication commitment and skip layout. Distinguish host identity checks, deserialization errors and relation refusals in results. A boolean or numerical helper is insufficient evidence for signed financial acceptance.

Preserve old failed allocations and immutable receipts. The successor lock removes unused packages while preserving every retained version/source/checksum. A missing archive must be fetched under an explicit bounded amendment, with no unlocked fallback or automatic retry. The current actual Rust caller compiled; its separate keygen extension remains uncompiled and unexecuted.

Future proving uses an exact resolver, published parameter hash, no-network child, actual native prover and independent verifier. It must retain raw proof/key/statement identities and mutation results. Native proof success would still leave constructor authority, contract-instance separation, ledger binding, real custody, replay exclusivity, property/intent/transition/history correspondence and Preview acceptance open.

## Active plan

1. Completed: adapter v3 actual check plus five relation refusals, independently reviewed.
2. Completed: repaired native ledger caller compiled; production handoff and actual keyless native preparation succeeded. The preceding compile and mixed-unit-operation handoff failures remain archived.
3. Completed: two fresh full current source and actual-result audits approved narrow successful v5 preparation publication with stated qualifications. No proof/application/Preview or new resource grant follows.
4. Source prepared separately: offline native keygen CLI; bounded SRS/build/keygen resource proposal is being prepared. It has not executed or obtained resource approval. The obsolete standalone binding0 prover is not the finalized financial target.
5. After required current reviews: acquire exact official parameters and generate keys under reviewed limits, then use real ledger-finalized nonzero binding for one complete financial proof, independent verification and strict application.
6. Preserve complete fees/state/events/replay/fault evidence; review actual results and publish only the proven scope.
7. Resolve authentic deployment/asset/funding/account/time/history correspondence and actual Preview compatibility before settlement claims.

Latest observed result: [V5-ACTUAL-PREPARATION.md](V5-ACTUAL-PREPARATION.md). Its [immutable history](evidence/ledger-v5-preparation-history/archive-manifest.json) retains original source/resource votes, source maps, actual handoff/config/artifacts and actual native preparation. Historical stage notes retain their original as-of status.

[Immutable evidence archive](evidence/archive-manifest.json) preserves byte-identical copies of the kernel, adapter source/preimages, failed dependency preparations and successful native source-check receipts. Absolute runtime paths are historical evidence, not a portable installation contract.

Research artifacts currently live under `/home/charl/research/moriarty-signed-intent-2026-10-01/`. This note is a working record; execution observations will be filed with raw receipts, exact hashes and current independent audits. It grants no native or network execution authority.
