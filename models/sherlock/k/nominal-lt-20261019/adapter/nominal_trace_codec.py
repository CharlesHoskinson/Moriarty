"""Complete nominal tape data codec; no evaluator or source transition rules."""
from dataclasses import dataclass
from pathlib import Path
import json
import re
API = {}
REG = {}
def configure(api_path, syntax_path):
    global API, REG
    API = json.loads(Path(api_path).read_text())
    syntax = re.sub(r'//[^\n]*', '', Path(syntax_path).read_text())
    REG = {}
    for sort, body in re.findall(r'syntax\s+(\w+)\s*::=\s*(.*?)(?=\n\s*syntax|\nendmodule)', syntax, re.S):
        for name, args in re.findall(r'(\w+)\(([^()]*)\)', body):
            REG[name] = (sort, tuple(x.strip() for x in args.split(',') if x.strip()))
    REG.update({'ngNoStates': ('NGStates', ()), 'ngStateRow': ('NGStates', ('NGState', 'NGStates')), 'ngTape': ('NGTape', ('NGState', 'NGStates', 'NGState')), 'ngInitial': ('NGState', ('NGProgram', 'Map'))})

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
        config = {'Variable': ('ngVariable', ['name_id']), 'Add': ('ngAdd', ['left', 'right']), 'Lt': ('ngLt', ['left', 'right']), 'Construct': ('ngConstruct', ['type_id', 'ctor_id', 'fields']), 'Call': ('ngCall', ['name_id', 'arguments']), 'Match': ('ngMatch', ['scrutinee', 'arms'])}
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
    if name == '@Map': return sorted([{'name_id': decode(k, ctx), 'value': decode(v, ctx)} for k, v in args], key=lambda row: row['name_id'])
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
        cfg = {'ngVariable': ('Variable', ['name_id']), 'ngAdd': ('Add', ['left', 'right']), 'ngLt': ('Lt', ['left', 'right']), 'ngConstruct': ('Construct', ['type_id', 'ctor_id', 'fields']), 'ngCall': ('Call', ['name_id', 'arguments']), 'ngMatch': ('Match', ['scrutinee', 'arms'])}
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

# Source-format data parsing only. No token, state, frame, or event is skipped.
# Parentheses/whitespace and Map's declared AC/unit syntax are presentation.
LEX = re.compile(r'\s*(\.Map|\|->|-?\d+|true\b|false\b|"(?:[^"\\]|\\.)*"|[A-Za-z][A-Za-z0-9]*|[(),])')

class PrintedTermParser:
    def __init__(self, text):
        self.tokens = []
        self.at = 0
        pos = 0
        while pos < len(text):
            found = LEX.match(text, pos)
            if found is None:
                if text[pos:].strip() == '': break
                raise ValueError('Unparsed printed byte at ' + str(pos))
            self.tokens.append(found.group(1))
            pos = found.end()

    def peek(self):
        return self.tokens[self.at] if self.at < len(self.tokens) else None

    def take(self, expected=None):
        token = self.peek()
        if token is None: raise ValueError('Unexpected end of complete tape')
        self.at += 1
        if expected is not None and token != expected:
            raise ValueError('Expected ' + expected + ', got ' + token)
        return token

    def term(self, sort):
        if sort == 'Map': return self.map_term()
        if self.peek() == '(':
            self.take('(')
            value = self.term(sort)
            self.take(')')
            return value
        token = self.take()
        if sort == 'Int':
            if re.fullmatch(r'-?\d+', token) is None: raise ValueError('Expected Int')
            return atom('Int', int(token))
        if sort == 'Bool':
            if token not in ('true', 'false'): raise ValueError('Expected Bool')
            return atom('Bool', token == 'true')
        if sort == 'String': return atom('String', json.loads(token))
        if token not in REG or REG[token][0] != sort:
            raise ValueError('Unknown constructor/sort ' + str((token, sort)))
        self.take('(')
        values = []
        for i, child_sort in enumerate(REG[token][1]):
            if i: self.take(',')
            values.append(self.term(child_sort))
        self.take(')')
        return C(token, *values)

    def map_term(self):
        rows = []
        saw_item = False
        while True:
            if self.peek() == '.Map':
                self.take('.Map'); saw_item = True
                continue
            checkpoint = self.at
            # Source environments have Int name IDs and NGValue bindings.
            # Optional key parentheses are surface formatting only.
            try:
                key = self.term('Int')
            except ValueError:
                self.at = checkpoint
                key = None
            if key is not None and self.peek() == '|->':
                self.take('|->')
                rows.append((key, self.term('NGValue')))
                saw_item = True
                continue
            self.at = checkpoint
            if self.peek() == '(':
                self.take('(')
                group = self.map_term()
                self.take(')')
                rows.extend(group.args)
                saw_item = True
                continue
            break
        if not saw_item: raise ValueError('Expected complete finite environment Map')
        # Retain all bindings, including any repeated rows; no dict overwrite.
        return T('Map', '@Map', tuple(sorted(rows, key=lambda row: row[0].args[0])))

    def complete(self, sort='NGTape'):
        value = self.term(sort)
        if self.at != len(self.tokens): raise ValueError('Trailing printed tape tokens')
        return value

def parse_printed(text, sort='NGTape'):
    return PrintedTermParser(text).complete(sort)

def expanded_expected(fixture):
    def state(s):
        return {'program': fixture['shared_program'], 'control': s['control'],
                'environment': s['environment'], 'frames_top_first': s['frames_top_first'],
                'events': fixture['expected_events'][:s['events_prefix_length']]}
    return {'initial': state(fixture['initial_state']),
            'after_each_event': [state(s) for s in fixture['after_each_event']],
            'final': state(fixture['expected_final_state'])}

