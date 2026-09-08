from pathlib import Path
import argparse,copy,hashlib,json,re,sys
p=argparse.ArgumentParser();p.add_argument('candidate');p.add_argument('--output',required=True);a=p.parse_args();src=Path(a.candidate);raw=src.read_bytes();d=json.loads(raw);failures=[];checks=[]
def at(root,path):
 value=root
 for k in path.split('.'):
  value=value[int(k)] if isinstance(value,list) else value[k]
 return value
def parent(root,path):
 parts=path.split('.');return (at(root,'.'.join(parts[:-1])) if len(parts)>1 else root),parts[-1]
def replace(root,path,value):
 par,key=parent(root,path)
 if isinstance(par,list):par[int(key)]=copy.deepcopy(value)
 elif isinstance(par,dict):par[key]=copy.deepcopy(value)
 else:raise TypeError('noncontainer parent '+path)
def compare(want,got,path=''):
 if type(want)!=type(got):return [{'path':path,'expected':want,'actual':got}]
 if isinstance(want,dict):
  result=[]
  for key in sorted(set(want)|set(got)):
   loc=path+'/'+str(key).replace('~','~0').replace('/','~1')
   if key not in want:result.append({'path':loc,'expectedMissing':True,'actual':got[key]})
   elif key not in got:result.append({'path':loc,'actualMissing':True,'expected':want[key]})
   else:result.extend(compare(want[key],got[key],loc))
  return result
 if isinstance(want,list):
  if len(want)!=len(got):return [{'path':path,'expected':want,'actual':got}]
  return [v for i,(x,y) in enumerate(zip(want,got)) for v in compare(x,y,path+'/'+str(i))]
 return [] if want==got else [{'path':path,'expected':want,'actual':got}]
assert d['statePatchRule']['id']=='typed-patch-inheritance/1'
for i,t in enumerate(d['traces']):
 post=t['expected']['postState'];match=re.fullmatch(r'traces\[id=(.+)\]\.input\.preState',post['inheritsFromPreState'])
 try:
  assert match,'unsupported/unresolved explicit inheritance reference'
  refs=[v for v in d['traces'] if v['id']==match.group(1)];assert len(refs)==1
  materialized=copy.deepcopy(refs[0]['input']['preState']);patch=post['patch']
  for path in patch['delete']:
   par,key=parent(materialized,path)
   if isinstance(par,list):del par[int(key)]
   else:del par[key]
  for path,val in patch['replace'].items():replace(materialized,path,val)
  for path,entries in patch.get('mapEntryReplace',{}).items():
   target=at(materialized,path);assert isinstance(target,dict) and isinstance(entries,dict)
   for k,v in entries.items():target[k]=copy.deepcopy(v)
  stored={k:v for k,v in post.items() if k not in ['inheritsFromPreState','patch']}
  diffs=compare(materialized,stored)
  checks.append({'trace':t['id'],'pass':not diffs,'differences':diffs})
  failures.extend({'trace':t['id'],**z} for z in diffs)
 except (KeyError,IndexError,TypeError,ValueError,AssertionError) as e:
  failures.append({'trace':t['id'],'applicationError':str(e)});checks.append({'trace':t['id'],'pass':False,'applicationError':str(e)})
assert hashlib.sha256(src.read_bytes()).digest()==hashlib.sha256(raw).digest()
result={'status':'pass' if not failures else 'blocked','candidateSha256':hashlib.sha256(raw).hexdigest(),'scope':'Only exact declared patch application versus stored complete state. Two documentary recipe keys are explicitly projected out; all other fields compared recursively. No economic/claim/footprint/runtime/ledger acceptance.','checks':checks,'failures':failures};Path(a.output).write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'status':result['status'],'traces':len(checks),'failedTraces':[x['trace'] for x in checks if not x['pass']],'fieldDifferences':len(failures)}));sys.exit(bool(failures))
