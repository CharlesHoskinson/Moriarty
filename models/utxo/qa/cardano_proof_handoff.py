#!/usr/bin/env python3
"""Inspect existing public K6..K10 bytes and expose the C2 builder inputs.

This module performs JSON, length and hash checks only. It neither generates nor
verifies proofs, and never starts a tool, reads a key or contacts a ledger.
Historical observations below are declarations in the pinned source records.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import stat


EVIDENCE = "evidence/utxo/"
PROOF = EVIDENCE + "current-rust-groth16-first-proof-2026-10-16/"
VERIFIER = EVIDENCE + "current-rust-groth16-public-verifier-record-2026-10-16/"
CONSUMERS = EVIDENCE + "current-cardano-native-proof-consumers-2026-10-16/"
WRAPPER = EVIDENCE + "current-cardano-compiled-claim-wrapper-2026-10-16/"
# These are publication-relative paths. The embedded historical home directory
# is part of K10's public archive, never an instruction to read outside repo.
ARCHIVE = WRAPPER + "payload/inputs/home/charl/"
GENERATED = ARCHIVE + "moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k10-public-inputs-generated-20261016/"

# Exact existing public payload pins; no proving-key path is consumed.
ARTIFACTS = {
    "proof_export": (PROOF + "run/receipts/invoice-partial.proof.json", 5800,
                     "930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b"),
    "setup_record": (VERIFIER + "receipts/invoice-partial.setup.json", 3926,
                     "86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"),
    "K6": (PROOF + "root/K6-ACTUAL-ADOPTION.json", 4005,
           "83a6e4fd1624774ee050ce88d3671187548e65c294ad147e5910a42685a6cb4d"),
    "K7": (VERIFIER + "root/K7-ACTUAL-ADOPTION.json", 1831,
           "8abe3d64c3aa1fee865e04c45c081f81f0d51a03309add4eb20cbb961dd80114"),
    "K8": (CONSUMERS + "k8/root/K8-ACTUAL-ADOPTION.json", 5015,
           "99541a43baf1a28c251a843ca7b62011c83f594d19a1dd65b942d85a9fcc76ff"),
    "K9": (CONSUMERS + "k9/root/K9-ACTUAL-ADOPTION.json", 7503,
           "76ccfd083d9dd123f03c4d58d6c358cc7edb26d5d4eb4069e09bee5481e01629"),
    "K10": (WRAPPER + "root/K10-ACTUAL-ADOPTION.json", 10781,
            "6421ba8478218c633c4604a9d25b82010afe4c1a9ffb359275d7cf16be6cfc18"),
    "blueprint": (ARCHIVE + "Moriarty/.worktrees/utxo/evidence/utxo/cardano-four-piece-settlement-2026-10-16/run/a2/b9-applied.json", 308672,
                  "10baa1794e0ae286cd780978408552fc0e994426367c2b92117e2c4b46ad1d36"),
    "fixture": (CONSUMERS + "k8/actual/staged/lib/pvc2/k8_fixture.ak", 5269,
                "1b9e2a9a2eb6a2b47915bfc9aacd3b68a59d76f189f98da5615b4c653923ef64"),
    "input_manifest": (GENERATED + "manifest.json", 10588,
                       "10e83e23d0ab6c6f7d37cb6898e23fb473394eac5e42350dfdbd589a74ef5f11"),
    "script_hex": (GENERATED + "script.hex", 5541,
                   "7068c0673286e9d506183106f39e34d24872c9c794f323d0113988ba7ff3139f"),
    "positive_context": (GENERATED + "contexts/K10-T1-positive-genuine-issue.uplc.txt", 2649,
                         "bad16e8d89d5ad47bc94965874ef3b243bee7ccb983a68f34499598b0d577b1e"),
}


def _read_public(repo: Path, relative: str, expected_size: int) -> bytes:
    """Read only the named regular public file within the selected checkout."""
    path = repo
    for part in Path(relative).parts:
        path = path / part
        if path.is_symlink():
            raise ValueError("public artifact path contains a symlink")
    info = path.stat()
    if not stat.S_ISREG(info.st_mode):
        raise ValueError("public artifact is not a regular file")
    if info.st_size != expected_size:
        raise ValueError(f"byte length differs: expected {expected_size}, found {info.st_size}; SHA-256 pin not satisfied")
    return path.read_bytes()


def inspect_handoff(repo: Path) -> dict:
    """Return a non-mutating handoff for the fixed public invoice-partial proof.

    `ready_for_payload_encoding` means the existing public bytes agree with
    their pins and ABI. It never means a transaction is ready for submission.
    """
    repo = Path(repo).resolve()
    artifacts, raw, blockers = {}, {}, []
    for name, (path, size, digest) in ARTIFACTS.items():
        item = {"path": path, "bytes": size, "sha256": digest, "present": False, "matches": False}
        artifacts[name] = item
        try:
            content = _read_public(repo, path, size)
            item["present"] = True
            if hashlib.sha256(content).hexdigest() != digest:
                raise ValueError("SHA-256 differs from the existing public artifact pin")
            item["matches"] = True
            raw[name] = content
        except (OSError, ValueError) as error:
            blockers.append(f"{name}: {error}")

    result = {
        "format": "doctorwatson-utxo-cardano-proof-handoff/1",
        "available": all(item["present"] for item in artifacts.values()),
        "ready_for_payload_encoding": False,
        "artifacts": artifacts, "identity": {}, "observations": {},
        "relation": {}, "statement": {},
        "required_lengths": {"verification_key_bytes": 488, "proof_bytes": 192,
                             "statement_bytes": 721, "compiled_script_bytes": 2770},
        "builder_payload": None, "blockers": blockers,
        "scope": {
            "trust_profile": "single-party-test-setup/1", "setup_class": "Class0",
            "context_description": "Historical K6/K10 caller-fabricated claimant 555, outref 2222, settle-A2, zero fee and unbound 2030 expiry.",
            "authenticated_ledger_context": False, "preview_settlement_observed": False,
            "qualification": "Pinned public records only; no new proof verification. Setup controller can forge; VK-to-matrix correspondence is unproved in the source records.",
        },
        "next_step": "Encode the existing C2 Issue redeemer and inline envelope datum for a Cardano transaction builder; bind the applied script to the Preview candidate, balance and evaluate with current protocol parameters before submission. A real-context settlement needs a proof for authenticated thread/wallet premises and genuine intent signatures; this historical fixture cannot establish those claims.",
    }
    if blockers:
        return result

    try:
        documents = {name: json.loads(data) for name, data in raw.items()
                     if artifacts[name]["path"].endswith(".json")}
        proof, setup = documents["proof_export"], documents["setup_record"]
        manifest, blueprint = documents["input_manifest"], documents["blueprint"]
        vk = bytes.fromhex(setup["verifyingKeyHex"])
        proof_bytes = bytes.fromhex(proof["proofHex"])
        statement = bytes.fromhex(proof["statementHex"])

        def require(condition, message):
            if not condition:
                raise ValueError(message)

        require((len(vk), len(proof_bytes), len(statement)) == (488, 192, 721),
                "public VK/proof/statement must have existing lengths 488/192/721")
        require(proof["setupRecordSha256"] == artifacts["setup_record"]["sha256"],
                "proof setup-record identity differs")
        for field in ("programHash", "sourceSha256", "action", "actionIndex", "compilerBuildDigest",
                      "r1csSha256", "predicateDigest", "abi2ShapeDigest"):
            require(proof[field] == setup["relation"][field], f"relation identity differs: {field}")
        require(hashlib.blake2b(vk, digest_size=32).hexdigest() == setup["vkDigestBlake2b256"]
                == proof["vkDigestBlake2b256"] == manifest["vkDigestHex"], "public VK digest differs")
        require(hashlib.blake2b(statement, digest_size=32).hexdigest()
                == proof["statementDigestBlake2b256"] == manifest["statementDigestHex"],
                "statement digest differs")
        mint = next(v for v in blueprint["validators"] if v["title"] == "c2.c2.mint")
        require("parameters" not in mint or mint["parameters"] == [], "C2 blueprint still needs parameter application")
        schema = blueprint["definitions"]["pvc2/types/C2Mint"]["anyOf"][0]
        require(schema["title"] == "Issue" and schema["index"] == 0
                and [f["title"] for f in schema["fields"]] == ["vk", "proof"],
                "C2 Issue constructor or field order differs")
        require(mint["redeemer"]["schema"]["$ref"] == "#/definitions/pvc2~1types~1C2Mint",
                "C2 mint redeemer schema differs")
        script = bytes.fromhex(mint["compiledCode"])
        require(len(script) == 2770 and script == bytes.fromhex(raw["script_hex"].decode().strip()),
                "applied C2 blueprint and existing script bytes differ")
        script_hash = hashlib.blake2b(b"\x03" + script, digest_size=28).hexdigest()
        require(script_hash == mint["hash"] == manifest["policyIdHex"], "compiled C2 script hash differs")
        positive = manifest["cases"][0]
        require(positive["id"] == "K10-T1-positive-genuine-issue"
                and positive["proofHexSha256"] == hashlib.sha256(proof_bytes).hexdigest(),
                "K10 positive fixture proof identity differs")
        require(positive["envVkDigestHex"] == proof["vkDigestBlake2b256"]
                and positive["envStatementDigestHex"] == proof["statementDigestBlake2b256"],
                "K10 positive fixture statement/VK identity differs")
        envelope_hex = manifest["positiveEnvelopeCborHex"]
        token_name = hashlib.blake2b(bytes.fromhex(envelope_hex), digest_size=32).hexdigest()
        require(token_name == manifest["positiveTokenNameHex"] == positive["tokenNameHex"],
                "K10 envelope token-name identity differs")
        fixture = raw["fixture"].decode()
        context = raw["positive_context"].decode()
        require(proof["proofHex"] in fixture and setup["verifyingKeyHex"] in fixture
                and proof["proofHex"] in context and setup["verifyingKeyHex"] in context,
                "existing Aiken/context fixture does not contain the public proof/VK")
        cases = documents["K10"]["cases"]
        require(len(cases) == 7 and cases[0]["validation"]["observed"] == "ACCEPT_UNIT"
                and all(c["validation"]["observed"] == "SEMANTIC_EVALUATION_FAILURE" for c in cases[1:]),
                "K10 recorded local result shape differs")
        result["identity"] = {
            "proof_export_sha256": artifacts["proof_export"]["sha256"],
            "proof_sha256": hashlib.sha256(proof_bytes).hexdigest(),
            "setup_record_sha256": proof["setupRecordSha256"],
            "vk_digest_blake2b256": proof["vkDigestBlake2b256"],
            "statement_digest_blake2b256": proof["statementDigestBlake2b256"],
            "program_hash": proof["programHash"], "r1cs_sha256": proof["r1csSha256"],
            "predicate_digest": proof["predicateDigest"], "abi2_shape_digest": proof["abi2ShapeDigest"],
            "script_hash": script_hash, "script_sha256": hashlib.sha256(script).hexdigest(),
            "token_name": token_name,
        }
        result["relation"] = {field: setup["relation"][field] for field in (
            "relationFormat", "relationVersion", "language", "semantics", "action",
            "actionIndex", "adapter", "publicInputCount", "headerLen", "bodyLen")}
        result["statement"] = {
            "digest_blake2b256": proof["statementDigestBlake2b256"],
            "public_inputs": proof["publicInputs"],
            "context": proof["context"],
            "bytes_source": {"path": artifacts["proof_export"]["path"], "field": "statementHex"},
        }
        result["observations"] = {
            name: {"source": artifacts[name]["path"], "scope": documents[name].get("scope", documents[name].get("predicate", "Public VK export only")),
                   "evidence_kind": "historical-local-record"}
            for name in ("K6", "K7", "K8", "K9", "K10")
        }
        result["observations"]["K10"].update(
            positive_cases=1, negative_cases=6, compiled_script_bytes=len(script),
            local_costs=cases[0]["validation"]["costs"], ledger_exunits_qualified=False)
        result["builder_payload"] = {
            "validator_title": mint["title"], "plutus_version": "v3", "policy_id_hex": script_hash,
            "script_hex_path": artifacts["script_hex"]["path"],
            "redeemer_data": {"constructor": 0, "fields": [{"bytes": setup["verifyingKeyHex"]}, {"bytes": proof["proofHex"]}]},
            "redeemer_abi": "C2Mint.Issue(vk, proof); constructor 0, positional byte strings; blueprint #/definitions/pvc2~1types~1C2Mint",
            "inline_datum_cbor_hex": envelope_hex, "token_name_hex": token_name, "mint_quantity": positive["quantity"],
            "context_fixture_path": artifacts["positive_context"]["path"],
            "context_fixture_format": "UPLC ScriptContext text for historical local evaluation; not transaction CBOR",
            "transaction_cbor": None,
        }
        result["ready_for_payload_encoding"] = True
    except (KeyError, TypeError, ValueError, StopIteration) as error:
        blockers.append(f"public artifact data: {error}")
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument("--json", action="store_true", help="Emit the public handoff as JSON (also the default).")
    args = parser.parse_args(argv)
    result = inspect_handoff(args.repo)
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if result["ready_for_payload_encoding"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
