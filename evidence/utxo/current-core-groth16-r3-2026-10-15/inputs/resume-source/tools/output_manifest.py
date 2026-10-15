"""Write OUTPUT-MANIFEST.json (exclusive): SHA-256 of every file in this directory except
the wrapper-owned and private files, which are skipped by NAME and never opened."""
import datetime, hashlib, json, os, resource

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

OWN = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015"
EXCLUDED_TOP = {"DELAYED-START.json", "PREPARATION.json", "PROCESS.json", "PROMPT.md", "process.stderr",
                "provider-result.private.json", "run-delayed.py", "FINAL-RESPONSE.md", "OUTPUT-MANIFEST.json"}
files, excluded_present = [], []
for dp, dns, fns in os.walk(OWN):
    dns[:] = [d for d in dns if d != "__pycache__"]
    for fn in fns:
        rel = os.path.relpath(os.path.join(dp, fn), OWN).replace(os.sep, "/")
        if rel in EXCLUDED_TOP or "private" in fn.lower():
            excluded_present.append(rel)
            continue
        b = open(os.path.join(OWN, rel), "rb").read()
        files.append({"path": rel, "sha256": hashlib.sha256(b).hexdigest(), "bytes": len(b)})
files.sort(key=lambda f: f["path"])
agg = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
doc = {"format": "evidence-repair-resume-output-manifest/1", "at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
       "rule": "Every file of this directory except the wrapper-owned and private names listed in excluded (skipped by name, never opened). aggregate = SHA-256 of the compact sorted-key JSON of files.",
       "excluded_names_present": sorted(excluded_present), "excluded_rule": sorted(EXCLUDED_TOP),
       "count": len(files), "aggregate_sha256": agg, "files": files}
data = (json.dumps(doc, indent=1) + "\n").encode()
fd = os.open(OWN + "/OUTPUT-MANIFEST.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
with os.fdopen(fd, "wb") as f:
    f.write(data)
print(hashlib.sha256(data).hexdigest(), len(files), agg, sorted(excluded_present))
