#!/usr/bin/env python3
"""Reviewer-owned: write REVIEW-MANIFEST.json (files a future authorized run would copy
from this review directory). Excludes harness-owned/private files, __pycache__, and the
documents that reference this manifest's digest (REPORT.md, CORRECTED-EXECUTION-REQUEST.json,
INPUT-MANIFEST.json, REVIEW-FAILURES.json). Usage: python3 -I review_manifest.py REVIEW_DIR"""
import resource, sys, os, json, hashlib
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
root = sys.argv[1]
include_top = {"heldout", "heldout-addendum-2", "proposals", "tools"}
include_files = {"HOSTILE-CLAIMS-ADDENDUM.json", "HELDOUT-FREEZE.json", "LOCK-UNION-ANALYSIS.json", "PIN-VERIFICATION.json"}
files = []
for dp, dns, fns in os.walk(root):
    dns[:] = sorted(d for d in dns if d != "__pycache__")
    for fn in sorted(fns):
        p = os.path.join(dp, fn)
        rel = os.path.relpath(p, root)
        top = rel.split(os.sep)[0]
        if (top in include_top and os.sep in rel) or rel in include_files:
            b = open(p, "rb").read()
            files.append({"path": rel, "sha256": hashlib.sha256(b).hexdigest(), "bytes": len(b)})
files.sort(key=lambda f: f["path"])
doc = {"format": "reviewer-review-manifest/1", "rule": "files a future authorized isolated run copies to <root>/review/; see tools/review_manifest.py",
       "files": files}
out = os.path.join(root, "REVIEW-MANIFEST.json")
with open(out, "w") as f:
    json.dump(doc, f, indent=2)
    f.write("\n")
print(len(files), hashlib.sha256(open(out, "rb").read()).hexdigest())
