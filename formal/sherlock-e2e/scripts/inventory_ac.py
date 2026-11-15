#!/usr/bin/env python3
"""Inventory of every EARS ID in Formal-Lean-AC scope, with its sentence and source.

Output: JSON list of {id, group, context, sentence, source} on stdout.
group: C | A | completion | reused
"""
import json, re, subprocess, sys, pathlib

WT = pathlib.Path("/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec")
MYC = pathlib.Path("/home/hoskinson/Projects/Moriarty-wt/mycroft-factory/mycroft/spec/specs")
E2E = WT / "openspec/changes/sherlock-e2e-prototype-20261115/specs"
COMP = WT / "openspec/changes/sherlock-completion-20261115/specs"

CTX_C = {"wrap": "wrap", "ledger-verifier": "ledger-verification", "ledger-contract": "settlement",
         "batched-settlement": "settlement", "governance": "governance",
         "preview-deployment": "network-operations", "wallet": "network-operations",
         "sdk-cli": "network-operations", "observability": "network-operations",
         "recovery": "network-operations", "helper-machines": "placement"}
CTX_A = {"compact-compiler": "compact-compilation", "rsl-statement": "sherlock-execution",
         "rsl-kits": "sherlock-execution", "universal-interpreter": "sherlock-execution",
         "compact-suite": "conformance", "user-flow": "call-flow"}
OWN = {"WRP", "LVR", "SLC", "BAT", "EGV", "PVW", "WAL", "SCL", "OBS", "FRC", "HLP",
       "ECC", "ERS", "EKT", "EUI", "ECS", "EUF"}
SLICE_B = {"LFP", "LTR", "UKS", "NAG", "GPH", "CLB", "ELA", "STM", "BXF", "GPV", "PDM", "ZKE", "SSG",
           "ST", "SC", "PF"}
REQ = re.compile(r"^### Requirement: ([A-Z]+(?:-[A-Z]+)?-\d{3}[a-z]?)\b.*$", re.M)
TOK = re.compile(r"\b([A-Z]{2,5}(?:-[A-Z]{2,5})?-\d{3}[a-z]?)\b")
RANGE = re.compile(r"\b([A-Z]{2,5}(?:-[A-Z]{2,5})?)-(\d{3}) (?:to|\.\.) (?:\1-)?(\d{3})\b")


def reqs(text, source):
    out = {}
    parts = REQ.split(text)
    # parts: [pre, id1, body1, id2, body2, ...]
    for i in range(1, len(parts), 2):
        rid, body = parts[i], parts[i + 1]
        sent = ""
        for line in body.splitlines():
            line = line.strip()
            if line.startswith(rid + ":"):
                sent = line[len(rid) + 1:].strip()
                break
            if "SHALL" in line and not sent:
                sent = line
        if not sent:
            for line in body.splitlines():
                if line.strip():
                    sent = line.strip(); break
        out[rid] = (sent, source)
    return out


def git_show(ref, path):
    return subprocess.run(["git", "-C", str(WT), "show", f"{ref}:{path}"], capture_output=True, text=True).stdout


def git_ls(ref):
    return subprocess.run(["git", "-C", str(WT), "ls-tree", "-r", "--name-only", ref, "openspec/changes"],
                          capture_output=True, text=True).stdout.split()


# definitions, lowest precedence first
defs = {}
for p in sorted((WT / "openspec/changes").glob("*/specs/*/spec.md")):
    ch = p.parts[-4]
    if ch in ("sherlock-completion-20261115", "sherlock-e2e-prototype-20261115"):
        continue
    rank = 2 if ch in ("sherlock-processor-final-20261111",) else 1
    for k, v in reqs(p.read_text(), f"{ch}/{p.parts[-2]}").items():
        if k not in defs or defs[k][2] <= rank:
            defs[k] = (v[0], v[1], rank)
# t22-governance supplies GOV/ERT/ELD/CRR only; t20-batched supplies the post-T20 LDG text and ANC-012
for ref in ("t22-governance", "t20-batched"):
    for path in git_ls(ref):
        if path.endswith("spec.md") and ((ref == "t22-governance" and "sherlock-governance/" in path) or (ref == "t20-batched" and ("modular-leaf-20261111/specs/sherlock-ledger-contract" in path or "specs/sherlock-anchor-chunks/" in path))):
            ch = path.split("/")[2]
            for k, v in reqs(git_show(ref, path), f"{ref}:{ch}/{path.split('/')[-2]}").items():
                if 'anchor-chunks' in path and k in defs and k != 'ANC-012':
                    continue
                defs[k] = (v[0], v[1], 3)
for p in sorted(MYC.glob("*/spec.md")):
    for k, v in reqs(p.read_text(), f"mycroft-factory:mycroft/spec/specs/{p.parts[-2]}").items():
        defs.setdefault(k, (v[0], v[1], 0))
comp = {}
for p in sorted(COMP.glob("*/spec.md")):
    for k, v in reqs(p.read_text(), f"completion/{p.parts[-2]}").items():
        comp[k] = v
        defs[k] = (v[0], v[1], 4)

rows = []
seen = set()
cited = {}
for group, ctxmap in (("C", CTX_C), ("A", CTX_A)):
    for cap, ctx in ctxmap.items():
        text = (E2E / f"sherlock-e2e-{cap}" / "spec.md").read_text()
        for k, (sent, src) in reqs(text, f"e2e/sherlock-e2e-{cap}").items():
            rows.append({"id": k, "group": group, "context": ctx, "sentence": sent, "source": src})
            seen.add(k)
        for t in TOK.findall(text):
            cited.setdefault(t, set()).add(ctx)
        for fam, a, b in RANGE.findall(text):
            for n in range(int(a), int(b) + 1):
                cited.setdefault(f"{fam}-{n:03d}", set()).add(ctx)
adopted = (WT / "research/sherlock-e2e-spec-20261115/bus/adopted-C.md").read_text()
for line in adopted.splitlines():
    if not line.startswith("| sherlock-e2e-"):
        continue
    cap = line.split("|")[1].strip().removeprefix("sherlock-e2e-")
    ctx = CTX_C[cap]
    cell = line.split("|")[2]
    for fam, a, b in RANGE.findall(cell):
        for n in range(int(a), int(b) + 1):
            cited.setdefault(f"{fam}-{n:03d}", set()).add(ctx)
    for t in TOK.findall(cell):
        cited.setdefault(t, set()).add(ctx)
for k, (sent, src) in comp.items():
    rows.append({"id": k, "group": "completion", "context": "completion", "sentence": sent, "source": src})
    seen.add(k)
missing = []
for k in sorted(cited):
    fam = k.rsplit("-", 1)[0]
    if k == "SHA-256" or k in seen or fam in OWN or fam in SLICE_B:
        continue
    if k not in defs:
        missing.append((k, sorted(cited[k])))
        continue
    sent, src, _ = defs[k]
    rows.append({"id": k, "group": "reused", "context": "+".join(sorted(cited[k])), "sentence": sent, "source": src})
    seen.add(k)
# one context per row: reused IDs go to the first context in ORDER that cites them (MID to
# ledger-verification); completion IDs follow Formal-Quint's placement table (bus/formal-findings.md)
ORDER = ["wrap", "ledger-verification", "settlement", "governance", "network-operations", "placement",
         "sherlock-execution", "compact-compilation", "conformance", "call-flow", "completion"]
def rng(a, b, f): return [f"{f}-{i:03d}" for i in range(a, b + 1)]
PLACE = {}
for i in rng(1, 8, "RSL") + ["LPC-018"]: PLACE[i] = "B:leaf-proving"
for i in rng(1, 3, "LPC"): PLACE[i] = "B:aggregation"
for i in rng(13, 19, "ANC") + rng(20, 22, "LW"): PLACE[i] = "B:bridge"
for i in ["RKY-001", "RKY-002", "RKY-005"]: PLACE[i] = "B:key-management"
PLACE["LPC-006"] = "B:scheduling"
for i in ["LPC-005", "LPC-007"]: PLACE[i] = "B:box-fleet"
for i in rng(12, 19, "ZK") + rng(14, 16, "SQG") + ["LPC-004", "LPC-016"]: PLACE[i] = "B:assurance"
for i in ["RKY-003", "RKY-004"]: PLACE[i] = "wrap"
for i in rng(1, 6, "STL"): PLACE[i] = "settlement"
PLACE["LPC-012"] = "governance"
for i in rng(8, 11, "LPC") + ["LPC-015", "LPC-020"]: PLACE[i] = "network-operations"
for i in ["CF-013", "CF-014", "LPC-013", "LPC-019"]: PLACE[i] = "compact-compilation"
for i in ["LPC-014", "LPC-017"]: PLACE[i] = "call-flow"
for r in rows:
    if r["group"] == "completion":
        r["assigned"] = PLACE[r["id"]]
    else:
        cs = r["context"].split("+")
        r["assigned"] = "ledger-verification" if r["id"].startswith("MID-") and "ledger-verification" in cs \
            else sorted(cs, key=ORDER.index)[0]
json.dump({"rows": rows, "undefined_citations": missing}, sys.stdout, indent=1)
