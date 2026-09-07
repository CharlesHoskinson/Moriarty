from scripts.a4_carrier import M,V,r,change,require
from scripts.a4_agreement import core,PARTIES,ASSETS,ACCOUNTS,CHOICES,account,party,oracle,observation
from scripts.a4_authority import KEYS,IDS,LEDGER_KEYS,PARENT_KEY,GOOD,needed,facts,gate,apply

ALICE=V('Alice'); BOB=V('Bob'); AA=r(owner=ALICE,asset=V('TokenA')); BB=r(owner=BOB,asset=V('TokenB'))
PROFILES=('SignAfterResolve','SignBeforeResolve')
MODES=('choice2','timeout100','timeout101','refuse100','refuse101')
I_SCENARIOS=('two-fills',)+tuple(f'recover-r{n}-{m}' for n in (0,1) for m in MODES)
S_SCENARIOS=('funded2-settle','funded2-refund')+tuple(f'funded{n}-timeout{t}' for n in (0,1,2) for t in (100,101))+tuple(f'funded2-refuse{t}-choice{c}' for t in (100,101) for c in (0,1))

def program(loop):
    table,root=core._fixture_nodes('installment-two-when-v1' if loop=='installment' else 'canonical-swap-v1')
    def node_id(x): return V(core._exact_node(x,table,'fixture'))
    def choice_id(x): return V(next(k for k,v in core.CHOICES.items() if v==x))
    def action(x):
        if isinstance(x,core.Deposit): return V('DepositA',r(account=account(x.account),depositor=party(x.depositor),amount=V('ConstantA',x.amount.quantity)))
        return V('ChoiceA',r(id=choice_id(x.choice_id),chooser=party(x.chooser),lower=x.lower_bound,upper=x.upper_bound))
    def node(x):
        if isinstance(x,core.Close): return V('CloseA')
        if isinstance(x,core.Pay): return V('PayA',r(account=account(x.account),payee=party(x.payee),amount=V('ConstantA',x.amount.quantity),continuation=node_id(x.continuation)))
        if isinstance(x,core.If): return V('IfA',r(observation=V('ChoiceEqualsA',r(id=choice_id(x.observation.choice_id),expected=x.observation.expected)),thenNode=node_id(x.then_contract),elseNode=node_id(x.else_contract)))
        return V('WhenA',r(cases=tuple(r(caseAction=action(c.action),continuation=node_id(c.continuation)) for c in x.cases),timeout=V('Time'+str(x.timeout)),timeoutNode=node_id(x.timeout_continuation)))
    return r(root=V(root),nodes=M(tuple((V(k),node(v)) for k,v in table.items())))

def environment(t): return r(physicalTime=t,anchor=0,implementationVersion=0,enforcementMechanism=0)
def key(loop,p,n): return r(domain=V('InstallmentDomain' if loop=='installment' else 'SwapDomain'),principal=p,nonce=n)
def before(loop,n):
    amounts={AA:(10 if loop=='installment' or n>=1 else 0),BB:(20 if loop=='swap' and n==2 else 0)}
    if loop=='installment': amounts[AA]=10-5*n
    cs=M(tuple((c,V('NoInt')) for c in CHOICES))
    if loop=='installment' and n: cs=cs.put(V('FirstFillId'),V('IntValue',1))
    if loop=='installment' and n==2: cs=cs.put(V('SecondFillId'),V('IntValue',1))
    return r(program=program(loop),state=r(accounts=M(tuple((a,amounts.get(a,0)) for a in ACCOUNTS)),choices=cs,
        continuation=V(('N4','N2','N0')[n] if loop=='installment' else ('N6','N5','N4')[n]),minimumTime=V('Time2' if loop=='installment' else 'Time'+str(n))))
def ledger(loop,n):
    balances={(V('Escrow',AA),V('TokenA')):10-5*n,(V('Wallet',BOB),V('TokenA')):5*n} if loop=='installment' else {
        (V('Wallet',ALICE),V('TokenA')):10 if n==0 else 0,(V('Wallet',BOB),V('TokenB')):20 if n<2 else 0,
        (V('Escrow',AA),V('TokenA')):10 if n>=1 else 0,(V('Escrow',BB),V('TokenB')):20 if n==2 else 0}
    return M(tuple((k,balances.get(k,0)) for k in LEDGER_KEYS))
def initial(loop):
    return r(authority=r(context=r(candidate=before(loop,0),ledger=ledger(loop,0),environment=environment(2 if loop=='installment' else 1),
        registry=M(tuple((k,V('AuthorityUnused')) for k in KEYS)),parents=M(tuple((k,V('ParentVacant')) for k in KEYS))),
        signing=M(tuple((k,V('NoSigningCheck')) for k in KEYS))),attempts=M(tuple((i,V('NoAttempt')) for i in IDS)))
def parameters(loop,scenario):
    if loop=='installment':
        require(scenario in I_SCENARIOS,'installment scenario')
        residual=scenario=='two-fills' or scenario.startswith('recover-r1-')
        mode='choice2' if scenario=='two-fills' else scenario.split('-',2)[2]
        return dict(residual=residual,mode=mode,time=2 if mode=='choice2' else int(mode[-3:]),refused=mode.startswith('refuse'))
    require(scenario in S_SCENARIOS,'swap scenario')
    pieces=scenario.split('-'); mode=pieces[1]
    return dict(funded=int(pieces[0][-1]),mode=mode,time=2 if mode in ('settle','refund') else int(mode[-3:]),
        refused=mode.startswith('refuse'),chosen=1 if mode=='settle' else 0 if mode=='refund' else int(pieces[2][-1]) if len(pieces)==3 else None)
def choice(id,actor,n): return V('PresentAInput',V('ChoiceInputA',r(id=V(id),chooser=actor,chosen=n)))
def request_for(loop,scenario,id):
    p=parameters(loop,scenario)
    if loop=='installment':
        residual=id in ('SecondFillAttempt','FreshCancelAttempt') or (id=='RecoveryAttempt' and p['residual'])
        b=before(loop,int(residual)); now=p['time'] if id=='RecoveryAttempt' else 2
        if id in ('CancelAttempt','FreshCancelAttempt'): return V('CancellationCallA',r(before=b,now=V('Time2')))
        supplied=choice('RecoveryId',ALICE,1) if id=='RecoveryAttempt' else choice('FirstFillId' if id=='FirstFillAttempt' else 'SecondFillId',BOB,1)
        if id=='RecoveryAttempt' and p['mode'].startswith('timeout'): supplied=V('NoAInput')
    else:
        n=0 if id=='FundingOneAttempt' else 1 if id=='FundingTwoAttempt' else p['funded']; b=before(loop,n)
        now=n+1 if id!='DispositionAttempt' else p['time']
        if id!='DispositionAttempt':
            a=AA if n==0 else BB; supplied=V('PresentAInput',V('DepositInputA',r(account=a,depositor=a['owner'],quantity=10 if n==0 else 20)))
        else: supplied=V('NoAInput') if p['chosen'] is None else choice('SettleId',BOB,p['chosen'])
    return V('AgreementCallA',r(before=b,input=supplied,now=V('Time'+str(now))))
def operation_for(loop,scenario,id):
    if loop=='installment':
        if id in ('CancelAttempt','FreshCancelAttempt'): return V('OpCancelParent',PARENT_KEY)
        if id=='RecoveryAttempt': return V('OpRecover',r(parent=PARENT_KEY,recovery=change(PARENT_KEY,nonce=1)))
        return V('OpFillSlot',r(parent=PARENT_KEY,slot=1 if id=='FirstFillAttempt' else 2))
    p=parameters(loop,scenario)
    return V('OpFund' if id!='DispositionAttempt' else 'OpDeadlineRefund' if p['chosen'] is None else 'OpSettle' if p['chosen']==1 else 'OpVoluntaryRefund')
def planned(loop,scenario,id):
    call=request_for(loop,scenario,id); op=operation_for(loop,scenario,id); p=parameters(loop,scenario)
    n=(int(p['residual']) if id=='RecoveryAttempt' else int(id in ('SecondFillAttempt','FreshCancelAttempt'))) if loop=='installment' else (0 if id=='FundingOneAttempt' else 1 if id=='FundingTwoAttempt' else p['funded'])
    parents=M(tuple((k,V('ParentAbsent')) for k in KEYS))
    if loop=='installment': parents=parents.put(PARENT_KEY,V('ParentPresent',r(cancelled=id=='RecoveryAttempt',usedSlots=frozenset((1,)) if n else frozenset(),paid=5*n,remainingAllowance=10-5*n,revision=n+int(id=='RecoveryAttempt'))))
    f=r(ledger=ledger(loop,n),environment=environment(int(call.value['now'].tag[4:])),parents=parents)
    dummy=r(identity=V('SwapPlanA'),operations=())
    o=observation(op,call,dummy,V('PublicDisplay')); require(o is not None,'fixture Core evaluation')
    return r(predecessor=o['predecessor'],proposedSuccessor=o['proposedSuccessor'],artifactAndCall=call,input=o['input'],operation=op,
        transactionTime=o['transactionTime'],effects=o['effects'],coreProjection=o['coreProjection'],predecessorFacts=f)
def plan_for(loop,scenario,id):
    if loop=='installment' and id!='RecoveryAttempt': return r(identity=V('InstallmentPlanA'),operations=tuple(planned(loop,scenario,i) for i in ('FirstFillAttempt','SecondFillAttempt','CancelAttempt','FreshCancelAttempt')))
    return r(identity=V('RecoveryPlanA' if loop=='installment' else 'SwapPlanA'),operations=(planned(loop,scenario,id),))
def expected_observation(loop,scenario,id):
    return observation(operation_for(loop,scenario,id),request_for(loop,scenario,id),plan_for(loop,scenario,id),V('PublicDisplay'))
def policy_for(loop,scenario,profile,id,principal=ALICE):
    plan=plan_for(loop,scenario,id); clauses=[]
    for x in plan['operations']:
        conditions=(V('InputIs',x['input']),)
        if loop=='installment': conditions+=(V('ParentMatches',r(key=PARENT_KEY,expected=x['predecessorFacts']['parents'][PARENT_KEY])),V('SourceBalanceIs',r(location=V('Escrow',AA),asset=V('TokenA'),quantity=x['predecessorFacts']['ledger'][(V('Escrow',AA),V('TokenA'))])))
        clauses.append(r(operation=x['operation'],conditions=conditions,requiredEffects=x['effects'],allowedEffects=x['effects'],effectOrder=V('ExactOrder')))
    parent=loop=='installment' and id!='RecoveryAttempt'; funding=loop=='swap' and id!='DispositionAttempt'
    caps=('FirstFillCapability','SecondFillCapability','CancelCapability') if parent else ('RecoveryCapability',) if loop=='installment' else ('FundCapability',) if funding else ('DisposeCapability',)
    t=plan['operations'][0]['transactionTime']
    location=V('Wallet',principal) if funding else V('Escrow',AA if principal==ALICE else BB)
    b=r(key=key(loop,principal,0 if parent or funding else 1),clauses=tuple(clauses),debitLocations=frozenset((location,)),capabilities=frozenset(V(c) for c in caps),
        disclosures=frozenset(),validFrom=2 if parent else t,validUntil=99 if parent else t,implementationVersion=0,enforcementMechanism=0)
    return r(body=b,profile=V(profile),binding=V('AfterResolution',r(identity=plan,operations=plan['operations'])) if profile=='SignAfterResolve' else V('BeforeResolution',V('AnyArtifactUnderMechanism')))
def evidence_for(a,loop,scenario,profile):
    return r(effect=r(attempt=a,disposition=GOOD),signatures=M(tuple((k,r(attempt=a,disposition=GOOD,signed=r(policy=policy_for(loop,scenario,profile,a['id'].tag,k['principal']),signer=k['principal'],token=0))) for k in needed(a['operation'],a['observation']['effects']))))
def ordinary(loop,scenario):
    p=parameters(loop,scenario)
    if loop=='installment':
        first='FirstFillAttempt'; cancel='CancelAttempt'; recovery='RecoveryAttempt'
        steps=[('Prepare',first,'Alice'),('Sign',first,'Alice'),('Propose',first),('Propose',cancel),('Verify',first),('Verify',cancel),
            ('Commit',first if p['residual'] else cancel),('RejectVerified',cancel if p['residual'] else first)]
        if scenario=='two-fills': return tuple(steps+[('Propose','SecondFillAttempt'),('Verify','SecondFillAttempt'),('Commit','SecondFillAttempt')])
        if p['residual']: steps += [('Propose','FreshCancelAttempt'),('Verify','FreshCancelAttempt'),('Commit','FreshCancelAttempt')]
        if p['time']!=2: steps += [('Advance',p['time'])]
        steps += [('Prepare',recovery,'Alice'),('Sign',recovery,'Alice'),('Propose',recovery)]
        steps += [('RejectProposed',recovery)] if p['refused'] else [('Verify',recovery),('Commit',recovery)]
        return tuple(steps)
    steps=[]
    for n in range(p['funded']):
        id=('FundingOneAttempt','FundingTwoAttempt')[n]; actor=('Alice','Bob')[n]
        if n: steps.append(('Advance',2))
        steps += [('Prepare',id,actor),('Sign',id,actor),('Propose',id),('Verify',id),('Commit',id)]
    if p['time']!= (1 if p['funded']==0 else p['funded']): steps.append(('Advance',p['time']))
    id='DispositionAttempt'
    if not p['refused']:
        for actor in ('Alice','Bob')[:p['funded']]: steps += [('Prepare',id,actor),('Sign',id,actor)]
    steps.append(('Propose',id))
    steps += [('RejectProposed',id)] if p['refused'] or not p['funded'] else [('Verify',id),('Commit',id)]
    return tuple(steps)
def materialize(s,loop,scenario,profile,step):
    tag=step[0]
    if tag=='Advance': return V(tag,r(environment=environment(step[1])))
    id=step[1]
    if tag in ('Prepare','Sign'):
        args=r(policy=policy_for(loop,scenario,profile,id,V(step[2])),signer=V(step[2]))
        return V(tag,change(args,token=0) if tag=='Sign' else args)
    if tag=='Propose':
        cell=s['attempts'][V(id)]
        if cell.tag=='NoAttempt':
            actor=BOB if id in ('FirstFillAttempt','SecondFillAttempt','FundingTwoAttempt','DispositionAttempt') else ALICE
            return V(tag,r(id=V(id),operation=operation_for(loop,scenario,id),observation=expected_observation(loop,scenario,id),actor=actor))
        a=cell.value['attempt'] if cell.tag in ('VerifiedOperation','ExecutedOperation','RejectedOperation') else cell.value
        return V(tag,r(id=a['id'],operation=a['operation'],observation=a['observation'],actor=a['actor']))
    if tag in ('Verify','RejectProposed'):
        a=s['attempts'][V(id)].value
        return V(tag,r(id=V(id),evidence=evidence_for(a,loop,scenario,profile)))
    return V(tag,r(id=V(id)))
def prefix(loop,scenario,profile,count):
    s=initial(loop)
    for step in ordinary(loop,scenario)[:count]:
        cmd=materialize(s,loop,scenario,profile,step); require(gate(s,cmd),'independent ordinary route guard'); s=apply(s,cmd)
    return s
