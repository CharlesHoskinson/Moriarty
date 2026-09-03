from __future__ import annotations

from dataclasses import replace

import pytest

from moriarty.backend import BackendInput, BackendMachine, BackendState
from moriarty.compact import lower_swap
from moriarty.swap import SwapParameters, canonical_swap


PARAMETERS = SwapParameters.example()


def test_lowerer_accepts_only_the_exact_canonical_shape() -> None:
    contract = canonical_swap(PARAMETERS)

    lowering = lower_swap(contract, PARAMETERS)
    changed = replace(contract, timeout=PARAMETERS.deadline + 1)

    assert lowering.manifest["core_sha256"]
    with pytest.raises(ValueError, match="canonical atomic-swap shape"):
        lower_swap(changed, PARAMETERS)


def test_generated_compact_has_no_unbounded_or_cross_contract_feature() -> None:
    source = lower_swap(canonical_swap(PARAMETERS), PARAMETERS).compact_source

    forbidden = (
        "Map<",
        "Set<",
        "List<",
        "Vector<",
        " for ",
        "while ",
        "Contract<",
        "crossContract",
    )
    assert all(term not in source for term in forbidden)
    assert "decision: Uint<0..2>" in source
    assert "if (disclose(decision) == 1)" in source
    assert "export enum Phase" in source


def test_every_witness_constraint_precedes_its_first_effect() -> None:
    source = lower_swap(canonical_swap(PARAMETERS), PARAMETERS).compact_source

    alice_constraint = source.index(
        "assert(aliceAuthority == authorityOfAliceWitness()"
    )
    bob_constraints = [
        index
        for index in range(len(source))
        if source.startswith(
            "assert(bobAuthority == authorityOfBobWitness()",
            index,
        )
    ]
    first_receive = source.index("receiveUnshielded")
    first_send = source.index("sendUnshielded")

    assert alice_constraint < first_receive
    assert len(bob_constraints) == 2
    assert bob_constraints[0] < source.index("receiveUnshielded", first_receive + 1)
    assert bob_constraints[1] < first_send


def test_manifest_names_effects_disclosures_witnesses_and_bounds() -> None:
    manifest = lower_swap(canonical_swap(PARAMETERS), PARAMETERS).manifest

    assert manifest["schema_version"] == 1
    assert manifest["status"] == "S3-experimental"
    assert manifest["bounds"] == {
        "max_inputs": 3,
        "max_live_accounts": 2,
        "max_payments": 2,
        "max_reductions": 3,
        "max_timeout": PARAMETERS.deadline,
        "syntax_nodes": 22,
    }
    assert {entry["name"] for entry in manifest["entry_points"]} == {
        "fundAlice",
        "fundBob",
        "decide",
        "expire",
    }
    assert {witness["name"] for witness in manifest["witnesses"]} == {
        "aliceSecret",
        "bobSecret",
    }
    assert manifest["disclosures"] == [
        "alice",
        "bob",
        "aliceAuthority",
        "bobAuthority",
        "tokenA",
        "tokenB",
        "amountA",
        "amountB",
        "deadline",
        "decision",
    ]
    assert len(manifest["compact_sha256"]) == 64


def test_manifest_machine_executes_success_and_deadline_refund() -> None:
    manifest = lower_swap(canonical_swap(PARAMETERS), PARAMETERS).manifest
    machine = BackendMachine(manifest)
    initial = BackendState()
    alice_deposit = BackendInput.deposit(
        PARAMETERS.alice.name,
        PARAMETERS.token_a.policy_id,
        PARAMETERS.token_a.asset_name,
        PARAMETERS.amount_a,
        now=5,
    )
    bob_deposit = BackendInput.deposit(
        PARAMETERS.bob.name,
        PARAMETERS.token_b.policy_id,
        PARAMETERS.token_b.asset_name,
        PARAMETERS.amount_b,
        now=6,
    )

    after_alice = machine.apply(initial, alice_deposit)
    assert after_alice.accepted
    assert after_alice.state.phase == "WaitingBob"

    timed_out = machine.apply(
        after_alice.state,
        None,
        now=PARAMETERS.deadline,
    )
    assert timed_out.accepted
    assert timed_out.state.phase == "Refunded"
    assert timed_out.payments == (
        (PARAMETERS.alice.name, PARAMETERS.token_a.policy_id, PARAMETERS.token_a.asset_name, PARAMETERS.amount_a),
    )

    after_bob = machine.apply(after_alice.state, bob_deposit)
    settled = machine.apply(
        after_bob.state,
        BackendInput.choice(PARAMETERS.bob.name, PARAMETERS.choice_id, 1, now=7),
    )
    assert settled.accepted
    assert settled.state.phase == "Settled"
    assert settled.payments == (
        (PARAMETERS.bob.name, PARAMETERS.token_a.policy_id, PARAMETERS.token_a.asset_name, PARAMETERS.amount_a),
        (PARAMETERS.alice.name, PARAMETERS.token_b.policy_id, PARAMETERS.token_b.asset_name, PARAMETERS.amount_b),
    )
