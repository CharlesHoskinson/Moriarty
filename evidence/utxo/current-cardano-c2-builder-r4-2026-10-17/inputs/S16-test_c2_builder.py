"""Ordinary tests for build_k10_inputs_r3.py (C2-BUILDER-CONTRACT.md CB01-CB12).

AUTHOR-PROPOSED by claude-opus-5-5 HIGH; never run or reviewed. Stdlib only: no
subprocess, network, curve, key, signing, proof, native or VM action. Writes go
only under ../scratch. Expected bytes come from literal historical reproduction
vectors and the independent formatters below, never from the code under test.
"""
import contextlib
import copy
import hashlib
import importlib.util
import io
import json
import os
import sys
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCRATCH = HERE.parent / "scratch"
sys.path.insert(0, str(HERE))  # -I implies -P; siblings resolve only from HERE

B9 = "inputs/b9-applied.json"
EXP = "inputs/K6-PUBLIC-PROOF.json"
SET = "inputs/K2-PUBLIC-SETUP-RECORD.json"
HIST = "inputs/HIST-09-positive-script-context.txt"
PINS = {
    "host_codec.py": (23252, "3c18539ebb277595192769a0d0d245e34ef26601303df62366863b76c07b1459"),
    "build_k10_inputs.py": (24480, "6c1cde7f971a4c3ade25a3089f22f557fe2290b3b1266c4c18b6b4b7ce918d00"),
    EXP: (5800, "930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b"),
    SET: (3926, "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"),
    B9: (None, "10baa1794e0ae286cd780978408552fc0e994426367c2b92117e2c4b46ad1d36"),
}
K6_EXPIRY = 1893456000000
POLICY = "6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1"
K6_TOKEN = "9fd5c53f6a1a03d20b0409f2ca3ad28bd02d5401564603fab5f81b289cc6d54e"
K6_PROOF_SHA = "ac7c60f332f77b8db6368ac27da91ed7385378add91c64f32529843a4a75a172"
FAMILY = b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2"
CASES = ("K10-T1-positive-genuine-issue", "K10-T5-quantity-two-reject",
         "K10-T6-token-name-bitflip-reject", "K10-T7-vk-digest-bitflip-reject")
GAPS = ("K10-T2-n1-body-digest-pairing-reject", "K10-T3-n2-context-digest-pairing-reject",
        "K10-T4-n3-proof-ac-swap-pairing-reject")
T1 = "contexts/%s.uplc.txt" % CASES[0]
ROSTER = sorted(["contexts/%s.uplc.txt" % c for c in CASES] + ["manifest.json", "script.hex"])
CASE_KEYS = {"contextFile", "envStatementDigestHex", "envVkDigestHex", "envelopeCborHex",
             "id", "proofHexSha256", "quantity", "tokenNameHex"}
MANIFEST_KEYS = set((
    "auditSourcePins builderLabels builderRevision cases claimantVkhHex "
    "compiledValidatorTitle consumerFormat consumerLabels coverageGaps expiry flags "
    "format inputs issueCborHex limitations mintValueCborHex policyIdHex "
    "positiveEnvelopeCborHex positiveTokenNameHex proofBytes scriptCompiledBytes "
    "scriptHashAlgorithm scriptHashClaimedHex scriptHashClassification "
    "scriptHashIndependentlyRecomputedHex scriptHashPreimage scriptHexFile "
    "scriptInnerFlatBytes settleOutref setupRecordSha256 statementDigestHex vkBytes "
    "vkDigestHex").split())
CONSUMER_LABELS = {
    "expiry": "CALLER_SUPPLIED_NOT_PROOF_BOUND", "context": "CALLER_DECLARED_UNAUTHENTICATED",
    "policyId": "CALLER_SUPPLIED_UNAUTHENTICATED", "trustProfile": "single-party-test-setup/1",
    "setupRecordLink": "DECLARED_NOT_BYTE_CHECKED", "groth16": "NOT_VERIFIED",
    "vkCurveValidity": "NOT_CHECKED", "ledger": "NO_CLAIM"}
BUILDER_LABELS = {
    "contextProvenance": "FABRICATED_LOCAL_VM_TEST_NOT_A_CARDANO_TRANSACTION",
    "setupRecordLink": "FILE_BYTES_SHA256_EQUALS_EXPORT_DECLARATION",
    "policyId": "RECOMPUTED_FROM_PINNED_HISTORICAL_B9_WRAPPER",
    "uplcBridge": "LEGACY_RENDERER_UNCHANGED_PER_CALL_CBOR_EQUALITY",
    "negativeCases": "BYTE_MUTATIONS_OF_ONE_GENUINE_PAYLOAD_NO_NEW_PROOFS",
    "transaction": "NO_FEE_MINADA_COLLATERAL_EXUNITS_FIT_OR_AUTHENTICITY_CLAIM",
    "operatorCompatibility": "NOT_CLAIMED"}
FLAGS = dict.fromkeys(("native", "operator", "wallet", "prover", "vm", "deploy",
                       "previewTested", "preprodTested", "groth16Verified"), False)
LIMITATIONS = [
    "Fabricated local VM test context (zero fee, empty inputs, zero txid, everything interval); not a Cardano transaction.",
    "Expiry is caller supplied and not proof bound; context is caller declared and unauthenticated.",
    "Single-party TEST ONLY setup; Groth16, curve validity, fees, min-ADA, ExUnits and ledger acceptance are not checked or claimed.",
]
_R3 = None


def _read(path, cap=16 << 20):
    with open(path, "rb") as handle:
        data = handle.read(cap + 1)
    if len(data) > cap:
        raise RuntimeError("prerequisite exceeds read cap: %s" % path)
    return data


def setUpModule():
    global RAW, EXPORT, SETUP, legacy, c2
    RAW = {}
    for rel, (size, sha) in PINS.items():
        data = _read(HERE / rel)
        if (size is not None and len(data) != size) or hashlib.sha256(data).hexdigest() != sha:
            raise RuntimeError("prerequisite pin mismatch: " + rel)
        RAW[rel] = data
    RAW[HIST] = _read(HERE / HIST)
    if not SCRATCH.is_dir() or SCRATCH.is_symlink() or os.listdir(SCRATCH):
        raise RuntimeError("prerequisite: scratch must be an empty directory")
    EXPORT, SETUP = json.loads(RAW[EXP]), json.loads(RAW[SET])
    import build_k10_inputs as legacy
    import c2_issue_artifacts as c2


def _builder(case):
    global _R3
    path = HERE / "build_k10_inputs_r3.py"
    if not path.is_file():
        case.fail("missing implementation: build_k10_inputs_r3.py")
    if _R3 is None:
        spec = importlib.util.spec_from_file_location("build_k10_inputs_r3", path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)  # import failures are ERRORs, never RED
        _R3 = module
    return _R3


def b2(data):
    return hashlib.blake2b(data, digest_size=32).digest()


def flip1(data):
    return bytes([data[0] ^ 1]) + data[1:]


def dumps(obj):
    return json.dumps(obj).encode()


def uint(n):
    if n < 24:
        return bytes([n])
    for ai, width in ((24, 1), (25, 2), (26, 4), (27, 8)):
        if n < 1 << 8 * width:
            return bytes([ai]) + n.to_bytes(width, "big")


def env_cbor(pred, vkd, cl, tx, ix, sd, exp):
    """EXPECTATIONS cborSegments: tag 121, indefinite list, definite fields of <=32 bytes."""
    b32 = bytes.fromhex("5820")
    return b"".join([bytes.fromhex("d8799f0200"), b32, FAMILY, b32, pred, b32, vkd,
                     bytes.fromhex("581c"), cl, bytes.fromhex("d8799f"), b32, tx, uint(ix),
                     bytes.fromhex("ff"), b32, sd, uint(exp), bytes.fromhex("ff")])


def ctx_text(token, qty, fields, vk, proof):
    """Historical 09 layout: fabricated local VM test context, not a transaction."""
    pred, vkd, cl, tx, ix, sd, exp = fields
    val = "Map [(B #%s, Map [(B #%s, I %d)])]" % (POLICY, token.hex(), qty)
    env = ("Constr 0 [I 2, I 0, B #%s, B #%s, B #%s, B #%s, Constr 0 [B #%s, I %d], B #%s, I %d]"
           % (FAMILY.hex(), pred.hex(), vkd.hex(), cl.hex(), tx.hex(), ix, sd.hex(), exp))
    out = ("Constr 0 [Constr 0 [Constr 1 [B #%s], Constr 1 []], %s, Constr 2 [%s], Constr 1 []]"
           % (POLICY, val, env))
    info = ("Constr 0 [List [], List [], List [%s], I 0, %s, List [], Map [], Constr 0 [Constr 0 "
            "[Constr 0 [], Constr 1 []], Constr 0 [Constr 2 [], Constr 1 []]], List [], Map [], "
            "Map [], B #%s, Map [], List [], Constr 1 [], Constr 1 []]" % (out, val, "00" * 32))
    return ("(con data (Constr 0 [%s, Constr 0 [B #%s, B #%s], Constr 0 [B #%s]]))\n"
            % (info, vk, proof, POLICY)).encode()


def k6_fields():
    return (bytes.fromhex(EXPORT["predicateDigest"]), bytes.fromhex(EXPORT["vkDigestBlake2b256"]),
            bytes.fromhex("55" * 28), bytes.fromhex("22" * 32), 0,
            bytes.fromhex(EXPORT["statementDigestBlake2b256"]), K6_EXPIRY)


def plan(r3, export=None, setup=None, expiry=K6_EXPIRY):
    return r3.plan_outputs(RAW[B9], RAW[EXP] if export is None else export,
                           RAW[SET] if setup is None else setup, expiry)


def refused(case, code, fn, *args):
    with case.assertRaises(ValueError) as caught:
        fn(*args)
    message = str(caught.exception)
    case.assertTrue(message.startswith(code + ":"), message)
    return message


def run(r3, *args):
    out, err = io.StringIO(), io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
        code = r3.main(["build_k10_inputs_r3.py"] + [str(a) for a in args])
    return code, out.getvalue(), err.getvalue()


def tree(root):
    return sorted(str(p.relative_to(root)) + ("/" if p.is_dir() else "") for p in root.rglob("*"))


class BuilderTests(unittest.TestCase):
    def test_01_prerequisites_and_historical_vectors(self):
        f = k6_fields()
        vk, pr = SETUP["verifyingKeyHex"], EXPORT["proofHex"]
        self.assertEqual(legacy.frozen_positive_cbor(f[1], f[5]), env_cbor(*f))
        self.assertEqual(b2(env_cbor(*f)).hex(), K6_TOKEN)
        self.assertEqual(hashlib.sha256(bytes.fromhex(pr)).hexdigest(), K6_PROOF_SHA)
        want = ctx_text(bytes.fromhex(K6_TOKEN), 1, f, vk, pr)
        self.assertEqual(RAW[HIST], want)
        old = legacy.script_context(bytes.fromhex(POLICY), legacy.env_data(f[1], f[5]),
                                    bytes.fromhex(K6_TOKEN), 1, bytes.fromhex(vk), bytes.fromhex(pr))
        self.assertEqual(("(con data (%s))\n" % legacy.uplc_term(old)).encode(), want)
        self.assertEqual(legacy.extract_compiled(RAW[B9])[1], POLICY)
        self.assertTrue(callable(c2.prepare_c2_issue))

    def test_02_k6_reproduces_historical_positive_context(self):
        r3 = _builder(self)
        a = plan(r3)
        self.assertEqual(sorted(a), ROSTER)
        self.assertTrue(all(type(v) is bytes for v in a.values()))
        self.assertEqual(a[T1], RAW[HIST])
        self.assertEqual(a["script.hex"], (legacy.extract_compiled(RAW[B9])[0] + "\n").encode())
        m = json.loads(a["manifest.json"])
        self.assertEqual(a["manifest.json"], (json.dumps(m, indent=2, sort_keys=True) + "\n").encode())
        self.assertEqual((m["positiveEnvelopeCborHex"], m["positiveTokenNameHex"], m["policyIdHex"]),
                         (env_cbor(*k6_fields()).hex(), K6_TOKEN, POLICY))
        self.assertEqual(m["mintValueCborHex"], "a1581c" + POLICY + "a15820" + K6_TOKEN + "01")
        self.assertEqual(plan(r3), a)

    def test_03_fields_are_drawn_from_consumer_not_constants(self):
        r3 = _builder(self)
        # NEGATIVE FIXTURE: the genuine K6 proof is reused unchanged and is not valid for
        # this edited statement; it exercises field drawing only and proves nothing.
        cl, tx, ix, exp = bytes.fromhex("66" * 28), bytes.fromhex("33" * 32), 7, 1700000000000
        e = copy.deepcopy(EXPORT)
        st = bytearray(bytes.fromhex(e["statementHex"]))
        st[144:172], st[233:265], st[265:267] = cl, tx, ix.to_bytes(2, "big")
        d = b2(bytes(st))
        e.update(statementHex=st.hex(), statementDigestBlake2b256=d.hex(), publicInputs={
            "x1": str(int.from_bytes(d[:16], "big")), "x2": str(int.from_bytes(d[16:], "big"))})
        e["context"].update(claimantVkh=cl.hex(), consumedTxid=tx.hex(), consumedIx=ix)
        e["cardano"]["envV2"].update(claimantVkh=cl.hex(), statementDigest=d.hex(),
                                     settleOutref={"transactionId": tx.hex(), "outputIndex": ix})
        a = plan(r3, dumps(e), None, exp)
        f = (bytes.fromhex(e["predicateDigest"]), bytes.fromhex(e["vkDigestBlake2b256"]), cl, tx, ix, d, exp)
        tok = b2(env_cbor(*f))
        self.assertEqual(a[T1], ctx_text(tok, 1, f, SETUP["verifyingKeyHex"], e["proofHex"]))
        m = json.loads(a["manifest.json"])
        self.assertEqual((m["claimantVkhHex"], m["settleOutref"], m["expiry"]),
                         (cl.hex(), {"transactionIdHex": tx.hex(), "outputIndex": ix}, exp))
        self.assertEqual((m["positiveEnvelopeCborHex"], m["positiveTokenNameHex"]),
                         (env_cbor(*f).hex(), tok.hex()))
        blob = b"".join(v for k, v in a.items() if k != "script.hex")
        for stale in (b"55" * 28, b"22" * 32, b"1893456000000", K6_TOKEN.encode()):
            self.assertNotIn(stale, blob)

    def test_04_setup_file_bytes_link_is_checked_not_declared(self):
        r3 = _builder(self)
        self.assertEqual(r3.check_setup_link(EXPORT, RAW[SET]), PINS[SET][1])
        refused(self, "C2B-LINK", plan, r3, None, RAW[SET] + b" ")
        for bad in ("0" * 64, PINS[SET][1].upper(), PINS[SET][1][:63], None, 7):
            e = copy.deepcopy(EXPORT)
            e["setupRecordSha256"] = bad
            refused(self, "C2B-LINK", plan, r3, dumps(e))
        e = copy.deepcopy(EXPORT)
        del e["setupRecordSha256"]
        refused(self, "C2B-LINK", plan, r3, dumps(e))

    def test_05_strict_bounded_json(self):
        r3 = _builder(self)
        t = RAW[EXP].decode()
        act, idx = '"action": "partial"', '"actionIndex": 1'
        texts = [t.replace(act, act + ", " + act, 1), "[" + t + "]", t + "{}",
                 t.replace(act, '"action": "' + chr(92) + 'ud800"', 1)]
        texts += [t.replace(idx, '"actionIndex": ' + v, 1)
                  for v in ("NaN", "Infinity", "-Infinity", "1.0", "1e0", "1" * 21)]
        raws = [x.encode() for x in texts] + [
            bytes.fromhex("efbbbf") + RAW[EXP], RAW[EXP].replace(b"partial", bytes.fromhex("ff") + b"partial", 1),
            b"[" * 100000, b" " * 1048577]
        for i, raw in enumerate(raws):
            with self.subTest(i=i):
                self.assertNotEqual(raw, RAW[EXP])
                refused(self, "C2B-JSON", plan, r3, raw)
        dup = RAW[SET].replace(b'"curve"', b'"curve": "x", "curve"', 1)
        for raw in (dup, b" " * 65537, b"[]"):
            self.assertNotEqual(raw, RAW[SET])
            refused(self, "C2B-JSON", plan, r3, None, raw)

    def test_06_expiry_is_explicit_canonical_u64_decimal(self):
        r3 = _builder(self)
        for text, value in (("0", 0), ("1893456000000", K6_EXPIRY), ("18446744073709551615", 2 ** 64 - 1)):
            got = r3.parse_expiry(text)
            self.assertIs(type(got), int)
            self.assertEqual(got, value)
        for text in ("", "01", "+1", "-1", " 1", "1 ", "1\n", "1e3", "1_0", "0x1", chr(0x661),
                     "18446744073709551616", "9" * 21, b"1", 1, None):
            with self.subTest(text=repr(text)):
                refused(self, "C2B-ARGV", r3.parse_expiry, text)
        for value in (True, -1, 2 ** 64, "1", 1.0):
            refused(self, "C2B-ARGV", plan, r3, None, None, value)

    def test_07_consumer_refusals_propagate_in_stage_order(self):
        r3 = _builder(self)
        e = copy.deepcopy(EXPORT)
        e["expiry"] = K6_EXPIRY
        self.assertIn("C2I-EXPORT:", refused(self, "C2B-CONSUMER", plan, r3, dumps(e)))
        s = copy.deepcopy(SETUP)
        s["notice"] = "edited"
        sraw = dumps(s)
        e2 = copy.deepcopy(EXPORT)
        e2["setupRecordSha256"] = hashlib.sha256(sraw).hexdigest()
        self.assertIn("C2I-SETUP:", refused(self, "C2B-CONSUMER", plan, r3, dumps(e2), sraw))
        refused(self, "C2B-LINK", plan, r3, dumps(e), sraw)
        refused(self, "C2B-JSON", plan, r3, b"{", b"{")
        refused(self, "C2B-POLICY", r3.plan_outputs, RAW[B9] + b" ", dumps(e), RAW[SET], K6_EXPIRY)

    def test_08_bridge_fails_closed_on_any_consumer_disagreement(self):
        r3 = _builder(self)
        real = c2.prepare_c2_issue
        good = real(copy.deepcopy(EXPORT), copy.deepcopy(SETUP), K6_EXPIRY, policy_id=bytes.fromhex(POLICY))
        f = k6_fields()
        self.assertEqual(r3.legacy_env_from_host(good["envData"]), legacy.env_data(f[1], f[5]))
        env, iss = good["envData"], good["issueData"]
        swap = lambda h: ("1" if h[0] == "0" else "0") + h[1:]
        edits = [("envCborHex", swap(good["envCborHex"])),
                 ("envelopeTokenNameHex", swap(good["envelopeTokenNameHex"])),
                 ("mintValueCborHex", swap(good["mintValueCborHex"])),
                 ("issueCborHex", swap(good["issueCborHex"])),
                 ("expiry", K6_EXPIRY + 1), ("policyIdHex", "00" * 28), ("format", "x"), ("extra", 1),
                 ("envData", ("constr", 1, env[2])), ("envData", ("constr", 0, env[2][:8])),
                 ("envData", ("constr", 0, [("int", True)] + env[2][1:])),
                 ("issueData", ("constr", 0, [("bytes", bytearray(iss[2][0][1])), iss[2][1]])),
                 ("issueData", ("constr", 0, [iss[2][0], ("bytes", iss[2][1][1][:191])]))]
        for key, value in edits:
            def fake(*args, _k=key, _v=value, **kwargs):
                result = real(*args, **kwargs)
                result[_k] = _v
                return result
            c2.prepare_c2_issue = fake
            try:
                with self.subTest(key=key):
                    refused(self, "C2B-BRIDGE", plan, r3)
            finally:
                c2.prepare_c2_issue = real

    def test_09_derived_negatives_coverage_gaps_and_manifest(self):
        r3 = _builder(self)
        a = plan(r3)
        m = json.loads(a["manifest.json"])
        f, tok = k6_fields(), bytes.fromhex(K6_TOKEN)
        vk, pr = SETUP["verifyingKeyHex"], EXPORT["proofHex"]
        bad = (f[0], flip1(f[1])) + f[2:]
        want = {CASES[1]: ctx_text(tok, 2, f, vk, pr), CASES[2]: ctx_text(flip1(tok), 1, f, vk, pr),
                CASES[3]: ctx_text(b2(env_cbor(*bad)), 1, bad, vk, pr)}
        for case, text in want.items():
            self.assertEqual(a["contexts/%s.uplc.txt" % case], text)
        self.assertEqual([c["id"] for c in m["cases"]], list(CASES))
        self.assertEqual([set(c) for c in m["cases"]], [CASE_KEYS] * 4)
        self.assertEqual([c["quantity"] for c in m["cases"]], [1, 2, 1, 1])
        self.assertEqual(m["cases"][3]["envelopeCborHex"], env_cbor(*bad).hex())
        self.assertEqual({c["proofHexSha256"] for c in m["cases"]}, {K6_PROOF_SHA})
        self.assertEqual(m["coverageGaps"], [{"id": g, "status": "NOT_GENERATED_HISTORICAL_FIXTURE_ONLY"} for g in GAPS])
        self.assertEqual(set(m), MANIFEST_KEYS)
        self.assertEqual((m["format"], m["builderRevision"], m["consumerFormat"], m["expiry"]),
                         ("cardano-k2-k10-public-inputs/2", "build_k10_inputs_r3", "moriarty-c2-issue-artifacts/1", K6_EXPIRY))
        self.assertEqual((m["consumerLabels"], m["builderLabels"], m["flags"], m["limitations"]),
                         (CONSUMER_LABELS, BUILDER_LABELS, FLAGS, LIMITATIONS))
        self.assertEqual(m["inputs"], [{"role": r, "sha256": hashlib.sha256(RAW[p]).hexdigest(), "bytes": len(RAW[p])}
                                       for r, p in (("b9-applied", B9), ("proof-export", EXP), ("setup-record", SET))])
        self.assertEqual(m["setupRecordSha256"], PINS[SET][1])

    def test_10_cli_reads_writes_and_refuses_with_exact_effects(self):
        r3 = _builder(self)
        ins = [HERE / B9, HERE / EXP, HERE / SET]
        out, pre, link, k6 = SCRATCH / "out-k6", SCRATCH / "pre-existing", SCRATCH / "link-out", "1893456000000"
        code, so, se = run(r3, *ins, k6, out)
        self.assertEqual((code, se, so.count("\n")), (0, "", 1))
        self.assertEqual(json.loads(so), {"output": str(out), "cases": 4})
        for rel, data in plan(r3).items():
            self.assertEqual((out / rel).read_bytes(), data)
        pre.mkdir()
        os.symlink(pre, link)
        (SCRATCH / "oversize.json").write_bytes(b" " * 1048577)
        os.symlink(HERE / EXP, SCRATCH / "input-link.json")
        cases = [((*ins, k6), "C2B-ARGV"), ((*ins, "01", SCRATCH / "n1"), "C2B-ARGV"),
                 ((*ins, k6, out), "C2B-OUTDIR"), ((*ins, k6, pre), "C2B-OUTDIR"),
                 ((*ins, k6, link), "C2B-OUTDIR"), ((*ins, k6, SCRATCH / "Bad Name"), "C2B-OUTDIR"),
                 ((*ins, k6, SCRATCH / "absent" / "n2"), "C2B-OUTDIR"),
                 ((ins[0], SCRATCH / "oversize.json", ins[2], k6, SCRATCH / "n3"), "C2B-READ"),
                 ((ins[0], SCRATCH / "input-link.json", ins[2], k6, SCRATCH / "n4"), "C2B-READ"),
                 ((ins[0], SCRATCH / "absent.json", ins[2], k6, SCRATCH / "n5"), "C2B-READ"),
                 ((ins[0], ins[1], SCRATCH, k6, SCRATCH / "n6"), "C2B-READ")]
        for args, want in cases:
            with self.subTest(want=want, last=str(args[-1])):
                code, so, se = run(r3, *args)
                self.assertEqual((code, so), (2, ""))
                self.assertTrue(se.startswith(want + ":") and se.endswith("\n") and se.count("\n") == 1, se)
        self.assertEqual(sorted(os.listdir(SCRATCH)),
                         ["input-link.json", "link-out", "out-k6", "oversize.json", "pre-existing"])
        self.assertEqual((tree(out), os.listdir(pre)), (sorted(ROSTER + ["contexts/"]), []))
        for rel in (B9, EXP, SET):
            self.assertEqual(_read(HERE / rel), RAW[rel])


if __name__ == "__main__":
    unittest.main(verbosity=2)
