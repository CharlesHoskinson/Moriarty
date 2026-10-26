# Profile /2 financial boundary specification

`FinancialItems.lean` follows the host financial route in
`crates/mori-core/src/stage/sherlock2.rs` and its report publisher
`crates/mori-core/src/sherlock/compile_artifact/suspension2_fin.rs` in the
`Moriarty-wt-factory-u` worktree. Those implementation files were in progress
at the time of the first comparison; the then-current branch head was
`0699194f9`. Hashes and verification state are in the accompanying JSON.
The current source comparison includes the later in-progress change at
`c0fb7664d`: retain-fees admission checks every action segment up front.

The module uses `Init` only. Snapshot, capture and effect types are parameters
representing complete payloads, not alternate truncated host encodings.
`Image` carries the host admission identities, phase policy, outcome, segment,
captures, committed snapshot and ordered effects, logical work, machine fuel
and terminal diagnostics. Work counters use natural numbers with an explicit
UInt128 accounting premise; accounting-effect amounts use `Fin (2^128)`.

The statements cover these boundary contracts:

| Requirement | Statements and limits |
| --- | --- |
| U017 | Successful await/complete publishes the supplied snapshot and ordered prefix plus fee/fallible effects. `accounting` checks each payment/fee accumulation and their final sum; overflow yields no report. A successful report has exact gross = receipt + fees and fits UInt128. The literal 30-payment/2-fee calculation yields gross 32, receipt 30, reserve arithmetic 68 and liability arithmetic 10. Statement evaluation producing the snapshot remains an obligation. |
| U018 | Successful await requires remaining fuel for resume, installs supplied captures, advances the segment and publishes Waiting with the committed prefix. Zero post-dispatch fuel instead enters the local Fuel-exhaustion path. A false/error guard's `notReady` leaves the engine image identical even when attempted logical work is nonzero; successful checkpoint publication also requires successful accounting. Capture extraction, guard evaluation, exact checkpoint bytes and Core/2 comparison remain obligations. |
| U019 | A failure with no fee seal retains the prior committed snapshot/log, discards the fee and fallible deltas in order, clears captures and publishes the specified rejection/exhaustion. Previously charged machine work remains charged, while committed logical work remains unchanged. |
| U020 | All-or-nothing retains the prior prefix. Retain-guaranteed-fees with an actual seal installs the sealed snapshot/log and reports retained fees separately from discarded fallible effects. A failing unsealed fee phase retains nothing new. The seal's production, admission correspondence and target phase correspondence remain obligations. |

`admitActionPolicy` mirrors the current whole-action `check_policy` scan.
Under the retain policy a fee after any statement other than Fee/Let/Requires
rejects admission, including in a segment that execution would never reach.
Let and Requires do not mark a segment as having entered its fallible phase.
All-or-nothing imposes no fees-first restriction. Exact source partitioning
into segments and the classification of Rust statements remain correspondence
obligations. These are syntactic checks, not proofs that a Requires succeeds.

`awaitCommit` and `completeCommit` start after successful charge and static
logical-plan checks. They do not assert that an arbitrary input may dispatch.
`localFailure` starts after any machine charges; fuel exhaustion before a
dispatch must supply its unchanged input image. The supplied seal must be
the actual successfully completed fee phase, including the last-fee boundary.
No theorem invents a seal, refund, rollback continuation or ledger action.

The current host fixes the earlier zero-fuel Waiting case: `awaitEnter`
models its new post-dispatch fuel guard, while `awaitCommit` requires
positive remaining fuel. A failed guard follows the existing local-failure
policy, including any sealed fee phase. Resume separately refuses an
imported zero-fuel Waiting image at R0. These are the host's current fuel
checks, not a proof of every resource row of the design's SI-29 table.

The current Rust `accounting` function replaces its earlier silent
saturation with checked UInt128 sums. The Lean result is therefore optional;
`accounting_success` proves exact report arithmetic and its gross bound from
successful publication. Three examples refuse gross, payment-accumulator
and fee-accumulator overflow. The publisher maps failure to an `accounting`
input error rather than emitting a saturated report or checkpoint. These
are report arithmetic properties, not per-account, per-asset balance
conservation or a guarantee that every otherwise valid action can publish.

No new `sorry` or custom axiom is declared. The companion
`FinancialItemsAxioms.lean` queries every named theorem. The current
fuel-guard/checked-accounting source in `284c563f1` passed the coordinator's
capped 17-job Lean 4.33.1 build. Lake reported 907 ms for this module and
224 ms for its query. This agent inspected the complete build log and checked
that the source bytes match the commit; it did not execute the build.
Its own capped invocation cannot connect to the sandbox user scope bus,
and no uncapped build was run.

The actual twenty-four-name query reports twelve empty axiom sets, eight
`propext`-only sets, three `propext`/`Quot.sound` sets and one
`propext`/`Classical.choice`/`Quot.sound` set. The last is
`charge_stays_uint128`; the three quotient-dependent statements are the two
general admission theorems and `charge_preserves_work`. The JSON records
every per-name set and log digest. No custom or `sorryAx` dependency was
observed. Prior nineteen-name metadata remains available in `0c1020244`.

`WorkAccounting` is a theorem premise, not a guarantee currently supplied by
financial checkpoint import. In the pinned host source, `decode_body`
checks canonical `FinImage2` JSON, `same_admission` checks program/plan/policy
identity, and the resume preflight checks positive fuel and snapshot predicates.
Its `canonical_boundary` call receives captures but ignores them; the separate
legacy `stage::admit` capture-layout check is not thereby supplied to this route.
The carrier also refuses a maximal UInt64 resume epoch and uses checked epoch
increment; the epoch/header is outside this module's `Image` projection. These
checks do not establish the full machine-counter balance or the segment/site
relation used by execution. Individually valid UInt128 values such as
remaining 1 and consumed `uint128Max` violate the accounting premise, so
the headroom theorem cannot be applied to that imported image. The older
decoded-counter adapter and staged-counter module concern a separate owner
path and do not supply this financial premise. These are source-audit
observations, not an executed Rust or forged-checkpoint test.

The mathematical record updates also preserve diagnostics unless explicitly
updated: in particular, `completeCommit` does not erase pre-existing
`discardedEffects` or `retainedByPolicy`. Thus a malformed Waiting image
carrying terminal diagnostics is not proved to satisfy SI-26 by these lemmas.
Capture-layout validation and terminal-diagnostic admission are additional
full-image obligations, not consequences of canonical JSON decoding.

The separate diagnostic committed by Quint-2 in `5c71be223` records synthetic
counter and segment imports reaching panics in an older pinned host binary.
Its source hashes precede the current fuel/accounting changes. This agent
inspected that committed report, but did not execute it; the report is not a
test of the current binary. The JSON records its digest and historical source
pins separately from the current source audit and Lean kernel observations.

This module does not establish full Rust refinement, statement semantics,
residual-duty extraction, source/plan admission, authenticated/consumed
observations, C6 encoding, in-circuit hashes, handwritten constraints,
recursive proofs, stock-verifier compatibility or concrete ledger-9 phase
evidence. In particular, it cannot close U020's target-evidence requirement.
