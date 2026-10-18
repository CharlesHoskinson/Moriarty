"""Write review-overlay/R3-MANIFEST.json and DIFFS/review/*.diff (both created exclusively).

Stdlib only. Inherited files are compared with their public-partial pins (seed record),
r2 files with R2-MANIFEST pins. Reads the r2 overlay and the seed record read-only.
"""
import difflib, hashlib, json, os, resource

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
OWN = BASE + "/current-core-groth16-evidence-repair-resume-source-20261015"
OV = OWN + "/review-overlay"
R2 = BASE + "/current-core-groth16-compiler-repair-independent-review-20261015/plumbing-r2"
SEED = OWN + "/records/SEED-RECORD.json"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def rd(p):
    return open(p, "rb").read()


def write_new(path, data):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "wb") as f:
        f.write(data)


seed = {r["dest"]: r for r in json.load(open(SEED))["files"]}
files = []
for dp, dns, fns in os.walk(OV):
    for fn in fns:
        rel = os.path.relpath(os.path.join(dp, fn), OV).replace(os.sep, "/")
        if rel == "R3-MANIFEST.json" or "__pycache__" in rel:
            continue
        b = rd(os.path.join(OV, rel))
        inherited = seed.get("review-overlay/" + rel)
        if inherited is None:
            origin = "resume-new"
        elif inherited["sha256"] == sha(b):
            origin = "inherited-partial-unchanged"
        else:
            origin = "resume-edited (inherited partial " + inherited["sha256"] + ")"
        files.append({"path": rel, "sha256": sha(b), "bytes": len(b), "origin": origin})
files.sort(key=lambda f: f["path"])
r2m = json.load(open(R2 + "/R2-MANIFEST.json"))
r2_paths = {f["path"] for f in r2m["files"]}
frozen = {f["path"] for f in json.load(open(BASE + "/current-core-groth16-compiler-independent-source-review-20261015/REVIEW-MANIFEST.json"))["files"]}
collide = sorted({f["path"] for f in files} & (r2_paths | frozen | {"R2-MANIFEST.json"}))
assert not collide, collide
manifest = {
    "format": "evidence-repair-r3-overlay-manifest/1",
    "author": "claude-opus-5-5, evidence-repair resume source author (AUTH overlay; NOT independent)",
    "rule": "Every file under review-overlay/ except this manifest. E0r3 (r3-tools/copy_verify_r3.py) copies each to <root>/review/<path> beside the 42 frozen packet files and the r2 overlay; no path collides with either (checked when this manifest was written).",
    "files": files,
}
data = (json.dumps(manifest, indent=1, sort_keys=True) + "\n").encode()
write_new(OV + "/R3-MANIFEST.json", data)

ddir = OWN + "/DIFFS/review"
os.makedirs(ddir, exist_ok=False)
INHERITED = OWN + "/inherited-r3-draft-view"  # never created; inherited bytes come from the old partial path
OLD = BASE + "/current-core-groth16-evidence-repair-source-20261015/review-overlay"
pairs = [
    ("heldout.rs.vs-r2.diff", R2 + "/proposals/heldout-harness-r2/tests/heldout.rs", OV + "/proposals/heldout-harness-r3/tests/heldout.rs", "r2/heldout.rs (reviewed, executed)", "r3-resume/heldout.rs"),
    ("heldout.rs.vs-inherited-r3-draft.diff", OLD + "/proposals/heldout-harness-r3/tests/heldout.rs", OV + "/proposals/heldout-harness-r3/tests/heldout.rs", "r3-inherited-draft/heldout.rs (0dd8b1f5, UNACCEPTED partial)", "r3-resume/heldout.rs"),
    ("Cargo.toml.vs-r2.diff", R2 + "/proposals/heldout-harness-r2/Cargo.toml", OV + "/proposals/heldout-harness-r3/Cargo.toml", "r2/Cargo.toml", "r3-resume/Cargo.toml"),
    ("Cargo.toml.vs-inherited-r3-draft.diff", OLD + "/proposals/heldout-harness-r3/Cargo.toml", OV + "/proposals/heldout-harness-r3/Cargo.toml", "r3-inherited-draft/Cargo.toml", "r3-resume/Cargo.toml"),
    ("lib.rs.vs-r2.diff", R2 + "/proposals/heldout-harness-r2/src/lib.rs", OV + "/proposals/heldout-harness-r3/src/lib.rs", "r2/src/lib.rs", "r3-resume/src/lib.rs"),
    ("copy_verify_r3.py.vs-r2.diff", R2 + "/r2-tools/copy_verify_r2.py", OV + "/r3-tools/copy_verify_r3.py", "r2-tools/copy_verify_r2.py (reviewed, executed as E0r2)", "r3-tools/copy_verify_r3.py (PROPOSED)"),
]
index = []
for name, a, b, fa, fb in pairs:
    ta = rd(a).decode().splitlines(keepends=True)
    tb = rd(b).decode().splitlines(keepends=True)
    text = "".join(difflib.unified_diff(ta, tb, fromfile=fa, tofile=fb, n=3)).encode()
    write_new(ddir + "/" + name, text)
    index.append({"diff": "DIFFS/review/" + name, "sha256": sha(text), "from": a, "from_sha256": sha(rd(a)), "to": b, "to_sha256": sha(rd(b)),
                  "lines_removed": sum(1 for l in text.decode().splitlines() if l.startswith("-") and not l.startswith("---")),
                  "lines_added": sum(1 for l in text.decode().splitlines() if l.startswith("+") and not l.startswith("+++"))})
write_new(ddir + "/INDEX.json", (json.dumps(index, indent=1) + "\n").encode())
print(sha(data), len(files))
for f in files:
    print(f["path"], f["origin"][:40], f["sha256"][:12])
for i in index:
    print(i["diff"], "-", i["lines_removed"], "+", i["lines_added"])
