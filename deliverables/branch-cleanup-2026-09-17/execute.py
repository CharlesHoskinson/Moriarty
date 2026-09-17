import json, subprocess, sys
from pathlib import Path
ROOT = Path(__file__).parent
plan = json.loads((ROOT / "plan.json").read_text())

def git(*args):
    return subprocess.check_output(["git", *args], text=True).strip()

def refs():
    return dict((ref, sha) for sha, ref in (line.split() for line in git("ls-remote", "--refs", "origin").splitlines()))

remote = refs()
assert remote["refs/heads/main"] == plan["main_sha"], "main changed"
for row in plan["branches"]:
    assert remote.get("refs/heads/" + row["branch"]) == row["sha"], "branch changed: " + row["branch"]
    tag = "refs/tags/" + row["archive_tag"]
    assert tag not in remote or remote[tag] == row["sha"], "archive collision: " + tag
mode = sys.argv[1]
if mode == "archive":
    specs = [row["sha"] + ":refs/tags/" + row["archive_tag"] for row in plan["branches"]]
    subprocess.run(["git", "push", "--atomic", "origin", *specs], check=True)
    remote = refs()
    assert all(remote.get("refs/tags/" + row["archive_tag"]) == row["sha"] for row in plan["branches"])
    (ROOT / "archive-verified.json").write_text(json.dumps({"verified": True, "tags": {r["archive_tag"]: r["sha"] for r in plan["branches"]}}, indent=2) + "\n")
elif mode == "retire":
    assert all(remote.get("refs/tags/" + row["archive_tag"]) == row["sha"] for row in plan["branches"]), "archive absent or changed"
    leases = ["--force-with-lease=refs/heads/" + r["branch"] + ":" + r["sha"] for r in plan["branches"]]
    specs = [":refs/heads/" + r["branch"] for r in plan["branches"]]
    subprocess.run(["git", "push", "--atomic", *leases, "origin", *specs], check=True)
    remote = refs()
    assert all("refs/heads/" + r["branch"] not in remote for r in plan["branches"])
    assert all(remote.get("refs/tags/" + r["archive_tag"]) == r["sha"] for r in plan["branches"])
    assert remote["refs/heads/main"] == plan["main_sha"]
    (ROOT / "retired-verified.json").write_text(json.dumps({"verified": True, "remaining_heads": {k: v for k,v in remote.items() if k.startswith("refs/heads/")}, "archive_tags_verified": len(plan["branches"])}, indent=2) + "\n")
else:
    raise SystemExit("expected archive or retire")
