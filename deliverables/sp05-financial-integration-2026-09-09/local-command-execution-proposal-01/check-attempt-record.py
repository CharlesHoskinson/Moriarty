"""Evaluate only the exact public attempt dict with inert paths and byte fixtures."""
import ast,builtins,hashlib,json,pathlib,symtable
O=pathlib.Path(__file__).resolve().parent
source=(O/'execute-once.py').read_text();tree=ast.parse(source)
call=next(n for n in ast.walk(tree) if isinstance(n,ast.Call) and isinstance(n.func,ast.Name) and n.func.id=='save' and isinstance(n.args[0],ast.BinOp) and isinstance(n.args[0].right,ast.Constant) and n.args[0].right.value=='attempt.json')
code=compile(ast.Expression(call.args[1]),'<exact-attempt-record>','eval')
class InertPath:
 def __init__(self,name,exists=False):self.name,self.exists=name,exists
 def __truediv__(self,name):return InertPath(self.name+'/'+name,self.exists)
 def is_file(self):return self.exists
rows=[]
for kind in ['loan','swap']:
 for earlier in [False,True]:
  allocation='sp05-local-'+kind+'-completion-20260910-01';A={'status':'ADMITTED','case':kind,'allocationId':allocation};admissionBytes=json.dumps(A,sort_keys=True).encode();otherBytes=b'previous consumed case fixture'
  admission=InertPath('admission');packet=InertPath('packet',earlier)
  sha=lambda p:hashlib.sha256(admissionBytes if p.name=='admission' else otherBytes).hexdigest()
  env={'A':A,'admission':admission,'sha':sha,'P':{'priorCharges':{'priorCombinedReservations':27}},'timerArgv':['fixed-inert-timer'],'PACKET':packet,'other':'swap' if kind=='loan' else 'loan'}
  record=eval(code,{'__builtins__':{}},env)
  assert record['admissionSha256']==hashlib.sha256(admissionBytes).hexdigest()
  assert record['allocationId']==allocation and record['status']=='CONSUMED' and record['retryAllowed'] is False
  assert record['otherCaseAttemptSha256']==(hashlib.sha256(otherBytes).hexdigest() if earlier else None)
  rows.append({'kind':kind,'earlierCase':earlier,'status':'PASS'})
# Ruff/pyflakes are unavailable. This finite symtable check identifies unresolved
# module/global names; it does not claim flow-sensitive whole-program analysis.
table=symtable.symtable(source,'execute-once.py','exec');defined={s.get_name() for s in table.get_symbols() if s.is_assigned() or s.is_imported()};allowed=defined|set(dir(builtins));unknown=set()
def walk(t):
 for s in t.get_symbols():
  if s.is_referenced() and (t.get_type()=='module' or s.is_global()) and s.get_name() not in allowed:unknown.add(s.get_name())
 for child in t.get_children():walk(child)
walk(table);assert not unknown,sorted(unknown)
print(json.dumps({'status':'PASS','actualAttemptDictCases':rows,'unresolvedGlobalNames':sorted(unknown),'scope':'Exact AST expression and finite static global-name check only; no operational module import, filesystem probes, subprocess, services or network in the extracted expression.'},indent=2))
