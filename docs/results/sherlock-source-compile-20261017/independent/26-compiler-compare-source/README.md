# Independent compile-only comparator

`compare_compile_only.py` compares only the declared selected fields against sealed R20 correction02. It verifies the exact oracle freeze and all41 members on every comparison. It imports no Moriarty/SDK/model code and cannot launch a compiler or run. Root owns any future capture and comparison.

Usage after independent source inspection and a separately authorized capture:

```sh
python3 compare_compile_only.py /path/to/sherlock-artifact.json --case quote
python3 compare_compile_only.py /path/to/sherlock-artifact.json --case eligibility
```

The comparator reads the given JSON file only. Exit0 means selected predicates match with explicitly unselected fields; exit1 means a selected mismatch; exit2 means input/contract error. `comparison_complete` remains false. Missing selected fields fail. Required but unselected fields must still be present. The result is never full artifact, source-semantics, runtime, native or target acceptance.

Quote compares all twelve ProgramView fields exactly. Eligibility derives the frozen hygienic role bijection from Let rows6/8/14/16, requires the exact four source names plus four distinct fresh symbols in UTF8 order, rewrites only NameIds and re-sorts the normalized names. It compares every resulting field/type/enum/scalar/array position/cardinality/multiplicity. ExprIds, constants, values, paths, function indices and argument slices are never renumbered or inferred. The input artifact is not mutated; output retains its raw ProgramView plus normalized ProgramView and explicit actual-ID/name mapping. The artifact hash preserves original bytes. Decimal owner numbers compare as exact strings and JSON integers have arbitrary precision; no numeric float normalization is used.

Selected compile-only predicates also include format, qualification, profile, input policy, plan dispositions, exact not-requested observation, actual typed admission identity/layout, known source metadata/digest and native/target/acceptance evidence tags. No genesis field is expected in compile-only output. Full preparation and unknown wrapper/evidence values remain unselected.

Root subsequently supplied the frozen body-free public R28 `BINDINGS.json` declaration and39 exact target bytes. Those are copied to `PUBLIC-WIRE-BINDINGS.json` and `inputs/target.json`, with fixed SHA256 pins. Config is exactly its nine supplied members plus file_digest, target is exactly `{descriptor,file_digest}`, and source action_index is exactly the canonical decimal string `"0"`. The comparator checks complete config/source/target identity objects and hashes exact file bytes, including LF. The descriptor is explicitly an unqualified HostOnly test descriptor; matching it does not qualify a native/stagenet target.

The optional `--wire-bindings FILE` and `--target-file FILE` may point to byte-identical copies of these pinned inputs only. The comparator rejects changed declaration or target bytes; different future public bindings/target inputs require an additive reviewed source amendment. No output-field discovery or dynamic schema guess is performed. Preparation and unknown evidence/wrapper values remain explicitly UNSELECTED, with `comparison_complete:false`.

`test_compare_compile_only.py` runs only inspected stdlib code over golden copies. It gives Eligibility different arbitrary fresh text and reordered raw NameIds, then checks exact alpha recovery.37 checks cover selected-cell changes, array order/multiplicity, empty-table presence, identity/layout/source/observation values and exact public R28 source/config/target metadata. Golden artifacts remain synthetic copies; their semantic expectations come only from the sealed oracle/public input declaration. No new compiler output, R19 implementation or SDK/private/native path was read or executed. Earlier correction02 and333 freezes remain unchanged.

Timing: Root reports seven ordinary positive compile-only tests completed before this comparator freeze. Their outputs were not supplied or read. The independent program expectations predate them; this comparator is frozen before any captured artifact/output is supplied to this author. No before-all-subject-execution claim is made.
