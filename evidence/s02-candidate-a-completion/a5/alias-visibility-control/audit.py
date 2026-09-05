"""Audit retained frontend-control evidence; never invoke a compiler or solver."""
import hashlib
import json
from pathlib import Path
import re
import tarfile

ROOT = Path(__file__).resolve().parents[4]
CONTROL = ROOT / '.superpowers/sdd/a5-factoring-receipts/alias-visibility-control'
PLAN = ROOT / '.superpowers/sdd/a5-alias-visibility-control-plan.md'


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def read(path):
    return json.loads(path.read_text())


def normalized(value):
    if isinstance(value, dict):
        return {k: normalized(v) for k, v in value.items() if k != 'id'}
    if isinstance(value, list):
        return [normalized(v) for v in value]
    return value


def audit():
    dispatch = read(CONTROL / 'dispatch.json')
    assert sha(PLAN) == dispatch['planSha256']
    assert dispatch['actualDispatchBase'] == '9ccbf0ed571e4055bc05962e5681fdf2fa75ad96'
    assert dispatch['runtimeBootstrapBase'] == '900bb2051225b4a3d99bf422c3b2e5e386e3e7bc'
    assert all(sha(ROOT / p) == h for p, h in dispatch['sources'].items())
    blocks = re.findall(r'```quint\n(.*?)```', PLAN.read_text(), re.S)
    for name, block in zip(('keys', 'generic', 'driver_original', 'driver_direct'), blocks, strict=True):
        assert (CONTROL / 'src' / (name + '.qnt')).read_bytes() == block.encode()
    recorder = re.findall(r'```python\n(.*?)```', PLAN.read_text(), re.S)
    assert (CONTROL / 'record.py').read_bytes() == recorder[0].encode()
    stages = {}
    for stage, expected in [('original-typecheck', 0), ('original-compile', 1),
                            ('direct-typecheck', 0), ('direct-compile', 0)]:
        folder = CONTROL / stage
        inp, result = read(folder / 'input.json'), read(folder / 'result.json')
        validation, outer = read(folder / 'control-validation.json'), read(folder / 'outer-command.json')
        assert result['exitCode'] == expected and result['timedOut'] is False
        assert result['sourceAndToolsUnchanged'] and result['runtimeUnchanged']
        assert validation['ok'] and all(validation['checks'].values())
        assert validation['dispatchSha256'] == sha(CONTROL / 'dispatch.json')
        assert validation['actualCommandExitCode'] == expected
        assert outer['wrapperExitCode'] == outer['toolResponses'][-1]['exit_code'] == 0
        assert inp['limits'] == {'wallSeconds': 120, 'jvmHeapMiB': 4096, 'nodeHeapMiB': 4096}
        assert inp['sourceCommit'] == dispatch['actualDispatchBase']
        assert inp['beforeDispatchCommit'] == dispatch['runtimeBootstrapBase']
        pins = {p['path']: p['sha256'] for p in inp['pins']}
        assert all(sha(Path(p)) == h for p, h in pins.items())
        archive = folder / 'source-and-runner.tar.gz'
        assert sha(archive) == inp['sourceArchiveSha256']
        with tarfile.open(archive) as source:
            members = source.getmembers()
            assert len({m.name for m in members}) == len(members)
            for member in members:
                assert member.isfile() and not Path(member.name).is_absolute()
                assert '..' not in Path(member.name).parts
                assert hashlib.file_digest(source.extractfile(member), 'sha256').hexdigest() == pins[str(ROOT / member.name)]
        compiling = stage.endswith('-compile')
        output = folder / ('input.qnt.json' if compiling else 'stdout.txt')
        assert sha(output) == result['stdoutSha256']
        assert sha(folder / 'stderr.txt') == result['stderrSha256']
        if expected == 1:
            assert output.stat().st_size == 0
            error = (folder / 'stderr.txt').read_text()
            assert re.findall(r'Error \[(QNT[0-9]+)\]: ([^\n]+)', error) == [('QNT404', "Type alias 'Key' not found")]
            assert 'error: name resolution failed' in error
        else:
            assert (folder / 'stderr.txt').read_bytes() == b''
        stages[stage] = {'childExit': expected, 'wrapperExit': 0,
                         'sourceMembers': len(members), 'resultSha256': sha(folder / 'result.json')}
    generated = read(CONTROL / 'direct-compile/input.qnt.json')
    assert generated['main'] == 'alias_visibility_direct'
    assert generated['errors'] == generated['warnings'] == []
    assert len(generated['modules']) == 1
    module = generated['modules'][0]
    assert module['name'] == generated['main']
    declarations = {d['name']: d for d in module['declarations']}
    assert list(declarations) == ['Concrete', 'box', 'init', 'step', 'safety', 'q::init', 'q::step', 'q::inv']
    for name, target in [('q::init', 'init'), ('q::step', 'step')]:
        assert declarations[name]['qualifier'] == 'action'
        assert normalized(declarations[name]['expr']) == {'kind': 'name', 'name': target}
    assert normalized(declarations['q::inv']['expr']) == {
        'kind': 'app', 'opcode': 'and', 'args': [{'kind': 'name', 'name': 'safety'}]}
    assert declarations['init']['expr']['opcode'] == declarations['step']['expr']['opcode'] == 'assign'
    assert normalized(declarations['step']['expr']['args']) == [{'kind': 'name', 'name': 'box'}] * 2
    assert declarations['safety']['expr']['opcode'] == 'and'
    assert len(declarations['safety']['expr']['args']) == 2
    return {'ok': True, 'scope': 'Four-command generic-alias frontend control only; no actual-model remedy or model checking',
            'stages': stages, 'generatedBytes': (CONTROL / 'direct-compile/input.qnt.json').stat().st_size,
            'generatedSha256': sha(CONTROL / 'direct-compile/input.qnt.json')}


if __name__ == '__main__':
    print(json.dumps(audit(), indent=2))
