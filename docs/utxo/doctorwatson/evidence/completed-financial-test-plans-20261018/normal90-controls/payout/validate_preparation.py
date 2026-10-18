"""Pure file/AST validation only; executing the driver or a child is forbidden."""
from pathlib import Path
import ast,hashlib,importlib.util,json,subprocess,sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent;O=D.parent/'payout-full-consumer-one600-preparation-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
called=[]
def forbidden(*args,**kwargs):
 called.append(True);raise AssertionError('Process execution forbidden in preparation validation')
subprocess.Popen=forbidden;subprocess.run=forbidden;subprocess.check_output=forbidden
b=json.loads((D/'BOUND-CANDIDATE.json').read_bytes());old=json.loads((O/'BOUND-CANDIDATE.json').read_bytes())
W=Path(b['checkout']);cycle=Path(b['actualOutput'])
assert b['head']==old['head']=='0345e172faf9e601404d7cb96211bc4e0255de4c'
assert b['tree']==old['tree']=='ea4b5c97779788ff25ca54f51725d6f759b688d9'
assert (b['termSeconds'],b['killGraceSeconds'],b['parentMaximumSeconds'])==(90,5,95)
assert b['cargoArgv']==old['cargoArgv']
assert b['timeoutArgv'][:3]==old['timeoutArgv'][:3]and b['timeoutArgv'][3]=='90s'and b['timeoutArgv'][4:]==old['timeoutArgv'][4:]
for field in ['testFunctions','captureFiles','publicCliArgv']:
 assert b[field]==old[field]
assert len(b['testFunctions'])==3 and len(b['captureFiles'])==14 and len(b['publicCliArgv'])==2
assert not cycle.exists()and cycle.parent==D.parent
assert sha(D/'run_check.py')==b['driverSha256']
for name,digest in b['controlHashes'].items():assert sha(D/name)==digest
for name in ['SOURCE-PINS.json','EXPECTATION-PINS.json','TOOL-PINS.json','ENVIRONMENT-CONTRACT.json','CURRENT-SOURCE-AND-EFFECT-BINDING.json']:
 assert (D/name).read_bytes()==(O/name).read_bytes()
source_rows=json.loads((D/'SOURCE-PINS.json').read_bytes())['files']
for row in source_rows:
 p=W/row['path'];assert p.is_file()and not p.is_symlink()and sha(p)==row['sha256']and p.stat().st_size==row['bytes']
 assert p.read_bytes()==(D/row['physical']).read_bytes()
expected=json.loads((D/'EXPECTATION-PINS.json').read_bytes())
for row in expected['originalPlanFiles']:
 p=D/row['physical'];assert sha(p)==row['sha256']and p.stat().st_size==row['bytes']
for row in expected['original79Mappings']:
 assert (W/row['path']).read_bytes()==(D/row['physical']).read_bytes()
for tool in json.loads((D/'TOOL-PINS.json').read_bytes()):
 p=Path(tool['path']);assert str(p.resolve())==tool['resolved']and sha(Path(tool['resolved']))==tool['sha256']
# Inspect imports/top-level declarations only; __main__ guard is not entered.
spec=importlib.util.spec_from_file_location('ordinary90_prepared_driver',D/'run_check.py');driver=importlib.util.module_from_spec(spec);spec.loader.exec_module(driver)
assert driver.D==D
new_text=(D/'run_check.py').read_text();new_tree=ast.parse(new_text)
old_tree=ast.parse((O/'run_check.py').read_text())
new_main=next(n for n in new_tree.body if isinstance(n,ast.FunctionDef)and n.name=='main')
old_main=next(n for n in old_tree.body if isinstance(n,ast.FunctionDef)and n.name=='main')
def assigned(tree,name):
 found=[]
 for node in ast.walk(tree):
  if isinstance(node,ast.Assign)and any(isinstance(x,ast.Name)and x.id==name for x in node.targets):found.append(node.value)
 return found
for name in ['env','command']:
 assert [ast.dump(n)for n in assigned(new_main,name)]==[ast.dump(n)for n in assigned(old_main,name)]
for name in ['verify','configs','sample']:
 a=next(n for n in old_main.body if isinstance(n,ast.FunctionDef)and n.name==name)
 z=next(n for n in new_main.body if isinstance(n,ast.FunctionDef)and n.name==name)
 assert ast.dump(a)==ast.dump(z)
# Every original finally block (summary/CLI/assertion/source/cleanup predicates) is unchanged.
a=next(n for n in old_main.body if isinstance(n,ast.Try)and n.finalbody)
z=next(n for n in new_main.body if isinstance(n,ast.Try)and n.finalbody)
assert [ast.dump(n)for n in a.finalbody]==[ast.dump(n)for n in z.finalbody]
assert not any(isinstance(n,ast.FunctionDef)and n.name=='adoption'for n in new_tree.body)
assert '--root-adoption'not in new_text and 'freshWholeHostReviews'not in new_text
assert 'deadline = start + 95' in new_text and "'90s', *command"in new_text
assert new_text.index("write(cycle / 'TERMINAL.json', result, new=True)")<new_text.index('capture_facts = []')<new_text.index("binary = root / 'target/debug/mori'")<new_text.index('remaining = members(child.pid, deadline)')
assert 'unknownOrLiveMembersRetainRoot' in new_text and 'allPersistentCapturesRetained' in new_text
consumer=W/'crates/mori/tests/core_three_way_payout.rs';text=consumer.read_text()
assert text.count('#[test]')==3 and all('fn '+name+'()'in text for name in b['testFunctions'])
assert 'Command::new(env!("CARGO_BIN_EXE_mori"))'in text
assert text.index('let ordinary_output = project.run_case(&cases[0]);')<text.index('let metered_output = project.run_case(&cases[1]);')<text.index('let ordinary = project.assert_case(')
env=json.loads((D/'ENVIRONMENT-CONTRACT.json').read_bytes())
assert len(env['baseExactly13'])==13 and list(env['onlyAddition'])==['MORI_PAYOUT_OBSERVATIONS']
assert len(assigned(new_main,'env')[0].keys)==14
prior=json.loads((D/'PRIOR-HOST-PUBLIC-FACTS.json').read_bytes())
for row in prior['files']:
 p=D/row['physical'];assert sha(p)==row['sha256']and p.stat().st_size==row['bytes']
assert prior['wholeOpinions']==prior['resourceVotes']==0 and prior['600AdoptionOrAllocation']is False
original=json.loads((O/'MANIFEST.json').read_bytes())
for row in original['files']:
 p=O/row['path'];q=D/'original-one600-preparation'/row['path'];assert sha(p)==sha(q)==row['sha256']
assert sha(O/'MANIFEST.json')=='5ccf8b443af960a0b5517ac079ae6d4a86749d414fefa4753f2360949bcaddec'
assert called==[]and not cycle.exists()
result={'status':'PASS_STATIC_ORDINARY90_PREPARATION','bindingSha256':sha(D/'BOUND-CANDIDATE.json'),'driverSha256':sha(D/'run_check.py'),
 'head':b['head'],'tree':b['tree'],'sourcePinsVerified':len(source_rows),'originalPlanFilePinsVerified':len(expected['originalPlanFiles']),
 'original79WholeBodiesIdentical':len(expected['original79Mappings']),'wholeTestFunctions':3,'genuineCaptureFilesExpected':14,'publicCliCallsExpected':2,
 'consumerAndFixtureSourcesUnchanged':True,'cargoArgvUnchanged':True,'environmentASTUnchanged':True,'exactEnvironmentFields':14,
 'verifyConfigsSampleFunctionASTUnchanged':True,'entirePostcheckAndCleanupFinallyASTUnchanged':True,'terminalBeforeCapturesBinaryCensus':True,
 'binaryHashAfterOnly':True,'freshCycleAbsent':True,'PopenAndProcessHelpersForbidden':True,'processCalls':0,'mainNotExecuted':True,
 'productOrProviderDispatches':0,'payoutStillUnrun':True,'600Unallocated':True,'wholeSourceActualResultAuditsRemainRequired':True,
 'limits':'Static file/AST guards only. No claim of actual deadline, cleanup, CPU accounting, CLI output, financial correctness or acceptance.'}
(D/'STATIC-VALIDATION.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
