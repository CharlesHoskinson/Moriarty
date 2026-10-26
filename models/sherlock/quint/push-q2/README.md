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
| `financialImport.qnt` | `suspension2_fin.rs::{decode_body,resume_fin}`, `sherlock2.rs::{resume,charge}`; a conditional projection of counter import/charge with all other admission premises fixed valid. |
| `financialSegmentImport.qnt` | `suspension2_fin.rs::{decode_body,resume_fin}`, `sherlock2.rs::{resume,run_segment,fee_end}`; conditional projection of pending `FinImage2.segment` vs `FinOutcome2::Waiting{site}` admission and `run_segment` slice/index bounds. |

The financial engine source SHA-256 at intake was
`6054a66cb35e7943f0c18013faf2316c9c9824de3a8b84bef97c8bd6f4f8a744`;
the report adapter `sherlock/compile_artifact/suspension2_fin.rs` was
`5dccd0d846331e0b0242658af5b78bf93b7bcd31502c33d1a642a3b1a59cfd19`.
Both were uncommitted at intake. The engine subsequently changed to
`5f2beded12d8ebd0070a487639300a03f89c3e33b0d3548f46d468dad0e1c08b`
(`check_policy` validating every action segment before start) and then to
`907e5fc7684934f28bb460d84b0d23f95459a74719fda4b48cc05999446eefb5` with
report adapter
`2852e4234df3ecf247728a22d49caccf8ab5482d9e4d438332f18e85d7528dc2`
(enforcing `remaining_work > 0` at await entry and R0 resume for SI-29,
replacing saturating `accounting` with `checked_add -> Option`, and refusing
`resume_epoch == u64::MAX`). Revisit these models when either source changes.

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

The import model assumes canonical public envelope/JSON, matching program, plan,
policy, waiting site and captures, valid Core/2 logical counters and snapshot,
and a true resume guard. Only the host fuel-counter triple varies. R0's
canonicality and each UInt128 field's range are modelled; actual envelope
construction and R1–R7 are fixed successful premises. `ArithmeticOverflow`
records the debug host operation's arithmetic failure boundary, not a published
successor or a claim about release overflow settings. This model does not claim
an end-to-end forged-checkpoint Rust reproduction or an accepted on-chain proof.

The fee-pool wrapper preserves the existing model's finite one-epoch, fixed-seat
domain and external time/auth/membership premises. The upstream driver samples
gross amounts 99/100/111 and the original bounded action inputs. Its invariants
are conditional source accounting, not a claim of universal financial coverage.

## Checks

Run `python3 models/sherlock/quint/push-q2/check_models.py` from the repository.
It typechecks each model, runs its deterministic tests, and samples 10,000 traces
of at most 30 steps with witnesses. It also runs the original fee-pool tests and
reproduces three expected counterexamples. It writes `verification.json` and ITF
traces only inside this directory. The script exits successfully only when
positive checks pass and the three candidate design properties fail specifically
with an invariant violation. The expected failures characterize current code;
they are not acceptance evidence for the violated design requirements.

`financialSegments` uses the Rust evaluator with four threads. The accounting
and fee-pool/import models require `--backend typescript`: Quint 0.32.0's Rust evaluator
rejects integer literals outside i64, including UInt64/UInt128 maxima. Every
sampled check explicitly sets `--max-samples 10000`; passing `--seed` alone makes
this Quint version run only one sample. This is sampled simulation, not exhaustive
model checking. No `quint verify` or Rust build was run for these artifacts.

## Reported disagreements

1. **Await-entry starvation rollback vs commit-first (design §6.1 / §7.3 / E2x).**
   `sherlock2.rs:319-325` (`907e5fc7`) repaired SI-29 (`waitingResumableInv` now
   holds in `allInvariants`): F01 with fuel 4 no longer publishes `Waiting` with
   `remaining_work == 0`. However, at `end < statements.len() && remaining == 0`
   it calls `fail(...)` before committing `tentative`, rolling back the prefix
   (or retaining only `fee_stage`), whereas design §6.1 / §7.3 step 4 / §11.2
   `E2x` specify that `tentative` is committed first before publishing
   `Exhausted(Fuel)`. Characterization: `zeroFuelWaitingExhaustsAtAwaitEntryTest`;
   candidate `awaitStarvedCommitsPrefixInv`, trace `waiting-zero-fuel.itf.json`.
2. **Checked aggregate report over unbounded transfer history (U017).**
   `sherlock2.rs::accounting` (`907e5fc7`) and `suspension2_fin.rs::fin_block`
   (`2852e423`) replaced silent `saturating_add` with `checked_add -> Option`:
   `exactReportInv` and `reportEquationInv` (`gross == payments + fees`) now hold
   in `allInvariants` whenever `reportOk` holds. Valid `u128` self-transfers or
   multi-account round trips can still exceed `u128::MAX` cumulative transfer
   volume without any balance overflow, where `accounting` now returns `None`
   (`reportOk == false`). Characterizations:
   `checkedGrossOverflowRefusedTest`, `roundTripOverflowRefusedTest`;
   candidate `unboundedHistoryFitsU128Inv`, trace `accounting-overflow.itf.json`.
3. **Unvalidated imported work counters (financial R0/SI-03).** Under the import
   model's fixed valid non-counter premises, canonical fields initial = 0,
   remaining = 1, consumed = 0 pass import and charge, although
   remaining + consumed ≠ initial. Canonical initial = MAX, remaining = 1,
   consumed = MAX also passes import and reaches the unchecked `consumed += 1`
   arithmetic boundary. The report adapter's `decode_body` checks canonical
   encoding; later identity/logical-plan/snapshot checks never jointly validate
   this counter triple. Characterizations:
   `canonicalMalformedCountersAcceptedTest`, `maximumConsumedCounterAcceptedTest`;
   candidate `acceptedWorkInv`, trace `financial-import-counters.itf.json`.
   Runtime host confirmation (`probe_financial_import.py`, recorded in
   `financial-import-rust-probe.json`): resealing `FinImage2` with
   `initial_fuel = 0, remaining_work = 5, consumed_work = 0` halts (`exit = 0`);
   resealing with `consumed_work = u128::MAX, remaining_work = 1` returns
   `not_ready` (`exit = 3`) on a false guard and panics in debug mode at
   `crates/mori-core/src/stage/sherlock2.rs:217` (`attempt to add with overflow`,
   `exit = 101`) on a true guard.
4. **Unvalidated imported pending segment index (financial R0/R1).** `FinImage2`
   carries both `outcome = Waiting{site}` and `segment: usize`. On resume, R1,
   R2, R6 and R7 check `site`, while `run_segment` indexes
   `segments(sp, action)[image.segment]` and `fee_end` slices
   `statements[begin..end]` with `begin = ordinal(site) + 1`. Neither
   `decode_body` nor `resume` checks `image.segment == usize::from(site)`.
   Characterizations: `outOfBoundsSegmentNotReadyTest`,
   `outOfBoundsSegmentPanicsOnAcceptTest`,
   `staleSegmentZeroRetainFeesPanicsTest`,
   `staleSegmentZeroDefaultPolicyFailsPlanCostTest`; candidate
   `acceptedSegmentInv`, trace `financial-import-segment.itf.json`. Runtime host
   confirmation (`probe_financial_import.py`): resealing `segment = 2` returns
   `not_ready` (`exit = 3`) on a false guard and panics at
   `sherlock2.rs:260` (`index out of bounds`, `exit = 101`) on a true guard;
   resealing `segment = 0` under `retain-guaranteed-fees/1` panics at
   `sherlock2.rs:137` (`slice index starts at 4 but ends at 3`, `exit = 101`).

All findings were sent to `sherlock-fable`, `sherlock-u2`, `sherlock-c4` and
`sherlock-lean-2`. Host checkpoint cases are runtime-confirmed read-only via
`probe_financial_import.py` (`financial-import-rust-probe.json`); no accepted
on-chain proof is claimed. The checked existing `account_effects`
adapter distinguishes exact amounts from overflow; the newer host aggregate
silently saturates. Concrete target phase evidence remains pending, as U020
explicitly states.
