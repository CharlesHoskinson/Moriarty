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
        phases = manifest.get("phases")
        if not isinstance(phases, list) or not all(
            isinstance(phase, str) for phase in phases
        ):
            raise ValueError("manifest must define string phases")
        self.phases = frozenset(phases)
        if manifest.get("initial_phase") != "WaitingAlice":
            raise ValueError("manifest initial phase must be WaitingAlice")
        terminal_phases = manifest.get("terminal_phases")
        if not isinstance(terminal_phases, list):
            raise ValueError("manifest must define terminal phases")
        self.terminal_phases = frozenset(str(phase) for phase in terminal_phases)

        entries = manifest.get("entry_points")
        if not isinstance(entries, list) or len(entries) != 4:
            raise ValueError("manifest must define four transition entries")
        self.transitions = {
            str(entry.get("name")): entry
            for entry in entries
            if isinstance(entry, dict)
        }
        if set(self.transitions) != {"fundAlice", "fundBob", "decide", "expire"}:
            raise ValueError("manifest must define the four canonical transitions")
        for transition in self.transitions.values():
            sources = transition.get("from")
            source_phases = [sources] if isinstance(sources, str) else sources
            if not isinstance(source_phases, list) or not set(source_phases) <= self.phases:
                raise ValueError("manifest transition contains an unknown source phase")
            targets = transition.get("to")
            target_phases = list(targets.values()) if isinstance(targets, dict) else [targets]
            if not all(target in self.phases for target in target_phases):
                raise ValueError("manifest transition contains an unknown target phase")
            expected_input = transition.get("input")
            if not isinstance(expected_input, dict) or expected_input.get("kind") not in {
                "choice",
                "deposit",
                "expire",
            }:
                raise ValueError("manifest transition must define a supported input")
            if expected_input["kind"] == "choice":
                outcome_effects = transition.get("outcome_effects")
                if not isinstance(outcome_effects, dict):
                    raise ValueError("choice transition must define outcome effects")
                for effects in outcome_effects.values():
                    self._validate_effects(effects)
            else:
                self._validate_effects(transition.get("effects"))

        parameters = manifest["parameters"]
        self.deadline = int(parameters["deadline"])

    @staticmethod
    def _validate_effects(effects: object) -> None:
        if not isinstance(effects, list):
            raise ValueError("manifest transition effects must be a list")
        for effect in effects:
            if not isinstance(effect, dict) or effect.get("kind") not in {
                "credit",
                "pay_all",
            }:
                raise ValueError("manifest transition contains an unsupported effect")
            if effect.get("account") not in {"alice", "bob"}:
                raise ValueError("manifest effect contains an unknown account")

    def _target(self, transition_name: str, outcome: int | None = None) -> str:
        target = self.transitions[transition_name]["to"]
        if isinstance(target, dict):
            if outcome is None or str(outcome) not in target:
                raise ValueError("manifest transition does not define the outcome")
            return str(target[str(outcome)])
        return str(target)

    def _transition(self, phase: str, input_kind: str) -> dict[str, Any] | None:
        matches = []
        for transition in self.transitions.values():
            sources = transition["from"]
            source_phases = [sources] if isinstance(sources, str) else sources
            if phase in source_phases and transition["input"]["kind"] == input_kind:
                matches.append(transition)
        if len(matches) > 1:
            raise ValueError("manifest has ambiguous transitions")
        return matches[0] if matches else None

    @staticmethod
    def _matches_input(expected: dict[str, Any], supplied: BackendInput) -> bool:
        if expected["kind"] != supplied.kind:
            return False
        if supplied.kind == "deposit":
            return (
                supplied.party == expected.get("party")
                and supplied.policy_id == expected.get("policy_id")
                and supplied.asset_name == expected.get("asset_name")
                and supplied.quantity == expected.get("quantity")
                and int(expected.get("quantity", 0)) > 0
            )
        if supplied.kind == "choice":
            return (
                supplied.party == expected.get("party")
                and supplied.choice_id == expected.get("choice_id")
            )
        return supplied.kind == "expire"

    @staticmethod
    def _apply_effects(
        state: BackendState,
        effects: list[dict[str, Any]],
    ) -> tuple[dict[str, int], tuple[Payment, ...]]:
        balances = {
            "alice": state.alice_balance,
            "bob": state.bob_balance,
        }
        payments: list[Payment] = []
        for effect in effects:
            account = str(effect["account"])
            if effect["kind"] == "credit":
                balances[account] += int(effect["quantity"])
                continue
            quantity = balances[account]
            if quantity:
                token = effect["token"]
                payments.append(
                    (
                        str(effect["to"]),
                        str(token["policy_id"]),
                        str(token["asset_name"]),
                        quantity,
                    )
                )
                balances[account] = 0
        return balances, tuple(payments)

    def _apply_transition(
        self,
        transition: dict[str, Any],
        state: BackendState,
        *,
        current_time: int,
        choice: int | None = None,
    ) -> BackendResult:
        if choice is None:
            effects = transition["effects"]
        else:
            effects = transition["outcome_effects"][str(choice)]
        balances, payments = self._apply_effects(state, effects)
        return BackendResult(
            True,
            BackendState(
                self._target(str(transition["name"]), choice),
                balances["alice"],
                balances["bob"],
                state.choice if choice is None else choice,
                current_time,
            ),
            payments=payments,
        )

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

        terminal = state.phase in self.terminal_phases
        if terminal:
            return BackendResult(False, state, error="contract_closed")

        if current_time >= self.deadline:
            if supplied is not None:
                return BackendResult(False, state, error="contract_closed")
            transition = self._transition(state.phase, "expire")
            if transition is None:
                return BackendResult(False, state, error="contract_closed")
            return self._apply_transition(
                transition,
                state,
                current_time=current_time,
            )

        if supplied is None:
            return BackendResult(False, state, error="input_required")

        transition = self._transition(state.phase, supplied.kind)
        if transition is None or not self._matches_input(transition["input"], supplied):
            return BackendResult(False, state, error="no_matching_input")
        choice = supplied.chosen if supplied.kind == "choice" else None
        if choice is not None:
            lower = int(transition["input"]["lower"])
            upper = int(transition["input"]["upper"])
            if not lower <= choice <= upper:
                return BackendResult(False, state, error="choice_out_of_bounds")
        return self._apply_transition(
            transition,
            state,
            current_time=current_time,
            choice=choice,
        )
