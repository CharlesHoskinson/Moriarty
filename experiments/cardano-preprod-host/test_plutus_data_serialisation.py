'''Frozen byte contract for host_codec.encode_data (Cardano serialise_data profile).

Expected values are hand-derived literals from the pinned primary sources in
PLUTUS-DATA-SOURCE-CONTRACT.md; none is produced by an encoder and string
repetition is notation for fixed public bytes only. Structural values are
public shape literals, not keys, proofs, signatures or ledger data; no proof
or key is validated. Big integers are data, not cryptographic arithmetic.
Standard library only: no network, subprocess, file write, entropy, SDK,
native code, mock, fallback encoder or evaluation of remote data.

Run: python3 -I -B /absolute/path/test_plutus_data_serialisation.py
'''

import ast
import inspect
import stat
import sys
import unittest
from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path

sys.dont_write_bytecode = True

HERE = Path(__file__).resolve().parent
TARGET = HERE / 'host_codec.py'
FORBIDDEN_NAMES = frozenset(
    ('eval', 'exec', 'compile', 'open', '__import__', 'breakpoint', 'input'))
FUTURE_APIS = (
    'build_prove_request', 'validate_prove_request', 'validate_context_input',
    'build_value_data', 'build_output_reference', 'build_env_data',
    'build_c2_mint_data', 'parse_statement_header', 'statement_digest_hex',
    'statement_public_inputs')
_LOADED = []


def load_host_codec(case):
    '''Audit, then load the exact regular sibling host_codec.py; no fallback.'''
    try:
        mode = TARGET.lstat().st_mode
    except FileNotFoundError:
        case.fail('missing implementation: expected regular sibling %s' % TARGET)
    case.assertTrue(stat.S_ISREG(mode), 'host_codec.py must be a regular non-symlink file')
    source = TARGET.read_bytes()
    for node in ast.walk(ast.parse(source, filename=str(TARGET))):
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            # Phase gate (I1-SCOPE-R2): __future__, or exactly `import hashlib` for the
            # statement digest APIs of host_codec.py; every other import is refused.
            case.assertTrue(
                (isinstance(node, ast.ImportFrom) and node.module == '__future__')
                or (isinstance(node, ast.Import) and len(node.names) == 1
                    and node.names[0].name == 'hashlib'
                    and node.names[0].asname is None),
                'import audit: host_codec.py line %d imports a module' % node.lineno)
        elif isinstance(node, ast.Name):
            case.assertNotIn(node.id, FORBIDDEN_NAMES, 'import audit: forbidden name')
    spec = spec_from_file_location('host_codec', str(TARGET))
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    case.assertEqual(Path(module.__file__).resolve(), TARGET)
    case.assertEqual(TARGET.read_bytes(), source, 'host_codec.py changed during load')
    _LOADED.append(module)
    return module


SEQ64 = ('000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f'
         '202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f')
CHUNK_00 = '5840' + '00' * 64
CHUNK_FF = '5840' + 'ff' * 64
FALSE = ('constr', 0, [])
TRUE = ('constr', 1, [])
# Public opaque 488/192-byte shape strings: length literals only, never a VK or proof.
VK_SHAPE = bytes.fromhex('aa' * 488)
PROOF_SHAPE = bytes.fromhex('bb' * 192)
VK_SHAPE_CBOR = '5f' + ('5840' + 'aa' * 64) * 7 + '5828' + 'aa' * 40 + 'ff'
PROOF_SHAPE_CBOR = '5f' + ('5840' + 'bb' * 64) * 3 + 'ff'
TXID_SHAPE = bytes.fromhex('77' * 32)

INT_ROWS = tuple((('int', v), h) for v, h in (
    (0, '00'), (1, '01'), (10, '0a'), (23, '17'), (24, '1818'), (109, '186d'),
    (255, '18ff'), (256, '190100'), (65535, '19ffff'), (65536, '1a00010000'),
    (4294967295, '1affffffff'), (4294967296, '1b0000000100000000'),
    (18446744073709551615, '1bffffffffffffffff'),
    (-1, '20'), (-24, '37'), (-25, '3818'), (-256, '38ff'), (-257, '390100'),
    (-65536, '39ffff'), (-65537, '3a00010000'), (-4294967296, '3affffffff'),
    (-4294967297, '3b0000000100000000'),
    (-18446744073709551616, '3bffffffffffffffff'),
))

BIGNUM_ROWS = tuple((('int', v), h) for v, h in (
    (2 ** 64, 'c24901' + '00' * 8),
    (2 ** 64 + 1, 'c24901' + '00' * 7 + '01'),
    (2 ** 72 - 1, 'c249' + 'ff' * 9),
    (2 ** 128, 'c25101' + '00' * 16),
    (2 ** 512 - 1, 'c2' + CHUNK_FF),
    (2 ** 512, 'c25f584001' + '00' * 63 + '4100ff'),
    (-(2 ** 64) - 1, 'c34901' + '00' * 8),
    (-(2 ** 64) - 2, 'c34901' + '00' * 7 + '01'),
    (-(2 ** 128) - 1, 'c35101' + '00' * 16),
    (-(2 ** 512), 'c3' + CHUNK_FF),
    (-(2 ** 512) - 1, 'c35f584001' + '00' * 63 + '4100ff'),
))

BYTES_ROWS = tuple((('bytes', bytes.fromhex(v)), h) for v, h in (
    ('', '40'), ('01', '4101'), ('0102', '420102'),
    ('ab' * 23, '57' + 'ab' * 23), ('ab' * 24, '5818' + 'ab' * 24),
    (SEQ64, '5840' + SEQ64),
    (SEQ64 + '40', '5f5840' + SEQ64 + '4140ff'),
    (SEQ64 + 'cd' * 64, '5f5840' + SEQ64 + '5840' + 'cd' * 64 + 'ff'),
    (SEQ64 * 2 + 'ef', '5f5840' + SEQ64 + '5840' + SEQ64 + '41efff'),
    ('aa' * 488, VK_SHAPE_CBOR), ('bb' * 192, PROOF_SHAPE_CBOR),
))

LIST_ROWS = (
    (('list', []), '80'),
    (('list', [('int', 1), ('int', 2)]), '9f0102ff'),
    (('list', [('list', [])]), '9f80ff'),
    (('list', [('list', [('int', 1)])]), '9f9f01ffff'),
    (('list', [('map', []), ('bytes', b'')]), '9fa040ff'),
    (('list', [('int', 0)] * 24), '9f' + '00' * 24 + 'ff'),
)

MAP_ROWS = (
    (('map', []), 'a0'),
    (('map', [(('int', 1), ('int', 2))]), 'a10102'),
    (('map', [(('int', 2), ('int', 20)), (('int', 1), ('int', 10))]), 'a20214010a'),
    (('map', [(('int', 1), ('int', 1)), (('int', 1), ('int', 2))]), 'a201010102'),
    (('map', [(('bytes', b''), ('int', 0)), (('int', 0), ('bytes', b''))]), 'a240000040'),
    (('map', [(('bytes', bytes.fromhex('ab')), ('list', [('int', 1)]))]), 'a141ab9f01ff'),
    (('map', [(('list', []), ('map', []))]), 'a180a0'),
    (('map', [(('int', i), ('int', 0)) for i in range(24)]),
     'b818' '0000010002000300040005000600070008000900'
     '0a000b000c000d000e000f00' '10001100120013001400150016001700'),
)

CONSTR_ROWS = (
    (FALSE, 'd87980'), (TRUE, 'd87a80'), (('constr', 6, []), 'd87f80'),
    (('constr', 7, []), 'd9050080'), (('constr', 8, []), 'd9050180'),
    (('constr', 127, []), 'd9057880'), (('constr', 128, []), 'd86682188080'),
    (('constr', 255, []), 'd8668218ff80'), (('constr', 256, []), 'd8668219010080'),
    (('constr', 2 ** 64 - 1, []), 'd866821bffffffffffffffff80'),
    (('constr', 0, [('int', 1)]), 'd8799f01ff'),
    (('constr', 6, [('int', 1), ('int', 2)]), 'd87f9f0102ff'),
    (('constr', 7, [('int', 1)]), 'd905009f01ff'),
    (('constr', 127, [('bytes', bytes.fromhex('01'))]), 'd905789f4101ff'),
    (('constr', 128, [('int', 1)]), 'd8668218809f01ff'),
    (('constr', 0, [('map', [])]), 'd8799fa0ff'),
    (('constr', 2, [('list', [('int', -1), ('bytes', b'')]),
                    ('map', [(('int', 0), FALSE)])]), 'd87b9f9f2040ffa100d87980ff'),
)

SHAPE_ROWS = (
    (('constr', 1, [TRUE]), 'd87a9fd87a80ff'),
    (('constr', 0, [FALSE]), 'd8799fd87980ff'),
    (('constr', 0, [('int', 109)]), 'd8799f186dff'),
    (('constr', 0, [('bytes', TXID_SHAPE), ('int', 3)]), 'd8799f5820' + '77' * 32 + '03ff'),
    (('constr', 0, [('bytes', VK_SHAPE), ('bytes', PROOF_SHAPE)]),
     'd8799f' + VK_SHAPE_CBOR + PROOF_SHAPE_CBOR + 'ff'),
    (('constr', 0, [('map', [(('bytes', bytes.fromhex('6b')), ('list', [('int', -5)]))])]),
     'd8799fa1416b9f24ffff'),
)

MALFORMED = (
    None, True, 0, b'', 'int', ['int', 1], (), ('int',), ('int', 1, 2),
    ('Int', 1), ('integer', 1), ('text', 'x'), (b'int', 1), ({}, 1), ('bytes',),
    ('list',), ('map',), ('constr', 0), ('constr', 0, [], []),
    ('tag', 2, ('bytes', b'')),
    ('int', True), ('int', False), ('int', 1.0), ('int', '1'), ('int', None),
    ('bytes', bytearray(b'a')), ('bytes', memoryview(b'a')), ('bytes', 'ab'),
    ('bytes', [1]), ('bytes', None),
    ('list', ()), ('list', None), ('list', ('int', 1)), ('list', [1]),
    ('list', [True]), ('list', [('int', True)]), ('list', [['int', 1]]),
    ('map', {}), ('map', ()), ('map', None), ('map', [(('int', 1),)]),
    ('map', [(('int', 1), ('int', 2), ('int', 3))]),
    ('map', [[('int', 1), ('int', 2)]]), ('map', [(('int', 1), 2)]),
    ('map', [(1, ('int', 2))]), ('map', [(('int', 1), ('int', False))]),
    ('constr', -1, []), ('constr', 2 ** 64, []), ('constr', True, []),
    ('constr', False, []), ('constr', 1.0, []), ('constr', '0', []),
    ('constr', None, []), ('constr', 0, ()), ('constr', 0, None),
    ('constr', 0, [5]), ('constr', 0, [('int', True)]),
    ('constr', 0, [('constr', -1, [])]),
)


# Inert private subclasses for A1 exactness rows; they define no hooks.
class _Str(str):
    pass


class _Int(int):
    pass


class _Bytes(bytes):
    pass


class _List(list):
    pass


class _Tuple(tuple):
    pass


SUBCLASS_ROWS = (
    (_Str('int'), 0), ('int', _Int(1)), ('bytes', _Bytes(b'a')), ('list', _List()),
    _Tuple(('int', 1)), ('constr', _Int(0), []), ('constr', 0, _List()),
    ('map', _List()), ('map', [_Tuple((('int', 0), ('int', 0)))]),
    ('list', [_Tuple(('int', 1))]),
)


def nest(depth, wrap, leaf):
    node = leaf
    for _ in range(depth - 1):
        node = wrap(node)
    return node


def as_list(node):
    return ('list', [node])


def as_constr(node):
    return ('constr', 0, [node])


def as_map_value(node):
    return ('map', [(('int', 0), node)])


class CodecCase(unittest.TestCase):
    def setUp(self):
        self.codec = _LOADED[0] if _LOADED else load_host_codec(self)

    def encode(self, node):
        out = self.codec.encode_data(node)
        self.assertIs(type(out), bytes)
        return out

    def check(self, rows):
        for index, (node, expected) in enumerate(rows):
            with self.subTest(index=index):
                self.assertEqual(self.encode(node).hex(), expected)

    def rejects(self, nodes):
        for index, node in enumerate(nodes):
            with self.subTest(index=index):
                with self.assertRaises(ValueError):
                    self.codec.encode_data(node)


class TestInterfaceAndScope(CodecCase):
    def test_exact_signature_and_return_type(self):
        params = list(inspect.signature(self.codec.encode_data).parameters.values())
        self.assertEqual([p.name for p in params], ['data'])
        self.assertEqual(params[0].kind, inspect.Parameter.POSITIONAL_OR_KEYWORD)
        self.assertIs(params[0].default, inspect.Parameter.empty)
        self.assertEqual(self.codec.encode_data(data=('int', 0)).hex(), '00')

    def test_phase1_scope_gate_only_encode_data_public(self):
        # Phase-1 increment gate only (contract I1 scope); not a permanent rule.
        for name in FUTURE_APIS:
            self.assertFalse(hasattr(self.codec, name), name)
        public = sorted(name for name, value in vars(self.codec).items()
                        if not name.startswith('_') and callable(value))
        self.assertEqual(public, ['encode_data'])

    def test_deterministic_and_input_not_mutated(self):
        node = SHAPE_ROWS[-1][0]
        before = repr(node)
        self.assertEqual(self.encode(node), self.encode(node))
        self.assertEqual(repr(node), before)


class TestIntegers(CodecCase):
    def test_cbor_integer_heads_at_every_width_boundary(self):
        self.check(INT_ROWS)

    def test_bignum_tags_minimal_magnitude_and_chunking(self):
        self.check(BIGNUM_ROWS)


class TestBytes(CodecCase):
    def test_definite_up_to_64_then_64_byte_chunks(self):
        self.check(BYTES_ROWS)


class TestContainers(CodecCase):
    def test_empty_definite_nonempty_indefinite_lists(self):
        self.check(LIST_ROWS)

    def test_definite_maps_in_given_order_with_duplicates(self):
        self.check(MAP_ROWS)


class TestConstructors(CodecCase):
    def test_tag_ranges_and_tag_102_definite_pair(self):
        self.check(CONSTR_ROWS)

    def test_nested_boolean_and_public_opaque_shapes(self):
        self.check(SHAPE_ROWS)

    def test_superseded_prior_vectors_are_not_reproduced(self):
        for node, wrong in (
                (TRUE, 'd87a9fff'), (('constr', 1, [TRUE]), 'd87a9fd87a9fffff'),
                (('constr', 7, []), 'd9050780'), (('constr', 128, []), 'd9058080'),
                (('map', []), 'bfff'), (('map', [(('int', 1), ('int', 2))]), 'bf0102ff')):
            with self.subTest(wrong=wrong):
                self.assertNotEqual(self.encode(node).hex(), wrong)


class TestRejection(CodecCase):
    def test_malformed_raw_bool_and_out_of_range_inputs(self):
        self.rejects(MALFORMED)

    def test_exact_types_reject_inert_subclasses(self):
        self.rejects(SUBCLASS_ROWS)

    def test_cycles_raise_value_error_not_recursion_error(self):
        items = []
        looped_list = ('list', items)
        items.append(looped_list)
        pairs = []
        looped_map = ('map', pairs)
        pairs.append((('int', 0), looped_map))
        fields = []
        looped_constr = ('constr', 0, fields)
        fields.append(looped_constr)
        key_pairs = []
        looped_key = ('map', key_pairs)
        key_pairs.append((looped_key, ('int', 0)))
        self.rejects((looped_list, looped_map, looped_constr, looped_key))

    def test_aliasing_is_admitted_and_encoded_per_occurrence(self):
        shared = ('list', [('int', 1)])
        self.check(((('list', [shared, shared]), '9f9f01ff9f01ffff'),))


class TestBounds(CodecCase):
    def test_depth_128_accepted_129_rejected(self):
        self.check((
            (nest(128, as_list, ('list', [])), '9f' * 127 + '80' + 'ff' * 127),
            (nest(128, as_constr, ('int', 7)), 'd8799f' * 127 + '07' + 'ff' * 127),
            (nest(128, as_map_value, ('int', 0)), 'a100' * 127 + '00'),
        ))
        self.rejects((
            nest(129, as_list, ('list', [])),
            nest(129, as_constr, ('int', 7)),
            nest(129, as_map_value, ('int', 0)),
            ('map', [(nest(128, as_list, ('list', [])), ('int', 0))]),
        ))

    def test_node_visits_65536_accepted_65537_rejected(self):
        self.check((
            (('list', [('int', 0)] * 65535), '9f' + '00' * 65535 + 'ff'),
            (('map', [(('int', 0), ('int', 0))] * 32767), 'b97fff' + '00' * 65534),
        ))
        bomb = ('int', 0)
        for _ in range(20):
            bomb = ('list', [bomb, bomb])
        self.rejects((('list', [('int', 0)] * 65536),
                      ('map', [(('int', 0), ('int', 0))] * 32768), bomb))

    def test_alias_bombs_abort_before_expansion(self):
        # Only early B2/B4 aborts finish in E1: 2**65-1 visits at depth 65, and
        # 65535 or 2*32767 aliases of a 67586-byte leaf encoding (~4.43e9 bytes).
        bomb = ('int', 0)
        for _ in range(64):
            bomb = ('list', [bomb, bomb])
        leaf = ('bytes', bytes(65536))
        self.rejects((bomb, ('list', [leaf] * 65535), ('map', [(leaf, leaf)] * 32767)))

    def test_leaf_and_bignum_magnitude_bounds(self):
        self.check((
            (('bytes', bytes(65536)), '5f' + CHUNK_00 * 1024 + 'ff'),
            (('int', 2 ** 524288 - 1), 'c25f' + CHUNK_FF * 1024 + 'ff'),
            (('int', -(2 ** 524288)), 'c35f' + CHUNK_FF * 1024 + 'ff'),
        ))
        self.rejects((('bytes', bytes(65537)), ('int', 2 ** 524288),
                      ('int', -(2 ** 524288) - 1)))

    def test_output_1048576_accepted_1048577_rejected(self):
        leaf = ('bytes', bytes(65536))
        exact = self.encode(('list', [leaf] * 15 + [('int', 0)] * 34784))
        self.assertEqual(len(exact), 1048576)
        self.assertEqual(exact, bytes.fromhex(
            '9f' + ('5f' + CHUNK_00 * 1024 + 'ff') * 15 + '00' * 34784 + 'ff'))
        self.rejects((('list', [leaf] * 15 + [('int', 0)] * 34785),
                      ('list', [leaf] * 16)))


if __name__ == '__main__':
    unittest.main()
