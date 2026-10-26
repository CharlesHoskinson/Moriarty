#!/usr/bin/env python3
"""Assemble the UIREQ-023 scenario-2 qualification record from a SherlockRun stagenet run.

Read-only against the public stagenet indexer and node RPC; no wallet. Every claim the record
makes is re-derived here first: deploy and call transactions indexed once with SUCCESS, the
deploy action is the ContractDeploy of the address and the call the ContractCall on acceptRun,
the pinned verifier body occurs once in both states within 64 bytes after the entry name, the
result window decodes to the layout's initial value before and to the reference after, and the
post-state hash. Writes (under OUT_DIR):
  u023/acceptRun.verifier   the installed run-decider key (copied from the bundle)
  u023/state-layout.json    moriarty-sherlock-state-layout/1
  u023-stagenet-e2e.json    moriarty-sherlock-native-qualification/1

Usage: assemble-u023-record.py --source SRC.mori --bundle GP_BUNDLE --deploy DEPLOY_RECEIPT
         --call CALL_RECEIPT --readback READBACK_DIR --reference N --out-dir DIR [--check-only]
"""
import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
import urllib.request
from pathlib import Path

INDEXER = "https://indexer.stagenet.shielded.tools/api/v4/graphql"
RPC = "https://rpc.stagenet.shielded.tools"
VK_TAG = re.compile(rb"^midnight:verifier-key\[v\d+\]:")
ENTRY = "acceptRun"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def post(url, payload):
    req = urllib.request.Request(url, json.dumps(payload).encode(), {"content-type": "application/json"})
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.load(r)


def gql(q):
    body = post(INDEXER, {"query": q})
    if body.get("errors"):
        raise SystemExit("indexer error: %r" % body["errors"])
    return body["data"]


def need(cond, msg):
    if not cond:
        raise SystemExit("REFUSED: " + msg)


def tx_once(h, label):
    t = gql('{transactions(offset:{hash:"%s"}){hash raw block{height} ... on RegularTransaction{transactionResult{status}}}}' % h)["transactions"]
    need(len(t) == 1 and (t[0].get("transactionResult") or {}).get("status") == "SUCCESS", f"{label} tx not indexed once with SUCCESS")
    need(sha(bytes.fromhex(t[0]["raw"])) == h, f"{label} indexed bytes do not hash to the tx hash")
    return t[0]["block"]["height"]


def action_at(address, h):
    return gql('{contractAction(address:"%s",offset:{transactionOffset:{hash:"%s"}}){__typename state '
               'transaction{hash} ... on ContractCall{entryPoint}}}' % (address, h))["contractAction"]


def installed(state, vkfile, label):
    m = VK_TAG.match(vkfile)
    need(m is not None, "verifier file has no midnight:verifier-key[vN]: header")
    body, name = vkfile[m.end():], ENTRY.encode()
    j = state.find(body)
    starts = [k for k in range(len(state)) if state.startswith(name, k)]
    need(j >= 0 and state.count(body) == 1 and any(0 <= j - (k + len(name)) <= 64 for k in starts),
         f"pinned key is not the key installed for {ENTRY} in the {label}")


def window(state, lay):
    f = lay["result"]
    w = state[f["offset"]: f["offset"] + 16]
    need(len(w) == 16, "result window outside the state")
    return int.from_bytes(w, "big" if f["encoding"] == "u128-be" else "little")


def main():
    ap = argparse.ArgumentParser()
    for a in ("source", "bundle", "deploy", "call", "readback", "reference", "out-dir"):
        ap.add_argument("--" + a, required=True)
    ap.add_argument("--repo", default=str(Path(__file__).resolve().parents[2]))
    ap.add_argument("--check-only", action="store_true")
    a = ap.parse_args()
    repo = Path(a.repo)
    src = Path(a.source).resolve()
    bundle = Path(a.bundle)
    deploy = json.loads(Path(a.deploy).read_text())
    call = json.loads(Path(a.call).read_text())
    rb = json.loads((Path(a.readback) / "readback.json").read_text())
    lay_raw = (Path(a.readback) / "state-layout.json").read_bytes()
    lay = json.loads(lay_raw)
    bj = json.loads((bundle / "bundle.json").read_text())
    vk = (bundle / "key" / "acceptRun.verifier").read_bytes()
    need(lay.get("format") == "moriarty-sherlock-state-layout/1", "layout format")
    need(sha(vk) == bj["acceptRun_verifier_sha256"] == deploy["verifier"]["sha256"], "bundle key != deployed key")
    need(deploy["constructor"] == [bj["prog_d"], bj["init_d"]], "deployed digests != bundle digests")
    need(call["address"] == deploy["address"] and call["stmt"] == bj["stmt"], "call does not settle this bundle's run")
    need(str(bj["result"]) == str(a.reference), f"bundle result {bj['result']} != reference {a.reference}")
    need(not bj.get("standin") or a.check_only, "stand-in bundle: rehearsal only, never a qualification record")
    addr, dtx, ctx = deploy["address"], deploy["txHash"], call["txHash"]
    need(dtx != ctx, "deploy and call are the same tx")
    dh, ch = tx_once(dtx, "deploy"), tx_once(ctx, "call")
    d, c = action_at(addr, dtx), action_at(addr, ctx)
    need(d and d["__typename"] == "ContractDeploy", "deploy tx is not the ContractDeploy of the address")
    need(c and c["__typename"] == "ContractCall" and c["entryPoint"] == ENTRY, "call tx is not acceptRun on the address")
    pre, postst = bytes.fromhex(d["state"]), bytes.fromhex(c["state"])
    installed(pre, vk, "deploy state")
    installed(postst, vk, "post-call state")
    got_pre, got = window(pre, lay), window(postst, lay)
    need(str(got_pre) == str(lay["result"]["initial"]), f"deploy-state result {got_pre} != layout initial")
    need(str(got) == str(a.reference) and got != got_pre, f"post-call result {got} (reference {a.reference})")
    need(sha(postst) == rb["state_sha256"], "readback state_sha256 != indexed post state")
    need(rb.get("effects", []) == [], "effects not empty")
    node_version = post(RPC, {"id": 1, "jsonrpc": "2.0", "method": "system_version", "params": []})["result"]
    print(f"CHECKED deploy {dtx} @{dh}, call {ctx} @{ch}, result {got_pre} -> {got}, post {sha(postst)[:16]}, node {node_version}")
    if a.check_only:
        return
    out = Path(a.out_dir)
    (out / "u023").mkdir(parents=True, exist_ok=True)
    (out / "u023" / "acceptRun.verifier").write_bytes(vk)
    (out / "u023" / "state-layout.json").write_bytes(lay_raw)
    rel = lambda p: str(Path(p).resolve().relative_to(repo.resolve()))
    head = subprocess.run(["git", "-C", str(repo), "rev-parse", "--short", "HEAD"], capture_output=True, text=True).stdout.strip()
    rec = {
        "format": "moriarty-sherlock-native-qualification/1",
        "source": {"path": rel(src), "sha256": sha(src.read_bytes())},
        "pins": {
            "network": "stagenet", "ledger_version": "9", "node_version": node_version,
            "toolchain": f"compact 0.34.0 --feature-zkir-v3 (SherlockRun); mori-recursion sherlock-gpr {head}; "
                         "midnight-zk-stdlib 2.3.5, midnight-circuits 7.2.4, midnight-proofs 0.8.3; proof server 9.0.0-rc.5",
            "relation_id": sha(vk), "relation_vk_path": rel(out / "u023" / "acceptRun.verifier"),
            "state_layout_path": rel(out / "u023" / "state-layout.json"), "state_layout_sha256": sha(lay_raw),
        },
        "stages": {
            "compile": {"status": "run", "detail": "mori simulate reference + C6 leaf witnesses; IVC node "
                        f"{bj['node_vk_sha256'][:16]} k{bj['node_k']}; run decider {bj['decider_vk_sha256'][:16]} k{bj['decider_k']}"},
            "prove": {"status": "run", "prover": "Moriarty mori-recursion sherlock_gp (CPU): leaves -> IVC fold -> run decider "
                      "(gp_call_prove)", "leaves": bj.get("leaves"), "steps": bj["steps"]},
            "verify": {"status": "run", "verifier": "Midnight stagenet node (ledger 9) on submission, plus the native "
                       "proof-verifying ledger 9 validate_call over the indexed transaction"},
            "deploy": {"status": "run", "tx": dtx, "address": addr, "block": dh},
            "call": {"status": "run", "tx": ctx, "entry_point": ENTRY, "block": ch},
            "readback": {"status": "run", "state_sha256": sha(postst), "observed_result": str(got), "effects": []},
        },
        "claims": {"execution_authority": "none", "preview_fenrir_evidence_transferred": False},
    }
    (out / "u023-stagenet-e2e.json").write_text(json.dumps(rec, indent=2) + "\n")
    print("WROTE", out / "u023-stagenet-e2e.json")


if __name__ == "__main__":
    main()
