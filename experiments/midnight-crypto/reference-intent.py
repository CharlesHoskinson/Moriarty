#!/usr/bin/env python3
"""Independent reference for Moriarty signed owner intent /1 (frozen strict closed JSON profile).

Written from wiki-llm/signed-intent-2026-10-01/PROTOCOL.md without importing or running any Rust or
TypeScript encoder. It never holds a private key: it only builds bytes, validates statements and verifies
signatures with its own secp256k1 arithmetic. A valid signature is possession evidence for a key, never account
authority, and nothing here establishes ledger acceptance.

  reference-intent.py check [VECTORS]   recompute vectors and corpus, compare with the checked-in file
  reference-intent.py write [VECTORS]   recompute and rewrite golden/corpus (inputs stay hand-written)
  reference-intent.py frame             stdin draft or full statement -> reference bytes or rejection class
  reference-intent.py verify            stdin [{statement,signatureHex}] -> independent signature results
"""
import hashlib
import json
import re
import struct
import sys
from pathlib import Path

DEFAULT_VECTORS = Path(__file__).with_name("fixtures") / "intent-vectors.json"
SIGNED_TAG = b"moriarty-signed-intent/1\x00"
OWNER_TAG = b"moriarty-owner-program/1\x00"
SCHEMES = ("schnorr_bip340", "ecdsa_secp256k1_sha256")
FRAMINGS = ("raw", "midnight-sign-data")
MONEY = 2**127 - 1
ROUNDS = 2**128 - 1
CONSTS = {
    "profile": "moriarty-signed-intent/1",
    "core": "moriarty-core/5",
    "sourceProfile": "moriarty-financial-agreement-source/6",
    "authoringProfile": "moriarty-beta/1",
}
PROJECTION = ["authoringProfile", "core", "sourceProfile", "agreementId", "actionName", "selectedActionId", "domain", "asset", "intent"]
TOP = ["profile", "core", "sourceProfile", "authoringProfile", "agreementId", "actionName", "selectedActionId", "sourceSha256", "domain", "asset", "signature", "intent"]
INTENT = ["version", "advertisedSourceHash", "advertisedPolicyDigest", "signer", "nonce", "preHead", "notBefore", "notAfter", "grossCap", "feeCap", "netFloor", "failure", "observations", "disclosures", "retainedEffects", "retainedDuties", "delegation", "recovery", "operation"]
EMPTY_ARRAYS = ["observations", "disclosures", "retainedEffects", "retainedDuties"]
TRANSFER_KEYS = ["kind", "from", "recipient", "feeRecipient", "amount", "fee"]
REPAY_KEYS = ["kind", "payer", "obligationId", "amount", "conversion"]


class Reject(Exception):
    def __init__(self, code, message=""):
        super().__init__(f"{code}: {message}")
        self.code = code


# ---------- bounded strict JSON (duplicates, numbers, lone surrogates, work limits reject) ----------
def _no_number(token):
    raise Reject("input", f"JSON numbers are not part of this protocol: {token}")


def _pairs(pairs):
    out = {}
    for k, v in pairs:
        if k in out:
            raise Reject("input", "duplicate decoded key")
        out[k] = v
    return out


def _walk(v, depth, counter):
    counter[0] += 1
    if depth > 32 or counter[0] > 4096:
        raise Reject("input", "JSON work or depth limit")
    if isinstance(v, str):
        if any(0xD800 <= ord(c) <= 0xDFFF for c in v):
            raise Reject("input", "lone surrogate")
    elif isinstance(v, list):
        for x in v:
            _walk(x, depth + 1, counter)
    elif isinstance(v, dict):
        for k, x in v.items():
            if any(0xD800 <= ord(c) <= 0xDFFF for c in k):
                raise Reject("input", "lone surrogate in key")
            _walk(x, depth + 1, counter)


def parse_strict(text):
    if len(text.encode("utf-8", "surrogatepass")) > 65536:
        raise Reject("input", "byte limit")
    try:
        v = json.loads(text, object_pairs_hook=_pairs, parse_int=_no_number, parse_float=_no_number, parse_constant=_no_number)
    except Reject:
        raise
    except (ValueError, RecursionError) as e:
        raise Reject("input", str(e))
    _walk(v, 0, [0])
    return v


# ---------- canonical JSON, projection, frame ----------
def jstr(s):
    out = ['"']
    for ch in s:
        o = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif o == 8:
            out.append("\\b")
        elif o == 12:
            out.append("\\f")
        elif o == 10:
            out.append("\\n")
        elif o == 13:
            out.append("\\r")
        elif o == 9:
            out.append("\\t")
        elif o < 0x20:
            out.append("\\u%04x" % o)
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def canonical(v):
    if v is None:
        return "null"
    if isinstance(v, str):
        return jstr(v)
    if isinstance(v, list):
        return "[" + ",".join(canonical(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{" + ",".join(jstr(k) + ":" + canonical(v[k]) for k in sorted(v)) + "}"
    raise TypeError(f"not in the closed JSON subset: {type(v)}")


def sha256(b):
    return hashlib.sha256(b).digest()


def tagged(tag, payload):
    return tag + struct.pack(">I", len(payload)) + payload


def owner_program_sha256(statement):
    p = canonical({k: statement[k] for k in PROJECTION}).encode("utf-8")
    return sha256(tagged(OWNER_TAG, p)).hex()


def signing_frame(full):
    c = canonical(full).encode("utf-8")
    f = tagged(SIGNED_TAG, c)
    if len(f) > 16384:
        raise Reject("range", "frame over 16384 bytes")
    return f


def signing_message(frame, framing):
    if framing == "raw":
        return frame
    return b"midnight_signed_message:" + str(len(frame)).encode() + b":" + frame


# ---------- validation ----------
ID_RE = re.compile(r"^[A-Za-z][A-Za-z0-9_]{0,63}$")
DEC_RE = re.compile(r"^(0|[1-9][0-9]*)$")
HEX_RE = re.compile(r"^[0-9a-f]*$")


def _obj(v, keys, code="schema"):
    if not isinstance(v, dict) or set(v) != set(keys) or len(v) != len(keys):
        raise Reject(code, "closed object required")
    return v


def _text(v):
    if not isinstance(v, str) or not v:
        raise Reject("schema", "nonempty text required")
    if len(v.encode("utf-8")) > 1024 or any(ord(c) <= 0x1F or ord(c) == 0x7F for c in v):
        raise Reject("schema", "text limits")
    return v


def _id(v):
    if not isinstance(v, str) or not ID_RE.match(v) or not v.isascii():
        raise Reject("schema", "identifier")
    return v


def _dec(v, maximum):
    if not isinstance(v, str):
        raise Reject("schema", "decimal string required")
    if not DEC_RE.match(v) or not v.isascii():
        raise Reject("range", "noncanonical decimal")
    if len(v) > 39:  # 2^128-1 has 39 digits; longer canonical decimals cannot be in range
        raise Reject("range", "decimal out of range")
    n = int(v)
    if n > maximum:
        raise Reject("range", "decimal out of range")
    return n


def _hex(v, nbytes, code):
    if not isinstance(v, str) or len(v) != nbytes * 2 or not HEX_RE.match(v):
        raise Reject(code, "lowercase hex of exact length required")
    return bytes.fromhex(v)


def _const(v, want):
    if v != want:
        raise Reject("schema", f"expected {want}")


def validate(v, build):
    _obj(v, TOP if build else TOP + ["ownerProgramSha256"])
    for k, w in CONSTS.items():
        _const(v[k], w)
    _id(v["agreementId"])
    _id(v["actionName"])
    _hex(v["sourceSha256"], 32, "hash")
    d = _obj(v["domain"], ["id", "chain", "network"])
    _id(d["id"])
    _text(d["chain"])
    _text(d["network"])
    a = _obj(v["asset"], ["id", "representation", "scale", "symbol"])
    _id(a["id"])
    _text(a["representation"])
    _dec(a["scale"], 18)
    if a["symbol"] is not None:
        _text(a["symbol"])
    sig = _obj(v["signature"], ["scheme", "publicKeyHex", "keyRef", "framing"])
    _text(sig["keyRef"])
    if sig["scheme"] not in SCHEMES:
        raise Reject("schema", "scheme")
    if sig["framing"] not in FRAMINGS:
        raise Reject("schema", "framing")
    key = _hex(sig["publicKeyHex"], 32 if sig["scheme"] == SCHEMES[0] else 33, "key")
    if (lift_x(int.from_bytes(key, "big")) if sig["scheme"] == SCHEMES[0] else decompress(key)) is None:
        raise Reject("key", "not a secp256k1 public key")
    i = _obj(v["intent"], INTENT)
    _const(i["version"], "moriarty-intent/3")
    _const(i["failure"], "success_only")
    _const(i["delegation"], "none")
    _const(i["recovery"], "none")
    for k in EMPTY_ARRAYS:
        if not isinstance(i[k], list) or i[k]:
            raise Reject("schema", "policy collection must be an empty array")
    for k in ("advertisedSourceHash", "advertisedPolicyDigest", "nonce", "preHead"):
        _text(i[k])
    _id(i["signer"])
    nb, na = _dec(i["notBefore"], ROUNDS), _dec(i["notAfter"], ROUNDS)
    if nb > na:
        raise Reject("range", "notBefore after notAfter")
    gross, cap, floor = _dec(i["grossCap"], MONEY), _dec(i["feeCap"], MONEY), _dec(i["netFloor"], MONEY)
    op = i["operation"]
    kind = op.get("kind") if isinstance(op, dict) else None
    if kind == "Transfer":
        _obj(op, TRANSFER_KEYS)
        _const(v["selectedActionId"], "TransferLiteralFee")
        for k in ("from", "recipient", "feeRecipient"):
            _id(op[k])
        amount, fee = _dec(op["amount"], MONEY), _dec(op["fee"], MONEY)
        if op["from"] != i["signer"] or len({op["from"], op["recipient"], op["feeRecipient"]}) != 3:
            raise Reject("schema", "transfer identity mismatch or alias")
        if amount == 0:
            raise Reject("range", "amount must be positive")
        if amount + fee > MONEY:
            raise Reject("range", "amount plus fee overflows")
        if amount + fee > gross or fee > cap or floor > amount:
            raise Reject("range", "transfer exceeds owner bounds")
    elif kind == "Repay":
        _obj(op, REPAY_KEYS)
        _const(v["selectedActionId"], "RepayAccrualFirst")
        _id(op["payer"])
        _id(op["obligationId"])
        _const(op["conversion"], "identity")
        amount = _dec(op["amount"], MONEY)
        if op["payer"] != i["signer"]:
            raise Reject("schema", "payer mismatch")
        if amount == 0:
            raise Reject("range", "amount must be positive")
        if amount > gross or cap != 0 or floor != 0:
            raise Reject("range", "invalid repayment bounds")
    else:
        raise Reject("schema", "unsupported operation")
    if not build and owner_program_sha256(v) != v["ownerProgramSha256"]:
        raise Reject("hash", "owner program digest mismatch")


def reference(command, text):
    """Return the reference result for intent-build / intent-frame or raise Reject."""
    v = parse_strict(text)
    if command not in ("intent-build", "intent-frame"):
        raise Reject("input", "unknown command")
    validate(v, command == "intent-build")
    full = dict(v)
    full["ownerProgramSha256"] = owner_program_sha256(v)
    frame = signing_frame(full)
    message = signing_message(frame, full["signature"]["framing"])
    return {
        "statement": full,
        "owner_program_sha256": full["ownerProgramSha256"],
        "frame_hex": frame.hex(),
        "frame_sha256": sha256(frame).hex(),
        "signing_message_hex": message.hex(),
    }


# ---------- secp256k1: key validation and signature verification (no signing) ----------
P = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
G = (0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798, 0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8)


def lift_x(x):
    if x >= P:
        return None
    c = (pow(x, 3, P) + 7) % P
    y = pow(c, (P + 1) // 4, P)
    if y * y % P != c:
        return None
    return (x, y if y % 2 == 0 else P - y)


def decompress(b):
    if len(b) != 33 or b[0] not in (2, 3):
        return None
    pt = lift_x(int.from_bytes(b[1:], "big"))
    if pt is None:
        return None
    return (pt[0], pt[1] if pt[1] % 2 == b[0] % 2 else P - pt[1])


def add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    if a[0] == b[0] and (a[1] + b[1]) % P == 0:
        return None
    lam = (3 * a[0] * a[0] * pow(2 * a[1], -1, P) if a == b else (b[1] - a[1]) * pow(b[0] - a[0], -1, P)) % P
    x = (lam * lam - a[0] - b[0]) % P
    return (x, (lam * (a[0] - x) - a[1]) % P)


def mul(k, pt):
    r = None
    while k:
        if k & 1:
            r = add(r, pt)
        pt = add(pt, pt)
        k >>= 1
    return r


def tagged_hash(tag, data):
    t = sha256(tag.encode())
    return sha256(t + t + data)


def verify_schnorr(message, pub, sig):
    # Documented native API (k256 0.13.4 `Verifier::verify`, used by midnight-base-crypto schnorr verify/sign): the message is
    # hashed with SHA-256 first and that 32-byte digest is the BIP340 challenge message. Raw/wallet framing bytes are the
    # bytes BEFORE this internal hash. This is not the pure BIP340 raw-message variant.
    message = sha256(message)
    if len(pub) != 32 or len(sig) != 64:
        raise Reject("signature", "length")
    pt = lift_x(int.from_bytes(pub, "big"))
    r, s = int.from_bytes(sig[:32], "big"), int.from_bytes(sig[32:], "big")
    if pt is None:
        raise Reject("key", "bad key")
    if r >= P or s >= N:
        raise Reject("signature", "malformed")
    e = int.from_bytes(tagged_hash("BIP0340/challenge", sig[:32] + pub + message), "big") % N
    R = add(mul(s, G), mul(N - e, pt))
    return R is not None and R[1] % 2 == 0 and R[0] == r


def verify_ecdsa(message, pub, sig):
    pt = decompress(pub)  # canonical SEC1 prefix 02/03 only: the native decoder also accepts a prefix-05 alias, which Rust now rejects first
    if pt is None:
        raise Reject("key", "bad key")
    if len(sig) != 64:
        raise Reject("signature", "length")
    r, s = int.from_bytes(sig[:32], "big"), int.from_bytes(sig[32:], "big")
    if not (1 <= r < N and 1 <= s < N):
        raise Reject("signature", "malformed")
    if s > N // 2:  # low-S only, as observed for the pinned native verifier
        return False
    z = int.from_bytes(sha256(message), "big")
    w = pow(s, -1, N)
    R = add(mul(z * w % N, G), mul(r * w % N, pt))
    return R is not None and R[0] % N == r


def verify_artifact(a):
    try:
        if not isinstance(a, dict) or set(a) != {"statement", "signatureHex"}:
            raise Reject("schema", "artifact shape")
        validate(a["statement"], False)
        sig = _hex(a["signatureHex"], 64, "signature")
        full = a["statement"]
        frame = signing_frame(full)
        message = signing_message(frame, full["signature"]["framing"])
        pub = bytes.fromhex(full["signature"]["publicKeyHex"])
        ok = verify_schnorr(message, pub, sig) if full["signature"]["scheme"] == SCHEMES[0] else verify_ecdsa(message, pub, sig)
        return {"valid": ok, "frame_sha256": sha256(frame).hex(), "error": None}
    except Reject as e:
        return {"valid": None, "frame_sha256": None, "error": e.code}


# ---------- fixtures, independent arithmetic and source extraction ----------
def fill(fixture, scheme, framing, keys):
    d = json.loads(json.dumps(fixture["draft"]))
    d["sourceSha256"] = sha256(fixture["source"].encode("utf-8")).hex()
    d["signature"] = {"scheme": scheme, "publicKeyHex": keys[scheme], "keyRef": d["signature"]["keyRef"], "framing": framing}
    return d


def atoms(text, scale):
    whole, _, frac = text.partition(".")
    if len(frac) != scale:
        raise Reject("range", f"expected {scale} decimals in {text}")
    return int(whole + frac)


def extract_from_source(src):
    """Read owner terms straight from the beta source text (starter shapes only)."""
    m = lambda pat: re.search(pat, src, re.S).groups()
    (agreement,) = m(r"\nagreement (\w+) \{")
    did, chain, network = m(r'domain \w+ = \{ id: "([^"]*)", chain: "([^"]*)", network: "([^"]*)" \}')
    asset = re.search(r'asset \w+ = \{ domain: \w+, id: "([^"]*)", scale: (\d+), representation: "([^"]*)"(?:, symbol: "([^"]*)")? \}', src)
    key, nonce, pre = m(r'key: "([^"]*)", nonce: "([^"]*)", pre_head: "([^"]*)"')
    lo, hi = m(r"rounds\(domain: \w+, from: (\d+), to: (\d+)\)")
    sh, pd = m(r'source_hash: "([^"]*)", policy_digest: "([^"]*)"')
    (action,) = m(r"action (\w+) uses")
    scale = int(asset.group(2))
    consts = {n: atoms(a, scale) for n, a in re.findall(r"const (\w+)(?:: Qty<\w+>)? = (\d+\.\d+) \w+;", src)}
    return {"agreementId": agreement, "actionName": action, "domain": {"id": did, "chain": chain, "network": network},
            "asset": {"id": asset.group(1), "representation": asset.group(3), "scale": str(scale), "symbol": asset.group(4)},
            "keyRef": key, "nonce": nonce, "preHead": pre, "notBefore": lo, "notAfter": hi, "sh": sh, "pd": pd, "consts": consts}


def check_fixture_inputs(f):
    d, e, x = f["draft"], f["expected_scope"], f["expected_economics"]
    i, op = d["intent"], d["intent"]["operation"]
    src = extract_from_source(f["source"])
    assert d["agreementId"] == src["agreementId"] and d["actionName"] == src["actionName"], f["name"]
    assert d["domain"] == src["domain"] and d["asset"] == src["asset"], f["name"]
    assert d["signature"]["keyRef"] == src["keyRef"] and i["nonce"] == src["nonce"] and i["preHead"] == src["preHead"], f["name"]
    assert (i["notBefore"], i["notAfter"]) == (src["notBefore"], src["notAfter"]), f["name"]
    assert (i["advertisedSourceHash"], i["advertisedPolicyDigest"]) == (src["sh"], src["pd"]), f["name"]
    c = src["consts"]
    if e["kind"] == "Transfer":
        amount, fee = c["price"], c["fee"]
        assert (int(op["amount"]), int(op["fee"])) == (amount, fee), f["name"]
        assert int(i["grossCap"]) == amount + fee and int(i["feeCap"]) == fee and int(i["netFloor"]) == amount, f["name"]
        assert int(x["debit"]) == amount + fee and int(x["opening"]) - int(x["debit"]) == int(x["payerAfter"]), f["name"]
        assert sum(int(v) for v in x["credits"].values()) == int(x["debit"]) and x["credits"]["Recipient"] == str(amount), f["name"]
        assert ("Fee" in x["credits"]) == (fee > 0), f["name"]
    else:
        amount = c["amount"]
        assert int(op["amount"]) == amount and int(i["grossCap"]) == amount and i["feeCap"] == "0" and i["netFloor"] == "0", f["name"]
        before, after = x["obligationBefore"], x["obligationAfter"]
        interest = min(amount, int(before["accrued"]))  # accrual first, hand rule from the repayment fixture comment
        principal_paid = amount - interest
        assert int(after["accrued"]) == int(before["accrued"]) - interest, f["name"]
        assert int(after["principal"]) == int(before["principal"]) - principal_paid, f["name"]
        assert (after["status"] == "Settled") == (int(after["principal"]) + int(after["accrued"]) == 0), f["name"]
        assert int(x["debit"]) == amount and int(x["opening"]) - amount == int(x["payerAfter"]), f["name"]
    for k in ("amount", "grossCap", "feeCap", "netFloor"):
        key = k if k != "amount" else None
        assert (e[k] if key is None else e[k]) == (op["amount"] if k == "amount" else i[k]), f"{f['name']} {k}"
    assert e["signer"] == i["signer"] and e["kind"] == op["kind"], f["name"]


# ---------- corpus ----------
def clone(x):
    return json.loads(json.dumps(x))


def reorder(v, mode):
    if isinstance(v, dict):
        keys = sorted(v, reverse=True) if mode == "reverse" else sorted(v, key=lambda k: sha256(k.encode()).hex())
        return {k: reorder(v[k], mode) for k in keys}
    if isinstance(v, list):
        return [reorder(x, mode) for x in v]
    return v


def set_path(obj, path, value):
    cur = obj
    parts = [p for p in path.split("/") if p]
    for p in parts[:-1]:
        cur = cur[p]
    if value is DELETE:
        del cur[parts[-1]]
    else:
        cur[parts[-1]] = value
    return obj


DELETE = object()


def leaf_paths(v, p=""):
    if isinstance(v, dict) and v:
        return [x for k, c in v.items() for x in leaf_paths(c, p + "/" + k)]
    return [p]


def off_curve_x():
    x = 1
    while lift_x(x) is not None:
        x += 1
    return x


def build_corpus(fixtures, keys):
    base_t = fill(fixtures[0], SCHEMES[0], "raw", keys)  # transfer, Schnorr
    base_r = fill(fixtures[3], SCHEMES[1], "midnight-sign-data", keys)  # repay, ECDSA
    corpus = []

    def add_entry(name, family, command, obj, expect, pad=None, text=None):
        t = text if text is not None else json.dumps(obj, ensure_ascii=False)
        want = None
        try:
            r = reference(command, t + (" " * pad if pad else ""))
            want = ("accept", r)
        except Reject as e:
            want = ("reject", e.code)
        if expect == "accept":
            assert want[0] == "accept", f"{name}: oracle rejected {want}"
            e = {"outcome": "accept", **{k: want[1][k] for k in ("statement", "frame_hex", "frame_sha256", "signing_message_hex", "owner_program_sha256")}}
        else:
            codes = expect if isinstance(expect, list) else [expect]
            assert want[0] == "reject" and want[1] in codes, f"{name}: oracle {want} vs manual {expect}"
            e = {"outcome": "reject", "code": expect}
        entry = {"name": name, "family": family, "command": command, "input_text": t, "expect": e}
        if pad:
            entry["pad_spaces"] = pad
        corpus.append(entry)

    def full(draft):
        d = clone(draft)
        d["ownerProgramSha256"] = owner_program_sha256(d)
        return d

    # unicode: exact scalar preservation, no normalization, escape spelling independence
    nfc, nfd = "café", "café"
    assert nfc != nfd
    for label, nonce in [("nfc", nfc), ("nfd", nfd), ("astral", "\U0001F989"), ("bmp-private", ""), ("supplementary-private", "\U00010000"),
                         ("line-separator", "a b"), ("c1-control", "a\u0085b"), ("quote-backslash", 'q"\\'), ("noncharacter", "￿"), ("zwnbsp", "﻿"),
                         ("rtl-override", "a‮b"), ("cjk", "中文"), ("max-bytes-bmp", "é" * 512), ("max-bytes-astral", "\U0001F989" * 256)]:
        d = clone(base_t)
        d["intent"]["nonce"] = nonce
        add_entry(f"unicode nonce {label}", "unicode", "intent-build", d, "accept")
        add_entry(f"unicode nonce {label} ascii-escaped", "unicode", "intent-build", None, "accept", text=json.dumps(d, ensure_ascii=True))
    d = clone(base_t)
    for path, v in [("/domain/chain", "mídnight"), ("/domain/network", "превью"), ("/asset/representation", "canônical"), ("/asset/symbol", "€"), ("/signature/keyRef", "kéy\U0001F511")]:
        set_path(d, path, v)
    add_entry("unicode metadata fields", "unicode", "intent-build", d, "accept")
    d_nfc, d_nfd = clone(base_t), clone(base_t)
    d_nfc["intent"]["nonce"], d_nfd["intent"]["nonce"] = nfc, nfd
    assert reference("intent-build", json.dumps(d_nfc, ensure_ascii=False))["frame_hex"] != reference("intent-build", json.dumps(d_nfd, ensure_ascii=False))["frame_hex"]
    for label, nonce in [("too long bmp", "é" * 513), ("too long astral", "\U0001F989" * 257), ("1025 ascii", "a" * 1025)]:
        d = clone(base_t)
        d["intent"]["nonce"] = nonce
        add_entry(f"scalar-text {label}", "scalar-text", "intent-build", d, "schema")
    for label, nonce in [("empty", ""), ("NUL", "a\u0000b"), ("US 0x1f", "\u001f"), ("DEL", "a\u007fb"), ("BEL", "\u0007"), ("newline", "a\nb"), ("tab", "a\tb")]:
        d = clone(base_t)
        d["intent"]["nonce"] = nonce
        add_entry(f"scalar-text nonce {label}", "scalar-text", "intent-build", d, "schema")
    raw_text = json.dumps(base_t, ensure_ascii=False)
    for label, esc in [("lone high surrogate", "\\ud800"), ("lone low surrogate", "\\udc00"), ("high then ascii", "\\ud83ex"), ("reversed pair", "\\udd89\\ud83e")]:
        add_entry(f"scalar-text {label}", "scalar-text", "intent-build", None, "input", text=raw_text.replace('"nonce": "n1"', f'"nonce": "{esc}"'))
    for label, ctl in [("NUL", "\\u0000"), ("0x1f", "\\u001f"), ("DEL", "\\u007f")]:
        d = clone(base_t)
        d["domain"]["chain"] = "x"
        add_entry(f"scalar-text chain {label}", "scalar-text", "intent-build", None, "schema", text=json.dumps(d).replace('"chain": "x"', f'"chain": "a{ctl}b"'))
    add_entry("scalar-text raw control char inside JSON string", "scalar-text", "intent-build", None, "input", text=raw_text.replace('"n1"', '"a\tb"'))

    # order: object key order, whitespace and escaped key spelling never change the bytes
    for label, st in [("reverse", reorder(base_t, "reverse")), ("hash order", reorder(base_t, "hash"))]:
        add_entry(f"order {label} transfer", "order", "intent-build", st, "accept")
        add_entry(f"order {label} repay", "order", "intent-build", reorder(base_r, label.split()[0]), "accept")
    add_entry("order pretty printed", "order", "intent-build", None, "accept", text=json.dumps(base_t, indent=4))
    add_entry("order tabs and CRLF", "order", "intent-build", None, "accept", text=json.dumps(base_t, indent="\t").replace("\n", "\r\n"))
    add_entry("order compact", "order", "intent-build", None, "accept", text=json.dumps(base_t, separators=(",", ":")))
    add_entry("order escaped key spelling", "order", "intent-build", None, "accept", text=raw_text.replace('"profile"', '"\\u0070rofile"'))
    add_entry("order full statement shuffled via frame", "order", "intent-frame", full(reorder(base_t, "hash")), "accept")
    add_entry("duplicate profile key", "order", "intent-build", None, "input", text=raw_text.replace("{", '{"profile": "moriarty-signed-intent/1", ', 1))
    add_entry("duplicate key through escape", "order", "intent-build", None, "input", text=raw_text.replace("{", '{"\\u0070rofile": "moriarty-signed-intent/1", ', 1))
    add_entry("duplicate nested key", "order", "intent-build", None, "input", text=raw_text.replace('"chain":', '"id": "Midnight", "chain":', 1))
    add_entry("duplicate with different later value", "order", "intent-build", None, "input", text=raw_text.replace("{", '{"profile": "other", ', 1))

    # source labels are opaque claims: accepted verbatim, never interpreted as provenance
    real = base_t["sourceSha256"]
    for label, v in [("64 hex not the real digest", "deadbeef" * 8), ("equals real source digest", real), ("uppercase real digest", real.upper()), ("0x prefixed", "0x" + real), ("short", "x"), ("spaces", " padded "), ("unicode", "häsh")]:
        d = clone(base_t)
        d["intent"]["advertisedSourceHash"] = v
        add_entry(f"source-label advertisedSourceHash {label}", "source-label", "intent-build", d, "accept")
        d = clone(base_t)
        d["intent"]["advertisedPolicyDigest"] = v
        add_entry(f"source-label advertisedPolicyDigest {label}", "source-label", "intent-build", d, "accept")
    a, b = clone(base_t), clone(base_t)
    a["intent"]["advertisedSourceHash"] = real
    b["intent"]["advertisedSourceHash"] = real.upper()
    assert reference("intent-build", json.dumps(a))["frame_hex"] != reference("intent-build", json.dumps(b))["frame_hex"]
    d = clone(base_t)
    d["intent"]["advertisedSourceHash"] = "a\nb"
    add_entry("source-label newline rejected as text", "source-label", "intent-build", d, "schema")
    d = clone(base_t)
    d["intent"]["advertisedPolicyDigest"] = ""
    add_entry("source-label empty rejected", "source-label", "intent-build", d, "schema")

    # hash: owner program digest and source digest
    good = full(base_t)
    add_entry("hash frame accepts the independent digest", "hash", "intent-frame", good, "accept")
    add_entry("hash build refuses supplied digest", "hash", "intent-build", good, "schema")
    for label, v in [("flipped first nibble", ("0" if good["ownerProgramSha256"][0] != "0" else "1") + good["ownerProgramSha256"][1:]), ("uppercase", good["ownerProgramSha256"].upper()),
                     ("63 chars", good["ownerProgramSha256"][:-1]), ("65 chars", good["ownerProgramSha256"] + "0"), ("0x prefix", "0x" + good["ownerProgramSha256"][2:]),
                     ("empty", ""), ("non-hex", "zz" * 32), ("zero", "00" * 32), ("frame sha not owner sha", reference("intent-build", json.dumps(base_t))["frame_sha256"]),
                     ("untagged projection hash", sha256(canonical({k: good[k] for k in PROJECTION}).encode()).hex()),
                     ("signed-intent tag instead of owner tag", sha256(tagged(SIGNED_TAG, canonical({k: good[k] for k in PROJECTION}).encode())).hex()),
                     ("hash of whole statement", sha256(tagged(OWNER_TAG, canonical(base_t).encode())).hex())]:
        d = clone(good)
        d["ownerProgramSha256"] = v
        add_entry(f"hash owner digest {label}", "hash", "intent-frame", d, "hash")
    for v in [None, 7, True, [], {}]:
        d = clone(good)
        d["ownerProgramSha256"] = v
        add_entry(f"hash owner digest type {json.dumps(v)}", "hash", "intent-frame", None, ["hash", "schema", "input"], text=json.dumps(d))
    d = clone(good)
    del d["ownerProgramSha256"]
    add_entry("hash frame without digest", "hash", "intent-frame", d, "schema")
    d = clone(good)
    d["intent"]["nonce"] = "n2"
    add_entry("hash stale digest after a projected change", "hash", "intent-frame", d, "hash")
    d = clone(good)
    d["signature"]["keyRef"] = "key2"
    add_entry("hash digest excludes signature metadata (keyRef not in projection)", "hash", "intent-frame", d, "accept")
    d = clone(good)
    d["sourceSha256"] = "11" * 32
    add_entry("hash digest excludes sourceSha256", "hash", "intent-frame", d, "accept")
    for label, v in [("uppercase", real.upper()), ("63 chars", real[:-1]), ("65 chars", real + "0"), ("0x prefix", "0x" + real[2:]), ("empty", ""), ("non-hex", "zz" * 32), ("null", None), ("bool", True)]:
        d = clone(base_t)
        d["sourceSha256"] = v
        add_entry(f"hash sourceSha256 {label}", "hash", "intent-build", None, ["hash", "schema"], text=json.dumps(d))

    # closed arrays, constants and unknown/missing fields
    for k in EMPTY_ARRAYS:
        for label, v in [("one string", ["x"]), ("null element", [None]), ("nested empty", [[]]), ("empty object element", [{}]), ("object", {}), ("null", None), ("string", ""), ("missing", DELETE)]:
            d = clone(base_t)
            set_path(d, "/intent/" + k, v)
            add_entry(f"closed-empty {k} {label}", "closed-empty", "intent-build", d, "schema")
        d = clone(base_t)
        add_entry(f"closed-empty {k} whitespace-only array accepted", "closed-empty", "intent-build", None, "accept", text=json.dumps(d).replace(f'"{k}": []', f'"{k}": [ \n ]'))
    for k, vals in [("delegation", ["None", "", None, "delegated"]), ("recovery", ["None", "", None, "social"]), ("failure", ["SuccessOnly", "best_effort", None]), ("version", ["moriarty-intent/2", "moriarty-intent/3 ", None])]:
        for v in vals:
            d = clone(base_t)
            d["intent"][k] = v
            add_entry(f"closed-empty {k} {v!r}", "closed-empty", "intent-build", d, "schema")
    for k, vals in [("profile", ["moriarty-signed-intent/2", "Moriarty-signed-intent/1", "", None]), ("core", ["moriarty-core/4"]), ("sourceProfile", ["moriarty-financial-agreement-source/5"]), ("authoringProfile", ["moriarty-beta/2"])]:
        for v in vals:
            d = clone(base_t)
            d[k] = v
            add_entry(f"closed-unknown constant {k} {v!r}", "closed-unknown", "intent-build", d, "schema")
    for p in leaf_paths(base_t):
        d = clone(base_t)
        set_path(d, p, DELETE)
        add_entry(f"closed-unknown missing {p}", "closed-unknown", "intent-build", d, "schema")
    for level in ["", "/domain", "/asset", "/signature", "/intent", "/intent/operation"]:
        for key in ["extra", "2", "10", "0", "-1", "", "a/b"]:
            d = clone(base_t)
            tgt = d if not level else d
            for part in [x for x in level.split("/") if x]:
                tgt = tgt[part]
            tgt[key] = "x"
            add_entry(f"{'numeric-key' if key.lstrip('-').isdigit() else 'closed-unknown'} extra key {key!r} at {level or '/'}", "numeric-key" if key.lstrip("-").isdigit() else "closed-unknown", "intent-build", d, "schema")
    for k in ["extra", "2"]:
        d = clone(good)
        d[k] = "x"
        add_entry(f"{'numeric-key' if k == '2' else 'closed-unknown'} extra key {k!r} in a full statement", "numeric-key" if k == "2" else "closed-unknown", "intent-frame", d, "schema")
    for p, v in [("/intent/operation/amount", True), ("/intent/operation/amount", ["1"]), ("/intent/operation/amount", None), ("/intent/operation/amount", {}), ("/domain", "Midnight"), ("/intent", []), ("/signature/framing", ["raw"]), ("/asset/symbol", ""), ("/asset/symbol", True), ("/intent/signer", None)]:
        d = clone(base_t)
        set_path(d, p, v)
        add_entry(f"closed-unknown type confusion {p} {v!r}", "closed-unknown", "intent-build", d, "schema")
    for label, t in [("top-level array", "[]"), ("top-level string", '"x"'), ("top-level null", "null"), ("empty object", "{}")]:
        add_entry(f"closed-unknown {label}", "closed-unknown", "intent-build", None, "schema", text=t)

    # overflow, exact decimals and hostile numeric spellings
    def amounts(amount, fee, gross, cap, floor):
        d = clone(base_t)
        d["intent"]["operation"].update(amount=amount, fee=fee)
        d["intent"].update(grossCap=gross, feeCap=cap, netFloor=floor)
        return d

    m127, m127s = str(MONEY), str(MONEY + 1)
    add_entry("overflow money max accepted (transfer, zero fee)", "overflow", "intent-build", amounts(m127, "0", m127, "0", m127), "accept")
    add_entry("overflow amount 2^127 rejected", "overflow", "intent-build", amounts(m127s, "0", m127s, "0", "0"), "range")
    add_entry("overflow amount plus fee over money", "overflow", "intent-build", amounts(m127, "1", m127, "1", "0"), "range")
    add_entry("overflow grossCap 2^127 rejected", "overflow", "intent-build", amounts("1", "0", m127s, "0", "0"), "range")
    add_entry("overflow 2^53+1 exact", "overflow", "intent-build", amounts("9007199254740993", "0", "9007199254740993", "0", "0"), "accept")
    add_entry("overflow 2^64 exact", "overflow", "intent-build", amounts("18446744073709551616", "0", "18446744073709551616", "0", "0"), "accept")
    for v in ["01", "+1", "-1", "1e3", "1E3", "1.0", " 1", "1 ", "0x10", "", "١٢", "１", "1_000", "1,000", "00", "-0", "½", "1\n"]:
        d = amounts("1000", "10", "1010", "10", "1000")
        d["intent"]["operation"]["amount"] = v
        add_entry(f"overflow noncanonical decimal {v!r}", "overflow", "intent-build", d, ["schema", "range"])
    d = amounts("1000", "10", "1010", "10", "1000")
    d["intent"]["operation"]["amount"] = "9" * 40
    add_entry("overflow 40-digit decimal", "overflow", "intent-build", d, "range")
    d["intent"]["operation"]["amount"] = "9" * 5000
    add_entry("overflow 5000-digit decimal exceeds text bound", "overflow", "intent-build", d, ["schema", "range"])
    for lit in ["1000", "1e999999", "1E400", "-0", "0.0", "1.5", "NaN", "Infinity", "-Infinity", "123456789012345678901234567890"]:
        add_entry(f"overflow JSON number literal {lit}", "overflow", "intent-build", None, "input", text=raw_text.replace('"amount": "1000"', f'"amount": {lit}'))
    d = clone(base_t)
    d["intent"]["notAfter"] = str(ROUNDS)
    add_entry("overflow notAfter 2^128-1 accepted", "overflow", "intent-build", d, "accept")
    d["intent"]["notAfter"] = str(ROUNDS + 1)
    add_entry("overflow notAfter 2^128 rejected", "overflow", "intent-build", d, "range")
    d = clone(base_t)
    d["intent"].update(notBefore="11", notAfter="10")
    add_entry("overflow reversed validity", "overflow", "intent-build", d, "range")
    d["intent"].update(notBefore="10", notAfter="10")
    add_entry("overflow equal validity accepted", "overflow", "intent-build", d, "accept")
    for v, ok in [("0", True), ("18", True), ("19", False), ("018", False), ("-1", False), ("2.0", False)]:
        d = clone(base_t)
        d["asset"]["scale"] = v
        add_entry(f"overflow scale {v}", "overflow", "intent-build", d, "accept" if ok else ["range", "schema"])
    d = clone(base_t)
    d["asset"]["scale"] = 2
    add_entry("overflow scale as JSON number", "overflow", "intent-build", None, "input", text=json.dumps(d))

    # financial typed constraints and identity
    for label, f in [("zero amount", lambda d: d["intent"]["operation"].update(amount="0")), ("gross below amount plus fee", lambda d: d["intent"].update(grossCap="1009")),
                     ("fee above cap", lambda d: d["intent"].update(feeCap="9")), ("floor above amount", lambda d: d["intent"].update(netFloor="1001"))]:
        d = clone(base_t)
        f(d)
        add_entry(f"identity transfer {label}", "identity", "intent-build", d, "range")
    for label, f in [("from is not signer", lambda d: d["intent"]["operation"].update({"from": "Other"})), ("recipient equals from", lambda d: d["intent"]["operation"].update(recipient="Owner")),
                     ("fee recipient equals from", lambda d: d["intent"]["operation"].update(feeRecipient="Owner")), ("fee recipient equals recipient", lambda d: d["intent"]["operation"].update(feeRecipient="Recipient")),
                     ("selected action is repay", lambda d: d.update(selectedActionId="RepayAccrualFirst")), ("unknown action", lambda d: d.update(selectedActionId="Transfer")),
                     ("unknown kind", lambda d: d["intent"]["operation"].update(kind="Swap"))]:
        d = clone(base_t)
        f(d)
        add_entry(f"identity transfer {label}", "identity", "intent-build", d, "schema")
    d = clone(base_t)
    d["intent"]["operation"].update(amount="1000", fee="0")
    d["intent"].update(grossCap="1000", feeCap="0")
    d["intent"]["operation"]["feeRecipient"] = "Recipient"
    add_entry("identity zero fee still needs distinct fee recipient", "identity", "intent-build", d, "schema")
    for label, f, code in [("fee cap nonzero", lambda d: d["intent"].update(feeCap="1"), "range"), ("net floor nonzero", lambda d: d["intent"].update(netFloor="1"), "range"),
                           ("gross below amount", lambda d: d["intent"].update(grossCap="2999"), "range"), ("zero amount", lambda d: d["intent"]["operation"].update(amount="0"), "range"),
                           ("payer is not signer", lambda d: d["intent"]["operation"].update(payer="Other"), "schema"), ("conversion", lambda d: d["intent"]["operation"].update(conversion="convert"), "schema"),
                           ("selected action is transfer", lambda d: d.update(selectedActionId="TransferLiteralFee"), "schema")]:
        d = clone(base_r)
        f(d)
        add_entry(f"identity repay {label}", "identity", "intent-build", d, code)
    for p in ["/agreementId", "/actionName", "/domain/id", "/asset/id", "/intent/signer", "/intent/operation/recipient", "/intent/operation/feeRecipient"]:
        for label, v in [("empty", ""), ("65 chars", "A" * 65), ("leading digit", "1a"), ("dash", "a-b"), ("dot", "a.b"), ("colon", "a:b"), ("space", "a b"), ("non-ascii", "éa"), ("underscore first", "_a")]:
            if p == "/intent/signer" or p == "/intent/operation/recipient" and False:
                continue
            d = clone(base_t)
            set_path(d, p, v)
            add_entry(f"identity id {p} {label}", "identity", "intent-build", d, "schema")
    for p, v in [("/agreementId", "A" * 64), ("/agreementId", "a_1"), ("/domain/id", "Midnight2"), ("/intent/operation/recipient", "Recipient_2")]:
        d = clone(base_t)
        set_path(d, p, v)
        add_entry(f"identity id {p} legal {v!r}", "identity", "intent-build", d, "accept")
    a, b = clone(base_t), clone(base_t)
    set_path(b, "/domain/id", "midnight")
    assert reference("intent-build", json.dumps(a))["frame_hex"] != reference("intent-build", json.dumps(b))["frame_hex"]

    # keys
    bad_x = off_curve_x()
    bad_x_hex = bad_x.to_bytes(32, "big").hex()
    for label, scheme, key in [("schnorr all ff", SCHEMES[0], "ff" * 32), ("schnorr zero", SCHEMES[0], "00" * 32), ("schnorr x off curve", SCHEMES[0], bad_x_hex), ("schnorr x equals p", SCHEMES[0], f"{P:064x}"),
                               ("schnorr 33 bytes", SCHEMES[0], keys[SCHEMES[1]]), ("schnorr uppercase", SCHEMES[0], keys[SCHEMES[0]].upper()), ("schnorr 0x", SCHEMES[0], "0x" + keys[SCHEMES[0]][2:]), ("schnorr short", SCHEMES[0], keys[SCHEMES[0]][:-2]),
                               ("ecdsa prefix 04", SCHEMES[1], "04" + keys[SCHEMES[0]]), ("ecdsa prefix 00", SCHEMES[1], "00" * 33), ("ecdsa prefix 05", SCHEMES[1], "05" + keys[SCHEMES[0]]), ("ecdsa x off curve", SCHEMES[1], "02" + bad_x_hex),
                               ("ecdsa all ff", SCHEMES[1], "ff" * 33), ("ecdsa 32 bytes", SCHEMES[1], keys[SCHEMES[0]]), ("ecdsa uppercase", SCHEMES[1], keys[SCHEMES[1]].upper()), ("ecdsa 65 bytes uncompressed", SCHEMES[1], "04" + keys[SCHEMES[0]] + "00" * 32)]:
        d = clone(base_t)
        d["signature"].update(scheme=scheme, publicKeyHex=key)
        add_entry(f"key {label}", "key", "intent-build", d, "key")
    for scheme, key in [(SCHEMES[0], keys[SCHEMES[0]]), (SCHEMES[1], keys[SCHEMES[1]]), (SCHEMES[1], "03" + keys[SCHEMES[0]])]:
        d = clone(base_t)
        d["signature"].update(scheme=scheme, publicKeyHex=key)
        add_entry(f"key valid {scheme} {key[:6]}", "key", "intent-build", d, "accept")
    for label, v in [("Schnorr", "Schnorr"), ("uppercase", "SCHNORR_BIP340"), ("alias", "bip340"), ("empty", ""), ("ecdsa short", "ecdsa")]:
        d = clone(base_t)
        d["signature"]["scheme"] = v
        add_entry(f"key scheme {label}", "key", "intent-build", d, "schema")
    for v in ["Raw", "wallet", "midnight_sign_data", "", None, "raw "]:
        d = clone(base_t)
        d["signature"]["framing"] = v
        add_entry(f"key framing {v!r}", "key", "intent-build", d, "schema")

    # input bounds
    add_entry("input-bound exactly 65537 bytes", "input-bound", "intent-build", base_t, "input", pad=65537)
    add_entry("input-bound empty text", "input-bound", "intent-build", None, "input", text="")
    add_entry("input-bound whitespace only", "input-bound", "intent-build", None, "input", text="  \n")
    add_entry("input-bound truncated", "input-bound", "intent-build", None, "input", text=raw_text[:-1])
    add_entry("input-bound trailing object", "input-bound", "intent-build", None, "input", text=raw_text + " {}")
    add_entry("input-bound trailing garbage", "input-bound", "intent-build", None, "input", text=raw_text + "x")
    add_entry("input-bound trailing comma", "input-bound", "intent-build", None, "input", text=raw_text[:-1] + ",}")
    add_entry("input-bound BOM", "input-bound", "intent-build", None, "input", text="﻿" + raw_text)
    add_entry("input-bound single quotes", "input-bound", "intent-build", None, "input", text=raw_text.replace('"', "'"))
    add_entry("input-bound comment", "input-bound", "intent-build", None, "input", text="/*x*/" + raw_text)
    d = clone(base_t)
    d["intent"]["observations"] = "[" * 40 + "]" * 40
    add_entry("input-bound depth over 32", "input-bound", "intent-build", None, "input", text=json.dumps(d).replace('"[' + "[" * 39 + "]" * 40 + '"', "[" * 40 + "]" * 40))
    d["intent"]["observations"] = "@@"
    add_entry("input-bound more than 4096 nodes", "input-bound", "intent-build", None, "input", text=json.dumps(d).replace('"@@"', "[" + ",".join(['"a"'] * 5000) + "]"))
    big = clone(base_t)
    for p in ["/intent/advertisedSourceHash", "/intent/advertisedPolicyDigest", "/intent/nonce", "/intent/preHead", "/signature/keyRef", "/domain/chain", "/domain/network", "/asset/representation", "/asset/symbol"]:
        set_path(big, p, '"' * 1024)
    add_entry("input-bound frame over 16384 bytes via escapes", "input-bound", "intent-build", big, ["range", "input"])
    d = clone(base_t)
    for p in ["/intent/advertisedSourceHash", "/intent/advertisedPolicyDigest", "/intent/nonce", "/intent/preHead", "/signature/keyRef", "/domain/chain", "/domain/network", "/asset/representation", "/asset/symbol"]:
        set_path(d, p, "a" * 1024)
    add_entry("input-bound large but legal ASCII statement", "input-bound", "intent-build", d, "accept")
    return corpus


def write_or_check(path, write):
    doc = json.loads(Path(path).read_text())
    keys = doc["keys"]
    for k, key in keys.items():  # public generator points only
        pt = lift_x(int(key, 16)) if k == SCHEMES[0] else decompress(bytes.fromhex(key))
        assert pt is not None and pt[0] == G[0], f"{k} golden key is not the public generator point"
    for f in doc["fixtures"]:
        check_fixture_inputs(f)
    golden = []
    for f in doc["fixtures"]:
        for scheme in SCHEMES:
            for framing in FRAMINGS:
                d = fill(f, scheme, framing, keys)
                r = reference("intent-build", json.dumps(d))
                golden.append({"fixture": f["name"], "scheme": scheme, "framing": framing, "draft": d, **r})
    corpus = build_corpus(doc["fixtures"], keys)
    names = [c["name"] for c in corpus]
    assert len(names) == len(set(names)), "duplicate corpus names"
    new = dict(doc)
    new["golden"], new["corpus"] = golden, corpus
    new["reference"] = {"file": "reference-intent.py", "golden_count": len(golden), "corpus_count": len(corpus), "independence": "no Rust, TypeScript or Core output used"}
    text = json.dumps(new, indent=1, ensure_ascii=True) + "\n"
    if write:
        Path(path).write_text(text)
        print(json.dumps({"status": "written", "golden": len(golden), "corpus": len(corpus)}))
        return 0
    same = Path(path).read_text() == text
    print(json.dumps({"status": "match" if same else "MISMATCH", "golden": len(golden), "corpus": len(corpus)}))
    return 0 if same else 1


def main(argv):
    cmd = argv[1] if len(argv) > 1 else ""
    if cmd in ("check", "write"):
        return write_or_check(argv[2] if len(argv) > 2 else DEFAULT_VECTORS, cmd == "write")
    if cmd == "frame":
        text = sys.stdin.read()
        command = argv[2] if len(argv) > 2 else "intent-build"
        try:
            print(json.dumps(reference(command, text)))
            return 0
        except Reject as e:
            print(json.dumps({"status": "Rejected", "code": e.code, "error": str(e)}))
            return 2
    if cmd == "verify":
        items = json.load(sys.stdin)
        print(json.dumps([verify_artifact(a) for a in items]))
        return 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
