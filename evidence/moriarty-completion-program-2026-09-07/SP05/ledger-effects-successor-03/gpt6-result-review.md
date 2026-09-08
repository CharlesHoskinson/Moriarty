# Independent source result review — incomplete

Candidate: `6774e0479c37bb95b3fd1dd6371d1fdd1729e8e40f34c770b1acadbbf3268fdc`.
Reviewer: OpenAI `gpt-6-astra`, fresh independent reviewer `/root/ledger_effects_successor03_review`.

`reviewExecution: INCOMPLETE_AUTOMATED_SAFETY_STOP`  
`candidate: NOT_ACCEPTED`  
`overallSP05: BLOCKED`

The parent reported an automated safety flag and directed a documentation-only handoff. Probing and testing stopped. This record preserves existing observations; it does not complete the independent audit or grant acceptance.

## Supported source predicates observed

The native comparison branch composes recipe transactions with native `Transaction.merge` and compares `eraseProofs().serialize()` bytes. This is a substantive improvement over partial action projections. One independently exercised native synthetic signed unshielded transaction passed the public funding path with unchanged native bind, reserved gross100 and one submission, and reached an inert submit adapter. No real submission occurred.

Source inspection found separate DUST `vFee` accounting and nonnegative bigint counter validation before reservation updates. Unshielded gross includes all inspected recipe parts and both offer classes without subtracting refunds. An independently exercised malformed negative reservation rejected before finalize, although after controlled balance/sign calls. Failed inert finalization retained reservations. These bounded observations do not establish full accounting coverage or proof compatibility.

## Blocking observations

- **Units across calls:** one persistent counter admitted distinct native token types, first100 and then200, producing reservedGross300. The gross cap has no persistent asset identity.
- **Diagnostic admission:** a plain object containing native intent Maps passed public `fundUnshielded` through inert submit. The report's diagnostic-only boundary is not enforced. Controlled adapters supplied these objects; this does not claim that the pinned facade ordinarily emits them or that the ledger accepts them.
- **Recipe shape:** an unknown recipe discriminator with a native base transaction also reached inert submit.
- **Registration authorization:** a native registration carrying a signature over an unrelated payload exposed an undefined signature getter and reached inert submit under a DUST cap. Missing verification is not evidence of authorization or ledger validity.
- **Required accounting:** shielded-input/transient gross and required multiple asset accounting remain unsupported, as the candidate report acknowledges.

A further observation needs disposition: allowFeePayment900 reached inert finalize under cap1000 with999 already reserved. The source compares authorization with the total cap and stores no authorization reservation. This establishes neither900 paid DUST nor a valid registration signature. Counter validation also followed controlled balance/sign calls, leaving the pre-side-effect requirement unresolved.

## Native DUST work not completed

No native DustSpend fixture was obtained. The initial synthetic local-state attempt failed because the UTXO was absent from wallet state. The updated retained probe failed with `for seq=0 we use DustPublicKey instead of DustSecretKey`. Native DUST public-path fee and normalization tests were therefore not performed. No further construction or probing followed the stop.

## Evidence and limits

`gpt6-independent-probes.mjs` and `gpt6-independent-results.json` contain the existing independent work. `gpt6-independent-hashes-before.json` records matching hashes for all8 frozen candidate files, all8 corresponding worktree files and201 binding source pins, with the exact candidate hash recomputed. No post-review hash check was performed after the stop. The reviewer made no source edits and did not alter retained root/predecessor evidence.

Root verification previously reported40 supplied tests,7 helper,3 native,3 shape and3 gross tests passing, plus38 predecessor expectations and11 decisive action/counter expectations. This review did not independently rerun those suites. Their passing status does not close the independently observed gaps.

All exercised finalization/submission boundaries used inert adapters. There was no wallet restore, real proof generation, native facade finalization, network operation or ledger submission. Genuine proof compatibility, full normalization coverage, native DUST accounting, registration authorization, typed persistent budgets, remaining R6 duties and original R1-R5/R7-R8 remain open. SP05 financial Preview settlement, native PCD and remaining sprint acceptance are unchanged. See `gpt6-result-review.json` for the explicit unresolved-requirement list.
