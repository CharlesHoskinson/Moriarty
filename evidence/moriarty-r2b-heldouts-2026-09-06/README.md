# R2b held-out cases

Date: 2026-09-06. Scope: source-only design intake for the finite R2b atomic
outcome profile. These three cases were selected before Core expansion and were
not implemented, executed against Moriarty, or tested for conformance. The
machine-readable record is [manifest.json](manifest.json).

The intake uses existing local repositories and evidence only. Repository
content is read at the recorded commits even where a live checkout has moved.
The ACTUS case is an actual fixture event: `nam19` capitalizes interest at its
`IPCI` event, increasing `notionalPrincipal` from `5000` to
`5236.461356333502` with payoff `0`. The DeFi cases preserve the existing
72-target mapping: Maple row 10 supplies bilateral refinance of an existing
loan, and Huma Finance V2 row 34 supplies pending, claimable redemption with an
unfilled obligation carried forward.

| Case | Frozen source behavior | R2b atomic-profile probe | Disposition |
| --- | --- | --- | --- |
| `ACTUS-NAM19-IPCI-2013-07-01` | An `IPCI` event capitalizes accrued interest into notional. | Existing loan `settle` cannot create or capitalize a liability. | `needs-extension` |
| `DEFIFORMAL-MAPLE-F8-REFINANCE` | Proposed term hash plus counterparty acceptance changes an existing loan's terms and liability state. | Existing loan `settle` only clears the fixed local dues; it has no refinance proposal/acceptance or old-debt/new-debt result. | `needs-extension` |
| `DEFIFORMAL-HUMA-F23-PENDING-REDEMPTION` | Redemption is requested, batch processed, may roll forward, and is claimed separately. | The atomic profile has no pending success and loan `settle` cannot preserve a redemption claim across calls. | `needs-extension` |

Each record keeps independent `source_observation`, `atomic_profile_probe`,
`authority_result`, `financial_state_result`, and `lifecycle_result` fields.
Those are design dispositions, not calculated outputs. No result is inferred by
running a different case, and no corpus taxonomy is redesigned here.

The supplied intents report is supporting secondary input at lines 1570–1607
(refinance and first-class liabilities) and 1609–1652 (request, pending,
claimable, claim, and preservation of the live obligation). Its discussion of
current standards and backend behavior is not independently re-verified by this
local-only intake.

Material gaps remain: a bounded liability-change action and its authorization;
refinance proposal/acceptance and before/after debt identity; an asynchronous
request/claim lifecycle with residual authority and obligation conservation;
and adapter refinement plus a positive feasible witness for each eventual
extension.
