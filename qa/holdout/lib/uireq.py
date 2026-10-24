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
                             fuel=check.get("fuel"), stack=check.get("stack"))


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
