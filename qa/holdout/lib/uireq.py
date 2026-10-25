#!/usr/bin/env python3
"""UIREQ holdout runner. Usage: uireq.py NNN

Reads qa/holdout/cases/uireq/NNN/case.json and executes every scenario whose
`status` is "runnable" through the real `mori simulate` Sherlock host route.
Expected outcomes live only in case.json (derived from the openspec spec text).

Exit: 0 all scenarios pass; 1 any assertion fails or a scenario is not implemented
(docs/evaluator-loop.md: "not-implemented" is exit 1); 2 a broken case/harness;
3 host unfit.
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import engine  # noqa: E402


class Fail(Exception):
    pass


def need(cond, msg):
    if not cond:
        raise Fail(msg)


def run(check):
    return engine.run_source(check.get("engine", "draft"), check["source"],
                             fuel=check.get("fuel"), stack=check.get("stack"),
                             extra=check.get("extra"), bounds=check.get("bounds"))


def runx(c, extra=(), fuel=None, stack=None, bounds=None, source=None):
    return engine.run_source(c.get("engine", "draft"), source if source is not None else c["source"],
                             fuel=fuel, stack=stack, extra=list(extra), bounds=bounds)


def need_executed(r, label):
    need(r.executed, f"{label}: no completed execution (stage={r.failure_stage}, rc={r.returncode}, "
                     f"stderr={r.stderr[:200]!r}, out={r.raw[:200]!r})")


# ---- check kinds ----------------------------------------------------------------

def check_value(c):
    r = run(c)
    need_executed(r, c.get("label", "value"))
    k, _ = engine.outcome_kind(r)
    need(k == "Halted", f"{c.get('label')}: expected Halted, got {r.outcome}")
    got = engine.decode_value(r)
    need(_norm(got) == _norm(c["expect"]), f"{c.get('label')}: expected {c['expect']!r}, got {got!r}")


def _norm(v):
    if isinstance(v, tuple):
        return [_norm(x) for x in v]
    if isinstance(v, list):
        return [_norm(x) for x in v]
    return v


def check_outcome(c):
    r = run(c)
    need_executed(r, c.get("label", "outcome"))
    k, detail = engine.outcome_kind(r)
    need(k == c["expect_kind"], f"{c.get('label')}: expected {c['expect_kind']}, got {r.outcome}")
    for key, want in c.get("expect_detail", {}).items():
        need(detail.get(key) == want, f"{c.get('label')}: outcome detail {key}: expected {want!r}, got {detail!r}")
    if c.get("consumed_work_equals_fuel"):
        need(int(r.summary["consumed_work"]) == int(c["fuel"]),
             f"{c.get('label')}: consumed_work {r.summary['consumed_work']} != fuel {c['fuel']}")
    if c.get("no_result"):
        need(r.execution["image"].get("result") is None, f"{c.get('label')}: a result was published")
    if c.get("progress"):
        need(int(r.summary["consumed_work"]) > 0 and int(r.summary["events"]) > 0, f"{c.get('label')}: no progress")


def check_failure(c):
    r = run(c)
    need(r.doc is not None, f"{c.get('label')}: unparsable output {r.raw[:200]!r}")
    need("execution" not in r.doc, f"{c.get('label')}: expected admission-level failure but program executed")
    need(r.doc.get("stage") == c["expect_stage"], f"{c.get('label')}: stage {r.doc.get('stage')!r} != {c['expect_stage']!r}")
    need(r.doc.get("evaluation_performed") is False, f"{c.get('label')}: evaluation_performed not false")
    if "expect_error_kind" in c:
        kind = r.doc["actual_error"]["error"]["kind"]
        need(c["expect_error_kind"] in kind, f"{c.get('label')}: error kind {kind!r}, wanted {c['expect_error_kind']}")
        if "expect_name" in c:
            payload = kind[c["expect_error_kind"]]
            name = payload if isinstance(payload, str) else payload.get("name")
            need(name == c["expect_name"], f"{c.get('label')}: error name {payload!r}, wanted {c['expect_name']!r}")


def check_typed_completed(c):
    r = run(c)
    need_executed(r, c.get("label", "typed"))
    bt = r.doc["preparation"]["body_typechecking"]["status"]
    need(bt == "completed", f"{c.get('label')}: body typing status {bt!r}")
    if "expect_kind" in c:
        k, _ = engine.outcome_kind(r)
        need(k == c["expect_kind"], f"{c.get('label')}: outcome {r.outcome}")


# ---- custom checks ----------------------------------------------------------------

def custom_same_machine(c):
    runs = [run(x) for x in c["sources"]]
    for i, r in enumerate(runs):
        need_executed(r, f"program {i}")
        need(engine.outcome_kind(r)[0] == "Halted", f"program {i} did not halt: {r.outcome}")
    a, b = runs
    need(a.doc["preparation"]["supported_profile"] == b.doc["preparation"]["supported_profile"],
         "supported profile differs between the two programs")
    ia = a.execution["image"]["run_id"]["identity"]
    ib = b.execution["image"]["run_id"]["identity"]
    need(ia == ib, f"run identity differs: {ia} vs {ib}")
    need(a.doc["preparation"]["execution_profile"] == b.doc["preparation"]["execution_profile"], "execution profile differs")
    need(a.doc["preparation"]["config"] == b.doc["preparation"]["config"], "configuration identity differs")


def custom_supported_profile_report(c):
    r = run(c)
    need_executed(r, "profile report")
    sp = r.doc["preparation"]["supported_profile"]
    exprs = set(sp["grammar"]["expressions"])
    for want in c["expressions"]:
        need(want in exprs, f"supported profile does not name expression {want}")
    nd = sp["numeric_domains"]
    need("UInt" in nd and "340282366920938463463374607431768211455" in nd["UInt"], "UInt128 domain not stated")
    need("Bool" in nd, "Bool domain not stated")
    need(bool(sp.get("schedule")), "schedule not reported")
    bounds = sp["extents"]["bounds"]
    for key in c["bound_keys"]:
        need(key in bounds, f"bound {key} not reported")
    need(any(k.startswith("max_") and "helper" in k for k in bounds), "helper-work bound not reported")
    need(sp.get("source_language") and sp.get("execution_profile"), "versioned profile identity missing")


def custom_distinct_failure_class(c):
    bad = run(c["admission_source"])
    need(bad.doc is not None and "execution" not in bad.doc and bad.doc.get("evaluation_performed") is False,
         "admission failure was not reported outside the runtime outcome schema")
    need("stage" in bad.doc, "admission failure lacks its stage")
    rt = engine.run_source(c.get("engine", "draft"), c["runtime_source"]["source"], stack=c["runtime_source"]["stack"])
    need_executed(rt, "runtime rejection")
    need(engine.outcome_kind(rt)[0] == "Rejected", f"runtime failure not an application outcome: {rt.outcome}")


def custom_left_to_right(c):
    for case in c["cases"]:
        r = engine.run_source("draft", case["source"], stack=c["stack"])
        need_executed(r, case["label"])
        k, detail = engine.outcome_kind(r)
        need(k == "Rejected" and detail.get("code") == case["expect_code"],
             f"{case['label']}: expected Rejected code {case['expect_code']}, got {r.outcome}")


def custom_depth_sweep(c):
    src = c["source"]
    full = engine.run_source("draft", src)
    need_executed(full, "full run")
    need(engine.outcome_kind(full)[0] == "Halted", f"full run not halted: {full.outcome}")
    need(engine.decode_value(full) == c["expect"], f"full result {engine.decode_value(full)!r} != {c['expect']!r}")
    img = full.execution["image"]
    need(img["frames"] == [], "Halted result with remaining frames")
    total = int(full.summary["consumed_work"])
    max_depth = 0
    return_with_frames = False
    for fuel in range(1, total):
        r = engine.run_source("draft", src, fuel=fuel)
        need_executed(r, f"prefix {fuel}")
        im = r.execution["image"]
        depth = len(im["frames"])
        max_depth = max(max_depth, depth)
        ctl = im["control"]
        if isinstance(ctl, dict) and "Return" in ctl and depth > 0 and engine.outcome_kind(r)[0] != "Halted":
            return_with_frames = True
    if "min_depth" in c:
        need(max_depth >= c["min_depth"], f"max pending-frame depth {max_depth} < {c['min_depth']}")
    if c.get("require_intermediate_return"):
        need(return_with_frames, "no intermediate Return state with remaining frames was observed")


def custom_work_identity(c):
    r = run(c)
    need_executed(r, "work run")
    s = r.summary
    cons, ref, helper = int(s["consumed_work"]), int(s["reference_cursor"]), int(s["helper_work"])
    need(helper > 0, "no helper transitions were charged")
    need(cons == ref + helper, f"consumed_work {cons} != legacy transitions {ref} + helper transitions {helper}")
    # terminal publication costs zero: exactly `cons` fuel suffices, one less does not
    exact = engine.run_source(c.get("engine", "draft"), c["source"], fuel=cons)
    need_executed(exact, "exact-fuel run")
    need(engine.outcome_kind(exact)[0] == "Halted", f"fuel == consumed_work did not halt: {exact.outcome}")
    short = engine.run_source(c.get("engine", "draft"), c["source"], fuel=cons - 1)
    need_executed(short, "short-fuel run")
    need(engine.outcome_kind(short) == ("Exhausted", {"reason": "Fuel"}), f"fuel-1 did not exhaust: {short.outcome}")


def _paths(pv):
    flat = pv["occurrence_paths"]
    out = []
    for o in pv["occurrences"]:
        st, ln = int(o["path"]["start"]), int(o["path"]["len"])
        out.append((o["function"], o["owner_is_function"], tuple(flat[st:st + ln])))
    return out


def custom_occurrences(c):
    r1 = run({**c, "source": c["source"]})
    need_executed(r1, "occurrence run")
    pv = r1.program_view
    occ = _paths(pv)
    need(len(occ) == len(set(occ)), "source occurrences are not pairwise distinct")
    calls = [e for e in pv["expressions"] if "Call" in e]
    names = pv["names"]
    to_self = [e for e in calls if names[int(e["Call"]["function_name"])] == c["function"]]
    need(len(to_self) == c["expect_calls_to_function"],
         f"expected {c['expect_calls_to_function']} call sites naming {c['function']}, found {len(to_self)}")
    # the two textually identical sites are separate expressions with separate occurrences
    ident = [i for i, e in enumerate(pv["expressions"]) if "Call" in e and
             names[int(e["Call"]["function_name"])] == c["function"] and e["Call"]["args"]["len"] == "1"]
    need(len(ident) >= 2, "repeated call sites were merged")
    r2 = run({**c, "source": c["source_other_entry"]})
    need_executed(r2, "other-entry run")
    need(len(r2.program_view["expressions"]) == len(pv["expressions"]) and
         len(r2.program_view["occurrences"]) == len(pv["occurrences"]),
         "program image size depends on the entry argument (recursion unrolled or input evaluated)")


def custom_correspondence(c):
    r = run(c)
    need_executed(r, "correspondence run")
    need(engine.decode_value(r) == c["expect"], f"result {engine.decode_value(r)!r} != {c['expect']!r}")
    corr = r.doc["preparation"]["correspondence"]
    need(corr["status"] == "named-unproved", f"correspondence claims status {corr['status']!r}")
    constructs = {o["construct"] for o in corr["obligations"]}
    for want in c["constructs"]:
        need(want in constructs, f"no semantic mapping obligation for {want}")
    for o in corr["obligations"]:
        for field in ("returned_values", "defined_failures", "order", "resource_obligation", "effects_and_duties"):
            need(o.get(field), f"obligation {o.get('id')} lacks {field}")
    need(corr["proof"]["semantic_correspondence"] == "unproved", "semantic correspondence over-claimed")
    need(corr["proof"]["constraint_plan"] == "missing", "host evaluation was presented as circuit evidence")


def custom_evidence_class(c):
    r = run(c)
    need_executed(r, "evidence run")
    ev = r.doc["preparation"]["evidence"]
    need(ev["native"] == "unrun" and ev["target"] == "unrun", f"native/target over-claimed: {ev}")
    need(ev["acceptance"] == "unestablished", f"acceptance over-claimed: {ev}")
    need("HostOnly" in r.doc["qualification"], f"qualification does not name the host-only class: {r.doc['qualification']!r}")
    prep = r.doc["preparation"]
    need(prep["source"]["source_digest"] and prep["execution_profile"] and prep["admission"]["bounds"] and "target" in prep,
         "report omits source identity, profile, bounds or target")


# ---- technical chunks / reference slices / bounded representation (U013 U016 U029 U032) ----
#
# These kinds read the HostOnly chunk/slice reports of `mori simulate` (draft route) but never
# trust a report's own equality flags alone: every claim is re-derived from an independent
# observation (an unchunked run, a fuel-limited unchunked run, a single-chunk event stream,
# differently grouped slices, or a capacity sweep).

_COUNTERS = ("consumed_work", "helper_work", "helper_visits", "reference_cursor")
_CHARGE_OF = {"consumed_work": "work", "helper_work": "helper_work",
              "helper_visits": "helper_visits", "reference_cursor": "reference_cursor"}
_VIEW_FIELDS = ("control", "frames", "env_root", "helper")


def _base(c, label="unchunked run"):
    b = runx(c)
    need_executed(b, label)
    need(engine.outcome_kind(b)[0] == "Halted", f"{label}: not halted: {b.outcome}")
    if "expect" in c:
        need(engine.decode_value(b) == c["expect"], f"{label}: result {engine.decode_value(b)!r} != {c['expect']!r}")
    return b


def _tc(r, label):
    need_executed(r, label)
    tc = r.execution.get("technical_chunks")
    need(isinstance(tc, dict), f"{label}: no execution.technical_chunks")
    need(tc.get("format") == "moriarty-sherlock-technical-chunks/1", f"{label}: chunk format {tc.get('format')!r}")
    return tc


def _same_run(r, base, label, bounds_changed=False):
    for key in ("summary", "image", "resource_report"):
        a, b = r.execution.get(key), base.execution.get(key)
        if key == "resource_report" and bounds_changed:
            # only the echoed supplied bounds may differ when a capacity bound was changed
            a = {k: v for k, v in a.items() if k != "supplied"}
            b = {k: v for k, v in b.items() if k != "supplied"}
        need(a == b, f"{label}: {key} differs from the unchunked run")
    need(engine.decode_value(r) == engine.decode_value(base), f"{label}: result differs from the unchunked run")


def _events(tc):
    out = []
    for ch in tc["chunks"]:
        out.extend(ch.get("events") or [])
    return out


def _refused(c, extra, label):
    r = runx(c, extra=extra)
    need(r.doc is not None, f"{label}: unparsable output {r.raw[:200]!r}")
    need("execution" not in r.doc and r.doc.get("evaluation_performed") is False,
         f"{label}: invalid plan {extra} was not refused before evaluation")
    need(r.doc.get("stage") == "config", f"{label}: invalid plan refused at stage {r.doc.get('stage')!r}, not config")


def _cut_state(c, ch, label):
    """Independent cut state: an unchunked run limited to the work consumed at the cut must stop in
    exactly the boundary's control/frames/environment/helper with the same counters."""
    w = int(ch["end_counters"]["consumed_work"])
    f = runx(c, fuel=w)
    need_executed(f, f"{label}: fuel-limited run at {w}")
    im = f.execution["image"]
    view = ch["boundary"]["view"]
    for k in _VIEW_FIELDS:
        need(im.get(k) == view.get(k), f"{label}: boundary {k} at event {ch['end_event']} != unchunked state after {w} work")
    for k in _COUNTERS:
        need(im[k] == ch["end_counters"][k], f"{label}: boundary {k} {ch['end_counters'][k]} != unchunked {im[k]}")
    need(len(view["frames"]) == int(ch["boundary"]["logical_depth"]), f"{label}: logical_depth != frame count")


def _check_chunk_report(tc, base, plan, label):
    summ = base.summary
    initial = int(base.execution["image"]["initial_fuel"])
    chunks = tc["chunks"]
    n = len(chunks)
    need(n >= 1 and int(tc["chunk_count"]) == n, f"{label}: chunk_count {tc['chunk_count']} vs {n} chunks")
    need(int(tc["technical_cuts"]) == n - 1, f"{label}: technical_cuts {tc['technical_cuts']} != {n - 1}")
    need(tc["financial_commits"] == "0", f"{label}: financial commits {tc['financial_commits']!r}")
    if plan[0] == "--chunk-at":
        want = [e for e in plan[1].split(",") if int(e) < int(summ["events"])]
        need([ch["end_event"] for ch in chunks[:-1]] == want, f"{label}: cuts {[ch['end_event'] for ch in chunks[:-1]]} != requested {want}")
    if plan[0] == "--chunk":
        size = int(plan[1])
        for ch in chunks[:-1]:
            need(int(ch["end_event"]) - int(ch["start_event"]) == size, f"{label}: chunk {ch['index']} is not {size} events")
    prev = {k: 0 for k in _COUNTERS}
    prev_end = 0
    sums = {k: 0 for k in _COUNTERS}
    ev_sum = 0
    for i, ch in enumerate(chunks):
        need(int(ch["index"]) == i, f"{label}: chunk index {ch['index']} at {i}")
        need(int(ch["start_event"]) == prev_end, f"{label}: chunk {i} starts at {ch['start_event']}, previous ended at {prev_end}")
        need(int(ch["end_event"]) > int(ch["start_event"]), f"{label}: empty chunk {i}")
        last = i == n - 1
        need(ch["end"] == ("terminal" if last else "technical-cut"), f"{label}: chunk {i} end {ch['end']!r}")
        end = ch["end_counters"]
        for k in _COUNTERS:
            d = int(end[k]) - prev[k]
            need(d >= 0 and int(ch["charges"][_CHARGE_OF[k]]) == d,
                 f"{label}: chunk {i} {k} charge {ch['charges'][_CHARGE_OF[k]]} != counter delta {d}")
            sums[k] += int(ch["charges"][_CHARGE_OF[k]])
        need(int(end["consumed_work"]) + int(end["remaining_work"]) == initial,
             f"{label}: chunk {i} consumed+remaining != initial fuel {initial}")
        need(int(ch["charges"]["events"]) == int(ch["end_event"]) - int(ch["start_event"]),
             f"{label}: chunk {i} event charge mismatch")
        ev_sum += int(ch["charges"]["events"])
        if not last:
            cp = ch["checkpoint"]
            need(isinstance(cp, dict), f"{label}: technical cut {i} has no checkpoint")
            need(cp["format"] == "C6RAWHS1" and int(cp["bytes"]) > 0 and len(cp["sha256"]) == 64,
                 f"{label}: chunk {i} checkpoint malformed: {cp}")
            need(cp["archive_round_trip"] == "equal" and cp["resumed_from_archive"] is True and
                 cp["resumed_capture"] == "equal", f"{label}: chunk {i} not resumed from its archived state: {cp}")
            need(cp["financial_commit"] == "none", f"{label}: chunk {i} checkpoint made a financial commit")
            need(ch["boundary"]["outcome"] == "Running", f"{label}: cut {i} boundary outcome {ch['boundary']['outcome']!r}")
        elif ch.get("checkpoint") is not None:
            need(ch["checkpoint"].get("financial_commit") == "none", f"{label}: terminal chunk made a financial commit")
        prev = {k: int(end[k]) for k in _COUNTERS}
        prev_end = int(ch["end_event"])
    need(prev_end == int(summ["events"]) and ev_sum == int(summ["events"]),
         f"{label}: chunks cover {prev_end} events, run has {summ['events']}")
    for k in _COUNTERS:
        need(sums[k] == int(summ[k]), f"{label}: summed chunk {k} {sums[k]} != one-trace {summ[k]}")
        need(chunks[-1]["end_counters"][k] == summ[k], f"{label}: final {k} != one-trace summary")
    cum = tc["cumulative"]
    for key, sk in (("work_delta_sum", "consumed_work"), ("helper_work_delta_sum", "helper_work"),
                    ("helper_visits_delta_sum", "helper_visits"), ("reference_cursor_delta_sum", "reference_cursor"),
                    ("event_count", "events")):
        need(cum[key]["status"] == "exact" and cum[key]["value"] == summ[sk],
             f"{label}: cumulative {key} {cum[key]} != one-trace {sk} {summ[sk]}")
    need(tc["cumulative_equals_summary"] is True, f"{label}: cumulative_equals_summary not true")


_REQUIRED_SEPARATE = {
    # spec-named separate fields -> (group, metrics) that must be reported unmeasured, never valued
    "physical": ["physical-dispatches", "frame-bytes", "environment-bytes"],
    "host": ["wall-time", "peak-memory"],
    "circuit": ["active-rows", "degree"],
    "proof": ["key-bytes", "proof-bytes"],
    "financial_target": ["financial-fees"],
    "overhead": ["preparation", "report-serialization"],
}


def custom_chunk_transparency(c):
    base = _base(c)
    summ = base.summary
    single = _tc(runx(c, extra=["--chunk", "1000000", "--chunk-events"]), "single-chunk run")
    need(len(single["chunks"]) == 1, "an oversized --chunk still cut the run")
    stream = [e["event"] for e in _events(single)]
    need(len(stream) == int(summ["events"]), f"single-chunk stream has {len(stream)} events, run has {summ['events']}")
    need(len(c["plans"]) >= 2 and any(p[0] == "--chunk-at" and "," not in p[1] for p in c["plans"]),
         "case must include a two-chunk --chunk-at plan")
    seen_two = False
    for plan in c["plans"]:
        label = " ".join(plan)
        r = runx(c, extra=list(plan) + ["--chunk-events"])
        tc = _tc(r, label)
        _same_run(r, base, label)
        _check_chunk_report(tc, base, plan, label)
        seen_two |= len(tc["chunks"]) == 2
        evs = _events(tc)
        need([int(e["position"]) for e in evs] == list(range(1, len(evs) + 1)), f"{label}: event positions not 1..N")
        need([e["event"] for e in evs] == stream, f"{label}: chunked event stream differs from the one-trace stream")
        cuts = tc["chunks"][:-1]
        sample = cuts if len(cuts) <= 3 else [cuts[0], cuts[len(cuts) // 2], cuts[-1]]
        for ch in sample:
            _cut_state(c, ch, label)
        # versioned ResourceManifest and separate fields
        rm = tc["resource_manifest"]
        prof = {p["reference_schedule_id"]: p for p in rm["retained_profiles"]}
        for sid in ("sherlock-k-a/1", "sherlock-quint-a/1", "sherlock-helper-microstep/1"):
            need(sid in prof, f"{label}: ResourceManifest does not retain {sid}")
        ka, qa = prof["sherlock-k-a/1"]["terminal_return_charge"], prof["sherlock-quint-a/1"]["terminal_return_charge"]
        need(ka != qa, f"{label}: k-a and quint-a terminal charges are equal ({ka})")
        need(ka == "0", f"{label}: sherlock-k-a/1 terminal publication is not zero-fuel ({ka})")
        need(rm["same_fuel_equivalence"] == "not-claimed", f"{label}: same-fuel equivalence claimed: {rm['same_fuel_equivalence']!r}")
        need(rm["measured_cost_winner"] == "none", f"{label}: measured cost winner claimed: {rm['measured_cost_winner']!r}")
        need(rm["reference_schedule_id"] == "sherlock-helper-microstep/1", f"{label}: selected schedule {rm['reference_schedule_id']!r}")
        sf = tc["separate_fields"]
        obs = set(sf["observed"])
        for f in ("consumed_work", "reference_cursor", "helper_work", "helper_visits", "logical_depth"):
            need(f in obs, f"{label}: {f} is not a separate observed field")
        un = sf["unmeasured"]
        un_metrics = set()
        for g, body in un.items():
            need(body.get("status") == "unmeasured" and body.get("reason") and "value" not in body,
                 f"{label}: unmeasured group {g} carries a value or lacks its reason")
            un_metrics |= set(body["metrics"])
        for g, ms in _REQUIRED_SEPARATE.items():
            need(g in un, f"{label}: separate field group {g} missing")
            for m in ms:
                need(m in un[g]["metrics"], f"{label}: separate field {g}/{m} missing")
        need(not (obs & un_metrics), f"{label}: a field is both observed and unmeasured: {obs & un_metrics}")
    need(seen_two, "no plan produced exactly two technical chunks")
    # negatives: chunking must not alter fuel semantics; invalid plans are refused before evaluation
    w = int(summ["consumed_work"])
    exact = runx(c, extra=["--chunk", "1"], fuel=w)
    need_executed(exact, "chunked exact-fuel run")
    need(engine.outcome_kind(exact)[0] == "Halted", f"chunked run with fuel == consumed_work did not halt: {exact.outcome}")
    short = runx(c, extra=["--chunk", "1"], fuel=w - 1)
    need_executed(short, "chunked short-fuel run")
    need(engine.outcome_kind(short) == ("Exhausted", {"reason": "Fuel"}), f"chunked fuel-1 run did not exhaust: {short.outcome}")
    for bad in c.get("bad_plans", []):
        _refused(c, bad, "invalid chunk plan")


def _is_target(view, value, op):
    ctl = view["control"]
    fr = view["frames"]
    return (isinstance(ctl, dict) and "Return" in ctl and
            view.get("returned_data_row") == {"U128": {"n": str(value)}} and
            len(fr) == 1 and "RightF" in fr[0] and fr[0]["RightF"]["op"] == op)


def custom_chunk_boundary_return(c):
    base = _base(c)
    full = runx(c, extra=["--chunk", "1", "--chunk-events"])
    tcf = _tc(full, "single-step plan")
    _same_run(full, base, "single-step plan")
    cuts = tcf["chunks"][:-1]
    callee_active = [int(ch["end_event"]) for ch in cuts if any("ReturnF" in f for f in ch["boundary"]["view"]["frames"])]
    targets = [ch for ch in cuts if _is_target(ch["boundary"]["view"], c["callee_value"], c["caller_op"])]
    need(callee_active, "no boundary observed with the callee frame active")
    need(targets, f"no boundary with callee Return {c['callee_value']} and only the pending caller {c['caller_op']}")
    # the target lies strictly between the callee frame pop and the caller's addition
    t = targets[0]
    need(int(t["end_event"]) > max(e for e in callee_active), "target boundary precedes the callee return")
    for ch in cuts:
        if any("ReturnF" in f for f in ch["boundary"]["view"]["frames"]):
            need(not _is_target(ch["boundary"]["view"], c["callee_value"], c["caller_op"]),
                 "a boundary with an active callee frame was classified as between return and addition")
    stream = [e["event"] for e in _tc(runx(c, extra=["--chunk", "1000000", "--chunk-events"]), "single chunk")["chunks"][0]["events"]]
    for ch in (targets[0], targets[-1]):
        e = ch["end_event"]
        label = f"cut at {e}"
        r = runx(c, extra=["--chunk-at", e, "--chunk-events"])
        tc = _tc(r, label)
        need(len(tc["chunks"]) == 2 and tc["chunks"][0]["end_event"] == e, f"{label}: not a two-chunk run cut at {e}")
        _same_run(r, base, label)
        _check_chunk_report(tc, base, ["--chunk-at", e], label)
        b = tc["chunks"][0]["boundary"]["view"]
        need(b == ch["boundary"]["view"], f"{label}: boundary differs from the single-step plan's state at {e}")
        need(_is_target(b, c["callee_value"], c["caller_op"]), f"{label}: boundary is not between callee return and caller addition")
        need([x["event"] for x in _events(tc)] == stream, f"{label}: frame/effect event sequence differs from one trace")
        need(r.execution["image"]["tasks"] == base.execution["image"]["tasks"], f"{label}: effects/duties (tasks) differ")
        _cut_state(c, tc["chunks"][0], label)


def _flat(rs):
    out = []
    for s in rs["slices"]:
        for rec in s["records"]:
            fc = rec["frame_check"]
            out.append((rec["position"], rec["rule_start_line"], rec["source_control"], rec["charge"],
                        rec["cumulative_work"], fc["depth_before"], fc["depth_after"], fc["within_limit"],
                        rec["first_outcome"], json.dumps(rec["outcome_after"], sort_keys=True)))
    return out


def _slices(c, m, fuel=None, stack=None, source=None):
    r = runx(c, extra=["--reference-slices", str(m)], fuel=fuel, stack=stack, source=source)
    need(r.doc is not None and "reference_slices" in r.doc, f"M={m}: no reference_slices report ({r.raw[:200]!r})")
    rs = r.doc["reference_slices"]
    need(rs["format"] == "moriarty-sherlock-reference-slices/1" and rs["reference_schedule_id"] == "sherlock-k-a/1",
         f"M={m}: slice format/schedule {rs['format']!r} {rs['reference_schedule_id']!r}")
    return r, rs


def _check_slices(rs, m, fuel, stack, label):
    need(int(rs["group_budget"]) == m, f"{label}: group_budget {rs['group_budget']}")
    need(int(rs["fuel"]) == fuel and int(rs["stack_limit"]) == stack, f"{label}: fuel/stack {rs['fuel']}/{rs['stack_limit']}")
    need(rs["financial_commits"] == "0", f"{label}: financial commits {rs['financial_commits']!r}")
    sl = rs["slices"]
    need(int(rs["technical_cuts"]) == len(sl) - 1, f"{label}: technical_cuts {rs['technical_cuts']} vs {len(sl)} slices")
    pos, cum, depth = 0, 0, 0
    for i, s in enumerate(sl):
        recs = s["records"]
        need(1 <= len(recs) <= m, f"{label}: slice {i} has {len(recs)} records (budget {m})")
        need(int(s["index"]) == i and int(s["start_event"]) == pos, f"{label}: slice {i} index/start_event")
        need([int(r["internal_position"]) for r in recs] == list(range(1, len(recs) + 1)),
             f"{label}: slice {i} internal positions not 1..{len(recs)}")
        pre = 0
        for r in recs:
            pos += 1
            need(int(r["position"]) == pos, f"{label}: record position {r['position']} != {pos}")
            need(int(r["charge"]) in (0, 1), f"{label}: record {pos} charge {r['charge']}")
            pre += int(r["charge"])
            cum += int(r["charge"])
            need(int(r["prefix_work"]) == pre, f"{label}: record {pos} prefix_work {r['prefix_work']} != {pre}")
            need(int(r["cumulative_work"]) == cum, f"{label}: record {pos} cumulative_work {r['cumulative_work']} != {cum}")
            fc = r["frame_check"]
            need(int(fc["depth_before"]) == depth, f"{label}: record {pos} depth_before {fc['depth_before']} != {depth}")
            need(int(fc["stack_limit"]) == stack and fc["within_limit"] is (int(fc["depth_after"]) <= stack),
                 f"{label}: record {pos} frame check inconsistent: {fc}")
            depth = int(fc["depth_after"])
            need(int(r["rule_start_line"]) > 0 and r["source_control"], f"{label}: record {pos} lacks rule/source occurrence")
        need([int(x) for x in s["prefix_work"]] == [0] + [int(r["prefix_work"]) for r in recs],
             f"{label}: slice {i} prefix_work list {s['prefix_work']}")
        need(s["padding"] == {"charge": "0", "reference_records": "0"}, f"{label}: slice {i} padding {s['padding']}")
        need(s["stop"].get("financial_commit", "none") == "none", f"{label}: slice {i} stop made a financial commit")
        if s["stop"]["kind"] == "technical-cut":
            need(s["stop"].get("financial_commit") == "none", f"{label}: technical cut {i} does not state financial_commit none")
        last = i == len(sl) - 1
        need(s["stop"]["kind"] == ("outcome" if last else "technical-cut"), f"{label}: slice {i} stop {s['stop']['kind']!r}")
        if i > 0:
            need(s["start"] == sl[i - 1]["end"], f"{label}: slice {i} start != slice {i - 1} end (state not resumable)")
        need(int(s["end"]["remaining_fuel"]) == fuel - cum, f"{label}: slice {i} end remaining_fuel {s['end']['remaining_fuel']} != {fuel - cum}")
        need(int(s["end"]["logical_depth"]) == depth and len(s["end"]["frames"]) == depth, f"{label}: slice {i} end depth")
        if not last:
            need(s["end"]["outcome"] == {"kind": "Pending"}, f"{label}: slice {i} cut published an outcome")
    need(int(sl[0]["start"]["remaining_fuel"]) == fuel, f"{label}: first slice does not start at full fuel")
    recs = [r for s in sl for r in s["records"]]
    need(int(rs["events"]) == len(recs), f"{label}: events {rs['events']} != {len(recs)} records")
    need(int(rs["reference_work"]) == cum, f"{label}: reference_work {rs['reference_work']} != summed charges {cum}")
    need([r["first_outcome"] for r in recs] == [False] * (len(recs) - 1) + [True], f"{label}: first_outcome not exactly the final record")
    need(all(r["outcome_after"] == {"kind": "Pending"} for r in recs[:-1]), f"{label}: an outcome was published before the earliest stop")
    need(recs[-1]["outcome_after"] == rs["outcome"] == sl[-1]["stop"]["outcome"], f"{label}: final outcome inconsistent")
    return recs


def custom_reference_slices(c):
    ms = c.get("budgets", [1, 2, 4, 8])
    need(sorted(ms) == [1, 2, 4, 8], "case must sweep budgets 1, 2, 4 and 8")
    for src in c["sources"]:
        sc = {**c, "source": src["source"]}
        base = _base(sc, f"{src['label']} unchunked")
        fuel0 = int(base.execution["image"]["initial_fuel"])
        stack0 = int(base.execution["image"]["stack_limit"])
        flat = None
        work = None
        for m in ms:
            label = f"{src['label']} M={m}"
            r, rs = _slices(sc, m)
            need(r.execution["summary"] == base.summary, f"{label}: grouping changed the host run")
            recs = _check_slices(rs, m, fuel0, stack0, label)
            need(rs["outcome"] == {"kind": "Done", "value": src["expect_value"]}, f"{label}: outcome {rs['outcome']}")
            need(recs[-1]["charge"] == "0", f"{label}: terminal publication charged {recs[-1]['charge']}")
            # the K-A reference work is the helper-free reference count of the same computation
            need(int(rs["reference_work"]) == int(base.summary["reference_cursor"]),
                 f"{label}: reference_work {rs['reference_work']} != host reference_cursor {base.summary['reference_cursor']}")
            if m > 1:
                need(len(rs["slices"]) < len(recs), f"{label}: budget {m} grouped nothing")
            f = _flat(rs)
            flat = flat or f
            need(f == flat, f"{label}: flattened StepRecord stream differs from M={ms[0]}")
            work = int(rs["reference_work"])
        # zero-fuel terminal publication and frame-cap Rejected(error4) precedence under every budget
        prec = {}
        for m in ms:
            _, ok = _slices(sc, m, fuel=work)
            need(ok["outcome"]["kind"] == "Done", f"{src['label']} M={m}: fuel == reference_work gave {ok['outcome']}")
            okr = _check_slices(ok, m, work, stack0, f"{src['label']} M={m} exact fuel")
            need(okr[-1]["charge"] == "0" and okr[-1]["first_outcome"], f"{src['label']} M={m}: terminal publication not zero-fuel")
            _, ex = _slices(sc, m, fuel=work - 1)
            need(ex["outcome"]["kind"] == "Exhausted", f"{src['label']} M={m}: fuel-1 gave {ex['outcome']}")
            _check_slices(ex, m, work - 1, stack0, f"{src['label']} M={m} short fuel")
            _, rj = _slices(sc, m, stack=1)
            need(rj["outcome"] == {"kind": "Rejected", "code": "4"}, f"{src['label']} M={m}: stack 1 gave {rj['outcome']}")
            rjr = _check_slices(rj, m, fuel0, 1, f"{src['label']} M={m} stack 1")
            need(rjr[-1]["charge"] == "0", f"{src['label']} M={m}: frame-cap rejection was charged")
            key = (tuple(_flat(ok)), tuple(_flat(ex)), tuple(_flat(rj)))
            prec.setdefault("k", key)
            need(prec["k"] == key, f"{src['label']} M={m}: bounded outcome streams depend on the grouping budget")
    _refused({**c, "source": c["sources"][0]["source"]}, ["--reference-slices", "0"], "zero group budget")


def _in_domain(rep, label):
    need(rep["status"] == "in-domain", f"{label}: representation {rep['status']!r} ({rep.get('detail')})")
    need(rep["format"] == "sherlock-c6-phys/1", f"{label}: representation format {rep['format']!r}")
    need(rep["decoded_equals_owner_image"] is True and rep["decoded_equals_capture"] is True,
         f"{label}: decode does not reproduce the owner image")
    need(rep["represented_image_claim"] is True, f"{label}: in-domain without a represented-image claim")
    need(int(rep["lanes"]) <= int(rep["max_total_lanes"]), f"{label}: lanes {rep['lanes']} exceed {rep['max_total_lanes']}")
    need(len(rep["lanes_sha256"]) == 64, f"{label}: lanes digest malformed")


def _common_rep(rep, label):
    need(rep["encode_decode_correspondence"] == "unproved", f"{label}: encode/decode correspondence over-claimed")
    need(rep["circuit_correspondence"] == "unimplemented", f"{label}: circuit correspondence over-claimed")
    sh = rep.get("sharing", "")
    need("structural occurrences" in sh and "physical identity is not source equality" in sh,
         f"{label}: sharing statement missing: {sh!r}")


def _pending_mul(view):
    return (view["helper"] is not None and view["env_root"] is not None and
            any("Mul" in json.dumps(f) for f in view["frames"]))


def custom_chunk_represent(c):
    base = _base(c)
    cfg = json.loads(engine.CONFIG_FILES[c.get("engine", "draft")].read_text())
    max_lanes = int(cfg["bounds"]["serialization_bytes"]) // 16
    plan = list(c["plan"])
    r = runx(c, extra=plan + ["--chunk-represent"])
    tc = _tc(r, "represented plan")
    _same_run(r, base, "represented plan")
    _check_chunk_report(tc, base, plan, "represented plan")
    chunks = tc["chunks"]
    lanes = []
    for ch in chunks:
        rep = ch.get("representation")
        need(isinstance(rep, dict), f"chunk {ch['index']}: no representation")
        _in_domain(rep, f"chunk {ch['index']}")
        _common_rep(rep, f"chunk {ch['index']}")
        need(int(rep["max_total_lanes"]) == max_lanes, f"lane capacity {rep['max_total_lanes']} != serialization_bytes/16 {max_lanes}")
        lanes.append(int(rep["lanes"]))
    digests = [ch["representation"]["lanes_sha256"] for ch in chunks]
    need(len(set(digests)) == len(digests), "two distinct boundary states share one represented image")
    tg = [ch for ch in chunks[:-1] if _pending_mul(ch["boundary"]["view"])]
    need(tg, "no in-domain boundary retains a pending multiplication, caller environment and helper cursor")
    for ch in (tg[0], tg[-1]):
        label = f"pending-Mul boundary at {ch['end_event']}"
        _cut_state(c, ch, label)  # the represented state is a real reference state of the unchunked run
        one = _tc(runx(c, extra=["--chunk-at", ch["end_event"], "--chunk-represent"]), label)
        rep1 = one["chunks"][0]["representation"]
        _in_domain(rep1, label)
        need(rep1["lanes_sha256"] == ch["representation"]["lanes_sha256"] and rep1["lanes"] == ch["representation"]["lanes"],
             f"{label}: the representation depends on the chunk plan, not only on the state")
    # capacity sweep: in-domain exactly when the state fits; out-of-capacity has no represented-image claim
    k = sorted(lanes)[len(lanes) // 2]
    need(min(lanes) <= k < max(lanes), "capacity sweep would not split the boundaries")
    t = runx(c, extra=plan + ["--chunk-represent"], bounds={"serialization_bytes": 16 * k})
    ttc = _tc(t, f"capacity {k} lanes")
    _same_run(t, base, f"capacity {k} lanes", bounds_changed=True)
    need(len(ttc["chunks"]) == len(chunks), "capacity bound changed the chunking")
    for ch, n in zip(ttc["chunks"], lanes):
        rep = ch["representation"]
        lbl = f"capacity {k}, chunk {ch['index']} ({n} lanes)"
        _common_rep(rep, lbl)
        need(int(rep["max_total_lanes"]) == k, f"{lbl}: capacity {rep['max_total_lanes']} != {k}")
        if n <= k:
            _in_domain(rep, lbl)
        else:
            need(rep["status"] == "out-of-domain" and rep["represented_image_claim"] is False,
                 f"{lbl}: out-of-capacity state was represented: {rep['status']}")
            need(rep["refused_by"] in ("encode", "decode", "bind", "layout", "observation"), f"{lbl}: refused_by {rep['refused_by']!r}")
            need("lanes" not in rep and "lanes_sha256" not in rep, f"{lbl}: out-of-domain result carries lanes")
            need(f"needed: {n}" in rep.get("detail", ""), f"{lbl}: refusal detail does not name the needed {n} lanes: {rep.get('detail')!r}")
    # the explicit single out-of-domain case
    ood = c["out_of_domain"]
    o = runx(c, extra=list(ood["extra"]) + ["--chunk-represent"], bounds=ood["bounds"])
    otc = _tc(o, "out-of-domain plan")
    _same_run(o, base, "out-of-domain plan", bounds_changed=True)
    rep = otc["chunks"][-1]["representation"]
    need(rep["status"] == "out-of-domain" and rep["refused_by"] == ood["expect_refused_by"] and
         rep["represented_image_claim"] is False and "lanes" not in rep,
         f"out-of-domain plan: {rep}")
    _common_rep(rep, "out-of-domain plan")


# ---- WitnessPlan materialization (U035) ----------------------------------------------
#
# Expected decoded states are authored in case.json from the program text (control value,
# environment, pending operands, active call frames, final value); they are not read from the
# materializer. Each materialized state is also cross-checked against a fuel-limited unchunked
# owner run, and against the same cut under a different chunk plan and a repeated run.

import re as _re

_FRAME_KIND = _re.compile(r"Frame\((\w+)")


def _wfields(w):
    return w["decoded"]["fields"]


def _wvalue(field):
    """Plain decoded value carried by a Control/Outcome payload, or None."""
    for f in field.get("fields", []):
        if f["field"] == "value":
            return f["value"].get("value")
    return None


def _wframes(w):
    out = []
    for f in _wfields(w)["frames"]["items"]:
        m = _FRAME_KIND.match(f["raw"])
        need(m, f"undecodable frame {f['raw']!r}")
        left = next((x["value"].get("value") for x in f["fields"] if x["field"] == "left"), None)
        out.append((m.group(1), left))
    return out


def _wenv(w):
    return {e["name"]: e["value"] for e in _wfields(w)["current_environment"]["environment"]}


def _witness_ok(w, label):
    need(isinstance(w, dict), f"{label}: no witness materialization")
    need(w.get("format") == "moriarty-sherlock-witness-materialization/1", f"{label}: witness format {w.get('format')!r}")
    need(w["status"] == "materialized", f"{label}: witness {w['status']!r}")
    need(w["schedules_agree"] is True, f"{label}: cached/lifetime schedules do not retain the uncached baseline")
    plan = w["plan"]
    need(plan["stable_order_is_identity"] is True, f"{label}: dependency order is not the stable identity order")
    n = int(plan["nodes"])
    need(n > 0 and 0 < int(plan["inputs"]) <= n and 0 < int(plan["live_outs"]) <= n, f"{label}: implausible plan {plan}")
    scope = " ".join(w["cache_key_scope"])
    for word in ("program", "profile", "layout", "immutable argument values"):
        need(word in scope, f"{label}: reusable cache key omits {word}: {w['cache_key_scope']}")
    d = w["diagnostics"]
    need("not bytes, time, prover memory or semantic work" in d["qualification"], f"{label}: diagnostics over-qualified")
    un, cold, warm, lu = d["uncached"], d["cold_cache"], d["warm_cache"], d["last_use"]
    for nm, host in (("uncached", un), ("cold", cold["host"]), ("warm", warm["host"]), ("last-use", lu["host"])):
        need(host["measured_host_bytes"] is None and host["measured_host_time_ns"] is None,
             f"{label}: {nm} diagnostics claim measured memory/time")
    for nm, s in (("cold", cold), ("warm", warm), ("last-use", lu)):
        need(int(s["scheduling"]["scheduled_nodes"]) == n, f"{label}: {nm} schedule skips nodes")
    need(int(un["evaluated_nodes"]) == n and int(cold["host"]["evaluated_nodes"]) == n,
         f"{label}: uncached/cold baseline did not evaluate every node")
    cm, wh = int(cold["scheduling"]["cache_misses"]), int(warm["scheduling"]["cache_hits"])
    need(int(cold["scheduling"]["cache_hits"]) == 0 and cm > 0, f"{label}: cold cache was not cold")
    need(int(warm["scheduling"]["cache_misses"]) == 0 and wh == cm, f"{label}: warm cache did not reuse every cold entry")
    need(int(warm["host"]["evaluated_nodes"]) < n, f"{label}: warm cache re-evaluated every node")
    need(int(lu["scheduling"]["reclaimed_slots"]) > 0 and int(lu["scheduling"]["peak_live_slots"]) < int(un["retained_slots"]),
         f"{label}: last-use lifetime reclaimed nothing")
    dk = set(w["decoded"]) | set(_wfields(w))
    for diag in ("cache_hits", "cache_misses", "peak_live_slots", "copied_value_nodes", "retained_slots"):
        need(diag not in dk, f"{label}: diagnostic {diag} leaked into the decoded snapshot")


def _owner_kinds(frames):
    return [next(iter(f)) for f in frames]


def _witness_vs_owner(c, src, ch, label):
    """Decoded counters, outcome and frame order against an unchunked owner run stopped by fuel."""
    w = ch["witness"]
    dec = w["decoded"]
    for k in _COUNTERS + ("remaining_work",):
        need(dec[k] == ch["end_counters"][k], f"{label}: decoded {k} {dec[k]} != chunk counter {ch['end_counters'][k]}")
    wk = int(ch["end_counters"]["consumed_work"])
    f = runx(c, fuel=wk, source=src)
    need_executed(f, f"{label}: fuel-limited owner run")
    im = f.execution["image"]
    for k in _COUNTERS:
        need(dec[k] == im[k], f"{label}: decoded {k} {dec[k]} != owner {im[k]}")
    kinds = [k for k, _ in _wframes(w)]
    need(kinds == list(reversed(_owner_kinds(im["frames"]))),
         f"{label}: decoded frames {kinds} are not the owner frames innermost-first {_owner_kinds(im['frames'])}")
    need(_FRAME_KIND.pattern and ("Return" in json.dumps(im["control"])) == ("Return" in _wfields(w)["control"]["raw"]),
         f"{label}: decoded control kind differs from the owner control")


_K_FRAME = _re.compile(r"ui(\w+F)\(")
_K_RIGHT_LEFT = _re.compile(r"^uiRightF\(ui\w+\(\), uiU\((\d+)\)\)$")
_K_RET_U = _re.compile(r"^uiRet\(uiU\((\d+)\)\)$")
_K_BIND = _re.compile(r"(\w+) \|-> ")


def _k_crosscheck(c, src, w, label):
    """The independent K-A reference model (sherlock-k-a/1, via --reference-slices 1) at the same
    reference position must show the same frames innermost-first, pending operands, control value
    and environment names as the materialized snapshot."""
    rc = int(w["decoded"]["reference_cursor"])
    r = runx(c, extra=["--reference-slices", "1"], source=src)
    need(r.doc is not None and "reference_slices" in r.doc, f"{label}: no K-A reference run")
    sl = r.doc["reference_slices"]["slices"]
    need(0 < rc <= len(sl), f"{label}: reference cursor {rc} outside the K-A run ({len(sl)} records)")
    k = sl[rc - 1]["end"]
    kinds = []
    for f in k["frames"]:
        m = _K_FRAME.match(f)
        need(m, f"{label}: unparsable K frame {f!r}")
        kinds.append(m.group(1))
    wf = _wframes(w)
    need([kd for kd, _ in wf] == kinds, f"{label}: frames {[kd for kd, _ in wf]} != K-A reference frames {kinds}")
    klefts = [_K_RIGHT_LEFT.match(f).group(1) for f in k["frames"] if f.startswith("uiRightF(") and _K_RIGHT_LEFT.match(f)]
    wlefts = [left for kd, left in wf if kd == "RightF" and isinstance(left, str)]
    need(wlefts == klefts, f"{label}: pending operands {wlefts} != K-A {klefts}")
    m = _K_RET_U.match(k["control"])
    if m:
        need(_wvalue(_wfields(w)["control"]) == m.group(1), f"{label}: control value != K-A {k['control']}")
    kenv = [] if k["environment"] == ".Map" else _K_BIND.findall(k["environment"])
    need(sorted(_wenv(w)) == sorted(kenv), f"{label}: environment names {sorted(_wenv(w))} != K-A {sorted(kenv)}")


def custom_witness_materialize(c):
    for prog in c["programs"]:
        src, name = prog["source"], prog["label"]
        sc = {**c, "source": src}
        base = _base({**sc, "expect": prog["expect_halted"]} if "expect_halted" in prog else sc, f"{name} unchunked")
        steps = _tc(runx(sc, extra=["--chunk", "1"]), f"{name} single-step plan")["chunks"]
        # final state: one chunk, decoded outcome is the authored value
        fin = runx(sc, extra=["--chunk", "100000", "--chunk-witness"])
        ftc = _tc(fin, f"{name} final")
        _same_run(fin, base, f"{name} witness run")
        fw = ftc["chunks"][-1]["witness"]
        _witness_ok(fw, f"{name} final")
        need("Halted" in _wfields(fw)["outcome"]["raw"], f"{name}: final decoded outcome {_wfields(fw)['outcome']['raw']}")
        need(_wvalue(_wfields(fw)["outcome"]) == prog["final"], f"{name}: final value {_wvalue(_wfields(fw)['outcome'])!r} != {prog['final']!r}")
        for k in _COUNTERS:
            need(fw["decoded"][k] == base.summary[k], f"{name}: final decoded {k} != one-trace summary")
        need(_wfields(fw)["frames"]["items"] == [], f"{name}: halted state still has frames")
        for st in prog["states"]:
            lbl = f"{name} return under {'/'.join(st['frames'])}"
            # select the cut from the owner view only: the first Return whose pending frames are the
            # authored kinds (innermost first)
            cands = [ch for ch in steps[:-1] if "Return" in ch["boundary"]["view"]["control"]
                     and list(reversed(_owner_kinds(ch["boundary"]["view"]["frames"]))) == st["frames"]]
            need(cands, f"{lbl}: the owner run never returns under frames {st['frames']}")
            e = cands[0]["end_event"]
            r = runx(sc, extra=["--chunk-at", e, "--chunk-witness"])
            tc = _tc(r, lbl)
            _same_run(r, base, lbl)
            ch = tc["chunks"][0]
            need(ch["end_event"] == e, f"{lbl}: cut at {ch['end_event']} not {e}")
            w = ch["witness"]
            _witness_ok(w, lbl)
            _witness_vs_owner(sc, src, ch, lbl)
            _k_crosscheck(sc, src, w, lbl)
            fl = _wfields(w)
            need("Running" in fl["outcome"]["raw"], f"{lbl}: outcome {fl['outcome']['raw']}")
            need(_wvalue(fl["control"]) == st["control"], f"{lbl}: decoded control value {_wvalue(fl['control'])!r} != {st['control']!r}")
            need(_wenv(w) == st["env"], f"{lbl}: decoded environment {_wenv(w)} != {st['env']}")
            fr = _wframes(w)
            lefts = [left for kind, left in fr if kind == "RightF"]
            need(lefts == st["pending_lefts"], f"{lbl}: pending operands innermost-first {lefts} != {st['pending_lefts']}")
            need([k for k, _ in fr] == st["frames"], f"{lbl}: decoded frames {[k for k, _ in fr]} != {st['frames']}")
            # determinism: same state under another plan, and on a repeated run
            half = str(max(1, int(e) // 2))
            r2 = runx(sc, extra=["--chunk-at", f"{half},{e}", "--chunk-witness"]) if half != e else r
            w2 = _tc(r2, f"{lbl} replanned")["chunks"][1 if half != e else 0]["witness"]
            for key in ("decoded", "plan", "cache_key_scope"):
                need(w2[key] == w[key], f"{lbl}: witness {key} depends on the chunk plan")
            w3 = _tc(runx(sc, extra=["--chunk-at", e, "--chunk-witness"]), f"{lbl} repeat")["chunks"][0]["witness"]
            need(w3 == w, f"{lbl}: materialization is not deterministic across runs")
    # immutable inputs are part of the reusable key: a different entry argument changes the decoded state
    a, b = c["input_pair"]
    wa = _tc(runx(c, extra=["--chunk", "100000", "--chunk-witness"], source=a), "input A")["chunks"][-1]["witness"]
    wb = _tc(runx(c, extra=["--chunk", "100000", "--chunk-witness"], source=b), "input B")["chunks"][-1]["witness"]
    need(wa["decoded"] != wb["decoded"], "different immutable inputs materialized the same snapshot")
    need(wa["cache_key_scope"] == wb["cache_key_scope"], "cache key scope varies with the input")
    _refused({**c, "source": c["programs"][0]["source"]}, ["--chunk-at", "0", "--chunk-witness"], "invalid witness plan")


# ---- native relation inventory (U023 scenario 1) -------------------------------------
#
# Two layers. (1) Honesty: the inventory's exercised rule multiset is re-derived from the owner
# event stream of an independent --chunk-events run; its summary is recomputed from its rows;
# every named fragment/consumer file and pub fn entry must exist in the tree, and no C6 fragment
# file may exist unclaimed; table_sha256 is recomputed. Any lie fails as "inventory dishonest".
# (2) Spec gates (UIREQ-023 scenario 1 THEN-clause): factorial and tree fold must be routed through
# a candidate native realization, so the inventory must name a dispatch constraint owner as well as
# scalar/frame/data owners, give every exercised model transition a native/circuit consumer, and
# wire the recursion/custom-decider adapter. A hand-authored, source-only gap report cannot meet
# these, so the check fails honestly until a native realization exists.

import hashlib as _hashlib
import os as _os
from collections import Counter as _Counter

_OWNERS = ("dispatch", "scalar", "frame", "data")


def _repo_file(rel, label):
    need(isinstance(rel, str) and rel and not rel.startswith("/") and ".." not in rel.split("/"),
         f"inventory dishonest: {label}: bad path {rel!r}")
    path = engine.REPO / rel
    need(path.is_file(), f"inventory dishonest: {label}: named file {rel} does not exist")
    return path.read_text()


# ---- compiled-and-run receipts for native fragments (U023) ---------------------------
#
# A fragment may claim evidence "compiled-and-run" only with a committed receipt
# (crates/mori-backend/src/sherlock_machine/testdata/receipts/*.json, format
# moriarty-sherlock-fragment-receipt/1). The receipt's own words are never trusted: every field is
# cross-checked against the factory run log (qa/sherlock/runs.jsonl), the run trace on disk (pinned
# by sha256), git history (the run's commit is an ancestor of HEAD and the fragment, consumer and
# test files are unchanged since), and the test sources (each named test is a #[test] fn whose body,
# or a local helper it calls, uses a pub fn of the fragment or its consumer; a mutated-witness
# rejection test is required). "cargo check" or "--no-run" never counts as run.

import re as _re2
import shlex as _shlex
import subprocess as _subprocess

RECEIPT_DIR = "crates/mori-backend/src/sherlock_machine/testdata/receipts/"
RECEIPT_FORMAT = "moriarty-sherlock-fragment-receipt/1"
SDK_CHECK = "ProofPreimage::check (nonproving)"
_EVIDENCE = ("source-only, uncompiled and unrun", "compiled-and-run")
_REJECT_MARKERS = ("is_err", "Err(", "assert!(!", "should_panic", "reject", "unsatisf", "refus")


class RepoOps:
    """Real repository oracle (git + files + factory run log). Tests substitute a fake."""

    def __init__(self, root):
        self.root = root

    def _git(self, *a):
        return _subprocess.run(["git", "-C", str(self.root), *a], capture_output=True, text=True)

    def read(self, rel):
        p = self.root / rel
        return p.read_text() if p.is_file() else None

    def tracked_clean(self, rel):
        if self._git("ls-files", "--error-unmatch", rel).returncode != 0:
            return False
        return self._git("diff", "--quiet", "HEAD", "--", rel).returncode == 0

    def tracked_names(self, d):
        out = self._git("ls-files", "--", d + "/").stdout.split()
        return [x[len(d) + 1:] for x in out if "/" not in x[len(d) + 1:]]

    def is_ancestor(self, sha):
        return self._git("merge-base", "--is-ancestor", sha, "HEAD").returncode == 0

    def unchanged_since(self, sha, rels):
        return self._git("diff", "--quiet", sha, "--", *rels).returncode == 0

    def runs(self):
        text = self.read("qa/sherlock/runs.jsonl") or ""
        out = []
        for line in text.splitlines():
            try:
                out.append(json.loads(line))
            except ValueError:
                pass
        return out


def _pub_fns(text):
    return set(_re2.findall(r"\bpub(?:\([^)]*\))?\s+fn\s+(\w+)", text or ""))


def _fn_body(text, fn):
    m = _re2.search(r"\bfn\s+" + _re2.escape(fn) + r"\s*[(<]", text)
    if not m:
        return None
    nxt = _re2.search(r"\n(?:#\[|fn\s|pub\s+fn\s)", text[m.end():])
    return text[m.start(): m.end() + (nxt.start() if nxt else len(text))]


def _is_test_fn(text, fn):
    m = _re2.search(r"#\[test\][^\n]*\n(?:\s*#\[[^\n]*\n)*\s*(?:pub\s+)?fn\s+" + _re2.escape(fn) + r"\s*\(", text)
    return m is not None


def _expanded(text, fn):
    body = _fn_body(text, fn) or ""
    extra = []
    for callee in set(_re2.findall(r"\b(\w+)\s*\(", body)) - {fn}:
        b = _fn_body(text, callee)
        if b:
            extra.append(b)
    return body + "\n".join(extra)


def verify_receipt(rule, fr, ops, label):
    """Fail unless fragment `fr` (an inventory fragment entry) is honestly evidenced."""
    ev = fr.get("evidence")
    need(ev in _EVIDENCE, f"inventory dishonest: {label}: {rule} evidence {ev!r} is not in the vocabulary {_EVIDENCE}")
    if ev == _EVIDENCE[0]:
        need(not fr.get("receipt"), f"inventory dishonest: {label}: {rule} is source-only but carries a receipt")
        return False
    bad = f"inventory dishonest: {label}: {rule} receipt"
    rp = fr.get("receipt")
    need(isinstance(rp, str) and rp.startswith(RECEIPT_DIR) and rp.endswith(".json") and ".." not in rp.split("/"),
         f"{bad}: path {rp!r} is not under {RECEIPT_DIR}")
    need(ops.tracked_clean(rp), f"{bad}: {rp} is not committed (untracked or modified)")
    try:
        rc = json.loads(ops.read(rp) or "")
    except ValueError:
        raise Fail(f"{bad}: {rp} is not JSON")
    need(rc.get("format") == RECEIPT_FORMAT, f"{bad}: format {rc.get('format')!r}")
    for k in ("file", "entry", "consumer"):
        need(rc.get(k) == fr.get(k), f"{bad}: {k} {rc.get(k)!r} != inventory {fr.get(k)!r}")
    need(rc.get("rule") == rule, f"{bad}: names rule {rc.get('rule')!r}")
    need(rc.get("sdk_check") == SDK_CHECK, f"{bad}: sdk_check {rc.get('sdk_check')!r} (only {SDK_CHECK!r} is accepted)")
    sha = rc.get("commit")
    need(isinstance(sha, str) and _re2.fullmatch(r"[0-9a-f]{40}", sha or "") is not None, f"{bad}: commit {sha!r}")
    need(ops.is_ancestor(sha), f"{bad}: commit {sha[:12]} is not an ancestor of HEAD")
    cmd = rc.get("command") or ""
    argv = _shlex.split(cmd)
    need(argv[:2] == ["cargo", "test"] and "--no-run" not in argv, f"{bad}: command {cmd!r} is not a cargo test run")
    # the factory run log, not the receipt, says what ran
    recs = [r for r in ops.runs() if r.get("run") == rc.get("run")]
    need(len(recs) == 1, f"{bad}: run {rc.get('run')!r} is not in qa/sherlock/runs.jsonl")
    rec = recs[0]
    need(rec.get("exit") == 0, f"{bad}: run {rec['run']} exited {rec.get('exit')}")
    need(rec.get("head") == sha, f"{bad}: run {rec['run']} was on {str(rec.get('head'))[:12]}, not {sha[:12]}")
    need(rec.get("cmd") == cmd, f"{bad}: run {rec['run']} ran {rec.get('cmd')!r}, not the receipt command")
    need(rec.get("trace") == rc.get("trace"), f"{bad}: trace path differs from the run log")
    trace = ops.read(rc.get("trace") or "")
    need(trace is not None, f"{bad}: trace {rc.get('trace')} is not on disk (unverifiable)")
    need(_hashlib.sha256(trace.encode()).hexdigest() == rc.get("trace_sha256"), f"{bad}: trace sha256 differs (trace edited)")
    results = _re2.findall(r"^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed", trace, _re2.M)
    need(results, f"{bad}: trace has no cargo test result")
    passed = sum(int(p) for _, p, _ in results)
    failed = sum(int(f) for _, _, f in results)
    need(all(s == "ok" for s, _, _ in results) and failed == 0 and rc.get("failed") == 0,
         f"{bad}: the run had failures")
    need(passed == rc.get("passed") and passed > 0, f"{bad}: passed {rc.get('passed')} != trace {passed}")
    need(not _re2.search(r"^test \S+ \.\.\. FAILED$", trace, _re2.M), f"{bad}: trace shows a failed test")
    # named tests: present and ok in the trace, real #[test] fns that exercise this fragment
    tests = rc.get("tests") or []
    roles = {t.get("role") for t in tests}
    need("positive" in roles and "rejects-mutated-witness" in roles,
         f"{bad}: needs a positive test and a mutated-witness rejection test, has {sorted(r for r in roles if r)}")
    need(roles <= {"positive", "rejects-mutated-witness"}, f"{bad}: unknown test roles {roles}")
    api = _pub_fns(ops.read(fr["file"])) | _pub_fns(ops.read(fr["consumer"]) if fr.get("consumer") else "")
    need(api, f"{bad}: fragment exposes no pub fn")
    modrs = ops.read(fr["file"].rsplit("/", 1)[0] + "/mod.rs") or ""
    api |= {alias for orig, alias in _re2.findall(r"\b(\w+)\s+as\s+(\w+)", modrs) if orig in api}
    test_files = []
    for t in tests:
        name, tf = t.get("name") or "", t.get("file") or ""
        need(_re2.search(r"^test " + _re2.escape(name) + r" \.\.\. ok$", trace, _re2.M),
             f"{bad}: test {name} did not pass in the trace")
        text = ops.read(tf) if tf.startswith("crates/") else None
        need(text is not None, f"{bad}: test file {tf!r} does not exist")
        fn = name.split("::")[-1]
        stem = tf.rsplit("/", 1)[-1][:-3]
        need("/tests/" in tf or stem in name.split("::"), f"{bad}: test {name} is not in {tf}")
        need(_is_test_fn(text, fn), f"{bad}: {tf} has no #[test] fn {fn}")
        body = _expanded(text, fn)
        need(any(_re2.search(r"\b" + _re2.escape(a) + r"\b", body) for a in api),
             f"{bad}: test {fn} never calls the fragment or its consumer")
        if t["role"] == "rejects-mutated-witness":
            need(any(m in body for m in _REJECT_MARKERS), f"{bad}: test {fn} asserts no rejection")
        test_files.append(tf)
    paths = [fr["file"]] + ([fr["consumer"]] if fr.get("consumer") else []) + test_files
    need(ops.unchanged_since(sha, sorted(set(paths))), f"{bad}: fragment, consumer or tests changed since {sha[:12]}")
    return True

def _inventory_honest(c, src, name, ops=None):
    ops = ops or RepoOps(engine.REPO)
    r = runx(c, extra=["--native-inventory"], source=src)
    need_executed(r, f"{name} inventory run")
    need(engine.outcome_kind(r)[0] == "Halted", f"{name}: inventory run did not halt: {r.outcome}")
    ni = r.doc.get("native_inventory")
    need(isinstance(ni, dict) and ni.get("format") == "moriarty-sherlock-native-inventory/1",
         f"{name}: no native_inventory report")
    # independent exercised-rule multiset from the owner event stream
    ev = runx(c, extra=["--chunk", "1000000", "--chunk-events"], source=src)
    events = [e["event"] for e in _tc(ev, f"{name} event run")["chunks"][0]["events"]]
    need(len(events) == int(r.summary["events"]) == int(ni["events"]), f"inventory dishonest: {name}: event count")
    fired = _Counter(e["rule_id"] for e in events)
    claimed = {}
    for row in ni["exercised"]:
        need(row["rule"] not in claimed, f"inventory dishonest: {name}: rule {row['rule']} listed twice")
        claimed[row["rule"]] = int(row["count"])
    need(claimed == dict(fired), f"inventory dishonest: {name}: exercised rules differ from the fired rules: "
         f"missing {sorted(set(fired) - set(claimed))}, extra {sorted(set(claimed) - set(fired))}, "
         f"miscounted {sorted(k for k in claimed if k in fired and claimed[k] != fired[k])}")
    # table identity
    table = ni["table"]
    want = _hashlib.sha256(json.dumps(table, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    need(ni["table_sha256"] == want, f"inventory dishonest: {name}: table_sha256 does not hash the reported table")
    need(ni["table_provenance"] == "hand-authored", f"{name}: table provenance {ni['table_provenance']!r}")
    rows = {row["rule"]: row for row in table["rows"]}
    # summary recomputed from the rows
    with_frag = sorted(k for k, row in ((x["rule"], x) for x in ni["exercised"]) if row.get("fragment"))
    gaps = sorted(x["rule"] for x in ni["exercised"] if not x.get("fragment"))
    unlisted = sorted(x["rule"] for x in ni["exercised"] if x.get("table") == "unlisted" or x["rule"] not in rows)
    s = ni["summary"]
    need(int(s["rules"]) == len(ni["exercised"]) and int(s["with_c6_fragment"]) == len(with_frag),
         f"inventory dishonest: {name}: summary counts {s['rules']}/{s['with_c6_fragment']} vs rows {len(ni['exercised'])}/{len(with_frag)}")
    need(sorted(s["gaps"]) == gaps and sorted(s["unlisted"]) == unlisted, f"inventory dishonest: {name}: summary gaps/unlisted")
    need(s["complete"] is (not gaps and not unlisted), f"inventory dishonest: {name}: complete={s['complete']} with {len(gaps)} gaps")
    for x in ni["exercised"]:
        row = rows.get(x["rule"])
        if row is None:
            continue
        for k in ("family", "owner", "fragment"):
            need(x.get(k) == row.get(k), f"inventory dishonest: {name}: exercised {x['rule']} {k} differs from its table row")
        need(x["owner"] in _OWNERS, f"inventory dishonest: {name}: {x['rule']} owner {x['owner']!r}")
        need(x["rule"] in ni["owners"][x["owner"]]["rules"], f"inventory dishonest: {name}: owner {x['owner']} does not list {x['rule']}")
    for o in _OWNERS:
        body = ni["owners"][o]
        frag_rules = sorted(k for k in body["rules"] if (rows.get(k) or {}).get("fragment"))
        need(sorted(body["rules_with_c6_fragment"]) == frag_rules, f"inventory dishonest: {name}: owner {o} fragment list")
        sc_ = body.get("section_constraints")
        need(sc_ is None or isinstance(sc_, (list, str)), f"inventory dishonest: {name}: owner {o} section_constraints {sc_!r}")
        for f in sc_ if isinstance(sc_, list) else []:
            _repo_file(f, f"{o} section constraint")

    def _paths(x):
        if isinstance(x, str):
            return [x] if x.endswith(".rs") else []
        if isinstance(x, dict):
            return [q for v in x.values() for q in _paths(v)]
        if isinstance(x, list):
            return [q for v in x for q in _paths(v)]
        return []
    for f in _paths(ni["owners"].get("sections")):
        _repo_file(f, "section emitter")
    # every named fragment and consumer exists with its entry and rule
    claimed_files = set()
    frags = [(k, row["fragment"]) for k, row in rows.items() if row.get("fragment")]
    frags += [(u["rule"], u) for u in table["unexercised_c6_fragments"]]
    # dispatch-owner candidates: same file/entry/rule/evidence checks as every other fragment
    dfr = table.get("dispatch_fragments") or []
    need(isinstance(dfr, list), f"inventory dishonest: {name}: dispatch_fragments is not a list")
    for df in dfr:
        need(df.get("rule") == "dispatch" and df.get("file") and df.get("entry"),
             f"inventory dishonest: {name}: malformed dispatch fragment {df}")
    frags += [(df["rule"], df) for df in dfr]
    cand = ni["owners"]["dispatch"].get("candidate")
    need(cand is None or cand in {df["file"] for df in dfr},
         f"inventory dishonest: {name}: dispatch candidate {cand} is not a claimed dispatch fragment")
    for rule, fr in frags:
        text = _repo_file(fr["file"], f"{rule} fragment")
        need(f"pub fn {fr['entry']}" in text, f"inventory dishonest: {name}: {fr['file']} has no pub fn {fr['entry']}")
        need(rule in text, f"inventory dishonest: {name}: {fr['file']} never names {rule}")
        fr["verified_run"] = verify_receipt(rule, fr, ops, name)
        claimed_files.add(fr["file"])
        if fr.get("consumer"):
            _repo_file(fr["consumer"], f"{rule} consumer")
            claimed_files.add(fr["consumer"])
    for row in rows.values():
        for nc in row.get("narrow_carriers") or []:
            text = _repo_file(nc["file"], f"{row['rule']} narrow carrier")
            need(f"pub fn {nc['entry']}" in text, f"inventory dishonest: {name}: {nc['file']} has no pub fn {nc['entry']}")
            claimed_files.add(nc["file"])
    for om in table["other_machine_fragments"]:
        text = _repo_file(om["file"], f"{om['rule']} other-machine fragment")
        need(f"pub fn {om['entry']}" in text, f"inventory dishonest: {name}: {om['file']} has no pub fn {om['entry']}")
        claimed_files.add(om["file"])
    # no hidden C6 fragment: every c6_*_step.rs / c6_*_consumer.rs in the tree is claimed
    d = "crates/mori-backend/src/sherlock_machine"
    # committed fragments only: untracked work in progress is not part of the realization at HEAD
    for fn in sorted(ops.tracked_names(d)):
        if fn.startswith("c6_") and (fn.endswith("_step.rs") or fn.endswith("_consumer.rs")):
            need(f"{d}/{fn}" in claimed_files, f"inventory dishonest: {name}: C6 fragment {d}/{fn} is not in the inventory")
    _repo_file(ni["recursion_adapter"]["relation"], "recursion relation")
    need(ni["evidence"].startswith("inventory only"), f"{name}: inventory evidence over-claimed: {ni['evidence']!r}")
    return ni


def custom_native_inventory(c):
    invs = [(p["label"], _inventory_honest(c, p["source"], p["label"])) for p in c["programs"]]
    # the same realization across programs
    a = invs[0][1]
    for name, ni in invs[1:]:
        for key in ("table_sha256", "realization", "interface", "owners", "recursion_adapter", "step_records", "prerequisites"):
            need(ni[key] == a[key], f"{name}: {key} differs from {invs[0][0]} (not one fixed relation across programs)")
    # spec gates that a gap report can satisfy
    rz = a["realization"]
    need(rz["kind"] in ("universal", "per-program", "finite-family") and
         rz["universal"] is (rz["kind"] == "universal") and rz["per_program"] is (rz["kind"] == "per-program"),
         f"realization class inconsistent: {rz}")
    it = a["interface"]
    need(it.get("bounded") and it["step_input_sections"] and it["step_output_sections"], f"no bounded StepInput/StepOutput interface: {it}")
    need(a["step_records"]["native_produces_step_records"] is False and "materializeStep" in a["step_records"]["producer"],
         "complete StepRecords not produced separately by host materializeStep")
    need(a["prerequisites"] == {"verify_proof": False, "ledger10": False}, f"VerifyProof/ledger10 prerequisite introduced: {a['prerequisites']}")
    # spec gates that require an actual candidate native realization
    gates = []
    disp = a["owners"]["dispatch"]
    if not disp.get("native_selection") or not disp.get("section_constraints"):
        gates.append(f"no dispatch constraint owner is named ({disp.get('status')})")
    for o in ("scalar", "frame", "data"):
        if not a["owners"][o].get("section_constraints"):
            gates.append(f"no {o} constraint owner")
    for name, ni in invs:
        s = ni["summary"]
        if not s["complete"]:
            gates.append(f"{name}: {len(s['gaps'])} of {s['rules']} exercised transitions have no native/circuit consumer")
        rows = {row["rule"]: row for row in ni["table"]["rows"]}
        unrun = sorted(x["rule"] for x in ni["exercised"]
                       if x.get("fragment") and not (rows.get(x["rule"]) or {}).get("fragment", {}).get("verified_run"))
        if unrun:
            gates.append(f"{name}: {len(unrun)} fragments have no verified compiled-and-run receipt")
    ra = a["recursion_adapter"]
    if ra.get("c6_fragments_wired") is not True:
        gates.append(f"recursion/custom-decider adapter not wired ({ra.get('status')})")
    need(not gates, "SPEC GATE (honest inventory, no candidate native realization): " + "; ".join(gates))


# ---- profile /2 suspension and finance (U014 U015 U017 U018 U019 U020) -----------------
#
# Authored from the spec (UIREQ-014..020) and docs/sherlock/design/suspension-and-finance.md
# (rev 4, §3.3-§8), not from the implementation. The CLI/JSON contract is the note's §3.5:
#   start:  mori simulate SRC --execution-profile sherlock-helper-microstep/2 --source-profile P
#           --execution-config C --target T (--entry | --action A --scenario S) [--checkpoint-out F]
#   resume: mori simulate SRC --execution-profile .../2 --source-profile P --execution-config C
#           --target T --resume F --observation OBS [--checkpoint-out F2]
#   status/exit: waiting 0, halted 0, not_ready 3, refused 4, rejected 1, exhausted 1.
# Independent oracles: the /1 route (pending-frame shape of the same pure program), the legacy
# Core/2 --action/--resume/--event route (U018), and models/sherlock/oracles/financial-prefixes.json
# case F01 (reserve 68, liability 10, gross debit 32, receipt 30, residual 10).
# Until the /2 entrypoint exists every scenario FAILS with "not-implemented:" (exit 1, per
# docs/evaluator-loop.md); exit 2 stays reserved for a broken case.

import tempfile as _tempfile

_EXIT2 = {"waiting": 0, "halted": 0, "not_ready": 3, "refused": 4, "rejected": 1, "exhausted": 1}
_CK_RE = _re2.compile(r"[0-9a-f]{64}")


class _Work:
    """One scratch directory per scenario: checkpoint files must persist between calls."""

    def __init__(self, c, route):
        self.c = c
        self.route = route
        self.dir = Path(_tempfile.mkdtemp(prefix="dw-holdout-p2-"))
        self.cfg = engine.config2_file(self.dir)
        (self.dir / "target.json").write_text("{}")
        self.n = 0

    def put(self, name, text):
        p = self.dir / name
        p.write_text(text if isinstance(text, str) else json.dumps(text))
        return name

    def base(self, src):
        return ["simulate", src, "--execution-profile", engine.PROFILE2,
                "--source-profile", engine.SOURCE_PROFILE2[self.route],
                "--execution-config", str(self.cfg), "--target", "target.json"]

    def call(self, args, label, expect=None):
        r = engine.mori_raw(args, self.dir)
        if r.returncode == 2 or r.doc is None or "status" not in (r.doc or {}):
            raise Fail(f"not-implemented: {label}: no /2 report (rc={r.returncode}, "
                       f"stderr={r.stderr.strip()[:160]!r}, out={r.raw[:120]!r})")
        st = r.doc["status"]
        need(st in _EXIT2, f"{label}: unknown status {st!r}")
        need(r.returncode == _EXIT2[st], f"{label}: status {st} exited {r.returncode}, contract says {_EXIT2[st]}")
        need(r.doc.get("profile") == engine.PROFILE2, f"{label}: report profile {r.doc.get('profile')!r}")
        need(r.doc.get("authentication") == "none" and r.doc.get("consumption") == "untracked" and r.doc.get("qualification"),
             f"{label}: report lacks the Core/2 labels (authentication none, consumption untracked, qualification)")
        if expect is not None:
            need(st == expect, f"{label}: status {st!r}, expected {expect!r}: {json.dumps(r.doc)[:300]}")
        return r.doc

    def start(self, src, label, ck=None, action=None, scenario=None, extra=()):
        a = self.base(src)
        a += ["--action", action, "--scenario", scenario] if action else ["--entry"]
        if ck:
            a += ["--checkpoint-out", ck]
        return self.call(a + list(extra), label)

    def resume(self, src, ck, obs, label, out=None, expect=None, extra=()):
        self.n += 1
        of = self.put(f"obs{self.n}.json", obs)
        a = self.base(src) + ["--resume", ck, "--observation", of] + (["--checkpoint-out", out] if out else [])
        return self.call(a + list(extra), label, expect)

    def read(self, name):
        p = self.dir / name
        return p.read_bytes() if p.is_file() else None


def _obs(ck, site, label, values):
    return {"format": "sherlock-observation/2", "checkpoint": ck, "site": site, "label": label,
            "values": [{"name": n, "value": {"type": t, "value": v}} for n, t, v in values]}


def _plain(v):
    """Report values may be plain or typed ({"type","value"}); compare as plain strings/bools."""
    if isinstance(v, dict) and "value" in v and set(v) <= {"type", "value"}:
        return _plain(v["value"])
    if isinstance(v, int) and not isinstance(v, bool):
        return str(v)
    return v


def _waiting_ok(d, label, site, lab, binders):
    need(d["status"] == "waiting", f"{label}: status {d['status']!r}, expected waiting: {json.dumps(d)[:300]}")
    need(d.get("site") == site and d.get("label") == lab, f"{label}: waiting at {d.get('site')}/{d.get('label')}, expected {site}/{lab}")
    need(isinstance(d.get("checkpoint"), str) and _CK_RE.fullmatch(d["checkpoint"]), f"{label}: checkpoint is not a 64-hex CK")
    got = [(b.get("name"), b.get("type")) for b in d.get("binders") or []]
    need(got == binders, f"{label}: binder layout {got} != {binders}")
    cw, aw = int(d["committed_work"]), int(d["attempted_work"])
    need(cw > 0 and aw >= 0, f"{label}: committed/attempted work {cw}/{aw}")


def _frame_kind(f):
    if isinstance(f, str):
        m = _re2.match(r"\W*(\w+)", f)
        return m.group(1) if m else f
    if isinstance(f, dict):
        if isinstance(f.get("kind"), str):
            return f["kind"]
        if len(f) == 1:
            return next(iter(f))
    return json.dumps(f)


# -- pure route ------------------------------------------------------------------------

def _pending_shape_v1(c):
    """Independent oracle: under /1 the await-free analog of the program, during inner's body,
    has the pending stack bottom-to-top [outer ReturnF, RightF Add (left 10), inner ReturnF]."""
    r = runx(c, extra=["--chunk", "1"], source=c["analog_v1"])
    chunks = _tc(r, "/1 analog")["chunks"][:-1]
    shapes = [[next(iter(f)) for f in ch["boundary"]["view"]["frames"]] for ch in chunks]
    need(c["pending_frames"] in shapes, f"/1 analog never reaches the pending stack {c['pending_frames']}")
    return c["pending_frames"]


def custom_suspend_pending(c):
    # /1 keeps refusing await (note §3.3, §9)
    v1 = runx(c, source=c["source_v1_await"])
    need(v1.doc is not None and "execution" not in v1.doc and v1.doc.get("evaluation_performed") is False,
         "/1 admitted an await program")
    shape = _pending_shape_v1(c)
    w = _Work(c, "draft")
    src = w.put("p.mori", c["source"])
    d = w.start(src, "start", ck="ck.bin")
    _waiting_ok(d, "start", c["site"], c["await_label"], [tuple(b) for b in c["binders"]])
    need(d.get("fin") is None, "pure route waiting report carries a financial prefix")
    data = w.read("ck.bin")
    need(data, "--checkpoint-out wrote no checkpoint file")
    need(str(d.get("checkpoint_file") or "").rsplit("/", 1)[-1] == "ck.bin", f"checkpoint_file {d.get('checkpoint_file')!r}")
    frames = d.get("frames") or []
    kinds = [_frame_kind(f) for f in frames]
    need(kinds == shape, f"waiting frames bottom-to-top {kinds} != pending stack {shape}")
    pend = json.dumps(frames[shape.index("RightF")])
    need("Add" in pend and "10" in pend, f"pending caller frame does not hold Add with left 10: {pend}")
    # the stack is ordered: bottom is outer's return frame, top is inner's (kinds alone are a palindrome)
    for idx, fn in c["return_functions"]:
        need(_re2.search(r"\b" + fn + r"\b", json.dumps(frames[idx])),
             f"waiting frame {idx} does not name the function {fn} it returns from: {json.dumps(frames[idx])}")
    # deterministic, complete checkpoint: a second start is byte-identical; resuming only the
    # written bytes (fresh directory, nothing else carried) completes the run
    w2 = _Work(c, "draft")
    src2 = w2.put("p.mori", c["source"])
    d2 = w2.start(src2, "second start", ck="ck.bin")
    need(w2.read("ck.bin") == data and d2["checkpoint"] == d["checkpoint"], "checkpoint is not deterministic")
    w3 = _Work(c, "draft")
    src3 = w3.put("p.mori", c["source"])
    (w3.dir / "ck.bin").write_bytes(data)
    h = w3.resume(src3, "ck.bin", _obs(d["checkpoint"], c["site"], c["await_label"], [("v", "UInt", c["ok_value"])]),
                  "resume from bytes alone", expect="halted")
    need(_plain(h.get("value")) == c["expect"], f"resume from the checkpoint bytes gave {h.get('value')!r}, expected {c['expect']}")
    # a corrupted checkpoint is refused, not resumed (the encoding is the state)
    bad = bytearray(data)
    bad[len(bad) // 2] ^= 0x01
    (w3.dir / "bad.bin").write_bytes(bytes(bad))
    rf = w3.resume(src3, "bad.bin", _obs(d["checkpoint"], c["site"], c["await_label"], [("v", "UInt", c["ok_value"])]), "corrupted checkpoint")
    need(rf["status"] == "refused" and rf.get("stage") in ("R0", "R1", "R2", "R3"), f"corrupted checkpoint: {rf['status']} {rf.get('stage')}")


def custom_resume_pending(c):
    w = _Work(c, "draft")
    src = w.put("p.mori", c["source"])
    d = w.start(src, "start", ck="ck.bin")
    _waiting_ok(d, "start", c["site"], c["await_label"], [tuple(b) for b in c["binders"]])
    ck, site, lab = d["checkpoint"], c["site"], c["await_label"]
    data = w.read("ck.bin")
    need(data, "--checkpoint-out wrote no checkpoint file")
    ok = _obs(ck, site, lab, [("v", "UInt", c["ok_value"])])
    h = w.resume(src, "ck.bin", ok, "resume v=4", expect="halted")
    need(_plain(h.get("value")) == c["expect"], f"resume returned {h.get('value')!r}, expected {c['expect']}")
    need(h.get("fin") is None, "pure halted report carries a financial prefix")
    need(int(h["committed_work"]) > int(d["committed_work"]), "resume committed no further work")
    h2 = w.resume(src, "ck.bin", ok, "resume v=4 again", expect="halted")
    need(h2 == h, "local resume is not a pure function of (checkpoint, observation)")
    # not ready: unchanged checkpoint (byte-identical with --checkpoint-out), host-only attempted work
    nr = w.resume(src, "ck.bin", _obs(ck, site, lab, [("v", "UInt", c["bad_value"])]), "resume v=5", out="nr.bin", expect="not_ready")
    need(nr.get("checkpoint") == ck and nr.get("site") == site, "not_ready changed the checkpoint identity")
    need(nr.get("reason") == "guard_false", f"not_ready reason {nr.get('reason')!r}")
    need(int(nr["attempted_work"]) > 0, "not_ready reports no attempted guard work")
    need(w.read("nr.bin") == data, "not_ready --checkpoint-out is not byte-identical to the input checkpoint")
    need(w.read("ck.bin") == data, "not_ready modified the input checkpoint file")
    nr2 = w.resume(src, "nr.bin", _obs(ck, site, lab, [("v", "UInt", c["bad_value"])]), "resume v=5 again", expect="not_ready")
    need(nr2.get("attempted_work") == nr["attempted_work"], "not_ready attempted work is not deterministic")
    h3 = w.resume(src, "nr.bin", ok, "resume after not_ready", expect="halted")
    need(_plain(h3.get("value")) == c["expect"] and h3 == h, "the checkpoint left by not_ready does not resume like the original")
    # refusals R1-R5 (note §4): first failing stage, exit 4, no successor
    refusals = [
        ("R3", _obs("0" * 64, site, lab, [("v", "UInt", c["ok_value"])]), "observation bound to another checkpoint"),
        ("R4", _obs(ck, site, "other", [("v", "UInt", c["ok_value"])]), "wrong label"),
        ("R4", _obs(ck, site + 1, lab, [("v", "UInt", c["ok_value"])]), "wrong site"),
        ("R5", _obs(ck, site, lab, [("w", "UInt", c["ok_value"])]), "wrong binder name"),
        ("R5", _obs(ck, site, lab, [("v", "Bool", True)]), "wrong binder sort"),
        ("R5", _obs(ck, site, lab, [("v", "UInt", "0" + c["ok_value"])]), "non-canonical UInt"),
        ("R5", _obs(ck, site, lab, [("v", "UInt", c["ok_value"]), ("x", "UInt", "1")]), "extra binder"),
        ("R5", _obs(ck, site, lab, []), "missing binder"),
    ]
    for stage, o, why in refusals:
        rf = w.resume(src, "ck.bin", o, why, out="refused.bin", expect="refused")
        need(rf.get("stage") == stage, f"{why}: refused at {rf.get('stage')!r}, expected {stage}")
        need(rf.get("error"), f"{why}: refusal carries no error")
        need(w.read("refused.bin") is None, f"{why}: a refused resume wrote a successor checkpoint")
    alt = w.put("alt.mori", c["source_other_program"])
    rf = w.resume(alt, "ck.bin", ok, "checkpoint of another program", expect="refused")
    need(rf.get("stage") == "R2", f"checkpoint of another program refused at {rf.get('stage')!r}, expected R2")
    # R1: a terminal checkpoint is refused, never "not ready"
    hh = w.resume(src, "ck.bin", ok, "halted with checkpoint-out", out="halted.bin", expect="halted")
    if w.read("halted.bin"):
        rf = w.resume(src, "halted.bin", ok, "resume a halted checkpoint", expect="refused")
        need(rf.get("stage") == "R1", f"halted checkpoint refused at {rf.get('stage')!r}, expected R1")


# -- financial route -------------------------------------------------------------------

def _oracle_prefix(pid):
    o = json.loads((engine.REPO / "models/sherlock/oracles/financial-prefixes.json").read_text())
    p = next(x for x in o["prefixes"] if x["id"] == pid)
    ev = {e["id"]: e for e in o["ordered_events"]}
    ref = p["committed_state_ref"]
    m = _re2.fullmatch(r"ordered_events\[(\d+)\]\.(pre|post)", ref)
    state = o["ordered_events"][int(m.group(1))][m.group(2)]
    return p, state, [ev[i] for i in p["committed_event_ids"]]


def _balances(snap):
    return {b["account"]: str(b["amount"]) for b in snap.get("balances", [])}


def _liabs(rows):
    return {l["name"]: str(l["amount"]) for l in rows or []}


def _fin(d, label):
    f = d.get("fin")
    need(isinstance(f, dict), f"{label}: financial report has no fin block")
    need(isinstance(f.get("committed"), dict) and isinstance(f["committed"].get("snapshot"), dict),
         f"{label}: fin block has no committed snapshot")
    return f


def _legacy(c, w, src, label, resume=None, event=None):
    args = ["simulate", src] + (["--resume", resume, "--event", event] if resume else
                                ["--action", c["action"], "--scenario", "scenario.json"])
    r = engine.mori_raw(args, w.dir)
    need(r.doc is not None and "status" in r.doc, f"legacy Core/2 {label}: no report ({r.stderr[:200]!r})")
    return r.doc


def _fin_start(c, w, label="fin start", source_key="source"):
    src = w.put("f.mori", c[source_key])
    w.put("scenario.json", c["scenario"])
    d = w.start(src, label, ck="ck.bin", action=c["action"], scenario="scenario.json")
    _waiting_ok(d, label, c["site"], c["await_label"], [tuple(b) for b in c["binders"]])
    return src, d


def _check_first_prefix(f, label, order=(("Transfer", "30"), ("Fee", "2"))):
    p, state, events = _oracle_prefix("first-paid-prefix")
    bal = _balances(f["committed"]["snapshot"])
    for acct, amt in state["balances"].items():
        need(bal.get(acct) == str(amt), f"{label}: committed {acct} {bal.get(acct)} != oracle {amt}")
    li = _liabs(f["committed"]["snapshot"].get("liabilities"))
    need(li.get("pay-beneficiary") == str(state["closing_liability"]), f"{label}: liability {li} != oracle {state['closing_liability']}")
    acc = f.get("accounting") or {}
    need(str(acc.get("gross_debit")) == str(state["gross_debit"]) and str(acc.get("fees")) == str(state["fees"])
         and str(acc.get("recipient_receipt")) == str(state["net_receipts"]),
         f"{label}: accounting {acc} != oracle gross {state['gross_debit']} fees {state['fees']} receipt {state['net_receipts']}")
    rd = _liabs(f.get("residual_duties"))
    need(rd == {"pay-beneficiary": str(state["residual_duty"]["amount"])}, f"{label}: residual duties {rd}")
    eff = f["committed"].get("effects") or []
    got = [(e.get("kind"), str(e.get("amount"))) for e in eff if e.get("kind") in ("Transfer", "Fee")]
    need(got == [tuple(x) for x in order], f"{label}: committed transfer/fee effects {got} != source order {list(order)}")
    return state


def custom_fin_first_prefix(c):
    w = _Work(c, "fin")
    _, d = _fin_start(c, w)
    f = _fin(d, "first paid prefix")
    _check_first_prefix(f, "first paid prefix")
    need(not (f.get("discarded_tentative") or {}).get("effects"), "a waiting prefix reports a discarded delta")
    need(int(f["committed"].get("logical_work", -1)) >= 0, "committed logical work not reported")


def custom_fin_await_core2(c):
    w = _Work(c, "fin")
    src, d = _fin_start(c, w)
    f = _fin(d, "/2 waiting")
    leg = _legacy(c, w, src, "start")
    need(leg["status"] == "waiting", f"legacy Core/2 did not wait: {leg.get('status')} {leg.get('error')}")
    lb = leg["continuation"]["boundary"]
    need(_balances(f["committed"]["snapshot"]) == _balances(lb), "committed balances differ from Core/2 continuation boundary")
    need(_liabs(f["committed"]["snapshot"].get("liabilities")) == _liabs(lb.get("liabilities")), "committed liabilities differ from Core/2")
    need(_liabs(f.get("residual_duties")) == _liabs(leg.get("residual")), f"residual {f.get('residual_duties')} != Core/2 {leg.get('residual')}")
    need(str(f["committed"].get("logical_work")) == str(leg["continuation"]["logical"]["committed"]),
         f"committed logical work {f['committed'].get('logical_work')} != Core/2 {leg['continuation']['logical']['committed']}")
    need(d.get("label") == leg["continuation"].get("site") or d.get("label") == c["await_label"], "await site differs from Core/2")
    _check_first_prefix(f, "/2 waiting vs oracle")
    # false observation: /2 not_ready leaves the checkpoint, Core/2 does not advance
    w.put("cont.json", leg["continuation"])
    w.put("ev-false.json", {"format": "moriarty-stage-event/1", "action": c["action"], "label": c["await_label"],
                            "values": [{"name": "go", "value": {"type": "Bool", "value": False}}]})
    w.put("ev-true.json", {"format": "moriarty-stage-event/1", "action": c["action"], "label": c["await_label"],
                           "values": [{"name": "go", "value": {"type": "Bool", "value": True}}]})
    lf = _legacy(c, w, src, "false resume", resume="cont.json", event="ev-false.json")
    need(lf["status"] == "waiting" and lf.get("advanced") is False, "legacy Core/2 false resume advanced")
    data = w.read("ck.bin")
    nr = w.resume(src, "ck.bin", _obs(d["checkpoint"], c["site"], c["await_label"], [("go", "Bool", False)]),
                  "/2 false observation", out="nr.bin", expect="not_ready")
    need(w.read("nr.bin") == data, "/2 not_ready changed the checkpoint bytes")
    # true observation: both complete with the same boundary and residual (present-zero duty kept)
    lt = _legacy(c, w, src, "true resume", resume="cont.json", event="ev-true.json")
    need(lt["status"] == "complete", f"legacy Core/2 true resume: {lt.get('status')} {lt.get('error')}")
    h = w.resume(src, "ck.bin", _obs(d["checkpoint"], c["site"], c["await_label"], [("go", "Bool", True)]), "/2 true observation", expect="halted")
    fh = _fin(h, "/2 complete")
    post = lt["segment"]["execution"]["post"]
    need(_balances(fh["committed"]["snapshot"]) == _balances(post), "completed balances differ from Core/2")
    need(_liabs(fh["committed"]["snapshot"].get("liabilities")) == _liabs(post.get("liabilities")), "completed liabilities differ from Core/2")
    need(_liabs(fh.get("residual_duties")) == _liabs(lt.get("residual")), "completed residual differs from Core/2")


def custom_fin_local_failure(c):
    w = _Work(c, "fin")
    src, d = _fin_start(c, w)
    f = _fin(d, "waiting before the failing segment")
    _check_first_prefix(f, "retained prefix at the await")
    rj = w.resume(src, "ck.bin", _obs(d["checkpoint"], c["site"], c["await_label"], [("go", "Bool", True)]),
                  "second segment", expect="rejected")
    fr = _fin(rj, "rejected")
    need(rj.get("code") is not None, "rejected report has no code")
    need(fr["committed"]["snapshot"] == f["committed"]["snapshot"], "the retained committed prefix changed on local failure")
    need(fr["committed"].get("effects") == f["committed"].get("effects"), "committed effect log changed on local failure")
    _check_first_prefix(fr, "retained prefix after failure")
    disc = (fr.get("discarded_tentative") or {}).get("effects") or []
    need(disc, "the discarded tentative delta is not reported")
    need(any(e.get("kind") == "Transfer" and str(e.get("amount")) == c["discarded_transfer"] for e in disc),
         f"discarded delta does not hold the failed segment's applied transfer {c['discarded_transfer']}: {disc}")
    need(int(rj["committed_work"]) > int(d["committed_work"]), "later consumed work of the failed segment is not reported")
    need(fr.get("ledger_rollback") == "none-inferred", f"ledger rollback {fr.get('ledger_rollback')!r}")
    need(_liabs(fr.get("residual_duties")) == {"pay-beneficiary": "10"}, f"residual duties after failure {fr.get('residual_duties')}")
    # stated difference from Core/2: the legacy rejection publishes no segment or continuation
    leg = _legacy(c, w, src, "start")
    w.put("cont.json", leg["continuation"])
    w.put("ev.json", {"format": "moriarty-stage-event/1", "action": c["action"], "label": c["await_label"],
                      "values": [{"name": "go", "value": {"type": "Bool", "value": True}}]})
    lr = _legacy(c, w, src, "failing resume", resume="cont.json", event="ev.json")
    need(lr["status"] == "rejected", f"legacy Core/2 did not reject the failing segment: {lr.get('status')}")


def _policy_argv(c):
    p = engine.HOLDOUT / "contracts" / f"{c['contract']}.json"
    if not p.is_file():
        raise Fail(f"not-implemented: no qa/holdout/contracts/{c['contract']}.json naming the phase-policy selection")
    k = json.loads(p.read_text())
    argv = (k.get("phase_policy") or {}).get("retain-guaranteed-fees/1")
    need(isinstance(argv, list) and argv and all(isinstance(a, str) for a in argv),
         f"contract {c['contract']} has no phase_policy.retain-guaranteed-fees/1 argv")
    return argv


def custom_fin_phase_policy(c):
    retain = _policy_argv(c)
    for policy, extra in (("all-or-nothing/1", []), ("retain-guaranteed-fees/1", retain)):
        w = _Work(c, "fin")
        src = w.put("f.mori", c["source"])
        w.put("scenario.json", c["scenario"])
        d = w.start(src, f"{policy} start", ck="ck.bin", action=c["action"], scenario="scenario.json", extra=extra)
        _waiting_ok(d, f"{policy} start", c["site"], c["await_label"], [tuple(b) for b in c["binders"]])
        f0 = _fin(d, f"{policy} waiting")
        _check_first_prefix(f0, f"{policy} waiting", order=c["prefix_order"])
        rj = w.resume(src, "ck.bin", _obs(d["checkpoint"], c["site"], c["await_label"], [("go", "Bool", True)]),
                      f"{policy} failing segment", expect="rejected", extra=extra)
        fr = _fin(rj, f"{policy} rejected")
        bal = _balances(fr["committed"]["snapshot"])
        rbp = fr.get("retained_by_policy")
        if policy == "all-or-nothing/1":
            need(bal == _balances(f0["committed"]["snapshot"]), f"default policy retained part of the failed segment: {bal}")
            need(not rbp or not rbp.get("effects"), f"default policy reports policy-retained effects: {rbp}")
        else:
            want = dict(_balances(f0["committed"]["snapshot"]))
            fee = int(c["guaranteed_fee"])
            want["reserve"] = str(int(want["reserve"]) - fee)
            want["fee_recipient"] = str(int(want["fee_recipient"]) + fee)
            need(bal == want, f"retain policy committed {bal}, expected the sealed fee only {want}")
            need(isinstance(rbp, dict) and rbp.get("policy") == policy, f"retained_by_policy {rbp}")
            eff = rbp.get("effects") or []
            need([(e.get("kind"), str(e.get("amount"))) for e in eff] == [("Fee", str(fee))],
                 f"policy-retained effects {eff} != one Fee {fee}")
        # four separate members: committed prefix, policy-retained fee, discarded delta, residual duties
        need(_liabs(fr.get("residual_duties")) == {"pay-beneficiary": "10"}, f"{policy}: residual duties {fr.get('residual_duties')}")
        need((fr.get("discarded_tentative") or {}).get("effects") is not None, f"{policy}: discarded delta member missing")
        need(fr.get("ledger_rollback") == "none-inferred", f"{policy}: ledger rollback {fr.get('ledger_rollback')!r}")
    # the note's worked example: a fee phase that fails is discarded whole (nothing retained)
    w = _Work(c, "fin")
    src = w.put("f.mori", c["source_fee_phase_fails"])
    w.put("scenario.json", c["scenario"])
    rj = w.start(src, "fee phase fails", action=c["action"], scenario="scenario.json", extra=retain)
    need(rj["status"] == "rejected", f"failing fee phase: status {rj['status']}")
    fr = _fin(rj, "failing fee phase")
    need(_balances(fr["committed"]["snapshot"]) == {k: str(v) for k, v in c["entry_balances"].items()},
         f"failing fee phase retained something: {_balances(fr['committed']['snapshot'])}")
    rbp = fr.get("retained_by_policy") or {}
    need(not rbp.get("effects"), f"failing fee phase reports retained fees {rbp}")
    # admission: a fee after a fallible effect is refused under the retain policy
    w = _Work(c, "fin")
    src = w.put("f.mori", c["source_fee_after_effect"])
    w.put("scenario.json", c["scenario"])
    r = engine.mori_raw(w.base(src) + ["--action", c["action"], "--scenario", "scenario.json"] + retain, w.dir)
    need(r.returncode != 0 and not (r.doc or {}).get("status") in ("waiting", "halted"),
         "a Fee after a fallible effect was admitted under retain-guaranteed-fees/1")


CHECKS = {
    "value": check_value,
    "outcome": check_outcome,
    "failure": check_failure,
    "typed-completed": check_typed_completed,
    "same-machine": custom_same_machine,
    "supported-profile-report": custom_supported_profile_report,
    "distinct-failure-class": custom_distinct_failure_class,
    "left-to-right": custom_left_to_right,
    "depth-sweep": custom_depth_sweep,
    "work-identity": custom_work_identity,
    "occurrences": custom_occurrences,
    "correspondence": custom_correspondence,
    "evidence-class": custom_evidence_class,
    "chunk-transparency": custom_chunk_transparency,
    "chunk-boundary-return": custom_chunk_boundary_return,
    "reference-slices": custom_reference_slices,
    "chunk-represent": custom_chunk_represent,
    "witness-materialize": custom_witness_materialize,
    "native-inventory": custom_native_inventory,
    "suspend-pending": custom_suspend_pending,
    "resume-pending": custom_resume_pending,
    "fin-first-prefix": custom_fin_first_prefix,
    "fin-await-core2": custom_fin_await_core2,
    "fin-local-failure": custom_fin_local_failure,
    "fin-phase-policy": custom_fin_phase_policy,
}


def main(argv):
    if len(argv) != 2 or not argv[1].isdigit():
        print("usage: uireq.py NNN", file=sys.stderr)
        return 1
    nnn = argv[1].zfill(3)
    case_path = engine.HOLDOUT / "cases" / "uireq" / nnn / "case.json"
    if not case_path.is_file():
        print(f"FAIL uireq-{nnn}: no such case", file=sys.stderr)
        return engine.EXIT_FAIL
    case = json.loads(case_path.read_text())
    failures, unimpl, passed = [], [], []
    for sc in case["scenarios"]:
        if sc["status"] != "runnable":
            unimpl.append(sc)
            continue
        try:
            for chk in sc["exec"]["checks"]:
                CHECKS[chk["kind"]](chk)
            passed.append(sc)
        except Fail as e:
            failures.append((sc["name"], str(e)))
        except engine.HostUnfit as e:
            print(f"HOSTUNFIT uireq-{nnn}: {e}")
            return engine.EXIT_UNFIT
        except engine.BuildFailed as e:
            print(f"FAIL uireq-{nnn}: system under test does not build\n{e}")
            return engine.EXIT_FAIL
        except Exception as e:  # harness/tool error counts as failure of the system under test
            failures.append((sc["name"], f"{type(e).__name__}: {e}"))
    cov = ",".join(sorted({s.get("coverage", "none") for s in passed})) or "-"
    if failures:
        for name, msg in failures:
            print(f"FAIL uireq-{nnn} [{name}]: {msg}")
        return engine.EXIT_FAIL
    if unimpl:
        # docs/evaluator-loop.md: not-implemented is a failure (exit 1); exit 2 is a broken case
        need_msgs = "; ".join(s.get("blocker", "specified only") for s in unimpl)
        print(f"FAIL uireq-{nnn}: not-implemented: {need_msgs} (runnable scenarios passed: {len(passed)})")
        return engine.EXIT_FAIL
    print(f"PASS uireq-{nnn} ({len(passed)} scenario(s), coverage={cov})")
    return engine.EXIT_PASS


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except engine.HostUnfit as e:
        print(f"HOSTUNFIT: {e}")
        sys.exit(engine.EXIT_UNFIT)
    except engine.BuildFailed as e:
        print(f"FAIL: system under test does not build\n{e}")
        sys.exit(engine.EXIT_FAIL)
