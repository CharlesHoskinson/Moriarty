#!/usr/bin/env python3
"""F-01 mechanical test-only repair (stdlib only). Edits files in place under CRATE/tests.

1. tests/common/mod.rs: `pub fn compile(` becomes `pub fn compile_ok(`, so `common::*` no
   longer exports a name that collides with the library's glob-exported `compile`.
2. Every bare call `compile(` in the seven test files becomes `compile_ok(`. A call is
   bare when the preceding character is not an identifier character or `:` (so
   `try_compile(`, `compile_source(` and `mori_r1cs::compile::<..>(` are untouched).
3. duties_frame.rs: the generic library call `compile::<ark_bls12_381::Fr>(` becomes the
   explicit `mori_r1cs::compile::<ark_bls12_381::Fr>(`.

Prints every changed line (before/after). Refuses if a file's change count differs from
the expected count, so no unreviewed line can change.

Usage: python3 -I f01_rename.py CRATE
"""
import json
import re
import resource
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

crate = sys.argv[1]
BARE = re.compile(r"(?<![A-Za-z0-9_:])compile\(")
GENERIC = re.compile(r"(?<![A-Za-z0-9_:])compile::<ark_bls12_381::Fr>\(")
TESTS = ["accounting.rs", "arithmetic.rs", "duties_frame.rs", "generic_pipeline.rs", "propose.rs", "refusals.rs", "tamper.rs"]

report = {}


def edit(rel, transform, expect):
    path = f"{crate}/tests/{rel}"
    lines = open(path, encoding="utf-8").read().split("\n")
    changed = []
    out = []
    for n, line in enumerate(lines, 1):
        new = transform(line)
        if new != line:
            changed.append({"line": n, "before": line, "after": new})
        out.append(new)
    if len(changed) != expect:
        sys.exit(f"{rel}: {len(changed)} changed lines, expected {expect}; nothing written")
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    report[rel] = changed


edit("common/mod.rs", lambda l: l.replace("pub fn compile(", "pub fn compile_ok("), 1)
counts = {}
for name in TESTS:
    text = open(f"{crate}/tests/{name}", encoding="utf-8").read()
    counts[name] = sum(1 for l in text.split("\n") if BARE.search(l) or (name == "duties_frame.rs" and GENERIC.search(l)))
for name in TESTS:
    def t(line, name=name):
        line = BARE.sub("compile_ok(", line)
        if name == "duties_frame.rs":
            line = GENERIC.sub("mori_r1cs::compile::<ark_bls12_381::Fr>(", line)
        return line
    edit(name, t, counts[name])
print(json.dumps(report, indent=1))
