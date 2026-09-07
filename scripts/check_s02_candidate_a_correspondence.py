#!/usr/bin/env python3
"""Independent finite-record checker for Candidate A correspondence exports.

This module deliberately does not import the Candidate A exporter, Quint
projection code, or any effect-extraction helper. Reference results come only
from pinned Python Core and independent literal fixture constructors. Successful
checks require exact raw-ITF provenance linkage; they are finite-record evidence,
not exhaustive correspondence or authenticated generator execution.
"""

from __future__ import annotations

import argparse
import hashlib
import inspect
import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from moriarty.core import (Account, Case, Choice, ChoiceEquals, ChoiceInput, Close,
                           Constant, Deposit, DepositInput, If, Party, Pay,
                           Payment, State, Token, TransactionResult, Warning, When,
                           compute_transaction, reduce_to_quiescence)
from moriarty.swap import SwapParameters, canonical_swap


@dataclass(frozen=True)
class CheckReport:
    ok: bool
    differences: tuple[str, ...]


class DecodeError(ValueError):
    pass


ROOT = Path(__file__).resolve().parents[1]
PINNED = {
    "moriarty/core.py": "564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b",
    "moriarty/swap.py": "82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797",
    "specs/quint/s02/candidate_a_core.qnt": "e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec",
    "specs/quint/s02/candidate_a_types.qnt": "40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b",
    "specs/quint/s02/candidate_a_programs.qnt": "bf814bce2924cafac5836a171b52a4c9bdba43e083fe7129bcdeadd810c42d5e",
    "specs/quint/s02/candidate_a_projection.qnt": "2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c",
    "specs/quint/s02/effects.qnt": "dccc9208d9a42f01685000b1358f1d2f9684bf5e8ebe1d5deadd0c992557504c",
    "specs/quint/s02/observations.qnt": "e44484ed9035e560ef9f4d012198671917750da42ab404022592db16cdd59bc3",
}
OPTIONAL_GENERATORS = {
    "swapTrace": "specs/quint/s02/candidate_a_harness.qnt",
    "installmentTrace": "specs/quint/s02/candidate_a_installment_harness.qnt",
    "diagnosticCases": "specs/quint/s02/candidate_a_cases.qnt",
}
PARTIES = {"Alice": Party("alice"), "Bob": Party("bob"), "Mallory": Party("mallory")}
TOKENS = {"TokenA": Token("aa", "A"), "TokenB": Token("bb", "B")}
CHOICES = {"SettleId": "settle", "FirstFillId": "fill1", "SecondFillId": "fill2",
           "RecoveryId": "recover", "OtherId": "other"}
TIMES = {"Time0": 0, "Time1": 1, "Time2": 2, "Time100": 100, "Time101": 101}
NODES = {f"N{number}" for number in range(16)}
CONSTANTS = {-1, 0, 1, 5, 10, 20}
CHOICE_VALUES = {-1, 0, 1, 2}
DEPOSIT_QUANTITIES = {-1, 0, 1, 5, 10, 20, 21}
CORE_ERRORS = {"time_before_state", "contract_closed", "input_required",
               "no_matching_input", "choice_out_of_bounds", "non_positive_deposit"}
FIXTURES = {"canonical-swap-v1", "installment-two-when-v1", "close-v1", "pay-zero-v1",
            "pay-negative-v1", "pay-ten-v1", "if-settle-zero-v1", "deposit-five-close-v1",
            "deposit-zero-close-v1", "pre-deposit-post-v1", "pre-warning-deposit-v1",
            "ordered-choice-v1", "ordered-choice-overlap-v1"}
CASE_FIXTURES = {
    "error-time-before-state": "canonical-swap-v1", "error-contract-closed": "close-v1",
    "error-input-required": "canonical-swap-v1", "error-no-matching-input": "canonical-swap-v1",
    "error-choice-out-of-bounds": "canonical-swap-v1", "error-non-positive-deposit": "deposit-zero-close-v1",
    "warning-zero": "pay-zero-v1", "warning-negative": "pay-negative-v1", "warning-partial": "pay-ten-v1", "warning-empty-account": "pay-ten-v1",
    "pre-deposit-post": "pre-deposit-post-v1", "speculative-payment-rejected": "pre-deposit-post-v1", "speculative-warning-rejected": "pre-warning-deposit-v1",
    "deposit-immediate-refund": "deposit-five-close-v1", "deposit-zero-mismatch": "deposit-five-close-v1", "deposit-without-payment": "canonical-swap-v1",
    "absent-choice": "if-settle-zero-v1", "stored-zero-choice": "if-settle-zero-v1", "ordered-first": "ordered-choice-v1", "ordered-fallthrough": "ordered-choice-v1",
    "ordered-overlap": "ordered-choice-overlap-v1", "ordered-no-acceptance": "ordered-choice-v1", "ordered-wrong-chooser": "ordered-choice-v1", "close-full-state": "close-v1",
}


def _record(value: object, path: str, required: set[str]) -> dict[str, Any]:
    if not isinstance(value, dict) or set(value) != required:
        raise DecodeError(f"{path}: expected exactly {sorted(required)}")
    return value


def _enum(value: object, path: str, allowed: set[str] | None = None) -> tuple[str, object]:
    item = _record(value, path, {"tag", "value"})
    tag = item["tag"]
    if not isinstance(tag, str) or (allowed is not None and tag not in allowed):
        raise DecodeError(f"{path}.tag: unsupported enum")
    return tag, item["value"]


def _nullary(value: object, path: str, allowed: set[str]) -> str:
    tag, payload = _enum(value, path, allowed)
    if payload != {"#tup": []}:
        raise DecodeError(f"{path}.{tag}: expected nullary tuple")
    return tag


def _integer(value: object, path: str) -> int:
    if type(value) is int:
        return value
    if not isinstance(value, dict) or set(value) != {"#bigint"} or not isinstance(value["#bigint"], str):
        raise DecodeError(f"{path}: expected integer or #bigint")
    text = value["#bigint"]
    if re.fullmatch(r"-?(0|[1-9][0-9]*)", text) is None or text == "-0":
        raise DecodeError(f"{path}: malformed #bigint")
    return int(text)


def _map(value: object, path: str) -> list[tuple[object, object]]:
    item = _record(value, path, {"#map"})
    if not isinstance(item["#map"], list):
        raise DecodeError(f"{path}.#map: expected list")
    pairs: list[tuple[object, object]] = []
    for index, pair in enumerate(item["#map"]):
        if not isinstance(pair, list) or len(pair) != 2:
            raise DecodeError(f"{path}.#map[{index}]: expected pair")
        pairs.append((pair[0], pair[1]))
    return pairs


def _party(value: object, path: str) -> Party:
    return PARTIES[_nullary(value, path, set(PARTIES))]


def _token(value: object, path: str) -> Token:
    return TOKENS[_nullary(value, path, set(TOKENS))]


def _account(value: object, path: str) -> Account:
    record = _record(value, path, {"owner", "asset"})
    return Account(_party(record["owner"], path + ".owner"), _token(record["asset"], path + ".asset"))


def _node(value: object, path: str) -> str:
    return _nullary(value, path, NODES)


def _time(value: object, path: str) -> int:
    return TIMES[_nullary(value, path, set(TIMES))]


def _finite_integer(value: object, path: str, domain: set[int]) -> int:
    number = _integer(value, path)
    if number not in domain:
        raise DecodeError(f"{path}: outside finite domain")
    return number


def _constant(value: object, path: str) -> Constant:
    tag, payload = _enum(value, path, {"ConstantA"})
    del tag
    return Constant(_finite_integer(payload, path + ".value", CONSTANTS))


def _action(value: object, path: str) -> Deposit | Choice:
    tag, payload = _enum(value, path, {"DepositA", "ChoiceA"})
    if tag == "DepositA":
        item = _record(payload, path + ".DepositA", {"account", "depositor", "amount"})
        return Deposit(_account(item["account"], path + ".account"), _party(item["depositor"], path + ".depositor"), _constant(item["amount"], path + ".amount"))
    item = _record(payload, path + ".ChoiceA", {"id", "chooser", "lower", "upper"})
    choice = _nullary(item["id"], path + ".id", set(CHOICES))
    lower = _finite_integer(item["lower"], path + ".lower", CHOICE_VALUES)
    upper = _finite_integer(item["upper"], path + ".upper", CHOICE_VALUES)
    if lower > upper:
        raise DecodeError(f"{path}: reversed choice bounds")
    return Choice(CHOICES[choice], _party(item["chooser"], path + ".chooser"), lower, upper)


def _decode_program(value: object, path: str) -> tuple[object, dict[str, object], str]:
    item = _record(value, path, {"root", "nodes"})
    root = _node(item["root"], path + ".root")
    entries = _map(item["nodes"], path + ".nodes")
    raw_nodes: dict[str, object] = {}
    for index, (raw_id, raw_contract) in enumerate(entries):
        node = _node(raw_id, f"{path}.nodes[{index}].key")
        if node in raw_nodes:
            raise DecodeError(f"{path}.nodes: duplicate node {node}")
        raw_nodes[node] = raw_contract
    if set(raw_nodes) != NODES:
        raise DecodeError(f"{path}.nodes: node domain differs from N0..N15")
    built: dict[str, object] = {}
    def build(node: str) -> object:
        if node in built:
            return built[node]
        def child(raw: object, subpath: str) -> object:
            target = _node(raw, subpath)
            if int(target[1:]) >= int(node[1:]):
                raise DecodeError(f"{subpath}: edge violates decreasing rank (cycle or forward edge)")
            return build(target)
        tag, payload = _enum(raw_nodes[node], f"{path}.nodes[{node}]", {"CloseA", "PayA", "IfA", "WhenA"})
        if tag == "CloseA":
            if payload != {"#tup": []}:
                raise DecodeError(f"{path}.nodes[{node}]: malformed CloseA")
            result: object = Close()
        elif tag == "PayA":
            data = _record(payload, f"{path}.nodes[{node}].PayA", {"account", "payee", "amount", "continuation"})
            result = Pay(_account(data["account"], path + ".account"), _party(data["payee"], path + ".payee"), _constant(data["amount"], path + ".amount"), child(data["continuation"], path + ".continuation"))
        elif tag == "IfA":
            data = _record(payload, f"{path}.nodes[{node}].IfA", {"observation", "thenNode", "elseNode"})
            observation_tag, observation = _enum(data["observation"], path + ".observation", {"ChoiceEqualsA"})
            del observation_tag
            obs = _record(observation, path + ".observation.value", {"id", "expected"})
            choice = _nullary(obs["id"], path + ".observation.id", set(CHOICES))
            result = If(ChoiceEquals(CHOICES[choice], _finite_integer(obs["expected"], path + ".observation.expected", CHOICE_VALUES)), child(data["thenNode"], path + ".thenNode"), child(data["elseNode"], path + ".elseNode"))
        else:
            data = _record(payload, f"{path}.nodes[{node}].WhenA", {"cases", "timeout", "timeoutNode"})
            if not isinstance(data["cases"], list) or len(data["cases"]) > 2:
                raise DecodeError(f"{path}.nodes[{node}].cases: expected list of at most two cases")
            cases: list[Case] = []
            for index, raw_case in enumerate(data["cases"]):
                case_data = _record(raw_case, f"{path}.nodes[{node}].cases[{index}]", {"caseAction", "continuation"})
                cases.append(Case(_action(case_data["caseAction"], path + ".caseAction"), child(case_data["continuation"], path + ".continuation")))
            result = When(tuple(cases), _time(data["timeout"], path + ".timeout"), child(data["timeoutNode"], path + ".timeoutNode"))
        built[node] = result
        return result
    return build(root), {node: build(node) for node in NODES}, root


def _state(value: object, path: str) -> State:
    item = _record(value, path, {"accounts", "choices", "continuation", "minimumTime"})
    continuation = _node(item["continuation"], path + ".continuation")
    accounts: dict[Account, int] = {}
    for index, (key, amount) in enumerate(_map(item["accounts"], path + ".accounts")):
        account = _account(key, f"{path}.accounts[{index}].key")
        if account in accounts:
            raise DecodeError(f"{path}.accounts: duplicate account")
        quantity = _integer(amount, f"{path}.accounts[{index}].value")
        if quantity < 0 or quantity + 20 * int(continuation[1:]) > 340:
            raise DecodeError(f"{path}.accounts: outside finite balance/potential domain")
        if quantity:
            accounts[account] = quantity
    expected_accounts = {Account(party, token) for party in PARTIES.values() for token in TOKENS.values()}
    raw_accounts = _map(item["accounts"], path + ".accounts")
    seen_accounts = {Account(_party(key["owner"], "account.owner"), _token(key["asset"], "account.asset"))
                     for key, _ in raw_accounts
                     if isinstance(key, dict) and set(key) == {"owner", "asset"}}
    if len(raw_accounts) != len(expected_accounts) or seen_accounts != expected_accounts:
        raise DecodeError(f"{path}.accounts: account domain differs from fixed six accounts")
    choices: dict[str, int] = {}
    seen_choice_ids: set[str] = set()
    for index, (key, stored) in enumerate(_map(item["choices"], path + ".choices")):
        choice = CHOICES[_nullary(key, f"{path}.choices[{index}].key", set(CHOICES))]
        if choice in seen_choice_ids:
            raise DecodeError(f"{path}.choices: duplicate choice")
        seen_choice_ids.add(choice)
        tag, payload = _enum(stored, f"{path}.choices[{index}].value", {"NoInt", "IntValue"})
        if tag == "NoInt":
            if payload != {"#tup": []}:
                raise DecodeError(f"{path}.choices[{index}]: malformed NoInt")
        else:
            choices[choice] = _finite_integer(payload, f"{path}.choices[{index}].value", CHOICE_VALUES)
    raw_choices = _map(item["choices"], path + ".choices")
    if len(raw_choices) != len(CHOICES) or set(seen_choice_ids) != set(CHOICES.values()):
        raise DecodeError(f"{path}.choices: choice domain differs from fixed five choices")
    return State(accounts=State.from_accounts(accounts).accounts, choices=tuple(sorted(choices.items())), min_time=_time(item["minimumTime"], path + ".minimumTime"))


def _state_node(value: object, path: str) -> str:
    data = _record(value, path, {"accounts", "choices", "continuation", "minimumTime"})
    return _node(data["continuation"], path + ".continuation")


def _input(value: object, path: str) -> DepositInput | ChoiceInput | None:
    tag, payload = _enum(value, path, {"NoAInput", "PresentAInput"})
    if tag == "NoAInput":
        if payload != {"#tup": []}:
            raise DecodeError(f"{path}: malformed NoAInput")
        return None
    inner_tag, inner = _enum(payload, path + ".PresentAInput", {"DepositInputA", "ChoiceInputA"})
    if inner_tag == "DepositInputA":
        data = _record(inner, path + ".DepositInputA", {"account", "depositor", "quantity"})
        return DepositInput(_account(data["account"], path + ".account"), _party(data["depositor"], path + ".depositor"), _finite_integer(data["quantity"], path + ".quantity", DEPOSIT_QUANTITIES))
    data = _record(inner, path + ".ChoiceInputA", {"id", "chooser", "chosen"})
    choice = _nullary(data["id"], path + ".id", set(CHOICES))
    return ChoiceInput(CHOICES[choice], _party(data["chooser"], path + ".chooser"), _finite_integer(data["chosen"], path + ".chosen", CHOICE_VALUES))


def _payment(value: object, path: str) -> Payment:
    data = _record(value, path, {"source", "recipient", "asset", "quantity"})
    source = _account(data["source"], path + ".source")
    token = _token(data["asset"], path + ".asset")
    if token != source.token:
        raise DecodeError(f"{path}: payment asset differs from source account")
    return Payment(source, _party(data["recipient"], path + ".recipient"), token, _integer(data["quantity"], path + ".quantity"))


def _warning(value: object, path: str) -> Warning:
    data = _record(value, path, {"code", "requested", "paid"})
    if data["code"] not in {"non_positive_payment", "partial_payment"}:
        raise DecodeError(f"{path}.code: unsupported warning")
    def optional_int(raw: object, subpath: str) -> int | None:
        tag, payload = _enum(raw, subpath, {"NoInt", "IntValue"})
        if tag == "NoInt":
            if payload != {"#tup": []}:
                raise DecodeError(f"{subpath}: malformed NoInt")
            return None
        return _integer(payload, subpath + ".IntValue")
    return Warning(data["code"], optional_int(data["requested"], path + ".requested"), optional_int(data["paid"], path + ".paid"))


def _error(value: object, path: str) -> str | None:
    tag, payload = _enum(value, path, {"NoCoreError", "CoreErrorCode"})
    if tag == "NoCoreError":
        if payload != {"#tup": []}:
            raise DecodeError(f"{path}: malformed NoCoreError")
        return None
    if not isinstance(payload, str) or payload not in CORE_ERRORS:
        raise DecodeError(f"{path}: unsupported Core error")
    return payload


def _result(value: object, path: str) -> TransactionResult:
    data = _record(value, path, {"accepted", "state", "error", "payments", "warnings", "reductions"})
    if type(data["accepted"]) is not bool:
        raise DecodeError(f"{path}.accepted: expected bool")
    if not isinstance(data["payments"], list) or not isinstance(data["warnings"], list):
        raise DecodeError(f"{path}: payments and warnings must be lists")
    return TransactionResult(data["accepted"], _state(data["state"], path + ".state"), Close(),
                             tuple(_payment(item, f"{path}.payments[{index}]") for index, item in enumerate(data["payments"])),
                             tuple(_warning(item, f"{path}.warnings[{index}]") for index, item in enumerate(data["warnings"])),
                             _integer(data["reductions"], path + ".reductions"), _error(data["error"], path + ".error"))


def _fixture(name: str) -> object:
    alice = PARTIES["Alice"]
    bob = PARTIES["Bob"]
    mallory = PARTIES["Mallory"]
    alice_a = Account(alice, TOKENS["TokenA"])
    if name == "canonical-swap-v1":
        return canonical_swap(SwapParameters.example())
    if name == "installment-two-when-v1":
        close = Close()
        second_pay = Pay(alice_a, bob, Constant(5), close)
        second = When((Case(Choice("fill2", bob, 1, 1), second_pay), Case(Choice("recover", alice, 1, 1), close)), 100, close)
        first_pay = Pay(alice_a, bob, Constant(5), second)
        return When((Case(Choice("fill1", bob, 1, 1), first_pay), Case(Choice("recover", alice, 1, 1), close)), 100, close)
    close = Close()
    if name.startswith("pay-"):
        amount = {"pay-zero-v1": 0, "pay-negative-v1": -1, "pay-ten-v1": 10}[name]
        return Pay(alice_a, bob, Constant(amount), close)
    if name == "close-v1":
        return close
    if name == "if-settle-zero-v1":
        return If(ChoiceEquals("settle", 0), Pay(alice_a, bob, Constant(5), close), close)
    if name in {"deposit-five-close-v1", "deposit-zero-close-v1"}:
        amount = 5 if name == "deposit-five-close-v1" else 0
        return When((Case(Deposit(alice_a, alice, Constant(amount)), close),), 100, close)
    if name in {"pre-deposit-post-v1", "pre-warning-deposit-v1"}:
        pre_amount = 1 if name == "pre-deposit-post-v1" else 0
        post = Pay(alice_a, bob, Constant(5), close)
        waiting = When((Case(Deposit(alice_a, alice, Constant(5)), post),), 100, close)
        return Pay(alice_a, bob, Constant(pre_amount), waiting)
    if name in {"ordered-choice-v1", "ordered-choice-overlap-v1"}:
        first = Case(Choice("settle", bob, 0, 0 if name == "ordered-choice-v1" else 1), Pay(alice_a, bob, Constant(5), close))
        second = Case(Choice("settle", bob, 1 if name == "ordered-choice-v1" else 0, 1), Pay(alice_a, mallory, Constant(5), close))
        return When((first, second), 100, close)
    raise DecodeError(f"fixture_id: unknown fixture {name}")


def _fixture_nodes(name: str) -> tuple[dict[str, object], str]:
    """Independent complete NodeId table; Close labels remain binding data."""
    table: dict[str, object] = {node: Close() for node in NODES}
    alice, bob, mallory = PARTIES["Alice"], PARTIES["Bob"], PARTIES["Mallory"]
    alice_a, bob_b = Account(alice, TOKENS["TokenA"]), Account(bob, TOKENS["TokenB"])
    if name == "canonical-swap-v1":
        table["N1"] = Pay(bob_b, alice, Constant(20), table["N0"])
        table["N2"] = Pay(alice_a, bob, Constant(10), table["N1"])
        table["N3"] = If(ChoiceEquals("settle", 1), table["N2"], table["N0"])
        table["N4"] = When((Case(Choice("settle", bob, 0, 1), table["N3"]),), 100, table["N0"])
        table["N5"] = When((Case(Deposit(bob_b, bob, Constant(20)), table["N4"]),), 100, table["N0"])
        table["N6"] = When((Case(Deposit(alice_a, alice, Constant(10)), table["N5"]),), 100, table["N0"])
        return table, "N6"
    if name == "installment-two-when-v1":
        table["N1"] = Pay(alice_a, bob, Constant(5), table["N0"])
        table["N2"] = When((Case(Choice("fill2", bob, 1, 1), table["N1"]), Case(Choice("recover", alice, 1, 1), table["N0"])), 100, table["N0"])
        table["N3"] = Pay(alice_a, bob, Constant(5), table["N2"])
        table["N4"] = When((Case(Choice("fill1", bob, 1, 1), table["N3"]), Case(Choice("recover", alice, 1, 1), table["N0"])), 100, table["N0"])
        return table, "N4"
    if name == "close-v1": return table, "N0"
    if name in {"pay-zero-v1", "pay-negative-v1", "pay-ten-v1"}:
        amount = {"pay-zero-v1": 0, "pay-negative-v1": -1, "pay-ten-v1": 10}[name]
        table["N1"] = Pay(alice_a, bob, Constant(amount), table["N0"]); return table, "N1"
    if name == "if-settle-zero-v1":
        table["N1"] = Pay(alice_a, bob, Constant(5), table["N0"])
        table["N2"] = If(ChoiceEquals("settle", 0), table["N1"], table["N0"]); return table, "N2"
    if name in {"deposit-five-close-v1", "deposit-zero-close-v1"}:
        amount = 5 if name == "deposit-five-close-v1" else 0
        table["N1"] = When((Case(Deposit(alice_a, alice, Constant(amount)), table["N0"]),), 100, table["N0"]); return table, "N1"
    if name in {"pre-deposit-post-v1", "pre-warning-deposit-v1"}:
        table["N1"] = Pay(alice_a, bob, Constant(5), table["N0"])
        table["N2"] = When((Case(Deposit(alice_a, alice, Constant(5)), table["N1"]),), 100, table["N0"])
        table["N3"] = Pay(alice_a, bob, Constant(1 if name == "pre-deposit-post-v1" else 0), table["N2"]); return table, "N3"
    if name in {"ordered-choice-v1", "ordered-choice-overlap-v1"}:
        table["N1"] = Pay(alice_a, bob, Constant(5), table["N0"])
        table["N2"] = Pay(alice_a, mallory, Constant(5), table["N0"])
        lower2 = 1 if name == "ordered-choice-v1" else 0
        upper1 = 0 if name == "ordered-choice-v1" else 1
        table["N3"] = When((Case(Choice("settle", bob, 0, upper1), table["N1"]), Case(Choice("settle", bob, lower2, 1), table["N2"])), 100, table["N0"]); return table, "N3"
    raise DecodeError(f"fixture table unavailable: {name}")


def _transfer(value: object, path: str) -> tuple[str, Party | Account, str, Party | Account, Token, int]:
    data = _record(value, path, {"source", "destination", "asset", "quantity"})
    def location(raw: object, subpath: str) -> tuple[str, Party | Account]:
        tag, payload = _enum(raw, subpath, {"Wallet", "Escrow"})
        return ("wallet", _party(payload, subpath + ".Wallet")) if tag == "Wallet" else ("escrow", _account(payload, subpath + ".Escrow"))
    source_kind, source = location(data["source"], path + ".source")
    destination_kind, destination = location(data["destination"], path + ".destination")
    return source_kind, source, destination_kind, destination, _token(data["asset"], path + ".asset"), _integer(data["quantity"], path + ".quantity")


def _expected_effects(program: object, before: State, supplied: DepositInput | ChoiceInput | None, now: int, result: TransactionResult) -> tuple[tuple[str, Party | Account, str, Party | Account, Token, int], ...]:
    if not result.accepted:
        return ()
    prefix = reduce_to_quiescence(program, State(before.accounts, before.choices, now))
    if result.payments[:len(prefix.payments)] != prefix.payments:
        raise DecodeError("reference pre-input payments are not a prefix of actual payments")
    effects: list[tuple[str, Party | Account, str, Party | Account, Token, int]] = []
    for payment in prefix.payments:
        effects.append(("escrow", payment.source, "wallet", payment.to, payment.token, payment.quantity))
    if isinstance(supplied, DepositInput):
        effects.append(("wallet", supplied.depositor, "escrow", supplied.account, supplied.account.token, supplied.quantity))
    for payment in result.payments[len(prefix.payments):]:
        effects.append(("escrow", payment.source, "wallet", payment.to, payment.token, payment.quantity))
    return tuple(effects)


def _result_node(raw_result: object, path: str) -> str:
    data = _record(raw_result, path, {"accepted", "state", "error", "payments", "warnings", "reductions"})
    state = _record(data["state"], path + ".state", {"accounts", "choices", "continuation", "minimumTime"})
    return _node(state["continuation"], path + ".state.continuation")


def _projection(value: object, path: str) -> tuple[TransactionResult, object, str]:
    data = _record(value, path, {"accepted", "error", "payments", "warnings", "state", "reductions"})
    state = _record(data["state"], path + ".state", {"accounts", "choices", "continuation", "minimumTime"})
    continuation = _record(state["continuation"], path + ".state.continuation", {"program", "node"})
    _decode_program(continuation["program"], path + ".state.continuation.program")
    node = _node(continuation["node"], path + ".state.continuation.node")
    # Projection choice keys are Core strings, whereas raw AState keys are AChoiceId enums.
    inverse_choices = {name: tag for tag, name in CHOICES.items()}
    projected_choice_pairs = _map(state["choices"], path + ".state.choices")
    native_choices: list[list[object]] = []
    for index, (key, chosen) in enumerate(projected_choice_pairs):
        if not isinstance(key, str) or key not in inverse_choices:
            raise DecodeError(f"{path}.state.choices[{index}]: unsupported projected choice key")
        native_choices.append([{"tag": inverse_choices[key], "value": {"#tup": []}}, chosen])
    neutral_time = _finite_integer(state["minimumTime"], path + ".state.minimumTime", set(TIMES.values()))
    time_tag = next(tag for tag, value in TIMES.items() if value == neutral_time)
    native_state = {"accounts": state["accounts"], "choices": {"#map": native_choices}, "continuation": continuation["node"], "minimumTime": {"tag": time_tag, "value": {"#tup": []}}}
    return _result({**data, "state": native_state}, path), continuation["program"], node


def _same_result(actual: TransactionResult, observed: TransactionResult, expected_contract: object, observed_node: str, nodes: dict[str, object]) -> str | None:
    for field in ("accepted", "state", "payments", "warnings", "reductions", "error"):
        if getattr(actual, field) != getattr(observed, field):
            return f"result.{field}"
    if nodes[observed_node] != expected_contract:
        return "result.state.continuation"
    return None


def _exact_node(contract: object, nodes: dict[str, object], path: str) -> str:
    """Recover the node identity retained by the reference evaluator, not equality."""
    matches = [node for node, candidate in nodes.items() if candidate is contract]
    if len(matches) != 1:
        raise DecodeError(f"{path}: evaluator continuation is not a unique decoded node")
    return matches[0]


def _symbolic_table(nodes: dict[str, object]) -> dict[str, tuple[object, ...]]:
    """Keep every edge label: equal Close subtrees are not interchangeable IDs."""
    def edge(contract: object) -> str:
        return _exact_node(contract, nodes, "symbolic edge")
    result = {}
    for node, contract in nodes.items():
        if isinstance(contract, Close):
            result[node] = ("Close",)
        elif isinstance(contract, Pay):
            result[node] = ("Pay", contract.account, contract.payee, contract.amount, edge(contract.continuation))
        elif isinstance(contract, If):
            result[node] = ("If", contract.observation, edge(contract.then_contract), edge(contract.else_contract))
        elif isinstance(contract, When):
            result[node] = ("When", tuple((case.action, edge(case.continuation)) for case in contract.cases),
                            contract.timeout, edge(contract.timeout_continuation))
        else:
            raise DecodeError(f"symbolic node {node}: unsupported constructor")
    return result


def _check_case(case: object, index: int, declared_pins: dict[str, str]) -> list[str]:
    path = f"cases[{index}]"
    data = _record(case, path, {"case_id", "fixture_id", "trace_id", "step_index", "request", "result", "projection", "effects", "provenance"})
    if not isinstance(data["case_id"], str) or not data["case_id"]:
        raise DecodeError(f"{path}.case_id: expected nonempty string")
    if not isinstance(data["fixture_id"], str) or data["fixture_id"] not in FIXTURES:
        raise DecodeError(f"{path}.fixture_id: unknown fixture")
    short_case_id = data["case_id"].rsplit("::", 1)[-1]
    if short_case_id in CASE_FIXTURES and data["fixture_id"] != CASE_FIXTURES[short_case_id]:
        raise DecodeError(f"{path}.fixture_id: differs from fixed diagnostic fixture mapping")
    if not isinstance(data["trace_id"], str) or not data["trace_id"] or type(data["step_index"]) is not int or data["step_index"] < 0:
        raise DecodeError(f"{path}: invalid trace identity")
    request = _record(data["request"], path + ".request", {"program", "before", "input", "now"})
    program, nodes, root = _decode_program(request["program"], path + ".request.program")
    fixture_nodes, fixture_root = _fixture_nodes(data["fixture_id"])
    if data["fixture_id"] == "canonical-swap-v1" and fixture_nodes[fixture_root] != canonical_swap(SwapParameters.example()):
        return [f"{path}.request.program: literal fixture differs from frozen canonical swap"]
    if root != fixture_root or _symbolic_table(nodes) != _symbolic_table(fixture_nodes):
        return [f"{path}.request.program: differs from fixture {data['fixture_id']}"]
    before = _state(request["before"], path + ".request.before")
    supplied = _input(request["input"], path + ".request.input")
    now = _time(request["now"], path + ".request.now")
    current_node = _state_node(request["before"], path + ".request.before")
    # The semantic oracle uses the independently constructed fixture, never an
    # observed result or exporter-built continuation. Object identity retains IDs.
    nodes = fixture_nodes
    current_contract = nodes[current_node]
    actual = compute_transaction(current_contract, before, supplied, now=now)
    observed = _result(data["result"], path + ".result")
    differences: list[str] = []
    observed_node = _result_node(data["result"], path + ".result")
    mismatch = _same_result(actual, observed, actual.contract, observed_node, nodes)
    if mismatch is not None:
        differences.append(f"{path}.{mismatch}: differs from frozen core")
    if not actual.accepted and observed_node != current_node:
        differences.append(f"{path}.result.state.continuation: rejected result must retain original node")
    if actual.accepted and observed_node != _exact_node(actual.contract, nodes, path + ".result"):
        differences.append(f"{path}.result.state.continuation: differs from exact evaluator node")
    projected, projection_program, projection_node = _projection(data["projection"], path + ".projection")
    _, projected_nodes, projected_root = _decode_program(projection_program, path + ".projection.program")
    if projected_root != fixture_root or _symbolic_table(projected_nodes) != _symbolic_table(fixture_nodes):
        differences.append(f"{path}.projection.state.continuation.program: differs from request.program")
    mismatch = _same_result(actual, projected, actual.contract, projection_node, nodes)
    if mismatch is not None:
        differences.append(f"{path}.projection.{mismatch}: differs from frozen core")
    if not actual.accepted and projection_node != current_node:
        differences.append(f"{path}.projection.state.continuation.node: rejected result must retain original node")
    if actual.accepted and projection_node != _exact_node(actual.contract, nodes, path + ".projection"):
        differences.append(f"{path}.projection.state.continuation.node: differs from exact evaluator node")
    if not isinstance(data["effects"], list):
        raise DecodeError(f"{path}.effects: expected list")
    effects = tuple(_transfer(item, f"{path}.effects[{effect_index}]") for effect_index, item in enumerate(data["effects"]))
    expected_effects = _expected_effects(current_contract, before, supplied, now, actual)
    if effects != expected_effects:
        differences.append(f"{path}.effects: differs from independently derived ordered effects")
    return differences


def _load_json(data: bytes | str, path: str) -> object:
    def unique_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
        result: dict[str, object] = {}
        for key, value in pairs:
            if key in result:
                raise DecodeError(f"{path}: duplicate JSON object key {key}")
            result[key] = value
        return result
    def nonfinite(value: str) -> object:
        raise DecodeError(f"{path}: nonfinite JSON number {value}")
    return json.loads(data, object_pairs_hook=unique_object, parse_constant=nonfinite)


def _same_json(left: object, right: object) -> bool:
    # Python equality alone conflates bool with int; raw payload identity must not.
    return json.dumps(left, sort_keys=True, allow_nan=False) == json.dumps(right, sort_keys=True, allow_nan=False)


def _relative_input(name: object, input_root: Path, path: str) -> Path:
    if not isinstance(name, str) or not name or "\\" in name:
        raise DecodeError(f"{path}: malformed relative input path")
    relative = Path(name)
    if relative.is_absolute() or any(part in {"", ".", ".."} for part in name.split("/")):
        raise DecodeError(f"{path}: input path must be normalized and relative")
    candidate = (input_root / relative).resolve()
    if input_root.resolve() not in candidate.parents:
        raise DecodeError(f"{path}: input path escapes input root")
    return candidate


def _raw_occurrences(raw: object, step: int, path: str) -> tuple[str, list[tuple[int, dict[str, Any]]]]:
    if not isinstance(raw, dict) or set(raw) - {"#meta", "vars", "states"}:
        raise DecodeError(f"{path}: malformed ITF document")
    variables = raw.get("vars")
    if not isinstance(variables, list) or len(variables) != 1 or variables[0] not in OPTIONAL_GENERATORS:
        raise DecodeError(f"{path}.vars: expected one known trace variable")
    variable = variables[0]
    metadata = raw.get("#meta")
    if not isinstance(metadata, dict) or metadata.get("format") != "ITF" or metadata.get("source") != OPTIONAL_GENERATORS[variable]:
        raise DecodeError(f"{path}.#meta: expected ITF format and exact declared generator source")
    states = raw.get("states")
    if not isinstance(states, list) or not states:
        raise DecodeError(f"{path}.states: expected nonempty list")
    occurrences = []
    previous: list[object] = []
    for index, state in enumerate(states):
        if not isinstance(state, dict) or set(state) - {"#meta", variable} or variable not in state:
            raise DecodeError(f"{path}.states[{index}]: malformed state")
        carrier = state[variable]
        if variable == "diagnosticCases":
            records = carrier
        else:
            carrier = _record(carrier, f"{path}.states[{index}].{variable}", {"agreement", "ledger", "records"})
            records = carrier["records"]
        if not isinstance(records, list):
            raise DecodeError(f"{path}.states[{index}]: expected records list")
        if len(records) < len(previous) or not _same_json(records[:len(previous)], previous):
            raise DecodeError(f"{path}.states[{index}]: conflicting or shrinking record prefix")
        if variable != "diagnosticCases":
            if index == 0 and records:
                raise DecodeError(f"{path}: stateful trace must begin with no records")
            if len(records) > (4 if variable == "swapTrace" else 3):
                raise DecodeError(f"{path}: trace exceeds finite record bound")
        if step < len(records):
            entry = records[step]
            fields = {"case_id", "fixture_id", "request", "payload"} if variable == "diagnosticCases" else {"request", "beforeLedger", "afterLedger", "payload"}
            entry = _record(entry, f"{path}.states[{index}].record[{step}]", fields)
            occurrences.append((index, entry))
        previous = records
    if not occurrences:
        raise DecodeError(f"{path}: missing indexed trace record")
    return variable, occurrences


def _verify_provenance(case: dict[str, Any], input_root: Path, path: str,
                       declared_pins: dict[str, str], cache: dict[str, tuple[bytes, object]]) -> None:
    """Check every repeated raw occurrence, without trusting exporter normalization."""
    evidence_list = case["provenance"]
    if not isinstance(evidence_list, list) or not evidence_list:
        raise DecodeError(f"{path}.provenance: expected nonempty list")
    trace_id = case["trace_id"]
    candidate = _relative_input(trace_id, input_root, path + ".trace_id")
    if trace_id not in cache:
        content = candidate.read_bytes()
        cache[trace_id] = content, _load_json(content, trace_id)
    content, raw = cache[trace_id]
    digest = hashlib.sha256(content).hexdigest()
    variable, occurrences = _raw_occurrences(raw, case["step_index"], trace_id)
    if OPTIONAL_GENERATORS[variable] not in declared_pins:
        raise DecodeError(f"{path}: missing generator source pin for {variable}")
    expected_indices = {index for index, _ in occurrences}
    seen_indices: set[int] = set()
    for offset, evidence in enumerate(evidence_list):
        item = _record(evidence, f"{path}.provenance[{offset}]",
                       {"trace_id", "state_index", "step_index", "input_path", "input_sha256"})
        if item["trace_id"] != trace_id or item["input_path"] != trace_id:
            raise DecodeError(f"{path}.provenance: trace/path identity mismatch")
        if type(item["step_index"]) is not int or item["step_index"] != case["step_index"]:
            raise DecodeError(f"{path}.provenance: step index mismatch")
        index = item["state_index"]
        if type(index) is not int or index not in expected_indices or index in seen_indices:
            raise DecodeError(f"{path}.provenance: missing, duplicate, or invalid state index")
        if item["input_sha256"] != digest:
            raise DecodeError(f"{path}.provenance: input file/hash mismatch")
        seen_indices.add(index)
    if seen_indices != expected_indices:
        raise DecodeError(f"{path}.provenance: omitted repeated-prefix occurrence")
    expected_tag = {"swapTrace": "SwapComputedA", "installmentTrace": "InstallmentComputedA",
                    "diagnosticCases": "CaseComputedA"}[variable]
    for _, record in occurrences:
        _, payload = _enum(record["payload"], path + ".provenance.payload", {expected_tag})
        fields = _record(payload, path + ".provenance.payload.value", {"raw", "projection", "effects"})
        if variable == "diagnosticCases":
            raw_id = record["case_id"]
            if not isinstance(raw_id, str) or raw_id not in CASE_FIXTURES:
                raise DecodeError(f"{path}: unknown diagnostic case identity")
            fixture = CASE_FIXTURES[raw_id]
            if record["fixture_id"] != fixture:
                raise DecodeError(f"{path}: diagnostic fixture identity differs")
            suffix = raw_id
        else:
            fixture = "canonical-swap-v1" if variable == "swapTrace" else "installment-two-when-v1"
            suffix = str(case["step_index"])
        if case["case_id"] != trace_id + "::" + suffix or case["fixture_id"] != fixture:
            raise DecodeError(f"{path}: case/fixture identity differs from indexed raw record")
        for name, observed in (("request", record["request"]), ("result", fields["raw"]),
                               ("projection", fields["projection"]), ("effects", fields["effects"])):
            if not _same_json(case[name], observed):
                raise DecodeError(f"{path}.provenance.{name}: copied payload differs from indexed raw record")


def _verify_complete_inventory(document: dict[str, Any], input_root: Path,
                               cache: dict[str, tuple[bytes, object]]) -> None:
    """Require every declared raw input and every final raw record, not a subset."""
    binding_path = input_root / "source-bindings.json"
    bindings = _record(_load_json(binding_path.read_bytes(), str(binding_path)), "source-bindings",
                       {"schema_version", "source_pins", "input_pins"})
    if type(bindings["schema_version"]) is not int or bindings["schema_version"] != 1:
        raise DecodeError("source-bindings.schema_version: expected integer 1")
    if not _same_json(bindings["source_pins"], document["source_pins"]):
        raise DecodeError("source-bindings.source_pins: differs from case document")
    pins = bindings["input_pins"]
    if not isinstance(pins, dict) or not pins:
        raise DecodeError("source-bindings.input_pins: expected nonempty inventory")
    actual_files = {path.relative_to(input_root).as_posix() for path in input_root.rglob("*.itf.json") if path.is_file()}
    if actual_files != set(pins):
        raise DecodeError("source-bindings.input_pins: missing or unlisted ITF input files")
    expected_positions: set[tuple[str, int]] = set()
    for name, digest in pins.items():
        candidate = _relative_input(name, input_root, "source-bindings.input_pins")
        if not name.endswith(".itf.json") or not isinstance(digest, str) or re.fullmatch(r"[0-9a-f]{64}", digest) is None:
            raise DecodeError("source-bindings.input_pins: malformed ITF name/digest")
        content = candidate.read_bytes()
        if hashlib.sha256(content).hexdigest() != digest:
            raise DecodeError(f"source-bindings.input_pins.{name}: input hash mismatch")
        raw = _load_json(content, name)
        cache[name] = content, raw
        variable, _ = _raw_occurrences(raw, 0, name)
        final = raw["states"][-1][variable]
        records = final if variable == "diagnosticCases" else final["records"]
        expected_positions.update((name, step) for step in range(len(records)))
    represented: set[tuple[str, int]] = set()
    for index, case in enumerate(document["cases"]):
        if not isinstance(case, dict) or not isinstance(case.get("trace_id"), str) or type(case.get("step_index")) is not int:
            raise DecodeError(f"cases[{index}]: invalid inventory position")
        position = case["trace_id"], case["step_index"]
        if position in represented:
            raise DecodeError(f"cases[{index}]: duplicate raw record inventory position")
        represented.add(position)
    if represented != expected_positions:
        raise DecodeError("document.cases: missing or extra exports relative to complete raw inventory")


def check_document(document: object, *, source_root: Path = ROOT, input_root: Path | None = None,
                   require_complete: bool = False) -> CheckReport:
    """Check selected records; require_complete also gates the declared inventory.

    The CLI always requests complete inventory unless --allow-subset is explicit.
    Both modes require full indexed provenance for every selected record.
    """
    try:
        data = _record(document, "document", {"schema_version", "source_pins", "cases"})
        if type(data["schema_version"]) is not int or data["schema_version"] != 1:
            raise DecodeError("document.schema_version: expected integer 1")
        if not isinstance(data["source_pins"], dict) or not set(PINNED).issubset(data["source_pins"]):
            raise DecodeError("document.source_pins: missing required base pins")
        if set(data["source_pins"]) - set(PINNED) - set(OPTIONAL_GENERATORS.values()) - {"scripts/export_s02_candidate_a_cases.py"}:
            raise DecodeError("document.source_pins: unsupported pin path")
        for name, expected in PINNED.items():
            if data["source_pins"][name] != expected:
                raise DecodeError(f"document.source_pins.{name}: differs from frozen pin")
            if hashlib.sha256((source_root / name).read_bytes()).hexdigest() != expected:
                raise DecodeError(f"frozen source changed locally: {name}")
        if not isinstance(data["cases"], list) or not data["cases"]:
            raise DecodeError("document.cases: expected nonempty list")
        for reference, name in ((compute_transaction, "moriarty/core.py"),
                                (reduce_to_quiescence, "moriarty/core.py"),
                                (canonical_swap, "moriarty/swap.py")):
            origin = inspect.getsourcefile(reference)
            if origin is None or hashlib.sha256(Path(origin).read_bytes()).hexdigest() != PINNED[name]:
                raise DecodeError(f"imported reference {reference.__name__}: source differs from frozen {name}")
        if input_root is None:
            raise DecodeError("input_root is required for verified raw provenance")
        for name, expected in data["source_pins"].items():
            if not isinstance(expected, str) or re.fullmatch(r"[0-9a-f]{64}", expected) is None:
                raise DecodeError(f"document.source_pins.{name}: malformed digest")
            if hashlib.sha256((source_root / name).read_bytes()).hexdigest() != expected:
                raise DecodeError(f"document.source_pins.{name}: source hash mismatch")
        identifiers: set[str] = set()
        cache: dict[str, tuple[bytes, object]] = {}
        if require_complete:
            _verify_complete_inventory(data, input_root, cache)
        differences: list[str] = []
        for index, case in enumerate(data["cases"]):
            if isinstance(case, dict) and isinstance(case.get("case_id"), str):
                if case["case_id"] in identifiers:
                    raise DecodeError(f"cases[{index}].case_id: duplicate")
                identifiers.add(case["case_id"])
            differences.extend(_check_case(case, index, data["source_pins"]))
            _verify_provenance(case, input_root, f"cases[{index}]", data["source_pins"], cache)
        return CheckReport(not differences, tuple(differences))
    except (DecodeError, TypeError, ValueError, OSError) as error:
        return CheckReport(False, (str(error),))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", required=True, type=Path)
    parser.add_argument("--report", type=Path)
    parser.add_argument("--source-root", type=Path, default=ROOT)
    parser.add_argument("--input-root", type=Path)
    parser.add_argument("--allow-subset", action="store_true",
                        help="Explicit selected-record control mode; do not require complete input inventory")
    arguments = parser.parse_args()
    try:
        document = _load_json(arguments.cases.read_bytes(), str(arguments.cases))
        report = check_document(document, source_root=arguments.source_root, input_root=arguments.input_root,
                                require_complete=not arguments.allow_subset)
    except (DecodeError, ValueError, OSError) as error:
        report = CheckReport(False, (str(error),))
    payload: dict[str, Any] = {"ok": report.ok, "differences": list(report.differences),
                             "scope": "selected-records" if arguments.allow_subset else "complete-inventory"}
    if arguments.report is not None:
        arguments.report.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    for difference in report.differences:
        print(difference)
    return 0 if report.ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
