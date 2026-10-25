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


def _inventory_honest(c, src, name):
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
    for rule, fr in frags:
        text = _repo_file(fr["file"], f"{rule} fragment")
        need(f"pub fn {fr['entry']}" in text, f"inventory dishonest: {name}: {fr['file']} has no pub fn {fr['entry']}")
        need(rule in text, f"inventory dishonest: {name}: {fr['file']} never names {rule}")
        need(fr["evidence"] == "source-only, uncompiled and unrun", f"{name}: {rule} fragment evidence over-claimed: {fr['evidence']!r}")
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
    for fn in sorted(_os.listdir(engine.REPO / d)):
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
        unrun = sorted({x["fragment"]["evidence"] for x in ni["exercised"] if x.get("fragment")} - {"compiled-and-run"})
        if unrun:
            gates.append(f"{name}: fragments are not a routed realization ({', '.join(unrun)})")
    ra = a["recursion_adapter"]
    if ra.get("c6_fragments_wired") is not True:
        gates.append(f"recursion/custom-decider adapter not wired ({ra.get('status')})")
    need(not gates, "SPEC GATE (honest inventory, no candidate native realization): " + "; ".join(gates))


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
