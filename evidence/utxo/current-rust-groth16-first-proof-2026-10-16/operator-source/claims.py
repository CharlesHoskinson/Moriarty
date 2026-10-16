"""Public-byte claims gate and negative staging for invoice/partial.

Pure Python standard library only. No filesystem, subprocess, network,
import-time I/O, RNG, key, curve or pairing operations. hashlib is used
only for public SHA-256 / BLAKE2b-256 digests. All inputs are explicit
bytes supplied by the caller. The gate never pairing-verifies: success
means public-byte bindings match the frozen oracle only. Native host
outcomes must be observed separately by the runner.
"""
import hashlib
import json

__all__ = ["GateError", "stage_negatives", "check_inspect_and_proof", "check_native_outcomes"]

class GateError(ValueError):
    pass

def _fail(msg):
    raise GateError(msg)

def _eq(what, expected, actual):
    if not _exact(expected, actual):
        _fail("mismatch on %s: expected %r, got %r" % (what, expected, actual))
    return actual

def _exact(expected, actual):
    if type(expected) is not type(actual):
        return False
    if isinstance(expected, dict):
        return set(expected) == set(actual) and all(_exact(expected[k], actual[k]) for k in expected)
    if isinstance(expected, (list, tuple)):
        return len(expected) == len(actual) and all(_exact(e, a) for e, a in zip(expected, actual))
    return expected == actual

def _slot_triples(slots, what):
    if not isinstance(slots, list):
        _fail(what + ": list required")
    out = []
    for s in slots:
        _keys(s, ["label", "encoding", "value"], what + " slot")
        if not isinstance(s["label"], str) or not isinstance(s["encoding"], str) or type(s["value"]) not in (str, bool):
            _fail(what + " slot: label/encoding must be str and value must be str or bool")
        out.append([s["label"], s["encoding"], s["value"]])
    return out

def _pin_request(data):
    if not isinstance(data, (bytes, bytearray)):
        _fail("request: bytes required")
    raw = bytes(data)
    if len(raw) != REQUEST_BYTES_LEN or _sha256(raw) != REQUEST_SHA256:
        _fail("request is not the exact fixed invoice-partial fixture (3008 bytes, sha256 aa1703e7b3aa2e0dd6288668cc871996fc1ee49ecc90861a3c4b40f02107f3cd); pinned-fixture qualification only, no general request grammar is claimed")

def _is_int(v):
    return type(v) is int

def _nodes(v, depth=0):
    if depth > 32:
        _fail("JSON depth exceeds 32")
    if isinstance(v, dict):
        return 1 + sum(_nodes(x, depth + 1) for x in v.values())
    if isinstance(v, list):
        return 1 + sum(_nodes(x, depth + 1) for x in v)
    return 1

def _strict_json(data, what):
    if not isinstance(data, (bytes, bytearray)):
        _fail(what + ": bytes required")
    data = bytes(data)
    if len(data) > 1048576:
        _fail(what + ": exceeds JSON bound")
    try:
        text = data.decode("utf-8")
    except Exception:
        _fail(what + ": not UTF-8")
    def hook(pairs):
        out = {}
        for k, val in pairs:
            if k in out:
                _fail(what + ": duplicate key " + k)
            out[k] = val
        return out
    def bad_const(x):
        _fail(what + ": non-finite constant " + x)
    try:
        obj = json.loads(text, object_pairs_hook=hook, parse_constant=bad_const)
    except GateError:
        raise
    except Exception as e:
        _fail(what + ": invalid JSON: " + str(e))
    if _nodes(obj) > 65536:
        _fail(what + ": too many JSON nodes")
    return obj

def _dump(obj):
    return (json.dumps(obj, ensure_ascii=True, indent=2) + "\n").encode("utf-8")

def _keys(obj, allowed, what):
    if not isinstance(obj, dict):
        _fail(what + ": object required")
    extra = [k for k in obj if k not in allowed]
    missing = [k for k in allowed if k not in obj]
    if extra:
        _fail(what + ": unknown fields " + ",".join(sorted(extra)))
    if missing:
        _fail(what + ": missing fields " + ",".join(sorted(missing)))

def _hexbytes(text, n, what):
    if not isinstance(text, str) or len(text) != 2 * n:
        _fail(what + ": expected %d lowercase hex chars" % (2 * n))
    if any(c not in "0123456789abcdef" for c in text):
        _fail(what + ": hex must be lowercase [0-9a-f]")
    return bytes.fromhex(text)

def _hexany(text, what, max_bytes=262144):
    if not isinstance(text, str) or len(text) % 2 or len(text) // 2 > max_bytes:
        _fail(what + ": bad bounded hex")
    if any(c not in "0123456789abcdef" for c in text):
        _fail(what + ": hex must be lowercase [0-9a-f]")
    return bytes.fromhex(text)

def _dec128(text, what):
    if not isinstance(text, str) or not text or len(text) > 39 or any(c not in "0123456789" for c in text):
        _fail(what + ": expected canonical unsigned decimal")
    if len(text) > 1 and text[0] == "0":
        _fail(what + ": leading zero in decimal")
    v = int(text)
    if v > 340282366920938463463374607431768211455:
        _fail(what + ": decimal exceeds u128")
    return v

def _uint(v, what, lo, hi):
    if not _is_int(v) or v < lo or v > hi:
        _fail(what + ": integer outside [%d,%d]" % (lo, hi))
    return v

def _blake(b):
    return hashlib.blake2b(b, digest_size=32).digest()

def _sha256(b):
    return hashlib.sha256(b).hexdigest()

# --- Frozen pins: actual K5 invoice/partial (K6-SOURCE-SPEC.json, K5g.json) ---
ACTUAL_VK_DIGEST = "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd"
GOLD_DECLARED_VK_SYNTHETIC = "aade9e4ec92bb00754e7704107b33f60036035bc8dc8454a54bd9068bacc39f0"
GOLD_BYTES_LEN = 39353
GOLD_SHA256 = "65c90d3d13912b762b4977e08ea86212b2b83a16bb2b3ddcaa7a21c905075e46"
RECORD_BYTES_LEN = 3926
RECORD_SHA256 = "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"
PK_BYTES_ACTUAL = 98715456
PK_BYTES_N11 = 98715457
REQUEST_BYTES_LEN = 3008
REQUEST_SHA256 = "aa1703e7b3aa2e0dd6288668cc871996fc1ee49ecc90861a3c4b40f02107f3cd"
DRIVER_ID = "mori-k2-driver/0.1.0-k2s1+k4h1"
SETUP_NOTICE = "TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. Whoever controlled the setup process could forge proofs for this relation. VK provenance is declared by this record; no verifier can check that the VK was generated from r1csSha256."
RECEIPT_NOTICE = "Ordinary receipt: no setup, key, proof or pairing. The verdict is the fixed setup-mode matrices' decision on this claim's full assignment. The VK digest is caller-declared. Context bytes are bound, not authenticated. Values were proposed by the request, not decided by the host."
PREMISES = [
    "The proof shows, under setup honesty and Groth16 knowledge soundness, that the statement's constant header and body satisfy the named compiled moriarty-core/1 relation.",
    "Context bytes (vk digest, claimant, network, settle hash, thread, consumed outref, seq, catalogue digest, intents digest) are caller-declared premises: bound by the proof, not authenticated by it. S2 recomputes each from its authenticated ScriptContext and datum.",
    "Trust profile single-party-test-setup/1: whoever ran setup could forge proofs for this relation.",
    "Groth16 proofs are malleable: the proof bytes are not a unique claim identifier.",
    "Signed intents, debit authority, frame completeness, physical asset delta, replay and phase outcomes are target obligations outside this proof.",
    "No ledger admission, deployment, settlement, Preprod or Midnight result is implied.",
]
DESCRIPTOR29 = {
    "relationFormat": "mori-r1cs-transition/1-unadopted", "relationVersion": 1,
    "language": "moriarty/1", "semantics": "moriarty-core/1",
    "logicalCostProfile": "moriarty-reference-logical-cost/1",
    "profileDigest": "f388e435d80aa53e35049128a773dbb1a19f345290b37d12151db205c8c9292a",
    "programHash": "d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9",
    "sourceSha256": "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9",
    "action": "partial", "actionIndex": 1,
    "relationShapeDigest": "4d755ee1f766b05981706355e27fc8c1bc0a7bf124742d0bd09aaab6bd5c6516",
    "compilerSourceManifestSha256": "eea1a420ec996e6369ef2eb1a60208f682ac21cf646cfa80c8729c7621722d47",
    "coreClosureSha256": "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed",
    "compilerBuildDigest": "0069e686c5a58165d177b6866531769680c8460fa300b2094a597b1639adad09",
    "adapter": "SettlementHashV2", "logicalWork": "28",
    "predicateDigest": "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd",
    "abi2ShapeDigest": "36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969",
    "abi2ShapeBytesHex": "4d524731362d5056432d5348502d30320001000b6f7261636c652d756e6974000e696e766f6963652d69737375657200046361736800000300066d61726b65720000066f70656e6564010004706169640000010006616d6f756e7400000300000006657363726f7700000004666565730000000673656c6c657200020007696e766f69636500000006657363726f77000673656c6c657200077265736572766500000006657363726f770005627579657200040200000001010000000203000001000002000100000008000000077061727469616c0001",
    "bodyLen": 378, "statementTagHex": "4d524731362d5056432d53544d542d32",
    "familyIdHex": "4d524731362d5056432d46414d322d4731362d424c532d41524b30342d484c32",
    "headerLen": 343, "fieldModulusLeHex": "01000000fffffffffe5bfeff02a4bd5305d8a10908d83933487d9d2953a7ed73",
    "r1csSha256": "e458153e583e0a817fbab08f24f4ba16465e459969c1f8cd4e13b801e82694c0",
    "constraints": 308790, "instanceVariables": 3, "witnessVariables": 306452, "publicInputCount": 2,
}
DESC_KEYS = list(DESCRIPTOR29.keys())
GOLD_SLOTS = [
    ["input.amount", "uint128", "35"], ["pre.state.marker", "uint128", "7"],
    ["pre.state.opened", "bool", True], ["pre.state.paid", "uint128", "0"],
    ["pre.cell.CASH.escrow", "uint128", "109"], ["pre.cell.CASH.fees", "uint128", "0"],
    ["pre.cell.CASH.seller", "uint128", "0"], ["pre.duty.invoice.present", "bool", True],
    ["pre.duty.invoice.amount", "uint128", "90"], ["pre.duty.reserve.present", "bool", True],
    ["pre.duty.reserve.amount", "uint128", "14"], ["post.state.marker", "uint128", "7"],
    ["post.state.opened", "bool", True], ["post.state.paid", "uint128", "35"],
    ["post.cell.CASH.escrow", "uint128", "72"], ["post.cell.CASH.fees", "uint128", "2"],
    ["post.cell.CASH.seller", "uint128", "35"], ["post.duty.invoice.present", "bool", True],
    ["post.duty.invoice.amount", "uint128", "55"], ["post.duty.reserve.present", "bool", True],
    ["post.duty.reserve.amount", "uint128", "14"], ["effect.0.fee.escrow.fees.amount", "uint128", "2"],
    ["effect.1.transfer.escrow.seller.amount", "uint128", "35"],
    ["effect.2.liability.invoice.before", "uint128", "90"],
    ["effect.2.liability.invoice.after", "uint128", "55"],
    ["effect.3.state.paid.before", "uint128", "0"],
    ["effect.3.state.paid.after", "uint128", "35"],
    ["accounting.CASH.escrow.gross_debit", "wide144", "37"],
    ["accounting.CASH.escrow.fees_paid", "wide144", "2"],
]
CTX_KEYS = ["vkDigest", "claimantVkh", "networkId", "settleHash", "threadId", "consumedTxid", "consumedIx", "seq", "catalogueDigest", "intentsDigest"]

def _check_descriptor(desc, what="descriptor"):
    _keys(desc, DESC_KEYS, what)
    for k in DESC_KEYS:
        _eq(what + "." + k, DESCRIPTOR29[k], desc[k])
    return desc

def _parse_descriptor_input(descriptor):
    if isinstance(descriptor, (bytes, bytearray)):
        obj = _strict_json(descriptor, "descriptor")
    elif isinstance(descriptor, dict):
        obj = descriptor
    else:
        _fail("descriptor: dict or JSON bytes required")
    return _check_descriptor(obj)

def _parse_setup(data):
    if not isinstance(data, (bytes, bytearray)):
        _fail("setup record: bytes required")
    raw = bytes(data)
    rec = _strict_json(raw, "setup record")
    _keys(rec, ["format", "trustProfile", "notice", "relation", "curve", "snark", "encoding", "verifyingKeyHex", "vkDigestBlake2b256", "provingKeySha256", "provingKeyBytes", "driver"], "setup record")
    _eq("setup format", "moriarty-k2-groth16-setup/1", rec["format"])
    _eq("trustProfile", "single-party-test-setup/1", rec["trustProfile"])
    _eq("setup notice", SETUP_NOTICE, rec["notice"])
    _eq("curve", "BLS12-381", rec["curve"])
    _eq("snark", "Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)", rec["snark"])
    _eq("encoding", "arkworks 0.4 compressed points (Zcash BLS12-381 layout); VK 488 bytes, proof 192 bytes", rec["encoding"])
    _eq("driver", DRIVER_ID, rec["driver"])
    _check_descriptor(rec["relation"], "setup relation")
    vk = _hexbytes(rec["verifyingKeyHex"], 488, "verifyingKeyHex")
    _eq("vkDigestBlake2b256", ACTUAL_VK_DIGEST, rec["vkDigestBlake2b256"])
    _eq("BLAKE2b-256(VK)", ACTUAL_VK_DIGEST, _blake(vk).hex())
    if vk[336:344] != bytes([3, 0, 0, 0, 0, 0, 0, 0]):
        _fail("VK IC prefix is not u64-LE 3 at offset 336 (public metadata cross-check only, not curve validation)")
    _hexbytes(rec["provingKeySha256"], 32, "provingKeySha256")
    _uint(rec["provingKeyBytes"], "provingKeyBytes", 1, 18446744073709551615)
    return rec, vk, raw

def _context_from_statement(st):
    return {
        "vkDigest": st[112:144].hex(), "claimantVkh": st[144:172].hex(),
        "networkId": st[172], "settleHash": st[173:201].hex(),
        "threadId": st[201:233].hex(), "consumedTxid": st[233:265].hex(),
        "consumedIx": int.from_bytes(st[265:267], "big"),
        "seq": int.from_bytes(st[267:275], "big"),
        "catalogueDigest": st[275:307].hex(), "intentsDigest": st[307:339].hex(),
    }

def _parse_statement(st, desc):
    if len(st) != 721:
        _fail("statement length %d differs from 343 + 378" % len(st))
    _eq("statement tag", bytes.fromhex(desc["statementTagHex"]), st[0:16])
    _eq("statement family", bytes.fromhex(desc["familyIdHex"]), st[16:48])
    _eq("statement predicateDigest", desc["predicateDigest"], st[48:80].hex())
    _eq("statement abi2ShapeDigest", desc["abi2ShapeDigest"], st[80:112].hex())
    _eq("statement vkDigest", ACTUAL_VK_DIGEST, st[112:144].hex())
    body_len = int.from_bytes(st[339:343], "big")
    _eq("statement bodyLen", 378, body_len)
    return _context_from_statement(st)

def _cardano(desc, ctx, digest_hex):
    return {
        "envV2": {"version": 2, "claimKind": 0, "familyId": desc["familyIdHex"],
                  "predicateDigest": desc["predicateDigest"], "vkDigest": ACTUAL_VK_DIGEST,
                  "claimantVkh": ctx["claimantVkh"],
                  "settleOutref": {"transactionId": ctx["consumedTxid"], "outputIndex": ctx["consumedIx"]},
                  "statementDigest": digest_hex},
        "catalogueActionEntry": {"name": desc["action"], "index": desc["actionIndex"],
                                 "predicateDigest": desc["predicateDigest"], "vkDigest": ACTUAL_VK_DIGEST,
                                 "shapeDigest": desc["abi2ShapeDigest"]},
        "intentTargetHashV2": desc["predicateDigest"] + ":" + ACTUAL_VK_DIGEST + ":" + desc["abi2ShapeDigest"],
        "intentAnchor": "cardano-plutus-v3:" + ctx["settleHash"],
    }

def _check_cardano_shape(c):
    _keys(c, ["envV2", "catalogueActionEntry", "intentTargetHashV2", "intentAnchor"], "cardano")
    _keys(c["envV2"], ["version", "claimKind", "familyId", "predicateDigest", "vkDigest", "claimantVkh", "settleOutref", "statementDigest"], "cardano.envV2")
    _keys(c["envV2"]["settleOutref"], ["transactionId", "outputIndex"], "cardano.envV2.settleOutref")
    _keys(c["catalogueActionEntry"], ["name", "index", "predicateDigest", "vkDigest", "shapeDigest"], "cardano.catalogueActionEntry")

def _proof_points(proof):
    if len(proof) != 192:
        _fail("proof must be exactly 192 bytes, got %d" % len(proof))
    ident_g1 = bytes([0xC0]) + bytes(47)
    ident_g2 = bytes([0xC0]) + bytes(95)
    if proof[0:48] == ident_g1 or proof[144:192] == ident_g1 or proof[48:144] == ident_g2:
        _fail("proof contains an identity point encoding (public-byte refusal; not curve validation)")
    return proof[0:48], proof[48:144], proof[144:192]

def _check_export(rec, desc, exp, record_raw):
    _keys(exp, ["format", "setupRecordSha256", "programHash", "sourceSha256", "action", "actionIndex", "compilerBuildDigest", "r1csSha256", "predicateDigest", "abi2ShapeDigest", "vkDigestBlake2b256", "context", "statementHex", "statementDigestBlake2b256", "publicInputs", "proofHex", "cardano", "premises"], "proof export")
    _eq("proof format", "moriarty-k2-groth16-proof/1", exp["format"])
    _eq("setupRecordSha256", _sha256(record_raw), exp["setupRecordSha256"])
    _eq("programHash", desc["programHash"], exp["programHash"])
    _eq("sourceSha256", desc["sourceSha256"], exp["sourceSha256"])
    _eq("action", desc["action"], exp["action"])
    _eq("actionIndex", desc["actionIndex"], _uint(exp["actionIndex"], "actionIndex", 0, 65535))
    _eq("compilerBuildDigest", desc["compilerBuildDigest"], exp["compilerBuildDigest"])
    _eq("r1csSha256", desc["r1csSha256"], exp["r1csSha256"])
    _eq("predicateDigest", desc["predicateDigest"], exp["predicateDigest"])
    _eq("abi2ShapeDigest", desc["abi2ShapeDigest"], exp["abi2ShapeDigest"])
    _eq("vkDigestBlake2b256 (export)", ACTUAL_VK_DIGEST, exp["vkDigestBlake2b256"])
    _keys(exp["context"], CTX_KEYS, "proof context")
    _uint(exp["context"]["networkId"], "context.networkId", 0, 255)
    _uint(exp["context"]["consumedIx"], "context.consumedIx", 0, 65535)
    _uint(exp["context"]["seq"], "context.seq", 0, 18446744073709551615)
    st = _hexany(exp["statementHex"], "statementHex")
    ctx = _parse_statement(st, desc)
    _eq("context fields vs statement bytes", ctx, exp["context"])
    digest = _blake(st)
    _eq("statementDigestBlake2b256", digest.hex(), exp["statementDigestBlake2b256"])
    x1 = int.from_bytes(digest[0:16], "big")
    x2 = int.from_bytes(digest[16:32], "big")
    _keys(exp["publicInputs"], ["x1", "x2"], "publicInputs")
    _eq("x1", x1, _dec128(exp["publicInputs"]["x1"], "x1"))
    _eq("x2", x2, _dec128(exp["publicInputs"]["x2"], "x2"))
    proof = _hexbytes(exp["proofHex"], 192, "proofHex")
    _proof_points(proof)
    _check_cardano_shape(exp["cardano"])
    _eq("cardano section", _cardano(desc, ctx, digest.hex()), exp["cardano"])
    _eq("premises", PREMISES, exp["premises"])
    return st, ctx, digest, x1, x2, proof

def _rederived(exp, desc, st):
    out = json.loads(_dump(exp))
    ctx = _parse_statement(st, desc)
    digest = _blake(st)
    out["statementHex"] = st.hex()
    out["context"] = ctx
    out["statementDigestBlake2b256"] = digest.hex()
    out["publicInputs"] = {"x1": str(int.from_bytes(digest[0:16], "big")), "x2": str(int.from_bytes(digest[16:32], "big"))}
    out["cardano"] = _cardano(desc, ctx, digest.hex())
    return out

def stage_negatives(proof_bytes, setup_record_bytes, request_bytes, descriptor):
    """Stage R2 negatives N1..N11 from actual positive bytes.

    proof_bytes is the positive proof-export JSON bytes. No proving-key
    bytes are accepted. Returns {files, cases, metadata}; files maps each
    fixed literal filename to bytes. N9 has no file (source reference
    only). N8/N11 setup variants are private (publish False).
    Raises GateError on any unmatched premise; never regenerates a proof.
    """
    _pin_request(request_bytes)
    desc = _parse_descriptor_input(descriptor)
    rec, _vk, record_raw = _parse_setup(setup_record_bytes)
    _eq("descriptor vs setup relation", rec["relation"], desc)
    _eq("setup record size", RECORD_BYTES_LEN, len(record_raw))
    _eq("setup record SHA-256", RECORD_SHA256, _sha256(record_raw))
    _eq("provingKeyBytes (positive record)", PK_BYTES_ACTUAL, rec["provingKeyBytes"])
    exp = _strict_json(proof_bytes, "proof export")
    st, _ctx, _digest, _x1, _x2, proof = _check_export(rec, desc, exp, record_raw)
    a, b, c = _proof_points(proof)
    if a == c:
        _fail("positive proof A == C; N3 swap is not a discriminator for this fixture (fail closed, no regeneration)")
    req = _strict_json(request_bytes, "request")
    expected_ctx, declared_vk = _check_request(req, desc)
    if declared_vk is not None:
        _eq("request context vkDigest (caller-declared)", ACTUAL_VK_DIGEST, declared_vk)
    _eq("request context vs proof context", expected_ctx, _ctx)
    files = {}
    cases = {}
    def add(cid, fname, data, cls, stage, publish, extra=None):
        if fname is not None:
            files[fname] = data
        m = {"expectedClass": cls, "expectedStage": stage, "publish": publish, "file": fname}
        if extra:
            m.update(extra)
        cases[cid] = m
    # N1: body offset 358 (last byte of input.amount uint128), 35 -> 34, full rederive.
    if st[358] != 35:
        _fail("N1 premise: statement[358] is %d, expected 35" % st[358])
    s1 = bytearray(st); s1[358] = 34
    add("N1", "n1-body-rederived.proof.json", _dump(_rederived(exp, desc, bytes(s1))), "ProofRejected", "pairing after all strict check_export invariants pass", True, {"edit": "statement[358] 35->34; statementHex/digest/x1/x2/whole cardano recomputed; proof and header context unchanged"})
    # N2: seq offset 274 (last byte of seq u64 BE), 1 -> 2, full rederive; context not authenticated.
    if st[274] != 1:
        _fail("N2 premise: statement[274] is %d, expected 1" % st[274])
    s2 = bytearray(st); s2[274] = 2
    add("N2", "n2-context-rederived.proof.json", _dump(_rederived(exp, desc, bytes(s2))), "ProofRejected", "pairing after all strict check_export invariants pass", True, {"edit": "statement[274] seq 1->2; context.seq/statementHex/digest/x1/x2/whole cardano recomputed", "contextAuthenticated": False})
    # N3: proof C||B||A.
    e3 = json.loads(_dump(exp)); e3["proofHex"] = (c + b + a).hex()
    add("N3", "n3-proof-ac-swapped.proof.json", _dump(e3), "ProofRejected", "pairing after strict proof decode and check_export", True, {"edit": "proofHex = C(48B)||B(96B)||A(48B); nothing else changes"})
    # N4: swap x1/x2 only.
    e4 = json.loads(_dump(exp)); e4["publicInputs"] = {"x1": exp["publicInputs"]["x2"], "x2": exp["publicInputs"]["x1"]}
    add("N4", "n4-x-swapped.proof.json", _dump(e4), "IdentityMismatch x1 or x2 (parsed what/expected/got; distinct canonical u128)", "check_export recomputed binding, pre-pairing", True, {"nativeValidExample": "refused: identity mismatch on x1: expected <recomputed x1 decimal>, got <stale swapped x2 decimal>", "nativeInvalidExample": "refused: identity mismatch on unrelated_field: expected 1, got 2"})
    # N5: one statement byte flipped, nothing rederived (stale digest).
    e5 = json.loads(_dump(exp)); s5 = bytearray(st); s5[358] = 34; e5["statementHex"] = bytes(s5).hex()
    add("N5", "n5-statement-stale-digest.proof.json", _dump(e5), "IdentityMismatch statementDigestBlake2b256", "check_export statementDigestBlake2b256 same() after parse_statement/check_statement, pre-pairing", True, {"digestDirection": "expected=BLAKE2b-256(tampered statementHex) recomputed in export.rs check_export; got=stale statementDigestBlake2b256 declared in the staged file", "nativeValidExample": "refused: identity mismatch on statementDigestBlake2b256: expected <64 lowercase hex of tampered statement>, got <64 lowercase hex stale declared digest, distinct>", "nativeInvalidExample": "refused: identity mismatch on unrelated_field: expected 1, got 2"})
    # N6: proof truncated by final byte (two hex chars).
    e6 = json.loads(_dump(exp)); e6["proofHex"] = exp["proofHex"][:-2]
    add("N6", "n6-proof-truncated.proof.json", _dump(e6), "Input proofHex expected 384 hex characters, got 382", "export check_export decode_exact proofHex via hexutil.rs, pre-pairing", True, {"nativeValidExample": "refused: input: proofHex: expected 384 hex characters, got 382", "nativeInvalidExample": "refused: encoding: unrelated"})
    # N7: proof A replaced by G1 identity encoding.
    e7 = json.loads(_dump(exp)); e7["proofHex"] = (bytes([0xC0]) + bytes(47) + b + c).hex()
    add("N7", "n7-proof-identity.proof.json", _dump(e7), "Encoding proof identity point", "encoding.rs decode_proof identity refusal, pre-pairing", True, {"nativeValidExample": "refused: encoding: proof: identity point", "nativeInvalidExample": "refused: encoding: unrelated"})
    # N8: single hex-char edit of record vkDigestBlake2b256 only (private variant).
    rec8 = json.loads(_dump(rec))
    old8 = rec8["vkDigestBlake2b256"]
    new8 = ("0" if old8[0] != "0" else "1") + old8[1:]
    rec8["vkDigestBlake2b256"] = new8
    b8 = _dump(rec8)
    add("N8", "n8-record-vk-digest.setup.json", b8, "IdentityMismatch vkDigestBlake2b256", "strict record parse, pre-pairing", False, {"jsonPointer": "/vkDigestBlake2b256", "old": old8, "new": new8, "variantSha256": _sha256(b8), "variantBytes": len(b8)})
    # N9: reserve source reference only; no generated proof.
    add("N9", None, None, "Compile unknown action partial (descriptor guard not observed); descriptor mismatch only if it names a known descriptor field with the fixed expected value", "verify --source recompiles with selector_for(record relation) before verify_export require_same, pre-pairing", False, {"sourceReferenceOnly": "conditional-reserve.mori", "generatedProof": False, "descriptorGuardObserved": False, "branchReason": "conditional-reserve.mori actions are allocate/rebate with no partial; selector_for uses the record action partial, so mori-r1cs plan_action UnknownAction precedes require_same", "nativeValidExample": "refused: compile: unknown action \"partial\"", "nativeInvalidExample": "refused: identity mismatch on relation descriptor field invented: expected 1, got 2"})
    # N10: finish selector request; selector has NO actionIndex field.
    req10 = json.loads(_dump(req))
    _keys(req10["selector"], ["format", "language", "semantics", "action", "adapter", "cap", "accounting", "frame"], "selector")
    req10["selector"]["action"] = "finish"
    if "actionIndex" in req10["selector"]:
        _fail("N10 premise: selector.actionIndex must be absent; RelationSelector has no actionIndex field")
    _want10 = json.loads(_dump(req))
    _want10["selector"]["action"] = "finish"
    _eq("N10 request (only selector.action changed)", _want10, req10)
    b10 = _dump(req10)
    add("N10", "n10-finish-selector.request.json", b10, "IdentityMismatch if finish compiles, else Compile", "require_same descriptor equality before proving-key open/read and before OsRng (static call-order premise, not dynamic non-use proof)", True, {"conditional": {"ifFinishCompiles": "IdentityMismatch", "else": "Compile"}, "pkByteReads": 0, "descriptorMismatchRule": "what must be relation descriptor field <one of the 29 DESCRIPTOR29 fields>, expected must equal the JSON rendering of that fixed descriptor value, got must be a distinct JSON scalar of the same type", "compileBranch": "exploratory/unmeasured: finish compilation is not measured in supplied evidence; a compile refusal is reported with descriptorGuardObserved=false and is never counted as a tested descriptor guard"})
    # N11: provingKeyBytes 98715456 -> 98715457 only (private variant).
    rec11 = json.loads(_dump(rec))
    rec11["provingKeyBytes"] = PK_BYTES_N11
    b11 = _dump(rec11)
    add("N11", "n11-key-size.setup.json", b11, "IdentityMismatch proving key size", "full bounded PK read, then load_proving_key size refusal before SHA-256/decode/RNG", False, {"jsonPointer": "/provingKeyBytes", "old": PK_BYTES_ACTUAL, "new": PK_BYTES_N11, "variantSha256": _sha256(b11), "variantBytes": len(b11), "pkBytesReadExpected": PK_BYTES_ACTUAL, "pkFullValidatedDeserializes": 0})
    for _cid in ("N1", "N2", "N3"):
        cases[_cid]["proofCurveValidatedByThisModule"] = False
        cases[_cid]["nativeStrictDecodePremise"] = "positive VK/proof already passed native strict decode (encoding.rs decode_vk/decode_proof) as observed by the runner; this module performs no curve validation"
    return {"files": files, "cases": cases, "metadata": {"positiveProofBytes": 192, "positiveAEqualsC": False, "caseIds": ["N1", "N2", "N3", "N4", "N5", "N6", "N7", "N8", "N9", "N10", "N11"], "pkBytesAcceptedAsInput": False, "proofGenerated": False, "proofCurveValidatedByThisModule": False, "nativeStrictDecodePremise": "N1-N3 ProofRejected expectations presuppose a runner-observed native positive proof/VK strict decode; this module performs no curve validation", "requestFixturePinned": True, "requestBytes": REQUEST_BYTES_LEN, "requestSha256": REQUEST_SHA256, "generalRequestCoverage": False}}

def _decode_body(st):
    out = []
    at = 343
    for label, enc, _v in GOLD_SLOTS:
        width = {"uint128": 16, "bool": 1, "wide144": 18}[enc]
        chunk = st[at:at + width]
        if len(chunk) != width:
            _fail("statement too short for slot " + label)
        if enc == "uint128":
            val = str(int.from_bytes(chunk, "big"))
        elif enc == "bool":
            if chunk[0] not in (0, 1):
                _fail("slot " + label + " byte is not canonical Bool 0/1")
            val = bool(chunk[0])
        else:
            top = int.from_bytes(chunk[0:2], "big")
            lo = int.from_bytes(chunk[2:18], "big")
            val = str(lo) if top == 0 else "%d*2^128+%d" % (top, lo)
        out.append({"label": label, "encoding": enc, "value": val})
        at += width
    _eq("body coverage", 721, at)
    return out

def _check_request(req, desc):
    _keys(req, ["format", "selector", "transition", "context"], "request")
    _eq("request format", "moriarty-k2-prove-request/1", req["format"])
    sel = req["selector"]
    if not isinstance(sel, dict):
        _fail("selector: object required")
    allowed_sel = ["format", "language", "semantics", "action", "adapter", "cap", "accounting", "frame", "expectedSourceSha256", "expectedProgramHash"]
    for k in sel:
        if k not in allowed_sel:
            _fail("selector: unknown field " + k)
    for k in ["format", "language", "semantics", "action", "adapter", "cap", "accounting", "frame"]:
        if k not in sel:
            _fail("selector: missing field " + k)
    _eq("selector.format", "moriarty-k2-relation-selector/1", sel["format"])
    _eq("selector.language", desc["language"], sel["language"])
    _eq("selector.semantics", desc["semantics"], sel["semantics"])
    _eq("selector.action", desc["action"], sel["action"])
    _eq("selector.adapter", desc["adapter"], sel["adapter"])
    _eq("selector.cap", "NotBound", sel["cap"])
    _eq("selector.accounting", "GrossDebitAndFees", sel["accounting"])
    _eq("selector.frame", "TouchedOnly", sel["frame"])
    if "expectedSourceSha256" in sel:
        _eq("selector.expectedSourceSha256", desc["sourceSha256"], sel["expectedSourceSha256"])
    if "expectedProgramHash" in sel:
        _eq("selector.expectedProgramHash", desc["programHash"], sel["expectedProgramHash"])
    ctx = req["context"]
    if not isinstance(ctx, dict):
        _fail("request context: object required")
    ckeys = ["claimantVkh", "networkId", "settleHash", "threadId", "consumedTxid", "consumedIx", "seq", "catalogueDigest", "intentsDigest", "vkDigest"]
    for k in ctx:
        if k not in ckeys:
            _fail("request context: unknown field " + k)
    for k in ckeys[:-1]:
        if k not in ctx:
            _fail("request context: missing field " + k)
    _hexbytes(ctx["claimantVkh"], 28, "claimantVkh")
    _uint(ctx["networkId"], "networkId", 0, 255)
    _hexbytes(ctx["settleHash"], 28, "settleHash")
    _hexbytes(ctx["threadId"], 32, "threadId")
    _hexbytes(ctx["consumedTxid"], 32, "consumedTxid")
    _uint(ctx["consumedIx"], "consumedIx", 0, 65535)
    _uint(ctx["seq"], "seq", 0, 18446744073709551615)
    _hexbytes(ctx["catalogueDigest"], 32, "catalogueDigest")
    _hexbytes(ctx["intentsDigest"], 32, "intentsDigest")
    declared = None
    if "vkDigest" in ctx:
        _hexbytes(ctx["vkDigest"], 32, "context vkDigest")
        declared = ctx["vkDigest"]
    expected_ctx = {"vkDigest": ACTUAL_VK_DIGEST, "claimantVkh": ctx["claimantVkh"], "networkId": ctx["networkId"], "settleHash": ctx["settleHash"], "threadId": ctx["threadId"], "consumedTxid": ctx["consumedTxid"], "consumedIx": ctx["consumedIx"], "seq": ctx["seq"], "catalogueDigest": ctx["catalogueDigest"], "intentsDigest": ctx["intentsDigest"]}
    return expected_ctx, declared

def check_inspect_and_proof(compile_descriptor_bytes, inspect_receipt_bytes, proof_export_bytes, setup_record_bytes, gold_bytes, request_bytes):
    """Independent public-byte gate. Returns a concise JSON-safe result
    only after every expected check is true; raises GateError otherwise.
    Does NOT pairing-verify and claims no ledger truth."""
    _pin_request(request_bytes)
    desc = _check_descriptor(_strict_json(compile_descriptor_bytes, "descriptor"), "descriptor")
    rec, _vk, record_raw = _parse_setup(setup_record_bytes)
    _eq("descriptor vs setup relation", rec["relation"], desc)
    _eq("setup record size", RECORD_BYTES_LEN, len(record_raw))
    _eq("setup record SHA-256", RECORD_SHA256, _sha256(record_raw))
    _eq("provingKeyBytes", PK_BYTES_ACTUAL, rec["provingKeyBytes"])
    gold_raw = bytes(gold_bytes)
    _eq("gold size", GOLD_BYTES_LEN, len(gold_raw))
    _eq("gold SHA-256", GOLD_SHA256, _sha256(gold_raw))
    gold = _strict_json(gold_raw, "gold")
    if not isinstance(gold, dict) or not isinstance(gold.get("relations"), dict):
        _fail("gold: relations object required")
    rel = gold["relations"].get("invoice/partial")
    if not isinstance(rel, dict):
        _fail("gold: relations[invoice/partial] object required")
    for _k in ("slots", "declaredVkDigest", "r1csSha256", "predicateDigest", "abi2ShapeDigest", "programHash", "constraints", "witnessVariables", "instanceVariables", "bodyLen", "statementLength"):
        if _k not in rel:
            _fail("gold relation: missing field " + _k)
    gold_slots = _slot_triples(rel["slots"], "gold slots")
    _eq("gold 29 slots vs frozen oracle", GOLD_SLOTS, gold_slots)
    _eq("gold declaredVkDigest (synthetic, not K6 VK)", GOLD_DECLARED_VK_SYNTHETIC, rel["declaredVkDigest"])
    if rel["declaredVkDigest"] == ACTUAL_VK_DIGEST:
        _fail("gold synthetic VK digest must not be used as the actual VK digest")
    for k, dk in [("r1csSha256", "r1csSha256"), ("predicateDigest", "predicateDigest"), ("abi2ShapeDigest", "abi2ShapeDigest"), ("programHash", "programHash"), ("constraints", "constraints"), ("witnessVariables", "witnessVariables"), ("instanceVariables", "instanceVariables"), ("bodyLen", "bodyLen")]:
        _eq("gold." + k, desc[dk], rel[k])
    _eq("gold statementLength", 721, rel["statementLength"])
    req = _strict_json(request_bytes, "request")
    expected_ctx, declared_vk = _check_request(req, desc)
    if declared_vk is not None:
        _eq("request context vkDigest (caller-declared)", ACTUAL_VK_DIGEST, declared_vk)
    exp = _strict_json(proof_export_bytes, "proof export")
    st, ctx, digest, x1, x2, proof = _check_export(rec, desc, exp, record_raw)
    _eq("proof context vs request+record context", expected_ctx, ctx)
    _proof_points(proof)
    rc = _strict_json(inspect_receipt_bytes, "inspect receipt")
    _keys(rc, ["format", "notice", "action", "programHash", "predicateDigest", "abi2ShapeDigest", "r1csSha256", "declaredVkDigest", "statementLen", "statementHex", "statementDigestBlake2b256", "x1", "x2", "slots", "fixedMatrix"], "inspect receipt")
    _eq("receipt format", "moriarty-k2-statement-receipt/1", rc["format"])
    _eq("receipt notice", RECEIPT_NOTICE, rc["notice"])
    _eq("receipt action", desc["action"], rc["action"])
    _eq("receipt programHash", desc["programHash"], rc["programHash"])
    _eq("receipt predicateDigest", desc["predicateDigest"], rc["predicateDigest"])
    _eq("receipt abi2ShapeDigest", desc["abi2ShapeDigest"], rc["abi2ShapeDigest"])
    _eq("receipt r1csSha256", desc["r1csSha256"], rc["r1csSha256"])
    _eq("receipt declaredVkDigest (actual, not synthetic gold VK)", ACTUAL_VK_DIGEST, rc["declaredVkDigest"])
    _eq("receipt statementLen", 721, _uint(rc["statementLen"], "statementLen", 0, 262144))
    _eq("receipt statementHex vs proof statementHex", exp["statementHex"], rc["statementHex"])
    _eq("receipt statementDigestBlake2b256", digest.hex(), rc["statementDigestBlake2b256"])
    _eq("receipt x1", str(x1), rc["x1"])
    _eq("receipt x2", str(x2), rc["x2"])
    _eq("receipt statement vs proof statement bytes", st, _hexany(rc["statementHex"], "receipt statementHex"))
    decoded = _decode_body(st)
    receipt_slots = _slot_triples(rc["slots"], "receipt slots")
    frozen = [[s["label"], s["encoding"], s["value"]] for s in decoded]
    _eq("receipt slots vs independently decoded body", frozen, receipt_slots)
    _eq("decoded slots vs gold", GOLD_SLOTS, receipt_slots)
    fm = rc["fixedMatrix"]
    _keys(fm, ["r1csSha256", "equalsDescriptor", "proveMatrixDigestEqual", "constraints", "violated", "firstViolated"], "fixedMatrix")
    _eq("fixedMatrix.r1csSha256", desc["r1csSha256"], fm["r1csSha256"])
    _eq("fixedMatrix.equalsDescriptor", True, fm["equalsDescriptor"])
    _eq("fixedMatrix.proveMatrixDigestEqual", True, fm["proveMatrixDigestEqual"])
    _eq("fixedMatrix.constraints", 308790, fm["constraints"])
    _eq("fixedMatrix.violated", 0, fm["violated"])
    _eq("fixedMatrix.firstViolated", [], fm["firstViolated"])
    vals = {s["label"]: s["value"] for s in decoded}
    def n(label):
        return int(vals[label])
    fin = {"fee": n("effect.0.fee.escrow.fees.amount"), "transfer": n("effect.1.transfer.escrow.seller.amount"), "liabilityBefore": n("effect.2.liability.invoice.before"), "liabilityAfter": n("effect.2.liability.invoice.after"), "paidBefore": n("effect.3.state.paid.before"), "paidAfter": n("effect.3.state.paid.after"), "escrowPre": n("pre.cell.CASH.escrow"), "escrowPost": n("post.cell.CASH.escrow"), "feesPre": n("pre.cell.CASH.fees"), "feesPost": n("post.cell.CASH.fees"), "sellerPre": n("pre.cell.CASH.seller"), "sellerPost": n("post.cell.CASH.seller"), "reserve": n("post.duty.reserve.amount"), "grossDebit": n("accounting.CASH.escrow.gross_debit"), "marker": n("post.state.marker"), "opened": vals["post.state.opened"]}
    _eq("financial oracle", {"fee": 2, "transfer": 35, "liabilityBefore": 90, "liabilityAfter": 55, "paidBefore": 0, "paidAfter": 35, "escrowPre": 109, "escrowPost": 72, "feesPre": 0, "feesPost": 2, "sellerPre": 0, "sellerPost": 35, "reserve": 14, "grossDebit": 37, "marker": 7, "opened": True}, fin)
    if fin["escrowPost"] + fin["feesPost"] + fin["sellerPost"] != 109:
        _fail("touched conservation 109 failed")
    if any("spectator" in s["label"] for s in decoded):
        _fail("spectator must remain frame-external, not a proved slot")
    return {"format": "claims-gate/1", "status": "PUBLIC_BYTE_BINDINGS_MATCH_FROZEN_ORACLE", "pairingVerifiedByThisGate": False, "nativePairingRequiredSeparately": True, "slotsChecked": 29, "headerBytes": 343, "bodyBytes": 378, "statementBytes": 721, "statementDigestBlake2b256": digest.hex(), "x1": str(x1), "x2": str(x2), "declaredVkDigest": ACTUAL_VK_DIGEST, "goldDeclaredVkDigestSynthetic": GOLD_DECLARED_VK_SYNTHETIC, "consumerVkToMatrixChecked": False, "ledgerTruthClaimed": False, "contextPremise": "caller-declared; bound by proof, not authenticated", "requestContextVkDigestLabel": "explicit-caller-declared-equals-actual-record-VK" if declared_vk is not None else "absent; VK digest derived from actual record", "spectator41": "frame-external; not claimed proved", "financial": fin, "touchedConservation": 109, "residualDuties": [55, 14], "recordBytes": RECORD_BYTES_LEN, "recordSha256": RECORD_SHA256}

# Parsed field-specific native records from the Rust Display texts, as wrapped
# by main.rs (one refused: <Display> line, exit 1; usage: exit 2). Data only.
_PROOF_REJECTED_DISPLAY = "proof rejected: Groth16 pairing check failed for this VK and statement digest"
_COMPILE_DISPLAY_HEADS = ("core refusal: ", "unsupported profile: ", "moriarty/2 admitted by stage::load but not lowered in this slice", "unknown action ", "unsupported ", "bound: ", "action can never succeed at ", "identity: ", "field: ", "option: ", "adapter: ", "synthesis: ", "assignment shape: ")
_N8_STAGED_GOT = ("0" if ACTUAL_VK_DIGEST[0] != "0" else "1") + ACTUAL_VK_DIGEST[1:]

def _refusal_display(case, error):
    text = error[:-1] if error.endswith("\n") else error
    if "\n" in text or "\r" in text or not text.startswith("refused: "):
        _fail(case + " raw stderr must be one anchored refused: line and expected exit 1")
    return text[len("refused: "):]

def _identity_parts(case, display):
    head = "identity mismatch on "
    if not display.startswith(head):
        _fail(case + " raw stderr is not an identity mismatch Display")
    rest = display[len(head):]
    what, sep, tail = rest.partition(": expected ")
    expected, sep2, got = tail.partition(", got ")
    if not sep or not sep2 or not what or not expected or not got or ", got " in got:
        _fail(case + " identity mismatch must parse exactly as what/expected/got with no arbitrary suffix")
    return what, expected, got

def _is_hex64(v):
    return len(v) == 64 and all(c in "0123456789abcdef" for c in v)

def _is_dec128(v):
    try:
        _dec128(v, "native decimal")
    except GateError:
        return False
    return True

def _descriptor_record(case, display):
    what, expected, got = _identity_parts(case, display)
    prefix = "relation descriptor field "
    if not what.startswith(prefix):
        _fail(case + " descriptor mismatch must name a relation descriptor field")
    field = what[len(prefix):]
    if field not in DESCRIPTOR29:
        _fail(case + " descriptor field is not a known RelationDescriptor field: " + field)
    want = json.dumps(DESCRIPTOR29[field], ensure_ascii=True)
    if expected != want:
        _fail(case + " descriptor expected value does not equal the fixed descriptor value for " + field)
    try:
        got_value = json.loads(got)
    except Exception:
        _fail(case + " descriptor got value is not the JSON scalar rendering for " + field)
    if type(got_value) is not type(DESCRIPTOR29[field]) or got == expected:
        _fail(case + " descriptor got value must be a distinct JSON scalar of the fixed field type for " + field)
    return {"branch": "identityMismatchDescriptor", "descriptorField": field, "expected": expected, "got": got, "descriptorGuardObserved": True}

def _native_record(case, error):
    display = _refusal_display(case, error)
    if case in ("N1", "N2", "N3"):
        if display != _PROOF_REJECTED_DISPLAY:
            _fail(case + " must be exactly the ProofRejected Display from verify.rs pairing_verify")
        return {"branch": "proofRejected", "display": display}
    if case == "N4":
        what, expected, got = _identity_parts(case, display)
        if what not in ("x1", "x2") or not _is_dec128(expected) or not _is_dec128(got) or expected == got:
            _fail("N4 must mismatch x1 or x2 with distinct canonical u128 expected/got values")
        return {"branch": "identityMismatch", "what": what, "expected": expected, "got": got}
    if case == "N5":
        what, expected, got = _identity_parts(case, display)
        if what != "statementDigestBlake2b256" or not _is_hex64(expected) or not _is_hex64(got) or expected == got:
            _fail("N5 must mismatch statementDigestBlake2b256 with distinct 64-char lowercase hex expected/got")
        return {"branch": "identityMismatch", "what": what, "expected": expected, "got": got, "direction": "expected=recomputed BLAKE2b-256 of tampered statementHex in export.rs check_export; got=stale declared digest"}
    if case == "N6":
        if display != "input: proofHex: expected 384 hex characters, got 382":
            _fail("N6 must be exactly the hexutil.rs decode_exact proofHex Input Display for 382 hex chars")
        return {"branch": "input", "what": "proofHex", "expectedHexChars": 384, "gotHexChars": 382}
    if case == "N7":
        if display != "encoding: proof: identity point":
            _fail("N7 must be exactly the encoding.rs decode_proof identity-point Encoding Display")
        return {"branch": "encodingIdentityPoint", "what": "proof"}
    if case == "N8":
        what, expected, got = _identity_parts(case, display)
        if what != "vkDigestBlake2b256" or expected != ACTUAL_VK_DIGEST or got != _N8_STAGED_GOT:
            _fail("N8 must mismatch vkDigestBlake2b256 with expected equal to the actual record VK digest and got equal to the staged one-char edit")
        return {"branch": "identityMismatch", "what": what, "expected": expected, "got": got}
    if case == "N9":
        if display == 'compile: unknown action "partial"':
            return {"branch": "compileUnknownAction", "descriptorGuardObserved": False, "reason": "conditional-reserve.mori has allocate/rebate and no partial; verify --source recompiles with selector_for(record relation) whose action is partial, so plan_action UnknownAction precedes require_same"}
        return _descriptor_record(case, display)
    if case == "N10":
        if display.startswith("compile: "):
            detail = display[len("compile: "):]
            if not detail or not any(detail.startswith(h) for h in _COMPILE_DISPLAY_HEADS):
                _fail("N10 compile branch must carry a mori-r1cs CompileError Display")
            return {"branch": "compile", "descriptorGuardObserved": False, "compileDetail": detail, "reason": "exploratory/unmeasured: finish compilation is not measured in supplied evidence; compile precedes require_same and any proving-key read, so a compiler refusal is not a tested descriptor guard"}
        return _descriptor_record(case, display)
    if case == "N11":
        if display != "identity mismatch on proving key size: expected 98715457, got 98715456":
            _fail("N11 must be exactly the proving key size mismatch with expected 98715457 and got 98715456")
        return {"branch": "identityMismatch", "what": "proving key size", "expected": "98715457", "got": "98715456"}
    _fail("unknown native case " + case)
_ORDER = ["A6", "I6", "P6", "V6-source", "V6-export", "N1", "N2", "N3", "N4", "N5", "N6", "N7", "N8", "N9", "N10", "N11", "usage"]
_POSITIVE = ("A6", "I6", "P6", "V6-source", "V6-export")

def check_native_outcomes(outcomes):
    """Host-outcome checker over {case: {exit, error}} where error is the raw
    stderr text observed by the runner. PASS requires the full ordered
    A6,I6,P6,V6-source,V6-export,N1..N11,usage sequence in input order;
    a missing or cutoff sequence returns INCOMPLETE with no success.
    Positives require exit 0 and empty stderr. Each negative requires exit 1
    and one anchored refused: line parsed as its field-specific native record
    (see interface); arbitrary suffixes, extra refusal lines and backend, io,
    proposal, unsupported, unsatisfied or unknown classes raise GateError.
    Usage requires exit 2 and an anchored usage: prefix and is grammar
    evidence only. PASS reports the actual N9/N10 branches and whether a
    descriptor guard was observed; a compile branch is never counted as one."""
    if not isinstance(outcomes, dict):
        _fail("native outcomes: dict required")
    for k in outcomes:
        if k not in _ORDER:
            _fail("native outcomes: unknown case " + str(k))
    present = list(outcomes.keys())
    if [_ORDER.index(c) for c in present] != sorted(_ORDER.index(c) for c in present):
        _fail("native outcomes: input sequence is out of order; it is not reordered")
    records = {}
    for i, case in enumerate(present):
        o = outcomes[case]
        if not isinstance(o, dict) or set(o.keys()) != {"exit", "error"}:
            _fail("native outcome " + case + ": exactly {exit, error} required")
        _uint(o["exit"], case + ".exit", 0, 255)
        if not isinstance(o["error"], str):
            _fail("native outcome " + case + ".error: raw stderr string required")
        if case in _POSITIVE:
            if o["exit"] != 0:
                if i != len(present) - 1:
                    _fail("downstream case present after failed positive " + case + "; no downstream success is accepted")
                _fail(case + " positive outcome failed with exit " + str(o["exit"]))
            if o["error"] != "":
                _fail(case + " positive outcome must have empty stderr (no refused: text)")
        elif case == "usage":
            if o["exit"] != 2 or not o["error"].startswith("usage: "):
                _fail("usage outcome must be exit 2 with raw stderr starting 'usage: '")
        else:
            if o["exit"] != 1:
                _fail(case + " negative outcome must have expected exit 1")
            records[case] = _native_record(case, o["error"])
    if present != _ORDER:
        return {"format": "claims-native-outcomes/1", "status": "INCOMPLETE", "success": False, "checked": present, "missing": [c for c in _ORDER if c not in outcomes], "parsedNegatives": records, "n9Branch": records.get("N9", {}).get("branch"), "n10Branch": records.get("N10", {}).get("branch"), "downstreamImplied": False, "proofCurveValidatedByThisModule": False}
    return {"format": "claims-native-outcomes/1", "status": "PASS", "success": True, "checked": present, "missing": [], "parsedNegatives": records, "n9Branch": records["N9"]["branch"], "n9DescriptorGuardObserved": records["N9"].get("descriptorGuardObserved", False), "n10Branch": records["N10"]["branch"], "n10DescriptorField": records["N10"].get("descriptorField"), "n10CompileDetail": records["N10"].get("compileDetail"), "n10Reason": records["N10"].get("reason"), "descriptorGuardObserved": records["N10"].get("descriptorGuardObserved", False), "downstreamImplied": False, "usageEvidence": "grammar only; not binding evidence", "proofCurveValidatedByThisModule": False}
