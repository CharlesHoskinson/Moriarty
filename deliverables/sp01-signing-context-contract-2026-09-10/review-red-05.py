import copy,importlib.util,json
from pathlib import Path
O=Path(__file__).parent;s=importlib.util.spec_from_file_location('f',O/'context-checker-03.test.py');f=importlib.util.module_from_spec(s);s.loader.exec_module(f);c=f.c
rows=[]
d=f.fixture();d['authority']['mode']='ExactPlan';d['authority']['exactPlans']=[d['steps'][0]['planHash']];f.rehash(d);before=d['authorityHash'];s=copy.deepcopy(d['steps'][0]);s['id']='second';s['post'],o,r=c.derive(d['steps'][0]['post'],s['plan'],d['authority']);d['steps'].append(s);f.rehash(d);rows.append({'case':'exactRepeat','authorityUnchanged':before==d['authorityHash'],'result':c.validate(d,f.context()),'expected':'EXACT_OCCURRENCE'})
d=f.fixture();s=d['steps'][0];s['plan']={'action':'accrue','arguments':[{'name':'loan','type':'ObligationId','value':'Loan01'},{'name':'interest','type':'UInt128','value':'10'}],'operations':[{'op':'Accrue','obligation':'Loan01','amount':{'literal':'10'}}],'sidecarHex':''};s['post'],o,r=c.derive(d['initial'],s['plan'],d['authority']);s['ordinaryCharge']=str(o);f.rehash(d);rows.append({'case':'accrueBeforeCreation','result':c.validate(d,f.context()),'expected':'DEBT_NOT_CREATED'})
d=f.fixture();d['authority']['netGoal']='18';d['steps'][0]['terminal']=True;f.rehash(d);s=copy.deepcopy(s);s['id']='afterTerminal';s['post'],o,r=c.derive(d['steps'][0]['post'],s['plan'],d['authority']);d['steps'].append(s);f.rehash(d);rows.append({'case':'terminalThenAccrue','result':c.validate(d,f.context()),'expected':'TERMINAL_CONTINUATION'})
print(json.dumps(rows,indent=2));raise SystemExit(1)
