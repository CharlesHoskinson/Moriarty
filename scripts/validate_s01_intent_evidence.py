#!/usr/bin/env python3
"""Recompute and validate the closed Moriarty S01 intent-theorem evidence."""

from __future__ import annotations

import hashlib
import importlib
import inspect
import json
import os
import platform
import sys
import tempfile
from collections.abc import Callable, Mapping, Sequence
from pathlib import Path, PurePosixPath
from typing import NoReturn


ROOT = Path(__file__).resolve().parents[1]
sys.path[:] = [entry for entry in sys.path if entry != str(ROOT)]
sys.path.insert(0, str(ROOT))

from jsonschema import Draft202012Validator  # noqa: E402
from jsonschema.exceptions import SchemaError, ValidationError as SchemaValidationError  # noqa: E402


EVIDENCE = ROOT / "evidence/s01-intent-theorem-freeze"
SCHEMA_PATH = ROOT / "schemas/intent/s01-artifacts-v1.json"
REPORT_PATH = EVIDENCE / "validation-report.json"
MANIFEST_PATH = EVIDENCE / "evidence-manifest.json"
EXPECTED_SCOPE_VERSION = "0.0.0-e00.2"
EXPECTED_SCOPE_PATH = "evidence/semantic-scope/moriarty-core-0.0.0-e00.2.json"
EXPECTED_SCOPE_SHA256 = "9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a"

ARTIFACT_DEFS = {
    "terminology": "terminology",
    "lifecycle-objects": "lifecycleObjects",
    "observation-model": "observationModel",
    "hard-predicates": "predicateRegistry",
    "optimization-preferences": "predicateRegistry",
    "assumption-registry": "assumptionRegistry",
    "intent-safety-judgment": "judgment",
    "ambiguity-resolutions": "ambiguityResolutions",
    "atomic-swap-extra-effect": "planVector",
}

ARTIFACT_IDENTITIES = {
    "terminology": ("moriarty.dev/intent-terminology-registry/v1", "intent-terminology-registry"),
    "lifecycle-objects": (
        "moriarty.dev/intent-lifecycle-object-registry/v1",
        "intent-lifecycle-object-registry",
    ),
    "observation-model": ("moriarty.dev/intent-observation-model/v1", "intent-observation-model"),
    "hard-predicates": (
        "moriarty.dev/intent-hard-predicate-registry/v1",
        "intent-hard-predicate-registry",
    ),
    "optimization-preferences": (
        "moriarty.dev/intent-optimization-preference-registry/v1",
        "intent-optimization-preference-registry",
    ),
    "assumption-registry": (
        "moriarty.dev/intent-assumption-registry/v1",
        "intent-assumption-registry",
    ),
    "intent-safety-judgment": (
        "moriarty.dev/intent-safety-judgment/v1",
        "intent-safety-judgment",
    ),
    "ambiguity-resolutions": (
        "moriarty.dev/intent-ambiguity-resolutions/v1",
        "intent-ambiguity-resolutions",
    ),
    "atomic-swap-extra-effect": ("moriarty.dev/intent-plan-vector/v1", "intent-plan-vector"),
}

REQUIRED_TERMS = (
    "IntentObjective", "UserIntent", "IntentDomain", "AllowedEffects",
    "ForbiddenEffects", "Validity", "SignerPolicy", "DisclosurePolicy",
    "DisclosureAcceptance", "CapabilityBudget", "FeeLedger", "StateAnchor",
    "StateCommitment", "FinalityPolicy", "FailurePolicy", "QuoteRequest",
    "Quote", "IntentAuthorization", "OrderPayload", "ResolutionSnapshot",
    "ResolvedPlan", "FillReceipt", "FulfillmentProof", "ClaimReceipt",
    "SettlementReceipt", "RefundReceipt", "CancellationReceipt",
    "AssumptionManifest", "AuthorizationReceipt", "CanonicalIntentBytes",
    "CircuitIdentity", "EffectSummary", "FinalityEvidence",
    "HumanReadableInterpretation", "IntentHash", "LocalProofVerification",
    "ProofReceipt", "ProofRequest", "RollbackEvidence", "SigningRequest",
    "StateEvidence", "StaticCertificate", "SubmissionReceipt",
    "TransactionVerification", "TranslationCertificate",
    "VerificationCertificate", "SigningState", "ExecutionState", "Trace",
    "Outcome", "SettlementLevel", "Projection", "DisplayProjection",
    "AuthorizationProjection", "Agreement", "PlanProposal", "PartialFillFamily",
    "IntentRefinement", "ZeroKnowledgeProof", "VerificationReceipt",
    "CorrectnessOfIntent", "ActusReferenceContract",
    "ActusReferenceVectorCompatibility", "DivisibleEconomicFill",
    "PartialOutputCompletion", "SameChainTransactionAtomicity",
    "ContiguousWalletExecution", "CrossDomainAllOrRefund", "EndToEndAtomicity",
)
REQUIRED_TERM_IDS = tuple(
    [f"TERM-W4-{index:03d}" for index in range(1, 28)]
    + [f"TERM-S01-{index:03d}" for index in range(1, 43)]
)
REQUIRED_XML_ALIASES = {
    "agreement", "intent", "objective", "quote", "authorization",
    "order_payload", "resolved_plan", "plan", "execution_trace", "fill",
    "fulfillment", "settlement", "partial_fill", "intent_refinement",
    "static_certificate", "zero_knowledge_proof", "verification_receipt",
    "correctness_of_intent", "actus_reference_contract",
    "actus_reference_vector_compatibility",
}
ATOMICITY_KINDS = {
    "DivisibleEconomicFill", "PartialOutputCompletion",
    "SameChainTransactionAtomicity", "ContiguousWalletExecution",
    "CrossDomainAllOrRefund", "EndToEndAtomicity",
}
LIFECYCLE_CATEGORIES = {
    "IntentObjective": "state", "UserIntent": "authorization",
    "IntentDomain": "state", "AllowedEffects": "authorization",
    "ForbiddenEffects": "authorization", "Validity": "authorization",
    "SignerPolicy": "authorization", "DisclosurePolicy": "authorization",
    "DisclosureAcceptance": "evidence", "CapabilityBudget": "authorization",
    "FeeLedger": "state", "StateAnchor": "state", "StateCommitment": "evidence",
    "FinalityPolicy": "authorization", "FailurePolicy": "authorization",
    "QuoteRequest": "command", "Quote": "carrier",
    "IntentAuthorization": "authorization", "OrderPayload": "carrier",
    "ResolutionSnapshot": "state", "ResolvedPlan": "carrier",
    "FillReceipt": "evidence", "FulfillmentProof": "evidence",
    "ClaimReceipt": "evidence", "SettlementReceipt": "evidence",
    "RefundReceipt": "evidence", "CancellationReceipt": "evidence",
}
REQUIRED_HARD_PREDICATES = {
    "allowed-effects", "forbidden-effects", "no-extra-spend",
    "no-diverted-change", "no-extra-mint-or-burn", "fee-compliance",
    "signer-compliance", "disclosure-compliance", "capability-non-escalation",
    "replay-rejection", "cancel-or-fill-exclusivity", "refund-authorization",
    "partial-fill-residual-correctness", "settlement-level-correspondence",
    "composition",
}
REQUIRED_PREFERENCES = {
    "minimum-fee", "earliest-finality", "maximum-output", "minimum-disclosure"
}
REQUIRED_ACTORS = {
    "user-or-signer", "counterparty", "resolver-or-solver", "prover",
    "runtime-or-indexer", "wallet-or-custodian", "ledger-observer", "auditor",
}
REQUIRED_EVENT_FIELDS = {
    "channel", "actor", "domain", "settlement_level", "time", "state_anchor",
    "payload_commitment", "visibility", "public_effects", "private_commitments",
    "approved_disclosures", "metadata", "failure", "timing", "availability",
}
REQUIRED_ASSUMPTION_KINDS = {
    "cryptographic", "compiler", "proof-system", "ledger",
    "wallet-or-custody", "oracle-or-registry", "resolver-or-solver",
    "runtime-or-indexer", "relay-bridge-or-finality", "availability-or-liveness",
}
REQUIRED_QUANTIFIERS = ["S_sign", "S_exec", "I", "P", "T", "O", "A", "H", "D", "N", "L"]
REQUIRED_PREMISES = [
    ("well-typed", "wellTyped(I)"),
    ("domain-bound", "I.domain = D"),
    ("nonce-bound", "I.nonce = N"),
    ("authorization-valid", "AuthorizationValid(I.authorization, S_exec, D, N)"),
    ("snapshot-fresh", "SnapshotFresh(A.snapshot, A.querySet, A.mutability, S_exec)"),
    ("resolver-bound", "ResolverBound(A.resolverId, A.codeHash, A.implementation, A.upgradeState)"),
    ("assumptions-verified", "AssumptionsVerified(H, A.witnesses)"),
    ("refinement-chain-bound", "RefinementChainBound(I, A, P)"),
    ("plan-valid", "verifyPlan(I, S_sign, S_exec, A, P) = VerificationCertificate.valid"),
    ("plan-executes", "executes(P, S_exec, A.asyncBoundary, T, O)"),
    ("settlement-predicate", "SettlementPredicate(L, I.finalityPolicy, T, O)"),
]
REQUIRED_BINDINGS = {
    "network", "ledger", "verifying-contract", "upgrade-state", "core",
    "serializer", "compiler", "circuit", "proof-parameters",
    "resolver-identity", "resolver-code-hash", "resolver-implementation",
    "proxy-implementation", "signing-state", "execution-state",
    "state-sequence", "nonce-domain", "validity-interval", "signers",
    "approval-scope", "assets", "effects", "fees", "disclosures",
    "capabilities", "assumptions", "failure-outcomes",
    "complete-effect-projection", "signing-order-profile",
}
REQUIRED_SUBSIDIARY_CLAIMS = REQUIRED_HARD_PREDICATES - {"allowed-effects", "forbidden-effects"}
REQUIRED_EXCLUSIONS = {
    "liveness", "economic-optimality", "legal-enforceability",
    "stronger-settlement-levels", "proof-system-soundness",
    "backend-correspondence", "ledger-correspondence",
}

PINNED_INPUTS = {
    "deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml": (
        "86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd",
        "prompt version 1.3 immutable input",
    ),
    "docs/superpowers/specs/2026-09-03-moriarty-s01-intent-theorem-freeze-design.md": (
        "1d2da7cc797ff4a4d06fa51aefd730d0083fb43d03c0f83df29c3763b07de6ab",
        "approved S01 design immutable input",
    ),
    "docs/superpowers/specs/2026-09-04-moriarty-s01-audit-resolutions.md": (
        "7bace035f5e6ff65c8cb0e8c6a42fca33c81092e28d30f3a45608c2912aab97f",
        "S01 audit resolution supplement immutable input",
    ),
    "evidence/semantic-scope/index.json": (
        "c42cce8891c8a82559c02df9ae0471bcd2f80e57246f7e54a83851c804c93c31",
        "semantic-scope index immutable input",
    ),
    EXPECTED_SCOPE_PATH: (
        EXPECTED_SCOPE_SHA256,
        "semantic-scope snapshot immutable input",
    ),
    "moriarty/core.py": (
        "564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b",
        "canonical Core source immutable input",
    ),
    "moriarty/__init__.py": (
        "6d6b58f9875c357d5d16858e55ce81edd1e56ecb1c5b28476c7a911cad2e1127",
        "local package import boundary immutable input",
    ),
    "moriarty/swap.py": (
        "82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797",
        "canonical atomic-swap source immutable input",
    ),
    "moriarty/intent.py": (
        "3d206f8e36c03e3799e196978898a3a2ac9f788a90c53da189ed04a68a0044d0",
        "local effect checker source immutable input",
    ),
}
REVIEWED_OUTPUT_PINS = {
    "schemas/intent/s01-artifacts-v1.json": "0ea3a96b8235808e7ab89a12b955743523d0bc24326fae80d7baf7a511fc7ed0",
    "openspec/changes/s01-intent-theorem-freeze/specs/intent-safety/spec.md": "ad89d72be3f7e9f44efdaf8214336354d687eca97191a8341b84dc20c29a18a3",
    "evidence/s01-intent-theorem-freeze/ambiguity-resolutions.json": "cd5be16bf7ca8b43da9878c10bf17b2d2e42e1e0d58cafa12ba0392a9502310f",
    "evidence/s01-intent-theorem-freeze/assumption-registry.json": "124921ae21d215c7fcecab480888955fc9352784f95ce50632465924fe74d66e",
    "evidence/s01-intent-theorem-freeze/atomic-swap-extra-effect.json": "e230e235ee7c5a062dde4628356edc0ce70385ba3cbf46d4f32bc5cbd1d61243",
    "evidence/s01-intent-theorem-freeze/hard-predicates.json": "ae617146e55392e6908a17775f74d91f3da0dfece045ccb24ff779a05ff13e38",
    "evidence/s01-intent-theorem-freeze/intent-safety-judgment.json": "c4ea5bfe4b87b0ace36a73d1903fa9abeddeb09297a4e490b16bd01ff921e0db",
    "evidence/s01-intent-theorem-freeze/lifecycle-objects.json": "9bb967ab06b835d61fba5f6adcc11b70da27a10befd9da497a7cb3de3d3a7d3c",
    "evidence/s01-intent-theorem-freeze/observation-model.json": "b56f7c8653906064568d9e132008ba68e0121d3f001e85211b8f6965a2fd899c",
    "evidence/s01-intent-theorem-freeze/optimization-preferences.json": "040ed99d4576db165f91a9289fb365c4d172b76e0ee4dab5ab8120d3b36ea132",
    "evidence/s01-intent-theorem-freeze/terminology.json": "b4572303254f932aba311ddc324cf774ad2501b01fe054a309cc75eb6ea935c3",
}
OUTPUT_ROLES = {
    **{path: "reviewed normative S01 artifact" for path in REVIEWED_OUTPUT_PINS},
    "scripts/validate_s01_intent_evidence.py": "fail-closed S01 evidence validator",
    "evidence/s01-intent-theorem-freeze/validation-report.json": "recomputed S01 validation report",
}
LIMITATIONS = [
    "The candidate theorem is not mechanized.",
    "The effect verifier covers exact transfer effects for the atomic-swap falsifier only.",
    "S01 does not establish Compact, ZKIR, proof-system, ledger, or wallet correspondence.",
    "S01 does not establish ACTUS reference-vector compatibility.",
    "The architecture choice remains open until S02.",
    "The manifest self-hash is integrity evidence, not a signature; coordinated replacement of the validator and its trusted constants is outside this guarantee.",
]
EVIDENCE_BOUNDARY = [
    "S01 specification and local transfer-checker experiment evidence only.",
    "No signing authority, mechanized proof, backend correspondence, ledger correspondence, or ACTUS compatibility is established.",
]


class ValidationError(ValueError):
    """A deterministic S01 evidence failure."""


def _path_label(path: Path) -> str:
    try:
        return path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        return str(path)


def _reject_constant(value: str) -> NoReturn:
    raise ValueError(f"non-finite JSON number {value}")


def _unique_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key {key!r}")
        result[key] = value
    return result


def load_json(path: Path) -> dict[str, object]:
    try:
        value = json.loads(
            path.read_text(encoding="utf-8"),
            object_pairs_hook=_unique_object,
            parse_constant=_reject_constant,
        )
    except (OSError, UnicodeError, json.JSONDecodeError, ValueError) as error:
        raise ValidationError(f"invalid JSON: {_path_label(path)}: {error}") from error
    if not isinstance(value, dict):
        raise ValidationError(f"expected JSON object: {_path_label(path)}")
    return value


def file_sha256(path: Path) -> str:
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except OSError as error:
        raise ValidationError(f"cannot hash evidence path {_path_label(path)}: {error}") from error


def canonical_sha256(value: object) -> str:
    try:
        encoded = json.dumps(
            value, sort_keys=True, separators=(",", ":"), allow_nan=False
        ).encode("utf-8")
    except (TypeError, ValueError) as error:
        raise ValidationError(f"cannot canonically hash non-finite JSON value: {error}") from error
    return hashlib.sha256(encoded).hexdigest()


def _json_bytes(value: object) -> bytes:
    try:
        return (json.dumps(value, indent=2, allow_nan=False) + "\n").encode("utf-8")
    except (TypeError, ValueError) as error:
        raise ValidationError(f"cannot serialize evidence JSON: {error}") from error


def load_artifacts() -> dict[str, dict[str, object]]:
    return {
        name: load_json(
            _closed_path(
                f"evidence/s01-intent-theorem-freeze/{name}.json",
                "normative artifact",
            )
        )
        for name in ARTIFACT_DEFS
    }


def _require_nonblank(value: object, context: str) -> None:
    if not isinstance(value, str) or not value.strip():
        raise ValidationError(f"{context} must be a nonblank string")


def _require_unique(values: Sequence[object], context: str) -> None:
    try:
        unique = set(values)
    except TypeError as error:
        raise ValidationError(f"{context} contains a non-scalar identity") from error
    if len(values) != len(unique):
        raise ValidationError(f"{context} identities must be unique")


def _require_finite_json(value: object, context: str = "artifact") -> None:
    if isinstance(value, float) and (value != value or value in (float("inf"), float("-inf"))):
        raise ValidationError(f"{context} contains a non-finite JSON number")
    if isinstance(value, Mapping):
        for key, child in value.items():
            _require_finite_json(child, f"{context}.{key}")
    elif isinstance(value, Sequence) and not isinstance(value, (str, bytes, bytearray)):
        for index, child in enumerate(value):
            _require_finite_json(child, f"{context}[{index}]")


def _load_schema() -> dict[str, object]:
    schema = load_json(
        _closed_path("schemas/intent/s01-artifacts-v1.json", "schema")
    )
    try:
        Draft202012Validator.check_schema(schema)
    except SchemaError as error:
        raise ValidationError(f"S01 schema is invalid: {error.message}") from error
    return schema


def _schema_validate_one(
    value: dict[str, object], definition: str, schema: dict[str, object], name: str
) -> None:
    wrapper = {
        "$schema": schema.get("$schema"),
        "$ref": f"#/$defs/{definition}",
        "$defs": schema.get("$defs"),
    }
    try:
        Draft202012Validator(wrapper).validate(value)
    except SchemaValidationError as error:
        location = ".".join(str(part) for part in error.absolute_path) or "<root>"
        raise ValidationError(
            f"S01 schema validation failed for {name} at {location}: {error.message}"
        ) from error


def _validate_artifacts(artifacts: Mapping[str, dict[str, object]]) -> None:
    expected = set(ARTIFACT_DEFS)
    if set(artifacts) != expected or len(artifacts) != len(expected):
        raise ValidationError("S01 artifact set is missing, extra, or inconsistent")
    schema = _load_schema()
    for name, definition in ARTIFACT_DEFS.items():
        value = artifacts[name]
        if not isinstance(value, dict):
            raise ValidationError(f"S01 artifact {name} is not a JSON object")
        _require_finite_json(value, name)
        _schema_validate_one(value, definition, schema, name)
        schema_id, artifact_kind = ARTIFACT_IDENTITIES[name]
        scope_key = "semantic_scope_version" if name == "atomic-swap-extra-effect" else "scope_version"
        if (
            value.get("schema_id") != schema_id
            or value.get("artifact_kind") != artifact_kind
            or value.get(scope_key) != EXPECTED_SCOPE_VERSION
        ):
            raise ValidationError(f"S01 artifact identity is inconsistent: {name}")


def _gate_01(artifacts: Mapping[str, dict[str, object]]) -> None:
    terms = artifacts["terminology"]["terms"]
    ids = [item["id"] for item in terms]
    nouns = [item["noun"] for item in terms]
    if tuple(ids) != REQUIRED_TERM_IDS or tuple(nouns) != REQUIRED_TERMS:
        raise ValidationError("S01 terminology inventory differs from the reviewed freeze")
    _require_unique(ids, "S01 terminology")
    _require_unique(nouns, "S01 terminology nouns")
    aliases = [alias for item in terms for alias in item["aliases"]]
    _require_unique(aliases, "S01 terminology alias")
    if any(not alias.strip() for alias in aliases):
        raise ValidationError("S01 terminology contains a blank alias")
    noun_owners = {noun: item["id"] for noun, item in zip(nouns, terms, strict=True)}
    for item in terms:
        if item["unresolved_aliases"]:
            raise ValidationError("S01 terminology contains an unresolved alias")
        for alias in item["aliases"]:
            if alias in noun_owners and noun_owners[alias] != item["id"]:
                raise ValidationError("S01 terminology alias collides with a canonical noun")
    if set(aliases) != REQUIRED_XML_ALIASES:
        raise ValidationError("S01 terminology does not cover the full XML inventory")
    if not ATOMICITY_KINDS <= set(nouns):
        raise ValidationError("S01 terminology lacks an explicit atomicity kind")
    if {"DisplayProjection", "AuthorizationProjection"} - set(nouns):
        raise ValidationError("S01 terminology lacks both projection types")
    for item in terms:
        _require_nonblank(
            item["definition"], f"S01 terminology {item['noun']} definition"
        )
    resolutions = artifacts["ambiguity-resolutions"]["resolutions"]
    _require_unique([item["id"] for item in resolutions], "S01 ambiguity resolution")
    g17 = next((item for item in resolutions if item["id"] == "AMB-S01-001"), None)
    if g17 is None or g17["resolved_predicate"] != (
        "two-human-pilots-prefer-workflow AND actus-g19-through-g24-pass"
    ):
        raise ValidationError("S01 G17 ambiguity is unresolved")


def _gate_02(artifacts: Mapping[str, dict[str, object]]) -> None:
    records = artifacts["lifecycle-objects"]["objects"]
    names = [item["name"] for item in records]
    _require_unique(names, "S01 lifecycle object")
    actual = {item["name"]: item["primary_category"] for item in records}
    if actual != LIFECYCLE_CATEGORIES or len(records) != len(LIFECYCLE_CATEGORIES):
        raise ValidationError("S01 lifecycle registry names or primary categories differ from the freeze")
    for item in records:
        for field in (
            "name", "producer", "identity_rule", "mutability_rule", "scope",
            "version", "authority_boundary",
        ):
            _require_nonblank(item[field], f"S01 lifecycle {item['name']} {field}")
        if not item["permitted_consumers"]:
            raise ValidationError("S01 lifecycle object lacks a permitted consumer")


def _gate_03(artifacts: Mapping[str, dict[str, object]]) -> None:
    registry = artifacts["hard-predicates"]
    if registry["registry_kind"] != "hard":
        raise ValidationError("S01 hard predicate registry has the wrong kind")
    predicates = registry["predicates"]
    ids = [item["id"] for item in predicates]
    _require_unique(ids, "S01 hard predicate")
    if set(ids) != REQUIRED_HARD_PREDICATES or len(ids) != len(REQUIRED_HARD_PREDICATES):
        raise ValidationError("S01 hard predicate set differs from the freeze")
    for item in predicates:
        if item["enters_authorized_at"] is not True:
            raise ValidationError("every S01 hard predicate must enter authorizedAt")
        signed = item["signed_binding"]
        external = item["external_premise"]
        if not ((isinstance(signed, str) and signed.strip()) or (isinstance(external, str) and external.strip())):
            raise ValidationError("S01 hard predicate lacks a binding or external premise")


def _gate_04(artifacts: Mapping[str, dict[str, object]]) -> None:
    registry = artifacts["optimization-preferences"]
    if registry["registry_kind"] != "preference":
        raise ValidationError("S01 optimization preference registry has the wrong kind")
    preferences = registry["predicates"]
    ids = [item["id"] for item in preferences]
    _require_unique(ids, "S01 optimization preference")
    if set(ids) != REQUIRED_PREFERENCES or len(ids) != len(REQUIRED_PREFERENCES):
        raise ValidationError("S01 optimization preference set differs from the freeze")
    for item in preferences:
        if item["enters_authorized_at"] is not False:
            raise ValidationError("an optimization preference enters authorizedAt")
        if item["ranking_precondition"] != "all-hard-predicates-valid":
            raise ValidationError("an optimization preference can rank an invalid plan")


def _gate_05(artifacts: Mapping[str, dict[str, object]]) -> None:
    judgment = artifacts["intent-safety-judgment"]
    if judgment["judgment_id"] != "INTENT-SAFETY" or judgment["version"] != "1":
        raise ValidationError("S01 theorem judgment identity differs from the freeze")
    if judgment["quantifiers"] != REQUIRED_QUANTIFIERS:
        raise ValidationError("S01 theorem quantifiers differ from the freeze")
    premises = [(item["id"], item["expression"]) for item in judgment["premises"]]
    if premises != REQUIRED_PREMISES:
        raise ValidationError("S01 theorem premise IDs or expressions differ from the freeze")
    if judgment["conclusion"] != {
        "operator": "authorizedAt",
        "expression": "authorizedAt(I, H, L, view(I.authorizer, D, L, T, O))",
    }:
        raise ValidationError("S01 theorem conclusion differs from the freeze")
    bindings = judgment["required_bindings"]
    _require_unique(bindings, "S01 theorem required binding")
    if set(bindings) != REQUIRED_BINDINGS:
        raise ValidationError("S01 theorem required binding set differs from the freeze")
    claims = judgment["subsidiary_claims"]
    _require_unique(claims, "S01 subsidiary claim")
    if set(claims) != REQUIRED_SUBSIDIARY_CLAIMS:
        raise ValidationError("S01 subsidiary claim set differs from the freeze")
    if judgment["proof_status"] != "candidate-unmechanized":
        raise ValidationError("S01 theorem judgment must remain candidate-unmechanized")
    if judgment["architecture_binding"] != "neutral":
        raise ValidationError("S01 theorem judgment selects an architecture before S02")
    exclusions = judgment["exclusions"]
    _require_unique(exclusions, "S01 theorem exclusion")
    if set(exclusions) != REQUIRED_EXCLUSIONS:
        raise ValidationError("S01 theorem exclusions differ from the freeze")


def _gate_06(artifacts: Mapping[str, dict[str, object]]) -> None:
    assumptions = artifacts["assumption-registry"]["assumptions"]
    ids = [item["id"] for item in assumptions]
    _require_unique(ids, "S01 assumption")
    if len(assumptions) != len(REQUIRED_ASSUMPTION_KINDS):
        raise ValidationError("S01 assumption registry length differs from the freeze")
    if {item["kind"] for item in assumptions} != REQUIRED_ASSUMPTION_KINDS:
        raise ValidationError("S01 assumption kinds differ from the freeze")
    for item in assumptions:
        for field in (
            "id", "version", "kind", "owner", "scope", "evidence_method",
            "freshness_rule", "revocation_rule", "failure_result",
        ):
            _require_nonblank(item[field], f"S01 assumption {field}")
        if not item["affected_claims"]:
            raise ValidationError("S01 assumption lacks affected claims")
        if item["verification_status"] == "verified" and item["observable"] is False:
            raise ValidationError("an unobservable S01 assumption is marked verified")


def _gate_07(artifacts: Mapping[str, dict[str, object]]) -> None:
    model = artifacts["observation-model"]
    actors = model["actors"]
    actor_ids = [item["id"] for item in actors]
    _require_unique(actor_ids, "S01 observation actor")
    if set(actor_ids) != REQUIRED_ACTORS or len(actor_ids) != len(REQUIRED_ACTORS):
        raise ValidationError("S01 observation actor set differs from the freeze")
    for actor in actors:
        _require_nonblank(
            actor["observation_scope"],
            f"S01 observation actor {actor['id']} definition",
        )
    fields = model["event_fields"]
    field_names = [item["name"] for item in fields]
    _require_unique(field_names, "S01 observation field")
    if set(field_names) != REQUIRED_EVENT_FIELDS or len(field_names) != len(REQUIRED_EVENT_FIELDS):
        raise ValidationError("S01 observation field set differs from the freeze")
    for field in fields:
        _require_nonblank(field["definition"], "S01 observation field definition")
        _require_nonblank(field["visibility_rule"], "S01 observation visibility rule")
        _require_nonblank(field["declassification_rule"], "S01 observation declassification rule")
    if model["authorization_effect_projection_complete"] is not True:
        raise ValidationError("S01 observation model can hide an authorization effect")
    display = model["display_projection"]
    authorization = model["authorization_projection"]
    if (
        display["name"] != "DisplayProjection"
        or display["retains_complete_effect_set"] is not False
        or "never establishes authorization" not in display["authority_rule"]
    ):
        raise ValidationError("S01 display projection violates the audit supplement")
    expected_authorization_projection = {
        "name": "AuthorizationProjection",
        "purpose": "Account for every public, private, fee, refund, change, and failed-path effect through complete inclusion and exclusion evidence.",
        "authority_rule": "authorizedAt evaluates every hard predicate only over this projection bound to trace, outcome, domain, level, signed intent, and predicate set.",
        "retains_complete_effect_set": True,
    }
    if authorization != expected_authorization_projection:
        raise ValidationError("S01 authorization projection lacks complete evidence")
    profiles = model["signing_profiles"]
    names = [item["name"] for item in profiles]
    _require_unique(names, "S01 signing profile")
    if set(names) != {"SignAfterResolve", "SignBeforeResolve"} or len(profiles) != 2:
        raise ValidationError("S01 signing profile set differs from the audit supplement")
    expected_requirements = {
        "SignAfterResolve": {
            "full resolved plan", "authenticated execution-relevant state",
            "complete effect evidence", "all hard predicates valid",
            "selected-profile premises valid",
        },
        "SignBeforeResolve": {
            "signed intent hard bounds",
            "domain, nonce, validity, cancellation, signers, capabilities, and disclosures",
            "execution-boundary mechanism identity",
            "later complete-plan validation against authorization",
        },
    }
    for profile in profiles:
        if profile["implementation_status"] != "not-implemented":
            raise ValidationError("S01 signing profile must not claim implementation")
        if set(profile["requirements"]) != expected_requirements[profile["name"]]:
            raise ValidationError("S01 signing profile requirements differ from the supplement")
        boundary = profile["authorization_boundary"]
        if profile["name"] == "SignAfterResolve" and "Before signing" not in boundary:
            raise ValidationError("S01 SignAfterResolve boundary differs from the supplement")
        if profile["name"] == "SignBeforeResolve" and "At execution" not in boundary:
            raise ValidationError("S01 SignBeforeResolve boundary differs from the supplement")


def _canonical_swap_effects() -> tuple[object, ...]:
    core_module, intent_module, swap_module = _load_local_api()
    parameters = swap_module.SwapParameters.example()
    contract = swap_module.canonical_swap(parameters)
    state = core_module.State()
    supplied = (
        core_module.DepositInput(
            parameters.alice_account, parameters.alice, parameters.amount_a
        ),
        core_module.DepositInput(
            parameters.bob_account, parameters.bob, parameters.amount_b
        ),
        core_module.ChoiceInput(parameters.choice_id, parameters.bob, 1),
    )
    settlement = None
    for now, transaction_input in enumerate(supplied, start=1):
        settlement = core_module.compute_transaction(
            contract, state, transaction_input, now=now
        )
        if not settlement.accepted:
            raise ValidationError(f"S01 canonical atomic-swap execution failed: {settlement.error}")
        contract, state = settlement.contract, settlement.state
    if settlement is None or settlement.warnings:
        raise ValidationError("S01 canonical atomic-swap effect evidence is unavailable")
    effects = intent_module.effects_from_payments(settlement.payments)
    if not effects:
        raise ValidationError("S01 canonical atomic-swap effect evidence is unavailable")
    return effects


def _certificate(vector: dict[str, object], field: str):
    _, intent_module, _ = _load_local_api()
    try:
        policy = intent_module.EffectPolicy.from_mapping(vector["policy"])
        effects = tuple(intent_module.Effect.from_mapping(item) for item in vector[field])
        return policy, effects, intent_module.verify_plan_effects(policy, effects)
    except (KeyError, TypeError, ValueError) as error:
        raise ValidationError(f"S01 atomic-swap effect evidence is unavailable: {error}") from error


def _certificate_record(certificate) -> dict[str, object]:
    return {
        "valid": certificate.valid,
        "reason": certificate.reason,
        "signing_request_permitted": certificate.signing_request_permitted,
    }


def _gate_08(artifacts: Mapping[str, dict[str, object]]) -> None:
    vector = artifacts["atomic-swap-extra-effect"]
    if (
        vector.get("application") != "canonical-atomic-swap"
        or vector.get("resolution") != "SignAfterResolve"
        or vector.get("scope") != "local-effect-check-only"
    ):
        raise ValidationError("S01 atomic-swap vector identity differs from the freeze")
    canonical = _canonical_swap_effects()
    policy, baseline, certificate = _certificate(vector, "baseline_effects")
    if not baseline:
        raise ValidationError("S01 atomic-swap effect evidence is unavailable")
    if baseline != canonical:
        raise ValidationError("S01 atomic-swap baseline differs from recomputed Core payments")
    if policy.allowed != canonical or policy.required != canonical:
        raise ValidationError("S01 atomic-swap policy differs from recomputed Core payments")
    expected = {"valid": True, "reason": "VALID", "signing_request_permitted": False}
    if vector["baseline_expected"] != expected or _certificate_record(certificate) != expected:
        raise ValidationError("S01 atomic-swap baseline does not produce the exact expected result")
    if certificate.unauthorized_effects or certificate.missing_effects:
        raise ValidationError("S01 atomic-swap baseline certificate contains unexpected effects")


def _gate_09(artifacts: Mapping[str, dict[str, object]]) -> None:
    _, intent_module, _ = _load_local_api()
    vector = artifacts["atomic-swap-extra-effect"]
    policy, mutant, certificate = _certificate(vector, "mutant_effects")
    baseline = tuple(
        intent_module.Effect.from_mapping(item) for item in vector["baseline_effects"]
    )
    extra = intent_module.Effect("transfer", "alice", "mallory", "aa", "A", 1)
    if mutant != baseline + (extra,):
        raise ValidationError("S01 atomic-swap mutant is not baseline plus the exact third-party effect")
    if policy.allowed != baseline or policy.required != baseline:
        raise ValidationError("S01 atomic-swap mutant changes the signed policy")
    expected = {
        "valid": False,
        "reason": "UNAUTHORIZED_EXTRA_EFFECT",
        "signing_request_permitted": False,
    }
    if vector["mutant_expected"] != expected or _certificate_record(certificate) != expected:
        raise ValidationError("S01 atomic-swap mutant does not produce the exact rejection result")
    if certificate.unauthorized_effects != (extra,) or certificate.missing_effects:
        raise ValidationError("S01 atomic-swap mutant certificate identifies the wrong effect")
    restored = intent_module.verify_plan_effects(policy, mutant[:-1])
    if mutant[:-1] != baseline or _certificate_record(restored) != {
        "valid": True, "reason": "VALID", "signing_request_permitted": False
    }:
        raise ValidationError("S01 atomic-swap restored baseline is not exactly valid")


def _gate_10(scope: dict[str, object]) -> None:
    if not scope:
        raise ValidationError("S01 semantic scope override is empty")
    if scope.get("current_version") != EXPECTED_SCOPE_VERSION:
        raise ValidationError("S01 semantic scope version changed")
    if scope.get("current_sha256") != EXPECTED_SCOPE_SHA256:
        raise ValidationError("S01 semantic scope digest changed")
    if scope.get("current_path") != EXPECTED_SCOPE_PATH:
        raise ValidationError("S01 semantic scope canonical path changed")
    snapshot = _closed_path(EXPECTED_SCOPE_PATH, "semantic scope")
    if file_sha256(snapshot) != EXPECTED_SCOPE_SHA256:
        raise ValidationError("S01 semantic scope snapshot bytes changed")


def _closed_path(path_text: object, context: str) -> Path:
    if not isinstance(path_text, str) or not path_text.strip():
        raise ValidationError(f"S01 {context} path is blank")
    pure = PurePosixPath(path_text)
    if pure.is_absolute() or pure.as_posix() != path_text or any(part in ("", ".", "..") for part in pure.parts):
        raise ValidationError(f"S01 {context} path is invalid: {path_text}")
    lexical = ROOT.joinpath(*pure.parts)
    try:
        resolved = lexical.resolve(strict=True)
    except OSError as error:
        raise ValidationError(f"S01 {context} path is missing: {path_text}") from error
    if not resolved.is_relative_to(ROOT.resolve()) or resolved != lexical.absolute() or not resolved.is_file():
        raise ValidationError(f"S01 {context} path escapes or is substituted: {path_text}")
    return resolved


def _validate_import_identity() -> None:
    core_module, intent_module, swap_module = _load_local_api()
    for module, relative in (
        (core_module, "moriarty/core.py"),
        (swap_module, "moriarty/swap.py"),
        (intent_module, "moriarty/intent.py"),
    ):
        source = inspect.getsourcefile(module)
        if source is None or Path(source).resolve() != (ROOT / relative).resolve():
            raise ValidationError(f"S01 source import identity mismatch: {relative}")


def _load_local_api():
    source_paths = (
        "moriarty/__init__.py",
        "moriarty/core.py",
        "moriarty/intent.py",
        "moriarty/swap.py",
    )
    for relative in source_paths:
        path = _closed_path(relative, "immutable input")
        expected = PINNED_INPUTS[relative][0]
        if file_sha256(path) != expected:
            raise ValidationError(f"S01 immutable input digest changed: {relative}")
    try:
        core_module = importlib.import_module("moriarty.core")
        intent_module = importlib.import_module("moriarty.intent")
        swap_module = importlib.import_module("moriarty.swap")
    except Exception as error:
        raise ValidationError(
            f"S01 pinned local source import failed: {type(error).__name__}: {error}"
        ) from error
    return core_module, intent_module, swap_module


def _validate_pinned_bytes() -> None:
    for path_text, (expected, _) in PINNED_INPUTS.items():
        path = _closed_path(path_text, "immutable input")
        if file_sha256(path) != expected:
            raise ValidationError(f"S01 immutable input digest changed: {path_text}")
    for path_text, expected in REVIEWED_OUTPUT_PINS.items():
        path = _closed_path(path_text, "reviewed normative output")
        if file_sha256(path) != expected:
            raise ValidationError(f"S01 reviewed normative output digest changed: {path_text}")


def recompute_gate(
    *,
    artifact_overrides: dict[str, dict[str, object]] | None = None,
    scope_override: dict[str, object] | None = None,
) -> dict[str, object]:
    """Recompute the ten semantic gates; this does not validate published receipts."""
    artifacts = load_artifacts() if artifact_overrides is None else artifact_overrides
    scope = (
        load_json(_closed_path("evidence/semantic-scope/index.json", "semantic scope"))
        if scope_override is None
        else scope_override
    )
    _validate_artifacts(artifacts)
    gates: tuple[Callable[[], None], ...] = (
        lambda: _gate_01(artifacts),
        lambda: _gate_02(artifacts),
        lambda: _gate_03(artifacts),
        lambda: _gate_04(artifacts),
        lambda: _gate_05(artifacts),
        lambda: _gate_06(artifacts),
        lambda: _gate_07(artifacts),
        lambda: _gate_08(artifacts),
        lambda: _gate_09(artifacts),
        lambda: _gate_10(scope),
    )
    gate_results: dict[str, bool] = {}
    for index, gate in enumerate(gates, start=1):
        gate()
        gate_results[f"S01-{index:02d}"] = True
    _validate_import_identity()
    _validate_pinned_bytes()
    if artifact_overrides is not None and artifact_overrides != load_artifacts():
        raise ValidationError("S01 in-memory artifact override differs from on-disk evidence")
    if scope_override is not None and scope_override != load_json(
        _closed_path("evidence/semantic-scope/index.json", "semantic scope")
    ):
        raise ValidationError("S01 semantic scope override differs from the pinned on-disk index")
    return {
        "schema_version": 1,
        "package": "S01",
        "status": "recomputed-package-gate-passed",
        "gate_results": gate_results,
    }


def _report_for(result: dict[str, object]) -> dict[str, object]:
    return {
        "schema_id": "moriarty.dev/intent-validation-report/v1",
        "artifact_kind": "intent-validation-report",
        "package_id": "s01-intent-theorem-freeze",
        "status": result["status"],
        "semantic_scope_version": EXPECTED_SCOPE_VERSION,
        "semantic_scope_sha256": EXPECTED_SCOPE_SHA256,
        "gate_results": result["gate_results"],
        "evidence_boundary": EVIDENCE_BOUNDARY,
    }


def _manifest_for(result: dict[str, object], report_bytes: bytes) -> dict[str, object]:
    inputs = [
        {"path": path, "sha256": digest, "role": role}
        for path, (digest, role) in sorted(PINNED_INPUTS.items())
    ]
    outputs = []
    for path, role in sorted(OUTPUT_ROLES.items()):
        digest = (
            hashlib.sha256(report_bytes).hexdigest()
            if path == REPORT_PATH.relative_to(ROOT).as_posix()
            else file_sha256(_closed_path(path, "manifest output"))
        )
        outputs.append({"path": path, "sha256": digest, "role": role})
    manifest: dict[str, object] = {
        "schema_id": "moriarty.dev/intent-evidence-manifest/v1",
        "artifact_kind": "intent-evidence-manifest",
        "package_id": "s01-intent-theorem-freeze",
        "sprint_id": "S01",
        "status": "validated-theorem-freeze-at-S3",
        "semantic_scope_version": EXPECTED_SCOPE_VERSION,
        "semantic_scope_sha256": EXPECTED_SCOPE_SHA256,
        "inputs": inputs,
        "outputs": outputs,
        "commands": [f"{sys.executable} {Path(__file__).resolve()} --write-evidence"],
        "environment": {"python": sys.version, "platform": platform.platform()},
        "gate_results": result["gate_results"],
        "limitations": LIMITATIONS,
    }
    manifest["manifest_sha256"] = canonical_sha256(manifest)
    return manifest


def _validate_report(report: dict[str, object], result: dict[str, object]) -> None:
    _schema_validate_one(report, "validationReport", _load_schema(), "validation-report")
    expected = _report_for(result)
    if report != expected:
        raise ValidationError("S01 validation report identity, status, scope, or gates do not match recomputation")


def _records_by_path(records: object, collection: str) -> dict[str, dict[str, object]]:
    if not isinstance(records, list):
        raise ValidationError(f"S01 evidence manifest {collection} is not an array")
    paths = [record.get("path") if isinstance(record, dict) else None for record in records]
    if len(paths) != len(set(paths)):
        raise ValidationError(f"S01 evidence manifest has duplicate {collection}")
    if any(not isinstance(record, dict) for record in records):
        raise ValidationError(f"S01 evidence manifest {collection} has a non-record entry")
    return {record["path"]: record for record in records}


def _validate_manifest(
    manifest: dict[str, object], result: dict[str, object], *, report_bytes: bytes | None = None
) -> None:
    _schema_validate_one(manifest, "evidenceManifest", _load_schema(), "evidence-manifest")
    identity = {
        "schema_id": "moriarty.dev/intent-evidence-manifest/v1",
        "artifact_kind": "intent-evidence-manifest",
        "package_id": "s01-intent-theorem-freeze",
        "sprint_id": "S01",
        "status": "validated-theorem-freeze-at-S3",
        "semantic_scope_version": EXPECTED_SCOPE_VERSION,
        "semantic_scope_sha256": EXPECTED_SCOPE_SHA256,
        "gate_results": result["gate_results"],
        "limitations": LIMITATIONS,
    }
    for field, expected in identity.items():
        if manifest.get(field) != expected:
            raise ValidationError(f"S01 evidence manifest {field} does not match recomputation")
    unhashed = dict(manifest)
    claimed = unhashed.pop("manifest_sha256", None)
    if claimed != canonical_sha256(unhashed):
        raise ValidationError("S01 evidence manifest self-hash is invalid")
    inputs = _records_by_path(manifest["inputs"], "inputs")
    outputs = _records_by_path(manifest["outputs"], "outputs")
    if set(inputs) != set(PINNED_INPUTS):
        raise ValidationError("S01 evidence manifest input path closure is incomplete or extra")
    if set(outputs) != set(OUTPUT_ROLES):
        raise ValidationError("S01 evidence manifest output path closure is incomplete or extra")
    for path_text, (expected_digest, expected_role) in PINNED_INPUTS.items():
        record = inputs[path_text]
        path = _closed_path(path_text, "evidence manifest input")
        if record["role"] != expected_role or record["sha256"] != expected_digest:
            raise ValidationError(f"S01 evidence manifest input identity is invalid: {path_text}")
        if file_sha256(path) != record["sha256"]:
            raise ValidationError(f"S01 evidence manifest input digest is invalid: {path_text}")
    report_relative = REPORT_PATH.relative_to(ROOT).as_posix()
    for path_text, expected_role in OUTPUT_ROLES.items():
        record = outputs[path_text]
        path = _closed_path(path_text, "evidence manifest output") if not (
            report_bytes is not None and path_text == report_relative
        ) else REPORT_PATH
        actual_digest = (
            hashlib.sha256(report_bytes).hexdigest()
            if report_bytes is not None and path_text == report_relative
            else file_sha256(path)
        )
        if record["role"] != expected_role or record["sha256"] != actual_digest:
            raise ValidationError(f"S01 evidence manifest output digest or role is invalid: {path_text}")


def validate_published_evidence() -> dict[str, object]:
    """Recompute semantic gates and close them against the report and manifest."""
    result = recompute_gate()
    report = load_json(
        _closed_path(
            "evidence/s01-intent-theorem-freeze/validation-report.json",
            "validation report",
        )
    )
    _validate_report(report, result)
    manifest = load_json(
        _closed_path(
            "evidence/s01-intent-theorem-freeze/evidence-manifest.json",
            "evidence manifest",
        )
    )
    _validate_manifest(manifest, result)
    return result


def _publish_atomically(path: Path, payload: bytes) -> None:
    descriptor, temporary_name = tempfile.mkstemp(prefix=f".{path.name}.", dir=path.parent)
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(payload)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, path)
    except BaseException:
        temporary.unlink(missing_ok=True)
        raise


def write_evidence() -> dict[str, object]:
    result = recompute_gate()
    report = _report_for(result)
    report_bytes = _json_bytes(report)
    manifest = _manifest_for(result, report_bytes)
    manifest_bytes = _json_bytes(manifest)
    _validate_report(report, result)
    _validate_manifest(manifest, result, report_bytes=report_bytes)
    _publish_atomically(REPORT_PATH, report_bytes)
    _publish_atomically(MANIFEST_PATH, manifest_bytes)
    return result


def _json_failure(message: str) -> None:
    print(json.dumps({"status": "invalid", "error": message}, sort_keys=True))


def main(argv: Sequence[str] | None = None) -> int:
    arguments = list(sys.argv[1:] if argv is None else argv)
    try:
        if arguments == []:
            result = validate_published_evidence()
        elif arguments == ["--write-evidence"]:
            result = write_evidence()
        else:
            raise ValidationError(f"invalid argument list: {arguments!r}")
    except ValidationError as error:
        _json_failure(str(error))
        return 1
    except Exception as error:  # fail closed without exposing a traceback from the CLI
        _json_failure(f"unexpected validation failure: {type(error).__name__}: {error}")
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
