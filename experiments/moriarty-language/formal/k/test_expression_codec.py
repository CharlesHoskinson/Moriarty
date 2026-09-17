"""String tokens observed in the pinned WSL LLVM KAST output."""
import unittest

import expression_codec as ex


class ExpressionStringTokenTests(unittest.TestCase):
    def decode(self, token):
        return ex.decode_term(ex._apply('ejString', ex._token('String', token)))

    def test_actual_trace109_utf8_byte_escapes(self):
        # trace-109.stdout, LitText-k-token-astral: K exited zero and emitted
        # these bytes in both immutable <request> and terminal exValue.
        self.assertEqual(self.decode('"\\xf0\\x9f\\x98\\x80"'), '😀')

    def test_multibyte_bmp_and_mixed_escape_forms(self):
        self.assertEqual(self.decode('"\\xc3\\xa9\\xe2\\x82\\xac"'), 'é€')
        self.assertEqual(self.decode('"\\u00e9\\U0001f600\\n\\x00"'), 'é😀\n\0')
        self.assertEqual(self.decode('"é\\xf0\\x9f\\x98\\x80"'), 'é😀')

    def test_no_second_decode_or_mojibake_heuristic(self):
        self.assertEqual(self.decode('"\\xc3\\xb0\\xc2\\x9f\\xc2\\x98\\xc2\\x80"'),
                         'ð\x9f\x98\x80')
        self.assertEqual(self.decode('"\\\\xf0"'), '\\xf0')
        self.assertEqual(self.decode('"\\u00f0"'), 'ð')

    def test_encoder_preserves_scalar_roundtrips(self):
        for value in ['ASCII', '\0\x08\x1b\x7f', '\x80éÿ', '€😀', '\\"\n\r\t\f']:
            with self.subTest(value=value):
                self.assertEqual(self.decode(ex._quote_k_string(value)), value)

    def test_invalid_utf8_and_invalid_unicode_are_refused(self):
        for token in ['"\\x80"', '"\\xf0\\x9f"', '"\\xc0\\xaf"',
                      '"\\xed\\xa0\\x80"', '"\\ud800"', '"\\U00110000"',
                      '"\\x0g"', '"\\x0"', '"\\q"']:
            with self.subTest(token=token), self.assertRaises(ex.CodecError):
                self.decode(token)


if __name__ == '__main__':
    unittest.main()
