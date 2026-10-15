import sys,json
models={};terminal={}
for line in sys.stdin:
 try:e=json.loads(line)
 except ValueError:continue
 p=e.get('payload',{})
 if e.get('payload_type')=='task.lifecycle.status':
  for f in p.get('event',{}).get('details',{}).get('facets',[]):
   d=f.get('detail',{})
   if f.get('kind')=='producer' and d.get('model'):
    models[d['model']]={k:d[k] for k in ('model','provider','request_id','response_id') if k in d}
 if e.get('payload_type')=='run.terminal.completed':terminal=p
print(json.dumps({'modelUsage':models,'text':terminal.get('text'),'stopReason':terminal.get('terminal'),'isError':terminal.get('terminal') not in ('completed',)}))
