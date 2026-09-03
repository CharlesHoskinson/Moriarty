"""A small, total reference semantics for the Moriarty E00 experiment."""

from __future__ import annotations

from dataclasses import dataclass, replace
from typing import TypeAlias


@dataclass(frozen=True, order=True)
class Party:
    name: str

    def __post_init__(self) -> None:
        if not self.name:
            raise ValueError("party name must not be empty")


@dataclass(frozen=True, order=True)
class Token:
    policy_id: str
    asset_name: str


@dataclass(frozen=True, order=True)
class Account:
    owner: Party
    token: Token


@dataclass(frozen=True)
class Constant:
    quantity: int


@dataclass(frozen=True)
class ChoiceEquals:
    choice_id: str
    expected: int


Observation: TypeAlias = ChoiceEquals


@dataclass(frozen=True)
class Deposit:
    account: Account
    depositor: Party
    amount: Constant


@dataclass(frozen=True)
class Choice:
    choice_id: str
    chooser: Party
    lower_bound: int
    upper_bound: int

    def __post_init__(self) -> None:
        if self.lower_bound > self.upper_bound:
            raise ValueError("choice lower bound exceeds upper bound")


Action: TypeAlias = Deposit | Choice


@dataclass(frozen=True)
class Close:
    pass


@dataclass(frozen=True)
class Pay:
    account: Account
    payee: Party
    amount: Constant
    continuation: Contract


@dataclass(frozen=True)
class If:
    observation: Observation
    then_contract: Contract
    else_contract: Contract


@dataclass(frozen=True)
class Case:
    action: Action
    continuation: Contract


@dataclass(frozen=True)
class When:
    cases: tuple[Case, ...]
    timeout: int
    timeout_continuation: Contract


Contract: TypeAlias = Close | Pay | If | When


def case(action: Action, continuation: Contract) -> Case:
    return Case(action, continuation)


@dataclass(frozen=True)
class DepositInput:
    account: Account
    depositor: Party
    quantity: int


@dataclass(frozen=True)
class ChoiceInput:
    choice_id: str
    chooser: Party
    chosen: int


Input: TypeAlias = DepositInput | ChoiceInput


@dataclass(frozen=True)
class State:
    accounts: tuple[tuple[Account, int], ...] = ()
    choices: tuple[tuple[str, int], ...] = ()
    min_time: int = 0

    def __post_init__(self) -> None:
        if self.min_time < 0:
            raise ValueError("minimum time must be non-negative")
        if any(quantity <= 0 for _, quantity in self.accounts):
            raise ValueError("stored account quantities must be positive")
        if tuple(sorted(self.accounts)) != self.accounts:
            raise ValueError("accounts must use canonical order")
        if tuple(sorted(self.choices)) != self.choices:
            raise ValueError("choices must use canonical order")

    @classmethod
    def from_accounts(
        cls,
        accounts: dict[Account, int],
        *,
        min_time: int = 0,
    ) -> State:
        return cls(
            accounts=tuple(sorted((key, value) for key, value in accounts.items() if value)),
            min_time=min_time,
        )

    def account_balance(self, account: Account) -> int:
        return dict(self.accounts).get(account, 0)

    def choice_value(self, choice_id: str) -> int | None:
        return dict(self.choices).get(choice_id)

    def with_account(self, account: Account, quantity: int) -> State:
        if quantity < 0:
            raise ValueError("account quantity must not be negative")
        accounts = dict(self.accounts)
        if quantity == 0:
            accounts.pop(account, None)
        else:
            accounts[account] = quantity
        return replace(self, accounts=tuple(sorted(accounts.items())))

    def with_choice(self, choice_id: str, chosen: int) -> State:
        choices = dict(self.choices)
        choices[choice_id] = chosen
        return replace(self, choices=tuple(sorted(choices.items())))


@dataclass(frozen=True)
class Payment:
    source: Account
    to: Party
    token: Token
    quantity: int


@dataclass(frozen=True)
class Warning:
    code: str
    requested: int | None = None
    paid: int | None = None


@dataclass(frozen=True)
class ReductionResult:
    state: State
    contract: Contract
    payments: tuple[Payment, ...]
    warnings: tuple[Warning, ...]
    reductions: int


@dataclass(frozen=True)
class TransactionResult:
    accepted: bool
    state: State
    contract: Contract
    payments: tuple[Payment, ...] = ()
    warnings: tuple[Warning, ...] = ()
    reductions: int = 0
    error: str | None = None


def _evaluate(observation: Observation, state: State) -> bool:
    if isinstance(observation, ChoiceEquals):
        return state.choice_value(observation.choice_id) == observation.expected
    raise TypeError(f"unsupported observation: {type(observation).__name__}")


def reduce_to_quiescence(contract: Contract, state: State) -> ReductionResult:
    payments: list[Payment] = []
    warnings: list[Warning] = []
    reductions = 0
    current = contract
    current_state = state

    while True:
        if isinstance(current, Close):
            if not current_state.accounts:
                break
            account, balance = current_state.accounts[0]
            current_state = current_state.with_account(account, 0)
            payments.append(Payment(account, account.owner, account.token, balance))
            reductions += 1
            continue

        if isinstance(current, Pay):
            requested = current.amount.quantity
            balance = current_state.account_balance(current.account)
            paid = min(max(requested, 0), balance)
            if requested <= 0:
                warnings.append(Warning("non_positive_payment", requested, 0))
            elif paid < requested:
                warnings.append(Warning("partial_payment", requested, paid))
            if paid:
                current_state = current_state.with_account(current.account, balance - paid)
                payments.append(
                    Payment(current.account, current.payee, current.account.token, paid)
                )
            current = current.continuation
            reductions += 1
            continue

        if isinstance(current, If):
            current = (
                current.then_contract
                if _evaluate(current.observation, current_state)
                else current.else_contract
            )
            reductions += 1
            continue

        if isinstance(current, When) and current_state.min_time >= current.timeout:
            current = current.timeout_continuation
            reductions += 1
            continue

        if isinstance(current, When):
            break

        raise TypeError(f"unsupported contract: {type(current).__name__}")

    return ReductionResult(
        state=current_state,
        contract=current,
        payments=tuple(payments),
        warnings=tuple(warnings),
        reductions=reductions,
    )


def _apply_input(contract: When, state: State, supplied: Input) -> tuple[State, Contract, str | None]:
    choice_bounds_mismatch = False
    for candidate in contract.cases:
        action = candidate.action
        if isinstance(action, Deposit) and isinstance(supplied, DepositInput):
            if (
                action.account == supplied.account
                and action.depositor == supplied.depositor
                and action.amount.quantity == supplied.quantity
            ):
                if supplied.quantity <= 0:
                    return state, contract, "non_positive_deposit"
                balance = state.account_balance(action.account)
                return (
                    state.with_account(action.account, balance + supplied.quantity),
                    candidate.continuation,
                    None,
                )
        if isinstance(action, Choice) and isinstance(supplied, ChoiceInput):
            if action.choice_id == supplied.choice_id and action.chooser == supplied.chooser:
                if action.lower_bound <= supplied.chosen <= action.upper_bound:
                    return (
                        state.with_choice(action.choice_id, supplied.chosen),
                        candidate.continuation,
                        None,
                    )
                choice_bounds_mismatch = True
    return (
        state,
        contract,
        "choice_out_of_bounds" if choice_bounds_mismatch else "no_matching_input",
    )


def compute_transaction(
    contract: Contract,
    state: State,
    supplied: Input | None,
    *,
    now: int,
) -> TransactionResult:
    if now < state.min_time:
        return TransactionResult(False, state, contract, error="time_before_state")

    working_state = replace(state, min_time=now)
    before = reduce_to_quiescence(contract, working_state)

    if supplied is None:
        if isinstance(before.contract, When):
            return TransactionResult(False, state, contract, error="input_required")
        return TransactionResult(
            True,
            before.state,
            before.contract,
            before.payments,
            before.warnings,
            before.reductions,
        )

    if not isinstance(before.contract, When):
        error = "contract_closed" if isinstance(before.contract, Close) else "no_matching_input"
        return TransactionResult(False, state, contract, error=error)

    applied_state, continuation, error = _apply_input(before.contract, before.state, supplied)
    if error is not None:
        return TransactionResult(False, state, contract, error=error)

    after = reduce_to_quiescence(continuation, applied_state)
    return TransactionResult(
        True,
        after.state,
        after.contract,
        before.payments + after.payments,
        before.warnings + after.warnings,
        before.reductions + after.reductions,
    )
