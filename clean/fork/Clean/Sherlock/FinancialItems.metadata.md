# Profile /2 financial boundary specification

`FinancialItems.lean` follows the host financial route in
`crates/mori-core/src/stage/sherlock2.rs` and its report publisher
`crates/mori-core/src/sherlock/compile_artifact/suspension2_fin.rs` in the
`Moriarty-wt-factory-u` worktree. Those implementation files were in progress
at the time of the first comparison; the then-current branch head was
`0699194f9`. Hashes and verification state are in the accompanying JSON.

The module uses `Init` only. Snapshot, capture and effect types are parameters
representing complete payloads, not alternate truncated host encodings.
`Image` carries the host admission identities, phase policy, outcome, segment,
captures, committed snapshot and ordered effects, logical work, machine fuel
and terminal diagnostics. Work counters use natural numbers with an explicit
UInt128 accounting premise; accounting-effect amounts use `Fin (2^128)`.

The statements cover these boundary contracts:

| Requirement | Statements and limits |
| --- | --- |
| U017 | Successful await/complete publishes the supplied snapshot and ordered prefix plus fee/fallible effects. `accounting` folds the projected log in order with independent saturating payment and fee sums. The literal 30-payment/2-fee calculation yields gross 32, receipt 30, reserve arithmetic 68 and liability arithmetic 10. Statement evaluation producing the snapshot remains an obligation. |
| U018 | Successful await installs the supplied captures, advances the segment and publishes Waiting together with the committed prefix. A false/error guard's `notReady` leaves the image identical even when attempted logical work is nonzero. Capture extraction, guard evaluation, exact checkpoint bytes and comparison with Core/2 remain obligations. |
| U019 | A failure with no fee seal retains the prior committed snapshot/log, discards the fee and fallible deltas in order, clears captures and publishes the specified rejection/exhaustion. Previously charged machine work remains charged, while committed logical work remains unchanged. |
| U020 | All-or-nothing retains the prior prefix. Retain-guaranteed-fees with an actual seal installs the sealed snapshot/log and reports retained fees separately from discarded fallible effects. A failing unsealed fee phase retains nothing new. The seal's production, fees-first admission and target phase correspondence remain obligations. |

`awaitCommit` and `completeCommit` start after successful charge and static
logical-plan checks. They do not assert that an arbitrary input may dispatch.
`localFailure` starts after any machine charges; fuel exhaustion before a
dispatch must supply its unchanged input image. The supplied seal must be
the actual successfully completed fee phase, including the last-fee boundary.
No theorem invents a seal, refund, rollback continuation or ledger action.

At this comparison point the host can publish Waiting with zero remaining
machine fuel, then exhaust on a ready resume. Quint-2 independently reported
that SI-29 drift to the implementation owners. `awaitCommit` mirrors this
current boundary and makes no post-await positive-fuel/resumability claim.
It must be revised if the host adds the design's await-entry resource guard.

The report sums saturate exactly as the current Rust `accounting` function.
Consequently the gross/receipt/fee equality is stated with a saturation bound;
the overflow example demonstrates why unconditional unbounded conservation
would not match the implementation. This is a report arithmetic property,
not per-account, per-asset balance conservation.

No new `sorry` or custom axiom is declared. The companion
`FinancialItemsAxioms.lean` queries every named theorem. Kernel compilation
and per-name dependency results are pending. The sandbox's first capped
invocation failed before Lean started because it could not connect to the
user scope bus. Sherlock-fable subsequently ran a capped build independently;
this module failed to parse because `seal` was used as a reserved binder name.
The binder is now `feeSeal`; a successful rebuild has not yet been observed.
No uncapped build was run. Treat this module as unverified until the JSON
records a successful capped build and actual dependency output.
The shared Lake package now pins Lean 4.33.1; the first failed invocation
selected the previously available 4.34.1 explicitly and never started Lean.

This module does not establish full Rust refinement, statement semantics,
residual-duty extraction, source/plan admission, authenticated/consumed
observations, C6 encoding, in-circuit hashes, handwritten constraints,
recursive proofs, stock-verifier compatibility or concrete ledger-9 phase
evidence. In particular, it cannot close U020's target-evidence requirement.
