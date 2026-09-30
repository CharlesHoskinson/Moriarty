"""Separate struct-based expected-byte generator; never imports the Node codec.

Synthetic package files only test serialization. The actual package vectors
read the three explicit reviewed module paths without changing them.
This is a second implementation by the same author, not an independent audit.
"""
import hashlib
import json
from pathlib import Path
import struct

BASE = Path(__file__).resolve().parent
SOURCE_PROFILE = "moriarty-financial-agreement-source/6"
PREFIX = b"moriarty-mil4-image/1\x00"


def nominal(sort, value):
    return {"sort": sort, "value": value}


def identifier(value):
    raw = value["value"].encode("ascii")
    return struct.pack(">H", len(raw)) + raw


def text(value):
    raw = value.encode("utf-8", errors="strict")
    return struct.pack(">H", len(raw)) + raw


def number(value, width):
    return int(value).to_bytes(width, "big", signed=False)


def definition(value):
    return (b"\x06" + text(SOURCE_PROFILE) + b"\x01"
            + identifier(value["agreementInstanceId"]) + identifier(value["domain"])
            + identifier(value["asset"]) + number(value["scale"], 1)
            + identifier(value["selectedActionId"])
            + bytes([1 if value["operationKind"] == "Transfer" else 2]))


def package(value):
    raw = (b"\x05" + text("moriarty-core/5") + b"\x06"
           + text("moriarty-intent/3") + b"\x01"
           + identifier(value["coreProgramId"])
           + bytes([1 if value["operationKind"] == "Transfer" else 2])
           + text("parseAndLowerSource6") + text("prepareMil4S0")
           + text("prepareSource6S0Unqualified") + b"\x00\x03")
    for role, file_hex in enumerate(value["filesHex"], start=1):
        content = bytes.fromhex(file_hex)
        raw += bytes([role]) + struct.pack(">I", len(content)) + content
    return raw


def policy(value):
    raw = (b"\x06\x05\x01" + identifier(value["agreementInstanceId"])
           + identifier(value["domain"]) + identifier(value["asset"])
           + number(value["scale"], 1) + identifier(value["actionId"])
           + identifier(value["coreProgramId"])
           + bytes([1 if value["operationKind"] == "Transfer" else 2])
           + bytes.fromhex(value["sourceHash"]) + bytes.fromhex(value["coreHash"])
           + identifier(value["signer"]) + text(value["keyRef"])
           + number(value["validFrom"], 8) + number(value["validUntil"], 8)
           + number(value["grossCap"], 16) + number(value["feeCap"], 16)
           + number(value["netFloor"], 16) + b"\x01"
           + b"\x00\x00" * 5 + b"\x00\x00")
    op = value["operation"]
    if op["kind"] == "Transfer":
        raw += (b"\x01" + identifier(op["owner"]) + identifier(op["recipient"])
                + identifier(op["feeRecipient"]) + number(op["amount"], 16)
                + number(op["fee"], 16))
    else:
        raw += (b"\x02" + identifier(op["obligationId"]) + identifier(op["payer"])
                + number(op["amount"], 16) + b"\x01\x01")
    return raw


def image(purpose, payload):
    envelope = PREFIX + bytes([purpose]) + struct.pack(">I", len(payload)) + payload
    return {"purpose": purpose, "payloadHex": payload.hex(),
            "preimageHex": envelope.hex(), "payloadBytes": len(payload),
            "sha256": hashlib.sha256(envelope).hexdigest()}


def inputs(kind, unicode_key=False):
    selected = "TransferLiteralFee" if kind == "Transfer" else "RepayAccrualFirst"
    source = {"sourceVersion": 6, "profile": SOURCE_PROFILE, "wireProfile": 1,
              "agreementInstanceId": nominal("agreement", "Agreement1"),
              "domain": nominal("domain", "Preview"), "asset": nominal("asset", "A"),
              "scale": "0", "selectedActionId": nominal("action", selected),
              "operationKind": kind}
    core = {"coreVersion": 5, "coreProfile": "moriarty-core/5", "sourceVersion": 6,
            "intentSchema": "moriarty-intent/3", "wireProfile": 1,
            "coreProgramId": nominal("coreProgram", selected), "operationKind": kind,
            "lowerEntry": "parseAndLowerSource6", "prepareEntry": "prepareMil4S0",
            "wrapperEntry": "prepareSource6S0Unqualified", "fileCount": 3,
            "filesHex": ["2f2f2066726f6e740d0a", "00ffefbbbf", ""]}
    terms = {"signer": nominal("signer", "Alice"), "keyRef": "key:e\u0301\U0001f600" if unicode_key else "key1",
             "validFrom": "0", "validUntil": "18446744073709551615",
             "grossCap": "170141183460469231731687303715884105727", "feeCap": "0",
             "netFloor": "1", "failureRelation": "success_only", "supplyChanges": "empty",
             "observations": "empty", "disclosures": "empty", "retainedEffects": "empty",
             "retainedDuties": "empty", "delegation": "none", "recovery": "none"}
    terms["operation"] = ({"kind": "Transfer", "owner": nominal("account", "Alice"),
                           "recipient": nominal("account", "Bob"),
                           "feeRecipient": nominal("account", "Fees"), "amount": "11", "fee": "0"}
                          if kind == "Transfer" else
                          {"kind": "Repay", "obligationId": nominal("obligation", "Loan1"),
                           "payer": nominal("account", "Alice"), "amount": "7",
                           "allocation": "AccrualFirst", "conversion": "identity"})
    return {"definition": source, "package": core, "terms": terms}


def make_vector(name, data):
    source_image = image(1, definition(data["definition"]))
    core_image = image(3, package(data["package"]))
    s = data["definition"]
    policy_input = {"sourceVersion": 6, "coreVersion": 5, "wireProfile": 1,
                    **{k: s[k] for k in ("agreementInstanceId", "domain", "asset", "scale", "operationKind")},
                    "actionId": s["selectedActionId"], "coreProgramId": data["package"]["coreProgramId"],
                    "sourceHash": source_image["sha256"], "coreHash": core_image["sha256"], **data["terms"]}
    return {"name": name, "inputs": data, "policyInput": policy_input,
            "images": {"source": source_image, "core": core_image,
                       "policy": image(4, policy(policy_input))}}


def main():
    vectors = [make_vector("synthetic-transfer", inputs("Transfer")),
               make_vector("synthetic-repayment", inputs("Repay")),
               make_vector("unicode-transfer", inputs("Transfer", True))]
    module_base = (BASE / "../../../src/successor").resolve()
    names = ["financial-agreement-source-v6-frontend.ts", "mil4-s0-core-v5.ts", "mil4-s0-source-v6.ts"]
    files = [(module_base / name).read_bytes() for name in names]
    actual = []
    for kind in ("Transfer", "Repay"):
        data = inputs(kind)
        data["package"]["filesHex"] = [raw.hex() for raw in files]
        actual.append({"operationKind": kind, **image(3, package(data["package"]))})
    # Full raw module bytes already live in the input tree. Freeze only length/digest
    # for actual package vectors, avoiding a second copy of the reviewed modules.
    for value in actual:
        del value["payloadHex"]
        del value["preimageHex"]
    result = {"status": "local-experiment-not-adopted", "vectorMethod": "separate Python struct serializer; same author; no provider audit",
              "specSha256": "52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc",
              "vectors": vectors, "actualPackages": actual,
              "actualModuleInputs": [{"role": i + 1, "name": name, "bytes": len(raw),
                                      "sha256": hashlib.sha256(raw).hexdigest()}
                                     for i, (name, raw) in enumerate(zip(names, files))]}
    (BASE / "vectors.json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
