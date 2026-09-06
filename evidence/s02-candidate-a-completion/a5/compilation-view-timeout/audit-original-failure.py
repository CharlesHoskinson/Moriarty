"""Standalone archive-only audit. No original workspace or runtime execution."""
import hashlib
import json
import posixpath
import re
import tarfile
import tempfile
from pathlib import Path

PACKAGE = Path(__file__).resolve().parent
CONTROL = '.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/'
PLAN = 'docs/superpowers/plans/2026-09-06-candidate-a-pilot-compilation-view.md'
BASE = '43f33721da7a3bfb11dd1a91f1f548d5110ab93c'
BOOTSTRAP = '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
STAGES = ['typecheck', 'prefix-test', 'sample-original', 'sample-factored', 'compile-original']

def digest(stream):
    return hashlib.file_digest(stream, 'sha256').hexdigest()

manifest = json.loads((PACKAGE / 'archive-manifest.json').read_text())
assert manifest['partLimitBytes'] == 16 * 1024 * 1024
expected = {row['path']: row for row in manifest['members']}
assert len(expected) == len(manifest['members'])
assert json.loads((PACKAGE / 'original-inputs.json').read_text()) == manifest['members']
with tempfile.TemporaryFile() as combined:
    for number, part in enumerate(manifest['parts']):
        assert part['path'] == f'original-receipts.tar.part-{number:03d}'
        path = PACKAGE / part['path']
        assert 0 < part['bytes'] <= manifest['partLimitBytes']
        assert path.stat().st_size == part['bytes']
        with path.open('rb') as source:
            assert digest(source) == part['sha256']
            source.seek(0)
            while data := source.read(1024 * 1024):
                combined.write(data)
    assert combined.tell() == manifest['archiveBytes']
    combined.seek(0)
    assert digest(combined) == manifest['archiveSha256']
    combined.seek(0)
    with tarfile.open(fileobj=combined, mode='r:') as archive:
        members = archive.getmembers()
        by_name = {m.name: m for m in members}
        assert len(by_name) == len(members) and by_name.keys() == expected.keys()
        for member in members:
            assert member.isfile() and member.size == expected[member.name]['bytes']
            assert digest(archive.extractfile(member)) == expected[member.name]['sha256']

        def read(name):
            return archive.extractfile(by_name[name]).read()

        def js(name):
            return json.loads(read(name))

        assert expected[PLAN]['sha256'] == '0621f1e3bcf002ec7d59f8ae90c38c0572a3c849dbc1a88bc3fd85c03954ab54'
        index_name = CONTROL + 'original-file-hash-index.json'
        assert expected[index_name]['sha256'] == '09091e9c0554babe37072722cfef17ff8b67fa650ae7f02d1f85d247ac483017'
        index = js(index_name)
        assert len(index['files']) == 81
        for row in index['files']:
            assert expected[row['path']]['sha256'] == row['sha256']
            assert expected[row['path']]['bytes'] == row['bytes']
        controls = {name for name in by_name if name.startswith(CONTROL)}
        assert controls == {row['path'] for row in index['files']} | {
            index_name, CONTROL + 'handoff-outer-command.json'}
        assert len(controls) == 83
        assert expected[CONTROL + 'handoff-outer-command.json']['sha256'] == 'e41fd2f7d1bc3ce9d6ab07a5c9ba505b0099ba87a127f3ec0fc3de576f56627c'
        for name in ['prepare-outer-command.json', 'dispatch-outer-command.json', 'handoff-outer-command.json']:
            outer = js(CONTROL + name)
            assert outer['wrapperExitCode'] == outer['toolResponses'][-1]['exit_code'] == 0
        dispatch = js(CONTROL + 'dispatch.json')
        assert dispatch['actualDispatchBase'] == BASE and dispatch['runtimeBootstrapBase'] == BOOTSTRAP
        for name, value in dispatch['sources'].items():
            assert expected[name]['sha256'] == value
        blocks = re.findall(r'```python\n(.*?)```', read(PLAN).decode(), re.S)
        for name, source in zip(['prepare.py', 'record.py'], blocks):
            assert read(CONTROL + name) == source.encode()
        view = js(CONTROL + 'view-manifest.json')
        assert len(view['files']) == 24 and view['task2FrozenCount'] == 30
        freeze_name = '.superpowers/sdd/a5-factoring-receipts/task2-frozen-source.json'
        freeze = js(freeze_name)['files']
        assert len(freeze) == 30
        original_paths = {row['original'] for row in view['files']} | {row['path'] for row in freeze}
        collected = {}
        summaries = []
        for number, stage in enumerate(STAGES):
            prefix = CONTROL + stage + '/'
            metadata = js(prefix + 'input.json')
            result = js(prefix + 'result.json')
            validation = js(prefix + 'view-validation.json')
            outer = js(prefix + 'outer-command.json')
            assert metadata['sourceCommit'] == BASE and metadata['beforeDispatchCommit'] == BOOTSTRAP
            assert metadata['limits'] == {'wallSeconds': 900 if stage == 'compile-original' else 1200,
                                         'jvmHeapMiB': 4096, 'nodeHeapMiB': 4096}
            assert result['runtimeUnchanged'] and result['sourceAndToolsUnchanged']
            assert validation['checks']['viewAndOriginalPinsStable']
            assert outer['wrapperExitCode'] == outer['toolResponses'][-1]['exit_code']
            emitted = json.loads(''.join(response['output'] for response in outer['toolResponses']))
            assert emitted == {'stage': 'pilot-compilation-view/' + stage, **result}
            assert validation['actualDispatchBase'] == BASE and validation['runtimeBootstrapBase'] == BOOTSTRAP
            assert validation['dispatchSha256'] == expected[CONTROL + 'dispatch.json']['sha256']
            output_name = 'input.qnt.json' if stage == 'compile-original' else 'stdout.txt'
            assert result['stdoutSha256'] == expected[prefix + output_name]['sha256']
            assert result['stderrSha256'] == expected[prefix + 'stderr.txt']['sha256']
            nested_name = prefix + 'source-and-runner.tar.gz'
            assert metadata['sourceArchiveSha256'] == expected[nested_name]['sha256']
            pins = {row['path']: row['sha256'] for row in metadata['pins']}
            assert len(pins) == len(metadata['pins'])
            cwd = metadata['cwd'] + '/'
            repo_pins = {name[len(cwd):]: value for name, value in pins.items() if name.startswith(cwd)}
            nested_hashes = {}
            with tarfile.open(fileobj=archive.extractfile(by_name[nested_name]), mode='r|gz') as nested:
                for member in nested:
                    assert member.isfile() and member.name not in nested_hashes
                    stream = nested.extractfile(member)
                    if stage == 'compile-original' and member.name in original_paths:
                        data = stream.read()
                        collected[member.name] = data
                        value = hashlib.sha256(data).hexdigest()
                    else:
                        value = digest(stream)
                    nested_hashes[member.name] = value
            assert nested_hashes == repo_pins
            assert len(repo_pins) == 146 + 10 * number and len(pins) == len(repo_pins) + 8
            for name, value in nested_hashes.items():
                if name in expected:
                    assert expected[name]['sha256'] == value
            if stage == 'compile-original':
                assert result['exitCode'] == -15 and result['timedOut'] is True
                assert validation['actualCommandExitCode'] == 124 and validation['ok'] is False
                assert outer['wrapperExitCode'] == 1
                for name in ['input.qnt.json', 'stderr.txt', 'resources.txt']:
                    assert read(prefix + name) == b''
                for row in view['originalPins']:
                    if row['path'].startswith(cwd):
                        assert nested_hashes[row['path'][len(cwd):]] == row['sha256']
                for row in view['references']:
                    assert nested_hashes[row['path']] == row['sha256']
            else:
                assert result['exitCode'] == validation['actualCommandExitCode'] == outer['wrapperExitCode'] == 0
                assert not result['timedOut'] and validation['ok'] and all(validation['checks'].values())
                assert read(prefix + 'stderr.txt') == b''
                if stage == 'typecheck':
                    assert read(prefix + 'stdout.txt') == b''
                if stage == 'prefix-test':
                    stdout = read(prefix + 'stdout.txt').decode()
                    assert 'ok allPilotPrefixesTest passed 1 test(s)' in stdout
                    assert re.search(r'\b1 passing\b', stdout)
                    assert validation['details']['quantifiedPrefixPairs'] == 22
                if stage.startswith('sample-'):
                    stdout = read(prefix + 'stdout.txt').decode()
                    assert '[ok] No violation found' in stdout
                    counts = re.findall(r'^(\w+) was witnessed in (\d+) trace\(s\) out of (\d+) explored', stdout, re.M)
                    assert len(counts) == 8
                    assert {name: (int(count), int(total)) for name, count, total in counts} == {
                        'prepared': (100, 100), 'signed': (100, 100), 'proposed': (100, 100),
                        'verified': (50, 100), 'committed': (50, 100), 'rejected': (50, 100),
                        'afterCompleted': (53, 100), 'beforeCompleted': (47, 100)}
            summaries.append({'stage': stage, 'timeWrappedChildExit': result['exitCode'],
                              'helperReturn': validation['actualCommandExitCode'],
                              'actualOuterExit': outer['wrapperExitCode'],
                              'sourceArchiveMembers': len(repo_pins), 'sourceAndToolPins': len(pins)})
        assert not any(name.startswith(CONTROL + 'compile-factored/') for name in by_name)
        assert collected.keys() == original_paths
        for row in freeze:
            assert hashlib.sha256(collected[row['path']]).hexdigest() == row['sha256']
        additions = 0
        copied = {}
        for row in view['files']:
            original = collected[row['original']]
            changed = read(row['view'])
            assert hashlib.sha256(original).hexdigest() == row['originalSha256']
            assert hashlib.sha256(changed).hexdigest() == row['viewSha256']
            if row['addedImport']:
                line = row['addedImport'].encode()
                wanted = (b'  import consumption.AuthorityKey from "./consumption"\n'
                          if row['original'].endswith('/candidate_a_authority_adapter.qnt') else
                          b'  import consumption.AuthorityKey from "../consumption"\n')
                assert line == wanted and changed.count(line) == 1
                header, body = original.split(b'\n', 1)
                assert changed == header + b'\n' + line + body
                additions += 1
            else:
                assert changed == original
            copied[row['original']] = changed
        assert additions == 2
        seen = set()
        pending = ['specs/quint/s02/' + name for name in view['roots']]
        while pending:
            name = pending.pop()
            if name in seen:
                continue
            seen.add(name)
            for line in copied[name].decode().splitlines():
                if re.match(r'^\s*import\b', line):
                    target = re.search(r'\bfrom "([^"]+)"', line)[1] + '.qnt'
                    destination = posixpath.normpath(posixpath.join(posixpath.dirname(name), target))
                    assert destination in copied
                    pending.append(destination)
        assert seen == copied.keys()
        intake = js('.superpowers/sdd/a5-pilot-view-task2-root-intake.json')
        assert len(intake['stages']) == 4
        for row in intake['stages']:
            for name, value in row['originalReceipts'].items():
                assert expected[CONTROL + row['stage'] + '/' + name]['sha256'] == value
        timeout_name = '.superpowers/sdd/a5-pilot-view-timeout-root-intake.json'
        assert expected[timeout_name]['sha256'] == '6acf80c85a41c4e475f0da29dd225ee137cb3968d29c120d1256519367f81eb0'
        timeout = js(timeout_name)
        assert timeout['timeWrappedChildExit'] == -15 and timeout['helperTimeoutReturn'] == 124 and timeout['actualOuterExit'] == 1
        assert timeout['nativeFrontendElapsedFromGNUTime'] is None and timeout['nativeMaxRssFromGNUTime'] is None
        for name, value in timeout['originalReceipts'].items():
            assert expected[CONTROL + 'compile-original/' + name]['sha256'] == value
        for name, value in timeout['handoff'].items():
            assert expected[CONTROL + name]['sha256'] == value
        shared = index['sharedEnvironmentReferences']
        assert len(shared) == 4
        for item in shared:
            name = item['path'][len(cwd):] if item['path'].startswith(cwd) else item['path'].lstrip('/')
            assert name not in by_name
print(json.dumps({'ok': True, 'scope': 'Archive-only preservation audit; no runtime/native/checker execution',
                  'archiveSha256': manifest['archiveSha256'], 'archiveBytes': manifest['archiveBytes'],
                  'parts': len(manifest['parts']), 'members': len(expected), 'controlOriginals': 83,
                  'originalIndexEntries': 81, 'postIndexOuterPreservedSeparately': True,
                  'viewModules': 24, 'exactImportAdditions': 2, 'unchangedTask2Files': 30,
                  'stages': summaries, 'nativeCompileGNUTimeElapsed': None, 'nativeCompileMaxRss': None,
                  'factoredCompilePerformed': False, 'solverPerformed': False, 'H1': 'unresolved',
                  'sharedEnvironmentReferences': shared}, indent=2))
