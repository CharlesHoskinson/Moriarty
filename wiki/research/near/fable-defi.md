---
title: "NEAR fable-defi expert study"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, near-teardown, research]
---

# Moriarty vs. NEAR Intents: strategy, economics, and what a language of provable intention can honestly borrow

Bounded independent pass. No external tools, no code executed, no pages read beyond the supplied excerpts. Every statement below is tagged in the claims array as verified-from-excerpt, inference, or unverified. The pinned repository commits (nearcore, intents, mpc, omni-bridge, amm-solver, sdk-monorepo) were named but their source was not excerpted, so contract-level behaviour of intents.near is not verified here.

## Explanation

### What NEAR Intents actually is, mechanically

Strip the marketing and the supplied evidence describes a request-for-quote (RFQ) market with on-chain settlement. A user or frontend posts a quote request. An off-chain WebSocket relay (the Message Bus / Solver Bus, run by Intents Technology Ltd., a BVI company) broadcasts it to connected solvers, who reply with signed intents. The user signs a matching intent, and the relay bundles both and submits them to a Verifier contract (intents.near) which settles by netting balances already deposited there (https://docs.near-intents.org/integration/market-makers/introduction lines 15-32; https://docs.near-intents.org/integration/market-makers/message-bus/introduction lines 8-25; https://docs.near.org/chain-abstraction/intents/overview lines 23-25).

Three properties follow from that shape and matter for Moriarty:

1. **Settlement is inventory-based, not liquidity-creating.** Solvers pre-deposit reserves into intents.near and register a public key (AMM solver README lines 62-96). The Verifier moves balances between accounts that are already inside it. Cross-chain delivery happens because the solver already holds inventory on the destination and moves it under its own control; the Solver Terms are explicit that Intents Technology 'supplies no Liquidity, capital, credit, balance sheet, or financing' (Solver Terms 7.1, line 239).

2. **The relay is optional in principle, gated in practice.** Both the market-maker introduction (line 33) and the Message Bus overview (lines 9-13) say the protocol can operate without the bus. But the Terms state that Intents Technology may add or remove gates 'including ONE_CLICK_API_ONLY' (6.2, line 220), the sample solver exposes that flag (README line 60), and relay access requires a Partner JWT plus KYC/KYB (Terms 2.3, line 144; guaranteed-delivery doc line 25). So the objective settlement layer is permissionless, while the distribution layer that carries user flow is permissioned. That split is the single most useful structural lesson for Moriarty, and it is exactly the split the product contract already draws between objective transaction acceptance and 'internal project work' (MORIARTY-PRODUCT-CONTRACT.md lines 45-52).

3. **'Atomic' is a settlement-contract property, not an end-to-end property.** The user-facing page says a swap 'either completes fully or you get automatically refunded' (what-are-intents line 25, line 46). The Terms say the opposite about the full pipeline: Intents Technology 'cannot reverse, retry, refund, recover, or unwind failed, partial, delayed, or locked Settlement' (6.8, line 232), the solver 'bears settlement-failure and partial-fill risk' (9.7, line 287), and bridged assets may be 'failed, partial, or stuck' (7.8(b), line 255). The narrow claim, that a single execute_intents call on intents.near is all-or-nothing, is plausible and not contradicted, but it is not established by any excerpt. The broad claim is contradicted by the operator's own legal text. Moriarty's product contract already refuses to promise global rollback (line 39); this evidence is the reason that refusal is correct.

### Distribution and 1Click

1Click is the distribution product: a hosted API where the caller submits an intent and the service 'handles the execution' (quickstart lines 8-14). There is no testnet (quickstart line 15). Fee capture sits here, not in the protocol. Verified schedule (https://docs.near-intents.org/resources/fees):

| Layer | Fee | Lines |
|---|---|---|
| On-chain protocol fee to fee_collector | 0.0001% | 9-17 |
| near-intents.org frontend | 0.2% to a Sputnik DAO | 19-25 |
| Withdrawal of NEAR/ZEC/STRK to Solana | 0.1% | 27-37 |
| 1Click without API key | +25 bps | 49-56 |
| 1Click with key | 20 bps, or 1 bp on stablecoin / same-asset routes | 57-62 |
| Partner appFees | 50/50 split, 1Click floor 20 bps (1 bp stable) | 63-69, 113-134 |
| Quote improvement | 50/50 split within about 30 minutes | 76-91 |

The economic reading is direct: the protocol contract earns almost nothing; the company earns on the API and on 'Capture Share' of price improvement, which the Terms say does not accrue to the solver (6.9, line 234; 9.3, line 279). Solvers keep only spread net of fees and their own gas and hedging (9.2, line 277). No volume or TVL figures appear in any excerpt and none are asserted here.

### Trust and service boundaries

The excerpts let us draw a boundary table (see Reference). The important asymmetries:

- The user-facing page claims 'Non-Custodial Security: you maintain control of your assets throughout' (what-are-intents lines 41-42). The Terms state the Verifier is 'subject to administrative roles' that may 'pause or upgrade the contracts, transfer or otherwise affect balances held in the Verifier' (7.7, line 251). Balances inside intents.near are therefore contract-custodied under an admin key, whatever the marketing says.
- Confidential Intents run on FAR, described as 'a permissioned instance of the NEAR protocol operated by a small validator set with no public networking' with a private RPC (confidential-intents lines 16-20). Shielding is a transfer to a Treasury account on NEAR plus a PoA bridge mint on FAR that is 'request-driven' rather than event-driven (lines 21-25). The Terms say the PoA Bridge is operated by Intents Technology (definition, line 95; 7.8, line 253). Privacy here is trust in a small validator set and a bridge operator, not a cryptographic argument. This is the opposite of what Moriarty's contract requires for private composition (product contract line 41).
- Compliance screening (TRM Labs, Binance AML, AMLBot/PureFi) is applied 'on integrated quote flows' and coverage 'can vary by flow and integration path' (risk-and-compliance lines 22-33). It is a distribution-layer policy, consistent with Moriarty's rule that a relay's policy binds that relay only (product contract line 52).

### Native DeFi vs cross-chain orchestration

NEAR's strategy captures position Intents alongside Chain Signatures (MPC signing by NEAR validators), Omnibridge, and Shade Agents in TEEs (strategy capture 0 lines 41-49, 64-99). Two of the four captures are 404 pages (captures 1 and 2), so the strategy corpus is thin. The verified design is orchestration: NEAR is the settlement and coordination hub, execution on other chains is done by solver inventory, an MPC network, or a PoA bridge. Native NEAR DeFi appears in the excerpts only as liquid staking, whose unstake delay of four epochs is bypassed by swapping the liquid token on a DEX (liquid-staking lines 14-15, 73-93).

Moriarty's position is different by construction. The product contract requires ZKIRv3 artifacts on Midnight (line 17) and forbids treating external observations as proofs (line 39). A Moriarty program can therefore be the *contract-side* of an RFQ market on Midnight, and can *model* an external leg as a named assumption with explicit states, but it cannot be the cross-chain orchestrator itself without a bridge whose security is an external assumption.

## Reference

### Service boundary matrix (verified from excerpts)

| Component | Operator per excerpts | Permissioned? | Locator |
|---|---|---|---|
| intents.near Verifier | Protocol contract with admin roles | Open to deposits; admin-upgradeable | Terms 7.7 line 251; definition line 137 |
| Solver Bus / Message Bus | Intents Technology | JWT + KYC/KYB | Terms 2.3 line 144; 1.definitions line 121 |
| 1Click API | Intents Technology | API key for fee schedule | fees lines 47-71 |
| FAR / intents.far | Small validator set, private RPC | Yes | confidential-intents lines 16-20 |
| PoA Bridge | Intents Technology | Yes | Terms line 95, 253 |
| Compliance screening | Intents Technology via vendors | Applies to integrated flows | risk-and-compliance lines 22-33 |

### Partial and contingent settlement patterns observed

- **Shared-nonce alternatives.** A confidential quote with a shield intent must include a recover intent that 'shares the swap's nonce, only one can land' (confidential-intents lines 136-147). This is a mutually exclusive pair of transitions under one affine authority.
- **Observation-gated release.** 'Solvers should only act on the quote_settle_successful event before releasing funds' (lines 148-150). The event is an off-chain relay signal, not a ledger proof.
- **Asynchronous shield status.** UNSHIELDED notifications arrive on a separate stream with acknowledgement and up to seven days of replay (lines 152-211; guaranteed-delivery lines 457-469). The guaranteed-delivery feature is marked early access with 'no solver has exercised it yet' (line 9).
- **Indicative then firm.** Quotes are non-binding until selection, then firm within a tolerance band (Terms 6.1, 6.3 lines 218, 222).
- **Solver-side partial risk.** Partial-fill and bridge-partial outcomes are explicitly the solver's risk (9.7 line 287; 7.8 line 255).

### Moriarty product-contract anchors

- Signed intent must carry 'allowed partial completion/recovery' (line 33).
- External effects require 'explicit pending, partial, unknown, settled and recovered states. Timeout is not evidence of nonexecution' (line 39).
- Failed fallible phases may retain guaranteed-phase effects and fees; every permitted failure outcome must be bound (line 39).
- Liabilities evolve separately from token supply (line 35).

## How-to: derive Moriarty support for partial fills and contingent settlement

These are recommendations, not implemented behaviour. Nothing in the excerpts shows Moriarty code for any of them.

1. **Encode partial completion as a signed constraint, not a runtime discovery.** Following product-contract line 33, the intent carries a minimum acceptable fill (per asset, net of fees), a maximum residual the user is willing to leave open, and whether the residual stays live or is cancelled. A candidate plan that fills less than the minimum is rejected by the acceptance relation, not refunded after the fact.
2. **Model mutually exclusive outcomes as alternative transitions over one affine authority.** The NEAR shield/recover pair shows the need. In Moriarty terms, one spending authority admits exactly one of N declared transitions; each transition's effect set is fully accounted (line 35), and residual duties are explicit in each branch (line 37). The proof obligation is that no two branches can both consume the authority and that every branch is a valid transition.
3. **Treat external settlement signals as named assumptions with typed states.** A quote_settle_successful event or an UNSHIELDED status becomes an observation of type pending | partial | unknown | settled | recovered. The program may condition on it, but the proof statement lists it as an assumption (line 39) and cannot claim external truth. Timeout maps to unknown, never to nonexecution.
4. **Bind fees and phase effects per branch.** Because Midnight's fallible phase can fail while guaranteed-phase effects persist (line 39), each partial-fill branch must state which fees are charged and which nonces are consumed. The lowerer may not choose.
5. **Keep distribution policy out of the language.** Any Moriarty-side RFQ relay, if built, is an optional application (product contract line 13). Its access gates, screening, and fee schedule bind that relay only (line 52). Two proposal sources within the same signed constraints must verify identically (line 66).
6. **Do not promise what NEAR does not deliver either.** No global rollback, no unconditional refund, no liquidity, no inclusion, no oracle or bridge honesty (line 39). The NEAR Terms are the operator's own admission that these are not deliverable end to end.

## Tutorial: walking one contingent swap through both systems

Suppose a user wants at least 990 units of asset B for 1000 units of asset A, accepting a partial fill down to 500 A in, and wants any unfilled A to remain spendable.

**On NEAR (per excerpts).** The frontend posts a quote request; solvers reply with indicative quotes; the user signs; the relay submits to intents.near. If the selected solver fills, balances net inside the Verifier. If it fades, the Terms make that a breach between solver and Intents Technology, not a user remedy (6.3, 6.4 lines 222-224). Whether the contract supports a partial fill of 500 A is not determinable from the excerpts; the sample AMM solver quotes a full pair with a margin parameter (README line 59) and nothing shown describes partial matching.

**In Moriarty (proposed).** The user signs an intent with asset identities, gross debit bound 1000 A, minimum net 990 B per 1000 A scaled linearly, minimum fill 500 A, residual policy 'retain authority'. A proposer submits a plan filling 700 A for 694 B. The verifier checks: program typing, intent refinement (700 within [500,1000], 694 at or above the pro-rata floor), transition validity, complete effects including fee recipients, and that the residual 300 A authority is unconsumed. If the plan instead depends on an external leg, the effect set includes a pending state, and the proof lists the settlement observation as an assumption. A second proposer submitting a 1000 A full fill under the same signed constraints must be admissible on identical terms.

The difference is not that Moriarty makes the fill happen. It is that the boundary between what was proven and what was assumed is stated in the artifact rather than in a terms-of-use page.
