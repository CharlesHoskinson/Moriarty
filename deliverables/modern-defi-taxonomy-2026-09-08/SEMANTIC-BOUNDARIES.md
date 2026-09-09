# F. Semantic boundaries and risk overlays

This analysis proposes integration rules from the inspected specifications and papers. It is not a conformance test, audit opinion, reserve attestation or legal determination. Normative references point to the exact snapshots in the [atlas](STANDARDS.md); implementation examples carry separate evidence in [validation](VALIDATION.md).

## What the holder owns

| Position | Backing and obligation | Ordinary exit | Why a common token interface is insufficient |
|---|---|---|---|
| Wrapped custody claim | Custodian/bridge locks or controls a referenced asset; holder rights depend on custody and redemption policy | Burn/redeem through custodian or bridge; alternatively market sale | Supply equality is not custody proof, legal title or redemption enforceability |
| Liquid staking receipt | Participation in security capital, rewards and penalties, with operator/queue dependencies | Request withdrawal and claim, or sell receipt | Exchange-rate appreciation and a fixed token balance differ from rebasing; market discount is not necessarily accounting insolvency |
| Collateral-backed money-like debt unit | Issuance associated with borrower liabilities and collateral resolution | Monetary system's specific redemption/swap/settlement route | Token holder and CDP owner are different positions; the holder need not have a direct pro-rata claim on every collateral unit |
| Synthetic price exposure | Payout references a price or index under margin and settlement rules | Close, exercise or settle according to instrument | Price tracking need not give possession or redemption of the referenced asset |
| Investment vault share | Residual/priority-defined participation in assets and receivables under a mandate | Synchronous operation, queued claim or secondary sale | Accounting conversion says nothing by itself about liquid assets available now |
| External fund token | Claim, if any, against a fund/issuer/entity under governing documents | Eligible-holder redemption through fund processes; market sale may be restricted | Technical transfer eligibility is not legal compliance, ownership of portfolio securities or guaranteed enforcement |

Gogol's broad synthetic/pegged grouping helps identify dependence on a reference asset, but these positions differ in backing, redemption, cash flows, payoff replication and counterparty obligation. Those differences justify separate claim facets rather than one universal synthetic asset class.[^1]

## Vault semantics and operation bounds

ERC-4626 standardizes a tokenized accounting interface for one ERC-20 asset. It does not mandate a strategy, lending model, reserve quality or profit. Treat `Assets<asset-id>` and `Shares<share-id>` as different units. ERC-4626 permits implementations to restrict share transfers; an ERC-20 representation does not promise a liquid market.[^2]

| Question | ERC-4626 answer | Integration implication |
|---|---|---|
| Ideal conversion | `convertToShares` and `convertToAssets` estimate an ideal, caller-independent, fee-excluding conversion and round down | An estimate is neither an executable quote nor an external market oracle |
| Deposit exact assets | `previewDeposit` estimates no more shares than deposit should produce under the stated same-transaction conditions; includes deposit fees | Protect minimum shares in an adapter if the entry point has no user-specified bound |
| Mint exact shares | `previewMint` estimates no fewer assets than required; includes deposit fees | Protect maximum input assets; rounding direction differs from deposit output |
| Withdraw exact assets | `previewWithdraw` estimates no fewer shares than burned; includes withdrawal fees | Protect maximum shares; check ownership/allowance and withdrawal limit |
| Redeem exact shares | `previewRedeem` estimates no more assets than returned; includes withdrawal fees | Protect minimum output assets |
| Operation capacity | `maxDeposit/maxMint/maxWithdraw/maxRedeem` reflect limits, with specified conservative behavior | A preview generally ignores user/global limits; a favorable preview does not establish availability |
| Managed value | `totalAssets` reports managed assets under the standard's accounting requirements | Illiquid receivables can affect value without producing immediate cash |

Read the exact MUST/SHOULD/MAY clauses in the pinned profile; these summaries do not replace error/revert conditions. ERC-5143 proposes slippage-bound overloads but is Stagnant at the cutoff. Protocol-specific routers can add bounds without establishing ERC-5143 conformance.[^2]

**Rebasing versus exchange-rate accounting.** A rebasing token changes balances; a share wrapper may keep balances fixed while conversion changes. Both can represent similar underlying exposure, but a consumer caching balances or assuming transfer deltas may behave differently. Share decimals need not equal asset decimals. Donation-based changes, rounding, fee deductions and accrued liabilities require implementation-specific accounting; a naive `balanceOf(vault)/totalSupply` model is not universally correct.[^1][^2]

**ERC-7540 requests.** An asynchronous side moves assets/shares into the vault's control at request time, enters Pending, then Claimable, then a user/operator pull claim through the appropriate inherited entry function. Deposit request quantities use asset units; redemption requests use share units. One or both sides may be asynchronous; affected previews must revert. Request scheduling, the exchange rate applicable to fulfillment and pending-period yield policy need the implementation's rules. An async vault is not a synchronous ERC-4626 drop-in just because some selectors match. A router grouping calls into one transaction does not erase the request lifecycle.[^3]

**ERC-7575 topology.** A shared token can be external to individual asset entry points. Model the entry-point contract, share token and portfolio separately. An entry point does not thereby implement all ERC-20 share operations itself. Multi-asset entry is a capability, not proof that every asset is interchangeable at a fixed rate. ERC-7540 explicitly requires ERC-7575; it is not three mutually exclusive financial products. Registry `requires` metadata and body-level obligations must be read together: listing ERC-2771 does not mean every ERC-7575 integration has the same trusted forwarder.[^4]

**Cancellation and NAV.** ERC-7887 is a Draft request-cancellation extension with internal wording/event inconsistencies noted in its profile; cancellation is another lifecycle, not universal immediate undo. ERC-8330 is a Review proposal for subject-linked NAV snapshots, with correction/invalidation and freshness distinctions. An assertion of NAV is not reserve verification or proof of exit liquidity.[^5]

## Debt positions are not investment vaults

A collateral vault can be an account-bound debt record: collateral belongs to a position owner subject to debt and liquidation rules. An investment vault pools an asset portfolio and represents investor claims. A custody container can hold assets without investing them. A strategy adapter can expose a common entry point without being the ultimate asset owner. “Vault” names all of these in ordinary usage. Classify the obligation and representation before applying an interface label; neither a CDP nor an interest-bearing receipt must implement ERC-4626.[^1][^6]

For credit, separate collateral eligibility, initial margin, maintenance thresholds, accrued debt, liquidation trigger, liquidator incentive, sale proceeds, default, reserve loss and withdrawal liquidity. Fixed rate does not imply fixed maturity; open-term callable credit has a notice lifecycle. A non-liquidating term loan can be overcollateralized while exposing a lender to collateral loss at expiry. Flash liquidity instead relies on repayment or revert within a declared atomic scope and generally has a principal-based fee rather than time-accruing interest.[^6][^7]

## Atomicity and settlement guarantees

| Boundary | What can be guaranteed under the stated model | What it does not imply |
|---|---|---|
| One transaction on one execution state | Its defined state transitions commit or revert together, subject to chain execution rules | Free gas, inclusion, truthful inputs or permanent finality |
| Conditional ordered bundle | Atomicity only if the accepted relay/builder/chain mechanism actually provides the specified all-or-nothing behavior | Every private bundle or same-block sequence has that guarantee |
| Same-block inclusion | Shared block context/order can be observed | Joint rollback of independent transactions |
| Cross-chain intent | A scoped resolver/order/settlement protocol describes a fulfillment and reimbursement process | Synchronous shared state, automatic fair exchange or chain-wide atomicity |
| Final settlement | Depends on chain consensus and any challenge/bridge/escrow release conditions | A destination transfer alone proves the solver's origin reimbursement is final and spendable |

Werner calls atomic exploitation technical and strictly non-atomic exploitation with intervening exposure economic. Keep that definition when citing the paper; oracle manipulation and governance abuse can occur in either timing regime. Zhou adds actor information and ordering powers. Neither same timestamps nor the word “intent” supplies an atomicity theorem.[^8][^9]

ERC-7683 changed on13May 2026 from order/settler interfaces to an off-chain-called resolver that describes solver steps, variables and payments. It deliberately does not prescribe a universal escrow or settlement protocol. Its guarantees are conditional on explicit aborts and documented implicit assumptions. The previous draft's `open/openFor/fill` interfaces and deployed or documented consumers of them must be labeled by their older revision. See the [revision comparison](STANDARDS-EXECUTION.md) and the Across case.[^10]

## Trace the source of return

| Gross cash-flow source | Ultimate payer or economic activity | Deductions and exposure to retain |
|---|---|---|
| Lending interest | Borrowers using advanced purchasing power | Default, utilization, reserve share, servicing and manager fees |
| Exchange fees | Traders paying for execution/liquidity | Adverse selection, inventory changes, hedging/rebalance cost |
| Consensus rewards | Protocol issuance and transaction-related compensation under chain rules | Operator fees, penalties, slashing, withdrawal delay |
| Shared-security payments | Service users or incentive budgets paying for security commitments | Service failure/slashing conditions; subsidies versus recurring service revenue |
| Derivative funding or basis | Opposite position holders and financing/convergence economics | Changing funding sign, liquidation, margin and settlement risk |
| External-asset income | Borrowers, governments, businesses or portfolio assets outside the chain | Issuer/custodian/fund fees, taxes/restrictions as applicable, default, legal enforcement |
| Token emissions | Dilutive incentive allocation or treasury subsidies | Reward-token price and schedule; not automatically sustainable operating income |
| Cover premiums | Protection buyers | Claims, exclusions, loss timing, expenses and capital investment results |

Net investor return also includes changes in asset value and transaction costs. A wrapper changes representation; an aggregator changes allocation and expenses. Neither supplies an unexplained additional source of yield. Leverage changes exposure and financing costs rather than creating a new payer. Do not equate APY, gross revenue, a time-window return and an annualized forecast.[^1][^6]

## Compact risk matrix

These are analyst-proposed investigation predicates, not universal vulnerabilities or probabilities. A bad outcome may be ordinary investment loss, an accident or an attack; identify the violated duty and actor before assigning an incident label.[^9]

| Function/claim | Dependency and actor | Failure predicate | Needed evidence or control |
|---|---|---|---|
| MON reserve-backed unit | Issuer, bank/custodian, redeemer gate | Backing unavailable or holder cannot enforce/qualify for redemption | Dated governing documents, asset/liability evidence, eligible exit path |
| MON+CRE debt-issued unit | Oracle, governance, liquidator, monetary liquidity | Debt and recoverable collateral diverge; liquidation cannot recover debt | Valuation policy, timing, market-depth and default-waterfall analysis |
| EXC spot/LP | Token implementation, hooks, orderer, liquidity | Transfer or callback violates adapter assumptions; adverse ordering defeats bounds | Version-bound behavioral adapter, actual balance deltas, authorization and slippage predicates |
| CRE lending claim | Borrower, collateral feed, liquidator, allocator | Interest/default losses or withdrawal demand exceed usable liquidity | Account health and loss allocation plus stress/liveness assumptions |
| CAP subscription | Issuer, sale operator, escrow/arbiter | Allocation/proceeds/refund rule differs from contributor rights | Primary issuance terms, cap/table accounting, release/refund authority |
| DER margin/outcome | Price/event resolver, counterparty, insurance fund | Incorrect resolution or insufficient settlement capital | Observation provenance, dispute/maturity rules, margin/backstop scope |
| MGT share/nested share | Strategy, NAV provider, underlying vault, manager | Reported assets exceed recoverable exit value; losses/fees hidden by nesting | Dependency expansion, valuation/fee units, limits, queue and exit simulations |
| SEC stake/receipt | Validator/operator, chain, service slasher | Penalty, correlated service exposure or delayed exit defeats assumptions | Duties, slashability, delegation permissions, withdrawal lifecycle |
| RSK cover/tranche | Assessor, waterfall, loss-bearing capital | Trigger denied/misapplied or capital insufficient for accepted claims | Contractual/discretionary distinction, exclusions, priority and payout procedure |
| Any cross-chain claim | Solver, verifier, relayer, destination/origin chains | Destination action completes while reimbursement or refund remains blocked | Per-leg finality, message verification, timeout and capital-at-risk model |
| Any authorized position | Signer, account module, proxy admin | Replay, excess authority, module change or upgrade invalidates intent | Domain/nonce/expiry limits, signer validity, implementation and admin binding |

A flash loan belongs in actor capabilities and funding edges. The root defect is the violated invariant or unsafe dependency, such as a manipulable spot valuation or unchecked callback. Removing the flash-loan label does not establish economic safety. Similarly, transfer restrictions can implement an issuer's technical policy without proving the policy meets law.[^7][^9]

## Language-design implications: proposed obligations

The taxonomy recommends distinct typed records for asset/share/debt/request units; observations versus external truth; gross debits, net outputs and residual duties; callback effects; request/claim/default state transitions; operator/admin authority; and cross-domain finality. The first tests should attempt counterexamples: fee-on-transfer collateral, rebasing balances, donation/rounding, stale NAV, queued redemption counted as cash, nested leverage, a changed proxy implementation and a destination fill with unpaid origin claim. These are **specified-only test targets**, not new executed Moriarty tests. Existing four-paper and vault-report proposed tests remain separately recorded in their original dossiers.

## Sources

[^1]: Gogol, supplied2023 PDF, §§3.4–3.7,4.1,6–7, pp12–17,23–25; [full extraction and PDF](../defi-taxonomy-papers-2026-09-08/paper-1-analysis.md).
[^2]: [Pinned ERC-4626](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-4626.md), Specification and Security Considerations; [ERC-5143](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5143.md).
[^3]: [Pinned ERC-7540](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7540.md), request lifecycle, modified ERC-4626 behavior and security considerations.
[^4]: [Pinned ERC-7575](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7575.md), external share token and security considerations.
[^5]: [Pinned ERC-7887](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7887.md); [ERC-8330](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-8330.md). Status is as-of pinned cutoff, verified9SepUTC.
[^6]: Kotzer, suppliedPDF, §§III–VII pp3–13; [full extraction and PDF](../defi-taxonomy-papers-2026-09-08/paper-2-analysis.md).
[^7]: [Pinned ERC-3156](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3156.md), lender/borrower callback and repayment requirements.
[^8]: Werner v6, §§4–6 pp5–11; [full extraction and PDF](../defi-taxonomy-papers-2026-09-08/paper-4-analysis.md).
[^9]: Zhou, §§II–VI pp2–12, especially TableIII and §VI insight4; [full extraction and PDF](../defi-taxonomy-papers-2026-09-08/paper-3-analysis.md).
[^10]: [ERC-7683 May 2026 redesign](https://github.com/ethereum/ERCs/commit/96d110fbbe7042b061064833edaf8fa2cf5db195); [pinned current specification](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7683.md).
