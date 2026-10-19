"""Existing canonical load_task/build_argv and custody only; process creation forbidden."""
from pathlib import Path
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
C = D.parent / 'agy-terminal-live-metadata-20261018'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
def forbidden(*args, **kwargs):
    raise AssertionError('process creation forbidden during preparation preflight')
subprocess.Popen = forbidden
subprocess.run = forbidden
subprocess.check_output = forbidden
sys.path.insert(0, str(C))
spec = importlib.util.spec_from_file_location('canonical_future_runner', C / 'future_runner.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
assert sha(C / 'future_runner.py') == 'c1fbcace3197c1417cd299d0ea794f80f39ba07549eb704be8b849f708ee959c'
assert sha(C / 'agy_terminal_selector.py') == 'ee3536dab1c2e33ec69ce734756973be0019dfa5a8622b27b58fda8371e9051a'
bundle = module.load_task(D / 'TASK.json', sha(D / 'TASK.json'))
assert module.pins_match(bundle['pins'])
readset = json.loads((D / 'READSET.json').read_bytes())
for row in readset['files']:
    path = D / 'packet' / row['path']
    assert path.stat().st_size == row['bytes'] and sha(path) == row['sha256']
    assert len(path.read_text().splitlines()) == row['lines']
    if row['delivery'] == 'inline':
        body = path.read_text()
        block = f"BEGIN {row['id']}\n{body}" + ('' if body.endswith('\n') or not body else '\n') + f"END {row['id']}"
        assert bundle['prompt'].count(block) == 1, row['id']
binding = json.loads((D / 'SOURCE-BINDINGS-FINAL.json').read_bytes())
for row in binding['candidateFiles']:
    path = Path(binding['checkout']) / row['path']
    assert path.stat().st_size == row['bytes'] and sha(path) == row['sha256']
for row in binding['unchangedInterfaces']:
    path = Path(binding['checkout']) / row['path']
    assert path.stat().st_size == row['bytes'] and sha(path) == row['sha256']
for row in json.loads((D / 'INVOKER-AND-CLI-PINS.json').read_bytes())['tools']:
    path = Path(row['configuredPath'])
    assert str(path.resolve()) == row['resolvedPath']
    assert (os.readlink(path) if path.is_symlink() else None) == row['symlinkText']
    assert path.resolve().stat().st_size == row['bytes'] and sha(path.resolve()) == row['sha256']
schema = bundle['schema']
assert len(schema['required']) == 11
assert schema['properties']['reviewToken']['const'] == bundle['task']['reviewToken']
def no_added_string_caps(value):
    if isinstance(value, dict):
        assert 'maxLength' not in value
        for part in value.values():
            no_added_string_caps(part)
    elif isinstance(value, list):
        for part in value:
            no_added_string_caps(part)
no_added_string_caps(schema)
from jsonschema import validators
validators.validator_for(schema).check_schema(schema)
out = Path(bundle['task']['outputDirectory'])
assert not out.exists()
assert bundle['task']['wallSeconds'] == 420
assert bundle['task']['killAfterSeconds'] == 5
assert bundle['task']['parentWallSeconds'] == 425
assert bundle['task']['maxAttempts'] == 1
argv = module.build_argv(bundle, out)
assert argv[:4] == ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '420s']
assert argv[-2:] == ['--print', bundle['prompt']]
assert '--json-schema' in argv and '--sandbox' in argv and '--mode' in argv
assert (D / 'CONTROLLER-DIFF.patch').read_bytes() == b''
print(json.dumps({'status': 'PASS_CANONICAL_LOAD_TASK_BUILD_ARGV_POPEN_FORBIDDEN',
    'taskSha256': sha(D / 'TASK.json'), 'canonicalRunnerSha256': sha(C / 'future_runner.py'),
    'selectorSha256': sha(C / 'agy_terminal_selector.py'), 'inputPins': len(bundle['pins']),
    'requiredWholeBodies': sum(row['requiredWhole'] for row in readset['files']),
    'inlineBodies': sum(row['delivery'] == 'inline' for row in readset['files']),
    'inlineRawBytes': sum(row['bytes'] for row in readset['files'] if row['delivery'] == 'inline'),
    'fileReadBodies': sum(row['delivery'] == 'file-read' for row in readset['files']),
    'fileReadRawBytes': sum(row['bytes'] for row in readset['files'] if row['delivery'] == 'file-read'),
    'promptBytes': len(bundle['prompt'].encode()), 'allFiveCandidatePinsMatch': True,
    'noAddedStringOrReasonCap': True, 'providerCalls': 0, 'productOrGitExecution': False,
    'outputAbsent': True, 'argvWithoutPrompt': argv[:-1]}, indent=2))
