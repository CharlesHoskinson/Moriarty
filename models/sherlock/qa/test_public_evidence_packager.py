#!/usr/bin/env python3
"""Hostile tests for the October 16 public evidence packager (findings S1, S2 and F1).

The packager source is compiled from its frozen delivery folder without writing
bytecode there, and main() never runs on real evidence. Every case packages a
tiny runtime written inside a temporary directory; output goes to that
directory, never to the frozen archive. Each integrity case runs twice: once as
ordinary source and once compiled with optimize=1, which strips assert statements
exactly as python -O does. One case also runs a real python -O child.
Nothing here starts a Sherlock tool, model, SDK, compiler or network call.
"""
import ast
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import types
import unittest

REPO = Path(__file__).resolve().parents[3]
DELIVERY = REPO / 'docs/audits/2026-10-16/sherlock-parallel-delivery'
PACKAGER = DELIVERY / 'package_public_evidence.py'
SELECTION_FORMAT = 'sherlock-parallel-public-selection/1'
SOURCES = {'src/core.rs': b'fn core() {}\n', 'models/qa/check.py': b"print('check')\n"}
OUTER_ONLY = {'models/qa/future_source.py': b'# excluded future source\n'}
TINY = {'programs': 1, 'runs': 1, 'records': 2, 'charged': 1, 'publications': 1,
        'itf_captures': 1, 'raw_states': 2, 'terminal_stutters': 1}
OPTIMIZE_CHILD = '\n'.join([
    'import sys',
    'from pathlib import Path',
    'path = sys.argv[1]',
    "namespace = {'__name__': 'packager_under_optimize', '__file__': path}",
    "exec(compile(open(path, 'rb').read(), path, 'exec'), namespace)",
    "print('optimize', sys.flags.optimize, file=sys.stderr)",
    "namespace['main'](sys.argv[2:5], out=Path(sys.argv[5]))",
])


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load_packager(optimize):
    """Compile the packager source; optimize=1 drops asserts as python -O does."""
    module = types.ModuleType(f'sherlock_public_packager_optimize_{optimize}')
    module.__file__ = str(PACKAGER)
    saved = sys.argv
    sys.argv = [str(PACKAGER)]
    try:
        exec(compile(PACKAGER.read_bytes(), str(PACKAGER), 'exec', optimize=optimize), module.__dict__)
    finally:
        sys.argv = saved
    return module


def encode(value):
    return value if isinstance(value, bytes) else (json.dumps(value, indent=1, sort_keys=True) + '\n').encode()


def build_fixture(root):
    """Write a tiny public runtime, empty checkout and explicit selection under root."""
    root = Path(os.path.realpath(root))
    checkout, runtime = root / 'checkout', root / 'runtime'
    out = checkout / 'package'
    out.mkdir(parents=True)
    files, selection = {}, []

    def put(name, value, select=True, archive_name=None):
        data = files[name] = encode(value)
        if select:
            selection.append({'path': archive_name or name, 'from': name, 'sha256': sha(data),
                              'bytes': len(data), 'origin': f'recorded-public-attempt:{name}'})
        return data

    source_map = {rel: sha(data) for rel, data in SOURCES.items()}
    outer_map = {**source_map, **{rel: sha(data) for rel, data in OUTER_ONLY.items()}}
    for rel, data in SOURCES.items():
        put(f'ordinary-01/source/{rel}', data)
    put('ordinary-01/result.json', {'before': source_map, 'exit': 0, 'elapsed_seconds': 0.5,
                                    'source_unchanged': True})
    put('ordinary-01/stderr', b'Ran 2 tests\n\nOK\n')
    observations = {'programs': [{'name': 'p0'}], 'runs': [{'records': [{'charge': 1}, {'charge': 0}]}]}
    summary = {'cross_run': {'prefix_runs': {'run_count': 1, 'record_count': 2,
                                             'charged_sum': 1, 'publication_count': 1}}}
    for group, layer in (('host-01', 'host'), ('image-01', 'image')):
        log = put(f'{group}/logs/run.stdout', f'{layer} ok\n'.encode())
        put(f'{group}/{layer}-observations.json', observations)
        put(f'{group}/receipt.json', {
            'terminal_state': 'succeeded',
            'sources': [{'path': rel, 'sha256_before': digest} for rel, digest in source_map.items()],
            'steps': [{'name': 'run', 'state': 'succeeded',
                       'child': {'exit_code': 0, 'duration_seconds': 0.1, 'timeout_seconds': 30,
                                 'timed_out': False}}],
            'logs': [{'path': str(runtime / group / 'logs/run.stdout'), 'sha256': sha(log), 'bytes': len(log)}],
            'tool': {'path': '/usr/bin/true', 'sha256': '0' * 64}})
        put(f'{group}-outer/result.json', {'source_before': outer_map, 'exit': 0, 'elapsed_seconds': 0.2})
        # The outer snapshot exists on disk, as in finding P1, but is not selected.
        for rel, data in OUTER_ONLY.items():
            put(f'{group}-outer/source/{rel}', data, select=False)
    itf = put('quint-01/captures/t0.itf.json', {'states': [{}, {}]})
    put('quint-01/capture-hashes.json', {'t0.itf.json': sha(itf)})
    compare = put('quint-01/compare.stdout', b'match\n')
    put('quint-01/quint-observations.json', {'observations': observations,
                                             'raw_traces': [{'itf': {'states': [{}, {}]}}],
                                             'projection': {'terminal_stutters': [1]}})
    put('quint-01/result.json', {'sources_before': source_map, 'terminal_state': 'succeeded',
                                 'elapsed_seconds': 0.3,
                                 'steps': [{'name': 'compare', 'exit': 0, 'elapsed_seconds': 0.1,
                                            'timeout_seconds': 30, 'timed_out': False}]})
    stdout, stderr = put('quint-context-fix/locate.stdout', b'located\n'), put('quint-context-fix/locate.stderr', b'')
    script = put('quint-context-fix/actual-context-check.py', b"print('context')\n")
    put('quint-context-fix/actual-compare.stdout', compare)
    put('quint-context-fix/red.json', {'exit': 1})
    put('quint-context-fix/recheck.json', {'source_sha256': source_map, 'commands': [
        {'name': 'locate', 'exit': 0, 'stdout_sha256': sha(stdout), 'stderr_sha256': sha(stderr),
         'script_sha256': sha(script)}]})
    for name in ('host-summary.stdout', 'image-summary-corrected.stdout', 'quint-summary.stdout'):
        put(f'work-summary-01/{name}', summary)
    put('work-summary-01/invocations.json', {'source_sha256': source_map,
                                             'commands': [{'name': 'summary', 'exit_code': 0}]})
    # Other groups bind the same recorded bytes by copying ordinary-01's snapshot.
    for group in ('host-01', 'host-01-outer', 'image-01', 'image-01-outer', 'quint-01',
                  'quint-context-fix', 'work-summary-01'):
        for rel, data in SOURCES.items():
            selection.append({'path': f'{group}/source/{rel}', 'from': f'ordinary-01/source/{rel}',
                              'sha256': sha(data), 'bytes': len(data),
                              'origin': f'ordinary-01/source/{rel}'})
    for name, data in files.items():
        (runtime / name).parent.mkdir(parents=True, exist_ok=True)
        (runtime / name).write_bytes(data)
    fixture = types.SimpleNamespace(root=root, checkout=checkout, runtime=runtime, out=out,
                                    selection_path=root / 'selection.json',
                                    selection={'format': SELECTION_FORMAT,
                                               'members': sorted(selection, key=lambda m: m['path'])})
    save_selection(fixture)
    return fixture


def save_selection(fixture):
    fixture.selection_path.write_bytes(encode(fixture.selection))


def rewrite(fixture, name, value, reselect=True):
    """Replace one runtime file; optionally update the selection so only later checks can fail."""
    data = encode(value)
    (fixture.runtime / name).write_bytes(data)
    if reselect:
        for member in fixture.selection['members']:
            if member['from'] == name:
                member['sha256'], member['bytes'] = sha(data), len(data)
        save_selection(fixture)


def runtime_json(fixture, name):
    return json.loads((fixture.runtime / name).read_bytes())


def drop_member(fixture, path):
    fixture.selection['members'] = [m for m in fixture.selection['members'] if m['path'] != path]
    save_selection(fixture)


def member(fixture, path):
    return next(m for m in fixture.selection['members'] if m['path'] == path)


def output_bytes(fixture):
    return {p.relative_to(fixture.out).as_posix(): p.read_bytes()
            for p in sorted(fixture.out.rglob('*')) if p.is_file()}


PACKAGERS = {}
SAVED_FLAGS = {}


def setUpModule():
    SAVED_FLAGS['dont_write_bytecode'] = sys.dont_write_bytecode
    sys.dont_write_bytecode = True
    PACKAGERS['ordinary'] = load_packager(0)
    PACKAGERS['optimize=1'] = load_packager(1)


def tearDownModule():
    sys.dont_write_bytecode = SAVED_FLAGS.pop('dont_write_bytecode')


class PackagerCase(unittest.TestCase):
    def fixture(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        return build_fixture(tmp.name)

    def package(self, packager, fixture, expected=TINY):
        return packager.package(fixture.checkout, fixture.runtime, fixture.selection_path,
                                fixture.out, expected=expected)

    def refused(self, mutate, message, expected=TINY):
        """Each packager build refuses the mutated fixture and leaves no archive."""
        for label, packager in PACKAGERS.items():
            with self.subTest(build=label):
                fixture = self.fixture()
                mutate(fixture)
                with self.assertRaises(packager.PackagingError) as caught:
                    self.package(packager, fixture, expected)
                self.assertIn(message, str(caught.exception))
                self.assertEqual(sorted(p.name for p in fixture.out.iterdir()), [])
                self.assertFalse((fixture.runtime / 'delivery-package.json').exists())


class IntegrityChecksSurviveOptimize(PackagerCase):
    """S1: every integrity check is an explicit error, including under -O."""

    def test_optimize_build_strips_asserts(self):
        exec(compile('assert False\n', '<probe>', 'exec', optimize=1), {})
        with self.assertRaises(AssertionError):
            exec(compile('assert False\n', '<probe>', 'exec', optimize=0), {})

    def test_packager_contains_no_assert_statement(self):
        tree = ast.parse(PACKAGER.read_bytes(), str(PACKAGER))
        self.assertEqual([node.lineno for node in ast.walk(tree) if isinstance(node, ast.Assert)], [])

    def test_selected_bytes_must_match_selection_digest(self):
        self.refused(lambda f: rewrite(f, 'host-01/logs/run.stdout', b'tampered\n', reselect=False),
                     'selected bytes differ from selection: host-01/logs/run.stdout')

    def test_receipt_digest_reference_checked(self):
        self.refused(lambda f: rewrite(f, 'host-01/logs/run.stdout', b'host ok\n'.upper()),
                     'receipt digest mismatch: host-01/logs/run.stdout')

    def test_receipt_byte_count_checked(self):
        def mutate(f):
            receipt = runtime_json(f, 'image-01/receipt.json')
            receipt['logs'][0]['bytes'] += 1
            rewrite(f, 'image-01/receipt.json', receipt)
        self.refused(mutate, 'receipt size mismatch: image-01/logs/run.stdout')

    def test_capture_digest_checked(self):
        self.refused(lambda f: rewrite(f, 'quint-01/captures/t0.itf.json', {'states': [{}]}),
                     'capture digest mismatch: quint-01/captures/t0.itf.json')

    def test_context_log_digest_checked(self):
        self.refused(lambda f: rewrite(f, 'quint-context-fix/locate.stderr', b'late\n'),
                     'context log digest mismatch: quint-context-fix/locate.stderr')

    def test_context_script_digest_checked(self):
        self.refused(lambda f: rewrite(f, 'quint-context-fix/actual-context-check.py', b'pass\n'),
                     'context script digest mismatch')

    def test_comparison_copy_checked(self):
        self.refused(lambda f: rewrite(f, 'quint-context-fix/actual-compare.stdout', b'differ\n'),
                     'context comparison differs from quint-01/compare.stdout')

    def test_source_binding_checked(self):
        def mutate(f):
            result = runtime_json(f, 'quint-01/result.json')
            result['sources_before']['src/core.rs'] = '1' * 64
            rewrite(f, 'quint-01/result.json', result)
        self.refused(mutate, 'selected source differs from recorded digest: quint-01/source/src/core.rs')

    def test_expected_counts_checked(self):
        for key in TINY:
            with self.subTest(count=key):
                self.refused(lambda f: None, f'{key} count', expected={**TINY, key: TINY[key] + 1})

    def test_summary_counts_checked(self):
        def mutate(f):
            summary = runtime_json(f, 'work-summary-01/image-summary-corrected.stdout')
            summary['cross_run']['prefix_runs']['charged_sum'] = 0
            rewrite(f, 'work-summary-01/image-summary-corrected.stdout', summary)
        self.refused(mutate, 'image charged count')

    def test_irreproducible_archive_refused(self):
        for label, packager in PACKAGERS.items():
            with self.subTest(build=label):
                fixture = self.fixture()
                original, calls = packager.create_archive, []

                def drifting(path, members):
                    calls.append(path)
                    original(path, members)
                    if len(calls) == 2:
                        with path.open('ab') as handle:
                            handle.write(b'\0')
                packager.create_archive = drifting
                self.addCleanup(setattr, packager, 'create_archive', original)
                with self.assertRaises(packager.PackagingError) as caught:
                    self.package(packager, fixture)
                self.assertIn('archive construction is not reproducible', str(caught.exception))
                self.assertEqual(sorted(p.name for p in fixture.out.iterdir()), [])

    def test_archive_member_list_checked(self):
        for label, packager in PACKAGERS.items():
            with self.subTest(build=label):
                fixture = self.fixture()
                original = packager.create_archive
                packager.create_archive = lambda path, members: original(
                    path, {k: v for k, v in members.items() if k != 'ordinary-01/stderr'})
                self.addCleanup(setattr, packager, 'create_archive', original)
                with self.assertRaises(packager.PackagingError) as caught:
                    self.package(packager, fixture)
                self.assertIn('archive member list differs from selection', str(caught.exception))
                self.assertEqual(sorted(p.name for p in fixture.out.iterdir()), [])

    def test_real_optimize_child_refuses_tampered_capture(self):
        fixture = self.fixture()
        rewrite(fixture, 'quint-01/captures/t0.itf.json', {'states': []})
        # The child uses the frozen default counts, so it stops at the capture digest first.
        child = subprocess.run(
            [sys.executable, '-O', '-B', '-I', '-c', OPTIMIZE_CHILD, str(PACKAGER),
             str(fixture.checkout), str(fixture.runtime), str(fixture.selection_path), str(fixture.out)],
            capture_output=True, text=True, timeout=60, cwd=fixture.root, env={'PATH': os.defpath})
        self.assertEqual(child.returncode, 1, child.stderr)
        self.assertIn('optimize 1', child.stderr)
        self.assertIn('package_public_evidence: refused: capture digest mismatch: '
                      'quint-01/captures/t0.itf.json', child.stderr)
        self.assertEqual(child.stdout, '')
        self.assertEqual(sorted(p.name for p in fixture.out.iterdir()), [])


class ExplicitSelectedMembership(PackagerCase):
    """S2: only selected runtime files are read; the checkout is never a byte source."""

    def test_archive_is_exactly_the_selection(self):
        for label, packager in PACKAGERS.items():
            with self.subTest(build=label):
                fixture = self.fixture()
                (fixture.runtime / 'host-01/stray-unselected.txt').write_bytes(b'not selected\n')
                (fixture.runtime / 'ordinary-01/source/models/qa/future_source.py').write_bytes(b'# stray\n')
                delivery = self.package(packager, fixture)
                expected = [{k: m[k] for k in ('path', 'sha256', 'bytes', 'origin')}
                            for m in fixture.selection['members']]
                listed = json.loads((fixture.out / 'ARCHIVE-MEMBERS.json').read_bytes())
                self.assertEqual(listed, expected)
                with tarfile.open(fixture.out / 'public-evidence.tar.gz', 'r:gz') as tar:
                    self.assertEqual(tar.getnames(), [m['path'] for m in expected])
                self.assertEqual(delivery['archive']['members'], len(expected))
                result = json.loads((fixture.out / 'RESULT.json').read_bytes())
                self.assertEqual(result['verification']['explicit_selection_sha256'],
                                 sha(fixture.selection_path.read_bytes()))
                self.assertEqual(result['verification']['included_receipt_digest_references_checked'], 2)
                self.assertEqual(result['verification']['excluded_executable_tool_or_other_digest_references'], 2)

    def test_packaging_is_deterministic(self):
        packager = PACKAGERS['ordinary']
        fixture = self.fixture()
        first = self.package(packager, fixture)
        archive = (fixture.out / 'public-evidence.tar.gz').read_bytes()
        fixture.out = fixture.checkout / 'package-again'
        second = self.package(packager, fixture)
        self.assertEqual(archive, (fixture.out / 'public-evidence.tar.gz').read_bytes())
        self.assertEqual(first['archive'], second['archive'])
        self.assertEqual(first['archive']['sha256'], sha(archive))

    def test_unselected_outer_context_is_listed_not_archived(self):
        packager = PACKAGERS['ordinary']
        fixture = self.fixture()
        self.package(packager, fixture)
        bindings = json.loads((fixture.out / 'SOURCE-BINDINGS.json').read_bytes())
        self.assertEqual([(u['attempt'], u['path']) for u in bindings['unrecovered_outer_context']],
                         [('host-01-outer', 'models/qa/future_source.py'),
                          ('image-01-outer', 'models/qa/future_source.py')])
        self.assertEqual(len(bindings['verified']), 2 * 8)
        names = [m['path'] for m in json.loads((fixture.out / 'ARCHIVE-MEMBERS.json').read_bytes())]
        self.assertFalse([n for n in names if 'future_source' in n])

    def test_no_checkout_fallback_for_unselected_source(self):
        def mutate(f):
            # The checkout holds the exact recorded bytes; they still must not be used.
            for rel, data in SOURCES.items():
                (f.checkout / rel).parent.mkdir(parents=True, exist_ok=True)
                (f.checkout / rel).write_bytes(data)
            drop_member(f, 'host-01/source/src/core.rs')
        self.refused(mutate, 'recorded source not selected: host-01:src/core.rs')

    def test_required_record_must_be_selected(self):
        self.refused(lambda f: drop_member(f, 'host-01/receipt.json'),
                     'required record not selected: host-01/receipt.json')

    def test_late_required_record_refused_before_any_output(self):
        # Finding F1: red.json was first read after the archive was published.
        self.refused(lambda f: drop_member(f, 'quint-context-fix/red.json'),
                     'required record not selected: quint-context-fix/red.json')
        packager = PACKAGERS['ordinary']
        fixture = self.fixture()
        self.package(packager, fixture)
        published = output_bytes(fixture)
        delivery = (fixture.runtime / 'delivery-package.json').read_bytes()
        drop_member(fixture, 'quint-context-fix/red.json')
        with self.assertRaises(packager.PackagingError):
            self.package(packager, fixture)
        self.assertEqual(output_bytes(fixture), published)
        self.assertEqual((fixture.runtime / 'delivery-package.json').read_bytes(), delivery)

    def test_missing_result_field_leaves_output_untouched(self):
        def mutate(f):
            result = runtime_json(f, 'ordinary-01/result.json')
            del result['exit']
            rewrite(f, 'ordinary-01/result.json', result)
        for label, packager in PACKAGERS.items():
            with self.subTest(build=label):
                fixture = self.fixture()
                mutate(fixture)
                with self.assertRaises(KeyError):
                    self.package(packager, fixture)
                self.assertEqual(list(fixture.out.iterdir()), [])
                self.assertFalse((fixture.runtime / 'delivery-package.json').exists())

    def test_member_that_is_also_a_folder_refused(self):
        def mutate(f):
            f.selection['members'].append({**member(f, 'ordinary-01/stderr'), 'path': 'ordinary-01/stderr/extra'})
            save_selection(f)
        self.refused(mutate, 'archive member is also a folder of another member: ordinary-01/stderr')

    def test_selected_source_needs_recorded_digest(self):
        def mutate(f):
            data = b'# unbound\n'
            (f.runtime / 'extra.py').write_bytes(data)
            f.selection['members'].append({'path': 'quint-01/source/extra.py', 'from': 'extra.py',
                                           'sha256': sha(data), 'bytes': len(data), 'origin': 'x'})
            save_selection(f)
        self.refused(mutate, 'selected source has no recorded digest: quint-01/source/extra.py')

    def test_selection_paths_confined(self):
        cases = {
            '../escape': ('from', '../escape', 'selected source is not normalized'),
            '/abs': ('from', '/etc/hostname', 'selected source is not a relative POSIX path'),
            'dot': ('from', 'ordinary-01/./stderr', 'selected source is not normalized'),
            'backslash': ('path', 'ordinary-01\\stderr', 'archive member is not a relative POSIX path'),
            'group': ('path', 'elsewhere/stderr', 'archive member outside the delivery groups'),
            'build': ('path', 'ordinary-01/build/stderr', 'build/tmp/__pycache__ member refused'),
            'cache': ('path', 'ordinary-01/__pycache__/x.pyc', 'build/tmp/__pycache__ member refused'),
            'missing': ('from', 'ordinary-01/absent', 'selected file is missing or not regular'),
            'directory': ('from', 'ordinary-01/source', 'selected file is missing or not regular'),
        }
        for case, (key, value, message) in cases.items():
            with self.subTest(case=case):
                def mutate(f, key=key, value=value):
                    member(f, 'ordinary-01/stderr')[key] = value
                    save_selection(f)
                self.refused(mutate, message)

    def test_symlinked_selection_refused(self):
        def leaf(f):
            target = f.root / 'outside.txt'
            target.write_bytes(b'Ran 2 tests\n\nOK\n')
            (f.runtime / 'ordinary-01/stderr').unlink()
            (f.runtime / 'ordinary-01/stderr').symlink_to(target)
        self.refused(leaf, 'symlink refused: ordinary-01/stderr')

        def parent(f):
            (f.root / 'elsewhere').mkdir()
            (f.root / 'elsewhere/stderr').write_bytes(b'Ran 2 tests\n\nOK\n')
            (f.runtime / 'linked').symlink_to(f.root / 'elsewhere')
            member(f, 'ordinary-01/stderr')['from'] = 'linked/stderr'
            save_selection(f)
        self.refused(parent, 'symlink refused: linked/stderr')

    def test_selection_shape_refused(self):
        def duplicate(f):
            f.selection['members'].append(dict(member(f, 'ordinary-01/stderr')))
            save_selection(f)

        def extra_key(f):
            member(f, 'ordinary-01/stderr')['mode'] = 0o755
            save_selection(f)

        def bad_digest(f):
            member(f, 'ordinary-01/stderr')['sha256'] = 'A' * 64
            save_selection(f)

        def bool_size(f):
            member(f, 'ordinary-01/stderr')['bytes'] = True
            save_selection(f)

        def wrong_format(f):
            f.selection['format'] = 'sherlock-parallel-public-selection/0'
            save_selection(f)

        def duplicate_key(f):
            text = f.selection_path.read_text()
            f.selection_path.write_text(text.replace('{\n', '{\n "format": "x",\n', 1))

        def empty(f):
            f.selection['members'] = []
            save_selection(f)
        cases = {duplicate: 'duplicate archive member: ordinary-01/stderr',
                 extra_key: 'selection member needs exactly',
                 bad_digest: 'selection digest malformed: ordinary-01/stderr',
                 bool_size: 'selection size malformed: ordinary-01/stderr',
                 wrong_format: 'selection must be',
                 duplicate_key: 'duplicate JSON key in selection',
                 empty: 'selection members must be a nonempty list'}
        for mutate, message in cases.items():
            with self.subTest(case=mutate.__name__):
                self.refused(mutate, message)

    def test_two_argument_scan_usage_refused(self):
        packager = PACKAGERS['optimize=1']
        fixture = self.fixture()
        with self.assertRaises(SystemExit) as caught:
            packager.main([str(fixture.checkout), str(fixture.runtime)], out=fixture.out)
        self.assertIn('SELECTION', str(caught.exception.code))
        self.assertEqual(list(fixture.out.iterdir()), [])
        self.assertFalse((fixture.runtime / 'delivery-package.json').exists())

    def test_output_must_lie_inside_checkout(self):
        packager = PACKAGERS['ordinary']
        fixture = self.fixture()
        with self.assertRaises(packager.PackagingError) as caught:
            packager.package(fixture.checkout, fixture.runtime, fixture.selection_path,
                             fixture.root / 'outside', expected=TINY)
        self.assertIn('output folder is outside the checkout', str(caught.exception))
        self.assertFalse((fixture.root / 'outside').exists())

    def test_frozen_member_names_remain_selectable(self):
        # Read-only: the frozen October 16 membership fits the explicit selection rules.
        packager = PACKAGERS['ordinary']
        frozen = json.loads((DELIVERY / 'ARCHIVE-MEMBERS.json').read_bytes())
        self.assertEqual(len(frozen), 487)
        for entry in frozen:
            packager.check_member_name(entry['path'])
        packager.check_member_folders([entry['path'] for entry in frozen])


if __name__ == '__main__':
    unittest.main()
