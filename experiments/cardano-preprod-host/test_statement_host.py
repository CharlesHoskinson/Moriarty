"""Independent stdlib tests for the three statement-host APIs of host_codec.py.

Scope: parse_statement_header, statement_digest_hex and statement_public_inputs
over the retained public K6 fixture and locally edited copies of its statement.
The tests compare host behaviour with the inspected Rust/Aiken ABI v2 source
profile only. They verify no proof, signature, VK correspondence, program
semantics or ledger context. The fixture context is caller-declared and its
single-party test setup is forgeable by whoever ran it.
"""

import hashlib
import importlib.util
import json
import os
import stat
import sys
import types
import unittest

sys.dont_write_bytecode = True

HERE = os.path.dirname(os.path.abspath(__file__))
HOST_NAME = "host_codec.py"
FIXTURE_NAME = "K6-PUBLIC-PROOF.json"

# Provenance pins from the frozen public index. SHA-256 is used only for these.
FIXTURE_BYTES = 5800
FIXTURE_SHA256 = "930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b"
HOST_PREFIX_BYTES = 16134
HOST_PREFIX_SHA256 = "8e15bc783afc7c8c96268ea1ade0f881e631d9397a59d9642d4b2a1bbc62a838"
HOST_READ_LIMIT = 4194304

# ABI v2 constants: abi2.rs STMT_TAG, FAMILY_V2, HEADER_LEN; claim.rs MAX_STATEMENT_BYTES.
STMT_TAG = b"MRG16-PVC-STMT-2"
FAMILY_V2 = b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2"
HEADER_LEN = 343
MAX_STATEMENT_BYTES = 262144

# Returned header fields: key, zero-based offset, width in bytes, kind. Integers
# are unsigned big-endian (claim.rs offsets and parse_statement, abi2.rs
# ContextV2::bytes and statement_bytes, core.ak statement).
LAYOUT = (
    ("predicateDigest", 48, 32, "hex"),
    ("shapeDigest", 80, 32, "hex"),
    ("vkDigest", 112, 32, "hex"),
    ("claimantVkh", 144, 28, "hex"),
    ("networkId", 172, 1, "int"),
    ("settleHash", 173, 28, "hex"),
    ("threadId", 201, 32, "hex"),
    ("consumedTxid", 233, 32, "hex"),
    ("consumedIx", 265, 2, "int"),
    ("seq", 267, 8, "int"),
    ("catalogueDigest", 275, 32, "hex"),
    ("intentsDigest", 307, 32, "hex"),
    ("bodyLen", 339, 4, "int"),
)
RESULT_KEYS = frozenset([entry[0] for entry in LAYOUT] + ["body"])
CONTEXT_KEYS = frozenset([
    "vkDigest", "claimantVkh", "networkId", "settleHash", "threadId",
    "consumedTxid", "consumedIx", "seq", "catalogueDigest", "intentsDigest",
])
API_NAMES = ("parse_statement_header", "statement_digest_hex", "statement_public_inputs")
LOWER_HEX = frozenset("0123456789abcdef")

# Literals copied from the retained public fixture; never derived by the host under test.
STATEMENT_BYTES = 721
BODY_LEN = 378
DIGEST = "c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc"
X1 = "268418678883876601707409168117995592842"
X2 = "8714689549842776844379904284721120460"
EXPECTED_HEADER = {
    "predicateDigest": "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd",
    "shapeDigest": "36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969",
    "vkDigest": "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd",
    "claimantVkh": "55" * 28,
    "networkId": 0,
    "settleHash": "a2" * 28,
    "threadId": "7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a",
    "consumedTxid": "22" * 32,
    "consumedIx": 0,
    "seq": 1,
    "catalogueDigest": "37de4837bc1417d65df0c40b4829d796b04d97fd037bb128fc0814abb5646e18",
    "intentsDigest": "b8e2f734456ad3d939e91441330cb0635707e09d70af45edead18923108fc308",
    "bodyLen": 378,
}

_HOST_CACHE = []


def _read_regular(path, limit):
    flags = os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0) | getattr(os, "O_CLOEXEC", 0)
    fd = os.open(path, flags)
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise AssertionError(path + " is not a regular file")
        chunks = []
        total = 0
        while True:
            chunk = os.read(fd, 65536)
            if not chunk:
                break
            total += len(chunk)
            if total > limit:
                raise AssertionError(path + " exceeds its read bound")
            chunks.append(chunk)
    finally:
        os.close(fd)
    return b"".join(chunks)


def _load_fixture():
    path = os.path.join(HERE, FIXTURE_NAME)
    if os.path.islink(path) or not os.path.isfile(path):
        raise AssertionError(FIXTURE_NAME + " must be a regular non-symlink sibling file")
    raw = _read_regular(path, FIXTURE_BYTES)
    if len(raw) != FIXTURE_BYTES or hashlib.sha256(raw).hexdigest() != FIXTURE_SHA256:
        raise AssertionError(FIXTURE_NAME + " is not the pinned public fixture")
    return raw


def _require_host_path(case):
    path = os.path.join(HERE, HOST_NAME)
    if not os.path.lexists(path):
        case.fail("missing implementation: sibling " + HOST_NAME + " is absent")
    if not stat.S_ISREG(os.lstat(path).st_mode):
        case.fail(HOST_NAME + " must be a regular non-symlink file")
    return path


def _host(case):
    if not _HOST_CACHE:
        path = _require_host_path(case)
        spec = importlib.util.spec_from_file_location("host_codec", path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        _HOST_CACHE.append(module)
    return _HOST_CACHE[0]


def _apis(case, *names):
    # Look up every requested API before any call, never via a module __getattr__.
    namespace = vars(_host(case))
    functions = []
    for name in names:
        function = namespace.get(name)
        if function is None:
            case.fail("missing implementation: host_codec." + name)
        case.assertIs(type(function), types.FunctionType, name + " must be a plain function")
        functions.append(function)
    return tuple(functions)


def _edit(statement, offset, replacement):
    return statement[:offset] + replacement + statement[offset + len(replacement):]


def _flip(statement, offset):
    return _edit(statement, offset, bytes([statement[offset] ^ 0x01]))


def _framed(header, body):
    # Rewrites bodyLen only. Structural boundary test data, not a program statement.
    return header[:339] + len(body).to_bytes(4, "big") + body


def _header_from_fields(fields):
    out = STMT_TAG + FAMILY_V2
    for name, offset, width, kind in LAYOUT:
        if len(out) != offset:
            raise AssertionError("layout is not contiguous at " + name)
        value = fields[name]
        out += bytes.fromhex(value) if kind == "hex" else value.to_bytes(width, "big")
    return out


class StatementHostTest(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.fixture_raw = _load_fixture()
        cls.fixture = json.loads(cls.fixture_raw.decode("utf-8"))
        cls.statement = bytes.fromhex(cls.fixture["statementHex"])

    def refuse_all(self, functions, statement, label):
        for function in functions:
            with self.assertRaises(ValueError, msg=label) as caught:
                function(statement)
            self.assertIs(type(caught.exception), ValueError, label)

    def assert_header_shape(self, result):
        self.assertIs(type(result), dict)
        self.assertEqual(set(result), RESULT_KEYS)
        for name, _offset, width, kind in LAYOUT:
            value = result[name]
            if kind == "hex":
                self.assertIs(type(value), str, name)
                self.assertEqual(len(value), 2 * width, name)
                self.assertLessEqual(set(value), LOWER_HEX, name)
            else:
                self.assertIs(type(value), int, name)
                self.assertTrue(0 <= value < 1 << (8 * width), name)
        self.assertIs(type(result["body"]), bytes)
        self.assertEqual(len(result["body"]), result["bodyLen"])

    def assert_digest_shape(self, value):
        self.assertIs(type(value), str)
        self.assertEqual(len(value), 64)
        self.assertLessEqual(set(value), LOWER_HEX)

    def assert_public_pair(self, pair, digest):
        self.assertIs(type(pair), tuple)
        self.assertEqual(len(pair), 2)
        for text in pair:
            self.assertIs(type(text), str)
            self.assertTrue(1 <= len(text) <= 39 and text.isascii() and text.isdigit(), text)
            self.assertTrue(text == "0" or text[0] != "0", text)
            self.assertLess(int(text), 1 << 128)
        # Big-endian 128-bit halves, as claim.rs digest_halves.
        self.assertEqual(int(pair[0]).to_bytes(16, "big").hex(), digest[:32])
        self.assertEqual(int(pair[1]).to_bytes(16, "big").hex(), digest[32:])

    def boundary_statement(self, body_len):
        # Structural boundary test data only: the genuine header with an edited
        # bodyLen and repeated genuine body bytes; not a program or ledger statement.
        body = self.statement[HEADER_LEN:]
        repeated = body * (body_len // len(body) + 1)
        return _framed(self.statement[:HEADER_LEN], repeated[:body_len])

    def test_public_fixture_pin_and_literal_consistency(self):
        # Fixture-only check: needs no host API.
        self.assertEqual(len(self.fixture_raw), FIXTURE_BYTES)
        self.assertEqual(hashlib.sha256(self.fixture_raw).hexdigest(), FIXTURE_SHA256)
        fixture = self.fixture
        self.assertEqual(fixture["format"], "moriarty-k2-groth16-proof/1")
        self.assertEqual(len(self.statement), STATEMENT_BYTES)
        self.assertEqual(len(self.statement), HEADER_LEN + BODY_LEN)
        self.assertEqual(fixture["statementDigestBlake2b256"], DIGEST)
        self.assertEqual(fixture["cardano"]["envV2"]["statementDigest"], DIGEST)
        self.assertEqual(fixture["cardano"]["envV2"]["familyId"], FAMILY_V2.hex())
        self.assertEqual(fixture["publicInputs"], {"x1": X1, "x2": X2})
        self.assertEqual(int(X1).to_bytes(16, "big").hex(), DIGEST[:32])
        self.assertEqual(int(X2).to_bytes(16, "big").hex(), DIGEST[32:])
        self.assertEqual(set(fixture["context"]), CONTEXT_KEYS)
        for key, value in fixture["context"].items():
            self.assertEqual(EXPECTED_HEADER[key], value, key)
        self.assertEqual(fixture["predicateDigest"], EXPECTED_HEADER["predicateDigest"])
        self.assertEqual(fixture["abi2ShapeDigest"], EXPECTED_HEADER["shapeDigest"])
        # The source offset table reproduces the genuine fixed header from fixture literals.
        self.assertEqual(_header_from_fields(EXPECTED_HEADER), self.statement[:HEADER_LEN])

    def test_host_prefix_preserved_byte_for_byte(self):
        path = _require_host_path(self)
        source = _read_regular(path, HOST_READ_LIMIT)
        self.assertGreaterEqual(len(source), HOST_PREFIX_BYTES)
        self.assertEqual(hashlib.sha256(source[:HOST_PREFIX_BYTES]).hexdigest(), HOST_PREFIX_SHA256)

    def test_three_statement_apis_are_present_plain_functions(self):
        functions = _apis(self, *API_NAMES)
        self.assertEqual(len(functions), 3)

    def test_parse_genuine_fixture_header_fields(self):
        (parse,) = _apis(self, "parse_statement_header")
        result = parse(self.statement)
        self.assert_header_shape(result)
        for key, value in EXPECTED_HEADER.items():
            self.assertEqual(result[key], value, key)
        for key, value in self.fixture["context"].items():
            self.assertEqual(result[key], value, key)
        self.assertEqual(result["predicateDigest"], self.fixture["predicateDigest"])
        self.assertEqual(result["shapeDigest"], self.fixture["abi2ShapeDigest"])
        self.assertEqual(result["bodyLen"], BODY_LEN)
        self.assertEqual(result["body"], self.statement[HEADER_LEN:])

    def test_parsed_fields_reconstruct_exact_fixed_header(self):
        # Fixed-header parity only; this is not the compiler re-encoding check.
        (parse,) = _apis(self, "parse_statement_header")
        result = parse(self.statement)
        self.assert_header_shape(result)
        header = _header_from_fields(result)
        self.assertEqual(len(header), HEADER_LEN)
        self.assertEqual(header, self.statement[:HEADER_LEN])
        self.assertEqual(header + result["body"], self.statement)

    def test_integer_fields_are_unsigned_big_endian(self):
        (parse,) = _apis(self, "parse_statement_header")
        # Locally edited copies of the genuine header: structural test data only.
        edited = _edit(self.statement, 172, bytes([0xfe]))
        edited = _edit(edited, 265, bytes([0x01, 0x02]))
        edited = _edit(edited, 267, bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]))
        result = parse(edited)
        self.assert_header_shape(result)
        self.assertEqual(result["networkId"], 0xfe)
        self.assertEqual(result["consumedIx"], 0x0102)
        self.assertEqual(result["seq"], 0x0102030405060708)
        self.assertEqual(result["bodyLen"], 0x0000017a)
        edited = _edit(self.statement, 265, bytes([0xff]) * 10)
        result = parse(edited)
        self.assert_header_shape(result)
        self.assertEqual(result["consumedIx"], 65535)
        self.assertEqual(result["seq"], (1 << 64) - 1)
        self.assertEqual(result["catalogueDigest"], EXPECTED_HEADER["catalogueDigest"])

    def test_digest_matches_published_fixture_literal(self):
        (digest_hex,) = _apis(self, "statement_digest_hex")
        value = digest_hex(self.statement)
        self.assert_digest_shape(value)
        self.assertEqual(value, DIGEST)
        self.assertEqual(value, self.fixture["statementDigestBlake2b256"])

    def test_public_inputs_are_published_big_endian_digest_halves(self):
        (public_inputs,) = _apis(self, "statement_public_inputs")
        pair = public_inputs(self.statement)
        self.assert_public_pair(pair, DIGEST)
        self.assertEqual(pair, (X1, X2))
        published = self.fixture["publicInputs"]
        self.assertEqual(pair, (published["x1"], published["x2"]))
        little = tuple(str(int.from_bytes(bytes.fromhex(half), "little"))
                       for half in (DIGEST[:32], DIGEST[32:]))
        self.assertNotEqual(pair, little)
        self.assertNotEqual(pair, (X2, X1))

    def test_wrong_tag_refused_by_all_three_apis(self):
        functions = _apis(self, *API_NAMES)
        variants = (
            ("lowercase first byte", _edit(self.statement, 0, b"m")),
            ("statement tag version 1", _edit(self.statement, 15, b"1")),
            ("flipped bit at offset 7", _flip(self.statement, 7)),
            ("zero tag", _edit(self.statement, 0, bytes(16))),
            ("shape tag in tag position", _edit(self.statement, 0, b"MRG16-PVC-SHP-02")),
        )
        for label, statement in variants:
            self.assertEqual(len(statement), STATEMENT_BYTES)
            self.refuse_all(functions, statement, label)

    def test_wrong_family_refused_by_all_three_apis(self):
        functions = _apis(self, *API_NAMES)
        variants = (
            ("flipped bit at offset 16", _flip(self.statement, 16)),
            ("flipped bit at offset 31", _flip(self.statement, 31)),
            ("flipped bit at offset 47", _flip(self.statement, 47)),
            ("zero family", _edit(self.statement, 16, bytes(32))),
            ("lowercase family", _edit(self.statement, 16, FAMILY_V2.lower())),
        )
        for label, statement in variants:
            self.assertEqual(len(statement), STATEMENT_BYTES)
            self.refuse_all(functions, statement, label)

    def test_truncated_and_overlong_framing_refused(self):
        functions = _apis(self, *API_NAMES)
        statement = self.statement
        variants = (
            ("empty", b""),
            ("one byte", statement[:1]),
            ("tag only", statement[:16]),
            ("342 bytes", statement[:HEADER_LEN - 1]),
            ("header without its body", statement[:HEADER_LEN]),
            ("body short by one byte", statement[:-1]),
            ("one trailing byte", statement + bytes(1)),
            ("body repeated", statement + statement[HEADER_LEN:]),
        )
        for label, candidate in variants:
            self.refuse_all(functions, candidate, label)

    def test_body_length_field_mismatch_refused(self):
        functions = _apis(self, *API_NAMES)
        for field in (0, BODY_LEN - 1, BODY_LEN + 1, STATEMENT_BYTES,
                      BODY_LEN + (1 << 16), (1 << 32) - 1):
            statement = _edit(self.statement, 339, field.to_bytes(4, "big"))
            self.assertEqual(len(statement), STATEMENT_BYTES)
            self.refuse_all(functions, statement, "bodyLen field " + str(field))

    def test_structural_length_boundaries(self):
        parse, digest_hex, public_inputs = _apis(self, *API_NAMES)
        empty = self.boundary_statement(0)
        self.assertEqual(len(empty), HEADER_LEN)
        result = parse(empty)
        self.assert_header_shape(result)
        self.assertEqual(result["bodyLen"], 0)
        self.assertEqual(result["body"], b"")
        value = digest_hex(empty)
        self.assert_digest_shape(value)
        self.assert_public_pair(public_inputs(empty), value)

        maximal = self.boundary_statement(MAX_STATEMENT_BYTES - HEADER_LEN)
        self.assertEqual(len(maximal), MAX_STATEMENT_BYTES)
        result = parse(maximal)
        self.assert_header_shape(result)
        self.assertEqual(result["bodyLen"], MAX_STATEMENT_BYTES - HEADER_LEN)
        self.assertEqual(result["body"], maximal[HEADER_LEN:])
        for key, expected in EXPECTED_HEADER.items():
            if key != "bodyLen":
                self.assertEqual(result[key], expected, key)
        value = digest_hex(maximal)
        self.assert_digest_shape(value)
        self.assert_public_pair(public_inputs(maximal), value)

        over = self.boundary_statement(MAX_STATEMENT_BYTES - HEADER_LEN + 1)
        self.assertEqual(len(over), MAX_STATEMENT_BYTES + 1)
        self.refuse_all((parse, digest_hex, public_inputs), over, "one byte over the cap")

    def test_far_oversized_inputs_refused(self):
        functions = _apis(self, *API_NAMES)
        header = self.statement[:HEADER_LEN]
        self.refuse_all(functions, _framed(header, bytes(4194304)), "consistently framed 4 MiB body")
        self.refuse_all(functions, self.statement + bytes(1048576), "1 MiB of trailing bytes")
        self.refuse_all(functions, bytes(MAX_STATEMENT_BYTES + 1), "zero bytes over the cap")

    def test_non_bytes_inputs_refused(self):
        functions = _apis(self, *API_NAMES)
        statement = self.statement
        candidates = (
            ("bytearray", bytearray(statement)),
            ("memoryview", memoryview(statement)),
            ("hex text", statement.hex()),
            ("latin-1 text", statement.decode("latin-1")),
            ("list of ints", list(statement)),
            ("tuple holding bytes", (statement,)),
            ("None", None),
            ("bool", True),
            ("int", STATEMENT_BYTES),
        )
        for label, candidate in candidates:
            self.refuse_all(functions, candidate, label)

    def test_bytes_subclass_refused_without_invoking_hooks(self):
        functions = _apis(self, *API_NAMES)
        calls = []

        class HookedBytes(bytes):
            def __len__(self):
                calls.append("__len__")
                return bytes.__len__(self)

            def __getitem__(self, index):
                calls.append("__getitem__")
                return bytes.__getitem__(self, index)

            def __eq__(self, other):
                calls.append("__eq__")
                return bytes.__eq__(self, other)

            def __ne__(self, other):
                calls.append("__ne__")
                return bytes.__ne__(self, other)

            __hash__ = bytes.__hash__

            def __buffer__(self, flags):
                calls.append("__buffer__")
                raise AssertionError("caller buffer hook invoked")

        hooked = HookedBytes(self.statement)
        self.refuse_all(functions, hooked, "bytes subclass with genuine content")
        self.assertEqual(calls, [])

    def test_body_bytes_are_opaque_to_header_parsing(self):
        parse, digest_hex, public_inputs = _apis(self, *API_NAMES)
        # Header-valid structural test data: accepting these bodies shows only that
        # body bytes are outside host validation, not that they suit any program.
        for filler in (0x00, 0xff):
            body = bytes([filler]) * BODY_LEN
            edited = self.statement[:HEADER_LEN] + body
            result = parse(edited)
            self.assert_header_shape(result)
            for key, expected in EXPECTED_HEADER.items():
                self.assertEqual(result[key], expected, key)
            self.assertEqual(result["body"], body)
            value = digest_hex(edited)
            self.assert_digest_shape(value)
            self.assertNotEqual(value, DIGEST)
            pair = public_inputs(edited)
            self.assert_public_pair(pair, value)
            self.assertNotEqual(pair, (X1, X2))

    def test_context_and_relation_fields_reported_not_checked(self):
        parse, digest_hex = _apis(self, "parse_statement_header", "statement_digest_hex")
        # Edited header copies: the host reports caller-declared bytes and checks no
        # descriptor, VK, catalogue or ledger value.
        edited = _edit(self.statement, 48, bytes(32))
        edited = _edit(edited, 112, bytes([0x11]) * 32)
        edited = _edit(edited, 144, bytes([0x66]) * 28)
        edited = _edit(edited, 172, bytes([0x01]))
        result = parse(edited)
        self.assert_header_shape(result)
        self.assertEqual(result["predicateDigest"], "00" * 32)
        self.assertEqual(result["vkDigest"], "11" * 32)
        self.assertEqual(result["claimantVkh"], "66" * 28)
        self.assertEqual(result["networkId"], 1)
        for key in ("shapeDigest", "settleHash", "threadId", "consumedTxid", "consumedIx",
                    "seq", "catalogueDigest", "intentsDigest", "bodyLen"):
            self.assertEqual(result[key], EXPECTED_HEADER[key], key)
        self.assertEqual(result["body"], self.statement[HEADER_LEN:])
        self.assertNotEqual(digest_hex(edited), DIGEST)

    def test_results_deterministic_and_detached(self):
        parse, digest_hex, public_inputs = _apis(self, *API_NAMES)
        first = parse(self.statement)
        second = parse(self.statement)
        self.assertEqual(first, second)
        self.assertIsNot(first, second)
        first["seq"] = 99
        first["body"] = b""
        first["extra"] = True
        self.assertEqual(parse(self.statement), second)
        self.assertEqual(second["seq"], EXPECTED_HEADER["seq"])
        self.assertEqual(digest_hex(self.statement), digest_hex(self.statement))
        self.assertEqual(public_inputs(self.statement), public_inputs(self.statement))
        self.assertEqual(self.statement.hex(), self.fixture["statementHex"])


if __name__ == "__main__":
    unittest.main()
