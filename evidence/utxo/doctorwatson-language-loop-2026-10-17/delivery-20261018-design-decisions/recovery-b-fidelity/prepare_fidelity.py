from pathlib import Path
import hashlib,json,re
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
ROOT=R/'durable-recovery-full-readonce-terminal-reviews-20261018'
P=ROOT/'packet'
OUT=R/'concurrency-increase-main238/recovery-b-fidelity'
def sha(b): return hashlib.sha256(b).hexdigest()
def dump(name,o): (OUT/name).write_text(json.dumps(o,indent=2,ensure_ascii=True)+'\n')
primary=(P/'PRIMARY.md').read_bytes(); lines=primary.decode().splitlines()
inv=json.loads((P/'MANDATORY-INVENTORY.json').read_text()); review_bytes=(ROOT/'job-b/TERMINAL-REVIEW.json').read_bytes();review=json.loads(review_bytes)
assert len(primary)==500104 and sha(primary)=='80c1630951009981e1eaac6d03660dfdeb6cc4164ae593e34f344b5968fd6b75'
assert len(review_bytes)==25735 and sha(review_bytes)=='e5ad71e3ba263feb3d60bba6b2a52d21d9b5db7574a3f4f51ae18395d8da2e1f'
assert [x['id'] for x in inv['files']]==[f'B{i:02}' for i in range(1,45)]
bodies={};checks=[];pins=[]
def pin(p,scope):
 b=p.read_bytes();rec={'path':str(p),'bytes':len(b),'sha256':sha(b),'scope':scope};pins.append(rec);return rec
for f in inv['files']:
 a,z=f['primary_body_byte_start'],f['primary_body_byte_end_exclusive'];b=primary[a:z];original=Path(f['original_absolute_path']).read_bytes()
 assert b==original and len(b)==f['bytes'] and sha(b)==f['sha256']
 first=primary[:a].count(b'\n')+1
 end=first+len(b.splitlines())-1
 bodies[f['id']]={'entry':f,'bytes':b,'first':first,'last':end}
 checks.append({**f,'primary_first_body_line_1based':first,'primary_last_body_line_1based':end,'original_bytes_match':True,'slice_size_match':True,'slice_sha256_match':True})
 pin(Path(f['original_absolute_path']),'complete original public body; hash/byte custody, not assertion of semantic review of every line')
assert sum(x['bytes'] for x in inv['files'])==488685
for n in ['PRIMARY.md','MANDATORY-INVENTORY.json','DELIVERY-MANIFEST.json','WHOLE-OPINION.schema.json','MANIFEST.json','PROMPT-B.md']:
 pin(P/n,'B decision packet custody; no A opinion read')
for n in ['TASK-B.json','job-b/TERMINAL-REVIEW.json','job-b/TERMINAL-EXPORT-RECEIPT.json','job-b/LIVE-METADATA.json']:
 pin(ROOT/n,'public B transport/whole opinion')
original_manifest=R/'durable-recovery-full-design-decision-review-20261018/MANIFEST.json'
assert pin(original_manifest,'candidate manifest identity only')['sha256']=='fefd01068978d1e5a8e7f3eb738e562795baf2c0a613c55594b114cbdbf281e4'
task=json.loads((ROOT/'TASK-B.json').read_text());receipt=json.loads((ROOT/'job-b/TERMINAL-EXPORT-RECEIPT.json').read_text());delivery=json.loads((P/'DELIVERY-MANIFEST.json').read_text())
assert sha((P/'MANIFEST.json').read_bytes())==task['manifestSha256']
assert sha((P/'WHOLE-OPINION.schema.json').read_bytes())==task['schemaSha256']
assert sha((P/'PROMPT-B.md').read_bytes())==task['promptSha256']
assert receipt['taskSha256']==sha((ROOT/'TASK-B.json').read_bytes())
assert review['candidate']['primary_sha256']==sha(primary)
assert review['candidate']['manifest_sha256']==sha(original_manifest.read_bytes())
manifest=json.loads((P/'MANIFEST.json').read_text())
for x in manifest['files']:
 if x['path']=='PROMPT-A.md': continue # not needed for B fidelity
 assert sha((P/x['path']).read_bytes())==x['sha256']
for src,dst in [('job-b/TERMINAL-REVIEW.json','UNCHANGED-TERMINAL-REVIEW.json'),('job-b/TERMINAL-EXPORT-RECEIPT.json','PUBLIC-EXPORT-RECEIPT.json'),('TASK-B.json','TASK-B.json')]:
 (OUT/dst).write_bytes((ROOT/src).read_bytes())
dump('BODY-CUSTODY-CHECKS.json',{'primary':{'path':str(P/'PRIMARY.md'),'bytes':len(primary),'sha256':sha(primary),'LF_terminated_physical_lines':len(lines),'LF_count_plus_editor_empty_final_line':primary.count(b'\n')+1},'body_count':len(checks),'body_raw_bytes':488685,'all44_exact_original_and_range_hash_matches':True,'checks':checks})
# Each numeric range cited by the five findings (including repetitions) uses PRIMARY coordinates.
# Preserve the unmodified review field; publish the file-local conversion separately.
ranges={
'F-01':{'B01':'57,84,107-113,107-132','B02':'330,342','B21':'3568-3572,3904-3915','B26':'5283-5291','B32':'6549'},
'F-02':{'B01':'60-85,158-169','B08':'1458-1461,1460','B20':'3335-3364,3354','B22':'3943-3955,4054-4077','B25':'4909-4911'},
'F-03':{'B04':'759-765,761-763','B06':'1262-1275','B25':'4953-4978,5058-5117'},
'F-04':{'B01':'182,218-220,259,248-260','B02':'354','B08':'1418-1421,1430,1443-1448,1414-1423,1440-1449'},
'F-05':{'B03':'641-653','B04':'743,777,773-778','B09':'1481-1483,1481-1484','B34':'6999-7016'}
}
anchors=[]
for fid,bs in ranges.items():
 for bid,rs in bs.items():
  body=bodies[bid]
  for r in rs.split(','):
   aa=r.split('-');a=int(aa[0]);z=int(aa[-1]);assert body['first']<=a<=z<=body['last']
   excerpt='\n'.join(lines[a-1:z])+'\n'
   local_a=a-body['first']+1;local_z=z-body['first']+1
   rawlines=body['bytes'].decode().splitlines()
   assert rawlines[local_a-1:local_z]==lines[a-1:z]
   anchors.append({'finding_id':fid,'body_id':bid,'source_path':body['entry']['original_absolute_path'],'source_sha256':body['entry']['sha256'],'review_numeric_anchor':r,'correct_coordinate_system':'PRIMARY.md 1-based physical lines','primary_lines':[a,z],'correct_file_local_lines':[local_a,local_z],'literal_excerpt':excerpt,'excerpt_sha256':sha(excerpt.encode()),'status':'SOURCE_SUPPORTED_AT_PRIMARY_COORDINATES; FILE_LOCAL_CITATION_REQUIRES_CORRECTION'})
# Additional exact evidence clarifies the claimed dependency, without selecting an owner solution.
extra=[]
for bid,a,z,why in [
 ('B21',3702,3723,'Step/Execution fields and explicit frozen v1 has no Await'),
 ('B21',3724,3772,'complete v1 Statement variants show no Await'),
 ('B22',4024,4048,'parse and parse_limited byte ceilings; same depth/node limits'),
 ('B25',4940,4943,'continuation decoder uses MAX_CONTINUATION_BYTES'),
 ('B26',5171,5172,'numeric 4 MiB continuation and 64 KiB event constants; cited B25 lines reexport/name message only'),
 ('B23',4313,4320,'value-returning start API'),('B23',4378,4386,'value-returning resume API'),
 ('B26',5211,5227,'value-returning load API'),
 ('B32',6535,6537,'ordinary adoption and separately designated-owner M03 distinction'),
 ('B19',3154,3192,'existing stage_test keeps actual pending continuation in memory, distinct from proposed fresh-process tests'),
 ('B32',6668,6681,'owner-prerequisite O-ID references existing designated-owner F4; not a new recovery identity adoption')]:
 body=bodies[bid];assert body['first']<=a<=z<=body['last'];txt='\n'.join(lines[a-1:z])+'\n'
 extra.append({'body_id':bid,'primary_lines':[a,z],'correct_file_local_lines':[a-body['first']+1,z-body['first']+1],'source_sha256':body['entry']['sha256'],'purpose':why,'literal_excerpt':txt,'excerpt_sha256':sha(txt.encode())})
dump('SOURCE-ANCHOR-CORRECTIONS.json',{'opinion_unchanged':True,'status':'coordinate corrections and supplementary source anchors only; no new ordinary vote or owner rule','all_numeric_finding_citations_accounted_for':True,'caveat':'Review writes named-file:line citations, but every listed number is a PRIMARY.md coordinate. Do not use those numbers directly in the original file. Repeated or overlapping citations remain visible. Identifiers/code words in prose are not falsely represented as exact sentence quotations.','anchors':anchors,'supplementary_support':extra})
occurrences=[]
pathmap={v['entry']['path']:k for k,v in bodies.items()}
for f in review['findings']:
 for field in ['source_location','finding','consequence','requested_disposition']:
  text=f[field]
  for m in re.finditer(r'(?P<path>[A-Za-z0-9_./-]+\.(?:json|md|rs)):(?P<ranges>\d+(?:-\d+)?(?:,\d+(?:-\d+)?)*)',text):
   bid=pathmap[m['path']]
   for rg in m['ranges'].split(','):
    assert any(x['finding_id']==f['id'] and x['body_id']==bid and x['review_numeric_anchor']==rg for x in anchors),(f['id'],bid,rg)
   occurrences.append({'review_json_pointer':'/findings/'+str(review['findings'].index(f))+'/'+field,'literal_citation':m.group(0),'citation_utf8_sha256':sha(m.group(0).encode()),'body_id':bid,'ranges':m['ranges'].split(','),'all_ranges_in_separate_corrections':True})
dump('CITATION-OCCURRENCES.json',{'scope':'Every filename:numeric-range citation in all five findings, consequences and requested dispositions, with duplicates retained. Non-numeric API/body/F4 references are addressed in supplementary source anchors and finding assessments.','count':len(occurrences),'occurrences':occurrences})
findings=[]
assessment={
'F-01':('SUPPORTED_CONDITIONAL_CODEC_DEPENDENCY','B01/B02 require complete baseline reports in saved history; B32 explicitly requires an Await-bearing Step on a true resume. B21 shared Statement/Step/Execution Deserialize routes via frozen v1 without Await, and B26 StageSegment is Serialize-only. Thus directly deriving shared decoding as-is is incompatible with the proposed complete-history carrier. No recovery decoder was executed or supplied. The suggested dedicated DTO/DeserializeSeed is a reviewer-requested owner disposition, not an owner-adopted algorithm.'),
'F-02':('SUPPORTED_DISTINCT_SIZE_ENVELOPES; NO_REPRODUCED_FAILURE','B22 exports through the 1 MiB parser, depth32/node65536 checks, ASCII escaping and final-LF bound. B20 exports each case independently; it deliberately does not put a 1 MiB ceiling on the entire multi-case aggregate. B25 uses the continuation-specific bound; B26 supplies its exact 4*1048576 constant. B08 proposes repeated report/checkpoint/stdout nesting. This supports an owner consistency question, not proof any concrete normal case currently fails, nor selection of new bounds.'),
'F-03':('SUPPORTED_COMBINED_DOMAIN_DEPENDENCY','B25 registry ranges over base p.actions; explicit account checks at the cited loop concern selected base action Transfer/Fee accounts. B04 states a reserve-to-buyer handler transfer absent from the base prefix transfer/fee list. The source summaries and fixed current scope support asking O-ACCT/O-REG for the paired domain; optional source specimens were not opened or certified. No new balance/authority/admission rule is authored here.'),
'F-04':('SUPPORTED_NONBLOCKING_SCHEMA_SYNCHRONIZATION_NOTE','B02 gives the named RecoveryPlan fields; B01 has the ceiling equation and complete plan inspection but no equivalent structured root Plan field-map block. B08 states reopen+transition for CLI resume/recover and distinguishes event-free reopen_report. This is the reviewer\'s note for a later schema freeze, with required_ordinary_changes still empty; it does not create a new blocking vote.'),
'F-05':('SUPPORTED_EVIDENCE_CUSTODY_LIMITATION','B04/B09 explicitly disclose author-assembled source-exposed specimens and L04 financial-only scope. B03 DR-016 and B34 UNI-020 retain distinct independent full expectations before scoring. Counts74/27 are proposal statements, not custody certification of those unread optional files. No example arithmetic, work, identity or bytes are certified by this opinion or this fidelity check.')
}
review_lines=review_bytes.decode().splitlines()
for f in review['findings']:
 aid,note=assessment[f['id']]
 fields={}
 for k in ['source_location','finding','consequence','requested_disposition']:
  quoted=json.dumps(f[k],ensure_ascii=True);hits=[i for i,s in enumerate(review_lines,1) if quoted in s];assert len(hits)==1
  fields[k]={'review_json_line':hits[0],'literal':f[k],'utf8_sha256':sha(f[k].encode())}
 findings.append({'id':f['id'],'severity':f['severity'],'kind':f['kind'],'fidelity_assessment':aid,'scope_and_qualification':note,'unchanged_review_fields':fields})
dump('FINDING-FIDELITY.json',{'scope':'ordinary literal/source fidelity, not a correctness or security vote','finding_count':5,'minor':3,'note':2,'blocking':0,'findings':findings})
# Exact35 crosswalk comparison against the complete original consolidated traceability table.
trace=json.loads(bodies['B05']['bytes'])['requirements'];table=bodies['B35']['bytes'].decode();ears=json.loads(bodies['B03']['bytes'])['requirements'];owners=json.loads(bodies['B06']['bytes'])['rules'];contract=json.loads(bodies['B01']['bytes'])
assert [x['id'] for x in trace]==[f'MPLR-{i:03}' for i in range(1,36)]
trace_checks=[]
for row in trace:
 pat=r'^\| \['+re.escape(row['id'])+r'\]\([^\n]+?\)\s*\|([^\n]+)\|$'
 matches=re.findall(pat,table,re.M);assert len(matches)==1,(row['id'],matches)
 cells=[s.strip() for s in matches[0].split('|')];assert cells==[row['existing_title'],row['existing_phase_owner'],row['existing_consolidation'],row['original_status']],(row['id'],cells)
 norm=next(x for x in inv['files'] if x['path']==row['whole_original_source']);body=bodies[norm['id']]
 assert row['id'] in body['bytes'].decode()
 assert row['original_status']=='specified-only' and 'not parent acceptance' in row['proposal_disposition']
 assert all(any(e['id']==rid and row['id'] in e['source_requirements'] or e['id']==rid and 'MPLR-001..035' in e['source_requirements'] for e in ears) for rid in row['proposal_rows_referencing_id'])
 trace_checks.append({**row,'original_table_fields_exact_match':True,'whole_normative_body_id':norm['id'],'whole_normative_body_sha256':norm['sha256'],'full_normative_body_present_in_PRIMARY':True})
assert len(ears)==20 and [x['id'] for x in ears]==[f'DR-{i:03}' for i in range(1,21)]
assert all(not x['acceptance_closed'] and x['state']=='PROPOSED_ONLY' for x in ears)
assert [x['id'] for x in owners]==contract['owner_rule_dependencies']==[x['id'] for x in review['owner_prerequisites']]
assert all(x['mandatory_before_dependent_implementation'] and not x['supplied_by_ordinary_author'] for x in owners)
assert all(x['mandatory_before_dependent_implementation'] and not x['security_rule_authored_here'] for x in review['owner_prerequisites'])
ownerchecks=[]
for a,b in zip(owners,review['owner_prerequisites']):
 ownerchecks.append({'id':a['id'],'proposal_exact_missing_contract':a['missing_exact_contract'],'review_exact_dependency':b['interface_gap_or_dependency'],'mandatory_before_dependent_implementation':True,'rule_authored_or_adopted_by_fidelity_lane':False,'review_authors_security_rule':False})
lifecycle=[
 ('paired source and whole inspection','DR-001','DR-002'),('complete pending and both closed records','DR-003','DR-020'),('actual start and original baseline','DR-004'),('publish, process exit, actual event-free fresh reopen','DR-005','DR-013'),('false and true normal resume, forward completion','DR-006','DR-011'),('captured site-bound refund and compensation','DR-007','DR-008','DR-009'),('present-zero and positive residual duties','DR-010'),('separate work/paired ceiling arithmetic','DR-011','DR-012'),('whole qualified public reports and bytes','DR-014'),('fresh OS process mori test and actual file inputs','DR-015'),('independent full expectations before scoring','DR-016'),('original35/financial/composition/targets/native/private/live exits','DR-017'),('owner complete failure/unknown/partial and resource prerequisites','DR-018','DR-019')]
dump('SCOPE-RETENTION.json',{'original35_exact_crosswalk':trace_checks,'original35_statuses_unchanged':True,'full_lifecycle_EARS':ears,'lifecycle_fidelity':[{'subject':x[0],'source_EARS_ids':list(x[1:]),'retained_in_B_scope':True} for x in lifecycle],'owner_prerequisites':ownerchecks,'review_retained_scope_exact':review['retained_scope'],'scope_qualification':'Retention is contractual scope, not fulfillment: all14 staged gaps;148requirements/292scenarios/20families;119UTXO/11first-contract/18PCD inventories;69U/M/MC/SP/G exits;ACTUS277;DeFi72;five composition operators;all proof/target/ledger/private/live-consumption and release obligations remain. B does not separately enumerate every14gap ID; their complete adopted scope source remains in B32/B34 and no closure is claimed. All20 DR rows remain proposed-only/acceptance_closed:false. This lane makes no new owner policy or adoption judgment.'})
schema=(P/'WHOLE-OPINION.schema.json').read_bytes()
dump('CUSTODY-CORRECTIONS.json',{'opinion_unchanged':True,'corrections':[
 {'subject':'DELIVERY-MANIFEST.json schema.sha256','recorded':delivery['schema']['sha256'],'actual_schema_sha256':sha(schema),'actual_schema_bytes':len(schema),'current_TASK_B_and_packet_MANIFEST_agree':True,'disposition':'Stale delivery descriptor only; preserve original. Use TASK-B/current packet MANIFEST plus actual schema bytes for exact subject identity. Do not equate the old descriptor SHA with the actual schema. No review validity/adoption decision here.'},
 {'subject':'Review stated8312/544 lines','review_primary_line_claim':8312,'primary_LF_terminated_physical_lines':len(primary.splitlines()),'primary_LF_plus_empty_terminal_editor_line':primary.count(b'\n')+1,'review_schema_line_claim':544,'schema_LF_terminated_physical_lines':len(schema.splitlines()),'schema_LF_plus_empty_terminal_editor_line':schema.count(b'\n')+1,'disposition':'Both files end in LF. Claimed counts match newline-count+1 including a terminal empty editor line, not nonempty/splitlines physical row count. Byte identities/44body range coverage are exact; this count convention does not establish or refute cognitive reading.'},
 {'subject':'500104B whole input vs mandatory bodies','primary_bytes':500104,'sum44_raw_body_bytes':488685,'wrapper_bytes':11419,'disposition':'The500104B subject is whole PRIMARY, not summed body payload bytes.'}
]})
assert review['decision']['ordinary_vote']=='AGREE' and review['decision']['selected_alternative']=='A_EXPLICIT_COMPLETE_RECORD'
assert review['decision']['required_ordinary_changes']==[] and review['decision']['blocking_ordinary_findings']==[]
assert review['read_scope']['mandatory_body_ids_read']==[f'B{i:02}' for i in range(1,45)]
assert review['read_scope']['all_mandatory_bodies_read_completely'] and not review['read_scope']['partial_or_unread_bodies']
assert not review['read_scope']['optional_context_read']
assert review['reviewer']['actual_model'] is None and not receipt['actualBackendIdentityAttested']
dump('FIDELITY-RESULT.json',{
 'status':'ORDINARY_B_FIDELITY_CHECK_COMPLETE_WITH_SEPARATE_ANCHOR_AND_DESCRIPTOR_CORRECTIONS',
 'scope':'One unchanged whole B ordinary design opinion only; not a fresh design/security vote, pair agreement, adoption, execution authorization, or product acceptance.',
 'whole_opinion':{'source':str(ROOT/'job-b/TERMINAL-REVIEW.json'),'copy':'UNCHANGED-TERMINAL-REVIEW.json','bytes':len(review_bytes),'sha256':sha(review_bytes),'unchanged':True},
 'actual_vote':review['decision'],'finding_severities':{'minor':3,'note':2,'blocking':0},'dissent_or_abstention_recorded':False,
 'findings_and_qualifications_retained':True,'all44_exact_bodies_and_offsets_verified':True,'original35_exact_crosswalk_verified':True,'full_original_lifecycle_retained':True,'all12_owner_dependencies_mandatory_unadopted_in_this_opinion':True,
 'source_fidelity':'The cited material supports the five findings at PRIMARY coordinates. File-local anchor corrections, stronger supplemental anchors for numeric ceiling and v1 enum, stale delivery-schema SHA and line-count conventions are retained separately. Conditional owner questions are not transformed into chosen owner rules or observed failures.',
 'read_scope_claim_exact':review['read_scope'],'read_scope_evidence_limit':'B reports complete once-only nonoverlapping reads, including ContentOffset continuations. Public input custody and allowlisted transport metadata do not independently demonstrate cognition, actual tool read ranges or absence of all shell calls. No private capture was read. This fidelity lane mechanically hashes all44 full source bodies and semantically inspects design/lifecycle and each cited finding; it does not make its own complete semantic reading claim for every500104B line.',
 'requested_identity':review['reviewer'],'backend_identity_attested':False,'public_transport':{'status':receipt['terminalStatusLiteral'],'exit_code':receipt['terminalExitCode'],'source_review_correctness_established':receipt['sourceReviewCorrectnessEstablished'],'parent_reported_B_session':63840,'session_evidence_basis':'parent instruction only; no independently observable backend receipt inferred'},
 'unread_context':'A opinion, private captures,74 optional authored examples,27 optional L04 files, new implementation/test outputs were not read in this task.',
 'prior_exposure':'Custodian previously authored related financial/recovery examples and Core2 multi-condition/composition oracles and has related public DTO/codec/source exposure. No global blindness or preimplementation independence is claimed. This is an independent ordinary literal/source-fidelity pass over B, not new semantic oracle custody.',
 'parent_acceptance_closed':0,'product_predicates_run':0,'security_rules_authored':0,'new_votes':0,'pair_agreement_determined':False,'root_adoption_determined':False,
 'startup':'develop skill, K AGENTS/product contract and read-only moriarty-dev status restored. Historical SP01.6 blocked status preserved; no deployment/wallet operation. No Git, provider, product/runtime/build/native/SDK/network/private capture operation.'
})
dump('SOURCE-PINS.json',{'policy':'Only public B packet/transport, exact44 original public bodies and candidate manifest bytes; no A opinion or private captures. Startup source was read as required but is not a candidate claim source.','pins':pins})
(OUT/'README.md').write_text('''# Recovery B fidelity\n\nThis packet preserves the complete 25,735-byte B opinion unchanged. B selects `A_EXPLICIT_COMPLETE_RECORD` with ordinary `AGREE`, three minor owner-interface dependencies and two notes. It records no blocking ordinary finding, required ordinary revision, dissent or abstention. Its full lifecycle, original 35 MPLR and twelve mandatory open owner prerequisites remain retained obligations.\n\n`BODY-CUSTODY-CHECKS.json` verifies all 44 exact original body hashes and byte ranges in the 500,104-byte PRIMARY. `SOURCE-ANCHOR-CORRECTIONS.json` converts every numeric finding citation from PRIMARY coordinates into the named file's local coordinates, preserving exact source excerpts. `FINDING-FIDELITY.json` keeps the conditional Await-history decoder and 1 MiB-per-case versus 4 MiB-continuation concerns as owner dependencies, with no selected rule or reproduced failure.\n\n`CUSTODY-CORRECTIONS.json` retains the stale DELIVERY-MANIFEST schema digest, terminal-empty-line count convention and whole-primary/raw-body size distinction separately. The actual schema, TASK-B and current packet manifest agree. None of these corrections edits or replaces the original opinion.\n\nThe review's complete-read claim remains a self-report. Public transport reports SUCCESS/0 and explicitly supplies no attested backend identity or source correctness judgment. A opinion, private capture and optional examples remain unread. Prior related source/DTO/oracle exposure is disclosed.\n\nThis packet supplies no new vote, pair agreement, root adoption, security rule, runtime evidence or acceptance. All original financial/composition/target/private/native/live/ledger/release exits and all fourteen staged gaps stay open at their existing scopes.\n\nRun `python3 check_fidelity.py` for static source/hash/anchor preservation checks and `sha256sum --check --quiet FREEZE.sha256` for this payload inventory. These commands do not execute Moriarty.\n''')
print('Prepared',len(checks),'body checks;',len(anchors),'cited ranges;',len(extra),'supplemental anchors;',len(trace_checks),'original MPLR crosswalks')
