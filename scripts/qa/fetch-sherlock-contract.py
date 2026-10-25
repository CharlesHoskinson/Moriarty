#!/usr/bin/env python3
"""Read the deployed Sherlock public state; no wallet or transaction operation."""
import hashlib
import json
from pathlib import Path
import sys
import urllib.request

DEPLOYMENT = Path(__file__).resolve().parents[2] / "crates/mori-recursion/tests/fixtures/stagenet-verifier.json"
ADDRESS = json.loads(DEPLOYMENT.read_bytes())["address"]
URL = "https://indexer.stagenet.shielded.tools/api/v4/graphql"


def main():
    output = Path(sys.argv[1])
    query = '{contract(address:"' + ADDRESS + '"){address state} contractAction(address:"' + ADDRESS + '"){__typename transaction{hash block{height}}}}'
    request = urllib.request.Request(URL, json.dumps({"query": query}).encode(), {"content-type": "application/json"})
    with urllib.request.urlopen(request, timeout=30) as response:
        raw = response.read(32 * 1024 * 1024 + 1)
    if len(raw) > 32 * 1024 * 1024:
        raise ValueError("indexer response exceeds 32 MiB")
    body = json.loads(raw)
    if body.get("errors"):
        raise ValueError(body["errors"])
    contract = body["data"]["contract"]
    if contract is None or contract["address"] != ADDRESS:
        raise ValueError("indexer returned a different contract")
    state = bytes.fromhex(contract["state"])
    if len(state) > 16 * 1024 * 1024:
        raise ValueError("contract state exceeds 16 MiB")
    output.write_bytes(state)
    print(json.dumps({"address": ADDRESS, "stateSHA256": hashlib.sha256(state).hexdigest(), "stateBytes": len(state), "action": body["data"]["contractAction"]}))


if __name__ == "__main__":
    main()
