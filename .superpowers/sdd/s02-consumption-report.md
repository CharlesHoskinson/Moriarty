# S02 parent consumption foundation report

## Status and scope

The bounded exact-parent bookkeeping foundation is implemented in the assigned
worktree at base commit `7d1517c3dbe830976dbd5224b8a6a4dcad7024c3`.
It models one nonce entry for a fixed ten-unit, two-slot installment parent.
Each slot pays five units to Bob. The entry stores the complete generic parent,
claims that parent on a fill or unused cancellation, tracks monotone revisions,
and preserves `paid + remainingAllowance = 10`.

This result is not an authorization, cancellation-authority, signature,
financial-transfer, recovery, registry-ownership, candidate, architecture,
S02-gate, Apalache, or TLC claim. Cancellation only revokes remaining parent
payment bookkeeping. It does not release or move money and does not authorize
recovery.

## Files

Created model files:

- `specs/quint/s02/consumption.qnt`
- `specs/quint/s02/consumption_harness.qnt`
- `specs/quint/s02/consumption_test.qnt`

Appended the required bounded-scope note to `specs/quint/s02/README.md`.
Created raw receipts, stage snapshots, two ITF paths, and a manifest under
`evidence/s02-model-comparison/consumption/`.

No effects source, S01 artifact, Core, swap, backend, E00, XML, OpenSpec, or
other repository source was edited.

## Implemented interface

`consumption.qnt` imports `Principal`, `Asset`, and `Location` from the existing
`effects.qnt`. It defines generic `Parent[p]` and `Entry[p]`, plus
`validEntry`, `canConsumeSlot`, `applyConsumeSlot`, `canCancelParent`,
`applyCancelParent`, and `validResidual`.

The fixed parent uses the `InstallmentDomain`, Alice nonce zero, Alice's TokenA
escrow, Bob as recipient, TokenA, budget ten, and the exact map
`Map(1 -> 5, 2 -> 5)`. The policy parameter is structurally compared but not
interpreted. The harness policy is only an equality fixture.

The entry's claim is either `Unclaimed` at the exact initial state or
`Claimed(parent)` after at least one fill or cancellation. Slot two requires
slot one. Duplicate, missing, and out-of-order slots are rejected. Paid value
is recomputed from used slots, remaining allowance is exact, and revision is
the used-slot count plus one when cancelled. No revision wraps.

Both fill and cancellation guards require the prepared entry to equal the
current entry. Therefore, either winner invalidates the loser's stale prepared
snapshot. Cancellation is disabled after full fill and after cancellation.

## Test-first and incremental evidence

The sixteen-test module was created before either implementation module. The
first required command exited `1` with `QNT013` and `QNT405` missing-module
diagnostics. The exact test snapshot, cwd, argv, raw stdout/stderr, exit code,
and hashes are in `evidence/s02-model-comparison/consumption/red/`.

The pure interface was then added with compile-only stubs. That signature
snapshot typechecked before the predicates were implemented. The completed
helpers typechecked, and a hand-built REPL fixture printed the complete initial,
first-fill, and unused-cancellation entries. The helper checks returned true
for initial validity, exact first-fill residual, cancellation validity, and
post-cancellation fill denial.

The harness was added in three preserved stages:

| Stage | Result | Witness counts |
| --- | --- | --- |
| First fill only | typecheck and run exit `0` | `firstFilled` 1000/1000 |
| Second fill added | typecheck and run exit `0` | `firstFilled` 1000/1000; `fullyFilled` 1000/1000 |
| Cancellation added | typecheck and run exit `0` | first 471, full 233, unused cancel 529, residual cancel 238 of 1000 |

Each stage has its exact harness snapshot, source digest, cwd, argv, exit codes,
and separate raw streams in its `stage-*` evidence directory.

## Final checks

Quint was `/home/charl/.npm-global/bin/quint` version `0.32.0`, SHA-256
`ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501`.
Node was `v24.18.1`; Python was `3.13.14` from the requested virtual
environment.

The three final Quint typechecks exited `0` with empty stdout and stderr. The
required test command used `--match 'Test$'`, exited `0`, and discovered exactly
sixteen passing tests.

The specified seed-42 run used 10,000 samples and at most four steps. It found
no `safety` counterexample in that sample. All four targets were nonzero:

- `firstFilled`: 4947/10000
- `fullyFilled`: 2471/10000
- `cancelledUnused`: 5053/10000
- `cancelledResidual`: 2476/10000

The two negated-witness runs intentionally exited `1` with `Invariant violated`.
Those exits are expected reachability evidence, not safety failures. The unused
cancellation ITF has two states and ends with the complete claimed parent,
`paid=0`, `remainingAllowance=10`, `revision=1`, no used slots, and
`cancelled=true`. The residual cancellation ITF has three states and ends with
the complete claimed parent, `paid=5`, `remainingAllowance=5`, `revision=2`,
used slot one, and `cancelled=true`.

The requested focused Python test reported `3 passed`. The S01 validator
recomputed S01-01 through S01-10 as true. The worker's pre-staging
`git diff --check` exited `0`; it did not include the untracked receipts.
All exact final commands, raw streams, exit codes, output hashes, and ITF hashes
are recorded in `evidence/s02-model-comparison/consumption/final/commands.txt`
and `manifest.json`.

## Controller verification and integration hold

The controller committed the candidate at
`8b905114c1cec79faf555974c0267f183ca31399`, without integrating it into main.
The controller separately reran all three typechecks, sixteen Quint tests,
the full Python suite (`285 passed in 8.95s`), and the ten S01 checks.
All these commands exited zero. The controller also checked all four source
digests, both ITF digests, and both complete terminal states.

The staged full-diff whitespace check exited `2`. A fresh
`git diff 7d1517c 8b90511 --check` reproduces trailing-space findings only in
`pure/repl.stdout.txt`, `pure/repl-corrected.stdout.txt`, and
`pure/repl-final.stdout.txt` under the consumption evidence directory. These
are raw Quint output bytes. They are preserved unchanged. The source-only
`git diff 7d1517c 8b90511 --check -- specs/quint/s02` exits zero. Do not
describe the complete candidate diff as whitespace-clean or treat the earlier
unstaged check as coverage of newly added receipts. The controller's initial
command batch committed the candidate after the check failed; that commit is
not an acceptance decision.

The implementation remains pending the requested three-provider Council review.
The Council runtime binding gap and model-identity policy decision are recorded
on main in `docs/superpowers/reviews/2026-09-05-council-runtime-binding-intake.md`.
No package gate or integration approval follows from this verification.

## Diagnostics and corrections

Quint does not re-export the imported `Alice` constructor through the harness
or consumption module. The first final test typecheck exited `1` with `QNT404`
at the altered-parent test. The permitted syntax/import correction added a
direct `effects.*` import to the test module. No test predicate changed.

The first non-interactive pure-helper REPL command also showed that loading the
module from the repository root caused `./effects` to resolve from Quint's
synthetic `<modules>` location. Quint returned process status zero despite those
diagnostics. Running from `specs/quint/s02` fixed import resolution. A multiline
record then remained at continuation prompts, so the same hand-built fixture
was formatted as one expression per line. The final transcript executed every
check. All three transcripts and input files are retained, and only the final
transcript is treated as helper evidence. No predicate was weakened.

## Source hashes

| File | SHA-256 |
| --- | --- |
| `specs/quint/s02/consumption.qnt` | `3dd07f3c5f274aff4fce4de9c245aa0b1f1fe7f68e29e5de95a0b1c42a00f93b` |
| `specs/quint/s02/consumption_harness.qnt` | `41440c269e7a42c597d41fede9fe3583177c3f2ae4118b0f788806666911eb0e` |
| `specs/quint/s02/consumption_test.qnt` | `0d561cdc75d142997a7fb2373570266d314b927ce67b15c74eca665a71801943` |
| `specs/quint/s02/README.md` | `9747b601b425047a7e772a28efdc76d3383e5535fe78ccec12ab85ecaa4cb8fb` |
| `specs/quint/s02/effects.qnt` (unchanged) | `dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c` |

## Limitations

The runs are bounded random simulation and deterministic scenario tests. They
are not exhaustive model checking or proofs. The helper guards accept only the
fixed fixture parent shape and do not authenticate policy content. This file
models one entry, not registry key uniqueness or ownership. It has no signing
event, balance state, transfer, recovery transaction, candidate execution, or
Core correspondence. A cancellation witness here does not satisfy the required
S02 recovery subscenarios or any package gate.
