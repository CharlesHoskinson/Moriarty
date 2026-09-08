from pathlib import Path
import urllib.request,json,datetime,sys
b=json.loads((Path(__file__).resolve().parent/'binding.json').read_text())
class NoRedirect(urllib.request.HTTPRedirectHandler):
 def redirect_request(self,*a,**kw):return None
opener=urllib.request.build_opener(NoRedirect)
rows=[]
for name,url,payload in [(m,b['rpc'],{'jsonrpc':'2.0','id':i,'method':m,'params':[]}) for i,m in enumerate(b['calls'][:3],1)]+[('indexer-typename',b['indexer'],{'query':'query {__typename}'})]:
 row={'name':name,'endpoint':url,'at':datetime.datetime.now(datetime.timezone.utc).isoformat()}
 try:
  req=urllib.request.Request(url,data=json.dumps(payload).encode(),headers={'Content-Type':'application/json','User-Agent':'Moriarty-readiness/1'},method='POST')
  with opener.open(req,timeout=10) as response:
   raw=response.read(102401);assert len(raw)<=102400,'response limit';body=json.loads(raw);row.update({'httpStatus':response.status,'body':body,'ok':response.status==200 and not body.get('error') and not body.get('errors')})
 except Exception as exc:row.update({'ok':False,'error':str(exc)[:500]})
 rows.append(row)
 if not row['ok']:break
print(json.dumps({'scope':b['scope'],'requests':rows,'status':'pass' if len(rows)==4 and all(x['ok'] for x in rows) else 'fail'},indent=2))
sys.exit(0 if len(rows)==4 and all(x['ok'] for x in rows) else 1)
