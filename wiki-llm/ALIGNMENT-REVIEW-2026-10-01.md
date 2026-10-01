# Design, roadmap and goal alignment review

2026-10-01. Repository observations and recommendation after reviewed key-generation publication, PR15 / main `df8b155143e90450168af04fbfd20f7620043620`. This review changes no semantic profile, target, acceptance obligation, resource allocation or recursion planning assumption.

## Decision

Retain the [product contract](../docs/MORIARTY-PRODUCT-CONTRACT.md), [consolidated design](../docs/MORIARTY-CONSOLIDATED-DESIGN.md) and the single [U0–U7 roadmap](../ROADMAP.md). Keep one implementation slice on U1/U2: connect independently authored supported financial source and canonical signed intent to actual native proof and complete ledger effects. The authoring beta is a delivered precursor; it does not close U2 or the full independent developer release U7.

## Goal-to-evidence check

| User goal | Current evidence | Remaining requirement |
| --- | --- | --- |
| A readable real language with financial sugar | Nine-domain research, five PL reviews, converged bounded beta, exact quantities, nominal IDs, eight-family authoring examples | Broad financial relations/lifecycle horizon remain specified-only; declarations do not authenticate on-chain assets/accounts |
| IDE and AI developer experience | CLI, language server, VSIX/Neovim, read-only MCP, six independent developer trials, guide/site plan, PR11 prerelease | Actual VS Code GUI activation, broader client adoption, deployed tutorial site and independent human onboarding remain unperformed |
| Real Midnight signatures consumed by Beta | Pinned official Rust primitives, canonical signed-intent codec, actual Beta API/CLI consumer, reviewed PR12/PR13 | Authenticated owner/key lifecycle and chain head/state provenance; valid signature alone is not authority |
| Atomic complete financial behavior | Local copy-on-write CAS, complete transfer/repayment effects and hostile controls | Native guaranteed/fallible phases and fees require explicit signed failure policies; local rollback does not prove native atomicity |
| Native financial proof and acceptance | Actual runtime/native relation match, keyless preparation, reviewed build/SRS/keygen, then a retained finalized proof/signed transaction | Latest strict WF refused OutputsNotSorted; independent verification and full ledger Success remain unproved |
| Real Preview transfer and repayment | Earlier scoped historical loan/swap demonstrations; read-only target research | The current signed/kernel target has no new submission/finality. Published-release format mismatch is conditional on live build correspondence; do not infer native9 compatibility or silently downgrade |
| Mandatory properties, intent, transition and history | Explicit controlling acceptance obligations and finite local checks | General compiler/constraint correspondence, legitimate origin, all retained native financial negatives, portable recursive/private PCD evidence |

Evidence: [beta checklist](beta-language-2026-09-30/CHECKLIST.md), [crypto delivery](midnight-crypto-2026-09-30/DELIVERY.md), [signed delivery](signed-intent-2026-10-01/DELIVERY.md), [reviewed keygen](native-financial-kernel-2026-10-01/REVIEWED-KEYGEN-MILESTONE.md), and [actual v8 refusal](native-financial-kernel-2026-10-01/V8-03-ACTUAL-REFUSAL.md). Historical snapshots retain their as-of counts and review status.

## Progress and alignment risks

Observed progress is executable: real signature verification, a production handoff with byte-identical native preimage, actual native key generation, and a finalized native proof path reaching a specific native envelope refusal. Documentation or merges alone would not establish these steps.

The strongest concern is specialization: a fixed manually funded kernel and stipulated identity map may pass without establishing the language's general supported-program contract. Keep that explicit. U2 requires a newly authored program and a structurally contrasting program, a generality argument over the admitted grammar and tamper controls. Do not certify a fixture-specific path as the general compiler or narrow the objective to make tests pass.

A second concern is failure semantics: native PartialSuccess can retain guaranteed-phase effects and fees. Preserve gross spending, signed fee/failure policy and residual duties through every permitted outcome. Neither an in-memory atomic store nor one successful fixture establishes those rules.

The authoring work is aligned with the user's syntax/DevEx request. The user has now explicitly requested a parallel tutorial website implementation and six designer reviews, including GPT-generated images. That bounded website work proceeds in separate worktrees. Further syntax expansion, optional AI automation, federation infrastructure and broad application growth should wait while the main native delivery capacity closes the signed financial acceptance path. The U3 two-asset conditional settlement/recovery discriminator remains the next extension after U2; full native history/private composition remains U4 and is not replaced by ledger induction.

## Next delivery sequence and stop conditions

1. Reproduce the native offer-ordering defect with a cheap check, repair envelope construction before binding/proving/signing, and freeze the exact successor. Preserve semantic effect order and every failed attempt.
2. Obtain fresh source/resource reviews for the changed caller and bounded build/preflight/prove/verify allocation. Use the existing parameters and keys where exact relation identity permits; no unchanged retry, fifth Compact compile or silent resource reset.
3. Demonstrate strict WF, complete ledger Success, positive fees/remainder, exact state/UTXOs, replay refusal and independent verification; audit the actual result. Expand missing financial negative controls against that same consumer.
4. Qualify authenticated genesis/deployment/funding/accounts/assets/time/history and source/compiler/native correspondence; add contrasting independently authored source. Keep the four external premises and four unverified bindings visible until each has evidence.
5. Revalidate actual Preview interfaces/build correspondence before any dependent wallet/public financial action. Preserve existing wallet identities and contract state; a compatibility blocker does not stop independent semantic/native repairs.
6. Continue U3–U7 under their original exits. Each claimed capability needs its own exact observable predicate; no milestone is closed by this review.

## Verification and revisit rule

Root inspected the controlling product contract, complete consolidated design, roadmap, beta convergence and current delivery/result records. The 107-file PR15 publication matched its frozen blob hashes; the scoped CLI tests passed again. Canonical SRC-0117 lint observed 20 historical navigation findings and zero provenance/read/configuration errors. Current v8 result and complete artifacts are separately frozen; stronger financial publication still requires fresh actual-result audits.

Revisit this alignment when strict native acceptance succeeds, a new semantic profile/target is proposed, an authenticated-state prerequisite changes, or released recursion/Preview interfaces change. Success means the original supported-language-to-settlement objective becomes true; it is not redefined around keys, local models or a single fixed proof.
