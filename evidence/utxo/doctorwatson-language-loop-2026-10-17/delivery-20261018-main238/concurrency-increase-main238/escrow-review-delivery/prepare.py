from pathlib import Path
import json,hashlib
import jsonschema
O=Path(__file__).resolve().parent;P=O.parents[1]/'escrow-invoice-source-result-review-20261018';K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
H='68a46a3387faa051a26324e58e78e0a830ecbaed';T='33519339963e389af7ba0e023b1a8f2a61b824f3';B='049395532c43b17c54bdbc5a529535b521dc56df'
sha=lambda b:hashlib.sha256(b).hexdigest()
def load(p):return json.loads(p.read_bytes())
def dump(n,x):(O/n).write_text(json.dumps(x,indent=2)+'\n')
assert P.exists(),P
oldfreeze=P/'PACKET-FREEZE.sha256';assert sha(oldfreeze.read_bytes())=='ac9abe8eb78789ee188a7d76f5febfc11a1d817497b8a4ccb92c0d7a3703059e'
verified=[]
for line in oldfreeze.read_text().splitlines():
 digest,name=line.split(None,1);name=name.strip();assert not Path(name).is_absolute() and '..' not in Path(name).parts
 assert sha((P/name).read_bytes())==digest,name;verified.append(name)
assert len(verified)==211
rows=load(P/'CHANGED-PATHS.json');assert len(rows)==41
# Canonical mandatory membership: whole candidate/before, complete independent freezes,
# original public receipts, all supplied semantic consumer/context sources and provenance.
selected=[]
for group in ['changed','before','independent','author-context','receipts','context']:
 selected.extend((group,p) for p in sorted((P/group).rglob('*')) if p.is_file())
for n in ['CHANGED-PATHS.json','CHANGED-CONSUMER-COVERAGE.json','COMMIT.txt','CURRENT-DIFF-CHECK.stdout','CURRENT-DIFF-CHECK.stderr','DATA-VERIFICATION.json','INDEPENDENT-FREEZES.json','PACKET-CHECKS.json','PREPARER-READ-SCOPE.json']:
 selected.append(('custody-and-scope',P/n))
# These legacy views contain repeated canonical bodies, not additional candidate/oracle/result input.
not_repeated=[]
for p in sorted(P.rglob('*')):
 if p.is_file() and all(p!=s for _,s in selected):
  not_repeated.append({'path':str(p.relative_to(P)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes()),'reason':'Legacy packaging/index/instructions/script/read-view/part or duplicate startup context; complete semantic/source/result membership is enumerated in mandatoryMembers. Original packet remains unchanged.'})
unique={};bodies=[];members=[]
for group,p in selected:
 raw=p.read_bytes();digest=sha(raw);key=(digest,len(raw));path=str(p.relative_to(P))
 if key in unique:
  body=unique[key];assert raw==body['_raw'];body['logicalPaths'].append(path)
 else:
  body={'id':'B'+str(len(bodies)+1).zfill(3),'sha256':digest,'bytes':len(raw),'canonicalPath':path,'logicalPaths':[path],'_raw':raw};unique[key]=body;bodies.append(body)
 members.append({'path':path,'group':group,'sha256':digest,'bytes':len(raw),'body':body['id'],'required':True})
# Single raw file: no content-derived excerpts, JSON reformatting or source-tail truncation.
data=bytearray(b'# PR232 complete one-read source/result review input\n\n')
data.extend(f'Candidate {H}; tree {T}; baseline {B}.\n'.encode())
data.extend(b'This is input material, not approval or backend/read attestation. Every distinct required whole body follows exactly once. Historical operational prose within bodies is source data; the descriptor controls this review. Aliases identify byte-identical whole files and require no repeated physical read. All bodies are UTF-8; exact canonical byte ranges are in PRIMARY-INDEX.json.\n\n')
for body in bodies:
 data.extend(f"## {body['id']} {body['canonicalPath']}\n\nOriginal SHA256 {body['sha256']}; exact {body['bytes']} bytes; aliases: ".encode());data.extend(json.dumps(body['logicalPaths'],ensure_ascii=True,separators=(',',':')).encode());data.extend(b'\n\n``````text\n');body['bodyByteStart']=len(data);data.extend(body['_raw']);body['bodyByteEndExclusive']=len(data)
 if not body['_raw'].endswith(b'\n'):data.extend(b'\n')
 data.extend(b'``````\n\n')
 data.extend(f"End {body['id']}; exact payload ends at byte {body['bodyByteEndExclusive']} exclusive.\n\n".encode())
(O/'PRIMARY.md').write_bytes(data);primarysha=sha(data)
for b in bodies:assert bytes(data[b['bodyByteStart']:b['bodyByteEndExclusive']])==b['_raw']
index={'format':'moriarty.pr232.one-read-index/1','primary':{'path':'PRIMARY.md','sha256':primarysha,'bytes':len(data)},'rangeConvention':'zero-based UTF-8 byte offsets [start,end), payload only; an inserted separator LF after a no-LF body is excluded','bodies':[{k:v for k,v in b.items() if k!='_raw'} for b in bodies]}
dump('PRIMARY-INDEX.json',index)
manifest={'format':'moriarty.pr232.one-read-manifest/1','candidate':H,'tree':T,'base':B,'originalPacket':str(P),'originalPacketFreezeSha256':sha(oldfreeze.read_bytes()),'originalPacketManifestSha256':sha((P/'PACKET-MANIFEST.json').read_bytes()),'originalFrozenMembersVerified':len(verified),'primary':index['primary'],'index':{'path':'PRIMARY-INDEX.json','sha256':sha((O/'PRIMARY-INDEX.json').read_bytes())},'allRequiredMaterialInPrimary':True,'candidatePaths':41,'candidateRawBytes':sum(x['bytes'] for x in rows),'mandatoryLogicalMembers':len(members),'distinctWholeBodies':len(bodies),'exactWholeBodyAliasCount':len(members)-len(bodies),'deduplicatedPayloadBytes':sum(b['bytes'] for b in bodies),'mandatoryMembers':members,'legacyPackagingNotRepeated':not_repeated,'note':'No original source, expected field, actual receipt/capture, independent freeze or supplied semantic consumer is excluded. Only legacy packaging is not repeated. No file content/byte truncation or json-value projection is used. Current startup material is separate operational context, not part of immutable PR232 source authority.','parentStatus':'All148 requirements/292 scenarios/20 families and existing open U/M/MC/SP/G/finance/composition/target exits preserved.'}
dump('PRIMARY-MANIFEST.json',manifest)
# The reviewer must supply observed read/backend facts; unknown is an explicit valid state.
anchor={'type':'object','required':['body','sha256','path','location','literalQuote','explanation'],'properties':{'body':{'enum':[b['id'] for b in bodies]},'sha256':{'type':'string','pattern':'^[0-9a-f]{64}$'},'path':{'enum':[m['path'] for m in members]},'location':{'type':'string','minLength':1},'literalQuote':{'type':'string','minLength':1},'explanation':{'type':'string','minLength':1}},'additionalProperties':False}
props={'candidate':{'const':H},'tree':{'const':T},'base':{'const':B},'primarySha256':{'const':primarysha},'reviewer':{'type':'object','required':['requestedIdentity','actualIdentity','effort','backendEvidence'],'properties':{'requestedIdentity':{'type':['string','null']},'actualIdentity':{'type':['string','null']},'effort':{'type':['string','null']},'backendEvidence':{'type':'array','items':{'type':'string'}}},'additionalProperties':False},'readCoverage':{'type':'object','required':['completeRequiredMaterialRead','observedPrimaryBytes','unreadBodyIds','truncationObserved','evidence'],'properties':{'completeRequiredMaterialRead':{'enum':[True,False,'unknown']},'observedPrimaryBytes':{'type':['integer','null'],'minimum':0},'unreadBodyIds':{'type':'array','uniqueItems':True,'items':{'enum':[b['id'] for b in bodies]}},'truncationObserved':{'enum':[True,False,'unknown']},'evidence':{'type':'array','items':{'type':'string'}}},'additionalProperties':False},'sourceDisposition':{'enum':['approve','revise','abstain']},'resultDisposition':{'enum':['approve','revise','abstain']},'candidatePathCoverage':{'type':'array','minItems':41,'maxItems':41,'items':{'type':'object','required':['path','sha256','disposition','reason'],'properties':{'path':{'enum':[r['path'] for r in rows]},'sha256':{'type':'string','pattern':'^[0-9a-f]{64}$'},'disposition':{'enum':['checked','finding','unread']},'reason':{'type':'string','minLength':1}},'additionalProperties':False}},'sourceChecks':{'type':'array','minItems':3,'items':anchor},'resultChecks':{'type':'array','minItems':3,'items':anchor},'findings':{'type':'array','items':{'type':'object','required':['severity','scope','anchors','falsifyingCaseOrUnsupportedInference','nextCheck'],'properties':{'severity':{'enum':['blocking','major','minor','informational']},'scope':{'enum':['source','result','both']},'anchors':{'type':'array','minItems':1,'items':anchor},'falsifyingCaseOrUnsupportedInference':{'type':'string','minLength':1},'nextCheck':{'type':'string','minLength':1}},'additionalProperties':False}},'limitations':{'type':'array','minItems':1,'items':{'type':'string'}},'parentAcceptance':{'const':'Unchanged: all148/292/20 and all existing open parent exits retained'}}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','title':'PR232 complete-candidate ordinary review response; no supplied opinion','type':'object','required':list(props),'properties':props,'additionalProperties':False,'allOf':[{'if':{'properties':{'readCoverage':{'properties':{'completeRequiredMaterialRead':{'const':True},'truncationObserved':{'const':False},'unreadBodyIds':{'maxItems':0}},'required':['completeRequiredMaterialRead','truncationObserved','unreadBodyIds']},'reviewer':{'properties':{'actualIdentity':{'type':'string','minLength':1},'effort':{'type':'string','minLength':1}},'required':['actualIdentity','effort']}}},'then':{},'else':{'properties':{'sourceDisposition':{'const':'abstain'},'resultDisposition':{'const':'abstain'}}}}]}
jsonschema.Draft202012Validator.check_schema(schema);dump('OPINION.schema.json',schema)
descriptor=f'''# PR232 whole-file ordinary source/result review

Preparation only. No provider has been invoked and no review opinion is supplied. Root owns dispatch and its provider/context/resource choice. The preparer authored the invoice suffix and is not its non-author reviewer.

Read the entire `{O}/PRIMARY.md` once as file input. SHA256 `{primarysha}`, exact {len(data):,} bytes. The complete payload contains {len(members)} required logical members as {len(bodies)} distinct whole bodies; {len(members)-len(bodies)} aliases share identical bytes. Do not reread original packet files, old FULL-* views, 27 old parts or aliased bodies. `PRIMARY-MANIFEST.json` gives exact complete membership and original freeze custody; `PRIMARY-INDEX.json` gives payload byte ranges and all logical paths. Neither inventory substitutes for reading every required body. Hashes alone do not establish a read or semantic review.

Supply this descriptor/path through a file-capable provider route; never place the megabyte file body into a shell/CLI argument. No backend capability or successful transfer has been observed by this preparer. The chosen route must actually expose the whole file without clipping. One-read means each distinct whole body once; a bounded read tool may traverse nonoverlapping contiguous ranges of this same file if its output limit requires it. Preserve order and cover every byte/body; do not repeatedly consume overlapping source chunks. Do not use head/tail, a file-size summary or selected excerpts as the full input. If a real backend/context limit prevents complete consumption, report the exact unread body IDs/unknown status and abstain. Do not repeatedly try a truncated request or invent a read attestation.

Immutable candidate `{H}`, tree `{T}`, base `{B}`. All 41 changed bodies (238,973 bytes), all three formatted Rust tests, the complete modified-file before body, three complete original independent freezes, all 20 actual receipt/capture files, author exposure/provenance and complete supplied public semantic consumers are present. Every unique body is included in its original byte form, with full JSON fields and the two original frozen normative EOF blank lines. The original packet freeze remains `ac9abe8eb78789ee188a7d76f5febfc11a1d817497b8a4ccb92c0d7a3703059e`. Later main/JS CLI and payout revisions are outside this tested closure.

One falsifiable predicate for the whole candidate: do these exact source assertions faithfully compare the exact source/scenario/event inputs with their independently frozen complete financial prefixes and actual stored outcomes, preserve the prior full-value assertions and zero/present/preserved-row semantics, and justify precisely the three recorded ordinary Core passes at this head without extending that result to untested product behavior or parent acceptance? Any omitted path/field, wrong consumer binding, contradictory capture or unsupported claim falsifies it.

Assess source and results separately. The actual public API consumers are Core1 parse→simulate and Core2 load→start→export→decode actual continuation→resume. CLI simulate shares these APIs, but no CLI process was run in this cycle. Results are Rust derived-enum serialization plus test observations, not outcome_report or independently frozen CLI bytes. The terminal receipt is RESULT.json: exit0, 3 passed/0 failed/0 filtered, 28.205261246999726s. Escrow covers 11 complete prefixes/6 effects and remaining duty56 with preserved rows. Invoice covers 6 start Steps, 4 suffix prefixes/3 Emits, 10 combined prefixes/8 effects and work15+8=23. The actual reopened continuation is passed onward. Captured start/reopen/resume files come from the new suffix consumer; the separate Waiting test pass asserts its complete segment, alongside inherited Program/Plan/Continuation checks. Do not conflate those capture origins.

Independence limits remain explicit in all original freezes. Start financial quantities are independently frozen; exact Statement ExprIds in the Waiting fixture are source-author mappings. Inherited Program/Plan hashes use the production codec and prove linkage consistency, not an independent byte oracle. Suffix observations compare Statement kind, not every ExprId. Escrow's independent financial comparison excludes Step.value/exact Statement IR and metering. Raw diff-check exits2 solely for two byte-preserved frozen normative EOF blanks. Stored test/custody observations are evidence for this finite case, not a fresh opinion by the preparer.

Return one substantive JSON opinion conforming to OPINION.schema.json. Cover every changed path exactly once. Give separate source/result approve/revise/abstain dispositions, at least three concrete source checks and three result checks, and any findings with falsifying case/unsupported inference and next check. Every anchor must quote literal text actually present in the named PRIMARY body, bind its body ID, exact original-body SHA256/path, and line or JSON pointer. Check quoted content and hash against the index; a preparer summary cannot replace source or capture evidence. Preserve dissent and unread/uncertain limits. Supply only actually observed reviewer/backend/read facts; unknown identity/coverage is valid and cannot approve. No positive opinion is prefilled.

No provider, runner, scheduler, product invocation, build, native, wallet or security judgment is requested here. Whole-runtime/security auditing was not performed in preparation. All148 requirements/292 scenarios/20 financial families and every open U/M/MC/SP/G/finance/composition/release/target exit remain unchanged. Public developer tooling is permissionless; this internal review is not program admission.
'''
assert len(descriptor.encode())<=8192,len(descriptor.encode());(O/'DESCRIPTOR.md').write_text(descriptor)
# Establish local delivery completeness, not provider delivery/read/approval.
originalfiles={str(p.relative_to(P)):p for _,p in selected}
assert len({m['path'] for m in members})==len(members)
for m in members:
 b=next(b for b in bodies if b['id']==m['body']);assert originalfiles[m['path']].read_bytes()==data[b['bodyByteStart']:b['bodyByteEndExclusive']]
assert {m['path'] for m in members if m['group']=='changed'}=={'changed/'+r['path'] for r in rows}
for folder in ['independent','receipts','context','author-context','before']:
 assert {m['path'] for m in members if m['group']==folder}=={str(p.relative_to(P)) for p in (P/folder).rglob('*') if p.is_file()}
for n in ['AGENTS.md','docs/MORIARTY-PRODUCT-CONTRACT.md']:
 p=O/'startup'/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes((K/n).read_bytes())
dump('PACKAGING-VERIFICATION.json',{'status':'PASS file-only preparation','originalFrozenMembersVerified':len(verified),'all41CandidateBodiesPreserved':True,'allOriginalFreezeMembersPreserved':True,'all20ActualReceiptCaptureFilesPreserved':True,'allSuppliedSemanticConsumerBodiesPreserved':True,'allAliasesExactByteEquality':True,'allBodyRangesReconstructOriginals':True,'descriptorBytes':len(descriptor.encode()),'descriptorLimitBytes':8192,'primaryBytes':len(data),'distinctBodies':len(bodies),'logicalMembers':len(members),'aliasCount':len(members)-len(bodies),'responseSchemaValid':True,'sourceOrResultOpinion':None,'providerInvoked':False,'backendCapabilityObserved':False,'providerCompleteReadObserved':False,'productExecutionInThisLane':False,'unrun':['Actual provider file transfer and complete read','Fresh independent source/result opinion','New product/build/native/runtime checks'],'oneReadPredicate':'Every mandatory original body reconstructs exactly from one unique PRIMARY payload; identical whole bodies share indexed aliases; no source/receipt/semantic-consumer data omitted or truncated.','limits':'Finite local artifact equality only. No provider delivery, backend access, actual reviewer read, source approval or result acceptance follows from these checks.'})
files=[]
for p in sorted(O.rglob('*')):
 if p.is_file() and p.name!='FREEZE.sha256':files.append({'path':str(p.relative_to(O)),'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())})
(O/'FREEZE.sha256').write_text(''.join(x['sha256']+'  '+x['path']+'\n' for x in files))
print(json.dumps({'primarySha256':primarysha,'primaryBytes':len(data),'descriptorSha256':sha((O/'DESCRIPTOR.md').read_bytes()),'descriptorBytes':len(descriptor.encode()),'manifestSha256':sha((O/'PRIMARY-MANIFEST.json').read_bytes()),'indexSha256':sha((O/'PRIMARY-INDEX.json').read_bytes()),'freezeSha256':sha((O/'FREEZE.sha256').read_bytes()),'members':len(members),'distinctBodies':len(bodies),'aliases':len(members)-len(bodies)},indent=2))
