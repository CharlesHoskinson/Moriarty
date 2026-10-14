#!/usr/bin/env python3
"""Consolidate immutable worker receipts and check retained payloads."""
import collections, datetime, hashlib, json, pathlib, subprocess

root=pathlib.Path(__file__).resolve().parent
records=[]; metadata={}; errors=[]
for p in sorted(root.glob('*/sources.json')):
    d=json.loads(p.read_text())
    rows=d if isinstance(d,list) else d.get('sources',d.get('source_receipts',[]))
    for row in rows:
        for key in ['artifact_receipt','receipt','receipt_path']:
            val=row.get(key)
            if val:
                q=pathlib.Path(val)
                if not q.is_absolute():q=p.parent/q
                metadata[str(q.resolve())]=row
        if not any(row.get(k) for k in ['artifact_receipt','receipt','receipt_path']):
            for q in p.parent.glob('*.receipt.json'):
                receipt=json.loads(q.read_text())
                if receipt.get('requested_url')==row.get('requested_url',row.get('url')):
                    metadata[str(q.resolve())]=row
for p in sorted(root.glob('*/*.receipt.json')):
    row=json.loads(p.read_text()); row['receipt_path']=str(p); row['lane']=p.parent.name
    row['bibliography']=metadata.get(str(p.resolve()),{})
    payload=row.get('payload_path'); outcome='not-applicable'
    if row.get('status')=='downloaded' and payload:
        q=pathlib.Path(payload)
        if row.get('method')=='git shallow clone':
            try:
                commit=subprocess.check_output(['git','-C',str(q),'rev-parse','HEAD'],text=True).strip()
                outcome='pass' if commit==row.get('commit') else 'commit-mismatch'
            except Exception:outcome='missing-repository'
        elif q.is_file():
            outcome='pass' if hashlib.sha256(q.read_bytes()).hexdigest()==row.get('sha256') else 'hash-mismatch'
            if q.suffix=='.pdf' and not q.read_bytes().startswith(b'%PDF-'):outcome='invalid-pdf'
        else:outcome='missing-payload'
        if outcome!='pass':errors.append({'receipt':str(p),'failure':outcome})
    row['integrity_check']=outcome;records.append(row)
urls=collections.defaultdict(list);repos=collections.defaultdict(list); content=collections.defaultdict(list)
for row in records:
    if row.get('status')!='downloaded':continue
    urls[row.get('canonical_url',row['requested_url'])].append(row['receipt_path'])
    if row.get('commit'):repos[(row.get('remote',row['requested_url']),row['commit'])].append(row['receipt_path'])
    elif row.get('sha256'):content[row['sha256']].append(row['receipt_path'])
summary={'generated_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),
         'total_receipts':len(records),'statuses':dict(collections.Counter(r.get('status','unknown') for r in records)),
         'successful_http_sources':sum(r.get('status')=='downloaded' and not r.get('commit') for r in records),
         'successful_repository_snapshots':sum(r.get('status')=='downloaded' and bool(r.get('commit')) for r in records),
         'unique_repository_commits':len(repos),'pdfs':sum(r.get('status')=='downloaded' and str(r.get('payload_path','')).endswith('.pdf') for r in records),
         'unique_successful_urls':len(urls),'duplicate_urls':{k:v for k,v in urls.items() if len(v)>1},
         'duplicate_repository_commits':[{'remote':k[0],'commit':k[1],'receipts':v} for k,v in repos.items() if len(v)>1],
         'duplicate_payload_hashes':{k:v for k,v in content.items() if len(v)>1},
         'worker_reports':sorted(str(p.relative_to(root)) for p in root.glob('[0-9][0-9]-*/REPORT.md')),
         'integrity_failures':errors,'limitations':['Source capture is not complete source review.','PDF count includes historical specifications and original research, not only papers.','Mirrors, duplicates and shared institutional sources are not independent corroboration.']}
(root/'SOURCE-INVENTORY.json').write_text(json.dumps({'summary':summary,'sources':records},indent=2)+'\n')
lines=['# Retained source inventory','',f"{len(records)} acquisition receipts. Detailed metadata and deduplication are in SOURCE-INVENTORY.json.",'','| Lane | Kind | Status | Source | Commit / content SHA-256 | Receipt |','|---|---|---|---|---|---|']
for r in records:
    b=r['bibliography'];title=str(b.get('title',pathlib.Path(r['receipt_path']).name.removesuffix('.receipt.json'))).replace('|','/')
    url=r['requested_url']; digest=r.get('commit') or r.get('sha256','—')
    lines.append(f"| {r['lane']} | {'repo' if r.get('commit') else 'PDF' if str(r.get('payload_path','')).endswith('.pdf') else 'web/text'} | {r.get('status')} | [{title}]({url}) | `{digest}` | [{pathlib.Path(r['receipt_path']).name}]({pathlib.Path(r['receipt_path']).relative_to(root)}) |")
(root/'SOURCE-INVENTORY.md').write_text('\n'.join(lines)+'\n')
(root/'VERIFICATION.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if not k.startswith('duplicate')},indent=2))
raise SystemExit(1 if errors else 0)
