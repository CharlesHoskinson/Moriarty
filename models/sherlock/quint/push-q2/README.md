# Financial boundary models (U017–U020)

These are finite executable abstractions of the host financial route and the
existing fee-pool component. They do not establish Rust refinement, circuit
coverage, ledger settlement, or stock-verifier acceptance. The working tree
containing these models is based on `0699194f9`; the newer financial route was
read without edits in the owning `Moriarty-wt-factory-u` worktree.

## Source map and assumptions

| Model | Source and properties |
| --- | --- |
| `financialSegments.qnt` | `crates/mori-core/src/stage/sherlock2.rs::{run_segment,fail,charge,resume,check_policy}`; ordered committed effects, F01 100/40 → 68/10 → 57/0, local rollback, sealed fee retention on rejection/exhaustion, fuel conservation. |
| `financialAccounting.qnt` | `crates/mori-core/src/runtime.rs::move_funds`, `helper_machine/shared/fin_carrier.rs::apply_transfer`, `stage/sherlock2.rs::accounting`; exact UInt128 checked transfers, self transfers, asset isolation and saturating aggregate reports. |
| `feePoolHistory.qnt` | Imports `specs/quint/fee_pool_component.qnt` unchanged; adds independent history replay for cash and residual liabilities, matching transfer/discharge totals and reserve coverage. |

The financial engine source SHA-256 at intake was
`6054a66cb35e7943f0c18013faf2316c9c9824de3a8b84bef97c8bd6f4f8a744`;
the report adapter `sherlock/compile_artifact/suspension2_fin.rs` was
`5dccd0d846331e0b0242658af5b78bf93b7bcd31502c33d1a642a3b1a59cfd19`.
Both were uncommitted at intake. The engine subsequently changed to
`5f2beded12d8ebd0070a487639300a03f89c3e33b0d3548f46d468dad0e1c08b`:
`check_policy` now validates every action segment before start rather than
checking each segment as it is entered. `actionAdmits` and two explicit tests
track this change, including an invalid unreached suffix. Segment execution,
fuel and saturating accounting are unchanged. Revisit these models when either
source changes.

Requirements come from the U017–U020 sections of
`openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md`
and `docs/sherlock/design/suspension-and-finance.md` §6–8. The U017–U020 JSON
contracts named by some holdout instructions were absent at intake.
No holdout was modified, and no U predicate is claimed to pass here.

The evaluator subsequently published `qa/holdout/contracts/U020.json` on the
owner branch in `c0fb7664d`. Read-only comparison confirms the same policy names,
policy binding on start/resume, separated committed/retained/discarded effects,
residual duties and `ledger_rollback: none-inferred` used here. The contract
permits refusal either at admission or segment entry; the current model's
`actionAdmits` follows the implementation's whole-action check. Report JSON shape and exit
codes remain adapter obligations, outside these typed accounting models. The
financial engine/report hashes at intake were unchanged when this contract arrived;
the whole-action check was added afterwards.

The segment model assumes an action satisfying `actionAdmits`, admitted
well-typed scalar statements and a valid
static Core/2 cost plan. Its program alphabet is transfer, fee, discharge and
requires. One asset, three distinct accounts and one present liability start
with reserve 100 and duty 40. Discharge changes the duty separately from payment;
zero liability remains a snapshot field, while positive duties form the residual.
Let/capture/emit/state semantics, actual expression costs, canonical encoding,
admission hashes, R0–R7 and chain consumption are outside this model. There is
one image variable, so checkpoint publication is atomic. `cuts` and `notReady`
are observer counters, not checkpoint fields. Mid-segment cuts observe staged
state; the current host driver only persists completed invocation images.
`committedEvents` is a ghost counter of host events at the last successful
commit, not Core/2 `committed_logical` (which counts expression work). Host fuel
costs match the inspected implementation: one per statement/commit/accept.

The accounting model assumes declared asset/account identities and existing
balance rows. It uses actual `u128::MAX`, two assets and three accounts, with
initial supply MAX per asset. Each operation is atomic. Sampling picks amounts
0, 1, MAX−1 and MAX; exact history sums use mathematical integers. No ledger
coin custody or authenticated source is inferred.

The fee-pool wrapper preserves the existing model's finite one-epoch, fixed-seat
domain and external time/auth/membership premises. The upstream driver samples
gross amounts 99/100/111 and the original bounded action inputs. Its invariants
are conditional source accounting, not a claim of universal financial coverage.

## Checks

Run `python3 models/sherlock/quint/push-q2/check_models.py` from the repository.
It typechecks each model, runs its deterministic tests, and samples 10,000 traces
of at most 30 steps with witnesses. It also runs the original fee-pool tests and
reproduces two expected counterexamples. It writes `verification.json` and ITF
traces only inside this directory. The script exits successfully only when
positive checks pass and the two candidate design properties fail specifically
with an invariant violation. The expected failures characterize current code;
they are not acceptance evidence for the violated design requirements.

`financialSegments` uses the Rust evaluator with four threads. The accounting
and fee-pool models require `--backend typescript`: Quint 0.32.0's Rust evaluator
rejects integer literals outside i64, including UInt64/UInt128 maxima. Every
sampled check explicitly sets `--max-samples 10000`; passing `--seed` alone makes
this Quint version run only one sample. This is sampled simulation, not exhaustive
model checking. No `quint verify` or Rust build was run for these artifacts.

## Reported disagreements

1. **Waiting without fuel (design SI-29).** F01's three statements with fuel 4
   followed by await commit publish reserve 68/duty 10 and `Waiting` with fuel 0.
   A ready resume then exhausts before progress. `waitingResumableInv` checks
   even the weaker necessary condition `Waiting ⇒ remaining > 0`; the design
   requires enough for accept plus guard evaluation. The host `run_segment`
   does not apply the design's post-commit resumability check. Characterization:
   `zeroFuelWaitingCounterexampleTest`; trace `waiting-zero-fuel.itf.json`.
2. **Silent aggregate saturation (U017).** Two valid self transfers, payment MAX
   then fee MAX, preserve every balance yet report gross = payments = fees = MAX.
   Thus gross ≠ payments + fees, and exact sums are lost without an overflow
   marker. A distinct-account MAX round trip also exceeds MAX cumulative payment
   without any balance overflow. Characterizations:
   `saturatedGrossCounterexampleTest`, `roundTripOverflowCounterexampleTest`;
   candidate `exactReportInv`, trace `accounting-overflow.itf.json`.

Both findings were sent to `sherlock-fable`, `sherlock-u2`, `sherlock-c4` and
`sherlock-lean-2`. These are model/source disagreements, not claims that a Rust
integration reproduction was executed. The checked existing `account_effects`
adapter distinguishes exact amounts from overflow; the newer host aggregate
silently saturates. Concrete target phase evidence remains pending, as U020
explicitly states.
