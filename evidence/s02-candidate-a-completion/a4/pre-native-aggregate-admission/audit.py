"""Standalone preliminary114 archive audit; no archived code or large-tree reads."""
import sys

# This guard must remain effective with -O/PYTHONOPTIMIZE: never use assert.
if sys.flags.optimize != 0:
    raise RuntimeError('Archive audit requires Python optimization disabled')

import ast
import hashlib
import json
from pathlib import Path
import xml.etree.ElementTree as ET

HERE = Path(__file__).resolve().parent
SDD = '.superpowers/sdd/'
R = SDD + 'a4-task6-resumption-20260906/'
F = R + 'fresh97/'
P = 'evidence/s02-candidate-a-completion/a4/pre-native-recovery-admission/'
TEST = 'tests/test_s02_candidate_a_integrated.py'
PYTHON = '/home/charl/Moriarty/.venv/bin/python'


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def decode(raw):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, 'duplicate JSON key: ' + key)
            result[key] = value
        return result
    return json.loads(raw, object_pairs_hook=unique)


def zero(value):
    return type(value) is int and value == 0


def main():
    index = decode((HERE / 'index.json').read_bytes())
    require(index['archive'] == 'original-small-evidence.tar.gz', 'archive filename')
    for entry in index['packageSupport']:
        require(Path(entry['path']).name == entry['path'], 'package source path')
        require(sha((HERE / entry['path']).read_bytes()) == entry['sha256'], 'package source hash')
    # Fixed local audit source, verified above. Never import anything from tar.
    import retained_checks
    raw = (HERE / index['archive']).read_bytes()
    require(sha(raw) == index['archiveSha256'], 'outer archive hash')
    files = retained_checks.tar_members(raw)
    require(len(index['members']) == len({e['path'] for e in index['members']}) == index['memberCount'],
            'outer index uniqueness')
    require(set(files) == {e['path'] for e in index['members']}, 'exact outer membership')
    require(sum(len(v) for v in files.values()) == index['memberBytes'], 'outer byte accounting')
    for entry in index['members']:
        require(len(files[entry['path']]) == entry['bytes'] and sha(files[entry['path']]) == entry['sha256'],
                'outer member pin: ' + entry['path'])
    require(index['scope'] == {'preliminaryAdmitted': 114, 'retained': 17, 'fresh': 97,
                              'nativeFinalRequired': 115, 'originalParentExit': None, 'a4Complete': False},
            'scope must remain preliminary only')
    prior_package = {name[len(P):]: value for name, value in files.items() if name.startswith(P)}
    retained_result = retained_checks.validate_retained(prior_package)
    require(retained_result['ok'] and retained_result['retainedControls'] == 17, 'nested retained17 audit')
    prior = retained_checks.tar_members(prior_package['original-small-evidence.tar.gz'])
    combined = dict(prior)
    for name, value in files.items():
        require(name not in combined or combined[name] == value, 'overlapping original bytes disagree')
        combined[name] = value
    root = index['originalRepositoryRoot']

    def relative(path):
        require(path.startswith(root + '/'), 'external path cannot be read by archive audit')
        return path[len(root) + 1:]

    def doc(name):
        return decode(combined[name])

    checked_pins = {}
    def pin_check(pin):
        name = relative(pin['path'])
        raw = combined[name]
        require(len(raw) == pin['bytes'] and sha(raw) == pin['sha256'], 'actual archived pin: ' + name)
        old = checked_pins.get(pin['path'])
        require(old is None or old == pin, 'inconsistent duplicate pin')
        checked_pins[pin['path']] = pin

    require(sha(files[R + 'root-admission114.json']) ==
            '2b112e8273b7f2a1b12adf9cc1404dc7c3da5fb933e14a972fc12f35519aba08', 'root114 adoption pin')
    require(sha(files[SDD + 'a4-fresh97-independent-intake-20260906.md']) ==
            '650058f9d22b21946f68e0b50034d5db4efc840e23c7f871e9ae9b36f85197e0', 'independent114 intake pin')
    require(sha(files[SDD + 'a4-fresh97-package-proposal-20260906.md']) ==
            'd013b17962cf8227925ebbe797b6a951b75dc31e65d19c797df069b2c959fffa', 'proposal pin')
    decision = doc(R + 'root-admission114.json')
    require([decision[k] for k in ('requiredPreNativeCount', 'retainedCount', 'freshCount')] == [114, 17, 97],
            'root admitted scope')
    require(decision['originalParentExit'] is None and all(zero(decision[k]) for k in
            ('freshPytestExit', 'freshRecorderExit', 'freshActualOuterExit')), 'root actual exit statements')
    require(index['limitations'] == decision['limitations'] and len(index['limitations']) == 5
            and index['stillRequired'] == decision['stillRequired'] and len(index['stillRequired']) == 4,
            'adopted limitations/remaining requirements')
    for pin in decision['files']:
        pin_check(pin)
    source_input = doc(F + 'input.json')
    outer_terminal = doc(F + 'terminal.json')
    require(zero(outer_terminal['exitCode']) and outer_terminal['error'] is None
            and outer_terminal['originalParentExit'] is None and not outer_terminal['artifactFinalizationBlocked'],
            'recorder actual outcome')
    require(outer_terminal['sourceChanged'] == outer_terminal['environmentChanged'] == [], 'recorded source changes')
    require(source_input['sourceBefore'] == outer_terminal['sourceAfter']
            and source_input['environmentBefore'] == outer_terminal['environmentAfter'], 'exact endpoint pin pairs')
    require(len(outer_terminal['sourceAfter']) == 46 and len(outer_terminal['environmentAfter']) == 3329,
            'source/runtime closure sizes')
    require(source_input['sourceCommit'] == decision['originalDispatchHead'] ==
            '43f33721da7a3bfb11dd1a91f1f548d5110ab93c', 'original dispatch HEAD')
    require(source_input['parentFlags'] == {'dont_write_bytecode': 1, 'optimize': 0}, 'new recorder flags')
    expected_parent = [PYTHON, '-B', '-X', 'pycache_prefix=' + root + '/' + R + 'recorder-cache-fresh97',
                       R + 'recorder.py', 'fresh97']
    require(source_input['parentOrigArgv'] == outer_terminal['parentOrigArgv'] == expected_parent,
            'exact parent orig_argv')
    for pin in outer_terminal['ownedFiles']:
        pin_check(pin)
    source_archive = retained_checks.tar_members(files[F + 'source.tar.gz'])
    expected_source = {entry['archivePath']: entry for entry in source_input['sourceBytes']}
    require(len(source_archive) == 59 and set(source_archive) == set(expected_source), 'exact59-member fresh source archive')
    for name, raw in source_archive.items():
        pin = expected_source[name]
        require(len(raw) == pin['bytes'] and sha(raw) == pin['sha256'], 'fresh nested source member')
        relative_name = relative(pin['path'])
        require(relative_name not in combined or combined[relative_name] == raw, 'nested source disagreement')
        combined[relative_name] = raw
    for pin in source_input['sourceBefore'] + source_input['supportBefore']:
        pin_check(pin)
    pin_check(source_input['supportManifest'])
    require(len(doc(R + 'support.json')['files']) == 12, 'support closure')
    for pin in doc(R + 'support.json')['files']:
        pin_check(pin)
    for name, wanted in retained_checks.FROZEN.items():
        if name in combined:
            require(sha(combined[name]) == wanted, 'frozen source/helper/retained receipt: ' + name)
    old = doc(SDD + 'a4-task6-receipts/green-tests/input.json')
    old_source = [(e['path'], e['bytes'], e['sha256']) for e in old['sourceBytes']]
    require([(e['path'], e['bytes'], e['sha256']) for e in source_input['sourceBefore']] == old_source,
            'source rows compare explicit pin fields, not archive-only metadata')
    require(source_input['environmentBefore'] == old['environmentBefore']
            and source_input['pythonBefore'] == old['pythonBefore'], 'shared recorded runtime')
    require(source_input['runtimeArchive']['sha256'] == old['pythonEnvironmentArchiveSha256'] ==
            index['externalEvidence'][2]['sha256'], 'external runtime archive binding')
    p = doc(R + 'partition.json')
    required, retained, fresh = (p[k] for k in
            ('requiredPreNativeNodeids', 'retainedOriginalChildNodeids', 'freshComplementNodeids'))
    require((len(required), len(retained), len(fresh)) == (114, 17, 97)
            and len(set(required)) == 114 and len(set(retained)) == 17 and len(set(fresh)) == 97
            and not set(retained) & set(fresh) and set(retained) | set(fresh) == set(required), 'exact partition')
    collected = doc(F + 'collected-nodeids.json')
    require(len(collected) == len(set(collected)) == 115
            and [n for n in collected if n != p['nativeDeferredNode']] == required, 'collection exact115/114')
    pytest = doc(F + 'pytest-terminal.json')
    require(zero(pytest['exitCode']) and pytest['eligible'], 'pytest authentic exit/eligibility')
    lifecycles = outer_terminal['commandLifecycles']
    require(len(lifecycles) == 2, 'exact two recorded collection/test commands')
    for label, args in (
        ('collection', [PYTHON, '-B', '-X', 'pycache_prefix=' + root + '/' + F + 'collection-cache',
                        '-m', 'pytest', '--collect-only', '-q', '-p', 'no:cacheprovider', TEST, 'tests/test_a4_json_stream.py']),
        ('pytest', [PYTHON, '-B', '-X', 'pycache_prefix=' + root + '/' + F + 'fresh-pycache',
                    '-m', 'pytest', '-q', '-p', 'no:cacheprovider', *fresh,
                    '--basetemp=' + root + '/' + F + 'basetemp', '--junitxml=' + root + '/' + F + 'junit.xml'])):
        command_input = doc(F + label + '-input.json')
        process = doc(F + label + '-process.json')
        terminal = doc(F + label + '-terminal.json')
        require(command_input['argv'] == process['argv'] == args and process['pid'] == process['ownedPgid'],
                'exact command argv/owned identity')
        require(command_input['parentFlags']['optimize'] == 0 and not command_input['childPythonOptimizePresent'],
                'optimization policy')
        require(command_input['environmentOverrides']['PYTHONPATH'] == ''
                and command_input['environmentOverrides']['PYTEST_ADDOPTS'] == '', 'recorded environment controls')
        require(zero(terminal['exitCode']) and terminal['eligible'] and terminal['monitorError'] is None
                and terminal['receivedSignals'] == [] and terminal['cleanup'] ==
                {'complete': True, 'forced': False, 'signals': [], 'errors': []}, 'complete unforced command lifecycle')
        require(terminal['ownedPgid'] == process['pid'] and any(
                all(life[k] == v for k, v in terminal.items()) for life in lifecycles), 'outer command lifecycle binding')
        pin_check(terminal['stdout'])
        pin_check(terminal['stderr'])
        require(files[F + label + '-stderr.txt'] == b'', 'original command stderr')
    xml = ET.fromstring(files[F + 'junit.xml'])
    cases = xml.findall('.//testcase')
    executed = [case.attrib['classname'].replace('.', '/') + '.py::' + case.attrib['name'] for case in cases]
    require(executed == fresh and len(cases) == 97 and not xml.findall('.//failure')
            and not xml.findall('.//error') and not xml.findall('.//skipped'), 'actual ordered JUnit97')
    require(files[F + 'pytest-stdout.txt'].decode().endswith('97 passed in 4999.83s (1:23:19)\n'),
            'original passed summary')
    outer = doc(F + 'outer-tool-receipt.json')
    expected_command = ' '.join(expected_parent)
    require(outer['args']['cmd'] == expected_command and outer['args']['workdir'] == root, 'actual outer command')
    responses = outer['responses']
    require(len(responses) == 87 and responses[0]['session_id'] == 68260
            and responses[0]['chunk_id'] == '47f87b' and responses[-1]['chunk_id'] == '31e729'
            and zero(responses[-1]['exit_code']), 'actual launch/final outer response')
    require(all(r.get('session_id') == 68260 and 'exit_code' not in r for r in responses[:-1]),
            'same nonterminal session identity')
    emitted = decode(''.join(r['output'] for r in responses))
    require(emitted == {'stage': root + '/' + F.rstrip('/'), 'exitCode': 0, 'error': None}, 'outer final summary')
    progress = [decode(line) for line in files[R + 'fresh97-outer-progress.ndjson'].splitlines()]
    require(progress[0]['args'] == outer['args'] and progress[0]['initialSessionId'] == 68260
            and progress[0]['kind'] == 'nonterminal-progress-log-not-pytest-success', 'progress header scope')
    polls = [line for line in progress if 'response' in line]
    require([line['index'] for line in polls] == list(range(87))
            and [line['response'] for line in polls] == responses, 'exact preserved response/progress sequence')
    gaps = [line for line in progress if line.get('kind') == 'monitoring-metadata-gap']
    require(len(gaps) == 1 and outer['monitoringLimitations'] == gaps and gaps[0]['monitorCell'] == '50',
            'missing polling metadata must remain disclosed, never reconstructed')
    manifest = doc(F + 'fresh-artifacts.json')
    regular = [e for e in manifest['entries'] if e['kind'] == 'file']
    require(len(regular) == 274 and sum(e['bytes'] for e in regular) == 6237424864
            and sum(e['kind'] == 'directory' for e in manifest['entries']) == 76
            and sum(e['kind'] == 'symlink' for e in manifest['entries']) == 20, 'complete fresh manifest accounting')
    require(index['smallThresholdBytes'] == 2_000_000, 'small selection threshold')
    selected = [e for e in regular if e['bytes'] <= index['smallThresholdBytes']]
    selected_paths = [relative(e['path']) for e in selected]
    require(len(selected) == 100 and sum(e['bytes'] for e in selected) == 32550779
            and selected_paths == index['selectedSmallArtifactPaths'], 'exact manifest-selected100 originals')
    archived_basetemp = {name for name in files if name.startswith(F + 'basetemp/')}
    require(archived_basetemp == set(selected_paths), 'small fixture/child archive membership')
    for pin in selected:
        pin_check({'path': pin['path'], 'bytes': pin['bytes'], 'sha256': pin['sha256']})
    # AST is inspected only to recover the exact archived worker command string.
    module = ast.parse(combined[TEST])
    worker = next(n for n in module.body if isinstance(n, ast.FunctionDef) and n.name == 'task6_process')
    script = next(ast.literal_eval(n.value) for n in worker.body if isinstance(n, ast.Assign)
                  and any(isinstance(t, ast.Name) and t.id == 'script' for t in n.targets))
    controls = doc(F + 'fresh-controls.json')['controls']
    expected_children = {}
    for node in fresh:
        if '::test_semantic_rebound_triples[' in node:
            expected_children[('semantic', node.split('[', 1)[1][:-1])] = node
        elif node.endswith('::test_raw_latest_field_substitution_triple'):
            expected_children[('substitution', 'profile')] = node
        elif '::test_provenance_locator_triples[' in node:
            expected_children[('locator', node.split('[', 1)[1][:-1])] = node
    require(len(controls) == len(expected_children) == 16, 'fresh child inventory')
    actual_children = {}
    for control in controls:
        key = (control['mode'], control['name'])
        require(key in expected_children and key not in actual_children
                and control['nodeid'] == expected_children[key], 'exact unique child identity')
        folder = relative(control['folder']) + '/'
        absolute = control['folder']
        argv = [PYTHON, '-B', '-X', 'pycache_prefix=' + absolute + '/fresh-cache',
                '-c', script, root + '/' + TEST, *key, absolute]
        require(doc(folder + 'argv.json') == control['argv'] == argv, 'actual archived worker argv')
        child_terminal = doc(folder + 'terminal.json')
        require(set(child_terminal) == {'exit'} and zero(child_terminal['exit'])
                and zero(control['originalChildExit']), 'original child actual exit')
        report = doc(folder + 'stdout.txt')
        keys = {'mode', 'name', 'mutant_rejected', 'corrected', 'unrelated'}
        if key[0] == 'semantic':
            keys |= {'changed_events', 'raw_linkage'}
            require(type(report['changed_events']) is int and report['changed_events'] > 0
                    and report['raw_linkage'] is True, 'effective semantic mutation')
        require(set(report) == keys and report == control['report'] and (report['mode'], report['name']) == key
                and all(report[k] is True for k in ('mutant_rejected', 'corrected', 'unrelated'))
                and combined[folder + 'stderr.txt'] == b'', 'exact child report/positive controls')
        for pin in control['records'].values():
            pin_check(pin)
        actual_children[control['nodeid']] = control
    require(set(actual_children) == set(expected_children.values()), 'no child omitted')
    original_retained = doc(R + 'admit17/retained-controls.json')['controls']
    by_retained = {c['nodeid']: c for c in original_retained}
    require(list(by_retained) == retained, 'retained17 identity mapping')
    for pin in doc(F + 'retained-admission-binding.json').values():
        pin_check(pin)
    aggregate = doc(F + 'aggregate-candidate.json')
    require([c['nodeid'] for c in aggregate['controls']] == required
            and aggregate['originalParentExit'] is None and aggregate['nativeFinal115StillRequired'] is True,
            'exact ordered aggregate/limits')
    aggregate_pins = {}
    for control in aggregate['controls']:
        node = control['nodeid']
        pins = [control['receipt']]
        if node in by_retained:
            require(control['kind'] == 'retained-original-child'
                    and control['receipt']['path'] == root + '/' + R + 'admit17/retained-controls.json'
                    and control['childRecords'] == by_retained[node]['records'], 'retained node binding')
        else:
            require(node in fresh and control['kind'] == 'fresh-pytest-pass'
                    and control['receipt']['path'] == root + '/' + F + 'pytest-terminal.json'
                    and control['junit']['path'] == root + '/' + F + 'junit.xml', 'fresh node actual pytest/JUnit binding')
            pins.append(control['junit'])
            if node in actual_children:
                require(control['childRecords'] == actual_children[node]['records'], 'fresh child per-node record binding')
            else:
                require('childRecords' not in control, 'invented child evidence for non-child test')
        pins.extend(control.get('childRecords', {}).values())
        for pin in pins:
            pin_check(pin)
            require(pin['path'] not in aggregate_pins or aggregate_pins[pin['path']] == pin, 'inconsistent aggregate pin')
            aggregate_pins[pin['path']] = pin
    require(len(aggregate_pins) == decision['rootSmallReceiptPinsChecked'] == 135, 'exact135 actual aggregate small pins')
    require(all(e['largeRehashPerformed'] is False for e in index['externalEvidence']), 'large-data packaging overclaim')
    for reference in index['externalEvidence'][:2]:
        require(sha(combined[relative(reference['manifestPath'])]) == reference['manifestSha256'], 'external raw manifest pin')
    result = {'ok': True, 'scope': 'root-admitted amended preliminary114 only',
              'archiveSha256': index['archiveSha256'], 'archiveMembers': len(files),
              'uncompressedOriginalBytes': index['memberBytes'], 'selectedSmallOriginals': len(selected),
              'selectedSmallOriginalBytes': sum(e['bytes'] for e in selected),
              'nestedRetainedArchiveMembers': retained_result['archiveMembers'], 'freshSourceArchiveMembers': 59,
              'retainedControls': 17, 'freshJUnitInstances': 97, 'freshChildControls': 16,
              'aggregateNodes': 114, 'aggregateDistinctSmallPins': len(aggregate_pins),
              'originalParentExit': None, 'preservedOuterResponses': len(responses),
              'pollingMetadataGapPreserved': True, 'optimization': sys.flags.optimize,
              'testsExecuted': False, 'archivedCodeExecuted': False, 'largeRawTreesRehashed': False,
              'runtimeArchiveRehashed': False, 'nativeFinal115StillRequired': True, 'a4Complete': False}
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == '__main__':
    main()
