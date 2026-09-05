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
