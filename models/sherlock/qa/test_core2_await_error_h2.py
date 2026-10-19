import unittest
import core2_await_boundary_wire as wire


class ErrorH2Test(unittest.TestCase):
    def test_accepted_none_zero_max(self):
        self.assertEqual(wire._error({'code': 'c', 'message': 'm', 'offset': None, 'step': None}), {'code': 'c', 'message': 'm', 'offset': 'NoUInt', 'step': 'NoUInt'})
        self.assertEqual(wire._error({'code': 'c', 'message': 'm', 'offset': 0, 'step': 0}), {'code': 'c', 'message': 'm', 'offset': {'SomeUInt': 0}, 'step': {'SomeUInt': 0}})
        self.assertEqual(wire._error({'code': 'c', 'message': 'm', 'offset': 4294967295, 'step': 65535}), {'code': 'c', 'message': 'm', 'offset': {'SomeUInt': 4294967295}, 'step': {'SomeUInt': 65535}})

    def test_rejected_wrong_kind_and_range(self):
        for bad in (True, False, 1.0, '1', -1, 4294967296):
            with self.subTest(offset=bad):
                with self.assertRaises(ValueError):
                    wire._error({'code': 'c', 'message': 'm', 'offset': bad, 'step': None})
        for bad in (True, False, 1.0, '1', -1, 65536):
            with self.subTest(step=bad):
                with self.assertRaises(ValueError):
                    wire._error({'code': 'c', 'message': 'm', 'offset': None, 'step': bad})


if __name__ == '__main__':
    unittest.main()
