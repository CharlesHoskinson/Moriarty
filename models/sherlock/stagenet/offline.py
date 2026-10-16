"""Inventory supplied Core/1 artifacts and compare existing ledger readback JSON.

This module performs file/JSON operations only. It does not implement Sherlock
lowering, generate witness values, deserialize ZKIR, or qualify a target. The
inventory is an offline input list, not a native proof input or acceptance record.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path
from typing import Any


ARTIFACT_FORMAT = "moriarty-zkir-artifact/3"
ABI = "moriarty-public-abi/3"
RELATION = "moriarty-zkir-relation/3"
READBACK_FIELDS = (
    "execution", "prospective_balances", "effects_hex", "charged_successor_hex",
    "semantic_successor_hex", "native_events_hex", "measured_gas_hex",
    "declared_gas_hex", "retained_utxos_hex", "prospective_outputs_hex",
    "effect_outputs", "change_indices", "requires_reconstruction_after_final_intent",
    "evidence", "preserved_frame",
)
EVIDENCE_FIELDS = (
    "qualification", "relation", "proof_verification", "signature_verification",
    "ledger_apply", "consumption", "fees_paid", "settlement", "deployed_qualification",
)
TARGET_FIELDS = (
    "profile", "network", "anchor", "ledger_revision", "zkir_version",
    "max_instructions", "max_artifact_bytes",
)
CIRCUIT_FIELDS = (
    "action", "zkir", "context_hash", "public_inputs", "authorization_bindings",
    "private_inputs", "instruction_count", "effect_frame", "balance_frame",
    "liability_frame", "replay_steps",
)


def _object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"repeated JSON member: {key}")
        result[key] = value
    return result


def _constant(value: str) -> Any:
    raise ValueError(f"non-JSON numeric constant: {value}")


def document(path: str | Path) -> tuple[dict[str, Any], Any]:
    """Read once; digest names raw file bytes, never a native identity."""
    path = Path(path)
    raw = path.read_bytes()
    value = json.loads(raw, object_pairs_hook=_object, parse_constant=_constant)
    return {"path": str(path.resolve()), "sha256": hashlib.sha256(raw).hexdigest(),
            "bytes": len(raw)}, value


def _fields(value: Any, names: tuple[str, ...], location: str) -> None:
    if not isinstance(value, dict):
        raise ValueError(f"{location}: expected an object")
    missing = [name for name in names if name not in value]
    if missing:
        raise ValueError(f"{location}: missing fields {', '.join(missing)}")


def _list(value: Any, location: str) -> None:
    if not isinstance(value, list):
        raise ValueError(f"{location}: expected an array")


def readback_shape(value: Any) -> None:
    """Require the existing complete projection, retaining all additional fields.

    This checks container membership only. Native hex values, evidence claims,
    and semantic correctness are not validated by Python JSON inspection.
    """
    _fields(value, READBACK_FIELDS, "Readback")
    execution = value["execution"]
    _fields(execution, ("semantics", "action", "pre", "post", "steps", "effects"),
            "Readback.execution")
    if execution["semantics"] != "moriarty-core/1":
        raise ValueError("Readback.execution: only the existing Core/1 projection is mapped")
    _list(execution["steps"], "Readback.execution.steps")
    _list(execution["effects"], "Readback.execution.effects")
    snapshots = [("pre", execution["pre"]), ("post", execution["post"])]
    for index, step in enumerate(execution["steps"]):
        location = f"Readback.execution.steps/{index}"
        _fields(step, ("index", "statement", "value", "snapshot", "effects"), location)
        _list(step["effects"], location + "/effects")
        snapshots.append((f"steps/{index}/snapshot", step["snapshot"]))
    for location, snapshot in snapshots:
        _fields(snapshot, ("state", "balances", "liabilities"), location)
        _fields(snapshot["state"], (), location + "/state")
        _list(snapshot["balances"], location + "/balances")
        _list(snapshot["liabilities"], location + "/liabilities")
    for name in ("prospective_balances", "native_events_hex", "retained_utxos_hex",
                 "prospective_outputs_hex", "effect_outputs", "change_indices"):
        _list(value[name], "Readback." + name)
    _fields(value["evidence"], EVIDENCE_FIELDS, "Readback.evidence")
    _fields(value["preserved_frame"], ("scope", "commitments"), "Readback.preserved_frame")
    _fields(value["preserved_frame"]["commitments"], (), "Readback.preserved_frame.commitments")


def _pointer(path: str, key: str | int) -> str:
    return path + "/" + str(key).replace("~", "~0").replace("/", "~1")


def first_difference(expected: Any, observed: Any, path: str = "") -> dict[str, Any] | None:
    """Compare all members and ordered array entries, with no numeric coercion."""
    def difference(kind: str, left: Any, right: Any, at: str = path) -> dict[str, Any]:
        return {"path": at, "kind": kind, "expected": left, "observed": right}

    if type(expected) is not type(observed):
        return difference("type", expected, observed)
    if isinstance(expected, dict):
        for key in sorted(expected.keys() | observed.keys()):
            at = _pointer(path, key)
            if key not in expected or key not in observed:
                result = difference("member", expected.get(key), observed.get(key), at)
                result.update(expected_present=key in expected, observed_present=key in observed)
                return result
            found = first_difference(expected[key], observed[key], at)
            if found is not None:
                return found
    elif isinstance(expected, list):
        for index, (left, right) in enumerate(zip(expected, observed)):
            found = first_difference(left, right, _pointer(path, index))
            if found is not None:
                return found
        if len(expected) != len(observed):
            return difference("array-length", expected, observed)
    elif expected != observed:
        return difference("value", expected, observed)
    return None


def frozen_readback(path: str | Path, sha256: str) -> tuple[dict[str, Any], Any]:
    ref, value = document(path)
    if ref["sha256"] != sha256:
        raise ValueError("expected readback bytes differ from the externally frozen SHA-256")
    readback_shape(value)
    return ref, value


def prepare(*, artifact_path: str | Path, target_path: str | Path,
            scenario_path: str | Path, expected_path: str | Path, expected_sha256: str,
            action: str, application_scope: str, source_path: str | Path | None = None,
            external_configuration_path: str | Path | None = None) -> dict[str, Any]:
    """Describe existing backend inputs without invoking that backend.

    Target is independently supplied; absent fields are reported, not filled
    from artifact.target. Source is only inventoried, never parsed or compiled.
    """
    artifact_ref, artifact = document(artifact_path)
    target_ref, target = document(target_path)
    scenario_ref, scenario = document(scenario_path)
    expected_ref, expected = frozen_readback(expected_path, expected_sha256)
    _fields(artifact, ("format", "abi", "relation", "authorization_mode", "program_hash",
                       "semantics", "target", "circuits", "resources", "gates"), "Artifact")
    if (artifact["format"], artifact["abi"], artifact["relation"]) != (ARTIFACT_FORMAT, ABI, RELATION):
        raise ValueError("no input adapter for this artifact/ABI/relation tuple")
    if artifact["semantics"] != "moriarty-core/1":
        raise ValueError("no input adapter for this artifact semantics")
    _fields(target, (), "Target")
    _fields(scenario, ("state", "inputs", "balances", "liabilities"), "Scenario")
    _list(artifact["circuits"], "Artifact.circuits")
    for circuit in artifact["circuits"]:
        _fields(circuit, CIRCUIT_FIELDS, "CircuitArtifact")
    selected = [c for c in artifact["circuits"] if c["action"] == action]
    if len(selected) != 1:
        raise ValueError("action must select exactly one supplied CircuitArtifact")
    circuit = selected[0]
    for name in ("public_inputs", "authorization_bindings", "private_inputs",
                 "effect_frame", "balance_frame", "liability_frame"):
        _list(circuit[name], "CircuitArtifact." + name)
    if expected["execution"]["action"] != action:
        raise ValueError("frozen readback action differs from selected artifact action")
    expected_pre = expected["execution"]["pre"]
    supplied_pre = {key: scenario[key] for key in ("state", "balances", "liabilities")}
    if first_difference(expected_pre, supplied_pre) is not None:
        raise ValueError("Scenario predecessor differs from frozen Readback.execution.pre")
    if application_scope not in ("pure/no-financial-effects", "financial"):
        raise ValueError("application scope must be supplied explicitly")
    if application_scope == "pure/no-financial-effects":
        snapshots = [expected["execution"]["pre"], expected["execution"]["post"]]
        snapshots.extend(step["snapshot"] for step in expected["execution"]["steps"])
        effects = list(expected["execution"]["effects"])
        for step in expected["execution"]["steps"]:
            effects.extend(step["effects"])
        if (circuit["balance_frame"] or circuit["liability_frame"]
                or any(s["balances"] or s["liabilities"] for s in snapshots)
                or any(e.get("kind") not in ("State", "Emit") for e in effects)
                or any(e.get("kind") not in ("state", "emit") for e in circuit["effect_frame"])):
            raise ValueError("supplied frames/readback do not describe the pure application slice")
    gaps = [
        "Sherlock/Clean-to-Midnight source-to-target adapter is not provided by this inventory.",
        "Backend Artifact/3 is not a mori-ledger native-call Artifact or PreparedNativeCall.",
        "Native field encoding, witness construction and ABI correspondence remain unestablished.",
        "Target endpoints, verifier compatibility, submission and readback position need external evidence.",
        "Proof/key identity, proof verification, deployment and target observation are not established.",
        "Frozen expectation independence requires non-author review; a digest only identifies bytes.",
        "Network fees remain unobserved, including for pure applications.",
    ]
    if application_scope == "financial":
        gaps.append("Gross debit, fees, net receipts and residual-duty accounting need an explicit adapter; no financial acceptance is supplied here.")
    result = {
        "purpose": "offline-run-input-inventory",
        "acceptance": "none",
        "requested_target": "Midnight stagenet",
        "application_scope": application_scope,
        "files": {"artifact": artifact_ref, "target": target_ref, "scenario": scenario_ref,
                  "expected_readback": expected_ref},
        "artifact": artifact,
        "target": target,
        "scenario": scenario,
        "expected_readback": expected,
        "selected_circuit_index": artifact["circuits"].index(circuit),
        "missing_target_fields": [name for name in TARGET_FIELDS if target.get(name) in (None, "")],
        "artifact_target_difference": first_difference(artifact["target"], target),
        "gaps": gaps,
    }
    if source_path is not None:
        path = Path(source_path)
        raw = path.read_bytes()
        result["files"]["source"] = {"path": str(path.resolve()), "sha256": hashlib.sha256(raw).hexdigest(), "bytes": len(raw)}
    else:
        gaps.append("Source bytes were not supplied.")
    if external_configuration_path is not None:
        ref, _ = document(external_configuration_path)
        result["files"]["external_configuration"] = ref
    else:
        gaps.append("External target configuration was not supplied.")
    return copy.deepcopy(result)


def compare(*, expected_path: str | Path, expected_sha256: str,
            observed_path: str | Path) -> dict[str, Any]:
    """Compare the complete supplied readback; never promote evidence labels."""
    expected_ref, expected = frozen_readback(expected_path, expected_sha256)
    observed_ref, observed = document(observed_path)
    # Keep incomplete observations diagnosable as differences; only the frozen
    # reference must have the complete existing Readback container shape.
    delta = first_difference(expected, observed)
    result = {"scope": "supplied-json-comparison-only", "acceptance": "none", "expected": expected_ref,
              "observed": observed_ref, "first_difference": delta}
    if delta is not None and delta["path"].startswith("/execution/steps/"):
        index_text = delta["path"].split("/")[3]
        if index_text.isdecimal():
            index = int(index_text)
            result["expected_step"] = expected["execution"]["steps"][index]
            result["observed_step"] = observed["execution"]["steps"][index]
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    inventory = commands.add_parser("prepare")
    for flag in ("artifact", "target", "scenario", "action", "application-scope"):
        inventory.add_argument("--" + flag, required=True)
    inventory.add_argument("--source")
    inventory.add_argument("--external-configuration")
    comparison = commands.add_parser("compare")
    comparison.add_argument("--observed", required=True)
    for command in (inventory, comparison):
        command.add_argument("--expected", required=True)
        command.add_argument("--expected-sha256", required=True)
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            result = prepare(artifact_path=args.artifact, target_path=args.target,
                             scenario_path=args.scenario, expected_path=args.expected,
                             expected_sha256=args.expected_sha256, action=args.action,
                             application_scope=args.application_scope, source_path=args.source,
                             external_configuration_path=args.external_configuration)
        else:
            result = compare(expected_path=args.expected, expected_sha256=args.expected_sha256,
                             observed_path=args.observed)
    except (OSError, ValueError) as error:
        parser.exit(2, f"{error}\n")
    print(json.dumps(result, indent=2, ensure_ascii=True, allow_nan=False))
    return 1 if args.command == "compare" and result["first_difference"] is not None else 0


if __name__ == "__main__":
    raise SystemExit(main())
