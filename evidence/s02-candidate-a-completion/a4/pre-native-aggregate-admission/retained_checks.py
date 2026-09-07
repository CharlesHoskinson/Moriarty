"""Fixed local audit source adapted from the prior retained17 auditor.

Inputs are in-memory package bytes. Archived source code is never executed.
"""
import ast
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import tarfile


HERE = Path(__file__).resolve().parent
SDD = '.superpowers/sdd/'
R = SDD + 'a4-task6-resumption-20260906/'
A = R + 'admit17/'
L = SDD + 'a4-task6-lifecycle-controls-20260906/'
O = SDD + 'a4-task6-receipts/green-tests/'
TEST = 'tests/test_s02_candidate_a_integrated.py'
HELPER = 'evidence/s02-candidate-a-completion/a4/native-resources/runner.py'
FROZEN = {
    R + 'recorder.py': '63d8f00d024dbafb08fef0d48da96c91ca594bfe33476946ab0960bfd7dcf666',
    R + 'lifecycle_controls.py': '689d767510ae4f3bf610b254bbdef7cd72600846e997593ab243484781ccd7c7',
    R + 'support.json': '9da5267f46318171340d11cd6f25b3baebf98aa75b2312f23f8ecd9f12b76a93',
    R + 'receipt-supplement.md': '01e663d18a91bc57414b319818b82c1e0cf868b31a5c94fbc2a88cc2c3905ca7',
    R + 'reviewed-amendment.md': '15bbb108e2a9cd3ce0c2d1fa458f0bcdb5fd9daf91dcc43976f23a3ac4015ab3',
    SDD + 'a4-task6-recovery-20260906.json': '1437366a220d08a26506d189c508d6fdad21a6d978f724410fec2d6166432f9e',
    SDD + 'a4-task6-admit17-review-20260906.md': 'bc3d430de6ec728ef7e3a0154134993116588dbbef06fbb7837798dd1645b2ec',
    HELPER: 'd8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d',
    A + 'terminal.json': '1db49479a8f3edb3ddd901978dc30ddb9f499dc992703adf1b5ea3193d9af075',
    A + 'outer-tool-receipt.json': 'faf532af46ba5b1641a0c3f2a2a964b766a72085b569d9440b5b0b887eeec02d',
    A + 'retained-controls.json': 'b339b8095bed87c285c11d5a623a39885e897b8911fb03251b689aad7ef43d13',
    A + 'original-artifacts.json': '043f61247e1fdafeab98d74ba9487e578aa386227da2855b9df28b805720ba7b',
}


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def decode(raw):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, 'duplicate JSON key ' + key)
            result[key] = value
        return result
    return json.loads(raw, object_pairs_hook=unique)


def tar_members(raw):
    result = {}
    with tarfile.open(fileobj=io.BytesIO(raw), mode='r:gz') as archive:
        for member in archive.getmembers():
            parts = PurePosixPath(member.name)
            require(member.isfile() and not parts.is_absolute() and '..' not in parts.parts,
                    'unsafe/nonregular archive member')
            require(member.name not in result and member.size <= 2_000_000,
                    'duplicate or oversized archive member')
            result[member.name] = archive.extractfile(member).read()
    return result


def validate_retained(package_files):
    index = decode(package_files['index.json'])
    for entry in index['packageSupport']:
        require(Path(entry['path']).name == entry['path'], 'unsafe package support path')
        require(sha(package_files[entry['path']]) == entry['sha256'], 'package support hash')
    require(index['archive'] == 'original-small-evidence.tar.gz', 'unexpected archive name')
    raw = package_files[index['archive']]
    require(sha(raw) == index['archiveSha256'], 'outer archive hash')
    files = tar_members(raw)
    entries = index['members']
    require(len(entries) == len({e['path'] for e in entries}) == index['memberCount'], 'index duplicates/count')
    require(set(files) == {e['path'] for e in entries}, 'archive exact membership')
    require(sum(len(v) for v in files.values()) == index['memberBytes'], 'archive total bytes')
    for entry in entries:
        require(len(files[entry['path']]) == entry['bytes'] and sha(files[entry['path']]) == entry['sha256'],
                'archive member bytes/hash: ' + entry['path'])
    require(not any('/fresh97/' in name for name in files), 'future fresh97 evidence included')
    require(not any(name.startswith(O + 'basetemp/') and Path(name).name not in
                    ('argv.json', 'stdout.txt', 'stderr.txt', 'terminal.json') for name in files),
            'large original generated artifacts must remain external')
    require(index['scope'] == {'retainedControlsAdmitted': 17, 'freshControlsPending': 97,
                              'preNativeAggregateRequired': 114, 'nativeFinalSuiteRequired': 115,
                              'originalParentExit': None, 'a4Complete': False}, 'scope overclaim')
    require(len(index['limitations']) == 4 and all(index['limitations']), 'missing limitations')
    for name, expected in FROZEN.items():
        require(sha(files[name]) == expected, 'frozen source/receipt hash: ' + name)
    root = index['originalRepositoryRoot']

    def relative(path):
        require(path.startswith(root + '/'), 'non-repository path used as local archived evidence')
        return path[len(root) + 1:]

    def doc(name):
        return decode(files[name])

    def pin_check(pin, lookup=files):
        content = lookup[relative(pin['path'])]
        require(sha(content) == pin['sha256'] and len(content) == pin['bytes'], 'record pin ' + pin['path'])

    admission = doc(A + 'input.json')
    terminal = doc(A + 'terminal.json')
    require(terminal['exitCode'] == 0 and terminal['error'] is None
            and terminal['originalParentExit'] is None and not terminal['artifactFinalizationBlocked'],
            'admission terminal outcome')
    require(terminal['sourceChanged'] == terminal['environmentChanged'] == [], 'after pin changes')
    require(len(terminal['sourceAfter']) == 46 and len(terminal['environmentAfter']) == 3329,
            'source/runtime closure counts')
    require(terminal['sourceAfter'] == admission['sourceBefore']
            and terminal['environmentAfter'] == admission['environmentBefore'], 'endpoint comparison')
    require(admission['originalParentExit'] is None and admission['parentFlags']['optimize'] == 0,
            'original exit/new flag statement')
    for pin in terminal['ownedFiles']:
        pin_check(pin)
    nested = tar_members(files[A + 'source.tar.gz'])
    expected_nested = {e['archivePath']: e for e in admission['sourceBytes']}
    require(len(nested) == 59 and set(nested) == set(expected_nested), 'exact59-member admission source archive')
    source_lookup = {}
    for name, content in nested.items():
        entry = expected_nested[name]
        require(len(content) == entry['bytes'] and sha(content) == entry['sha256'], 'nested source hash')
        source_lookup[relative(entry['path'])] = content
    for name in set(source_lookup) & set(files):
        require(source_lookup[name] == files[name], 'original/nested source disagreement')
    combined = dict(source_lookup, **files)
    for pin in admission['sourceBefore']:
        pin_check(pin, combined)
    support = doc(R + 'support.json')
    require(len(support['files']) == 12, 'new support closure count')
    for pin in support['files']:
        pin_check(pin, combined)
    original = doc(O + 'input.json')
    old_archive = tar_members(files[O + 'source.tar.gz'])
    require(len(old_archive) == 46 and set(old_archive) == {e['archivePath'] for e in original['sourceBytes']},
            'exact46-member original source archive')
    require(sha(files[O + 'source.tar.gz']) == original['sourceArchiveSha256'], 'original source archive pin')
    for pin in original['sourceBytes']:
        require(sha(old_archive[pin['archivePath']]) == pin['sha256']
                and len(old_archive[pin['archivePath']]) == pin['bytes'], 'original source member bytes')
    env = doc(SDD + 'a4-checker-task1-receipts/python-environment.json')
    require(sha(files[SDD + 'a4-checker-task1-receipts/python-environment.json']) ==
            original['pythonEnvironmentManifestSha256'], 'reused runtime manifest')
    require(len(env['sourceBytes']) == 3329 and env['archiveSha256'] ==
            original['pythonEnvironmentArchiveSha256'] == index['externalEvidence'][1]['sha256'],
            'reused runtime archive reference')
    require([(e['path'], e['sha256'], e['bytes']) for e in env['sourceBytes']] ==
            [(e['path'], e['sha256'], e['bytes']) for e in original['environmentBefore']],
            'original runtime manifest binding')

    def outer_check(name, expected_argv, expected_stdout):
        outer = doc(name)
        require(outer['args']['cmd'] == expected_argv and outer['args']['workdir'] == root, 'exact outer command')
        require(outer['responses'][-1]['exit_code'] == 0, 'actual outer tool exit')
        output = ''.join(response['output'] for response in outer['responses'])
        require(decode(output) == expected_stdout, 'actual outer emitted result binding')
    python = '/home/charl/Moriarty/.venv/bin/python'
    command = (python + ' -B -X pycache_prefix=' + root + '/' + R +
               'recorder-cache-admit17 ' + R + 'recorder.py admit17')
    outer_check(A + 'outer-tool-receipt.json', command,
                {'stage': root + '/' + A.rstrip('/'), 'exitCode': 0, 'error': None})
    partition = doc(R + 'partition.json')
    required, retained, fresh = (partition[k] for k in
        ('requiredPreNativeNodeids', 'retainedOriginalChildNodeids', 'freshComplementNodeids'))
    require((len(required), len(retained), len(fresh)) == (114, 17, 97), 'partition counts')
    require(len(set(required)) == 114 and not set(retained) & set(fresh)
            and set(retained) | set(fresh) == set(required), 'exact disjoint partition')
    collected = doc(A + 'collected-nodeids.json')
    require(len(collected) == len(set(collected)) == 115 and
            [n for n in collected if n != partition['nativeDeferredNode']] == required,
            'collection exact115/114 inventory')
    collection = doc(A + 'collection-terminal.json')
    require(collection['exitCode'] == 0 and collection['eligible'] and collection['cleanup']['complete']
            and not collection['cleanup']['forced'] and not collection['cleanup']['errors'],
            'collection-only owned lifecycle')
    collection_input = doc(A + 'collection-input.json')
    require('--collect-only' in collection_input['argv'] and '-p' in collection_input['argv']
            and 'no:cacheprovider' in collection_input['argv'] and not collection_input['childPythonOptimizePresent'],
            'collection-only command/cache/optimization')
    require(len(terminal['commandLifecycles']) == 1, 'admission must not execute semantic commands')
    controls = doc(A + 'retained-controls.json')
    require(controls['originalParentExit'] is None and [c['nodeid'] for c in controls['controls']] == retained,
            'retained control set/original parent unknown')
    test_source = old_archive[next(e['archivePath'] for e in original['sourceBytes']
                                  if e['path'] == root + '/' + TEST)]
    module = ast.parse(test_source)
    function = next(n for n in module.body if isinstance(n, ast.FunctionDef) and n.name == 'task6_process')
    runner = next(ast.literal_eval(n.value) for n in function.body if isinstance(n, ast.Assign)
                  and any(isinstance(t, ast.Name) and t.id == 'script' for t in n.targets))
    for number, control in enumerate(controls['controls']):
        folder = O + 'basetemp/test_semantic_rebound_triples_' + str(number) + '/'
        absolute = root + '/' + folder.rstrip('/')
        name = retained[number].split('[', 1)[1][:-1]
        expected_argv = [python, '-B', '-X', 'pycache_prefix=' + absolute + '/fresh-cache',
                         '-c', runner, root + '/' + TEST, 'semantic', name, absolute]
        require(doc(folder + 'argv.json') == control['argv'] == expected_argv, 'exact original child argv')
        require(doc(folder + 'terminal.json') == {'exit': 0} and control['originalChildExit'] == 0,
                'authentic original child exit')
        require(files[folder + 'stderr.txt'] == b'', 'original child stderr')
        report = doc(folder + 'stdout.txt')
        require(report == control['report'] and report['name'] == name and report['mode'] == 'semantic'
                and type(report['changed_events']) is int and report['changed_events'] > 0
                and all(report[k] is True for k in ('raw_linkage', 'mutant_rejected', 'corrected', 'unrelated')),
                'effective mutation/positive controls')
        for pin in control['records'].values():
            pin_check(pin)
    require(not (O + 'terminal.json') in files, 'invented original parent terminal')
    partial = O + 'basetemp/test_semantic_rebound_triples_17/'
    require(partial + 'terminal.json' not in files and files[partial + 'stdout.txt'] == b'',
            'partial child status was promoted')
    require(any('[fabricated-cancellation-core]' in n for n in fresh), 'partial child missing from fresh complement')
    witness = controls['assertionIntegrityEvidence']
    require(witness['directOptimizeFlagRecorded'] is False
            and witness['directPythonOptimizeEnvironmentRecorded'] is False
            and witness['testSourceSha256'] == sha(test_source) and len(witness['assumptions']) == 3,
            'historical assertion inference limitations')
    rebound = next(n for n in module.body if isinstance(n, ast.FunctionDef) and n.name == 'task6_rebind')
    require(ast.unparse(rebound.body[0]) == witness['mkdirStatement'] == 'folder.mkdir()', 'fresh-directory witness')
    assertion = next(n for n in rebound.body if ast.unparse(n) == witness['rawWriterStatement'])
    require(isinstance(assertion, ast.Assert)
            and ast.unparse(rebound.body[rebound.body.index(assertion) + 1]) ==
            witness['followingStatement'] == 'sha = file_digest(raw)', 'mandatory raw writer witness')
    manifest = doc(A + 'original-artifacts.json')
    regular = [e for e in manifest['entries'] if e['kind'] == 'file']
    require(len(manifest['entries']) == 422 and len(regular) == 318
            and sum(e['bytes'] for e in regular) == 9795796111
            and sum(e['kind'] == 'directory' for e in manifest['entries']) == 88
            and sum(e['kind'] == 'symlink' for e in manifest['entries']) == 16, 'raw manifest accounting')
    recorded = {e['path']: e for e in regular}
    for name in files:
        if name.startswith(O):
            entry = recorded[root + '/' + name]
            require(sha(files[name]) == entry['sha256'] and len(files[name]) == entry['bytes'],
                    'archived small original record/raw manifest binding')
    require(all(e['packagingRehashPerformed'] is False for e in index['externalEvidence']), 'external rehash overclaim')
    lifecycle = doc(L + 'terminal.json')
    outer_check(L + 'outer-tool-receipt.json', python + ' -B ' + R + 'lifecycle_controls.py', lifecycle)
    require(lifecycle['ok'] and lifecycle['error'] is None and lifecycle['sourceStable']
            and lifecycle['fallbackCleanup'] == [] and len(lifecycle['controls']) == 4, 'four lifecycle controls')
    expected_lifecycle = [('normal', 0, True), ('exception-after-spawn', -15, False),
                          ('leader-exit-descendant', 0, False), ('sigterm-during-wait', -15, False)]
    for name, code, eligible in expected_lifecycle:
        result = doc(L + name + '/control-terminal.json')
        require(result['exitCode'] == code and result['eligible'] is eligible
                and result['cleanup']['complete'] and not result['cleanup']['errors'], 'lifecycle actual result')
        require(result['cleanup']['forced'] is (not eligible), 'forced lifecycle eligibility')
        require(doc(L + name + '/child-flags.json') == {'optimize': 0}, 'real child optimization flag')
        pin_check(result['stdout'])
        pin_check(result['stderr'])
    heartbeat = doc(L + 'leader-exit-descendant/control-result.json')['observations']
    require(heartbeat['descendantStateAfter'] is None
            and heartbeat['heartbeatBytesBeforeAfter'][0] == heartbeat['heartbeatBytesBeforeAfter'][1],
            'descendant stopped writing')
    decision = doc(R + 'root-admission17.json')
    require(decision['originalParentExit'] is None and len(decision['limitations']) == 3,
            'root limitation fields')
    require(decision['admissionTerminalSha256'] == sha(files[A + 'terminal.json'])
            and decision['admissionOuterReceiptSha256'] == sha(files[A + 'outer-tool-receipt.json'])
            and decision['retainedControlsSha256'] == sha(files[A + 'retained-controls.json'])
            and decision['originalArtifactManifestSha256'] == sha(files[A + 'original-artifacts.json']),
            'root decision original-receipt pins')
    result = {'ok': True, 'scope': 'root-admitted retained17 small archive only',
              'archiveSha256': index['archiveSha256'], 'archiveMembers': len(files),
              'archiveUncompressedBytes': index['memberBytes'], 'nestedSourceMembers': 59,
              'originalSourceMembers': 46, 'retainedControls': 17, 'freshPending': 97,
              'collectionOnlyNodes': 115, 'requiredPreNative': 114, 'lifecycleControls': 4,
              'recordedOriginalFiles': 318, 'recordedOriginalBytes': 9795796111,
              'originalParentExit': None, 'externalRawRehashPerformed': False,
              'externalRuntimeRehashPerformed': False, 'testsExecuted': False,
              'archivedCodeExecuted': False, 'a4Complete': False}
    return result

