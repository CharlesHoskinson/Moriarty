# Local Core/2 staged tests

The additive `moriarty-tests/2` manifest runs existing `moriarty/2` programs
through the ordinary Rust `mori test DIRECTORY` command. It calls the actual
Core/2 `stage::load`, `stage::start` and `stage::resume` APIs. The language,
semantics and continuation formats are unchanged. `moriarty-tests/1` keeps its
existing atomic consumer and report format.

This candidate is source-only: build, public command execution, tests and
actual-output comparisons are **UNRUN**. The checked-in
[examples](../models/core-stage-tests/mori-tests.json) now name six complete
normal report candidates copied exactly from the separately authored
[R3 freeze](../models/core-stage-tests/independent-oracle-r3/INDEPENDENT-FREEZE.json),
together with fresh complete program/plan preimages. This source/data integration
is not a runtime comparison or an admission. Historical source/draft dispositions
are recorded in the [header-correction audit binding](audits/2026-10-16/core-staged-tests-r3/header-correction-r1/REVIEW-BINDING.json)
and [earlier binding](audits/2026-10-16/core-staged-tests-r3/REVIEW-BINDING.json),
each for its exact reviewed candidate. [PR75](https://github.com/CharlesHoskinson/Moriarty/pull/75)
was merged at [baf61ebd](https://github.com/CharlesHoskinson/Moriarty/commit/baf61ebd0cb49f2c4441cb7fa8de53b343e056dc).
Those historical dispositions and the merge do not establish runtime results or
approve later changed bytes. New source changes require fresh applicable review;
current repository instructions govern later execution.

R3 sources append unused matching-duty `declare` actions after their target
actions; target deliver/charge code and incoming duty5/duty3 scenarios remain
unchanged. Historical author/R1/R2 positive expectations and data approvals were
invalidated by the omitted R7 registry premise and remain preserved in
[blocked history](../models/core-stage-tests/history/blocked-r2/) and the r2
capture. The unchanged root `expectation-freeze.json` describes archived old
bytes, not current root reports/preimages. See
[fixture provenance](../models/core-stage-tests/PROVENANCE.md) for current full
program/plan digests, disclosed representation/admission premises, historical
C1–C6/N1–N6 limits and all fourteen open draft gaps.

The [CLI selector](../crates/mori/src/commands/test.rs) requires an explicit
`mori test DIRECTORY` and opens only `DIRECTORY/mori-tests.json`; it does not
discover manifests recursively. Selecting `models/core-stage-tests` uses the
current root manifest. The [public staged-test regression](../crates/mori/tests/stage_test_public.rs)
copies only root regular files and skips subdirectories, including
`history/blocked-r2`. Explicitly selecting
`models/core-stage-tests/history/blocked-r2` from the repository root would read
its preserved historical manifest; those invalidated positive expectations
remain unusable for current validation. This is a source-inspection finding,
not a runtime observation. External wrappers are outside this finding.

A directory contains `mori-tests.json`:

```json
{
  "profile": "moriarty-tests/2",
  "cases": [{
    "name": "two-deliveries",
    "source": "two-deliveries.mori",
    "action": "deliver",
    "scenario": "two-deliveries-scenario.json",
    "logical_cap": "16",
    "expected": "two-deliveries-start.json",
    "events": [
      {"event": "approval-false.json", "expected": "two-deliveries-observed.json"},
      {"event": "approval-true.json", "expected": "two-deliveries-approved.json"},
      {"event": "receipt-true.json", "expected": "two-deliveries-complete.json"}
    ]
  }]
}
```

All keys shown are required. `logical_cap` is a canonical unsigned decimal
string or explicit `null`. Paths resolve from the manifest directory using the
existing test-command path convention.
paths are not confined to the directory; treat the manifest as trusted input.
There are 1–256 cases with unique,
nonempty names and at most 255 events per case, giving at most 256 invocations
including start. An empty event list tests the start result alone. Each case
starts independently from its scenario.

Under the existing R7 source check, liability identities at an awaiting start
and at a resume boundary must match an exact identity registered from a
`Liability` statement across the program's actions. The examples' unused
`declare` actions provide those matching identities without being executed by
the selected deliver/charge trajectories. Declaration presence alone implies
neither liability creation nor discharge. This is the source premise identified
in the final Opus finding; no runtime pass is claimed.

The start report is compared to `expected`; each ordered event resumes the
actual pending continuation and compares its report to that event's expected
file. Expected values never provide runtime state or continuations. A false
guard retains the actual pending continuation. A rejected resume also retains
its actual input for a later explicitly listed event. After a complete result,
or a rejected start, a further event is a command error because no actual
pending continuation exists.
A rejection before resume admission (event read/decode) reports attempted 0 and committed 0, as public `simulate` does; the retained continuation's committed work is unchanged.

Each expected file contains the **entire** report envelope printed by public
`mori simulate` for that invocation. Waiting reports include status, advanced,
segment (including explicit null), continuation, attempted/committed logical
work and residual liabilities. Complete reports include the full segment,
work and residuals. Rejected reports include the complete error and work.
Every actual report retains these literal qualifications:

```json
{"qualification":"local-reference-stage","authentication":"none","consumption":"untracked"}
```

Segments include action, pending frames, semantics, pre/post snapshots, ordered
steps with serialized statements and expression IDs, and ordered effects.
Continuations include program/plan hashes, site, ordered captures, logical cap
and committed work, and the whole boundary. Supply those fields explicitly;
there are no wildcards, omitted-field defaults, generated expectations or
selected-field comparisons. Object key order and JSON whitespace do not affect
equality. Array order, absent versus present fields, explicit nulls, values and
all additional fields do affect equality. Unknown manifest fields are errors;
additional expected-report fields cause a comparison mismatch.

Manifest, source, scenario and expected files retain the existing 1 MiB reader
ceiling and strict JSON parser. Event files use the existing strict Core/2
64 KiB event decoder. The harness passes actual typed continuations in memory;
it does not read or write continuation files or record their consumption.
Existing language limits and logical-cost semantics still apply. Logical work
is distinct from circuit cost, DUST, service fees and recovery reserves.

The aggregate retains `status` and ordered `cases`. Each case reports `name`,
`passed`, and ordered `stages`; each stage reports zero-based `index`, `passed`
and complete `actual`.
`passed` is a comparison verdict over local-reference-stage reports; the qualification labels are carried by each `actual`.
An expectation mismatch fails that case, retains its
actual report, and continues with actual state through later events and cases.
Any mismatch returns nonzero. A manifest/input error or impossible event
sequence also returns nonzero. Individual case export keeps the existing 1 MiB
export bound; an unexportable case publishes no partial JSON aggregate.

This is local, unauthenticated execution with consumption untracked. Test
agreement does not establish native proofs, settlement, authenticated events,
once-only consumption, circuit correspondence or complete conditional recovery.
U3, M10, the partial/conditional requirements, helper/circuit/native work and
every retained roadmap exit remain open. Security and hostile-test judgments
remain assigned to Opus; this document supplies none.
