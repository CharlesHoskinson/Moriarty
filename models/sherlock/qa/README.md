# Sherlock SDK-free normal QA

Run locally from the checkout root with Python 3.10 or newer:

```sh
python3 models/sherlock/qa/check.py
python3 -m unittest discover -s models/sherlock/qa -p 'test_*.py' -v
```

The first command verifies the frozen K source and four independent JSON corpus
files, validates eight normal A-profile cases, then checks the real committed
Rust oracle literals using the existing `check-reference.py` serializer. It never
calls that helper's compiler/test entry point. It reads 208 complete state rows,
200 charged edges, eight startup/publication boundaries and all 216 literal fuel
prefixes. Expected values remain the independently authored committed corpus;
this tool cannot update them. The pre-implementation contract is [CONTRACT.md](CONTRACT.md).

Compare a supplied readback in the **existing corpus document shapes**:

```sh
python3 models/sherlock/qa/check.py \
  --family normal \
  --observed-programs path/to/normal-programs.json \
  --observed-prefixes path/to/normal-prefixes.json
```

Use `--family extra-normal` for N07/N08. Both observed files are required.
Exit 0 means the requested literal QA checks passed, 1 reports a supplied
observation mismatch, and 2 reports an input, frozen-source or transcription
error. JSON is printed to stdout; input errors go to stderr. `--repo PATH`
selects another checkout containing the same frozen expectations.
That checkout's `check-reference.py` module is executed, so name only a trusted
checkout. It is executed only after its bytes match the pinned SHA-256
`a5ae942e9899e736581cb067843d25502df802672fb407cc88d581bd7dd9c354`; a differing
helper is not executed and is an input error (exit 2). Exactly the hashed bytes
are compiled, so a file swap after hashing or cached bytecode is not run.
Time-of-check/time-of-use limit: the pin covers those helper bytes only, not
the helper's imports, the Python interpreter, or a consistent snapshot of other
checkout files read at different moments.
Observed files are parsed strictly: a duplicate object member or a
NaN, Infinity or overflowing number is an input error (exit 2), never a match.

Every document field is compared, including metadata and extra observed fields.
Profile/source identity is checked first, then program and prefix documents in
expected member order. Arrays retain order. Complete immutable code/entry,
initial configuration, control, environment, saved frame values, frame order,
depth, selected control/arm terms, work, incoming rule, startup, terminal outcome
and every budget-publication row are retained. A first difference includes its
JSON pointer and raw values; trace differences retain both complete raw traces,
case/index, rule and literal state reconstruction where available. Reconstruction
only overlays the corpus's documented fixed fields and prefix columns.
K terms are compared as exact text: differing whitespace or map presentation
can produce a mismatch even if the terms would be semantically equivalent.

`status: pass` concerns corpus/source-literal QA only. Each report explicitly
lists K, Quint, Lean, Clean, handwritten circuit, Rust host and native/ledger
execution as `not-run`; an absent supplied observation is `not-supplied`.
The Rust source comparison establishes literal transcription, not interpreter
execution. This consumer supplies neither a new semantic machine/state ABI nor
a runtime exporter. It invokes no SDK, compiler, subprocess, model, network,
proof or ledger action. No B, hostile, financial, constraint, refinement,
stagenet or release acceptance follows from this result.

Observed local evidence on 2026-10-16: the first 23 checks failed against inert
stubs; two additional diagnostic checks failed before their implementation;
all 25 tests then passed. The default consumer passed both actual Rust literal
transcriptions. Runtime receipts, exact commands, Python version, source hashes
and RED/GREEN output are under
`/home/charl/Moriarty/.moriarty-dev/runtime/v1-factory-planning-20261016/factory-qa-build/`.
These are local development receipts, separate from reviewed product acceptance.

Opus 5.5 high subsequently authored six input-handling and exact-byte readback
regressions with their fixes. The pre-fix run reproduced failures; the patched
local command passes all 31 tests. See the
[public result record](../../../docs/audits/2026-10-16/doctorwatson/README.md).

Two later Opus-authored tests reproduced the unpinned helper and unusable
observed-index findings before their fixes. The corrected local entrypoint
passes 33 tests; its current capture is recorded at the same public result link.

Compare a complete supplied normal host diagnostic artifact with the new opt-in mode:

```sh
python3 -I -B models/sherlock/qa/check.py --host-observations path/to/host-observations.json
```

This is a file comparison, and it cannot be combined with the two supplied literal
corpus document flags. It checks all eight program/entry identities before the
216 complete prefix runs, including every record's rule/charge/full before/after
state and terminal publication. The independently reconstructed expectation has
5,355 records; [HOST-OBSERVATIONS.md](HOST-OBSERVATIONS.md) freezes the projection
and its limits. The [producer](../reference/normal_observations.rs) uses actual
`Machine::run_steps` and `TraceView` observations on fixed trusted typed inputs.
Its standalone `--test` build includes only the new bounded normal host tests.
The producer's compilation/execution is a separately inspected wrapper action;
this Python mode never launches it. Supplied producer/profile/source fields are
declarations. The report keeps execution receipt `not-supplied`, producer
authenticity `not-established`, constructor startup `unsupported`, and every
execution layer `not-run`. Actual build/run receipts must be reported separately.
