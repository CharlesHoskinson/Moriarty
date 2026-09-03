#!/usr/bin/env python3
"""Validate Moriarty sprint instructions and declared evidence closure."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator
from jsonschema.exceptions import ValidationError as JsonSchemaValidationError


ROOT = Path(__file__).parents[1]


class ValidationError(ValueError):
    """A deterministic instruction or evidence validation failure."""


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ValidationError(f"invalid JSON: {path.relative_to(ROOT)}: {error}") from error
    if not isinstance(value, dict):
        raise ValidationError(f"expected JSON object: {path.relative_to(ROOT)}")
    return value


def file_sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical_sha256(value: object) -> str:
    payload = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def validate_evidence_manifest(
    evidence: dict[str, Any],
    expected_package_id: str,
    current_scope_index: dict[str, Any],
) -> None:
    required = {
        "schema_id",
        "package_id",
        "sprint_id",
        "semantic_scope_version",
        "semantic_scope_sha256",
        "semantic_scope_index_sha256",
        "inputs",
        "outputs",
        "commands",
        "environment",
        "status",
        "gate_passed",
        "gate_results",
        "limitations",
        "manifest_sha256",
    }
    missing = required - set(evidence)
    if missing:
        raise ValidationError(f"evidence manifest lacks fields: {sorted(missing)}")
    if evidence.get("schema_id") != "moriarty.dev/evidence-manifest/v1":
        raise ValidationError("evidence manifest schema identifier is unsupported")
    if evidence.get("package_id") != expected_package_id:
        raise ValidationError(
            f"evidence manifest package mismatch: expected {expected_package_id}"
        )
    sprint_id = evidence.get("sprint_id")
    if not isinstance(sprint_id, str) or not sprint_id.startswith("S"):
        raise ValidationError("evidence manifest sprint identifier is invalid")
    if evidence.get("semantic_scope_version") != current_scope_index.get(
        "current_version"
    ):
        raise ValidationError("evidence manifest semantic scope version mismatch")
    if evidence.get("semantic_scope_sha256") != current_scope_index.get(
        "current_sha256"
    ):
        raise ValidationError("evidence manifest semantic scope digest mismatch")
    scope_index_path = ROOT / "evidence/semantic-scope/index.json"
    if evidence.get("semantic_scope_index_sha256") != file_sha256(scope_index_path):
        raise ValidationError("evidence manifest semantic scope index digest mismatch")
    if not isinstance(evidence.get("inputs"), dict) or not evidence["inputs"]:
        raise ValidationError("evidence manifest inputs are invalid")
    if (
        not isinstance(evidence.get("commands"), list)
        or not evidence["commands"]
        or not all(isinstance(item, str) and item for item in evidence["commands"])
    ):
        raise ValidationError("evidence manifest commands are invalid")
    if not isinstance(evidence.get("environment"), dict) or not evidence["environment"]:
        raise ValidationError("evidence manifest environment is invalid")
    if (
        not isinstance(evidence.get("limitations"), list)
        or not evidence["limitations"]
        or not all(isinstance(item, str) and item for item in evidence["limitations"])
    ):
        raise ValidationError("evidence manifest limitations are invalid")
    if not isinstance(evidence.get("status"), str) or not evidence["status"]:
        raise ValidationError("evidence manifest status is invalid")
    if evidence.get("gate_passed") is not True:
        raise ValidationError("evidence manifest gate did not pass")
    if not isinstance(evidence.get("gate_results"), dict):
        raise ValidationError("evidence manifest gate results are invalid")
    claimed_manifest_sha256 = evidence.get("manifest_sha256")
    unhashed = dict(evidence)
    unhashed.pop("manifest_sha256", None)
    if claimed_manifest_sha256 != canonical_sha256(unhashed):
        raise ValidationError("evidence manifest self-hash is invalid")
    outputs = evidence.get("outputs")
    if not isinstance(outputs, list) or not outputs:
        raise ValidationError("evidence manifest has no declared outputs")
    for output in outputs:
        if not isinstance(output, dict):
            raise ValidationError("evidence manifest output must be an object")
        relative = output.get("path")
        expected_sha256 = output.get("sha256")
        if not isinstance(relative, str) or not isinstance(expected_sha256, str):
            raise ValidationError("evidence manifest output lacks path or SHA-256")
        path = (ROOT / relative).resolve()
        if not path.is_relative_to(ROOT.resolve()):
            raise ValidationError("evidence manifest output escapes the repository")
        if not path.is_file():
            raise ValidationError(f"declared evidence output is missing: {relative}")
        if file_sha256(path) != expected_sha256:
            raise ValidationError(f"declared evidence digest mismatch: {relative}")


def validate_sdk_contracts(
    index_override: dict[str, Any] | None = None,
    component_inventory_override: dict[str, Any] | None = None,
    data_contracts_override: dict[str, Any] | None = None,
) -> dict[str, int]:
    sdk_root = ROOT / "openspec/changes/wp09-complete-development-sdk"
    component_inventory_path = sdk_root / "sdk-component-inventory.json"
    data_contracts_path = sdk_root / "sdk-data-contracts.json"
    component_inventory = (
        component_inventory_override
        if component_inventory_override is not None
        else load_json(component_inventory_path)
    )
    data_contracts = (
        data_contracts_override
        if data_contracts_override is not None
        else load_json(data_contracts_path)
    )
    components = component_inventory.get("components")
    contracts = data_contracts.get("contracts")
    if not isinstance(components, list) or len(components) != 65:
        raise ValidationError("SDK component inventory must contain exactly 65 components")
    if not isinstance(contracts, list) or len(contracts) != 28:
        raise ValidationError("SDK data-contract inventory must contain exactly 28 contracts")

    component_schema_path = ROOT / str(component_inventory.get("contract_schema"))
    data_schema_path = ROOT / str(data_contracts.get("contract_schema"))
    component_validator = Draft202012Validator(load_json(component_schema_path))
    data_validator = Draft202012Validator(load_json(data_schema_path))
    try:
        for component in components:
            component_validator.validate(component)
        for contract in contracts:
            data_validator.validate(contract)
    except JsonSchemaValidationError as error:
        raise ValidationError(f"SDK contract schema validation failed: {error.message}") from error

    component_ids = [component["id"] for component in components]
    contract_names = [contract["name"] for contract in contracts]
    contract_schema_ids = [contract["schema_id"] for contract in contracts]
    if len(set(component_ids)) != len(component_ids):
        raise ValidationError("SDK component identifiers are not unique")
    if len(set(contract_names)) != len(contract_names):
        raise ValidationError("SDK data-contract names are not unique")
    if len(set(contract_schema_ids)) != len(contract_schema_ids):
        raise ValidationError("SDK data-contract schema identifiers are not unique")

    for component in components:
        specification = sdk_root / component["specification"]
        if not specification.is_file():
            raise ValidationError(
                f"SDK component specification is missing: {component['id']}"
            )

    component_id_set = set(component_ids)
    unresolved: list[str] = []
    for contract in contracts:
        for reference in [contract["producer"], *contract["consumers"]]:
            is_external = isinstance(reference, str) and re.fullmatch(
                r"external:[a-z][a-z0-9-]*", reference
            )
            if reference not in component_id_set and not is_external:
                unresolved.append(f"{contract['name']}:{reference}")
    if unresolved:
        raise ValidationError(f"SDK data-contract references are unresolved: {unresolved}")

    minimum_safety_spine = sum(
        component["implementation_scope"] == "minimum-safety-spine"
        for component in components
    )
    statistics = {
        "components": len(components),
        "data_contracts": len(contracts),
        "capability_specs": 6,
        "minimum_safety_spine": minimum_safety_spine,
        "complete_sdk_specification": len(components) - minimum_safety_spine,
        "unique_component_ids": len(set(component_ids)),
        "unique_data_contract_names": len(set(contract_names)),
        "unique_data_contract_schema_ids": len(set(contract_schema_ids)),
        "unresolved_component_references": len(unresolved),
    }
    index = (
        index_override
        if index_override is not None
        else load_json(ROOT / "evidence/wp09/sdk-contract-index.json")
    )
    current_scope = load_json(ROOT / "evidence/semantic-scope/index.json")
    expected_index_identity = {
        "schema_version": 1,
        "package": "WP09",
        "sprint": "S08",
        "status": "specified-only",
        "semantic_scope_version": current_scope["current_version"],
        "semantic_scope_sha256": current_scope["current_sha256"],
    }
    for field, expected in expected_index_identity.items():
        if index.get(field) != expected:
            raise ValidationError(f"SDK contract index {field} is stale")
    expected_top_level_fields = {
        "schema_version",
        "package",
        "sprint",
        "status",
        "semantic_scope_version",
        "semantic_scope_sha256",
        "component_inventory",
        "data_contract_inventory",
        "capability_specs",
        "statistics",
        "completeness_predicate",
        "gate_state",
        "limitations",
    }
    if set(index) != expected_top_level_fields:
        raise ValidationError("SDK contract index has unvalidated fields")
    if index.get("statistics") != statistics:
        raise ValidationError("SDK contract-index statistics are stale")
    inventory_checks = (
        (
            "component_inventory",
            component_inventory_path,
            component_schema_path,
            {
                "components": len(components),
                "unique_identifiers": len(set(component_ids)),
                "minimum_safety_spine": minimum_safety_spine,
                "specified_only_after_safety_spine": len(components)
                - minimum_safety_spine,
            },
        ),
        (
            "data_contract_inventory",
            data_contracts_path,
            data_schema_path,
            {
                "contracts": len(contracts),
                "unique_identifiers": len(set(contract_names)),
            },
        ),
    )
    for key, path, schema_path, expected_counts in inventory_checks:
        record = index.get(key, {})
        expected_record_fields = {
            "path",
            "sha256",
            "schema",
            "schema_sha256",
            *expected_counts,
        }
        if set(record) != expected_record_fields:
            raise ValidationError(f"SDK contract index {key} fields are stale")
        if record.get("path") != str(path.relative_to(ROOT)):
            raise ValidationError(f"SDK contract index {key} path is stale")
        if record.get("schema") != str(schema_path.relative_to(ROOT)):
            raise ValidationError(f"SDK contract index {key} schema path is stale")
        if record.get("sha256") != file_sha256(path):
            raise ValidationError(f"SDK contract-index {key} digest is stale")
        if record.get("schema_sha256") != file_sha256(schema_path):
            raise ValidationError(f"SDK contract-index {key} schema digest is stale")
        for field, expected in expected_counts.items():
            if record.get(field) != expected:
                raise ValidationError(f"SDK contract index {key}.{field} is stale")
    expected_capability_specs = []
    for path in sorted((sdk_root / "specs").glob("*/spec.md")):
        expected_capability_specs.append(
            {
                "id": path.parent.name,
                "path": str(path.relative_to(ROOT)),
                "sha256": file_sha256(path),
            }
        )
    if index.get("capability_specs") != expected_capability_specs:
        raise ValidationError("SDK contract index capability specifications are stale")
    expected_completeness = {
        "component_fields": [
            "id",
            "package",
            "inputs",
            "outputs",
            "failure_codes",
            "trust_roots",
            "version_fields",
            "security_checks",
            "conformance_vectors",
            "specification",
            "implementation_scope",
        ],
        "data_contract_fields": [
            "name",
            "schema_id",
            "required_fields",
            "canonical_encoding",
            "producer",
            "consumers",
            "validation",
            "unknown_fields",
            "privacy",
        ],
        "command": "uv run pytest tests/test_openspec_work_packages.py -q",
    }
    if index.get("completeness_predicate") != expected_completeness:
        raise ValidationError("SDK contract index completeness predicate is stale")
    expected_gate_state = {
        "complete_specification": True,
        "minimum_safety_spine_implemented": False,
        "malicious_plan_suite_passed": False,
        "signing_path_verified": False,
        "wp09_gate_passed": False,
    }
    if index.get("gate_state") != expected_gate_state:
        raise ValidationError("SDK contract index gate state is stale")
    expected_limitations = [
        "This index establishes specification completeness, not implementation completeness.",
        "Conformance vector paths are contracted but most vectors do not yet exist.",
        "WP10 and WP11 cannot use specified-only components as measured or audited implementation evidence.",
    ]
    if index.get("limitations") != expected_limitations:
        raise ValidationError("SDK contract index limitations are stale")
    return statistics


def validate_instruction_manifest(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    if manifest.get("schema_version") != 1:
        raise ValidationError("unsupported work-package manifest schema")
    if manifest.get("progress_unit") != "evidence-gated-sprint":
        raise ValidationError("progress_unit must be evidence-gated-sprint")

    packages = manifest.get("packages")
    if not isinstance(packages, list) or len(packages) != 12:
        raise ValidationError("manifest must contain exactly twelve packages")

    expected_ids = [f"WP{number:02d}" for number in range(1, 13)]
    actual_ids = [package.get("id") for package in packages]
    if actual_ids != expected_ids:
        raise ValidationError("package identifiers must be ordered WP01 through WP12")

    seen: set[str] = set()
    declared_outputs: set[str] = set()
    required = {
        "id",
        "sprint",
        "dependencies",
        "artifact_root",
        "outputs",
        "validator_command",
        "gate",
        "failure_outcomes",
        "failure_selection_rule",
    }
    for package in packages:
        missing = required - set(package)
        if missing:
            raise ValidationError(f"{package.get('id', 'unknown')} missing {sorted(missing)}")
        package_id = package["id"]
        dependencies = package["dependencies"]
        if not isinstance(dependencies, list) or any(item not in seen for item in dependencies):
            raise ValidationError(f"{package_id} has a forward or unknown dependency")
        expected_root = f"evidence/{package_id.lower()}/"
        if package["artifact_root"] != expected_root:
            raise ValidationError(f"{package_id} artifact root must be {expected_root}")
        outputs = package["outputs"]
        if not isinstance(outputs, list) or not outputs:
            raise ValidationError(f"{package_id} has no declared outputs")
        duplicates = declared_outputs.intersection(outputs)
        allowed_shared = {"evidence/semantic-scope/index.json"}
        if duplicates - allowed_shared:
            raise ValidationError(f"duplicate output ownership: {sorted(duplicates)}")
        declared_outputs.update(outputs)
        expected_command = (
            "uv run python scripts/validate_sprint_evidence.py "
            f"--package {package_id} --manifest openspec/work-packages.json"
        )
        if package["validator_command"] != expected_command:
            raise ValidationError(f"{package_id} validator command is not canonical")
        if not package["gate"] or not package["failure_outcomes"]:
            raise ValidationError(f"{package_id} has no closed gate or failure outcome")
        seen.add(package_id)

    scope_index_path = ROOT / manifest["semantic_scope_index"]
    scope_index = load_json(scope_index_path)
    snapshot_path = ROOT / scope_index["current_path"]
    if file_sha256(snapshot_path) != scope_index["current_sha256"]:
        raise ValidationError("semantic-scope index digest does not match its snapshot")

    validate_sdk_contracts()
    return packages


def validate_wp01_gate(evidence: dict[str, Any]) -> None:
    expected_results = {
        "fresh_environment": True,
        "semantic_artifact_digests_match": True,
        "compiler_artifact_digests_match_for_recorded_command": True,
        "trace_divergence_count": 0,
        "invariant_failure_count": 0,
        "third_party_disclosure_negative_rejected": True,
        "moriarty_missing_disclosure_rejected": True,
        "moriarty_additional_disclosure_rejected": True,
        "unbounded_witness_path": False,
    }
    if evidence.get("gate_results") != expected_results:
        raise ValidationError("WP01 gate results do not satisfy the closed predicate")

    reproduction = load_json(ROOT / "evidence/wp01/reproduction-receipt.json")
    reproduction_path = ROOT / "evidence/wp01/reproduction-receipt.json"
    clean_input = evidence.get("inputs", {}).get("clean_reproduction", {})
    if clean_input.get("receipt_sha256") != file_sha256(reproduction_path):
        raise ValidationError("WP01 clean reproduction receipt digest is invalid")
    if reproduction.get("package") != "WP01" or reproduction.get("sprint") != "S00":
        raise ValidationError("WP01 clean reproduction receipt identity is invalid")
    if reproduction.get("source", {}).get("repository_commit") != evidence.get(
        "inputs", {}
    ).get("clean_reproduction", {}).get("repository_commit"):
        raise ValidationError("WP01 clean reproduction source binding is invalid")
    reproduced_results = reproduction.get("results", {})
    if any(
        (
            reproduced_results.get("focused_tests", {}).get("failed") != 0,
            reproduced_results.get("traces", 0) < 1_000,
            reproduced_results.get("unique_traces") != reproduced_results.get("traces"),
            reproduced_results.get("divergence_count") != 0,
            reproduced_results.get("invariant_failure_count") != 0,
            reproduced_results.get("positive_compact_exit_code") != 0,
            reproduced_results.get("zkir_mock_circuit_count") != 4,
            reproduced_results.get("zkir_mock_exit_code") != 0,
            reproduced_results.get("negative_disclosure_exit_code") == 0,
        )
    ):
        raise ValidationError("WP01 clean reproduction receipt does not satisfy its gate")
    digest_comparison = reproduction.get("digest_comparison", {})
    for field in (
        "semantic_artifacts_match",
        "compiler_manifest_matches_with_recorded_relative_command",
        "zkir_artifacts_match",
        "negative_control_matches",
    ):
        if digest_comparison.get(field) is not True:
            raise ValidationError(f"WP01 clean reproduction lacks {field}")

    current_path = ROOT / "evidence/wp01/current-checkout-validation.json"
    current_input = evidence.get("inputs", {}).get("current_checkout_validation", {})
    if current_input.get("receipt_sha256") != file_sha256(current_path):
        raise ValidationError("WP01 current-checkout receipt digest is invalid")
    current = load_json(current_path)
    if current.get("schema_id") != "moriarty.dev/current-checkout-validation/v1":
        raise ValidationError("WP01 current-checkout receipt schema is invalid")
    if current.get("package_id") != "WP01" or current.get("sprint_id") != "S00":
        raise ValidationError("WP01 current-checkout receipt identity is invalid")
    if current.get("fresh_archive") is not False or current.get("commit_identity") is not None:
        raise ValidationError("WP01 current-checkout evidence boundary is misstated")
    if current.get("binding") != "source-hash-bound":
        raise ValidationError("WP01 current-checkout receipt is not source-hash-bound")
    if current.get("semantic_scope_sha256") != evidence.get("semantic_scope_sha256"):
        raise ValidationError("WP01 current-checkout scope binding is invalid")
    if current.get("semantic_scope_version") != evidence.get("semantic_scope_version"):
        raise ValidationError("WP01 current-checkout scope version is invalid")
    if current.get("semantic_scope_index_sha256") != evidence.get(
        "semantic_scope_index_sha256"
    ):
        raise ValidationError("WP01 current-checkout scope index binding is invalid")
    receipt_unhashed = dict(current)
    claimed_receipt_sha = receipt_unhashed.pop("receipt_sha256", None)
    if claimed_receipt_sha != canonical_sha256(receipt_unhashed):
        raise ValidationError("WP01 current-checkout receipt self-hash is invalid")
    source_inputs = current.get("source_inputs")
    if not isinstance(source_inputs, list) or not source_inputs:
        raise ValidationError("WP01 current-checkout receipt has no source inputs")
    source_paths = [source.get("path") for source in source_inputs]
    if len(source_paths) != len(set(source_paths)):
        raise ValidationError("WP01 current-checkout source inputs are not unique")
    for source in source_inputs:
        source_path = ROOT / source["path"]
        if (
            not source_path.resolve().is_relative_to(ROOT.resolve())
            or not source_path.is_file()
            or file_sha256(source_path) != source.get("sha256")
        ):
            raise ValidationError(
                f"WP01 current-checkout source digest mismatch: {source.get('path')}"
            )
    commands = current.get("commands")
    if not isinstance(commands, list) or not commands:
        raise ValidationError("WP01 current-checkout receipt has no commands")
    if any(command.get("exit_code") != 0 for command in commands):
        raise ValidationError("WP01 current-checkout command failed")
    command_text = "\n".join(command.get("command", "") for command in commands)
    for required_test in (
        "test_generated_compact_disclosures_match_the_manifest",
        "test_compiler_metadata_matches_the_e00_manifest",
    ):
        if required_test not in command_text:
            raise ValidationError(f"WP01 current-checkout receipt lacks {required_test}")

    from moriarty.evidence import validate_e00_evidence

    experiment = ROOT / "experiments/moriarty-core-swap"
    try:
        validate_e00_evidence(
            load_json(experiment / "translation-certificate.json"),
            load_json(experiment / "toolchain-results.json"),
            artifact_directory=experiment,
        )
    except SystemExit as error:
        raise ValidationError(f"WP01 deterministic evidence check failed: {error}") from error


def validate_package(
    manifest: dict[str, Any], packages: list[dict[str, Any]], package_id: str
) -> dict[str, Any]:
    package = next((item for item in packages if item["id"] == package_id), None)
    if package is None:
        raise ValidationError(f"unknown package: {package_id}")
    if package_id != "WP01":
        raise ValidationError(f"{package_id} package gate validator is not implemented")

    current_scope_index = load_json(ROOT / manifest["semantic_scope_index"])

    missing_dependencies: list[str] = []
    for dependency in package["dependencies"]:
        dependency_manifest = ROOT / f"evidence/{dependency.lower()}/evidence-manifest.json"
        if not dependency_manifest.is_file():
            missing_dependencies.append(str(dependency_manifest.relative_to(ROOT)))
            continue
        validate_evidence_manifest(
            load_json(dependency_manifest), dependency, current_scope_index
        )
    if missing_dependencies:
        raise ValidationError(
            f"{package_id} dependencies lack evidence manifests: {missing_dependencies}"
        )

    missing_outputs: list[str] = []
    output_digests: dict[str, str] = {}
    for output in package["outputs"]:
        path = ROOT / output
        if not path.is_file():
            missing_outputs.append(output)
            continue
        output_digests[output] = file_sha256(path)
        if path.suffix == ".json":
            load_json(path)
    if missing_outputs:
        raise ValidationError(f"{package_id} missing declared outputs: {missing_outputs}")

    evidence_manifest = ROOT / package["artifact_root"] / "evidence-manifest.json"
    if not evidence_manifest.is_file():
        raise ValidationError(f"{package_id} evidence manifest is missing")
    evidence = load_json(evidence_manifest)
    validate_evidence_manifest(evidence, package_id, current_scope_index)
    if evidence.get("sprint_id") != package["sprint"]:
        raise ValidationError(f"{package_id} evidence sprint does not match its instruction")
    validate_wp01_gate(evidence)

    return {
        "schema_version": 1,
        "package": package_id,
        "status": "recomputed-package-gate-passed",
        "semantic_scope_index_sha256": file_sha256(
            ROOT / manifest["semantic_scope_index"]
        ),
        "output_sha256": output_digests,
        "limitation": "This validator recomputes the implemented WP01 gate. Other package gates fail closed until their validators exist.",
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--package")
    parser.add_argument("--instructions-only", action="store_true")
    args = parser.parse_args()
    if args.instructions_only == bool(args.package):
        parser.error("select exactly one of --instructions-only or --package")
    return args


def main() -> int:
    args = parse_args()
    try:
        manifest = load_json(ROOT / args.manifest)
        packages = validate_instruction_manifest(manifest)
        result: dict[str, Any]
        if args.instructions_only:
            statistics = validate_sdk_contracts()
            result = {
                "schema_version": 1,
                "status": "valid-instructions",
                "package_count": len(packages),
                "sdk_component_count": statistics["components"],
                "sdk_data_contract_count": statistics["data_contracts"],
            }
        else:
            result = validate_package(manifest, packages, args.package)
    except ValidationError as error:
        print(json.dumps({"status": "invalid", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
