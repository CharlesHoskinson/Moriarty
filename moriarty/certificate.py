"""Differential trace certificate for the Moriarty E00 stop test."""

from __future__ import annotations

import hashlib
import itertools
import json
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Callable

from moriarty.backend import BackendInput, BackendMachine, BackendState
from moriarty.compact import lower_swap
from moriarty.core import (
    Account,
    ChoiceInput,
    DepositInput,
    Party,
    State,
    Token,
    compute_transaction,
)
from moriarty.swap import SwapParameters, canonical_swap


REQUIRED_ACCEPTED_TRANSITIONS = frozenset(
    {
        "WaitingAlice:deposit->WaitingBob",
        "WaitingAlice:expire->Refunded",
        "WaitingBob:deposit->WaitingDecision",
        "WaitingBob:expire->Refunded",
        "WaitingDecision:choice-0->Refunded",
        "WaitingDecision:choice-1->Settled",
        "WaitingDecision:expire->Refunded",
    }
)
REQUIRED_REJECTION_CODES = frozenset(
    {
        "choice_out_of_bounds",
        "contract_closed",
        "input_required",
        "no_matching_input",
        "time_before_state",
    }
)
REQUIRED_TERMINAL_PHASES = frozenset({"Refunded", "Settled"})
REQUIRED_DEADLINE_OFFSETS = frozenset({-1, 0, 1})
REQUIRED_REJECTED_TRANSITIONS = frozenset(
    {
        "Refunded:expire->contract_closed",
        "Settled:expire->contract_closed",
    }
)
REQUIRED_BOUNDARY_CELLS = frozenset(
    {
        "WaitingAlice:deposit:-1->accepted",
        "WaitingAlice:deposit:0->contract_closed",
        "WaitingAlice:deposit:1->contract_closed",
        "WaitingAlice:expire:-1->input_required",
        "WaitingAlice:expire:0->accepted",
        "WaitingAlice:expire:1->accepted",
        "WaitingBob:deposit:-1->accepted",
        "WaitingBob:deposit:0->contract_closed",
        "WaitingBob:deposit:1->contract_closed",
        "WaitingBob:expire:-1->input_required",
        "WaitingBob:expire:0->accepted",
        "WaitingBob:expire:1->accepted",
        "WaitingDecision:choice:-1->accepted",
        "WaitingDecision:choice:0->contract_closed",
        "WaitingDecision:choice:1->contract_closed",
        "WaitingDecision:expire:-1->input_required",
        "WaitingDecision:expire:0->accepted",
        "WaitingDecision:expire:1->accepted",
    }
)
PRODUCER_MODULES = (
    "backend.py",
    "bounds.py",
    "certificate.py",
    "compact.py",
    "core.py",
    "swap.py",
)


@dataclass(frozen=True)
class Stimulus:
    kind: str
    now: int
    party: str = ""
    policy_id: str = ""
    asset_name: str = ""
    quantity: int | None = None
    choice_id: str = ""
    chosen: int | None = None

    @classmethod
    def deposit(
        cls,
        party: str,
        token: Token,
        quantity: int,
        now: int,
    ) -> Stimulus:
        return cls(
            "deposit",
            now,
            party=party,
            policy_id=token.policy_id,
            asset_name=token.asset_name,
            quantity=quantity,
        )

    @classmethod
    def choice(
        cls,
        party: str,
        choice_id: str,
        chosen: int,
        now: int,
    ) -> Stimulus:
        return cls(
            "choice",
            now,
            party=party,
            choice_id=choice_id,
            chosen=chosen,
        )

    @classmethod
    def expire(cls, now: int) -> Stimulus:
        return cls("expire", now)


@dataclass(frozen=True)
class Trace:
    steps: tuple[Stimulus, ...]


def _boundary_traces(parameters: SwapParameters) -> tuple[Trace, ...]:
    alice_early = Stimulus.deposit(
        parameters.alice.name,
        parameters.token_a,
        parameters.amount_a,
        max(0, parameters.deadline - 3),
    )
    bob_early = Stimulus.deposit(
        parameters.bob.name,
        parameters.token_b,
        parameters.amount_b,
        max(0, parameters.deadline - 2),
    )
    traces: list[Trace] = []
    for offset in (-1, 0, 1):
        boundary_time = parameters.deadline + offset
        traces.extend(
            (
                Trace(
                    (
                        Stimulus.deposit(
                            parameters.alice.name,
                            parameters.token_a,
                            parameters.amount_a,
                            boundary_time,
                        ),
                    )
                ),
                Trace((Stimulus.expire(boundary_time),)),
                Trace(
                    (
                        alice_early,
                        Stimulus.deposit(
                            parameters.bob.name,
                            parameters.token_b,
                            parameters.amount_b,
                            boundary_time,
                        ),
                    )
                ),
                Trace((alice_early, Stimulus.expire(boundary_time))),
                Trace(
                    (
                        alice_early,
                        bob_early,
                        Stimulus.choice(
                            parameters.bob.name,
                            parameters.choice_id,
                            1,
                            boundary_time,
                        ),
                    )
                ),
                Trace(
                    (
                        alice_early,
                        bob_early,
                        Stimulus.expire(boundary_time),
                    )
                ),
            )
        )
    return tuple(traces)


def _seed_traces(parameters: SwapParameters) -> tuple[Trace, ...]:
    action_time = max(0, parameters.deadline - 1)
    alice = Stimulus.deposit(
        parameters.alice.name,
        parameters.token_a,
        parameters.amount_a,
        action_time,
    )
    bob = Stimulus.deposit(
        parameters.bob.name,
        parameters.token_b,
        parameters.amount_b,
        action_time,
    )
    settle = Stimulus.choice(
        parameters.bob.name,
        parameters.choice_id,
        1,
        action_time,
    )
    cancel = Stimulus.choice(
        parameters.bob.name,
        parameters.choice_id,
        0,
        action_time,
    )
    baseline = (
        Trace((alice, bob, settle)),
        Trace((alice, bob, cancel)),
        Trace((Stimulus.expire(parameters.deadline),)),
        Trace((alice, Stimulus.expire(parameters.deadline))),
        Trace((alice, bob, Stimulus.expire(parameters.deadline))),
        Trace(
            (
                Stimulus.deposit(
                    parameters.bob.name,
                    parameters.token_a,
                    parameters.amount_a,
                    action_time,
                ),
            )
        ),
        Trace((Stimulus.expire(action_time),)),
        Trace(
            (
                Stimulus.expire(parameters.deadline),
                Stimulus.expire(action_time),
            )
        ),
        Trace(
            (
                alice,
                bob,
                Stimulus.choice(
                    parameters.bob.name,
                    parameters.choice_id,
                    2,
                    action_time,
                ),
            )
        ),
        Trace(
            (
                alice,
                bob,
                settle,
                Stimulus.deposit(
                    parameters.alice.name,
                    parameters.token_a,
                    parameters.amount_a,
                    parameters.deadline + 1,
                ),
            )
        ),
        Trace((alice, bob, settle, Stimulus.expire(parameters.deadline + 1))),
        Trace(
            (
                Stimulus.expire(parameters.deadline),
                Stimulus.expire(parameters.deadline + 1),
            )
        ),
    )
    return baseline + _boundary_traces(parameters)


def _alphabet(parameters: SwapParameters) -> tuple[Stimulus, ...]:
    action_time = max(0, parameters.deadline - 1)
    return (
        Stimulus.deposit(
            parameters.alice.name,
            parameters.token_a,
            parameters.amount_a,
            action_time,
        ),
        Stimulus.deposit(
            parameters.alice.name,
            parameters.token_a,
            parameters.amount_a + 1,
            action_time,
        ),
        Stimulus.deposit(
            parameters.bob.name,
            parameters.token_a,
            parameters.amount_a,
            action_time,
        ),
        Stimulus.deposit(
            parameters.bob.name,
            parameters.token_b,
            parameters.amount_b,
            action_time,
        ),
        Stimulus.deposit(
            parameters.alice.name,
            parameters.token_b,
            parameters.amount_b,
            action_time,
        ),
        Stimulus.choice(parameters.bob.name, parameters.choice_id, 0, action_time),
        Stimulus.choice(parameters.bob.name, parameters.choice_id, 1, action_time),
        Stimulus.choice(parameters.bob.name, parameters.choice_id, -1, action_time),
        Stimulus.choice(parameters.bob.name, parameters.choice_id, 2, action_time),
        Stimulus.choice(parameters.alice.name, parameters.choice_id, 1, action_time),
        Stimulus.choice(parameters.bob.name, "wrong-choice", 1, action_time),
        Stimulus.expire(action_time),
        Stimulus.expire(parameters.deadline),
        Stimulus.expire(parameters.deadline + 1),
    )


def generate_traces(
    parameters: SwapParameters,
    *,
    minimum: int,
) -> tuple[Trace, ...]:
    if minimum <= 0:
        raise ValueError("minimum trace count must be positive")
    ordered: list[Trace] = []
    seen: set[tuple[Stimulus, ...]] = set()

    def add(trace: Trace) -> None:
        if trace.steps not in seen and len(ordered) < minimum:
            seen.add(trace.steps)
            ordered.append(trace)

    for trace in _seed_traces(parameters):
        add(trace)
    alphabet = _alphabet(parameters)
    for length in range(1, 6):
        for steps in itertools.product(alphabet, repeat=length):
            add(Trace(steps))
            if len(ordered) == minimum:
                return tuple(ordered)
    raise ValueError(f"could generate only {len(ordered)} distinct traces")


def _core_input(stimulus: Stimulus):
    if stimulus.kind == "expire":
        return None
    if stimulus.kind == "deposit":
        token = Token(stimulus.policy_id, stimulus.asset_name)
        return DepositInput(
            Account(Party(stimulus.party), token),
            Party(stimulus.party),
            int(stimulus.quantity),
        )
    if stimulus.kind == "choice":
        return ChoiceInput(
            stimulus.choice_id,
            Party(stimulus.party),
            int(stimulus.chosen),
        )
    raise ValueError(f"unknown stimulus kind: {stimulus.kind}")


def _backend_input(stimulus: Stimulus) -> BackendInput:
    if stimulus.kind == "expire":
        return BackendInput.expire(now=stimulus.now)
    if stimulus.kind == "deposit":
        return BackendInput.deposit(
            stimulus.party,
            stimulus.policy_id,
            stimulus.asset_name,
            int(stimulus.quantity),
            now=stimulus.now,
        )
    if stimulus.kind == "choice":
        return BackendInput.choice(
            stimulus.party,
            stimulus.choice_id,
            int(stimulus.chosen),
            now=stimulus.now,
        )
    raise ValueError(f"unknown stimulus kind: {stimulus.kind}")


def _next_phase(phase: str, stimulus: Stimulus, accepted: bool) -> str:
    if not accepted:
        return phase
    if stimulus.kind == "expire" and phase not in {"Settled", "Refunded"}:
        return "Refunded"
    if phase == "WaitingAlice" and stimulus.kind == "deposit":
        return "WaitingBob"
    if phase == "WaitingBob" and stimulus.kind == "deposit":
        return "WaitingDecision"
    if phase == "WaitingDecision" and stimulus.kind == "choice":
        return "Settled" if stimulus.chosen == 1 else "Refunded"
    return phase


def _core_observation(result, phase: str) -> dict:
    return {
        "accepted": result.accepted,
        "error": result.error,
        "phase": phase,
        "accounts": [
            [account.owner.name, account.token.policy_id, account.token.asset_name, amount]
            for account, amount in result.state.accounts
        ],
        "choices": [[choice_id, chosen] for choice_id, chosen in result.state.choices],
        "payments": [
            [
                payment.to.name,
                payment.token.policy_id,
                payment.token.asset_name,
                payment.quantity,
            ]
            for payment in result.payments
        ],
        "warnings": [
            [warning.code, warning.requested, warning.paid]
            for warning in result.warnings
        ],
        "min_time": result.state.min_time,
    }


def _backend_observation(
    result,
    parameters: SwapParameters,
) -> dict:
    accounts = []
    if result.state.alice_balance:
        accounts.append(
            [
                parameters.alice.name,
                parameters.token_a.policy_id,
                parameters.token_a.asset_name,
                result.state.alice_balance,
            ]
        )
    if result.state.bob_balance:
        accounts.append(
            [
                parameters.bob.name,
                parameters.token_b.policy_id,
                parameters.token_b.asset_name,
                result.state.bob_balance,
            ]
        )
    accounts.sort()
    choices = (
        []
        if result.state.choice is None
        else [[parameters.choice_id, result.state.choice]]
    )
    return {
        "accepted": result.accepted,
        "error": result.error,
        "phase": result.state.phase,
        "accounts": accounts,
        "choices": choices,
        "payments": [list(payment) for payment in result.payments],
        "warnings": [[warning, None, None] for warning in result.warnings],
        "min_time": result.state.min_time,
    }


def _transition_label(before: str, stimulus: Stimulus, after: str) -> str:
    if stimulus.kind == "choice":
        action = f"choice-{stimulus.chosen}"
    else:
        action = stimulus.kind
    return f"{before}:{action}->{after}"


def _canonical_json(value) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def _sha256_json(value) -> str:
    return hashlib.sha256(_canonical_json(value).encode("utf-8")).hexdigest()


def _producer_hashes() -> dict[str, str]:
    package = Path(__file__).resolve().parent
    return {
        f"moriarty/{filename}": hashlib.sha256(
            (package / filename).read_bytes()
        ).hexdigest()
        for filename in PRODUCER_MODULES
    }


def validate_translation(
    parameters: SwapParameters,
    traces: tuple[Trace, ...],
    *,
    machine_factory: Callable[[dict], BackendMachine] = BackendMachine,
) -> dict:
    lowering = lower_swap(canonical_swap(parameters), parameters)
    accepted_transitions: set[str] = set()
    rejection_codes: set[str] = set()
    rejected_transitions: set[str] = set()
    terminal_phases: set[str] = set()
    deadline_offsets: set[int] = set()
    boundary_cells: set[str] = set()
    divergence_count = 0
    invariant_failure_count = 0
    first_divergence = None

    for trace_index, trace in enumerate(traces):
        contract = canonical_swap(parameters)
        core_state = State()
        core_phase = "WaitingAlice"
        backend_state = BackendState()
        machine = machine_factory(lowering.manifest)
        deposits: dict[tuple[str, str], int] = {}
        payments: dict[tuple[str, str], int] = {}

        for step_index, stimulus in enumerate(trace.steps):
            if abs(stimulus.now - parameters.deadline) <= 1:
                deadline_offsets.add(stimulus.now - parameters.deadline)
            before_phase = core_phase
            core_result = compute_transaction(
                contract,
                core_state,
                _core_input(stimulus),
                now=stimulus.now,
            )
            core_phase = _next_phase(core_phase, stimulus, core_result.accepted)
            backend_supplied = _backend_input(stimulus)
            backend_result = machine.apply(backend_state, backend_supplied)

            core_observation = _core_observation(core_result, core_phase)
            backend_observation = _backend_observation(backend_result, parameters)
            if abs(stimulus.now - parameters.deadline) <= 1:
                result_label = (
                    "accepted" if core_result.accepted else str(core_result.error)
                )
                boundary_cells.add(
                    f"{before_phase}:{stimulus.kind}:"
                    f"{stimulus.now - parameters.deadline}->{result_label}"
                )
            if core_observation != backend_observation:
                divergence_count += 1
                if first_divergence is None:
                    first_divergence = {
                        "trace_index": trace_index,
                        "step_index": step_index,
                        "stimulus": asdict(stimulus),
                        "core": core_observation,
                        "backend": backend_observation,
                    }

            if core_result.accepted:
                if core_phase != before_phase:
                    accepted_transitions.add(
                        _transition_label(before_phase, stimulus, core_phase)
                    )
                if stimulus.kind == "deposit":
                    key = (stimulus.policy_id, stimulus.asset_name)
                    deposits[key] = deposits.get(key, 0) + int(stimulus.quantity)
                for payment in core_result.payments:
                    key = (payment.token.policy_id, payment.token.asset_name)
                    payments[key] = payments.get(key, 0) + payment.quantity
                balances: dict[tuple[str, str], int] = {}
                for account, quantity in core_result.state.accounts:
                    key = (account.token.policy_id, account.token.asset_name)
                    balances[key] = balances.get(key, 0) + quantity
                if set(deposits) | set(payments) | set(balances):
                    for key in set(deposits) | set(payments) | set(balances):
                        if deposits.get(key, 0) != payments.get(key, 0) + balances.get(key, 0):
                            invariant_failure_count += 1
                if any(quantity < 0 for _, quantity in core_result.state.accounts):
                    invariant_failure_count += 1
                contract = core_result.contract
                core_state = core_result.state
                backend_state = backend_result.state
            elif core_result.error is not None:
                rejection_codes.add(core_result.error)
                rejected_transitions.add(
                    f"{before_phase}:{stimulus.kind}->{core_result.error}"
                )

            if core_phase in {"Settled", "Refunded"}:
                terminal_phases.add(core_phase)

    trace_data = [[asdict(step) for step in trace.steps] for trace in traces]
    manifest_sha256 = _sha256_json(lowering.manifest)
    required_coverage = {
        "accepted_transitions": sorted(REQUIRED_ACCEPTED_TRANSITIONS),
        "rejection_codes": sorted(REQUIRED_REJECTION_CODES),
        "terminal_phases": sorted(REQUIRED_TERMINAL_PHASES),
        "deadline_offsets": sorted(REQUIRED_DEADLINE_OFFSETS),
        "rejected_transitions": sorted(REQUIRED_REJECTED_TRANSITIONS),
        "boundary_cells": sorted(REQUIRED_BOUNDARY_CELLS),
    }
    certificate = {
        "schema_version": 1,
        "status": "S3-translation-validation-evidence",
        "research_date": "2026-09-03",
        "trace_algorithm": "seeded-boundary-prefix-plus-lexicographic-product-v2",
        "trace_count": len(traces),
        "unique_trace_count": len({trace.steps for trace in traces}),
        "trace_corpus_sha256": _sha256_json(trace_data),
        "producer_source_sha256": _producer_hashes(),
        "core_sha256": lowering.manifest["core_sha256"],
        "compact_sha256": lowering.manifest["compact_sha256"],
        "manifest_sha256": manifest_sha256,
        "divergence_count": divergence_count,
        "first_divergence": first_divergence,
        "invariant_failure_count": invariant_failure_count,
        "coverage": {
            "accepted_transitions": sorted(accepted_transitions),
            "rejection_codes": sorted(rejection_codes),
            "terminal_phases": sorted(terminal_phases),
            "deadline_offsets": sorted(deadline_offsets),
            "rejected_transitions": sorted(rejected_transitions),
            "boundary_cells": sorted(boundary_cells),
        },
        "required_coverage": required_coverage,
        "claims_not_established": [
            "Compact compiler correctness",
            "ZKIR correctness or proof soundness",
            "proof generation or network deployment",
            "fees, proving latency, or production security",
        ],
    }
    certificate["stop_test_passed"] = (
        len(traces) >= 1_000
        and len({trace.steps for trace in traces}) == len(traces)
        and divergence_count == 0
        and invariant_failure_count == 0
        and REQUIRED_ACCEPTED_TRANSITIONS <= accepted_transitions
        and REQUIRED_REJECTION_CODES <= rejection_codes
        and REQUIRED_TERMINAL_PHASES <= terminal_phases
        and REQUIRED_DEADLINE_OFFSETS <= deadline_offsets
        and REQUIRED_REJECTED_TRANSITIONS <= rejected_transitions
        and REQUIRED_BOUNDARY_CELLS <= boundary_cells
    )
    certificate["certificate_sha256"] = _sha256_json(certificate)
    return certificate


def write_experiment(
    output_directory: Path,
    parameters: SwapParameters,
    *,
    minimum_traces: int,
) -> dict:
    output_directory.mkdir(parents=True, exist_ok=True)
    lowering = lower_swap(canonical_swap(parameters), parameters)
    traces = generate_traces(parameters, minimum=minimum_traces)
    certificate = validate_translation(parameters, traces)
    (output_directory / "swap.compact").write_text(
        lowering.compact_source,
        encoding="utf-8",
    )
    (output_directory / "artifact-manifest.json").write_text(
        json.dumps(lowering.manifest, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    (output_directory / "translation-certificate.json").write_text(
        json.dumps(certificate, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    return certificate
