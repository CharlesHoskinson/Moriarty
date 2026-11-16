#!/usr/bin/env python3
"""Leaf Proving: project Quint traces (one world of 8 programs, 6 runs) onto the oracle (one run of one program).
Entity = one R_SL run of the model (runs 1,2,3,4,6; run 5 is the interpreted family, outside the oracle's LFP scope).
usage: project_leaf_proving.py TRACEDIR OUTDIR"""
import sys, glob, os
sys.path.insert(0, os.path.dirname(__file__))
from common import load, tag, Out

# Model catalog (leaf_proving.qnt programCatalog / runCatalog), main instance: SHARD_STEP_CAP 4, MAX_SHARDS 3
PROG = {1: (5, False, False, 12000), 2: (5, False, False, 8000), 3: (5, False, False, 14000),
        4: (8, True, False, 16346), 5: (8, False, True, 30000), 7: (11, True, True, 22000), 8: (5, False, False, 30001)}
RUN = {1: (1, 1), 2: (2, 3), 3: (4, 2), 4: (5, 1), 6: (3, 1)}   # run -> (prog, shards)
CAP = 4
if os.environ.get('NEGATIVE_CONTROL'):  # wrong constant: d04 gets 9 app AIRs; the replay must DISAGREE
    PROG[4] = (9, True, False, 16346)

def base(p, shards):
    airs, sha, kec, cols = PROG[p]
    return dict(appAirs=airs, hasSha=sha, hasKeccak=kec, appCols=cols, nonCanonical=False, shapeOk=True,
                progD=p, inputD=11, tapeD=11, stmtD=13, fp1Outcome=13, steps=shards * CAP, shardCap=CAP,
                keyRoot=21, manifestD=1, admitted=False, shardsProved=0, appProofs=0, leafProofs=0, leafProgD=0,
                foreignLeaves=0, badLeaves=0, records=0, recordLeaves=0, merged=False)

def slots(m, r):
    return [int(k) for k in m if int(k) // 10 == r]

def view(st, r):
    """Oracle-vocabulary view of run r, computed from the model state."""
    p, shards = RUN[r]
    s = base(p, shards)
    s['admitted'] = p in st['admittedProgs']
    n = len(slots(st['leafProofs'], r)); s['shardsProved'] = n
    s['appProofs'] = len(slots(st['appProofs'], r)); s['leafProofs'] = n
    s['leafProgD'] = p if n else 0
    # D-leaf-2 (lead ruling): a leaf of another program is refused (LFP-007); it never joins the run
    pub = str(r) in st['readyEvents'] or r in st['readyEvents']
    s['records'] = 1 if pub else 0
    s['recordLeaves'] = len(st['readyEvents'].get(r, st['readyEvents'].get(str(r), {})).get('entries', [])) if pub else 0
    acc = st['n1Accepted'].get(r, st['n1Accepted'].get(str(r), []))
    s['merged'] = len(acc) > 0
    return s

TAMPER = {'RefWrongTapeInputD': ('tapeD', 12, 'input-mismatch'), 'RefBadFp1Outcome': ('fp1Outcome', 14, 'statement-unbound'),
          'RefNonCanonicalWord': ('nonCanonical', True, 'noncanonical-word'), 'RefBadShardHeights': ('shapeOk', False, 'shape-mismatch')}

def main(tdir, odir):
    out = Out(odir)
    for f in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(f)[:-9]; S = load(f)
        for r, (p, shards) in RUN.items():
            chain = [dict(view(S[0], r), op=-1)]; merged_once = False
            for i in range(1, len(S)):
                op, a = S[i]['lastOp'], S[i]['lastArgs']
                cur = chain[-1]
                if op == 'admitProgram' and a['progId'] == p:
                    chain.append(dict(view(S[i], r), op=0))
                elif op == 'proveRslShard' and a['runId'] == r:
                    chain.append(dict(view(S[i], r), op=16))
                elif op == 'publishLeafProofsReady' and a['runId'] == r:
                    chain.append(dict(view(S[i], r), op=48))
                elif op == 'n1AcceptLeaf' and a['runId'] == r:
                    if not merged_once: chain.append(dict(view(S[i], r), op=64)); merged_once = True
                    else: out.skip('n1AcceptLeaf after the first (oracle merges once)')
                elif op == 'n1RefuseWrongProgD' and a['runId'] == r:
                    other = RUN.get(a['slotId'] // 10, (6, 0))[0]
                    st = {k: x for k, x in cur.items() if k != 'op'}
                    out.refusal(f'{name}/run{r}/step{i}/n1RefuseWrongProgD', st, 2 * 16 + other, ['wrong-prog-d'])
                elif op == 'refuseRslShardTamper' and a['runId'] == r:
                    code = tag(a['refusal'])
                    if code in TAMPER:
                        fld, v, oc = TAMPER[code]; st = {k: x for k, x in cur.items() if k != 'op'}; st[fld] = v
                        out.refusal(f'{name}/run{r}/step{i}/{code}', st, 16, [oc])
                    else: out.skip(f'tamper {code}: no oracle field')
                elif op in ('refuseProgramOverAirs', 'refuseProgramOverCols') and r == 1:
                    q = a['progId']; st = base(q, 1)
                    codes = ['air-count-over-bound', 'mixed-hash', 'outside-bucket'] if op == 'refuseProgramOverAirs' else ['column-bound']
                    out.refusal(f'{name}/prog{q}/step{i}/{op}', st, 0, codes)
            out.chain(f'{name}_run{r}', chain)
        for i in range(1, len(S)):
            if S[i]['lastOp'] in ('proveInterpretedShard', 'refuseInterpretedShardTamper', 'runSuiteCallProve', 'recordSuiteCallBlocked'):
                out.skip(S[i]['lastOp'] + ' (outside oracle scope)')
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
