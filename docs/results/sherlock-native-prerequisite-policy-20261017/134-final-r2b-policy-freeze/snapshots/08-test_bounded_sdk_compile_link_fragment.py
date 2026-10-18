'''Owner-contract unit tests for the R5 Git-symlink fragment: link_identity and link_target only.

Unexecuted source proposal; grants no authority and claims no run. Expected values come from
Git blob framing and literal owner-contract expectations, never from guard helpers or
candidate output. Only synthetic links inside an owned TemporaryDirectory are created; no real
SDK path, checkout, link text or target payload is touched. No FIFO, shell, subprocess,
network, namespace, cgroup or host probe is used.
'''
import builtins
import contextlib
import hashlib
import importlib.util
import os
import re
import stat
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

_GUARD_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'bounded_sdk_compile.py')


def _load_guard():
    # Reuse an already loaded guard. Import only; never __main__, main, run or worker.
    existing = sys.modules.get('bounded_sdk_compile')
    if existing is not None:
        return existing
    spec = importlib.util.spec_from_file_location('bounded_sdk_compile', _GUARD_FILE)
    module = importlib.util.module_from_spec(spec)
    sys.modules['bounded_sdk_compile'] = module
    try:
        spec.loader.exec_module(module)
    except BaseException:
        sys.modules.pop('bounded_sdk_compile', None)
        raise
    return module


guard = _load_guard()

NUL = bytes([0])
LF = bytes([10])
# Published Git object ids (git hash-object) anchoring the oracle itself.
GIT_BLOB_KATS = (
    (b'', 'e69de29bb2d1d6434b8b29ae775ad8c2e48c5391'),
    (b'hello world' + LF, '3b18e512dba79e4c8300dd08aeb37f8e728b8dad'),
    (b'test content' + LF, 'd670460b4b4aece5915caf5c68d12f560a9fe3e4'),
)


def git_blob_oid(data):
    # Independent oracle: Git object framing 'blob <decimal length>' NUL content, then SHA-1.
    return hashlib.sha1(b'blob ' + str(len(data)).encode('ascii') + NUL + data).hexdigest()


def link_text(n):
    # Synthetic relative link text of exactly n bytes; dangling from a/b, no real preimage.
    text = b'../../' + b'p' * (n - 10) + b'.bin'
    if len(text) != n:
        raise ValueError('fixture length')
    return text


def _forbid(stack, *targets):
    for owner, name in targets:
        stack.enter_context(mock.patch.object(
            owner, name, side_effect=AssertionError('forbidden filesystem call: ' + name)))


class LinkIdentityTests(unittest.TestCase):
    def setUp(self):
        tmp = tempfile.TemporaryDirectory(prefix='r5-link-identity-')
        self.addCleanup(tmp.cleanup)
        # Normalize only the owned temp root so no ancestor of a fixture is a symlink.
        self.checkout = Path(os.path.realpath(tmp.name)) / 'checkout'
        (self.checkout / 'a' / 'b').mkdir(parents=True)

    def make_link(self, rel, text):
        path = self.checkout / rel
        os.symlink(text, os.fsencode(path))
        st = os.lstat(path)
        self.assertTrue(stat.S_ISLNK(st.st_mode))
        self.assertEqual(st.st_size, len(text))
        return str(path)

    def watch_readlink(self):
        return mock.patch.object(guard.os, 'readlink', wraps=os.readlink)

    def test_identity_is_git_blob_of_link_text_33_to_55(self):
        for data, oid in GIT_BLOB_KATS:
            self.assertEqual(git_blob_oid(data), oid)
        for n in (33, 34, 44, 54, 55):
            with self.subTest(size=n):
                text = link_text(n)
                path = self.make_link('a/b/link-%d' % n, text)
                events = []

                def stable(p, _real=guard.stable_stat, _events=events):
                    _events.append('lstat')
                    return _real(p)

                def readlink(p, _real=os.readlink, _events=events):
                    _events.append('readlink')
                    return _real(p)

                with mock.patch.object(guard, 'stable_stat', stable), \
                        mock.patch.object(guard.os, 'readlink', readlink):
                    got_text, got_oid = guard.link_identity(path)
                self.assertIs(type(got_text), bytes)
                self.assertEqual(got_text, text)
                self.assertEqual(got_oid, git_blob_oid(text))
                self.assertTrue(re.fullmatch('[0-9a-f]{40}', got_oid))
                self.assertEqual(events.count('readlink'), 1)
                at = events.index('readlink')
                self.assertIn('lstat', events[:at])
                self.assertIn('lstat', events[at + 1:])

    def test_size_32_and_56_refused_before_readlink(self):
        for n in (32, 56, 120):
            with self.subTest(size=n):
                path = self.make_link('a/b/size-%d' % n, link_text(n))
                with self.watch_readlink() as rl, self.assertRaises(guard.Refusal):
                    guard.link_identity(path)
                rl.assert_not_called()

    def test_regular_file_and_directory_refused_without_coercion(self):
        regular = self.checkout / 'a' / 'b' / 'regular-holding-link-text'
        with open(regular, 'wb') as fh:
            fh.write(link_text(40))
        directory = self.checkout / 'a' / 'b' / 'directory-entry'
        directory.mkdir()
        for path in (regular, directory):
            with self.subTest(path=path.name), contextlib.ExitStack() as stack:
                rl = stack.enter_context(self.watch_readlink())
                _forbid(stack, (builtins, 'open'), (guard.os, 'open'))
                with self.assertRaises(guard.Refusal):
                    guard.link_identity(str(path))
                rl.assert_not_called()

    def test_symlink_ancestor_refused_before_readlink(self):
        (self.checkout / 'real').mkdir()
        self.make_link('real/link', link_text(40))
        os.symlink(b'real', os.fsencode(self.checkout / 'alias'))
        with self.watch_readlink() as rl, self.assertRaises(guard.Refusal):
            guard.link_identity(str(self.checkout / 'alias' / 'link'))
        rl.assert_not_called()

    def test_target_never_opened_stated_or_resolved(self):
        target_dir = self.checkout / 'a' / 'payload-directory-x'
        target_dir.mkdir()
        target = target_dir / 'target-payload.bin'
        payload = b'synthetic target payload that must never be read'
        with open(target, 'wb') as fh:
            fh.write(payload)
        target_b = os.fsencode(target)
        for name, text in (('to-existing', b'../payload-directory-x/target-payload.bin'),
                           ('to-absent', b'../payload-directory-x/absent-target.bin')):
            with self.subTest(link=name):
                self.assertTrue(33 <= len(text) <= 55)
                link = self.make_link('a/b/' + name, text)
                link_b = os.fsencode(link)
                touched = []
                real_stat, real_lstat, real_readlink = os.stat, os.lstat, os.readlink

                def t_stat(p, *args, **kwargs):
                    touched.append(('stat', p, kwargs.get('follow_symlinks', True)))
                    return real_stat(p, *args, **kwargs)

                def t_lstat(p, *args, **kwargs):
                    touched.append(('lstat', p, False))
                    return real_lstat(p, *args, **kwargs)

                def t_readlink(p, *args, **kwargs):
                    touched.append(('readlink', p, False))
                    return real_readlink(p, *args, **kwargs)

                with contextlib.ExitStack() as stack:
                    stack.enter_context(mock.patch.object(guard.os, 'stat', t_stat))
                    stack.enter_context(mock.patch.object(guard.os, 'lstat', t_lstat))
                    stack.enter_context(mock.patch.object(guard.os, 'readlink', t_readlink))
                    _forbid(stack, (builtins, 'open'), (guard.os, 'open'), (guard.os, 'scandir'),
                            (guard.os, 'listdir'), (guard.os.path, 'realpath'))
                    got = guard.link_identity(link)
                self.assertEqual(got, (text, git_blob_oid(text)))
                self.assertNotEqual(got[1], git_blob_oid(payload))
                self.assertIn('readlink', [kind for kind, _, _ in touched])
                for kind, p, follow in touched:
                    if isinstance(p, int):
                        continue
                    p_b = os.fsencode(p)
                    self.assertNotEqual(p_b, target_b, kind)
                    if kind == 'readlink':
                        self.assertEqual(p_b, link_b)
                    if p_b == link_b:
                        self.assertFalse(follow, kind)

    def test_lstat_change_or_text_length_drift_refused(self):
        # Mocked race: the second lstat differs, or readlink returns a different length.
        for key in ('device', 'inode', 'size', 'mtime_ns', 'mode', 'links'):
            with self.subTest(changed=key):
                path = self.make_link('a/b/drift-' + key, link_text(44))
                calls = []

                def drifting(p, _key=key, _calls=calls, _real=guard.stable_stat):
                    s = dict(_real(p))
                    _calls.append(p)
                    if len(_calls) > 1:
                        s[_key] += 1
                    return s

                with mock.patch.object(guard, 'stable_stat', drifting), \
                        self.assertRaises(guard.Refusal):
                    guard.link_identity(path)
        for replaced in (link_text(44)[:-1], link_text(44) + b'q'):
            with self.subTest(readlink_len=len(replaced)):
                path = self.make_link('a/b/swap-%d' % len(replaced), link_text(44))
                with mock.patch.object(guard.os, 'readlink', return_value=replaced), \
                        self.assertRaises(guard.Refusal):
                    guard.link_identity(path)


class LinkTargetLexicalTests(unittest.TestCase):
    def assert_refused(self, cases):
        for link, text, rows in cases:
            with self.subTest(link=link, text=text):
                with self.assertRaises(guard.Refusal):
                    guard.link_target(link, text, frozenset(rows))

    def test_valid_leading_parent_targets_within_checkout(self):
        cases = (
            ('a/b/c/link.bin', b'../../x/real.bin', 'a/x/real.bin'),
            ('a/b/link.bin', b'../../top.bin', 'top.bin'),
            ('a/link.bin', b'sibling.bin', 'a/sibling.bin'),
            ('a/link.bin', b'x/y/deep.bin', 'a/x/y/deep.bin'),
            ('link.bin', b'x/real.bin', 'x/real.bin'),
            ('w/p/m/d/spend.zkir', b'../../../../zk/spend.zkir', 'zk/spend.zkir'),
            ('a/b/link.bin', b'../x/!a-b_c~.bin', 'a/x/!a-b_c~.bin'),
            ('a/link.bin', b'../.gitignore', '.gitignore'),
        )
        for link, text, expected in cases:
            with self.subTest(link=link, text=text):
                got = guard.link_target(link, text, frozenset({expected, 'decoy/other.bin'}))
                self.assertIs(type(got), str)
                self.assertEqual(got, expected)

    def test_absolute_empty_dot_and_non_normalized_text_refused(self):
        self.assert_refused((
            ('a/link', b'/x/real.bin', {'x/real.bin', '/x/real.bin'}),
            ('a/link', b'//x/real.bin', {'x/real.bin'}),
            ('a/link', b'', {'a', 'a/link'}),
            ('a/link', b'.', {'a'}),
            ('a/link', b'./real.bin', {'a/real.bin'}),
            ('a/b/link', b'../x/./real.bin', {'a/x/real.bin'}),
            ('a/b/link', b'../x//real.bin', {'a/x/real.bin'}),
            ('a/b/link', b'../x/real.bin/', {'a/x/real.bin'}),
        ))

    def test_escape_and_ascend_after_descent_refused(self):
        self.assert_refused((
            ('link', b'../real.bin', {'real.bin'}),
            ('a/link', b'../../real.bin', {'real.bin'}),
            ('a/b/link', b'../../../a/real.bin', {'a/real.bin'}),
            ('a/b/link', b'../x/../real.bin', {'a/real.bin', 'a/x/real.bin'}),
            ('a/link', b'x/../real.bin', {'a/real.bin'}),
            ('a/link', b'x/../../a/real.bin', {'a/real.bin'}),
            ('a/b/link', b'x/y/../../../../etc/passwd', {'etc/passwd'}),
        ))

    def test_dot_git_component_refused(self):
        self.assert_refused((
            ('a/link', b'../.git/config', {'.git/config'}),
            ('a/link', b'.git/HEAD', {'a/.git/HEAD'}),
            ('a/b/link', b'../../x/.git/objects/pack.bin', {'x/.git/objects/pack.bin'}),
            ('a/link', b'../x/.git', {'x/.git'}),
            ('.git/hooks/link', b'../config', {'.git/config'}),
        ))

    def test_nonprintable_or_non_ascii_text_refused(self):
        cases = []
        for byte in (0x00, 0x09, 0x0a, 0x1f, 0x20, 0x7f, 0x80, 0xc3, 0xff):
            cases.append(('a/link', b'../x/a' + bytes([byte]) + b'b.bin', {'x/a' + chr(byte) + 'b.bin'}))
        self.assert_refused(cases)

    def test_undeclared_chain_directory_and_root_targets_refused(self):
        self.assert_refused((
            ('a/b/link', b'../x/missing.bin', {'a/x/real.bin'}),
            ('a/b/link', b'../X/real.bin', {'a/x/real.bin'}),
            # a/l2 is itself a declared link, so it is never in the regular rows: no chains.
            ('a/l1', b'l2', {'a/real.bin'}),
            ('a/link', b'link', {'a/real.bin'}),
            # Directories are decided lexically: x holds regular rows but is not itself one.
            ('a/b/link', b'../../x', {'x/real.bin'}),
            ('a/b/link', b'../x', {'a/x/real.bin'}),
            ('a/link', b'..', {'a/real.bin'}),
            ('a/b/link', b'../..', {'x/real.bin'}),
            # Rows of another checkout are not supplied here, so membership is empty.
            ('a/link', b'../real.bin', set()),
        ))

    def test_target_resolution_is_lexical_only(self):
        # None of these relative names exist; any filesystem consultation is a failure.
        with contextlib.ExitStack() as stack:
            _forbid(stack, (builtins, 'open'), (guard.os, 'open'), (guard.os, 'stat'),
                    (guard.os, 'lstat'), (guard.os, 'readlink'), (guard.os, 'scandir'),
                    (guard.os, 'listdir'), (guard.os, 'access'), (guard.os.path, 'realpath'))
            got = guard.link_target('a/b/link', b'../x/real.bin', frozenset({'a/x/real.bin'}))
            with self.assertRaises(guard.Refusal):
                guard.link_target('a/b/link', b'../../../escape.bin', frozenset({'escape.bin'}))
            with self.assertRaises(guard.Refusal):
                guard.link_target('a/b/link', b'../x/missing.bin', frozenset({'a/x/real.bin'}))
        self.assertEqual(got, 'a/x/real.bin')
