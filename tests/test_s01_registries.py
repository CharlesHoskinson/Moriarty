from __future__ import annotations

import copy
import json
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator, ValidationError


ROOT = Path(__file__).parents[1]
EVIDENCE = ROOT / "evidence/s01-intent-theorem-freeze"
SCHEMA_PATH = ROOT / "schemas/intent/s01-artifacts-v1.json"


def schema() -> dict[str, object]:
    return json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))


def load(name: str) -> dict[str, object]:
    return json.loads((EVIDENCE / name).read_text(encoding="utf-8"))


FILES = {
    "terminology.json": "terminology",
    "lifecycle-objects.json": "lifecycleObjects",
    "observation-model.json": "observationModel",
    "hard-predicates.json": "predicateRegistry",
    "optimization-preferences.json": "predicateRegistry",
    "assumption-registry.json": "assumptionRegistry",
    "ambiguity-resolutions.json": "ambiguityResolutions",
}


def test_s01_registries_match_their_strict_schemas() -> None:
    definitions = schema()["$defs"]
    for name, definition in FILES.items():
        Draft202012Validator(definitions[definition]).validate(load(name))


def test_all_ten_definitions_are_self_contained_and_strict() -> None:
    definitions = schema()["$defs"]
    assert set(definitions) == {
        "terminology", "lifecycleObjects", "observationModel",
        "predicateRegistry", "assumptionRegistry", "judgment",
        "ambiguityResolutions", "planVector", "validationReport",
        "evidenceManifest",
    }
    for definition in definitions.values():
        assert "$ref" not in json.dumps(definition)
        assert definition["additionalProperties"] is False


@pytest.mark.parametrize("name,definition", FILES.items())
def test_schema_rejects_unknown_top_level_fields(name: str, definition: str) -> None:
    changed = copy.deepcopy(load(name))
    changed["unexpected"] = "must be rejected"
    with pytest.raises(ValidationError):
        Draft202012Validator(schema()["$defs"][definition]).validate(changed)


@pytest.mark.parametrize(
    "name,definition,payload",
    [
        ("terminology.json", "terminology", "terms"),
        ("lifecycle-objects.json", "lifecycleObjects", "objects"),
        ("observation-model.json", "observationModel", "event_fields"),
        ("hard-predicates.json", "predicateRegistry", "predicates"),
        ("optimization-preferences.json", "predicateRegistry", "predicates"),
        ("assumption-registry.json", "assumptionRegistry", "assumptions"),
        ("ambiguity-resolutions.json", "ambiguityResolutions", "resolutions"),
    ],
)
def test_schema_rejects_unknown_nested_fields(
    name: str, definition: str, payload: str
) -> None:
    changed = copy.deepcopy(load(name))
    changed[payload][0]["unexpected"] = "must be rejected"
    with pytest.raises(ValidationError):
        Draft202012Validator(schema()["$defs"][definition]).validate(changed)


def test_schema_rejects_wrong_types_and_missing_metadata() -> None:
    validator = Draft202012Validator(schema()["$defs"]["lifecycleObjects"])
    wrong_type = copy.deepcopy(load("lifecycle-objects.json"))
    wrong_type["objects"] = "not-an-array"
    with pytest.raises(ValidationError):
        validator.validate(wrong_type)
    missing = copy.deepcopy(load("lifecycle-objects.json"))
    del missing["scope_version"]
    with pytest.raises(ValidationError):
        validator.validate(missing)


def test_plan_vector_schema_accepts_current_falsifier_metadata() -> None:
    Draft202012Validator(schema()["$defs"]["planVector"]).validate(
        load("atomic-swap-extra-effect.json")
    )


def test_required_terminology_and_lifecycle_objects_are_distinct() -> None:
    w4 = {
        "IntentObjective", "UserIntent", "IntentDomain", "AllowedEffects",
        "ForbiddenEffects", "Validity", "SignerPolicy", "DisclosurePolicy",
        "DisclosureAcceptance", "CapabilityBudget", "FeeLedger", "StateAnchor",
        "StateCommitment", "FinalityPolicy", "FailurePolicy", "QuoteRequest",
        "Quote", "IntentAuthorization", "OrderPayload", "ResolutionSnapshot",
        "ResolvedPlan", "FillReceipt", "FulfillmentProof", "ClaimReceipt",
        "SettlementReceipt", "RefundReceipt", "CancellationReceipt",
    }
    xml_aliases = {
        "agreement", "intent", "objective", "quote", "authorization",
        "order_payload", "resolved_plan", "plan", "execution_trace", "fill",
        "fulfillment", "settlement", "partial_fill", "intent_refinement",
        "static_certificate", "zero_knowledge_proof", "verification_receipt",
        "correctness_of_intent", "actus_reference_contract",
        "actus_reference_vector_compatibility",
    }
    terms = load("terminology.json")["terms"]
    names = [item["noun"] for item in terms]
    aliases = {alias for item in terms for alias in item["aliases"]}
    assert w4 <= set(names)
    assert xml_aliases <= aliases
    assert len(names) == len(set(names))
    assert all(not item["unresolved_aliases"] for item in terms)
    objects = load("lifecycle-objects.json")["objects"]
    assert w4 == {item["name"] for item in objects}
    assert len(objects) == 27


def test_partial_fill_family_and_projections_are_explicit() -> None:
    names = {item["noun"] for item in load("terminology.json")["terms"]}
    assert {
        "DivisibleEconomicFill", "PartialOutputCompletion",
        "SameChainTransactionAtomicity", "ContiguousWalletExecution",
        "CrossDomainAllOrRefund", "EndToEndAtomicity",
        "DisplayProjection", "AuthorizationProjection",
    } <= names


def test_preferences_cannot_authorize_a_plan() -> None:
    hard = load("hard-predicates.json")
    preferences = load("optimization-preferences.json")
    assert hard["registry_kind"] == "hard"
    assert preferences["registry_kind"] == "preference"
    assert all(item["enters_authorized_at"] for item in hard["predicates"])
    assert not any(item["enters_authorized_at"] for item in preferences["predicates"])


def test_observation_model_freezes_complete_authorization_projection() -> None:
    model = load("observation-model.json")
    assert {item["name"] for item in model["event_fields"]} == {
        "channel", "actor", "domain", "settlement_level", "time",
        "state_anchor", "payload_commitment", "visibility", "public_effects",
        "private_commitments", "approved_disclosures", "metadata", "failure",
        "timing", "availability",
    }
    assert model["authorization_projection"]["retains_complete_effect_set"] is True
    assert model["declassification_rule"].startswith("Declassification changes visibility only")


def test_actus_is_not_classified_as_a_human_pilot() -> None:
    resolutions = load("ambiguity-resolutions.json")["resolutions"]
    g17 = next(item for item in resolutions if item["id"] == "AMB-S01-001")
    assert g17["source_locator"] == "release_gates/G17"
    assert g17["pilot_kind"] == "human-team"
    assert g17["actus_kind"] == "automated-benchmark"
    assert g17["resolved_predicate"] == (
        "two-human-pilots-prefer-workflow AND actus-g19-through-g24-pass"
    )
