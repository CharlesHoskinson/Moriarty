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
    if (contract.get('schemaVersion') != 'moriarty-expression-contract/0'
            or contract.get('status') != 'proposed-not-registered'
            or any(contract.get(k) is not False for k in
                   ['fullRp01Complete', 'runtimeImplemented', 'proofsEstablished'])):
        errors.append('claim-scope')
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
            if row.get('caseIds') != [name + '-positive', name + '-reject'] or set(row.get('caseIds', [])) != {v['id'] for v in pair}:
                errors.append('case-reference:' + name)
    return errors


if __name__ == '__main__':
    here = Path(__file__).resolve().parent
    contract = json.loads((here / 'expression-signatures.json').read_text())
    cases = json.loads((here / 'expression-cases.json').read_text())
    errors = validate(contract, cases)
    print(json.dumps({'scope': 'document-structure-only', 'errors': errors,
                      'constructors': len(contract['constructors']),
                      'casePairs': len(cases['cases']) // 2}, indent=2))
    raise SystemExit(bool(errors))
