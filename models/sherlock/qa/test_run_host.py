#!/usr/bin/env python3
"""Regression tests for the pure guards of the bounded normal Sherlock host wrapper.

The actual run_host.py is imported by file spec; main() never runs in-process.
Nothing here launches rustc, Cargo, a model, an SDK, native code or the network:
helpers receive inputs built inside temporary directories, and the CLI cases run
only the wrapper's own refusals with a nonexistent compiler path, so even a
regressed guard stops at preflight before any child is started.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import unittest

WRAPPER = Path(__file__).resolve().with_name('run_host.py')
SPEC = importlib.util.spec_from_file_location('sherlock_qa_run_host_under_test', WRAPPER)
run_host = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(run_host)
Failure = run_host.Failure
COUNT = len(run_host.EXPECTED_TESTS)
SUMMARY = (f'test result: ok. {COUNT} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; '
           'finished in 0.01s')
PASSING = [(name, 'ok') for name in run_host.EXPECTED_TESTS]
FAKE_RUSTC = b'plain text, not a compiler\n'
CLI_ENV = {'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8'}


def libtest_output(results, running=COUNT, summary=SUMMARY):
    """Render stdout shaped like a libtest binary run with --nocapture."""
    lines = ['', f'running {running} tests'] + [f'test {name} ... {outcome}' for name, outcome in results]
    return '\n'.join(lines + ['', summary, ''])


def temporary_root(case):
    """A fresh resolved temporary directory, removed when the test ends."""
    tmp = tempfile.TemporaryDirectory()
    case.addCleanup(tmp.cleanup)
    return Path(os.path.realpath(tmp.name))


def declared_rust_tests(relative):
    """Names of #[test] functions in a checkout source file; it is read, never compiled."""
    text = (run_host.REPO / relative).read_text(encoding='utf-8')
    return re.findall(r'(?m)^#\[test\]\s*\nfn ([a-z0-9_]+)\(', text)


class LibtestResults(unittest.TestCase):
    def test_exact_fourteen_named_passes_are_accepted_in_any_order(self):
        self.assertEqual(COUNT, 14)
        self.assertEqual(run_host.libtest_results(libtest_output(PASSING)), list(run_host.EXPECTED_TESTS))
        reordered = PASSING[::-1]
        self.assertEqual(run_host.libtest_results(libtest_output(reordered)), [name for name, _ in reordered])

    def test_either_module_alone_with_consistent_counts_is_refused(self):
        for module in ('reference_bounded_tests::', 'reference_bounded_extra_tests::'):
            alone = [row for row in PASSING if row[0].startswith(module)]
            with self.subTest(module):
                self.assertEqual(len(alone), 7)
                text = libtest_output(alone, running=7, summary=SUMMARY.replace(f'{COUNT} passed', '7 passed'))
                with self.assertRaises(Failure):
                    run_host.libtest_results(text)

    def test_an_empty_declared_suite_is_never_a_pass(self):
        empty = libtest_output([], running=0, summary=SUMMARY.replace(f'{COUNT} passed', '0 passed'))
        with self.assertRaisesRegex(Failure, 'empty suite is never a pass'):
            run_host.libtest_results(empty, ())

    def test_missing_extra_failed_ignored_or_misnamed_results_are_refused(self):
        (first, _), rest = PASSING[0], PASSING[1:]
        cases = dict(
            missing=libtest_output(rest),
            missing_with_consistent_counts=libtest_output(
                rest, running=COUNT - 1, summary=SUMMARY.replace(f'{COUNT} passed', f'{COUNT - 1} passed')),
            extra=libtest_output(PASSING + [('reference_bounded_tests::unexpected', 'ok')]),
            duplicated=libtest_output(rest + rest[:1]),
            failed=libtest_output([(first, 'FAILED')] + rest),
            ignored=libtest_output([(first, 'ignored')] + rest),
            unqualified=libtest_output([(first.split('::', 1)[1], 'ok')] + rest),
            other_module=libtest_output([(first.replace('reference_bounded_tests', 'reference_tests'), 'ok')] + rest),
            swapped_module=libtest_output(
                [(first.replace('reference_bounded_tests', 'reference_bounded_extra_tests'), 'ok')] + rest),
        )
        for name, text in cases.items():
            with self.subTest(name):
                with self.assertRaises(Failure):
                    run_host.libtest_results(text)

    def test_filtered_or_incomplete_summaries_are_refused(self):
        cases = dict(
            filtered=libtest_output(PASSING, summary=SUMMARY.replace('0 filtered out', '1 filtered out')),
            failed_summary=libtest_output(PASSING, summary=SUMMARY.replace('test result: ok.', 'test result: FAILED.')),
            ignored_summary=libtest_output(PASSING, summary=SUMMARY.replace('0 ignored', '1 ignored')),
            no_summary=libtest_output(PASSING, summary=''),
            wrong_announcement=libtest_output(PASSING, running=COUNT + 1),
            no_announcement=libtest_output(PASSING).replace(f'running {COUNT} tests', ''),
        )
        for name, text in cases.items():
            with self.subTest(name):
                with self.assertRaises(Failure):
                    run_host.libtest_results(text)


class CompileInputs(unittest.TestCase):
    def setUp(self):
        self.base = temporary_root(self)
        self.snapshot = self.base / 'source'
        for path in (self.snapshot, self.base / 'build'):
            path.mkdir()

    def inside(self, closure):
        return [str(self.snapshot / relative) for relative in closure]

    def dep_info(self, deps):
        """Write dep-info in rustc's --emit=dep-info layout; no compiler runs."""
        target = self.base / 'build' / 'crate'
        rule = ' '.join(deps)
        lines = [f'{target}.d: {rule}', f'{target}: {rule}', ''] + [dep + ':' for dep in deps]
        # A comment line naming an outside path is not an input.
        lines += ['', '# note: ' + str(self.base / 'outside.rs'), '']
        path = self.base / 'build' / 'crate.d'
        path.write_text('\n'.join(lines), encoding='utf-8')
        return path

    def test_exact_declared_closure_is_accepted(self):
        for closure in (run_host.PRODUCER_CLOSURE, run_host.TEST_CLOSURE, run_host.IMAGE_CLOSURE):
            with self.subTest(members=len(closure)):
                result = run_host.compile_inputs(self.dep_info(self.inside(closure)), self.snapshot, closure)
                self.assertEqual(result, sorted(closure))

    def test_omitted_or_undeclared_members_are_refused(self):
        producer, tests = run_host.PRODUCER_CLOSURE, run_host.TEST_CLOSURE
        cases = dict(
            omitted=(self.inside(producer[1:]), producer),
            test_module_in_producer=(self.inside(tests), producer),
            unread_test_module=(self.inside(producer), tests),
            undeclared_sibling=(self.inside(producer + (run_host.SHERLOCK + 'mod.rs',)), producer),
            unread_extra_module=(self.inside(tuple(m for m in tests if 'extra_tests' not in m)), tests),
            unread_literal_includes=(self.inside(tuple(m for m in tests if not m.endswith('_oracles.rs'))), tests),
            machine_in_image=(self.inside(run_host.IMAGE_CLOSURE + (run_host.SHERLOCK + 'reference.rs',)),
                              run_host.IMAGE_CLOSURE),
            image_test_module=(self.inside(run_host.IMAGE_CLOSURE + (run_host.SHERLOCK + 'program_image_tests.rs',)),
                               run_host.IMAGE_CLOSURE),
            host_producer_for_image=(self.inside(producer), run_host.IMAGE_CLOSURE),
        )
        for name, (deps, closure) in cases.items():
            with self.subTest(name):
                with self.assertRaisesRegex(Failure, 'compile closure mismatch'):
                    run_host.compile_inputs(self.dep_info(deps), self.snapshot, closure)

    def test_outside_or_relative_inputs_are_refused(self):
        producer = run_host.PRODUCER_CLOSURE
        inside = self.inside(producer)
        cases = dict(
            outside_root=inside + [str(self.base / 'outside.rs')],
            sibling_prefix=inside + [str(self.base / 'source-shadow' / 'x.rs')],
            parent_traversal=inside + [str(self.snapshot / 'models' / '..' / '..' / 'outside.rs')],
            system=inside + ['/usr/lib/x.rs'],
            relative=[producer[0]] + inside[1:],
        )
        for name, deps in cases.items():
            with self.subTest(name):
                with self.assertRaisesRegex(Failure, 'outside the source snapshot'):
                    run_host.compile_inputs(self.dep_info(deps), self.snapshot, producer)


class ToolchainChannel(unittest.TestCase):
    def test_exactly_one_numeric_pin_is_accepted(self):
        raw = b'[toolchain]\nchannel = "1.85.0"\ncomponents = ["rustfmt", "clippy"]\n'
        self.assertEqual(run_host.toolchain_channel(raw), '1.85.0')
        commented = b'[toolchain]\n# channel = "1.0.0"\nchannel = "1.85.0"\n'
        self.assertEqual(run_host.toolchain_channel(commented), '1.85.0')

    def test_absent_duplicate_or_nonnumeric_pins_are_refused(self):
        cases = dict(
            absent=b'[toolchain]\ncomponents = ["rustfmt"]\n',
            duplicate=b'[toolchain]\nchannel = "1.85.0"\nchannel = "1.85.0"\n',
            conflicting=b'[toolchain]\nchannel = "1.85.0"\nchannel = "1.86.0"\n',
            named=b'[toolchain]\nchannel = "stable"\n',
            two_components=b'[toolchain]\nchannel = "1.85"\n',
            nightly=b'[toolchain]\nchannel = "nightly-2025-01-01"\n',
            commented=b'[toolchain]\n# channel = "1.85.0"\n',
            indented=b'[toolchain]\n  channel = "1.85.0"\n',
        )
        for name, raw in cases.items():
            with self.subTest(name):
                with self.assertRaisesRegex(Failure, 'exactly one numeric channel pin'):
                    run_host.toolchain_channel(raw)


class CompilerRecord(unittest.TestCase):
    def setUp(self):
        self.base = temporary_root(self)

    def fake_rustc(self, home, mode=0o755, sysroot=False):
        """Plain non-compiler text named rustc; it is only stat()ed and hashed, never run."""
        binary = self.base / home / 'bin' / 'rustc'
        binary.parent.mkdir(parents=True)
        binary.write_bytes(FAKE_RUSTC)
        binary.chmod(mode)
        if sysroot:
            (self.base / home / 'lib' / 'rustlib').mkdir(parents=True)
        return str(binary)

    def test_rustup_proxy_shape_is_refused(self):
        rustc = self.fake_rustc('cargo')
        with self.assertRaisesRegex(Failure, 'rustup proxy is not accepted'):
            run_host.compiler_record(rustc)

    def test_sysroot_shape_is_recorded_without_hashing_the_sysroot(self):
        # Positive control: the guard is shape-only, so plain text in a sysroot layout passes.
        rustc = self.fake_rustc('toolchain', sysroot=True)
        record = run_host.compiler_record(rustc)
        self.assertEqual((record['path'], record['bytes']), (rustc, len(FAKE_RUSTC)))
        self.assertEqual(record['sha256'], hashlib.sha256(FAKE_RUSTC).hexdigest())
        self.assertEqual(record['sysroot'], str(self.base / 'toolchain'))
        self.assertIs(record['sysroot_hashed'], False)

    def test_symlinked_relative_nonexecutable_or_missing_compilers_are_refused(self):
        rustc = self.fake_rustc('toolchain', sysroot=True)
        link = self.base / 'rustc-link'
        link.symlink_to(rustc)
        for name, path in (('symlink', str(link)), ('relative', 'toolchain/bin/rustc')):
            with self.subTest(name):
                with self.assertRaisesRegex(Failure, 'must not traverse a symlink'):
                    run_host.compiler_record(path)
        directory = self.base / 'directory' / 'bin' / 'rustc'
        directory.mkdir(parents=True)
        (self.base / 'directory' / 'lib' / 'rustlib').mkdir(parents=True)
        for name, path in (('not_executable', self.fake_rustc('readonly', mode=0o644, sysroot=True)),
                           ('directory', str(directory))):
            with self.subTest(name):
                with self.assertRaisesRegex(Failure, 'not an executable regular file'):
                    run_host.compiler_record(path)
        # The CLI cases below rely on a nonexistent compiler failing here, before any child.
        with self.assertRaises(OSError):
            run_host.compiler_record(str(self.base / 'absent' / 'bin' / 'rustc'))


class ComparisonHelpers(unittest.TestCase):
    LOAD = b"path = Path(__file__).resolve().with_name('%s')\n"

    def contents(self, check, helper=b''):
        contents = {run_host.CHECK: check}
        contents.update({path: helper for path in run_host.CHECK_HELPERS})
        return contents

    def test_exact_declared_loads_are_accepted(self):
        loads = b''.join(self.LOAD % os.path.basename(path).encode() for path in run_host.CHECK_HELPERS)
        self.assertEqual(run_host.comparison_helpers(self.contents(loads)), sorted(run_host.CHECK_HELPERS))

    def test_undeclared_or_unloaded_helpers_are_refused(self):
        declared = b''.join(self.LOAD % os.path.basename(path).encode() for path in run_host.CHECK_HELPERS)
        cases = dict(
            undeclared_in_check=self.contents(declared + self.LOAD % b'image_observations.py'),
            undeclared_in_helper=self.contents(declared, helper=self.LOAD % b'layer_observations.py'),
            double_quoted=self.contents(declared + b'x.with_name("k_terms.py")\n'),
            unloaded=self.contents(b'# host_observations.py is not loaded\n'),
        )
        for name, contents in cases.items():
            with self.subTest(name):
                with self.assertRaisesRegex(Failure, 'comparison helper loads differ'):
                    run_host.comparison_helpers(contents)

    def test_checkout_check_loads_exactly_the_declared_helpers(self):
        contents = run_host.read_sources(run_host.REPO, run_host.COMMON_PATHS)
        self.assertEqual(run_host.comparison_helpers(contents), sorted(run_host.CHECK_HELPERS))


class TerminalState(unittest.TestCase):
    PLAN = ('preflight', 'rustc-version', 'compile-producer', 'compile-tests', 'run-tests', 'run-producer',
            'compare', 'postflight')

    def steps(self, **states):
        return [dict(name=name, state=states.get(name.replace('-', '_'), 'ok')) for name in self.PLAN]

    def test_complete_host_success_and_declared_image_skip(self):
        self.assertEqual(run_host.terminal_state(self.steps()), 'success')
        skipped = self.steps(compile_tests='not-selected', run_tests='not-selected')
        self.assertEqual(run_host.terminal_state(skipped, run_host.NOT_SELECTED), 'success')

    def test_undeclared_skips_and_failures_are_not_success(self):
        cases = dict(
            host_skips_tests=(self.steps(compile_tests='not-selected', run_tests='not-selected'), (),
                              'not-selected:compile-tests'),
            image_skips_compare=(self.steps(compile_tests='not-selected', run_tests='not-selected',
                                            compare='not-selected'), run_host.NOT_SELECTED, 'not-selected:compare'),
            image_not_run_tests=(self.steps(compile_tests='not-selected', run_tests='not-run'),
                                 run_host.NOT_SELECTED, 'not-run:run-tests'),
            timeout=(self.steps(run_tests='timeout', run_producer='not-run'), (), 'timeout:run-tests'),
            interrupted=(self.steps(compare='running'), (), 'interrupted:compare'),
        )
        for name, (steps, allowed, expected) in cases.items():
            with self.subTest(name):
                self.assertEqual(run_host.terminal_state(steps, allowed), expected)


class ReadSources(unittest.TestCase):
    def tree(self, paths=run_host.SOURCE_PATHS):
        root = temporary_root(self)
        for relative in paths:
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(relative.encode())
        return root

    def test_complete_regular_tree_is_read_in_declared_order(self):
        for paths in (run_host.SOURCE_PATHS, run_host.IMAGE_SOURCE_PATHS):
            with self.subTest(members=len(paths)):
                contents = run_host.read_sources(self.tree(paths), paths)
                self.assertEqual(list(contents), list(paths))
                self.assertTrue(all(raw == path.encode() for path, raw in contents.items()))

    def test_image_member_absent_from_a_host_only_tree_is_refused(self):
        with self.assertRaises(OSError):
            run_host.read_sources(self.tree(), run_host.IMAGE_SOURCE_PATHS)

    def test_symlinked_member_or_parent_is_refused(self):
        root = self.tree()
        member = root / run_host.PRODUCER
        outside = temporary_root(self) / 'producer.rs'
        outside.write_bytes(run_host.PRODUCER.encode())
        member.unlink()
        member.symlink_to(outside)
        with self.assertRaisesRegex(Failure, 'symlinked source member'):
            run_host.read_sources(root)
        root = self.tree()
        oracles, moved = root / 'models' / 'sherlock' / 'oracles', root / 'oracles-elsewhere'
        oracles.rename(moved)
        oracles.symlink_to(moved, target_is_directory=True)
        with self.assertRaisesRegex(Failure, 'symlinked source member'):
            run_host.read_sources(root)

    def test_missing_or_nonregular_member_is_refused(self):
        root = self.tree()
        (root / run_host.CHECK).unlink()
        with self.assertRaises(OSError):
            run_host.read_sources(root)
        root = self.tree()
        member = root / run_host.SERIALIZER
        member.unlink()
        member.mkdir()
        with self.assertRaisesRegex(Failure, 'not a regular file'):
            run_host.read_sources(root)


class SourceMembership(unittest.TestCase):
    def test_source_paths_are_unique_normalized_and_dep_info_safe(self):
        for paths in (run_host.SOURCE_PATHS, run_host.IMAGE_SOURCE_PATHS):
            self.assertEqual(len(set(paths)), len(paths))
            for relative in paths:
                with self.subTest(relative):
                    self.assertFalse(os.path.isabs(relative))
                    self.assertEqual(os.path.normpath(relative), relative)
                    self.assertNotIn('..', relative.split('/'))
                    self.assertTrue(run_host.SAFE_PATH.fullmatch('/' + relative))

    def test_closures_bind_both_bounded_normal_test_modules(self):
        producer, tests = run_host.PRODUCER_CLOSURE, run_host.TEST_CLOSURE
        added = tuple(run_host.SHERLOCK + name for name in (
            'reference_bounded_tests.rs', 'reference_bounded_extra_tests.rs',
            'normal_oracles.rs', 'extra_normal_oracles.rs'))
        self.assertEqual(producer[0], run_host.PRODUCER)
        self.assertEqual(len(set(tests)), len(tests))
        self.assertEqual(tests, producer + added)
        self.assertFalse(set(added) & set(producer))
        self.assertLessEqual(set(tests), set(run_host.SOURCE_PATHS))
        for member in (run_host.CHECK, run_host.SERIALIZER, 'rust-toolchain.toml',
                       WRAPPER.relative_to(run_host.REPO).as_posix()) + run_host.CHECK_HELPERS:
            self.assertIn(member, run_host.SOURCE_PATHS)
            self.assertIn(member, run_host.IMAGE_SOURCE_PATHS)
        self.assertEqual(len(set(run_host.EXPECTED_TESTS)), COUNT)
        modules = [name.split('::', 1)[0] for name in run_host.EXPECTED_TESTS]
        self.assertEqual(modules, ['reference_bounded_tests'] * 7 + ['reference_bounded_extra_tests'] * 7)

    def test_image_closure_excludes_machine_tests_and_host_producer(self):
        image = run_host.IMAGE_CLOSURE
        self.assertEqual(image[0], run_host.IMAGE_PRODUCER)
        self.assertEqual(len(set(image)), len(image))
        for member in (run_host.PRODUCER, run_host.SHERLOCK + 'reference.rs', run_host.SHERLOCK + 'compare.rs'):
            self.assertNotIn(member, image)
            self.assertNotIn(member, run_host.IMAGE_SOURCE_PATHS)
        self.assertFalse([member for member in image if member.endswith('_tests.rs')])
        self.assertLessEqual(set(image), set(run_host.IMAGE_SOURCE_PATHS))
        for module in ('reference_bounded_tests.rs', 'reference_bounded_extra_tests.rs'):
            self.assertNotIn(run_host.SHERLOCK + module, run_host.IMAGE_SOURCE_PATHS)

    def test_realizations_are_fixed_and_distinct(self):
        self.assertEqual(sorted(run_host.REALIZATIONS), ['host', 'image'])
        host, image = run_host.REALIZATIONS['host'], run_host.REALIZATIONS['image']
        self.assertEqual((host.format, host.producer, host.tests, host.sources, host.test_closure),
                         (run_host.FORMAT, run_host.PRODUCER, run_host.EXPECTED_TESTS, run_host.SOURCE_PATHS,
                          run_host.TEST_CLOSURE))
        self.assertEqual((image.tests, image.test_closure, image.sources),
                         ((), None, run_host.IMAGE_SOURCE_PATHS))
        self.assertEqual(image.claims['acceptance'], 'none')
        self.assertTrue(image.claims['tests'].startswith('not-selected'))
        for field in ('format', 'producer', 'crate', 'compare_flag', 'report_key', 'output'):
            with self.subTest(field):
                self.assertNotEqual(getattr(host, field), getattr(image, field))

    def test_expected_test_names_match_the_checkout_test_sources(self):
        for module in ('reference_bounded_tests', 'reference_bounded_extra_tests'):
            with self.subTest(module):
                declared = declared_rust_tests(run_host.SHERLOCK + module + '.rs')
                expected = [name.split('::', 1)[1] for name in run_host.EXPECTED_TESTS
                            if name.split('::', 1)[0] == module]
                self.assertEqual(sorted(declared), sorted(expected))

    def test_checkout_members_pin_one_channel_and_the_workspace_edition(self):
        self.assertEqual(list(run_host.read_sources(run_host.REPO, run_host.IMAGE_SOURCE_PATHS)),
                         list(run_host.IMAGE_SOURCE_PATHS))
        contents = run_host.read_sources(run_host.REPO)
        self.assertEqual(list(contents), list(run_host.SOURCE_PATHS))
        self.assertEqual(contents['models/sherlock/qa/run_host.py'], WRAPPER.read_bytes())
        parts = run_host.toolchain_channel(contents['rust-toolchain.toml']).split('.')
        self.assertEqual(len(parts), 3)
        self.assertTrue(all(part.isdigit() for part in parts))
        pinned = f'edition = "{run_host.EDITION}"'
        self.assertIn(pinned, [line.strip() for line in contents['Cargo.toml'].decode().splitlines()])
        for line in contents['crates/mori-core/Cargo.toml'].decode().splitlines():
            if line.strip().startswith('edition'):
                self.assertTrue('workspace' in line or line.strip() == pinned, line)


@unittest.skipUnless(sys.executable and os.path.isabs(sys.executable), 'no absolute interpreter path')
class CliPreflight(unittest.TestCase):
    """Argument refusals only; the compiler path never exists, so no child can start."""

    def setUp(self):
        self.base = str(temporary_root(self))
        self.rustc = os.path.join(self.base, 'absent', 'bin', 'rustc')

    def wrapper(self, *args):
        return subprocess.run([sys.executable, '-I', '-B', str(WRAPPER), *args], cwd=self.base, env=CLI_ENV,
                              stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=30)

    def test_missing_arguments_exit_2(self):
        for args in ((), ('--output-dir', os.path.join(self.base, 'attempt')), ('--rustc', self.rustc)):
            with self.subTest(args=args):
                result = self.wrapper(*args)
                self.assertEqual(result.returncode, 2)
                self.assertIn('required', result.stderr)
        self.assertEqual(os.listdir(self.base), [])

    def test_existing_output_dir_exits_2_and_is_untouched(self):
        existing = os.path.join(self.base, 'existing')
        repo = str(run_host.REPO)
        if not run_host.SAFE_PATH.fullmatch(existing) or os.path.commonpath([existing, repo]) == repo:
            self.skipTest('temporary directory is not an accepted output location here')
        os.mkdir(existing, 0o700)
        sentinel = os.path.join(existing, 'sentinel')
        with open(sentinel, 'xb') as handle:
            handle.write(b'keep\n')
        result = self.wrapper('--rustc', self.rustc, '--output-dir', existing)
        self.assertEqual(result.returncode, 2)
        self.assertIn('no fresh attempt directory', result.stderr)
        self.assertEqual(result.stdout, '')
        self.assertEqual(os.listdir(existing), ['sentinel'])
        with open(sentinel, 'rb') as handle:
            self.assertEqual(handle.read(), b'keep\n')

    def test_relative_output_dir_exits_2_without_creating_it(self):
        result = self.wrapper('--rustc', self.rustc, '--output-dir', 'attempt')
        self.assertEqual(result.returncode, 2)
        self.assertIn('--output-dir: expected a normalized absolute path', result.stderr)
        self.assertEqual(os.listdir(self.base), [])

    def test_unsafe_output_dir_characters_exit_2_without_creating_it(self):
        result = self.wrapper('--rustc', self.rustc, '--output-dir', os.path.join(self.base, 'attempt dir'))
        self.assertEqual(result.returncode, 2)
        self.assertIn('--output-dir: only', result.stderr)
        self.assertEqual(os.listdir(self.base), [])

    def test_unknown_realization_exits_2_without_creating_it(self):
        for value in ('source', 'HOST', ''):
            with self.subTest(value=value):
                result = self.wrapper('--rustc', self.rustc, '--output-dir', os.path.join(self.base, 'attempt'),
                                      '--realization', value)
                self.assertEqual(result.returncode, 2)
                self.assertIn('invalid choice', result.stderr)
        self.assertEqual(os.listdir(self.base), [])

    def test_absent_compiler_stops_each_realization_at_preflight_without_a_child(self):
        repo = str(run_host.REPO)
        for realization in ('host', 'image'):
            attempt = os.path.join(self.base, 'attempt-' + realization)
            if not run_host.SAFE_PATH.fullmatch(attempt) or os.path.commonpath([attempt, repo]) == repo:
                self.skipTest('temporary directory is not an accepted output location here')
            with self.subTest(realization):
                result = self.wrapper('--rustc', self.rustc, '--output-dir', attempt, '--realization', realization)
                self.assertEqual(result.returncode, 1, result.stderr)
                summary = json.loads(result.stdout)
                self.assertEqual((summary['realization'], summary['terminal_state']),
                                 (realization, 'failed:preflight'))
                with open(os.path.join(attempt, 'receipt.json'), encoding='utf-8') as handle:
                    receipt = json.load(handle)
                self.assertEqual(receipt['format'], run_host.REALIZATIONS[realization].format)
                self.assertFalse([step for step in receipt['steps'] if 'child' in step])
                self.assertEqual(os.listdir(os.path.join(attempt, 'logs')), [])
                states = {step['name']: step['state'] for step in receipt['steps']}
                skipped = 'not-selected' if realization == 'image' else 'not-run'
                self.assertEqual((states['compile-tests'], states['run-tests']), (skipped, skipped))
                self.assertEqual(states['compile-producer'], 'not-run')

    def test_empty_compiler_argument_exits_2_before_the_output_dir(self):
        result = self.wrapper('--rustc', '', '--output-dir', os.path.join(self.base, 'attempt'))
        self.assertEqual(result.returncode, 2)
        self.assertIn('--rustc: expected a normalized absolute path', result.stderr)
        self.assertEqual(os.listdir(self.base), [])


if __name__ == '__main__':
    unittest.main()
