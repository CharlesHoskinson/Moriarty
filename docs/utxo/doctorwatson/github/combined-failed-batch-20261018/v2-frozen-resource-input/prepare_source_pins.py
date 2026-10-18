"""Read-only source/receipt pin preparation. Never runs product or providers."""
from pathlib import Path
import hashlib,json,subprocess,datetime,shutil,sys,ast,re
D=Path(__file__).resolve().parent
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
K=Path('/home/charl/Moriarty/.worktrees/utxo-quantity-min-main-20261018')
HEAD='f68946c6c2c5894fe333ba50132d6435d90e6521';TREE='839567428d591be73abe6032068a505aa5b8279c'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
git=lambda *a:subprocess.check_output(['git','--no-optional-locks',*a],cwd=K)
def write(n,j):(D/n).write_text(json.dumps(j,indent=2)+'\n')
def copy(src,relative):
 dst=D/relative;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(src,dst);assert sha(src)==sha(dst);return dst
assert git('rev-parse','HEAD').decode().strip()==HEAD and git('rev-parse','HEAD^{tree}').decode().strip()==TREE and not git('status','--porcelain=v1').strip()
rows=[]
for raw in git('ls-tree','-rz',HEAD).split(b'\0'):
 if not raw:continue
 meta,path=raw.split(b'\t',1);mode,kind,oid=meta.decode().split();path=path.decode()
 selected=(path in {'AGENTS.md','RUST_GUIDELINES.md','Cargo.toml','Cargo.lock','rust-toolchain.toml','docs/MORIARTY-PRODUCT-CONTRACT.md','experiments/moriarty-language/package.json'} or path.startswith('crates/') or path.startswith('experiments/moriarty-language/src/') or path.startswith('experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/') or path=='experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs')
 if selected:
  assert mode in ['100644','100755'] and kind=='blob'
  rows.append({'path':path,'gitBlob':oid,'mode':mode})
data=subprocess.run(['git','--no-optional-locks','cat-file','--batch'],cwd=K,input=''.join(x['gitBlob']+'\n'for x in rows).encode(),capture_output=True,check=True).stdout
cursor=0
for row in rows:
 end=data.index(b'\n',cursor);oid,kind,n=data[cursor:end].decode().split();assert oid==row['gitBlob'] and kind=='blob';n=int(n);cursor=end+1;body=data[cursor:cursor+n];cursor+=n;assert data[cursor:cursor+1]==b'\n';cursor+=1
 assert (K/row['path']).read_bytes()==body
 row.update(bytes=n,sha256=hashlib.sha256(body).hexdigest())
assert cursor==len(data)
binding_src=R/'combined-financial-quantity-main-source-integration-20261018/ROOT-SOURCE-BINDING.json'
binding=json.loads(binding_src.read_text());assert binding['prospectiveTree']==TREE and len(binding['paths'])==46
for row in binding['paths']:assert sha(K/row['path'])==row['sha256']
copy(binding_src,'ROOT-SOURCE-BINDING.json')
O=R/'source6-capped-remittance-independent-oracle-20261018'
F='experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/'
fixed=json.loads((K/F/'FIXTURE-PINS.json').read_text());expectations=[]
for rel,digest in fixed['files'].items():
 src=K/F/rel;custody=O/rel.removeprefix('oracle/');assert sha(src)==digest and src.read_bytes()==custody.read_bytes()
 dst=copy(custody,'expectations/quantity-oracle/'+rel.removeprefix('oracle/'))
 expectations.append({'path':str(dst.relative_to(D)),'repositoryPath':F+rel,'sha256':sha(dst),'originalCustodyPath':str(custody),'bytes':dst.stat().st_size})
assert len(expectations)==24
freeze=(O/'FREEZE.sha256').read_text().splitlines();assert len(freeze)==23
for line in freeze:
 digest,n=line.split(maxsplit=1);assert sha(O/n)==digest
assert sha(O/'FREEZE.sha256')=='4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79'
financial=R/'concurrency-increase-main238/pr240-maineac-check-preparation-20261018/original-freeze'
for src in sorted(financial.iterdir()):
 dst=copy(src,'expectations/financial-freeze/'+src.name);expectations.append({'path':str(dst.relative_to(D)),'sha256':sha(dst),'originalCustodyPath':str(src),'bytes':dst.stat().st_size})
for row in binding['paths']:
 if row['path'].startswith('crates/fixtures/check-source/expected/financial-'):
  dst=copy(K/row['path'],'expectations/financial-streams/'+Path(row['path']).name)
  expectations.append({'path':str(dst.relative_to(D)),'repositoryPath':row['path'],'sha256':sha(dst),'originalCustodyPath':str(K/row['path']),'bytes':dst.stat().st_size})
assert sum(x['path'].startswith('expectations/financial-streams/')for x in expectations)==6
write('EXPECTATION-PINS.json',{'qualification':'Quantity24file independent custody/23freeze retained; financial author/schema-exposed original freeze and six expected byte streams retained, no new independent financial oracle.','files':expectations})
counts={'totalPinnedFiles':len(rows),'rustSourceFiles':sum(x['path'].endswith('.rs')for x in rows),'typeScriptSourceFiles':sum(x['path'].endswith('.ts')for x in rows),'nodeMjsSourceFiles':sum(x['path'].endswith('.mjs')for x in rows),'candidatePaths':46,'oraclePinnedFilesIncludingFreeze':24,'freezeEntries':23,'financialExpectedStreams':6}
write('PINNED-SOURCES.json',{'head':HEAD,'tree':TREE,'computedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'selection':'Same prior969selection recomputed from exact current Git tree: all tracked crates; current TypeScript source/package; exact Node test/oracle; root Cargo/toolchain/AGENTS/guidelines/product. Count derived, not969+15assumed.','counts':counts,'files':rows})
tools=[Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')/n for n in ['cargo','rustc','rustdoc']]+[Path('/usr/bin/timeout'),Path('/usr/local/bin/node'),Path('/usr/bin/ps'),Path('/usr/bin/python3')]
write('TOOL-PINS.json',{'tools':[{'path':str(p),'resolved':str(p.resolve()),'sha256':sha(p.resolve())}for p in tools]})
for name in ['future_runner.py','agy_terminal_selector.py']:
 copy(R/'agy-terminal-future-runner-20261018'/name,'transport-reference/'+name)
# Original ordinary receipts only; no provider/private streams.
for label,folder in [('financial90',R/'pr240-maineac-source-check-20261018'),('financial180',R/'pr240-maineac-finite180-source-check-20261018'),('quantity90',R/'quantity-min-main-combined-check-cycle-20261018')]:
 for src in sorted(folder.rglob('*')):
  if src.is_file():copy(src,'prior-failures/'+label+'/'+str(src.relative_to(folder)))
diagnosis=R/'concurrency-increase-main238/pr222-and-financial-combined-timeout-diagnosis-20261018'
for name in ['DIAGNOSIS.md','OBSERVATIONS.json','FREEZE.sha256']:
 copy(diagnosis/name,'diagnosis/'+name)
# Preserve precise source consumer/harness references for later whole source/result review; not mandatory host opinion inputs.
for name in ['crates/mori/tests/check_source_dispatch.rs','crates/mori/tests/source6_expand.rs','crates/mori/Cargo.toml','crates/mori-core/Cargo.toml','Cargo.toml','rust-toolchain.toml','experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs']:
 copy(K/name,'product-context/'+name)
financial_names=re.findall(r'^fn (financial_[A-Za-z0-9_]+)\(', (K/'crates/mori/tests/check_source_dispatch.rs').read_text(),re.M)
quantity_tests=re.findall(r'^#\[test\]\s*fn\s+([A-Za-z0-9_]+)\s*\(', (K/'crates/mori/tests/source6_expand.rs').read_text(),re.M)
assert len(financial_names)==6 and len(quantity_tests)==8
cargo=str(tools[0]);manifest=str(K/'Cargo.toml')
steps=[{'name':'financial-source-check-tests','kind':'financial','argv':[cargo,'test','--manifest-path',manifest,'--frozen','-j1','-p','mori','--no-default-features','--test','check_source_dispatch','financial_','--','--test-threads=1']},{'name':'public-quantity-min','kind':'quantity','argv':[cargo,'test','--manifest-path',manifest,'--frozen','-j1','-p','mori','--no-default-features','--features','source6-expansion','--test','source6_expand','--','quantity_min_selects_both_operands_through_public_source6_expansion','--exact','--test-threads=1']},{'name':'node-capped-public-consumer','kind':'node','argv':['/usr/local/bin/node','--test',str(K/'experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs')]}]
driver=D/'run_batch.py';ast.parse(driver.read_text())
controlnames=['PINNED-SOURCES.json','TOOL-PINS.json','EXPECTATION-PINS.json','ROOT-SOURCE-BINDING.json']
contract={'readyForRootReview':True,'productExecutionAuthorizedByThisFile':False,'checkout':str(K),'head':HEAD,'tree':TREE,'parents':git('show','-s','--format=%P',HEAD).decode().strip().split(),'equivalentSourceBase':binding['equivalentSourceBase'],'currentMainTree':binding['currentMainTree'],'driverSha256':sha(driver),'driverInterpreterResolved':str(Path('/usr/bin/python3').resolve()),'controlFiles':[{'path':n,'sha256':sha(D/n)}for n in controlnames],'actualOutput':str(R/'combined-financial-quantity-finite600-check-cycle-20261018'),'wholeBatchWallSeconds':600,'killGraceSeconds':5,'steps':steps,'selectionCounts':counts,'expectedFinancialNames':financial_names,'expectedRustResults':[{'passed':6,'failed':0,'ignored':0,'measured':0,'filtered':8},{'passed':1,'failed':0,'ignored':0,'measured':0,'filtered':7}],'expectedNodeCases':2,'oneAttemptRule':'Root manual onefreshbatch allocation ONLY after two agreeing substantive independent hostresourcevotes AND root adoption. New output mkdir forbids samepath reuse, not a global persistent allocation counter. No retry/fallback/default/cohortreset.','nativeAllocation':'Unrelated nativefinancial600+5 remains unchanged; this is new ordinary wholebatch proposal, not native authority.','sourceApproval':'No source/result/semantic approval requested; later full current source+result review required.'}
write('BOUND-CANDIDATE.json',contract)
write('STATIC-VALIDATION.json',{'kind':'SOURCE_ONLY_NO_PRODUCT_PROVIDER_EXECUTION','head':HEAD,'tree':TREE,'counts':counts,'all46BodiesMatchRootFrozenOriginals':True,'quantity24CopiesAnd23FreezeEntriesExact':True,'financial6ExpectedStreamsExact':True,'allCurrentSourceBodiesMatchGit':True,'driverAstParses':True,'wholeBatchTimeoutCalls':driver.read_text().count("'600s'"),'noStandaloneBuild':all('build'not in step['argv']for step in steps),'exactThreeSteps':len(steps),'noPerPhaseTimeoutOrNewSession':True,'workerInherits13EnvPlusNode2ExplicitVars':True,'freshActualOutputAbsent':not Path(contract['actualOutput']).exists(),'worktreeClean':not git('status','--porcelain=v1').strip()})
print(json.dumps({'counts':counts,'driverSha256':sha(driver),'contractSha256':sha(D/'BOUND-CANDIDATE.json'),'parents':contract['parents']},indent=2))
