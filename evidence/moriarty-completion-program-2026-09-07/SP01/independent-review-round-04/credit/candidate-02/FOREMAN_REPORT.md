# FOREMAN_REPORT credit financial-content correction

Status is specified-only. Independent GPT-6 Astra review is pending.

This pass repairs CREDIT-GPT6-01 through CREDIT-GPT6-05 in owned bytes. This pass is not runtime. This pass is not proof. This pass is not full RP01. This pass is not a semantic freeze. This pass is not sprint acceptance.

## Provenance of prior failed processes

Prior candidate01 remains immutable. Its financial fragment hash is `fc7cc675c0b0a76e8e393e9d899da46814c6c276077e29827495de3d35692de6`. Independent GPT-6 Astra verdict on that freeze is BLOCKED.

These author processes remain failed. This report does not convert them to success.

- design-worker exit 124 after 892.1841544149793s of charged 900s
- completion-worker exit 124 after 292.21378911699867s of charged 300s
- quantity-repair worker cancelled with exit 1
- interrupted-credit.json SHA-256 `97578f05c3cf33bad94c4827cdf59b1d1148194a214f2ad40e57673d0fadb77f` (187564 bytes)

The quantity-repair packaging pass wrote compact `credit.json` at 113543 bytes. Independent review then blocked five financial content groups. That packaging pass is not this correction.

## This pass

Worker is Grok 4.6 high. Worktree is `/home/charl/Moriarty/.worktrees/sp01-map-credit-grok`. Command is `python3 /tmp/fix_credit.py`. First run exit 1 on a string/int expected check. The 35 recorded expected strings were unchanged. The check was coerced to those strings. Second run exit 0.

Independent Python stdlib recomputed cash, debt, work, prefixes, and negative prerequisites. Moriarty evaluator was not used. `verify-credit.py` was not run.

- python3 /tmp/fix_credit.py first attempt: exit 1
- python3 /tmp/fix_credit.py second attempt: exit 0
- independent ref/cash/prefix verifier: exit 0
- independent arithmetic failures: 0

## Corrected bytes

- credit.json SHA-256: `fae6d1f56d0a74a45141506d8ec0065af369dcaa2f85182131d4f711f1340ccb`
- credit.json file bytes with final newline: 255667
- json.dumps compact bytes: 255666
- input-packet.json SHA-256 unchanged: `b6a72b079f4b64f9eae2ca89d048b5d1e67d65b6f2c8d57c54cdc73d18b66dcf`
- serialization: Python json.dumps separators comma/colon, ensure_ascii=False, plus final newline

Five rows, five traces, and the original ten mutation identities remain. Three isolated controls were added.

## GPT-6 findings in these bytes

CREDIT-GPT6-01. Each of the ten negatives now has a complete typed pre-context, a complete attempted state, an admission boundary, a reject stage, empty rejected-attempt effects, and an exact ref-only rejected state. Intermediate prefixes are materialized under `/library/committedStates`. Redemption mutation 7 rejects after step 2 with ordinary remaining 982. It does not use postState 974.

CREDIT-GPT6-02. Mutation `heldouts:pending-redemption:mut:replay-claim` uses named source `vaultDeployed` to fund vaultLiquid 3800 after the first claim. R1 claimable remains 0. Type, authority, balance, resource, and currentness pass. The claimable/replay predicate fails. The unfunded vaultLiquid 0 case is a separate control.

CREDIT-GPT6-03. Row footprints list per-step reads, writes, deletes, consumes, and creates. Refinance reads fee.unconditional.USDC and consumes the old service duty. Redemption reads and writes lock and fill authorities. Bad debt reads and writes seize and nominal authorities, deletes pos-collateral-bob, and consumes bob.serviceDebt.1000. Alias expansion is deterministic. Frame lists reads that are not writes.

CREDIT-GPT6-04. The four claims are finite predicates on explicit input records. Primitive updates, full post equality, frame, ownership agreement, current-head membership, and unique consumption are defined. Named assumption is ledger-current-heads-at-admission. Isolated negatives cover nominal holder buyer with tokens still at shareholders, and a stale current head.

CREDIT-GPT6-05. Request field sharesClaimable is written 40 to 0 and remains 0 on the final request. Lock domain and fill-burn domain are separate counters. Partial branch is Pending/Claimable/Pending. Full settled branch is Pending/Claimable/Claimed/Settled. Bad-debt continuation controller is market-controller. MC07 is implementation owner only. Collateral gross cap 20 cannot cover two 20 moves. Residual write-off updates principal, accrual, total, status, and discharged together.

## Unchanged integers

All 35 recorded verification expected strings match the candidate01 freeze. Cash, debt, and work oracles still use conversion 95, recovery 400, remaining debt 600, loss 600, and refinance 50000 plus 50 from newLender.

Independent replay:

- refinance transfers 4, mismatches 0, work 60 remaining 964
- redemption transfers 2 plus burn 40, mismatches 0, work 50 remaining 974
- badDebt transfers 2, mismatches 0, work 56 remaining 968

Local refs unresolved: 0.

## Not accepted

No runtime behavior. No native or recursive proof. No full RP01. No SP01 completion. No semantic freeze. No protocol conformance. No network or ledger settlement. No all-twelve-sprint acceptance.

Owned files:

- experiments/moriarty-language/spec/successor/financial-fragments/credit.json
- FOREMAN_REPORT.md
- FOREMAN_REPORT.json

No git write. No network. No proof. No build.
