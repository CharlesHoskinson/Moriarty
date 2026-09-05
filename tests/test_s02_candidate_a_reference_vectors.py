"""Independent frozen-reference vectors; not Quint correspondence evidence.

These tests do not import the candidate evaluator, codec, or projection. Complete
expected records are specified independently against the unchanged Python Core.
"""

from __future__ import annotations

from hashlib import sha256
from pathlib import Path

import pytest

from moriarty.core import (
    Account, Choice, ChoiceEquals, ChoiceInput, Close, Constant, Deposit,
    DepositInput, If, Party, Pay, Payment, ReductionResult, State, Token,
    TransactionResult, Warning, When, case, compute_transaction,
    reduce_to_quiescence,
)
from moriarty.swap import SwapParameters, canonical_swap


ROOT = Path(__file__).resolve().parents[1]
ALICE, BOB, MALLORY = Party("alice"), Party("bob"), Party("mallory")
TOKEN_A, TOKEN_B = Token("aa", "A"), Token("bb", "B")
ALICE_A, BOB_B = Account(ALICE, TOKEN_A), Account(BOB, TOKEN_B)
WAIT = When((), 100, Close())


@pytest.mark.parametrize(
    ("path", "digest"),
    [
        ("moriarty/core.py", "564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b"),
        ("moriarty/swap.py", "82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797"),
    ],
)
def test_frozen_reference_hash(path: str, digest: str) -> None:
    assert sha256((ROOT / path).read_bytes()).hexdigest() == digest


@pytest.mark.parametrize(
    ("requested", "balance", "paid", "warning"),
    [
        (10, 10, 10, None),
        (10, 4, 4, Warning("partial_payment", 10, 4)),
        (10, 0, 0, Warning("partial_payment", 10, 0)),
        (0, 5, 0, Warning("non_positive_payment", 0, 0)),
        (-1, 5, 0, Warning("non_positive_payment", -1, 0)),
    ],
)
def test_complete_pay_reduction(requested: int, balance: int, paid: int, warning: Warning | None) -> None:
    before = State.from_accounts({ALICE_A: balance}, min_time=2)
    contract = Pay(ALICE_A, BOB, Constant(requested), WAIT)
    expected = ReductionResult(
        State.from_accounts({ALICE_A: balance - paid}, min_time=2),
        WAIT,
        (Payment(ALICE_A, BOB, TOKEN_A, paid),) if paid else (),
        (warning,) if warning else (),
        1,
    )
    assert reduce_to_quiescence(contract, before) == expected


def test_close_refunds_all_six_accounts_in_reference_order() -> None:
    ordered = tuple(Account(party, token) for party in (ALICE, BOB, MALLORY) for token in (TOKEN_A, TOKEN_B))
    before = State.from_accounts({account: i for i, account in enumerate(reversed(ordered), 1)}, min_time=2)
    expected_payments = tuple(
        Payment(account, account.owner, account.token, 6 - i)
        for i, account in enumerate(ordered)
    )
    assert reduce_to_quiescence(Close(), before) == ReductionResult(
        State(min_time=2), Close(), expected_payments, (), 6
    )


@pytest.mark.parametrize("present", [False, True])
def test_absent_choice_and_stored_zero_choose_different_continuations(present: bool) -> None:
    before = State(choices=(("settle", 0),) if present else (), min_time=2)
    contract = If(ChoiceEquals("settle", 0), Close(), WAIT)
    assert reduce_to_quiescence(contract, before) == ReductionResult(
        before, Close() if present else WAIT, (), (), 1
    )


@pytest.mark.parametrize("overlapping", [False, True])
def test_choice_scan_uses_first_accepting_case_not_first_matching_identity(overlapping: bool) -> None:
    first = Choice("settle", BOB, 0, 1 if overlapping else 0)
    contract = When((case(first, WAIT), case(Choice("settle", BOB, 1, 1), Close())), 100, Close())
    result = compute_transaction(contract, State(min_time=1), ChoiceInput("settle", BOB, 1), now=2)
    assert result == TransactionResult(
        True, State(choices=(("settle", 1),), min_time=2), WAIT if overlapping else Close()
    )


def test_rejected_input_discards_speculative_warning_and_restores_every_field() -> None:
    waiting = When((case(Choice("settle", BOB, 0, 1), Close()),), 100, Close())
    contract = Pay(ALICE_A, BOB, Constant(0), waiting)
    before = State(accounts=((ALICE_A, 5),), choices=(("settle", 0),), min_time=1)
    result = compute_transaction(contract, before, ChoiceInput("settle", ALICE, 1), now=2)
    assert result == TransactionResult(False, before, contract, error="no_matching_input")


def test_pre_input_payment_prefix_requires_a_deposit_insertion_point() -> None:
    after_deposit = Pay(ALICE_A, BOB, Constant(5), Close())
    waiting = When((case(Deposit(ALICE_A, ALICE, Constant(5)), after_deposit),), 100, Close())
    contract = Pay(ALICE_A, BOB, Constant(1), waiting)
    before = State.from_accounts({ALICE_A: 1}, min_time=1)
    prefix = reduce_to_quiescence(contract, State.from_accounts({ALICE_A: 1}, min_time=2))
    assert prefix == ReductionResult(
        State(min_time=2), waiting, (Payment(ALICE_A, BOB, TOKEN_A, 1),), (), 1
    )
    result = compute_transaction(contract, before, DepositInput(ALICE_A, ALICE, 5), now=2)
    assert result == TransactionResult(
        True, State(min_time=2), Close(),
        (Payment(ALICE_A, BOB, TOKEN_A, 1), Payment(ALICE_A, BOB, TOKEN_A, 5)),
        (), 2,
    )
    # The deposit is between these payments, but absent from Core's Payment list.
    assert result.payments[:len(prefix.payments)] == prefix.payments


@pytest.mark.parametrize("supplied_input", [False, True])
def test_funded_swap_deadline_commit_and_complete_rollback(supplied_input: bool) -> None:
    contract = canonical_swap(SwapParameters.example()).cases[0].continuation.cases[0].continuation
    before = State.from_accounts({ALICE_A: 10, BOB_B: 20}, min_time=2)
    supplied = ChoiceInput("settle", BOB, 1) if supplied_input else None
    result = compute_transaction(contract, before, supplied, now=100)
    expected = (
        TransactionResult(False, before, contract, error="contract_closed")
        if supplied_input else TransactionResult(
            True, State(min_time=100), Close(),
            (Payment(ALICE_A, ALICE, TOKEN_A, 10), Payment(BOB_B, BOB, TOKEN_B, 20)),
            (), 3,
        )
    )
    assert result == expected


@pytest.mark.parametrize(
    ("required", "supplied", "error"),
    [(0, 0, "non_positive_deposit"), (-1, -1, "non_positive_deposit"), (10, 0, "no_matching_input")],
)
def test_nonpositive_deposit_error_requires_an_exact_case_match(required: int, supplied: int, error: str) -> None:
    contract = When((case(Deposit(ALICE_A, ALICE, Constant(required)), Close()),), 100, Close())
    before = State(choices=(("settle", 0),), min_time=1)
    assert compute_transaction(contract, before, DepositInput(ALICE_A, ALICE, supplied), now=2) == TransactionResult(
        False, before, contract, error=error
    )
