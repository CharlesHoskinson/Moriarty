# Four finite nominal K captures

Root observed complete typed JSON equality with the independently frozen oracle for four ordinary cases on October 18 UTC. All four halted normally. The captures contain 455 full states, including the initial state and the state after every event.

| Case | Advancing records | Full states |
|---|---:|---:|
| Structural mutual recursion, three arguments, A3/B2, caller restored | 133 | 134 |
| Structural mutual recursion, three arguments, A2/B3, caller restored | 133 | 134 |
| Nested Schedule first matching arm and separate addition | 87 | 88 |
| Nested Schedule miss followed by ordered fallback | 98 | 99 |

All six root fields and all five state fields matched: complete Program tables and recursive patterns, current and saved environments, ordered frames and events, binding paths and identities, array order, and scalar kinds. Raw actual and expected JSON bytes differ in formatting. This result concerns the complete typed values.

Equality means the recorded `COMPLETE-WIRE-COMPARISON-CONTRACT` relation: objects have exactly the same key sets and recursively equal values, object-key order is ignored, lists have exactly the same length and order, and scalar kinds and values match strictly, including boolean versus integer. The published `serialization/factor_json.py` function `exact_json` is object-key-order-sensitive. It checks each DAG’s own lossless round trip and is **not** the actual/expected comparison relation. The original comparator was an inline Root command and is not published as a script.

The exact canonical encoding that reproduces the stored `*CanonicalSHA256` values is:

```python
json.dumps(v, sort_keys=True, separators=(',', ':'),
           ensure_ascii=False, allow_nan=False).encode('utf-8')
```

There is no trailing newline; SHA-256 is applied to those bytes. Object keys are sorted recursively while array order and JSON scalar kinds are retained. Root’s new data-only `ROOT-PUBLICATION-P2-CANONICAL-ENCODING-READBACK.json` reproduces all four stored actual digests from the frozen actual wires. This is new representation QA, not recovery of the historical comparator source or a repeat of the expected comparison. Canonical digest agreement supplements the recorded strict whole-value comparison; it does not replace it.


Root froze all four actual wires and their independently derived `observerCuts` at 23:08:10.893352 UTC, before opening expected bodies. Strict comparison completed at 23:08:39.271119 UTC. The expected ordinals were independently frozen earlier; neither side filled the other. Source authors remained blind to populated expectations. Fresh Opus 5.5 high returned **APPROVE** for the finite result, with qualifications, and **CONDITIONAL APPROVE** for the exact publication subject to P1/P2 in these two READMEs. The reviewer structurally read both DAG arrays, all four Programs and all endpoints, and decoded about 30 state bodies. The remaining approximately 425 bodies were not individually decoded; the 378-object receipt DAG was spot-checked, and no hashes, gzip inverses or DAG expansion were recomputed by the reviewer. Whole-455 equality rests on Root’s recorded complete machine comparison and the finite independent oracle evidence, with those review checks. The original `STATUS.json` retains the proposal-construction pending value; the actual vote is bound in Root’s correction/result record rather than silently rewriting that historical status.

The captures exercise original wildcard patterns, structural mutual recursion between two nominal types, three typed function arguments, saved caller environments, nested constructor fields, ordered arm selection, and nested miss followed by fallback. The SAFE inputs and exact event and Program declarations accompany the source. These are finite observations of the uncharged reference profile. Production refinement, K/B/Quint clock correspondence, circuit/native/target evidence, and EARS completion remain separate obligations.

The existing capture route used fixed 16-step K windows. Each continuation carried the actual raw `ngTape` final state verbatim, decoded the complete printed tape, checked the complete typed boundary, and removed only its duplicate boundary row. JSON or expected data never supplied a model transition. The first case resumed an already observed prefix through ordinal 32 and used 21 new children. The remaining three used 69 children. Those 90 children are the two final selections; they exclude historical compile, parse and prefix work. Earlier passes and failed full-512 attempts remain in the receipt register and exact timing records. No old passing model prefix was rerun.

Case-1 ordinals 0–16 were reduced and printed with the predecessor `generic-nominal-let-if-five-case-actual-qa-20261018/actual/nominal-kompiled` definition. `inspect_prefix_wires.py` decoded that cut-016 prefix to JSON before the current reversed-state-observer compile resumed from its verbatim raw final state at ordinal 16. The complete typed boundary at ordinal 16 matched, and the predecessor/current kast outputs were byte-identical (30,419 bytes, SHA prefix `7dff96ac`). Independent equality covers those prefix values; it does not attribute their production to the current three published K files.

The decoder `inspect_prefix_wires.py`, the case-1 surface input `01-structural-mutual-three-args-A3-B2-postcaller.term`, and the `cut-016.kore` query are not published. Published `inputs/` contains cases 02–04 only. These missing provenance/input items remain explicit gaps; the source archive is not a complete standalone reproduction of the initial case-1 prefix.


Prior source and operation opinions approved the exact consumers and finite operations subject to Root's directory, static pin, dependency, inventory and checkpoint checks. The actual receipt register retains those gates. On a start-zero window, `complete_typed_boundary_equal=true` denotes the declared-input identity check; no duplicate boundary exists there, and `duplicate_boundary_rows_removed=0`. The remaining-three plan retained stale descriptive arithmetic of `90*33=2970s`; its 291 maximum children give `291*33=9603s`. Neither expression establishes an aggregate deadline, and the frozen plan was preserved.

All child caps stayed at 30 seconds wall time plus a 3-second sweep, CORE 0, FSIZE 512 MiB, umask 077, and process-group kill with token sweep. JVM parse jobs retained 512 MiB heap, 32 MiB stack and two active processors. No aggregate CPU, RSS or disk hard cap was established. Raw Kore growth, parser and consumer costs, and host pressure remain empirical risks for other programs. These four halts promise no completion for another input. Compiled artifact identity and unchanged K semantic premises retain their prior qualifications.

`actual/WHOLE-455-ACTUAL.lossless.json` contains the complete actual data in the existing typed JSON DAG format. The copied `serialization/factor_json.py` utility restores every object, key, scalar type and ordered list. Raw wire gzip files additionally preserve the exact original JSON bytes. The receipt and gate DAG retains exact slot timings. Raw Kore and original printer streams are deterministic gzip archives; `PUBLICATION-COPY-MANIFEST.json` records decompressed size and SHA proofs. These archives contain public SAFE-input K captures, with no SDK payload or witness.

Original independent expected pins, the original expectation freeze pin, and SAFE declarations supply provenance. The independent owner also stages four original expectation JSON gzip files and exact freeze, derivation, materializer, checker and wire-proof archives. Their bytes are hashed and copied opaquely; source authors do not inspect them. Root binds the complete expected transport and archive text directly to the fresh reviewer after all four actual wires were frozen. Public source copies preserve the observed bytes. Archived operation contracts retain their original absolute R paths as evidence. They add no runner or public developer permission requirement.
