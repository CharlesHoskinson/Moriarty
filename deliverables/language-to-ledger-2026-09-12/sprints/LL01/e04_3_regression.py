"""Independent E04-3 refusal before any Nix query. No native K execution."""
import copy
import importlib.util
import json
from pathlib import Path
from unittest.mock import patch
import unittest

ROOT = Path('/home/charl/Moriarty')
HERE = ROOT / '.moriarty-dev'
spec = importlib.util.spec_from_file_location('shim_e043', HERE / 'k-macro05-trace106-diagnostic.py')
shim = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shim)
PINS = shim.load_pins()
OWN = json.loads((HERE / 'k-macro05-trace106-ownership.json').read_text())
HOST = shim.load_host_evidence(OWN)

class RequisiteRegression(unittest.TestCase):
    def check_missing(self, missing):
        host = copy.deepcopy(HOST)
        self.assertTrue(missing in host, missing)
        del host[missing]
        queries = []
        def stopped_query(*args, **kwargs):
            queries.append(args)
            raise AssertionError('query reached with missing protected observation: ' + missing)
        with patch.object(shim, 'load_host_evidence', return_value=host):
            with self.assertRaises(shim.PreflightError):
                shim.verify_requisites(PINS, HERE, home='/home/charl', nix_run=stopped_query)
        self.assertEqual(queries, [])

    def test_unbound_selected_env_rejected_before_query(self):
        self.check_missing('/usr/bin/env')

    def test_unbound_nix_database_rejected_before_query(self):
        self.check_missing('/nix/var/nix/db/db.sqlite')

    def test_unbound_nix_database_ancestor_rejected_before_query(self):
        self.check_missing('/nix/var/nix')

    def test_unbound_query_executable_rejected_before_query(self):
        self.check_missing(next(i['path'] for i in PINS['selectedExecutables'] if i['role'] == 'nix-query'))

if __name__ == '__main__':
    unittest.main()
