# B. Canonical category reference

These are recommended research categories, not historical paper labels or accepted Moriarty syntax. Classification is multi-label at an explicitly named component/position scope. Standards listed here are capability candidates; the crosswalk records their role and does not assert deployment conformance. Representative labels link to the source-scoped benchmark in [F](VALIDATION.md); uncovered design cells are identified explicitly.

## FIN-MON — Money, payments and escrow

Create money-like liabilities or discharge/secure payment obligations.

**Parents:** Financial function root. **Neighbors:** FIN-CRE, FIN-EXC.

**Include when:** A unit targets monetary use, or a service undertakes payment delivery as its financial purpose. **Exclude/redirect when:** Incidental transfers inside another function do not add a payment-service label. Trading the unit is exchange; external backing is a facet.

**Mechanisms:** issuance/redemption; conditional transfer; streaming. **Assets/positions:** money-like liability; payment entitlement.

**Examples:** Maker Vat/Dai; Sablier Lockup; Across SpokePool deposit (trade leg). **Relevant capabilities:** ERC-20, ERC-3643, ERC-7943, ERC-721, ERC-2612, ERC-3009, ERC-1271, ERC-5164.

**Dependencies:** reserves; custodians; peg liquidity; governance; payer funding; cancellation authority; clock; transfer behavior; arbiter; timeouts; counterparty fulfillment; finality.

**Foundation:** P1 §3.5 pp13–14; P4 §3.3 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-EXC — Exchange and trade execution

Exchange existing exposures and organize price discovery or execution for traders.

**Parents:** Financial function root. **Neighbors:** FIN-CAP, FIN-DER.

**Include when:** A service matches, prices, routes or settles a trade. **Exclude/redirect when:** Primary capital raising adds CAP; writing a derivative adds DER.

**Mechanisms:** AMM; order book; RFQ; batch auction; solver routing. **Assets/positions:** spot balances; LP inventory claim; execution order.

**Examples:** Uniswap V3 pool; Osmosis pool; CoW settlement; Across SpokePool. **Relevant capabilities:** ERC-20, ERC-721, ERC-6909, ERC-7683, EIP-712, ERC-1271, ERC-2612, ERC-3009.

**Dependencies:** price impact; adverse selection; ordering; token behavior; solver competition; authorization; venue adapters; chain finality.

**Foundation:** P1 §3.1 pp7–10; P4 §3.1 pp3–4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-CRE — Credit and financing

Advance purchasing power subject to repayment or agreed collateral/default resolution.

**Parents:** Financial function root. **Neighbors:** FIN-MON, FIN-MGT, FIN-DER.

**Include when:** A lender or issuer creates a repayment obligation. **Exclude/redirect when:** A token price target alone does not imply credit; allocating to loans adds MGT.

**Mechanisms:** inventory lending; debt issuance; margin; underwriting; liquidation. **Assets/positions:** receivable; debt position; lender share.

**Examples:** Aave V3 market; Morpho Blue market; Maple loan; Maker Vat/Dai; Aave V3 flashLoan; Uniswap flash behavior. **Relevant capabilities:** ERC-20, ERC-4626, ERC-3525, ERC-3475, ERC-7092, ERC-3156.

**Dependencies:** oracle; underwriting; collateral sale liquidity; default waterfall; collateral appraisal; mint authority; liquidation; monetary feedback; callback; initiator identity; fee; approved asset; atomic scope.

**Foundation:** P2 §§III–V pp3–8; P4 §3.2 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-CAP — Capital formation

Raise resources for an issuer, venture or collective through primary issuance or contributions.

**Parents:** Financial function root. **Neighbors:** FIN-EXC, FIN-CRE, FIN-MGT.

**Include when:** Resources pass to an issuer/project under a primary funding arrangement. **Exclude/redirect when:** Secondary trading is EXC; passive treasury allocation is MGT; ordinary incentive distribution alone is neither.

**Mechanisms:** subscription; issuance auction; bonding curve; conditional crowdfunding. **Assets/positions:** newly issued ownership/debt/utility claim; contribution claim.

**Examples:** Balancer V2 liquidity bootstrapping use; Coverage design; no measured deployment asserted. **Relevant capabilities:** ERC-20, ERC-3525, ERC-3475, ERC-3643, ERC-1155.

**Dependencies:** issuer authority; allocation policy; legal rights; proceeds custody; beneficiary; adjudication; deadline; project delivery.

**Foundation:** P1 Fig1 p2 limited coverage; analyst extension tested by LBP case; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-DER — Derivatives and contingent exposure

Create a payoff determined by prices, rates or events rather than ownership of the referenced asset.

**Parents:** Financial function root. **Neighbors:** FIN-CRE, FIN-RSK, FIN-EXC.

**Include when:** An agreed payoff references a variable or event and has margin, settlement or exercise rules. **Exclude/redirect when:** A fully redeemable custody receipt is not automatically a derivative; protection responding to defined loss also has RSK.

**Mechanisms:** futures/perpetual funding; options; outcome resolution; payoff tranching. **Assets/positions:** margined position; option; outcome share; rate claim.

**Examples:** Drift perpetual market; Opyn Gamma option design; Gnosis Conditional Tokens; Pendle PT/YT market. **Relevant capabilities:** ERC-20, ERC-1155, ERC-3525, ERC-165, ERC-5095, ERC-5115.

**Dependencies:** oracle; funding; insurance/backstop; liquidator and liquidity; option writers; premium; margin; exercise window; settlement data; resolver; event wording; challenge process; collateral; underlying yield asset; maturity; negative yield; index/oracle.

**Foundation:** P1 §3.8 pp15–16; P4 §3.5 pp4–5; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-MGT — Asset and portfolio management

Allocate or manage capital for a mandate with returns passed to participants.

**Parents:** Financial function root. **Neighbors:** FIN-CRE, FIN-EXC, FIN-SEC.

**Include when:** An allocator/strategy controls portfolio exposure or its recurring management. **Exclude/redirect when:** A container or ERC-4626 ABI alone supplies no mandate; operating the underlying market is a separate function.

**Mechanisms:** strategy execution; rebalancing; curated allocation; index tracking. **Assets/positions:** portfolio share; segregated managed position.

**Examples:** Morpho MetaMorpho; Yearn V3 vault; Centrifuge fund vault; Set portfolio design; Arrakis liquidity vault design. **Relevant capabilities:** ERC-4626, ERC-7540, ERC-7575, ERC-3643, ERC-7943, ERC-721.

**Dependencies:** underlying venues; allocator; caps; fees; unwind liquidity; valuation; custody; manager; tracking error; external rights; venue hooks; adverse selection; rebalance execution.

**Foundation:** P2 §VII pp11–13; P4 §3.4 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-SEC — Consensus and shared-security capital services

Commit capital to consensus or other security services in exchange for rewards with specified duties and penalties.

**Parents:** Financial function root. **Neighbors:** FIN-MGT, FIN-CRE, FIN-RSK.

**Include when:** Capital backs validator/operator duties or is delegated to such exposure. **Exclude/redirect when:** Node software alone is a supporting capability; ordinary collateralized borrowing is CRE even if collateral is a staking receipt.

**Mechanisms:** staking delegation; liquid receipt issuance; restaking allocation. **Assets/positions:** stake; liquid staking claim; restaked security exposure.

**Examples:** Ethereum validator deposit; Lido staking pool; Lido stETH/wstETH; EigenLayer allocation/delegation. **Relevant capabilities:** ERC-20, ERC-721.

**Dependencies:** validator; chain rewards; slashing; exit queue; operators; share accounting; queue; secondary liquidity; service rules; operator; slashing authority; collateral reuse.

**Foundation:** P1 §3.6 pp14–15; contemporary case extension; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-RSK — Protection and loss allocation

Transfer specified loss exposure or assign differentiated loss-bearing obligations to capital.

**Parents:** Financial function root. **Neighbors:** FIN-DER, FIN-CRE, FIN-MGT.

**Include when:** A cover, guarantee, tranche or backstop defines who absorbs a loss and on what trigger. **Exclude/redirect when:** Ordinary investment loss is not a protection service; a generic price bet is DER.

**Mechanisms:** mutual assessment; parametric payout; subordination; first-loss fund. **Assets/positions:** cover entitlement; junior/senior claim; guarantee.

**Examples:** Nexus Mutual cover; Centrifuge tranche design; Drift insurance fund. **Relevant capabilities:** ERC-721, ERC-20, ERC-3525, ERC-7575.

**Dependencies:** claims authority; exclusions; capital sufficiency; dispute process; underlying default; valuation; waterfall implementation; fund adequacy; governance; deficit measurement; recapitalization.

**Foundation:** P3 threat overlay distinct from function; analyst extension tested by cover case; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-MON.1 — Monetary issuance and redemption

Issue/cancel units designed for monetary use and define reserve, peg and redemption arrangements.

**Parents:** FIN-MON. **Neighbors:** FIN-CRE.2.

**Include when:** An issuer or monetary mechanism creates money-like units. **Exclude/redirect when:** Using USDC for a swap does not make the AMM a monetary issuer.

**Mechanisms:** reserve subscriptions; collateralized minting; supply adjustment. **Assets/positions:** issuer liability or protocol redemption claim.

**Examples:** Maker Vat/Dai. **Relevant capabilities:** ERC-20, ERC-3643, ERC-7943.

**Dependencies:** reserves; custodians; peg liquidity; governance.

**Foundation:** P1 §3.5 pp13–14; P4 §3.3 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-MON.2 — Payments and streaming

Transfer value to discharge a payment duty, immediately or over time.

**Parents:** FIN-MON. **Neighbors:** FIN-MON.3, FIN-EXC.2.

**Include when:** Receiver obtains a payment or accrued withdrawal entitlement. **Exclude/redirect when:** Routing a priced asset exchange is EXC.2; mere token transfer interface is capability.

**Mechanisms:** push/pull payment; vesting stream; recurring settlement. **Assets/positions:** accrued payment right.

**Examples:** Sablier Lockup. **Relevant capabilities:** ERC-20, ERC-721, ERC-2612, ERC-3009.

**Dependencies:** payer funding; cancellation authority; clock; transfer behavior.

**Foundation:** P1 §3.5 pp13–14; P4 §3.3 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-MON.3 — Escrow and conditional settlement

Hold value until agreed release/refund conditions are met.

**Parents:** FIN-MON. **Neighbors:** FIN-MON.2, FIN-EXC.2.

**Include when:** A custodied balance secures delivery or another obligation. **Exclude/redirect when:** A cross-chain message alone moves no funds and proves no escrow.

**Mechanisms:** hash/time conditions; adjudicated release; atomic delivery versus payment. **Assets/positions:** conditional beneficiary/refund claim.

**Examples:** Across SpokePool deposit (trade leg). **Relevant capabilities:** ERC-20, ERC-1271, ERC-5164.

**Dependencies:** arbiter; timeouts; counterparty fulfillment; finality.

**Foundation:** P1 §3.5 pp13–14; P4 §3.3 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-EXC.1 — Trading venues and liquidity provision

Price or match transfers and maintain trading liquidity.

**Parents:** FIN-EXC. **Neighbors:** FIN-EXC.2, FIN-DER.1.

**Include when:** Traders execute against matched orders or a liquidity inventory. **Exclude/redirect when:** A price oracle publishes data; it need not execute trades.

**Mechanisms:** constant-function/concentrated AMM; CLOB; RFQ; batch matching. **Assets/positions:** inventory ownership or liquidity position.

**Examples:** Uniswap V3 pool; Osmosis pool. **Relevant capabilities:** ERC-20, ERC-721, ERC-6909.

**Dependencies:** price impact; adverse selection; ordering; token behavior.

**Foundation:** P1 §3.1 pp7–10; P4 §3.1 pp3–4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-EXC.2 — Routing and coordinated trade execution

Select and coordinate venues or counterparties to satisfy a trade instruction.

**Parents:** FIN-EXC. **Neighbors:** FIN-EXC.1, FIN-MON.3.

**Include when:** The service chooses a path, match or fulfillment plan for an exchange. **Exclude/redirect when:** A settlement message bus without financial routing is infrastructure.

**Mechanisms:** aggregator path search; solver auction; intent resolution. **Assets/positions:** signed trade authority; escrow and fulfillment claims.

**Examples:** CoW settlement; Across SpokePool. **Relevant capabilities:** ERC-7683, EIP-712, ERC-1271, ERC-2612, ERC-3009.

**Dependencies:** solver competition; authorization; venue adapters; chain finality.

**Foundation:** P1 §3.1 pp7–10; P4 §3.1 pp3–4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-CRE.1 — Inventory-funded lending

Lend existing assets against repayment duties, with pooled, isolated or matched funding.

**Parents:** FIN-CRE. **Neighbors:** FIN-CRE.2, FIN-CRE.3, FIN-MGT.1.

**Include when:** An agreed financing advance creates repayment duties denominated in an existing asset. **Exclude/redirect when:** Pure debt-asset minting is CRE.2, not inventory lending. A solver reimbursement incidental to trade settlement is EXC.2; add CRE.1 only for a separately agreed financing service. An allocator owns MGT.1.

**Mechanisms:** utilization curves; bilateral negotiation; credit accounts; liquidation. **Assets/positions:** lender receivable/share; borrower debt and collateral position.

**Examples:** Aave V3 market; Morpho Blue market; Maple loan. **Relevant capabilities:** ERC-20, ERC-4626, ERC-3525, ERC-3475, ERC-7092.

**Dependencies:** oracle; underwriting; collateral sale liquidity; default waterfall.

**Foundation:** P2 §§III–V pp3–8; P4 §3.2 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-CRE.2 — Credit issuance

Create new debt units against an obligation to repay or settle collateral.

**Parents:** FIN-CRE. **Neighbors:** FIN-MON.1, FIN-CRE.1.

**Include when:** The advance increases an issued debt asset supply under a borrower obligation. **Exclude/redirect when:** Reserve-backed token subscription without borrower debt is MON.1 only.

**Mechanisms:** collateralized debt position; debt ceilings; stability fee; mint/burn. **Assets/positions:** issuer debt unit plus separate borrower debt position.

**Examples:** Maker Vat/Dai. **Relevant capabilities:** ERC-20.

**Dependencies:** collateral appraisal; mint authority; liquidation; monetary feedback.

**Foundation:** P2 §§III–V pp3–8; P4 §3.2 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-CRE.3 — Transaction-scoped liquidity

Advance assets with repayment or an equivalent settlement condition before the enclosing atomic scope ends.

**Parents:** FIN-CRE. **Neighbors:** FIN-CRE.1, FIN-EXC.1.

**Include when:** A callback grants temporary purchasing power and enforces repay-or-revert. **Exclude/redirect when:** Same-block timestamps do not provide rollback across transactions.

**Mechanisms:** flash loan; flash swap; flash accounting. **Assets/positions:** temporary obligation without persistent lender receivable.

**Examples:** Aave V3 flashLoan; Uniswap flash behavior. **Relevant capabilities:** ERC-3156.

**Dependencies:** callback; initiator identity; fee; approved asset; atomic scope.

**Foundation:** P2 §§III–V pp3–8; P4 §3.2 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-CAP.1 — Primary issuance and subscription

Sell or allocate newly issued claims to raise capital.

**Parents:** FIN-CAP. **Neighbors:** FIN-CRE.2, FIN-EXC.1.

**Include when:** A distinct primary offering raises capital for an issuer or project in exchange for newly issued rights. **Exclude/redirect when:** Routine deposit receipts or redeemable share accounting for an existing investment mandate are MGT, not automatically capital formation. Add CAP only for a separately identified capital-raising offering. Secondary trading is EXC.1.

**Mechanisms:** fixed subscription; auction; bonding-curve issuance. **Assets/positions:** equity/fund/debt/utility units with explicit rights.

**Examples:** Balancer V2 liquidity bootstrapping use. **Relevant capabilities:** ERC-20, ERC-3525, ERC-3475, ERC-3643.

**Dependencies:** issuer authority; allocation policy; legal rights; proceeds custody.

**Foundation:** P1 Fig1 p2 limited coverage; analyst extension tested by LBP case; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-CAP.2 — Conditional collective funding

Pool contributions subject to a funding goal, delivery condition or refund rule.

**Parents:** FIN-CAP. **Neighbors:** FIN-MON.3, FIN-CAP.1.

**Include when:** Contributor capital finances a project subject to collective conditions. **Exclude/redirect when:** An unconditional payment donation is MON.2; treasury investment is MGT.2.

**Mechanisms:** crowdfund escrow; milestone releases; refund thresholds. **Assets/positions:** contribution and possible refund claim.

**Examples:** Coverage design; no measured deployment asserted. **Relevant capabilities:** ERC-20, ERC-1155.

**Dependencies:** beneficiary; adjudication; deadline; project delivery.

**Foundation:** P1 Fig1 p2 limited coverage; analyst extension tested by LBP case; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-DER.1 — Linear price exposure

Provide linear or approximately linear exposure to a reference price with explicit margin/funding/settlement.

**Parents:** FIN-DER. **Neighbors:** FIN-EXC.1, FIN-DER.2.

**Include when:** Position payout responds to the referenced price without requiring asset delivery now. **Exclude/redirect when:** Holding a redeemable wrapped asset is not sufficient.

**Mechanisms:** futures; perpetual mark/index; cash or physical settlement. **Assets/positions:** margined long/short obligation.

**Examples:** Drift perpetual market. **Relevant capabilities:** ERC-20.

**Dependencies:** oracle; funding; insurance/backstop; liquidator and liquidity.

**Foundation:** P1 §3.8 pp15–16; P4 §3.5 pp4–5; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-DER.2 — Optional and structured payoffs

Provide exercise rights or nonlinear combinations of reference-dependent payments.

**Parents:** FIN-DER. **Neighbors:** FIN-DER.1, FIN-RSK.1.

**Include when:** Payoff changes nonlinearly or is conditional on exercise/structured thresholds. **Exclude/redirect when:** A loss-specific cover also uses RSK; leverage alone is not an option.

**Mechanisms:** options; capped/floored return; payoff replication. **Assets/positions:** option or structured note.

**Examples:** Opyn Gamma option design. **Relevant capabilities:** ERC-20, ERC-1155, ERC-3525.

**Dependencies:** option writers; premium; margin; exercise window; settlement data.

**Foundation:** P1 §3.8 pp15–16; P4 §3.5 pp4–5; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-DER.3 — Event and outcome claims

Allocate payoffs according to a resolved external or on-chain event.

**Parents:** FIN-DER. **Neighbors:** FIN-RSK.1, FIN-DER.2.

**Include when:** A resolution rule determines which contingent claims pay. **Exclude/redirect when:** Claims paying specifically on insured loss also use RSK.1.

**Mechanisms:** binary/multioutcome shares; oracle resolution; dispute window. **Assets/positions:** outcome token.

**Examples:** Gnosis Conditional Tokens. **Relevant capabilities:** ERC-1155, ERC-165.

**Dependencies:** resolver; event wording; challenge process; collateral.

**Foundation:** P1 §3.8 pp15–16; P4 §3.5 pp4–5; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-DER.4 — Interest and yield exposure transformation

Separate, exchange or hedge future cash-flow/rate exposure.

**Parents:** FIN-DER. **Neighbors:** FIN-CRE.1, FIN-MGT.1.

**Include when:** A claim changes exposure to future yield or an interest index. **Exclude/redirect when:** Fixed-interest debt alone stays CRE.1; a principal strip also records its underlying debt/asset claim.

**Mechanisms:** principal/yield split; rate swap; fixed/floating exchange. **Assets/positions:** principal strip; yield strip; rate derivative.

**Examples:** Pendle PT/YT market. **Relevant capabilities:** ERC-5095, ERC-5115, ERC-20.

**Dependencies:** underlying yield asset; maturity; negative yield; index/oracle.

**Foundation:** P1 §3.8 pp15–16; P4 §3.5 pp4–5; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-MGT.1 — Strategy and credit allocation

Allocate capital among selected strategies or credit venues under a mandate.

**Parents:** FIN-MGT. **Neighbors:** FIN-CRE.1, FIN-MGT.2.

**Include when:** Manager/allocator chooses exposures or limits for participants. **Exclude/redirect when:** The lending market itself remains CRE.1; automated execution alone need not pool assets.

**Mechanisms:** curated lending allocation; harvest/reinvest; leverage; nested vaults. **Assets/positions:** portfolio share or managed account claim.

**Examples:** Morpho MetaMorpho; Yearn V3 vault. **Relevant capabilities:** ERC-4626, ERC-7540, ERC-7575.

**Dependencies:** underlying venues; allocator; caps; fees; unwind liquidity.

**Foundation:** P2 §VII pp11–13; P4 §3.4 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-MGT.2 — Portfolio, index and treasury management

Maintain diversified or policy-constrained holdings and manage liquidity for a beneficiary.

**Parents:** FIN-MGT. **Neighbors:** FIN-CAP.1, FIN-MGT.1.

**Include when:** A mandate controls holdings, weights, cash or treasury exposures. **Exclude/redirect when:** An index number without an investable mandate is data infrastructure.

**Mechanisms:** rebalancing; index replication; treasury cash management. **Assets/positions:** portfolio/fund share or segregated assets.

**Examples:** Centrifuge fund vault; Set portfolio design. **Relevant capabilities:** ERC-4626, ERC-7540, ERC-7575, ERC-3643, ERC-7943.

**Dependencies:** valuation; custody; manager; tracking error; external rights.

**Foundation:** P2 §VII pp11–13; P4 §3.4 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-MGT.3 — Liquidity-position management

Choose and maintain exchange liquidity positions for capital owners.

**Parents:** FIN-MGT. **Neighbors:** FIN-EXC.1, FIN-MGT.1.

**Include when:** An agent/strategy manages ranges, inventory or fee reinvestment. **Exclude/redirect when:** The exchange pool supplies EXC.1; self-directed LP ownership is a position/strategy.

**Mechanisms:** range rebalance; inventory hedging; fee compounding. **Assets/positions:** managed LP share.

**Examples:** Arrakis liquidity vault design. **Relevant capabilities:** ERC-4626, ERC-721.

**Dependencies:** venue hooks; manager; adverse selection; rebalance execution.

**Foundation:** P2 §VII pp11–13; P4 §3.4 p4; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-SEC.1 — Consensus capital participation and delegation

Commit or delegate capital to consensus validation under reward and penalty rules.

**Parents:** FIN-SEC. **Neighbors:** FIN-SEC.2, FIN-MGT.1.

**Include when:** Capital is exposed to validator duties and consensus withdrawal rules. **Exclude/redirect when:** An ERC-20 staking incentive with no security duty is not consensus staking.

**Mechanisms:** validator deposit; delegation; reward accrual; slashing. **Assets/positions:** stake or delegated stake claim.

**Examples:** Ethereum validator deposit; Lido staking pool. **Relevant capabilities:** ERC-20.

**Dependencies:** validator; chain rewards; slashing; exit queue.

**Foundation:** P1 §3.6 pp14–15; contemporary case extension; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-SEC.2 — Liquid security-capital claims

Issue transferable claims on capital committed to consensus or shared security.

**Parents:** FIN-SEC. **Neighbors:** FIN-SEC.1, FIN-SEC.3.

**Include when:** A receipt gives economic exposure to underlying security capital and its exit conditions. **Exclude/redirect when:** Liquid receipt collateral in a loan adds CRE, without relabeling the loan as staking.

**Mechanisms:** rebasing receipt; exchange-rate wrapper; withdrawal request. **Assets/positions:** staking or restaking receipt.

**Examples:** Lido stETH/wstETH. **Relevant capabilities:** ERC-20, ERC-721.

**Dependencies:** operators; share accounting; queue; secondary liquidity.

**Foundation:** P1 §3.6 pp14–15; contemporary case extension; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-SEC.3 — Shared-security allocation

Assign capital-backed security obligations across additional services.

**Parents:** FIN-SEC. **Neighbors:** FIN-SEC.1, FIN-RSK.2.

**Include when:** Committed capital secures additional service duties with defined allocation and penalties. **Exclude/redirect when:** Looped lending yield without security duties is not restaking.

**Mechanisms:** operator delegation; restaking; slashable allocation; reward distribution. **Assets/positions:** restaked exposure and contingent penalty obligation.

**Examples:** EigenLayer allocation/delegation. **Relevant capabilities:** ERC-20.

**Dependencies:** service rules; operator; slashing authority; collateral reuse.

**Foundation:** P1 §3.6 pp14–15; contemporary case extension; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-RSK.1 — Protection and cover

Provide payment or loss reimbursement when defined covered conditions are accepted.

**Parents:** FIN-RSK. **Neighbors:** FIN-DER.3, FIN-RSK.2.

**Include when:** Beneficiary acquires a protection entitlement separate from ordinary asset appreciation. **Exclude/redirect when:** Outcome speculation alone is DER.3; transfer controls are not legal insurance.

**Mechanisms:** mutual claims assessment; parametric trigger; capped indemnity. **Assets/positions:** cover claim.

**Examples:** Nexus Mutual cover. **Relevant capabilities:** ERC-721, ERC-20.

**Dependencies:** claims authority; exclusions; capital sufficiency; dispute process.

**Foundation:** P3 threat overlay distinct from function; analyst extension tested by cover case; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-RSK.2 — Subordination and first-loss capital

Assign differing priorities of loss absorption to claims against shared exposures.

**Parents:** FIN-RSK. **Neighbors:** FIN-CRE.1, FIN-MGT.2, FIN-RSK.3.

**Include when:** A contractual waterfall specifies junior/senior treatment. **Exclude/redirect when:** Every risky token is not a tranche; priority must be explicit.

**Mechanisms:** seniority; credit enhancement; tranche reallocation. **Assets/positions:** senior/junior claim.

**Examples:** Centrifuge tranche design. **Relevant capabilities:** ERC-20, ERC-3525, ERC-7575.

**Dependencies:** underlying default; valuation; waterfall implementation.

**Foundation:** P3 threat overlay distinct from function; analyst extension tested by cover case; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.

## FIN-RSK.3 — Default and settlement backstops

Commit capital or guarantees to residual shortfalls from financial settlement.

**Parents:** FIN-RSK. **Neighbors:** FIN-RSK.2, FIN-CRE.1, FIN-DER.1.

**Include when:** A reserve or guarantor has a stated role absorbing deficits. **Exclude/redirect when:** Protocol revenue retained without a defined backstop is only treasury capital.

**Mechanisms:** insurance fund; guarantee; auction recapitalization; mutualization. **Assets/positions:** contingent loss-bearing capital.

**Examples:** Drift insurance fund. **Relevant capabilities:** ERC-20.

**Dependencies:** fund adequacy; governance; deficit measurement; recapitalization.

**Foundation:** P3 threat overlay distinct from function; analyst extension tested by cover case; [paper crosswalk](PAPER-CROSSWALK.md), [case evidence](VALIDATION.md), [normative atlas](STANDARDS.md). The category boundary is analyst synthesis.
