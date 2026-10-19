#!/usr/bin/env python3
"""Lossless typed-JSON carrier and partial ProgramSnapshot -> NGProgram port.

This module does not authenticate primary admission or evaluate a program. IDs
are public table indices, not the unrelated IDs of the earlier K example tapes.
Unsupported source forms invalidate the WHOLE port, including unreachable rows.
"""

import argparse
import hashlib
import json
from pathlib import Path


PROGRAM_FIELDS = (
    "entry", "expressions", "occurrences", "occurrence_paths", "children",
    "functions", "parameters", "arms", "patterns", "pattern_children", "types",
    "type_args", "constructors", "ctor_fields", "decl_param_counts",
    "decl_ctor_counts", "decl_templates", "plan_nodes", "plan_cases",
    "field_paths", "constants", "constant_fields", "names",
)
PROFILE = "primary-programsnapshot-to-ngprogram/common-positive/1"
PREMISES = [
    "JSON source authenticity and successful primary admission are external, unverified premises.",
    "Input is the public resolved ProgramSnapshot, with finite acyclic constant/pattern graphs and valid references.",
    "The common positive profile is typed and binder-linear; this port neither proves nor replaces primary admission.",
    "Add behavior is only claimed where its evaluated result is in UInt range; no arithmetic is evaluated here.",
    "Primary TypeIds are concrete monomorphic instances; templates are retained without substitution.",
    "Derived recursive_group is SCC-min TypeId of concrete constructor-field edges, not a primary group identifier.",
    "NG source_arm_id is the global primary ArmRow index, not an original source AST identifier.",
    "Match plans, admission/layout metadata, runtime/work clocks and native proofs have no counterpart claim here.",
    "Full source_document retention is typed JSON preservation, not original JSON whitespace or byte preservation.",
]


class PortError(Exception):
    def __init__(self, path, code, detail):
        super().__init__(detail)
        self.issue = {"path": path, "code": code, "detail": detail}


def record(value, keys, path):
    if type(value) is not dict or set(value) != set(keys):
        raise PortError(path, "shape", "Expected exactly these object fields: " + ",".join(keys))
    return value


def natural(value, path):
    if type(value) is not int or value < 0:
        raise PortError(path, "shape", "Expected nonnegative integer, excluding Bool")
    return value


def boolean(value, path):
    if type(value) is not bool:
        raise PortError(path, "shape", "Expected Bool")
    return value


def sequence(value, path):
    if type(value) is not list:
        raise PortError(path, "shape", "Expected array")
    return value


def variant(value, path):
    if type(value) is str:
        return value, {}
    if type(value) is dict and len(value) == 1:
        tag, payload = next(iter(value.items()))
        if type(payload) is dict:
            return tag, payload
    raise PortError(path, "shape", "Expected public string-unit or single-key object variant")


def select_pointer(document, pointer):
    if not pointer:
        return document
    if not pointer.startswith("/"):
        raise PortError("program_pointer", "pointer", "JSON Pointer must be empty or begin with /")
    current = document
    for token in pointer[1:].split("/"):
        # RFC6901 only permits ~0 and ~1; never guess a different path.
        if any(not part or part[0] not in "01" for part in token.split("~")[1:]):
            raise PortError("program_pointer", "pointer", "Invalid JSON Pointer escape")
        token = token.replace("~1", "/").replace("~0", "~")
        if type(current) is dict and token in current:
            current = current[token]
        elif type(current) is list and token.isascii() and token.isdecimal() and (token == "0" or not token.startswith("0")):
            index = int(token)
            if index >= len(current):
                raise PortError("program_pointer", "pointer", "Array pointer is out of range")
            current = current[index]
        else:
            raise PortError("program_pointer", "pointer", "Pointer does not select an existing value")
    return current


class Port:
    def __init__(self, program):
        self.p = record(program, PROGRAM_FIELDS, "program")
        self.issues = []
        self.origins = {
            "expressions": [], "functions": [], "arms": [], "patterns": [],
            "constants": [], "declarations": [], "constructor_rows": [],
            "recursive_group_convention": {
                "kind": "derivedSccMinimumTypeId",
                "edge_source": "constructors -> ctor_fields -> type_id",
                "includes_type_args_as_edges": False,
            },
        }
        for key in PROGRAM_FIELDS[1:]:
            sequence(self.p[key], key)
        self.entry = self.ref("expressions", self.p["entry"], "entry")
        for key in ("occurrence_paths", "children", "pattern_children", "type_args",
                    "decl_param_counts", "decl_ctor_counts", "field_paths", "constant_fields"):
            for index, value in enumerate(self.p[key]):
                natural(value, f"{key}/{index}")
        for index, value in enumerate(self.p["names"]):
            if type(value) is not str:
                raise PortError(f"names/{index}", "shape", "Expected name string")
        if len(self.p["occurrences"]) != len(self.p["expressions"]):
            raise PortError("occurrences", "unalignedOccurrenceRows", "Expression and occurrence row counts must agree")

    def ref(self, table, value, path):
        natural(value, path)
        if value >= len(self.p[table]):
            raise PortError(path, "reference", "Row outside " + table)
        return value

    def sliced(self, table, sl, path):
        record(sl, ("start", "len"), path)
        start = natural(sl["start"], path + "/start")
        length = natural(sl["len"], path + "/len")
        if start + length > len(self.p[table]):
            raise PortError(path, "reference", "Slice outside " + table)
        return self.p[table][start:start + length]

    def refs(self, table, values, path):
        return [self.ref(table, value, f"{path}/{i}") for i, value in enumerate(values)]

    def attempt(self, fn, *args):
        try:
            return fn(*args)
        except PortError as exc:
            self.issues.append(exc.issue)
            return None

    def pattern(self, index, completed, active):
        # Explicit stack: preserve recursive shapes without Python recursion.
        stack = [(index, False)]
        while stack:
            rowid, exiting = stack.pop()
            if rowid in completed:
                continue
            path = f"patterns/{rowid}"
            self.ref("patterns", rowid, path)
            tag, row = variant(self.p["patterns"][rowid], path)
            if exiting:
                children = self.refs("patterns", self.sliced("pattern_children", row["children"], path + "/children"), path + "/children")
                completed[rowid] = {"Constructor": {
                    "type_id": self.ref("types", row["type_id"], path + "/type_id"),
                    "ctor_id": self.ref("constructors", row["ctor_id"], path + "/ctor_id"),
                    "fields": [completed[child] for child in children],
                }}
                active.remove(rowid)
                continue
            if rowid in active:
                raise PortError(path, "cyclicPattern", "Positive port requires finite acyclic pattern data")
            if tag == "Constructor":
                record(row, ("type_id", "ctor_id", "children"), path)
                children = self.refs("patterns", self.sliced("pattern_children", row["children"], path + "/children"), path + "/children")
                active.add(rowid)
                stack.append((rowid, True))
                stack.extend((child, False) for child in reversed(children))
            elif tag == "PVar":
                record(row, ("x",), path)
                completed[rowid] = {"Bind": natural(row["x"], path + "/x")}
            elif tag == "Wildcard":
                record(row, (), path)
                completed[rowid] = {"Wildcard": {}}
            elif tag in ("PNil", "PCons", "PLeaf", "PBranch"):
                fields = {"PNil": (), "PCons": ("x", "y"), "PLeaf": ("x",), "PBranch": ("x", "y")}[tag]
                record(row, fields, path)
                tid, cid = {"PNil": (2, 0), "PCons": (2, 1), "PLeaf": (3, 2), "PBranch": (3, 3)}[tag]
                self.ref("types", tid, path + "/builtin_type")
                self.ref("constructors", cid, path + "/builtin_ctor")
                completed[rowid] = {"Constructor": {"type_id": tid, "ctor_id": cid,
                    "fields": [{"Bind": natural(row[key], path + "/" + key)} for key in fields]}}
            else:
                raise PortError(path, "unsupportedPattern", tag)
        return completed[index]

    def constant(self, index, completed, active):
        stack = [(index, False)]
        while stack:
            rowid, exiting = stack.pop()
            if rowid in completed:
                continue
            path = f"constants/{rowid}"
            self.ref("constants", rowid, path)
            tag, row = variant(self.p["constants"][rowid], path)
            if tag in ("Constructor", "Cons", "Leaf", "Branch"):
                keys = ("marker_free", "type_id", "ctor_id", "fields") if tag == "Constructor" else ("marker_free", "fields")
                record(row, keys, path)
                children = self.refs("constants", self.sliced("constant_fields", row["fields"], path + "/fields"), path + "/fields")
                if tag == "Constructor":
                    tid = self.ref("types", row["type_id"], path + "/type_id")
                    cid = self.ref("constructors", row["ctor_id"], path + "/ctor_id")
                else:
                    tid, cid = {"Cons": (2, 1), "Leaf": (3, 2), "Branch": (3, 3)}[tag]
                    self.ref("types", tid, path + "/builtin_type")
                    self.ref("constructors", cid, path + "/builtin_ctor")
                if exiting:
                    completed[rowid] = {"Nominal": {"type_id": tid, "ctor_id": cid, "fields": [completed[child] for child in children]}}
                    active.remove(rowid)
                    continue
            else:
                children = []
            if rowid in active:
                raise PortError(path, "cyclicConstant", "Positive port requires finite acyclic constant data")
            if "marker_free" not in row or not boolean(row["marker_free"], path + "/marker_free"):
                raise PortError(path, "markerBearingConstant", "Only marker-free primary values have this counterpart")
            if tag == "U128":
                record(row, ("marker_free", "n"), path)
                text = row["n"]
                if type(text) is not str or not text.isascii() or not text.isdecimal():
                    raise PortError(path + "/n", "shape", "U128 wire is a decimal string")
                number = int(text)
                if number >= 2 ** 128:
                    raise PortError(path + "/n", "unsupportedConstant", "U128 outside declared range")
                completed[rowid] = {"UInt": number}
            elif tag == "Bool":
                record(row, ("marker_free", "b"), path)
                completed[rowid] = {"Bool": boolean(row["b"], path + "/b")}
            elif tag == "Nil":
                record(row, ("marker_free",), path)
                self.ref("types", 2, path + "/builtin_type")
                self.ref("constructors", 0, path + "/builtin_ctor")
                completed[rowid] = {"Nominal": {"type_id": 2, "ctor_id": 0, "fields": []}}
            elif tag in ("Constructor", "Cons", "Leaf", "Branch"):
                active.add(rowid)
                stack.append((rowid, True))
                stack.extend((child, False) for child in reversed(children))
            else:
                raise PortError(path, "unsupportedConstant", tag)
        return completed[index]

    def expression(self, index, constants, patterns):
        path = f"expressions/{index}"
        tag, row = variant(self.p["expressions"][index], path)
        er = lambda value, suffix: self.ref("expressions", value, path + "/" + suffix)
        if tag == "Literal":
            record(row, ("constant",), path)
            cid = self.ref("constants", row["constant"], path + "/constant")
            if cid not in constants:
                raise PortError(path, "unportedConstant", "Literal refers to an unported constant")
            kind = {"Literal": constants[cid]}
        elif tag == "Var":
            record(row, ("binder",), path)
            kind = {"Variable": {"name_id": natural(row["binder"], path + "/binder")}}
        elif tag == "Let":
            record(row, ("binder", "value", "body"), path)
            kind = {"Let": {"binder": natural(row["binder"], path + "/binder"),
                "bound_expr": er(row["value"], "value"), "body_expr": er(row["body"], "body")}}
        elif tag == "If":
            record(row, ("cond", "then_expr", "else_expr"), path)
            kind = {"If": {"condition_expr": er(row["cond"], "cond"),
                "yes_expr": er(row["then_expr"], "then_expr"), "no_expr": er(row["else_expr"], "else_expr")}}
        elif tag == "Call":
            record(row, ("function_name", "args"), path)
            kind = {"Call": {"name_id": natural(row["function_name"], path + "/function_name"),
                "arguments": self.refs("expressions", self.sliced("children", row["args"], path + "/args"), path + "/args")}}
        elif tag == "Construct":
            record(row, ("type_id", "ctor_id", "fields", "constant"), path)
            if row["constant"] is not None:
                self.ref("constants", row["constant"], path + "/constant")
            kind = {"Construct": {"type_id": self.ref("types", row["type_id"], path + "/type_id"),
                "ctor_id": self.ref("constructors", row["ctor_id"], path + "/ctor_id"),
                "fields": self.refs("expressions", self.sliced("children", row["fields"], path + "/fields"), path + "/fields")}}
        elif tag == "Match":
            record(row, ("scrutinee", "arms", "plan"), path)
            if row["plan"] is not None:
                self.ref("plan_nodes", row["plan"], path + "/plan")
            source_arms = self.sliced("arms", row["arms"], path + "/arms")
            arms = []
            for local, arm in enumerate(source_arms):
                armid = row["arms"]["start"] + local
                record(arm, ("pattern", "body"), f"arms/{armid}")
                patternid = self.ref("patterns", arm["pattern"], f"arms/{armid}/pattern")
                if patternid not in patterns:
                    raise PortError(f"arms/{armid}/pattern", "unportedPattern", "Arm refers to an unported pattern")
                arms.append({"source_arm_id": armid, "pattern": patterns[patternid], "body": er(arm["body"], "arm_body")})
                self.origins["arms"].append({"match_expr_id": index, "local_arm_ordinal": local,
                    "global_arm_row": armid, "pattern_row": patternid, "source_row": arm})
            kind = {"Match": {"scrutinee": er(row["scrutinee"], "scrutinee"), "arms": arms}}
        elif tag == "Binary":
            record(row, ("op", "left", "right"), path)
            left, right = er(row["left"], "left"), er(row["right"], "right")
            if row["op"] == "Add":
                kind = {"Add": {"left": left, "right": right}}
            elif row["op"] in ("Cons", "Branch"):
                tid, cid = {"Cons": (2, 1), "Branch": (3, 3)}[row["op"]]
                self.ref("types", tid, path + "/builtin_type")
                self.ref("constructors", cid, path + "/builtin_ctor")
                kind = {"Construct": {"type_id": tid, "ctor_id": cid, "fields": [left, right]}}
            else:
                raise PortError(path + "/op", "unsupportedExpression", str(row["op"]))
        elif tag == "Unary":
            record(row, ("op", "operand"), path)
            if row["op"] != "Leaf":
                raise PortError(path + "/op", "unsupportedExpression", str(row["op"]))
            self.ref("types", 3, path + "/builtin_type")
            self.ref("constructors", 2, path + "/builtin_ctor")
            kind = {"Construct": {"type_id": 3, "ctor_id": 2, "fields": [er(row["operand"], "operand")]}}
        else:
            raise PortError(path, "unsupportedExpression", tag)
        return {"id": index, "occurrence": index, "kind": kind}

    def declaration(self, index, groups):
        path = f"types/{index}"
        row = record(self.p["types"][index], ("name_id", "decl", "args", "ctor_start", "ctor_len"), path)
        natural(row["name_id"], path + "/name_id")
        args = self.refs("types", self.sliced("type_args", row["args"], path + "/args"), path + "/args")
        if row["decl"] is not None:
            self.ref("decl_templates", row["decl"], path + "/decl")
        ctors = self.sliced("constructors", {"start": row["ctor_start"], "len": row["ctor_len"]}, path + "/constructors")
        mapped = []
        for local, ctor in enumerate(ctors):
            cid = row["ctor_start"] + local
            field_types = self.ctor_fields(cid)
            mapped.append({"type_id": index, "ctor_id": cid,
                "source_order": natural(ctor["source_order"], f"constructors/{cid}/source_order"), "field_types": field_types})
        self.origins["declarations"].append({"type_id": index, "source_row": row,
            "source_type_arguments": args, "derived_recursive_group": groups[index]})
        return {"type_id": index, "recursive_group": groups[index], "ctors": mapped}

    def ctor_fields(self, index):
        path = f"constructors/{index}"
        row = record(self.p["constructors"][index], ("owning_type", "name_id", "source_order", "field_start", "field_len"), path)
        self.ref("types", row["owning_type"], path + "/owning_type")
        natural(row["name_id"], path + "/name_id")
        natural(row["source_order"], path + "/source_order")
        fields = self.sliced("ctor_fields", {"start": row["field_start"], "len": row["field_len"]}, path + "/fields")
        output = []
        for local, field in enumerate(fields):
            fieldpath = f"ctor_fields/{row['field_start'] + local}"
            record(field, ("name_id", "type_id"), fieldpath)
            natural(field["name_id"], fieldpath + "/name_id")
            output.append(self.ref("types", field["type_id"], fieldpath + "/type_id"))
        return output

    def groups(self):
        # Iterative Kosaraju; labels depend on concrete graph, never row order
        # of a DFS, source declaration grouping, or any example's group ID.
        graph = [set() for _ in self.p["types"]]
        reverse = [set() for _ in graph]
        for cid, row in enumerate(self.p["constructors"]):
            fields = self.ctor_fields(cid)
            owner = row["owning_type"]
            graph[owner].update(fields)
            for target in fields:
                reverse[target].add(owner)
            self.origins["constructor_rows"].append({"ctor_id": cid, "source_row": row,
                "source_fields": self.sliced("ctor_fields", {"start": row["field_start"], "len": row["field_len"]}, f"constructors/{cid}/fields")})
        visited, order = set(), []
        for root in range(len(graph)):
            stack = [(root, False)]
            while stack:
                node, exiting = stack.pop()
                if exiting:
                    order.append(node)
                elif node not in visited:
                    visited.add(node)
                    stack.append((node, True))
                    stack.extend((child, False) for child in sorted(graph[node], reverse=True) if child not in visited)
        labels = {}
        for root in reversed(order):
            if root in labels:
                continue
            component, stack = set(), [root]
            while stack:
                node = stack.pop()
                if node in component or node in labels:
                    continue
                component.add(node)
                stack.extend(reverse[node])
            label = min(component)
            labels.update((node, label) for node in component)
        return labels

    def function(self, index):
        path = f"functions/{index}"
        row = record(self.p["functions"][index], ("function_name", "params", "return_type", "body"), path)
        params = self.sliced("parameters", row["params"], path + "/params")
        mapped = []
        for local, param in enumerate(params):
            ppath = f"parameters/{row['params']['start'] + local}"
            record(param, ("binder", "sort"), ppath)
            mapped.append({"name_id": natural(param["binder"], ppath + "/binder"),
                "type_id": self.ref("types", param["sort"], ppath + "/sort")})
        self.origins["functions"].append({"function_row": index, "source_row": row, "ordered_parameter_rows": params})
        return {"name_id": natural(row["function_name"], path + "/function_name"), "parameters": mapped,
            "return_type_id": self.ref("types", row["return_type"], path + "/return_type"),
            "body": self.ref("expressions", row["body"], path + "/body")}

    def occurrence(self, index):
        path = f"occurrences/{index}"
        row = record(self.p["occurrences"][index], ("owner_is_function", "function", "path"), path)
        is_function = boolean(row["owner_is_function"], path + "/owner_is_function")
        owner = natural(row["function"], path + "/function")
        owner_row = self.p["functions"][self.ref("functions", owner, path + "/function")] if is_function else None
        source_path = self.sliced("occurrence_paths", row["path"], path + "/path")
        self.origins["expressions"].append({"expression_id": index, "ng_occurrence_id": index,
            "source_expression": self.p["expressions"][index], "occurrence_row": row,
            "owner_function_row": owner_row, "root_first_source_path": source_path})

    def run(self):
        constants, patterns = {}, {}
        # Each row starts an independent work stack so one unsupported/cyclic
        # row cannot leak pending state into subsequent rows.
        for index, row in enumerate(self.p["constants"]):
            self.attempt(self.constant, index, constants, set())
            self.origins["constants"].append({"constant_row": index, "source_row": row})
        for index, row in enumerate(self.p["patterns"]):
            self.attempt(self.pattern, index, patterns, set())
            self.origins["patterns"].append({"pattern_row": index, "source_row": row})
        groups = self.attempt(self.groups)
        declarations = []
        if groups is not None:
            for index in range(len(self.p["types"])):
                decl = self.attempt(self.declaration, index, groups)
                if index >= 2:
                    declarations.append(decl)
        expressions = []
        for index in range(len(self.p["expressions"])):
            self.attempt(self.occurrence, index)
            expressions.append(self.attempt(self.expression, index, constants, patterns))
        functions = [self.attempt(self.function, index) for index in range(len(self.p["functions"]))]
        program = {"declarations": declarations, "expressions": expressions, "functions": functions, "entry": self.entry}
        return None if self.issues else program


def port_document(document, program_pointer=""):
    """Return a lossless carrier; ng_program is all-or-nothing for this profile."""
    envelope = {
        "format": "moriarty-primary-programsnapshot-ngprogram-port/1",
        "profile": PROFILE, "partial_counterpart": True,
        "source_authenticity": "unverified; primary admission/capture provenance must be independently bound",
        "source_document": document, "program_pointer": program_pointer,
        "ng_program": None, "origins": {},
        "field_inventory": {
            "source_program_fields": list(PROGRAM_FIELDS),
            "mapped_to_ng": ["entry", "expressions", "children", "functions", "parameters", "arms", "patterns", "pattern_children", "types", "constructors", "ctor_fields", "constants", "constant_fields"],
            "source_origin_sidecars": ["occurrences", "occurrence_paths", "type_args", "decl_param_counts", "decl_ctor_counts", "decl_templates", "names"],
            "retained_without_plan_or_work_equivalence": ["plan_nodes", "plan_cases", "field_paths"],
            "complete_original_carrier": "source_document; includes every ProgramSnapshot field and all surrounding input fields",
            "non_program_catalog_scope": "Remaining public capture types are retained in source_document but not interpreted or mapped",
        },
        "unhandled": [], "premises": list(PREMISES),
    }
    try:
        port = Port(select_pointer(document, program_pointer))
        envelope["ng_program"] = port.run()
        envelope["origins"] = port.origins
        envelope["unhandled"] = port.issues
    except PortError as exc:
        envelope["unhandled"].append(exc.issue)
    envelope["status"] = "ported" if envelope["ng_program"] is not None else "unhandled"
    return envelope


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--program-pointer", required=True, help="Explicit RFC6901 pointer; empty string selects the root")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    raw = args.input.read_bytes()
    document = json.loads(raw)
    output = port_document(document, args.program_pointer)
    output["source_file"] = {"bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}
    encoded = (json.dumps(output, ensure_ascii=False, allow_nan=False, indent=2) + "\n").encode("utf-8")
    with args.output.open("xb") as stream:
        stream.write(encoded)
    print(json.dumps({"status": output["status"], "output": str(args.output),
        "output_bytes": len(encoded), "output_sha256": hashlib.sha256(encoded).hexdigest(),
        "unhandled_count": len(output["unhandled"])}))
    return 0 if output["status"] == "ported" else 2


if __name__ == "__main__":
    raise SystemExit(main())
