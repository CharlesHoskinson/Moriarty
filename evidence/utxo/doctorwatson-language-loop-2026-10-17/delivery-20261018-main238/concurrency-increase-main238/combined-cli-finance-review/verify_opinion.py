#!/usr/bin/env python3
"""Validate a whole returned review and its literal quotes; never create a verdict."""
from pathlib import Path
import hashlib
import json
import sys
import jsonschema

D = Path(__file__).resolve().parent
opinion = json.loads(Path(sys.argv[1]).read_text())
schema = json.loads((D/'WHOLE-OPINION.schema.json').read_text())
jsonschema.Draft202012Validator(schema).validate(opinion)
primary = (D/'PRIMARY.md').read_bytes()
index = json.loads((D/'PRIMARY-INDEX.json').read_text())
bodies = {}
for row in index:
    offset = primary.index(f"<BEGIN {row['id']}>\n".encode()) + len(f"<BEGIN {row['id']}>\n".encode())
    data = primary[offset:offset + row['bytes']]
    bodies[(row['logicalPath'], row['sha256'])] = data
    bodies[(row['id'], row['sha256'])] = data
    bodies[(row['logicalPath'].removeprefix(str(D) + '/'), row['sha256'])] = data
for field in ['sourceAssessment', 'actualResultAssessment']:
    for name, assessment in opinion[field]['predicates'].items():
        for q in assessment['quotes']:
            key = (q['path'], q['sha256'])
            assert key in bodies, f'{field}/{name}: unknown quote body {key}'
            lines = bodies[key].decode().splitlines()
            assert q['lineEnd'] >= q['lineStart']
            exact = '\n'.join(lines[q['lineStart']-1:q['lineEnd']])
            assert q['exactQuote'] == exact, f'{field}/{name}: nonliteral or wrong-range quote'
print(json.dumps({'schemaAndLiteralQuoteValidation': 'PASS', 'verdict': opinion['verdict'],
                  'rootReviewToken': opinion['rootReviewToken'],
                  'meaning': 'Mechanical transport/schema/quote validation only; substantive review and reviewer identity require root assessment.'}, indent=2))
