"""Validate this closed source schema and arithmetic encodings, not financial execution."""
import copy,hashlib,json
from pathlib import Path
D=Path(__file__).resolve().parent; R=D.parents[1]
def canonical(x):return json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
def jsonnodes(x):return 1+sum(jsonnodes(v) for v in x.values()) if isinstance(x,dict) else 1+sum(jsonnodes(v) for v in x) if isinstance(x,list) else 1
def check(schema):
 s=schema['sigma'];records=s['recordTypes'];enums=s['enumTypes'];variants=s['variantTypes'];edges={('Record',x):set() for x in records}|{('Variant',x):set() for x in variants}
 primitives={'UInt64','UInt128','UInt256','SInt128','Bool','Text'}
 def typecheck(t,owner=None):
  assert isinstance(t,list) and t and isinstance(t[0],str),('type',t)
  k=t[0]
  if k in primitives:assert len(t)==1;return
  if k in ('Record','Enum','Variant'):
   assert len(t)==2 and t[1] in {'Record':records,'Enum':enums,'Variant':variants}[k],('dangling',t)
   if owner is not None and k!='Enum':edges[owner].add((k,t[1]))
  elif k=='Option':assert len(t)==2;typecheck(t[1],owner)
  elif k=='Collection':assert len(t)==3 and str(int(t[2]))==t[2] and 0<=int(t[2])<=128;typecheck(t[1],owner)
  elif k in ('Amount','NetAmount','SignedAmount'):assert len(t)==2 and t[1] in s['assets']
  elif k=='Shares':assert len(t)==3 and t[1] in s['vaults'] and t[2] in s['parties']
  elif k=='Rate':assert len(t)==2 and str(int(t[1]))==t[1] and 0<=int(t[1])<=18
  elif k=='Price':assert len(t)==4 and t[1] in s['assets'] and t[2] in s['assets'] and t[1]!=t[2] and 0<=int(t[3])<=18
  else:raise AssertionError(('unknown',t))
 assert set(s)=={'units','assets','vaults','parties','recordTypes','enumTypes','variantTypes','fields','args','observations','operations'}
 for roster in ['units','assets','vaults','parties']:assert s[roster]==sorted(set(s[roster]))
 for n,fields in records.items():
  assert 0<len(fields)<=64
  for typ in fields.values():typecheck(typ,('Record',n))
 for n,members in enums.items():assert members==sorted(set(members)) and 0<len(members)<=128
 for n,choices in variants.items():
  assert 0<len(choices)<=128 and len({canonical(t) for t in choices.values()})==len(choices)
  for typ in choices.values():typecheck(typ,('Variant',n))
 for op,name in s['operations'].items():assert name in records
 for typ in [x['type'] for x in s['fields'].values()]+list(s['args'].values())+list(s['observations'].values()):typecheck(typ)
 visiting=set();done=set()
 def visit(x):
  assert x not in visiting,('cycle',x)
  if x in done:return
  visiting.add(x)
  for y in edges[x]:visit(y)
  visiting.remove(x);done.add(x)
 for x in edges:visit(x)
 for rule in schema['closedRuleSignatures']:typecheck(rule['inputType']);typecheck(rule['resultType'])
 assert len(canonical(s))<=65536 and jsonnodes(s)<=4096
 assert len(records)+len(enums)+len(variants)<=256
 return {'records':len(records),'enums':len(enums),'variantFamilies':len(variants),'sigmaCanonicalBytes':len(canonical(s)),'sigmaJsonNodes':jsonnodes(s)}
j=json.loads((D/'financial-schema.json').read_text()); result=check(j)
bind=json.loads((D/'financial-operation-bindings.json').read_text())['operations']
old=json.loads((R/'evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/successor/candidate-04-partial/experiments/moriarty-language/spec/successor/signing-display-schema.json').read_text())['$defs']['CoreOp']['enum']
assert [x['operation'] for x in bind]==old
assert all(j['sigma']['operations'][x['operation']]==x['argumentType'][1] and x['effectType']==['Record','OperationEffect'] for x in bind)
assert {x for q in j['closedRuleSignatures'] for x in q['operations']}==set(old)
negative=[]
for name,mutate in [
 ('missing-debt-type',lambda x:x['sigma']['recordTypes'].pop('Debt')),
 ('unknown-rule-result',lambda x:x['closedRuleSignatures'][0].update(resultType=['Record','MissingResult'])),
 ('recursive-variant',lambda x:x['sigma']['variantTypes']['FinancialQuantity'].update(Recursive=['Variant','FinancialQuantity'])),
 ('wrong-share-holder',lambda x:x['sigma']['variantTypes']['FinancialQuantity'].update(Wrong=['Shares','Vault01','MissingHolder'])),
 ('oversized-record',lambda x:x['sigma']['recordTypes'].update(TooMany={str(i):['UInt128'] for i in range(65)})),
 ('unknown-Any-type',lambda x:x['sigma']['recordTypes']['Debt'].update(anything=['Any'])),
]:
 x=copy.deepcopy(j);mutate(x)
 try:check(x)
 except AssertionError:negative.append({'id':name,'rejected':True})
 else:raise AssertionError('mutation accepted: '+name)
# Compare actual representations of equivalent family choices; scalar payload5.
comparisons=[]
for count in (8,64,128):
 names=[f'T{i:03}' for i in range(count)]
 variant={'tag':names[0],'value':'5'}
 option={name:[] for name in names};option[names[0]]=['5']
 comparisons.append({'choices':count,'variantBytes':len(canonical(variant)),'optionRecordBytes':len(canonical(option)),'optionRecordFits64Fields':count<=64})
# Direct independent arithmetic boundaries, not a Core interpreter.
U=(1<<128)-1; wide=U*U
assert wide<(1<<256) and wide//U==U and wide//1>U
assert (-U,U)==(0-U,U-0)
# Retained independent case files and protected source remain byte-identical.
for name in ['draft-01-preservation.json','draft-02-preservation.json']:
 for f in json.loads((D/name).read_text())['files']:assert hashlib.sha256((D/f['preservedPath']).read_bytes()).hexdigest()==f['sha256']
c=json.loads((R/'deliverables/sp01-expression-contract-2026-09-10/source-candidate-02.json').read_text())
for f in c['files']:assert hashlib.sha256((R/f['path']).read_bytes()).hexdigest()==f['sha256']
print(json.dumps({'status':'PASS_SCOPED','closedSchema':result,'boundOperations':len(bind),'closedRuleKinds':len(j['closedRuleSignatures']),'negativeControls':negative,'representationComparison':comparisons,'protectedExpressionPins':len(c['files']),'scope':'Schema reference/type closure, acyclicity, exact roster/bindings, scalar arithmetic and encoding sizes only. No full value validator, operation interpreter, source/Core/K correspondence, signatures or ledger acceptance.'},indent=2))
