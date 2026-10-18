#!/usr/bin/env python3
"""Link isolated-final native receipts onto the source/UPLC matrix.

`npm run validate` writes nativeStatus pending-unsupported because it does not
submit to a node. This overlay records the measured public prototype and keeps
all 20 designer families native-unsupported.
"""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path("/home/charl/moriarty-eutxo-validation-2026-10-12")
MATRIX = Path("/home/charl/Moriarty-wt-eutxo-validation-20261012/experiments/moriarty-eutxo/artifacts/matrix.json")
TX = ROOT / "tooling" / "final-runtime" / "native-final-transactions.json"
READBACK = ROOT / "tooling" / "final-runtime" / "native-final-readback.json"
COMPILE = ROOT / "tooling" / "final-runtime" / "native-compile.json"
T2 = ROOT / "tooling" / "final-runtime" / "native-final-t2-ogmios.json"
T3B = ROOT / "tooling" / "final-runtime" / "native-final-wrong-datum-t3b.json"
OUT = ROOT / "NATIVE-MATRIX-OVERLAY.json"


def main() -> None:
    matrix = json.loads(MATRIX.read_text())
    txs = json.loads(TX.read_text())
    readback = json.loads(READBACK.read_text())
    compile_ = json.loads(COMPILE.read_text())
    t2 = json.loads(T2.read_text())
    t3b = json.loads(T3B.read_text())
    compact = [
        {
            k: r.get(k)
            for k in (
                "label",
                "hash",
                "computedHash",
                "status",
                "errorClass",
                "note",
                "nodeClass",
                "semanticWrongState",
                "raceType",
            )
            if k in r
        }
        for r in txs
    ]
    prototype = {
        "predicate": "public-single-head-wallet-transfer/1",
        "profile": "moriarty-bounded-atomic/1",
        "source": "tooling/final-runtime/examples/native-revision-step.mori",
        "horizonFixture": "2000000000000 POSIX ms; source example 2000000000 left unchanged",
        "quantum": "exactly-1-nominal-per-ledger-unit",
        "programHash": compile_.get("programHash"),
        "sourceHash": compile_.get("sourceHash"),
        "boundsHash": compile_.get("boundsHash"),
        "validatorHash": compile_.get("validatorHash"),
        "validatorHashMeaning": "SHA-256 of generated Aiken atomic.ak; not the Cardano payment credential",
        "paymentCredential": "56d0170da0c16b0c8beb3031c3c8e8c1be003516be58295be123e26b",
        "scriptAddress": "addr_test1wptdq9cd5rqkkrytavcrrs7garqmuqp4z6l9s22muy37y6cgzmrzj",
        "aikenTests": 20,
        "aikenCheckStatus": compile_.get("checkStatus"),
        "privateVerification": compile_.get("admission", {}).get("privateVerification"),
        "frozenComplete": compile_.get("admission", {}).get("nativeSubset", {}).get("frozenComplete"),
        "transactions": compact,
        "latestHead": {
            "txHash": readback["successor"]["txHash"],
            "revision": readback["expectedLatest"]["revision"],
            "remaining": readback["expectedLatest"]["remaining"],
            "count": readback["expectedLatest"]["count"],
            "lovelace": readback["successor"]["assets"]["lovelace"],
            "threadQty": readback["expectedLatest"]["threadQty"],
            "payoutEach": readback["expectedLatest"]["payoutEach"],
            "payeeCount": len(readback.get("payouts") or []),
        },
        "t2Ogmios": {
            "http": t2.get("http"),
            "rpcCode": t2.get("body", {}).get("error", {}).get("code"),
            "evalCode": (t2.get("body", {}).get("error", {}).get("data") or [{}])[0].get("error", {}).get("code"),
        },
        "t3b": {
            "status": t3b.get("t3b", {}).get("status"),
            "hash": t3b.get("t3b", {}).get("hash"),
            "computedHash": t3b.get("t3b", {}).get("computedHash"),
            "headUnchanged": t3b.get("headUnchanged"),
            "isValid": t3b.get("t3b", {}).get("isValid"),
            "errorClass": "ValidationTagMismatch",
            "semanticWrongState": t3b.get("t3b", {}).get("semanticWrongState"),
        },
        "raceType": "sequential-conflicting-spend",
        "backendLimits": {
            "unusedTextArgs": "compile and admit; lower to unconstrained ByteArray; no UTF-8 or length enforcement",
            "roundingNoneFloorDiv": "enforced in source Simulation; not admission/Aiken; floor_div emit with rounding none admits and lowers to checked_div",
            "feeEffect": "exact-quantity wallet output in admitted effect set; not Cardano txfee; net delta is not gross whole-input",
            "genesis": "trusted external consumed-seed; general untrusted genesis not proved",
        },
        "qualification": readback.get("qualification"),
        "receipts": {
            "transactions": str(TX),
            "readback": str(READBACK),
            "compile": str(COMPILE),
        },
    }
    matrix["nativeStatus"] = "measured-public-single-head-wallet-transfer-prototype; 20 designer families native-unsupported"
    matrix["nativeDriver"] = "isolated-final-yaci-haskell-11.0.1-pv11"
    matrix["nativePrototype"] = prototype
    matrix["production"] = False
    matrix["privateVerification"] = "explicitly-not-performed"
    for row in matrix.get("rows") or []:
        native = row.setdefault("native", {})
        native["tier"] = "native"
        native["status"] = "unsupported-measured-gap"
        native["driver"] = "isolated-final-yaci-haskell-11.0.1-pv11"
        native["note"] = (
            "Native evidence is the public Core counter/Transfer prototype only. "
            "This designer family was not submitted as a native script. "
            "See matrix.nativePrototype."
        )
        private = row.setdefault("private", {})
        private["tier"] = "private"
        private["status"] = "unsupported"
        private["note"] = "public experiment does not verify mandatory private/PCD claims"
    MATRIX.write_text(json.dumps(matrix, indent=2) + "\n")
    OUT.write_text(json.dumps({"ok": True, "rows": len(matrix.get("rows") or []), "nativeStatus": matrix["nativeStatus"], "prototypeTx": compact}, indent=2) + "\n")
    print(json.dumps({"ok": True, "path": str(MATRIX), "nativeStatus": matrix["nativeStatus"], "rows": len(matrix.get("rows") or [])}))


if __name__ == "__main__":
    main()
