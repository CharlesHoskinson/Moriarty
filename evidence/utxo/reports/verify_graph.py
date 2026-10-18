from pathlib import Path
import json,datetime
out=Path(__file__).parent/'graphify-out'
x=json.loads((out/'graph.json').read_text()); ids={n['id'] for n in x['nodes']}; links=x.get('links') or x.get('edges',[])
bad=[e for e in links if e['source'] not in ids or e['target'] not in ids]
assert not bad, len(bad)
assert len(ids)==len(x['nodes'])
inv=json.loads((out/'SOURCE-FILES.json').read_text()); inventory={n['id']:n for n in x['nodes'] if n.get('coverage_layer')=='source_inventory'}
missing=[r['path'] for r in inv if r['id'] not in inventory or inventory[r['id']].get('sha256')!=r['sha256'] or inventory[r['id']].get('source_file')!=r['path']]
assert not missing, len(missing)
result={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'nodes':len(ids),'export_relationships':len(links),'dangling_export_edges':len(bad),'inventory_files':len(inv),'missing_inventory_hashes':len(missing),'semantic_model_requested':'gpt-6-luna','semantic_reviewed_files':20,'qualification':'Exact retained-file inventory and structural links plus targeted Luna semantics plus Grok native-evidence nodes; Luna did not review Grok native facts; not full-body semantic review.'}
(out/'EXPORT-VERIFICATION.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
