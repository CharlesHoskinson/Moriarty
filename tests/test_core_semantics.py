from __future__ import annotations

from moriarty.core import (
    Account,
    Choice,
    ChoiceEquals,
    ChoiceInput,
    Close,
    Constant,
    Deposit,
    DepositInput,
    If,
    Party,
    Pay,
    State,
    Token,
    When,
    case,
    compute_transaction,
)


ALICE = Party("alice")
BOB = Party("bob")
TOKEN_A = Token("aa", "A")
TOKEN_B = Token("bb", "B")


def test_deposit_must_match_party_account_token_and_quantity() -> None:
    contract = When(
        cases=(
            case(
                Deposit(Account(ALICE, TOKEN_A), ALICE, Constant(10)),
                Close(),
            ),
        ),
        timeout=10,
        timeout_continuation=Close(),
    )

    wrong = compute_transaction(
        contract,
        State(),
        DepositInput(Account(ALICE, TOKEN_A), BOB, 10),
        now=9,
    )
    right = compute_transaction(
        contract,
        State(),
        DepositInput(Account(ALICE, TOKEN_A), ALICE, 10),
        now=9,
    )

    assert not wrong.accepted
    assert wrong.error == "no_matching_input"
    assert wrong.state == State()
    assert right.accepted
    assert right.payments[0].to == ALICE
    assert right.payments[0].quantity == 10


def test_timeout_wins_at_the_exact_deadline() -> None:
    contract = When(
        cases=(
            case(
                Deposit(Account(ALICE, TOKEN_A), ALICE, Constant(10)),
                Close(),
            ),
        ),
        timeout=10,
        timeout_continuation=Close(),
    )

    result = compute_transaction(
        contract,
        State(),
        DepositInput(Account(ALICE, TOKEN_A), ALICE, 10),
        now=10,
    )

    assert not result.accepted
    assert result.error == "contract_closed"
    assert result.state == State()


def test_choice_is_bounded_and_drives_if_reduction() -> None:
    choice = Choice("settle", BOB, lower_bound=0, upper_bound=1)
    contract = When(
        cases=(
            case(
                choice,
                If(ChoiceEquals("settle", 1), Close(), Close()),
            ),
        ),
        timeout=20,
        timeout_continuation=Close(),
    )

    rejected = compute_transaction(
        contract,
        State(),
        ChoiceInput("settle", BOB, 2),
        now=19,
    )
    accepted = compute_transaction(
        contract,
        State(),
        ChoiceInput("settle", BOB, 1),
        now=19,
    )

    assert not rejected.accepted
    assert rejected.error == "choice_out_of_bounds"
    assert accepted.accepted
    assert accepted.state.choice_value("settle") == 1
    assert isinstance(accepted.contract, Close)


def test_sequential_pays_reduce_in_one_transaction() -> None:
    contract = Pay(
        Account(ALICE, TOKEN_A),
        BOB,
        Constant(10),
        Pay(
            Account(BOB, TOKEN_B),
            ALICE,
            Constant(20),
            Close(),
        ),
    )
    state = State.from_accounts(
        {
            Account(ALICE, TOKEN_A): 10,
            Account(BOB, TOKEN_B): 20,
        }
    )

    result = compute_transaction(contract, state, None, now=5)

    assert result.accepted
    assert [(payment.to, payment.token, payment.quantity) for payment in result.payments] == [
        (BOB, TOKEN_A, 10),
        (ALICE, TOKEN_B, 20),
    ]
    assert result.state.accounts == ()
    assert result.reductions == 2


def test_close_refunds_accounts_in_deterministic_order() -> None:
    state = State.from_accounts(
        {
            Account(BOB, TOKEN_B): 7,
            Account(ALICE, TOKEN_A): 3,
        }
    )

    result = compute_transaction(Close(), state, None, now=0)

    assert result.accepted
    assert [(payment.to.name, payment.token.asset_name) for payment in result.payments] == [
        ("alice", "A"),
        ("bob", "B"),
    ]
    assert result.state.accounts == ()
    assert result.reductions == 2


def test_partial_payment_is_typed_and_conserves_the_account() -> None:
    contract = Pay(
        Account(ALICE, TOKEN_A),
        BOB,
        Constant(10),
        Close(),
    )
    state = State.from_accounts({Account(ALICE, TOKEN_A): 4})

    result = compute_transaction(contract, state, None, now=0)

    assert result.accepted
    assert result.payments[0].quantity == 4
    assert result.warnings[0].code == "partial_payment"
    assert result.warnings[0].requested == 10
    assert result.warnings[0].paid == 4
    assert result.state.accounts == ()


def test_time_cannot_move_backwards() -> None:
    state = State(min_time=8)

    result = compute_transaction(Close(), state, None, now=7)

    assert not result.accepted
    assert result.error == "time_before_state"
    assert result.state == state
