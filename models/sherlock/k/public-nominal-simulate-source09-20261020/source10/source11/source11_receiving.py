#!/usr/bin/env python3
"""Additive Source11 NIWindowObservation K query renderer and whole-window receiver.

Designated security/source-owner implementation extending existingSource10
receiving/rendering, CompleteKWire, nominal_trace_codec, c6_data framing/strict-JSON
loader, and R2 KAST JSONv4 compiled-label transport to the 211-symbol Source11
compiled registry and NIWindowObservation(niRunWindow / niWindowObserved) root.
No model evaluation, subprocess execution, or semantic oracle derivation occurs here.
"""

from __future__ import annotations

import argparse
import copy
from dataclasses import dataclass
import hashlib
import json
from pathlib import Path
import re
import sys
from typing import Any, Iterable, Mapping, Sequence


UINT128_MAX_EXCLUSIVE = 1 << 128
INDEX64_MAX_EXCLUSIVE = 1 << 64

RENDER_SCHEMA = "sherlock-source11-ni-window-render/1"
DECODE_SCHEMA = "sherlock-source11-ni-window-decoded-observation/1"
REGISTRY_SCHEMA = "source11-actual-qualified-receiving-registry-DATA/1"

ACTUAL_211_REGISTRY_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/postcompile-actual-qualification01/ACTUAL-COMPILED-KLABEL-RECEIVING-REGISTRY.DATA.json",
    "bytes": 146444,
    "sha256": "7a1594f5ef4b78ac65ed1a88f764b2ea0d4491352682cbc82cb645d329b2380e",
}
ACTUAL_RECEIVING_REGISTRY_PIN = ACTUAL_211_REGISTRY_PIN
NEXT_RECEIVER_BINDING_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/postcompile-actual-qualification01/NEXT-DESIGNATED-RECEIVER-BINDING.DATA.json",
    "bytes": 7255,
    "sha256": "c2cbaa58494ac69de97cf93dd94f92a76c9a237fee1607a894803cb1ec234627",
}
COMPILED_REUSE_EXACT15_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/postcompile-actual-qualification01/COMPILED-REUSE-EXACT15.json",
    "bytes": 5745,
    "sha256": "c40535e93120e90d885e27092cccc6d5f379554255d98bc06c9ddae3cbecd6c4",
}
SOURCE_FREEZE_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/SOURCE-FREEZE.json",
    "bytes": 7419,
    "sha256": "9f92e671898959b912912c0a0406428c6ff75a9a6cd1d85367e8f586ebc57cbc",
}
SOURCE_DECLARATION_PINS_ADDENDUM_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/designated-security-receiver-source-request01/SOURCE-DECLARATION-PINS-ADDENDUM.DATA.json",
    "bytes": 1148,
    "sha256": "36dbc2e2ac196f2561ea770d49033398dc300c2c1390abf5cab989a15ced8955",
}
PUBLIC_OBSERVATION_AND_WIRE_CONTRACT_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/SAFE/PUBLIC-OBSERVATION-AND-WIRE-CONTRACT.json",
    "bytes": 1907,
    "sha256": "a3cb458b94c906a3a566eba1abb13cb5ea9faff6462c6e13359aa3c5af1ac128",
}
DEFINITION_KORE_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/actual/programview-state-event-source11-kompiled/definition.kore",
    "bytes": 2010784,
    "sha256": "55197f1ab942158a41f53b51995cdef08197c7b9e96fce2b9330e6de107c0e0b",
}
ORIGINAL_QUALIFICATION_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/postcompile-actual-qualification01/ACTUAL-COMPLETE-USER-LABEL-SORT-ARITY-MAP.json",
    "bytes": 382367,
    "sha256": "f8bb43735a651b4e5e41e384c6f7bbd2f70dd359878435b35717fc6a252a0366",
}
COMPILED_TEXT_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/actual/programview-state-event-source11-kompiled/compiled.txt",
    "bytes": 432354,
    "sha256": "3bb7955b4b756ee452bc8e66296399d4aa5c9e07cb5203a1d638d70fde7defc0",
}
PUBLIC_API_PIN = {
    "path": "/home/charl/Moriarty/.worktrees/sherlock-c6-physical-host-codecs-20261019/models/sherlock/k/nominal-full-state-20261018/context/PUBLIC-API.json",
    "bytes": 10944,
    "sha256": "ebbdd2ea4d79215dba9d1514a8e56af6eed6921e3daeb90164b2c8970f85c6f3",
}
COMPLETE_K_WIRE_PIN = {
    "path": "/home/charl/Moriarty/.worktrees/sherlock-c6-physical-host-codecs-20261019/models/sherlock/k/nominal-full-state-20261018/capture/complete_k_wire.py",
    "bytes": 13275,
    "sha256": "b475887749e86b77b55645651d7df442546d768684d855b316971803408dcb06",
}
NOMINAL_TRACE_CODEC_PIN = {
    "path": "/home/charl/Moriarty/.worktrees/sherlock-c6-physical-host-codecs-20261019/models/sherlock/k/nominal-full-state-20261018/capture/nominal_trace_codec.py",
    "bytes": 14188,
    "sha256": "6ddfc623f8870bffe5477933c24b52458c88e19fcc676127612fa3b4eb928cb6",
}
POSITIVE_IDENTITY_PORT_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-identity-source10/source/positive_identity_port.py",
    "bytes": 10293,
    "sha256": "fc1dcadf0e3e8b18167fd41cd1ee9773e1dc2293161b575ab035f6148772debc",
}
PROGRAM_VIEW_TO_NGPROGRAM_PIN = {
    "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-identity-source10/source/program_view_to_ngprogram.py",
    "bytes": 26468,
    "sha256": "ab7408993da8b1e46e8460b8d2a13ed8d436af1da735093ece9627db5f469fa0",
}
K_SOURCE_PINS = (
    {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/source/nominal-product-syntax.k",
        "bytes": 5090,
        "sha256": "750f9b820a672865b2a1abc50bef4f417f2c6c02e7b77f661a0431388dacab8c",
    },
    {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/source/nominal-product.k",
        "bytes": 21856,
        "sha256": "29da74d7ab2b89cb23155521b94ffb6880920befcbf9806941ba97826d47b197",
    },
    {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/source/normal-trace-observations.k",
        "bytes": 1735,
        "sha256": "6cd7fc81c394a06ed7a963f8be21a36a57dbeead252820203e65de618a795af0",
    },
    {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/source/programview-identity-source10.k",
        "bytes": 1308,
        "sha256": "d0dfcb4e1ee97011d3611ef409810d10357548fdad7d9a88a311b562287fd094",
    },
    {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/source/programview-state-event-source11.k",
        "bytes": 728,
        "sha256": "c595a6b053fccdeaa158961f71eaa25c29704ca2597826edda3e678d80f1ece2",
    },
    {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/source/public-nominal-source09.k",
        "bytes": 3530,
        "sha256": "6e961f2c8efc205fc83e16d0e6448753266ffd6c6ebf18b960e99d080cca08c1",
    },
    {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-state-event-source11/tests/programview-state-event-source11-laws.k",
        "bytes": 5508,
        "sha256": "efc474558276370c88d2cc2212e4da24a87b867b419bcca18912495c651a24a4",
    },
)
K_SOURCE_PINS_BY_PATH = {row["path"]: row for row in K_SOURCE_PINS}

EXPECTED_NEW_SURFACE_NAMES = (
    "ngAppendState",
    "ngNoStates",
    "ngObserve",
    "ngObserveLoop",
    "ngObserveNext",
    "ngReverseStates",
    "ngStateRow",
    "ngTape",
    "niRunWindow",
    "niWindowObserved",
    "nwCallerAfter",
    "nwCallerBefore",
    "nwCheckCaller",
    "nwCheckFour",
    "nwCheckHalted",
    "nwCheckOutside",
    "nwCheckTwo",
    "nwCheckZero",
    "nwExpectedTape",
    "nwHeldOutProgram",
    "nwHeldOutSource",
    "nwLawChronologicalTwo",
    "nwLawCompleteFourSteps",
    "nwLawDistinctCallerAndTailMaps",
    "nwLawHaltedNoStutter",
    "nwLawOutsideNoStutter",
    "nwLawZeroExactSource",
    "nwS0",
    "nwS1",
    "nwS2",
    "nwS3",
    "nwS4",
)
EXPECTED_NEW32_SURFACE_NAMES = EXPECTED_NEW_SURFACE_NAMES

ADDITIVE_NOT_FRAME_PAYLOADS = {
    "ngNotArgument": ("expr_id", "occurrence", "saved_environment"),
}
ADDITIVE_NOT_EVENT_PAYLOADS = {
    "ngNotStarted": ("expr_id", "occurrence", "operand_expr_id", "saved_environment"),
    "ngNotReturned": (
        "expr_id",
        "occurrence",
        "operand_value",
        "result",
        "restored_environment",
    ),
}


@dataclass(frozen=True)
class DecodeBudget:
    """Explicit bounded decode budgets for untrusted/printed/JSONv4 inputs."""

    max_bytes: int = 16 * 1024 * 1024
    max_tokens: int = 2_000_000
    max_nodes: int = 500_000
    max_depth: int = 4096
    max_rows: int = 65_536

    def validate(self) -> None:
        for field_name in (
            "max_bytes",
            "max_tokens",
            "max_nodes",
            "max_depth",
            "max_rows",
        ):
            val = getattr(self, field_name)
            if type(val) is not int or val <= 0:
                raise ValueError(f"Invalid DecodeBudget.{field_name}: {val!r}")


DEFAULT_BUDGET = DecodeBudget()


def pin(path: str | Path) -> dict[str, Any]:
    resolved = Path(path).resolve()
    raw = resolved.read_bytes()
    return {
        "path": str(resolved),
        "bytes": len(raw),
        "sha256": hashlib.sha256(raw).hexdigest(),
    }


def checked(row: Mapping[str, Any]) -> bytes:
    if (
        type(row) is not dict
        or set(row) != {"path", "bytes", "sha256"}
        or type(row["path"]) is not str
        or type(row["bytes"]) is not int
        or type(row["sha256"]) is not str
        or pin(row["path"]) != row
    ):
        raise ValueError("exact file pin differs")
    return Path(row["path"]).read_bytes()


def write(path: str | Path, raw: bytes) -> dict[str, Any]:
    if type(raw) is not bytes:
        raise ValueError("write expects raw bytes")
    with Path(path).open("xb") as stream:
        stream.write(raw)
    return pin(path)


# Verify pinned existing source modules and Source11 declaration metadata before importing.
for _module_pin in (
    COMPLETE_K_WIRE_PIN,
    NOMINAL_TRACE_CODEC_PIN,
    POSITIVE_IDENTITY_PORT_PIN,
    PROGRAM_VIEW_TO_NGPROGRAM_PIN,
    NEXT_RECEIVER_BINDING_PIN,
    COMPILED_REUSE_EXACT15_PIN,
    SOURCE_FREEZE_PIN,
    SOURCE_DECLARATION_PINS_ADDENDUM_PIN,
    PUBLIC_OBSERVATION_AND_WIRE_CONTRACT_PIN,
):
    checked(_module_pin)

_CAPTURE_DIR = str(Path(COMPLETE_K_WIRE_PIN["path"]).parent)
_SOURCE10_DIR = str(Path(POSITIVE_IDENTITY_PORT_PIN["path"]).parent)
if _CAPTURE_DIR not in sys.path:
    sys.path.insert(0, _CAPTURE_DIR)
if _SOURCE10_DIR not in sys.path:
    sys.path.insert(0, _SOURCE10_DIR)

import complete_k_wire as _ckw  # noqa: E402
import nominal_trace_codec as _ntc  # noqa: E402
import positive_identity_port as _port10  # noqa: E402
import program_view_to_ngprogram as _port_base  # noqa: E402

CompleteKWire = _ckw.CompleteKWire
ObservationFormatError = _ckw.ObservationFormatError
Term = _ckw.Term
T = _ntc.T
atom = _ntc.atom
LEX = _ntc.LEX
PROGRAM_FIELDS = _port10.PROGRAM_FIELDS
PortError = _port_base.PortError


def _require_keys(value: Any, want: Iterable[str], context: str = "object") -> dict[str, Any]:
    expected = set(want)
    if type(value) is not dict or set(value) != expected:
        raise ObservationFormatError(f"exact {context} keyset differs")
    return value


def _check_index64(value: Any, label: str) -> int:
    if type(value) is not int or value < 0 or value >= INDEX64_MAX_EXCLUSIVE:
        raise ObservationFormatError(f"Expected nonnegative index64 for {label}, got {value!r}")
    return value


def _check_uint128(value: Any, label: str) -> int:
    if type(value) is not int or value < 0 or value >= UINT128_MAX_EXCLUSIVE:
        raise ObservationFormatError(f"Expected UInt128 in [0, 2**128) for {label}, got {value!r}")
    return value


def _check_ni_int(value: Any) -> int:
    if (
        type(value) is not int
        or value <= -UINT128_MAX_EXCLUSIVE
        or value >= UINT128_MAX_EXCLUSIVE
    ):
        raise ObservationFormatError(f"NIJSON integer out of bounded range: {value!r}")
    return value


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if type(key) is not str:
            raise ObservationFormatError("JSON object key must be str")
        if key in result:
            raise ObservationFormatError("duplicate JSON key: " + key)
        result[key] = value
    return result


def _validate_json_value_budget(value: Any, budget: DecodeBudget) -> None:
    budget.validate()
    nodes = 0
    stack: list[tuple[Any, int]] = [(value, 0)]
    while stack:
        current, depth = stack.pop()
        if depth > budget.max_depth:
            raise ObservationFormatError("JSON depth exceeds DecodeBudget.max_depth")
        nodes += 1
        if nodes > budget.max_nodes:
            raise ObservationFormatError("JSON node count exceeds DecodeBudget.max_nodes")
        if current is None or type(current) is bool or type(current) is str:
            continue
        if type(current) is int:
            _check_ni_int(current)
            continue
        if type(current) is list:
            if len(current) > budget.max_rows:
                raise ObservationFormatError("JSON array length exceeds DecodeBudget.max_rows")
            for item in reversed(current):
                stack.append((item, depth + 1))
            continue
        if type(current) is dict:
            if len(current) > budget.max_rows:
                raise ObservationFormatError("JSON object size exceeds DecodeBudget.max_rows")
            for k, v in reversed(list(current.items())):
                if type(k) is not str:
                    raise ObservationFormatError("JSON key must be str")
                nodes += 1
                if nodes > budget.max_nodes:
                    raise ObservationFormatError("JSON node count exceeds DecodeBudget.max_nodes")
                stack.append((v, depth + 1))
            continue
        raise ObservationFormatError(f"Unsupported JSON value type: {type(current).__name__}")


def loads(text: str | bytes, *, budget: DecodeBudget = DEFAULT_BUDGET) -> Any:
    budget.validate()
    if type(text) is bytes:
        raw_len = len(text)
        try:
            decoded_text = text.decode("utf-8")
        except UnicodeDecodeError as exc:
            raise ObservationFormatError("Invalid UTF-8 in JSON input") from exc
    elif type(text) is str:
        raw_len = len(text.encode("utf-8"))
        decoded_text = text
    else:
        raise ObservationFormatError("loads expects str or bytes")
    if raw_len > budget.max_bytes:
        raise ObservationFormatError("JSON byte length exceeds DecodeBudget.max_bytes")
    try:
        parsed = json.loads(
            decoded_text,
            object_pairs_hook=unique_object,
            parse_constant=lambda x: (_ for _ in ()).throw(
                ObservationFormatError("Nonfinite JSON constant: " + str(x))
            ),
        )
    except ObservationFormatError:
        raise
    except Exception as exc:
        raise ObservationFormatError(f"Malformed JSON: {exc}") from exc
    _validate_json_value_budget(parsed, budget)
    return parsed


def load(path: str | Path, *, budget: DecodeBudget = DEFAULT_BUDGET) -> Any:
    raw = Path(path).read_bytes()
    return loads(raw, budget=budget)


def same(a: Any, b: Any) -> bool:
    """Iterative type-exact structural equality (never equates Bool and Int)."""
    stack: list[tuple[Any, Any]] = [(a, b)]
    while stack:
        left, right = stack.pop()
        if type(left) is not type(right):
            return False
        if type(left) is dict:
            if left.keys() != right.keys():
                return False
            for k in left:
                stack.append((left[k], right[k]))
        elif type(left) is list or type(left) is tuple:
            if len(left) != len(right):
                return False
            for x, y in zip(left, right):
                stack.append((x, y))
        elif left != right:
            return False
    return True


def kore_label(raw: str) -> str:
    """R2 restricted declared label alphabet encoding (ModuleToKORE/StringUtil)."""
    if type(raw) is not str or not raw:
        raise ObservationFormatError("compiled_klabel must be a nonempty string")
    codes = {"(": "LPar", ")": "RPar", ",": "Comm", "_": "Unds"}
    result = "Lbl"
    quoted = False
    for char in raw:
        if char.isascii() and (char.isalnum() or char == "-"):
            if quoted:
                result += "'"
                quoted = False
            result += char
        else:
            if char not in codes:
                raise ObservationFormatError("outside declared compiled-label alphabet")
            if not quoted:
                result += "'"
                quoted = True
            result += codes[char]
    return result + ("'" if quoted else "")


def validate_receiving_registry(
    data: Mapping[str, Any],
    *,
    definition_lines: Sequence[str] | None = None,
) -> dict[str, Any]:
    """Validate the 211-symbol Source11 compiled registry against definition.kore."""
    _require_keys(
        data,
        (
            "schema",
            "qualification",
            "definition",
            "original_complete_qualification",
            "compiled_text",
            "fields",
            "rows",
            "count",
            "new_surface_names",
            "map_not_receiving_implementation",
        ),
        "receiving registry",
    )
    if data["schema"] != REGISTRY_SCHEMA:
        raise ObservationFormatError("receiving registry schema differs")
    expected_fields = [
        "surface_constructor",
        "sort",
        "argument_sorts",
        "kind",
        "compiled_klabel",
        "raw_kore_label",
        "declaring_module",
        "declaration_source_path",
        "location",
        "definition_line",
        "compiled_text_rule_use_observed",
    ]
    if data["fields"] != expected_fields:
        raise ObservationFormatError("receiving registry field list differs")
    if (
        type(data["count"]) is not int
        or data["count"] != 211
        or type(data["rows"]) is not list
        or len(data["rows"]) != 211
    ):
        raise ObservationFormatError("complete 211-row compiled registry required")
    if data["new_surface_names"] != list(EXPECTED_NEW_SURFACE_NAMES):
        raise ObservationFormatError("new_surface_names list differs")
    if data["map_not_receiving_implementation"] is not True:
        raise ObservationFormatError("map_not_receiving_implementation flag differs")
    if data["definition"] != DEFINITION_KORE_PIN:
        raise ObservationFormatError("definition.kore pin metadata differs")
    if data["original_complete_qualification"] != ORIGINAL_QUALIFICATION_PIN:
        raise ObservationFormatError("original qualification pin metadata differs")
    if data["compiled_text"] != COMPILED_TEXT_PIN:
        raise ObservationFormatError("compiled.txt pin metadata differs")

    if definition_lines is None:
        definition_lines = checked(data["definition"]).decode("utf-8").splitlines()
    for k_pin in K_SOURCE_PINS:
        checked(k_pin)

    all_symbols: dict[str, dict[str, Any]] = {}
    data_reg: dict[str, tuple[str, tuple[str, ...]]] = {}
    forward_labels: dict[str, str] = {}
    reverse_labels: dict[str, str] = {}
    seen_kore: set[str] = set()
    observed_new: list[str] = []

    for row in data["rows"]:
        if type(row) is not list or len(row) != 11:
            raise ObservationFormatError("registry row must have 11 elements")
        (
            surface_name,
            sort,
            arg_sorts,
            kind,
            compiled_klabel,
            raw_kore,
            module,
            source_path,
            location,
            def_line,
            rule_use,
        ) = row
        if (
            type(surface_name) is not str
            or not surface_name
            or type(sort) is not str
            or not sort
            or type(arg_sorts) is not list
            or not all(type(s) is str and bool(s) for s in arg_sorts)
            or kind not in ("constructor", "function")
            or type(compiled_klabel) is not str
            or type(raw_kore) is not str
            or type(module) is not str
            or not module
            or type(source_path) is not str
            or source_path not in K_SOURCE_PINS_BY_PATH
            or type(location) is not list
            or len(location) != 4
            or not all(type(loc) is int and loc >= 1 for loc in location)
            or type(def_line) is not int
            or type(rule_use) is not bool
        ):
            raise ObservationFormatError(f"Malformed registry row for {surface_name!r}")
        if (
            surface_name in all_symbols
            or compiled_klabel in reverse_labels
            or raw_kore in seen_kore
        ):
            raise ObservationFormatError(f"Duplicate registry declaration: {surface_name!r}")

        sorts_tuple = tuple(arg_sorts)
        expected_klabel = (
            surface_name
            + "("
            + ",".join("_" for _ in sorts_tuple)
            + ")_"
            + module
            + "_"
            + sort
            + "".join("_" + s for s in sorts_tuple)
        )
        if compiled_klabel != expected_klabel or raw_kore != kore_label(compiled_klabel):
            raise ObservationFormatError(
                f"Compiled label or KORE symbol mismatch for {surface_name!r}"
            )
        if not (1 <= def_line <= len(definition_lines)):
            raise ObservationFormatError(f"Definition line out of range for {surface_name!r}")
        line_body = definition_lines[def_line - 1]
        if not line_body.startswith("  symbol " + raw_kore + "{}("):
            raise ObservationFormatError(
                f"definition.kore symbol header mismatch at line {def_line}"
            )
        signature = (
            "{}("
            + ", ".join("Sort" + s + "{}" for s in sorts_tuple)
            + ") : Sort"
            + sort
            + "{} "
        )
        if signature not in line_body:
            raise ObservationFormatError(
                f"definition.kore signature mismatch for {surface_name!r}"
            )
        expected_attr = "[constructor{}()" if kind == "constructor" else "[function{}()"
        if expected_attr not in line_body:
            raise ObservationFormatError(
                f"definition.kore constructor/function attribute mismatch for {surface_name!r}"
            )
        loc_str = f"Location({location[0]},{location[1]},{location[2]},{location[3]})"
        src_str = f"Source({source_path})"
        if loc_str not in line_body or src_str not in line_body:
            raise ObservationFormatError(
                f"definition.kore Location/Source attribute mismatch for {surface_name!r}"
            )

        entry = {
            "surface_constructor": surface_name,
            "sort": sort,
            "argument_sorts": sorts_tuple,
            "kind": kind,
            "compiled_klabel": compiled_klabel,
            "raw_kore_label": raw_kore,
            "declaring_module": module,
            "declaration_source_path": source_path,
            "location": tuple(location),
            "definition_line": def_line,
            "compiled_text_rule_use_observed": rule_use,
        }
        all_symbols[surface_name] = entry
        forward_labels[surface_name] = compiled_klabel
        reverse_labels[compiled_klabel] = surface_name
        seen_kore.add(raw_kore)
        if kind == "constructor":
            data_reg[surface_name] = (sort, sorts_tuple)
        if module in (
            "SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS",
            "SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11",
            "SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11-LAWS",
        ):
            observed_new.append(surface_name)

    if sorted(observed_new) != list(EXPECTED_NEW_SURFACE_NAMES):
        raise ObservationFormatError("Observed new surface names differ from expected")
    if len(data_reg) != 134 or len(all_symbols) - len(data_reg) != 77:
        raise ObservationFormatError("Compiled registry constructor/function split differs")
    if (
        "niRunWindow" not in all_symbols
        or all_symbols["niRunWindow"]["kind"] != "function"
    ):
        raise ObservationFormatError("niRunWindow root function missing from registry")
    ni_run_window_entry = all_symbols["niRunWindow"]
    if (
        ni_run_window_entry["sort"] != "NIWindowObservation"
        or ni_run_window_entry["argument_sorts"] != ("NIJSON", "NGProgram", "Int")
        or ni_run_window_entry["declaring_module"]
        != "SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11"
    ):
        raise ObservationFormatError("niRunWindow signature differs")
    if (
        "niWindowObserved" not in all_symbols
        or all_symbols["niWindowObserved"]["kind"] != "constructor"
    ):
        raise ObservationFormatError(
            "niWindowObserved DATA constructor missing from registry"
        )
    ni_window_observed_entry = all_symbols["niWindowObserved"]
    if (
        ni_window_observed_entry["sort"] != "NIWindowObservation"
        or ni_window_observed_entry["argument_sorts"] != ("NIJSON", "NGTape")
        or ni_window_observed_entry["declaring_module"]
        != "SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11"
    ):
        raise ObservationFormatError("niWindowObserved signature differs")
    query_reg = dict(data_reg)
    query_reg["niRunWindow"] = (
        ni_run_window_entry["sort"],
        ni_run_window_entry["argument_sorts"],
    )
    return {
        "all_symbols": all_symbols,
        "data_reg": data_reg,
        "query_reg": query_reg,
        "forward_labels": forward_labels,
        "reverse_labels": reverse_labels,
    }


def _load_public_api_with_not() -> dict[str, Any]:
    raw_api = loads(checked(PUBLIC_API_PIN))
    api = copy.deepcopy(raw_api)
    for name, fields in ADDITIVE_NOT_FRAME_PAYLOADS.items():
        if name in api["frame_payload_order"]:
            raise ObservationFormatError(f"Duplicate frame payload declaration: {name}")
        api["frame_payload_order"][name] = list(fields)
    for name, fields in ADDITIVE_NOT_EVENT_PAYLOADS.items():
        if name in api["event_payload_order"]:
            raise ObservationFormatError(f"Duplicate event payload declaration: {name}")
        api["event_payload_order"][name] = list(fields)
    return api


REGISTRY_BUNDLE = validate_receiving_registry(loads(checked(ACTUAL_211_REGISTRY_PIN)))
ALL_SYMBOLS: dict[str, dict[str, Any]] = REGISTRY_BUNDLE["all_symbols"]
DATA_REG: dict[str, tuple[str, tuple[str, ...]]] = REGISTRY_BUNDLE["data_reg"]
QUERY_REG: dict[str, tuple[str, tuple[str, ...]]] = REGISTRY_BUNDLE["query_reg"]
LABELS: dict[str, str] = REGISTRY_BUNDLE["forward_labels"]
REVERSE_LABELS: dict[str, str] = REGISTRY_BUNDLE["reverse_labels"]
PUBLIC_API: dict[str, Any] = _load_public_api_with_not()


def make_term(
    name: str,
    *args: Term,
    registry: Mapping[str, tuple[str, Sequence[str]]] = DATA_REG,
) -> T:
    if type(name) is not str or name not in registry:
        raise ObservationFormatError(f"Unknown constructor: {name!r}")
    sort, child_sorts = registry[name]
    if len(args) != len(child_sorts):
        raise ObservationFormatError(f"Constructor arity differs for {name}")
    for arg, expected_sort in zip(args, child_sorts):
        if arg.sort != expected_sort:
            raise ObservationFormatError(
                f"Child sort differs for {name}: expected {expected_sort}, got {arg.sort}"
            )
    return T(sort, name, tuple(args))


def render_term(term: Term, *, budget: DecodeBudget = DEFAULT_BUDGET) -> str:
    """Iterative pretty-renderer matching nominal_trace_codec.render."""
    budget.validate()
    out: list[str] = []
    nodes = 0
    stack: list[tuple[str, Any, int]] = [("TERM", term, 0)]
    while stack:
        kind, payload, depth = stack.pop()
        if kind == "LIT":
            out.append(payload)
            continue
        if depth > budget.max_depth:
            raise ObservationFormatError("Render depth exceeds DecodeBudget.max_depth")
        nodes += 1
        if nodes > budget.max_nodes:
            raise ObservationFormatError("Render node count exceeds DecodeBudget.max_nodes")
        t: Term = payload
        if t.name == "@Int":
            if t.sort != "Int" or len(t.args) != 1 or type(t.args[0]) is not int:
                raise ObservationFormatError("Invalid @Int term")
            out.append(str(t.args[0]))
        elif t.name == "@Bool":
            if t.sort != "Bool" or len(t.args) != 1 or type(t.args[0]) is not bool:
                raise ObservationFormatError("Invalid @Bool term")
            out.append("true" if t.args[0] else "false")
        elif t.name == "@String":
            if t.sort != "String" or len(t.args) != 1 or type(t.args[0]) is not str:
                raise ObservationFormatError("Invalid @String term")
            out.append(json.dumps(t.args[0]))
        elif t.name == "@Map":
            if t.sort != "Map":
                raise ObservationFormatError("Invalid @Map term sort")
            if not t.args:
                out.append(".Map")
            else:
                if len(t.args) > budget.max_rows:
                    raise ObservationFormatError("Map entries exceed DecodeBudget.max_rows")
                stack.append(("LIT", ")", depth))
                for idx in range(len(t.args) - 1, -1, -1):
                    pair = t.args[idx]
                    if (
                        type(pair) is not tuple
                        or len(pair) != 2
                        or pair[0].sort != "Int"
                        or pair[1].sort != "NGValue"
                    ):
                        raise ObservationFormatError("Invalid @Map binding pair")
                    stack.append(("LIT", ")", depth))
                    stack.append(("TERM", pair[1], depth + 1))
                    stack.append(("LIT", " |-> ", depth))
                    stack.append(("TERM", pair[0], depth + 1))
                    stack.append(("LIT", " (" if idx > 0 else "(", depth))
                stack.append(("LIT", "(.Map ", depth))
        else:
            if len(t.args) > budget.max_rows:
                raise ObservationFormatError("Constructor arity exceeds DecodeBudget.max_rows")
            stack.append(("LIT", ")", depth))
            for idx in range(len(t.args) - 1, -1, -1):
                stack.append(("TERM", t.args[idx], depth + 1))
                if idx > 0:
                    stack.append(("LIT", ",", depth))
            stack.append(("LIT", t.name + "(", depth))
    return "".join(out)


def complete_term(text: str, *, budget: DecodeBudget = DEFAULT_BUDGET) -> str:
    """Strict single-<k>-cell unframing from c6_data.complete_term."""
    budget.validate()
    if type(text) is not str:
        raise ObservationFormatError("complete_term expects str")
    if len(text.encode("utf-8")) > budget.max_bytes:
        raise ObservationFormatError("Printed output exceeds DecodeBudget.max_bytes")
    stripped = text.strip()
    if not stripped:
        raise ObservationFormatError("Empty printed output")
    has_top = stripped.startswith("<generatedTop>") or stripped.endswith("</generatedTop>")
    if has_top:
        if not (
            stripped.startswith("<generatedTop>") and stripped.endswith("</generatedTop>")
        ):
            raise ObservationFormatError("Unbalanced <generatedTop> wrapper")
        stripped = stripped[len("<generatedTop>") : -len("</generatedTop>")].strip()
        if not (stripped.startswith("<k>") and stripped.endswith("</k>")):
            raise ObservationFormatError("<generatedTop> must directly wrap a single <k> cell")
    has_k = stripped.startswith("<k>") or stripped.endswith("</k>")
    if has_k:
        if not (stripped.startswith("<k>") and stripped.endswith("</k>")):
            raise ObservationFormatError("Unbalanced <k> cell wrapper")
        stripped = stripped[len("<k>") : -len("</k>")].strip()
        if stripped.endswith("~> .K"):
            stripped = stripped[: -len("~> .K")].rstrip()
    if not stripped:
        raise ObservationFormatError("Empty inner <k> term")
    return stripped


class Source10PrintedParser:
    """Bounded explicit-stack printed-term data parser extending PrintedTermParser."""

    def __init__(
        self,
        text: str,
        *,
        registry: Mapping[str, tuple[str, Sequence[str]]] = DATA_REG,
        budget: DecodeBudget = DEFAULT_BUDGET,
    ) -> None:
        budget.validate()
        if type(text) is not str:
            raise ObservationFormatError("PrintedTermParser expects str")
        if len(text.encode("utf-8")) > budget.max_bytes:
            raise ObservationFormatError("Printed text exceeds DecodeBudget.max_bytes")
        self.budget = budget
        self.registry = {
            name: (sort, tuple(children))
            for name, (sort, children) in registry.items()
        }
        self.tokens: list[str] = []
        self.at = 0
        self.nodes = 0
        pos = 0
        while pos < len(text):
            found = LEX.match(text, pos)
            if found is None:
                if text[pos:].strip() == "":
                    break
                raise ObservationFormatError("Unparsed printed byte at " + str(pos))
            self.tokens.append(found.group(1))
            if len(self.tokens) > budget.max_tokens:
                raise ObservationFormatError("Token count exceeds DecodeBudget.max_tokens")
            pos = found.end()

    def peek(self) -> str | None:
        return self.tokens[self.at] if self.at < len(self.tokens) else None

    def take(self, expected: str | None = None) -> str:
        token = self.peek()
        if token is None:
            raise ObservationFormatError("Unexpected end of complete tape")
        self.at += 1
        if expected is not None and token != expected:
            raise ObservationFormatError("Expected " + expected + ", got " + token)
        return token

    def _bump_node(self) -> None:
        self.nodes += 1
        if self.nodes > self.budget.max_nodes:
            raise ObservationFormatError("Node count exceeds DecodeBudget.max_nodes")

    @staticmethod
    def _parse_int_token(token: str) -> int:
        if token != "0" and re.fullmatch(r"-?[1-9]\d*", token) is None:
            raise ObservationFormatError("Expected canonical Int token, got " + repr(token))
        val = int(token)
        _check_ni_int(val)
        return val

    def _try_take_map_key(self, depth: int) -> T | None:
        checkpoint = self.at
        parens = 0
        while self.peek() == "(":
            self.take("(")
            parens += 1
            if depth + parens > self.budget.max_depth:
                self.at = checkpoint
                raise ObservationFormatError("Parse depth exceeds DecodeBudget.max_depth")
        tok = self.peek()
        if tok is None or (tok != "0" and re.fullmatch(r"-?[1-9]\d*", tok) is None):
            self.at = checkpoint
            return None
        self.take()
        for _ in range(parens):
            if self.peek() != ")":
                self.at = checkpoint
                return None
            self.take(")")
        if self.peek() != "|->":
            self.at = checkpoint
            return None
        key_val = self._parse_int_token(tok)
        _check_index64(key_val, "Map.name_id")
        self._bump_node()
        return atom("Int", key_val)

    def term(self, sort: str) -> T:
        results: list[T] = []
        stack: list[tuple[Any, ...]] = [("PARSE", sort, 0)]
        while stack:
            op = stack.pop()
            tag = op[0]
            if tag == "EXPECT":
                _, expected_tok = op
                self.take(expected_tok)
                continue
            if tag == "BUILD_CTOR":
                _, ctor_sort, ctor_name, arity = op
                if arity:
                    children = tuple(results[-arity:])
                    del results[-arity:]
                else:
                    children = ()
                results.append(make_term(ctor_name, *children, registry=self.registry))
                continue
            if tag == "ADD_MAP_BINDING":
                _, rows, key_term = op
                val_term = results.pop()
                if val_term.sort != "NGValue":
                    raise ObservationFormatError("Map binding value sort must be NGValue")
                rows.append((key_term, val_term))
                continue
            if tag == "MERGE_MAP_GROUP":
                _, parent_rows = op
                group_term = results.pop()
                if group_term.sort != "Map" or group_term.name != "@Map":
                    raise ObservationFormatError("Expected nested Map group")
                if len(parent_rows) + len(group_term.args) > self.budget.max_rows:
                    raise ObservationFormatError("Map entries exceed DecodeBudget.max_rows")
                parent_rows.extend(group_term.args)
                continue
            if tag == "MAP_STEP":
                _, rows, state, depth, is_root_map = op
                if depth > self.budget.max_depth:
                    raise ObservationFormatError("Parse depth exceeds DecodeBudget.max_depth")
                while self.peek() == ".Map":
                    self.take(".Map")
                    state["saw_item"] = True
                key_term = self._try_take_map_key(depth + 1)
                if key_term is not None:
                    self.take("|->")
                    if len(rows) + 1 > self.budget.max_rows:
                        raise ObservationFormatError("Map entries exceed DecodeBudget.max_rows")
                    state["saw_item"] = True
                    stack.append(("MAP_STEP", rows, state, depth, is_root_map))
                    stack.append(("ADD_MAP_BINDING", rows, key_term))
                    stack.append(("PARSE", "NGValue", depth + 1))
                    continue
                if self.peek() == "(":
                    self.take("(")
                    state["saw_item"] = True
                    stack.append(("MAP_STEP", rows, state, depth, is_root_map))
                    stack.append(("MERGE_MAP_GROUP", rows))
                    stack.append(("EXPECT", ")"))
                    stack.append(("MAP_STEP", [], {"saw_item": False}, depth + 1, False))
                    continue
                if not state["saw_item"]:
                    raise ObservationFormatError("Expected complete finite environment Map")
                if is_root_map:
                    self._bump_node()
                seen_keys: set[int] = set()
                for k_t, _ in rows:
                    k_int = k_t.args[0]
                    if k_int in seen_keys:
                        raise ObservationFormatError(f"Duplicate Map key: {k_int}")
                    seen_keys.add(k_int)
                results.append(
                    T("Map", "@Map", tuple(sorted(rows, key=lambda row: row[0].args[0])))
                )
                continue

            # tag == "PARSE"
            _, target_sort, depth = op
            if depth > self.budget.max_depth:
                raise ObservationFormatError("Parse depth exceeds DecodeBudget.max_depth")
            if target_sort == "Map":
                stack.append(("MAP_STEP", [], {"saw_item": False}, depth, True))
                continue
            if self.peek() == "(":
                self.take("(")
                stack.append(("EXPECT", ")"))
                stack.append(("PARSE", target_sort, depth + 1))
                continue
            self._bump_node()
            token = self.take()
            if target_sort == "Int":
                results.append(atom("Int", self._parse_int_token(token)))
            elif target_sort == "Bool":
                if token not in ("true", "false"):
                    raise ObservationFormatError("Expected Bool token")
                results.append(atom("Bool", token == "true"))
            elif target_sort == "String":
                if not (len(token) >= 2 and token.startswith('"') and token.endswith('"')):
                    raise ObservationFormatError("Expected quoted String token")
                try:
                    parsed_str = json.loads(token)
                except Exception as exc:
                    raise ObservationFormatError(f"Malformed String token: {exc}") from exc
                if type(parsed_str) is not str:
                    raise ObservationFormatError("Expected String literal")
                results.append(atom("String", parsed_str))
            else:
                if token not in self.registry or self.registry[token][0] != target_sort:
                    raise ObservationFormatError(
                        "Unknown constructor/sort " + str((token, target_sort))
                    )
                child_sorts = self.registry[token][1]
                if len(child_sorts) > self.budget.max_rows:
                    raise ObservationFormatError("Constructor arity exceeds DecodeBudget.max_rows")
                self.take("(")
                stack.append(("BUILD_CTOR", target_sort, token, len(child_sorts)))
                stack.append(("EXPECT", ")"))
                for idx in range(len(child_sorts) - 1, -1, -1):
                    stack.append(("PARSE", child_sorts[idx], depth + 1))
                    if idx > 0:
                        stack.append(("EXPECT", ","))

        if len(results) != 1:
            raise ObservationFormatError("Internal parser stack imbalance")
        return results[0]

    def complete(self, sort: str = "NIWindowObservation") -> T:
        value = self.term(sort)
        if self.at != len(self.tokens):
            raise ObservationFormatError("Trailing printed tape tokens")
        return value


Source11PrintedParser = Source10PrintedParser


def parse_printed(
    text: str,
    sort: str = "NIWindowObservation",
    *,
    registry: Mapping[str, tuple[str, Sequence[str]]] = DATA_REG,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> T:
    return Source10PrintedParser(text, registry=registry, budget=budget).complete(sort)


def from_source10_ast(
    tree: Mapping[str, Any],
    *,
    allow_root_query: bool = False,
    allow_map: bool = False,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> T:
    """Convert literal-keyed AST into typed T term using explicit stack."""
    budget.validate()
    results: list[T] = []
    nodes = 0
    stack: list[tuple[Any, ...]] = [("NODE", tree, 0, True)]
    while stack:
        op = stack.pop()
        tag = op[0]
        if tag == "BUILD_CTOR":
            _, sort, name, arity, reg = op
            if arity:
                children = tuple(results[-arity:])
                del results[-arity:]
            else:
                children = ()
            results.append(make_term(name, *children, registry=reg))
            continue
        if tag == "BUILD_MAP":
            _, entry_count = op
            if entry_count:
                flat = results[-(2 * entry_count) :]
                del results[-(2 * entry_count) :]
                pairs = [
                    (flat[2 * i], flat[2 * i + 1]) for i in range(entry_count)
                ]
            else:
                pairs = []
            seen_keys: set[int] = set()
            for k_t, v_t in pairs:
                if k_t.sort != "Int" or k_t.name != "@Int" or v_t.sort != "NGValue":
                    raise ObservationFormatError("Invalid Map entry key/value sorts")
                k_int = _check_index64(k_t.args[0], "Map.name_id")
                if k_int in seen_keys:
                    raise ObservationFormatError(f"Duplicate Map key in AST: {k_int}")
                seen_keys.add(k_int)
            results.append(
                T("Map", "@Map", tuple(sorted(pairs, key=lambda row: row[0].args[0])))
            )
            continue

        _, node_obj, depth, is_root = op
        if depth > budget.max_depth:
            raise ObservationFormatError("AST depth exceeds DecodeBudget.max_depth")
        nodes += 1
        if nodes > budget.max_nodes:
            raise ObservationFormatError("AST node count exceeds DecodeBudget.max_nodes")
        if type(node_obj) is not dict or type(node_obj.get("sort")) is not str:
            raise ObservationFormatError("Typed AST node object with sort required")
        sort = node_obj["sort"]
        if sort in ("Int", "Bool", "String"):
            # Scalar AST uses 'literal', never older R2 'value'.
            _require_keys(node_obj, ("sort", "literal"), "primitive AST node")
            lit = node_obj["literal"]
            expected_type = {"Int": int, "Bool": bool, "String": str}[sort]
            if type(lit) is not expected_type:
                raise ObservationFormatError(f"Wrong scalar literal type for {sort}")
            if sort == "Int":
                _check_ni_int(lit)
            results.append(atom(sort, lit))
            continue
        if sort == "Map":
            if not allow_map:
                raise ObservationFormatError("Map node forbidden in this AST profile")
            _require_keys(node_obj, ("sort", "entries"), "Map AST node")
            entries = node_obj["entries"]
            if type(entries) is not list or len(entries) > budget.max_rows:
                raise ObservationFormatError("Invalid or oversized Map AST entries")
            stack.append(("BUILD_MAP", len(entries)))
            for entry in reversed(entries):
                _require_keys(entry, ("key", "value"), "Map entry AST")
                if (
                    type(entry["key"]) is not dict
                    or entry["key"].get("sort") != "Int"
                    or type(entry["value"]) is not dict
                    or entry["value"].get("sort") != "NGValue"
                ):
                    raise ObservationFormatError("Map entry requires Int key and NGValue value")
                stack.append(("NODE", entry["value"], depth + 1, False))
                stack.append(("NODE", entry["key"], depth + 1, False))
            continue

        _require_keys(node_obj, ("sort", "constructor", "arguments"), "constructor AST node")
        name = node_obj["constructor"]
        args = node_obj["arguments"]
        if type(name) is not str or type(args) is not list:
            raise ObservationFormatError("Constructor name must be str and arguments must be list")
        active_reg = QUERY_REG if (is_root and allow_root_query) else DATA_REG
        if name not in active_reg:
            raise ObservationFormatError(f"Disallowed or unknown constructor: {name!r}")
        actual_sort, child_sorts = active_reg[name]
        if sort != actual_sort or len(args) != len(child_sorts):
            raise ObservationFormatError(f"Typed AST sort or arity differs for {name}")
        if len(args) > budget.max_rows:
            raise ObservationFormatError("Constructor arguments exceed DecodeBudget.max_rows")
        stack.append(("BUILD_CTOR", sort, name, len(args), active_reg))
        for arg, child_sort in reversed(list(zip(args, child_sorts))):
            if type(arg) is not dict or arg.get("sort") != child_sort:
                raise ObservationFormatError(
                    f"Child AST sort differs for {name}: expected {child_sort}"
                )
            stack.append(("NODE", arg, depth + 1, False))

    if len(results) != 1:
        raise ObservationFormatError("Internal AST conversion stack imbalance")
    return results[0]


from_source11_ast = from_source10_ast


def to_source10_ast(
    term: Term,
    *,
    allow_map: bool = False,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> dict[str, Any]:
    """Convert typed T term into literal-keyed AST using explicit stack."""
    budget.validate()
    results: list[dict[str, Any]] = []
    nodes = 0
    stack: list[tuple[Any, ...]] = [("TERM", term, 0, True)]
    while stack:
        op = stack.pop()
        tag = op[0]
        if tag == "BUILD_CTOR":
            _, sort, name, arity = op
            if arity:
                children = list(results[-arity:])
                del results[-arity:]
            else:
                children = []
            results.append({"sort": sort, "constructor": name, "arguments": children})
            continue
        if tag == "BUILD_MAP":
            _, entry_count = op
            if entry_count:
                flat = results[-(2 * entry_count) :]
                del results[-(2 * entry_count) :]
                entries = [
                    {"key": flat[2 * i], "value": flat[2 * i + 1]}
                    for i in range(entry_count)
                ]
            else:
                entries = []
            results.append({"sort": "Map", "entries": entries})
            continue

        _, current, depth, is_root = op
        if depth > budget.max_depth:
            raise ObservationFormatError("Term depth exceeds DecodeBudget.max_depth")
        nodes += 1
        if nodes > budget.max_nodes:
            raise ObservationFormatError("Term node count exceeds DecodeBudget.max_nodes")
        if current.name in ("@Int", "@Bool", "@String"):
            expected_sort = current.name[1:]
            expected_type = {"Int": int, "Bool": bool, "String": str}[expected_sort]
            if (
                current.sort != expected_sort
                or len(current.args) != 1
                or type(current.args[0]) is not expected_type
            ):
                raise ObservationFormatError("Invalid scalar Term representation")
            if expected_sort == "Int":
                _check_ni_int(current.args[0])
            results.append({"sort": current.sort, "literal": current.args[0]})
            continue
        if current.name == "@Map":
            if not allow_map or current.sort != "Map":
                raise ObservationFormatError("Map term forbidden in this AST profile")
            if len(current.args) > budget.max_rows:
                raise ObservationFormatError("Map entries exceed DecodeBudget.max_rows")
            stack.append(("BUILD_MAP", len(current.args)))
            for pair in reversed(current.args):
                if (
                    type(pair) is not tuple
                    or len(pair) != 2
                    or pair[0].sort != "Int"
                    or pair[1].sort != "NGValue"
                ):
                    raise ObservationFormatError("Invalid Map binding pair")
                stack.append(("TERM", pair[1], depth + 1, False))
                stack.append(("TERM", pair[0], depth + 1, False))
            continue

        active_reg = QUERY_REG if is_root else DATA_REG
        if current.name not in active_reg:
            raise ObservationFormatError(f"Unhandled constructor in AST export: {current.name!r}")
        expected_sort, child_sorts = active_reg[current.name]
        if current.sort != expected_sort or len(current.args) != len(child_sorts):
            raise ObservationFormatError(f"Term sort/arity mismatch for {current.name}")
        stack.append(("BUILD_CTOR", current.sort, current.name, len(current.args)))
        for child, expected_child_sort in reversed(list(zip(current.args, child_sorts))):
            if child.sort != expected_child_sort:
                raise ObservationFormatError(f"Child Term sort mismatch for {current.name}")
            stack.append(("TERM", child, depth + 1, False))

    if len(results) != 1:
        raise ObservationFormatError("Internal Term-to-AST stack imbalance")
    return results[0]


to_source11_ast = to_source10_ast


def encode_node(
    term: Term,
    *,
    allow_root_query: bool = False,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> dict[str, Any]:
    """Encode a typed T term into a KAST JSONv4 node using actual compiled KLabels."""
    budget.validate()
    results: list[dict[str, Any]] = []
    nodes = 0
    stack: list[tuple[Any, ...]] = [("ENCODE", term, 0, True)]
    while stack:
        op = stack.pop()
        tag = op[0]
        if tag == "BUILD_KAPPLY":
            _, klabel_name, arity = op
            if arity:
                children = list(results[-arity:])
                del results[-arity:]
            else:
                children = []
            results.append(
                {
                    "node": "KApply",
                    "label": {"node": "KLabel", "name": klabel_name, "params": []},
                    "arity": arity,
                    "args": children,
                }
            )
            continue

        _, current, depth, is_root = op
        if depth > budget.max_depth:
            raise ObservationFormatError("KAST encode depth exceeds DecodeBudget.max_depth")
        nodes += 1
        if nodes > budget.max_nodes:
            raise ObservationFormatError("KAST encode node count exceeds DecodeBudget.max_nodes")
        if current.name in ("@Int", "@Bool", "@String"):
            expected = {"Int": int, "Bool": bool, "String": str}
            if (
                current.sort not in expected
                or current.name != "@" + current.sort
                or len(current.args) != 1
                or type(current.args[0]) is not expected[current.sort]
            ):
                raise ObservationFormatError("Scalar kind differs in KAST encode")
            if current.sort == "Int":
                _check_ni_int(current.args[0])
            results.append(
                {
                    "node": "KToken",
                    "sort": {"node": "KSort", "name": current.sort, "params": []},
                    "token": render_term(current, budget=budget),
                }
            )
            continue

        active_reg = QUERY_REG if (is_root and allow_root_query) else DATA_REG
        if current.name not in active_reg:
            raise ObservationFormatError(
                "Outside closed JSONv4 carrier profile (including Map/Ref/residual function)"
            )
        sort, child_sorts = active_reg[current.name]
        if current.sort != sort or tuple(x.sort for x in current.args) != child_sorts:
            raise ObservationFormatError("Typed constructor sort/children differ in KAST encode")
        if current.name not in LABELS:
            raise ObservationFormatError("Constructor lacks compiled label mapping")
        if len(current.args) > budget.max_rows:
            raise ObservationFormatError("KApply arity exceeds DecodeBudget.max_rows")
        stack.append(("BUILD_KAPPLY", LABELS[current.name], len(current.args)))
        for child in reversed(current.args):
            stack.append(("ENCODE", child, depth + 1, False))

    if len(results) != 1:
        raise ObservationFormatError("Internal KAST encode stack imbalance")
    return results[0]


def envelope(
    term: Term,
    *,
    allow_root_query: bool = False,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> dict[str, Any]:
    return {
        "format": "KAST",
        "version": 4,
        "term": encode_node(term, allow_root_query=allow_root_query, budget=budget),
    }


def encode_query_envelope(
    term: Term, *, budget: DecodeBudget = DEFAULT_BUDGET
) -> dict[str, Any]:
    if term.sort != "NIWindowObservation" or term.name != "niRunWindow":
        raise ObservationFormatError(
            "Query envelope root must be NIWindowObservation.niRunWindow"
        )
    return envelope(term, allow_root_query=True, budget=budget)


def decode_node(
    value: Mapping[str, Any],
    sort: str,
    *,
    allow_root_query: bool = False,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> T:
    """Decode a KAST JSONv4 node into a typed T term using actual compiled KLabels."""
    budget.validate()
    results: list[T] = []
    nodes = 0
    stack: list[tuple[Any, ...]] = [("DECODE", value, sort, 0, True)]
    while stack:
        op = stack.pop()
        tag = op[0]
        if tag == "BUILD_CTOR":
            _, surface_name, arity, active_reg = op
            if arity:
                children = tuple(results[-arity:])
                del results[-arity:]
            else:
                children = ()
            results.append(make_term(surface_name, *children, registry=active_reg))
            continue

        _, current, expected_sort, depth, is_root = op
        if depth > budget.max_depth:
            raise ObservationFormatError("KAST decode depth exceeds DecodeBudget.max_depth")
        nodes += 1
        if nodes > budget.max_nodes:
            raise ObservationFormatError("KAST decode node count exceeds DecodeBudget.max_nodes")
        if type(current) is not dict:
            raise ObservationFormatError("KAST node object required")
        if current.get("node") == "KToken":
            _require_keys(current, ("node", "sort", "token"), "KToken")
            s_obj = _require_keys(current["sort"], ("node", "name", "params"), "KSort")
            if (
                s_obj != {"node": "KSort", "name": expected_sort, "params": []}
                or expected_sort not in ("Int", "Bool", "String")
                or type(current["token"]) is not str
            ):
                raise ObservationFormatError("Exact primitive KSort/token required")
            parsed_term = parse_printed(
                current["token"], expected_sort, registry=DATA_REG, budget=budget
            )
            expected_py_type = {"Int": int, "Bool": bool, "String": str}[expected_sort]
            if (
                type(parsed_term.args[0]) is not expected_py_type
                or render_term(parsed_term, budget=budget) != current["token"]
            ):
                raise ObservationFormatError("Noncanonical scalar lexeme in KToken")
            results.append(parsed_term)
            continue

        _require_keys(current, ("node", "label", "arity", "args"), "KApply")
        if current["node"] != "KApply":
            raise ObservationFormatError("Outside KApply/KToken carrier profile")
        label = _require_keys(current["label"], ("node", "name", "params"), "KLabel")
        if (
            label["node"] != "KLabel"
            or label["params"] != []
            or type(label["name"]) is not str
            or label["name"] not in REVERSE_LABELS
        ):
            raise ObservationFormatError("Unknown or parametric KLabel")
        surface_name = REVERSE_LABELS[label["name"]]
        active_reg = QUERY_REG if (is_root and allow_root_query) else DATA_REG
        if surface_name not in active_reg:
            raise ObservationFormatError(
                f"Symbol {surface_name!r} is not permitted in this carrier position"
            )
        actual_sort, child_sorts = active_reg[surface_name]
        if (
            actual_sort != expected_sort
            or type(current["arity"]) is not int
            or current["arity"] != len(child_sorts)
            or type(current["args"]) is not list
            or len(current["args"]) != len(child_sorts)
        ):
            raise ObservationFormatError("Exact KApply sort/arity/argument count differs")
        if len(child_sorts) > budget.max_rows:
            raise ObservationFormatError("KApply arity exceeds DecodeBudget.max_rows")
        stack.append(("BUILD_CTOR", surface_name, len(child_sorts), active_reg))
        for child_node, child_sort in reversed(list(zip(current["args"], child_sorts))):
            stack.append(("DECODE", child_node, child_sort, depth + 1, False))

    if len(results) != 1:
        raise ObservationFormatError("Internal KAST decode stack imbalance")
    return results[0]


def decode_document(
    value: Mapping[str, Any],
    sort: str,
    *,
    allow_root_query: bool = False,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> T:
    _require_keys(value, ("format", "version", "term"), "KAST JSONv4 envelope")
    if (
        value["format"] != "KAST"
        or type(value["version"]) is not int
        or value["version"] != 4
    ):
        raise ObservationFormatError("KAST JSONv4 envelope required")
    return decode_node(
        value["term"], sort, allow_root_query=allow_root_query, budget=budget
    )


def decode_query_document(
    value: Mapping[str, Any],
    sort: str = "NIWindowObservation",
    *,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> T:
    term = decode_document(value, sort, allow_root_query=True, budget=budget)
    if term.sort != "NIWindowObservation" or term.name != "niRunWindow":
        raise ObservationFormatError(
            "Decoded query document root must be NIWindowObservation.niRunWindow"
        )
    return term


class Source10KWire(CompleteKWire):
    """Additive Source10/Source11 extension of CompleteKWire for NI, NP, Not, and NIWindowObservation."""

    SCHEMA = "sherlock-source11-nominal-state-event-window/1"
    SEQUENCES = {
        **CompleteKWire.SEQUENCES,
        "NPTypes": ("npNoTypes", "npType"),
        "NPFieldNames": ("npNoFieldNames", "npFieldName"),
        "NIJItems": ("niNoItems", "niItem"),
    }
    EXPRESSIONS = {
        **CompleteKWire.EXPRESSIONS,
        "ngNot": ("Not", ("operand",)),
    }
    RECORDS = {
        "NGCtor": ("ngCtor", ("type_id", "ctor_id", "source_order", "field_types")),
        "NGFunction": ("ngFunction", ("name_id", "parameters", "return_type_id", "body")),
        "NGProgram": ("ngProgram", ("declarations", "expressions", "functions", "entry")),
        "NGState": (
            "ngState",
            ("program", "control", "environment", "frames_top_first", "events"),
        ),
        "NGTape": ("ngTape", ("initial", "after_each_event", "final")),
        "NPCtorSchema": ("npCtorSchema", ("name", "fields")),
        "NPObservation": ("npObserved", ("state", "value")),
        "NIObservation": ("niObserved", ("source_document", "np_observation")),
        "NIWindowObservation": ("niWindowObserved", ("source_document", "tape")),
    }

    def _bounded_rows(
        self, term: Term, empty: str, row: str, budget: DecodeBudget, depth: int
    ) -> list[tuple[Term, ...]]:
        sort = term.sort
        rows: list[tuple[Term, ...]] = []
        current = term
        steps = 0
        while True:
            if depth + steps > budget.max_depth:
                raise ObservationFormatError("Sequence spine depth exceeds DecodeBudget.max_depth")
            args = self._args(current, sort)
            if current.name == empty:
                if args:
                    raise ObservationFormatError("Nonempty list terminator")
                return rows
            if current.name != row or not args:
                raise ObservationFormatError("Unexpected list constructor")
            rows.append(args[:-1])
            if len(rows) > budget.max_rows:
                raise ObservationFormatError("Sequence length exceeds DecodeBudget.max_rows")
            current = args[-1]
            steps += 1

    def decode(self, term: Term, *, budget: DecodeBudget = DEFAULT_BUDGET) -> Any:
        """Bounded explicit-stack decoder for NG, NP, Not, NIJSON, and NIWindowObservation terms."""
        budget.validate()
        results: list[Any] = []
        nodes = 0
        stack: list[tuple[Any, ...]] = [("TERM", term, 0)]
        while stack:
            op = stack.pop()
            tag = op[0]

            if tag == "BUILD_MAP":
                _, count = op
                if count:
                    vals = list(results[-count:])
                    del results[-count:]
                else:
                    vals = []
                seen_keys: set[int] = set()
                rows: list[dict[str, Any]] = []
                for idx in range(0, len(vals), 2):
                    name_id = _check_index64(vals[idx], "Map.name_id")
                    if name_id in seen_keys:
                        raise ObservationFormatError(f"Duplicate Map key: {name_id}")
                    seen_keys.add(name_id)
                    rows.append({"name_id": name_id, "value": vals[idx + 1]})
                results.append(sorted(rows, key=lambda binding: binding["name_id"]))
                continue

            if tag == "BUILD_SEQ":
                _, count, seq_sort = op
                if count:
                    items = list(results[-count:])
                    del results[-count:]
                else:
                    items = []
                if seq_sort in ("NGTypes", "NGExprIds", "NGPath"):
                    for item in items:
                        _check_index64(item, seq_sort)
                elif seq_sort == "NGCtors":
                    seen_ctors: set[tuple[int, int]] = set()
                    for item in items:
                        key_pair = (item["type_id"], item["ctor_id"])
                        if key_pair in seen_ctors:
                            raise ObservationFormatError(f"Duplicate NGCtor {key_pair}")
                        seen_ctors.add(key_pair)
                elif seq_sort == "NGFunctions":
                    seen_fns: set[int] = set()
                    for item in items:
                        fn_id = item["name_id"]
                        if fn_id in seen_fns:
                            raise ObservationFormatError(f"Duplicate NGFunction name_id: {fn_id}")
                        seen_fns.add(fn_id)
                results.append(items)
                continue

            if tag == "BUILD_ROW_RECORDS":
                _, row_count, fields, sort_name = op
                arity = len(fields)
                total = row_count * arity
                if total:
                    flat = list(results[-total:])
                    del results[-total:]
                else:
                    flat = []
                out_rows: list[dict[str, Any]] = []
                for r_idx in range(row_count):
                    chunk = flat[r_idx * arity : (r_idx + 1) * arity]
                    rec = self._record(fields, chunk)
                    out_rows.append(rec)
                if sort_name == "NPFields":
                    seen_f: set[str] = set()
                    for rec in out_rows:
                        if rec["name"] in seen_f:
                            raise ObservationFormatError(
                                f"Duplicate NPField name: {rec['name']!r}"
                            )
                        seen_f.add(rec["name"])
                elif sort_name == "NPTypeRows":
                    seen_t: set[int] = set()
                    for rec in out_rows:
                        tid = _check_index64(rec["runtime_type_id"], "NPTypeRow.runtime_type_id")
                        if tid in seen_t:
                            raise ObservationFormatError(f"Duplicate NPTypeRow id: {tid}")
                        seen_t.add(tid)
                elif sort_name == "NPCtorRows":
                    seen_c: set[tuple[int, int]] = set()
                    for rec in out_rows:
                        tid = _check_index64(rec["type_id"], "NPCtorRow.type_id")
                        cid = _check_index64(rec["ctor_id"], "NPCtorRow.ctor_id")
                        if (tid, cid) in seen_c:
                            raise ObservationFormatError(
                                f"Duplicate NPCtorRow {(tid, cid)}"
                            )
                        seen_c.add((tid, cid))
                        if len(set(rec["fields"])) != len(rec["fields"]):
                            raise ObservationFormatError(
                                f"Duplicate field name in NPCtorRow {rec['name']!r}"
                            )
                elif sort_name == "NGDecls":
                    seen_decls: set[int] = set()
                    for rec in out_rows:
                        tid = _check_index64(rec["type_id"], "NGDecl.type_id")
                        _check_index64(rec["recursive_group"], "NGDecl.recursive_group")
                        if tid in seen_decls:
                            raise ObservationFormatError(f"Duplicate NGDecl type_id: {tid}")
                        seen_decls.add(tid)
                elif sort_name == "NGParams":
                    seen_params: set[int] = set()
                    for rec in out_rows:
                        pid = _check_index64(rec["name_id"], "NGParam.name_id")
                        _check_index64(rec["type_id"], "NGParam.type_id")
                        if pid in seen_params:
                            raise ObservationFormatError(f"Duplicate NGParam name_id: {pid}")
                        seen_params.add(pid)
                elif sort_name == "NGArms":
                    seen_arms: set[int] = set()
                    for rec in out_rows:
                        aid = _check_index64(rec["source_arm_id"], "NGArm.source_arm_id")
                        _check_index64(rec["body"], "NGArm.body")
                        if aid in seen_arms:
                            raise ObservationFormatError(f"Duplicate NGArm source_arm_id: {aid}")
                        seen_arms.add(aid)
                elif sort_name == "NGBindings":
                    seen_binds: set[int] = set()
                    for rec in out_rows:
                        bid = _check_index64(rec["name_id"], "NGBinding.name_id")
                        _check_index64(rec["match_expr_id"], "NGBinding.match_expr_id")
                        _check_index64(rec["source_arm_id"], "NGBinding.source_arm_id")
                        if bid in seen_binds:
                            raise ObservationFormatError(f"Duplicate NGBinding name_id: {bid}")
                        seen_binds.add(bid)
                results.append(out_rows)
                continue

            if tag == "BUILD_NGNODES":
                _, row_count = op
                total = row_count * 2
                if total:
                    flat = list(results[-total:])
                    del results[-total:]
                else:
                    flat = []
                seen_ids: set[int] = set()
                node_rows: list[dict[str, Any]] = []
                for r_idx in range(row_count):
                    expr_id = _check_index64(flat[2 * r_idx], "NGNodes.id")
                    if expr_id in seen_ids:
                        raise ObservationFormatError(f"Duplicate NGNode id: {expr_id}")
                    seen_ids.add(expr_id)
                    expr_body = flat[2 * r_idx + 1]
                    node_rows.append({"id": expr_id, **expr_body})
                results.append(node_rows)
                continue

            if tag == "BUILD_GROUP":
                _, ctor_name, keys = op
                arity = len(keys)
                if arity:
                    vals = list(results[-arity:])
                    del results[-arity:]
                else:
                    vals = []
                for key_name, val in zip(keys, vals):
                    if key_name.endswith("_id") or key_name in (
                        "occurrence",
                        "call_occurrence",
                        "constructor_occurrence",
                        "match_occurrence",
                        "completed_expr_occurrence",
                        "bound_occurrence",
                        "condition_occurrence",
                        "argument_cursor",
                        "field_cursor",
                        "source_expr",
                        "binder",
                        "bound_expr",
                        "body_expr",
                        "condition_expr",
                        "yes_expr",
                        "no_expr",
                        "selected_expr",
                    ):
                        _check_index64(val, f"{ctor_name}.{key_name}")
                results.append({ctor_name: self._record(keys, vals)})
                continue

            if tag == "BUILD_NGVALUE":
                _, ctor_name, arity = op
                vals = list(results[-arity:]) if arity else []
                if arity:
                    del results[-arity:]
                if ctor_name == "ngUInt":
                    u_val = _check_uint128(self._single(vals), "ngUInt")
                    results.append({"UInt": u_val})
                elif ctor_name == "ngBool":
                    results.append({"Bool": self._single(vals)})
                else:
                    rec = self._record(("type_id", "ctor_id", "fields"), vals)
                    _check_index64(rec["type_id"], "ngNominal.type_id")
                    _check_index64(rec["ctor_id"], "ngNominal.ctor_id")
                    results.append({"Nominal": rec})
                continue

            if tag == "BUILD_NPVALUE":
                _, ctor_name, arity = op
                vals = list(results[-arity:]) if arity else []
                if arity:
                    del results[-arity:]
                if ctor_name == "npUInt":
                    u_val = _check_uint128(self._single(vals), "npUInt")
                    results.append({"UInt": u_val})
                elif ctor_name == "npBool":
                    results.append({"Bool": self._single(vals)})
                else:
                    rec = self._record(("type", "constructor", "fields"), vals)
                    results.append({"Nominal": rec})
                continue

            if tag == "BUILD_NPTYPE":
                _, ctor_name, arity = op
                vals = list(results[-arity:]) if arity else []
                if arity:
                    del results[-arity:]
                if ctor_name == "npUIntType":
                    results.append({"kind": "UInt"})
                elif ctor_name == "npBoolType":
                    results.append({"kind": "Bool"})
                else:
                    results.append(
                        {"kind": "Nominal", "name": vals[0], "arguments": vals[1]}
                    )
                continue

            if tag == "BUILD_NGPATTERN":
                _, ctor_name, arity = op
                vals = list(results[-arity:]) if arity else []
                if arity:
                    del results[-arity:]
                if ctor_name == "ngWildcard":
                    results.append({"Wildcard": {}})
                elif ctor_name == "ngBind":
                    b_id = _check_index64(self._single(vals), "ngBind")
                    results.append({"Bind": b_id})
                else:
                    rec = self._record(("type_id", "ctor_id", "fields"), vals)
                    _check_index64(rec["type_id"], "ngConstructorPattern.type_id")
                    _check_index64(rec["ctor_id"], "ngConstructorPattern.ctor_id")
                    results.append({"Constructor": rec})
                continue

            if tag == "BUILD_NGEXPR":
                _, expr_tag, fields = op
                arity = 1 + len(fields)
                vals = list(results[-arity:])
                del results[-arity:]
                occ = _check_index64(vals[0], f"NGExpr.{expr_tag}.occurrence")
                payload = self._record(fields, vals[1:])
                for f_key, f_val in payload.items():
                    if f_key in (
                        "name_id",
                        "left",
                        "right",
                        "type_id",
                        "ctor_id",
                        "scrutinee",
                        "binder",
                        "bound_expr",
                        "body_expr",
                        "condition_expr",
                        "yes_expr",
                        "no_expr",
                        "operand",
                    ):
                        _check_index64(f_val, f"NGExpr.{expr_tag}.{f_key}")
                results.append(
                    {
                        "occurrence": occ,
                        "kind": {
                            expr_tag: payload["value"] if expr_tag == "Literal" else payload
                        },
                    }
                )
                continue

            if tag == "BUILD_NIJSON":
                _, ctor_name = op
                if ctor_name == "niNull":
                    results.append(None)
                elif ctor_name in ("niBool", "niString"):
                    results.append(results.pop())
                elif ctor_name == "niInt":
                    results.append(_check_ni_int(results.pop()))
                elif ctor_name == "niArray":
                    results.append(results.pop())
                elif ctor_name == "niObject":
                    field_rows = results.pop()
                    obj: dict[str, Any] = {}
                    for row_item in field_rows:
                        k = row_item["key"]
                        if k in obj:
                            raise ObservationFormatError(f"Duplicate NIJSON object key: {k!r}")
                        obj[k] = row_item["value"]
                    results.append(obj)
                continue

            if tag == "BUILD_RECORD":
                _, sort_name, fields = op
                arity = len(fields)
                vals = list(results[-arity:]) if arity else []
                if arity:
                    del results[-arity:]
                rec = self._record(fields, vals)
                if sort_name == "NGCtor":
                    _check_index64(rec["type_id"], "NGCtor.type_id")
                    _check_index64(rec["ctor_id"], "NGCtor.ctor_id")
                    _check_index64(rec["source_order"], "NGCtor.source_order")
                elif sort_name == "NGFunction":
                    _check_index64(rec["name_id"], "NGFunction.name_id")
                    _check_index64(rec["return_type_id"], "NGFunction.return_type_id")
                    _check_index64(rec["body"], "NGFunction.body")
                elif sort_name == "NGProgram":
                    _check_index64(rec["entry"], "NGProgram.entry")
                elif sort_name == "NPCtorSchema":
                    if len(set(rec["fields"])) != len(rec["fields"]):
                        raise ObservationFormatError(
                            f"Duplicate field name in NPCtorSchema {rec['name']!r}"
                        )
                results.append(rec)
                continue

            # tag == "TERM"
            _, current, depth = op
            if depth > budget.max_depth:
                raise ObservationFormatError("Decode depth exceeds DecodeBudget.max_depth")
            nodes += 1
            if nodes > budget.max_nodes:
                raise ObservationFormatError("Decode node count exceeds DecodeBudget.max_nodes")

            sort, name, args = current.sort, current.name, current.args
            scalar_types = {"Int": int, "Bool": bool, "String": str}
            if sort in scalar_types:
                if (
                    name != "@" + sort
                    or len(args) != 1
                    or type(args[0]) is not scalar_types[sort]
                ):
                    raise ObservationFormatError("Unexpected scalar representation")
                if sort == "Int":
                    _check_ni_int(args[0])
                results.append(args[0])
                continue

            if sort == "Map":
                if name != "@Map":
                    raise ObservationFormatError("Expected complete parsed Map")
                if len(args) > budget.max_rows:
                    raise ObservationFormatError("Map size exceeds DecodeBudget.max_rows")
                stack.append(("BUILD_MAP", 2 * len(args)))
                for pair in reversed(args):
                    if (
                        type(pair) is not tuple
                        or len(pair) != 2
                        or pair[0].sort != "Int"
                        or pair[1].sort != "NGValue"
                    ):
                        raise ObservationFormatError("Map binding shape differs")
                    stack.append(("TERM", pair[1], depth + 1))
                    stack.append(("TERM", pair[0], depth + 1))
                continue

            if sort in self.SEQUENCES:
                empty, row_ctor = self.SEQUENCES[sort]
                seq_rows = self._bounded_rows(current, empty, row_ctor, budget, depth)
                nodes += len(seq_rows)
                if nodes > budget.max_nodes:
                    raise ObservationFormatError("Decode node count exceeds DecodeBudget.max_nodes")
                for payload in seq_rows:
                    if len(payload) != 1:
                        raise ObservationFormatError("Sequence element arity differs")
                stack.append(("BUILD_SEQ", len(seq_rows), sort))
                for payload in reversed(seq_rows):
                    stack.append(("TERM", payload[0], depth + 1))
                continue

            if sort in self.GROUPS:
                actual = self._args(current, sort)
                keys = self.fields[sort].get(name)
                if keys is None:
                    raise ObservationFormatError("Variant absent from public API: " + name)
                stack.append(("BUILD_GROUP", name, keys))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort == "NGValue" and name in ("ngUInt", "ngBool", "ngNominal"):
                actual = self._args(current, sort)
                stack.append(("BUILD_NGVALUE", name, len(actual)))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort == "NPValue" and name in ("npUInt", "npBool", "npNominal"):
                actual = self._args(current, sort)
                stack.append(("BUILD_NPVALUE", name, len(actual)))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort == "NPType" and name in ("npUIntType", "npBoolType", "npNominalType"):
                actual = self._args(current, sort)
                stack.append(("BUILD_NPTYPE", name, len(actual)))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort == "NGPattern" and name in (
                "ngWildcard",
                "ngBind",
                "ngConstructorPattern",
            ):
                actual = self._args(current, sort)
                stack.append(("BUILD_NGPATTERN", name, len(actual)))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort == "NGExpr":
                actual = self._args(current, sort)
                declaration = self.EXPRESSIONS.get(name)
                if declaration is None or not actual:
                    raise ObservationFormatError("Expression form is not declared")
                expr_tag, fields = declaration
                stack.append(("BUILD_NGEXPR", expr_tag, fields))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort == "NIJSON" and name in (
                "niNull",
                "niBool",
                "niInt",
                "niString",
                "niArray",
                "niObject",
            ):
                actual = self._args(current, sort)
                stack.append(("BUILD_NIJSON", name))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort == "NGNodes":
                seq_rows = self._bounded_rows(current, "ngNoNodes", "ngNode", budget, depth)
                nodes += len(seq_rows)
                if nodes > budget.max_nodes:
                    raise ObservationFormatError("Decode node count exceeds DecodeBudget.max_nodes")
                for payload in seq_rows:
                    if len(payload) != 2:
                        raise ObservationFormatError("Expression row arity differs")
                stack.append(("BUILD_NGNODES", len(seq_rows)))
                for payload in reversed(seq_rows):
                    stack.append(("TERM", payload[1], depth + 1))
                    stack.append(("TERM", payload[0], depth + 1))
                continue

            row_specs = {
                "NGDecls": ("ngNoDecls", "ngDecl", ("type_id", "recursive_group", "ctors")),
                "NGParams": ("ngNoParams", "ngParam", ("name_id", "type_id")),
                "NGArms": ("ngNoArms", "ngArm", ("source_arm_id", "pattern", "body")),
                "NGBindings": (
                    "ngNoBindings",
                    "ngBinding",
                    ("name_id", "match_expr_id", "source_arm_id", "field_path", "value"),
                ),
                "NPTypeRows": ("npNoTypeRows", "npTypeRow", ("runtime_type_id", "type")),
                "NPCtorRows": (
                    "npNoCtorRows",
                    "npCtorRow",
                    ("type_id", "ctor_id", "name", "fields"),
                ),
                "NPFields": ("npNoFields", "npField", ("name", "value")),
                "NIJFields": ("niNoFields", "niField", ("key", "value")),
            }
            if sort in row_specs:
                empty, row_ctor, fields = row_specs[sort]
                seq_rows = self._bounded_rows(current, empty, row_ctor, budget, depth)
                nodes += len(seq_rows)
                if nodes > budget.max_nodes:
                    raise ObservationFormatError("Decode node count exceeds DecodeBudget.max_nodes")
                for payload in seq_rows:
                    if len(payload) != len(fields):
                        raise ObservationFormatError(f"{sort} row arity differs")
                stack.append(("BUILD_ROW_RECORDS", len(seq_rows), fields, sort))
                for payload in reversed(seq_rows):
                    for child in reversed(payload):
                        stack.append(("TERM", child, depth + 1))
                continue

            lookup_specs = {
                "NGMatching": {"ngMatched": ("bindings",), "ngMiss": ()},
                "NGEnvironment": {"ngBoundEnvironment": ("environment",), "ngBadBindings": ()},
                "NGExprLookup": {"ngFoundExpr": ("expr",), "ngMissingExpr": ()},
                "NGFunctionLookup": {"ngFoundFunction": ("function",), "ngMissingFunction": ()},
                "NGCtorLookup": {"ngFoundCtor": ("ctor",), "ngMissingCtor": ()},
            }
            if sort in lookup_specs and name in lookup_specs[sort]:
                actual = self._args(current, sort)
                keys = lookup_specs[sort][name]
                stack.append(("BUILD_GROUP", name, keys))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            if sort in self.RECORDS:
                constructor, fields = self.RECORDS[sort]
                actual = self._args(current, sort, constructor)
                stack.append(("BUILD_RECORD", sort, fields))
                for child in reversed(actual):
                    stack.append(("TERM", child, depth + 1))
                continue

            raise ObservationFormatError("Undecoded declared datum: " + str((sort, name)))

        if len(results) != 1:
            raise ObservationFormatError("Internal Source10KWire stack imbalance")
        return results[0]


Source11KWire = Source10KWire
WIRE = Source10KWire(PUBLIC_API, DATA_REG)


def _strip_runtime_type_ids(np_type_meta: Mapping[str, Any]) -> dict[str, Any]:
    """Project port_positive's type metadata to K's NPType shape for inverse check."""
    if np_type_meta["kind"] in ("UInt", "Bool"):
        return {"kind": np_type_meta["kind"]}
    return {
        "kind": "Nominal",
        "name": np_type_meta["name"],
        "arguments": [
            _strip_runtime_type_ids(arg["type"]) for arg in np_type_meta["arguments"]
        ],
    }


def _port_checked_source_document(
    source_document: Mapping[str, Any], *, budget: DecodeBudget = DEFAULT_BUDGET
) -> dict[str, Any]:
    budget.validate()
    if type(source_document) is not dict:
        raise ObservationFormatError("Bound source_document must be a dict")
    _validate_json_value_budget(source_document, budget)
    if (
        "program_view" not in source_document
        or type(source_document["program_view"]) is not dict
        or set(source_document["program_view"]) != set(PROGRAM_FIELDS)
    ):
        raise ObservationFormatError(
            "Bound source_document must contain all 23 ProgramView fields"
        )
    try:
        ported = _port10.port_positive(source_document)
    except (PortError, KeyError, TypeError, ValueError) as exc:
        raise ObservationFormatError(f"ProgramView positive port failed: {exc}") from exc
    if ported.get("unhandled") or ported.get("ng_program") is None:
        raise ObservationFormatError(
            f"ProgramView contains unhandled constructs: {ported.get('unhandled')!r}"
        )
    if not same(ported["source_document"], source_document) or not same(
        _port10.source_roundtrip(ported), source_document
    ):
        raise ObservationFormatError("Ported source_document roundtrip mismatch")
    return ported


def window_query_ast(
    ported: Mapping[str, Any], observation_cut: int
) -> dict[str, Any]:
    """Build closed constructor-name AST for NIWindowObservation.niRunWindow(3).

    Constructs only the new Source11 root from existing positive_identity_port
    DATA child APIs (json_ast, program_ast, literal). Evaluates 0 model steps.
    """
    _check_index64(observation_cut, "observation_cut")
    return _port10.node(
        "NIWindowObservation",
        "niRunWindow",
        _port10.json_ast(ported["source_document"]),
        _port10.program_ast(ported["ng_program"]),
        _port10.literal("Int", observation_cut),
    )


def render_source11(
    source_document: Mapping[str, Any],
    observation_cut: int,
    *,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> dict[str, Any]:
    """Render bound input-only Source11 document + observation cut into KAST JSONv4.

    Calls existing port_positive and builds NIWindowObservation.niRunWindow(3)
    from existing DATA child APIs, verifies complete typed query AST, JSONv4
    carrier, and Source10KWire inverses before returning. Evaluates 0 model steps
    and reads 0 expected files.
    """
    budget.validate()
    _check_index64(observation_cut, "observation_cut")
    saved_input = copy.deepcopy(source_document)
    ported = _port_checked_source_document(source_document, budget=budget)
    if not same(source_document, saved_input):
        raise ObservationFormatError("port_positive mutated caller source_document")

    ast = window_query_ast(ported, observation_cut)
    if (
        type(ast) is not dict
        or ast.get("sort") != "NIWindowObservation"
        or ast.get("constructor") != "niRunWindow"
        or type(ast.get("arguments")) is not list
        or len(ast["arguments"]) != 3
    ):
        raise ObservationFormatError(
            "window_query_ast did not produce NIWindowObservation.niRunWindow(3)"
        )

    if not same(_port10.json_from_ast(ast["arguments"][0]), source_document):
        raise ObservationFormatError(
            "window_query_ast NIJSON inverse differs from source_document"
        )

    query_term = from_source10_ast(
        ast, allow_root_query=True, allow_map=False, budget=budget
    )
    if not same(to_source10_ast(query_term, allow_map=False, budget=budget), ast):
        raise ObservationFormatError("Term-to-AST roundtrip differs on niRunWindow query")

    # Verify all three niRunWindow DATA children through the strict receiving decoder.
    if (
        not same(WIRE.decode(query_term.args[0], budget=budget), source_document)
        or not same(WIRE.decode(query_term.args[1], budget=budget), ported["ng_program"])
        or not same(WIRE.decode(query_term.args[2], budget=budget), observation_cut)
    ):
        raise ObservationFormatError(
            "Source10KWire child inverse differs on niRunWindow query"
        )

    kast_jsonv4 = encode_query_envelope(query_term, budget=budget)
    decoded_query = decode_query_document(
        kast_jsonv4, "NIWindowObservation", budget=budget
    )
    if decoded_query != query_term or not same(
        to_source10_ast(decoded_query, allow_map=False, budget=budget), ast
    ):
        raise ObservationFormatError("KAST JSONv4 query roundtrip inverse differs")

    printed_query = render_term(query_term, budget=budget)
    parsed_printed_query = parse_printed(
        printed_query, "NIWindowObservation", registry=QUERY_REG, budget=budget
    )
    if parsed_printed_query != query_term:
        raise ObservationFormatError("Printed query lexical roundtrip differs")

    return {
        "schema": RENDER_SCHEMA,
        "observation_cut": observation_cut,
        "ported": ported,
        "query_ast": ast,
        "kast_jsonv4": kast_jsonv4,
        "printed_query": printed_query,
        "inverse_verified": True,
        "model_steps_evaluated": 0,
        "expected_documents_opened": 0,
    }


def decode_source11(
    source_document: Mapping[str, Any],
    raw_kprint_text: str,
    *,
    budget: DecodeBudget = DEFAULT_BUDGET,
) -> dict[str, Any]:
    """Decode whole raw Kprint NIWindowObservation output against bound Source11 input.

    Consumes the entire printed niWindowObserved(NIJSON, NGTape) tree with
    DATA-only constructors, exports complete raw typed AST alongside decoded
    source_document and complete NGTape (initial, after_each_event, final),
    and verifies decoded source_document equals the bound input document.
    """
    budget.validate()
    ported = _port_checked_source_document(source_document, budget=budget)
    inner_text = complete_term(raw_kprint_text, budget=budget)
    parsed_term = parse_printed(
        inner_text, "NIWindowObservation", registry=DATA_REG, budget=budget
    )
    if (
        parsed_term.sort != "NIWindowObservation"
        or parsed_term.name != "niWindowObserved"
        or len(parsed_term.args) != 2
    ):
        raise ObservationFormatError(
            "Expected complete DATA NIWindowObservation.niWindowObserved"
        )
    tape_term = parsed_term.args[1]
    if (
        tape_term.sort != "NGTape"
        or tape_term.name != "ngTape"
        or len(tape_term.args) != 3
    ):
        raise ObservationFormatError("Expected complete DATA NGTape.ngTape child")

    raw_typed_ast = to_source10_ast(parsed_term, allow_map=True, budget=budget)
    reconstructed = from_source10_ast(
        raw_typed_ast, allow_root_query=False, allow_map=True, budget=budget
    )
    if reconstructed != parsed_term:
        raise ObservationFormatError(
            "Raw typed NIWindowObservation AST inverse differs"
        )

    decoded = WIRE.decode(parsed_term, budget=budget)
    decoded_source = decoded["source_document"]
    tape = _require_keys(
        decoded["tape"], ("initial", "after_each_event", "final"), "NGTape"
    )
    if type(tape["after_each_event"]) is not list:
        raise ObservationFormatError("NGTape.after_each_event must be a list")

    if not same(decoded_source, ported["source_document"]):
        raise ObservationFormatError(
            "Decoded NIJSON source_document differs from bound input document"
        )

    return {
        "schema": DECODE_SCHEMA,
        "raw_kprint": raw_kprint_text,
        "framed_inner_term": inner_text,
        "raw_typed_ast": raw_typed_ast,
        "source_document": decoded_source,
        "source_identity_verified": True,
        "tape": tape,
    }


def main(argv: Sequence[str] | None = None) -> int:
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest="action", required=True)

    r_cmd = sub.add_parser("render")
    r_cmd.add_argument("--input", type=Path, required=True)
    r_cmd.add_argument("--cut", type=int, required=True)
    r_cmd.add_argument("--kast-output", type=Path, required=True)
    r_cmd.add_argument("--ast-output", type=Path)
    r_cmd.add_argument("--printed-output", type=Path)

    d_cmd = sub.add_parser("decode")
    d_cmd.add_argument("--input", type=Path, required=True)
    d_cmd.add_argument("--printed", type=Path, required=True)
    d_cmd.add_argument("--output", type=Path, required=True)
    d_cmd.add_argument("--raw-ast-output", type=Path)

    args = cli.parse_args(argv)
    if args.action == "render":
        source_doc = load(args.input)
        rendered = render_source11(source_doc, args.cut)
        kast_bytes = (
            json.dumps(rendered["kast_jsonv4"], separators=(",", ":"), allow_nan=False)
            + "\n"
        ).encode("utf-8")
        products = {"kast_jsonv4": write(args.kast_output, kast_bytes)}
        if args.ast_output is not None:
            ast_bytes = (
                json.dumps(rendered["query_ast"], indent=2, allow_nan=False) + "\n"
            ).encode("utf-8")
            products["query_ast"] = write(args.ast_output, ast_bytes)
        if args.printed_output is not None:
            products["printed_query"] = write(
                args.printed_output, (rendered["printed_query"] + "\n").encode("utf-8")
            )
        print(
            json.dumps(
                {
                    "status": "DATA_PERSISTED",
                    "action": "render",
                    "products": products,
                    "model_steps_evaluated": 0,
                    "expected_documents_opened": 0,
                }
            )
        )
        return 0

    source_doc = load(args.input)
    raw_printed = args.printed.read_bytes().decode("utf-8")
    decoded = decode_source11(source_doc, raw_printed)
    out_bytes = (json.dumps(decoded, indent=2, allow_nan=False) + "\n").encode("utf-8")
    products = {"decoded_observation": write(args.output, out_bytes)}
    if args.raw_ast_output is not None:
        ast_bytes = (
            json.dumps(decoded["raw_typed_ast"], indent=2, allow_nan=False) + "\n"
        ).encode("utf-8")
        products["raw_typed_ast"] = write(args.raw_ast_output, ast_bytes)
    print(
        json.dumps(
            {
                "status": "DATA_PERSISTED",
                "action": "decode",
                "products": products,
                "expected_documents_opened": 0,
            }
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
