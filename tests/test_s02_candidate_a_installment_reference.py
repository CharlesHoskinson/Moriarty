"""Independent reference expectations for the proposed two-When composition.

This is a workload assembled from frozen Core constructors, not a new Core
feature. No candidate evaluator, projection, codec, or authorization is imported.
Passing these vectors is not Quint correspondence or recovery authorization.
"""

from __future__ import annotations

import pytest

from moriarty.core import (
    Account, Choice, ChoiceInput, Close, Constant, Party, Pay, Payment, State,
    Token, TransactionResult, When, case, compute_transaction,
)


ALICE, BOB = Party("alice"), Party("bob")
TOKEN_A = Token("aa", "A")
ALICE_A = Account(ALICE, TOKEN_A)
SECOND_WAIT = When(
    (
        case(Choice("fill2", BOB, 1, 1), Pay(ALICE_A, BOB, Constant(5), Close())),
        case(Choice("recover", ALICE, 1, 1), Close()),
    ),
    100,
    Close(),
)
INSTALLMENT = When(
    (
        case(Choice("fill1", BOB, 1, 1), Pay(ALICE_A, BOB, Constant(5), SECOND_WAIT)),
        case(Choice("recover", ALICE, 1, 1), Close()),
    ),
    100,
    Close(),
)
PREFUNDED = State(accounts=((ALICE_A, 10),), min_time=1)
AFTER_FIRST = State(accounts=((ALICE_A, 5),), choices=(("fill1", 1),), min_time=2)


def test_first_fill_stops_at_second_when_with_five_retained() -> None:
    assert compute_transaction(
        INSTALLMENT, PREFUNDED, ChoiceInput("fill1", BOB, 1), now=2
    ) == TransactionResult(
        True, AFTER_FIRST, SECOND_WAIT, (Payment(ALICE_A, BOB, TOKEN_A, 5),), (), 1
    )


def test_second_fill_requires_and_completes_a_second_transaction() -> None:
    first = compute_transaction(INSTALLMENT, PREFUNDED, ChoiceInput("fill1", BOB, 1), now=2)
    assert first.accepted and first.state == AFTER_FIRST and first.contract == SECOND_WAIT
    second = compute_transaction(first.contract, first.state, ChoiceInput("fill2", BOB, 1), now=2)
    assert second == TransactionResult(
        True, State(choices=(("fill1", 1), ("fill2", 1)), min_time=2), Close(),
        (Payment(ALICE_A, BOB, TOKEN_A, 5),), (), 1,
    )
    assert first.payments + second.payments == (
        Payment(ALICE_A, BOB, TOKEN_A, 5), Payment(ALICE_A, BOB, TOKEN_A, 5)
    )


@pytest.mark.parametrize("after_first", [False, True])
def test_agreement_recovery_choice_refunds_actual_ten_or_five(after_first: bool) -> None:
    contract, before = (SECOND_WAIT, AFTER_FIRST) if after_first else (INSTALLMENT, PREFUNDED)
    quantity = 5 if after_first else 10
    choices = (("fill1", 1), ("recover", 1)) if after_first else (("recover", 1),)
    assert compute_transaction(
        contract, before, ChoiceInput("recover", ALICE, 1), now=2
    ) == TransactionResult(
        True, State(choices=choices, min_time=2), Close(),
        (Payment(ALICE_A, ALICE, TOKEN_A, quantity),), (), 1,
    )
    # Agreement legality precedes and does not establish envelope authorization.
    # No cancellation flag or signing registry exists in these Core constructors.


@pytest.mark.parametrize("after_first", [False, True])
@pytest.mark.parametrize("now", [100, 101])
@pytest.mark.parametrize("supplied_recovery", [False, True])
def test_timeout_commits_without_input_but_rolls_back_supplied_recovery(
    after_first: bool, now: int, supplied_recovery: bool
) -> None:
    contract, before = (SECOND_WAIT, AFTER_FIRST) if after_first else (INSTALLMENT, PREFUNDED)
    supplied = ChoiceInput("recover", ALICE, 1) if supplied_recovery else None
    expected = (
        TransactionResult(False, before, contract, error="contract_closed")
        if supplied_recovery else TransactionResult(
            True, State(choices=before.choices, min_time=now), Close(),
            (Payment(ALICE_A, ALICE, TOKEN_A, 5 if after_first else 10),), (), 2,
        )
    )
    assert compute_transaction(contract, before, supplied, now=now) == expected


@pytest.mark.parametrize("after_first", [False, True])
@pytest.mark.parametrize("wrong", ["phase", "chooser", "bounds", "clock", "no_input"])
def test_invalid_installment_request_preserves_complete_original_state(
    after_first: bool, wrong: str
) -> None:
    contract, before = (SECOND_WAIT, AFTER_FIRST) if after_first else (INSTALLMENT, PREFUNDED)
    choice_id = "fill2" if after_first else "fill1"
    supplied = ChoiceInput(choice_id, BOB, 1)
    now, error = 2, "no_matching_input"
    if wrong == "phase":
        supplied = ChoiceInput("fill1" if after_first else "fill2", BOB, 1)
    elif wrong == "chooser":
        supplied = ChoiceInput(choice_id, ALICE, 1)
    elif wrong == "bounds":
        supplied = ChoiceInput(choice_id, BOB, 0)
        error = "choice_out_of_bounds"
    elif wrong == "clock":
        now, error = 0, "time_before_state"
    elif wrong == "no_input":
        supplied, error = None, "input_required"
    assert compute_transaction(contract, before, supplied, now=now) == TransactionResult(
        False, before, contract, error=error
    )
