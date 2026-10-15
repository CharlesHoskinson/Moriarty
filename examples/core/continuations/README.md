# Core continuation examples

Source candidate; these examples and tests have not been executed. They use
`moriarty/2`, `moriarty-core/2`, and the local reference-stage consumer.
All U0–U7, M1–M17, and MC/SP/G acceptance exits remain open.

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

`normal-oracle.json` is a byte-for-byte copy of the independently authored
2026-10-15 oracle, SHA-256
`76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`.
The `.mori`, scenario, and event files contain its frozen literals. Normal CLI
tests compare complete financial snapshots, ordered effects, original step
indices, observed values, captures, and attempted/committed work against those
literals. The oracle was prepared before implementation and remains
specified-only until separately authorized execution and result review.

`atomic-compatibility.mori` retains the `/1` atomic predicate behavior for
comparison. A false predicate rejects the complete invocation; a true
predicate completes in one execution.
