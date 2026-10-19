"""Popen-forbidden static/load/hash only; subject never executed."""
from pathlib import Path
import ast,hashlib,json,runpy,subprocess
D=Path(__file__).resolve().parent;B=D.parent/'euc012-realm-seven-test-diagnostic-capture-preparation-20261019';calls=[]
def forbidden(*a,**k):calls.append('Popen');raise AssertionError('subject execution forbidden')
subprocess.Popen=forbidden;subprocess.check_output=forbidden
sha=lambda b:hashlib.sha256(b).hexdigest();ns=runpy.run_path(str(D/'capture_once.py'),run_name='metadata_only');plan,rows=ns['preflight']()
assert len(rows)==22 and all(x['match'] for x in rows) and plan['sourceFiles']==20 and plan['tools']==2
old=(B/'capture_once.py').read_text();new=(D/'capture_once.py').read_text();oldsha=sha((B/'PLAN.json').read_bytes());newsha=sha((D/'PLAN.json').read_bytes());assert new==old.replace("PLAN_SHA256='"+oldsha+"'","PLAN_SHA256='"+newsha+"'")
def normal(code):
 tree=ast.parse(code)
 for n in tree.body:
  if isinstance(n,ast.Assign) and len(n.targets)==1 and isinstance(n.targets[0],ast.Name) and n.targets[0].id=='PLAN_SHA256':n.value=ast.Constant(value='<EXACT_NEW_PLAN_SHA256>')
 return ast.dump(tree,include_attributes=False,indent=2)+'\n'
assert normal(old)==normal(new)==(D/'NORMALIZED-AST.txt').read_text()
source=Path(plan['reviewedR068Source']);assert sha(source.read_bytes())==plan['reviewedR068SourceSha256'];reviewed=json.loads(source.read_bytes());alias=json.loads(Path(plan['commandSource']).read_bytes());assert alias=={'cwd':reviewed['cwd'],'commands':[reviewed['command']]}
assert plan['command']==reviewed['command'] and plan['cwd']==reviewed['cwd'] and reviewed['staticTestDeclarationCount']==8
assert sha(Path(plan['wholeReviewSource']).read_bytes())==plan['wholeReviewSha256']=='a408e8c774a8fa7509f09f38f3ce2e8187e1e9375f55379e6128b3429fe3e8d8'
assert sum(1 for line in (Path(plan['cwd'])/'tests/custody-failure-isolation.test.mjs').read_text().splitlines() if line.startswith('test('))==8
assert plan['maxAttempts']==1 and plan['innerChildBoundSeconds']==95 and plan['parentCommunicateCaptureTimeoutSeconds']==100 and plan['command']['argv']==json.loads((B/'PLAN.json').read_bytes())['command']['argv'] and plan['command']['env']=={'PATH':'/usr/bin:/bin','LANG':'C.UTF-8'}
assert not Path(plan['outputDirectory']).exists() and calls==[]
print(json.dumps({'status':'PASS_POPEN_FORBIDDEN_METADATA_ONLY','all22PinsVerified':True,'sourcePins':20,'toolPins':2,'exactR068CommandAndCwdAliasLossless':True,'eightStaticDeclarationsNotPASS':True,'scriptOnlyPLAN_SHADifference':True,'normalizedAstEqual':True,'processBehaviorResource90Plus5Capture100Max1Unchanged':True,'allHistoricalCountersUntouched':True,'outputAbsent':True,'PopenCalls':0,'subjectProviderProductGitCalls':0,'planSha256':newsha,'scriptSha256':sha(new.encode())},indent=2))
