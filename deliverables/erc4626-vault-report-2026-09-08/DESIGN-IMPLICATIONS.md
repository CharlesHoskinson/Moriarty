# Vault accounting, collateral value and Moriarty

Status: S2 research comparison, 2026-09-08. This supplements the [four-paper study](../defi-taxonomy-papers-2026-09-08/README.md). The supplied report is secondary synthesis; its citations and linked atlas were not provided as resolvable primary evidence. [The analysis](ANALYSIS.md) preserves its architectures, facets, result labels, questions and limitations. Recommendations below are proposed requirements/tests, not implemented features or reproduced incidents.

## The useful addition

The report separates four quantities: **accounting value, redemption value, market value and stressed liquidation value** (source lines 311–325). This sharpens the four papers' claim/observation distinction. A vault share can be accounted for correctly while being illiquid, temporarily unredeemable or unsuitable for a lender's collateral policy. Moriarty should require an explicit policy and assumption at each conversion from an asset/share quantity to debt capacity or recoverable proceeds.

The report's chain from custody through accounting, claims, valuation, borrowing power, liquidation and loss allocation (lines 246–265) is a useful dependency model. Each arrow names an additional obligation. None of the existing mandatory claims should be interpreted as proving an unmodeled arrow merely because a previous step passed.

## How its taxonomy fits

| Report architecture | Moriarty family/facet interpretation | Required distinction |
| --- | --- | --- |
| A: asset-management/yield vault | F6, with the underlying action families retained | Shareholder owns a claim; owning a share does not itself create borrower debt. |
| B: collateralized debt position | F2; position/collateral facets | Individual borrower liability differs from a fungible pooled receipt. |
| C: lending/credit vault | F2, possibly F6 allocation | Depositor claim and borrower liabilities coexist with loss allocation. |
| D: vault share used as collateral | F2 composed with the upstream family | Separate upstream claim accounting from downstream borrowing and seizure. |
| E: shared collateral/margin | F2/F3; authority and encumbrance facets | One account's collateral may support several duties; avoid duplicate release. |
| F: staking/restaking collateral | F4; settlement/loss facets | Rewards, slashing and delayed exit change entitlement and availability. |
| G: async/permissioned/RWA vault | F5 or F6 plus timing/custody/legal facets | A processing state or commitment does not establish external fulfillment. |
| H: multi-asset/nested/structured vault | F6 composition, with F1/F2/F3 as applicable | Common shares, different assets, allocation, tranches and dependencies need distinct policies. |

These are overlapping architecture patterns, not eight new Core constructors or proof of product coverage. T01–T11 are research facets spanning standards, accounting, valuation, leverage, liquidation, settlement, nesting, governance, attacks, verification and empirical structure (report lines 51–65). Preserve these identifiers in [the crosswalk](crosswalk.csv) alongside F1–F6/P and DA01–DA24; do not replace the existing 72-row denominator.

## Concrete semantic refinements

1. **Name quantity and valuation roles.** Keep shares and asset amounts nominally distinct. Model a valuation observation's purpose (accounting, redemption, market or stress), scale, time, source, validity policy and assumptions. Do not add an implicit cast from a vault conversion into `Price` or debt capacity.
2. **Separate queries from promises and authorization.** Conversion, preview, current limit and execution are different operations. A preview is not a user's slippage authorization. Signed outcome intent must bind the actual minimum net shares/assets, maximum gross spend, fees, recipients and liability limits, then prove refinement against concrete effects.
3. **Record effects across all accounting paths.** Direct token transfers, fees, rebases, rounding residuals and strategy reports can change balances or value without an ordinary deposit action. A cap must name the state it constrains. An authenticated but incomplete effect summary cannot establish exposure limits.
4. **Carry pending obligations and cancellation state.** Model request identity/controller, locked assets or shares, partial fulfillment, entitlement and residual work. The report's cancellation sketch is a proposal to investigate against a pinned extension, not authority to silently add a cancellation route or delete duties.
5. **Treat composition as a finite dependency graph with assumptions.** Nested shares can conceal repeated exposure to the same underlying collateral. Dependency depth, fan-in, valuations and work remain bounded; cycle handling and correlated loss need an explicit policy. Computing an accounting net exposure does not prove independent redemption liquidity.
6. **Make liquidation and loss allocation distinct transitions.** Auction, reserve absorption, redistribution and immediate sale have different effects and timing. The report's descriptions are discovery targets; source/version fixtures are still required before implementing each mechanism.

## Proposed supplement to the twelve paper-driven tests

These examples are intentionally small hypothetical models. They demonstrate desired distinctions and do not reproduce the named incidents or assert a deployed implementation's behavior.

| Test | Positive expectation | Distinguishing failure | Existing owner |
| --- | --- | --- | --- |
| VX01: donation and minimum shares | In a naive model, 1 share backed by 1 asset plus a donation of 100 gives 101 assets/share. A later 100-asset deposit would mint floor(100/101)=0; an intent requiring at least 1 share rejects before accepted effects. | Accept assets while satisfying a forged minimum-share claim, or assert a mitigation is safe for every future deposit sequence after one test. | SP08 DA17; SP09 intent refinement; SP11 |
| VX02: accounting versus collateral value | A donation changes accounting assets/share from 100/100 to 200/100. An explicitly selected external valuation/cap policy independently determines allowable debt. | Convert the doubled ratio directly into twice the debt allowance without the signed/pinned collateral policy. Donation cost remains part of any profitability analysis. | SP08 DA05/DA06/DA20; SP11 |
| VX03: preview, limit and execution | A hypothetical preview is 10 shares while the current deposit limit is zero: query results can coexist, but the deposit rejects under the selected limit policy. Same-state preview inequalities follow the method's contract. | Treat a nonzero preview as permission or liquidity guarantee; use an earlier preview as a future exact quote. | SP08 DA17/DA18/DA21 |
| VX04: asynchronous claim and race | A request preserves owner/controller and remaining amount through partial fulfillment and claim. Unsupported preview paths reject; any cancellation policy must specify when the claim wins or cancels. | Duplicate claim/cancel payout, loss of unfilled entitlement, or unapproved transfer of a pending right. | SP08 DA18/DA21; SP10 |
| VX05: repeated rounding and loss | A finite sequence records every rounding remainder, fee and injected strategy loss and preserves the selected quantitative invariant. | A local round-trip passes while repeated operations inflate a downstream ratio or a “monotone share price” invariant ignores allowed losses. | SP03 accounting; SP08 DA17/DA19; SP11 |
| VX06: nested exposure and privileges | Two allocation paths referencing one underlying position remain traceable to that shared exposure; material oracle/adapter/cap changes bind a versioned authority rule. | Count two wrappers as independently backed reserves, restore gross spending by netting, or reinterpret signed debt after an unbound upgrade. | SP08 DA19/DA22/DA24; SP09/SP10 |

## What was locally checked

The retained [ERC-4626 text](../../raw/sources/moriarty-intent-semantics-2026-09-03/erc-4626.md), sections `convertToShares`, `convertToAssets`, `previewDeposit`, `previewMint`, `previewWithdraw` and `previewRedeem`, distinguishes ideal conversions, fee-aware previews, limits and method-specific inequalities. The retained [ERC-7540 text](../../raw/sources/moriarty-intent-semantics-2026-09-03/erc-7540.md), Request Lifecycle and asynchronous deposit/redemption flows, distinguishes Pending/Claimable/Claimed and requires the corresponding async previews to revert. These existing local snapshots support a bounded comparison; this intake does not certify current standard status, every implementation or full conformance.

The report's normative matrix omits `previewRedeem`, an important complementary case: for the same-transaction conditions defined in ERC-4626, it must not overestimate assets delivered by redeem. The final fixture matrix must cover all four preview directions plus limits and failure conditions, rather than reproduce that omission.

The report also exposed an older library metadata error: the stored SRC-0084 capsule says creation date 2023-08-30, while both the retained ERC-7540 text and the HTML underlying that capsule say 2023-10-18. The source inventory correction preserves the immutable capsule and records why its metadata differs. No network lookup was needed for this correction.

## Evidence that remains missing

The report says it has an 80-source atlas, 14 results, 15 projects, three incidents and typed edges (lines 335–353). Only the Markdown report was supplied. Its sandbox links do not provide those files in this workspace; no CSV/JSON, BibTeX or ZIP was silently reconstructed. The visible project table has 14 rows, so the claimed 15-project artifact count remains unverified. Standard statuses, current product/version claims, preprint theorems and historical incident loss figures remain report claims until pinned primary evidence is inspected.

The source's short description of Werner as contract correctness versus economic security should be read with the inspected [Werner paper analysis](../defi-taxonomy-papers-2026-09-08/paper-4-analysis.md): its precise division concerns atomicity and economic exposure. Similarly, the report's suggested reproduction program is research input, not an instruction to launch a new infrastructure or exploit campaign. The next useful implementation remains a bounded source/Core/K/evaluator fixture within the existing sprints.
