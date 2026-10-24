#!/usr/bin/env python3
"""Passive saved-capture projection; no source evaluator or producer imports.

project has no expected-input option. compare opens expected only after checking
the externally Root-selected freeze and every retained actual/projection pin.
This finite consumer is not an admission checker or physical-step oracle.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import sys

SCHEMA = "sherlock-nominal-text-symbolic-expectation/1"
ACTUAL_SCHEMA = "sherlock-nominal-text-typed-runtime-actual/1"
PROFILE = "moriarty-sherlock-nominal-body-typed/1"
JUDGMENTS = ["eq-any", "unique-parameters-every-declaration", "unique-binders-every-arm"]
SAFE_SHA = "be61b73bdf3629e02206d6e25b496823e027524bb55b317a449a00c168c063cf"
INPUT_SHA = "c74ae94bcd9ce7fb11f77122beabbc28855bd4e7d873764d74a11879964e8a5b"
PROGRAM_KEYS = set("entry expressions occurrences occurrence_paths children functions parameters arms patterns pattern_children types type_args constructors ctor_fields decl_param_counts decl_ctor_counts decl_templates plan_nodes plan_cases field_paths constants constant_fields names".split())
IMAGE_KEYS = set("run_id control control_origin env_root stack_limit outcome initial_fuel remaining_work consumed_work helper_work helper_visits reference_cursor alloc active helper result seg_base data data_fields env frames tasks value_slots".split())
SEMANTIC = ("final_value", "calls", "matches", "constructors", "pending_checkpoints")
LIMITS = ["source-symbolic equality only; no physical clocks/IDs/cost/complete microstep oracle", "no admission, native, circuit, recursive proof, target or EARS acceptance"]

# Owner new_checked seed, by position: builtin TypeIds 0..3 (name, ctor_start, ctor_len),
# constructors 0..3 (owning_type, source_order, field_start, field_len), field type IDs 0..4.
BUILTIN_TYPES = (("UInt", 0, 0), ("Bool", 0, 0), ("List", 0, 2), ("Tree", 2, 2))
BUILTIN_CTORS = ((2, 0, 0, 0), (2, 1, 0, 2), (3, 0, 2, 1), (3, 1, 3, 2))
BUILTIN_FIELDS = (0, 2, 0, 3, 3)


class Gap(Exception):
    def __init__(self, path, reason):
        self.path, self.reason = path, reason
        super().__init__(path + ": " + reason)


def need(condition, path, reason):
    if not condition:
        raise Gap(path, reason)


def keys(value, wanted, path):
    need(type(value) is dict and set(value) == set(wanted), path,
         "exact object fields required: " + ",".join(sorted(wanted)))
    return value


def index(value, path):
    need(type(value) is int and value >= 0, path, "nonnegative integer required (not bool)")
    return value


def at(rows, pos, path):
    index(pos, path)
    need(type(rows) is list and pos < len(rows), path, "row out of bounds")
    return rows[pos]


def variant(value, path):
    if type(value) is str:
        return value, {}
    need(type(value) is dict and len(value) == 1, path, "one enum variant required")
    return next(iter(value.items()))


def tag(value, expected, path):
    name, body = variant(value, path)
    need(name == expected, path, "required variant " + expected)
    return body


def cut(rows, sl, path, normalized=False):
    # Only owner-normalized roles (occurrence paths) require start 0 when empty;
    # other empty program/image slices are bounds-checked only.
    keys(sl, ("start", "len"), path)
    a, n = index(sl["start"], path), index(sl["len"], path)
    need(n != 0 or a == 0 or not normalized, path, "non-normalized empty slice")
    need(type(rows) is list and a + n <= len(rows), path, "slice out of bounds")
    return rows[a:a+n]


def same(a, b):
    # Python otherwise aliases bool to integer. JSON objects ignore key order.
    # One immutable parsed object (the normalized admission) equals itself.
    if a is b:
        return True
    if type(a) is not type(b):
        return False
    if type(a) is dict:
        return set(a) == set(b) and all(same(a[k], b[k]) for k in a)
    if type(a) is list:
        return len(a) == len(b) and all(same(x, y) for x, y in zip(a, b))
    return a == b


def decimal(value, path):
    need(type(value) is str and re.fullmatch(r"0|[1-9][0-9]*", value) is not None,
         path, "canonical decimal UInt128 string required")
    need(len(value) <= 39 and int(value) < 2**128, path, "UInt128 out of range")
    return value


def reject_pairs(pairs):
    out = {}
    for key, value in pairs:
        need(key not in out, "$", "duplicate JSON key: " + key)
        out[key] = value
    return out


def parse(raw, path):
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=reject_pairs,
                          parse_float=lambda _: (_ for _ in ()).throw(Gap(path, "float forbidden")),
                          parse_constant=lambda _: (_ for _ in ()).throw(Gap(path, "nonfinite JSON forbidden")))
    except (UnicodeError, ValueError, RecursionError) as exc:
        raise Gap(path, "invalid or excessive JSON: " + str(exc)) from exc


def pin_raw(path):
    # One read: the exact bytes hashed here are the bytes returned to callers.
    raw = Path(path).read_bytes()
    return {"path": str(Path(path).resolve()), "bytes": len(raw),
            "sha256": hashlib.sha256(raw).hexdigest()}, raw


def pin(path):
    return pin_raw(path)[0]


def checked(spec):
    keys(spec, ("path", "bytes", "sha256"), "pin")
    need(type(spec["path"]) is str and Path(spec["path"]).is_absolute(), "pin", "absolute file path required")
    actual, raw = pin_raw(spec["path"])
    need(same(actual, spec), spec["path"], "file byte/hash/path pin mismatch")
    return raw


def json_pin(spec):
    return parse(checked(spec), spec["path"])


def save(directory, name, value):
    raw = (json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2) + "\n").encode()
    path = directory / name
    with path.open("xb") as out:
        out.write(raw)
        out.flush()
        os.fsync(out.fileno())
    return pin(path)


def sync_dir(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


class Projection:
    """Decode actual tables and recognize only successful source transitions."""
    def __init__(self, program, provenance, source):
        keys(program, PROGRAM_KEYS, "program")
        self.p, self.prov, self.source = program, provenance, source
        self.ledger, self.gaps = [], []
        self.out = {"final_value": None, "calls": [], "matches": [],
                    "constructors": {"open_order": [], "materialize_order": []},
                    "pending_checkpoints": [], "limitations": LIMITS[:]}
        self.coords, self.types, self.pattern_coords = {}, {}, {}
        self.calls, self.live_ctor, self.materializing = [], {}, None
        self.enter, self.published = None, False
        self.plan = None
        self._associate()

    def name(self, ident):
        name = at(self.p["names"], ident, "program.names")
        need(type(name) is str and name != "", "program.names", "nonempty source name required")
        return name

    def source_name(self, row, field="name_span"):
        span = keys(row[field], ("start", "end"), "provenance." + field)
        a, b = index(span["start"], "span.start"), index(span["end"], "span.end")
        raw = self.source.encode("utf-8")
        need(a <= b <= len(raw), "span", "source byte span out of bounds")
        try:
            text = raw[a:b].decode("utf-8")
        except UnicodeError as exc:
            raise Gap("span", "span splits UTF-8") from exc
        need(text == row["name"], "provenance.name", "name not exact original source span")
        return text

    def owner(self, row):
        if same(row, {"entry": True}):
            return {"entry": True}
        keys(row, ("function_index",), "provenance.owner")
        i = index(row["function_index"], "provenance.owner")
        return {"function": self.source_name(at(self.prov["functions"], i, "provenance.functions"))}

    def children(self, expr):
        t, b = variant(expr, "expression")
        if t in ("Literal", "Var"):
            return []
        if t == "Let":
            return [b["value"], b["body"]]
        if t == "Binary":
            return [b["left"], b["right"]]
        if t == "Unary":
            return [b["operand"]]
        if t == "If":
            return [b["cond"], b["then_expr"], b["else_expr"]]
        if t in ("Call", "Construct"):
            return cut(self.p["children"], b["args" if t == "Call" else "fields"], "program.children")
        if t == "Match":
            return [b["scrutinee"]] + [a["body"] for a in cut(self.p["arms"], b["arms"], "program.arms")]
        raise Gap("expression", "unsupported expression variant " + t)

    def _associate(self):
        p, v = self.p, self.prov
        need(len(p["expressions"]) == len(p["occurrences"]) == len(v["expressions"]),
             "program.occurrences", "whole expression/provenance membership differs")
        need(len(p["functions"]) == len(v["functions"]), "program.functions", "function membership differs")
        function_names = []
        for i, f in enumerate(p["functions"]):
            src = v["functions"][i]
            name = self.source_name(src)
            need(name == self.name(f["function_name"]) and name not in function_names,
                 "program.functions", "ambiguous or mismatched source function")
            function_names.append(name)
            params = cut(p["parameters"], f["params"], "parameters")
            need(len(params) == len(src["parameters"]), "parameters", "source parameter membership differs")
            for actual, declared in zip(params, src["parameters"]):
                need(self.name(actual["binder"]) == self.source_name(declared, "binder_span"),
                     "parameters", "parameter source name mismatch")
        roots = [(p["entry"], {"entry": True})] + [(f["body"], {"function": function_names[i]}) for i, f in enumerate(p["functions"])]
        seen = set()
        for root, owner in roots:
            stack = [(root, [])]
            while stack:
                ident, path = stack.pop()
                row = at(p["expressions"], ident, "expressions")
                need(ident not in seen, "expressions", "shared/cyclic or multiple-owner expression")
                seen.add(ident)
                occ = p["occurrences"][ident]
                keys(occ, ("owner_is_function", "function", "path"), "occurrence")
                need(type(occ["owner_is_function"]) is bool, "occurrence", "owner flag not boolean")
                index(occ["function"], "occurrence.function")
                actual_owner = {"entry": True} if not occ["owner_is_function"] else {"function": at(function_names, occ["function"], "occurrence.function")}
                if not occ["owner_is_function"]:
                    need(occ["function"] == 0, "occurrence", "entry function index must be zero")
                need(same(actual_owner, owner) and same(cut(p["occurrence_paths"], occ["path"], "occurrence_paths", True), path),
                     "occurrence", "C1/C4 owner/path/ExprID association mismatch")
                src = v["expressions"][ident]
                need(same(self.owner(src["owner"]), owner) and same(src["path"], path),
                     "provenance.expressions", "source preorder does not equal emitted occurrence")
                kind, body = variant(row, "expression")
                allowed = {"Literal": {"uint", "bool", "nil"}, "Var": {"var"}, "Let": {"let"},
                           "Binary": {"binary", "dedicated-constructor"}, "Unary": {"dedicated-constructor"},
                           "If": {"if"}, "Call": {"call"}, "Match": {"match"}, "Construct": {"construct"}}
                need(src["kind"] in allowed.get(kind, set()), "provenance.kind", "source/admitted expression kind mismatch")
                self.coords[ident] = {"owner": owner, "path": path}
                self.ledger.append({"association": "expression", "expr_id": ident, "source": self.coords[ident], "span": src["span"]})
                children = self.children(row)
                for child_index in reversed(range(len(children))):
                    stack.append((children[child_index], path + [child_index]))
        need(len(seen) == len(p["expressions"]), "expressions", "unreachable extra expression membership")
        self._associate_patterns()
        self._builtin_rows()
        for i in range(len(p["types"])):
            self.typ(i)
        self._declarations()

    def _builtin_rows(self):
        # Owner generic-view header plus exact new_checked seed rows. Builtin
        # name_id 0 is padding and is never resolved against program.names.
        p = self.p
        need(same(p["entry"], 0) and len(p["types"]) >= len(BUILTIN_TYPES)
             and len(p["constructors"]) >= len(BUILTIN_CTORS) and len(p["ctor_fields"]) >= len(BUILTIN_FIELDS),
             "program", "owner entry/builtin row membership differs")
        for cid, (owner, order, start, length) in enumerate(BUILTIN_CTORS):
            seed = {"owning_type": owner, "name_id": 0, "source_order": order, "field_start": start, "field_len": length}
            need(same(p["constructors"][cid], seed), "constructors", "builtin constructor row differs from owner seed")
        for fid, tid in enumerate(BUILTIN_FIELDS):
            need(same(p["ctor_fields"][fid], {"name_id": 0, "type_id": tid}),
                 "ctor_fields", "builtin constructor field row differs from owner seed")

    def _associate_patterns(self):
        declared = {}
        for row in self.prov["patterns"]:
            key = json.dumps([self.owner(row["owner"]), row["match_path"], row["arm"], row["field_path"]], sort_keys=True)
            need(key not in declared, "provenance.patterns", "duplicate source pattern coordinate")
            declared[key] = row
        consumed = set()
        for expr, source in self.coords.items():
            kind, body = variant(self.p["expressions"][expr], "expression")
            if kind != "Match":
                continue
            for ordinal, arm in enumerate(cut(self.p["arms"], body["arms"], "match.arms")):
                stack = [(arm["pattern"], [])]
                while stack:
                    ident, path = stack.pop()
                    need(ident not in self.pattern_coords, "patterns", "shared or cyclic pattern row")
                    key = json.dumps([source["owner"], source["path"], ordinal, path], sort_keys=True)
                    need(key in declared, "patterns", "missing complete source pattern association")
                    src = declared[key]
                    consumed.add(key)
                    tagname, row = variant(at(self.p["patterns"], ident, "patterns"), "pattern")
                    expected_kind = {"PVar": "binder", "Wildcard": "wildcard", "Constructor": "constructor-pattern",
                                     "PNil": "nil", "PCons": "dedicated-pattern", "PLeaf": "dedicated-pattern", "PBranch": "dedicated-pattern"}
                    need(src["kind"] == expected_kind.get(tagname), "patterns", "source pattern kind mismatch")
                    if tagname == "PVar":
                        self.source_name({"name": self.name(row["x"]), "span": src["span"]}, "span")
                    self.pattern_coords[ident] = {"source_match": source, "arm": ordinal, "field_path": path}
                    self.ledger.append({"association": "pattern", "pattern_id": ident, **self.pattern_coords[ident], "span": src["span"]})
                    if tagname == "Constructor":
                        children = cut(self.p["pattern_children"], row["children"], "pattern_children")
                        for i in reversed(range(len(children))):
                            stack.append((children[i], path + [i]))
        need(consumed == set(declared) and len(self.pattern_coords) == len(self.p["patterns"]),
             "patterns", "unconsumed extra source/admitted pattern membership")

    def coord(self, ident):
        index(ident, "expr_id")
        need(ident in self.coords, "expr_id", "unassociated source expression")
        return self.coords[ident]

    def typ(self, ident, trail=()):
        index(ident, "type_id")
        if ident in self.types:
            return self.types[ident]
        need(ident not in trail, "type_args", "cyclic type application")
        row = keys(at(self.p["types"], ident, "types"), ("name_id", "decl", "args", "ctor_start", "ctor_len"), "types")
        if ident < len(BUILTIN_TYPES):
            # Owner rows 0..3 are positional; name_id 0 is padding, never a name.
            name, start, length = BUILTIN_TYPES[ident]
            need(row["decl"] is None and same(row["args"], {"start": 0, "len": 0})
                 and same([row["name_id"], row["ctor_start"], row["ctor_len"]], [0, start, length]),
                 "types", "builtin type row differs from owner positional layout")
            value = {"builtin": name}
        else:
            need(row["decl"] is not None, "types", "user type row >= 4 requires a declaration")
            name = self.name(row["name_id"])
            args = [self.typ(x, trail + (ident,)) for x in cut(self.p["type_args"], row["args"], "type_args")]
            decl = at(self.p["decl_templates"], row["decl"], "decl_templates")
            need(name == decl["name"] and len(args) == len(decl["parameters"])
                 == at(self.p["decl_param_counts"], row["decl"], "decl_param_counts"),
                 "types", "nominal declaration/application mismatch")
            value = {"named": name, "arguments": args}
        self.types[ident] = value
        return value

    def type_expr(self, expr, params):
        kind, b = variant(expr, "type expression")
        if kind in ("UInt", "Bool", "List", "Tree"):
            return {"builtin": kind}
        if kind == "Param":
            need(b["name"] in params, "type expression", "unbound declaration parameter")
            return params[b["name"]]
        if kind in ("Named", "Apply"):
            return {"named": b["name"], "arguments": [self.type_expr(x, params) for x in b.get("args", [])]}
        raise Gap("type expression", "unknown source type variant")

    def _declarations(self):
        ds = self.p["decl_templates"]
        need(len(ds) == len(self.prov["declarations"]) == len(self.p["decl_param_counts"]) == len(self.p["decl_ctor_counts"]),
             "declarations", "whole declaration membership differs")
        names = []
        for d, src in zip(ds, self.prov["declarations"]):
            name = self.source_name(src)
            need(d["name"] == name and name not in names, "declarations", "ambiguous declaration name")
            names.append(name)
            need(d["parameters"] == [self.source_name(x, "span") for x in src["parameters"]], "declarations", "type parameter source mismatch")
            need([c["name"] for c in d["ctors"]] == [self.source_name(c) for c in src["constructors"]],
                 "declarations", "constructor source order mismatch")
            for ctor, declared in zip(d["ctors"], src["constructors"]):
                need([f["name"] for f in ctor["fields"]] == [self.source_name(f, "binder_span") for f in declared["fields"]],
                     "declarations", "constructor declaration field source order mismatch")
        for i, t in enumerate(self.p["types"]):
            if t["decl"] is None:
                continue
            d = ds[t["decl"]]
            need(self.p["decl_param_counts"][t["decl"]] == len(d["parameters"]) and self.p["decl_ctor_counts"][t["decl"]] == len(d["ctors"]),
                 "declarations", "declaration count mismatch")
            params = dict(zip(d["parameters"], self.typ(i)["arguments"]))
            need(t["ctor_len"] == len(d["ctors"]), "constructors", "constructor membership differs")
            for order, declared in enumerate(d["ctors"]):
                ctor_id = t["ctor_start"] + order
                c = at(self.p["constructors"], ctor_id, "constructors")
                need(c["owning_type"] == i and c["source_order"] == order and self.name(c["name_id"]) == declared["name"],
                     "constructors", "nominal owner/name/source order mismatch")
                fields = self.p["ctor_fields"][c["field_start"]:c["field_start"]+c["field_len"]]
                need(len(fields) == c["field_len"] == len(declared["fields"]), "ctor_fields", "field membership differs")
                for a, b in zip(fields, declared["fields"]):
                    need(self.name(a["name_id"]) == b["name"] and same(self.typ(a["type_id"]), self.type_expr(b["ty"], params)),
                         "ctor_fields", "field name or instantiated type mismatch")

    def ctor(self, tid, cid):
        c = at(self.p["constructors"], cid, "constructor")
        t = at(self.p["types"], tid, "constructor.type")
        need(tid >= len(BUILTIN_TYPES), "constructor", "dedicated builtin TypeId in nominal constructor kind")
        index(c["owning_type"], "constructor.owner")
        need(c["owning_type"] == tid and t["ctor_start"] <= cid < t["ctor_start"] + t["ctor_len"],
             "constructor", "wrong owning type")
        return self.typ(tid), self.name(c["name_id"]), c

    def image(self, image):
        keys(image, IMAGE_KEYS, "image")
        for table in ("data", "data_fields", "env", "value_slots"):
            need(index(image["active"][table], "image.active") == len(image[table]),
                 "image." + table, "complete active copied table required")

    def value(self, ref, image, trail=()):
        kind, ident = variant(ref, "value ref")
        need(kind in ("Const", "Data"), "value ref", "unsupported ref tag")
        key = (kind, index(ident, "value ref"))
        need(key not in trail, "value ref", "cyclic finite value")
        row = at(self.p["constants"] if kind == "Const" else image["data"], ident, "value row")
        t, b = variant(row, "value row")
        if kind == "Const":
            need(b.get("marker_free") is True, "constant", "invalid/marker constant")
        if t == "U128":
            return {"UInt": decimal(b["n"], "value.UInt")}
        if t == "Bool":
            need(type(b["b"]) is bool, "value.Bool", "Boolean required")
            return {"Bool": b["b"]}
        need(t == "Constructor", "value", "representation contract does not define dedicated value form " + t)
        ty, name, c = self.ctor(b["type_id"], b["ctor_id"])
        refs = cut(self.p["constant_fields"] if kind == "Const" else image["data_fields"], b["fields"], "value.fields")
        need(len(refs) == c["field_len"], "value.fields", "constructor arity differs")
        vals = [self.value({"Const": x} if kind == "Const" else x, image, trail + (key,)) for x in refs]
        for n, val in enumerate(vals):
            expected = self.typ(at(self.p["ctor_fields"], c["field_start"] + n, "ctor_fields")["type_id"])
            need(same(self.value_type(val), expected), "value.fields", "field value type mismatch")
        # No unlocated per-decode ledger row; raw refs stay in located lifecycle rows.
        return {"Nominal": {"type": ty, "constructor": name, "fields": vals}}

    @staticmethod
    def value_type(value):
        kind, b = variant(value, "symbolic value")
        return {"builtin": "UInt" if kind == "UInt" else "Bool"} if kind in ("UInt", "Bool") else b["type"]

    def env(self, root, image):
        result, seen = {}, set()
        while root is not None:
            index(root, "env_root")
            need(root not in seen, "env", "cyclic lexical environment")
            seen.add(root)
            cell = at(image["env"], root, "env")
            keys(cell, ("binder", "value", "previous"), "env.cell")
            name = self.name(cell["binder"])
            # Actual EnvGet follows root -> previous and stops at first match.
            if name not in result:
                result[name] = self.value(cell["value"], image)
            root = cell["previous"]
        return result

    def vals(self, seq, image):
        return [self.value(r, image) for r in cut(image["value_slots"], seq, "value_slots")]

    def pending(self, image):
        result = []
        seen = set()
        for depth, frame in enumerate(image["frames"]):
            name, f = variant(frame, "frame")
            if name != "ConstructorFieldsF":
                continue
            need(depth in self.live_ctor, "pending", "constructor frame has no unique opening history")
            obj = self.live_ctor[depth]
            need(f["origin"] == obj["expr"] and f["type_id"] == obj["tid"] and f["ctor_id"] == obj["cid"],
                 "pending", "frame changed construction identity")
            i = index(f["index"], "pending.index")
            vals = self.vals(f["completed"], image)
            need(i < len(obj["record"]["field_sources"]) and len(vals) == i,
                 "pending", "active field/completed prefix mismatch")
            source_fields = cut(self.p["children"], tag(self.p["expressions"][obj["expr"]], "Construct", "pending.source")["fields"], "pending.source.fields")
            need(same(cut(self.p["children"], f["rest"], "pending.rest"), source_fields[i+1:]),
                 "pending", "remaining field expressions differ from original source suffix")
            ctor = self.ctor(obj["tid"], obj["cid"])[2]
            need(f["field_type"] == at(self.p["ctor_fields"], ctor["field_start"] + i, "pending.field_type")["type_id"],
                 "pending", "active field declared type differs")
            need(f["rest"]["len"] == len(obj["record"]["field_sources"]) - i - 1,
                 "pending", "remaining source field count mismatch")
            need(same(self.env(f["saved_env"], image), obj["record"]["saved_environment"]),
                 "pending", "saved lexical environment changed")
            seen.add(depth)
            result.append({"construction_invocation": obj["record"]["invocation"],
                           "source": obj["record"]["source"], "current_field": i,
                           "completed_fields": vals, "saved_environment": obj["record"]["saved_environment"]})
        need(seen == set(self.live_ctor), "pending", "opened live constructor missing from complete frames")
        return result

    def _open(self, expr, before, record):
        b = tag(at(self.p["expressions"], expr, "construct.expr"), "Construct", "construct.expr")
        ty, name, _ = self.ctor(b["type_id"], b["ctor_id"])
        item = {"source": self.coord(expr), "invocation": len(self.out["constructors"]["open_order"]),
                "type": ty, "constructor": name,
                "field_sources": [self.coord(x) for x in cut(self.p["children"], b["fields"], "construct.fields")],
                "field_values": None, "saved_environment": self.env(before["env_root"], before),
                "in_call": self.calls[-1]["invocation"] if self.calls else None}
        obj = {"record": item, "expr": expr, "tid": b["type_id"], "cid": b["ctor_id"], "opened": record}
        self.out["constructors"]["open_order"].append(item)
        return obj

    def _complete(self, obj, after, record):
        value = self.value(tag(after["control"], "Return", "construct.return")["value"], after)
        b = tag(value, "Nominal", "construct.value")
        need(same(b["type"], obj["record"]["type"]) and b["constructor"] == obj["record"]["constructor"],
             "construct.value", "materialized a different nominal constructor")
        need(len(b["fields"]) == len(obj["record"]["field_sources"]), "construct.value", "field arity differs")
        if "materialize_fields" in obj:
            need(same(b["fields"], obj["materialize_fields"]), "construct.value", "BuildDone value differs from actual completed field vector")
        obj["record"]["field_values"] = b["fields"]
        self.out["constructors"]["materialize_order"].append(obj["record"])
        self.ledger.append({"association": "construction", "invocation": obj["record"]["invocation"],
                            "expr_id": obj["expr"], "open_record": obj["opened"], "complete_record": record,
                            "raw_ref": tag(after["control"], "Return", "construct.return")["value"]})

    def pattern(self, ident, value_ref, image, path=(), seen=()):
        need(ident not in seen, "pattern", "cyclic pattern")
        kind, p = variant(at(self.p["patterns"], ident, "pattern"), "pattern")
        value = self.value(value_ref, image)
        if kind == "PVar":
            return [{"name": self.name(p["x"]), "pattern_path": list(path), "value": value}]
        if kind == "Wildcard":
            return []
        need(kind == "Constructor", "pattern", "unsupported dedicated pattern representation " + kind)
        ty, name, _ = self.ctor(p["type_id"], p["ctor_id"])
        actual = tag(value, "Nominal", "selected.pattern.value")
        need(same(actual["type"], ty) and actual["constructor"] == name,
             "pattern", "actual selected constructor pattern does not fit scrutinee")
        children = cut(self.p["pattern_children"], p["children"], "pattern_children")
        refkind, rid = variant(value_ref, "pattern.ref")
        _, rawvalue = variant(at(self.p["constants"] if refkind == "Const" else image["data"], rid, "pattern.value"), "pattern.value")
        refs = cut(self.p["constant_fields"] if refkind == "Const" else image["data_fields"], rawvalue["fields"], "pattern.fields")
        need(len(children) == len(refs), "pattern", "selected field membership differs")
        result = []
        for i, (child, ref) in enumerate(zip(children, refs)):
            result += self.pattern(child, {"Const": ref} if refkind == "Const" else ref, image, path + (i,), seen + (ident,))
        return result

    def step(self, before, event, after, record):
        self.image(before)
        self.image(after)
        expr = event["source_expr_id"]
        need(expr == before["control_origin"], "event.source_expr_id", "event must retain pre-image origin")
        coord = self.coord(expr)
        occ = tag(event["source_occurrence"], "Occurrence", "event.source_occurrence")
        raw_occ = at(self.p["occurrences"], expr, "occurrence")
        need(occ["owner_is_function"] is raw_occ["owner_is_function"] and occ["function"] == raw_occ["function"]
             and same(occ["ast_path"], coord["path"]), "event.source_occurrence", "event/source path mismatch")
        rule = event["rule_id"]
        disposition = event["disposition"]
        need(not self.published, "event", "event after the single successful publication")
        if disposition != "Applied":
            need(rule == "KAdminPublishValue" and disposition == "Published" and event["event_kind"] == "Publish",
                 "event", "capture did not complete successfully")
            need(not self.calls and not self.live_ctor and self.materializing is None
                 and self.enter is None and self.plan is None,
                 "final", "publication with pending source continuation")
            self.published = True
            return
        need(event["event_kind"] in ("LegacyComplete", "HelperEnter", "HelperVisit", "HelperExit", "HelperFail", "Bookkeeping"),
             "event.event_kind", "unexpected applied event class")
        if rule.startswith("KMachine") or (rule.startswith("XAdt") and not rule.startswith("XAdtHelper")):
            need(event["event_kind"] == "LegacyComplete", "event.event_kind", "source transition not LegacyComplete")
        if rule == "KMachineEnterBind":
            ent = tag(before["control"], "Enter", "call.enter")
            need(ent["function"] is not None and self.enter is None, "call.enter", "missing/overlapping successful entry candidate")
            bound = tag(after["control"], "Bound", "call.bound")
            env = tag(bound["binding"], "Ok", "call.binding")["env"]
            f = at(self.p["functions"], ent["function"], "call.function")
            call = tag(at(self.p["expressions"], expr, "call.expr"), "Call", "call.expr")
            need(call["function_name"] == f["function_name"] and bound["body"] == f["body"] and bound["return_type"] == f["return_type"],
                 "call", "callee does not match original source Call and signature")
            args = self.vals(ent["args"], before)
            params = cut(self.p["parameters"], f["params"], "call.parameters")
            need(len(params) == len(args), "call.parameters", "argument arity differs")
            names = [self.name(p["binder"]) for p in params]
            need(len(set(names)) == len(names), "call.parameters", "duplicate parameter names")
            need(all(same(self.value_type(v), self.typ(p["sort"])) for p, v in zip(params, args)),
                 "call.parameters", "argument type differs")
            callee = self.env(env, after)
            need(same(callee, dict(zip(names, args))), "call.parameters", "callee has missing/extra/wrong parameter bindings")
            self.enter = {"expr": expr, "function": self.name(f["function_name"]), "arguments": args,
                          "caller_environment": self.env(before["env_root"], before), "callee_parameters": callee,
                          "env": env, "body": f["body"], "return_type": f["return_type"], "start": record, "scheduled": False}
        elif rule == "KMachineBoundValid":
            need(self.enter is not None and self.enter["scheduled"] is False, "call.bound", "missing/duplicate bound transition")
            c = self.enter
            bound = tag(before["control"], "Bound", "call.bound")
            schedule = tag(after["control"], "ScheduleNext", "call.schedule")
            need(tag(bound["binding"], "Ok", "call.binding")["env"] == c["env"] and bound["body"] == c["body"]
                 and schedule["next"] == c["body"] and schedule["env"] == c["env"], "call.bound", "bound signature/body/env changed")
            c["scheduled"] = True
        elif rule == "KMachinePushAccept":
            schedule = tag(before["control"], "ScheduleNext", "push")
            framekind, frame = variant(schedule["frame"], "push.frame")
            need(same(after["frames"], before["frames"] + [schedule["frame"]]), "push.frames", "successful push is not exact append")
            if framekind == "ReturnF":
                need(self.enter is not None and self.enter["scheduled"] is True, "call", "ReturnF entry without unique EnterBind and BoundValid")
                c = self.enter
                need(frame["origin"] == c["expr"] == expr and frame["return_type"] == c["return_type"]
                     and schedule["next"] == c["body"] and schedule["env"] == c["env"]
                     and same(self.env(frame["saved_env"], before), c["caller_environment"])
                     and same(self.env(after["env_root"], after), c["callee_parameters"]),
                     "call", "scheduled body/caller/callee identity mismatch")
                invocation = len(self.out["calls"])
                item = {k: c[k] for k in ("function", "arguments", "caller_environment", "callee_parameters")}
                item.update(invocation=invocation, call_site=self.coord(c["expr"]))
                self.out["calls"].append(item)
                self.calls.append({"invocation": invocation, "depth": len(before["frames"]), "expr": expr})
                self.out["pending_checkpoints"].append({"at_call": invocation, "milestone": "function_entry_after_all_arguments",
                                                       "pending": self.pending(after), "caller_environment": c["caller_environment"],
                                                       "callee_parameters": c["callee_parameters"]})
                self.ledger.append({"association": "call", "invocation": invocation, "entry_bind_record": c["start"],
                                    "body_entry_record": record, "expr_id": expr, "return_frame_depth": len(before["frames"])})
                self.enter = None
                # Raw caller/callee roots and a direct handle stay with this call row.
                self.ledger[-1].update(caller_env_root=frame["saved_env"], callee_env_root=c["env"])
                self.calls[-1]["row"] = self.ledger[-1]
        elif rule == "KMachineReturnCallValid":
            need(self.calls and self.calls[-1]["depth"] == len(before["frames"]) - 1,
                 "call.return", "return not paired to current semantic call")
            frame = tag(before["frames"][-1], "ReturnF", "call.return")
            need(frame["origin"] == self.calls[-1]["expr"] and same(after["frames"], before["frames"][:-1]),
                 "call.return", "return frame identity changed")
            # The direct row handle avoids a ledger rescan on every return.
            self.calls.pop()["row"]["return_record"] = record
        elif rule in ("XAdtEvalConstructFields", "XAdtEvalConstructNullary"):
            need(tag(before["control"], "Eval", "construct.open")["expr"] == expr, "construct.open", "origin not source construction")
            obj = self._open(expr, before, record)
            if rule.endswith("Nullary"):
                need(not obj["record"]["field_sources"], "construct", "nullary fields not empty")
                self._complete(obj, after, record)
            else:
                depth = len(before["frames"])
                need(depth not in self.live_ctor and obj["record"]["field_sources"], "construct", "overlapping constructor frame")
                schedule = tag(after["control"], "ScheduleNext", "construct.schedule")
                frame = tag(schedule["frame"], "ConstructorFieldsF", "construct.frame")
                source_fields = self.children(self.p["expressions"][expr])
                need(frame["index"] == 0 and frame["checked"] is False and frame["completed"]["len"] == 0 and frame["origin"] == expr
                     and frame["type_id"] == obj["tid"] and frame["ctor_id"] == obj["cid"]
                     and schedule["next"] == source_fields[0]
                     and same(cut(self.p["children"], frame["rest"], "construct.rest"), source_fields[1:])
                     and same(self.env(frame["saved_env"], after), obj["record"]["saved_environment"]),
                     "construct.frame", "initial field lifecycle differs")
                self.live_ctor[depth] = obj
        elif rule == "XAdtReturnCtorFieldsFinal":
            depth = len(before["frames"]) - 1
            need(depth in self.live_ctor and self.materializing is None, "construct.final", "unpaired constructor final")
            self.materializing = self.live_ctor.pop(depth)
            mat = tag(after["control"], "Materialize", "construct.materialize")
            frame = tag(before["frames"][-1], "ConstructorFieldsF", "construct.final.frame")
            need(frame["origin"] == self.materializing["expr"] and frame["checked"] is True
                 and frame["index"] == len(self.materializing["record"]["field_sources"]) - 1
                 and frame["rest"]["len"] == 0 and same(after["frames"], before["frames"][:-1]),
                 "construct.final", "final field/frame identity differs")
            need(mat["type_id"] == self.materializing["tid"] and mat["ctor_id"] == self.materializing["cid"],
                 "construct.final", "materialization type/constructor differs")
            appended = tag(before["result"], "AppendDone", "construct.final.result")["seq"]
            need(same(mat["fields"], appended), "construct.final", "materialization not actual completed vector")
            self.materializing["materialize_fields"] = self.vals(mat["fields"], after)
        elif rule == "XAdtMaterializeDone":
            need(self.materializing is not None, "construct.complete", "materialization without opening")
            self._complete(self.materializing, after, record)
            self.materializing = None
        elif rule == "XAdtHelperPlanEntry":
            need(self.plan is None, "match.plan", "overlapping plan lifecycle")
            cases = tag(before["control"], "Cases", "match.entry")
            self.plan = {"expr": expr, "binds": [], "start": record, "scrutinee": cases["value"], "saved_env": cases["saved_env"]}
        elif rule == "XAdtHelperPlanBind":
            need(self.plan is not None and self.plan["expr"] == expr, "match.bind", "bind outside plan lifecycle")
            h = tag(before["helper"], "Plan", "match.helper")
            node = tag(at(self.p["plan_nodes"], h["node"], "plan_nodes"), "Bind", "plan.bind")
            need(node["source_match"] == expr and h["source_match"] == expr, "match.bind", "different source Match")
            self.plan["binds"].append({"name": self.name(node["binder"]),
                                      "pattern_path": cut(self.p["field_paths"], node["path"], "field_paths"),
                                      "value": self.value(h["cur"], before)})
        elif rule == "XAdtCasesPlanHit":
            need(self.plan is not None and self.plan["expr"] == expr, "match", "hit without one plan lifecycle")
            m = tag(at(self.p["expressions"], expr, "match.expression"), "Match", "match.expression")
            result = tag(before["result"], "PlanMatched", "match.result")
            arm = index(result["arm"], "match.arm")
            ordinal = arm - m["arms"]["start"]
            need(0 <= ordinal < m["arms"]["len"], "match.arm", "winner outside original source arm slice")
            a = at(self.p["arms"], arm, "match.arm")
            control = tag(before["control"], "Cases", "match.control")
            need(same(control["value"], self.plan["scrutinee"]) and control["saved_env"] == self.plan["saved_env"],
                 "match", "plan changed original scrutinee/environment")
            bindings = self.pattern(a["pattern"], control["value"], before)
            need(same(bindings, self.plan["binds"]), "match.bindings", "actual ordered plan binds differ from selected source-pattern preorder")
            names = [x["name"] for x in bindings]
            need(len(names) == len(set(names)), "match.bindings", "duplicate source binders")
            env = self.env(control["saved_env"], before)
            env.update({x["name"]: x["value"] for x in bindings})
            need(same(env, self.env(result["env"], before)), "match.bindings", "selected environment has missing/extra/wrong names")
            schedule = tag(after["control"], "ScheduleNext", "match.selected_body")
            need(schedule["next"] == a["body"] and schedule["env"] == result["env"], "match.selected_body", "winner body/environment mismatch")
            selected = self.coord(a["body"])
            need(same(selected, {"owner": coord["owner"], "path": coord["path"] + [ordinal + 1]}),
                 "match.selected_body", "source arm body association differs")
            item = {"source": coord, "invocation": len(self.out["matches"]), "selected_arm": ordinal,
                    "scrutinee": self.value(control["value"], before), "bindings": bindings, "selected_body": selected}
            self.out["matches"].append(item)
            self.ledger.append({"association": "match", "invocation": item["invocation"], "expr_id": expr,
                                "global_arm": arm, "plan_entry_record": self.plan["start"], "hit_record": record})
            self.plan = None
            self.ledger[-1].update(scrutinee_ref=control["value"], saved_env_root=control["saved_env"], arm_env_root=result["env"])
        elif rule in ("KMachineCasesHit", "KMachineCasesEmpty", "XAdtCasesPlanMiss"):
            raise Gap("match", "unbound or unsuccessful match lifecycle: " + rule)

    def finish(self, image):
        self.image(image)
        need(not self.calls and not self.live_ctor and self.enter is None and self.materializing is None and self.plan is None,
             "final", "unconsumed semantic lifecycle")
        need(self.published, "final", "no single successful KAdminPublishValue publication")
        need(image["frames"] == [] and image["helper"] is None and image["result"] is None,
             "final", "pending runtime continuation")
        value = tag(image["outcome"], "Halted", "final.outcome")["value"]
        need(same(tag(image["control"], "Return", "final.control")["value"], value), "final", "Halted ref differs from Return")
        self.out["final_value"] = self.value(value, image)
        return self.out


def jsonl(raw, name):
    """Parse one LF-terminated JSON value at a time from the one checked buffer."""
    need(raw == b"" or raw.endswith(b"\n"), name, "unterminated final JSONL line")
    start, n = 0, 0
    while start < len(raw):
        end = raw.index(b"\n", start)
        yield n, parse(raw[start:end], name + ":" + str(n))
        start, n = end + 1, n + 1


def capture_case(binding):
    """Retain every diagnostic; never treat missing/failed capture as success."""
    failures, ledger, projection, errors, pins, where = [], [], None, [], [], "capture"
    try:
        files = binding["capture"]
        keys(files, ("input.json", "compile.json", "records.jsonl", "errors.jsonl", "final.json"), "capture")
        parents = {str(Path(x["path"]).parent) for x in files.values()}
        need(len(parents) == 1 and Path(next(iter(parents))).name == binding["capture_case"],
             "capture", "all five files must be in the selected case directory")
        for name, spec in files.items():
            need(Path(spec["path"]).name == name and not Path(spec["path"]).is_symlink(),
                 "capture", "literal named capture file required; no alias to expected")
        raw = {}
        for name, spec in list(files.items()) + [("source", binding["source"])]:
            # Only accepted single-buffer pins are retained and later rechecked.
            where = name
            raw[name] = checked(spec)
            pins.append(spec)
        source = raw.pop("source").decode("utf-8")
        where = "errors.jsonl"
        for _, error in jsonl(raw["errors.jsonl"], "errors.jsonl"):
            errors.append(error)
        need(not errors, "errors.jsonl", "producer diagnostics retained; complete success not established")
        where = "documents"
        inp, comp, final = [parse(raw[n], n) for n in ("input.json", "compile.json", "final.json")]
        for doc in (inp, comp, final):
            need(doc["schema"] == ACTUAL_SCHEMA and doc["case"] == binding["capture_case"], "capture", "schema/case mismatch")
        need(inp["source"] == source, "input.source", "original UTF-8 source bytes differ")
        need(inp["independent_expected_cells_read"] is False and final["independent_expected_cells_read"] is False,
             "capture", "producer declared independent expected read")
        need(inp["source_profile"] == comp["source_profile"] == binding["source_profile"] == PROFILE, "source_profile", "not selected adopted profile")
        need(comp["body_check"]["status"] == "completed" and comp["evaluation_performed"] is False,
             "compile", "body completion/no-evaluation contract missing")
        need(same(comp["body_check"]["judgments"], JUDGMENTS), "compile.body_check", "not exact adopted body-check judgments")
        need(same(inp["configuration"], comp["configuration"]), "configuration", "configuration changed")
        genesis = keys(comp["genesis"], ("admission", "image"), "genesis")
        admission = genesis["admission"]
        keys(admission, ("identity", "bounds", "view", "layout"), "admission")
        where = "projection"
        p = Projection(admission["view"], comp["provenance"], source)
        projection, ledger = p.out, p.ledger
        previous, events, terminals, count = {"admission": admission, "image": genesis["image"]}, 0, 0, 0

        def boundary(bound):
            keys(bound, ("admission", "image"), "boundary")
            a = bound["admission"]
            if not same(a, {"same_as": "compile.json#/genesis/admission"}):
                keys(a, ("inline",), "boundary.admission")
                need(same(a["inline"], admission), "boundary.admission", "inline admission changed")
            # Reference and equality-validated inline share ONE immutable admission
            # object, so continuity short-circuits instead of re-walking Program.
            return {"admission": admission, "image": bound["image"]}

        where = "records.jsonl"
        for n, rec in jsonl(raw["records.jsonl"], "records.jsonl"):
            # Only the current parsed record is live; the file is never listed.
            where = "records.jsonl:" + str(n)
            need(type(rec) is dict and rec.get("kind") in ("Event", "Terminal"), where, "Event or Terminal record required")
            keys(rec, ("record", "kind", "before", "event", "after") if rec["kind"] == "Event"
                 else ("record", "kind", "before", "after"), where)
            need(type(rec["record"]) is int and rec["record"] == n and terminals == 0,
                 where, "noncontiguous index or record after terminal")
            before, after = boundary(rec["before"]), boundary(rec["after"])
            need(same(before, previous), where + ".before", "whole-boundary discontinuity")
            if rec["kind"] == "Event":
                p.step(before["image"], rec["event"], after["image"], n)
                events += 1
            else:
                need(same(before, after), where, "missing/changed terminal boundary")
                terminals += 1
            previous, count = after, n + 1
        where = "final.json"
        need(terminals == 1 and count == index(final["records"], "final.records") and events == index(final["events"], "final.events"), "final.counts", "complete record/event membership differs")
        need(same(previous, final["owner"]), "final.owner", "last complete boundary differs")
        need(same(comp["preparation"], final["preparation"]), "preparation", "preparation changed")
        keys(final["actual_capture_checks"], ("continuous", "final_equals_last", "admission_unchanged", "preparation_unchanged"), "capture_checks")
        need(all(x is True for x in final["actual_capture_checks"].values()), "capture_checks", "producer reported incomplete capture")
        projection = p.finish(final["owner"]["image"])
    except (Gap, KeyError, IndexError, TypeError, RecursionError, AttributeError, ValueError, OSError) as exc:
        # Located per-case receipt; already retained pins/diagnostics/ledger stay.
        failures.append({"path": getattr(exc, "path", where), "reason": getattr(exc, "reason", repr(exc))})
    return {"status": "ESTABLISHED" if not failures else "NOT_ESTABLISHED", "case": binding["case"],
            "projection": projection, "gaps": failures, "producer_errors": errors,
            "association_ledger": ledger, "input_pins": pins}


def first_difference(actual, expected, path="$"):
    if type(actual) is not type(expected):
        return {"path": path, "reason": "JSON type differs", "actual": actual, "expected": expected}
    if type(actual) is dict:
        if set(actual) != set(expected):
            return {"path": path, "reason": "object membership differs", "actual": sorted(actual), "expected": sorted(expected)}
        for key in sorted(actual):
            diff = first_difference(actual[key], expected[key], path + "/" + key.replace("~", "~0").replace("/", "~1"))
            if diff:
                return diff
    elif type(actual) is list:
        if len(actual) != len(expected):
            return {"path": path, "reason": "array length differs", "actual": len(actual), "expected": len(expected)}
        for i, (a, e) in enumerate(zip(actual, expected)):
            diff = first_difference(a, e, path + "/" + str(i))
            if diff:
                return diff
    elif actual != expected:
        return {"path": path, "reason": "value differs", "actual": actual, "expected": expected}
    return None


def compare_case(actual, expected):
    keys(expected, (*SEMANTIC, "limitations"), "expected.case")
    if actual["status"] != "ESTABLISHED":
        return {"status": "NOT_ESTABLISHED", "gaps": actual["gaps"], "first_divergence": None}
    observed = actual["projection"]
    results, first = {}, None
    for field in SEMANTIC:
        if field == "pending_checkpoints":
            chosen = []
            seen = set()
            for checkpoint in expected[field]:
                ident = index(checkpoint["at_call"], "expected.pending.at_call")
                need(ident not in seen, "expected.pending", "duplicate at_call checkpoint")
                seen.add(ident)
            for checkpoint in expected[field]:
                ident = index(checkpoint["at_call"], "expected.pending.at_call")
                matches = [x for x in observed[field] if x["at_call"] == ident]
                if len(matches) != 1:
                    diff = {"path": "$/pending_checkpoints", "reason": "expected milestone absent/ambiguous", "actual": len(matches), "expected": 1}
                    break
                chosen.append(matches[0])
            else:
                diff = first_difference(chosen, expected[field], "$/" + field)
        else:
            diff = first_difference(observed[field], expected[field], "$/" + field)
        results[field] = "PASS" if diff is None else "FAIL"
        if first is None and diff is not None:
            # bounded excerpts only; full immutable input documents are pinned.
            first = {k: (json.dumps(v, ensure_ascii=False)[:512] if k in ("actual", "expected") else v) for k, v in diff.items()}
    return {"status": "PASS" if first is None else "FAIL", "fields": results, "first_divergence": first}


def project_command(args):
    binding_pin, binding_raw = pin_raw(args.bindings)
    bindings = parse(binding_raw, binding_pin["path"])
    keys(bindings, ("source_manifest", "cases"), "bindings")
    need(bindings["source_manifest"]["sha256"] == INPUT_SHA, "bindings", "not original independent input manifest")
    manifest = json_pin(bindings["source_manifest"])
    originals = {x["path"]: x for x in manifest["source_and_safe_readset"] if x["path"].endswith(".mori")}
    selected = bindings["cases"]
    need(len(selected) == len(originals) and len({x["case"] for x in selected}) == len(selected), "bindings", "whole unique source case membership required")
    for item in selected:
        keys(item, ("case", "capture_case", "source_profile", "source", "capture"), "bindings.cases")
        need(item["source"] == originals.get(item["source"]["path"]) and Path(item["source"]["path"]).name == item["case"],
             "bindings", "source not member of original safe manifest")
        need(item["capture_case"] == Path(item["case"]).stem, "bindings", "capture case not original input basename")
    out = Path(args.outdir)
    out.mkdir(exist_ok=False)
    args.created_out = True
    results = []
    for item in selected:
        results.append(capture_case(item))
    projected = {"schema": "nominal-actual-symbolic-projection/1", "cases": {r["case"]: r for r in results},
                 "expected_read": False, "scope": LIMITS}
    output = save(out, "ACTUAL-PROJECTION.json", projected)
    all_inputs = [binding_pin, bindings["source_manifest"], pin(__file__)] + [p for r in results for p in r["input_pins"]]
    for spec in all_inputs:
        checked(spec)
    receipt = save(out, "PHASE-A-RECEIPT.json", {"schema": "nominal-passive-phase-a-receipt/1", "expected_read": False,
                   "inputs": all_inputs, "outputs": [output], "case_status": {r["case"]: r["status"] for r in results},
                   "next": "Root durably freezes these exact files before selecting compare; no expected input was opened"})
    sync_dir(out)
    print(json.dumps({"projection": output, "receipt": receipt, "expected_read": False}))
    return 0 if all(r["status"] == "ESTABLISHED" for r in results) else 2


def compare_command(args):
    # Root's data-only freeze is supplied externally; this program never makes
    # a freeze for itself. Check ALL actual/projection bytes before unblinding.
    freeze_pin = pin(args.frozen)
    frozen = json_pin(freeze_pin)
    keys(frozen, ("schema", "actual_and_projection_frozen_before_expected_read", "phase_a_receipt", "projection", "safe_bindings"), "Root.freeze")
    need(frozen["schema"] == "root-nominal-actual-projection-freeze/1" and frozen["actual_and_projection_frozen_before_expected_read"] is True,
         "Root.freeze", "Root pre-unblinding freeze required")
    receipt = json_pin(frozen["phase_a_receipt"])
    need(receipt["expected_read"] is False and frozen["projection"] in receipt["outputs"], "Root.freeze", "projection not original phase-A output")
    for spec in receipt["inputs"] + receipt["outputs"]:
        checked(spec)
    need(frozen["safe_bindings"]["sha256"] == SAFE_SHA, "Root.freeze", "not original oracle-safe binding")
    safe = json_pin(frozen["safe_bindings"])
    projected = json_pin(frozen["projection"])
    need(projected["expected_read"] is False, "projection", "phase-A isolation false")
    # The first populated expected read occurs on THIS line, after full checks.
    expected = json_pin(safe["private_expected"])
    keys(expected, ("schema", "source_pins", "normative_pins", "author", "scope", "cases"), "expected")
    need(expected["schema"] == SCHEMA and set(expected["cases"]) == set(projected["cases"]) == set(safe["case_keys"]),
         "expected", "schema or whole case membership differs")
    result = {"schema": "nominal-symbolic-exact-comparison/1", "cases": {}, "scope": LIMITS,
              "root_freeze": freeze_pin, "expected": safe["private_expected"], "expected_provenance_compared_as_semantics": False}
    for case in sorted(expected["cases"]):
        result["cases"][case] = compare_case(projected["cases"][case], expected["cases"][case])
    for spec in receipt["inputs"] + receipt["outputs"] + [freeze_pin, frozen["phase_a_receipt"], frozen["safe_bindings"], safe["private_expected"]]:
        checked(spec)
    out = Path(args.outdir)
    out.mkdir(exist_ok=False)
    args.created_out = True
    saved = save(out, "COMPARISON.json", result)
    sync_dir(out)
    print(json.dumps(saved))
    return 0 if all(x["status"] == "PASS" for x in result["cases"].values()) else 1


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest="command", required=True)
    a = sub.add_parser("project")
    a.add_argument("--bindings", required=True)
    a.add_argument("--outdir", required=True)
    b = sub.add_parser("compare")
    b.add_argument("--frozen", required=True)
    b.add_argument("--outdir", required=True)
    args = cli.parse_args()
    try:
        return project_command(args) if args.command == "project" else compare_command(args)
    except (Gap, OSError, KeyError, IndexError, TypeError, UnicodeError, ValueError, AttributeError, RecursionError) as exc:
        # Preserve failure under the newly selected output directory where
        # possible. Never overwrite input, projection, expected or old result.
        directory = Path(args.outdir)
        try:
            if not getattr(args, "created_out", False):
                directory.mkdir(exist_ok=False)
            failure = save(directory, "FAILURE.json", {"status": "NOT_ESTABLISHED", "path": getattr(exc, "path", "input"),
                           "reason": getattr(exc, "reason", repr(exc)), "command": args.command})
            sync_dir(directory)
            print(json.dumps(failure))
        except OSError:
            print("failure receipt could not be created exclusively", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
