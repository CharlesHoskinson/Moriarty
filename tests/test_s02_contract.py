import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CHANGE = ROOT / "openspec/changes/s02-model-comparison"
REGISTRY = ROOT / "evidence/s02-model-comparison/requirements.json"


def test_s02_requirement_vocabulary_is_closed():
    value = json.loads(REGISTRY.read_text())
    assert set(value) == {
        "schema_version", "package", "status", "prompt_sha256",
        "scope_version", "scope_sha256", "candidates", "workloads",
        "signing_profiles", "properties", "witnesses", "controls",
        "package_gates", "excluded_claims", "verification_backend",
        "evidence", "selected_candidate",
    }
    assert value["schema_version"] == 1
    assert value["package"] == "S02"
    assert value["status"] == "specified-only"
    assert value["prompt_sha256"] == "86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd"
    assert value["scope_version"] == "0.0.0-e00.2"
    assert value["scope_sha256"] == "9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a"
    assert value["candidates"] == ["A", "B", "C", "D"]
    assert value["workloads"] == ["canonical-swap", "two-installment-obligation"]
    assert value["signing_profiles"] == ["SignAfterResolve", "SignBeforeResolve"]
    assert value["properties"] == [
        "complete-signed-effects", "asset-conservation", "nonnegative-balances",
        "authorized-refunds", "nonce-replay-exclusion", "cancel-fill-exclusion",
        "residual-authority-conservation", "authority-bindings",
        "rejection-preservation", "display-is-not-authority",
        "settlement-evidence-level", "nonterminal-enabled",
    ]
    assert value["witnesses"] == [
        "settlement", "voluntary-refund", "deadline-refund",
        "deadline-input-rollback", "extra-effect-rejection",
        "first-installment-residual", "second-installment-completion",
        "cancel-wins", "fill-wins", "after-resolve-execution",
        "before-resolve-execution",
    ]
    assert value["controls"] == [
        "missing-deposit-dependency", "executed-artifact-substitution",
        "reversed-timeout-priority", "installment-replay", "residual-expansion",
        "elaboration-corruption", "bridge-corruption",
        "agreement-only-advancement", "intent-only-consumption", "stale-bridge",
        "extraction-corruption",
        "post-sign-plan-substitution", "omitted-state-verification",
        "abstraction-map-corruption",
    ]
    assert value["package_gates"] == [f"S02-{n:02}" for n in range(1, 11)]
    assert value["excluded_claims"] == [
        "unbounded-proof", "cryptographic-authenticity", "compact-correspondence",
        "ledger-execution", "actus-completeness", "human-preference",
        "production-cost", "semantic-scope-change",
    ]
    assert value["verification_backend"] == "quint-apalache"
    assert value["evidence"] == []
    assert value["selected_candidate"] is None


def test_s02_contract_is_complete_but_not_execution_evidence():
    paths = {
        ".openspec.yaml", "README.md", "proposal.md", "design.md", "tasks.md",
        "specs/architecture-comparison/spec.md",
    }
    assert {str(p.relative_to(CHANGE)) for p in CHANGE.rglob("*") if p.is_file()} == paths
    text = "\n".join((CHANGE / path).read_text() for path in sorted(paths))
    for heading in (
        "Dependencies", "Immutable inputs", "Exact outputs", "Acceptance predicates",
        "Negative controls", "Evidence manifest", "Failure outcomes", "Rollback",
        "Semantic-scope transition",
    ):
        assert heading in text
    readme = (CHANGE / "README.md").read_text()
    assert "Status: specified-only" in readme
    assert "No architecture has been selected" in readme
    assert "not passed results" in readme
    spec = (CHANGE / "specs/architecture-comparison/spec.md").read_text()
    assert spec.count("#### Scenario:") == 10
    for n in range(1, 11):
        assert f"S02-{n:02}" in spec
    assert "--backend apalache" in text
    assert "0.0.0-e00.2" in text
    assert "candidate-unmechanized" in text
    assert "277" in text
