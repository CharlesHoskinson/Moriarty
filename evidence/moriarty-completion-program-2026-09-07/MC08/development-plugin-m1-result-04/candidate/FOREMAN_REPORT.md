# FOREMAN_REPORT

This pass repairs two evidence-identity defects. It does not accept Milestone 1.

Parent candidate `268e8ff679774265a2aa9a4dd7beadeb5ed9d460e4a84b36af7d65cccfbf5bd7` is unchanged. This pass owns only `records.py`, `test_records.py`, and these two reports.

## Initial red

Independent GPT-6 review of candidate 03 was BLOCKED (M1-C3-R1, M1-C3-R2, and review verdict in M1-C3-R5).

Reproduced before edits against current source:

- `publication-rewritten`: `candidateCurrent=true` after arbitrary replacement of `semantic-challenges.json` with an updated `publishedControlSha256`.
- `prerequisite-no-proof`: `entryEligible=true` when the required campaign kept only `candidateHash` and `status=complete`.
- `review-unknown-verdict`: `NOT_REVIEWED` left `candidateCurrent=true`.

Copied-budget admission is not live resource authority. Resource defects were left open.

## Changed behavior

Publication: freeze `ownedFiles` is the original control commitment. Current bytes must match `publishedControlSha256`. Identity holds when those digests equal the freeze digest. A transformed control must match archived freeze bytes next to `*-freeze.json`. Only `subsets.RP01-MC02` acceptance, control metadata, output hashes/status, and closure wording may differ. Other fields, including non-JSON substitution, are `publication-control-substitution`. Missing archive bytes are `publication-original-unavailable`. The live path is not exempt from identity.

Prerequisites: each required stage reuses binding and candidate checks, including review and acceptance. Stage, owner, profile, and candidate must agree. Complete status is required. Resource admission is not required for a completed prerequisite. Hash-plus-status is not evidence. Mutated or missing prerequisite files fail. Unrelated F0 blockage still does not block SP05. Cycle detection is unchanged.

Review and acceptance: required string fields are closed. Acceptance status must be `complete` or `complete-*` without `revoked`. Review pass is `APPROVED`. `BLOCKED` and `REJECTED` remain blocked. `NOT_REVIEWED` and any other verdict are `review-verdict-unresolved`.

## Exact checks

Positive: genuine `sp01-loan-swap-grok-01` report and funded review keep `candidateCurrent` and `entryEligible` true with the archived candidate-04 control. A complete prerequisite that uses the same loan freeze, acceptance, review, and binding keeps `entryEligible` true.

Failures: publication rewrite and forbidden-field edits set `candidateCurrent` false. Prerequisite label-only records and missing review files set `entryEligible` false. `NOT_REVIEWED` sets `candidateCurrent` false. These flags are independent of `resourceAdmitted`.

## Remaining open

Resource defects M1-C3-R3, M1-C3-R4, and resource-amendment/pass-accounting schema gaps stay open. Unknown resource fields, omitted charges, and unrelated pass scope still admit under the copied-budget fixture.

Milestone 2 store and CLI, Milestone 3 packaging, and Milestone 4 ledger demonstration are not in this pass. Genuine atomic-prepare and atomic-accept records still lack the supported freeze/acceptance shape. Those prerequisites stay named unavailable unless a supported complete candidate is present.

This pass does not accept the plugin, any sprint, or product behavior.
