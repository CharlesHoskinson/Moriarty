#!/usr/bin/env python3
"""Send each refusal check of refusals.jsonl to `ORACLE --step`; require a refusal that names the expected code.
usage: check_refusals.py DIR ORACLE...   exit 0 all agree, 1 a disagreement, 2 protocol error"""
import json, subprocess, sys, os
d, oracle = sys.argv[1], sys.argv[2:]
p = subprocess.Popen(oracle + ['--step'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env={**os.environ, 'LEAN_ABORT_ON_PANIC': '1'})
bad = n = 0
for line in open(os.path.join(d, 'refusals.jsonl')):
    r = json.loads(line); n += 1
    p.stdin.write(json.dumps({'state': r['state'], 'op': r['op']}) + '\n'); p.stdin.flush()
    a = json.loads(p.stdout.readline())
    if 'error' in a: print('ERROR', r['name'], a); sys.exit(2)
    ok = 'refused' in a and any(c in a['refused'] for c in r['expect'])
    if not ok:
        bad += 1
        if bad <= 10: print('DISAGREE', r['name'], 'expect', r['expect'], 'got', a.get('refused', 'accepted'), 'op', r['op'])
print(f'refusal checks: {n}, disagreements: {bad}')
sys.exit(1 if bad else 0)
