"""Canonical two-party atomic swap for the Moriarty E00 experiment."""

from __future__ import annotations

from dataclasses import dataclass

from moriarty.core import (
    Account,
    Choice,
    ChoiceEquals,
    Close,
    Constant,
    Deposit,
    If,
    Party,
    Pay,
    Token,
    When,
    case,
)


@dataclass(frozen=True)
class SwapParameters:
    alice: Party
    bob: Party
    token_a: Token
    token_b: Token
    amount_a: int
    amount_b: int
    deadline: int
    choice_id: str = "settle"

    def __post_init__(self) -> None:
        if self.alice == self.bob:
            raise ValueError("swap parties must be distinct")
        if self.token_a == self.token_b:
            raise ValueError("swap tokens must be distinct")
        if self.amount_a <= 0 or self.amount_b <= 0:
            raise ValueError("swap amounts must be positive")
        if self.amount_a >= 2**128 or self.amount_b >= 2**128:
            raise ValueError("swap amounts must fit Compact Uint<128>")
        if self.deadline <= 0:
            raise ValueError("swap deadline must be positive")
        if self.deadline >= 2**64:
            raise ValueError("swap deadline must fit Compact Uint<64>")
        if not self.choice_id:
            raise ValueError("swap choice identifier must not be empty")

    @property
    def alice_account(self) -> Account:
        return Account(self.alice, self.token_a)

    @property
    def bob_account(self) -> Account:
        return Account(self.bob, self.token_b)

    @classmethod
    def example(cls) -> SwapParameters:
        return cls(
            alice=Party("alice"),
            bob=Party("bob"),
            token_a=Token("aa", "A"),
            token_b=Token("bb", "B"),
            amount_a=10,
            amount_b=20,
            deadline=100,
        )


def canonical_swap(parameters: SwapParameters) -> When:
    settle = Pay(
        parameters.alice_account,
        parameters.bob,
        Constant(parameters.amount_a),
        Pay(
            parameters.bob_account,
            parameters.alice,
            Constant(parameters.amount_b),
            Close(),
        ),
    )
    decide = When(
        cases=(
            case(
                Choice(parameters.choice_id, parameters.bob, 0, 1),
                If(
                    ChoiceEquals(parameters.choice_id, 1),
                    settle,
                    Close(),
                ),
            ),
        ),
        timeout=parameters.deadline,
        timeout_continuation=Close(),
    )
    deposit_b = When(
        cases=(
            case(
                Deposit(
                    parameters.bob_account,
                    parameters.bob,
                    Constant(parameters.amount_b),
                ),
                decide,
            ),
        ),
        timeout=parameters.deadline,
        timeout_continuation=Close(),
    )
    return When(
        cases=(
            case(
                Deposit(
                    parameters.alice_account,
                    parameters.alice,
                    Constant(parameters.amount_a),
                ),
                deposit_b,
            ),
        ),
        timeout=parameters.deadline,
        timeout_continuation=Close(),
    )
