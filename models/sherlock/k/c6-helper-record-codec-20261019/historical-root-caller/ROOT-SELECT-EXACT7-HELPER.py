"""Root-only fixed exact7 typed-root runtime QA selection using existing F03.launch.

Ordinary adaptation of the retained new8 caller (27149B c3b7a088...). No containment,
launch policy, runner or subject rule is added; the old schema-compare phase is removed
because this profile has one independent whole typed expected. Root must read the full
actual review and this caller, then select explicitly. The author has not imported or
executed this caller or any subject child.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import sys

M = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019')
Q = M / 'runtime-qa-postcompile-01'
A = Q / 'actual'
COMPILED = M / 'actual/c6-helper-kompiled'
BASE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
# ROOT_BIND_AFTER_MATERIALIZATION: exact typed three-key pins computed by Root
# from the materialized helper plan, source freeze, conditions, opaque
# binding index, and render manifest. No hash is asserted in this source.
PLAN_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/runtime-qa-postcompile-01/revision02-complete-owner-code-01/PROPOSED-EXACT7-RUNTIME-OPERATIONS.json', 'bytes': 29729, 'sha256': 'a29e6fd1416f15fe12371d076383e5856b89681a36276e94caaf7e4f3b723aba'}
FREEZE_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/runtime-qa-postcompile-01/revision02-complete-owner-code-01/SOURCE-FREEZE.json', 'bytes': 10523, 'sha256': '743f527d0039fa7de5ad91db888133b91800d01af13b60eb332a32e818270608'}
CONDITIONS_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/runtime-qa-postcompile-01/revision02-complete-owner-code-01/ROOT-CONDITIONS.json', 'bytes': 4266, 'sha256': '0ca5b57c6cdfbb186ae7ffd612bc336296e4c6a2331c417f202736856de41dd3'}
BINDING_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/runtime-qa-postcompile-01/revision02-complete-owner-code-01/OPAQUE-EXPECTED-BINDING.json', 'bytes': 2035, 'sha256': '89c43040b679b78e0e2649eeee15c6d3f3e464f36ba85209f09497cdf7126e0a'}
RENDER_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/runtime-qa-postcompile-01/revision02-complete-owner-code-01/RENDER-MANIFEST.json', 'bytes': 631, 'sha256': 'a3ea20c8f193f0d4166d9b977e476db6e7c23ff9d94c8e0a730643d67d6e3571'}
LABELQ_PIN = {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/postcompile-label-qualification-01/ACTUAL-LABEL-QUALIFICATION.json', 'bytes': 7177, 'sha256': 'b1018cbfc6eebec1b34973f0b7047f22615ab979e4f26118619f48c9e4b38aa8'}
FROZEN = [PLAN_PIN, FREEZE_PIN, CONDITIONS_PIN, BINDING_PIN, RENDER_PIN, LABELQ_PIN]
PLAN = Path(PLAN_PIN['path']) if PLAN_PIN else None
FREEZE = Path(FREEZE_PIN['path']) if FREEZE_PIN else None
CONDITIONS = Path(CONDITIONS_PIN['path']) if CONDITIONS_PIN else None
BINDING = Path(BINDING_PIN['path']) if BINDING_PIN else None
ORDER = ['00-data-unit', '01-render', '02-kast-json', '03-reduce', '04-print', '05-decode', '06-compare']
FSIZE = 536870912
TOKEN = 'SHERLOCK_K_ATTEMPT_TOKEN'
CASES = 31; HELPERS = 11; RESULTS = 20; LABELS = 209; DATA_LABELS = 208
CARRIER_PASS = 'PASS_LOSSLESS_C6_HELPER_QUERY_CARRIER'
CAPTURE_PASS = 'PASS_COMPLETE_C6_HELPER_ANSWER_CAPTURE'
CAPTURE_FAIL_DECODED = 'FAIL_DECODED_SOURCE_WHOLE_CARRIER_MISMATCH'
COMPARE_PASS = 'PASS_WHOLE_TYPED_C6_HELPER_CODEC'
RESULT_PASS = 'PASS_SCOPED_C6_HELPER_RECORD_CODEC'
SCOPE06 = 'independent-complete-C6-helper-result-records-31'
OUTPUT_SCHEMA = 'sherlock-c6-helper-record-positive-codec-wire/1'
PROFILE = 'logical-C6-helper-record-positive-codec/1'
KAST02_PREFIX = b'LblchObserveAll'
HELPER_CASE_IDS = ['helper-none', 'helper-env-get', 'helper-env-put', 'helper-find', 'helper-bind', 'helper-append', 'helper-validate', 'helper-equal', 'helper-match', 'helper-plan', 'helper-build']
RESULT_CASE_IDS = ['result-none', 'result-validated', 'result-env-found', 'result-env-missing', 'result-env-updated', 'result-find-found', 'result-find-missing', 'result-bind-ok', 'result-bind-bad', 'result-append-done', 'result-scalar-value', 'result-scalar-error', 'result-scalar-selector', 'result-scalar-type-test', 'result-equal-done', 'result-match-matched', 'result-match-no-match', 'result-plan-matched', 'result-plan-no-match', 'result-build-done']
FIXED_WRAPPER = {'schema': 'sherlock-independent-3b-positive-record-expected/1', 'scope': 'Complete derivable record lane vectors; Scalar helper unresolved, no section/task expectations.', 'missing': {'carrier': 'HelperControlC::Scalar / ScalarStaged', 'exact_missing_premise': 'Numeric ScalarStaged tag-to-variant mapping for None, U128Value, BoolValue, PendingNode, PrimError, Selector and TypeTest; header supplies tag range and width, not this mapping.', 'consequence': 'No Scalar helper vector or ScalarStaged-specific vector is derived, including None. No declaration-order-as-tag guess or producer inspection.', 'public_variants': ['None', 'U128Value', 'BoolValue', 'PendingNode', 'PrimError', 'Selector', 'TypeTest'], 'fullwide_staged_value': 'U128Value full-width payload remains owed with its tag map; common.origin_reference_cursor full-width transport is independently covered here.'}}


def pin(path):
    path=Path(path);raw=path.read_bytes()
    return {'path':str(path),'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}


def exact(row):
    if (type(row)is not dict or set(row)!={'path','bytes','sha256'} or type(row['path'])is not str
            or type(row['bytes'])is not int or row['bytes']<0 or type(row['sha256'])is not str
            or len(row['sha256'])!=64 or any(c not in '0123456789abcdef' for c in row['sha256'])):
        raise ValueError('exact typed three-key file pin required')
    return row


def checked(row):
    want=exact({k:row[k] for k in ('path','bytes','sha256')})
    if pin(row['path'])!=want:raise ValueError('pin changed: '+row['path'])
    if 'resolved_path' in row and str(Path(row['path']).resolve())!=row['resolved_path']:
        raise ValueError('resolved tool path changed')
    return want


def unique(pairs):
    out={}
    for k,v in pairs:
        if k in out:raise ValueError('duplicate Root metadata key')
        out[k]=v
    return out


def loads(raw):
    return json.loads(raw,object_pairs_hook=unique,
        parse_constant=lambda x:(_ for _ in ()).throw(ValueError(x)))


def read(path):return loads(Path(path).read_bytes())


def save(path,value):
    with Path(path).open('xb') as stream:
        stream.write((json.dumps(value,sort_keys=True,indent=2,allow_nan=False)+'\n').encode())
        stream.flush();os.fsync(stream.fileno())
    return pin(path)


def sync_file(path):
    with Path(path).open('rb') as stream:os.fsync(stream.fileno())


def sync_dir(path):
    fd=os.open(path,os.O_RDONLY|os.O_DIRECTORY)
    try:os.fsync(fd)
    finally:os.close(fd)


def tree_pins(root):
    entries=sorted(Path(root).rglob('*'))
    if any(p.is_symlink() for p in entries):raise ValueError('symlink in generated file inventory')
    return [pin(p) for p in entries if p.is_file()]


def sorted_rows(rows):return sorted(rows,key=lambda r:r['path'])


def canonical_pin(row,path):
    # Producer metadata only: its exact pin must equal the canonical on-disk file.
    exact(row)
    if str(Path(path).resolve())!=path or row!=pin(path):raise ValueError('producer pin differs from canonical on-disk: '+path)
    return row


def is_int(value,want):return type(value)is int and value==want


def same(a,b):
    # Exact typed JSON equality, as existing c6_data.same.
    if type(a)is not type(b):return False
    if type(a)is dict:return a.keys()==b.keys() and all(same(a[k],b[k]) for k in a)
    if type(a)is list:return len(a)==len(b) and all(same(x,y) for x,y in zip(a,b))
    return a==b


def merge(rows):
    out={}
    for row in rows:
        three=exact({k:row[k] for k in ('path','bytes','sha256')})
        if out.setdefault(three['path'],three)!=three:raise ValueError('conflicting pins for one path: '+three['path'])
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
            or not is_int(invariant.get('helper_case_count'), HELPERS) or not is_int(invariant.get('result_case_count'), RESULTS)
            or not is_int(invariant.get('case_count'), CASES) or invariant.get('same_whole_typed_query') is not True
            or not is_int(invariant.get('expected_body_reads'), 0) or invariant.get('no_model_or_lane_evaluation_in_Python') is not True):
        raise ValueError('01 INVARIANT-REPORT status/product/input/definition/map/count differs')
    return {'status': CARRIER_PASS, **pins}


def capture_status(phase, operand, helpers, results):
    vectors_path, answers_path, report = phase['products']
    status = loads(Path(phase['stdout']).read_bytes())
    if type(status) is not dict or set(status) != {'status', 'products', 'report'} or status['status'] not in (CAPTURE_PASS, CAPTURE_FAIL_DECODED):
        raise ValueError('05 stdout is not exactly one capture status JSON')
    if type(status['products']) is not dict or set(status['products']) != {'complete_actual_record_vectors', 'complete_actual_CHAnswers'}:
        raise ValueError('05 stdout products keyset differs')
    pins = {'complete_actual_record_vectors': canonical_pin(status['products']['complete_actual_record_vectors'], vectors_path),
            'complete_actual_CHAnswers': canonical_pin(status['products']['complete_actual_CHAnswers'], answers_path),
            'report': canonical_pin(status['report'], report)}
    capture = read(report)
    if (type(capture) is not dict or capture.get('status') != status['status']
            or capture.get('products') != {'complete_actual_record_vectors': pins['complete_actual_record_vectors'], 'complete_actual_CHAnswers': pins['complete_actual_CHAnswers']}
            or capture.get('product') != pins['complete_actual_record_vectors']
            or capture.get('source_input') != operand['source_input'] or capture.get('printed') != operand['printed']
            or not is_int(capture.get('helper_case_count'), HELPERS) or not is_int(capture.get('result_case_count'), RESULTS)
            or not is_int(capture.get('case_count'), CASES)
            or capture.get('all_source_original_carriers_retained') is not True
            or capture.get('all_actual_encoded_lanes_and_decoded_carriers_retained') is not True
            or type(capture.get('decoded_source_whole_carrier_equal')) is not bool
            or not is_int(capture.get('expected_body_reads'), 0)):
        raise ValueError('05 CAPTURE-REPORT status/products/operands/count differs')
    if status['status'] == CAPTURE_PASS and capture.get('decoded_source_whole_carrier_equal') is not True:
        raise ValueError('05 PASS status contradicts decoded whole-carrier identity')
    if status['status'] == CAPTURE_FAIL_DECODED and capture.get('decoded_source_whole_carrier_equal') is not False:
        raise ValueError('05 mismatch status contradicts decoded whole-carrier identity')
    wire = read(answers_path)
    if (type(wire) is not dict or set(wire) != {'schema', 'profile', 'answers'} or wire['schema'] != OUTPUT_SCHEMA
            or wire['profile'] != PROFILE or type(wire['answers']) is not list or len(wire['answers']) != CASES):
        raise ValueError('05 complete CHAnswers schema/profile/count differs')
    combined = [('chHelperAnswer', r, 'CHHelperOption') for r in helpers] + [('chResultAnswer', r, 'CHResultOption') for r in results]
    for row, (constructor, case, option_sort) in zip(wire['answers'], combined):
        args = row.get('arguments') if type(row) is dict else None
        if (type(row) is not dict or set(row) != {'sort', 'constructor', 'arguments'} or row['sort'] != 'CHAnswer'
                or row['constructor'] != constructor or type(args) is not list or len(args) != 4
                or not same(args[0], {'sort': 'String', 'value': case['case_id']})
                or type(args[1]) is not dict or args[1].get('sort') != option_sort
                or type(args[2]) is not dict or args[2].get('sort') != 'CTLanes'
                or type(args[3]) is not dict or args[3].get('sort') != option_sort):
            raise ValueError('05 answer identity/order/carrier/lane structure differs')
    wrapper = read(vectors_path)
    if (type(wrapper) is not dict or set(wrapper) != {'schema', 'scope', 'helper_vectors', 'result_vectors', 'missing'}
            or wrapper['schema'] != FIXED_WRAPPER['schema'] or wrapper['scope'] != FIXED_WRAPPER['scope']
            or not same(wrapper['missing'], FIXED_WRAPPER['missing'])
            or type(wrapper['helper_vectors']) is not list or len(wrapper['helper_vectors']) != HELPERS
            or type(wrapper['result_vectors']) is not list or len(wrapper['result_vectors']) != RESULTS):
        raise ValueError('05 vector wrapper fixed metadata/count differs')
    if [r.get('case_id') for r in wrapper['helper_vectors']] != HELPER_CASE_IDS or [r.get('case_id') for r in wrapper['result_vectors']] != RESULT_CASE_IDS:
        raise ValueError('05 vector case order differs')
    for records, width in ((wrapper['helper_vectors'], 23), (wrapper['result_vectors'], 4)):
        for record in records:
            if (type(record) is not dict or set(record) != {'case_id', 'variant', 'lanes'}
                    or type(record['case_id']) is not str
                    or not (record['variant'] is None or type(record['variant']) is str)
                    or type(record['lanes']) is not list or len(record['lanes']) != width
                    or any(type(x) is not int for x in record['lanes'])):
                raise ValueError('05 vector record/native lane structure differs')
    if status['status'] == CAPTURE_FAIL_DECODED:
        return {'status': CAPTURE_FAIL_DECODED, 'products': {k: pins[k] for k in ('complete_actual_record_vectors', 'complete_actual_CHAnswers')}, 'report': pins['report'], 'decoded_source_whole_carrier_equal': False}
    return {'status': CAPTURE_PASS, 'products': {k: pins[k] for k in ('complete_actual_record_vectors', 'complete_actual_CHAnswers')}, 'report': pins['report'], 'decoded_source_whole_carrier_equal': True}


def main():
    sys.dont_write_bytecode = True
    if any(row is None for row in FROZEN):
        raise ValueError('Root helper plan/freeze/conditions/binding/render/label pins not yet bound')
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
    if ([x['id'] for x in phases] != ORDER or plan['no_compile_or_old8_prefix_replay'] is not True
            or plan['callback'].get('oldmain_ledger_retry_unselected') is not True or plan['source_freeze'] != FREEZE_PIN
            or plan['conditions'] != CONDITIONS_PIN or plan['opaque_binding'] != BINDING_PIN
            or plan['actual_label_qualification'] != LABELQ_PIN):
        raise ValueError('frozen helper order/operands differ')
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
    if public != {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/Chunk3B-positive-helper-result-20261020/SAFE/INPUTS.json', 'bytes': 11221, 'sha256': 'd824cd01c0be8b9668bed718d8db671db7c5f22d9532714d1df043770a365633'}:
        raise ValueError('public original-input binding differs')
    binding = read(BINDING); expected = exact(binding['expected']); private = exact(binding['private_freeze'])
    if dyn['before05']['source_input'] != public or dyn['before06']['expected'] != expected or (expected['bytes'], private['bytes']) != (8389, 3560):
        raise ValueError('opaque expected/private freeze or source input binding differs')
    for opaque in (expected, private):
        path = Path(opaque['path'])
        if str(path.resolve()) != str(path) or any(p.is_symlink() for p in [path, *path.parents]):
            raise ValueError('opaque operand not canonical symlink-free')
    compiled = plan['compiled15_reuse']
    definition = [r for r in compiled if r['path'] == str(COMPILED / 'definition.kore')]
    label_map = [r for r in freeze['members'] if r['path'] == '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/postcompile-label-qualification-01/COMPILED-KLABEL-MAP.json']
    if len(label_map) != 1 or label_map[0] != {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-helper-control-result-codec-20261019/postcompile-label-qualification-01/COMPILED-KLABEL-MAP.json', 'bytes': 270342, 'sha256': 'd1b6122c6d5601cd86fb90f822f6bf1921ec2cff5c57a5057d863415a9c73be9'}:
        raise ValueError('compiled label map binding differs')
    render = read(RENDER_PIN['path']); labelq = read(LABELQ_PIN['path'])
    if (len(compiled) != 15 or len(definition) != 1
            or render != {'source_input': public, 'compiled_definition': definition[0]}
            or labelq.get('map') != label_map[0]):
        raise ValueError('compiled15/definition/actual209 map binding differs')
    expect01 = {'source_input': public, 'definition': definition[0], 'label_map': label_map[0]}
    tools = read(checked(plan['tools'])['path']); closure = read(checked(plan['K_builtin_closure'])['path'])
    callback = [exact(plan['callback'][k]) for k in ('fullsource', 'full_callable_closure', 'full_import_projection')]
    raw = [*FROZEN, *freeze['members'], freeze['model_freeze'], *freeze['model_sources'], public, freeze['registry'],
           freeze['output_root_declaration'], *freeze['reused_helper_sources'], *compiled, *tools['entries'],
           *tools['classpath_jar_membership'], *closure['files'], *callback, plan['tools'], plan['K_builtin_closure'],
           labelq['actual_compile_result'], labelq['compiled15'], labelq['primary']['source'], binding['SAFE_handoff'],
           pin(Path(__file__).resolve()), pin(ack_path), opinion]
    merge(raw); rows = list({r['path']: r for r in raw}.values())
    for row in rows:
        checked(row)
    source_doc = read(public['path'])
    helpers = source_doc.get('helper_cases'); results_cases = source_doc.get('result_cases')
    if (type(helpers) is not list or type(results_cases) is not list or len(helpers) != HELPERS
            or len(results_cases) != RESULTS or [r.get('case_id') for r in helpers] != HELPER_CASE_IDS
            or [r.get('case_id') for r in results_cases] != RESULT_CASE_IDS
            or len({r['case_id'] for r in helpers + results_cases}) != CASES):
        raise ValueError('current 11/20/31 source identities differ')
    registry_rows = read(freeze['registry']['path'])['constructors']
    helper_answer_rows = [r for r in registry_rows if r['name'] == 'chHelperAnswer']
    result_answer_rows = [r for r in registry_rows if r['name'] == 'chResultAnswer']
    if (len(helper_answer_rows) != 1 or helper_answer_rows[0]['sort'] != 'CHAnswer'
            or helper_answer_rows[0]['argument_sorts'] != ['String', 'CHHelperOption', 'CTLanes', 'CHHelperOption']
            or len(result_answer_rows) != 1 or result_answer_rows[0]['sort'] != 'CHAnswer'
            or result_answer_rows[0]['argument_sorts'] != ['String', 'CHResultOption', 'CTLanes', 'CHResultOption']):
        raise ValueError('public chHelperAnswer/chResultAnswer declarations differ')
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
            raise ValueError('__pycache__ present in source/helper directory')
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
        'render01_status_pins_gate_before02': True, 'kast02_acceptance': 'clean exit0, nonempty stdout, EMPTY stderr (conservative STOP); chObserveAll prefix non-gating',
        'compile_warning_53062B_ec611a63': 'UNADJUDICATED', 'no_compile_or_old8_prefix_replay': True, 'expected_body_reads': 0,
        'Root_opaque_hashes_excluded': True, 'no_aggregate_CPU_RSS_disk_cap': True, 'stdlib_unpinned': True, 'hash_then_reread_not_atomic': True})
    produced[selection['path']] = selection
    local = [r for r in rows if not r['path'].startswith('/nix/store/')]
    results = []; selected = []; failure = None; compare_class = 'NOT_REACHED'; expected_reads = 0

    def freeze06(vectors, answers):
        if str(Path(vectors['path']).resolve()) != vectors['path'] or vectors != pin(phases[5]['products'][0]):
            raise ValueError('05 vectors product not canonical/current')
        if str(Path(answers['path']).resolve()) != answers['path'] or answers != pin(phases[5]['products'][1]):
            raise ValueError('05 CHAnswers product not canonical/current')
        past_files = [pin(r['path']) for r in f03.tree(str(M / 'actual')) if r['kind'] == 'file']
        members = merge([*local, *produced.values(), *tree_pins(A), *past_files])
        if vectors not in members or answers not in members:
            raise ValueError('05 products absent from complete barrier')
        for row in members:
            checked(row); sync_file(row['path'])
        for directory in sorted((p for p in A.rglob('*') if p.is_dir()), reverse=True):
            sync_dir(directory)
        for directory in sorted({str(Path(r['path']).parent) for r in members}):
            sync_dir(directory)
        sync_dir(A); sync_dir(Q); sync_dir(M)
        barrier = save(barrier_path, {'expected_body_reads': 0, 'expected_body_read_scope': SCOPE06, 'members': members,
            'actual': vectors, 'complete_actual_CHAnswers': answers, 'compiled15': compiled,
            'directories': {'qa_actual': f03.tree(str(A)), 'past_compile': past},
            'Root_opaque_hashes_excluded': True, 'fsync': 'all member files and containing directories before 06 dispatch'})
        manifest = {'actual': vectors, 'expected': expected, 'durable_freeze': barrier}
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
                freeze06(capture['products']['complete_actual_record_vectors'], capture['products']['complete_actual_CHAnswers'])
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
                    'chObserveAll_prefix_observed_nongating': stdout.startswith(KAST02_PREFIX), 'streams': streams})
                produced[kast['path']] = kast; results[-1]['kast02_acceptance'] = kast; ok = ok and not stderr
            if label == '05-decode':
                try:
                    check = capture_status(phase, read(decode_path), helpers, results_cases) if ok else {'not_evaluated': '05 not clean exit0'}
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
                compared = results[-2]['capture_status_and_pins']['products']['complete_actual_record_vectors']
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
        'C6_HELPER_RECORD_MISMATCH' if compare_class.startswith('MISMATCH') else 'INFRASTRUCTURE_STOP')
    outcome = {'status': status, 'phases': results, 'failure': failure, 'selected_dispatches': selected,
        'unselected': [x for x in ORDER if x not in selected], 'comparison_classification': compare_class,
        'expected_body_reads': expected_reads, 'expected_parse_dispatch_selected': '06-compare' in selected,
        'expected_parse_attempted_count_unknown': expected_reads is None and '06-compare' in selected,
        'Root_private_expected_semantic_reads': 0, 'Root_opaque_hashes_excluded': True, 'no_compile_or_old8_prefix_replay': True,
        'source_model_inputs_expected_argv_env_or_caps_changed': False,
        'scope': 'Positive UNADMITTED C6 helper/result record codec only; no Scalar helper, interpreter, native, or refinement claim',
        'final_qualifications': {'fixtures': '31 UNADMITTED', 'JVM_heap_fit': 'UNOBSERVED; no headroom claim',
            'compile_warning_53062B_ec611a63': 'UNADJUDICATED', 'stdlib_unpinned': True, 'hash_then_reread_not_atomic': True,
            'no_aggregate_CPU_RSS_disk_cap': True},
        'read_count_semantics': 'None = parse dispatch attempted, count unknown, never zero; Root opaque hashes are not semantic reads',
        'selection': selection}
    save(A / 'ROOT-RESULT.json', outcome); sync_dir(A)
    print(json.dumps(outcome, sort_keys=True))
    if status != RESULT_PASS:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
