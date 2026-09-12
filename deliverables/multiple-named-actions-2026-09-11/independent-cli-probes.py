from pathlib import Path
import copy, json, subprocess, sys, tempfile

root = Path(sys.argv[1])
package = root / 'experiments/moriarty-language'
baseline = Path('/home/charl/Moriarty/.worktrees/source-defined-repayment/experiments/moriarty-language')
examples = baseline / 'spec/successor/examples'
profile = 'moriarty-financial-agreement-source/2'
single_profile = 'moriarty-financial-agreement-source/1'
original = (examples / 'source-defined-payment.mori').read_text()
prefix = original.split('  action pay(')[0]
body = '''
    requires pre.due > 0;
    requires is_negative(nominal) == false;
    let payment = magnitude(nominal);
    requires payment > 0;
    next.paid = pre.paid + payment;
    emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };
    emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };
    ensures post.paid == pre.paid + payment;
'''
repay = '  action repay(nominal: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text) {' + body + '  }\n'
installment = '  action repay_installment(transferId: Text, allocationId: Text) {\n    let nominal = quantity<Units<Cash,1>,0>(20);\n' + body + '  }\n'
source = prefix.replace(single_profile, profile) + repay + installment + '}\n'
base_snapshot = json.loads((examples / 'expression-funded-payment.snapshots.json').read_text())
base_snapshot['Args'] = {'nominal': '30', 'transferId': 'T1', 'allocationId': 'Alloc1'}
base_state = json.loads((examples / 'expression-funded-payment.state.json').read_text())
results = []

with tempfile.TemporaryDirectory(prefix='moriarty-multi-cli-') as td:
    temp = Path(td)

    def write(name, value):
        p = temp / name
        p.write_text(value if isinstance(value, str) else json.dumps(value, sort_keys=True, separators=(',', ':')))
        return str(p)

    def call(command, text=source, action='repay', snapshot=None, state=None, old=False, extra=None):
        pkg = baseline if old else package
        args = ['node', str(pkg / 'src/cli.ts'), command, '--profile', single_profile if old else profile]
        if command == 'simulate':
            if not old and action is not None:
                args += ['--action', action]
            args += ['--snapshots', write('snapshot.json', snapshot if snapshot is not None else base_snapshot),
                     '--repayment-state', write('state.json', state if state is not None else base_state)]
        args += extra or []
        args += [write('input.mori', text)]
        return subprocess.run(args, capture_output=True, text=True, timeout=15)

    def accepted(response, name):
        assert response.returncode == 0, (name, response.returncode, response.stdout, response.stderr)
        assert response.stderr == '', (name, response.stderr)
        return json.loads(response.stdout)

    def rejected(response, name, token=None, text=source, status=1):
        assert response.returncode == status and response.stdout == '', (name, response.returncode, response.stdout, response.stderr)
        value = json.loads(response.stderr)
        assert not any(k in value for k in ['post', 'financialPost', 'effects', 'descriptors']), (name, value)
        if token is not None:
            span = value.get('span', {})
            assert span.get('kind') == 'source', (name, value)
            location = text.encode()[int(span['start']):int(span['end'])].decode()
            assert token in location, (name, token, location)
        results.append({'case': name, 'code': value.get('code')})
        return value

    checked = accepted(call('check'), 'check all actions')
    assert [a['action'] for a in checked['actions']] == ['repay', 'repay_installment']
    results.append({'case': 'all actions check'})
    fmt = call('format')
    assert fmt.returncode == 0, fmt.stderr
    twice = call('format', fmt.stdout)
    assert twice.returncode == 0 and fmt.stdout == twice.stdout
    results.append({'case': 'idempotent format'})

    first = accepted(call('simulate'), 'first payment')
    first_old = accepted(call('simulate', prefix + repay + '}\n', old=True), 'isolated first payment')
    assert first['action'] == 'repay' and first['result'] == first_old['result']
    result = first['result']
    assert result['financialPost']['obligations'][0]['principal'] == '70'
    assert result['post']['paid'] == '30'
    formatted = accepted(call('simulate', fmt.stdout), 'formatted simulation')
    assert formatted['result'] == result
    results.append({'case': 'first complete result equals reviewed isolated action', 'result': result})

    second_snapshot = {'Pre': result['post'], 'Args': {'transferId': 'T2', 'allocationId': 'Alloc2'}, 'Obs': {}, 'workInitial': result['workRemaining']}
    second = accepted(call('simulate', action='repay_installment', snapshot=second_snapshot, state=result['financialPost']), 'second payment')
    second_old = accepted(call('simulate', prefix + installment + '}\n', snapshot=second_snapshot, state=result['financialPost'], old=True), 'isolated installment')
    assert second['result'] == second_old['result']
    final = second['result']
    assert final['post']['paid'] == '50'
    assert final['financialPost']['obligations'][0]['principal'] == '50'
    assert final['financialPost']['obligations'][0]['outstanding'] == '50'
    assert [b['amount'] for b in final['financialPost']['balances']] == ['50', '50']
    assert final['financialPost']['allowances'][0]['remaining'] == '50'
    assert final['financialPost']['allowances'][0]['spent'] == '50'
    assert final['financialPost']['usedTransferIds'] == ['T1', 'T2']
    assert final['financialPost']['usedAllocationIds'] == ['Alloc1', 'Alloc2']
    assert final['financialPost']['work']['closureReserve'] == base_state['work']['closureReserve']
    results.append({'case': 'continued installment complete result equals reviewed isolated action', 'result': final})

    reversed_source = prefix.replace(single_profile, profile) + installment + repay + '}\n'
    reversed_result = accepted(call('simulate', reversed_source), 'reordered actions')
    assert reversed_result['result'] == result
    results.append({'case': 'declaration order does not change named runtime result or work'})
    extra_action = source[:-2] + 'action never() { requires false; }\n}\n'
    extra_result = accepted(call('simulate', extra_action), 'unselected runtime guard')
    assert extra_result['result'] == result
    results.append({'case': 'unselected runtime guard and work isolation'})

    for selector in ['missing', 'Repay', 'constructor', 'toString', '__proto__']:
        rejected(call('simulate', action=selector), 'bad selector ' + repr(selector))
    rejected(call('simulate', action=None), 'missing CLI selector', status=2)
    rejected(call('check', extra=['--action', 'repay']), 'selector forbidden on check', status=2)
    bad_args = copy.deepcopy(base_snapshot)
    bad_args['Args']['otherActionArg'] = '1'
    rejected(call('simulate', snapshot=bad_args), 'extra selected Args')
    bad_args['Args'] = {'transferId': 'T1', 'allocationId': 'Alloc1'}
    rejected(call('simulate', snapshot=bad_args), 'missing selected Args')
    bad_args['Args']['nominal'] = True
    rejected(call('simulate', snapshot=bad_args), 'wrong selected argument type')

    duplicate = source.replace('action repay_installment(', 'action repay(')
    rejected(call('check', duplicate), 'duplicate action', 'action repay(', duplicate)
    no_actions = prefix.replace(single_profile, profile) + '}\n'
    rejected(call('check', no_actions), 'zero actions')
    cross_scope = '// UTF-8 λ🙂\n' + source.replace('let nominal = quantity<Units<Cash,1>,0>(20);', 'let nominal = secret;')
    rejected(call('simulate', cross_scope, snapshot='{}', state='{}'), 'unselected static error before runtime inputs', 'secret', cross_scope)
    for borrowed in ['nominal', 'payment']:
        leaked = source.replace('let nominal = quantity<Units<Cash,1>,0>(20);',
                                'let borrowed = ' + borrowed + ';\n    let nominal = quantity<Units<Cash,1>,0>(20);')
        rejected(call('check', leaked), 'another action binding is unavailable: ' + borrowed, borrowed, leaked)
    per_action = source[:-2] + 'action typed(nominal: Text) { requires nominal == "ok"; }\n}\n'
    accepted(call('check', per_action), 'same parameter name with different type')
    results.append({'case': 'per-action parameter types are independent'})
    bad_state = copy.deepcopy(base_state)
    bad_state['allowances'][0]['remaining'] = '29'
    rejected(call('simulate', state=bad_state), 'funding failure has no tentative output')

print(json.dumps({'passed': len(results), 'results': results}, indent=2))
