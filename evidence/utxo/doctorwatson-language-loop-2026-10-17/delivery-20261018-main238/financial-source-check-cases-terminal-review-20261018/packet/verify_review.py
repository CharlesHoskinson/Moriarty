"""Offline metadata/quotation checks; this does not authenticate reviewers."""
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent

def digest(raw):
    return hashlib.sha256(raw).hexdigest()

def read_json(path):
    return json.loads(path.read_bytes())

def delivered_path(path):
    if (ROOT / 'head' / path).is_file():
        return ROOT / 'head' / path
    if (ROOT / path).is_file():
        return ROOT / path
    return ROOT / 'context' / path

def verify(response):
    schema = read_json(ROOT / 'REVIEW-RESPONSE.schema.json')
    assert set(response) == set(schema['required']), 'exact response fields'
    for name in ['format', 'reviewToken', 'sourceManifestSha256',
                 'receiptManifestSha256', 'head', 'tree', 'base']:
        assert response[name] == schema['properties'][name]['const'], name
    assert response['sourceManifestSha256'] == digest((ROOT / 'SOURCE-PINS.json').read_bytes())
    assert response['receiptManifestSha256'] == digest((ROOT / 'RECEIPT-PINS.json').read_bytes())
    pins = read_json(ROOT / 'SOURCE-PINS.json')
    expected = {row['path']: row['sha256'] for row in pins['changed']}
    assert set(response['coverage']) == set(expected), 'all15 exact paths'
    accepting = response['sourceDisposition'] == 'accept-scoped-source'
    for path, expected_hash in expected.items():
        coverage = response['coverage'][path]
        assert coverage['sha256'] == expected_hash, path
        assert coverage['readScope'] in ['full', 'partial', 'unread'], path
        assert len(coverage['assessment']) >= 40, path
        if accepting:
            assert coverage['readScope'] == 'full', path
    blocks = {row['path']: row for row in read_json(ROOT / 'INLINE-BLOCKS.json')['blocks']}
    def quote(q):
        assert q['path'] in blocks, 'quote must cite delivered input'
        block = blocks[q['path']]
        assert q['sha256'] == block['sha256'], q['path']
        raw = delivered_path(q['path']).read_bytes()
        assert digest(raw) == q['sha256'], q['path']
        text = raw.decode()
        assert len(q['exactQuote']) >= 16 and q['exactQuote'] in text, 'meaningful exact source quotation'
        assert len(q['supports']) >= 40, 'explain what quotation establishes'
        if 'lineStart' in q:
            assert 1 <= q['lineStart'] <= q['lineEnd'] <= len(text.splitlines())
            selected = '\n'.join(text.splitlines()[q['lineStart'] - 1:q['lineEnd']])
            assert q['exactQuote'] in selected, 'quotation line attribution'
        else:
            assert 'jsonPointer' in q, 'line range or JSON pointer required'
            value = json.loads(text)
            pointer = q['jsonPointer']
            assert pointer == '' or pointer.startswith('/')
            for part in pointer.split('/')[1:]:
                part = part.replace('~1', '/').replace('~0', '~')
                value = value[int(part)] if isinstance(value, list) else value[part]
            candidate = value if isinstance(value, str) else json.dumps(value, ensure_ascii=False, indent=2)
            # The full raw quotation must also be locatable in the selected value.
            assert q['exactQuote'] in candidate or q['exactQuote'] in json.dumps(value, ensure_ascii=False), 'quotation JSON pointer attribution'
    for axis in ['sourceChecks', 'resultChecks']:
        assert set(response[axis]) == set(schema['properties'][axis]['required']), axis
        for value in response[axis].values():
            assert len(value['reason']) >= 100 and value['quotes'] and value['evidenceRefs']
            assert value['disposition'] in ['supported', 'defect', 'partial', 'unassessed']
            for q in value['quotes']:
                quote(q)
    for finding in response['findings']:
        assert finding['quotes'] and len(finding['predicate']) >= 30
        for q in finding['quotes']:
            quote(q)
    if accepting or response['resultDisposition'] == 'stored-six-case-results-consistent':
        reviewer = response['reviewer']
        assert reviewer['authorOfCandidate'] is False and reviewer['independent'] is True
        assert reviewer['terminalStatus'] == 'completed'
    assert response['retainedOpenRequirements'], 'retain parent obligations'
    return {'metadata': 'consistent', 'reviewerAuthentication': 'not established by this script',
            'semanticAdoption': 'requires substantive root review'}

if __name__ == '__main__':
    assert len(sys.argv) == 2, 'usage: python3 verify_review.py REVIEW.json'
    print(json.dumps(verify(read_json(Path(sys.argv[1])))))
