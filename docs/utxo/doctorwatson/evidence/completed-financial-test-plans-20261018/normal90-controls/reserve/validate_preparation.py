"""Inspect prepared files/AST with all process creation forbidden. Never run main."""
from pathlib import Path
import ast,hashlib,importlib.util,json,subprocess,sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent;P=D.parent/'payout-full-consumer-normal90-preparation-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
calls=[]
def forbidden(*a,**k):calls.append(True);raise AssertionError('No subprocess/product/provider/Git execution in reserve preparation')
subprocess.Popen=forbidden;subprocess.run=forbidden;subprocess.check_output=forbidden
b=json.loads((D/'BOUND-CANDIDATE.json').read_bytes());old=json.loads((P/'BOUND-CANDIDATE.json').read_bytes());W=Path(b['checkout'])
assert (b['termSeconds'],b['killGraceSeconds'],b['parentMaximumSeconds'])==(90,5,95)
assert b['head']==old['head']=='0345e172faf9e601404d7cb96211bc4e0255de4c'and b['tree']==old['tree']=='ea4b5c97779788ff25ca54f51725d6f759b688d9'
assert b['cargoArgv']==[('core_reserve_spend_refund'if x=='core_three_way_payout'else x)for x in old['cargoArgv']]
assert b['timeoutArgv']==old['timeoutArgv'][:4]+b['cargoArgv']
assert len(b['testFunctions'])==2 and len(b['captureFiles'])==5 and len(b['publicCliArgv'])==1
assert b['captureFiles']==['reserve-spend-refund.mori','opening.scenario.json','stdout.json','stderr.txt','process.json']
cycle=Path(b['actualOutput']);assert cycle.parent==D.parent and cycle.is_absolute()and not cycle.exists()
assert sha(D/'run_check.py')==b['driverSha256']
for name,digest in b['controlHashes'].items():assert sha(D/name)==digest
assert (D/'SOURCE-PINS.json').read_bytes()==(P/'SOURCE-PINS.json').read_bytes()
sources=json.loads((D/'SOURCE-PINS.json').read_bytes())['files']
for row in sources:
 p=W/row['path'];assert p.is_file()and not p.is_symlink()and p.stat().st_size==row['bytes']and sha(p)==row['sha256']
 assert p.read_bytes()==(D/row['physical']).read_bytes()
expect=json.loads((D/'EXPECTATION-PINS.json').read_bytes())
for row in expect['originalPlanFiles']+expect['independentReserveOracleFiles']:
 p=D/row['physical'];assert p.is_file()and not p.is_symlink()and p.stat().st_size==row['bytes']and sha(p)==row['sha256']
for row in expect['original79Mappings']:assert (W/row['path']).read_bytes()==(D/row['physical']).read_bytes()
for tool in json.loads((D/'TOOL-PINS.json').read_bytes()):assert str(Path(tool['path']).resolve())==tool['resolved']and sha(Path(tool['resolved']))==tool['sha256']
consumer=W/'crates/mori/tests/core_reserve_spend_refund.rs';text=consumer.read_text();assert sha(consumer)=='27c502d178d7422b2c53f03e1b294cdb792dfc719aac1e772ca2cf405b514b90'
assert text.count('#[test]')==2 and all('fn '+name+'()'in text for name in b['testFunctions'])
assert text.count('Command::new(')==1 and 'Command::new(env!("CARGO_BIN_EXE_mori"))'in text
assert text.count('std::env::var_os(')==1 and 'MORI_CORE_REFUND_OBSERVATIONS'in text and 'MORI_PAYOUT_OBSERVATIONS'not in text
assert 'fn retain_json'not in text and 'simulate_metered'not in text and '--logical-cap'not in text
assert 'fs::create_dir(&root).expect("use a fresh observation directory")'in text
assert 'if self.temporary && !std::thread::panicking()'in text
assert text.index('fs::write(project.root.join("stdout.json")')<text.index('assert_eq!(output.status.code(), Some(0));')
assert text.index('project.root.join("process.json")',text.index('// Save complete actual process'))<text.index('assert_eq!(output.status.code(), Some(0));')
old_tree=ast.parse((P/'run_check.py').read_text());new_text=(D/'run_check.py').read_text();new_tree=ast.parse(new_text)
def main_node(tree):return next(n for n in tree.body if isinstance(n,ast.FunctionDef)and n.name=='main')
a,z=main_node(old_tree),main_node(new_tree)
def nested(main,name):return next(n for n in main.body if isinstance(n,ast.FunctionDef)and n.name==name)
for name in ['configs','members','sample']:assert ast.dump(nested(a,name))==ast.dump(nested(z,name))
for name in ['sha','now','write','require_public_file']:
 x=next(n for n in old_tree.body if isinstance(n,ast.FunctionDef)and n.name==name);y=next(n for n in new_tree.body if isinstance(n,ast.FunctionDef)and n.name==name);assert ast.dump(x)==ast.dump(y)
def assigned(main,name):return next(n.value for n in ast.walk(main)if isinstance(n,ast.Assign)and any(isinstance(t,ast.Name)and t.id==name for t in n.targets))
e,f=assigned(a,'env'),assigned(z,'env');assert len(f.keys)==14
assert [ast.dump(x)for x in e.values]==[ast.dump(x)for x in f.values]
assert [x.value for x in e.keys][:-1]==[x.value for x in f.keys][:-1]and f.keys[-1].value=='MORI_CORE_REFUND_OBSERVATIONS'
# The whole product Popen/wait/deadline loop, terminal-first write and session census are preserved.
def product_try(main):return next(n for n in ast.walk(main)if isinstance(n,ast.Try)and any(isinstance(x,ast.With)and any(isinstance(c,ast.Call)and isinstance(c.func,ast.Attribute)and c.func.attr=='Popen'for c in ast.walk(x))for x in n.body))
x,y=product_try(a),product_try(z)
wa=next(n for n in x.body if isinstance(n,ast.With));wz=next(n for n in y.body if isinstance(n,ast.With));assert ast.dump(wa)==ast.dump(wz)
# Cleanup statements after the original result verification are identical.
for x,y in zip(x.finalbody[-4:],y.finalbody[-4:]):assert ast.dump(x)==ast.dump(y)
assert new_text.index("write(cycle / 'TERMINAL.json', result, new=True)")<new_text.index('capture_facts = []')<new_text.index("binary = root / 'target/debug/mori'")<new_text.index('remaining = members(child.pid, deadline)')
assert "'expectedCount': 5"in new_text and '(2, 0, 0, 0, 0)'in new_text
assert "{'executable', 'args', 'status', 'success'}"in new_text and 'spawn_error'not in new_text
assert 'root-original-input-copies'not in new_text and 'MORI_PAYOUT'not in new_text and '--root-adoption'not in new_text
spec=importlib.util.spec_from_file_location('reserve_prepared_driver',D/'run_check.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);assert module.D==D
# Inspect frozen independent source data; never execute the deriving script or product.
F=D/'current-source/crates/fixtures/reserve-spend-refund';projection=json.loads((F/'financial-execution-projection.expected.json').read_bytes());kinds=json.loads((F/'source-statement-kinds.expected.json').read_bytes());prefix=json.loads((F/'cumulative-prefix-effects.expected.json').read_bytes())
assert len(projection['steps'])==len(kinds)==len(prefix)==20 and len(projection['effects'])==17
assert len(projection['pre']['balances'])==len(projection['post']['balances'])==8
work=json.loads((D/'independent-reserve-oracle/WORK-SCOPE.json').read_bytes());assert work['dispatches']=='20'and work['informational_sum']=='60'
assert calls==[]and not cycle.exists()
result={'status':'PASS_STATIC_RESERVE_PREPARATION_ONLY','driverSha256':sha(D/'run_check.py'),'bindingSha256':sha(D/'BOUND-CANDIDATE.json'),
 'sourcePinsVerified':len(sources),'original79WholeBodiesVerified':len(expect['original79Mappings']),'originalPlanPinsVerified':len(expect['originalPlanFiles']),
 'independentOracleWholeFrozenPayloads':len(expect['independentReserveOracleFiles'])-1,'wholeTestFunctions':2,'oneActualUnmeteredCliCallExpected':True,'captureFilesExpected':b['captureFiles'],
 'environment14OnlyExistingReserveHookChanged':True,'unchangedDeadlineProcessLoopAndCleanupTailAST':True,'terminalBeforeCaptureHashBinaryCensus':True,'binaryAfterOnly':True,
 'existingProjectionSteps':20,'existingOrderedEffects':17,'existingPrePostBalanceRows':8,'existingInformationalWork60IsNotAssertedOrObserved':True,
 'payoutAssumptionsNotCarried':{'noMeteredCall':True,'noLibraryCapture':True,'noWholeProgramOrRawWireAssertion':True,'processKeys':'executable,args,status,success; launchError on spawn failure'},
 'PopenAndProcessHelpersForbidden':True,'processCalls':0,'GitCalls':0,'driverMainExecuted':False,'freshActualCycleAbsent':True,
 'reserveRuntime':'UNRUN','newResourceCeiling':False,'600Allocated':False,'sourceAndActualResultAuditsRemainRequired':True,
 'limits':'Pure metadata/file/AST checks only; current HEAD/tree/clean Git check deferred to future driver preflight; no product/provider execution or test/cleanup success inferred.'}
(D/'STATIC-VALIDATION.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
