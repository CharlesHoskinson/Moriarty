#!/usr/bin/env python3
"""Repair-author lexical static checks (stdlib only; reads ORIGIN and SUCCESSOR, writes OUT
exclusively). These are text checks, not a compile or type check: they can catch the F-01
name collision class and byte-level invariance, not type errors.

Usage: python3 -I repair_static_checks.py ORIGIN SUCCESSOR OUT_JSON
"""
import json
import re
import resource
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

origin, succ, out = sys.argv[1:4]
CRATE = "source/mori-r1cs"
TESTS7 = ["accounting.rs", "arithmetic.rs", "duties_frame.rs", "generic_pipeline.rs", "propose.rs", "refusals.rs", "tamper.rs"]
checks = []


def read(root, rel):
    return open(f"{root}/{rel}", encoding="utf-8").read()


def check(cid, ok, detail):
    checks.append({"id": cid, "pass": bool(ok), "detail": detail})


def strip_code(text):
    """Remove comments, string, raw-string and char literals (approximate Rust lexer)."""
    out, i, n = [], 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            depth, i = 1, i + 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif text.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
        elif c == "r" and re.match(r'r#*"', text[i:]) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = re.match(r'r(#*)"', text[i:]).group(1)
            end = text.find('"' + hashes, i + 2 + len(hashes))
            i = n if end < 0 else end + 1 + len(hashes)
            out.append('""')
        elif c == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            out.append('""')
        elif c == "'" and re.match(r"'(\\.|[^\\'])'", text[i:]):
            i += len(re.match(r"'(\\.|[^\\'])'", text[i:]).group(0))
            out.append("' '")
        else:
            out.append(c)
            i += 1
    return "".join(out)


def balanced(text):
    code, stack, pairs = strip_code(text), [], {")": "(", "]": "[", "}": "{"}
    for ch in code:
        if ch in "([{":
            stack.append(ch)
        elif ch in ")]}":
            if not stack or stack.pop() != pairs[ch]:
                return False
    return not stack


# C1: glob-import name collision between tests/common and the library (the F-01 class).
common = read(succ, f"{CRATE}/tests/common/mod.rs")
common_pub = set(re.findall(r"^pub (?:fn|const|struct|enum|static|mod|type) ([A-Za-z_][A-Za-z0-9_]*)", common, re.M))
lib = read(succ, f"{CRATE}/src/lib.rs")
lib_pub = set(re.findall(r"^pub mod ([A-Za-z_0-9]+);", lib, re.M)) | set(re.findall(r"^pub const ([A-Z_0-9]+)", lib, re.M))
for group in re.findall(r"^pub use [a-z0-9_:]+::\{([^}]*)\};", lib, re.M):
    lib_pub |= {x.strip() for x in group.split(",") if x.strip()}
# Pass-2/3 fix (pass 3: module paths may contain digits, e.g. r1cs): single-item re-exports such as `pub use r1cs::Tamper;` (missed in pass 1).
lib_pub |= set(re.findall(r"^pub use [a-z0-9_:]+::([A-Za-z_][A-Za-z0-9_]*);", lib, re.M))
collisions = sorted(common_pub & lib_pub)
check("C1-no-glob-collision", not collisions and "compile_ok" in common_pub and "compile" in lib_pub,
      {"collisions": collisions, "common_pub": sorted(common_pub), "lib_pub_count": len(lib_pub)})

# C2: no bare `compile(` call remains in any test file; the generic library call is qualified.
bare = re.compile(r"(?<![A-Za-z0-9_:])compile\(")
generic = re.compile(r"(?<![A-Za-z0-9_:])compile::<")
leftover = {}
for name in TESTS7 + ["assignment_shape.rs", "common/mod.rs"]:
    code = strip_code(read(succ, f"{CRATE}/tests/{name}"))
    hits = len(bare.findall(code)) + len(generic.findall(code))
    if hits:
        leftover[name] = hits
dframe = read(succ, f"{CRATE}/tests/duties_frame.rs")
check("C2-no-ambiguous-compile-call", not leftover and "mori_r1cs::compile::<ark_bls12_381::Fr>(admitted" in dframe, {"leftover": leftover})

# C3: frozen-outcome invariance of the seven inherited test files. After undoing the
# helper rename, every line must equal the origin, except the enumerated F-11 lines.
f11_allowed = {
    "//! The falsifiable source predicate: one compiler, one path (source -> actual Core",
    "//! parse/validate -> typed Program -> plan -> constraint lowering), several",
    "fn compile_source_is_the_only_path_and_it_never_reads_program_names() {",
    # Pass-2 fix: the F-11 comment line that was extended in place (missed in pass 1).
    "    // renamed agreement (different Program) yields a different program hash.",
}
inv = {}
for name in TESTS7 + ["common/mod.rs"]:
    a = read(origin, f"{CRATE}/tests/{name}").split("\n")
    b = read(succ, f"{CRATE}/tests/{name}").split("\n")
    nb = [l.replace("mori_r1cs::compile::<", "compile::<").replace("compile_ok(", "compile(") for l in b]
    import difflib
    sm = difflib.SequenceMatcher(a=a, b=nb, autojunk=False)
    removed, added = [], []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag != "equal":
            removed += a[i1:i2]
            added += nb[j1:j2]
    unexpected_removed = [l for l in removed if l not in f11_allowed]
    assert_touched = [l for l in removed + added if re.search(r"assert|matches!|panic!", l)]
    inv[name] = {"renamed_lines": sum(1 for x, y in zip(b, nb) if x != y), "removed_after_normalisation": removed,
                 "added_after_normalisation": added, "unexpected_removed": unexpected_removed, "assertion_lines_touched": assert_touched}
ok3 = all(not v["unexpected_removed"] and not v["assertion_lines_touched"] for v in inv.values())
ok3 = ok3 and all(not v["removed_after_normalisation"] and not v["added_after_normalisation"] for k, v in inv.items() if k != "generic_pipeline.rs")
check("C3-frozen-outcome-invariance", ok3, inv)

# C4: test inventory (expected: 39 inherited with one F-11 rename, plus 2 new F-12 tests).
tests = {}
for name in TESTS7 + ["assignment_shape.rs"]:
    text = read(succ, f"{CRATE}/tests/{name}")
    tests[name.removesuffix(".rs")] = re.findall(r"#\[test\]\s*\n\s*fn ([a-z0-9_]+)\(", text)
total = sum(len(v) for v in tests.values())
check("C4-test-inventory", total == 41 and "compile_source_is_deterministic_and_never_reads_program_names" in tests["generic_pipeline"]
      and "compile_source_is_the_only_path_and_it_never_reads_program_names" not in tests["generic_pipeline"], {"total": total, "tests": tests})

# C5: constraint-relevant source bytes unchanged.
unchanged = ["Cargo.toml"] + [f"src/{f}" for f in ["frontend.rs", "gadgets.rs", "groth16.rs", "identity.rs", "lib.rs", "lower.rs",
                                                   "plan.rs", "r1cs.rs", "staged.rs", "statement.rs", "witness.rs"]]
diff5 = [f for f in unchanged if read(origin, f"{CRATE}/{f}") != read(succ, f"{CRATE}/{f}")]
check("C5-constraint-sources-byte-identical", not diff5, {"files": unchanged, "differing": diff5})


def fn_text(text, name):
    m = re.search(r"^(?:pub )?fn " + re.escape(name) + r"\b", text, re.M)
    if not m:
        return None
    start, depth, i = text.index("{", m.end()), 0, None
    for i in range(start, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                break
    return text[m.start(): i + 1]


co, cs = read(origin, f"{CRATE}/src/circuit.rs"), read(succ, f"{CRATE}/src/circuit.rs")
same_fns = {n: fn_text(co, n) == fn_text(cs, n) and fn_text(co, n) is not None
            for n in ["compile", "compile_source", "probe", "synthesize", "alloc_value", "cval_wire", "uint_of", "bool_of", "hex_le"]}
check("C6-circuit-synthesis-functions-byte-identical", all(same_fns.values()), same_fns)

# C7: F-04 status and text.
cov = read(succ, f"{CRATE}/src/coverage.rs")
m = re.search(r'"mori::core::balance \(missing explicit balance\)",\s*Support::(\w+)', cov)
adapter = read(succ, f"{CRATE}/src/adapter.rs")
tob = adapter[adapter.index("pub const TARGET_OBLIGATIONS"):adapter.index("/// A typed slot wire")]
covmd = read(succ, "COVERAGE.md")
check("C7-F04-target-premise", m is not None and m.group(1) == "TargetPremise" and "TargetPremise," in cov
      and "review F-04" in tob and "review F-05" in tob and "TargetPremise" in covmd and "M-14" in covmd,
      {"missing_balance_support": m.group(1) if m else None})

# C8: the P3-7 wording does not assert external-only authority.
check("C8-P3-7-not-external-only", "f3ae8556" in tob and "the signer of each debited external account (KeyHeldExternal) and owner" not in tob, {})

# C9: delimiter balance of every Rust file (approximate lexer).
rust = [f"{CRATE}/src/{f}" for f in ["adapter.rs", "circuit.rs", "coverage.rs", "error.rs"]] + [f"{CRATE}/tests/{f}" for f in TESTS7 + ["assignment_shape.rs", "common/mod.rs"]]
bal = {f: balanced(read(succ, f)) for f in rust}
check("C9-delimiters-balanced", all(bal.values()), bal)

# C10: no CRLF or tab in edited Rust/Markdown files; files end with a newline.
fmt = {}
for f in rust + ["COVERAGE.md"]:
    t = read(succ, f)
    fmt[f] = {"crlf": "\r" in t, "tab": "\t" in t, "final_newline": t.endswith("\n")}
check("C10-format", all(not v["crlf"] and not v["tab"] and v["final_newline"] for v in fmt.values()), fmt)

# C11: F-12 wiring: circuit() calls the length check; the variant exists with a Display arm.
err = read(succ, f"{CRATE}/src/error.rs")
check("C11-F12-wiring", "self.check_assignment_lengths(a)?;" in cs and "AssignmentShape(String)," in err
      and "Self::AssignmentShape(m) =>" in err and "use crate::statement::{BodyValues," in cs, {})

# C12: reviewer harness names still exported (lib.rs unchanged covers it; record explicitly).
harness_names = ["AccountingBinding", "CapBinding", "CellRef", "CompileError", "CompileOptions", "CompiledRelation", "CompilerBuild",
                 "Digest32", "FrameCells", "StatementAdapter", "Tamper", "TransitionValues", "admit_program", "compile", "compile_source",
                 "initial_ledger_target_admits", "witness"]
missing = [n for n in harness_names if n not in lib_pub]
check("C12-harness-imports-exported", not missing, {"missing": missing})

record = {"format": "repair-static-checks/1", "nature": "lexical text checks only; not a compile, type check or test run",
          "origin": origin, "successor": succ, "checks": checks, "all_pass": all(c["pass"] for c in checks)}
with open(out, "x") as f:
    json.dump(record, f, indent=2)
    f.write("\n")
print(json.dumps({c["id"]: c["pass"] for c in checks} | {"all_pass": record["all_pass"]}, indent=1))
