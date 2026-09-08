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

## What this pass did

The interrupted seed already had closed JSON defs. This pass wrote the missing prose and decisions against those defs. It did not re-author the generator.

Narrow correction: `allowedEffectSet` stays `CoreOp`. The accrue example no longer lists EffectKind names `Accrual` and `DueCreated`. It lists `DebtAccrue` and `DebtCreate`. Schema was not widened.

## R1 to R8

R1 specified. Finite CoreOp, EffectKind, StoredValue, positions, requests, messages, EventClaim, RewardAccount, genesis, admin. Source versus Core versus effect layers are distinct.

R2 specified. True-end patterns. DomainValidation widths. Four claim kinds exactly once. Schema and domain stages are separate.

R3 specified. FIELD_META is schema `x-display`. Every-leaf walk. Empty arrays display as `[]`. Price unit is base per quote.

R4 specified. Closed bodies and framed DAG. Preimages sit in `preimageRegistry`. Native hash remains unresolved.

R5 specified. Debt principal, accrued, dates, controller, allocation, conversion, negative rate, cap, Price `basePerQuote`, Remainder dust.

R6 specified. Actor-indexed ledgers. Successor binds the original intent. Pinned versus AdmittedContext versus GenesisNone. Refunds do not restore gross. Fees count in net. Duties survive cancel.

R7 specified. Charge 1 per CoreOp. Recovery aliases the recovery reserve. Split, join, cancel, migrate equations. Profile maxima. Interleave conflict `write_path_or_identity_or_cap_key`. Par frame `disjoint_keys_concat_obs`.

R8 specified. Invalid fixtures reconstruct from `base` plus `sets` plus context. Net-after-fees includes Fee 30. Clone has two children. Cancel has a winning fill. Migrate recomputes `executionBodyHash`. All `executionStatus` values are specified-only.

## Verification that ran

Python stdlib `json` and `hashlib`. Installed `jsonschema` 4.19.2 used read-only as Draft 2020-12. No package install.

1. `Draft202012Validator` on four positive canonical documents. All pass after the accrue CoreOp fix.
2. Newline, CR, U+2028, cap newline, and four-copy `ContractInvariant` controls fail schema.
3. UInt64 max and UInt128 max pass schema. Max plus one passes schema and is a domain reject.
4. Independent framed SHA-256 of profile, program, claims, genesis, observations, execution bodies, and the four document digests.
5. Invalid rows match `schemaExpected` after reconstruction.
6. Old profile SHA-256 values.

No context evaluator ran. No signature verification is claimed.

## Remaining freeze gates

SP01.2 challenge reconciliation. User-selected majority on `majorityRequired` decisions. Native hash correspondence. Usability evaluation. Formal proofs. Successor `bounds.json` as a new SP02 file. Native statement for the four claims.
