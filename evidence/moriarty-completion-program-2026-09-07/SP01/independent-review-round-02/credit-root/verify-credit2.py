from pathlib import Path
import json,ast,operator,re,hashlib
S=Path(__file__).resolve().parent;B=json.loads((S/'credit/binding.json').read_text());P=Path(B['worktree'])/B['ownedFiles'][0];j=json.loads(P.read_text());checks=[]
binops={ast.Add:operator.add,ast.Sub:operator.sub,ast.Mult:operator.mul,ast.FloorDiv:operator.floordiv};cmpops={ast.Eq:operator.eq,ast.NotEq:operator.ne,ast.LtE:operator.le,ast.GtE:operator.ge}
def calc(n,env):
 if isinstance(n,ast.Expression):return calc(n.body,env)
 if isinstance(n,ast.Constant) and isinstance(n.value,(int,str)):return n.value
 if isinstance(n,ast.Name):return env[n.id]
 if isinstance(n,ast.Call):
  assert isinstance(n.func,ast.Name) and not n.keywords
  if n.func.id=='int' and len(n.args)==1:return int(calc(n.args[0],env))
  if n.func.id=='max' and 1<=len(n.args)<=16:
   values=[calc(a,env) for a in n.args];assert all(isinstance(v,int) for v in values);return max(values)
  raise AssertionError('unsupported function')
 if isinstance(n,ast.BinOp):return binops[type(n.op)](calc(n.left,env),calc(n.right,env))
 if isinstance(n,ast.Compare):
  assert len(n.ops)==len(n.comparators)==1
  return cmpops[type(n.ops[0])](calc(n.left,env),calc(n.comparators[0],env))
 raise AssertionError(('unsupported expression node',type(n).__name__))
for row in j['verification']:
 assert len(row['expression'])<1000
 result=calc(ast.parse(row['expression'],mode='eval'),row['inputs'])
 assert str(result)==str(row['expected']),(row['id'],result,row['expected'])
 checks.append({'id':row['id'],'observed':str(result)})
def amount(x):
 assert isinstance(x,str) and re.fullmatch(r'0|[1-9][0-9]*',x)
 n=int(x);assert n<2**128;return n
def balances(state):
 out={}
 for acct in state['accounts']:
  for b in acct['balances']:
   k=(acct['id'],b['asset'],b['unit'],b['scale']);assert k not in out
   assert not re.fullmatch('[0-9]+',b['asset']);out[k]=amount(b['amount'])
 return out
cash=[]
for name,o in j['library']['oracles'].items():
 current=balances(o['preState']);post=balances(o['postState']);assert set(current)==set(post)
 for step in o['steps']:
  for t in step['transfers']:
   n=amount(t['amount']);a=(t['from'],t['asset'],t['unit'],t['scale']);b=(t['to'],t['asset'],t['unit'],t['scale'])
   assert a in current and b in current and current[a]>=n,(name,step['index'],a,n)
   current[a]-=n;current[b]+=n
  for mutation in step['nominalChanges']:
   if mutation['kind']=='burn-from-lock':
    assert name=='redemption' and mutation['object']=='requestEscrow.FundShares'
    k=('requestEscrow','FundShares','shares','1');assert current[k]==amount(mutation['from']);assert amount(mutation['to'])<=current[k];current[k]=amount(mutation['to'])
 assert current==post,(name,'post cash mismatch',current,post)
 work=o['postState']['work'];assert amount(work['ordinaryCharged'])+amount(work['ordinaryRemaining'])==amount(work['lifetimeOriginal'])
 assert sum(amount(x['ordinary']) for x in work['charges'])==amount(work['ordinaryCharged'])
 assert amount(work['recoveryReserveCharged'])+amount(work['recoveryReserveRemaining'])==amount(work['recoveryReserveOriginal'])
 assert sum(amount(x['reserve']) for x in work['charges'])==amount(work['recoveryReserveCharged'])
 cash.append({'oracle':name,'accountAssetEntries':len(current),'steps':len(o['steps']),'fullPostCashMatchesReplayedTransfersAndExplicitShareBurn':True})
report={'status':'scoped-pass','fragmentSha256':hashlib.sha256(P.read_bytes()).hexdigest(),'arithmeticChecks':checks,'cashReplay':cash,'scope':'Independent restricted AST integer arithmetic, full account balance replay and ordinary/recovery totals. No untrusted commands executed. Debt/authority/currentness/context predicates and full financial acceptance still require independent review. No Moriarty runtime/proof/network claim.'}
(S/'credit-root-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
