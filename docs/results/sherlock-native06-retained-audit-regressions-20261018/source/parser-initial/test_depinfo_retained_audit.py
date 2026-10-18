import os
from pathlib import Path
import tempfile
import unittest

import bounded_sdk_compile as runner


def _write_file(path: Path, content: bytes = b'data\n', mode: int = 0o644) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(content)
    os.chmod(path, mode)
    return path


class DepinfoNormalizationTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.base = Path(self._tmp.name).resolve()
        self.worktree = self.base / 'worktree'
        self.source_home = self.base / 'source-home'
        self.target = self.base / 'target'
        self.worktree.mkdir(parents=True)
        self.source_home.mkdir(parents=True)
        self.target.mkdir(parents=True)

        # Real directory hierarchy matching the native06 mori-backend & midnight-ledger layout
        (self.worktree / 'crates/mori-backend/src/sherlock_machine').mkdir(parents=True)
        self.lib_rs = _write_file(self.worktree / 'crates/mori-backend/src/lib.rs', b'// lib\n')
        self.expected_json = _write_file(
            self.worktree / 'models/sherlock/machine-circuit/expected.json',
            b'{"circuit":"ok"}\n',
        )
        self.out_rlib = _write_file(
            self.target / 'debug/deps/libmori_backend-0123456789abcdef.rlib',
            b'rlib\n',
        )

        self.ledger_root = (
            self.source_home
            / 'git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d'
        )
        (self.ledger_root / 'zswap/src').mkdir(parents=True)
        self.zswap_lib = _write_file(self.ledger_root / 'zswap/src/lib.rs', b'// zswap\n')
        self.spend_verifier = _write_file(
            self.ledger_root / 'zswap/static/spend.verifier.sha256',
            b'0123456789abcdef\n',
        )

    def tearDown(self):
        self._tmp.cleanup()

    def test_parse_depinfo_resolves_relative_and_crate_contained_parent_segments(self):
        rel_dotdot = (
            'crates/mori-backend/src/sherlock_machine/'
            '../../../../models/sherlock/machine-circuit/expected.json'
        )
        abs_dotdot = f'{self.ledger_root}/zswap/src/../static/spend.verifier.sha256'

        self.assertEqual(
            runner.depinfo_path(rel_dotdot, self.worktree, roots=(str(self.ledger_root),)),
            str(self.expected_json),
        )
        self.assertEqual(
            runner.depinfo_path(abs_dotdot, self.worktree, roots=(str(self.ledger_root),)),
            str(self.spend_verifier),
        )

        dep_text = (
            f'{self.out_rlib}: {self.lib_rs} \\\n'
            f'  {rel_dotdot} \\\n'
            f'  {abs_dotdot}\n'
            f'\n'
            f'# env-dep:RUSTC_BOOTSTRAP=1\n'
            f'{self.lib_rs}:\n'
            f'{rel_dotdot}:\n'
            f'{abs_dotdot}:\n'
        )

        records = runner.parse_depinfo(
            dep_text,
            self.worktree,
            roots=(str(self.ledger_root),),
        )
        self.assertEqual(
            records,
            [
                (
                    {str(self.out_rlib)},
                    {
                        str(self.lib_rs),
                        str(self.expected_json),
                        str(self.spend_verifier),
                    },
                ),
                ({str(self.lib_rs)}, set()),
                ({str(self.expected_json)}, set()),
                ({str(self.spend_verifier)}, set()),
            ],
        )

    def test_parse_depinfo_preserves_make_continuations_and_escaped_filename_tokens(self):
        special_file = _write_file(
            self.worktree / 'models/sherlock/machine-circuit/spec #1:v2\\final.json',
            b'{"v":2}\n',
        )
        escaped_rel_dotdot = (
            'crates/mori-backend/src/sherlock_machine/'
            '../../../../models/sherlock/machine-circuit/spec\\ \\#1\\:v2\\\\final.json'
        )
        dep_text = (
            f'{self.out_rlib}: \\\n'
            f'  {escaped_rel_dotdot}\n'
            f'{escaped_rel_dotdot}:\n'
        )

        self.assertEqual(
            runner.make_words(escaped_rel_dotdot),
            [
                'crates/mori-backend/src/sherlock_machine/'
                '../../../../models/sherlock/machine-circuit/spec #1:v2\\final.json'
            ],
        )
        records = runner.parse_depinfo(dep_text, self.worktree)
        self.assertEqual(
            records,
            [
                ({str(self.out_rlib)}, {str(special_file)}),
                ({str(special_file)}, set()),
            ],
        )

    def test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals(self):
        outside_file = _write_file(self.base / 'escape.json', b'outside\n')
        self.assertTrue(outside_file.exists())

        symlink_parent = self.worktree / 'symlink_dir'
        os.symlink(self.worktree / 'crates', symlink_parent)

        symlink_target = self.worktree / 'models/sherlock/machine-circuit/symlink_expected.json'
        os.symlink(self.expected_json, symlink_target)

        pkg_a = self.source_home / 'registry/src/index/pkg-a-1.0.0'
        pkg_b = self.source_home / 'registry/src/index/pkg-b-1.0.0'
        (pkg_a / 'src').mkdir(parents=True)
        pkg_b_file = _write_file(pkg_b / 'src/file.rs', b'// pkg-b\n')
        self.assertTrue(pkg_b_file.exists())

        refusal_cases = [
            (
                'relative immediate escape above worktree',
                f'{self.out_rlib}: ../escape.json\n',
                'non-normalized dep-info path',
            ),
            (
                'relative escape above worktree after descent',
                f'{self.out_rlib}: crates/../../escape.json\n',
                'non-normalized dep-info path',
            ),
            (
                'relative traversal collapsing to worktree root itself',
                f'{self.out_rlib}: crates/..\n',
                'non-normalized dep-info path',
            ),
            (
                'dot segment in dep-info path',
                f'{self.out_rlib}: ./crates/mori-backend/src/lib.rs\n',
                'non-normalized dep-info path',
            ),
            (
                'empty segment double-slash in dep-info path',
                f'{self.out_rlib}: crates//mori-backend/src/lib.rs\n',
                'non-normalized dep-info path',
            ),
            (
                'nonexistent intermediate directory before ..',
                (
                    f'{self.out_rlib}: '
                    'nonexistent_dir/../models/sherlock/machine-circuit/expected.json\n'
                ),
                f'dep-info parent directory identity: {self.worktree / "nonexistent_dir"}',
            ),
            (
                'symlink intermediate directory before ..',
                (
                    f'{self.out_rlib}: '
                    'symlink_dir/../models/sherlock/machine-circuit/expected.json\n'
                ),
                f'dep-info parent directory identity: {symlink_parent}',
            ),
            (
                'regular file traversed as parent directory before ..',
                (
                    f'{self.out_rlib}: '
                    'crates/mori-backend/src/lib.rs/'
                    '../../../../../models/sherlock/machine-circuit/expected.json\n'
                ),
                f'dep-info parent directory identity: {self.lib_rs}',
            ),
            (
                'cross-crate source_home traversal escaping admitted crate root',
                f'{self.out_rlib}: {pkg_a}/src/../../pkg-b-1.0.0/src/file.rs\n',
                'non-normalized dep-info path',
            ),
            (
                'absolute .. path outside admitted roots',
                f'{self.out_rlib}: {self.base}/unlisted/sub/../escape.json\n',
                'non-normalized dep-info path',
            ),
            (
                'missing resolved target after valid .. traversal',
                (
                    f'{self.out_rlib}: '
                    'crates/mori-backend/src/sherlock_machine/'
                    '../../../../models/sherlock/machine-circuit/missing.json\n'
                ),
                (
                    'dep-info target missing: '
                    f'{self.worktree / "models/sherlock/machine-circuit/missing.json"}'
                ),
            ),
            (
                'symlink resolved target after valid .. traversal',
                (
                    f'{self.out_rlib}: '
                    'crates/mori-backend/src/sherlock_machine/'
                    '../../../../models/sherlock/machine-circuit/symlink_expected.json\n'
                ),
                f'symlink path: {symlink_target}',
            ),
            (
                'directory resolved target after valid .. traversal',
                (
                    f'{self.out_rlib}: '
                    'crates/mori-backend/src/sherlock_machine/'
                    '../../../../models/sherlock/machine-circuit\n'
                ),
                f'not regular: {self.worktree / "models/sherlock/machine-circuit"}',
            ),
            (
                'rule line missing colon delimiter',
                f'{self.out_rlib} {self.lib_rs}\n',
                'unrecognized dep-info rule',
            ),
            (
                'trailing backslash escape at end of rule',
                f'{self.out_rlib}: {self.lib_rs} \\',
                'trailing dep-info escape',
            ),
        ]

        roots = (str(self.ledger_root), str(pkg_a), str(pkg_b))
        for label, dep_text, expected_msg in refusal_cases:
            with self.subTest(case=label):
                with self.assertRaises(runner.Refusal) as ctx:
                    runner.parse_depinfo(dep_text, self.worktree, roots=roots)
                self.assertIn(expected_msg, str(ctx.exception))

        for ch in ('$', ';', '|'):
            with self.subTest(unsupported_char=ch):
                with self.assertRaises(runner.Unsupported) as uctx:
                    runner.parse_depinfo(
                        f'{self.out_rlib}: {self.lib_rs}{ch}extra\n',
                        self.worktree,
                        roots=roots,
                    )
                self.assertIn('unsupported dep-info syntax', str(uctx.exception))


if __name__ == '__main__':
    unittest.main()
