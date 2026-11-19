#!/usr/bin/env python3
"""Replay competitive-landscape checks and emit public JSON receipts."""
import argparse
import hashlib
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LANG = Path("experiments/moriarty-language")
LEDGER = Path("experiments/moriarty-midnight-financial/ledger")
FEE_TEST = LANG / "tests/authority-fee-binding.test.mjs"
MUTATION_TEST = "exact quantum conversion rejects nondivisibility and overflow"
INTENT = LANG / "src/successor/financial-outcome-intent.ts"
REPORT = Path("deliverables/competitor-report")
TIMEOUT = 240
SCOPES = {
    "source": LANG / "src/frontend.ts",
    "kernel": LANG / "src/lower-compact.ts",
    "outcomeIntent": INTENT,
    "ledgerReadTransport": LEDGER / "financial-rpc.mjs",
}
COVERAGE_SUITES = [LANG / "tests/frontend.test.mjs", LANG / "tests/lowering.test.mjs",
                   FEE_TEST, LEDGER / "preview-rpc.test.mjs"]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def test_inventory(xml, expected):
    """Require actual passing test cases from every expected test module."""
    tree = ET.fromstring(xml)
    cases = tree.findall(".//testcase")
    if not cases or any(case.find(tag) is not None for case in cases
                        for tag in ("failure", "error", "skipped")):
        raise ValueError("missing, failed or skipped test case")
    files = {Path(case.attrib["file"]).resolve() for case in cases}
    missing = {path.resolve() for path in expected} - files
    if missing:
        raise ValueError("omitted test modules: " + ", ".join(map(str, sorted(missing))))
    return {case.attrib["name"] for case in cases}


def coverage(lcov, required):
    """Count distinct executed and instrumented lines for each declared file."""
    rows = {}
    for record in lcov.split("end_of_record"):
        fields = record.splitlines()
        names = [Path(line[3:]) for line in fields if line.startswith("SF:")]
        if len(names) != 1 or names[0] not in required:
            continue
        name = str(names[0])
        lines = {}
        for line in fields:
            if line.startswith("DA:"):
                number, hits = map(int, line[3:].split(",")[:2])
                if number in lines or number <= 0 or hits < 0:
                    raise ValueError(f"invalid coverage line in {name}")
                lines[number] = hits
        if name in rows or not any(lines.values()):
            raise ValueError(f"incomplete or duplicate coverage for {name}")
        rows[name] = {"covered": sum(hits > 0 for hits in lines.values()), "total": len(lines)}
    if set(rows) != {str(path) for path in required}:
        raise ValueError("missing scoped coverage files")
    return rows


def mutation_outcome(exit_code, xml):
    if exit_code != 1 or "<testsuites" not in xml:
        raise ValueError("mutation needs an executed assertion failure")
    failed = [case for case in ET.fromstring(xml).iter("testcase") if case.find("failure") is not None]
    if len(failed) != 1 or failed[0].get("name") != MUTATION_TEST:
        raise ValueError("unexpected failing check for the controlled mutation")
    failure = failed[0].find("failure")
    if "ERR_ASSERTION" not in ET.tostring(failure, encoding="unicode"):
        raise ValueError("mutation did not reach the amount assertion")
    return [MUTATION_TEST]


def probe_mutation(scratch):
    fixture = scratch / "language"
    shutil.copytree(ROOT / LANG / "src", fixture / "src")
    (fixture / "spec").symlink_to(ROOT / LANG / "spec", target_is_directory=True)
    shutil.copy2(ROOT / LANG / "package.json", fixture / "package.json")
    (fixture / "tests").mkdir()
    shutil.copy2(ROOT / FEE_TEST, fixture / "tests" / FEE_TEST.name)
    target = fixture / INTENT.relative_to(LANG)
    before = target.read_bytes()
    old = b'(product/BigInt(denominator)).toString()'
    new = b'(product/BigInt(denominator) + 1n).toString()'
    if before.count(old) != 1:
        raise ValueError("amount-conversion mutation site changed")
    after = before.replace(old, new)
    target.write_bytes(after)
    cmd = ["node", "--test", "--test-reporter=junit", "tests/" + FEE_TEST.name]
    proc = subprocess.run(cmd, cwd=fixture, text=True, capture_output=True, timeout=TIMEOUT)
    failed = mutation_outcome(proc.returncode, proc.stdout)
    return {"command": cmd, "file": str(INTENT), "beforeSha256": sha(before), "afterSha256": sha(after),
            "replace": old.decode(), "with": new.decode(), "exit": proc.returncode, "failedTests": failed}


def execute(name, cmd, checks, **kwargs):
    proc = subprocess.run(cmd, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                          timeout=TIMEOUT, **kwargs)
    checks[name] = {"command": list(map(str, cmd)), "exit": proc.returncode, "stdoutSha256": sha(proc.stdout),
                    "stderrSha256": sha(proc.stderr)}
    if proc.returncode:
        raise ValueError(f"{name} exited {proc.returncode}\n{(proc.stdout + proc.stderr).decode(errors='replace')[-2400:]}")
    return proc.stdout


def baseline(scratch, result):
    checks = result["checks"]
    npm = ["npm", "--prefix", str(LANG), "--silent"]
    reference = json.loads((ROOT / REPORT / "I.1/result.json").read_text())
    manifest = json.loads((ROOT / LANG / "package.json").read_bytes())
    if manifest["scripts"]["build"] != "tsc -p tsconfig.json":
        raise ValueError("expected the full TypeScript project build")
    for name in ("demo", "loan-lifecycle-demo", "build"):
        # Use the documented invocation, including npm's command header, for exact demo effects.
        raw = execute(name, npm[:-1] + ["run", name], checks)
        if name != "build":
            expected = next(x["output"]["sha256"] for x in reference["deliveryReplay"]["commands"]
                            if x["command"][-1] == name)
            if sha(raw) != expected:
                raise ValueError(f"{name} output differs from the clean I.1 delivery")
    junit_env = dict(os.environ, NODE_OPTIONS="--test-reporter=junit")
    xml = execute("languageSuite", npm + ["test"], checks, env=junit_env)
    suites = sorted((ROOT / LANG / "tests").glob("*.test.mjs"))
    test_inventory(xml, set(suites))
    checks["languageSuite"]["files"] = [str(p.relative_to(ROOT)) for p in suites]
    checks["languageSuite"]["testCases"] = len(ET.fromstring(xml).findall(".//testcase"))
    if checks["languageSuite"]["testCases"] < reference["deliveryReplay"]["summary"]["tests"]:
        raise ValueError("language suite executed fewer tests than the I.1 baseline")
    cov = scratch / "lines.lcov"
    cmd = ["node", "--test", "--experimental-test-coverage", "--test-reporter=junit",
           "--test-reporter-destination=stdout", "--test-reporter=lcov",
           "--test-reporter-destination=" + str(cov)]
    cmd += ["--test-coverage-include=" + str(p) for p in SCOPES.values()]
    xml = execute("scopedTests", cmd + list(map(str, COVERAGE_SUITES)), checks)
    checks["scopedTests"]["command"][6] = "--test-reporter-destination=<scratch>/lines.lcov"
    test_inventory(xml, {ROOT / path for path in COVERAGE_SUITES})
    result["coverage"] = {"metric":"Node V8 executed lines / instrumented lines",
                          "scope":{k:str(v) for k,v in SCOPES.items()},
                          "files":coverage(cov.read_text(), set(SCOPES.values())),
                          "exclusions":"Other source files, generated Compact/ZKIR and native ledger execution. The ledger suite exercises block-anchored RPC reads with a fixture transport."}
    result["controlledRegression"] = probe_mutation(scratch)
    # The baseline remains runnable on its own; each installed native predicate
    # supplies coverage from its executable cases at the exact receipt identity.
    evidence = {}
    for case, (_, _, checker, _, _) in NATIVE.items():
        filename = "factory-check.json" if checker in ("check-feed","check-o8") else "result.json"
        path = ROOT / REPORT / case / filename
        if path.is_file():
            record = json.loads(path.read_bytes())
            if record.get("passed") is True:
                evidence[case] = {"receipt":str(path.relative_to(ROOT)), "sha256":sha(path.read_bytes()),
                                  "tests":record["tests"], "scopedCoverage":record.get("coverage")}
    result["orderBehaviorCoverage"] = evidence


def check_invariants():
    receipt = json.loads((ROOT / REPORT / "II.6/result.json").read_text())
    for name, digest in receipt["fileSha256"].items():
        if sha((ROOT / name).read_bytes()) != digest:
            raise ValueError(f"II.6 input digest changed: {name}")
    version = (ROOT / "clean/fork/lean-toolchain").read_text().strip().split(":v")[-1]
    if subprocess.check_output(["lean", "--short-version"], text=True).strip() != version:
        raise ValueError("II.6 requires the repository's pinned Lean version")
    suite = Path("packages/moriarty-orders/test/invariants.test.mjs")
    output = execute("finiteTests", ["node", "--test", "--test-reporter=junit", str(suite)], {})
    test_inventory(output, {ROOT / suite})
    if len(ET.fromstring(output).findall(".//testcase")) != receipt["finiteTests"]["passed"]:
        raise ValueError("invariant boundary cases changed")
    analysis = receipt["formalAnalysis"]
    model = "clean/fork/Clean/Sherlock/OrderInvariants.lean"

    def audit(output):
        found = {}
        for theorem in analysis["axiomsByTheorem"]:
            name = analysis["namespace"] + "." + theorem
            match = re.search("'" + re.escape(name) + r"' (?:depends on axioms: \[([^\]]*)\]|does not depend on any axioms)", output)
            if not match:
                raise ValueError("missing axiom audit for " + name)
            found[theorem] = set(match[1].split(", ")) if match[1] else set()
        return found

    lean = execute("lean", ["lean", "-DwarningAsError=true", model], {}).decode()
    for theorem, axioms in audit(lean).items():
        if axioms != set(analysis["axiomsByTheorem"][theorem]) or "sorryAx" in axioms:
            raise ValueError("unexpected axiom audit for " + theorem)
    print(lean, end="")
    # The negative control is the model with one isolated mutation and must fail in its target only.
    control = analysis["negativeControl"]
    mutation = control["isolatedMutation"]
    command = ["lean", "-DwarningAsError=true", control["path"]]
    source = (ROOT / model).read_text()
    mutant = (ROOT / control["path"]).read_text()
    if (control["command"] != command or source.count(mutation["replace"]) != 1
            or mutant != source.replace(mutation["replace"], mutation["with"])):
        raise ValueError("II.6 negative control is not the model with the recorded mutation")
    proc = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=TIMEOUT)
    output = proc.stdout + proc.stderr
    lines = mutant.splitlines()
    start = next(i for i, line in enumerate(lines) if line.startswith("theorem " + control["target"] + " "))
    end = next((i for i in range(start + 1, len(lines)) if lines[i].startswith("theorem ")), len(lines))
    errors = re.findall("^" + re.escape(control["path"]) + r":(\d+):\d+: error: (.*)$", output, flags=re.MULTILINE)
    found = audit(output)
    if (proc.returncode != control["exit"] or proc.returncode == 0 or len(errors) != 1
            or not start < int(errors[0][0]) <= end or not errors[0][1].startswith(control["diagnostic"])
            or "sorryAx" not in found.pop(control["target"])
            or any(axioms != set(analysis["axiomsByTheorem"][name]) for name, axioms in found.items())):
        raise ValueError("II.6 negative control did not fail only in " + control["target"] + "\n" + output[-2400:])
    print(f"negative control {control['path']}: exit {proc.returncode} in {control['target']}: {errors[0][1]}")


ORDERS = Path("packages/moriarty-orders")
# Each row pairs a finite suite with an independent strict native replay.
NATIVE = {
    "II.1": ("F27", "acceptance", "check-acceptance", [], "src/authorization.mjs"),
    "II.2": ("U036", "o1", "check-acceptance", [], "native/o1-accounting.mjs"),
    "II.3": ("F30", "recovery", "check-recovery", [], "src/recovery.mjs"),
    "II.5": ("F28", "feed", "check-feed", ["II.5"], "src/feed.mjs"),
    "II.4": ("F32", "continuation", "check-staged", ["II.4"], "src/continuation.mjs"),
    "III.5": ("U040", "executors", "check-executors", [], "src/executors.mjs"),
    "IV.3": ("U037", "o8", "check-o8", [], "src/atomic-exchange.mjs"),
    "IV.4": ("U039", "o9", "check-staged", ["IV.4"], None),
    "IV.2": ("U038", "o5", "check-feed", ["IV.2"], "src/o5.mjs"),
}


def write_record(dest, record):
    dest.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", dir=dest.parent, delete=False) as file:
        json.dump(record, file, indent=2)
        file.write("\n")
    os.replace(file.name, dest)


def native_case(case):
    predicate, suite, checker, extra, scope = NATIVE[case]
    test = ORDERS / "test" / (suite + ".test.mjs")
    check = ORDERS / "native" / (checker + ".mjs")
    evidence = ROOT / REPORT / case
    # Feed generation retains its own primary result; replay adds a second record.
    dest = evidence / ("factory-check.json" if checker in ("check-feed", "check-o8") else "result.json")
    result = {"requirement": case, "predicate": predicate, "passed": False, "checks": {},
              "runner": ["python3", "scripts/qa/order-report.py", case], "assessment": "independent review pending",
              "target": "local undeployed Midnight ledger, full Rust validation and ZKIRv3",
              "limitations": ["Controlled local block clock and public test wallets", "Public-network inclusion and finality unassessed"]}
    try:
        result["revision"] = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
        result["environment"] = {"node": subprocess.check_output(["node","--version"], text=True).strip(),
                                  "platform": platform.platform(), "machine": platform.machine(),
                                  "sdkLockSha256": sha((ROOT / "tools/sherlock-sdk/package-lock.json").read_bytes())}
        with tempfile.TemporaryDirectory(prefix="order-coverage-") as tmp:
            cmd = ["node", "--test", "--test-reporter=junit", "--test-reporter-destination=stdout"]
            if scope:
                cov = Path(tmp) / "lines.lcov"
                cmd.extend(["--experimental-test-coverage", "--test-reporter=lcov", "--test-reporter-destination=" + str(cov),
                            "--test-coverage-include=" + str(ORDERS / scope)])
            xml = execute("finiteTests", cmd + [str(test)], result["checks"])
            result["tests"] = sorted(test_inventory(xml, {ROOT / test}))
            if scope:
                result["coverage"] = {"metric": "Node V8 executed lines / instrumented lines",
                                      "files": coverage(cov.read_text(), {ORDERS / scope})}
                result["checks"]["finiteTests"]["command"][6] = "--test-reporter-destination=<scratch>/lines.lcov"
        raw = execute("strictNativeReplay", ["node", str(check), *extra], result["checks"])
        # Compact may print its compilation progress before the JSON replay.
        output = raw.decode()
        start = re.search(r"^\{", output, flags=re.MULTILINE)
        if not start:
            raise ValueError("missing native replay details")
        result["replay"] = json.loads(output[start.start():])
        result["inputs"] = {str(path): sha((ROOT / path).read_bytes()) for path in [test, check, ORDERS / "native/replay.mjs"]}
        result["passed"] = True
    except (OSError, ValueError, subprocess.SubprocessError, ET.ParseError) as error:
        result["error"] = str(error)
        print(error, file=sys.stderr)
    write_record(dest, result)
    return 0 if result["passed"] else 1


def distribution():
    suite = ORDERS / "test/distribution.test.mjs"
    base = ROOT / REPORT / "III.6"
    cmd = ["node","--test","--test-reporter=junit",str(suite)]
    checks = {}
    with tempfile.TemporaryDirectory(prefix="order-install-") as directory:
        receipt = Path(directory) / "install.json"
        env = dict(os.environ, MORIARTY_ORDER_INSTALL_RECEIPT=str(receipt))
        names = test_inventory(execute("cleanInstall", cmd, checks, env=env), {ROOT / suite})
        install = json.loads(receipt.read_bytes())
    release = json.loads((base/"release.json").read_bytes())
    archive = base / release["file"]
    if install["sha256"] != sha(archive.read_bytes()) or install["sha256"] != release["sha256"]:
        raise ValueError("installed archive digest differs from the release record")
    write_record(base/"clean-install.json", install)
    write_record(base/"result.json", {"requirement":"III.6","predicate":"F33","localReplayPassed":True,
                 "status":"partial","checks":checks,"tests":sorted(names),"package":release["package"],
                 "version":release["version"],"archiveSha256":release["sha256"],"platform":release["platform"],
                 "installReceipt":"clean-install.json","releaseRecord":"release.json",
                 "publication":release["publication"],"outsideReproduction":"deferred"})
    return 0


def decision():
    directory = ROOT / REPORT / "IV.6"
    path = directory / "decision.md"
    text = path.read_text()
    for link in re.findall(r"\]\(([^)]+)\)", text):
        if not (directory / link).resolve().is_file():
            raise ValueError("missing O10 decision evidence " + link)
    cmd = ["node", "--test", "--test-reporter=junit", "--test-name-pattern", "bridge authoring",
           "packages/moriarty-beta/tests/examples.test.mjs"]
    checks = {}
    names = test_inventory(execute("bridgeScope", cmd, checks), {ROOT / cmd[-1]})
    if len(names) != 1 or "Defer the two-chain order" not in text:
        raise ValueError("decision and bridge execution check do not agree")
    write_record(directory / "result.json", {"requirement":"IV.6", "decision":"defer", "passed":True,
                 "decisionSha256":sha(path.read_bytes()), "checks":checks, "revision":subprocess.check_output(
                 ["git","rev-parse","HEAD"],cwd=ROOT,text=True).strip(),
                 "O10":{"coverage":"open","settlementCost":"unavailable","recoveryCost":"unavailable"}})
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("case", choices=["I.2", "II.6", "III.1", "III.6", "IV.6", "reconcile", *NATIVE])
    args = parser.parse_args()
    if args.case == "III.6":
        return distribution()
    if args.case == "reconcile":
        from order_evidence import reconcile
        return reconcile()
    if args.case == "III.1":
        from order_cost_report import check_costs
        suite = ORDERS / "test/measurement.test.mjs"
        test_inventory(execute("measurements", ["node","--test","--test-reporter=junit",str(suite)], {}), {ROOT / suite})
        check_costs()
        return 0
    if args.case in NATIVE:
        return native_case(args.case)
    if args.case == "IV.6":
        return decision()
    if args.case == "II.6":
        check_invariants()
        return 0
    dest = ROOT / REPORT / args.case
    dest.mkdir(parents=True, exist_ok=True)
    checks = {}
    result = {"requirement":args.case, "predicate":"F26", "checks":checks, "passed":False,
              "runner":["python3", "scripts/qa/order-report.py", args.case],
              "testCommand":["python3","-m","unittest","discover","-s","scripts/qa/tests","-p","test_order_report.py"],
              "environment":{"system":platform.system(),"machine":platform.machine(),"python":platform.python_version()}}
    try:
        result["revision"] = subprocess.check_output(["git","rev-parse","HEAD"], cwd=ROOT, text=True).strip()
        for tool,cmd in {"node":["node","--version"],"npm":["npm","--version"],
                         "compact":["compact","compile","--version"],
                         "typescript":[str(ROOT / LANG / "node_modules/.bin/tsc"),"--version"]}.items():
            result["environment"][tool] = subprocess.check_output(cmd, text=True, timeout=30).strip()
        result["implementationSha256"] = sha(Path(__file__).read_bytes())
        result["dependencyLockSha256"] = sha((ROOT / LANG / "package-lock.json").read_bytes())
        with tempfile.TemporaryDirectory(prefix="order-report-") as directory:
            baseline(Path(directory), result)
        result["passed"] = True
    except (OSError, ValueError, subprocess.SubprocessError, ET.ParseError) as error:
        result["error"] = str(error)
        print(error, file=sys.stderr)
    with tempfile.NamedTemporaryFile(mode="w", dir=dest, delete=False) as file:
        json.dump(result, file, indent=2)
        file.write("\n")
    os.replace(file.name, dest / "result.json")
    print(f"{args.case} {'PASS' if result['passed'] else 'FAIL'}: {dest.relative_to(ROOT)}/result.json")
    return 0 if result["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
