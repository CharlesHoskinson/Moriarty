from collections import Counter
from scripts.a4_carrier import R,M,V,r,change,typed,require,Invalid
from scripts.a4_agreement import PARTIES,ASSETS,ACCOUNTS,valid_before,observation

KEYS=frozenset(r(domain=V(d),principal=p,nonce=n) for d in ('SwapDomain','InstallmentDomain') for p in PARTIES for n in (0,1))
IDS=frozenset(V(x) for x in ('FundingOneAttempt','FundingTwoAttempt','DispositionAttempt','FirstFillAttempt','SecondFillAttempt','CancelAttempt','FreshCancelAttempt','RecoveryAttempt'))
LOCS=frozenset(V('Wallet',p) for p in PARTIES)|frozenset(V('Escrow',a) for a in ACCOUNTS)
LEDGER_KEYS=frozenset((loc,asset) for loc in LOCS for asset in ASSETS)
GOOD=V('EvidenceValid'); UNUSED=V('AuthorityUnused')
PARENT_KEY=r(domain=V('InstallmentDomain'),principal=V('Alice'),nonce=0)

def balance(ledger,loc,asset): return ledger.get((loc,asset),0)
def owner(loc): return loc.value if loc.tag=='Wallet' else loc.value['owner']
def accepts(loc,asset): return loc.tag=='Wallet' or loc.value['asset']==asset
def valid_ledger(l):
    return l.keys()==LEDGER_KEYS and all(q>=0 and (accepts(loc,a) or q==0) for (loc,a),q in l.pairs)
def valid_effect(e):
    return e['quantity']>0 and e['source']!=e['destination'] and accepts(e['source'],e['asset']) and accepts(e['destination'],e['asset'])
def transfers(l,es):
    for e in es:
        src=(e['source'],e['asset']); dst=(e['destination'],e['asset']); q=e['quantity']
        l=l.put(src,l.get(src,0)-q); l=l.put(dst,l.get(dst,0)+q)
    return l
def can_transfer(l,es):
    if not valid_ledger(l): return False
    for e in es:
        if not valid_effect(e) or balance(l,e['source'],e['asset'])<e['quantity']: return False
        l=transfers(l,(e,))
    return True
def valid_env(e):
    return e['physicalTime'] in (1,2,100,101) and e['anchor'] in (0,1,2) and e['implementationVersion'] in (0,1) and e['enforcementMechanism'] in (0,1)
def facts_valid(f): return valid_ledger(f['ledger']) and valid_env(f['environment']) and f['parents'].keys()==KEYS
def coupling(c):
    b=c['candidate']; l=c['ledger']
    return valid_before(b) and valid_ledger(l) and all(b['state']['accounts'][a]==l[(V('Escrow',a),a['asset'])] for a in ACCOUNTS)
def op_parents(op):
    if op.tag=='OpCancelParent': return frozenset((op.value,))
    if op.tag in ('OpFillSlot','OpRecover'): return frozenset((op.value['parent'],))
    return frozenset()
def valid_op(op):
    ps=op_parents(op)
    if ps and not all(k in KEYS and k['domain']==V('InstallmentDomain') and k['nonce']==0 for k in ps): return False
    if op.tag=='OpFillSlot': return op.value['slot'] in (1,2)
    if op.tag=='OpRecover': return op.value['recovery']==change(op.value['parent'],nonce=1)
    return True
def capability(op):
    if op.tag=='OpFillSlot': return V('FirstFillCapability' if op.value['slot']==1 else 'SecondFillCapability')
    return V({'OpFund':'FundCapability','OpCancelParent':'CancelCapability','OpRecover':'RecoveryCapability'}.get(op.tag,'DisposeCapability'))
def key_allows(key,op):
    if key not in KEYS or not valid_op(op): return False
    if op.tag=='OpFillSlot': return key==op.value['parent']
    if op.tag=='OpCancelParent': return key==op.value
    if op.tag=='OpRecover': return key==op.value['recovery']
    return key['domain']==V('SwapDomain') and key['nonce']==(0 if op.tag=='OpFund' else 1)
def subbag(a,b): return all(n<=Counter(b)[x] for x,n in Counter(a).items())
def valid_clause(c):
    return valid_op(c['operation']) and all(valid_effect(e) for e in c['requiredEffects']+c['allowedEffects']) and subbag(c['requiredEffects'],c['allowedEffects'])
def valid_body(b):
    return b['key'] in KEYS and 0<=b['validFrom']<=b['validUntil'] and b['implementationVersion'] in (0,1) and b['enforcementMechanism'] in (0,1) and bool(b['clauses']) and all(owner(l)==b['key']['principal'] for l in b['debitLocations']) and all(valid_clause(c) and key_allows(b['key'],c['operation']) and capability(c['operation']) in b['capabilities'] for c in b['clauses'])
def condition(c,inp,t,f):
    x=c.value
    if c.tag=='InputIs': return x==inp
    if c.tag=='BeforeTime': return t<x
    if c.tag=='AtOrAfterTime': return t>=x
    if c.tag=='ParentMatches': return x['key'] in f['parents'].keys() and f['parents'][x['key']]==x['expected']
    return x['quantity']>=0 and balance(f['ledger'],x['location'],x['asset'])==x['quantity']
def body_allows(b,op,inp,t,es,f):
    if not (valid_body(b) and facts_valid(f) and key_allows(b['key'],op) and capability(op) in b['capabilities']): return False
    env=f['environment']
    if t!=env['physicalTime'] or not b['validFrom']<=t<=b['validUntil'] or b['implementationVersion']!=env['implementationVersion'] or b['enforcementMechanism']!=env['enforcementMechanism']: return False
    if not can_transfer(f['ledger'],es) or not all(owner(e['source'])!=b['key']['principal'] or e['source'] in b['debitLocations'] for e in es): return False
    return any(c['operation']==op and all(condition(x,inp,t,f) for x in c['conditions']) and subbag(c['requiredEffects'],es) and subbag(es,c['allowedEffects']) and (c['effectOrder']==V('AnyOrder') or es==c['allowedEffects']) for c in b['clauses'])
def common_policy(p):
    if not valid_body(p['body']): return False
    bind=p['binding']
    if bind.tag=='BeforeResolution': return p['profile']==V('SignBeforeResolve')
    ops=bind.value['operations']
    return p['profile']==V('SignAfterResolve') and bool(ops) and all(body_allows(p['body'],x['operation'],x['input'],x['transactionTime'],x['effects'],x['predecessorFacts']) for x in ops)
def planned_matches(x,op,o,f):
    return x==r(predecessor=o['predecessor'],proposedSuccessor=o['proposedSuccessor'],artifactAndCall=o['artifactAndCall'],input=o['input'],operation=op,transactionTime=o['transactionTime'],effects=o['effects'],coreProjection=o['coreProjection'],predecessorFacts=f)
def plan_valid(plan):
    if not 1<=len(plan['operations'])<=4: return False
    for x in plan['operations']:
        f=x['predecessorFacts']
        if not (valid_op(x['operation']) and facts_valid(f) and coupling(r(candidate=x['predecessor'],ledger=f['ledger'])) and x['transactionTime']==f['environment']['physicalTime']): return False
        o=observation(x['operation'],x['artifactAndCall'],plan,V('PublicDisplay'))
        if o is None or not planned_matches(x,x['operation'],o,f): return False
    return True
def a_policy(p):
    return common_policy(p) and (p['binding'].tag=='BeforeResolution' or (p['binding'].value['operations']==p['binding'].value['identity']['operations'] and plan_valid(p['binding'].value['identity'])))
def binding(p,op,o,f):
    b=p['binding']
    if b.tag=='BeforeResolution': return b.value.tag=='AnyArtifactUnderMechanism' or b.value.value==o['artifactAndCall']
    return b.value['identity']==o['resolvedPlan'] and any(planned_matches(x,op,o,f) for x in b.value['operations'])
def policy_allows(p,op,o,f):
    return common_policy(p) and body_allows(p['body'],op,o['input'],o['transactionTime'],o['effects'],f) and binding(p,op,o,f)
def signed_valid(s): return common_policy(s['policy']) and s['signer']==s['policy']['body']['key']['principal'] and s['token'] in (0,1)
def is_parent(key): return key['domain']==V('InstallmentDomain') and key['nonce']==0
def parent_for(signed):
    return r(key=signed['policy']['body']['key'],policy=signed,source=V('Escrow',r(owner=V('Alice'),asset=V('TokenA'))),recipient=V('Bob'),asset=V('TokenA'),budget=10,slots=M(((1,5),(2,5))))
def initial_entry(parent): return r(claim=V('Unclaimed'),usedSlots=frozenset(),paid=0,remainingAllowance=parent['budget'],cancelled=False,revision=0)
def parent_valid(p): return p==change(parent_for(p['policy']),key=PARENT_KEY) and p['key']==PARENT_KEY
def entry_valid(p,e):
    if not parent_valid(p): return False
    if e['claim']==V('Unclaimed'): claim=e==initial_entry(p)
    else: claim=e['claim']==V('Claimed',p) and (bool(e['usedSlots']) or e['cancelled'])
    slots=e['usedSlots']
    return claim and slots<=frozenset((1,2)) and (2 not in slots or 1 in slots) and e['paid']==sum(p['slots'].get(i,0) for i in slots) and e['remainingAllowance']==p['budget']-e['paid'] and e['remainingAllowance']>=0 and e['revision']==len(slots)+int(e['cancelled']) and (not e['cancelled'] or e['remainingAllowance']>0)
def context_valid(c):
    if c['registry'].keys()!=KEYS or c['parents'].keys()!=KEYS or not valid_ledger(c['ledger']) or not valid_env(c['environment']): return False
    for key,cell in c['registry'].pairs:
        if cell.tag!='AuthorityUnused':
            signed=cell.value if cell.tag=='AuthorityRegistered' else cell.value['signed']
            if not signed_valid(signed) or signed['policy']['body']['key']!=key: return False
            if cell.tag=='AuthorityConsumed' and cell.value['revision']<1: return False
        parent=c['parents'][key]
        if parent.tag=='ParentVacant':
            if is_parent(key) and cell!=UNUSED: return False
        else:
            p=parent.value['parent']; e=parent.value['entry']
            if p['key']!=key or not entry_valid(p,e) or not signed_valid(p['policy']) or p['policy']['policy']['body']['key']!=key: return False
            if cell.tag=='AuthorityUnused': return False
            if cell.tag=='AuthorityRegistered' and (cell.value!=p['policy'] or e!=initial_entry(p)): return False
            if cell.tag=='AuthorityConsumed' and (cell.value['signed']!=p['policy'] or cell.value['revision']!=e['revision'] or e['revision']<=0): return False
    return True
def signing_valid(s): return context_valid(s['context']) and s['signing'].keys()==KEYS
def execution_valid(s): return signing_valid(s['authority']) and s['attempts'].keys()==IDS
def a_execution(s): return execution_valid(s) and coupling(s['authority']['context'])
def dependencies(body):
    out=frozenset()
    for c in body['clauses']:
        out|=op_parents(c['operation'])
        out|=frozenset(x.value['key'] for x in c['conditions'] if x.tag=='ParentMatches')
    return out
def snapshot(c,b):
    deps=dependencies(b)
    return r(candidate=c['candidate'],ledger=c['ledger'],environment=c['environment'],keyCell=c['registry'][b['key']],
             parentCells=M(tuple((k,c['parents'][k]) for k in deps)),parentAuthorities=M(tuple((k,c['registry'][k]) for k in deps)))
def can_prepare(s,p,signer):
    a=s['authority']; c=a['context']; b=p['body']; key=b['key']
    if not (signing_valid(a) and coupling(c) and a_policy(p) and dependencies(b)<=KEYS): return False
    return signer==key['principal'] and c['registry'][key]==UNUSED and (not is_parent(key) or key['principal']==V('Alice')) and b['implementationVersion']==c['environment']['implementationVersion'] and b['enforcementMechanism']==c['environment']['enforcementMechanism'] and a['signing'][key]!=V('PreparedSigning',r(policy=p,signer=signer,snapshot=snapshot(c,b)))
def prepare(s,p,signer):
    a=s['authority']; record=V('PreparedSigning',r(policy=p,signer=signer,snapshot=snapshot(a['context'],p['body'])))
    return change(s,authority=change(a,signing=a['signing'].put(p['body']['key'],record)))
def sign(s,p,signer,token):
    a=s['authority']; c=a['context']; key=p['body']['key']; signed=r(policy=p,signer=signer,token=token)
    parents=c['parents']
    if is_parent(key):
        parent=parent_for(signed); parents=parents.put(key,V('ParentLive',r(parent=parent,entry=initial_entry(parent))))
    return change(s,authority=change(a,context=change(c,parents=parents,registry=c['registry'].put(key,V('AuthorityRegistered',signed))),signing=a['signing'].put(key,V('CompletedSigning',signed))))
def can_sign(s,p,signer,token):
    a=s['authority']; c=a['context']; key=p['body']['key']
    if not (signing_valid(a) and coupling(c) and a_policy(p) and dependencies(p['body'])<=KEYS): return False
    checked=a['signing'][key]
    return checked==V('PreparedSigning',r(policy=p,signer=signer,snapshot=snapshot(c,p['body']))) and signer==key['principal'] and token in (0,1) and c['registry'][key]==UNUSED and context_valid(sign(s,p,signer,token)['authority']['context'])
def id_matches(id,op):
    if op.tag=='OpFund': return id in (V('FundingOneAttempt'),V('FundingTwoAttempt'))
    if op.tag=='OpFillSlot': return id==V('FirstFillAttempt' if op.value['slot']==1 else 'SecondFillAttempt')
    if op.tag=='OpCancelParent': return id in (V('CancelAttempt'),V('FreshCancelAttempt'))
    return id==V('RecoveryAttempt' if op.tag=='OpRecover' else 'DispositionAttempt')
def can_propose(s,id,op): return execution_valid(s) and valid_op(op) and id_matches(id,op) and s['attempts'][id]==V('NoAttempt')
def facts(c):
    parents=[]
    for k,cell in c['parents'].pairs:
        if cell.tag=='ParentVacant': pf=V('ParentAbsent')
        else:
            e=cell.value['entry']; pf=V('ParentPresent',r(**{x:e[x] for x in ('cancelled','usedSlots','paid','remainingAllowance','revision')}))
        parents.append((k,pf))
    return r(ledger=c['ledger'],environment=c['environment'],parents=M(tuple(parents)))
def needed(op,es):
    keys=set()
    if op.tag=='OpRecover': keys.add(op.value['recovery'])
    else: keys.update(op_parents(op))
    for e in es:
        p=owner(e['source'])
        if op.tag in ('OpFillSlot','OpCancelParent','OpRecover'):
            base=op.value if op.tag=='OpCancelParent' else op.value['recovery' if op.tag=='OpRecover' else 'parent']
            keys.add(change(base,principal=p))
        else: keys.add(r(domain=V('SwapDomain'),principal=p,nonce=0 if op.tag=='OpFund' else 1))
    return frozenset(keys)
def consume_allowed(p,e,slot,prepared):
    return entry_valid(p,e) and prepared==e and not e['cancelled'] and slot in p['slots'].keys() and slot not in e['usedSlots'] and (slot==1 or 1 in e['usedSlots']) and p['slots'][slot]<=e['remainingAllowance']
def cancel_allowed(p,e,prepared): return entry_valid(p,e) and prepared==e and not e['cancelled'] and e['remainingAllowance']>0
def financial(c,op,es):
    if not op_parents(op): return bool(es)
    key=next(iter(op_parents(op))); cell=c['parents'][key]
    if cell.tag=='ParentVacant': return False
    p=cell.value['parent']; e=cell.value['entry']
    if op.tag=='OpCancelParent': return cancel_allowed(p,e,e) and es==()
    if op.tag=='OpFillSlot':
        slot=op.value['slot']; allowed=consume_allowed(p,e,slot,e); q=p['slots'].get(slot,0); recipient=p['recipient']
    else:
        allowed=e['cancelled'] and e['remainingAllowance']>0 and balance(c['ledger'],p['source'],p['asset'])==e['remainingAllowance']; q=e['remainingAllowance']; recipient=p['key']['principal']
    return allowed and es==(r(source=p['source'],destination=V('Wallet',recipient),asset=p['asset'],quantity=q),)
def projection_admits(op,pr):
    if op.tag=='OpCancelParent': return pr==V('NoCoreProjection')
    return pr.tag=='NoCoreProjection' or (pr.value['accepted'] and pr.value['error']==V('NoCoreError'))
def attempt_fidelity(a):
    o=a['observation']
    return valid_op(a['operation']) and observation(a['operation'],o['artifactAndCall'],o['resolvedPlan'],o['display'])==o and plan_valid(o['resolvedPlan'])
def evidence_policies(e): return all(a_policy(proof['signed']['policy']) for _,proof in e['signatures'].pairs)
def proof_allows(c,key,a,proof):
    p=proof['signed']['policy']; cell=c['registry'][key]
    allowed=policy_allows(p,a['operation'],a['observation'],facts(c))
    if proof['disposition']!=GOOD or proof['attempt']!=a or p['body']['key']!=key or not allowed: return False
    if cell.tag=='AuthorityRegistered': return cell.value==proof['signed']
    return cell.tag=='AuthorityConsumed' and cell.value['signed']==proof['signed'] and a['operation'].tag in ('OpFillSlot','OpCancelParent') and key in op_parents(a['operation'])
def common_verify(c,a,e):
    op=a['operation']; o=a['observation']
    if not context_valid(c) or not valid_op(op): return False
    keys=needed(op,o['effects'])
    if e['signatures'].keys()!=keys or not keys<=KEYS or not keys: return False
    return id_matches(a['id'],op) and a['context']==c and o['predecessor']==c['candidate'] and o['transactionTime']==c['environment']['physicalTime'] and o['effectEvidence']==GOOD and projection_admits(op,o['coreProjection']) and e['effect']==r(attempt=a,disposition=GOOD) and can_transfer(c['ledger'],o['effects']) and financial(c,op,o['effects']) and all(proof_allows(c,k,a,e['signatures'][k]) for k in keys)
def can_verify(s,id,e):
    if not a_execution(s): return False
    cell=s['attempts'][id]
    return cell.tag=='ProposedAttempt' and cell.value['id']==id and attempt_fidelity(cell.value) and evidence_policies(e) and common_verify(s['authority']['context'],cell.value,e)
def can_commit(s,id):
    if not a_execution(s): return False
    cell=s['attempts'][id]
    if cell.tag!='VerifiedOperation': return False
    a=cell.value['attempt']; e=cell.value['evidence']
    return a['id']==id and attempt_fidelity(a) and evidence_policies(e) and common_verify(s['authority']['context'],a,e)
def conflict(c,op):
    if not op_parents(op): return False
    cell=c['parents'][next(iter(op_parents(op)))]
    if cell.tag=='ParentVacant': return True
    p=cell.value['parent']; e=cell.value['entry']
    if op.tag=='OpFillSlot': return not consume_allowed(p,e,op.value['slot'],e)
    if op.tag=='OpCancelParent': return not cancel_allowed(p,e,e)
    return not e['cancelled'] or e['remainingAllowance']<=0
def reason(s,a,e):
    c=s['authority']['context']; o=a['observation']; op=a['operation']; proofs=e['signatures']
    if not coupling(c) or not attempt_fidelity(a) or not evidence_policies(e): return V('UnauthorizedEffect')
    if a['context']!=c or o['predecessor']!=c['candidate'] or o['transactionTime']!=c['environment']['physicalTime'] or e['effect']['attempt']!=a or any(p['attempt']!=a for _,p in proofs.pairs): return V('StaleBindings')
    if not context_valid(c) or not valid_op(op): return V('UnauthorizedEffect')
    keys=needed(op,o['effects']); dispositions=(o['effectEvidence'],e['effect']['disposition'])+tuple(p['disposition'] for _,p in proofs.pairs)
    if not keys<=proofs.keys() or V('EvidenceUnavailable') in dispositions: return V('EvidenceMissing')
    if proofs.keys()!=keys or any(d!=GOOD for d in dispositions): return V('UnauthorizedEffect')
    pr=o['coreProjection']
    if pr.tag=='CoreProjected' and not pr.value['accepted'] and pr.value['error'].tag=='CoreErrorCode' and pr.value['error'].value in ('time_before_state','contract_closed','input_required','no_matching_input','choice_out_of_bounds','non_positive_deposit') and op.tag!='OpCancelParent': return V('CoreRejected',pr.value['error'])
    if projection_admits(op,pr) and conflict(c,op): return V('ConsumptionConflict')
    return V('UnauthorizedEffect')
def rejection_domain(s):
    a=s['authority']; c=a['context']
    return s['attempts'].keys()==IDS and a['signing'].keys()==KEYS and c['registry'].keys()==KEYS and c['parents'].keys()==KEYS
def can_reject(s,id,e=None):
    if not rejection_domain(s): return False
    cell=s['attempts'][id]
    if e is None: return cell.tag=='VerifiedOperation' and cell.value['attempt']['id']==id and not can_commit(s,id)
    return cell.tag=='ProposedAttempt' and cell.value['id']==id and not can_verify(s,id,e)
def reject(s,id,e=None):
    cell=s['attempts'][id]; stage=V('CommitBoundary' if e is None else 'VerificationBoundary')
    a=cell.value['attempt'] if e is None else cell.value
    evidence=cell.value['evidence'] if e is None else e
    rejected=r(attempt=a,evidence=evidence,observedContext=s['authority']['context'],reason=reason(s,a,evidence),stage=stage)
    return change(s,attempts=s['attempts'].put(id,V('RejectedOperation',rejected)))
def commit(s,id):
    done=s['attempts'][id].value; a=done['attempt']; c=a['context']; op=a['operation']; o=a['observation']; parents=c['parents']; reg=c['registry']
    for key in needed(op,o['effects']):
        rev=parents[key].value['entry']['revision']+1 if key in op_parents(op) and is_parent(key) and parents[key].tag=='ParentLive' else 1
        reg=reg.put(key,V('AuthorityConsumed',r(signed=done['evidence']['signatures'][key]['signed'],revision=rev)))
    if op.tag in ('OpFillSlot','OpCancelParent'):
        key=next(iter(op_parents(op))); live=parents[key].value; p=live['parent']; e=live['entry']
        updated=change(e,claim=V('Claimed',p),revision=e['revision']+1)
        if op.tag=='OpCancelParent': updated=change(updated,cancelled=True)
        else:
            slot=op.value['slot']; q=p['slots'][slot]
            updated=change(updated,usedSlots=e['usedSlots']|frozenset((slot,)),paid=e['paid']+q,remainingAllowance=e['remainingAllowance']-q)
        parents=parents.put(key,V('ParentLive',change(live,entry=updated)))
    after=change(c,candidate=o['proposedSuccessor'],ledger=transfers(c['ledger'],o['effects']),registry=reg,parents=parents)
    return change(s,authority=change(s['authority'],context=after),attempts=s['attempts'].put(id,V('ExecutedOperation',done)))

# Internal semantic commands are mapped from the exact public A4 ADT in Task4.
def gate(s,cmd):
    x=cmd.value; tag=cmd.tag
    if tag=='Prepare': return can_prepare(s,x['policy'],x['signer'])
    if tag=='Sign': return can_sign(s,x['policy'],x['signer'],x['token'])
    if tag=='Propose': return can_propose(s,x['id'],x['operation'])
    if tag=='Verify': return can_verify(s,x['id'],x['evidence'])
    if tag=='Commit': return can_commit(s,x['id'])
    if tag=='RejectProposed': return can_reject(s,x['id'],x['evidence'])
    if tag=='RejectVerified': return can_reject(s,x['id'])
    if tag=='Advance': return a_execution(s) and valid_env(x['environment']) and x['environment']['physicalTime']>s['authority']['context']['environment']['physicalTime']
    if tag=='CanConsume': return consume_allowed(x['parent'],x['entry'],x['slot'],x['prepared'])
    if tag=='CanCancel': return cancel_allowed(x['parent'],x['entry'],x['prepared'])
    if tag=='FinancialGuard': return financial(s['authority']['context'],x['operation'],x['effects'])
    raise Invalid('unknown command')

def apply(s,cmd):
    require(gate(s,cmd),'transition guard false')
    x=cmd.value; tag=cmd.tag
    if tag=='Prepare': return prepare(s,x['policy'],x['signer'])
    if tag=='Sign': return sign(s,x['policy'],x['signer'],x['token'])
    if tag=='Propose':
        a=r(id=x['id'],actor=x['actor'],operation=x['operation'],observation=x['observation'],context=s['authority']['context'])
        return change(s,attempts=s['attempts'].put(x['id'],V('ProposedAttempt',a)))
    if tag=='Verify': return change(s,attempts=s['attempts'].put(x['id'],V('VerifiedOperation',r(attempt=x['evidence']['effect']['attempt'],evidence=x['evidence']))))
    if tag=='Commit': return commit(s,x['id'])
    if tag=='RejectProposed': return reject(s,x['id'],x['evidence'])
    if tag=='RejectVerified': return reject(s,x['id'])
    if tag=='Advance': return change(s,authority=change(s['authority'],context=change(s['authority']['context'],environment=x['environment'])))
    raise Invalid('probe is not a transition')
