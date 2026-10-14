# G3 report

## Identity

Lane G3 continues session `01a139e1-08fe-7590-be4c-7eadb7e8c9ce`.
The requested author is `grok-4.7` at high effort.
The diagnosis records returned usage as `grok-4.7-build`.
That usage name does not establish equivalence to `grok-4.7`.
Parent HEAD is `97e77ea86d3832c642ebade4678f195b4e178775`.
The diagnosis candidate SHA-256 is `42d5134ac8501f0037aa2c48b56c4617e1eb88dd6cb8b1ff5313fa4cd03f3ce4`.
This resume inspected the eight owned files.
Each byte length and SHA-256 matches the diagnosis map.
This resume did not change those files.

Diagnosis path: `/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/G3-headless-ready-terminal-diagnosis.json`.

## Objective

The objective is local source preparation only.
The existing compiler and runtime accept two same-domain actions.
Both actions use one exact base-per-quote price for every participant.
Accepted effects include gross, fee, net, duty, and both asset balances.
The same consumers refuse stale, unfair, overflow, and cross-domain candidates.
This report does not close U6, M14, or M15.

## Owned files

| Path | Bytes | SHA-256 |
| --- | ---: | --- |
| `crates/mori-backend/tests/batch_finance.rs` | 13950 | `765f2b8de75fe3443252e58ca6c346828228460ce73399d2192cc5761eace9a3` |
| `crates/mori-core/tests/batch_finance.rs` | 29073 | `4fcd6a9bbdccd6cc79c42066982b52fd8dbd6f55fa40c71a088128eede661a30` |
| `examples/core/batch-finance/atomic-swap.json` | 1679 | `14661671c6d6f1588204be361a1e30465a7e367599bbffb001ef5d461861855e` |
| `examples/core/batch-finance/batch.mori` | 4463 | `ecfd52d85e1a1e6615f0f7dfc1b0c1b767411c60c6c364470884e3a5e6ae0b39` |
| `examples/core/batch-finance/clear.json` | 2243 | `6f3f7dd6cc618555bae503f3f36e1d6aa23d359a2c48d701c3bc6f57b96da968` |
| `examples/core/batch-finance/mori-tests.json` | 10244 | `3ce3f6ee3d49501055e340f4bd32b60f2bf0030cd1d5445495407cb39d00e991` |
| `examples/core/batch-finance/reject-cross-domain.json` | 2243 | `7c03cec036f6d257405250a1d209cdad5cb1c6f64039244cdeaefb1128e945b2` |
| `examples/core/batch-finance/reject-stale.json` | 2243 | `629838c2e4332c7eacb98fd24c2f42a12fbf9690c60985ba611c5e6d86b5671b` |

The candidate SHA-256 covers these eight files.
These two report files are outside that identity.

## Source and consumers

`examples/core/batch-finance/batch.mori` defines agreement `ExactBatchSwap`.
Action `clear` is a two-participant batch against maker inventory.
Action `swap` is one bilateral exchange between left and right.
The core test parses that file and calls `simulate` and `replay`.
The backend test compiles that same program and calls `ProofPreimage::check`.
`mori test examples/core/batch-finance` reads `mori-tests.json`.
No lane edited shared IR, runtime, exports, Cargo, or CLI dispatch.

## Price and premises

`price` is a supplied `UInt`.
One price unit means one atomic BASE unit for one atomic QUOTE unit.
The caller supplies the quote quantity.
Acceptance requires `baseQty == quoteQty * price`.
The product is exact.
The source does not divide and does not round.
The source does not treat `price` as a certified price type.
`sameDomain`, `oracleAuthorized`, and `solverAuthorized` are supplied Bool premises.
`observedAt` and `now` are supplied UInt premises.
The freshness window is 8 time units.
These premises are not an authenticated oracle, clock, or owner policy.

BASE is network `local`, issuer `base-desk`, reference `BASE`, scale 0.
QUOTE is network `local`, issuer `quote-desk`, reference `QUOTE`, scale 0.
Scale is part of asset identity.

## Accounting

Gross increases by quote notionals plus fees.
Rebates stay outside gross.
Net increases by quote notionals minus fees.
`minNet` applies after fees.
Duty increases by fee minus `settledDuty`.
`clearDuty` is QUOTE owed by maker to reserve.
`swapDuty` is QUOTE owed by left to reserve.
A full settlement of new swap duty leaves a present zero entry.
A present zero entry is different from an absent duty.

## Clear effects

Manifest case `exact-canonical-clear` uses price 4.
Alice pays 3 QUOTE and receives 12 BASE.
Bob pays 5 QUOTE and receives 20 BASE.
Fees are 1 and 2 QUOTE to pool.
Rebates are 2 and 1 QUOTE from maker.
Gross is 11.
Fees are 3.
Net is 5.
Duty is 2.
`clearDuty` moves from 0 to 3 and then from 3 to 2.
BASE sums stay 40.
QUOTE sums stay 30.

## Swap effects

Manifest case `exact-canonical-swap` uses price 7.
`quoteQty` is 6 and `baseQty` is 42.
`leftFee` is 2 and `settledDuty` is 2.
Gross is 8.
Fees are 2.
Net is 4.
Duty is 0.
`swapDuty` moves from 0 to 2 and then from 2 to 0.
The `swapDuty` entry remains.
BASE sums stay 51.
QUOTE sums stay 28.

## Independent cases

Positive cases are `exact-canonical-clear` and `exact-canonical-swap`.
Refusal cases include quote-per-base amounts and unequal participant prices.
Refusal cases include solver cap 31, solver cap 0, stale age 9, and a future observation.
Refusal cases include a false domain, oracle, or solver premise.
A gross cap of 10 refuses the clear fixture.
Rebates would let a netted cap of 10 pass.
A `minNet` of 6 refuses the clear fixture because the fee-adjusted net is 5.
A short Alice QUOTE balance fails at the fee step and leaves the input unchanged.
The same reference with another issuer, reference, scale, or network is an asset refusal.
Boundary cases include an exact solver cap, an exact gross cap, and freshness ages 8 and 0.
A u64 limb product that fits in u128 is accepted.
Multiplication overflow, base-sum overflow, fee-sum overflow, and gross-sum overflow are refused.
The cumulative case runs clear and then swap.
`clearDuty` stays 2.
`swapDuty` ends present at 0.
Combined gross is 19, fees are 5, net is 9, and duty is 2.
A second-action gross cap of 18 leaves that input unchanged.
Relation tamper adds one to post balance, post gross, one effect, post liability, or context.
Each tamper fails the ordinary relation check.
Stale `now` 49 and an overflow public input fail the clear relation.
Target network `other` fails compilation.
Replay of a dropped effect fails.

## Outer gate

This resume did not rerun the checks.
The diagnosis attributes the results to the prior outer gate.
Core `batch_finance` passed 11 tests and failed 0.
Backend `batch_finance` passed 3 tests and failed 0.
The manifest passed 4 cases and failed 0.
Runner seq 159 records `gate_rc` 0 and `report_fresh` false.
The mode is offline locked ordinary reference and relation checking.
The inherited toolchain is `RUSTUP_TOOLCHAIN=1.99.0`.
Native execution was not performed.

## Quint

Quint status is unconfirmed.
The diagnosis text says no public Quint result is in the outer gate.
The same text says no public Quint result is in the permitted final vendor fields.
This resume did not run Quint.
An unconfirmed model check is not a pass.
The Quint model is not a source-refinement certificate.

## Failure history

The author session first failed to compile the backend test.
Rust error E0368 reported that `Fr` has no `AddAssign`.
The repair replaced `+=` with `+` and removed an unused `mut` binding.
The frozen backend hash includes that repair.
The repair is not an open defect.

Vendor queue 2148 then stopped.
`stopReason` is `cancelled`.
The public error is `Error: max turns reached`.
Usage records `grok-4.7-build` with 40 model calls.
The task result is Failed 1.
The public vendor text stopped before either report existed.

The outer gate then passed the fixed checks.
Runner seq 160 records kind `round_incomplete`.
`gate_rc` is 0 and `report_fresh` is false.
The diagnosis calls this a report completion failure.
It is not a test failure.
Both report files were absent at `2026-10-14T10:44:20.811285+00:00`.

Startup `status --json` reported unresolved SP01 loan-swap admission.
That gap blocks the named campaign dispatch.
It does not block this source preparation.
This resume did not edit admission.

## Open gaps

Current Core provides checked add, subtract, and multiply.
Current Core does not provide general directed rounding or price conversion.
Authenticated oracle, clock, and owner policy remain absent.
M3 same-domain native settlement remains open.
M14 full batch remains open.
M15 all supported families remains open.
No native proof, SRS, key, or witness work is claimed.

## Audits and scope

Author self-review is not approval.
Fresh non-author `gpt-6.1-sol` and `grok-4.7` audits remain required.
The diagnosis shows audit count 0.
Inspection found no product defect.
No CORRECT action is requested.
This resume writes only `FOREMAN_REPORT.md` and `FOREMAN_REPORT.json`.
No commit and no push were made.
