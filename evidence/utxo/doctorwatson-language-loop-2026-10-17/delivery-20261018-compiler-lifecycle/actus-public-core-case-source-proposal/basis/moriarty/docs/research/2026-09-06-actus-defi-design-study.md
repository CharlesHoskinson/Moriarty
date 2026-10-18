# ACTUS and DeFi requirements for the Moriarty design

Date: 2026-09-06. Status: S2 source study and design requirements; not language
implementation, protocol certification or all-vector conformance. Two bounded
research lanes inspected ACTUS and DeFi independently; the lead agent reconciled
the findings, checked decisive source passages and generated complete row
inventories. No compiler, model checker, prover or native campaign ran.

The study supports a shared bounded transition semantics with typed financial
packages. It also establishes why a transfer-only Core demonstration is not
enough: interest, loss allocation, authority, ordering and external dependencies
change the meaning of otherwise plausible balance movements.

## Coverage and source identity

| Inventory | Actual scope |
| --- | --- |
| [ACTUS matrix](../../evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv) | All 32 dictionary taxonomy rows have one Moriarty disposition. All 18 executable types have inspected techspec sections and representative fixtures. 276 type fixtures plus one ANN analysis-date fixture are inventoried; none was executed in this sprint. |
| [DeFi matrix](../../evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv) | All 72 original product rows have exact source JSON pointers, family requirements, individual deltas and explicit model omissions. Representative family source was inspected; this is not a full independent source-code audit of all 72 protocols. |
| [Source manifest](../../evidence/moriarty-design-sprint-2026-09-06/source-manifest.json) | Full repository pins, file hashes, read-only inventory scope and worked-example provenance. |

ACTUS tests are pinned at `f7a8064872b69db1f0beabac771c99dc3ce0c397`,
dictionary at `356f7663f26091105cc4fef4ae3496942dcf0ebf`, techspec at
`94ef09e4992f79d573f84f41d8480f557365870e`, and comparative Haskell at
`42451170dc61c5c11c4144bd5a69046f149e8016`. DeFiFormal is pinned at
`8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Claims concern these source versions,
not the current behavior of live named protocols.

The DeFi crosswalk's inherited 60 included/12 absent construction classifications
and 36 yes/33 no/3 conditional kernel flags remain separate from proposed
language coverage. No row is marked implemented or verified. The shared
`quint-models-v2/kernel.qnt` provides arithmetic helpers; it is not an implemented
semantic kernel for every financial product.

## ACTUS requirements by executable type

The matrix contains exact type-specific source locators, fixture counts,
capabilities, pitfalls and remaining gaps. The following groups explain the
common machinery they require:

| Types | Financial behavior that must survive elaboration |
| --- | --- |
| PAM, LAM | Accrual, resets, principal redemption, interest base, caps/floors, derived maturity and final residuals. |
| LAX, NAM, ANN | Array schedules and changing principal; negative amortization without clamping; fixed installments versus recalculated annuities. |
| CLM, UMP | Calls/notices and unscheduled principal events, with explicit finite horizons and observation bounds. |
| CSH, STK, COM | Zero-payoff analysis events; dividends; quantity and price; signed purchase versus termination. |
| FXOUT, SWPPV | Currency/leg orientation, gross versus net settlement, distinct accruals and fixed/floating events. |
| SWAPS, CAPFL | Bounded child composition; stable merge and identity; paired child evaluation with controlled term overrides. |
| OPTNS, FUTUR | Exercise versus settlement; option positive-part versus signed futures payoff; observed underlying and fixing time. |
| CEG, CEC | Referenced exposure, credit triggers, collateral value, coverage caps, valuation time and settlement lag. |

Calendar functions must preserve scalar/array cycles, stubs, end-of-month rules,
business-day adjustment, distinct calculation/payment dates and event precedence.
Numeric functions must preserve scale, rounding, sign, year fraction, annuity,
quantity and conversion rules. A supplied schedule, observation or underlying
value is not an unconstrained witness that can decide the result arbitrarily.

All nine present result-field kinds matter: date, event type, payoff, currency,
principal, interest rate, accrued interest, exercise amount and exercise date.
The largest expected result list has 241 rows; that source observation is not a
universal event bound. The comparative Haskell implementation is useful evidence,
but its excluded cases and narrower/Float32 comparator prevent using its test
harness as a complete oracle.

The 14 non-vector taxonomy nodes retain their own dispositions. BCS has a
retained descriptive specification/example without a reference vector and with
completeness gaps. NAX and REP are planned. CDSWP, MAR, BNDCP, CLNTE, EXOTi,
ANX, PBN, SCRCR, SCRMR, TRSWP and BNDWR remain taxonomy-only in this inspected
executable corpus. An upstream “Implemented” label does not establish Moriarty
implementation or supply a missing executable vector.

## Source discrepancies and dispositions

These are source observations followed by explicit proposed dispositions. An
interpretation supported by a fixture is not yet an executed conformance result.

| ID | Evidence | Disposition and remaining acceptance obligation |
| --- | --- | --- |
| DS-01 | TEX schedule section says non-working day; dictionary and Haskell `DateShift.hs:33–103` shift to business days. `pam08` CSF and `pam09` SCF move payment to April 1 but report interest 26.6666666666667 versus 27.5. | Propose business-day semantics and preserve calculation-before/after-shift. Use the contrast as a required regression, then independent all-field comparison. |
| DS-02 | Dictionary `calculateShiftModifiedPreceding` repeats SCMP; TEX and Haskell distinguish SCMP/CSMP. No CSMP fixture was found in the bounded pass. | Preserve raw tokens and typed identifiers; propose CSMP for that identifier without reinterpreting existing SCMP fixtures. Add independently sourced convention tests before conformance. |
| DS-03 | ANN initial `Prnxt` TEX formula around line1688 contains `todo/todo`. | Public TEX is incomplete at this point. Derive an explicit initialization rule from the annuity definition, pinned comparative implementation and fixtures, and check independently before implementation acceptance. No invented completed formula in this sprint. |
| DS-04 | COM TD table points to PRD/STK; `com01` and Haskell `Payoff.hs:319/387` include quantity and opposite purchase/termination signs. | Propose signed quantity × unit price with purchase/termination direction. Preserve erratum and independently check all COM cases. |
| DS-05 | CLM references PR/PAM although PAM has no PR schedule; surveyed Haskell excludes CLM cases. | Keep call/redemption semantics unresolved where this matters. A complete callable package needs an explicit schedule/state rule and all-case comparison. |
| DS-06 | `fxout01` uses two MD result records where TEX names settlement-event variants. | Preserve vector event identities; reconcile the representation with the package rule before claiming FXOUT compatibility. No silent event renaming. |
| DS-07 | FUTUR taxonomy describes margining breadth beyond sampled TEX/vector lifecycle; BNDCP name and callable/puttable description differ. | Preserve taxonomy breadth separately. Futures settlement does not certify a margin-account package; do not label BNDCP a proved convertible-note implementation. |

Lossless import and numeric profiles are an additional cross-cutting obligation.
For example, the first `lam01` interest expression is an exact rational, whereas
the expected trace uses a finite decimal. A chosen representation/tolerance
must be justified per field before running a compatibility gate. The design
does not choose a convenient global tolerance to make the corpus pass.

## DeFi behavior families and their limits

| Family | Required state and properties | Boundary exposed by source inspection |
| --- | --- | --- |
| F1 exchange | Reserves, LP positions, fees, slippage, orders and fills | Uniswap constant product cannot stand in for Curve convergence, concentrated positions, hooks or order auctions. |
| F2 credit | Debt/supply shares, accrual, collateral, liquidation and loss bearer | Morpho's supplied interest increment does not establish a correct interest-rate model. Fluid combines exchange and debt in one position. |
| F3 derivatives | Signed positions/PnL, funding or borrow fees, scenario margin, expiry and reservation | The named Hyperliquid model covers Bridge2 only. Derive takes an empty-option path. Hegic's supplied payoff does not implement its pricer. |
| F4 consensus claims | Operator allocation, slash liability, rewards and exit queues | Lido/EigenLayer abstractions omit parts of validator, delay and proof behavior; custody and consensus remain named dependencies. |
| F5 external claims | Issuer/obligor, eligibility, backing statements, redemption and registry | WBTC separates requests from custodian confirmation. A model variable called reserve is not proof of external reserves. |
| F6 management | Investor claims, role-scoped mandates, caps, strategy accounting and ordered redemption | MetaMorpho omits cap timelock/fees/guardian/underlying accrual; Huma requires junior-loss and senior-redemption ordering. |
| P conditional claims | Conditions, partitions, payout rules, resolution and single redemption | Polymarket's model does not automatically cover Kalshi clearing or Azuro pool liabilities, and has its own dispute/negative-risk omissions. |
| Infrastructure | Domain-separated receipts, asset topology, consumption, timeout and finality | Bridge settlement and relayer reimbursement can finalize separately; they are not one local atomic transfer. |

Each of the 72 CSV rows adds its own requirement delta and exact source pointer.
These deltas are inherited corpus evidence interpreted for design, not newly
verified current protocol facts. Shared families are an index for study and
packages; they are not seven generic correctness proofs.

The strongest cross-family findings are that rounding changes the beneficiary
of value, ordering changes policy outcomes, and authority checks can disappear
inside abstractions. The language must make those distinctions observable.
Examples include first-minimum index selection in Derive, ordered maker fills
in Polymarket and senior-before-junior redemption in Huma. A set or aggregate
balance cannot replace the ordered computation.

## Proposed boundary and design consequences

Use bounded typed data, checked operations, state transitions and exact effects
as shared machinery. Put amortization, pricing, liquidation, allocation, exercise
and calendar rules in versioned packages where expressible. Represent external
prices/rates, outcome resolution, custody, registry, validator events, execution,
governance and cross-domain finality as explicit capability profiles.

PCD should certify execution of those exact package rules from admissible
predecessors. It must not certify a simplified model while presenting the omitted
financial behavior as covered. A contract's finite analysis/execution envelope
must also be explicit: open-ended accounts, perpetuals and pools need finite
epochs or another reviewed bounded scope. Renewal cannot silently extend a
lifetime theorem.

The [semantic proposal](../superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md)
and [developer interface](../superpowers/specs/2026-09-06-moriarty-developer-interface-design.md)
apply these requirements. The row inventories are complete; package algorithms,
source-gap resolutions, all-field numeric conformance and deployment feasibility
remain the explicitly identified work needed before implementation acceptance.
