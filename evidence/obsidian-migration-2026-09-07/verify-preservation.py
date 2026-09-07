"""Compare adopted notes with the published cleanup tree, without altering notes."""
from pathlib import Path
import csv, datetime, difflib, hashlib, io, json, re, subprocess
ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
BASE = '8df38d25f6863e90bb7e16b691b578231c62458d'
FRAGMENTS = [
 '#clm-0195-intents-report-separates-authority-plans-and-receipts',
 '#clm-0187-user-reset-after-verification-detour',
 '#clm-0186--checkpoint-after-the-a5-view-typecheck',
 '#source-discrepancies-and-dispositions',
]
INDEX_ADDITION = '[[wiki/overview|Vault overview]] · [[wiki/workflow|Research workflow]] · [[wiki/canvases/moriarty|Language map]] · [[wiki/meta/provenance|Provenance mapping]]\n\n'
LOG_HEADING = '\n## 2026-09-07: Adopt the research workflow as an Obsidian vault\n'
def git(*args): return subprocess.check_output(['git', *args], cwd=ROOT)
def digest(value): return hashlib.sha256(value).hexdigest()
def split(text):
 match = re.match(r'---\n(.*?)\n---\n', text, re.S)
 return (match.group(1), text[match.end():]) if match else ('', text)
records = []; diffs = []; all_ids = set()
for path in git('ls-tree', '-r', '--name-only', BASE, 'wiki/').decode().splitlines():
 if not path.endswith('.md'): continue
 before = git('show', BASE + ':' + path); after = (ROOT/path).read_bytes()
 old_meta, old_body = split(before.decode()); new_meta, new_body = split(after.decode())
 expected = old_body
 repairs = []
 for fragment in FRAGMENTS:
  if fragment in expected:
   repairs.append({'removed_fragment':fragment, 'occurrences':expected.count(fragment), 'disposition':'Link retains its same target note and visible label.'})
   expected = expected.replace(fragment, '')
 for repair in json.loads((OUT/'portable-link-repairs.json').read_text())['repairs']:
  if repair['page']==path:
   expected=expected.replace(repair['old'],repair['new']); repairs.append(repair)
 compared = new_body
 if path == 'wiki/index.md':
  assert compared.count(INDEX_ADDITION) == 1
  compared = compared.replace(INDEX_ADDITION, '', 1)
 if path == 'wiki/log.md':
  assert compared.count(LOG_HEADING) == 1
  compared = compared.split(LOG_HEADING, 1)[0]
 # Leading/trailing blank lines around adopted frontmatter are formatting only.
 body_ok = expected.strip('\n') == compared.strip('\n')
 # Every original frontmatter line remains in order. Only new Obsidian properties are added.
 old_lines = old_meta.splitlines(); new_lines = iter(new_meta.splitlines())
 metadata_ok = all(any(line == candidate for candidate in new_lines) for line in old_lines)
 def properties(meta):
  result={}; key=None
  for line in meta.splitlines():
   match=re.match(r'^([A-Za-z_][A-Za-z0-9_-]*):',line)
   if match:
    key=match.group(1); assert key not in result; result[key]=[line]
   elif key is not None: result[key].append(line)
  return {key:'\n'.join(value).strip() for key,value in result.items()}
 old_properties=properties(old_meta); new_properties=properties(new_meta)
 metadata_ok = metadata_ok and all(new_properties.get(key)==value for key,value in old_properties.items())
 ids = set(re.findall(r'\bCLM-\d{4}\b', old_body)); all_ids |= ids
 assert ids <= set(re.findall(r'\bCLM-\d{4}\b', new_body))
 assert body_ok and metadata_ok, path
 records.append({'path':path,'baseline_sha256':digest(before),'current_sha256':digest(after),'baseline_body_sha256':digest(old_body.encode()),'current_body_sha256':digest(new_body.encode()),'body_equal_after_enumerated_navigation_changes':body_ok,'original_frontmatter_preserved_in_order':metadata_ok,'link_repairs':repairs,'addition':'vault navigation' if path=='wiki/index.md' else 'migration log entry' if path=='wiki/log.md' else None})
 diffs.extend(difflib.unified_diff(old_body.splitlines(True),new_body.splitlines(True),fromfile=BASE+'/'+path,tofile='current/'+path))
rows=list(csv.DictReader((ROOT/'evidence/source-inventory.csv').open()))
baseline_rows=list(csv.DictReader(io.StringIO(git('show',BASE+':evidence/source-inventory.csv').decode())))
assert rows[:len(baseline_rows)]==baseline_rows
ledger=json.loads((ROOT/'wiki/meta/ledgers/source-ledger.json').read_text())
imported=[row for record in ledger['sources'].values() for row in record['legacy_records']]
assert sorted(rows,key=lambda x:x['source_id'])==sorted(imported,key=lambda x:x['source_id'])
index=json.loads((ROOT/'wiki/meta/legacy-claim-index.json').read_text())['claims']; assert set(index)==all_ids
assert not git('diff','--name-only','--diff-filter=MDTR',BASE,'--','raw/').strip()
assert not git('diff',BASE,'--','ROADMAP.md').strip()
canvas=json.loads((ROOT/'wiki/canvases/moriarty.canvas').read_text());ids={n['id'] for n in canvas['nodes']};assert len(ids)==len(canvas['nodes'])
assert all((ROOT/n['file']).is_file() and not Path(n['file']).is_absolute() and '..' not in Path(n['file']).parts for n in canvas['nodes'])
assert all(e['fromNode'] in ids and e['toNode'] in ids for e in canvas['edges'])
result={'observed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':'python3 evidence/obsidian-migration-2026-09-07/verify-preservation.py','baseline_commit':BASE,'passed':True,'legacy_notes':len(records),'legacy_claim_ids':len(all_ids),'all_original_frontmatter_preserved':True,'legacy_body_changes':'Only the exact fragment and portable source-link repairs, index navigation and migration log addition enumerated per note; surrounding blank-line differences ignored. No Humanizer rewrite of legacy note bodies.','source_rows':len(rows),'portable_records':len(ledger['sources']),'source_rows_preserved_exactly':True,'shared_portable_identities':[{'portable_id':key,'legacy_ids':[x['source_id'] for x in value['legacy_records']]} for key,value in ledger['sources'].items() if len(value['legacy_records'])>1],'raw_unchanged':True,'roadmap_unchanged':True,'canvas_valid':True,'notes':records}
(OUT/'preservation.json').write_text(json.dumps(result,indent=2)+'\n');(OUT/'legacy-note-bodies.diff').write_text(''.join(diffs));print(json.dumps({k:v for k,v in result.items() if k!='notes'},indent=2))
