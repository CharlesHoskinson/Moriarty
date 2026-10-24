# Why manifest reporting keeps identity and I/O gaps explicit

The UIREQ-013 implementation adds callable report projections to the existing public
A slice, Core scalar run and draft host run consumers. It preserves the existing
v1 reporters and adds opt-in v2/v3 envelopes. It neither changes a transition nor
runs a new evaluator when asked to report existing observations. The
[API reference](../reference/resource-manifest-api.md) names the exact exported
methods, result types and wire fields.

A source-definition hash identifies the frozen K definition. It does not identify
a rule inventory, a prepared program image or a lowering. The supplied A slice
has no selected producers for those three identities or for layout version.
The B owner carries an admission identity and supplied bounds/layout, but no
selected producers for rule-inventory digest, ProgramImageId or lowering digest.
The implementation reports those four A and three B gaps directly. Source text,
Debug display and a selected membership digest cannot silently fill them.
Physical frameBytesByFunction/helperScratchBounds and other unavailable physical
or host measurements also remain gaps; logical lengths are not physical bytes.

The JSON writer exposes a useful separate boundary: the complete serialized
report bytes accepted by a caller's `Write`. A report's collection I/O stays
unmeasured because neither supplied A snapshots nor a B host observation carries
that measurement. A successful write receipt describes compact report JSON only.
It excludes input, checkpoint and physical I/O; it supplies no timer, peak RSS,
circuit, key/proof-byte, ledger-fee or native-cost measurement. File creation,
flush and publication policy remain with the caller. The writer's success is
also distinct from authenticated evidence or transition validity.

At the October18 preparation cut, the ordinary attempt completed six phases:
normal changed-Core compilation, external consumer compilation, an exact
three-test listing and three separately selected tests. All six exited zero and
all 499 selected input pins matched in the persisted whole-attempt pre/post
comparison; no per-phase or continuous immutability observation is claimed. The external tests supplied frozen
observations for N05 calls/branches, N08 fuel40 exhaustion and ERROR10 application
rejection. Each compared whole/first/second/cumulative reports against four full
independent expected JSON objects, then wrote/read back those four reports with
the actual public writer and real files. Twelve new JSON files were observed.
These are new reporter/File observations over supplied typed data; no Machine,
owner execution, old N01 collection, native SDK or proof engine was rerun.
The error case is an application `Rejected(10)` observation, not injected
serialization or filesystem failure. No new fault-injection result is claimed.

The earlier fixed N01 supplied-report and live two-chunk results keep their own
source identities and narrower scope. Passing new reporter cases does not
retroactively make old receipts current for changed sources. The initial
independent review's invented schema/type and case descriptions are not adopted.
The corrected review assessed the exact source, actual report/write results
and earlier UIREQ-002/022/027 evidence. Root adopted that scoped finding and
explicitly reconciled source pins while preserving the historical receipts.
The [adoption record](../../results/sherlock-resource-manifest-20261018/ROOT-REPORTING-ADOPTION.json)
retains the corrections and qualifications; the reader returned CURRENT12
after its earlier STALE9 observation. Hash equality alone establishes neither
semantic approval nor requirement acceptance.

Original UIREQ-013/016/024/032 remain partial and OPEN, as do the eight affected
refinements SCON-001/003/009/011/015/023/024/025. No acceptance predicate, adopted
scenario or historical receipt is rewritten. Full manifest identity production,
whole-path collection I/O, applicable grouping/optimization predicates and
formal/native/target obligations remain separate. K's zero-charge terminal
publication, Quint's distinct terminal tick and B's helper microsteps retain
separate clocks; listing them in one report supplies neither same-fuel equivalence
nor a measured performance winner.


## October21 UTC — a failed native compile still has measured costs

The dated Native12 receipt records a real pinned ledger9 Cargo compile failure with three Rust errors, guard exit2 and outer exit1. A separate numeric Cargo exit was not retained. No current ledger9 ELF or recursive proof was produced. The subsequently applied three-line source repair is reviewed but uncompiled/unrun.

The attempt records E119800871/P92784030/S27016841 microseconds, with the phase-F wall time and exact remaining supervisor allowance still unknown. Root identifies the positive worker value as exposing the frozen accounting invariant; a pending source correction is not an adopted accounting result. A failed build cannot be reported as zero worker spend.

The same receipt distinguishes host restoration complete from raw cleanup owed: four conditional success artifacts were absent, so preservation debt remains unresolved in the historical record. Do not collapse physical restoration, evidence completeness, compilation and proof acceptance into one success flag. The source06 receipt capture preserves those separate fields and their pins.

Quint Source12's six finite calls and locked SDK archive/install checks keep their own scopes. Archive byte agreement does not establish a single receiving SDK/module graph, live wallet compatibility, native execution or target settlement. Benchmark completion still requires actual postdeployment target lineage and measurements; no inferred degree or performance is added here.
