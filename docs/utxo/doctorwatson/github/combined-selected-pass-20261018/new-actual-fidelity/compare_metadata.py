"""Read existing public artifacts; compare JSON/hash/text only. No product imports/children.

This is an artifact comparison, not a replacement runtime, oracle or review vote.
All writes are confined to this new external output directory.
"""
from pathlib import Path
import hashlib
import json
import re

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
D = R / 'combined-financial-quantity-observer-fixed-actual-fidelity-20261018'
W = Path('/home/charl/Moriarty/.worktrees/utxo-quantity-min-main-20261018')
C = R / 'combined-financial-quantity-observer-fixed-finite600-check-cycle-20261018'
P = R / 'combined-financial-quantity-observer-fixed-one600-proposal-20261018/v2'
F = W / 'experiments/moriarty-language/regressions/fixtures/source6-capped-remittance'
O = F / 'oracle'
N = C / 'node-observations'
H = W / 'experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs'


def sha(b):
    return hashlib.sha256(b).hexdigest()


def unique_object(pairs):
    out = {}
    for key, value in pairs:
        assert key not in out, ('duplicate JSON key', key)
        out[key] = value
    return out


def load(p):
    return json.loads(p.read_bytes(), object_pairs_hook=unique_object)


def write(name, obj):
    p = D / name
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(obj, ensure_ascii=False, indent=2) + '\n')


def pin(p, relative_to):
    b = p.read_bytes()
    return {'path': p.relative_to(relative_to).as_posix(), 'bytes': len(b), 'sha256': sha(b)}


def snapshot_tree(root, name):
    out = []
    for p in sorted(root.rglob('*')):
        if p.is_file():
            assert not p.is_symlink()
            info = pin(p, root)
            dest = D / name / info['path']
            dest.parent.mkdir(parents=True, exist_ok=True)
            b = p.read_bytes()
            dest.write_bytes(b)
            assert dest.read_bytes() == b
            out.append(info)
    return out


def esc(s):
    return str(s).replace('~', '~0').replace('/', '~1')


def leaves(v, path=''):
    if isinstance(v, dict) and v:
        return sum((leaves(x, path + '/' + esc(k)) for k, x in v.items()), [])
    if isinstance(v, list) and v:
        return sum((leaves(x, path + '/' + str(i)) for i, x in enumerate(v)), [])
    return [{'pointer': path, 'actual': v}]


def differences(actual, expected, path=''):
    if type(actual) is not type(expected):
        return [{'pointer': path, 'actual': actual, 'expected': expected, 'kind': 'type'}]
    if isinstance(actual, dict):
        out = []
        for key in sorted(actual.keys() | expected.keys()):
            ptr = path + '/' + esc(key)
            if key not in actual:
                out.append({'pointer': ptr, 'kind': 'missing-actual', 'expected': expected[key]})
            elif key not in expected:
                out.append({'pointer': ptr, 'kind': 'extra-actual', 'actual': actual[key]})
            else:
                out.extend(differences(actual[key], expected[key], ptr))
        return out
    if isinstance(actual, list):
        if len(actual) != len(expected):
            return [{'pointer': path, 'kind': 'array-length', 'actual': actual, 'expected': expected}]
        return sum((differences(a, b, path + '/' + str(i)) for i, (a, b) in enumerate(zip(actual, expected))), [])
    return [] if actual == expected else [{'pointer': path, 'kind': 'value', 'actual': actual, 'expected': expected}]


def frame(v):
    return {k: v[k] for k in ['domain', 'asset', 'balances', 'allowances']}


# Preserve every original payload, not a projection of the two returned objects.
oracle_inventory = snapshot_tree(O, 'oracle')
observation_inventory = snapshot_tree(N, 'node-observations')
cycle_inventory = snapshot_tree(C, 'cycle')
assert len(oracle_inventory) == 24 and len(observation_inventory) == 13
snapshots = []
for p, name in [(H, 'source/rust-source6-capped-remittance.mjs'),
                (F / 'FIXTURE-PINS.json', 'source/FIXTURE-PINS.json')]:
    dest = D / name
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes(p.read_bytes())
    snapshots.append({'original': str(p), 'snapshot': name, **pin(p, p.parent)})
for name in ['run_batch.py', 'BOUND-CANDIDATE.json', 'PINNED-SOURCES.json',
             'TOOL-PINS.json', 'EXPECTATION-PINS.json', 'ROOT-SOURCE-BINDING.json']:
    dest = D / 'preparation' / name
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes((P / name).read_bytes())
    snapshots.append({'original': str(P / name), 'snapshot': 'preparation/' + name,
                      **pin(P / name, P)})

freeze = []
for line in (O / 'FREEZE.sha256').read_text().splitlines():
    digest, name = line.split('  ', 1)
    actual = sha((O / name).read_bytes())
    freeze.append({'path': name, 'expectedSha256': digest, 'actualSha256': actual, 'matches': actual == digest})
assert len(freeze) == 23 and all(x['matches'] for x in freeze)
manifest = load(O / 'MANIFEST.json')
assert len(manifest['files']) == 22
for x in manifest['files']:
    assert pin(O / x['path'], O) == x
fixture_pins = load(F / 'FIXTURE-PINS.json')
assert len(fixture_pins['files']) == 24
for name, digest in fixture_pins['files'].items():
    assert sha((F / name).read_bytes()) == digest

# Public source bytes only; no tools/binaries/caches/configs are read or executed.
source = load(P / 'PINNED-SOURCES.json')
source_checks = []
for info in source['files']:
    p = W / info['path']
    b = p.read_bytes()
    digest = sha(b)
    blob = hashlib.sha1(b'blob ' + str(len(b)).encode() + b'\0' + b).hexdigest()
    source_checks.append({'path': info['path'], 'bytes': len(b), 'sha256': digest,
                          'gitBlobCalculatedWithoutGit': blob,
                          'matches': len(b) == info['bytes'] and digest == info['sha256'] and blob == info['gitBlob']})
assert len(source_checks) == 982 and all(x['matches'] for x in source_checks)
bound = load(P / 'BOUND-CANDIDATE.json')
assert sha((P / 'run_batch.py').read_bytes()) == bound['driverSha256']
for x in bound['controlFiles']:
    assert sha((P / x['path']).read_bytes()) == x['sha256']

result = load(C / 'RESULT.json')
phases = {}
for name in ['financial-source-check-tests', 'public-quantity-min', 'node-capped-public-consumer']:
    root = C / name
    pr = load(root / 'RESULT.json')
    stdout, stderr = (root / 'STDOUT.log').read_bytes(), (root / 'STDERR.log').read_bytes()
    assert sha(stdout) == pr['stdoutSha256'] and sha(stderr) == pr['stderrSha256']
    phases[name] = {'result': pr, 'start': load(root / 'START.json'),
                    'stdout': stdout.decode(), 'stderr': stderr.decode(), 'streamHashBindingsMatch': True}
for stream in ['stdout', 'stderr']:
    assert sha((C / (stream.upper() + '.log')).read_bytes()) == result['child'][stream + 'Sha256']
node_text = phases['node-capped-public-consumer']['stdout']
tap_counts = {key: int(value) for key, value in re.findall(r'^# (tests|pass|fail) (\d+)$', node_text, re.MULTILINE)}
spec_counts = {key: int(value) for key, value in re.findall(r'^ℹ (tests|pass|fail) (\d+)$', node_text, re.MULTILINE)}
assert tap_counts == {} and spec_counts == {'tests': 2, 'pass': 2, 'fail': 0}
assert result['child']['exitCode'] == 0 and result['allSelectedChecksSatisfied'] is True
assert result['failure'] is None
assert all(x['result']['exitCode'] == 0 for x in phases.values())
assert all(x['result']['validationComplete'] is True for x in phases.values())

# Cross-bind complete recorded root/phase/source/command metadata, not allPassed flags alone.
plan, start = load(C / 'CHECK-PLAN.json'), load(C / 'START.json')
assert load(C / 'CHILD-RESULT.json') == result['child']
assert start['contractSha256'] == sha((P / 'BOUND-CANDIDATE.json').read_bytes())
assert start['argv'] == ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '600s',
    '/usr/bin/python3', str(P / 'run_batch.py'), '--worker', start['contractSha256'], str(C / 'CHECK-PLAN.json')]
assert plan['steps'] == bound['steps']
assert plan['head'] == bound['head'] == result['head'] == source['head']
assert plan['tree'] == bound['tree'] == result['tree'] == source['tree']
assert plan['wholeBatchSeconds'] == 600 and plan['killGraceSeconds'] == 5
assert len(plan['environment']) == 13 and start['environment'] == plan['environment']
assert start['cwd'] == plan['ownedRoot'] + '/cwd'
assert not result['child']['parentDeadlineExceeded'] and result['child']['directChildReaped']
assert result['child']['attemptConsumed'] is True and result['child']['ownedGroupStop'] is None
assert result['child']['wallSeconds'] < 600
assert (C / 'STDERR.log').read_bytes() == b''
root_phase_rows = [json.loads(x, object_pairs_hook=unique_object) for x in (C / 'STDOUT.log').read_text().splitlines()]
assert root_phase_rows == [{'step': name, **phases[name]['result']} for name in phases]
for step in plan['steps']:
    name = step['name']
    ps, pr = phases[name]['start'], phases[name]['result']
    assert ps['argv'] == step['argv'] and ps['cwd'] == start['cwd']
    env = dict(plan['environment'])
    if step['kind'] == 'node':
        env.update(MORI_RUST_BIN=plan['ownedRoot'] + '/target/debug/mori', MORI_SOURCE6_OBSERVATIONS=str(N))
    assert ps['environment'] == env and ps['oneWholeBatchDeadline'] is True
    child = load(C / name / 'CHILD.json')
    assert child['session'] == load(C / 'CHILD.json')['session']
    assert child['inheritsWholeBatchAllocation'] is True
    if step['kind'] != 'node':
        rows = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', phases[name]['stdout'])
        assert len(rows) == 1
        assert dict(zip(['passed', 'failed', 'ignored', 'measured', 'filtered'], map(int, rows[0]))) == pr['rustSummary']
        expected = {'passed': 6 if step['kind'] == 'financial' else 1, 'failed': 0, 'ignored': 0, 'measured': 0,
                    'filtered': 8 if step['kind'] == 'financial' else 7}
        assert pr['rustSummary'] == expected
        names = bound['expectedFinancialNames'] if step['kind'] == 'financial' else ['quantity_min_selects_both_operands_through_public_source6_expansion']
        assert all('test ' + test + ' ... ok' in phases[name]['stdout'] for test in names)
source_receipts = [result['sourceBefore'], result['sourceAfter'], plan['sourceBefore']]
source_receipts += [v[key]['sourceBefore' if key == 'start' else 'sourceAfter'] for v in phases.values() for key in ['start', 'result']]
for sr in source_receipts:
    assert sr['head'] == source['head'] and sr['tree'] == source['tree']
    assert sr['sourcePinsVerified'] == 982 and sr['sourceTaskToolExpectationPinsVerified'] is True and sr['clean'] is True
    if 'emptyAncestorConfigsVerified' in sr:
        assert sr['emptyAncestorConfigsVerified'] == 18
worker = load(C / 'WORKER-COMPLETE.json')
assert worker['allThreePhasesValidated'] is True
assert (worker['expectedFinancialTests'], worker['expectedQuantityTests'], worker['expectedNodeCases']) == (6, 1, 2)
cleanup, process_cleanup = load(C / 'CLEANUP.json'), load(C / 'PROCESS-CLEANUP.json')
assert cleanup['ownedRoot'] == plan['ownedRoot'] and cleanup['existsAfter'] is False
assert cleanup['remainingBeforeImmediateOwnedKill'] == [] and cleanup['cleanupError'] is None
assert process_cleanup['remainingSessionMembers'] == []
assert process_cleanup['ownedSession'] == load(C / 'CHILD.json')['session']
assert result['temporaryRootExistsAfter'] is False

run = load(N / 'run.json')
before = load(C / 'SOURCE6-BINARY-IDENTITY.json')
node_before = load(C / 'node-capped-public-consumer/BINARY-IDENTITY.json')
quantity = phases['public-quantity-min']['result']
assert before['sha256'] == node_before['sha256'] == run['binarySha256'] == quantity['binarySha256']
assert before['path'] == node_before['path'] == run['binary']
node_result = phases['node-capped-public-consumer']['result']
assert node_result['binarySha256After'] == node_result['nodeRunBinarySha256'] == before['sha256']
assert node_result['postNodeBinaryMatches'] is True and node_result['nodeRunBinaryMatches'] is True
assert node_result['nodeSummary'] == spec_counts
financial_identity = load(C / 'FINANCIAL-BINARY-IDENTITY.json')
assert financial_identity['sha256'] == phases['financial-source-check-tests']['result']['binarySha256']
assert financial_identity['path'] == before['path']
assert financial_identity['recordedBeforeSource6Cargo'] is True and before['recordedBeforeNode'] is True
assert run['oracleFreezeSha256'] == sha((O / 'FREEZE.sha256').read_bytes())
assert run['oracleManifestSha256'] == sha((O / 'MANIFEST.json').read_bytes())
assert run['expectedSemanticsSha256'] == sha((O / 'EXPECTED-SEMANTICS.json').read_bytes())
copy_bindings = []
for a, b in [('capped-remittance.mori', 'inputs/capped-remittance.mori'),
             ('capped-remittance.scenario.json', 'inputs/capped-remittance.scenario.json'),
             ('oracle-MANIFEST.json', 'MANIFEST.json'), ('oracle-FREEZE.sha256', 'FREEZE.sha256')]:
    matched = (N / a).read_bytes() == (O / b).read_bytes()
    assert matched
    copy_bindings.append({'observation': a, 'oracle': b, 'byteIdentical': matched})

facts = load(O / 'EXPECTED-SEMANTICS.json')
assert load(N / 'capped-remittance.scenario.json') == facts['common']['inputScenario']
cases = []
for ex in facts['cases']:
    action = ex['action']
    ast = load(O / ex['source6AstFile'])
    actual = load(N / action / 'actual-consumer.json')
    proc = load(N / action / 'process.json')
    stdout, stderr = (N / action / 'stdout.source6').read_bytes(), (N / action / 'stderr.txt').read_bytes()
    assert sha(stdout) == proc['stdoutSha256'] and sha(stderr) == proc['stderrSha256']
    assert proc['status'] == 0 and proc['signal'] is None and proc['error'] is None and not stderr
    assert stdout.endswith(b'\n') and not stdout.endswith(b'\n\n')
    stdout_text = stdout.decode('utf-8', errors='strict')
    assert proc['executable'] == run['binary']
    assert proc['args'][6] == action
    assert proc['args'] == ['--quiet', '--color', 'never', 'expand', str(O / 'inputs/capped-remittance.mori'),
                            '--action', action, '--scenario', str(O / 'inputs/capped-remittance.scenario.json'), '--emit-source6-transfer']
    assert proc['cwd'] == str(F) + '/'
    comparisons = []

    def compare(label, ptr, observed, expected, basis, classification='HARNESS_ASSERTION_RECHECKED'):
        diffs = differences(observed, expected)
        comparisons.append({'label': label, 'actualPointer': ptr, 'basis': basis,
                            'classification': classification, 'equal': not diffs,
                            'actual': observed, 'expected': expected, 'differences': diffs})
        assert not diffs, (action, label, diffs)

    for key, val in [('decodeError', None), ('loweringError', None), ('preparationError', None),
                     ('loweringAttempted', True), ('preparationAttempted', True)]:
        compare(key, '/' + key, actual[key], val, 'Harness process/API path assertion, not independent qualification oracle')
    for path, val in [('/lowered/ast', actual['lowered']['ast']), ('/outcome/ast', actual['outcome']['ast'])]:
        compare('FULL Source6 AST', path, val, ast, ex['source6AstFile'] + ' entire JSON object; exact keys/array order')
    low, cand = actual['lowered'], actual['outcome']['candidate']
    post = ex['conditionalClosingFinancialProjection']
    compare('Source6 full ordered effects', '/lowered/ast/submitted/effects', low['ast']['submitted']['effects'],
            ex['source6OrderedEffects'], 'EXPECTED-SEMANTICS.json source6OrderedEffects')
    compare('Source6 effect count', '/lowered/ast/submitted/effects', len(low['ast']['submitted']['effects']), ex['effectCount'], 'EXPECTED-SEMANTICS.json effectCount')
    compare('complete provided opening financial frame', '/lowered/state', frame(low['state']), ex['openingFinancialFrame'], 'EXPECTED-SEMANTICS.json openingFinancialFrame; projection explicitly limited to four keys')
    for key in ['head', 'round']:
        compare('opening ' + key, '/lowered/state/' + key, low['state'][key], ast['authenticated'][key], ex['source6AstFile'] + '/authenticated/' + key)
    for key, expected_key in [('workRemaining', 'remaining'), ('workSpent', 'spent')]:
        compare('opening input ' + key, '/lowered/state/' + key, low['state'][key], ex['source6InputWork'][expected_key], 'Source6 input work transport only, not cost')
    compare('selected key absent initially', '/lowered/state/consumedReplay', post['selectedReplay']['encodedKey'] in low['state']['consumedReplay'], False, 'Membership-only premise; NOT complete global replay inventory')
    intent_expected = {
        'sourceProfile': ast['profile'], 'programId': ast['selected']['actionId'],
        'sourceHash': ast['selected']['sourceHash'], 'policyDigest': ast['selected']['policyDigest'],
        'keyRef': ast['intent']['keyRef'], 'domain': ast['domain'], 'asset': ast['settlement']['asset'],
        'signer': ast['intent']['signer'], 'nonce': ast['intent']['nonce'], 'preHead': ast['intent']['preHead'],
        'notBefore': ast['intent']['notBefore'], 'notAfter': ast['intent']['notAfter'],
        'grossCap': ex['grossCap'], 'feeCap': ex['feeCap'], 'netFloor': ex['netFloor'],
        'kind': ast['intent']['signedAction']['kind'], 'recipient': ast['intent']['signedAction']['to'],
        'feeRecipient': ast['intent']['signedAction']['feeTo'], 'amount': ex['value'], 'fee': ex['fee']}
    compare('20 selected intent fields', '/lowered/intent', {k: low['intent'][k] for k in intent_expected}, intent_expected,
            'Full Source6 AST + EXPECTED-SEMANTICS; version/core and optional signedDigest not asserted')
    compare('full lowered Core5 ordered effects', '/lowered/submittedEffects', low['submittedEffects'], ex['core5SemanticOrderedEffects'], 'EXPECTED-SEMANTICS core5SemanticOrderedEffects; entire ordered array')
    compare('lowered proposed head', '/lowered/proposedPostHead', low['proposedPostHead'], post['head'], 'EXPECTED-SEMANTICS conditional projection/head')
    compare('candidate prehead', '/outcome/candidate/preHead', cand['preHead'], ast['authenticated']['head'], ex['source6AstFile'] + '/authenticated/head')
    compare('full candidate Core5 ordered effects', '/outcome/candidate/effects', cand['effects'], ex['core5SemanticOrderedEffects'], 'EXPECTED-SEMANTICS core5SemanticOrderedEffects; entire ordered array')
    compare('candidate effect count', '/outcome/candidate/effects', len(cand['effects']), ex['effectCount'], 'EXPECTED-SEMANTICS effectCount')
    compare('complete provided closing financial projection', '/outcome/candidate/candidatePost', frame(cand['candidatePost']), frame(post),
            ex['closingProjectionQualification'])
    for key in ['head', 'round']:
        compare('closing ' + key, '/outcome/candidate/candidatePost/' + key, cand['candidatePost'][key], post[key], 'EXPECTED-SEMANTICS conditionalClosingFinancialProjection/' + key)
    selected = next((x for x in cand['candidatePost']['consumedReplay'] if x == post['selectedReplay']['encodedKey']), None)
    compare('selected closing replay membership', '/outcome/candidate/candidatePost/consumedReplay', selected, post['selectedReplay']['encodedKey'], 'Membership only; NOT whole global replay inventory/history claim')
    compare('selected closing replay tuple', '/outcome/candidate/candidatePost/consumedReplay', json.loads(selected), post['selectedReplay']['tuple'], 'Selected tuple only; no authenticity or consumption judgment')

    # Supplemental text only: preserve whitespace mismatch rather than invent wire equality.
    lines = load(O / ex['orderedTargetLinesFile'])['lines']
    stripped_lines = [x.strip() for x in stdout_text.splitlines()]
    text_diffs = [{'line': i + 1, 'actual': a, 'expected': b} for i, (a, b) in enumerate(zip(stripped_lines, lines)) if a != b]
    # One explicitly literal presentation difference in the current captured texts.
    assert text_diffs == [{'line': 10, 'actual': 'valid 2..20;', 'expected': 'valid 2 .. 20;'}]
    assert len(stripped_lines) == len(lines) == 44

    financial_keys = ['domain', 'asset', 'balances', 'allowances', 'head', 'round', 'workRemaining', 'workSpent']
    def coverage(ptr):
        if ptr in ['/decodeError', '/loweringError', '/preparationError', '/loweringAttempted', '/preparationAttempted']:
            return 'HARNESS_API_PATH_ASSERTION; no independent qualification claim'
        if any(ptr == pre or ptr.startswith(pre + '/') for pre in ['/lowered/ast', '/outcome/ast', '/lowered/submittedEffects', '/outcome/candidate/effects']):
            return 'FULL_INDEPENDENT_JSON_EQUALITY'
        if ptr == '/lowered/proposedPostHead' or ptr == '/outcome/candidate/preHead':
            return 'INDEPENDENT_NAMED_FIELD_EQUALITY'
        for k in financial_keys:
            p = '/lowered/state/' + k
            if ptr == p or ptr.startswith(p + '/'):
                return 'INDEPENDENT_PROVIDED_FINANCIAL_OR_INPUT_TRANSPORT_FIELD'
        if ptr.startswith('/lowered/intent/') and ptr.split('/')[3] in intent_expected:
            return 'INDEPENDENT_SELECTED_INTENT_FIELD; 20 fields only'
        for k in ['domain', 'asset', 'balances', 'allowances', 'head', 'round']:
            p = '/outcome/candidate/candidatePost/' + k
            if ptr == p or ptr.startswith(p + '/'):
                return 'INDEPENDENT_CONDITIONAL_FINANCIAL_PROJECTION_FIELD'
        if ptr.startswith('/lowered/state/consumedReplay') or ptr.startswith('/outcome/candidate/candidatePost/consumedReplay'):
            return 'SELECTED_KEY_MEMBERSHIP_ONLY; global inventory UNASSERTED'
        return 'UNASSERTED_OBSERVATION_ONLY'

    leaf_coverage = [{**x, 'coverage': coverage(x['pointer'])} for x in leaves(actual)]
    assert len(leaf_coverage) == 244
    assert sum(x['coverage'].startswith('UNASSERTED') for x in leaf_coverage) == 19
    case = {'action': action, 'comparisons': comparisons,
            'allListedComparisonEqualitiesHold': all(x['equal'] for x in comparisons),
            'process': proc, 'processStreamsHashBound': True,
            'singleFinalLf': True, 'strictUtf8Decodable': True,
            'fullAstEqualityCount': 2, 'effectArraysComparedInFull': 3,
            'declaredGrossArithmetic': {'value': ex['value'], 'fee': ex['fee'], 'gross': ex['gross'],
                                       'valuePlusFeeEqualsGross': int(ex['value']) + int(ex['fee']) == int(ex['gross'])},
            'wholeActualObjectLeafCoverage': leaf_coverage,
            'unassertedObservedFields': [x for x in leaf_coverage if x['coverage'].startswith('UNASSERTED')],
            'wireTextSupplement': {'byteOracleClaimed': False,
                'wholeRawBytesEqualIndependentlyIndentedText': stdout == (O / ex['independentSource6File']).read_bytes(),
                'rawBytes': len(stdout), 'independentTextBytes': (O / ex['independentSource6File']).stat().st_size,
                'linesBoth': 44, 'lineByLineAfterOuterWhitespaceOnlyDifferences': text_diffs,
                'basis': 'Literal text comparison only; no parser execution/token normalization. Harness full AST comparison remains the semantic observation.'},
            'closingProjectionQualification': ex['closingProjectionQualification']}
    cases.append(case)

# The old failed-attempt fidelity packet is read-only and must retain every byte.
OLD = R / 'combined-financial-quantity-actual-fidelity-20261018'
old_freeze_expected = '78e1350c057c4e08b91ac9659b3624cf8d4d8f89e3d498f8bd9162261d5987e9'
assert sha((OLD / 'FREEZE.sha256').read_bytes()) == old_freeze_expected
old_rows = []
for line in (OLD / 'FREEZE.sha256').read_text().splitlines():
    digest, name = line.split('  ', 1)
    assert sha((OLD / name).read_bytes()) == digest
    old_rows.append(name)
assert len(old_rows) == 92
for x in oracle_inventory:
    assert (O / x['path']).read_bytes() == (OLD / 'oracle' / x['path']).read_bytes()
assert H.read_bytes() == (OLD / 'source/rust-source6-capped-remittance.mjs').read_bytes()

gaps = [
    {'id': 'G1', 'scope': 'NEW_RECORDED_POST_NODE_BINARY_HASH', 'status': 'PRESENT_AND_HASH_STRINGS_EQUAL',
     'detail': 'New cycle records binarySha256After and postNodeBinaryMatchestrue before summary parsing. It equals the quantity/source6-before/Node-before/run-record digest. This task compares retained receipts, not the deleted binary itself. The previous failed cycle still has its irrecoverable oldposthash gap; no reconstruction or cross-attempt transfer.'},
    {'id': 'G2', 'scope': 'NEW_WRAPPER_RUN_RECORD_COMPARISON', 'status': 'RECORDED_COMPLETE',
     'detail': 'New cycle records nodeRunBinarySha256 and nodeRunBinaryMatchestrue; this metadata comparison agrees with actual retained run.json. Both new checks executed according to retained source/receipts; no original failed receipt is rewritten.'},
    {'id': 'G3', 'scope': 'FINITE_BATCH_ONLY', 'status': 'ORIGINAL_NEW_SELECTED_CHECKS_SATISFIED',
     'detail': 'All three phase childrenexit0 and validationtrue; Node2/2/0; outerexit0/allSelectedChecksSatisfiedtrue; WORKER-COMPLETE exists. This is the finite selected check result, not source acceptance, a review vote, fullCore/native/proof/ledger/parent acceptance.'},
    {'id': 'G4', 'scope': 'FULL_CORE_POST_GLOBAL_REPLAY_RESIDUAL_DUTIES', 'status': 'UNASSERTED',
     'detail': 'Full serialized pre/post objects retained, including empty obligations and observed replay arrays, but oracle/harness do not assert full Core object/core tags, full obligation/residual-duty DTO, global replay inventory/uniqueness/history or unlisted authenticated cells. Only selected-key absence/presence is tested.'},
    {'id': 'G5', 'scope': 'WORK_COST_COUNTERS', 'status': 'UNASSERTED_EXCEPT_INPUT_TRANSPORT',
     'detail': 'Input counters12/2 asserted. Actual candidatePost11/3 captured for each case, not independently expected; no logical/diagnostic/attempted/committed/CLI/parser/compiler/global counters, resource/cost cap or paid work acceptance claim.'},
    {'id': 'G6', 'scope': 'STATUS_QUALIFICATION_GATES_PREMISES_BINDINGS', 'status': 'UNASSERTED',
     'detail': 'Actual PreparedUnqualified status and candidate.status, core tags, requiredPremises four and unverifiedBindings four retained; no independent status/gate/authentication/admission/qualification or parent acceptance oracle. Candidate presence and financial fields only are required.'},
    {'id': 'G7', 'scope': 'WHOLE_PROGRAM_TARGET_ARTIFACT_IDENTITY', 'status': 'NO_ORACLE_NO_OBSERVATION',
     'detail': 'Source6Ast is not Core Program/Target/Artifact. No Program/Plan/circuit/ABI/native/SDK/VM/proof/ledger result. srcC/policyC are opaque source claims, not canonical identity hashes. Custody SHA256 proves exact retained file agreement only.'},
    {'id': 'G8', 'scope': 'WHOLE_WIRE_AND_COMPLETE_INTENT', 'status': 'PARTIALLY_ASSERTED',
     'detail': 'Actual stdout bytes retained; no byte-for-byte Source6 golden, whitespace/span/token/node oracle. Independent text differs in indentation and valid2..20 spacing. FinalLF/stderr/exit are harness process assertions.20 intent fields compared; version/core and optional signedDigest absent are not independent intent-wire checks. JSON capture omits undefined properties and is not all in-memory object properties.'},
    {'id': 'G9', 'scope': 'COMPOSITION_GENERALITY', 'status': 'NOT_ASSERTED',
     'detail': 'Two alternative expansions start from same opening scenario and nC. They are not sequential composition, combined-success replay, shared-state/atomic/asynchronous acceptance or full language coverage. All parent/native/security obligations remain open.'},
    {'id': 'G10', 'scope': 'ORACLE_INDEPENDENCE_LIMITATIONS', 'status': 'QUALIFIED',
     'detail': 'Freeze predates this execution and is source-derived; prior Source6 implementation/other fixture exposure and DTO excerpts disclosed in fullPROVENANCE. Basis74a909 is historical, not f689. No globallyblind/compiler-wire-independent/fullCore/work/status oracle is claimed.'}
]

report = {
    'format': 'ordinary-existing-actual-fidelity-metadata/1',
    'sourceAcceptance': False, 'reviewVote': False, 'productExecutionDuringTask': False,
    'authority': 'Pure JSON/hash/text comparison of retained original artifacts only; no original receipt rewritten.',
    'originalCycle': str(C), 'capturedHead': source['head'], 'capturedTree': source['tree'],
    'previousFailedAttempt': {'path': str(OLD), 'freezeSha256': old_freeze_expected,
        'all92PayloadHashesStillMatch': True, 'oldMissingPostNodeHashStillMissing': True,
        'noNewResultTransferredToOldAttempt': True},
    'batch': {'result': result, 'phases': phases, 'tapRegexCounts': tap_counts,
              'supplementalSpecTextCounts': spec_counts,
              'observerSource': 'preparation/run_batch.py node_summary and worker post-Node custody branch',
              'finiteSelectedBatchResultVerifiedFromReceipts': True, 'noBroaderAcceptanceClaim': True},
    'completeRecordedCrossBindingsVerified': {'rootAndPhaseStreams': True, 'rootStdoutEqualsThreeCompletePhaseResults': True,
        'exactCommandsAndEnvironment': True, 'sourceBeforeAfterEveryPhase': True,
        'allRecordedBeforeAfterBinaryHashes': True, 'workerComplete': worker,
        'cleanup': cleanup, 'ownedSessionCleanup': process_cleanup,
        'qualification': 'Receipt/byte/hash equality only; binary deleted after completed original checks, not reconstructed.'},
    'binary': {'financialBeforeOverwrite': load(C / 'FINANCIAL-BINARY-IDENTITY.json'),
               'source6BeforeNode': before, 'nodePhaseBefore': node_before, 'runCapturedBeforeCases': run,
               'retainedBeforeIdentityAgreement': True, 'postNodeBinaryIdentityRecorded': phases['node-capped-public-consumer']['result']['binarySha256After'],
               'recordedPostNodeHashMatches': True, 'postNodeBinaryRehashedDuringThisTask': False,
               'runRecordComparisonHereOnly': True, 'originalRunRecordComparisonExecuted': True},
    'oracleCustody': {'inventoryCount': len(oracle_inventory), 'freezeEntryCount': len(freeze),
                     'manifestPayloadCount': len(manifest['files']), 'freezeVerification': freeze,
                     'all24FixturePinsMatch': True, 'frozenCopiesMatch': copy_bindings,
                     'fullQualifications': load(O / 'UNKNOWN.json'), 'fullProvenance': load(O / 'PROVENANCE.json')},
    'sourcePins': {'countVerifiedNowByPureFileHashAndGitBlobCalculation': len(source_checks),
                   'all982Match': True,
                   'qualification': 'Matches bytes pinned to capturedf689; no current HEAD/clean tree queried, no source acceptance/semantic validation. Original sourceBefore/After receipts retained.',
                   'details': 'SOURCE-HASH-COMPARISON.json'},
    'cases': cases, 'allUnassertedGaps': gaps,
    'originalUnassertedList': run['unasserted'],
    'oracle24Inventory': oracle_inventory, 'observations13Inventory': observation_inventory,
    'cycleFullInventory': cycle_inventory, 'preparationAndSourceSnapshots': snapshots,
}
write('REPORT.json', report)
write('SOURCE-HASH-COMPARISON.json', source_checks)
write('INPUT-INVENTORY.json', {'oracle24': oracle_inventory, 'observations13': observation_inventory,
                              'cycle': cycle_inventory, 'sourceAndPreparation': snapshots})

lines = [
    'This is an ordinary artifact-fidelity report of the new finite passed run, not a source acceptance, review vote or broader acceptance.',
    '',
    'The captured full Source6 ASTs from both actual APIs agree exactly with each independent frozen case AST. '
    'All full ordered Source6/loweredCore5/candidate effect arrays and declared conditional financial projections agree. '
    'Both actions are alternatives from the same opening state. No sequential/shared-nonce execution is claimed.',
    '',
    f'Captured source: {source["head"]}, tree {source["tree"]}. All982 source file SHA256, lengths and independently calculated Git blob IDs match their existing pins. '
    'This task does not query a new Git HEAD, certify current cleanliness or reexecute any product. Complete harness, original driver/control pins, all24 oracle files, all13 observations and complete cycle receipts are copied without byte changes.',
    '',
    '| Phase | Exit | Original validation | Selected/failed/filtered | Wall seconds |',
    '| --- | --- | --- | --- | --- |',
]
for name, v in phases.items():
    rr = v['result']
    count = rr.get('rustSummary')
    desc = f'{count["passed"]}/{count["failed"]}/{count["filtered"]}' if count else 'Node tests2/pass2/fail0; observer accepts SPEC' 
    lines.append(f'| {name} | {rr["exitCode"]} | {rr["validationComplete"]} | {desc} | {rr["wallSeconds"]} |')
lines += ['', 'The new outer child exited0 after255.53403750299185s. Parent deadline was not exceeded. '
          'All three phase validationComplete fields are true, the complete worker receipt records allThreePhasesValidatedtrue, '
          'and final allSelectedChecksSatisfiedtrue. The new observer accepts one complete SPEC summary; raw stdout remains unchanged. '
          'This is the finite sixfinancial/oneRust/twoNode selection, not full workspace/compilerSDK/native/proof/ledger/parent acceptance. '
          'Its one600+5 allocation and all earlier attempt counters/failed receipts remain separate.', '',
          'The new observer persists postNode binary SHA and run-record comparison before parsing the summary. '
          'Quantity/source6-before/Node-before/run-record/postNode hashes all agree at SHA256`' + before['sha256'] + '`. '
          'postNodeBinaryMatches and nodeRunBinaryMatches are both true in the actual Node phase receipt. '
          'This metadata task verifies retained byte/hash bindings only; the removed binary is not rehashed/rebuilt. '
          'Owned session is empty and ownedRoot gone per original new cleanup receipts. '
          'The earlier failed report remains unchanged, including its unrecoverable old postNodehash gap; this new result does not fill that old gap.', '',
          '| Alternative | Full AST comparisons | Effects per vector | Owner/Recipient/Fee closing | Allowance remaining/spent |',
          '| --- | --- | --- | --- | --- |',
          '| pay_tight | lowerer + preparer wholeAST equal | 5 | 790/215/7 | 790/235 |',
          '| pay_loose | lowerer + preparer wholeAST equal | 5 | 775/230/7 | 775/250 |', '',
          'Each opening financial frame is domainMidnight/assetA, ordered balances1000/5/7, allowance1000/25. '
          'Both preserve fee row7 and round4, move headc0→c1, and match the selected replay tuple[Midnight,Owner,nC] only. '
          'Gross is value+zeroFee:210 or225. Full vectors retain Debit, Credit, UseAllowance, UseReplay, AdvanceHead in order; feeCredit is absent. '
          'Input work12/2 is asserted; observed post11/3 is not an independent cost oracle. '
          'REPORT.json includes every actual JSON leaf and its exact assertion/UNASSERTED classification without dropping empty arrays.', '',
          'All four raw AST objects, effect arrays and full returned objects are retained under node-observations/. '
          'Every comparison uses exact JSON types, complete object keysets and ordered arrays within the declared scope. '
          'Twenty selected intent fields are checked; the complete intent object also contains version/core fields which remain unasserted. '
          'Wire texts are921bytes versus independently indented1065bytes, and literal stripped line10 differs (`valid 2..20;` versus `valid 2 .. 20;`). '
          'These are not byte goldens; both full actual ASTs match. No product parser is run by this report.', '',
          'Every remaining gap follows. UNKNOWN.json and PROVENANCE.json are preserved in full, including prior implementation exposure and historical74a909 normative basis.', '']
for gap in gaps:
    lines += [f'{gap["id"]} {gap["scope"]}: {gap["status"]}. {gap["detail"]}', '']
lines += ['Exact unasserted serialized fields follow. Both complete actual objects have244 scalar/empty-container leaves each. '
          'These19 leaves percase are observed only, with the values shown below; their presence is not an oracle assertion. '
          'The two replay inventories are additionally only selected-key membership checks, not full-inventory assertions. '
          'No machine-leaf-map is needed to see these limitations; it remains available in REPORT.json.', '',
          '| Pointer in actual-consumer.json | pay_tight observed | pay_loose observed |', '| --- | --- | --- |']
u0, u1 = cases[0]['unassertedObservedFields'], cases[1]['unassertedObservedFields']
assert [x['pointer'] for x in u0] == [x['pointer'] for x in u1]
for a, b in zip(u0, u1):
    lines.append('| ' + a['pointer'] + ' | ' + json.dumps(a['actual']) + ' | ' + json.dumps(b['actual']) + ' |')
lines += ['', 'Selected-key-only inventories in both cases are opening`[]` and closing`["[\\"Midnight\\",\\"Owner\\",\\"nC\\"]"]`. '
          'The harness requires selected-key absence initially and the exact selected tuple afterward; it does not require that these arrays exhaust global replay state. '
          'Serialized undefined optional members are omitted, including signedDigest; no absent-field authenticity or in-memory completeness is inferred.', '',
          'Complete compared financial/effect scope: all three ordered balance cells and the one allowance record, domain and asset, '
          'plus named head/round and selected replay are checked. The whole Source6 AST additionally checks every explicit source field, '
          'settlement scale, agreement and selected IDs, intent terms, predecessor, signed/submitted actions and source input work. '
          'Full Core equality and residual obligations are not silently inferred from that AST.', '',
          '| Ordered effect | Source6 representation | Core5 representation |', '| --- | --- | --- |',
          '| 1 | Debit Owner/A amount210 or225 | same |',
          '| 2 | Credit Recipient/A amount210 or225 | same |',
          '| 3 | UseAllowance Owner amount210 or225 | same |',
          '| 4 | UseReplay bare nonce nC | UseReplay encoded tuple[Midnight,Owner,nC] |',
          '| 5 | AdvanceHead predecessorc0 successorc1 | same |', '',
          'There is no feeCredit when fee0; the explicit Fee balance7 remains present and unchanged. '
          'The comparison retains gross debit210/225 rather than replacing it with a net balance delta. '
          'Source constants75×3=225 and min(225,210)=210/min(225,250)=225 explain the two frozen values; '
          'the actual product outputs, not a recomputed substitute, are compared. '
          'No new source program/financial scenario or oracle values are authored by this report.', '']
for title, inv in [('Independent oracle24 files (23 freeze entries plus freeze)', oracle_inventory), ('Complete Node observations13 files', observation_inventory)]:
    lines += [title, '', '| Path | Bytes | SHA256 |', '| --- | --- | --- |']
    lines += [f'| {x["path"]} | {x["bytes"]} | {x["sha256"]} |' for x in inv]
    lines += ['']
lines += ['Complete actual phase stdout and stderr follow. Compiler output is retained, not interpreted as source/native acceptance.', '']
for name, v in phases.items():
    for stream in ['stdout', 'stderr']:
        lines += [name + '/' + stream, '', '```text', v[stream], '```', '']
lines += ['Original wrapper stderr', '', '```text', (C / 'STDERR.log').read_text(), '```', '',
          'Actionable boundary: root can provide this immutable capture plus actual source for a fresh nonauthor source/result review. '
          'The new observer-fixed metadata is complete for its finite selected checks; neither it nor this report retroactively fills the prior failed attempt missing postNodehash. '
          'No new oracle, product execution, provider call, security judgment or adoption occurs here.', '']
lines += ['Prior failed-attempt fidelity remains at`' + str(OLD) + '` with unchanged FREEZE.sha256`' + old_freeze_expected + '`. '
          'All92 prior payload hashes were checked without modifying the prior packet. '
          'Its old unexecuted postNodehash is irrecoverable from that capture and remains a failure gap.', '']
(D / 'REPORT.md').write_text('\n'.join(lines))

# Confirm all original snapshots remain byte-identical at the end.
for root, inv, dest in [(O, oracle_inventory, 'oracle'), (N, observation_inventory, 'node-observations'), (C, cycle_inventory, 'cycle')]:
    for x in inv:
        assert (root / x['path']).read_bytes() == (D / dest / x['path']).read_bytes()
inventory = [pin(p, D) for p in sorted(D.rglob('*')) if p.is_file() and p != D / 'FREEZE.sha256' and p != D / 'INVENTORY.json']
write('INVENTORY.json', inventory)
payloads = [p for p in sorted(D.rglob('*')) if p.is_file() and p != D / 'FREEZE.sha256']
(D / 'FREEZE.sha256').write_text(''.join(sha(p.read_bytes()) + '  ' + p.relative_to(D).as_posix() + '\n' for p in payloads))
print(json.dumps({'output': str(D), 'payloads': len(payloads), 'freezeSha256': sha((D / 'FREEZE.sha256').read_bytes()),
                  'reportJsonSha256': sha((D / 'REPORT.json').read_bytes()), 'oracleFiles': len(oracle_inventory),
                  'observationFiles': len(observation_inventory), 'sourcePins': len(source_checks),
                  'cases': [{'action': c['action'], 'listedComparisons': len(c['comparisons']),
                             'leafCoverage': len(c['wholeActualObjectLeafCoverage'])} for c in cases]}, indent=2))
