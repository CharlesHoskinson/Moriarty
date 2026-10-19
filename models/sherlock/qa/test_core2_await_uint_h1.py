import unittest

import core2_await_boundary_wire as wire

MAX = (1 << 128) - 1


class CoreUIntH1Test(unittest.TestCase):
    def test_canonical_strings_normalize_exactly(self):
        for text, expected in (('0', 0), ('1', 1), ('10', 10), ('18446744073709551616', 1 << 64),
                               (str(MAX - 1), MAX - 1), ('340282366920938463463374607431768211455', MAX)):
            with self.subTest(text=text):
                result = wire._uint(text)
                self.assertIs(type(result), int)
                self.assertEqual(result, expected)

    def test_noncanonical_nonstring_and_out_of_range_rejected(self):
        for value in ('', '00', '01', '-0', '-1', '+1', ' 1', '1 ', '1\n', '1_000', '1.0', '1e3',
                      '0x10', '\u0661', '\uff11', '\u00b2', '340282366920938463463374607431768211456',
                      str(1 << 129), '9' * 5000, 0, 1, True, False, 1.0, None, b'1', ['1']):
            with self.subTest(value=value):
                with self.assertRaises(ValueError):
                    wire._uint(value)


if __name__ == '__main__':
    unittest.main()