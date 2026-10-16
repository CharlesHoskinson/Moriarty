"""Ordinary finite diagnostic syntax, independently specified from KDisplay.

These hand-authored strings and frozen literal documents are not K, Quint,
Lean, circuit or native execution observations.
"""
import json
from pathlib import Path
import unittest

try:
    import k_terms as terms
except ModuleNotFoundError:
    terms = None


class DiagnosticTermTests(unittest.TestCase):
    def setUp(self):
        self.assertIsNotNone(terms, 'shared diagnostic term normalizer is required')

    def test_scalar_and_empty_display(self):
        for source, expected in [
                ('  .K \n', '.K'), (' .Map ', '.Map'),
                (' uiNoFrames ( ) ', 'uiNoFrames()'),
                (' uiU(340282366920938463463374607431768211455) ',
                 'uiU(340282366920938463463374607431768211455)'),
                (' uiVar ( -12 ) ', 'uiVar(-12)'),
                (' uiB ( true ) ', 'uiB(true)'),
                (' uiB ( false ) ', 'uiB(false)')]:
            with self.subTest(source=source):
                self.assertEqual(terms.normalize_term(source), expected)

    def test_decimal_meaning_and_boolean_are_preserved(self):
        self.assertEqual(terms.normalize_term(' uiLet ( -00, uiLiteral(uiU(007)), uiVar(00)) '),
                         'uiLet(0, uiLiteral(uiU(7)), uiVar(0))')
        self.assertEqual(terms.normalize_term('uiIf(uiLiteral(uiB(false)), '
                                             'uiLiteral(uiU(1)), uiLiteral(uiU(2)))'),
                         'uiIf(uiLiteral(uiB(false)), uiLiteral(uiU(1)), uiLiteral(uiU(2)))')

    def test_lists_and_trees_keep_source_order(self):
        source = ('uiBranch ( uiLeaf (uiU(9)), uiLeaf(uiU(2)))')
        self.assertEqual(terms.normalize_term(source),
                         'uiBranch(uiLeaf(uiU(9)), uiLeaf(uiU(2)))')
        self.assertEqual(terms.normalize_term('uiCons (uiU(9),uiCons(uiU(2),uiNil()))'),
                         'uiCons(uiU(9), uiCons(uiU(2), uiNil()))')

    def test_map_display_matches_host_string_key_order(self):
        source = '2 |-> uiU(20) -2 |-> uiU(2) 10 |-> uiB(true) -10 |-> uiU(10)'
        expected = '-10 |-> uiU(10) -2 |-> uiU(2) 10 |-> uiB(true) 2 |-> uiU(20)'
        self.assertEqual(terms.normalize_term(source), expected)
        self.assertEqual(terms.normalize_term('(' + source + ')'), expected)

    def test_saved_environments_and_frame_order(self):
        source = ('uiFrame (uiArgsF(4,uiValue(uiU(8),uiNoValues()),'
                  'uiExpr(uiLiteral(uiU(3)),uiNoExprs()),'
                  '(2 |-> uiU(20) 10 |-> uiU(10))),'
                  'uiFrame(uiScopeF(.Map),uiNoFrames()))')
        expected = ('uiFrame(uiArgsF(4, uiValue(uiU(8), uiNoValues()), '
                    'uiExpr(uiLiteral(uiU(3)), uiNoExprs()), '
                    '10 |-> uiU(10) 2 |-> uiU(20)), '
                    'uiFrame(uiScopeF(.Map), uiNoFrames()))')
        self.assertEqual(terms.normalize_term(source), expected)
        node = terms.parse_term(source)
        self.assertEqual(node[0], 'uiFrame')
        self.assertEqual(node[1][0][0], 'uiArgsF')
        self.assertEqual(node[1][1][1][0][0], 'uiScopeF')
        self.assertEqual(terms.render_term(node), expected)

    def test_function_parameter_and_case_order(self):
        source = ('uiFunction(uiFun(12,uiParam(2,uiUIntT(),'
                  'uiParam(10,uiListT(),uiNoParams())),uiTreeT(),'
                  'uiMatch(uiVar(10),uiCase(uiPNil(),uiLiteral(uiLeaf(uiU(0))),'
                  'uiCase(uiPCons(5,6),uiUnary(uiLeafOp(),uiVar(5)),uiNoCases())))),'
                  'uiNoFuns())')
        expected = ('uiFunction(uiFun(12, uiParam(2, uiUIntT(), '
                    'uiParam(10, uiListT(), uiNoParams())), uiTreeT(), '
                    'uiMatch(uiVar(10), uiCase(uiPNil(), uiLiteral(uiLeaf(uiU(0))), '
                    'uiCase(uiPCons(5, 6), uiUnary(uiLeafOp(), uiVar(5)), uiNoCases())))), '
                    'uiNoFuns())')
        self.assertEqual(terms.normalize_term(source), expected)

    def test_helper_results_are_only_rendered(self):
        for source, expected in [
                ('uiPrimResult (uiPrimValue(uiU(5)))', 'uiPrimResult(uiPrimValue(uiU(5)))'),
                ('uiBound(uiBindings(2 |-> uiU(2) 10 |-> uiU(10)),uiUIntT(),uiVar(2))',
                 'uiBound(uiBindings(10 |-> uiU(10) 2 |-> uiU(2)), uiUIntT(), uiVar(2))'),
                ('uiError(3)', 'uiError(3)'),
                ('uiRejected(3)', 'uiRejected(3)'),
                ('uiDone(uiB(true))', 'uiDone(uiB(true))')]:
            with self.subTest(source=source):
                self.assertEqual(terms.normalize_term(source), expected)

    def test_frozen_normal_literal_term_corpus_roundtrips(self):
        root = Path(__file__).resolve().parents[1] / 'oracles'
        visited = []

        def walk(value):
            if type(value) is dict:
                for key, child in value.items():
                    if key.endswith('_k') and type(child) is str:
                        visited.append(child)
                        canonical = terms.normalize_term(child)
                        self.assertEqual(terms.normalize_term(canonical), canonical)
                    elif key == 'frames_top_first_k':
                        for frame in child:
                            visited.append(frame)
                            canonical = terms.normalize_term(frame)
                            self.assertEqual(terms.normalize_term(canonical), canonical)
                    else:
                        walk(child)
            elif type(value) is list:
                for child in value:
                    walk(child)

        for name in ('normal-programs.json', 'normal-prefixes.json',
                     'extra-normal-programs.json', 'extra-normal-prefixes.json'):
            walk(json.loads((root / name).read_text()))
        self.assertGreater(len(visited), 500)


if __name__ == '__main__':
    unittest.main()
