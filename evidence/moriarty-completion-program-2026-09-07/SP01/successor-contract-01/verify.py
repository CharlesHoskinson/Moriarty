from pathlib import Path
import json,hashlib,subprocess,copy,importlib.metadata
from jsonschema import Draft202012Validator
S=Path(__file__).resolve().parent;B=json.loads((S/'binding.json').read_text());W=Path(B['worktree']);R=Path(B['sourceRoot']);P=W/'experiments/moriarty-language/spec/successor'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();canonical=lambda x:json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode();H=lambda domain,x:hashlib.sha256(domain.encode()+b'\0'+canonical(x)).hexdigest()
for p,h in B['inputs'].items():assert sha(R/p)==h,('source changed',p)
files={p:sha(W/p) for p in B['ownedFiles']};total=sum((W/p).stat().st_size for p in files);assert total<=120*1024,total
schema=json.loads((P/'signing-display-schema.json').read_text());Draft202012Validator.check_schema(schema);validator=Draft202012Validator(schema);examples=json.loads((P/'signing-examples.json').read_text());assert examples['status']=='proposed';docs={};hashchecks=[]
for row in examples['valid']:
 doc=json.loads(row['canonicalUtf8']);assert canonical(doc).decode()==row['canonicalUtf8'];validator.validate(doc);docs[row['id']]=doc
 if row['kind']=='ExactPlan':
  assert H('MORIARTY-SUCC-EXEC-BODY/0',doc['executionBody'])==doc['executionBodyHash']==row['hashes']['executionBodyHash'];assert H('MORIARTY-SUCC-EXACT-PLAN/0',doc)==row['hashes']['exactPlanDigest'];hashchecks.extend(['executionBodyHash','exactPlanDigest'])
 else:assert H('MORIARTY-SUCC-OUTCOME-INTENT/0',doc)==row['hashes']['outcomeIntentDigest'];hashchecks.append('outcomeIntentDigest')
assert len(docs)==2
checks=[]
for row in examples['invalid']:
 d=copy.deepcopy(docs[row['base']])
 for path,val in row['sets'].items():
  keys=path.split('.');cur=d
  for k in keys[:-1]:cur=cur[int(k)] if isinstance(cur,list) else cur[k]
  k=keys[-1]
  if isinstance(cur,list):cur[int(k)]=val
  else:cur[k]=val
 if row.get('recomputeExecutionBodyHash'):d['executionBodyHash']=H('MORIARTY-SUCC-EXEC-BODY/0',d['executionBody'])
 passed=validator.is_valid(d);assert passed==(row['schemaExpected']=='pass'),(row['id'],list(validator.iter_errors(d)));checks.append({'id':row['id'],'schemaPass':passed,'claimedContextStage':row['intendedFailureStage'],'contextExecuted':False})
decisions=json.loads((P/'semantic-decisions.json').read_text());assert decisions['status']=='proposed';assert all(r['status'] in ['proposed','inherited-constraint'] for r in decisions['entries'])
q=subprocess.run(['git','diff','--check'],cwd=W,capture_output=True,text=True);assert q.returncode==0,q.stdout+q.stderr
candidate=hashlib.sha256(json.dumps(files,sort_keys=True,separators=(',',':')).encode()).hexdigest();report={'candidateSha256':candidate,'files':files,'ownedBytes':total,'inputCount':len(B['inputs']),'schemaValidator':'jsonschema '+importlib.metadata.version('jsonschema'),'positiveSchemaDocuments':len(docs),'negativeSchemaAndContextCases':checks,'independentHashesChecked':hashchecks,'limits':'No context evaluator exists in this proposal. Context-only invalid cases are specified, not executed. Acceptance/program/profile/genesis/state observation preimage hash verification not claimed by this check. No signature/proof/network/runtime acceptance.'};(S/'candidate-01-freeze.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
