### Fresh Designated Argon Security Guard-Unit Source Author Metadata

- **Conversation ID (`actualFIRSTinitCID`)**: `4d7aec1a-9f87-472d-922c-093e97314516`
- **Role**: Fresh Designated Argon Security Guard-Unit Source Author (Gemini 4 Argon High), independent of assembler/decoder authors (`8b557`, `dfa2`, `735`) and reviewer (`7d931`).
- **Execution / Vote Status**: Zero tools, commands, tests, model dispatches, or file writes executed. This response supplies the bounded guard-unit source text only and is **not** a review approval vote, source-change approval, or `UIREQ021` closure.
- **Contract Assessment**: In candidate-04 (`k_capture_observations.py`), `_logged_map_key_line` and `_logged_input_gaps` admit all-inline (`+10` spaces) and all-wrapped (`+10` / `+14` / `+10` spaces) `SortInt{}` -> `SortKItem{}` map-key injection lines across `I=12`, `I=16`, and `I=20` (including negative integer literals and multi-key maps) while preserving raw `configuration_text` and `input_kore` without normalization, and rejecting indentation tampering, integer/sort/comment/envelope/predicate/substitution tampering, non-key line wrapping, mixed inline+wrapped multi-key representations, and `None`/oversized/non-ASCII inputs with `['logged-configuration-is-not-input-kore']`. Direct helper unit coverage here is distinct from full 94-edge `decode_capture` chain propagation, which remains an integration obligation below.

---

### Complete Standard Python `unittest` Module (192 lines)

```python
"""Focused offline unit guards for candidate-04 _logged_map_key_line and _logged_input_gaps.

Covers owner-specified positive and negative obligations for inline (+10) vs
kore-exec 3-line wrapped (+10/+14/+10) SortInt{} map-key injection bindings,
truthful artifact retention, and boundary input rejections without cloning
candidate implementation logic or mutating on-disk capture artifacts.
"""
import hashlib
import importlib.util
from pathlib import Path
import unittest

CANDIDATE_PATH = Path(
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/'
    'logged-input-chain-candidate-04/afterimages/models/sherlock/qa/'
    'k_capture_observations.py'
)
ACTUAL_INPUT_KORE_PATH = Path(
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    '251-f03-paired-model-operation-proposal/actual/k/exec-009/state.kore'
)
ACTUAL_LOGGED_CONFIG_PATH = Path(
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/'
    'logged-input-chain-candidate-04/focused-chain-guard-inputs/'
    'actual-edge008-logged-configuration.txt'
)

EXPECTED_GAP = ['logged-configuration-is-not-input-kore']
EDGE008_INPUT_KEY_I12 = (
    b'            /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("0")),'
)
EDGE008_INLINE_KEY_I12 = (
    '                      /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("0")),'
)
EDGE008_WRAPPED_KEY_I12 = (
    '                      /* Inj: */ inj{SortInt{}, SortKItem{}}(\n'
    '                          \\dv{SortInt{}}("0")\n'
    '                      ),'
)

# Synthetic guard-only fixture (not a producer capture) exercising I=12, I=16, I=20 and >1 key.
SYNTHETIC_MULTI_KEY_INPUT_KORE = (
    b"Lbl'-LT-'generatedTop'-GT-'{}(\n"
    b"    Lbl'-LT-'env'-GT-'{}(\n"
    b"        Lbl'Unds'Map'Unds'{}(\n"
    b'            /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("0")),\n'
    b"            Lbl'Unds'Map'Unds'{}(\n"
    b'                /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("1")),\n'
    b"                Lbl'UndsPipe'-'-GT-Unds'{}(\n"
    b'                    /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("-2")),\n'
    b'                    /* Inj: */ inj{SortUIUInt{}, SortKItem{}}(\\dv{SortInt{}}("9"))\n'
    b'                )\n'
    b'            )\n'
    b'        )\n'
    b'    )\n'
    b')'
)
SYNTHETIC_MULTI_KEY_INLINE_LOGGED = (
    '      \\and{SortGeneratedTopCell{}}(\n'
    '          /* term: */\n'
    "          Lbl'-LT-'generatedTop'-GT-'{}(\n"
    "              Lbl'-LT-'env'-GT-'{}(\n"
    "                  Lbl'Unds'Map'Unds'{}(\n"
    '                      /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("0")),\n'
    "                      Lbl'Unds'Map'Unds'{}(\n"
    '                          /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("1")),\n'
    "                          Lbl'UndsPipe'-'-GT-Unds'{}(\n"
    '                              /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("-2")),\n'
    '                              /* Inj: */ inj{SortUIUInt{}, SortKItem{}}(\\dv{SortInt{}}("9"))\n'
    '                          )\n'
    '                      )\n'
    '                  )\n'
    '              )\n'
    '          ),\n'
    '      \\and{SortGeneratedTopCell{}}(\n'
    '          /* predicate: */\n'
    '          \\top{SortGeneratedTopCell{}}(),\n'
    '          /* substitution: */\n'
    '          \\top{SortGeneratedTopCell{}}()\n'
    '      ))\n'
)
SYNTHETIC_MULTI_KEY_WRAPPED_LOGGED = (
    '      \\and{SortGeneratedTopCell{}}(\n'
    '          /* term: */\n'
    "          Lbl'-LT-'generatedTop'-GT-'{}(\n"
    "              Lbl'-LT-'env'-GT-'{}(\n"
    "                  Lbl'Unds'Map'Unds'{}(\n"
    '                      /* Inj: */ inj{SortInt{}, SortKItem{}}(\n'
    '                          \\dv{SortInt{}}("0")\n'
    '                      ),\n'
    "                      Lbl'Unds'Map'Unds'{}(\n"
    '                          /* Inj: */ inj{SortInt{}, SortKItem{}}(\n'
    '                              \\dv{SortInt{}}("1")\n'
    '                          ),\n'
    "                          Lbl'UndsPipe'-'-GT-Unds'{}(\n"
    '                              /* Inj: */ inj{SortInt{}, SortKItem{}}(\n'
    '                                  \\dv{SortInt{}}("-2")\n'
    '                              ),\n'
    '                              /* Inj: */ inj{SortUIUInt{}, SortKItem{}}(\\dv{SortInt{}}("9"))\n'
    '                          )\n'
    '                      )\n'
    '                  )\n'
    '              )\n'
    '          ),\n'
    '      \\and{SortGeneratedTopCell{}}(\n'
    '          /* predicate: */\n'
    '          \\top{SortGeneratedTopCell{}}(),\n'
    '          /* substitution: */\n'
    '          \\top{SortGeneratedTopCell{}}()\n'
    '      ))\n'
)


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

    def test_logged_map_key_line_indents_12_16_20_and_non_key_passthrough(self):
        for indent_len, literal in ((12, '0'), (16, '1'), (20, '-42')):
            line = (' ' * indent_len) + f'/* Inj: */ inj{{SortInt{{}}, SortKItem{{}}}}(\\dv{{SortInt{{}}}}("{literal}")),'
            pad = ' ' * (indent_len + 10)
            expected = f'{pad}/* Inj: */ inj{{SortInt{{}}, SortKItem{{}}}}(\n{pad}    \\dv{{SortInt{{}}}}("{literal}")\n{pad}),'
            self.assertEqual(self.mod._logged_map_key_line(line), expected)
        for non_key in (
            '            /* Inj: */ inj{SortUIListValue{}, SortKItem{}}(',
            '            /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("0"))',
            '    Lbl\'-LT-\'depth\'-GT-\'{}(\\dv{SortInt{}}("1")),',
        ):
            self.assertEqual(self.mod._logged_map_key_line(non_key), '          ' + non_key)

    def test_nested_indents_12_16_20_and_multi_key_positive_and_mixed_refusal(self):
        self.assertEqual(
            self.mod._logged_input_gaps({'configuration_text': SYNTHETIC_MULTI_KEY_INLINE_LOGGED}, SYNTHETIC_MULTI_KEY_INPUT_KORE),
            [],
        )
        self.assertEqual(
            self.mod._logged_input_gaps({'configuration_text': SYNTHETIC_MULTI_KEY_WRAPPED_LOGGED}, SYNTHETIC_MULTI_KEY_INPUT_KORE),
            [],
        )
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

    def test_boundary_none_oversized_non_ascii_and_empty_inputs_refused(self):
        entry = {'configuration_text': self.actual_logged_text}
        self.assertEqual(self.mod._logged_input_gaps(None, self.actual_input_kore), EXPECTED_GAP)
        self.assertEqual(self.mod._logged_input_gaps(entry, b'a' * (self.mod.MAX_KORE_BYTES + 1)), EXPECTED_GAP)
        self.assertEqual(self.mod._logged_input_gaps(entry, self.actual_input_kore + b'\x80'), EXPECTED_GAP)
        self.assertEqual(self.mod._logged_input_gaps(entry, b''), EXPECTED_GAP)
        self.assertEqual(self.mod._logged_input_gaps({'configuration_text': ''}, self.actual_input_kore), EXPECTED_GAP)


if __name__ == '__main__':
    unittest.main()
```

---

### Exact Selector List

Assuming Root materializes the module as `test_logged_input_map_key_guards.py` (adjust the leading module name if materialized under a package path):

1. `test_logged_input_map_key_guards`
2. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards`
3. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards.test_actual_edge008_wrapped_and_inline_positive_and_retention`
4. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards.test_logged_map_key_line_indents_12_16_20_and_non_key_passthrough`
5. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards.test_nested_indents_12_16_20_and_multi_key_positive_and_mixed_refusal`
6. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards.test_negative_wrapped_indentation_tampering_refused`
7. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards.test_negative_value_sort_and_non_key_wrapping_refused`
8. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards.test_negative_comment_envelope_predicate_substitution_and_whitespace_refused`
9. `test_logged_input_map_key_guards.TestLoggedInputMapKeyGuards.test_boundary_none_oversized_non_ascii_and_empty_inputs_refused`

---

### Short Remaining Obligations Before General Adoption

1. **Fresh Non-Author Audit & Resource-Bound Freeze**: Root materializes this guard module and obtains a fresh independent non-author review before freezing the proposed offline unit resource envelope (`10s` wall / `25s` CPU / `1 GiB` address space / `64 MiB` write / `core 0` / `131072` streams) and executing the tests.
2. **Full 94-Edge / 95-Capture `decode_capture` Chain Integration**: Verify end-to-end across the actual 95 post-startup captures and 94 rewrite edges (`exec-001` through `exec-096`) that candidate-04 propagates all 94 `rule_start_line` values with `comparison_ready == True`, and sets `observed_rule_identities[i]` / `rule_start_lines` to `None` with `comparison_ready == False` when an edge's logged configuration is tampered.
3. **Prior Multiline Single-Item `<k>` Head Guards**: Retain and pass all positive/negative offline guards for multiline single-item `<k>` heads (`_single_k_item` / `_configuration`).
4. **All Six Mandatory R328 Offline Assembler/Decoder Unit Categories**:
   - Positive 195-child + initializer integration (`k_commands_bytes=None` and pinned `k_commands_bytes`, plus direct `kast` launcher).
   - Negative `kast.receipt.argv` option/flag/positional validation refusals.
   - Negative `_validate_kast_stdout_kore` single-term lexer/sort/size refusals.
   - Pre-startup chain link and command-template mismatch refusals (`initializer.kore`, `exec-000`, `print-000`, `exec-001`).
   - Post-startup `strict_stage_validation=False` (canonical gaps, `comparison_ready=False`) vs `True` (`ValueError`) stage checks.
   - On-disk `load_stage_artifacts_from_actual_dir` 488-file symlink, `st_size > max_bytes`, oracle basename/SHA-256, >1 MiB JSON, and rule-span row validation guards.

