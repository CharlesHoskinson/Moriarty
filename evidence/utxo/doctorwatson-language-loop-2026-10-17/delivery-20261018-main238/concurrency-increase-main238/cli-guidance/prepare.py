from pathlib import Path
import subprocess,json,hashlib,shutil,re
O=Path(__file__).resolve().parent;J='/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018';R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');H='f2975f1fbfbaf060207a9d26331ad52745ce5247';B='71c22f8a477231be869786eb42f04e3b50d1b3a7';OLD='8269b5b7ceeec5169c706aa31ea33f390d49c969';P=R/'cli-profile-guidance-main8269-source-result-review-20261018'
def git(*a):return subprocess.check_output(['git',*a],cwd=J)
def sha(b):return hashlib.sha256(b).hexdigest()
def dump(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def body(c,p):return git('show',c+':'+p)
def objhash(kind,b):return hashlib.sha1(kind.encode()+b' '+str(len(b)).encode()+b'\0'+b).hexdigest()
def store(rel,b):p=O/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
changed=['crates/mori/src/cli.rs','crates/mori/src/commands/expand.rs','docs/NATIVE-CORE.md'];pins=[];afters={}
for p in changed:
 before=body(B,p);after=body(H,p);assert before==body(OLD,p);afters[p]=after
 store('before/'+p,before);store('after/'+p,after)
 pins.append({'path':p,'mode':'100644','before':{'commit':B,'gitBlob':git('rev-parse',B+':'+p).decode().strip(),'sha256':sha(before),'bytes':len(before)},'after':{'sourceCommit':H,'gitBlob':git('rev-parse',H+':'+p).decode().strip(),'sha256':sha(after),'bytes':len(after)},'currentBeforeByteIdenticalToReviewedBase':True,'afterVerbatimReviewedF297':True})
patch=git('diff',OLD,H,'--',*changed);store('CANDIDATE.patch',patch)
assert patch==(P/'ACTUAL-THREE-PATH.patch').read_bytes()
# Parse each unified hunk and reconstruct the entire changed file from current-base bytes.
ps=patch.decode().splitlines(keepends=True);reconstructed={};i=0
while i<len(ps):
 if not ps[i].startswith('diff --git '):i+=1;continue
 path=ps[i].split(' b/',1)[1].rstrip('\n');base=body(B,path).decode().splitlines(keepends=True);out=[];pos=0;i+=1
 while i<len(ps) and not ps[i].startswith('diff --git '):
  if not ps[i].startswith('@@ '):i+=1;continue
  m=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',ps[i]);assert m
  start=int(m[1])-1;assert start>=pos;out+=base[pos:start];pos=start;i+=1;oldn=newn=0
  while i<len(ps) and not ps[i].startswith(('@@ ','diff --git ')):
   line=ps[i]
   if line.startswith((' ','-')):assert base[pos]==line[1:];pos+=1;oldn+=1
   if line.startswith((' ','+')):out.append(line[1:]);newn+=1
   assert line[0] in ' +-';i+=1
  assert oldn==int(m[2] or 1) and newn==int(m[4] or 1)
 out+=base[pos:];reconstructed[path]=''.join(out).encode();assert reconstructed[path]==afters[path]
assert set(reconstructed)==set(changed)
# Pure in-memory Git tree hashing; creates no Git index/object/worktree writes.
entries={}
for row in git('ls-tree','-r','-z',B).split(b'\0'):
 if not row:continue
 meta,p=row.split(b'\t',1);mode,kind,oid=meta.split();entries[p.decode()]=(mode.decode(),oid.decode())
def tree_hash(es):
 root={}
 for p,value in es.items():
  d=root;parts=p.split('/')
  for part in parts[:-1]:d=d.setdefault(part,{})
  d[parts[-1]]=value
 def encode(d):
  chunks=[]
  for name,v in sorted(d.items(),key=lambda kv:kv[0].encode()+(b'/' if isinstance(kv[1],dict) else b'')):
   mode,oid=('40000',encode(v)) if isinstance(v,dict) else v
   chunks.append(mode.encode()+b' '+name.encode()+b'\0'+bytes.fromhex(oid))
  return objhash('tree',b''.join(chunks))
 return encode(root)
base_tree=git('rev-parse',B+'^{tree}').decode().strip();assert tree_hash(entries)==base_tree
for p,b in afters.items():entries[p]=('100644',objhash('blob',b))
expected=tree_hash(entries)
prior=json.loads((P/'CURRENT-SOURCE-PINS.json').read_text());context=[]
paths=[v['path'] for v in prior['files']]+['crates/mori-core/src/parser.rs','crates/mori-core/tests/language.rs','crates/mori-core/Cargo.toml','RUST_GUIDELINES.md','docs/MORIARTY-PRODUCT-CONTRACT.md']
for p in paths:
 b=afters.get(p,body(B,p));store('candidate-context/'+p,b)
 context.append({'path':p,'sha256':sha(b),'bytes':len(b),'gitBlob':objhash('blob',b),'sourceCommit':H if p in afters else B,'changedByProposal':p in afters,'sameAsF297':b==body(H,p) if p not in ['crates/mori-core/src/parser.rs','crates/mori-core/tests/language.rs'] else False})
store('CURRENT-HEADER-DIAGNOSTIC.patch',git('diff',OLD,B,'--','crates/mori-core/src/parser.rs','crates/mori-core/tests/language.rs'))
shutil.copytree(P/'source-preparation/doc-shell',O/'doc-shell')
for n in ['EXPECTED-CLAP-CONTRACT.json','DOC-EXPECTATIONS.json','DOC-COMMAND-PREDICATES.json']:
 store('historical-expectations/'+n,(P/'source-preparation'/n).read_bytes())
lang=body(B,'crates/mori-core/tests/language.rs').decode();assert 'fn language_header_diagnostics_name_both_supported_core_profiles()' in lang
cli=afters['crates/mori/src/cli.rs'].decode();assert 'fn help_distinguishes_core_consumers_from_source6_proposals()' in cli
corecargo=body(B,'crates/mori-core/Cargo.toml').decode();assert 'midnight-compat-tests = [' in corecargo
commands=[{'id':'clap-help','argv':['cargo','test','--frozen','-j1','-p','mori','--no-default-features','--bin','mori','cli::tests::help_distinguishes_core_consumers_from_source6_proposals','--','--exact','--test-threads=1'],'predicate':'One existing Clap schema metadata test; names two Core headers and explicit Source6 boundaries; no command execution/help snapshot.'},{'id':'current-core-header','argv':['cargo','test','--frozen','-j1','-p','mori-core','--no-default-features','--test','language','language_header_diagnostics_name_both_supported_core_profiles','--','--exact','--test-threads=1'],'predicate':'Existing PR238 regression: both accepted header probes and all4 explicit diagnostic prefix/code/offset/suffix cases. No Midnight compatibility feature.'}]
for n in range(1,6):commands.append({'id':'doc-shell-'+str(n),'argv':['bash','-n',str(O/'doc-shell'/f'{n:02}.sh')],'predicate':'Syntax only; never execute script body/product examples.'})
dump(O/'ORDINARY-REGRESSION-ARGV.json',{'status':'INSPECTED_UNRUN_ON_PROSPECTIVE_MAIN71_TREE','commands':commands,'executionPreconditions':'Root sole integrator checks exact candidate/current pins and uses existing offline execution/resource plan; no new execution granted by this packet.','sideEffectsIfRootAuthorizes':'Cargo build products and selected metadata/header tests; shell parser only. No compiler/native/SDK/proof/key/wallet/ledger operation selected. Broad suites remain unrun.'})
dump(O/'SOURCE-PINS.json',{'base':B,'baseTree':base_tree,'reviewedHistoricalHead':H,'reviewedHistoricalTree':git('rev-parse',H+'^{tree}').decode().strip(),'changedFiles':pins,'context':context})
dump(O/'INTEGRATION-FACTS.json',{'status':'SOURCE_ONLY_UNAPPLIED_UNRUN_UNREVIEWED_ON_NEW_TREE','base':B,'baseTree':base_tree,'prospectiveTree':expected,'prospectiveTreeMethod':'Pure in-memory Git tree SHA1 encoding from read-only git ls-tree; recomputed base tree exactly then replaced3blobIDs. No Git objects/index/commit/worktree writes.','patchSha256':sha(patch),'patchSameBytesAsReviewedF297Delta':True,'completeHunksReconstructedAgainstCurrentBase':True,'conflicts':[],'changedPaths':changed,'currentCoreHeaderDeltaPaths':git('diff','--name-only',OLD,B).decode().splitlines(),'newCandidateCommit':None,'newProductChecks':0,'providerCalls':0,'GitOrSharedWrites':0})
predicates={'falsifiablePredicate':'Verbatim f297 CLI guidance applies to main71 with all3current before bodies unchanged, while retaining actual current Core header diagnostic consumer and no source conflict or omitted linked consumer.','sourcePredicates':['Only help metadata, its existing schema unit test and profile walkthrough change; no production routing, output bytes, feature selection or semantics change.','Compile/simulate help names supported Core1/Core2; beta and Source6 remain separate inputs. Default Core1 expand and explicit Source6 transfer feature behavior remain unchanged.','Current shared check dispatch, Core1 parse, Core2 language probe, compile/simulate/core_io routes and Source6/Node direct contexts all remain available physically under candidate-context; previous40-body consumer set retained without omissions.','PR238 current parser::language error prefix names both profiles; code/offset/suffix expected by actual current regression remain untouched. Header acceptance unchanged. This is a source observation, not a new runtime observation.','Full3after bodies are exactly f297; applicable CheckReport and all previously selected direct consumer bodies match f297. Current parser/test changes are retained verbatim from main71.'],'expectationProvenance':'Historical source-derived doc/Clap expectations copied unchanged; no new independent financial oracle or updated expected runtime golden. Current header expectations are existing upstream test literals.','historicalApproval':'Root identifies f297 as exact reviewed/approved. Original packet and approvals remain immutable at their original identities. No approval or runtime result is transferred to prospective new tree.','preserved':'All U0–U7/M1–M17/MC/SP/G/148requirements/292scenarios/20families/financial/composition/native/target/Artifact/proof/ledger/security-owner limits remain. No security judgment supplied.'}
dump(O/'SOURCE-PREDICATES.json',predicates)
dump(O/'HISTORICAL-BOUNDARY.json',{'oldPacket':str(P),'oldFreezeSha256':sha((P/'FREEZE.sha256').read_bytes()),'oldSubjectSha256':sha((P/'SUBJECT.json').read_bytes()),'approvalSource':'Parent/root explicit task names exact f297 approval; no new provider vote fabricated/copied as current approval.','oldExactResult':'1Clap/7filtered+5bash-n at f297 remain historical; no checks executed here on main71 prospective tree.'})
(O/'README.md').write_text('Unapplied PR237 CLI guidance integration on main71\n\nBase '+B+' / tree '+base_tree+'. Prospective tree '+expected+' (pure in-memory Git tree calculation, not an actual commit). CANDIDATE.patch is the exact previously reviewed f297 3-path delta; every current before body equals the earlier8269 before body. All hunks reconstruct exact f297 after bodies. No conflicts or resolutions.\n\nFull before/after3files and45full candidate context bodies are physical. The previous40-body public-consumer context set is retained; parser.rs, its language regression, mori-core Cargo features and normative guidance are added. Main71 retains PR238 Core-header error changes; current check wrapper/helper, module dispatch, compile, simulate, default expand, explicit Source6 and Node contexts remain byteequal to f297. CURRENT-HEADER-DIAGNOSTIC.patch records the exact upstream difference.\n\nSOURCE-PREDICATES and SOURCE-PINS identify source expectations and exact blobs. ORDINARY-REGRESSION-ARGV contains existing selected Clap metadata/header regression and5shell syntax commands, all inspected and UNRUN on this prospective tree. No new semantic/test authoring or output goldens. Root alone integrates/checks/reviews. Existing exact f297 approval and actual results remain historical; no new-tree approval is claimed. No product/native/SDK/proof/wallet execution, provider calls or Git/shared writes occurred.\n')
# Final local-only source checks.
assert all(v['sameAsF297'] for v in context if v['path'] in [x['path'] for x in prior['files']])
for n in range(1,6):assert (O/'doc-shell'/f'{n:02}.sh').read_text().strip() in afters['docs/NATIVE-CORE.md'].decode()
dump(O/'STATIC-VALIDATION.json',{'result':'PASS_SOURCE_TRANSPORT_ONLY','complete3HunksReconstructed':True,'complete3AfterEqualsF297':True,'all40PriorConsumerBodiesRetainedAndEqualF297':True,'currentHeaderParserAndRegressionPreserved':True,'pureTreeHashReproducedBase':True,'fiveShellInputsPresentInCandidateGuide':True,'productChecksRun':0,'newApproval':False})
manifest={'format':'cli-guidance-main238-source-proposal/1','files':[{'path':str(p.relative_to(O)),'sha256':sha(p.read_bytes()),'bytes':p.stat().st_size} for p in sorted(O.rglob('*')) if p.is_file() and p.name not in ['MANIFEST.json','FREEZE.sha256']]};dump(O/'MANIFEST.json',manifest)
(O/'FREEZE.sha256').write_text(''.join(sha(p.read_bytes())+'  '+str(p.relative_to(O))+'\n' for p in sorted(O.rglob('*')) if p.is_file() and p.name!='FREEZE.sha256'))
print(json.dumps({'output':str(O),'baseTree':base_tree,'prospectiveTree':expected,'patchSha256':sha(patch),'manifestSha256':sha((O/'MANIFEST.json').read_bytes()),'freezeSha256':sha((O/'FREEZE.sha256').read_bytes()),'files':len(manifest['files'])},indent=2))
