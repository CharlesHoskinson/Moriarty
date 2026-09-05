from __future__ import annotations
import argparse
import hashlib
import inspect
import json
import re
from pathlib import Path,PurePosixPath
import sys
if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from scripts.a4_carrier import R,M,V,Invalid,require,loads,decode,encode,typed
from scripts.a4_inventory import descriptors,history
from scripts.a4_agreement import core
from scripts.a4_json_stream import iter_array_document, hash_stream, JsonStreamError, ResourceLimit

ROOT=Path(__file__).resolve().parents[1]
FROZEN_EXTRA={
 'moriarty/__init__.py':'6d6b58f9875c357d5d16858e55ce81edd1e56ecb1c5b28476c7a911cad2e1127',
 'specs/quint/s02/consumption.qnt':'3dd07f3c5f274aff4fce4de9c245aa0b1f1fe7f68e29e5de95a0b1c42a00f93b',
 'specs/quint/s02/policies.qnt':'0886619203c3de16f2326e27e9d4ae2ddfda8c54742c231dd4e824397c5fa9fe',
 'specs/quint/s02/authorization.qnt':'b59779d5e2e7f1bf0a14952dfe1cd3ab70bcfe813210d690abbb3e3205a6df61',
 'specs/quint/s02/execution.qnt':'cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928',
 'specs/quint/s02/candidate_a_authority_adapter.qnt':'3f3b093f4c718dd08eda38e610de700d0a24138beb82fd7b2b12dcf9d300bda8',
 'specs/quint/s02/candidate_a_authority_boundary.qnt':'95ddf75ef32b432acaec5384c89826d0dc12245b80f78fd8a60ff78d0da8164b',
 'specs/quint/s02/candidate_a_authority_swap.qnt':'294633d4213d44075df76085f67b9bd24fab23d07bc863fff8b4f22e79d10024',
 'specs/quint/s02/candidate_a_authority_swap_fixtures.qnt':'fb407555584e167ae0fddc3e59fbf6d64ecd79a00d2ad03d02f159da8429ed15',
 'specs/quint/s02/candidate_a_authority_swap_harness.qnt':'bb8991e439153af69f3c8ec52df1771786a0ce0d5360f975189958b978555e29',
 'specs/quint/s02/candidate_a_authority_swap_test.qnt':'5aed1719310ef472f8fdd5a19e8cc5188a47c7c136f7fc3158bb5a12943636c5',
 'specs/quint/s02/candidate_a_authority_installment.qnt':'f12d91938098d48a313baf7cb5218f84b6bd840da8c518d54f195e0f7fa1e4cd',
 'specs/quint/s02/candidate_a_authority_installment_fixtures.qnt':'22d975d6d615e1f8e80c453115ded79fa68f783880a036f73470814221bf1a92',
 'specs/quint/s02/candidate_a_authority_installment_harness.qnt':'9a49b2a76d0c9e27a0d06b942b4f6a3338bf54a1e005cbba6d6388e91dcf82ca',
 'specs/quint/s02/candidate_a_authority_installment_test.qnt':'b1d9c21c5285105b382525df8df2ebaa41dbf10979f400d975b40500c3f7c3d1',
 'scripts/check_s02_candidate_a_correspondence.py':'c6923d0e08ec0206dfddad70eaff5a3a9a2f4a160328802e0e69be965be90aec',
}
TRANSPORT = 'case-sharded-itf-v1'
ENTRIES = {d['case_id']: f'specs/quint/s02/candidate_a_integrated_case_{i:03d}.qnt'
           for i, d in enumerate(descriptors())}
PYTHON_SOURCES=frozenset(('moriarty/__init__.py','moriarty/core.py','moriarty/swap.py','scripts/check_s02_candidate_a_correspondence.py',
    'scripts/a4_json_stream.py','tests/test_a4_json_stream.py','scripts/a4_carrier.py','scripts/a4_agreement.py','scripts/a4_authority.py','scripts/a4_cases.py','scripts/a4_inventory.py',
    'scripts/check_s02_candidate_a_integrated.py','scripts/s02_candidate_a_integrated_inventory.py',
    'scripts/export_s02_candidate_a_integrated.py','scripts/record_s02_candidate_a_integrated.py',
    'tests/test_s02_candidate_a_integrated.py','tests/test_s02_candidate_a_integrated_export.py'))
QNT_TESTS=('specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt',
    'specs/quint/s02/candidate_a_integrated_export_test.qnt',
    'specs/quint/s02/candidate_a_authority_installment_test.qnt','specs/quint/s02/candidate_a_authority_swap_test.qnt')
RAW_NAMES = frozenset(f'raw/case-{i:03d}.itf.json' for i in range(78))
CASE_NAMES = frozenset(f'cases/case-{i:03d}.json' for i in range(78))
VARS=('authorityState','caseIndex','cursor','latestEvent')
EVENT_FIELDS=frozenset(('case_id','profile','sequence','kind','arguments','observed_guard','computations','before','after','provenance'))
RENAME={'case_id':'caseId','profile':'profile','sequence':'sequence','kind':'kind','arguments':'arguments','observed_guard':'observedGuard','computations':'computations'}

def canonical(value): return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def digest(data): return hashlib.sha256(data).hexdigest()
def fields(value,names,label): require(type(value) is dict and set(value)==set(names),label+' fields')
def same(a,b): return canonical(encode(a))==canonical(encode(b))
def raw_same(a,b): return canonical(a)==canonical(b)
def safe_path(root,name):
    require(type(name) is str and name!='' and '\\' not in name,'relative path string')
    path=PurePosixPath(name)
    require(not path.is_absolute() and all(p not in ('','..','.') for p in name.split('/')) and str(path)==name,'canonical relative path')
    resolved=root.resolve(); current=resolved
    for component in path.parts:
        current=current/component; require(not current.is_symlink(),'symlink forbidden')
    require(current.resolve().is_relative_to(resolved) and current.is_file(),'file containment')
    return current
def file_digest(path):
    with Path(path).open('rb') as stream: return hash_stream(stream)[0]
def pin_shape(pins):
    require(type(pins) is dict and pins, 'nonempty pin map')
    for name, sha in pins.items():
        require(type(name) is str, 'pin path string')
        require(type(sha) is str and re.fullmatch('[0-9a-f]{64}', sha), 'SHA256 format')
def pin_bytes(root,pins):
    pin_shape(pins)
    for name, sha in pins.items():
        require(file_digest(safe_path(root,name)) == sha, 'hash mismatch '+name)
def qnt_closure(root,entries):
    found=set(); todo=list(entries)
    while todo:
        name=todo.pop()
        if name in found: continue
        source=safe_path(root,name).read_text(); found.add(name)
        # Frozen files use relative from clauses. Refuse unsupported absolute imports.
        for suffix in re.findall(r'\bfrom\s+"([^"]+)"',source):
            require(suffix.startswith('./'),'unsupported Quint import path')
            child=str(PurePosixPath(name).parent/(suffix[2:]+'.qnt'))
            todo.append(child)
    return found
def inventory():
    return {'schema_version':2,'cases':[d|{'event_count':len(history(d))} for d in descriptors()]}

def shape(raw,expected):
    # Decode equality alone conflates Python bool/int and ITF tuple/list; reject both.
    encoded=encode(expected)
    if type(encoded) in (int,bool,str):
        if type(encoded) is int and type(raw) is dict and set(raw)=={'#bigint'}:
            require(type(decode(raw)) is int,'integer carrier'); return
        require(type(raw) is type(encoded),'scalar carrier'); return
    if isinstance(encoded,list):
        require(type(raw) is list and len(raw)==len(encoded),'ordered-list carrier')
        for a,b in zip(raw,expected): shape(a,b)
        return
    require(type(raw) is dict and set(raw)==set(encoded),'record/carrier keys')
    if isinstance(expected,M):
        require(type(raw['#map']) is list and len(raw['#map'])==len(expected.pairs),'map carrier')
        values={canonical(encode(k)):(k,v) for k,v in expected.pairs}
        for pair in raw['#map']:
            require(type(pair) is list and len(pair)==2,'map pair')
            k=decode(pair[0]); key=canonical(encode(k)); require(key in values,'map key set')
            ek,ev=values[key]
            if type(ek) is tuple:
                require(type(pair[0]) is dict and set(pair[0])=={'#tup'} and len(pair[0]['#tup'])==len(ek),'tuple map key')
                for rawkey,expectedkey in zip(pair[0]['#tup'],ek): shape(rawkey,expectedkey)
            else: shape(pair[0],ek)
            shape(pair[1],ev)
        return
    if isinstance(expected,frozenset):
        require(type(raw['#set']) is list and len(raw['#set'])==len(expected),'set carrier')
        values={canonical(encode(v)):v for v in expected}
        for x in raw['#set']:
            k=canonical(encode(decode(x))); require(k in values,'set member'); shape(x,values[k])
        return
    if isinstance(expected,V):
        require(type(raw['tag']) is str,'tag scalar')
        if expected.value==() and expected.tag!='EffectsExtractedA': require(raw['value']=={'#tup':[]},'unit carrier')
        else: shape(raw['value'],expected.value)
        return
    require(isinstance(expected,R),'record carrier')
    for name,value in expected.fields: shape(raw[name],value)

def compare_event(supplied, want):
    fields(supplied, EVENT_FIELDS, 'event')
    require(type(supplied['sequence']) is int, 'exported sequence integer')
    for public, internal in RENAME.items():
        value = want['latest'][internal]
        shape(supplied[public], value)
        require(same(decode(supplied[public]), value), 'semantic event '+public)
    for boundary in ('before', 'after'):
        actual = typed(decode(supplied[boundary]), 'Execution')
        shape(supplied[boundary], want[boundary])
        require(same(actual, want[boundary]), 'semantic state '+boundary)


def compare_case(case, descriptor):
    fields(case, set(descriptor)|{'events'}, 'case')
    require(all(type(case[k]) is str and case[k] == v for k, v in descriptor.items()), 'case descriptor')
    expected = history(descriptor)
    require(type(case['events']) is list and len(case['events']) == len(expected), 'event count')
    for supplied, want in zip(case['events'], expected): compare_event(supplied, want)
    return expected


def bind_window(event, previous, current, index, input_name, input_sha):
    p = event['provenance']
    fields(p, ('input_path', 'input_sha256', 'before_index', 'after_index'), 'provenance')
    require(p['input_path'] == input_name and p['input_sha256'] == input_sha, 'input pin linkage')
    require(type(p['before_index']) is int and type(p['after_index']) is int, 'integer provenance index')
    require(p['after_index'] == index and p['before_index'] == max(0, index-1), 'adjacent provenance indices')
    for name, internal in RENAME.items():
        if name == 'sequence':
            raw_sequence = decode(current['latestEvent'][internal])
            require(type(event[name]) is int and type(raw_sequence) is int and event[name] == raw_sequence, 'raw provenance sequence')
            continue
        exported_raw = canonical(event[name])
        raw_field = canonical(current['latestEvent'][internal])
        require(exported_raw == raw_field, 'raw provenance field '+name)
    require(raw_same(event['before'], previous['authorityState']) and raw_same(event['after'], current['authorityState']), 'raw authority state linkage')


def bind_event(event, raw, index, input_name, input_sha):
    # Small buffered test adapter; actual package admission uses bind_window.
    return bind_window(event, raw['states'][max(0, index-1)], raw['states'][index], index, input_name, input_sha)


def array_items(stream, target, metadata):
    allowed = {'states': {'#meta', 'vars'},
               'events': {'case_id', 'lifecycle', 'profile', 'scenario', 'control'}}
    require(target in allowed, 'stream target')
    ended = False
    for kind, key, value in iter_array_document(stream, target):
        if kind == 'field':
            require(key in allowed[target], 'unexpected stream metadata')
            metadata[key] = value
        elif kind == 'item': yield value
        else:
            require(kind == 'end' and key == target and not ended, 'array completion')
            ended = True
    require(ended, 'missing document end')


def validate_raw_metadata(metadata, entry):
    fields(metadata, ('#meta', 'vars'), 'ITF')
    meta = metadata['#meta']
    fields(meta, ('format', 'format-description', 'source', 'status', 'description', 'timestamp'), 'ITF metadata')
    require(meta['format'] == 'ITF' and meta['source'] == entry, 'ITF source/format metadata')
    require(meta['format-description'] == 'https://apalache-mc.org/docs/adr/015adr-trace.html', 'ITF format description')
    require(meta['status'] == 'ok' and type(meta['description']) is str and type(meta['timestamp']) is int, 'ITF terminal metadata')
    require(metadata['vars'] == list(VARS), 'exact raw vars and order')


def validate_raw_state(state, index, global_index):
    fields(state, ('#meta', *VARS), 'ITF state')
    fields(state['#meta'], ('index',), 'ITF state metadata')
    require(type(state['#meta']['index']) is int and state['#meta']['index'] == index, 'raw state metadata index')
    case_index = decode(state['caseIndex'])
    cursor = decode(state['cursor'])
    require(type(case_index) is int and case_index == global_index, 'raw global case index')
    require(type(cursor) is int and cursor == index, 'raw cursor')
    fields(state['latestEvent'], RENAME.values(), 'latest event')


def check_shard(case_path, raw_path, descriptor, global_index, input_sha):
    require(type(global_index) is int and 0 <= global_index < 78, 'global case index')
    require(raw_same(descriptor, descriptors()[global_index]), 'fixed shard descriptor')
    input_name = f'raw/case-{global_index:03d}.itf.json'
    entry = ENTRIES[descriptor['case_id']]
    require(file_digest(raw_path) == input_sha, 'raw shard hash')
    expected = history(descriptor)
    case_metadata = {}
    raw_metadata = {}
    absent = object()
    with Path(case_path).open('rb') as case_stream, Path(raw_path).open('rb') as raw_stream:
        submitted = array_items(case_stream, 'events', case_metadata)
        states = array_items(raw_stream, 'states', raw_metadata)
        previous = None
        for index, want in enumerate(expected):
            event = next(submitted, absent)
            current = next(states, absent)
            require(event is not absent and current is not absent, 'missing shard event/state')
            validate_raw_state(current, index, global_index)
            compare_event(event, want)
            bind_window(event, current if previous is None else previous, current, index, input_name, input_sha)
            if index == 0:
                require(event['kind'] == 'case-start' and raw_same(event['before'], event['after']), 'initial before after identity')
            else:
                require(event['kind'] != 'case-start', 'hidden reset')
                require(raw_same(event['before'], previous['authorityState']), 'history continuity')
            previous = current
        # Exhaust both generators through suffix fields and strict EOF.
        require(next(submitted, absent) is absent, 'extra shard event')
        require(next(states, absent) is absent, 'extra raw state')
    fields(case_metadata, descriptor, 'case descriptor fields')
    require(raw_same(case_metadata, descriptor), 'case descriptor')
    validate_raw_metadata(raw_metadata, entry)
    return len(expected)


def shard_records(inv):
    return [row | {'global_index': index, 'entry': ENTRIES[row['case_id']],
                   'input_path': f'raw/case-{index:03d}.itf.json',
                   'case_path': f'cases/case-{index:03d}.json'}
            for index, row in enumerate(inv['cases'])]


def manifest_contract(document, admission):
    fields(document, ('schema_version', 'transport', 'inventory_sha256', 'source_pins', 'input_pins', 'case_pins', 'receipt_pins', 'shards'), 'document')
    fields(admission, ('schema_version', 'transport', 'inventory_sha256', 'source_pins', 'input_pins', 'case_pins', 'receipt_pins', 'entries'), 'admission')
    require(type(document['schema_version']) is int and document['schema_version'] == 3 and type(admission['schema_version']) is int and admission['schema_version'] == 3, 'schema3')
    require(document['transport'] == admission['transport'] == TRANSPORT, 'transport')
    require(raw_same(admission['entries'], ENTRIES), 'admitted entry modules')
    for group in ('source_pins', 'input_pins', 'case_pins', 'receipt_pins'):
        pin_shape(document[group])
        require(raw_same(document[group], admission[group]), 'admitted pin map '+group)
    require(set(document['input_pins']) == RAW_NAMES, 'complete raw input inventory')
    require(set(document['case_pins']) == CASE_NAMES, 'complete case file inventory')
    groups = [set(document[name]) for name in ('input_pins', 'case_pins', 'receipt_pins')]
    require(all(not groups[i]&groups[j] for i in range(3) for j in range(i)), 'input/case/receipt separation')


def validate_sources(source_root, sourcepins):
    expected_sources = qnt_closure(source_root, tuple(ENTRIES.values())+QNT_TESTS)|PYTHON_SOURCES
    require(set(sourcepins) == expected_sources, 'complete source/tooling closure')
    pin_bytes(source_root, sourcepins)
    for name, pin in (core.PINNED|FROZEN_EXTRA).items(): require(sourcepins.get(name) == pin, 'frozen source pin '+name)
    origin = inspect.getsourcefile(core)
    require(origin is not None and file_digest(origin) == sourcepins['scripts/check_s02_candidate_a_correspondence.py'], 'imported Core bridge origin')
    for name in ('a4_carrier', 'a4_agreement', 'a4_authority', 'a4_cases', 'a4_inventory', 'a4_json_stream'):
        module = sys.modules['scripts.'+name]
        origin = inspect.getsourcefile(module)
        require(origin is not None and file_digest(origin) == sourcepins['scripts/'+name+'.py'], 'imported checker origin '+name)
    require(file_digest(__file__) == sourcepins['scripts/check_s02_candidate_a_integrated.py'], 'executed checker origin')
    for fn in (core.compute_transaction, core.reduce_to_quiescence):
        origin = inspect.getsourcefile(fn)
        require(origin is not None and file_digest(origin) == core.PINNED['moriarty/core.py'], 'imported frozen Core origin')


def load_manifest(path):
    # Manifests are small; unbounded raw/case arrays never pass through here.
    with Path(path).open('rb') as stream: data = stream.read(16777217)
    if len(data) > 16777216: raise ResourceLimit('manifest byte limit')
    return loads(data)


def check_package(manifest_path, admission_path, source_root, input_root, inventory_path):
    document = load_manifest(manifest_path)
    admission = load_manifest(admission_path)
    manifest_contract(document, admission)
    inv = inventory()
    require(raw_same(load_manifest(inventory_path), inv), 'independent inventory mismatch')
    invhash = digest(canonical(inv).encode())
    require(document['inventory_sha256'] == admission['inventory_sha256'] == invhash, 'inventory hash')
    require(raw_same(document['shards'], shard_records(inv)), 'complete ordered shard inventory')
    validate_sources(Path(source_root), document['source_pins'])
    input_root = Path(input_root)
    for group in ('input_pins', 'case_pins', 'receipt_pins'): pin_bytes(input_root, document[group])
    count = 0
    for index, descriptor in enumerate(descriptors()):
        count += check_shard(safe_path(input_root, f'cases/case-{index:03d}.json'),
                             safe_path(input_root, f'raw/case-{index:03d}.itf.json'),
                             descriptor, index, document['input_pins'][f'raw/case-{index:03d}.itf.json'])
    require(count == 1557, 'complete event count')
    return {'ok': True, 'scope': 'complete-inventory finite-record agreement-and-authority replay',
            'transport': TRANSPORT, 'shards': 78, 'cases': 78, 'events': count,
            'validated_inputs': len(document['input_pins']), 'validated_case_files': len(document['case_pins']),
            'validated_receipts': len(document['receipt_pins']),
            'symbolic_premises': ['EvidenceValid', 'finite signature token abstraction'],
            'limitations': ['not model checking', 'not cryptographic verification',
                            'root admission is not authenticated generator execution',
                            'shared lexical transport utility; independent semantic rules']}


def main():
    parser = argparse.ArgumentParser()
    for name in ('cases', 'admission', 'inventory', 'source-root', 'input-root'): parser.add_argument('--'+name, type=Path, required=True)
    parser.add_argument('--report', type=Path)
    args = parser.parse_args()
    try:
        report = check_package(args.cases, args.admission, args.source_root, args.input_root, args.inventory)
    except ResourceLimit as error:
        report = {'ok': False, 'scope': 'complete-inventory', 'failure_kind': 'resource', 'differences': [str(error)]}
    except (Invalid, JsonStreamError, core.DecodeError, ValueError, TypeError, KeyError, IndexError, OSError) as error:
        report = {'ok': False, 'scope': 'complete-inventory', 'failure_kind': 'invalid', 'differences': [str(error)]}
    if args.report is not None: args.report.write_text(json.dumps(report, sort_keys=True, indent=2)+'\n')
    print(json.dumps(report, sort_keys=True))
    return 0 if report['ok'] else 1


if __name__ == '__main__': raise SystemExit(main())
