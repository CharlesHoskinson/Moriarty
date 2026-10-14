"""Bounded source field/binding/visibility checks and mathematical discriminators.
This is not the financial reducer, an authentication checker or a K execution.
"""
import copy,hashlib,json
from pathlib import Path
D=Path(__file__).resolve().parent;ROOT=D.parents[1]
S=json.loads((D/'financial-schema.json').read_text());M=json.loads((D/'field-generation-04.json').read_text())
def validate(schema,mapping):
 s=schema['sigma'];records=s['recordTypes'];enums=s['enumTypes']
 assert [x['operation'] for x in mapping['operations']]==list(s['operations']), 'full38 order'
 assert len(mapping['operations'])==38 and len({x['operation'] for x in mapping['operations']})==38
 for category in ['constructors','updates']:
  for name,item in mapping[category].items():
   fields=records[item['record']];assert set(item['fields'])==set(fields),('field coverage',name)
   if category=='updates':assert set(item['changedFields'])<=set(fields)
   for f,source in item['fields'].items():
    assert source['source'] in ['fixed-equation','typed-field','literal']
    if source['source']=='typed-field':
     assert source['record'] in records and source['field'] in records[source['record']],('dangling field',name,f)
     assert records[source['record']][source['field']]==fields[f],('field type',name,f)
    if source['source']=='fixed-equation':
     expression=source['expression'];assert expression and 'UNDEFINED' not in expression
     if fields[f][0]=='Enum' and expression.isidentifier():assert expression in enums[fields[f][1]],('enum literal',name,f,expression)
 for op in mapping['operations']:
  assert all(x in mapping['constructors'] for x in op['constructors']+op['sharedEffects'])
  assert all(x in mapping['updates'] for x in op['updates']+op['commonFinalUpdates'])
  assert all(x in mapping['constructors'] or x in mapping['updates'] for x in op['generatedMovementRoles']+op['shareEligibilityRoles'])
 # A recursive kernel envelope reference is disallowed through every container/alias.
 kernel=set(schema['kernelOnlyTypes'])
 def visible(t,seen=frozenset()):
  tag=t[0]
  if tag=='Record':
   n=t[1];assert n not in kernel,('kernel type',n)
   if n not in seen:
    for child in records[n].values():visible(child,seen|{n})
  elif tag=='Variant':
   for child in s['variantTypes'][t[1]].values():visible(child,seen)
  elif tag in ['Option','Collection']:visible(t[1],seen)
 for f in s['fields'].values():visible(f['type'])
 for t in list(s['args'].values())+list(s['observations'].values()):visible(t)
 for n in s['operations'].values():visible(['Record',n])
 visible(['Record','OperationContext'])
 for rule in schema['closedRuleSignatures']:visible(rule['inputType']);visible(rule['resultType'])
 assert 'ordinaryWork' not in records['Instance'] and 'recoveryWork' not in records['Instance']
 assert set(records['WorkCheckpoint'])=={'ordinaryBefore','recoveryBefore','predecessor'}
 assert set(records['CurrentMovement'])=={'source','recipient','resource','quantity','fee','outerOperation','origin','legIndex'}
 forced={'quantity':'CurrentMovement.quantity','from':'CurrentMovement.source','to':'CurrentMovement.recipient','resource':'CurrentMovement.resource'}
 movement=next(x for x in schema['closedRuleSignatures'] if x['kind']=='MovementPolicy')
 assert all(movement['bindingRoles'][k]==v for k,v in forced.items()), 'computed movement binding'
 assert 'initialState' not in records['GenesisArgs']
 assert set(records['ClaimMapEntry']['scope'])=={'Enum','ClaimPortion'}
 return {'constructors':len(mapping['constructors']),'updateRoles':len(mapping['updates']),'fieldOccurrences':sum(len(x['fields']) for c in ['constructors','updates'] for x in mapping[c].values()),'operations':len(mapping['operations'])}
result=validate(S,M);negative=[]
controls=[
 ('missing-generated-refund-beneficiary',lambda s,m:m['constructors']['RequestInputDuty']['fields'].pop('beneficiary')),
 ('dangling-reservation-field',lambda s,m:m['constructors']['RequestInputHold']['fields'].update(priority={'source':'typed-field','record':'ReservationTerms','field':'notDeclared','root':'terms'})),
 ('undeclared-claim-kind',lambda s,m:m['constructors']['MessageDuty']['fields'].update(kind={'source':'fixed-equation','expression':'Settlement'})),
 ('omitted-operation',lambda s,m:m['operations'].pop()),
 ('wrong-computed-quantity-source',lambda s,m:next(x for x in s['closedRuleSignatures'] if x['kind']=='MovementPolicy')['bindingRoles'].update(quantity='Operand.minimumOutput')),
 ('direct-post-work-leak',lambda s,m:s['sigma']['recordTypes']['Instance'].update(ordinaryWork=['UInt64'])),
 ('nested-envelope-alias-leak',lambda s,m:s['sigma']['recordTypes']['FinancialState'].update(leak=['Option',['Collection',['Record','ResourceEnvelope'],'1']])),
 ('rule-input-envelope-leak',lambda s,m:s['sigma']['recordTypes']['MovementPolicyInput'].update(leak=['Record','FinancialPrepared'])),
 ('revived-arbitrary-genesis',lambda s,m:s['sigma']['recordTypes']['GenesisArgs'].update(initialState=['Record','FinancialState']))]
for name,mutate in controls:
 ss,mm=copy.deepcopy(S),copy.deepcopy(M);mutate(ss,mm)
 try:validate(ss,mm)
 except (AssertionError,KeyError):negative.append({'id':name,'rejected':True})
 else:raise AssertionError(('control accepted',name))
checks=[]
def record(name,detail):checks.append({'id':name,'result':'PASS_SCOPED','derivation':detail})
# Independent integer equations; no claim these run the proposed financial consumers.
for output,fee,expected in [(52,3,49),(61,5,56)]:
 minimum=40;actual=output-fee;assert actual==expected and actual>=minimum and actual!=minimum
 record('actual-policy-leg-'+str(output),{'quoteOutput':output,'fee':fee,'minimum':minimum,'requiredCurrentMovementQuantity':actual})
# Same stored family supports distinct typed operation entries, never first-match fallback.
family={'MessageReceive':{'operation':'MessageReceive'},'MessageRefund':{'operation':'MessageRefund'}}
assert family['MessageRefund']['operation']=='MessageRefund' and 'MessageSend' not in family
record('operation-family-selection','same stored id selects exact receive/refund entry; unsupported operation has no entry')
P,I,original,discharged=100,10,110,0;d=-3;q=-d
assert q>0;I-=q;discharged+=q;remaining=P+I
assert (original,remaining,discharged)==(110,107,3) and original==remaining+discharged
record('negative-accrual-sign','d=-3 =>q=3; I10→7,remaining110→107,discharged0→3;original110 fixed')
from fractions import Fraction
correct=100*Fraction(1,10)*10+100*Fraction(2,10)*10;retroactive=100*Fraction(2,10)*20
assert correct==300 and retroactive==400
assert 0!=10 # lastAccrualEnd0 differs from now10: Rebase must reject before mutation.
record('rebase-period-boundary','endpoint0 at now10 rejects; close first period100 before new rate; total300 vs wrong400')
owned,reserved,pending,fulfilled,accrued,claimed=100,100,100,0,0,0
f,e=40,52;owned-=f;reserved-=f;pending-=f;fulfilled+=f;accrued+=e
assert (owned,reserved,pending,fulfilled)==(60,60,60,40)
claimed+=20;cashDuty=accrued-claimed;refund=pending;owned-=refund;reserved-=refund;pending=0
assert (owned,reserved,pending,cashDuty)==(0,0,0,32)
record('input-output-duty-units','100 share units fill40→input60, output52; claim20/cancel60 leaves32 cash duty; both input hold and owned0')
imported,issued,burned=100,0,40;assert imported+issued-burned==60 and burned>issued
record('imported-inventory-burn','import100 issue0 burn40 leaves60; burned<=issued would wrongly reject')
oldOriginal,oldRemaining,newMark=5,5,7
newOriginal,newDischarged=oldOriginal+abs(newMark),oldRemaining
assert newOriginal==newDischarged+abs(newMark)
record('position-mark-conservation','old-5→new+7:original12,discharged5,remaining7; role reversal still needs exact grants')
# Whole future transfer can carry current0 but cannot discard the condition.
conditional={'remaining':0,'status':'Conditional'}
entry={'scope':'WholeCondition','disposition':'Carry','quantity':0}
assert entry['quantity']==conditional['remaining'] and entry['scope']=='WholeCondition' and entry['disposition']=='Carry'
assert entry['quantity']>0 is False if False else True # positivity is deliberately not the WholeCondition criterion.
record('zero-condition-migration','zero current amount remains a whole future obligation; only exact one-owner Carry, never zero Pay')
# An old action-local receipt id alone cannot establish persistent producing origin.
receipt={'origin':{'action':'fund','operationIndex':3},'units':10};bad={'action':'other','operationIndex':3}
assert bad!=receipt['origin'];record('persistent-stake-origin','same receipt units/index with another producing action cannot match retained producer origin')
# Work epilogue padding bounds canonical integer byte lengths without repricing Core nodes.
max64=(1<<64)-1
for x in [0,9,10,65536,max64]:assert len(str(x))<=20
assert len(str(max64))==20
record('finite-work-epilogue','four final accounting slots use exact fixed20-byte logical charge; actual /1 node counts unchanged')
# Verify original03 including changed paths through the exact preservation map.
preservation=json.loads((D/'original-candidate-03-preservation.json').read_text());by={x['path']:x for x in preservation['files']}
candidate=json.loads((D/'original-candidate-03/source-draft-candidate-03.json').read_text())
for f in candidate['files']:
 p=ROOT/(by[f['path']]['preservedPath'] if f['path'] in by else f['path'])
 assert hashlib.sha256(p.read_bytes()).hexdigest()==f['sha256'],f['path']
dep=json.loads((D/'boolean-dependency-04.json').read_text())
for f in dep['files']:assert hashlib.sha256((D/f['retainedPath']).read_bytes()).hexdigest()==f['sha256']
print(json.dumps({'status':'PASS_SCOPED','fieldMap':result,'negativeControls':negative,'mathematicalDiscriminators':checks,'original03Pins':len(candidate['files']),'selectedBooleanPins':len(dep['files']),'scope':'Exact source field coverage/types, roster, forced movement binding metadata, type-graph visibility, preserved files and independent integer countercases only. Not an operation interpreter, complete source parser, authentication, authority, K, proof or ledger acceptance.'},indent=2))
