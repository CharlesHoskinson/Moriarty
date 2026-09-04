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


def test_settlement_process_is_distinct_from_receipt_evidence() -> None:
    terms = {item["noun"]: item for item in load("terminology.json")["terms"]}
    settlement_alias_owner = next(
        item for item in terms.values() if "settlement" in item["aliases"]
    )
    receipt = terms["SettlementReceipt"]

    assert settlement_alias_owner["noun"] == "SettlementProcess"
    assert settlement_alias_owner["semantic_category"] == "process"
    assert "fills and proofs" in settlement_alias_owner["definition"]
    assert "final, spendable outcomes" in settlement_alias_owner["definition"]
    assert "authorized recovery path" in settlement_alias_owner["definition"]
    assert receipt["semantic_category"] == "evidence"
    assert receipt["aliases"] == []
    assert "settlement process" in receipt["excluded_meanings"][0].lower()


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


def test_lifecycle_metadata_names_real_pipeline_consumers() -> None:
    records = {item["name"]: item for item in load("lifecycle-objects.json")["objects"]}
    assert "quote-provider" in records["QuoteRequest"]["permitted_consumers"]
    assert "user-or-client" in records["Quote"]["permitted_consumers"]
    assert "resolver-or-solver" in records["OrderPayload"]["permitted_consumers"]
    assert "user-or-signer" in records["ResolvedPlan"]["permitted_consumers"]
    assert "wallet-or-custodian" in records["ResolvedPlan"]["permitted_consumers"]
    assert "settlement-mechanism" in records["FulfillmentProof"]["permitted_consumers"]
    assert len({tuple(item["permitted_consumers"]) for item in records.values()}) > 10


def test_w5_runtime_terms_describe_runtime_producers_and_consumers() -> None:
    terms = {item["noun"]: item for item in load("terminology.json")["terms"]}
    expected = {
        "ProofRequest": ("verifier-or-client", "prover", "command"),
        "ProofReceipt": ("prover-or-proof-verifier", "intent-verifier", "evidence"),
        "SigningRequest": ("wallet-or-client", "user-or-signer", "command"),
        "SubmissionReceipt": ("submission-interface", "runtime-or-indexer", "evidence"),
        "ResolvedPlan": ("resolver-or-solver", "wallet-or-custodian", "carrier"),
    }
    for noun, (producer, consumer, category) in expected.items():
        assert terms[noun]["producer"] == producer
        assert consumer in terms[noun]["permitted_consumers"]
        assert terms[noun]["semantic_category"] == category
        assert terms[noun]["canonical_representation_status"] == "profile-dependent"
        assert "S01-design-authority" not in terms[noun]["authority_boundary"]


def test_report_and_manifest_require_exactly_the_ten_frozen_gates() -> None:
    expected = {f"S01-{index:02d}" for index in range(1, 11)}
    definitions = schema()["$defs"]
    for name in ("validationReport", "evidenceManifest"):
        gate_schema = definitions[name]["properties"]["gate_results"]
        assert gate_schema["additionalProperties"] is False
        assert set(gate_schema["required"]) == expected
        assert set(gate_schema["properties"]) == expected


def test_report_gate_schema_rejects_missing_and_invented_gates() -> None:
    gate_schema = schema()["$defs"]["validationReport"]["properties"]["gate_results"]
    validator = Draft202012Validator(gate_schema)
    gates = {f"S01-{index:02d}": True for index in range(1, 11)}
    validator.validate(gates)
    missing = dict(gates)
    del missing["S01-10"]
    with pytest.raises(ValidationError):
        validator.validate(missing)
    invented = dict(gates)
    invented["S01-11"] = True
    with pytest.raises(ValidationError):
        validator.validate(invented)


def test_manifest_digest_records_require_a_nonblank_role() -> None:
    manifest = schema()["$defs"]["evidenceManifest"]
    for field in ("inputs", "outputs"):
        record = manifest["properties"][field]["items"]
        assert "role" in record["required"]
        validator = Draft202012Validator(record)
        validator.validate({"path": "artifact.json", "sha256": "a" * 64,
                            "role": "normative-input"})
        with pytest.raises(ValidationError):
            validator.validate({"path": "artifact.json", "sha256": "a" * 64})
        with pytest.raises(ValidationError):
            validator.validate({"path": "artifact.json", "sha256": "a" * 64,
                                "role": ""})


def test_intent_safety_judgment_has_the_frozen_shape() -> None:
    judgment = load("intent-safety-judgment.json")
    judgment_schema = schema()["$defs"]["judgment"]
    Draft202012Validator(judgment_schema).validate(judgment)
    assert judgment_schema["additionalProperties"] is False
    assert judgment_schema["properties"]["premises"]["items"]["additionalProperties"] is False
    assert judgment_schema["properties"]["conclusion"]["additionalProperties"] is False
    assert judgment["schema_id"] == "moriarty.dev/intent-safety-judgment/v1"
    assert judgment["artifact_kind"] == "intent-safety-judgment"
    assert judgment["scope_version"] == "0.0.0-e00.2"
    assert judgment["judgment_id"] == "INTENT-SAFETY"
    assert judgment["version"] == "1"
    assert judgment["architecture_binding"] == "neutral"
    assert judgment["proof_status"] == "candidate-unmechanized"
    assert judgment["quantifiers"] == [
        "S_sign", "S_exec", "I", "P", "T", "O", "A", "H", "D", "N", "L"
    ]
    assert [(item["id"], item["expression"]) for item in judgment["premises"]] == [
        ("well-typed", "wellTyped(I)"),
        ("domain-bound", "I.domain = D"),
        ("nonce-bound", "I.nonce = N"),
        ("authorization-valid", "AuthorizationValid(I.authorization, S_exec, D, N)"),
        ("snapshot-fresh", "SnapshotFresh(A.snapshot, A.querySet, A.mutability, S_exec)"),
        ("resolver-bound", "ResolverBound(A.resolverId, A.codeHash, A.implementation, A.upgradeState)"),
        ("assumptions-verified", "AssumptionsVerified(H, A.witnesses)"),
        ("refinement-chain-bound", "RefinementChainBound(I, A, P)"),
        (
            "plan-valid",
            "verifyPlan(I, S_sign, S_exec, A, P) = VerificationCertificate.valid",
        ),
        ("plan-executes", "executes(P, S_exec, A.asyncBoundary, T, O)"),
        ("settlement-predicate", "SettlementPredicate(L, I.finalityPolicy, T, O)"),
    ]
    assert judgment["conclusion"] == {
        "operator": "authorizedAt",
        "expression": "authorizedAt(I, H, L, view(I.authorizer, D, L, T, O))",
    }
    assert set(judgment["required_bindings"]) == {
        "network", "ledger", "verifying-contract", "upgrade-state", "core",
        "serializer", "compiler", "circuit", "proof-parameters",
        "resolver-identity", "resolver-code-hash", "resolver-implementation",
        "proxy-implementation", "signing-state", "execution-state",
        "state-sequence", "nonce-domain", "validity-interval", "signers",
        "approval-scope", "assets", "effects", "fees", "disclosures",
        "capabilities", "assumptions", "failure-outcomes",
        "complete-effect-projection", "signing-order-profile",
    }
    assert set(judgment["subsidiary_claims"]) == {
        "no-extra-spend", "no-diverted-change", "no-extra-mint-or-burn",
        "fee-compliance", "signer-compliance", "disclosure-compliance",
        "capability-non-escalation", "replay-rejection",
        "cancel-or-fill-exclusivity", "refund-authorization",
        "partial-fill-residual-correctness", "settlement-level-correspondence",
        "composition",
    }
    assert set(judgment["exclusions"]) == {
        "liveness", "economic-optimality", "legal-enforceability",
        "stronger-settlement-levels", "proof-system-soundness",
        "backend-correspondence", "ledger-correspondence",
    }
    assert "optimization-preference" not in json.dumps(judgment)
