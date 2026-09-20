from pathlib import Path
import json,hashlib,datetime
R=Path(__file__).parent;V=Path('/home/charl/Moriarty-aeon-study');D=V/'deliverables/near-teardown-2026-09-19'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
expected={}
for name in ['ingest-near-teardown','repair-near-portable-links']:
 bundle=read(R/(name+'.json'))
 for w in bundle['writes']:
  expected[w['path']]=hashlib.sha256(Path(w['content_file']).read_bytes() if 'content_file' in w else w['content'].encode()).hexdigest()
receipt=read(R/'ingest-near-teardown-result.json');assert receipt['status']=='complete'
assert read(R/'repair-near-result.json')['status']=='complete'
assert all(sha(V/p)==h for p,h in expected.items())
base=read(R/'wiki-lint.json');final=read(R/'wiki-lint-final.json')
categories=['dead_links','provenance_errors','orphans','missing_frontmatter','configuration_errors','read_errors','ambiguous_targets','stale_index_entries']
new={k:[x for x in final[k] if x not in base[k]] for k in categories};assert not any(new.values()),new
sources=read(V/'wiki/meta/ledgers/source-ledger.json')['sources'];captures=[]
for sid,s in sources.items():
 cap=s.get('captured_payload','')
 if cap.startswith('.raw/captured/near-teardown-2026-09-19/'):
  assert sha(V/cap)==s['content_sha256'];captures.append(sid)
cons=read(R/'panel/FINAL-CONSENSUS.json');assert sha(R/'panel'/cons['candidate'])==cons['candidate_sha256']
for vote in cons['votes']:assert sha(R/'panel'/vote['file'])==vote['sha256'] and vote['verdict']=='ENDORSE'
review=read(R/'studies/astra-intents/spec-review.json')
assert all(sha(Path(x['path']))==x['sha256'] for x in review['exact_hashes'])
assert len(list((V/'wiki/research/near').glob('*.md')))==12
assert len(list((V/'wiki/research/mplr').glob('MPLR-*.md')))==18
spec=(V/'openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md').read_text()
report={'verified_at':datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),'status':'pass','operation_id':receipt['operation_id'],'original_writes':175,'final_paths_verified':len(expected),'captured_sources_hash_verified':len(captures),'near_notes':12,'mplrs':18,'consensus_votes_hash_verified':len(cons['votes']),'reviewed_files_hash_verified':len(review['exact_hashes']),'new_lint_findings':new,'existing_lint_summary':final['summary'],'limits':['Research and documentation verification only.','Whole-vault historic broken links remain; NEAR introduces none.','No target implementation or proof completion.']}
(R/'ingestion-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items() if k not in ['existing_lint_summary','new_lint_findings']},indent=2))
