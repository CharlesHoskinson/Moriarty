#!/usr/bin/env python3
'''Independent literal tests for the four host_codec Data factories.

Contract: DATA-FACTORY-CONTRACT.md, clauses DF-01..DF-15. Author: claude-opus-5-5,
tools off, nothing executed. Standard library only.

The test reads only the regular, non-symlink sibling host_codec.py and executes exactly
the bytes it pinned. It writes nothing and uses no network, subprocess, RNG, VM, proof,
signature, SDK or private material.

Expected Data and CBOR bytes are literals taken from types.ak, transaction.ak, claim.ak,
core.ak, build_inputs.py and the serialise_data profile. No expected value is computed
by code under test, and SHA-256 is used only for the source pin. The suffix AST check is
best-effort and not a sandbox: root inspects both files in full before a run.

Run: python3 -I -B /abs/dir/test_data_factories.py
'''
import ast
import hashlib
import inspect
import os
import stat
import sys
import types
import unittest

sys.dont_write_bytecode = True

HOST_NAME = 'host_codec.py'
MAX_HOST_BYTES = 2 * 1024 * 1024
PREFIX_LEN = 19096
PREFIX_SHA256 = '8f59b28f8307c789dc1f4abe7251b0de661156af130650e9b358bceb668df377'
NEWLINE = bytes((10,))
PRIOR_APIS = ('encode_data', 'build_prove_request', 'validate_prove_request',
              'validate_context_input', 'parse_statement_header',
              'statement_digest_hex', 'statement_public_inputs')
FORBIDDEN_NAMES = frozenset((
    'eval', 'exec', 'compile', 'open', '__import__', 'breakpoint', 'input',
    'globals', 'locals', 'vars', 'getattr', 'setattr', 'delattr', '__builtins__'))

HEX = bytes.fromhex
U64 = 18446744073709551615
FAMILY = b'MRG16-PVC-FAM2-G16-BLS-ARK04-HL2'
TXID = HEX('22') * 32
P1 = HEX('10') * 28
P2 = HEX('20') * 28
NAME32 = HEX('ab') * 32
PD = HEX('a1') * 32
VKD = HEX('b2') * 32
VKH = HEX('c3') * 28
SD = HEX('e5') * 32
EXPIRY = 1700000060000

_CACHE = {}


class _Bytes(bytes):
    pass


class _Int(int):
    pass


class _Str(str):
    pass


class _List(list):
    pass


class _Tuple(tuple):
    pass


def _same(a, b):
    # Type-exact structural equality: ('int', True) never equals ('int', 1).
    if type(a) is not type(b):
        return False
    if type(a) is tuple or type(a) is list:
        return len(a) == len(b) and all(_same(x, y) for x, y in zip(a, b))
    return a == b


def _read_host():
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), HOST_NAME)
    try:
        before = os.lstat(path)
    except OSError:
        raise AssertionError('host_codec.py is missing beside the test') from None
    if not stat.S_ISREG(before.st_mode):
        raise AssertionError('host_codec.py must be a regular non-symlink file')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        after = os.fstat(fd)
        if (not stat.S_ISREG(after.st_mode) or after.st_dev != before.st_dev
                or after.st_ino != before.st_ino or after.st_size > MAX_HOST_BYTES):
            raise AssertionError('host_codec.py changed or exceeds the read bound')
        chunks = []
        total = 0
        while True:
            chunk = os.read(fd, 65536)
            if not chunk:
                break
            total += len(chunk)
            if total > MAX_HOST_BYTES:
                raise AssertionError('host_codec.py exceeds the read bound')
            chunks.append(chunk)
    finally:
        os.close(fd)
    return path, b''.join(chunks)


def _top_level_names(tree):
    names = set()
    for stmt in tree.body:
        if isinstance(stmt, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            names.add(stmt.name)
            continue
        for node in ast.walk(stmt):
            if isinstance(node, ast.Name) and isinstance(node.ctx, (ast.Store, ast.Del)):
                names.add(node.id)
            elif isinstance(node, (ast.Import, ast.ImportFrom)):
                names.update((alias.asname or alias.name).split('.')[0]
                             for alias in node.names)
            elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                names.add(node.name)
    return names


def _suffix_problem(data):
    try:
        prefix_tree = ast.parse(data[:PREFIX_LEN].decode('utf-8'))
        suffix_tree = ast.parse(data[PREFIX_LEN:].decode('utf-8'))
    except (SyntaxError, ValueError):
        return 'prefix and suffix must each parse as UTF-8 Python (DF-01)'
    for node in ast.walk(suffix_tree):
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            return 'suffix imports a module (DF-02 phase gate)'
        if isinstance(node, (ast.Global, ast.Nonlocal)):
            return 'suffix uses global or nonlocal (DF-01, DF-02)'
        if isinstance(node, ast.Name) and node.id in FORBIDDEN_NAMES:
            return 'suffix names a forbidden builtin (DF-02)'
        if isinstance(node, ast.Attribute) and node.attr.startswith('__'):
            return 'suffix uses dunder attribute access (DF-02)'
    if _top_level_names(prefix_tree) & _top_level_names(suffix_tree):
        return 'suffix rebinds a prefix top-level name (DF-01)'
    return None


def _load():
    if 'host' not in _CACHE:
        path, data = _read_host()
        if (len(data) < PREFIX_LEN
                or hashlib.sha256(data[:PREFIX_LEN]).hexdigest() != PREFIX_SHA256):
            raise AssertionError('host_codec.py lacks the pinned 19096-byte prefix')
        problem = _suffix_problem(data)
        if problem is not None:
            raise AssertionError(problem)
        module = types.ModuleType('host_codec')
        module.__file__ = path
        # Execute exactly the bytes pinned and inspected above; there is no second read.
        exec(compile(data, path, 'exec', dont_inherit=True), module.__dict__)
        _CACHE['host'] = (module, path, data)
    return _CACHE['host']


class DataFactoryTests(unittest.TestCase):

    def _api(self, name):
        module, path, data = _load()
        fn = module.__dict__.get(name)
        if not inspect.isfunction(fn):
            self.fail('missing implementation: ' + name)
        code = fn.__code__
        if code.co_filename != path or code.co_firstlineno <= data[:PREFIX_LEN].count(NEWLINE):
            self.fail(name + ' is not defined in the appended suffix (DF-01)')
        return fn

    def _prior(self, name):
        module = _load()[0]
        fn = module.__dict__.get(name)
        self.assertTrue(inspect.isfunction(fn), 'prior API absent: ' + name)
        return fn

    def _signature(self, fn, names, defaults):
        params = list(inspect.signature(fn).parameters.values())
        self.assertEqual([p.name for p in params], list(names))
        for p in params:
            self.assertIs(p.kind, inspect.Parameter.POSITIONAL_OR_KEYWORD, p.name)
            if p.name in defaults:
                self.assertIs(p.default, defaults[p.name], p.name)
            else:
                self.assertIs(p.default, inspect.Parameter.empty, p.name)

    def _assert_data(self, got, want):
        if not _same(got, want):
            self.fail('Data differs from the literal expectation: ' + repr(got)[:400])

    def _refuses(self, prefix, fn, *args, **kwargs):
        try:
            fn(*args, **kwargs)
        except ValueError as exc:
            message = str(exc)
            self.assertTrue(message.startswith(prefix),
                            'expected refusal prefix ' + prefix + ' got ' + message[:200])
            return message
        except Exception as exc:
            self.fail('refusal raised ' + type(exc).__name__ + ', not ValueError')
        self.fail('input accepted; expected refusal ' + prefix)

    def test_00_host_prefix_pin_and_suffix_inspection(self):
        path, data = _read_host()
        self.assertGreaterEqual(len(data), PREFIX_LEN)
        self.assertEqual(hashlib.sha256(data[:PREFIX_LEN]).hexdigest(), PREFIX_SHA256)
        self.assertIsNone(_suffix_problem(data))
        prefix_names = _top_level_names(ast.parse(data[:PREFIX_LEN].decode('utf-8')))
        for name in PRIOR_APIS:
            self.assertIn(name, prefix_names)
        module, loaded_path, loaded = _load()
        self.assertEqual(loaded_path, path)
        self.assertEqual(loaded, data)
        prefix_lines = data[:PREFIX_LEN].count(NEWLINE)
        for name in PRIOR_APIS:
            fn = module.__dict__.get(name)
            self.assertTrue(inspect.isfunction(fn), name)
            self.assertEqual(fn.__code__.co_filename, path)
            self.assertLessEqual(fn.__code__.co_firstlineno, prefix_lines, name)

    def test_01_value_positive_cases(self):
        build = self._api('build_value_data')
        encode = self._prior('encode_data')
        self._signature(build, ('lovelace', 'tokens'), {})
        cases = (
            ('ada only', 1000000, [],
             ('map', [(('bytes', b''), ('map', [(('bytes', b''), ('int', 1000000))]))]),
             HEX('a140a1401a000f4240')),
            ('empty value', None, [], ('map', []), HEX('a0')),
            ('ada and one token', 2000000, [(P1, NAME32, 1)],
             ('map', [(('bytes', b''), ('map', [(('bytes', b''), ('int', 2000000))])),
                      (('bytes', P1), ('map', [(('bytes', NAME32), ('int', 1))]))]),
             HEX('a240a1401a001e8480581c') + P1 + HEX('a15820') + NAME32 + HEX('01')),
            ('mint burn without ada', None, [(P1, NAME32, -1)],
             ('map', [(('bytes', P1), ('map', [(('bytes', NAME32), ('int', -1))]))]),
             HEX('a1581c') + P1 + HEX('a15820') + NAME32 + HEX('20')),
            ('zero ada and grouped policies', 0, [(P1, b'', 5), (P1, b'a', 7), (P2, b'z', 1)],
             ('map', [(('bytes', b''), ('map', [(('bytes', b''), ('int', 0))])),
                      (('bytes', P1), ('map', [(('bytes', b''), ('int', 5)),
                                               (('bytes', b'a'), ('int', 7))])),
                      (('bytes', P2), ('map', [(('bytes', b'z'), ('int', 1))]))]),
             HEX('a340a14000581c') + P1 + HEX('a24005416107581c') + P2 + HEX('a1417a01')),
        )
        for name, lovelace, tokens, want, encoded in cases:
            with self.subTest(name):
                got = build(lovelace, tokens)
                self._assert_data(got, want)
                self.assertEqual(encode(got), encoded)
        self._assert_data(build(tokens=[], lovelace=1000000), cases[0][3])

    def test_02_value_refuses_types_and_ranges(self):
        build = self._api('build_value_data')
        good = [(P1, b'a', 1)]
        groups = (
            ('lovelace:', lambda bad: (bad, good),
             (True, False, 1.0, '1', b'1', -1, U64 + 1, _Int(1), [1])),
            ('tokens:', lambda bad: (1, bad),
             (None, (), ((P1, b'a', 1),), {}, 'x', b'', _List(good))),
            ('tokens[]:', lambda bad: (1, [bad]),
             ([P1, b'a', 1], (P1, b'a'), (P1, b'a', 1, 0), (), None, _Tuple((P1, b'a', 1)))),
            ('tokens[].policy:', lambda bad: (1, [(bad, b'a', 1)]),
             (bytearray(P1), memoryview(P1), P1.hex(), HEX('10') * 27, HEX('10') * 29, b'',
              None, _Bytes(P1), 0)),
            ('tokens[].name:', lambda bad: (1, [(P1, bad, 1)]),
             (bytearray(b'a'), memoryview(b'a'), 'a', bytes(33), None, _Bytes(b'a'), 0)),
            ('tokens[].quantity:', lambda bad: (1, [(P1, b'a', bad)]),
             (True, False, 0, 1.0, '1', U64 + 1, -U64 - 1, None, _Int(1), b'1')),
        )
        for prefix, make, values in groups:
            for index, bad in enumerate(values):
                with self.subTest(prefix=prefix, case=index):
                    self._refuses(prefix, build, *make(bad))

    def test_03_value_order_count_and_refusal_order(self):
        build = self._api('build_value_data')
        unordered = (
            ('descending policy', [(P2, b'a', 1), (P1, b'a', 1)]),
            ('descending name', [(P1, b'b', 1), (P1, b'a', 1)]),
            ('duplicate pair', [(P1, b'a', 1), (P1, b'a', 2)]),
            ('extension before prefix', [(P1, b'a' + bytes(1), 1), (P1, b'a', 1)]),
            ('policy regrouping', [(P1, b'a', 1), (P2, b'a', 1), (P1, b'b', 1)]),
            ('signed-byte order', [(P1, HEX('80'), 1), (P1, HEX('7f'), 1)]),
        )
        for name, tokens in unordered:
            with self.subTest(name):
                self._refuses('tokens.order:', build, None, tokens)
        over = [(P1, b'', 1)] + [(P1, bytes((i,)), 1) for i in range(256)]
        self.assertEqual(len(over), 257)
        self._refuses('tokens:', build, None, over)
        self._refuses('tokens:', build, None, [None] + over[1:])
        checks = (
            ('lovelace:', (-1, 'not-a-list')),
            ('lovelace:', (True, [None])),
            ('tokens[].policy:', (None, [(b'', bytes(33), 0)])),
            ('tokens[].name:', (None, [(P1, 'a', 0)])),
            ('tokens[].policy:', (None, [(b'short', b'a', 1), (P1, None, 1)])),
            ('tokens[]:', (None, [(P1, b'a', 1), (P1, b'b'), (b'', b'c', 1)])),
            ('tokens[].quantity:', (None, [(P2, b'a', 1), (P1, b'a', 0)])),
            ('tokens.order:', (None, [(P2, b'a', 1), (P1, b'a', 1), None])),
        )
        for index, (prefix, args) in enumerate(checks):
            with self.subTest(order_case=index):
                self._refuses(prefix, build, *args)

    def test_04_value_accepted_boundaries(self):
        build = self._api('build_value_data')
        encode = self._prior('encode_data')
        got = build(U64, [])
        self._assert_data(got, ('map', [(('bytes', b''), ('map', [
            (('bytes', b''), ('int', 18446744073709551615))]))]))
        self.assertEqual(encode(got), HEX('a140a1401b') + HEX('ff') * 8)
        got = build(None, [(P1, b'', U64), (P1, NAME32, -U64)])
        self._assert_data(got, ('map', [(('bytes', P1), ('map', [
            (('bytes', b''), ('int', 18446744073709551615)),
            (('bytes', NAME32), ('int', -18446744073709551615))]))]))
        self.assertEqual(encode(got), (
            HEX('a1581c') + P1 + HEX('a2401b') + HEX('ff') * 8 + HEX('5820') + NAME32
            + HEX('3b') + HEX('ff') * 7 + HEX('fe')))
        names = [b'', b'a', b'a' + bytes(1), b'b', HEX('7f'), HEX('80')]
        got = build(None, [(P1, name, 1) for name in names])
        self._assert_data(got, ('map', [(('bytes', P1), ('map', [
            (('bytes', b''), ('int', 1)), (('bytes', b'a'), ('int', 1)),
            (('bytes', b'a' + bytes(1)), ('int', 1)), (('bytes', b'b'), ('int', 1)),
            (('bytes', HEX('7f')), ('int', 1)), (('bytes', HEX('80')), ('int', 1))]))]))
        many = [(P1, bytes((i,)), 1) for i in range(256)]
        got = build(None, many)
        self._assert_data(got, ('map', [(('bytes', P1), ('map', [
            (('bytes', bytes((i,))), ('int', 1)) for i in range(256)]))]))
        self.assertEqual(encode(got), (
            HEX('a1581c') + P1 + HEX('b90100')
            + b''.join(bytes((0x41, i, 0x01)) for i in range(256))))

    def test_05_output_reference_positive_cases(self):
        build = self._api('build_output_reference')
        encode = self._prior('encode_data')
        self._signature(build, ('transaction_id', 'output_index'), {})
        for index, tail in ((0, '00'), (3, '03'), (23, '17'), (24, '1818'), (255, '18ff'),
                            (256, '190100'), (65535, '19ffff')):
            with self.subTest(output_index=index):
                got = build(TXID, index)
                self._assert_data(got, ('constr', 0, [('bytes', TXID), ('int', index)]))
                self.assertEqual(encode(got), HEX('d8799f5820') + TXID + HEX(tail + 'ff'))
        got = build(output_index=1, transaction_id=bytes(32))
        self._assert_data(got, ('constr', 0, [('bytes', bytes(32)), ('int', 1)]))
        self.assertEqual(encode(got), HEX('d8799f5820') + bytes(32) + HEX('01ff'))

    def test_06_output_reference_refusals(self):
        build = self._api('build_output_reference')
        bad_ids = (bytearray(TXID), memoryview(TXID), TXID.hex(), HEX('22') * 31,
                   HEX('22') * 33, b'', None, _Bytes(TXID), list(TXID), 0)
        for index, bad in enumerate(bad_ids):
            with self.subTest(transaction_id=index):
                self._refuses('transaction_id:', build, bad, 0)
        bad_indexes = (True, False, -1, 65536, U64, 1.0, '0', None, _Int(0), b'0')
        for index, bad in enumerate(bad_indexes):
            with self.subTest(output_index=index):
                self._refuses('output_index:', build, TXID, bad)
        self._refuses('transaction_id:', build, b'', -1)
        self._refuses('transaction_id:', build, 'x', True)
        self._refuses('transaction_id:', build, TXID[:28], 70000)

    def test_07_env_positive_cases(self):
        build = self._api('build_env_data')
        encode = self._prior('encode_data')
        self._signature(build, ('predicate_digest', 'vk_digest', 'claimant_vkh',
                                'settle_transaction_id', 'settle_output_index',
                                'statement_digest', 'expiry'), {})
        self.assertEqual(len(FAMILY), 32)
        got = build(PD, VKD, VKH, TXID, 0, SD, EXPIRY)
        self._assert_data(got, ('constr', 0, [
            ('int', 2), ('int', 0), ('bytes', b'MRG16-PVC-FAM2-G16-BLS-ARK04-HL2'),
            ('bytes', PD), ('bytes', VKD), ('bytes', VKH),
            ('constr', 0, [('bytes', TXID), ('int', 0)]),
            ('bytes', SD), ('int', 1700000060000)]))
        self.assertEqual(encode(got), (
            HEX('d8799f02005820') + FAMILY + HEX('5820') + PD + HEX('5820') + VKD
            + HEX('581c') + VKH + HEX('d8799f5820') + TXID + HEX('00ff5820') + SD
            + HEX('1b0000018bcfe65260ff')))
        got = build(predicate_digest=PD, vk_digest=VKD, claimant_vkh=VKH,
                    settle_transaction_id=TXID, settle_output_index=24,
                    statement_digest=SD, expiry=0)
        self._assert_data(got, ('constr', 0, [
            ('int', 2), ('int', 0), ('bytes', FAMILY), ('bytes', PD), ('bytes', VKD),
            ('bytes', VKH), ('constr', 0, [('bytes', TXID), ('int', 24)]),
            ('bytes', SD), ('int', 0)]))
        self.assertEqual(encode(got), (
            HEX('d8799f02005820') + FAMILY + HEX('5820') + PD + HEX('5820') + VKD
            + HEX('581c') + VKH + HEX('d8799f5820') + TXID + HEX('1818ff5820') + SD
            + HEX('00ff')))

    def test_08_env_field_refusals(self):
        build = self._api('build_env_data')
        base = {'predicate_digest': PD, 'vk_digest': VKD, 'claimant_vkh': VKH,
                'settle_transaction_id': TXID, 'settle_output_index': 0,
                'statement_digest': SD, 'expiry': 0}
        sized = (('predicate_digest', 32), ('vk_digest', 32), ('claimant_vkh', 28),
                 ('settle_transaction_id', 32), ('statement_digest', 32))
        for field, size in sized:
            bads = (bytes(size - 1), bytes(size + 1), b'', bytearray(size),
                    memoryview(bytes(size)), '00' * size, None, _Bytes(bytes(size)), size)
            for index, bad in enumerate(bads):
                with self.subTest(field=field, case=index):
                    args = dict(base)
                    args[field] = bad
                    self._refuses(field + ':', build, **args)
        ints = (('settle_output_index', (-1, 65536, U64, True, False, 1.0, '0', None, _Int(0))),
                ('expiry', (-1, U64 + 1, -U64, True, 1.0, '0', None, _Int(0), b'')))
        for field, bads in ints:
            for index, bad in enumerate(bads):
                with self.subTest(field=field, case=index):
                    args = dict(base)
                    args[field] = bad
                    self._refuses(field + ':', build, **args)
        self._refuses('vk_digest:', build, PD, VKH, VKH, TXID, 0, SD, 0)
        self._refuses('claimant_vkh:', build, PD, VKD, PD, TXID, 0, SD, 0)
        self._refuses('settle_transaction_id:', build, PD, VKD, VKH, VKH, 0, SD, 0)

    def test_09_env_refusal_order_and_accepted_boundaries(self):
        build = self._api('build_env_data')
        encode = self._prior('encode_data')
        fields = ('predicate_digest', 'vk_digest', 'claimant_vkh', 'settle_transaction_id',
                  'settle_output_index', 'statement_digest', 'expiry')
        good = (PD, VKD, VKH, TXID, 0, SD, 0)
        broken = (b'', b'', b'', b'', -1, b'', -1)
        for first in range(len(fields)):
            for second in range(first + 1, len(fields)):
                with self.subTest(first=fields[first], second=fields[second]):
                    args = list(good)
                    args[first] = broken[first]
                    args[second] = broken[second]
                    self._refuses(fields[first] + ':', build, *args)
        self._refuses('predicate_digest:', build, *broken)
        got = build(FAMILY, bytes(32), VKH, TXID, 65535, HEX('ff') * 32, U64)
        self._assert_data(got, ('constr', 0, [
            ('int', 2), ('int', 0), ('bytes', FAMILY), ('bytes', FAMILY), ('bytes', bytes(32)),
            ('bytes', VKH), ('constr', 0, [('bytes', TXID), ('int', 65535)]),
            ('bytes', HEX('ff') * 32), ('int', 18446744073709551615)]))
        self.assertEqual(encode(got), (
            HEX('d8799f02005820') + FAMILY + HEX('5820') + FAMILY + HEX('5820') + bytes(32)
            + HEX('581c') + VKH + HEX('d8799f5820') + TXID + HEX('19ffffff5820')
            + HEX('ff') * 32 + HEX('1b') + HEX('ff') * 9))

    def test_10_c2_mint_positive_cases(self):
        mint = self._api('build_c2_mint_data')
        encode = self._prior('encode_data')
        self._signature(mint, ('kind', 'vk', 'proof'), {'vk': None, 'proof': None})
        calls = (('positional', lambda: mint('Burn')),
                 ('explicit None', lambda: mint('Burn', None, None)),
                 ('keywords', lambda: mint(kind='Burn', vk=None, proof=None)))
        for name, call in calls:
            with self.subTest(name):
                got = call()
                self._assert_data(got, ('constr', 1, []))
                self.assertEqual(encode(got), HEX('d87a80'))
        got = mint('Issue', HEX('0102'), HEX('03'))
        self._assert_data(got, ('constr', 0, [('bytes', HEX('0102')), ('bytes', HEX('03'))]))
        self.assertEqual(encode(got), HEX('d8799f4201024103ff'))
        got = mint(kind='Issue', proof=HEX('02') * 192, vk=HEX('01') * 488)
        self._assert_data(got, ('constr', 0, [('bytes', HEX('01') * 488),
                                             ('bytes', HEX('02') * 192)]))
        self.assertEqual(encode(got), (
            HEX('d8799f5f') + (HEX('5840') + HEX('01') * 64) * 7 + HEX('5828') + HEX('01') * 40
            + HEX('ff5f') + (HEX('5840') + HEX('02') * 64) * 3 + HEX('ffff')))
        got = mint('Issue', bytes(1), HEX('ff') * 65536)
        self._assert_data(got, ('constr', 0, [('bytes', bytes(1)), ('bytes', HEX('ff') * 65536)]))
        self.assertEqual(encode(got), (
            HEX('d8799f41005f') + (HEX('5840') + HEX('ff') * 64) * 1024 + HEX('ffff')))

    def test_11_c2_mint_refusals(self):
        mint = self._api('build_c2_mint_data')
        bad_kinds = ('issue', 'burn', 'ISSUE', 'Issue ', ' Burn', '', b'Issue', None, 0, True,
                     _Str('Issue'), ['Burn'])
        for index, bad in enumerate(bad_kinds):
            with self.subTest(kind=index):
                self._refuses('kind:', mint, bad, b'v', b'p')
        bad_payloads = (None, b'', bytearray(b'v'), memoryview(b'v'), 'v', 'ff', _Bytes(b'v'),
                        bytes(65537), 1, [1])
        for index, bad in enumerate(bad_payloads):
            with self.subTest(issue_vk=index):
                self._refuses('vk:', mint, 'Issue', bad, b'p')
            with self.subTest(issue_proof=index):
                self._refuses('proof:', mint, 'Issue', b'v', bad)
        for index, bad in enumerate((b'', b'v', bytearray(), 0, False, '')):
            with self.subTest(burn_vk=index):
                self._refuses('vk:', mint, 'Burn', bad)
                self._refuses('vk:', mint, 'Burn', vk=bad)
            with self.subTest(burn_proof=index):
                self._refuses('proof:', mint, 'Burn', None, bad)
                self._refuses('proof:', mint, 'Burn', proof=bad)
        self._refuses('kind:', mint, 'Mint', None, None)
        self._refuses('kind:', mint, 'Mint')
        self._refuses('vk:', mint, 'Issue', b'', b'')
        self._refuses('vk:', mint, 'Issue')
        self._refuses('proof:', mint, 'Issue', b'v')
        self._refuses('vk:', mint, 'Burn', b'v', b'p')

    def test_12_cross_factory_purity_detachment_and_fixed_messages(self):
        value = self._api('build_value_data')
        outref = self._api('build_output_reference')
        env = self._api('build_env_data')
        mint = self._api('build_c2_mint_data')
        env_want = ('constr', 0, [
            ('int', 2), ('int', 0), ('bytes', FAMILY), ('bytes', PD), ('bytes', VKD),
            ('bytes', VKH), ('constr', 0, [('bytes', TXID), ('int', 1)]), ('bytes', SD),
            ('int', 2)])
        with self.subTest('fresh containers per call (DF-03)'):
            first = mint('Burn')
            first[2].append(('int', 9))
            self._assert_data(mint('Burn'), ('constr', 1, []))
            first = mint('Issue', b'v', b'p')
            first[2].reverse()
            self._assert_data(mint('Issue', b'v', b'p'),
                              ('constr', 0, [('bytes', b'v'), ('bytes', b'p')]))
            first = value(None, [])
            first[1].append((('bytes', b''), ('int', 0)))
            self._assert_data(value(None, []), ('map', []))
            first = value(1, [])
            first[1][0][1][1].clear()
            self._assert_data(value(1, []), ('map', [(('bytes', b''), ('map', [
                (('bytes', b''), ('int', 1))]))]))
            first = outref(TXID, 7)
            first[2].clear()
            self._assert_data(outref(TXID, 7), ('constr', 0, [('bytes', TXID), ('int', 7)]))
            first = env(PD, VKD, VKH, TXID, 1, SD, 2)
            first[2][6][2].clear()
            first[2].clear()
            self._assert_data(env(PD, VKD, VKH, TXID, 1, SD, 2), env_want)
        with self.subTest('arguments neither mutated nor aliased (DF-02, DF-03)'):
            tokens = [(P1, b'a', 1)]
            entry = tokens[0]
            got = value(None, tokens)
            self.assertEqual(len(tokens), 1)
            self.assertIs(tokens[0], entry)
            self.assertIsNot(got[1], tokens)
            tokens.append((P2, b'b', 1))
            self._assert_data(got, ('map', [(('bytes', P1), ('map', [
                (('bytes', b'a'), ('int', 1))]))]))
        with self.subTest('deterministic (DF-02)'):
            self._assert_data(env(PD, VKD, VKH, TXID, 1, SD, 2),
                              env(PD, VKD, VKH, TXID, 1, SD, 2))
        with self.subTest('structure only, no digest, VK or proof validation (DF-11)'):
            self._assert_data(mint('Issue', HEX('ff'), HEX('ff')),
                              ('constr', 0, [('bytes', HEX('ff')), ('bytes', HEX('ff'))]))
            zero = bytes(32)
            self._assert_data(env(zero, zero, bytes(28), zero, 0, zero, 0), ('constr', 0, [
                ('int', 2), ('int', 0), ('bytes', FAMILY), ('bytes', zero), ('bytes', zero),
                ('bytes', bytes(28)), ('constr', 0, [('bytes', zero), ('int', 0)]),
                ('bytes', zero), ('int', 0)]))
        with self.subTest('refusal messages never echo arguments (DF-05)'):
            marker = b'MARKER-7f3a'
            messages = [
                self._refuses('transaction_id:', outref, marker * 3, 0),
                self._refuses('tokens[].name:', value, None, [(P1, marker * 4, 1)]),
                self._refuses('lovelace:', value, 987654321987654321987654321, []),
                self._refuses('expiry:', env, PD, VKD, VKH, TXID, 0, SD, -424242424242),
                self._refuses('kind:', mint, 'MARKER-7f3a'),
                self._refuses('claimant_vkh:', env, PD, VKD, marker * 3, TXID, 0, SD, 0),
            ]
            for message in messages:
                for needle in ('MARKER', '4d41524b4552', '987654321', '424242'):
                    self.assertNotIn(needle, message)


if __name__ == '__main__':
    unittest.main(verbosity=2)
