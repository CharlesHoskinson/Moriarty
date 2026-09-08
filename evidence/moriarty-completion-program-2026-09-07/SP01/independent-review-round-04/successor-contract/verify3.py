from pathlib import Path
import json,hashlib,copy,sys,shutil,importlib.metadata,subprocess,tempfile
from jsonschema import Draft202012Validator,validators,ValidationError
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree']);R=Path(B['sourceRoot']);P=W/'experiments/moriarty-language/spec/successor'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();canon=lambda x:json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode();H=lambda d,x:hashlib.sha256(d.encode()+b'\0'+canon(x)).hexdigest()
errors=[];checks=[]
def require(ok,description):
 checks.append({'check':description,'pass':bool(ok)})
 if not ok:errors.append(description)
for p,h in B['inputs'].items():require(sha((R/p) if (R/p).is_file() else (S/p))==h,'protected input '+p)
files={p:sha(W/p) for p in B['ownedFiles']};total=sum((W/p).stat().st_size for p in files);require(total<=786432,'owned files <=768KiB')
authorReceiptName='generator-completion-worker-receipt.json'
receipt=json.loads((S/authorReceiptName).read_text());require(receipt['exitCode']==0 and not receipt['unauthorizedGitActivity'] and not receipt['storageStop'],'terminal successful author receipt');require('grok-4.6-build' in receipt.get('modelUsage',{}),'actual Grok model identity')
commands=[]
def run_capture(argv,name,timeout):
 q=subprocess.run(argv,cwd=P,capture_output=True,timeout=timeout)
 (S/(name+'.stdout')).write_bytes(q.stdout);(S/(name+'.stderr')).write_bytes(q.stderr)
 commands.append({'argv':argv,'cwd':str(P),'exitCode':q.returncode,'stdoutSha256':sha(S/(name+'.stdout')),'stderrSha256':sha(S/(name+'.stderr'))})
 require(q.returncode==0,name+' exit0');return q
run_capture(['/usr/bin/python3',str(P/'signing-contract.test.py')],'generator-tests3',45)
with tempfile.TemporaryDirectory(prefix='moriarty-generator-verification-') as scratch:
 outputs=[]
 for i in range(2):
  out=Path(scratch)/('examples-'+str(i)+'.json')
  run_capture(['/usr/bin/python3',str(P/'generate-signing-examples.py'),'--output',str(out)],'generator-reproduction3-'+str(i),40)
  require(out.is_file(),'generation '+str(i)+' produced explicit output')
  if out.is_file():outputs.append(out.read_bytes())
 require(len(outputs)==2 and outputs[0]==outputs[1]==(P/'signing-examples.json').read_bytes(),'two clean deterministic generations equal retained bytes')
schema=json.loads((P/'signing-display-schema.json').read_text());Draft202012Validator.check_schema(schema);v=Draft202012Validator(schema)
def domain(validator,tag,instance,sch):
 if not isinstance(instance,str):return
 try:n=int(instance)
 except ValueError:return
 if 'x-inclusiveMin' in sch and n<int(sch['x-inclusiveMin']):yield ValidationError('below '+tag+' inclusive minimum')
 if 'x-inclusiveMax' in sch and n>int(sch['x-inclusiveMax']):yield ValidationError('above '+tag+' inclusive maximum')
DomainValidator=validators.extend(Draft202012Validator,{'x-domain':domain});dv=DomainValidator(schema)
x=json.loads((P/'signing-examples.json').read_text());docs={};registry=x['preimageRegistry'];hashchecks=[]
def leaves(z,p=''):
 if isinstance(z,dict) and z:
  for k in sorted(z):yield from leaves(z[k],p+'.'+k if p else k)
 elif isinstance(z,list) and z:
  for i,e in enumerate(z):yield from leaves(e,p+'.'+str(i))
 else:yield p,z
for row in x['valid']:
 d=json.loads(row['canonicalUtf8']);docs[row['id']]=d
 require(canon(d).decode()==row['canonicalUtf8'],row['id']+' canonical bytes')
 require(v.is_valid(d),row['id']+' schema valid')
 require(dv.is_valid(d),row['id']+' numeric domains valid')
 if not v.is_valid(d):errors.extend(str(e.message)[:500] for e in v.iter_errors(d))
 displayed=row['displayProjection'];expected=list(leaves(d));actual=[(r['path'],r['value']) for r in displayed]
 require(actual==expected,row['id']+' complete ordered signed leaf projection')
 candidates={**{('preimageRegistry.'+k):val for k,val in registry.items() if isinstance(val,(dict,list))},'signedDocument':d}
 if 'executionBody' in d:candidates['signedDocument.executionBody']=d['executionBody']
 for item in row['hashDag']:
  if item['name']=='sourceHash':
   matches=['preimageRegistry.sourceUtf8'] if hashlib.sha256(registry['sourceUtf8'].encode()).hexdigest()==item['digest'] else []
  else:matches=[k for k,val in candidates.items() if H(item['domain'],val)==item['digest']]
  require(bool(matches),row['id']+' reproducible '+item['name'])
  hashchecks.append({'document':row['id'],'name':item['name'],'domain':item['domain'],'digest':item['digest'],'matchingRetainedPreimages':matches})
 if row['kind']=='ExactPlan':
  require(H('MORIARTY-SUCC-EXEC-BODY/0',d['executionBody'])==d['executionBodyHash']==row['hashes']['executionBodyHash'],row['id']+' selected execution body bound')
  require(H('MORIARTY-SUCC-EXACT-PLAN/0',d)==row['hashes']['exactPlanDigest'],row['id']+' exact signed digest')
 else:require(H('MORIARTY-SUCC-OUTCOME-INTENT/0',d)==row['hashes']['outcomeIntentDigest'],row['id']+' outcome signed digest')
require(len(docs)==4,'four positive documents including genesis/nonexchange')
negatives=[]
for row in x['invalid']:
 d=copy.deepcopy(docs[row['base']])
 for path,val in row.get('sets',{}).items():
  keys=path.split('.');cur=d
  for k in keys[:-1]:cur=cur[int(k)] if isinstance(cur,list) else cur[k]
  key=keys[-1]
  if isinstance(cur,list):cur[int(key)]=val
  else:cur[key]=val
 if row.get('recomputeExecutionBodyHash'):d['executionBodyHash']=H('MORIARTY-SUCC-EXEC-BODY/0',d['executionBody'])
 schemaok=v.is_valid(d);domainok=dv.is_valid(d)
 require(schemaok==(row['schemaExpected']=='pass'),row['id']+' stated schema stage')
 if schemaok:require(domainok==(row['intendedFailureStage']!='domain'),row['id']+' numeric domain stage')
 negatives.append({'id':row['id'],'schemaPass':schemaok,'typedDomainPass':domainok,'intendedStage':row['intendedFailureStage'],'contextExecuted':False,'dependentHashReconstructionExecuted':False})
require(len((P/'semantic-contract.md').read_text().split())<=10000,'semantic contract <=10000words maximum admitted precise-table extension')
for p,h in files.items():require(sha(W/p)==h,'owned file unchanged during verification '+p)
report={'status':'pass' if not errors else 'blocked','candidateSha256':hashlib.sha256(json.dumps(files,sort_keys=True,separators=(',',':')).encode()).hexdigest(),'files':files,'ownedBytes':total,'authorReceipt':authorReceiptName,'commands':commands,'schemaValidator':'jsonschema '+importlib.metadata.version('jsonschema'),'checks':checks,'errors':errors,'hashChecks':hashchecks,'negativeCases':negatives,'limits':'Schema/domain checks, exact signed leaf values and retained preimage digest reproduction only. Display labels/units, hash dependency semantics, full financial contextual negatives, algebraic/proof/runtime/ledger claims require independent review. No signatures or contextual evaluator executed.'}
(S/'candidate-03-freeze.json').write_text(json.dumps(report,indent=2)+'\n');O=S/'candidate-03';O.mkdir(exist_ok=False)
for p in files:t=O/p;t.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(W/p,t)
print(json.dumps(report,indent=2));sys.exit(1 if errors else 0)
