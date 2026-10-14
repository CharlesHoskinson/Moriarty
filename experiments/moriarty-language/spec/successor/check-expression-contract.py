"""Check completeness/references of the proposed documents. Never evaluate Core."""
import json
from pathlib import Path

EXPECTED = frozenset('''LitUInt LitSInt LitBool LitText LitAmount LitQuantity
LitShares LitRate LitPrice ReadLocal ReadPre ReadArg ReadObs ProjectField
ProjectIndex AccessField AccessIndex ConstructRecord ConstructEnum ConstructSome
ConstructNone ConstructCollection Add Sub Mul FloorDiv CeilDiv Eq Lt Lte Gt Gte
Not And Or Require Let NextWrite Ensure Emit'''.split())


def validate(contract, cases):
    errors = []
    if not isinstance(contract, dict) or not isinstance(cases, dict):
        return ['document-shape']
    if (contract.get('schemaVersion') != 'moriarty-expression-contract/1'
            or contract.get('status') != 'proposed-not-registered'
            or any(contract.get(k) is not False for k in
                   ['fullRp01Complete', 'runtimeImplemented', 'proofsEstablished'])):
        errors.append('claim-scope')
    if cases.get('schemaVersion') != 'moriarty-expression-cases/1' or cases.get('contract') != 'moriarty-expression-contract/1':
        errors.append('case-version')
    rows = contract.get('constructors')
    values = cases.get('cases')
    if not isinstance(rows, list) or not isinstance(values, list):
        return errors + ['document-shape']
    if not all(isinstance(r, dict) for r in rows + values):
        return errors + ['row-shape']
    names = [r.get('constructor') for r in rows]
    if not all(isinstance(n, str) for n in names):
        return errors + ['constructor-shape']
    if set(names) != EXPECTED:
        errors.append('constructor-set')
    if len(names) != len(set(names)):
        errors.append('duplicate-constructor')
    types = contract.get('types', {})
    rules = contract.get('rules', [])
    sources = contract.get('sources', {})
    if not isinstance(types, dict) or not isinstance(rules, list) or not isinstance(sources, dict):
        return errors + ['registry-shape']
    for row in rows:
        name = row['constructor']
        operands = row.get('operands')
        evaluated = row.get('evaluatedOperands')
        if (not isinstance(operands, list) or not isinstance(evaluated, list)
                or not all(isinstance(o, dict) and isinstance(o.get('name'), str)
                           and isinstance(o.get('type'), str) for o in operands)
                or not all(isinstance(e, str) for e in evaluated)):
            errors.append('operand-shape:' + name)
            continue
        op_names = [o['name'] for o in operands]
        if len(op_names) != len(set(op_names)) or len(evaluated) != len(set(evaluated)):
            errors.append('duplicate-operand:' + name)
        conditional = row.get('conditionalEvaluatedOperands', [])
        if name in {'And', 'Or'}:
            expected_condition = 'left=true' if name == 'And' else 'left=false'
            if (evaluated != ['left']
                    or conditional != [{'name': 'right', 'when': expected_condition}]
                    or row.get('staticOperands') != ['left', 'right']
                    or row.get('evaluationOrder') != 'left-then-selected-right'):
                errors.append('boolean-evaluation-metadata:' + name)
        elif conditional:
            errors.append('unexpected-conditional-operand:' + name)
        if any(n not in op_names for n in evaluated):
            errors.append('dangling-operand:' + name)
        result = row.get('result')
        if not isinstance(result, str) or result not in types or any(o['type'] not in types for o in operands):
            errors.append('dangling-type:' + name)
        if row.get('rule') not in rules:
            errors.append('dangling-rule:' + name)
        refs = row.get('sources')
        if not isinstance(refs, list) or not refs or any(not isinstance(r, str) or r not in sources for r in refs):
            errors.append('dangling-source:' + name)
        if any(not isinstance(row.get(k), str) or not row[k].strip()
               for k in ['typing', 'reduction', 'frame']):
            errors.append('empty-contract:' + name)
        if type(row.get('entryWork')) is not int or row['entryWork'] != 1:
            errors.append('work-charge:' + name)
        if row.get('decisionStatus') != 'proposed':
            errors.append('decision-scope:' + name)
    identities = [v.get('id') for v in values]
    if not all(isinstance(i, str) for i in identities):
        return errors + ['case-id-shape']
    if len(identities) != len(set(identities)):
        errors.append('duplicate-case')
    for v in values:
        if v.get('constructor') not in EXPECTED:
            errors.append('unknown-case-constructor')
        if (v.get('status') != 'author-derived-specified-only'
                or not isinstance(v.get('expected'), dict)
                or any(not isinstance(v.get(k), str) or not v[k].strip()
                       for k in ['term', 'context', 'derivation'])):
            errors.append('case-shape:' + v['id'])
    for name in sorted(EXPECTED):
        pair = [v for v in values if v.get('constructor') == name]
        if len(pair) != 2 or {v.get('kind') for v in pair} != {'positive', 'rejection'}:
            errors.append('case-pair:' + name)
        else:
            row = next((r for r in rows if r['constructor'] == name), {})
            positive_id = name + '-short-circuit-positive-v1' if name in {'And', 'Or'} else name + '-positive'
            if row.get('caseIds') != [positive_id, name + '-reject'] or set(row.get('caseIds', [])) != {v['id'] for v in pair}:
                errors.append('case-reference:' + name)
    return errors


def validate_booleans(contract, supplement):
    """Check fixture/reference structure and specified accounting, never reduce Core."""
    errors = []
    if (not isinstance(supplement, dict)
            or supplement.get('schemaVersion') != 'moriarty-boolean-cases/1'
            or supplement.get('contract') != 'moriarty-expression-contract/1'
            or supplement.get('status') != 'author-derived-specified-only'):
        return ['boolean-document-scope']
    rows = supplement.get('cases', [])
    profiles = supplement.get('inputProfiles', {})
    if not isinstance(rows, list) or not rows or not isinstance(profiles, dict):
        return ['boolean-document-shape']
    signatures = {r['constructor']: r for r in contract['constructors']}
    ids, proposals, truths = set(), set(), set()

    def occurrences(node, path, result):
        result[tuple(path)] = node
        if 'statements' in node:
            if set(node) != {'statements', 'span'}:
                raise ValueError('action-shape')
            kids = node['statements']
        else:
            if set(node) != {'constructor', 'operands', 'span'}:
                raise ValueError('node-shape')
            sig = signatures[node['constructor']]
            ops = node['operands']
            if set(ops) != {o['name'] for o in sig['operands']}:
                raise ValueError('operand-keys')
            kids = []
            for operand in sig['operands']:
                value = ops[operand['name']]
                if operand['type'] == 'Expression':
                    kids.append(value)
                elif operand['type'] == 'ExpressionList':
                    kids.extend(value)
                elif operand['type'] == 'NamedExpressionList':
                    kids.extend(item['value'] for item in value)
        for i, child in enumerate(kids):
            occurrences(child, path + [i], result)

    for row in rows:
        name = row.get('id', '')
        try:
            if (not name.startswith('BC1-') or name in ids
                    or row.get('status') != 'author-derived-specified-only'
                    or row.get('contract') != 'moriarty-expression-contract/1'):
                raise ValueError('identity-scope')
            ids.add(name)
            if row['inputProfile'] not in profiles:
                raise ValueError('input-profile')
            profile = profiles[row['inputProfile']]
            if set(profile) != {'schema', 'Pre', 'Args', 'Obs'}:
                raise ValueError('input-profile-shape')
            if not isinstance(row['source'], str) or not row['derivation']:
                raise ValueError('source-derivation')
            nodes = {}
            occurrences(row['core'], [], nodes)
            for node in nodes.values():
                span = node['span']
                if (set(span) != {'kind', 'start', 'end'}
                        or span['kind'] not in {'source', 'synthetic'}
                        or any(not isinstance(span[k], str) or not span[k].isdigit()
                               for k in ['start', 'end'])):
                    raise ValueError('span-shape')
                invalid = (int(span['start']) > int(span['end'])
                           or (span['kind'] == 'synthetic' and (span['start'], span['end']) != ('0', '0'))
                           or (span['kind'] == 'source' and int(span['end']) > len(row['source'].encode('utf-8'))))
                if invalid and row['expected'].get('code') != 'INPUT_SPAN':
                    raise ValueError('unexpected-invalid-span')
            initial = int(row['workInitial'])
            used = int(row['derivationAccounting']['workUsed'])
            remaining = int(row['derivationAccounting']['workRemaining'])
            entered = [tuple(path) for path in row['enteredNodePaths']]
            if (not 0 <= used <= initial <= 65536 or remaining != initial - used
                    or len(entered) != used or len(set(entered)) != len(entered)
                    or any(path not in nodes or 'constructor' not in nodes[path] for path in entered)):
                raise ValueError('specified-entry-accounting')
            expected = row['expected']
            if expected.get('status') == 'Rejected':
                path = tuple(int(i) for i in expected['nodePath'])
                if expected.get('workUsed') != str(used) or path not in nodes:
                    raise ValueError('rejection-accounting-path')
                if row['rejectionPhase'] == 'snapshot':
                    if expected['span'] != {'kind': 'synthetic', 'start': '0', 'end': '0'} or path:
                        raise ValueError('snapshot-fallback')
                elif expected['span'] != nodes[path]['span']:
                    raise ValueError('rejection-span-provenance')
                if row['rejectionPhase'] != 'evaluation' and used != 0:
                    raise ValueError('admission-work')
                if expected['code'] == 'WORK_EXHAUSTED' and (remaining != 0 or path in entered):
                    raise ValueError('work-exhausted-entry')
            elif expected.get('workRemaining') != str(remaining):
                raise ValueError('success-accounting')
            if 'truth-table' in row['coverage']:
                n = row['core']
                truths.add((n['constructor'], n['operands']['left']['operands']['value'],
                            n['operands']['right']['operands']['value']))
            proposals.update(row['proposalCaseIds'])
        except (KeyError, TypeError, ValueError, AttributeError) as exc:
            errors.append('boolean-case:' + str(name) + ':' + str(exc))
    if truths != {(op, left, right) for op in ['And', 'Or'] for left in [False, True] for right in [False, True]}:
        errors.append('boolean-truth-coverage')
    if proposals != {'BOOL%02d' % i for i in range(1, 15)}:
        errors.append('boolean-proposal-coverage')
    return errors


if __name__ == '__main__':
    here = Path(__file__).resolve().parent
    contract = json.loads((here / 'expression-signatures.json').read_text())
    cases = json.loads((here / 'expression-cases.json').read_text())
    booleans = json.loads((here / 'boolean-cases.json').read_text())
    errors = validate(contract, cases) + validate_booleans(contract, booleans)
    print(json.dumps({'scope': 'document-structure-only', 'errors': errors,
                      'constructors': len(contract['constructors']),
                      'casePairs': len(cases['cases']) // 2,
                      'specifiedBooleanCases': len(booleans['cases']),
                      'runtimeExecuted': False, 'semanticExpectedResultsValidated': False}, indent=2))
    raise SystemExit(bool(errors))
