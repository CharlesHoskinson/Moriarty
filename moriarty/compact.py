"""Restricted Moriarty Core to Compact lowering for experiment E00."""

from __future__ import annotations

import hashlib
import json
from dataclasses import asdict, dataclass, fields, is_dataclass
from typing import Any

from moriarty.bounds import analyze_bounds
from moriarty.core import Contract
from moriarty.swap import SwapParameters, canonical_swap


TOOLCHAIN = {
    "compact_compiler": "0.34.100",
    "compact_language": "0.26.0",
    "compact_runtime": "0.19.100",
    "midnight_ledger": "9.1.0.0-rc.3",
    "zkir": "3",
}


@dataclass(frozen=True)
class Lowering:
    compact_source: str
    manifest: dict[str, Any]


def _data(value: Any) -> Any:
    if is_dataclass(value) and not isinstance(value, type):
        return {
            "type": value.__class__.__name__,
            **{field.name: _data(getattr(value, field.name)) for field in fields(value)},
        }
    if isinstance(value, tuple):
        return [_data(item) for item in value]
    if isinstance(value, (str, int, bool)) or value is None:
        return value
    raise TypeError(f"cannot canonically encode {type(value).__name__}")


def _canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=True,
    ).encode("utf-8")


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def disclosure_negative_control(source: str) -> str:
    disclosed = "if (disclose(decision) == 1)"
    if source.count(disclosed) != 1:
        raise ValueError("Compact source lacks one canonical decision disclosure")
    return source.replace(disclosed, "if (decision == 1)")


def _compact_source(parameters: SwapParameters) -> str:
    refund_sends = {
        parameters.alice_account: (
            "    sendUnshielded(tokenA, amountA, "
            "right<ContractAddress, UserAddress>(alice));"
        ),
        parameters.bob_account: (
            "    sendUnshielded(tokenB, amountB, "
            "right<ContractAddress, UserAddress>(bob));"
        ),
    }
    ordered_refunds = "\n".join(
        refund_sends[account] for account in sorted(refund_sends)
    )
    source = """// Generated from the canonical Moriarty Core E00 atomic swap.
// This code is experimental and has not received an independent audit.

pragma language_version >= 0.26 && <= 0.26;
import CompactStandardLibrary;

export enum Phase { WaitingAlice, WaitingBob, WaitingDecision, Settled, Refunded }
export new type PartySecret = Bytes<32>;

export sealed ledger alice: UserAddress;
export sealed ledger bob: UserAddress;
export sealed ledger aliceAuthority: Bytes<32>;
export sealed ledger bobAuthority: Bytes<32>;
export sealed ledger tokenA: Bytes<32>;
export sealed ledger tokenB: Bytes<32>;
export sealed ledger amountA: Uint<128>;
export sealed ledger amountB: Uint<128>;
export sealed ledger deadline: Uint<64>;
export ledger phase: Phase;

witness aliceSecret(): PartySecret;
witness bobSecret(): PartySecret;

constructor(
  initialAlice: UserAddress,
  initialBob: UserAddress,
  initialAliceAuthority: Bytes<32>,
  initialBobAuthority: Bytes<32>,
  initialTokenA: Bytes<32>,
  initialTokenB: Bytes<32>,
  initialAmountA: Uint<128>,
  initialAmountB: Uint<128>,
  initialDeadline: Uint<64>
) {
  alice = disclose(initialAlice);
  bob = disclose(initialBob);
  aliceAuthority = disclose(initialAliceAuthority);
  bobAuthority = disclose(initialBobAuthority);
  tokenA = disclose(initialTokenA);
  tokenB = disclose(initialTokenB);
  amountA = disclose(initialAmountA);
  amountB = disclose(initialAmountB);
  deadline = disclose(initialDeadline);
  assert(alice != bob, "swap parties must be distinct");
  assert(amountA > 0, "token A amount must be positive");
  assert(amountB > 0, "token B amount must be positive");
  assert(tokenA != tokenB, "swap tokens must be distinct");
  assert(deadline > 0, "swap deadline must be positive");
  phase = Phase.WaitingAlice;
}

circuit authorityOfAliceWitness(): Bytes<32> {
  return persistentHash<PartySecret>(aliceSecret());
}

circuit authorityOfBobWitness(): Bytes<32> {
  return persistentHash<PartySecret>(bobSecret());
}

export circuit fundAlice(): [] {
  assert(phase == Phase.WaitingAlice, "swap does not expect Alice's deposit");
  assert(!blockTimeGte(deadline), "swap deadline has passed");
  assert(aliceAuthority == authorityOfAliceWitness(), "Alice authorization failed");
  receiveUnshielded(tokenA, amountA);
  phase = Phase.WaitingBob;
}

export circuit fundBob(): [] {
  assert(phase == Phase.WaitingBob, "swap does not expect Bob's deposit");
  assert(!blockTimeGte(deadline), "swap deadline has passed");
  assert(bobAuthority == authorityOfBobWitness(), "Bob authorization failed");
  receiveUnshielded(tokenB, amountB);
  phase = Phase.WaitingDecision;
}

export circuit decide(decision: Uint<0..2>): [] {
  assert(phase == Phase.WaitingDecision, "swap does not expect a decision");
  assert(!blockTimeGte(deadline), "swap deadline has passed");
  assert(bobAuthority == authorityOfBobWitness(), "Bob authorization failed");
  if (disclose(decision) == 1) {
    sendUnshielded(tokenA, amountA, right<ContractAddress, UserAddress>(bob));
    sendUnshielded(tokenB, amountB, right<ContractAddress, UserAddress>(alice));
    phase = Phase.Settled;
  } else {
__ORDERED_REFUNDS__
    phase = Phase.Refunded;
  }
}

export circuit expire(): [] {
  assert(phase != Phase.Settled, "settled swap cannot expire");
  assert(phase != Phase.Refunded, "refunded swap cannot expire");
  assert(blockTimeGte(deadline), "swap deadline has not passed");
  if (phase == Phase.WaitingBob) {
    sendUnshielded(tokenA, amountA, right<ContractAddress, UserAddress>(alice));
  }
  if (phase == Phase.WaitingDecision) {
__ORDERED_REFUNDS__
  }
  phase = Phase.Refunded;
}
"""
    return source.replace("__ORDERED_REFUNDS__", ordered_refunds)


def lower_swap(contract: Contract, parameters: SwapParameters) -> Lowering:
    if contract != canonical_swap(parameters):
        raise ValueError("lowerer accepts only the canonical atomic-swap shape")

    source = _compact_source(parameters)
    bounds = analyze_bounds(contract)
    core_data = _data(contract)
    parameter_data = {
        "alice": parameters.alice.name,
        "bob": parameters.bob.name,
        "token_a": asdict(parameters.token_a),
        "token_b": asdict(parameters.token_b),
        "amount_a": parameters.amount_a,
        "amount_b": parameters.amount_b,
        "deadline": parameters.deadline,
        "choice_id": parameters.choice_id,
    }
    constructor_schema = [
        {
            "name": "initialAlice",
            "compact_type": "UserAddress",
            "core_parameter": "alice",
            "abstract_value": parameters.alice.name,
            "encoding": "midnight-js encodeUserAddress",
        },
        {
            "name": "initialBob",
            "compact_type": "UserAddress",
            "core_parameter": "bob",
            "abstract_value": parameters.bob.name,
            "encoding": "midnight-js encodeUserAddress",
        },
        {
            "name": "initialAliceAuthority",
            "compact_type": "Bytes<32>",
            "core_parameter": None,
            "abstract_value": None,
            "encoding": "persistentHash<PartySecret> output",
        },
        {
            "name": "initialBobAuthority",
            "compact_type": "Bytes<32>",
            "core_parameter": None,
            "abstract_value": None,
            "encoding": "persistentHash<PartySecret> output",
        },
        {
            "name": "initialTokenA",
            "compact_type": "Bytes<32>",
            "core_parameter": "token_a",
            "abstract_value": asdict(parameters.token_a),
            "encoding": "Midnight ledger token color",
        },
        {
            "name": "initialTokenB",
            "compact_type": "Bytes<32>",
            "core_parameter": "token_b",
            "abstract_value": asdict(parameters.token_b),
            "encoding": "Midnight ledger token color",
        },
        {
            "name": "initialAmountA",
            "compact_type": "Uint<128>",
            "core_parameter": "amount_a",
            "abstract_value": parameters.amount_a,
            "encoding": "unsigned integer",
        },
        {
            "name": "initialAmountB",
            "compact_type": "Uint<128>",
            "core_parameter": "amount_b",
            "abstract_value": parameters.amount_b,
            "encoding": "unsigned integer",
        },
        {
            "name": "initialDeadline",
            "compact_type": "Uint<64>",
            "core_parameter": "deadline",
            "abstract_value": parameters.deadline,
            "encoding": "Midnight block-time seconds",
        },
    ]
    for parameter in constructor_schema:
        parameter["binding"] = "required-in-deployment-manifest"
    refund_effects_by_account = {
        parameters.alice_account: {
            "kind": "pay_all",
            "account": "alice",
            "to": parameters.alice.name,
            "token": asdict(parameters.token_a),
        },
        parameters.bob_account: {
            "kind": "pay_all",
            "account": "bob",
            "to": parameters.bob.name,
            "token": asdict(parameters.token_b),
        },
    }
    refund_effects = [
        refund_effects_by_account[account]
        for account in sorted(refund_effects_by_account)
    ]
    settle_effects = [
        {
            "kind": "pay_all",
            "account": "alice",
            "to": parameters.bob.name,
            "token": asdict(parameters.token_a),
        },
        {
            "kind": "pay_all",
            "account": "bob",
            "to": parameters.alice.name,
            "token": asdict(parameters.token_b),
        },
    ]
    entry_points = [
        {
            "name": "fundAlice",
            "from": "WaitingAlice",
            "to": "WaitingBob",
            "authorization": "aliceSecret",
            "time_guard": "before-deadline",
            "input": {
                "kind": "deposit",
                "party": parameters.alice.name,
                "policy_id": parameters.token_a.policy_id,
                "asset_name": parameters.token_a.asset_name,
                "quantity": parameters.amount_a,
            },
            "effects": [
                {
                    "kind": "credit",
                    "account": "alice",
                    "quantity": parameters.amount_a,
                }
            ],
        },
        {
            "name": "fundBob",
            "from": "WaitingBob",
            "to": "WaitingDecision",
            "authorization": "bobSecret",
            "time_guard": "before-deadline",
            "input": {
                "kind": "deposit",
                "party": parameters.bob.name,
                "policy_id": parameters.token_b.policy_id,
                "asset_name": parameters.token_b.asset_name,
                "quantity": parameters.amount_b,
            },
            "effects": [
                {
                    "kind": "credit",
                    "account": "bob",
                    "quantity": parameters.amount_b,
                }
            ],
        },
        {
            "name": "decide",
            "from": "WaitingDecision",
            "to": {"0": "Refunded", "1": "Settled"},
            "authorization": "bobSecret",
            "time_guard": "before-deadline",
            "input": {
                "kind": "choice",
                "party": parameters.bob.name,
                "choice_id": parameters.choice_id,
                "lower": 0,
                "upper": 1,
            },
            "choice": {"id": parameters.choice_id, "lower": 0, "upper": 1},
            "outcome_effects": {
                "0": refund_effects,
                "1": settle_effects,
            },
        },
        {
            "name": "expire",
            "from": ["WaitingAlice", "WaitingBob", "WaitingDecision"],
            "to": "Refunded",
            "authorization": None,
            "time_guard": "at-or-after-deadline",
            "preempts_other_inputs": True,
            "input": {"kind": "expire"},
            "effects": refund_effects,
        },
    ]
    manifest: dict[str, Any] = {
        "schema_version": 1,
        "status": "S3-experimental",
        "core": core_data,
        "core_sha256": _sha256(_canonical_bytes(core_data)),
        "compact_sha256": _sha256(source.encode("utf-8")),
        "parameters": parameter_data,
        "binding_status": "abstract-template",
        "constructor_schema": constructor_schema,
        "unbound_constructor_parameters": [
            parameter["name"] for parameter in constructor_schema
        ],
        "phases": [
            "WaitingAlice",
            "WaitingBob",
            "WaitingDecision",
            "Settled",
            "Refunded",
        ],
        "initial_phase": "WaitingAlice",
        "terminal_phases": ["Settled", "Refunded"],
        "backend_model_scope": {
            "included": [
                "acceptance",
                "public phase",
                "balances",
                "choices",
                "payments",
                "transaction time",
            ],
            "excluded": [
                "witness execution",
                "privacy semantics",
                "compiler correctness",
                "proof-system soundness",
            ],
        },
        "bounds": asdict(bounds),
        "entry_points": entry_points,
        "witnesses": [
            {
                "name": "aliceSecret",
                "constraint": "persistentHash(aliceSecret) == aliceAuthority",
                "used_by": ["fundAlice"],
            },
            {
                "name": "bobSecret",
                "constraint": "persistentHash(bobSecret) == bobAuthority",
                "used_by": ["fundBob", "decide"],
            },
        ],
        "disclosures": [
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
        ],
        "toolchain": TOOLCHAIN,
        "expected_zkir": ["fundAlice", "fundBob", "decide", "expire"],
    }
    return Lowering(source, manifest)
