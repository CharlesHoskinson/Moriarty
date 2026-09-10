"""Independent inert AST controls. Never imports or runs the operational module."""
import ast
import copy
import hashlib
import json
import pathlib
import re
import types

HERE = pathlib.Path(__file__).resolve().parent
PACKET = HERE.parent
ROOT = PACKET.parents[2]
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
tree = ast.parse((PACKET / 'execute-once.py').read_text())
rows = []

def check(label, fn, rejects=False):
    try:
        fn()
    except (AssertionError, ValueError, NameError, KeyError):
        assert rejects, label
        rows.append({'case': label, 'outcome': 'REJECTED_AS_REQUIRED'})
    else:
        assert not rejects, label
        rows.append({'case': label, 'outcome': 'PASS'})

candidate = json.loads((PACKET / 'source-candidate-02.json').read_text())
proposal = json.loads((PACKET / 'resource-proposal-03.json').read_text())
original = json.loads((PACKET / 'source-candidate-01.json').read_text())
preservation = json.loads((PACKET / 'executor-preservation-01.json').read_text())
assert len(candidate['files']) == 134 and len(original['files']) == 119
for path, digest in candidate['files'].items():
    assert sha(ROOT / path) == digest, path
for path, digest in original['files'].items():
    assert sha(ROOT / preservation.get(path, {}).get('archivePath', path)) == digest, path
for field in ['packetFiles', 'runtimeClosure']:
    for path, digest in proposal[field].items():
        assert sha(ROOT / path) == digest, path
pins = json.loads((ROOT / 'experiments/moriarty-midnight-financial/ledger/launch-runtime-pins.json').read_text())
sdk_root = pathlib.Path('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules')
sdk_count = 0
for package, pin in pins.items():
    for path, digest in pin['files'].items():
        assert sha(sdk_root / package / path) == digest
        sdk_count += 1

def record_expr(t):
    call = next(n for n in ast.walk(t) if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id == 'save' and isinstance(n.args[0], ast.BinOp) and isinstance(n.args[0].right, ast.Constant) and n.args[0].right.value == 'attempt.json')
    return compile(ast.Expression(call.args[1]), '<inert-attempt-dict>', 'eval')

class InertPath:
    def __init__(self, text, exists=False): self.text, self.exists = text, exists
    def __truediv__(self, text): return InertPath(self.text + '/' + text, self.exists)
    def is_file(self): return self.exists

for kind in ['loan', 'swap']:
    for previous in [False, True]:
        env = {'A': {'allocationId': kind}, 'P': {'priorCharges': proposal['priorCharges']}, 'admission': InertPath('admission'), 'PACKET': InertPath('packet', previous), 'other': 'other', 'timerArgv': ['inert'], 'sha': lambda p: hashlib.sha256(p.text.encode()).hexdigest()}
        def verify_record():
            out = eval(record_expr(tree), {'__builtins__': {}}, env)
            assert out['admissionSha256'] == env['sha'](env['admission'])
            assert out['otherCaseAttemptSha256'] == (env['sha'](InertPath('packet/other/attempt.json')) if previous else None)
            assert out['status'] == 'CONSUMED' and out['retryAllowed'] is False
            assert out['priorCharges'] == proposal['priorCharges']
        check(f'corrected-attempt-{kind}-prior-{previous}', verify_record)
        old_tree = ast.parse((PACKET / 'original-executor-01/execute-once.py').read_text())
        check(f'original-unbound-name-{kind}-prior-{previous}', lambda: eval(record_expr(old_tree), {'__builtins__': {}}, env), rejects=True)

# Run the exact review-binding loop with public in-memory receipt fixtures.
review_loop = next(n for n in tree.body if isinstance(n, ast.For) and getattr(n, 'lineno', 0) == 36)
review_code = compile(ast.Module(body=[review_loop], type_ignores=[]), '<inert-review-bindings>', 'exec')
class Receipt:
    def __init__(self, data): self.data = data
    def read_text(self): return json.dumps(self.data)
def review_fixture(mutation=None):
    a = {'sourceCandidate': {'sha256': 'source'}, 'proposalSha256': 'proposal', 'sourceReviews': [], 'resourceVotes': []}
    receipts = {}
    for group, field, digest, verdict in [('sourceReviews', 'candidateSha256', 'source', 'PASS_SCOPED'), ('resourceVotes', 'proposalSha256', 'proposal', 'APPROVE_BOUNDED_RUN')]:
        for reviewer in ['gpt-6-astra', 'grok-4.6']:
            key = group + reviewer
            a[group].append({'reviewer': reviewer, 'path': key})
            receipts[key] = {'reviewer': reviewer, 'verdict': verdict, field: digest, 'actualReview': {'path': 'raw'}}
    if mutation: mutation(a, receipts)
    def bound(ref):
        if ref['path'] == 'raw': return Receipt({})
        return Receipt(receipts[ref['path']])
    exec(review_code, {'A': a, 'bound': bound, 'json': json})
check('two-independent-exact-source-and-resource-votes', review_fixture)
for label, mutate in [
    ('missing-resource-review', lambda a, r: a['resourceVotes'].pop()),
    ('duplicate-reviewer', lambda a, r: a['sourceReviews'][1].update(reviewer='gpt-6-astra')),
    ('stale-source-digest', lambda a, r: r['sourceReviewsgrok-4.6'].update(candidateSha256='stale')),
    ('stale-resource-digest', lambda a, r: r['resourceVotesgrok-4.6'].update(proposalSha256='stale')),
    ('resource-dissent', lambda a, r: r['resourceVotesgrok-4.6'].update(verdict='REQUEST_CHANGES')),
    ('claimed-wrong-provider', lambda a, r: r['sourceReviewsgrok-4.6'].update(reviewer='other')),
    ('missing-actual-review', lambda a, r: r['sourceReviewsgrok-4.6'].pop('actualReview')),
]: check(label, lambda mutate=mutate: review_fixture(mutate), rejects=True)

functions = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name in ['activate', 'setupcheck']]
activation_code = compile(ast.Module(body=functions, type_ignores=[]), '<inert-activation>', 'exec')
for diagnostic in [False, True]:
    for label, start, delay, should_reject in [('timely', 5, 0, False), ('near-expiry', 117.9, 0, False), ('fsync-consumes-reserve', 117, 2, True), ('expired', 120, 0, True)]:
        def activation():
            clock = [start]; saved = []; calls = []
            def save(path, value): saved.append(copy.deepcopy(value)); clock[0] += delay
            def run(argv, timeout, input=None): calls.append(argv); return 'inert'
            env = {'time': types.SimpleNamespace(monotonic=lambda: clock[0]), 'setupDeadline': 120, 'O': pathlib.Path('/inert'), 'save': save, 'run': run}
            exec(activation_code, env)
            try: env['activate'](['systemd-run', '--user', '--unit=inert', '--property=Type=exec'], 'record', isDiagnostic=diagnostic)
            except AssertionError:
                assert not calls
                raise
            assert calls == [saved[0]['argv']]
            assert any(x.startswith('--property=TimeoutStartSec=') for x in calls[0])
            assert any(x.startswith('--property=RuntimeMaxSec=') for x in calls[0]) == diagnostic
        check(f'activation-{label}-diagnostic-{diagnostic}', activation, rejects=should_reject)

predicate_tree = ast.parse((ROOT / proposal['retentionPredicate']['path']).read_text())
predicate = next(n for n in predicate_tree.body if isinstance(n, ast.FunctionDef) and n.name == 'require_exit_zero')
env = {'re': re}
exec(compile(ast.Module(body=[predicate], type_ignores=[]), '<inert-terminal-predicate>', 'exec'), env)
invocation = '1' * 32
fields = {'LoadState': 'loaded', 'ActiveState': 'active', 'SubState': 'exited', 'Type': 'exec', 'RemainAfterExit': 'yes', 'Transient': 'yes', 'MainPID': '0', 'Result': 'success', 'ExecMainCode': '1', 'ExecMainStatus': '0', 'InvocationID': invocation}
check('loaded-zero-retained-before-stop', lambda: env['require_exit_zero'](fields, invocation))
for key, value in [('LoadState', 'not-found'), ('InvocationID', '2' * 32), ('ExecMainStatus', '1'), ('ExecMainCode', '0'), ('Result', 'timeout'), ('MainPID', '42')]:
    check('terminal-reject-' + key, lambda key=key, value=value: env['require_exit_zero']({**fields, key: value}, invocation), rejects=True)
check('terminal-reject-zero-startup-invocation', lambda: env['require_exit_zero'](fields, '0' * 32), rejects=True)

result = {'status': 'PASS', 'sourceCandidateSha256': sha(PACKET / 'source-candidate-02.json'), 'proposalSha256': sha(PACKET / 'resource-proposal-03.json'), 'executorSha256': sha(PACKET / 'execute-once.py'), 'candidatePinsVerified': 134, 'originalPinsPreserved': 119, 'sdkFilesVerified': sdk_count, 'cases': rows, 'scope': 'Public-byte hashes and inert selected AST execution only. No operational module import/run, private wallet reads, service operations, proof/build, or network.'}
print(json.dumps(result, indent=2))
