"""Finite positive DATA conversion for two already captured nominal observations.

Source-only candidate. No compiler, evaluator, admission, model or backend call.
Unsupported shapes raise ordinary lookup/unpack errors; those are not Core
refusals, hostile-input policy or a new validated public import format.

All paths are explicit CLI inputs. Original observation bytes are retained.
The reviewed literal lexer/reader/emitter/exact functions are copied unchanged.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re

# Same literal helper bytes and loading pattern as the reviewed quote adapter.
HELPER_PATH = Path(__file__).with_name("positive_literal_helpers.py")
helper_namespace = {"re": re, "json": json}
exec(compile(HELPER_PATH.read_text(), "unchanged-reviewed-positive-literal-helpers", "exec"), helper_namespace)
lex, Reader, literal, exact = (helper_namespace[k] for k in ("lex", "Reader", "literal", "exact"))

CAPTURES = {
    "map": (39189, "03395b13df1e294d74b48c1b527b77fbfa2702b4c325813c5e84f077835cd330", "q11ActualMapQueueReturned"),
    "mirror": (44552, "d2a59a9d4d4fc377bba99954d1f62a8ee8cfd61c3a700620a09f531ffc2b73f6", "q11ActualMirrorArborReturned"),
}
IMPORTS = ("c6PositiveHostTypes", "c6PositiveMetaTypes", "c6PositiveProgramTypes", "c6PositiveContextTypes", "c6PositiveStateTypes")
DECIMAL_IMAGE_FIELDS = ("control_origin", "stack_limit", "initial_fuel", "remaining_work", "consumed_work", "helper_work", "helper_visits", "reference_cursor")
BUILTINS = {"UInt": 0, "Bool": 1, "List": 2, "Tree": 3}


def variant(tag, value=None, has=False):
    return ("variant", tag, has, value)


def nul(tag):
    return variant(tag)


def val(tag, value):
    return variant(tag, value, True)


def numbers(row):
    return {k: int(v) for k, v in row.items()}


def decimals(row):
    return {k: str(v) for k, v in row.items()}


def utf8(text):
    return list(text.encode("utf-8"))


def unutf8(data):
    return bytes(data).decode("utf-8")


def pair(data):
    return {"first": utf8(data[0]), "second": utf8(data[1])}


def unpair(data):
    return [unutf8(data["first"]), unutf8(data["second"])]


def optpair(data):
    return nul("C6MNoPair") if data is None else val("C6MSomePair", pair(data))


def unoptpair(data):
    return None if not data[2] else unpair(data[3])


def typeid(data):
    return BUILTINS[data] if type(data) is str else int(data["TypeId"])


def untypeid(data):
    names = {v: k for k, v in BUILTINS.items()}
    return names[data] if data in names else {"TypeId": str(data)}


def option(data):
    return nul("C6PNone") if data is None else val("C6PSome", int(data))


def unoption(data):
    return None if not data[2] else str(data[3])


def ref(data):
    tag, index = next(iter(data.items()))
    return val({"Const": "C6Const", "Data": "C6Data"}[tag], int(index))


def unref(data):
    return {{"C6Const": "Const", "C6Data": "Data"}[data[1]]: str(data[3])}


def env(data):
    return nul("C6EnvNone") if data is None else val("C6EnvSome", int(data))


def unenv(data):
    return None if not data[2] else str(data[3])


def identity(data):
    out = copy.deepcopy(data)
    out["grammar_domain"] = nul({"GenericNominalExtensionReserved": "C6MGenericNominalExtensionReserved"}[data["grammar_domain"]])
    out["finance"] = nul({"Absent": "C6MFinanceAbsent"}[data["finance"]])
    out["layout_version"] = pair(data["layout_version"])
    out["field_id"] = optpair(data["field_id"])
    for key in ("layout_digest", "bounds_digest", "program_membership_digest"):
        out[key] = nul({"Unselected": "C6MUnselected"}[data[key]])
    return out


def unidentity(data):
    out = copy.deepcopy(data)
    out["grammar_domain"] = {"C6MGenericNominalExtensionReserved": "GenericNominalExtensionReserved"}[data["grammar_domain"][1]]
    out["finance"] = {"C6MFinanceAbsent": "Absent"}[data["finance"][1]]
    out["layout_version"] = unpair(data["layout_version"])
    out["field_id"] = unoptpair(data["field_id"])
    for key in ("layout_digest", "bounds_digest", "program_membership_digest"):
        out[key] = {"C6MUnselected": "Unselected"}[data[key][1]]
    return out


def layout(data):
    out = copy.deepcopy(data)
    out["layout_version"] = pair(data["layout_version"])
    out["bounds"] = numbers(data["bounds"])
    out["capacities"] = numbers(data["capacities"])
    for key in ("field_id", "scalar_layout_id"):
        out[key] = optpair(data[key])
    out["widths"] = {None: nul("C6MNoWidths")}[data["widths"]]
    return out


def unlayout(data):
    out = copy.deepcopy(data)
    out["layout_version"] = unpair(data["layout_version"])
    out["bounds"] = decimals(data["bounds"])
    out["capacities"] = decimals(data["capacities"])
    for key in ("field_id", "scalar_layout_id"):
        out[key] = unoptpair(data[key])
    out["widths"] = {"C6MNoWidths": None}[data["widths"][1]]
    return out


def type_tokens(data):
    if type(data) is str:
        return [nul({"UInt": "C6PTUInt", "Bool": "C6PTBool", "List": "C6PTList", "Tree": "C6PTTree"}[data])]
    tag, payload = next(iter(data.items()))
    if tag in ("Named", "Param"):
        return [val({"Named": "C6PTNamed", "Param": "C6PTParam"}[tag], {"name": utf8(payload["name"])})]
    args = payload["args"]
    first = val({"Apply": "C6PTApply"}[tag], {"name": utf8(payload["name"]), "arity": len(args)})
    return [first] + [token for child in args for token in type_tokens(child)]


def untype_tokens(tokens):
    # Inverse of the selected positive prefix tree vocabulary, not a new parser.
    def one(cursor):
        token = tokens[cursor]
        name = token[1]
        if not token[2]:
            return {"C6PTUInt": "UInt", "C6PTBool": "Bool", "C6PTList": "List", "C6PTTree": "Tree"}[name], cursor + 1
        payload = token[3]
        if name in ("C6PTNamed", "C6PTParam"):
            return {{"C6PTNamed": "Named", "C6PTParam": "Param"}[name]: {"name": unutf8(payload["name"])}}, cursor + 1
        args = []
        cursor += 1
        for _ in range(payload["arity"]):
            child, cursor = one(cursor)
            args.append(child)
        return {{"C6PTApply": "Apply"}[name]: {"name": unutf8(payload["name"]), "args": args}}, cursor
    result, end = one(0)
    assert end == len(tokens), "positive type-expression inverse is incomplete"
    return result


def declaration(data):
    return {"name": utf8(data["name"]), "parameters": [utf8(x) for x in data["parameters"]],
            "ctors": [{"name": utf8(c["name"]), "fields": [{"name": utf8(f["name"]), "ty": type_tokens(f["ty"])} for f in c["fields"]]} for c in data["ctors"]]}


def undeclaration(data):
    return {"name": unutf8(data["name"]), "parameters": [unutf8(x) for x in data["parameters"]],
            "ctors": [{"name": unutf8(c["name"]), "fields": [{"name": unutf8(f["name"]), "ty": untype_tokens(f["ty"])} for f in c["fields"]]} for c in data["ctors"]]}


EXPR_TAGS = {"Literal": "C6PELiteral", "Var": "C6PEVar", "Call": "C6PECall", "Binary": "C6PEBinary", "Unary": "C6PEUnary", "Construct": "C6PEConstruct", "Match": "C6PEMatch"}
BINARY_TAGS = {"Add": "C6PAdd"}
UNARY_TAGS = {"Not": "C6PNot"}


def expression(data):
    tag, payload = next(iter(data.items()))
    if tag in ("Literal", "Var"):
        out = numbers(payload)
    elif tag == "Call":
        out = {"function_name": int(payload["function_name"]), "args": numbers(payload["args"])}
    elif tag == "Binary":
        out = {"op": nul(BINARY_TAGS[payload["op"]]), "left": int(payload["left"]), "right": int(payload["right"])}
    elif tag == "Unary":
        out = {"op": nul(UNARY_TAGS[payload["op"]]), "operand": int(payload["operand"])}
    elif tag == "Construct":
        out = {"type_id": typeid(payload["type_id"]), "ctor_id": int(payload["ctor_id"]), "fields": numbers(payload["fields"]), "constant": option(payload["constant"])}
    else:
        out = {"scrutinee": int(payload["scrutinee"]), "arms": numbers(payload["arms"]), "plan": option(payload["plan"])}
    return val(EXPR_TAGS[tag], out)


def unexpression(data):
    tag = {v: k for k, v in EXPR_TAGS.items()}[data[1]]
    payload = data[3]
    if tag in ("Literal", "Var"):
        out = decimals(payload)
    elif tag == "Call":
        out = {"function_name": str(payload["function_name"]), "args": decimals(payload["args"])}
    elif tag == "Binary":
        out = {"op": {v: k for k, v in BINARY_TAGS.items()}[payload["op"][1]], "left": str(payload["left"]), "right": str(payload["right"])}
    elif tag == "Unary":
        out = {"op": {v: k for k, v in UNARY_TAGS.items()}[payload["op"][1]], "operand": str(payload["operand"])}
    elif tag == "Construct":
        out = {"type_id": untypeid(payload["type_id"]), "ctor_id": str(payload["ctor_id"]), "fields": decimals(payload["fields"]), "constant": unoption(payload["constant"])}
    else:
        out = {"scrutinee": str(payload["scrutinee"]), "arms": decimals(payload["arms"]), "plan": unoption(payload["plan"])}
    return {tag: out}


def constant(data):
    tag, payload = next(iter(data.items()))
    if tag == "U128":
        return val("C6PKUInt", {"marker_free": payload["marker_free"], "n": int(payload["n"])})
    if tag == "Bool":
        return val("C6PKBool", copy.deepcopy(payload))
    return val({"Constructor": "C6PKConstructor"}[tag], {"marker_free": payload["marker_free"], "type_id": typeid(payload["type_id"]), "ctor_id": int(payload["ctor_id"]), "fields": numbers(payload["fields"])})


def unconstant(data):
    payload = data[3]
    if data[1] == "C6PKUInt":
        return {"U128": {"marker_free": payload["marker_free"], "n": str(payload["n"])}}
    if data[1] == "C6PKBool":
        return {"Bool": copy.deepcopy(payload)}
    return {{"C6PKConstructor": "Constructor"}[data[1]]: {"marker_free": payload["marker_free"], "type_id": untypeid(payload["type_id"]), "ctor_id": str(payload["ctor_id"]), "fields": decimals(payload["fields"])}}


def pattern(data):
    tag, payload = next(iter(data.items()))
    if tag == "PVar":
        return val("C6PPVar", numbers(payload))
    return val({"Constructor": "C6PPConstructor"}[tag], {"type_id": typeid(payload["type_id"]), "ctor_id": int(payload["ctor_id"]), "children": numbers(payload["children"])})


def unpattern(data):
    payload = data[3]
    if data[1] == "C6PPVar":
        return {"PVar": decimals(payload)}
    return {{"C6PPConstructor": "Constructor"}[data[1]]: {"type_id": untypeid(payload["type_id"]), "ctor_id": str(payload["ctor_id"]), "children": decimals(payload["children"])}}


PLAN_TAGS = {"Switch": "C6PSwitch", "Bind": "C6PBind", "Select": "C6PSelect", "NoMatch": "C6PNoMatch"}


def plan(data):
    tag, payload = next(iter(data.items()))
    out = {}
    for key, item in payload.items():
        out[key] = numbers(item) if key in ("path", "cases") else typeid(item) if key == "type_id" else int(item)
    return val(PLAN_TAGS[tag], out)


def unplan(data):
    out = {}
    for key, item in data[3].items():
        out[key] = decimals(item) if key in ("path", "cases") else untypeid(item) if key == "type_id" else str(item)
    return {{v: k for k, v in PLAN_TAGS.items()}[data[1]]: out}


def program(data):
    out = copy.deepcopy(data)
    out["entry"] = int(data["entry"])
    for key in ("children", "occurrence_paths", "pattern_children", "field_paths", "constant_fields", "decl_param_counts", "decl_ctor_counts"):
        out[key] = [int(x) for x in data[key]]
    out["expressions"] = [expression(x) for x in data["expressions"]]
    out["constants"] = [constant(x) for x in data["constants"]]
    out["patterns"] = [pattern(x) for x in data["patterns"]]
    out["plan_nodes"] = [plan(x) for x in data["plan_nodes"]]
    out["plan_cases"] = [option(x) for x in data["plan_cases"]]
    out["decl_templates"] = [declaration(x) for x in data["decl_templates"]]
    out["names"] = [utf8(x) for x in data["names"]]
    out["type_args"] = [typeid(x) for x in data["type_args"]]
    out["arms"] = [numbers(x) for x in data["arms"]]
    out["parameters"] = [{"binder": int(x["binder"]), "sort": typeid(x["sort"])} for x in data["parameters"]]
    out["occurrences"] = [{"function": int(x["function"]), "owner_is_function": x["owner_is_function"], "path": numbers(x["path"])} for x in data["occurrences"]]
    out["functions"] = [{"body": int(x["body"]), "function_name": int(x["function_name"]), "params": numbers(x["params"]), "return_type": typeid(x["return_type"])} for x in data["functions"]]
    out["types"] = [{"name_id": int(x["name_id"]), "decl": option(x["decl"]), "args": numbers(x["args"]), "ctor_start": int(x["ctor_start"]), "ctor_len": int(x["ctor_len"])} for x in data["types"]]
    out["constructors"] = [{**numbers({k: v for k, v in x.items() if k != "owning_type"}), "owning_type": typeid(x["owning_type"])} for x in data["constructors"]]
    out["ctor_fields"] = [{"name_id": int(x["name_id"]), "type_id": typeid(x["type_id"])} for x in data["ctor_fields"]]
    return out


def unprogram(data):
    out = copy.deepcopy(data)
    out["entry"] = str(data["entry"])
    for key in ("children", "occurrence_paths", "pattern_children", "field_paths", "constant_fields", "decl_param_counts", "decl_ctor_counts"):
        out[key] = [str(x) for x in data[key]]
    out["expressions"] = [unexpression(x) for x in data["expressions"]]
    out["constants"] = [unconstant(x) for x in data["constants"]]
    out["patterns"] = [unpattern(x) for x in data["patterns"]]
    out["plan_nodes"] = [unplan(x) for x in data["plan_nodes"]]
    out["plan_cases"] = [unoption(x) for x in data["plan_cases"]]
    out["decl_templates"] = [undeclaration(x) for x in data["decl_templates"]]
    out["names"] = [unutf8(x) for x in data["names"]]
    out["type_args"] = [untypeid(x) for x in data["type_args"]]
    out["arms"] = [decimals(x) for x in data["arms"]]
    out["parameters"] = [{"binder": str(x["binder"]), "sort": untypeid(x["sort"])} for x in data["parameters"]]
    out["occurrences"] = [{"function": str(x["function"]), "owner_is_function": x["owner_is_function"], "path": decimals(x["path"])} for x in data["occurrences"]]
    out["functions"] = [{"body": str(x["body"]), "function_name": str(x["function_name"]), "params": decimals(x["params"]), "return_type": untypeid(x["return_type"])} for x in data["functions"]]
    out["types"] = [{"name_id": str(x["name_id"]), "decl": unoption(x["decl"]), "args": decimals(x["args"]), "ctor_start": str(x["ctor_start"]), "ctor_len": str(x["ctor_len"])} for x in data["types"]]
    out["constructors"] = [{**decimals({k: v for k, v in x.items() if k != "owning_type"}), "owning_type": untypeid(x["owning_type"])} for x in data["constructors"]]
    out["ctor_fields"] = [{"name_id": str(x["name_id"]), "type_id": untypeid(x["type_id"])} for x in data["ctor_fields"]]
    return out


def data_row(data):
    tag, payload = next(iter(data.items()))
    if tag == "U128":
        return val("C6HU128", {"n": int(payload["n"])})
    if tag == "Bool":
        return val("C6HBool", copy.deepcopy(payload))
    return val({"Constructor": "C6HConstructor"}[tag], {"type_id": typeid(payload["type_id"]), "ctor_id": int(payload["ctor_id"]), "fields": numbers(payload["fields"])})


def undata_row(data):
    payload = data[3]
    if data[1] == "C6HU128":
        return {"U128": {"n": str(payload["n"])}}
    if data[1] == "C6HBool":
        return {"Bool": copy.deepcopy(payload)}
    return {{"C6HConstructor": "Constructor"}[data[1]]: {"type_id": untypeid(payload["type_id"]), "ctor_id": str(payload["ctor_id"]), "fields": decimals(payload["fields"])}}


def run_id(data):
    return {"identity": identity(data["identity"]), "fuel_input": val("C6MNat", int(data["fuel_input"]["Nat"])), "stack_limit_input": val("C6MNat", int(data["stack_limit_input"]["Nat"]))}


def unrun_id(data):
    return {"identity": unidentity(data["identity"]), "fuel_input": {"Nat": str(data["fuel_input"][3])}, "stack_limit_input": {"Nat": str(data["stack_limit_input"][3])}}


def image(data):
    out = copy.deepcopy(data)
    for key in DECIMAL_IMAGE_FIELDS:
        out[key] = int(data[key])
    out["run_id"] = run_id(data["run_id"])
    out["control"] = val("C6Return", {"value": ref(data["control"]["Return"]["value"])})
    out["outcome"] = val("C6Halted", ref(data["outcome"]["Halted"]["value"]))
    out["env_root"] = env(data["env_root"])
    out["alloc"] = numbers(data["alloc"])
    out["active"] = numbers(data["active"])
    out["seg_base"] = {"active": numbers(data["seg_base"]["active"]), "alloc": numbers(data["seg_base"]["alloc"]), "consumed_work": int(data["seg_base"]["consumed_work"])}
    out["helper"] = {None: nul("C6HNoHelper")}[data["helper"]]
    out["result"] = {None: nul("C6HNoResult")}[data["result"]]
    out["data"] = [data_row(x) for x in data["data"]]
    for key in ("data_fields", "value_slots"):
        out[key] = [ref(x) for x in data[key]]
    out["env"] = [{"binder": int(x["binder"]), "value": ref(x["value"]), "previous": env(x["previous"])} for x in data["env"]]
    # Only the observed terminal empty frames/tasks are covered. Do not claim
    # conversion of nonempty frames/tasks or preserve unconverted dictionaries.
    out["frames"] = {0: []}[len(data["frames"])]
    out["tasks"] = {0: []}[len(data["tasks"])]
    return out


def unimage(data):
    out = copy.deepcopy(data)
    for key in DECIMAL_IMAGE_FIELDS:
        out[key] = str(data[key])
    out["run_id"] = unrun_id(data["run_id"])
    out["control"] = {"Return": {"value": unref(data["control"][3]["value"])}}
    out["outcome"] = {"Halted": {"value": unref(data["outcome"][3])}}
    out["env_root"] = unenv(data["env_root"])
    out["alloc"] = decimals(data["alloc"])
    out["active"] = decimals(data["active"])
    out["seg_base"] = {"active": decimals(data["seg_base"]["active"]), "alloc": decimals(data["seg_base"]["alloc"]), "consumed_work": str(data["seg_base"]["consumed_work"])}
    out["helper"] = {"C6HNoHelper": None}[data["helper"][1]]
    out["result"] = {"C6HNoResult": None}[data["result"][1]]
    out["data"] = [undata_row(x) for x in data["data"]]
    for key in ("data_fields", "value_slots"):
        out[key] = [unref(x) for x in data[key]]
    out["env"] = [{"binder": str(x["binder"]), "value": unref(x["value"]), "previous": unenv(x["previous"])} for x in data["env"]]
    out["frames"] = {0: []}[len(data["frames"])]
    out["tasks"] = {0: []}[len(data["tasks"])]
    return out


def host(observation):
    admission = observation["preparation"]["admission"]
    return {"admission": {"identity": identity(admission["identity"]), "bounds": numbers(admission["bounds"]), "layout": layout(admission["layout"]), "view": program(admission["program_view"])}, "image": image(observation["execution"]["image"])}


def inverse_host(data, status, representation):
    admission = data["admission"]
    return {"admission": {"identity": unidentity(admission["identity"]), "bounds": decimals(admission["bounds"]), "layout": unlayout(admission["layout"]), "program_view": unprogram(admission["view"]), "status": status, "representation": representation}, "image": unimage(data["image"])}


def module_text(name, prefix, supplied_host, root, status, representation):
    imports = "".join('  import ' + module + '.* from "./' + module + '"\n' for module in IMPORTS)
    return ("module " + name + " {\n" + imports + "\n  pure val " + prefix + "ActualReturned:C6WholeHost = " + literal(supplied_host, 2)
            + "\n  pure val " + prefix + "ActualRoot:C6Ref = " + literal(root, 2)
            + "\n  pure val " + prefix + "AdmissionStatus:str = " + literal(status, 2)
            + "\n  pure val " + prefix + "AdmissionRepresentation:str = " + literal(representation, 2) + "\n}\n")


def read_module(text, prefix):
    # Fixed DATA declaration framing; lexing and literal parsing are unchanged.
    start = text.index("pure val " + prefix + "ActualReturned:C6WholeHost = ")
    body = text[start:].split(" = ", 1)[1]
    reader = Reader(lex(body))
    supplied_host = reader.value()
    values = []
    for suffix, typename in (("ActualRoot", "C6Ref"), ("AdmissionStatus", "str"), ("AdmissionRepresentation", "str")):
        for token in ("pure", "val", prefix + suffix, ":", typename, "="):
            reader.pop(token)
        values.append(reader.value())
    reader.pop("}")
    assert reader.peek() is None
    return supplied_host, *values


def fingerprint(data):
    return {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


def run_case(prefix, input_path, output_dir, candidate_dir):
    byte_count, digest, name = CAPTURES[prefix]
    raw = input_path.read_bytes()
    assert len(raw) == byte_count and hashlib.sha256(raw).hexdigest() == digest, "selected capture identity mismatch"
    observation = json.loads(raw)
    supplied_host = host(observation)
    root = ref(observation["execution"]["image"]["outcome"]["Halted"]["value"])
    admission = observation["preparation"]["admission"]
    text = module_text(name, prefix, supplied_host, root, admission["status"], admission["representation"])
    parsed_host, parsed_root, status, representation = read_module(text, prefix)
    inverse = inverse_host(parsed_host, status, representation)
    selected = {"admission": admission, "image": observation["execution"]["image"]}
    assert exact(inverse, selected), "emitted selected host/admission native-type inverse mismatch"
    assert exact(unref(parsed_root), observation["execution"]["image"]["outcome"]["Halted"]["value"]), "root inverse mismatch"
    # Also validate the actual source-authored candidate DATA if supplied. The
    # independent expected returns are never loaded by this adapter.
    candidate = None
    if candidate_dir is not None:
        candidate_path = candidate_dir / (name + ".qnt")
        candidate_parts = read_module(candidate_path.read_text(), prefix)
        candidate_inverse = inverse_host(candidate_parts[0], candidate_parts[2], candidate_parts[3])
        assert exact(candidate_inverse, selected), "authored candidate selected host inverse mismatch"
        assert exact(unref(candidate_parts[1]), unref(parsed_root)), "authored candidate root inverse mismatch"
        candidate = fingerprint(candidate_path.read_bytes())
    # All envelope fields not represented by C6WholeHost are preserved in the
    # original raw file. This is selected-structure inverse + full raw retention,
    # not an independently encoded inverse for the remaining envelope fields.
    case_dir = output_dir / prefix
    case_dir.mkdir()
    (case_dir / "original-observation.json").write_bytes(raw)
    (case_dir / (name + ".qnt")).write_text(text)
    (case_dir / "inverse-selected-admission-and-image.json").write_text(json.dumps(inverse, indent=2) + "\n")
    (case_dir / "inverse-root.json").write_text(json.dumps(unref(parsed_root), indent=2) + "\n")
    receipt = {"status": "POSITIVE_DATA_SELECTED_INVERSE_PASS_NOT_QUINT_CHECKED", "input": fingerprint(raw), "data": fingerprint(text.encode()), "candidate": candidate,
               "selectors": {"admission": "$.preparation.admission", "image": "$.execution.image", "root": "$.execution.image.outcome.Halted.value"},
               "admissionFields": 6, "viewFields": 23, "imageFields": 23, "allSelectedNativeTypesAndOrderedListsInverseExact": True,
               "rawEnvelopeByteExactRetained": True, "unmodeledEnvelopeFieldsIndependentlyEncoded": False,
               "expectedReturnsRead": False, "modelCompilerProductExecuted": False}
    (case_dir / "INVERSE-RESULT.json").write_text(json.dumps(receipt, indent=2) + "\n")
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--map-capture", type=Path, required=True)
    parser.add_argument("--mirror-capture", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True, help="new output directory")
    parser.add_argument("--candidate-dir", type=Path, help="also inverse-check the authored two DATA modules")
    args = parser.parse_args()
    args.out.mkdir(exist_ok=False)
    receipts = [run_case("map", args.map_capture, args.out, args.candidate_dir), run_case("mirror", args.mirror_capture, args.out, args.candidate_dir)]
    print(json.dumps({"status": "TWO_POSITIVE_DATA_INVERSES_PASS", "receipts": receipts, "subjectExecuted": False}, indent=2))


if __name__ == "__main__":
    main()
