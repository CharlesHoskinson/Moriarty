"""Unit counterexample for the pinned U023 qualification checker.

Execute its exact source against in-memory repository/indexer oracles. The
indexed state contains result 999, while the record claims/reference returns 6.
No network, wallet, ledger call, or real qualification claim is made.
"""
import hashlib
import json
import subprocess
import sys
import types

COMMITS = ("64822e4c712fd6a0d3bcedf7ea0345009c3502a8",
           "0eb431b92c17ff1e9cadb59db800a1969b58810b")


def sha(value):
    return hashlib.sha256(value).hexdigest()


def main():
    source_text = 'language "moriarty-sherlock-draft/1"; program P { entry 6; }'
    vk = b"committed key oracle"
    indexed_state = json.dumps({"result": 999}).encode()
    deploy_bytes, call_bytes = b"indexed successful deploy", b"indexed successful call"
    deploy_id, call_id = sha(deploy_bytes), sha(call_bytes)
    record = {
        "format": "moriarty-sherlock-native-qualification/1",
        "claims": {"execution_authority": "none", "preview_fenrir_evidence_transferred": False},
        "source": {"path": "program.mori", "sha256": sha(source_text.encode())},
        "pins": {"network": "stagenet", "ledger_version": "9", "node_version": "pinned",
                 "toolchain": "pinned", "relation_vk_path": "vk.bin", "relation_id": sha(vk)},
        "stages": {name: {"status": "run"} for name in
                   ("compile", "prove", "verify", "deploy", "call", "readback")},
    }
    record["stages"]["prove"]["prover"] = "prover"
    record["stages"]["verify"]["verifier"] = "independent-verifier"
    record["stages"]["deploy"].update(tx=deploy_id, address="contract-address")
    record["stages"]["call"].update(tx=call_id, entry_point="entry")
    record["stages"]["readback"].update(observed_result=6, state_sha256=sha(indexed_state))

    class Failure(Exception):
        pass

    class KeyPath:
        def read_bytes(self):
            return vk

    class Root:
        def __truediv__(self, path):
            assert path == "vk.bin"
            return KeyPath()

    class Repository:
        def __init__(self, root):
            pass

        def tracked_clean(self, path):
            return path in ("qualification.json", "program.mori", "vk.bin")

        def read(self, path):
            return {"qualification.json": json.dumps(record), "program.mori": source_text}[path]

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
        "entryPoint": "entry", "state": indexed_state.hex(),
        "transaction": {"hash": deploy_id if deploy_id in query else call_id},
    }}
    sys.modules["stagenet_check"] = indexer
    accepted = []
    for commit in COMMITS:
        raw = subprocess.run(["git", "show", f"{commit}:qa/holdout/lib/plans.py"],
                             check=True, capture_output=True, text=True).stdout
        subject = {"__name__": "pinned_qualification_checker"}
        exec(compile(raw, f"{commit}:plans.py", "exec"), subject)
        subject["U"] = types.SimpleNamespace(RepoOps=Repository)
        subject["_contract"] = lambda cid: {"qualification_record": "qualification.json"}
        # Independent reference oracle: the source literal evaluates to 6.
        subject["_halted_value"] = lambda result, label: result
        subject["custom_native_qualification"]({"engine": "draft"})
        accepted.append({"checker_commit": commit, "checker_accepted": True})
    observations = {
        "checks": accepted,
        "oracle_kind": "in-memory repository and indexer unit fixture",
        "reference_result": 6,
        "recorded_observed_result": 6,
        "indexed_state_result": json.loads(indexed_state)["result"],
        "real_stagenet_execution": False,
    }
    assert observations["indexed_state_result"] != observations["reference_result"]
    print(json.dumps(observations, indent=2))


if __name__ == "__main__":
    main()
