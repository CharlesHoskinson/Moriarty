#!/usr/bin/env python3
"""PROPOSED E0 tool for an authorized isolated run (reviewer-authored, stdlib only).
Builds <dest>/{moriarty,r1cs,review} as byte copies of exactly the pinned files and
verifies every SHA-256 after copying. With --dry-run it only verifies the SOURCES
(read-only) and writes nothing; the reviewer ran only --dry-run.

Usage:
  python3 -I copy_verify.py --dry-run CHECKOUT CANDIDATE REVIEW
  python3 -I copy_verify.py --copy CHECKOUT CANDIDATE REVIEW DEST   (DEST must not exist)
"""
import resource, sys, os, json, hashlib, shutil
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

mode = sys.argv[1]
checkout, candidate, review = sys.argv[2:5]
dest = sys.argv[5] if mode == "--copy" else None


def sha(p):
    with open(p, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


plan = []  # (src, rel_dest, expected_sha)
pins = json.load(open(os.path.join(candidate, "CORE-CLOSURE-PINS.json")))
for f in pins["files"]:
    plan.append((os.path.join(checkout, f["path"]), os.path.join("moriarty", f["path"]), f["sha256"]))
sm = json.load(open(os.path.join(candidate, "SOURCE-MANIFEST.json")))
for f in sm["files"]:
    plan.append((os.path.join(candidate, f["path"]), os.path.join("r1cs", f["path"]), f["sha256"]))
plan.append((os.path.join(candidate, "SOURCE-MANIFEST.json"), os.path.join("r1cs", "SOURCE-MANIFEST.json"), sha(os.path.join(candidate, "SOURCE-MANIFEST.json"))))
rm = json.load(open(os.path.join(review, "REVIEW-MANIFEST.json")))
for f in rm["files"]:
    plan.append((os.path.join(review, f["path"]), os.path.join("review", f["path"]), f["sha256"]))

bad = [(s, e, sha(s) if os.path.exists(s) else None) for s, _, e in plan if not os.path.exists(s) or sha(s) != e]
summary = {"mode": mode, "planned_files": len(plan), "moriarty": sum(1 for _, d, _ in plan if d.startswith("moriarty/")),
           "r1cs": sum(1 for _, d, _ in plan if d.startswith("r1cs/")), "review": sum(1 for _, d, _ in plan if d.startswith("review/")),
           "source_mismatches": bad,
           "core_closure_sha256": pins["closure_sha256"],
           "candidate_source_manifest_sha256": sha(os.path.join(candidate, "SOURCE-MANIFEST.json")),
           "review_manifest_sha256": sha(os.path.join(review, "REVIEW-MANIFEST.json"))}
if bad:
    print(json.dumps(summary, indent=1))
    sys.exit(2)
if mode == "--copy":
    if os.path.exists(dest):
        sys.exit("destination exists; refusing")
    for src, rel, exp in plan:
        out = os.path.join(dest, rel)
        os.makedirs(os.path.dirname(out), exist_ok=True)
        shutil.copyfile(src, out)
        if sha(out) != exp:
            sys.exit(f"post-copy mismatch {rel}")
    summary["copied"] = True
print(json.dumps(summary, indent=1))
