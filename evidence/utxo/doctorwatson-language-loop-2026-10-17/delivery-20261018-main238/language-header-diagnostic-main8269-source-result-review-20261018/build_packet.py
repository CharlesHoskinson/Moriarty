from pathlib import Path
import json, hashlib, shutil, datetime
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
D=R/'language-header-diagnostic-main8269-source-result-review-20261018'; P=D/'packet'
O=R/'language-header-diagnostic-source-result-review-20261018'/'packet'
C=R/'language-header-diagnostic-main8269-check-cycle-20261018'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p,obj):
 p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(obj,indent=2)+'\n')
s=json.loads((D/'SOURCE-PREPARED.json').read_text());base=s['base'];head=s['head'];tree=s['tree']
assert not (P/'MANIFEST.json').exists()
shutil.copytree(C,P/'actual-check')
shutil.copy2(R/'run_language_header_diagnostic_check_20261018_main8269.py',P/'run_actual.py')
shutil.copytree(R/'language-header-diagnostic-main8269-source-integration-20261018',P/'source-integration')
final=json.loads((P/'actual-check/FINAL-RESULT.json').read_text());plan=json.loads((P/'actual-check/CHECK-PLAN.json').read_text());result=json.loads((P/'actual-check/language-header-test/RESULT.json').read_text());pins=json.loads((P/'actual-check/SOURCE-PINS.json').read_text());pinmap={p['path']:p['sha256']for p in pins['files']}
assert final['head']==plan['head']==pins['head']==head
assert final['tree']==plan['tree']==pins['tree']==tree and plan['base']==base
assert result['exitCode']==0 and result['ownedSessionRemaining']==[] and final['existsAfter']==False
for name in ['stdout','stderr']:
 assert sha(P/f'actual-check/language-header-test/{name.upper()}.log')==result[name+'Sha256']
assert sha(P/'original-author/MANIFEST.json')==plan['sourceManifestSha256']
integ=json.loads((P/'source-integration/SOURCE-INTEGRATION.json').read_text())
assert sha(P/'run_actual.py')==integ['driverSha256']
assert sha(P/'CANDIDATE.patch')==integ['candidateDiffSha256']=='8931f6b177d0b1515b9e587f2bd45e483636689294dad0b47647fafc438089e3'
for f in s['wholeSourceFiles']:
 assert sha(P/f['packetPath'])==f['sha256']
 if f['commit']==head:assert f['sha256']==pinmap[f['repoPath']]
oldcust=json.loads((O/'SOURCE-CUSTODY.json').read_text());ranges=[]
for r in oldcust['ranges']:
 content=''.join((P/r['wholeFile']).read_text().splitlines(keepends=True)[r['lineStart']-1:r['lineEnd']]);q=P/r['path'];q.parent.mkdir(exist_ok=True);q.write_text(content)
 assert sha(q)==r['sha256'];r={**r,'commit':head};ranges.append(r)
cust={**s,'ranges':ranges,'actualRecordedSourcePinCount':len(pins['files']),'originalAuthorPatchSha256':sha(P/'original-author/CANDIDATE.patch'),'finalCandidatePatchSha256':sha(P/'CANDIDATE.patch'),'formattingDifferenceSha256':sha(P/'ROOT-FORMATTING.diff'),'actualResultStatus':'root terminal exit 0 explicitly authorized for intake; current merged source result','runtimeScope':'Read-only terminal receipt intake and packet construction; no subject execution.'}
write(P/'SOURCE-CUSTODY.json',cust)
q=json.loads((O/'EVIDENCE-QUOTES.json').read_text())['quotes'];q=[x for x in q if x['path']!='source-context/crates/mori/src/commands/check.rs']
for path,needle in [('source-context/crates/mori/src/commands/check_source.rs','        let staged = mori_core::language(source)'),('source-context/crates/mori/src/commands/check.rs','    let outcome = check_source::dispatch(')]:
 lines=(P/path).read_text().splitlines();i=next(i for i,x in enumerate(lines) if x.startswith(needle));end=i+6 if 'check_source.rs' in path else i+10
 q.append({'path':path,'lineStart':i+1,'lineEnd':end,'literal':'\n'.join(lines[i:end])})
for x in q:assert '\n'.join((P/x['path']).read_text().splitlines()[x['lineStart']-1:x['lineEnd']])==x['literal']
write(P/'EVIDENCE-QUOTES.json',{'quotes':q})
scope=f'''# Ordinary exact-source and actual-result review

Review ONLY the public Core language-header diagnostic increment. No code authoring, runtime commands, builds, network/provider calls, delegation, private-file reads or security judgments. Read the complete inline candidate and actual receipts; relevant whole public source files are physically supplied. Exact range headings are excerpts, not whole-file inspection claims.

Exact immutable base {base}; candidate commit {head}; whole tree {tree}. Parents are 1cb98865f37863152cdf51a38773ca4548adea12 and {base}. Diff versus this base contains exactly parser.rs and tests/language.rs. Both changed base/candidate source bodies and the whole two-file patch are byte-identical to the older 79de/1cb diagnostic increment. This is a NEW composition and NEW actual result, with current public caller context. No old approval/result transfers. Root integration metadata is supplied; source body and receipt hashes independently bind this packet. No approval applies to a hypothetical future base/head/tree or changed consumed input.

SOURCE AXIS: exported mori_core::language() reads a Core header and returns moriarty/1 or moriarty/2. Its fallback previously named only moriarty/1. The candidate names both profiles on unsupported headers and adds both-profile context to missing keyword or quoted-profile errors. Verify actual Error code/offset preservation, unchanged lexer path, success matches, parse dispatch and accepted grammar. Only this function's diagnostic construction changes. Profile-specific parse_profile() still admits one declared profile at a time, with its unchanged corresponding message.

CURRENT PUBLIC CONSUMER LIMIT: check.rs now delegates to check_source::dispatch, whose language(source).is_ok_and(Core2) chooses staged loading or Core1 parsing. The complete CURRENT check.rs and check_source.rs are inline, and current commands/mod.rs is physically supplied. compile/inspect also use is_ok_and and simulate compares Ok(Core2). These routes discard language() errors. This test calls the public library API directly; it does not establish changed CLI error output. Unsupported CLI profiles still pass to the separate parse_profile() diagnostic. Do not imply this increment modifies shared-checker, MCP/LSP, help, compiler/target or feature semantics, or that unsupported source reaches Core2.

RESULT AXIS: root's new exact-source driver ran ONE selected public-library integration test at {head}/{tree}, with native features disabled, 90-second timeout and 5-second kill grace, frozen offline dependencies, and a fresh owned temporary root. Actual terminal exit 0; {result['wallSeconds']:.9f} seconds; 1 passed and 18 filtered. Two supported and four negative case iterations occur inside that one test, not six separately run tests. Assertions compare literal profile/message/code/offset values. Complete new stdout/stderr/start/child/result, plan, source pins and driver are supplied. The driver checks clean source and pins before and after; FINAL-RESULT records owned root removed and session empty. Root explicitly confirmed terminal collection before this packet intake. This is captured evidence, not a rerun by the packet author. Do not invent argv, temporary roots, assertions, report fields or broader checks.

FORMATTING HISTORY: historical-formatting/FORMATTING.json is the preserved ROOT-WRITTEN record from the original integration, not raw formatter stdout/stderr or a new formatting run. It records first author-candidate rustfmt --check exit 1 (root tool 20305a, two tuple layouts only), then formatted check exit 0 (root tool a839d3). No separate raw formatting logs exist. ROOT-FORMATTING.diff shows the exact byte difference, which remains identical for this merged candidate. Preserve this initial failure context. Original author UNRUN/static metadata is historical source preparation. Historical 1cb formatting and the NEW 3bb selected-test receipts are separate evidence families; the older selected-test logs are not offered as current evidence.

BOUNDARIES: no native, SDK relation, circuit, proof/key/wallet/ledger/target or financial execution is established. Ordinary dependency compilation is not cryptographic correctness testing. No full grammar/parser/CLI/property suite or the 18 filtered tests are established. All original U/M/MC/SP/G, finance/composition and target acceptance obligations remain. Do not supply security judgment, resource waiver, provider identity attestation or hypothetical future-base approval. This packet grants no approval.

Deliver exactly one whole structured review with separate sourceFindings and actualResultFindings, accurate filesRead/range coverage, literal evidence, preserved limitations/dissent, and the exact base/head/tree guard. State missing source or contradiction instead of guessing. Scope any approval to this ordinary increment and the new captured result only.
'''
(P/'REVIEW-SCOPE.md').write_text(scope)
whole=['REVIEW-SCOPE.md','CANDIDATE.patch','ROOT-FORMATTING.diff','candidate/crates/mori-core/src/parser.rs','candidate/crates/mori-core/tests/language.rs','original-author/EXPECTED-DIAGNOSTICS.json','original-author/CANDIDATE.patch','source-integration/SOURCE-INTEGRATION.json','actual-check/CHECK-PLAN.json','actual-check/FINAL-RESULT.json','historical-formatting/FORMATTING.json']+[f'actual-check/language-header-test/{n}' for n in ['START.json','CHILD.json','RESULT.json','STDOUT.log','STDERR.log']]+['run_actual.py','source-context/crates/mori-core/src/lib.rs']+[f'source-context/crates/mori-lexer/src/{n}.rs'for n in ['lib','token','scan','string','number']]+['source-context/crates/mori/src/commands/check.rs','source-context/crates/mori/src/commands/check_source.rs']
primary=''
for path in whole:primary+=f'## COMPLETE FILE: {path}\n```text\n'+(P/path).read_text()+'\n```\n\n'
for r in ranges:primary+=f"## EXACT RANGE backed by {r['wholeFile']} lines {r['lineStart']}-{r['lineEnd']}: {r['path']}\n```text\n"+(P/r['path']).read_text()+'\n```\n\n'
(P/'PRIMARY.md').write_text(primary)
token=D.name+'-whole-review'
prompt=f'Fresh independent ordinary source and actual-result audit. Return one whole object using reviewToken {token}. All mandatory candidate source, complete expected diagnostic cases, final patch, actual new result logs, driver, lexer semantics, and current check dispatch are inline. Physical source/evidence is available in the packet; range headings disclose exact excerpts, not complete-file inspection. Read-only source inspection is permitted. No runtime/provider calls, authoring or security judgments.\n\n'+primary
(D/'PROMPT.md').write_text(prompt);assert len(prompt.encode())<125000
schema=json.loads((O.parent/'OUTPUT-SCHEMA.json').read_text());schema['properties']['reviewToken']['const']=token
for k,v in [('base',base),('head',head),('tree',tree)]:schema['properties']['exactGuard']['properties'][k]['const']=v
write(D/'OUTPUT-SCHEMA.json',schema)
files=[{'path':str(p.relative_to(P)),'bytes':p.stat().st_size,'sha256':sha(p)}for p in sorted(P.rglob('*'))if p.is_file()and p.name!='MANIFEST.json']
write(P/'MANIFEST.json',{'base':base,'head':head,'tree':tree,'files':files})
for f in files:assert sha(P/f['path'])==f['sha256']
validation={'checkedAtUtc':now(),'allPacketMemberHashesVerified':len(files),'wholeSourceBodyPinsVerified':len(s['wholeSourceFiles']),'currentSourceBodiesMatchNewRootRecordedSourcePins':True,'exactRangesVerified':len(ranges),'evidenceQuotesVerified':True,'rootSourceManifestBindingVerified':True,'actualStdoutStderrHashesVerified':True,'newDriverHashVerified':True,'wholeCandidatePatchByteIdenticalToHistorical':True,'promptBytes':len(prompt.encode()),'promptBelow125000':True,'mandatoryFullCandidateFilesInline':all((P/p).read_text()in prompt for p in whole if p.startswith('candidate/')),'providerCalls':0,'subjectExecutionCalls':0,'sharedWrites':0}
write(D/'VALIDATION.json',validation)
write(D/'PREPARED.json',{'preparedAtUtc':now(),'base':base,'head':head,'tree':tree,'reviewToken':token,'manifestSha256':sha(P/'MANIFEST.json'),'promptSha256':sha(D/'PROMPT.md'),'promptBytes':len(prompt.encode()),'selectedTestExitCode':0,'selectedTestWallSeconds':result['wallSeconds'],'selectedTestPassed':1,'selectedTestFiltered':18,'providerCalls':0,'subjectExecutionCalls':0,'sharedWrites':0,'status':'prepared only; no reviewer dispatched or approval granted','oldFrozenPacket':'language-header-diagnostic-source-result-review-20261018 unchanged'})
(D/'README.md').write_text(f'''Prepared ordinary public-library diagnostic source/result packet for base {base}, head {head}, tree {tree}.\n\nPROMPT.md ({len(prompt.encode())} bytes) includes the complete two-file candidate and actual new selected-test receipts. packet/MANIFEST.json pins every physical member. Current CLI check routing is represented by the new check_source helper; the test establishes library API diagnostics only. The new root-captured test passed once, with 18 filtered; no subject execution or provider calls occurred during packet construction. Historical root formatting metadata and original patch remain distinct from new raw test evidence. Old frozen packets remain unchanged. No approval or broader product/security/native claim is made.\n''')
print(json.dumps({'prepared':json.loads((D/'PREPARED.json').read_text()),'validation':validation},indent=2))
