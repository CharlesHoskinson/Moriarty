import unittest

import core2_await_boundary_wire as wire


def _waiting(advanced):
    continuation = {'format': 'f', 'program_hash': 'ph', 'plan_hash': 'pl',
                    'action': 'a', 'site': 's', 'captures': [],
                    'logical': {'profile': 'p', 'cap': None, 'committed': '0'},
                    'boundary': {'state': {}, 'balances': [], 'liabilities': []}}
    return {'Waiting': {'advanced': advanced, 'segment': None,
                        'continuation': continuation, 'attempted': '0', 'residual': []}}


class Core2AwaitResultAdvancedH2Tests(unittest.TestCase):
    def test_waiting_preserves_exact_bool(self):
        for value in (True, False):
            with self.subTest(value=value):
                result = wire.core2_result(_waiting(value))
                self.assertIs(result['Waiting']['advanced'], value)
                self.assertIs(type(result['Waiting']['advanced']), bool)

    def test_waiting_rejects_non_bool_advanced(self):
        for value in (0, 1, 0.0, 'true', None):
            with self.subTest(value=value):
                with self.assertRaises(ValueError):
                    wire.core2_result(_waiting(value))


if __name__ == '__main__':
    unittest.main()
