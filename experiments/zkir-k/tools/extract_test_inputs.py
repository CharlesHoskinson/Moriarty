"""Attach the crate's own test preimages to corpus/ledger9-92e8bdd3-tests/manifest.json.

For every program extracted from zkir-v3/tests/*.rs at midnight-ledger 92e8bdd3,
find the `ProofPreimage { .. }` literal (or `assert_*` call) that follows it in
the same test function and record the integer-literal parts of `inputs`,
`private_transcript`, `public_transcript_inputs`, `public_transcript_outputs`
and `binding_input`. Vectors that contain computed values (point coordinates,
hashes) are left out; the differential harness then falls back to generated
inputs for those programs.
"""
import json
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
MANIFEST = HERE.parent / 'corpus' / 'ledger9-92e8bdd3-tests' / 'manifest.json'
LEDGER = Path.home() / 'Moriarty/repos/midnightntwrk/midnight-ledger'
FIELDS = ['inputs', 'private_transcript', 'public_transcript_inputs', 'public_transcript_outputs']
LIT = re.compile(r'^\s*\(?(-?\d+)\)?\s*\.into\(\)\s*$')


def literal_vec(text: str):
    text = text.strip()
    if text == '':
        return []
    parts = [p for p in text.split(',') if p.strip()]
    out = []
    for p in parts:
        m = LIT.match(p)
        if not m:
            return None
        out.append(int(m.group(1)))
    return out


manifest = json.loads(MANIFEST.read_text())
sources = {}
for entry in manifest['programs']:
    f = entry['source']
    if f not in sources:
        sources[f] = subprocess.run(['git', '-C', str(LEDGER), 'show', f'92e8bdd3:{f}'], capture_output=True, text=True, check=True).stdout
    src = sources[f]
    lines = src.split('\n')
    start = entry['line'] - 1
    # the test function body: up to the next `fn ` after the program
    end = next((i for i in range(start + 1, len(lines)) if re.match(r'\s*(async )?fn ', lines[i])), len(lines))
    body = '\n'.join(lines[start:end])
    pre = {}
    for field in FIELDS:
        m = re.search(r'(?<![A-Za-z0-9_])' + field + r':\s*vec!\[(.*?)\]', body, re.S)
        if m:
            vals = literal_vec(m.group(1))
            if vals is not None:
                pre[field] = [str(v) for v in vals]
    m = re.search(r'binding_input:\s*\(?(\d+)\)?\.into\(\)', body)
    if m:
        pre['binding_input'] = m.group(1)
    # assert_check_err_contains(ir, vec![...], ..) / assert_typed_output_roundtrip(ir, vec![...], ..)
    m = re.search(r'(assert_check_err_contains|assert_typed_output_roundtrip)\(\s*r#"', body)
    if m and 'inputs' not in pre:
        m2 = re.search(r'"#,\s*vec!\[(.*?)\]', body, re.S)
        if m2:
            vals = literal_vec(m2.group(1))
            if vals is not None:
                pre['inputs'] = [str(v) for v in vals]
    if 'inputs' in pre:
        entry['test_preimage'] = pre
    else:
        entry.pop('test_preimage', None)
MANIFEST.write_text(json.dumps(manifest, indent=2) + '\n')
have = [e['file'] for e in manifest['programs'] if 'test_preimage' in e]
print(len(have), 'programs with literal test inputs:', ', '.join(have))
