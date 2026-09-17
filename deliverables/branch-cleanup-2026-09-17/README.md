# Branch cleanup — September 17, 2026

The user requested branch cleanup before resuming the delivery plan.

Baseline main: `26074698c89fdc183b6b12e819d043cc6da96696`.

Every retired branch is preserved by an exact remote tag under `archive/2026-09-17/`. Unmerged work is retained for review; archiving does not approve or merge it. Main is unchanged.

| Retired branch | In main | Recovery tag |
| --- | --- | --- |
| `DeFiInterface` | Yes | `archive/2026-09-17/DeFiInterface` |
| `feat/expression-funded-repayment` | Yes | `archive/2026-09-17/feat/expression-funded-repayment` |
| `feat/financial-postconditions` | Yes | `archive/2026-09-17/feat/financial-postconditions` |
| `feat/financial-state-reads` | Yes | `archive/2026-09-17/feat/financial-state-reads` |
| `feat/loan-lifecycle` | Yes | `archive/2026-09-17/feat/loan-lifecycle` |
| `feat/loan-origination-accrual` | Yes | `archive/2026-09-17/feat/loan-origination-accrual` |
| `feat/moriarty-dev-plugin` | Yes | `archive/2026-09-17/feat/moriarty-dev-plugin` |
| `feat/multiple-named-actions` | Yes | `archive/2026-09-17/feat/multiple-named-actions` |
| `feat/security-token-asset-agenda` | Yes | `archive/2026-09-17/feat/security-token-asset-agenda` |
| `feat/session-handoff-2026-09-12` | No | `archive/2026-09-17/feat/session-handoff-2026-09-12` |
| `feat/source-defined-repayment` | Yes | `archive/2026-09-17/feat/source-defined-repayment` |
| `feat/sp05-adverse-integration` | Yes | `archive/2026-09-17/feat/sp05-adverse-integration` |
| `feat/sp05-fee-comparison` | Yes | `archive/2026-09-17/feat/sp05-fee-comparison` |
| `feat/sp05-financial-integration` | Yes | `archive/2026-09-17/feat/sp05-financial-integration` |
| `feat/sp05-preview-exit` | Yes | `archive/2026-09-17/feat/sp05-preview-exit` |
| `feat/sp05-preview-owner` | Yes | `archive/2026-09-17/feat/sp05-preview-owner` |
| `fix/sp05-deadline-and-review-diagnostic` | Yes | `archive/2026-09-17/fix/sp05-deadline-and-review-diagnostic` |
| `foreman/moriarty-ll-loop-20260912/implement/authority-fee-binding` | No | `archive/2026-09-17/foreman/moriarty-ll-loop-20260912/implement/authority-fee-binding` |
| `foreman/moriarty-ll-loop-20260912/implement/handoff-protocol` | No | `archive/2026-09-17/foreman/moriarty-ll-loop-20260912/implement/handoff-protocol` |
| `foreman/moriarty-ll-loop-20260912/implement/prover-wrapper` | No | `archive/2026-09-17/foreman/moriarty-ll-loop-20260912/implement/prover-wrapper` |
| `foreman/moriarty-ll-loop-20260912/implement/scope-reconciliation` | No | `archive/2026-09-17/foreman/moriarty-ll-loop-20260912/implement/scope-reconciliation` |
| `foreman/moriarty-ll-loop-20260912/misc/k-corpus-base` | No | `archive/2026-09-17/foreman/moriarty-ll-loop-20260912/misc/k-corpus-base` |
| `foreman/moriarty-ll-loop-20260912/plan/k-grounded-proposal` | No | `archive/2026-09-17/foreman/moriarty-ll-loop-20260912/plan/k-grounded-proposal` |
| `foreman/moriarty-ll-loop-20260912/plan/preview-constraint-freeze` | No | `archive/2026-09-17/foreman/moriarty-ll-loop-20260912/plan/preview-constraint-freeze` |
| `foreman/moriarty-ll-successor-20260913b/misc/integration` | No | `archive/2026-09-17/foreman/moriarty-ll-successor-20260913b/misc/integration` |
| `foreman/moriarty-orch-remediation-20260912/implement/runner-round-ownership` | No | `archive/2026-09-17/foreman/moriarty-orch-remediation-20260912/implement/runner-round-ownership` |
| `plan/mori-language-architecture-2026-09-13` | No | `archive/2026-09-17/plan/mori-language-architecture-2026-09-13` |
| `research/typescript-elm-unison-language-design` | Yes | `archive/2026-09-17/research/typescript-elm-unison-language-design` |

Recover any archived branch without changing main:

```sh
git fetch origin --tags
git switch -c recovered-work archive/2026-09-17/BRANCH_NAME
```

The delivery branch is `feat/lifecycle-corpus-delivery`. The recovered local corpus remains unfinished; K execution, authenticated lifecycle settlement and wider roadmap gates remain open.

Exact branch tips and review/verification records accompany this file.
