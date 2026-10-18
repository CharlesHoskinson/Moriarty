"""Recompute this resume candidate's identities and its delta against the parent.

Stdlib only; reads the resume candidate/, the parent a6e17efd tree and the pinned Core
closure record; writes records/CANDIDATE-IDENTITY.json and DIFFS/candidate/*.diff
(both created exclusively). Imports no candidate, parent or operator code.

compiler_build_digest is predicted as SHA-256 of the compact sorted-key JSON of
CompilerBuild {compiler_source_manifest_sha256, core_closure_sha256} (Core
intent::canonical as reproduced by the result reviewer); the parent's observed value
a6911ddd... is recomputed first as a method control. A prediction is arithmetic only.
"""
import datetime, difflib, hashlib, json, os, resource, sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
OWN = BASE + "/current-core-groth16-evidence-repair-resume-source-20261015"
CAND = OWN + "/candidate"
PARENT = BASE + "/current-core-groth16-compiler-review-repair-source-20261015"
PARENT_SM = "a6e17efd0a47194c70ac6e4467efce2d47f9254dc47a8e596f6add2b92aef97e"
PARENT_BM = "4e3ba2e2796b425598b9b2abc022c4d6782bf032044aa53e124af4946c543b47"
CORE = "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed"
OBSERVED_PARENT_DIGEST = "a6911ddd6b3883dedfa4c155e4924d98fc645d41d72806d4d6bcd0c398366d44"
SIDECAR = "SOURCE-MANIFEST.json.sha256"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def rd(p):
    return open(p, "rb").read()


def compact(o):
    return json.dumps(o, sort_keys=True, separators=(",", ":")).encode()


def files_under(root):
    out = []
    for dp, dns, fns in os.walk(root):
        for fn in fns:
            p = os.path.join(dp, fn)
            out.append(os.path.relpath(p, root).replace(os.sep, "/"))
    return sorted(out)


def main():
    rec = {"format": "evidence-repair-resume-candidate-identity/1",
           "at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "stops": []}
    # BUILD-MANIFEST: entries equal bytes; entry set = Cargo.toml + src/**.
    bm_b = rd(CAND + "/BUILD-MANIFEST.json")
    bm = json.loads(bm_b)
    listed = []
    for e in bm["files"]:
        b = rd(CAND + "/" + e["path"])
        if sha(b) != e["sha256"] or len(b) != e["bytes"]:
            rec["stops"].append("BUILD entry differs: " + e["path"])
        listed.append(e["path"])
    rule = sorted(["source/mori-r1cs/Cargo.toml"] + ["source/mori-r1cs/src/" + p for p in files_under(CAND + "/source/mori-r1cs/src")])
    if sorted(listed) != rule:
        rec["stops"].append("BUILD entry set differs from Cargo.toml + src/**")
    parent_bm = json.loads(rd(PARENT + "/BUILD-MANIFEST.json"))
    rec["build_manifest"] = {"sha256": sha(bm_b), "bytes": len(bm_b), "entries": len(listed), "format": bm["format"],
                             "keys_equal_parent": sorted(bm) == sorted(parent_bm), "format_equal_parent": bm["format"] == parent_bm["format"] and bm["rule"] == parent_bm["rule"],
                             "origin_build_manifest_sha256": bm.get("origin_build_manifest_sha256"),
                             "entries_changed_vs_parent": sorted(e["path"] for e in bm["files"] if {f["path"]: f["sha256"] for f in parent_bm["files"]}.get(e["path"]) != e["sha256"])}
    # SOURCE-MANIFEST: every candidate file except itself and sidecar.
    sm_b = rd(CAND + "/SOURCE-MANIFEST.json")
    sm = json.loads(sm_b)
    on_disk = [p for p in files_under(CAND) if p not in ("SOURCE-MANIFEST.json", SIDECAR)]
    listed_sm = sorted(f["path"] for f in sm["files"])
    bad = [f["path"] for f in sm["files"] if sha(rd(CAND + "/" + f["path"])) != f["sha256"]]
    if listed_sm != on_disk or bad:
        rec["stops"].append("SOURCE-MANIFEST does not cover the candidate exactly")
    if sm["build_manifest_sha256"] != sha(bm_b):
        rec["stops"].append("SOURCE-MANIFEST build_manifest_sha256 differs")
    side = rd(CAND + "/" + SIDECAR).decode().split()[0]
    rec["source_manifest"] = {"sha256": sha(sm_b), "entries": len(sm["files"]), "sidecar_equal": side == sha(sm_b),
                              "build_manifest_sha256": sm["build_manifest_sha256"], "mismatch": bad}
    # compiler_build_digest prediction with a method control.
    control = sha(compact({"compiler_source_manifest_sha256": PARENT_BM, "core_closure_sha256": CORE}))
    rec["compiler_build_digest"] = {"method_control_parent": control, "method_control_equal_observed": control == OBSERVED_PARENT_DIGEST,
                                    "predicted_successor": sha(compact({"compiler_source_manifest_sha256": sha(bm_b), "core_closure_sha256": CORE})),
                                    "status": "arithmetic prediction only; evidence only when an authorized run of the reviewed r3 harness prints it from the copied bytes"}
    if control != OBSERVED_PARENT_DIGEST:
        rec["stops"].append("method control failed")
    # Delta against the parent SOURCE-MANIFEST.
    psm_b = rd(PARENT + "/SOURCE-MANIFEST.json")
    if sha(psm_b) != PARENT_SM:
        rec["stops"].append("parent SOURCE-MANIFEST changed")
    prior = {f["path"]: f["sha256"] for f in json.loads(psm_b)["files"]}
    ddir = OWN + "/DIFFS/candidate"
    os.makedirs(ddir, exist_ok=False)
    rows = []
    for rel in on_disk:
        now = sha(rd(CAND + "/" + rel))
        if rel not in prior:
            rows.append({"path": rel, "status": "added", "sha256": now})
        elif prior[rel] == now:
            rows.append({"path": rel, "status": "unchanged", "sha256": now})
        else:
            a = rd(PARENT + "/" + rel).decode("utf-8").splitlines(keepends=True)
            b = rd(CAND + "/" + rel).decode("utf-8").splitlines(keepends=True)
            text = "".join(difflib.unified_diff(a, b, fromfile="parent-a6e17efd/" + rel, tofile="resume-candidate/" + rel, n=3))
            name = rel.replace("/", "__") + ".diff"
            fd = os.open(ddir + "/" + name, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
            with os.fdopen(fd, "w", encoding="utf-8") as fh:
                fh.write(text)
            rows.append({"path": rel, "status": "changed", "parent_sha256": prior[rel], "sha256": now, "diff": "DIFFS/candidate/" + name,
                         "diff_sha256": sha(rd(ddir + "/" + name)),
                         "lines_removed": sum(1 for l in text.splitlines() if l.startswith("-") and not l.startswith("---")),
                         "lines_added": sum(1 for l in text.splitlines() if l.startswith("+") and not l.startswith("+++"))})
    rec["delta_vs_parent"] = {"parent": PARENT, "parent_source_manifest_sha256": PARENT_SM,
                              "counts": {s: sum(1 for r in rows if r["status"] == s) for s in ("unchanged", "changed", "added")},
                              "parent_files_missing": sorted(set(prior) - set(on_disk)),
                              "changed": [r for r in rows if r["status"] != "unchanged"]}
    rec["verdict"] = "PASS" if not rec["stops"] else "STOP"
    out = OWN + "/records/CANDIDATE-IDENTITY.json"
    fd = os.open(out, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "w") as fh:
        json.dump(rec, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(json.dumps({k: rec[k] for k in ("verdict", "stops", "build_manifest", "source_manifest", "compiler_build_digest")}, indent=1))
    print(rec["delta_vs_parent"]["counts"], [(r["path"], r["status"]) for r in rec["delta_vs_parent"]["changed"]])


if __name__ == "__main__":
    main()
