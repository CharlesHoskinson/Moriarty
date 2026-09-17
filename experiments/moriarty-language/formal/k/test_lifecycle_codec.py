"""Transport-only tests: these fixtures are not K execution evidence."""
import unittest

import expression_codec as ex
import lifecycle_codec as lc


def packet(financial='{}'):
    return lc.admit_packet({
        'schema': '{}', 'request': '{}', 'financialPreState': financial,
    })


def terminal(admitted, result, *, financial=None, continuation=None):
    fields = admitted['fields']
    cells = [
        ex._apply('<k>', continuation or {'node': 'KSequence', 'arity': 0, 'items': []}),
        ex._apply('<out>', result),
        ex._apply('<sigma>', fields['schema']['term']),
        ex._apply('<request>', fields['request']['term']),
        ex._apply('<financial>', financial or fields['financialPreState']['term']),
    ]
    return ex.serialize_term({
        'format': 'KAST', 'version': 4, 'term': ex._apply('<generatedTop>', *cells),
    })


def kernel_rejection():
    return ex._apply('lcKernelRejected', ex._string('SCHEMA'), ex._apply('lcActionIndexNull'))


class LifecycleCodecTests(unittest.TestCase):
    def test_only_omission_means_missing_factory_context(self):
        missing = lc.admit_packet({'schema': '{}', 'request': '{}'})
        self.assertEqual(missing['constructor'], 'lifecycleMissingContext')
        for value in [None, False, {}, 0]:
            with self.subTest(value=value), self.assertRaises(lc.CodecError):
                lc.admit_packet({'schema': '{}', 'request': '{}', 'financialPreState': value})

    def test_financial_bounds_and_parse_failures_reach_k_after_static_checks(self):
        for text in ['x' * 65537, '{', '{"amount":NaN}']:
            with self.subTest(length=len(text)):
                admitted = packet(text)
                self.assertEqual(admitted['constructor'], 'lifecycleRequest')
                self.assertEqual(admitted['fields']['financialPreState']['original'], text)
        # A real JSON object equal to the old failure sentinel is still parsed.
        parsed = packet('{"tag":"parseFailure"}')['fields']['financialPreState']['term']
        self.assertEqual(parsed['args'][1]['label']['name'], 'lcParsed')

    def test_json_parse_and_clone_lexical_behavior(self):
        raw = ' { "x": 1, "x": 2, "large": 9007199254740993, "overflow": 1e400 } '
        admitted = packet(raw)['fields']['financialPreState']
        self.assertEqual(admitted['original'], raw)
        self.assertEqual(admitted['parsed'], {'x': 2, 'large': 9007199254740992, 'overflow': None})
        for invalid in ['NaN', 'Infinity', '[1,]', '{"a":1,}', 'true false', '01']:
            with self.subTest(invalid=invalid):
                self.assertIs(lc.financial_parse(invalid), lc.PARSE_FAILURE)
        with self.assertRaises(lc.CodecError):
            lc.admit_packet({'schema': '{"x":1,"x":2}', 'request': '{}'})

    def test_escaped_lone_surrogates_remain_lexical_input_for_k(self):
        for raw in ['{"party":"\\ud800"}', '{"\\ud800":1}']:
            admitted = packet(raw)
            parsed = admitted['fields']['financialPreState']['term']['args'][1]
            self.assertEqual(parsed['label']['name'], 'lcParsed')
            self.assertEqual(admitted['fields']['financialPreState']['original'], raw)

    def test_mutated_parsed_financial_input_is_not_a_bound_result(self):
        admitted = packet()
        altered = ex._apply('lcText', ex._string('{}'),
                            ex._apply('lcParsed', lc.encode_json_value({'extra': True})))
        with self.assertRaises(lc.CodecError):
            lc.decode_result(terminal(admitted, kernel_rejection(), financial=altered), admitted)

    def test_llvm_utf8_tokens_bind_to_original_unicode_financial_text(self):
        admitted = packet('"😀"')
        # LLVM's observed trace109 byte spelling, in both the original JSON
        # string and its parsed EJSON string. The raw original includes quotes.
        financial = ex._apply(
            'lcText', ex._token('String', '"\\"\\xf0\\x9f\\x98\\x80\\""'),
            ex._apply('lcParsed', ex._apply(
                'ejString', ex._token('String', '"\\xf0\\x9f\\x98\\x80"'))),
        )
        self.assertEqual(lc.decode_result(
            terminal(admitted, kernel_rejection(), financial=financial), admitted),
            {'status': 'Rejected', 'code': 'SCHEMA', 'actionIndex': None})
        # Normalization must not excuse an extra token field or invalid UTF-8.
        financial['args'][1]['args'][0]['args'][0]['extra'] = 'ignored'
        with self.assertRaises(lc.CodecError):
            lc.decode_result(terminal(admitted, kernel_rejection(), financial=financial), admitted)

    def test_nonempty_diagnostic_path_survives_transport(self):
        admitted = packet()
        result = ex._apply(
            'lcExpressionRejected', ex._string('ENSURES_FAILED'),
            lc.encode_json_value({'kind': 'source', 'start': '1', 'end': '2'}),
            ex._list([lc.encode_json_value('3'), lc.encode_json_value('0')]),
            ex._token('Int', '99'),
        )
        self.assertEqual(lc.decode_result(terminal(admitted, result), admitted), {
            'status': 'Rejected', 'code': 'ENSURES_FAILED',
            'span': {'kind': 'source', 'start': '1', 'end': '2'},
            'nodePath': ['3', '0'], 'workUsed': '99',
        })

    def test_deep_financial_input_does_not_depend_on_python_recursion_limit(self):
        admitted = packet('[' * 2000 + '0' + ']' * 2000)
        self.assertEqual(lc.decode_result(terminal(admitted, kernel_rejection()), admitted), {
            'status': 'Rejected', 'code': 'SCHEMA', 'actionIndex': None,
        })

    def test_stuck_or_unknown_output_is_an_execution_failure(self):
        admitted = packet()
        continuation = {'node': 'KSequence', 'arity': 1, 'items': [ex._apply('lcPending')]}
        with self.assertRaises(lc.CodecError):
            lc.decode_result(terminal(admitted, kernel_rejection(), continuation=continuation), admitted)
        with self.assertRaises(lc.CodecError):
            lc.decode_result(terminal(admitted, ex._apply('unknownResult')), admitted)


if __name__ == '__main__':
    unittest.main()
