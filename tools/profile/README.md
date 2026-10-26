# Recursion phase profiling

This is the isolated dependency workspace for OpenSpec
`recursion-phase-profile`, tasks 2.1–2.3 and 4.1–4.3. It is not an instrumented
prover yet. No profiling run or speedup is claimed.

The vendor directory is the exact published midnight-proofs 0.8.3 archive,
verified before extraction against SHA-256
`19b2e4b1acb581ad741b0cb7c0e9dbdf51142f4e9518886f25fcff40a6702dcc`.
`identity.json` pins the archive, patch and each vendored file. `profile.patch`
is empty: its SHA-256 is
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
Task 3 must update the patch and identity manifest when it adds instrumentation.
The separate Cargo manifest depends on Moriarty by path and patches only its own
midnight-proofs resolution. Its lockfile was generated with
`cargo generate-lockfile --offline --manifest-path tools/profile/Cargo.toml`.
No Cargo build, check or test has been run for this change.

Run these checks from the repository root:

```sh
python3 -B scripts/qa/profile-recursion.py prepare --check-only
python3 -B scripts/qa/check-profile-isolation.py
python3 -B -m unittest discover -s scripts/qa/tests -p 'test_profile_*.py'
```

`prepare` is read-only, with or without `--check-only`. It checks the cached
archive, patch, vendor file hashes, separate lockfile presence and production
isolation. Use `--archive PATH` for another local copy of the archive.
The isolation check reads the root manifest, lockfile and `crates/*/Cargo.toml`;
it rejects the profile feature, vendor references and a non-registry
midnight-proofs lock entry. T0 wiring is left to the lead.

## Command plans

```sh
python3 -B scripts/qa/profile-recursion.py calibrate --workload A2 --pairs 5 --run-id a2-cold
python3 -B scripts/qa/profile-recursion.py one --workload A3 --cache-mode resident_pk --run-id a3-warm
```

These commands print shell-quoted `scripts/doctorwatson heavy` commands. They
create no directories and execute nothing unless explicitly given `--run`.
The plans build off/on release variants in separate target directories, then
invoke the profiling executable under the workload's existing factory budget.
`--hypothesis TEXT` forwards a trace-grounded retry hypothesis to heavy.
Calibration plans contain at least five off/on pairs in one cache stratum.
The paired analysis must be fixed before running them; this tool validates the
supplied statistics, as the Quint model does, and does not estimate confidence
bounds from synthetic data.

Task 3 must provide `src/bin/profile-recursion.rs`, the default-off
`recursion-phase-profile` feature, and the following executable interface:

```text
profile-recursion --workload A2|A3|A4|B3 \
  --cache-mode process_cold|resident_pk --sidecar PATH
```

The executable must perform the public workload, create its sidecar directory,
record exact identities, and preserve failed/partial traces. A resident-PK run
must prime the key and measure the second proof within the same process.
The planner names sidecars as
`qa/sherlock/profiles/<run>/<workload>-<cache>-<pair>-<variant>.json`.
Execution is refused while that executable is missing. The runner, timers,
bounded buffer, sampler and caller instrumentation remain task 3 work. Build
and proof commands have not been executed here.

## Validation and reports

```sh
python3 -B scripts/qa/profile-recursion.py reduce path/to/sidecar.json --output /tmp/partition.json
python3 -B scripts/qa/profile-recursion.py decide path/to/sidecar.json --output /tmp/decisions.json
python3 -B scripts/qa/profile-recursion.py reduce --run a2-cold
python3 -B scripts/qa/profile-recursion.py decide --run a2-cold
```

Run-directory mode reads sidecars and writes `reduce.json` or `decide.json` in
that directory. Validation completes before a report is written. Invalid,
failed, capped and incomplete inputs are rejected and left in place. A report
cannot overwrite its source. The tools never append factory receipts.

The JSON Schema is in the OpenSpec change. Additional semantic checks implement
the Quint accounting predicates with integer nanoseconds and exact decimal
ratios. Seconds with subnanosecond precision are rejected. The reducer does not
round the 5% limits. An accepted partition must reconcile exactly; its explicit
unexplained bucket must also be at most 5%. `unattributed_wall_seconds` names the
same leftover as `unexplained_other_seconds`, not another additive bucket.
Reported owner shares are checked against recomputed integer durations; their
decimal fractions permit only 1e-12 serialization error.

Every interval ID is unique. One diagnostic command root contains diagnostic
proof roots and nonoverlapping accounting spans. Nested worker diagnostics
receive no exclusive credit. Instrumentation must split parent residuals into
separate accounting spans instead of reporting overlapping inclusive intervals
as exclusive. Command coverage includes wrappers separately from proof
coverage. This first reducer accepts isolated, sequential proof accounting;
it rejects overlapping exclusive work instead of guessing process CPU shares.
CPU core-seconds include all threads, and unattributable CPU needs a reason.

RSS uses `max(0, sampled_peak - entry)`, including boundary readings in the
sampled maximum. The schema adds optional `rss.sample_points` records with
`at_monotonic_ns` and `rss_bytes`; they are required semantically whenever the
sample count is nonzero. Samples must be unique and strictly interior. Fewer
than two samples means boundary-only quality. Neither quality certifies an
instantaneous peak or admission. Optional `certifies_instantaneous` defaults to
false and cannot be true. Missing counters need reasons; unsupported DRAM
counters remain null. Logical copy bytes are never promoted to DRAM traffic.

Qualification is pass exactly when there are at least five pairs and the
one-sided 95% wall ratio upper bound is at most 1.01. A lower mean alone never
passes. Negative measured mean overhead is allowed, as in Quint. Full buffers,
dropped events, failed verification and changed protocol declarations invalidate
a profile. Secret field names and assignment patterns in free text are rejected
without printing their values. Fixtures contain public synthetic markers only.

`decide` emits C1–C10, each with a named metric or an explicit reason. It uses the
first eligible baseline proof in input order as one named denominator and never
pools fractions across shapes or strata. Phase shares are arithmetic bounds,
not measured candidate gains. C3 uses explicitly named MSM spans. C9 reports
exclusive non-MSM credit, inclusive multiopen time and shared MSM IDs separately;
composed credit cannot reuse a span. C1, C2 and C5 remain unresolved without
validated candidate evidence. C8 is inapplicable on 0.8.3. C10 remains unresolved
until matched cold/warm evidence and cache-hit counters exist.

`validate_comparison` checks matched identity, allowing only the binary hash and
observation feature to differ. Historical 0.8.2 inputs fail normal sidecar
validation. Explicit version/candidate comparisons through that helper never
transfer baseline phase fractions; they do not bypass sidecar validation.

Tasks 5–7 remain: real A2 on/off verification and overhead pairs, the cold/warm
A3/A4 and B3 matrix, the measured decision report, and production regressions.
No synthetic fixture is evidence that these tasks passed.
