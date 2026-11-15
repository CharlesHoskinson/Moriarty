#!/usr/bin/env python3
"""Zero-transaction probe of the T18 (a) Preview deploys (T23 Part A).

For every receipt in the T18 (a) receipts directory: the block hash at each recorded deploy and call height
still equals the recorded block hash (same chain, no relaunch or reorg), and the contract state is still served
by the node (midnight_contractState), and whether it still equals its state at the deploy block. Also records the node identity (genesis, system_version, spec version,
ledger version, tip) and the indexer's HTTP status. Sends no transaction and opens no wallet.

Usage: preview_probe.py RECEIPTS_DIR OUT.json
"""
import hashlib
import json
import pathlib
import sys
import time
import urllib.error
import urllib.request

RPC = "https://rpc.preview.midnight.network"
INDEXER = "https://indexer.preview.midnight.network/api/v4/graphql"
GENESIS = "0x3c096de209e06a1a8c52be7bda109ee8c891a42288c0a9db5495f9616dd13796"


def post(url, body, timeout=40):
    req = urllib.request.Request(url, json.dumps(body).encode(), {"content-type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.status, json.loads(r.read())
    except urllib.error.HTTPError as e:
        return e.code, None
    except Exception as e:  # timeouts, resets
        return f"error: {type(e).__name__}", None


def rpc(method, params=()):
    for k in range(4):
        st, b = post(RPC, {"id": 1, "jsonrpc": "2.0", "method": method, "params": list(params)})
        if st == 200 and b is not None:
            return b.get("result")
        time.sleep(3 * (k + 1))
    raise RuntimeError(f"rpc {method}: {st}")


def main():
    rdir, outp = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    ident = {"at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "genesis": rpc("chain_getBlockHash", [0]),
             "system_version": rpc("system_version"), "specVersion": rpc("state_getRuntimeVersion")["specVersion"],
             "ledgerVersion": rpc("midnight_ledgerVersion"), "tip": int(rpc("chain_getHeader")["number"], 16)}
    assert ident["genesis"] == GENESIS, "not the pinned Preview genesis"
    ist, _ = post(INDEXER, {"query": "{ block { height } }"})
    ident["indexer_status"] = ist
    rows = []
    for f in sorted(rdir.glob("*.json")):
        if f.name.startswith(("balance-", "summary")):
            continue
        r = json.loads(f.read_text())
        dep = r.get("deploy") or {}
        if not dep.get("address"):
            continue
        heights = [(dep["blockHeight"], dep["blockHash"], "deploy")]
        heights += [(c["blockHeight"], c["blockHash"], c["label"]) for c in r.get("calls", []) if c.get("blockHash")]
        bad = []
        for h, want, what in heights:
            got = rpc("chain_getBlockHash", [h])
            if (got or "").removeprefix("0x") != want.removeprefix("0x"):
                bad.append({"what": what, "height": h, "want": want, "got": got})
        state = rpc("midnight_contractState", [dep["address"]])
        at_dep = rpc("midnight_contractState", [dep["address"], "0x" + dep["blockHash"].removeprefix("0x")])
        rows.append({"id": r["id"], "file": f.name, "address": dep["address"], "deployBlock": dep["blockHeight"],
                     "blocksChecked": len(heights), "blockHashMismatches": bad,
                     "statePresent": bool(state), "stateBytes": len(state) // 2 if state else 0,
                     "stateSha256": hashlib.sha256(bytes.fromhex(state)).hexdigest() if state else None,
                     "stateEqualsDeployBlockState": bool(state) and state == at_dep,
                     "includedCalls": sum(1 for c in r.get("calls", []) if c.get("verdict") == "included")})
    out = {"schema": "t23-preview-probe/1", "identity": ident, "previewTxSent": 0, "contracts": rows,
           "summary": {"contracts": len(rows), "statePresent": sum(r["statePresent"] for r in rows),
                       "unchangedSinceDeploy": sum(r["stateEqualsDeployBlockState"] for r in rows),
                       "blocksChecked": sum(r["blocksChecked"] for r in rows),
                       "blockHashMismatches": sum(len(r["blockHashMismatches"]) for r in rows)}}
    outp.write_text(json.dumps(out, indent=1))
    print(json.dumps({"identity": ident, "summary": out["summary"]}))


if __name__ == "__main__":
    main()
