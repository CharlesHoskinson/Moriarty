import ast,json,pathlib,hashlib,fractions
root=pathlib.Path('/home/charl/Moriarty');p=root/'openspec/sprints/verify.py';tree=ast.parse(p.read_text());failures=[]
for n in tree.body:
 if isinstance(n,ast.FunctionDef) and n.name=='require':n.body=ast.parse('if not condition: audit_failures.append(message)').body
# Printed terminal 'pass' belongs to instrumented audit only; failures below remain authoritative.
exec(compile(ast.fix_missing_locations(tree),str(p),'exec'),{'__file__':str(p),'audit_failures':failures})
print('ALL_COLLECTED_FAILURES',json.dumps(failures))
s=json.loads((root/'openspec/sprints/sprints.json').read_text());a=json.loads((root/'openspec/sprints/asset-study.json').read_text());tasks={t for x in s['sprints'] for g in x['entryGates'] for t in g['tasks']};owners={x['id']:set(x['owners']) for x in s['sprints']}
assert {x['id'] for x in a['requirements']}=={f'AS{i:02}' for i in range(1,12)}
assert {x['id'] for x in a['cases']}=={f'AT{i:02}' for i in range(1,9)}
assert len(a['requirements'])==11 and len(a['cases'])==8
h=hashlib.sha256((root/a['sourcePath']).read_bytes()).hexdigest();assert h==a['sourceSha256'];print('ASSET_SOURCE_SHA256',h)
for x in a['requirements']:
 ts=x['tasks']+[v['task'] for v in x.get('conditionalTasks',[])];assert set(ts)<=tasks;assert set(x['owners'])<=set().union(*(owners[t.split('.')[0]] for t in ts));print('VALID_REQUIREMENT_OWNERS',x['id'])
for x in a['cases']:
 assert set([x['primaryTask']]+x['contributingTasks'])<=tasks;assert set(x['requirements'])<={v['id'] for v in a['requirements']};assert x['positive'].strip() and x['reject'].strip();print('VALID_CASE_AND_NEGATIVE',x['id'])
print('NO_ECONOMIC_CASE_REFERENCE',sorted({x['id'] for x in a['requirements']}-{v for x in a['cases'] for v in x['requirements']}))
print('TX03_EXACT_RATIONAL',fractions.Fraction(100)*fractions.Fraction(10,100)*fractions.Fraction(30,360))
# Deny all asset-map access in this read-only view, then collect the full validator predicates again.
oldtext=pathlib.Path.read_text;oldbytes=pathlib.Path.read_bytes;oldisfile=pathlib.Path.is_file;asset=root/'openspec/sprints/asset-study.json';reads=[]
def rt(self,*args,**kw):
 if self==asset:reads.append(str(self));raise FileNotFoundError('controlled absent asset map')
 return oldtext(self,*args,**kw)
def rb(self,*args,**kw):
 if self==asset:reads.append(str(self));raise FileNotFoundError('controlled absent asset map')
 return oldbytes(self,*args,**kw)
pathlib.Path.read_text=rt;pathlib.Path.read_bytes=rb;pathlib.Path.is_file=lambda self:False if self==asset else oldisfile(self)
failures=[];exec(compile(ast.fix_missing_locations(tree),str(p),'exec'),{'__file__':str(p),'audit_failures':failures})
print('CONTROLLED_ASSET_ABSENCE',json.dumps({'assetReadAttempts':len(reads),'collectedFailures':failures,'canonicalValidatorPass':False}))
