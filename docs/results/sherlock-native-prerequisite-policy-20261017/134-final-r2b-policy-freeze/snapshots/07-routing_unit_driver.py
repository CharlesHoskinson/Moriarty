#!/usr/bin/env python3
"""Ordinary synthetic host-containment unit test driver for R33 bounded_sdk_compile.py.

Derived narrowly from r5_mode120000_unit_driver.py (sha256=3c100445cb944ab85b4a9233a26fa7b398730bdc9d52f744afd1d4fd09d53604):
1. Preserves top-level `import unittest.mock` before installing any tripwires.
2. Verifies exact SHA-256 pins of the runner, 4 data files, and 5 test modules
   (4 retained modules [61 methods, keeping test_config_source.py byte-identical]
   + test_receipt_routing.py [8 methods] = 69 methods total), bound via
   SOURCE-FREEZE.json.
3. Installs the 17 baseline process/network/timer/rlimit HOST_SURFACES tripwires
   BEFORE loading bounded_sdk_compile.py (leaving ordinary synthetic filesystem
   fixture operations untripwired).
4. Installs 27 GUARD_SURFACES tripwires AFTER loading bounded_sdk_compile.py,
   leaving `guard.consent` live (while `guard.check_pin` remains tripwired by
   default and is patched inside test_receipt_routing.py) so `validate(c)` and
   `consent(c)` receipt-routing predicates are directly unit-tested without
   process, network, timer, rlimit, or unpatched guard effects.
"""
import argparse
import hashlib
import importlib.util
import json
import os
import pathlib
import resource
import signal
import socket
import subprocess
import sys
import unittest
import unittest.mock


GUARD = 'bounded_sdk_compile.py'
DATA = (
    'config.example.json',
    'config.schema.json',
    'decision-records.schema.json',
    'exact16_git_symlinks.json',
)
TESTS = (
    'test_config_source.py',
    'test_bounded_sdk_compile_link_fragment.py',
    'test_bounded_sdk_compile_symlink_config.py',
    'test_r5_walk_treehash_freeze.py',
    'test_receipt_routing.py',
)

IMMUTABLE_PINS = {
    'config.schema.json': 'a6496e818fab67cf59297e3d1593d9aabe1593b7f0d10e216b6a69a6c717f210',
    'config.example.json': '4b12357a4af85553be2ca833760d3bd4a08bb9ad4f5585823d63e800f0ff42cb',
    'exact16_git_symlinks.json': '47cd59df82333a8987d0be85b013793b8a524d2f5bdd33c257a989c97cc48a41',
    'test_config_source.py': '0f966d96932136e0762c18b80ba0ca0d13230079125850ce3c35e438a37e5e9d',
    'test_bounded_sdk_compile_link_fragment.py': '920983f6f18435eaf2a0f85a74853aa23dcabf76185c1369a1dad96361615739',
    'test_bounded_sdk_compile_symlink_config.py': '0d826a454f2bbf4d717e981d470d391916cd0d69f2b177f7f10fe98f8268a9a3',
    'test_r5_walk_treehash_freeze.py': 'f6b7b2d49e1f64053982a7cce4af471e9aac764e103482c794fc89a9f742d513',
}

HOST_SURFACES = (
    (subprocess, 'Popen', 'subprocess:Popen'),
    (os, 'system', 'os:system'),
    (os, 'fork', 'os:fork'),
    (os, 'forkpty', 'os:forkpty'),
    (os, 'posix_spawn', 'os:posix_spawn'),
    (os, 'posix_spawnp', 'os:posix_spawnp'),
    (os, 'execv', 'os:execv'),
    (os, 'execve', 'os:execve'),
    (os, 'execvp', 'os:execvp'),
    (os, 'execvpe', 'os:execvpe'),
    (os, 'memfd_create', 'os:memfd_create'),
    (socket, 'socket', 'socket:socket'),
    (socket, 'socketpair', 'socket:socketpair'),
    (signal, 'signal', 'signal:signal'),
    (signal, 'setitimer', 'signal:setitimer'),
    (signal, 'alarm', 'signal:alarm'),
    (resource, 'setrlimit', 'resource:setrlimit'),
)

GUARD_SURFACES = (
    'main',
    'run',
    'worker',
    'freeze_inputs',
    'stage',
    'verify_raw',
    'metadata_check',
    'output_check',
    'executing_identity',
    'bound_identities',
    'old_home_stats',
    'check_pin',
    'quotas',
    'check_tmpfs',
    'owned_paths',
    'write_new',
    'receipt',
    'sealed_memfd',
    'network_pending',
    '_cg_text',
    '_cg_children',
    '_cg_writable',
    '_cg_dirstat',
    '_cg_mkdir',
    '_cg_rmdir',
    '_cg_write',
    '_mount_rows',
)


def sha256_file(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tripwire(label):
    def _blocked(*_a, **_kw):
        raise RuntimeError(f'Forbidden surface called during unit test: {label}')
    return _blocked


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f'Cannot create module spec for {name} at {path}')
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


def resolve_expected_pins(src_dir, freeze_path):
    pins = dict(IMMUTABLE_PINS)
    if freeze_path is not None:
        raw = json.loads(pathlib.Path(freeze_path).read_text(encoding='utf-8'))
        freeze_map = raw.get('files', raw)
        if isinstance(freeze_map, list):
            freeze_map = {row['path']: row['sha256'] for row in freeze_map if 'path' in row and 'sha256' in row}
        for fname in (GUARD, 'decision-records.schema.json', 'test_receipt_routing.py'):
            if fname not in freeze_map:
                raise RuntimeError(f'Missing {fname} pin in {freeze_path}')
            pins[fname] = freeze_map[fname]
        for k, v in IMMUTABLE_PINS.items():
            if k in freeze_map and freeze_map[k] != v:
                raise RuntimeError(f'Immutable pin mismatch in {freeze_path} for {k}: {freeze_map[k]} != {v}')
    else:
        default_freeze = src_dir / 'SOURCE-FREEZE.json'
        if not default_freeze.is_file():
            raise RuntimeError('SOURCE-FREEZE.json is required to bind newly materialized R33 file pins')
        return resolve_expected_pins(src_dir, default_freeze)
    return pins


def main(argv=None):
    parser = argparse.ArgumentParser(description='R33 ordinary synthetic unit driver')
    parser.add_argument('--src-dir', default=str(pathlib.Path(__file__).resolve().parent))
    parser.add_argument('--source-freeze', default=None)
    args = parser.parse_args(argv)

    src_dir = pathlib.Path(args.src_dir).resolve()
    sys.path.insert(0, str(src_dir))
    expected_pins = resolve_expected_pins(src_dir, args.source_freeze)

    for fname in (GUARD, *DATA, *TESTS):
        fpath = src_dir / fname
        if not fpath.is_file():
            raise RuntimeError(f'Missing required file: {fpath}')
        actual_sha = sha256_file(fpath)
        if actual_sha != expected_pins[fname]:
            raise RuntimeError(f'Pin mismatch for {fname}: actual={actual_sha} expected={expected_pins[fname]}')

    for obj, attr, label in HOST_SURFACES:
        setattr(obj, attr, tripwire(label))

    guard = load_module('bounded_sdk_compile', src_dir / GUARD)

    for attr in GUARD_SURFACES:
        if not hasattr(guard, attr):
            raise RuntimeError(f'Guard module missing expected attribute: {attr}')
        setattr(guard, attr, tripwire(f'guard:{attr}'))

    suite = unittest.TestSuite()
    loader = unittest.defaultTestLoader
    loaded_counts = {}
    for tfile in TESTS:
        mod_name = tfile[:-3]
        mod = load_module(mod_name, src_dir / tfile)
        mod_suite = loader.loadTestsFromModule(mod)
        loaded_counts[tfile] = mod_suite.countTestCases()
        suite.addTest(mod_suite)

    runner = unittest.TextTestRunner(stream=sys.stdout, verbosity=2)
    result = runner.run(suite)
    summary = {
        'driver': 'routing_unit_driver.py',
        'host_surfaces_tripwired_before_guard_load': len(HOST_SURFACES),
        'guard_surfaces_tripwired_after_guard_load': len(GUARD_SURFACES),
        'module_method_counts': loaded_counts,
        'total_tests_run': result.testsRun,
        'failures': len(result.failures),
        'errors': len(result.errors),
        'skipped': len(result.skipped),
        'ok': result.wasSuccessful(),
    }
    print('ORDINARY_ROUTING_UNIT_SUMMARY_JSON=' + json.dumps(summary, sort_keys=True))
    return 0 if result.wasSuccessful() and result.testsRun == 69 else 1


if __name__ == '__main__':
    sys.exit(main())
