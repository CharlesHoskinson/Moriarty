"""Source-only comparison-format cases prefrozen before implementation."""
import unittest
from program_snapshot_normalizer import canonical_json, loads_strict


class ComparisonTests(unittest.TestCase):
    def test_comparison_preserves_json_scalar_kinds_and_array_order(self):
        self.assertNotEqual(canonical_json({'x': True}), canonical_json({'x': 1}))
        self.assertNotEqual(canonical_json({'x': '1'}), canonical_json({'x': 1}))
        self.assertNotEqual(canonical_json([2, 0, 2]), canonical_json([0, 2, 2]))
        self.assertEqual(canonical_json({'b': [2, 0, 2], 'a': '007'}),
                         '{"a":"007","b":[2,0,2]}')
        self.assertEqual(canonical_json(loads_strict('{"b":2,"a":1}')),
                         canonical_json(loads_strict('{"a":1,"b":2}')))


if __name__ == '__main__':
    unittest.main()
