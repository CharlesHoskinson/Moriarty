# SP01.3 successor contract proposal

Status: proposal complete, pending review. Not a semantic freeze.

Author: Grok 4.6 high. Worktree: `/home/charl/Moriarty/.worktrees/sp01-successor-contract-grok`. No subagents. No network. No installs. No runtime edits. No commits.

## Owned files

- `experiments/moriarty-language/spec/successor/semantic-contract.md`
- `experiments/moriarty-language/spec/successor/signing-display-schema.json`
- `experiments/moriarty-language/spec/successor/semantic-decisions.json`
- `experiments/moriarty-language/spec/successor/signing-examples.json`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

Old `bounds.json`, `types.ts`, and `codec.ts` bytes are unchanged.

## What the proposal defines

Named finite domains, staged `pre`/`next`/`post` evaluation, typed Core operations with reject codes, five composition operators, request states, recovery work, and four mandatory claims. ExactPlan and OutcomeIntent are closed tagged documents. Host SHA-256 with `MORIARTY-SUCC-*/0` domains is proposed only. Atomic SHA domains are not reused as accepted successor crypto.

## Verification that ran

Python 3.14.4 stdlib `json` and `hashlib`. Installed `jsonschema` 4.19.2 used read-only as Draft 2020-12. No package install.

Commands (stdin helpers, none retained):

1. SHA-256 pins of LANGUAGE-DESIGN, RP01, SP01.3, action-targets, worked-examples, loan-swap design, and old profile files.
2. `Draft202012Validator.check_schema` on the signing schema.
3. Validate both valid canonical documents.
4. Reconstruct 15 invalid mutations. Schema fail or pass matches the intended stage.
5. Recompute execution-body and intent digests from canonical UTF-8. Independent of `codec.ts`.
6. Confirm old profile SHA-256 values.

Results: contract 4424 words. JSON parses. Local `$ref` targets exist. `additionalProperties` is false on records. Valid ExactPlan digest `40d8a0c5f2365226cdf010204643b78131757441b3c5ccc411b85f414d0d4261`. Valid OutcomeIntent digest `875cefef0592f403dda3168e1364737e56cd3a273bbcca7cf4f55dacccb803eb`. No signature verification is claimed.

## Remaining freeze gates

SP01.2 challenge reconciliation. User-selected majority on `majorityRequired` decisions. Native hash correspondence. Usability evaluation. Formal proofs. Successor `bounds.json` as a new SP02 file.

GPT-6 must review this proposal. This report does not approve it.
