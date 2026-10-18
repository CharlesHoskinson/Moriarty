#!/usr/bin/env python3
"""Public data adapter for the genuine compiled-VI experiment (SOURCE ONLY).

Ordinary Python, standard library only: no subprocess, no network, no
curve or SDK operations. It reads the two public producer outputs and the
frozen B9 blueprint, independently recomputes the VI script hash as
blake2b-224(0x03 || compiledCode bytes) (the inner flat payload is NOT
hashed, following the K10 adapter method), extracts vi.vi.withdraw,
rebuilds the Aiken owner-intent JSON by the explicit intent.ak fragment
order and compares it byte-for-byte with the Core signing_message carried
in the producer output (fail closed on any mismatch), then emits
script.hex, eight raw PlutusV3 ScriptContext terms and manifest.json.

All contexts are FABRICATED LOCAL VM inputs: the 2222.. predecessor, zero
fee, POSIX-style millisecond window example and local UTxOs are not
Preprod genesis, seed, fee, claim or ledger facts, and no old K6 proof
context is relabelled as a ledger context. The native C2 Groth16 proof is
not rerun; VI does not establish that a complete T3 accepts.
"""
import hashlib, json, sys
from copy import deepcopy
from pathlib import Path

C2_HASH = "6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1"
VI_HASH = "fc2266228a6a5072946ba7bef649be56fd984ee851d50e57977c1f64"
THREAD_POLICY = "e315b315819a3c7695eb2d33fb468d2186b7be4cfba485c34850f255"  # s2p hash, chain.json
THREAD_ID = bytes.fromhex("7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a")
SETTLE_TXID = bytes.fromhex("22" * 32)  # fabricated local fixture only
CLAIMANT_VKH = bytes.fromhex("55" * 28)  # fabricated local fixture only
WINDOW_LO, WINDOW_HI = 1700000000000, 1700000060000

def fail(msg): raise ValueError(msg)
def sha256(b): return hashlib.sha256(b).digest()
def blake256(b): return hashlib.blake2b(b, digest_size=32).digest()
def blake224(b): return hashlib.blake2b(b, digest_size=28).digest()

# ---- Plutus Data model: ('I',n) ('B',bytes) ('Constr',i,[f]) ('List',[x]) ('Map',[(k,v)])
def I(n): return ("I", int(n))
def B(b): return ("B", bytes(b))
def C(i, f): return ("Constr", int(i), list(f))
def L(x): return ("List", list(x))
def M(p): return ("Map", list(p))
def uplc(d):
    k = d[0]
    if k == "I": return "I %d" % d[1]
    if k == "B": return "B #%s" % d[1].hex()
    if k == "Constr": return "Constr %d [%s]" % (d[1], ", ".join(uplc(x) for x in d[2]))
    if k == "List": return "List [%s]" % ", ".join(uplc(x) for x in d[1])
    if k == "Map": return "Map [%s]" % ", ".join("(%s, %s)" % (uplc(a), uplc(b)) for a, b in d[1])
    fail("bad Data kind")

# ---- Aiken-compatible JSON reconstruction (intent.ak fragment order) ----
def jstr(b):
    if isinstance(b, str): b = b.encode()
    if any(c < 0x20 or c > 0x7E or c in (0x22, 0x5C) for c in b): fail("string outside escape-free ASCII subset")
    return b'"' + b + b'"'
def juint(n):
    if not (0 <= int(n) < 2**128): fail("UInt out of range")
    return b'"' + str(int(n)).encode() + b'"'
def jobj(pairs): return b"{" + b",".join(jstr(k) + b":" + v for k, v in pairs) + b"}"
def jarr(xs): return b"[" + b",".join(xs) + b"]"
def asset_json(a):
    return jobj([("issuer", jstr(a["issuer"])), ("network", jstr(a["network"])), ("reference", jstr(a["reference"])), ("scale", str(a["scale"]).encode())])
def value_json(v):
    return jobj([("type", jstr("UInt")), ("value", juint(v[1]))]) if v[0] == "UInt" else jobj([("type", jstr("Bool")), ("value", b"true" if v[1] else b"false")])

def aiken_message(facts):
    """Rebuild Core payload + signing_message exactly as intent.ak does."""
    asset = facts["asset"]
    balances = jarr([jobj([("account", jstr(r[0])), ("amount", juint(r[2])), ("asset", asset_json(asset))]) for r in facts["balances"]])
    liabilities = jarr([jobj([("amount", juint(r[3])), ("asset", asset_json(asset)), ("creditor", jstr(r[2])), ("debtor", jstr(r[1])), ("name", jstr(r[0]))]) for r in facts["liabilities"]])
    named = lambda pairs: jobj(sorted(((k, value_json(v)) for k, v in pairs), key=lambda kv: kv[0].encode()))
    scenario = jobj([("balances", balances), ("inputs", named(facts["inputs"].items())), ("liabilities", liabilities), ("state", named(facts["state"].items()))])
    scenario_hash = hashlib.sha256(scenario).hexdigest()
    if scenario_hash != facts["intent"]["scenario_hash"]: fail("scenario hash mismatch vs Core intent")
    auth = facts["intent"]["authority"]
    limits = jarr([jobj([("asset", asset_json(asset)), ("beneficiary", jstr(l["beneficiary"])), ("fee_cap", juint(int(l["fee_cap"]))), ("gross_cap", juint(int(l["gross_cap"]))), ("liability_cap", juint(int(l["liability_cap"]))), ("net_floor", juint(int(l["net_floor"])))]) for l in auth["limits"]])
    authority = jobj([("anchor", jstr(auth["anchor"])), ("limits", limits), ("network", jstr(auth["network"])), ("nonce", juint(int(auth["nonce"]))), ("not_after", juint(int(auth["not_after"]))), ("not_before", juint(int(auth["not_before"]))), ("predecessor", jstr(auth["predecessor"])), ("program_address", jstr(auth["program_address"])), ("public_key_hex", jstr(auth["public_key_hex"])), ("scheme", jstr(facts["scheme"])), ("signer", jstr(auth["signer"])), ("target_hash", jstr(auth["target_hash"]))])
    payload = jobj([("action", jstr(facts["intent"]["action"])), ("assets", jarr([asset_json(asset)])), ("authority", authority), ("profile", jstr("moriarty-owner-intent/2")), ("program_hash", jstr(facts["intent"]["program_hash"])), ("scenario_hash", jstr(scenario_hash)), ("semantics", jstr("moriarty-core/1")), ("source_hash", jstr(facts["intent"]["source_hash"]))])
    if len(payload) > 65536: fail("payload over 64 KiB")
    return b"moriarty-owner-intent/2\x00" + len(payload).to_bytes(4, "big") + payload

def load_producer(path, scheme, key_len):
    doc = json.loads(Path(path).read_text(encoding="utf-8"))
    if doc.get("format") != "moriarty-vi-signer-producer-output/1" or doc.get("scheme") != scheme: fail("producer format/scheme mismatch: " + path)
    if doc.get("core_verify", {}).get("status") != "ok": fail("producer Core verify is not honestly ok: " + path)
    intent = doc["signed_intent"]["intent"]
    if intent["authority"]["public_key_hex"] != doc["public_key_hex"]: fail("public key did not propagate into intent")
    if len(bytes.fromhex(doc["public_key_hex"])) != key_len: fail("public key length mismatch")
    message = bytes.fromhex(doc["signing_message_hex"])
    if message[:23] != b"moriarty-owner-intent/2" or message[23] != 0: fail("signing message tag/NUL mismatch")
    if int.from_bytes(message[24:28], "big") != len(message) - 28: fail("signing message u32 length mismatch")
    if hashlib.sha256(message).hexdigest() != doc["prepared"]["signing_message_sha256"]: fail("signing message SHA-256 mismatch")
    if len(bytes.fromhex(doc["signed_intent"]["signature_hex"])) != 64: fail("signature must be 64 bytes")
    return doc

def base_facts(doc, scheme):
    intent = doc["signed_intent"]["intent"]
    pred, vk_d, shape_d = intent["authority"]["target_hash"].split(":")
    return {
        "scheme": scheme, "intent": intent, "signature": bytes.fromhex(doc["signed_intent"]["signature_hex"]),
        "public_key": bytes.fromhex(doc["public_key_hex"]),
        "asset": {"network": "oracle-unit", "issuer": "invoice-issuer", "reference": "cash", "scale": 0},
        "balances": [("escrow", 0, 109), ("spectator", 0, 41), ("fees", 0, 0), ("seller", 0, 0)],
        "liabilities": [("invoice", "escrow", "seller", 90), ("reserve", "escrow", "buyer", 14)],
        "state": {"marker": ("UInt", 7), "opened": ("Bool", True), "paid": ("UInt", 0)},
        "inputs": {"amount": ("UInt", 35)},
        "predicate": bytes.fromhex(pred), "vk_digest": bytes.fromhex(vk_d), "shape_digest": bytes.fromhex(shape_d),
        "seq": 1, "window": (WINDOW_LO, WINDOW_HI), "native": None,
    }

# ---- PlutusV3 Data builders (constructor order = types.ak declaration order) ----
def outref(tx, ix): return C(0, [B(tx), I(ix)])
def credential_script(h): return C(1, [B(h)])
def credential_key(h): return C(0, [B(h)])
def address(pay, stake_none=True): return C(0, [pay, C(1, [])])
def value(pairs): return M([(B(p), M([(B(n), I(q)) for n, q in ns])) for p, ns in pairs])
def output(addr, val, datum): return C(0, [addr, val, datum, C(1, [])])
def inline(d): return C(2, [d])
def bool_data(b): return C(1, []) if b else C(0, [])
def value_v2(v): return C(0, [I(v[1])]) if v[0] == "UInt" else C(1, [bool_data(v[1])])

def build_context(f):
    message = aiken_message(f)
    digest = sha256(message)
    signer = f["intent"]["authority"]["signer"].encode()
    intents_digest = blake256(len(signer).to_bytes(2, "big") + signer + digest)
    is_schnorr = f["scheme"] == "schnorr_bip340"
    key = C(0, [B(f["public_key"])]) if is_schnorr else C(1, [B(f["public_key"])])
    account = C(0, [B(signer), C(0, []), key, C(1, []) if f["native"] is None else C(0, [credential_key(f["native"])])])
    asset_map = C(0, [C(0, [B(b"oracle-unit"), B(b"invoice-issuer"), B(b"cash"), I(0)]), C(0, [C(0, [])]), I(0)])
    entry = C(0, [B(b"partial"), I(0), B(f["predicate"]), B(f["vk_digest"]), B(f["shape_digest"]), L([B(signer)])])
    catalogue = C(0, [I(0), B(b"oracle-unit"), B(bytes.fromhex(f["intent"]["program_hash"])), B(bytes.fromhex(f["intent"]["source_hash"])), L([asset_map]), L([account]), L([C(0, [B(b"marker"), I(0)]), C(0, [B(b"opened"), I(1)]), C(0, [B(b"paid"), I(0)])]), L([entry]), address(credential_key(CLAIMANT_VKH)), I(0)])
    td = C(0, [I(2), B(THREAD_ID), B(bytes.fromhex(C2_HASH)), catalogue,
               L([value_v2(("UInt", 7)), value_v2(("Bool", True)), value_v2(("UInt", 0))]),
               L([C(0, [I(a), B(n.encode()), I(v)]) for n, a, v in f["balances"]]),
               L([C(0, [B(n.encode()), I(0), B(d.encode()), B(c.encode()), I(v)]) for n, d, c, v in f["liabilities"]]),
               I(1000000), I(f["seq"]), B(bytes(32))])
    thread_addr = address(credential_script(bytes.fromhex(THREAD_POLICY)))
    thread_value = value([("", [("", 1000000)]), (THREAD_POLICY, [(THREAD_ID, 1)])])
    thread_input = C(0, [outref(SETTLE_TXID, 0), output(thread_addr, thread_value, inline(td))])
    auth_intent = f["intent"]["authority"]
    limits = L([C(0, [I(0), B(l["beneficiary"].encode()), I(int(l["gross_cap"])), I(int(l["fee_cap"])), I(int(l["net_floor"])), I(int(l["liability_cap"]))]) for l in auth_intent["limits"]])
    signed_intent = C(0, [B(signer), limits, I(int(auth_intent["not_before"])), I(int(auth_intent["not_after"])), B(f["signature"])])
    shape = C(0, [L([C(0, [B(b"amount"), I(0)])]), L([C(0, [I(0), B(signer)])]), L([]), L([C(1, [I(0), I(0)])]), L([I(0)]), I(1), I(0)])
    settle = C(0, [I(0), shape, L([value_v2(("UInt", 35))]), L([value_v2(("UInt", 7)), value_v2(("Bool", True)), value_v2(("UInt", 0))]), L([I(109)]), L([]), L([C(1, [I(35)])]), L([signed_intent]), L([]), I(0), I(0)])
    cont_output = output(thread_addr, thread_value, inline(td))
    vi_cred = credential_script(bytes.fromhex(VI_HASH))
    auth_shard = C(0, [B(bytes.fromhex(THREAD_POLICY)), L([C(0, [outref(SETTLE_TXID, 0), B(intents_digest)])])])
    lo, hi = f["window"]
    interval = C(0, [C(0, [C(1, [I(lo)]), bool_data(True)]), C(0, [C(1, [I(hi)]), bool_data(True)])])
    redeemers = M([(C(1, [outref(SETTLE_TXID, 0)]), settle), (C(2, [vi_cred]), auth_shard)])
    tx = C(0, [L([thread_input]), L([]), L([cont_output]), I(0), value([]), L([]), M([(vi_cred, I(0))]), interval, L([]), redeemers, M([]), B(bytes(32)), M([]), L([]), C(1, []), C(1, [])])
    # PlutusV3 ScriptInfo Rewarding = constructor 2 [credential]. This is
    # the encoding the fresh actual-input audit must confirm; test_ctx.ak
    # covers only Minting/Spending and is not used here.
    return C(0, [tx, auth_shard, C(2, [vi_cred])]), intents_digest.hex(), digest.hex()

CASES = [
    ("VI-P1-schnorr-positive", "schnorr", None, "ACCEPT_UNIT", "none; genuine BIP340 signature verified by compiled VI"),
    ("VI-P2-ecdsa-positive", "ecdsa", None, "ACCEPT_UNIT", "none; genuine low-S ECDSA signature verified by compiled VI"),
    ("VI-N1-schnorr-signature-bitflip", "schnorr", "sigflip", "SEMANTIC_EVALUATION_FAILURE", "intent.verify Schnorr builtin (source-expected only)"),
    ("VI-N2-ecdsa-signature-bitflip", "ecdsa", "sigflip", "SEMANTIC_EVALUATION_FAILURE", "intent.verify ECDSA builtin (source-expected only)"),
    ("VI-N3-stale-seq-nonce", "schnorr", "stale_seq", "SEMANTIC_EVALUATION_FAILURE", "nonce/seq binding in rebuilt signing message (source-expected only)"),
    ("VI-N4-domain-predicate-bitflip", "ecdsa", "domain", "SEMANTIC_EVALUATION_FAILURE", "domain/2 target_hash binding in rebuilt signing message (source-expected only)"),
    ("VI-N5-window-exceeds-intent", "schnorr", "window", "SEMANTIC_EVALUATION_FAILURE", "intent.window_ok whole-interval check (source-expected only)"),
    ("VI-N6-native-coauthority-absent", "ecdsa", "native", "SEMANTIC_EVALUATION_FAILURE", "settle_intent_native catalogue native_ok (source-expected only)"),
]

def apply_delta(f, delta):
    if delta is None: return f
    f = deepcopy(f)
    if delta == "sigflip": f["signature"] = bytes([f["signature"][0] ^ 1]) + f["signature"][1:]
    elif delta == "stale_seq": f["seq"] = 0
    elif delta == "domain": f["predicate"] = bytes([f["predicate"][0] ^ 1]) + f["predicate"][1:]
    elif delta == "window": f["window"] = (WINDOW_LO, int(f["intent"]["authority"]["not_after"]) + 1)
    elif delta == "native": f["native"] = bytes.fromhex("99" * 28)
    else: fail("unknown delta")
    return f

def main(argv):
    if len(argv) != 5: print("usage: build_inputs.py <b9-applied.json> <producer-schnorr.json> <producer-ecdsa.json> <output-dir>", file=sys.stderr); return 2
    b9_path, schnorr_path, ecdsa_path, out_dir = argv[1], argv[2], argv[3], Path(argv[4])
    if out_dir.exists() and any(out_dir.iterdir()): print("output dir exists and is not empty; refusing to overwrite", file=sys.stderr); return 2
    b9 = json.loads(Path(b9_path).read_text(encoding="utf-8"))
    by_title = {v.get("title"): v for v in b9.get("validators", [])}
    codes = {by_title[t]["compiledCode"] for t in ("vi.vi.withdraw", "vi.vi.else") if t in by_title}
    if len(codes) != 1: fail("VI titles do not share one compiledCode")
    code_hex = codes.pop().lower(); raw = bytes.fromhex(code_hex)
    if by_title["vi.vi.withdraw"].get("hash") != VI_HASH: fail("B9 declared VI hash mismatch")
    if blake224(b"\x03" + raw).hex() != VI_HASH: fail("independently recomputed VI hash mismatch; refusing tool-declared identity")
    docs = {"schnorr": load_producer(schnorr_path, "schnorr_bip340", 32), "ecdsa": load_producer(ecdsa_path, "ecdsa_secp256k1_sha256", 33)}
    bases = {k: base_facts(v, "schnorr_bip340" if k == "schnorr" else "ecdsa_secp256k1_sha256") for k, v in docs.items()}
    for base in bases.values():
        core_message = bytes.fromhex(docs["schnorr" if base["scheme"].startswith("schnorr") else "ecdsa"]["signing_message_hex"])
        if aiken_message(base) != core_message: fail("Core canonical bytes != independent Aiken reconstruction")
    out_dir.mkdir(parents=True, exist_ok=True); (out_dir / "contexts").mkdir()
    (out_dir / "script.hex").write_text(code_hex + "\n", encoding="utf-8")
    manifest_cases = []
    for case_id, which, delta, outcome, guard in CASES:
        facts = apply_delta(bases[which], delta)
        ctx, intents_digest, msg_digest = build_context(facts)
        rel = "contexts/%s.uplc.txt" % case_id
        (out_dir / rel).write_text("(con data (%s))\n" % uplc(ctx), encoding="utf-8")
        manifest_cases.append({"id": case_id, "contextFile": rel, "scheme": facts["scheme"], "expectedOutcome": outcome, "expectedGuard": guard, "intentsDigestHex": intents_digest, "signingMessageSha256Hex": msg_digest})
    manifest = {"format": "moriarty-genuine-vi-public-inputs/1", "scriptTitle": "vi.vi.withdraw", "scriptHashIndependentlyRecomputedHex": VI_HASH, "scriptHashDeclaredHex": VI_HASH, "c2HashHex": C2_HASH, "threadPolicyHex": THREAD_POLICY, "compiledBytes": len(raw), "cases": manifest_cases, "fabricatedLocalInputs": "2222.. predecessor/own outref, 5555.. claimant, zero fee and the millisecond window example are fabricated local VM inputs, not Preprod or ledger facts", "nonClaims": ["Core self-verification is internal agreement, not an independent certificate", "a VI unit result is finite consumer behavior, not compiler, constraint, Groth16, T3 or ledger acceptance", "shape-digest-to-shape binding, custody, claim and statement checks belong to VS/S2 prime and are not established by VI"]}
    (out_dir / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": "GENERATED_AWAITING_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT", "cases": len(manifest_cases)}))
    return 0

if __name__ == "__main__": sys.exit(main(sys.argv))
