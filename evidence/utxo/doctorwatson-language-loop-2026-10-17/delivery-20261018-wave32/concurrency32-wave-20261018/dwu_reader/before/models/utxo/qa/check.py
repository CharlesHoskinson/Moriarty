#!/usr/bin/env python3
"""Read public UTXO metadata and report declarations and byte currentness.

No declared command is executed. File agreement does not authenticate evidence,
reviewers, results, product semantics, cryptography or target acceptance.
"""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import sys


BASELINE = "docs/utxo/doctorwatson/EARS-BASELINE.json"
EVIDENCE = "openspec/changes/utxo-doctorwatson/current-evidence.json"
LIMIT = "Declarations and file currentness only; no authentication or semantic acceptance."


class Malformed(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise Malformed(message)


def obj(value, fields, where):
    require(type(value) is dict, f"{where}: expected object")
    for field in fields:
        require(field in value, f"{where}: missing {field}")
    return value


def string(value, where):
    require(type(value) is str and bool(value.strip()), f"{where}: expected nonempty string")
    return value


def array(value, where):
    require(type(value) is list, f"{where}: expected array")
    return value


def unique_members(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON member: {key}")
        result[key] = value
    return result


class Files:
    """Read only explicit repository-relative regular files, never follow links."""

    def __init__(self, root):
        self.root = root.resolve(strict=True)
        require(self.root.is_dir(), "repository must be a directory")

    def read(self, name):
        string(name, "path")
        path = PurePosixPath(name)
        require(not path.is_absolute() and "\\" not in name and
                not any(part in ("", ".", "..") for part in name.split("/")) and
                not re.match(r"^[A-Za-z]:", name) and "\x00" not in name,
                f"path must be explicit repository-relative: {name!r}")
        reserved = {"private", "wallet", "wallets", "secrets", "credentials", "keystore", "keystores"}
        require(not any(part.startswith(".") or part.lower() in reserved for part in path.parts)
                and path.suffix.lower() not in {".pem", ".key", ".keystore"},
                f"path must name public metadata: {name!r}")
        # O_NOFOLLOW applies to every path component, including ancestor directories.
        descriptor = os.open(self.root, os.O_RDONLY | os.O_DIRECTORY)
        try:
            for part in path.parts[:-1]:
                following = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                                    dir_fd=descriptor)
                os.close(descriptor)
                descriptor = following
            final = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
                            dir_fd=descriptor)
            with os.fdopen(final, "rb") as stream:
                require(stat.S_ISREG(os.fstat(stream.fileno()).st_mode),
                        f"path must name a regular file: {name}")
                return stream.read()
        finally:
            os.close(descriptor)

    def document(self, name):
        try:
            return json.loads(self.read(name), object_pairs_hook=unique_members,
                              parse_constant=lambda x: require(False, f"invalid JSON number: {x}"))
        except (UnicodeError, json.JSONDecodeError) as exc:
            raise Malformed(f"{name}: malformed JSON: {exc}") from exc

    def pin(self, value, where, issues):
        obj(value, ("path", "sha256"), where)
        string(value["path"], f"{where}.path")
        require(type(value["sha256"]) is str and
                re.fullmatch(r"[0-9a-f]{64}", value["sha256"]) is not None,
                f"{where}.sha256: expected full lowercase SHA-256")
        try:
            raw = self.read(value["path"])
        except FileNotFoundError:
            issues.append(f"{where}: missing file {value['path']}")
            return None
        except OSError as exc:
            raise Malformed(f"{where}: unsafe or unreadable path {value['path']}: {exc.strerror}") from exc
        if hashlib.sha256(raw).hexdigest() != value["sha256"]:
            issues.append(f"{where}: stale pin {value['path']}")
        return raw


def headings(raw):
    """Retain complete requirement/scenario names and their source order."""
    result = []
    current = None
    fenced = False
    for line in raw.decode("utf-8").splitlines():
        if re.match(r"^\s*(```|~~~)", line):
            fenced = not fenced
            continue
        if fenced:
            continue
        match = re.match(r"^#{1,6}\s+Requirement:\s*(.*?)\s*#*\s*$", line)
        if match:
            current = {"heading": match.group(1), "scenarios": []}
            result.append(current)
        else:
            match = re.match(r"^#{1,6}\s+Scenario:\s*(.*?)\s*#*\s*$", line)
            if match and current is not None:
                current["scenarios"].append(match.group(1))
    return result


def duplicates(values):
    return [value for value, count in Counter(values).items() if count > 1]


def scope(value):
    return (value.get("profile"), value.get("target")) if type(value) is dict else (None, None)


def inspect(files, baseline, evidence):
    obj(baseline, ("format", "groups", "families"), "baseline")
    require(baseline["format"] == "doctorwatson-utxo-design-baseline/1", "baseline format mismatch")
    obj(evidence, ("format", "rows", "families"), "evidence")
    require(evidence["format"] == "doctorwatson-utxo.evidence/1", "evidence format mismatch")
    issues = []
    expected = {}
    sources = {}
    source_membership_issues = {}
    ids = []
    for group in array(baseline["groups"], "baseline.groups"):
        obj(group, ("group", "requirements"), "baseline group")
        string(group["group"], "baseline group name")
        for row in array(group["requirements"], "baseline requirements"):
            obj(row, ("id", "heading", "source", "scenarios"), "baseline requirement")
            string(row["heading"], "baseline heading")
            require(row["id"] is None or type(row["id"]) is str, "baseline id must be string or null")
            if row["id"] is not None:
                string(row["id"], "baseline id")
                ids.append(row["id"])
                if row["heading"].split()[0].rstrip(":") != row["id"]:
                    issues.append(f"baseline ID differs from normative heading: {row['heading']}")
            files.pin(row["source"], "baseline source", issues)
            key = row["source"]["path"] + "#" + row["heading"]
            if "key" in row and row["key"] != key:
                issues.append(f"baseline key mismatch: {key}")
            if key in expected:
                issues.append(f"duplicate baseline requirement: {key}")
            scenarios = []
            for scenario in array(row["scenarios"], f"{key}.scenarios"):
                obj(scenario, ("name",), "baseline scenario")
                scenarios.append(string(scenario["name"], "scenario name"))
            if duplicates(scenarios):
                issues.append(f"duplicate baseline scenario: {key}")
            expected[key] = (row, group["group"], scenarios)
            sources.setdefault(row["source"]["path"], []).append((row["heading"], scenarios))
    if duplicates(ids):
        issues.append("duplicate baseline IDs: " + ", ".join(duplicates(ids)))
    for path, declared in sources.items():
        try:
            actual = [(r["heading"], r["scenarios"]) for r in headings(files.read(path))]
        except FileNotFoundError:
            continue
        if declared != actual:
            message = f"normative heading/scenario membership or order mismatch: {path}"
            issues.append(message)
            source_membership_issues[path] = message

    rows = {}
    reported = []
    for row in array(evidence["rows"], "evidence.rows"):
        obj(row, ("key", "id", "group", "source", "heading", "scenarioNames", "applicability",
                  "implementation", "validation", "acceptance", "scenarioRecords", "sourcePins",
                  "dependsOn", "nextAction"), "evidence row")
        key = string(row["key"], "row.key")
        require(row["id"] is None or type(row["id"]) is str, "row id must be string or null")
        if key in rows:
            issues.append(f"duplicate evidence requirement: {key}")
        rows[key] = row
        local = []
        for field, names in (("applicability", ("status", "profile", "target", "reason")),
                             ("implementation", ("status", "detail")),
                             ("validation", ("status", "evidenceRefs")),
                             ("acceptance", ("status", "scope", "reason", "reviewRefs")),
                             ("nextAction", ("consumer", "expected", "owner", "blockedBy", "readiness"))):
            obj(row[field], names, f"{key}.{field}")
            for name in names:
                if name not in ("evidenceRefs", "reviewRefs", "blockedBy"):
                    string(row[field][name], f"{key}.{field}.{name}")
        for blocked in array(row["nextAction"]["blockedBy"], f"{key}.blockedBy"):
            string(blocked, "blockedBy item")
        for dependency in array(row["dependsOn"], f"{key}.dependsOn"):
            string(dependency, "dependency")
        if duplicates(row["dependsOn"]):
            local.append("duplicate dependencies")
        record_names = []
        refs = [(ref, "validation evidence") for ref in array(row["validation"]["evidenceRefs"], "validation refs")]
        reviews = array(row["acceptance"]["reviewRefs"], "review refs")
        refs += [(ref, "review") for ref in reviews]
        pins = array(row["sourcePins"], "source pins")
        refs += [(ref, "source pin") for ref in pins]
        files.pin(row["source"], f"{key}: source", local)
        for record in array(row["scenarioRecords"], f"{key}.scenarioRecords"):
            obj(record, ("name", "status", "expectation", "evidenceRefs"), "scenario record")
            for field in ("name", "status", "expectation"):
                string(record[field], f"scenario.{field}")
            record_names.append(record["name"])
            refs += [(ref, f"scenario {record['name']} evidence") for ref in array(record["evidenceRefs"], "scenario refs")]
        for ref, label in refs:
            files.pin(ref, f"{key}: {label}", local)
        scenario_names = array(row["scenarioNames"], "scenarioNames")
        for name in scenario_names:
            string(name, "scenarioNames item")
        if duplicates(scenario_names) or duplicates(record_names) or scenario_names != record_names:
            local.append("scenario record/name membership mismatch or duplicate")
        if key not in expected:
            local.append("unknown evidence requirement")
        else:
            base, group, names = expected[key]
            if base["source"]["path"] in source_membership_issues:
                local.append(source_membership_issues[base["source"]["path"]])
            for field, wanted in (("id", base["id"]), ("group", group), ("heading", base["heading"]),
                                  ("scenarioNames", names)):
                if row[field] != wanted:
                    local.append(f"baseline {field} mismatch")
            if any(row["source"].get(field) != base["source"].get(field) for field in ("path", "sha256")):
                local.append("baseline source mismatch")
            if not any(all(pin.get(field) == base["source"].get(field) for field in ("path", "sha256")) for pin in pins):
                local.append("missing normative source pin")
        if row["acceptance"]["status"] == "closed":
            obj(row["acceptance"], ("profile", "target"), f"{key}.acceptance")
            for field in ("profile", "target"):
                string(row["acceptance"][field], f"{key}.acceptance.{field}")
            base = expected.get(key, ({}, None, []))[0]
            original = scope(base.get("originalScope"))
            applicable = scope(row["applicability"])
            if row["applicability"]["status"] != "applicable" or None in original or applicable != original:
                local.append("closed declaration lacks exact original profile/target applicability")
            if scope(row["acceptance"]) != applicable:
                local.append("closed acceptance profile/target mismatch")
            if row["implementation"]["status"] != "complete" or row["validation"]["status"] != "observed":
                local.append("closed declaration lacks complete implementation/observed validation")
            if not row["validation"]["evidenceRefs"]:
                local.append("closed declaration lacks actual validation evidence")
            if not row["scenarioRecords"] or any(record["status"] != "observed" or not record["evidenceRefs"] for record in row["scenarioRecords"]):
                local.append("closed declaration lacks complete scenario observations")
            for ref, label in refs:
                if label != "source pin" and scope(ref) != applicable:
                    local.append(f"closed {label} profile/target mismatch")
            required_roles = {"independent-source", "actual-result"}
            roles = set()
            for ref in reviews:
                if (ref.get("independent") is True and type(ref.get("author")) is str and
                        type(ref.get("reviewer")) is str and ref["author"].strip() and
                        ref["reviewer"].strip() and ref["author"] != ref["reviewer"]):
                    if type(ref.get("role")) is str:
                        roles.add(ref["role"])
            if not required_roles <= roles:
                local.append("closed declaration lacks independent source and actual-result review declarations")
        original = expected.get(key, ({}, None, []))[0]
        reported.append({"key": key, "id": row["id"], "group": row["group"],
                         "clause": original.get("clause"),
                         "source": original.get("source", row["source"]),
                         "heading": original.get("heading", row["heading"]),
                         "scenarios": original.get("scenarios", []),
                         "scenarioNames": row["scenarioNames"],
                         "scenarioRecords": row["scenarioRecords"],
                         "applicability": row["applicability"], "nextAction": row["nextAction"],
                         "implementation": row["implementation"], "observations": row["validation"],
                         "acceptance": row["acceptance"], "currentness": "stale" if local else "current",
                         "gate": "open", "issues": local})

    for key in expected.keys() - rows.keys():
        issues.append(f"missing evidence requirement: {key}")
    by_key = {row["key"]: row for row in reported}
    for key, row in rows.items():
        for dependency in row["dependsOn"]:
            if dependency not in rows:
                by_key[key]["issues"].append(f"unknown dependency: {dependency}")
    visiting, visited = [], set()

    def visit(key):
        if key in visiting:
            for member in visiting[visiting.index(key):]:
                by_key[member]["issues"].append("cyclic dependency")
            return
        if key in visited:
            return
        visiting.append(key)
        for dependency in rows[key]["dependsOn"]:
            if dependency in rows:
                visit(dependency)
        visiting.pop()
        visited.add(key)

    for key in rows:
        visit(key)
    # Fixed point also propagates staleness through cycles and arbitrary input order.
    stale = {key for key, row in by_key.items() if row["issues"]}
    while True:
        added = {key for key, row in rows.items() if any(dep in stale for dep in row["dependsOn"])} - stale
        if not added:
            break
        stale.update(added)
    for key, report in by_key.items():
        if key in stale:
            report["currentness"] = "stale"
            if not report["issues"]:
                report["issues"].append("stale dependency or inventory")
        if key not in stale and rows[key]["acceptance"]["status"] == "closed":
            report["gate"] = "closed"
        issues.extend(f"{key}: {issue}" for issue in report["issues"])

    closed = {key for key, report in by_key.items() if report["gate"] == "closed"}
    while True:
        removed = {key for key in closed if any(dep not in closed for dep in rows[key]["dependsOn"])}
        if not removed:
            break
        closed.difference_update(removed)
    for key, report in by_key.items():
        if report["gate"] == "closed" and key not in closed:
            report["gate"] = "open"
            message = "closed declaration has an open dependency gate"
            report["issues"].append(message)
            issues.append(f"{key}: {message}")

    family_expected = {}
    for family in array(baseline["families"], "baseline.families"):
        obj(family, ("id", "name"), "baseline family")
        identifier = string(family["id"], "family id")
        string(family["name"], "family name")
        if identifier in family_expected:
            issues.append(f"duplicate baseline family: {identifier}")
        family_expected[identifier] = family["name"]
    if len(family_expected) != 20:
        issues.append("baseline must retain all twenty original families")
    family_ids = []
    families = array(evidence["families"], "evidence.families")
    for family in families:
        obj(family, ("id", "name", "source", "admission", "compiled", "native", "private"), "family")
        for field in ("id", "name", "source", "admission", "compiled", "native", "private"):
            string(family[field], f"family.{field}")
        family_ids.append(family["id"])
        if family_expected.get(family["id"]) != family["name"]:
            issues.append(f"family membership/name mismatch: {family['id']}")
    if duplicates(family_ids):
        issues.append("duplicate evidence family")
    if set(family_ids) != set(family_expected):
        issues.append("missing or unknown family membership")
    # Family inventory failures invalidate the whole report's integrity and gates.
    if issues:
        for report in reported:
            report["gate"] = "open"
    return {"format": "doctorwatson-utxo.report/1", "workstream": "doctorwatson<UTXO>",
            "limitations": LIMIT, "integrity": {"status": "failed" if issues else "ok", "issues": issues},
            "requirements": reported, "families": families,
            "nextActions": sorted(
                [dict(key=key, id=row["id"], **row["nextAction"])
                 for key, row in rows.items() if by_key[key]["gate"] != "closed"],
                key=lambda action: not action["readiness"].startswith("ready"))}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", default=str(Path(__file__).resolve().parents[3]))
    parser.add_argument("--baseline", default=BASELINE)
    parser.add_argument("--evidence", default=EVIDENCE)
    parser.add_argument("--json", action="store_true")
    select = parser.add_mutually_exclusive_group()
    select.add_argument("--next", action="store_true")
    select.add_argument("--requirement")
    select.add_argument("--gate")
    args = parser.parse_args(argv)
    try:
        files = Files(Path(args.repo))
        report = inspect(files, files.document(args.baseline), files.document(args.evidence))
        code = 0 if report["integrity"]["status"] == "ok" else 1
        selection = args.requirement or args.gate
        if selection is not None:
            report["requirements"] = [row for row in report["requirements"] if selection in (row["key"], row["id"])]
            if len(report["requirements"]) != 1:
                report["integrity"]["issues"].append(f"unknown or ambiguous requirement: {selection}")
                report["integrity"]["status"] = "failed"
                code = 1
            if args.gate and (len(report["requirements"]) != 1 or report["requirements"][0]["gate"] != "closed"):
                code = 1
        if args.json:
            print(json.dumps(report, indent=2, ensure_ascii=False))
        else:
            print(f"doctorwatson<UTXO>: integrity {report['integrity']['status']}; {len(report['requirements'])} requirements; {len(report['families'])} families")
            for issue in report["integrity"]["issues"]:
                print(f"  {issue}")
            if args.next:
                for action in report["nextActions"]:
                    print(f"  {action['id'] or action['key']}: {action['consumer']} — {action['expected']} ({action['readiness']})")
            elif selection:
                for row in report["requirements"]:
                    print(f"  {row['id'] or row['key']}: declared {row['acceptance']['status']}; {row['currentness']}; gate {row['gate']}")
                    print(f"  Source: {row['source']['path']}#{row['heading']}")
                    if row["clause"]:
                        print(row["clause"])
                    for scenario in row["scenarios"]:
                        print(f"  Scenario: {scenario['name']}")
                        if "text" in scenario:
                            print(scenario["text"])
                    for record in row["scenarioRecords"]:
                        print(f"  Observation {record['name']}: {record['status']}")
                        print(f"    Expected: {record['expectation']}")
                        if record.get("gap"):
                            print(f"    Gap: {record['gap']}")
                    application = row["applicability"]
                    print(f"  Applicability: {application['status']}; {application['profile']}; {application['target']}")
                    print(f"    {application['reason']}")
                    action = row["nextAction"]
                    print(f"  Next: {action['consumer']} — {action['expected']} ({action['owner']}; {action['readiness']})")
                    for blocker in action["blockedBy"]:
                        print(f"    Blocked by: {blocker}")
            print(LIMIT)
        return code
    except (Malformed, OSError, UnicodeError, RecursionError) as exc:
        report = {"format": "doctorwatson-utxo.report/1", "limitations": LIMIT,
                  "integrity": {"status": "failed", "issues": [f"malformed input: {exc}"]},
                  "requirements": [], "families": [], "nextActions": []}
        print(json.dumps(report, indent=2) if args.json else f"doctorwatson<UTXO>: malformed input: {exc}")
        return 2


if __name__ == "__main__":
    sys.exit(main())
