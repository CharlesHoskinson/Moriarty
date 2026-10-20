"""Root-only fixed exact7 padded DATA runtime QA selection using existing F03.launch.

Ordinary adaptation of the retained helper caller. No containment, launch
policy, runner, scheduler, or subject rule is added. Root alone dispatches
providers and subjects. The author has not imported or executed this caller
or any subject child.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import sys

M = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020')
Q = M / 'runtime-qa-postcompile-01/revision02-test-registry-lifetime-correction-01'
A = Q / 'actual'
COMPILED = M / 'actual/c6-positive-data-kompiled'
BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
# ROOT_BIND_AFTER_MATERIALIZATION: exact typed three-key pins computed by Root
# from the materialized DATA plan, source freeze, conditions, opaque binding
# index, and render manifest. No hash is asserted in this source.
PLAN_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/runtime-qa-postcompile-01/revision02-test-registry-lifetime-correction-01/PROPOSED-EXACT7-DATA-RUNTIME-OPERATIONS.json', 'bytes': 32139, 'sha256': '67fee4ad565a561f59d47a31393448c8403b3e2755244bace20b4e4fa7bc2081'}
FREEZE_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/runtime-qa-postcompile-01/revision02-test-registry-lifetime-correction-01/SOURCE-FREEZE.json', 'bytes': 10205, 'sha256': 'ecac0ca6a493debbecdf95a2c389e17cd595b4121672060ae260dfe0f6f923ed'}
CONDITIONS_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/runtime-qa-postcompile-01/revision02-test-registry-lifetime-correction-01/ROOT-CONDITIONS.json', 'bytes': 5103, 'sha256': '34ddbc7838b7e1da58083e87455c6ed1975c9feef380c3e9c91d38518e452098'}
BINDING_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/runtime-qa-postcompile-01/revision02-test-registry-lifetime-correction-01/OPAQUE-EXPECTED-BINDING.json', 'bytes': 2147, 'sha256': '1b14fcb55980adbab8dc18d92b465291557ac04b34c9ca128b168cdbd98d2ed5'}
RENDER_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/runtime-qa-postcompile-01/revision02-test-registry-lifetime-correction-01/RENDER-MANIFEST.json', 'bytes': 618, 'sha256': '9c35c72cb26bbcf73089adde1d00ba9d5eb4440db507bd40f5a52bd601020feb'}
LABELQ_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/postcompile-label-qualification-01/ACTUAL-LABEL-QUALIFICATION.json', 'bytes': 21377, 'sha256': '3fc2cb251c67d8847f139176617b7bb8e976c08b431ed920357559126e026a4e'}
FROZEN = [PLAN_PIN, FREEZE_PIN, CONDITIONS_PIN, BINDING_PIN, RENDER_PIN, LABELQ_PIN]
PLAN = Path(PLAN_PIN['path']) if PLAN_PIN else None
FREEZE = Path(FREEZE_PIN['path']) if FREEZE_PIN else None
CONDITIONS = Path(CONDITIONS_PIN['path']) if CONDITIONS_PIN else None
BINDING = Path(BINDING_PIN['path']) if BINDING_PIN else None
ORDER = ['00-combined-new-units', '01-render', '02-kast-json', '03-reduce', '04-print', '05-decode', '06-compare']
FSIZE = 536870912
TOKEN = 'SHERLOCK_K_ATTEMPT_TOKEN'
LABELS = 27
DATA_LABELS = 26
CARRIER_PASS = 'PASS_LOSSLESS_C6_DATA_QUERY_CARRIER'
CAPTURE_PASS = 'PASS_COMPLETE_C6_DATA_OBSERVATION_CAPTURE'
CAPTURE_FAIL_DECODED = 'FAIL_DECODED_SOURCE_WHOLE_TABLE_MISMATCH'
COMPARE_PASS = 'PASS_WHOLE_TYPED_C6_DATA_OBSERVATION'
RESULT_PASS = 'PASS_SCOPED_C6_DATA_POSITIVE_PADDED_OBSERVATION'
SCOPE06 = 'independent-complete-C6-positive-padded-DATA-one-observation'
PROFILE = 'sherlock-c6-data-positive-padded/1'
CASE_ID = 'seven-variants-partial-unequal-capacities-01'
KAST02_PREFIX = b'LbldpObserve'
PUBLIC_INPUT = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch34-data-positive-padded-source-task-20261020/SAFE-INPUT-ONLY.proposed.json', 'bytes': 2512, 'sha256': 'c1a694df525a79d158c4a07bb0ac5ecaf12083890fc8d51d74fa82717023ae2e'}
EXPECTED_BYTES = 2353
PRIVATE_FREEZE_BYTES = 5756
LABEL_MAP_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-positive-padded-DATA-20261020/postcompile-label-qualification-01/COMPILED-KLABEL-MAP.json', 'bytes': 36504, 'sha256': '67b6475ae68e3fd17e425bc7b7b17804500d56bc4eee16f9f8a3a212a98e26c1'}


def pin(path):
    path = Path(path); raw = path.read_bytes()
    return {'path': str(path), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}


def exact(row):
    if (type(row) is not dict or set(row) != {'path', 'bytes', 'sha256'} or type(row['path']) is not str
            or type(row['bytes']) is not int or row['bytes'] < 0 or type(row['sha256']) is not str
            or len(row['sha256']) != 64 or any(c not in '0123456789abcdef' for c in row['sha256'])):
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
    return json.loads(raw, object_pairs_hook=unique,
        parse_constant=lambda x: (_ for _ in ()).throw(ValueError(x)))


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
    if type(status) is not dict or set(status) != {'status', 'product', 'report'} or status['status'] != CARRIER_PASS:
        raise ValueError('01 stdout is not exactly one carrier PASS status JSON')
    pins = {'product': canonical_pin(status['product'], query), 'report': canonical_pin(status['report'], report)}
    invariant = read(report)
    if (type(invariant) is not dict or invariant.get('status') != CARRIER_PASS or invariant.get('product') != pins['product']
            or invariant.get('source_input') != expect['source_input'] or invariant.get('compiled_definition') != expect['definition']
            or invariant.get('compiled_label_map') != expect['label_map'] or not is_int(invariant.get('actual_label_count'), LABELS)
            or not is_int(invariant.get('data_label_count'), DATA_LABELS)
            or invariant.get('profile') != PROFILE or invariant.get('case_id') != CASE_ID
            or invariant.get('module') != 'SHERLOCK-C6-POSITIVE-PADDED-DATA-LAWS'
            or invariant.get('sort') != 'DPObservation' or invariant.get('same_whole_typed_query') is not True
            or not is_int(invariant.get('expected_body_reads'), 0) or invariant.get('no_model_or_lane_evaluation_in_Python') is not True):
        raise ValueError('01 INVARIANT-REPORT status/product/input/definition/map/count differs')
    return {'status': CARRIER_PASS, **pins}


def capture_status(phase, operand):
    observation_path, typed_path, report = phase['products']
    status = loads(Path(phase['stdout']).read_bytes())
    if type(status) is not dict or set(status) != {'status', 'products', 'report'} or status['status'] not in (CAPTURE_PASS, CAPTURE_FAIL_DECODED):
        raise ValueError('05 stdout is not exactly one capture status JSON')
    if type(status['products']) is not dict or set(status['products']) != {'complete_actual_observation', 'complete_actual_DPObservation'}:
        raise ValueError('05 stdout products keyset differs')
    pins = {'complete_actual_observation': canonical_pin(status['products']['complete_actual_observation'], observation_path),
            'complete_actual_DPObservation': canonical_pin(status['products']['complete_actual_DPObservation'], typed_path),
            'report': canonical_pin(status['report'], report)}
    capture = read(report)
    if (type(capture) is not dict or capture.get('status') != status['status']
            or capture.get('products') != {'complete_actual_observation': pins['complete_actual_observation'], 'complete_actual_DPObservation': pins['complete_actual_DPObservation']}
            or capture.get('product') != pins['complete_actual_observation']
            or capture.get('source_input') != operand['source_input'] or capture.get('printed') != operand['printed']
            or capture.get('profile') != PROFILE or capture.get('case_id') != CASE_ID
            or capture.get('all_actual_geometry_counts_padded_lanes_and_decoded_tables_retained') is not True
            or capture.get('all_complete_typed_fields_retained') is not True
            or type(capture.get('decoded_source_whole_table_equal')) is not bool
            or not is_int(capture.get('expected_body_reads'), 0)):
        raise ValueError('05 CAPTURE-REPORT status/products/operands differs')
    if status['status'] == CAPTURE_PASS and capture.get('decoded_source_whole_table_equal') is not True:
        raise ValueError('05 PASS status contradicts decoded whole-table identity')
    if status['status'] == CAPTURE_FAIL_DECODED and capture.get('decoded_source_whole_table_equal') is not False:
        raise ValueError('05 mismatch status contradicts decoded whole-table identity')
    observation = read(observation_path)
    if (type(observation) is not dict or list(observation) != ['geometry', 'counts', 'padded_lanes', 'decoded_tables']
            or type(observation['geometry']) is not dict
            or list(observation['geometry']) != ['data_base', 'fields_base', 'env_base', 'values_base', 'total']
            or any(type(v) is not int for v in observation['geometry'].values())
            or type(observation['counts']) is not list or len(observation['counts']) != 4
            or any(type(v) is not int for v in observation['counts'])
            or type(observation['padded_lanes']) is not list
            or any(type(v) is not int for v in observation['padded_lanes'])
            or type(observation['decoded_tables']) is not dict
            or list(observation['decoded_tables']) != ['data', 'data_fields', 'env', 'value_slots']):
        raise ValueError('05 complete observation four-key/native structure differs')
    typed = read(typed_path)
    args = typed.get('arguments') if type(typed) is dict else None
    if (type(typed) is not dict or set(typed) != {'sort', 'constructor', 'arguments'}
            or typed['sort'] != 'DPObservation' or typed['constructor'] != 'dpObservation'
            or type(args) is not list or len(args) != 4
            or args[0].get('sort') != 'DPGeometry' or args[1].get('sort') != 'CTActive'
            or args[2].get('sort') != 'CTLanes' or args[3].get('sort') != 'DPTables'):
        raise ValueError('05 complete typed DPObservation structure differs')
    if status['status'] == CAPTURE_FAIL_DECODED:
        return {'status': CAPTURE_FAIL_DECODED, 'products': {k: pins[k] for k in ('complete_actual_observation', 'complete_actual_DPObservation')}, 'report': pins['report'], 'decoded_source_whole_table_equal': False}
    return {'status': CAPTURE_PASS, 'products': {k: pins[k] for k in ('complete_actual_observation', 'complete_actual_DPObservation')}, 'report': pins['report'], 'decoded_source_whole_table_equal': True}


def main():
    sys.dont_write_bytecode = True
    if any(row is None for row in FROZEN):
        raise ValueError('Root DATA plan/freeze/conditions/binding/render/label pins not yet bound')
    for row in FROZEN:
        checked(row)
    ack_path = Path(__file__).resolve().parent / 'ROOT-ACTUAL-REVIEW-SELECTION.json'
    ack = read(ack_path)
    if type(ack) is not dict or ack.get('Root_fullread_and_selected') is not True or ack.get('phase_order') != ORDER:
        raise ValueError('Root full review/caller selection not recorded')
    opinion = checked(ack['actual_final_opinion'])
    if opinion['bytes'] == 0:
        raise ValueError('actual final source opinion empty')
    current = {'selected_caller': pin(Path(__file__).resolve()), 'plan': pin(PLAN), 'source_freeze': pin(FREEZE),
               'root_conditions': pin(CONDITIONS), 'opaque_binding': pin(BINDING)}
    if any(ack.get(k) != v for k, v in current.items()):
        raise ValueError('Root ACK does not bind current caller/plan/freeze/conditions/opaque index')
    plan = read(PLAN); phases = plan['phases']; dyn = plan['dynamic_metadata']
    if ([x['id'] for x in phases] != ORDER or plan['no_compile_or_old_prefix_replay'] is not True
            or plan['callback'].get('oldmain_ledger_retry_unselected') is not True or plan['source_freeze'] != FREEZE_PIN
            or plan['conditions'] != CONDITIONS_PIN or plan['opaque_binding'] != BINDING_PIN
            or plan['actual_label_qualification'] != LABELQ_PIN):
        raise ValueError('frozen DATA order/operands differ')
    limits = {'wall_seconds': 30, 'sweep_seconds': 3, 'rlimit_core': 0, 'rlimit_fsize': FSIZE, 'umask': '0o077'}
    if any(plan['limits'].get(k) != v for k, v in limits.items()) or plan['limits'].get('no_aggregate_CPU_RSS_disk_cap') is not True:
        raise ValueError('plan limits differ')
    for phase in phases:
        cwd = phase['cwd']; env = phase['environment']
        if (Path(cwd).parent != A or Path(cwd).name != phase['id'] or any(phase['limits'].get(k) != v for k, v in limits.items())
                or phase['inherit_environment'] is not False or phase['stdin'] != 'DEVNULL'
                or phase['stdout'] != cwd + '/stdout' or phase['stderr'] != cwd + '/stderr'
                or env.get('HOME') != cwd + '/home' or env.get('TMPDIR') != cwd + '/tmp'
                or type(env.get(TOKEN)) is not str or not env[TOKEN] or type(phase['products']) is not list):
            raise ValueError('phase metadata differs from F03.launch layout: ' + phase['id'])
    if '-Xmx512m' not in phases[2]['argv']:
        raise ValueError('02 JVM heap qualification differs')
    decode_path = Path(dyn['before05']['path']); barrier_path = Path(dyn['before06']['barrier']); manifest_path = Path(dyn['before06']['manifest'])
    if (dyn['before05']['exactkeys'] != ['source_input', 'printed'] or dyn['before06']['exactkeys'] != ['actual', 'expected', 'durable_freeze']
            or {decode_path.parent, barrier_path.parent, manifest_path.parent} != {A}
            or phases[5]['argv'][-4] != str(decode_path) or phases[6]['argv'][-1] != str(manifest_path)):
        raise ValueError('dynamic decode/barrier/manifest operands differ')
    freeze = read(FREEZE); public = exact(freeze['public_input'])
    if public != PUBLIC_INPUT:
        raise ValueError('public original-input binding differs')
    binding = read(BINDING); expected = exact(binding['expected']); private = exact(binding['private_freeze'])
    if dyn['before05']['source_input'] != public or dyn['before06']['expected'] != expected or (expected['bytes'], private['bytes']) != (EXPECTED_BYTES, PRIVATE_FREEZE_BYTES):
        raise ValueError('opaque expected/private freeze or source input binding differs')
    for opaque in (expected, private):
        path = Path(opaque['path'])
        if str(path.resolve()) != str(path) or any(p.is_symlink() for p in [path, *path.parents]):
            raise ValueError('opaque operand not canonical symlink-free')
    compiled = plan['compiled15_reuse']
    definition = [r for r in compiled if r['path'] == str(COMPILED / 'definition.kore')]
    label_map = [r for r in freeze['members'] if r['path'] == LABEL_MAP_PIN['path']]
    if len(label_map) != 1 or label_map[0] != LABEL_MAP_PIN:
        raise ValueError('compiled label map binding differs')
    render = read(RENDER_PIN['path']); labelq = read(LABELQ_PIN['path'])
    if (len(compiled) != 15 or len(definition) != 1
            or render != {'source_input': public, 'compiled_definition': definition[0]}
            or labelq.get('map') != label_map[0]):
        raise ValueError('compiled15/definition/actual27 map binding differs')
    expect01 = {'source_input': public, 'definition': definition[0], 'label_map': label_map[0]}
    tools = read(checked(plan['tools'])['path']); closure = read(checked(plan['K_builtin_closure'])['path'])
    callback = [exact(plan['callback'][k]) for k in ('fullsource', 'full_callable_closure', 'full_import_projection')]
    raw = [*FROZEN, *freeze['members'], freeze['model_freeze'], *freeze['model_sources'], public, freeze['registry'],
           freeze['output_root_declaration'], *compiled, *tools['entries'],
           *tools['classpath_jar_membership'], *closure['files'], *callback, plan['tools'], plan['K_builtin_closure'],
           labelq['actual_compile_result'], labelq['compiled15'], labelq['primary']['source'], binding['SAFE_handoff'],
           pin(Path(__file__).resolve()), pin(ack_path), opinion]
    merge(raw); rows = list({r['path']: r for r in raw}.values())
    for row in rows:
        checked(row)
    source_doc = read(public['path'])
    if (source_doc.get('profile') != PROFILE or source_doc.get('caseId') != CASE_ID
            or type(source_doc.get('caps')) is not dict or len(source_doc['caps']) != 9
            or type(source_doc.get('tables')) is not dict
            or list(source_doc['tables']) != ['data', 'data_fields', 'env', 'value_slots']):
        raise ValueError('current DATA source profile/case/caps/tables differ')
    registry_rows = read(freeze['registry']['path'])['constructors']
    observation_rows = [r for r in registry_rows if r['name'] == 'dpObservation']
    if (len(observation_rows) != 1 or observation_rows[0]['sort'] != 'DPObservation'
            or observation_rows[0]['argument_sorts'] != ['DPGeometry', 'CTActive', 'CTLanes', 'DPTables']):
        raise ValueError('public dpObservation declaration differs')
    hard = resource.getrlimit(resource.RLIMIT_FSIZE)[1]
    if hard != resource.RLIM_INFINITY and hard < FSIZE:
        raise ValueError('Root hard FSIZE below512MiB')
    spec = importlib.util.spec_from_file_location('root_existing_f03_only', callback[0]['path'])
    f03 = importlib.util.module_from_spec(spec); spec.loader.exec_module(f03)
    if (f03.CHILD_SECONDS, f03.SWEEP_SECONDS, f03.FILE_SIZE_LIMIT, f03.TOKEN_NAME) != (30, 3, FSIZE, TOKEN):
        raise ValueError('existing F03 constants differ')
    past = f03.tree(str(M / 'actual'))
    produced = {}

    def all_pins():
        for directory, names in ((Path(FREEZE).parent / 'source', freeze['source_exact_directory']), (Path(FREEZE).parent / 'tests', freeze['test_exact_directory'])):
            entries = sorted(directory.iterdir())
            if any(p.is_symlink() or not p.is_file() for p in entries) or [str(p) for p in entries] != sorted(names):
                raise ValueError('source/test exact membership differs: ' + str(directory))
        if any(os.path.lexists(d / '__pycache__') for d in (Path(FREEZE).parent / 'source', Path(FREEZE).parent / 'tests', BASE / 'source', M / 'source')):
            raise ValueError('__pycache__ present in source directory')
        if sorted_rows(tree_pins(COMPILED)) != sorted_rows(compiled):
            raise ValueError('compiled exact15 bytes/membership differs')
        if f03.tree(str(M / 'actual')) != past:
            raise ValueError('past compile files/directories changed')
        jars = Path(tools['classpath_pattern']).parent
        if sorted(str(p) for p in jars.glob('*.jar')) != sorted(r['path'] for r in tools['classpath_jar_membership']):
            raise ValueError('classpath JAR membership differs')
        return {'pins': [checked(r) for r in [*rows, *produced.values()]], 'opaque_hash_only': [checked(expected), checked(private)]}

    all_pins()
    if os.path.lexists(A):
        raise ValueError('fresh QA actual already exists; no retry/reset')
    A.mkdir(mode=0o700); receipts = A / 'root-receipts'; receipts.mkdir(mode=0o700)
    for phase in phases:
        cwd = Path(phase['cwd']); cwd.mkdir(mode=0o700); (cwd / 'home').mkdir(mode=0o700); (cwd / 'tmp').mkdir(mode=0o700)
        (receipts / phase['id']).mkdir(mode=0o700)
    selection = save(A / 'ROOT-SELECTION.json', {
        'plan': pin(PLAN), 'source_freeze': pin(FREEZE), 'caller': pin(Path(__file__).resolve()), 'actual_review_selection': pin(ack_path),
        'Root_ACK_bound_current': current, 'actual_review': opinion, 'phase_order': ORDER, 'compiled15': compiled, 'Root_hard_fsize': hard,
        'opaque_hash_only': [expected, private], 'schema_compare_phase': 'REMOVED: one independent whole typed expected; 00 unit, actual KJSON frontend and whole model compare remain',
        'render01_status_pins_gate_before02': True, 'kast02_acceptance': 'clean exit0, nonempty stdout, EMPTY stderr (conservative STOP); dpObserve prefix non-gating',
        'compile_warning_75887B_68fec773': 'UNADJUDICATED', 'no_compile_or_old_prefix_replay': True, 'expected_body_reads': 0,
        'Root_opaque_hashes_excluded': True, 'no_aggregate_CPU_RSS_disk_cap': True, 'stdlib_unpinned': True, 'hash_then_reread_not_atomic': True})
    produced[selection['path']] = selection
    local = [r for r in rows if not r['path'].startswith('/nix/store/')]
    results = []; selected = []; failure = None; compare_class = 'NOT_REACHED'; expected_reads = 0

    def freeze06(observation, typed):
        if str(Path(observation['path']).resolve()) != observation['path'] or observation != pin(phases[5]['products'][0]):
            raise ValueError('05 observation product not canonical/current')
        if str(Path(typed['path']).resolve()) != typed['path'] or typed != pin(phases[5]['products'][1]):
            raise ValueError('05 DPObservation product not canonical/current')
        past_files = [pin(r['path']) for r in f03.tree(str(M / 'actual')) if r['kind'] == 'file']
        members = merge([*local, *produced.values(), *tree_pins(A), *past_files])
        if observation not in members or typed not in members:
            raise ValueError('05 products absent from complete barrier')
        for row in members:
            checked(row); sync_file(row['path'])
        for directory in sorted((p for p in A.rglob('*') if p.is_dir()), reverse=True):
            sync_dir(directory)
        for directory in sorted({str(Path(r['path']).parent) for r in members}):
            sync_dir(directory)
        sync_dir(A); sync_dir(Q); sync_dir(M)
        barrier = save(barrier_path, {'expected_body_reads': 0, 'expected_body_read_scope': SCOPE06, 'members': members,
            'actual': observation, 'complete_actual_DPObservation': typed, 'compiled15': compiled,
            'directories': {'qa_actual': f03.tree(str(A)), 'past_compile': past},
            'Root_opaque_hashes_excluded': True, 'fsync': 'all member files and containing directories before 06 dispatch'})
        manifest = {'actual': observation, 'expected': expected, 'durable_freeze': barrier}
        saved = save(manifest_path, manifest); sync_dir(A)
        produced[barrier['path']] = barrier; produced[saved['path']] = saved
        if read(manifest_path) != manifest:
            raise ValueError('Root exact comparison operand binding differs')

    try:
        for phase in phases:
            label = phase['id']; cwd = Path(phase['cwd']); rd = receipts / label
            if any(r['accepted'] is not True for r in results):
                raise ValueError('prior phase not accepted')
            if sorted(p.name for p in cwd.iterdir()) != ['home', 'tmp'] or any(list((cwd / n).iterdir()) for n in ('home', 'tmp')):
                raise ValueError('phase cwd/home/tmp not fresh')
            if label == '02-kast-json' and results[-1].get('producer_status_and_pins', {}).get('status') != CARRIER_PASS:
                raise ValueError('01 producer status/pins not accepted; STOP before02')
            if label == '05-decode':
                printed = pin(phases[4]['stdout'])
                if printed not in results[-1]['streams'] or printed['bytes'] == 0:
                    raise ValueError('04 printed stream changed or empty')
                operand = {'source_input': public, 'printed': printed}
                dm = save(decode_path, operand); sync_dir(A); produced[dm['path']] = dm
                if read(decode_path) != operand:
                    raise ValueError('decode operand manifest differs')
            if label == '06-compare':
                capture = results[-1].get('capture_status_and_pins', {})
                if capture.get('status') != CAPTURE_PASS:
                    raise ValueError('05 capture not accepted; STOP before06')
                freeze06(capture['products']['complete_actual_observation'], capture['products']['complete_actual_DPObservation'])
            pre = save(rd / 'ROOT-PRECHECK.json', {'phase': label, **all_pins()})
            command = save(rd / 'COMMAND.json', {'id': label, 'argv': phase['argv'], 'environment': phase['environment'],
                'cwd': phase['cwd'], 'stdin': 'DEVNULL', 'limits': phase['limits']})
            retained = read(command['path'])
            if any(retained[k] != phase[k] for k in ('argv', 'environment', 'cwd')):
                raise ValueError('COMMAND disagrees with frozen phase')
            readset = save(rd / 'ROOT-READSET.json', {'argv_file_operands': [pin(p) for p in phase['argv'] if os.path.isfile(p)],
                'declared_pins': 'ROOT-PRECHECK static and produced pins', 'basis': 'conservative declared closure, not access instrumentation',
                'expected_parse_selected': label == '06-compare', 'Root_opaque_hashes_excluded': True})
            record = {'id': label}; selected.append(label)
            if label == '06-compare':
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
            clean = (record.get('outcome') == 'exited' and record.get('timed_out') is False
                and record.get('containment_breach') is False and record.get('remaining') == []
                and not unexpected and len(products) == len(phase['products']) and all(r['bytes'] > 0 for r in products))
            stdout = Path(phase['stdout']).read_bytes(); stderr = Path(phase['stderr']).read_bytes()
            ok = clean and record.get('returncode') == 0
            if label in ('02-kast-json', '04-print'):
                ok = ok and bool(stdout)
            if label == '01-render':
                try:
                    check = carrier_status(phase, expect01) if ok else {'not_evaluated': '01 not clean exit0'}
                except Exception as error:
                    check = {'failure': type(error).__name__ + ': ' + str(error)}
                ps = save(rd / 'PRODUCER-STATUS-PINS.json', check); produced[ps['path']] = ps
                results[-1]['producer_status_and_pins'] = check; ok = ok and check.get('status') == CARRIER_PASS
            if label == '02-kast-json':
                kast = save(rd / 'KAST02-ACCEPTANCE.json', {'stdout_nonempty': bool(stdout), 'stderr_empty': not stderr,
                    'dpObserve_prefix_observed_nongating': stdout.startswith(KAST02_PREFIX), 'streams': streams})
                produced[kast['path']] = kast; results[-1]['kast02_acceptance'] = kast; ok = ok and not stderr
            if label == '05-decode':
                try:
                    check = capture_status(phase, read(decode_path)) if ok else {'not_evaluated': '05 not clean exit0'}
                except Exception as error:
                    check = {'failure': type(error).__name__ + ': ' + str(error)}
                cs = save(rd / 'CAPTURE-STATUS-PINS.json', check); produced[cs['path']] = cs
                results[-1]['capture_status_and_pins'] = check; ok = ok and check.get('status') == CAPTURE_PASS
            if label == '06-compare':
                try:
                    comparison = loads(stdout)
                except Exception:
                    comparison = None
                compare_class = 'INFRASTRUCTURE_EXCEPTION_OR_INVALID_COMPARISON_STREAM'
                compared = results[-2]['capture_status_and_pins']['products']['complete_actual_observation']
                if (clean and not stderr and type(comparison) is dict and comparison.get('actual') == compared
                        and comparison.get('expected') == expected):
                    if record.get('returncode') == 0 and comparison.get('status') == COMPARE_PASS and comparison.get('whole_typed_equal') is True:
                        compare_class = 'PASS_WHOLE_TYPED'; expected_reads = 1
                    elif record.get('returncode') == 1 and comparison.get('status') == 'FAIL' and comparison.get('whole_typed_equal') is False:
                        compare_class = 'MISMATCH_NO_REPAIR_REDERIVE_OR_RETRY'; expected_reads = 1
                results[-1]['comparison_classification'] = compare_class; ok = compare_class == 'PASS_WHOLE_TYPED'
            results[-1]['accepted'] = ok; sync_dir(rd)
            if not ok:
                raise ValueError('fail-stop child: ' + label)
    except Exception as error:
        failure = {'type': type(error).__name__, 'message': str(error)}
    status = RESULT_PASS if failure is None and len(results) == 7 and all(r['accepted'] for r in results) else (
        'C6_DATA_OBSERVATION_MISMATCH' if compare_class.startswith('MISMATCH') else 'INFRASTRUCTURE_STOP')
    outcome = {'status': status, 'phases': results, 'failure': failure, 'selected_dispatches': selected,
        'unselected': [x for x in ORDER if x not in selected], 'comparison_classification': compare_class,
        'expected_body_reads': expected_reads, 'expected_parse_dispatch_selected': '06-compare' in selected,
        'expected_parse_attempted_count_unknown': expected_reads is None and '06-compare' in selected,
        'Root_private_expected_semantic_reads': 0, 'Root_opaque_hashes_excluded': True, 'no_compile_or_old_prefix_replay': True,
        'source_model_inputs_expected_argv_env_or_caps_changed': False,
        'scope': 'Positive finite capacity-padded DATA observation only; no arbitrary raw decoder, admission, native, or refinement claim',
        'final_qualifications': {'fixtures': 'one positive DATA observation UNADMITTED', 'JVM_heap_fit': 'UNOBSERVED; no headroom claim',
            'compile_warning_75887B_68fec773': 'UNADJUDICATED', 'stdlib_unpinned': True, 'hash_then_reread_not_atomic': True,
            'no_aggregate_CPU_RSS_disk_cap': True},
        'read_count_semantics': 'None = parse dispatch attempted, count unknown, never zero; Root opaque hashes are not semantic reads',
        'selection': selection}
    save(A / 'ROOT-RESULT.json', outcome); sync_dir(A)
    print(json.dumps(outcome, sort_keys=True))
    if status != RESULT_PASS:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
