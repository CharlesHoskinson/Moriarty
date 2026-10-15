#!/usr/bin/env python3
"""Adopt explicit unknown-digest omission metadata into a finalized generation; reads manifests only, never source bodies."""
import argparse,hashlib,json,os,tempfile
from pathlib import Path
MANIFESTS=['paths.jsonl','exclusions.jsonl','objects.jsonl','public-log-extractions.jsonl']
LEGACY={'source-changed-since-recorded-digest','source-unavailable'}
RECEIPT='OMISSION-METADATA-ADOPTION.json'
def sha(b):return hashlib.sha256(b).hexdigest()
def plain(path):
 p=Path(os.path.abspath(path))
 for c in [p,*p.parents]:
  if c.is_symlink():raise ValueError('symlink or symlink parent refused')
 return p
def replace(p,data):
 mode=p.stat().st_mode&0o777 if p.exists() else 0o644;tmp=None
 try:
  with tempfile.NamedTemporaryFile(dir=p.parent,prefix='.omission-adoption-',delete=False) as f:tmp=Path(f.name);f.write(data);f.flush();os.fsync(f.fileno())
  os.chmod(tmp,mode);plain(p);os.replace(tmp,p);tmp=None
 finally:
  if tmp is not None:tmp.unlink(missing_ok=True)
def adopt(archive,gate_sha256,expect_rows,reconstruct,reconstruct_sha256):
 h=plain(archive);gate_bytes=plain(h/'FINALIZATION-GATE.json').read_bytes()
 if sha(gate_bytes)!=gate_sha256:raise ValueError('finalization gate differs from the pinned generation')
 gate=json.loads(gate_bytes);attestation=plain(h/'FILTER-ATTESTATION.json').read_bytes()
 if gate.get('input_filter_attestation_sha256')!=sha(attestation):raise ValueError('filter attestation differs from finalization gate')
 if set(gate.get('final_manifest_sha256',{}))!=set(MANIFESTS):raise ValueError('incomplete finalization gate')
 data={name:plain(h/name).read_bytes() for name in MANIFESTS}
 if any(sha(data[name])!=gate['final_manifest_sha256'][name] for name in MANIFESTS):raise ValueError('manifest differs from finalization gate')
 if plain(h/RECEIPT).exists():raise ValueError('omission metadata already adopted')
 tool=plain(reconstruct).read_bytes()
 if sha(tool)!=reconstruct_sha256:raise ValueError('reconstructor differs from pinned reviewed helper')
 origins={}
 for line in data['exclusions.jsonl'].splitlines():
  r=json.loads(line);origins.setdefault((r.get('root'),r.get('path')),[]).append(r)
 lines=data['public-log-extractions.jsonl'].splitlines(keepends=True);changes=[]
 for i,line in enumerate(lines):
  r=json.loads(line)
  if r.get('sha256') is not None:continue
  if not line.endswith(b'\n') or r.get('status') not in LEGACY or 'public_sha256' not in r or r['public_sha256'] is not None or 'public_bytes' in r or not isinstance(r.get('reason'),str) or not r['reason']:raise ValueError('unknown-digest row is not a payload-free legacy omission')
  # Only a never-inventoried exclusion origin may become an explicit unknown digest.
  origin=origins.get((r.get('root'),r.get('path')),[])
  if len(origin)!=1 or origin[0].get('sha256') is not None or origin[0].get('reason')!=r['reason']:raise ValueError('omission has a recorded digest or no unique exclusion origin')
  after=json.dumps({**r,'sha256':None,'status':'source-uninventoried','status_before_metadata_adoption':r['status']},sort_keys=True)
  changes.append({'line':i+1,'before':line[:-1].decode(),'after':after});lines[i]=after.encode()+b'\n'
 if len(changes)!=expect_rows:raise ValueError('unexpected unknown-digest row count %d'%len(changes))
 adopted=b''.join(lines);old_tool=plain(h/'reconstruct.py')
 receipt={'schema':'moriarty-utxo-omission-metadata-adoption/1','scope':'metadata-only explicit unknown source digests; no source body read, no reclassification','finalization_gate_sha256':gate_sha256,'filter_attestation_sha256':sha(attestation),'policy_sha256':gate.get('policy_sha256'),'original_attestations_retained_unchanged':True,'original_attestations_cover_adopted_manifest':False,'manifest':'public-log-extractions.jsonl','manifest_sha256_before':sha(data['public-log-extractions.jsonl']),'manifest_sha256_after':sha(adopted),'manifest_line_count':len(lines),'changed_lines':changes,'unchanged_manifest_sha256':{name:sha(data[name]) for name in MANIFESTS if name!='public-log-extractions.jsonl'},'adoption_tool_sha256':sha(Path(__file__).read_bytes()),'reconstruct_sha256_before':sha(old_tool.read_bytes()) if old_tool.exists() else None,'reconstruct_sha256_after':reconstruct_sha256,'source_bodies_read':False,'publication_exception':False,'source_approval':False,'native_approval':False}
 # Receipt first: an interrupted adoption leaves a receipt whose after-pins visibly disagree, and reruns refuse.
 with (h/RECEIPT).open('x') as f:f.write(json.dumps(receipt,sort_keys=True,indent=2)+'\n')
 replace(h/'public-log-extractions.jsonl',adopted);replace(old_tool,tool)
 if sha((h/'public-log-extractions.jsonl').read_bytes())!=receipt['manifest_sha256_after'] or sha(old_tool.read_bytes())!=reconstruct_sha256:raise ValueError('adopted bytes differ from receipt')
 return receipt
def main():
 p=argparse.ArgumentParser();p.add_argument('--archive',type=Path,required=True);p.add_argument('--gate-sha256',required=True);p.add_argument('--expect-rows',type=int,required=True);p.add_argument('--reconstruct',type=Path,required=True);p.add_argument('--reconstruct-sha256',required=True);a=p.parse_args()
 r=adopt(a.archive,a.gate_sha256,a.expect_rows,a.reconstruct,a.reconstruct_sha256)
 print(json.dumps({k:r[k] for k in ['finalization_gate_sha256','manifest_sha256_before','manifest_sha256_after','reconstruct_sha256_before','reconstruct_sha256_after']}|{'changed_lines':[c['line'] for c in r['changed_lines']]},sort_keys=True))
if __name__=='__main__':main()
