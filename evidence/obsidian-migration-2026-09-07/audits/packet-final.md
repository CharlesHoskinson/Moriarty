Independent FINAL review of the Obsidian migration. Use only supplied evidence, no filesystem tools are needed. User explicitly authorizes the migration using AgriciDaniel/claude-obsidian after cleanup and GitHub publication. Cleanup has already been published. The requested exact reviewer route is Fable 5.1 medium and fresh GPT-6 high; this review accepts only the vault maintenance, not research/proof/financial correctness. Return JSON using the output schema.

The prior candidate needed stronger proof that legacy note bodies were preserved. The complete source of the deterministic comparison, baseline/current hashes for all 33 notes and a full body-only unified diff follow. The checker compares current notes against published cleanup commit 8df38d25f6863e90bb7e16b691b578231c62458d, preserving original frontmatter properties/values and original source rows exactly. It allows only the enumerated old-to-new links, added index navigation and appended migration log entry; no legacy body was humanized. The per-note baseline_sha256 and current_sha256 fields disambiguate the hashes. All 620 claim IDs remain; no automatic claim extraction or acceptance occurred. The 78 rows map to 76 records because SRC-0022, SRC-0028 and SRC-0031 share one content-derived identity; all three original rows and ID mappings remain intact.

Other corrections: removed the redundant optional wiki/canvases/index.md catalog under existing reversible repository-cleanup authority, retaining its bytes in a local backup and in catalog-cleanup.json. The core's create/replace-only API was not falsely described as supporting deletion. Canvas remains linked from the canonical wiki index. New note updated_at metadata, source backlink lists and new inventory vocabulary were corrected; both ingest documents now consistently use .raw/captured and require the SRC inventory/mapping together. Obsidian writes Markdown links, does not auto-rewrite links on rename, and shows orphans. ALL strict lint categories now zero; no allowlists or suppressions.

Fresh staged checkout independently caught 24 links that depended on ignored local directories. The Agda links now use the same full commit already cited in the unchanged claim text, verified through GitHub's tree API. The small historical decision graph is retained byte-identically inside the wiki. These exact navigation replacements appear in the body diff and repair receipt. Doctor and strict lint now pass from a clean directory populated only with staged files, without ignored runtime/repo/graph files. Desktop GUI interaction remains unverified and the guide instructs the user to Open folder as vault. Source/runtime inventory records and the complete ROADMAP remain scoped as before; current language/package/proof acceptance unchanged.

### evidence/obsidian-migration-2026-09-07/verify-preservation.py
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
result={'observed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':'python3 evidence/obsidian-migration-2026-09-07/verify-preservation.py','baseline_commit':BASE,'passed':True,'legacy_notes':len(records),'legacy_claim_ids':len(all_ids),'all_original_frontmatter_preserved':True,'legacy_body_changes':'Only the exact fragment repairs, index navigation and migration log addition enumerated per note; surrounding blank-line differences ignored. No Humanizer rewrite of legacy note bodies.','source_rows':len(rows),'portable_records':len(ledger['sources']),'source_rows_preserved_exactly':True,'shared_portable_identities':[{'portable_id':key,'legacy_ids':[x['source_id'] for x in value['legacy_records']]} for key,value in ledger['sources'].items() if len(value['legacy_records'])>1],'raw_unchanged':True,'roadmap_unchanged':True,'canvas_valid':True,'notes':records}
(OUT/'preservation.json').write_text(json.dumps(result,indent=2)+'\n');(OUT/'legacy-note-bodies.diff').write_text(''.join(diffs));print(json.dumps({k:v for k,v in result.items() if k!='notes'},indent=2))


### evidence/obsidian-migration-2026-09-07/preservation.json
{
  "observed_at": "2026-09-07T18:08:29.828813+00:00",
  "command": "python3 evidence/obsidian-migration-2026-09-07/verify-preservation.py",
  "baseline_commit": "8df38d25f6863e90bb7e16b691b578231c62458d",
  "passed": true,
  "legacy_notes": 33,
  "legacy_claim_ids": 620,
  "all_original_frontmatter_preserved": true,
  "legacy_body_changes": "Only the exact fragment repairs, index navigation and migration log addition enumerated per note; surrounding blank-line differences ignored. No Humanizer rewrite of legacy note bodies.",
  "source_rows": 78,
  "portable_records": 76,
  "source_rows_preserved_exactly": true,
  "shared_portable_identities": [
    {
      "portable_id": "src-702c7ccb908ece125e72",
      "legacy_ids": [
        "SRC-0022",
        "SRC-0028",
        "SRC-0031"
      ]
    }
  ],
  "raw_unchanged": true,
  "roadmap_unchanged": true,
  "canvas_valid": true,
  "notes": [
    {
      "path": "wiki/benchmarks.md",
      "baseline_sha256": "76368bab63f0a71e1d283f8efd2aeaa1362da918effbc9963cfcb827919a357b",
      "current_sha256": "0c909f4a5cc6ce4e6d5ffca3cacbfa5ae79317204ccc41409053df459d46d1e6",
      "baseline_body_sha256": "36305b8e6dbebfc5ed781d190e8314ee9b653e845f8adae57e2eb375b9373a55",
      "current_body_sha256": "36305b8e6dbebfc5ed781d190e8314ee9b653e845f8adae57e2eb375b9373a55",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/contradictions.md",
      "baseline_sha256": "d16fa4dd391f1f621188f56973ebc35e587d5518255a7281250d02232b5e1153",
      "current_sha256": "38da79b300c05a356e0e41188ad5e6baa604e30f4f7ffacf9da15fe4b79204dd",
      "baseline_body_sha256": "7108169de3ef809ddf2fccbfdee0355a653cfb57da2a1d9c8594d6f855fdff74",
      "current_body_sha256": "0ba7944949c597a0253dda24bcfe60335f1f4e78d6f84cef4bf8e4c054df677d",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [
        {
          "removed_fragment": "#source-discrepancies-and-dispositions",
          "occurrences": 1,
          "disposition": "Link retains its same target note and visible label."
        }
      ],
      "addition": null
    },
    {
      "path": "wiki/decision.md",
      "baseline_sha256": "1d3fee1f75f220620fe771a7204f49b8af6b77e3a0b578700935df4f2b409e69",
      "current_sha256": "bd7bc67dc8f14d337210768df18aec764d18942f240d292bfcdb6186c3760490",
      "baseline_body_sha256": "fb6731ff77fcf98c07ceb1792abfd11a160da677b7489a2746953697d9a9d2fe",
      "current_body_sha256": "fb6731ff77fcf98c07ceb1792abfd11a160da677b7489a2746953697d9a9d2fe",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/defiformal-taxonomy.md",
      "baseline_sha256": "885119687b2753810e09e6a8de371b4e24d0ac24fe495c64c01f8b90bf1e0d08",
      "current_sha256": "faeb6ddb2bd68e6bd59d4c8ad30424f9b156402a73908569008aa5a0141b5e3d",
      "baseline_body_sha256": "26e94f9e865c522eb644da05035085accba69b29b23b0300502b00c4b9d490f4",
      "current_body_sha256": "1ba9d2d7e11320de8eaca14691805bb03851bed1733e766284bad3aeebb6276b",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [
        {
          "removed_fragment": "#clm-0195-intents-report-separates-authority-plans-and-receipts",
          "occurrences": 1,
          "disposition": "Link retains its same target note and visible label."
        }
      ],
      "addition": null
    },
    {
      "path": "wiki/formal-assurance.md",
      "baseline_sha256": "1e8b739038b0876bcf15125d9050fd55bf0b3dd57f4c6040f5e6e035e4bf339e",
      "current_sha256": "331973ce31330b4bf2457f619f556c72033824c15f96f0955a271004b6a74c05",
      "baseline_body_sha256": "c026947f07eecc77a7d08d1f724766940e32aa12f425cd8a9eb563422947971c",
      "current_body_sha256": "c026947f07eecc77a7d08d1f724766940e32aa12f425cd8a9eb563422947971c",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/index.md",
      "baseline_sha256": "05abada312115d0aac398e65b690a5e7e748c903d4020b0d2b6a0bd52ada48e6",
      "current_sha256": "990cd4a4acb9faae9c627b64f16378bff7f070699be41d0e5d2a2565ad82649a",
      "baseline_body_sha256": "05abada312115d0aac398e65b690a5e7e748c903d4020b0d2b6a0bd52ada48e6",
      "current_body_sha256": "fad4e7d4675e24d06f64e7cff178fc42e84d6f8ccb87ade66ccfb0f4f0a7459a",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [
        {
          "removed_fragment": "#clm-0187-user-reset-after-verification-detour",
          "occurrences": 1,
          "disposition": "Link retains its same target note and visible label."
        },
        {
          "removed_fragment": "#clm-0186--checkpoint-after-the-a5-view-typecheck",
          "occurrences": 1,
          "disposition": "Link retains its same target note and visible label."
        }
      ],
      "addition": "vault navigation"
    },
    {
      "path": "wiki/k-framework/k-backends-and-tools.md",
      "baseline_sha256": "d97ac8e7238a01408356c73cd6ce271eca9bc6d0790cc0959f4928f5891a427c",
      "current_sha256": "04d4b366c6cba46a5e9a66f6df89926a48743bb3b79df205452309d80a23a0a0",
      "baseline_body_sha256": "9ca1ca1546fa3b9bfba534723911526b4eb52fcecd9521731ee82d6fbaadc922",
      "current_body_sha256": "9ca1ca1546fa3b9bfba534723911526b4eb52fcecd9521731ee82d6fbaadc922",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/k-framework/k-best-practices.md",
      "baseline_sha256": "0b73fd4dc0c23869721988789439290aecb03eda8fffe9474d5cb8cd2b6e21b7",
      "current_sha256": "1ec4ce4d90d4cb14a6273da55a295ea42abf699c10b279be8af30b5a1b78e801",
      "baseline_body_sha256": "37acf7c5ec0d7378761cbd6ac1d73bd4f3845c94d248ed1b7b5ef42176e03cf6",
      "current_body_sha256": "37acf7c5ec0d7378761cbd6ac1d73bd4f3845c94d248ed1b7b5ef42176e03cf6",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/k-framework/k-builtins.md",
      "baseline_sha256": "b3bd0f89281cbff5afd92cc7f362c8eea1ca0fa23668dc2e3f250a1b4bd96a33",
      "current_sha256": "e4a512c71b22cef9e314c3b8bbb7cfba77859ec2a06961560a20e27b2be4a530",
      "baseline_body_sha256": "369e1bf64c40416aa1ca1bb648c49ef460b6185886ff2555011c3a874fa1cbf3",
      "current_body_sha256": "369e1bf64c40416aa1ca1bb648c49ef460b6185886ff2555011c3a874fa1cbf3",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/k-framework/k-documentation-graph.md",
      "baseline_sha256": "cafeaf954ab35cf8247a02daf65d9002f83830fad42c7d1c78203c4f57d08664",
      "current_sha256": "75f88fafde9ad864528f1e5cce13a2970cd8c46a84d1c84642fbff3e9f746dcb",
      "baseline_body_sha256": "3cb4b10cbc6762cada5a17c56f1b1575c86414dcd66eb704b441054c44032e32",
      "current_body_sha256": "3cb4b10cbc6762cada5a17c56f1b1575c86414dcd66eb704b441054c44032e32",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/k-framework/k-framework-overview.md",
      "baseline_sha256": "e73b1cadcb66945f9dd5d9742444d873672d3cec1343c2cb8487c59a32472933",
      "current_sha256": "366b93d2e47f1ac7e973e9e0f296e5fc879c7a3a2e8bf3c143749e0d7ade9aa6",
      "baseline_body_sha256": "c0e7af3bd80e59261aab0b82c773d25fcaa0ae2a8e991b7741b83247bfe5f9bc",
      "current_body_sha256": "c0e7af3bd80e59261aab0b82c773d25fcaa0ae2a8e991b7741b83247bfe5f9bc",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/k-framework/k-tutorial-basic.md",
      "baseline_sha256": "9537e1a47d744dda597bb24d6b6bf38a859033864911f75e74339beb9cb59360",
      "current_sha256": "1eb509eccba14e3e6e096f2b6807ae41770a2b7b2f845a578562744c6dd55ce9",
      "baseline_body_sha256": "26b127079c9280602be9aca0dfeaa1cfc4d67317b936a3dfb67a21fe6a7356e9",
      "current_body_sha256": "26b127079c9280602be9aca0dfeaa1cfc4d67317b936a3dfb67a21fe6a7356e9",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/k-framework/k-tutorial-intermediate.md",
      "baseline_sha256": "e92c473b0cd38f7801d60b99cc670a020a294072840f45d505f87ddafe3344d6",
      "current_sha256": "411b055ed479f80a4928a20da9ee8da04623a3c32d60b3474e8ffaad7a138660",
      "baseline_body_sha256": "6650640ab3b7fa9948af773425686ee3fd9e905863cad81ec30c7cab1160338a",
      "current_body_sha256": "6650640ab3b7fa9948af773425686ee3fd9e905863cad81ec30c7cab1160338a",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/k-framework/k-user-manual.md",
      "baseline_sha256": "b2beb534bbb4f83c00359ec690b72f3da1f01272ea21cf67ea32d4b97a63f79f",
      "current_sha256": "d29f8c65db2ce939cd65af615aa6057d470ce11fb3ca6cc9bd9431e084960f23",
      "baseline_body_sha256": "ad3a5afdbccc37916da7761f80c7a373031536500189a6269372420d46715b2c",
      "current_body_sha256": "ad3a5afdbccc37916da7761f80c7a373031536500189a6269372420d46715b2c",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/log.md",
      "baseline_sha256": "21597119519b4fc262aa4400c75260e9048b4d1ff4fb37132f8c8be11a64da78",
      "current_sha256": "f1d8c77af2d4adab6e8a47e732dc1887476cf205d03d6da605dede3cb5f3e16d",
      "baseline_body_sha256": "21597119519b4fc262aa4400c75260e9048b4d1ff4fb37132f8c8be11a64da78",
      "current_body_sha256": "eb85a2a15162e80c41bbe3749bfd0fbfd0f5b67f20847bafcad90f86ca54f863",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": "migration log entry"
    },
    {
      "path": "wiki/marlowe-baseline.md",
      "baseline_sha256": "e818303111b8a9f1c5edafb6a913f7e715d8cfb1042dd1304ab74941f5b22aae",
      "current_sha256": "3464e6e0ec68dce5a0d2758e557d0bf19acfacf6a74c8bbff6a76f12b36a028b",
      "baseline_body_sha256": "fc935f03188ec018b4770b8ad20b2b17a94f7d785a044af4fca8615fb9b5ca66",
      "current_body_sha256": "fc935f03188ec018b4770b8ad20b2b17a94f7d785a044af4fca8615fb9b5ca66",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/marlowe-repository-graph.md",
      "baseline_sha256": "3a41e9797fb0b6757a0f0fc244d8fddad957cad6286e1c801185bd83db43973b",
      "current_sha256": "be4cb664331b1fbf67d1663e0685b1e4ef024d2ce7862803409510713e70ae8b",
      "baseline_body_sha256": "39ab9d44f6aeca68a355b43f3c18c3d87d48b98e5d21e3de4fbc048fba649798",
      "current_body_sha256": "39ab9d44f6aeca68a355b43f3c18c3d87d48b98e5d21e3de4fbc048fba649798",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/midnight-repositories.md",
      "baseline_sha256": "69febb59a745aaed93801709ccdbc43b8e42af8d6226c13261cf3449c77f213f",
      "current_sha256": "c4b7821fd70f72cfdb379d7db1f051512f4f9b326bf02f202b3856658908e5cf",
      "baseline_body_sha256": "2324051377df8be381b5897b51064fed2f3d531ccfc6487e1b3d95e16273d62e",
      "current_body_sha256": "2324051377df8be381b5897b51064fed2f3d531ccfc6487e1b3d95e16273d62e",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/moriarty-architecture.md",
      "baseline_sha256": "a7e87aa3fdcfb5bf2e0773e2d5a82ae4bcaafffeaa96a56f7a7254a07f9f2c14",
      "current_sha256": "09fd386d431215eebb9ff5f6c54d6aab72797e8049fe2358938f2e5a9d4497b3",
      "baseline_body_sha256": "8dc6a3d0800aa3fd5c899efda746957a1799a590b87abee47f7a995714fb0b50",
      "current_body_sha256": "8dc6a3d0800aa3fd5c899efda746957a1799a590b87abee47f7a995714fb0b50",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/open-questions.md",
      "baseline_sha256": "009768c12ec70cff44c0de8910514505952daad826a9d29ebc72389e1bde7731",
      "current_sha256": "c5a62f9023310553fc6541254bd4ee2c147936f0b4c531bd26883252e5ce3898",
      "baseline_body_sha256": "5ad90e2df8919fcc6b03ffeb9c3ed4ceb7226ab4e893aa23bbab5d716073860a",
      "current_body_sha256": "5ad90e2df8919fcc6b03ffeb9c3ed4ceb7226ab4e893aa23bbab5d716073860a",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/research-journal.md",
      "baseline_sha256": "22322967d65959da6425125512626c9118f7a6eac271662b469560ef521fad53",
      "current_sha256": "34640760e7b97e99f871937521cc69d9043c8456795ee228c806a22eb5c0ba79",
      "baseline_body_sha256": "31c8142cf9074d00a7e0214d82a783a91212630bb3f92249cab4fec119875bcf",
      "current_body_sha256": "31c8142cf9074d00a7e0214d82a783a91212630bb3f92249cab4fec119875bcf",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/research-program.md",
      "baseline_sha256": "8817f5875a355f7b67b926bb60026ccee9ad5f23708e6b9afb1edb051fc9e2a1",
      "current_sha256": "333952d914514866b8a7b71da181c2b937bd626ccd186f14f33fe36a98111464",
      "baseline_body_sha256": "12dbaef880c9a0b215c275838d4c97d411181ebb04103856552181b891f804cb",
      "current_body_sha256": "a969d7a3a9d969b8ea8e2489fd7ccd50f8d36641811636c89e0f08ee86b32eb8",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [
        {
          "page": "wiki/research-program.md",
          "old": "../graphs/moriarty-decision-corpus/graphify-out/graph.json",
          "new": "meta/legacy-decision-graph.json",
          "occurrences": 2
        }
      ],
      "addition": null
    },
    {
      "path": "wiki/security.md",
      "baseline_sha256": "b5a1a73c1b9dc7f456d97c0c0b58b29f121b31131e8ea94281da5832622eddf7",
      "current_sha256": "299cb8d5349c9ef05e14d261f0378dd72684bea4b8d8e39f9e00a0076211770c",
      "baseline_body_sha256": "9b20526b7b916c96f20ac53cd5807ed0847f1c969ca925452fddb9bcdf53a0da",
      "current_body_sha256": "9b20526b7b916c96f20ac53cd5807ed0847f1c969ca925452fddb9bcdf53a0da",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/sources/llm-wiki-pattern.md",
      "baseline_sha256": "3a531b7280f59facc959313e151f5db8c3fdabf658df05541f8729b17e34dcf8",
      "current_sha256": "ec80f205b1a779ab04168cd96f6ad635e275914c8e0a122b0fda8da139b05eff",
      "baseline_body_sha256": "147744e5afaa732c0efe87e054477011f2138d2a43154bf2c24443a7d9e53d99",
      "current_body_sha256": "147744e5afaa732c0efe87e054477011f2138d2a43154bf2c24443a7d9e53d99",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir-k-semantics-plan.md",
      "baseline_sha256": "e7a8452002d276067ab48619099214ffae52d3041158dde773b088dff246d927",
      "current_sha256": "958326c9119e286d6f48741b2d0781f49a0fada96c238457347e636d1ebdda14",
      "baseline_body_sha256": "130dbdc0054840b002beccc7206b864456955ad044981d2ae743b863b375d5e2",
      "current_body_sha256": "130dbdc0054840b002beccc7206b864456955ad044981d2ae743b863b375d5e2",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir/compact-to-zkir-pipeline.md",
      "baseline_sha256": "0257de08c5ea51efb72e750d18e5ed95736ef9339c5426019ea3145f0a2c5597",
      "current_sha256": "4f94cf480810269c7d37b1faf900db6282857ae950b3a09f1aa8cb95e0bfae49",
      "baseline_body_sha256": "ed3d0c74a9ac74d199b8b53aa189afdfe1ef61dc12f53cb255132f405d445133",
      "current_body_sha256": "ed3d0c74a9ac74d199b8b53aa189afdfe1ef61dc12f53cb255132f405d445133",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir/midnight-k-tooling.md",
      "baseline_sha256": "622a393059d068cc4d91f407ec49be97eb413ce768e208ecec8b46b29c461354",
      "current_sha256": "ba8b0ae47a066c3730fe72a7fa572998db587e16b457ba174fac342bb22627c4",
      "baseline_body_sha256": "b8997026b85fcdd4fe1f730506a8e2be27c4755840a6ff072be3643bbd4d773c",
      "current_body_sha256": "b8997026b85fcdd4fe1f730506a8e2be27c4755840a6ff072be3643bbd4d773c",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir/zkir-formal-spec-agda.md",
      "baseline_sha256": "2793a8727eba976953cad516b832fd6bb6048af9522548778df22ff5a30ca414",
      "current_sha256": "a03a87b1ab9591482733a54f4086bb8b21a453db3b6a59caf10a316fa50dc3a1",
      "baseline_body_sha256": "20a1ac952cfd3d178e039be22c8179ac8053baac59cddf9d46792da33129adcc",
      "current_body_sha256": "ee3e90c81fb9d320a8416cad433c9250aa8b28329fd80ca433996bb1c1c35c18",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [
        {
          "page": "wiki/zkir/zkir-formal-spec-agda.md",
          "old": "../../repos/input-output-hk/arc-zkir/",
          "new": "https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/",
          "occurrences": 23
        }
      ],
      "addition": null
    },
    {
      "path": "wiki/zkir/zkir-instruction-set.md",
      "baseline_sha256": "d04af68d75617781ace85b76c344a7da638c56db771ba6313f1c52dd33dd2575",
      "current_sha256": "32b1992a6830699b19af0067f501aa8436a844772ba7e3bc2beeae7b721753d8",
      "baseline_body_sha256": "42a3c468e5cc11658e053718948ec632a3b669223142e1283e4f1550c826a928",
      "current_body_sha256": "42a3c468e5cc11658e053718948ec632a3b669223142e1283e4f1550c826a928",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir/zkir-k-definition.md",
      "baseline_sha256": "0fc592a736257fd3e6c9291b5eedd2924b999eacf96403a4790722d1c0ce2fee",
      "current_sha256": "c3512889786735b4318f94791d7dad578e48b306ce2c55b5392d0238ab221d2d",
      "baseline_body_sha256": "54bd8e9cf08e7171ebb2c255e6b65a41f5524895a0f13f23a49a4af29b294bae",
      "current_body_sha256": "54bd8e9cf08e7171ebb2c255e6b65a41f5524895a0f13f23a49a4af29b294bae",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir/zkir-type-system.md",
      "baseline_sha256": "fe3ec613d085b57d23612e727426bb00418b96bc99505d8107b4157db848cb07",
      "current_sha256": "437a59196dfa1bef5322e2d2a0c154c76b09502cc9887a461253403b370dfe4a",
      "baseline_body_sha256": "c782b2c7473de726675fff8309f44281b1df57ce6337c4ed7d0c7bdb2c07254f",
      "current_body_sha256": "c782b2c7473de726675fff8309f44281b1df57ce6337c4ed7d0c7bdb2c07254f",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir/zkir-v3-divergence-review.md",
      "baseline_sha256": "0703e5136cf38a3114031ba71d80629fbd52f46930887aa5649faaad743bcdef",
      "current_sha256": "5482ab127f424572c882ebb36ee895a062262ff1187bd116572196bcfc3640d7",
      "baseline_body_sha256": "b19ff51766de6fbc78d8810ce19eb71716cb778ce129903c0c4a69b1e0e95638",
      "current_body_sha256": "b19ff51766de6fbc78d8810ce19eb71716cb778ce129903c0c4a69b1e0e95638",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    },
    {
      "path": "wiki/zkir/zkir-vm-semantics.md",
      "baseline_sha256": "cbf5ef0dc98358b06fdfd03a10bf33367645fcca7f94087e8f7f4b2bb3277c7b",
      "current_sha256": "877375c1fc816a1e39071ba0547185b8488bb3df667f7f06de8548d7c40f594e",
      "baseline_body_sha256": "60db04c92902882f34986f1d1715d7cb7995f67cfcb19bdef94b8a9c7d1e010d",
      "current_body_sha256": "60db04c92902882f34986f1d1715d7cb7995f67cfcb19bdef94b8a9c7d1e010d",
      "body_equal_after_enumerated_navigation_changes": true,
      "original_frontmatter_preserved_in_order": true,
      "link_repairs": [],
      "addition": null
    }
  ]
}


### evidence/obsidian-migration-2026-09-07/legacy-note-bodies.diff
--- 8df38d25f6863e90bb7e16b691b578231c62458d/wiki/contradictions.md
+++ current/wiki/contradictions.md
@@ -101,7 +101,7 @@
 
 ## CLM-0190: ACTUS design-study source gaps and DeFi model scope
 
-SRC-0042's [target study](../docs/research/2026-09-06-actus-defi-design-study.md#source-discrepancies-and-dispositions)
+SRC-0042's [target study](../docs/research/2026-09-06-actus-defi-design-study.md)
 records DS-01 through DS-07. Business-day interpretation now has pinned Haskell
 and pam08/pam09 support, but the missing CSMP vector prevents a conformance
 claim. ANN initial Prnxt contains an incomplete formula; COM has quantity/sign
--- 8df38d25f6863e90bb7e16b691b578231c62458d/wiki/defiformal-taxonomy.md
+++ current/wiki/defiformal-taxonomy.md
@@ -11,7 +11,7 @@
 classifies declaration names and labels the output a keyword seed. This source
 observation does not rerun the mathematics or certify a new basis. Preserve the
 72-row target corpus and semantic counterexamples while selecting Core from
-required behaviors. [CLM-0195](research-journal.md#clm-0195-intents-report-separates-authority-plans-and-receipts)
+required behaviors. [CLM-0195](research-journal.md)
 and the [reconciliation](../docs/research/2026-09-06-intents-report-integration.md)
 state the resulting R2b authority/refinement obligations.
 
--- 8df38d25f6863e90bb7e16b691b578231c62458d/wiki/index.md
+++ current/wiki/index.md
@@ -1,4 +1,7 @@
+
 # Moriarty language wiki index
+
+[[wiki/overview|Vault overview]] · [[wiki/workflow|Research workflow]] · [[wiki/canvases/moriarty|Language map]] · [[wiki/meta/provenance|Provenance mapping]]
 
 Read this page before searching externally. Merge new evidence into the existing
 topic pages and preserve their source and claim identifiers.
@@ -56,7 +59,7 @@
   transactions, bounded correctness and backend questions.
 - [New design cycle](../docs/superpowers/plans/2026-09-06-actus-defi-pcd-replanning.md):
   target study, unified-semantics proposal and developer mock.
-- [Journal CLM-0187](research-journal.md#clm-0187-user-reset-after-verification-detour)
+- [Journal CLM-0187](research-journal.md)
   records the scope change. Older continuation instructions below do not
   authorize automatic native work.
 
@@ -98,7 +101,7 @@
   limitations, both offline Apalache heap failures, and the candidate-specific
   signing/verification/commit boundary under implementation, and the reviewed
   native Candidate B design with explicit stranded-escrow and clock boundaries.
-- [Candidate A continuation evidence](research-journal.md#clm-0186--checkpoint-after-the-a5-view-typecheck) —
+- [Candidate A continuation evidence](research-journal.md) —
   admitted A4 source gate and failed-export preservation, successful A5 view
   typecheck, and saved follow-up drafts; exports and verification remain open.
 
--- 8df38d25f6863e90bb7e16b691b578231c62458d/wiki/log.md
+++ current/wiki/log.md
@@ -1,3 +1,4 @@
+
 # Wiki log
 
 ## [2026-09-04] checkpoint | Complete S01 and begin S02 contract
@@ -961,3 +962,7 @@
 ## 2026-09-07: Preserve old work and consolidate current development
 
 SRC-0076 authorizes publication and cleanup. The [historical archive](../docs/ARCHIVE.md) retains superseded implementations and their evidence; the current tree keeps the Midnight packages, financial studies and MC01-MC08 program. Publication authority changes, while product and proof acceptance remain unchanged.
+
+## 2026-09-07: Adopt the research workflow as an Obsidian vault
+
+The user requested AgriciDaniel/claude-obsidian. Adopted the WSL repository as one vault, preserving note paths, source captures, identifiers and claim scopes. Added Obsidian metadata, navigation, a language Canvas and portable source-identity mapping. The legacy claim index is navigational; no claim was automatically accepted. See [[wiki/meta/provenance|provenance mapping]] and [[wiki/workflow|workflow]].
--- 8df38d25f6863e90bb7e16b691b578231c62458d/wiki/research-program.md
+++ current/wiki/research-program.md
@@ -61,7 +61,7 @@
 inferred relationships; status S3.
 
 The graph is at
-[`../graphs/moriarty-decision-corpus/graphify-out/graph.json`](../graphs/moriarty-decision-corpus/graphify-out/graph.json).
+[`meta/legacy-decision-graph.json`](meta/legacy-decision-graph.json).
 Its shortest useful synthesis path joins the candidate taxonomy, M5 profile,
 atomic-swap stop test, backend boundedness condition, and library-only fallback.
 Graph centrality is not evidence authority.
--- 8df38d25f6863e90bb7e16b691b578231c62458d/wiki/zkir/zkir-formal-spec-agda.md
+++ current/wiki/zkir/zkir-formal-spec-agda.md
@@ -25,7 +25,7 @@
 
 ### Abstract syntax and data structures
 
-The formal abstract syntax of ZKIR v3 is mechanized in [`Syntax.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Syntax.agda) and documented in `docs/zkir-v3-spec.md` §5 (CLM-0606; SRC-0038 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).
+The formal abstract syntax of ZKIR v3 is mechanized in [`Syntax.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Syntax.agda) and documented in `docs/zkir-v3-spec.md` §5 (CLM-0606; SRC-0038 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).
 A ZKIR circuit is represented by the record `IrSource`:
 
 ```agda
@@ -53,7 +53,7 @@
 A declared input is a pair `TypedIdentifier` combining a name and an `IrType` (CLM-0606; SRC-0038 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).
 The minor version `IrMinorVersion` contains only the constructor `V0` (CLM-0606; SRC-0038 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).
 
-ZKIR v3 specifies exactly 13 types in `IrType`, mechanized in [`Types.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Types.agda) and matching `zkir-v3/src/ir_types.rs` (CLM-0607; SRC-0038 src/zkir-v3/Types.agda; source fact; not reproduced; high; S4):
+ZKIR v3 specifies exactly 13 types in `IrType`, mechanized in [`Types.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Types.agda) and matching `zkir-v3/src/ir_types.rs` (CLM-0607; SRC-0038 src/zkir-v3/Types.agda; source fact; not reproduced; high; S4):
 1. `Native`: element of the BLS12-381 scalar field `Fr`, with encoded length 1.
 2. `Bytes32`: 32-byte sequence represented as two field elements, with encoded length 2.
 3. `JubjubPoint`: point on the embedded twisted Edwards curve Jubjub, encoded as affine coordinates `(x, y)` with encoded length 2.
@@ -77,7 +77,7 @@
 ### Operational semantics (off-circuit witness generation)
 
 The operational semantics, designated *preprocess*, represents witness generation executed by the prover before proving (CLM-0609; SRC-0038 docs/zkir-v3-spec.md §6; source fact; not reproduced; high; S4).
-It is mechanized as a deterministic step function in [`Semantics.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Semantics.agda) and discussed in [zkir-vm-semantics.md](zkir-vm-semantics.md).
+It is mechanized as a deterministic step function in [`Semantics.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Semantics.agda) and discussed in [zkir-vm-semantics.md](zkir-vm-semantics.md).
 The prover executes a circuit with a proof preimage `P` of type `ProofPreimage`:
 
 ```agda
@@ -112,7 +112,7 @@
 ### Circuit semantics (in-circuit constraint synthesis)
 
 The circuit semantics formalizes the constraint system synthesized from `IrSource` alone, without knowledge of the preimage (CLM-0612; SRC-0038 docs/zkir-v3-spec.md §7; source fact; not reproduced; high; S4).
-Synthesis is mechanized in [`Circuit.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Circuit.agda) as a total function `synth : IrSource → Circuit` (CLM-0612; SRC-0038 src/zkir-v3/Circuit.agda; source fact; not reproduced; high; S4).
+Synthesis is mechanized in [`Circuit.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Circuit.agda) as a total function `synth : IrSource → Circuit` (CLM-0612; SRC-0038 src/zkir-v3/Circuit.agda; source fact; not reproduced; high; S4).
 A circuit is represented by the record `Circuit`:
 
 ```agda
@@ -137,7 +137,7 @@
 
 ## The cryptographic trust base and the `--safe` discipline
 
-The cryptographic primitives and algebraic axioms are encapsulated in the record `Assumptions` in [`Assumptions.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Assumptions.agda) (CLM-0614; SRC-0038 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
+The cryptographic primitives and algebraic axioms are encapsulated in the record `Assumptions` in [`Assumptions.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Assumptions.agda) (CLM-0614; SRC-0038 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
 Every module in `src/zkir-v3` parameterizes over `Assumptions`:
 
 ```agda
@@ -169,7 +169,7 @@
 ### 1. Circuit faithfulness (Property P5)
 
 Circuit faithfulness establishes that witness generation succeeds if and only if the synthesized circuit accepts the canonical witness (CLM-0619; SRC-0038 docs/zkir-v3-spec.md §8.3; source fact; not reproduced; high; S4).
-The program-level theorem is mechanized in [`CircuitProof.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitProof.agda) (CLM-0619; SRC-0038 src/zkir-v3/CircuitProof.agda; source fact; not reproduced; high; S4):
+The program-level theorem is mechanized in [`CircuitProof.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/CircuitProof.agda) (CLM-0619; SRC-0038 src/zkir-v3/CircuitProof.agda; source fact; not reproduced; high; S4):
 
 ```agda
 circuit-faithful : ∀ {S P s st0}
@@ -215,7 +215,7 @@
 ### 2. Statement soundness
 
 Statement soundness establishes that every satisfying circuit witness corresponds to a genuine operational execution agreeing on public inputs (CLM-0622; SRC-0038 docs/zkir-v3-spec.md §8.4; source fact; not reproduced; high; S4).
-The result is mechanized in [`StatementSoundness.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/StatementSoundness.agda) (CLM-0622; SRC-0038 src/zkir-v3/StatementSoundness.agda; source fact; not reproduced; high; S4).
+The result is mechanized in [`StatementSoundness.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/StatementSoundness.agda) (CLM-0622; SRC-0038 src/zkir-v3/StatementSoundness.agda; source fact; not reproduced; high; S4).
 The realizer record `SubRealizer` packages the extracted witness data:
 
 ```agda
@@ -277,7 +277,7 @@
 ### 3. Extraction uniqueness
 
 Extraction uniqueness proves that the extracted execution explaining a satisfying witness is mathematically unique (CLM-0624; SRC-0038 docs/zkir-v3-spec.md §8.4; source fact; not reproduced; high; S4).
-The theorems are mechanized in [`StatementUniqueness.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/StatementUniqueness.agda) (CLM-0624; SRC-0038 src/zkir-v3/StatementUniqueness.agda; source fact; not reproduced; high; S4):
+The theorems are mechanized in [`StatementUniqueness.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/StatementUniqueness.agda) (CLM-0624; SRC-0038 src/zkir-v3/StatementUniqueness.agda; source fact; not reproduced; high; S4):
 
 ```agda
 statement-unique : ∀ {S w} → producer-SA S → WInputs w (IrSource.inputs S)
@@ -313,21 +313,21 @@
 
 | Module | Contents and Primary Definitions | Lines / Bytes |
 |---|---|---|
-| [`Assumptions.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Assumptions.agda) | Cryptographic trust base record: carrier types (`Fr`, `Alignment`, curves), field arithmetic, bit decomposition, Jubjub/foreign curve contracts, hash functions, non-triviality `1ᶠ≢0ᶠ`, Group C round-trips. | 557 lines / 28.5 KB |
-| [`Types.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Types.agda) | Definitions of `IrType` (13 constructors), `IrValue` (13 constructors), `encoded-len`, `typeof`, decidable type equality `_≟T_`. | 387 lines / 17.9 KB |
-| [`Encoding.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Encoding.agda) | Wire format transformations `encode : IrValue → List Fr` and `decode : IrType → List Fr → Maybe IrValue`. | 96 lines / 4.0 KB |
-| [`Syntax.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Syntax.agda) | Abstract syntax: `Identifier`, `Operand` (`var`, `imm`), `TypedIdentifier`, `IrMinorVersion` (`V0`), the 34 `Instruction` variants, and record `IrSource`. | 296 lines / 8.2 KB |
-| [`Semantics.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Semantics.agda) | Operational semantics interpreter: `ProofPreimage`, `State`, `step`, `run`, `init`, `preprocess`. | 577 lines / 27.5 KB |
-| [`SemanticsProperties.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/SemanticsProperties.agda) | Structural properties of operational semantics: store ordering `_⊑_`, domain growth `step-dom`, memory extension `run-extends`, run inversion `run-inv`, and `preprocess-walk-consumed`. | 772 lines / 40.1 KB |
-| [`Circuit.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Circuit.agda) | Constraint vocabulary `Constraint`, witness model `CircuitWitness`, satisfaction relations `holds` and `satisfies`, and synthesis function `synth`. | 939 lines / 40.5 KB |
-| [`CircuitBridge.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitBridge.agda) | Bridge definitions: canonical witness constructor `witness-of`, constraint monotonicity `holds-mono`, constraint lowering `holds-lower`, and constraint extractor `csOf`. | 1,173 lines / 55.2 KB |
-| [`CircuitFaithfulness.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitFaithfulness.agda) | Forward faithfulness: per-instruction forward lemmas `*-fwd` and program-level induction `forward`. | 2,828 lines / 125.9 KB |
-| [`CircuitBackward.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitBackward.agda) | Backward step reconstruction: 43 per-instruction inversion lemmas `*-bwd` reconstructing operational transitions from constraint satisfaction. | 2,130 lines / 106.9 KB |
-| [`Obligations.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Obligations.agda) | Static producer checks: single assignment `producer-SA`/`producer-SA?`, value typing `producer-WT`/`producer-WT?`, bit bounds `producer-WF2`/`producer-WF2?`, and transfer theorem `preprocessʳ-agree`. | 2,499 lines / 129.2 KB |
-| [`CircuitProof.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitProof.agda) | Program-level theorem assembly: `BwdWalk`, `bwd-go`, run-spine projection `preprocess→BwdWalk`, `backward`, `forward-sa`, and headline theorem `circuit-faithful`. | 2,151 lines / 112.2 KB |
-| [`StatementSoundness.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/StatementSoundness.agda) | Statement soundness: witness shape predicate `WShape`/`WShape?`, preimage construction `build`, record `SubRealizer`, `statement-sound`, `extractor-complete`, and `preprocess→WShape`. | 6,144 lines / 318.6 KB |
-| [`StatementUniqueness.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/StatementUniqueness.agda) | Extraction uniqueness: transcript pinning lemmas, `statement-unique`, and exactly-one packaging `statement-sound-unique`. | 915 lines / 47.6 KB |
-| [`Main.agda`](../../repos/input-output-hk/arc-zkir/src/zkir-v3/Main.agda) | Aggregation module importing all files above, verifying whole-development compilation under `--safe`. | 27 lines / 1.0 KB |
+| [`Assumptions.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Assumptions.agda) | Cryptographic trust base record: carrier types (`Fr`, `Alignment`, curves), field arithmetic, bit decomposition, Jubjub/foreign curve contracts, hash functions, non-triviality `1ᶠ≢0ᶠ`, Group C round-trips. | 557 lines / 28.5 KB |
+| [`Types.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Types.agda) | Definitions of `IrType` (13 constructors), `IrValue` (13 constructors), `encoded-len`, `typeof`, decidable type equality `_≟T_`. | 387 lines / 17.9 KB |
+| [`Encoding.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Encoding.agda) | Wire format transformations `encode : IrValue → List Fr` and `decode : IrType → List Fr → Maybe IrValue`. | 96 lines / 4.0 KB |
+| [`Syntax.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Syntax.agda) | Abstract syntax: `Identifier`, `Operand` (`var`, `imm`), `TypedIdentifier`, `IrMinorVersion` (`V0`), the 34 `Instruction` variants, and record `IrSource`. | 296 lines / 8.2 KB |
+| [`Semantics.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Semantics.agda) | Operational semantics interpreter: `ProofPreimage`, `State`, `step`, `run`, `init`, `preprocess`. | 577 lines / 27.5 KB |
+| [`SemanticsProperties.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/SemanticsProperties.agda) | Structural properties of operational semantics: store ordering `_⊑_`, domain growth `step-dom`, memory extension `run-extends`, run inversion `run-inv`, and `preprocess-walk-consumed`. | 772 lines / 40.1 KB |
+| [`Circuit.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Circuit.agda) | Constraint vocabulary `Constraint`, witness model `CircuitWitness`, satisfaction relations `holds` and `satisfies`, and synthesis function `synth`. | 939 lines / 40.5 KB |
+| [`CircuitBridge.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/CircuitBridge.agda) | Bridge definitions: canonical witness constructor `witness-of`, constraint monotonicity `holds-mono`, constraint lowering `holds-lower`, and constraint extractor `csOf`. | 1,173 lines / 55.2 KB |
+| [`CircuitFaithfulness.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/CircuitFaithfulness.agda) | Forward faithfulness: per-instruction forward lemmas `*-fwd` and program-level induction `forward`. | 2,828 lines / 125.9 KB |
+| [`CircuitBackward.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/CircuitBackward.agda) | Backward step reconstruction: 43 per-instruction inversion lemmas `*-bwd` reconstructing operational transitions from constraint satisfaction. | 2,130 lines / 106.9 KB |
+| [`Obligations.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Obligations.agda) | Static producer checks: single assignment `producer-SA`/`producer-SA?`, value typing `producer-WT`/`producer-WT?`, bit bounds `producer-WF2`/`producer-WF2?`, and transfer theorem `preprocessʳ-agree`. | 2,499 lines / 129.2 KB |
+| [`CircuitProof.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/CircuitProof.agda) | Program-level theorem assembly: `BwdWalk`, `bwd-go`, run-spine projection `preprocess→BwdWalk`, `backward`, `forward-sa`, and headline theorem `circuit-faithful`. | 2,151 lines / 112.2 KB |
+| [`StatementSoundness.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/StatementSoundness.agda) | Statement soundness: witness shape predicate `WShape`/`WShape?`, preimage construction `build`, record `SubRealizer`, `statement-sound`, `extractor-complete`, and `preprocess→WShape`. | 6,144 lines / 318.6 KB |
+| [`StatementUniqueness.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/StatementUniqueness.agda) | Extraction uniqueness: transcript pinning lemmas, `statement-unique`, and exactly-one packaging `statement-sound-unique`. | 915 lines / 47.6 KB |
+| [`Main.agda`](https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/src/zkir-v3/Main.agda) | Aggregation module importing all files above, verifying whole-development compilation under `--safe`. | 27 lines / 1.0 KB |
 
 ## Evolution from ZKIR v2 to ZKIR v3
 


### evidence/obsidian-migration-2026-09-07/portable-link-repairs.json
{
  "repairs": [
    {
      "page": "wiki/research-program.md",
      "old": "../graphs/moriarty-decision-corpus/graphify-out/graph.json",
      "new": "meta/legacy-decision-graph.json",
      "occurrences": 2
    },
    {
      "page": "wiki/zkir/zkir-formal-spec-agda.md",
      "old": "../../repos/input-output-hk/arc-zkir/",
      "new": "https://github.com/input-output-hk/arc-zkir/blob/fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9/",
      "occurrences": 23
    }
  ],
  "arc_zkir_pin": "fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9",
  "pin_basis": "Same full commit already cited by CLM-0601 in the preserved note and present in the inspected source checkout.",
  "graph_source_sha256": "ab9d0f047891835a96daaf99103c11418cbb5d86d5584909010ed79b816e64d8",
  "retained_graph_sha256": "ab9d0f047891835a96daaf99103c11418cbb5d86d5584909010ed79b816e64d8"
}


### evidence/obsidian-migration-2026-09-07/fresh-checkout.json
{
  "observed_at": "2026-09-07T18:09:51.325065+00:00",
  "source_tree": "f98070103febc76da0758564c681282301cd846a",
  "method": "git checkout-index -a into a new directory; no ignored local runtime/source files copied",
  "checkout_path": "/tmp/moriarty-vault-checkout-nhe8t3cl",
  "results": [
    {
      "command": [
        "python3",
        "/home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py",
        "doctor"
      ],
      "exit_code": 0,
      "result": {
        "checks": {
          "meta_writable": true,
          "mutation_lock_held": false,
          "obsidian_config": true,
          "raw": true,
          "vault_exists": true,
          "wiki": true
        },
        "legacy_layout": false,
        "ok": true,
        "schema": "claude-obsidian.doctor.v1",
        "selection_source": "workspace-config",
        "vault_root": "/tmp/moriarty-vault-checkout-nhe8t3cl",
        "version": "2.1.1"
      }
    },
    {
      "command": [
        "python3",
        "/home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py",
        "lint",
        "--strict"
      ],
      "exit_code": 0,
      "result": {
        "allowlisted_dangling_links": [],
        "ambiguous_targets": [],
        "as_of": "2026-09-07",
        "configuration_errors": [],
        "dead_links": [],
        "duplicate_basenames": [],
        "empty_sections": [],
        "engine_version": "1.1.1",
        "missing_frontmatter": [],
        "orphans": [],
        "provenance_errors": [],
        "read_errors": [],
        "stale_index_entries": [],
        "summary": {
          "allowlisted_dangling_links": 0,
          "category_counts": {
            "ambiguous_targets": 0,
            "configuration_errors": 0,
            "dead_links": 0,
            "duplicate_basenames": 0,
            "empty_sections": 0,
            "missing_frontmatter": 0,
            "orphans": 0,
            "provenance_errors": 0,
            "read_errors": 0,
            "stale_index_entries": 0
          },
          "issues_found": 0,
          "links_scanned": 258,
          "pages_scanned": 37
        },
        "version": 1
      }
    }
  ]
}


### evidence/obsidian-migration-2026-09-07/lint-final.json
{
  "allowlisted_dangling_links": [],
  "ambiguous_targets": [],
  "as_of": "2026-09-07",
  "configuration_errors": [],
  "dead_links": [],
  "duplicate_basenames": [],
  "empty_sections": [],
  "engine_version": "1.1.1",
  "missing_frontmatter": [],
  "orphans": [],
  "provenance_errors": [],
  "read_errors": [],
  "stale_index_entries": [],
  "summary": {
    "allowlisted_dangling_links": 0,
    "category_counts": {
      "ambiguous_targets": 0,
      "configuration_errors": 0,
      "dead_links": 0,
      "duplicate_basenames": 0,
      "empty_sections": 0,
      "missing_frontmatter": 0,
      "orphans": 0,
      "provenance_errors": 0,
      "read_errors": 0,
      "stale_index_entries": 0
    },
    "issues_found": 0,
    "links_scanned": 258,
    "pages_scanned": 37
  },
  "version": 1
}


### docs/OBSIDIAN.md
# Moriarty Obsidian vault

Open `/home/charl/Moriarty` with **Open folder as vault** in Obsidian. Start at
[the vault overview](../wiki/overview.md), [research index](../wiki/index.md) or
[language map](../wiki/canvases/moriarty.canvas). The repository itself is the
vault, so Git and Obsidian edit the same notes and citations.

The Windows path is `\\wsl.localhost\Ubuntu-26.04\home\charl\Moriarty`.
Agent writes run in WSL. Linux Obsidian is installed on this machine; using the
Windows editor does not move the vault out of WSL.

## Workflow and tool

[Research workflow](../wiki/workflow.md) describes ingest, query, save and lint.
[WIKI_SCHEMA.md](../WIKI_SCHEMA.md) retains Moriarty's evidence contract.
[Provenance mapping](../wiki/meta/provenance.md) explains how existing source and
claim identifiers coexist with the portable ledgers.

The requested [claude-obsidian project](https://github.com/AgriciDaniel/claude-obsidian)
is pinned at `ad67087cad22ad84cc3288f915588ae42c0c2b44`. Its installed location is
`/home/charl/.local/share/claude-obsidian`, separate from the vault. Existing Codex
skills link to that checkout; no second installation is needed. The
[tool receipt](../raw/sources/claude-obsidian-2026-09-07/receipt.json) records the pin
and captured instructions. The [upstream WSL guide](https://github.com/AgriciDaniel/claude-obsidian/blob/ad67087cad22ad84cc3288f915588ae42c0c2b44/docs/windows-wsl.md)
explains its filesystem requirements.

From the repository:

```sh
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py doctor
python3 /home/charl/.local/share/claude-obsidian/scripts/claude-obsidian.py lint --strict
```

Use the installed `wiki`, `wiki-ingest`, `wiki-query`, `save`, `wiki-lint` and
`canvas` skills in Codex. Claude Code can load the same local product with
`claude --plugin-dir /home/charl/.local/share/claude-obsidian` from this repository.
The workspace config resolves the vault relative to its own location, so a
fresh clone can also act as a vault after installing the pinned tool.

For another WSL machine, clone the tool outside Moriarty and install its portable
Codex links:

```sh
git clone https://github.com/AgriciDaniel/claude-obsidian.git ../claude-obsidian
git -C ../claude-obsidian checkout ad67087cad22ad84cc3288f915588ae42c0c2b44
bash ../claude-obsidian/bin/setup-multi-agent.sh --host codex
bash ../claude-obsidian/bin/setup-multi-agent.sh --host codex --apply
```

Inspect the first command's installation preview before applying. Substitute
that checkout's absolute path in the core commands above. Do not use the tool
checkout as a vault or overwrite an existing unrelated skill installation.

## Source control

The notes, shared vault settings, Canvas and evidence are tracked. Personal
workspace state, local transports, staged inbox inputs and transaction recovery
journals are ignored. Review source classification before publishing a new
capture. Commits and pushes are explicit; no automatic sync hooks are installed.

The graph starts with `path:wiki`. Source captures, repository clones, generated
graphs and local runtime directories are excluded from routine Obsidian search.
They remain on disk and retain their source locators. Historical execution work
is accessible through [the recovery archive](ARCHIVE.md).


### wiki/meta/provenance.md
---
id: moriarty.vault.provenance
title: Provenance mapping
type: overview
status: active
created: 2026-09-07
updated: 2026-09-07
tags:
  - moriarty
  - research
sources:
  - SRC-0077
  - SRC-0078
updated_at: 2026-09-07T18:01:49Z
---

# Provenance mapping

The original [source inventory](../../evidence/source-inventory.csv) retains every `SRC-####` record and its original metadata. Existing `CLM-####` claims retain their wording, citations, confidence and S0-S7 lifecycle labels in the notes.

The portable [source ledger](ledgers/source-ledger.json) assigns the skill's content-derived `src-...` identity and stores a lossless `legacy_records` list for each original record. `legacy_source_ids` maps every original ID to its portable record. File hashes describe bytes observed during migration; `legacy_hash_matches_observed` distinguishes those hashes from the original inventory assertion. Missing, archived or compound locators are preserved as manual, unreviewed identities rather than invented file matches. Review state remains `unreviewed` until the portable provenance contract is assessed; this does not downgrade or replace the original scoped evidence.

The [legacy claim index](legacy-claim-index.json) maps exact observed claim identifiers to their note paths. It is a navigation index, not a semantic extraction or new proof. The portable [claim ledger](ledgers/claim-ledger.json) starts empty: migration does not automatically extract or accept claims from prose. Future scoped ingestion can add explicit support and contradiction records after reviewing the evidence. No automated claim acceptance occurred.

`created` on adopted notes records the earliest retained Git addition date. `updated` records the metadata migration date; the original `updated_at` remains available. Existing types, stable page IDs and lifecycle vocabularies are preserved.

The [migration receipt](../../evidence/obsidian-migration-2026-09-07/README.md) records the skill pin, transactions and checks. Return to [[wiki/workflow|the workflow]] or [[wiki/index|the research index]].


### .obsidian/app.json
{
  "userIgnoreFilters": [
    ".raw/",
    ".vault-meta/",
    "raw/",
    "repos/",
    "graphs/",
    "graphify-out/",
    ".worktrees/",
    ".venv/",
    "node_modules/"
  ],
  "newLinkFormat": "absolute",
  "useMarkdownLinks": true,
  "alwaysUpdateLinks": false
}


### .obsidian/graph.json
{
  "search": "path:wiki",
  "showAttachments": false,
  "showArrow": true,
  "showOrphans": true
}

