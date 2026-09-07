# Candidate A Independent Replay Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Independently check complete A4 agreement and authority histories, including every required denial and retained rejection, without changing schema1 acceptance.

**Architecture:** A strict typed ITF decoder feeds a Python authority replay engine. Agreement results come from frozen Python Core through the existing independently written decoder/effect helpers. A separately enumerated case/event inventory and exact raw-position provenance gate surround semantic replay; neither an exported boolean nor a digest grants semantic acceptance.

**Tech Stack:** Existing Python 3 environment and pytest; frozen common/A source at `d14cfea98a1c9213e5ef5f12c1a088f4e966083d`; adopted design `effb7af`; existing schema1 checker read-only.

## Global Constraints

- Status: specified-only concrete checker subplan, subject to independent review before execution. Embedded code has not been executed by this planner.
- Follow main completion XML A4-R01–R05 and `docs/superpowers/specs/2026-09-05-candidate-a-integrated-export-design.md`.
- Do not edit Core, common/A Quint, accepted lifecycle modules, schema1 exporter/checker, producer files, main status, or session databases.
- Checker implementation owner must not author the producer. Root owns shared inventory/schema admission and final evidence archival.
- Preserve all maps, optional choices, full programs, ordered effects/payments, original attempts/evidence and exact signing/parent/nonce history.
- A retained `EvidenceValid` value is a finite external premise, not cryptographic verification. `attempt.actor` is metadata.
- Never accept a subset. Keep actual observed Core evaluation separate from an adversarial claimed observation. A denied pure guard is not an executed rejection.
- No tests, source implementation or commits are authorized by this planning task. Commands below are for the future implementation task.

## File ownership and review boundaries

Create `scripts/a4_carrier.py` (strict immutable typed carrier), `scripts/a4_agreement.py` (read-only Core bridge), `scripts/a4_authority.py` (independent guards/updates), `scripts/a4_cases.py` (independent fixture and policy construction), `scripts/a4_inventory.py` (independent fixed case/event enumeration), `scripts/check_s02_candidate_a_integrated.py` (package/provenance/replay CLI), and `tests/test_s02_candidate_a_integrated.py` (independent unit and mutation controls). Root owns `evidence/s02-candidate-a-completion/a4/` and the admitted inventory JSON. No checker module imports producer code.

Tasks are gated separately: carrier/agreement, authority, fixture construction, negative inventory, raw-package admission, and mutation/end-to-end acceptance. Preserve original behavioral RED source bytes, not reconstructed historical stubs. A parser/import error is not behavioral RED. Each gate requires source review and terminal results before its explicit-path commit.

## Exact carrier convention

Exported nested authority values retain native ITF JSON: records are JSON objects; enums are `{tag,value}`; unit is `{"#tup":[]}`; tuples/maps/sets use `#tup/#map/#set`; integers may use canonical `#bigint`. Lists stay ordered. The checker decodes to immutable `R`, `M`, `V`, tuples and frozensets; these types are checker-private. Map/set order is semantically irrelevant, but duplicate members are errors. Raw provenance compares unnormalized JSON, so normalization cannot hide substitutions.

### Task 1: strict carrier and frozen agreement bridge

**Files:** create `scripts/a4_carrier.py`, `scripts/a4_agreement.py`; create `tests/test_s02_candidate_a_integrated.py`.

**Interfaces:** `decode(raw)->Value`, `encode(value)->JSON`, `typed(value,name)->Value`, `oracle(request)->(raw_result, ordered_effects)`, `observation(operation,call,plan,display)->R|None`. Invalid typed carriers raise `Invalid`; semantic A-domain failures return false/None at guard boundaries. An unsupported actual computation payload blocks export acceptance, never becomes a Core rejection.

- [ ] Add carrier code with the single compiling RED change `if len(pairs) != len(dict(pairs)): pass` in `M.__post_init__`; run `test_duplicate_map_rejected`, retain failure and source closure; restore the raising branch shown below.
- [ ] Add the complete carrier implementation:

```python
from __future__ import annotations
from dataclasses import dataclass
from typing import Any
import json
import re

class Invalid(ValueError):
    pass

class ITFList(tuple):
    pass

class ITFTuple(tuple):
    pass

def require(condition, message):
    if not condition:
        raise Invalid(message)

@dataclass(frozen=True)
class R:
    fields: tuple
    def __getitem__(self, key):
        return dict(self.fields)[key]
    def keys(self):
        return set(dict(self.fields))

def r(**fields):
    return R(tuple(sorted(fields.items())))

def change(record, **fields):
    return r(**(dict(record.fields) | fields))

@dataclass(frozen=True)
class V:
    tag: str
    value: Any = ()

@dataclass(frozen=True)
class M:
    pairs: tuple
    def __post_init__(self):
        pairs = self.pairs
        if len(pairs) != len(dict(pairs)):
            raise Invalid('duplicate map key')
        object.__setattr__(self, 'pairs', tuple(sorted(pairs, key=lambda pair: repr(pair[0]))))
    def keys(self):
        return frozenset(key for key, _ in self.pairs)
    def get(self, key, default=None):
        return dict(self.pairs).get(key, default)
    def __getitem__(self, key):
        return dict(self.pairs)[key]
    def put(self, key, value):
        return M(tuple((k, v) for k, v in self.pairs if k != key) + ((key, value),))

def loads(raw):
    def unique(pairs):
        require(len(pairs) == len(dict(pairs)), 'duplicate JSON key')
        return dict(pairs)
    return json.loads(raw, object_pairs_hook=unique,
                      parse_constant=lambda value: (_ for _ in ()).throw(Invalid('nonfinite number')))

def decode(x):
    if type(x) in (str, bool, int):
        return x
    if isinstance(x, list):
        return ITFList(decode(y) for y in x)
    require(type(x) is dict, 'invalid ITF scalar')
    if set(x) == {'#bigint'}:
        require(type(x['#bigint']) is str and re.fullmatch(r'-?(0|[1-9][0-9]*)', x['#bigint'])
                and x['#bigint'] != '-0', 'invalid bigint')
        return int(x['#bigint'])
    if set(x) == {'#tup'}:
        require(isinstance(x['#tup'], list), 'tuple payload')
        return ITFTuple(decode(y) for y in x['#tup'])
    if set(x) == {'#set'}:
        require(isinstance(x['#set'], list), 'set payload')
        values = tuple(decode(y) for y in x['#set'])
        require(len(values) == len(frozenset(values)), 'duplicate set member')
        return frozenset(values)
    if set(x) == {'#map'}:
        require(isinstance(x['#map'], list), 'map payload')
        require(all(isinstance(y, list) and len(y) == 2 for y in x['#map']), 'map pair')
        return M(tuple((decode(k), decode(v)) for k, v in x['#map']))
    if set(x) == {'tag', 'value'}:
        require(type(x['tag']) is str, 'enum tag')
        return V(x['tag'], decode(x['value']))
    require(not any(k.startswith('#') for k in x), 'unknown ITF carrier')
    return r(**{k: decode(v) for k, v in x.items()})

def encode(x):
    if isinstance(x, R): return {k: encode(v) for k, v in x.fields}
    if isinstance(x, V): return {'tag': x.tag, 'value': {'#tup': []} if x.value == () and x.tag!='EffectsExtractedA' else encode(x.value)}
    if isinstance(x, M): return {'#map': [[{'#tup':[encode(y) for y in k]} if isinstance(k,tuple) else encode(k), encode(v)] for k,v in x.pairs]}
    if isinstance(x, frozenset): return {'#set': [encode(y) for y in sorted(x, key=repr)]}
    if isinstance(x, tuple): return [encode(y) for y in x]
    require(type(x) in (int, bool, str), 'cannot encode')
    return x

# Schema tuples: list, set, map, tuple, sum. Record schemas have exact fields.
def L(t): return ('list', t)
def S(t): return ('set', t)
def D(k, v): return ('map', k, v)
def T(*ts): return ('tuple', *ts)
def U(**tags): return ('sum', tags)
def E(*tags): return U(**{tag: T() for tag in tags})
SCHEMA = {
 'Principal': E('Alice','Bob','Mallory'), 'Asset': E('TokenA','TokenB'),
 'Domain': E('SwapDomain','InstallmentDomain'), 'Time': E('Time0','Time1','Time2','Time100','Time101'),
 'Node': E(*(f'N{i}' for i in range(16))),
 'ChoiceId': E('SettleId','FirstFillId','SecondFillId','RecoveryId','OtherId'),
 'Optional': U(NoInt=T(),IntValue='int'), 'Error': U(NoCoreError=T(),CoreErrorCode='str'),
 'Account': {'owner':'Principal','asset':'Asset'},
 'Location': U(Wallet='Principal',Escrow='Account'),
 'Transfer': {'source':'Location','destination':'Location','asset':'Asset','quantity':'int'},
 'Payment': {'source':'Account','recipient':'Principal','asset':'Asset','quantity':'int'},
 'Warning': {'code':'str','requested':'Optional','paid':'Optional'},
 'Value': U(ConstantA='int'), 'Observation': U(ChoiceEqualsA={'id':'ChoiceId','expected':'int'}),
 'CaseAction': U(DepositA={'account':'Account','depositor':'Principal','amount':'Value'},
                 ChoiceA={'id':'ChoiceId','chooser':'Principal','lower':'int','upper':'int'}),
 'Case': {'caseAction':'CaseAction','continuation':'Node'},
 'ANode': U(CloseA=T(),PayA={'account':'Account','payee':'Principal','amount':'Value','continuation':'Node'},
            IfA={'observation':'Observation','thenNode':'Node','elseNode':'Node'},
            WhenA={'cases':L('Case'),'timeout':'Time','timeoutNode':'Node'}),
 'Program': {'root':'Node','nodes':D('Node','ANode')},
 'State': {'accounts':D('Account','int'),'choices':D('ChoiceId','Optional'),'continuation':'Node','minimumTime':'Time'},
 'Before': {'program':'Program','state':'State'},
 'Input': U(NoAInput=T(),PresentAInput=U(DepositInputA={'account':'Account','depositor':'Principal','quantity':'int'},
                                       ChoiceInputA={'id':'ChoiceId','chooser':'Principal','chosen':'int'})),
 'Request': {'before':'Before','input':'Input','now':'Time'},
 'Call': U(AgreementCallA='Request',CancellationCallA={'before':'Before','now':'Time'}),
 'Neutral': U(NoInput=T(),DepositLike={'location':'Location','depositor':'Principal','asset':'Asset','quantity':'int'},
              ChoiceLike={'id':'str','chooser':'Principal','chosen':'int'}),
 'Raw': {'accepted':'bool','state':'State','error':'Error','payments':L('Payment'),'warnings':L('Warning'),'reductions':'int'},
 'ProjectedState': {'accounts':D('Account','int'),'choices':D('str','Optional'),
                    'continuation':{'program':'Program','node':'Node'},'minimumTime':'int'},
 'Projected': {'accepted':'bool','state':'ProjectedState','error':'Error','payments':L('Payment'),'warnings':L('Warning'),'reductions':'int'},
 'Projection': U(NoCoreProjection=T(),CoreProjected='Projected'),
 'Key': {'domain':'Domain','principal':'Principal','nonce':'int'},
 'Environment': {'physicalTime':'int','anchor':'int','implementationVersion':'int','enforcementMechanism':'int'},
 'Ledger': D(T('Location','Asset'),'int'),
 'ParentFacts': U(ParentAbsent=T(),ParentPresent={'cancelled':'bool','usedSlots':S('int'),'paid':'int','remainingAllowance':'int','revision':'int'}),
 'Facts': {'ledger':'Ledger','environment':'Environment','parents':D('Key','ParentFacts')},
 'Operation': U(OpFund=T(),OpSettle=T(),OpVoluntaryRefund=T(),OpDeadlineRefund=T(),
                OpFillSlot={'parent':'Key','slot':'int'},OpCancelParent='Key',OpRecover={'parent':'Key','recovery':'Key'}),
 'Capability': E('FundCapability','DisposeCapability','FirstFillCapability','SecondFillCapability','CancelCapability','RecoveryCapability'),
 'Condition': U(InputIs='Neutral',BeforeTime='int',AtOrAfterTime='int',
                ParentMatches={'key':'Key','expected':'ParentFacts'},SourceBalanceIs={'location':'Location','asset':'Asset','quantity':'int'}),
 'Clause': {'operation':'Operation','conditions':L('Condition'),'requiredEffects':L('Transfer'),'allowedEffects':L('Transfer'),'effectOrder':E('ExactOrder','AnyOrder')},
 'Body': {'key':'Key','clauses':L('Clause'),'debitLocations':S('Location'),'capabilities':S('Capability'),'disclosures':S('str'),
          'validFrom':'int','validUntil':'int','implementationVersion':'int','enforcementMechanism':'int'},
 'Planned': {'predecessor':'Before','proposedSuccessor':'Before','artifactAndCall':'Call','input':'Neutral','operation':'Operation',
             'transactionTime':'int','effects':L('Transfer'),'coreProjection':'Projection','predecessorFacts':'Facts'},
 'Plan': {'identity':E('SwapPlanA','InstallmentPlanA','RecoveryPlanA'),'operations':L('Planned')},
 'Profile': E('SignAfterResolve','SignBeforeResolve'),
 'Policy': {'body':'Body','profile':'Profile','binding':U(AfterResolution={'identity':'Plan','operations':L('Planned')},
                                                      BeforeResolution=U(AnyArtifactUnderMechanism=T(),PinnedArtifact='Call'))},
 'Signed': {'policy':'Policy','signer':'Principal','token':'int'},
 'AuthorityCell': U(AuthorityUnused=T(),AuthorityRegistered='Signed',AuthorityConsumed={'signed':'Signed','revision':'int'}),
 'Parent': {'key':'Key','policy':'Signed','source':'Location','recipient':'Principal','asset':'Asset','budget':'int','slots':D('int','int')},
 'Entry': {'claim':U(Unclaimed=T(),Claimed='Parent'),'usedSlots':S('int'),'paid':'int','remainingAllowance':'int','cancelled':'bool','revision':'int'},
 'ParentCell': U(ParentVacant=T(),ParentLive={'parent':'Parent','entry':'Entry'}),
 'Context': {'candidate':'Before','ledger':'Ledger','environment':'Environment','registry':D('Key','AuthorityCell'),'parents':D('Key','ParentCell')},
 'Snapshot': {'candidate':'Before','ledger':'Ledger','environment':'Environment','keyCell':'AuthorityCell',
              'parentCells':D('Key','ParentCell'),'parentAuthorities':D('Key','AuthorityCell')},
 'SigningRecord': U(NoSigningCheck=T(),PreparedSigning={'policy':'Policy','signer':'Principal','snapshot':'Snapshot'},CompletedSigning='Signed'),
 'Signing': {'context':'Context','signing':D('Key','SigningRecord')},
 'Reason': U(CoreRejected='Error',UnauthorizedEffect=T(),StaleBindings=T(),EvidenceMissing=T(),ConsumptionConflict=T()),
 'Outcome': U(FundingAccepted=T(),Settlement=T(),VoluntaryRefund=T(),DeadlineRefund=T(),FirstInstallment=T(),SecondInstallment=T(),
              Cancellation=T(),Recovery=T(),Rejected='Reason'),
 'Disposition': E('EvidenceValid','EvidenceUnavailable','EvidenceInvalid'),
 'CandidateObservation': {'predecessor':'Before','proposedSuccessor':'Before','artifactAndCall':'Call','resolvedPlan':'Plan','transactionTime':'int',
                          'input':'Neutral','effects':L('Transfer'),'outcome':'Outcome','coreProjection':'Projection','effectEvidence':'Disposition',
                          'display':E('PublicDisplay','AlternateDisplay')},
 'AttemptId': E('FundingOneAttempt','FundingTwoAttempt','DispositionAttempt','FirstFillAttempt','SecondFillAttempt','CancelAttempt','FreshCancelAttempt','RecoveryAttempt'),
 'Attempt': {'id':'AttemptId','actor':'Principal','operation':'Operation','observation':'CandidateObservation','context':'Context'},
 'Evidence': {'effect':{'attempt':'Attempt','disposition':'Disposition'},
              'signatures':D('Key',{'attempt':'Attempt','signed':'Signed','disposition':'Disposition'})},
 'Verified': {'attempt':'Attempt','evidence':'Evidence'},
 'AttemptRecord': U(NoAttempt=T(),ProposedAttempt='Attempt',VerifiedOperation='Verified',ExecutedOperation='Verified',
                    RejectedOperation={'attempt':'Attempt','evidence':'Evidence','observedContext':'Context','reason':'Reason',
                                       'stage':E('VerificationBoundary','CommitBoundary')}),
 'Execution': {'authority':'Signing','attempts':D('AttemptId','AttemptRecord')},
}

def typed(x, schema):
    if type(schema) is str:
        if schema in ('int','bool','str'):
            require(type(x) is {'int':int,'bool':bool,'str':str}[schema], 'scalar type '+schema)
            return x
        return typed(x, SCHEMA[schema])
    if isinstance(schema, dict):
        require(isinstance(x,R) and x.keys()==set(schema), 'record fields')
        for k,t in schema.items(): typed(x[k],t)
    elif schema[0]=='sum':
        require(isinstance(x,V) and x.tag in schema[1], 'variant')
        typed(x.value,schema[1][x.tag])
    elif schema[0]=='map':
        require(isinstance(x,M),'map type')
        for k,v in x.pairs: typed(k,schema[1]); typed(v,schema[2])
    elif schema[0]=='set':
        require(isinstance(x,frozenset),'set type')
        for y in x: typed(y,schema[1])
    elif schema[0]=='list':
        require(type(x) in (tuple,ITFList),'list type')
        for y in x: typed(y,schema[1])
    else:
        require(type(x) in (tuple,ITFTuple) and len(x)==len(schema)-1,'tuple arity')
        for y,t in zip(x,schema[1:]): typed(y,t)
    return x
```

ITF tuple encoding inside map keys uses `#tup`, not JSON list. Raw provenance checks retain the original JSON in addition to this semantic carrier.

- [ ] Add the agreement bridge below. It imports existing independent Python helpers read-only; no evaluator clauses are copied from Quint.

```python
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
```

- [ ] Add independent tests:

```python
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
```

- [ ] Run `/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s02_candidate_a_integrated.py`; expect terminal0 after correction. Retain actual result, not an assumed count. Root reviews imported helper pins, full typed schema and bridge output before Task2.

### Task 2: independently derived authority guards and atomic replay

**Files:** create `scripts/a4_authority.py`; extend `tests/test_s02_candidate_a_integrated.py`.

**Interfaces:** consumes immutable typed carriers and `observation`/`valid_before`; produces `gate(state,command)->bool`, `apply(state,command)->Execution`, `reason(state,attempt,evidence)->Reason`. Public replay decodes/validates types first. False semantic validity must not throw. `apply` has the explicit precondition `gate`; package replay checks it before every transition. `gate` supports direct common proposal and specified parent-consumption probes as well as A gates.

- [ ] Add this implementation with one RED change: return `False` in `can_reject` when coupling fails. Add `test_uncoupled_rejection_is_reachable`, observe assertion failure with well-typed state, archive exact closure, restore the final code. This is separate from the original lifecycle RED receipts.
- [ ] Add the complete authority implementation:

```python
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
```

**Important separation:** structural carrier typechecking does not establish complete finite maps or semantic validity. Proposal uses unchanged common `canPropose`; lifecycle route descriptors separately require its A-state precondition. Rejection validates its structural envelope and stage rather than requiring the A predicate that failed. Future facts in plans are deterministic fidelity checks, not reachability proofs. In particular an expired policy is not automatically invalid for `canSign`: preserve the actual source guard, which checks snapshot freshness rather than inventing a wall-clock signing policy.

`FinancialGuard` is exactly `operationFinancialGuard`, without an invented `validAuthorityContext` or `validOperation` conjunct. Its non-parent branches test only that effects are nonempty; authorization, effect validity, transferable balances and current context are enforced by their separate enclosing gates. The direct no-cancel probe supplies the independently reconstructed complete current context.

- [ ] Add the synthetic tests below. The uncoupled control deliberately exercises the rejection boundary with a structurally complete state, without depending on a later fixture task.

```python
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
```

- [ ] Run the focused tests and recursive import check via `/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s02_candidate_a_integrated.py`. Root reviews rule-by-rule against effects/policies/authorization/execution/consumption/A-boundary source before acceptance. Do not call this independent cryptographic verification.

### Task 3: independent lifecycle fixtures and ordinary command inventory

**Files:** create `scripts/a4_cases.py`; extend `tests/test_s02_candidate_a_integrated.py`.

**Interfaces:** `initial(loop)->Execution`; `parameters(loop,scenario)->dict`; `policy_for(loop,scenario,profile,id,principal)->Policy`; `planned(loop,scenario,id)->Planned`; `ordinary(loop,scenario)->tuple[tuple]`; `materialize(state,loop,scenario,profile,step)->V`. The symbolic step tuples are checker-owned descriptors, not imported Quint commands. All expected agreement results are evaluated by the frozen Python Core, while initial programs, inputs and policy structure are independently fixed.

- [ ] Add the code below with a behavioral RED variant: in `initial`, replace `ledger=ledger(loop,0)` with `ledger=ledger(loop,2) if loop=='installment' else ledger(loop,0)`. This changes installment escrow from 10 to 0 while retaining the initial candidate. `test_independent_initial_coupling` must fail after imports succeed. Retain RED bytes/results, then restore the shown value.

```python
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
```

- [ ] Add these complete tests, then run the same explicit pytest file. Both signing profiles are real registrations in every positive prefix; fixtures do not seed signatures.

```python
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
```

- [ ] Gate3: non-author review compares every fixture, expected policy body, full program table, request and route against accepted lifecycle source and XML. Run `/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s02_candidate_a_integrated.py`; expect terminal0. An independent Python route failure is investigated, not bypassed by using exported successor state.

### Task 4: exact expanded negative schedules and expected event replay

**Files:** create `scripts/a4_inventory.py`; extend `tests/test_s02_candidate_a_integrated.py`.

**Interfaces:** `descriptors()->tuple[dict]` fixes 78 records; `history(descriptor)->tuple[R]` independently constructs every latest event and its expected before/after execution values. Internal event records have `latest`, `before`, `after`. No function takes a submitted history as its expected result. The checker compares submitted values to these independently recomputed records.

The public event schema is exact. JSON event fields are `case_id, profile, sequence, kind, arguments, observed_guard, computations, before, after, provenance`. Provenance has `input_path,input_sha256,before_index,after_index`. Nested ITF `latestEvent` fields are `caseId,profile,sequence,kind,arguments,observedGuard,computations`. `arguments` is `{guard:A4Guard,command:A4Command}`. `profile` is one of the two profile-name strings, while all embedded authority policies keep the actual `SigningProfile` variant. The command suffix `A4` is mandatory. Sequence starts at 0 for each case-start. The raw state retains `authorityState,latestEvent,caseIndex,cursor`; cursor equals latest-event sequence and caseIndex equals the descriptor's position in its loop's fixed inventory.

The only exported scalar conversion is `sequence`: export a plain JSON integer after strict lossless decoding of raw ITF `#bigint`; booleans are forbidden. `caseIndex` and `cursor` are similarly decoded only for positional validation, not exported as authority state. All nested arguments, computations, before and after values remain unnormalized raw JSON. Provenance uses strict integer equality for sequence and exact raw JSON value equality for every other event field.

Case descriptors are exactly `{case_id,lifecycle,profile,scenario,control}`. IDs are `lifecycle/scenario/control/profile`. Negative installment scenarios use `recover-r0-choice2` except unused-successor (`two-fills`); negative swap scenarios use `funded2-settle`. Profiles are ordered After then Before per scenario/control. Loop inventories start with ordinary cases, then verified-stale (swap only), then negatives in the literal order below. A driver may emit one complete loop per ITF or split only on complete case boundaries; no partial or overlapping case may be admitted.

| Inventory family | Cases | Events |
| --- | ---: | ---: |
| Installment ordinary | 22 | 516 |
| Swap ordinary | 24 | 484 |
| Swap verified-stale | 2 | 44 |
| Installment negatives | 10 | 122 |
| Swap negatives | 20 | 591 |
| Total | 78 | 1757 |

Counts include case-start and case-end. Installment ordinary executed IDs each receive distinct denied proposal and commit events; every cancelled ordinary parent receives denied cancellation and both slot probes. Swap ordinary funding IDs and disposition ID receive both replay probes, including retained refused disposition. Negative cases do not reuse ordinary acceptance records or silently count test definitions as exported executions.

- [ ] Add code below with the single behavioral RED variant: omit `('old-nonce','recover-r0-choice2')` from `I_CONTROLS`. `test_inventory_is_not_submission_defined` must fail its literal 78/1757 assertions. Restore the literal entry only after terminal RED is retained.

```python
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
```

Public aliases are fixed, not heuristic: `CanCancel` emits `CancelParentProbeA4` with `CancelParentGuardA4`; `CanConsume` emits `ConsumeSlotProbeA4` with `ConsumeSlotGuardA4`; `FinancialGuard` emits `FinancialProbeA4` with `FinancialGuardA4`. These mappings change names only; the full parameter record remains intact.

- [ ] Add these complete inventory and semantics tests. The first loop computes expected histories, not accepted export fixtures; package acceptance still requires raw provenance and producer records.

```python
from scripts.a4_inventory import descriptors,history,terminal

def test_inventory_is_not_submission_defined():
    ds=descriptors(); assert len(ds)==78 and len({d['case_id'] for d in ds})==78
    assert sum(len(history(d)) for d in ds)==1757
    totals={}
    for d in ds:
        family=d['lifecycle']+'/'+('ordinary' if d['control']=='ordinary' else 'verified-stale' if d['control']=='verified-stale' else 'negative')
        totals[family]=totals.get(family,0)+len(history(d))
    assert totals=={'installment/ordinary':516,'swap/ordinary':484,'swap/verified-stale':44,'installment/negative':122,'swap/negative':591}

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
```

- [ ] Gate4: root and non-author reviewer compare 78-case/1757-event literal inventory with producer plan independently. Run `/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s02_candidate_a_integrated.py`; every expected negative guard must evaluate false, each actual rejection true. Refusal terminal and negative-complete are case statuses, not financial claims. Negative prefixes may intentionally retain unrelated pending attempts or signing records.

### Task 5: mandatory full-package, source and adjacent-ITF provenance admission

**Files:** create `scripts/check_s02_candidate_a_integrated.py`; extend `tests/test_s02_candidate_a_integrated.py`.

**Interfaces:** `compare_case(case,descriptor)->None` raises `Invalid` for semantic/inventory mismatch; `check_document(document,admission,source_root,input_root,inventory_bytes)->dict` returns a finite-record report or raises `Invalid`; CLI requires all roots plus `--admission` and `--inventory`, has no subset option, exits 0 only for the complete inventory. Diagnostic events are retained in original ITF by the producer but not part of the accepted fixed inventory.

Top-level document keys: `schema_version,inventory_sha256,source_pins,input_pins,receipt_pins,cases`. Root admission keys: `schema_version,inventory_sha256,source_pins,input_pins,receipt_pins,entries`. `schema_version` is integer 2. Each pin map maps canonical relative path to lowercase SHA256. `entries` is exactly `{installment: entry path,swap: entry path}`. Admission is a separately reviewed, explicit CLI trust input: passing a rewritten admission authorizes only those bytes, not semantics. Neither admission nor a hash proves generator execution. Runtime receipt integrity is root's external admission; this checker checks the admitted receipt bytes and closed source/input inventory. Symbolic evidence premises remain premises.

The independently generated inventory JSON is `{schema_version:2,cases:[{descriptor fields,event_count}]}` in descriptor order. Canonical JSON hashing uses UTF-8 `json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False)` with no final newline. Root records the canonical hash; actual inventory bytes may have whitespace. Producer and checker independently generate the same logical inventory before either is run on actual exports.

Exactly two raw inputs are required: `installment.itf.json` and `swap.itf.json`. Both contain every case for their loop in the specified order. No event is dropped; every raw state is exactly one event, including first case-start. For state k, `after_index=k` and `before_index=max(0,k-1)`. Later case-start is the only permitted discontinuity and must immediately follow case-end. It records a transition from the previous final state to the new unsigned state. Within-case continuity has no reset escape. The first state of each raw file is unsigned and binds before=after. The latest event avoids duplicating the full history in each raw state, but the checker consumes the complete raw history.

- [ ] Add the code below with a compiling behavioral RED: skip the `require(exported_raw == raw_field, 'raw provenance field '+name)` line in `bind_event`. `test_binding_checks_observed_field` below must fail. Retain RED and restore this one check before Task5 GREEN. Task6 additionally tests the same boundary with real-shaped independent histories.

```python
from __future__ import annotations
import argparse
import hashlib
import inspect
import json
import re
from pathlib import Path,PurePosixPath
import sys
from scripts.a4_carrier import R,M,V,Invalid,require,loads,decode,encode,typed
from scripts.a4_inventory import descriptors,history
from scripts.a4_agreement import core

ROOT=Path(__file__).resolve().parents[1]
FROZEN_EXTRA={
 'specs/quint/s02/consumption.qnt':'3dd07f3c5f274aff4fce4de9c245aa0b1f1fe7f68e29e5de95a0b1c42a00f93b',
 'specs/quint/s02/policies.qnt':'0886619203c3de16f2326e27e9d4ae2ddfda8c54742c231dd4e824397c5fa9fe',
 'specs/quint/s02/authorization.qnt':'b59779d5e2e7f1bf0a14952dfe1cd3ab70bcfe813210d690abbb3e3205a6df61',
 'specs/quint/s02/execution.qnt':'cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928',
 'specs/quint/s02/candidate_a_authority_adapter.qnt':'3f3b093f4c718dd08eda38e610de700d0a24138beb82fd7b2b12dcf9d300bda8',
 'specs/quint/s02/candidate_a_authority_boundary.qnt':'95ddf75ef32b432acaec5384c89826d0dc12245b80f78fd8a60ff78d0da8164b',
 'specs/quint/s02/candidate_a_authority_swap.qnt':'294633d4213d44075df76085f67b9bd24fab23d07bc863fff8b4f22e79d10024',
 'specs/quint/s02/candidate_a_authority_swap_fixtures.qnt':'fb407555584e167ae0fddc3e59fbf6d64ecd79a00d2ad03d02f159da8429ed15',
 'specs/quint/s02/candidate_a_authority_swap_harness.qnt':'bb8991e439153af69f3c8ec52df1771786a0ce0d5360f975189958b978555e29',
 'specs/quint/s02/candidate_a_authority_swap_test.qnt':'5aed1719310ef472f8fdd5a19e8cc5188a47c7c136f7fc3158bb5a12943636c5',
 'specs/quint/s02/candidate_a_authority_installment.qnt':'f12d91938098d48a313baf7cb5218f84b6bd840da8c518d54f195e0f7fa1e4cd',
 'specs/quint/s02/candidate_a_authority_installment_fixtures.qnt':'22d975d6d615e1f8e80c453115ded79fa68f783880a036f73470814221bf1a92',
 'specs/quint/s02/candidate_a_authority_installment_harness.qnt':'9a49b2a76d0c9e27a0d06b942b4f6a3338bf54a1e005cbba6d6388e91dcf82ca',
 'specs/quint/s02/candidate_a_authority_installment_test.qnt':'b1d9c21c5285105b382525df8df2ebaa41dbf10979f400d975b40500c3f7c3d1',
 'scripts/check_s02_candidate_a_correspondence.py':'c6923d0e08ec0206dfddad70eaff5a3a9a2f4a160328802e0e69be965be90aec',
}
ENTRIES={'installment':'specs/quint/s02/candidate_a_integrated_installment_export.qnt',
         'swap':'specs/quint/s02/candidate_a_integrated_swap_export.qnt'}
PYTHON_SOURCES=frozenset(('moriarty/core.py','moriarty/swap.py','scripts/check_s02_candidate_a_correspondence.py',
    'scripts/a4_carrier.py','scripts/a4_agreement.py','scripts/a4_authority.py','scripts/a4_cases.py','scripts/a4_inventory.py',
    'scripts/check_s02_candidate_a_integrated.py','scripts/s02_candidate_a_integrated_inventory.py',
    'scripts/export_s02_candidate_a_integrated.py','scripts/record_s02_candidate_a_integrated.py',
    'tests/test_s02_candidate_a_integrated.py','tests/test_s02_candidate_a_integrated_export.py'))
QNT_TESTS=('specs/quint/s02/candidate_a_integrated_export_test.qnt',
    'specs/quint/s02/candidate_a_authority_installment_test.qnt','specs/quint/s02/candidate_a_authority_swap_test.qnt')
RAW_NAMES=frozenset(('installment.itf.json','swap.itf.json'))
VARS=('authorityState','latestEvent','caseIndex','cursor')
EVENT_FIELDS=frozenset(('case_id','profile','sequence','kind','arguments','observed_guard','computations','before','after','provenance'))
RENAME={'case_id':'caseId','profile':'profile','sequence':'sequence','kind':'kind','arguments':'arguments','observed_guard':'observedGuard','computations':'computations'}

def canonical(value): return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def digest(data): return hashlib.sha256(data).hexdigest()
def fields(value,names,label): require(type(value) is dict and set(value)==set(names),label+' fields')
def same(a,b): return canonical(encode(a))==canonical(encode(b))
def raw_same(a,b): return canonical(a)==canonical(b)
def safe_path(root,name):
    require(type(name) is str and name!='' and '\\' not in name,'relative path string')
    path=PurePosixPath(name)
    require(not path.is_absolute() and all(p not in ('','..','.') for p in name.split('/')) and str(path)==name,'canonical relative path')
    resolved=root.resolve(); current=resolved
    for component in path.parts:
        current=current/component; require(not current.is_symlink(),'symlink forbidden')
    require(current.resolve().is_relative_to(resolved) and current.is_file(),'file containment')
    return current
def pin_bytes(root,pins):
    require(type(pins) is dict and pins,'nonempty pin map')
    out={}
    for name,sha in pins.items():
        require(type(sha) is str and re.fullmatch('[0-9a-f]{64}',sha),'SHA256 format')
        data=safe_path(root,name).read_bytes(); require(digest(data)==sha,'hash mismatch '+name); out[name]=data
    return out
def qnt_closure(root,entries):
    found=set(); todo=list(entries)
    while todo:
        name=todo.pop()
        if name in found: continue
        source=safe_path(root,name).read_text(); found.add(name)
        # Frozen files use relative from clauses. Refuse unsupported absolute imports.
        for suffix in re.findall(r'\bfrom\s+"([^"]+)"',source):
            require(suffix.startswith('./'),'unsupported Quint import path')
            child=str(PurePosixPath(name).parent/(suffix[2:]+'.qnt'))
            todo.append(child)
    return found
def inventory():
    return {'schema_version':2,'cases':[d|{'event_count':len(history(d))} for d in descriptors()]}

def shape(raw,expected):
    # Decode equality alone conflates Python bool/int and ITF tuple/list; reject both.
    encoded=encode(expected)
    if type(encoded) in (int,bool,str):
        if type(encoded) is int and type(raw) is dict and set(raw)=={'#bigint'}:
            require(type(decode(raw)) is int,'integer carrier'); return
        require(type(raw) is type(encoded),'scalar carrier'); return
    if isinstance(encoded,list):
        require(type(raw) is list and len(raw)==len(encoded),'ordered-list carrier')
        for a,b in zip(raw,expected): shape(a,b)
        return
    require(type(raw) is dict and set(raw)==set(encoded),'record/carrier keys')
    if isinstance(expected,M):
        require(type(raw['#map']) is list and len(raw['#map'])==len(expected.pairs),'map carrier')
        values={canonical(encode(k)):(k,v) for k,v in expected.pairs}
        for pair in raw['#map']:
            require(type(pair) is list and len(pair)==2,'map pair')
            k=decode(pair[0]); key=canonical(encode(k)); require(key in values,'map key set')
            ek,ev=values[key]
            if type(ek) is tuple:
                require(type(pair[0]) is dict and set(pair[0])=={'#tup'} and len(pair[0]['#tup'])==len(ek),'tuple map key')
                for rawkey,expectedkey in zip(pair[0]['#tup'],ek): shape(rawkey,expectedkey)
            else: shape(pair[0],ek)
            shape(pair[1],ev)
        return
    if isinstance(expected,frozenset):
        require(type(raw['#set']) is list and len(raw['#set'])==len(expected),'set carrier')
        values={canonical(encode(v)):v for v in expected}
        for x in raw['#set']:
            k=canonical(encode(decode(x))); require(k in values,'set member'); shape(x,values[k])
        return
    if isinstance(expected,V):
        require(type(raw['tag']) is str,'tag scalar')
        if expected.value==() and expected.tag!='EffectsExtractedA': require(raw['value']=={'#tup':[]},'unit carrier')
        else: shape(raw['value'],expected.value)
        return
    require(isinstance(expected,R),'record carrier')
    for name,value in expected.fields: shape(raw[name],value)

def compare_case(case,descriptor):
    fields(case,set(descriptor)|{'events'},'case')
    require(all(type(case[k]) is str and case[k]==v for k,v in descriptor.items()),'case descriptor')
    expected=history(descriptor); require(type(case['events']) is list and len(case['events'])==len(expected),'event count')
    for supplied,want in zip(case['events'],expected):
        fields(supplied,EVENT_FIELDS,'event')
        require(type(supplied['sequence']) is int,'exported sequence integer')
        for public,internal in RENAME.items():
            value=want['latest'][internal]; shape(supplied[public],value)
            require(same(decode(supplied[public]),value),'semantic event '+public)
        for boundary in ('before','after'):
            actual=typed(decode(supplied[boundary]),'Execution')
            # case-start reset before is checked against adjacent raw state by bind_event.
            if boundary=='before' and supplied['kind']=='case-start': continue
            shape(supplied[boundary],want[boundary])
            require(same(actual,want[boundary]),'semantic state '+boundary)
    return expected

def bind_event(event,raw,index,input_name,input_sha):
    p=event['provenance']; fields(p,('input_path','input_sha256','before_index','after_index'),'provenance')
    require(p['input_path']==input_name and p['input_sha256']==input_sha,'input pin linkage')
    require(type(p['before_index']) is int and type(p['after_index']) is int,'integer provenance index')
    require(p['after_index']==index and p['before_index']==max(0,index-1),'adjacent provenance indices')
    current=raw['states'][index]; previous=raw['states'][max(0,index-1)]
    for name,internal in RENAME.items():
        if name=='sequence':
            raw_sequence=decode(current['latestEvent'][internal])
            require(type(event[name]) is int and type(raw_sequence) is int and event[name]==raw_sequence,'raw provenance sequence')
            continue
        exported_raw=canonical(event[name]); raw_field=canonical(current['latestEvent'][internal])
        require(exported_raw == raw_field, 'raw provenance field '+name)
    require(raw_same(event['before'],previous['authorityState']) and raw_same(event['after'],current['authorityState']),'raw authority state linkage')

def check_document(document,admission,source_root,input_root,inventory_bytes):
    fields(document,('schema_version','inventory_sha256','source_pins','input_pins','receipt_pins','cases'),'document')
    fields(admission,('schema_version','inventory_sha256','source_pins','input_pins','receipt_pins','entries'),'admission')
    require(type(document['schema_version']) is int and document['schema_version']==2 and type(admission['schema_version']) is int and admission['schema_version']==2,'schema2')
    inv=inventory(); supplied_inventory=loads(inventory_bytes)
    require(raw_same(supplied_inventory,inv),'independent inventory mismatch')
    invhash=digest(canonical(inv).encode())
    require(document['inventory_sha256']==admission['inventory_sha256']==invhash,'inventory hash')
    require(admission['entries']==ENTRIES,'admitted entry modules')
    for group in ('source_pins','input_pins','receipt_pins'):
        require(raw_same(document[group],admission[group]),'admitted pin map '+group)
    sourcepins=document['source_pins']; expected_sources=qnt_closure(source_root,tuple(ENTRIES.values())+QNT_TESTS)|PYTHON_SOURCES
    require(set(sourcepins)==expected_sources,'complete source/tooling closure')
    pin_bytes(source_root,sourcepins)
    for name,pin in (core.PINNED|FROZEN_EXTRA).items(): require(sourcepins.get(name)==pin,'frozen source pin '+name)
    for module in (core,):
        name='scripts/check_s02_candidate_a_correspondence.py'; origin=inspect.getsourcefile(module)
        require(origin is not None and digest(Path(origin).read_bytes())==sourcepins[name],'imported Core bridge origin')
    for name in ('a4_carrier','a4_agreement','a4_authority','a4_cases','a4_inventory'):
        module=sys.modules['scripts.'+name]; origin=inspect.getsourcefile(module)
        require(origin is not None and digest(Path(origin).read_bytes())==sourcepins['scripts/'+name+'.py'],'imported checker origin '+name)
    require(digest(Path(__file__).read_bytes())==sourcepins['scripts/check_s02_candidate_a_integrated.py'],'executed checker origin')
    for fn in (core.compute_transaction,core.reduce_to_quiescence):
        origin=inspect.getsourcefile(fn); require(origin is not None and digest(Path(origin).read_bytes())==core.PINNED['moriarty/core.py'],'imported frozen Core origin')
    require(set(document['input_pins'])==RAW_NAMES,'complete raw input inventory')
    inputs=pin_bytes(input_root,document['input_pins']); pin_bytes(input_root,document['receipt_pins'])
    require(not (set(document['input_pins'])&set(document['receipt_pins'])),'receipt/input separation')
    ds=descriptors(); require(type(document['cases']) is list and len(document['cases'])==78,'complete case count')
    by_loop={loop:[] for loop in ENTRIES}
    for case,descriptor in zip(document['cases'],ds):
        compare_case(case,descriptor); by_loop[descriptor['lifecycle']].append(case)
    for loop,cases in by_loop.items():
        name=loop+'.itf.json'; raw=loads(inputs[name]); fields(raw,('#meta','vars','states'),'ITF')
        fields(raw['#meta'],('format','format-description','source','status','description','timestamp'),'ITF metadata')
        require(raw['#meta']['format']=='ITF' and raw['#meta']['source']==ENTRIES[loop],'ITF source/format metadata')
        require(raw['#meta']['format-description']=='https://apalache-mc.org/docs/adr/015adr-trace.html','ITF format description')
        require(type(raw['#meta']['timestamp']) is int and all(type(raw['#meta'][k]) is str for k in ('status','description')),'ITF metadata types')
        require(raw['vars']==list(VARS),'exact raw vars and order')
        require(type(raw['states']) is list,'ITF states')
        events=[event for case in cases for event in case['events']]
        require(len(raw['states'])==len(events),'all raw states accounted')
        index=0
        for case_index,case in enumerate(cases):
            for sequence,event in enumerate(case['events']):
                state=raw['states'][index]; fields(state,('#meta',*VARS),'ITF state')
                fields(state['#meta'],('index',),'ITF state metadata')
                require(state['#meta']['index']==index and type(state['#meta']['index']) is int,'raw state metadata index')
                require(type(decode(state['caseIndex'])) is int and decode(state['caseIndex'])==case_index,'raw case index')
                require(type(decode(state['cursor'])) is int and decode(state['cursor'])==sequence,'raw cursor')
                fields(state['latestEvent'],RENAME.values(),'latest event')
                bind_event(event,raw,index,name,document['input_pins'][name])
                if sequence==0:
                    require(event['kind']=='case-start' and (index==0 or events[index-1]['kind']=='case-end'),'reset boundary')
                    if index==0: require(raw_same(event['before'],event['after']),'initial before after identity')
                else:
                    require(event['kind']!='case-start','hidden reset')
                    require(raw_same(event['before'],events[index-1]['after']),'history continuity')
                index+=1
    return {'ok':True,'scope':'complete-inventory finite-record agreement-and-authority replay','cases':78,'events':1757,
            'symbolic_premises':['EvidenceValid','finite signature token abstraction'],
            'limitations':['not model checking','not cryptographic verification','root admission is not authenticated generator execution']}

def main():
    parser=argparse.ArgumentParser()
    for name in ('cases','admission','inventory','source-root','input-root'): parser.add_argument('--'+name,type=Path,required=True)
    parser.add_argument('--report',type=Path)
    args=parser.parse_args()
    try: report=check_document(loads(args.cases.read_bytes()),loads(args.admission.read_bytes()),args.source_root,args.input_root,args.inventory.read_bytes())
    except (Invalid,core.DecodeError,ValueError,TypeError,KeyError,IndexError,OSError) as error:
        report={'ok':False,'scope':'complete-inventory','differences':[str(error)]}
    if args.report is not None: args.report.write_text(json.dumps(report,sort_keys=True,indent=2)+'\n')
    print(json.dumps(report,sort_keys=True)); return 0 if report['ok'] else 1

if __name__=='__main__': raise SystemExit(main())
```

- [ ] Add this focused test and run `/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s02_candidate_a_integrated.py::test_binding_checks_observed_field`. Expected RED is missing rejection of the substituted profile; final expected result is terminal0.

```python
def test_binding_checks_observed_field():
    from copy import deepcopy
    from scripts.check_s02_candidate_a_integrated import bind_event,RENAME
    event={k:0 for k in RENAME}; event['profile']='SignBeforeResolve'
    event.update(before={},after={},provenance={'input_path':'swap.itf.json','input_sha256':'0'*64,'before_index':0,'after_index':0})
    raw={'states':[{'authorityState':{},'latestEvent':{v:event[k] for k,v in RENAME.items()}}]}
    bind_event(event,raw,0,'swap.itf.json','0'*64)
    bad=deepcopy(event); bad['profile']='SignAfterResolve'
    with pytest.raises(Invalid,match='raw provenance field profile'): bind_event(bad,raw,0,'swap.itf.json','0'*64)
```

- [ ] Root reviews the manifest boundary before integration: admitted receipt pins must include all original producer command/stdout/stderr/exit/source-closure records, not a reduced list chosen by the export. The checker verifies the exact root-admitted byte set; it does not reinterpret a textual receipt as proof of execution. Root's receipt audit remains necessary.
- [ ] Use `/home/charl/Moriarty/.venv/bin/python -m scripts.check_s02_candidate_a_integrated --cases evidence/s02-candidate-a-completion/a4/cases.json --admission evidence/s02-candidate-a-completion/a4/admission.json --inventory evidence/s02-candidate-a-completion/a4/inventory.json --source-root . --input-root evidence/s02-candidate-a-completion/a4 --report evidence/s02-candidate-a-completion/a4/checker-report.json`. Expected terminal0 with exactly 78 cases and 1757 events only after the independent producer delivers its frozen complete raw files. A missing producer artifact is a dependency, not permission to fabricate a passing export.

### Task 6: semantic/provenance mutant triples and final acceptance gate

**Files:** extend `tests/test_s02_candidate_a_integrated.py`. No producer or schema1 modifications.

**Interfaces:** tests invoke `compare_case` for semantic rejection and `bind_event` for raw linkage independently. Synthetic unit cases are explicitly not CLI-accepted complete packages. For each semantic mutant the test rebuilds adjacent raw states, sequence, raw hash and all event provenance links before semantic comparison. Thus a semantic mutant cannot pass merely because its old checksum was rejected. Corrected counterpart and unrelated honest case must both pass. The full producer package remains a separate mandatory end-to-end gate.

- [ ] Add the following full test code. First retain Task5's omitted-field-check RED against `test_raw_latest_field_substitution_triple`; it must fail with “DID NOT RAISE”. Restore the single provenance comparison and capture terminal GREEN. Then run all mutation triples and the complete actual producer package; do not declare a semantic control effective if only provenance rejected it.

```python
from copy import deepcopy
from scripts.a4_carrier import loads
from scripts.check_s02_candidate_a_integrated import (canonical,digest,compare_case,bind_event,RENAME,VARS,ENTRIES,safe_path,shape)

def unit_case(descriptor):
    case=descriptor|{'events':[]}
    for event in history(descriptor):
        latest=event['latest']
        value={public:encode(latest[internal]) for public,internal in RENAME.items()}
        value.update(before=encode(event['before']),after=encode(event['after']),provenance={})
        case['events'].append(value)
    return rebind_unit(case)

def rebind_unit(case):
    # Synthetic stage-control raw bytes only: not a CLI complete package or runtime receipt.
    case=deepcopy(case); loop=case['lifecycle']; states=[]
    for index,e in enumerate(case['events']):
        e['sequence']=index
        if index: e['before']=deepcopy(case['events'][index-1]['after'])
        else: e['before']=deepcopy(e['after'])
        states.append({'#meta':{'index':index},'authorityState':deepcopy(e['after']),
            'latestEvent':{internal:deepcopy(e[public]) for public,internal in RENAME.items()},'caseIndex':0,'cursor':index})
    raw={'#meta':{'format':'ITF','source':ENTRIES[loop],'description':'synthetic unit control; not generator evidence'},'vars':list(VARS),'states':states}
    sha=digest(canonical(raw).encode()); name=loop+'.itf.json'
    for index,e in enumerate(case['events']): e['provenance']={'input_path':name,'input_sha256':sha,'before_index':max(0,index-1),'after_index':index}
    return case,raw,sha

def bound(case,raw,sha):
    for index,event in enumerate(case['events']): bind_event(event,raw,index,case['lifecycle']+'.itf.json',sha)

def descriptor(loop,scenario,control='ordinary',profile='SignBeforeResolve'):
    return next(d for d in descriptors() if (d['lifecycle'],d['scenario'],d['control'],d['profile'])==(loop,scenario,control,profile))

def rewrite(value,rule):
    if type(value) is dict: value={k:rewrite(v,rule) for k,v in value.items()}
    elif type(value) is list: value=[rewrite(v,rule) for v in value]
    return rule(value)

def mutated(case,name):
    original=deepcopy(case); case=deepcopy(case)
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
    if name=='concealed-rejection':
        index=next(i for i,e in enumerate(case['events']) if e['kind']=='transition' and e['arguments']['command']['tag'].startswith('Reject'))
        del case['events'][index]
    elif name=='omitted-computation':
        event=next(e for e in case['events'] if e['computations']); event['computations']=[]
    elif name=='fabricated-cancellation-core':
        event=next(e for e in case['events'] if e['arguments']['command']['tag']=='ProposeA4' and e['arguments']['command']['value']['operation']['tag']=='OpCancelParent')
        event['computations']=deepcopy(next(e['computations'] for e in case['events'] if e['computations']))
    elif name=='denial-as-transition': next(e for e in case['events'] if e['kind']=='denied-probe')['kind']='transition'
    elif name=='observed-guard': next(e for e in case['events'] if e['kind']=='denied-probe')['observed_guard']=True
    elif name=='event-order': case['events'][1],case['events'][2]=case['events'][2],case['events'][1]
    else: case=rewrite(case,rule)
    require(canonical(case)!=canonical(original),'effective mutation '+name)
    return case

SEMANTIC_MUTANTS=('optional-zero','nonce','signer','token','revision','parent-paid','rejection-reason','rejection-stage',
    'raw-reductions','claimed-reductions','effects-order','request-chooser','plan-omission','unused-program','signing-snapshot',
    'concealed-rejection','omitted-computation','fabricated-cancellation-core','denial-as-transition','observed-guard','event-order',
    'payment-order','deposit-effect','rollback-time','retained-loser','proof-context','cancellation-identity')

@pytest.mark.parametrize('name',SEMANTIC_MUTANTS)
def test_semantic_rebound_triples(name):
    d=descriptor('swap','funded2-settle') if name in ('effects-order','payment-order','deposit-effect') else descriptor('installment','recover-r1-refuse100' if name=='rollback-time' else 'recover-r1-choice2')
    original,raw,sha=unit_case(d)
    bad,badraw,badsha=rebind_unit(mutated(original,name))
    bound(bad,badraw,badsha)  # Checksum, adjacent indices and every mirrored raw event agree.
    with pytest.raises(Invalid): compare_case(bad,d)
    corrected,correctedraw,correctedsha=rebind_unit(original)
    bound(corrected,correctedraw,correctedsha); compare_case(corrected,d)
    other_d=descriptor('swap','funded2-refund','ordinary','SignAfterResolve')
    other,otherraw,othersha=unit_case(other_d)
    bound(other,otherraw,othersha); compare_case(other,other_d)

def test_raw_latest_field_substitution_triple():
    d=descriptor('swap','funded2-settle'); original,raw,sha=unit_case(d)
    bad=deepcopy(original); bad['events'][0]['profile']='SignAfterResolve'
    with pytest.raises(Invalid,match='raw provenance field profile'): bind_event(bad['events'][0],raw,0,'swap.itf.json',sha)
    bound(original,raw,sha); compare_case(original,d)
    other_d=descriptor('installment','two-fills'); other,otherraw,othersha=unit_case(other_d)
    bound(other,otherraw,othersha); compare_case(other,other_d)

@pytest.mark.parametrize('field,value',(('before_index',99),('after_index',99),('after_index',True),('input_path','../swap.itf.json'),('input_sha256','0'*64)))
def test_provenance_locator_triples(field,value):
    d=descriptor('swap','funded2-settle'); original,raw,sha=unit_case(d); bad=deepcopy(original)
    bad['events'][1]['provenance'][field]=value
    with pytest.raises(Invalid): bind_event(bad['events'][1],raw,1,'swap.itf.json',sha)
    bound(original,raw,sha)
    other_d=descriptor('installment','two-fills'); other,otherraw,othersha=unit_case(other_d); bound(other,otherraw,othersha)

def test_duplicate_json_map_set_and_unit_controls():
    with pytest.raises(Invalid): loads('{"schema_version":2,"schema_version":2}')
    with pytest.raises(Invalid): decode({'#set':[1,1]})
    with pytest.raises(Invalid): decode({'#map':[[1,0],[1,0]]})
    with pytest.raises(Invalid): shape({'tag':'NoInt','value':[]},V('NoInt'))
    shape({'tag':'NoInt','value':{'#tup':[]}},V('NoInt'))
    shape({'tag':'IntValue','value':{'#bigint':'0'}},V('IntValue',0))

def test_path_traversal_and_symlink_triples(tmp_path):
    target=tmp_path/'honest.json'; target.write_text('{}')
    alias=tmp_path/'alias.json'; alias.symlink_to(target)
    with pytest.raises(Invalid): safe_path(tmp_path,'../honest.json')
    with pytest.raises(Invalid): safe_path(tmp_path,'alias.json')
    assert safe_path(tmp_path,'honest.json')==target
    other=tmp_path/'other.json'; other.write_text('[]'); assert safe_path(tmp_path,'other.json')==other
```

- [ ] Add explicit complete-package controls after the honest producer files exist. Each control starts from those admitted actual bytes; the test code below supplies complete mutations for inventory and source closure checks, then rechecks the unmodified package. These controls cannot be run by passing synthetic subset cases to the CLI.

```python
from pathlib import Path
from scripts.check_s02_candidate_a_integrated import check_document

def test_actual_complete_package_and_inventory_triples():
    root=Path(__file__).resolve().parents[1]
    folder=root/'evidence/s02-candidate-a-completion/a4'
    require((folder/'cases.json').is_file(),'actual A4 package required; no skip')
    doc=loads((folder/'cases.json').read_bytes()); admission=loads((folder/'admission.json').read_bytes()); inv=(folder/'inventory.json').read_bytes()
    assert check_document(doc,admission,root,folder,inv)['events']==1757
    for name in ('missing-case','duplicate-case','extra-case','missing-transitive-pin','changed-frozen-pin','missing-input','changed-receipt'):
        bad=deepcopy(doc); admitted=deepcopy(admission)
        if name=='missing-case': bad['cases'].pop()
        elif name=='duplicate-case': bad['cases'][-1]=deepcopy(bad['cases'][0])
        elif name=='extra-case': bad['cases'].append(deepcopy(bad['cases'][0]))
        elif name=='missing-transitive-pin':
            path='specs/quint/s02/consumption.qnt'; del bad['source_pins'][path]; del admitted['source_pins'][path]
        elif name=='changed-frozen-pin':
            path='specs/quint/s02/consumption.qnt'; bad['source_pins'][path]='0'*64; admitted['source_pins'][path]='0'*64
        elif name=='missing-input':
            del bad['input_pins']['swap.itf.json']; del admitted['input_pins']['swap.itf.json']
        else:
            path=next(iter(bad['receipt_pins'])); bad['receipt_pins'][path]='0'*64
        with pytest.raises(Invalid): check_document(bad,admitted,root,folder,inv)
        assert check_document(doc,admission,root,folder,inv)['ok'] is True
    # Separate unrelated honest case control does not masquerade as full acceptance.
    d=descriptor('installment','two-fills'); control,raw,sha=unit_case(d); bound(control,raw,sha); compare_case(control,d)
```

- [ ] During Tasks1–5 use `/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s02_candidate_a_integrated.py -k 'not test_actual_complete_package_and_inventory_triples'` once the integration test is present. At final Task6 gate run `/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s02_candidate_a_integrated.py`, with no deselection/skip of actual-package checks. Capture both original source closure and terminal results for all RED/GREEN runs.
- [ ] Root runs shared schema1/Python regressions using the existing accepted command inventory, without modifying schema1 files. Root archives the full checker/producer/import/tool/test source pins, complete raw inputs, admitted inventory, original receipts, JSON report, mutation test report and non-author source review. Expected checker acceptance is finite-record agreement and independent authority replay only.
- [ ] After each task's source and runtime gate, commit only that task's explicit new checker paths plus test file; never `git add -A`. Use commit messages `feat(a4): add strict independent carriers`, `feat(a4): replay authority boundaries independently`, `feat(a4): enumerate independent lifecycle expectations`, `feat(a4): require full negative event inventory`, `feat(a4): bind complete raw authority histories`, and `test(a4): reject rebound semantic and provenance mutants`, respectively. Root owns final integration and broader acceptance; this plan does not authorize those during drafting.

Exact future task-local commit commands, one block per accepted task:

```bash
git add scripts/a4_carrier.py scripts/a4_agreement.py tests/test_s02_candidate_a_integrated.py
git commit -m "feat(a4): add strict independent carriers"
```

```bash
git add scripts/a4_authority.py tests/test_s02_candidate_a_integrated.py
git commit -m "feat(a4): replay authority boundaries independently"
```

```bash
git add scripts/a4_cases.py tests/test_s02_candidate_a_integrated.py
git commit -m "feat(a4): enumerate independent lifecycle expectations"
```

```bash
git add scripts/a4_inventory.py tests/test_s02_candidate_a_integrated.py
git commit -m "feat(a4): require full negative event inventory"
```

```bash
git add scripts/check_s02_candidate_a_integrated.py tests/test_s02_candidate_a_integrated.py
git commit -m "feat(a4): bind complete raw authority histories"
```

```bash
git add tests/test_s02_candidate_a_integrated.py
git commit -m "test(a4): reject rebound semantic and provenance mutants"
```

## Acceptance map and claim limits

- A4-R01: Tasks3–5 fix 78 cases/1757 observed events and require actual full raw ITF coverage, actual current/retained states, exact prepare/sign/propose/verify/commit/reject/cancel calls and all required probes.
- A4-R02: Tasks1/3/4 recompute actual raw Core outcomes, optional choices, full symbolic programs, ordered payments/effects and cancellation identity. Observed second deterministic evaluation is separate from claimed projection; plan fidelity is not future reachability.
- A4-R03: Task2 independently implements common/A guard/update and rejection precedence, original nonce0 parent history, nonce1 recovery, dependency snapshots and exact evidence premises. Task4 pins exact lifecycle guards and required negative derivations.
- A4-R04: Tasks5/6 enforce closed inventory, source/import/tool pins, raw adjacency, full event continuity, strict ITF carriers and rebound mutation triples. Root admission and cryptographic premises are explicit trust boundaries.
- A4-R05: Task6 requires honest package acceptance, behavioral RED/GREEN, semantic and provenance mutation rejection with corrected and unrelated controls, and separate non-author review. No model-checking, exhaustive correctness, cryptographic validity, Council, A4 final acceptance or integration is claimed from this plan.
