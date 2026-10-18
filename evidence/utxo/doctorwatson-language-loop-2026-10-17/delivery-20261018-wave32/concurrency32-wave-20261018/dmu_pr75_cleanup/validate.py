"""Pure offline validation of a literal two-document patch; no product imports."""
import argparse
import hashlib
import json
import pathlib
import posixpath
import re
import subprocess

ap = argparse.ArgumentParser()
ap.add_argument("--repo", required=True)
args = ap.parse_args()
root = pathlib.Path(__file__).resolve().parent
manifest = json.loads((root / "MANIFEST.json").read_text())
base = manifest["base_commit"]
def git(*parts):
    return subprocess.check_output(["git", "-C", args.repo, *parts])
def digest(value):
    return hashlib.sha256(value).hexdigest()
patch = (root / manifest["patch_file"]).read_bytes()
assert digest(patch) == manifest["patch_sha256"]
lines = patch.decode().splitlines(True)
i = 0
checked = []
while i < len(lines):
    assert lines[i].startswith("--- a/")
    path = lines[i][6:].strip()
    i += 1
    assert lines[i] == "+++ b/" + path + "\n"
    i += 1
    pin = next(p for p in manifest["changed_files"] if p["path"] == path)
    raw = git("show", base + ":" + path)
    assert digest(raw) == pin["before_sha256"]
    assert git("rev-parse", base + ":" + path).decode().strip() == pin["git_blob"]
    old = raw.decode().splitlines(True)
    result = []
    cursor = 0
    while i < len(lines) and lines[i].startswith("@@ "):
        match = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", lines[i])
        assert match
        i += 1
        start = int(match[1]) - 1
        assert start >= cursor
        result.extend(old[cursor:start])
        cursor = start
        removed = added = 0
        while i < len(lines) and lines[i][:1] in (" ", "+", "-") and not lines[i].startswith("--- a/"):
            line = lines[i]
            i += 1
            if line[0] in " -":
                assert old[cursor] == line[1:]
                cursor += 1
                removed += 1
            if line[0] in " +":
                result.append(line[1:])
                added += 1
        assert removed == int(match[2] or 1)
        assert added == int(match[4] or 1)
    result.extend(old[cursor:])
    after = "".join(result).encode()
    assert after == (root / "files" / path).read_bytes()
    assert digest(after) == pin["after_sha256"]
    marker = b"## Shared library report constructor"
    if marker in raw:
        assert raw[raw.index(marker):] == after[after.index(marker):]
    for target in re.findall(r"\]\(([^)]+)\)", after.decode()):
        if "://" in target:
            continue
        resolved = posixpath.normpath(posixpath.join(posixpath.dirname(path), target)).rstrip("/")
        git("cat-file", "-e", base + ":" + resolved)
    checked.append(path)
assert checked == ["docs/CORE-STAGED-TESTS.md", "models/core-stage-tests/PROVENANCE.md"]
for pin in manifest["source_pins"]:
    assert digest(git("show", base + ":" + pin["path"])) == pin["sha256"]
    assert git("rev-parse", base + ":" + pin["path"]).decode().strip() == pin["git_blob"]
print(json.dumps({"result": "PASS_STATIC_ONLY", "base": base, "patch_sha256": digest(patch), "paths": checked, "product_runtime": "UNRUN"}, indent=2))
