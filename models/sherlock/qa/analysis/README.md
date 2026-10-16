# Supplied logical work summaries

Run the opt-in consumer on an explicit complete diagnostic artifact:

```sh
python3 -I -B models/sherlock/qa/analysis/work_summary.py /absolute/path/to/observations.json
python3 -I -B -m unittest discover -s models/sherlock/qa/analysis -p 'test_work_summary.py' -v
```

The CLI writes a deterministic JSON report to stdout. Exit 0 means the supplied
artifact was summarized; exit 2 reports an input error to stderr. It accepts the
common [HOST-OBSERVATIONS](../HOST-OBSERVATIONS.md) document and the existing
`sherlock-lean-normal-observations/1`, `sherlock-quint-itf-observations/1` and
`sherlock-image-observations/1` envelopes. Envelopes retain their projection
declarations. The report preserves declared producer, profile, source hash,
boundary, complete ordered program identities and ordered run membership.

Each run has the key `(case_id, initial_fuel)`, record count, charged sum,
charged event count, free publication count, per-source-rule event/charge totals,
maximum observed frame count and depth, supplied initial/terminal work counters,
and the supplied `terminal_has_next` Boolean. A publication is an observed
zero-charge edge from `uiPending()` to a different output. Counter deltas are
reported separately from record charges; unknown counters remain JSON `null`.
Exact normalized-JSON run and ordered record hashes retain the full snapshot
fields. The CLI additionally hashes the exact input bytes. These hashes bind
local data; they do not authenticate execution or a producer declaration.

Cross-run columns have distinct denominators:

| Column | What is counted |
| --- | --- |
| `prefix_runs` | All supplied runs and records, including repeated prefixes and all free terminal publications |
| `prefix_event_fingerprints` | All observed event fingerprints, their distinct count and repetition count across runs of each program |
| `charged_prefix_events` | The same fingerprint counts restricted to positive-charge records |
| `unique_program_full_traces` | The first supplied no-next `uiDone(...)` run with observed free publication for each declared program; selected run indices are explicit |

A cross-run event fingerprint ignores only the before/after `initial_fuel` and
`remaining_fuel` fields, and retains the case ID, source rule, charge, charged
steps and every other state/record field. This is diagnostic deduplication,
not a proof of semantic equivalence. Publication fingerprints are kept separate
from charged-event counts. Successful full-run totals remain `null` if any
declared program lacks a supplied completion; observed subtotals remain named
as subtotals. The selection does not assert equality of other completed runs.
For the eight-case normal corpus, 200 is the charged edge sum of one complete
trace per program; 5,355 is the record count across 216 supplied fuel-prefix
runs. They are different quantities. No full-trace count is inferred from a
zero-fuel or middle-prefix run.

Runtime read closure: this module, the explicit input JSON and Python's standard
library/interpreter. It imports no repository helper, reads no frozen expected
trace, follows no envelope file reference and launches no process. The tests
read this module and construct independently authored N01/N02-shaped normal
fixtures; their one child process invokes this Python CLI on a temporary JSON.
No interpreter, compiler, model, private Lean checkout, circuit, network or
ledger action runs. There is no `__init__.py`; these tests require the explicit
analysis discovery command and do not change the existing default QA membership.

The eight focused normal tests cover zero/middle/full/surplus summaries, source
rule multiplicity, unknown counters, declarations/order/hashes, all three known
envelopes, repeated publication/charged event accounting and actual Python CLI
input binding. They establish supplied-data accounting, separately from oracle
comparison and actual producer execution. The callable predicate is: every
reported run statistic equals the supplied ordered records and states, with
cross-run totals retaining their declared denominator.

This implements the logical reporting portion of ResourceQ. Physical costs,
DUST and fees are unmeasured; helper work is unknown because these A-profile
snapshots have no helper-work counter. There is no B-bound change, state
synthesis, circuit cost estimate, native qualification or roadmap acceptance.
