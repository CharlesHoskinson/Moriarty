"""Focused BoundaryKind unit-enum port regression; synthetic only.

Derives expectations from the public StepEventC declaration
(boundary: BoundaryKind, a plain unit enum) and the pre-existing
public Wire rule (unit variant -> exact variant-name String;
Option None -> JSON null). No real afterstate, no expected operand,
no private oracle, and no transition computation in Python.
Imports only the new candidate03 corrected port, never the frozen
original port.
"""
import importlib.util
import unittest
from pathlib import Path

_PORT_PATH = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01/actual-before-field-port-source01/designated-owner-existing-R2-window-runtime-source-request01/bounded-renderer-decoder-only-request02/literal-owner-runtime-candidate01/focused-owner-map-and-query-inverse-fix01/literal-corrected-runtime-candidate02/after-freeze-actual-only-window-diagnosis01/publicport-boundary-enum-source-correction-request01/literal-owner-boundary-correction-candidate03/source/envget_ports.py')
_spec = importlib.util.spec_from_file_location(
    'envget_ports_candidate03', _PORT_PATH)
p = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(p)


def _int(value):
    return p.literal('Int', value)


def synthetic_event(origin, phase_before, phase_after):
    step = p.node('egStepId',
                  p.from_json({'run': ['x"y', 1, True, None]}), _int(0))
    alloc = p.node('ctAlloc', _int(0), _int(0), _int(0))
    occurrence = p.node(
        'egOccurrenceLocator', p.literal('Bool', False), _int(0),
        p.node('egValue', p.node('egInteger', _int(0)),
               p.node('egNoValues')))
    args = [step, p.node('egEnvGetCellRule'), p.node('egHelperVisit'),
            p.node('egSiteSome', p.node('chSiteEvalVar')),
            p.node('egSomeEnvGet'), p.node('egApplied'),
            _int(1), _int(1), _int(1), _int(0), alloc, _int(0),
            _int(2), _int(2), _int(0), _int(0),
            phase_before, phase_after,
            p.node('chIndexSome', _int(0)),
            p.node('chIndexSome', _int(0)),
            p.node('egNoResultTag'), origin, _int(3),
            occurrence, p.node('egNoExhaustion'),
            p.node('egRunningTag'), p.node('ctRunning'),
            p.node('egNoBoundary')]
    assert len(args) == 28
    return p.node('egEvent', *args)


class BoundaryEnumMapping(unittest.TestCase):
    def test_imports_new_corrected_port_only(self):
        self.assertEqual(Path(p.__file__).resolve(), _PORT_PATH.resolve())

    def test_boundary_kind_none_unit_maps_to_string_none(self):
        value = p.event_field(p.node('egNoBoundary'))
        self.assertEqual(value, 'None')
        self.assertIs(type(value), str)
        self.assertIsNotNone(value)

    def test_true_optional_none_constructors_stay_json_null(self):
        for name in ('egNoPhase', 'egNoResultTag', 'egNatNone',
                     'egNoExhaustion'):
            self.assertIsNone(p.event_field(p.node(name)), name)

    def test_nat_some_zero_is_distinct_from_nat_none(self):
        self.assertEqual(p.event_field(p.node('egNatSome', _int(0))),
                         {'Some': 0})
        self.assertIsNone(p.event_field(p.node('egNatNone')))
        self.assertNotEqual(p.event_field(p.node('egNatSome', _int(0))),
                            p.event_field(p.node('egNatNone')))

    def test_complete_28_field_event_boundary_plain_unit_optional_nulls(self):
        event = p.to_event(synthetic_event(
            p.node('egNatNone'), p.node('egNoPhase'),
            p.node('egNoPhase')))
        self.assertEqual(list(event), list(p.EVENT_FIELDS))
        self.assertEqual(len(event), 28)
        self.assertEqual(event['boundary'], 'None')
        self.assertIs(type(event['boundary']), str)
        self.assertIsNone(event['helper_phase_before'])
        self.assertIsNone(event['helper_phase_after'])
        self.assertIsNone(event['result_tag'])
        self.assertIsNone(event['origin_reference_cursor'])
        self.assertIsNone(event['exhausted_kind'])

    def test_complete_28_field_event_some_zero_and_boundary_coexist(self):
        event = p.to_event(synthetic_event(
            p.node('egNatSome', _int(0)),
            p.node('egTriPhase', p.node('chTriPending')),
            p.node('egTriPhase', p.node('chTriPositive'))))
        self.assertEqual(list(event), list(p.EVENT_FIELDS))
        self.assertEqual(len(event), 28)
        self.assertEqual(event['boundary'], 'None')
        self.assertEqual(event['origin_reference_cursor'], {'Some': 0})
        self.assertEqual(event['helper_phase_before'],
                         {'Some': {'Tri': 'Pending'}})
        self.assertEqual(event['helper_phase_after'],
                         {'Some': {'Tri': 'Positive'}})
        self.assertEqual(event['helper_cursor_before'], {'Some': 0})
        self.assertIsNone(event['exhausted_kind'])


if __name__ == '__main__':
    unittest.main()
