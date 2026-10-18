#!/usr/bin/env python3
"""Check immutable packet transport/hashes/anchors, not reviewer or product claims."""
from pathlib import Path
import hashlib
import json

D = Path(__file__).resolve().parent
sha = lambda b: hashlib.sha256(b).hexdigest()
primary = (D/'PRIMARY.md').read_bytes()
index = json.loads((D/'PRIMARY-INDEX.json').read_text())
assert len({row['id'] for row in index}) == len(index)
for row in index:
    marker = f"<BEGIN {row['id']}>\n".encode()
    assert primary.count(marker) == 1, row['id']
    offset = primary.index(marker) + len(marker)
    delivered = primary[offset:offset + row['bytes']]
    assert sha(delivered) == row['sha256'], row['id']
    assert primary[:offset].count(b'\n') + 1 == row['primaryBodyLineStart'], row['id']
    assert row['primaryBodyLineEnd'] == row['primaryBodyLineStart'] + len(delivered.splitlines()) - 1

coverage = json.loads((D/'FILE-COVERAGE.json').read_text())
assert len(coverage['changed']) == 18
assert len(coverage['baseModified']) == 5
assert len(coverage['baseline']) == 16
assert len(coverage['receipts']) == 43
anchors = json.loads((D/'ANCHORS.json').read_text())
assert set(anchors) == {row['logicalPath'] for row in coverage['changed']}
for path, anchor in anchors.items():
    data = (D/'candidate'/path).read_bytes()
    assert sha(data) == anchor['sha256']
    lines = data.decode().splitlines()
    assert '\n'.join(lines[anchor['lineStart']-1:anchor['lineEnd']]) == anchor['exactQuote']

subject_data = (D/'SUBJECT.json').read_bytes()
subject = json.loads(subject_data)
assert subject['contextPinsCanonicalSha256'] == sha(json.dumps(coverage['context'], sort_keys=True, separators=(',', ':')).encode())
assert subject['receiptPinsCanonicalSha256'] == sha(json.dumps(coverage['receipts'], sort_keys=True, separators=(',', ':')).encode())
assert subject['actualSourcePinsSha256'] == sha((D/'actual-receipts/SOURCE-PINS.json').read_bytes())
assert subject['checkPlanSha256'] == sha((D/'actual-receipts/CHECK-PLAN.json').read_bytes())
assert subject['finalResultSha256'] == sha((D/'actual-receipts/FINAL-RESULT.json').read_bytes())
schema = json.loads((D/'WHOLE-OPINION.schema.json').read_text())
assert schema['properties']['reviewedSubjectSha256']['const'] == sha(subject_data)
assert schema['properties']['rootReviewToken']['const'] == subject['rootReviewToken']
assert set(schema['properties']['coverage']['required']) == set(anchors)
assert set(schema['properties']['contextCoverage']['required']) == {row['id'] for row in coverage['context']}
assert len((D/'DESCRIPTOR.md').read_bytes()) <= 8192
try:
    import jsonschema
    jsonschema.Draft202012Validator.check_schema(schema)
except ModuleNotFoundError:
    raise RuntimeError('jsonschema is required to validate the packet schema')

freeze_entries = []
for line in (D/'FREEZE.sha256').read_text().splitlines():
    digest, path = line.split('  ', 1)
    assert sha((D/path).read_bytes()) == digest, path
    freeze_entries.append(path)
assert set(freeze_entries) == {str(p.relative_to(D)) for p in D.rglob('*') if p.is_file() and p.name != 'FREEZE.sha256'}
print(json.dumps({'packetIntegrity': 'PASS', 'fullPrimarySections': len(index),
                  'changedBodies': 18, 'modifiedBaseBodies': 5, 'baselineBodies': 16,
                  'contextBodies': len(coverage['context']), 'terminalReceiptBodies': 43,
                  'subjectSha256': sha(subject_data), 'primarySha256': sha(primary),
                  'schemaValid': True, 'descriptorBytes': len((D/'DESCRIPTOR.md').read_bytes()),
                  'all18LiteralAnchorsValid': True, 'allFreezeEntriesValid': len(freeze_entries)}, indent=2))
