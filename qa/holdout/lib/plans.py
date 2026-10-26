"""UIREQ-024 / UIREQ-031 / UIREQ-033 holdout checks (evaluator-owned).

Expectations come from the spec text, interfaces.md ("Witness, constraints and exports",
"Bounded static residualization"), demonstrators.md D23/D30/D31/D33/D35 and the optimization
synthesis (docs/research/sherlock/optimization/10), never from implementation output.

Binding rule (same evaluator judgment as U023): every artifact must bind to the REAL reference
machine of this checkout. The independent comparison source is the K-A reference stream that
`mori simulate --reference-slices 1` already publishes (sherlock-k-a/1 stepReference); a report
generated from a hand-written rule table never matches it and fails.

The flag spellings are read from the evaluator contracts qa/holdout/contracts/U0NN.json, so a
spelling change is a CONTRACT CHANGE, not a holdout edit. Helpers (runx, _check_slices, _flat,
need_executed, RepoOps, ...) are bound from uireq.py at import time (uireq sets `U`).
"""
import hashlib
import json
import os
import re
import shutil
import tempfile
from pathlib import Path

import engine

Fail = engine.Fail
U = None  # the uireq module, bound by uireq.py


def need(cond, msg):
    if not cond:
        raise Fail(msg)


def canon_id(obj, id_key):
    """sha256 of canonical JSON (sorted keys, no spaces, UTF-8) of obj without id_key."""
    body = {k: v for k, v in obj.items() if k != id_key}
    raw = json.dumps(body, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()
    return hashlib.sha256(raw).hexdigest()


HEX64 = re.compile(r"[0-9a-f]{64}")


def _contract(cid):
    p = engine.HOLDOUT / "contracts" / f"{cid}.json"
    need(p.is_file(), f"not-implemented: no qa/holdout/contracts/{cid}.json")
    return json.loads(p.read_text())


def _flags(con, key, **subst):
    argv = (con.get("flags") or {}).get(key)
    need(isinstance(argv, list) and all(isinstance(a, str) for a in argv),
         f"contract {con.get('id')}: no flags.{key} argv")
    return [a.format(**subst) for a in argv]


def _halted_value(r, label, expect=None):
    U.need_executed(r, label)
    k, _ = engine.outcome_kind(r)
    need(k == "Halted", f"{label}: not halted: {r.outcome}")
    v = engine.decode_value(r)
    if expect is not None:
        need(v == expect, f"{label}: result {v!r} != {expect!r}")
    return v


def _refused_any(c, extra, label):
    """An invalid selection is refused: no completed execution carrying the new report."""
    r = U.runx(c, extra=extra)
    ok = r.returncode != 0 and not (r.doc and r.doc.get("execution", {}).get("completed"))
    need(ok, f"{label}: invalid selection {extra} was not refused (rc={r.returncode})")


# =================================================================================== U033

FAMILIES = ("Schedule", "Scalar", "CallBind", "FrameReturn", "DataMatch")
FOLDABLE = {"code", "declarations", "layout", "family", "source-occurrence-ids", "operand-slices"}
DYNAMIC = {"control", "data", "environment", "frames", "depth", "fuel", "work", "helper-cursor",
           "capacity-checks", "outcome"}
BLOCK_KINDS = {"static-dispatch", "dynamic-branch", "runtime-call", "return", "step", "generic-dispatch"}
IR_KEYS = ("ir_id", "reference_definition_id", "profile_id", "program_image_id", "layout_version", "blocks",
           "dynamic_inputs", "static_bindings", "rule_boundary_map", "source_occurrence_map",
           "generic_fallbacks", "omissions")
REALIZATION_KIND = {"universal": "empty", "family": "family", "per-program": "program-image"}


def _res_flags(con, real, family=None, limit=None):
    if real == "family":
        argv = _flags(con, "family", family=family)
    else:
        argv = _flags(con, real)
    if limit is not None:
        argv += _flags(con, "size_limit", n=str(limit))
    return argv


def _residual_run(c, con, real, label, family=None, limit=None, fuel=None, stack=None, source=None):
    r = U.runx(c, extra=_res_flags(con, real, family, limit), fuel=fuel, stack=stack, source=source)
    need(r.doc is not None and isinstance(r.doc.get("residual"), dict),
         f"{label}: no residual report (rc={r.returncode}, out={r.raw[:200]!r}, err={r.stderr[:200]!r})")
    return r, r.doc["residual"]


def _ref_rule_charges(recs):
    charges = {}
    for rec in recs:
        charges.setdefault(rec["rule_start_line"], set()).add(rec["charge"])
    return charges


def _check_ir(res, real, family, label, ref_recs, limit=None):
    need(res.get("format") == "moriarty-sherlock-residual-step/1", f"{label}: residual format {res.get('format')!r}")
    need(res.get("realization") == real, f"{label}: realization {res.get('realization')!r} != {real}")
    cfg = res.get("config") or {}
    need(cfg.get("static_mask") == REALIZATION_KIND[real], f"{label}: static_mask {cfg.get('static_mask')!r}")
    need(cfg.get("family") == (family if real == "family" else None), f"{label}: config.family {cfg.get('family')!r}")
    lim = cfg.get("residual_size_limit")
    need(isinstance(lim, str) and lim.isdigit() and int(lim) > 0, f"{label}: residual_size_limit {lim!r}")
    if limit is not None:
        need(int(lim) == limit, f"{label}: residual_size_limit {lim} != requested {limit}")
    ir = res.get("ir")
    need(isinstance(ir, dict), f"{label}: no ir")
    miss = [k for k in IR_KEYS if k not in ir]
    need(not miss, f"{label}: ResidualStepIR lacks {miss}")
    need(HEX64.fullmatch(str(ir["ir_id"])) is not None and ir["ir_id"] == canon_id(ir, "ir_id"),
         f"{label}: ir_id is not the sha256 of the canonical IR")
    need(ir["reference_definition_id"] == "sherlock-k-a/1", f"{label}: reference_definition_id {ir['reference_definition_id']!r}")
    if real == "per-program":
        need(HEX64.fullmatch(str(ir["program_image_id"])) is not None, f"{label}: per-program IR has no program_image_id")
    else:
        need(ir["program_image_id"] is None, f"{label}: {real} IR is bound to a program image")
    # static versus dynamic
    sb = ir["static_bindings"]
    kinds = set(sb.get("kinds") or [])
    need(kinds <= FOLDABLE, f"{label}: static binding kinds {sorted(kinds - FOLDABLE)} are not immutable code/declaration/layout facts")
    allowed = {"universal": {"layout"}, "family": {"layout", "family"}, "per-program": FOLDABLE}[real]
    need(kinds <= allowed, f"{label}: {real} binds {sorted(kinds - allowed)} statically")
    if real == "family":
        need("family" in kinds and sb.get("family") == family, f"{label}: family binding {sb.get('family')!r}")
    if real == "per-program":
        need("code" in kinds and sb.get("program_image_id") == ir["program_image_id"], f"{label}: per-program binds no code image")
    dyn = set(ir["dynamic_inputs"])
    need(DYNAMIC <= dyn, f"{label}: dynamic_inputs lacks {sorted(DYNAMIC - dyn)}")
    blocks = ir["blocks"]
    need(isinstance(blocks, list) and blocks, f"{label}: no residual blocks")
    need(len(blocks) <= int(lim), f"{label}: {len(blocks)} blocks exceed residual_size_limit {lim}")
    ids = [b.get("block_id") for b in blocks]
    need(len(set(ids)) == len(ids) and all(isinstance(i, str) and i for i in ids), f"{label}: block ids not unique")
    idset = set(ids)
    for b in blocks:
        bl = f"{label}: block {b.get('block_id')}"
        need(b.get("kind") in BLOCK_KINDS, f"{bl}: kind {b.get('kind')!r}")
        fo = set(b.get("folded") or [])
        need(fo <= FOLDABLE, f"{bl}: folds {sorted(fo - FOLDABLE)}")
        need(fo <= kinds | {"source-occurrence-ids", "operand-slices"} if real == "per-program" else fo <= kinds,
             f"{bl}: folds {sorted(fo)} beyond the static mask {sorted(kinds)}")
        succ = b.get("successors")
        need(isinstance(succ, list) and set(succ) <= idset, f"{bl}: successors {succ} name no block")
        need(isinstance(b.get("rules"), list), f"{bl}: no rules list")
        if b["kind"] == "dynamic-branch":
            need(len(set(succ)) >= 2, f"{bl}: a dynamic condition keeps fewer than two successors")
            need(b.get("selector") in DYNAMIC, f"{bl}: branch selector {b.get('selector')!r} is not dynamic")
        if b["kind"] == "runtime-call":
            need(b.get("unrolled") is False, f"{bl}: call is unrolled")
            need(b.get("frame_check") == "dynamic", f"{bl}: call frame/cap check {b.get('frame_check')!r} not dynamic")
    # the IR must cover every reference rule the program actually exercises, with its charge
    rbm = ir["rule_boundary_map"]
    need(isinstance(rbm, list), f"{label}: rule_boundary_map not a list")
    mapped = {}
    for m in rbm:
        need(m.get("block_id") in idset, f"{label}: rule_boundary_map row {m} names no block")
        mapped.setdefault(str(m.get("rule_start_line")), set()).add(str(m.get("charge")))
    for rule, ch in _ref_rule_charges(ref_recs).items():
        need(rule in mapped, f"{label}: reference rule {rule} (exercised by this program) has no residual boundary")
        if len(ch) == 1:
            need(mapped[rule] == ch, f"{label}: rule {rule} charge {sorted(mapped[rule])} != reference {sorted(ch)}")
    need(isinstance(ir["source_occurrence_map"], list), f"{label}: source_occurrence_map not a list")
    if real == "per-program":
        need(ir["source_occurrence_map"], f"{label}: per-program IR maps no source occurrence")
    gf = ir["generic_fallbacks"]
    need(isinstance(gf, list) and all(g.get("reason") == "residual-size-limit" for g in gf),
         f"{label}: generic_fallbacks {gf!r}")
    wl = res.get("worklist") or {}
    need(wl.get("fallback_used") is bool(gf), f"{label}: worklist.fallback_used {wl.get('fallback_used')!r} vs {len(gf)} fallbacks")
    if gf:
        need(any(b["kind"] == "generic-dispatch" for b in blocks), f"{label}: fallbacks without a generic-dispatch block")
    return ir


def _residual_exec(res, label, fuel, stack, ref):
    ex = res.get("execution")
    need(isinstance(ex, dict) and ex.get("format") == "moriarty-sherlock-reference-slices/1",
         f"{label}: residual.execution is not an executed reference-slices/1 stream")
    need(ex.get("reference_schedule_id") == "sherlock-k-a/1", f"{label}: execution schedule {ex.get('reference_schedule_id')!r}")
    recs = U._check_slices(ex, 1, fuel, stack, label)
    need(U._flat(ex) == U._flat(ref), f"{label}: residual StepRecord stream differs from the reference stream "
                                      f"(rules/occurrences/charges/frame checks/stops)")
    need(ex["outcome"] == ref["outcome"], f"{label}: residual outcome {ex['outcome']} != reference {ref['outcome']}")
    return recs


def _sig(ir):
    return sorted((b["kind"], tuple(sorted(str(x) for x in b["rules"]))) for b in ir["blocks"])


def custom_residual_masks(c):
    con = _contract("U033")
    ids = {}
    for src in c["sources"]:
        sc = {**c, "source": src["source"]}
        base = U.runx(sc)
        _halted_value(base, f"{src['label']} host run", src.get("expect"))
        fuel0 = int(base.execution["image"]["initial_fuel"])
        stack0 = int(base.execution["image"]["stack_limit"])
        _, ref = U._slices(sc, 1)
        ref_recs = U._check_slices(ref, 1, fuel0, stack0, f"{src['label']} reference")
        need(ref["outcome"] == {"kind": "Done", "value": src["expect_value"]}, f"{src['label']}: reference outcome {ref['outcome']}")
        work = int(ref["reference_work"])
        variants = {"exact": (work, stack0), "short": (work - 1, stack0), "stack1": (fuel0, 1)}
        refs = {}
        for name, (f, s) in variants.items():
            _, rv = U._slices(sc, 1, fuel=f, stack=s)
            refs[name] = rv
        need(refs["exact"]["outcome"]["kind"] == "Done" and refs["short"]["outcome"]["kind"] == "Exhausted"
             and refs["stack1"]["outcome"] == {"kind": "Rejected", "code": "4"},
             f"{src['label']}: reference bound variants are not Done/Exhausted/Rejected(4)")
        masks = [("universal", None)] + [("family", f) for f in FAMILIES] + [("per-program", None)]
        for real, fam in masks:
            label = f"{src['label']} {real}{'/' + fam if fam else ''}"
            r, res = _residual_run(sc, con, real, label, family=fam)
            U.need_executed(r, label)
            need(r.execution["summary"] == base.summary, f"{label}: residualization changed the host run")
            ir = _check_ir(res, real, fam, label, ref_recs)
            _residual_exec(res, label, fuel0, stack0, ref)
            for name, (f, s) in variants.items():
                rl = f"{label} {name}"
                _, rv = _residual_run(sc, con, real, rl, family=fam, fuel=f, stack=s)
                need(rv.get("ir", {}).get("ir_id") == ir["ir_id"],
                     f"{rl}: the residual IR depends on fuel/stack (no depth/fuel specialization allowed)")
                _residual_exec(rv, rl, f, s, refs[name])
            ids[(src["label"], real, fam)] = (ir["ir_id"], ir["program_image_id"], ir)
        # bounded worklist: a tiny residual-size limit falls back to generic dispatch, no exhaustion
        for real in ("universal", "per-program"):
            label = f"{src['label']} {real} limit {c['tiny_limit']}"
            r, res = _residual_run(sc, con, real, label, limit=c["tiny_limit"])
            U.need_executed(r, label)
            ir = _check_ir(res, real, None, label, ref_recs, limit=c["tiny_limit"])
            need(ir["generic_fallbacks"], f"{label}: residual-size limit reached without a generic-dispatch fallback")
            _residual_exec(res, label, fuel0, stack0, ref)
        for bad, why in ((_flags(con, "universal") + _flags(con, "size_limit", n="0"), "zero size limit"),
                         (_flags(con, "family", family="NoSuchFamily"), "unknown family")):
            _refused_any(sc, bad, f"{src['label']} {why}")
    labels = [s["label"] for s in c["sources"]]
    a, b = labels[0], labels[1]
    need(ids[(a, "universal", None)][0] == ids[(b, "universal", None)][0],
         "universal residual differs between the two programs (an empty static mask binds no program)")
    for f in FAMILIES:
        need(ids[(a, "family", f)][0] == ids[(b, "family", f)][0],
             f"family {f} residual differs between the two programs (a family mask binds no program)")
    need(ids[(a, "per-program", None)][1] != ids[(b, "per-program", None)][1],
         "per-program residuals of two different programs share one program_image_id")
    # no recursion unrolling or depth specialization: same function table, deeper entry
    deep = c["depth_variant"]
    sc = {**c, "source": deep["source"]}
    base = U.runx(sc)
    _halted_value(base, f"{deep['label']} host run", deep.get("expect"))
    _, ref = U._slices(sc, 1)
    ref_recs = U._check_slices(ref, 1, int(base.execution["image"]["initial_fuel"]),
                               int(base.execution["image"]["stack_limit"]), f"{deep['label']} reference")
    r, res = _residual_run(sc, con, "per-program", deep["label"])
    ir = _check_ir(res, "per-program", None, deep["label"], ref_recs)
    shallow = ids[(deep["compare_to"], "per-program", None)][2]
    need(_sig(ir) == _sig(shallow), f"{deep['label']}: per-program residual block structure changes with the recursion depth "
                                    f"({len(ir['blocks'])} vs {len(shallow['blocks'])} blocks): recursion was unrolled or specialized")
    need(any(bk["kind"] == "runtime-call" for bk in ir["blocks"]), f"{deep['label']}: recursion is not a runtime call")
    _residual_exec(res, deep["label"], int(base.execution["image"]["initial_fuel"]),
                   int(base.execution["image"]["stack_limit"]), ref)


# =================================================================================== U031

CAPS = ("constraint-json", "structured-witness", "native-witness-closure", "lookup", "interaction", "custom-gate")
CAP_FIELD = {"structured-witness": "structured_witness_ir", "native-witness-closure": "native_closures",
             "lookup": "lookup", "interaction": "interaction", "custom-gate": "custom_gates"}
BC_KEYS = ("backend_id", "source_membership", "field_id", "formats", "constraint_operations", "witness_operations",
           "lookup", "interaction", "custom_gates", "structured_witness_ir", "native_closures", "field_layouts")
STATUS = ("Specified", "Implemented", "Unsupported")
WITNESS_KEYS = ("nodes", "stable_topological_order", "witness_program", "input_bindings")


def _select_step(ref, sel, label):
    for s in ref["slices"]:
        st, en = s["start"], s["end"]
        if "end_control" in sel and en.get("control") != sel["end_control"]:
            continue
        if "start_control_prefix" in sel and not str(st.get("control") or "").startswith(sel["start_control_prefix"]):
            continue
        if "top_frame_prefix" in sel and not (st.get("frames") and str(st["frames"][0]).startswith(sel["top_frame_prefix"])):
            continue
        return s["records"][0]
    raise Fail(f"{label}: the reference stream has no step matching {sel} (harness/program drift)")


def _typed(xs, label):
    need(isinstance(xs, list) and all(isinstance(x, dict) and x.get("name") and x.get("type") for x in xs),
         f"{label}: not an ordered typed list")


def _check_plans(ep, rec, label, want_op):
    need(ep.get("format") == "moriarty-sherlock-export-plans/1", f"{label}: export_plans format {ep.get('format')!r}")
    st = ep.get("step") or {}
    need(str(st.get("position")) == str(rec["position"]) and str(st.get("rule_start_line")) == str(rec["rule_start_line"])
         and st.get("source_control") == rec["source_control"] and st.get("profile_id") == "sherlock-k-a/1",
         f"{label}: step {st} is not reference record {rec['position']} (rule {rec['rule_start_line']})")
    om = ep.get("operation_manifest") or {}
    for k in ("manifest_id", "reference_definition_id", "profile_id", "layout_version", "operations", "omissions"):
        need(k in om, f"{label}: operation_manifest lacks {k}")
    need(om["manifest_id"] == canon_id(om, "manifest_id"), f"{label}: manifest_id is not the sha256 of the manifest")
    ops = om["operations"]
    need(isinstance(ops, list) and ops, f"{label}: empty OperationManifest")
    opids = [o.get("operation_id") for o in ops]
    need(len(set(opids)) == len(opids) and all(opids), f"{label}: operation ids not unique")
    orders = [o.get("ordering") for o in ops]
    need(all(isinstance(x, int) for x in orders) and len(set(orders)) == len(orders), f"{label}: operation ordering not distinct ints")
    for o in ops:
        ol = f"{label}: operation {o.get('operation_id')}"
        need(isinstance(o.get("op"), str) and o["op"], f"{ol}: no typed operation name")
        _typed(o.get("inputs"), f"{ol} inputs")
        _typed(o.get("outputs"), f"{ol} outputs")
        need(isinstance(o.get("source_locators"), list) and o["source_locators"], f"{ol}: no source locator")
        need(o.get("component_membership") in ("source", "component", "source+component"), f"{ol}: component_membership {o.get('component_membership')!r}")
        need(isinstance(o.get("witness_dependencies"), list) and set(o["witness_dependencies"]) <= set(opids), f"{ol}: witness_dependencies")
        ss = o.get("supported_status") or {}
        need(ss.get("kind") in STATUS, f"{ol}: supported_status {ss!r}")
        if ss["kind"] == "Unsupported":
            need(ss.get("reason"), f"{ol}: Unsupported without a reason")
        need("witness_program_kind" in o and "constraint_component_locator" in o, f"{ol}: witness/constraint locators missing")
    need(any(o["op"] == want_op for o in ops), f"{label}: the step's manifest has no {want_op} operation "
                                              f"(ops: {sorted({o['op'] for o in ops})})")
    need(isinstance(om["omissions"], list), f"{label}: manifest omissions not listed")
    cp, wp = ep.get("constraint_plan") or {}, ep.get("witness_plan") or {}
    for k in ("plan_id", "operation_manifest_id", "layout_version", "field_id", "ordered_public_inputs",
              "ordered_private_inputs", "ordered_outputs", "components", "required_capabilities", "omissions"):
        need(k in cp, f"{label}: ConstraintPlan lacks {k}")
    for k in ("plan_id", "operation_manifest_id", "layout_version", "input_bindings", "nodes",
              "stable_topological_order", "output_bindings", "slot_counts", "program_kind"):
        need(k in wp, f"{label}: WitnessPlan lacks {k}")
    need(cp["plan_id"] == canon_id(cp, "plan_id") and wp["plan_id"] == canon_id(wp, "plan_id"), f"{label}: plan ids are not content hashes")
    need(len({cp["plan_id"], wp["plan_id"], om["manifest_id"]}) == 3, f"{label}: ConstraintPlan and WitnessPlan are not separate artifacts")
    need(cp["operation_manifest_id"] == wp["operation_manifest_id"] == om["manifest_id"], f"{label}: plans do not share the manifest")
    covered = set()
    for comp in cp["components"]:
        need(comp.get("operation_id") in opids and comp.get("component_id") and "source_locator" in comp and "parameters" in comp,
             f"{label}: constraint component {comp} is not a located manifest operation")
        covered.add(comp["operation_id"])
    omitted = {x.get("operation_id") for x in cp["omissions"] if isinstance(x, dict)}
    need(set(opids) <= covered | omitted, f"{label}: operations {sorted(set(opids) - covered - omitted)} neither constrained nor listed as omitted")
    nids = [n.get("node_id") for n in wp["nodes"]]
    need(nids and len(set(nids)) == len(nids), f"{label}: witness node ids")
    need(sorted(wp["stable_topological_order"]) == sorted(nids), f"{label}: topological order is not a permutation of the nodes")
    pos = {n: i for i, n in enumerate(wp["stable_topological_order"])}
    wcov = set()
    for n in wp["nodes"]:
        nl = f"{label}: witness node {n.get('node_id')}"
        need(n.get("operation_id") in opids, f"{nl}: operation {n.get('operation_id')!r} not in the manifest")
        _typed(n.get("typed_inputs"), f"{nl} typed_inputs")
        _typed(n.get("typed_outputs"), f"{nl} typed_outputs")
        deps = n.get("dependencies")
        need(isinstance(deps, list) and set(deps) <= set(nids) and all(pos[d] < pos[n["node_id"]] for d in deps),
             f"{nl}: dependencies {deps} not earlier in the stable topological order")
        need("last_use" in n, f"{nl}: no last_use")
        wcov.add(n["operation_id"])
    unsup = {o["operation_id"] for o in ops if o["supported_status"]["kind"] == "Unsupported"}
    need(set(opids) <= wcov | unsup, f"{label}: operations {sorted(set(opids) - wcov - unsup)} have no witness node")
    bc = ep.get("backend_capabilities") or {}
    miss = [k for k in BC_KEYS if k not in bc]
    need(not miss, f"{label}: BackendCapabilities does not list {miss} separately")
    for k in ("lookup", "interaction", "custom_gates", "structured_witness_ir", "native_closures", "field_layouts"):
        need(isinstance(bc[k], dict) and isinstance(bc[k].get("supported"), bool), f"{label}: capability {k} has no supported flag")
    fmts = bc["formats"]
    need(isinstance(fmts, list) and fmts, f"{label}: no formats listed")
    for f in fmts:
        need(f.get("midnight_zkir_exporter") is False, f"{label}: format {f.get('format')!r} claims to be a Midnight/ZKIR exporter")
        need(isinstance(f.get("supplies_witness_computation"), bool), f"{label}: format {f.get('format')!r} lacks supplies_witness_computation")
        if f.get("format") == "constraint-json":
            need(f["supplies_witness_computation"] is False, f"{label}: basic constraint JSON claims to supply witness computation")
    cl = ep.get("claims") or {}
    need(cl.get("constraint_json_supplies_witness") is False and cl.get("midnight_zkir_exporter") is False,
         f"{label}: claims {cl}")
    return om, cp, wp, bc


def _supported(bc, cap):
    if cap == "constraint-json":
        return any(f.get("format") == "constraint-json" for f in bc["formats"])
    return bc[CAP_FIELD[cap]]["supported"]


def _check_exports(ep, caps, d, label, om, cp, wp, bc):
    ex = ep.get("exports")
    need(isinstance(ex, list) and sorted(e.get("request") for e in ex) == sorted(caps),
         f"{label}: exports {[e.get('request') for e in ex or []]} != requested {caps}")
    emitted = {}
    for e in ex:
        cap = e["request"]
        el = f"{label}: export {cap}"
        disp = e.get("disposition")
        need(disp in ("emitted", "unsupported"), f"{el}: disposition {disp!r}")
        need((disp == "emitted") == _supported(bc, cap), f"{el}: disposition {disp} contradicts BackendCapabilities")
        if disp == "unsupported":
            need(e.get("artifact") is None, f"{el}: Unsupported but an artifact is named")
            us = e.get("unsupported") or {}
            need(us.get("feature") and us.get("reason") and (us.get("operation_id") or us.get("source_locator")),
                 f"{el}: tooling Unsupported is not located ({us})")
            continue
        a = e.get("artifact") or {}
        p = a.get("path")
        need(isinstance(p, str) and p and not p.startswith("/") and ".." not in p.split("/"), f"{el}: artifact path {p!r}")
        f = Path(d) / p
        need(f.is_file(), f"{el}: artifact {p} was not written to the export directory")
        raw = f.read_bytes()
        need(hashlib.sha256(raw).hexdigest() == a.get("sha256") and str(len(raw)) == str(a.get("bytes")),
             f"{el}: reported sha256/bytes do not identify the emitted content")
        doc = json.loads(raw)
        if cap == "constraint-json":
            need(doc.get("plan_id") == cp["plan_id"], f"{el}: emitted constraints are not ConstraintPlan {cp['plan_id'][:12]}")
            need(not any(re.search(rf"(^|\.){k}($|\.|\[)", path) for k in WITNESS_KEYS for path, _ in _walk(doc)),
                 f"{el}: basic constraint JSON carries witness computation")
        if cap == "structured-witness":
            need(doc.get("plan_id") == wp["plan_id"], f"{el}: emitted witness program is not WitnessPlan {wp['plan_id'][:12]}")
            need(len(doc.get("nodes") or []) == len(wp["nodes"]), f"{el}: emitted witness program node count differs from the plan")
        emitted[p] = a["sha256"]
    on_disk = sorted(str(x.relative_to(d)) for x in Path(d).rglob("*") if x.is_file())
    need(on_disk == sorted(emitted), f"{label}: export directory holds {on_disk}, reports only {sorted(emitted)}")
    return {e["request"]: (e["disposition"], (e.get("artifact") or {}).get("sha256")) for e in ex}


def _walk(v, path=""):
    if isinstance(v, dict):
        for k, w in v.items():
            p = f"{path}.{k}" if path else k
            yield p, w
            yield from _walk(w, p)
    elif isinstance(v, list):
        for i, w in enumerate(v):
            yield from _walk(w, f"{path}[{i}]")


def _export_run(c, con, pos, caps, label):
    d = tempfile.mkdtemp(prefix="dw-export-")
    try:
        extra = _flags(con, "plan", position=str(pos)) + _flags(con, "dir", dir=d)
        for cap in caps:
            extra += _flags(con, "request", capability=cap)
        r = U.runx(c, extra=extra)
        need(r.doc is not None and isinstance(r.doc.get("export_plans"), dict),
             f"{label}: no export_plans report (rc={r.returncode}, out={r.raw[:200]!r}, err={r.stderr[:200]!r})")
        return r, r.doc["export_plans"], d
    except Exception:
        shutil.rmtree(d, ignore_errors=True)
        raise


def custom_export_plans(c):
    con = _contract("U031")
    sc = {**c}
    base = U.runx(sc)
    _halted_value(base, "host run", c.get("expect"))
    _, ref = U._slices(sc, 1)
    manifests = {}
    for step in c["steps"]:
        rec = _select_step(ref, step["select"], step["label"])
        label = step["label"]
        r, ep, d = _export_run(sc, con, rec["position"], list(CAPS), f"{label} all requests")
        try:
            U.need_executed(r, label)
            need(r.returncode == 0, f"{label}: tooling Unsupported changed the application exit ({r.returncode})")
            need(r.execution["summary"] == base.summary, f"{label}: export changed the application run")
            om, cp, wp, bc = _check_plans(ep, rec, label, step["expect_op"])
            together = _check_exports(ep, list(CAPS), d, label, om, cp, wp, bc)
        finally:
            shutil.rmtree(d, ignore_errors=True)
        need(together["constraint-json"][0] == "emitted" and together["structured-witness"][0] == "emitted",
             f"{label}: constraint export and structured witness export were not both emitted")
        need(any(together[k][0] == "unsupported" for k in ("native-witness-closure", "lookup", "interaction")),
             f"{label}: no requested native closure/lookup/interaction export is reported Unsupported")
        manifests[label] = om["manifest_id"]
        # each capability requested separately gives the same disposition and the same bytes
        for cap in CAPS:
            r1, ep1, d1 = _export_run(sc, con, rec["position"], [cap], f"{label} {cap} alone")
            try:
                need(ep1["operation_manifest"]["manifest_id"] == om["manifest_id"], f"{label} {cap}: manifest not deterministic")
                one = _check_exports(ep1, [cap], d1, f"{label} {cap} alone", om, cp, wp, bc)
            finally:
                shutil.rmtree(d1, ignore_errors=True)
            need(one[cap] == together[cap], f"{label}: {cap} requested alone gives {one[cap]}, together {together[cap]}")
        r0, ep0, d0 = _export_run(sc, con, rec["position"], [], f"{label} no request")
        try:
            need(ep0.get("exports") == [], f"{label}: exports emitted without a request")
            need(not any(Path(d0).iterdir()), f"{label}: files written without a request")
        finally:
            shutil.rmtree(d0, ignore_errors=True)
    need(len(set(manifests.values())) == len(manifests), "two different steps produced the same OperationManifest")
    _refused_any(sc, _flags(con, "plan", position="0"), "export position 0")


# =================================================================================== U024

PLANES = ("values", "rejection-exhaustion", "evaluation-effect-order", "boundaries", "duties", "resource-schedule")
COSTS = ("logical-work", "logical-allocation", "physical-bytes", "rows-degree", "proof-key-sizes", "host-time",
         "host-memory", "dust")
CLAUSE_STATUS = ("evidenced", "gap", "evidenced-with-gaps")
COST_STATUS = ("measured", "unmeasured", "unchanged-by-construction")
OPT_RECEIPT_FORMAT = "moriarty-sherlock-optimization-receipt/1"
OPT_ROLES = ("preserves", "detects-violation")
_VIOLATION_MARKERS = ("is_err", "Err(", "assert!(!", "should_panic", "violat", "reject", "mismatch", "assert_ne!")


def _ids(xs, what):
    need(isinstance(xs, list), f"record: {what} is not a list")
    ids = [x.get("id") for x in xs]
    need(all(isinstance(i, str) and i for i in ids) and len(set(ids)) == len(ids), f"record: {what} ids missing or repeated")
    return set(ids)


def verify_opt_receipt(ev, candidate, ops):
    bad = f"record dishonest: evidence {ev.get('id')} receipt"
    rp = ev.get("receipt")
    need(isinstance(rp, str) and rp.endswith(".json") and not rp.startswith("/") and ".." not in rp.split("/"), f"{bad}: path {rp!r}")
    need(ops.tracked_clean(rp), f"{bad}: {rp} is not committed (untracked or modified)")
    try:
        rc = json.loads(ops.read(rp) or "")
    except ValueError:
        raise Fail(f"{bad}: {rp} is not JSON")
    need(rc.get("format") == OPT_RECEIPT_FORMAT, f"{bad}: format {rc.get('format')!r}")
    need(rc.get("candidate") == candidate, f"{bad}: candidate {rc.get('candidate')!r}")
    sha = rc.get("commit") or ""
    need(re.fullmatch(r"[0-9a-f]{40}", sha) is not None and ops.is_ancestor(sha), f"{bad}: commit {sha!r} is not an ancestor of HEAD")
    cmd = rc.get("command") or ""
    argv = cmd.split()
    need(argv[:2] == ["cargo", "test"] and "--no-run" not in argv, f"{bad}: command {cmd!r} is not a cargo test run")
    recs = [r for r in ops.runs() if r.get("run") == rc.get("run")]
    need(len(recs) == 1, f"{bad}: run {rc.get('run')!r} is not in qa/sherlock/runs.jsonl")
    rec = recs[0]
    need(rec.get("exit") == 0 and rec.get("head") == sha and rec.get("cmd") == cmd and rec.get("trace") == rc.get("trace"),
         f"{bad}: run log record {rec.get('run')} disagrees (exit/head/cmd/trace)")
    trace = ops.read(rc.get("trace") or "")
    need(trace is not None, f"{bad}: trace {rc.get('trace')} is not on disk (unverifiable)")
    need(hashlib.sha256(trace.encode()).hexdigest() == rc.get("trace_sha256"), f"{bad}: trace sha256 differs")
    results = re.findall(r"^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed", trace, re.M)
    need(results and all(s == "ok" for s, _, _ in results) and sum(int(f) for _, _, f in results) == 0 and rc.get("failed") == 0,
         f"{bad}: the run had failures or no result")
    need(sum(int(p) for _, p, _ in results) == rc.get("passed"), f"{bad}: passed count differs from the trace")
    tests = rc.get("tests") or []
    roles = {t.get("role") for t in tests}
    need(set(OPT_ROLES) <= roles <= set(OPT_ROLES), f"{bad}: needs one 'preserves' and one 'detects-violation' test, has {sorted(r for r in roles if r)}")
    src = rc.get("file") or ""
    api = U._pub_fns(ops.read(src))
    need(api, f"{bad}: candidate file {src!r} exposes no pub fn")
    files = [src]
    for t in tests:
        name, tf = t.get("name") or "", t.get("file") or ""
        need(re.search(r"^test " + re.escape(name) + r" \.\.\. ok$", trace, re.M), f"{bad}: test {name} did not pass in the trace")
        text = ops.read(tf) if tf.startswith("crates/") else None
        need(text is not None, f"{bad}: test file {tf!r} missing")
        fn = name.split("::")[-1]
        need(U._is_test_fn(text, fn), f"{bad}: {tf} has no #[test] fn {fn}")
        body = U._expanded(text, fn)
        need(any(re.search(r"\b" + re.escape(a) + r"\b", body) for a in api), f"{bad}: test {fn} never calls the candidate")
        if t["role"] == "detects-violation":
            need(any(m in body for m in _VIOLATION_MARKERS), f"{bad}: test {fn} asserts no detected violation")
        files.append(tf)
    need(ops.unchanged_since(sha, sorted(set(files))), f"{bad}: candidate or tests changed since {sha[:12]}")


def check_record(rec, con, ops):
    need(rec.get("format") == "sherlock-optimization-preservation/1", f"record: format {rec.get('format')!r}")
    cand = rec.get("candidate") or {}
    need(cand.get("id") == con["candidate"], f"record: candidate {cand.get('id')!r} != contract {con['candidate']!r}")
    need((cand.get("replaces") or {}).get("kind") in ("operation", "representation") and (cand.get("replaces") or {}).get("reference"),
         f"record: candidate.replaces {cand.get('replaces')!r} names no replaced reference operation/representation")
    need(cand.get("class") in ("frame-storage", "sharing"), f"record: candidate.class {cand.get('class')!r} (frame storage or sharing)")
    need((rec.get("reference") or {}).get("profile"), "record: no reference profile")
    pp = rec.get("preservation_predicate") or {}
    need(isinstance(pp.get("statement"), str) and len(pp["statement"]) > 20, "record: no stated preservation predicate")
    need(pp.get("file") and ops.tracked_clean(pp["file"]), f"record: predicate file {pp.get('file')!r} is not committed")
    need(pp.get("entry") in U._pub_fns(ops.read(pp["file"])), f"record: predicate entry {pp.get('entry')!r} is not a pub fn of {pp.get('file')}")
    evid = _ids(rec.get("evidence"), "evidence")
    gaps = _ids(rec.get("unresolved_gaps"), "unresolved_gaps")
    need(not (evid & gaps), f"record: ids {sorted(evid & gaps)} are both evidence and gap (must be listed separately)")
    clauses = rec.get("clauses") or []
    obs = [cl.get("observation") for cl in clauses]
    need(sorted(obs) == sorted(PLANES), f"record: clauses cover {obs}, required exactly {list(PLANES)}")
    cited_e, cited_g = {}, {}
    for cl in clauses:
        o = cl["observation"]
        e, g = cl.get("evidence"), cl.get("gaps")
        need(isinstance(e, list) and isinstance(g, list) and set(e) <= evid and set(g) <= gaps, f"record: clause {o} cites unknown ids")
        need(isinstance(cl.get("statement"), str) and cl["statement"], f"record: clause {o} states no predicate")
        st = cl.get("status")
        need(st in CLAUSE_STATUS, f"record: clause {o} status {st!r}")
        need((st == "evidenced" and e and not g) or (st == "gap" and g and not e) or (st == "evidenced-with-gaps" and e and g),
             f"record: clause {o} status {st} inconsistent with evidence {e} / gaps {g}")
        for i in e:
            cited_e.setdefault(i, set()).add(o)
        for i in g:
            cited_g.setdefault(i, set()).add(o)
    costs = rec.get("costs") or []
    metrics = [x.get("metric") for x in costs]
    need(set(COSTS) <= set(metrics) and len(set(metrics)) == len(metrics), f"record: costs {metrics} lack {sorted(set(COSTS) - set(metrics))}")
    for x in costs:
        m, st = x["metric"], x.get("status")
        e, g = x.get("evidence") or [], x.get("gaps") or []
        need(st in COST_STATUS and set(e) <= evid and set(g) <= gaps, f"record: cost {m} status/ids invalid")
        need((st == "unmeasured" and g) or (st in ("measured", "unchanged-by-construction") and e),
             f"record: cost {m} is {st} without {'a gap' if st == 'unmeasured' else 'evidence'} (never asserted bare)")
        for i in e:
            cited_e.setdefault(i, set()).add(m)
        for i in g:
            cited_g.setdefault(i, set()).add(m)
    need(set(cited_e) == evid, f"record: evidence {sorted(evid - set(cited_e))} supports no clause or cost")
    for gp in rec["unresolved_gaps"]:
        need(gp["id"] in cited_g and set(gp.get("clauses") or []) == cited_g[gp["id"]],
             f"record: gap {gp['id']} clauses {gp.get('clauses')} != citing planes/costs {sorted(cited_g.get(gp['id'], []))}")
        need(isinstance(gp.get("statement"), str) and gp["statement"], f"record: gap {gp['id']} has no statement")
    replay = False
    for ev in rec["evidence"]:
        k = ev.get("kind")
        need(k in ("receipt", "holdout-replay"), f"record: evidence {ev['id']} kind {k!r}")
        if k == "receipt":
            verify_opt_receipt(ev, con["candidate"], ops)
        else:
            replay = True
    need(rec.get("acceptance") in ("none", "not-claimed"), f"record: acceptance {rec.get('acceptance')!r} (security acceptance is Opus-owned)")
    return replay


def _opt_report(r, label, candidate):
    o = r.doc.get("optimization")
    need(isinstance(o, dict) and o.get("format") == "moriarty-sherlock-optimization-run/1" and o.get("candidate") == candidate,
         f"{label}: no optimization report for {candidate}")
    need(isinstance(o.get("applied"), bool), f"{label}: optimization.applied missing")
    lg, ph = o.get("logical"), o.get("physical")
    need(isinstance(lg, dict) and lg and isinstance(ph, dict), f"{label}: optimization lacks logical/physical counters")
    for k, v in lg.items():
        need(str(v.get("reference")) == str(v.get("candidate")), f"{label}: logical counter {k} changed {v} (sharing/interning never reduces logical allocation)")
    return o


def _summ(r):
    return r.execution["summary"]


def custom_optimization_preservation(c):
    con = _contract("U024")
    rp = con.get("record")
    need(isinstance(rp, str), "contract U024 names no record")
    ops = U.RepoOps(engine.REPO)
    need(ops.tracked_clean(rp), f"not-implemented: optimization record {rp} is not committed")
    rec = json.loads(ops.read(rp))
    check_record(rec, con, ops)
    flag = _flags(con, "candidate", candidate=con["candidate"])
    reduced = False
    for sp in c["specimens"]:
        sc = {**c, "source": sp["source"]}
        lab = sp["label"]
        b = U.runx(sc)
        v = _halted_value(b, f"{lab} reference", sp["expect"])
        k = U.runx(sc, extra=flag)
        _halted_value(k, f"{lab} candidate", sp["expect"])
        o = _opt_report(k, f"{lab} candidate", con["candidate"])
        need(_summ(k) == _summ(b), f"{lab}: candidate summary {_summ(k)} != reference {_summ(b)} (work/events/outcome)")
        need(k.execution["image"].get("tasks") == b.execution["image"].get("tasks"), f"{lab}: effects/duties (tasks) differ")
        need(k.execution["image"].get("frames") == b.execution["image"].get("frames") == [], f"{lab}: frames remain")
        if sp.get("equal_subtrees"):
            need(o["applied"], f"{lab}: the candidate was not applied to equal subtrees")
            if any(int(x.get("candidate", 0)) < int(x.get("reference", 0)) for x in o["physical"].values()):
                reduced = True
        # order and boundaries: the K-A reference record stream
        _, rs_b = U._slices(sc, 1)
        rk = U.runx(sc, extra=flag + ["--reference-slices", "1"])
        need(rk.doc and "reference_slices" in rk.doc, f"{lab}: candidate run lost the reference slices")
        need(U._flat(rk.doc["reference_slices"]) == U._flat(rs_b), f"{lab}: evaluation order / boundaries differ")
        total = int(_summ(b)["consumed_work"])
        stack0 = int(b.execution["image"]["stack_limit"])
        for fuel, stack, kind in ((total, None, "Halted"), (total - 1, None, "Exhausted"), (None, 1, "Rejected"),
                                  (total // 2, None, "Exhausted"), (max(1, total // 4), None, "Exhausted")):
            bb = U.runx(sc, fuel=fuel, stack=stack)
            kk = U.runx(sc, extra=flag, fuel=fuel, stack=stack)
            l2 = f"{lab} fuel={fuel} stack={stack}"
            U.need_executed(bb, f"{l2} reference")
            U.need_executed(kk, f"{l2} candidate")
            need(engine.outcome_kind(bb)[0] == kind, f"{l2}: reference outcome {bb.outcome} (harness)")
            need(_summ(kk) == _summ(bb), f"{l2}: candidate {_summ(kk)} != reference {_summ(bb)} (rejection/exhaustion/work)")
            need(len(kk.execution["image"]["frames"]) == len(bb.execution["image"]["frames"]), f"{l2}: pending frame count differs")
            _opt_report(kk, l2, con["candidate"])
    need(reduced, "no specimen shows a physical reduction: the candidate is vacuous on equal subtrees")


# =================================================================================== U023 s2

QUAL_STAGES = ("compile", "prove", "verify", "deploy", "call", "readback")
QUAL_PINS = ("network", "ledger_version", "node_version", "toolchain", "relation_id")


def _indexer_tx(sn, h, label):
    t = sn.txs(h)
    need(len(t) == 1 and (t[0].get("transactionResult") or {}).get("status") == "SUCCESS",
         f"{label}: transaction {str(h)[:16]} is not indexed once with SUCCESS")
    need(sn.sha(bytes.fromhex(t[0]["raw"])) == h, f"{label}: indexed bytes do not hash to {str(h)[:16]}")
    return t[0]


def _action_at(sn, address, h):
    return sn.gql('{contractAction(address:"%s",offset:{transactionOffset:{hash:"%s"}}){__typename state '
                  'transaction{hash} ... on ContractCall{entryPoint}}}' % (address, h))["contractAction"]


def custom_native_qualification(c):
    """UIREQ-023 scenario 2: verify a committed end-to-end qualification record; never trust its words."""
    con = _contract("U023")
    rp = con.get("qualification_record")
    need(isinstance(rp, str), "contract U023 names no qualification_record")
    ops = U.RepoOps(engine.REPO)
    need(ops.tracked_clean(rp), f"not-implemented: qualification record {rp} is not committed")
    q = json.loads(ops.read(rp))
    need(q.get("format") == "moriarty-sherlock-native-qualification/1", f"record: format {q.get('format')!r}")
    cl = q.get("claims") or {}
    need(cl.get("execution_authority") == "none" and cl.get("preview_fenrir_evidence_transferred") is False,
         f"record: claims {cl} (no execution authority, no Preview/Fenrir transfer)")
    src = q.get("source") or {}
    need(src.get("path") and ops.tracked_clean(src["path"]), f"record: source {src.get('path')!r} is not committed")
    text = ops.read(src["path"])
    need(hashlib.sha256(text.encode()).hexdigest() == src.get("sha256"), "record: source sha256 differs from the committed source")
    pins = q.get("pins") or {}
    miss = [k for k in QUAL_PINS if not pins.get(k)]
    need(not miss, f"record: unpinned identities {miss}")
    need(pins["network"] == "stagenet" and str(pins["ledger_version"]) == "9", f"record: pins {pins['network']}/{pins['ledger_version']} (stagenet, ledger 9)")
    vk = pins.get("relation_vk_path")
    need(vk and ops.tracked_clean(vk), f"record: relation verifying key {vk!r} is not committed")
    need(hashlib.sha256((engine.REPO / vk).read_bytes()).hexdigest() == pins["relation_id"], "record: relation_id is not the sha256 of the committed verifying key")
    st = q.get("stages") or {}
    bad = [k for k in QUAL_STAGES if (st.get(k) or {}).get("status") not in ("run", "not-run")]
    need(not bad, f"record: stages {bad} have no run/not-run status")
    notrun = [k for k in QUAL_STAGES if st[k]["status"] != "run"]
    need(not notrun, f"SPEC GATE (honest record): stages not run: {notrun}")
    need(st["verify"].get("verifier") and st["verify"]["verifier"] != st["prove"].get("prover"),
         "record: verification is not independent of the prover")
    # expected observation: independent of the record, the reference run of the same committed source
    base = engine.run_source(c.get("engine", "draft"), text)
    want = _halted_value(base, "reference run of the qualified source")
    need(str(st["readback"].get("observed_result")) == str(want),
         f"readback: observed result {st['readback'].get('observed_result')!r} != reference {want!r}")
    import stagenet_check as sn
    try:
        dep = _indexer_tx(sn, st["deploy"]["tx"], "deploy")
        call = _indexer_tx(sn, st["call"]["tx"], "call")
        a = _action_at(sn, st["deploy"]["address"], st["call"]["tx"])
    except (OSError, RuntimeError) as e:
        raise engine.HostUnfit(f"stagenet indexer unreachable: {e}")
    need(dep and call, "")
    need(a and a["__typename"] == "ContractCall" and a["entryPoint"] == st["call"].get("entry_point"),
         "call: not a ContractCall on the deployed address with the recorded entry point")
    need(hashlib.sha256(bytes.fromhex(a["state"])).hexdigest() == st["readback"].get("state_sha256"),
         "readback: recorded state does not hash like the indexed post-call state (complete state readback)")


CHECKS = {
    "residual-masks": custom_residual_masks,
    "export-plans": custom_export_plans,
    "optimization-preservation": custom_optimization_preservation,
    "native-qualification": custom_native_qualification,
}
