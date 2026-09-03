from __future__ import annotations

import hashlib
import importlib.util
import json
import re
import subprocess
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator


ROOT = Path(__file__).parents[1]


def load_json(relative_path: str) -> dict[str, object]:
    return json.loads((ROOT / relative_path).read_text(encoding="utf-8"))


def canonical_sha256(value: object) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(encoded.encode("utf-8")).hexdigest()


def load_validator_module():
    module_path = ROOT / "scripts/validate_sprint_evidence.py"
    spec = importlib.util.spec_from_file_location("sprint_evidence_validator", module_path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_work_package_manifest_is_complete_and_acyclic() -> None:
    manifest = load_json("openspec/work-packages.json")
    packages = manifest["packages"]
    assert isinstance(packages, list)
    assert [package["id"] for package in packages] == [
        f"WP{index:02d}" for index in range(1, 13)
    ]

    seen: set[str] = set()
    for package in packages:
        package_id = package["id"]
        assert all(dependency in seen for dependency in package["dependencies"])
        assert package["artifact_root"].startswith(f"evidence/{package_id.lower()}/")
        assert package["outputs"]
        assert package["validator_command"].startswith("uv run python scripts/")
        assert package["gate"]
        assert package["failure_outcomes"]
        assert package["failure_selection_rule"]
        seen.add(package_id)


def test_active_openspec_has_no_calendar_completion_language() -> None:
    paths = [ROOT / "openspec/config.yaml", ROOT / "openspec/WORK-PACKAGES.md"]
    paths.extend((ROOT / "openspec/changes").glob("wp*/**/*"))
    text = "\n".join(
        path.read_text(encoding="utf-8") for path in paths if path.is_file()
    ).lower()
    assert "day-90" not in text
    assert "days 1-" not in text


def test_semantic_scope_index_matches_immutable_snapshot() -> None:
    index = load_json("evidence/semantic-scope/index.json")
    for entry in index["history"]:
        historical_path = ROOT / entry["path"]
        assert hashlib.sha256(historical_path.read_bytes()).hexdigest() == entry["sha256"]
    snapshot_path = ROOT / str(index["current_path"])
    snapshot_bytes = snapshot_path.read_bytes()
    assert hashlib.sha256(snapshot_bytes).hexdigest() == index["current_sha256"]

    snapshot = json.loads(snapshot_bytes)
    assert re.fullmatch(r"0\.\d+\.\d+(?:-[a-z0-9.]+)?", snapshot["scope_version"])
    assert snapshot["scope_version"] == index["current_version"]
    assert snapshot["core"]["constructors"]
    assert snapshot["core"]["actions"]
    assert snapshot["proof_obligations"]
    assert snapshot["evidence"]
    for key in ("surface_only", "backend_only", "external", "deferred", "rejected"):
        assert key in snapshot


def test_sdk_inventory_contracts_every_named_component() -> None:
    inventory = load_json(
        "openspec/changes/wp09-complete-development-sdk/sdk-component-inventory.json"
    )
    components = inventory["components"]
    assert isinstance(components, list)
    assert len(components) == 65
    ids = [component["id"] for component in components]
    assert len(ids) == len(set(ids))
    assert {"backend-validator", "coin-selector", "prover-client", "proof-parameter-verifier"} <= set(ids)
    schema = load_json("schemas/sdk/component-contract-v1.json")
    validator = Draft202012Validator(schema)

    required = {
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
    }
    for component in components:
        validator.validate(component)
        assert set(component) == required
        assert all(component[key] for key in required - {"trust_roots"})
        assert isinstance(component["trust_roots"], list)
        spec_path = (
            ROOT
            / "openspec/changes/wp09-complete-development-sdk"
            / component["specification"]
        )
        assert spec_path.is_file()


def test_sdk_wire_contracts_cover_the_complete_safety_path() -> None:
    data_inventory = load_json(
        "openspec/changes/wp09-complete-development-sdk/sdk-data-contracts.json"
    )
    contracts = data_inventory["contracts"]
    assert isinstance(contracts, list)
    assert len(contracts) == 28
    names = [contract["name"] for contract in contracts]
    schema_ids = [contract["schema_id"] for contract in contracts]
    assert len(names) == len(set(names))
    assert len(schema_ids) == len(set(schema_ids))
    assert {
        "CoreArtifact",
        "ResourceCertificate",
        "VisibilityManifest",
        "TranslationCertificate",
        "UserIntent",
        "ChainStateEvidence",
        "TransactionPlan",
        "ProofParameterSet",
        "ProofReceipt",
        "VerificationResult",
        "SigningRequest",
        "EvidenceManifest",
    } <= set(names)
    schema = load_json("schemas/sdk/data-contract-v1.json")
    validator = Draft202012Validator(schema)
    required = {
        "name",
        "schema_id",
        "required_fields",
        "canonical_encoding",
        "producer",
        "consumers",
        "validation",
        "unknown_fields",
        "privacy",
    }
    for contract in contracts:
        validator.validate(contract)
        assert set(contract) == required
        assert all(contract[field] for field in required)

    component_ids = {
        component["id"]
        for component in load_json(
            "openspec/changes/wp09-complete-development-sdk/sdk-component-inventory.json"
        )["components"]
    }
    for contract in contracts:
        for reference in [contract["producer"], *contract["consumers"]]:
            assert reference in component_ids or re.fullmatch(
                r"external:[a-z][a-z0-9-]*", reference
            )

    component_inventory = load_json(
        "openspec/changes/wp09-complete-development-sdk/sdk-component-inventory.json"
    )
    for inventory in (component_inventory, data_inventory):
        schema_path = ROOT / inventory["contract_schema"]
        assert schema_path.is_file()
        load_json(str(schema_path.relative_to(ROOT)))

    index = load_json("evidence/wp09/sdk-contract-index.json")
    for section in ("component_inventory", "data_contract_inventory"):
        artifact = index[section]
        path = ROOT / artifact["path"]
        assert hashlib.sha256(path.read_bytes()).hexdigest() == artifact["sha256"]
        schema_path = ROOT / artifact["schema"]
        assert hashlib.sha256(schema_path.read_bytes()).hexdigest() == artifact["schema_sha256"]
    for spec in index["capability_specs"]:
        path = ROOT / spec["path"]
        assert hashlib.sha256(path.read_bytes()).hexdigest() == spec["sha256"]
    assert index["statistics"] == {
        "components": 65,
        "data_contracts": 28,
        "capability_specs": 6,
        "minimum_safety_spine": 17,
        "complete_sdk_specification": 48,
        "unique_component_ids": 65,
        "unique_data_contract_names": 28,
        "unique_data_contract_schema_ids": 28,
        "unresolved_component_references": 0,
    }


def test_sdk_index_rejects_stale_scope_and_legacy_counts() -> None:
    module = load_validator_module()
    index = load_json("evidence/wp09/sdk-contract-index.json")
    for field, value in (
        ("semantic_scope_sha256", "0" * 64),
        ("component_inventory.components", 64),
        ("data_contract_inventory.unique_identifiers", 27),
    ):
        changed = json.loads(json.dumps(index))
        target = changed
        parts = field.split(".")
        for part in parts[:-1]:
            target = target[part]
        target[parts[-1]] = value
        with pytest.raises(module.ValidationError, match="SDK contract index"):
            module.validate_sdk_contracts(changed)

    changed = json.loads(json.dumps(index))
    changed["capability_specs"][0]["id"] = "unbound-specification"
    with pytest.raises(module.ValidationError, match="SDK contract index"):
        module.validate_sdk_contracts(changed)

    changed = json.loads(json.dumps(index))
    changed["gate_state"]["wp09_gate_passed"] = True
    with pytest.raises(module.ValidationError, match="SDK contract index"):
        module.validate_sdk_contracts(changed)

    changed = json.loads(json.dumps(index))
    changed["limitations"] = ["WP09 is fully implemented and audited."]
    with pytest.raises(module.ValidationError, match="SDK contract index"):
        module.validate_sdk_contracts(changed)

    data_contracts = load_json(
        "openspec/changes/wp09-complete-development-sdk/sdk-data-contracts.json"
    )
    changed_contracts = json.loads(json.dumps(data_contracts))
    changed_contracts["contracts"][0]["consumers"][0] = "external:"
    with pytest.raises(module.ValidationError, match="references are unresolved"):
        module.validate_sdk_contracts(data_contracts_override=changed_contracts)


def test_legacy_regression_manifest_names_thirteen_vectors() -> None:
    manifest = load_json(
        "openspec/changes/wp08-72-row-coverage/legacy-regressions.json"
    )
    regressions = manifest["regressions"]
    assert len(regressions) == 13
    assert len({item["id"] for item in regressions}) == 13
    for item in regressions:
        assert item["source_locator"]
        assert item["vector_path"]
        assert item["expected_trace"]


def test_precommitted_scorecards_have_complete_weights_and_ties() -> None:
    assurance = load_json(
        "openspec/changes/wp05-normative-assurance/assurance-scorecard.json"
    )
    terminal = load_json(
        "openspec/changes/wp12-council-decision/decision-scorecard.json"
    )
    assert sum(criterion["weight"] for criterion in assurance["criteria"]) == 100
    assert assurance["viability_threshold"]
    assert assurance["tie_rule"]
    assert assurance["maintainer_evidence"]
    assert len(assurance["prototype_obligations"]) == 3
    assert sum(criterion["weight"] for criterion in terminal["criteria"]) == 100
    assert terminal["eligibility"]
    assert terminal["selection_order"]
    assert terminal["tie_rule"]
    assert terminal["signing"]


def test_sprint_evidence_validator_accepts_instruction_manifest() -> None:
    result = subprocess.run(
        [
            "uv",
            "run",
            "python",
            "scripts/validate_sprint_evidence.py",
            "--manifest",
            "openspec/work-packages.json",
            "--instructions-only",
        ],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, result.stdout + result.stderr


def test_sprint_evidence_validator_rejects_an_internal_digest_mismatch() -> None:
    module = load_validator_module()
    scope_index = load_json("evidence/semantic-scope/index.json")
    valid_path = ROOT / "experiments/moriarty-core-swap/artifact-manifest.json"
    evidence = {
        "schema_id": "moriarty.dev/evidence-manifest/v1",
        "package_id": "WP01",
        "sprint_id": "S00",
        "semantic_scope_version": scope_index["current_version"],
        "semantic_scope_sha256": scope_index["current_sha256"],
        "semantic_scope_index_sha256": hashlib.sha256(
            (ROOT / "evidence/semantic-scope/index.json").read_bytes()
        ).hexdigest(),
        "inputs": {"source": "test"},
        "outputs": [
            {
                "path": str(valid_path.relative_to(ROOT)),
                "sha256": hashlib.sha256(valid_path.read_bytes()).hexdigest(),
            }
        ],
        "commands": ["test command"],
        "environment": {"kind": "test"},
        "status": "passed-at-S3",
        "gate_passed": True,
        "gate_results": {
            "fresh_environment": True,
            "semantic_artifact_digests_match": True,
            "compiler_artifact_digests_match_for_recorded_command": True,
            "trace_divergence_count": 0,
            "invariant_failure_count": 0,
            "third_party_disclosure_negative_rejected": True,
            "moriarty_missing_disclosure_rejected": True,
            "moriarty_additional_disclosure_rejected": True,
            "unbounded_witness_path": False,
        },
        "limitations": ["test-only fixture"],
    }
    evidence["manifest_sha256"] = canonical_sha256(evidence)

    with pytest.raises(module.ValidationError, match="declared evidence digest"):
        changed = json.loads(json.dumps(evidence))
        changed["outputs"][0]["sha256"] = "0" * 64
        changed["manifest_sha256"] = canonical_sha256(
            {key: value for key, value in changed.items() if key != "manifest_sha256"}
        )
        module.validate_evidence_manifest(changed, "WP01", scope_index)

    with pytest.raises(module.ValidationError, match="gate did not pass"):
        changed = json.loads(json.dumps(evidence))
        changed["gate_passed"] = False
        changed["manifest_sha256"] = canonical_sha256(
            {key: value for key, value in changed.items() if key != "manifest_sha256"}
        )
        module.validate_evidence_manifest(changed, "WP01", scope_index)


def test_sprint_evidence_manifest_is_bound_to_package_scope_and_self_hash() -> None:
    module = load_validator_module()
    scope_index = load_json("evidence/semantic-scope/index.json")
    evidence = load_json("evidence/wp01/evidence-manifest.json")
    module.validate_evidence_manifest(evidence, "WP01", scope_index)

    for field, value, message in [
        ("package_id", "WP99", "package"),
        ("semantic_scope_sha256", "0" * 64, "semantic scope"),
        ("manifest_sha256", "0" * 64, "self-hash"),
    ]:
        changed = json.loads(json.dumps(evidence))
        changed[field] = value
        if field != "manifest_sha256":
            changed["manifest_sha256"] = canonical_sha256(
                {key: item for key, item in changed.items() if key != "manifest_sha256"}
            )
        with pytest.raises(module.ValidationError, match=message):
            module.validate_evidence_manifest(changed, "WP01", scope_index)

    changed = json.loads(json.dumps(evidence))
    changed["gate_results"]["trace_divergence_count"] = 1
    changed["manifest_sha256"] = canonical_sha256(
        {key: item for key, item in changed.items() if key != "manifest_sha256"}
    )
    with pytest.raises(module.ValidationError, match="closed predicate"):
        module.validate_wp01_gate(changed)


def test_sprint_evidence_validator_refuses_unimplemented_package_gates() -> None:
    module = load_validator_module()
    manifest = load_json("openspec/work-packages.json")
    packages = module.validate_instruction_manifest(manifest)
    with pytest.raises(module.ValidationError, match="not implemented"):
        module.validate_package(manifest, packages, "WP02")
