"""Documentary source check: forbid Popen; never import or execute run_once.py."""
from pathlib import Path
import ast
import difflib
import hashlib
import json
import subprocess
import sys

D = Path(__file__).resolve().parent
A = D.parent / 'rust-core-external-starter-preparation-20261019-a2'
P = D.parent / 'rust-core-external-starter-one600-resource-choice-20261019'
O = A / 'inputs'
K = Path('/home/charl/Moriarty/.worktrees/utxo-guarded-full-prefix-main-20261018')


def forbidden_popen(*args, **kwargs):
    raise AssertionError('Popen forbidden in documentary source preflight')


subprocess.Popen = forbidden_popen


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def checked(p, row):
    assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256'], str(p)


def main():
    assert len(sys.argv) == 2, 'Pass exact BOUND-CANDIDATE.json SHA256.'
    assert sha(D / 'BOUND-CANDIDATE.json') == sys.argv[1]
    binding = json.loads((D / 'BOUND-CANDIDATE.json').read_bytes())
    assert sha(D / 'run_once.py') == binding['driverSha256']
    for name, digest in binding['controlHashes'].items():
        assert sha(D / name) == digest, name
    references = json.loads((D / 'INPUT-REFERENCES.json').read_bytes())
    for row in references['files']:
        checked(Path(row['path']), row)
    old = (A / 'run_once.py').read_text()
    new = (D / 'run_once.py').read_text()
    old_ast, new_ast = ast.parse(old), ast.parse(new)
    assert (D / 'A2-SUCCESSOR.diff').read_text() == ''.join(difflib.unified_diff(
        old.splitlines(keepends=True), new.splitlines(keepends=True),
        fromfile=str(A / 'run_once.py'), tofile=str(D / 'run_once.py')))
    old_control = ast.get_source_segment(old, next(n for n in old_ast.body if isinstance(n, ast.FunctionDef) and n.name == 'run_selected'))
    new_control = ast.get_source_segment(new, next(n for n in new_ast.body if isinstance(n, ast.FunctionDef) and n.name == 'run_selected'))
    normalized = new_control.replace(', name, parent_seconds):', ', name):')
    normalized = normalized.replace('# Existing terminal-first control block; only the plan-bound parent deadline varies.', '# Extracted existing normal90 control block; see CONTROL-REUSE.json.')
    normalized = normalized.replace('deadline = start + parent_seconds', 'deadline = start + 95')
    normalized = normalized.replace('never add wait/grace after the selected parent deadline.', 'never add wait/grace after95.')
    normalized = normalized.replace("f'Parent{parent_seconds} exhausted; owned membership unknown; no extra wait.'", "'Parent95 exhausted; owned membership unknown; no extra wait.'")
    assert normalized == old_control, 'Only proposed parent deadline adaptation in selected control.'
    membership = json.loads((A / 'INPUT-MEMBERSHIP.json').read_bytes())
    original = Path(membership['originalRoot'])
    for row in membership['files']:
        for root in [original, O]:
            checked(root / row['path'], row)
    assert {str(p.relative_to(O)) for p in O.rglob('*') if p.is_file()} == {r['path'] for r in membership['files']}
    plan = json.loads((P / 'PROPOSED-CHECK-PLAN.json').read_bytes())
    prior = json.loads((O / 'CHECK-PLAN.json').read_bytes())
    rebind = json.loads((P / 'PATH-REBINDING.json').read_bytes())
    assert plan['head'] == prior['head'] == binding['head']
    assert plan['tree'] == prior['tree'] == binding['tree']
    assert plan['ownedRoot'] == rebind['newRoot'] == binding['ownedRoot']
    assert plan['actualOutput'] == binding['actualOutput']
    for key in ['ownedRoot', 'oneInitiallyAbsentColdTarget', 'actualOutput']:
        path = Path(plan[key])
        assert not path.exists() and not path.is_symlink(), str(path)
    assert len(plan['steps']) == 9 and len(plan['environmentExact13']) == 13
    assert [s['wallSeconds'] for s in plan['steps']] == [600] + [90] * 8
    assert [s['parentMaximumSeconds'] for s in plan['steps']] == [605] + [95] * 8
    assert all(s['killGraceSeconds'] == 5 and s['attempts'] == 1 for s in plan['steps'])
    assert sum(s['parentMaximumSeconds'] for s in plan['steps']) == plan['finiteMaximumSelectedCommandWallSeconds'] == 1365
    def transport(v):
        if isinstance(v, str):
            return v.replace(rebind['oldRoot'], rebind['newRoot'])
        if isinstance(v, list):
            return [transport(x) for x in v]
        if isinstance(v, dict):
            return {k: transport(x) for k, x in v.items()}
        return v
    assert plan['environmentExact13'] == transport(prior['environmentExact13'])
    for i, (before, after) in enumerate(zip(prior['steps'], plan['steps'])):
        expected = transport(before)
        expected['wallSeconds'] = 600 if i == 0 else 90
        expected['parentMaximumSeconds'] = 605 if i == 0 else 95
        assert after == expected
    freeze = json.loads((O / 'EXPECTED-FREEZE.json').read_bytes())
    changed = rebind['expectedFileChanges']
    unchanged = rebind['unchangedExpectedFiles']
    assert len(changed) == 4 and len(unchanged) == 20 and len(freeze['files']) == 24
    assert {r['path'] for r in freeze['files']} == {r['expectationPath'] for r in changed} | {r['path'] for r in unchanged}
    assert sum(r['occurrences'] for r in changed) == 8
    for row in changed:
        before, after = O / row['expectationPath'], P / row['after']['path']
        checked(before, row['before'])
        checked(after, row['after'])
        assert before.read_bytes().count(rebind['oldRoot'].encode()) == row['occurrences']
        assert after.read_bytes() == before.read_bytes().replace(rebind['oldRoot'].encode(), rebind['newRoot'].encode())
    for row in unchanged:
        checked(O / row['path'], row)
    tools = json.loads((O / 'TOOL-PINS.json').read_bytes())['tools']
    for row in tools:
        assert str(Path(row['path']).resolve(strict=True)) == row['resolved']
        checked(Path(row['resolved']), row)
    python = next(row for row in tools if row['path'] == '/usr/bin/python3')
    assert str(Path(sys.executable).resolve(strict=True)) == python['resolved']
    source_count = 0
    for name in ['SOURCE-PINS.json', 'WORKSPACE-EXPORT-PINS.json']:
        for row in json.loads((O / name).read_bytes())['files']:
            checked(K / row['path'], row)
            source_count += 1
    registry = json.loads((O / 'REGISTRY-SOURCE-PINS.json').read_bytes())
    registry_count = 0
    for pkg in registry['packages']:
        assert sha(Path(pkg['archivePath'])) == pkg['archiveSha256']
        for row in pkg['files']:
            checked(Path(pkg['sourceRoot']) / row['path'], row)
            registry_count += 1
    root = Path(plan['ownedRoot'])
    config_count = 0
    for directory in [root / 'source/crates/mori', *(root / 'source/crates/mori').parents,
                      root / 'cwd', *(root / 'cwd').parents, root / 'cargo-home']:
        for name in ['config', 'config.toml']:
            p = directory / name if directory == root / 'cargo-home' else directory / '.cargo' / name
            assert not p.exists() and not p.is_symlink(), str(p)
            config_count += 1
    result = {'kind': 'POPEN_FORBIDDEN_DOCUMENTARY_SOURCE_PREFLIGHT', 'passed': True,
              'bindingSha256': sys.argv[1], 'driverSha256': binding['driverSha256'],
              'status': 'UNRUN_RESOURCE_PROPOSAL_UNADOPTED',
              'runnerAstParsedWithoutImport': True, 'popenForbidden': subprocess.Popen is forbidden_popen,
              'selectedControlOnlyParentDeadlineAdapted': True,
              'immutableOriginalAndA2InputFilesVerified': len(membership['files']),
              'expectedWireFilesLiteralRootOnly': 4, 'rootOccurrences': 8, 'otherExpectedFilesUnchanged': 20,
              'toolPinsVerified': len(tools), 'sourcePinsVerified': source_count,
              'publicRegistryArchivesVerified': len(registry['packages']), 'publicRegistrySourceFilesVerified': registry_count,
              'prospectiveConfigPathsAbsentChecks': config_count,
              'subjectRunnerGitProductProviderExecution': False,
              'limits': 'No observed Git HEAD/tree/clean state, process behavior, Cargo unit graph, compilation, installed binary or consumer results. No source/result/product acceptance; two agreeing fresh resource opinions, root adoption and whole successor inspection remain required before execution.'}
    (D / 'SOURCE-PREFLIGHT.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))


if __name__ == '__main__':
    main()
