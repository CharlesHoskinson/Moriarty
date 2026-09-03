from __future__ import annotations

from collections import Counter

from moriarty.bounds import analyze_bounds
from moriarty.core import ChoiceInput, DepositInput, State, compute_transaction
from moriarty.swap import SwapParameters, canonical_swap


PARAMETERS = SwapParameters.example()


def apply_valid_prefix(length: int):
    contract = canonical_swap(PARAMETERS)
    state = State()
    results = []
    inputs = (
        DepositInput(PARAMETERS.alice_account, PARAMETERS.alice, PARAMETERS.amount_a),
        DepositInput(PARAMETERS.bob_account, PARAMETERS.bob, PARAMETERS.amount_b),
        ChoiceInput(PARAMETERS.choice_id, PARAMETERS.bob, 1),
    )
    for supplied in inputs[:length]:
        result = compute_transaction(contract, state, supplied, now=5)
        assert result.accepted
        results.append(result)
        state = result.state
        contract = result.contract
    return state, contract, results


def test_successful_swap_is_atomic_and_conserves_each_token() -> None:
    state, contract, results = apply_valid_prefix(3)
    settlement = results[-1]

    assert state.accounts == ()
    assert len(settlement.payments) == 2
    assert {(payment.to, payment.token, payment.quantity) for payment in settlement.payments} == {
        (PARAMETERS.bob, PARAMETERS.token_a, PARAMETERS.amount_a),
        (PARAMETERS.alice, PARAMETERS.token_b, PARAMETERS.amount_b),
    }
    paid = Counter()
    for payment in settlement.payments:
        paid[payment.token] += payment.quantity
    assert paid == Counter(
        {
            PARAMETERS.token_a: PARAMETERS.amount_a,
            PARAMETERS.token_b: PARAMETERS.amount_b,
        }
    )
    assert contract.__class__.__name__ == "Close"


def test_zero_choice_refunds_both_deposits() -> None:
    state, contract, _ = apply_valid_prefix(2)

    result = compute_transaction(
        contract,
        state,
        ChoiceInput(PARAMETERS.choice_id, PARAMETERS.bob, 0),
        now=6,
    )

    assert result.accepted
    assert [(payment.to, payment.token, payment.quantity) for payment in result.payments] == [
        (PARAMETERS.alice, PARAMETERS.token_a, PARAMETERS.amount_a),
        (PARAMETERS.bob, PARAMETERS.token_b, PARAMETERS.amount_b),
    ]
    assert result.state.accounts == ()


def test_one_deadline_refunds_every_nonterminal_phase() -> None:
    expected_refunds = {
        0: [],
        1: [(PARAMETERS.alice, PARAMETERS.token_a, PARAMETERS.amount_a)],
        2: [
            (PARAMETERS.alice, PARAMETERS.token_a, PARAMETERS.amount_a),
            (PARAMETERS.bob, PARAMETERS.token_b, PARAMETERS.amount_b),
        ],
    }
    for prefix_length in range(3):
        state, contract, _ = apply_valid_prefix(prefix_length)

        result = compute_transaction(
            contract,
            state,
            None,
            now=PARAMETERS.deadline,
        )

        assert result.accepted
        assert [
            (payment.to, payment.token, payment.quantity)
            for payment in result.payments
        ] == expected_refunds[prefix_length]
        assert result.state.accounts == ()


def test_static_bounds_cover_the_complete_swap_tree() -> None:
    bounds = analyze_bounds(canonical_swap(PARAMETERS))

    assert bounds.max_inputs == 3
    assert bounds.max_reductions == 3
    assert bounds.max_payments == 2
    assert bounds.max_live_accounts == 2
    assert bounds.max_timeout == PARAMETERS.deadline
    assert bounds.syntax_nodes == 22
