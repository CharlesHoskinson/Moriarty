#!/usr/bin/env python3
"""Data-only renderer/decoder for the isolated complete logical-image K model.

No subject launcher, evaluator, expected-output reader, hash derivation or SDK.
The context is mandatory caller data, not authenticated by this adapter/model.
"""
from __future__ import annotations

import argparse
from dataclasses import dataclass
import json
from pathlib import Path
import re
from typing import Any

INPUT_FORMAT = "moriarty-k-literal-complete-images-input/1"
OUTPUT_FORMAT = "moriarty-k-literal-complete-images-observation/1"
PROGRAM_WIDTHS = (2, 1, 4, 1, 11, 1, 6, 2, 3, 2, 3, 4, 1, 22, 2)
STATE_WIDTHS = (22, 2, 5, 2, 17, 36)
UINT128 = 1 << 128


class FormatError(ValueError):
    """A carrier/framing error; never an interpreter/admission result."""


def natural(value: Any, label: str, bound: int | None = None) -> int:
    if type(value) is not int or value < 0 or (bound is not None and value >= bound):
        raise FormatError(f"{label}: expected nonnegative integer" +
                          (f" below {bound}" if bound is not None else ""))
    return value


def keys(obj: Any, names: set[str], label: str) -> dict:
    if type(obj) is not dict or set(obj) != names:
        raise FormatError(f"{label}: exact keys required: {sorted(names)}")
    return obj


def vector(value: Any, count: int, label: str, bound: int | None) -> list[int]:
    if type(value) is not list or len(value) != count:
        raise FormatError(f"{label}: expected {count} integer lanes")
    return [natural(x, f"{label}[{i}]", bound) for i, x in enumerate(value)]


def image(value: Any, root_width: int, capacities: list[int],
          widths: tuple[int, ...], label: str) -> dict:
    obj = keys(value, {"root", "tables"}, label)
    root = vector(obj["root"], root_width, label + ".root", UINT128)
    tables = obj["tables"]
    if type(tables) is not list or len(tables) != len(widths):
        raise FormatError(f"{label}: expected {len(widths)} tables, including empty ones")
    result = []
    for i, (rows, capacity, width) in enumerate(zip(tables, capacities, widths)):
        if type(rows) is not list or len(rows) != capacity:
            raise FormatError(f"{label}.tables[{i}]: allocated row count differs from capacity")
        result.append([vector(row, width, f"{label}.tables[{i}][{j}]", UINT128)
                       for j, row in enumerate(rows)])
    return {"root": root, "tables": result}


def request(value: Any) -> dict:
    obj = keys(value, {"format", "context", "capacities", "program", "init",
                       "observation_budget"}, "request")
    if obj["format"] != INPUT_FORMAT:
        raise FormatError("wrong input format")
    context = vector(obj["context"], 2, "context", UINT128)
    caps = keys(obj["capacities"], {"program", "state", "stack_limit"}, "capacities")
    pc = vector(caps["program"], 15, "capacities.program", None)
    sc = vector(caps["state"], 6, "capacities.state", None)
    sl = natural(caps["stack_limit"], "capacities.stack_limit", UINT128)
    # This is data shape, not a replacement for the checked producer/admission.
    program = image(obj["program"], 16, pc, PROGRAM_WIDTHS, "program")
    init = image(obj["init"], 45, sc, STATE_WIDTHS, "init")
    budget = natural(obj["observation_budget"], "observation_budget")
    return {"format": INPUT_FORMAT, "context": context,
            "capacities": {"program": pc, "state": sc, "stack_limit": sl},
            "program": program, "init": init, "observation_budget": budget}


def chain(items: list[str], constructor: str, empty: str) -> str:
    # Iterative construction avoids depending on Python recursion limits.
    return "".join(constructor + "(" + item + "," for item in items) + empty + ")" * len(items)


def row_term(row: list[int]) -> str:
    return chain([str(x) for x in row], "lcCell", "lcNoCells()")


def image_term(obj: dict) -> str:
    tables = [chain([row_term(r) for r in table], "lcRow", "lcNoRows()")
              for table in obj["tables"]]
    return "lcImage(" + row_term(obj["root"]) + "," + chain(tables, "lcTable", "lcNoTables()") + ")"


def render(value: Any) -> str:
    obj = request(value)
    caps = obj["capacities"]
    a, b = obj["context"]
    return (f"lcRun(lcContext({a},{b}),lcCaps(" + row_term(caps["program"]) + "," +
            row_term(caps["state"]) + f",{caps['stack_limit']})," +
            image_term(obj["program"]) + "," + image_term(obj["init"]) +
            f",{obj['observation_budget']})\n")


@dataclass(frozen=True)
class Term:
    name: str
    args: tuple[Any, ...]


ARITIES = {"lcNoCells": 0, "lcCell": 2, "lcNoRows": 0, "lcRow": 2,
           "lcNoTables": 0, "lcTable": 2, "lcImage": 2, "lcContext": 2,
           "lcCaps": 3, "lcMachine": 4, "lcNoStates": 0, "lcState": 2,
           "lcNoEvents": 0, "lcEvent": 6, "lcTrace": 4, "lcResult": 2}
TOKEN = re.compile(r"\s*(-?[0-9]+|[A-Za-z][A-Za-z0-9_]*|[(),])")


def parse(text: str) -> Term:
    """Parse only the complete emitted constructor tree, without evaluating it."""
    stripped = text.strip()
    # kprint's existing configuration wrapper is framing, not another state source.
    if stripped.startswith("<k>") and stripped.endswith("</k>"):
        stripped = stripped[3:-4].strip()
    tokens = []
    pos = 0
    while pos < len(stripped):
        match = TOKEN.match(stripped, pos)
        if match is None:
            raise FormatError(f"unexpected K-print character at offset {pos}")
        tokens.append(match.group(1))
        pos = match.end()
    stack: list[list[Any]] = []
    result = None
    index = 0
    need_value = True
    while index < len(tokens):
        token = tokens[index]
        if need_value:
            if re.fullmatch(r"-?[0-9]+", token):
                value: Any = int(token)
                index += 1
            elif token in ARITIES:
                if index + 1 >= len(tokens) or tokens[index + 1] != "(":
                    raise FormatError("constructor must have parentheses")
                stack.append([token, []])
                index += 2
                if index < len(tokens) and tokens[index] == ")":
                    name, args = stack.pop()
                    if ARITIES[name] != 0:
                        raise FormatError("empty non-nullary constructor")
                    value = Term(name, ())
                    index += 1
                else:
                    continue
            else:
                raise FormatError("unevaluated/unknown constructor or malformed value: " + token)
            if stack:
                stack[-1][1].append(value)
            elif result is None:
                result = value
            else:
                raise FormatError("multiple top-level terms")
            need_value = False
        else:
            if not stack:
                raise FormatError("trailing tokens after complete term")
            if token == ",":
                need_value = True
                index += 1
            elif token == ")":
                name, args = stack.pop()
                if len(args) != ARITIES[name]:
                    raise FormatError("wrong constructor arity: " + name)
                value = Term(name, tuple(args))
                if stack:
                    stack[-1][1].append(value)
                elif result is None:
                    result = value
                else:
                    raise FormatError("multiple top-level terms")
                index += 1
            else:
                raise FormatError("expected comma or closing parenthesis")
    if stack or need_value or not isinstance(result, Term):
        raise FormatError("incomplete constructor tree")
    return result


def fields(term: Any, name: str) -> tuple:
    if not isinstance(term, Term) or term.name != name:
        raise FormatError("expected " + name)
    return term.args


def unchain(term: Any, cons: str, empty: str) -> list:
    result = []
    while isinstance(term, Term) and term.name == cons:
        head, term = term.args
        result.append(head)
    fields(term, empty)
    return result


def row_data(term: Any) -> list[int]:
    return [natural(v, "actual lane", UINT128)
            for v in unchain(term, "lcCell", "lcNoCells")]


def image_data(term: Any) -> dict:
    root, tables = fields(term, "lcImage")
    return {"root": row_data(root), "tables": [
        [row_data(r) for r in unchain(t, "lcRow", "lcNoRows")]
        for t in unchain(tables, "lcTable", "lcNoTables")]}


def machine_data(term: Any) -> dict:
    context, caps, program, state = fields(term, "lcMachine")
    a, b = fields(context, "lcContext")
    pc, sc, sl = fields(caps, "lcCaps")
    pc_data, sc_data = row_data(pc), row_data(sc)
    context_data = [natural(a, "actual context[0]", UINT128), natural(b, "actual context[1]", UINT128)]
    capacities = {"program": pc_data, "state": sc_data,
                  "stack_limit": natural(sl, "actual stack_limit", UINT128)}
    if len(pc_data) != 15 or len(sc_data) != 6:
        raise FormatError("actual capacity vector shape")
    return {"context": context_data, "capacities": capacities,
            "program": image(image_data(program), 16, pc_data, PROGRAM_WIDTHS, "actual program"),
            "state": image(image_data(state), 45, sc_data, STATE_WIDTHS, "actual state")}


def logical_layout(machine: dict) -> dict:
    """Derived coordinate metadata, never a source of actual lane values."""
    pc = machine["capacities"]["program"]
    sc = machine["capacities"]["state"]
    program_size = 16 + sum(n * w for n, w in zip(pc, PROGRAM_WIDTHS))
    state_size = 45 + sum(n * w for n, w in zip(sc, STATE_WIDTHS))
    cursor = 2
    segments = {"context": {"start": 0, "length": 2}}
    for name, size in (("program", program_size), ("init", state_size),
                       ("intermediate", state_size), ("done", state_size),
                       ("literal_envelope", 2), ("return_envelope", 2)):
        segments[name] = {"start": cursor, "length": size}
        cursor += size
    return {"provenance": "derived geometry from actual capacities and pinned L4 widths",
            "segments": segments, "statement_lanes": cursor}


def decode(text: str) -> dict:
    trace, statement = fields(parse(text), "lcResult")
    start, afters, finish, events = fields(trace, "lcTrace")
    actual_events = []
    while isinstance(events, Term) and events.name == "lcEvent":
        line, tag, charge, before, after, events = events.args
        actual_events.append({"source_line": natural(line, "actual source_line"),
                              "rule_tag": natural(tag, "actual rule_tag"),
                              "charge": natural(charge, "actual charge"),
                              "before": machine_data(before), "after": machine_data(after)})
    fields(events, "lcNoEvents")
    initial = machine_data(start)
    lanes = row_data(statement)
    layout = logical_layout(initial)
    if len(lanes) != layout["statement_lanes"]:
        raise FormatError("actual K-emitted statement length differs from carrier geometry")
    # Values and the statement are read from K. No state or envelope is predicted.
    return {"format": OUTPUT_FORMAT, "initial": initial,
            "afters": [machine_data(m) for m in unchain(afters, "lcState", "lcNoStates")],
            "final": machine_data(finish), "events": actual_events,
            "statement": lanes, "logical_layout": layout,
            "context_authentication": "NOT_ESTABLISHED",
            "source": "complete K-print constructor tree; no expected-output fill"}


def unique_object(pairs: list[tuple[str, Any]]) -> dict:
    result = {}
    for name, value in pairs:
        if name in result:
            raise FormatError("duplicate JSON key: " + name)
        result[name] = value
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("render", "decode"))
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    raw = args.input.read_text(encoding="utf-8")
    if args.action == "render":
        result = render(json.loads(raw, object_pairs_hook=unique_object))
    else:
        result = json.dumps(decode(raw), sort_keys=True, separators=(",", ":")) + "\n"
    # Retain any old capture instead of overwriting it. Caller supplies fresh path.
    with args.output.open("x", encoding="utf-8", newline="\n") as output:
        output.write(result)


if __name__ == "__main__":
    main()
