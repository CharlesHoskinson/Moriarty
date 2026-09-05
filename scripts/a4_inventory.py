from scripts.a4_carrier import M,V,r,change,require
from scripts.a4_agreement import oracle,ACCOUNTS
from scripts.a4_authority import gate,apply,a_execution,plan_valid,financial,cancel_allowed,consume_allowed,GOOD,PARENT_KEY
from scripts.a4_cases import (ALICE,BOB,AA,PROFILES,I_SCENARIOS,S_SCENARIOS,initial,parameters,ordinary,materialize,
    policy_for,expected_observation,evidence_for,environment,key)

I_CONTROLS=(('no-cancel','recover-r0-choice2'),('unsigned','recover-r0-choice2'),('old-nonce','recover-r0-choice2'),
            ('fresh-duplicate-cancel','recover-r0-choice2'),('unused-successor','two-fills'))
S_CONTROLS=('stale-signing','unused-successor','reversed-effects','reductions','neutral-chooser','second-plan','wrong-Core-chooser','wrong-signer','wrong-nonce','stale-facts')
OBS_CONTROLS=('unused-successor','reversed-effects','reductions','neutral-chooser')

def descriptors():
    out=[]
    for loop,scenarios in (('installment',I_SCENARIOS),('swap',S_SCENARIOS)):
        families=[(s,'ordinary') for s in scenarios]
        if loop=='swap': families.append(('funded2-settle','verified-stale'))
        families += [(s,c) for c,s in I_CONTROLS] if loop=='installment' else [('funded2-settle',c) for c in S_CONTROLS]
        for scenario,control in families:
            for profile in PROFILES:
                out.append(dict(case_id='/'.join((loop,scenario,control,profile)),lifecycle=loop,profile=profile,scenario=scenario,control=control))
    return tuple(out)

def legacy(loop,step):
    tag=step[0]
    if loop=='installment':
        if tag in ('Prepare','Sign'): return V(tag+('RecoveryI' if step[1]=='RecoveryAttempt' else 'ParentI'))
        if tag=='Advance': return V('AdvanceI')
        if tag=='RejectProposed': return V('RejectRecoveryI')
        return V(('RejectStale' if tag=='RejectVerified' else tag)+'I',V(step[1]))
    if tag in ('Prepare','Sign'): return V(tag+'S',r(id=V(step[1]),principal=V(step[2])))
    return V(tag+'S',V('Time'+str(step[1])) if tag=='Advance' else V(step[1]))

def route_guard(s,loop,step,cmd):
    if not gate(s,cmd): return False
    if step[0]=='Propose' and not a_execution(s): return False
    if loop=='installment':
        parent=s['authority']['context']['parents'][PARENT_KEY]
        cancelled=parent.tag=='ParentLive' and parent.value['entry']['cancelled']
        if step[0]=='Advance': return cancelled and s['authority']['context']['environment']['physicalTime']==2
        if step[0] in ('Prepare','Sign') and step[1]=='RecoveryAttempt': return cancelled
        if step[0]=='Commit' and step[1] in ('FirstFillAttempt','CancelAttempt'):
            a=s['attempts'][V('FirstFillAttempt')]; b=s['attempts'][V('CancelAttempt')]
            return a.tag==b.tag=='VerifiedOperation' and a.value['attempt']['context']==b.value['attempt']['context']==s['authority']['context']
    return True

def mutate_observation(o,name):
    if name=='unused-successor':
        p=o['proposedSuccessor']['program']; nodes=p['nodes'].put(V('N15'),V('PayA',r(account=AA,payee=BOB,amount=V('ConstantA',5),continuation=V('N0'))))
        return change(o,proposedSuccessor=change(o['proposedSuccessor'],program=change(p,nodes=nodes)))
    if name=='reversed-effects': return change(o,effects=tuple(reversed(o['effects'])))
    if name=='reductions': return change(o,coreProjection=V('CoreProjected',change(o['coreProjection'].value,reductions=o['coreProjection'].value['reductions']+1)))
    if name=='neutral-chooser': return change(o,input=V('ChoiceLike',r(id='settle',chooser=ALICE,chosen=1)))
    if name=='stale-facts':
        plan=o['resolvedPlan']; x=plan['operations'][0]
        x=change(x,predecessorFacts=change(x['predecessorFacts'],environment=environment(100)))
        return change(o,resolvedPlan=change(plan,operations=(x,)))
    raise ValueError(name)

def computation(request):
    raw,effects=oracle(request)
    return r(request=request,evaluation=V('TransactionComputedA',raw),extraction=V('ExtractionObservedA4',V('EffectsExtractedA',effects)))

def call_computations(calls):
    return tuple(computation(c.value) for c in calls if c.tag=='AgreementCallA')

def computations(s,cmd):
    x=cmd.value; tag=cmd.tag
    if tag in ('Prepare','Sign'):
        b=x['policy']['binding']
        return call_computations(tuple(p['artifactAndCall'] for p in b.value['identity']['operations'])) if b.tag=='AfterResolution' else ()
    if tag=='Propose': return call_computations((x['observation']['artifactAndCall'],))
    if tag in ('Verify','Commit','RejectProposed','RejectVerified'):
        cell=s['attempts'][x['id']]
        if cell.tag=='NoAttempt': return ()
        a=cell.value if cell.tag=='ProposedAttempt' else cell.value['attempt']
        return call_computations((a['observation']['artifactAndCall'],))
    if tag=='Derive': return call_computations((x['attempt']['observation']['artifactAndCall'],))
    if tag=='CoreAcceptedProbe': return (computation(x['request']),)
    if tag=='PlanMatchesProbe': return call_computations(tuple(p['artifactAndCall'] for p in x['plan']['operations']))
    return ()

def terminal(s):
    c=s['authority']['context']
    return c['candidate']['state']['continuation']==V('N0') and all(c['ledger'][(V('Escrow',a),a['asset'])]==0 for a in ACCOUNTS) and all(x.tag not in ('ProposedAttempt','VerifiedOperation') for _,x in s['attempts'].pairs) and all(x.tag!='PreparedSigning' for _,x in s['authority']['signing'].pairs)

def history(d):
    loop=d['lifecycle']; scenario=d['scenario']; profile=d['profile']; control=d['control']; s=initial(loop); events=[]
    desc=r(caseId=d['case_id'],lifecycle=loop,profile=profile,scenario=scenario,control=control)
    def emit(kind,cmd,guard,observed,after=None):
        nonlocal s
        require(type(observed) is bool,'guard result')
        result=s if after is None else after
        public_tag={'CanCancel':'CancelParentProbeA4','CanConsume':'ConsumeSlotProbeA4','FinancialGuard':'FinancialProbeA4'}.get(cmd.tag,cmd.tag+'A4')
        latest=r(caseId=d['case_id'],profile=profile,sequence=len(events),kind=kind,
            arguments=r(guard=guard,command=V(public_tag,cmd.value)),observedGuard=observed,computations=computations(s,cmd))
        events.append(r(latest=latest,before=s,after=result)); s=result
    def transition(step,lifecycle=True,override=None):
        cmd=materialize(s,loop,scenario,profile,step) if override is None else override
        ready=route_guard(s,loop,step,cmd) if lifecycle else gate(s,cmd)
        require(ready,'required transition denied '+repr(step))
        guard=V('InstallmentCommandGuardA4' if loop=='installment' else 'SwapCommandGuardA4',legacy(loop,step)) if lifecycle else V(cmd.tag+'GuardA4')
        emit('transition',cmd,guard,ready,apply(s,cmd))
    def denied(step,lifecycle=False,override=None):
        cmd=materialize(s,loop,scenario,profile,step) if override is None else override
        if cmd.tag=='CoreAcceptedProbe': ready=oracle(cmd.value['request'])[0]['accepted']; g='CoreAcceptedGuardA4'
        elif cmd.tag=='PlanMatchesProbe': ready=plan_valid(cmd.value['plan']); g='PlanMatchesGuardA4'
        else:
            ready=route_guard(s,loop,step,cmd) if lifecycle else gate(s,cmd)
            g={'CanCancel':'CancelParentGuardA4','CanConsume':'ConsumeSlotGuardA4','FinancialGuard':'FinancialGuardA4'}.get(cmd.tag,cmd.tag+'GuardA4')
        require(not ready,'required negative guard unexpectedly permits '+repr(step))
        guard=V('InstallmentCommandGuardA4' if loop=='installment' else 'SwapCommandGuardA4',legacy(loop,step)) if lifecycle else V(g)
        emit('denied-probe',cmd,guard,False)
    def route(count):
        for st in ordinary(loop,scenario)[:count]: transition(st)
    def rejection(id,e): transition(('RejectProposed',id),False,V('RejectProposed',r(id=V(id),evidence=e)))
    def denied_verify(id,e): denied(('Verify',id),override=V('Verify',r(id=V(id),evidence=e)))
    def derive(name,base_sequence,base_attempt,stage):
        a=change(base_attempt,observation=mutate_observation(base_attempt['observation'],name)); e=evidence_for(a,loop,scenario,profile)
        cmd=V('Derive',r(mutationId=name,baseCaseId=d['case_id'],baseSequence=base_sequence,attempt=a,evidence=e,
            stage=V('ProposedDerivationA4' if stage=='proposed' else 'VerifiedDerivationA4')))
        cell=V('ProposedAttempt',a) if stage=='proposed' else V('VerifiedOperation',r(attempt=a,evidence=e))
        emit('adversarial-derivation',cmd,V('NoGuardA4'),True,change(s,attempts=s['attempts'].put(a['id'],cell)))
        return a,e
    emit('case-start',V('CaseStart',desc),V('NoGuardA4'),True)
    if control=='ordinary':
        route(len(ordinary(loop,scenario)))
        ids=[step[1] for step in ordinary(loop,scenario) if step[0]=='Commit']
        if loop=='swap' and 'DispositionAttempt' not in ids: ids.append('DispositionAttempt')
        for id in ids: denied(('Propose',id)); denied(('Commit',id))
        parent=s['authority']['context']['parents'][PARENT_KEY]
        if loop=='installment' and parent.tag=='ParentLive' and parent.value['entry']['cancelled']:
            x=parent.value; args=r(parent=x['parent'],entry=x['entry'],prepared=x['entry'])
            denied(('CanCancel',),override=V('CanCancel',args))
            for slot in (1,2): denied(('CanConsume',),override=V('CanConsume',change(args,slot=slot)))
    elif control=='verified-stale':
        route(17); transition(('Advance',100)); denied(('Commit','DispositionAttempt')); transition(('RejectVerified','DispositionAttempt'))
    elif loop=='installment' and control in ('no-cancel','unsigned','old-nonce','fresh-duplicate-cancel'):
        route(2 if control=='no-cancel' else 8)
        id='FreshCancelAttempt' if control=='fresh-duplicate-cancel' else 'RecoveryAttempt'
        if control=='no-cancel':
            denied(('Prepare','RecoveryAttempt','Alice'),True)
            obs=expected_observation(loop,scenario,id)
            denied(('FinancialGuard',),override=V('FinancialGuard',r(operation=V('OpRecover',r(parent=PARENT_KEY,recovery=change(PARENT_KEY,nonce=1))),effects=obs['effects'])))
        cmd=materialize(s,loop,scenario,profile,('Propose',id))
        if control=='fresh-duplicate-cancel': cmd=V('Propose',change(cmd.value,observation=expected_observation(loop,scenario,'CancelAttempt')))
        transition(('Propose',id),False,cmd)
        a=s['attempts'][V(id)].value; e=evidence_for(a,loop,scenario,profile)
        if control=='old-nonce':
            signed=r(policy=policy_for(loop,scenario,profile,'FirstFillAttempt'),signer=ALICE,token=0)
            e=change(e,signatures=M(((PARENT_KEY,r(attempt=a,disposition=GOOD,signed=signed)),)))
        denied_verify(id,e); rejection(id,e)
        if control=='unsigned': denied(('Prepare','FirstFillAttempt','Alice'))
    elif control in OBS_CONTROLS or control=='stale-facts':
        count=4 if loop=='installment' else 16; route(count)
        id='FirstFillAttempt' if loop=='installment' else 'DispositionAttempt'; original=s['attempts'][V(id)].value
        base_sequence=3 if loop=='installment' else 16
        a,e=derive(control,base_sequence,original,'proposed'); denied_verify(id,e); rejection(id,e)
        if control!='stale-facts':
            a,e=derive(control,base_sequence,original,'verified'); denied(('Commit',id)); transition(('RejectVerified',id),False)
    elif control=='stale-signing':
        route(11); transition(('Prepare','DispositionAttempt','Alice')); transition(('Advance',100)); denied(('Sign','DispositionAttempt','Alice'))
    elif control=='wrong-Core-chooser':
        route(11)
        from scripts.a4_cases import request_for,choice
        req=request_for(loop,scenario,'DispositionAttempt').value
        req=change(req,input=choice('SettleId',ALICE,1))
        denied(('CoreAcceptedProbe',),override=V('CoreAcceptedProbe',r(request=req)))
    elif control in ('wrong-signer','wrong-nonce'):
        route(16); id='DispositionAttempt'; a=s['attempts'][V(id)].value; e=evidence_for(a,loop,scenario,profile); k=key(loop,ALICE,1); proof=e['signatures'][k]
        if control=='wrong-signer': signatures=e['signatures'].put(k,change(proof,signed=change(proof['signed'],signer=V('Mallory'))))
        else:
            signed=r(policy=policy_for(loop,scenario,profile,'FundingOneAttempt'),signer=ALICE,token=0)
            signatures=M(tuple((j,v) for j,v in e['signatures'].pairs if j!=k)).put(key(loop,ALICE,0),change(proof,signed=signed))
        e=change(e,signatures=signatures); denied_verify(id,e); rejection(id,e)
    elif control=='second-plan':
        id='FundingOneAttempt'; policy=policy_for(loop,scenario,profile,id); obs=expected_observation(loop,scenario,id); original=obs['resolvedPlan']['operations'][0]
        badplan=change(obs['resolvedPlan'],operations=(original,change(original,proposedSuccessor=initial(loop)['authority']['context']['candidate'])))
        denied(('PlanMatchesProbe',),override=V('PlanMatchesProbe',r(plan=badplan)))
        badpolicy=change(policy,binding=V('AfterResolution',r(identity=badplan,operations=badplan['operations']))) if profile=='SignAfterResolve' else policy
        prep=V('Prepare',r(policy=badpolicy,signer=ALICE))
        if profile=='SignAfterResolve': denied(('Prepare',id,'Alice'),override=prep); transition(('Prepare',id,'Alice'),False)
        else: transition(('Prepare',id,'Alice'),False,prep)
        transition(('Sign',id,'Alice'),False)
        cmd=materialize(s,loop,scenario,profile,('Propose',id)); cmd=V('Propose',change(cmd.value,observation=change(obs,resolvedPlan=badplan)))
        transition(('Propose',id),False,cmd); a=s['attempts'][V(id)].value; e=evidence_for(a,loop,scenario,profile)
        denied_verify(id,e); rejection(id,e)
    else: raise ValueError('unknown fixed control '+control)
    status=('financial-terminal' if terminal(s) else 'refusal-terminal') if control=='ordinary' else 'negative-complete'
    emit('case-end',V('CaseEnd',r(status=status)),V('NoGuardA4'),True)
    return tuple(events)
