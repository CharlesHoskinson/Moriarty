#!/usr/bin/env python3
"""Print an apply_patch input. Never write or overwrite model files."""
import argparse
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "specs/quint/s02"
TARGET = SOURCE / "factored_verification"
PINNED_ORIGINALS = {
    "candidate_a_authority_adapter": "3f3b093f4c718dd08eda38e610de700d0a24138beb82fd7b2b12dcf9d300bda8",
    "candidate_a_authority_adapter_harness": "d7b54410ec0ed2ce8de0b63e5f13dca685f91298730e33342759bf52aad9dd23",
    "candidate_a_authority_adapter_test": "029b2dfd24380f385c961a3145d52528339ee78b9a4d63b9009e7770f2077cf8",
    "candidate_a_authority_boundary": "95ddf75ef32b432acaec5384c89826d0dc12245b80f78fd8a60ff78d0da8164b",
    "candidate_a_authority_boundary_fixtures": "32e68b44ac77f01df74e200cde3c23150b12e40ff772167dc5fa3c79464b52ad",
    "candidate_a_authority_boundary_harness": "26d13e6f34b4e16158dc92d80ccbdf0075d9291121deb1a88aa76de817c39652",
    "candidate_a_authority_boundary_test": "a5c7645697166d8979125f8f46e81ccb6938d589c69165c01bd540b92a9585cd",
    "candidate_a_authority_installment": "f12d91938098d48a313baf7cb5218f84b6bd840da8c518d54f195e0f7fa1e4cd",
    "candidate_a_authority_installment_fixtures": "22d975d6d615e1f8e80c453115ded79fa68f783880a036f73470814221bf1a92",
    "candidate_a_authority_installment_harness": "9a49b2a76d0c9e27a0d06b942b4f6a3338bf54a1e005cbba6d6388e91dcf82ca",
    "candidate_a_authority_installment_test": "b1d9c21c5285105b382525df8df2ebaa41dbf10979f400d975b40500c3f7c3d1",
    "candidate_a_authority_swap": "294633d4213d44075df76085f67b9bd24fab23d07bc863fff8b4f22e79d10024",
    "candidate_a_authority_swap_fixtures": "fb407555584e167ae0fddc3e59fbf6d64ecd79a00d2ad03d02f159da8429ed15",
    "candidate_a_authority_swap_harness": "bb8991e439153af69f3c8ec52df1771786a0ce0d5360f975189958b978555e29",
    "candidate_a_authority_swap_test": "5aed1719310ef472f8fdd5a19e8cc5188a47c7c136f7fc3158bb5a12943636c5",
    "candidate_a_boundary_test": "1bb9ff76c2cf7f6e73e72f3c3076b1ff490e1ed0e9cc90c0bb71d7e7c01ca623",
    "candidate_a_cases": "03da5af65cd4af2fa2d7e23f87ea43d9212f8ac9edef5dfe86e02ee5c79705cb",
    "candidate_a_cases_test": "b1a6165ba7a69b0557d628594bcb754ada8de0b18da1bb15edd10928fc80e239",
    "candidate_a_core": "e759d4c13032d83a4d839e08bef0fcf8272943792f73bb3ab7910357b0f28dec",
    "candidate_a_core_test": "91482debc44658a6a55eb6f74b63253f715a9372ed19228e43b16b1261c12375",
    "candidate_a_harness": "0651108d66e40e0295f7ef568665257bee59b93300dae1f7dd6aad684ce8429c",
    "candidate_a_installment_harness": "5912655f3f8b35e7f85602099851fea9bdddf6a28493964ddc834e53b098194e",
    "candidate_a_installment_test": "6ce3ed1077f46c5aabbaf4449548adb39b3916935d4021acf76af1ebd2a80f8f",
    "candidate_a_programs": "bf814bce2924cafac5836a171b52a4c9bdba43e083fe7129bcdeadd810c42d5e",
    "candidate_a_projection": "2eb66db8010a3d47ddba8f99d647326d2eb7bf3c9da5177c7dd2dcd88145452c",
    "candidate_a_projection_test": "3a6eac6f132b86003d0973c0f6fd26af938b56a082a7a648de597c4f5a2c96a2",
    "candidate_a_test": "7906400286288d3419ea22b48d424c68f21b5842fa8349ac156df26286077d25",
    "candidate_a_types": "40901738fd1749527761b624a84253da94bbc24c209d408dc2fd4e895daaae0b"
}
NAMES = tuple(n for n in PINNED_ORIGINALS if n not in ("candidate_a_types", "candidate_a_programs"))
J_IMPORT = '  import candidate_a_joint_f as J from "./candidate_a_joint_f"\n'
WRAPPERS = {
    "candidate_a_core": (
        "computeTransaction", None,
        "  pure def computeTransaction(program: AProgram, before: AState, supplied: AInput, now: ATime): ATransactionEvaluation =\n"
        "    J::computeTransactionF(program, before, supplied, now)\n"),
    "candidate_a_projection": (
        "extractCommittedEffects", None,
        "  pure def extractCommittedEffects(program: AProgram, before: AState, input: AInput, now: ATime, result: ATransactionResult): AEffectExtraction =\n"
        "    J::extractCommittedEffectsF(program, before, input, now, result)\n"),
    "candidate_a_authority_adapter": (
        "adaptAuthorityA", "authorityObservationMatchesA",
        "  pure def adaptAuthorityA(request: AAuthorityRequest, op: Operation, plan: AResolvedPlan, display: DisplayProjection): AAuthorityAdaptation =\n"
        "    J::adaptAuthorityF(request, op, plan, display)\n"),
}

def digest(data):
    return hashlib.sha256(data).hexdigest()

def replace_body(text, first, following, replacement):
    start = text.index("  pure def " + first + "(")
    end = text.index("  pure def " + following + "(", start) if following else text.rfind("}")
    return text[:start] + replacement + text[end:]

def boundary_bindings(text):
    # Only declaration names change. Bodies, field labels, strings and selectors are untouched.
    pattern = re.compile(r"(?m)^  pure def (\w+)\(([^)]*)\): ([^=]+?)=")
    matches = list(pattern.finditer(text))
    for m in reversed(matches):
        name = m.group(1)
        internal = "a5_body_" + name
        assert internal not in text
        params = []
        for param in m.group(2).split(","):
            param_name, typ = param.strip().split(":", 1)
            assert re.fullmatch(r"[A-Za-z]\w*", param_name)
            assert typ.strip()
            params.append(param_name)
        bindings = "".join("    val shared_" + p + " = " + p + "\n" for p in params)
        args = ", ".join("shared_" + p for p in params)
        wrapper = m.group(0) + " {\n" + bindings + "    " + internal + "(" + args + ")\n  }\n"
        renamed = m.group(0).replace("pure def " + name + "(", "pure def " + internal + "(", 1)
        text = text[:m.start()] + wrapper + renamed + text[m.end():]
    return text

def check_boundary_roundtrip(original, wrapped):
    wrapper = re.compile(
        r"(?m)^  pure def (\w+)\([^)]*\): [^=]+?= \{\n"
        r"(?:    val shared_\w+ = \w+\n)+"
        r"    a5_body_\1\([^\n]*\)\n  }\n")
    restored = wrapper.sub("", wrapped)
    restored = restored.replace("pure def a5_body_", "pure def ")
    assert restored == original, "Wrapper changed an original body, selector, label or literal"

def derived(name):
    text = (SOURCE / (name + ".qnt")).read_text()
    if name in WRAPPERS:
        first, following, replacement = WRAPPERS[name]
        text = replace_body(text, first, following, replacement)
    if name == "candidate_a_authority_boundary":
        original = text
        text = boundary_bindings(text)
        check_boundary_roundtrip(original, text)
    text = re.sub(r"\bmodule\s+" + name + r"\b", "module " + name + "_f", text, count=1)
    def imported(m):
        module, rest, path = m.groups()
        assert path == module, (module, path)
        if module in NAMES:
            return "import " + module + "_f" + rest + ' from "./' + module + '_f"'
        return "import " + module + rest + ' from "../' + path + '"'
    text = re.sub(r'import (\w+)([^\n]*?) from "\./(\w+)"', imported, text)
    if name in WRAPPERS:
        pos = text.index("\n") + 1
        text = text[:pos] + J_IMPORT + text[pos:]
    return text

def patch_add(path, content):
    return "*** Add File: " + str(path.relative_to(ROOT)) + "\n" + "".join("+" + line + "\n" for line in content.splitlines())

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    assert len(NAMES) == 26, NAMES
    for name, wanted in PINNED_ORIGINALS.items():
        assert digest((SOURCE / (name + ".qnt")).read_bytes()) == wanted, "Original pin mismatch: " + name
    records = [{"path": str((SOURCE / (n + ".qnt")).relative_to(ROOT)),
                "sha256": digest((SOURCE / (n + ".qnt")).read_bytes()),
                "runs": re.findall(r"(?m)^\s*run\s+(\w+)\s*=", (SOURCE / (n + ".qnt")).read_text())}
               for n in NAMES]
    generated = {TARGET / (name + "_f.qnt"): derived(name) for name in NAMES}
    generated[TARGET / "derivation.json"] = json.dumps({"originals": records,
        "rule": "module/import renaming; three joint delegates; boundary identity let bindings only"}, indent=2) + "\n"
    if args.check:
        for path, content in generated.items():
            assert path.read_text() == content, str(path)
        return
    assert all(not p.exists() for p in generated), "Refuse to overwrite a derivation"
    print("*** Begin Patch")
    for path, content in generated.items():
        print(patch_add(path, content), end="")
    print("*** End Patch")

if __name__ == "__main__":
    main()
