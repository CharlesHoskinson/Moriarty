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
