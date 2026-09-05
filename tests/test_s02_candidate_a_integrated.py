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
