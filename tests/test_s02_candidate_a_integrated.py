import pytest
from scripts.a4_carrier import Invalid,M,V,r,decode,encode,typed,require

def test_duplicate_map_rejected():
    with pytest.raises(Invalid,match='duplicate map'):
        decode({'#map': [[1,True],[1,False]]})

def test_unknown_fields_and_boolean_integer_rejected():
    with pytest.raises(Invalid): typed(r(domain=V('SwapDomain'),principal=V('Alice'),nonce=True),'Key')
    with pytest.raises(Invalid): typed(r(domain=V('SwapDomain'),principal=V('Alice'),nonce=0,extra=1),'Key')

def test_absent_is_not_zero():
    assert V('NoInt') != V('IntValue',0)
    assert decode(encode(V('IntValue',0))) == V('IntValue',0)

def test_list_and_tuple_carriers_are_not_interchangeable():
    from scripts.a4_carrier import L,T
    with pytest.raises(Invalid): typed(decode({'tag':'NoInt','value':[]}), 'Optional')
    with pytest.raises(Invalid): typed(decode({'#tup':[]}),L('int'))
    with pytest.raises(Invalid): typed(decode([]),T())
    typed(decode({'tag':'NoInt','value':{'#tup':[]}}),'Optional')
    assert encode(V('EffectsExtractedA',()))=={'tag':'EffectsExtractedA','value':[]}

def test_frozen_oracle_close_refund_and_rollback():
    from scripts.a4_agreement import ACCOUNTS,CHOICES,oracle
    aa=r(owner=V('Alice'),asset=V('TokenA'))
    program=r(root=V('N0'),nodes=M(tuple((V('N'+str(i)),V('CloseA')) for i in range(16))))
    state=r(continuation=V('N0'),accounts=M(tuple((a,5 if a==aa else 0) for a in ACCOUNTS)),
        choices=M(tuple((c,V('NoInt')) for c in CHOICES)),minimumTime=V('Time2'))
    request=r(before=r(program=program,state=state),input=V('NoAInput'),now=V('Time2'))
    result,effects=oracle(request)
    assert result['accepted'] is True and result['reductions']==1 and result['state']['accounts'][aa]==0
    assert result['payments']==(r(source=aa,recipient=V('Alice'),asset=V('TokenA'),quantity=5),)
    assert effects==(r(source=V('Escrow',aa),destination=V('Wallet',V('Alice')),asset=V('TokenA'),quantity=5),)
    from scripts.a4_carrier import change
    supplied=V('PresentAInput',V('ChoiceInputA',r(id=V('RecoveryId'),chooser=V('Alice'),chosen=1)))
    refused,none=oracle(change(request,input=supplied))
    assert refused==r(accepted=False,state=state,error=V('CoreErrorCode','contract_closed'),payments=(),warnings=(),reductions=0)
    assert none==()

from scripts.a4_authority import transfers,can_transfer,LEDGER_KEYS
from scripts.a4_authority import KEYS,IDS,can_reject,reject
from scripts.a4_carrier import change

def test_uncoupled_rejection_is_reachable():
    program=r(root=V('N0'),nodes=M(tuple((V('N'+str(i)),V('CloseA')) for i in range(16))))
    from scripts.a4_agreement import ACCOUNTS,CHOICES
    accounts=M(tuple((a,1 if a==r(owner=V('Alice'),asset=V('TokenA')) else 0) for a in ACCOUNTS))
    before=r(program=program,state=r(continuation=V('N0'),accounts=accounts,
        choices=M(tuple((c,V('NoInt')) for c in CHOICES)),minimumTime=V('Time2')))
    context=r(candidate=before,ledger=M(tuple((k,0) for k in LEDGER_KEYS)),
        environment=r(physicalTime=2,anchor=0,implementationVersion=0,enforcementMechanism=0),
        registry=M(tuple((k,V('AuthorityUnused')) for k in KEYS)),parents=M(tuple((k,V('ParentVacant')) for k in KEYS)))
    obs=r(predecessor=before,proposedSuccessor=before,artifactAndCall=V('AgreementCallA',r(before=before,input=V('NoAInput'),now=V('Time2'))),
        resolvedPlan=r(identity=V('SwapPlanA'),operations=()),transactionTime=2,input=V('NoInput'),effects=(),
        outcome=V('DeadlineRefund'),coreProjection=V('NoCoreProjection'),effectEvidence=V('EvidenceValid'),display=V('PublicDisplay'))
    attempt=r(id=V('DispositionAttempt'),actor=V('Alice'),operation=V('OpDeadlineRefund'),observation=obs,context=context)
    evidence=r(effect=r(attempt=attempt,disposition=V('EvidenceValid')),signatures=M(()))
    state=r(authority=r(context=context,signing=M(tuple((k,V('NoSigningCheck')) for k in KEYS))),
        attempts=M(tuple((i,V('NoAttempt')) for i in IDS)).put(attempt['id'],V('ProposedAttempt',attempt)))
    typed(state,'Execution'); typed(evidence,'Evidence')
    assert can_reject(state,attempt['id'],evidence)
    result=reject(state,attempt['id'],evidence)
    assert result['authority']==state['authority']
    assert result['attempts'][attempt['id']].value['reason']==V('UnauthorizedEffect')

def test_transfer_order():
    alice=V('Wallet',V('Alice')); bob=V('Wallet',V('Bob')); a=V('TokenA')
    ledger=M(tuple((k,10 if k==(alice,a) else 0) for k in LEDGER_KEYS))
    one=r(source=alice,destination=bob,asset=a,quantity=10)
    two=r(source=bob,destination=alice,asset=a,quantity=10)
    assert can_transfer(ledger,(one,two))
    assert not can_transfer(ledger,(two,one))
    assert transfers(ledger,(one,two))==ledger

from scripts.a4_cases import initial,ordinary,I_SCENARIOS,S_SCENARIOS,PROFILES,prefix,program,policy_for,planned
from scripts.a4_authority import a_execution,plan_valid

def test_independent_initial_coupling():
    assert a_execution(initial('installment')) and a_execution(initial('swap'))
    assert initial('installment')['authority']['context']['ledger'][(V('Escrow',r(owner=V('Alice'),asset=V('TokenA'))),V('TokenA'))]==10

@pytest.mark.parametrize('profile',PROFILES)
@pytest.mark.parametrize('loop,scenarios',(('installment',I_SCENARIOS),('swap',S_SCENARIOS)))
def test_independent_ordinary_routes(profile,loop,scenarios):
    for scenario in scenarios:
        state=prefix(loop,scenario,profile,len(ordinary(loop,scenario)))
        assert a_execution(state)
        assert all(cell.tag not in ('ProposedAttempt','VerifiedOperation') for _,cell in state['attempts'].pairs)
    assert len(scenarios)==(11 if loop=='installment' else 12)

def test_complete_parent_program_plan():
    p=policy_for('installment','two-fills','SignAfterResolve','FirstFillAttempt')
    assert len(p['binding'].value['identity']['operations'])==4
    assert plan_valid(p['binding'].value['identity'])
    assert len(program('installment')['nodes'].pairs)==16
    assert planned('installment','recover-r1-refuse100','RecoveryAttempt')['coreProjection'].value['reductions']==0

from scripts.a4_inventory import descriptors,history,terminal

def test_inventory_is_not_submission_defined():
    ds=descriptors(); assert len(ds)==78 and len({d['case_id'] for d in ds})==78
    assert sum(len(history(d)) for d in ds)==1557
    totals={}
    for d in ds:
        family=d['lifecycle']+'/'+('ordinary' if d['control']=='ordinary' else 'verified-stale' if d['control']=='verified-stale' else 'negative')
        totals[family]=totals.get(family,0)+len(history(d))
    assert totals=={'installment/ordinary':516,'swap/ordinary':484,'swap/verified-stale':44,'installment/negative':122,'swap/negative':391}

def test_denials_rejections_and_derivations_are_distinct():
    for d in descriptors():
        for event in history(d):
            latest=event['latest']; kind=latest['kind']
            if kind=='denied-probe': assert event['before']==event['after'] and latest['observedGuard'] is False
            if kind=='adversarial-derivation': assert event['before']['authority']==event['after']['authority']
            if kind=='transition' and latest['arguments']['command'].tag.startswith('Reject'):
                assert event['before']['authority']==event['after']['authority']
                assert event['before']['attempts']!=event['after']['attempts']

def test_observed_raw_is_not_forged_projection():
    for d in descriptors():
        if d['control']!='reductions': continue
        e=next(x for x in history(d) if x['latest']['kind']=='adversarial-derivation')
        latest=e['latest']; claimed=latest['arguments']['command'].value['attempt']['observation']['coreProjection'].value
        raw=latest['computations'][0]['evaluation'].value
        assert claimed['reductions']==raw['reductions']+1

def test_binding_checks_observed_field():
    from copy import deepcopy
    from scripts.check_s02_candidate_a_integrated import bind_event,RENAME
    event={k:0 for k in RENAME}; event['profile']='SignBeforeResolve'
    event.update(before={},after={},provenance={'input_path':'swap.itf.json','input_sha256':'0'*64,'before_index':0,'after_index':0})
    raw={'states':[{'authorityState':{},'latestEvent':{v:event[k] for k,v in RENAME.items()}}]}
    bind_event(event,raw,0,'swap.itf.json','0'*64)
    bad=deepcopy(event); bad['profile']='SignAfterResolve'
    with pytest.raises(Invalid,match='raw provenance field profile'): bind_event(bad,raw,0,'swap.itf.json','0'*64)

from scripts.check_s02_candidate_a_integrated import (
    bind_window, check_shard, compare_case, file_digest, manifest_contract,
    ENTRIES, RAW_NAMES, CASE_NAMES, TRANSPORT, VARS, RENAME, PYTHON_SOURCES, QNT_TESTS)
from scripts.a4_json_stream import JsonStreamError, write_array_document


def synthetic_shard(tmp_path, ordinal=36, raw_change=None, case_change=None):
    # Small file-backed unit control, explicitly not native generator evidence.
    from copy import deepcopy
    import json
    d = descriptors()[ordinal]
    records = history(d)
    states = []
    events = []
    for index, record in enumerate(records):
        latest = {name: encode(value) for name, value in record['latest'].fields}
        latest['sequence'] = {'#bigint': str(index)}
        states.append({'#meta': {'index': index}, 'authorityState': encode(record['after']),
                       'latestEvent': latest, 'caseIndex': {'#bigint': str(ordinal)},
                       'cursor': {'#bigint': str(index)}})
        event = {public: encode(record['latest'][internal]) for public, internal in RENAME.items()}
        event.update(before=encode(record['before']), after=encode(record['after']))
        events.append(event)
    meta = {'format': 'ITF', 'format-description': 'https://apalache-mc.org/docs/adr/015adr-trace.html',
            'source': ENTRIES[d['case_id']], 'status': 'ok',
            'description': 'synthetic unit; not generator evidence', 'timestamp': 0}
    raw = {'#meta': meta, 'vars': ['authorityState', 'caseIndex', 'cursor', 'latestEvent'], 'states': states}
    if raw_change is not None: raw_change(raw)
    raw_path = tmp_path/f'raw-{ordinal}.json'
    with raw_path.open('wb') as stream:
        write_array_document(stream, 'states', raw['states'], after=(('#meta', raw['#meta']), ('vars', raw['vars'])))
    sha = file_digest(raw_path)
    for index, event in enumerate(events):
        event['provenance'] = {'input_path': f'raw/case-{ordinal:03d}.itf.json', 'input_sha256': sha,
                               'before_index': max(0, index-1), 'after_index': index}
    case = dict(d, events=events)
    if case_change is not None: case_change(case)
    case_path = tmp_path/f'case-{ordinal}.json'
    with case_path.open('wb') as stream:
        write_array_document(stream, 'events', case['events'], after=((k, v) for k, v in case.items() if k != 'events'))
    return d, ordinal, case_path, raw_path, sha, case


def test_sharded_window_profile_red(tmp_path):
    import json
    from copy import deepcopy
    d, ordinal, case_path, raw_path, sha, _ = synthetic_shard(tmp_path)
    event = json.loads(case_path.read_bytes())['events'][0]
    current = json.loads(raw_path.read_bytes())['states'][0]
    name = f'raw/case-{ordinal:03d}.itf.json'
    bind_window(event, current, current, 0, name, sha)
    bad = deepcopy(event)
    bad['profile'] = 'SignBeforeResolve'
    assert bad['profile'] != event['profile']
    with pytest.raises(Invalid, match='raw provenance field profile'):
        bind_window(bad, current, current, 0, name, sha)


@pytest.mark.parametrize('ordinal', (36, 37))
def test_sharded_bounded_matches_buffered(tmp_path, ordinal):
    d, g, case_path, raw_path, sha, case = synthetic_shard(tmp_path, ordinal)
    expected = compare_case(case, d)
    assert check_shard(case_path, raw_path, d, g, sha) == len(expected) == 7


@pytest.mark.parametrize('mutation', ('ordinal', 'cursor', 'index', 'status', 'vars', 'sequence', 'missing', 'extra'))
def test_sharded_raw_controls(tmp_path, mutation):
    from copy import deepcopy
    def alter(raw):
        if mutation == 'ordinal': raw['states'][0]['caseIndex'] = {'#bigint': '0'}
        elif mutation == 'cursor': raw['states'][0]['cursor'] = True
        elif mutation == 'index': raw['states'][0]['#meta']['index'] = 99
        elif mutation == 'status': raw['#meta']['status'] = 'error'
        elif mutation == 'vars': raw['vars'].reverse()
        elif mutation == 'sequence': raw['states'][0]['latestEvent']['sequence'] = {'#bigint': '99'}
        elif mutation == 'missing': raw['states'].pop()
        else: raw['states'].append(deepcopy(raw['states'][-1]))
    d, g, case_path, raw_path, sha, _ = synthetic_shard(tmp_path, raw_change=alter)
    with pytest.raises(Invalid): check_shard(case_path, raw_path, d, g, sha)


@pytest.mark.parametrize('mutation', ('descriptor', 'missing', 'extra', 'initial-before'))
def test_sharded_case_controls(tmp_path, mutation):
    from copy import deepcopy
    def alter(case):
        if mutation == 'descriptor': case['profile'] = 'SignBeforeResolve'
        elif mutation == 'missing': case['events'].pop()
        elif mutation == 'extra': case['events'].append(deepcopy(case['events'][-1]))
        else: case['events'][0]['before']['authority']['context']['environment']['anchor'] = 1
    d, g, case_path, raw_path, sha, _ = synthetic_shard(tmp_path, case_change=alter)
    with pytest.raises(Invalid): check_shard(case_path, raw_path, d, g, sha)


@pytest.mark.parametrize('target', ('case', 'raw'))
def test_sharded_stream_suffix_failure(tmp_path, target):
    d, g, case_path, raw_path, sha, case = synthetic_shard(tmp_path)
    path = case_path if target == 'case' else raw_path
    with path.open('ab') as stream: stream.write(b' trailing')
    if target == 'raw':
        sha = file_digest(raw_path)
        for event in case['events']: event['provenance']['input_sha256'] = sha
        with case_path.open('wb') as stream:
            write_array_document(stream, 'events', case['events'], after=((k, v) for k, v in d.items()))
    with pytest.raises(JsonStreamError): check_shard(case_path, raw_path, d, g, sha)


def test_sharded_manifest_closed_schema_and_paths():
    from copy import deepcopy
    ds = descriptors()
    assert len(ENTRIES) == len(RAW_NAMES) == len(CASE_NAMES) == 78
    assert ENTRIES[ds[36]['case_id']] == 'specs/quint/s02/candidate_a_integrated_case_036.qnt'
    assert 'scripts/a4_json_stream.py' in PYTHON_SOURCES and 'tests/test_a4_json_stream.py' in PYTHON_SOURCES
    assert 'specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt' in QNT_TESTS
    pins = {'source_pins': {'file.py': '0'*64}, 'input_pins': {p: '0'*64 for p in RAW_NAMES},
            'case_pins': {p: '0'*64 for p in CASE_NAMES}, 'receipt_pins': {'receipt.json': '0'*64}}
    document = dict(schema_version=3, transport=TRANSPORT, inventory_sha256='0'*64, shards=[], **pins)
    admitted = {k: v for k, v in document.items() if k != 'shards'} | {'entries': ENTRIES}
    # Schema/pin-shape unit only: no filesystem or package success is asserted.
    manifest_contract(document, admitted)
    for mutation in ('schema2', 'staging', 'missing-raw', 'missing-case', 'overlap', 'bad-entry'):
        bad = deepcopy(document)
        admission = deepcopy(admitted)
        if mutation == 'schema2': bad['schema_version'] = 2
        elif mutation == 'staging': del admission['case_pins']
        elif mutation == 'missing-raw':
            name = next(iter(bad['input_pins']))
            del bad['input_pins'][name]; del admission['input_pins'][name]
        elif mutation == 'missing-case':
            name = next(iter(bad['case_pins']))
            del bad['case_pins'][name]; del admission['case_pins'][name]
        elif mutation == 'overlap':
            bad['receipt_pins'] = dict(bad['input_pins']); admission['receipt_pins'] = dict(bad['input_pins'])
        else: admission['entries'][ds[36]['case_id']] = 'wrong.qnt'
        with pytest.raises(Invalid): manifest_contract(bad, admission)


@pytest.mark.parametrize('target', ('states', 'events'))
def test_sharded_unknown_metadata_rejected_before_items(target):
    import io
    from scripts.check_s02_candidate_a_integrated import array_items
    source = io.BytesIO(('{"unknown":0,"'+target+'":[1]}').encode())
    metadata = {}
    items = array_items(source, target, metadata)
    with pytest.raises(Invalid, match='unexpected stream metadata'):
        next(items)
    assert metadata == {}

# Task6 additions: synthetic files are never native-package evidence.
from copy import deepcopy
from pathlib import Path
import gc
import json
import os
import subprocess
import sys
from scripts.a4_carrier import loads
from scripts.check_s02_candidate_a_integrated import (
    canonical, digest, array_items, validate_raw_state, validate_raw_metadata,
    raw_same, check_package, load_manifest, safe_path, shape)

SEMANTIC_MUTANTS = (
    'optional-zero','nonce','signer','token','revision','parent-paid',
    'rejection-reason','rejection-stage','raw-reductions','claimed-reductions',
    'effects-order','request-chooser','plan-omission','unused-program',
    'signing-snapshot','concealed-rejection','omitted-computation',
    'fabricated-cancellation-core','denial-as-transition','observed-guard',
    'event-order','payment-order','deposit-effect','rollback-time',
    'retained-loser','proof-context','cancellation-identity')

def task6_descriptor(loop, scenario, profile='SignBeforeResolve'):
    return next(d for d in descriptors() if
                (d['lifecycle'], d['scenario'], d['control'], d['profile']) ==
                (loop, scenario, 'ordinary', profile))

def rewrite(value, rule):
    if type(value) is dict: value = {k: rewrite(v, rule) for k, v in value.items()}
    elif type(value) is list: value = [rewrite(v, rule) for v in value]
    return rule(value)

def mutation_value(value, name):
    def rule(x):
        if type(x) is not dict: return x
        if name=='optional-zero' and x=={'tag':'NoInt','value':{'#tup':[]}}: return {'tag':'IntValue','value':0}
        if name=='nonce' and set(x)=={'policy','signer','token'} and x['policy']['body']['key']['nonce']==1:
            p=x['policy']; b=p['body']; return x|{'policy':p|{'body':b|{'key':b['key']|{'nonce':0}}}}
        if name=='signer' and set(x)=={'policy','signer','token'}: return x|{'signer':encode(V('Mallory'))}
        if name=='token' and set(x)=={'policy','signer','token'}: return x|{'token':1}
        if name=='revision' and 'revision' in x and type(x['revision']) is int: return x|{'revision':x['revision']+1}
        if name=='parent-paid' and {'paid','remainingAllowance','usedSlots'}<=set(x): return x|{'paid':x['paid']+1,'remainingAllowance':x['remainingAllowance']-1}
        if name=='rejection-reason' and {'reason','stage','observedContext'}<=set(x): return x|{'reason':encode(V('UnauthorizedEffect'))}
        if name=='rejection-stage' and {'reason','stage','observedContext'}<=set(x): return x|{'stage':encode(V('VerificationBoundary'))}
        if name=='raw-reductions' and x.get('tag')=='TransactionComputedA': return x|{'value':x['value']|{'reductions':x['value']['reductions']+1}}
        if name=='payment-order' and 'payments' in x and type(x['payments']) is list and len(x['payments'])==2: return x|{'payments':list(reversed(x['payments']))}
        if name=='deposit-effect' and set(x)=={'source','destination','asset','quantity'} and x['source'].get('tag')=='Wallet' and x['destination'].get('tag')=='Escrow': return x|{'quantity':x['quantity']+1}
        if name=='rollback-time' and x.get('tag')=='TransactionComputedA' and x['value']['accepted'] is False:
            raw=x['value']; return x|{'value':raw|{'state':raw['state']|{'minimumTime':encode(V('Time100'))}}}
        if name=='retained-loser' and x.get('tag')=='RejectedOperation': return encode(V('NoAttempt'))
        if name=='proof-context' and set(x)=={'attempt','disposition'}:
            a=x['attempt']; c=a['context']; return x|{'attempt':a|{'context':c|{'environment':c['environment']|{'anchor':1}}}}
        if name=='cancellation-identity' and 'artifactAndCall' in x and x['artifactAndCall'].get('tag')=='CancellationCallA' and 'proposedSuccessor' in x:
            b=x['proposedSuccessor']; return x|{'proposedSuccessor':b|{'state':b['state']|{'minimumTime':encode(V('Time100'))}}}
        if name=='claimed-reductions' and x.get('tag')=='CoreProjected': return x|{'value':x['value']|{'reductions':x['value']['reductions']+1}}
        if name=='effects-order' and 'effects' in x and type(x['effects']) is list and len(x['effects'])==2: return x|{'effects':list(reversed(x['effects']))}
        if name=='request-chooser' and x.get('tag')=='ChoiceInputA': return x|{'value':x['value']|{'chooser':encode(V('Mallory'))}}
        if name=='plan-omission' and 'operations' in x and type(x['operations']) is list and len(x['operations'])==4: return x|{'operations':x['operations'][:-1]}
        if name=='unused-program' and set(x)=={'root','nodes'}:
            nodes=deepcopy(x['nodes'])
            for pair in nodes['#map']:
                if pair[0]==encode(V('N15')): pair[1]=encode(V('PayA',r(account=r(owner=V('Alice'),asset=V('TokenA')),payee=V('Bob'),amount=V('ConstantA',5),continuation=V('N0'))))
            return x|{'nodes':nodes}
        if name=='signing-snapshot' and set(x)=={'physicalTime','anchor','implementationVersion','enforcementMechanism'}: return x|{'anchor':1}
        return x
    return rewrite(value, rule)

def task6_items(path, target):
    metadata = {}
    with Path(path).open('rb') as stream:
        yield from array_items(stream, target, metadata)

def task6_write(path, target, items, metadata):
    with Path(path).open('xb') as stream:
        return write_array_document(stream, target, items, after=metadata.items())

def task6_honest(path, d):
    def events():
        # Only this one independent history is retained; no full inventory history.
        for record in history(d):
            e = {public: encode(record['latest'][internal])
                 for public, internal in RENAME.items()}
            yield e | {'before': encode(record['before']),
                       'after': encode(record['after']), 'provenance': {}}
    task6_write(path, 'events', events(), d)
    gc.collect()

def task6_mutate(source, output, d, name):
    changed = 0
    done = False
    saved = None
    sample = None
    def events():
        nonlocal changed, done, saved, sample
        for index, e in enumerate(task6_items(source, 'events')):
            before = canonical(e)
            if sample is None and e['computations']:
                sample = deepcopy(e['computations'])
            if name == 'event-order':
                if index == 1:
                    saved = e
                    continue
                if index == 2:
                    changed += 2
                    yield e
                    yield saved
                    saved = None
                    continue
            elif name == 'concealed-rejection' and not done:
                if e['kind'] == 'transition' and e['arguments']['command']['tag'].startswith('Reject'):
                    changed += 1
                    done = True
                    continue
            elif name == 'omitted-computation' and not done and e['computations']:
                e['computations'] = []
                done = True
            elif name == 'fabricated-cancellation-core' and not done:
                cmd = e['arguments']['command']
                if cmd['tag'] == 'ProposeA4' and cmd['value']['operation']['tag'] == 'OpCancelParent':
                    require(sample is not None, 'cancellation mutation needs earlier actual computation')
                    e['computations'] = deepcopy(sample)
                    done = True
            elif name in ('denial-as-transition', 'observed-guard') and not done:
                if e['kind'] == 'denied-probe':
                    if name == 'denial-as-transition': e['kind'] = 'transition'
                    else: e['observed_guard'] = True
                    done = True
            elif name not in ('concealed-rejection', 'omitted-computation',
                              'fabricated-cancellation-core',
                              'denial-as-transition', 'observed-guard', 'event-order'):
                e = mutation_value(e, name)
            if canonical(e) != before: changed += 1
            yield e
        require(saved is None, 'event-order fixture has second event')
    task6_write(output, 'events', events(), d)
    require(changed > 0, 'effective mutation '+name)
    require(file_digest(source) != file_digest(output), 'changed artifact '+name)
    return changed

def task6_rebind(source, folder, d):
    folder.mkdir()
    ordinal = descriptors().index(d)
    normalized = folder/'normalized.json'
    raw = folder/'raw.itf.json'
    case = folder/'case.json'
    def normalized_events():
        previous = None
        for index, e in enumerate(task6_items(source, 'events')):
            e['sequence'] = index
            e['before'] = deepcopy(e['after'] if previous is None else previous)
            previous = e['after']
            e['provenance'] = {}
            yield e
    count = task6_write(normalized, 'events', normalized_events(), d)
    meta = {'format': 'ITF',
            'format-description': 'https://apalache-mc.org/docs/adr/015adr-trace.html',
            'source': ENTRIES[d['case_id']], 'status': 'ok',
            'description': 'synthetic rebound control; not generator evidence',
            'timestamp': 0}
    def states():
        for index, e in enumerate(task6_items(normalized, 'events')):
            latest = {internal: e[public] for public, internal in RENAME.items()}
            latest['sequence'] = {'#bigint': str(index)}
            yield {'#meta': {'index': index}, 'authorityState': e['after'],
                   'latestEvent': latest, 'caseIndex': {'#bigint': str(ordinal)},
                   'cursor': {'#bigint': str(index)}}
    assert task6_write(raw, 'states', states(), {'#meta': meta, 'vars': list(VARS)}) == count
    sha = file_digest(raw)
    def rebound_events():
        for index, e in enumerate(task6_items(normalized, 'events')):
            e['provenance'] = {'input_path': f'raw/case-{ordinal:03d}.itf.json',
                               'input_sha256': sha, 'before_index': max(0, index-1),
                               'after_index': index}
            yield e
    assert task6_write(case, 'events', rebound_events(), d) == count
    return case, raw, sha, ordinal, count

def task6_bound(artifact, d):
    case, raw, sha, ordinal, expected_count = artifact
    assert file_digest(raw) == sha
    case_meta, raw_meta = {}, {}
    absent = object()
    count = 0
    with case.open('rb') as cs, raw.open('rb') as rs:
        events = array_items(cs, 'events', case_meta)
        states = array_items(rs, 'states', raw_meta)
        previous = None
        while True:
            e, current = next(events, absent), next(states, absent)
            if e is absent or current is absent:
                assert e is absent and current is absent
                break
            validate_raw_state(current, count, ordinal)
            bind_window(e, current if previous is None else previous, current,
                        count, f'raw/case-{ordinal:03d}.itf.json', sha)
            previous = current
            count += 1
    assert count == expected_count
    assert raw_same(case_meta, d)
    validate_raw_metadata(raw_meta, ENTRIES[d['case_id']])
    return count

def task6_accept(artifact, d):
    task6_bound(artifact, d)
    case, raw, sha, ordinal, count = artifact
    assert check_shard(case, raw, d, ordinal, sha) == count

def task6_other(folder, loop='swap'):
    d = (task6_descriptor('swap', 'funded2-refund', 'SignAfterResolve')
         if loop == 'swap' else task6_descriptor('installment', 'two-fills'))
    source = folder/'other-source.json'
    task6_honest(source, d)
    artifact = task6_rebind(source, folder/'other', d)
    task6_accept(artifact, d)
    gc.collect()

def task6_first_pair(artifact, index):
    case, raw, sha, ordinal, count = artifact
    # Read only the requested bounded window, closing both generators explicitly.
    es = task6_items(case, 'events')
    ss = task6_items(raw, 'states')
    previous = None
    try:
        for i in range(index+1):
            e, current = next(es), next(ss)
            if i == index:
                return e, current if previous is None else previous, current
            previous = current
    finally:
        es.close()
        ss.close()

def task6_worker(mode, name, folder):
    folder = Path(folder)
    if mode == 'semantic':
        d = (task6_descriptor('swap', 'funded2-settle')
             if name in ('effects-order', 'payment-order', 'deposit-effect')
             else task6_descriptor('installment',
                  'recover-r1-refuse100' if name == 'rollback-time' else 'recover-r1-choice2'))
        source = folder/'honest-source.json'
        task6_honest(source, d)
        mutated = folder/'mutated-source.json'
        changed = task6_mutate(source, mutated, d, name)
        bad = task6_rebind(mutated, folder/'bad', d)
        task6_bound(bad, d)  # MUST succeed before semantic rejection is counted.
        with pytest.raises(Invalid):
            case, raw, sha, ordinal, _ = bad
            check_shard(case, raw, d, ordinal, sha)
        corrected = task6_rebind(source, folder/'corrected', d)
        task6_accept(corrected, d)
        task6_other(folder)
        return {'mode': mode, 'name': name, 'changed_events': changed,
                'raw_linkage': True, 'mutant_rejected': True,
                'corrected': True, 'unrelated': True}
    d = task6_descriptor('swap', 'funded2-settle')
    source = folder/'honest-source.json'
    task6_honest(source, d)
    honest = task6_rebind(source, folder/'honest', d)
    index = 0 if mode == 'substitution' else 1
    event, previous, current = task6_first_pair(honest, index)
    if mode == 'substitution':
        assert event['profile'] == 'SignBeforeResolve'
        event['profile'] = 'SignAfterResolve'
        message = 'raw provenance field profile'
    else:
        field, value = (('before_index',99), ('after_index',99),
                        ('after_index',True), ('input_path','../swap.itf.json'),
                        ('input_sha256','0'*64))[int(name)]
        event['provenance'][field] = value
        message = None
    bad_case = folder/'substituted-case.json'
    def substituted_events():
        for i, original_event in enumerate(task6_items(honest[0], 'events')):
            yield event if i == index else original_event
    task6_write(bad_case, 'events', substituted_events(), d)
    bad_artifact = (bad_case, *honest[1:])
    event, previous, current = task6_first_pair(bad_artifact, index)
    with pytest.raises(Invalid, match=message):
        bind_window(event, previous, current, index,
                    f'raw/case-{honest[3]:03d}.itf.json', honest[2])
    del event, previous, current
    task6_accept(honest, d)
    task6_other(folder, 'installment')
    return {'mode': mode, 'name': name, 'mutant_rejected': True,
            'corrected': True, 'unrelated': True}

def task6_process(tmp_path, mode, name):
    # No moving producer imports. Same test source is the pinned worker.
    root = Path(__file__).resolve().parents[1]
    script = "import runpy,sys,json; m=runpy.run_path(sys.argv[1]); print(json.dumps(m['task6_worker'](sys.argv[2],sys.argv[3],sys.argv[4]),sort_keys=True))"
    argv = [sys.executable, '-B', '-X', 'pycache_prefix='+str(tmp_path/'fresh-cache'),
            '-c', script, str(Path(__file__).resolve()), mode, str(name), str(tmp_path)]
    env = os.environ.copy()
    env.pop('PYTHONPATH', None)
    env['PYTHONDONTWRITEBYTECODE'] = '1'
    env['PYTHONPYCACHEPREFIX'] = str(tmp_path/'fresh-cache')
    (tmp_path/'argv.json').write_text(json.dumps(argv))
    with (tmp_path/'stdout.txt').open('xb') as out, (tmp_path/'stderr.txt').open('xb') as err:
        completed = subprocess.run(argv, cwd=root, env=env, stdout=out, stderr=err)
    (tmp_path/'terminal.json').write_text(json.dumps({'exit': completed.returncode}))
    assert completed.returncode == 0, (tmp_path/'stderr.txt').read_text()
    report = json.loads((tmp_path/'stdout.txt').read_text())
    assert report['mode'] == mode and report['name'] == str(name)
    assert report['mutant_rejected'] and report['corrected'] and report['unrelated']
    return report

@pytest.mark.parametrize('name', SEMANTIC_MUTANTS)
def test_semantic_rebound_triples(tmp_path, name):
    report = task6_process(tmp_path, 'semantic', name)
    assert report['changed_events'] > 0 and report['raw_linkage']

def test_raw_latest_field_substitution_triple(tmp_path):
    task6_process(tmp_path, 'substitution', 'profile')

@pytest.mark.parametrize('locator', range(5))
def test_provenance_locator_triples(tmp_path, locator):
    task6_process(tmp_path, 'locator', str(locator))

def test_duplicate_json_map_set_and_unit_controls():
    with pytest.raises(Invalid): loads('{"schema_version":3,"schema_version":3}')
    with pytest.raises(Invalid): decode({'#set':[1,1]})
    with pytest.raises(Invalid): decode({'#map':[[1,0],[1,0]]})
    with pytest.raises(Invalid): shape({'tag':'NoInt','value':[]},V('NoInt'))
    shape({'tag':'NoInt','value':{'#tup':[]}},V('NoInt'))
    shape({'tag':'IntValue','value':{'#bigint':'0'}},V('IntValue',0))

def test_path_traversal_and_symlink_triples(tmp_path):
    target = tmp_path/'honest.json'
    target.write_text('{}')
    alias = tmp_path/'alias.json'
    alias.symlink_to(target)
    with pytest.raises(Invalid): safe_path(tmp_path,'../honest.json')
    with pytest.raises(Invalid): safe_path(tmp_path,'alias.json')
    assert safe_path(tmp_path,'honest.json') == target
    other = tmp_path/'other.json'
    other.write_text('[]')
    assert safe_path(tmp_path,'other.json') == other

def test_actual_complete_package_and_inventory_triples(tmp_path):
    root = Path(__file__).resolve().parents[1]
    folder = root/'evidence/s02-candidate-a-completion/a4'
    manifest, admission, inv = folder/'cases.json', folder/'admission.json', folder/'inventory.json'
    require(manifest.is_file() and admission.is_file(), 'actual A4 package required; no skip')
    doc, admitted = load_manifest(manifest), load_manifest(admission)
    def honest():
        result = check_package(manifest, admission, root, folder, inv)
        assert result['ok'] is True and result['events'] == 1557
        assert result['cases'] == result['shards'] == result['validated_inputs'] == result['validated_case_files'] == 78
        gc.collect()
    honest()
    for name in ('missing-case','duplicate-case','extra-case','missing-transitive-pin',
                 'changed-frozen-pin','missing-input','changed-receipt'):
        bad, auth = deepcopy(doc), deepcopy(admitted)
        if name == 'missing-case': bad['shards'].pop()
        elif name == 'duplicate-case': bad['shards'][-1] = deepcopy(bad['shards'][0])
        elif name == 'extra-case': bad['shards'].append(deepcopy(bad['shards'][0]))
        elif name == 'missing-transitive-pin':
            path = 'specs/quint/s02/consumption.qnt'
            del bad['source_pins'][path]
            del auth['source_pins'][path]
        elif name == 'changed-frozen-pin':
            path = 'specs/quint/s02/consumption.qnt'
            bad['source_pins'][path] = auth['source_pins'][path] = '0'*64
        elif name == 'missing-input':
            path = 'raw/case-032.itf.json'
            del bad['input_pins'][path]
            del auth['input_pins'][path]
        else:
            path = sorted(bad['receipt_pins'])[0]
            bad['receipt_pins'][path] = '0'*64
        bad_path, auth_path = tmp_path/(name+'-manifest.json'), tmp_path/(name+'-admission.json')
        bad_path.write_text(canonical(bad))
        auth_path.write_text(canonical(auth))
        with pytest.raises(Invalid): check_package(bad_path, auth_path, root, folder, inv)
        honest()  # All 78 semantics revisited; no semantic-result cache.
    task6_other(tmp_path, 'installment')

