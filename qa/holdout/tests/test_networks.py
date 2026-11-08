"""Unit tests for the network-parametrised holdout tooling (E12). No network, wallet or cargo.

Run: python3 -m unittest discover -s qa/holdout/tests -t qa/holdout/tests
"""
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HOLDOUT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(HOLDOUT / "lib"))

import engine  # noqa: E402
import network_check  # noqa: E402
import networks  # noqa: E402
import plans  # noqa: E402
import rejections  # noqa: E402

U128_MAX = (1 << 128) - 1
DOMAIN = (0, 1, 256, 1 << 64, U128_MAX)


def sha(b):
    return hashlib.sha256(b).hexdigest()


def trimmed(v, prefix="u8"):
    b = v.to_bytes(16, "big").rstrip(b"\0")
    n = len(b) if prefix == "u8" else len(b) << 2
    return bytes([n]) + b


class Selection(unittest.TestCase):
    def test_selectable_networks(self):
        for n in ("undeployed", "preview", "mainnet"):
            name, cfg = networks.select(n)
            self.assertEqual(name, n)
            self.assertEqual(cfg["networkId"], n)
            self.assertEqual(cfg["ledgerVersion"], "9.1.0.1")

    def test_stagenet_is_retired(self):
        with self.assertRaises(networks.Retired) as e:
            networks.select("stagenet")
        self.assertIn("RETIRED", str(e.exception))

    def test_unknown(self):
        with self.assertRaises(networks.Unknown):
            networks.select("preprod")

    def test_env_overrides_default(self):
        with mock.patch.dict(os.environ, {networks.ENV: "mainnet"}):
            self.assertEqual(networks.from_env("preview")[0], "mainnet")
        with mock.patch.dict(os.environ, {}, clear=True):
            self.assertEqual(networks.from_env("preview")[0], "preview")
        with mock.patch.dict(os.environ, {networks.ENV: "stagenet"}):
            with self.assertRaises(networks.Retired):
                networks.from_env("preview")

    def test_qualifying(self):
        reg = networks.registry()["networks"]
        self.assertFalse(reg["undeployed"]["qualifying"])
        self.assertTrue(reg["preview"]["qualifying"] and reg["mainnet"]["qualifying"])

    def test_mainnet_indexer_needs_token(self):
        ix = networks.Indexer(*networks.select("mainnet"))
        with mock.patch.dict(os.environ, {}, clear=True):
            with self.assertRaises(networks.Unreachable):
                ix.url()
        with mock.patch.dict(os.environ, {"DW_BLOCKFROST_MAINNET_PROJECT_ID": "tok"}):
            self.assertTrue(ix.url().endswith("/api/v0?project_id=tok"))

    def test_preview_and_undeployed_endpoints(self):
        self.assertEqual(networks.Indexer(*networks.select("preview")).url(),
                         "https://indexer.preview.midnight.network/api/v4/graphql")
        with mock.patch.dict(os.environ, {"DW_UNDEPLOYED_INDEXER": "http://h:1/g"}):
            self.assertEqual(networks.Indexer(*networks.select("undeployed")).url(), "http://h:1/g")

    def test_genesis(self):
        _, pv = networks.select("preview")
        _, ud = networks.select("undeployed")
        self.assertIsNone(networks.check_genesis(pv, pv["genesisHash"]))
        self.assertIsNotNone(networks.check_genesis(pv, "0x" + "0" * 64))
        self.assertIsNone(networks.check_genesis(ud, "0x" + "ab" * 32))
        self.assertIsNotNone(networks.check_genesis(ud, None))


class RunSh(unittest.TestCase):
    def run_sh(self, *args):
        env = {k: v for k, v in os.environ.items() if not k.startswith("DW_")}
        return subprocess.run([str(HOLDOUT / "run.sh"), *args], capture_output=True, text=True, env=env, timeout=60)

    def test_stagenet_suite_refused(self):
        r = self.run_sh("rejections-stagenet")
        self.assertEqual(r.returncode, 1)
        self.assertIn("RETIRED", r.stdout)

    def test_preview_suite_unimplemented_without_sealed_address(self):
        cfg = networks.select("preview")[1]
        if cfg["sherlockRun"]["address"]:
            self.skipTest("preview address sealed; the suite would read the indexer")
        r = self.run_sh("rejections-preview")
        self.assertEqual(r.returncode, 2, r.stdout + r.stderr)
        self.assertIn("summary preview: pass=0 fail=0 unimplemented=5", r.stdout)

    def test_network_flag_only_for_u023(self):
        self.assertEqual(self.run_sh("uireq-001", "--network", "preview").returncode, 1)
        self.assertEqual(self.run_sh("rejections-preview", "--network", "preview").returncode, 1)

    def test_list(self):
        out = self.run_sh("list").stdout
        self.assertIn("rejections-preview\tn-c-other-program", out)
        self.assertIn("rejections-mainnet\tn-a-flipped-proof", out)
        self.assertNotIn("rejections-mainnet\tn-c-other-program", out)
        self.assertNotIn("rejections-stagenet", out)
        self.assertIn("retired-stagenet\ts-a-tampered-inner-proof\tretired", out)


class CaseSelection(unittest.TestCase):
    def test_per_network(self):
        cases = rejections.load_cases()
        ids = lambda s: sorted(c["id"] for c in rejections.select_cases(s, cases))
        five = ["n-a-flipped-proof", "n-b-reused-proof", "n-c-other-program", "n-d-other-init", "n-e-replayed-stmt"]
        self.assertEqual(ids("preview"), five)
        self.assertEqual(ids("undeployed"), five)
        self.assertEqual(ids("mainnet"), ["n-a-flipped-proof", "n-b-reused-proof", "n-e-replayed-stmt"])
        self.assertEqual(ids("stagenet"), [])
        self.assertTrue(all(c["id"].startswith(("a", "b", "c", "d", "e")) for c in rejections.select_cases("local", cases)))
        self.assertTrue(all(c.get("retired") for c in cases if c["scope"] == "stagenet"))


class Decoder(unittest.TestCase):
    def lay(self, prefix="u8", offset=3):
        return {"offset": offset, "encoding": "u128-be-trimmed", "length_prefix": prefix, "initial": "1"}

    def test_whole_domain(self):
        for prefix in ("u8", "scale-compact"):
            for v in DOMAIN:
                state = b"abc" + trimmed(v, prefix) + b"\0\xff tail"
                self.assertEqual(plans._decode_field(state, self.lay(prefix), "t"), v, (prefix, v))

    def test_refusals(self):
        bad = [
            (b"abc" + bytes([17]) + b"\1" * 17, self.lay()),                 # longer than 16
            (b"abc" + bytes([2, 1, 0]), self.lay()),                         # not trimmed
            (b"abc" + bytes([4]) + b"\1", self.lay()),                       # truncated
            (b"abc" + bytes([5, 1]), self.lay("scale-compact")),             # not a single-byte compact
            (b"abc" + bytes(16), {"offset": 3, "length": 16, "encoding": "u128-be"}),  # layout/1 fixed window
            (b"abc", self.lay()),                                            # prefix outside the state
        ]
        for state, lay in bad:
            with self.assertRaises(engine.Fail, msg=repr((state, lay))):
                plans._decode_field(state, lay, "t")


class FakeIndexer:
    sha = staticmethod(sha)

    def __init__(self, txs, actions, latest=None):
        self._txs, self._actions, self._latest = txs, actions, latest or {}

    def txs(self, h):
        return self._txs.get(h, [])

    def action_at(self, address, h):
        return self._actions.get((address, h))

    def latest(self, address):
        return self._latest.get(address)


def git(repo, *a):
    subprocess.run(["git", "-C", str(repo), *a], check=True, capture_output=True)


class NetworkCheck(unittest.TestCase):
    """network_check.run against a temporary committed receipt and an in-memory indexer."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.repo = Path(self.tmp.name)
        git(self.repo, "init", "-q")
        git(self.repo, "config", "user.email", "t@t")
        git(self.repo, "config", "user.name", "t")
        self.name, cfg = networks.select("preview")
        self.cfg = json.loads(json.dumps(cfg))
        main, op, oi = "aa" * 32, "bb" * 32, "cc" * 32
        self.cfg["sherlockRun"].update(address=main, otherProgramAddress=op, otherInitAddress=oi)
        twin = bytes(range(64)) * 2
        flip = {"txOffset": 40, "mask": 4, "proofStart": 32, "proofLength": 32}
        mutated = bytearray(twin); mutated[40] ^= 4
        pre_state, post_state = b"pre-state", b"post-state"
        self.twin_h = sha(twin)
        names = {"flipped-proof": sha(bytes(mutated)), "reused-proof": "11" * 32, "other-program": "22" * 32,
                 "other-init": "33" * 32}
        att = [{"name": n, "txSHA256": h, "nodeOutcome": "refused",
                "refusal": [{"tag": "Invalid", "message": "1010: Invalid Transaction: failed to verify proof"}]}
               for n, h in names.items()]
        att.append({"name": "twin", "txSHA256": self.twin_h, "nodeOutcome": "accepted", "refusal": []})
        att.append({"name": "replay", "txSHA256": "44" * 32, "nodeOutcome": "refused",
                    "refusal": [{"tag": "Invalid", "message": "1010: Invalid Transaction: transcript rejected"}]})
        stage = {"flipped-proof": "proof/contract check", "reused-proof": "proof/contract check"}
        self.rc = {"schema": network_check.SCHEMA, "network": "preview", "genesisHash": self.cfg["genesisHash"],
                   "submitted": True, "finishedAt": "2026-11-09T00:00:00Z", "address": main,
                   "otherProgramAddress": op, "otherInitAddress": oi, "preActionTx": "55" * 32,
                   "preStateSHA256": sha(pre_state), "stateAfterMutantsSHA256": sha(pre_state), "attempts": att,
                   "prediction": {a["name"]: {"applied": False, "stage": stage.get(a["name"], "transcript application")}
                                  for a in att if a["name"] != "twin"},
                   "flip": flip, "twinProofSHA256": sha(twin[32:64])}
        self.ix = FakeIndexer(
            {self.twin_h: [{"raw": twin.hex(), "transactionResult": {"status": "SUCCESS"}}]},
            {(main, self.twin_h): {"__typename": "ContractCall", "entryPoint": "acceptRun", "state": post_state.hex()},
             (main, "55" * 32): {"__typename": "ContractDeploy", "state": pre_state.hex()}},
            {op: {"__typename": "ContractDeploy"}, oi: {"__typename": "ContractDeploy"}})
        self.cases = [c for c in rejections.load_cases() if c["scope"] == "network"]

    def tearDown(self):
        self.tmp.cleanup()

    def commit(self, rc, trace=True):
        rel = network_check.receipt_rel(self.cfg)
        d = self.repo / Path(rel).parent
        d.mkdir(parents=True, exist_ok=True)
        (self.repo / rel).write_text(json.dumps(rc))
        lines = [f"[reject] output {d.resolve()}"] + [
            f"[reject] {a['name']}: node {'ACCEPTED' if a['nodeOutcome'] == 'accepted' else 'refused'} {a['txSHA256']}"
            for a in rc["attempts"]]
        tr = self.repo / "qa/sherlock/traces/r1.log"
        tr.parent.mkdir(parents=True, exist_ok=True)
        tr.write_text("\n".join(lines) + "\n" if trace else "")
        (self.repo / "qa/sherlock/runs.jsonl").write_text(json.dumps(
            {"id": "OP-PV-REJECT", "run": "r1", "exit": 0, "trace": "qa/sherlock/traces/r1.log"}) + "\n")
        git(self.repo, "add", rel)
        git(self.repo, "commit", "-q", "-m", "receipt")

    def test_all_five_pass(self):
        self.commit(self.rc)
        p, f, u = network_check.run(self.name, self.cfg, self.cases, self.repo, self.ix)
        self.assertEqual((len(p), f, u), (5, [], []), (p, f, u))

    def test_unsealed_address_is_unimplemented(self):
        cfg = networks.select("preview")[1]
        p, f, u = network_check.run("preview", cfg, self.cases, self.repo, self.ix)
        self.assertEqual((p, f, len(u)), ([], [], 5))
        self.assertIn("has not sealed", u[0][1])

    def test_uncommitted_receipt_is_unimplemented(self):
        p, f, u = network_check.run(self.name, self.cfg, self.cases, self.repo, self.ix)
        self.assertEqual(len(u), 5)
        self.assertIn("committed OP-PV-REJECT receipt", u[0][1])

    def test_included_mutant_fails(self):
        self.ix._txs["11" * 32] = [{"raw": "00"}]
        self.commit(self.rc)
        p, f, u = network_check.run(self.name, self.cfg, self.cases, self.repo, self.ix)
        self.assertEqual([c for c, _ in f], ["n-b-reused-proof"])

    def test_wrong_network_receipt_fails_all(self):
        rc = dict(self.rc, network="undeployed")
        self.commit(rc)
        p, f, u = network_check.run(self.name, self.cfg, self.cases, self.repo, self.ix)
        self.assertEqual(len(f), 5)
        self.assertIn("positive control failed", f[0][1])

    def test_fee_refusal_not_counted(self):
        rc = json.loads(json.dumps(self.rc))
        rc["attempts"][0]["refusal"] = [{"tag": "Invalid", "message": "insufficient DUST for fee"}]
        self.commit(rc)
        p, f, u = network_check.run(self.name, self.cfg, self.cases, self.repo, self.ix)
        self.assertEqual([c for c, _ in f], ["n-a-flipped-proof"])

    def test_unmeasured_run_fails(self):
        self.commit(self.rc, trace=False)
        p, f, u = network_check.run(self.name, self.cfg, self.cases, self.repo, self.ix)
        self.assertEqual(len(f), 5)


class U023Network(unittest.TestCase):
    def test_default_is_preview_and_stagenet_refused(self):
        con = json.loads((HOLDOUT / "contracts" / "U023.json").read_text())
        self.assertEqual(con["default_network"], "preview")
        self.assertEqual(len(con["qualification_records"]["preview"]), 3)
        self.assertNotIn("stagenet", con["qualification_records"])
        with mock.patch.dict(os.environ, {}, clear=True):
            self.assertEqual(plans._network(con)[0], "preview")
        with mock.patch.dict(os.environ, {networks.ENV: "stagenet"}):
            with self.assertRaises(engine.Fail):
                plans._network(con)

    def record(self, root, result=256, **pin):
        """A /2 record over an in-memory indexer; result 256 has a zero low byte (trimmed to 15 bytes)."""
        src = 'language "moriarty-sherlock-draft/1"; program P { entry 256; }'
        vk = b"midnight:verifier-key[v1]:" + b"key-body"
        layout = json.dumps({"format": plans.LAYOUT_FORMAT, "result": {
            "offset": 0, "encoding": "u128-be-trimmed", "length_prefix": "u8", "initial": "1"}}).encode()
        files = {"p.mori": src.encode(), "vk.bin": vk, "layout.json": layout}
        for k, v in files.items():
            (root / k).write_bytes(v)
        state = lambda v: trimmed(v) + b"acceptRun" + b"\0\0" + b"key-body"
        pre, post = state(1), state(result)
        dtx, ctx = sha(b"deploy"), sha(b"call")
        _, cfg = networks.select("preview")
        pins = {"network": "preview", "genesis_hash": cfg["genesisHash"], "ledger_version": "9.1.0.1",
                "node_version": "2.1.101", "toolchain": "compact 0.36.0", "relation_vk_path": "vk.bin",
                "relation_id": sha(vk), "state_layout_path": "layout.json", "state_layout_sha256": sha(layout), **pin}
        stages = {k: {"status": "run"} for k in plans.QUAL_STAGES}
        stages["prove"]["prover"], stages["verify"]["verifier"] = "prover", "verifier"
        stages["deploy"].update(tx=dtx, address="ad")
        stages["call"].update(tx=ctx, entry_point="acceptRun")
        stages["readback"].update(state_sha256=sha(post), effects=[])
        q = {"format": plans.QUAL_FORMAT, "source": {"path": "p.mori", "sha256": sha(src.encode())}, "pins": pins,
             "stages": stages, "claims": {"execution_authority": "none", "preview_fenrir_evidence_transferred": False,
                                          "stagenet_evidence_transferred": False}}
        files["q.json"] = json.dumps(q).encode()
        ops = mock.Mock()
        ops.tracked_clean = lambda p: p in files
        ops.read = lambda p: files[p].decode()
        ix = FakeIndexer({dtx: [{"raw": b"deploy".hex(), "transactionResult": {"status": "SUCCESS"}}],
                          ctx: [{"raw": b"call".hex(), "transactionResult": {"status": "SUCCESS"}}]},
                         {("ad", dtx): {"__typename": "ContractDeploy", "state": pre.hex()},
                          ("ad", ctx): {"__typename": "ContractCall", "entryPoint": "acceptRun", "state": post.hex()}})
        return ops, cfg, ix

    def qualify(self, **kw):
        with tempfile.TemporaryDirectory() as t, mock.patch.object(engine, "REPO", Path(t)), \
                mock.patch.object(engine, "run_source", lambda prof, text: 256), \
                mock.patch.object(plans, "_halted_value", lambda r, label: r):
            ops, cfg, ix = self.record(Path(t), **kw)
            return plans._qualify_record({"engine": "draft"}, ops, "q.json", "preview", cfg, ix)

    def test_record_with_trimmed_result_qualifies(self):
        self.assertEqual(self.qualify()[1], sha(b"call"))

    def test_record_refusals(self):
        for kw, msg in (({"network": "stagenet"}, "not the selected network"),
                        ({"ledger_version": "9"}, "ledger_version"),
                        ({"genesis_hash": "0x" + "0" * 64}, "genesis"),
                        ({"result": 257}, "!= reference 256")):
            with self.assertRaises(engine.Fail, msg=repr(kw)) as e:
                self.qualify(**kw)
            self.assertIn(msg, str(e.exception))

    def test_case_requires_three_preview_records(self):
        c = json.loads((HOLDOUT / "cases" / "uireq" / "023" / "case.json").read_text())
        chk = c["scenarios"][1]["exec"]["checks"][0]
        self.assertEqual(chk["kind"], "native-qualification")
        self.assertEqual(chk["records_required"], {"undeployed": 1, "preview": 3, "mainnet": 1})


if __name__ == "__main__":
    unittest.main()
