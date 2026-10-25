#!/usr/bin/env python3
"""UIREQ holdout runner. Usage: uireq.py NNN

Reads qa/holdout/cases/uireq/NNN/case.json and executes every scenario whose
`status` is "runnable" through the real `mori simulate` Sherlock host route.
Expected outcomes live only in case.json (derived from the openspec spec text).

Exit: 0 all scenarios pass and none unimplemented; 1 any assertion fails;
2 at least one scenario is specified-only (no executable entrypoint yet);
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
        need_msgs = "; ".join(s.get("blocker", "specified only") for s in unimpl)
        print(f"UNIMPLEMENTED: uireq-{nnn}: {need_msgs} (runnable scenarios passed: {len(passed)})")
        return engine.EXIT_UNIMPL
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
