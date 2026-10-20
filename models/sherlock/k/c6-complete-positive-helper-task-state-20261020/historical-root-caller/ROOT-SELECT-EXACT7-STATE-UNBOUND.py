"""Root-only fixed exact7 HELPER/TASK/STATE runtime QA caller using existing F03.launch only.
No new runner or containment. Root alone dispatches. Author executed nothing.
PLAN_PIN, FREEZE_PIN, CONDITIONS_PIN, BINDING_PIN, and LABELQ_PIN are None until
Root mechanically binds real reviewed bytes. ACK is read from this caller directory.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import sys

M = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/producer-source01')
Q = M / 'runtime-qa-postcompile-01/revision03-frame-contrast-testonly'
A = Q / 'actual'
COMPILED = M / 'actual/c6-complete-helper-task-state-kompiled'
BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
PLAN_PIN = None
FREEZE_PIN = None
CONDITIONS_PIN = None
BINDING_PIN = None
LABELQ_PIN = None
FROZEN = [PLAN_PIN, FREEZE_PIN, CONDITIONS_PIN, BINDING_PIN, LABELQ_PIN]
PLAN = Path(PLAN_PIN['path']) if PLAN_PIN else None
FREEZE = Path(FREEZE_PIN['path']) if FREEZE_PIN else None
CONDITIONS = Path(CONDITIONS_PIN['path']) if CONDITIONS_PIN else None
BINDING = Path(BINDING_PIN['path']) if BINDING_PIN else None
ORDER = ['00-combined-new-units', '01-field-render', '02-kast-json', '03-depth1-reduce', '04-full-print', '05-strict-decode', '06-one-fixed-batch-whole-compare']
FSIZE = 536870912
TOKEN = 'SHERLOCK_K_ATTEMPT_TOKEN'
LABELS, DATA_LABELS = 265, 264
RENDER_PASS = 'PASS_LOSSLESS_C6_HELPER_TASK_STATE_QUERY_CARRIER'
CAPTURE_PASS = 'PASS_COMPLETE_CSOBSERVATIONS_CAPTURE'
CAPTURE_FAIL = 'FAIL_DECODED_HELPER_TASK_STATE_SOURCE_IDENTITY'
RESULT_CAPTURE_MISMATCH = 'C6_HELPER_TASK_STATE_CAPTURE_SOURCE_IDENTITY_MISMATCH'
COMPARE_PASS = 'PASS_WHOLE_TYPED_C6_HELPER_TASK_STATE_COMMONJSON_AND_TYPEDAST'
RESULT_PASS = 'PASS_SCOPED_C6_HELPER_TASK_STATE_POSITIVE_COMMONJSON_AND_TYPEDAST'
SCOPE06 = 'independent-complete-K-HELPER-TASK-STATE-commonJSON-and-typedAST-eight-root-fields'
PROFILE = 'sherlock-c6-complete-helper-task-state-positive/1'
KAST02_PREFIX = b'LblcsObserveAll'
CASE_IDS = {'helper_record_cases': ['helper-00', 'helper-01', 'helper-02', 'helper-03', 'helper-04', 'helper-05', 'helper-06', 'helper-07', 'helper-08', 'helper-09', 'helper-10', 'helper-11', 'helper-12', 'helper-13', 'helper-14', 'helper-15', 'helper-16', 'helper-17'], 'result_record_cases': ['result-00', 'result-01', 'result-02', 'result-03', 'result-04', 'result-05', 'result-06', 'result-07', 'result-08', 'result-09', 'result-10', 'result-11', 'result-12', 'result-13', 'result-14', 'result-15', 'result-16', 'result-17', 'result-18', 'result-19'], 'task_record_cases': ['task-00', 'task-01', 'task-02'], 'helper_section_cases': ['present-plan-result-nonpalindromic-task-order', 'present-build-result-contrasting-task-order', 'absent-helper-present-env-result-empty-tasks'], 'state_cases': ['nonempty-plan-state-ordered-mixed-task-stack', 'build-state-present-context-different-task-order']}
PUBLIC_INPUT = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/declaration-transport01/SAFE/SAFE-INPUTS.json', 'bytes': 31059, 'sha256': 'af3361b9d910cc3bd738cea4d3015fe4170efdc99d08315d27c953fd8ebc050a'}
EXPECTED_COMMON = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/B39-positive-helper-task-state-20261020/private/EXPECTED-COMPLETE-OBSERVATIONS.json', 'bytes': 54571, 'sha256': '32c823b636e97103ba71df9e88ec32701655454b17d61d61ebd25eb325e8f73e'}
EXPECTED_AST = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/B39-positive-helper-task-state-20261020/k-transport-01/private/EXPECTED-COMPLETE-CSObservations.json', 'bytes': 186296, 'sha256': '7de9cc220c230f978dd71a73964fec89313015a738ed24bd6a8683d9581ee684'}
LABEL_MAP_PIN = {'path': str(M / 'postcompile-label-qualification-01/COMPILED-KLABEL-MAP.json'), 'bytes': 368969, 'sha256': '7c4c239dbc8217839b301ca5494df6a24607a0b445ab0194d10f27919e4d18f4'}
PARSER_CLOSURE_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/producer-source01/designated-owner-existing-R2-complete-runtime-source-request-01/public-runtime-premises02/PUBLIC-B39-FIELD-CONSUMER-AND-PARSER-CLOSURE.json', 'bytes': 3424, 'sha256': 'bb74891f9ad3a3f4c0e5a41447a0e6230e61593fd34545d96c52488029049fdb'}
STDLIB_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-META-20261020/postcompile-runtime-dependency-closure-01/ACTUAL-PYTHON-STDLIB-OBSERVED-CLOSURE-PINS.json', 'bytes': 74042, 'sha256': '8b97d55958a56a89d34c0ba8f8f157e75c20cee1f0167031ca2bec846985bb72'}


def pin(path):
    path = Path(path); raw = path.read_bytes()
    return {'path': str(path), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}


def exact(row):
    if type(row) is not dict or set(row) != {'path', 'bytes', 'sha256'} or type(row['path']) is not str or type(row['bytes']) is not int or row['bytes'] < 0 or type(row['sha256']) is not str or len(row['sha256']) != 64 or any(c not in '0123456789abcdef' for c in row['sha256']):
        raise ValueError('exact typed three-key file pin required')
    return row


def checked(row):
    want = exact({k: row[k] for k in ('path', 'bytes', 'sha256')})
    if pin(row['path']) != want:
        raise ValueError('pin changed: ' + row['path'])
    if 'resolved_path' in row and str(Path(row['path']).resolve()) != row['resolved_path']:
        raise ValueError('resolved tool path changed')
    return want


def unique(pairs):
    out = {}
    for k, v in pairs:
        if k in out:
            raise ValueError('duplicate Root metadata key')
        out[k] = v
    return out


def loads(raw):
    return json.loads(raw, object_pairs_hook=unique, parse_constant=lambda x: (_ for _ in ()).throw(ValueError(x)))


def read(path):
    return loads(Path(path).read_bytes())


def save(path, value):
    with Path(path).open('xb') as stream:
        stream.write((json.dumps(value, sort_keys=True, indent=2, allow_nan=False) + '\n').encode())
        stream.flush(); os.fsync(stream.fileno())
    return pin(path)


def sync_file(path):
    with Path(path).open('rb') as stream:
        os.fsync(stream.fileno())


def sync_dir(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def tree_pins(root):
    entries = sorted(Path(root).rglob('*'))
    if any(p.is_symlink() for p in entries):
        raise ValueError('symlink in generated file inventory')
    return [pin(p) for p in entries if p.is_file()]


def sorted_rows(rows):
    return sorted(rows, key=lambda r: r['path'])


def canonical_pin(row, path):
    exact(row)
    if str(Path(path).resolve()) != path or row != pin(path):
        raise ValueError('producer pin differs from canonical on-disk: ' + path)
    return row


def is_int(value, want):
    return type(value) is int and value == want


def same(a, b):
    if type(a) is not type(b):
        return False
    if type(a) is dict:
        return a.keys() == b.keys() and all(same(a[k], b[k]) for k in a)
    if type(a) is list:
        return len(a) == len(b) and all(same(x, y) for x, y in zip(a, b))
    return a == b


def merge(rows):
    out = {}
    for row in rows:
        three = exact({k: row[k] for k in ('path', 'bytes', 'sha256')})
        if out.setdefault(three['path'], three) != three:
            raise ValueError('conflicting pins for one path: ' + three['path'])
    return list(out.values())


def carrier_status(phase, expect):
    query, report = phase['products']
    status = loads(Path(phase['stdout']).read_bytes())
    if type(status) is not dict or set(status) != {'status', 'product', 'report'} or status['status'] != RENDER_PASS:
        raise ValueError('01 stdout is not exactly one carrier PASS status JSON')
    pins = {'product': canonical_pin(status['product'], query), 'report': canonical_pin(status['report'], report)}
    invariant = read(report)
    if (invariant.get('status') != RENDER_PASS or invariant.get('product') != pins['product'] or invariant.get('source_input') != expect['source_input'] or invariant.get('compiled_definition') != expect['definition'] or invariant.get('compiled_label_map') != expect['label_map'] or not is_int(invariant.get('actual_label_count'), LABELS) or not is_int(invariant.get('data_label_count'), DATA_LABELS) or invariant.get('profile') != PROFILE or invariant.get('case_ids') != expect['case_ids'] or invariant.get('module') != 'SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE-LAWS' or invariant.get('sort') != 'CSObservations' or invariant.get('same_whole_typed_query') is not True or not is_int(invariant.get('expected_body_reads'), 0) or invariant.get('no_model_or_lane_evaluation_in_Python') is not True):
        raise ValueError('01 INVARIANT-REPORT differs')
    return {'status': RENDER_PASS, **pins}


def capture_status(phase, operand):
    common_path, internal_path, wire_path, report = phase['products']
    status = loads(Path(phase['stdout']).read_bytes())
    if type(status) is not dict or set(status) != {'status', 'products', 'report'} or status['status'] not in (CAPTURE_PASS, CAPTURE_FAIL):
        raise ValueError('05 stdout is not exactly one capture status JSON')
    if set(status['products']) != {'complete_actual_common', 'complete_actual_internal', 'complete_actual_oracle_wire'}:
        raise ValueError('05 stdout products keyset differs')
    pins = {'complete_actual_common': canonical_pin(status['products']['complete_actual_common'], common_path), 'complete_actual_internal': canonical_pin(status['products']['complete_actual_internal'], internal_path), 'complete_actual_oracle_wire': canonical_pin(status['products']['complete_actual_oracle_wire'], wire_path), 'report': canonical_pin(status['report'], report)}
    capture = read(report)
    if (capture.get('status') != status['status'] or capture.get('products') != {k: pins[k] for k in ('complete_actual_common', 'complete_actual_internal', 'complete_actual_oracle_wire')} or capture.get('product') != pins['complete_actual_common'] or capture.get('source_input') != operand['source_input'] or capture.get('printed') != operand['printed'] or capture.get('profile') != PROFILE or capture.get('case_ids') != CASE_IDS or capture.get('module') != 'SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE-LAWS' or capture.get('sort') != 'CSObservations' or not is_int(capture.get('observation_count'), 46) or capture.get('observation_counts') != {'helper_record_cases': 18, 'result_record_cases': 20, 'task_record_cases': 3, 'helper_section_cases': 3, 'state_cases': 2} or type(capture.get('decoded_source_whole_equal')) is not bool or capture.get('source_input_whole_identity') != capture.get('decoded_source_whole_equal') or capture.get('decoded_state_and_context_whole_equal') != capture.get('decoded_source_whole_equal') or type(capture.get('source_case_ids_whole_equal')) is not bool or not is_int(capture.get('expected_body_reads'), 0)):
        raise ValueError('05 CAPTURE-REPORT differs')
    if (status['status'] == CAPTURE_PASS) != (capture['decoded_source_whole_equal'] is True):
        raise ValueError('05 status contradicts decoded source whole identity')
    if capture['decoded_source_whole_equal'] is True and capture.get('source_case_ids_whole_equal') is not True:
        raise ValueError('05 status contradicts source case-ids whole identity')
    common = read(common_path)
    if (list(common) != ['schema', 'profile', 'case_ids', 'helper_record_observations', 'result_record_observations', 'task_record_observations', 'helper_section_observations', 'state_observations'] or len(common['helper_record_observations']) != 18 or len(common['result_record_observations']) != 20 or len(common['task_record_observations']) != 3 or len(common['helper_section_observations']) != 3 or len(common['state_observations']) != 2):
        raise ValueError('05 complete common structure differs')
    internal = read(internal_path)
    if internal.get('sort') != 'CSObservations' or set(internal) != {'sort', 'constructor', 'arguments'}:
        raise ValueError('05 complete internal typed structure differs')
    wire = read(wire_path)
    if wire.get('sort') != 'CSObservations' or set(wire) != {'sort', 'constructor', 'arguments'}:
        raise ValueError('05 complete oracle-wire structure differs')
    return {'status': status['status'], 'products': {k: pins[k] for k in ('complete_actual_common', 'complete_actual_internal', 'complete_actual_oracle_wire')}, 'report': pins['report'], 'decoded_source_whole_equal': capture['decoded_source_whole_equal']}


def unit00_stderr_ok(stderr):
    try:
        text = stderr.decode('utf-8')
    except UnicodeDecodeError:
        return False
    lines = text.splitlines()
    stripped = [line.strip() for line in lines if line.strip()]
    if not stripped or stripped[-1] != 'OK':
        return False
    ran = [line.strip() for line in lines if line.strip().startswith('Ran ')]
    if len(ran) != 1 or not (ran[0] == 'Ran 15 tests' or ran[0].startswith('Ran 15 tests in ')):
        return False
    if any(line.strip().startswith(('FAIL:', 'ERROR:', 'FAILED')) for line in lines):
        return False
    return True


def main():
    sys.dont_write_bytecode = True
    if any(row is None for row in FROZEN):
        raise ValueError('Root plan/freeze/conditions/binding/label pins not yet bound')
    for row in FROZEN:
        checked(row)
    ack_path = Path(__file__).resolve().parent / 'ROOT-ACTUAL-REVIEW-SELECTION.json'
    ack = read(ack_path)
    if ack.get('Root_fullread_and_selected') is not True or ack.get('phase_order') != ORDER:
        raise ValueError('Root full review/caller selection not recorded')
    opinion = checked(ack['actual_final_opinion'])
    if opinion['bytes'] == 0:
        raise ValueError('actual final source opinion empty')
    current = {'selected_caller': pin(Path(__file__).resolve()), 'plan': pin(PLAN), 'source_freeze': pin(FREEZE), 'root_conditions': pin(CONDITIONS), 'opaque_binding': pin(BINDING)}
    if any(ack.get(k) != v for k, v in current.items()):
        raise ValueError('Root ACK does not bind current caller/plan/freeze/conditions/opaque index')
    plan = read(PLAN); phases = plan['phases']
    if ([x['id'] for x in phases] != ORDER or plan['no_compile_or_oldpassingprefix_replay'] is not True or plan['callback'].get('oldmain_ledger_retry_unselected') is not True or plan['no_source_or_operation_selected'] is not True):
        raise ValueError('frozen order/operands differ')
    limits = {'wall_seconds': 30, 'sweep_seconds': 3, 'rlimit_core': 0, 'rlimit_fsize': FSIZE, 'umask': '0o077'}
    if any(plan['limits'].get(k) != v for k, v in limits.items()) or plan['limits'].get('no_aggregate_CPU_RSS_disk_cap') is not True:
        raise ValueError('plan limits differ')
    for phase in phases:
        cwd, env = phase['cwd'], phase['environment']
        if (Path(cwd).parent != A or Path(cwd).name != phase['id'] or any(phase['limits'].get(k) != v for k, v in limits.items()) or phase['inherit_environment'] is not False or phase['stdin'] != 'DEVNULL' or phase['stdout'] != cwd + '/stdout' or phase['stderr'] != cwd + '/stderr' or env.get('HOME') != cwd + '/home' or env.get('TMPDIR') != cwd + '/tmp' or type(env.get(TOKEN)) is not str or not env[TOKEN] or type(phase['products']) is not list):
            raise ValueError('phase metadata differs: ' + phase['id'])
    if '-Xmx512m' not in phases[2]['argv'] or '-Xss32m' not in phases[2]['argv'] or '-XX:ActiveProcessorCount=2' not in phases[2]['argv']:
        raise ValueError('02 JVM heap/stack/processor qualification differs')
    if not is_int(phases[0].get('expected_unit_methods'), 15):
        raise ValueError('00 combined unit count differs')
    decode_path = Path(phases[5]['argv'][6]); manifest_path = Path(phases[6]['argv'][6]); barrier_path = A / 'ROOT-ACTUAL-FROZEN-BEFORE-EXPECTED.json'
    render_manifest = Path(phases[1]['argv'][6])
    if (render_manifest.parent != Q or decode_path.parent != A or manifest_path.parent != A or phases[5]['argv'][5] != 'decode' or phases[6]['argv'][5] != 'compare'):
        raise ValueError('dynamic render/decode/manifest operands differ')
    freeze = read(FREEZE); public = exact(freeze['public_input'])
    if public != PUBLIC_INPUT:
        raise ValueError('public original-input binding differs')
    registry = exact(freeze['registry'])
    binding = read(BINDING)
    exp_common = exact(binding['original_commonJSON']); exp_ast = exact(binding['independent_AST_transport'])
    if exp_common != EXPECTED_COMMON or exp_ast != EXPECTED_AST:
        raise ValueError('opaque expected binding differs')
    opaque_rows = [exp_common, exp_ast, exact(binding['original_freeze_metadata_only']), exact(binding['independent_AST_SAFE_handoff']), exact(binding['FORMAT_freeze_metadata_only'])]
    for opaque in opaque_rows:
        path = Path(opaque['path'])
        if str(path.resolve()) != str(path) or any(p.is_symlink() for p in [path, *path.parents]):
            raise ValueError('opaque operand not canonical symlink-free')
    compiled_pin = exact(plan['compiled15_reuse']); compiled = read(checked(compiled_pin)['path'])
    if type(compiled) is not list or len(compiled) != 15:
        raise ValueError('compiled15 binding differs')
    definition = [r for r in compiled if r['path'] == str(COMPILED / 'definition.kore')]
    label_map = [r for r in freeze['members'] if r['path'] == LABEL_MAP_PIN['path']]
    if len(label_map) != 1 or label_map[0] != LABEL_MAP_PIN or len(definition) != 1:
        raise ValueError('compiled15/definition/label-map binding differs')
    labelq_pin = checked(LABELQ_PIN)
    if labelq_pin != label_map[0]:
        raise ValueError('actual label qualification is not the actual265 map itself')
    labelq = read(LABELQ_PIN['path'])
    if labelq.get('constructor_count') != 265 or type(labelq.get('constructors')) is not list or len(labelq['constructors']) != 265:
        raise ValueError('actual label qualification map differs')
    source_doc = read(public['path'])
    if source_doc.get('profile') != PROFILE or {k: [c['case_id'] for c in source_doc[k]] for k in ('helper_record_cases', 'result_record_cases', 'task_record_cases', 'helper_section_cases', 'state_cases')} != CASE_IDS:
        raise ValueError('current source profile/case order differs')
    expect01 = {'source_input': public, 'definition': definition[0], 'label_map': label_map[0], 'case_ids': CASE_IDS}
    tools_pin = exact(plan['tools']); tools = read(checked(tools_pin)['path'])
    closure_pin = exact(plan['K_builtin_closure']); closure = read(checked(closure_pin)['path'])
    stdlib_pin = exact(plan['stdlib'])
    if stdlib_pin != STDLIB_PIN:
        raise ValueError('stdlib pin differs')
    stdlib_doc = read(checked(stdlib_pin)['path']); stdlib_members = stdlib_doc['members']
    if len(stdlib_members) != 175:
        raise ValueError('observed stdlib member count differs')
    parser_doc = read(checked(PARSER_CLOSURE_PIN)['path']); parser_members = parser_doc['user_defined_members']
    if len(parser_members) != 5:
        raise ValueError('parser closure member count differs')
    callback = [exact(plan['callback'][k]) for k in ('fullsource', 'full_callable_closure', 'full_import_projection')]
    raw = [*FROZEN, *freeze['members'], public, registry, *compiled, *tools['entries'], *tools['classpath_jar_membership'], *closure['files'], *callback, tools_pin, closure_pin, stdlib_pin, *stdlib_members, *parser_members, PARSER_CLOSURE_PIN, labelq_pin, pin(Path(__file__).resolve()), pin(ack_path), opinion]
    merge(raw); rows = list({r['path']: r for r in raw}.values())
    for row in rows:
        checked(row)
    hard = resource.getrlimit(resource.RLIMIT_FSIZE)[1]
    if hard != resource.RLIM_INFINITY and hard < FSIZE:
        raise ValueError('Root hard FSIZE below 512MiB')
    spec = importlib.util.spec_from_file_location('root_existing_f03_only', callback[0]['path'])
    f03 = importlib.util.module_from_spec(spec); spec.loader.exec_module(f03)
    if (f03.CHILD_SECONDS, f03.SWEEP_SECONDS, f03.FILE_SIZE_LIMIT, f03.TOKEN_NAME) != (30, 3, FSIZE, TOKEN):
        raise ValueError('existing F03 constants differ')
    past = f03.tree(str(M / 'actual')); produced = {}

    def all_pins():
        for directory, names in ((Path(FREEZE).parent / 'source', freeze['source_exact_directory']), (Path(FREEZE).parent / 'tests', freeze['test_exact_directory'])):
            entries = sorted(directory.iterdir())
            if any(p.is_symlink() or not p.is_file() for p in entries) or [str(p) for p in entries] != sorted(names):
                raise ValueError('source/test exact membership differs: ' + str(directory))
        if any(os.path.lexists(d / '__pycache__') for d in (Path(FREEZE).parent / 'source', Path(FREEZE).parent / 'tests', BASE / 'source', M / 'source')):
            raise ValueError('__pycache__ present')
        if sorted_rows(tree_pins(COMPILED)) != sorted_rows(compiled):
            raise ValueError('compiled exact15 differs')
        if f03.tree(str(M / 'actual')) != past:
            raise ValueError('past compile files changed')
        jars = Path(tools['classpath_pattern']).parent
        if sorted(str(p) for p in jars.glob('*.jar')) != sorted(r['path'] for r in tools['classpath_jar_membership']):
            raise ValueError('classpath JAR membership differs')
        return {'pins': [checked(r) for r in [*rows, *produced.values()]], 'opaque_hash_only': [checked(r) for r in opaque_rows]}

    all_pins()
    if os.path.lexists(A):
        raise ValueError('fresh QA actual already exists; no retry/reset')
    A.mkdir(mode=0o700); receipts = A / 'root-receipts'; receipts.mkdir(mode=0o700)
    for phase in phases:
        cwd = Path(phase['cwd']); cwd.mkdir(mode=0o700); (cwd / 'home').mkdir(mode=0o700); (cwd / 'tmp').mkdir(mode=0o700)
        (receipts / phase['id']).mkdir(mode=0o700)
    selection = save(A / 'ROOT-SELECTION.json', {'plan': pin(PLAN), 'source_freeze': pin(FREEZE), 'caller': pin(Path(__file__).resolve()), 'actual_review_selection': pin(ack_path), 'Root_ACK_bound_current': current, 'actual_review': opinion, 'phase_order': ORDER, 'compiled15': compiled, 'Root_hard_fsize': hard, 'opaque_hash_only': opaque_rows, 'render01_status_pins_gate_before02': True, 'kast02_acceptance': 'clean exit0, nonempty stdout, EMPTY stderr; csObserveAll prefix non-gating', 'no_compile_or_oldpassingprefix_replay': True, 'expected_body_reads': 0, 'Root_opaque_hashes_excluded': True, 'no_aggregate_CPU_RSS_disk_cap': True, 'stdlib_observed_only_not_total_TCB': True, 'hash_then_reread_not_atomic': True})
    produced[selection['path']] = selection
    local = [r for r in rows if not r['path'].startswith('/nix/store/')]
    results, selected, failure = [], [], None
    capture_class, compare_class, expected_reads = 'NOT_REACHED', 'NOT_REACHED', 0

    def freeze06(common, ast):
        if common != pin(phases[5]['products'][0]) or ast != pin(phases[5]['products'][2]):
            raise ValueError('05 products not canonical/current')
        past_files = [pin(r['path']) for r in f03.tree(str(M / 'actual')) if r['kind'] == 'file']
        members = merge([*local, *produced.values(), *tree_pins(A), *past_files])
        if common not in members or ast not in members:
            raise ValueError('05 products absent from barrier')
        for row in members:
            checked(row); sync_file(row['path'])
        for directory in sorted((p for p in A.rglob('*') if p.is_dir()), reverse=True):
            sync_dir(directory)
        for directory in sorted({str(Path(r['path']).parent) for r in members}):
            sync_dir(directory)
        sync_dir(A); sync_dir(Q); sync_dir(M)
        barrier = save(barrier_path, {'expected_body_reads': 0, 'expected_body_read_scope': SCOPE06, 'members': members, 'commonJSON_actual': common, 'typedAST_actual': ast, 'compiled15': compiled, 'directories': {'qa_actual': f03.tree(str(A)), 'past_compile': past}, 'Root_opaque_hashes_excluded': True, 'fsync': 'all member files and containing directories before 06 dispatch'})
        manifest = {'commonJSON': {'actual': common, 'expected': exp_common}, 'typedAST': {'actual': ast, 'expected': exp_ast}, 'durable_freeze': barrier}
        saved = save(manifest_path, manifest); sync_dir(A)
        produced[barrier['path']] = barrier; produced[saved['path']] = saved
        if read(manifest_path) != manifest:
            raise ValueError('comparison operand binding differs')

    try:
        for phase in phases:
            label, cwd, rd = phase['id'], Path(phase['cwd']), receipts / phase['id']
            if any(r['accepted'] is not True for r in results):
                raise ValueError('prior phase not accepted')
            if sorted(p.name for p in cwd.iterdir()) != ['home', 'tmp'] or any(list((cwd / n).iterdir()) for n in ('home', 'tmp')):
                raise ValueError('phase cwd/home/tmp not fresh')
            if label == '02-kast-json' and results[-1].get('producer_status_and_pins', {}).get('status') != RENDER_PASS:
                raise ValueError('01 producer not accepted; STOP before02')
            if label == '05-strict-decode':
                printed = pin(phases[4]['stdout'])
                if printed not in results[-1]['streams'] or printed['bytes'] == 0:
                    raise ValueError('04 printed stream changed or empty')
                operand = {'source_input': public, 'printed': printed}
                dm = save(decode_path, operand); sync_dir(A); produced[dm['path']] = dm
                if read(decode_path) != operand:
                    raise ValueError('decode operand manifest differs')
            if label == '06-one-fixed-batch-whole-compare':
                capture = results[-1].get('capture_status_and_pins', {})
                if capture.get('status') != CAPTURE_PASS:
                    raise ValueError('05 capture not accepted; STOP before06')
                freeze06(capture['products']['complete_actual_common'], capture['products']['complete_actual_oracle_wire'])
            pre = save(rd / 'ROOT-PRECHECK.json', {'phase': label, **all_pins()})
            command = save(rd / 'COMMAND.json', {'id': label, 'argv': phase['argv'], 'environment': phase['environment'], 'cwd': phase['cwd'], 'stdin': 'DEVNULL', 'limits': phase['limits']})
            retained = read(command['path'])
            if any(retained[k] != phase[k] for k in ('argv', 'environment', 'cwd')):
                raise ValueError('COMMAND disagrees with frozen phase')
            readset = save(rd / 'ROOT-READSET.json', {'argv_file_operands': [pin(p) for p in phase['argv'] if os.path.isfile(p)], 'declared_pins': 'ROOT-PRECHECK static and produced pins', 'basis': 'conservative declared closure, not access instrumentation', 'expected_parse_selected': label.startswith('06-'), 'Root_opaque_hashes_excluded': True})
            record = {'id': label}; selected.append(label)
            if label.startswith('06-'):
                expected_reads = None
            f03.launch(record, phase['argv'], dict(phase['environment']), phase['cwd'], phase['environment'][TOKEN], FSIZE)
            launch = save(rd / 'LAUNCH-RESULT.json', record)
            results.append({'id': label, 'launch': launch, 'accepted': False, 'wall_seconds': record.get('elapsed_seconds')})
            inventory = save(rd / 'STAGE-INVENTORY.json', {'tree': f03.tree(str(cwd))})
            allowed = {'home', 'tmp', 'stdout', 'stderr', *[Path(p).name for p in phase['products']]}
            unexpected = [p.name for p in cwd.iterdir() if p.name not in allowed]
            streams = [pin(phase['stdout']), pin(phase['stderr'])]
            products = [pin(path) for path in phase['products'] if Path(path).is_file()]
            sp = save(rd / 'STREAM-PINS.json', streams); pp = save(rd / 'PRODUCT-PINS.json', products)
            results[-1]['streams'] = streams
            for row in [pre, command, readset, launch, inventory, sp, pp, *streams, *products, *tree_pins(cwd)]:
                produced[row['path']] = row
            post = save(rd / 'ROOT-POSTCHECK.json', {'phase': label, **all_pins(), 'unexpected': unexpected})
            produced[post['path']] = post
            clean = (record.get('outcome') == 'exited' and record.get('timed_out') is False and record.get('containment_breach') is False and record.get('remaining') == [] and not unexpected and len(products) == len(phase['products']) and all(r['bytes'] > 0 for r in products))
            stdout, stderr = Path(phase['stdout']).read_bytes(), Path(phase['stderr']).read_bytes()
            ok = clean and record.get('returncode') == 0
            if label == '00-combined-new-units':
                unit00_ok = unit00_stderr_ok(stderr) if ok else False
                u00 = save(rd / 'UNIT00-ACCEPTANCE.json', {'ran_15_tests': unit00_ok, 'final_ok': unit00_ok, 'no_failure_or_error_result': unit00_ok, 'complete_stderr_accepted': unit00_ok, 'streams': streams})
                produced[u00['path']] = u00; results[-1]['unit00_acceptance'] = u00; results[-1]['unit00_stderr_ran15_final_ok'] = unit00_ok; ok = ok and unit00_ok
            if label in ('02-kast-json', '04-full-print'):
                ok = ok and bool(stdout)
            if label == '01-field-render':
                try:
                    check = carrier_status(phase, expect01) if ok else {'not_evaluated': '01 not clean'}
                except Exception as error:
                    check = {'failure': type(error).__name__ + ': ' + str(error)}
                ps = save(rd / 'PRODUCER-STATUS-PINS.json', check); produced[ps['path']] = ps
                results[-1]['producer_status_and_pins'] = check; ok = ok and check.get('status') == RENDER_PASS
            if label == '02-kast-json':
                kast = save(rd / 'KAST02-ACCEPTANCE.json', {'stdout_nonempty': bool(stdout), 'stderr_empty': not stderr, 'csObserveAll_prefix_observed_nongating': stdout.startswith(KAST02_PREFIX), 'streams': streams})
                produced[kast['path']] = kast; results[-1]['kast02_acceptance'] = kast; ok = ok and not stderr
            if label == '05-strict-decode':
                try:
                    check = capture_status(phase, read(decode_path)) if ok else {'not_evaluated': '05 not clean'}
                except Exception as error:
                    check = {'failure': type(error).__name__ + ': ' + str(error)}
                cs = save(rd / 'CAPTURE-STATUS-PINS.json', check); produced[cs['path']] = cs
                results[-1]['capture_status_and_pins'] = check
                if ok and check.get('status') == CAPTURE_FAIL and check.get('decoded_source_whole_equal') is False:
                    capture_class = 'CAPTURE_SOURCE_IDENTITY_MISMATCH'
                results[-1]['capture_classification'] = capture_class; ok = ok and check.get('status') == CAPTURE_PASS
            if label.startswith('06-'):
                try:
                    comparison = loads(stdout)
                except Exception:
                    comparison = None
                compare_class = 'INFRASTRUCTURE_EXCEPTION_OR_INVALID_COMPARISON_STREAM'
                cap = results[-2]['capture_status_and_pins']['products']
                if (clean and not stderr and type(comparison) is dict and comparison.get('commonJSON', {}).get('actual') == cap['complete_actual_common'] and comparison.get('typedAST', {}).get('actual') == cap['complete_actual_oracle_wire'] and comparison.get('commonJSON', {}).get('expected') == exp_common and comparison.get('typedAST', {}).get('expected') == exp_ast and is_int(comparison.get('expected_body_reads'), 2)):
                    if record.get('returncode') == 0 and comparison.get('status') == COMPARE_PASS and comparison.get('whole_typed_equal') is True:
                        compare_class = 'PASS_WHOLE_TYPED_BOTH'; expected_reads = 2
                    elif record.get('returncode') == 1 and comparison.get('status') == 'FAIL' and comparison.get('whole_typed_equal') is False:
                        compare_class = 'MISMATCH_NO_REPAIR_REDERIVE_OR_RETRY'; expected_reads = 2
                results[-1]['comparison_classification'] = compare_class; ok = compare_class == 'PASS_WHOLE_TYPED_BOTH'
            results[-1]['accepted'] = ok; sync_dir(rd)
            if not ok:
                raise ValueError('fail-stop child: ' + label)
    except Exception as error:
        failure = {'type': type(error).__name__, 'message': str(error)}
    status = RESULT_PASS if failure is None and len(results) == 7 and all(r['accepted'] for r in results) else (RESULT_CAPTURE_MISMATCH if capture_class == 'CAPTURE_SOURCE_IDENTITY_MISMATCH' else ('C6_HELPER_TASK_STATE_MISMATCH' if compare_class.startswith('MISMATCH') else 'INFRASTRUCTURE_STOP'))
    outcome = {'status': status, 'phases': results, 'failure': failure, 'selected_dispatches': selected, 'unselected': [x for x in ORDER if x not in selected], 'capture_classification': capture_class, 'comparison_classification': compare_class, 'expected_body_reads': expected_reads, 'expected_parse_dispatch_selected': any(x.startswith('06-') for x in selected), 'expected_parse_attempted_count_unknown': expected_reads is None and any(x.startswith('06-') for x in selected), 'Root_private_expected_semantic_reads': 0, 'Root_opaque_hashes_excluded': True, 'no_compile_or_oldpassingprefix_replay': True, 'scope': 'Positive HELPER/TASK/STATE commonJSON + typedAST only; UNADMITTED representation, no admission/native/refinement/EARS claim', 'final_qualifications': {'stdlib_observed_only_not_total_TCB': True, 'hash_then_reread_not_atomic': True, 'no_aggregate_CPU_RSS_disk_cap': True}, 'read_count_semantics': 'None = 06 dispatch attempted, count unknown, never zero; 2 only valid full comparison', 'selection': selection}
    save(A / 'ROOT-RESULT.json', outcome); sync_dir(A)
    print(json.dumps(outcome, sort_keys=True))
    if status != RESULT_PASS:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
