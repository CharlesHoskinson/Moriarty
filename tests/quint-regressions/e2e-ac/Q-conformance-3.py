#!/usr/bin/env python3
"""Q-conformance-3 regression (ECS-002, text against code).
Pinned: Moriarty worktree mycroft-factory @ 6c93d6421 (mycroft/tests/compact-suite, ref_vectors.py:47
skips every call whose outcome is not "ok"); spec: worktree sherlock-e2e-spec @ c154662c4,
openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-compact-suite/spec.md ECS-002.
ECS-002 says every suite call gets a preimage and one `mycroft-native-ref/1` record, and a rejected call's
record holds the rejection. PASS when either (a) every natively rejected call has a ref record of schema
mycroft-native-ref/1 (and a preimage), or (b) ECS-002 is amended so rejected calls need neither.
Reads only <label>.json outcome fields and file names (no vector/secret files). Exit 0 = PASS, 1 = FAIL."""
import json, pathlib, re, sys
H = pathlib.Path.home()
SUITE = H / "Projects/Moriarty-wt/mycroft-factory/mycroft/tests/compact-suite/native"
SPEC = H / "Projects/Moriarty-wt/sherlock-e2e-spec/openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-compact-suite/spec.md"
rej, no_ref, no_pre = 0, [], []
for p in sorted(SUITE.glob("*/transcripts/*.json")):
    if p.name.endswith(".ref.json") or re.search(r"\.\d+\.", p.name):
        continue
    try:
        d = json.loads(p.read_text())
    except Exception:
        continue
    if not isinstance(d, dict) or "outcome" not in d or d["outcome"] == "ok":
        continue
    rej += 1
    lab = p.name[:-5]
    refs = [r for r in p.parent.glob(lab + ".*ref.json")]
    if not any(json.loads(r.read_text()).get("schema") == "mycroft-native-ref/1" for r in refs):
        no_ref.append(f"{p.parent.parent.name}/{lab}")
    if not list(p.parent.glob(lab + ".*preimage.bin")):
        no_pre.append(f"{p.parent.parent.name}/{lab}")
print(f"rejected calls: {rej}; without a mycroft-native-ref/1 record: {len(no_ref)}; without a preimage: {len(no_pre)}")
code_ok = rej > 0 and not no_ref and not no_pre
txt = SPEC.read_text()
blk = txt[txt.index("ECS-002:"):txt.index("### Requirement", txt.index("ECS-002:"))]
ears = blk.splitlines()[0]
text_ok = bool(re.search(r"(accepted|ok|not rejected|that the native circuit accepts)", ears)) and "reference record holds the rejection" not in blk
print(f"ECS-002 EARS: {ears[:160]}...")
print(f"code side fixed: {code_ok}; text side amended: {text_ok}")
if code_ok or text_ok:
    print("PASS"); sys.exit(0)
print("FAIL: ECS-002 asks a preimage and a mycroft-native-ref/1 record for every call; the 62 rejected calls have neither"); sys.exit(1)
