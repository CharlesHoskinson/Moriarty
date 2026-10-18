# Root mechanical validation

Run from this output directory. The frozen packet is `../bridge-caller-graph-packet/`.

1. Run `graphify diagnose multigraph --graph graph.json --json`; expect zero dangling endpoints, zero directed endpoint collapses, and zero unverified nodes.
2. Use the standard-library-only Python check below to verify packet hashes, every citation excerpt and every edge endpoint.
3. Compare `SOURCE-CHECKS.json.old_output_sha256` with the prior output files if confirming immutability.

```python
from pathlib import Path
import hashlib, json
packet = Path('../bridge-caller-graph-packet')
manifest = json.loads((packet/'MANIFEST.json').read_text())
files = {f['path']: f for f in manifest['files']}
graph = json.loads(Path('graph.json').read_text())
for f in files.values():
    body = (packet/f['path']).read_bytes()
    assert len(body) == f['bytes']
    assert hashlib.sha256(body).hexdigest() == f['sha256']
for obj in graph['nodes'] + graph['edges']:
    citations = obj.get('source_citations') or [obj]
    for c in citations:
        f = files[c['packet_file']]
        assert c['source_file'] == f['source'] and c['source_sha256'] == f['sha256']
        assert (packet/c['packet_file']).read_text().splitlines()[c['line']-1].strip() == c['excerpt']
ids = {n['id'] for n in graph['nodes']}
assert len(ids) == len(graph['nodes'])
assert all(e['source'] in ids and e['target'] in ids for e in graph['edges'])
cfg_calls = [e for e in graph['edges'] if e.get('scope') == 'cfg(test) source only' and e.get('relation') == 'calls']
assert {(e['source'], e['target'], e['line']) for e in cfg_calls} == {
    ('factorial-source-test', 'factorial-expected', 155),
    ('factorial-source-test', 'factorial-provenance', 155),
    ('treefold-source-test', 'treefold-expected', 161),
    ('treefold-source-test', 'treefold-provenance', 161),
}
```

The four `calls` edges above are intentional: each source line contains nested calls to the expected-AST and provenance helper functions as arguments to `check_parse`. The graph separately labels the outer test-to-`check_parse` relationship `test_calls`. Both relations describe source syntax in `cfg(test)`; neither says a test was executed. Do not apply a blanket rule that forbids `calls` edges in test-only source.
