### Role & Scope Qualification

- **Role**: Tests-only non-author review (`CID 8b5575c5-d1b9-4add-bcb1-5dd7c719e100`, original working directory). Because I authored the production correction in `k_capture_observations.py` (`candidate-04`), I claim **no** vote or approval on production code. I did **not** author `test_logged_input_map_key_guards.py` (`13,119`B, SHA256 `020e6f1791b3a7783f90542c8cad3a4168bc2992f00329ac53777167eb20e0f0`, 240 lines), which was independently authored by `4d7aec1a-9f87-472d-922c-093e97314516`.
- **Execution Discipline**: Zero tools, commands, tests, model/provider calls, or file writes were executed. No production or test source hunks are returned, and no test execution outcome is predicted.

---

### 1. Verbatim Source Quotes for Fidelity Verification

#### 1.1 Module Loader and Read-Only Fixture Initialization (Lines 117–130 of `COMPLETE BODY 1`)
```python
def _load_candidate():
    spec = importlib.util.spec_from_file_location('candidate04_k_capture_observations', CANDIDATE_PATH)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class TestLoggedInputMapKeyGuards(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = _load_candidate()
        cls.actual_input_kore = ACTUAL_INPUT_KORE_PATH.read_bytes()
        cls.actual_logged_text = ACTUAL_LOGGED_CONFIG_PATH.read_text(encoding='ascii')
```

#### 1.2 Actual Edge-008 Fixture Assertions & `decode_log` Retention (Lines 136–162 of `COMPLETE BODY 1`)
```python
    def test_actual_edge008_wrapped_and_inline_positive_and_retention(self):
        self.assertTrue(self.actual_input_kore.startswith(b"Lbl'-LT-'generatedTop'-GT-'{}(\n"))
        self.assertTrue(self.actual_input_kore.endswith(b'(\dv{SortInt{}}("0"))\n)'))
        self.assertIn(EDGE008_INPUT_KEY_I12, self.actual_input_kore)
        self.assertIn(EDGE008_WRAPPED_KEY_I12, self.actual_logged_text)
        entry = {'configuration_text': self.actual_logged_text}
        self.assertEqual(self.mod._logged_input_gaps(entry, self.actual_input_kore), [])
        self.assertEqual(entry['configuration_text'], self.actual_logged_text)
        inline_logged = self.actual_logged_text.replace(EDGE008_WRAPPED_KEY_I12, EDGE008_INLINE_KEY_I12, 1)
        self.assertNotEqual(inline_logged, self.actual_logged_text)
        self.assertEqual(self.mod._logged_input_gaps({'configuration_text': inline_logged}, self.actual_input_kore), [])
        raw_log = (
            'kore-exec: Debug (DebugAppliedRewriteRules):\n'
            '    The rules at following locations were applied:\n'
            '    /tmp/main.k:10:1-10:20\n'
            '    On configuration:\n'
            + self.actual_logged_text
            + 'kore-exec: Info (InfoExecDepth):\n'
            '    execution complete: exec depth: 1\n'
        ).encode('ascii')
        decoded = self.mod.decode_log(raw_log, source_path='/tmp/main.k')
        self.assertEqual(decoded['gaps'], [])
        applied = decoded['applied_entries'][0]
        self.assertEqual(applied['configuration_text'], self.actual_logged_text)
        self.assertEqual(applied['configuration_sha256'], hashlib.sha256(self.actual_logged_text.encode('ascii')).hexdigest())
        self.assertEqual(self.mod._logged_input_gaps(applied, self.actual_input_kore), [])
```

#### 1.3 Negative Mutation Blocks (Lines 185–227 of `COMPLETE BODY 1`)
```python
        mixed_logged = SYNTHETIC_MULTI_KEY_INLINE_LOGGED.replace(EDGE008_INLINE_KEY_I12, EDGE008_WRAPPED_KEY_I12, 1)
        self.assertEqual(
            self.mod._logged_input_gaps({'configuration_text': mixed_logged}, SYNTHETIC_MULTI_KEY_INPUT_KORE),
            EXPECTED_GAP,
        )

    def test_negative_wrapped_indentation_tampering_refused(self):
        for bad_block in (
            EDGE008_WRAPPED_KEY_I12.replace('                          \\dv', '                        \\dv', 1),
            EDGE008_WRAPPED_KEY_I12.replace('                          \\dv', '                            \\dv', 1),
            EDGE008_WRAPPED_KEY_I12.replace('                      /* Inj: */', '                    /* Inj: */', 1),
            EDGE008_WRAPPED_KEY_I12.replace('\n                      ),', '\n                        ),', 1),
        ):
            mutated = self.actual_logged_text.replace(EDGE008_WRAPPED_KEY_I12, bad_block, 1)
            self.assertEqual(self.mod._logged_input_gaps({'configuration_text': mutated}, self.actual_input_kore), EXPECTED_GAP)

    def test_negative_value_sort_and_non_key_wrapping_refused(self):
        for target, replacement in (
            ('                          \\dv{SortInt{}}("0")\n', '                          \\dv{SortInt{}}("1")\n'),
            ('inj{SortInt{}, SortKItem{}}(\n', 'inj{SortUInt{}, SortKItem{}}(\n'),
            ('inj{SortInt{}, SortKItem{}}(\n', 'inj{SortInt{}, SortK{}}(\n'),
            (
                '                      /* Inj: */ inj{SortUIListValue{}, SortKItem{}}(\n',
                '                      /* Inj: */ inj{SortUIListValue{}, SortKItem{}}(\n                          ',
            ),
            (
                '              Lbl\'-LT-\'depth\'-GT-\'{}(\\dv{SortInt{}}("1")),\n',
                '              Lbl\'-LT-\'depth\'-GT-\'{}(\n                  \\dv{SortInt{}}("1")\n              ),\n',
            ),
        ):
            mutated = self.actual_logged_text.replace(target, replacement, 1)
            self.assertNotEqual(mutated, self.actual_logged_text)
            self.assertEqual(self.mod._logged_input_gaps({'configuration_text': mutated}, self.actual_input_kore), EXPECTED_GAP)

    def test_negative_comment_envelope_predicate_substitution_and_whitespace_refused(self):
        for target, replacement in (
            ('/* Inj: */ inj{SortInt{}, SortKItem{}}(\n', '/* Inj:  */ inj{SortInt{}, SortKItem{}}(\n'),
            ('          /* term: */\n', '          /* term:  */\n'),
            ('          /* predicate: */\n', '          /* predicate:  */\n'),
            ('          /* substitution: */\n', '          /* substitution:  */\n'),
            ('      \\and{SortGeneratedTopCell{}}(\n', '      \\and{SortGeneratedTopCell{}} (\n'),
            ('          \\top{SortGeneratedTopCell{}}(),\n', '          \\bottom{SortGeneratedTopCell{}}(),\n'),
            ('          \\top{SortGeneratedTopCell{}}()\n      ))\n', '          \\bottom{SortGeneratedTopCell{}}()\n      ))\n'),
            ('          ),\n      \\and{SortGeneratedTopCell{}}(', '          )\n      \\and{SortGeneratedTopCell{}}('),
        ):
            mutated = self.actual_logged_text.replace(target, replacement, 1)
            self.assertNotEqual(mutated, self.actual_logged_text)
            self.assertEqual(self.mod._logged_input_gaps({'configuration_text': mutated}, self.actual_input_kore), EXPECTED_GAP)
        self.assertEqual(self.mod._logged_input_gaps({'configuration_text': self.actual_logged_text.rstrip('\n')}, self.actual_input_kore), EXPECTED_GAP)
        self.assertEqual(self.mod._logged_input_gaps({'configuration_text': self.actual_logged_text + '\n'}, self.actual_input_kore), EXPECTED_GAP)
```

---

### 2. Literal 7-Method Coverage Inventory

| # | Method Name (`TestLoggedInputMapKeyGuards`) | Candidate APIs Called | Fixtures / Inputs Used | Non-Vacuity & Independence Verification | Obligation Covered |
| :- | :--- | :--- | :--- | :--- | :--- |
| 1 | `test_actual_edge008_wrapped_and_inline_positive_and_retention` | `_logged_input_gaps`, `decode_log` | Pinned `exec-009/state.kore` (`8,626`B) and `actual-edge008-logged-configuration.txt` (`10,311`B) | Checks prefix, suffix, and `EDGE008_INPUT_KEY_I12` / `EDGE008_WRAPPED_KEY_I12` membership; verifies `inline_logged != actual_logged_text`; checks `decode_log` preserves `configuration_text` and `configuration_sha256` without mutation. | Positive inline (`+10`) and 3-line wrapped (`+10/+14/+10`) match on actual edge 8 (`I=12`) + raw log retention. |
| 2 | `test_logged_map_key_line_indents_12_16_20_and_non_key_passthrough` | `_logged_map_key_line` | Explicit strings for indents `12` (`"0"`), `16` (`"1"`), `20` (`"-42"`), plus 3 non-key lines | Expected strings are built from explicit padding (`indent_len + 10`, `+ 14`, `+ 10`); verifies non-`SortInt{}` injection, comma-less `SortInt{}` injection, and `<depth>` cell line pass through with `' ' * 10` prefix only. | Direct line-helper expansion across `I=12, 16, 20` (including negative integer literal) and non-key passthrough. |
| 3 | `test_nested_indents_12_16_20_and_multi_key_positive_and_mixed_refusal` | `_logged_input_gaps` | Independent literal constants `SYNTHETIC_MULTI_KEY_INPUT_KORE`, `..._INLINE_LOGGED`, `..._WRAPPED_LOGGED` | Uses hand-written full literal strings (not computed by calling candidate functions) with 3 `SortInt{}` keys at `I=12, 16, 20` and a `SortUIUInt{}` map value; also verifies mixed partial wrapping (1 wrapped, 2 inline) returns `EXPECTED_GAP`. | Multi-key nested `I=12, 16, 20` positive admission and mixed-representation refusal. |
| 4 | `test_negative_wrapped_indentation_tampering_refused` | `_logged_input_gaps` | In-memory copies of `actual_logged_text` with 4 indentation mutations on `EDGE008_WRAPPED_KEY_I12` | `EDGE008_WRAPPED_KEY_I12` is confirmed present in `actual_logged_text`; tests inner `\dv` at `+12` (`24` spaces) and `+16` (`28` spaces), opening `/* Inj: */` at `+8` (`20` spaces), and closing `),` at `+12` (`24` spaces). | Indentation tampering refusal on all 3 lines of wrapped map-key block. |
| 5 | `test_negative_value_sort_and_non_key_wrapping_refused` | `_logged_input_gaps` | In-memory copies of `actual_logged_text` with 5 value/sort/non-key mutations | Asserts `mutated != self.actual_logged_text` on every case; every target substring occurs verbatim in `COMPLETE BODY 4` (lines 51, 52, 54, 82). | Integer value (`"0"` -> `"1"`), source/target sort (`SortUInt{}`, `SortK{}`), map-value (`SortUIListValue{}`), and `<depth>` cell wrapping refusal. |
| 6 | `test_negative_comment_envelope_predicate_substitution_and_whitespace_refused` | `_logged_input_gaps` | In-memory copies of `actual_logged_text` with 8 comment/envelope/comma mutations + 2 trailing-LF mutations | Asserts `mutated != self.actual_logged_text` on all 8 `.replace(...)` cases; every target substring occurs verbatim in `COMPLETE BODY 4` (lines 1, 2, 51, 145–150). | Comment (`/* Inj: */`, `/* term: */`, `/* predicate: */`, `/* substitution: */`), `\and`/`\top` envelope, outer comma, and trailing-LF tampering refusal. |
| 7 | `test_boundary_none_oversized_non_ascii_and_empty_inputs_refused` | `_logged_input_gaps` | `None`, `b'a' * (MAX_KORE_BYTES + 1)`, `actual_input_kore + b'\x80'`, `b''`, and `{'configuration_text': ''}` | Exercises every guard branch in `_logged_input_gaps` (`entry is None`, `len(input_kore) > MAX_KORE_BYTES`, `not input_kore.isascii()`, empty input/config). | Boundary and malformed input refusal. |

---

### 3. Concrete Review Findings

1. **Exact Match of All Fixture Substrings Against `COMPLETE BODY 3` and `COMPLETE BODY 4` (Pass)**:
   - Every target string used in `.replace(target, replacement, 1)`, `.startswith(...)`, `.endswith(...)`, and `assertIn(...)` across all 7 methods exists verbatim in `exec-009/state.kore` (`COMPLETE BODY 3`) or `actual-edge008-logged-configuration.txt` (`COMPLETE BODY 4`), and every called symbol (`_logged_input_gaps`, `_logged_map_key_line`, `decode_log`, `MAX_KORE_BYTES`) exists with the exact called signature in `k_capture_observations.py` (`COMPLETE BODY 2`).
2. **Harmless Unescaped `\d` in Bytes Literal at Line 138 (Non-Blocking Observation)**:
   - Line 138 writes `b'(\dv{SortInt{}}("0"))\n)'` instead of `b'(\\dv{SortInt{}}("0"))\n)'` or `rb'(\dv{SortInt{}}("0"))\n)'`. In Python 3, an unrecognized `\d` escape in a bytes literal evaluates to the exact 2 bytes `0x5c 0x64` (`\d`), matching the final two lines of `exec-009/state.kore` (`Lbl'-LT-'generatedCounter'-GT-'{}(\dv{SortInt{}}("0"))\n)`). Under Python 3.12+, compiling the file emits a `SyntaxWarning: invalid escape sequence '\d'` to `stderr`, which does not fail the test or alter the exit code under `/usr/bin/python3 -I -B`.
3. **Author Metadata Count Discrepancy Already Reconciled by Root (Non-Blocking Observation)**:
   - `MATERIALIZATION.json` (`COMPLETE BODY 7`) records that the test author (`4d7aec1a-9f87-472d-922c-093e97314516`) misstated the line count as `192` (actual: `240` lines) and listed `9` selectors by including module/class aliases (actual: `7` test methods in `TestLoggedInputMapKeyGuards`). `OPERATION.proposed.json` (`COMPLETE BODY 6`) accurately pins the `7`-method membership set.
4. **Explicit Scope Limitation — Helper-Level Coverage Only (`APPROVE_WITH_LIMITS`)**:
   - The 7 methods test `_logged_input_gaps`, `_logged_map_key_line`, and `decode_log` directly; they do **not** invoke `decode_capture` (to test end-to-end `rules[i] = None` / `comparison_ready = False` propagation on a full 95-capture bundle), do **not** cover the single-item multiline `<k>` head guards (`_single_k_item` / `_configuration`), and do **not** cover the 6 mandatory R328 general assembler offline guard categories. Those remain separate mandatory prerequisites before general assembler/decoder adoption.

---

### 4. Verdict & Exact Passive Operation Eligibility

- **`TEST_SOURCE` Verdict (`test_logged_input_map_key_guards.py`, `13,119`B, SHA256 `020e6f1791b3a7783f90542c8cad3a4168bc2992f00329ac53777167eb20e0f0`)**: **`APPROVE_WITH_LIMITS`**
  - Approved as a focused, non-vacuous, read-only offline unit test suite for `_logged_map_key_line`, `_logged_input_gaps`, and `decode_log` `On configuration:` retention.
  - Scope is strictly limited to these 7 focused helper/log unit tests; it does not audit or approve my own production code (`candidate-04`), does not establish general assembler/decoder adoption (which still requires all 6 R328 offline guard categories, multiline single-item `<k>` guards, and `decode_capture` chain-tampering guard integration), and does not close `UIREQ-021`.
- **Exact Passive Operation Eligibility (`focusedLoggedInputMapKeyGuards` in `OPERATION.proposed.json`, `COMPLETE BODY 6`)**: **`ELIGIBLE`**
  - The proposed command (`/usr/bin/timeout --signal=KILL 10s /usr/bin/prlimit --cpu=25:25 --as=1073741824:1073741824 --fsize=67108864:67108864 --core=0:0 -- /usr/bin/python3 -I -B .../test_logged_input_map_key_guards.py`, `stdin=DEVNULL`, `inherit_environment=false`, minimal `PATH`/`LANG`/`TMPDIR`, `131,072`B stream cap, `serialFailStopNoRetry=true`, `outputs=[]`) is bounded, read-only on all 518 pinned inputs, and eligible for single-shot execution by Root.

