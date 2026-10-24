"""Invented synthetic fixtures only; PROPOSED and UNRUN.

Every program, source span, declaration, capture record and expectation below
is manually invented saved data written only under TMPDIR. It is not an
implementation of source semantics, not an owner-executed trace and not a
blind oracle for any real case; real outcomes are never predicted. Bridge
events (rules the comparator does not recognize) and all physical counters
and IDs are synthetic and unbound. Imports only the passive consumer.

Test obligations, in file order:
 a  builtin TypeIds 0..3 resolve positionally while names[0] is "build".
 a  Refusals: nonzero builtin padding, decl Some below 4, decl None at 4,
    wrong builtin ctor slice.
 h  Generic Opt[UInt] application, ctor owner, instantiated field type,
    cyclic application.
 h  Exact UInt128 bounds and Bool-vs-integer distinctions in values and
    comparison.
 h  First-visible lexical shadowing and cyclic environment refusal.
 h  Repeated source site gets distinct dynamic construction identities.
 b  Full five-file capture through capture_case:
    - EnterBind -> BoundValid -> PushAccept(ReturnF) -> ReturnCallValid;
    - Construct open -> Final -> MaterializeDone;
    - PlanEntry -> PlanBind -> PlanHit;
    - Published -> Terminal.
    Exact five fields, compare PASS, call lifecycle ledger records.
 d  Two nested constructor frames: outermost first, strict prefixes and
    suffixes, caller/callee/saved contexts.
 c  Mutations, each a refusal or FAIL with prior observations kept:
    - global arm offset;
    - Bool value (full JSON first divergence);
    - duplicated body-entry push;
    - swapped materialization field order;
    - Applied after Published;
    - changed inline admission.
 h  Truncated capture ending on an intermediate Return is never success.
 h  Checkpoints selected by integer at_call; whole calls arrays compared.
 e  Early NUL capture path gives a located per-case gap, not an escape.
 e  main project keeps a valid case beside an AttributeError case.
 e  Top-level AttributeError and ValueError each produce FAILURE.json.
 h  Existing output directory is refused without overwrite.
 f  checked returns the hashed buffer from one mocked read.
 f  checked refuses a changed file.
 g  compare with a valid synthetic SAFE pin:
    - positive PASS control reads expected only after all pins are checked;
    - changed projection is refused before expected is opened.
"""
import builtins
import contextlib
import copy
import hashlib
import io
import json
import os
import pathlib
import sys
import tempfile
import types
import unittest
from pathlib import Path
from unittest import mock

import symbolic_compare as sc

SRC = ("type Box { Wrap(flag: Bool, inner: Box, tag: UInt); Unit(n: UInt); }\n"
       "type Opt<T> { Some(v: T); }\n"
       "fn build(k: UInt) -> Box { Wrap(true, Unit(id(k)), 9) }\n"
       "fn id(x: UInt) -> UInt { x }\n"
       "entry match build(7) { Wrap(f, Unit(m), _) => m }\n")
NAMES = ["build", "id", "Box", "Wrap", "Unit", "flag", "inner", "tag", "n", "k", "x", "f", "m", "Opt", "Some", "v"]
PATHS = [[], [0], [0, 0], [1], [], [0], [1], [1, 0], [1, 0, 0], [2], []]
KINDS = "match call uint var construct bool construct call var uint var".split()
EXPR_SPANS = [("match build(7) { Wrap(f, Unit(m), _) => m }", None), ("build(7)", None), ("7", "build(7)"),
              ("m", "=> m"), ("Wrap(true, Unit(id(k)), 9)", None), ("true", None), ("Unit(id(k))", None),
              ("id(k)", None), ("k", "id(k)"), ("9", None), ("x", "{ x }")]
PATS = [([], "constructor-pattern", "Wrap(f, Unit(m), _)", None), ([0], "binder", "f", "Wrap(f"),
        ([1], "constructor-pattern", "Unit(m)", None), ([1, 0], "binder", "m", "Unit(m)"), ([2], "wildcard", "_", None)]
FILES = ("input.json", "compile.json", "records.jsonl", "errors.jsonl", "final.json")
BOUNDS = dict({k: 4096 for k in ("code_rows function_rows code_child_slots parameter_slots pattern_rows match_plan_rows "
                                 "match_arm_rows type_rows constructor_rows constructor_field_slots max_arity constant_nodes "
                                 "constant_field_slots data_nodes data_field_slots environment_cells frames helper_tasks "
                                 "serialization_bytes").split()},
              max_reference_work="4096", max_helper_work="4096", step_cap="4096")
LAYOUT = {"layout_version": ["synthetic", "1"], "field_id": None, "widths": None, "scalar_layout_id": None, "bounds": BOUNDS,
          "capacities": {"name_rows": 64, "name_bytes": 4096, "occurrence_path_slots": 64, "value_slots": 4096},
          "boundary_capacities_present": False}
IDENT = {"profile_id": 1, "grammar_domain": "GenericNominalExtensionReserved", "finance": "Absent",
         "layout_version": ["synthetic", "1"], "layout_digest": "Unselected", "bounds_digest": "Unselected",
         "field_id": None, "program_membership_digest": "Unselected"}
RUN_ID = {"identity": IDENT, "fuel_input": {"Nat": "4096"}, "stack_limit_input": {"Nat": "64"}}
CONFIG = {"bounds": BOUNDS, "layout": LAYOUT, "domain": {"grammar": "GenericNominalExtensionReserved", "finance": "Absent"},
          "fuel": {"Nat": "4096"}, "stack": {"Nat": "64"}}
BUDGET = {"synthetic_extent_%d" % i: 4096 for i in range(8)}
PREP = {"work_map": []}
PLANS = {k: {"status": "missing", "artifact": None, "reason": "synthetic"} for k in ("witness", "constraint", "export")}

E, B = {"entry": True}, {"function": "build"}
U7, U9, T = {"UInt": "7"}, {"UInt": "9"}, {"Bool": True}
BOX = {"named": "Box", "arguments": []}
UNIT7 = {"Nominal": {"type": BOX, "constructor": "Unit", "fields": [U7]}}
WRAP = {"Nominal": {"type": BOX, "constructor": "Wrap", "fields": [T, UNIT7, U9]}}
K, X = {"k": U7}, {"x": U7}


def src(owner, *path):
    return {"owner": owner, "path": list(path)}


OUTER = {"source": src(B), "invocation": 0, "type": BOX, "constructor": "Wrap", "field_sources": [src(B, 0), src(B, 1), src(B, 2)],
         "field_values": [T, UNIT7, U9], "saved_environment": K, "in_call": 0}
INNER = {"source": src(B, 1), "invocation": 1, "type": BOX, "constructor": "Unit", "field_sources": [src(B, 1, 0)],
         "field_values": [U7], "saved_environment": K, "in_call": 0}
MILESTONE = "function_entry_after_all_arguments"
EXPECTED = {
    "final_value": U7,
    "calls": [{"invocation": 0, "function": "build", "call_site": src(E, 0), "arguments": [U7], "caller_environment": {}, "callee_parameters": K},
              {"invocation": 1, "function": "id", "call_site": src(B, 1, 0), "arguments": [U7], "caller_environment": K, "callee_parameters": X}],
    "matches": [{"source": src(E), "invocation": 0, "selected_arm": 0, "scrutinee": WRAP, "selected_body": src(E, 1),
                 "bindings": [{"name": "f", "pattern_path": [0], "value": T}, {"name": "m", "pattern_path": [1, 0], "value": U7}]}],
    "constructors": {"open_order": [OUTER, INNER], "materialize_order": [INNER, OUTER]},
    "pending_checkpoints": [
        {"at_call": 0, "milestone": MILESTONE, "pending": [], "caller_environment": {}, "callee_parameters": K},
        {"at_call": 1, "milestone": MILESTONE, "caller_environment": K, "callee_parameters": X, "pending": [
            {"construction_invocation": 0, "source": src(B), "current_field": 1, "completed_fields": [T], "saved_environment": K},
            {"construction_invocation": 1, "source": src(B, 1), "current_field": 0, "completed_fields": [], "saved_environment": K}]}],
    "limitations": ["synthetic invented expectation; limitations are not compared"]}


def sl(start=0, length=0):
    return {"start": start if length else 0, "len": length}


def C(i): return {"Const": i}
def D(i): return {"Data": i}
def ev(e): return {"Eval": {"expr": e}}
def ret(v): return {"Return": {"value": v}}
def sched(frame, nxt, env): return {"ScheduleNext": {"frame": frame, "next": nxt, "env": env}}
def cell(binder, value, prev): return {"binder": binder, "value": value, "previous": prev}
def row(ctor, start, n): return {"Constructor": {"type_id": 4, "ctor_id": ctor, "fields": {"start": start, "len": n}}}


def cf(ctor, index, ftype, checked, completed, rest, origin, saved=0):
    return {"ConstructorFieldsF": {"type_id": 4, "ctor_id": ctor, "index": index, "field_type": ftype, "checked": checked,
                                   "completed": completed, "rest": rest, "saved_env": saved, "origin": origin}}


def plan(node, phase, after, cur, path, cursor, partial, arm=None):
    return {"Plan": {"common": {"site": "Cases", "origin_reference_cursor": "0", "parent_resume": None}, "source_match": 0,
                     "scrutinee": D(1), "saved_env": None, "partial": partial, "node": node, "phase": phase,
                     "after_walk": after, "cur": cur, "path": path, "path_cursor": cursor, "arm": arm, "status": "Pending"}}


def span(text, ctx=None):
    s = SRC.index(text, SRC.index(ctx) if ctx else 0)
    return {"start": s, "end": s + len(text)}


def site(name, ctx=None):
    return {"name": name, "name_span": span(name, ctx), "span": span(name, ctx)}


def fld(name, ty, ctx):
    return {"name": name, "binder_span": span(name, ctx), "type_span": span(ty, ctx)}


def provenance():
    owners = [E] * 4 + [{"function_index": 0}] * 6 + [{"function_index": 1}]
    return {
        "language_span": span("type"), "program_name_span": span("entry"),
        "declarations": [
            dict(site("Box"), parameters=[], constructors=[
                dict(site("Wrap"), fields=[fld("flag", "Bool", "flag: Bool"), fld("inner", "Box", "inner: Box"), fld("tag", "UInt", "tag: UInt")]),
                dict(site("Unit"), fields=[fld("n", "UInt", "(n: UInt")])]),
            dict(site("Opt"), parameters=[{"name": "T", "span": span("T", "<T>")}],
                 constructors=[dict(site("Some"), fields=[fld("v", "T", "(v: T)")])])],
        "functions": [dict(site("build"), parameters=[fld("k", "UInt", "(k: UInt)")], return_type_span=span("Box", "-> Box")),
                      dict(site("id", "fn id"), parameters=[fld("x", "UInt", "(x: UInt)")], return_type_span=span("UInt", "-> UInt"))],
        "expressions": [{"owner": o, "path": p, "span": span(*s), "kind": k} for o, p, s, k in zip(owners, PATHS, EXPR_SPANS, KINDS)],
        "patterns": [{"owner": E, "match_path": [], "arm": 0, "field_path": fp, "span": span(t, c), "kind": k} for fp, k, t, c in PATS]}


def program():
    flat, occ = [], []
    for i, path in enumerate(PATHS):
        occ.append({"owner_is_function": i >= 4, "function": int(i == 10), "path": sl(len(flat), len(path))})
        flat += path
    def ctor(t, n, o, s, k): return {"owning_type": t, "name_id": n, "source_order": o, "field_start": s, "field_len": k}
    def ty(**kw): return dict({"name_id": 0, "decl": None, "args": sl()}, **kw)
    def fd(n, t): return {"name_id": n, "type_id": t}
    return {
        "entry": 0, "names": list(NAMES), "occurrences": occ, "occurrence_paths": flat, "children": [2, 5, 6, 9, 7, 8],
        "expressions": [{"Match": {"scrutinee": 1, "arms": sl(0, 1), "plan": 0}}, {"Call": {"function_name": 0, "args": sl(0, 1)}},
                        {"Literal": {"constant": 0}}, {"Var": {"binder": 12}},
                        {"Construct": {"type_id": 4, "ctor_id": 4, "fields": sl(1, 3), "constant": None}}, {"Literal": {"constant": 1}},
                        {"Construct": {"type_id": 4, "ctor_id": 5, "fields": sl(4, 1), "constant": None}},
                        {"Call": {"function_name": 1, "args": sl(5, 1)}}, {"Var": {"binder": 9}}, {"Literal": {"constant": 2}},
                        {"Var": {"binder": 10}}],
        "functions": [{"function_name": 0, "params": sl(0, 1), "return_type": 4, "body": 4},
                      {"function_name": 1, "params": sl(1, 1), "return_type": 0, "body": 10}],
        "parameters": [{"binder": 9, "sort": 0}, {"binder": 10, "sort": 0}],
        "arms": [{"pattern": 4, "body": 3}],
        "patterns": [{"PVar": {"x": 11}}, {"PVar": {"x": 12}}, {"Constructor": {"type_id": 4, "ctor_id": 5, "children": sl(0, 1)}},
                     "Wildcard", {"Constructor": {"type_id": 4, "ctor_id": 4, "children": sl(1, 3)}}],
        "pattern_children": [1, 0, 2, 3],
        "types": [ty(ctor_start=s, ctor_len=n) for s, n in ((0, 0), (0, 0), (0, 2), (2, 2))]
                 + [ty(name_id=2, decl=0, ctor_start=4, ctor_len=2), ty(name_id=13, decl=1, args=sl(0, 1), ctor_start=6, ctor_len=1)],
        "type_args": [0],
        "constructors": [ctor(2, 0, 0, 0, 0), ctor(2, 0, 1, 0, 2), ctor(3, 0, 0, 2, 1), ctor(3, 0, 1, 3, 2),
                         ctor(4, 3, 0, 5, 3), ctor(4, 4, 1, 8, 1), ctor(5, 14, 0, 9, 1)],
        "ctor_fields": [fd(0, 0), fd(0, 2), fd(0, 0), fd(0, 3), fd(0, 3), fd(5, 1), fd(6, 4), fd(7, 0), fd(8, 0), fd(15, 0)],
        "decl_param_counts": [0, 1], "decl_ctor_counts": [2, 1],
        "decl_templates": [
            {"name": "Box", "parameters": [], "ctors": [
                {"name": "Wrap", "fields": [{"name": "flag", "ty": "Bool"}, {"name": "inner", "ty": {"Named": {"name": "Box"}}}, {"name": "tag", "ty": "UInt"}]},
                {"name": "Unit", "fields": [{"name": "n", "ty": "UInt"}]}]},
            {"name": "Opt", "parameters": ["T"], "ctors": [{"name": "Some", "fields": [{"name": "v", "ty": {"Param": {"name": "T"}}}]}]}],
        "plan_nodes": [{"Switch": {"source_match": 0, "path": sl(), "type_id": 4, "cases": sl(0, 2), "default": 5}},
                       {"Switch": {"source_match": 0, "path": sl(0, 1), "type_id": 4, "cases": sl(2, 2), "default": 5}},
                       {"Bind": {"source_match": 0, "path": sl(1, 1), "binder": 11, "next": 3}},
                       {"Bind": {"source_match": 0, "path": sl(2, 2), "binder": 12, "next": 4}},
                       {"Select": {"source_match": 0, "arm": 0}}, {"NoMatch": {"source_match": 0}}],
        "plan_cases": [1, None, None, 2], "field_paths": [1, 0, 1, 0],
        "constants": [{"U128": {"marker_free": True, "n": "7"}}, {"Bool": {"marker_free": True, "b": True}},
                      {"U128": {"marker_free": True, "n": "9"}}],
        "constant_fields": []}


def fix(img):
    img["active"] = {t: len(img[t]) for t in ("data", "data_fields", "env", "value_slots")}
    img["alloc"] = {"data_nodes": len(img["data"]), "data_fields": len(img["data_fields"]), "environment_cells": len(img["env"])}
    return img


def image(**kw):
    zero = {"data_nodes": 0, "data_fields": 0, "environment_cells": 0}
    img = {"run_id": RUN_ID, "control": ev(0), "control_origin": 0, "env_root": None, "stack_limit": 64, "outcome": "Running",
           "initial_fuel": "4096", "remaining_work": "4096", "consumed_work": "0", "helper_work": "0", "helper_visits": "0",
           "reference_cursor": "0", "alloc": zero, "active": {}, "helper": None, "result": None,
           "seg_base": {"active": {"data": 0, "data_fields": 0, "env": 0, "value_slots": 0}, "alloc": dict(zero), "consumed_work": "0"},
           "data": [], "data_fields": [], "env": [], "frames": [], "tasks": [], "value_slots": []}
    img.update(kw)
    return fix(img)


def event(n, rule, kind, disp, b, a):
    o = b["control_origin"]
    tag = next(iter(a["outcome"])) if isinstance(a["outcome"], dict) else a["outcome"]
    return {"step_id": {"run_id": RUN_ID, "ordinal": str(n)}, "rule_id": rule, "event_kind": kind, "site": None,
            "helper_kind": None, "disposition": disp, "work_delta": int(disp == "Applied"), "helper_work_delta": 0,
            "helper_visits_delta": 0, "reference_cursor_delta": int(kind == "LegacyComplete"),
            "alloc_deltas": {"data_nodes": 0, "data_fields": 0, "environment_cells": 0},
            "value_slot_writes": len(a["value_slots"]) - len(b["value_slots"]), "logical_depth_before": len(b["frames"]),
            "logical_depth_after": len(a["frames"]), "task_cursor_before": 0, "task_cursor_after": 0,
            "helper_phase_before": None, "helper_phase_after": None, "helper_cursor_before": None, "helper_cursor_after": None,
            "result_tag": None, "origin_reference_cursor": None, "source_expr_id": o,
            "source_occurrence": {"Occurrence": {"owner_is_function": o >= 4, "function": int(o == 10), "ast_path": PATHS[o]}},
            "exhausted_kind": None, "outcome_tag": tag, "outcome_payload": a["outcome"],
            "boundary": "None" if tag == "Running" else "Terminal"}


class Chain:
    """Invented image chain; each after image is the next before image."""
    def __init__(self, hooks=None):
        self.cur, self.steps, self.at, self.hooks = image(), [], {}, hooks or {}

    def go(self, rule, kind="LegacyComplete", disp="Applied", **changes):
        key = "%s#%d" % (rule, sum(s[0] == rule for s in self.steps))
        after = copy.deepcopy(self.cur)
        after.update(copy.deepcopy(changes))
        if key in self.hooks:
            self.hooks[key](after)
        self.at[key] = len(self.steps)
        self.steps.append((rule, kind, disp, self.cur, fix(after)))
        self.cur = after

    def add(self, table, *rows):
        return self.cur[table] + list(rows)

    def top(self, frame):
        return self.cur["frames"][:-1] + [frame]

    def push(self):
        s = self.cur["control"]["ScheduleNext"]
        self.go("KMachinePushAccept", frames=self.add("frames", s["frame"]), env_root=s["env"], control=ev(s["next"]), control_origin=s["next"])

    def enter(self, ret_t, body, env):
        rf = {"ReturnF": {"return_type": ret_t, "saved_env": self.cur["env_root"], "origin": self.cur["control_origin"]}}
        self.go("KMachineEnterBind", control={"Bound": {"binding": {"Ok": {"env": env}}, "return_type": ret_t, "body": body}}, result=None)
        self.go("KMachineBoundValid", control=sched(rf, body, env))
        self.push()
        return rf


def run(hooks=None, dup_push=False, late_applied=False):
    c = Chain(hooks)
    g = c.go
    g("KMachineEvalMatch", control=sched({"MatchF": {"arms": sl(0, 1), "saved_env": None, "origin": 0}}, 1, None))
    c.push()
    g("KMachineReturnArgumentsFinal", value_slots=[C(0)], env=[cell(9, C(0), None)],
      control={"Enter": {"function": 0, "args": sl(0, 1)}}, result={"BindOk": {"env": 0}})
    c.enter(4, 4, 0)
    g("XAdtEvalConstructFields", control=sched(cf(4, 0, 1, False, sl(), sl(2, 2), 4), 5, 0))
    c.push()
    g("XAdtReturnCtorFieldsNext", value_slots=c.add("value_slots", C(1)),
      frames=c.top(cf(4, 1, 4, False, sl(1, 1), sl(3, 1), 4)), control=ev(6), control_origin=6)
    g("XAdtEvalConstructFields", control=sched(cf(5, 0, 0, False, sl(), sl(), 6), 7, 0))
    c.push()
    g("KMachineReturnArgumentsFinal", value_slots=c.add("value_slots", C(0)), env=c.add("env", cell(10, C(0), None)),
      control={"Enter": {"function": 1, "args": sl(2, 1)}}, result={"BindOk": {"env": 1}})
    rf = c.enter(0, 10, 1)
    if dup_push:
        g("BBookkeepingResumeRestore", "HelperVisit", frames=c.cur["frames"][:-1], env_root=0, control=sched(rf, 10, 1), control_origin=7)
        c.push()
    g("KMachineEvalVariablePresent", control=ret(C(0)), result={"ScalarTypeTest": {"ok": True}})
    g("KMachineReturnCallValid", frames=c.cur["frames"][:-1], env_root=0, control_origin=7, result=None)
    g("BHelperAppendExit", "HelperExit", frames=c.top(cf(5, 0, 0, True, sl(), sl(), 6)),
      value_slots=c.add("value_slots", C(0)), result={"AppendDone": {"seq": sl(3, 1)}})
    g("XAdtReturnCtorFieldsFinal", frames=c.cur["frames"][:-1], control={"Materialize": {"type_id": 4, "ctor_id": 5, "fields": sl(3, 1)}},
      control_origin=6, result=None)
    g("XAdtHelperBuildExit", "HelperExit", data=[row(5, 0, 1)], data_fields=[C(0)], result={"BuildDone": {"value": D(0)}})
    g("XAdtMaterializeDone", control=ret(D(0)), result=None)
    g("XAdtReturnCtorFieldsNext", value_slots=c.add("value_slots", C(1), D(0)),
      frames=c.top(cf(4, 2, 0, False, sl(4, 2), sl(), 4)), control=ev(9), control_origin=9)
    g("BHelperAppendExit", "HelperExit", value_slots=c.add("value_slots", C(1), D(0), C(2)),
      frames=c.top(cf(4, 2, 0, True, sl(4, 2), sl(), 4)), control=ret(C(2)), result={"AppendDone": {"seq": sl(6, 3)}})
    g("XAdtReturnCtorFieldsFinal", frames=c.cur["frames"][:-1], control={"Materialize": {"type_id": 4, "ctor_id": 4, "fields": sl(6, 3)}},
      control_origin=4, result=None)
    g("XAdtHelperBuildExit", "HelperExit", data=c.add("data", row(4, 1, 3)), data_fields=c.add("data_fields", C(1), D(0), C(2)),
      result={"BuildDone": {"value": D(1)}})
    g("XAdtMaterializeDone", control=ret(D(1)), result=None)
    g("BHelperScalarExit", "HelperExit", result={"ScalarTypeTest": {"ok": True}})
    g("KMachineReturnCallValid", frames=c.cur["frames"][:-1], env_root=None, control_origin=1, result=None)
    g("KMachineReturnMatch", frames=[], control={"Cases": {"value": D(1), "arms": sl(0, 1), "saved_env": None}}, control_origin=0)
    g("XAdtHelperPlanEntry", "HelperEnter", helper=plan(0, "Walk", "Test", D(1), sl(), 0, None))
    g("XAdtHelperPlanWalk", "HelperVisit", helper=plan(2, "Bind", "Bind", C(1), sl(1, 1), 1, None))
    g("XAdtHelperPlanBind", "HelperVisit", env=c.add("env", cell(11, C(1), None)), helper=plan(3, "Walk", "Bind", D(1), sl(2, 2), 0, 2))
    g("XAdtHelperPlanWalk", "HelperVisit", helper=plan(3, "Bind", "Bind", C(0), sl(2, 2), 2, 2))
    g("XAdtHelperPlanBind", "HelperVisit", env=c.add("env", cell(12, C(0), 2)), helper=plan(4, "Ready", "Bind", C(0), sl(2, 2), 2, 3, 0))
    g("XAdtHelperPlanExit", "HelperExit", helper=None, result={"PlanMatched": {"arm": 0, "env": 3}})
    g("XAdtCasesPlanHit", control=sched({"ScopeF": {"saved_env": None, "origin": 0}}, 3, 3), result=None)
    c.push()
    g("KMachineEvalVariablePresent", control=ret(C(0)))
    g("KMachineReturnScope", frames=[], env_root=None, control_origin=0)
    g("KAdminPublishValue", "Publish", "Published", outcome={"Halted": {"value": C(0)}})
    if late_applied:
        g("KMachineEvalLiteralValid")
    return c


def dumps(value):
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def pin(path):
    raw = Path(path).read_bytes()
    return {"path": str(Path(path).resolve()), "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}


def write_case(root, steps, name="demo", prog=None, edit=None):
    adm = {"identity": IDENT, "bounds": BOUNDS, "view": prog or program(), "layout": LAYOUT}
    ref = {"same_as": "compile.json#/genesis/admission"}
    def bd(img, inline=False): return {"admission": {"inline": adm} if inline else ref, "image": img}
    recs = [{"record": n, "kind": "Event", "before": bd(b, n == 6), "event": event(n, r, k, d, b, a), "after": bd(a, n == 5)}
            for n, (r, k, d, b, a) in enumerate(steps)]
    last = steps[-1][4]
    recs.append({"record": len(recs), "kind": "Terminal", "before": bd(last), "after": bd(last)})
    if edit:
        edit(recs, adm)
    head = {"schema": sc.ACTUAL_SCHEMA, "case": name}
    docs = {"input.json": dict(head, source=SRC, source_profile=sc.PROFILE, configuration=CONFIG, locator=None,
                               parser_budget=BUDGET, independent_expected_cells_read=False),
            "compile.json": dict(head, name=name, provenance=provenance(), source_profile=sc.PROFILE,
                                 body_check={"status": "completed", "judgments": list(sc.JUDGMENTS)},
                                 genesis={"admission": adm, "image": steps[0][3]}, preparation=PREP, configuration=CONFIG,
                                 plans=PLANS, evaluation_performed=False, qualification="synthetic invented fixture"),
            "records.jsonl": b"".join(map(dumps, recs)), "errors.jsonl": b"",
            "final.json": dict(head, records=len(recs), events=len(steps), owner={"admission": adm, "image": last}, preparation=PREP,
                               actual_capture_checks=dict.fromkeys(("continuous", "final_equals_last", "admission_unchanged",
                                                                    "preparation_unchanged"), True),
                               runtime_path="synthetic", independent_expected_cells_read=False, unbound=["synthetic counters and IDs"])}
    d = root / name
    d.mkdir()
    for f, v in docs.items():
        (d / f).write_bytes(v if isinstance(v, bytes) else dumps(v))
    source = root / (name + ".mori")
    source.write_bytes(SRC.encode())
    return {"case": name + ".mori", "capture_case": name, "source_profile": sc.PROFILE, "source": pin(source),
            "capture": {f: pin(d / f) for f in FILES}}


def project_inputs(root, bad_edit=None):
    cases = [write_case(root, run().steps, "demo"), write_case(root, run().steps, "bad", edit=bad_edit)]
    (root / "manifest.json").write_bytes(dumps({"source_and_safe_readset": [x["source"] for x in cases]}))
    mp = pin(root / "manifest.json")
    (root / "bindings.json").write_bytes(dumps({"source_manifest": mp, "cases": cases}))
    return str(root / "bindings.json"), mp["sha256"]


def run_main(*argv):
    with mock.patch.object(sys, "argv", ["symbolic_compare.py", *argv]), \
            contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
        return sc.main()


@contextlib.contextmanager
def reads():
    seen, read_bytes, real_open = [], pathlib.Path.read_bytes, builtins.open
    def rb(self):
        seen.append(str(self))
        return read_bytes(self)
    def op(file, *a, **k):
        if isinstance(file, (str, os.PathLike)):
            seen.append(os.fspath(file))
        return real_open(file, *a, **k)
    with mock.patch.object(pathlib.Path, "read_bytes", rb), mock.patch("builtins.open", op):
        yield seen


class SyntheticFixtureTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls._tmp = tempfile.TemporaryDirectory()
        cls.root = Path(cls._tmp.name).resolve()
        cls.chain = run()
        cls.binding = write_case(cls.root, cls.chain.steps)
        cls.actual = sc.capture_case(cls.binding)

    @classmethod
    def tearDownClass(cls):
        cls._tmp.cleanup()

    def tmp(self):
        t = tempfile.TemporaryDirectory()
        self.addCleanup(t.cleanup)
        return Path(t.name).resolve()

    def capture(self, steps=None, prog=None, edit=None, **kw):
        return sc.capture_case(write_case(self.tmp(), steps or run(**kw).steps, prog=prog, edit=edit))

    def refused(self, fragment, observed, **kw):
        r = self.capture(**kw)
        self.assertEqual(r["status"], "NOT_ESTABLISHED")
        self.assertTrue(r["gaps"] and r["gaps"][0]["path"])
        if fragment:
            self.assertIn(fragment, r["gaps"][0]["path"])
        calls, built, matched = observed
        pr = r["projection"]
        self.assertEqual(pr["calls"], EXPECTED["calls"][:calls])
        self.assertEqual(pr["constructors"]["materialize_order"], EXPECTED["constructors"]["materialize_order"][:built])
        self.assertEqual(pr["matches"], EXPECTED["matches"][:matched])
        return r

    def test_owner_layout_resolves_builtins_by_position_not_names_padding(self):
        prog = program()
        self.assertEqual(prog["names"][0], "build")
        p = sc.Projection(prog, provenance(), SRC)
        self.assertEqual([p.typ(i) for i in range(4)], [{"builtin": n} for n in ("UInt", "Bool", "List", "Tree")])
        self.assertEqual(p.typ(4), BOX)
        self.assertEqual(p.value(C(1), image()), T)

    def test_owner_layout_refuses_bad_padding_decl_placement_and_builtin_ctor_slice(self):
        edits = {"nonzero builtin padding": lambda p: p["types"][1].update(name_id=5),
                 "decl Some below four": lambda p: p["types"][1].update(decl=0),
                 "decl None at four": lambda p: p["types"][4].update(decl=None),
                 "wrong builtin ctor slice": lambda p: p["types"][2].update(ctor_len=1)}
        for label, edit in edits.items():
            with self.subTest(label):
                prog = program()
                edit(prog)
                with self.assertRaises(sc.Gap):
                    sc.Projection(prog, provenance(), SRC)

    def test_generic_application_owner_field_type_and_cycle_refusals(self):
        p = sc.Projection(program(), provenance(), SRC)
        opt = {"named": "Opt", "arguments": [{"builtin": "UInt"}]}
        self.assertEqual(p.typ(5), opt)
        self.assertEqual(p.ctor(5, 6)[:2], (opt, "Some"))
        with self.assertRaises(sc.Gap):
            p.ctor(4, 6)
        edits = {"field type": lambda p: p["ctor_fields"][9].update(type_id=1),
                 "cyclic application": lambda p: p.update(type_args=[5]),
                 "wrong owner": lambda p: p["constructors"][6].update(owning_type=4)}
        for label, edit in edits.items():
            with self.subTest(label):
                prog = program()
                edit(prog)
                with self.assertRaises(sc.Gap):
                    sc.Projection(prog, provenance(), SRC)

    def test_exact_uint128_bool_values_and_json_type_distinctions(self):
        p, big = sc.Projection(program(), provenance(), SRC), str(2 ** 128 - 1)
        img = image(data=[{"U128": {"n": big}}, {"U128": {"n": "00"}}, {"U128": {"n": str(2 ** 128)}},
                          {"U128": {"n": 7}}, {"Bool": {"b": 1}}])
        self.assertEqual(p.value(D(0), img), {"UInt": big})
        for i in range(1, 5):
            with self.subTest(i), self.assertRaises(sc.Gap):
                p.value(D(i), img)
        changes = (("$/final_value/UInt", lambda e: e.__setitem__("final_value", {"UInt": 7})),
                   ("$/matches/0/bindings/0/value/Bool", lambda e: e["matches"][0]["bindings"][0].__setitem__("value", {"Bool": 1})))
        for path, change in changes:
            e = copy.deepcopy(EXPECTED)
            change(e)
            r = sc.compare_case(self.actual, e)
            self.assertEqual((r["status"], r["first_divergence"]["path"], r["first_divergence"]["reason"]),
                             ("FAIL", path, "JSON type differs"))

    def test_env_first_visible_shadowing_and_cycle_refusal(self):
        p = sc.Projection(program(), provenance(), SRC)
        img = image(env=[cell(9, C(0), None), cell(10, C(1), 0), cell(9, C(2), 1)])
        self.assertEqual(p.env(2, img), {"k": U9, "x": T})
        img["env"][0]["previous"] = 2
        with self.assertRaises(sc.Gap):
            p.env(2, img)

    def test_repeated_source_site_gets_distinct_dynamic_construction_identities(self):
        p = sc.Projection(program(), provenance(), SRC)
        frame = cf(4, 0, 1, False, sl(), sl(2, 2), 4, saved=None)
        before = image(control=ev(4), control_origin=4)
        for n in range(2):
            after = image(**dict(copy.deepcopy(before), control=sched(frame, 5, None)))
            p.step(before, event(n, "XAdtEvalConstructFields", "LegacyComplete", "Applied", before, after), after, n)
            before = image(**dict(copy.deepcopy(after), frames=after["frames"] + [frame], control=ev(4)))
        opened = p.out["constructors"]["open_order"]
        self.assertEqual([(o["invocation"], o["source"]) for o in opened], [(0, src(B)), (1, src(B))])
        self.assertEqual([x["construction_invocation"] for x in p.pending(before)], [0, 1])

    def test_full_lifecycle_capture_projects_exact_five_fields_and_compares_pass(self):
        a, at = self.actual, self.chain.at
        self.assertEqual((a["status"], a["gaps"], a["producer_errors"], a["case"]), ("ESTABLISHED", [], [], "demo.mori"))
        for field in sc.SEMANTIC:
            self.assertEqual(a["projection"][field], EXPECTED[field], field)
        self.assertEqual(sc.compare_case(a, EXPECTED),
                         {"status": "PASS", "fields": dict.fromkeys(sc.SEMANTIC, "PASS"), "first_divergence": None})
        call = [r for r in a["association_ledger"] if r.get("association") == "call" and r.get("invocation") == 1]
        self.assertEqual(len(call), 1)
        self.assertEqual((call[0]["entry_bind_record"], call[0]["body_entry_record"], call[0]["return_record"]),
                         (at["KMachineEnterBind#1"], at["KMachinePushAccept#4"], at["KMachineReturnCallValid#0"]))
        for entry in a["association_ledger"]:
            if entry.get("association") == "value":
                self.assertIn("record", entry)

    def test_nested_constructor_checkpoint_outermost_first_with_prefixes_suffixes_and_contexts(self):
        pr = self.actual["projection"]
        cps, opened = pr["pending_checkpoints"], pr["constructors"]["open_order"]
        self.assertEqual([c["at_call"] for c in cps], [0, 1])
        self.assertEqual(cps[0]["pending"], [])
        outer, inner = cps[1]["pending"]
        self.assertEqual([(x["construction_invocation"], x["source"]) for x in (outer, inner)], [(0, src(B)), (1, src(B, 1))])
        for item in (outer, inner):
            self.assertEqual(len(item["completed_fields"]), item["current_field"])
            self.assertEqual(item["saved_environment"], K)
        self.assertEqual(outer["completed_fields"], opened[0]["field_values"][:1])
        self.assertEqual(opened[0]["field_sources"][outer["current_field"] + 1:], [src(B, 2)])
        self.assertEqual(opened[1]["field_sources"][inner["current_field"] + 1:], [])
        self.assertEqual((cps[1]["caller_environment"], cps[1]["callee_parameters"]), (K, X))
        self.assertEqual([o["in_call"] for o in opened], [0, 0])

    def test_mutation_global_arm_offset_refused_with_prior_observations(self):
        self.refused("match.arm", (2, 2, 0), hooks={"XAdtHelperPlanExit#0": lambda a: a["result"]["PlanMatched"].update(arm=1)})

    def test_mutation_bool_value_fails_comparison_at_first_json_divergence(self):
        prog = program()
        prog["constants"][1]["Bool"]["b"] = False
        a = self.capture(prog=prog)
        self.assertEqual(a["status"], "ESTABLISHED")
        r = sc.compare_case(a, EXPECTED)
        self.assertEqual(r["fields"], {"final_value": "PASS", "calls": "PASS", "matches": "FAIL",
                                       "constructors": "FAIL", "pending_checkpoints": "FAIL"})
        d = r["first_divergence"]
        self.assertEqual((r["status"], d["path"], d["reason"], d["actual"], d["expected"]),
                         ("FAIL", "$/matches/0/bindings/0/value/Bool", "value differs", "false", "true"))

    def test_mutation_duplicated_body_entry_push_refused(self):
        r = self.refused("call", (2, 0, 0), dup_push=True)
        self.assertEqual(r["projection"]["pending_checkpoints"], EXPECTED["pending_checkpoints"])

    def test_mutation_swapped_materialization_field_order_refused(self):
        def swap(a):
            a["value_slots"][6], a["value_slots"][7] = a["value_slots"][7], a["value_slots"][6]
        self.refused("construct.value", (2, 1, 0), hooks={"BHelperAppendExit#1": swap})

    def test_mutation_applied_after_published_refused(self):
        self.refused(None, (2, 2, 1), late_applied=True)

    def test_changed_inline_admission_refused(self):
        def edit(recs, adm):
            bad = copy.deepcopy(adm)
            bad["view"]["names"][0] = "changed"
            recs[5]["after"]["admission"] = recs[6]["before"]["admission"] = {"inline": bad}
        self.refused("admission", (0, 0, 0), edit=edit)

    def test_truncated_capture_ending_on_intermediate_return_is_not_success(self):
        cut = self.chain.at["KMachineReturnCallValid#1"] + 1
        r = self.refused("final", (2, 2, 0), steps=self.chain.steps[:cut])
        self.assertIsNone(r["projection"]["final_value"])
        self.assertEqual(sc.compare_case(r, EXPECTED)["status"], "NOT_ESTABLISHED")

    def test_checkpoints_selected_by_integer_at_call_and_whole_calls_compared(self):
        e = copy.deepcopy(EXPECTED)
        e["pending_checkpoints"] = e["pending_checkpoints"][1:]
        self.assertEqual(sc.compare_case(self.actual, e)["status"], "PASS")
        e["pending_checkpoints"][0]["at_call"] = 5
        r = sc.compare_case(self.actual, e)
        self.assertEqual((r["status"], r["first_divergence"]["reason"]), ("FAIL", "expected milestone absent/ambiguous"))
        e["pending_checkpoints"][0]["at_call"] = "1"
        with self.assertRaises(sc.Gap):
            sc.compare_case(self.actual, e)
        e["pending_checkpoints"] = copy.deepcopy([EXPECTED["pending_checkpoints"][1]] * 2)
        with self.assertRaises(sc.Gap):
            sc.compare_case(self.actual, e)
        e = copy.deepcopy(EXPECTED)
        e["calls"].append(copy.deepcopy(e["calls"][0]))
        r = sc.compare_case(self.actual, e)
        self.assertEqual(r["fields"], dict(dict.fromkeys(sc.SEMANTIC, "PASS"), calls="FAIL"))
        self.assertEqual((r["first_divergence"]["path"], r["first_divergence"]["reason"]), ("$/calls", "array length differs"))

    def test_early_nul_capture_path_yields_located_case_gap(self):
        b = copy.deepcopy(self.binding)
        for name, spec in b["capture"].items():
            spec["path"] = str(self.root) + "/nul\x00dir/demo/" + name
        r = sc.capture_case(b)
        self.assertEqual((r["status"], r["case"]), ("NOT_ESTABLISHED", "demo.mori"))
        self.assertTrue(r["gaps"] and r["gaps"][0]["path"])

    def test_main_project_keeps_valid_case_beside_attribute_error_case(self):
        root, idx = self.tmp(), self.chain.at["XAdtCasesPlanHit#0"]
        bind, sha = project_inputs(root, lambda recs, adm: recs[idx]["event"].update(rule_id=7))
        with mock.patch.object(sc, "INPUT_SHA", sha):
            self.assertEqual(run_main("project", "--bindings", bind, "--outdir", str(root / "phase-A")), 2)
        cases = json.loads((root / "phase-A" / "ACTUAL-PROJECTION.json").read_text())["cases"]
        good, bad = cases["demo.mori"], cases["bad.mori"]
        self.assertEqual(good["status"], "ESTABLISHED")
        for field in sc.SEMANTIC:
            self.assertEqual(good["projection"][field], EXPECTED[field], field)
        self.assertEqual(bad["status"], "NOT_ESTABLISHED")
        self.assertTrue(bad["gaps"] and bad["gaps"][0]["path"])
        self.assertEqual(bad["projection"]["calls"], EXPECTED["calls"])
        receipt = json.loads((root / "phase-A" / "PHASE-A-RECEIPT.json").read_text())
        self.assertEqual(receipt["case_status"], {"demo.mori": "ESTABLISHED", "bad.mori": "NOT_ESTABLISHED"})
        self.assertIs(receipt["expected_read"], False)
        self.assertFalse((root / "phase-A" / "FAILURE.json").exists())

    def test_main_failure_receipts_for_top_level_attribute_and_value_errors(self):
        root = self.tmp()
        (root / "manifest.json").write_bytes(dumps({"source_and_safe_readset": [{"path": 5, "bytes": 0, "sha256": "0"}]}))
        mp = pin(root / "manifest.json")
        (root / "bindings.json").write_bytes(dumps({"source_manifest": mp, "cases": []}))
        for label, path, sha in (("attribute", str(root / "bindings.json"), mp["sha256"]),
                                 ("value", str(root / "nul\x00bindings.json"), sc.INPUT_SHA)):
            with self.subTest(label):
                out = root / ("fail-" + label)
                with mock.patch.object(sc, "INPUT_SHA", sha):
                    self.assertEqual(run_main("project", "--bindings", path, "--outdir", str(out)), 2)
                failure = json.loads((out / "FAILURE.json").read_text())
                self.assertEqual((failure["status"], failure["command"]), ("NOT_ESTABLISHED", "project"))
                self.assertTrue(failure["reason"])
                self.assertEqual(os.listdir(out), ["FAILURE.json"])

    def test_project_refuses_existing_output_directory_without_overwrite(self):
        root = self.tmp()
        bind, sha = project_inputs(root)
        out = root / "phase-A"
        out.mkdir()
        (out / "KEEP").write_bytes(b"prior")
        with mock.patch.object(sc, "INPUT_SHA", sha):
            self.assertEqual(run_main("project", "--bindings", bind, "--outdir", str(out)), 2)
        self.assertEqual((os.listdir(out), (out / "KEEP").read_bytes()), (["KEEP"], b"prior"))

    def test_checked_returns_hashed_buffer_from_single_read(self):
        p = self.tmp() / "pinned.bin"
        p.write_bytes(b"hashed-bytes")
        spec, calls = pin(p), []
        def nxt():
            calls.append(1)
            return b"hashed-bytes" if len(calls) == 1 else b"second-read"
        with mock.patch.object(pathlib.Path, "read_bytes", lambda self: nxt()), \
                mock.patch.object(pathlib.Path, "open", lambda self, *a, **k: io.BytesIO(nxt())), \
                mock.patch("builtins.open", lambda *a, **k: io.BytesIO(nxt())):
            out = sc.checked(spec)
        self.assertEqual((out, len(calls)), (b"hashed-bytes", 1))

    def test_checked_refuses_changed_file(self):
        p = self.tmp() / "pinned.bin"
        p.write_bytes(b"original")
        spec = pin(p)
        self.assertEqual(sc.checked(spec), b"original")
        p.write_bytes(b"changed!")
        with self.assertRaises(sc.Gap):
            sc.checked(spec)

    def test_compare_reads_expected_only_after_projection_checks_with_pass_control(self):
        root = self.tmp()
        def put(name, value):
            (root / name).write_bytes(dumps(value))
            return pin(root / name)
        proj = put("ACTUAL-PROJECTION.json", {"schema": "nominal-actual-symbolic-projection/1", "cases": {"demo.mori": self.actual},
                                              "expected_read": False, "scope": sc.LIMITS})
        receipt = put("PHASE-A-RECEIPT.json", {"schema": "nominal-passive-phase-a-receipt/1", "expected_read": False,
                                               "inputs": self.actual["input_pins"], "outputs": [proj]})
        expected = put("expected.json", {"schema": sc.SCHEMA, "source_pins": [], "normative_pins": [], "author": "synthetic",
                                         "scope": "synthetic", "cases": {"demo.mori": EXPECTED}})
        safe = put("SAFE.json", {"private_expected": expected, "case_keys": ["demo.mori"]})
        frozen = put("FREEZE.json", {"schema": "root-nominal-actual-projection-freeze/1",
                                     "actual_and_projection_frozen_before_expected_read": True,
                                     "phase_a_receipt": receipt, "projection": proj, "safe_bindings": safe})
        def go(out):
            with mock.patch.object(sc, "SAFE_SHA", safe["sha256"]), reads() as seen, contextlib.redirect_stdout(io.StringIO()):
                try:
                    return sc.compare_command(types.SimpleNamespace(frozen=frozen["path"], outdir=str(root / out))), seen
                except sc.Gap as exc:
                    return exc, seen
        rc, seen = go("pass")
        self.assertEqual(rc, 0)
        comparison = json.loads((root / "pass" / "COMPARISON.json").read_text())
        self.assertEqual(comparison["cases"]["demo.mori"]["status"], "PASS")
        first = seen.index(expected["path"])
        for spec in (frozen, receipt, proj, safe):
            self.assertIn(spec["path"], seen[:first])
        Path(proj["path"]).write_bytes(Path(proj["path"]).read_bytes() + b" ")
        exc, seen = go("fail")
        self.assertIsInstance(exc, sc.Gap)
        self.assertEqual(exc.path, proj["path"])
        self.assertNotIn(expected["path"], seen)
        self.assertFalse((root / "fail").exists())


if __name__ == "__main__":
    unittest.main()
