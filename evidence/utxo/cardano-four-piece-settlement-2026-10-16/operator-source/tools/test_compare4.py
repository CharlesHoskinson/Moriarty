#!/usr/bin/env python3
"""OPX2x: extraction and comparison of the filtered `aiken check` JSON report (O''). NOT RUN BY ITS AUTHOR.

Usage: python3 -I test_compare4.py OPX2_LOG OUT_TSV OUT_JSON EXPECTATIONS_JSON PARENT_TSV

Reads exactly the named inputs; writes OUT_TSV and OUT_JSON with mode "x". EXPECTATIONS_JSON
(the candidate's frozen index, 80cf1601...) and PARENT_TSV (attempt 2's raw OP2-tests.tsv,
2771106b..., read for the PC4 host-CEK baseline only, not adopted) must match their pins.

Report rule (aiken v1.1.24, observed in attempt 2's OP2.log): stdout is not a terminal, so the
report is a JSON object; it is the text from the first line equal to "{" through the last line
equal to "}". Schema observed: {seed, summary{total, passed, failed, kind{unit, property}},
modules[{name, summary, tests[{title, status, on_failure, execution_units{mem, cpu}}]}]};
a `fail` test that passes has on_failure "succeed_eventually", any other "fail_immediately".

Expected set: the 125 shard4TestModule names (9 `fail`) and 22 shard4CostModule names, by module.
Classification written to OUT_JSON "classification":
  PREDICTIONS_HOLD            membership exact (147, both modules, nothing else) and every test
                              status "pass" with the expected on_failure;
  FAILED(prediction)          membership exact, but a status or on_failure deviates (preserved);
  UNCLASSIFIED(match-filter)  any membership deviation: a test from another module, a missing or
                              extra name, a duplicate, or zero tests (the filter, not the candidate);
  UNCLASSIFIED(report-schema) a parsed report whose fields or summary are inconsistent.
Exit 0: report parsed, both outputs written. Exit 3: no parseable report (OUT_JSON records the
first compiler diagnostic and location, if any; no TSV). Exit 2: a pin differs (nothing written).
"""
import collections
import hashlib
import json
import re
import sys

PINS = {
    "expectations": "80cf1601480be0c415649ac1b071326b0097f89c2132279b3636a38c15df8393",
    "parentTsv": "2771106b677cb957277f2efe2dbc9338292a5f060d23b63cd0486e9a3f3e8aef",
}
MODULES = ["pvc2/shard4_test", "pvc2/shard4_cost_test"]
ON_FAILURE_FAIL = "succeed_eventually"
ON_FAILURE_PLAIN = "fail_immediately"
LOCATION = re.compile(r"\[(\./[^\]:]+):(\d+):(\d+)\]")
QUALIFIER = ("mem/cpu are host CEK observations of library predicates, decoded through test_ctx, "
             "under aiken v1.1.24's default cost model. They are not ledger ExUnits, not costs of "
             "the compiled validator wrappers, not signature costs and not a budget fit.")


def load_pinned(path, pin, label):
    with open(path, "rb") as f:
        b = f.read()
    if hashlib.sha256(b).hexdigest() != pin:
        sys.stderr.write("digest differs from pin: %s\n" % label)
        sys.exit(2)
    return b


def compile_diagnostic(text):
    lines = text.split("\n")
    first = next((l.strip() for l in lines if l.strip().startswith("Error aiken::")), None)
    loc = LOCATION.search(text)
    return {"firstError": first,
            "firstLocation": None if loc is None else {"path": loc.group(1), "line": int(loc.group(2)), "col": int(loc.group(3))},
            "summaryLines": [l.strip() for l in lines if l.strip().startswith("Summary ")]}


def parse_report(text):
    lines = text.split("\n")
    try:
        s = lines.index("{")
        e = max(i for i, l in enumerate(lines) if l == "}")
        return json.loads("\n".join(lines[s:e + 1])), None
    except ValueError as ex:
        return None, "no JSON report block: %s" % ex


def parent_units(raw):
    out = {}
    for line in raw.decode("utf-8").split("\n"):
        f = line.split("\t")
        if len(f) == 6 and f[0] == "pvc2/shard_cost_test":
            out[f[1]] = (int(f[4]), int(f[5]), f[2])
    return out


def pc4(units, parent):
    """PC4-1..PC4-4 (EXPECTATIONS directionalPredictions), verbatim formulas; plus P04 records."""
    def get(n):
        u = units.get(n)
        return u if u is not None and all(isinstance(x, int) for x in u[:2]) else None
    need = ["s4c_p01_decode", "s4c_p01_s2p", "s4c_p07_decode"] + ["s4c_p01_vs%d" % k for k in range(1, 5)] \
        + ["s4c_p07_vs%d" % k for k in range(1, 5)]
    pneed = ["shc_p01_decode", "shc_p01_vs", "shc_p01_s2p", "shc_p07_decode", "shc_p07_vs"]
    missing = [n for n in need if get(n) is None] + [n for n in pneed if n not in parent]
    if missing:
        return {"evaluated": False, "missing": missing}
    res = {"evaluated": True, "benchStatuses": {n: units[n][2] for n in need},
           "parentBenchStatuses": {n: parent[n][2] for n in pneed}}
    for i, axis in enumerate(["mem", "cpu"]):
        d4 = [get("s4c_p01_vs%d" % k)[i] - get("s4c_p01_decode")[i] for k in range(1, 5)]
        dp = parent["shc_p01_vs"][i] - parent["shc_p01_decode"][i]
        d7 = [get("s4c_p07_vs%d" % k)[i] - get("s4c_p07_decode")[i] for k in range(1, 5)]
        dp7 = parent["shc_p07_vs"][i] - parent["shc_p07_decode"][i]
        s2 = get("s4c_p01_s2p")[i] - get("s4c_p01_decode")[i]
        ps2 = parent["shc_p01_s2p"][i] - parent["shc_p01_decode"][i]
        res[axis] = {
            "PC4-1": {"fourPieceSum": sum(d4), "parentVs": dp, "holds": sum(d4) > dp},
            "PC4-2": {"perPiece": d4, "parentVs": dp, "holdsPerPiece": [x < dp for x in d4], "holds": all(x < dp for x in d4)},
            "PC4-3": {"fourPieceSum": sum(d7), "parentVs": dp7, "holds": sum(d7) > dp7},
            "PC4-4": {"fourPieceS2p": s2, "parentS2p": ps2, "holds": s2 > ps2},
        }
    p04 = {}
    for n4, np in [("s4c_p04_vi_gate", "shc_p04_vi_gate"), ("s4c_p04_s2p", "shc_p04_s2p")] + \
            [("s4c_p04_vs%d" % k, None) for k in range(1, 5)]:
        u = get(n4)
        base = get("s4c_p04_decode")
        if u is not None and base is not None:
            row = {"minusDecode": [u[0] - base[0], u[1] - base[1]]}
            if np and np in parent and "shc_p04_decode" in parent:
                row["parentMinusDecode"] = [parent[np][0] - parent["shc_p04_decode"][0],
                                            parent[np][1] - parent["shc_p04_decode"][1]]
            p04[n4] = row
    if "shc_p04_vs" in parent and "shc_p04_decode" in parent:
        p04["parent shc_p04_vs"] = {"minusDecode": [parent["shc_p04_vs"][0] - parent["shc_p04_decode"][0],
                                                    parent["shc_p04_vs"][1] - parent["shc_p04_decode"][1]]}
    res["p04RecordedNotPredicted"] = p04
    res["caveats"] = [
        "Each s4c bench builds the five-shard transaction; each shc bench built the two-shard one. "
        "Subtracting the same-context decode bench isolates predicate work only approximately.",
        "s4c_p07_decode includes the two harness digest computations (shard4_cost_test header).",
        "Parent rows are attempt 2's raw observation (actual-result review pending), read by digest, not adopted.",
        QUALIFIER,
    ]
    return res


def main():
    if len(sys.argv) != 6:
        sys.stderr.write("usage: test_compare4.py OPX2_LOG OUT_TSV OUT_JSON EXPECTATIONS_JSON PARENT_TSV\n")
        sys.exit(2)
    log, out_tsv, out_json, p_exp, p_parent = sys.argv[1:6]
    exp = json.loads(load_pinned(p_exp, PINS["expectations"], "EXPECTATIONS.json"))
    parent = parent_units(load_pinned(p_parent, PINS["parentTsv"], "parent OP2-tests.tsv"))

    rows = [(exp["shard4TestModule"]["module"], t["name"], bool(t["fail"])) for t in exp["shard4TestModule"]["tests"]]
    rows += [(exp["shard4CostModule"]["module"], n, False) for n in exp["shard4CostModule"]["names"]]
    index_notes = []
    if len(rows) != 147 or sum(r[2] for r in rows) != 9 or sorted({r[0] for r in rows}) != sorted(MODULES):
        index_notes.append("assembled index is not 147 tests / 9 fail / two modules")

    with open(log, encoding="utf-8", errors="replace") as f:
        text = f.read()
    collecting = [l.strip() for l in text.split("\n") if "Collecting" in l]
    doc = {
        "format": "four-way-vs-operator/opx2x-comparison/1",
        "pins": PINS,
        "indexNotes": index_notes,
        "collectingLines": collecting,
        "qualifier": QUALIFIER,
    }
    report, why = parse_report(text)
    if report is None:
        doc.update({"reportParsed": False, "reason": why, "compileDiagnostic": compile_diagnostic(text),
                    "classification": None})
        with open(out_json, "x", encoding="utf-8") as f:
            json.dump(doc, f, indent=1)
            f.write("\n")
        sys.exit(3)

    schema = []
    actual = []
    mods = report.get("modules")
    if not isinstance(mods, list):
        schema.append("modules is not a list")
        mods = []
    for m in mods:
        tests = m.get("tests") if isinstance(m, dict) else None
        if not isinstance(tests, list):
            schema.append("module without a tests list: %r" % (m.get("name") if isinstance(m, dict) else m))
            continue
        for t in tests:
            if not isinstance(t, dict):
                schema.append("test entry is not an object in %r" % m.get("name"))
                continue
            eu = t.get("execution_units")
            row = (m.get("name"), t.get("title"), t.get("status"), t.get("on_failure"),
                   eu.get("mem") if isinstance(eu, dict) else None, eu.get("cpu") if isinstance(eu, dict) else None)
            if not all(isinstance(x, str) for x in row[:4]) or not all(isinstance(x, int) for x in row[4:]):
                schema.append("test row with missing or mistyped fields: %r" % (row[:2],))
            actual.append(row)
    summ = report.get("summary")
    if not isinstance(summ, dict):
        schema.append("summary is not an object")
        summ = {}
    kind = summ.get("kind") if isinstance(summ.get("kind"), dict) else {}
    counts = [summ.get(k) for k in ("total", "passed", "failed")]
    if not all(isinstance(x, int) for x in counts) or counts[0] != len(actual) \
            or counts[1] + counts[2] != counts[0] or kind.get("property") != 0:
        schema.append("summary inconsistent with rows or property tests present: %r" % (summ,))

    with open(out_tsv, "x", encoding="utf-8") as f:
        for a in actual:
            f.write("\t".join(str(x) for x in a) + "\n")
        f.write("#summary\t%s\t#seed\t%s\n" % (json.dumps(summ), report.get("seed")))

    exp_keys = collections.Counter((r[0], r[1]) for r in rows)
    act_keys = collections.Counter((a[0], a[1]) for a in actual)
    fail_of = {(r[0], r[1]): r[2] for r in rows}
    membership = {
        "missing": [list(k) for k in sorted((exp_keys - act_keys).elements())],
        "unexpected": [list(k) for k in sorted((act_keys - exp_keys).elements(), key=str)],
        "duplicates": [list(k) for k, n in act_keys.items() if n > 1],
        "otherModules": sorted({str(a[0]) for a in actual} - set(MODULES)),
        "actualCount": len(actual),
    }
    match_anomaly = bool(membership["missing"] or membership["unexpected"] or membership["duplicates"]
                         or membership["otherModules"] or len(actual) != 147)
    outcomes = {
        "statusNotPass": [list(a[:3]) for a in actual if a[2] != "pass"],
        "onFailureMismatch": [[a[0], a[1], a[3], ON_FAILURE_FAIL if fail_of[(a[0], a[1])] else ON_FAILURE_PLAIN]
                              for a in actual if (a[0], a[1]) in fail_of
                              and a[3] != (ON_FAILURE_FAIL if fail_of[(a[0], a[1])] else ON_FAILURE_PLAIN)],
    }
    if schema:
        cls = "UNCLASSIFIED(report-schema)"
    elif match_anomaly:
        cls = "UNCLASSIFIED(match-filter)"
    elif outcomes["statusNotPass"] or outcomes["onFailureMismatch"]:
        cls = "FAILED(prediction)"
    else:
        cls = "PREDICTIONS_HOLD"
    per = collections.OrderedDict()
    for a in actual:
        c = per.setdefault(str(a[0]), [0, 0])
        c[0] += 1
        c[1] += int(a[3] == ON_FAILURE_FAIL)
    units = {a[1]: (a[4], a[5], a[2]) for a in actual if a[0] == "pvc2/shard4_cost_test"}
    doc.update({
        "reportParsed": True,
        "seed": report.get("seed"),
        "summary": summ,
        "schemaProblems": schema,
        "expectedTests": len(rows),
        "perModuleActual": per,
        "membership": membership,
        "matchFilterAnomaly": match_anomaly,
        "outcomes": outcomes,
        "exact": cls == "PREDICTIONS_HOLD",
        "classification": cls,
        "neverRefit": "A deviation is preserved as observed; EXPECTATIONS.json is never edited.",
        "directional": pc4(units, parent),
    })
    with open(out_json, "x", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
        f.write("\n")
    print("OPX2x: %d tests parsed, classification %s" % (len(actual), cls))
    sys.exit(0)


if __name__ == "__main__":
    main()
