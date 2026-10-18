'''Ordinary tests for the proposed c2_issue_artifacts_r2 consumer (S33 C2-PROFILE-R2).

Author-proposed PART A prefix; RED and GREEN unobserved and not run here. Once PART A
(this prefix), PART B (case generators and helpers) and PART C (18 methods T00-T17)
are combined and independently audited, the whole file runs only as
python3 -I -B test_c2_issue_artifacts_r2.py inside the read-only dedicated directory
of C2-PROFILE-R2-CONTRACT.md section 7. Effects: bounded O_NOFOLLOW reads of sibling
files, execution of the pinned host bytes, of the pinned v1 consumer bytes and of the
r2 consumer bytes read once, a process-local sys.modules['host_codec'], unittest output
and one stderr line naming the r2 SHA-256. SHA-256 is provenance only: every expected
digest, token and CBOR byte is a public literal, or literal framing around pinned
fixture bytes, never a call to a production hash or encoder.
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
SETUP_FILE, SETUP_SIZE = 'K2-PUBLIC-SETUP-RECORD.json', 3926
SETUP_SHA256 = '86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b'
V1_FILE, V1_SIZE = 'c2_issue_artifacts.py', 15231
V1_SHA256 = '23c3f99d5e0aacfdacc462803b605b5c3c47b6e7e2b5b18b217a562fa0e80f76'
R2_FILE = 'c2_issue_artifacts_r2.py'
# The single phrase S33 reserves for r2 absence; raised only on FileNotFoundError
# from the exact r2 open below, never on any other condition.
MISSING_R2 = 'missing r2 implementation: c2_issue_artifacts_r2.py is absent'
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
RESULT_FORMAT = 'moriarty-c2-issue-artifacts/2'
# The 16 r2 labels, exactly as C2-PROFILE-R2-API.json lists them; every value is a
# public literal honesty marker, never a verification claim.
LABELS = {
    'expiry': 'CALLER_SUPPLIED_NOT_PROOF_BOUND',
    'context': 'CALLER_DECLARED_UNAUTHENTICATED',
    'policyId': 'CALLER_SUPPLIED_UNAUTHENTICATED',
    'trustProfile': 'single-party-test-setup/1',
    'setupRecordLink': 'DECLARED_NOT_BYTE_CHECKED',
    'groth16': 'NOT_VERIFIED',
    'vkCurveValidity': 'NOT_CHECKED',
    'vkProfile': 'NOT_CHECKED',
    'proofCurveValidity': 'NOT_CHECKED',
    'statementReencode': 'NOT_CHECKED',
    'jsonParseLimits': 'NOT_CHECKED',
    'relationProfile': 'DESCRIPTOR_PROFILE_CHECKED_NOT_RECOMPILED',
    'driverId': 'DECLARED_NAME_NOT_PROVENANCE',
    'rustAcceptance': 'NOT_ESTABLISHED',
    'circuitVkCorrespondence': 'OPEN',
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
    v1 = _read(V1_FILE, 65536)
    if (len(host), _sha(host)) != (HOST_SIZE, HOST_SHA256):
        raise AssertionError('host_codec.py is not the pinned reviewed 23252-byte host')
    if (len(proof), _sha(proof)) != (PROOF_SIZE, PROOF_SHA256):
        raise AssertionError(PROOF_FILE + ' is not the pinned K6 public proof export')
    if _sha(setup) != SETUP_SHA256:
        raise AssertionError(SETUP_FILE + ' differs from the declared setupRecordSha256')
    if (len(v1), _sha(v1)) != (V1_SIZE, V1_SHA256):
        raise AssertionError(V1_FILE + ' is not the pinned 15231-byte v1 consumer')
    FIX.update(proof=proof, setup=setup, v1_bytes=v1, previous=sys.modules.get('host_codec'))
    FIX['host'] = _module('host_codec', host)
    sys.modules['host_codec'] = FIX['host']
    FIX['v1'] = _module('c2_issue_artifacts', v1)


def tearDownModule():
    if FIX.get('previous') is None:
        sys.modules.pop('host_codec', None)
    else:
        sys.modules['host_codec'] = FIX['previous']


def _prepare():
    # Lazy r2 load by the same bounded reader, cap 131072. Only a FileNotFoundError
    # from this exact open means r2 is absent and carries the reserved phrase; every
    # other condition (non-regular file, oversize, any other OSError, an exec failure
    # or an absent/non-function prepare_c2_issue) surfaces text without that phrase.
    if 'prepare' not in FIX:
        try:
            data = _read(R2_FILE, 131072)
        except FileNotFoundError:
            raise AssertionError(MISSING_R2) from None
        FIX['r2_bytes'] = data
        print(R2_FILE + ' sha256 ' + _sha(data), file=sys.stderr)
        FIX['r2'] = _module('c2_issue_artifacts_r2', data)
        FIX['prepare'] = FIX['r2'].__dict__.get('prepare_c2_issue')
    if type(FIX['prepare']) is not types.FunctionType:
        raise AssertionError('r2 module exposes no prepare_c2_issue function')
    return FIX['prepare']


# === C2-R2-TEST-PART-A-END ===


# === C2-R2-TEST-PART-B: case generators and helpers ===
# Every r2 and v1 expectation below is a literal transcribed from S33 section 6 and
# is never derived by running r2 or v1. Section 6 allows exactly one computed kind:
# a token expectation hashlib.blake2b(bytes.fromhex(E), digest_size=32) over a
# literal expected envCborHex E, plus the T17 statement digest and x1/x2. Those
# values are NON-ORACLE (T17: SYNTHETIC_NOT_PROOF_NOT_ORACLE): independent of
# host_codec code but not of hashlib, and not evidence about Rust or any proof.
# Equality with v1 is consistency through the shared host, not an oracle.

STAGES = ('ARG', 'SETUP', 'PROFILE', 'EXPORT', 'RELATION', 'STATEMENT', 'CONTEXT',
          'DIGEST', 'PROOF', 'CARDANO', 'BUILD')
ADMIT = 'ADMIT'
V1_FORMAT = 'moriarty-c2-issue-artifacts/1'
# The 8 v1 labels, a value-identical subset of LABELS (S33 section 4).
V1_LABELS = {
    'expiry': 'CALLER_SUPPLIED_NOT_PROOF_BOUND',
    'context': 'CALLER_DECLARED_UNAUTHENTICATED',
    'policyId': 'CALLER_SUPPLIED_UNAUTHENTICATED',
    'trustProfile': 'single-party-test-setup/1',
    'setupRecordLink': 'DECLARED_NOT_BYTE_CHECKED',
    'groth16': 'NOT_VERIFIED',
    'vkCurveValidity': 'NOT_CHECKED',
    'ledger': 'NO_CLAIM',
}
SYNTHETIC = 'SYNTHETIC_NOT_PROOF_NOT_ORACLE'
SURROGATE = '\ud800'
# Change value meaning "remove the key at this path".
DROP = object()
BYTE_FIELDS = ('envCborHex', 'envelopeTokenNameHex', 'issueCborHex', 'mintValueCborHex')
HEX_DIGITS = set('0123456789abcdef')

SETUP_KEYS = ('format', 'trustProfile', 'notice', 'relation', 'curve', 'snark', 'encoding',
              'verifyingKeyHex', 'vkDigestBlake2b256', 'provingKeySha256', 'provingKeyBytes',
              'driver')
# The 13 relation keys v1 consumes, and the 16 it leaves open (29 RelationDescriptor keys).
V1_CONSUMED = ('programHash', 'sourceSha256', 'action', 'actionIndex', 'compilerBuildDigest',
               'r1csSha256', 'predicateDigest', 'abi2ShapeDigest', 'statementTagHex',
               'familyIdHex', 'headerLen', 'bodyLen', 'publicInputCount')
NON_CONSUMED = ('relationFormat', 'relationVersion', 'language', 'semantics',
                'logicalCostProfile', 'profileDigest', 'relationShapeDigest',
                'compilerSourceManifestSha256', 'coreClosureSha256', 'adapter', 'logicalWork',
                'abi2ShapeBytesHex', 'fieldModulusLeHex', 'constraints', 'instanceVariables',
                'witnessVariables')
EXPORT_KEYS = ('format', 'setupRecordSha256', 'programHash', 'sourceSha256', 'action',
               'actionIndex', 'compilerBuildDigest', 'r1csSha256', 'predicateDigest',
               'abi2ShapeDigest', 'vkDigestBlake2b256', 'context', 'statementHex',
               'statementDigestBlake2b256', 'publicInputs', 'proofHex', 'cardano', 'premises')
CONTEXT_KEYS = ('vkDigest', 'claimantVkh', 'networkId', 'settleHash', 'threadId',
                'consumedTxid', 'consumedIx', 'seq', 'catalogueDigest', 'intentsDigest')
CARDANO_KEYS = ('envV2', 'catalogueActionEntry', 'intentTargetHashV2', 'intentAnchor')
ENV_KEYS = ('version', 'claimKind', 'familyId', 'predicateDigest', 'vkDigest', 'claimantVkh',
            'settleOutref', 'statementDigest')
OUTREF_KEYS = ('transactionId', 'outputIndex')
ACTION_KEYS = ('name', 'index', 'predicateDigest', 'vkDigest', 'shapeDigest')
# The six nested export objects (S33 T07, N04); a list-valued export itself is ARG (T03).
NESTED = (('context',), ('publicInputs',), ('cardano',), ('cardano', 'envV2'),
          ('cardano', 'envV2', 'settleOutref'), ('cardano', 'catalogueActionEntry'))
OBJECTS = (((), EXPORT_KEYS),) + tuple(zip(NESTED, (CONTEXT_KEYS, ('x1', 'x2'), CARDANO_KEYS,
                                                    ENV_KEYS, OUTREF_KEYS, ACTION_KEYS)))
INT_FIELDS = (('actionIndex',), ('context', 'networkId'), ('context', 'consumedIx'),
              ('context', 'seq'), ('cardano', 'envV2', 'version'),
              ('cardano', 'envV2', 'claimKind'), ('cardano', 'envV2', 'settleOutref', 'outputIndex'),
              ('cardano', 'catalogueActionEntry', 'index'))
# T15 source scan: the v1 test_11 fixture-identity and effect tokens, plus 'global'.
EFFECT_TOKENS = ('55' * 14, '22' * 16, 'a2' * 14, chr(92) + 'x55', chr(92) + 'x22',
                 str(HIST_EXPIRY), HIST_TOKEN[:16], HIST_POLICY[:16], PRED[:16], VKD[:16],
                 STMTD[:16], '7a7fafdd', '37de4837', 'b8e2f734',
                 'open(', 'subprocess', 'socket', 'eval(', 'exec(', 'compile(', '__import__',
                 'importlib', 'environ', 'urllib', 'print(', 'global')


def _same(left, right):
    # Data equality that also distinguishes int from bool and tuple from list.
    if type(left) is not type(right):
        return False
    if type(left) in (tuple, list):
        return len(left) == len(right) and all(_same(a, b) for a, b in zip(left, right))
    if type(left) is dict:
        return list(left) == list(right) and all(_same(left[k], right[k]) for k in left)
    return left == right


def _env(expiry, claimant='55' * 28, txid='22' * 32, index=0, digest=STMTD):
    # Expected EnvV2 host Data; the defaults are the K6 context.
    return ('constr', 0, [
        ('int', 2), ('int', 0), ('bytes', FAMILY), ('bytes', bytes.fromhex(PRED)),
        ('bytes', bytes.fromhex(VKD)), ('bytes', bytes.fromhex(claimant)),
        ('constr', 0, [('bytes', bytes.fromhex(txid)), ('int', index)]),
        ('bytes', bytes.fromhex(digest)), ('int', expiry)])


def _issue_cbor(vk_hex, proof_hex):
    # serialise_data: tag 121, indefinite fields, bytes over 64 as indefinite
    # 64-byte chunks: 710 bytes, VK as 7x64+40 chunks and proof as 3x64 chunks.
    vk = ''.join('5840' + vk_hex[i:i + 128] for i in range(0, 896, 128))
    proof = ''.join('5840' + proof_hex[i:i + 128] for i in range(0, 384, 128))
    return 'd8799f5f' + vk + '5828' + vk_hex[896:] + 'ff5f' + proof + 'ffff'


def _mint_cbor(policy_hex, token_hex):
    return 'a1581c' + policy_hex + 'a15820' + token_hex + '01'


def _token(env_hex):
    # NON-ORACLE test-computed expectation over a literal envCborHex (S33 section 6).
    return hashlib.blake2b(bytes.fromhex(env_hex), digest_size=32).hexdigest()


def _flip(text, offset):
    # Flip the low bit of one byte of a hex string (structural boundary data only).
    i = 2 * offset
    return text[:i] + format(int(text[i:i + 2], 16) ^ 1, '02x') + text[i + 2:]


def _one(text):
    # One-character change of the last character; the result stays an ASCII str.
    return text[:-1] + ('0' if text[-1] != '0' else '1')


def _hex63(text):
    return text[:-1]


def _surrogate(text):
    return text + SURROGATE


def _as_list(value):
    return list(value.values())


def _mutated(changes):
    # Fresh fixture dicts with each (target, path, value) change applied in order:
    # DROP removes the key, a callable maps the old value, anything else replaces it.
    export, setup = _inputs()
    for target, path, value in changes:
        node = export if target == 'export' else setup
        for key in path[:-1]:
            node = node[key]
        if value is DROP:
            del node[path[-1]]
        elif callable(value):
            node[path[-1]] = value(node[path[-1]])
        else:
            node[path[-1]] = value
    return export, setup


class _Hooked(dict):
    # Any hook call proves the record was touched before the C2I-ARG refusal.
    def _touched(self, *args, **kwargs):
        raise AssertionError('record touched before the C2I-ARG refusal')
    __getitem__ = __iter__ = __len__ = __contains__ = __eq__ = __ne__ = _touched
    __setitem__ = __delitem__ = __copy__ = __deepcopy__ = __reduce__ = __reduce_ex__ = _touched
    keys = items = values = get = copy = pop = setdefault = update = _touched


def _containers(root):
    # Identities of every mutable container (dict, list, set, bytearray) reachable
    # through dicts, lists, tuples, sets and frozensets. str and int leaves may be
    # shared (S33 CP07) and are not containers.
    found, seen, pending = set(), set(), [root]
    while pending:
        value = pending.pop()
        if (not isinstance(value, (dict, list, tuple, set, frozenset, bytearray))
                or id(value) in seen):
            continue
        seen.add(id(value))
        if isinstance(value, (dict, list, set, bytearray)):
            found.add(id(value))
        if isinstance(value, dict):
            pending.extend(dict.keys(value))
            pending.extend(dict.values(value))
        elif not isinstance(value, bytearray):
            pending.extend(value)
    return found


def _globals(module):
    # Module state other than dunder entries such as the injected __builtins__.
    return {name: value for name, value in module.__dict__.items()
            if not name.startswith('__')}


def _case(name, r2, v1, *changes, expiry=HIST_EXPIRY, policy=POLICY):
    # One literal row case: r2 and v1 are stage names (C2I-<stage>) or ADMIT.
    return (name, r2, v1, changes, expiry, policy)


def _s(*args):
    return ('setup', args[:-1], args[-1])


def _r(*args):
    return ('setup', ('relation',) + args[:-1], args[-1])


def _x(*args):
    return ('export', args[:-1], args[-1])


def _dotted(path):
    return '.'.join(('export',) + path)


# T03 (args, ARG; v1 =): the expiry and policy cases. The record-type cases need
# non-dict arguments and _Hooked wrappers, so the T03 method builds them itself.
ARG_CASES = tuple(
    [_case('expiry ' + label, 'ARG', 'ARG', expiry=value)
     for label, value in (('-1', -1), ('2**64', 1 << 64), ('True', True), ('None', None),
                          ('float', float(HIST_EXPIRY)), ('str', str(HIST_EXPIRY)))]
    + [_case('policy ' + label, 'ARG', 'ARG', policy=value)
       for label, value in (('27 bytes', POLICY[:27]), ('29 bytes', POLICY + bytes(1)),
                            ('bytearray', bytearray(POLICY)), ('str', HIST_POLICY),
                            ('None', None))])

# T04 (S, SETUP; v1 SETUP unless noted). '(v1 ADMIT)' applies only to the driver
# one-char change and provingKeyBytes 2**64 (N05).
CASE_DRIVER_ONE = _case('driver one-char change', 'SETUP', ADMIT, _s('driver', _one))
CASE_PK_BYTES_2_64 = _case('provingKeyBytes 2**64', 'SETUP', ADMIT,
                           _s('provingKeyBytes', 1 << 64))
T04_CASES = tuple(
    [_case('one-char ' + key, 'SETUP', 'SETUP', _s(key, _one))
     for key in ('format', 'trustProfile', 'notice', 'curve', 'snark', 'encoding')]
    + [CASE_DRIVER_ONE,
       _case('driver int', 'SETUP', 'SETUP', _s('driver', 1)),
       _case('provingKeySha256 uppercase', 'SETUP', 'SETUP', _s('provingKeySha256', str.upper)),
       _case('provingKeySha256 63 hex', 'SETUP', 'SETUP', _s('provingKeySha256', _hex63)),
       _case('provingKeyBytes 0', 'SETUP', 'SETUP', _s('provingKeyBytes', 0)),
       _case('provingKeyBytes -1', 'SETUP', 'SETUP', _s('provingKeyBytes', -1)),
       _case('provingKeyBytes True', 'SETUP', 'SETUP', _s('provingKeyBytes', True)),
       CASE_PK_BYTES_2_64,
       _case('extra key', 'SETUP', 'SETUP', _s('extra', 0))]
    + [_case('missing ' + key, 'SETUP', 'SETUP', _s(key, DROP)) for key in SETUP_KEYS]
    + [_case('relation as a list', 'SETUP', 'SETUP', _s('relation', _as_list)),
       _case('VK 974 hex', 'SETUP', 'SETUP', _s('verifyingKeyHex', lambda v: v[:-2])),
       _case('VK uppercase', 'SETUP', 'SETUP', _s('verifyingKeyHex', str.upper)),
       _case('vkDigest flip', 'SETUP', 'SETUP',
             _s('vkDigestBlake2b256', lambda v: _flip(v, 31)))])

# T05a (S.rel, PROFILE; v1 SETUP): faults in the 13 v1-consumed relation keys.
T05A_CASES = tuple(
    [_case('missing ' + key, 'PROFILE', 'SETUP', _r(key, DROP)) for key in V1_CONSUMED]
    + [_case(key + ' ' + label, 'PROFILE', 'SETUP', _r(key, change))
       for key in ('programHash', 'compilerBuildDigest', 'r1csSha256', 'predicateDigest',
                   'abi2ShapeDigest', 'sourceSha256')
       for label, change in (('63 hex', _hex63), ('uppercase', str.upper))]
    + [_case('statementTagHex flip', 'PROFILE', 'SETUP',
             _r('statementTagHex', lambda v: _flip(v, 0))),
       _case('familyIdHex flip', 'PROFILE', 'SETUP', _r('familyIdHex', lambda v: _flip(v, 0))),
       _case('headerLen 342', 'PROFILE', 'SETUP', _r('headerLen', 342)),
       _case('bodyLen -1', 'PROFILE', 'SETUP', _r('bodyLen', -1)),
       _case('bodyLen 261802', 'PROFILE', 'SETUP', _r('bodyLen', 261802)),
       _case('publicInputCount 3', 'PROFILE', 'SETUP', _r('publicInputCount', 3)),
       _case('actionIndex 65536', 'PROFILE', 'SETUP', _r('actionIndex', 65536)),
       _case('actionIndex True', 'PROFILE', 'SETUP', _r('actionIndex', True)),
       _case('action int', 'PROFILE', 'SETUP', _r('action', 1))])

# T05b (S.rel, PROFILE; v1 ADMIT): faults in the 16 keys v1 leaves open.
LOGICAL_WORK_BAD = (
    ("''", ''), ("'01'", '01'), ("'+1'", '+1'), ("'-1'", '-1'), ('40 digits', '1' * 40),
    ('2**128 as decimal', str(1 << 128)), ('Arabic-Indic two-eight', '٢٨'),
    ("'1_0'", '1_0'), ("' 28'", ' 28'), ("'28' plus LF", '28\n'), ('5000 digits', '1' * 5000),
    ('int 28', 28))
T05B_CASES = tuple(
    [_case('missing ' + key, 'PROFILE', ADMIT, _r(key, DROP)) for key in NON_CONSUMED]
    + [_case('extra key', 'PROFILE', ADMIT, _r('extra', 0))]
    + [_case(key + ' one-char', 'PROFILE', ADMIT, _r(key, _one))
       for key in ('language', 'semantics', 'adapter')]
    + [_case(key + ' ' + label, 'PROFILE', ADMIT, _r(key, change))
       for key in ('profileDigest', 'relationShapeDigest', 'compilerSourceManifestSha256',
                   'coreClosureSha256')
       for label, change in (('63 hex', _hex63), ('uppercase', str.upper))]
    + [_case('logicalWork ' + label, 'PROFILE', ADMIT, _r('logicalWork', value))
       for label, value in LOGICAL_WORK_BAD]
    + [_case('instanceVariables 4', 'PROFILE', ADMIT, _r('instanceVariables', 4))]
    + [_case(key + ' ' + label, 'PROFILE', ADMIT, _r(key, value))
       for key in ('relationVersion', 'constraints', 'witnessVariables')
       for label, value in (('True', True), ('-1', -1), ('2**64', 1 << 64))]
    + [_case(key + ' int', 'PROFILE', ADMIT, _r(key, 1))
       for key in ('relationFormat', 'logicalCostProfile', 'fieldModulusLeHex')]
    + [_case('abi2ShapeBytesHex odd', 'PROFILE', ADMIT, _r('abi2ShapeBytesHex', _hex63)),
       _case('abi2ShapeBytesHex uppercase', 'PROFILE', ADMIT,
             _r('abi2ShapeBytesHex', str.upper)),
       _case('abi2ShapeBytesHex 524290 chars', 'PROFILE', ADMIT,
             _r('abi2ShapeBytesHex', '00' * 262145)),
       _case('abi2ShapeBytesHex one byte flipped', 'PROFILE', ADMIT,
             _r('abi2ShapeBytesHex', lambda v: _flip(v, 0)))])

# T05c (S.rel, PROFILE; v1 RELATION): setup-side abi2ShapeDigest, another valid hex64.
T05C_CASES = (_case('abi2ShapeDigest another valid hex64', 'PROFILE', 'RELATION',
                    _r('abi2ShapeDigest', '00' * 32)),)

# T06 (S.rel and E; listed stages). U+D800 placement keeps every length check passing.
CASE_SURROGATE_REL = tuple(
    _case('U+D800 in relation ' + key, 'PROFILE', ADMIT, _r(key, _surrogate))
    for key in ('relationFormat', 'logicalCostProfile', 'fieldModulusLeHex'))
# REDUNDANT (S33 T06, N11): the closed-key check refuses this key with or without
# the CP04 scan, so it does not discriminate the scan.
CASE_SURROGATE_KEY = _case('U+D800 in a relation extra key (REDUNDANT)', 'PROFILE', ADMIT,
                           _r(SURROGATE, 0))
CASE_SURROGATE_ACTION = _case(
    'U+D800 consistently in relation action, export action and catalogue name',
    'PROFILE', ADMIT, _r('action', _surrogate), _x('action', _surrogate),
    _x('cardano', 'catalogueActionEntry', 'name', _surrogate))
T06_CASES = CASE_SURROGATE_REL + (
    CASE_SURROGATE_KEY,
    _case('U+D800 in relation action alone', 'PROFILE', 'RELATION', _r('action', _surrogate)),
    CASE_SURROGATE_ACTION,
    _case('U+D800 in export action alone', 'EXPORT', 'RELATION', _x('action', _surrogate)),
    _case('U+D800 in catalogue name alone', 'EXPORT', 'CARDANO',
          _x('cardano', 'catalogueActionEntry', 'name', _surrogate)),
    _case('U+D800 in intentTargetHashV2 alone', 'EXPORT', 'CARDANO',
          _x('cardano', 'intentTargetHashV2', _surrogate)),
    _case('U+D800 in intentAnchor alone', 'EXPORT', 'CARDANO',
          _x('cardano', 'intentAnchor', _surrogate)),
    # Not scanned at EXPORT: left to the STATEMENT and PROOF length-first hex checks
    # and the DIGEST exact-string equality (CP04).
    _case('U+D800 in statementHex', 'STATEMENT', 'STATEMENT',
          _x('statementHex', lambda v: v[:-1] + SURROGATE)),
    _case('U+D800 in proofHex', 'PROOF', 'PROOF', _x('proofHex', lambda v: v[:-1] + SURROGATE)),
    _case('U+D800 in x1', 'DIGEST', 'DIGEST', _x('publicInputs', 'x1', _surrogate)))

# T07 (E, EXPORT; v1 =).
T07_CASES = tuple(
    [_case('extra key in ' + _dotted(path), 'EXPORT', 'EXPORT', _x(*path, 'extra', 0))
     for path, _keys in OBJECTS]
    + [_case('expiry in export.cardano.envV2', 'EXPORT', 'EXPORT',
             _x('cardano', 'envV2', 'expiry', HIST_EXPIRY))]
    + [_case('missing ' + _dotted(path + (key,)), 'EXPORT', 'EXPORT', _x(*path, key, DROP))
       for path, keys in OBJECTS for key in keys]
    + [_case(_dotted(path) + ' as a list', 'EXPORT', 'EXPORT', _x(*path, _as_list))
       for path in NESTED]
    + [_case(_dotted(path) + ' ' + str(value), 'EXPORT', 'EXPORT', _x(*path, value))
       for path, value in ((('context', 'networkId'), 256), (('cardano', 'envV2', 'version'), 256),
                           (('cardano', 'envV2', 'claimKind'), 256), (('actionIndex',), 65536),
                           (('context', 'consumedIx'), 65536),
                           (('cardano', 'envV2', 'settleOutref', 'outputIndex'), 65536),
                           (('cardano', 'catalogueActionEntry', 'index'), 65536),
                           (('context', 'seq'), 1 << 64))]
    + [_case(_dotted(path) + ' True', 'EXPORT', 'EXPORT', _x(*path, True)) for path in INT_FIELDS]
    + [_case('action int', 'EXPORT', 'EXPORT', _x('action', 1)),
       _case('sourceSha256 64 uppercase A', 'EXPORT', 'EXPORT', _x('sourceSha256', 'A' * 64)),
       _case('premises reordered', 'EXPORT', 'EXPORT',
             _x('premises', lambda v: [v[1], v[0]] + v[2:])),
       _case('premises as a tuple', 'EXPORT', 'EXPORT', _x('premises', tuple)),
       _case('premises with 5 items', 'EXPORT', 'EXPORT', _x('premises', lambda v: v[:5])),
       _case('premises one char changed', 'EXPORT', 'EXPORT',
             _x('premises', lambda v: [_one(v[0])] + v[1:]))])

# T08 (E and S.rel, RELATION; v1 =): schema-valid identity replacements.
T08_CASES = tuple(
    [_case('export ' + key + ' 00x32', 'RELATION', 'RELATION', _x(key, '00' * 32))
     for key in ('programHash', 'compilerBuildDigest', 'r1csSha256', 'predicateDigest',
                 'abi2ShapeDigest', 'vkDigestBlake2b256')]
    + [_case('export sourceSha256 null', 'RELATION', 'RELATION', _x('sourceSha256', None)),
       _case('export action full', 'RELATION', 'RELATION', _x('action', 'full')),
       _case('export actionIndex 2', 'RELATION', 'RELATION', _x('actionIndex', 2))]
    + [_case('relation ' + key + ' 00x32', 'RELATION', 'RELATION', _r(key, '00' * 32))
       for key in ('programHash', 'compilerBuildDigest', 'r1csSha256', 'predicateDigest')]
    + [_case('relation sourceSha256 null', 'RELATION', 'RELATION', _r('sourceSha256', None)),
       _case('relation action full', 'RELATION', 'RELATION', _r('action', 'full')),
       _case('relation actionIndex 2', 'RELATION', 'RELATION', _r('actionIndex', 2))])

# T09 (E and S.rel, STATEMENT; v1 =).
T09_CASES = tuple(
    [_case('statementHex 00x262145', 'STATEMENT', 'STATEMENT', _x('statementHex', '00' * 262145)),
     _case('statementHex odd length', 'STATEMENT', 'STATEMENT', _x('statementHex', _hex63)),
     _case('statementHex uppercase', 'STATEMENT', 'STATEMENT', _x('statementHex', str.upper)),
     _case('statementHex containing g', 'STATEMENT', 'STATEMENT',
           _x('statementHex', lambda v: v[:-1] + 'g'))]
    + [_case('statement flip at ' + str(offset), 'STATEMENT', 'STATEMENT',
             _x('statementHex', lambda v, offset=offset: _flip(v, offset)))
       for offset in (0, 16, 48, 80, 112)]
    + [_case('statement last byte removed', 'STATEMENT', 'STATEMENT',
             _x('statementHex', lambda v: v[:-2])),
       _case('relation bodyLen 377', 'STATEMENT', 'STATEMENT', _r('bodyLen', 377))])

# T10 (E, CONTEXT; v1 =): each replacement keeps its exact type and range.
T10_CASES = tuple(
    [_case('context ' + key + ' changed', 'CONTEXT', 'CONTEXT', _x('context', key, value))
     for key, value in (('vkDigest', '00' * 32), ('claimantVkh', '54' * 28), ('networkId', 1),
                        ('settleHash', '00' * 28), ('threadId', '00' * 32),
                        ('consumedTxid', '00' * 32), ('consumedIx', 1), ('seq', 2),
                        ('catalogueDigest', '00' * 32), ('intentsDigest', '00' * 32))]
    + [_case('statement byte 144 flipped', 'CONTEXT', 'CONTEXT',
             _x('statementHex', lambda v: _flip(v, 144))),
       _case('networkId 1 in statement byte 172 and context', 'CONTEXT', 'CONTEXT',
             _x('statementHex', lambda v: _flip(v, 172)), _x('context', 'networkId', 1))])

# T11 (E, DIGEST; v1 =). x1 and x2 are tampered with the str '0', never int 0.
T11_CASES = (
    _case('statementDigest 00x32', 'DIGEST', 'DIGEST', _x('statementDigestBlake2b256', '00' * 32)),
    _case('body byte 400 flipped', 'DIGEST', 'DIGEST', _x('statementHex', lambda v: _flip(v, 400))),
    _case("x1 set to '0'", 'DIGEST', 'DIGEST', _x('publicInputs', 'x1', '0')),
    _case("x2 set to '0'", 'DIGEST', 'DIGEST', _x('publicInputs', 'x2', '0')),
    _case('x1 with a leading 0', 'DIGEST', 'DIGEST', _x('publicInputs', 'x1', lambda v: '0' + v)))

# T12 (E, PROOF; v1 =).
T12_CASES = (
    _case('proofHex uppercase', 'PROOF', 'PROOF', _x('proofHex', str.upper)),
    _case('proofHex 382 chars', 'PROOF', 'PROOF', _x('proofHex', lambda v: v[:-2])),
    _case('proofHex 386 chars', 'PROOF', 'PROOF', _x('proofHex', lambda v: v + '00')),
    _case('proofHex containing g', 'PROOF', 'PROOF', _x('proofHex', lambda v: v[:-1] + 'g')))

# T13 (E, CARDANO; v1 =): each replacement keeps its exact type and range.
CASE_CARDANO_ANCHOR = _case('intentAnchor changed', 'CARDANO', 'CARDANO',
                            _x('cardano', 'intentAnchor', lambda v: v.replace('v3', 'v2')))
T13_CASES = tuple(
    [_case('envV2 ' + '.'.join(path) + ' changed', 'CARDANO', 'CARDANO',
           _x('cardano', 'envV2', *path, value))
     for path, value in ((('version',), 3), (('claimKind',), 1), (('familyId',), '00' * 32),
                         (('predicateDigest',), '00' * 32), (('vkDigest',), '00' * 32),
                         (('claimantVkh',), '54' * 28),
                         (('settleOutref', 'transactionId'), '00' * 32),
                         (('settleOutref', 'outputIndex'), 1),
                         (('statementDigest',), '00' * 32))]
    + [_case('catalogueActionEntry ' + key + ' changed', 'CARDANO', 'CARDANO',
             _x('cardano', 'catalogueActionEntry', key, value))
       for key, value in (('name', 'full'), ('index', 2), ('predicateDigest', '00' * 32),
                          ('vkDigest', '00' * 32), ('shapeDigest', '00' * 32))]
    + [_case('intentTargetHashV2 changed', 'CARDANO', 'CARDANO',
             _x('cardano', 'intentTargetHashV2', _one)),
       CASE_CARDANO_ANCHOR])

# T14 (mixed): all nine first-fault pairs; the earlier stage is expected. BUILD is
# excluded as unreachable for admitted input (S34 unreachableForAdmittedInput).
CASE_DRIVER_LANGUAGE = _case('SETUP/PROFILE', 'SETUP', ADMIT, _s('driver', _one),
                             _r('language', _one))
T14_CASES = (
    _case('ARG/SETUP', 'ARG', 'ARG', _s('trustProfile', _one), expiry=-1),
    CASE_DRIVER_LANGUAGE,
    _case('PROFILE/EXPORT', 'PROFILE', 'EXPORT', _r('language', _one), _x('extra', 0)),
    _case('EXPORT/RELATION', 'EXPORT', 'EXPORT', _x('context', 'seq', True),
          _x('programHash', '00' * 32)),
    _case('RELATION/STATEMENT', 'RELATION', 'RELATION', _x('programHash', '00' * 32),
          _x('statementHex', lambda v: v[:-2])),
    _case('STATEMENT/CONTEXT', 'STATEMENT', 'STATEMENT',
          _x('statementHex', lambda v: _flip(v, 48)), _x('context', 'consumedIx', 1)),
    _case('CONTEXT/DIGEST', 'CONTEXT', 'CONTEXT', _x('context', 'consumedIx', 1),
          _x('statementDigestBlake2b256', '00' * 32)),
    _case('DIGEST/PROOF', 'DIGEST', 'DIGEST', _x('statementDigestBlake2b256', '00' * 32),
          _x('proofHex', lambda v: v[:-2])),
    _case('PROOF/CARDANO', 'PROOF', 'PROOF', _x('proofHex', lambda v: v[:-2]),
          _x('cardano', 'intentAnchor', lambda v: v.replace('v3', 'v2'))))

# T16: strictness only. Exactly the S33/S34 T16StrictnessCases (v1 ADMIT, r2 refuses);
# the both-admit rows T02 and T17 are excluded and nothing here samples CP06.
T16_CASES = ((CASE_DRIVER_ONE, CASE_PK_BYTES_2_64) + T05B_CASES + CASE_SURROGATE_REL
             + (CASE_SURROGATE_KEY, CASE_SURROGATE_ACTION, CASE_DRIVER_LANGUAGE))

# The single-fault r2 TAMPER table keyed by the 11 r2 stages. BUILD stays empty:
# no schema-valid input reaches a host factory refusal.
TAMPER = {stage: tuple(case for case in ARG_CASES + T04_CASES + T05A_CASES + T05B_CASES
                       + T05C_CASES + T06_CASES + T07_CASES + T08_CASES + T09_CASES
                       + T10_CASES + T11_CASES + T12_CASES + T13_CASES if case[1] == stage)
          for stage in STAGES}


# === C2-R2-TEST-PART-B-END ===


# === C2-R2-TEST-PART-C: the 18 methods T00-T17 ===
class C2IssueArtifactR2Tests(unittest.TestCase):
    # T00 never calls _prepare(), so it passes with r2 absent. Each of T01-T17 calls
    # _prepare() as its first statement, before any other assertion or subTest.

    def call(self, export, setup, expiry=HIST_EXPIRY, policy=POLICY):
        return _prepare()(export, setup, expiry, policy_id=policy)

    def refused(self, code, export, setup, expiry=HIST_EXPIRY, policy=POLICY, prepare=None):
        # Exactly ValueError, never a subclass, prefixed by the first failing stage.
        prepare = _prepare() if prepare is None else prepare
        try:
            prepare(export, setup, expiry, policy_id=policy)
        except ValueError as error:
            self.assertIs(type(error), ValueError)
            self.assertTrue(str(error).startswith(code + ':'), (code, str(error)))
            return
        self.fail('accepted; expected ' + code)

    def v1_refused(self, code, export, setup, expiry=HIST_EXPIRY, policy=POLICY):
        self.refused(code, export, setup, expiry, policy, prepare=FIX['v1'].prepare_c2_issue)

    def v1_admitted(self, export, setup, expiry=HIST_EXPIRY, policy=POLICY):
        # v1 ADMIT: its /1 result with exactly the 8 v1 labels.
        result = FIX['v1'].prepare_c2_issue(export, setup, expiry, policy_id=policy)
        self.assertIs(type(result), dict)
        self.assertEqual(sorted(result), sorted(RESULT_KEYS))
        self.assertEqual(result['format'], V1_FORMAT)
        self.assertIs(type(result['labels']), dict)
        self.assertEqual(len(result['labels']), 8)
        self.assertEqual(result['labels'], V1_LABELS)
        return result

    def check(self, label, case):
        # One literal row case on fresh fixture dicts: r2 first, then v1 on new dicts.
        name, r2, v1, changes, expiry, policy = case
        with self.subTest(label, case=name):
            self.assertIn(r2, STAGES[:-1])
            export, setup = _mutated(changes)
            before = copy.deepcopy((export, setup))
            self.refused('C2I-' + r2, export, setup, expiry, policy)
            self.assertTrue(_same((export, setup), before), 'r2 mutated an input')
            export, setup = _mutated(changes)
            if v1 == ADMIT:
                result = self.v1_admitted(export, setup, expiry, policy)
                # Every v1-ADMIT row keeps the K6 Env fields, HIST_EXPIRY and POLICY.
                self.assertEqual((result['envCborHex'], result['envelopeTokenNameHex']),
                                 (HIST_ENV_CBOR, HIST_TOKEN))
            else:
                self.v1_refused('C2I-' + v1, export, setup, expiry, policy)

    def test_T00_pins_fixtures_and_literals_without_r2(self):
        # No assertion about r2: this method must pass at RED with r2 absent.
        for name, size, digest, cap in ((HOST_FILE, HOST_SIZE, HOST_SHA256, 32768),
                                        (V1_FILE, V1_SIZE, V1_SHA256, 65536),
                                        (PROOF_FILE, PROOF_SIZE, PROOF_SHA256, 65536),
                                        (SETUP_FILE, SETUP_SIZE, SETUP_SHA256, 65536)):
            with self.subTest('T00', file=name):
                data = _read(name, cap)
                self.assertEqual((len(data), _sha(data)), (size, digest))
        self.assertEqual((len(FIX['setup']), _sha(FIX['setup'])), (SETUP_SIZE, SETUP_SHA256))
        self.assertEqual((len(FIX['proof']), _sha(FIX['proof'])), (PROOF_SIZE, PROOF_SHA256))
        self.assertEqual((len(FIX['v1_bytes']), _sha(FIX['v1_bytes'])), (V1_SIZE, V1_SHA256))
        self.assertIs(sys.modules.get('host_codec'), FIX['host'])
        # The strict loader refuses duplicate keys and non-finite constants.
        for text in ('{"a": 1, "a": 2}', '[NaN]', '[Infinity]', '[-Infinity]'):
            with self.subTest('T00', json=text):
                with self.assertRaises(AssertionError):
                    json.loads(text, object_pairs_hook=_unique, parse_constant=_no_constant)
        export, setup = _inputs()
        self.assertIs(type(export), dict)
        self.assertIs(type(setup), dict)
        self.assertEqual(sorted(setup), sorted(SETUP_KEYS))
        self.assertEqual(len(setup['relation']), 29)
        self.assertEqual(sorted(setup['relation']), sorted(V1_CONSUMED + NON_CONSUMED))
        for path, keys in OBJECTS:
            node = export
            for key in path:
                node = node[key]
            self.assertEqual(sorted(node), sorted(keys), path)
        # Literal provenance of the pinned public records (as v1 test_01).
        self.assertEqual(export['setupRecordSha256'], SETUP_SHA256)
        self.assertEqual(setup['trustProfile'], 'single-party-test-setup/1')
        self.assertEqual(setup['driver'], 'mori-k2-driver/0.1.0-k2s1+k4h1')
        self.assertEqual((export['vkDigestBlake2b256'], setup['vkDigestBlake2b256']), (VKD, VKD))
        self.assertEqual((export['predicateDigest'], export['statementDigestBlake2b256']),
                         (PRED, STMTD))
        context = export['context']
        self.assertEqual((context['claimantVkh'], context['consumedTxid'], context['consumedIx'],
                          context['networkId']), ('55' * 28, '22' * 32, 0, 0))
        self.assertEqual((len(setup['verifyingKeyHex']), len(export['proofHex'])), (976, 384))
        self.assertEqual((setup['relation']['headerLen'], setup['relation']['bodyLen']), (343, 378))
        self.assertNotIn(b'expiry', FIX['proof'].lower())
        self.assertNotIn(b'expiry', FIX['setup'].lower())
        segments = ('d8799f', '02', '00', '5820' + FAMILY.hex(), '5820' + PRED, '5820' + VKD,
                    '581c' + '55' * 28, 'd8799f5820' + '22' * 32 + '00ff', '5820' + STMTD,
                    '1b000001b8dac5b400', 'ff')
        self.assertEqual(''.join(segments), HIST_ENV_CBOR)
        # Label literals: 16 r2 labels; the 8 v1 labels are a value-identical subset.
        self.assertEqual(len(LABELS), 16)
        self.assertEqual({key: LABELS[key] for key in V1_LABELS}, V1_LABELS)
        # Table structure only: every stage but BUILD has cases; BUILD is unreachable.
        self.assertEqual(tuple(TAMPER), STAGES)
        self.assertEqual(TAMPER['BUILD'], ())
        for stage in STAGES[:-1]:
            self.assertTrue(TAMPER[stage], stage)

    def test_T01_sole_function_imports_and_seven_host_apis(self):
        prepare = _prepare()
        text = FIX['r2_bytes'].decode('utf-8')
        imports = sorted(line.strip() for line in text.splitlines()
                         if line.strip().startswith(('import ', 'from ')))
        self.assertEqual(imports, ['import hashlib', 'import host_codec'])
        code = prepare.__code__
        self.assertEqual((code.co_argcount, code.co_posonlyargcount, code.co_kwonlyargcount),
                         (3, 0, 1))
        self.assertEqual(code.co_flags & 0x0C, 0)
        self.assertEqual(code.co_varnames[:4],
                         ('proof_export', 'setup_record', 'expiry', 'policy_id'))
        self.assertIsNone(prepare.__defaults__)
        self.assertIsNone(prepare.__kwdefaults__)
        r2 = FIX['r2']
        self.assertIs(r2.__dict__.get('host_codec'), FIX['host'])
        self.assertIs(r2.__dict__.get('hashlib'), hashlib)
        for name in HOST_APIS:
            with self.subTest('T01', api=name):
                self.assertNotIn(name, r2.__dict__)
        public = sorted(name for name, value in r2.__dict__.items()
                        if not name.startswith('_') and callable(value)
                        and type(value) is not types.ModuleType)
        self.assertEqual(public, ['prepare_c2_issue'])
        # r2 neither holds nor replaces the v1 module.
        self.assertFalse(any(value is FIX['v1'] for value in r2.__dict__.values()))
        # Re-execute the r2 bytes against a counting host proxy; restore sys.modules.
        counts = dict.fromkeys(HOST_APIS, 0)
        proxy = types.ModuleType('host_codec')
        proxy.__dict__.update(_globals(FIX['host']))

        def counting(name, api):
            def wrapper(*args, **kwargs):
                counts[name] += 1
                return api(*args, **kwargs)
            return wrapper

        for name in HOST_APIS:
            setattr(proxy, name, counting(name, getattr(FIX['host'], name)))
        sys.modules['host_codec'] = proxy
        try:
            module = _module('c2_issue_artifacts_r2', FIX['r2_bytes'])
        finally:
            sys.modules['host_codec'] = FIX['host']
        self.assertIs(sys.modules['host_codec'], FIX['host'])
        self.assertIs(module.__dict__.get('host_codec'), proxy)
        counted = module.__dict__.get('prepare_c2_issue')
        self.assertIs(type(counted), types.FunctionType)
        result = counted(*_inputs(), HIST_EXPIRY, policy_id=POLICY)
        self.assertEqual(result['envCborHex'], HIST_ENV_CBOR)
        for name in HOST_APIS:
            with self.subTest('T01', called=name):
                self.assertGreater(counts[name], 0)

    def test_T02_k6_admitted_cbor_labels_keys_expiry_policy_matrix(self):
        prepare = _prepare()
        heads = ((0, '00'), (23, '17'), (24, '1818'), (1 << 32, '1b0000000100000000'),
                 (HIST_EXPIRY, '1b000001b8dac5b400'), ((1 << 64) - 1, '1bffffffffffffffff'))
        # NON-ORACLE consistency: hashlib over the literal reproduces the historical token.
        self.assertEqual(_token(HIST_ENV_CBOR), HIST_TOKEN)
        for policy_hex in (HIST_POLICY, '01' * 28):
            policy = bytes.fromhex(policy_hex)
            for expiry, head in heads:
                with self.subTest('T02', expiry=expiry, policy=policy_hex):
                    export, setup = _inputs()
                    vk_hex, proof_hex = setup['verifyingKeyHex'], export['proofHex']
                    result = prepare(export, setup, expiry, policy_id=policy)
                    env = ENV_PREFIX + head + 'ff'
                    if expiry == HIST_EXPIRY:
                        self.assertEqual(env, HIST_ENV_CBOR)
                        token = HIST_TOKEN
                    else:
                        token = _token(env)
                    self.assertIs(type(result), dict)
                    self.assertEqual(len(result), 12)
                    self.assertEqual(sorted(result), sorted(RESULT_KEYS))
                    self.assertEqual(result['format'], RESULT_FORMAT)
                    self.assertIs(type(result['labels']), dict)
                    self.assertEqual(len(result['labels']), 16)
                    self.assertEqual(result['labels'], LABELS)
                    self.assertEqual(result['envCborHex'], env)
                    self.assertEqual(result['envelopeTokenNameHex'], token)
                    self.assertEqual(result['issueCborHex'], _issue_cbor(vk_hex, proof_hex))
                    self.assertEqual(len(result['issueCborHex']), 1420)
                    self.assertEqual(result['mintValueCborHex'], _mint_cbor(policy_hex, token))
                    self.assertTrue(_same(result['envData'], _env(expiry)))
                    self.assertTrue(_same(result['issueData'], ('constr', 0, [
                        ('bytes', bytes.fromhex(vk_hex)), ('bytes', bytes.fromhex(proof_hex))])))
                    self.assertTrue(_same(result['mintValueData'], ('map', [
                        (('bytes', policy), ('map', [(('bytes', bytes.fromhex(token)),
                                                      ('int', 1))]))])))
                    self.assertIs(type(result['expiry']), int)
                    self.assertEqual(result['expiry'], expiry)
                    self.assertEqual(result['policyIdHex'], policy_hex)
                    self.assertEqual(result['setupRecordSha256Declared'], SETUP_SHA256)
                    for key in BYTE_FIELDS + ('policyIdHex', 'setupRecordSha256Declared'):
                        self.assertIs(type(result[key]), str)
                        self.assertLessEqual(set(result[key]), HEX_DIGITS)
                    v1 = FIX['v1'].prepare_c2_issue(*_inputs(), expiry, policy_id=policy)
                    self.assertEqual(v1['format'], V1_FORMAT)
                    for key in BYTE_FIELDS:
                        self.assertEqual(result[key], v1[key], key)

    def test_T03_argument_types_refused_before_records_are_touched(self):
        prepare = _prepare()
        export, setup = _inputs()
        before = copy.deepcopy((export, setup))
        hooked_export, hooked_setup = _Hooked(export), _Hooked(setup)
        records = (
            ('export list', [], setup), ('export None', None, setup),
            ('export str', FIX['proof'].decode('utf-8'), setup),
            ('export _Hooked', hooked_export, setup),
            ('setup list', export, []), ('setup None', export, None),
            ('setup str', export, FIX['setup'].decode('utf-8')),
            ('setup _Hooked', export, hooked_setup))
        for name, proof_export, setup_record in records:
            with self.subTest('T03', case=name):
                self.refused('C2I-ARG', proof_export, setup_record, prepare=prepare)
                self.v1_refused('C2I-ARG', proof_export, setup_record)
        for name, r2, v1, changes, expiry, policy in ARG_CASES:
            with self.subTest('T03', case=name):
                self.assertEqual((r2, v1, changes), ('ARG', 'ARG', ()))
                self.refused('C2I-ARG', export, setup, expiry, policy, prepare=prepare)
                self.v1_refused('C2I-ARG', export, setup, expiry, policy)
        # Records untouched: unchanged values, and the hooked wrappers' own keys
        # read through dict itself, never through a hook.
        self.assertTrue(_same((export, setup), before))
        self.assertEqual(sorted(dict.keys(hooked_export)), sorted(before[0]))
        self.assertEqual(sorted(dict.keys(hooked_setup)), sorted(before[1]))

    def test_T04_setup_record_refused_at_setup(self):
        _prepare()
        for case in T04_CASES:
            self.check('T04', case)

    def test_T05_relation_profile_subtests_T05a_T05b_T05c(self):
        _prepare()
        self.assertEqual((len(T05A_CASES), len(T05B_CASES), len(T05C_CASES)), (34, 57, 1))
        for label, cases in (('T05a', T05A_CASES), ('T05b', T05B_CASES), ('T05c', T05C_CASES)):
            for case in cases:
                self.check(label, case)

    def test_T06_surrogate_code_points_by_stage_owner(self):
        _prepare()
        for case in T06_CASES:
            self.check('T06', case)

    def test_T07_export_closed_schema_and_exact_types(self):
        _prepare()
        for case in T07_CASES:
            self.check('T07', case)

    def test_T08_relation_identity(self):
        _prepare()
        for case in T08_CASES:
            self.check('T08', case)

    def test_T09_statement_framing(self):
        _prepare()
        for case in T09_CASES:
            self.check('T09', case)

    def test_T10_context_equals_header(self):
        _prepare()
        for case in T10_CASES:
            self.check('T10', case)

    def test_T11_digest_and_public_inputs(self):
        _prepare()
        for case in T11_CASES:
            self.check('T11', case)

    def test_T12_proof_length_and_hex(self):
        _prepare()
        for case in T12_CASES:
            self.check('T12', case)

    def test_T13_cardano_derivation(self):
        _prepare()
        for case in T13_CASES:
            self.check('T13', case)

    def test_T14_first_failing_stage_wins_for_all_nine_pairs(self):
        _prepare()
        self.assertEqual(len(T14_CASES), 9)
        # BUILD excluded as unreachable: no table case expects it.
        self.assertEqual(TAMPER['BUILD'], ())
        for case in T14_CASES:
            name, r2, _v1, changes, expiry, _policy = case
            first, second = name.split('/')
            self.assertEqual(r2, first)
            self.assertEqual(STAGES.index(first) + 1, STAGES.index(second))
            self.assertEqual(len(changes) + (expiry != HIST_EXPIRY), 2, name)
            self.check('T14', case)

    def test_T15_detachment_nonaliasing_and_source_effects(self):
        prepare = _prepare()
        module, host = FIX['r2'], FIX['host']
        names = sorted(module.__dict__)
        identities = {name: id(value) for name, value in module.__dict__.items()}
        state = copy.deepcopy({name: value for name, value in _globals(module).items()
                               if isinstance(value, (dict, list, set, bytearray, tuple,
                                                     frozenset))})
        host_identities = {name: id(value) for name, value in host.__dict__.items()}
        kept = []
        # Success: inputs equal their deep copies; the result holds no input dict or
        # list by identity; two results share no mutable container.
        export, setup = _inputs()
        kept.append((export, setup))
        before = copy.deepcopy((export, setup))
        first = prepare(export, setup, HIST_EXPIRY, policy_id=POLICY)
        second = prepare(export, setup, HIST_EXPIRY, policy_id=POLICY)
        self.assertTrue(_same((export, setup), before))
        inputs = _containers((export, setup))
        first_ids, second_ids = _containers(first), _containers(second)
        self.assertFalse(inputs & first_ids)
        self.assertFalse(inputs & second_ids)
        self.assertFalse(first_ids & second_ids)
        self.assertEqual(first, second)
        # Mutating one result changes no later result.
        first['labels'].clear()
        first['envData'][2].clear()
        first['issueData'][2].clear()
        first['mintValueData'][1].clear()
        third = self.call(*_inputs())
        self.assertEqual(third['labels'], LABELS)
        self.assertTrue(_same(third['envData'], _env(HIST_EXPIRY)))
        self.assertEqual(third['envCborHex'], HIST_ENV_CBOR)
        # First-fault refusal of each T14 pair plus a CARDANO fault leaves both inputs
        # equal to their deep copies.
        for name, r2, _v1, changes, expiry, policy in T14_CASES + (CASE_CARDANO_ANCHOR,):
            with self.subTest('T15', case=name):
                export, setup = _mutated(changes)
                kept.append((export, setup))
                before = copy.deepcopy((export, setup))
                self.refused('C2I-' + r2, export, setup, expiry, policy, prepare=prepare)
                self.assertTrue(_same((export, setup), before))
        # Module namespace keys and mutable globals unchanged, holding no input container.
        self.assertEqual(sorted(module.__dict__), names)
        self.assertEqual({name: id(value) for name, value in module.__dict__.items()},
                         identities)
        for name, value in state.items():
            with self.subTest('T15', global_=name):
                self.assertEqual(module.__dict__[name], value)
        held = _containers(tuple(_globals(module).values()))
        for pair in kept:
            self.assertFalse(held & _containers(pair))
        self.assertFalse(held & (first_ids | second_ids))
        # The host namespace is not patched.
        self.assertEqual({name: id(value) for name, value in host.__dict__.items()},
                         host_identities)
        self.assertIs(sys.modules.get('host_codec'), host)
        # Effect-token scan (v1 test_11 tokens plus 'global'). CP04 scan constraints
        # (no str.encode, no input-proportional copy, length before scan) are not
        # discriminated by any outcome here and require later source inspection.
        text = FIX['r2_bytes'].decode('utf-8')
        for token in EFFECT_TOKENS:
            with self.subTest('T15', token=token):
                self.assertNotIn(token, text)

    def test_T16_strictness_only_v1_admits_r2_refuses(self):
        _prepare()
        # Dedicated strictness table: v1 returns /1 with 8 labels, r2 refuses at the
        # listed stage. Not evidence of the CP06 subset property (T02 and T17 sample it).
        self.assertEqual(len(T16_CASES), 65)
        for case in T16_CASES:
            self.assertEqual(case[2], ADMIT, case[0])
            self.assertIn(case[1], ('SETUP', 'PROFILE'), case[0])
            self.check('T16', case)

    def test_T17_synthetic_changed_context_both_admit(self):
        prepare = _prepare()

        def synthetic_changed_context():
            # SYNTHETIC_NOT_PROOF_NOT_ORACLE public test fixture derived from K6: not a
            # proof (proofHex is unchanged and proves nothing), not an oracle and not
            # authenticated ledger data. statementDigestBlake2b256 and x1/x2 are computed
            # here by hashlib.blake2b(digest_size=32) and int.from_bytes, never by the host.
            export, setup = _inputs()
            statement = bytearray.fromhex(export['statementHex'])
            statement[144:172] = bytes.fromhex('66' * 28)
            statement[173:201] = bytes.fromhex('b3' * 28)
            statement[233:265] = bytes.fromhex('33' * 32)
            statement[265:267] = (7).to_bytes(2, 'big')
            statement[267:275] = (2).to_bytes(8, 'big')
            statement = bytes(statement)
            digest = hashlib.blake2b(statement, digest_size=32).digest()
            export['statementHex'] = statement.hex()
            export['statementDigestBlake2b256'] = digest.hex()
            export['context'].update(claimantVkh='66' * 28, settleHash='b3' * 28,
                                     consumedTxid='33' * 32, consumedIx=7, seq=2)
            export['publicInputs'].update(x1=str(int.from_bytes(digest[:16], 'big')),
                                          x2=str(int.from_bytes(digest[16:], 'big')))
            env_v2 = export['cardano']['envV2']
            env_v2['claimantVkh'] = '66' * 28
            env_v2['settleOutref'].update(transactionId='33' * 32, outputIndex=7)
            env_v2['statementDigest'] = digest.hex()
            export['cardano']['intentAnchor'] = 'cardano-plutus-v3:' + 'b3' * 28
            return export, setup, statement, digest.hex()

        export, setup, statement, digest = synthetic_changed_context()
        self.assertEqual(SYNTHETIC, 'SYNTHETIC_NOT_PROOF_NOT_ORACLE')
        self.assertEqual(len(statement), 343 + 378)
        self.assertEqual(statement[172], 0)
        self.assertNotEqual(digest, STMTD)
        # Literal framing segments around the synthetic values; HIST_EXPIRY and
        # HIST_POLICY are explicit. The token is the NON-ORACLE hashlib expectation.
        env = ''.join(('d8799f', '02', '00', '5820' + FAMILY.hex(), '5820' + PRED,
                       '5820' + VKD, '581c' + '66' * 28,
                       'd8799f5820' + '33' * 32 + '07ff', '5820' + digest,
                       '1b000001b8dac5b400', 'ff'))
        token = _token(env)
        policy = bytes.fromhex(HIST_POLICY)
        before = copy.deepcopy((export, setup))
        result = prepare(export, setup, HIST_EXPIRY, policy_id=policy)
        self.assertTrue(_same((export, setup), before))
        self.assertIs(type(result), dict)
        self.assertEqual(sorted(result), sorted(RESULT_KEYS))
        self.assertEqual(result['format'], RESULT_FORMAT)
        self.assertEqual(result['labels'], LABELS)
        self.assertEqual(result['envCborHex'], env)
        self.assertEqual(result['envelopeTokenNameHex'], token)
        self.assertEqual(result['issueCborHex'],
                         _issue_cbor(setup['verifyingKeyHex'], export['proofHex']))
        self.assertEqual(result['mintValueCborHex'], _mint_cbor(HIST_POLICY, token))
        self.assertTrue(_same(result['envData'], _env(HIST_EXPIRY, '66' * 28, '33' * 32, 7,
                                                      digest)))
        v1_export, v1_setup = synthetic_changed_context()[:2]
        v1 = self.v1_admitted(v1_export, v1_setup, HIST_EXPIRY, policy)
        for key in BYTE_FIELDS:
            self.assertEqual(result[key], v1[key], key)


if __name__ == '__main__':
    unittest.main(verbosity=2)