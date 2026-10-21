"""Ordinary semantic projection on the existing positive NP/NG value domain.

Inputs are already received/typed DATA, plus the existing positive_identity_port
result. This module does not parse files or authenticate/admit programs. It does
not evaluate K, receive raw output, read expectations, or project states/events.
All output nodes are fresh. Unsupported/inconsistent positive-domain objects
raise ProjectionDomainError rather than returning a partial logical result.
"""


class ProjectionDomainError(ValueError):
    """The supplied typed objects do not denote this common return domain."""


def _record(value, fields, role):
    if type(value) is not dict or set(value) != set(fields):
        raise ProjectionDomainError(role + ": unsupported record shape")
    return value


def _index(value, role):
    if type(value) is not int or not 0 <= value < (1 << 64):
        raise ProjectionDomainError(role + ": expected positive-profile index")
    return value


def _list(value, role):
    if type(value) is not list:
        raise ProjectionDomainError(role + ": expected ordered list")
    return value


def _type_index(rows):
    out = {}
    for row in _list(rows, "type_rows"):
        _record(row, ("runtime_type_id", "type"), "type row")
        tid = _index(row["runtime_type_id"], "runtime type id")
        if tid in out:
            raise ProjectionDomainError("nonunique runtime type binding")
        out[tid] = row["type"]
    return out


def _np_type(metadata, types):
    """Erase only the argument-ID wrappers defined by existing np_type_ast.

    Each wrapper's ID-to-type link is checked before erasing the wrapper. IDs
    remain the independently selected NG runtime identity, never name lookup.
    Iterative traversal preserves all generic argument order and multiplicity.
    """
    tasks = [("visit", metadata)]
    values = []
    while tasks:
        op, node = tasks.pop()
        if op == "build":
            name, count = node
            arguments = values[-count:] if count else []
            if count:
                del values[-count:]
            values.append({"kind": "Nominal", "name": name, "arguments": arguments})
            continue
        if type(node) is not dict:
            raise ProjectionDomainError("type metadata is unsupported")
        kind = node.get("kind")
        if kind in ("UInt", "Bool"):
            _record(node, ("kind",), "builtin type")
            values.append({"kind": kind})
            continue
        _record(node, ("kind", "name", "arguments"), "nominal type")
        if kind != "Nominal" or type(node["name"]) is not str:
            raise ProjectionDomainError("unsupported nominal type")
        args = _list(node["arguments"], "ordered type arguments")
        tasks.append(("build", (node["name"], len(args))))
        for arg in reversed(args):
            _record(arg, ("runtime_type_id", "type"), "argument identity")
            tid = _index(arg["runtime_type_id"], "argument runtime id")
            if tid not in types or arg["type"] != types[tid]:
                raise ProjectionDomainError("generic argument identity disagrees")
            tasks.append(("visit", arg["type"]))
    if len(values) != 1:
        raise ProjectionDomainError("type metadata did not denote one type")
    return values[0]


def _constructor_index(rows):
    out = {}
    for row in _list(rows, "ctor_rows"):
        _record(row, ("type_id", "ctor_id", "name", "fields"), "constructor metadata")
        key = (_index(row["type_id"], "constructor owner"), _index(row["ctor_id"], "constructor id"))
        names = _list(row["fields"], "ordered field names")
        if type(row["name"]) is not str or any(type(n) is not str for n in names):
            raise ProjectionDomainError("constructor names are not source text")
        if key in out:
            raise ProjectionDomainError("nonunique constructor binding")
        out[key] = row
    return out


def project_return_pair(np_value, ng_value, ported):
    """Project a complete already-typed NP/NG pair to one logical result node.

    `ported` is the existing positive_identity_port.port_positive return DATA.
    Its complete source_document and other fields remain untouched. Required
    metadata: type_rows, ctor_rows, normalized_program_view constructors and
    ctor_fields. Positive complete/acyclic metadata and finite JSON value trees
    are premises; this is not a hostile-input parser or admission procedure.

    NG supplies exact runtime type/constructor identities. NP supplies their
    typed names, generic arguments and named field order. Source metadata binds
    both and checks each child's declared type. UInt decimal text is the exact
    existing independent-result notation; Bool stays native Bool. No wrapping
    None/Some or structural empty variant is inferred from a scalar zero.
    """
    if type(ported) is not dict or ported.get("schema") != "sherlock-programview23-positive-reference-input/1":
        raise ProjectionDomainError("existing positive port result required")
    try:
        types = _type_index(ported["type_rows"])
        ctors = _constructor_index(ported["ctor_rows"])
        view = ported["normalized_program_view"]
        raw_ctors = _list(view["constructors"], "source constructor rows")
        raw_fields = _list(view["ctor_fields"], "source field rows")
    except (KeyError, TypeError) as exc:
        raise ProjectionDomainError("positive binding metadata missing") from exc
    if types.get(0) != {"kind": "UInt"} or types.get(1) != {"kind": "Bool"}:
        raise ProjectionDomainError("builtin runtime type identity differs")

    result = []
    tasks = [(np_value, ng_value, None, result)]
    while tasks:
        np, ng, declared_tid, destination = tasks.pop()
        if type(np) is not dict or type(ng) is not dict or len(np) != 1 or len(ng) != 1:
            raise ProjectionDomainError("one supported value constructor required")
        tag = next(iter(ng))
        if tag not in ("UInt", "Bool", "Nominal") or set(np) != {tag}:
            raise ProjectionDomainError("unsupported or disagreeing value constructor")
        n, g = np[tag], ng[tag]
        if tag in ("UInt", "Bool"):
            tid = 0 if tag == "UInt" else 1
            scalar_type = int if tag == "UInt" else bool
            if type(n) is not scalar_type or type(g) is not scalar_type or n != g:
                raise ProjectionDomainError("NP/NG scalar payload disagrees")
            if declared_tid is not None and tid != declared_tid:
                raise ProjectionDomainError("scalar violates declared field type")
            if tag == "UInt" and not 0 <= g < (1 << 128):
                raise ProjectionDomainError("UInt outside represented UInt128 domain")
            destination.append({"kind": tag, "runtimeTypeId": tid, "value": str(g) if tag == "UInt" else g})
            continue

        _record(g, ("type_id", "ctor_id", "fields"), "NG nominal")
        _record(n, ("type", "constructor", "fields"), "NP nominal")
        tid, cid = _index(g["type_id"], "value type"), _index(g["ctor_id"], "value constructor")
        if declared_tid is not None and tid != declared_tid:
            raise ProjectionDomainError("nominal violates declared field type")
        if tid not in types or (tid, cid) not in ctors or cid >= len(raw_ctors):
            raise ProjectionDomainError("source nominal binding absent")
        metadata = ctors[(tid, cid)]
        if types[tid].get("kind") != "Nominal" or n["type"] != _np_type(types[tid], types):
            raise ProjectionDomainError("NP nominal type/generic arguments disagree")
        if type(n["constructor"]) is not str or n["constructor"] != metadata["name"]:
            raise ProjectionDomainError("NP constructor name disagrees")
        raw = raw_ctors[cid]
        if type(raw) is not dict:
            raise ProjectionDomainError("source constructor binding absent")
        try:
            owner = _index(raw["owning_type"], "source constructor owner")
            start = _index(raw["field_start"], "raw field start")
            count = _index(raw["field_len"], "raw field length")
        except KeyError as exc:
            raise ProjectionDomainError("source constructor shape incomplete") from exc
        if owner != tid:
            raise ProjectionDomainError("source constructor owner disagrees")
        np_fields, ng_fields = _list(n["fields"], "NP fields"), _list(g["fields"], "NG fields")
        if len(np_fields) != count or len(ng_fields) != count or len(metadata["fields"]) != count:
            raise ProjectionDomainError("complete source field arity disagrees")
        # A zero-length source slice retains its raw start; never read a row for it.
        if count and start + count > len(raw_fields):
            raise ProjectionDomainError("declared field slice absent")
        children = []
        destination.append({"kind": "Constructor", "runtimeTypeId": tid, "runtimeCtorId": cid, "fields": children})
        for ordinal in reversed(range(count)):
            field = _record(np_fields[ordinal], ("name", "value"), "NP field")
            if type(field["name"]) is not str or field["name"] != metadata["fields"][ordinal]:
                raise ProjectionDomainError("source field name/order disagrees")
            try:
                child_type = _index(raw_fields[start + ordinal]["type_id"], "source field type")
            except (KeyError, TypeError) as exc:
                raise ProjectionDomainError("source field type binding absent") from exc
            if child_type not in types:
                raise ProjectionDomainError("declared child type metadata absent")
            tasks.append((field["value"], ng_fields[ordinal], child_type, children))
    if len(result) != 1:
        raise ProjectionDomainError("complete pair did not project to one return")
    return result[0]
