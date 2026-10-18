#!/usr/bin/env python3
"""Check retained source bytes, snapshot identity, local links and coverage."""
import hashlib,json,pathlib,re,subprocess,datetime
root=pathlib.Path(__file__).resolve().parent;failures=[];counts={}
inventory=json.loads((root/'SOURCE-INVENTORY.json').read_text())
for row in inventory['sources']:
    if row.get('status')!='downloaded':continue
    p=pathlib.Path(row['payload_path'])
    if row.get('commit'):
        actual=subprocess.check_output(['git','-C',str(p),'rev-parse','HEAD'],text=True).strip()
        status=subprocess.check_output(['git','-C',str(p),'status','--porcelain','--untracked-files=no'],text=True)
        if actual!=row['commit'] or status:failures.append({'source':str(p),'error':'snapshot commit/content changed','status':status[:500]})
    elif not p.exists() or hashlib.sha256(p.read_bytes()).hexdigest()!=row['sha256']:
        failures.append({'source':str(p),'error':'source hash mismatch'})
for folder in ['cardano-developer-docs','cardano-plutus-api']:
    p=root/folder/'MANIFEST.json'
    if not p.exists():failures.append({'source':str(p),'error':'crawl manifest missing'});continue
    doc=json.loads(p.read_text());stats=doc['summary'];rows=doc['pages']
    if not stats['link_graph_exhausted'] or stats['remaining_queue']!=0:failures.append({'source':str(p),'error':'crawl queue not exhausted'})
    for row in rows:
        if row.get('payload_path'):
            data=pathlib.Path(row['payload_path']).read_bytes()
            if hashlib.sha256(data).hexdigest()!=row['sha256']:failures.append({'source':row['requested_url'],'error':'documentation hash mismatch'})
    counts[folder]={'attempted':len(rows),'downloaded':sum(x['status']=='downloaded' for x in rows),'errors':sum(x['status']!='downloaded' for x in rows),'zero_text_successes':sum(x['status']=='downloaded' and x.get('extract_characters',0)==0 for x in rows),'short_successful_extracts':sum(x['status']=='downloaded' and x.get('extract_characters',0)<200 for x in rows),'link_graph_exhausted':stats['link_graph_exhausted']}
local=json.loads((root/'MORIARTY-EVIDENCE.json').read_text())
for row in local['files']:
    if hashlib.sha256(pathlib.Path(row['path']).read_bytes()).hexdigest()!=row['sha256']:failures.append({'source':row['path'],'error':'Moriarty baseline changed during research'})
reports=list(root.glob('[0-9][0-9]-*/REPORT.md'));counts['researcher_reports']=len(reports)
if len(reports)!=10:failures.append({'source':'research reports','error':'expected ten reports'})
for p in root.glob('[0-9][0-9]-*/sources.json'):json.loads(p.read_text())
for name in ['INDEX.md','ASSESSMENT.md','REFERENCE-APPS.md','DEVELOPER-LEARNING-GUIDE.md','PAPERS.md','SOURCE-INVENTORY.md','EXPERIMENTS.md','CLAIMS.md','LOCAL-ARCHITECTURE.md']:
    p=root/name
    if not p.exists():failures.append({'source':str(p),'error':'deliverable missing'});continue
    for href in re.findall(r'\[[^\]]*\]\(([^)]+)\)',p.read_text()):
        if href.startswith(('http:','https:','mailto:','#')):continue
        target=href.split('#')[0].strip('<>')
        if target and not (p.parent/target).exists():failures.append({'source':str(p),'error':'broken local link','target':target})
result={'verified_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'pass' if not failures else 'fail','counts':counts,'source_acquisition_summary':inventory['summary'],'failures':failures,'limits':['Integrity/existence/coverage checks only; not code correctness, cryptographic soundness, source independence or ledger acceptance.','Scope exhaustion does not prove unlinked/external/historical documentation completeness.']}
(root/'COLLECTION-VERIFICATION.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'status':result['status'],'counts':counts,'failures':failures},indent=2));raise SystemExit(1 if failures else 0)
