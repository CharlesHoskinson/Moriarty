"""Unit counterexample for the pinned U023 qualification checker.

Execute its exact source against in-memory repository/indexer oracles. The
indexed state contains result 999, while the record claims/reference returns 6.
Also check the repaired version against eleven pinned layout/key fixtures.
No network, wallet, ledger call, or real qualification claim is made.
"""
import hashlib
import json
import subprocess
import sys
import types

COMMITS = ("64822e4c712fd6a0d3bcedf7ea0345009c3502a8",
           "0eb431b92c17ff1e9cadb59db800a1969b58810b")
REPAIRED = "489bbdaaff4358cfd47e386956696207e736a667"


def sha(value):
    return hashlib.sha256(value).hexdigest()


def check(commit, post_value=999, pre_value=0, initial=0, encoding="u128-le",
          observed=6, wrong_key=False, wrong_layout_hash=False, offset=0, effects=None):
    source_text = 'language "moriarty-sherlock-draft/1"; program P { entry 6; }'
    body = b"unit-verifier-key-body"
    vk = b"midnight:verifier-key[v1]:" + body
    installed = b"wrong-verifier-key-body" if wrong_key else body
    endian = "little" if encoding == "u128-le" else "big"
    def state(value):
        return value.to_bytes(16, endian) + b"entry" + b"\xff\xff" + installed
    pre_state, indexed_state = state(pre_value), state(post_value)
    layout = json.dumps({"format": "moriarty-sherlock-state-layout/1",
                         "result": {"offset": offset, "length": 16,
                                    "encoding": encoding, "initial": str(initial)}}).encode()
    deploy_bytes, call_bytes = b"indexed successful deploy", b"indexed successful call"
    deploy_id, call_id = sha(deploy_bytes), sha(call_bytes)
    record = {
        "format": "moriarty-sherlock-native-qualification/1",
        "claims": {"execution_authority": "none", "preview_fenrir_evidence_transferred": False},
        "source": {"path": "program.mori", "sha256": sha(source_text.encode())},
        "pins": {"network": "stagenet", "ledger_version": "9", "node_version": "pinned",
                 "toolchain": "pinned", "relation_vk_path": "vk.bin", "relation_id": sha(vk),
                 "state_layout_path": "layout.json",
                 "state_layout_sha256": "0" * 64 if wrong_layout_hash else sha(layout)},
        "stages": {name: {"status": "run"} for name in
                   ("compile", "prove", "verify", "deploy", "call", "readback")},
    }
    record["stages"]["prove"]["prover"] = "prover"
    record["stages"]["verify"]["verifier"] = "independent-verifier"
    record["stages"]["deploy"].update(tx=deploy_id, address="contract-address")
    record["stages"]["call"].update(tx=call_id, entry_point="entry")
    record["stages"]["readback"].update(state_sha256=sha(indexed_state), effects=[] if effects is None else effects)
    if observed is not None:
        record["stages"]["readback"]["observed_result"] = observed
    files = {"qualification.json": json.dumps(record).encode(), "program.mori": source_text.encode(),
             "vk.bin": vk, "layout.json": layout}

    class Failure(Exception):
        pass

    class KeyPath:
        def __init__(self, path):
            self.path = path

        def read_bytes(self):
            return files[self.path]

    class Root:
        def __truediv__(self, path):
            return KeyPath(path)

    class Repository:
        def __init__(self, root):
            pass

        def tracked_clean(self, path):
            return path in files

        def read(self, path):
            return files[path].decode()

    engine = types.ModuleType("engine")
    engine.Fail = Failure
    engine.HostUnfit = Failure
    engine.REPO = Root()
    engine.run_source = lambda profile, text: 6
    sys.modules["engine"] = engine
    indexer = types.ModuleType("stagenet_check")
    tx_bytes = {deploy_id: deploy_bytes, call_id: call_bytes}
    indexer.txs = lambda tx_id: [{"transactionResult": {"status": "SUCCESS"}, "raw": tx_bytes[tx_id].hex()}]
    indexer.sha = sha
    indexer.gql = lambda query: {"contractAction": {
        "__typename": "ContractDeploy" if deploy_id in query else "ContractCall",
        "entryPoint": "entry", "state": (pre_state if deploy_id in query else indexed_state).hex(),
        "transaction": {"hash": deploy_id if deploy_id in query else call_id},
    }}
    sys.modules["stagenet_check"] = indexer
    raw = subprocess.run(["git", "show", f"{commit}:qa/holdout/lib/plans.py"],
                         check=True, capture_output=True, text=True).stdout
    subject = {"__name__": "pinned_qualification_checker"}
    exec(compile(raw, f"{commit}:plans.py", "exec"), subject)
    subject["U"] = types.SimpleNamespace(RepoOps=Repository)
    subject["_contract"] = lambda cid: {"qualification_record": "qualification.json"}
    # Independent reference oracle: the source literal evaluates to 6.
    subject["_halted_value"] = lambda result, label: result
    try:
        subject["custom_native_qualification"]({"engine": "draft"})
        return {"checker_accepted": True}
    except Failure as error:
        return {"checker_accepted": False, "reason": str(error)}


def main():
    accepted = []
    for commit in COMMITS:
        result = check(commit)
        assert result["checker_accepted"], result
        accepted.append({"checker_commit": commit, **result})
    cases = [
        ("correct-little-endian", {}, True, None),
        ("correct-big-endian", {"encoding": "u128-be"}, True, None),
        ("indexed-result-999", {"post_value": 999}, False, "indexed post-call result 999 != reference 6"),
        ("indexed-result-999-no-recorded-result", {"post_value": 999, "observed": None}, False, "indexed post-call result 999"),
        ("lying-recorded-result", {"observed": 999}, False, "observed_result disagrees"),
        ("wrong-installed-key", {"wrong_key": True}, False, "pinned verifying key is not the key installed"),
        ("wrong-initial-field", {"pre_value": 1}, False, "deploy-state result 1 != layout initial"),
        ("unchanged-field", {"pre_value": 6, "initial": 6}, False, "result field did not change"),
        ("layout-hash-mismatch", {"wrong_layout_hash": True}, False, "state_layout_sha256 differs"),
        ("unexpected-effects", {"effects": ["unexpected"]}, False, "pure qualification program reports effects"),
        ("layout-out-of-range", {"offset": 999}, False, "window lies outside the state"),
    ]
    repaired_cases = []
    for name, arguments, want_accepted, reason in cases:
        arguments = {"post_value": 6, **arguments}
        result = check(REPAIRED, **arguments)
        assert result["checker_accepted"] == want_accepted, (name, result)
        if reason:
            assert reason in result.get("reason", ""), (name, result)
        repaired_cases.append({"case": name, **result})
    observations = {
        "checks": accepted,
        "oracle_kind": "in-memory repository and indexer unit fixture",
        "reference_result": 6,
        "recorded_observed_result": 6,
        "indexed_state_result": 999,
        "state_encoding": "pinned u128-le field plus entry point and verifier-key body",
        "repaired_checker_commit": REPAIRED,
        "repair_cases": repaired_cases,
        "real_stagenet_execution": False,
    }
    print(json.dumps(observations, indent=2))


if __name__ == "__main__":
    main()
