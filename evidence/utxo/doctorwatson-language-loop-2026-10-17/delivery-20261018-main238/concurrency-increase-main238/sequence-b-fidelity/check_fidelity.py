"""Offline ordinary source/quote fidelity only. No provider/product/private IO."""
from pathlib import Path
from collections import Counter
import json,hashlib,re
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
P=R/'completed-successor-two-design-reviews-20261018/packet'
J=R/'completed-successor-inline-design-reviews-20261018'
O=Path(__file__).resolve().parent
EXPECTED_PRIMARY='243a2e1eef296ac3e48f47c618b7cbee23eaf330a8840aec73c6a4736e9d7b2c'
def sha(b):return hashlib.sha256(b).hexdigest()
def put(n,v):(O/n).write_text(json.dumps(v,sort_keys=True,indent=2,ensure_ascii=True)+'\n')
def atline(s,q):
 a=s.find(q);return None if a<0 else s[:a].count('\n')+1
review_bytes=(J/'job-b/TERMINAL-REVIEW.json').read_bytes();review=json.loads(review_bytes)
primary_bytes=(P/'PRIMARY.md').read_bytes();primary=primary_bytes.decode()
task=json.loads((J/'TASK-B.json').read_text());schema=json.loads((J/'SCHEMA-B.json').read_text())
receipt_bytes=(J/'job-b/TERMINAL-EXPORT-RECEIPT.json').read_bytes();receipt=json.loads(receipt_bytes)
assert sha(primary_bytes)==EXPECTED_PRIMARY==review['primarySha256']==schema['properties']['primarySha256']['const']
assert sha((P/'MANIFEST.json').read_bytes())==task['manifestSha256']
assert sha((J/'PROMPT-B.md').read_bytes())==task['promptSha256']
assert primary_bytes in (J/'PROMPT-B.md').read_bytes()
assert sha((J/'SCHEMA-B.json').read_bytes())==task['schemaSha256']
assert review['reviewToken']==task['reviewToken']==schema['properties']['reviewToken']['const']
assert sha((J/'TASK-B.json').read_bytes())==receipt['taskSha256']
assert receipt['id']==task['id']
assert receipt['terminalStatusLiteral']=='SUCCESS' and receipt['terminalExitCode']==0 and receipt['resultEvents']==1
assert receipt['sourceReviewCorrectnessEstablished'] is False
assert review['reviewer']['actualBackendIdentityAttested'] is False and receipt['actualBackendIdentityAttested'] is False
assert set(review['decisions'])=={f'SC{i}' for i in range(1,7)}
assert set(review['owner_dependencies'])=={f'OD{i}' for i in range(1,9)}
assert all(v['vote']=='agree' and len(v['reason'])>=20 and len(v['evidence'])>=1 for v in review['decisions'].values())
assert review['verdict']=='recommend-B-conditional'
# Preserve the whole object byte-for-byte; no filtered rewrite is substituted.
(O/'TERMINAL-REVIEW.original.json').write_bytes(review_bytes)
(O/'TERMINAL-EXPORT-RECEIPT.original.json').write_bytes(receipt_bytes)
assert (O/'TERMINAL-REVIEW.original.json').read_bytes()==review_bytes
# Public packet hash integrity only; semantic reading is separately limited.
manifest=json.loads((P/'MANIFEST.json').read_text());manifest_pins=[]
for entry in manifest['files']:
 rel=Path(entry['path']);assert not rel.is_absolute() and '..' not in rel.parts
 assert not rel.name.endswith(('.private','.private.jsonl','.private.log')) and rel.name not in {'provider.private.jsonl','agy.private.log'}
 f=P/rel;b=f.read_bytes();assert sha(b)==entry['sha256'] and len(b)==entry['bytes'],str(rel)
 manifest_pins.append({'path':str(rel),'bytes':len(b),'sha256':sha(b)})
put('PACKET-HASH-CHECKS.json',{'scope':'Hash integrity of public packet members; NOT a claim all physical bodies were read by reviewer or this fidelity lane','manifest_sha256':task['manifestSha256'],'members':manifest_pins})
quote_rows=[]
subjects=[(sc,d['evidence']) for sc,d in review['decisions'].items()]+[(f'finding-{n+1}',f['evidence']) for n,f in enumerate(review['findings'])]
for subject,evs in subjects:
 for i,e in enumerate(evs):
  path=P/e['path'];b=path.read_bytes();s=b.decode();q=e['quote']
  a=s.find(q);z=primary.find(q);assert a>=0 and z>=0,(subject,e['path'])
  encoded=json.dumps(q,ensure_ascii=True);rl=atline(review_bytes.decode(),encoded)
  if rl is None:rl=atline(review_bytes.decode(),json.dumps(q,ensure_ascii=False))
  assert rl is not None
  quote_rows.append({'subject':subject,'evidence_index':i,'path':e['path'],'source_sha256':sha(b),'source_start_line':s[:a].count('\n')+1,'source_end_line':s[:a+len(q)].count('\n')+1,
   'primary_start_line':primary[:z].count('\n')+1,'primary_end_line':primary[:z+len(q)].count('\n')+1,'review_encoded_quote_line':rl,'quote_utf8_sha256':sha(q.encode()),'literal_quote':q,'status':'EXACT; no whitespace/text normalization'})
assert len(quote_rows)==20
put('QUOTE-FIDELITY.json',{'primary_sha256':EXPECTED_PRIMARY,'quotes_checked':20,'all_exact_in_cited_source_and_primary':True,'rows':quote_rows})
# Complete original required-behavior fields, not only35 identifiers.
original=json.loads((P/'ORIGINAL-MPLR-001-035.json').read_text());assert [x['id'] for x in original]==[f'MPLR-{i:03}' for i in range(1,36)]
lineage=(P/'RETAINED-LINEAGE.md').read_text();mplr=[]
for x in original:
 f=P/'basis'/x['source_path'];b=f.read_bytes();literal=x['verbatim_required_behavior'];head='## Required behavior\n\n'
 assert sha(b)==x['source_sha256'] and literal in b.decode() and literal.startswith(head)
 body=literal[len(head):].rstrip('\n');needle=x['id']+'\n'+body
 assert needle in primary
 row=re.search(r'^\| '+x['id'][5:]+r'\s.*$',lineage,re.M);assert row is not None
 mplr.append({'id':x['id'],'source_path':'basis/'+x['source_path'],'source_sha256':sha(b),'source_required_behavior_line':atline(b.decode(),literal),'verbatim_required_behavior_sha256':sha(literal.encode()),'primary_start_line':atline(primary,needle),'primary_end_line':atline(primary,needle)+needle.count('\n'),'lineage_line':lineage[:row.start()].count('\n')+1,'literal_lineage_row':row.group(),'status':'complete required behavior and source note hash preserved'})
put('MPLR-RETENTION.json',{'count':35,'original_record_sha256':sha((P/'ORIGINAL-MPLR-001-035.json').read_bytes()),'rows':mplr,'limit':'Retention is not fulfillment, owner acceptance or proof/ledger closure'})
choices=json.loads((P/'DESIGN-CHOICES.json').read_text())
notes={
 'SC1':'Addresses actual stateless full CompletePrefix A versus owned Sequence B; does not mistake A for a bare existing Complete. Reason identifies full carrier/reopen cost and owned next() boundary.',
 'SC2':'Retains differing schemas, shared exact financial domains, explicit mapping, all selected-asset account rows and outside frame; no caller-built successor state or R7 bypass is recommended.',
 'SC3':'Retains original complete call/segment/Step/value/frame/capture/continuation/effect/work data and global attachments; false observations remain lossless null-segment records.',
 'SC4':'Endorses one proposed production compose loader/driver shared by test consumer, with preloaded expectations excluded from runtime state/accounting; not a claim implementation exists.',
 'SC5':'Addresses full gross/fee/duty-presence/effect classification and per-call attempted versus per-instance latest committed accounting. Explicit resource/closure enforcement remains OD4.',
 'SC6':'Explicitly retains all35 MPLR, allfive operators, existing Pair/DTO/profile behavior and separate native/private/persistent/target/owner obligations; no parent closure is supplied.'}
sc=[]
for ch in choices['choices']:
 sid=ch['id'];d=review['decisions'][sid]
 sc.append({'id':sid,'actual_vote':d['vote'],'proposal_exact_recommendation':ch['exact_recommendation'],'review_reason_sha256':sha(d['reason'].encode()),'review_reason_line':atline(review_bytes.decode(),json.dumps(d['reason'],ensure_ascii=True)),'ordinary_fidelity_assessment':notes[sid],'new_design_vote_from_this_lane':False})
put('SC1-SC6-FIDELITY.json',{'all_six_present_and_substantive':True,'rows':sc,'scope':'Ordinary opinion-to-proposal fidelity, not agreement with/adoption of the design'})
owner=(P/'OWNER-DEPENDENCIES.md').read_text();ods=[]
for i in range(1,9):
 key=f'OD{i}';match=re.search(r'^\| '+key+r'\s.*$',owner,re.M);assert match
 dep=review['owner_dependencies'][key];assert dep.startswith('Required ')
 ods.append({'id':key,'review_dependency_literal':dep,'review_dependency_sha256':sha(dep.encode()),'primary_owner_table_line':atline(primary,match.group()),'owner_document_line':owner[:match.start()].count('\n')+1,
 'status_in_reviewed_packet':'mandatory separate designated-owner dependency; UNADOPTED/unsupplied by ordinary proposal and this fidelity check','this_lane_owner_selection_or_judgment':False})
put('OD1-OD8-RETENTION.json',{'rows':ods,'applicability_warning':'Do not replace scoped OD5 authority/OD6 native/OD7 persistent/OD8 nonserial conditions with a new blanket maintainer permission gate. Detailed dependencies and applicable owner contracts retain their original scope.'})
sourcepins=json.loads((P/'SOURCE-PINS.json').read_text())
put('FIDELITY-RESULT.json',{
 'result':'ordinary text/hash/line fidelity verified, with qualifications retained; no new design or security vote',
 'review':{'path':str(J/'job-b/TERMINAL-REVIEW.json'),'sha256':sha(review_bytes),'bytes':len(review_bytes),'copy':'TERMINAL-REVIEW.original.json','reviewToken':review['reviewToken'],'verdict':review['verdict'],'sc_votes':{k:v['vote'] for k,v in review['decisions'].items()},'findings_count':len(review['findings']),'finding_severities':dict(Counter(f['severity'] for f in review['findings']))},
 'identity_and_transport':{'parent_reported_session':1246,'public_receipt_status':receipt['terminalStatusLiteral'],'public_receipt_exit':receipt['terminalExitCode'],'requested_model':review['reviewer']['requestedModel'],'returned_model_label':review['reviewer']['returnedModel'],'effort':review['reviewer']['effort'],'independent_reviewer_self_report':review['reviewer']['independent'],'backend_identity_attested':False,'receipt_source_correctness_established':False,'no_private_capture_reread':True},
 'primary':{'path':str(P/'PRIMARY.md'),'sha256':EXPECTED_PRIMARY,'bytes':len(primary_bytes),'lines':len(primary.splitlines()),'literal_whole_primary_in_pinned_public_prompt':True,'proposal_status':'UNADOPTED source-only; this lane records no adoption'},
 'input_pins':{'manifest_sha256':task['manifestSha256'],'public_prompt_sha256':task['promptSha256'],'schema_sha256':task['schemaSha256'],'task_sha256':sha((J/'TASK-B.json').read_bytes()),'public_export_receipt_sha256':sha(receipt_bytes),'basis_commit':sourcepins['commit'],'basis_tree':sourcepins['tree']},
 'literal_checks':{'cited_quotes':20,'exact_quotes':20,'sc_assessments':6,'original_MPLR_complete_fields':35,'retained_owner_dependencies':8,'public_manifest_members_hash_checked':len(manifest_pins)},
 'dissent_and_conditions':'No SC revise/abstain votes. Conditional verdict and all four findings (three minor,one note), exact nextStep requests and all owner/oracle limitations remain in the unchanged whole opinion; not an unconditional approval.',
 'qualification_caveats':[
  'Summary compresses OD1-OD8 into before-production language, while detailed OD5-OD8 and primary owner table scope obligations by authority/native/persistent/nonserial applicability. Retain detailed scope; no new blanket gate or owner rule is inferred.',
  'SC5 uses sound/faithful and satisfies MPLR005/017/025 phrasing inside an explicitly ordinary accounting vote. Do not promote it to complete authenticated/MPLR/security/resource/native acceptance; original35 full clauses and explicit limits remain operative.',
  'ReadScope16 entries refer to inline primary subjects/excerpts. Reviewer expressly leaves physical71-file baseline beyond inline excerpts and optional worked arithmetic/fixtures unread; no whole-physical-file audit or independent financial/byte oracle is certified.',
  'Recommendations concerning injectivity, incoming-duty admission, resource bounds and resume cap/error handling are reviewer requests directed to OD owners, not selected rules from this fidelity lane.'
 ],
 'root_only_decisions':['adoption','agreement with the other review','source vote eligibility/acceptance','applicable owner-contract adoption'],
 'non_actions':'No provider/private capture/product/runtime/build/Git/network/shared writes; no new owner/security authorship or hostile test; no existing opinion edit',
 'scope_preserved':'All35 original MPLR/five operators/148requirements/292scenarios/20families and all U/M/MC/SP/G/financial/native/private/persistent/ledger/target acceptance remain unchanged'})
print(json.dumps({'review_sha256':sha(review_bytes),'primary_sha256':EXPECTED_PRIMARY,'manifest_members_verified':len(manifest_pins),'literal_quotes':len(quote_rows),'MPLR_fields':len(mplr),'verdict':review['verdict'],'new_vote':False},sort_keys=True))
