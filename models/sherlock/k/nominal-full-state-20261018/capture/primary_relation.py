"""Declared weak-boundary relation for captured positive primary/K prefixes.

This is a partial observation relation, not a primary stepper or a general B
projection. Boundaries are fixed in an input manifest BEFORE K execution. Every
intervening primary record is retained and checked for continuity/accounting.
Only declared aligned controls/frame forms are decoded; others fail explicitly.
"""
from render_input import same, fields, tagged
from match_scope_evidence import scope_history, selected_event


def span(rows, spec):
    fields(spec, ("start", "len"))
    start, length = spec["start"], spec["len"]
    if type(start) is not int or type(length) is not int or min(start, length) < 0 or start + length > len(rows):
        raise ValueError("Captured slice exceeds complete table")
    return rows[start:start + length]


class AlignedPrimaryView:
    def __init__(self, compile_document, ng_program, image, scope_evidence=None):
        self.scope_evidence = {} if scope_evidence is None else scope_evidence
        self.source = compile_document["canonicalProgramView"]
        self.program = ng_program
        self.image = image
        self.nodes = {node["id"]: node for node in ng_program["expressions"]}
        if len(self.nodes) != len(ng_program["expressions"]):
            raise ValueError("Ambiguous input node IDs")

    def node(self, index, tag):
        node = self.nodes[index]
        return fields(node["kind"][tag], {
            "Let": ("binder", "bound_expr", "body_expr"),
            "Call": ("name_id", "arguments"),
            "Construct": ("type_id", "ctor_id", "fields"),
            "Match": ("scrutinee", "arms"),
        }[tag])

    def value(self, reference, active=None):
        active = set() if active is None else active
        space, index = tagged(reference)
        if space not in ("Const", "Data") or type(index) is not int or index < 0:
            raise ValueError("Unknown captured value reference")
        identity = (space, index)
        if identity in active:
            raise ValueError("Cyclic value outside positive captured input")
        active.add(identity)
        try:
            if space == "Const":
                row = self.source["constants"][index]
                tag, payload = tagged(row)
                if payload["marker_free"] is not True:
                    raise ValueError("Captured constant is not marker-free")
                fields_table = self.source["constant_fields"]
                child = lambda value: {"Const": value}
            else:
                if index >= self.image["active"]["data"]:
                    raise ValueError("Data reference outside actual active prefix")
                row = self.image["data"][index]
                tag, payload = (row, {}) if type(row) is str else tagged(row)
                fields_table = self.image["data_fields"][:self.image["active"]["data_fields"]]
                child = lambda value: value
            if tag == "U128":
                number = payload["n"]
                if type(number) is not str or not number.isascii() or not number.isdecimal():
                    raise ValueError("Captured UInt decimal wire differs")
                return {"UInt": int(number)}
            if tag == "Bool":
                if type(payload["b"]) is not bool:
                    raise ValueError("Captured Bool differs")
                return {"Bool": payload["b"]}
            if tag == "Constructor":
                type_id, ctor_id = payload["type_id"], payload["ctor_id"]
            elif tag in ("Nil", "Cons", "Leaf", "Branch"):
                type_id, ctor_id = {"Nil": (2, 0), "Cons": (2, 1), "Leaf": (3, 2), "Branch": (3, 3)}[tag]
            else:
                raise ValueError("Unsupported captured value variant: " + str(tag))
            refs = [] if tag == "Nil" else span(fields_table, payload["fields"])
            return {"Nominal": {"type_id": type_id, "ctor_id": ctor_id,
                "fields": [self.value(child(ref), active) for ref in refs]}}
        finally:
            active.remove(identity)

    def environment(self, root):
        rows = self.image["env"][:self.image["active"]["env"]]
        seen_rows, values = set(), {}
        while root is not None:
            if type(root) is not int or root < 0 or root >= len(rows) or root in seen_rows:
                raise ValueError("Invalid captured environment chain")
            seen_rows.add(root)
            row = fields(rows[root], ("binder", "previous", "value"))
            if row["binder"] not in values:
                values[row["binder"]] = self.value(row["value"])
            root = row["previous"]
        return [{"name_id": name, "value": values[name]} for name in sorted(values)]

    def slots(self, spec):
        return [self.value(ref) for ref in span(self.image["value_slots"][:self.image["active"]["value_slots"]], spec)]

    def control(self):
        tag, payload = tagged(self.image["control"])
        if tag == "Eval":
            fields(payload, ("expr",))
            return {"ngEval": {"expr_id": payload["expr"]}}
        if tag == "Return":
            fields(payload, ("value",))
            return {"ngReturn": {"value": self.value(payload["value"])}}
        if tag == "Invoke":
            fields(payload, ("args", "function_name"))
            origin = self.image["control_origin"]
            call = self.node(origin, "Call")
            if call["name_id"] != payload["function_name"]:
                raise ValueError("Invoke source origin differs")
            return {"ngApply": {"call_expr_id": origin, "call_occurrence": self.nodes[origin]["occurrence"],
                "function_name_id": payload["function_name"], "arguments": self.slots(payload["args"])}}
        if tag == "Cases":
            fields(payload, ("arms", "saved_env", "value"))
            origin = self.image["control_origin"]
            source = self.node(origin, "Match")
            ids = list(range(payload["arms"]["start"], payload["arms"]["start"] + payload["arms"]["len"]))
            if ids != [arm["source_arm_id"] for arm in source["arms"]]:
                raise ValueError("Cases source arm suffix is outside this declared boundary profile")
            return {"ngChoose": {"match_expr_id": origin, "match_occurrence": self.nodes[origin]["occurrence"],
                "scrutinee_value": self.value(payload["value"]), "remaining_source_arms": source["arms"],
                "saved_environment": self.environment(payload["saved_env"])}}
        raise ValueError("No selected aligned control counterpart: " + str(tag))

    def frame(self, frame, frame_index=None):
        tag, payload = tagged(frame)
        origin = payload["origin"]
        occurrence = self.nodes[origin]["occurrence"]
        saved = self.environment(payload["saved_env"])
        if tag == "LetF":
            fields(payload, ("binder", "body", "origin", "saved_env"))
            source = self.node(origin, "Let")
            if (payload["binder"], payload["body"]) != (source["binder"], source["body_expr"]):
                raise ValueError("Let frame source fields differ")
            return {"ngLetValue": {"source_expr": origin, "occurrence": occurrence,
                "binder": payload["binder"], "bound_expr": source["bound_expr"],
                "body_expr": payload["body"], "saved_environment": saved}}
        if tag == "ScopeF":
            fields(payload, ("origin", "saved_env"))
            if "Let" in self.nodes[origin]["kind"]:
                source = self.node(origin, "Let")
                return {"ngLetScope": {"source_expr": origin, "occurrence": occurrence,
                    "binder": source["binder"], "saved_environment": saved}}
            self.node(origin, "Match")
            evidence = self.scope_evidence.get(frame_index)
            if evidence is None or evidence["match_expr_id"] != origin or not same(evidence["saved_environment"], saved):
                raise ValueError("Match ScopeF lacks causal source-arm selection evidence")
            return {"ngArmScope": {"match_expr_id": origin, "match_occurrence": occurrence,
                "source_arm_id": evidence["source_arm_id"], "saved_environment": saved}}
        if tag == "ArgsF":
            fields(payload, ("completed", "function_name", "origin", "rest", "saved_env"))
            source = self.node(origin, "Call")
            prefix = self.slots(payload["completed"])
            cursor = len(prefix)
            remaining = span(self.source["children"], payload["rest"])
            if source["name_id"] != payload["function_name"] or not same(remaining, source["arguments"][cursor + 1:]):
                raise ValueError("Argument frame ordered remainder differs")
            return {"ngArguments": {"call_expr_id": origin, "call_occurrence": occurrence,
                "function_name_id": payload["function_name"], "argument_cursor": cursor,
                "active_expr_id": source["arguments"][cursor], "evaluated_prefix": prefix,
                "remaining_expr_ids": remaining, "saved_environment": saved}}
        if tag == "ReturnF":
            fields(payload, ("origin", "return_type", "saved_env"))
            source = self.node(origin, "Call")
            return {"ngFunctionReturn": {"call_expr_id": origin, "call_occurrence": occurrence,
                "function_name_id": source["name_id"], "return_type_id": payload["return_type"],
                "saved_environment": saved}}
        if tag == "ConstructorFieldsF":
            fields(payload, ("checked", "completed", "ctor_id", "field_type", "index", "origin", "rest", "saved_env", "type_id"))
            source = self.node(origin, "Construct")
            types = [ctor["field_types"] for decl in self.program["declarations"] for ctor in decl["ctors"]
                if (ctor["type_id"], ctor["ctor_id"]) == (payload["type_id"], payload["ctor_id"])]
            if len(types) != 1:
                raise ValueError("Ambiguous constructor metadata")
            prefix, cursor = self.slots(payload["completed"]), payload["index"]
            remaining = span(self.source["children"], payload["rest"])
            if len(prefix) != cursor or not same(remaining, source["fields"][cursor + 1:]) or types[0][cursor] != payload["field_type"]:
                raise ValueError("Constructor frame cursor/type/remainder differs")
            if (source["type_id"], source["ctor_id"]) != (payload["type_id"], payload["ctor_id"]):
                raise ValueError("Constructor frame source identity differs")
            return {"ngConstructorFields": {"constructor_expr_id": origin, "constructor_occurrence": occurrence,
                "type_id": payload["type_id"], "ctor_id": payload["ctor_id"], "field_types": types[0],
                "field_cursor": cursor, "active_expr_id": source["fields"][cursor], "evaluated_prefix": prefix,
                "remaining_expr_ids": remaining, "saved_environment": saved}}
        if tag == "MatchF":
            fields(payload, ("arms", "origin", "saved_env"))
            source = self.node(origin, "Match")
            source_arm_ids = list(range(payload["arms"]["start"], payload["arms"]["start"] + payload["arms"]["len"]))
            if source_arm_ids != [arm["source_arm_id"] for arm in source["arms"]]:
                raise ValueError("Match frame global ordered arms differ")
            return {"ngScrutinee": {"match_expr_id": origin, "match_occurrence": occurrence,
                "source_arms": source["arms"], "saved_environment": saved}}
        raise ValueError("No selected aligned frame counterpart: " + str(tag))

    def shared(self):
        if self.image["helper"] is not None:
            raise ValueError("Frozen aligned boundary has a pending primary helper")
        return {"control": self.control(), "environment": self.environment(self.image["env_root"]),
            "frames_top_first": [self.frame(frame, index) for index, frame in reversed(list(enumerate(self.image["frames"])))]}


def expand(boundary, compile_document):
    admission = boundary["admission"]
    if admission == {"same_as": "compile.json#/genesis/admission"}:
        admission = compile_document["genesis"]["admission"]
    elif type(admission) is dict and set(admission) == {"inline"}:
        admission = admission["inline"]
    else:
        raise ValueError("Unknown captured admission reference")
    return {"admission": admission, "image": boundary["image"]}


def relate(compile_document, ng_program, primary_records, schedule, k_states):
    scope_at, selections = scope_history(compile_document, ng_program, primary_records, AlignedPrimaryView)
    rows, previous = [], compile_document["genesis"]
    if not same(previous["admission"]["view"], compile_document["canonicalProgramView"]):
        raise ValueError("Whole canonical ProgramView differs from actual admission view")
    accounting = []
    for ordinal, record in enumerate(primary_records):
        if record["record"] != ordinal or record["kind"] != "Event":
            raise ValueError("Primary prefix is not the complete fixed advancing prefix")
        before, after = expand(record["before"], compile_document), expand(record["after"], compile_document)
        if not same(before, previous):
            raise ValueError("Complete primary record continuity differs")
        if not same(after["admission"], compile_document["genesis"]["admission"]):
            raise ValueError("Complete actual admission changed")
        event = record["event"]
        expr = event["source_expr_id"]
        if type(expr) is not int or expr < 0 or expr >= len(compile_document["canonicalProgramView"]["occurrences"]):
            raise ValueError("Primary event has no selected source occurrence")
        origin = compile_document["canonicalProgramView"]["occurrences"][expr]
        source_occurrence = {"Occurrence": {"ast_path": span(compile_document["canonicalProgramView"]["occurrence_paths"], origin["path"]),
            "function": origin["function"], "owner_is_function": origin["owner_is_function"]}}
        if not same(event["source_occurrence"], source_occurrence):
            raise ValueError("Complete primary event source origin differs from compiled occurrence row")
        b, a = before["image"], after["image"]
        for field, delta in (("consumed_work", "work_delta"), ("helper_work", "helper_work_delta"),
                             ("helper_visits", "helper_visits_delta"), ("reference_cursor", "reference_cursor_delta")):
            if int(a[field]) - int(b[field]) != int(event[delta]):
                raise ValueError("Primary observed counter delta differs: " + field)
        if int(b["remaining_work"]) - int(a["remaining_work"]) != int(event["work_delta"]):
            raise ValueError("Primary observed remaining work differs")
        accounting.append({"primary_record": ordinal, "full_event": event,
            "before": {key: b[key] for key in ("initial_fuel", "remaining_work", "consumed_work", "helper_work", "helper_visits", "reference_cursor")},
            "after": {key: a[key] for key in ("initial_fuel", "remaining_work", "consumed_work", "helper_work", "helper_visits", "reference_cursor")}})
        previous = after
    if len(schedule["pairs"]) != len(k_states):
        raise ValueError("Actual K prefix does not cover the independently fixed boundary schedule")
    for pair, state in zip(schedule["pairs"], k_states):
        ordinal, recordid = pair["k_state_ordinal"], pair["primary_after_record"]
        image = compile_document["genesis"]["image"] if recordid is None else primary_records[recordid]["after"]["image"]
        primary = AlignedPrimaryView(compile_document, ng_program, image, {} if recordid is None else scope_at[recordid]).shared()
        k_shared = {key: state[key] for key in ("control", "environment", "frames_top_first")}
        rows.append({"k_state_ordinal": ordinal, "primary_after_record": recordid,
            "complete_K_program_equal_input": same(state["program"], ng_program),
            "shared_control_environment_frames_equal": same(k_shared, primary),
            "primary_shared": primary, "actual_K_shared": k_shared})
    protocols = []
    for block in schedule["event_block_protocol"]:
        event = k_states[block["k_event_ordinal"] + 1]["events"][-1]
        event_tag, _ = tagged(event)
        endpoint = primary_records[block["primary_last_record"]]["event"]
        ok = (event_tag == block["k_event_tag"] and endpoint["rule_id"] == block["primary_last_rule"]
            and endpoint["source_expr_id"] == block["primary_event_source_expr"])
        payload_relation = None
        if event_tag == "ngScrutineeReturned":
            view = AlignedPrimaryView(compile_document, ng_program, primary_records[block["primary_last_record"]]["after"]["image"])
            choose = view.control()["ngChoose"]
            declared_event = {event_tag: {"match_expr_id": choose["match_expr_id"], "match_occurrence": choose["match_occurrence"],
                "value": choose["scrutinee_value"], "source_arms": choose["remaining_source_arms"],
                "saved_environment": choose["saved_environment"]}}
            payload_relation = same(event, declared_event)
            ok = ok and payload_relation
        elif event_tag == "ngArmSelected":
            declared_event = selected_event(selections[block["primary_last_record"]])
            payload_relation = same(event, declared_event)
            ok = ok and payload_relation
        protocols.append({"frozen_block": block, "event_source_protocol_equal": ok,
            "selected_match_full_payload_relation": payload_relation,
            "full_actual_K_event": event, "full_primary_endpoint_event": endpoint})
    return {"status": "PASS" if all(row["complete_K_program_equal_input"] and row["shared_control_environment_frames_equal"] for row in rows)
            and all(row["event_source_protocol_equal"] for row in protocols) else "FAIL",
        "match_scope_causal_source_evidence": list(selections.values()),
        "boundary_pairs": rows, "event_blocks": protocols, "complete_primary_accounting": accounting,
        "retained_raw_primary_records": len(primary_records), "K_work_carrier": "ABSENT; NO CLOCK/FUEL/WORK EQUALITY CLAIM",
        "remaining_observation_budget": schedule["k_observation_budget"] - (len(k_states) - 1),
        "unmapped_primary_fields": "All primary images/admissions/events are retained verbatim in the pinned raw prefix; physical refs, allocation, helper internals, checked flag, layout/capacity/identity/counters have no K equality carrier.",
        "scope": "One prefrozen weak-boundary prefix; no boundary fitting, primary step execution or expected-state calculation"}
