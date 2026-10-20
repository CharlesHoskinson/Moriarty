"""Positive CTX field-only port. No model, encoder, arithmetic, parser or policy.

Caller supplies declared positive input or a whole strictly decoded typed tree.
All widths/admission remain external. No file/expected/actual reads or execution.
"""
import copy

EXT_FIELDS = ('plan_case_slots', 'field_path_slots', 'pattern_child_slots', 'input_rows', 'asset_rows', 'state_rows', 'account_rows', 'liability_rows', 'await_site_rows', 'await_binder_slots', 'balance_cells', 'journal_rows', 'pending_op_rows', 'observations_per_invocation', 'max_lifetime_work', 'resume_rows')
BUDGET_FIELDS = ('type_expr_nodes', 'template_bytes')
BOUND_FIELDS = ('code_rows', 'function_rows', 'code_child_slots', 'parameter_slots', 'pattern_rows', 'match_plan_rows', 'match_arm_rows', 'type_rows', 'constructor_rows', 'constructor_field_slots', 'max_arity', 'constant_nodes', 'constant_field_slots', 'data_nodes', 'data_field_slots', 'environment_cells', 'frames', 'helper_tasks', 'serialization_bytes', 'max_reference_work', 'max_helper_work', 'step_cap')
LAYOUT_FIELDS = ('name_rows', 'name_bytes', 'occurrence_path_slots', 'value_slots')
LIMIT_FIELDS = ('max_total_lanes', 'max_state_lanes', 'max_metadata_bytes', 'max_string_bytes', 'max_width_entries')
PHYS_FIELDS = ('max_frames', 'max_expressions', 'max_names', 'max_arms', 'max_children', 'max_value_slots', 'max_constants', 'max_data_nodes', 'max_env_cells', 'max_functions', 'max_arity', 'max_types', 'max_occurrences', 'max_occurrence_paths', 'max_parameters', 'max_patterns', 'max_pattern_children', 'max_constructors', 'max_ctor_fields', 'max_decl_templates', 'max_plan_nodes', 'max_plan_cases', 'max_field_paths', 'max_constant_fields', 'max_name_bytes', 'max_type_expr_nodes', 'max_template_bytes')


def atom(sort, value):
    return {'sort': sort, 'value': value}


def term(sort, constructor, *args):
    return {'sort': sort, 'constructor': constructor, 'arguments': list(args)}


def record(sort, constructor, names, value):
    return term(sort, constructor, *[atom('Int', value[name]) for name in names])


def ext(value):
    if value == 'C6CtxAbsent':
        return term('CXExt', 'cxAbsent')
    return term('CXExt', 'cxPresent', record('CXExtFields', 'cxExt', EXT_FIELDS, value['C6CtxPresent']))


def payload(value):
    return term('CXPayload', 'cxPayload', ext(value['ext']),
                record('CXBudgets', 'cxBudgets', BUDGET_FIELDS, value['budgets']))


def input_value(value):
    return term('CXInput', 'cxInput', payload(value['context']),
                record('CXBounds', 'cxBounds', BOUND_FIELDS, value['bounds']),
                record('CXLayoutCaps', 'cxLayoutCaps', LAYOUT_FIELDS, value['capacities']),
                record('CXHostLimits', 'cxHostLimits', LIMIT_FIELDS, value['limits']))


def port_input(document):
    cases = term('CXCases', 'cxNoCases')
    for case in reversed(document['cases']):
        cases = term('CXCases', 'cxCase', atom('String', case['case_id']), input_value(case['input']), cases)
    return {'source_document': copy.deepcopy(document), 'profile': document['profile'],
            'case_ids': [case['case_id'] for case in document['cases']],
            'query': term('CXObservations', 'cxObserveAll', cases)}


def record_document(tree, names):
    return {name: value['value'] for name, value in zip(names, tree['arguments'])}


def ext_document(tree):
    return ('C6CtxAbsent' if tree['constructor'] == 'cxAbsent' else
            {'C6CtxPresent': record_document(tree['arguments'][0], EXT_FIELDS)})


def payload_document(tree):
    e, b = tree['arguments']
    return {'ext': ext_document(e), 'budgets': record_document(b, BUDGET_FIELDS)}


def input_document(tree):
    p, b, c, h = tree['arguments']
    return {'context': payload_document(p), 'bounds': record_document(b, BOUND_FIELDS),
            'capacities': record_document(c, LAYOUT_FIELDS), 'limits': record_document(h, LIMIT_FIELDS)}


def query_cases_document(tree):
    result = []
    cases = tree['arguments'][0]
    while cases['constructor'] != 'cxNoCases':
        name, value, cases = cases['arguments']
        result.append({'case_id': name['value'], 'input': input_document(value)})
    return result


def lane_values(tree):
    result = []
    while tree['constructor'] != 'ctNoLanes':
        value, tree = tree['arguments']
        result.append(value['value'])
    return result


def state_document(tree):
    caps, present, limits = tree['arguments']
    phys, data, helper = caps['arguments']
    return {'caps': {'phys': record_document(phys, PHYS_FIELDS),
                     'max_data_field_slots': data['value'], 'max_helper_tasks': helper['value']},
            'ext_present': present['value'],
            'limits': {'max_total_lanes': limits['arguments'][0]['value']}}


def observation_document(tree):
    encoded, decoded, effective, state, descriptor = tree['arguments']
    return {'encoded_lanes': lane_values(encoded), 'decoded_context': payload_document(decoded),
            'effective_ext': record_document(effective, EXT_FIELDS),
            'state_context': state_document(state), 'state_descriptor': lane_values(descriptor)}


def observations_document(tree, public_source):
    # Common wire literals and case IDs come from frozen public declarations/input,
    # never expected. This projects existing K fields; it does not compute them.
    result = []
    while tree['constructor'] != 'cxNoObservations':
        item, tree = tree['arguments']
        result.append(observation_document(item))
    return {'schema': 'independent-c6-context-positive/1', 'profile': public_source['profile'],
            'case_ids': [case['case_id'] for case in public_source['cases']], 'observations': result}
