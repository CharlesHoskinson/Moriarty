"""Pure field-only positive DATA interface. No lane encoder or parser/guard policy.

Consumes already-declared positive external records / strict-decoder typed trees.
Does not read files, expectations, model results or execute K. Bounds/admission,
malformed carriers and native widths remain external premises. Every numeric
value is carried without int/float/String conversion.
"""
import copy

CAP_FIELDS = ('max_data_nodes', 'max_data_field_slots', 'max_env_cells',
              'max_value_slots', 'max_constants', 'max_types',
              'max_constructors', 'max_names', 'max_arity')
TABLE_FIELDS = ('data', 'data_fields', 'env', 'value_slots')


def atom(sort, value):
    return {'sort': sort, 'value': value}


def term(sort, constructor, *args):
    return {'sort': sort, 'constructor': constructor, 'arguments': list(args)}


def reference(row):
    name, value = next(iter(row.items()))
    constructor = {'Const': 'ctConst', 'Data': 'ctData'}[name]
    return term('CTRef', constructor, atom('Int', value))


def environment(row):
    name, value = next(iter(row.items()))
    return (term('CTEnv', 'ctEnvNone') if name == 'None' else
            term('CTEnv', 'ctEnvSome', atom('Int', value)))


def slice_value(row):
    return term('CTSlice', 'ctSlice', atom('Int', row['start']), atom('Int', row['len']))


def data_node(row):
    name, value = next(iter(row.items()))
    if name == 'U128':
        return term('DPData', 'dpU128', atom('Int', value['n']))
    if name == 'Bool':
        return term('DPData', 'dpBool', atom('Bool', value['b']))
    if name == 'Nil':
        return term('DPData', 'dpNil')
    if name == 'Constructor':
        return term('DPData', 'dpConstructor', atom('Int', value['type_id']),
                    atom('Int', value['ctor_id']), slice_value(value['fields']))
    return term('DPData', {'Cons': 'dpCons', 'Leaf': 'dpLeaf', 'Branch': 'dpBranch'}[name],
                slice_value(value['fields']))


def env_cell(row):
    return term('DPEnvCell', 'dpEnvCell', atom('Int', row['binder']),
                reference(row['value']), environment(row['previous']))


def linked(sort, empty, cons, values, project):
    result = term(sort, empty)
    for value in reversed(values):
        result = term(sort, cons, project(value), result)
    return result


def tables(row):
    return term('DPTables', 'dpTables',
                linked('DPNodes', 'dpNoNodes', 'dpNode', row['data'], data_node),
                linked('DPRefs', 'dpNoRefs', 'dpRef', row['data_fields'], reference),
                linked('DPEnvs', 'dpNoEnvs', 'dpEnv', row['env'], env_cell),
                linked('DPRefs', 'dpNoRefs', 'dpRef', row['value_slots'], reference))


def port_input(document):
    caps = term('DPCaps', 'dpCaps', *[atom('Int', document['caps'][k]) for k in CAP_FIELDS])
    return {'source_document': copy.deepcopy(document),
            'profile': document['profile'], 'case_id': document['caseId'],
            'query': term('DPObservation', 'dpObserve', caps, tables(document['tables']))}


def reference_document(tree):
    return {{'ctConst': 'Const', 'ctData': 'Data'}[tree['constructor']]: tree['arguments'][0]['value']}


def environment_document(tree):
    return ({'None': None} if tree['constructor'] == 'ctEnvNone' else
            {'Some': tree['arguments'][0]['value']})


def slice_document(tree):
    return {'start': tree['arguments'][0]['value'], 'len': tree['arguments'][1]['value']}


def node_document(tree):
    name, args = tree['constructor'], tree['arguments']
    if name == 'dpU128':
        return {'U128': {'n': args[0]['value']}}
    if name == 'dpBool':
        return {'Bool': {'b': args[0]['value']}}
    if name == 'dpNil':
        return {'Nil': None}
    if name == 'dpConstructor':
        return {'Constructor': {'type_id': args[0]['value'], 'ctor_id': args[1]['value'],
                                'fields': slice_document(args[2])}}
    return {{'dpCons': 'Cons', 'dpLeaf': 'Leaf', 'dpBranch': 'Branch'}[name]:
            {'fields': slice_document(args[0])}}


def env_cell_document(tree):
    args = tree['arguments']
    return {'binder': args[0]['value'], 'value': reference_document(args[1]),
            'previous': environment_document(args[2])}


def unlinked(tree, empty, project):
    result = []
    while tree['constructor'] != empty:
        value, tree = tree['arguments']
        result.append(project(value))
    return result


def tables_document(tree):
    data, fields, env, values = tree['arguments']
    return {'data': unlinked(data, 'dpNoNodes', node_document),
            'data_fields': unlinked(fields, 'dpNoRefs', reference_document),
            'env': unlinked(env, 'dpNoEnvs', env_cell_document),
            'value_slots': unlinked(values, 'dpNoRefs', reference_document)}


def observation_document(tree):
    """Called only on a whole DPObservation validated by the existing strict codec.

    This is field projection, not a substitute decoder/admission gate. Does not
    compute any header, capacity base, padding, encoded lane or expected value.
    """
    geometry, counts, lanes, decoded = tree['arguments']
    names = ('data_base', 'fields_base', 'env_base', 'values_base', 'total')
    return {'geometry': {key: value['value'] for key, value in zip(names, geometry['arguments'])},
            'counts': [value['value'] for value in counts['arguments']],
            'padded_lanes': unlinked(lanes, 'ctNoLanes', lambda value: value['value']),
            'decoded_tables': tables_document(decoded)}
