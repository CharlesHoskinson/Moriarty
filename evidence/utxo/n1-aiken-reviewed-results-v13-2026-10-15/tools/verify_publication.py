"""Read-only snapshot verification. Never import or execute captured code."""
from pathlib import Path
from html.parser import HTMLParser
from urllib.parse import urlsplit, unquote
import collections
import datetime
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[1]
EXCLUDED = {'preservation/REPRODUCIBILITY-MANIFEST.json', 'preservation/VERIFICATION.json'}


def read(path):
    return json.loads(path.read_text())


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class References(HTMLParser):
    def __init__(self):
        super().__init__()
        self.references = []

    def handle_starttag(self, tag, attrs):
        self.references.extend(value for key, value in attrs if key in {'src', 'href'} and value)


def main():
    manifest = read(ROOT / 'preservation/REPRODUCIBILITY-MANIFEST.json')
    identities = 0
    for row in manifest['files']:
        path = ROOT / row['path']
        assert path.is_file() and not path.is_symlink(), row['path']
        assert path.stat().st_size == row['bytes'], row['path']
        assert digest(path) == row['sha256'], row['path']
        identities += 1
    measured = {str(p.relative_to(ROOT)) for p in ROOT.rglob('*') if p.is_file()}
    assert measured - EXCLUDED == {r['path'] for r in manifest['files']}
    seeds = read(ROOT / 'preservation/SEED-COPY-MANIFEST.json')['files']
    assert len(seeds) == 2698 and sum(r['bytes'] for r in seeds) == 77526804
    for row in seeds:
        assert digest(ROOT / row['path']) == row['sha256']
    portable = read(ROOT / 'source/PORTABLE-LAYOUT.json')['files']
    for row in portable:
        assert (ROOT / row['snapshot']).read_bytes() == (ROOT / row['captured']).read_bytes()
    assert len(portable) == 70
    parent = read(ROOT / 'graph/frozen/parent/graph-v12.json')
    delta = read(ROOT / 'graph/frozen/revision-3/semantic-v13-luna.json')
    raw = read(ROOT / 'graph/graph.json')
    assert raw == read(ROOT / 'graph/graph-raw.json')
    assert raw['nodes'] == parent['nodes'] + delta['nodes']
    assert raw['links'] == parent['links'] + delta['edges']
    assert raw['hyperedges'] == parent['hyperedges'] + delta['hyperedges']
    assert {k:v for k,v in raw.items() if k not in {'nodes','links','hyperedges'}} == {
        k:v for k,v in parent.items() if k not in {'nodes','links','hyperedges'}}
    ids = [n['id'] for n in raw['nodes']]
    assert len(ids) == len(set(ids)) == 6177
    assert len(raw['links']) == 16856 and len(raw['hyperedges']) == 1
    assert all(e['source'] in ids and e['target'] in ids for e in raw['links'])
    objects = collections.Counter(json.dumps(e, sort_keys=True) for e in raw['links'])
    assert sum(n - 1 for n in objects.values()) == 20
    assert sum(e['source'] == e['target'] for e in raw['links']) == 5
    source_map = {r['source_file']:r for r in read(ROOT / 'graph/SOURCE-PATH-MAP.json')['files']}
    assert len(source_map) == 40 and sum(r['bytes'] for r in source_map.values()) == 468951
    bindings = spans = pointers = 0
    for item in delta['nodes'] + delta['edges']:
        row = source_map[item['source_file']]
        p = ROOT / row['portable']
        assert digest(p) == row['sha256'] == item['source_sha256']
        bindings += 1
        for start, end in re.findall(r'L(\d+)(?:-L?(\d+))?', item['source_location']):
            assert 1 <= int(start) <= int(end or start) <= len(p.read_text().splitlines())
            spans += 1
        for pointer in re.findall(r'\$(/[^;\s]+)', item['source_location']):
            value = read(p)
            for key in pointer.split('/')[1:]:
                key = key.replace('~1', '/').replace('~0', '~')
                value = value[int(key)] if isinstance(value, list) else value[key]
            pointers += 1
    assert (bindings, spans, pointers) == (115, 75, 92)
    analysis = read(ROOT / 'graph/graph-analysis.json')
    assert len(analysis['nodes']) == 6132 and len(analysis['links']) == 16025
    assert {n['id'] for n in delta['nodes']}.issubset({n['id'] for n in analysis['nodes']})
    local_html = 0
    remote_html = []
    for name in ['graph/graph.html', 'graph/SOURCE-INDEX.html']:
        parser = References()
        p = ROOT / name
        parser.feed(p.read_text())
        for ref in parser.references:
            url = urlsplit(ref)
            if url.scheme or url.netloc:
                remote_html.append(ref)
            elif url.path:
                assert (p.parent / unquote(url.path)).resolve().is_relative_to(ROOT)
                assert (p.parent / unquote(url.path)).is_file(), (name, ref)
                local_html += 1
    markdown_links = 0
    authored_readmes = ['README.md', 'REPORT.md', 'source/catalogue/README.md', 'source/aiken/README.md', 'graph/README.md']
    for name in authored_readmes:
        p = ROOT / name
        for ref in re.findall(r'\]\(([^)]+)\)', p.read_text()):
            url = urlsplit(ref)
            if not url.scheme and not url.netloc and url.path:
                target = p.parent / unquote(url.path)
                # Two existing sibling evidence references are repository inputs.
                if not target.resolve().is_relative_to(ROOT):
                    assert ref.startswith('../current-core-') or ref.startswith('../p3-r1-')
                    continue
                assert target.is_file(), (name, ref)
                markdown_links += 1
    print(json.dumps(dict(format='publication-mechanical-verification/1',
        observedAt=datetime.datetime.now(datetime.timezone.utc).isoformat(), status='PASS',
        manifestIdentities=identities, immutableSeedFiles=2698, immutableSeedBytes=77526804,
        portableSourceCopies=70, rawNodes=6177, rawLinks=16856, rawHyperedges=1,
        exactDuplicateRawObjects=20, rawSelfLoops=5, analysisNodes=6132, analysisEdges=16025,
        citations=bindings, lineSpans=spans, jsonPointers=pointers,
        localHtmlReferences=local_html, remoteHtmlReferences=sorted(set(remote_html)),
        authoredMarkdownLocalReferences=markdown_links,
        graphFullPayloadPreservation='PASS', capturedRuntimeRerun='UNRUN',
        portableCatalogueExecution='UNRUN', portableAikenExecution='UNRUN', browserRuntime='UNRUN',
        sourceSemanticSupport='PENDING_FRESH_NONAUTHOR_AUDIT', securityJudgment='NOT_PERFORMED',
        nativeProofLedger='OUTSIDE_SCOPE_NO_AUTHORITY_CREATED'), indent=2))


if __name__ == '__main__':
    main()
