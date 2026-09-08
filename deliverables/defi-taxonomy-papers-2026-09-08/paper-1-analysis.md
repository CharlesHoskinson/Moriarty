# Gogol et al., June 2023 draft: taxonomy extraction and bounded design analysis

## Identity and reading coverage

Source: [SRC-0100 full PDF](../../.raw/captured/1c7211969624807b9a9aaf2308c4a80374816fb9c119d785b0d13437093cc192.pdf). Title: *SoK: Decentralized Finance (DeFi) - Fundamentals, Taxonomy and Risks*. Authors: Krzysztof Gogol, Christian Killer, Malte Schlosser, Thomas Bocek, Burkhard Stiller. First page gives June 2023 and a placeholder ACM DOI `10.1145/nnnnnnn.nnnnnnn`; final page says Received 1 June 2023. This is a 29-page manuscript submitted to ACM, not publication verification. Do not merge its version identity with the later 2024 arXiv capsule SRC-0085.

Read extracted text for every PDF page 1–29, including references pp. 25–29. The page JSON has 30 elements: the last is an empty extraction trailer, not an additional content page. Visually inspected all six figures: Fig. 1 p. 2, Figs. 2–3 p. 9, Fig. 4 p. 17, Fig. 5 p. 18, Fig. 6 p. 19; also visually inspected the full symbol matrix Table 6 p. 20. Read Tables 1–5 in extracted text; Table 3's symbols were not visually audited. No web lookup or empirical reproduction performed. Page locators below are PDF 1-based and match printed pagination.

## Selection scope and contribution

The source selects DeFiLlama categories above USD 5 billion TVL on 18 December 2022, claiming 89.09% of DeFi TVL, then adds DEX aggregators and algorithmic stablecoins because TVL does not apply to them (§1.1, p. 3, Table 1). Table 1: DEX 648 protocols/$15.86b; interest-rate lending 196/$10.83b; liquid staking 60/$8.63b; CDP crypto-backed stablecoins 60/$7.92b; bridges 36/$7.81b; yield 364/$5.3b; other 656/$7.18b. Perpetuals, described as $2b and below threshold, nevertheless receive a short subsection (§3.8, pp. 15–16). This is a historical sample and conceptual synthesis, not a completeness proof or current market census. The abstract's broader phrase is over 80%.

The strongest contribution is separation of four questions: what service a product offers; what mechanism it uses; what token is involved; and how the protocol is deployed. Risk is then conditioned on agent, protocol, and token (§4, p. 16; §7, p. 25). Its claim that every protocol belongs to one of three mechanism classes is the authors' organizing hypothesis; classification overlaps and exclusions remain.

## Source taxonomy, preserving hierarchy and facets

### Value proposition (Fig. 1, p. 2; §2.5, pp. 6–7)

- Trading → DEX; DEX aggregators.
- Lending → interest-rate protocols; crypto-backed stablecoins/CDPs.
- Asset management → yield farming; liquid staking.
- Interoperability → bridges/wrapped tokens (figure examples WBTC and Multichain).

These are user-service categories, not the same tree as the algorithm taxonomy. Liquid staking sits under asset management here and under synthetic issuance in Fig. 5; lending appears both as pooled lending and CDP minting.

### Token taxonomy (§4.1, pp. 16–17; Fig. 4, p. 17)

Technology: native blockchain tokens versus smart-contract-enabled tokens. The latter split by underlying value:

- Network value/token → governance and utility demand (figure example MakerDAO).
- Asset-backed → on-chain asset/LP participation tokens; synthetic assets; real-world assets.
- Share-like/share token → value from the holder's or issuer's work, illustrated with art NFTs/CryptoPunks.

Figure 4 calls the first asset-backed leaf “LP Tokens,” with aDAI and cDAI; text calls it “On-chain Asset” and illustrates AMM participation. Synthetic examples DAI, wBTC, stETH; RWA example USDT. The synthetic class is tied to another reference asset and discussed as derivative-like. The graph preserves the figure/text LP label difference rather than silently treating all receipt tokens as AMM shares. Native/contract is a technology distinction; backing, governance demand, and share-like value are economic facets and may overlap in actual designs.

### Algorithm taxonomy (§4.2, pp. 17–18; Fig. 5, p. 18)

The diagram begins with blockchain-related financial services, branching to DeFi protocols and CeFi systems.

DeFi protocols:

- Liquidity pools → DEX → AMM → constant function; concentrated liquidity. Also liquidity pools → interest-rate protocols → linear; nonlinear; kinked rates.
- Aggregators → DEX aggregators; yield farming. The figure draws separate boxes for simple lending, leverage borrow, liquidity provisioning without visible connectors to yield farming; §§3.4.1–3.4.3, pp. 12–13 establish those strategy relationships explicitly.
- Synthetic tokens → liquid staking → rebase; reward-bearing; dual. Also synthetic tokens → wrapped tokens/bridges. Also synthetic tokens → stablecoins → algorithmic → seigniorage; rebasing; and stablecoins → crypto-backed.

CeFi systems:

- Blockchain settlement → fiat-backed stablecoins; order-book exchange (examples 0x and dYdX).
- Off-chain settlement → centralized exchanges.

Figure 5 contains placeholder “XXX” examples for linear and nonlinear rates. The §4.2 introduction incorrectly says the algorithm tree is Fig. 6; its actual caption is Fig. 5. CLOB categorization is internally unstable: §3.1 pp. 7–8 contrasts peer-to-peer CLOB with peer-to-pool AMM and says off-chain order books *may* be CeFi, but §4.2.1 p. 17 mentions CLOB under a liquidity-pool discussion, whereas Fig. 5 places order books on the CeFi branch. Do not encode CLOB as necessarily a liquidity pool or necessarily CeFi.

### Mechanism detail omitted by a shallow taxonomy

AMM reserve curves (§§3.1.1–3.1.6, pp. 8–10; Figs. 2–3 p. 9): constant sum x+y=k (fixed price; can drain one reserve); constant product xy=k; weighted multiasset constant mean; StableSwap interpolating sum/product behavior via amplification A; concentrated liquidity restricts a position's active price interval and uses translated virtual reserves. Figures 2–3 are schematic reserve curves, not experimental results. Concentration improves capital efficiency at the cost of interval management and inactive/out-of-range exposure. The source's conservation-function regularity and price notation should not be copied into a formal semantics without mathematical review: its CPMM displayed price x/y requires a quote-direction convention, and the bare product is not globally concave though a normalized equivalent may be.

Pooled lending (§3.3, pp. 10–11): peer-to-pool funding, utilization U=B/L, borrower/supplier interest, overcollateralization, discounted liquidation. Linear rates α+βU; nonlinear polynomial-like rates; kinked piecewise rate with a steeper slope above Uopt. Supplier rates can include utilization and reserve factors. TVL is defined differently from a gross loan balance. Zero-liquidation loans add explicit tenor and a borrower option to redeem collateral before expiry (§3.3.5). Flash loans rely on atomic repayment, not collateral (§3.3.6); Algorithm 1 p. 12 composes a flash loan, NFT purchase, collateral deposit, and secured borrowing. The text alternates transaction/block terminology; use transaction atomicity as the intended mechanism and flag exact boundary in a language design.

Yield strategies (§3.4, pp. 12–13, Algorithms 2–4): simple lending deposits and accrues supply interest; leverage borrowing recursively deposits borrowed tokens as collateral before unwinding; AMM liquidity provision earns trading fees. Governance-token rewards are additional income; management/performance fees may apply. Strategies can be aggregators while reusing pooled lending and AMMs, so service taxonomy is not an exclusivity constraint.

Stablecoins (§3.5, pp. 13–14): fiat/commodity collateral off chain (treated as CeFi); crypto collateral on chain with CDP mint/burn lifecycle; uncollateralized algorithmic supply adjustment, split into rebasing balances and seigniorage mint/burn. Source acknowledges disagreement over whether rebasing assets count as stablecoins. CDP liquidation requires external transaction initiation; contracts do not spontaneously execute. CBDC is defined by central-bank issuer, can be wholesale/retail and centralized/decentralized or non-DLT (§3.5.5), so it is an issuer/infrastructure facet rather than necessarily DeFi.

Liquid staking (§3.6 pp. 14–15, Table 5 p. 15): rebasing changes quantity while targeting 1:1 value (stETH); reward-bearing changes exchange value per token (wstETH, mSol, rETH); dual tokens separate principal and reward (sETH2/rETH2). Rebasing is described as less compatible with other protocols; dual tokens fragment liquidity. Independent validator-admission facet: whitelisted reputation; permissionless credentials/minimum criteria; posted collateral. Source identifies slashing as well as depeg (§6.3.3 p. 25). A reward-bearing token's reference rate is time-varying, so a fixed 1:1 peg is not a universal specification.

Bridges (§3.7 p. 15): wrapped tokens; liquidity networks; hybrid. Wrapped-token verification subdivides external validators/federations versus light clients/relayers. Another cited trust taxonomy is trusted, “Binded” [sic], insured, trustless; do not silently normalize the unexplained “Binded” label. Liquidity-network and hybrid mechanisms are only sketched, not adequately specified. This is not evidence that every bridge mints a synthetic token.

Perpetuals (§3.8 pp. 15–16): no fixed expiry, collateral maintenance and liquidation; virtual AMMs discussed for counterparty-free pricing. No comprehensive margin/funding/settlement semantics are supplied. Vault/index tokens appear later (§6.2.2 p. 24; §7 p. 25); index value reflects underlying portfolio, whereas reserve-currency vault value is described as supply/demand driven.

### Network architecture (§4.3, p. 19; Fig. 6)

- Single chain: one deployment/chain (figure PancakeSwap).
- Multi chain: independent deployments on multiple chains; no value transfer required (figure UniSwap, Lido; text Aave and Uniswap).
- Cross chain: value crosses chains using interoperability (WBTC, Multichain).
- App chain: protocol-specific sovereign chain; security model may be called L1/L2/L3 (figure ThorChain, dYdX).

These labels should be represented as described historical categories. App-chain specialization is not logically disjoint from deployment multiplicity or cross-chain messaging. The text's descriptions of chain/security classes are coarse, not a proof of trust inheritance.

## Risks and stakeholders

Table 4 p. 6 gives service customers (profit/credit/rewards), liquidity providers (capital and fee/reward participation), arbitrageurs (price-efficiency profits), and governance users (design/maintenance and governance-token appreciation). Roles can be exercised by the same actor. Table 6 p. 20 crosses protocol family with role; its LP/YF column explicitly includes yield farming exposure. §7 p. 25 says aggregators accumulate risks of their underlying protocols and tokens.

Risk layering: economic DeFi risks (rug pull, slippage, impermanent loss, liquidation, depeg); infrastructure (blockchain attacks, transaction ordering/MEV); middle smart-contract layer (coding errors, unsafe external calls, access controls); application (oracle manipulation), §§5.1–5.4 pp. 20–23. Scalability/congestion and governance centralization are additional concerns (§5 p. 20). Algorithms 5–6 pp. 21–22 distinguish price and LP sandwich attacks; Algorithm 7 p. 22 composes flash borrowing, AMM price distortion, and borrowing against inflated collateral; Algorithm 8 p. 23 is a liquidity-withdrawal rug pull.

Visually recovered Table 6 using V=vulnerable, N=not vulnerable, D=depends. Column order: CLOB(C,LP,A); AMM(C,LP/YF,A); Interest(C,LP,A); Stable(C,A); Bridge(C,A); Staking(C,A); G.

| Risk | CLOB | AMM | Interest | Stable | Bridge | Staking | G |
|---|---|---|---|---|---|---|---|
| Rug pull | V,V,V | V,V,V | N,N,N | N,N | N,N | N,N | N |
| Slippage | N,N,N | V,N,V | N,N,N | N,N | N,N | N,N | N |
| Impermanent loss | N,N,N | N,D,N | N,N,N | N,N | N,N | N,N | N |
| Liquidation | N,N,N | N,N,N | V,N,V | V,N | N,N | N,N | N |
| Depeg | N,D,N | N,D,N | N,D,N | V,N | V,N | V,N | V |
| MEV | V,N,V | V,N,V | N,N,N | N,N | N,N | N,N | N |
| Blockchain attack | D,V,D | D,V,D | V,V,D | V,D | D,D | V,D | V |
| Smart-contract attack | V,V,V | V,V,V | V,V,V | V,V | V,V | V,V | V |
| Oracle attack | V,V,V | V,V,V | V,V,V | V,V | V,V | V,V | V |

This is the source's schematic matrix, not endorsed guarantees. Text/table tensions: Table 6 marks lending arbitrageurs vulnerable to liquidation, whereas §5.4.4 p. 23 describes service customers only; §5.1.1 p. 20 describes customer blockchain-attack impact as partial broadly, while Table 6 marks interest-rate, stablecoin and staking customers fully vulnerable. Table 6's N entries do not establish impossibility. Source asserts AMM slippage occurs only at AMM DEX (§5.4.2), all LP capital is lost in a blockchain attack (§5.1.1), governance collateral approval makes lending resistant to rug pulls (§5.4.1), and stable-value pairs can avoid impermanent loss (§5.4.3). These are overbroad without assumptions, especially where depeg is also explicitly recognized (§5.4.5). Keep these as attributed classifications, not compiler discharge rules.

## Bounded critique and language-design implications (analyst inference)

1. **Use orthogonal product descriptors.** Infer from Fig. 1 versus Figs. 4–6 that a descriptor should separate service intent, mechanism composition, asset/claim representation, chain topology, and actor role. A single enum such as `Lending | Staking | Bridge` would collapse distinctions the source exposes. This is a language proposal, not the authors' language design.
2. **Distinguish transferable asset, receipt, collateral position, and redemption claim.** LP receipts, CDPs, wrapped claims, rebasing quantities, exchange-rate-accreting rewards and split principal/yield need different balance and valuation semantics. Specify underlying reference, custodian/domain, quantity evolution, conversion function, redemption condition, and authority. Asset conservation may need shares/index accounting rather than raw wallet balances.
3. **Make state and time explicit.** Model debt accrual, utilization/kink parameters, collateral thresholds, concentrated price ranges, loan expiry, slashing and reward updates as state transitions. A peg is an economic objective, not a statically guaranteed equality. Model liquidation as an enabled externally triggered action and distinguish eligibility from eventual execution.
4. **Separate atomic scope from cross-chain scope.** Flash loan repayment composes within one transaction; bridge verification and finality have additional domains and delays. Annotate chain identity, settlement/finality assumptions, escrow/mint/burn obligations, and external attestations. Independent multichain deployment does not authorize shared synchronous state.
5. **Propagate exposure through composition.** Aggregators should retain references to underlying pools/tokens and actor positions. Infer risk conditions from those dependencies; do not inherit “safe” from an aggregator product label. Algorithm 7 specifically motivates checking valuation dependency loops: the collateral oracle must not be implicitly treated as an immutable trusted price if it reads a manipulable pool in the same atomic flow.
6. **Represent authority as capabilities and assumptions.** Issuance, mint/burn, custody, censorship, upgrades, governance votes, external validators and liquidation keepers require explicit authority/trust descriptions. The CeFi/DeFi boundary does not fit a universal Boolean trusted/untrusted declaration.
7. **Separate proofs from economic evidence.** Conservation and permission checks can be formal obligations; peg stability, oracle resilience, adequate arbitrage, liquidation liveness and market liquidity require explicit assumptions, adversarial models, or empirical evidence. This paper supports obligation discovery; it does not discharge those obligations.

Priority inferred language questions: Can an AST distinguish pool reserves from claim shares? Can it express time-varying redemption rather than 1:1 equality? Can an effect record oracle reads and valuation-sensitive borrowing in a composed transaction? Can topology and authority be checked independently of financial product names? These proposals are mapped in [the design crosswalk](taxonomy-design-crosswalk.csv); they are not claims about the existing implementation.

## Limits and source quality

The source is a draft with placeholder DOI, unresolved references (`[?]` in Table 2 and §3.3.2), placeholder examples, and a misnumbered figure reference. Bibliography [88], cited among bridges on p. 15, is a source-code summarization paper on p. 28 and has a placeholder DOI; that citation appears mismatched. Bibliography examples [96] and [106] also have suspiciously generic/mismatched URLs; no external bibliographic verification was done. Its 25% DEX aggregator volume in §1.1 p. 3 differs from 21.5% in §3.2 p. 10 without aligned snapshot explanation. TVL scope favors capital-intensive primitives and the authors themselves note that TVL is not usage or trading volume (§6.1.1 p. 23).

Taxonomy coverage is detailed for AMMs, lending rates, stablecoins and liquid staking, thin for bridges and perpetuals. No formal disjointness/completeness argument, reproducible classifier, calibrated loss model, or protocol-by-protocol validation dataset is provided in this PDF. References and claims about current examples belong to the 2023 draft's historical context. Index/reserve-currency descriptions and undercollateralized credit alternatives (§6.2.2–6.3.2 p. 24) indicate broader design space beyond the central tree.

## Graph conventions

The companion graph has 35 significant nodes. Detailed subtype leaves are preserved in node rationale attributes and this analysis; typed pairwise edges use the extraction schema's allowed relation vocabulary. Hierarchical `references` edges mean the source parent taxonomy explicitly includes/describes the child; they are not universal ontology subsumption. Graph provenance is remapped to the immutable captured PDF. Source_url is null for extracted concepts because the draft has no verified publication URL. Three explicit cited-work nodes preserve [77] yield aggregation, [120] synthetic assets, and [134] AMM DEX foundations; their URLs are transcribed citations, not independently acquired sources. One AMBIGUOUS edge preserves token receipt/synthetic overlap; an INFERRED semantic similarity edge relates balance rebasing in staking and stablecoins. Root may expand the subtype leaves if a larger graph is useful.


---

Library identity: SRC-0100. Authority: primary descriptive research. Version/date: 2023-06. Reviewed 2026-09-08. Source transcription confidence high; language inference provisional, S2, not implemented or reproduced. [Source record](sources.json) · [Design comparison](DESIGN-IMPLICATIONS.md) · [Library](README.md).
