"""Supplied-data offline regression tests for the F03 95-capture decoder bound.

No K execution, subprocess launch, or fabricated actual K log/Kore capture is
present. F03 run_input and expected post-startup states come from the frozen
R251/R260 supplied artifacts. Synthetic 95-capture inputs exercise offline
capture-count admission (64, 95, 96, and explicit max_captures) while remaining
comparison_ready=False because actual F03 K logs, state.kore files, startup
probe, terminal probe, and receipts have not yet been captured.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import unittest


MODULE = Path(__file__).with_name('k_capture_observations.py')
SPEC = importlib.util.spec_from_file_location('k_capture_observations', MODULE)
k = None
if MODULE.exists():
    k = importlib.util.module_from_spec(SPEC)
    SPEC.loader.exec_module(k)

OBS_MODULE = Path(__file__).with_name('k_observations.py')
OBS_SPEC = importlib.util.spec_from_file_location('k_observations', OBS_MODULE)
obs = None
if OBS_MODULE.exists():
    obs = importlib.util.module_from_spec(OBS_SPEC)
    OBS_SPEC.loader.exec_module(obs)

SOURCE = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/compile-input/models/sherlock/k/main.k'
F03_RUN_INPUT = (
    'uiRun(uiFunction(uiFun(3, uiParam(0, uiListT(), uiNoParams()), uiUIntT(), '
    'uiMatch(uiVar(0), uiCase(uiPNil(), uiLiteral(uiU(0)), uiCase(uiPCons(1, 2), '
    'uiBinary(uiAdd(), uiVar(1), uiCall(3, uiExpr(uiVar(2), uiNoExprs()))), '
    'uiNoCases())))), uiNoFuns()), uiCall(3, uiExpr(uiLiteral(uiCons(uiU(1), '
    'uiCons(uiU(2), uiCons(uiU(3), uiNil())))), uiNoExprs())), 93, 64)\n'
)
F03_RUN_INPUT_RECORD = (
    359, 'c1ab51c62fc70ea5693e1d9046445cba6a03eb3222273c045b511913c0f6b91b'
)
F03_PROGRAM_IDENTITY = '832f1c6521477d052b1021cb7cff25c2a7da3b807e6374243479e7b3126e4100'
F03_CODE_K = (
    'uiFunction(uiFun(3, uiParam(0, uiListT(), uiNoParams()), uiUIntT(), '
    'uiMatch(uiVar(0), uiCase(uiPNil(), uiLiteral(uiU(0)), uiCase(uiPCons(1, 2), '
    'uiBinary(uiAdd(), uiVar(1), uiCall(3, uiExpr(uiVar(2), uiNoExprs()))), '
    'uiNoCases())))), uiNoFuns())'
)
F03_ENTRY_K = (
    'uiCall(3, uiExpr(uiLiteral(uiCons(uiU(1), uiCons(uiU(2), '
    'uiCons(uiU(3), uiNil())))), uiNoExprs()))'
)
F03_EXPECTED_RULE_LINES = (
    [131, 135, 102, 181, 200, 204, 208, 135]
    + [126, 135, 108, 173, 214, 217, 214, 220, 135,
       117, 135, 108, 149,
       131, 135, 108, 181, 200, 204, 208, 135] * 3
    + [126, 135, 108, 173, 214, 220, 135, 102, 195, 185]
    + [153, 223, 195, 185] * 3
    + [80]
)


def _stack_text(frames_top_first):
    node = 'uiNoFrames()'
    for frame in reversed(frames_top_first):
        node = f'uiFrame({frame}, {node})'
    return node


def _config_text(control_k, env_k, frames_top_first, depth, fuel, out_k='uiPending()', unit_tail=True):
    k_body = f'{control_k} ~> .K' if unit_tail else control_k
    return (
        '<generatedTop>\n'
        f'  <k>\n    {k_body}\n  </k>\n'
        f'  <env>\n    {env_k}\n  </env>\n'
        f'  <stack>\n    {_stack_text(frames_top_first)}\n  </stack>\n'
        f'  <depth>\n    {depth}\n  </depth>\n'
        '  <stackLimit>\n    64\n  </stackLimit>\n'
        f'  <code>\n    {F03_CODE_K}\n  </code>\n'
        f'  <fuel>\n    {fuel}\n  </fuel>\n'
        '  <initialFuel>\n    93\n  </initialFuel>\n'
        f'  <out>\n    {out_k}\n  </out>\n'
        '</generatedTop>\n'
    )


def _synthetic_f03_95_configurations():
    """Build 95 synthetic post-startup configurations with F03's fuel/depth/publication trajectory.

    Includes the exact initial configuration, the exact peak depth-11 frame stack
    from F010 state 73, and the exact free publication terminal state.
    """
    L123 = 'uiCons(uiU(1), uiCons(uiU(2), uiCons(uiU(3), uiNil())))'
    L23 = 'uiCons(uiU(2), uiCons(uiU(3), uiNil()))'
    L3 = 'uiCons(uiU(3), uiNil())'
    CASES = (
        'uiCase(uiPNil(), uiLiteral(uiU(0)), uiCase(uiPCons(1, 2), '
        'uiBinary(uiAdd(), uiVar(1), uiCall(3, uiExpr(uiVar(2), uiNoExprs()))), '
        'uiNoCases()))'
    )
    peak_frames = [
        f'uiMatchF({CASES}, 0 |-> uiNil())',
        f'uiReturnF(uiUIntT(), 0 |-> {L3} 1 |-> uiU(3) 2 |-> uiNil())',
        'uiRightF(uiAdd(), uiU(3))',
        f'uiScopeF(0 |-> {L3})',
        f'uiReturnF(uiUIntT(), 0 |-> {L23} 1 |-> uiU(2) 2 |-> {L3})',
        'uiRightF(uiAdd(), uiU(2))',
        f'uiScopeF(0 |-> {L23})',
        f'uiReturnF(uiUIntT(), 0 |-> {L123} 1 |-> uiU(1) 2 |-> {L23})',
        'uiRightF(uiAdd(), uiU(1))',
        f'uiScopeF(0 |-> {L123})',
        'uiReturnF(uiUIntT(), .Map)',
    ]
    configs = [
        _config_text(f'uiEval({F03_ENTRY_K})', '.Map', [], 0, 93),
    ]
    for step in range(1, 93):
        remaining = 93 - step
        if step == 73:
            configs.append(_config_text('uiEval(uiVar(0))', '0 |-> uiNil()', peak_frames, 11, remaining))
        else:
            configs.append(_config_text('uiRet(uiU(6))', '.Map', ['uiReturnF(uiUIntT(), .Map)'], 1, remaining))
    configs.append(_config_text('uiRet(uiU(6))', '.Map', [], 0, 0))
    configs.append(_config_text('.K', '.Map', [], 0, 0, out_k='uiDone(uiU(6))', unit_tail=False))
    return configs


class OfflineF03CaptureBoundTests(unittest.TestCase):
    def setUp(self):
        self.assertIsNotNone(k, 'capture adapter module is absent')
        self.assertIsNotNone(obs, 'observations module is absent')

    def test_f03_supplied_run_input_and_program_identity(self):
        raw = F03_RUN_INPUT.encode('ascii')
        self.assertEqual((len(raw), hashlib.sha256(raw).hexdigest()), F03_RUN_INPUT_RECORD)
        retained = k.retained_run_input(F03_RUN_INPUT)
        self.assertEqual(retained['code_k'], F03_CODE_K)
        self.assertEqual(retained['entry_k'], F03_ENTRY_K)
        self.assertEqual(retained['initial_fuel'], 93)
        self.assertEqual(retained['stack_limit'], 64)
        self.assertEqual(retained['raw_sha256'], F03_RUN_INPUT_RECORD[1])
        program_record = dict(
            case_id='F03',
            code_k=retained['code_k'],
            entry_k=retained['entry_k'],
            exact_completion_fuel=retained['initial_fuel'],
            initial_arguments='Explicit literals in entry_k; no external arguments.',
            stack_limit=retained['stack_limit'],
        )
        canonical = json.dumps(program_record, separators=(',', ':'), sort_keys=True).encode('utf-8')
        self.assertEqual(hashlib.sha256(canonical).hexdigest(), F03_PROGRAM_IDENTITY)

    def test_all_other_per_artifact_bounds_remain_unchanged(self):
        self.assertEqual(k.MAX_CAPTURES, 64)
        self.assertEqual(k.F03_MAX_CAPTURES, 95)
        self.assertEqual(k.MAX_LOG_BYTES, 4 * 1024 * 1024)
        self.assertEqual(k.MAX_LOG_LINES, 65536)
        self.assertEqual(k.MAX_KORE_BYTES, 1024 * 1024)
        self.assertEqual(k.MAX_CONFIGURATION_BYTES, 8192)
        self.assertEqual(k.MAX_NESTING, 32)
        self.assertEqual(k.MAX_TERM_BYTES, 4096)
        self.assertEqual(k.MAX_ARGV, 64)
        self.assertEqual(k.MAX_ARG_CHARS, 4096)
        self.assertEqual(k.MAX_RULE_SPANS, 4096)

    def test_f03_95_capture_selection_and_peak_depth_11_bounds(self):
        self.assertEqual(len(F03_EXPECTED_RULE_LINES), 94)
        configs = _synthetic_f03_95_configurations()
        self.assertEqual(len(configs), 95)
        reduced = [k._configuration(text)[0] for text in configs]
        decoded_run = obs.decode_run(
            'F03', F03_ENTRY_K, reduced,
            rule_start_lines=list(F03_EXPECTED_RULE_LINES),
            terminal_has_next=False,
        )
        self.assertEqual(decoded_run['gaps'], [])
        self.assertTrue(decoded_run['comparison_ready'])
        self.assertEqual(len(decoded_run['observation']['records']), 94)
        self.assertEqual(
            [r['charge'] for r in decoded_run['observation']['records']],
            [1] * 93 + [0],
        )
        self.assertEqual(decoded_run['observation']['terminal']['output_k'], 'uiDone(uiU(6))')
        self.assertEqual(
            max(d['snapshot']['depth'] for d in decoded_run['decoded_configurations']),
            11,
        )
        self.assertEqual(
            len(decoded_run['decoded_configurations'][73]['snapshot']['frames_top_first_k']),
            11,
        )
        captures_95 = [
            dict(configuration=configs[0]),
            *[
                dict(configuration=text, incoming_log=b'', state_kore=b'')
                for text in configs[1:]
            ],
        ]
        with self.assertRaises(ValueError):
            k.decode_capture(
                'F03', F03_ENTRY_K, captures_95,
                source_path=SOURCE, run_input=F03_RUN_INPUT,
            )
        with self.assertRaises(ValueError):
            k.decode_capture(
                'F03', F03_ENTRY_K, captures_95[:k.MAX_CAPTURES + 1],
                source_path=SOURCE,
            )
        default_64 = k.decode_capture(
            'F03', F03_ENTRY_K, captures_95[:k.MAX_CAPTURES],
            source_path=SOURCE,
        )
        self.assertEqual(len(default_64['raw_captures']), k.MAX_CAPTURES)
        self.assertFalse(default_64['comparison_ready'])
        result = k.decode_capture(
            'F03', F03_ENTRY_K, captures_95,
            source_path=SOURCE, run_input=F03_RUN_INPUT,
            max_captures=k.F03_MAX_CAPTURES,
        )
        self.assertEqual(len(result['raw_captures']), 95)
        self.assertEqual(result['observation']['initial']['initial_fuel'], 93)
        self.assertEqual(result['observation']['initial']['stack_limit'], 64)
        self.assertEqual(result['observation']['terminal']['output_k'], 'uiDone(uiU(6))')
        self.assertFalse(result['comparison_ready'])
        self.assertIn('rule-span-inventory-not-supplied', result['gaps'])
        self.assertIn('startup-probe-not-supplied', result['gaps'])
        self.assertIn('terminal-has-next-not-supplied', result['gaps'])
        for field in ('code_k', 'entry_k', 'initial_fuel', 'stack_limit'):
            self.assertNotIn(
                'run-input-' + field + '-differs-from-post-startup-initial',
                result['gaps'],
            )
        with self.assertRaises(ValueError):
            k.decode_capture(
                'F03', F03_ENTRY_K, captures_95,
                source_path=SOURCE, max_captures=k.MAX_CAPTURES,
            )
        with self.assertRaises(ValueError):
            k.decode_capture(
                'F03', F03_ENTRY_K,
                captures_95 + [dict(configuration=configs[-1], incoming_log=b'', state_kore=b'')],
                source_path=SOURCE, max_captures=k.F03_MAX_CAPTURES,
            )
        with self.assertRaises(TypeError):
            k._captures(captures_95, k.F03_MAX_CAPTURES)
        for bad_limit in (0, -1, k.F03_MAX_CAPTURES + 1, True, False, '95', None):
            with self.subTest(max_captures=bad_limit):
                with self.assertRaises(ValueError):
                    k.decode_capture(
                        'F03', F03_ENTRY_K, captures_95[:1],
                        source_path=SOURCE, max_captures=bad_limit,
                    )


if __name__ == '__main__':
    unittest.main()
