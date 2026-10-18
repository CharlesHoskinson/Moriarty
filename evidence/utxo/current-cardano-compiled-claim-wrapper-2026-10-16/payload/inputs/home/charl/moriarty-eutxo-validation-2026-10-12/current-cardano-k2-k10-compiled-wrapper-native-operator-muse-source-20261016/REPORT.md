# K10 compiled-C2 native wrapper operator — source authored (Muse Wonder fallback)

## Verdict

**FOR — source-ready as a candidate, authorizing nothing.** The top finding is that the minimal five-file operator correctly implements the already-compiled C2 mint wrapper's sourced native contract, while its candidate AUTH deliberately cannot run. First blocked boundary: the pending fresh actual-input audit of the nine generated inputs, before any root AUTH.

## What was authored

- `runner/operator.py` — stdlib-only operator: typed exact AUTH (no bool-as-int anywhere), eleven literal pins, release identity parsed from the pinned data-only manifest, seven sequential `uplc eval --cbor /inputs/script.hex` calls with each 2649-byte context as one argv element, strict T1 JSON and exact EvaluationFailure/Costs parsers, intent counted before Popen, partial-stage preservation, and whole-custody hashing of the nine inputs, staged mode-555 tool, AUTH inputs and five operator files before and after.
- `AUTH-CANDIDATE.json` — `CANDIDATE_NOT_AUTHORIZED`, literal placeholder operator SHA for root to fill, and PENDING input-audit attestation; the operator rejects each of those states and reads no future audit file or invented hash.
- `EXECUTION-PROPOSAL.json`, `README.md`, `DESIGN-POSITIONS.json` — the bounded order (pending input audit → root materialization/SHA → fresh full non-author audit with two substantive votes → exact new AUTH → seven calls → fresh actual-result audit → only then FX1/signatures/settle/qualified ledger cost/Preprod), budgets (7×60 s + 180 s reserve = 600 s outer + 10 s kill; CPU 30:35; 8 GiB; 16 MiB caps), isolation, and twelve FOR author positions.

## Evidence and limits

The nine pins match the generated manifest and the ordinary root RESULT (exit 0, one attempt, pins unchanged), policy `6a4b67e7…59c1` is the independently recomputed blake2b-224 over `0x03` plus the 2770-byte CBOR compiled script (one `590acf` string, 2767 inner flat bytes, inner flat not hashed) — ordinary inspection evidence, not a compile theorem. Guard names are source-expected finite predicates, not runtime instrumentation. The fabricated `2222…/555…` context, zero fee and unbound 2030 expiry remain LOCAL FABRICATED/UNBOUND; local VM costs are not ledger ExUnits. Source readiness, finite ordinary evidence, proposed authority and actual proof/target acceptance are kept distinct throughout.

## Independence

This is Muse Wonder 1.0 source authorship under the user's explicit Gemini/Muse fallback after the original Opus author timed out (fallback confirmation `nativeExecutionAuthorityCreated=false`). It is not an independent review, not Opus, and grants no approval or execution authority; a fresh non-author audit must AGREE before the exact new AUTH.

