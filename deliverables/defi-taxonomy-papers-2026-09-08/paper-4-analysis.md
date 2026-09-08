# Werner et al. (2022 v6), SoK: Decentralized Finance (DeFi)

Source: [SRC-0103 full PDF](../../.raw/captured/9079375c121a8033457a952b58b5dc2fb6f2ad9d94ba9ba247b1994b127c5b2c.pdf). arXiv:2101.08778v6, 15 September 2022, as printed on PDF page 1. Authors: Sam Werner, Daniel Perez, Lewis Gudgeon, Ariah Klages-Mundt, Dominik Harz, William J. Knottenbelt.

Coverage: all 17 PDF pages read, including references and Appendices A–C. Visually inspected Figure 1 (PDF p. 3), Figure 2 (p. 6), Figure 3 (p. 9), and Table 1 (p. 8). All locators below are **1-based PDF pages**, coinciding with printed page numbers here. No source experiment or incident was reproduced. Historical ecosystem counts and software limitations are source-period observations, not independently checked present-day facts.

Duplicate provenance: prior source SRC-0095 has capsule `/home/charl/Moriarty/.raw/captured/defi-language-design-2026-09-07/werner.json`, pointing to `https://arxiv.org/html/2101.08778v6`, with retrieved timestamp 2026-09-07T18:31:26.703935+00:00 and selected-section coverage §§2, 3, 5–7. This PDF is the **same work and version**, a new capture/format and fuller inspection, not an independent supporting source. Do not count the capsule and PDF as two papers.

## What is classified

The paper explicitly separates three axes: supporting primitives, operation-defined protocol families, and security properties. Its defining technical/economic distinction is atomicity and the attacker's exposure to intervening events, **not** code bugs versus financial mechanisms. An atomic oracle-price or governance exploit is “technical” under this terminology even when financially sophisticated. [Abstract/§1, PDF p. 1; §§5–6, pp. 6–9]

The ideal-form DeFi criteria are noncustodial, permissionless, openly auditable, and composable. These are descriptive ideals, not a theorem that real deployed protocols satisfy them. The underlying distributed ledger is taken as an input with consistency, integrity, and availability assumed. [§1, PDF p. 1; §2 opening, p. 2]

| Axis | Source categories | Role |
|---|---|---|
| Primitives | Smart contracts/transactions; keepers; oracles; governance | Execution, external activation, information import, and rule-change mechanisms |
| Protocol operations | On-chain exchange; loanable funds markets; stablecoins; portfolio management; derivatives; privacy-preserving mixers | Economic service families, not atomic instructions |
| Exploit origin | Implementation differs from intention; incidental ground-truth/estimate divergence is exploited; deliberate divergence is created | How an intended property can fail |
| Technical security | Smart-contract vulnerabilities; single-transaction attacks; transaction-ordering attacks | Exploit can complete atomically, including suitable transaction bundles |
| Economic security | Collateralization risks; MEV threats; governance/GEV; market/oracle manipulation | Strictly non-atomic strategies with cost and uncertain intervening reactions |

Sources: §§2–6, PDF pp. 2–11. Figure 1 (p. 3) visually places ledger, contracts/tokens, market/oracle/governance mechanisms, protocols, and composability in different roles; collateral/liquid markets/arbitrage/liquidations sit beside the mechanism/protocol arrangement. It is a conceptual ecosystem map, not a specification of all legal operations or a strict five-layer stack.

## Primitives and operation families

Smart contracts must express protocol rules, support conditional execution/bounded iteration, communicate in a shared execution context, and support atomic success/failure. Composition creates a large space of emergent interactions. The source also distinguishes ordinary transaction atomicity and bundle atomicity. Keepers are external actors incentivized to submit transactions because contracts do not spontaneously schedule themselves. Oracles import otherwise inaccessible off-chain data. Governance changes system parameters/code by agents or algorithms, with deployed examples involving dictators, multisigs, tokens, proposal thresholds, and quorum. [§§2.1–2.4, PDF pp. 2–3]

1. Exchange: order-book forms (individual/batch settlement) and AMMs. AMM reserve/pricing rules permit liquidity provision, swaps, fee claims, and liquidity redemption; arbitrage is part of ordinary price alignment. [§3.1, PDF pp. 3–4]
2. Loanable funds: pool deposits, collateralized borrowing, interest, liquidation by keepers, and uncollateralized transaction-scoped flash loans. Flash loans have legitimate arbitrage and collateral-swap uses; their existence is not itself a vulnerability. [§3.2, PDF p. 4]
3. Stablecoins: collateral, agents, governance, issuance, and oracles. Collateral can be exogenous, endogenous, or implicit. Custodial stablecoins are outside the paper's noncustodial stablecoin scope but may be assets used elsewhere in DeFi. [§3.3, PDF p. 4]
4. Portfolio management: smart-contract strategies allocate/rebalance across other protocols to earn interest/fees and token rewards, making dependencies intrinsic. [§3.4, PDF p. 4]
5. Derivatives: synthetic assets, futures, perpetual swaps, options; time dependence, collateral and liquidity matter. These are financial contracts with different lifecycle obligations rather than four interchangeable calls. [§3.5, PDF pp. 4–5; last clause analyst inference]
6. Mixers: pooled mixing or shielded transaction contents via zero-knowledge proofs, useful for privacy and capable of hiding illicit flows. [§3.6, PDF p. 5]

Appendix A Table 2 supplies example protocols for five operational families (mixers are omitted from that example table); it is illustrative, not exhaustive. Appendix C explains keeper competition in batch settlement and that fair settlement depends on competition and an ungamed scoring rule. [PDF p. 17]

## Intended properties and information boundary

Section 4 distinguishes on-chain ground truth G_ON, off-chain ground truth G_OFF, and on-chain estimates of off-chain truth Ghat_OFF. Contracts can access the first and third, not G_OFF itself. Intended Boolean properties are functions of information through the current state, and correct execution requires both code implementing its intended logic and estimates accurate enough to preserve the relevant property: P_i(G_ON,G_OFF) = P_i(G_ON,Ghat_OFF). This requires property-relevant accuracy, not literal equality of every measured number. [§4, PDF pp. 5–6]

The source defines three exploit origins: implementation/intent gap; exploiting incidental data divergence; and deliberately creating data divergence. An ordinary AMM/off-chain price discrepancy followed by arbitrage is expected behavior; using an AMM spot price as a dependent protocol's truth estimate can instead violate that dependent protocol's intended property. Incidental deviations without an exploiting actor may be system failures (footnote 3). It also allows that currency runs such as Terra may breach economic mechanism limits without being a formal-property exploit. Security and economic stability are related but distinct concepts. [§4 and footnote 3, PDF pp. 5–6; §6, p. 9]

## Technical and economic split, with important qualifications

Technical security is inability to atomically exploit value held by protocol/users. A sequence of calls can be one transaction or an atomic ordered bundle. Failure reverts the effect, although gas may be lost. Figure 2 draws a closed loop of contract calls with the return to the attacker in one execution. The framework includes:

- Smart-contract vulnerabilities: reentrancy, integer over/underflow, unit/scale errors, ordinary logic bugs. Reentrancy guards and finishing state updates before untrusted calls are discussed. [§5.1, PDF pp. 6–7]
- Single-transaction attacks, independent of another pending user transaction: instant governance takeover and spot-price manipulation through composable contracts. Governance token accumulation may be flash-funded; manipulating an AMM, exploiting a consumer, then unwinding is the source's single-transaction sandwich. [§5.2, PDF p. 7]
- Ordering attacks: displacement/generalized front-running and multi-transaction sandwiches targeting another transaction. Gas prices influence but do not guarantee order; a miner or atomically ordered bundle has stronger control than an ordinary competing sender. [§5.3, PDF pp. 7–8]

Economic security instead means economic infeasibility of strictly non-atomic exploitation. Figure 3 explicitly inserts changing market conditions between actions and marks profit with uncertainty. Markets/agents can react, capital is exposed, and failure has tangible costs. An instantaneous spot oracle can permit a technical exploit; a time-weighted price can force sustained manipulation but leave an economic exploit profitable. Similarly a governance timelock removes instant update structure without proving governance incentives safe. [§5.2, PDF p. 7; §6/Fig. 3, PDF pp. 8–9]

The source's “risk-free” is conditional on execution control/atomic rollback and excludes gas; do not upgrade it to no cost or guaranteed inclusion/profit. Number of transactions alone does not decide the category: many transactions in one guaranteed atomic bundle can remain technical, while a short cross-block strategy can be economic. The introductory language and §5.3 admit ordinary non-miner order competition, making explicit scheduler/atomicity assumptions essential in any use of the taxonomy. [§§5–6, PDF pp. 6–9; analyst critique]

## Economic risk detail

Collateralization is an incentive/safety mechanism, not unconditional solvency. Illiquidity, adverse price shocks, keeper unprofitability, and endogenous debt-asset prices can cause failed liquidation and deleveraging spirals. The Dai discussion distinguishes collateral value, debt demand, and feedback rather than merely checking a ratio at one point. [§6.1, PDF pp. 9–10]

MEV includes trading/liquidation opportunities and extends to consensus instability. Undercutting, time-bandit reorgs, and censorship that creates liquidation opportunities depend on intertemporal incentives. The ledger is assumed secure in §2, but §6.2 explicitly returns to feedback that can threaten this assumption. [§6.2, PDF p. 10; analyst interpretation of assumption boundary]

GEV (governance extractable value) captures governors preferring harmful extraction over long-term cashflows/stewardship, including risky collateral selection, takeover, minting, and loss of minority rights. The source treats MEV as interpretable as a special case of GEV in footnote 4. Governance-token borrowability complicates both instant attack structure and long-horizon incentive compatibility. Proposed defenses cited include user veto/Optimistic Approval; the paper does not prove all-governance safety. [§6.3 and footnote 4, PDF pp. 10–11; §7.2, p. 12]

Market manipulation must be distinguished from oracle corruption. An honest feed may report a manipulated underlying market. Sustaining a price imbalance costs capital and may lose to market reactions. A centralized oracle may lie for sufficient reward; decentralized reporting may align on a false answer because consensus about external truth is not itself objective verification. On-chain DEX prices cannot directly price off-chain fiat truth. [§6.4, PDF p. 11]

## Open research agenda and critical reading

The six research areas are composability, governance, oracles, MEV, program analysis, and anonymity/privacy. Composition includes wrapping and reuse of claims, dependencies between stablecoin stability and lending solvency, and borrowed governance rights. The source explicitly calls for both technical and economic analysis of connected protocols. It cites Tolmach et al.'s process-algebraic property verification but does not equate it with complete compositional security. [§§7.1–7.6, PDF pp. 12–13; reference [152], p. 16]

Program analysis distinguishes convenient automatic checks (commonly isolated contract patterns) from semi-automated verification of user-defined business properties. It asks for composition and financial semantics such as effects on token balances. The paper's MEV optimization hardness discussion is a conjecture and motivating reduction sketch, not a full general hardness/approximation theorem. It explicitly says Clockwork Finance's use of “economic security” is closer to this paper's technical/atomic MEV category. Terminology must be carried with the source. [§7.4, PDF p. 12; §7.5, p. 13]

Historical incident examples are illustrative. Table 1's caption says February 2020–March 2021 but visibly includes Cream dated 27 October 2021; §5 says Table 1 is in Appendix A, but the PDF places it on page 8 (Appendix A instead has Table 2 on page 17). Cite actual PDF locations and preserve the inconsistency. Do not derive prevalence rankings or audit effectiveness from its selected cases. Table 1 says figures are seizure amounts at incident time and ignore recovered funds. [Table 1, PDF p. 8; §5, p. 6; Appendix A, p. 17]

Appendix B provides three useful semantic examples: ERC-777 callbacks break dForce's collateral accounting; YAM multiplication fails to rescale and prevents governance quorum; bZx self-transfer erroneously increases balance and allows redemption without backing. These support resource/accounting/scale modeling more directly than a generic checklist. Their monetary figures remain source reports. [Appendix B, PDF p. 17]

Analyst critique: the atomic/non-atomic split is useful for choosing verification and economic models, but it is not a partition of underlying defect classes. Oracle and governance failures occur on either side depending on timing and control. A compilation-success theorem cannot establish intended economic properties that were never specified, and successful verification relative to Ghat_OFF cannot establish accuracy relative to unavailable G_OFF. “Non-custodial” ideals also coexist with substantial upgrade/governance authority in the surveyed designs. The framework should expose these assumptions rather than erase them.

## Language implications — analyst inference, not paper recommendations

| Candidate | Supporting source passage | Compiler/proof boundary | Runtime or external boundary |
|---|---|---|---|
| Amount<asset,unit,scale> with explicit conversion/rounding; conservation postconditions | §5.1 pp. 6–7; YAM/bZx Appendix B p. 17 | Reject invalid unit use; prove declared arithmetic/accounting properties | Prices/fees/realized balances still depend on adapter and execution semantics |
| Observation<source,time,block,unit> distinct from authoritative ownership/state | §4 pp. 5–6 | Track evidence kind and allowed property inputs | Freshness/deviation bounds can be checked; actual external truth and feed incentives remain assumptions |
| Explicit call/callback effects and protocol adapter invariants | §§2.1,5.1 pp. 2,6–7; dForce Appendix B p. 17 | Prove modeled state/callback behavior under stated code dependencies | Unknown or upgraded counterparties need validation and trust scope |
| Atomic transaction versus bundle scope, with scheduling rights declared | §§2.1,5–6 pp. 2,6–9 | Expose scope and derive rollback obligations | Compiler cannot make a sequencer honor a bundle, ensure inclusion, or supply cross-chain atomicity |
| Staged governance lifecycle and bounded authority | §§2.4,5.2,6.3 pp. 3,7,10–11 | Timelock, quorum, proposal authority and immutable limits can be specified/checked | Borrowed voting power, coalitions, bribery and stewardship incentives need external economic models |
| Collateral/debt/liquidation state machines and keeper contracts | §§2.2,3.2,6.1 pp. 2,4,9–10 | Check local solvency rule and allowed transitions against specified observations | Liquidity, keeper availability/profitability, price shocks and debt feedback remain economic/liveness assumptions |
| Separate verified transition properties from economic model obligations | §§6,7.1,7.4–7.5 pp. 8–13 | A property proof is relative to the model, bounded composition and assumptions | Incentive compatibility/equilibrium claims require identified strategies, utilities, market dynamics and evidence |

For a Moriarty action vocabulary, source categories motivate discovery questions (which transfers, claims, authority changes, observations, and deferred obligations implement each service) rather than directly prescribing six syntax forms. Treat exchange/lending/stablecoin/portfolio/derivative/mixer as service metadata; represent actual effects and lifecycle in a separate action model. This is design inference, not a claim the paper supplies such a grammar or that any existing implementation satisfies it.


---

Library identity: SRC-0103. Authority: primary descriptive research. Version/date: 2022-09-15. Reviewed 2026-09-08. Source transcription confidence high; language inference provisional, S2, not implemented or reproduced. [Source record](sources.json) · [Design comparison](DESIGN-IMPLICATIONS.md) · [Library](README.md).
