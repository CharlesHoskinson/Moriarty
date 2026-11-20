#!/usr/bin/env python3
"""Retroactive failing-first runs for sections delivered before their failing run was retained.

For each section, a scratch worktree at the clean HEAD removes the behaviour that the section's
own test command pins, runs that command, and requires assertion failures. The same command must
pass on the unmodified worktree. Each record is labelled retroactive: the run postdates delivery.
"""
import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import tarfile
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REPORT = Path("deliverables/competitor-report")
ORDERS = "packages/moriarty-orders"
UNITTEST = ["python3", "-m", "unittest", "discover", "-s", "scripts/qa/tests", "-p", "test_order_report.py", "-v"]


def node_test(name):
    return ["node", "--test", "--test-reporter=junit", f"{ORDERS}/test/{name}.test.mjs"]


# Each mutation removes the behaviour the section's test command must pin.
SECTIONS = {
    "I.2": {"test": UNITTEST, "file": "scripts/qa/order-report.py",
            "replace": 'if names != sorted(mutation["tests"]):', "with": "if False:",
            "removes": "refusal of a controlled mutation caught by the wrong test"},
    "II.1": {"test": node_test("acceptance"), "file": f"{ORDERS}/contracts/acceptance.compact",
             "replace": 'ecAdd(sig.announcement,ecMul(ownerKey,c as JubjubScalar))), "ORDER_SIGNATURE");',
             "with": 'ecAdd(sig.announcement,ecMul(ownerKey,c as JubjubScalar))) || true, "ORDER_SIGNATURE");',
             "removes": "owner signature verification in the compiled order"},
    "II.2": {"test": node_test("o1"), "file": f"{ORDERS}/native/o1-accounting.mjs",
             "replace": "  assert.deepEqual(changes,frozen[scenario].change,'complete '+scenario+' A/B changes');\n", "with": "",
             "removes": "reconciliation of every account's A/B change against the frozen O1 deltas"},
    "II.3": {"test": node_test("recovery"), "file": f"{ORDERS}/contracts/recovery.compact",
             "replace": 'assert(revision == expected, "ORDER_REVISION");', "with": "",
             "removes": "revision binding that makes the losing fill or cancellation stale"},
    "II.4": {"test": node_test("continuation"), "file": f"{ORDERS}/src/continuation.mjs",
             "replace": "  if (head.terminal !== 0n || head.current.f2 !== 1n) throw Error('Continuation has been consumed');\n",
             "with": "", "removes": "the client's refusal to act on a consumed continuation"},
    "II.5": {"test": node_test("feed"), "file": f"{ORDERS}/contracts/feed-policy.compact",
             "replace": 'assert(feedPrice < 3, "FEED_THRESHOLD");', "with": 'assert(feedPrice <= 3, "FEED_THRESHOLD");',
             "removes": "the strict price threshold"},
    "II.6": {"test": node_test("invariants"), "file": "experiments/moriarty-language/src/lower-compact.ts",
             "replace": "revision: checkedAdd(revision, 1)", "with": "revision: checkedAdd(revision, 2)",
             "removes": "the generated kernel's single revision step per transition"},
    "III.1": {"test": node_test("measurement"), "file": "scripts/qa/measure-order-lifecycle.mjs",
              "replace": "  assert.equal(uint(row.dustBefore)-uint(row.dustAfter),fee);\n", "with": "",
              "removes": "reconciliation of the paid fee with the same-time DUST debit"},
    "III.5": {"test": node_test("executors"), "file": f"{ORDERS}/contracts/executors.compact",
              "replace": "const payee = role ? fallbackAccount : terms.executor;", "with": "const payee = terms.executor;",
              "removes": "payment of A to the client that delivered B"},
    "IV.2": {"test": node_test("o5"), "file": f"{ORDERS}/contracts/o5.compact",
             "replace": "  requireFeedCondition(expectedFeed);\n", "with": "",
             "removes": "the execution-time feed condition in the O5 fill"},
    "IV.3": {"test": node_test("o8"), "file": f"{ORDERS}/contracts/atomic-exchange.compact",
             "replace": 'assert(jubjubSchnorrVerify<1>([message],signatures[i],signingKeys[i]), "O8_SIGNATURE");', "with": "",
             "removes": "verification of each owner's leg grant"},
    "IV.4": {"test": node_test("o9"), "file": f"{ORDERS}/contracts/staged-exchange.compact",
             "replace": 'assert(blockTimeGte((terms.epoch + 1200) as Uint<64>), "FIRST_OPEN");', "with": "",
             "removes": "the first-trade deadline guarding the owner exit"},
    "III.6": {"test": ["node", "--test", "--test-reporter=junit", f"{ORDERS}/test/distribution.test.mjs"],
              "archive": "@midnightntwrk/ledger-v9",
              "removes": "the bundled ledger dependency from the candidate archive"},
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def clean_revision():
    dirty = subprocess.check_output(["git", "status", "--porcelain", "--", ".", ":!deliverables/competitor-report",
                                     ":!qa/sherlock/status.json", ":!qa/sherlock/runs.jsonl"], cwd=ROOT, text=True).strip()
    if dirty:
        raise SystemExit("run from a clean committed tree:\n" + dirty)
    return subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()


def failures(cmd, output):
    """Return the failing test names; every failure must be an assertion."""
    if cmd[0] == "python3":
        names = re.findall(r"^FAIL: (\S+)", output, flags=re.MULTILINE)
        if re.search(r"^ERROR: ", output, flags=re.MULTILINE) or "AssertionError" not in output:
            raise ValueError("the unittest run failed without an assertion")
        return sorted(names)
    start = output.find("<?xml") if "<?xml" in output else output.find("<testsuites")
    cases = list(ET.fromstring(output[start:]).iter("testcase"))
    failed = [c for c in cases if c.find("failure") is not None or c.find("error") is not None]
    for case in failed:
        text = ET.tostring(case, encoding="unicode")
        if "ERR_ASSERTION" not in text and "AssertionError" not in text:
            raise ValueError(f"{case.get('name')} failed without an assertion")
    return sorted(c.get("name") for c in failed)


def run(cmd, cwd, docker=False):
    if docker:
        inner = " ".join(cmd)
        cmd = ["docker", "run", "--rm", "--network", "none", "-v", f"{cwd}:/repo", "-w", "/repo",
               "node:26.0.0-bookworm-slim", "sh", "-c", inner]
        cwd = ROOT
    proc = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=1800)
    return proc.returncode, proc.stdout + proc.stderr


def strip_dependency(tree, name):
    """Repack the candidate archive without one bundled dependency and point release.json at it."""
    base = tree / REPORT / "III.6"
    release = json.loads((base / "release.json").read_bytes())
    archive = base / release["file"]
    with tempfile.TemporaryDirectory() as scratch:
        with tarfile.open(archive) as tar:
            tar.extractall(scratch, filter="data")
        shutil.rmtree(Path(scratch) / "package/node_modules" / name)
        with tarfile.open(archive, "w:gz") as tar:
            tar.add(Path(scratch) / "package", arcname="package")
    release["sha256"] = sha(archive.read_bytes())
    (base / "release.json").write_text(json.dumps(release, indent=2) + "\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sections", nargs="*", default=list(SECTIONS))
    args = parser.parse_args()
    revision = clean_revision()
    scratch = Path(tempfile.mkdtemp(prefix="test-first-"))
    tree = scratch / "tree"
    subprocess.run(["git", "worktree", "add", "--detach", str(tree), revision], cwd=ROOT, check=True, capture_output=True)
    try:
        for link in ("experiments/moriarty-language/node_modules", "tools/sherlock-sdk/node_modules"):
            (tree / link).symlink_to(ROOT / link, target_is_directory=True)
        (tree / "target/debug/examples").mkdir(parents=True)
        (tree / "target/debug/examples/validate_call").symlink_to(ROOT / "target/debug/examples/validate_call")
        for section in args.sections:
            spec = SECTIONS[section]
            docker = "archive" in spec
            clean_exit, clean_output = run(spec["test"], tree, docker)
            if clean_exit != 0:
                raise SystemExit(f"{section}: the unmodified test command fails:\n{clean_output[-2000:]}")
            record = {"section": section, "retroactive": True, "revision": revision,
                      "note": "Run after delivery: the original change committed tests and implementation together.",
                      "testCommand": spec["test"], "removes": spec["removes"],
                      "environment": "linux arm64 container node:26.0.0-bookworm-slim" if docker else f"{os.uname().sysname} {os.uname().machine}"}
            if docker:
                strip_dependency(tree, spec["archive"])
                record["mutation"] = {"archiveWithout": spec["archive"]}
            else:
                path = tree / spec["file"]
                before = path.read_bytes()
                if before.count(spec["replace"].encode()) != 1:
                    raise SystemExit(f"{section}: mutation site changed in {spec['file']}")
                after = before.replace(spec["replace"].encode(), spec["with"].encode())
                path.write_bytes(after)
                record["mutation"] = {"file": spec["file"], "replace": spec["replace"], "with": spec["with"],
                                      "beforeSha256": sha(before), "afterSha256": sha(after)}
            exit_code, output = run(spec["test"], tree, docker)
            subprocess.run(["git", "checkout", "--", "."], cwd=tree, check=True)
            if exit_code == 0:
                raise SystemExit(f"{section}: the test command passed without the behaviour")
            names = failures(spec["test"], output) if not docker else []
            if not docker and not names:
                raise SystemExit(f"{section}: no failing test recorded")
            if docker and "ledger-v9" not in output and "ENOTCACHED" not in output:
                raise SystemExit(f"{section}: the distribution check failed for an unexpected reason")
            restored_exit, _ = run(spec["test"], tree, docker)
            record.update({"exit": exit_code, "failingTests": names, "outputSha256": sha(output.encode()),
                           "outputTail": output[-1200:], "cleanExit": clean_exit, "restoredExit": restored_exit})
            if restored_exit != 0:
                raise SystemExit(f"{section}: the restored tree does not pass")
            destination = ROOT / REPORT / section / "test-first-retroactive.json"
            destination.write_text(json.dumps(record, indent=2) + "\n")
            print(f"{section}: exit {exit_code}, failing {names or 'distribution check'}")
    finally:
        subprocess.run(["git", "worktree", "remove", "--force", str(tree)], cwd=ROOT, capture_output=True)
        shutil.rmtree(scratch, ignore_errors=True)


if __name__ == "__main__":
    main()
