from pathlib import Path
import ast,difflib,hashlib,json,stat
D=Path(__file__).resolve().parent;R=D.parent
O=R/'payout-full-consumer-one600-preparation-20261018'
W=Path('/home/charl/Moriarty/.worktrees/utxo-guarded-full-prefix-main-20261018')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(name,value):
 p=D/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(value,indent=2)+'\n')
def copy(src,dst):
 assert not src.is_symlink()and stat.S_ISREG(src.lstat().st_mode)
 assert all('.private'not in part for part in src.parts)
 dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(src.read_bytes());assert sha(src)==sha(dst)
old_manifest=json.loads((O/'MANIFEST.json').read_bytes())
assert sha(O/'MANIFEST.json')=='5ccf8b443af960a0b5517ac079ae6d4a86749d414fefa4753f2360949bcaddec'
for row in old_manifest['files']:
 p=O/row['path'];assert sha(p)==row['sha256']and p.stat().st_size==row['bytes']
 copy(p,D/'original-one600-preparation'/row['path'])
 if row['path'].startswith(('current-source/','original-next-consumer-plan/')):
  copy(p,D/row['path'])
for name in ['MANIFEST.json','FREEZE.sha256']:copy(O/name,D/'original-one600-preparation'/name)
# The frozen original top-level manifest excludes nested manifest/freeze names.
# Its explicit source/expectation pin tables nevertheless name these public files.
listed={r['path']for r in old_manifest['files']};additional={}
pin_rows=json.loads((O/'SOURCE-PINS.json').read_bytes())['files']+json.loads((O/'EXPECTATION-PINS.json').read_bytes())['originalPlanFiles']
for row in pin_rows:
 physical=row['physical'];src=O/physical
 assert sha(src)==row['sha256']and src.stat().st_size==row['bytes']
 if physical not in listed:
  copy(src,D/physical);copy(src,D/'original-one600-preparation'/physical)
  additional[physical]={'path':physical,'sha256':sha(src),'bytes':src.stat().st_size,'basis':'Explicit original SOURCE-PINS/EXPECTATION-PINS row; original outer manifest excluded nested manifest/freeze names.'}
write('ADDITIONAL-PINNED-PUBLIC-INPUTS.json',{'originalManifestUnchanged':True,'explicitAdditionalFiles':list(additional.values()),'count':len(additional),'sourceOrOracleChanges':0})
for name in ['SOURCE-PINS.json','EXPECTATION-PINS.json','TOOL-PINS.json','ENVIRONMENT-CONTRACT.json','CURRENT-SOURCE-AND-EFFECT-BINDING.json']:
 copy(O/name,D/name)
for path in ['AGENTS.md','docs/MORIARTY-PRODUCT-CONTRACT.md']:
 copy(W/path,D/'startup-source'/path)
# Exact existing driver body; remove only the 600-specific adoption interface and retime it.
old=(O/'run_check.py').read_text()
assert sha(O/'run_check.py')=='4c9f285e0b8236974b5658b1def17f985d8e0d6a6e882ac9c0e477aa21d751da'
new=old.replace('PROPOSED one whole ordinary payout600TERM+5KILL/parent605; root adoption required.','Root-invoked one whole ordinary payout90TERM+5KILL/parent95; preparation only.')
start=new.index('def adoption(');end=new.index('\ndef main():',start);new=new[:start]+new[end+1:]
new=new.replace("    ap.add_argument('--root-adoption', type=Path, required=True)\n",'')
new=new.replace("    authorization = adoption(binding, args.binding_sha, args.root_adoption)\n", "    invocation = {'mode': 'root-selected ordinary90 whole target', 'bindingSha256': args.binding_sha,\n                  'hostVotesRequiredByThisOrdinaryDriver': False, '600AllocationGranted': False,\n                  'priorAllocationsReset': False, 'noAutomaticRetry': True}\n")
new=new.replace("'authorization': authorization","'invocation': invocation")
new=new.replace("'605 exhausted: membership unknown; no extra wait.'","'95 exhausted: membership unknown; no extra wait.'")
new=new.replace("prefix='doctorwatson-payout-one600-'","prefix='doctorwatson-payout-normal90-'")
new=new.replace("'600s', *command","'90s', *command")
new=new.replace("'termSeconds': 600","'termSeconds': 90").replace("'parentMaximumSeconds': 605","'parentMaximumSeconds': 95")
new=new.replace('deadline = start + 605','deadline = start + 95').replace('after605.','after95.')
assert 'adoption('not in new and '--root-adoption'not in new and "'600s'"not in new and 'start + 605'not in new
ast.parse(new);(D/'run_check.py').write_text(new)
(D/'DRIVER-DIFF.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='frozen-one600/run_check.py',tofile='ordinary-normal90/run_check.py')))
contract={'kind':'distinct ordinary90 whole unchanged payout consumer check; preparation only',
 'preparedOnly':True,'productExecutionAuthorizedByThisFile':False,'rootMustInspectAndSelectDispatch':True,
 'checkout':str(W),'head':'0345e172faf9e601404d7cb96211bc4e0255de4c','tree':'ea4b5c97779788ff25ca54f51725d6f759b688d9',
 'target':'mori::core_three_way_payout','wholeUnchangedTestFunctions':3,'genuineCaptureFiles':14,'actualPublicCliCalls':2,
 'termSeconds':90,'killGraceSeconds':5,'parentMaximumSeconds':95,'oneFixedFreshOutputAttempt':True,
 'freshColdTarget':True,'frozenOffline':True,'jobs':1,'defaultFeatures':False,'testThreads':1,'testFilter':None,
 'productEnvironmentExactly14':True,'original13EnvironmentPlusOnly':'MORI_PAYOUT_OBSERVATIONS',
 'nativeAllocationChanged':False,'600AllocationGranted':False,'priorAllocationsReset':False,
 '600SpecificTwoVoteInterfaceRemovedForDistinctOrdinary90':True,
 'oneFutureTimeoutConsumesThis90Attempt':True,'automaticRetryRenewalOrFallback':False,
 'scopeChange':'Timing reduced600/605 to90/95; unchanged full target/source/fixtures/expectations/environment/captures/CLI controls.',
 'limits':['This preparation does not dispatch the product target or a provider.',
 'Payout remains UNRUN on GP. No promised success or observed payout duration exists.',
 'Whole full source and actual-result audits remain required after execution; source-exposed expected Program and exact ASCII/keysort/indent/escape premises remain conditional.',
 'GNU timeout TERM90+KILL5 and parent monitoring95 cover the product phase; preflight and post-result metadata/hash/source checks run outside that phase and are not a95-total-driver-wall promise.',
 'Post-terminal census consumes only the same remaining parent deadline; unknown/live membership preserves the temporary root and fails the selected predicate.',
 'Binary hash is sampled AFTER the selected Cargo target returns; no pre-call binary hash or hook exists.',
 'Persistent root-original-input-copies are pre-execution fixture snapshots, not actual child temporary-project copies; the unchanged Project Drop removes its own temporary inputs.',
 'Previously observed escrow/financial timings and source equality do not predict this payout workload or justify600.',
 'All U/M/MC/SP/G, grammar/Program/Target/Artifact/composition/native/proof/ledger acceptance remains open.']}
write('ORDINARY-CHECK-CONTRACT.json',contract)
old_binding=json.loads((O/'BOUND-CANDIDATE.json').read_bytes());binding=dict(old_binding)
binding['actualOutput']=str(R/'payout-full-consumer-normal90-check-cycle-20261018')
binding['driverSha256']=sha(D/'run_check.py')
binding['controlHashes']={name:sha(D/name)for name in ['SOURCE-PINS.json','EXPECTATION-PINS.json','TOOL-PINS.json','ENVIRONMENT-CONTRACT.json','CURRENT-SOURCE-AND-EFFECT-BINDING.json','ORDINARY-CHECK-CONTRACT.json']}
binding.update(termSeconds=90,killGraceSeconds=5,parentMaximumSeconds=95)
binding['timeoutArgv']=list(binding['timeoutArgv']);assert binding['timeoutArgv'][3]=='600s';binding['timeoutArgv'][3]='90s'
binding.pop('hostAxes')
binding['scope']='One unchanged all-three-function ordinary payout target,14 genuine captures and two actual public simulates; no filter/source/default/native change;90TERM+5KILL/parent95.'
binding['limitations']=contract['limits'];binding['ordinaryAttemptPolicy']={'fixedNewOutput':True,'attempts':1,'noAutomaticRetryRenewalOrFallback':True,'no600AllocationOrCounterReset':True,'rootDispatchNotYetAuthorizedByPreparation':True}
write('BOUND-CANDIDATE.json',binding)
(D/'BINDING-DIFF.patch').write_text(''.join(difflib.unified_diff((O/'BOUND-CANDIDATE.json').read_text().splitlines(True),(D/'BOUND-CANDIDATE.json').read_text().splitlines(True),fromfile='frozen-one600/BOUND-CANDIDATE.json',tofile='ordinary-normal90/BOUND-CANDIDATE.json')))
public=[]
for role in ('a','b'):
 job=R/('payout-one600-host-'+role+'-canonical-20261018-a1')
 for name in ['START.json','CHILD.json','LIVE-METADATA.json','CLEANUP.json','TERMINAL-EXPORT-RECEIPT.json']:
  src=job/name;dst=D/'prior-host-public'/role/name;copy(src,dst);public.append({'role':role,'source':str(src),'physical':str(dst.relative_to(D)),'sha256':sha(src),'bytes':src.stat().st_size})
 assert not (job/'TERMINAL-REVIEW.json').exists()
 receipt=json.loads((job/'TERMINAL-EXPORT-RECEIPT.json').read_bytes());assert receipt['decision']=='NO_EXPORT'
write('PRIOR-HOST-PUBLIC-FACTS.json',{'kind':'fixed public metadata only; no private capture read',
 'files':public,'A':{'terminalExitCode':3,'parsedTopLevelValues':4,'resultEvents':1,'terminalStatusLiteral':None,'decision':'NO_EXPORT','reason':'NONZERO_OR_MISSING_EXIT'},
 'B':{'terminalExitCode':0,'parsedTopLevelValues':3,'resultEvents':1,'terminalStatusLiteral':'SUCCESS','decision':'NO_EXPORT','reason':'MISSING_STRUCTURED_OUTPUT'},
 'wholeOpinions':0,'resourceVotes':0,'600AdoptionOrAllocation':False,'automaticProviderRetry':False,'causeOrOutageClaim':'NONE; raw private captures were not inspected'})
write('CHANGE-LEDGER.json',{'originalDriverSha256':sha(O/'run_check.py'),'newDriverSha256':sha(D/'run_check.py'),
 'originalBindingSha256':sha(O/'BOUND-CANDIDATE.json'),'newBindingSha256':sha(D/'BOUND-CANDIDATE.json'),
 'changes':['Separate normal90 preparation and absent output names, plus normal90 temporary-root prefix.',
 '90TERM+5KILL/parent95 replaces600TERM+5KILL/parent605; diagnostic sampling stays bounded to first90seconds.',
 'Removed adoption() and --root-adoption two-HOST interface; replaced authorization metadata by factual root-invocation mode, exact binding and no600 grant/reset.',
 'Active ordinary-check contract replaces600 resource-proposal/rationale binding controls. Original files remain intact under original-one600-preparation.',
 'No other driver control flow or expected source, fixtures, test bodies, cargo flags,14 environment fields,14 captures or two public CLI argument vectors changed.'],
 'fullDiff':'DRIVER-DIFF.patch and BINDING-DIFF.patch','sourceChanges':0,'oracleChanges':0,'testFilterAdded':False,'sourceResultAuditsStillRequired':True})
command='/usr/bin/python3 '+str(D/'run_check.py')+' --binding-sha '+sha(D/'BOUND-CANDIDATE.json')
(D/'ROOT-COMMAND.md').write_text('# Root command — prepared only\n\nRoot must inspect the whole driver, binding and controls before deciding dispatch. The fixed check-cycle path must remain absent. This invokes the full unchanged target once; it does not retry the failed HOST transport or allocate600.\n\n```bash\n'+command+'\n```\n\nA timeout consumes this ordinary90 attempt. No immediate retry, renewal, fallback, new target filter or600 allocation is authorized.\n')
print(json.dumps({'driverSha256':sha(D/'run_check.py'),'bindingSha256':sha(D/'BOUND-CANDIDATE.json'),'originalFrozenPayloads':len(old_manifest['files']),'rootCommand':command,'actualOutputAbsent':not Path(binding['actualOutput']).exists(),'productOrProviderExecuted':False}))
