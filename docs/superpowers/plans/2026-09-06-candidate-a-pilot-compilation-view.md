# Candidate A Pilot Compilation View Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans after root adoption and explicit dispatch. Steps use checkbox syntax. This document is specified-only; it authorizes no execution by its presence.

**Goal:** Test whether the admitted single-name alias-visibility correction permits the original and factored funding pilots to compile, preserving their complete semantics and provenance before a separate checker gate.

**Architecture:** Copy exactly the 24-file transitive Quint closure of the original pilot, factored pilot and prefix-test roots into one fresh ignored view, preserving repository-relative hierarchy. Add one explicit `consumption.AuthorityKey` import to each adapter copy and change no other bytes. Reuse the immutable `record()` function with the entire view in `extra`; no helper globals change and no general runner is introduced.

**Tech Stack:** Pinned Quint 0.32.0, actual Node 24.18.1, admitted Rust evaluator, CPython 3.13.14 with `-B` and fresh parent cache prefixes; existing shared runtime/Python manifests and archives.

## Global Constraints

- Planning base: `a09d34a73c0872035e8ec5fb27983aea69a28a8b`. Root binds the actual full current dispatch commit separately from historical runtime bootstrap `900bb2051225b4a3d99bf422c3b2e5e386e3e7bc`; do not rewrite the shared dispatch.
- All actual model files, 30 Task2 frozen files, derivation pins, original failed compile and admitted control receipts remain immutable. Only new ignored files under `.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/` and the nine designated fresh parent caches may be created during execution. Under `.superpowers/sdd/`, the six native-stage cache names are `a5-pilot-view-typecheck-parent-cache`, `a5-pilot-view-prefix-test-parent-cache`, `a5-pilot-view-sample-original-parent-cache`, `a5-pilot-view-sample-factored-parent-cache`, `a5-pilot-view-compile-original-parent-cache`, and `a5-pilot-view-compile-factored-parent-cache`; the three preparation/freeze/handoff cache names are `a5-pilot-view-prepare-parent-cache`, `a5-pilot-view-dispatch-parent-cache`, and `a5-pilot-view-handoff-parent-cache`.
- Preserve module names, relative import destinations, generic bodies, complete state, actual Core evaluation, full histories, every guard/action/invariant and the existing four routes. Add only the two explicit import lines specified below. No wildcard addition, namespace rewrite, local alias duplication, new stutter or changed domain.
- Existing helper SHA256 remains `816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2`. Reuse its pinned 4,758-file runtime and 3,329-file Python archives; no installation, runtime/helper modification or replacement environment archive.
- Preserve original Task3 gate order: recursive typecheck; one quantified test covering all 22 prefix pairs; original and factored 100-sample/eight-witness runs; original and factored compilation. Typecheck/test/sample children retain 1200 seconds; compilation children retain 900 seconds. Every child retains 4096 MiB Node/JVM settings. Runtime preparation/checking is outside the child's wall-clock interval.
- Run each command once and separately to actual terminal. Any pin failure, unexpected exit/diagnostic, missing witness, timeout, invalid JSON or non-smaller factored input stops the series. Preserve all original evidence; no automatic retry, changed predicate, resource escalation or additional alias correction.
- No solver/checker invocation is part of this plan. Root reviews both generated inputs, structural predicates and measured size before any separate checker dispatch. Successful compilation is neither semantic verification nor H1 confirmation; failed translation leaves H1 unresolved.
- Root owns adoption, independent audit, tracked evidence integration and commits. The implementer never changes an existing receipt or integrates its own evidence.

## Evidence and exact changed hypothesis

Repository observation: `evidence/s02-candidate-a-completion/a5/pilot-compile-stop/` preserves original compiler child exit 1, QNT404 `AuthorityKey`, before flattening/Apalache. `evidence/s02-candidate-a-completion/a5/alias-visibility-control/README.md` and `validation.json` admit the four-command toy control: the generic consumer typechecks, fails compilation for its unqualified dependency, then compiles after one direct single-name import. This supports the proposed mechanism without establishing that the full pilot will succeed.

Repository observation: the original Task3 plan is `docs/superpowers/plans/2026-09-05-candidate-a-factored-verification.md`, SHA256 `fe96c2343c369e9170952c04cfb847fc826d706a8f0ad6818efc283f37615563`. Its preserved `task3-pilot-typecheck`, `task3-pilot-prefix-test`, `sample-original-pilotSafety`, `sample-factored-pilotSafety`, and `compile-original-pilotSafety` receipts provide the exact command/budget baseline. The 30-file Task2 freeze is `.superpowers/sdd/a5-factoring-receipts/task2-frozen-source.json`; `derivation.json` separately records 26 transformed original modules. These counts describe different sets.

Inference/hypothesis: making `AuthorityKey` directly visible in both adapter copies may remove this frontend name-resolution failure without changing semantic definitions. A different missing alias or further failure falsifies the adequacy of this particular remedy and ends this series; no broader patch is implicit.

## File structure and review units

All execution paths below are relative to `/home/charl/Moriarty/.worktrees/s01-audit-start`.

| New path under `.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/` | Responsibility |
| --- | --- |
| `prepare.py` | One-shot exact 24-file copying, two import insertions, deterministic paired manifest and read-only validation |
| `record.py` | Six fixed stage choices calling unchanged `record()`; acceptance and preserved predecessor closure |
| `view/specs/quint/s02/**/*.qnt` | Exactly 24 copied modules, including three pilot/test roots |
| `view-manifest.json` | Original/view hashes, exact added lines/imports, baseline pins and references |
| `dispatch.json` | Exclusive root-created freeze of plan/scripts/view and actual current base |
| Six stage directories | Original helper outputs, validation and actual outer-terminal metadata |
| `handoff.json`, `original-file-hash-index.json` | Terminal result summary, full file/pin index for independent intake |

### Task 1: Prepare and freeze the exact view

**Interfaces:** `prepare.py` exports `ROOT`, `CONTROL`, `VIEW`, `PLAN`, `BOOTSTRAP`, `HELPER_SHA`, `EXPECTED`, `ROOTS`, `sha(path)`, `write(path, value)`, and `validate()` returning the verified manifest. `python prepare.py` only prepares a fresh view. `record.py` imports this local module without triggering preparation.

**CV001/EARS:** When root adopts and dispatches preparation, the implementer SHALL create exactly the two scripts below with `apply_patch`, verify their bytes against this plan, and prepare the absent view once. The preparation SHALL reject any moved original pin or closure other than the enumerated 24 files.

**OpenSpec CV001:** Given the admitted original and factored pilot/test sources, preparation succeeds only if all original pins match and deleting the two exact added import lines reproduces every original byte. Every copied import resolves inside the copied 24-file closure. Existing view or manifest files cause failure, not reuse.

- [ ] Verify the control directory and each named parent cache do not exist before creating the scripts. Read applicable AGENTS.md instructions. Use `apply_patch` to create `prepare.py` with exactly this source:

```python
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
CONTROL = Path(__file__).resolve().parent
VIEW = CONTROL / 'view'
PLAN = ROOT / 'docs/superpowers/plans/2026-09-06-candidate-a-pilot-compilation-view.md'
BASE = ROOT / 'specs/quint/s02'
RECEIPTS = ROOT / '.superpowers/sdd/a5-factoring-receipts'
BOOTSTRAP = '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
HELPER_SHA = '816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2'
OLD_STAGES = ['task3-pilot-typecheck', 'task3-pilot-prefix-test',
              'sample-original-pilotSafety', 'sample-factored-pilotSafety',
              'compile-original-pilotSafety']
ROOTS = ['factored_verification/candidate_a_funding_pilot.qnt',
         'factored_verification/candidate_a_funding_pilot_f.qnt',
         'factored_verification/candidate_a_funding_pilot_test.qnt']
EXPECTED = '''authorization.qnt
candidate_a_authority_adapter.qnt
candidate_a_authority_boundary.qnt
candidate_a_authority_swap.qnt
candidate_a_authority_swap_fixtures.qnt
candidate_a_core.qnt
candidate_a_programs.qnt
candidate_a_projection.qnt
candidate_a_types.qnt
consumption.qnt
effects.qnt
execution.qnt
factored_verification/candidate_a_authority_adapter_f.qnt
factored_verification/candidate_a_authority_boundary_f.qnt
factored_verification/candidate_a_authority_swap_f.qnt
factored_verification/candidate_a_authority_swap_fixtures_f.qnt
factored_verification/candidate_a_core_f.qnt
factored_verification/candidate_a_funding_pilot.qnt
factored_verification/candidate_a_funding_pilot_f.qnt
factored_verification/candidate_a_funding_pilot_test.qnt
factored_verification/candidate_a_joint_f.qnt
factored_verification/candidate_a_projection_f.qnt
observations.qnt
policies.qnt'''.splitlines()
ADDITIONS = {
    'candidate_a_authority_adapter.qnt': '  import consumption.AuthorityKey from "./consumption"\n',
    'factored_verification/candidate_a_authority_adapter_f.qnt': '  import consumption.AuthorityKey from "../consumption"\n',
}

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def write(path, value):
    with Path(path).open('x') as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write('\n')

def imports(path):
    lines = [line for line in path.read_text().splitlines()
             if re.match(r'^\s*import\b', line)]
    result = []
    for line in lines:
        match = re.fullmatch(r'\s*import\s+\w+(?:\.\w+|\.\*|\s+as\s+\w+)?\s+from\s+"(\.[^"\n]+)"\s*', line)
        assert match is not None, (str(path), line)
        result.append(match[1])
    return result

def closure(base):
    pending = [base / name for name in ROOTS]
    seen = set()
    while pending:
        path = pending.pop()
        assert path.is_file() and not path.is_symlink()
        path = path.resolve()
        assert path.is_relative_to(base)
        if path in seen:
            continue
        seen.add(path)
        pending.extend((path.parent / (name + '.qnt')).resolve()
                       for name in imports(path))
    result = sorted(str(path.relative_to(base)) for path in seen)
    assert result == EXPECTED, result
    return result

def changed(name, data):
    if name not in ADDITIONS:
        return data
    line = ADDITIONS[name].encode()
    assert line not in data
    header, body = data.split(b'\n', 1)
    expected_module = Path(name).stem
    assert header == ('module ' + expected_module + ' {').encode()
    return header + b'\n' + line + body

def baseline():
    helper = ROOT / 'scripts/run_s02_candidate_a_factoring_pilot.py'
    assert sha(helper) == HELPER_SHA
    freeze = RECEIPTS / 'task2-frozen-source.json'
    frozen = json.loads(freeze.read_text())['files']
    assert len(frozen) == 30
    pins = {str((ROOT / e['path']).resolve()): e['sha256'] for e in frozen}
    assert len(pins) == 30
    references = [freeze, RECEIPTS / 'task3-dispatch.json',
                  BASE / 'factored_verification/derivation.json',
                  ROOT / 'docs/superpowers/plans/2026-09-05-candidate-a-factored-verification.md']
    for name in OLD_STAGES:
        parent = RECEIPTS / name
        old = json.loads((parent / 'input.json').read_text())
        for item in old['pins']:
            path = str(Path(item['path']).resolve())
            assert path not in pins or pins[path] == item['sha256']
            pins[path] = item['sha256']
        references.extend(sorted(p for p in parent.iterdir() if p.is_file()))
    admitted = ROOT / 'evidence/s02-candidate-a-completion/a5/alias-visibility-control'
    references.extend(sorted(p for p in admitted.iterdir() if p.is_file()))
    assert json.loads((admitted / 'validation.json').read_text())['ok'] is True
    for path, digest in pins.items():
        assert sha(path) == digest, path
    references = sorted(set(references))
    return {'task2FrozenCount': 30,
            'originalPins': [{'path': p, 'sha256': h} for p, h in sorted(pins.items())],
            'references': [{'path': str(p.relative_to(ROOT)), 'sha256': sha(p)}
                           for p in references]}

def expected_manifest():
    original_base = baseline()
    names = closure(BASE)
    rows = []
    for name in names:
        original = BASE / name
        relative = original.relative_to(ROOT)
        data = original.read_bytes()
        transformed = changed(name, data)
        rows.append({'original': str(relative), 'view': str((VIEW / relative).relative_to(ROOT)),
                     'originalSha256': sha(original),
                     'viewSha256': hashlib.sha256(transformed).hexdigest(),
                     'originalBytes': len(data), 'viewBytes': len(transformed),
                     'addedImport': ADDITIONS.get(name), 'originalImports': imports(original)})
    return {'schema': 'moriarty.a5-pilot-compilation-view/v1',
            'roots': ROOTS, 'files': rows, **original_base}

def validate():
    manifest = json.loads((CONTROL / 'view-manifest.json').read_text())
    assert manifest == expected_manifest()
    copied_base = VIEW / 'specs/quint/s02'
    assert closure(copied_base) == EXPECTED
    actual_files = sorted(str(p.relative_to(copied_base)) for p in VIEW.rglob('*') if p.is_file())
    assert actual_files == EXPECTED
    for row in manifest['files']:
        original = ROOT / row['original']
        copied = ROOT / row['view']
        name = str(original.relative_to(BASE))
        expected = changed(name, original.read_bytes())
        assert copied.read_bytes() == expected
        assert sha(copied) == row['viewSha256']
        if name in ADDITIONS:
            assert copied.read_bytes().replace(ADDITIONS[name].encode(), b'', 1) == original.read_bytes()
        wanted = ([ADDITIONS[name].split(' from "')[1].split('"')[0]]
                  if name in ADDITIONS else []) + row['originalImports']
        assert imports(copied) == wanted
    return manifest

if __name__ == '__main__':
    assert len(sys.argv) == 1 and sys.dont_write_bytecode and sys.pycache_prefix
    assert not VIEW.exists() and not (CONTROL / 'view-manifest.json').exists()
    manifest = expected_manifest()
    VIEW.mkdir()
    for row in manifest['files']:
        source = ROOT / row['original']
        target = ROOT / row['view']
        target.parent.mkdir(parents=True, exist_ok=True)
        with target.open('xb') as stream:
            stream.write(changed(str(source.relative_to(BASE)), source.read_bytes()))
    write(CONTROL / 'view-manifest.json', manifest)
    assert validate() == manifest
    print(json.dumps({'ok': True, 'viewFiles': 24, 'addedImports': 2,
                      'manifestSha256': sha(CONTROL / 'view-manifest.json')}))
```

- [ ] Use `apply_patch` to create `record.py` with exactly this source. This finite adapter does not alter the helper's module globals or dispatch behavior:

```python
import json
import re
import subprocess
import sys
from pathlib import Path
from prepare import ROOT, CONTROL, VIEW, PLAN, BOOTSTRAP, HELPER_SHA, sha, write, validate

assert sys.dont_write_bytecode and sys.pycache_prefix
sys.path.insert(0, str(ROOT))
from scripts.run_s02_candidate_a_factoring_pilot import record, QUINT, WITNESSES

STAGES = ['typecheck', 'prefix-test', 'sample-original', 'sample-factored',
          'compile-original', 'compile-factored']
assert len(sys.argv) == 2 and sys.argv[1] in STAGES
stage = sys.argv[1]
dispatch_path = CONTROL / 'dispatch.json'
d = json.loads(dispatch_path.read_text())
assert d['runtimeBootstrapBase'] == BOOTSTRAP
assert re.fullmatch('[0-9a-f]{40}', d['actualDispatchBase'])
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip() == d['actualDispatchBase']
assert d['planSha256'] == sha(PLAN)
assert d['runtimeHelperSha256'] == HELPER_SHA
assert all(sha(ROOT / name) == digest for name, digest in d['sources'].items())
manifest = validate()
assert sha(CONTROL / 'view-manifest.json') == d['manifestSha256']
extras = [ROOT / name for name in d['sources']] + [dispatch_path]
extras += [ROOT / row['path'] for row in manifest['references']]
for previous in STAGES[:STAGES.index(stage)]:
    parent = CONTROL / previous
    assert json.loads((parent / 'view-validation.json').read_text())['ok'] is True
    assert (parent / 'outer-command.json').is_file()
    extras += sorted(p for p in parent.iterdir() if p.is_file())
# The unchanged helper already pins/archives this complete original closure.
# Keep it once; the manifest retains all paired original/view references.
old_input = ROOT / '.superpowers/sdd/a5-factoring-receipts/compile-original-pilotSafety/input.json'
automatic_pins = {Path(row['path']) for row in json.loads(old_input.read_text())['pins']}
extras = tuple(p for p in dict.fromkeys(extras) if p not in automatic_pins)
base = VIEW / 'specs/quint/s02/factored_verification'
factored = stage.endswith('-factored')
stem = 'candidate_a_funding_pilot' + ('_f' if factored else '')
entry = base / (stem + '.qnt')
compiling = stage.startswith('compile-')
if stage == 'typecheck':
    argv = [str(QUINT), 'typecheck', str(base / 'candidate_a_funding_pilot_test.qnt')]
elif stage == 'prefix-test':
    argv = [str(QUINT), 'test', str(base / 'candidate_a_funding_pilot_test.qnt'),
            '--backend=rust', '--seed=42', '--match', 'allPilotPrefixesTest']
elif stage.startswith('sample-'):
    argv = [str(QUINT), 'run', str(entry), '--backend=rust', '--seed=42',
            '--max-samples=100', '--max-steps=5', '--invariant=pilotSafety',
            '--witnesses', *WITNESSES, '--verbosity=1']
else:
    argv = [str(QUINT), 'compile', str(entry), '--main=' + stem,
            '--target=json', '--invariant=pilotSafety', '--verbosity=0']
code = record('pilot-compilation-view/' + stage, argv, 900 if compiling else 1200,
              4096, 'pilotSafety', 5, compile_output=compiling, extra=extras,
              before_dispatch_base=BOOTSTRAP,
              domain='Original four-route/22-prefix Candidate A funding pilot compilation view; '
                     'two direct AuthorityKey imports only; actualDispatchBase=' + d['actualDispatchBase'])
out = CONTROL / stage
result = json.loads((out / 'result.json').read_text())
stderr = (out / 'stderr.txt').read_text()
output = out / ('input.qnt.json' if compiling else 'stdout.txt')
checks = {'childExitZero': code == 0, 'noTimeout': result['timedOut'] is False,
          'runtimeStable': result['runtimeUnchanged'] is True,
          'sourceStable': result['sourceAndToolsUnchanged'] is True,
          'emptyStderr': stderr == '', 'noMeasurementError': 'measurementError' not in result}
try:
    checks['viewAndOriginalPinsStable'] = validate() == manifest
except (AssertionError, OSError, ValueError):
    checks['viewAndOriginalPinsStable'] = False
details = {}
if stage == 'prefix-test':
    stdout = output.read_text()
    checks['exactPrefixTestPassed'] = 'ok allPilotPrefixesTest passed 1 test(s)' in stdout
    checks['onePassing'] = re.search(r'\b1 passing\b', stdout) is not None
    details['quantifiedPrefixPairs'] = 22
if stage.startswith('sample-'):
    found = re.findall(r'^(\w+) was witnessed in (\d+) trace\(s\) out of (\d+) explored',
                       output.read_text(), re.M)
    counts = {name: (int(count), int(total)) for name, count, total in found}
    checks['allEightWitnessesPositive'] = (len(found) == len(counts) == len(WITNESSES)
        and set(counts) == set(WITNESSES)
        and all(0 < count <= total == 100 for count, total in counts.values()))
    details['witnesses'] = counts
if compiling:
    try:
        document = json.loads(output.read_text())
        modules = document['modules']
        declarations = {item['name']: item for item in modules[0]['declarations']}
        checks['generatedMain'] = document['main'] == stem and len(modules) == 1 and modules[0]['name'] == stem
        checks['generatedBindingsPresent'] = all(name in declarations for name in ('init', 'step', 'pilotSafety', 'q::init', 'q::step', 'q::inv'))
        checks['generatedBytesMeasured'] = result['generated']['bytes'] == output.stat().st_size > 0
        details['generated'] = result['generated']
        details['generatedSha256'] = sha(output)
        if factored:
            baseline = json.loads((CONTROL / 'compile-original/result.json').read_text())
            checks['factoredStrictlySmaller'] = result['generated']['bytes'] < baseline['generated']['bytes']
    except (ValueError, KeyError, IndexError, TypeError):
        checks['generatedJSONAndMeasurements'] = False
ok = all(checks.values())
write(out / 'view-validation.json', {'ok': ok, 'checks': checks, 'details': details,
      'actualCommandExitCode': code, 'actualDispatchBase': d['actualDispatchBase'],
      'runtimeBootstrapBase': BOOTSTRAP, 'dispatchSha256': sha(dispatch_path),
      'parentArgv': sys.orig_argv, 'parentBytecodeDisabled': sys.dont_write_bytecode,
      'parentCachePrefix': sys.pycache_prefix,
      'classification': 'Pilot view gate only; no solver run or H1 conclusion'})
raise SystemExit(0 if ok else 1)
```

- [ ] Compare both script bytes to their complete plan blocks. Run preparation exactly once:

```sh
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-prepare-parent-cache .superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/prepare.py
```

Expected: wrapper exit 0 and JSON `ok: true`, `viewFiles: 24`, `addedImports: 2`. This command is source preparation, not a Quint command. Retain its actual outer terminal record under the control directory. If it fails, preserve the partial directory and stop; do not recreate or repair it under this plan.

- [ ] Root reviews both scripts, exact 24-file manifest, original pins, both import-only diffs and all copied import destinations before creating the exclusive dispatch with this complete command. At this point all proposed source bytes are concrete and reviewable. No child command precedes root's freeze:

```sh
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-dispatch-parent-cache - <<'PY'
import json, subprocess, sys
from pathlib import Path
root = Path.cwd()
c = root / '.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view'
sys.path.insert(0, str(c))
from prepare import PLAN, BOOTSTRAP, HELPER_SHA, sha, write, validate
manifest = validate()
paths = [c / 'prepare.py', c / 'record.py', PLAN, c / 'view-manifest.json']
paths += [root / row['view'] for row in manifest['files']]
assert sha(root / 'scripts/run_s02_candidate_a_factoring_pilot.py') == HELPER_SHA
d = {'schema': 'moriarty.a5-pilot-view-dispatch/v1',
     'actualDispatchBase': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
     'runtimeBootstrapBase': BOOTSTRAP, 'runtimeHelperSha256': HELPER_SHA,
     'planSha256': sha(PLAN), 'manifestSha256': sha(c / 'view-manifest.json'),
     'sources': {str(p.relative_to(root)): sha(p) for p in paths}}
write(c / 'dispatch.json', d)
print(json.dumps(d, indent=2))
PY
```

Root retains its actual freeze-command terminal record. If current HEAD changed from the reviewed dispatch state, root explicitly reviews the new actual base before launching; historical runtime bootstrap remains unchanged. No source or plan changes after dispatch.

### Task 2: Repeat the complete original pilot gate on the frozen view

**Interfaces:** Frozen `dispatch.json` and `view-manifest.json` are prerequisites. Each stage emits original helper artifacts plus exclusive `view-validation.json`. The actual outer tool terminal response is preserved as `outer-command.json` before the next stage starts; never substitute a planned or inferred wrapper exit.

**CV002/EARS:** When the view is frozen, each stage SHALL preserve the complete view plus original baseline source references in its source archive and SHALL check every original/view pin before and after execution. The exact prefix test and both representative samples SHALL pass before compilation.

**OpenSpec CV002:** Given unchanged original semantic bodies and all 30 frozen files, the recursive test-root typecheck must exit 0; `allPilotPrefixesTest` must report one passing test covering all 22 quantified prefix pairs; each sample must report all eight original witnesses strictly positive out of 100 explored traces. Different test selection, a sample without witness coverage or a timeout stops before compilation.

- [ ] Run each line as a separate tool command, to terminal, in the displayed order. Do not combine lines into a shell sequence. Preserve the complete returned tool response(s), session/chunk identifiers, exact parent argv, cwd and actual terminal wrapper exit with `apply_patch` as that stage's `outer-command.json`; copy the actual records rather than inventing timestamps or success values. Later stages include these earlier top-level originals in their source archives. If a tool returns a live session, wait on that session without relaunching the command.

```sh
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-typecheck-parent-cache .superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/record.py typecheck
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-prefix-test-parent-cache .superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/record.py prefix-test
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-sample-original-parent-cache .superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/record.py sample-original
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-sample-factored-parent-cache .superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/record.py sample-factored
```

Expected for each: child exit 0, actual wrapper exit 0, `view-validation.json` `ok: true`, empty stderr, no timeout, stable runtime/source/view/original pins. Inspect raw stdout as well as the acceptance file. The exact original `--backend=rust --seed=42 --max-samples=100 --max-steps=5 --invariant=pilotSafety --witnesses prepared signed proposed verified committed rejected afterCompleted beforeCompleted --verbosity=1` sample settings remain unchanged.

- [ ] Root intakes the four actual terminal outcomes and preserved closures before the paired compilation unit. No additional tests, samples or inferred coverage are substituted. Failure ends execution and proceeds only to evidence handoff.

### Task 3: Compile the paired views and stop at root input review

**Interfaces:** Accepted Task2 receipts and frozen sources. Produces both raw generated JSON files, native child resource streams, stable pins and measured generated-byte/object/let/app counts through unchanged `record()`.

**CV003/EARS:** When both view pilot gates are admitted, the original view SHALL compile once and the factored view SHALL compile once with the original 900-second/4096-MiB limits. The original compile must reach an accepted terminal result before factored compilation starts. Invalid or failed compilation SHALL stop without any follow-up trial.

**OpenSpec CV003:** Given successful original compilation and unchanged source/runtime pins, factored compilation must exit 0 with valid measured JSON strictly smaller than original JSON. Both raw inputs must be retained. A successful but non-smaller factored result is preserved as successful compilation and failed size gate; it does not justify a solver run.

- [ ] Run original compilation as one exact command, wait to terminal, inspect validation and preserve its actual outer command record before the next command:

```sh
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-compile-original-parent-cache .superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/record.py compile-original
```

- [ ] Only on accepted original terminal success, run factored compilation separately, wait to terminal and preserve its actual outer command record:

```sh
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-compile-factored-parent-cache .superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/record.py compile-factored
```

Expected: both child exits 0, both wrapper exits 0, valid generated JSON, correct unchanged module names and `init`/`step`/`pilotSafety`/`q::init`/`q::step`/`q::inv` declarations, measured bytes and stable complete pins. The factored wrapper exits 1 if compilation succeeds but its size gate fails; report those outcomes separately. The recorder's structural presence checks are only a preliminary check: root must inspect the actual binding expressions and complete pilot state/history/guard structure, not merely declaration names.

### Task 4: Preserve handoff and independent review gate

**CV004/EARS:** At the first failure or after paired compilation, the implementer SHALL retain every available original output and attempt the handoff/index command when its stated prerequisites hold. If preparation or the handoff command itself fails, the implementer SHALL preserve the actual failed command and partial originals and transfer inventory ownership to root, explicitly identifying any missing or partial handoff/index. Root SHALL inspect source archive membership against pins, actual terminal exits, raw generated input predicates, both module bodies and size/resource metrics before considering a separately authorized solver experiment.

**OpenSpec CV004:** Given readable partial or complete terminal receipts, handoff enumerates present and missing stages separately, preserves the actual failed child and wrapper outcomes, records generated sizes and GNU-time streams, and does not convert absent stages into successes. If a missing pin, unreadable archive or other error prevents that command from completing, its actual failed terminal record and partial originals trigger root takeover. Root inventories missing inputs and unavailable outputs explicitly; no missing original is fabricated and no native command is rerun.

- [ ] Run this read-only audit plus exclusive handoff/index creation after terminal intake, including after an unexpected native-command failure when the required manifest exists. Do not rerun it in the same directory. With readable inputs it preserves failed states as such; absent stages remain absent. If preparation failed before a manifest exists, retain the actual preparation tool failure and partial files for root rather than running this command. This handoff command can itself fail, including with `OSError` on a missing pin or `tarfile.TarError` on an unreadable archive; it does not guarantee creation of both output files on every failure path.

```sh
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-pilot-view-handoff-parent-cache - <<'PY'
import hashlib, json, sys, tarfile
from pathlib import Path
root = Path.cwd()
c = root / '.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view'
sys.path.insert(0, str(c))
from prepare import sha, write, validate
stages = ['typecheck', 'prefix-test', 'sample-original', 'sample-factored',
          'compile-original', 'compile-factored']
rows = []
for stage in stages:
    parent = c / stage
    if not (parent / 'result.json').exists():
        rows.append({'stage': stage, 'status': 'no terminal result'})
        continue
    metadata = json.loads((parent / 'input.json').read_text())
    pins = {e['path']: e['sha256'] for e in metadata['pins']}
    archive_path = parent / 'source-and-runner.tar.gz'
    archive_ok = sha(archive_path) == metadata['sourceArchiveSha256']
    with tarfile.open(archive_path, 'r:gz') as archive:
        members = archive.getmembers()
        archive_ok = archive_ok and all(m.isfile() for m in members)
        for member in members:
            if not member.isfile():
                continue
            archive_ok = archive_ok and (hashlib.sha256(archive.extractfile(member).read()).hexdigest()
                                          == pins.get(str(root / member.name)))
    validation_path = parent / 'view-validation.json'
    rows.append({'stage': stage, 'result': json.loads((parent / 'result.json').read_text()),
                 'validation': json.loads(validation_path.read_text()) if validation_path.exists() else None,
                 'outerTerminalRetained': (parent / 'outer-command.json').is_file(),
                 'sourceArchiveMembers': len(members), 'sourceArchivePinsMatch': archive_ok,
                 'livePinsMatch': all(sha(p) == h for p, h in pins.items()),
                 'resources': (parent / 'resources.txt').read_text()})
try:
    validate()
    view_stable = True
except (AssertionError, OSError, ValueError):
    view_stable = False
write(c / 'handoff.json', {'schema': 'moriarty.a5-pilot-view-handoff/v1',
      'viewAndOriginalPinsStable': view_stable, 'stages': rows,
      'scope': 'Pilot prefix/sample/frontend evidence only; no solver execution or H1 conclusion',
      'rootInputReview': 'pending independent review'})
files = []
for path in sorted(c.rglob('*')):
    if path.is_file():
        files.append({'path': str(path.relative_to(root)), 'bytes': path.stat().st_size,
                      'sha256': sha(path)})
manifest = json.loads((c / 'view-manifest.json').read_text())
runtime = root / '.superpowers/sdd/a5-factoring-receipts/tool-store/manifest.json'
shared = json.loads(runtime.read_text())
references = [runtime, runtime.with_name('runtime.tar.gz'),
              Path(shared['reusedPythonManifest']), Path(shared['reusedPythonArchive'])]
write(c / 'original-file-hash-index.json', {'schema': 'moriarty.a5-pilot-view-index/v1',
      'selfExcluded': str((c / 'original-file-hash-index.json').relative_to(root)),
      'files': files, 'baselineReferences': manifest['references'],
      'originalPins': manifest['originalPins'],
      'sharedEnvironmentReferences': [{'path': str(p), 'sha256': sha(p)} for p in references]})
print(json.dumps({'handoffSha256': sha(c / 'handoff.json'),
                  'indexSha256': sha(c / 'original-file-hash-index.json'),
                  'indexedFiles': len(files), 'viewAndOriginalPinsStable': view_stable}))
PY
```

- [ ] If the handoff command fails, preserve its actual outer terminal response, exception stream and all partial originals. Report whether `handoff.json` and `original-file-hash-index.json` are absent, partial or complete according to their actual bytes; do not report the planned index as created. Root then takes over the inventory of present files and missing inputs. Do not fabricate missing source/output bytes, overwrite partial files, rerun the handoff command or rerun any native command.
- [ ] Report actual stage child exits, actual outer wrapper exits, prefix/witness coverage, each compile size and object counts, each GNU-time frontend elapsed/max-RSS, all validation failures, exact index/handoff hashes and absent stages. Do not call post-child runtime verification duration the native frontend duration. Preserve inherited helper classification/deadlock/fairness prose verbatim; this plan's typecheck/test/compile stages do not invoke the checker, and sample evidence is not exhaustive checker evidence.
- [ ] Root independently inspects both generated inputs themselves, including `q::init`/`q::step`/`q::inv` targets, actual `pilotSafety`, complete state/history, and the original/factored byte and object-count comparison. Root audits original-source/member pins, all 30 frozen derivation files, two exact import-only diffs and all actual terminal records. If either compile failed or factored input is non-smaller, H1 remains unresolved and this experiment series stops.
- [ ] Stop after this bounded handoff. No baseline checker, factored checker, `neverPrepared` control, full-workload A5 execution, integration or completion claim is authorized by this plan. Those remain the later original gates, subject to independent root review and separate dispatch.

## Planning self-review

CV001–CV004 cover exact closure, immutable paired provenance, original prefix/sample gates, sequential bounded compilation and independent input review. All executable source and shell commands are present; no command has been executed during planning. The import graph was traversed read-only and found exactly the enumerated 24 files. Original stage receipt commands/budgets were read, including the eight witness names and 22-prefix test. No original gate is waived. Unresolved empirical question: whether the full pilot compiles and, if both inputs compile, whether the factored generated JSON is strictly smaller. These are execution outcomes, not discretionary implementation choices.
