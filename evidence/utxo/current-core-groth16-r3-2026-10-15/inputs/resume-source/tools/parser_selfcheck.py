"""Own synthetic checks of operator-proposals/libtest_rows.py. Scope and effects:
records/PARSER-CHECK-SCOPE.md (written before this ran). Stdlib only; no subprocess,
network or candidate/operator/reviewer code; synthetic inputs only."""
import hashlib
import json
import os
import resource
import sys
import types

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

OWN = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015"
HELPER = OWN + "/operator-proposals/libtest_rows.py"
WORK = OWN + "/records/parser-check"
RESULT = OWN + "/records/PARSER-CHECK-RESULT.json"

os.mkdir(WORK)  # refuses to run twice into the same directory
helper_bytes = open(HELPER, "rb").read()
lt = types.ModuleType("libtest_rows")
lt.__file__ = HELPER
exec(compile(helper_bytes, HELPER, "exec"), lt.__dict__)

cases = []


def case(cid, ok, observed):
    cases.append({"id": cid, "ok": bool(ok), "observed": observed})


# Verbatim copy of the reviewed operator's parser lines (run_pathb.py c9cd7237, lines
# 231-237 and the e5_identity_lines filter of line 344), used only to reproduce N-04.
def legacy_tests_of(text):
    res = {}
    for line in text.splitlines():
        if line.startswith('test ') and ' ... ' in line:
            name, _, outcome = line[5:].partition(' ... ')
            res.setdefault(name.strip(), []).append(outcome.strip())
    return res


def legacy_identity(text):
    return [l for l in text.splitlines() if l.startswith('identity ')]


R2_LIKE = "\n".join([
    "",
    "running 0 tests",
    "",
    "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s",
    "",
    "",
    "running 2 tests",
    "test build_identity_is_recomputed_from_the_copied_bytes ... identity build_manifest_sha256 " + "a" * 64 + " (16 entries equal to the compiled crate bytes)",
    "identity core_closure_sha256 " + "b" * 64 + " (70 files recomputed from <root>/moriarty)",
    "identity compiler_build_digest " + "c" * 64,
    "ok",
    "test heldout_vectors_core_and_relation ... heldout OK   H1-P1: Core reference equals reviewer oracle",
    "heldout OK   H1-P1: W 36 vs reviewer 36",
    "heldout OK   H1-R1: Core refusal mori::core::requires step Some(0) vs reviewer mori::core::requires Some(0)",
    "ok",
    "",
    "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.31s",
    "",
])

# C1
rt = lt.result_tests(R2_LIKE)
case("C1-inline-first-line",
     rt["outcomes"] == {"build_identity_is_recomputed_from_the_copied_bytes": ["ok"], "heldout_vectors_core_and_relation": ["ok"]}
     and len(rt["identity_lines"]) == 3 and rt["identity_lines"][0].startswith("identity build_manifest_sha256 ") and not rt["anomalies"],
     rt)
rows = lt.rows_of(lt.parse_stream(R2_LIKE))
tmap, _ = lt.tables(rows)
case("C1-r2-rows", tmap.get("heldout", {}).get("rows") == 3 and tmap["heldout"]["distinct_ids"] == 2, tmap.get("heldout"))

# C2
legacy = legacy_tests_of(R2_LIKE)
legacy_ids = legacy_identity(R2_LIKE)
case("C2-legacy-defect-reproduced",
     legacy["heldout_vectors_core_and_relation"] == ["heldout OK   H1-P1: Core reference equals reviewer oracle"]
     and len(legacy_ids) == 2 and not any("build_manifest_sha256" in l for l in legacy_ids),
     {"legacy_outcomes": legacy, "legacy_identity_lines": legacy_ids})

# C3
R3_ROWS = "\n".join([
    "running 1 test",
    "test targeted_mutations ... mutations OK   M-01: [UNSAT] is_satisfied = false (expected UNSAT)",
    "mutations OPEN M-07: [TARGET-GAP] no FrameCells option can represent the unregistered duty",
    "mutations FAIL M-14: [CORE-REFUSED] Core refused the M-14 construction",
    "mutations OK   M-13/program_hash: [UNSAT] is_satisfied = false (expected UNSAT)",
    "summary mutations ok=2 fail=1 open=1 classes=CORE-REFUSED=1,TARGET-GAP=1,UNSAT=2",
    "summary mutations ok=3 fail=0 open=1 classes=UNSAT=3,TARGET-GAP=1",
    "FAILED",
    "",
    "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s",
])
p3 = lt.parse_stream(R3_ROWS)
tm3, checks3 = lt.tables(lt.rows_of(p3))
m = tm3.get("mutations", {})
case("C3-typed-rows",
     m.get("tags") == {"OK": 2, "OPEN": 1, "FAIL": 1} and m.get("classes") == {"UNSAT": 2, "TARGET-GAP": 1, "CORE-REFUSED": 1}
     and [c["equal"] for c in checks3] == [True, False] and p3["binaries"][0]["tests"][0]["outcome"] == "FAILED" and not p3["anomalies"],
     {"table": m, "checks": checks3, "anomalies": p3["anomalies"]})

# C4
R_FAIL = "\n".join([
    "running 1 test",
    "test static_refusals ... static OK   S-1: [REFUSED] x",
    "static FAIL S-9/states: [BOUND] y",
    "FAILED",
    "",
    "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s",
])
p4 = lt.parse_stream(R_FAIL)
case("C4-failed-nocapture", p4["binaries"][0]["tests"][0]["outcome"] == "FAILED" and len(p4["binaries"][0]["tests"][0]["captured"]) == 2 and not p4["anomalies"], p4)

# C5
R_CAPTURE = "\n".join([
    "",
    "running 2 tests",
    "test a_test ... ok",
    "test b_test ... FAILED",
    "",
    "failures:",
    "",
    "---- b_test stdout ----",
    "some captured line",
    "test lookalike ... not a header inside a failure section",
    "",
    "failures:",
    "    b_test",
    "",
    "test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s",
])
p5 = lt.parse_stream(R_CAPTURE)
case("C5-capture-failures-section",
     [t["outcome"] for t in p5["binaries"][0]["tests"]] == ["ok", "FAILED"]
     and p5["failure_sections"].get("b_test") == ["some captured line", "test lookalike ... not a header inside a failure section"]
     and not p5["anomalies"],
     p5)

# C6
R_ANOM = "\n".join([
    "running 3 tests",
    "test x ... first line",
    "test y ... ok",
    "test z ... row",
])
p6 = lt.parse_stream(R_ANOM)
kinds = sorted(a["kind"] for a in p6["anomalies"])
case("C6-anomalies",
     "new test header while a test was open" in kinds and "stream ended with an open test" in kinds
     and [t["outcome"] for t in p6["binaries"][0]["tests"]] == ["UNTERMINATED", "ok", "UNTERMINATED"],
     p6["anomalies"])
p6b = lt.parse_stream("running 2 tests\ntest only ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out")
case("C6-running-count-mismatch", any(a["kind"] == "running count differs from parsed tests" for a in p6b["anomalies"]), p6b["anomalies"])

# C7
p7 = lt.parse_stream("running 1 test\ntest c ... ignored, slow\n\ntest result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out")
case("C7-ignored-with-reason", p7["binaries"][0]["tests"][0]["outcome"] == "ignored, slow" and not p7["anomalies"], p7)

# C8
trace = {"entries": [
    {"id": "HW-A", "class": "RELATION-EXECUTED", "refs": [{"table": "mutations", "row": "M-01"}], "e4": ["e4_ok"]},
    {"id": "HW-B", "class": "RELATION-EXECUTED", "refs": [{"table": "mutations", "row": "M-14"}], "e4": []},
    {"id": "HW-C", "class": "RELATION-PARTIAL", "refs": [{"table": "mutations", "row": "M-99"}], "e4": ["e4_missing"]},
    {"id": "HW-D", "class": "TARGET-GAP", "refs": [{"table": "mutations", "row": "M-07"}], "e4": []},
    {"id": "HW-E", "class": "STRUCTURAL-SOURCE", "refs": [], "e4": []},
]}
hw = {h["id"]: h for h in lt.resolve_hw(trace, tm3, {"e4_ok": ["ok"]})}
case("C8-hw-resolution",
     hw["HW-A"]["overall"] == "ALL-REFS-OK" and hw["HW-B"]["overall"] == "FAIL-OR-MISSING" and hw["HW-C"]["refs"][0]["status"] == "MISSING"
     and hw["HW-D"]["overall"] == "SOME-REFS-OPEN" and hw["HW-E"]["overall"] == "NO-EXECUTABLE-REF"
     and hw["HW-C"]["e4"] == [{"test": "e4_missing", "outcomes": ["MISSING"]}],
     hw)

# C9: CLI analyze in-process, synthetic run dir under records/parser-check/.
run = os.path.join(WORK, "run")
out = os.path.join(WORK, "analysis")
os.mkdir(run)
os.mkdir(out)
E4 = "\n".join(["", "running 1 test", "test e4_ok ... ok", "", "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out", ""])
open(os.path.join(run, "E4.stdout"), "w").write(E4)
open(os.path.join(run, "E5-r3.stdout"), "w").write(R2_LIKE.replace("running 2 tests", "running 2 tests"))
legacy_result = {"tests": {"E4": {"outcomes": legacy_tests_of(E4)}, "E5-r3": {"outcomes": legacy_tests_of(R2_LIKE)}}}
open(os.path.join(run, "RESULT.json"), "w").write(json.dumps(legacy_result))
req = {"e5_step": "E5-r3", "expected_tests": {"E4": ["e4_ok"], "E5-r3": ["build_identity_is_recomputed_from_the_copied_bytes", "heldout_vectors_core_and_relation"]}}
req_path = os.path.join(WORK, "request.json")
open(req_path, "w").write(json.dumps(req))
tr_path = os.path.join(WORK, "trace.json")
open(tr_path, "w").write(json.dumps(trace))
e0_path = os.path.join(WORK, "E0.stdout")
open(e0_path, "w").write(json.dumps({"build_manifest_recomputed": "a" * 64, "core_closure_recomputed": "b" * 64, "compiler_build_digest_predicted": "c" * 64}))
args = ["analyze", "--run", run, "--request", req_path, "--hw-trace", tr_path, "--out-dir", out, "--e0", e0_path]
rc1 = lt.main(args)
rc2 = lt.main(args)
files = sorted(os.listdir(out))
index = open(os.path.join(out, "ANALYSIS-INDEX.jsonl")).read().splitlines()
analyses = [f for f in files if f.startswith("ANALYSIS-") and f.endswith(".json") and not f.endswith(".error.json") and f != "ANALYSIS-INDEX.jsonl"]
first = json.load(open(os.path.join(out, analyses[0]))) if analyses else {}
step = first.get("steps", {}).get("E5-r3", {})
case("C9-analyze-two-dated-files",
     rc1 == 0 and rc2 == 0 and len(analyses) == 2 and len(index) == 2 and step.get("missing") == [] and step.get("all_ok") is True
     and first.get("identity_equals_E0") == {"build_manifest_sha256": True, "core_closure_sha256": True, "compiler_build_digest_vs_predicted": True}
     and isinstance(first.get("result_json_consistency", {}).get("E5-r3"), dict)
     and "heldout_vectors_core_and_relation" in first["result_json_consistency"]["E5-r3"]["outcome_differences"],
     {"rc": [rc1, rc2], "files": files, "index_lines": len(index), "identity_equals_E0": first.get("identity_equals_E0"),
      "result_json_consistency": first.get("result_json_consistency")})
before = sorted(os.listdir(run))
rc3 = lt.main(["analyze", "--run", run, "--request", req_path, "--hw-trace", tr_path, "--out-dir", run])
case("C9-out-dir-inside-run-refused", rc3 == 2 and sorted(os.listdir(run)) == before, {"rc": rc3, "run_files": sorted(os.listdir(run))})
rc4 = lt.main(["analyze", "--run", run, "--request", os.path.join(WORK, "absent.json"), "--hw-trace", tr_path, "--out-dir", out])
errs = [f for f in os.listdir(out) if f.endswith(".error.json")]
case("C9-error-artifact", rc4 == 3 and len(errs) == 1 and len(open(os.path.join(out, "ANALYSIS-INDEX.jsonl")).read().splitlines()) == 3,
     {"rc": rc4, "errors": errs})

record = {"format": "evidence-repair-resume-parser-check/1", "helper": HELPER, "helper_sha256": hashlib.sha256(helper_bytes).hexdigest(),
          "scope": "records/PARSER-CHECK-SCOPE.md", "synthetic_only": True, "cases": cases,
          "passed": sum(c["ok"] for c in cases), "failed": [c["id"] for c in cases if not c["ok"]]}
fd = os.open(RESULT, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
with os.fdopen(fd, "w") as f:
    json.dump(record, f, indent=1, sort_keys=True)
    f.write("\n")
print(json.dumps({"passed": record["passed"], "total": len(cases), "failed": record["failed"], "helper_sha256": record["helper_sha256"]}))
sys.exit(0 if not record["failed"] else 1)
