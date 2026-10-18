"""Complementary offline security and contract guard tests for F03 candidate-04 QA modules.

Covers the remaining mandatory prerequisites before general tool adoption:
1. All six R328 offline assembler/decoder categories against candidate-04.
2. Multiline <k> single-KItem head positive normalization vs multi-item, wrong-root,
   and >MAX_TERM_BYTES refusals.
3. Whole decode_capture 95-capture/94-edge logged-input, predecessor state_kore, and
   engine/printer path-chain propagation and tampering rejections.

No import-time file I/O or sys.path mutation occurs. All four production modules are
verified byte-for-byte against their pinned SHA-256 digests and loaded from their
exact candidate-04 paths.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

CANDIDATE_QA_DIR = Path(
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/'
    'logged-input-chain-candidate-04/afterimages/models/sherlock/qa'
)
PROPOSAL_DIR = Path(
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    '251-f03-paired-model-operation-proposal'
)
K_ACTUAL_DIR = Path(
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01'
)

PRODUCTION_PINS = {
    'assemble_k_f03_observations.py': (
        63706,
        '83a6424f9896345fbbd22e51842af2561eb9aba17de7be6c34bb54507a9510eb',
    ),
    'k_capture_observations.py': (
        42838,
        '85e4e79a64752ee2b6b928e7dd1f749d0a9cedfacfbac0307f63d63107b62a1a',
    ),
    'k_observations.py': (
        7686,
        '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07',
    ),
    'k_terms.py': (
        5666,
        'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca',
    ),
}

EDGE008_INLINE_KEY_I12 = (
    '                      /* Inj: */ inj{SortInt{}, SortKItem{}}(\\dv{SortInt{}}("0")),'
)
EDGE008_WRAPPED_KEY_I12 = (
    '                      /* Inj: */ inj{SortInt{}, SortKItem{}}(\n'
    '                          \\dv{SortInt{}}("0")\n'
    '                      ),'
)


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _load_candidate_modules():
    """Load candidate-04 decoder and assembler by exact file location without old worktree paths."""
    for name, (expected_len, expected_sha) in PRODUCTION_PINS.items():
        raw = (CANDIDATE_QA_DIR / name).read_bytes()
        if len(raw) != expected_len or _sha256(raw) != expected_sha:
            raise AssertionError(f'Production pin mismatch for {name}')
    dec_path = CANDIDATE_QA_DIR / 'k_capture_observations.py'
    dec_spec = importlib.util.spec_from_file_location('k_capture_observations', dec_path)
    dec_mod = importlib.util.module_from_spec(dec_spec)
    prev_k_cap = sys.modules.get('k_capture_observations')
    prev_models = sys.modules.get('models.sherlock.qa.k_capture_observations')
    sys.modules['k_capture_observations'] = dec_mod
    sys.modules['models.sherlock.qa.k_capture_observations'] = dec_mod
    try:
        dec_spec.loader.exec_module(dec_mod)
        asm_path = CANDIDATE_QA_DIR / 'assemble_k_f03_observations.py'
        asm_spec = importlib.util.spec_from_file_location('assemble_k_f03_observations', asm_path)
        asm_mod = importlib.util.module_from_spec(asm_spec)
        asm_spec.loader.exec_module(asm_mod)
    finally:
        if prev_k_cap is None:
            sys.modules.pop('k_capture_observations', None)
        else:
            sys.modules['k_capture_observations'] = prev_k_cap
        if prev_models is None:
            sys.modules.pop('models.sherlock.qa.k_capture_observations', None)
        else:
            sys.modules['models.sherlock.qa.k_capture_observations'] = prev_models
    return asm_mod, dec_mod


def _clone_stages(stages: dict) -> dict:
    """Create an isolated mutable stage container without duplicating immutable byte payloads."""
    cloned = {}
    for stage_id, val in stages.items():
        if isinstance(val, dict):
            entry = dict(val)
            if 'receipt' in entry and isinstance(entry['receipt'], dict):
                entry['receipt'] = {
                    rk: (list(rv) if rk == 'argv' else rv)
                    for rk, rv in entry['receipt'].items()
                }
            cloned[stage_id] = entry
        else:
            cloned[stage_id] = val
    return cloned


def _clone_assembled(assembled: dict) -> dict:
    """Create an isolated mutable copy of an assemble_f03_capture_payload result."""
    out = dict(assembled)
    for probe_key in ('startup_probe', 'terminal_probe'):
        probe = dict(assembled[probe_key])
        for rec_key in ('engine_receipt', 'print_receipt'):
            probe[rec_key] = {
                k: (list(v) if k == 'argv' else v) for k, v in probe[rec_key].items()
            }
        out[probe_key] = probe
    caps = []
    for idx, cap in enumerate(assembled['captures']):
        c = dict(cap)
        if idx > 0:
            for rec_key in ('engine_receipt', 'print_receipt'):
                c[rec_key] = {
                    k: (list(v) if k == 'argv' else v) for k, v in c[rec_key].items()
                }
        caps.append(c)
    out['captures'] = caps
    out['rule_spans'] = [dict(r) for r in assembled['rule_spans']]
    return out


class _F03GuardTestBase(unittest.TestCase):
    asm = None
    dec = None
    stages = None
    receipts = None
    run_input = None
    inventory = None
    commands = None
    source_k = None
    assembled_control = None

    @classmethod
    def setUpClass(cls):
        if _F03GuardTestBase.asm is None:
            asm, dec = _load_candidate_modules()
            receipt_doc = json.loads((PROPOSAL_DIR / 'actual/k/receipt.json').read_text(encoding='utf-8'))
            receipts = {
                row['stage']: row['child_receipt']
                for row in receipt_doc['stages']
                if row['stage'] != 'initializer'
            }
            stages = asm.load_stage_artifacts_from_actual_dir(
                PROPOSAL_DIR / 'actual/k',
                receipts_by_stage_id=receipts,
            )
            run_input = (PROPOSAL_DIR / 'input/F03-fuel093-stack064.ui').read_bytes()
            inventory = (K_ACTUAL_DIR / 'COMPILED-MAIN-RULE-SPANS-01.json').read_bytes()
            commands = (PROPOSAL_DIR / 'K-COMMANDS.json').read_bytes()
            source_k = (K_ACTUAL_DIR / 'compile-input/models/sherlock/k/main.k').read_bytes()
            assembled_control = asm.assemble_f03_capture_payload(
                stages,
                run_input_bytes=run_input,
                rule_span_inventory_bytes=inventory,
                k_commands_bytes=commands,
                strict_stage_validation=True,
            )
            _F03GuardTestBase.asm = asm
            _F03GuardTestBase.dec = dec
            _F03GuardTestBase.stages = stages
            _F03GuardTestBase.receipts = receipts
            _F03GuardTestBase.run_input = run_input
            _F03GuardTestBase.inventory = inventory
            _F03GuardTestBase.commands = commands
            _F03GuardTestBase.source_k = source_k
            _F03GuardTestBase.assembled_control = assembled_control


class TestR328AssemblerAndStageGuards(_F03GuardTestBase):
    """Covers all six mandatory R328 offline assembler and stage guard categories."""

    def test_cat1_positive_195_child_initializer_jvm_pinned_unpinned_and_direct_kast(self):
        self.assertEqual(
            Path(self.asm.decode_capture.__code__.co_filename).resolve(),
            (CANDIDATE_QA_DIR / 'k_capture_observations.py').resolve(),
        )
        self.assertEqual(
            Path(self.dec._OBSERVATIONS.__file__).resolve(),
            (CANDIDATE_QA_DIR / 'k_observations.py').resolve(),
        )
        self.assertEqual(
            Path(self.dec._TERMS.__file__).resolve(),
            (CANDIDATE_QA_DIR / 'k_terms.py').resolve(),
        )
        kast_argv = self.stages['kast']['receipt']['argv']
        self.assertEqual(Path(kast_argv[0]).name, 'java')
        self.assertIn('org.kframework.main.Main', kast_argv)
        self.assertIn('-kast', kast_argv)

        unpinned = self.asm.decode_f03_observations(
            self.stages,
            run_input_bytes=self.run_input,
            rule_span_inventory_bytes=self.inventory,
            source_k_bytes=self.source_k,
            k_commands_bytes=None,
            strict_stage_validation=True,
        )
        pinned = self.asm.decode_f03_observations(
            self.stages,
            run_input_bytes=self.run_input,
            rule_span_inventory_bytes=self.inventory,
            source_k_bytes=self.source_k,
            k_commands_bytes=self.commands,
            strict_stage_validation=True,
        )
        self.assertFalse(unpinned['assembled_provenance']['k_commands_pinned_verified'])
        self.assertTrue(pinned['assembled_provenance']['k_commands_pinned_verified'])
        self.assertEqual(unpinned['bundle'], pinned['bundle'])
        self.assertEqual(unpinned['decoded_run'], pinned['decoded_run'])

        direct_stages = _clone_stages(self.stages)
        kast_flag_idx = kast_argv.index('-kast')
        direct_stages['kast']['receipt']['argv'] = ['/usr/bin/kast', *kast_argv[kast_flag_idx + 1 :]]
        recipe_bytes = (PROPOSAL_DIR / 'INITIALIZER-RECIPE.json').read_bytes()
        direct_stages['initializer'] = {
            'initializer_kore': self.stages['initializer']['initializer_kore'],
            'recipe_bytes': recipe_bytes,
        }
        direct = self.asm.decode_f03_observations(
            direct_stages,
            run_input_bytes=self.run_input,
            rule_span_inventory_bytes=self.inventory,
            source_k_bytes=self.source_k,
            k_commands_bytes=None,
            strict_stage_validation=True,
        )
        self.assertTrue(
            direct['assembled_provenance']['pre_startup']['initializer']['recipe_bytes_verified']
        )
        with self.assertRaisesRegex(
            ValueError, 'kast.receipt.argv does not match pinned K-COMMANDS.json stage argv'
        ):
            self.asm.assemble_f03_capture_payload(
                direct_stages,
                run_input_bytes=self.run_input,
                rule_span_inventory_bytes=self.inventory,
                k_commands_bytes=self.commands,
                strict_stage_validation=True,
            )

        for res in (unpinned, pinned, direct):
            self.assertTrue(res['bundle']['comparison_ready'])
            self.assertTrue(res['decoded_run']['comparison_ready'])
            self.assertEqual(res['decoded_run']['gaps'], [])
            self.assertEqual(len(res['decoded_run']['raw_captures']), 95)
            run = res['bundle']['observations']['runs'][0]
            self.assertEqual(len(run['records']), 94)
            self.assertEqual([r['charge'] for r in run['records']], [1] * 93 + [0])
            self.assertTrue(all(isinstance(r['rule_start_line'], int) and r['rule_start_line'] >= 1 for r in run['records']))
            self.assertEqual(run['records'][-1]['rule_start_line'], 80)
            self.assertIs(run['terminal_has_next'], False)
            self.assertEqual(res['decoded_run']['startup_probe']['gaps'], [])
            self.assertEqual(res['decoded_run']['terminal_probe']['gaps'], [])
            self.assertEqual(
                res['decoded_run']['raw_startup_probe']['engine_receipt'],
                self.receipts['exec-001'],
            )
            self.assertEqual(
                res['decoded_run']['raw_terminal_probe']['engine_receipt'],
                self.receipts['exec-096'],
            )
            pre = res['assembled_provenance']['pre_startup']
            self.assertEqual(set(pre), {'scope', 'kast', 'initializer', 'exec-000', 'print-000'})
            self.assertEqual(
                pre['exec-000']['state_kore_sha256'],
                self.receipts['exec-000']['output_sha256'],
            )

    def test_cat2_negative_kast_receipt_argv_refusals(self):
        orig_argv = list(self.stages['kast']['receipt']['argv'])
        main_idx = orig_argv.index('org.kframework.main.Main')
        cp_idx = orig_argv.index('-cp')
        tail_args = orig_argv[main_idx + 2 :]

        def check_bad_argv(mutated_argv: list[str], pattern: str):
            st = _clone_stages(self.stages)
            st['kast']['receipt']['argv'] = mutated_argv
            with self.assertRaisesRegex(ValueError, pattern):
                self.asm.assemble_f03_capture_payload(
                    st,
                    run_input_bytes=self.run_input,
                    rule_span_inventory_bytes=self.inventory,
                    k_commands_bytes=None,
                )

        check_bad_argv(
            ['/usr/bin/python3', *tail_args],
            r'kast\.receipt\.argv\[0\] must name the pinned java launcher',
        )
        check_bad_argv(
            orig_argv[: main_idx + 1] + tail_args,
            'kast.receipt.argv java launcher must invoke org.kframework.main.Main -kast',
        )
        check_bad_argv(
            [orig_argv[0], *tail_args],
            'kast.receipt.argv java launcher must invoke org.kframework.main.Main -kast',
        )
        check_bad_argv(
            orig_argv[:main_idx] + ['-server'] + orig_argv[main_idx:],
            'kast.receipt.argv java launcher has unadmitted JVM token',
        )
        bad_cp_flag = list(orig_argv)
        bad_cp_flag[cp_idx + 1] = '-Xss8m'
        check_bad_argv(
            bad_cp_flag,
            'kast.receipt.argv java launcher has invalid -cp/-classpath argument',
        )
        check_bad_argv(
            [orig_argv[0], '-cp', 'org.kframework.main.Main', '-kast', *tail_args],
            'kast.receipt.argv java launcher has invalid -cp/-classpath argument',
        )

        for required_opt in ('--definition', '--module', '--sort', '--input', '--output'):
            idx = orig_argv.index(required_opt)
            missing_opt_argv = orig_argv[:idx] + orig_argv[idx + 2 :]
            check_bad_argv(
                missing_opt_argv,
                f'kast.receipt.argv missing required kast options: \\[{required_opt!r}\\]',
            )
            missing_val_argv = list(orig_argv)
            missing_val_argv[idx + 1] = '--another-flag'
            check_bad_argv(
                missing_val_argv,
                f'kast.receipt.argv has duplicate or missing value for {required_opt}',
            )

        check_bad_argv(
            orig_argv + ['--unadmitted-flag'],
            'kast.receipt.argv contains unadmitted option',
        )
        for opt, bad_val, expected_msg in (
            ('--module', 'OTHER-MODULE', 'kast.receipt.argv --module must be MORIARTY-UI-SYNTAX'),
            ('--sort', 'UIExpr', 'kast.receipt.argv --sort must be UIRun'),
            ('--input', 'kore', 'kast.receipt.argv --input must be program'),
            ('--output', 'pretty', 'kast.receipt.argv --output must be kore'),
        ):
            mut = list(orig_argv)
            mut[mut.index(opt) + 1] = bad_val
            check_bad_argv(mut, expected_msg)

        wrong_pos = list(orig_argv)
        wrong_pos[-1] = '/tmp/wrong-input.ui'
        check_bad_argv(
            wrong_pos,
            "kast.receipt.argv must name exactly one 'F03-fuel093-stack064.ui' input file positional argument",
        )
        check_bad_argv(
            orig_argv[:-1],
            "kast.receipt.argv must name exactly one 'F03-fuel093-stack064.ui' input file positional argument",
        )
        check_bad_argv(
            orig_argv + [orig_argv[-1]],
            "kast.receipt.argv must name exactly one 'F03-fuel093-stack064.ui' input file positional argument",
        )

    def test_cat3_negative_validate_kast_stdout_kore_single_term_refusals(self):
        cases = (
            (
                b'LbluiRun{}() Lblother{}()\n',
                r'kast\.stdout must be a single KORE term \(root application closed before end of line\)',
            ),
            (
                b'LbluiRun{"str_before_lparen"}()\n',
                'kast.stdout string literal outside root KORE application',
            ),
            (
                b'LbluiRun{{}()\n',
                'kast.stdout has unclosed sort braces before parenthesis',
            ),
            (
                b'LbluiRun{}(Lblfoo{)\n',
                'kast.stdout has unclosed sort braces inside parentheses',
            ),
            (
                b'LbluiRun{}(\\dv{SortInt{}}("unterminated))\n',
                'kast.stdout has unbalanced quotes, braces, or parentheses',
            ),
            (
                b'LbluiRun{}((Lblfoo{}())\n',
                'kast.stdout has unbalanced quotes, braces, or parentheses',
            ),
            (
                b'LbluiRun{}}(Lblfoo{}())\n',
                'kast.stdout has unbalanced braces',
            ),
            (
                b'LbluiRun{}(VarX:SortInt{})\n',
                'kast.stdout must be a ground KORE term without free variable declarations',
            ),
            (
                b'LbluiRun{}(\\and{SortInt{}}())\n',
                r'kast\.stdout must be a ground KORE term \(only \\dv\{\.\.\.\} backslash construct admitted\)',
            ),
            (
                b'LblotherRoot{}(\\dv{SortInt{}}("0"))\n',
                'kast.stdout root KORE constructor must be a SortUIRun\\{\\} / uiRun term',
            ),
            (
                b'not_a_kore_application\n',
                'kast.stdout must be a KORE constructor or domain-value application',
            ),
            (
                b'LbluiRun{}(' + (b'a' * 65530) + b')\n',
                'kast.stdout exceeds bound',
            ),
        )
        for bad_stdout, pattern in cases:
            with self.assertRaisesRegex(ValueError, pattern):
                self.asm._validate_kast_stdout_kore(bad_stdout)
            st = _clone_stages(self.stages)
            st['kast']['stdout'] = bad_stdout
            st['kast']['receipt']['stdout_sha256'] = _sha256(bad_stdout)
            with self.assertRaisesRegex(ValueError, pattern):
                self.asm.assemble_f03_capture_payload(
                    st,
                    run_input_bytes=self.run_input,
                    rule_span_inventory_bytes=self.inventory,
                    k_commands_bytes=None,
                )

    def test_cat4_negative_prestartup_chain_and_template_refusals(self):
        def check_prestartup(mutator, pattern: str):
            st = _clone_stages(self.stages)
            mutator(st)
            with self.assertRaisesRegex(ValueError, pattern):
                self.asm.assemble_f03_capture_payload(
                    st,
                    run_input_bytes=self.run_input,
                    rule_span_inventory_bytes=self.inventory,
                    k_commands_bytes=None,
                )

        orig_init = self.stages['initializer']['initializer_kore']
        check_prestartup(
            lambda st: st['initializer'].__setitem__(
                'initializer_kore',
                orig_init.replace(b'LblinitGeneratedTopCell{}', b'LblwrongTopCell{}', 1),
            ),
            'initializer.kore does not match deterministic INITIALIZER-RECIPE.json binding of kast/stdout',
        )
        check_prestartup(
            lambda st: st['initializer'].__setitem__('initializer_kore', orig_init[:-2] + b'\n'),
            'initializer.kore does not match deterministic INITIALIZER-RECIPE.json binding of kast/stdout',
        )
        check_prestartup(
            lambda st: st['initializer'].__setitem__('recipe_bytes', b'{"bad": true}\n'),
            'initializer.recipe_bytes does not match pinned INITIALIZER-RECIPE.json',
        )

        def mutate_flag(stage_id: str, flag: str, new_val: str):
            def _mut(st):
                argv = st[stage_id]['receipt']['argv']
                argv[argv.index(flag) + 1] = new_val
            return _mut

        check_prestartup(
            mutate_flag('exec-000', '--pattern', '/tmp/not-initializer.kore'),
            'exec-000.receipt.argv --pattern must reference initializer.kore',
        )
        e0_argv = self.stages['exec-000']['receipt']['argv']
        e0_out = e0_argv[e0_argv.index('--output') + 1]
        check_prestartup(
            mutate_flag('exec-000', '--pattern', e0_out),
            'exec-000.receipt.argv must declare distinct --pattern and --output paths',
        )
        check_prestartup(
            lambda st: st['print-000']['receipt']['argv'].__setitem__(2, '/tmp/wrong-state.kore'),
            'pre-startup path chain mismatch: print-000 input path does not match exec-000 --output path',
        )
        check_prestartup(
            mutate_flag('exec-001', '--pattern', '/tmp/wrong-exec000-output.kore'),
            'pre-startup path chain mismatch: exec-001 --pattern does not match exec-000 --output path',
        )
        check_prestartup(
            mutate_flag('exec-001', '--output', e0_out),
            'pre-startup path chain mismatch: exec-001 --output duplicates exec-000 --output',
        )
        check_prestartup(
            mutate_flag('exec-000', '--smt', 'z3'),
            'pre-startup command template mismatch between exec-000 and exec-001',
        )
        check_prestartup(
            lambda st: st['print-001']['receipt']['argv'].__setitem__(0, '/usr/local/bin/kprint-alt'),
            'pre-startup command template mismatch between print-000 and print-001',
        )

    def test_cat5_poststartup_strict_false_gaps_vs_strict_true_value_error(self):
        def run_both(mutator, strict_err_pattern: str, expected_gaps: tuple[str, ...]):
            st_true = _clone_stages(self.stages)
            mutator(st_true)
            with self.assertRaisesRegex(ValueError, strict_err_pattern):
                self.asm.decode_f03_observations(
                    st_true,
                    run_input_bytes=self.run_input,
                    rule_span_inventory_bytes=self.inventory,
                    source_k_bytes=self.source_k,
                    k_commands_bytes=None,
                    strict_stage_validation=True,
                )
            st_false = _clone_stages(self.stages)
            mutator(st_false)
            res = self.asm.decode_f03_observations(
                st_false,
                run_input_bytes=self.run_input,
                rule_span_inventory_bytes=self.inventory,
                source_k_bytes=self.source_k,
                k_commands_bytes=None,
                strict_stage_validation=False,
            )
            self.assertFalse(res['decoded_run']['comparison_ready'])
            self.assertFalse(res['bundle']['comparison_ready'])
            for gap in expected_gaps:
                self.assertIn(gap, res['decoded_run']['gaps'])
            return res

        run_both(
            lambda st: st['exec-002']['receipt'].__setitem__('exit_status', 1),
            r'exec-002\.receipt\.exit_status must be int 0',
            ('edge-0-engine-receipt-exit-status-nonzero',),
        )
        run_both(
            lambda st: st['exec-002']['receipt'].__setitem__('stderr_sha256', '0' * 64),
            r'exec-002\.stderr SHA256 does not match receipt\.stderr_sha256',
            ('edge-0-engine-receipt-stderr-log-sha256-mismatch',),
        )
        run_both(
            lambda st: st['exec-002']['receipt'].__setitem__('output_sha256', '0' * 64),
            r'exec-002\.state_kore SHA256 does not match receipt\.output_sha256',
            ('edge-0-engine-receipt-output-kore-sha256-mismatch',),
        )
        run_both(
            lambda st: st['print-002']['receipt'].__setitem__('stdout_sha256', '0' * 64),
            r'print-002\.stdout SHA256 does not match receipt\.stdout_sha256',
            ('edge-0-print-receipt-stdout-configuration-sha256-mismatch',),
        )

        def set_depth_2(st):
            argv = st['exec-002']['receipt']['argv']
            argv[argv.index('--depth') + 1] = '2'

        run_both(
            set_depth_2,
            r'exec-002\.receipt\.argv must declare --depth 1',
            (
                'edge-0-engine-receipt-argv-does-not-declare-exactly-depth-1',
                'edge-0-has-no-single-observed-source-rewrite',
            ),
        )

        over_log = b'a' * (self.dec.MAX_LOG_BYTES + 1)
        over_log_sha = _sha256(over_log)
        run_both(
            lambda st: (
                st['exec-002'].__setitem__('stderr', over_log),
                st['exec-002']['receipt'].__setitem__('stderr_sha256', over_log_sha),
            ),
            r'exec-002\.stderr exceeds bound',
            ('edge-0-log-exceeds-decoder-size-bound',),
        )

        over_kore = b'a' * (self.dec.MAX_KORE_BYTES + 1)
        over_kore_sha = _sha256(over_kore)
        run_both(
            lambda st: (
                st['exec-002'].__setitem__('state_kore', over_kore),
                st['exec-002']['receipt'].__setitem__('output_sha256', over_kore_sha),
            ),
            r'exec-002\.state_kore exceeds bound',
            (
                'edge-0-state-kore-exceeds-retention-bound',
                'edge-1-logged-configuration-is-not-input-kore',
            ),
        )

        def terminal_rewrite_mutator(st):
            rw_log = st['exec-095']['stderr']
            st['exec-096']['stderr'] = rw_log
            st['exec-096']['receipt']['stderr_sha256'] = _sha256(rw_log)
            st['exec-096']['receipt']['exit_status'] = 2

        term_res = run_both(
            terminal_rewrite_mutator,
            r'exec-096\.receipt\.exit_status must be int 0',
            (
                'terminal-engine-receipt-exit-status-nonzero',
                'terminal-has-no-observed-no-rule-entry-at-depth-0',
                'terminal-replay-observed-a-rewrite',
                'supplied-terminal-has-another-transition',
            ),
        )
        self.assertIs(term_res['decoded_run']['terminal_probe']['terminal_has_next'], True)

    def test_cat6_ondisk_488_file_symlink_size_oracle_and_span_inventory_guards(self):
        min_kast_out = b'LbluiRun{}(\\dv{SortInt{}}("0"))\n'
        min_init_kore = (
            self.asm.F03_INITIALIZER_PREFIX.encode('ascii')
            + min_kast_out[:-1]
            + self.asm.F03_INITIALIZER_SUFFIX.encode('ascii')
        )
        min_log = b'kore-exec: Info (InfoExecDepth):\n    execution complete: exec depth: 1\n'
        min_kore = b"Lbl'-LT-'generatedTop'-GT-'{}()"
        min_cfg = '<generatedTop><k>.K</k></generatedTop>\n'
        min_receipts = {
            stage_id: dict(self.receipts[stage_id]) for stage_id in self.asm.STAGE_IDS
        }

        with tempfile.TemporaryDirectory() as tmp_root:
            tmp_path = Path(tmp_root)
            base_dir = tmp_path / 'actual_k'
            (base_dir / 'kast').mkdir(parents=True)
            (base_dir / 'kast/stdout').write_bytes(min_kast_out)
            (base_dir / 'kast/stderr').write_bytes(b'')
            (base_dir / 'initializer').mkdir()
            (base_dir / 'initializer/initializer.kore').write_bytes(min_init_kore)
            for idx in range(self.asm.F03_STAGE_MAX_INDEX + 1):
                ed = base_dir / f'exec-{idx:03d}'
                pd = base_dir / f'print-{idx:03d}'
                ed.mkdir()
                pd.mkdir()
                (ed / 'stdout').write_bytes(b'')
                (ed / 'stderr').write_bytes(min_log)
                (ed / 'state.kore').write_bytes(min_kore)
                (pd / 'stdout').write_bytes(min_cfg.encode('ascii'))
                (pd / 'stderr').write_bytes(b'')

            loaded = self.asm.load_stage_artifacts_from_actual_dir(
                base_dir, receipts_by_stage_id=min_receipts
            )
            self.assertEqual(len(loaded), 196)

            sym_base = tmp_path / 'symlink_actual_k'
            sym_base.symlink_to(base_dir)
            with self.assertRaisesRegex(ValueError, 'actual_k_dir must not be a symlink'):
                self.asm.load_stage_artifacts_from_actual_dir(
                    sym_base, receipts_by_stage_id=min_receipts
                )

            real_sub = base_dir / 'exec-000.real'
            (base_dir / 'exec-000').rename(real_sub)
            (base_dir / 'exec-000').symlink_to(real_sub)
            try:
                with self.assertRaisesRegex(ValueError, 'stage directory must not be a symlink'):
                    self.asm.load_stage_artifacts_from_actual_dir(
                        base_dir, receipts_by_stage_id=min_receipts
                    )
            finally:
                (base_dir / 'exec-000').unlink()
                real_sub.rename(base_dir / 'exec-000')

            real_file = base_dir / 'kast/stdout.real'
            (base_dir / 'kast/stdout').rename(real_file)
            (base_dir / 'kast/stdout').symlink_to(real_file)
            try:
                with self.assertRaisesRegex(ValueError, r'kast\.stdout must not be a symlink'):
                    self.asm.load_stage_artifacts_from_actual_dir(
                        base_dir, receipts_by_stage_id=min_receipts
                    )
            finally:
                (base_dir / 'kast/stdout').unlink()
                real_file.rename(base_dir / 'kast/stdout')

            os.truncate(base_dir / 'kast/stdout', self.asm.F03_KAST_STDOUT_MAX_BYTES + 1)
            try:
                with self.assertRaisesRegex(ValueError, r'kast\.stdout file size .* exceeds bound'):
                    self.asm.load_stage_artifacts_from_actual_dir(
                        base_dir, receipts_by_stage_id=min_receipts
                    )
            finally:
                (base_dir / 'kast/stdout').write_bytes(min_kast_out)

            os.truncate(base_dir / 'exec-000/state.kore', self.dec.MAX_KORE_BYTES + 1)
            try:
                with self.assertRaisesRegex(
                    ValueError, r'exec-000\.state_kore file size .* exceeds bound'
                ):
                    self.asm.load_stage_artifacts_from_actual_dir(
                        base_dir, receipts_by_stage_id=min_receipts
                    )
            finally:
                (base_dir / 'exec-000/state.kore').write_bytes(min_kore)

            for forbidden_name in self.asm.FORBIDDEN_ORACLE_BASENAMES:
                bad_dir = tmp_path / forbidden_name
                bad_dir.mkdir()
                with self.assertRaisesRegex(
                    ValueError, 'actual_k_dir references forbidden oracle name'
                ):
                    self.asm.load_stage_artifacts_from_actual_dir(
                        bad_dir, receipts_by_stage_id=min_receipts
                    )

            with mock.patch.object(
                self.asm,
                'FORBIDDEN_ORACLE_SHA256S',
                self.asm.FORBIDDEN_ORACLE_SHA256S | {_sha256(min_kore)},
            ):
                with self.assertRaisesRegex(
                    ValueError, 'matches a? ?forbidden independent oracle'
                ):
                    self.asm.load_stage_artifacts_from_actual_dir(
                        base_dir, receipts_by_stage_id=min_receipts
                    )
                with self.assertRaisesRegex(
                    ValueError, 'matches a forbidden independent oracle artifact SHA256'
                ):
                    self.asm._require_bytes(min_kore, 'probe')

        with self.assertRaisesRegex(ValueError, 'prospective command record'):
            self.asm.load_stage_artifacts_from_actual_dir(
                PROPOSAL_DIR / 'actual/k',
                receipts_by_stage_id={'prospective': True},
            )
        with self.assertRaisesRegex(ValueError, 'independent oracle artifact'):
            self.asm.assemble_f03_capture_payload(
                {'format': 'sherlock-normal-observations/1'},
                run_input_bytes=self.run_input,
                rule_span_inventory_bytes=self.inventory,
            )

        with self.assertRaisesRegex(ValueError, 'rule_span_inventory exceeds bound'):
            self.asm.load_rule_span_inventory(
                b'{' + (b' ' * self.asm.MAX_RULE_SPANS_JSON_BYTES) + b'}',
                require_pinned_sha256=False,
            )
        with self.assertRaisesRegex(ValueError, 'k_commands_bytes exceeds bound'):
            self.asm.load_prospective_k_commands(
                b'{' + (b' ' * self.asm.MAX_K_COMMANDS_JSON_BYTES) + b'}',
                require_pinned_sha256=False,
            )

        inv_doc = json.loads(self.inventory.decode('utf-8'))

        def check_bad_inventory(row_mutator, pattern: str):
            doc = dict(inv_doc)
            doc['rules'] = [dict(r) for r in inv_doc['rules']]
            row_mutator(doc['rules'])
            with self.assertRaisesRegex(ValueError, pattern):
                self.asm.load_rule_span_inventory(doc, require_pinned_sha256=False)

        check_bad_inventory(
            lambda rules: rules.pop(),
            r'rule_span_inventory\.rules must be a list of 113 rows',
        )
        check_bad_inventory(
            lambda rules: rules[0].pop('end_column'),
            r'rule_span_inventory\.rules\[0\] missing required span keys',
        )
        check_bad_inventory(
            lambda rules: rules[0].__setitem__('source_path', '/tmp/other.k'),
            r'rule_span_inventory\.rules\[0\]\.source_path does not match',
        )
        check_bad_inventory(
            lambda rules: rules[0].__setitem__('start_line', 0),
            r'rule_span_inventory\.rules\[0\] requires positive ordered int span coordinates',
        )
        check_bad_inventory(
            lambda rules: (
                rules[0].__setitem__('start_line', 50),
                rules[0].__setitem__('end_line', 49),
            ),
            r'rule_span_inventory\.rules\[0\] requires positive ordered int span coordinates',
        )
        check_bad_inventory(
            lambda rules: rules[0].__setitem__('unique_id', 'not-64-hex'),
            r'rule_span_inventory\.rules\[0\]\.unique_id must be lowercase 64-hex SHA256',
        )
        check_bad_inventory(
            lambda rules: rules[1].update(
                start_line=rules[0]['start_line'],
                start_column=rules[0]['start_column'],
                end_line=rules[0]['end_line'],
                end_column=rules[0]['end_column'],
            ),
            r'rule_span_inventory\.rules\[1\] repeats a span or unique_id',
        )
        check_bad_inventory(
            lambda rules: rules[1].__setitem__('unique_id', rules[0]['unique_id']),
            r'rule_span_inventory\.rules\[1\] repeats a span or unique_id',
        )


class TestMultilineKHeadGuards(_F03GuardTestBase):
    """Covers multiline <k> single KItem positive normalization and negative refusals."""

    def test_multiline_single_k_item_positive_normalization_and_retention(self):
        base_cfg = self.stages['print-001']['stdout'].decode('ascii')
        multiline_head = (
            'uiEval(\n'
            '    uiCall(\n'
            '      3,\n'
            '      uiExpr(uiLiteral(uiCons(uiU(1), uiCons(uiU(2), uiCons(uiU(3), uiNil())))), uiNoExprs())\n'
            '    )\n'
            '  )'
        )
        k_start = base_cfg.index('<k>') + 3
        k_end = base_cfg.index('</k>')
        self.assertNotIn('~>', base_cfg[:k_start] + base_cfg[k_end:])
        multiline_cfg = (
            base_cfg[:k_start] + '\n    ' + multiline_head + ' ~> .K\n  ' + base_cfg[k_end:]
        )
        reduced, note = self.dec._configuration(multiline_cfg)
        self.assertIsNotNone(note)
        self.assertNotIn('~>', reduced)
        decoded = self.dec._decode(multiline_cfg, self.asm.F03_ENTRY_K)
        self.assertEqual(decoded['original_configuration'], multiline_cfg)
        self.assertEqual(
            decoded['snapshot']['control_k'],
            'uiEval(' + self.asm.F03_ENTRY_K + ')',
        )
        for k_ctor, sample in (
            ('uiRet', 'uiRet(\n  uiU(0)\n)'),
            ('uiPush', 'uiPush(\n  uiScopeF(.Map),\n  uiEval(uiVar(0)),\n  0\n)'),
            ('uiError', 'uiError(\n  uiExhausted()\n)'),
        ):
            self.assertIn(k_ctor, self.dec._K_ITEM_CONSTRUCTORS)
            self.assertTrue(self.dec._single_k_item(sample))

    def test_multiline_multiple_items_wrong_root_and_overbound_term_refusals(self):
        base_cfg = self.stages['print-001']['stdout'].decode('ascii')
        k_start = base_cfg.index('<k>') + 3
        k_end = base_cfg.index('</k>')

        def make_cfg(k_body: str) -> str:
            return base_cfg[:k_start] + k_body + base_cfg[k_end:]

        overbound_head = 'uiEval(\n' + (' ' * (self.dec.MAX_TERM_BYTES + 10)) + 'uiVar(0)\n)'
        self.assertGreater(len(overbound_head), self.dec.MAX_TERM_BYTES)
        overbound_cfg = make_cfg('\n  ' + overbound_head + ' ~> .K\n')
        self.assertLessEqual(len(overbound_cfg), self.dec.MAX_CONFIGURATION_BYTES)

        for bad_k_body in (
            '\n  uiEval(\n    uiVar(0)\n  ) ~> uiRet(uiU(0)) ~> .K\n',
            '\n  uiEval(\n    uiVar(0)\n  )\n  uiRet(uiU(0)) ~> .K\n',
            '\n  uiBinary(\n    uiAdd(),\n    uiVar(1),\n    uiVar(2)\n  ) ~> .K\n',
            '\n  uiDone(\n    uiU(0)\n  ) ~> .K\n',
            '\n  uiFrame(\n    uiScopeF(.Map),\n    uiNoFrames()\n  ) ~> .K\n',
            '\n  ' + overbound_head + ' ~> .K\n',
        ):
            cfg = make_cfg(bad_k_body)
            with self.assertRaisesRegex(
                ValueError,
                r'actual-format gap: K sequence other than one <k> item with \.K unit tail',
            ):
                self.dec._configuration(cfg)


class TestDecodeCaptureFullChainGuards(_F03GuardTestBase):
    """Covers whole decode_capture 95-capture/94-edge propagation and tampering refusals."""

    def _run_decode_capture(self, assembled: dict) -> dict:
        return self.dec.decode_capture(
            self.asm.CASE_ID,
            self.asm.F03_ENTRY_K,
            assembled['captures'],
            source_path=self.asm.F03_COMPILED_MAIN_K_PATH,
            rule_spans=assembled['rule_spans'],
            terminal_probe=assembled['terminal_probe'],
            startup_probe=assembled['startup_probe'],
            run_input=assembled['run_input_text'],
            max_captures=self.dec.F03_MAX_CAPTURES,
        )

    def test_full_chain_positive_control_and_edge008_inline_equivalence(self):
        control = self._run_decode_capture(self.assembled_control)
        self.assertTrue(control['comparison_ready'])
        self.assertEqual(control['gaps'], [])
        self.assertEqual(len(control['observed_rule_identities']), 94)
        self.assertTrue(all(r is not None for r in control['observed_rule_identities']))
        links = control['edge_links']
        self.assertEqual(len(links), 94)
        self.assertEqual(
            links[0]['input_kore_sha256'],
            _sha256(self.assembled_control['startup_probe']['state_kore']),
        )
        for idx in range(1, 94):
            self.assertEqual(
                links[idx]['input_kore_sha256'],
                links[idx - 1]['engine_output_kore_sha256'],
            )
        edge8_log_cfg = control['decoded_incoming_logs'][8]['applied_entries'][0]['configuration_text']
        self.assertIn(EDGE008_WRAPPED_KEY_I12, edge8_log_cfg)

        inline_asm = _clone_assembled(self.assembled_control)
        orig_edge8_log = inline_asm['captures'][9]['incoming_log'].decode('ascii')
        self.assertIn(EDGE008_WRAPPED_KEY_I12, orig_edge8_log)
        inline_edge8_log = orig_edge8_log.replace(
            EDGE008_WRAPPED_KEY_I12, EDGE008_INLINE_KEY_I12, 1
        ).encode('ascii')
        inline_asm['captures'][9]['incoming_log'] = inline_edge8_log
        inline_asm['captures'][9]['engine_receipt']['stderr_sha256'] = _sha256(inline_edge8_log)
        inline_decoded = self._run_decode_capture(inline_asm)
        self.assertTrue(inline_decoded['comparison_ready'])
        self.assertEqual(inline_decoded['gaps'], [])
        self.assertEqual(
            [r['rule_start_line'] for r in inline_decoded['observation']['records']],
            [r['rule_start_line'] for r in control['observation']['records']],
        )

    def test_full_chain_logged_input_and_predecessor_state_kore_tampering_refusals(self):
        tampered_log_asm = _clone_assembled(self.assembled_control)
        bad_edge8_log = (
            tampered_log_asm['captures'][9]['incoming_log']
            .decode('ascii')
            .replace(
                EDGE008_WRAPPED_KEY_I12,
                EDGE008_WRAPPED_KEY_I12.replace('"0"', '"1"', 1),
                1,
            )
            .encode('ascii')
        )
        tampered_log_asm['captures'][9]['incoming_log'] = bad_edge8_log
        tampered_log_asm['captures'][9]['engine_receipt']['stderr_sha256'] = _sha256(bad_edge8_log)
        res_log = self._run_decode_capture(tampered_log_asm)
        self.assertFalse(res_log['comparison_ready'])
        self.assertIn('edge-8-logged-configuration-is-not-input-kore', res_log['gaps'])
        self.assertIn('rule_start_lines-not-supplied', res_log['gaps'])
        self.assertIsNone(res_log['observed_rule_identities'][8])
        self.assertIsNotNone(res_log['observed_rule_identities'][7])
        self.assertIsNotNone(res_log['observed_rule_identities'][9])
        self.assertTrue(all(r['rule_start_line'] is None for r in res_log['observation']['records']))
        self.assertEqual(res_log['raw_captures'][9]['incoming_log']['sha256'], _sha256(bad_edge8_log))

        tampered_pred_asm = _clone_assembled(self.assembled_control)
        bad_pred_kore = tampered_pred_asm['captures'][8]['state_kore'].replace(
            b'\\dv{SortInt{}}("85")', b'\\dv{SortInt{}}("84")', 1
        )
        self.assertNotEqual(bad_pred_kore, tampered_pred_asm['captures'][8]['state_kore'])
        tampered_pred_asm['captures'][8]['state_kore'] = bad_pred_kore
        tampered_pred_asm['captures'][8]['engine_receipt']['output_sha256'] = _sha256(bad_pred_kore)
        res_pred = self._run_decode_capture(tampered_pred_asm)
        self.assertFalse(res_pred['comparison_ready'])
        self.assertNotIn('edge-7-engine-receipt-output-kore-sha256-mismatch', res_pred['gaps'])
        self.assertIsNotNone(res_pred['observed_rule_identities'][7])
        self.assertIn('edge-8-logged-configuration-is-not-input-kore', res_pred['gaps'])
        self.assertIsNone(res_pred['observed_rule_identities'][8])
        self.assertEqual(res_pred['edge_links'][8]['input_kore_sha256'], _sha256(bad_pred_kore))

        tampered_term_pred = _clone_assembled(self.assembled_control)
        bad_final_kore = tampered_term_pred['captures'][-1]['state_kore'].replace(
            b'\\dv{SortInt{}}("0")', b'\\dv{SortInt{}}("99")', 1
        )
        tampered_term_pred['captures'][-1]['state_kore'] = bad_final_kore
        tampered_term_pred['captures'][-1]['engine_receipt']['output_sha256'] = _sha256(bad_final_kore)
        res_term = self._run_decode_capture(tampered_term_pred)
        self.assertFalse(res_term['comparison_ready'])
        self.assertIn('terminal-logged-configuration-is-not-input-kore', res_term['gaps'])
        self.assertIn('terminal-replay-changed-raw-kore', res_term['gaps'])
        self.assertIsNone(res_term['terminal_probe']['terminal_has_next'])

    def test_full_chain_engine_and_printer_path_and_template_tampering_refusals(self):
        asm_chain = _clone_assembled(self.assembled_control)
        e8_argv = asm_chain['captures'][9]['engine_receipt']['argv']
        e8_argv[e8_argv.index('--pattern') + 1] = '/tmp/broken-predecessor-link.kore'
        t_argv = asm_chain['terminal_probe']['engine_receipt']['argv']
        t_argv[t_argv.index('--pattern') + 1] = '/tmp/broken-terminal-link.kore'
        e5_argv = asm_chain['captures'][6]['engine_receipt']['argv']
        e4_argv = asm_chain['captures'][5]['engine_receipt']['argv']
        e5_argv[e5_argv.index('--output') + 1] = e4_argv[e4_argv.index('--output') + 1]
        asm_chain['captures'][6]['print_receipt']['argv'][2] = e5_argv[e5_argv.index('--output') + 1]
        e5_argv[e5_argv.index('--smt') + 1] = 'z3'
        asm_chain['captures'][6]['print_receipt']['argv'][0] = '/usr/local/bin/kprint-custom'

        res = self._run_decode_capture(asm_chain)
        self.assertFalse(res['comparison_ready'])
        for expected_gap in (
            'chain-edge-8-pattern-is-not-edge-7-output',
            'chain-terminal-pattern-is-not-edge-93-output',
            'chain-engine-output-paths-not-distinct',
            'chain-engine-command-template-differs',
            'chain-print-command-template-differs',
        ):
            self.assertIn(expected_gap, res['gaps'])


if __name__ == '__main__':
    unittest.main()
