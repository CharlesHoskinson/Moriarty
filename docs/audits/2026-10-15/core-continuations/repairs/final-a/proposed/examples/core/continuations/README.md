# Core continuation examples

These examples have been executed through the local Core and public CLI suites.
Current successor evidence covers 78 distinct focused Rust test functions:
49 affected functions passed in fresh post-repair captures, while 29 unchanged
functions are carried forward from earlier captures by source and call-path
projections. This is not a full post-repair rerun of all 78. Separate Quint QA has
11 normal tests and is excluded from the Rust total. The earlier 58-test
candidate remains a historical observation. These examples use `moriarty/2`,
`moriarty-core/2`, and the local reference-stage consumer. See the
[developer guide](../../../docs/CORE-CONTINUATIONS.md) and
[evidence dossier](../../../docs/audits/2026-10-15/core-continuations/README.md)
for the captured scope and its
[current selections](../../../docs/audits/2026-10-15/core-continuations/OBSERVATIONS-CURRENT.json).
The successor is local and unmerged; fresh successor Muse and Opus reviews
remain pending. All U0–U7, M1–M17, and MC/SP/G acceptance exits remain open.

`delivery.mori` suspends after moving 10 units into escrow and creating a
10-unit release duty. A false `ready` observation leaves its continuation
unchanged. A true observation completes the remaining transfer and discharge.
The independently counted successful segment costs are 5 and 6, committed 11.

`reserve.mori` has two suspension points. It retains a computed `held=42`
across the first boundary and the accepted event value `quote=43` across the
second. The successful costs are 11, 15, and 10, committed 36. Completion
preserves the existing positive allocation duty of 5.

The CLI forms are:

```text
mori simulate delivery.mori --action deliver --scenario delivery-scenario.json --logical-cap 11
mori simulate delivery.mori --resume continuation.json --event delivery-false-resume.json
mori simulate delivery.mori --resume continuation.json --event delivery-true-resume-complete.json
mori compile delivery.mori --target target.json --out new-output-directory
```

Save only the first result's `continuation` object as `continuation.json`.
A false observation returns that same continuation, `advanced:false`, and
`segment:null`. Completion reports residual duties separately.

Every staged result carries these exact labels:

```json
{"qualification":"local-reference-stage","authentication":"none","consumption":"untracked"}
```

The frozen Opus contract states: “Unsigned `/2` stages do not authenticate event
origin, correlation, finality or the resumer.” It also states: “The local
evaluator is a pure function of untrusted files.” These examples do not claim
conditional settlement, local consumption, native proof acceptance, or ledger
acceptance. The compiler's static `/4` artifacts retain their separate
unperformed qualification gates.

The `cap` stored in `continuation.json` under `logical` is outside both
continuation digests, `program_hash` and `plan_hash`. Anyone holding the file
can edit, remove or raise it without detection, so it is a local advisory
ceiling and not an authenticated lifetime work budget. Resume still checks
`committed` against the planned prefix work for the stored `site`. It refuses
a resume `--logical-cap` above a present stored cap, and the flag (or else the
stored cap) must cover `committed` plus the next segment. These checks neither
authenticate the file nor record its use.

`normal-oracle.json` is a byte-for-byte copy of the independently authored
2026-10-15 oracle, SHA-256
`76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`.
The `.mori`, scenario, and event files contain its frozen literals. Normal CLI
tests compare complete financial snapshots, ordered effects, original step
indices, observed values, captures, and attempted/committed work against those
literals. The oracle was prepared before implementation; those frozen
assertions have now been exercised in the captured local Core and CLI runs.
The independent oracle bytes remain unchanged. These observations do not
establish native proof or ledger acceptance.

`atomic-compatibility.mori` retains the `/1` atomic predicate behavior for
comparison. A false predicate rejects the complete invocation; a true
predicate completes in one execution.
