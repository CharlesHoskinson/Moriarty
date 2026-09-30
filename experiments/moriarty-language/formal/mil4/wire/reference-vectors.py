#!/usr/bin/env python3
"""Post-audit independent reproduction of SPEC.md's provisional /3 vectors.

Python standard library only. Construction uses authorization data and the
ordered specification below; expected fixture values are read only afterward
for comparison. This is neither a production codec nor provenance evidence
that the original fixtures preceded the Node implementation.
"""

import argparse
import hashlib
import json
from pathlib import Path
import re
import sys


# SPEC.md's common field table, in tag order. Tags are 1 through 35.
COMMON = (
    ("profile", "literal", ("s0-provisional/1", 1)),
    ("domain", "id", None),
    ("agreementId", "id", None),
    ("stageId", "id", None),
    ("episodeId", "id", None),
    ("actionId", "id", None),
    ("sourceVersion", "literal", (6, 6)),
    ("sourceHash", "hash32", None),
    ("coreVersion", "literal", (5, 5)),
    ("coreProgramId", "id", None),
    ("coreHash", "hash32", None),
    ("policyHash", "hash32", None),
    ("signer", "id", None),
    ("keyScheme", "literal", ("schnorr_bip340", 1)),
    ("signerKey", "hash32", None),
    ("asset", "id", None),
    ("scale", "scale", None),
    ("preHead", "hash32", None),
    ("predecessor", "hash32", None),
    ("nonce", "hash32", None),
    ("validFrom", "u64", None),
    ("validUntil", "u64", None),
    ("grossCap", "nominal", None),
    ("feeCap", "nominal", None),
    ("netFloor", "nominal", None),
    ("effectCommitment", "hash32", None),
    ("failurePolicy", "literal", ("atomic-reject-terminal-success", 1)),
    ("supplyChanges", "empty", None),
    ("observations", "empty", None),
    ("disclosures", "empty", None),
    ("retainedEffects", "empty", None),
    ("retainedDuties", "empty", None),
    ("delegation", "literal", ("none", 0)),
    ("recovery", "literal", ("none", 0)),
    ("operation", "operation", None),
)
OPERATIONS = {
    "transfer": (1, (
        ("owner", "id", None),
        ("recipient", "id", None),
        ("feeRecipient", "id", None),
        ("amount", "nominal", None),
        ("fee", "nominal", None),
    )),
    "repayment": (2, (
        ("obligationId", "id", None),
        ("payer", "id", None),
        ("debtor", "id", None),
        ("creditor", "id", None),
        ("amount", "nominal", None),
        ("allocation", "literal", ("AccrualFirst", 1)),
        ("conversion", "literal", ("identity", 1)),
    )),
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def primitive(value, kind, literal=None):
    if kind == "literal":
        presentation, byte = literal
        require(type(value) is type(presentation) and value == presentation,
                "incorrect literal")
        return bytes([byte])
    if kind == "id":
        require(isinstance(value, str), "noncanonical identifier")
        require(1 <= len(value) <= 64, "identifier exceeds cap")
        require(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._:/-]*", value),
                "noncanonical identifier")
        payload = value.encode("ascii")
        require(1 <= len(payload) <= 64, "identifier exceeds cap")
        return len(payload).to_bytes(2, "big") + payload
    if kind == "hash32":
        require(isinstance(value, str) and
                re.fullmatch(r"[0-9a-f]{64}", value),
                "noncanonical 32-byte hex string")
        return bytes.fromhex(value)
    if kind in ("u64", "nominal"):
        require(isinstance(value, str), "noncanonical decimal string")
        width, cap = (8, 2**64 - 1) if kind == "u64" else (16, 2**127 - 1)
        require(len(value) <= len(str(cap)), "integer exceeds cap")
        require(re.fullmatch(r"0|[1-9][0-9]*", value),
                "noncanonical decimal string")
        number = int(value)
        require(number <= cap, "integer exceeds cap")
        return number.to_bytes(width, "big")
    if kind == "scale":
        require(type(value) is int and 0 <= value <= 38, "invalid scale")
        return bytes([value])
    if kind == "empty":
        require(type(value) is list and not value, "nonempty deferred field")
        return (0).to_bytes(2, "big")
    raise ValueError("unknown primitive type: " + kind)


def reference_vector(authorization):
    """Build bytes and offsets without accepting or consulting expected data."""
    require(type(authorization) is dict and set(authorization) ==
            {"schemaVersion", *(name for name, _, _ in COMMON)},
            "incorrect authorization shape")
    operation = authorization["operation"]
    require(type(operation) is dict and
            isinstance(operation.get("kind"), str) and
            operation["kind"] in OPERATIONS, "incorrect operation shape/kind")
    operation_tag, operation_fields = OPERATIONS[operation["kind"]]
    require(set(operation) == {"kind", *(name for name, _, _ in operation_fields)},
            "incorrect operation keys")
    require(authorization["schemaVersion"] == "moriarty-intent/3",
            "incorrect schema version")

    wire = bytearray(authorization["schemaVersion"].encode("ascii") + b"\x00")
    offsets = {}
    for tag, (name, kind, literal) in enumerate(COMMON, start=1):
        offsets[name] = {"tag": len(wire), "value": len(wire) + 1}
        wire.append(tag)
        if kind == "operation":
            wire.append(operation_tag)
            for leaf, leaf_kind, leaf_literal in operation_fields:
                # fixtures.json indexes variable operation leaves. Fixed
                # repayment allocation/conversion suffixes have no entries.
                if leaf_kind != "literal":
                    offsets["operation." + leaf] = {"value": len(wire)}
                wire.extend(primitive(operation[leaf], leaf_kind, leaf_literal))
            require(int(operation["amount"]) > 0, "zero operation amount")
        else:
            wire.extend(primitive(authorization[name], kind, literal))
        if name == "validUntil":
            require(int(authorization["validUntil"]) >=
                    int(authorization["validFrom"]), "reversed validity interval")
    require(len(wire) <= 4096, "record exceeds cap")
    return {
        "wireHex": wire.hex(),
        "digestHex": hashlib.sha256(wire).hexdigest(),
        "length": len(wire),
        "fieldOffsets": offsets,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fixtures", nargs="?", type=Path,
                        default=Path(__file__).with_name("fixtures.json"))
    parser.add_argument("--json", action="store_true",
                        help="emit all calculated vectors and comparison results")
    args = parser.parse_args()
    try:
        document = json.loads(args.fixtures.read_text(encoding="utf-8"))
        fixtures = document["fixtures"]
        require(type(fixtures) is list and fixtures, "fixtures must be nonempty")
        results = []
        for fixture in fixtures:
            calculated = reference_vector(fixture["authorization"])
            expected = fixture["expected"]
            require(type(expected) is dict and set(expected) == set(calculated),
                    "incorrect expected vector keys")
            mismatches = [key for key in calculated if calculated[key] != expected[key]]
            results.append({"id": fixture["id"], "calculated": calculated,
                            "mismatches": mismatches})
        passed = all(not result["mismatches"] for result in results)
        if args.json:
            print(json.dumps({"passed": passed, "vectors": results}, indent=2))
        else:
            for result in results:
                vector = result["calculated"]
                verdict = "PASS" if not result["mismatches"] else "FAIL"
                print(f"{verdict} {result['id']}: length={vector['length']} "
                      f"digestHex={vector['digestHex']} "
                      f"fieldOffsets={len(vector['fieldOffsets'])}")
                if result["mismatches"]:
                    print("  mismatches: " + ", ".join(result["mismatches"]))
            print(f"{'PASS' if passed else 'FAIL'}: {len(results)} vectors; "
                  "compared wireHex, digestHex, length, fieldOffsets")
        return 0 if passed else 1
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
