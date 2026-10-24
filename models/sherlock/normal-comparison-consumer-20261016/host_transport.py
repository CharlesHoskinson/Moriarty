"""Explicit presentation-only map to tagged Rust JSON; no machine execution.
Candidate06 supplies exact depth/taskLive/segBase readonly carriers. Other gaps remain explicit. Missing transport never becomes equality.
"""
import re
from pathlib import Path
from normal_compare import read
from normal_compare import require, differences

ENUM_FIELDS = {'grammarDomain', 'finance', 'op', 'sort', 'returnType', 'mode', 'expected',
               'phase', 'status', 'visit', 'sel', 'patTag', 'eventKind', 'site', 'helperKind',
               'disposition', 'resultTag', 'outcomeTag', 'boundary', 'reason'}
HELPERS = {'Scalar', 'EnvGet', 'EnvPut', 'Find', 'Bind', 'Append', 'Validate', 'Equal', 'Match'}


def snake(key):
    return re.sub(r'(?<!^)(?=[A-Z])', '_', key).lower()


def escape(key):
    return str(key).replace('~', '~0').replace('/', '~1')


def leaves(value, path):
    if type(value) is dict and value:
        for key, child in value.items():
            yield from leaves(child, path + '/' + escape(key))
    elif type(value) is list and value:
        for i, child in enumerate(value):
            yield from leaves(child, path + '/' + str(i))
    else:
        yield path, value


class HostTransport:
    def __init__(self):
        self.mapping = []
        self.gaps = []
        self.shapes = []
        registry = read(Path(__file__).resolve().parent / 'RULE-ID-TRANSPORT.json')
        self.rules = registry['forward']
        require({v: k for k, v in self.rules.items()} == registry['inverse'], '/ruleTransport', 'non-bijective frozen spelling lookup')

    def atom(self, value, source, target, operation='identity', output=None):
        self.mapping.append({'expectedPath': source, 'actualPath': target, 'operation': operation})
        return value if output is None else output

    def gap(self, value, source, reason):
        for path, atom in leaves(value, source):
            self.gaps.append({'expectedPath': path, 'reason': reason, 'expected': atom})

    def value(self, value, source, target, field=None):
        if field == 'ruleId':
            require(type(value) is str and value in self.rules, source,
                    'rule-ID spelling is outside the frozen normal presentation lookup')
            return {'$variant': self.atom(value, source, target + '/$variant',
                                          'dotted-rule-to-enum', self.rules[value])}
        if value is None:
            return self.atom(value, source, target)
        if type(value) is str and field in ENUM_FIELDS:
            return {'$variant': self.atom(value, source, target + '/$variant', 'unit-enum')}
        if type(value) is list:
            if not value: self.atom(value, source, target, 'empty-array')
            return [self.value(v, source + '/' + str(i), target + '/' + str(i)) for i, v in enumerate(value)]
        if type(value) is not dict:
            return self.atom(value, source, target)
        if field in {'layoutVersion', 'fieldId', 'scalarLayoutId'}:
            require(set(value) == {'namespace', 'name'}, source, 'named pair expected')
            return [self.atom(value['namespace'], source + '/namespace', target + '/0', 'named-pair'),
                    self.atom(value['name'], source + '/name', target + '/1', 'named-pair')]
        if not value:
            self.atom(value, source, target, 'empty-object')
            return {}
        out = {}
        helper = value.get('tag') in HELPERS and 'site' in value
        for key, v in value.items():
            s = source + '/' + escape(key)
            if helper and key in {'site', 'originReferenceCursor', 'parentResume'}:
                out.setdefault('common', {})[snake(key)] = self.value(v, s, target + '/common/' + snake(key), key)
            elif key == 'tag':
                out['$variant'] = self.atom(v, s, target + '/$variant', 'tag-key')
            elif key == 'ref' and value.get('tag') == 'ValidateTask':
                out['r'] = self.value(v, s, target + '/r')
            elif key == 'n' and value.get('tag') == 'Nat':
                out['value'] = self.value(v, s, target + '/value')
            elif key == 'ruleId':
                out['rule_id'] = self.value(v, s, target + '/rule_id', 'ruleId')
            else:
                out[snake(key)] = self.value(v, s, target + '/' + snake(key), key)
        return out

    def program(self, value, source, target):
        out = {}
        for key, v in value.items():
            s = source + '/' + key
            if key == 'activeLengths':
                for table, length in v.items():
                    self.shapes.append({'expectedPath': s + '/' + table, 'actualPath': target + '/' + snake(table),
                                        'operation': 'array-length', 'expected': length})
            elif key == 'occurrences':
                out[key] = []
                if not v: self.atom(v, s, target + '/' + key, 'empty-array')
                for i, row in enumerate(v):
                    p, q = s + '/' + str(i), target + '/occurrences/' + str(i)
                    require(set(row) == {'owner', 'path'}, p, 'unrecognized occurrence shape')
                    owner = row['owner']
                    if owner == 'Entry' or owner == {'tag': 'Entry'}:
                        tag_path = p + '/owner' if type(owner) is str else p + '/owner/tag'
                        owned = self.atom('Entry', tag_path, q + '/owner_is_function', 'owner-entry-bit', False)
                        # Inactive optional Function payload is not inferred as an integer zero.
                        function = None
                        self.gaps.append({'expectedPath': tag_path, 'reason': 'inactive Rust occurrence.function usize has no separate expected literal; no invented zero'})
                    else:
                        require(type(owner) is dict, p, 'owner object expected')
                        if set(owner) == {'Function'}:
                            owned = True
                            # The sole Function key is the frozen sum discriminator.
                            index = owner['Function']; index_path = p + '/owner/Function'
                        else:
                            require(set(owner) == {'tag', 'index'} and owner['tag'] == 'Function', p, 'unknown function owner')
                            owned = self.atom('Function', p + '/owner/tag', q + '/owner_is_function', 'owner-function-bit', True)
                            index = owner['index']; index_path = p + '/owner/index'
                        function = self.atom(index, index_path, q + '/function', 'owner-function-payload')
                    out[key].append({'owner_is_function': owned, 'function': function,
                                     'path': self.value(row['path'], p + '/path', q + '/path')})
            elif key == 'names':
                out['names'] = []
                if not v: self.atom(v, s, target + '/names', 'empty-array')
                for i, name in enumerate(v):
                    require(set(name) == {'bytes'} and type(name['bytes']) is str, s, 'literal name String carrier required')
                    out['names'].append(self.atom(name['bytes'], s + '/' + str(i) + '/bytes', target + '/names/' + str(i), 'name-String'))
            else:
                out[snake(key)] = self.value(v, s, target + '/' + snake(key), key)
        return out

    def layout(self, value, source, target):
        out = {}
        for key, v in value.items():
            if key == 'boundaryCapacities':
                if v is None:
                    out['boundary_capacities_present'] = self.atom(v, source + '/' + key, target + '/boundary_capacities_present', 'absent-boundary-presence', False)
                else:
                    self.gap(v, source + '/' + key, 'boundary capacities payload absent in public observer')
            else:
                out[snake(key)] = self.value(v, source + '/' + key, target + '/' + snake(key), key)
        return out

    def prepare(self, expected):
        out = {'selection': {k: expected[k] for k in ('family', 'case', 'fuel')}, 'observations': [], 'events': [], 'deltas': expected['deltas']}
        for key in ('family', 'case', 'fuel'):
            # CLI selectors are provenance, not semantic observer fields.
            self.atom(expected[key], '/' + key, '/selection/' + key, 'external-selector')
        self.gap(expected['sourceProgram'], '/sourceProgram', 'typed source input absent in HostObservation; independent exact input binding required')
        self.gap(expected['valueObservations'], '/valueObservations', 'materialized value observation carrier absent')
        self.gap(expected['terminalNext'], '/terminalNext', 'terminal next-call capture absent or no expected literal')
        self.gap(expected['eventAnnotations'], '/eventAnnotations', 'oracle source occurrence/link annotations require exact separate transport')
        self.gap(expected['deltaOrder'], '/deltaOrder', 'producer delta-coordinate declaration absent')
        identity = self.value(expected['identity'], '/identity', '/observations/0/admission/identity')
        bounds = self.value(expected['bounds'], '/bounds', '/observations/0/admission/bounds')
        layout = self.layout(expected['layout'], '/layout', '/observations/0/admission/layout')
        program = self.program(expected['programView'], '/programView', '/observations/0/admission/view')
        # Admission is captured once per complete before/post image, never replaced by a digest.
        for i, s in enumerate(expected['states']):
            src, dst = '/states/' + str(i), '/observations/' + str(i)
            binding = s['binding']
            self.gap(binding, src + '/binding', 'symbolic admission reference retained pending Root binding of the eventual producer to the exact owner causal contract; no pointer/token omission is claimed')
            image = {}
            for key, v in s['reg'].items():
                target_key = 'taskLive' if key == 'taskLive' else snake(key)
                image[target_key] = self.value(v, src + '/reg/' + key, dst + '/image/' + target_key, key)
            for key, v in s['img'].items():
                image[snake(key)] = self.value(v, src + '/img/' + key, dst + '/image/' + snake(key), key)
            image['run_id'] = self.value(expected['events'][0]['stepId']['runId'], '/events/0/stepId/runId', dst + '/image/run_id')
            out['observations'].append({'admission': {'identity': identity, 'bounds': bounds, 'layout': layout, 'view': program}, 'image': image})
        for i, event in enumerate(expected['events']):
            src, dst = '/events/' + str(i), '/events/' + str(i)
            # Public writer declares structured summaries; their exact payload carrier must be pinned.
            row = {}
            for key, v in event.items():
                if key in {'helperPhaseBefore', 'helperPhaseAfter'}:
                    q = dst + '/' + snake(key)
                    if v is None:
                        row[snake(key)] = self.atom(v, src + '/' + key, q)
                    else:
                        kind = event['helperKind']
                        wrappers = {'Scalar': 'Scalar', 'EnvGet': 'Tri', 'EnvPut': 'Tri', 'Find': 'Tri', 'Bind': 'Bind', 'Append': 'Append', 'Validate': 'Tri', 'Equal': 'Tri', 'Match': 'Match'}
                        require(kind in wrappers, src + '/' + key, 'phase wrapper needs literal helperKind')
                        row[snake(key)] = {'$variant': wrappers[kind], 'phase': {'$variant': self.atom(v, src + '/' + key, q + '/phase/$variant', 'typed-phase-wrapper')}}
                elif key == 'exhaustedKind':
                    q = dst + '/exhausted_kind'
                    if v is None: row['exhausted_kind'] = self.atom(v, src + '/' + key, q)
                    else:
                        require(type(v) is int and v == 0, src + '/' + key, 'only frozen Fuel ordinal0 is mapped')
                        row['exhausted_kind'] = {'$variant': self.atom(v, src + '/' + key, q + '/$variant', 'Root-normal-JSON-Fuel-ordinal', 'Fuel')}
                elif key == 'outcomePayload':
                    self.gap(v, src + '/' + key, 'inactive payload conventions need exact contextual transport; null/0/object are preserved separately')
                else:
                    row[snake(key)] = self.value(v, src + '/' + key, dst + '/' + snake(key), key)
            out['events'].append(row)
        for i, row in enumerate(expected['deltas']):
            for j, v in enumerate(row):
                self.atom(v, f'/deltas/{i}/{j}', f'/deltas/{i}/{j}')
        return out

    def compare(self, expected, actual):
        tree = self.prepare(expected)
        issues = differences(tree, actual)
        for shape in self.shapes:
            try:
                from normal_compare import ptr
                value = ptr(actual, shape['actualPath'])
                if type(value) is not list or len(value) != shape['expected']:
                    issues.append({'path': shape['actualPath'], 'kind': 'shape_length', 'expected': shape['expected'], 'actual': len(value) if type(value) is list else value})
            except (KeyError, IndexError, TypeError):
                issues.append({'path': shape['actualPath'], 'kind': 'missing_shape_carrier'})
        accounted = {m['expectedPath'] for m in self.mapping} | {m['expectedPath'] for m in self.gaps} | {m['expectedPath'] for m in self.shapes}
        for path, atom in leaves(expected, ''):
            if path not in accounted:
                self.gaps.append({'expectedPath': path, 'expected': atom, 'reason': 'unmapped required expected atom'})
        return {'status': 'GAP' if self.gaps else ('MISMATCH' if issues else 'EXACT_DATA_MATCH'),
                'engineComparison': 'UNOBSERVED_WITHOUT_ROOT_PRODUCER_ORIGIN',
                'differences': issues, 'transportGaps': self.gaps, 'mapping': self.mapping,
                'shapeObligations': self.shapes,
                'bijectionStatus': 'INCOMPLETE; mapped leaves and explicit gaps are reported; immutable envelope values are compared at every observation, not deduplicated',
                'preservation': 'Every observation admission compares the same full expected identity/bounds/layout/view; lists remain distinct ordered arrays'}
