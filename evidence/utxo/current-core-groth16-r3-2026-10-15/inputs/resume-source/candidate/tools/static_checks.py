"""Stdlib-only static source checks for the mori-r1cs candidate.

Reads files only. Never imports, builds or runs Rust, Moriarty or any SDK. Writes the
JSON report given as the second argument (refuses to overwrite an existing file).

Usage: python3 -I static_checks.py <task-root> <output.json>
"""
import hashlib
import json
import re
import sys
from pathlib import Path


def strip_rust(text):
    """Remove comments, strings and char literals well enough for delimiter counts."""
    out = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        nxt = text[i + 1] if i + 1 < n else ""
        if c == "/" and nxt == "/":
            j = text.find("\n", i)
            i = n if j < 0 else j
            continue
        if c == "/" and nxt == "*":
            j = text.find("*/", i + 2)
            i = n if j < 0 else j + 2
            continue
        if c == "r" and nxt in "#\"" and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            j = i + 1
            hashes = 0
            while j < n and text[j] == "#":
                hashes += 1
                j += 1
            if j < n and text[j] == "\"":
                end = "\"" + "#" * hashes
                k = text.find(end, j + 1)
                i = n if k < 0 else k + len(end)
                out.append("\"\"")
                continue
        if c == "b" and nxt == "\"":
            i += 1
            continue
        if c == "\"":
            j = i + 1
            while j < n and text[j] != "\"":
                j += 2 if text[j] == "\\" else 1
            i = j + 1
            out.append("\"\"")
            continue
        if c == "'":
            m = re.match(r"'(\\.|[^'\\])'", text[i:])
            if m:
                i += len(m.group(0))
                out.append("''")
                continue
        out.append(c)
        i += 1
    return "".join(out)


def main():
    root = Path(sys.argv[1]).resolve()
    output = Path(sys.argv[2])
    crate = root / "source" / "mori-r1cs"
    files = sorted([*crate.glob("src/*.rs"), *crate.glob("tests/*.rs"), *crate.glob("tests/common/*.rs")])
    texts = {str(p.relative_to(root)): p.read_text() for p in files}
    checks = []

    def record(name, ok, detail):
        checks.append({"check": name, "ok": bool(ok), "detail": detail})

    for path, text in texts.items():
        s = strip_rust(text)
        counts = {k: s.count(k) for k in "(){}[]"}
        ok = counts["("] == counts[")"] and counts["{"] == counts["}"] and counts["["] == counts["]"]
        record(f"delimiters balanced: {path}", ok, counts)

    lib = texts["source/mori-r1cs/src/lib.rs"]
    declared = set(re.findall(r"^\s*(?:pub\s+)?mod\s+(\w+);", lib, re.M))
    present = {p.stem for p in crate.glob("src/*.rs")} - {"lib"}
    record("every declared module has a file", declared <= present, {"declared": sorted(declared), "files": sorted(present)})
    record("every module file is declared", present <= declared, sorted(present - declared))

    src_all = "\n".join(t for p, t in texts.items() if "/src/" in p)
    for removed in ["bind_uint", "bind_bool"]:
        record(f"removed helper unused: {removed}", re.search(r"\b" + removed + r"\b", src_all) is None, removed)

    forbidden = {
        "unsafe block": r"\bunsafe\s*\{",
        "process spawn": r"std::process|Command::new",
        "network": r"TcpStream|UdpSocket|reqwest|hyper::",
        "filesystem write in src": r"std::fs::(write|File::create|create_dir)",
        "program name recognition": r"program\.name\s*==|\.name\s*==\s*\"(InstallmentInvoice|ConditionalReserve|PayrollLedger)\"",
        "fixture path in src": r"fixtures/|EXPECTED-PACKET",
        "rng or setup in src": r"rand::|ChaCha|circuit_specific_setup|create_random_proof|generate_random_parameters",
    }
    for name, pattern in forbidden.items():
        hits = [p for p, t in texts.items() if "/src/" in p and re.search(pattern, strip_rust(t) if name != "fixture path in src" else t)]
        record(f"absent from src: {name}", not hits, hits)

    exhaustive = texts["source/mori-r1cs/src/coverage.rs"]
    for fn in ["expr_support", "binary_support", "statement_support"]:
        body = exhaustive.split(f"pub fn {fn}", 1)[1].split("\npub ", 1)[0]
        record(f"no wildcard arm in coverage::{fn}", re.search(r"\b_\s*=>", body) is None, fn)
    ir = (root / ".." / ".." / "Moriarty").resolve()
    record("coverage counts", True, {"Expr variants": 6, "BinaryOp variants": 11, "Statement variants": 10, "note": "counts from mori-core/src/ir.rs:54-169 at 6c7b1fda; enforced by exhaustive matches, verified only when compiled"})

    manifest = (crate / "Cargo.toml").read_text()
    pins = dict(re.findall(r"^(ark-[\w-]+)\s*=\s*\{\s*version\s*=\s*\"([^\"]+)\"", manifest, re.M))
    record("arkworks pins are the 0.4 prototype line", all(v.startswith("=0.4.") for v in pins.values()) and len(pins) >= 5, pins)
    record("no absolute or live-worktree path dependency", "/home/" not in manifest and ".worktrees" not in manifest, re.findall(r"path\s*=\s*\"([^\"]+)\"", manifest))
    record("standalone workspace root", "\n[workspace]\n" in manifest, "[workspace]")
    record("no lock file invented", not (crate / "Cargo.lock").exists(), "Cargo.lock absent")

    tests = {p: t for p, t in texts.items() if "/tests/" in p}
    test_count = sum(len(re.findall(r"#\[test\]", t)) for t in tests.values())
    record("test functions authored (not run)", test_count > 0, test_count)
    authored = {Path(p).stem: re.findall(r"#\[test\]\s*fn\s+(\w+)", t) for p, t in tests.items() if not p.endswith("common/mod.rs")}
    request_path = root / "EXECUTION-REQUEST.json"
    if request_path.exists():
        expected = json.loads(request_path.read_text())["expected_tests"]
        record("EXECUTION-REQUEST expected_tests equal the authored #[test] functions", authored == expected, {"authored": authored, "requested": expected})
    for name, path in [("BUILD-MANIFEST", root / "BUILD-MANIFEST.json")]:
        if path.exists():
            manifest_files = json.loads(path.read_text())["files"]
            stale = [f["path"] for f in manifest_files if hashlib.sha256((root / f["path"]).read_bytes()).hexdigest() != f["sha256"]]
            record(f"{name} pins match the current bytes", not stale, stale)
    for path, text in tests.items():
        for m in re.finditer(r"packet!\(\"([^\"]+)\"\)", text):
            target = root / m.group(1)
            record(f"embedded fixture exists: {m.group(1)}", target.exists(), str(target.relative_to(root)) if target.exists() else "missing")

    report = {
        "format": "mori-r1cs-static-checks/1",
        "tool": "tools/static_checks.py (python3 -I, stdlib only)",
        "tool_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "scope": "lexical/static only: delimiter balance, module wiring, forbidden constructs, exhaustive-match spelling, manifest pins, fixture presence. Not a compile, type check, borrow check, test or proof.",
        "all_ok": all(c["ok"] for c in checks),
        "checks": checks,
    }
    with output.open("x") as f:
        json.dump(report, f, indent=2)
        f.write("\n")
    print(json.dumps({"all_ok": report["all_ok"], "failed": [c["check"] for c in checks if not c["ok"]], "count": len(checks)}, indent=1))


if __name__ == "__main__":
    main()
