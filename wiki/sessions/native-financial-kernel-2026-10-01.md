---
id: moriarty.native-financial-kernel.20261001
title: Native financial kernel and actual proof route
type: session
status: research-draft
created: 2026-10-01
updated: 2026-10-02
tags: [moriarty, midnight, native-proof, signatures]
sources: [SRC-0115, SRC-0116, SRC-0117, SRC-0118, SRC-0119]
---

# Native financial kernel and actual proof route

## Initial source snapshot

The following initial observations retain their original as-of scope. Current execution evidence is recorded below.

[Working dossier](../../wiki-llm/native-financial-kernel-2026-10-01/README.md) records the next missing predicate after [published signed-intent integration](../../wiki-llm/signed-intent-2026-10-01/DELIVERY.md). PR13 merged at `033e9a90465d4c06b787dedb5010e4e18715e030`; the historical signed-intent session snapshot remains unchanged. The delivery record supersedes its pending-review status and earlier test count.

Source facts: the pinned Compact0.35/ledger9 family supplies an actual SHA256/ECDSA kernel and official runtime-to-proof preimage converter. The high-level Rust proving wrapper discards the public statement; the public `Zkir::prove` interface returns native proof, statement and skips. The pinned full verifier route checks transcript EOF. [Source collection](../../raw/sources/native-financial-kernel-2026-10-01/collection.json) preserves exact source commits, hashes and coverage limits. These facts do not establish a compatible build or proof.

Experiment observations: a fixed manually specialized financial kernel was compiled without keys/proofs and evaluated locally. It checks signature/low S, frame digest, state/head, replay nonce and full transfer/fee effects. A row model reports k17/114250 rows; this is not witness acceptance or proof fit. Runtime tests exposed cross-instance constructor trust and predecessor-race gaps. The initial locked native-adapter fetch failed and its allocation remains consumed. A version-preserving successor lock and separate bounded fetch/build/check amendment are under fresh independent review. Native source check, key generation and proof remain unperformed in this snapshot.

Proposed next consumer: retain the actual native public statement, perform native self-verification and verify immutable proof/statement in a separate process with real mutations and strict outer decoding. Parameter acquisition, source/result audits and bounded resource votes are distinct prerequisites. The published k17 SRS has only a HEAD receipt here; its body and ceremony trust are unverified locally.

Open financial acceptance: manual fixture escrow, constructor/account authority, contract-instance separation, actual ledger binding, exclusive replay commit, Moriarty compiler lowering, property/intent/transition/history correspondence and Preview settlement. Observed Preview ledger8 differs from the new ledger9/compiler0.35 route. No real wallet/state or public transaction was changed. A successful local check or proof must not close those gates.


## Reviewed actual preparation — 2026-10-01

Experiment observation: the production signed Beta transfer now reaches generated keyless native artifacts and actual Rust ledger preparation. Both phases exited0 with preserved source and compiled-binary identities. All293operations/47reads, complete4,316-byte frozen preimage, all eight native call-context fields and native financial partition checks matched. Actual IR preprocessing returned293None skips. Native preliminary fee estimate1128519108356650 is below stipulated allowance1e20 and generationless fixture availability5e21; this is not consumed fees.

Two fresh complete current source and actual-result audits approve only this narrow preparation publication. [Original result, audits and limitations](../../wiki-llm/native-financial-kernel-2026-10-01/V5-ACTUAL-PREPARATION.md) retain exact source/artifact/binary hashes and the original failed compile/handoff attempts. Grok returned grok-4.6-build/end_turn/process0; Astra was requested through host dispatch as gpt-6-astra medium without separately exposed returned-provider attestation. Frozen monitor metadata overstates RX sampling and thread enforcement; corrections and one harmless operation-kind typo in the Grok report are preserved beside the unchanged originals.

No registered VK, PK, SRS body, key generation, proof, strict well_formed, application, applied8990escrow, or Preview transaction is established. Future keygen source is staged separately and uncompiled; its resources are under fresh review. Actual partition has guaranteed=None/fallibleSome: future guaranteed fees/registration may survive a fallible failure, so no blanket rollback or full success_only correspondence follows.

Source fact plus conditional inference: at08:10:16UTC, public RPC/indexer reported the same Preview block with ledger8.1.2/spec1000300. Matching official published runtime and ledger sources support IR2.0/V2proof/V3operation; the candidate requires IR3.1/V3proof/V4operation. Their serializers/loaders reject the candidate versions. This is incompatibility if the live node executes those matching published sources. It is not an observed network rejection or live build/WASM attestation. [Supplemental immutable collection](../../raw/sources/native-preparation-preview-2026-10-01/collection.json) links the original redacted responses and pinned public source captures.

Authentication of address04, A1funding, account roles, destination ownership, NIGHT/time, stored round1, predecessor/history, generic lowering, contract properties, intent refinement, transition validity and recursive PCD remains open. ECDSA intent owner and Schnorr NIGHT payer are distinct; application Afee10 and protocol DUST are distinct. No wallet, user state or public transaction was changed. No canonical financial claim is promoted by this intake.

## Actual build and key generation — 2026-10-01

The preparation section above retains its recorded scope. Later experiment observations: the isolated native keygen caller compiled offline; the new full public parameter request matched exactly25,166,212bytes/SHA4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74; actual official k17 key generation produced tagged PK/VK/IR with loader EOF and byte round-trip checks. All three parent receipts record exit0/no stop and preserved source/new-binary identity. [Actual result and immutable evidence](../../wiki-llm/native-financial-kernel-2026-10-01/V7-ACTUAL-KEYGEN.md) record complete source/result hashes, resource measurements, externally retained large artifacts and historical labels. [SRC-0117 collection](../../raw/sources/native-keygen-v7-2026-10-01/collection.json) is provenance metadata over inside-vault source/receipt archives; it does not claim large binary bodies were copied.

The original timed-out320KiB download and original624MB executable remain preserved. The new request did not resume or erase that failure. Ceremony trust remains independently unaudited. Keys satisfy neither financial proof nor well-formedness/application/ledger acceptance; resolver/registration agreement remains NotChecked. Fresh actual-result review is separate from the prior resource approval.

[Next proof source preparation](../../wiki-llm/native-financial-kernel-2026-10-01/NEXT-PROOF-SOURCE-PREPARATION.md) preserves an original unexecuted draft that incorrectly supplied tagged IR to a JSON loader, its correction, and an exact667-input v8-03 successor. The proposal uses one real finalized financial proof and a separate native verifier process under new source/resource/result gates. This intake records source preparation only, with no v8 resource authorization or proof execution. Broader native financial negative controls remain partly specified-only.

Authenticated custody, mint/deploy/funding, account/asset/time/history bindings, generic language/property/intent/transition/history/PCD correspondence and real Preview settlement remain open. No accepted financial claim or user wallet/state/transaction is changed by this intake.

## Reviewed sorted proof and actual application refusal — 2026-10-01

Experiment observation, SRC-0118: [the sorted R3 attempt](../../wiki-llm/native-financial-kernel-2026-10-01/R3-04-ACTUAL-APPLICATION-REFUSAL.md) retained a finalized 6336-byte proof, statement and signed sealed transaction. Source ordering and retained application outputs establish successful real producer verification and default Real native well-formedness in that process. Native application then returned Failure(InvariantViolation(NightBalance(24001000000000000))). Genesis and returned state are byte-identical. Independent verification, full Success, accepted effects, fees and replay-after-success remain unperformed.

Both fresh full actual-result audits approve source and failed-result faithfulness only; root inspected both complete reports and actual terminal Grok metadata. Requested Grok4.6 high returned grok-4.6-build/end_turn/process0. Astra was requested as gpt-6-astra medium without separate returned-provider attestation. [Collection](../../raw/sources/native-financial-r3-failure-2026-10-01/collection.json) binds the portable byte-identical evidence archive and preserves the original failed attempt; no new accepted financial claim follows.

Source fact and inference: native default genesis reserves the entire NIGHT supply; the test fixture manually adds1e12 NIGHT. Their sum equals the observed excess. A separately reviewed retained-genesis invariant diagnostic is the next empirical predicate; this intake establishes no completed diagnostic or funded repair. A supply-consistent local funding route would still require a new context-bound proof and would not authenticate public-chain funding, owner mappings, time/history or general compiler/property/intent/transition/PCD correspondence. Existing Preview format limitations and all product exits remain open.

## Reviewed R4 proof and complete R5 verification — 2026-10-02

Experiment observation, SRC-0119, S3: [R4/R5 evidence](../../wiki-llm/native-financial-kernel-2026-10-01/R4-R5-REVIEWED-SUCCESS.md) supersedes the earlier R3 refusal for the fixed trusted public fixture. The retained finalized proof passed default Real native application with Success, complete accepted effects, positive consumed DUST and replay refusal. R5 independently verified it, checked 1035 public-input mutations twice and completed all six native history/claim fault refusals. Three fresh actual-result audits approved only this scope; unchanged reports, receipts, source, proof and full logs are mapped in the portable archive. [Source collection](../../raw/sources/native-financial-r4-r5-2026-10-02/collection.json) pins the exact original acceptance and freeze.

The partial R4 verification helper failure and earlier consumed attempts remain failures. Resource figures are sampled; 293 operations/47 reads were enforced at R4 construction, not recounted by R5. Large compiled/key/parameter bodies and dependency closures remain external and hash-bound; this publication is not a self-contained native rebuild. Generic compiler correspondence, source-owner/native-payer and authenticated funding/deployment, general contract/intent/transition/history/PCD and Preview financial settlement remain open. No public transaction was submitted, and no canonical financial claim is promoted.
