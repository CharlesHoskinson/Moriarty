from pathlib import Path
import ast,difflib,hashlib,json,stat
D=Path(__file__).resolve().parent;R=D.parent
P=R/'payout-full-consumer-normal90-preparation-20261018'
W=Path('/home/charl/Moriarty/.worktrees/utxo-guarded-full-prefix-main-20261018')
OR=R/'concurrency32-wave-20261018/two_program_financial_oracles'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(name,value):
 p=D/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(value,indent=2)+'\n')
def copy(src,dst):
 assert not src.is_symlink()and stat.S_ISREG(src.lstat().st_mode)
 assert all('.private'not in part for part in src.parts)
 dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(src.read_bytes());assert sha(src)==sha(dst)
assert sha(P/'MANIFEST.json')=='e53392f80e4daa1e03d3ed884fbdb767cffb55f7ccba46aa4f98c7a3adf1e590'
prior_rows=json.loads((P/'MANIFEST.json').read_bytes())['files']
for row in prior_rows:
 if row['path'].startswith(('current-source/','original-next-consumer-plan/')):
  src=P/row['path'];assert sha(src)==row['sha256']and src.stat().st_size==row['bytes'];copy(src,D/row['path'])
for name in ['run_check.py','BOUND-CANDIDATE.json','ENVIRONMENT-CONTRACT.json','CURRENT-SOURCE-AND-EFFECT-BINDING.json','STATIC-VALIDATION.json']:
 copy(P/name,D/'reference-payout-controls'/name)
for name in ['TOOL-PINS.json','EXPECTATION-PINS.json']:copy(P/name,D/name)
# Current task forbids Git and runtime calls; retain latest startup observations explicitly as historical.
for name in ['STARTUP-STATUS.json','STARTUP-READONLY-METHOD.json','CURRENT-GP-METADATA.json']:
 copy(P/name,D/'startup-reference'/name)
write('STARTUP-METHOD.json',{'skillRead':'moriarty-dev:develop','checkout':str(W),'AGENTSAndProductContract':'Full unchanged bodies retained in current-source; read in preceding same-checkout task.',
 'status':'Latest ordinary90 startup status retained verbatim under startup-reference; no fresh status invocation because it invokes Git and this task prohibits Git.',
 'currentHeadClaim':'Bound to GP0345 from preceding exact metadata; pure current-file pin checks now, fresh Git HEAD/tree/clean check deferred to root driver preflight.',
 'noGitOrProductCommandExecuted':True,'historicalSP01BlockersNotDispatchedOrReset':True})
# Copy every reserve fixture by an explicit public filename list, not a traversal.
fixture_names=['ORACLE-CUSTODY.json','ORACLE-FREEZE.sha256','reserve-spend-refund.mori','opening.scenario.json','financial-execution-projection.expected.json','cumulative-prefix-effects.expected.json','source-statement-kinds.expected.json']
source_table=json.loads((P/'SOURCE-PINS.json').read_bytes())
for name in fixture_names:
 relative='crates/fixtures/reserve-spend-refund/'+name;copy(W/relative,D/'current-source'/relative)
 row=next(r for r in source_table['files']if r['path']==relative)
 assert sha(W/relative)==row['sha256']and (W/relative).stat().st_size==row['bytes']
copy(P/'SOURCE-PINS.json',D/'SOURCE-PINS.json')
custody=json.loads((W/'crates/fixtures/reserve-spend-refund/ORACLE-CUSTODY.json').read_bytes())
assert sha(OR/'FREEZE.sha256')==custody['original_freeze_sha256']=='1b606141dff7ca162bcd132d55d62014187de4f936cfc6528c23c8d22591aefb'
assert (OR/'FREEZE.sha256').read_bytes()==(W/'crates/fixtures/reserve-spend-refund/ORACLE-FREEZE.sha256').read_bytes()
oracle_rows=[]
for line in (OR/'FREEZE.sha256').read_text().splitlines():
 digest,relative=line.split('  ',1);assert not Path(relative).is_absolute()and '..'not in Path(relative).parts
 src=OR/relative;assert sha(src)==digest;copy(src,D/'independent-reserve-oracle'/relative)
 oracle_rows.append({'physical':'independent-reserve-oracle/'+relative,'sha256':digest,'bytes':src.stat().st_size})
copy(OR/'FREEZE.sha256',D/'independent-reserve-oracle/FREEZE.sha256')
oracle_rows.append({'physical':'independent-reserve-oracle/FREEZE.sha256','sha256':sha(OR/'FREEZE.sha256'),'bytes':(OR/'FREEZE.sha256').stat().st_size})
expected=json.loads((P/'EXPECTATION-PINS.json').read_bytes());expected['independentReserveOracleFiles']=oracle_rows
expected['reserveFixtureWholePacketCustody']=custody;write('EXPECTATION-PINS.json',expected)
for name,digest in custody['selected_files'].items():assert sha(W/'crates/fixtures/reserve-spend-refund'/name)==digest
base_env=json.loads((P/'ENVIRONMENT-CONTRACT.json').read_bytes());base_env['onlyAddition']={'MORI_CORE_REFUND_OBSERVATIONS':'<FIXED_OUTPUT>/observations'}
base_env['captureRoot']='Absolute absent directory with existing cycle parent. Existing Project::new create_dir; two inputs copied there and raw output/process files fs::write; temporary=false retains actual child project on success and failure. No create_new or library capture hook claimed.'
write('ENVIRONMENT-CONTRACT.json',base_env)
old=(P/'run_check.py').read_text();new=old.replace('payout90TERM','reserve-spend-refund90TERM').replace('Three unchanged Rust functions, one cold cargo target, two real public CLI calls.','Two unchanged Rust functions, one cold cargo target, one real public CLI call.')
new=new.replace('payout-normal90-','reserve-spend-refund-normal90-').replace('MORI_PAYOUT_OBSERVATIONS','MORI_CORE_REFUND_OBSERVATIONS').replace("'core_three_way_payout'","'core_reserve_spend_refund'")
new=new.replace("        expected = json.loads((D / 'EXPECTATION-PINS.json').read_bytes())\n", "        expected = json.loads((D / 'EXPECTATION-PINS.json').read_bytes())\n        for row in expected['independentReserveOracleFiles']:\n            p = require_public_file(D / row['physical'], D)\n            assert p.stat().st_size == row['bytes'] and sha(p) == row['sha256']\n")
a=new.index('        # These root copies preserve original fixture inputs;');z=new.index('        command = ',a);new=new[:a]+new[z:]
new=new.replace("'oneWholeTargetAll3Tests': True","'oneWholeTargetAll2Tests': True")
new=new.replace("'inputCopyProvenance': 'Root preexecution fixture snapshots, not actual child temporary-project captures.'","'inputCopyProvenance': 'Existing Project::new copies actual two inputs into persistent observations; bytes are checked after selected target only.'")
new=new.replace("'expectedCount': 14","'expectedCount': 5").replace('(3, 0, 0, 0, 0)','(2, 0, 0, 0, 0)').replace("'passed': 3","'passed': 2")
a=new.index("                assert len(result.get('captureFiles', [])) == 14");z=new.index("                result['wholeSelectedTargetPredicate'] = True",a)
new=new[:a]+'''                assert len(result.get('captureFiles', [])) == 5 and all(x['exists'] for x in result['captureFiles'])
                for name in ['reserve-spend-refund.mori', 'opening.scenario.json']:
                    assert (cycle / 'observations' / name).read_bytes() == (checkout / 'crates/fixtures/reserve-spend-refund' / name).read_bytes()
                process = json.loads((cycle / 'observations/process.json').read_bytes())
                assert set(process) == {'executable', 'args', 'status', 'success'}
                assert process['args'] == binding['publicCliArgv'][0] and process['status'] == 0 and process['success'] is True
                assert process['executable'] == str(root / 'target/debug/mori')
                assert (cycle / 'observations/stderr.txt').read_bytes() == b''
                report = json.loads((cycle / 'observations/stdout.json').read_bytes())
                assert set(report) == {'execution', 'qualification', 'status'}
                assert report['status'] == 'accepted' and report['qualification'] == 'local-reference-execution'
''' +new[z:]
assert 'three-way-payout'not in new and 'MORI_PAYOUT'not in new and 'All3Tests'not in new and "'expectedCount': 14"not in new
ast.parse(new);(D/'run_check.py').write_text(new)
(D/'DRIVER-DIFF.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='inspected-payout-normal90/run_check.py',tofile='reserve-normal90/run_check.py')))
functions=['reserve_library_keeps_complete_financial_prefixes','reserve_public_cli_keeps_complete_financial_prefixes']
captures=['reserve-spend-refund.mori','opening.scenario.json','stdout.json','stderr.txt','process.json']
cli_args=['--color','never','simulate','reserve-spend-refund.mori','--action','run','--scenario','opening.scenario.json']
closure=json.loads((P/'CURRENT-SOURCE-AND-EFFECT-BINDING.json').read_bytes())
closure.update(target='mori::core_reserve_spend_refund',functions=functions,captures=captures)
closure['selectedEffectPath']={'consumer':'reserve-spend-refund','callPath':['library test: fixture source -> mori_core::parse -> mori_core::simulate(program,run,scenario) -> runtime::simulate_with_meter(None) -> financial projection assertions',
 'CLI test: CARGO_BIN_EXE_mori --color never simulate reserve-spend-refund.mori --action run --scenario opening.scenario.json',
 'main/CLI -> commands::simulate::run -> legacy inputs; no resume/trace/target/authorization/logical-cap -> core_io::read_source -> language Core1 -> parse -> read_json -> simulate -> core_io::print/json::export'],
 'sourceReads':['Full reserve source/scenario and independent financial projection/kinds/prefix fixtures','Public CLI reads the two actual observation-directory copies'],
 'writes':['Existing Project::new creates fresh persistent observations and fs::copy inputs','Existing fs::write raw stdout.json/stderr.txt/process.json before result assertions; failure launchError process record if spawn fails'],
 'childProcesses':['Exactly one public unmetered simulate Command::output call; no native/replay/target flags selected'],
 'libraryCaptureHook':None,'captureEnvironment':'MORI_CORE_REFUND_OBSERVATIONS'}
closure['captureOrder']='Actual two inputs copied before spawn. On launched process, stdout/stderr/process written before exit/envelope/financial assertions; launch error is written before expect panic. Library parse/execution is not separately captured.'
closure['temporaryInputs']='Observation-hook Project temporary=false; actual two input copies remain in persistent cycle directory. No separate root-original-input snapshots or claimed Drop deletion.'
closure['expectationPremises']='Independent financial projection/full20snapshots/values/effect deltas and cumulative prefixes/source kinds. Whole Program/ExprIds/raw Statement fields/exact wire/logical work remain unasserted; informational work60 is not observed or enforced.'
closure['runtimeStatus']='RESERVE_UNRUN_ON_GP; prior payout/escrow/invoice/ACTUS results do not transfer'
closure['binaryIdentity']='Process record exposes CARGO_BIN_EXE path only; driver samples binary hash AFTER whole target, no pre-call hash hook.'
write('CURRENT-SOURCE-AND-EFFECT-BINDING.json',closure)
limits=['Prepared only: reserve target UNRUN; root must inspect and decide dispatch.',
 'Whole two-test target, no filter; one actual CLI call, no meter/cap, no library Program/Execution capture hook.',
 'Existing five-file persistent hook uses create_dir plus copy/fs::write, not create_new. Preserve every actual partial/full capture on failure.',
 'Only original financial projection/source kinds/cumulative-prefix comparisons and public envelope/status are asserted. Whole Program/ExprIds/raw Statement fields/exact wire/work/target are not independently asserted.',
 'Original WORK-SCOPE dispatches20/informational sum60 remains informational only, no new cap or observed cost.',
 'Binary hash sampled AFTER selected target; input copies are actual persistent Project files. No pre-call hash or source change.',
 '90TERM+5KILL/parent95 applies to product phase, not total Python wall. Metadata outside that phase and remaining-deadline census limits remain.',
 'Unknown/live owned session preserves root and fails selected predicate. Timeout consumes ordinary90 attempt; no retry/renewal/fallback/resource increase or600 allocation.',
 'All original8scope/source+result audits and148/292/20/U/M/MC/SP/G/ACTUS/DeFi/composition/grammar/Program/Target/Artifact/native/proof/ledger obligations remain open.']
contract={'preparedOnly':True,'productExecutionAuthorizedByThisFile':False,'ordinaryTiming':[90,5,95],'rootMustInspectAndSelectDispatch':True,'wholeTestFunctions':functions,'captureFiles':captures,'oneActualPublicCliArgv':cli_args,'exactProductEnvironmentFields':14,'onlyObservationAddition':'MORI_CORE_REFUND_OBSERVATIONS','sourceOrOracleChange':False,'oneAttemptNoRetryRenewalOrFallback':True,'600AllocatedOrPriorCountersReset':False,'limitations':limits}
write('ORDINARY-CHECK-CONTRACT.json',contract)
b=json.loads((P/'BOUND-CANDIDATE.json').read_bytes());b['actualOutput']=str(R/'reserve-spend-refund-normal90-check-cycle-20261018');b['driverSha256']=sha(D/'run_check.py')
b['controlHashes']={n:sha(D/n)for n in ['SOURCE-PINS.json','EXPECTATION-PINS.json','TOOL-PINS.json','ENVIRONMENT-CONTRACT.json','CURRENT-SOURCE-AND-EFFECT-BINDING.json','ORDINARY-CHECK-CONTRACT.json']}
b['cargoArgv']=[('core_reserve_spend_refund'if x=='core_three_way_payout'else x)for x in b['cargoArgv']];b['timeoutArgv']=b['timeoutArgv'][:4]+b['cargoArgv'];b.update(target='core_reserve_spend_refund',testFunctions=functions,captureFiles=captures,publicCliArgv=[cli_args],scope='One unchanged whole two-function reserve-spend-refund target; one unmetered public CLI, five persistent hook files,90TERM+5KILL/parent95; no source/filter/oracle change.',limitations=limits)
write('BOUND-CANDIDATE.json',b)
write('ORIGINAL-EIGHT-SCOPE-MAPPING.json',{'originalScope':'Original PR232 three and PR234 five functions remain eight distinct obligations, never replaced by this target.',
 'originalTable':'original-next-consumer-plan/FULL-EIGHT-SCOPE-CUSTODY.json','newFiveTable':'original-next-consumer-plan/NEW-FIVE-CONSUMERS.json',
 'selectedOverlapTarget':{'package':'mori','target':'core_reserve_spend_refund','functions':functions,'wholeBodySha256':sha(W/'crates/mori/tests/core_reserve_spend_refund.rs'),'originalWholeBodySha256':'27c502d178d7422b2c53f03e1b294cdb792dfc719aac1e772ca2cf405b514b90','preparedRuntime':'UNRUN'},
 'otherOverlapTarget':{'package':'mori-core','target':'program_plan_roundtrip','function':'invoice_program_plan_and_zero_duty_continuation_roundtrip_as_complete_values','noNewExecutionOrPassTransferClaim':True},
 'thisTargetPreparationDoesNotCompleteOriginalEight':True,'sourceAndActualResultReviewsStillRequired':True})
command='/usr/bin/python3 '+str(D/'run_check.py')+' --binding-sha '+sha(D/'BOUND-CANDIDATE.json')
(D/'ROOT-COMMAND.md').write_text('# Root command — prepared only\n\nInspect complete driver, binding, full consumer/closure and preserved oracle first; then root decides dispatch. This preparation runs nothing. The fixed output must be absent.\n\n```bash\n'+command+'\n```\n\nOne ordinary90 attempt; a future timeout consumes it. No immediate retry, renewal, fallback,600 allocation or earlier-counter reset.\n')
print(json.dumps({'driverSha256':sha(D/'run_check.py'),'bindingSha256':sha(D/'BOUND-CANDIDATE.json'),'sourcePins':len(source_table['files']),'oraclePayloads':len(oracle_rows)-1,'captureFiles':captures,'actualOutputAbsent':not Path(b['actualOutput']).exists(),'rootCommand':command,'productProviderGitCalls':0}))
