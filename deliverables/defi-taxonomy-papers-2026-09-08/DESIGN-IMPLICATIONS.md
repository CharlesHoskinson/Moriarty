# DeFi taxonomies and the Moriarty language

Status: S2 research recommendations, 2026-09-08. Source observations are scoped to the four supplied PDF versions; the proposed rules and tests below are not implemented or proved by this study. [The source register](sources.json) pins the PDFs and related earlier captures. [Individual paper analyses](README.md#paper-library) provide exact page references and disagreements.

## Decision supported by the papers

Keep Moriarty's existing economic families plus mandatory facets. Add precision to the behavior and threat descriptions used to select language features. Do not turn the survey headings into a mutually exclusive product enum or into one Core constructor per product. The papers answer different questions: Gogol organizes mechanisms, assets and stakeholder risk; Kotzer resolves credit and aggregation variations; Werner distinguishes application categories and security by atomicity and economic exposure; Zhou classifies vulnerabilities, adversary capabilities and incidents.

This supports the current [M2+M3 and M5 direction](../../wiki/defiformal-taxonomy.md), rather than establishing a new primitive basis. None of the surveys proves that a small set of language constructors is complete. Their categories cannot certify coverage of Moriarty's 72 historical DeFi rows, 277 ACTUS fixtures or 32 ACTUS taxonomy dispositions.

## A useful taxonomy record

Use one record per modeled product/version and lifecycle action, with independently populated dimensions:

| Dimension | What the record must distinguish | Design consequence |
| --- | --- | --- |
| Economic purpose | Exchange, credit, derivatives, consensus claims, external claims, delegated management, event claims | Retain F1–F6/P as library/navigation labels. A product may participate in multiple families. |
| Mechanism | Liquidity pool, order book, collateralized issuance, strategy composition | Record operational behavior; do not force order books into pools to follow an inconsistent figure. |
| Asset and claim | Base asset, redeemable claim, pool share, debt claim, wrapped asset, rebasing or appreciation-based entitlement | Nominal asset identity, denomination, redemption rights and quantitative accounting must remain distinct. |
| Funding and timing | Pool or matched funding; fixed or open maturity; atomic borrowing or persistent debt | Select different state machines and proof boundaries. “Same block” does not imply atomic rollback. |
| Collateral and loss | Over/under/uncollateralized; liquidation policy; default; shortfall and recovery | Preserve residual liabilities and authorized loss allocation. Liquidation-free does not mean risk-free. |
| Rate and valuation | Utilization input, piecewise rate policy, accrual interval, price source, rounding and scale | Treat these as pinned financial definitions with explicit units and observation timing. |
| Composition and settlement | Local atomic composition, shared-state interleaving, asynchronous requests/messages, foreign settlement | Specify read/write conflicts, intermediate observations, finality, obligations and work budgets. |
| Authority and assumptions | Borrower, lender, liquidator, manager, oracle, governor, custodian, bridge or sequencer | Separate what Core checks, what ledger acceptance enforces, and what remains external. |
| Threat description | Adversary capability/knowledge, vulnerable layer, precondition, action sequence and loss outcome | Generate discriminating negative traces; a flash loan is a capability, not a vulnerability label. |

This is a proposed metadata contract, not accepted `.mori` syntax. [The crosswalk](taxonomy-design-crosswalk.csv) maps each requirement to existing DA targets and sprint owners. Unmapped survey details remain explicit rather than being dropped to improve a coverage count.

## Consequences for syntax, types and K

The surveys supply semantic requirements, not BNF. Finish the complete lexical/EBNF contract in SP02 using the existing [surface design](../defi-language-design-2026-09-07/LANGUAGE-DESIGN.md). Do not expand syntax before an independent financial fixture distinguishes the new behavior. The existing atomic grammar is a different profile; its ordered `set` semantics cannot silently become the successor's immutable `pre` and tentative `next` semantics.

The strongest type requirements are identified claims and obligations; distinct `Amount<Asset>`, `Shares<Vault>` and debt orientation; rates/prices with scales; separate observation time, accrual time and contractual maturity; and named authority for minting, burning, liability creation and seizure. A linear resource discipline alone cannot prove numeric conservation or solvency. A wrapped token's on-chain existence does not prove its backing or redemption availability.

K should expose the financial state needed to distinguish these cases: immutable pre-state, tentative writes, ordered effects, debt principal/interest/fees, claim shares, pending requests, encumbrances, authority, observations, residual duties and remaining work. Those are requirements for the successor design, not a claim that the cells or rules already exist. An accepted transition must bind the complete observation record through source/Core, K, evaluator, Compact and ledger correspondence. Equal final balances can conceal different debt, shares, fees or third-party effects.

Atomic failure must discard tentative financial changes across every leg admitted to the atomic operation. Persistent lending and asynchronous redemption instead preserve intermediate states and residual duties. Source transaction atomicity must not be generalized to a whole block, another chain, or an external custodian. Network fees and future availability remain outside local rollback guarantees.

## Proposed distinguishing fixtures

These are new test specifications, not test results or reproductions of paper incidents. The arithmetic examples use deliberately small illustrative parameters; no example asserts a deployed protocol's current formula.

| ID | Positive case and independent expectation | Distinguishing invalid case | Where it belongs |
| --- | --- | --- | --- |
| TX01 | A vault with 100 assets and 80 shares issues 20 shares for a 25-asset deposit under an explicitly selected floor policy. | Asset/share unit substitution; wrong rounding direction; zero-supply case that divides by zero; donation changes entitlement unexpectedly. | SP02 types; SP08 DA02/DA04/DA17; SP11 conformance |
| TX02 | Debt is principal 100, accrued interest 10. Payment 7 under interest-first allocation leaves principal 100 and interest 3. | Marking the position closed, erasing residual interest, or silently capitalizing it without liability authority. | SP03 residual-duty rules; SP07; SP08 DA06/DA09 |
| TX03 | Simple interest 100 × 10/100 × 30/360 equals 5 before the selected final rounding; accrue once over that declared interval. | Substituting block count for elapsed contractual time, applying the same interval twice, or moving across a rate kink using the wrong pre/post utilization. | SP07 time policy; SP08 DA06 |
| TX04 | Flash principal 100 plus fee 1 is repaid within one admitted atomic operation; all effects bind to that operation. | Repay only 100; repay 101 in a later transaction in the same block; fail the final leg after earlier tentative transfers. | SP03 rollback; SP08 DA08; SP09 acceptance |
| TX05 | With outstanding debt 100 and an illustrative 50% close limit, authorized partial liquidation repays 40 and preserves debt 60, with separately calculated collateral/bonus. | Repay 60 despite the limit; seize using a stale/wrong-unit price; erase shortfall or remaining collateral rights. | SP08 DA05–DA07; SP11 |
| TX06 | A redemption request moves Pending → Claimable → Claimed with exact remaining shares, controller and entitlement. | Illiquidity erases the request; double claim; unauthorized cancellation; exhausted work deletes a surviving right. | SP08 DA18; SP10 handoff |
| TX07 | Two calls on shared reserves compose under declared observation order and conflict rules. | Independent local checks reuse the same pre-state, accept incompatible effects, or inspect an intermediate state through a callback. | SP03 composition model; SP08 DA24; SP09/SP10 |
| TX08 | A signed observation has the selected feed, units, domain, freshness and sequence. | A validly signed but manipulable spot price is treated as a guarantee of economic truth. The model must expose the assumption and the attack can remain economically possible. | SP08 DA20; SP11 scenario analysis |
| TX09 | One outcome authorization accumulates gross spending, fees and net delivery across partial fills. | Refund resets gross allowance; reordered fills spend the same residual record; same endpoint balance hides an extra intermediate call or liability. | SP08 DA21; SP09/SP11 |
| TX10 | A parameter update or loss allocation uses the bound authorized version and preserves existing duties. | New governance settings retroactively reinterpret a signed obligation; temporary borrowed voting power is treated as durable consent; compromised custody is assumed impossible. | SP08 DA22; SP09 lineage; external-assumption review |
| TX11 | A bounded strategy withdraws a specified upstream claim, accounts for fees/slippage and deposits the authorized amount downstream. | Partial leg success is reported as atomic success; recursive strategy expansion exceeds bounds; zero-liquidation lending is misclassified as guaranteed principal preservation. | SP08 DA19; SP10 |
| TX12 | A foreign redemption/message creates a bounded pending state and requires the selected authenticated settlement observation before discharge. | A local proof or bridge message is treated as proof of foreign finality, reserve custody or actual payment; unavailable witness is equated to invalid commitment. | SP08 DA16/DA23; SP09/SP10 |

## Security claims must state their boundary

Zhou's layers and Werner's atomicity distinction should annotate each fixture; they do not justify a universal “safe DeFi” compiler flag. The compiler can reject unit errors, unknown effects and unbound authority. Core/K can define accounting, valid transition order, residual-duty preservation and finite work. The proof relation can bind those predicates to the concrete statement. Ledger acceptance must consume the correct predecessor and residual authority exactly once and enforce the actual effects. None of those steps establishes market liquidity, oracle truth, profitable strategy selection, honest governance, external custody, inclusion fairness or future settlement.

In particular, a price bound or time-weighted observation can reduce a specified attacker's feasible strategies, but cannot automatically remove economic risk. Solvency is a predicate over selected prices, assets and liabilities under assumptions. The four mandatory claims—ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance—remain mandatory and must name those assumptions. No taxonomy field disables them.

## What this adds to the roadmap

1. **SP01:** add the source-version and threat crosswalk to financial requirement selection; keep all existing denominators. The reports expose ambiguities to resolve with pinned lifecycle sources.
2. **SP02–SP03:** make shares, debt, time, rounding, authority and composition distinctions testable in the successor syntax/static contract and bounded K observations. First implement TX02 alongside the existing loan slice, so a surviving obligation is visible in source, Core, K and evaluator output.
3. **SP07–SP08:** pin protocol-specific lifecycle rules before implementing the proposed rate, liquidation, vault, strategy and administrative cases. These four surveys narrow the questions but do not replace normative/protocol fixtures.
4. **SP09–SP11:** use the same cases for proof/effect correspondence, concurrent residual-authority consumption, private continuation and full financial conformance. Record actual Preview transaction IDs only when an admitted network run occurs.

No sprint, grammar, K theorem, network milestone or existing financial coverage gate is closed by this intake. The concrete next development result remains a bounded financial trace with independent expected observations, not another orchestration system.
