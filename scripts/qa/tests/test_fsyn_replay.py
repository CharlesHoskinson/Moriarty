"""The independent replay snapshot must preserve pins and reject path escapes."""
import hashlib
import os
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'models/sherlock/rollup-synth'))
from common import Error, canonical
from replay import prepare_oracle


class ReplaySourceTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.source = self.root / 'source'
        self.source.mkdir()
        (self.source / 'main.rs').write_bytes(b'pinned public Rust')
        self.row = {'source': 'main.rs', 'path': 'generator/src/main.rs',
                    'bytes': 18, 'sha256': hashlib.sha256(b'pinned public Rust').hexdigest()}
        self.pin = self.root / 'pins.json'
        self.output = self.root / 'prepared'
        self.write_pin()

    def write_pin(self):
        rows = [self.row]
        self.pin.write_bytes(canonical({'schema': 'sherlock-b2-oracle-sources/1',
            'files': rows, 'content_digest': hashlib.sha256(canonical(rows)).hexdigest()}))

    def test_pinned_bytes_are_copied_and_rechecked(self):
        prepare_oracle(self.pin, self.source, self.output)
        self.assertEqual((self.output / self.row['path']).read_bytes(), b'pinned public Rust')
        prepare_oracle(self.pin, self.source, self.output)
        (self.output / self.row['path']).write_bytes(b'changed cache')
        with self.assertRaisesRegex(Error, 'prepared oracle differs'):
            prepare_oracle(self.pin, self.source, self.output)

    def test_stale_source_leaves_no_output(self):
        (self.source / 'main.rs').write_bytes(b'newer source')
        with self.assertRaisesRegex(Error, 'stale oracle source'):
            prepare_oracle(self.pin, self.source, self.output)
        self.assertFalse(self.output.exists())

    def test_source_and_destination_escapes_are_rejected(self):
        for key in ('source', 'path'):
            with self.subTest(key=key):
                before = self.row[key]
                self.row[key] = '../escape'
                self.write_pin()
                with self.assertRaises(Error):
                    prepare_oracle(self.pin, self.source, self.output)
                self.row[key] = before

    def test_pin_digest_tamper_is_rejected(self):
        self.pin.write_bytes(self.pin.read_bytes().replace(self.row['sha256'].encode(), b'0' * 64))
        with self.assertRaisesRegex(Error, 'oracle cut digest'):
            prepare_oracle(self.pin, self.source, self.output)

    def test_unlisted_cached_source_is_rejected(self):
        prepare_oracle(self.pin, self.source, self.output)
        (self.output / 'injected.rs').write_bytes(b'unlisted')
        with self.assertRaisesRegex(Error, 'prepared oracle differs'):
            prepare_oracle(self.pin, self.source, self.output)

    def test_nonregular_cached_member_is_rejected(self):
        prepare_oracle(self.pin, self.source, self.output)
        os.mkfifo(self.output / 'injected.fifo')
        with self.assertRaisesRegex(Error, 'prepared oracle'):
            prepare_oracle(self.pin, self.source, self.output)


if __name__ == '__main__':
    unittest.main()
