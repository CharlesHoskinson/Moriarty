from pathlib import Path
import json, subprocess, tempfile, sys
root=Path(sys.argv[1]); package=root/'experiments/moriarty-language'; ex=package/'spec/successor/examples'
source=ex/'expression-funded-payment.mori';schema=ex/'expression-funded-payment.schema.json';snap=json.loads((ex/'expression-funded-payment.snapshots.json').read_text());state=json.loads((ex/'expression-funded-payment.state.json').read_text())
results=[]
with tempfile.TemporaryDirectory(prefix='moriarty-funded-cli-') as temp:
 t=Path(temp)
 for name,first,second,accrued,cap in [('principal','10','20','0','100'),('interest','3','4','10','100'),('unfunded','10','20','0','29')]:
  ss=json.loads(json.dumps(snap)); st=json.loads(json.dumps(state));ss['Args']['first']=first;ss['Args']['second']=second
  st['obligations'][0]['accrued']=accrued;st['obligations'][0]['outstanding']=str(100+int(accrued));st['allowances'][0]['remaining']=cap
  (t/'snap.json').write_text(json.dumps(ss,sort_keys=True,separators=(',',':')));(t/'state.json').write_text(json.dumps(st))
  cmd=['node',str(package/'src/cli.ts'),'simulate','--profile','moriarty-financial-expression-source/1','--schema',str(schema),'--snapshots',str(t/'snap.json'),'--repayment-state',str(t/'state.json'),str(source)]
  result=subprocess.run(cmd,capture_output=True,text=True,timeout=10)
  if name=='unfunded':
   assert result.returncode==1,(result.returncode,result.stdout,result.stderr); assert result.stdout==''
   outcome=json.loads(result.stderr);assert outcome['status']=='Rejected';assert not any(k in outcome for k in ['post','financialPost','effects','descriptors'])
  else:
   assert result.returncode==0,(result.returncode,result.stderr);assert result.stderr==''
   outcome=json.loads(result.stdout);assert outcome['financialPre']==st;assert outcome['pre']==ss['Pre'];r=outcome['result'];assert r['status']=='FundedExpressionPrepared'
   pay=int(first)+int(second);di=min(pay,int(accrued));dp=pay-di
   assert [b['amount'] for b in r['financialPost']['balances']]==[str(100-pay),str(pay)]
   debt=r['financialPost']['obligations'][0];assert (debt['principal'],debt['accrued'],debt['outstanding'])==(str(100-dp),str(int(accrued)-di),str(100+int(accrued)-pay))
   assert r['post']['paid']==str(pay);assert r['effects'][0]['amount']==str(pay);assert r['effects'][1]['nominalAmount']==str(pay)
   assert r['financialPost']['work']['closureReserve']==st['work']['closureReserve']
  results.append({'case':name,'exitCode':result.returncode,'outcome':outcome})
print(json.dumps({'passed':len(results),'results':results},indent=2))
