from __future__ import annotations

import json
from pathlib import Path

import pytest

from moriarty.core import ChoiceInput, DepositInput, State, compute_transaction
from moriarty.intent import Effect, EffectPolicy, effects_from_payments, verify_plan_effects
from moriarty.swap import SwapParameters, canonical_swap


ROOT = Path(__file__).parents[1]
EFFECT_KEYS = {
    "kind": "transfer",
    "source": "alice",
    "destination": "bob",
    "policy_id": "aa",
    "asset_name": "A",
    "quantity": 10,
}


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


def test_atomic_swap_effects_satisfy_the_exact_policy_without_authorizing_signing() -> None:
    effects = effects_from_payments(settle_swap().payments)
    certificate = verify_plan_effects(
        EffectPolicy(allowed=effects, required=effects),
        effects,
    )
    assert certificate.valid
    assert certificate.reason == "VALID"
    assert certificate.unauthorized_effects == ()
    assert certificate.missing_effects == ()
    assert certificate.signing_request_permitted is False


def test_additional_third_party_payment_is_rejected_before_signing() -> None:
    effects = effects_from_payments(settle_swap().payments)
    mutant = effects + (Effect("transfer", "alice", "mallory", "aa", "A", 1),)
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
    assert mutant[:-1] == baseline
    assert len(mutant) == len(baseline) + 1
    assert policy == EffectPolicy(allowed=baseline, required=baseline)
    assert verify_plan_effects(policy, mutant).reason == "UNAUTHORIZED_EXTRA_EFFECT"
    restored = verify_plan_effects(policy, mutant[:-1])
    assert restored.reason == "VALID"
    assert restored.signing_request_permitted is False
    assert vector["baseline_expected"] == {
        "valid": True,
        "reason": "VALID",
        "signing_request_permitted": False,
    }
    assert vector["mutant_expected"] == {
        "valid": False,
        "reason": "UNAUTHORIZED_EXTRA_EFFECT",
        "signing_request_permitted": False,
    }
    assert vector["resolution"] == "SignAfterResolve"
    assert vector["scope"] == "local-effect-check-only"
    assert vector["limitations"] == [
        "No authenticated-completeness guarantee.",
        "No other authorization checks are performed.",
    ]


def test_reordering_allowed_effects_is_valid() -> None:
    effects = effects_from_payments(settle_swap().payments)
    certificate = verify_plan_effects(
        EffectPolicy(allowed=effects, required=effects), tuple(reversed(effects))
    )
    assert certificate.reason == "VALID"


def test_duplicate_effect_requires_a_separate_permission() -> None:
    effects = effects_from_payments(settle_swap().payments)
    certificate = verify_plan_effects(
        EffectPolicy(allowed=effects, required=effects), effects + (effects[0],)
    )
    assert certificate.reason == "UNAUTHORIZED_EXTRA_EFFECT"
    assert certificate.unauthorized_effects == (effects[0],)


def test_missing_required_effect_reports_multiset_difference() -> None:
    effect = Effect.from_mapping(EFFECT_KEYS)
    policy = EffectPolicy(allowed=(effect, effect), required=(effect, effect))
    certificate = verify_plan_effects(policy, (effect,))
    assert certificate.reason == "MISSING_REQUIRED_EFFECT"
    assert certificate.missing_effects == (effect,)
    assert certificate.signing_request_permitted is False


def test_required_multiplicity_cannot_exceed_allowed() -> None:
    effect = Effect.from_mapping(EFFECT_KEYS)
    with pytest.raises(ValueError, match="required effects must be allowed"):
        EffectPolicy(allowed=(effect,), required=(effect, effect))


def test_empty_exact_policy_accepts_only_empty_actual_effects() -> None:
    policy = EffectPolicy(allowed=(), required=())
    certificate = verify_plan_effects(policy, ())
    assert certificate.reason == "VALID"
    assert certificate.signing_request_permitted is False


@pytest.mark.parametrize("field", EFFECT_KEYS)
def test_effect_mapping_requires_every_exact_key(field: str) -> None:
    malformed = dict(EFFECT_KEYS)
    malformed.pop(field)
    with pytest.raises(ValueError, match="exactly"):
        Effect.from_mapping(malformed)


def test_effect_mapping_rejects_unknown_keys() -> None:
    with pytest.raises(ValueError, match="exactly"):
        Effect.from_mapping({**EFFECT_KEYS, "memo": "surprise"})


@pytest.mark.parametrize("field", ("kind", "source", "destination", "policy_id", "asset_name"))
@pytest.mark.parametrize("bad_value", (None, 1, True, "", "  ", [], {}))
def test_effect_mapping_rejects_non_string_or_blank_identifiers(
    field: str, bad_value: object
) -> None:
    malformed = {**EFFECT_KEYS, field: bad_value}
    with pytest.raises((TypeError, ValueError)):
        Effect.from_mapping(malformed)


@pytest.mark.parametrize("bad_quantity", (True, 1.0, "1", None, [], {}, 0, -1))
def test_effect_mapping_rejects_non_positive_integer_quantities(
    bad_quantity: object,
) -> None:
    with pytest.raises((TypeError, ValueError)):
        Effect.from_mapping({**EFFECT_KEYS, "quantity": bad_quantity})


@pytest.mark.parametrize(
    "kwargs",
    (
        {"source": " "},
        {"destination": 1},
        {"policy_id": ""},
        {"asset_name": None},
        {"quantity": True},
        {"quantity": 0},
        {"kind": "mint"},
    ),
)
def test_direct_effect_construction_is_validated(kwargs: dict[str, object]) -> None:
    values = dict(EFFECT_KEYS)
    values.update(kwargs)
    with pytest.raises((TypeError, ValueError)):
        Effect(**values)


def test_policy_mapping_requires_exact_keys_and_immutable_sequences() -> None:
    effect = dict(EFFECT_KEYS)
    with pytest.raises(ValueError, match="exactly"):
        EffectPolicy.from_mapping({"allowed": [effect]})
    with pytest.raises(ValueError, match="exactly"):
        EffectPolicy.from_mapping({"allowed": [effect], "required": [effect], "extra": []})
    with pytest.raises(TypeError, match="array"):
        EffectPolicy.from_mapping({"allowed": "not-an-array", "required": []})
    with pytest.raises(TypeError, match="tuples"):
        EffectPolicy(allowed=[Effect.from_mapping(effect)], required=[])
