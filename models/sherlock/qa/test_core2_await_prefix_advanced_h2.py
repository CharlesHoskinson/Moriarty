import unittest

import core2_await_boundary_wire as wire


def _waiting(advanced):
    return {'Waiting': {'advanced': advanced, 'action': 'act', 'site': 7,
            'captures': [], 'logical': {'profile': 'p', 'cap': None, 'committed': '0'},
            'committed_boundary': {'state': {}, 'balances': [], 'liabilities': []},
            'residual': [], 'published_segment': None, 'attempted': '0'}}


class Core2AwaitPrefixAdvancedH2Tests(unittest.TestCase):
    def test_waiting_prefix_preserves_exact_bool(self):
        self.assertIs(wire.core2_await_prefix(_waiting(True))['WaitingPrefix']['advanced'], True)
        self.assertIs(wire.core2_await_prefix(_waiting(False))['WaitingPrefix']['advanced'], False)

    def test_waiting_prefix_rejects_non_bool_advanced(self):
        for bad in (1, 0, 'true', None):
            with self.assertRaises(ValueError):
                wire.core2_await_prefix(_waiting(bad))


if __name__ == '__main__':
    unittest.main()
