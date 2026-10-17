# How to prepare an additional normal case

Use this guide to prepare a new independently specified case for Sherlock's regression corpus. The existing command accepts frozen N01–N08 membership; there is no add-case CLI. An added case needs source, oracle and consumer changes before it becomes a runnable supported comparison.

## Specify the case independently

Choose one behavior and name the profile and semantic rules it exercises. Keep normal finite inputs separate from hostile-input, financial, general-grammar and bounded-B claims. Record the program/entry identity, argument order, stack limit, exact completion fuel, expected value and source digest before observing the implementation under test.

Derive each complete pending state by the stated semantics. Preserve code/entry, control, environment, top-first frames with saved values, depth, stack limit, initial/remaining fuel, charged work and incoming rule. Specify startup separately from the post-startup boundary. List every fuel budget, free publication, terminal outcome and whether another transition exists.

Use the existing `models/sherlock/qa/CONTRACT.md` and `HOST-OBSERVATIONS.md` as field checklists. For fixed Core-source work, also derive the full preparation report and source-occurrence mapping, following `models/sherlock/oracles/SOURCE-NORMAL-PROVENANCE.md`. Do not obtain expected states, work or outcomes by running the candidate evaluator.

## Extend the consuming closure

Keep existing frozen artifacts and receipts intact. Add a separately identified family or explicitly version the affected contract; do not append N09 to a frozen file and only update its hash. Track every consumer of the changed membership:

- Python `FAMILIES`, frozen hashes, validation and comparator tests in `models/sherlock/qa/check.py`.
- The pinned serializer and literal Rust transcription consumer in `models/sherlock/reference/check-reference.py`.
- Actual typed input builders, oracle constants and producer membership.
- Expected observation counts, source snapshot membership, sibling helper declarations and wrapper comparison assumptions.
- Applicable K/Quint/Lean projection inputs and their separate correspondence gaps.

For a fixed-source addition, the current checker also fixes S01/S02 order, fixture hashes, preparation fields, 70 runs and 1,259 records. Update those consumers together only after freezing the new independent expectations.

## Verify the new case through ordinary checks

Add normal-data regressions that prove the consumer rejects a missing state/run, reordered frame or case, wrong charge/work/outcome, identity drift and omitted free publication. A comparison-only fixture remains labeled as such. Check the actual CLI input mode and first-difference diagnostics, rather than testing a duplicate evaluator.

Run the existing ordinary entrypoint after implementing the consumer changes:

```bash
./scripts/doctorwatson.sh
```

Retain source hashes, independent derivation, exact changed membership, raw output and terminal status. An ordinary pass checks corpus structure, transcription and Python regressions. Any actual host/model/circuit run needs its own command and receipt; new source does not inherit old execution results.

See [DoctorWatson reference](../reference/doctorwatson.md) for existing lanes and [compare complete observations](compare-complete-observations.md) for the supplied-file path. Full V1, new design, proof and target acceptance remain separate. Maintainer review controls repository integration; it is not permission for a developer to author, compile, prove or deploy a supported Moriarty program.

See [evidence and acceptance](../explanation/evidence-and-acceptance.md) for the independent-expectation loop.
