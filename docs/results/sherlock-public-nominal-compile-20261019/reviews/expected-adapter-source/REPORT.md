No blocking source defect found. The adapter is correct for the narrow representation contract, and the five tests are eligible for a separate Root-run ordinary regression. Hidden numeric equality remains unproven, and the first comparison remains an immutable mismatch.

## Source correctness: pass

- The only conversion is in [expected_snapshot_adapter.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/expected-u128-representation-adapter-01/expected_snapshot_adapter.py), where the pinned declaration `identity_u128_decimal_string` changes an exact integer to `str(number)`.
- Reading [program_snapshot_normalizer.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/strict-program-snapshot-normalizer-01/program_snapshot_normalizer.py) confirms that this declaration occurs only at `ConstantSnapshot.U128.n`, and that `ConstantSnapshot` is reached only through `Program.constants`.
- There is no global search for keys named `U128` or `n`, and no other numeric coercion. Names such as `"U128"` and `"n"` therefore remain strings when the schema types them as strings.
- `_integer` uses `type(value) is int`, so Python booleans are rejected as a different JSON kind. The u128 branch also rejects negatives and values above `2^128 - 1`. `str(number)` is the canonical decimal form, and the transformation is injective.
- Other indices and TypeIds remain exact nonnegative integers. Booleans, strings, options, enum tags, payloads, exact field sets, missing/extra keys, array order, multiplicity, and padding are preserved by the generic traversal.
- The adapter does not invoke or modify the old `normalize_program_view`/`_convert` producer. It imports only `FormatError`, `_PROGRAM`, and `_TYPES` from the pinned normalizer.
- It constructs new lists and dictionaries rather than mutating the supplied expected structure.

Three non-blocking observations:

- Non-u128 integer fields have no explicit usize or TypeId width limit. This is permissive, but it cannot manufacture a match against a genuine Rust-derived actual value; an out-of-width expected value would simply mismatch.
- `normalize_expected_snapshot` does not itself call `loads_strict`. The proposed Root comparison must therefore retain the documented strict-decode step before applying it. The test checks `loads_strict` separately rather than binding that pipeline end to end.
- Object-key order is rebuilt in schema order, while array order is preserved. This is consistent with the frozen map’s statement that object-key order has no semantic meaning and with comparison through `canonical_json`. It would not be correct if literal pre-canonical object ordering were required.

## Ordinary regression eligibility: eligible, unrun

The tests in [test_expected_snapshot_adapter.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/expected-u128-representation-adapter-01/test_expected_snapshot_adapter.py) are suitable for Root to select as an ordinary synthetic regression:

- They cover `0`, maximum u128, duplicated values, true/false markers, unchanged integer/string/boolean fields elsewhere, bool-as-integer rejection, string/float/negative/overflow u128 rejection, missing and extra keys, and strict JSON duplicate/float rejection.
- The proposed command uses only `unittest`, `copy`, the adapter, and the pinned normalizer. It needs no compiler, native code, network, subprocess, private values, or filesystem fixture.
- Coverage is not exhaustive for every populated row type and nested option/enum combination. That limits the regression’s breadth, but does not make it ineligible; source review covers the shared generic traversal.
- Before execution, Root should verify the adapter, test, and old-normalizer pins in its own environment. Passing would establish only these synthetic format properties.

## Status separation

- Source-only adapter: **correct for the stated contract**.
- Proposed regression: **eligible for Root selection; not executed and not passed**.
- Hidden values: **not inspected and not shown to agree**.
- Full snapshot equality: **still UNPASSED**.
- First comparison: **remains immutable MISMATCH4ONLY**.

A later equality claim still requires the separate Root-owned fresh comparison described in [REVIEW-NOTES.txt](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/expected-u128-representation-adapter-01/REVIEW-NOTES.txt).