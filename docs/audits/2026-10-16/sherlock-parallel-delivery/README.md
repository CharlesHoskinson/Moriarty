# Sherlock public actual results, October 16

These captured local results exercise the working Sherlock host and the existing
separate flat-ID image interpreter against the frozen normal expectations. The
[machine-readable result](RESULT.json) separates implementation observations,
source bindings, review status and acceptance. All parent exits remain open.

| Recorded attempt | Actual observation | Original record |
|---|---|---|
| Ordinary local QA | Exit 0; 140 Python tests; 5.833 s | [Result](records/ordinary-01/result.json), [test output](records/ordinary-01/stderr) |
| Host | Exit 0; all 14 selected Rust tests; 25 source files; 24.156 s outer duration | [Receipt](records/host-01/receipt.json), [tests](records/host-01/logs/run-tests.stdout), [outer result](records/host-01-outer/result.json) |
| Flat-ID image | Exit 0; 23 source files; 1.119 s outer duration; unit tests not selected | [Receipt](records/image-01/receipt.json), [outer result](records/image-01-outer/result.json) |
| Quint | Typecheck, 216 deterministic real ITF captures, adaptation and comparison all exit 0; 20.772 s overall | [Result](records/quint-01/result.json), [tests](records/quint-01/normal-tests.stdout), [comparison](records/quint-01/compare.stdout) |
| Diagnostic context repair | Earlier RED exit 1 retained; 14 layer tests and 33 regressions pass; existing actual Quint comparison is byte-identical after repair | [RED](records/quint-context-fix/red.json), [recheck](records/quint-context-fix/recheck.json), [actual locator output](records/quint-context-fix/actual-context.stdout) |
| Public logical-work analysis | Eight Python tests; three actual supplied artifacts summarized; wrong image filename exit 2 preserved before corrected exit 0 | [Invocations](records/work-summary-01/invocations.json), [failed attempt](records/work-summary-01/image-summary.stderr), [tests](records/work-summary-01/normal-tests-green.stderr) |

Each host, image and Quint projection contains eight programs, 216 prefix runs,
5,355 records, 5,139 charged events and 216 free publications. These prefix
aggregates repeat work. The selected eight full traces contain 208 records and
200 charged events. Quint additionally retains 5,579 raw states and eight terminal
stutters. These are different denominators, not extra semantic transitions.
Every recorded host/image child completed within its 30-second cap. Reviewer
timeout predictions remain advisory estimates; they are not actual timeouts.

The public [archive](public-evidence.tar.gz) contains unchanged original receipts,
logs, observations, all 216 ITF captures and exact source snapshots. It contains
no compiled executables. [Member hashes](ARCHIVE-MEMBERS.json) bind every archive
file; [source bindings](SOURCE-BINDINGS.json) identify each recovered source and
its recorded digest. The host and image receipt source closures are complete.
The wider outer-wrapper source maps include context outside those closures.
The recovered historical `source_observations.rs` is source-prepared context;
these attempts never executed it. Changed later source is not substituted for
the recorded bytes. The later checker repair has its own source hashes and
consumer-only recheck; the original host/image checker remains captured intact.

Packaging independently checked the selected bytes, receipt-local digests,
source bindings, ITF hashes and aggregate counts. It constructed the archive
twice with identical bytes and checked every extracted member in memory.
[The packaging script](package_public_evidence.py) fixes member ordering, owner,
mode and timestamps and the gzip header. Run it with the checkout and the
existing public runtime directory as its two arguments; it does not execute any
captured command. Its optional historical context recovery uses only the
integrator-selected public `recovered-public/source_observations-pre-H1.rs`.

The first ordinary capture predates five new diagnostic-context tests. A fresh
full 145-test run and final candidate source/result review remain with the
integrator. Author observations above do not stand in for independent review.
Opus owns the separate pre-push review; this package supplies no security judgment.

The common profile is `sherlock-k-a/1` at the post-startup boundary. The flat-ID
interpreter excludes `reference.rs` from its compile closure, a bounded source
observation that does not prove implementation independence. Image diagnostics,
the stdlib current-body analysis and private `cfg(test)` host proposals do not
establish a Candidate B representation or native execution. Source-Core,
bounded scalar and streaming features under preparation are not delivered by
these captures.

No actual K capture, Lean reference success, Clean constraint check, handwritten
circuit, native proof/ledger result or Midnight stagenet result is claimed.
The Lean producer failure and repair remain separately owned. Startup, full
grammar and numeric coverage, canonical serialization, model refinement,
circuit correspondence and financial/physical costs remain open. Toolchain
runtime dependencies are incompletely pinned. All U0–U7, M1–M17 and MC/SP/G
parent exits remain open; native authority remains zero. Public developer use
stays permissionless.
