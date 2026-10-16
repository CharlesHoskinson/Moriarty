#!/usr/bin/env python3
"""Public data adapter for the genuine compiled-VI experiment (SOURCE ONLY).

Ordinary Python, standard library only: no subprocess, no network, no
curve or SDK operations. It reads the two public producer outputs (format
moriarty-vi-signer-producer-output/2, exactly two signed intents each:
escrow, the debit signer, and seller, the invoice-discharge creditor
required by thread.required_signers for the full invoice-partial shape)
and the frozen B9 blueprint. It independently recomputes the VI script
hash as blake2b-224(0x03 || compiledCode bytes) (the inner flat payload
is NOT hashed), extracts vi.vi.withdraw, rebuilds both Aiken
owner-intent messages by the explicit intent.ak fragment order and
compares them byte-for-byte with the Core signing_message bytes carried
in the producer outputs (fail closed on any mismatch), then emits
script.hex, eight raw PlutusV3 ScriptContext terms and manifest.json.

Encodings follow the pinned stdlib SOURCE declarations (source
inspection only; compiled-decoder parity is a fresh-actual-input audit
item, not established here): Transaction has exactly 16 fields in
declaration order; ScriptPurpose Withdraw and ScriptInfo Withdrawing
are both the third variant (constructor 2); Datum InlineDatum is
constructor 2; Credential is VerificationKey 0 / Script 1;
IntervalBoundType Finite is constructor 1.

All contexts are FABRICATED LOCAL VM inputs: the 2222.. predecessor, the
5555.. claimant, zero transaction fee, the millisecond window example,
reserve 1000000 and the zero event chain in the continuation datum are
not Preprod genesis, seed, fee, claim or ledger facts, and no old K6
proof context is relabelled as a ledger context. The catalogue asset
carries NO Cardano unit mapping (unit None): no physical custody leg is
claimed, and custody value, claim, statement and continuation checks
belong to VS/S2 prime and are not established by VI. The fees and
spectator catalogue accounts carry the escrow ephemeral public key as
a disclosed never-signing placeholder key solely to satisfy the
AccountV2 record shape (their key bytes are never verified by VI and
never sign anything); the catalogue is therefore NOT claimed to be
genesis-well-formed. The shape carried in the redeemer is the full
gold partial shape (Fee 2, Transfer 35, Discharge invoice 90->55,
State paid 0->35 over cells escrow/fees/seller and duties
invoice/reserve); its shape-bytes digest equality with the pinned gold
ABI v2 shape digest is an audit gate, because VI itself never checks
that binding (VS does).
"""
import hashlib, json, sys
from copy import deepcopy
from pathlib import Path

C2_HASH = "6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1"
VI_HASH = "fc2266228a6a5072946ba7bef649be56fd984ee851d50e57977c1f64"
THREAD_POLICY_HEX = "e315b315819a3c7695eb2d33fb468d2186b7be4cfba485c34850f255"  # S2P thread policy, chain.json
THREAD_ID_HEX = "7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a"
SOURCE_HASH = "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9"
PROGRAM_HASH = "d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9"
PREDICATE_HEX = "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd"
VK_HEX = "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd"
SHAPE_HEX = "36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969"
TARGET_HASH = PREDICATE_HEX + ":" + VK_HEX + ":" + SHAPE_HEX
SETTLE_TXID_HEX = "22" * 32  # fabricated local own outref only
CLAIMANT_VKH_HEX = "55" * 28  # fabricated local fixture only
WINDOW_LO, WINDOW_HI = 1700000000000, 1700000060000
NOT_BEFORE, NOT_AFTER = 0, 2000000000000
ASSET = {"network": "oracle-unit", "issuer": "invoice-issuer", "reference": "cash", "scale": 0}
BALANCES = [("escrow", 109), ("spectator", 41), ("fees", 0), ("seller", 0)]  # Core request order (signing fact)
LIABILITIES = [("invoice", "escrow", "seller", 90), ("reserve", "escrow", "buyer", 14)]
STATE = {"marker": ("UInt", 7), "opened": ("Bool", True), "paid": ("UInt", 0)}
# Gold-true limits per signer (beneficiary, gross, fee, net floor, closing liability cap).
EXPECTED_LIMITS = {
    "escrow": ("seller", 37, 2, 0, 69),
    "seller": ("seller", 0, 0, 0, 0),
}
SIGNERS = ["escrow", "seller"]

def fail(msg): raise ValueError(msg)
def sha256(b): return hashlib.sha256(b).digest()
def blake256(b): return hashlib.blake2b(b, digest_size=32).digest()
def blake224(b): return hashlib.blake2b(b, digest_size=28).digest()

# ---- strict public JSON: duplicates, non-finite numbers, bool-as-int ----
def _no_dup(pairs):
    out = {}
    for k, v in pairs:
        if k in out: fail("duplicate JSON key: " + str(k))
        out[k] = v
    return out
def _no_const(x): fail("non-finite JSON constant: " + x)
def load_strict(path):
    text = Path(path).read_text(encoding="utf-8")
    return json.loads(text, object_pairs_hook=_no_dup, parse_constant=_no_const)
def req_str(x, what):
    if type(x) is not str: fail(what + " must be a string")
    return x
def req_int(x, what):
    if type(x) is not int: fail(what + " must be an int (bool is not an int)")
    return x
def req_u128(x, what):
    # Core UInt JSON is a canonical decimal string.
    if type(x) is not str or not x.isdigit() or (len(x) > 1 and x[0] == "0"): fail(what + " must be a canonical UInt string")
    n = int(x)
    if not (0 <= n < 2**128): fail(what + " out of u128 range")
    return n
def hex_bytes(s, what, length=None):
    # Explicit hex decoding: never bytes(hex_string), which would be ASCII.
    if type(s) is not str or len(s) % 2 or any(c not in "0123456789abcdef" for c in s): fail(what + " must be lowercase hex")
    b = bytes.fromhex(s)
    if length is not None and len(b) != length: fail(what + " length mismatch")
    return b
def ascii_bytes(s, what):
    # Explicit ASCII label encoding: never bytes(str) with no encoding.
    if type(s) is not str: fail(what + " must be a string")
    try: return s.encode("ascii")
    except UnicodeEncodeError: fail(what + " is not ASCII")

# ---- Plutus Data model: ('I',n) ('B',bytes) ('Constr',i,[f]) ('List',[x]) ('Map',[(k,v)]) ----
def I(n):
    if type(n) is not int: fail("Data int must be an int")
    return ("I", n)
def B(b):
    if type(b) is not bytes: fail("Data bytes must be bytes, not str")
    return ("B", b)
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
    if isinstance(b, str): b = b.encode("ascii")
    if any(c < 0x20 or c > 0x7E or c in (0x22, 0x5C) for c in b): fail("string outside escape-free ASCII subset")
    return b'"' + b + b'"'
def juint(n):
    if type(n) is not int or not (0 <= n < 2**128): fail("UInt out of range")
    return b'"' + str(n).encode() + b'"'
def jobj(pairs): return b"{" + b",".join(jstr(k) + b":" + v for k, v in pairs) + b"}"
def jarr(xs): return b"[" + b",".join(xs) + b"]"
def asset_json(a):
    return jobj([("issuer", jstr(a["issuer"])), ("network", jstr(a["network"])), ("reference", jstr(a["reference"])), ("scale", str(req_int(a["scale"], "asset scale")).encode())])
def value_json(v):
    return jobj([("type", jstr("UInt")), ("value", juint(v[1]))]) if v[0] == "UInt" else jobj([("type", jstr("Bool")), ("value", b"true" if v[1] else b"false")])
def scenario_bytes():
    balances = jarr([jobj([("account", jstr(a)), ("amount", juint(n)), ("asset", asset_json(ASSET))]) for a, n in BALANCES])
    liabilities = jarr([jobj([("amount", juint(n)), ("asset", asset_json(ASSET)), ("creditor", jstr(c)), ("debtor", jstr(d)), ("name", jstr(nm))]) for nm, d, c, n in LIABILITIES])
    named = lambda pairs: jobj(sorted(((k, value_json(v)) for k, v in pairs), key=lambda kv: kv[0].encode()))
    return jobj([("balances", balances), ("inputs", named({"amount": ("UInt", 35)}.items())), ("liabilities", liabilities), ("state", named(STATE.items()))])
def owner_message(signer, public_key_hex, scheme, limits, nonce, predicate_hex):
    """Rebuild one Core signing_message exactly as intent.ak does.
    limits: list of (beneficiary, gross, fee, net, liability) tuples."""
    scenario_hash = hashlib.sha256(scenario_bytes()).hexdigest()
    lim = jarr([jobj([("asset", asset_json(ASSET)), ("beneficiary", jstr(b)), ("fee_cap", juint(f)), ("gross_cap", juint(g)), ("liability_cap", juint(l)), ("net_floor", juint(nf))]) for (b, g, f, nf, l) in limits])
    target = predicate_hex + ":" + VK_HEX + ":" + SHAPE_HEX
    authority = jobj([("anchor", jstr("cardano-plutus-v3:" + THREAD_POLICY_HEX)), ("limits", lim), ("network", jstr("oracle-unit")), ("nonce", juint(nonce)), ("not_after", juint(NOT_AFTER)), ("not_before", juint(NOT_BEFORE)), ("predecessor", jstr(SETTLE_TXID_HEX + "#0")), ("program_address", jstr("cardano-thread:" + THREAD_ID_HEX)), ("public_key_hex", jstr(public_key_hex)), ("scheme", jstr(scheme)), ("signer", jstr(signer)), ("target_hash", jstr(target))])
    payload = jobj([("action", jstr("partial")), ("assets", jarr([asset_json(ASSET)])), ("authority", authority), ("profile", jstr("moriarty-owner-intent/2")), ("program_hash", jstr(PROGRAM_HASH)), ("scenario_hash", jstr(scenario_hash)), ("semantics", jstr("moriarty-core/1")), ("source_hash", jstr(SOURCE_HASH))])
    if len(payload) > 65536: fail("payload over 64 KiB")
    return b"moriarty-owner-intent/2\x00" + len(payload).to_bytes(4, "big") + payload, scenario_hash

def load_producer(path, scheme, key_len):
    doc = load_strict(path)
    if not isinstance(doc, dict) or doc.get("format") != "moriarty-vi-signer-producer-output/2" or doc.get("scheme") != scheme: fail("producer format/scheme mismatch: " + path)
    if doc.get("source_hash") != SOURCE_HASH or doc.get("program_hash") != PROGRAM_HASH: fail("producer source/program identity mismatch: " + path)
    intents = doc.get("intents")
    if not isinstance(intents, list) or len(intents) != 2: fail("producer must carry exactly two intents: " + path)
    out = {}
    for pos, signer in enumerate(SIGNERS):
        entry = intents[pos]
        if not isinstance(entry, dict) or entry.get("signer") != signer: fail("producer intent signer/order mismatch: " + path)
        signed = entry.get("signed_intent")
        prepared = entry.get("prepared")
        if not isinstance(signed, dict) or not isinstance(prepared, dict): fail("producer intent shape mismatch: " + path)
        if prepared.get("intent") != signed.get("intent"): fail("prepared and signed intents differ: " + path)
        intent = signed["intent"]
        auth = intent.get("authority") if isinstance(intent, dict) else None
        if not isinstance(auth, dict): fail("producer authority missing: " + path)
        if auth.get("public_key_hex") != entry.get("public_key_hex"): fail("public key did not propagate into intent: " + path)
        key = hex_bytes(entry["public_key_hex"], "public key", key_len)
        if key_len == 33 and key[0] not in (2, 3): fail("ECDSA key must be compressed SEC1: " + path)
        if intent.get("profile") != "moriarty-owner-intent/2" or intent.get("semantics") != "moriarty-core/1" or intent.get("action") != "partial": fail("producer intent profile/action mismatch: " + path)
        if intent.get("source_hash") != SOURCE_HASH or intent.get("program_hash") != PROGRAM_HASH: fail("producer intent hash mismatch: " + path)
        if auth.get("target_hash") != TARGET_HASH or auth.get("network") != "oracle-unit" or auth.get("signer") != signer: fail("producer domain mismatch: " + path)
        if auth.get("anchor") != "cardano-plutus-v3:" + THREAD_POLICY_HEX or auth.get("program_address") != "cardano-thread:" + THREAD_ID_HEX: fail("producer anchor/address mismatch: " + path)
        if auth.get("predecessor") != SETTLE_TXID_HEX + "#0" or req_u128(auth.get("nonce"), "nonce") != 1: fail("producer predecessor/nonce mismatch: " + path)
        if req_u128(auth.get("not_before"), "not_before") != NOT_BEFORE or req_u128(auth.get("not_after"), "not_after") != NOT_AFTER: fail("producer window mismatch: " + path)
        limits = auth.get("limits")
        if not isinstance(limits, list) or len(limits) != 1: fail("producer limits shape mismatch: " + path)
        lim = limits[0]
        got = (lim.get("beneficiary"), req_u128(lim.get("gross_cap"), "gross"), req_u128(lim.get("fee_cap"), "fee"), req_u128(lim.get("net_floor"), "net"), req_u128(lim.get("liability_cap"), "liability"))
        if got != EXPECTED_LIMITS[signer]: fail("producer limits are not the gold-true limits for " + signer + ": " + path)
        if lim.get("asset") != ASSET: fail("producer limit asset mismatch: " + path)
        # Producer core_verify is recorded, never trusted as external
        # verification: this adapter independently rebuilds the message
        # bytes below, and the genuine verifier is the compiled VI.
        if entry.get("core_verify", {}).get("status") != "ok": fail("producer Core verify is not honestly ok (completeness check only): " + path)
        message = hex_bytes(entry.get("signing_message_hex"), "signing message")
        if message[:23] != b"moriarty-owner-intent/2" or message[23] != 0: fail("signing message tag/NUL mismatch")
        if int.from_bytes(message[24:28], "big") != len(message) - 28: fail("signing message u32 length mismatch")
        if prepared.get("signing_message_hex") != entry["signing_message_hex"]: fail("prepared message mismatch: " + path)
        if hashlib.sha256(message).hexdigest() != prepared.get("signing_message_sha256"): fail("signing message SHA-256 mismatch")
        signature = hex_bytes(signed.get("signature_hex"), "signature", 64)
        rebuilt, scenario_hash = owner_message(signer, entry["public_key_hex"], scheme, [EXPECTED_LIMITS[signer]], 1, PREDICATE_HEX)
        if rebuilt != message: fail("Core canonical bytes != independent Aiken reconstruction for " + signer)
        if intent.get("scenario_hash") != scenario_hash: fail("producer scenario hash mismatch for " + signer)
        out[signer] = {"key": key, "key_hex": entry["public_key_hex"], "signature": signature}
    return out

# ---- PlutusV3 Data builders (constructor order = stdlib declaration order) ----
def outref(tx_bytes, ix): return C(0, [B(tx_bytes), I(ix)])
def credential_script(h): return C(1, [B(h)])   # Credential: VerificationKey 0, Script 1
def credential_key(h): return C(0, [B(h)])
def address(pay_cred): return C(0, [pay_cred, C(1, [])])  # stake credential None
def some_d(x): return C(0, [x])
def none_d(): return C(1, [])
def value(pairs):
    # pairs: [(policy_bytes, [(token_bytes, qty)])]; empty bytes are the
    # lovelace policy/token, hex policies are decoded bytes, never ASCII.
    return M([(B(p), M([(B(n), I(q)) for n, q in ns])) for p, ns in pairs])
def output(addr, val, datum): return C(0, [addr, val, datum, none_d()])
def inline(d): return C(2, [d])  # Datum: NoDatum 0, DatumHash 1, InlineDatum 2
def bool_data(b): return C(1, []) if b else C(0, [])
def value_v2(v): return C(0, [I(v[1])]) if v[0] == "UInt" else C(1, [bool_data(v[1])])

def build_context(intents, scheme, delta):
    case = {"seq": 1, "predicate_hex": PREDICATE_HEX, "window": (WINDOW_LO, WINDOW_HI), "escrow_native": None, "signatures": {s: intents[s]["signature"] for s in SIGNERS}}
    if delta == "sigflip":
        s = case["signatures"]["escrow"]
        case["signatures"]["escrow"] = bytes([s[0] ^ 1]) + s[1:]  # documented single delta: escrow signature only
    elif delta == "stale_seq": case["seq"] = 0
    elif delta == "domain":
        p = bytes.fromhex(PREDICATE_HEX)
        case["predicate_hex"] = (bytes([p[0] ^ 1]) + p[1:]).hex()  # catalogue action-entry predicate only
    elif delta == "window": case["window"] = (WINDOW_LO, NOT_AFTER + 1)  # whole tx interval exceeds both intent windows
    elif delta == "native": case["escrow_native"] = bytes.fromhex("99" * 28)  # absent native co-authority (VK 99..)
    elif delta is not None: fail("unknown delta")
    # VI-rebuilt messages for the intents digest (nonce = datum seq, target
    # predicate = catalogue entry), digests as in thread.settle_messages.
    digest_parts = b""
    msg_digests = {}
    for signer in SIGNERS:
        msg, _ = owner_message(signer, intents[signer]["key_hex"], scheme, [EXPECTED_LIMITS[signer]], case["seq"], case["predicate_hex"])
        d = sha256(msg)
        msg_digests[signer] = d.hex()
        name = ascii_bytes(signer, "signer")
        digest_parts += len(name).to_bytes(2, "big") + name + d
    intents_digest = blake256(digest_parts)
    key_data = lambda b: C(0, [B(b)]) if scheme == "schnorr_bip340" else C(1, [B(b)])
    escrow_key = key_data(intents["escrow"]["key"])
    def account(name, kd, native):
        return C(0, [B(ascii_bytes(name, "account")), C(0, []), kd, none_d() if native is None else some_d(credential_key(native))])
    # Account order is strictly ascending by name. fees/spectator keys are
    # the disclosed never-signing escrow-key placeholder (see docstring).
    accounts = L([
        account("escrow", escrow_key, case["escrow_native"]),
        account("fees", escrow_key, None),
        account("seller", key_data(intents["seller"]["key"]), None),
        account("spectator", escrow_key, None),
    ])
    asset_map = C(0, [C(0, [B(ascii_bytes("oracle-unit", "network")), B(ascii_bytes("invoice-issuer", "issuer")), B(ascii_bytes("cash", "reference")), I(0)]), none_d(), I(0)])  # unit None: unmapped
    entry = C(0, [B(ascii_bytes("partial", "action")), I(1), B(hex_bytes(case["predicate_hex"], "predicate", 32)), B(hex_bytes(VK_HEX, "vk", 32)), B(hex_bytes(SHAPE_HEX, "shape", 32)), L([B(ascii_bytes("escrow", "advancer"))])])  # action index 1 = program declaration order (open, partial, finish)
    catalogue = C(0, [I(0), B(ascii_bytes("oracle-unit", "network")), B(hex_bytes(PROGRAM_HASH, "program hash", 32)), B(hex_bytes(SOURCE_HASH, "source hash", 32)), L([asset_map]), accounts, L([C(0, [B(ascii_bytes("marker", "state")), I(0)]), C(0, [B(ascii_bytes("opened", "state")), I(1)]), C(0, [B(ascii_bytes("paid", "state")), I(0)])]), L([entry]), address(credential_key(hex_bytes(CLAIMANT_VKH_HEX, "claimant", 28))), I(0)])
    def thread_datum(state_vals, balances, liabilities, seq):
        return C(0, [I(2), B(hex_bytes(THREAD_ID_HEX, "thread id", 32)), B(hex_bytes(C2_HASH, "c2 hash", 28)), catalogue,
                     L([value_v2(v) for v in state_vals]),
                     L([C(0, [I(0), B(ascii_bytes(a, "balance account")), I(n)]) for a, n in balances]),
                     L([C(0, [B(ascii_bytes(nm, "duty")), I(0), B(ascii_bytes(d, "debtor")), B(ascii_bytes(c, "creditor")), I(n)]) for nm, d, c, n in liabilities]),
                     I(1000000), I(seq), B(bytes(32))])
    pre_td = thread_datum([("UInt", 7), ("Bool", True), ("UInt", 0)], BALANCES, LIABILITIES, case["seq"])
    post_balances = [("escrow", 72), ("spectator", 41), ("fees", 2), ("seller", 35)]  # same order as pre; spectator untouched
    post_liabilities = [("invoice", "escrow", "seller", 55), ("reserve", "escrow", "buyer", 14)]
    next_td = thread_datum([("UInt", 7), ("Bool", True), ("UInt", 35)], post_balances, post_liabilities, case["seq"] + 1)  # zero event chain: fabricated, not VS-checked here
    thread_addr = address(credential_script(hex_bytes(THREAD_POLICY_HEX, "thread policy", 28)))
    thread_value = value([(b"", [(b"", 1000000)]), (hex_bytes(THREAD_POLICY_HEX, "thread policy", 28), [(hex_bytes(THREAD_ID_HEX, "thread id", 32), 1)])])
    own_ref = outref(hex_bytes(SETTLE_TXID_HEX, "settle txid", 32), 0)
    thread_input = C(0, [own_ref, output(thread_addr, thread_value, inline(pre_td))])
    def signed_intent_data(signer):
        b, g, f, nf, l = EXPECTED_LIMITS[signer]
        limits = L([C(0, [I(0), B(ascii_bytes(b, "beneficiary")), I(g), I(f), I(nf), I(l)])])
        return C(0, [B(ascii_bytes(signer, "signer")), limits, I(NOT_BEFORE), I(NOT_AFTER), B(case["signatures"][signer])])
    # Full gold partial shape: cells (escrow, fees, seller), duties
    # (invoice, reserve), effects Fee->Transfer->Discharge->State(paid).
    shape = C(0, [L([C(0, [B(ascii_bytes("amount", "input")), I(0)])]),
                  L([C(0, [I(0), B(ascii_bytes("escrow", "cell"))]), C(0, [I(0), B(ascii_bytes("fees", "cell"))]), C(0, [I(0), B(ascii_bytes("seller", "cell"))])]),
                  L([C(0, [B(ascii_bytes("invoice", "duty")), I(0), B(ascii_bytes("escrow", "debtor")), B(ascii_bytes("seller", "creditor"))]), C(0, [B(ascii_bytes("reserve", "duty")), I(0), B(ascii_bytes("escrow", "debtor")), B(ascii_bytes("buyer", "creditor"))])]),
                  L([C(2, [I(0), I(1)]), C(1, [I(0), I(2)]), C(3, [I(0), bool_data(True)]), C(0, [I(2)])]),  # EFee, ETransfer, ELiability(discharge), EState(paid=state 2)
                  L([I(0)]), I(8), I(0)])  # debit cells [escrow]; 8 program statements; 0 liability-creating statements
    settle = C(0, [I(0), shape, L([value_v2(("UInt", 35))]),
                   L([value_v2(("UInt", 7)), value_v2(("Bool", True)), value_v2(("UInt", 35))]),
                   L([I(72), I(2), I(35)]),
                   L([C(0, [bool_data(True), I(55)]), C(0, [bool_data(True), I(14)])]),
                   L([C(1, [I(2)]), C(1, [I(35)]), C(2, [I(90), I(55)]), C(0, [value_v2(("UInt", 0)), value_v2(("UInt", 35))])]),  # EvAmount fee, EvAmount transfer, EvDuty, EvState
                   L([signed_intent_data("escrow"), signed_intent_data("seller")]), L([]), I(0), I(0)])
    cont_output = output(thread_addr, thread_value, inline(next_td))
    vi_cred = credential_script(hex_bytes(VI_HASH, "vi hash", 28))
    auth_shard = C(0, [B(hex_bytes(THREAD_POLICY_HEX, "thread policy", 28)), L([C(0, [own_ref, B(intents_digest)])])])
    lo, hi = case["window"]
    bound = lambda t: C(0, [C(1, [I(t)]), bool_data(True)])  # Finite (constructor 1), inclusive
    interval = C(0, [bound(lo), bound(hi)])
    spend_purpose = C(1, [own_ref])       # ScriptPurpose: Mint 0, Spend 1, Withdraw 2
    withdraw_purpose = C(2, [vi_cred])    # ScriptPurpose Withdraw (third variant)
    redeemers = M([(spend_purpose, settle), (withdraw_purpose, auth_shard)])
    # Transaction: exactly the 16 stdlib fields in declaration order.
    tx = C(0, [L([thread_input]), L([]), L([cont_output]), I(0), value([]), L([]), M([(vi_cred, I(0))]), interval, L([]), redeemers, M([]), B(bytes(32)), M([]), L([]), none_d(), none_d()])
    # ScriptInfo Withdrawing (third variant, constructor 2) [credential].
    ctx = C(0, [tx, auth_shard, C(2, [vi_cred])])
    return ctx, intents_digest.hex(), msg_digests

CASES = [
    ("VI-P1-schnorr-positive", "schnorr", None, "ACCEPT_UNIT", "none; genuine BIP340 signatures (escrow and seller) verified by compiled VI"),
    ("VI-P2-ecdsa-positive", "ecdsa", None, "ACCEPT_UNIT", "none; genuine low-S ECDSA signatures (escrow and seller) verified by compiled VI"),
    ("VI-N1-schnorr-signature-bitflip", "schnorr", "sigflip", "SEMANTIC_EVALUATION_FAILURE", "intent.verify Schnorr builtin over the escrow message (source-expected only)"),
    ("VI-N2-ecdsa-signature-bitflip", "ecdsa", "sigflip", "SEMANTIC_EVALUATION_FAILURE", "intent.verify ECDSA builtin over the escrow message (source-expected only)"),
    ("VI-N3-stale-seq-nonce", "schnorr", "stale_seq", "SEMANTIC_EVALUATION_FAILURE", "nonce/seq binding in rebuilt signing messages (source-expected only)"),
    ("VI-N4-domain-predicate-bitflip", "ecdsa", "domain", "SEMANTIC_EVALUATION_FAILURE", "domain/2 target_hash predicate binding in rebuilt signing messages (source-expected only)"),
    ("VI-N5-window-exceeds-intent", "schnorr", "window", "SEMANTIC_EVALUATION_FAILURE", "intent.window_ok whole-interval check (source-expected only)"),
    ("VI-N6-native-coauthority-absent", "ecdsa", "native", "SEMANTIC_EVALUATION_FAILURE", "settle_intent_native catalogue native_ok for escrow (source-expected only)"),
]

def main(argv):
    if len(argv) != 5: print("usage: build_inputs.py <b9-applied.json> <producer-schnorr.json> <producer-ecdsa.json> <output-dir>", file=sys.stderr); return 2
    b9_path, schnorr_path, ecdsa_path, out_dir = argv[1], argv[2], argv[3], Path(argv[4])
    if out_dir.exists() and any(out_dir.iterdir()): print("output dir exists and is not empty; refusing to overwrite", file=sys.stderr); return 2
    b9 = load_strict(b9_path)
    if not isinstance(b9, dict): fail("B9 blueprint must be an object")
    by_title = {v.get("title"): v for v in b9.get("validators", []) if isinstance(v, dict)}
    codes = {by_title[t]["compiledCode"] for t in ("vi.vi.withdraw", "vi.vi.else") if t in by_title}
    if len(codes) != 1: fail("VI titles do not share one compiledCode")
    code_hex = codes.pop().lower(); raw = hex_bytes(code_hex, "compiledCode")
    if by_title["vi.vi.withdraw"].get("hash") != VI_HASH: fail("B9 declared VI hash mismatch")
    if blake224(b"\x03" + raw).hex() != VI_HASH: fail("independently recomputed VI hash mismatch; refusing tool-declared identity")
    docs = {"schnorr": load_producer(schnorr_path, "schnorr_bip340", 32), "ecdsa": load_producer(ecdsa_path, "ecdsa_secp256k1_sha256", 33)}
    schemes = {"schnorr": "schnorr_bip340", "ecdsa": "ecdsa_secp256k1_sha256"}
    out_dir.mkdir(parents=True, exist_ok=True); (out_dir / "contexts").mkdir()
    (out_dir / "script.hex").write_text(code_hex + "\n", encoding="utf-8")
    manifest_cases = []
    for case_id, which, delta, outcome, guard in CASES:
        ctx, intents_digest, msg_digests = build_context(docs[which], schemes[which], delta)
        rel = "contexts/%s.uplc.txt" % case_id
        (out_dir / rel).write_text("(con data (%s))\n" % uplc(ctx), encoding="utf-8")
        manifest_cases.append({"id": case_id, "contextFile": rel, "scheme": schemes[which], "expectedOutcome": outcome, "expectedGuard": guard, "intentsDigestHex": intents_digest, "signingMessageSha256Hex": msg_digests})
    manifest = {"format": "moriarty-genuine-vi-public-inputs/2", "scriptTitle": "vi.vi.withdraw", "scriptHashIndependentlyRecomputedHex": VI_HASH, "scriptHashDeclaredHex": VI_HASH, "c2HashHex": C2_HASH, "threadPolicyHex": THREAD_POLICY_HEX, "programHashHex": PROGRAM_HASH, "sourceHashHex": SOURCE_HASH, "shapeDigestPinnedHex": SHAPE_HEX, "compiledBytes": len(raw), "cases": manifest_cases, "fabricatedLocalInputs": "2222.. predecessor/own outref, 5555.. claimant, zero transaction fee, millisecond window example, reserve 1000000 and zero continuation event chain are fabricated local VM inputs, not Preprod or ledger facts", "cataloguePlaceholders": "fees and spectator accounts carry the escrow ephemeral public key as a never-signing placeholder; the catalogue is not claimed genesis-well-formed; the CASH asset is unmapped (unit None), so no custody value is claimed", "nonClaims": ["Core self-verification is internal agreement, not an independent certificate", "a VI unit result is finite consumer behaviour, not compiler, constraint, Groth16, T3, VS, custody, claim, statement, continuation or ledger acceptance", "shape-digest-to-shape binding is pinned from gold and must be confirmed by the fresh actual-input audit; VI itself does not check that binding", "all 20 UTxO families, ACTUS, DeFi and general obligations, recursion, private handoffs, bounded multi-parent PCD, universal certificates and real Preprod remain open"]}
    (out_dir / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": "GENERATED_AWAITING_FRESH_ACTUAL_PUBLIC_INPUT_AUDIT", "cases": len(manifest_cases)}))
    return 0

if __name__ == "__main__": sys.exit(main(sys.argv))
