# Recursion phase profiling

This is the isolated dependency workspace for OpenSpec
`recursion-phase-profile`. It now contains the default-off observation feature
and caller timers, process counters and an RSS sampler (tasks 3.1–3.4),
alongside identity and reducer tooling. The A2 binary is implemented but unrun.
No proof, real profile, overhead qualification or speedup is claimed.

The vendor base is published midnight-proofs 0.8.3, archive SHA-256
`19b2e4b1acb581ad741b0cb7c0e9dbdf51142f4e9518886f25fcff40a6702dcc`.
`patches/midnight-proofs-0.8.3-profile.patch` records the complete unified diff
against that archive. `identity.json` pins its SHA-256 and every vendored file.
`prepare --check-only` verifies those hashes and reconstructs the diff from the
archive to ensure it describes the checked-in vendor tree exactly.
The separate manifest and lockfile affect only this workspace.

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

`one --workload A2` prints both off/on builds and runs by default; select one
with `--variant off` or `--variant on`. Each command uses
`scripts/doctorwatson heavy --budget a2-linked-children -- ...`. Build plans set
`RUSTC_WRAPPER=` and `CARGO_BUILD_JOBS=4`, with distinct on/off target directories.
These are command plans, not run receipts.

`src/bin/profile-a2.rs` reproduces the factory A2 literal-7 fixture using the
public request, child-key, proof, linked-verification and artifact APIs. It uses
the existing production RNG and authenticated local SRS, checks A102 k13 and
A80 k12, and calls stock verification on both proofs. Each reconstructed key
also checks a corrupted proof and a changed statement directly; the factory's
linked corrupted-proof and wrong-key controls remain present. Key reconstruction
spans include public preparation, SRS loading and VK construction; the backend's
internal `setup_pk` has its own nested vendor span.

```text
profile-a2 --workload A2 --cache-mode process_cold --sidecar PATH [--params-dir PATH]
```

The default SRS path is `MORIARTY_TEST_PARAMS_DIR`, falling back to the same
`$HOME/.cache/midnight/zk-params` path as the factory script. Neither environment
values nor SRS contents are written to sidecars. `resident_pk` is explicitly
refused: the public child APIs rebuild their PK on every proof. A3/A4/B3 runners
and the resident-PK matrix remain future work, with execution refused when the
corresponding executable is missing.

The planner names sidecars as
`qa/sherlock/profiles/<run>/<workload>-<cache>-<pair>-<variant>.json`.
The writer reserves a fresh sidecar and retains an incomplete marker if killed;
a returned error flushes the observed event trace. Public child artifacts go to
the adjacent `.children` directory. An existing sidecar is never overwritten.
Raw `recursion.phase-events/1` sidecars include counters, public proof/key/SRS
hashes and verification results. They are **not** qualified
`recursion.phase-profile/1` records: full resolved build identity, coverage and
paired overhead qualification still need completion. Feature-off runs retain
command wall time and verification evidence but allocate no phase recorder or
sampler. No proof commands have been run.

## Observation API and event mapping

Enable either `midnight-proofs/recursion-phase-profile` or the harness's
`recursion-phase-profile` forwarding feature. Both are off by default. Exported
macros are selected in the dependency itself, so enabling just the dependency
feature also enables caller timers. Off macros expand to the original expression
or no statements; they perform no clock access, buffering or I/O.

`src/lib.rs` provides key loading through `mori-recursion`, reusable `setup_pk`,
generic stock prove/verify wrappers (caller-selected transcript and RNG), and
backend child prove/verify wrappers. The vendor also times keygen's `setup_pk`
work when it is hidden inside an existing backend API. `command` closes its
diagnostic root and calls `profile::flush()` after success or a returned error.
It returns the original operation result and the flush I/O result separately.

The recorder preallocates 65,536 interval slots. Entry reserves a slot; RAII drop
records its end even on a returned error or unwind. Integer records contain only
phase, interval/parent, thread, public advice-phase/circuit ordinals, accounting
and completion metadata. Timestamps
are `Instant` nanoseconds relative to a process epoch. Recorder reentry is
suppressed before clock access or locking. Full buffers, dropped events, failed
clocks, reentry and incomplete/failed intervals invalidate observer integrity.
There are no per-cell events or recorded witness/scalar/challenge/seed values.

Accounting events own residual time after nested accounting children are
subtracted. MSMs own their intervals, including the two multiopen commitments.
NTTs are diagnostics; their enclosing phase owns their cost. Joined worker NTTs
are attached to their containing accounting interval by the mapper. Prove and
command roots are diagnostics so unexplained work cannot earn artificial credit.
Transcript calls retain their order, and synthesis/resolution/commitments remain
separate repeated intervals within each advice phase.

Set `MORIARTY_PROFILE_EVENTS` to an output path before invoking the harness.
When unset, flush creates no file. Flush emits `recursion.phase-events/1` JSON;
`flush_wall_ns` measures encoding and event-body writing/flushing, excluding its
own trailing metadata write. The caller receives any I/O failure. Flush after
worker joins; flushing an open span retains a visibly incomplete trace.

`profile-recursion.py reduce events.json` maps raw events to schema-compatible
phase fragments and disjoint accounting residuals. It retains failed raw traces
without awarding them accounting credit. An invocation with no fixed commitment
batch gets a zero-cost `not_applicable` marker. Raw mapping reports
`profile_valid: false`: raw events alone do not establish full identity,
coverage or qualification. Counter measurements from the harness enrich both
inclusive diagnostics and disjoint residuals. Missing counters remain null with
reasons. `pid=0` (legacy traces) and `source_line=0` mean uncollected. Mapped
fragments cannot be passed to `decide` as qualified sidecars.

## Harness counters

`src/counters.rs` owns all proc parsing and sampling. The vendor change only
exposes the monotonic epoch and an optional boundary callback, invoked outside
the event lock under the reentrancy guard. It neither reads proc files nor starts
threads. The harness installs the callback before starting command spans.

The RSS thread reads `/proc/self/status` every 10 ms with bounded storage. Both
ends of each read must lie strictly inside an interval before that sample is
attributed. Entry/exit VmRSS readings participate in the peak, while VmHWM stays
in a separate process-lifetime summary. Fewer than two interior samples yields
`boundary_only`; all sampled peaks are lower bounds, never admission guarantees.
Sample/read failures and capacity overflow invalidate observation without
changing the caller's result. Shutdown wakes and joins the sampler promptly.

At each span boundary, `/proc/self/stat` supplies process `utime + stime`,
converted using `_SC_CLK_TCK`; child-process times are excluded. These process
counters include all worker threads. Residual intervals reuse their actual
boundary snapshots, never a fraction of inclusive CPU. Unrelated overlapping
proofs and worker NTT diagnostics get null CPU/I/O with an attribution reason.
Inclusive diagnostics never receive exclusive CPU credit.

`/proc/self/io` supplies `read_bytes` and `write_bytes`. These are storage I/O,
not `rchar`/`wchar`, logical copies or DRAM traffic. Unavailable or regressing
counters are null with reasons. Logical, hardware-memory-controller and device
counters remain null with explicit reasons. Sampler thread CPU and read-wall
cost, bounded-buffer drops and flush cost are recorded separately.

Malformed-proof controls can produce expected transcript-read errors. The harness
marks only those transcript events inside an explicit verifier-control scope;
raw returned-error status is preserved. The mapper requires a successful
verification ancestor and passing control evidence before accepting them in an
otherwise valid run. Unexpected errors, clock failures and overflow still
invalidate observation.

Observed 2026-10-26: the prescribed offline harness test command passed
20 integration tests (8 counter, 11 event, 1 sidecar); 61 Python tests passed.
Isolation reported OK and prepare verified the regenerated vendor patch and
identity. The prescribed `cargo check --bins` commands passed with and without
the feature, using `RUSTC_WRAPPER=`, four jobs and the isolated target directory.
No proof, DoctorWatson command or network access was performed.

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
