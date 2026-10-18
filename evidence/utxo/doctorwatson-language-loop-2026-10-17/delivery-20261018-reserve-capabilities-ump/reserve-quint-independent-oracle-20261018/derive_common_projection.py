"""Independent literal projection/notation assembly, never executing Mori or Quint."""
from pathlib import Path
from copy import deepcopy
import json,hashlib,re
P=Path(__file__).resolve().parent
HANDOFF=P.parent/'quint-current-toolchain-custody-20261018/input-only/COMMON-PROJECTION-CODEC.json'
codec=HANDOFF.read_bytes();(P/'COMMON-PROJECTION-CODEC.json').write_bytes(codec)
def put(name,v):(P/name).write_text(json.dumps(v,sort_keys=True,indent=2)+'\n')
# Typed mathematical values independently calculated from the source/scenario.
bal0={'observer':42,'reserve':100,'target':0,'treasury':0}
bal1={'observer':42,'reserve':58,'target':42,'treasury':0}
bal2={'observer':42,'reserve':56,'target':42,'treasury':2}
st0={'allocated':0,'basis':40,'marker':77}
st1={'allocated':0,'basis':35,'marker':77}
st2={'allocated':42,'basis':78,'marker':77}
initial={'specimen':'Reserve','inputValue':2,'balances':bal0,'programState':st0,
 'dutyPresent':True,'dutyAmount':5,'pendingSite':0,'captureValues':[],
 'logicalAttempted':0,'logicalCommitted':0,'effects':[]}
prefix={'specimen':'Reserve','inputValue':2,'balances':bal0,'programState':st1,
 'dutyPresent':True,'dutyAmount':47,'pendingSite':1,'captureValues':[2,42],
 'logicalAttempted':11,'logicalCommitted':11,
 'effects':[{'tag':'StateChange','before':40,'after':35},{'tag':'LiabilityChange','before':5,'after':47}]}
def pending(balances,states,amount,site,captures,committed):return {'specimen':'Reserve',
 'balances':balances,'programState':states,'dutyPresent':True,'dutyAmount':amount,
 'site':site,'captures':captures,'committed':committed}
c1=pending(bal0,st1,47,1,[2,42],11)
c2=pending(bal1,st2,5,2,[2,42,43],26)
c3=pending(bal2,st2,5,0,[],36)
def invocation(status,attempted,advanced,segment,new_cont,effects,steps):return {
 'status':status,'attempted':attempted,'advanced':advanced,'hasSegment':segment,
 'newContinuation':new_cont,'effects':effects,'sourceSteps':steps,'errorSteps':[]}
empty=invocation('NotInvoked',0,False,False,False,[],[])
false1=invocation('Waiting',4,False,False,False,[],[])
advance=invocation('Waiting',15,True,True,True,[
 {'tag':'StateValue','name':'allocated','before':0,'after':42},
 {'tag':'TransferAmount','amount':42},
 {'tag':'DutyValue','before':47,'after':5},
 {'tag':'StateValue','name':'basis','before':35,'after':78}],[3,4,5,6,7])
false2=invocation('Waiting',2,False,False,False,[],[])
complete=invocation('Complete',10,True,True,False,[{'tag':'FeeAmount','amount':2},{'tag':'EmitQuote','quote':43}],[8,9,10,11])
observations=[{'c':c1,'last':empty,'attemptedTotal':11},
 {'c':c1,'last':false1,'attemptedTotal':15},
 {'c':c2,'last':advance,'attemptedTotal':30},
 {'c':c2,'last':false2,'attemptedTotal':32},
 {'c':c3,'last':complete,'attemptedTotal':42}]
whole={'initial':initial,'prefix':prefix,'observations':observations}
put('reserve-common-projection.expected.json',whole)
put('initial.financial-frame.expected.json',initial)
put('prefix.financial-frame.expected.json',prefix)
for name,value in zip(['prefix-init','approval41','approval43','settlement-false','settlement-true'],observations):put(name+'.observation.expected.json',value)
# A typed Quint notation artifact avoids pretending the authored common JSON codec
# is the actual Quint ITF serialization. This is not a runnable module/test.
def q(v,field=None):
 if isinstance(v,bool):return str(v).lower()
 if isinstance(v,int):return str(v)
 if isinstance(v,str):return v if field in ['specimen','status'] else json.dumps(v)
 if isinstance(v,list):return 'List('+', '.join(q(x) for x in v)+')'
 if 'tag' in v:
  tag=v['tag'];other={k:x for k,x in v.items() if k!='tag'}
  if tag in ['Transfer','TransferAmount','FeeAmount']:payload=str(other['amount'])
  elif tag=='EmitQuote':payload=str(other['quote'])
  else:payload=q(other)
  return tag+'('+payload+')'
 if field in ['balances','programState']:
  return 'Map('+', '.join(json.dumps(k)+' -> '+str(v[k]) for k in sorted(v))+')'
 return '{ '+', '.join(k+': '+q(val,k) for k,val in v.items())+' }'
(P/'MODEL-TYPED-VALUES.qnt.literal').write_text('// Documentary pure-value fragments, not a module, test or actual trace.\n'
 + 'pure val reserveInitial: FinancialFrame = '+q(initial)+'\n\n'
 + 'pure val reservePrefix: FinancialFrame = '+q(prefix)+'\n\n'
 + 'pure val reserveObservations: List[Observation] = '+q(observations)+'\n')
f=P/'INDEPENDENT-FINANCIAL-WORK.json';facts=json.loads(f.read_text());facts['status']='Complete independent finite source facts; shared JSON representation conditional on authored COMMON-PROJECTION-CODEC.json, not Quint ITF';facts['start_cap']='36';facts['resume_overrides']='None; preserve stored cap36 in actual Rust continuation when present';put(f.name,facts)
# Keep first public DTO occurrence; duplicate nested historical DTO declarations
# incidentally displayed during extraction add no implementation behavior premise.
f=P/'basis/public-ir.rs.txt';src=f.read_text();seen=set();parts=[]
for block in src.strip().split('\n\n'):
 m=re.match(r'pub (?:struct|enum) (\w+)',block)
 if m and m.group(1) not in seen:parts.append(block);seen.add(m.group(1))
f.write_text('\n\n'.join(parts)+'\n')
# Complete exact key sets from schema; immutable maps may be shared in assembly,
# but serialized expected objects contain independent full values.
C=json.loads(codec)
for frame in [initial,prefix]:assert set(frame)==set(C['FinancialFrame'])
for o in observations:
 assert set(o)==set(C['Observation']) and set(o['c'])==set(C['PendingState']) and set(o['last'])==set(C['Invocation'])
 assert set(o['c']['balances'])=={'observer','reserve','target','treasury'}
 assert set(o['c']['programState'])=={'allocated','basis','marker'}
 assert sum(o['c']['balances'].values())==142 and o['c']['dutyPresent'] is True
assert observations[0]['c']==observations[1]['c'] and observations[2]['c']==observations[3]['c']
assert [o['attemptedTotal'] for o in observations]==[11,15,30,32,42]
assert [o['c']['committed'] for o in observations]==[11,11,26,26,36]
assert [o['last']['attempted'] for o in observations]==[0,4,15,2,10]
assert observations[-1]['last']['sourceSteps']==[8,9,10,11]
print(json.dumps({'common_codec_sha256':hashlib.sha256(codec).hexdigest(),'whole_projection_sha256':hashlib.sha256((P/'reserve-common-projection.expected.json').read_bytes()).hexdigest(),'static':'all literal typed keysets/finite equations checked; actual ITF/consumer UNRUN'},sort_keys=True))
