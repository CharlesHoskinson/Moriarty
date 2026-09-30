# Independent PL audit 04: language expressivity and financial horizon

Status: independent design review; no implementation, financial qualification, or grammar-freeze approval.

## Scope and identity

Requested reviewer: GPT-6.1 Sol, medium effort, fourth of five PL reviewers. The task message states that routing, but this agent context does not provide a machine-verifiable returned model or effort receipt. Actual returned identity/effort: unverified here. This audit must not be counted as proof of model-routing conformance until the host receipt is attached. No peer audits were read.

Reviewed frozen candidate: `BETA-DESIGN.md`, SHA-256 `6af4ca5356ed3fd58818b5717401d5b218b672b65769e342f708d4a4e0cd061a`; `sha256sum -c audits/candidate-v1.sha256` returned OK. Supplementary proposed mockup was hashed independently: `PROGRAMMER-MOCKUP.md`, SHA-256 `5ef082e9369506bcd4a7a5f1b1e8ec7a5b1ee8e9ba6f95c2ac233fd4736a7bfb`. The mockup is a separate review input, not covered by the candidate-v1 manifest.

Loaded the checked-in `moriarty-dev:develop` skill and inspected guarded status. Status reports SP01.6 loan-swap-subset, unresolved operational history, stale binding/candidate inputs, missing current accounting and unavailable live resource state. Pending transactions: none. These blocks concern dependent dispatch; this read-only review and its assigned report remain authorized.

Evidence inspected: candidate and mockup; [mockup requirements](../../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md); domain memos 01–04 and 09 (bounded inspected excerpts); [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md); [eight-family coverage](../../../deliverables/mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md); [kernel recommendation](../../kernel-api-recommendation-2026-09-29.md); existing [Source/6 contract](../../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md), Source/6 wrapper and relevant Core/5 preparation code. This was repository evidence inspection, without new source acquisition, runtime experiments, native proofs or ledger submission. External ecosystem comparisons are recommendations grounded in the inspected research memos, not independently reproduced ecosystem behavior.

## Decisions

| Choice | Vote | Scope and reason |
| --- | --- | --- |
| External brace language, closed financial operations and exact asset suffixes | Agree | Familiar data notation and named roles give a useful authoring beta. `10.00 USD` carries nominal scale; a scalar cannot silently become money. |
| Preserve nominal domain/account/asset/representation and separate financial resources | Agree | Same ticker, address bytes or numeric payload must not imply equal identity, authority, shares or liabilities. The ecosystem provisions correctly borrow explicitness without importing runtime guarantees. |
| S0-only local preparation with strict Source/6/Core/5 reuse | Agree | A narrow executable beta is coherent if unsupported profiles produce explicit no-effect rejection and the local result retains every missing premise and binding. |
| Source-fixed S0 values, separate untrusted completion and provider evidence | Agree | Source cannot delegate recipients or caps to a fixture. Full-language typed holes require a separately specified refinement relation. |
| Episodes, durable unknown outcomes and qualified recovery | Agree | These preserve the wider product's financial meaning; local cancellation/timeouts cannot discharge foreign claims. |
| Current mockup satisfies the full programmer-facing horizon requirements | Disagree | It names all eight families, but core financial interfaces remain prose strings and several required lifecycle operations are missing from source. |
| Freeze the wider grammar from these examples or count family coverage as complete | Disagree | Repairs H1/H2 and M1/M2 below are required before that conclusion. |

The S0 surface is useful and understandable, although its repeated empty fields expose wire constraints. Keep them explicit in the first beta. Any future shorthand should have an inspection view showing their exact expansion and should preserve financial observations. Do not reduce signed bounds or duty visibility merely to shorten the source.

## Findings

### H1 — The full-language source has records containing prose where the requirements demand typed interfaces

**Repository observation:** mockup AMM `reads`, `writes`, `kernel`, `completion`, `rounding` and `relation` are strings or lists of strings (lines 51–57). The option strike is a string (lines 132–134); its fixing and payoff contract are strings. Oracle `unit` is text and there is no observation value (lines 154–158). No example uses typed `pre`/`post` cells, a typed price, signed delta/position, typed clock, typed outcome or hole/completion constructor. The mockup explicitly admits that `Price` is not implemented; that is honest, but does not supply a proposed programmer interface. Requirements sections “Language surface to show” and “Intent language and DeFi kernel boundary” require that interface even when it is open.

**Inference:** these records are readable requirement sketches, not yet a coherent proposal for programming the full financial language. The risk is deciding that the eight-family horizon is represented because a parser can preserve annotations that have no typed or relational meaning. The candidate's disclaimer that strings do not erase full-language features is good; the mockup needs to demonstrate how those features would actually be expressed.

**Exact discriminator:** in a proposed full-language example, substitute `Price<GOLD,USD>` for `Price<USD,GOLD>` under the adopted Base-per-Quote convention, a foreign-domain round for a Preview round, or a balance cell for a share claim. The specification must identify the exact typing/formation rule that rejects each substitution, without consulting English inside `relation`. Add a misspelled/omitted fee cell and show how the derived footprint differs from the authored footprint. These are design discriminators; no current executable non-S0 checker is claimed.

**Repair:** propose one consistent, explicitly open typed notation for cells, observations/value/unit, prices, clocks, outcomes, finite holes and certified relation references. Name each relation's typed inputs, outputs, authority, footprint and error cases. Preserve prose as explanatory comments. Link each proposed construct to its semantic obligation and mark execution unsupported. This does not require implementing the full horizon in beta.

### H2 — The composed episode has no source-level binding to a complete signed economic promise

**Repository observation:** `BridgeThenSwap` (mockup lines 247–258) links three stage names and prose duty/authority fields. `DestinationSwap.signed_floor` is a string. Neither stage nor episode references a typed parent intent, fixed destination recipient, actual output asset/amount, per-asset gross/fee accounting, selected program/policy, bounded completion or signed retained-outcome relation. The bridge stages carry the same raw paired-claim string, but typed claim/attempt bindings are deferred in prose. The research workflow sketch already describes an enclosing signed intent with caps and separate recovery alternatives; the new common notation does not express that relationship.

**Inference:** the example communicates no-global-rollback correctly, but a programmer cannot identify from source which composed result was authorized or what remains owed after a destination failure. This misses the product contract's cumulative bounds and residual-duty requirement.

**Exact discriminator:** complete an episode with a valid source escrow and destination receipt, then (a) change destination recipient, (b) add an AMM fee, (c) retry after Unknown and allocate a second logical claim, or (d) return wrapped destination funds instead of origin funds. For each case, show the source field or signed alternative determining acceptance/rejection and the surviving duty. Matching stage names alone cannot decide.

**Repair:** add an explicit proposed episode-to-intent reference; typed economic claim IDs and attempt IDs; destination asset/recipient/net floor; per-asset cumulative debit and fee rules; pinned evidence/program policies; and typed outcomes for receipt, swap rejection, unknown receipt, qualified refund and destination return. Require refinement against the same signed relation at every continuation. No new canonical signing codec should be invented by the mockup.

### M1 — Several required family lifecycles are named in prose but absent from source examples

**Repository observation:** lending names roll-forward and aggregate locks as read strings rather than transitions/invariants. The option has only Exercise; premium payment, fixing and actual settlement stages are absent. Staking has Deposit and Unbond, without a reward/slash action or withdrawal claim. Stablecoin Shutdown has no emergency settlement or claim discharge. The eight sections exist, but existence of a section is weaker than the mockup's minimum-example requirements.

**Exact discriminator:** for a 100-share unbond request followed by a slash before maturity, identify the source-defined final claim amount, priority, loss beneficiary and withdrawal transition. For an exercised option with missing/disputed fixing, identify the continuation to authenticated fixing and payoff commitment. The current source cannot supply those answers.

**Repair:** provide small explicit proposed transitions for each missing mechanism, with read/write/evidence contracts and surviving duties. Add an eight-family by required-feature coverage matrix; distinguish shown source, explanatory mention, existing specification and open mechanism. Keep every financial implementation/proof gate open.

### M2 — Status language conflates the existing S0 implementation with the proposed beta authoring surface

**Repository observation:** mockup lines 4–8 say Local means the existing path and that names/quantities “can be checked by the beta.” The reference table (lines 267–272) labels profile/agreement/action, nominal declarations and quantity sugar “local authoring,” “local checks,” or “local preparation.” This document begins as proposed notation; the frozen design requests building that frontend. Inspection of the actual Source/6 wrapper establishes the old closed presentation and preparation result, not the proposed decimal-suffix beta parser, nominal checker or action selectors.

**Exact discriminator:** ask which current executable accepts the complete `moriarty-beta/1` transfer example with `10.00 USD`. A Source/6 run cannot establish that frontend capability; it expects its distinct closed profile. If no beta command receipt exists, the table must not communicate implemented status for the new spelling.

**Repair:** use separate columns for proposed beta support and currently demonstrated implementation. Mark proposed surface `specified`/`planned`; mark the existing underlying Source/6/Core/5 path `local`. Replace present-tense capability claims with proposed checks until command evidence exists. Add repository evidence links per row; the current construct table is not the requirements' evidence-backed coverage matrix.

### M3 — The S0 quantity range is underspecified at the new surface boundary

**Repository observation:** candidate grammar describes UInt128 intermediates and rejects out-of-range atom values, but does not state the Qty limit. Source/6 caps nominal values, fees, caps and floors at `2^127−1`; balances and counters may reach `2^128−1`. Domain02 correctly distinguishes them. Leaving only “checked nominal quantity” in the candidate lets frontend authors reasonably choose the wrong limit.

**Exact discriminator:** at scale zero, compare `atoms(asset: USD, value: 170141183460469231731687303715884105727)` with the next integer. The first is within Source/6's nominal bound; the second must reject as an authoring range error. A UInt128 balance containing the second value is a distinct admissible numeric category, subject to its other constraints.

**Repair:** state Qty/scalar/balance/intermediate ranges explicitly and name the formation diagnostic. Apply the Qty bound to final elaborated quantities while retaining checked UInt128 intermediates; retain the explicit outstanding/principal/accrued restrictions of S0. Do not report a financial Core failure for a frontend formation error.

## Financial coverage and preserved boundaries

| Family | Useful source evidence | Remaining horizon obligation |
| --- | --- | --- |
| AMMs/exchanges | Exact input, floor, fee cap, separate LP redeem | Typed reserve/custody relation and share conversion; routes and clearing remain open |
| Lending | Origination and liquidation sketches; exact local repayment mapping | Aggregate encumbrance invariant, roll-forward and explicit partial/default continuation |
| Stablecoins/synthetics | Mint/redeem and shutdown sketches | Typed supply/backing/debt equation and emergency claim settlement |
| Derivatives | Collateral, fixing and exercise narrative | Price/position typing, premium/fixing/settlement transitions; perpetuals remain additional work |
| Oracles | Source, round, age, finality and stale/disputed narrative | Typed value, subject, unit, authenticated selection and aggregation policy |
| Governance | Queue/enact, veto narrative and old-duty preservation | Typed authorities, revocation/quorum/timelock relation and preservation interface |
| Bridges | Distinct domains/assets, paired stages and no-timeout-refund | Typed paired claims, foreign proof, attempt identity and qualified recovery outcomes |
| Staking/restaking/yield | Deposit/unbond and rounding/priority narrative | Reward/slash transition, withdrawal claim, competing locks and loss waterfall |

The candidate does preserve the full-language goals in prose and explicitly rejects unsupported execution. I do not interpret S0 as a legitimate replacement for the full product horizon. The repairs above concern making that horizon reviewable as language, without inflating its execution status.

Actual code inspection confirms `prepareSource6S0Unqualified` returns four unverified bindings: agreement ID, selected program, asset scale and authenticated predecessor. Core/5 returns `PreparedUnqualified` with four required premises: canonical intent signature, snapshot-to-head, head extension and atomic ledger compare-and-consume. Ordered complete transfer/repayment effects and null published effects on rejection are the correct beta baseline. A later kernel remains optional and subordinate to Moriarty authority; typed service acknowledgements must not redefine language success or manufacture financial proof.

## Abstentions and disposition

Abstain on executable beta correctness, human comprehension/AI accuracy, ecosystem runtime guarantees, general elaboration equivalence, K/Quint/native correspondence, privacy, authenticated observations and ledger acceptance: no relevant experiments were performed in this audit. Abstain on actual requested-model conformance pending a host receipt.

Recommendation: retain the S0 beta direction and its nominal/exact-quantity provisions. Do not mark the programmer-facing full-language mockup accepted or freeze its broader grammar until H1/H2 and M1/M2 have concrete source-level dispositions. M3 needs an explicit numeric rule before beta lowering implementation. Design agreement is not executable or formal evidence.
