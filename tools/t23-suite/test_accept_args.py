#!/usr/bin/env python3
"""Tests of accept_args.py. Run: python3 -I tools/t23-suite/test_accept_args.py (or pytest).

Oracles, none computed by accept_args itself:
- ENC-1/ENC-2 vectors (research/sherlock-enc-fp3-20261112/VECTORS.txt, branch t17-preview; Rust
  reference enc_vectors.rs.txt:283-351) and the FP-3 closure values (fp3-closure/VALUES.txt;
  stark-backend enc-fp3 65d1a03 crates/stark-sdk/tests/fp3_closure.rs:551-571, 650-693).
- The T17 (a) honest acceptRun on Preview (fixtures-accept/: the T10m `lfr_call args` output, the
  ledger-args.json that t10a-ledger-run.mjs sent, the honest and R1 call vectors, the receipt).
"""
import copy
import hashlib
import json
import pathlib
import sys
import unittest

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import accept_args as A  # noqa: E402

FX = HERE / "fixtures-accept"


def h(x):
    return int(x, 16)


class Primitives(unittest.TestCase):
    def test_p2_constants(self):
        self.assertEqual(A.p2(1, []), [1601720553, 1813189633, 533296967, 1863848096, 1186913909, 1231646054,
                                       526551043, 780997282])  # enc2.D_EMPTY
        self.assertEqual(A.val_none(), [177090646, 988898068, 1058487993, 1357020063, 70888938, 1990745353,
                                        257326398, 976638622])  # enc1.VAL_NONE

    def test_tags(self):
        self.assertEqual(A.tag_fr("SHK_PROGC"), h("43474f52505f4b4853"))
        self.assertEqual(A.tag_fr("SHK_OUTCOME"), h("454d4f4354554f5f4b4853"))

    def test_transient_hash_parity(self):
        par = {0: "0", 1: "2600648c3ade18be1e76095f9caf3824d4e2c49dd2f6058b3dda172304a4671d",
               2: "4ad818f39d91567d105c5bea1ec4b5ac201dc45b784e39a2beef781790bf5177",
               3: "2416a898714a84833f3690e09279c3b175418c956399a8c8928b8d1d4150ab7b",
               5: "03d92ce21dccdc1597cdbbcd35545745f21fd11bd29ce099e3f5b3d6bad41f17",
               16: "245c287e5ed0d239d638a1dc0068a093d78949809be88148806e146fae7dee4e"}
        for n, want in par.items():
            self.assertEqual(A.transient_hash(list(range(1, n + 1))), h(want), n)

    def test_digest_1to8(self):
        d2 = A.pack([1, 2, 3, 4, 5, 6, 7, 8])
        self.assertEqual(A.digest("SHK_PROGC", d2), h("4e5ebfb63419246bf08041fbd42eeb9f224456652546bc7ee6d132d3b7e1db97"))
        self.assertEqual(A.digest("SHK_OUTCOME", d2), h("71d60d6aea20059dcd39489435709a56c6e1c2cccc5cd54ba28259ef8b265028"))

    def test_pack_roundtrip(self):
        w = [P - 1 for P in [A.P] * 8]
        self.assertEqual(A.unpack(A.pack(w)), w)


class Enc1Chain(unittest.TestCase):
    """enc1: r_P 101..108, prog_d 201..208, salt 1..8, VAL_NONE, fuel 2^64, stack 2^16, Rejected(7)."""

    def test_enc1(self):
        r_p, prog_d = list(range(101, 109)), list(range(201, 209))
        self.assertEqual(A.p2(A.DOM_PROGC, r_p + prog_d),
                         [520957427, 984145394, 1989647505, 623846375, 1856043986, 928030188, 771931842, 1420568668])
        ind = A.input_d(list(range(1, 9)), A.val_none(), 1 << 64, 1 << 16)
        self.assertEqual(ind, [1844765863, 1946263212, 480254560, 1610161683, 900750315, 731375867, 384171174, 1479089448])
        self.assertEqual(A.outcome_rejected(7),
                         [1050408218, 1118957329, 591472421, 1582239, 1404349968, 1660012201, 1063109601, 1001026748])
        res = A.build({"program": {"prog_d": prog_d, "r_p": r_p},
                       "input": {"salt": list(range(1, 9)), "val": {"val_d": A.val_none()}, "fuel": 1 << 64,
                                 "stack_limit": 1 << 16},
                       "outcome": {"rejected": 7}})
        self.assertEqual(res["c_p"], h("65f6874b0be5aaa5a42b2e3b6b94ee9adf10dad60d7eff1211c65e3331ff2994"))
        self.assertEqual(res["input_pub"], h("675781f2fdaf30f1e06631db92a4806ee474b86159f82f31d650f648fb6fc555"))
        self.assertEqual(res["outcome_pub"], h("5eeb4e2b1dd1044fcbb1391358218509b3b3623b96682a5ec4ca8e71183ee5fe"))


# FP-3 closure collection runs k = 0, 1, 2: Halted(vd([k])), the full chain to claim_pub.
COLL = [
    ("25a506d195d2043784a3b10ac8a62ed37a262b9e5f0db732e17582baa37729e1",
     "20da0efc8515aa3e25dd57e2fc0cbe1ff52cbdc4490cd83011c4861593946efb",
     "57a4342f55c3327165c38fd599aa52f2911992f8695cf83cf6e0bbd1fb01e5b7",
     "09cca21dd1663ea8ae00ee29adf93f834b47d71c987296ee9779cb1b387844f9"),
    ("6d6da1211b48508a677d65333b26dda24dbb0122c8d59504a586aee86c436524",
     "57270cdd067bff97418d4b6edbb71b9c09543ab4e694163bb0c22e3b3db77893",
     "1e7c4bebd59d2270d94ae4320eee5ff6e8fc83c829aee8d886c6e175dd9e9067",
     "31ad9bce5eff42c7379a8c9279d7bd19a3ddf9f16c250e3cfb9e25471b8a16c9"),
    ("6aec3d5486430aeca4612257559878fb19c2c573adc72d0c86a75c6ff3a2f6ad",
     "0264ec12e35dcb37bc6ac8df82d997b5a3e2d4f9353d575c85eac464e30ae16c",
     "3011f694853e3680cab2618dd2a518789379b847d7a3b55dc6448386b7dfd077",
     "726d5a38d1c6274e93d845c7c1fcf2f0386c86a8e19c3fcbf48ae7dccebd39af"),
]


def coll_bundle(k):
    return {"program": {"prog_d": A.p2(5, [k, 1]), "r_p": [50 + k] * 8},
            "input": {"salt": [1000 * (k + 1) + i for i in range(8)], "val": {"val_d": A.val_none()},
                      "fuel": 1 << 64, "stack_limit": 1 << 16},
            "outcome": {"halted": [str(k)]}}


class HaltedChain(unittest.TestCase):
    def test_collection_runs(self):
        for k, (cp, ip, op, cl) in enumerate(COLL):
            res = A.build(coll_bundle(k))
            self.assertEqual([res["c_p"], res["input_pub"], res["outcome_pub"], res["claim_pub"]],
                             [h(cp), h(ip), h(op), h(cl)], k)

    def test_negative_changed_outcome_word(self):
        """Negative: one changed outcome word gives another outcome_pub (and claim_pub); c_p and
        input_pub do not move."""
        good = A.build(coll_bundle(1))
        b = coll_bundle(1)
        b["outcome"]["halted"] = ["2"]
        bad = A.build(b)
        self.assertNotEqual(bad["outcome_pub"], good["outcome_pub"])
        self.assertNotEqual(bad["claim_pub"], good["claim_pub"])
        self.assertEqual((bad["c_p"], bad["input_pub"]), (good["c_p"], good["input_pub"]))
        with self.assertRaises(A.Mismatch):
            b["public"] = {"outcome_pub": str(good["outcome_pub"])}
            A.build(b)


class T17Receipt(unittest.TestCase):
    """The accepted T17 (a) acceptRun (Preview block 115,187) reproduced byte for byte."""

    @classmethod
    def setUpClass(cls):
        cls.args = json.loads((FX / "t17-args.json").read_text())
        cls.rec = json.loads((FX / "t17-receipt.json").read_text())
        cls.res = A.build(A.bundle_from_t10_args(cls.args))

    def test_words(self):
        for k in ("c_p", "input_pub", "outcome_pub", "claim_pub", "anchor"):
            self.assertEqual(self.res[k], int(self.args[k]), k)
        self.assertEqual(self.res["claim_pub"], int(self.rec["acceptRun"]["claimPub"]))
        self.assertEqual(self.res["outcome_pub"], int(self.rec["acceptRun"]["runsLookup"]))
        self.assertEqual(self.res["anchor"], int(self.rec["constructorArgs"]["anchor"]))

    def test_progc_operand_from_prog_d(self):
        f = {e["label"]: e["value"] for e in self.args["forbid"]}
        self.assertEqual(self.res["operands"]["progc_d2"], int(f["R0.d2"]))
        self.assertEqual(self.res["operands"]["progc_d2"], int(self.args["anchor_d2"]))
        self.assertEqual(self.res["depth"]["c_p"], "prog_d+r_p")

    def test_ledger_args_bytes(self):
        b = A.ledger_args_bytes(self.res)
        self.assertEqual(b, (FX / "t17-ledger-args.json").read_bytes())
        self.assertEqual(hashlib.sha256(b).hexdigest(), self.rec["inputs"]["ledger-args.json"])

    def test_call_vector_lanes(self):
        cv = json.loads((FX / "t17-valid.call-vector.json").read_text())["words"]
        A.check_public(self.res, {"call_vector": cv})

    def test_r1_vector_refused(self):
        """R1 (outcome_pub + 1, refused by the node): the lanes do not match A's arguments, and its
        claim lanes are transientHash over the changed outcome_pub."""
        cv = json.loads((FX / "t17-r1.call-vector.json").read_text())["words"]
        with self.assertRaises(A.Mismatch):
            A.check_public(self.res, {"call_vector": cv})
        r = self.res
        self.assertEqual(int(cv[20]), A.transient_hash([r["c_p"], r["input_pub"], r["outcome_pub"] + 1]))


class MycroftCall(unittest.TestCase):
    def setUp(self):
        self.call = json.loads((FX / "t02-increment.call.json").read_text())
        self.kw = dict(prog_d=A.p2(5, [7, 7]), r_p=[9] * 8, salt=list(range(11, 19)))

    def test_outcome_is_comm_then_eff(self):
        b = A.bundle_from_mycroft_call(self.call, **self.kw)
        self.assertEqual(b["outcome"]["halted"], [self.call["comm"]] + self.call["eff"])
        res = A.build(b)
        ind = A.input_d(self.kw["salt"], A.vd_fr_list([int(x) for x in self.call["tape"]]), 0, 0)
        od = A.outcome_halted([int(self.call["comm"])] + [int(x) for x in self.call["eff"]])
        self.assertEqual(res["outcome_pub"], A.digest("SHK_OUTCOME", A.pack(A.p2(A.DOM_OUTHIDE, ind + od))))

    def test_negative_changed_effect_word(self):
        good = A.build(A.bundle_from_mycroft_call(self.call, **self.kw))
        c = copy.deepcopy(self.call)
        c["eff"][-1] = str(int(c["eff"][-1]) + 1)
        bad = A.build(A.bundle_from_mycroft_call(c, **self.kw))
        self.assertNotEqual(bad["outcome_pub"], good["outcome_pub"])
        self.assertNotEqual(bad["claim_pub"], good["claim_pub"])
        self.assertEqual(bad["c_p"], good["c_p"])

    def test_prove_result_crosscheck(self):
        res = A.build(A.bundle_from_mycroft_call(self.call, **self.kw))
        with self.assertRaises(A.Mismatch):  # the t23.py stub prover's placeholder words
            A.check_prove_result(res, {"c_p": "1", "input_pub": "2", "outcome_pub": "3"})


if __name__ == "__main__":
    unittest.main(verbosity=2)
