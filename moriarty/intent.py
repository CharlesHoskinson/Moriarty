"""Architecture-neutral local intent-effect verification for S01."""

from __future__ import annotations

from collections import Counter
from collections.abc import Iterable, Mapping, Sequence
from dataclasses import dataclass

from moriarty.core import Payment


_EFFECT_KEYS = frozenset(
    {"kind", "source", "destination", "policy_id", "asset_name", "quantity"}
)
_POLICY_KEYS = frozenset({"allowed", "required"})


def _require_exact_keys(value: Mapping[str, object], expected: frozenset[str]) -> None:
    if frozenset(value.keys()) != expected:
        raise ValueError(f"mapping keys must be exactly {sorted(expected)}")


@dataclass(frozen=True, order=True)
class Effect:
    kind: str
    source: str
    destination: str
    policy_id: str
    asset_name: str
    quantity: int

    def __post_init__(self) -> None:
        for name, value in (
            ("kind", self.kind),
            ("source", self.source),
            ("destination", self.destination),
            ("policy_id", self.policy_id),
            ("asset_name", self.asset_name),
        ):
            if not isinstance(value, str):
                raise TypeError(f"{name} must be a string")
            if not value.strip():
                raise ValueError(f"{name} must not be blank")
        if self.kind != "transfer":
            raise ValueError("S01 supports only transfer effects")
        if type(self.quantity) is not int:
            raise TypeError("effect quantity must be an integer")
        if self.quantity <= 0:
            raise ValueError("effect quantity must be positive")

    @classmethod
    def from_mapping(cls, value: Mapping[str, object]) -> Effect:
        if not isinstance(value, Mapping):
            raise TypeError("effect must be a mapping")
        _require_exact_keys(value, _EFFECT_KEYS)
        return cls(
            kind=value["kind"],
            source=value["source"],
            destination=value["destination"],
            policy_id=value["policy_id"],
            asset_name=value["asset_name"],
            quantity=value["quantity"],
        )


@dataclass(frozen=True)
class EffectPolicy:
    allowed: tuple[Effect, ...]
    required: tuple[Effect, ...]

    def __post_init__(self) -> None:
        if not isinstance(self.allowed, tuple) or not isinstance(self.required, tuple):
            raise TypeError("direct policy construction requires tuples")
        if not all(isinstance(effect, Effect) for effect in self.allowed + self.required):
            raise TypeError("policy entries must be Effect values")
        if Counter(self.required) - Counter(self.allowed):
            raise ValueError("required effects must be allowed")

    @classmethod
    def from_mapping(cls, value: Mapping[str, object]) -> EffectPolicy:
        if not isinstance(value, Mapping):
            raise TypeError("policy must be a mapping")
        _require_exact_keys(value, _POLICY_KEYS)
        allowed = value["allowed"]
        required = value["required"]
        for name, items in (("allowed", allowed), ("required", required)):
            if not isinstance(items, Sequence) or isinstance(items, (str, bytes, bytearray)):
                raise TypeError(f"policy {name} must be an array")
        return cls(
            allowed=tuple(Effect.from_mapping(item) for item in allowed),
            required=tuple(Effect.from_mapping(item) for item in required),
        )


@dataclass(frozen=True)
class VerificationCertificate:
    """A local effect-comparison result, never a signing authorization."""

    valid: bool
    reason: str
    unauthorized_effects: tuple[Effect, ...] = ()
    missing_effects: tuple[Effect, ...] = ()

    @property
    def signing_request_permitted(self) -> bool:
        return False


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
