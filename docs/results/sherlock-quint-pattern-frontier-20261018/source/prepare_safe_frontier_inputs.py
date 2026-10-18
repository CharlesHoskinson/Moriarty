"""Source-only preparation of immutable input tables/literals. No matching/evaluator.
Reads only SAFE-FRONTIER-INPUTS. Never reads any expected/actual/oracle artifact.
Decoded input values/pattern tokens are representation witnesses, not run results.
"""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE / 'SAFE-FRONTIER-INPUTS.json'
DEST = HERE / 'afterimages/models/sherlock/quint/pattern-frontier'


def qlist(items):
    return 'List(' + ', '.join(items) + ')'


def qmap(items):
    return 'Map(' + ', '.join(k + ' -> ' + v for k, v in items) + ')'


def qpath(path):
    return qlist([str(i) for i in path])


def qtoken(token, pattern=False):
    kind, payload = next(iter(token.items()))
    if kind == 'UInt':
        return f'UIntToken({payload})'
    if kind == 'Bool':
        return 'BoolToken(' + str(payload).lower() + ')'
    if kind == 'Wildcard':
        return 'PatternWildcard'
    if kind == 'Bind':
        return f'PatternBind({payload})'
    if kind == 'Constructor':
        prefix = 'PatternConstructor' if pattern else 'ConstructorToken'
        return prefix + '({typeId: ' + str(payload['typeId']) + ', ctorId: ' + str(payload['ctorId']) + ', arity: ' + str(payload['arity']) + '})'
    raise ValueError(kind)


def qslice(segment):
    return '{start: ' + str(segment['start']) + ', len: ' + str(segment['len']) + '}'


def prepare(case):
    nodes = {node['nodeId']: node for node in case['valueGraph']['nodes']}
    value_rows, value_fields, decoded = {}, [], {}
    def decode(node_id, active=()):
        if node_id in active:
            raise ValueError('Source input must be finite, acyclic value graph')
        node = nodes[node_id]
        if node['kind'] in ('UInt', 'Bool'):
            return [{node['kind']: node['value']}]
        token = {'Constructor': {'typeId': node['typeId'], 'ctorId': node['ctorId'], 'arity': len(node['fieldRefs'])}}
        return [token] + [t for child in node['fieldRefs'] for t in decode(child, active + (node_id,))]
    for node_id, node in nodes.items():
        refs = node.get('fieldRefs', [])
        tokens = decode(node_id)
        value_rows[node_id] = {'token': tokens[0], 'children': {'start': len(value_fields), 'len': len(refs)}}
        value_fields.extend(refs)
        decoded[node_id] = tokens
    pattern_rows, pattern_fields, encoded = {}, [], {}
    source_paths, binder_origins, arms, origin_rows = [], [], [], []
    def pattern(source_pattern, path, arm_index, paths, binders):
        row_id = len(pattern_rows)
        pattern_rows[row_id] = None  # reserve parent independently of child numbering
        paths.append(path)
        origin_rows.append({'armIndex': arm_index, 'sourceArmId': case['arms'][arm_index]['sourceArmId'],
                            'path': path, 'patternRow': row_id, 'sourcePattern': source_pattern})
        if 'Constructor' in source_pattern:
            ctor = source_pattern['Constructor']
            child_ids = [pattern(child, path + [position], arm_index, paths, binders)
                         for position, child in enumerate(ctor['fields'])]
            token = {'Constructor': {'typeId': ctor['typeId'], 'ctorId': ctor['ctorId'], 'arity': len(child_ids)}}
        else:
            child_ids = []
            token = source_pattern
            if 'Bind' in source_pattern:
                binders.append({'name': source_pattern['Bind'], 'path': path})
        segment = {'start': len(pattern_fields), 'len': len(child_ids)}
        pattern_fields.extend(child_ids)
        pattern_rows[row_id] = {'token': token, 'children': segment}
        encoded[row_id] = [token] + [t for child in child_ids for t in encoded[child]]
        return row_id
    for arm_index, arm in enumerate(case['arms']):
        paths, binders = [], []
        root = pattern(arm['pattern'], [], arm_index, paths, binders)
        source_paths.append(paths)
        binder_origins.append(binders)
        arms.append({'sourceArmId': arm['sourceArmId'], 'bodyExprId': arm['bodyExprId'], 'patternRoot': root})
    layouts = [{'typeId': decl['typeId'], 'ctorId': ctor['ctorId'], 'fields': ctor['fieldTypes']}
               for decl in case['declarations'] for ctor in decl['constructors']]
    return {'caseId': case['caseId'], 'sourceMatchId': case['sourceMatchId'], 'arms': arms,
            'valueRoot': case['valueGraph']['root'], 'valueRows': value_rows, 'valueFields': value_fields,
            'patternRows': pattern_rows, 'patternFields': pattern_fields, 'decodedValues': decoded,
            'encodedPatterns': encoded, 'sourcePaths': source_paths, 'binderOrigins': binder_origins,
            'savedEnvironment': case['savedEnvironment'], 'layouts': layouts, 'analysisBounds': case['analysisBounds'],
            'patternSourceRows': origin_rows}


def quint_input(table):
    entries = [
        'sourceMatchId: ' + str(table['sourceMatchId']),
        'arms: ' + qlist(['{sourceArmId: '+str(a['sourceArmId'])+', bodyExprId: '+str(a['bodyExprId'])+', patternRoot: '+str(a['patternRoot'])+'}' for a in table['arms']]),
        'valueRoot: '+str(table['valueRoot']),
    ]
    for key, pat in [('valueRows', False), ('patternRows', True)]:
        entries.append(key+': '+qmap([(str(k), '{token: '+qtoken(v['token'], pat)+', children: '+qslice(v['children'])+'}') for k,v in table[key].items()]))
    for key in ['valueFields', 'patternFields']:
        entries.append(key+': '+qlist([str(i) for i in table[key]]))
    for key, pat in [('decodedValues', False), ('encodedPatterns', True)]:
        entries.append(key+': '+qmap([(str(k), qlist([qtoken(t, pat) for t in v])) for k,v in table[key].items()]))
    entries.append('sourcePaths: '+qlist([qlist([qpath(p) for p in paths]) for paths in table['sourcePaths']]))
    entries.append('binderOrigins: '+qlist([qlist(['{name: '+str(b['name'])+', path: '+qpath(b['path'])+'}' for b in arm]) for arm in table['binderOrigins']]))
    entries.append('savedEnvironment: '+qmap([(str(b['name']), qlist([qtoken(b['value'])])) for b in table['savedEnvironment']]))
    entries.append('layouts: '+qmap([('('+str(l['typeId'])+', '+str(l['ctorId'])+')', '{typeId: '+str(l['typeId'])+', ctorId: '+str(l['ctorId'])+', fields: '+qlist([str(t) for t in l['fields']])+'}') for l in table['layouts']]))
    entries.append('analysisBounds: {advances: '+str(table['analysisBounds']['advances'])+', maxTasks: '+str(table['analysisBounds']['maxTasks'])+', maxPath: '+str(table['analysisBounds']['maxPath'])+'}')
    return '{\n    '+',\n    '.join(entries)+'\n  }'


def main():
    source = json.loads(SOURCE.read_text())
    tables = [prepare(case) for case in source['cases']]
    (HERE/'SAFE-SOURCE-TO-TABLE-ALL-ROWS.json').write_text(json.dumps({'status': 'INPUT_TABLES_AND_LITERAL_REPRESENTATION_WITNESSES_ONLY_NO_MATCH_RESULTS_OR_WORK', 'profile': source['profile'], 'cases': tables}, indent=2)+'\n')
    text = 'module patternFrontierInputs {\n  import nominalConstruction.* from "./nominalConstruction"\n  import patternFrontier as F from "./patternFrontier"\n'
    for i, table in enumerate(tables):
        text += '  // '+table['caseId']+'\n  pure val frontierInput'+str(i)+': F::FrontierInput = '+quint_input(table)+'\n'
    text += '  pure val frontierCohort: List[F::FrontierInput] = List('+', '.join('frontierInput'+str(i) for i in range(len(tables)))+')\n}\n'
    (DEST/'patternFrontierInputs.qnt').write_text(text)

if __name__ == '__main__':
    main()
