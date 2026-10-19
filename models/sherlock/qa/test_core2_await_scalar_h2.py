import unittest

import core2_await_boundary_wire as wire


class Core2AwaitScalarH2Test(unittest.TestCase):
    def test_core_bool_exact_and_uint_preserved(self):
        for payload in (True, False):
            result = wire._scalar({'type': 'Bool', 'value': payload})
            self.assertEqual(result, {'BoolValue': payload})
            self.assertIs(result['BoolValue'], payload)
        top = (1 << 128) - 1
        self.assertEqual(wire._scalar({'type': 'UInt', 'value': str(top)}), {'UIntValue': top})
        self.assertEqual(wire._scalar({'type': 'UInt', 'value': '0'}), {'UIntValue': 0})

    def test_core_bool_nonbool_payload_rejected(self):
        for payload in (0, 1, 1.0, 'true', 'True', None, [], {}):
            with self.subTest(payload=payload):
                with self.assertRaises(ValueError):
                    wire._scalar({'type': 'Bool', 'value': payload})


if __name__ == '__main__':
    unittest.main()
