"""Independent transition-manifest machine for the E00 Compact lowering."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any


Payment = tuple[str, str, str, int]


@dataclass(frozen=True)
class BackendInput:
    kind: str
    party: str
    now: int
    policy_id: str | None = None
    asset_name: str | None = None
    quantity: int | None = None
    choice_id: str | None = None
    chosen: int | None = None

    @classmethod
    def deposit(
        cls,
        party: str,
        policy_id: str,
        asset_name: str,
        quantity: int,
        *,
        now: int,
    ) -> BackendInput:
        return cls("deposit", party, now, policy_id, asset_name, quantity)

    @classmethod
    def choice(
        cls,
        party: str,
        choice_id: str,
        chosen: int,
        *,
        now: int,
    ) -> BackendInput:
        return cls("choice", party, now, choice_id=choice_id, chosen=chosen)


@dataclass(frozen=True)
class BackendState:
    phase: str = "WaitingAlice"
    alice_balance: int = 0
    bob_balance: int = 0
    choice: int | None = None
    min_time: int = 0


@dataclass(frozen=True)
class BackendResult:
    accepted: bool
    state: BackendState
    payments: tuple[Payment, ...] = ()
    warnings: tuple[str, ...] = ()
    error: str | None = None


class BackendMachine:
    def __init__(self, manifest: dict[str, Any]) -> None:
        parameters = manifest["parameters"]
        self.alice = str(parameters["alice"])
        self.bob = str(parameters["bob"])
        self.token_a = (
            str(parameters["token_a"]["policy_id"]),
            str(parameters["token_a"]["asset_name"]),
        )
        self.token_b = (
            str(parameters["token_b"]["policy_id"]),
            str(parameters["token_b"]["asset_name"]),
        )
        self.amount_a = int(parameters["amount_a"])
        self.amount_b = int(parameters["amount_b"])
        self.deadline = int(parameters["deadline"])
        self.choice_id = str(parameters["choice_id"])

    def _refunds(self, state: BackendState) -> tuple[Payment, ...]:
        payments: list[Payment] = []
        if state.alice_balance:
            payments.append((self.alice, *self.token_a, state.alice_balance))
        if state.bob_balance:
            payments.append((self.bob, *self.token_b, state.bob_balance))
        return tuple(payments)

    def apply(
        self,
        state: BackendState,
        supplied: BackendInput | None,
        *,
        now: int | None = None,
    ) -> BackendResult:
        current_time = supplied.now if supplied is not None else now
        if current_time is None:
            raise ValueError("a transaction time is required")
        if current_time < state.min_time:
            return BackendResult(False, state, error="time_before_state")

        terminal = state.phase in {"Settled", "Refunded"}
        if terminal:
            if supplied is not None:
                return BackendResult(False, state, error="contract_closed")
            return BackendResult(
                True,
                BackendState(
                    state.phase,
                    state.alice_balance,
                    state.bob_balance,
                    state.choice,
                    current_time,
                ),
            )

        if current_time >= self.deadline:
            if supplied is not None:
                return BackendResult(False, state, error="contract_closed")
            return BackendResult(
                True,
                BackendState("Refunded", choice=state.choice, min_time=current_time),
                payments=self._refunds(state),
            )

        if supplied is None:
            return BackendResult(False, state, error="input_required")

        if state.phase == "WaitingAlice":
            if not self._matches_deposit(
                supplied,
                self.alice,
                self.token_a,
                self.amount_a,
            ):
                return BackendResult(False, state, error="no_matching_input")
            return BackendResult(
                True,
                BackendState("WaitingBob", self.amount_a, min_time=current_time),
            )

        if state.phase == "WaitingBob":
            if not self._matches_deposit(
                supplied,
                self.bob,
                self.token_b,
                self.amount_b,
            ):
                return BackendResult(False, state, error="no_matching_input")
            return BackendResult(
                True,
                BackendState(
                    "WaitingDecision",
                    state.alice_balance,
                    self.amount_b,
                    min_time=current_time,
                ),
            )

        if state.phase == "WaitingDecision":
            if (
                supplied.kind != "choice"
                or supplied.party != self.bob
                or supplied.choice_id != self.choice_id
            ):
                return BackendResult(False, state, error="no_matching_input")
            if supplied.chosen not in {0, 1}:
                return BackendResult(False, state, error="choice_out_of_bounds")
            if supplied.chosen == 1:
                payments = (
                    (self.bob, *self.token_a, state.alice_balance),
                    (self.alice, *self.token_b, state.bob_balance),
                )
                phase = "Settled"
            else:
                payments = self._refunds(state)
                phase = "Refunded"
            return BackendResult(
                True,
                BackendState(phase, choice=supplied.chosen, min_time=current_time),
                payments=payments,
            )

        raise ValueError(f"unknown backend phase: {state.phase}")

    @staticmethod
    def _matches_deposit(
        supplied: BackendInput,
        party: str,
        token: tuple[str, str],
        quantity: int,
    ) -> bool:
        return (
            supplied.kind == "deposit"
            and supplied.party == party
            and (supplied.policy_id, supplied.asset_name) == token
            and supplied.quantity == quantity
            and quantity > 0
        )
