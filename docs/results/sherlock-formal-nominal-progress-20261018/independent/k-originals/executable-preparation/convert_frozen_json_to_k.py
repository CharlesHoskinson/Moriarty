"""Data-only lossless serialization of previously frozen independent literals.

No evaluator, subject rule, K tool, provider, or expected-value derivation runs.
"""
from pathlib import Path
from dataclasses import dataclass
from collections import Counter
import copy
import hashlib
import json
import re
import sys

sys.setrecursionlimit(50000)
HERE = Path(__file__).resolve().parent
BASE = HERE.parent
TEAM = BASE.parent
SAFE = TEAM / 'generic-nominal-product-model-v2-review-operation-20261018/source'
API_PATH = TEAM / 'generic-nominal-product-model-v2-20261018/PUBLIC-API.json'
API = json.loads(API_PATH.read_text())
FROZEN = json.loads((BASE / 'EXPECTATION-FREEZE.json').read_text())
assert hashlib.sha256((BASE / 'EXPECTATION-FREEZE.json').read_bytes()).hexdigest() == 'd4c9aee74c507299413c4009e7a4e8545f7c076b166876a8f32bde3cd47d7826'
for row in FROZEN['files']:
    assert hashlib.sha256((BASE / row['path']).read_bytes()).hexdigest() == row['sha256']
syntax = (SAFE / 'nominal-product-syntax.k').read_text()
assert not re.search(r'^\s*(rule|context)\b', syntax, re.M)
assert hashlib.sha256(syntax.encode()).hexdigest() == 'e80251e7aee1b7bf68bd86993f7d250cf06e7bbc4398b0f631b367d923310c20'
syntax = re.sub(r'//[^\n]*', '', syntax)
REG = {}
for sort, body in re.findall(r'syntax\s+(\w+)\s*::=\s*(.*?)(?=\n\s*syntax|\nendmodule)', syntax, re.S):
    for name, args in re.findall(r'(\w+)\(([^()]*)\)', body):
        REG[name] = (sort, tuple(x.strip() for x in args.split(',') if x.strip()))
REG.update({'ngNoStates': ('NGStates', ()), 'ngStateRow': ('NGStates', ('NGState', 'NGStates')), 'ngTape': ('NGTape', ('NGState', 'NGStates', 'NGState'))})

@dataclass(frozen=True)
class T:
    sort: str
    name: str
    args: tuple

def C(name, *args):
    sort, sorts = REG[name]
    assert tuple(a.sort for a in args) == sorts, (name, tuple(a.sort for a in args), sorts)
    return T(sort, name, tuple(args))

def atom(sort, value):
    return T(sort, '@' + sort, (value,))

def chain(items, empty, row, before=None):
    tail = C(empty)
    for x in reversed(items):
        args = before(x) if before else [x]
        tail = C(row, *args, tail)
    return tail

GROUPS = {'NGControl': 'control_payload_order', 'NGFrame': 'frame_payload_order', 'NGEvent': 'event_payload_order'}

def arms_index(program):
    return {a['source_arm_id']: a for n in program['expressions'] if 'Match' in n['kind'] for a in n['kind']['Match']['arms']}

def encode(sort, value, ctx):
    if sort in ('Int', 'Bool', 'String'):
        return atom(sort, value)
    if sort == 'Map':
        return T('Map', '@Map', tuple((encode('Int', x['name_id'], ctx), encode('NGValue', x['value'], ctx)) for x in value))
    if sort == 'NGValue':
        tag, v = next(iter(value.items()))
        if tag == 'UInt': return C('ngUInt', encode('Int', v, ctx))
        if tag == 'Bool': return C('ngBool', encode('Bool', v, ctx))
        return C('ngNominal', encode('Int', v['type_id'], ctx), encode('Int', v['ctor_id'], ctx), encode('NGValues', v['fields'], ctx))
    lists = {'NGValues': ('NGValue', 'ngNoValues', 'ngValue'), 'NGTypes': ('Int', 'ngNoTypes', 'ngType'), 'NGExprIds': ('Int', 'ngNoExprs', 'ngExprId'), 'NGPatterns': ('NGPattern', 'ngNoPatterns', 'ngPattern'), 'NGFrames': ('NGFrame', 'ngNoFrames', 'ngFrame'), 'NGEvents': ('NGEvent', 'ngNoEvents', 'ngEvent'), 'NGStates': ('NGState', 'ngNoStates', 'ngStateRow'), 'NGCtors': ('NGCtor', 'ngNoCtors', 'ngCtorRow'), 'NGFunctions': ('NGFunction', 'ngNoFunctions', 'ngFunctionRow')}
    if sort in lists:
        sub, empty, row = lists[sort]
        return chain([encode(sub, x, ctx) for x in value], empty, row)
    if sort == 'NGPath':
        return chain([encode('Int', x, ctx) for x in value], 'ngRoot', 'ngAt')
    if sort == 'NGPattern':
        tag, v = next(iter(value.items()))
        if tag == 'Wildcard': return C('ngWildcard')
        if tag == 'Bind': return C('ngBind', encode('Int', v, ctx))
        return C('ngConstructorPattern', encode('Int', v['type_id'], ctx), encode('Int', v['ctor_id'], ctx), encode('NGPatterns', v['fields'], ctx))
    if sort == 'NGArms':
        rows = [ctx['arms'][v] if isinstance(v, int) else v for v in value]
        return chain(rows, 'ngNoArms', 'ngArm', lambda v: [encode('Int', v['source_arm_id'], ctx), encode('NGPattern', v['pattern'], ctx), encode('Int', v['body'], ctx)])
    if sort == 'NGBindings':
        fields = ['name_id', 'match_expr_id', 'source_arm_id', 'field_path', 'value']
        return chain(value, 'ngNoBindings', 'ngBinding', lambda v: [encode(s, v[k], ctx) for s, k in zip(REG['ngBinding'][1][:-1], fields)])
    if sort in GROUPS:
        if sort == 'NGEvent': name, payload = value['event'], {k: v for k, v in value.items() if k != 'event'}
        else: name, payload = next(iter(value.items()))
        keys = API[GROUPS[sort]][name]
        assert set(payload) == set(keys)
        return C(name, *(encode(s, payload[k], ctx) for s, k in zip(REG[name][1], keys)))
    if sort == 'NGExpr':
        i, occ, kinds = value['id'], value['occurrence'], value['kind']
        tag, v = next(iter(kinds.items()))
        if tag == 'Literal': return C('ngLiteral', encode('Int', occ, ctx), encode('NGValue', v, ctx))
        config = {'Variable': ('ngVariable', ['name_id']), 'Add': ('ngAdd', ['left', 'right']), 'Construct': ('ngConstruct', ['type_id', 'ctor_id', 'fields']), 'Call': ('ngCall', ['name_id', 'arguments']), 'Match': ('ngMatch', ['scrutinee', 'arms'])}
        name, keys = config[tag]
        return C(name, encode('Int', occ, ctx), *(encode(s, v[k], ctx) for s, k in zip(REG[name][1][1:], keys)))
    if sort == 'NGNodes':
        return chain(value, 'ngNoNodes', 'ngNode', lambda v: [encode('Int', v['id'], ctx), encode('NGExpr', v, ctx)])
    if sort == 'NGParams':
        return chain(value, 'ngNoParams', 'ngParam', lambda v: [encode('Int', v['name_id'], ctx), encode('Int', v['type_id'], ctx)])
    if sort == 'NGFunction':
        return C('ngFunction', encode('Int', value['name_id'], ctx), encode('NGParams', value['parameters'], ctx), encode('Int', value['return_type_id'], ctx), encode('Int', value['body'], ctx))
    if sort == 'NGCtor':
        return C('ngCtor', *(encode(s, value[k], ctx) for s, k in zip(REG['ngCtor'][1], ['type_id', 'ctor_id', 'source_order', 'field_types'])))
    if sort == 'NGDecls':
        return chain(value, 'ngNoDecls', 'ngDecl', lambda v: [encode('Int', v['type_id'], ctx), encode('Int', v['recursive_group'], ctx), encode('NGCtors', v['ctors'], ctx)])
    if sort == 'NGProgram':
        return C('ngProgram', *(encode(s, value[k], ctx) for s, k in zip(REG['ngProgram'][1], ['declarations', 'expressions', 'functions', 'entry'])))
    if sort == 'NGState':
        return C('ngState', encode('NGProgram', value['program'], ctx), encode('NGControl', value['control'], ctx), encode('Map', value['environment'], ctx), encode('NGFrames', value['frames_top_first'], ctx), encode('NGEvents', value['events'], ctx))
    raise ValueError(sort)

def unchain(t):
    out = []
    while t.args:
        out.append(t.args[:-1]); t = t.args[-1]
    return out

def decode(t, ctx):
    s, name, args = t.sort, t.name, t.args
    if name in ('@Int', '@Bool', '@String'): return args[0]
    if name == '@Map': return [{'name_id': decode(k, ctx), 'value': decode(v, ctx)} for k, v in args]
    if s == 'NGValue':
        if name == 'ngUInt': return {'UInt': decode(args[0], ctx)}
        if name == 'ngBool': return {'Bool': decode(args[0], ctx)}
        return {'Nominal': dict(zip(['type_id', 'ctor_id', 'fields'], [decode(a, ctx) for a in args]))}
    lists = {'NGValues', 'NGTypes', 'NGExprIds', 'NGPatterns', 'NGFrames', 'NGEvents', 'NGStates', 'NGCtors', 'NGFunctions', 'NGPath'}
    if s in lists: return [decode(row[0], ctx) for row in unchain(t)]
    if s == 'NGPattern':
        if name == 'ngWildcard': return {'Wildcard': {}}
        if name == 'ngBind': return {'Bind': decode(args[0], ctx)}
        return {'Constructor': dict(zip(['type_id', 'ctor_id', 'fields'], [decode(a, ctx) for a in args]))}
    if s == 'NGArms': return [dict(zip(['source_arm_id', 'pattern', 'body'], [decode(a, ctx) for a in row])) for row in unchain(t)]
    if s == 'NGBindings': return [dict(zip(['name_id', 'match_expr_id', 'source_arm_id', 'field_path', 'value'], [decode(a, ctx) for a in row])) for row in unchain(t)]
    if s in GROUPS:
        keys = API[GROUPS[s]][name]; payload = {}
        for k, a in zip(keys, args):
            v = decode(a, ctx)
            if k in ('source_arms', 'remaining_source_arms'):
                assert all(ctx['arms'][v0['source_arm_id']] == v0 for v0 in v)
                v = [v0['source_arm_id'] for v0 in v]
            payload[k] = v
        return {'event': name, **payload} if s == 'NGEvent' else {name: payload}
    if s == 'NGExpr':
        occ = decode(args[0], ctx)
        if name == 'ngLiteral': return {'occurrence': occ, 'kind': {'Literal': decode(args[1], ctx)}}
        cfg = {'ngVariable': ('Variable', ['name_id']), 'ngAdd': ('Add', ['left', 'right']), 'ngConstruct': ('Construct', ['type_id', 'ctor_id', 'fields']), 'ngCall': ('Call', ['name_id', 'arguments']), 'ngMatch': ('Match', ['scrutinee', 'arms'])}
        tag, keys = cfg[name]
        return {'occurrence': occ, 'kind': {tag: dict(zip(keys, [decode(a, ctx) for a in args[1:]]))}}
    if s == 'NGNodes': return [{'id': decode(row[0], ctx), **decode(row[1], ctx)} for row in unchain(t)]
    if s == 'NGParams': return [dict(zip(['name_id', 'type_id'], [decode(a, ctx) for a in row])) for row in unchain(t)]
    if s == 'NGFunction': return dict(zip(['name_id', 'parameters', 'return_type_id', 'body'], [decode(a, ctx) for a in args]))
    if s == 'NGCtor': return dict(zip(['type_id', 'ctor_id', 'source_order', 'field_types'], [decode(a, ctx) for a in args]))
    if s == 'NGDecls': return [dict(zip(['type_id', 'recursive_group', 'ctors'], [decode(a, ctx) for a in row])) for row in unchain(t)]
    if s == 'NGProgram': return dict(zip(['declarations', 'expressions', 'functions', 'entry'], [decode(a, ctx) for a in args]))
    if s == 'NGState': return dict(zip(['program', 'control', 'environment', 'frames_top_first', 'events'], [decode(a, ctx) for a in args]))
    if s == 'NGTape': return {'initial': decode(args[0], ctx), 'after_each_event': decode(args[1], ctx), 'final': decode(args[2], ctx)}
    raise ValueError(s)

def render(t):
    if t.name == '@Int': return str(t.args[0])
    if t.name == '@Bool': return str(t.args[0]).lower()
    if t.name == '@String': return json.dumps(t.args[0])
    if t.name == '@Map':
        return '.Map' if not t.args else '(.Map ' + ' '.join('(' + render(k) + ' |-> ' + render(v) + ')' for k, v in t.args) + ')'
    if t.name == '@Ref': return t.args[0] + '()'
    return t.name + '(' + ','.join(render(a) for a in t.args) + ')'

TOKEN = re.compile(r'\s*(\.Map|\|->|-?\d+|true\b|false\b|"(?:[^"\\]|\\.)*"|[A-Za-z][A-Za-z0-9]*|[(),])')
def parse(text):
    tokens = TOKEN.findall(text); at = 0
    def take(want=None):
        nonlocal at
        x = tokens[at]; at += 1
        if want is not None: assert x == want, (want, x)
        return x
    def term():
        nonlocal at
        x = take()
        if x == '.Map': return T('Map', '@Map', ())
        if x == '(':
            take('.Map'); rows = []
            while tokens[at] == '(':
                take('('); k = term(); take('|->'); v = term(); take(')'); rows.append((k, v))
            take(')'); return T('Map', '@Map', tuple(rows))
        if re.fullmatch(r'-?\d+', x): return atom('Int', int(x))
        if x in ('true', 'false'): return atom('Bool', x == 'true')
        if x.startswith('"'): return atom('String', json.loads(x))
        take('('); args = []
        if tokens[at] != ')':
            args.append(term())
            while tokens[at] == ',': take(','); args.append(term())
        take(')'); return C(x, *args)
    value = term(); assert at == len(tokens); return value

def reorder(template, value):
    if isinstance(template, dict): return {k: reorder(v, value[k]) for k, v in template.items()}
    if isinstance(template, list): return [reorder(t, v) for t, v in zip(template, value)]
    return value

def expand_snapshot(x, snap):
    return {'program': x['shared_program'], 'control': snap['control'], 'environment': snap['environment'], 'frames_top_first': snap['frames_top_first'], 'events': x['expected_events'][:snap['events_prefix_length']]}

def ref(sort, name): return T(sort, '@Ref', (name,))
def children(t):
    if t.name == '@Map': return tuple(z for row in t.args for z in row)
    if t.name.startswith('@'): return ()
    return t.args

raw_dir = HERE / 'raw-tapes'; raw_dir.mkdir(exist_ok=True)
query_dir = HERE / 'queries'; query_dir.mkdir(exist_ok=True)
roots = {}; tapes = []; records = []; proofs = []
for i, source in enumerate(sorted((BASE / 'expected').glob('*.json'))):
    x = json.loads(source.read_text()); ctx = {'arms': arms_index(x['shared_program'])}
    expanded = {'initial': expand_snapshot(x, x['initial_state']), 'after_each_event': [expand_snapshot(x, s) for s in x['after_each_event']], 'final': expand_snapshot(x, x['expected_final_state'])}
    tape = C('ngTape', encode('NGState', expanded['initial'], ctx), encode('NGStates', expanded['after_each_event'], ctx), encode('NGState', expanded['final'], ctx))
    literal = render(tape) + '\n'; target = raw_dir / (x['case_id'] + '.ng'); target.write_text(literal)
    parsed = parse(literal); assert parsed == tape
    decoded = decode(parsed, ctx); assert decoded == expanded
    def compress(s):
        assert s['program'] == x['shared_program']
        assert s['events'] == x['expected_events'][:len(s['events'])]
        return {'program_binding': 'shared-program-with-case-entry', 'control': s['control'], 'environment': s['environment'], 'frames_top_first': s['frames_top_first'], 'events_prefix_length': len(s['events'])}
    back = {'case_id': x['case_id'], 'shared_program': decoded['initial']['program'], 'initial_state': compress(decoded['initial']), 'expected_events': decoded['final']['events'], 'after_each_event': [compress(s) for s in decoded['after_each_event']], 'expected_final_state': compress(decoded['final'])}
    assert back == x
    roundtrip = json.dumps(reorder(x, back), indent=2) + '\n'; assert roundtrip.encode() == source.read_bytes()
    ev_name = 'xe' + str(i)
    roots[ev_name] = encode('NGEvents', x['expected_events'], ctx)
    def factored_state(s):
        q = encode('NGState', s, ctx)
        return T('NGState', 'ngState', q.args[:-1] + (T('NGEvents', 'xp', (ref('NGEvents', ev_name), atom('Int', len(s['events'])))),))
    initial = factored_state(expanded['initial']); after = [factored_state(s) for s in expanded['after_each_event']]; final = factored_state(expanded['final'])
    program_name = 'inP' + str(i); environment_name = 'inE' + str(i)
    roots[program_name] = encode('NGProgram', x['shared_program'], ctx)
    roots[environment_name] = encode('Map', x['initial_state']['environment'], ctx)
    tape_name = 'xt' + str(i)
    roots[tape_name] = C('ngTape', initial, chain(after, 'ngNoStates', 'ngStateRow'), final)
    tapes.append((tape_name, tape))
    stepcount = len(x['expected_events']); bound = stepcount + 10
    input_term = 'ngInitial(' + render(encode('NGProgram', x['shared_program'], ctx)) + ',' + render(encode('Map', x['initial_state']['environment'], ctx)) + ')'
    actual = f'ngObserve({input_term},{bound})'
    for kind, text in [('actual', actual), ('expected', tape_name + '()'), ('equal', actual + ' ==K ' + tape_name + '()')]:
        (query_dir / (x['case_id'] + '-' + kind + '.ng')).write_text(text + '\n')
    compact_actual = f'ngObserve(ngInitial({program_name}(),{environment_name}()),{bound})'
    compact_queries = {}
    for kind, text in [('actual', compact_actual), ('expected', tape_name + '()'), ('equal', compact_actual + ' ==K ' + tape_name + '()')]:
        f = query_dir / (x['case_id'] + '-compact-' + kind + '.ng'); f.write_text(text + '\n'); compact_queries[kind] = str(f.relative_to(HERE))
    records.append({'case_id': x['case_id'], 'frozen_expected_json': str(source.relative_to(BASE)), 'expected_tape_literal': str(target.relative_to(HERE)), 'expected_tape_function': tape_name, 'input_program_function': program_name, 'input_environment_function': environment_name, 'initial_input_case': x['case_id'], 'hand_derived_reference_events': stepcount, 'finite_observer_margin': 10, 'observation_bound': bound, 'compact_queries': compact_queries, 'queries': {k: str((query_dir / (x['case_id'] + '-' + k + '.ng')).relative_to(HERE)) for k in ['actual', 'expected', 'equal']}})
    proofs.append({'case_id': x['case_id'], 'raw_literal_parse_restores_typed_ast': True, 'decoded_complete_program_controls_maps_frames_events_equal_expanded_json': True, 'restored_original_json_bytes_sha256': hashlib.sha256(roundtrip.encode()).hexdigest(), 'frozen_json_sha256': hashlib.sha256(source.read_bytes()).hexdigest()})

# Greedy acyclic sharing is a serialization choice; no subject evaluation.
selected = {}; names = {}; serial = 0
def walked(t, counter, skip=None):
    if t in selected and t != skip: return
    if not t.name.startswith('@') and t.name != 'xp': counter[t] += 1
    for c in children(t): walked(c, counter, skip)
def rshared(t, skip=None):
    if t in names and t != skip: return names[t] + '()'
    if t.name == '@Map': return '.Map' if not t.args else '(.Map ' + ' '.join('(' + rshared(k) + ' |-> ' + rshared(v) + ')' for k, v in t.args) + ')'
    if t.name.startswith('@'): return render(t)
    return t.name + '(' + ','.join(rshared(c) for c in t.args) + ')'
for iteration in range(2000):
    counts = Counter()
    for t in roots.values(): walked(t, counts)
    for t in selected: walked(t, counts, t)
    choices = []
    for t, count in counts.items():
        if count < 2 or t in selected: continue
        size = len(rshared(t)); helper = 'x' + str(serial)
        overhead = len(f' syntax {t.sort} ::= {helper}() [function]\n rule {helper}() => ') + size + 1
        savings = count * size - (count * (len(helper) + 2) + overhead)
        if savings > 2: choices.append((savings, size, render(t), t))
    if not choices: break
    t = max(choices, key=lambda x: (x[0], x[1], x[2]))[-1]
    selected[t] = True; names[t] = 'x' + str(serial); serial += 1

module = ['requires "normal-trace-observations.k"', '', '// Expected literals only. No call to ngStep, ngObserve, or ngAdvance.', 'module SHERLOCK-NOMINAL-PRODUCT-GOLDENS', ' imports SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS', ' syntax NGEvents ::= xp(NGEvents,Int) [function]', ' rule xp(Es,0) => ngNoEvents()', ' rule xp(ngEvent(E,Es),N) => ngEvent(E,xp(Es,N -Int 1)) requires N >Int 0']
declarations = {}
for t in selected: declarations.setdefault(t.sort, []).append(names[t])
for name, t in roots.items(): declarations.setdefault(t.sort, []).append(name)
for sort, func_names in declarations.items(): module.append(' syntax ' + sort + ' ::= ' + ' | '.join(n + '() [function]' for n in func_names))
for t in selected:
    name = names[t]; module.append(f' rule {name}() => {rshared(t, t)}')
for name, t in roots.items(): module.append(f' rule {name}() => {rshared(t)}')
module += ['endmodule', '']
module_text = '\n'.join(module); (HERE / 'independent-expected-tapes.k').write_text(module_text)

# Independently expand the exact generated factoring graph back to literal AST.
by_name = {name: t for t, name in names.items()}; by_name.update(roots)
def expand(t, skip=None):
    if t.name == '@Ref': return expand(by_name[t.args[0]])
    if t.name == 'xp':
        evs = expand(t.args[0]); count = t.args[1].args[0]
        rows = unchain(evs); assert 0 <= count <= len(rows)
        return chain([row[0] for row in rows[:count]], 'ngNoEvents', 'ngEvent')
    if t.name == '@Map': return T('Map', '@Map', tuple((expand(k), expand(v)) for k, v in t.args))
    if t.name.startswith('@'): return t
    return T(t.sort, t.name, tuple(expand(a) for a in t.args))
for name, raw in tapes:
    assert expand(roots[name]) == raw

# Parse the exact emitted constant-definition RHS bytes independently of the
# original sharing graph, then expand those parsed definitions to raw ASTs.
REG['xp'] = ('NGEvents', ('NGEvents', 'Int'))
for name, t in by_name.items(): REG[name] = (t.sort, ())
parsed_defs = {}
for line in (HERE / 'independent-expected-tapes.k').read_text().splitlines():
    match = re.fullmatch(r' rule ([A-Za-z][A-Za-z0-9]*)\(\) => (.*)', line)
    if match: parsed_defs[match[1]] = parse(match[2])
assert set(parsed_defs) == set(by_name)
def expand_written(t):
    if t.name in parsed_defs and not t.args: return expand_written(parsed_defs[t.name])
    if t.name == 'xp':
        evs = expand_written(t.args[0]); count = t.args[1].args[0]
        rows = unchain(evs); assert 0 <= count <= len(rows)
        return chain([row[0] for row in rows[:count]], 'ngNoEvents', 'ngEvent')
    if t.name == '@Map': return T('Map', '@Map', tuple((expand_written(k), expand_written(v)) for k, v in t.args))
    if t.name.startswith('@'): return t
    return T(t.sort, t.name, tuple(expand_written(a) for a in t.args))
for name, raw in tapes:
    assert expand_written(T(raw.sort, name, ())) == raw

REG['ngInitial'] = ('NGState', ('NGProgram', 'Map'))
REG['ngObserve'] = ('NGTape', ('NGState', 'Int'))
for row in records:
    raw_actual = parse((HERE / row['queries']['actual']).read_text())
    compact_actual = parse((HERE / row['compact_queries']['actual']).read_text())
    expanded_actual = expand_written(compact_actual)
    assert expanded_actual == raw_actual
    assert render(expanded_actual) + '\n' == (HERE / row['queries']['actual']).read_text()

for row in records:
    for key in ['expected_tape_literal']:
        f = HERE / row[key]; row[key + '_bytes'] = f.stat().st_size; row[key + '_sha256'] = hashlib.sha256(f.read_bytes()).hexdigest()
    row['compact_query_pins'] = {k: {'bytes': (HERE / v).stat().st_size, 'sha256': hashlib.sha256((HERE / v).read_bytes()).hexdigest()} for k, v in row['compact_queries'].items()}
    row['query_pins'] = {k: {'bytes': (HERE / v).stat().st_size, 'sha256': hashlib.sha256((HERE / v).read_bytes()).hexdigest()} for k, v in row['queries'].items()}
manifest = {'kind': 'METADATA_ONLY_NO_POPULATED_VALUES_OR_TRACES', 'original_independent_freeze_sha256': 'd4c9aee74c507299413c4009e7a4e8545f7c076b166876a8f32bde3cd47d7826', 'module': {'path': 'independent-expected-tapes.k', 'main_module': 'SHERLOCK-NOMINAL-PRODUCT-GOLDENS', 'syntax_module': 'SHERLOCK-NOMINAL-PRODUCT-GOLDENS', 'bytes': len(module_text.encode()), 'sha256': hashlib.sha256(module_text.encode()).hexdigest()}, 'cases': records, 'bound_qualification': 'Reference observer operation count only, finite margin10; no A/B charge/fuel/native mapping', 'author_boundary': 'Primary model author may opaque-copy/hash module, literal/query bytes, never parse/print expected payloads; safe metadata only is readable', 'no_subject_execution': True}
(HERE / 'SAFE-MANIFEST.json').write_text(json.dumps(manifest, indent=2) + '\n')
proof = {'kind': 'DATA_ONLY_ROUNDTRIP_NOT_K_EXECUTION_OR_ACCEPTANCE', 'original_freeze_verified_unchanged': True, 'syntax_only_pin': 'e80251e7aee1b7bf68bd86993f7d250cf06e7bbc4398b0f631b367d923310c20', 'api_pin': hashlib.sha256(API_PATH.read_bytes()).hexdigest(), 'cases': proofs, 'lossless_shared_program_prefix_and_subterm_expansion_equals_every_raw_tape_ast': True, 'exact_emitted_constant_rhs_parsed_and_expanded_to_every_raw_tape_ast': True, 'input_only_compact_actual_queries_expand_to_byte_exact_raw_actual_queries': True, 'factored_helpers': len(selected), 'factored_module_bytes': len(module_text.encode()), 'model_rules_read_or_run': False, 'K_parser_compiler_or_model_run': False, 'limitation': 'Rendered raw literals were parsed only by the included data-only literal parser. K syntax/typecheck/compile/reduction and actual-result compatibility remain unobserved.'}
(HERE / 'ROUNDTRIP-PROOF.json').write_text(json.dumps(proof, indent=2) + '\n')
print(json.dumps({'factored_module_bytes': len(module_text.encode()), 'factored_helpers': len(selected), 'raw_literal_bytes': sum((HERE / r['expected_tape_literal']).stat().st_size for r in records), 'roundtrips_verified': len(proofs), 'no_subject_execution': True}))
