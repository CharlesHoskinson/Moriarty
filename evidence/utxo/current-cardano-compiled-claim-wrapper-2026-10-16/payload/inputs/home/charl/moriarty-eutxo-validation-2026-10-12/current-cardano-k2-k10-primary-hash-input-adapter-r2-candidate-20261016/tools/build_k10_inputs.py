#!/usr/bin/env python3
"""K10 DATA-only input adapter (source-only, no execution of any validator).

R2 revision: the script policy is now INDEPENDENTLY RECOMPUTED from the
pinned compiledCode, grounded in the exact Pallas 0.35.1 source acquired
in the mirror, and the tool-declared artifact hash is used only as a
cross-check that must agree or generation fails closed. See
DESIGN-POSITIONS.json clauses K10-C2/K10-C3 (R2) and K10R2-C9/C10.

Builds, from pinned public mirror inputs only, the exact inputs needed by a
future separately-authorized native step that would run the already compiled
APPLIED B9 C2 minting wrapper under the pinned Aiken `uplc eval --cbor`
interface:

  * script.hex          - verbatim compiledCode of title c2.c2.mint
  * contexts/*.uplc.txt - one `(con data (...))` PlutusV3 ScriptContext term
                          per frozen test (parser grammar: uplc parser.rs
                          constant_data / data rules)
  * manifest.json       - pins, envelope CBOR preimages, token names, the
                          claimed and independently recomputed script hash,
                          and the source-derived expectation for every test

Hash algorithm (source-grounded, not guessed):
  pallas-traverse 0.35.1 src/hashes.rs:85-89 --
    impl ComputeHash<28> for PlutusScript<VERSION>:
      Hasher::<224>::hash_tagged(&self.0, VERSION as u8)
  pallas-crypto 0.35.1 src/hash/hasher.rs:59,73-78 -- Hasher<224> is
    Blake2b with a 28-byte digest; hash_tagged inputs one raw tag byte
    then the byte slice, with no CBOR encoding of its own.
  uplc ast.rs:151-187 -- compiled_code_and_hash builds
    conway::PlutusScript::<3>(program.to_cbor()) and Serialize writes
    compiledCode as hex of that same script byte slice and hash from it.
  Therefore the hashed slice is exactly the compiledCode byte slice:
    recomputed = blake2b-224(0x03 || compiledCode bytes), digest_size=28.
  The compiledCode is first validated as exactly one definite CBOR byte
  string (strict length, no trailing bytes) as a structural guard; the
  inner flat payload is NOT hashed, and no second CBOR layer is added.

Token name: blake2b-256 of the PlutusData CBOR of the EnvV2 datum
(claim.envelope_name). The minimal encoder below is grounded in stdlib
cbor.ak (serialise examples + decoder), uplc runtime
convert_constr_to_tag, and cross-checked against pallas-primitives
0.35.1 src/plutus_data.rs (Constr tag+fields encode; BoundedBytes:
definite bytes for <=64 bytes, indefinite 64-byte chunks above). It
fails closed on byte strings longer than 64 bytes: no frozen EnvV2
field exceeds 32 bytes, so that path is unreachable for the seven
frozen tests.

Stdlib Python only. No network, no subprocess, no curve operations.
"""
import hashlib
import json
import re
import sys
from pathlib import Path

# ---- pinned public inputs (SHA-256 from INPUT-INDEX.json) -------------------
PIN_B9_APPLIED = "10baa1794e0ae286cd780978408552fc0e994426367c2b92117e2c4b46ad1d36"
PIN_K8_FIXTURE = "1b9e2a9a2eb6a2b47915bfc9aacd3b68a59d76f189f98da5615b4c653923ef64"
EXPECTED_OUTPUT_BASENAME = "current-cardano-k2-k10-public-inputs-generated-20261016"
TOOL_DECLARED_C2_HASH = "6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1"
C2_TITLES = ("c2.c2.mint", "c2.c2.spend", "c2.c2.else")
COMPILED_BYTES_LEN = 2770  # CBOR 0x59 0x0acf (2767) + 3 header bytes
INNER_FLAT_LEN = 2767      # payload of the single CBOR byte string
PLUTUS_V3_TAG = 3          # hashes.rs: VERSION as u8 for PlutusScript<3>
SCRIPT_HASH_BYTES = 28     # ComputeHash<28> / Hasher<224>

# Audit-only Pallas/Aiken source pins (SHA-256 from the Pallas
# SOURCE-MANIFEST.json and INPUT-INDEX.json). These are citations recorded
# in the generated manifest for the fresh non-author audit; they are NOT
# runtime inputs of this generator and are never read by it.
AUDIT_SOURCE_PINS = [
    {"role": "pallas-traverse hashes.rs ComputeHash<28> PlutusScript",
     "sha256": "eecfb1f4bf763673087e7a2764ab1aa25fafe918d5e0442ecd94933a6b26c6ab"},
    {"role": "pallas-crypto hasher.rs Hasher<224> hash_tagged",
     "sha256": "80fb90446b39458957aa47be290d674a8edcee86c9a5ab4b5efdbd916d30c37e"},
    {"role": "pallas-primitives lib.rs PlutusScript transparent Bytes",
     "sha256": "a7c70f2594c8b9335d48c4bb38a7683662ce1a844f486546afa272a9cc90b601"},
    {"role": "pallas-primitives plutus_data.rs PlutusData encoding",
     "sha256": "adfebe4d9b53be5656bed1fce51e627fac0e3e1f14ee990dfe0af66bd26c924b"},
    {"role": "pallas-codec utils.rs Bytes transparent ByteVec",
     "sha256": "7019c76d3ba34edf028e7276a251248bd3bde2b5e759bd4f7847c394c6df798f"},
    {"role": "aiken uplc ast.rs compiled_code_and_hash/Serialize",
     "sha256": "d3d9009b93d71514017385b7b37fd3592c2c1d49585cd64d36db27bd7456dd7c"},
    {"role": "aiken uplc eval.rs uplc eval --cbor interface",
     "sha256": "2d63c16dd509842660c01d2f8ed3a712220d2dcd74d79bee51534055f33e2725"},
]

FAMILY_V2 = b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2"
PREDICATE_DIGEST = bytes.fromhex(
    "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd")
CLAIMANT_VKH = bytes.fromhex("55" * 28)          # fabricated local fixture only
SETTLE_TXID = bytes.fromhex("22" * 32)           # fabricated local fixture only
SETTLE_INDEX = 0
TEST_EXPIRY = 1893456000000                      # 2030-01-01, builder-chosen, unbound
ZERO_TXID = bytes(32)                            # placeholder transaction id


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def blake2b256(data: bytes) -> bytes:
    return hashlib.blake2b(data, digest_size=32).digest()


def blake2b224(data: bytes) -> bytes:
    # pallas-crypto Hasher<224>: Blake2b::new(224/8) (hasher.rs:59).
    return hashlib.blake2b(data, digest_size=SCRIPT_HASH_BYTES).digest()


def decode_single_cbor_bytes(raw: bytes) -> bytes:
    """Strictly decode exactly one definite CBOR byte string (major type 2).

    Structural guard only: the returned inner flat payload is validated
    for shape and is NOT the hashed slice (see module docstring). Fails
    closed on indefinite length, non-shortest-form length, truncation,
    or any trailing byte.
    """
    if not raw:
        raise ValueError("empty compiledCode bytes")
    first = raw[0]
    if first >> 5 != 2:
        raise ValueError("compiledCode is not a CBOR byte string")
    ai = first & 0x1F
    if ai < 24:
        length, offset = ai, 1
        if length < 24 and ai != length:
            raise ValueError("non-shortest CBOR length")
    elif ai == 24:
        if len(raw) < 2:
            raise ValueError("truncated CBOR length")
        length, offset = raw[1], 2
        if length < 24:
            raise ValueError("non-shortest CBOR length")
    elif ai == 25:
        if len(raw) < 3:
            raise ValueError("truncated CBOR length")
        length, offset = int.from_bytes(raw[1:3], "big"), 3
        if length < 256:
            raise ValueError("non-shortest CBOR length")
    elif ai == 26:
        if len(raw) < 5:
            raise ValueError("truncated CBOR length")
        length, offset = int.from_bytes(raw[1:5], "big"), 5
        if length < 65536:
            raise ValueError("non-shortest CBOR length")
    elif ai == 27:
        if len(raw) < 9:
            raise ValueError("truncated CBOR length")
        length, offset = int.from_bytes(raw[1:9], "big"), 9
        if length < 4294967296:
            raise ValueError("non-shortest CBOR length")
    else:
        raise ValueError("indefinite/reserved CBOR byte string not allowed")
    end = offset + length
    if end != len(raw):
        raise ValueError(
            "single CBOR byte string must consume compiledCode exactly "
            "(declared %d payload bytes, total %d, actual %d)"
            % (length, end, len(raw)))
    inner = raw[offset:end]
    if not inner:
        raise ValueError("empty inner script payload")
    return inner


def recompute_script_hash(compiled_raw: bytes) -> bytes:
    """Independently recompute the PlutusV3 script hash (policy id).

    Preimage per hashes.rs:85-89 + hasher.rs:73-78 is the single raw tag
    byte 0x03 followed by the PlutusScript byte slice, which ast.rs
    Serialize (ast.rs:179-187) shows is exactly the compiledCode slice.
    The strict CBOR decode above is a guard; the inner flat payload is
    not hashed and no double-CBOR re-wrap is hashed.
    """
    inner = decode_single_cbor_bytes(compiled_raw)
    if len(inner) != INNER_FLAT_LEN:
        raise ValueError("inner flat payload length mismatch")
    return blake2b224(bytes([PLUTUS_V3_TAG]) + compiled_raw)


# ---- minimal PlutusData model ------------------------------------------------
# ('I', int) ('B', bytes) ('Constr', index, [fields]) ('List', [items])
# ('Map', [(key, value)])
def I(n):
    return ("I", int(n))


def B(b):
    return ("B", bytes(b))


def Constr(index, fields):
    return ("Constr", int(index), list(fields))


def List(items):
    return ("List", list(items))


def Map(pairs):
    return ("Map", list(pairs))


# ---- CBOR for serialise_data (token-name preimage only) ----------------------
# Cross-checked against pallas-primitives plutus_data.rs: Constr encodes
# its CBOR tag then fields, BoundedBytes is definite for <=64 bytes and
# indefinite 64-byte chunks above; every frozen envelope field is <=32
# bytes, so only the definite path below is exercised (see >64 guard).
def _cbor_head(major: int, value: int) -> bytes:
    if value < 24:
        return bytes([(major << 5) | value])
    if value < 256:
        return bytes([(major << 5) | 24, value])
    if value < 65536:
        return bytes([(major << 5) | 25]) + value.to_bytes(2, "big")
    if value < 4294967296:
        return bytes([(major << 5) | 26]) + value.to_bytes(4, "big")
    return bytes([(major << 5) | 27]) + value.to_bytes(8, "big")


def _cbor_int(n: int) -> bytes:
    if n >= 0:
        return _cbor_head(0, n)
    return _cbor_head(1, -1 - n)


def _constr_tag(index: int) -> int:
    # uplc runtime convert_constr_to_tag; mirrors to_plutus_data wrap logic.
    if 0 <= index <= 6:
        return 121 + index
    if 7 <= index <= 127:
        return 1280 - 7 + index
    raise ValueError("constructor index out of sourced tag range")


def cbor_encode(data) -> bytes:
    kind = data[0]
    if kind == "I":
        return _cbor_int(data[1])
    if kind == "B":
        raw = data[1]
        if len(raw) > 64:
            raise ValueError(
                "byte string >64 bytes: serialise_data chunking is not "
                "independently sourced in this adapter (fail-closed)")
        return _cbor_head(2, len(raw)) + raw
    if kind == "List":
        items = data[1]
        if not items:
            return b"\x80"  # cbor.ak: serialise([]) == #"80"
        return b"\x9f" + b"".join(cbor_encode(x) for x in items) + b"\xff"
    if kind == "Map":
        pairs = data[1]
        return _cbor_head(5, len(pairs)) + b"".join(
            cbor_encode(k) + cbor_encode(v) for k, v in pairs)
    if kind == "Constr":
        index, fields = data[1], data[2]
        tag = _cbor_head(6, _constr_tag(index))
        if not fields:
            return tag + b"\x80"  # cbor.ak: serialise(None) == #"d87a80"
        return tag + b"\x9f" + b"".join(cbor_encode(f) for f in fields) + b"\xff"
    raise ValueError("unknown Data kind %r" % (kind,))


# ---- UPLC textual Data term (uplc parser.rs data rule) -----------------------
def uplc_term(data) -> str:
    kind = data[0]
    if kind == "I":
        return "I %d" % data[1]
    if kind == "B":
        return "B #%s" % data[1].hex()
    if kind == "Constr":
        return "Constr %d [%s]" % (
            data[1], ", ".join(uplc_term(f) for f in data[2]))
    if kind == "List":
        return "List [%s]" % ", ".join(uplc_term(x) for x in data[1])
    if kind == "Map":
        return "Map [%s]" % ", ".join(
            "(%s, %s)" % (uplc_term(k), uplc_term(v)) for k, v in data[1])
    raise ValueError("unknown Data kind %r" % (kind,))


# ---- envelope (EnvV2 datum) ---------------------------------------------------
def env_data(vk_digest: bytes, statement_digest: bytes):
    if len(FAMILY_V2) != 32:
        raise ValueError("family_id must be 32 bytes")
    outref = Constr(0, [B(SETTLE_TXID), I(SETTLE_INDEX)])
    return Constr(0, [
        I(2), I(0), B(FAMILY_V2), B(PREDICATE_DIGEST), B(vk_digest),
        B(CLAIMANT_VKH), outref, B(statement_digest), I(TEST_EXPIRY),
    ])


def envelope_cbor(vk_digest: bytes, statement_digest: bytes) -> bytes:
    return cbor_encode(env_data(vk_digest, statement_digest))


def token_name(vk_digest: bytes, statement_digest: bytes) -> bytes:
    return blake2b256(envelope_cbor(vk_digest, statement_digest))


# Frozen positive preimage, assembled field-by-field from the sources above.
# d8799f | 02 | 00 | 5820 family | 5820 predicate | 5820 vk_digest |
# 581c claimant | d8799f 5820 settle-txid 00 ff | 5820 statement_digest |
# 1b000001b8dac5b400 (expiry) | ff
def frozen_positive_cbor(vk_digest: bytes, statement_digest: bytes) -> bytes:
    return b"".join([
        bytes.fromhex("d8799f"), b"\x02", b"\x00",
        b"\x58\x20" + FAMILY_V2,
        b"\x58\x20" + PREDICATE_DIGEST,
        b"\x58\x20" + vk_digest,
        b"\x58\x1c" + CLAIMANT_VKH,
        bytes.fromhex("d8799f"), b"\x58\x20" + SETTLE_TXID, b"\x00\xff",
        b"\x58\x20" + statement_digest,
        bytes.fromhex("1b000001b8dac5b400"),
        b"\xff",
    ])


# ---- PlutusV3 ScriptContext (uplc tx/to_plutus_data.rs, V3 branch) -----------
def value_map(policy: bytes, name: bytes, qty: int):
    return Map([(B(policy), Map([(B(name), I(qty))]))])


def claim_output(policy: bytes, env, name: bytes, qty: int):
    address = Constr(0, [Constr(1, [B(policy)]), Constr(1, [])])
    datum = Constr(2, [env])  # InlineDatum
    return Constr(0, [address, value_map(policy, name, qty), datum,
                      Constr(1, [])])  # reference_script: None


def everything_interval():
    # interval.everything; infinite bounds carry is_inclusive True (Constr 1).
    lower = Constr(0, [Constr(0, []), Constr(1, [])])   # NegativeInfinity
    upper = Constr(0, [Constr(2, []), Constr(1, [])])   # PositiveInfinity
    return Constr(0, [lower, upper])


def script_context(policy: bytes, env, name: bytes, qty: int,
                   vk: bytes, proof: bytes):
    tx_info = Constr(0, [
        List([]),                                   # inputs
        List([]),                                   # reference_inputs
        List([claim_output(policy, env, name, qty)]),  # outputs
        I(0),                                       # fee (V3: bare Coin)
        value_map(policy, name, qty),               # mint
        List([]),                                   # certificates
        Map([]),                                    # withdrawals
        everything_interval(),                      # validity_range
        List([]),                                   # extra_signatories
        Map([]),                                    # redeemers (placeholder)
        Map([]),                                    # datums
        B(ZERO_TXID),                               # id (bare bytes in V3)
        Map([]),                                    # votes
        List([]),                                   # proposal_procedures
        Constr(1, []),                              # current_treasury: None
        Constr(1, []),                              # treasury_donation: None
    ])
    redeemer = Constr(0, [B(vk), B(proof)])         # C2Mint::Issue
    script_info = Constr(0, [B(policy)])            # ScriptInfo::Minting
    return Constr(0, [tx_info, redeemer, script_info])


# ---- pinned input extraction ---------------------------------------------------
def read_pinned(path: Path, expected_sha: str, label: str) -> bytes:
    raw = path.read_bytes()
    actual = sha256_hex(raw)
    if actual != expected_sha:
        raise ValueError("%s pin mismatch: %s != %s" % (label, actual, expected_sha))
    return raw


def extract_fixture(text: str):
    def const_bytes(name):
        m = re.search(r'pub const %s: ByteArray = #"([0-9a-f]+)"' % name, text)
        if not m:
            raise ValueError("fixture constant %s not found" % name)
        return bytes.fromhex(m.group(1))
    fx = {n: const_bytes(n) for n in (
        "vk", "vk_digest", "proof", "statement", "statement_digest",
        "n1_statement_digest", "n2_statement_digest", "n3_proof")}
    if len(fx["vk"]) != 488 or len(fx["proof"]) != 192:
        raise ValueError("VK/proof length mismatch")
    if len(fx["statement"]) != 721:
        raise ValueError("statement length mismatch")
    if blake2b256(fx["vk"]) != fx["vk_digest"]:
        raise ValueError("vk_digest does not equal blake2b-256(vk)")
    if blake2b256(fx["statement"]) != fx["statement_digest"]:
        raise ValueError("statement_digest does not equal blake2b-256(statement)")
    if len(fx["n3_proof"]) != 192:
        raise ValueError("n3_proof length mismatch")
    return fx


def extract_compiled(b9_raw: bytes):
    doc = json.loads(b9_raw.decode("utf-8"))
    by_title = {}
    for v in doc.get("validators", []):
        by_title[v.get("title")] = v
    codes = []
    claimed = []
    for title in C2_TITLES:
        v = by_title.get(title)
        if v is None:
            raise ValueError("validator title %s missing" % title)
        if "parameters" in v:
            raise ValueError("validator %s unexpectedly has parameters" % title)
        claimed.append(v.get("hash"))
        codes.append(v["compiledCode"])
    if len(set(codes)) != 1:
        raise ValueError("c2 titles do not share one compiledCode")
    if len(set(claimed)) != 1:
        raise ValueError("c2 titles do not share one declared hash")
    code_hex = codes[0].lower()
    raw = bytes.fromhex(code_hex)
    if len(raw) != COMPILED_BYTES_LEN or not code_hex.startswith("590acf"):
        raise ValueError("compiledCode length/prefix mismatch")
    # Independent recomputation is the basis; the artifact's declared
    # hash is only a cross-check and must never be trusted on its own.
    recomputed = recompute_script_hash(raw)
    if claimed[0] != TOOL_DECLARED_C2_HASH:
        raise ValueError("declared hash is not the pinned expected value")
    if recomputed.hex() != claimed[0]:
        raise ValueError(
            "independently recomputed script hash %s != tool-declared "
            "hash %s; refusing to use the tool-declared identity"
            % (recomputed.hex(), claimed[0]))
    return code_hex, recomputed.hex()


def flip_first_bit(raw: bytes) -> bytes:
    return bytes([raw[0] ^ 0x01]) + raw[1:]


def main(argv) -> int:
    if len(argv) != 4:
        print("usage: build_k10_inputs.py <b9-applied.json> <k8_fixture.ak> "
              "<output-dir>", file=sys.stderr)
        return 2
    b9_path, fixture_path, out_dir = Path(argv[1]), Path(argv[2]), Path(argv[3])
    if out_dir.name != EXPECTED_OUTPUT_BASENAME:
        print("output dir basename must be %s" % EXPECTED_OUTPUT_BASENAME,
              file=sys.stderr)
        return 2
    if out_dir.exists() and any(out_dir.iterdir()):
        print("output dir exists and is not empty; refusing to overwrite",
              file=sys.stderr)
        return 2
    b9_before = read_pinned(b9_path, PIN_B9_APPLIED, "b9-applied.json")
    fx_before = read_pinned(fixture_path, PIN_K8_FIXTURE, "k8_fixture.ak")
    fx = extract_fixture(fx_before.decode("utf-8"))
    code_hex, recomputed_hash = extract_compiled(b9_before)
    policy = bytes.fromhex(recomputed_hash)
    if len(policy) != SCRIPT_HASH_BYTES:
        raise ValueError("policy must be 28 bytes")
    vk, proof = fx["vk"], fx["proof"]
    vk_d, stmt_d = fx["vk_digest"], fx["statement_digest"]
    pos_cbor = envelope_cbor(vk_d, stmt_d)
    if pos_cbor != frozen_positive_cbor(vk_d, stmt_d):
        raise ValueError("positive envelope CBOR != frozen preimage")
    pos_name = blake2b256(pos_cbor)
    bad_vk_d = flip_first_bit(vk_d)
    cases = [
        # id, env vk_digest, env statement_digest, token name, qty, proof
        ("K10-T1-positive-genuine-issue", vk_d, stmt_d, pos_name, 1, proof),
        ("K10-T2-n1-body-digest-pairing-reject", vk_d,
         fx["n1_statement_digest"],
         token_name(vk_d, fx["n1_statement_digest"]), 1, proof),
        ("K10-T3-n2-context-digest-pairing-reject", vk_d,
         fx["n2_statement_digest"],
         token_name(vk_d, fx["n2_statement_digest"]), 1, proof),
        ("K10-T4-n3-proof-ac-swap-pairing-reject", vk_d, stmt_d, pos_name, 1,
         fx["n3_proof"]),
        ("K10-T5-quantity-two-reject", vk_d, stmt_d, pos_name, 2, proof),
        ("K10-T6-token-name-bitflip-reject", vk_d, stmt_d,
         flip_first_bit(pos_name), 1, proof),
        ("K10-T7-vk-digest-bitflip-reject", bad_vk_d, stmt_d,
         token_name(bad_vk_d, stmt_d), 1, proof),
    ]
    out_dir.mkdir(parents=True, exist_ok=True)
    ctx_dir = out_dir / "contexts"
    ctx_dir.mkdir(exist_ok=True)
    (out_dir / "script.hex").write_text(code_hex + "\n", encoding="utf-8")
    manifest_cases = []
    for case_id, env_vk_d, env_stmt_d, name, qty, case_proof in cases:
        env = env_data(env_vk_d, env_stmt_d)
        ctx = script_context(policy, env, name, qty, vk, case_proof)
        rel = "contexts/%s.uplc.txt" % case_id
        (out_dir / rel).write_text(
            "(con data (%s))\n" % uplc_term(ctx), encoding="utf-8")
        manifest_cases.append({
            "id": case_id,
            "contextFile": rel,
            "envelopeCborHex": envelope_cbor(env_vk_d, env_stmt_d).hex(),
            "tokenNameHex": name.hex(),
            "quantity": qty,
            "envVkDigestHex": env_vk_d.hex(),
            "envStatementDigestHex": env_stmt_d.hex(),
            "proofHexSha256": sha256_hex(case_proof),
        })
    manifest = {
        "format": "cardano-k2-k10-public-inputs/1",
        "compiledValidatorTitle": "c2.c2.mint",
        "scriptHexFile": "script.hex",
        "scriptCompiledBytes": COMPILED_BYTES_LEN,
        "scriptInnerFlatBytes": INNER_FLAT_LEN,
        "policyIdHex": recomputed_hash,
        "scriptHashClaimedHex": TOOL_DECLARED_C2_HASH,
        "scriptHashIndependentlyRecomputedHex": recomputed_hash,
        "scriptHashClassification": (
            "independently recomputed by this adapter from the pinned "
            "compiledCode via Pallas ComputeHash<28> for PlutusScript<3>; "
            "the tool-declared hash was used only as a cross-check and "
            "generation would have failed had it disagreed"),
        "scriptHashPreimage": (
            "0x03 || compiledCode CBOR bytes (2770 bytes); strict "
            "single-CBOR-byte-string guard validated 2767 inner flat "
            "bytes; inner flat is not hashed and no double-CBOR "
            "re-wrap is hashed"),
        "scriptHashAlgorithm": (
            "blake2b digest_size=28 (Pallas Hasher<224>::hash_tagged, "
            "pallas-traverse hashes.rs PlutusScript<VERSION>, "
            "VERSION=3)"),
        "auditSourcePins": AUDIT_SOURCE_PINS,
        "vkBytes": len(vk), "proofBytes": len(proof),
        "vkDigestHex": vk_d.hex(), "statementDigestHex": stmt_d.hex(),
        "positiveEnvelopeCborHex": pos_cbor.hex(),
        "positiveTokenNameHex": pos_name.hex(),
        "inputs": [
            {"role": "b9-applied", "sha256": PIN_B9_APPLIED},
            {"role": "k8_fixture", "sha256": PIN_K8_FIXTURE},
        ],
        "cases": manifest_cases,
        "limitations": [
            "Local fabricated context (2222../555.. retained from K9); not a ledger transaction.",
            "Expiry 1893456000000 is builder-chosen and cryptographically unbound.",
            "Policy is independently recomputed as above; no ledger, fee, conservation or Preprod claim.",
        ],
    }
    (out_dir / "manifest.json").write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    # Inputs must be byte-identical before and after generation.
    if read_pinned(b9_path, PIN_B9_APPLIED, "b9-applied.json") != b9_before:
        raise ValueError("b9 input changed during generation")
    if read_pinned(fixture_path, PIN_K8_FIXTURE, "k8_fixture.ak") != fx_before:
        raise ValueError("fixture input changed during generation")
    print(json.dumps({"output": str(out_dir), "cases": len(manifest_cases)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
