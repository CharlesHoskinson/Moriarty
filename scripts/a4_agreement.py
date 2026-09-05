from scripts import check_s02_candidate_a_correspondence as core
from scripts.a4_carrier import R,M,V,r,change,decode,encode,typed,Invalid,require

PARTIES = tuple(V(x) for x in ('Alice','Bob','Mallory'))
ASSETS = tuple(V(x) for x in ('TokenA','TokenB'))
ACCOUNTS = frozenset(r(owner=p,asset=a) for p in PARTIES for a in ASSETS)
CHOICES = tuple(V(x) for x in core.CHOICES)

def time(x): return core.TIMES[x.tag]
def party(x): return next(V(k) for k,v in core.PARTIES.items() if x==v)
def asset(x): return next(V(k) for k,v in core.TOKENS.items() if x==v)
def account(x): return r(owner=party(x.owner),asset=asset(x.token))
def optional(x): return V('NoInt') if x is None else V('IntValue',x)

def valid_before(before):
    try:
        typed(before,'Before')
        core._decode_program(encode(before['program']),'program')
        core._state(encode(before['state']),'state')
        return True
    except (Invalid,core.DecodeError,ValueError,KeyError,TypeError): return False

def oracle(request):
    typed(request,'Request')
    before=request['before']; program=before['program']; state=before['state']
    _,nodes,_=core._decode_program(encode(program),'program')
    decoded=core._state(encode(state),'state')
    supplied=core._input(encode(request['input']),'input')
    now=time(request['now']); contract=nodes[state['continuation'].tag]
    actual=core.compute_transaction(contract,decoded,supplied,now=now)
    node=state['continuation'] if not actual.accepted else V(core._exact_node(actual.contract,nodes,'result'))
    balances=dict(actual.state.accounts); choices=dict(actual.state.choices)
    result_state=r(accounts=M(tuple((a,balances.get(core._account(encode(a),'account'),0)) for a in ACCOUNTS)),
                   choices=M(tuple((k,optional(choices.get(core.CHOICES[k.tag]))) for k in CHOICES)),
                   continuation=node,minimumTime=V('Time'+str(actual.state.min_time)))
    payments=tuple(r(source=account(p.source),recipient=party(p.to),asset=asset(p.token),quantity=p.quantity) for p in actual.payments)
    warnings=tuple(r(code=w.code,requested=optional(w.requested),paid=optional(w.paid)) for w in actual.warnings)
    raw=r(accepted=actual.accepted,state=result_state,error=V('NoCoreError') if actual.error is None else V('CoreErrorCode',actual.error),
          payments=payments,warnings=warnings,reductions=actual.reductions)
    effects=[]
    for sk,src,dk,dst,token,q in core._expected_effects(contract,decoded,supplied,now,actual):
        effects.append(r(source=V('Wallet',party(src)) if sk=='wallet' else V('Escrow',account(src)),
                         destination=V('Wallet',party(dst)) if dk=='wallet' else V('Escrow',account(dst)),asset=asset(token),quantity=q))
    return raw,tuple(effects)

def projection(program,raw):
    s=raw['state']
    return V('CoreProjected',change(raw,state=r(accounts=s['accounts'],
       choices=M(tuple((core.CHOICES[k.tag],v) for k,v in s['choices'].pairs)),
       continuation=r(program=program,node=s['continuation']),minimumTime=time(s['minimumTime']))))

def neutral(supplied):
    if supplied.tag=='NoAInput': return V('NoInput')
    x=supplied.value.value
    if supplied.value.tag=='ChoiceInputA':
        return V('ChoiceLike',r(id=core.CHOICES[x['id'].tag],chooser=x['chooser'],chosen=x['chosen']))
    return V('DepositLike',r(location=V('Escrow',x['account']),depositor=x['depositor'],asset=x['account']['asset'],quantity=x['quantity']))

def observation(op,call,plan,display):
    before=call.value['before']; now=call.value['now']
    if now==V('Time0') or not valid_before(before): return None
    if op.tag=='OpCancelParent':
        if call.tag!='CancellationCallA': return None
        return r(predecessor=before,proposedSuccessor=before,artifactAndCall=call,resolvedPlan=plan,transactionTime=time(now),
                 input=V('NoInput'),effects=(),outcome=V('Cancellation'),coreProjection=V('NoCoreProjection'),
                 effectEvidence=V('EvidenceValid'),display=display)
    if call.tag!='AgreementCallA': return None
    try: raw,effects=oracle(call.value)
    except (Invalid,core.DecodeError,ValueError,TypeError,KeyError): return None
    tag={'OpFund':'FundingAccepted','OpSettle':'Settlement','OpVoluntaryRefund':'VoluntaryRefund',
         'OpDeadlineRefund':'DeadlineRefund','OpRecover':'Recovery'}.get(op.tag)
    if op.tag=='OpFillSlot': tag='FirstInstallment' if op.value['slot']==1 else 'SecondInstallment'
    if tag is None: return None
    return r(predecessor=before,proposedSuccessor=r(program=before['program'],state=raw['state']),artifactAndCall=call,
             resolvedPlan=plan,transactionTime=time(now),input=neutral(call.value['input']),effects=effects,
             outcome=V(tag) if raw['accepted'] else V('Rejected',V('CoreRejected',raw['error'])),
             coreProjection=projection(before['program'],raw),effectEvidence=V('EvidenceValid'),display=display)
