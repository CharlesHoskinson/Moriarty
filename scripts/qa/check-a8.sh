#!/usr/bin/env bash
# A8: read-only indexer query; passes only when a finalized ContractCall exists on the pinned stagenet Sherlock contract
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
python3 -I - <<'PY'
import json, sys, urllib.request
from pathlib import Path
URL = "https://indexer.stagenet.shielded.tools/api/v4/graphql"
ADDR = json.loads(Path("crates/mori-recursion/tests/fixtures/stagenet-verifier.json").read_bytes())["address"]
def q(height):
    off = "" if height is None else ', offset:{blockOffset:{height:%d}}' % height
    query = ('{ contractAction(address:"%s"%s){ __typename '
             '... on ContractCall { entryPoint } '
             'transaction { hash block { height } } } }') % (ADDR, off)
    req = urllib.request.Request(URL, json.dumps({"query": query}).encode(),
                                 {"content-type": "application/json"})
    with urllib.request.urlopen(req, timeout=30) as r:
        body = json.load(r)
    if body.get("errors"):
        raise SystemExit("indexer error: %r" % (body["errors"],))
    return body["data"]["contractAction"]
try:
    height = None
    for _ in range(500):
        a = q(height)
        if a is None:
            print("FAIL: no contract action found for", ADDR); sys.exit(1)
        kind = a["__typename"]; h = a["transaction"]["block"]["height"]
        if kind == "ContractCall":
            print("PASS: finalized ContractCall %r in block %d tx %s" % (a["entryPoint"], h, a["transaction"]["hash"]))
            sys.exit(0)
        if kind == "ContractDeploy":
            print("FAIL: only a ContractDeploy (block %d); zero contract calls observed" % h); sys.exit(1)
        height = h - 1
    print("FAIL: walked 500 actions without a call or deploy"); sys.exit(1)
except OSError as e:
    print("FAIL: indexer unreachable: %s" % e); sys.exit(1)
PY
