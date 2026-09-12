from pathlib import Path
import json, subprocess, tempfile, sys
r=Path(sys.argv[1]);p=r/'experiments/moriarty-language';ex=p/'spec/successor/examples'
old=(ex/'expression-funded-payment.mori').read_text();profile='moriarty-financial-agreement-source/1'
declarations='''
  unit Cash;
  asset Cash: Asset;
  record TransferFields {
    id: Text; from: Text; to: Text; settlementAsset: Text; transferAmount: Amount<Cash>;
  }
  record RepayFields {
    allocationId: Text; transferId: Text; obligationId: Text; payer: Text;
    nominalAmount: Quantity<Units<Cash,1>,0>;
  }
  operation Transfer: TransferFields;
  operation Repay: RepayFields;
  state due: UInt128;
  state paid: UInt128;
'''
source=old.replace('moriarty-financial-expression-source/1',profile).replace('agreement FundedPayment {','agreement FundedPayment {'+declarations)
baseSnap=json.loads((ex/'expression-funded-payment.snapshots.json').read_text());baseState=json.loads((ex/'expression-funded-payment.state.json').read_text());results=[]
with tempfile.TemporaryDirectory(prefix='moriarty-source-cli-') as td:
 t=Path(td)
 def write(name,value,canonical=False):
  q=t/name;q.write_text(json.dumps(value,sort_keys=True,separators=(',',':')) if canonical else value);return str(q)
 def call(mode,src,ss=None,st=None,oldMode=False):
  sf=write('old.mori' if oldMode else 'new.mori',src)
  cmd=['node',str(p/'src/cli.ts'),mode,'--profile','moriarty-financial-expression-source/1' if oldMode else profile]
  if oldMode and mode!='format':cmd+=['--schema',str(ex/'expression-funded-payment.schema.json')]
  if mode=='simulate':cmd+=['--snapshots',write('snap.json',ss,True),'--repayment-state',write('state.json',st,True)]
  cmd+=[sf];res=subprocess.run(cmd,capture_output=True,text=True,timeout=10)
  return res
 def rejected(src,token=None):
  res=call('check',src);assert res.returncode==1,(res.returncode,res.stdout,res.stderr);assert res.stdout=='';x=json.loads(res.stderr);assert x['status']=='Rejected',x
  if token:
   span=x.get('span',{});assert span.get('kind')=='source',x;a=int(span['start']);b=int(span['end']);encoded=src.encode('utf-8');assert 0<=a<=b<=len(encoded);selected=encoded[a:b].decode('utf-8');assert token in selected,(token,selected,x)
  results.append({'case':'reject','code':x.get('code'),'token':token});return x
 checked=call('check',source);assert checked.returncode==0,(checked.stdout,checked.stderr);results.append({'case':'check without schema'})
 formatted=call('format',source);assert formatted.returncode==0,formatted.stderr;twice=call('format',formatted.stdout);assert twice.returncode==0 and twice.stdout==formatted.stdout;results.append({'case':'idempotent format'})
 for first,second,interest in [('10','20','0'),('3','4','10')]:
  ss=json.loads(json.dumps(baseSnap));st=json.loads(json.dumps(baseState));ss['Args']['first']=first;ss['Args']['second']=second;st['obligations'][0]['accrued']=interest;st['obligations'][0]['outstanding']=str(100+int(interest))
  current=call('simulate',source,ss,st);previous=call('simulate',old,ss,st,True);assert current.returncode==previous.returncode==0,(current.stderr,previous.stderr)
  x=json.loads(current.stdout);y=json.loads(previous.stdout);assert x['result']==y['result'];assert x['financialPre']==y['financialPre'];assert x['pre']==y['pre'];assert x['sourceProfile']==profile
  fmt=call('simulate',formatted.stdout,ss,st);assert fmt.returncode==0 and json.loads(fmt.stdout)['result']==x['result'];results.append({'case':'full-result equality','payment':str(int(first)+int(second)),'result':x['result']})
  ss['Pre']=x['result']['post'];ss['workInitial']=x['result']['workRemaining'];st=x['result']['financialPost'];ss['Args']['transferId']='NextTransfer';ss['Args']['allocationId']='NextAllocation'
  again=call('simulate',source,ss,st);againOld=call('simulate',old,ss,st,True);assert again.returncode==againOld.returncode==0,(again.stderr,againOld.stderr);assert json.loads(again.stdout)['result']==json.loads(againOld.stdout)['result'];results.append({'case':'continuation equality'})
 rejected(source.replace('unit Cash;','unit Cash; unit Cash;'),'unit Cash;')
 rejected(source.replace('state due: UInt128;','state due: UnknownThing;'),'UnknownThing')
 rejected(source.replace('id: Text; from: Text;','id: Text; id: Text; from: Text;'),'id: Text;')
 rejected(source.replace('state due: UInt128;','state due: UInt128; state due: UInt128;'),'state due: UInt128;')
 rejected(source.replace('transferAmount: Amount<Cash>;','transferAmount: UInt128;'))
 rejected(source.replace('Units<Cash,1>,0>;','Units<Cash,2>,0>;'))
 rejected(source.replace('operation Repay: RepayFields;','operation Notice: RepayFields;'))
 rejected(source.replace('state paid: UInt128;','state paid: UInt128 = 0;'))
 rejected(source.replace('unit Cash;','unit Cash; unit Other;').replace('second: Quantity<Units<Cash,1>,0>','second: Quantity<Units<Other,1>,0>'))
 shifted=source.replace('state due: UInt128;','/* é😀 comment */ state due: UnknownThing;');rejected(shifted,'UnknownThing')
 # Forward record references preserve the same contract.
 forward=source.replace('  operation Transfer: TransferFields;','').replace('  operation Repay: RepayFields;','').replace('  unit Cash;','  operation Transfer: TransferFields; operation Repay: RepayFields; unit Cash;')
 res=call('check',forward);assert res.returncode==0,res.stderr;results.append({'case':'forward operation record references'})
 badState=json.loads(json.dumps(baseState));badState['allowances'][0]['remaining']='29';bad=call('simulate',source,baseSnap,badState);assert bad.returncode==1 and bad.stdout=='';out=json.loads(bad.stderr);assert not any(k in out for k in ['post','financialPost','effects','descriptors']);results.append({'case':'funding rejection no leakage','code':out['code']})
print(json.dumps({'passed':len(results),'results':results},indent=2))
