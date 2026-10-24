# How to locate the first observation difference

Use this guide after a comparison exits 1, or when identity or frozen-input checks fail. Keep the original files and complete output before editing anything.

## Check identity before state

Read the report's `scope`, `profile`, frozen source identity and selected input mode. For normal artifacts, expect `sherlock-k-a/1` and the pinned `models/sherlock/k/main.k` digest. The normal checker compares profile/source identity before program and prefix documents. Source artifacts also retain full source text, language/semantics profiles and preparation reports.

If `first_difference.path` names a profile, source or program identity, resolve that mismatch first. Do not interpret a later state as belonging to the intended program. If the command exits 2 with a frozen digest error, compare the changed source to its recorded baseline; a new pin needs independently justified expectations and corresponding consumers.

## Follow the pointer

Open the saved JSON report and inspect `first_difference`:

- `path` identifies the first field, using JSON Pointer escapes: `~1` means `/`, and `~0` means `~`.
- `kind` distinguishes type, value, missing member and array-length differences. Missing and present-with-null differ.
- `expected` and `observed` retain raw values; `expected_present` and `observed_present` disambiguate missing members where supplied.
- Run differences add `case_id`, `initial_fuel` and, when applicable, `record_index`, `rule_start_line`, complete states and records.

For example, the existing source regression deliberately changes a saved frame and expects:

```text
/runs/34/records/30/after/frames_top_first_k/0
case_id: S01
initial_fuel: 34
record_index: 30
```

That is a regression-test observation, not a recorded defect in a real source run. Inspect the full saved frame and its surrounding before/after states, rather than the final scalar result alone.

For supplied K/Quint envelopes, inspect the retained `raw_context` and its availability status. A missing or ambiguous raw-run candidate remains a diagnostic gap. For K, use the expected case and exact fuel to locate the raw capture; for Quint, retain the ITF state indices and terminal stutter distinction.

## Reproduce the smallest mismatch

Rerun the same comparison command from [compare complete observations](compare-complete-observations.md), with unchanged artifact bytes. Record the command, exit, source hash, artifact hash and first pointer. If the difference is an ordered frame, code, environment, charge, publication or terminal field, preserve it in the reproducer.

Use the independently derived semantic expectation to decide the repair. Exact K text comparison can fail on whitespace or map presentation; record that fact explicitly rather than silently converting a textual mismatch into semantic agreement. Recheck the affected complete comparison after the repair.

If the artifact matches but an executed layer is absent, keep that layer absent. Matching copied literals, projected Debug output and a declared no-next field cannot establish a producer run or formal correspondence. See [profiles and results](../reference/profiles-results.md) and [host interfaces](../reference/host-machine.md).

For the meaning of differing clocks and projections, read [observation boundaries](../explanation/observation-boundaries.md).
