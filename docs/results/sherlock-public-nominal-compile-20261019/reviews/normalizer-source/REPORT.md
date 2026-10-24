## Decision: approve — source-only, no blocking changes

The supplied sources meet the written format predicate for Rust-bounded `ProgramView` inputs.

No candidate tests, imports, actual artifacts, or private expected bodies were run or read in this review.

### Predicate review

- **Exact scalar kinds:** [program_snapshot_normalizer.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/strict-program-snapshot-normalizer-01/program_snapshot_normalizer.py) uses identity `type(...)` checks, so booleans are not accepted as integers and integers are not accepted as booleans or decimal strings.
- **Decimal canonicality:** `_decimal` accepts only `0` or a nonzero digit followed by ASCII digits. It rejects leading zeroes, signs, whitespace, fractions, exponents, empty strings, non-ASCII digits, JSON numbers, booleans, and null.
- **`TypeIdC` versus `TypeExprSrc`:** builtin `TypeIdC` strings map to `0..3`; custom values must be exactly `{"TypeId": "<canonical decimal >= 4>"}` and become integers. `TypeExprSrc` values named `UInt`, `Bool`, `List`, and `Tree` remain strings.
- **Enum shapes:** unit variants must be bare strings. Payload variants must be single-key objects with exactly the declared fields. Unit variants supplied payloads, payload variants supplied bare strings, unknown tags, and multi-tag objects are rejected.
- **Options:** `None` maps only to JSON `null`. Other option values are recursively normalized. Rust-style wrappers such as `{"Some": ...}` or `"None"` are rejected.
- **All 23 fields:** `_PROGRAM` contains exactly the 23 required top-level fields. Missing and extra fields are rejected at every declared object level, not only at the top.
- **No defaults or omissions:** object conversion requires exact key sets. Arrays are converted element-by-element without sorting, deduplication, filtering, or padding removal.
- **Order and multiplicity:** array order, repeated values, duplicate names, repeated decimal entries, and padding rows are preserved.
- **Full JSON duplicate detection:** `loads_strict` installs an `object_pairs_hook` over the whole document, including objects outside `/admission/program_view` and keys made identical through Unicode escapes. It also rejects float/exponent tokens and `NaN`/`Infinity`.
- **U128 preservation:** valid U128 values remain canonical decimal strings and are range-checked against `2^128 - 1`; they are not converted to JSON numbers.
- **Precise comparison:** `canonical_json` distinguishes `true` from `1` and `"1"` from `1`. Sorting is restricted to object keys. [test_normalizer_comparison.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/strict-program-snapshot-normalizer-01/test_normalizer_comparison.py) covers those distinctions and array-order sensitivity.

The synthetic cases in [test_program_snapshot_normalizer.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/strict-program-snapshot-normalizer-01/test_program_snapshot_normalizer.py) cover all declared expression, pattern, plan-node, and constant variants, builtin and custom type IDs, null and present options, operator units, padding-like repeated arrays, duplicate names, and the principal refusal shapes. They remain author-derived format cases, not semantic expectations.

One residual hardening note: extremely long decimal strings beyond Python’s integer-string conversion limit can raise plain `ValueError` from `int(...)` rather than `FormatError`, and deeply nested `TypeExprSrc` values can hit Python recursion limits. Rust-generated `usize` and `u128` wire values cannot reach the decimal-length case, so this does not block the stated bounded comparison. It should not be silently presented as graceful handling of arbitrary hostile input.

### Proposed ordinary regression command

The proposed command is suitable for Root to select separately:

```text
python3 -B -m unittest -v test_program_snapshot_normalizer test_normalizer_comparison
```

run from the candidate leaf directory.

It names six methods in the main test module and one in the comparison module, for seven total. The inspected imports are limited to the Python standard library and the local normalizer. No subprocess, network access, external tool, or filesystem fixture appears in the supplied modules. `-B` suppresses bytecode-cache writes.

That command would establish only the synthetic format regression. It would not verify hashes, semantic correspondence, an actual snapshot, or the private comparison. Its status remains **UNRUN** until Root executes it and retains the receipt.