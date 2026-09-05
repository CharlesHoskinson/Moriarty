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
