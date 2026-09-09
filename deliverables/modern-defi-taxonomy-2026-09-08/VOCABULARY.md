# G. Vocabulary and annotated research atlas

Definitions below are controlled vocabulary for this recommended taxonomy. They translate ambiguous usage rather than claiming that every paper or protocol uses the terms identically. Normative interface meanings defer to the pinned [standard profiles](STANDARDS.md); historical usages retain the [paper crosswalk](PAPER-CROSSWALK.md) locators.

## Fifty essential terms

| # | Term | Definition and boundary |
|---|---|---|
| 1 | Financial function | The economic service and obligation supplied to a beneficiary; distinct from the implementation mechanism. |
| 2 | Mechanism | Rules for pricing, matching, allocation, accrual, collateral, issuance or payout that implement a function. |
| 3 | Category | A stable, defined classification with explicit inclusion/exclusion tests; not simply a marketing name. |
| 4 | Facet | An independently assignable descriptor, such as external backing, permissioned exit or queued redemption. |
| 5 | Implementation pattern | A recurring technical arrangement, such as singleton accounting, proxy delegation or a vault adapter. |
| 6 | Industry narrative | A grouping by market discourse, such as RWA or agentic finance; it may span many functions and trust models. |
| 7 | Protocol family | Related designs associated with a project; not a single immutable product or contract. |
| 8 | Protocol version | A specified code/design revision within a family; changes may alter economic and interface behavior. |
| 9 | Deployment | An instantiated program and configuration in a named execution domain, ideally bound to addresses and code identity. |
| 10 | Component | A bounded module responsible for part of a service: market, allocator, oracle adapter or settlement contract. |
| 11 | Market/vault instance | A concrete parameterized venue or portfolio, with its own assets, limits, authority and lifecycle. |
| 12 | Asset | A resource or transferable unit; its representation does not establish a claim's economic substance. |
| 13 | Position | A holder's scoped balances, rights, duties and exposures, which may include nontransferable debt or collateral state. |
| 14 | Claim | A right or protocol-defined entitlement to value or performance, naming an obligor and conditions where applicable. |
| 15 | Receipt | Evidence or representation of a deposit/position; redemption rights must be specified separately. |
| 16 | Share | A unit participating in portfolio accounting or a priority-defined claim; not automatically legal equity. |
| 17 | Vault | Ambiguous: debt position, custody container, investment vehicle, strategy adapter or standardized accounting entry point. Always qualify. |
| 18 | Collateral | Value committed to secure an obligation and subject to its release or loss-resolution rules. |
| 19 | Backing | Assets, capital or obligations supporting a claim; backing quality and enforceability are additional questions. |
| 20 | Reserve | Held resources intended to meet redemption, liquidity or loss obligations; reserve balances and liabilities need aligned scope. |
| 21 | Synthetic | Narrow usage: reference-dependent payoff without direct asset ownership. Gogol's broader historical usage includes wrapped/staking/monetary claims; retain that source translation. |
| 22 | Wrapped asset | A representation linked to an underlying asset or claim through custody/conversion rules; wrapper trust and redemption are explicit. |
| 23 | Stablecoin | Money-like token targeting a reference value; issuer, reserve, peg, redemption and yield policies are independent dimensions. |
| 24 | Peg | A target price relationship supported by mechanisms and incentives; not a guaranteed accounting equality. |
| 25 | Redemption | Exercising an entitlement against an issuer/vault to obtain specified value; different from selling to a third party. |
| 26 | NAV | Valuation of net assets for a scoped entity and time; may differ from realizable cash or stressed liquidation proceeds. |
| 27 | Rebasing | Updating account quantities through balance/index rules; distinct from a fixed share balance with changing conversion rate. |
| 28 | Yield | Return flow or rate attributed to a source; specify payer, horizon, gross/net basis, risk, compounding and subsidies. |
| 29 | APY | An annualized compounding measure under stated assumptions; not a guarantee or a synonym for a 30-day observation. |
| 30 | Principal | Contract-defined capital denomination or repayment base; strip naming alone does not make it guaranteed money. |
| 31 | Principal/yield strip | Claims separating principal-like maturity value and future yield; preserve underlying redemption, negative-yield and timing rules. |
| 32 | Liquidity | Ability to transact, borrow or exit at a time and cost; distinguish market depth, funding availability and redemption capacity. |
| 33 | TVL | A provider-defined aggregation of locked value; duplicated claims, collateral reuse and valuation methods limit comparisons. |
| 34 | Leverage | Economic exposure financed or supported beyond unencumbered capital, with financing and loss-amplification effects. |
| 35 | Utilization | A version-specific ratio of borrowed/used to available or supplied capital; denominator and accounting must be declared. |
| 36 | Liquidation | Enforced conversion or reassignment of collateral/positions under a risk or default rule; eligibility is not execution. |
| 37 | Default | Failure or contract-defined inability to meet a duty; may lead to forfeiture, restructuring, loss allocation or enforcement. |
| 38 | Tranche | A priority/payoff-defined claim against common exposures, including explicit seniority or first-loss treatment. |
| 39 | Strategy | A user's or manager's sequence/policy of allocations and actions across protocols; distinct from the underlying protocol types. |
| 40 | Aggregator | A component combining routes, venues or strategies; its dependencies and income sources remain visible. |
| 41 | Intent | A scoped desired outcome and authorization interpreted by an execution system; the word alone supplies no atomicity or replay protection. |
| 42 | Solver | An actor selecting or fulfilling execution steps, often committing capital and later collecting payment under protocol rules. |
| 43 | Oracle | A mechanism importing or deriving observations for consumers; retrieval, signature verification and external truth are different predicates. |
| 44 | Settlement | Completion of specified delivery/payment duties within a named domain and finality model; several legs may settle at different times. |
| 45 | Atomicity | Defined changes commit or revert together under an execution mechanism; distinguish transaction, conditional bundle and multi-chain boundaries. |
| 46 | Composability | Ability to combine components while respecting their behavioral contracts and assumptions; ABI agreement is only one prerequisite. |
| 47 | Finality | The relevant assurance that a recorded state will not be reversed, including chain, challenge and bridge conditions. |
| 48 | Liquid staking | Transferable representation of consensus-capital exposure; rewards, penalties, operator authority and withdrawal conditions persist. |
| 49 | Restaking/shared security | Capital committed to additional security duties; distinguish slashable allocation from ordinary leveraged lending or token-lock rewards. |
| 50 | Conformance | Satisfaction of a specific standard revision's requirements by a scoped implementation under a stated test or argument; not implied by branding. |

Definitions1–20 and34–50 are analyst normalization informed by the four papers and validation cases. Definitions21–33 preserve the token, financial-accounting and measurement distinctions analyzed in the [semantic guide](SEMANTIC-BOUNDARIES.md). Interface terms in entries17,25–27,31,41–47,50 additionally defer to the atlas's normative clauses and revision boundaries.

## Twenty annotated resources beyond the attachments

These resources were used for specific evidence, not selected by popularity. Primary specifications establish intended behavior; technical documentation establishes stated design; code establishes available implementation. Deployment and economic activity require additional evidence. Access and version pins are in the source manifest and profiles.

| # | Resource | Contribution and placement | Reading purpose |
|---|---|---|---|
| 1 | [EIP-1: EIP Purpose and Guidelines](https://eips.ethereum.org/EIPS/eip-1) | Standards process, categories and status vocabulary | Read before interpreting Final, Draft, Interface or Core |
| 2 | [ERC-20](https://eips.ethereum.org/EIPS/eip-20) | Base balances, allowances and optional metadata | Separate a transferable unit from an economic claim |
| 3 | [ERC-165](https://eips.ethereum.org/EIPS/eip-165) | Interface discovery with explicit detection behavior | Understand why a selector declaration is not behavioral certification |
| 4 | [ERC-4626](https://eips.ethereum.org/EIPS/eip-4626) | Asset/share accounting, previews and operation limits | Start the vault portion with precise units and bounds |
| 5 | [ERC-7540](https://eips.ethereum.org/EIPS/eip-7540) | Pending/claimable request lifecycle | Identify where synchronous integrations fail |
| 6 | [ERC-7575](https://eips.ethereum.org/EIPS/eip-7575) | External share token and asset entry points | Model topology separately from the portfolio's financial function |
| 7 | [ERC-3156](https://eips.ethereum.org/EIPS/eip-3156) | Flash callback and repayment contract | Distinguish atomic purchasing power from persistent credit |
| 8 | [ERC-5115](https://eips.ethereum.org/EIPS/eip-5115) | Standardized-yield wrapper proposal, still Draft | Study yield representation without assuming all Pendle components conform |
| 9 | [ERC-7683 and revision history](https://github.com/ethereum/ERCs/commits/master/ERCS/erc-7683.md) | Resolver redesign and earlier order/settler boundary | Read old and new versions before assigning adoption evidence |
| 10 | [EIP-712](https://eips.ethereum.org/EIPS/eip-712) | Typed structured-data signing; Interface category | Recognize that readable typed data does not supply application nonces or economic authority limits |
| 11 | [ERC-1271](https://eips.ethereum.org/EIPS/eip-1271) | Contract-based signature validation | Follow state-dependent signer validity beyond EOA recovery |
| 12 | [ERC-4337](https://eips.ethereum.org/EIPS/eip-4337) | UserOperation/account-abstraction entry and validation | Separate account execution, paymasters and economic product semantics; bind code to revision |
| 13 | [ERC-3643](https://eips.ethereum.org/EIPS/eip-3643) | Identity/compliance-related token controls | Describe technical transfer policy without making legal conclusions |
| 14 | [ERC-7943](https://eips.ethereum.org/EIPS/eip-7943) | Common external-asset transfer/control interface | Compare its2026 directional checks with older drafts and with 3643's broader architecture |
| 15 | [ERC-8330](https://eips.ethereum.org/EIPS/eip-8330) | Review proposal for subject-linked NAV observations | Distinguish valuation time, publication time, invalidation and redemption liquidity |
| 16 | [Morpho MetaMorpho source](https://github.com/morpho-org/metamorpho) | Lending allocation wrapped in ERC-4626, with governance and liquidity constraints | Trace allocator→market dependencies and V1/V1.1 differences from the pinned case |
| 17 | [Centrifuge protocol source](https://github.com/centrifuge/protocol) | AsyncVault, share-token and cross-domain fund infrastructure | Trace request lifecycle while keeping legal fund identity/deployment binding separate |
| 18 | [Across contracts](https://github.com/across-protocol/contracts) | Escrow, fills and old ERC-7683 interface explicitly deprecated in pinned source | Study solver capital-at-risk and revision-scoped compatibility |
| 19 | [Uniswap V4 core](https://github.com/Uniswap/v4-core) | Singleton pool accounting, hooks and ERC-6909-style claims | Understand why modular execution changes mechanism/callback assumptions rather than the exchange function |
| 20 | [Zhou authors' incident repository](https://github.com/Research-Imperium/SoKDeFiAttacks) | Evolving incident dataset tied to the supplied paper and later research | Follow forward research leads; do not silently substitute updated counts for the paper's2018–2022 corpus |

The first 19 entries are specifications or source systems inspected directly or via the case/atlas evidence. Entry 20's project record was inspected for provenance and forward references; its updated incident dataset was **not** remeasured or relabeled. Mutable URLs here are discovery links; use the pinned atlas/case URLs for reproducible claims.

Suggested sequence: four supplied papers in order P4→P1→P2→P3; then resources 1–3;4–6;7–8;10–12;9and18;13–15and17;16and19;20for further security research. This moves from financial objects to interfaces and then to difficult compositions. The research does not require reading every standard numerically.
