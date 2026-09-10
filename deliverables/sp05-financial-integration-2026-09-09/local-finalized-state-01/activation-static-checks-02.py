import ast,json,pathlib,types
p=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-deadline-review/deliverables/sp05-financial-integration-2026-09-09/local-finalized-state-01')
s=(p/'execute-once.py').read_text();ast.parse(s)
start=s.index(' assert time.monotonic()<activationDeadline;activationAllowance=')
end=s.index('\n fields=',start)
code=compile('\n'.join(x[1:] for x in s[start:end].splitlines()),str(p/'execute-once.py'),'exec')
results=[]
for start_time,write_delay,expected in [(12.2,0.1,True),(87.2,0.1,True),(87.2,2,False),(89.1,0,False)]:
 clock=[start_time];saved=[];calls=[]
 def save(n,v):saved.append(json.loads(json.dumps(v)));clock[0]+=write_delay
 def run(argv,timeout):calls.append({'argv':list(argv),'timeout':timeout,'at':clock[0]})
 ns={'time':types.SimpleNamespace(monotonic=lambda:clock[0]),'activationDeadline':90,'argv':['systemd-run','--user','--unit=x','--property=Type=exec','node'],'save':save,'run':run}
 rejected=False
 try:exec(code,ns)
 except AssertionError:rejected=True
 assert bool(calls)==expected
 if calls:
  allowance=saved[0]['activationAllowanceSeconds'];assert allowance<=90-calls[0]['at'];assert '--property=TimeoutStartSec='+str(allowance)+'s' in calls[0]['argv'];assert saved[0]['argv']==calls[0]['argv']
 results.append({'startTime':start_time,'durableWriteDelay':write_delay,'submitted':bool(calls),'rejected':rejected,'saved':saved,'calls':calls})
(p/'activation-static-checks-02.json').write_text(json.dumps({'status':'PASS','cases':results,'scope':'Extracted actual executor activation statements with controlled clock/save/run; no operational import, daemon, subprocess or network.'},indent=2)+'\n')
print('4 controlled activation cases PASS')
