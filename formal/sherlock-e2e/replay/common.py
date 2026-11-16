"""Shared helpers for the Quint -> oracle projections (Formal-Quint lead).

A projector reads the Quint ITF traces of one context (model vocabulary) and
writes, per trace and per projected entity, an ITF trace in the oracle
vocabulary: every state holds every oracle field plus `op` (the oracle
operation that produced it). The expected values are computed from the MODEL
state, never from the oracle. quint_replay.py then replays these traces
through the oracle's --step mode.

Model transitions that the spec says are refusals are written as refusal
checks (`refusals.jsonl`: state, op, expected codes); check_refusals.py sends
each to the oracle and requires a refusal whose codes include the expected one.
"""
import json, pathlib, re

def itf(v):
    if isinstance(v, list): return [itf(x) for x in v]
    if not isinstance(v, dict): return v
    if '#bigint' in v: return int(v['#bigint'])
    if '#tup' in v: return [itf(x) for x in v['#tup']]
    if '#set' in v: return [itf(x) for x in v['#set']]
    if '#map' in v: return {json.dumps(itf(k)) if not isinstance(itf(k),(str,int)) else itf(k): itf(x) for k, x in v['#map']}
    if 'tag' in v and 'value' in v and len(v) == 2: return {'tag': v['tag'], 'value': itf(v['value'])}
    return {k: itf(x) for k, x in v.items()}

def load(path):
    t = json.load(open(path))
    out = []
    for s in t['states']:
        out.append({k.split('::')[-1]: itf(x) for k, x in s.items() if k != '#meta'})
    return out

def tag(v):
    return v['tag'] if isinstance(v, dict) and 'tag' in v else v

def write_trace(path, states):
    path = pathlib.Path(path); path.parent.mkdir(parents=True, exist_ok=True)
    js = {'#meta': {'format': 'ITF', 'source': 'formal-quint projection'},
          'vars': sorted(states[0].keys()),
          'states': [{'#meta': {'index': i}, **s} for i, s in enumerate(states)]}
    path.write_text(json.dumps(js))

class Out:
    def __init__(self, outdir):
        self.dir = pathlib.Path(outdir); self.dir.mkdir(parents=True, exist_ok=True)
        for f in self.dir.glob('*.itf.json'): f.unlink()
        self.ref = open(self.dir / 'refusals.jsonl', 'w'); self.n = 0; self.nref = 0; self.skipped = {}
    def chain(self, name, states):
        if len(states) >= 2:
            write_trace(self.dir / f'{name}.itf.json', states); self.n += 1
    def refusal(self, name, state, op, codes):
        self.ref.write(json.dumps({'name': name, 'state': state, 'op': op, 'expect': codes}) + '\n'); self.nref += 1
    def skip(self, why):
        self.skipped[why] = self.skipped.get(why, 0) + 1
    def close(self):
        self.ref.close()
        (self.dir / 'SUMMARY.json').write_text(json.dumps({'chains': self.n, 'refusals': self.nref, 'skipped': self.skipped}, indent=1))
        print(json.dumps({'chains': self.n, 'refusals': self.nref, 'skipped': self.skipped}))
