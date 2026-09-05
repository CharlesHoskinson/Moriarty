#!/usr/bin/env python3
"""Independent finite-record checker for Candidate A correspondence exports.

This module deliberately does not import the Candidate A exporter, Quint
projection code, or any effect-extraction helper.  It is a typed TDD shell;
the decoder and reference comparison are added after its assertion RED.
"""

from __future__ import annotations

import argparse
import hashlib
import json
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
    if text in {"", "-"} or (text.startswith("-") and not text[1:].isdigit()) or (not text.startswith("-") and not text.isdigit()):
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
    if type(value) is int or (isinstance(value, dict) and "#bigint" in value):
        return _integer(value, path)
    return TIMES[_nullary(value, path, set(TIMES))]


def _constant(value: object, path: str) -> Constant:
    tag, payload = _enum(value, path, {"ConstantA"})
    del tag
    return Constant(_integer(payload, path + ".value"))


def _action(value: object, path: str) -> Deposit | Choice:
    tag, payload = _enum(value, path, {"DepositA", "ChoiceA"})
    if tag == "DepositA":
        item = _record(payload, path + ".DepositA", {"account", "depositor", "amount"})
        return Deposit(_account(item["account"], path + ".account"), _party(item["depositor"], path + ".depositor"), _constant(item["amount"], path + ".amount"))
    item = _record(payload, path + ".ChoiceA", {"id", "chooser", "lower", "upper"})
    choice = _nullary(item["id"], path + ".id", set(CHOICES))
    return Choice(CHOICES[choice], _party(item["chooser"], path + ".chooser"), _integer(item["lower"], path + ".lower"), _integer(item["upper"], path + ".upper"))


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
        tag, payload = _enum(raw_nodes[node], f"{path}.nodes[{node}]", {"CloseA", "PayA", "IfA", "WhenA"})
        if tag == "CloseA":
            if payload != {"#tup": []}:
                raise DecodeError(f"{path}.nodes[{node}]: malformed CloseA")
            result: object = Close()
        elif tag == "PayA":
            data = _record(payload, f"{path}.nodes[{node}].PayA", {"account", "payee", "amount", "continuation"})
            result = Pay(_account(data["account"], path + ".account"), _party(data["payee"], path + ".payee"), _constant(data["amount"], path + ".amount"), build(_node(data["continuation"], path + ".continuation")))
        elif tag == "IfA":
            data = _record(payload, f"{path}.nodes[{node}].IfA", {"observation", "thenNode", "elseNode"})
            observation_tag, observation = _enum(data["observation"], path + ".observation", {"ChoiceEqualsA"})
            del observation_tag
            obs = _record(observation, path + ".observation.value", {"id", "expected"})
            choice = _nullary(obs["id"], path + ".observation.id", set(CHOICES))
            result = If(ChoiceEquals(CHOICES[choice], _integer(obs["expected"], path + ".observation.expected")), build(_node(data["thenNode"], path + ".thenNode")), build(_node(data["elseNode"], path + ".elseNode")))
        else:
            data = _record(payload, f"{path}.nodes[{node}].WhenA", {"cases", "timeout", "timeoutNode"})
            if not isinstance(data["cases"], list):
                raise DecodeError(f"{path}.nodes[{node}].cases: expected list")
            cases: list[Case] = []
            for index, raw_case in enumerate(data["cases"]):
                case_data = _record(raw_case, f"{path}.nodes[{node}].cases[{index}]", {"caseAction", "continuation"})
                cases.append(Case(_action(case_data["caseAction"], path + ".caseAction"), build(_node(case_data["continuation"], path + ".continuation"))))
            result = When(tuple(cases), _time(data["timeout"], path + ".timeout"), build(_node(data["timeoutNode"], path + ".timeoutNode")))
        built[node] = result
        return result
    return build(root), {node: build(node) for node in NODES}, root


def _state(value: object, path: str) -> State:
    item = _record(value, path, {"accounts", "choices", "continuation", "minimumTime"})
    _node(item["continuation"], path + ".continuation")
    accounts: dict[Account, int] = {}
    for index, (key, amount) in enumerate(_map(item["accounts"], path + ".accounts")):
        account = _account(key, f"{path}.accounts[{index}].key")
        if account in accounts:
            raise DecodeError(f"{path}.accounts: duplicate account")
        quantity = _integer(amount, f"{path}.accounts[{index}].value")
        if quantity < 0:
            raise DecodeError(f"{path}.accounts: negative account balance")
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
            choices[choice] = _integer(payload, f"{path}.choices[{index}].value")
    raw_choices = _map(item["choices"], path + ".choices")
    if len(raw_choices) != len(CHOICES) or set(seen_choice_ids) != set(CHOICES.values()):
        raise DecodeError(f"{path}.choices: choice domain differs from fixed five choices")
    return State.from_accounts(accounts, min_time=_time(item["minimumTime"], path + ".minimumTime")).__class__(accounts=State.from_accounts(accounts).accounts, choices=tuple(sorted(choices.items())), min_time=_time(item["minimumTime"], path + ".minimumTime"))


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
        return DepositInput(_account(data["account"], path + ".account"), _party(data["depositor"], path + ".depositor"), _integer(data["quantity"], path + ".quantity"))
    data = _record(inner, path + ".ChoiceInputA", {"id", "chooser", "chosen"})
    choice = _nullary(data["id"], path + ".id", set(CHOICES))
    return ChoiceInput(CHOICES[choice], _party(data["chooser"], path + ".chooser"), _integer(data["chosen"], path + ".chosen"))


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
    tag, payload = _enum(value, path, {"TransactionComputedA"})
    del tag
    data = _record(payload, path + ".TransactionComputedA", {"accepted", "state", "error", "payments", "warnings", "reductions"})
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
    effects: list[tuple[str, Party | Account, str, Party | Account, Token, int]] = []
    for payment in prefix.payments:
        effects.append(("escrow", payment.source, "wallet", payment.to, payment.token, payment.quantity))
    if isinstance(supplied, DepositInput):
        effects.append(("wallet", supplied.depositor, "escrow", supplied.account, supplied.account.token, supplied.quantity))
    for payment in result.payments[len(prefix.payments):]:
        effects.append(("escrow", payment.source, "wallet", payment.to, payment.token, payment.quantity))
    return tuple(effects)


def _result_node(raw_result: object, path: str) -> str:
    _, payload = _enum(raw_result, path, {"TransactionComputedA"})
    data = _record(payload, path + ".TransactionComputedA", {"accepted", "state", "error", "payments", "warnings", "reductions"})
    state = _record(data["state"], path + ".state", {"accounts", "choices", "continuation", "minimumTime"})
    return _node(state["continuation"], path + ".state.continuation")


def _projection(value: object, path: str) -> tuple[TransactionResult, object, str]:
    data = _record(value, path, {"accepted", "error", "payments", "warnings", "state", "reductions"})
    state = _record(data["state"], path + ".state", {"accounts", "choices", "continuation", "minimumTime"})
    continuation = _record(state["continuation"], path + ".state.continuation", {"program", "node"})
    program, _, _ = _decode_program(continuation["program"], path + ".state.continuation.program")
    node = _node(continuation["node"], path + ".state.continuation.node")
    # Projection choice keys are Core strings, whereas raw AState keys are AChoiceId enums.
    inverse_choices = {name: tag for tag, name in CHOICES.items()}
    projected_choice_pairs = _map(state["choices"], path + ".state.choices")
    native_choices: list[list[object]] = []
    for index, (key, chosen) in enumerate(projected_choice_pairs):
        if not isinstance(key, str) or key not in inverse_choices:
            raise DecodeError(f"{path}.state.choices[{index}]: unsupported projected choice key")
        native_choices.append([{"tag": inverse_choices[key], "value": {"#tup": []}}, chosen])
    native_state = {"accounts": state["accounts"], "choices": {"#map": native_choices}, "continuation": continuation["node"], "minimumTime": state["minimumTime"]}
    wrapped = {"tag": "TransactionComputedA", "value": {**data, "state": native_state}}
    return _result(wrapped, path), program, node


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
    generator = OPTIONAL_GENERATORS.get(data["trace_id"])
    if generator is None:
        raise DecodeError(f"{path}.trace_id: unsupported trace variable")
    if generator not in declared_pins:
        raise DecodeError(f"{path}: required generator pin missing for {data['trace_id']}")
    if not isinstance(data["provenance"], list) or not data["provenance"]:
        raise DecodeError(f"{path}.provenance: expected nonempty list")
    for provenance_index, provenance in enumerate(data["provenance"]):
        evidence = _record(provenance, f"{path}.provenance[{provenance_index}]", {"trace_id", "state_index", "step_index", "input_path", "input_sha256"})
        if evidence["trace_id"] != data["trace_id"] or evidence["step_index"] != data["step_index"]:
            raise DecodeError(f"{path}.provenance[{provenance_index}]: trace identity mismatch")
        if type(evidence["state_index"]) is not int or evidence["state_index"] < 0 or not isinstance(evidence["input_path"], str) or evidence["input_path"].startswith("/") or not isinstance(evidence["input_sha256"], str) or len(evidence["input_sha256"]) != 64 or any(character not in "0123456789abcdef" for character in evidence["input_sha256"]):
            raise DecodeError(f"{path}.provenance[{provenance_index}]: malformed evidence")
    request = _record(data["request"], path + ".request", {"program", "before", "input", "now"})
    program, nodes, _ = _decode_program(request["program"], path + ".request.program")
    fixture_nodes, fixture_root = _fixture_nodes(data["fixture_id"])
    if program != fixture_nodes[fixture_root] or nodes != fixture_nodes:
        return [f"{path}.request.program: differs from fixture {data['fixture_id']}"]
    before = _state(request["before"], path + ".request.before")
    supplied = _input(request["input"], path + ".request.input")
    now = _time(request["now"], path + ".request.now")
    current_node = _state_node(request["before"], path + ".request.before")
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
    if projection_program != program:
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


def _verify_provenance(case: dict[str, Any], input_root: Path, path: str) -> None:
    """Bind copied fields to one indexed raw ITF record without normalization."""
    occurrences = 0
    for evidence in case["provenance"]:
        candidate = (input_root / evidence["input_path"]).resolve()
        if input_root.resolve() not in candidate.parents:
            raise DecodeError(f"{path}.provenance: input path escapes input root")
        if not candidate.is_file() or hashlib.sha256(candidate.read_bytes()).hexdigest() != evidence["input_sha256"]:
            raise DecodeError(f"{path}.provenance: input file/hash mismatch")
        raw = json.loads(candidate.read_text())
        trace = evidence["trace_id"]
        if trace in {"swapTrace", "installmentTrace"}:
            states = raw.get("states") if isinstance(raw, dict) else None
            if not isinstance(states, list) or evidence["state_index"] >= len(states):
                raise DecodeError(f"{path}.provenance: missing indexed ITF state")
            state = states[evidence["state_index"]]
            entry = state.get(trace) if isinstance(state, dict) else None
            records = entry.get("records") if isinstance(entry, dict) else None
            if not isinstance(records, list) or evidence["step_index"] >= len(records):
                raise DecodeError(f"{path}.provenance: missing indexed trace record")
            record = records[evidence["step_index"]]
            payload = record.get("payload") if isinstance(record, dict) else None
            if not isinstance(record, dict) or not isinstance(payload, dict):
                raise DecodeError(f"{path}.provenance: malformed trace record")
            tag, value = _enum(payload, path + ".provenance.payload", {"SwapComputedA", "InstallmentComputedA"})
            del tag
            expected = _record(value, path + ".provenance.payload.value", {"raw", "projection", "effects"})
            source_request = record.get("request")
        else:
            entries = raw.get("diagnosticCases") if isinstance(raw, dict) else None
            if not isinstance(entries, list) or evidence["step_index"] >= len(entries):
                raise DecodeError(f"{path}.provenance: missing indexed diagnostic case")
            entry = entries[evidence["step_index"]]
            if not isinstance(entry, dict): raise DecodeError(f"{path}.provenance: malformed diagnostic case")
            source_request = entry.get("request")
            tag, value = _enum(entry.get("payload"), path + ".provenance.payload", {"CaseComputedA"})
            del tag
            expected = _record(value, path + ".provenance.payload.value", {"raw", "projection", "effects"})
        if source_request != case["request"] or expected["raw"] != case["result"] or expected["projection"] != case["projection"] or expected["effects"] != case["effects"]:
            raise DecodeError(f"{path}.provenance: copied payload differs from indexed raw record")
        occurrences += 1
    if occurrences != 1:
        raise DecodeError(f"{path}.provenance: expected exactly one indexed source occurrence")


def check_document(document: object, *, source_root: Path = ROOT, input_root: Path | None = None) -> CheckReport:
    """Check one schema-1 finite record document without exporter dependencies."""
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
        identifiers: set[str] = set()
        differences: list[str] = []
        for index, case in enumerate(data["cases"]):
            if isinstance(case, dict) and isinstance(case.get("case_id"), str):
                if case["case_id"] in identifiers:
                    raise DecodeError(f"cases[{index}].case_id: duplicate")
                identifiers.add(case["case_id"])
            differences.extend(_check_case(case, index, data["source_pins"]))
            if input_root is not None:
                _verify_provenance(case, input_root, f"cases[{index}]")
        return CheckReport(not differences, tuple(differences))
    except (DecodeError, RecursionError, TypeError, ValueError) as error:
        return CheckReport(False, (str(error),))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", required=True, type=Path)
    parser.add_argument("--report", type=Path)
    parser.add_argument("--source-root", type=Path, default=ROOT)
    parser.add_argument("--input-root", type=Path)
    arguments = parser.parse_args()
    report = check_document(json.loads(arguments.cases.read_text()), source_root=arguments.source_root, input_root=arguments.input_root)
    payload: dict[str, Any] = {"ok": report.ok, "differences": list(report.differences)}
    if arguments.report is not None:
        arguments.report.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    for difference in report.differences:
        print(difference)
    return 0 if report.ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
