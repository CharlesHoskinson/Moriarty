# Fresh independent nonsecurity integrated review — extra normal oracles N07/N08

**Top finding (nonsecurity): APPROVE on all axes below.** The eight candidate files connect the two added typed programs and the frozen literal tables through the *same* production comparator used for N01–N06, and the supplied ordinary host run passed that comparison. This approval is nonsecurity only, is not a security verdict, and infers no circuit, constraint, formal-refinement, native, stagenet, or Candidate B acceptance. Per instruction I executed nothing; hashes, base `30b65d925d1fc8210cb3521e7091aba787a69141`, and membership `f701291ac45f86f3f5e272977467608fab5792ae06c28404475df3fecb6220e9` are taken as the stated exact candidate identity.

## Axis 1 — Candidate correctness (full states, arithmetic, step order)

**APPROVE**

- N07 (`crates/mori-core/src/sherlock/extra_normal_oracles.rs`, `ExpectedCase "N07"`; `models/sherlock/oracles/extra-normal-programs.json`, case N07): function 17 binds `(1,2,3)` in source order to `0,1,2`, computes `10*2=20`, `1+20=21`, `100*3=300`, `21+300=321`; 39 states (0–38), 40 prefix rows, max depth 4 ≤ limit 8, 38 charges. Argument append via rules 177/181 and caller restore via 185 are in the literal sequence.
- N08 (`extra_normal_oracles.rs`, `ExpectedCase "N08"`): function 18 is genuinely self-recursive (`if n==0 then 1 else n * call18(n-1)`), environments `E2/E1/E0` and suspended multiplications are preserved (decisive stack at state 64; restores at states 76, 79, 82), yielding `uiU(2)`; 83 states (0–82), 84 prefix rows, max depth 7 ≤ 8, 82 charges (`8+28+28+12+3+3`).
- All charged boundaries carry control, environment, complete top-first frames, depth, and via-rule locators consistent with `models/sherlock/k/main.k` (e.g. 131/135 scheduling and push, 149/153 left/right return, 161/165 branch selection, 177/181 argument continuation/final, 185 return, 200/204/208 invoke/bind/bound, 223 primitive return).

## Axis 2 — Independence, fidelity, and connection through the actual common comparator

**APPROVE**

- Typed construction: `crates/mori-core/src/sherlock/extra_oracle_programs.rs` — `extra_program("N07")` builds `(a + 10*b) + 100*c` over parameters `0,1,2` with entry `(1,2,3)`; `extra_program("N08")` builds the `Eq/Sub/Mul/Call(18)` factorial body with entry `(2)`. These constructors reproduce, term for term, the `code`/`entry` literals in the frozen tables.
- Common path, no side channel: `crates/mori-core/src/sherlock/reference_oracles_tests.rs` — `normal_program` falls through to `extra_program` only for non-N01–N06 ids, and `normal_n07_three_ordered_arguments` / `normal_n08_recursive_factorial` call the identical `compare_case` used by N01–N06. `compare_case` asserts `program.to_k() == case.code` and `program.entry.to_k() == case.entry` before execution, so constructor-to-literal fidelity is checked, not assumed.
- Full-state/fuel prefix equality: the same file's `compare_snapshot` asserts code, entry, control, environment, complete frames, depth, stack limit, initial fuel, remaining fuel, charged steps, and outcome at startup, at every charged step (`step.before == prior`, charge 1, actual rule line vs `via_rule_start_line`, `remaining = initial_fuel − charged_steps`), at each prefix boundary, and at free publication (charge 0, control `.K`, expected output), plus terminal quiescence and `run() == publication.after`. N07/N08 receive exactly this treatment; there is no weaker extra-only comparator.
- Transcription fidelity: `models/sherlock/reference/check-reference.py` — `serialize_oracles` regenerates both Rust constant files from the JSON (including `case_ids=("N07","N08")` and `programs_file="extra-normal-programs.json"`) and byte-compares them, after verifying the `FROZEN` hashes. The supplied run records `exact_match: true` for both `normal_oracles.rs` and `extra_normal_oracles.rs`.
- Independence: `models/sherlock/oracles/build-extra-normal-literals.py` is a serializer over hand-listed states (stdlib `hashlib`/`json`/`pathlib` only; string constants expand repeated K text; no evaluator, transition function, or subject import), and the Rust expectations are mechanical transcriptions of those JSON literals. The expectations therefore remain independent of the production `Machine`; the typed ASTs are independently written and cross-checked by display-term equality.

## Axis 3 — Actual-result scope

**APPROVE**

- Supplied evidence (`extra-normal-host-execution/RUN.json`, `RUN.stdout`): sandboxed ordinary host execution of `check-reference.py` exited 0 in 0.65 s with `source_unchanged: true` and empty stderr; all **31 tests passed**, including `normal_n07_three_ordered_arguments`, `normal_n08_recursive_factorial`, the six original normal tests, comparator tests, and resource tests. The comparison event reports 8 normal programs and 216 literal prefixes (92 original + 40 N07 + 84 N08), qualified as “finite normal reference comparison only.”
- Scope bound, respected: this is a host Rust reference comparison beginning at the post-startup state 0 (K startup rule 74 and any K/Quint/Lean execution are not performed), over the typed valid-UInt128 common domain. It establishes nothing about circuits, constraints, formal refinement, native/stagenet targets, helper-microstep B work, or financial acceptance, and I infer none of those.

## Axis 4 — Merge readiness

**APPROVE (nonsecurity)**

- Existing six untouched: `reference_oracles_tests.rs` — `frozen_normal_oracle_membership` still asserts `CASES == [N01..N06]` with 92 prefixes, and `check-reference.py` keeps the original JSON and `main.k` in `FROZEN`; the change is purely additive (two cases, two tests, one program module, checker extension to eight).
- One structural note, not a change request: eight-way membership is pinned by the checker's `FROZEN` hashes plus `serialize_oracles` membership checks and the two named tests, rather than by extending `frozen_normal_oracle_membership` to `EXTRA_CASES`. That coverage is sufficient as supplied.
- Status wording: the frozen artifacts (`EXTRA-NORMAL-DERIVATION.md`, both extra JSON files) still say “specified-only/unexecuted,” which correctly describes source-preparation time. Changing them would break the frozen hashes; merge readiness therefore rests on the superseding actual result record and the independent reviews (this nonsecurity review, plus the separate Opus-owned security review, which remains outstanding and is not substituted by this report) accompanying publication, as instructed.

Required nonsecurity changes: none.

END_EXTRA_NORMAL_INTEGRATED_MUSE
