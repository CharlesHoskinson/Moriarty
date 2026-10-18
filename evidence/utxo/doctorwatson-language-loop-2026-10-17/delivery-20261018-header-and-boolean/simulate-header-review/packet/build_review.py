from pathlib import Path
import json,hashlib,datetime,copy,re
from jsonschema import Draft202012Validator
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
O=R/'concurrency-increase-main238/simulate-header-review'
A=R/'concurrency-increase-main238/grammar-current5f7'
C=R/'simulate-header-dispatch-check-20261018'
B=Path('/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
original={str(p):sha(p)for root in [A,C]for p in root.rglob('*')if p.is_file()}
mandatory=[];units=[]
def put(p,value):
 q=O/p;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(value.encode()if isinstance(value,str)else value);return q
def cp(src,dst):return put(dst,src.read_bytes())
def jput(p,value):return put(p,json.dumps(value,ensure_ascii=False,indent=2)+'\n')
def req(p):mandatory.append(p)
def full(path):cp(B/path,'source/'+path);req('source/'+path)
def excerpt(root,path,start,end,label):
 p='units/'+label;lines=(root/path).read_text().splitlines(keepends=True)
 put(p,''.join(lines[start-1:end]));req(p)
 units.append({'packetPath':p,'origin':str(root/path),'originSha256':sha(root/path),'startLine':start,'endLine':end,'unitSha256':sha(O/p),'scope':'Complete named unit(s); not a full transitive unchanged module closure'})
def fn_unit(root,path,signature,label):
 lines=(root/path).read_text().splitlines();start=next(i for i,l in enumerate(lines)if l.startswith(signature));indent=len(signature)-len(signature.lstrip());end=next(j for j in range(start+1,len(lines))if lines[j]==' '*indent+'}')
 excerpt(root,path,start+1,end+1,label)
# Exact original custody, never rewritten. Author manifest lists154 other files; itself is155th.
for root,sub in [(A,'original-author'),(C,'actual')]:
 for p in sorted(root.rglob('*')):
  if p.is_file():cp(p,sub+'/'+str(p.relative_to(root)))
am=json.loads((A/'MANIFEST.json').read_text());assert len(am['entries'])==154
assert len([p for p in A.rglob('*')if p.is_file()])==156  # 155 frozen members plus FREEZE.sha256 receipt
assert all(sha(A/e['path'])==e['sha256']and(A/e['path']).stat().st_size==e['bytes']for e in am['entries'])
for freeze_name in ['FREEZE.sha256','EXPECTED-FREEZE.sha256']:
 rows=[line.split('  ',1) for line in (A/freeze_name).read_text().splitlines() if line]
 assert all(sha(A/name)==digest for digest,name in rows),freeze_name
binding=json.loads((C/'POST-COMMIT-BINDING.json').read_text());assert binding['head']=='615eceee9c6250d2f203cb5807d5c75482dbdb5d'
bef=json.loads((C/'SOURCE-PINS-BEFORE.json').read_text());aft=json.loads((C/'SOURCE-PINS-AFTER.json').read_text())
bm={p['path']:p['sha256']for p in bef['files']};fm={p['path']:p['sha256']for p in aft['files']};assert len(fm)==902 and set(bm)==set(fm)
assert all(sha(B/p)==h for p,h in fm.items())
changed=[{'path':p,'sha256':fm[p]}for p in binding['changedPaths']]
assert len(changed)==2
assert [p for p in bm if bm[p]!=fm[p]]==['crates/mori/src/commands/simulate.rs']
assert binding['sourcePinsAfterSha256']==sha(C/'SOURCE-PINS-AFTER.json')
for c in changed:
 assert (B/c['path']).read_bytes()==(A/'candidate'/c['path']).read_bytes();full(c['path'])
TOKEN='mori-simulate-header-615eceee-'+hashlib.sha256(b''.join((B/c['path']).read_bytes()for c in changed)).hexdigest()[:20]
phases=['red','green','core1-positive','core2-positive']
for p in sorted((O/'actual').rglob('*')):
 if p.is_file()and p.name not in ['SOURCE-PINS-BEFORE.json','SOURCE-PINS-AFTER.json','SIMULATE-AFTER.rs']:
  req(str(p.relative_to(O)))
for name in ['README.md','SOURCE-TRACE.md','EXPECTED-CASE-SPEC.json','EXPECTED-FREEZE.sha256','PRESERVED-27-CONSTRUCTOR-SCOPE.json','PRIOR-APPLICABILITY-DIAGNOSIS.json','RENDERER-SOURCE-PINS.json','FORMAT.json','OFFLINE-VALIDATION.json']:
 # Duplicate case specification/freeze is already read in original actual receipts.
 if name not in ['EXPECTED-CASE-SPEC.json','EXPECTED-FREEZE.sha256']:req('original-author/'+name)
for p in sorted((A/'preexecution-fixtures').iterdir()):req('original-author/preexecution-fixtures/'+p.name)
# Complete direct ordinary routing producer, rendering and JSON units. No backend closure is mandatory.
for path in ['crates/mori/src/main.rs','crates/mori/src/commands/mod.rs','crates/mori/src/shell.rs','crates/mori/src/error.rs','crates/mori/src/display.rs','crates/mori-core/src/parser.rs','crates/mori-core/src/json.rs','crates/mori-lexer/src/lib.rs','crates/mori-lexer/src/token.rs','crates/mori-lexer/src/string.rs','crates/mori-lexer/src/scan.rs','crates/mori/Cargo.toml','crates/mori-core/Cargo.toml','crates/mori-core/src/stage/report.rs','rust-toolchain.toml']:
 full(path)
for signature,label in [('pub fn read_source','core_io-read_source.rs'),('pub fn operation','core_io-operation.rs'),('pub fn print','core_io-print.rs'),('pub fn read_json','core_io-read_json.rs')]:fn_unit(B,'crates/mori/src/commands/core_io.rs',signature,label)
excerpt(B,'crates/mori-core/src/ir.rs',1,9,'core-profile-constants.rs');excerpt(B,'crates/mori-core/src/ir.rs',466,483,'core-Error-dto.rs')
excerpt(B,'crates/mori-core/tests/language.rs',428,475,'existing-library-header-test.rs')
excerpt(B,'crates/mori-core/src/stage/mod.rs',27,42,'stage-labels-and-profiles.rs');excerpt(B,'crates/mori-core/src/stage/mod.rs',44,71,'staged-program-accessors.rs');excerpt(B,'crates/mori-core/src/stage/mod.rs',73,92,'stage-load.rs')
excerpt(B,'crates/mori/src/cli.rs',37,63,'cli-global-options.rs');excerpt(B,'crates/mori/src/cli.rs',99,114,'cli-color-conversion.rs')
render='crates/mori-diagnostics/src/render.rs';lines=(B/render).read_text().splitlines();excerpt(B,render,1,lines.index('#[cfg(test)]'),'mori-diagnostics-render-production.rs')
# Keep complete positive-test units and unchanged full oracles in physical custody. Mandatory oracle projection contains every value actually accessed by the selected positive test.
for path in ['crates/mori/tests/logical_meter.rs','crates/mori/tests/core_continuations.rs','examples/core/continuations/normal-oracle.json','examples/core/continuations/delivery.mori','examples/core/continuations/reserve.mori','examples/core/continuations/atomic-compatibility.mori']:
 cp(B/path,'positive-custody/'+path)
for path,target,label in [('crates/mori/tests/logical_meter.rs','fn existing_simulate_bytes_and_execution_stay_unchanged_with_optional_cap','positive-core1-complete-helpers-and-test.rs'),('crates/mori/tests/core_continuations.rs','fn delivery_false_observation_preserves_boundary_then_completes_at_eleven','positive-core2-complete-helpers-and-test.rs')]:
 lines=(B/path).read_text().splitlines();i=next(i for i,l in enumerate(lines)if l.startswith(target));z=next(j for j in range(i+1,len(lines))if lines[j]=='}');excerpt(B,path,1,z+1,label)
full('examples/core/continuations/delivery.mori')
oracle_path='examples/core/continuations/normal-oracle.json';oracle=json.loads((B/oracle_path).read_text())
# These are complete parsed JSON fields, not regenerated expectations. Keep provenance alongside all directly accessed values.
used={k:v for k,v in oracle.items()if k not in ['reserve','compatibility_v1']}
jput('positive-oracle-used-fields.json',used);req('positive-oracle-used-fields.json')
jput('POSITIVE-ORACLE-SCOPE.json',{'fullOraclePath':'positive-custody/'+oracle_path,'originalSha256':sha(B/oracle_path),'originalBytes':(B/oracle_path).stat().st_size,'mandatoryProjection':'positive-oracle-used-fields.json','selection':'Every original top-level field except unused reserve and compatibility_v1; complete escrow object and normal_output_labels_from_opus are unchanged. Includes all oracle provenance/uncertainty fields. No semantic field read by the selected Core2 test is omitted. Full original oracle physically preserved byte-identically.','core1Oracle':'Existing test compares actual public bytes with shared library simulate+json exporter, and metered execution equality/logical16. This is an unchanged compatibility control, not an independent financial or compiler oracle.','core2Oracle':'Existing frozen literal escrow and normal labels; selected assertions compare fields written in helpers (segment statement fields beyond selected keys and other report members are not inferred tested). No expansion of its evidence scope.'});req('POSITIVE-ORACLE-SCOPE.json')
# Complete miette methods actually implicated by no-source/no-label/no-help Operation diagnostics.
renderer=A/'renderer-source';g='miette-7.6.0/src/handlers/graphical.rs'
excerpt(renderer,g,1,49,'miette-graphical-handler-types.rs')
for sig,label in [('    pub fn new_themed','miette-new-themed.rs'),('    pub fn with_links','miette-with-links.rs'),('    pub fn with_width','miette-with-width.rs'),('    pub fn render_report','miette-render-report.rs'),('    fn render_report_inner','miette-render-report-inner.rs'),('    fn render_header','miette-render-header.rs'),('    fn render_causes','miette-render-causes.rs'),('    fn render_footer','miette-render-footer.rs'),('    fn render_related','miette-render-related.rs'),('    fn render_snippets','miette-render-snippets.rs'),('    fn wrap(','miette-wrap.rs')]:fn_unit(renderer,g,sig,label)
# Relevant pinned theme/default wrapping sources, complete retained bodies. Renderer source pins are unchanged.
for path in ['miette-7.6.0/src/handlers/theme.rs','textwrap-0.16.4/src/options.rs','textwrap-0.16.4/src/word_separators.rs','textwrap-0.16.4/src/wrap_algorithms.rs']:
 q=cp(renderer/path,'renderer/'+path);req(str(q.relative_to(O)))
# Exact governing clauses, without unrelated later numeric or target design scope.
spec='openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md';lines=(B/spec).read_text().splitlines();a=lines.index('### Requirement: UNI-018 Profile and consumer admission')+1;z=next(i for i in range(a,len(lines))if lines[i].startswith('**Normal accepted report clauses'));excerpt(B,spec,a,z,'UNI-018.md')
guid='RUST_GUIDELINES.md';lines=(B/guid).read_text().splitlines();excerpt(B,guid,lines.index('## Diagnostics')+1,lines.index('## Later'),'RUST-diagnostics-testing-cli.md')
excerpt(B,'docs/MORIARTY-PRODUCT-CONTRACT.md',1,17,'permissionless-product-boundary.md')
doc='docs/CORE-STAGED-TESTS.md';lines=(B/doc).read_text().splitlines();a=next(i for i,l in enumerate(lines)if 'normally complete empty action' in l or 'normally completed empty action' in l);excerpt(B,doc,a+1,a+1,'empty-core2-contract.md')
# Mechanical checks of immutable captures and complete frozen values. No Moriarty import or execution.
frozen=json.loads((C/'EXPECTED-CASE-SPEC.json').read_text());comparisons=[]
for phase in ['red','green']:
 for case in frozen['header_cases']:
  p=C/phase/'public-cli'/case['name'];assert(p/'header.mori').read_bytes()==case['source'].encode();assert json.loads((p/'scenario.json').read_text())==frozen['empty_core2_control']['scenario'];process=json.loads((p/'process.json').read_text());error=json.loads((p/'producer-error.json').read_text())
  assert error==case['producer_error_json'];assert process['status']==case['public_simulate_exit'];assert(p/'stdout.bin').read_bytes()==b''
  matched=(p/'stderr.txt').read_bytes()==case['public_simulate_stderr'].encode();assert matched==(phase=='green')
  comparisons.append({'phase':phase,'case':case['name'],'producerErrorExact':True,'exit':process['status'],'stdoutEmpty':True,'stderrMatchesFrozenRepair':matched,'stderrSha256':sha(p/'stderr.txt')})
 p=C/phase/'public-cli/empty-core2';assert(p/'header.mori').read_bytes()==frozen['empty_core2_control']['source'].encode();assert json.loads((p/'scenario.json').read_text())==frozen['empty_core2_control']['scenario'];report=json.loads((p/'stdout.bin').read_bytes());assert report['segment']['execution']==frozen['empty_core2_control']['execution'];assert report['status']=='complete';assert(p/'stderr.txt').read_bytes()==b'';assert json.loads((p/'process.json').read_text())['status']==0
 comparisons.append({'phase':phase,'case':'empty-core2','frozenCompleteExecutionProjectionExact':True,'statusComplete':True,'emptyStderrAndExit0':True,'otherFullReportFieldsCapturedButNotAsserted':True,'aggregateRustEmptyAssertionsReached':phase=='green'})
for phase in phases:
 res=json.loads((C/phase/'RESULT.json').read_text());assert res['stdoutSha256']==sha(C/phase/'STDOUT.log');assert res['stderrSha256']==sha(C/phase/'STDERR.log')
verification={'format':'simulate-header-review-exact-custody/1','checkedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'reportedCommit':binding['head'],'reportedTree':binding['tree'],'reportedParent':binding['parent'],'rootGitBinding':'actual/POST-COMMIT-BINDING.json','rootGitVsBuilder':'Root receipt reports exact2paths, clean tree and902afterpins matching committed Git/worktree. Builder performs no Git operations; independently compares all902 recorded afterpins with current file bytes.','beforeAndAfterSourcePinsCount':902,'all902AfterPinsMatchWorktree':True,'soleBetweenPhaseProductionMutation':[{'path':p,'beforeSha256':bm[p],'afterSha256':fm[p]}for p in bm if bm[p]!=fm[p]],'newFrozenTestPresentInBothPhaseManifests':True,'changedPathHashes':changed,'allActualSourceAndTestBytesMatchAuthorCandidate':True,'authorFrozenManifestEntries':154,'authorFrozenMembersIncludingManifest':155,'authorPhysicalFilesIncludingFreezeReceipt':156,'allAuthorEntryHashesAndSizesMatch':True,'actualLogHashesAllFourPhasesMatch':True,'capturedObservationComparisons':comparisons,'sourceManifestDigest':sha(C/'SOURCE-PINS-AFTER.json'),'fullManifestModelReadRequirement':False,'scope':'All semantic captures, assertions, relevant producer/consumer/miette units and exact custody summary mandatory. Complete902-row manifests and original155member author packet physically preserved; unrelated hash rows and unchanged native/backend closures are not mandatory model reading. No whole-repository or full-manifest model-read claim.'}
jput('SOURCE-RESULT-CUSTODY.json',verification);req('SOURCE-RESULT-CUSTODY.json')
jput('SOURCE-UNIT-MAP.json',units);req('SOURCE-UNIT-MAP.json')
jput('CANDIDATE.json',{'head':binding['head'],'tree':binding['tree'],'parent':binding['parent'],'changedPaths':changed,'beforeFullSource':'actual/SIMULATE-BEFORE.rs','newTestBefore':'absent in parent; present during RED and GREEN','reviewToken':TOKEN});req('CANDIDATE.json')
request=f'''Review the exact ordinary simulate diagnostic repair and its actual results as ONE whole independent opinion with separate sourceAssessment and actualResultAssessment. Do not force approval; approve, request_changes or abstain are available. Direct reviewToken: {TOKEN}. Read the entire FULL-INPUT once, including every mandatory source/result body, before judging. Report genuine read limits and dissent. Do not fabricate reads or quotes. No source/provider/product/Git execution, private captures, native/SDK/ledger/wallet action or security judgment is requested. Original148/292/20 and allU0-U7/M1-M17/MC/SP/G, complete language/UTXO/financial/composition/proof/target exits remain unchanged.

Exact candidate commit{binding['head']}/tree{binding['tree']}, parent{binding['parent']}, exactly2paths in CANDIDATE.json. Both complete bodies and fullbefore simulate are mandatory. The production edit replaces discarded language() Err with existing map_err(core_io::operation) before normal source-profile dispatch. Existing Core2 success routing, Core1 parse/simulate, metering, trace/target and resume branches retain their code. No ad-hoc prefix detector, supported-profile migration or new diagnostic format is proposed. Inspect whole bodies and actual producer/CLI/miette path; cite real source lines and exact changed hashes. UNI-018 supports explicit profile reporting; the existing library/header test, not UNI-018 alone, fixes dual-profile wording.

Source-exposed freeze: original-author155frozenmembers plus FREEZE.sha256receipt are preserved unchanged; its manifest lists154otherfiles. The prior27constructor scope and gaps remain OPEN, not repaired or closed here. Full original author source/oracles/renderer files remain custody. Mandatory review covers complete relevant units with original absolute path/full-sourceSHA/line/unitSHA in SOURCE-UNIT-MAP. It does not require reading all902unrelated source-pin rows or unrelated native/backend modules. SOURCE-RESULT-CUSTODY and original POST-COMMIT-BINDING are mandatory exact summaries; do not claim full-repository or full-manifest model reads. No older header packet was modified.

Read all four actual phases' full argv/START/CHILD/RESULT/stdout/stderr plus complete rootdriver/CHECKPLAN/final/mutation/postcommit/root frozen comparison, all8rawpublicprocesscaptures (red/green x unknown/missing/commented/empty-core2) and every captured input/process/stdout/stderr/producerError field. Empty stdout files are real mandatory0byte inputs, not omitted reports. Red aggregate:1FAILED exit101; all4publicprocesses were captured BEFORE first humanstderrassertion. Rust loop stops on first unknown humanstreammismatch, so later Rust case assertions and emptycontrol assertions are not reached onRED. The separate root FROZEN-OBSERVATION-COMPARISON and builder mechanical comparison check every raw producerErrorJSON/offset16/9/25, process exit/empty stdout and emptyCore2completeExecution projection onBOTHphases. Distinguish this separate raw comparison from Rust assertion reachability. GREENaggregate:1PASS exit0; all3producer/error streams plus emptyCore2control assertions run. Report human stream has no offsetfield; offsets are complete producerErrorDTO assertions/captures. Publicsimulate exposes the existing humanreport, not a newJSONerrorflag.

EmptyCore2 rawreport is fully captured, but only statuscomplete, exit0, emptystderr and complete segment.execution (semantics/action/pre/post/steps/effects) are asserted. Work/hash/continuation/qualification/other wrapper fields remain unasserted; do not infer whole-report agreement. The freeze was derived with source exposure, not a blind independentfinancial/work oracle. Do not derive new goldens from outputs.

Positive controls: selected existingCore1logicaltestPASS and Core2deliverycontinuationtestPASS. Mandatory complete helper+selectedtestunits and all values they access are preserved unchanged. Core1 uses shared librarysimulate/export byte compatibility, metered execution equality and logical16; it is NOT independent financial/compiler proof. Core2 reads original normal-oracle escrow and normal_output_labels; every used value and provenance/uncertainty field is mandatory in positive-oracle-used-fields.json. Full original124820B oracle is byte-identical in physicalpositivecustody; only unused reserve/compatibility_v1 top-level values are outside mandatory reading. No used semantic field is omitted. Segment helpers assert only their written field keys; no extra report equality is inferred. Full positive source files/oracles are physically preserved. Their original external fixture paths are existing guidelines deviations, not permission to add one.

Real typed Draft2020-12 schema is REVIEW-RESPONSE.schema.json (separate file, not a filled response exemplar). Return one whole JSON opinion with directtoken, bothassessments, exact2path/fullread/hash/realquoteanchors,4phase/assertion/logcoverage,8rawcapturecoverage, rootrawcomparisonvsREDassertionreachability and unassertedwrapper dispostions, mandatoryinputcoverage, actualreadlimits,dissent,findings,limitations. A complete schema shape does not prove real reads/quotes/reasoning. If unable to finish mandatory reading, name exact missing paths, abstain for affected assessment and return both objects honestly. No provider timeout or packet author report is approval. All product/source/result acceptance layers remain separate.
'''
put('REVIEW-REQUEST.md',request);req('REVIEW-REQUEST.md')
mandatory=list(dict.fromkeys(mandatory));paths=mandatory
inventory=[{'path':p,'bytes':(O/p).stat().st_size,'sha256':sha(O/p)}for p in mandatory]
jput('MANDATORY-INPUTS.json',{'reviewToken':TOKEN,'files':inventory,'custodyOnly':['actual/SOURCE-PINS-BEFORE.json','actual/SOURCE-PINS-AFTER.json','original-author/MANIFEST.json','positive-custody/examples/core/continuations/normal-oracle.json'],'scope':'Exact2path source+finite actualresult. Original155author and902sourcepin rows preserved but unrelated unchanged entries not mandatory reads.'})
parts=[request,'\nAll mandatory original file bodies follow once; no omitted semantic assertion/report field.\n']
for item in inventory:
 if item['path']=='REVIEW-REQUEST.md':continue
 parts.append('\nBEGIN FILE '+item['path']+' | SHA256 '+item['sha256']+' | BYTES '+str(item['bytes'])+'\n'+(O/item['path']).read_text()+'\nEND FILE '+item['path']+'\n')
put('FULL-INPUT.md','\n'.join(parts))
# Real typed schema logic copied as source text from the previously validated generator, never imported or executed at its old paths.
prior=(R/'concurrency-increase-main238/public-header-dispatch-runtime-schema/generate_schema.py').read_text();a=prior.index("S={'type':'string'");z=prior.index('Draft202012Validator.check_schema(schema)')
code=prior[a:z].replace('moriarty:public-header-dispatch:5f7c2a28','moriarty:simulate-header-dispatch:615eceee').replace('public Core header source','normal simulate header source').replace('receiptNamingIssueDisposition','observationComparisonDisposition')
exec(compile(code,str(O/'build_review.py')+':schema-source','exec'))
# Additional typed capture coverage, without pretending the schema proves actual reads.
capVariants=[]
for phase in ['red','green']:
 for case in ['unknown','missing','commented','empty-core2']:
  capVariants.append(obj({'phase':{'type':'string','const':phase},'case':{'type':'string','const':case},'completeInputsProcessAndRawStreamsRead':BOOL,'producerErrorDtoRead':{'type':['boolean','null'],'const':None}if case=='empty-core2'else BOOL,'scopeAssessment':S}))
cap=arr({'oneOf':capVariants},minItems=8,maxItems=8,allOf=[{'contains':{'properties':{'phase':{'const':phase},'case':{'const':case}},'required':['phase','case']},'minContains':1,'maxContains':1}for phase in ['red','green']for case in ['unknown','missing','commented','empty-core2']])
actual=schema['properties']['actualResultAssessment'];actual['properties'].update({'capturedProcessCoverage':cap,'redAssertionReachabilityDisposition':S,'unassertedEmptyReportFieldsDisposition':S,'positiveOracleIndependenceDisposition':S});actual['required']+=['capturedProcessCoverage','redAssertionReachabilityDisposition','unassertedEmptyReportFieldsDisposition','positiveOracleIndependenceDisposition']
actual['allOf'].append({'if':{'properties':{'capturedProcessCoverage':{'contains':equals('completeInputsProcessAndRawStreamsRead',False)}}},'then':{'properties':{'verdict':{'const':'abstain'}}}})
incompleteCapture={'properties':{'capturedProcessCoverage':{'contains':{'anyOf':[equals('completeInputsProcessAndRawStreamsRead',False),equals('producerErrorDtoRead',False)]}}}}
actual['allOf'].append({'if':incompleteCapture,'then':{'properties':{'verdict':{'const':'abstain'}}}})
schema['allOf'].append({'if':{'properties':{'actualResultAssessment':incompleteCapture}},'then':{'properties':{'verdict':{'const':'abstain'},'mandatoryInputCoverage':{'properties':{'entireInlineInputRead':{'const':False}}},'actualReadLimits':{'minItems':1}}}})
Draft202012Validator.check_schema(schema);jput('REVIEW-RESPONSE.schema.json',schema)
launcher=f'''Return ONE whole independent ordinary source+actual-result review. Read the COMPLETE original packet once at {O/'FULL-INPUT.md'} ({(O/'FULL-INPUT.md').stat().st_size}bytes,SHA256{sha(O/'FULL-INPUT.md')}). This short launcher does NOT replace or narrow any mandatory body. The packet contains every full candidate body, before source, relevant complete producer/consumer/miette units, frozen used positive oracles, allsemanticrawcaptures/logs/driver/receipts and mandatory limits/dissent. Read all mandatory bodies, avoiding repeated reads. If tools truncate, obtain missing parts before asserting fullread. If unable, identify exact missing paths and abstain for affected assessment while returning both whole assessment objects. Do not fabricate reads/quotes/results.

Use actual typed JSON Schema {O/'REVIEW-RESPONSE.schema.json'}; direct top-level reviewToken={TOKEN}. Return JSON only, not a response exemplar. Both sourceAssessment and actualResultAssessment, exact2path/fullread/hash/realquote coverage, all4phase loganchors, all8capturedprocess coverage, observationcomparison/REDassertionreachability/unassertedemptyreport/positive-oracle limits, mandatoryinputcoverage, actualreadlimits,dissent,findings,limitations are required. approve/request_changes/abstain allavailable; no forced approval. Schema cannot establish genuine reads, realquotes or substantive reasoning.

The902-row fullsourcepinmanifests and155-member originalauthorpacket are physicalcustody. Mandatory review reads exactcustodysummaries and originalpostcommitbinding instead of hundreds of unrelated unchanged rows. No fullrepo/fullmanifest modelread claim. Preserve all27constructorobligations and broader exits OPEN. Read fullsource/result requirements in REVIEW-REQUEST within FULL-INPUT, including REDall4processescapturedbeforefirststderrassertionfailure, separate rootrawmechanicalcomparisons, GREENaggregate1PASS, twoexistingpositivecontrols, allcompleteErrorDTOoffset16/9/25 vs humanrendernooffset, emptyexecutionprojectiononly/otherwrapperfieldsunasserted, author-source-exposedoracles and existingpositivecontrols' writtenassertionlimits. Nousedsemanticfield is omitted. No provider/product/Git/privatecapture/SDK/native/ledger/wallet action or securityjudgment. No new execution or admission. Return one complete whole opinion with honest actual limits and dissent.
'''
assert len(launcher.encode())<=125000;put('READ-ONCE-PROMPT.md',launcher)
# Schema validation only; specimens are never written or called opinions.
coverage=[{'path':c['path'],'sha256':c['sha256'],'completeBodyRead':True,'quoteAnchors':[{'originalLine':1,'exactQuote':'SCHEMA-ONLY','analysis':'SCHEMA-ONLY'}]}for c in changed]
phaseCoverage=[{'phase':p,'sourceAndAssertionsRead':True,'completeStdoutAndStderrRead':True,'quoteAnchors':[{'packetPath':f'actual/{p}/STDOUT.log','exactQuote':'SCHEMA-ONLY','analysis':'SCHEMA-ONLY'}]}for p in phases]
captures=[{'phase':p,'case':c,'completeInputsProcessAndRawStreamsRead':True,'producerErrorDtoRead':None if c=='empty-core2'else True,'scopeAssessment':'SCHEMA-ONLY'}for p in ['red','green']for c in ['unknown','missing','commented','empty-core2']]
x={'reviewToken':TOKEN,'verdict':'request_changes','sourceAssessment':{'verdict':'request_changes','reason':'SCHEMA-ONLY','changedPathCoverage':coverage},'actualResultAssessment':{'verdict':'request_changes','reason':'SCHEMA-ONLY','phaseCoverage':phaseCoverage,'observationComparisonDisposition':'SCHEMA-ONLY','capturedProcessCoverage':captures,'redAssertionReachabilityDisposition':'SCHEMA-ONLY','unassertedEmptyReportFieldsDisposition':'SCHEMA-ONLY','positiveOracleIndependenceDisposition':'SCHEMA-ONLY'},'mandatoryInputCoverage':{'entireInlineInputRead':True,'filesReadCompletely':paths,'unreadMandatoryFiles':[],'coverageSummary':'SCHEMA-ONLY'},'actualReadLimits':[],'dissent':[],'findings':[],'limitations':['SCHEMA-ONLY; no review']}
v=Draft202012Validator(schema);v.validate(x)
yes=copy.deepcopy(x);yes['verdict']='approve';yes['sourceAssessment']['verdict']='approve';yes['actualResultAssessment']['verdict']='approve';v.validate(yes)
limited=copy.deepcopy(x);limited['verdict']='abstain';limited['actualResultAssessment']['verdict']='abstain';limited['actualResultAssessment']['capturedProcessCoverage'][0]['completeInputsProcessAndRawStreamsRead']=False;limited['mandatoryInputCoverage']['entireInlineInputRead']=False;limited['mandatoryInputCoverage']['filesReadCompletely']=paths[:-1];limited['mandatoryInputCoverage']['unreadMandatoryFiles']=paths[-1:];limited['actualReadLimits']=[{'scope':'SCHEMA-ONLY','unreadInputPaths':paths[-1:],'reason':'SCHEMA-ONLY','affectedAssessments':['actualResultAssessment']}];v.validate(limited)
neg=[]
def bad(name,mutate):
 y=copy.deepcopy(x);mutate(y);assert not v.is_valid(y),name;neg.append(name)
bad('missing direct token',lambda y:y.pop('reviewToken'));bad('missing actual assessment',lambda y:y.pop('actualResultAssessment'));bad('missing source assessment',lambda y:y.pop('sourceAssessment'));bad('wrong changedhash',lambda y:y['sourceAssessment']['changedPathCoverage'][0].update(sha256='0'*64));bad('missing phase',lambda y:y['actualResultAssessment']['phaseCoverage'].pop());bad('missing rawcapture',lambda y:y['actualResultAssessment']['capturedProcessCoverage'].pop());bad('missing assertionreachability disposition',lambda y:y['actualResultAssessment'].pop('redAssertionReachabilityDisposition'));bad('missing unasserted wrapper limitation',lambda y:y['actualResultAssessment'].pop('unassertedEmptyReportFieldsDisposition'));bad('missing dissent',lambda y:y.pop('dissent'));bad('missing mandatory input',lambda y:y['mandatoryInputCoverage']['filesReadCompletely'].pop());bad('overall approval without whole approvals',lambda y:y.update(verdict='approve'))
for p,d in original.items():assert sha(Path(p))==d,p
physical=[{'path':str(p.relative_to(O)),'bytes':p.stat().st_size,'sha256':sha(p)}for p in sorted(O.rglob('*'))if p.is_file()and p.name not in ['PACKET-MANIFEST.json','VALIDATION.json']]
jput('PACKET-MANIFEST.json',{'reviewToken':TOKEN,'fullInputBytes':(O/'FULL-INPUT.md').stat().st_size,'fullInputSha256':sha(O/'FULL-INPUT.md'),'schemaSha256':sha(O/'REVIEW-RESPONSE.schema.json'),'promptBytes':len(launcher.encode()),'mandatoryCount':len(mandatory),'files':physical})
jput('VALIDATION.json',{'format':'simulate-header-source-result-packet-validation/1','reviewToken':TOKEN,'allOriginalAuthorAndActualFilesUnchanged':True,'allAuthor154EntryDigestsAndSizesMatch':True,'authorFrozenMembers':155,'authorPhysicalFilesIncludingFreezeReceipt':156,'all902ActualAfterPinsMatchCurrentWorktree':True,'rawBeforeAfterFrozenComparison':'All8completeprocesscaptureinputs/exits/stdout/stderr and6producerErrorDTOs checked; bothcompleteemptyExecution projections exact. Otherrawreportfields preserved/unasserted.','allFourOriginalLogDigestPairsMatch':True,'schemaMetaValidation':'Draft202012Validator.check_schema PASS','schemaOnlySpecimens':'approve, request_changes and honest partial-capture abstention accepted; not actual reviewer opinions','negativeSchemaCasesRejected':neg,'fullInputBytes':(O/'FULL-INPUT.md').stat().st_size,'fullInputSha256':sha(O/'FULL-INPUT.md'),'promptBytes':len(launcher.encode()),'scope':'No provider/product/Git execution. RootGitbinding preserved; no independentGitprovenance claim. Physical original155author and902manifests preserved, unrelatedrows notmandatory modelreading. Complete selectedpositiveoracle fields mandatory; fulloriginaloracle unchanged custody. No originalheaderpacket writes.'})
print(json.dumps({'reviewToken':TOKEN,'fullInputBytes':(O/'FULL-INPUT.md').stat().st_size,'mandatory':len(mandatory),'promptBytes':len(launcher.encode()),'schemaBytes':(O/'REVIEW-RESPONSE.schema.json').stat().st_size},indent=2))
