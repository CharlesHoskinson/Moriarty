#!/usr/bin/env python3
"""PROPOSED operator helper (evidence-repair resume source author, claude-opus-5-5,
2026-10-15). Stdlib only. UNREVIEWED and NEVER USED BY ANY OPERATOR: every effect below
needs a fresh non-author review before use (result review P-4, N-04).

One parser for both future consumers, so RESULT.json and the analysis cannot disagree:
- the future run operator's RESULT.json test block: `result_tests(text)` replaces
  run_pathb.py `tests_of()` and its `e5_identity_lines` filter (OPERATOR-RESULT-PARSER.diff);
- post-run analysis: the CLI `analyze` uses the same `parse_stream()`.

The defect it fixes (result review section 5, N-04): with `--nocapture`, libtest prints the
first captured line of a test on its own header line, `test NAME ... <first line>`, and
the outcome token (`ok` / `FAILED`) later on a line of its own. The reviewed operator's
`tests_of()` took `<first line>` as the outcome and `e5_identity_lines` (lines starting
with `identity `) missed an identity line printed inline. Here the header is split
first; the outcome is the terminal token; captured lines include the inline one.

Effects (complete list):
- `result-tests LOG`: reads LOG, prints JSON to stdout. Writes nothing.
- `analyze --run RUN --request REQ --hw-trace TRACE --out-dir OUT [--e0 E0.stdout]`:
  reads RUN/E4.stdout, RUN/<E5 step>.stdout, RUN/RESULT.json (if present), REQ, TRACE and
  E0 (all read-only); creates exactly one new file OUT/ANALYSIS-<UTC>.json with O_EXCL
  (never overwrites, so a first defective or failed analysis survives as a dated
  artifact) and appends one line to OUT/ANALYSIS-INDEX.jsonl. If the analysis raises,
  it creates OUT/ANALYSIS-<UTC>.error.json instead. OUT must not be RUN or inside it.
  No network, no subprocess, no import of candidate or operator code; CPU 60 s, AS 1 GiB.
Limitations: a test that prints a line equal to `ok`, `FAILED` or `ignored` ends its
captured block early, and parallel `--nocapture` output can interleave; both are reported
as anomalies only when detectable. The r3 harness runs with --test-threads=1 and never
prints a bare outcome token.
"""
import datetime
import hashlib
import json
import os
import re
import sys

OUTCOMES = ("ok", "FAILED", "ignored")
HEADER = re.compile(r"^test (\S+) \.\.\. ?(.*)$")
RUNNING = re.compile(r"^running (\d+) tests?$")
SUMMARY = re.compile(r"^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out")
FAILURE_SECTION = re.compile(r"^---- (\S+) stdout ----$")
ROW_TYPED = re.compile(r"^(?P<table>\w+) (?P<tag>OK|FAIL|OPEN) +(?P<id>[^:]+): \[(?P<cls>[^\]]+)\] (?P<detail>.*)$")
ROW_PLAIN = re.compile(r"^(?P<table>\w+) (?P<tag>OK|FAIL) +(?P<id>[^:]+): (?P<detail>.*)$")
TABLE_SUMMARY = re.compile(r"^summary (?P<table>\w+) ok=(?P<ok>\d+) fail=(?P<fail>\d+) open=(?P<open>\d+) classes=(?P<classes>.*)$")
IDENTITY = re.compile(r"^identity (?P<key>\S+) (?P<value>\S+)(?: (?P<rest>.*))?$")
KV = re.compile(r"(\w+)=(\S+)")


def _is_outcome(s):
    return s in OUTCOMES or s.startswith("ignored, ")


def parse_stream(text):
    """libtest stdout -> binaries/tests/captured lines. Pure function."""
    binaries, anomalies, failure_sections = [], [], {}
    cur_bin, cur_test, section = None, None, None

    def close(test, outcome):
        test["outcome"] = outcome

    for n, line in enumerate(text.splitlines(), 1):
        m = RUNNING.match(line)
        if m:
            if cur_test is not None:
                close(cur_test, "UNTERMINATED")
                anomalies.append({"line": n, "kind": "test without outcome before 'running'", "test": cur_test["name"]})
                cur_test = None
            section = None
            cur_bin = {"running": int(m.group(1)), "tests": [], "summary": None}
            binaries.append(cur_bin)
            continue
        m = SUMMARY.match(line)
        if m:
            if cur_test is not None:
                close(cur_test, "UNTERMINATED")
                anomalies.append({"line": n, "kind": "test without outcome before 'test result'", "test": cur_test["name"]})
                cur_test = None
            section = None
            summary = {"result": m.group(1), "passed": int(m.group(2)), "failed": int(m.group(3)), "ignored": int(m.group(4)),
                       "measured": int(m.group(5)), "filtered_out": int(m.group(6)), "line": line}
            if cur_bin is None:
                anomalies.append({"line": n, "kind": "summary outside a binary block"})
            else:
                cur_bin["summary"] = summary
            continue
        m = HEADER.match(line)
        if m and section is None:
            if cur_test is not None:
                close(cur_test, "UNTERMINATED")
                anomalies.append({"line": n, "kind": "new test header while a test was open", "test": cur_test["name"]})
                cur_test = None
            if cur_bin is None:
                anomalies.append({"line": n, "kind": "test header outside a binary block"})
                cur_bin = {"running": None, "tests": [], "summary": None}
                binaries.append(cur_bin)
            name, rest = m.group(1), m.group(2)
            test = {"name": name, "outcome": None, "captured": [], "inline_first_line": False, "line": n}
            cur_bin["tests"].append(test)
            if _is_outcome(rest):
                close(test, rest)
            else:
                test["captured"].append(rest)
                test["inline_first_line"] = True
                cur_test = test
            continue
        if cur_test is not None:
            if _is_outcome(line):
                close(cur_test, line)
                cur_test = None
            else:
                cur_test["captured"].append(line)
            continue
        m = FAILURE_SECTION.match(line)
        if m:
            section = m.group(1)
            failure_sections.setdefault(section, [])
            continue
        if line == "failures:" or (section is not None and line == ""):
            if line == "failures:":
                section = None
            continue
        if section is not None:
            failure_sections[section].append(line)
    if cur_test is not None:
        close(cur_test, "UNTERMINATED")
        anomalies.append({"line": None, "kind": "stream ended with an open test", "test": cur_test["name"]})
    for b in binaries:
        if b["running"] is not None and b["running"] != len(b["tests"]):
            anomalies.append({"kind": "running count differs from parsed tests", "running": b["running"], "parsed": len(b["tests"])})
        s = b["summary"]
        if s is not None:
            counted = {o: sum(1 for t in b["tests"] if (t["outcome"] or "").split(",")[0] == o) for o in OUTCOMES}
            if (counted["ok"], counted["FAILED"], counted["ignored"]) != (s["passed"], s["failed"], s["ignored"]):
                anomalies.append({"kind": "summary differs from parsed outcomes", "summary": s["line"], "parsed": counted})
    return {"format": "libtest-rows/1", "binaries": binaries, "anomalies": anomalies, "failure_sections": failure_sections}


def parse_row(line):
    """One captured line -> typed row, r2 row, table summary, identity line or None."""
    m = ROW_TYPED.match(line)
    if m:
        return {"kind": "row", "table": m.group("table"), "tag": m.group("tag"), "id": m.group("id").strip(), "class": m.group("cls"),
                "detail": m.group("detail")}
    m = TABLE_SUMMARY.match(line)
    if m:
        classes = dict(kv.split("=", 1) for kv in m.group("classes").split(",") if "=" in kv)
        return {"kind": "summary", "table": m.group("table"), "ok": int(m.group("ok")), "fail": int(m.group("fail")),
                "open": int(m.group("open")), "classes": {k: int(v) for k, v in classes.items()}}
    m = ROW_PLAIN.match(line)
    if m:
        return {"kind": "row", "table": m.group("table"), "tag": m.group("tag"), "id": m.group("id").strip(), "class": None,
                "detail": m.group("detail")}
    m = IDENTITY.match(line)
    if m:
        return {"kind": "identity", "key": m.group("key"), "value": m.group("value"), "rest": m.group("rest")}
    return None


def result_tests(text):
    """The RESULT.json block of one test step. Same parser as the analysis."""
    p = parse_stream(text)
    outcomes, identity = {}, []
    for b in p["binaries"]:
        for t in b["tests"]:
            outcomes.setdefault(t["name"], []).append(t["outcome"])
            for line in t["captured"]:
                r = parse_row(line)
                if r and r["kind"] == "identity":
                    identity.append(line)
    return {"format": "libtest-rows-result/1", "outcomes": outcomes,
            "summaries": [b["summary"]["line"] for b in p["binaries"] if b["summary"]],
            "identity_lines": identity, "anomalies": p["anomalies"]}


def rows_of(parsed):
    """Every typed row with its test name, in stream order."""
    out = []
    for b in parsed["binaries"]:
        for t in b["tests"]:
            for line in t["captured"]:
                r = parse_row(line)
                if r is not None:
                    r["test"] = t["name"]
                    out.append(r)
    return out


def tables(rows):
    """Per table: tag counts, class counts, distinct ids and per-id tags; plus a check
    that each harness-printed `summary` line equals the rows actually parsed."""
    out, checks = {}, []
    for r in rows:
        if r["kind"] != "row":
            continue
        t = out.setdefault(r["table"], {"rows": 0, "tags": {}, "classes": {}, "ids": {}})
        t["rows"] += 1
        t["tags"][r["tag"]] = t["tags"].get(r["tag"], 0) + 1
        if r["class"] is not None:
            t["classes"][r["class"]] = t["classes"].get(r["class"], 0) + 1
        t["ids"].setdefault(r["id"], []).append(r["tag"])
    for r in rows:
        if r["kind"] != "summary":
            continue
        t = out.get(r["table"], {"tags": {}, "classes": {}})
        parsed = {"ok": t["tags"].get("OK", 0), "fail": t["tags"].get("FAIL", 0), "open": t["tags"].get("OPEN", 0)}
        printed = {"ok": r["ok"], "fail": r["fail"], "open": r["open"]}
        checks.append({"table": r["table"], "printed": printed, "parsed": parsed, "classes_equal": r["classes"] == t["classes"],
                       "equal": printed == parsed and r["classes"] == t["classes"]})
    for t in out.values():
        t["distinct_ids"] = len(t["ids"])
    return out, checks


def resolve_hw(trace, table_map, e4_outcomes):
    """Each HW entry's refs resolved against the printed rows; nothing inferred."""
    out = []
    for e in trace["entries"]:
        refs = []
        for ref in e.get("refs", []):
            tags = table_map.get(ref["table"], {}).get("ids", {}).get(ref["row"])
            if not tags:
                status = "MISSING"
            elif "FAIL" in tags:
                status = "FAIL"
            elif all(t == "OK" for t in tags):
                status = "OK"
            elif all(t == "OPEN" for t in tags):
                status = "OPEN"
            else:
                status = "MIXED-OK-OPEN"
            refs.append({"table": ref["table"], "row": ref["row"], "status": status, "tags": tags or []})
        e4 = [{"test": n, "outcomes": e4_outcomes.get(n, ["MISSING"])} for n in e.get("e4", [])]
        statuses = {r["status"] for r in refs}
        if not refs:
            overall = "NO-EXECUTABLE-REF"
        elif statuses & {"FAIL", "MISSING"}:
            overall = "FAIL-OR-MISSING"
        elif statuses == {"OK"}:
            overall = "ALL-REFS-OK"
        else:
            overall = "SOME-REFS-OPEN"
        out.append({"id": e["id"], "class": e["class"], "refs": refs, "e4": e4, "overall": overall,
                    "note": "row status only; a class other than RELATION-EXECUTED is never upgraded by OK refs"})
    return out


def analyze(run_dir, request, hw_trace, e0_path=None):
    def read_text(p):
        with open(p, "r", encoding="utf-8", errors="replace") as f:
            return f.read()

    def sha_file(p):
        with open(p, "rb") as f:
            return hashlib.sha256(f.read()).hexdigest()

    req = json.loads(read_text(request))
    trace = json.loads(read_text(hw_trace))
    e5_step = req.get("e5_step", "E5-r3")
    inputs = {}
    out = {"format": "libtest-rows-analysis/1", "inputs": inputs, "steps": {}}
    for sid in ("E4", e5_step):
        p = os.path.join(run_dir, sid + ".stdout")
        if not os.path.isfile(p):
            out["steps"][sid] = {"present": False}
            continue
        inputs[p] = sha_file(p)
        parsed = parse_stream(read_text(p))
        rows = rows_of(parsed)
        tmap, checks = tables(rows)
        names = [t["name"] for b in parsed["binaries"] for t in b["tests"]]
        expected = sorted(req["expected_tests"][sid])
        out["steps"][sid] = {
            "present": True,
            "tests": {t["name"]: {"outcome": t["outcome"], "captured_lines": len(t["captured"]), "inline_first_line": t["inline_first_line"]}
                      for b in parsed["binaries"] for t in b["tests"]},
            "expected_tests": expected, "missing": sorted(set(expected) - set(names)), "unexpected": sorted(set(names) - set(expected)),
            "duplicates": sorted({n for n in names if names.count(n) > 1}),
            "all_ok": all(t["outcome"] == "ok" for b in parsed["binaries"] for t in b["tests"]),
            "summaries": [b["summary"] for b in parsed["binaries"] if b["summary"]],
            "anomalies": parsed["anomalies"], "tables": tmap, "summary_checks": checks,
            "identity": {r["key"]: r["value"] for r in rows if r["kind"] == "identity"},
            "probe": [dict(KV.findall(r["detail"]), id=r["id"]) for r in rows if r["kind"] == "row" and r["table"] == "probe"],
        }
        out["steps"][sid]["result_tests_block"] = result_tests(read_text(p))
    e4_outcomes = {}
    if out["steps"].get("E4", {}).get("present"):
        e4_outcomes = {n: [v["outcome"]] for n, v in out["steps"]["E4"]["tests"].items()}
    if out["steps"].get(e5_step, {}).get("present"):
        out["hw"] = resolve_hw(trace, out["steps"][e5_step]["tables"], e4_outcomes)
        inputs[hw_trace] = sha_file(hw_trace)
    if e0_path and os.path.isfile(e0_path):
        inputs[e0_path] = sha_file(e0_path)
        e0 = json.loads(read_text(e0_path))
        ident = out["steps"].get(e5_step, {}).get("identity", {})
        out["identity_equals_E0"] = {
            "build_manifest_sha256": ident.get("build_manifest_sha256") == e0.get("build_manifest_recomputed"),
            "core_closure_sha256": ident.get("core_closure_sha256") == e0.get("core_closure_recomputed"),
            "compiler_build_digest_vs_predicted": ident.get("compiler_build_digest") == e0.get("compiler_build_digest_predicted"),
        }
    res_path = os.path.join(run_dir, "RESULT.json")
    if os.path.isfile(res_path):
        inputs[res_path] = sha_file(res_path)
        res = json.loads(read_text(res_path))
        cmp = {}
        for sid, step in out["steps"].items():
            if not step.get("present"):
                continue
            recorded = res.get("tests", {}).get(sid)
            if recorded is None:
                cmp[sid] = "absent from RESULT.json"
            elif recorded.get("format") == "libtest-rows-result/1":
                cmp[sid] = "equal" if recorded == step["result_tests_block"] else "DIFFERS (same parser: investigate, never rewrite RESULT.json)"
            else:
                legacy = recorded.get("outcomes", {})
                cmp[sid] = {"recorded_by": "legacy tests_of (inline-first-line defect)",
                            "outcome_differences": {n: {"legacy": legacy.get(n), "parsed": v["outcome"]}
                                                    for n, v in step["tests"].items() if legacy.get(n) != [v["outcome"]]}}
        out["result_json_consistency"] = cmp
    out["inputs_sha256"] = inputs
    return out


def _utc_stamp():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")


def _write_new(path, obj):
    data = (json.dumps(obj, indent=1, sort_keys=True) + "\n").encode()
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "wb") as f:
        f.write(data)
    return hashlib.sha256(data).hexdigest()


def main(argv):
    import resource
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    if len(argv) == 2 and argv[0] == "result-tests":
        with open(argv[1], "r", encoding="utf-8", errors="replace") as f:
            print(json.dumps(result_tests(f.read()), indent=1, sort_keys=True))
        return 0
    if not argv or argv[0] != "analyze":
        print("usage: libtest_rows.py result-tests LOG | analyze --run RUN --request REQ --hw-trace TRACE --out-dir OUT [--e0 E0.stdout]", file=sys.stderr)
        return 2
    opts = dict(zip(argv[1::2], argv[2::2]))
    need = {"--run", "--request", "--hw-trace", "--out-dir"}
    if not need <= set(opts) or set(opts) - need - {"--e0"} or len(argv[1:]) % 2:
        print("analyze needs --run --request --hw-trace --out-dir [--e0]", file=sys.stderr)
        return 2
    run = os.path.realpath(opts["--run"])
    out_dir = os.path.realpath(opts["--out-dir"])
    if out_dir == run or out_dir.startswith(run + os.sep):
        print("refusing: --out-dir must be outside the raw run directory", file=sys.stderr)
        return 2
    stamp = _utc_stamp()
    try:
        record = analyze(run, opts["--request"], opts["--hw-trace"], opts.get("--e0"))
        record["at_utc"] = stamp
        name = f"ANALYSIS-{stamp}.json"
        digest = _write_new(os.path.join(out_dir, name), record)
        status = "written"
    except Exception as e:  # recorded as its own dated artifact, never retried here
        name = f"ANALYSIS-{stamp}.error.json"
        digest = _write_new(os.path.join(out_dir, name), {"format": "libtest-rows-analysis-error/1", "at_utc": stamp, "error": repr(e)})
        status = "error"
    fd = os.open(os.path.join(out_dir, "ANALYSIS-INDEX.jsonl"), os.O_WRONLY | os.O_CREAT | os.O_APPEND, 0o644)
    with os.fdopen(fd, "a") as f:
        f.write(json.dumps({"at_utc": stamp, "file": name, "sha256": digest, "status": status}, sort_keys=True) + "\n")
    print(json.dumps({"file": name, "sha256": digest, "status": status}))
    return 0 if status == "written" else 3


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
