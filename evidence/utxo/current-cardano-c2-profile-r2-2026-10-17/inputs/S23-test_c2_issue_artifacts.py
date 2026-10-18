'''Ordinary tests for the proposed c2_issue_artifacts consumer (contract CI01-CI12).

Author-proposed; RED and GREEN unobserved. Run only as
python3 -I -B test_c2_issue_artifacts.py inside the read-only dedicated
directory of C2-ISSUE-ARTIFACT-CONTRACT.md section 7. Effects: bounded
O_NOFOLLOW reads of sibling files, execution of the pinned host bytes and of
the consumer bytes read once, process-local sys.modules['host_codec'],
unittest output and one stderr line naming the consumer SHA-256. SHA-256 is
provenance only: every expected digest, token and CBOR byte is a public
literal, or literal framing around pinned fixture bytes, never a call to a
production hash or encoder.
'''
import copy
import hashlib
import json
import os
import stat
import sys
import types
import unittest

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))

HOST_FILE, HOST_SIZE = 'host_codec.py', 23252
HOST_SHA256 = '3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459'
PROOF_FILE, PROOF_SIZE = 'K6-PUBLIC-PROOF.json', 5800
PROOF_SHA256 = '930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b'
SETUP_FILE = 'K2-PUBLIC-SETUP-RECORD.json'
SETUP_SHA256 = '86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b'
CONSUMER_FILE = 'c2_issue_artifacts.py'
HOST_APIS = ('build_env_data', 'build_c2_mint_data', 'build_value_data', 'encode_data',
             'parse_statement_header', 'statement_digest_hex', 'statement_public_inputs')

FAMILY = b'MRG16-PVC-FAM2-G16-BLS-ARK04-HL2'
PRED = '0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd'
VKD = 'd19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd'
STMTD = 'c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc'
# Historical K10 R2 builder output (generated manifest lines 32-42 and 104-135):
# a reproduction vector for the captured K6 context, not an independent oracle.
HIST_EXPIRY = 1893456000000
HIST_POLICY = '6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1'
HIST_TOKEN = '9fd5c53f6a1a03d20b0409f2ca3ad28bd02d5401564603fab5f81b289cc6d54e'
HIST_ENV_CBOR = (
    'd8799f020058204d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32'
    '58200d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd'
    '5820d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd'
    '581c55555555555555555555555555555555555555555555555555555555'
    'd8799f5820222222222222222222222222222222222222222222222222222222222222222200ff'
    '5820c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc'
    '1b000001b8dac5b400ff')
ENV_PREFIX = HIST_ENV_CBOR[:-20]
POLICY = bytes.fromhex(HIST_POLICY)
RESULT_KEYS = ('envCborHex', 'envData', 'envelopeTokenNameHex', 'expiry', 'format',
               'issueCborHex', 'issueData', 'labels', 'mintValueCborHex', 'mintValueData',
               'policyIdHex', 'setupRecordSha256Declared')
LABELS = {
    'expiry': 'CALLER_SUPPLIED_NOT_PROOF_BOUND',
    'context': 'CALLER_DECLARED_UNAUTHENTICATED',
    'policyId': 'CALLER_SUPPLIED_UNAUTHENTICATED',
    'trustProfile': 'single-party-test-setup/1',
    'setupRecordLink': 'DECLARED_NOT_BYTE_CHECKED',
    'groth16': 'NOT_VERIFIED',
    'vkCurveValidity': 'NOT_CHECKED',
    'ledger': 'NO_CLAIM',
}
FIX = {}


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _read(name, cap):
    fd = os.open(os.path.join(HERE, name), os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise AssertionError(name + ': expected a regular file')
        data = b''
        while len(data) <= cap:
            chunk = os.read(fd, 65536)
            if not chunk:
                return data
            data += chunk
        raise AssertionError(name + ': exceeds the read bound')
    finally:
        os.close(fd)


def _module(name, data):
    # Execute exactly the bytes already read and pinned: no import path, no pycache.
    module = types.ModuleType(name)
    module.__file__ = os.path.join(HERE, name + '.py')
    exec(compile(data, module.__file__, 'exec', dont_inherit=True), module.__dict__)
    return module


def _unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise AssertionError('duplicate JSON key ' + key)
        result[key] = value
    return result


def _no_constant(name):
    raise AssertionError('non-finite JSON constant ' + name)


def _inputs():
    # Fresh JSON-compatible dictionaries for every call.
    return tuple(json.loads(FIX[key].decode('utf-8'), object_pairs_hook=_unique,
                            parse_constant=_no_constant) for key in ('proof', 'setup'))


def setUpModule():
    host = _read(HOST_FILE, 32768)
    proof = _read(PROOF_FILE, 65536)
    setup = _read(SETUP_FILE, 65536)
    if (len(host), _sha(host)) != (HOST_SIZE, HOST_SHA256):
        raise AssertionError('host_codec.py is not the pinned reviewed 23252-byte host')
    if (len(proof), _sha(proof)) != (PROOF_SIZE, PROOF_SHA256):
        raise AssertionError(PROOF_FILE + ' is not the pinned K6 public proof export')
    if _sha(setup) != SETUP_SHA256:
        raise AssertionError(SETUP_FILE + ' differs from the declared setupRecordSha256')
    FIX.update(proof=proof, setup=setup, previous=sys.modules.get('host_codec'))
    FIX['host'] = _module('host_codec', host)
    sys.modules['host_codec'] = FIX['host']


def tearDownModule():
    if FIX.get('previous') is None:
        sys.modules.pop('host_codec', None)
    else:
        sys.modules['host_codec'] = FIX['previous']


def _prepare():
    if 'prepare' not in FIX:
        try:
            data = _read(CONSUMER_FILE, 131072)
        except FileNotFoundError:
            raise AssertionError('missing implementation: ' + CONSUMER_FILE + ' is absent') from None
        FIX['consumer_bytes'] = data
        print(CONSUMER_FILE + ' sha256 ' + _sha(data), file=sys.stderr)
        FIX['consumer'] = _module('c2_issue_artifacts', data)
        FIX['prepare'] = FIX['consumer'].__dict__.get('prepare_c2_issue')
    if type(FIX['prepare']) is not types.FunctionType:
        raise AssertionError('missing implementation: prepare_c2_issue')
    return FIX['prepare']


def _same(left, right):
    # Data equality that also distinguishes int from bool and tuple from list.
    if type(left) is not type(right):
        return False
    if type(left) in (tuple, list):
        return len(left) == len(right) and all(_same(a, b) for a, b in zip(left, right))
    return left == right


def _env(expiry):
    return ('constr', 0, [
        ('int', 2), ('int', 0), ('bytes', FAMILY), ('bytes', bytes.fromhex(PRED)),
        ('bytes', bytes.fromhex(VKD)), ('bytes', bytes.fromhex('55' * 28)),
        ('constr', 0, [('bytes', bytes.fromhex('22' * 32)), ('int', 0)]),
        ('bytes', bytes.fromhex(STMTD)), ('int', expiry)])


def _issue_cbor(vk_hex, proof_hex):
    # serialise_data: tag 121, indefinite fields, bytes over 64 as indefinite
    # 64-byte chunks (EXPECTATIONS cborRule; pallas plutus_data.rs 423-443).
    vk = ''.join('5840' + vk_hex[i:i + 128] for i in range(0, 896, 128))
    proof = ''.join('5840' + proof_hex[i:i + 128] for i in range(0, 384, 128))
    return 'd8799f5f' + vk + '5828' + vk_hex[896:] + 'ff5f' + proof + 'ffff'


def _mint_cbor(policy_hex, token_hex):
    return 'a1581c' + policy_hex + 'a15820' + token_hex + '01'


def _flip(text, offset):
    # Flip the low bit of one statement byte (structural boundary data only).
    i = 2 * offset
    return text[:i] + format(int(text[i:i + 2], 16) ^ 1, '02x') + text[i + 2:]


def _mutated(changes):
    export, setup = _inputs()
    for target, path, value in changes:
        node = export if target == 'export' else setup
        for key in path[:-1]:
            node = node[key]
        node[path[-1]] = value(node[path[-1]]) if callable(value) else value
    return export, setup


class _Hooked(dict):
    # Any hook call proves the record was touched before the C2I-ARG refusal.
    def _touched(self, *args, **kwargs):
        raise AssertionError('record touched before the C2I-ARG refusal')
    __getitem__ = __iter__ = __len__ = __contains__ = keys = items = values = get = _touched


TAMPER = (
    ('C2I-SETUP', 'setup', ('trustProfile',), 'multi-party/1'),
    ('C2I-SETUP', 'setup', ('extra',), 0),
    ('C2I-SETUP', 'setup', ('relation', 'headerLen'), 344),
    ('C2I-SETUP', 'setup', ('verifyingKeyHex',), str.upper),
    ('C2I-SETUP', 'setup', ('verifyingKeyHex',), lambda v: v[:-2]),
    ('C2I-SETUP', 'setup', ('vkDigestBlake2b256',), lambda v: v[:-1] + 'c'),
    ('C2I-EXPORT', 'export', ('format',), 'moriarty-k2-groth16-proof/2'),
    ('C2I-EXPORT', 'export', ('extra',), 0),
    ('C2I-EXPORT', 'export', ('premises',), lambda v: v[:5]),
    ('C2I-EXPORT', 'export', ('context', 'seq'), True),
    ('C2I-EXPORT', 'export', ('cardano', 'envV2', 'expiry'), HIST_EXPIRY),
    ('C2I-RELATION', 'export', ('programHash',), '00' * 32),
    ('C2I-RELATION', 'export', ('actionIndex',), 2),
    ('C2I-RELATION', 'export', ('sourceSha256',), None),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: v[:-2]),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: v[:-1]),
    ('C2I-STATEMENT', 'export', ('statementHex',), str.upper),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: _flip(v, 48)),
    ('C2I-STATEMENT', 'export', ('statementHex',), lambda v: _flip(v, 112)),
    ('C2I-CONTEXT', 'export', ('statementHex',), lambda v: _flip(v, 144)),
    ('C2I-CONTEXT', 'export', ('context', 'consumedIx'), 1),
    ('C2I-DIGEST', 'export', ('statementHex',), lambda v: _flip(v, 400)),
    ('C2I-DIGEST', 'export', ('statementDigestBlake2b256',), '00' * 32),
    ('C2I-DIGEST', 'export', ('publicInputs', 'x1'), lambda v: '0' + v),
    ('C2I-PROOF', 'export', ('proofHex',), lambda v: v[:-2]),
    ('C2I-PROOF', 'export', ('proofHex',), lambda v: v + '00'),
    ('C2I-CARDANO', 'export', ('cardano', 'envV2', 'claimantVkh'), '54' * 28),
    ('C2I-CARDANO', 'export', ('cardano', 'envV2', 'settleOutref', 'outputIndex'), 1),
    ('C2I-CARDANO', 'export', ('cardano', 'intentAnchor'), lambda v: v.replace('v3', 'v2')),
)


class C2IssueArtifactTests(unittest.TestCase):

    def call(self, export, setup, expiry=HIST_EXPIRY, policy=POLICY):
        return _prepare()(export, setup, expiry, policy_id=policy)

    def refused(self, code, export, setup, expiry=HIST_EXPIRY, policy=POLICY):
        prepare = _prepare()
        try:
            prepare(export, setup, expiry, policy_id=policy)
        except ValueError as error:
            self.assertIs(type(error), ValueError)
            self.assertTrue(str(error).startswith(code + ':'), (code, str(error)))
            return
        self.fail('accepted; expected ' + code)

    def test_01_public_records_literal_provenance(self):
        export, setup = _inputs()
        self.assertEqual(export['setupRecordSha256'], SETUP_SHA256)
        self.assertEqual(setup['trustProfile'], 'single-party-test-setup/1')
        self.assertEqual((export['vkDigestBlake2b256'], setup['vkDigestBlake2b256']), (VKD, VKD))
        self.assertEqual((export['predicateDigest'], export['statementDigestBlake2b256']), (PRED, STMTD))
        context = export['context']
        self.assertEqual((context['claimantVkh'], context['consumedTxid'], context['consumedIx'],
                          context['networkId']), ('55' * 28, '22' * 32, 0, 0))
        self.assertEqual((len(setup['verifyingKeyHex']), len(export['proofHex'])), (976, 384))
        # Expiry is absent from both Rust records: only the caller can supply it.
        self.assertNotIn(b'expiry', FIX['proof'].lower())
        self.assertNotIn(b'expiry', FIX['setup'].lower())
        segments = ('d8799f', '02', '00', '5820' + FAMILY.hex(), '5820' + PRED, '5820' + VKD,
                    '581c' + '55' * 28, 'd8799f5820' + '22' * 32 + '00ff', '5820' + STMTD,
                    '1b000001b8dac5b400', 'ff')
        self.assertEqual(''.join(segments), HIST_ENV_CBOR)

    def test_02_api_shape_and_host_reuse(self):
        prepare = _prepare()
        code = prepare.__code__
        self.assertEqual((code.co_argcount, code.co_kwonlyargcount), (3, 1))
        self.assertEqual(code.co_flags & 0x0C, 0)
        self.assertEqual(code.co_varnames[:4], ('proof_export', 'setup_record', 'expiry', 'policy_id'))
        self.assertIsNone(prepare.__defaults__)
        self.assertIsNone(prepare.__kwdefaults__)
        consumer = FIX['consumer']
        self.assertIs(consumer.__dict__.get('host_codec'), FIX['host'])
        for name in HOST_APIS:
            self.assertNotIn(name, consumer.__dict__)
        public = sorted(name for name, value in consumer.__dict__.items()
                        if not name.startswith('_') and type(value) is types.FunctionType)
        self.assertEqual(public, ['prepare_c2_issue'])

    def test_03_k6_env_data_cbor_and_token_reproduce_historical_vector(self):
        result = self.call(*_inputs())
        self.assertTrue(_same(result['envData'], _env(HIST_EXPIRY)))
        self.assertEqual(result['envCborHex'], HIST_ENV_CBOR)
        self.assertEqual(result['envelopeTokenNameHex'], HIST_TOKEN)

    def test_04_k6_issue_data_and_chunked_cbor(self):
        export, setup = _inputs()
        result = self.call(export, setup)
        vk, proof = bytes.fromhex(setup['verifyingKeyHex']), bytes.fromhex(export['proofHex'])
        self.assertTrue(_same(result['issueData'], ('constr', 0, [('bytes', vk), ('bytes', proof)])))
        self.assertEqual(result['issueCborHex'], _issue_cbor(setup['verifyingKeyHex'], export['proofHex']))
        self.assertEqual(len(result['issueCborHex']), 1420)

    def test_05_mint_value_no_ada_and_caller_policy(self):
        for policy_hex in (HIST_POLICY, '01' * 28):
            with self.subTest(policy=policy_hex):
                result = self.call(*_inputs(), policy=bytes.fromhex(policy_hex))
                expected = ('map', [(('bytes', bytes.fromhex(policy_hex)),
                                     ('map', [(('bytes', bytes.fromhex(HIST_TOKEN)), ('int', 1))]))])
                self.assertTrue(_same(result['mintValueData'], expected))
                self.assertEqual(result['mintValueCborHex'], _mint_cbor(policy_hex, HIST_TOKEN))
                self.assertEqual(result['policyIdHex'], policy_hex)
                # The policy is not an EnvV2 field: Env bytes and token are unchanged.
                self.assertEqual((result['envCborHex'], result['envelopeTokenNameHex']),
                                 (HIST_ENV_CBOR, HIST_TOKEN))

    def test_06_result_schema_labels_bounds_and_detachment(self):
        export, setup = _inputs()
        before = copy.deepcopy((export, setup))
        first = self.call(export, setup)
        second = self.call(export, setup)
        self.assertEqual((export, setup), before)
        self.assertIs(type(first), dict)
        self.assertEqual(sorted(first), sorted(RESULT_KEYS))
        self.assertEqual(first['format'], 'moriarty-c2-issue-artifacts/1')
        self.assertIs(type(first['labels']), dict)
        self.assertEqual(first['labels'], LABELS)
        self.assertIs(type(first['expiry']), int)
        self.assertEqual(first['expiry'], HIST_EXPIRY)
        self.assertEqual(first['setupRecordSha256Declared'], SETUP_SHA256)
        widths = {'envCborHex': 440, 'envelopeTokenNameHex': 64, 'issueCborHex': 1420,
                  'mintValueCborHex': 134, 'policyIdHex': 56, 'setupRecordSha256Declared': 64}
        for key, width in widths.items():
            self.assertIs(type(first[key]), str)
            self.assertEqual(len(first[key]), width)
            self.assertLessEqual(set(first[key]), set('0123456789abcdef'))
        self.assertEqual(first, second)
        self.assertIsNot(first['envData'][2], second['envData'][2])
        self.assertIsNot(first['labels'], second['labels'])
        first['envData'][2].clear()
        first['labels'].clear()
        third = self.call(*_inputs())
        self.assertTrue(_same(third['envData'], _env(HIST_EXPIRY)))
        self.assertEqual(third['labels'], LABELS)

    def test_07_expiry_is_caller_explicit_and_bounded(self):
        cases = ((0, '00'), (23, '17'), (24, '1818'), ((1 << 64) - 1, '1b' + 'ff' * 8))
        for expiry, encoded in cases:
            with self.subTest(expiry=expiry):
                result = self.call(*_inputs(), expiry=expiry)
                self.assertTrue(_same(result['envData'], _env(expiry)))
                self.assertEqual(result['envCborHex'], ENV_PREFIX + encoded + 'ff')
                self.assertEqual(result['expiry'], expiry)
                self.assertEqual(len(result['envelopeTokenNameHex']), 64)
                self.assertNotEqual(result['envelopeTokenNameHex'], HIST_TOKEN)
        for index, bad in enumerate((-1, 1 << 64, True, None, float(HIST_EXPIRY), str(HIST_EXPIRY))):
            with self.subTest(bad=index):
                self.refused('C2I-ARG', *_inputs(), expiry=bad)

    def test_08_each_tamper_refused_at_its_stage(self):
        for index, (code, target, path, value) in enumerate(TAMPER):
            with self.subTest(index=index, code=code):
                export, setup = _mutated([(target, path, value)])
                self.refused(code, export, setup)

    def test_09_first_failing_stage_wins(self):
        bad_setup = ('setup', ('trustProfile',), 'multi-party/1')
        bad_format = ('export', ('format',), 'moriarty-k2-groth16-proof/2')
        bad_seq = ('export', ('context', 'seq'), True)
        bad_program = ('export', ('programHash',), '00' * 32)
        short_statement = ('export', ('statementHex',), lambda v: v[:-2])
        bad_predicate = ('export', ('statementHex',), lambda v: _flip(v, 48))
        mainnet = [('export', ('statementHex',), lambda v: _flip(v, 172)),
                   ('export', ('context', 'networkId'), 1)]
        bad_digest = ('export', ('statementDigestBlake2b256',), '00' * 32)
        short_proof = ('export', ('proofHex',), lambda v: v[:-2])
        bad_anchor = ('export', ('cardano', 'intentAnchor'), 'cardano-plutus-v3:00')
        cases = (
            ('C2I-ARG', [bad_setup, bad_format], -1),
            ('C2I-SETUP', [bad_setup, bad_format], HIST_EXPIRY),
            ('C2I-EXPORT', [bad_seq, bad_program], HIST_EXPIRY),
            ('C2I-RELATION', [bad_program, short_statement], HIST_EXPIRY),
            ('C2I-STATEMENT', [bad_predicate, short_proof, bad_anchor], HIST_EXPIRY),
            ('C2I-CONTEXT', mainnet + [short_proof, bad_anchor], HIST_EXPIRY),
            ('C2I-DIGEST', [bad_digest, short_proof, bad_anchor], HIST_EXPIRY),
            ('C2I-PROOF', [short_proof, bad_anchor], HIST_EXPIRY),
        )
        for code, changes, expiry in cases:
            with self.subTest(code=code):
                export, setup = _mutated(changes)
                self.refused(code, export, setup, expiry=expiry)

    def test_10_argument_types_refused_before_records_are_touched(self):
        export, setup = _inputs()
        cases = (
            (_Hooked(export), setup, POLICY), (export, _Hooked(setup), POLICY),
            ([], setup, POLICY), (None, setup, POLICY),
            (FIX['proof'].decode('utf-8'), setup, POLICY), (export, FIX['setup'], POLICY),
            (export, setup, POLICY[:27]), (export, setup, POLICY + bytes(1)),
            (export, setup, bytearray(POLICY)), (export, setup, HIST_POLICY), (export, setup, None))
        for index, (proof_export, setup_record, policy) in enumerate(cases):
            with self.subTest(index=index):
                self.refused('C2I-ARG', proof_export, setup_record, policy=policy)

    def test_11_consumer_source_has_no_fixture_identity_or_effects(self):
        _prepare()
        text = FIX['consumer_bytes'].decode('utf-8')
        imports = sorted(line.strip() for line in text.splitlines()
                         if line.strip().startswith(('import ', 'from ')))
        self.assertEqual(imports, ['import hashlib', 'import host_codec'])
        fixture_identity = ('55' * 14, '22' * 16, 'a2' * 14, chr(92) + 'x55', chr(92) + 'x22',
                            str(HIST_EXPIRY), HIST_TOKEN[:16], HIST_POLICY[:16], PRED[:16],
                            VKD[:16], STMTD[:16], '7a7fafdd', '37de4837', 'b8e2f734')
        effects = ('open(', 'subprocess', 'socket', 'eval(', 'exec(', 'compile(', '__import__',
                   'importlib', 'environ', 'urllib', 'print(')
        for token in fixture_identity + effects:
            with self.subTest(token=token):
                self.assertNotIn(token, text)


if __name__ == '__main__':
    unittest.main(verbosity=2)
