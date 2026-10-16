"""Private normal host diagnostic projection; no evaluator or execution launcher.

Expected records expand only the independent corpus's explicit prefix overlay
contract. Supplied producer/source identity remains an unauthenticated declaration.
"""

PRODUCER = 'sherlock-host-reference'


def literal_snapshot(program, trace, state, fuel, *, published_output=None):
    """Reconstruct one complete literal state, without evaluating any rule."""
    return dict(code_k=program['code_k'], entry_k=program['entry_k'],
                control_k='.K' if published_output is not None else state['control_k'],
                environment_k=state['environment_k'],
                frames_top_first_k=list(state['frames_top_first_k']), depth=state['depth'],
                stack_limit=trace['fixed_state_fields']['stack_limit'],
                remaining_fuel=fuel - state['charged_steps'], initial_fuel=fuel,
                output_k=published_output if published_output is not None else trace['fixed_state_fields']['output_k'],
                charged_steps=state['charged_steps'])


def expected_observations(families, *, producer=PRODUCER):
    """Expand validated frozen inputs/prefix literals into full observation rows."""
    identities, runs = [], []
    for programs, prefixes in families:
        for program, trace in zip(programs['programs'], prefixes['traces'], strict=True):
            identities.append({key: program[key] for key in ('case_id', 'code_k', 'entry_k')})
            for fuel, boundary, remaining, output, rule in trace['ordinary_fuel_prefixes']:
                snapshots = [literal_snapshot(program, trace, state, fuel)
                             for state in trace['states'][:boundary + 1]]
                records = [dict(rule_start_line=trace['states'][index]['via_rule_start_line'],
                                charge=1, before=snapshots[index - 1], after=snapshots[index])
                           for index in range(1, boundary + 1)]
                terminal = literal_snapshot(program, trace, trace['states'][boundary], fuel,
                                            published_output=output)
                if terminal['remaining_fuel'] != remaining:
                    raise ValueError('literal prefix remaining fuel inconsistent with state overlay')
                records.append(dict(rule_start_line=rule, charge=0, before=snapshots[-1], after=terminal))
                runs.append(dict(case_id=program['case_id'], initial=snapshots[0], records=records,
                                 terminal=terminal, terminal_has_next=False))
    first = families[0][0]
    return dict(producer=producer, profile=first['profile'], source_sha256=first['source']['sha256'],
                boundary='post-startup', programs=identities, runs=runs)


def compare_observations(expected, observed, first_difference):
    """Compare supplied diagnostics only; an artifact is not execution evidence."""
    if observed is None:
        raise ValueError('absent host observation artifact')
    found = first_difference(expected, observed)
    if found is None:
        return None
    parts = found['path'].split('/')
    if len(parts) > 2 and parts[1] == 'runs' and parts[2].isdecimal():
        index = int(parts[2])
        left = expected['runs'][index] if index < len(expected['runs']) else None
        rows = observed.get('runs') if type(observed) is dict else None
        right = rows[index] if type(rows) is list and index < len(rows) else None
        found.update(run_index=index, case_id=left.get('case_id') if left else None,
                     initial_fuel=left['initial']['initial_fuel'] if left else None)
        if len(parts) > 4 and parts[3] == 'records' and parts[4].isdecimal():
            record_index = int(parts[4])
            left_record = left['records'][record_index] if left and record_index < len(left['records']) else None
            right_rows = right.get('records') if type(right) is dict else None
            right_record = right_rows[record_index] if type(right_rows) is list and record_index < len(right_rows) else None
            found.update(record_index=record_index,
                         rule_start_line=left_record.get('rule_start_line') if left_record else None,
                         expected_record=left_record, observed_record=right_record)
            if len(parts) > 5 and parts[5] in ('before', 'after'):
                field = parts[5]
                found.update(expected_state=left_record.get(field) if left_record else None,
                             observed_state=right_record.get(field) if type(right_record) is dict else None)
        elif len(parts) > 3 and parts[3] in ('initial', 'terminal'):
            field = parts[3]
            found.update(expected_state=left.get(field) if left else None,
                         observed_state=right.get(field) if type(right) is dict else None)
    return found
