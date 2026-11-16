#!/usr/bin/env python3
"""Q-compact-compilation-1 regression (CF-002 vs ECC-003, text against code; ABI half only).
Pinned: Moriarty worktree mycroft-factory @ 6c93d6421, mycroft/compiler/src/lower.rs:47-54 (`struct Abi`:
selector, tape_len, slots, l_e, n_e, kits; no per-builtin count); openvm t25-int @ e8c2896
crates/sherlock-leaf/tools/mycroft-emit/src/main.rs:120-124 (no count field either); spec worktree
sherlock-e2e-spec @ c154662c4, openspec/changes/sherlock-processor-final-20261111/specs/sherlock-compact-frontend/spec.md:15
(CF-002: abi.json gives "the count per builtin").
PASS when either the ABI carries a per-builtin count, or CF-002 / ECC-003 is amended so the prototype ABI
does not need it. The `sherlock <verb>` half is OPEN DESIGN (D-085) and not checked. Exit 0 = PASS, 1 = FAIL."""
import pathlib, re, sys
H = pathlib.Path.home()
LOWER = H / "Projects/Moriarty-wt/mycroft-factory/mycroft/compiler/src/lower.rs"
CF = H / "Projects/Moriarty-wt/sherlock-e2e-spec/openspec/changes/sherlock-processor-final-20261111/specs/sherlock-compact-frontend/spec.md"
ECC = H / "Projects/Moriarty-wt/sherlock-e2e-spec/openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-compact-compiler/spec.md"
src = LOWER.read_text()
m = re.search(r"pub struct Abi \{(.*?)\}", src, re.S)
fields = re.findall(r"pub (\w+):", m.group(1)) if m else []
print(f"Abi fields: {fields}")
code_ok = any(re.search(r"builtin|count", f) for f in fields)
cf = CF.read_text(); cf002 = cf[cf.index("CF-002"):cf.index("### Requirement: CF-003")]
ecc = ECC.read_text(); ecc003 = ecc[ecc.index("ECC-003:"):ecc.index("### Requirement", ecc.index("ECC-003:"))]
text_ok = ("count per builtin" not in cf002) or re.search(r"Where the build is not the e2e prototype", cf002) is not None \
          or re.search(r"(builtin count|count per builtin|builtin_counts)", ecc003.splitlines()[0]) is not None
print(f"CF-002 still asks the count per builtin: {'count per builtin' in cf002}; code side fixed: {code_ok}; text side amended: {bool(text_ok)}")
if code_ok or text_ok:
    print("PASS"); sys.exit(0)
print("FAIL: CF-002 abi.json needs the count per builtin; Mycroft's Abi has none"); sys.exit(1)
