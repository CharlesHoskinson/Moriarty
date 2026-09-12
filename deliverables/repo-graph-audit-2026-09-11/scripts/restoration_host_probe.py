import subprocess,json,select,time,pathlib
out=pathlib.Path('/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11')
p=subprocess.Popen(['codex','app-server','--stdio'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=open('/tmp/moriarty-restoration-host.stderr','w'),text=True,bufsize=1)
records=[]
def send(i,m,v):
 p.stdin.write(json.dumps({'jsonrpc':'2.0','id':i,'method':m,'params':v})+'\n');p.stdin.flush()
def read():
 line=p.stdout.readline()
 if not line:raise RuntimeError('EOF')
 d=json.loads(line)
 if 'hook' in d.get('method','').lower():
  records.append(d);out.joinpath('host-restoration-events.json').write_text(json.dumps(records,indent=2))
  print(json.dumps(d),flush=True)
 return d
def req(i,m,v):
 send(i,m,v)
 while True:
  d=read()
  if d.get('id')==i:return d
req(1,'initialize',{'clientInfo':{'name':'moriarty_hook_probe','version':'1'},'capabilities':{'experimentalApi':True}})
r=req(2,'thread/start',{'cwd':'/home/charl','ephemeral':True,'approvalPolicy':'never','model':'gpt-6-astra','baseInstructions':'For this hook smoke test, execute pwd once via the shell tool, then reply OK. Do not read files or edit anything.'})
tid=r['result']['thread']['id'];print('thread',tid,flush=True)
req(3,'turn/start',{'threadId':tid,'input':[{'type':'text','text':'Execute pwd once via the shell tool, then reply OK. No other tools or actions.','text_elements':[]}],'effort':'low'})
while True:
 d=read()
 if d.get('method')=='turn/completed':break
p.terminate()
