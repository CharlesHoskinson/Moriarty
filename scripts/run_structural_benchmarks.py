#!/usr/bin/env python3
"""Reproducible, ledger-independent structural benchmarks for Marlowe V1 JSON."""

from __future__ import annotations

import hashlib
import json
import sys
import time
import zlib
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
REFERENCE = ROOT / "repos/marlowe-cardano/marlowe-test/reference/data"
OUTPUT = ROOT / "experiments/structural-benchmarks-2026-09-02.json"

CASES = {
    "escrow": "escrow.contract",
    "escrow_with_collateral": "escrow-with-collateral.contract",
    "atomic_token_swap": "swap-of-ada-and-dollar-token.contract",
    "zero_coupon_bond": "zero-coupon-bond.contract",
    "recurring_revenue_loan": "revenue-based-loan.contract",
    "covered_call": "covered-call.contract",
    "contract_for_differences": "contract-for-differences.contract",
    "contract_for_differences_oracle": "contract-for-differences-with-oracle.contract",
    "actus_pam": "actus-pam.contract",
    "nft_pawn": "nft-pawned.contract",
    "nft_swap": "nft-swap.contract",
    "nft_oracle": "nft-oracle.contract",
}


def walk(value: Any) -> tuple[int, int, Counter[str]]:
    keys: Counter[str] = Counter()
    nodes = 0
    maximum_depth = 0
    stack = [(value, 0)]
    while stack:
        current, depth = stack.pop()
        nodes += 1
        maximum_depth = max(maximum_depth, depth)
        if isinstance(current, dict):
            keys.update(current.keys())
            stack.extend((child, depth + 1) for child in current.values())
        elif isinstance(current, list):
            stack.extend((child, depth + 1) for child in current)
    return nodes, maximum_depth, keys


def stats(value: Any, raw: bytes) -> dict[str, Any]:
    nodes, depth, keys = walk(value)
    compact = json.dumps(value, separators=(",", ":"), sort_keys=True).encode()
    pretty = json.dumps(value, indent=2, sort_keys=True).encode()
    return {
        "source_bytes": len(raw),
        "canonical_json_bytes": len(compact),
        "pretty_json_bytes": len(pretty),
        "zlib_level_9_bytes_nonledger_experiment": len(zlib.compress(compact, 9)),
        "recursive_json_nodes": nodes,
        "maximum_json_depth": depth,
        "when_constructs": keys["when"],
        "cases": keys["case"],
        "payments": keys["pay"],
        "deposits": keys["deposits"],
        "choices": keys["choose_between"],
        "timeouts": keys["timeout"],
        "sha256": hashlib.sha256(raw).hexdigest(),
    }


def role(name: str) -> dict[str, str]:
    return {"role_token": name}


ADA = {"currency_symbol": "", "token_name": ""}


def recurring_contract(periods: int) -> Any:
    contract: Any = "close"
    for period in reversed(range(periods)):
        due = 1_800_000_000_000 + period * 2_592_000_000
        amount = 1_000_000
        contract = {
            "when": [
                {
                    "case": {
                        "party": role("Payer"),
                        "of_token": ADA,
                        "into_account": role("Payer"),
                        "deposits": amount,
                    },
                    "then": {
                        "from_account": role("Payer"),
                        "to": {"party": role("Payee")},
                        "token": ADA,
                        "pay": amount,
                        "then": contract,
                    },
                }
            ],
            "timeout": due,
            "timeout_continuation": "close",
        }
    return contract


def high_fanout_state(participants: int) -> dict[str, Any]:
    return {
        "accounts": [
            [[{"role_token": f"Participant-{index:04d}"}, ADA], 2_000_000]
            for index in range(participants)
        ],
        "choices": [],
        "boundValues": [],
        "minTime": 1_800_000_000_000,
    }


def main() -> None:
    # The 1,000-period V1 contract is intentionally thousands of levels deep.
    # CPython's JSON encoder is recursive even though the structural walk is not.
    sys.setrecursionlimit(50_000)
    result: dict[str, Any] = {
        "scope": (
            "Structural JSON and generation-time measures only. zlib is a relocation probe, "
            "not a valid Cardano datum encoding. No execution units or fees are claimed."
        ),
        "reference_commit": "99f432d8609fcfd3bc66660c24df632f11707660",
        "reference_contracts": {},
        "recurring_unrolling": {},
        "high_fanout_close_input_state": {},
    }
    for name, filename in CASES.items():
        path = REFERENCE / filename
        raw = path.read_bytes()
        result["reference_contracts"][name] = {"path": str(path.relative_to(ROOT)), **stats(json.loads(raw), raw)}
    for periods in (10, 100, 1_000):
        started = time.perf_counter_ns()
        contract = recurring_contract(periods)
        elapsed = time.perf_counter_ns() - started
        raw = json.dumps(contract, separators=(",", ":")).encode()
        result["recurring_unrolling"][str(periods)] = {
            "generation_time_ns": elapsed,
            "semantic_transaction_upper_bound": periods,
            **stats(contract, raw),
        }
    for participants in (10, 100, 1_000):
        state = high_fanout_state(participants)
        raw = json.dumps(state, separators=(",", ":")).encode()
        result["high_fanout_close_input_state"][str(participants)] = {
            "abstract_refund_payments": participants,
            **stats(state, raw),
        }
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(OUTPUT)


if __name__ == "__main__":
    main()
