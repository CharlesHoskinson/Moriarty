# A5 Task2 original-suite sequencing amendment

Status: proposed only. Root review/adoption and explicit dispatch are required. No Python collection or regression has been run for this amendment, and the unused wrapper has not been edited.

## Scope and EARS requirements

- WHEN Task2 validates the **original full Python suite**, the runner SHALL execute every test in the 23 source-pinned historical files below, without test-selection filters, ignores, skips, or deselections.
- WHEN collecting that inventory, the runner SHALL retain the fresh complete node-ID output and require 441 unique collected IDs. The admitted historical receipt records `441 passed in 20.24s`, not historical node IDs; no historical node-ID identity is claimed.
- WHEN executing the inventory, the runner SHALL require exit0, 441 passing JUnit testcases, no failures/errors/skips, and exact set equality between runtime names and the fresh collected IDs. It SHALL retain original stdout/stderr, JUnit bytes/digest and both name inventories.
- IF any source/config/input/runtime pin moves, a collection/result count differs, or a command fails, the runner SHALL stop that sequence and retain the original failure. It SHALL NOT drop a test to pass.
- WHILE validating this historical checkpoint, the runner SHALL retain the old2673-file fixture selection at **current original bytes**, all current external fixture bytes, the original cases.json and all14 raw ITFs, and the unchanged shared runtime snapshot. It SHALL NOT auto-select newly added A4 sources or packages that the original tests do not use.
- WHEN validating independent Core correspondence, the runner SHALL retain the unchanged complete-inventory command, require exit0 and53 cases, and SHALL NOT use `--allow-subset`.
- BEFORE final A4/A5 admission, the expanded A4 tests, all78 native exports/1557events, independent package replay and final unrestricted `python -m pytest -q` SHALL still pass with complete fresh source/input closures. This amendment neither admits those gates nor makes them optional.

## Evidence and exact inventory

Historical commit: `28d35d82d9f404844d05e0e6bd0ce2dc86b8c311`. All23 current test files compare byte-for-byte to that commit. The literal `ORIGINAL_TESTS` dictionary in the replacement below is the complete path/SHA256 inventory, ordered as the original two testpaths (tests first, compact experiment last). The unchanged pyproject hash is also pinned.

New A4 files, outside this explicitly historical checkpoint but still mandatory later: `tests/test_a4_json_stream.py`, `tests/test_s02_candidate_a_integrated.py`, and `tests/test_s02_candidate_a_integrated_export.py`. Their current definition counts are11/27/6, not expanded collection counts. The actual-package test at integrated.py:623 requires evidence/s02-candidate-a-completion/a4/{cases,admission,inventory}.json and all78 native shards, with no skip. No code in these files is changed.

Old fixture selection: `.superpowers/sdd/a2-a3-final-regressions/snapshot.json`, SHA256 `eae070bcb504fff68d4677c972aea862f7085d3a80adeb2438070e72b9735dad`. Reuse its2673 paths, not all new tracked receipt/runtime artifacts. Capture fresh bytes, including the78 external paths; retain the manifest itself. Root must grant a finite original-fixture freeze before preparation. New A4 code need not be frozen for this historical checkpoint.

## Exact command amendment

Cwd remains `/home/charl/Moriarty/.worktrees/s01-audit-start`. The new collection is read-only but imports tests and is therefore recorded like a test. `-c` binds the unchanged configuration; `--junitxml` is reporting-only. No selection filter is used.

```text
/home/charl/Moriarty/.venv/bin/python -m pytest -q -c pyproject.toml --collect-only tests/test_compact_lowering.py tests/test_core_semantics.py tests/test_deep_research_prompt.py tests/test_defiformal_taxonomy_crosswalk.py tests/test_defiformal_taxonomy_metrics.py tests/test_goal_completion_matrix.py tests/test_intent_verifier.py tests/test_marlowe_graph_analysis.py tests/test_marlowe_graph_evidence.py tests/test_moriarty_decision_graph.py tests/test_openspec_work_packages.py tests/test_s01_intent_evidence.py tests/test_s01_openspec.py tests/test_s01_registries.py tests/test_s02_candidate_a_correspondence.py tests/test_s02_candidate_a_export.py tests/test_s02_candidate_a_installment_reference.py tests/test_s02_candidate_a_reference_vectors.py tests/test_s02_contract.py tests/test_semantics_intent_prompt.py tests/test_swap_bounds.py tests/test_translation_certificate.py experiments/moriarty-compact-escrow/test_contract.py
/home/charl/Moriarty/.venv/bin/python -m pytest -q -c pyproject.toml --junitxml=/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-factoring-receipts/task2-original-python-regression/junit.xml tests/test_compact_lowering.py tests/test_core_semantics.py tests/test_deep_research_prompt.py tests/test_defiformal_taxonomy_crosswalk.py tests/test_defiformal_taxonomy_metrics.py tests/test_goal_completion_matrix.py tests/test_intent_verifier.py tests/test_marlowe_graph_analysis.py tests/test_marlowe_graph_evidence.py tests/test_moriarty_decision_graph.py tests/test_openspec_work_packages.py tests/test_s01_intent_evidence.py tests/test_s01_openspec.py tests/test_s01_registries.py tests/test_s02_candidate_a_correspondence.py tests/test_s02_candidate_a_export.py tests/test_s02_candidate_a_installment_reference.py tests/test_s02_candidate_a_reference_vectors.py tests/test_s02_contract.py tests/test_semantics_intent_prompt.py tests/test_swap_bounds.py tests/test_translation_certificate.py experiments/moriarty-compact-escrow/test_contract.py
/home/charl/Moriarty/.venv/bin/python scripts/check_s02_candidate_a_correspondence.py --cases /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-export-stages/final/cases.json --input-root /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-export-stages/final/inputs --report /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-factoring-receipts/core-comparison/report.json
```

This explicitly amends the original discovery argv only for Task2's historical-suite checkpoint. The literal unrestricted discovery command remains a later mandatory gate. Runtime collection must reconfirm441; source inspection alone is not a new passing test result.

## Exact unused-wrapper change

After adoption, replace only `.superpowers/sdd/a5-factoring-receipts/capture-task2-python.py` (current SHA256 `444313759f0e1da8bb50cc61392ff0a82eb6207f0cd19a8501779b24a9fa8df6`) with the complete code below, using apply_patch. No shared helper, original dispatch, test source, frozen Quint source or main/evidence file changes. The changes are: literal original inventory/config pins; no moving-A4 additions; recorded collect stage; JUnit/name verification; preparation runtime checks; explicit historical scope.

```python
"""Fresh original fixture bytes; no runtime duplication or helper edits."""
import hashlib
import io
import json
import sys
import tarfile
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT))
from scripts.run_s02_candidate_a_factoring_pilot import record, RECEIPTS, verify_runtime

BASE = RECEIPTS / "task2-python-inputs"
OLD = ROOT / ".superpowers/sdd/a2-a3-final-regressions/snapshot.json"
FINAL = ROOT / ".superpowers/sdd/candidate-a-export-stages/final"
TASK_DISPATCH = RECEIPTS / "task2-dispatch.json"
PYTHON = "/home/charl/Moriarty/.venv/bin/python"
BOOTSTRAP = "900bb2051225b4a3d99bf422c3b2e5e386e3e7bc"
ORIGINAL_COMMIT = "28d35d82d9f404844d05e0e6bd0ce2dc86b8c311"
ORIGINAL_TESTS = {
    "tests/test_compact_lowering.py": "97da10680f4888d319d1c6235ddc744cb5c697460619c3da951cf525b802c075",
    "tests/test_core_semantics.py": "6a462fe2bb2793f5a187f8f2a3251af0abaad08b7ded067b96e6941b09d994e7",
    "tests/test_deep_research_prompt.py": "8b87d61fdc00e27ebb56969fffd646d16d13fefd3325fb9a46bceb2646ecb142",
    "tests/test_defiformal_taxonomy_crosswalk.py": "79403e5af3d4b619656a3044ed9b2881039f8d970fcf843728dcd4917729022b",
    "tests/test_defiformal_taxonomy_metrics.py": "604d85535b76df6dfc30e861481f103e0ac85d7a252839a644907f9474c9d6b9",
    "tests/test_goal_completion_matrix.py": "8d98bf7164316869abeecbe394d3846d07507e2d35dc23e6fb241e738c7f1845",
    "tests/test_intent_verifier.py": "33782e6da928ec026e35131b23b69e8956188e9e878ca6688ac97d9d74f3ff92",
    "tests/test_marlowe_graph_analysis.py": "affde2122e25ef0ab1375dd9da963f9773e5d3716d85ad3b279463538a2273c4",
    "tests/test_marlowe_graph_evidence.py": "55cdff9a7dc3bc37bd6bc19ed6d49a6673d36d0bd618a39856f16adbf414c4eb",
    "tests/test_moriarty_decision_graph.py": "836352e4a558ef8aeec6384cef0fa2eb0d259ca7d6cef0f4845e92b7aa6e2a79",
    "tests/test_openspec_work_packages.py": "682a84e977384e4ddfaff1360cb5f3923ef1f6f595d31d3bf5233910a50b6474",
    "tests/test_s01_intent_evidence.py": "14c09bca8f882bcd245ad6800f01e16273751027ced8b32fc753bf2dc942e40f",
    "tests/test_s01_openspec.py": "fbe1a140e14e59eb597a94f1122821545b2dba68f1cb84386d8eae5fc17f779f",
    "tests/test_s01_registries.py": "7edef98a53ae0b9798cfe1e5b128d1cd9bd131108e88612f9bb9c7b711827f40",
    "tests/test_s02_candidate_a_correspondence.py": "eb6bdf21259b2f567b405469f57fe291ee71f98b850ffcac317f929196f20fd2",
    "tests/test_s02_candidate_a_export.py": "35c2d87726f0191127a73d1604c2691e1a1a9626736c219898aacab3b1328757",
    "tests/test_s02_candidate_a_installment_reference.py": "9d91935a09382e68dd6ccf24b565ca243d4b0524acf0a4af79999ee830de2633",
    "tests/test_s02_candidate_a_reference_vectors.py": "6d04f8373dc8cbee53eb5472b1f652158b7c5f5d45f555310874a4b83b1c3ef3",
    "tests/test_s02_contract.py": "854e261ff6e4104e737c8f78785c98d82bc4e687ef75fcbde2888a58512d706b",
    "tests/test_semantics_intent_prompt.py": "be5b12824e347c5ae1d8da5e776f3e7f161071248af8221b164dbba0daa643b2",
    "tests/test_swap_bounds.py": "72ee270c37a11973cebc1e439518609f87c7f61b43cfc9ef3f041927015d97fa",
    "tests/test_translation_certificate.py": "209aa4354a1c6a5d62deb5d29a28a7c2ae79ef5cd2f2857d5443ec0a05e193a7",
    "experiments/moriarty-compact-escrow/test_contract.py": "1ad89d71de1b15dc9759b340bca12fef072899862d444a9f9af98799e84a0826"
}
CONFIG_SHA = "2fdbd39d368677002932ede2ba2b4d736c3f2fb6a53b169b8810faea02b618d2"
EXPECTED_COUNT = 441

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def write_json(path, value):
    with path.open("x") as stream:
        json.dump(value, stream, indent=2)
        stream.write("\n")

def original_pins():
    assert len(ORIGINAL_TESTS) == 23
    assert all(sha(ROOT/name) == digest for name,digest in ORIGINAL_TESTS.items())
    assert sha(ROOT/"pyproject.toml") == CONFIG_SHA
    for parent in (ROOT, ROOT/"tests", ROOT/"experiments", ROOT/"experiments/moriarty-compact-escrow"):
        assert not (parent/"conftest.py").exists(), "New collection fixture requires review"

def additions():
    paths = {ROOT/name for name in ORIGINAL_TESTS}
    paths.update((FINAL/"inputs").glob("*.itf.json"))
    paths.update((FINAL/"cases.json", Path(__file__).resolve(), TASK_DISPATCH, OLD, ROOT/"pyproject.toml"))
    return paths

def check(manifest):
    original_pins()
    assert sha(BASE/"source-inputs.tar.gz") == manifest["archiveSha256"]
    assert sorted(str(p) for p in additions()) == manifest["additionPaths"]
    assert all(sha(e["path"]) == e["sha256"] for e in manifest["files"])

mode = sys.argv[1]
assert sha(OLD) == "eae070bcb504fff68d4677c972aea862f7085d3a80adeb2438070e72b9735dad"
assert sys.dont_write_bytecode and sys.pycache_prefix, "Launch with -B and a fresh -X pycache_prefix"
if mode == "prepare":
    original_pins()
    runtime_before = verify_runtime(BOOTSTRAP)
    BASE.mkdir(exist_ok=False)
    old = json.loads(OLD.read_text())
    paths = {Path(e["path"]) for e in old["sourceAndInputClosure"]} | additions()
    assert len(list((FINAL/"inputs").glob("*.itf.json"))) == 14
    assert len(json.loads((FINAL/"cases.json").read_text())["cases"]) == 53
    entries = []
    with tarfile.open(BASE/"source-inputs.tar.gz", "x:gz") as archive:
        for path in sorted(paths):
            raw = path.read_bytes()
            name = str(path).lstrip("/")
            info = tarfile.TarInfo(name)
            info.size = len(raw)
            archive.addfile(info, io.BytesIO(raw))
            entries.append({"path":str(path), "archivePath":name, "bytes":len(raw),
                "sha256":hashlib.sha256(raw).hexdigest()})
    with tarfile.open(BASE/"source-inputs.tar.gz", "r:gz") as archive:
        members = archive.getmembers()
        expected = {e["archivePath"]:e for e in entries}
        assert len(members) == len(expected)
        assert all(m.isfile() and m.name in expected for m in members)
        for member in members:
            assert hashlib.sha256(archive.extractfile(member).read()).hexdigest() == expected[member.name]["sha256"]
    manifest = {"selectionSource":str(OLD), "selectionSourceSha256":sha(OLD),
        "oldSelectionCount":len(old["sourceAndInputClosure"]),
        "additionPaths":sorted(str(p) for p in additions()), "files":entries,
        "archiveSha256":sha(BASE/"source-inputs.tar.gz"),
        "actualTask2Base":"d824fa3381ecda846be7e8dd027732f1a7f62e91",
        "parentArgv":sys.orig_argv,"parentCachePrefix":sys.pycache_prefix,
        "parentBytecodeDisabled":sys.dont_write_bytecode,
        "originalCommit":ORIGINAL_COMMIT,"originalTests":ORIGINAL_TESTS,"expectedCount":EXPECTED_COUNT,
        "runtimeSnapshot":runtime_before,
        "scope":"Complete historical 23-file suite; old2673 fixture selection at current bytes plus Core53 inputs; new A4 and final unrestricted suite remain mandatory later"}
    check(manifest)
    assert verify_runtime(BOOTSTRAP) == runtime_before
    write_json(BASE/"manifest.json", manifest)
    print(json.dumps({"files":len(entries), "archiveSha256":manifest["archiveSha256"]}))
elif mode in ("collect", "python", "core-comparison"):
    manifest = json.loads((BASE/"manifest.json").read_text())
    check(manifest)
    stage = {"collect":"task2-original-python-collect", "python":"task2-original-python-regression",
        "core-comparison":"core-comparison"}[mode]
    extra = [Path(__file__).resolve(),TASK_DISPATCH,BASE/"manifest.json",BASE/"source-inputs.tar.gz"]
    pytest_argv = [PYTHON,"-m","pytest","-q","-c","pyproject.toml"]
    if mode == "collect":
        argv = [*pytest_argv,"--collect-only",*ORIGINAL_TESTS]
    elif mode == "python":
        collected = RECEIPTS/"task2-original-python-collect"
        prior = json.loads((collected/"result.json").read_text())
        assert prior["exitCode"] == 0 and prior["sourceAndToolsUnchanged"] and prior["runtimeUnchanged"]
        ids = json.loads((collected/"collected-nodeids.json").read_text())
        assert len(ids) == len(set(ids)) == EXPECTED_COUNT
        extra += [collected/name for name in ("input.json","result.json","stdout.txt","stderr.txt",
            "collected-nodeids.json","fixture-checks.json","inventory-result.json")]
        argv = [*pytest_argv,"--junitxml="+str(RECEIPTS/stage/"junit.xml"),*ORIGINAL_TESTS]
    else:
        argv = [PYTHON,"scripts/check_s02_candidate_a_correspondence.py","--cases",str(FINAL/"cases.json"),
            "--input-root",str(FINAL/"inputs"),"--report",str(RECEIPTS/stage/"report.json")]
    code = record(stage,argv,1200,4096,
        "historical441Collection" if mode == "collect" else "originalFull441Suite" if mode == "python" else "independent53CoreRecords",0,
        extra=tuple(extra),before_dispatch_base=BOOTSTRAP,
        domain="Task2 actual base d824fa3381ecda846be7e8dd027732f1a7f62e91; helper field is immutable runtimeBootstrapBase; original23-file suite only; expanded A4 and final unrestricted suite not admitted")
    unchanged = True
    try:
        check(manifest)
    except (AssertionError, OSError):
        unchanged = False
    write_json(RECEIPTS/stage/"fixture-checks.json", {"unchangedBefore":True,
        "unchangedAfter":unchanged,"manifestSha256":sha(BASE/"manifest.json"),"commandExitCode":code,
        "parentArgv":sys.orig_argv,"parentCachePrefix":sys.pycache_prefix,
        "parentBytecodeDisabled":sys.dont_write_bytecode,"originalCaseCount":53})
    assert unchanged, "Source/fixture movement invalidates command"
    if code == 0 and mode == "collect":
        stdout = (RECEIPTS/stage/"stdout.txt").read_text()
        ids = [line for line in stdout.splitlines() if any(line.startswith(p+"::") for p in ORIGINAL_TESTS)]
        assert len(ids) == len(set(ids)) == EXPECTED_COUNT
        write_json(RECEIPTS/stage/"collected-nodeids.json", ids)
        write_json(RECEIPTS/stage/"inventory-result.json", {"count":len(ids),"nodeidsSha256":sha(RECEIPTS/stage/"collected-nodeids.json"),
            "provenance":"Fresh runtime collection; no historical node-ID claim"})
    elif code == 0 and mode == "python":
        document = ET.parse(RECEIPTS/stage/"junit.xml")
        cases = document.findall(".//testcase")
        actual = []
        for case in cases:
            assert all(case.find(tag) is None for tag in ("failure","error","skipped")), "Non-passing testcase"
            classname, name = case.attrib["classname"], case.attrib["name"]
            matches = [p for p in ORIGINAL_TESTS if classname == p[:-3].replace("/",".")
                or classname.startswith(p[:-3].replace("/",".")+".")]
            assert len(matches) == 1, (classname,name)
            path = matches[0]
            suffix = classname[len(path[:-3].replace("/",".")):].lstrip(".")
            actual.append(path+"::"+(suffix.replace(".","::")+"::" if suffix else "")+name)
        assert len(actual) == len(set(actual)) == EXPECTED_COUNT and set(actual) == set(ids)
        write_json(RECEIPTS/stage/"inventory-result.json", {"count":len(actual),"runtimeNodeids":actual,
            "matchesFreshCollection":True,"junitSha256":sha(RECEIPTS/stage/"junit.xml"),
            "scope":"Complete original suite; expanded A4/final unrestricted suite still mandatory"})
    elif code == 0:
        report = json.loads((RECEIPTS/stage/"report.json").read_text())
        assert report == {"ok":True,"differences":[],"scope":"complete-inventory"}
    raise SystemExit(code)
else:
    raise SystemExit("Unsupported mode; no implicit retries")
```

## Dispatch sequence after separate authorization

Run prepare, collect, python, then core-comparison. Retain each process to terminal, never overwrite a stage, and use a unique initially absent cache prefix per parent process. Example exact preparation command:

```text
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/task2-original-prepare-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python.py prepare
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/task2-original-collect-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python.py collect
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/task2-original-run-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python.py python
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/task2-original-core-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python.py core-comparison
```

The unchanged recorder supplies per-command controlled environments, fresh child cache prefixes,1200-second limits, source/runtime before-after checks, and raw archives. Its generic pilot/deadlock metadata remains untouched; these actual argv are collection/regression/Core comparison, not a pilot or model-checking claim. Actual Task2 base remains d824fa3381ecda846be7e8dd027732f1a7f62e91; the helper's bootstrap base remains900bb2051225b4a3d99bf422c3b2e5e386e3e7bc, separately pinned through task2-dispatch.json. Parent argv/cache/bytecode state is retained.

Root independently verifies this inventory, reads the replacement and authorizes execution. Current equivalence and factored corpus commands continue unchanged. No Task3 pilot or final A5 admission follows from this amendment alone.
