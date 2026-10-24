"""Serialize a complete declared NGProgram input; never evaluate a K step.

This renderer covers the declared common positive wire, including Let and If.
It does not call the predecessor codec's obsolete encode/Map render functions.
An independently implemented, already used parser/complete decoder checks the
entire serialized input before the files are created.
"""
import argparse
import hashlib
import json
from pathlib import Path

import nominal_trace_codec as parser
from complete_k_wire import CompleteKWire


def same(a, b):
    if type(a) is not type(b):
        return False
    if type(a) is dict:
        return a.keys() == b.keys() and all(same(a[k], b[k]) for k in a)
    if type(a) is list:
        return len(a) == len(b) and all(same(x, y) for x, y in zip(a, b))
    return a == b


def fields(value, keys):
    if type(value) is not dict or set(value) != set(keys):
        raise ValueError("Unexpected complete input fields: " + str(keys))
    return value


def nat(value):
    if type(value) is not int or value < 0:
        raise ValueError("Expected a nonnegative integer, not Bool")
    return str(value)


def call(name, *args):
    return name + "(" + ",".join(args) + ")"


def chain(rows, empty, name, encode):
    tail = call(empty)
    for row in reversed(rows):
        tail = call(name, *encode(row), tail)
    return tail


def tagged(value):
    if type(value) is not dict or len(value) != 1:
        raise ValueError("Expected exactly one declared variant")
    return next(iter(value.items()))


def value(v):
    tag, payload = tagged(v)
    if tag == "UInt":
        return call("ngUInt", nat(payload))
    if tag == "Bool":
        if type(payload) is not bool:
            raise ValueError("Expected Bool")
        return call("ngBool", str(payload).lower())
    if tag == "Nominal":
        fields(payload, ("type_id", "ctor_id", "fields"))
        return call("ngNominal", nat(payload["type_id"]), nat(payload["ctor_id"]), values(payload["fields"]))
    raise ValueError("Unsupported value variant: " + str(tag))


def values(rows):
    return chain(rows, "ngNoValues", "ngValue", lambda row: [value(row)])


def ints(rows, empty, name):
    return chain(rows, empty, name, lambda row: [nat(row)])


def pattern(p):
    tag, payload = tagged(p)
    if tag == "Wildcard":
        fields(payload, ())
        return call("ngWildcard")
    if tag == "Bind":
        return call("ngBind", nat(payload))
    if tag == "Constructor":
        fields(payload, ("type_id", "ctor_id", "fields"))
        children = chain(payload["fields"], "ngNoPatterns", "ngPattern", lambda row: [pattern(row)])
        return call("ngConstructorPattern", nat(payload["type_id"]), nat(payload["ctor_id"]), children)
    raise ValueError("Unsupported pattern variant: " + str(tag))


def arms(rows):
    def row(a):
        fields(a, ("source_arm_id", "pattern", "body"))
        return [nat(a["source_arm_id"]), pattern(a["pattern"]), nat(a["body"])]
    return chain(rows, "ngNoArms", "ngArm", row)


def expression(node):
    fields(node, ("id", "occurrence", "kind"))
    occ = nat(node["occurrence"])
    tag, v = tagged(node["kind"])
    if tag == "Literal":
        return call("ngLiteral", occ, value(v))
    spec = {
        "Variable": ("ngVariable", ("name_id",)),
        "Add": ("ngAdd", ("left", "right")),
        "Construct": ("ngConstruct", ("type_id", "ctor_id", "fields")),
        "Call": ("ngCall", ("name_id", "arguments")),
        "Match": ("ngMatch", ("scrutinee", "arms")),
        "Let": ("ngLet", ("binder", "bound_expr", "body_expr")),
        "If": ("ngIf", ("condition_expr", "yes_expr", "no_expr")),
    }
    if tag not in spec:
        raise ValueError("Unsupported expression variant: " + str(tag))
    name, keys = spec[tag]
    fields(v, keys)
    args = [occ]
    for key in keys:
        if key in ("fields", "arguments"):
            args.append(ints(v[key], "ngNoExprs", "ngExprId"))
        elif key == "arms":
            args.append(arms(v[key]))
        else:
            args.append(nat(v[key]))
    return call(name, *args)


def program(p):
    fields(p, ("declarations", "expressions", "functions", "entry"))
    def ctor(c):
        fields(c, ("type_id", "ctor_id", "source_order", "field_types"))
        return call("ngCtor", nat(c["type_id"]), nat(c["ctor_id"]), nat(c["source_order"]), ints(c["field_types"], "ngNoTypes", "ngType"))
    def decl(d):
        fields(d, ("type_id", "recursive_group", "ctors"))
        cs = chain(d["ctors"], "ngNoCtors", "ngCtorRow", lambda c: [ctor(c)])
        return [nat(d["type_id"]), nat(d["recursive_group"]), cs]
    def parameter(row):
        fields(row, ("name_id", "type_id"))
        return [nat(row["name_id"]), nat(row["type_id"])]
    def function(f):
        fields(f, ("name_id", "parameters", "return_type_id", "body"))
        ps = chain(f["parameters"], "ngNoParams", "ngParam", parameter)
        return [call("ngFunction", nat(f["name_id"]), ps, nat(f["return_type_id"]), nat(f["body"]))]
    return call("ngProgram", chain(p["declarations"], "ngNoDecls", "ngDecl", decl),
        chain(p["expressions"], "ngNoNodes", "ngNode", lambda row: [nat(row["id"]), expression(row)]),
        chain(p["functions"], "ngNoFunctions", "ngFunctionRow", function), nat(p["entry"]))


def environment(rows):
    names = set()
    out = []
    for row in rows:
        fields(row, ("name_id", "value"))
        key = nat(row["name_id"])
        if key in names:
            raise ValueError("Duplicate extensional environment key")
        names.add(key)
        out.append(key + " |-> " + value(row["value"]))
    # This is the already observed MAP surface form; no extra Map parentheses.
    return " ".join(out) if out else ".Map"


def write(path, raw):
    with Path(path).open("xb") as stream:
        stream.write(raw)
    return {"path": str(path), "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--input", type=Path, required=True)
    cli.add_argument("--api", type=Path, required=True)
    cli.add_argument("--syntax", type=Path, required=True)
    cli.add_argument("--output-dir", type=Path, required=True)
    args = cli.parse_args()
    source = json.loads(args.input.read_bytes())
    initial = call("ngInitial", program(source["ng_program"]), environment(source["initial_environment"]))
    parser.configure(args.api, args.syntax)
    decoded = parser.parse_printed(initial, sort="NGState")
    if decoded.name != "ngInitial":
        raise ValueError("Input serializer changed ngInitial")
    adapter = CompleteKWire(parser.API, parser.REG)
    if not same(adapter.decode(decoded.args[0]), source["ng_program"]) or not same(adapter.decode(decoded.args[1]), source["initial_environment"]):
        raise ValueError("Complete Program/Map inverse differs; no input emitted")
    budget = nat(source["observation_budget"])
    initial_pin = write(args.output_dir / "initial.ng", (initial + "\n").encode())
    query_pin = write(args.output_dir / "query.term", (call("ngObserve", initial, budget) + "\n").encode())
    print(json.dumps({"status": "PASS_COMPLETE_INPUT_SERIALIZATION_INVERSE", "initial": initial_pin,
        "query": query_pin, "model_steps_evaluated": 0, "expected_documents_opened": 0}))


if __name__ == "__main__":
    main()
