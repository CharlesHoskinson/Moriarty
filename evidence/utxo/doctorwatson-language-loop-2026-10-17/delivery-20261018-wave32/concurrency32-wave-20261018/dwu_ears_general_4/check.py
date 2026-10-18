#!/usr/bin/env python3
"""Offline source/literal/patch integrity only; never runs product commands."""
import argparse
import difflib
import hashlib
import json
from pathlib import Path
import re
import subprocess

packet = Path(__file__).resolve().parent
args = argparse.ArgumentParser()
args.add_argument("--repo", required=True)
repo = Path(args.parse_args().repo)
manifest = json.loads((packet / "MANIFEST.json").read_text())
base = manifest["baseCommit"]
assert re.fullmatch(r"[0-9a-f]{40}", base)
sha = lambda value: hashlib.sha256(value).hexdigest()
source = {}
for pin in manifest["sourcePins"]:
    value = subprocess.check_output(["git", "show", f"{base}:{pin['path']}"], cwd=repo)
    blob = subprocess.check_output(["git", "rev-parse", f"{base}:{pin['path']}"], cwd=repo).decode().strip()
    assert sha(value) == pin["sha256"] and blob == pin["gitBlob"], pin["path"]
    source[pin["path"]] = value.decode()
expected_bytes = (packet / manifest["expectations"]["path"]).read_bytes()
assert sha(expected_bytes) == manifest["expectations"]["sha256"]
expected = json.loads(expected_bytes)
patch = ""
after = {}
for change in manifest["changes"]:
    path = change["path"]
    before = (packet / "before" / path).read_bytes()
    value = (packet / "after" / path).read_bytes()
    assert before.decode() == source[path]
    assert sha(before) == change["beforeSha256"] and sha(value) == change["afterSha256"]
    after[path] = value.decode()
    patch += "".join(difflib.unified_diff(before.decode().splitlines(keepends=True), value.decode().splitlines(keepends=True), fromfile="a/"+path, tofile="b/"+path))
assert patch == (packet / manifest["patch"]["path"]).read_text()
assert sha(patch.encode()) == manifest["patch"]["sha256"]
assert set(after) == {"packages/moriarty-beta/GETTING-STARTED.md", "docs/NATIVE-CORE.md"}
beta = after["packages/moriarty-beta/GETTING-STARTED.md"].split("## Read the program")[0]
core = after["docs/NATIVE-CORE.md"].split("The following examples select")[0]
pkg = json.loads(source["packages/moriarty-beta/package.json"])
b = expected["beta"]
assert pkg["name"] == b["package"] and pkg["version"] == b["version"]
assert pkg["bin"] == {"mori": b["bin"]} and b["archive"] in beta
assert "['src/cli.ts','dist/cli.js']" in source["packages/moriarty-beta/build.mjs"]
cli = source["packages/moriarty-beta/src/cli.ts"]
assert "filename=repay?'repayment.mori':'invoice.mori',action=repay?'repay_loan':'pay'" in cli
assert "writeFileSync(join(path,'README.md')" in cli
assert "profile.value !== 'moriarty-beta/1'" in source["packages/moriarty-beta/src/frontend.ts"]
for mode in ["transfer", "repay"]:
    assert b[mode]["source"] in beta and "--action " + b[mode]["action"] in beta
assert "MORI_BETA_PKG=\"$PWD/node_modules/@moriarty-lang/beta\"" in beta
assert 'mori() { node "$MORI_BETA_PKG/dist/cli.js" "$@"; }' in beta
cargo = source["crates/mori/Cargo.toml"]
assert 'name = "mori"' in cargo and 'default = ["native"]' in cargo
for line in core.splitlines():
    if line.startswith("cargo run "):
        assert "--no-default-features --bin mori --" in line
initializer = source["crates/mori/src/commands/init.rs"]
assert "std::env::current_exe()" in initializer and "executable.replace" in initializer
assert '(TRANSFER, "pay"' in initializer and '(REPAY, "repay"' in initializer
assert 'language "moriarty/1";' in initializer
assert 'mori_core::parse(source)' in source["crates/mori/src/commands/check_source.rs"]
assert 'post == &actual.post && effects == &actual.effects' in source["crates/mori/src/commands/test.rs"]
for block in re.findall(r"```sh\n(.*?)```", beta + core, re.S):
    assert not any(item in block for item in ["/absolute/", "NEW_DIR", "/path/to/", "COMMAND"])
for path, marker in [("packages/moriarty-beta/GETTING-STARTED.md", "## Read the program"), ("docs/NATIVE-CORE.md", "`check` reads at most 1 MiB")]:
    assert source[path].split(marker,1)[1] == after[path].split(marker,1)[1]
print(json.dumps({"status":"source-literal-checks-pass", "baseCommit":base, "sourcePins":len(source), "changedDocuments":2, "patchSha256":sha(patch.encode()), "expectationsSha256":sha(expected_bytes), "runtime":"not executed", "acceptance":"unestablished"}, indent=2))
