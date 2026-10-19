"""Relate captured arm identity and bindings to immutable recursive patterns.

This reads actual primary carriers, never K outputs or expected cells. It is a
structural observation predicate, not a primary/K transition implementation.
Positive selected patterns must be binder-linear; no admission policy is added.
"""
from render_input import fields, same, tagged


def pattern_bindings(pattern, value, match_id, arm_id, path=()):
    tag, payload = tagged(pattern)
    if tag == "Wildcard":
        fields(payload, ())
        return []
    if tag == "Bind":
        if type(payload) is not int:
            raise ValueError("Noninteger source binder")
        return [{"name_id": payload, "match_expr_id": match_id,
                 "source_arm_id": arm_id, "field_path": list(path), "value": value}]
    if tag != "Constructor":
        raise ValueError("Unsupported recursive source pattern")
    fields(payload, ("type_id", "ctor_id", "fields"))
    value_tag, actual = tagged(value)
    if value_tag != "Nominal":
        return None
    fields(actual, ("type_id", "ctor_id", "fields"))
    if (payload["type_id"], payload["ctor_id"]) != (actual["type_id"], actual["ctor_id"]):
        return None
    if len(payload["fields"]) != len(actual["fields"]):
        return None
    bindings = []
    for index, (child, field) in enumerate(zip(payload["fields"], actual["fields"])):
        found = pattern_bindings(child, field, match_id, arm_id, path + (index,))
        if found is None:
            return None
        bindings.extend(found)
    return bindings


def selected_event(evidence):
    return {"ngArmSelected": {key: evidence[key] for key in
        ("match_expr_id", "match_occurrence", "source_arm_id", "bindings", "body_expr_id", "body_environment")}}


def scope_history(compile_document, program, records, view_type):
    """Keep selected-arm provenance on actual frame stack positions.

    A positive Plan exit supplies an actual global ArmRow ID; recursive source
    matching checks first-success/order and binder paths independently of plan
    node identities. CasesPlanHit and the following push must preserve that ID's
    body/environment/frame payload. The frame alone never determines the arm.
    No canonicalMatchPlan or physical-B equivalence is inferred.
    """
    previous = compile_document["genesis"]["image"]
    labels = [None] * len(previous["frames"])
    snapshots, selections, pending = [], {}, None
    for ordinal, record in enumerate(records):
        before, after = record["before"]["image"], record["after"]["image"]
        rule = record["event"]["rule_id"]
        if not same(before, previous):
            raise ValueError("Scope evidence primary image continuity differs")
        view = view_type(compile_document, program, before)
        if rule == "XAdtHelperPlanExit":
            helper = before["helper"]
            if type(helper) is dict and "Plan" in helper:
                plan = helper["Plan"]
                if plan["common"]["site"] == "Cases" and plan["status"] == "Positive":
                    if pending is not None or plan["phase"] != "Done":
                        raise ValueError("Overlapping or unfinished source-arm selection")
                    cases = fields(before["control"]["Cases"], ("arms", "saved_env", "value"))
                    match_id = plan["source_match"]
                    if match_id != before["control_origin"] or plan["saved_env"] != cases["saved_env"] or not same(plan["scrutinee"], cases["value"]):
                        raise ValueError("Plan selection does not retain Cases source/scrutinee")
                    source = view.node(match_id, "Match")
                    arm_ids = list(range(cases["arms"]["start"], cases["arms"]["start"] + cases["arms"]["len"]))
                    if arm_ids != [arm["source_arm_id"] for arm in source["arms"]]:
                        raise ValueError("Cases arm layout differs from immutable source")
                    value = view.value(cases["value"])
                    chosen, bindings = None, None
                    for arm in source["arms"]:
                        found = pattern_bindings(arm["pattern"], value, match_id, arm["source_arm_id"])
                        if found is not None:
                            chosen, bindings = arm, found
                            break
                    if chosen is None or chosen["source_arm_id"] != plan["arm"]:
                        raise ValueError("Actual selected global ArmRow is not source first success")
                    names = [binding["name_id"] for binding in bindings]
                    if len(set(names)) != len(names):
                        raise ValueError("Selected pattern is outside binder-linear positive profile")
                    saved = view.environment(cases["saved_env"])
                    merged = {row["name_id"]: row["value"] for row in saved}
                    merged.update({row["name_id"]: row["value"] for row in bindings})
                    body_env = [{"name_id": name, "value": merged[name]} for name in sorted(merged)]
                    if not same(body_env, view.environment(plan["partial"])):
                        raise ValueError("Actual completed binding environment differs from recursive source paths")
                    pending = {"match_expr_id": match_id, "match_occurrence": view.nodes[match_id]["occurrence"],
                        "source_arm_id": chosen["source_arm_id"], "bindings": bindings,
                        "body_expr_id": chosen["body"], "body_environment": body_env,
                        "saved_environment": saved, "saved_env_root": cases["saved_env"],
                        "body_env_root": plan["partial"], "actual_plan_exit_record": ordinal}
        if rule == "XAdtCasesPlanHit":
            if pending is None:
                raise ValueError("Cases hit has no captured positive source-arm provenance")
            scheduled = fields(after["control"]["ScheduleNext"], ("env", "frame", "next"))
            expected_frame = {"ScopeF": {"origin": pending["match_expr_id"], "saved_env": pending["saved_env_root"]}}
            if (before["control_origin"] != pending["match_expr_id"] or scheduled["next"] != pending["body_expr_id"]
                    or scheduled["env"] != pending["body_env_root"] or not same(scheduled["frame"], expected_frame)):
                raise ValueError("Cases hit changes captured selected arm body/environment/scope")
            pending["actual_cases_hit_record"] = ordinal
        old_frames, new_frames = before["frames"], after["frames"]
        common = 0
        while common < min(len(old_frames), len(new_frames)) and same(old_frames[common], new_frames[common]):
            common += 1
        new_labels = labels[:common]
        for index in range(common, len(new_frames)):
            frame = new_frames[index]
            evidence = None
            if "ScopeF" in frame and "Match" in view.nodes[frame["ScopeF"]["origin"]]["kind"]:
                if (pending is None or rule != "KMachinePushAccept" or index != len(old_frames)
                        or len(new_frames) != len(old_frames) + 1 or common != len(old_frames)
                        or pending.get("actual_cases_hit_record") != ordinal - 1):
                    raise ValueError("New Match scope lacks adjacent Cases-hit/push provenance")
                scheduled = before["control"]["ScheduleNext"]
                if (not same(frame, scheduled["frame"]) or after["control"] != {"Eval": {"expr": pending["body_expr_id"]}}
                        or after["env_root"] != pending["body_env_root"]):
                    raise ValueError("Match scope push changes captured selected body/environment")
                evidence = dict(pending, actual_scope_push_record=ordinal)
                selections[ordinal] = evidence
                pending = None
            new_labels.append(evidence)
        labels = new_labels
        snapshots.append({index: item for index, item in enumerate(labels) if item is not None})
        previous = after
    return snapshots, selections
