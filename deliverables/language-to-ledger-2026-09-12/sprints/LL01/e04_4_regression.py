"""Independent E04-4 actual immutable symlink regressions. No native invocation."""
import copy
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path('/home/charl/Moriarty')
HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('shim_e044', ROOT / '.moriarty-dev/k-macro05-trace106-diagnostic.py')
shim = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shim)
OBS = json.loads((HERE / 'host-ownership-observation-05.json').read_text())
HOST = {e['path']: e for e in OBS['entries']}
UID = shim.namespace_uid_for_host(Path('/proc/self/uid_map').read_text(), 0, 65534)
LINKS = [e['path'] for e in OBS['entries'] if e['isSymlink'] and e['hostUid'] == e['lstatUid'] == 0 and not e['writableByHostUser']]

class OwnershipRegression(unittest.TestCase):
    def test_real_unchanged_links_and_targets_are_accepted(self):
        self.assertTrue(LINKS)
        for path in LINKS:
            with self.subTest(path=path):
                shim.verify_ownership(path, HOST, UID, 0)

    def test_independent_link_and_target_drift_rejected(self):
        path = LINKS[0]
        for field in ['inode', 'device', 'mode', 'hostUid', 'lstatInode', 'lstatDevice', 'lstatMode', 'lstatUid']:
            with self.subTest(field=field):
                changed = copy.deepcopy(HOST)
                changed[path][field] += 1
                with self.assertRaises(shim.PreflightError):
                    shim.verify_ownership(path, changed, UID, 0)

    def test_missing_fields_do_not_skip_identity_checks(self):
        path = LINKS[0]
        for field in ['inode', 'device', 'mode', 'hostUid', 'lstatInode', 'lstatDevice', 'lstatMode', 'lstatUid']:
            with self.subTest(field=field):
                changed = copy.deepcopy(HOST)
                del changed[path][field]
                with self.assertRaisesRegex(shim.PreflightError, 'HOST_EVIDENCE_SCHEMA'):
                    shim.verify_ownership(path, changed, UID, 0)

    def test_missing_binding_and_wrong_namespace_rejected(self):
        path = LINKS[0]
        with self.assertRaisesRegex(shim.PreflightError, 'ARBITRARY_OVERFLOW'):
            shim.verify_ownership(path, {}, UID, 0)
        with self.assertRaisesRegex(shim.PreflightError, 'NAMESPACE_UID'):
            shim.verify_ownership(path, HOST, UID + 1, 0)

if __name__ == '__main__':
    unittest.main()
