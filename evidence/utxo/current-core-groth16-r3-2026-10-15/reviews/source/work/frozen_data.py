"""Reviewer data-only checks over the frozen packet (no candidate/Core code).

1. Recompute every frozen accounting field of every accepted vector from the frozen
   pre/post/effects records with my own arithmetic and report contradictions.
2. Inventory: multi-asset, self-transfer, self-fee, capped, presence/present-zero/residual cases.
3. Rejected vectors: frozen phase vs (step is null), work_attempted presence.
4. Predict r3 table row counts from the harness's loop structure (my derivation).
5. HW-TRACEABILITY-2 refs against the row ids the harness can print (static derivation).
Writes work/FROZEN-DATA-CHECK.json (exclusive create)."""
import json

B = '/home/charl/moriarty-eutxo-validation-2026-10-12/'
SR = B + 'current-core-groth16-compiler-independent-source-review-20261015/'
A = B + 'current-core-groth16-evidence-repair-resume-source-20261015/'
OUT = B + 'current-core-groth16-evidence-repair-independent-source-review-20261015/work/FROZEN-DATA-CHECK.json'

V = json.load(open(SR + 'heldout/HELDOUT-VECTORS.json'))
H = json.load(open(SR + 'HOSTILE-CLAIMS-ADDENDUM.json'))
A2 = json.load(open(SR + 'heldout-addendum-2/HELDOUT-ADDENDUM-2.json'))
TR = json.load(open(A + 'review-overlay/heldout-addendum-4/HW-TRACEABILITY-2.json'))
PRED = json.load(open(A + 'records/PREDICTED-ROWS.json'))


def key(asset):
    return (asset['network'], asset['issuer'], asset['reference'], asset['scale'])


res = {'accounting_contradictions': {}, 'inventory': {}, 'rejected': [], 'counts': {}}
inv = {'multi_asset': [], 'self_transfer': [], 'self_fee': [], 'fee': [], 'capped': [],
       'present_zero_after': [], 'residual_positive_nonempty': [], 'duty_absent_before_present_after': [],
       'unregistered_or_extra_duty_rows': [], 'frame_rows_zero_gross': []}
vectors = V['vectors']
accepted = [v for v in vectors if v['expected']['accepted']]
rejected = [v for v in vectors if not v['expected']['accepted']]
for v in accepted:
    ex = v['expected']
    acc = ex['accounting']
    pre = {(key(b['asset']), b['account']): int(b['amount']) for b in ex['pre']['balances']}
    post = {(key(b['asset']), b['account']): int(b['amount']) for b in ex['post']['balances']}
    eff = ex['effects']
    diffs = []
    assets = set()
    for row in acc['balances']:
        k = (key(row['asset']), row['account'])
        assets.add(k[0])
        g = gx = f = 0
        for e in eff:
            if e['kind'] in ('Transfer', 'Fee') and (key(e['asset']), e['from']) == k:
                a = int(e['amount'])
                g += a
                if e['to'] != e['from']:
                    gx += a
                if e['kind'] == 'Fee':
                    f += a
        mine = {'pre': pre.get(k), 'post': post.get(k), 'gross_debit': g, 'gross_debit_excluding_self': gx,
                'fees_paid': f}
        mine['net_delta'] = None if pre.get(k) is None else post[k] - pre[k]
        for fld, val in mine.items():
            fz = row[fld]
            if val is None or int(fz) != val:
                diffs.append(f"{row['account']}/{k[0][2]} {fld}: frozen {fz} mine {val}")
        if g == 0 and f == 0:
            inv['frame_rows_zero_gross'].append(v['id'] + ':' + row['account'] + '/' + k[0][2])
    if set(pre) != {(key(r['asset']), r['account']) for r in acc['balances']}:
        diffs.append('balance row set differs from pre snapshot rows')
    preL = {l['name']: int(l['amount']) for l in ex['pre']['liabilities']}
    postL = {l['name']: int(l['amount']) for l in ex['post']['liabilities']}
    for row in acc['duties']:
        n = row['name']
        acc_, dis = 0, 0
        for e in eff:
            if e['kind'] == 'Liability' and e['name'] == n:
                b_, a_ = int(e['before']), int(e['after'])
                if a_ >= b_:
                    acc_ += a_ - b_
                else:
                    dis += b_ - a_
        mine = {'opening': preL.get(n, 0), 'closing': postL.get(n, 0), 'accrued': acc_, 'discharged': dis,
                'present_before': n in preL, 'present_after': n in postL,
                'present_zero_after': postL.get(n) == 0}
        for fld, val in mine.items():
            if row[fld] != val and not (isinstance(val, int) and not isinstance(val, bool) and int(row[fld]) == val):
                diffs.append(f'duty {n} {fld}: frozen {row[fld]} mine {val}')
        # liability evolution identity
        if mine['opening'] + acc_ - dis != mine['closing']:
            diffs.append(f'duty {n}: opening + accrued - discharged != closing')
        if mine['present_zero_after']:
            inv['present_zero_after'].append(v['id'] + ':' + n)
        if not mine['present_before'] and mine['present_after']:
            inv['duty_absent_before_present_after'].append(v['id'] + ':' + n)
    if set(preL) | set(postL) != {r['name'] for r in acc['duties']}:
        diffs.append('duty row set differs from pre/post liabilities')
    rp = sorted(n for n, a in postL.items() if a > 0)
    if sorted(acc['residual_positive']) != rp:
        diffs.append(f"residual_positive frozen {acc['residual_positive']} mine {rp}")
    if diffs:
        res['accounting_contradictions'][v['id']] = diffs
    if len(assets) > 1:
        inv['multi_asset'].append(v['id'])
    for e in eff:
        if e['kind'] == 'Transfer' and e['from'] == e['to']:
            inv['self_transfer'].append(v['id'])
        if e['kind'] == 'Fee':
            inv['fee'].append(v['id'])
            if e['from'] == e['to']:
                inv['self_fee'].append(v['id'])
    if v.get('logical_cap') is not None:
        inv['capped'].append(v['id'])
    if acc['residual_positive']:
        inv['residual_positive_nonempty'].append(v['id'])
for k in inv:
    inv[k] = sorted(set(inv[k]))
res['inventory'] = inv
res['inventory']['accepted_field_names'] = {
    'balance': sorted(accepted[0]['expected']['accounting']['balances'][0]),
    'duty': sorted(accepted[0]['expected']['accounting']['duties'][0]) if accepted[0]['expected']['accounting']['duties'] else [],
    'block': sorted(accepted[0]['expected']['accounting'])}

for v in rejected + [x for x in A2['vectors'] if not x['expected']['accepted']]:
    ex = v['expected']
    res['rejected'].append({'id': v['id'], 'program': v.get('program'), 'action': v.get('action'),
                            'error': ex.get('error'), 'step': ex.get('step'), 'phase': ex.get('phase'),
                            'work_attempted': ex.get('work_attempted'), 'cap': v.get('logical_cap'),
                            'phase_rule_consistent': (ex.get('phase') == ('admission' if ex.get('step') is None else 'execution'))})

# --- predicted table counts (reviewer derivation from the harness loop structure) ---
n_acc, n_rej = len(accepted), len(rejected)
a2_acc = [x for x in A2['vectors'] if x['expected']['accepted']]
a2_rej = [x for x in A2['vectors'] if not x['expected']['accepted']]
claims = H['rejected_vector_claims']
res['counts'] = {
    'vectors': len(vectors), 'accepted': n_acc, 'rejected': n_rej, 'hostile_claims': len(claims),
    'null_claims': sorted(c['base_vector'] for c in claims if c['claim'] is None),
    'addendum2_accepted': len(a2_acc), 'addendum2_rejected': len(a2_rej),
    'heldout_rows': 3 * n_acc + n_rej,
    'hostile_rows': len(claims) + 1,
    'addendum2_rows': 3 * len(a2_acc) + 2 * len(a2_rej),
    'rejected_rows': 2 * (n_rej + len(a2_rej)) + sum(1 for v in rejected + a2_rej if v.get('logical_cap') is None),
    'accounting_rows_without_outside': 3 * n_acc + len(a2_acc),
    'author_predicted': PRED,
}

# --- HW trace refs vs statically derivable row ids ---
ids_acc = {v['id'] for v in accepted}
ids_all = {v['id'] for v in vectors}
claim_ids = {c['base_vector'] for c in claims}
static_rows = {
    'mutations': {'M-01', 'M-02', 'M-08', 'M-03', 'M-04', 'M-05-honest', 'M-05', 'M-06/encoder', 'M-06/relation', 'M-07',
                  'M-09', 'M-10-honest', 'M-10', 'M-10/wire', 'HW-23-r3', 'M-11-honest', 'M-11a', 'M-11b', 'M-12',
                  'M-13/program_hash', 'M-13/shape_digest', 'M-13/action_index', 'M-14'},
    'addendum4': {'format', 'HW-14-r3/identity', 'HW-14-r3/control', 'HW-14-r3', 'HW-15-r3', 'HW-18-r3',
                  'HW-23-r3/H6-B1/control', 'HW-23-r3/H6-B1/two', 'HW-23-r3/H6-B1/minus-one', 'HW-23-r3/H6-B1/wire-two',
                  'HW-26-r3/unsorted', 'HW-26-r3/padding'},
    'hw25': {'HW-25-r3/core', 'HW-25-r3/layout', 'HW-25-r3/premise', 'HW-25-r3/encoder'},
    'heldout': ids_all,
    'hostile': claim_ids | {'open-rows'},
    'addendum2': {x['id'] for x in A2['vectors']} | {x['id'] + '/W' for x in a2_acc},
    'rejected': {f"{v['id']}/{s}" for v in rejected + a2_rej for s in ('phase', 'work_attempted', 'meter-method')},
}
bad = []
for e in TR['entries']:
    for r in e['refs']:
        rows = static_rows.get(r['table'])
        if rows is None or r['row'] not in rows:
            bad.append((e['id'], r['table'], r['row']))
res['hw_refs_unresolvable_statically'] = bad
res['hw_ids'] = [e['id'] for e in TR['entries']]
res['hw_frozen_ids'] = [h['id'] for h in V['hostile_witness_proposals']]
res['hw_classes'] = {e['id']: e['class'] for e in TR['entries']}

with open(OUT, 'x') as f:
    json.dump(res, f, indent=1, sort_keys=True)
print(json.dumps({k: res[k] for k in ('accounting_contradictions', 'inventory', 'counts', 'hw_refs_unresolvable_statically')}, indent=1))
print([r for r in res['rejected'] if not r['phase_rule_consistent']])
print([(r['id'], r['error'], r['step'], r['phase'], r['work_attempted'], r['cap']) for r in res['rejected']])
