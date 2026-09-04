from __future__ import annotations

import copy
import importlib.util
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).parents[1]
PYTHON = Path(sys.executable)
SCRIPT = Path("scripts/validate_s01_intent_evidence.py")


def load_module(root: Path = ROOT):
    path = root / SCRIPT
    name = f"s01_validator_{abs(hash(path))}"
    spec = importlib.util.spec_from_file_location(name, path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def artifacts(module):
    return copy.deepcopy(module.load_artifacts())


def copied_package(tmp_path: Path) -> Path:
    root = tmp_path / "copy"
    for relative in (
        "deliverables",
        "docs/superpowers/specs",
        "evidence/s01-intent-theorem-freeze",
        "evidence/semantic-scope",
        "moriarty",
        "openspec/changes/s01-intent-theorem-freeze/specs/intent-safety",
        "schemas/intent",
        "scripts",
    ):
        source = ROOT / relative
        target = root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(source, target)
    return root


def run_cli(root: Path, *arguments: str, env: dict[str, str] | None = None):
    return subprocess.run(
        [str(PYTHON), str(root / SCRIPT), *arguments],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
        env=env,
    )


def run_recompute_api(root: Path):
    program = """
import importlib.util
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
spec = importlib.util.spec_from_file_location("s01_programmatic_validator", path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
try:
    value = module.recompute_gate()
except Exception as error:
    print(json.dumps({
        "status": "invalid",
        "error_type": type(error).__name__,
        "error": str(error),
    }, sort_keys=True))
    raise SystemExit(1)
print(json.dumps(value, sort_keys=True))
"""
    return subprocess.run(
        [str(PYTHON), "-c", program, str(root / SCRIPT)],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )


def rewrite_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def reseal_manifest(module, root: Path, manifest: dict[str, object]) -> None:
    for collection in ("inputs", "outputs"):
        for record in manifest[collection]:
            path = root / record["path"]
            if path.is_file():
                record["sha256"] = module.file_sha256(path)
    manifest.pop("manifest_sha256", None)
    manifest["manifest_sha256"] = module.canonical_sha256(manifest)
    rewrite_json(root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json", manifest)


def assert_invalid(call, pattern: str) -> None:
    module = load_module()
    with pytest.raises(module.ValidationError, match=pattern):
        call(module)


def test_recompute_gate_recomputes_all_ten_semantic_gates() -> None:
    result = load_module().recompute_gate()
    assert result == {
        "schema_version": 1,
        "package": "S01",
        "status": "recomputed-package-gate-passed",
        "gate_results": {f"S01-{index:02d}": True for index in range(1, 11)},
    }


@pytest.mark.parametrize(
    ("mutation", "pattern"),
    (
        (lambda a: a["terminology"]["terms"].pop(), "terminology"),
        (
            lambda a: a["terminology"]["terms"][1].__setitem__(
                "noun", a["terminology"]["terms"][0]["noun"]
            ),
            "terminology",
        ),
        (
            lambda a: a["terminology"]["terms"][1]["aliases"].append("objective"),
            "alias",
        ),
        (
            lambda a: a["terminology"]["terms"][0].__setitem__("definition", " "),
            "terminology",
        ),
        (
            lambda a: a["ambiguity-resolutions"]["resolutions"][0].__setitem__(
                "resolved_predicate", "actus-g19-through-g24-pass"
            ),
            "G17",
        ),
        (
            lambda a: a["terminology"]["terms"][64].__setitem__(
                "noun", "MissingAtomicityKind"
            ),
            "terminology",
        ),
    ),
)
def test_gate_01_rejects_incomplete_or_ambiguous_terminology(mutation, pattern) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match=pattern):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["lifecycle-objects"]["objects"].pop(),
        lambda a: a["lifecycle-objects"]["objects"].append(
            copy.deepcopy(a["lifecycle-objects"]["objects"][0])
        ),
        lambda a: a["lifecycle-objects"]["objects"][0].__setitem__(
            "primary_category", "invalid-category"
        ),
        lambda a: a["lifecycle-objects"]["objects"][0].__setitem__(
            "authority_boundary", " "
        ),
    ),
)
def test_gate_02_rejects_invalid_lifecycle_registry(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="lifecycle|schema"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["hard-predicates"]["predicates"].pop(),
        lambda a: a["hard-predicates"]["predicates"].append(
            copy.deepcopy(a["hard-predicates"]["predicates"][0])
        ),
        lambda a: a["hard-predicates"]["predicates"][0].__setitem__(
            "enters_authorized_at", False
        ),
        lambda a: (
            a["hard-predicates"]["predicates"][0].__setitem__("signed_binding", None),
            a["hard-predicates"]["predicates"][0].__setitem__("external_premise", None),
        ),
    ),
)
def test_gate_03_rejects_weakened_hard_predicates(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="hard predicate|schema"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["optimization-preferences"]["predicates"].pop(),
        lambda a: a["optimization-preferences"]["predicates"].append(
            copy.deepcopy(a["optimization-preferences"]["predicates"][0])
        ),
        lambda a: a["optimization-preferences"]["predicates"][0].__setitem__(
            "enters_authorized_at", True
        ),
        lambda a: a["optimization-preferences"]["predicates"][0].__setitem__(
            "ranking_precondition", "some-hard-predicates-valid"
        ),
    ),
)
def test_gate_04_rejects_invalid_preferences(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="optimization preference"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["intent-safety-judgment"]["quantifiers"].reverse(),
        lambda a: a["intent-safety-judgment"]["premises"][0].__setitem__(
            "expression", "wellTyped(P)"
        ),
        lambda a: a["intent-safety-judgment"]["premises"].append(
            copy.deepcopy(a["intent-safety-judgment"]["premises"][0])
        ),
        lambda a: a["intent-safety-judgment"]["conclusion"].__setitem__(
            "operator", "displayedAt"
        ),
        lambda a: a["intent-safety-judgment"]["required_bindings"].remove(
            "complete-effect-projection"
        ),
        lambda a: a["intent-safety-judgment"]["subsidiary_claims"].pop(),
        lambda a: a["intent-safety-judgment"].__setitem__(
            "proof_status", "mechanized"
        ),
        lambda a: a["intent-safety-judgment"].__setitem__(
            "architecture_binding", "agreement-core"
        ),
        lambda a: a["intent-safety-judgment"]["exclusions"].remove(
            "ledger-correspondence"
        ),
    ),
)
def test_gate_05_rejects_changed_theorem(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="theorem|judgment|subsidiary"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["assumption-registry"]["assumptions"].pop(),
        lambda a: a["assumption-registry"]["assumptions"].append(
            copy.deepcopy(a["assumption-registry"]["assumptions"][0])
        ),
        lambda a: a["assumption-registry"]["assumptions"][0].__setitem__(
            "owner", " "
        ),
        lambda a: a["assumption-registry"]["assumptions"][0].__setitem__(
            "freshness_rule", " "
        ),
        lambda a: a["assumption-registry"]["assumptions"][1].update(
            {"verification_status": "verified", "observable": False}
        ),
    ),
)
def test_gate_06_rejects_invalid_assumptions(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="assumption|schema"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["observation-model"]["actors"].pop(),
        lambda a: a["observation-model"]["actors"].append(
            copy.deepcopy(a["observation-model"]["actors"][0])
        ),
        lambda a: a["observation-model"]["actors"][0].__setitem__(
            "observation_scope", " "
        ),
        lambda a: a["observation-model"]["event_fields"].pop(),
        lambda a: a["observation-model"]["event_fields"][0].__setitem__(
            "visibility_rule", " "
        ),
        lambda a: a["observation-model"].__setitem__(
            "authorization_effect_projection_complete", False
        ),
        lambda a: a["observation-model"]["authorization_projection"].__setitem__(
            "retains_complete_effect_set", False
        ),
        lambda a: a["observation-model"]["authorization_projection"].update(
            {
                "purpose": "public private fee refund change failed-path",
                "authority_rule": "every hard predicate",
            }
        ),
        lambda a: a["observation-model"]["signing_profiles"].pop(),
        lambda a: a["observation-model"]["signing_profiles"][0].__setitem__(
            "implementation_status", "implemented"
        ),
    ),
)
def test_gate_07_rejects_incomplete_observation_model(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="observation|projection|signing|schema"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["atomic-swap-extra-effect"]["baseline_effects"][0].__setitem__(
            "quantity", 9
        ),
        lambda a: (
            a["atomic-swap-extra-effect"]["baseline_effects"][0].__setitem__(
                "quantity", 9
            ),
            a["atomic-swap-extra-effect"]["policy"]["allowed"][0].__setitem__(
                "quantity", 9
            ),
            a["atomic-swap-extra-effect"]["policy"]["required"][0].__setitem__(
                "quantity", 9
            ),
            a["atomic-swap-extra-effect"]["baseline_expected"].update(
                {"valid": True, "reason": "VALID", "signing_request_permitted": False}
            ),
        ),
        lambda a: a["atomic-swap-extra-effect"]["baseline_expected"].__setitem__(
            "signing_request_permitted", True
        ),
        lambda a: a["atomic-swap-extra-effect"]["baseline_effects"].clear(),
        lambda a: a["atomic-swap-extra-effect"].pop("resolution"),
        lambda a: a["atomic-swap-extra-effect"].pop("scope"),
    ),
)
def test_gate_08_rejects_noncanonical_or_unavailable_baseline(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="atomic-swap|baseline|effect evidence"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a["atomic-swap-extra-effect"]["mutant_effects"][-1].__setitem__(
            "destination", "eve"
        ),
        lambda a: a["atomic-swap-extra-effect"]["mutant_effects"].pop(),
        lambda a: a["atomic-swap-extra-effect"]["mutant_expected"].__setitem__(
            "reason", "VALID"
        ),
        lambda a: a["atomic-swap-extra-effect"]["mutant_expected"].__setitem__(
            "signing_request_permitted", True
        ),
        lambda a: a["atomic-swap-extra-effect"]["policy"]["allowed"].append(
            copy.deepcopy(a["atomic-swap-extra-effect"]["mutant_effects"][-1])
        ),
    ),
)
def test_gate_09_rejects_wrong_mutant_policy_or_restoration(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="mutant|policy|restored"):
        module.recompute_gate(artifact_overrides=changed)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda s: s.__setitem__("current_version", "0.0.0-e00.3"),
        lambda s: s.__setitem__("current_sha256", "0" * 64),
        lambda s: s.__setitem__("current_path", ""),
        lambda s: s.__setitem__("current_path", "../semantic-scope/moriarty-core-0.0.0-e00.2.json"),
    ),
)
def test_gate_10_rejects_changed_or_noncanonical_scope(mutation) -> None:
    module = load_module()
    scope = module.load_json(ROOT / "evidence/semantic-scope/index.json")
    mutation(scope)
    with pytest.raises(module.ValidationError, match="semantic scope"):
        module.recompute_gate(scope_override=scope)


def test_empty_overrides_do_not_fall_back_to_disk() -> None:
    module = load_module()
    with pytest.raises(module.ValidationError, match="artifact set"):
        module.recompute_gate(artifact_overrides={})
    with pytest.raises(module.ValidationError, match="semantic scope"):
        module.recompute_gate(scope_override={})


def test_scope_override_must_match_the_pinned_on_disk_index() -> None:
    module = load_module()
    scope = module.load_json(ROOT / "evidence/semantic-scope/index.json")
    scope["unreviewed_metadata"] = "must not be ignored"
    with pytest.raises(module.ValidationError, match="semantic scope"):
        module.recompute_gate(scope_override=scope)


@pytest.mark.parametrize(
    "mutation",
    (
        lambda a: a.pop("terminology"),
        lambda a: a.__setitem__("unexpected-artifact", {}),
        lambda a: a["terminology"].__setitem__("unknown", True),
        lambda a: a["terminology"]["terms"][0].__setitem__("definition", float("nan")),
        lambda a: a["terminology"].__setitem__("artifact_kind", "wrong-kind"),
        lambda a: a["terminology"].__setitem__("scope_version", "wrong-scope"),
    ),
)
def test_artifact_set_schema_and_identity_fail_closed(mutation) -> None:
    module = load_module()
    changed = artifacts(module)
    mutation(changed)
    with pytest.raises(module.ValidationError, match="artifact|schema|finite"):
        module.recompute_gate(artifact_overrides=changed)


def test_json_loader_rejects_malformed_duplicate_and_nonfinite_json(tmp_path: Path) -> None:
    module = load_module()
    cases = (
        "{",
        '{"same": 1, "same": 2}',
        '{"number": NaN}',
        "[]",
    )
    for index, payload in enumerate(cases):
        path = tmp_path / f"bad-{index}.json"
        path.write_text(payload, encoding="utf-8")
        with pytest.raises(module.ValidationError, match="invalid JSON|expected JSON object"):
            module.load_json(path)


def test_write_mode_is_explicit_and_default_mode_requires_receipts(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    (root / "evidence/s01-intent-theorem-freeze/validation-report.json").unlink(missing_ok=True)
    (root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json").unlink(missing_ok=True)

    missing = run_cli(root)
    assert missing.returncode != 0
    assert json.loads(missing.stdout)["status"] == "invalid"

    written = run_cli(root, "--write-evidence")
    assert written.returncode == 0, written.stdout + written.stderr
    assert json.loads(written.stdout)["status"] == "recomputed-package-gate-passed"

    checked = run_cli(root)
    assert checked.returncode == 0, checked.stdout + checked.stderr


@pytest.mark.parametrize(
    "mutation",
    ("missing-report", "stale-report", "missing-manifest", "stale-manifest", "stale-validator"),
)
def test_recompute_gate_is_closed_over_published_receipts(
    tmp_path: Path, mutation: str
) -> None:
    root = copied_package(tmp_path)
    report_path = root / "evidence/s01-intent-theorem-freeze/validation-report.json"
    manifest_path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    if mutation == "missing-report":
        report_path.unlink()
    elif mutation == "stale-report":
        report = json.loads(report_path.read_text())
        report["gate_results"]["S01-10"] = False
        rewrite_json(report_path, report)
    elif mutation == "missing-manifest":
        manifest_path.unlink()
    elif mutation == "stale-manifest":
        manifest = json.loads(manifest_path.read_text())
        manifest["manifest_sha256"] = "0" * 64
        rewrite_json(manifest_path, manifest)
    else:
        validator = root / SCRIPT
        validator.write_text(validator.read_text() + "\n# stale validator\n", encoding="utf-8")

    result = run_recompute_api(root)

    assert result.returncode != 0
    parsed = json.loads(result.stdout)
    assert parsed["status"] == "invalid"
    assert parsed["error_type"] == "ValidationError"


@pytest.mark.parametrize("mutation", ("missing-definition", "unresolved-reference"))
def test_programmatic_schema_reference_failures_are_validation_errors(
    tmp_path: Path, mutation: str
) -> None:
    root = copied_package(tmp_path)
    schema_path = root / "schemas/intent/s01-artifacts-v1.json"
    schema = json.loads(schema_path.read_text())
    if mutation == "missing-definition":
        schema["$defs"].pop("terminology")
    else:
        schema["$defs"]["terminology"] = {"$ref": "#/$defs/not-present"}
    rewrite_json(schema_path, schema)

    result = run_recompute_api(root)

    assert result.returncode != 0
    parsed = json.loads(result.stdout)
    assert parsed["error_type"] == "ValidationError"
    assert "schema" in parsed["error"]


@pytest.mark.parametrize("keyword", ("$ref", "$dynamicRef"))
def test_external_schema_references_are_rejected_before_resolution(
    tmp_path: Path, keyword: str
) -> None:
    root = copied_package(tmp_path)
    schema_path = root / "schemas/intent/s01-artifacts-v1.json"
    schema = json.loads(schema_path.read_text())
    schema["$defs"]["terminology"] = {
        keyword: "https://schemas.invalid/external.json"
    }
    rewrite_json(schema_path, schema)

    result = run_recompute_api(root)

    assert result.returncode != 0
    parsed = json.loads(result.stdout)
    assert parsed["error_type"] == "ValidationError"
    assert "unsupported schema reference" in parsed["error"]
    assert "reference resolution failed" not in parsed["error"]


def test_failed_write_mode_does_not_replace_existing_receipts(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    report_path = root / "evidence/s01-intent-theorem-freeze/validation-report.json"
    manifest_path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    before = (report_path.read_bytes(), manifest_path.read_bytes())
    terminology = json.loads((root / "evidence/s01-intent-theorem-freeze/terminology.json").read_text())
    terminology["terms"].pop()
    rewrite_json(root / "evidence/s01-intent-theorem-freeze/terminology.json", terminology)

    result = run_cli(root, "--write-evidence")
    assert result.returncode != 0
    assert (report_path.read_bytes(), manifest_path.read_bytes()) == before


def test_direct_cli_uses_the_addressed_worktree_package(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    hostile = tmp_path / "hostile"
    (hostile / "moriarty").mkdir(parents=True)
    (hostile / "moriarty/__init__.py").write_text("", encoding="utf-8")
    (hostile / "moriarty/intent.py").write_text("raise RuntimeError('wrong package')\n", encoding="utf-8")
    env = os.environ.copy()
    env["PYTHONPATH"] = os.pathsep.join((str(hostile), str(root)))
    result = run_cli(root, env=env)
    assert result.returncode == 0, result.stdout + result.stderr


def test_default_cli_reports_stale_report_and_manifest(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    report_path = root / "evidence/s01-intent-theorem-freeze/validation-report.json"
    report = json.loads(report_path.read_text())
    report["gate_results"]["S01-10"] = False
    rewrite_json(report_path, report)
    result = run_cli(root)
    assert result.returncode != 0
    assert "validation report" in json.loads(result.stdout)["error"]

    root = copied_package(tmp_path / "second")
    manifest_path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    manifest["manifest_sha256"] = "0" * 64
    rewrite_json(manifest_path, manifest)
    result = run_cli(root)
    assert result.returncode != 0
    assert "self-hash" in json.loads(result.stdout)["error"]


@pytest.mark.parametrize("attack", ("missing", "extra", "duplicate", "escaping"))
def test_manifest_requires_exact_non_escaping_path_closure(tmp_path: Path, attack: str) -> None:
    root = copied_package(tmp_path)
    module = load_module(root)
    path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    manifest = json.loads(path.read_text())
    if attack == "missing":
        manifest["outputs"].pop()
    elif attack == "extra":
        manifest["outputs"].append(
            {"path": "moriarty/__init__.py", "sha256": "0" * 64, "role": "extra"}
        )
    elif attack == "duplicate":
        manifest["inputs"].append(copy.deepcopy(manifest["inputs"][0]))
    else:
        manifest["outputs"][0]["path"] = "../outside.json"
    reseal_manifest(module, root, manifest)

    result = run_cli(root)
    assert result.returncode != 0
    assert "manifest" in json.loads(result.stdout)["error"]


def test_symlink_escape_is_rejected(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    module = load_module(root)
    manifest_path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    victim = root / manifest["outputs"][0]["path"]
    outside = tmp_path / "outside"
    outside.write_bytes(victim.read_bytes())
    victim.unlink()
    victim.symlink_to(outside)
    reseal_manifest(module, root, manifest)

    result = run_cli(root)
    assert result.returncode != 0
    assert "path" in json.loads(result.stdout)["error"]


@pytest.mark.parametrize(
    "relative",
    (
        "evidence/s01-intent-theorem-freeze/terminology.json",
        "schemas/intent/s01-artifacts-v1.json",
        "evidence/semantic-scope/index.json",
        "evidence/s01-intent-theorem-freeze/validation-report.json",
        "evidence/s01-intent-theorem-freeze/evidence-manifest.json",
    ),
)
def test_package_paths_are_closed_before_json_is_read(tmp_path: Path, relative: str) -> None:
    root = copied_package(tmp_path)
    outside = tmp_path / "outside-malformed.json"
    outside.write_text("{outside-content-must-not-be-parsed", encoding="utf-8")
    target = root / relative
    target.unlink()
    target.symlink_to(outside)

    result = run_cli(root)

    assert result.returncode != 0
    error = json.loads(result.stdout)["error"]
    assert "escapes or is substituted" in error
    assert "invalid JSON" not in error


def test_coordinated_immutable_source_and_manifest_edit_fails_pin(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    module = load_module(root)
    core = root / "moriarty/core.py"
    core.write_text(core.read_text() + "\n# coordinated edit\n", encoding="utf-8")
    manifest_path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    reseal_manifest(module, root, manifest)

    result = run_cli(root)
    assert result.returncode != 0
    assert "immutable input" in json.loads(result.stdout)["error"]


def test_modified_local_source_is_rejected_before_it_can_execute(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    sentinel = tmp_path / "source-executed"
    core = root / "moriarty/core.py"
    core.write_text(
        core.read_text()
        + f"\nfrom pathlib import Path\nPath({str(sentinel)!r}).write_text('executed')\n",
        encoding="utf-8",
    )

    result = run_cli(root)

    assert result.returncode != 0
    assert "immutable input digest" in json.loads(result.stdout)["error"]
    assert not sentinel.exists()


def test_coordinated_normative_definition_and_manifest_edit_fails_pin(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    module = load_module(root)
    term_path = root / "evidence/s01-intent-theorem-freeze/terminology.json"
    terminology = json.loads(term_path.read_text())
    terminology["terms"][0]["definition"] += " Altered but schema-valid."
    rewrite_json(term_path, terminology)
    manifest_path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    reseal_manifest(module, root, manifest)

    result = run_cli(root)
    assert result.returncode != 0
    assert "reviewed normative" in json.loads(result.stdout)["error"]


def test_coordinated_weakened_schema_and_manifest_edit_fails_pin(tmp_path: Path) -> None:
    root = copied_package(tmp_path)
    module = load_module(root)
    schema_path = root / "schemas/intent/s01-artifacts-v1.json"
    schema = json.loads(schema_path.read_text())
    schema["$defs"]["terminology"]["additionalProperties"] = True
    rewrite_json(schema_path, schema)
    manifest_path = root / "evidence/s01-intent-theorem-freeze/evidence-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    reseal_manifest(module, root, manifest)

    result = run_cli(root)
    assert result.returncode != 0
    assert "reviewed normative" in json.loads(result.stdout)["error"]


def test_stale_validator_or_verifier_digest_is_rejected(tmp_path: Path) -> None:
    for relative in (SCRIPT, Path("moriarty/intent.py")):
        root = copied_package(tmp_path / relative.stem)
        path = root / relative
        path.write_text(path.read_text() + "\n# stale receipt mutation\n", encoding="utf-8")
        result = run_cli(root)
        assert result.returncode != 0
        assert "digest" in json.loads(result.stdout)["error"]


def test_cli_returns_stable_json_for_malformed_and_duplicate_artifact_json(tmp_path: Path) -> None:
    for name, payload in (
        ("malformed", "{"),
        (
            "duplicate",
            '{"schema_id":"x","schema_id":"y","artifact_kind":"intent-terminology-registry","scope_version":"0.0.0-e00.2","terms":[]}',
        ),
    ):
        root = copied_package(tmp_path / name)
        path = root / "evidence/s01-intent-theorem-freeze/terminology.json"
        path.write_text(payload, encoding="utf-8")
        result = run_cli(root)
        assert result.returncode != 0
        parsed = json.loads(result.stdout)
        assert parsed["status"] == "invalid"
        assert "invalid JSON" in parsed["error"]
        assert "Traceback" not in result.stderr


def test_cli_rejects_unknown_arguments_as_json() -> None:
    result = run_cli(ROOT, "--unknown")
    assert result.returncode != 0
    parsed = json.loads(result.stdout)
    assert parsed["status"] == "invalid"
    assert "argument" in parsed["error"]
