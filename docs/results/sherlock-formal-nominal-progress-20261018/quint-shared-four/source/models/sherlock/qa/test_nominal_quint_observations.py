"""Normal synthetic serialization examples; no model/evaluator/expected-cell input."""
import copy
import unittest

import nominal_quint_observations as subject


def tag(name, payload=()):
    return dict(tag=name, value=payload)


def itf(value):
    if type(value) is bool:
        return value
    if type(value) is int:
        return {'#bigint': str(value)}
    if type(value) is tuple:
        return {'#tup': [itf(item) for item in value]}
    if type(value) is list:
        return [itf(item) for item in value]
    if type(value) is dict:
        if not all(type(key) is str for key in value):
            return {'#map': [[itf(key), itf(item)] for key, item in value.items()]}
        return {key: itf(item) for key, item in value.items()}
    return value


def pending_snapshot():
    scalar = [tag('UIntToken', 9)]
    saved = {20: [tag('BoolToken', True)], 1: scalar}
    construction = dict(typeId=20, ctorId=23, occurrence=1000, source=0,
                        completed=[scalar], remaining=[9], saved=saved)
    return dict(
        program=dict(expressions={9: tag('Variable', 1), 0: tag('Constructor',
            dict(typeId=20, ctorId=23, occurrence=1000, fields=[1, 9]))},
            functions={30: dict(name=30, parameters=[40], parameterTypes=[0], resultType=0, body=9)},
            layouts={(20, 23): dict(typeId=20, ctorId=23, fields=[0, 0])}, entry=0),
        control=tag('Return', scalar), environment=saved,
        frames=[tag('ConstructorFields', construction), tag('ScopeFrame', {})],
        initialFuel=96, remaining=91, consumed=5, helperWork=0,
        constructorNodes=0, constructorFieldSlots=0, outcome=tag('Pending'),
        source=1, event=tag('LiteralEvent'), builds=[], selectedArms=[],
        enteredArguments=[[scalar]], maxDepth=2,
    )


class NormalNominalWireTests(unittest.TestCase):
    def test_complete_pending_frame_maps_and_preorder_values(self):
        snapshot = subject.snapshot_wire(subject.decode_itf(itf(pending_snapshot())))
        self.assertEqual(set(snapshot), subject.SNAPSHOT_KEYS)
        self.assertEqual(len(snapshot), 17)
        self.assertEqual(snapshot['environment'], [
            {'name': 1, 'value': [{'UIntToken': 9}]},
            {'name': 20, 'value': [{'BoolToken': True}]},
        ])
        self.assertEqual(snapshot['program']['layouts'], [dict(typeId=20, ctorId=23, fields=[0, 0])])
        self.assertEqual([row['exprId'] for row in snapshot['program']['expressions']], [0, 9])
        self.assertEqual(snapshot['program']['functions'], [dict(name=30,
            function=dict(name=30, parameters=[40], parameterTypes=[0], resultType=0, body=9))])
        frame = snapshot['frames'][0]['ConstructorFields']
        self.assertEqual(frame['completed'], [[{'UIntToken': 9}]])
        self.assertEqual(frame['remaining'], [9])
        self.assertEqual(frame['saved'], snapshot['environment'])
        self.assertEqual(snapshot['frames'][1], {'ScopeFrame': []})
        self.assertEqual(snapshot['control'], {'Return': [{'UIntToken': 9}]})
        self.assertEqual(snapshot['event'], 'LiteralEvent')
        self.assertEqual(snapshot['enteredArguments'], [[[{'UIntToken': 9}]]])

    def test_raw_trace_and_terminal_stutter_ordinals_are_retained(self):
        pending = pending_snapshot()
        terminal = copy.deepcopy(pending)
        terminal.update(control=tag('Stopped'), frames=[], outcome=tag('Done', [tag('UIntToken', 9)]),
                        remaining=90, consumed=6, event=tag('TerminalPublication'))
        # Synthetic encoding states only; this test never claims a transition
        # relation from the invented pending carrier to the terminal carrier.
        trace = dict(states=[{'machine': itf(state)} for state in [pending, pending, terminal, terminal]])
        case = dict(caseId='synthetic-format-only', expressions=[dict(exprId=9, occurrence=1009),
            dict(exprId=0, occurrence=1000)], declarations=[dict(typeId=20, recursiveGroup=9)])
        bundle = subject.normalize_trace(case, trace, terminal_step_contract='synthetic terminal encoding')
        self.assertEqual(bundle['raw_itf'], trace)
        self.assertEqual(bundle['projection']['comparison_to_raw_ordinals'], [0, 1, 2])
        self.assertEqual(bundle['projection']['exact_terminal_stutter_ordinals'], [3])
        self.assertEqual(bundle['observation']['sourceOccurrences'], [dict(exprId=0, occurrence=1000),
            dict(exprId=9, occurrence=1009)])
        self.assertEqual(bundle['observation']['recursiveGroups'], [dict(typeId=20, recursiveGroup=9)])
        self.assertEqual(len(bundle['observation']['states']), 3)
        self.assertEqual(bundle['evidence']['producer_authenticity'], 'not-established')


if __name__ == '__main__':
    unittest.main()
