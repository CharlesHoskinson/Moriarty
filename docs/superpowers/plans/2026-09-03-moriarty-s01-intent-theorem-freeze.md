# Moriarty S01 Intent-Theorem Freeze Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Freeze and validate an architecture-neutral intent-safety judgment without changing Moriarty Core.

**Architecture:** Store the normative S01 model in strict JSON artifacts under one OpenSpec change. Add a small Python effect verifier for the atomic-swap falsifier. A package-specific validator recomputes all ten gates and checks every evidence digest.

**Tech Stack:** Python 3.13, frozen dataclasses, JSON Schema Draft 2020-12, pytest, OpenSpec Markdown, SHA-256.

## Execution audit — 2026-09-04

Read the [execution audit](../reviews/2026-09-04-moriarty-v1.3-execution-audit.md)
before using the examples below. The audit identifies corrections to the
sample parser, signing permission, and package validator.

Begin with Task 1 and the independent Task 4 transfer-effect experiment.
Keep Tasks 2, 3, and 5 open until their freeze obligations have explicit dispositions.
The XML and approved-design digests remain unchanged.

For Task 4, reject malformed mapping fields without coercion. A successful
effect comparison does not permit signing. Set `signing_request_permitted`
to false for both the valid baseline and the rejected mutant. Treat `valid`
as an effect-comparison verdict only. Record this correction in the vector's limitations.

For Task 5, apply A05 and A07 from the audit before implementing the validator.
Use measured test counts. Do not copy a predicted count into completion evidence.

## Global Constraints

- Keep semantic scope version `0.0.0-e00.2`.
- Keep semantic-scope digest `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.
- Use prompt version `1.3` with SHA-256 `86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd` as an immutable input.
- Use design SHA-256 `1d2da7cc797ff4a4d06fa51aefd730d0083fb43d03c0f83df29c3763b07de6ab` as an immutable input.
- Do not change any Core constructor, type, action, observation, value, warning, error, or backend artifact.
- Do not modify `evidence/marlowe-org-full-graph-2026-09-02.json`.
- Keep hard predicates separate from optimization preferences.
- Reject unauthorized effects before creating a signing request.
- Do not claim mechanized proof, backend correspondence, ledger correspondence, or ACTUS compatibility from S01.
- Apply ASD-STE100 writing rules to OpenSpec instructions, evidence prose, wiki updates, and commit messages.

---

## File structure

Create these files:

- `openspec/changes/s01-intent-theorem-freeze/.openspec.yaml` identifies the change.
- `openspec/changes/s01-intent-theorem-freeze/README.md` gives the package entry point.
- `openspec/changes/s01-intent-theorem-freeze/proposal.md` states scope and impact.
- `openspec/changes/s01-intent-theorem-freeze/design.md` states boundaries and evidence flow.
- `openspec/changes/s01-intent-theorem-freeze/tasks.md` tracks package closure.
- `openspec/changes/s01-intent-theorem-freeze/specs/intent-safety/spec.md` contains normative requirements and scenarios.
- `schemas/intent/s01-artifacts-v1.json` defines strict schemas for every S01 JSON artifact.
- `evidence/s01-intent-theorem-freeze/terminology.json` freezes canonical nouns and excluded meanings.
- `evidence/s01-intent-theorem-freeze/lifecycle-objects.json` freezes lifecycle categories and authority boundaries.
- `evidence/s01-intent-theorem-freeze/observation-model.json` freezes actors, fields, visibility, and declassification.
- `evidence/s01-intent-theorem-freeze/hard-predicates.json` freezes safety predicates.
- `evidence/s01-intent-theorem-freeze/optimization-preferences.json` freezes non-safety ranking rules.
- `evidence/s01-intent-theorem-freeze/assumption-registry.json` freezes assumption evidence and failure rules.
- `evidence/s01-intent-theorem-freeze/intent-safety-judgment.json` freezes the candidate theorem and subsidiary claims.
- `evidence/s01-intent-theorem-freeze/ambiguity-resolutions.json` separates the ACTUS benchmark from human pilot evidence.
- `evidence/s01-intent-theorem-freeze/atomic-swap-extra-effect.json` stores the positive and negative vectors.
- `evidence/s01-intent-theorem-freeze/validation-report.json` stores the recomputed gate result.
- `evidence/s01-intent-theorem-freeze/evidence-manifest.json` binds all S01 inputs and outputs.
- `moriarty/intent.py` implements the architecture-neutral effect check.
- `scripts/validate_s01_intent_evidence.py` validates schemas, semantics, vectors, scope, and digests.
- `tests/test_s01_openspec.py` tests the OpenSpec package contract.
- `tests/test_s01_registries.py` tests the registries and candidate theorem.
- `tests/test_intent_verifier.py` tests the effect verifier and atomic-swap vector.
- `tests/test_s01_intent_evidence.py` tests the fail-closed package validator.

Modify these files:

- `wiki/moriarty-architecture.md` marks the current architecture recommendation as provisional until S02.
- `wiki/research-journal.md` records the evidence-only S01 transition.
- `wiki/index.md` links the S01 specification and evidence.

Do not add S01 to `openspec/work-packages.json`. That manifest defines the
earlier twelve-package program and has a strict compatibility test.

---

### Task 1: Create the S01 OpenSpec contract

**Files:**

- Create: `tests/test_s01_openspec.py`
- Create: `openspec/changes/s01-intent-theorem-freeze/.openspec.yaml`
- Create: `openspec/changes/s01-intent-theorem-freeze/README.md`
- Create: `openspec/changes/s01-intent-theorem-freeze/proposal.md`
- Create: `openspec/changes/s01-intent-theorem-freeze/design.md`
- Create: `openspec/changes/s01-intent-theorem-freeze/tasks.md`
- Create: `openspec/changes/s01-intent-theorem-freeze/specs/intent-safety/spec.md`

**Interfaces:**

- Consumes: prompt version `1.3`, the approved S01 design, and semantic scope `0.0.0-e00.2`.
- Produces: package identifier `s01-intent-theorem-freeze` and ten named acceptance predicates.

- [ ] **Step 1: Write the failing OpenSpec structure test**

```python
from pathlib import Path


ROOT = Path(__file__).parents[1]
CHANGE = ROOT / "openspec/changes/s01-intent-theorem-freeze"


def test_s01_openspec_has_every_required_section() -> None:
    required_files = {
        ".openspec.yaml",
        "README.md",
        "proposal.md",
        "design.md",
        "tasks.md",
        "specs/intent-safety/spec.md",
    }
    assert {
        str(path.relative_to(CHANGE))
        for path in CHANGE.rglob("*")
        if path.is_file()
    } == required_files

    text = "\n".join(
        (CHANGE / relative).read_text(encoding="utf-8")
        for relative in sorted(required_files)
    )
    for phrase in (
        "Dependencies",
        "Immutable inputs",
        "Exact outputs",
        "Acceptance predicates",
        "Negative controls",
        "Evidence manifest",
        "Failure outcomes",
        "Rollback",
        "Semantic-scope transition",
    ):
        assert phrase in text
    assert "0.0.0-e00.2" in text
    assert "Do not change Core" in text


def test_s01_normative_spec_has_ten_scenarios() -> None:
    text = (CHANGE / "specs/intent-safety/spec.md").read_text(encoding="utf-8")
    assert text.count("#### Scenario:") == 10
    assert "UNAUTHORIZED_EXTRA_EFFECT" in text
    assert "optimization preference" in text
```

- [ ] **Step 2: Run the test and confirm the missing-package failure**

Run: `uv run pytest tests/test_s01_openspec.py -q`

Expected: FAIL because `openspec/changes/s01-intent-theorem-freeze` does not exist.

- [ ] **Step 3: Create the OpenSpec files**

Use this metadata:

```yaml
schema: spec-driven
created: 2026-09-03
goal: Freeze and validate the architecture-neutral Moriarty intent-safety judgment.
```

Give the normative spec these ten scenarios in this order:

1. Every required term has one definition and no unresolved alias.
2. Every lifecycle object has one primary category and authority boundary.
3. Every hard predicate has a signed binding or explicit external premise.
4. No optimization preference enters `authorizedAt`.
5. The theorem contains all required variables, premises, bindings, and claims.
6. Every assumption has a scope, evidence rule, and failure result.
7. Every observation field has visibility and declassification rules.
8. The atomic-swap baseline passes.
9. The extra-effect mutant fails with `UNAUTHORIZED_EXTRA_EFFECT`.
10. The semantic-scope version and digest remain unchanged.

State these failure outcomes exactly:

- `block-s01-on-ambiguity`
- `reject-incomplete-effect-projection`
- `move-architecture-operation-to-s02`
- `reject-mutant-without-widening-intent`

State that rollback removes only S01 outputs. State that rollback preserves all
prior immutable evidence and semantic-scope files.

- [ ] **Step 4: Run the OpenSpec structure test**

Run: `uv run pytest tests/test_s01_openspec.py -q`

Expected: `2 passed`.

- [ ] **Step 5: Commit the OpenSpec contract**

```bash
git add tests/test_s01_openspec.py openspec/changes/s01-intent-theorem-freeze
git commit -m "spec: define the S01 intent-safety gate"
```

### Task 2: Freeze the S01 registries and schema

**Files:**

- Create: `schemas/intent/s01-artifacts-v1.json`
- Create: `tests/test_s01_registries.py`
- Create: `evidence/s01-intent-theorem-freeze/terminology.json`
- Create: `evidence/s01-intent-theorem-freeze/lifecycle-objects.json`
- Create: `evidence/s01-intent-theorem-freeze/observation-model.json`
- Create: `evidence/s01-intent-theorem-freeze/hard-predicates.json`
- Create: `evidence/s01-intent-theorem-freeze/optimization-preferences.json`
- Create: `evidence/s01-intent-theorem-freeze/assumption-registry.json`
- Create: `evidence/s01-intent-theorem-freeze/ambiguity-resolutions.json`

**Interfaces:**

- Consumes: W4, W5, W8, G01, G06, G08, G16, and G17 from prompt version `1.3`.
- Produces: strict JSON objects keyed by `artifact_kind` and validated through `$defs` in one schema.

- [ ] **Step 1: Write the failing schema and content tests**

```python
from __future__ import annotations

import json
from pathlib import Path

from jsonschema import Draft202012Validator


ROOT = Path(__file__).parents[1]
EVIDENCE = ROOT / "evidence/s01-intent-theorem-freeze"
SCHEMA = json.loads(
    (ROOT / "schemas/intent/s01-artifacts-v1.json").read_text(encoding="utf-8")
)


def load(name: str) -> dict[str, object]:
    return json.loads((EVIDENCE / name).read_text(encoding="utf-8"))


def test_s01_registries_match_their_strict_schemas() -> None:
    files = {
        "terminology.json": "terminology",
        "lifecycle-objects.json": "lifecycleObjects",
        "observation-model.json": "observationModel",
        "hard-predicates.json": "predicateRegistry",
        "optimization-preferences.json": "predicateRegistry",
        "assumption-registry.json": "assumptionRegistry",
        "ambiguity-resolutions.json": "ambiguityResolutions",
    }
    for name, definition in files.items():
        Draft202012Validator(SCHEMA["$defs"][definition]).validate(load(name))


def test_required_w4_lifecycle_objects_are_distinct() -> None:
    expected = {
        "IntentObjective", "UserIntent", "IntentDomain", "AllowedEffects",
        "ForbiddenEffects", "Validity", "SignerPolicy", "DisclosurePolicy",
        "DisclosureAcceptance", "CapabilityBudget", "FeeLedger", "StateAnchor",
        "StateCommitment", "FinalityPolicy", "FailurePolicy", "QuoteRequest",
        "Quote", "IntentAuthorization", "OrderPayload", "ResolutionSnapshot",
        "ResolvedPlan", "FillReceipt", "FulfillmentProof", "ClaimReceipt",
        "SettlementReceipt", "RefundReceipt", "CancellationReceipt",
    }
    records = load("lifecycle-objects.json")["objects"]
    names = [record["name"] for record in records]
    assert expected <= set(names)
    assert len(names) == len(set(names))
    assert all(record["primary_category"] in {
        "command", "authorization", "carrier", "state", "observation", "evidence"
    } for record in records)


def test_preferences_cannot_authorize_a_plan() -> None:
    hard = load("hard-predicates.json")
    preferences = load("optimization-preferences.json")
    assert hard["registry_kind"] == "hard"
    assert preferences["registry_kind"] == "preference"
    assert all(item["enters_authorized_at"] for item in hard["predicates"])
    assert not any(item["enters_authorized_at"] for item in preferences["predicates"])


def test_actus_is_not_classified_as_a_human_pilot() -> None:
    resolutions = load("ambiguity-resolutions.json")["resolutions"]
    g17 = next(item for item in resolutions if item["id"] == "AMB-S01-001")
    assert g17["source_locator"] == "release_gates/G17"
    assert g17["pilot_kind"] == "human-team"
    assert g17["actus_kind"] == "automated-benchmark"
    assert g17["resolved_predicate"] == (
        "two-human-pilots-prefer-workflow AND actus-g19-through-g24-pass"
    )
```

- [ ] **Step 2: Run the tests and confirm the missing-schema failure**

Run: `uv run pytest tests/test_s01_registries.py -q`

Expected: FAIL because the S01 schema and registries do not exist.

- [ ] **Step 3: Create the strict combined schema**

Set `$schema` to Draft 2020-12. Set `$id` to
`https://moriarty.dev/schemas/intent/s01-artifacts-v1.json`.

Define `$defs` for these object kinds:

- `terminology`
- `lifecycleObjects`
- `observationModel`
- `predicateRegistry`
- `assumptionRegistry`
- `judgment`
- `ambiguityResolutions`
- `planVector`
- `validationReport`
- `evidenceManifest`

Set `additionalProperties` to `false` on each record and top-level object. Use
non-empty strings for every rule and definition. Require non-empty arrays where
the design requires at least one item.

Keep each `$defs` entry self-contained. Do not use a root-relative `$ref` from
a definition that the tests validate independently.

Give each terminology record an `aliases` array and an `unresolved_aliases`
array. The second array must be empty when the package passes.

Give each hard predicate nullable `signed_binding` and `external_premise`
fields. Require at least one of these fields through the package validator.

Use these primary-category values:

```json
["command", "authorization", "carrier", "state", "observation", "evidence"]
```

Use these assumption-kind values:

```json
[
  "cryptographic", "compiler", "proof-system", "ledger",
  "wallet-or-custody", "oracle-or-registry", "resolver-or-solver",
  "runtime-or-indexer", "relay-bridge-or-finality", "availability-or-liveness"
]
```

Use these result values:

```json
["valid", "invalid", "unavailable"]
```

Require `schema_id`, `artifact_kind`, `scope_version`, and the artifact payload
on each registry. Require the theorem fields used in Task 3.

Require these vector fields:

```text
schema_id, artifact_kind, application, semantic_scope_version, policy,
baseline_effects, mutant_effects, baseline_expected, mutant_expected, limitations
```

Require these validation-report fields:

```text
schema_id, artifact_kind, package_id, status, semantic_scope_version,
semantic_scope_sha256, gate_results, evidence_boundary
```

Require these evidence-manifest fields:

```text
schema_id, artifact_kind, package_id, sprint_id, status,
semantic_scope_version, semantic_scope_sha256, inputs, outputs, commands,
environment, gate_results, limitations, manifest_sha256
```

- [ ] **Step 4: Create the terminology and lifecycle registries**

Give every terminology record all fields from the approved design. Include all
27 W4 objects from the test. Include these W5 and theorem nouns:

```text
AssumptionManifest, AuthorizationReceipt, CanonicalIntentBytes, CircuitIdentity,
EffectSummary, FinalityEvidence, HumanReadableInterpretation, IntentHash,
LocalProofVerification, ProofReceipt, ProofRequest, RollbackEvidence,
SigningRequest, StateEvidence, StaticCertificate, SubmissionReceipt,
TransactionVerification, TranslationCertificate, VerificationCertificate,
SigningState, ExecutionState, Trace, Outcome, SettlementLevel, Projection
```

Use one noun per meaning. Add excluded meanings for every overloaded term.
Define `Quote` as provider-authenticated information, not authorization. Define
`ResolvedPlan` as a complete candidate execution, not proof of execution.

Classify all 27 W4 objects in `lifecycle-objects.json`. Give each object one
producer, permitted consumers, identity rule, mutability rule, scope, version,
and authority boundary.

- [ ] **Step 5: Create the observation model**

Use these actor identifiers:

```json
[
  "user-or-signer", "counterparty", "resolver-or-solver", "prover",
  "runtime-or-indexer", "wallet-or-custodian", "ledger-observer", "auditor"
]
```

Use these event fields:

```json
[
  "channel", "actor", "domain", "settlement_level", "time",
  "state_anchor", "payload_commitment", "visibility", "public_effects",
  "private_commitments", "approved_disclosures", "metadata", "failure",
  "timing", "availability"
]
```

Give each field a visibility rule and declassification rule. State that
declassification changes visibility only. State that every projection retains
the complete effect set for authorization checks.

- [ ] **Step 6: Create the predicate registries**

Use these hard-predicate identifiers:

```text
allowed-effects, forbidden-effects, no-extra-spend, no-diverted-change,
no-extra-mint-or-burn, fee-compliance, signer-compliance,
disclosure-compliance, capability-non-escalation, replay-rejection,
cancel-or-fill-exclusivity, refund-authorization,
partial-fill-residual-correctness, settlement-level-correspondence, composition
```

Give each predicate canonical parameters, evaluation state, required artifacts,
three-valued results, stable failure reasons, binding requirements, settlement
levels, residual behavior, and a composition rule.

Use these preference identifiers:

```text
minimum-fee, earliest-finality, maximum-output, minimum-disclosure
```

Set `enters_authorized_at` to `false` for every preference. State that ranking
starts only after every hard predicate passes.

- [ ] **Step 7: Create the assumption and ambiguity registries**

Create one assumption for each schema assumption kind. Give each assumption an
owner, scope, affected claims, evidence method, freshness rule, revocation rule,
and failure result.

Set every unobservable real-world dependency to `assumed` or `unavailable`.
Never set such a dependency to `verified`.

Create `AMB-S01-001` for G17. Record this resolved predicate exactly:

```text
two-human-pilots-prefer-workflow AND actus-g19-through-g24-pass
```

State that ACTUS is an automated benchmark. State that ACTUS cannot report a
human preference. Preserve the original G17 text and locator in the record.

- [ ] **Step 8: Run the registry tests**

Run: `uv run pytest tests/test_s01_registries.py -q`

Expected: `4 passed`.

- [ ] **Step 9: Commit the registries**

```bash
git add schemas/intent tests/test_s01_registries.py evidence/s01-intent-theorem-freeze
git commit -m "spec: freeze S01 intent registries"
```

### Task 3: Freeze the candidate theorem artifact

**Files:**

- Modify: `tests/test_s01_registries.py`
- Create: `evidence/s01-intent-theorem-freeze/intent-safety-judgment.json`

**Interfaces:**

- Consumes: the S01 terminology, observation, predicate, and assumption registries.
- Produces: judgment identifier `INTENT-SAFETY`, version `1`, and thirteen subsidiary claims.

- [ ] **Step 1: Add the failing theorem-shape test**

```python
def test_intent_safety_judgment_has_the_frozen_shape() -> None:
    judgment = load("intent-safety-judgment.json")
    Draft202012Validator(SCHEMA["$defs"]["judgment"]).validate(judgment)
    assert judgment["judgment_id"] == "INTENT-SAFETY"
    assert judgment["architecture_binding"] == "neutral"
    assert judgment["proof_status"] == "candidate-unmechanized"
    assert judgment["quantifiers"] == [
        "S_sign", "S_exec", "I", "P", "T", "O", "A", "H", "D", "N", "L"
    ]
    assert [item["id"] for item in judgment["premises"]] == [
        "well-typed", "domain-bound", "nonce-bound", "authorization-valid",
        "snapshot-fresh", "resolver-bound", "assumptions-verified",
        "refinement-chain-bound", "plan-valid", "plan-executes",
        "settlement-predicate",
    ]
    assert judgment["conclusion"]["operator"] == "authorizedAt"
    assert "optimization-preference" not in json.dumps(judgment)
    assert set(judgment["subsidiary_claims"]) == {
        "no-extra-spend", "no-diverted-change", "no-extra-mint-or-burn",
        "fee-compliance", "signer-compliance", "disclosure-compliance",
        "capability-non-escalation", "replay-rejection",
        "cancel-or-fill-exclusivity", "refund-authorization",
        "partial-fill-residual-correctness", "settlement-level-correspondence",
        "composition",
    }
```

- [ ] **Step 2: Run the theorem test and confirm the missing-file failure**

Run: `uv run pytest tests/test_s01_registries.py::test_intent_safety_judgment_has_the_frozen_shape -q`

Expected: FAIL because `intent-safety-judgment.json` does not exist.

- [ ] **Step 3: Create the theorem artifact**

Encode the approved theorem without architecture-specific operations. Use
these premise expressions exactly:

```text
wellTyped(I)
I.domain = D
I.nonce = N
AuthorizationValid(I.authorization, S_exec, D, N)
SnapshotFresh(A.snapshot, A.querySet, A.mutability, S_exec)
ResolverBound(A.resolverId, A.codeHash, A.implementation, A.upgradeState)
AssumptionsVerified(H, A.witnesses)
RefinementChainBound(I, A, P)
verifyPlan(I, S_sign, S_exec, A, P) = VerificationCertificate.valid
executes(P, S_exec, A.asyncBoundary, T, O)
SettlementPredicate(L, I.finalityPolicy, T, O)
```

Use this conclusion exactly:

```text
authorizedAt(I, H, L, view(I.authorizer, D, L, T, O))
```

Include every binding from the approved design. Include the thirteen subsidiary
claims from the test. Include explicit exclusions for liveness, economic
optimality, legal enforceability, stronger settlement levels, proof-system
soundness, backend correspondence, and ledger correspondence.

- [ ] **Step 4: Run the registry and theorem tests**

Run: `uv run pytest tests/test_s01_registries.py -q`

Expected: `5 passed`.

- [ ] **Step 5: Commit the theorem artifact**

```bash
git add tests/test_s01_registries.py evidence/s01-intent-theorem-freeze/intent-safety-judgment.json
git commit -m "spec: freeze the candidate intent-safety judgment"
```

### Task 4: Implement the atomic-swap extra-effect falsifier

**Files:**

- Create: `moriarty/intent.py`
- Create: `tests/test_intent_verifier.py`
- Create: `evidence/s01-intent-theorem-freeze/atomic-swap-extra-effect.json`

**Interfaces:**

- Consumes: `moriarty.core.Payment`, the canonical atomic swap, and exact effect policies.
- Produces: `Effect`, `EffectPolicy`, `VerificationCertificate`, `effects_from_payments`, and `verify_plan_effects`.

- [ ] **Step 1: Write the failing unit tests**

```python
from __future__ import annotations

import json
from pathlib import Path

from moriarty.core import ChoiceInput, DepositInput, State, compute_transaction
from moriarty.intent import Effect, EffectPolicy, effects_from_payments, verify_plan_effects
from moriarty.swap import SwapParameters, canonical_swap


ROOT = Path(__file__).parents[1]


def settle_swap():
    parameters = SwapParameters.example()
    contract = canonical_swap(parameters)
    first = compute_transaction(
        contract,
        State(),
        DepositInput(parameters.alice_account, parameters.alice, parameters.amount_a),
        now=1,
    )
    second = compute_transaction(
        first.contract,
        first.state,
        DepositInput(parameters.bob_account, parameters.bob, parameters.amount_b),
        now=2,
    )
    return compute_transaction(
        second.contract,
        second.state,
        ChoiceInput(parameters.choice_id, parameters.bob, 1),
        now=3,
    )


def test_atomic_swap_effects_satisfy_the_exact_policy() -> None:
    effects = effects_from_payments(settle_swap().payments)
    certificate = verify_plan_effects(
        EffectPolicy(allowed=effects, required=effects),
        effects,
    )
    assert certificate.valid
    assert certificate.reason == "VALID"
    assert certificate.unauthorized_effects == ()
    assert certificate.missing_effects == ()


def test_additional_third_party_payment_is_rejected_before_signing() -> None:
    effects = effects_from_payments(settle_swap().payments)
    mutant = effects + (
        Effect("transfer", "alice", "mallory", "aa", "A", 1),
    )
    certificate = verify_plan_effects(
        EffectPolicy(allowed=effects, required=effects),
        mutant,
    )
    assert not certificate.valid
    assert certificate.reason == "UNAUTHORIZED_EXTRA_EFFECT"
    assert certificate.unauthorized_effects == (mutant[-1],)
    assert certificate.signing_request_permitted is False


def test_removing_the_extra_effect_restores_the_valid_result() -> None:
    vector = json.loads(
        (ROOT / "evidence/s01-intent-theorem-freeze/atomic-swap-extra-effect.json")
        .read_text(encoding="utf-8")
    )
    policy = EffectPolicy.from_mapping(vector["policy"])
    baseline = tuple(Effect.from_mapping(item) for item in vector["baseline_effects"])
    mutant = tuple(Effect.from_mapping(item) for item in vector["mutant_effects"])
    assert baseline == effects_from_payments(settle_swap().payments)
    assert verify_plan_effects(policy, mutant).reason == "UNAUTHORIZED_EXTRA_EFFECT"
    assert verify_plan_effects(policy, mutant[:-1]).reason == "VALID"
    assert mutant[:-1] == baseline
```

- [ ] **Step 2: Run the verifier tests and confirm the import failure**

Run: `uv run pytest tests/test_intent_verifier.py -q`

Expected: FAIL because `moriarty.intent` does not exist.

- [ ] **Step 3: Implement the minimal effect verifier**

```python
"""Architecture-neutral intent-effect verification for S01."""

from __future__ import annotations

from collections import Counter
from dataclasses import dataclass
from typing import Iterable, Mapping

from moriarty.core import Payment


@dataclass(frozen=True, order=True)
class Effect:
    kind: str
    source: str
    destination: str
    policy_id: str
    asset_name: str
    quantity: int

    def __post_init__(self) -> None:
        if self.kind != "transfer":
            raise ValueError("S01 supports only transfer effects")
        for name, value in (
            ("source", self.source),
            ("destination", self.destination),
            ("policy_id", self.policy_id),
            ("asset_name", self.asset_name),
        ):
            if not value:
                raise ValueError(f"{name} must not be empty")
        if type(self.quantity) is not int or self.quantity <= 0:
            raise ValueError("effect quantity must be a positive integer")

    @classmethod
    def from_mapping(cls, value: Mapping[str, object]) -> "Effect":
        return cls(
            kind=str(value["kind"]),
            source=str(value["source"]),
            destination=str(value["destination"]),
            policy_id=str(value["policy_id"]),
            asset_name=str(value["asset_name"]),
            quantity=int(value["quantity"]),
        )


@dataclass(frozen=True)
class EffectPolicy:
    allowed: tuple[Effect, ...]
    required: tuple[Effect, ...]

    def __post_init__(self) -> None:
        allowed = Counter(self.allowed)
        required = Counter(self.required)
        if required - allowed:
            raise ValueError("required effects must be allowed")

    @classmethod
    def from_mapping(cls, value: Mapping[str, object]) -> "EffectPolicy":
        return cls(
            allowed=tuple(Effect.from_mapping(item) for item in value["allowed"]),
            required=tuple(Effect.from_mapping(item) for item in value["required"]),
        )


@dataclass(frozen=True)
class VerificationCertificate:
    valid: bool
    reason: str
    unauthorized_effects: tuple[Effect, ...] = ()
    missing_effects: tuple[Effect, ...] = ()

    @property
    def signing_request_permitted(self) -> bool:
        return self.valid


def _expanded(counter: Counter[Effect]) -> tuple[Effect, ...]:
    return tuple(sorted(counter.elements()))


def verify_plan_effects(
    policy: EffectPolicy,
    actual: Iterable[Effect],
) -> VerificationCertificate:
    actual_counter = Counter(actual)
    extra = actual_counter - Counter(policy.allowed)
    if extra:
        return VerificationCertificate(
            False,
            "UNAUTHORIZED_EXTRA_EFFECT",
            unauthorized_effects=_expanded(extra),
        )
    missing = Counter(policy.required) - actual_counter
    if missing:
        return VerificationCertificate(
            False,
            "MISSING_REQUIRED_EFFECT",
            missing_effects=_expanded(missing),
        )
    return VerificationCertificate(True, "VALID")


def effects_from_payments(payments: Iterable[Payment]) -> tuple[Effect, ...]:
    return tuple(
        Effect(
            "transfer",
            payment.source.owner.name,
            payment.to.name,
            payment.token.policy_id,
            payment.token.asset_name,
            payment.quantity,
        )
        for payment in payments
    )
```

- [ ] **Step 4: Add the machine-readable vector**

Set `schema_id` to `moriarty.dev/intent-plan-vector/v1`. Set `application` to
`canonical-atomic-swap`. Copy the two settlement payments into `allowed`,
`required`, and `baseline_effects`.

Append this exact effect only to `mutant_effects`:

```json
{
  "kind": "transfer",
  "source": "alice",
  "destination": "mallory",
  "policy_id": "aa",
  "asset_name": "A",
  "quantity": 1
}
```

Set the baseline reason to `VALID`. Set the mutant reason to
`UNAUTHORIZED_EXTRA_EFFECT`. Set both signing expectations explicitly.

- [ ] **Step 5: Run the verifier tests**

Run: `uv run pytest tests/test_intent_verifier.py -q`

Expected: `3 passed`.

- [ ] **Step 6: Run the existing Core and swap regression tests**

Run: `uv run pytest tests/test_core_semantics.py tests/test_swap_bounds.py tests/test_translation_certificate.py -q`

Expected: `31 passed` with no changed E00 artifact.

- [ ] **Step 7: Commit the falsifier**

```bash
git add moriarty/intent.py tests/test_intent_verifier.py evidence/s01-intent-theorem-freeze/atomic-swap-extra-effect.json
git commit -m "feat: reject unauthorized swap effects"
```

### Task 5: Implement the fail-closed S01 validator

**Files:**

- Create: `scripts/validate_s01_intent_evidence.py`
- Create: `tests/test_s01_intent_evidence.py`
- Create: `evidence/s01-intent-theorem-freeze/validation-report.json`
- Create: `evidence/s01-intent-theorem-freeze/evidence-manifest.json`

**Interfaces:**

- Consumes: every S01 registry, the theorem, the vector, the scope index, the prompt, and the approved design.
- Produces: `ValidationError`, `recompute_gate`, CLI exit status, a ten-result report, and a self-hashed evidence manifest.

- [ ] **Step 1: Write the failing validator tests**

```python
from __future__ import annotations

import importlib.util
import json
import subprocess
from pathlib import Path

import pytest


ROOT = Path(__file__).parents[1]


def load_module():
    path = ROOT / "scripts/validate_s01_intent_evidence.py"
    spec = importlib.util.spec_from_file_location("s01_validator", path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_s01_validator_recomputes_all_ten_gates() -> None:
    result = load_module().recompute_gate()
    assert result["status"] == "recomputed-package-gate-passed"
    assert list(result["gate_results"]) == [f"S01-{index:02d}" for index in range(1, 11)]
    assert all(result["gate_results"].values())


def test_s01_validator_rejects_preference_authorization() -> None:
    module = load_module()
    artifacts = module.load_artifacts()
    changed = json.loads(json.dumps(artifacts))
    changed["optimization-preferences"]["predicates"][0]["enters_authorized_at"] = True
    with pytest.raises(module.ValidationError, match="optimization preference"):
        module.recompute_gate(artifact_overrides=changed)


def test_s01_validator_rejects_a_missing_theorem_binding() -> None:
    module = load_module()
    artifacts = module.load_artifacts()
    changed = json.loads(json.dumps(artifacts))
    changed["intent-safety-judgment"]["required_bindings"].remove("proof-parameters")
    with pytest.raises(module.ValidationError, match="required binding"):
        module.recompute_gate(artifact_overrides=changed)


def test_s01_validator_rejects_a_stale_scope_digest() -> None:
    module = load_module()
    scope = module.load_json(ROOT / "evidence/semantic-scope/index.json")
    scope["current_sha256"] = "0" * 64
    with pytest.raises(module.ValidationError, match="semantic scope"):
        module.recompute_gate(scope_override=scope)


def test_s01_validator_cli_passes() -> None:
    result = subprocess.run(
        ["uv", "run", "python", "scripts/validate_s01_intent_evidence.py"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    assert json.loads(result.stdout)["status"] == "recomputed-package-gate-passed"
```

- [ ] **Step 2: Run the validator tests and confirm the missing-script failure**

Run: `uv run pytest tests/test_s01_intent_evidence.py -q`

Expected: FAIL because `scripts/validate_s01_intent_evidence.py` does not exist.

- [ ] **Step 3: Implement schema and semantic validation**

Use this validator structure. Keep each additional schema check inside the
indicated gate function.

```python
#!/usr/bin/env python3
"""Recompute the closed Moriarty S01 intent-theorem gate."""

from __future__ import annotations

import hashlib
import json
import sys
from collections.abc import Mapping
from pathlib import Path

from jsonschema import Draft202012Validator
from jsonschema.exceptions import ValidationError as SchemaError

from moriarty.intent import Effect, EffectPolicy, verify_plan_effects


ROOT = Path(__file__).parents[1]
EVIDENCE = ROOT / "evidence/s01-intent-theorem-freeze"
SCHEMA_PATH = ROOT / "schemas/intent/s01-artifacts-v1.json"
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
    "validation-report": "validationReport",
    "evidence-manifest": "evidenceManifest",
}
EXPECTED_SCOPE_VERSION = "0.0.0-e00.2"
EXPECTED_SCOPE_SHA256 = "9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a"
REQUIRED_LIFECYCLE_OBJECTS = {
    "IntentObjective", "UserIntent", "IntentDomain", "AllowedEffects",
    "ForbiddenEffects", "Validity", "SignerPolicy", "DisclosurePolicy",
    "DisclosureAcceptance", "CapabilityBudget", "FeeLedger", "StateAnchor",
    "StateCommitment", "FinalityPolicy", "FailurePolicy", "QuoteRequest",
    "Quote", "IntentAuthorization", "OrderPayload", "ResolutionSnapshot",
    "ResolvedPlan", "FillReceipt", "FulfillmentProof", "ClaimReceipt",
    "SettlementReceipt", "RefundReceipt", "CancellationReceipt",
}
REQUIRED_TERMS = REQUIRED_LIFECYCLE_OBJECTS | {
    "AssumptionManifest", "AuthorizationReceipt", "CanonicalIntentBytes",
    "CircuitIdentity", "EffectSummary", "FinalityEvidence",
    "HumanReadableInterpretation", "IntentHash", "LocalProofVerification",
    "ProofReceipt", "ProofRequest", "RollbackEvidence", "SigningRequest",
    "StateEvidence", "StaticCertificate", "SubmissionReceipt",
    "TransactionVerification", "TranslationCertificate",
    "VerificationCertificate", "SigningState", "ExecutionState", "Trace",
    "Outcome", "SettlementLevel", "Projection",
}
REQUIRED_HARD_PREDICATES = {
    "allowed-effects", "forbidden-effects", "no-extra-spend",
    "no-diverted-change", "no-extra-mint-or-burn", "fee-compliance",
    "signer-compliance", "disclosure-compliance", "capability-non-escalation",
    "replay-rejection", "cancel-or-fill-exclusivity", "refund-authorization",
    "partial-fill-residual-correctness", "settlement-level-correspondence",
    "composition",
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
REQUIRED_QUANTIFIERS = [
    "S_sign", "S_exec", "I", "P", "T", "O", "A", "H", "D", "N", "L"
]
REQUIRED_PREMISES = [
    "well-typed", "domain-bound", "nonce-bound", "authorization-valid",
    "snapshot-fresh", "resolver-bound", "assumptions-verified",
    "refinement-chain-bound", "plan-valid", "plan-executes",
    "settlement-predicate",
]
REQUIRED_BINDINGS = {
    "network", "ledger", "verifying-contract", "upgrade-state", "core",
    "serializer", "compiler", "circuit", "proof-parameters",
    "resolver-identity", "resolver-code-hash", "resolver-implementation",
    "proxy-implementation",
    "signing-state", "execution-state", "state-sequence", "nonce-domain",
    "validity-interval", "signers", "approval-scope", "assets", "effects",
    "fees", "disclosures", "capabilities", "assumptions", "failure-outcomes",
}
REQUIRED_SUBSIDIARY_CLAIMS = {
    "no-extra-spend", "no-diverted-change", "no-extra-mint-or-burn",
    "fee-compliance", "signer-compliance", "disclosure-compliance",
    "capability-non-escalation", "replay-rejection",
    "cancel-or-fill-exclusivity", "refund-authorization",
    "partial-fill-residual-correctness", "settlement-level-correspondence",
    "composition",
}
EXPECTED_OUTPUT_PATHS = {
    "schemas/intent/s01-artifacts-v1.json",
    "openspec/changes/s01-intent-theorem-freeze/specs/intent-safety/spec.md",
    *{
        f"evidence/s01-intent-theorem-freeze/{name}.json"
        for name in ARTIFACT_DEFS
        if name != "evidence-manifest"
    },
}
REQUIRED_INPUT_PATHS = {
    "deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml",
    "docs/superpowers/specs/2026-09-03-moriarty-s01-intent-theorem-freeze-design.md",
    "evidence/semantic-scope/index.json",
    "evidence/semantic-scope/moriarty-core-0.0.0-e00.2.json",
    "moriarty/core.py",
    "moriarty/swap.py",
}


class ValidationError(ValueError):
    """A deterministic S01 evidence failure."""


def load_json(path: Path) -> dict[str, object]:
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
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(encoded.encode("utf-8")).hexdigest()


def load_artifacts() -> dict[str, dict[str, object]]:
    return {name: load_json(EVIDENCE / f"{name}.json") for name in ARTIFACT_DEFS}


def _schema_validate(artifacts: Mapping[str, dict[str, object]]) -> None:
    schema = load_json(SCHEMA_PATH)
    definitions = schema.get("$defs")
    if not isinstance(definitions, dict):
        raise ValidationError("S01 schema has no definitions")
    try:
        for name, definition in ARTIFACT_DEFS.items():
            Draft202012Validator(definitions[definition]).validate(artifacts[name])
    except (KeyError, SchemaError) as error:
        message = error.message if isinstance(error, SchemaError) else str(error)
        raise ValidationError(f"S01 schema validation failed: {message}") from error


def _gate_01(artifacts: Mapping[str, dict[str, object]]) -> None:
    terms = artifacts["terminology"]["terms"]
    identifiers = [item["id"] for item in terms]
    nouns = [item["noun"] for item in terms]
    if len(identifiers) != len(set(identifiers)) or len(nouns) != len(set(nouns)):
        raise ValidationError("S01 terminology identifiers and nouns must be unique")
    if any(item["unresolved_aliases"] for item in terms):
        raise ValidationError("S01 terminology contains an unresolved alias")
    if not REQUIRED_TERMS <= set(nouns):
        raise ValidationError("S01 terminology lacks a required noun")
    resolutions = artifacts["ambiguity-resolutions"]["resolutions"]
    g17 = next((item for item in resolutions if item["id"] == "AMB-S01-001"), None)
    if g17 is None or g17["resolved_predicate"] != (
        "two-human-pilots-prefer-workflow AND actus-g19-through-g24-pass"
    ):
        raise ValidationError("S01 G17 ambiguity is unresolved")


def _gate_02(artifacts: Mapping[str, dict[str, object]]) -> None:
    records = artifacts["lifecycle-objects"]["objects"]
    names = [item["name"] for item in records]
    if len(names) != len(set(names)):
        raise ValidationError("S01 lifecycle object names must be unique")
    if any(not item["authority_boundary"] for item in records):
        raise ValidationError("S01 lifecycle object lacks an authority boundary")
    if not REQUIRED_LIFECYCLE_OBJECTS <= set(names):
        raise ValidationError("S01 lifecycle registry lacks a required object")


def _gate_03(artifacts: Mapping[str, dict[str, object]]) -> None:
    predicates = artifacts["hard-predicates"]["predicates"]
    if {item["id"] for item in predicates} != REQUIRED_HARD_PREDICATES:
        raise ValidationError("S01 hard predicate set differs from the freeze")
    if any(not item["signed_binding"] and not item["external_premise"] for item in predicates):
        raise ValidationError("S01 hard predicate lacks a binding or external premise")


def _gate_04(artifacts: Mapping[str, dict[str, object]]) -> None:
    preferences = artifacts["optimization-preferences"]["predicates"]
    if any(item["enters_authorized_at"] for item in preferences):
        raise ValidationError("an optimization preference enters authorizedAt")
    if any(item["ranking_precondition"] != "all-hard-predicates-valid" for item in preferences):
        raise ValidationError("an optimization preference can rank an invalid plan")


def _gate_05(artifacts: Mapping[str, dict[str, object]]) -> None:
    judgment = artifacts["intent-safety-judgment"]
    if judgment["quantifiers"] != REQUIRED_QUANTIFIERS:
        raise ValidationError("S01 theorem quantifiers differ from the freeze")
    if [item["id"] for item in judgment["premises"]] != REQUIRED_PREMISES:
        raise ValidationError("S01 theorem premises differ from the freeze")
    if set(judgment["required_bindings"]) != REQUIRED_BINDINGS:
        raise ValidationError("S01 theorem required binding set differs from the freeze")
    if set(judgment["subsidiary_claims"]) != REQUIRED_SUBSIDIARY_CLAIMS:
        raise ValidationError("S01 subsidiary claims differ from the freeze")
    if judgment["architecture_binding"] != "neutral":
        raise ValidationError("S01 theorem selects an architecture before S02")
    if judgment["conclusion"]["expression"] != (
        "authorizedAt(I, H, L, view(I.authorizer, D, L, T, O))"
    ):
        raise ValidationError("S01 theorem conclusion differs from the freeze")


def _gate_06(artifacts: Mapping[str, dict[str, object]]) -> None:
    assumptions = artifacts["assumption-registry"]["assumptions"]
    identifiers = [item["id"] for item in assumptions]
    if len(identifiers) != len(set(identifiers)):
        raise ValidationError("S01 assumption identifiers must be unique")
    if {item["kind"] for item in assumptions} != REQUIRED_ASSUMPTION_KINDS:
        raise ValidationError("S01 assumption kinds differ from the freeze")
    for item in assumptions:
        if not item["scope"] or not item["evidence_method"] or not item["failure_result"]:
            raise ValidationError("S01 assumption lacks scope, evidence, or failure")
        if item["verification_status"] == "verified" and item["observable"] is False:
            raise ValidationError("an unobservable assumption is marked verified")


def _gate_07(artifacts: Mapping[str, dict[str, object]]) -> None:
    model = artifacts["observation-model"]
    if model["authorization_effect_projection_complete"] is not True:
        raise ValidationError("S01 observation model can hide an authorization effect")
    if {item["id"] for item in model["actors"]} != REQUIRED_ACTORS:
        raise ValidationError("S01 observation actor set differs from the freeze")
    if {item["name"] for item in model["event_fields"]} != REQUIRED_EVENT_FIELDS:
        raise ValidationError("S01 observation field set differs from the freeze")
    for field in model["event_fields"]:
        if not field["visibility_rule"] or not field["declassification_rule"]:
            raise ValidationError("S01 observation field lacks a visibility rule")


def _certificate(vector: dict[str, object], field: str):
    policy = EffectPolicy.from_mapping(vector["policy"])
    effects = tuple(Effect.from_mapping(item) for item in vector[field])
    return verify_plan_effects(policy, effects)


def _gate_08(artifacts: Mapping[str, dict[str, object]]) -> None:
    vector = artifacts["atomic-swap-extra-effect"]
    certificate = _certificate(vector, "baseline_effects")
    expected = vector["baseline_expected"]
    if (
        certificate.reason != expected["reason"]
        or certificate.valid != expected["valid"]
        or certificate.signing_request_permitted
        != expected["signing_request_permitted"]
    ):
        raise ValidationError("S01 atomic-swap baseline does not pass")


def _gate_09(artifacts: Mapping[str, dict[str, object]]) -> None:
    vector = artifacts["atomic-swap-extra-effect"]
    certificate = _certificate(vector, "mutant_effects")
    expected = vector["mutant_expected"]
    if certificate.reason != "UNAUTHORIZED_EXTRA_EFFECT" or certificate.reason != expected["reason"]:
        raise ValidationError("S01 extra-effect mutant has the wrong rejection reason")
    if (
        certificate.valid != expected["valid"]
        or certificate.signing_request_permitted
        != expected["signing_request_permitted"]
        or certificate.signing_request_permitted
    ):
        raise ValidationError("S01 extra-effect mutant reaches signing")


def _gate_10(scope: dict[str, object]) -> None:
    if scope.get("current_version") != EXPECTED_SCOPE_VERSION:
        raise ValidationError("S01 semantic scope version changed")
    if scope.get("current_sha256") != EXPECTED_SCOPE_SHA256:
        raise ValidationError("S01 semantic scope digest changed")
    snapshot = ROOT / str(scope["current_path"])
    if file_sha256(snapshot) != EXPECTED_SCOPE_SHA256:
        raise ValidationError("S01 semantic scope snapshot bytes changed")


def _validate_report(
    report: dict[str, object], gate_results: dict[str, bool]
) -> None:
    if report["status"] != "recomputed-package-gate-passed":
        raise ValidationError("S01 validation report status is not passing")
    if report["gate_results"] != gate_results:
        raise ValidationError("S01 validation report does not match recomputation")


def _validate_manifest(manifest: dict[str, object]) -> None:
    if manifest["semantic_scope_version"] != EXPECTED_SCOPE_VERSION:
        raise ValidationError("S01 manifest semantic scope version changed")
    if manifest["semantic_scope_sha256"] != EXPECTED_SCOPE_SHA256:
        raise ValidationError("S01 manifest semantic scope digest changed")
    unhashed = dict(manifest)
    claimed = unhashed.pop("manifest_sha256")
    if claimed != canonical_sha256(unhashed):
        raise ValidationError("S01 evidence manifest self-hash is invalid")

    inputs = manifest["inputs"]
    if len(inputs) != len(REQUIRED_INPUT_PATHS):
        raise ValidationError("S01 evidence manifest has duplicate inputs")
    if {item["path"] for item in inputs} != REQUIRED_INPUT_PATHS:
        raise ValidationError("S01 evidence manifest input set is incomplete")
    outputs = manifest["outputs"]
    if len(outputs) != len(EXPECTED_OUTPUT_PATHS):
        raise ValidationError("S01 evidence manifest has duplicate outputs")
    if {item["path"] for item in outputs} != EXPECTED_OUTPUT_PATHS:
        raise ValidationError("S01 evidence manifest output set is incomplete")
    for item in [*inputs, *outputs]:
        path = (ROOT / item["path"]).resolve()
        if not path.is_relative_to(ROOT.resolve()) or not path.is_file():
            raise ValidationError(f"S01 evidence path is invalid: {item['path']}")
        if file_sha256(path) != item["sha256"]:
            raise ValidationError(f"S01 evidence digest is invalid: {item['path']}")


def recompute_gate(
    *,
    artifact_overrides: dict[str, dict[str, object]] | None = None,
    scope_override: dict[str, object] | None = None,
) -> dict[str, object]:
    artifacts = artifact_overrides or load_artifacts()
    scope = scope_override or load_json(ROOT / "evidence/semantic-scope/index.json")
    _schema_validate(artifacts)
    gates = (
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
    _validate_report(artifacts["validation-report"], gate_results)
    _validate_manifest(artifacts["evidence-manifest"])
    return {
        "schema_version": 1,
        "package": "S01",
        "status": "recomputed-package-gate-passed",
        "gate_results": gate_results,
    }


def main() -> int:
    try:
        result = recompute_gate()
    except ValidationError as error:
        print(json.dumps({"status": "invalid", "error": str(error)}, sort_keys=True))
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
```

Load the schema once. Validate each artifact with its named `$defs` entry.
Reject unknown top-level and record fields through JSON Schema.

Implement the ten gates in OpenSpec order. Use stable keys `S01-01` through
`S01-10`. Stop at the first invalid condition.

For `S01-08` and `S01-09`, reconstruct `EffectPolicy` and `Effect` values from
the vector. Call `verify_plan_effects`. Compare the complete certificate with
the expected status and reason.

For `S01-10`, verify these values and the snapshot bytes:

```python
EXPECTED_SCOPE_VERSION = "0.0.0-e00.2"
EXPECTED_SCOPE_SHA256 = "9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a"
```

Validate the evidence manifest after the ten gates pass. Recompute its canonical
self-hash after removing `manifest_sha256`. Recompute every declared output
digest. Reject a missing, undeclared, duplicate, or repository-escaping path.

- [ ] **Step 4: Create the validation report**

Use this exact result map:

```json
{
  "S01-01": true,
  "S01-02": true,
  "S01-03": true,
  "S01-04": true,
  "S01-05": true,
  "S01-06": true,
  "S01-07": true,
  "S01-08": true,
  "S01-09": true,
  "S01-10": true
}
```

Set status to `recomputed-package-gate-passed`. State that the report is S01
specification and local-verifier evidence only.

- [ ] **Step 5: Create and self-hash the evidence manifest**

Declare these immutable inputs with actual SHA-256 values:

- prompt version `1.3`
- approved S01 design
- semantic-scope index
- semantic-scope snapshot
- canonical atomic-swap source files

Declare every S01 evidence JSON except the manifest itself as an output. Include
its actual SHA-256 value and role. Include the schema and OpenSpec normative spec
as outputs.

Use this status:

```text
validated-theorem-freeze-at-S3
```

Use these limitations:

```text
The candidate theorem is not mechanized.
The effect verifier covers exact transfer effects for the atomic-swap falsifier only.
S01 does not establish Compact, ZKIR, proof-system, ledger, or wallet correspondence.
S01 does not establish ACTUS reference-vector compatibility.
The architecture choice remains open until S02.
```

Compute `manifest_sha256` over canonical JSON without that field. Insert the
result with `apply_patch`.

Use this read-only command to calculate the value:

```bash
uv run python - <<'PY'
import hashlib
import json
from pathlib import Path

path = Path("evidence/s01-intent-theorem-freeze/evidence-manifest.json")
value = json.loads(path.read_text(encoding="utf-8"))
value.pop("manifest_sha256", None)
payload = json.dumps(value, sort_keys=True, separators=(",", ":"))
print(hashlib.sha256(payload.encode("utf-8")).hexdigest())
PY
```

- [ ] **Step 6: Run the focused validator tests**

Run: `uv run pytest tests/test_s01_intent_evidence.py -q`

Expected: `5 passed`.

- [ ] **Step 7: Run the package validator directly**

Run: `uv run python scripts/validate_s01_intent_evidence.py`

Expected: exit code 0 and status `recomputed-package-gate-passed`.

- [ ] **Step 8: Commit the closed evidence gate**

```bash
git add scripts/validate_s01_intent_evidence.py tests/test_s01_intent_evidence.py evidence/s01-intent-theorem-freeze
git commit -m "feat: validate the S01 evidence gate"
```

### Task 6: Record the evidence-only scope transition

**Files:**

- Modify: `wiki/moriarty-architecture.md`
- Modify: `wiki/research-journal.md`
- Modify: `wiki/index.md`
- Modify: `openspec/changes/s01-intent-theorem-freeze/tasks.md`

**Interfaces:**

- Consumes: a passing S01 validator and unchanged semantic-scope files.
- Produces: a discoverable S01 record and an explicit S02 architecture handoff.

- [ ] **Step 1: Add the provisional-decision notice**

Add this meaning near the start of `wiki/moriarty-architecture.md`:

```text
The agreement-Core-plus-intent-envelope architecture is the current candidate.
It is not the prompt version 1.3 architecture freeze. S02 must compare all four
required architectures against the S01 intent-safety interface before selection.
```

Do not remove the prior recommendation or its evidence.

- [ ] **Step 2: Add the S01 journal entry**

Record these facts:

- The user approved the architecture-neutral design.
- S01 froze terminology, observations, hard predicates, preferences, assumptions, and the candidate theorem.
- The atomic-swap extra-effect mutant failed before signing.
- G17 now separates two human pilots from the ACTUS automated benchmark.
- The transition is evidence-only.
- Scope remains `0.0.0-e00.2` with the frozen digest.
- S02 owns architecture selection.

Label the theorem as `candidate-unmechanized`. Label the executable verifier as
an S3 experiment. Do not use proof language for either result.

- [ ] **Step 3: Link the S01 artifacts from the wiki index**

Add one link to the OpenSpec package. Add one link to the evidence manifest.
State the S01 evidence boundary in each description.

- [ ] **Step 4: Mark completed OpenSpec tasks**

Mark only actions that have passing repository evidence. Leave no completed box
for mechanization, backend correspondence, ledger execution, or ACTUS work.

- [ ] **Step 5: Run wiki and package checks**

Run: `uv run pytest tests/test_s01_openspec.py tests/test_s01_registries.py tests/test_intent_verifier.py tests/test_s01_intent_evidence.py -q`

Expected: `15 passed`.

Run: `uv run python scripts/validate_s01_intent_evidence.py`

Expected: exit code 0 and status `recomputed-package-gate-passed`.

- [ ] **Step 6: Commit the S01 handoff**

```bash
git add wiki/moriarty-architecture.md wiki/research-journal.md wiki/index.md openspec/changes/s01-intent-theorem-freeze/tasks.md
git commit -m "docs: record the S01 theorem freeze"
```

### Task 7: Verify S01 and prepare S02

**Files:**

- Verify only: all tracked repository files.
- Preserve: `evidence/marlowe-org-full-graph-2026-09-02.json`.

**Interfaces:**

- Consumes: all completed S01 tasks.
- Produces: final test evidence, a clean S01 diff, and the S02 architecture-comparison obligation.

- [ ] **Step 1: Run whitespace and placeholder checks**

Run: `git diff --check 110b69a..HEAD`

Expected: no output and exit code 0.

Run: `rg -n 'TB[D]|TO[D]O|FIXM[E]|implement late[r]|fill i[n]' openspec/changes/s01-intent-theorem-freeze schemas/intent evidence/s01-intent-theorem-freeze moriarty/intent.py scripts/validate_s01_intent_evidence.py`

Expected: no output and exit code 1.

- [ ] **Step 2: Run the S01 package validator**

Run: `uv run python scripts/validate_s01_intent_evidence.py`

Expected: every gate from `S01-01` through `S01-10` is `true`.

- [ ] **Step 3: Run the complete test suite**

Run: `uv run pytest -q`

Expected: `92 passed`. Record the exact count in the checkpoint measurement.

- [ ] **Step 4: Confirm Core and prior evidence did not change**

Run: `git diff 110b69a..HEAD -- moriarty/core.py moriarty/swap.py moriarty/compact.py moriarty/backend.py evidence/semantic-scope experiments/moriarty-core-swap`

Expected: no output. No Core, backend, scope, or E00 experiment file can appear.

- [ ] **Step 5: Confirm the user-owned file remains unstaged**

Run: `git status --short`

Expected: `evidence/marlowe-org-full-graph-2026-09-02.json` remains modified and
unstaged. No other uncommitted path remains.

- [ ] **Step 6: Record S01 completion and the next obligation**

Record the passing validator command, complete test count, commit, unchanged
scope digest, and S01 evidence-manifest path in the checkpoint database.

Close the S01 execution obligation. Add this next obligation:

```text
Execute S02 by model-checking all four architecture candidates against the frozen S01 intent-safety interface.
```

- [ ] **Step 7: Request a code review before S02**

Use `superpowers:requesting-code-review`. Give the reviewer the range from
`110b69a` through the S01 final commit. Require review of gate soundness,
effect-multiset handling, schema closure, digest closure, and Core immutability.
