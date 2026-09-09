/**
 * DeFi category model: seven economic families plus the cross-cutting group.
 *
 * These identifiers organize the classification. They are not Moriarty source
 * syntax — no agreement ever names an `F2`.
 */

export type FamilyId = 'F1' | 'F2' | 'F3' | 'F4' | 'F5' | 'F6' | 'P' | 'X';

export type FacetName =
  | 'Execution'
  | 'Settlement'
  | 'Custody'
  | 'Legal dependence'
  | 'Collateral and solvency'
  | 'Oracles'
  | 'Authorization and mandate'
  | 'Price discovery';

/** All eight mandatory facets, in canonical order. Every instance carries a value on each. */
export const FACETS: readonly FacetName[] = [
  'Execution',
  'Settlement',
  'Custody',
  'Legal dependence',
  'Collateral and solvency',
  'Oracles',
  'Authorization and mandate',
  'Price discovery',
] as const;

export interface Family {
  id: FamilyId;
  /** Short label for the tab strip. */
  tab: string;
  /** Full name used in prose. */
  name: string;
  /** The economic service supplied. */
  fn: string;
  /** What this family excludes, and why the boundary sits there. */
  boundary: string;
  /** Where instances of this family typically sit on each of the eight facets. */
  facets: Record<FacetName, string>;
  /** The failure this category produces in practice, stated as a mechanism. */
  goesWrong: string;
  /** The language construct, type, policy or authority rule that answers it. */
  response: string;
  /** Pinned standards and papers behind this family's targets. */
  sources: string[];
}

export const FAMILIES: readonly Family[] = [
  {
    id: 'F1',
    tab: 'Exchange',
    name: 'Exchange and price discovery',
    fn: 'Convert one asset into another and, in doing so, produce a price.',
    boundary:
      'The exchange function is separate from the mechanism that implements it. Singleton accounting and hooks change the mechanism and its callback assumptions; they do not change the exchange function. Routing and aggregation are execution-facet properties layered on top, not a distinct family.',
    facets: {
      Execution: 'Atomic on-chain, or intent- and solver-mediated',
      Settlement: 'Same-domain, immediate',
      Custody: 'Pool-held for the duration of the swap',
      'Legal dependence': 'None',
      'Collateral and solvency': 'Reserves are the backing',
      Oracles:
        'Usually none for constant-function makers — the pool is the price; oracle-dependent for some venues',
      'Authorization and mandate': 'Per-transaction, bounded by slippage',
      'Price discovery': 'This is the family that produces it',
    },
    goesWrong:
      'Integer division decides who keeps the remainder. A swap that rounds the wrong way, or a share calculation whose zero-supply case is unguarded, transfers value silently and legally. Reserve safety is not implied by the constant-product identity: an output that empties a reserve satisfies the formula and destroys the pool.',
    response:
      'Rounding is a declared artifact rather than a side effect of division. A policy block names the unit, the derivation, the rounding direction, the disposition of the remainder, the comparison basis and the proof obligation. Reserve safety and the net minimum are named guards, and a failure is a named rejection rather than a silent adjustment.',
    sources: [
      'AMM literature §§2.3.2, 3.1, 3.3, 4',
      'Uniswap V4 core — singleton accounting and hook-modified mechanism assumptions',
    ],
  },
  {
    id: 'F2',
    tab: 'Credit',
    name: 'Credit and collateralized debt',
    fn: 'Create an obligation to repay, usually secured, and manage it through accrual, partial performance, default and discharge.',
    boundary:
      'Nominal debt is not a transfer. This is the distinction Moriarty is architecturally organized around: a credit system that models debt as a balance rather than an obligation cannot express partial repayment correctly.',
    facets: {
      Execution: 'On-chain, often multi-step',
      Settlement: 'Same-domain, but discharge and transfer are separate events',
      Custody: 'Collateral is encumbered, not surrendered',
      'Legal dependence': 'None on-chain; material for collateral backed by external claims',
      'Collateral and solvency': 'The defining facet',
      Oracles: 'Required — liquidation depends on an external price',
      'Authorization and mandate':
        'Borrower authority to draw and liquidator authority to seize are different mandates',
      'Price discovery': 'Consumed, not produced',
    },
    goesWrong:
      'A repayment reduces a balance without discharging the right obligation. Interest and principal are allocated by whichever subtraction the code happened to perform first. A withdrawal that cannot be funded erases the claim instead of rejecting. A refinance issues new debt without discharging the old.',
    response:
      'Debt<T> is a distinct type from Amount<T>. A payment is a transfer plus an allocation against a named obligation, and the allocation policy is explicit — AccrualFirst, PrincipalFirst or ProRata — with explicit none, floor or ceil conversion rounding. The source states the invariant directly: paying interest may not touch principal.',
    sources: [
      'Lending literature §§3.2–3.4',
      'Kotzer §§III-B, III-C, V-C',
      'Werner §3.2',
      'Gogol §IV-A3',
      'ERC-3156 — flash callback and repayment contract',
    ],
  },
  {
    id: 'F3',
    tab: 'Derivatives',
    name: 'Derivatives',
    fn: 'Payoffs that reference something else — a price, a rate, an event — rather than conveying ownership of it.',
    boundary:
      '"Synthetic" is used narrowly: a reference-dependent payoff without direct asset ownership. Broader historical usage that folds in wrapped, staking and monetary claims is preserved as a source translation, not adopted.',
    facets: {
      Execution: 'Continuous margin maintenance, not one-shot',
      Settlement: 'Periodic funding plus terminal settlement, on different clocks',
      Custody: 'Margin is posted and at risk',
      'Legal dependence': 'None on-chain',
      'Collateral and solvency': 'Margin adequacy and insolvency handling are the core risk',
      Oracles: 'Mandatory and adversarial',
      'Authorization and mandate': 'Position-holder mandate distinct from liquidator mandate',
      'Price discovery': 'Consumed; funding rates feed back into it',
    },
    goesWrong:
      'Funding sign inversion. Settlement convention ambiguity — whether a position settles at mark, index or last trade, and at which timestamp. An expiry sweep that cancels obligations already validly exercised. Insolvency recognized after the loss rather than as a state.',
    response:
      'Time and duration are distinct types from amounts, so an exercise date cannot be compared against a quantity. Obligations survive independently of the position that created them, so expiry cannot silently erase an exercised duty without violating an ensures clause. Observations are typed with provenance, freshness and domain. Bounds are declared on the agreement itself, so a derivative cannot become an unbounded subscription.',
    sources: ['Werner §3.5', 'Gogol §IV-B3'],
  },
  {
    id: 'F4',
    tab: 'Consensus claims',
    name: 'Consensus-position claims',
    fn: 'Claims whose value derives from participation in consensus: staking, liquid staking, restaking and shared security.',
    boundary:
      'Distinguish slashable allocation from ordinary leveraged lending or a token-lock reward. Restaking commits capital to additional security duties; that is a different exposure from lending the same capital.',
    facets: {
      Execution: 'Two-phase — request then completion — with a protocol-imposed delay',
      Settlement: 'Delayed, and sometimes queued',
      Custody: 'Delegated to an operator, whose authority persists',
      'Legal dependence': 'None',
      'Collateral and solvency': 'Slashing is loss allocation, not liquidation',
      Oracles: 'Reward and penalty accounting is internal but externally observable',
      'Authorization and mandate': 'Delegation is a mandate with its own scope',
      'Price discovery': 'Consumed',
    },
    goesWrong:
      'Integrations treat a rebasing balance as a fixed claim, or a fixed share balance as if quantity were entitlement. Exit requests are modelled as instantaneous, so the pending state — capital still slashable, no longer earning, claim not yet transferable — has no representation at all.',
    response:
      'The pending state is an object with an identity, not a gap between two states. A request creates a durable identified obligation, and partial or delayed completion consumes it without resetting it. Affine authority applies: a partially completed exit consumes its authorization and the residual cannot grow.',
    sources: ['Gogol §IV-A1', 'Pending-workflow analysis for exit lifecycles'],
  },
  {
    id: 'F5',
    tab: 'External claims',
    name: 'Tokenized off-chain claims',
    fn: 'On-chain representations of claims that are ultimately enforced off-chain.',
    boundary:
      'A representation does not establish a claim’s economic substance. A receipt is evidence of a deposit; the redemption right is a separate specification. Identity and compliance controls describe technical transfer policy and are not legal conclusions.',
    facets: {
      Execution: 'On-chain action, off-chain performance',
      Settlement: 'Split across domains with different finality',
      Custody: 'An off-chain custodian holds the underlying — the defining facet',
      'Legal dependence': 'Maximal. The claim is only as good as its enforceability',
      'Collateral and solvency': 'Issuer solvency is the risk',
      Oracles: 'Attestation is the oracle, and the attestor can lie',
      'Authorization and mandate': 'Issuance authority is institutional, not cryptographic',
      'Price discovery': 'External, often with a valuation-time and publication-time gap',
    },
    goesWrong:
      'The on-chain leg completes and is treated as settlement. The system has no way to represent "the token was burned and the wire has not arrived", so it represents it as done. Valuation time, publication time, invalidation and redemption liquidity collapse into one number.',
    response:
      'Every oracle and external effect has a named capability and an assurance boundary recorded in the deployable manifest. A typed observation carries source, feed, unit, timestamp, freshness, sequence, bounds and fallback — and the residual risk is stated beside it: the source can still lie. External contract calls are excluded from V0; external behavior cannot inherit Moriarty guarantees. A duty that depends on off-chain performance stays a duty until evidence of that performance exists.',
    sources: [
      'Gogol §§III-B, IV-A2',
      'ERC-3643 and ERC-7943 — transfer control',
      'ERC-8330 — subject-linked NAV observation boundaries',
      'Centrifuge protocol source — cross-domain fund infrastructure',
    ],
  },
  {
    id: 'F6',
    tab: 'Asset management',
    name: 'Delegated asset management',
    fn: 'Someone else allocates your capital: vaults, yield strategies, allocators and standardized accounting entry points.',
    boundary:
      '"Vault" is the most overloaded word in the vocabulary — a debt position, a custody container, an investment vehicle, a strategy adapter or a standardized accounting entry point. Always qualify it. A share is a unit in portfolio accounting or a priority-defined claim; it is not automatically legal equity.',
    facets: {
      Execution: 'Synchronous deposit and withdraw, or an asynchronous request lifecycle',
      Settlement: 'Immediate or queued',
      Custody: 'The vault holds; the strategy may re-delegate, producing nested exposure',
      'Legal dependence': 'Varies with the underlying',
      'Collateral and solvency':
        'Obligations to depositors can exceed realizable value under stress',
      Oracles: 'Valuation is the oracle problem in disguise',
      'Authorization and mandate':
        'The depositor authorized a policy, not each trade; the manager’s mandate has scope and limits',
      'Price discovery': 'Net asset value is a valuation, not a realizable price',
    },
    goesWrong:
      'The donation case: a direct transfer inflates assets without minting shares, and the first depositor’s rounding does the rest. Rounding in the depositor’s favour on one method and the vault’s on another, applied consistently, drains one side. An asynchronous claim processed twice because the request carried no residual amount. An unwind that reports the position closed while its debt persists.',
    response:
      'Direct-transfer and rounding accounting are complete rather than inferred: each conversion states its rounding direction and remainder disposition, so method-specific rounding is a declared property that can be checked. Request duties are persistent identified obligations, so Pending, Claimable and Claimed are distinct states and a residual amount survives a partial claim. Valuation roles are separated — conversion, preview, limit and execution semantics are distinct, so a preview cannot be mistaken for an authorization.',
    sources: [
      'ERC-4626 — methods and security considerations',
      'ERC-7540 — request lifecycle and requestRedeem',
      'ERC-7575 — external share-token topology',
      'Yield literature §§III-A Fig. 2, III-B, V-B',
      'Morpho MetaMorpho source — allocator to market dependencies',
    ],
  },
  {
    id: 'P',
    tab: 'Prediction',
    name: 'Prediction markets and event-contingent claims',
    fn: 'Claims indexed by the outcome of an event, resolved by evidence.',
    boundary:
      'Held separate from derivatives because resolution is evidentiary rather than price-mechanical: the question is not what the number is, but what happened and who says so.',
    facets: {
      Execution: 'Mint, trade, resolve',
      Settlement: 'After resolution, all at once',
      Custody: 'Collateral held against the full outcome set',
      'Legal dependence': 'Varies by jurisdiction and event type',
      'Collateral and solvency': 'A complete set must always be fully collateralized',
      Oracles: 'The resolution source is the entire trust model',
      'Authorization and mandate':
        'Who may resolve, and under what evidence, is the critical mandate',
      'Price discovery': 'Produces probability estimates',
    },
    goesWrong:
      'Split and merge are treated as token operations rather than conservation-preserving transformations of a claim set, so a merge can duplicate value. Resolution is accepted from a source whose authority was never modelled. An invalid or contested resolution has no rejection path, so the first answer wins.',
    response:
      'Split and merge are composition operators governed by a conservation property: split partitions work and claims, and join cannot duplicate resource. That is checkable against value conservation rather than trusted to a token contract. Resolution is a typed observation with a named capability, and evidence that does not meet the declared provenance, freshness and domain requirements rejects. A valid oracle signature does not establish economic truth.',
    sources: ['Contingent-claim targets', 'Werner §3.5'],
  },
  {
    id: 'X',
    tab: 'Cross-cutting',
    name: 'Cross-cutting: what makes the other seven composable',
    fn: 'Observation, authorization, governance, messaging and composition — the machinery every other category depends on.',
    boundary:
      'Not a leftover group. These are the targets that turn seven separate families into one classification, and they are where cross-category composition actually fails.',
    facets: {
      Execution: 'Defines what execution means for every other family',
      Settlement: 'Pending commitments and finality evidence',
      Custody: 'Route-specific, and stated per layer rather than assumed',
      'Legal dependence': 'Surfaces as declared assumptions rather than silence',
      'Collateral and solvency': 'Duties propagate across composed steps',
      Oracles: 'Typed observations with provenance, freshness and domain',
      'Authorization and mandate': 'Affine authority, gross accounting, enumerated recipients',
      'Price discovery': 'Route and solver choice sits here, not in a family',
    },
    goesWrong:
      'Atomicity claimed at one layer is lifted to another: an internal ledger batch succeeds and a bridge withdrawal is rolled back, and the model has no way to represent the difference. Governance is treated as outside the financial semantics, so positions change retroactively. A join duplicates a resource that a split had partitioned.',
    response:
      'Atomicity is defined per layer and per route, never lifted. Routes publish a custody and authority manifest before approval, and confidentiality is a named adapter profile with explicit disclosure and custody assumptions. Governance is a bounded administrative action under a fixed claim policy that cannot downgrade claims or reset accrued work. The five composition operators each carry their own authority rules, duty propagation, conflict semantics and fan-in behavior.',
    sources: [
      'Contradiction register — per-layer atomicity and custody',
      'Report reconciliation — five composition operators',
    ],
  },
] as const;
