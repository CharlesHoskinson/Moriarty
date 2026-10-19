# PROPOSED ordinary Root capture composition, NOT EXECUTED by preparer.
# Existing helper/API class only. Root selects execution; no new framework.
from pathlib import Path
import json, hashlib, os, re, runpy, datetime
D = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019')
CONTROLS = {'proposal': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/VALIDATION-PROPOSAL.json', 'bytes': 8743, 'sha256': '3a4b770e4d8504a6cee548eab908ff270f98f385d913c5ded3fb3f967ec691f3'}, 'flat': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/ROOT-FLAT-PINS.proposed.json', 'bytes': 1131364, 'sha256': '6438592cb0366376af8287576fc2fa20b9451ba453a630911f5c5c9c06b55468'}, 'helper': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/capture-helper.FULL.py', 'bytes': 13595, 'sha256': '713119d9400f82d559967e4296313cec8cde7125dc91a74152500577ced00626'}, 'expectations': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/EXPECTATIONS-FROZEN-BEFORE-EXECUTION.json', 'bytes': 1930, 'sha256': '2dec11062bab1f99175a9cb6c1525e1f47fb74c3475c1bc76f66406f30ec7edf'}, 'review': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/actual-opus-B23-review-01/FINAL-RESPONSE.txt', 'bytes': 17219, 'sha256': '417326ef54951204e6d12cf4893b86b29c356a4a7858b4a7fadaab41d6bec4f1'}, 'reviewResult': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/actual-opus-B23-review-01/RESULT.json', 'bytes': 908, 'sha256': '91f8d1cb9b10456b094042b032618695b63819aa702fa1e431b7d34e896e2ee3'}, 'reviewCommand': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/actual-opus-B23-review-01/COMMAND.json', 'bytes': 1000, 'sha256': '57836f92abf1c714003fd25986a48664354b5a49e097b39e0d5802c68b79fbe8'}, 'reviewStdout': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/actual-opus-B23-review-01/stdout.json', 'bytes': 19362, 'sha256': 'adf620d8a28c4653fc60423182465951cee6ac07b89cc01e82b0f8539c7875d3'}, 'reviewStderr': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/actual-opus-B23-review-01/stderr.bin', 'bytes': 0, 'sha256': 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'}, 'reviewStdin': {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch23-parametric-guard-discriminators-current-source-20261019/PROMPT.proposed.txt', 'bytes': 261532, 'sha256': '5162791ea4d0633f126b90a9989b540ffd59a220885c4fd930046ae156d1a505'}}

def pin(path):
    path = Path(path)
    raw = path.read_bytes()
    return dict(path=str(path), bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())

def checked(row):
    actual = pin(row['path'])
    assert actual == row, row['path']
    return actual

# Bind ALL reviewed controls before JSON load / helper code load.
initial_controls = {key: checked(row) for key, row in CONTROLS.items()}
plan = json.loads(Path(CONTROLS['proposal']['path']).read_bytes())
flat = json.loads(Path(CONTROLS['flat']['path']).read_bytes())
review_result = json.loads(Path(CONTROLS['reviewResult']['path']).read_bytes())
assert review_result['exitCode'] == 0 and review_result['providerIsError'] is False
assert review_result['subjectExecution'] is False
assert review_result['actualModelUsage']['claude-opus-5-5']['canonicalModel'] == 'claude-opus-5-5'
api = runpy.run_path(CONTROLS['helper']['path'], run_name='capture_only')
assert len(plan['operations']) == 2
assert [op['id'] for op in plan['operations']] == [
    '01-directFrontend-literalParametricGuardCoverageTest', '02-twelveNewGuardDiscriminators']
assert len(flat['files']) == 4675 and len(flat['symlinks']) == 3
assert plan['fullRawPinChecksEachPrePost'] == 4678
assert plan['sourceExactEntries'] == [
    'literalParametricGuardCoverageTest.qnt', 'literalParametricImages.qnt', 'literalParametricInputs.qnt']
assert len(plan['sourceFiles']) == 3
assert {x['path']:x for x in flat['files'] if x['path'].startswith(str(D/'source')+'/')} == {
    x['path']:x for x in plan['sourceFiles']}
assert CONTROLS['expectations'] == plan['expectationControl']['pin']
assert CONTROLS['helper'] == plan['captureHelper']
for op in plan['operations']:
    assert op['cwd'] == str(D/'source')
    assert op['environment'] == {'PATH':'/usr/bin:/bin', 'LANG':'C.UTF-8'}
    assert op['stdin'] == 'DEVNULL' and op['wallSeconds'] == 60
    assert op['maxStdoutBytes'] == op['maxStderrBytes'] == 4194304
    assert op['argv'][1] == '--max-old-space-size=512'

phase_paths = [D/('actual-'+op['id']) for op in plan['operations']]
aggregate = D/'ROOT-ACTUAL-TWO-OPERATIONS-OUTCOME.json'
selection_path = D/'ROOT-ACTUAL-TWO-SELECTION.json'
assert not any(p.exists() for p in phase_paths)
assert not aggregate.exists() and not selection_path.exists()

def checks():
    controls = {key: checked(row) for key,row in CONTROLS.items()}
    result = api['static_check'](flat['files'])
    result['symlinks'] = [dict(path=x['path'], match=(
        Path(x['path']).is_symlink() and os.readlink(x['path']) == x['target']
        and str(Path(x['path']).resolve(strict=True)) == x['resolved']
        and Path(x['path']).is_dir() == x['isDirectory'])) for x in flat['symlinks']]
    result['exactSourceDirectoryMatch'] = sorted(x.name for x in (D/'source').iterdir()) == plan['sourceExactEntries']
    result['rawChecks'] = len(result['files']) + len(result['symlinks'])
    result['separateReviewedControlChecks'] = controls
    result['separateExpectationFullPin'] = controls['expectations']
    result['all_match'] = (result['all_match'] and all(x['match'] for x in result['symlinks'])
        and result['exactSourceDirectoryMatch'] and result['rawChecks'] == 4678)
    return result

api['save'](selection_path, dict(
    utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    selection='Root-executed exact two-operation reviewed ordinary composition',
    composition=pin(__file__), controls=initial_controls,
    operations=plan['operations'], oldPassingCohortSelected=False,
    review=CONTROLS['review'], retry=False,
    scope='B23 finite/source-exposed model tests only; D1-D7 and full remaining coverage retained; no Rust/crosslayer/native/recursive/EARS claim.'))
observed = []
for op, phase in zip(plan['operations'], phase_paths):
    phase.mkdir(mode=0o700, exist_ok=False)
    before = checks()
    api['save'](phase/'ROOT-PRECHECK.json', before)
    assert before['all_match']
    actualop = dict(op, keptBytesPerStream=4194304)
    result = api['capture'](actualop, phase)
    # Raw COMMAND and streams saved by capture; RESULT saved immediately, before checks/assertions.
    api['save'](phase/'RESULT.json', result)
    after = checks()
    api['save'](phase/'ROOT-POSTCHECK.json', after)
    assert after['all_match']
    assert result['actual_exit'] == 0 and result['controller_failure'] is None
    assert len(result['streams']) == 2
    for stream in result['streams']:
        assert stream['full_stream_retained'] is True
        assert stream['bytes_seen'] == stream['bytes_retained'] == stream['bytes']
    out = (phase/'stdout.bin').read_bytes()
    err = (phase/'stderr.bin').read_bytes()
    assert err == b''
    assert not re.search(rb'QNT\d+|\[DEBUG\]|error:', out, re.I)
    if op['id'] == '01-directFrontend-literalParametricGuardCoverageTest':
        assert out == b''
    elif op['id'] == '02-twelveNewGuardDiscriminators':
        text = out.decode('utf-8')
        # Exact retained B21/B22 regex conditions; only count/names/header parameter changes.
        names = re.findall(r'^\s*ok (\w+) passed 1 test\(s\)\s*$', text, re.M)
        assert names == op['expected']['exactPassingNames'] and len(names) == 12
        assert len(re.findall(r'^\s*ok ', text, re.M)) == 12
        assert len(re.findall(r'^\s*12 passing \(\d+ms\)\s*$', text, re.M)) == 1
        assert not re.search(r'failed|ignored|pending|QNT|DEBUG|error:|truncat', text, re.I)
        other = []
        for line in text.splitlines():
            stripped = line.strip()
            if not stripped: continue
            if re.fullmatch(r'ok (\w+) passed 1 test\(s\)', stripped): continue
            if re.fullmatch(r'12 passing \(\d+ms\)', stripped): continue
            other.append(stripped)
        assert other == ['literalParametricGuardCoverageTest'], other
        api['save'](phase/'ROOT-EXACT-TEST-PARSER.json', dict(
            names=names, moduleHeader=other, count=12, stderrBytes=0,
            parserProvenance='Retained B21 passive readback/B22 accepted exact parser, parameter12/newnames/header; B21 original execution cell unavailable.',
            finiteSourceExposed=True, D7SpaceTwoNotIsolated=True))
    else:
        raise AssertionError('unknown operation ID')
    observed.append(dict(id=op['id'],exit=result['actual_exit'],seconds=result['elapsed_seconds'],
        fullStreams=result['streams'],rawChecksBefore=before['rawChecks'],rawChecksAfter=after['rawChecks'],
        separateExpectationBefore=before['separateExpectationFullPin'],separateExpectationAfter=after['separateExpectationFullPin']))
    print(json.dumps(observed[-1]),flush=True)
api['save'](aggregate, dict(
    utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    status='PASS_TWO_OPERATIONS_TWELVE_NEW_TESTS', operations=observed,
    sourceReview=CONTROLS['review'], sourceDirectoryExactThreeMembers=True,
    rawSourceToolSymlinkPinChecksEachPrePost=4678,
    expectationJsonSeparateFullPinCheckedEachPrePost=True,
    oldPassingCohortReplayed=False, sourceChanged=False,
    D7='space2 case is overdetermined; isolated space-domain refusal remains unobserved',
    scope='Finite new model tests; same-author/source-exposed. D1-D7 and remaining review gaps retained. No Rust/crosslayer/native/crypto/recursive/EARS evidence.'))
