"""Unit tests for scripts/qa/check-target-pins.py (F15, SWF-007, SMN-001, SMN-004)."""
import copy
import io
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from test_profile_identity import ROOT, load

LEDGER = 'https://github.com/midnightntwrk/midnight-ledger'


def cargo_toml(rev, circuits='7.3.0', stdlib='2.4.0', proofs='0.8.3'):
    return f'''[workspace]
members = ["crates/*"]

[workspace.dependencies]
midnight-ledger = {{ git = "{LEDGER}", rev = "{rev}", package = "midnight-ledger-v9" }}
midnight-storage = {{ git = "{LEDGER}", rev = "{rev}" }}
midnight-proofs = "={proofs}"
midnight-circuits = {{ version = "={circuits}", default-features = false }}
midnight-zk-stdlib = {{ version = "={stdlib}", default-features = false }}

[patch.crates-io]
midnight-transient-crypto-old = {{ git = "{LEDGER}", rev = "a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1", package = "midnight-transient-crypto" }}
midnight-serialize = {{ git = "{LEDGER}", rev = "{rev}" }}
'''


def cargo_lock(ledger='1.0.1', zkir=('midnight-zkir', '3.2.0')):
    return f'''version = 4

[[package]]
name = "midnight-ledger-v9"
version = "{ledger}"

[[package]]
name = "{zkir[0]}"
version = "{zkir[1]}"

[[package]]
name = "midnight-zkir"
version = "2.2.0"
'''


def sdk_module(js='5.0.0', wallet='2.0.0', runtime='0.21.0', proof='9.0.1'):
    return f'''for (const [name, version] of Object.entries({{
  '@midnightntwrk/midnight-js-contracts': '{js}',
  '@midnightntwrk/wallet-sdk': '{wallet}',
  '@midnightntwrk/compact-runtime': '{runtime}',
}})) check(name, version);
assert.equal((await (await fetch(endpoints.proof + '/version')).text()).trim(), '{proof}');
'''


class TargetPinsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = load('check-target-pins')
        cls.pins = json.loads((ROOT / 'qa/sherlock/target-pins.json').read_text())

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.commit = self.pins['networks']['preview']['ledger']['commit']
        self.write('Cargo.toml', cargo_toml(self.commit))
        self.write('Cargo.lock', cargo_lock())
        self.write('crates/demo/Cargo.toml', '[package]\nname = "demo"\n\n[dependencies]\nserde = "1"\n')
        self.write('tools/sherlock-sdk/session.mjs', sdk_module())
        self.write('tools/sherlock-sdk/session.test.mjs', sdk_module(js='0.0.1'))
        self.write_pins(self.pins)
        # Source paths named in the pins file must exist under the fixture root.
        for name in ('openspec/changes/sherlock-vm-consensus-20261107/requirements.md',):
            self.write(name, '| SWF-009 |\n')

    def tearDown(self):
        self.tmp.cleanup()

    def write(self, rel, text):
        path = self.root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)

    def write_pins(self, pins):
        self.write('qa/sherlock/target-pins.json', json.dumps(pins))

    def run_check(self, **kw):
        out = io.StringIO()
        rc = self.mod.run(self.root, out=out, **kw)
        return rc, out.getvalue()

    def assertFails(self, needle, **kw):
        rc, out = self.run_check(**kw)
        self.assertEqual(rc, 1, out)
        self.assertIn(needle, out)
        return out

    def test_matching_workspace_passes(self):
        rc, out = self.run_check()
        self.assertEqual(rc, 0, out)
        self.assertIn("F15 PASS: workspace pins match the 'preview' row", out)
        self.assertIn('F15 workspace matches: undeployed, preview, mainnet', out)
        self.assertIn('F15 NOTE: no SDK root', out)

    def test_release_candidate_workspace_fails_against_target(self):
        self.write('Cargo.toml', cargo_toml('04c9c5d9bcebb8d4427d8589fb54d58a55599c14', '7.2.4', '2.3.5'))
        self.write('Cargo.lock', cargo_lock('1.0.0', ('midnight-zkir-v3', '3.0.0-rc.2')))
        self.write('tools/sherlock-sdk/session.mjs', sdk_module('5.0.0-beta.7', '2.0.0-beta.2', '0.19.0', '9.0.0-rc.5'))
        out = self.assertFails('F15 FAIL')
        for needle in ('ledger.commit', 'circuits.version', 'zkStdlib.version', 'ledger.crateVersion',
                       'zkir: workspace midnight-zkir-v3 3.0.0-rc.2', 'midnightJs.version', 'walletSdk.version',
                       'compact.runtime', 'proofServer'):
            self.assertIn(needle, out)
        self.assertIn('F15 workspace matches: stagenet', out)
        rc, out = self.run_check(network='stagenet')
        self.assertEqual(rc, 0, out)

    def test_single_mismatch_fails(self):
        self.write('Cargo.toml', cargo_toml(self.commit, circuits='7.3.1'))
        self.assertFails('circuits.version: workspace 7.3.1 != pins 7.3.0')

    def test_crate_manifest_ledger_rev_is_checked(self):
        self.write('crates/demo/Cargo.toml',
                   f'[package]\nname = "demo"\n\n[dependencies]\nx = {{ git = "{LEDGER}", rev = "deadbeef" }}\n')
        self.assertFails('ledger.commit: workspace deadbeef')

    def test_renamed_compat_patch_is_ignored(self):
        rc, out = self.run_check()
        self.assertNotIn('a01a1ea0', out)
        self.assertEqual(rc, 0, out)

    def test_proofs_caret(self):
        self.write('Cargo.toml', cargo_toml(self.commit, proofs='0.9.0'))
        self.assertFails('proofs.versionReq: workspace 0.9.0 does not satisfy pins ^0.8.2')
        self.assertTrue(self.mod._caret_ok('0.8.2', '^0.8.2'))
        self.assertFalse(self.mod._caret_ok('0.8.1', '^0.8.2'))
        self.assertTrue(self.mod._caret_ok('0.8.3', '=0.8.3'))

    def test_proof_server_assert(self):
        self.write('tools/sherlock-sdk/session.mjs', sdk_module(proof='9.1.0'))
        self.assertFails("proofServer: workspace asserts 9.1.0, pins accept ['9.0.0', '9.0.1']")

    def test_test_modules_are_ignored(self):
        rc, out = self.run_check()
        self.assertNotIn('0.0.1', out)

    def test_sdk_root_lockfile(self):
        sdk = self.root / 'sdk'
        self.write('sdk/package.json', json.dumps({'dependencies': {'@midnightntwrk/midnight-js-contracts': '^5.0.0'}}))
        self.write('sdk/package-lock.json', json.dumps({'packages': {
            'node_modules/@midnightntwrk/midnight-js-contracts': {'version': '5.0.0'},
            'node_modules/@midnightntwrk/ledger-v9': {'version': '1.0.0-rc.3'}}}))
        out = self.assertFails('ledgerJs.version: workspace @midnightntwrk/ledger-v9 1.0.0-rc.3 != pins 1.0.0', sdk_root=sdk)
        self.assertNotIn('NOTE: no SDK root', out)

    def test_structure_errors(self):
        cases = [
            (lambda p: p['networks'].pop('mainnet'), 'mainnet: row missing'),
            (lambda p: p['networks']['undeployed']['chain'].pop('genesisNote'),
             'undeployed: chain.genesisHash is null without a note'),
            (lambda p: p['networks']['preview']['chain'].pop('genesisHash'), 'preview: chain.genesisHash missing'),
            (lambda p: p['networks']['stagenet'].update(status='active'), 'stagenet: status must be retired'),
            (lambda p: p.update(workspaceTarget='stagenet'), "workspaceTarget 'stagenet' is a retired row"),
            (lambda p: p['networks']['preview']['circuits'].update(source='nowhere.md'),
             'preview: circuits.source missing or does not resolve'),
            (lambda p: p['networks']['preview'].pop('indexer'), 'preview: indexer missing'),
            (lambda p: p['networks']['preview'].update(walletSeed='x'), 'looks like a secret'),
            (lambda p: p['networks']['preview']['chain'].update(specVersion='2001100'),
             'preview: chain.specVersion must be an integer'),
        ]
        for mutate, needle in cases:
            with self.subTest(needle=needle):
                pins = copy.deepcopy(self.pins)
                mutate(pins)
                self.write_pins(pins)
                out = self.assertFails(needle)
                self.assertIn('structural error', out)

    def test_repository_pins_file_is_well_formed(self):
        self.assertEqual(self.mod.check_structure(self.pins, ROOT), [])

    def test_repository_source_paths_exist(self):
        for key, text in self.pins['sources'].items():
            first = text.split()[0].rstrip(',')
            # repos/ holds local, gitignored clones; those sources also cite a tracked wiki page.
            if first.startswith(('http', 'repos/')):
                continue
            with self.subTest(source=key):
                self.assertTrue((ROOT / first.split('#')[0]).exists(), first)


if __name__ == '__main__':
    unittest.main()
