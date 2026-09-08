# Supplied ERC-4626 collateral-vault report: taxonomy and design analysis

Source: `.raw/captured/21f40f3a7faa56406001dd783c4b9d33ae1c25751ffba875a198d75c716993ba.md`.
Title: *ERC-4626 and Collateral Vaults: A Research Taxonomy and Technical Atlas*. Author and original generation provenance are unspecified. Supplied file: 63,876 bytes; SHA256 `21f40f3a7faa56406001dd783c4b9d33ae1c25751ffba875a198d75c716993ba`.

Coverage: all **383 logical input lines** read, including the un-terminated final line (the file has 382 newline characters). All tables, diagrams, results, standards descriptions, project descriptions, gaps and artifact links inspected as text. Locators below are input 1-based lines. This is an anonymous secondary research synthesis. No network, source retrieval, primary repository inspection, formal proof check or incident reproduction occurred. Report assertions of current standards, implementations, empirical results and theorems remain source-reported. `EXTRACTED` graph confidence means a relationship is explicit in this report, not independently established.

The report's `turn…` citation tokens have no resolvable bibliography in the supplied bytes. Its sandbox links at lines 339–349 do not supply the claimed atlas, ZIP, JSON, CSV or BibTeX files. The 80 sources, 15 projects, 14 results and typed relationship dataset asserted at line 335 are report claims; this intake does not recreate or claim inspection of those attachments. A named reference can guide later bounded acquisition, but its existence here is not an independent supporting source.

## Main contribution and comparison with the four supplied papers

The strongest addition is the separation of **share accounting → share valuation → collateral valuation → borrowing power → liquidation proceeds** (lines 5–30), extended through custody, strategy accounting, claim, oracle, eligibility, execution and loss allocation (lines 245–265). Its four valuation quantities are accounting, redemption, market and stressed-liquidation value (lines 311–325). Equality between them is an assumption requiring justification. An ERC interface check, a local accounting invariant, and a lender solvency assertion are different predicates.

This refines rather than replaces the four paper analyses. Gogol's separate service/token/algorithm/deployment axes and Kotzer's distinction between aggregator, vault, strategy, venue and receipt claim support using architecture and research facets separately. Kotzer's liquidation eligibility, incentive and recovery distinction gains concrete auction/Stability-Pool/JIT/redistribution/reserve-absorption examples here. Zhou's unsafe dependencies, token semantic incompatibility and privileged operators gain specific donation, cap-accounting and asynchronous-claim examples. Werner's estimate-versus-truth distinction gains four explicit valuation kinds. These comparisons are analyst inference from the existing scratch analyses, not independent corroboration of the report's uninspected references.

A terminology warning: the report calls Werner a source for separating contract correctness and economic security (line 161). Werner's actual technical/economic taxonomy in paper-4-analysis.md is atomic exploitability versus non-atomic exposure to intervening events. Do not replace that definition with a code-bug/economics split. Donation and oracle attacks may occupy either side depending on execution control and timing.

## Economic architectures A–H

These report classes distinguish ownership, tokenized claim, liability location and loss-bearing structure. They are not a claim that every protocol belongs to one exclusive class. The report explicitly says Sky/CDPs and Compound account collateral must not become ERC-4626 merely because they are called vaults (line 34). Assign an interface relationship per component, as the Aave row emphasizes (line 223). Representative systems below are report examples, not current support verification. Source: lines 38–47.

| ID | Architecture | Claim and liability structure | Representative systems |
|---|---|---|---|
| **A** | Tokenized asset-management / yield vault | Depositor receives fungible shares on managed assets. The ERC-4626 layer need not itself create borrower debt. | OpenZeppelin base ERC-4626, Yearn V3, Aave Earn Vaults, Morpho Vault V2, Silo Vaults. Aave's current Earn product explicitly exposes ERC-4626 shares while routing underlying assets into Aave markets.  |
| **B** | Collateralized debt position | User owns a position containing collateral and owes protocol/stablecoin debt; the position is not necessarily a fungible pooled share. | Sky Vaults/CDPs and Liquity Troves.  |
| **C** | Lending / credit vault | Depositors own a claim on a pool whose assets fund borrower liabilities; bad debt can impair the pool. | Euler Vault Kit and allocation vaults feeding lending markets. Euler's repository includes dedicated audits, Certora artifacts, source and tests.  |
| **D** | Vault shares used as collateral | Debt exists in a second system, while the collateral is itself a tokenized claim on a first system. | ERC-4626 shares accepted by lending markets; Venus wUSDM-style integration failures illustrate the key oracle risk.  |
| **E** | Shared-collateral / margin architecture | Account-level collateral supports borrowing or other obligations, often through internal accounting rather than one ERC-20 vault share. | Euler's EVC architecture; Aave and Compound account collateral. Compound III records collateral balances inside Comet.  |
| **F** | Staking/restaking/security collateral | Receipt represents capital exposed to validator or security-system obligations, including possible slashing; exits can be delayed. | Liquid-staking tokens are an explicit motivating asynchronous-redemption example in ERC-7540.  |
| **G** | Asynchronous / permissioned / RWA vault | Investment claim may involve request queues, issuer processing, valuation dates, custodians, or off-chain settlement. | ERC-7540 and Centrifuge. Centrifuge currently exposes request-based investment and redemption flows and manager-mediated fulfillment.  |
| **H** | Structured / multi-asset / nested vault | A common claim spans several asset entry points, strategies, tranches, or downstream vaults. | ERC-7575 and multi-strategy allocators; Aave's current stable-vault documentation describes allocation among approved ERC-4626 strategies.  |

## Research facets T01–T11

The report expressly identifies these as facets rather than subclasses (line 49). Keep the question and inclusion boundary, rather than reducing a facet to an implementation action. Source: lines 51–63.

| ID | Research facet | Inclusion boundary | Central unresolved question |
|---|---|---|---|
| **T01** | Standards and interoperability | Interfaces, extensions, adapters and detection | How much risk metadata can be standardized without embedding policy? |
| **T02** | Asset/share accounting | `totalAssets`, supply, conversions, fees, rounding | Which invariants survive losses, debt and asynchronous settlement? |
| **T03** | Valuation and collateral oracles | Accounting value → economic/NAV/collateral value | What is a manipulation-resistant *realizable* price for a vault share? |
| **T04** | Lending and leverage | Borrowing, rates, allocation, recursive exposure | How should gross recursive positions be reduced to net underlying exposure? |
| **T05** | Liquidation/default | Auctions, bad debt, stability pools, recovery | Which mechanisms remain solvent under congestion and thin markets? |
| **T06** | Liquidity and settlement | Withdrawal capacity, queues, gates, claims | How should pending and claimable states be valued by composable systems? |
| **T07** | Nesting and contagion | Vault-of-vaults, cross-chain and downstream dependencies | How should dependency graphs translate into enforceable exposure limits? |
| **T08** | Governance/operations | Owners, curators, allocators, timelocks, upgrades | How should privileged control enter a machine-readable risk model? |
| **T09** | Attacks and defenses | Exploitable mechanisms, mitigations, incidents | Can integrations specify manipulation resistance independently of one oracle design? |
| **T10** | Verification and testing | Fuzzing, invariants, formal models, reproducibility | What properties can be composed across vault and lender boundaries? |
| **T11** | Empirical market structure | Liquidators, utilization, concentration, on-chain behavior | What reproducible longitudinal datasets should become research infrastructure? |

## Standards lineage and the incomplete requirements matrix

All status and creation-date entries in this paragraph are **reported by the source**, dated to its stated September 8, 2026 cutoff (line 329); none is independently checked in this delegated intake.

| Standard | Reported creation/status | Reported semantic addition and boundary | Lines |
|---|---|---|---|
| ERC-4626 | 2021-12-22; Final | ERC-20 share representation and one ERC-20 accounting/deposit/withdrawal asset; shares may disable transfers by reverting | 67 |
| ERC-5143 | 2022-06-09; Stagnant | Explicit execution slippage bounds; informational preview alone is not a signed acceptable outcome | 69 |
| ERC-7535 | 2023-10-12; Final | Native assets, payable deposit/mint and asset sentinel; not wrapped ETH as ERC-20 | 71 |
| ERC-7540 | 2023-10-18; Final | Asynchronous deposits, redemptions or both; Pending → Claimable → Claimed; affected previews revert | 73 |
| ERC-7575 | 2023-12-11; Final | Multiple differently denominated entry points share an external ERC-20 representation; common economic valuation unspecified | 75 |
| ERC-7887 | 2025-02-18; Draft | Separate cancellation lifecycle; pending cancellation blocks corresponding new requests | 77 |
| ERC-8330 | 2026-07-05; Review at cutoff | Provider-attributed timed NAV snapshots, correction/invalidation/staleness/aggregation; explicitly not an ERC-4626 extension or executable-price guarantee | 79 |

The report's requirements matrix (lines 81–92) describes `asset()` and `totalAssets()` non-reversion; managed assets including fees; caller-independent, fee/slippage-excluding, downward-rounded conversions; conservative limit-aware `maxDeposit`/`maxMint`; fee-aware same-transaction preview inequalities; and exact-quantity authorized entry/exit effects. These are reported normative descriptions, not a complete normative transcription. **`previewRedeem`, `maxWithdraw` and `maxRedeem` have no dedicated rows.** The phrase conversions “generally cannot revert” does not spell out all exceptional cases. A conformance specification must return to the primary pinned standard for all methods, rounding directions, events, limit/preview relations and exceptions rather than infer symmetry or completeness from this matrix.

## Result ledger R001–R014

Preserve each report-assigned evidence class exactly, while placing the entire ledger under secondary-source status. `STANDARD_REQUIREMENT`, `IMPLEMENTED_DESIGN`, `DEMONSTRATED_COUNTEREXAMPLE`, `EMPIRICALLY_SUPPORTED`, `PROVED_WITHIN_MODEL`, `PROPOSED`, and `UNVERIFIED` below are the report's assessments, not acceptance states of this intake. No empirical numbers become 2026 parameters and no preprint theorem becomes contract safety. Source: lines 106–121.

| ID | Major result | Evidence class | Scope and boundary |
|---|---|---|---|
| **R001** | ERC-4626 creates a common single-underlying tokenized-vault interface but leaves economic strategy and collateral policy outside the standard. | `STANDARD_REQUIREMENT` | Interface-level result.  |
| **R002** | Conversion functions, previews, limits and execution calls intentionally answer different questions. | `STANDARD_REQUIREMENT` | Misusing one as another is an integration error, not necessarily a vault bug.  |
| **R003** | Balance-sensitive empty vaults can be manipulated so subsequent deposits round to negligible or zero shares. | `DEMONSTRATED_COUNTEREXAMPLE` | Canonical first-deposit/inflation model.  |
| **R004** | Virtual assets/shares and increased share precision materially improve resistance to the canonical inflation attack. | `IMPLEMENTED_DESIGN` | Not a universal proof of exchange-rate safety; OpenZeppelin's public issue tracker contains a repeated-victim counterexample discussion for the zero-offset case, reinforcing the need to state assumptions.  |
| **R005** | Direct donations can manipulate assets-per-share and become exploitable when a second protocol treats the resulting rate as collateral value. | `DEMONSTRATED_COUNTEREXAMPLE`; incident-supported | Strongest practical ERC-4626-related result in this survey.  |
| **R006** | Generalized property tests can encode many ERC-4626 interface obligations and round-trip relationships independently of implementation internals. | `IMPLEMENTED_DESIGN` | The a16z suite explicitly excludes strategy-specific yield/share-price correctness.  |
| **R007** | Asynchronous vaults require a request lifecycle and deliberately invalidate atomic preview assumptions on the asynchronous side. | `STANDARD_REQUIREMENT` | Fulfillment timing and exchange-rate policy remain implementation dependent.  |
| **R008** | Multi-asset entry points can share one ERC-20 claim without standardizing a common economic NAV. | `STANDARD_REQUIREMENT` | ERC-7575 solves representation/composability, not valuation.  |
| **R009** | Historical liquidation data show that relatively small price movements can make very large collateral positions liquidatable and that liquidator behavior materially determines realized protocol safety. | `EMPIRICALLY_SUPPORTED` | Perez et al. found, in historical Compound data, that a 3% asset-price variation could render more than $10 million liquidatable and that immediate liquidation rates had risen above 70%; Qin et al. broadened empirical study to Aave, Compound, MakerDAO and dYdX. These are historical findings, not 2026 parameters.  |
| **R010** | Overcollateralization does not eliminate systemic instability: endogenous stablecoin demand and liquidation can generate deleveraging spirals. | `PROVED_WITHIN_MODEL` | Formal stochastic/equilibrium result, not a contract-level proof.  |
| **R011** | Under a specific CPMM spot-oracle/liquidator model, transaction fees can become a security parameter large enough to remove profitability of OEV manipulation. | `PROVED_WITHIN_MODEL`; `PROPOSED` | 2026 preprint; highly model-dependent.  |
| **R012** | Curated modular vaults create an underwriting/governance layer distinct from the base lending protocol. | `PROPOSED`; `UNVERIFIED` | Supported by 2025–2026 preprints on curator concentration and vault credit metrics; independent replication is still needed.  |
| **R013** | A credit vault can combine ERC-4626-style depositor accounting with explicit borrower liabilities and a separate collateral/authentication connector. | `IMPLEMENTED_DESIGN` | Euler-specific architecture; not a universal ERC-4626 property.  |
| **R014** | Liquidation is not one mechanism: auction, Stability-Pool, JIT, redistribution and reserve-absorption systems create fundamentally different liquidity and loss paths. | `IMPLEMENTED_DESIGN` | Cross-system comparison requires stress assumptions rather than interface labels.  |

## Mechanisms, defenses, and quantifiers

Nine mechanisms are explicit (lines 125–135): first-deposit inflation; direct donation; rounding accumulation/stealth donation; preview/execution mismatch; oracle manipulation; liquidity/queue failure; strategy/adapter loss; privileged-role/upgrade failure; balance/cap accounting bypass. Their causal structures are more useful for a language than a list of protocol names.

The report's illustrative empty-vault arithmetic (line 96) assumes an unprotected balance-sensitive vault: attacker deposits 1, donates 100, victim deposits 100 and receives floor(100/101)=0 shares; the attacker share represents 201 assets. This arithmetic follows its stipulated model; it is not a replay of any implementation. The other illustration doubles the accounting ratio by donating 100 to a 100-assets/100-shares vault (line 98). The report correctly warns that donation costs capital, so the price jump alone is not a profitability proof. Repeated rounding residuals and multiple victims need sequence-level analysis; virtual assets/shares and decimal precision protect a stated threat model, with a reported zero-offset multi-victim edge case (lines 127–143). The source's general praise of these defenses is not a universal exchange-rate safety theorem.

Asynchronous semantics must retain pending commitment, fulfillment, claimable entitlement, changing conversion rate and pull-based claiming (lines 100, 237, 296–309). Cancellation is a second lifecycle, with pending/claimable/claimed cancellation states in the diagram. The drawing is schematic: its connector placement cannot replace normative transition guards or authorization rules. Request acceptance does not guarantee a deadline, redemption price, liquidity, off-chain settlement or legal recovery.

Nested allocation propagates strategy losses, stale reports, shared venues and management powers (lines 133–134, 231–239). Isolated underlying markets need not mean an allocator is economically insulated. The report proposes graph-based gross-to-net exposure normalization, but supplies no algorithm, cycle treatment or proof (line 278). Privilege metadata must identify who can change adapters, caps, oracles, fees and withdrawal rules, with delay (line 281). Balance/cap bypass is a distinct alignment problem: risk limits and solvency must observe the same economic state, including unsolicited transfers (line 135).

Liquidation branches are not interchangeable. Reported Sky auctions, Liquity Stability Pool → JIT → redistribution and Compound reserve absorption shift execution delay, keeper funding, collateral custody, loss bearing and residual debt differently (lines 145–147, 241–243). Conformance or a collateral ratio at eligibility cannot prove realizable recovery.

## Twelve research gaps, without invented source IDs

The supplied report has **no O-prefixed question labels**. Its twelve gap rows are unnumbered. Preserve their names and proposed smallest artifacts (lines 269–284):

| Gap | What is known | Missing result | Smallest useful research artifact |
|---|---|---|---|
| **Composable collateral-oracle specification** | Donation attacks show that internal assets/share rates can be unsafe collateral prices.  | Necessary/sufficient conditions under which a vault conversion is safe enough for lending. | A formal interface between vault assumptions and lender oracle assumptions, plus adversarial tests. |
| **Loss-aware ERC-4626 invariants** | Existing conformance properties cover interface relationships and round trips.  | Properties that remain valid through strategy loss, bad debt, fees and partial recovery. | Stateful invariant suite with explicit loss injection. |
| **Multi-victim mitigation bounds** | Virtual shares/assets improve canonical first-deposit economics; repeated-victim edge cases exist.  | Tight attacker-profit bounds over arbitrary sequences of deposits, donations and withdrawals. | Symbolic state-machine model parameterized by offset and victim sequence. |
| **Asynchronous collateral valuation** | ERC-7540 standardizes Pending/Claimable/Claimed state but leaves fulfillment economics flexible.  | Haircuts and solvency rules for pending/claimable claims. | Stress model linking settlement delay distributions to lender LTV. |
| **Multi-asset NAV safety** | ERC-7575 standardizes common shares; ERC-8330 proposes auditable NAV publication.  | A connection between reported NAV and actually realizable redemption/liquidation proceeds. | Dataset comparing published NAV, redemption realizations and secondary-market prices. |
| **Nested-vault leverage normalization** | Research shows recursive lending can create large gross exposure.  | General method to collapse vault-of-vault dependency graphs into net economic exposure. | Graph algorithm plus on-chain benchmark corpus. |
| **Liquidation under congestion and MEV** | Empirical work measures historical liquidation and new models incorporate OEV.  | Cross-mechanism stress comparison under identical price/liquidity/congestion shocks. | Reproducible simulator for auction, fixed-spread, Stability-Pool and reserve-absorb designs. |
| **Curator/allocator governance risk** | Recent work identifies curator-layer concentration and proposes metrics.  | Independently validated predictors of realized vault loss/liquidity stress. | Longitudinal curator-action and outcome dataset with preregistered metrics. |
| **Machine-readable privilege risk** | Current protocols expose materially different manager/governance powers.  | Standard representation of who can change caps, adapters, oracles, fees and withdrawal rules, and after what delay. | Static permission extractor and normalized role schema. |
| **Non-standard underlying tokens** | ERC-4626 assumes an ERC-20 asset but strategy implementations must interact with real token behavior.  | Systematic tests for fee-on-transfer, rebasing, callbacks, blacklists and balance-changing tokens. | Differential corpus of pathological ERC-20 mocks run against major vault libraries. |
| **Incident reproduction corpus** | Individual postmortems reveal reusable accounting/oracle mechanisms.  | Standardized local replays with pre-state, exploit invariant, patch and regression property. | Foundry/Anvil archive of sanitized incident reproductions. |
| **Technical versus legal RWA claims** | Asynchronous standards and NAV interfaces expose timing/provider metadata but do not establish backing or legal recoverability.  | Model linking on-chain share ownership to custody, servicing and insolvency claims. | Cross-disciplinary asset-class template pairing contract state with legal entitlement state. |

The three audience tracks—standards implementer, security researcher and collateral-system designer—are reading/reproduction recommendations, not executed work (lines 286–294). The local progression starts with conversions and round trips, direct-transfer attacker, supply/decimal-offset variation, toy lender using conversion as oracle, then bounded pricing, liquidity and liquidation. This is a useful staged experiment specification, not an implementation result.

## Concrete refinement requirements for the existing Moriarty proposal

Repository observation: `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md` is explicitly S2 specified-only. It already separates shares from assets (line 82), proposes explicit floor/ceiling division (line 54), retains identified requests/residual duties (Obligations and workflows), distinguishes composition operators (line 122), and disclaims oracle truth and timely settlement from HistoryCompliance (line 136). The following are **analyst-proposed refinements**, not report requirements or evidence of existing implementation:

| Target | Required refinement | Distinguishing future check | Report locator |
|---|---|---|---|
| DA17 vault entry/exit; DA21 intent refinement | Keep method-specific accounting quantity, preview, execution bound and capacity separate; make fee/rounding policy and input asset behavior explicit | Zero/near-zero supply, direct donation, repeated victims/round trips, fee changes, insufficient liquidity, and min-output failure; no proof inferred from a preview | 81–100, 127–143, 282 |
| DA20 observations; DA05 collateral; DA06 borrowing | Use tagged accounting/redemption/market/stressed-liquidation observations with provider, time, units, validity and strategy dependencies; require explicit valuation-to-borrowing policy | Same accounting balance with different realizable liquidity yields distinct permitted debt; donation alone cannot silently increase debt limits; stale/invalid NAV rejects according to declared policy | 11, 55–56, 79, 98, 128–135, 311–323 |
| DA18 asynchronous workflow; DA15 staking exit; DA16 external claim | Extend the action target beyond asynchronous redemption to deposits where required; specify controller/operator authorization, fulfillment evidence, claim/cancel mutual exclusion and partial residuals | Pending cannot masquerade as immediately spendable shares; partial fulfillment/cancellation cannot duplicate claims or erase duties; changed rates do not retroactively create preview guarantees | 73, 77, 100, 237, 296–309 |
| DA07 liquidation/default | Separate eligibility, seizure, external execution, proceeds allocation, reserve use, residual bad debt and recovery; choose a mechanism fixture explicitly | Equal marked collateral can produce different recoveries under congestion, liquidity shortfall and differing loss waterfalls | 121, 145–147, 241–243, 279 |
| DA19 allocation/rebalancing; DA24 composition | Carry bounded dependency and liability graphs through nested shares, strategy losses and recursive borrowing; explicitly handle repeated underlying exposure and cycles | An apparently diversified or isolated allocator still reveals shared dependencies; unwind preserves debt and loss; finite graph traversal rejects beyond bounds | 59, 133, 233–239, 278 |
| DA22 administration; DA20 observations | Bind delegated authority, permitted parameter changes, timelock and adapter/oracle versions to obligations and existing collateral acceptance | An authorized risk-changing upgrade triggers the declared revalidation rule; a cap cannot ignore direct-transfer economic exposure | 60, 134–135, 281 |
| DA14/DA16 heterogeneous collateral | Preserve slashing, delayed exit, custody and legal-claim assumptions distinctly from generic fungible vault shares | Same receipt balance may suffer slashing or unenforceable recovery; neither interface compliance nor published NAV discharges the legal/economic obligation | 45–46, 79, 284 |

The proposed compiler/proof role is to reject invalid quantity/claim uses and establish declared transition, authority, accounting and residual-duty predicates under explicit observations and dependencies. External price truth, future liquidity, keeper participation, legal backing and generalized profitable-attack resistance require separately named models and evidence. These refinements can enrich the action matrix before any new syntax is chosen. They do not establish Moriarty settlement, proof correspondence or feature acceptance.

## Contradictions, omissions and evidence disposition

- The report explicitly preserves a Venus wUSDM postmortem year conflict: its opening allegedly says February 27, 2024 while publication/timeline artifacts indicate February 2025 (line 185). Preserve the conflict as **reported**, not resolved by this intake. The reported $902,000 bad debt/$185,000 recovery are unverified. Resupply estimates differ, roughly $9.5–9.8 million (lines 15, 186); do not choose a fabricated precise amount.
- Venus THE is explicitly described as not ERC-4626, an economic analogue for cap/accounting bypass (lines 17, 135). Its reported exposure multiple and loss amounts cannot establish an ERC-4626 vulnerability.
- The claimed atlas counts and artifact contents are unavailable (lines 335–349). The visible project table has fourteen grouped rows (lines 212–225), while the absent catalog is claimed to have fifteen project records. Grouped entries could explain this; it is an unresolved coverage mismatch, not proof that the count is false.
- Current versions, licensing, deprecation, deployments and audit/Certora artifact availability are all source reports (lines 179, 212–239). No repository revisions, theorem names, proof assumptions, audit findings or code correspondence were checked here. The report itself acknowledges incomplete audit/PDF inspection (line 333).
- The report's normative matrix omits methods and exceptions noted above. Its schematic asynchronous diagram is not an executable state machine. Its recursive-exposure gap is not a solved algorithm. Its claim that the T01–T11 taxonomy survived literature review (line 49) does not provide a reproducible review protocol.
- Evidence classes remain typed and scoped. Repeated citations to Werner and other source families add no independent replication. No present-day market census, current normative verification, theorem validation, incident replay, full source atlas or complete systematic-review coverage is claimed.

Source-only extraction coverage: 43 nodes, 71 ordinary edges, one hyperedge. The integrated graph adds four paper-context nodes and six Moriarty requirement nodes; see GRAPH_REPORT.md for its 53-node/95-edge totals. All A–H classes, T01–T11 facets, seven standards, nine attack mechanisms and seven organizing concepts are nodes; the complete R001–R014 ledger, twelve unnumbered gaps and detailed ecosystem descriptions are retained in concept rationale attributes to bound graph size. Published graph paths resolve to the immutable capture; source reading and graph drafting were separate from the reviewed vault apply.


[Captured report](../../.raw/captured/21f40f3a7faa56406001dd783c4b9d33ae1c25751ffba875a198d75c716993ba.md) · [Library and graph](README.md) · [Moriarty comparison](DESIGN-IMPLICATIONS.md). Source SRC-0104; secondary synthesis; inspected 2026-09-08; S2 research use, external claims unverified.
