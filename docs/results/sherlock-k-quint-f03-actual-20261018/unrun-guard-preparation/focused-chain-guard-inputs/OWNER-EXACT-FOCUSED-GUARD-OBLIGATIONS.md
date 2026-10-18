### 4. Exact Offline Guard-Unit Obligations Before General Adoption

In addition to the 6 R328 offline guard-unit categories (and the single-item multiline `<k>` head guards from the prior turn), the independent offline guard suite must include unit tests covering `_logged_input_gaps`:

1. **Positive — Inline and 3-Line Logger-Wrapped `SortInt{}` Map-Key Match**:
   - Verify that `_logged_input_gaps(entry, input_kore)` returns `[]` both when `entry['configuration_text']` uses single-line `/* Inj: */ inj{SortInt{}, SortKItem{}}(\dv{SortInt{}}("0")),` at `+10` spaces and when it uses `kore-exec`'s 3-line wrapped form at `+10` / `+14` / `+10` spaces across single-entry (`I=12`) and nested (`I=16`, `I=20`) maps, propagating all 94 rule `start_line` values into `decode_capture`.
2. **Negative — Indentation, Value, Sort, Comment, or Envelope Tampering Refusal**:
   - Verify that `_logged_input_gaps(entry, input_kore)` returns `['logged-configuration-is-not-input-kore']` (and `decode_capture` sets the edge's `rule_start_line` to `None` with `comparison_ready == False`) when:
     - the wrapped `\dv{SortInt{}}` line uses wrong indentation (e.g., `+12` or `+16` instead of `+14` relative to `input_kore`),
     - the integer literal inside `\dv{SortInt{}}("...")` differs from `input_kore` (e.g., `"1"` vs `"0"`),
     - a non-map-key line or non-`SortInt{}` injection (e.g., map value `inj{SortVal{}, SortKItem{}}(...)`) is line-wrapped,
     - any comment (`/* Inj: */`, `/* term: */`, `/* predicate: */`, `/* substitution: */`) or the `\top{SortGeneratedTopCell{}}()` envelope is altered, or
     - arbitrary whitespace/newlines are inserted or deleted outside the exact `_INLINE_MAP_KEY_INJ` 3-line pattern.

