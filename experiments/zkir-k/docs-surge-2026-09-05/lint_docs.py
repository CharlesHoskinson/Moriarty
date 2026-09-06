#!/usr/bin/env python3
"""Lint the chapters: placeholders, process traces, titles, em-dashes, notes sections."""
import re, sys, pathlib
DOCS = pathlib.Path(__file__).resolve().parent.parent / 'docs'
BAD = [r'\bTODO\b', r'\bTBD\b', r'\bFIXME\b', r'placeholder', r'coming soon', r'\bpersona\b', r'\bwave\b', r'\bdraft(ed|ing)?\b', r'\breviewer', r'\bauditor', r'\bagent\b', r'this document', r'—', r'\bCLM-\d{4}\b(?!.*plan)']
rc = 0
for f in sorted(DOCS.glob('*.md')):
    t = f.read_text()
    titles = [l for l in t.splitlines() if l.startswith('# ')]
    words = len(re.sub(r'```.*?```', '', t, flags=re.S).split())
    hits = []
    for i, l in enumerate(t.splitlines(), 1):
        for b in BAD:
            if re.search(b, l, re.I):
                hits.append((i, b, l.strip()[:90]))
    notes = '## Notes for maintainers' in t
    print(f'{f.name}: {words} words, {len(titles)} title(s), notes={notes}, {len(hits)} hits')
    for h in hits:
        print('   ', *h)
    if len(titles) != 1 or hits:
        rc = 1
sys.exit(rc)
