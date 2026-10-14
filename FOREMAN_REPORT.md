# FOREMAN_REPORT — G2 author

Lane G2 has a local sponsored-preparation candidate. The prior outer gate passed 12 Rust tests. This resume did not rerun those tests. No milestone is closed. This author self-review is not approval.

## Identity

| Item | Value |
|---|---|
| Lane | G2 |
| Requested author | `grok-4.7` at high effort |
| Returned vendor model in the diagnosis | `grok-4.7-build` |
| Vendor session | `01a139e0-dc72-7053-ae78-c60bb882e193` |
| Vendor stop | `cancelled` after `Error: max turns reached` |
| Base commit | `97e77ea86d3832c642ebade4678f195b4e178775` |
| Head | same commit. No commit and no push. |
| Diagnosis | `/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/G2-headless-ready-terminal-diagnosis.json` |
| Diagnosis time | `2026-10-14T10:37:43.381379+00:00` |
| Queue | `2147` |
| Diagnosis conclusion | Vendor stopped at the max-turn cap. The fixed Cargo gate passed all 12 tests. Reports were absent. The runner emitted `round_incomplete` with `gate_rc=0` and `report_fresh=false`. That event is not a test failure. |

The diagnosis `modelUsage` key is `grok-4.7-build`. That recorded key does not by itself establish equivalence to requested `grok-4.7`. Fresh non-author Sol and Grok reviews are still required. The round-1 seats are S2 and G3. Diagnosis `liveContractStatus.counts.audit` is `0`.

## Objective and scope

A real source execution can be prepared under explicit local cost and sponsorship caps. Ordinary logical work cannot consume the reserved recovery slice. Unknown outcomes retain exposure. Concurrent candidate preparation cannot reserve beyond the same cap. Refusal preserves all counters.

Scope is `independent-local-source-preparation-only`. The labels `U3.preparation`, `U5.preparation`, `M4`, `M11.preparation`, and `M16` name preparation only. Acceptance dependencies `U2`, `U3`, and `U5` remain open.

## Owned paths

The write set is exact. Git status for those paths is one modification and two untracked files.

| Path | Git | Bytes | SHA-256 |
|---|---|---:|---|
| `crates/mori-core/src/policy.rs` | `M` | 19080 | `6f295a31781ce65a4bae14578abed5d431c3fdcb79208cda9fdfadbe27347b2d` |
| `crates/mori-core/tests/recovery_budget.rs` | `??` | 7764 | `b8237a97a4decb0ca68da3542c98b77357bcfdbb9a7e6d621a68b93713ccdbb7` |
| `crates/mori-core/tests/sponsored_execution.rs` | `??` | 20676 | `1cb191104effa1ab97eb4acc11deec5172ee75c15deea2593137924b9ec2a78e` |

These three hashes and byte counts equal `candidateIdentity.ownedPaths` in the diagnosis. This resume recomputed them and did not edit the files.

`git diff --stat -- crates/mori-core/src/policy.rs` is `311 insertions(+)` and no deletions. The diff blob is 11369 bytes. Its SHA-256 is `a037c4103608a59e2e31adc938180b48b373f61722304420e04d62493076924f`.

The diagnosis `candidateSha256` is `1fb55987f2b638f391d4e6dc7174739d2d0295bf6517bff1a6b9bf4625c5c389`. This resume reproduced that digest. The input is compact JSON with sorted keys. The object has `baseCommit` and `ownedPaths`. Each owned path object has `bytes`, `path`, and `sha256`. Path order is `policy.rs`, `recovery_budget.rs`, then `sponsored_execution.rs`.

`FederationPolicy` and `authorize` have no changed diff hunks. Their bytes and API stay in place for later epoch work.

## Behavior

`prepare_sponsored_execution` calls `crate::simulate` on the supplied program, action, and scenario. The execution result is that evaluator result. A successful hold stores `SponsorReservation::Reserved` for the dust amount and leaves `spent` at zero.

The function clones the grant and the work budget before mutation. Any error restores both clones. Evaluator failure, scope refusal, overflow, cap refusal, and an unauthenticated settlement premise therefore leave both counters unchanged.

Scope is explicit. The grant id, network, anchor, program, action, beneficiary, and candidate id must be non-empty. Those scope fields must equal the grant. `program.name` and the action string must equal the grant program and action. A wrong program object or a wrong action refuses before reservation.

Dust reservation must be positive. Two candidates share one `FeeCapability`. The second reservation fails when the remaining cap is short. The failed call restores the earlier reservation.

`LocalObservation::Timeout` moves the hold to `Unknown`. `release_unused` then refuses. `settle_authenticated` with `authenticated=false` refuses. `release_authenticated_unused` with `authenticated=false` refuses. Those refusals keep the unknown amount reserved. `release_authenticated_unused` with `authenticated=true` releases only an `Unknown` hold.

`authenticated` is a caller premise. The function does not authenticate a receipt, a sponsor, funding, or a ledger observation. It does not establish funding or actual settlement.

`LocalObservation::Settle` with `authenticated=true` debits `actual` once through the existing `settle` path. `actual` above the reserved dust refuses with unchanged state. A second settle on the same id refuses. The unused remainder of the reservation does not become a second debit.

`WorkBudget` meters logical, dust, service, and external amounts separately. `reserve_recovery` holds a slice. `charge_ordinary` refuses when the next ordinary amount would enter that slice. `charge_recovery` cannot spend ordinary spare capacity above the slice. A charge on one plane leaves the other planes unchanged. Overflow and a zero recovery reservation refuse with the budget unchanged.

Ordinary costs passed into preparation are caller-declared local amounts. They are not measured native fees.

## Integrator interface needs

No shared path was edited. `crates/mori-core/src/lib.rs`, Cargo manifests, Cargo.lock, and CLI dispatch stay unchanged.

The local entry point is already public as `mori_core::policy::prepare_sponsored_execution`. It consumes `crate::simulate`, `FeeCapability`, and `WorkBudget`. A crate-root re-export is not required for this predicate.

A later `mori` command can call that function. That call site belongs to the root integrator. This lane does not own `crates/mori`.

The native signed caller does not consume this policy. That gap is an acceptance blocker. It is not a missing local type in this write set.

The integrator must keep `FederationPolicy` and `authorize` unchanged when composing this candidate.

## Outer gate

The 12 passing tests belong to the prior outer gate recorded in the diagnosis `publicGateOutput`. This resume did not execute Cargo or Quint.

The gate output records these results.

- `sponsored_execution-bd14dd5cadbd6286`: 7 passed, 0 failed, 0 ignored, finished in 0.01s. Compile finished in 12.01s.
- `recovery_budget-c0032cfac4384922`: 5 passed, 0 failed, 0 ignored, finished in 0.02s. Compile finished in 1.91s.
- Clean removed 20 files, 322.7MiB total, before that gate compile.
- Runner event seq 133 at `2026-10-14T10:27:38Z` has `gate_rc=0` and `report_fresh=false`.
- Runner event seq 134 at `2026-10-14T10:27:39Z` has `kind=round_incomplete` with the same gate values.

The passing test names are:

- `evaluator_failure_restores_sponsor_and_budget_atomically`
- `interleaved_candidates_stop_at_the_same_cap_boundary`
- `ordinary_prepare_cannot_consume_the_recovery_slice`
- `scope_and_overflow_refusals_keep_every_counter`
- `settlement_below_reservation_debits_actual_once`
- `source_success_matches_evaluator_and_keeps_recovery_slice`
- `timeout_cannot_release_without_authenticated_unused_premise`
- `cost_planes_remain_independent`
- `cumulative_plane_accounting_stays_inside_each_cap`
- `ordinary_logical_work_cannot_consume_reserved_recovery`
- `overflow_and_empty_recovery_reservation_keep_budget`
- `recovery_charge_cannot_use_ordinary_spare_capacity`

The gate qualification is ordinary source and reference checking. It is not native certification.

## Prior author commands

These commands ran in the author session before the vendor stop. They are history. They are not a claim that this resume reran them. They are distinct from the outer-gate binaries above.

Each Cargo and Quint command held `flock -x /tmp/moriarty-language-wave1-build.lock`. The author session exported these variables.

- `CARGO_TARGET_DIR=/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/cargo-target`
- `CARGO_BUILD_JOBS=2`
- `RUST_TEST_THREADS=1`

The author session did not record `RUSTUP_TOOLCHAIN=1.99.0` in those commands. This resume ran no Cargo or Quint command.

### Author attempt 1

Log `/tmp/g2-verify/cargo.txt`.

```text
cargo clean --offline --locked -p mori -p mori-core -p mori-backend -p mori-ledger
cargo test --offline --locked -p mori-core --test sponsored_execution -- --test-threads=1
cargo test --offline --locked -p mori-core --test recovery_budget -- --test-threads=1
```

`sponsored_execution` failed to compile. The compiler reported 8 `E0618` errors. Local bindings named `grant` and `budget` shadowed the helper functions of those names. The retained tail shows calls at lines 483, 587, 588, 615, and 616. `SPONSORED_EXIT` was 101.

`recovery_budget` then passed 5 tests and 0 failed. `RECOVERY_EXIT` was 0. That recovery pass is an author-session result during a failed sibling compile. It is not the outer gate.

### Author attempt 2

The repair renamed the helpers to `new_grant` and `new_budget`. Log `/tmp/g2-verify/final.txt`.

`CLEAN_EXIT` was 0. `sponsored_execution` failed again at line 184. `let retry = scenario(3)` was `E0618` because the local `Scenario` binding shadowed `fn scenario`. `SPONSORED_EXIT` was 101.

`recovery_budget` again passed 5 tests. `RECOVERY_EXIT` was 0.

Quint then exited 0 for all four model commands. Acceptance reported 30 passing. Lifecycle reported 11 passing. Node printed a warning that `NO_COLOR` is ignored because `FORCE_COLOR` is set. The warning did not change the exit code. Those Quint runs are model QA only. They are not source refinement and they are not native acceptance. The cited outer-gate output does not include Quint.

### Author attempt 3

The repair renamed `scenario` to `scenario_with`. Log `/tmp/g2-verify/final2.txt`. Quint was `/home/charl/.npm-global/bin/quint` version `0.32.0`.

`cargo clean` removed 14 files and 35.8MiB. `CLEAN_EXIT` was 0.

`sponsored_execution-8cf611e3e40d347b` passed 7 tests in 0.03s. The test build finished in 10.25s. `SPONSORED_EXIT` was 0.

`recovery_budget-66db5bb2df31ea68` passed 5 tests in 0.02s. The test build finished in 1.40s. `RECOVERY_EXIT` was 0.

The same four Quint commands exited 0. Acceptance reported `30 passing (301ms)`. Lifecycle reported `11 passing (277ms)`. The same Node color warning appeared. These binaries differ from the later outer-gate binaries. The source hashes match the diagnosis snapshot.

## Current failures

The outer gate has no failing test in the diagnosis. The incomplete item at diagnosis time was the absent report pair. This resume writes that pair. The runner has not accepted these new report bytes. The unchanged outer gate will verify this attempt separately.

No product defect was found in this resume. Product bytes were left unchanged.

## Remaining gaps

These acceptance blockers stay open.

- Authenticated funding and durable real allowance storage are absent.
- Native DUST and actual settlement observations are absent.
- The same signed scoped financial and recovery policy is not connected through the native caller.

`U2`, `U3`, and `U5` acceptance artifacts are absent. Local preparation does not close `U3`, `U5`, `M4`, `M11`, or `M16`.

Declared logical, dust, service, and external amounts are local caller inputs. The implementation does not measure native fees.

`release_unused` still releases a never-submitted `Reserved` hold. The Quint lifecycle model recovers an unknown hold only after a proved-unused premise. This report does not claim that the Rust API refines that model.

Fresh non-author `gpt-6.1-sol` and `grok-4.7` reviews of the full candidate have not run. Author self-review is not approval. No native certification is claimed.
