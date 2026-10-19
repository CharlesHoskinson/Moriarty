import unittest

import core2_await_boundary_wire as wire


class BoundaryObservationEqualityH5Test(unittest.TestCase):
    def test_python_equal_type_confusion_has_distinct_bytes(self):
        for left, right in (({'advanced': True}, {'advanced': 1}),
                            ({'advanced': False}, {'advanced': 0}),
                            ({'step': 1}, {'step': 1.0}),
                            ([{'value': {'BoolValue': True}}], [{'value': {'BoolValue': 1}}])):
            self.assertEqual(left, right)
            self.assertFalse(wire.boundary_observations_equal(left, right))
        self.assertTrue(wire.boundary_observations_equal({'advanced': True}, {'advanced': True}))

    def test_complete_object_array_order_and_fields_discriminate(self):
        base = {'schema': wire.SCHEMA, 'profile': wire.PROFILE,
                'result': {'Complete': {'residual': [{'name': 'a'}, {'name': 'b'}], 'attempted': 2}},
                'awaitPrefix': 'CompletePrefix'}
        reordered = {key: base[key] for key in reversed(list(base))}
        self.assertTrue(wire.boundary_observations_equal(base, reordered))
        swapped = {**base, 'result': {'Complete': {'residual': [{'name': 'b'}, {'name': 'a'}], 'attempted': 2}}}
        dropped = {key: base[key] for key in base if key != 'awaitPrefix'}
        extra = {**base, 'extra': None}
        nulled = {**base, 'awaitPrefix': None}
        for other in (swapped, dropped, extra, nulled):
            self.assertFalse(wire.boundary_observations_equal(base, other))
            self.assertFalse(wire.boundary_observations_equal(other, base))


if __name__ == '__main__':
    unittest.main()
