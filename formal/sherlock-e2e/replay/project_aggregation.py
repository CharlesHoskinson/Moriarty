#!/usr/bin/env python3
"""Aggregation: project Quint traces (catalog of 10 N1 shards, 5 runs, MERGE-WORK keys, foreign units)
onto the oracle (one SEG accumulator of one run).
Entities: run r (1..5): the oracle merges the run's leaves in step order when the model's N1 records of r
form a prefix; the model's SEAL (sealRunNodeTree, proveGp2Seal*) or its terminal N3 root read by L (NODE-003:
the root L reads is the sealed claim) is the oracle seal; proveLRun is lCheck. MERGE-WORK key k: keygenWork.
Foreign units: foreignUnit. Forgeries the model refuses become refusal checks.
usage: project_aggregation.py TRACEDIR OUTDIR"""
import sys, glob, os
sys.path.insert(0, os.path.dirname(__file__))
from common import load, tag, Out

N1 = {1: (1, 0, 0, 8, 1000, 1008), 2: (1, 0, 8, 12, 1008, 1012), 3: (2, 3, 0, 4, 2000, 2004), 4: (2, 3, 4, 8, 2004, 2008),
      5: (3, 0, 0, 4, 3000, 3004), 6: (3, 3, 4, 8, 3004, 3008), 7: (4, 0, 0, 8, 1000, 1008), 8: (4, 0, 8, 12, 1008, 1012),
      9: (5, 0, 0, 4, 5000, 5004), 10: (6, 0, 0, 4, 1000, 1004)}   # n1Id -> (run, chipSet, hStart, hEnd, preD, postD)
RUN = {1: (1, 1001, 77, [1, 2]), 2: (2, 1002, 77, [3, 4]), 3: (3, 1003, 77, [5, 6]), 4: (1, 1001, 0, [7, 8]), 5: (4, 1004, 77, [9])}
OUT = {1: 501, 2: 502, 3: 503, 4: 501, 5: 505}
NEG = bool(os.environ.get('NEGATIVE_CONTROL'))

def base(r):
    prog, kr, rr, _ = RUN[r]
    return dict(runProg=prog, keyRoot=kr, regRoot=rr, sMax=16, mwColumns=5121, nodeArity=2, segOn=False, segPre=0, segPost=0,
                segLo=0, segHi=0, chipSets=0, leaves=0, works=0, depsD=0, sealed=False, claimSteps=0, outcome=0, lAccepted=False,
                mwKeyOk=False, foreignUnits=0, suspended=False, ckPost=0, ckHi=0, cKind=0, cProg=prog, cKeyRoot=kr, cPre=0, cPost=0,
                cHPre=0, cSteps=0, cAirs=7, cRegRoot=0, cChipSet=0, cForged=0, cKeyRow=0, cStatus=0, cOutcome=0, cChain=0, cManifestOk=True)

def cand(s, n):
    _, cs, h0, h1, pre, post = N1[n]
    if NEG: post += 1
    return {**s, 'cKind': 0, 'cPre': pre, 'cPost': post, 'cHPre': h0, 'cSteps': h1 - h0, 'cChipSet': cs, 'cRegRoot': 0, 'cForged': 0, 'cAirs': 7}

def nodes(st):
    return {int(k): v for k, v in st['nodeProofs'].items()}

def n1_present(st, r):
    nd = nodes(st)
    return [n for n in RUN[r][3] if (10 + n) in nd]

def sealed_model(st, r):
    nd = nodes(st); lr = {int(k) for k in st['lRunProofs']}
    sid = 45 if r == 5 else 40 + r
    return sid in nd or r in lr

def main(tdir, odir):
    out = Out(odir)
    for f in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(f)[:-9]; S = load(f)
        for r in RUN:
            cur = base(r); chain = [dict(cur, op=-1)]; merged = 0; consumed = False
            def push(state, op):
                chain.append(dict(state, op=op))
            for i in range(1, len(S)):
                st = S[i]; op = st['lastOp']
                pres = n1_present(st, r)
                # merges in step order, for the prefix of the run's N1 shards the model has proved
                while merged < len(RUN[r][3]) and RUN[r][3][merged] in pres:
                    n = RUN[r][3][merged]; c = cand(cur, n); _, cs, h0, h1, pre, post = N1[n]
                    nxt = {**c, 'segOn': True, 'segPost': N1[n][5], 'segHi': h1, 'leaves': merged + 1, 'chipSets': cur['chipSets'] | (1 << cs)}
                    if merged == 0: nxt.update(segPre=pre, segLo=h0)
                    # the candidate sits in the pre-state; quint_replay sends the pre-state with the op
                    chain[-1] = {**chain[-1], **{k: c[k] for k in c if k.startswith('c')}}
                    cur = nxt; push(cur, 0); merged += 1
                if sealed_model(st, r) and not cur['sealed'] and merged == len(RUN[r][3]):
                    nd = nodes(st)
                    if r == 5 and not consumed:
                        chain[-1] = {**chain[-1], 'cStatus': 1, 'cKind': 0}
                        cur = {**cur, 'cStatus': 1, 'cKind': 0, 'depsD': 2}; push(cur, 4 * 16 + 1); consumed = True
                        chain[-1] = {**chain[-1], 'cChain': 2}
                        cur = {**cur, 'cChain': 2}
                    o = OUT[r]
                    chain[-1] = {**chain[-1], 'cStatus': 1, 'cOutcome': o}
                    cur = {**cur, 'cStatus': 1, 'cOutcome': o, 'sealed': True, 'claimSteps': cur['segHi'], 'outcome': o}
                    push(cur, 5 * 16 if r == 5 else 3 * 16)
                if op == 'proveLRun' and st['lastArgs']['primaryId'] == r and cur['sealed']:
                    chain[-1] = {**chain[-1], 'cOutcome': OUT[r]}
                    cur = {**cur, 'cOutcome': OUT[r], 'lAccepted': True}; push(cur, 6 * 16)
                if r == 1:
                    a = st['lastArgs']; f0 = {k: v for k, v in base(1).items()}
                    if op == 'refuseMissingChildAir':
                        out.refusal(f'{name}/s{i}/{op}', {**cand(f0, 1), 'cAirs': 3}, 0, ['air-missing'])
                    elif op == 'refuseN1WrongKeyRoot':
                        out.refusal(f'{name}/s{i}/{op}', {**cand(f0, 1), 'cKeyRoot': 1999}, 0, ['key-root-mismatch', 'prog-d-mismatch'])
                    elif op in ('refuseT10qForgery', 'refuseGp2C21Forgery'):
                        out.refusal(f'{name}/s{i}/{op}', {**cand(f0, 1), 'cForged': 1 if op == 'refuseT10qForgery' else 5}, 0, ['forged-witness'])
                    elif op == 'refuseDisjointOrSwappedChildren':
                        s1 = {**f0, 'segOn': True, 'segPre': 1000, 'segPost': 1008, 'segLo': 0, 'segHi': 8, 'leaves': 1, 'chipSets': 1}
                        out.refusal(f'{name}/s{i}/{op}', cand(s1, 1), 0, ['link-broken'])
                    elif op == 'refuseLWrongOutcome':
                        s1 = {**f0, 'segOn': True, 'segPre': 1000, 'segPost': 1012, 'segLo': 0, 'segHi': 12, 'leaves': 2, 'chipSets': 1,
                              'sealed': True, 'claimSteps': 12, 'outcome': 501, 'cOutcome': 999}
                        out.refusal(f'{name}/s{i}/{op}', s1, 6 * 16, ['outcome-mismatch'])
                    elif op == 'refuseMixedRegRootJoin':
                        out.refusal(f'{name}/s{i}/{op}', {**cand(f0, 1), 'cKind': 1, 'cRegRoot': 78}, 0, ['reg-root-mismatch'])
                    elif op == 'refuseGp2ForgedDep':
                        b5 = base(5)
                        s5 = {**b5, 'segOn': True, 'segPre': 5000, 'segPost': 5004, 'segLo': 0, 'segHi': 4, 'leaves': 1, 'chipSets': 1,
                              'depsD': 2, 'cStatus': 1, 'cChain': 999, 'cOutcome': 505}
                        out.refusal(f'{name}/s{i}/{op}', s5, 5 * 16, ['deps-mismatch'])
                    elif op == 'refuseMergeWorkKeyOverCols':
                        k = a['primaryId']; out.refusal(f'{name}/s{i}/{op}', {**f0, 'mwColumns': 5401 if k == 3 else 5417}, 2 * 16, ['columns'])
                    elif op == 'refuseUnlistedFvUnit':
                        out.refusal(f'{name}/s{i}/{op}', {**f0, 'cKind': 2, 'cKeyRow': 9}, 7 * 16, ['key-row'])
                    elif op not in ('proveN1', 'proveN2Merge', 'proveN3Root', 'proveRunNodeTree', 'sealRunNodeTree', 'proveLRun',
                                    'proveGp2Seal', 'proveGp2SealWithDeps', 'admitMergeWorkKey', 'proveN3ForeignUnit'):
                        out.skip(f'{op}: no oracle operation')
            out.chain(f'{name}_run{r}', chain)
        # MERGE-WORK keys and foreign units
        ch = [dict(base(1), op=-1)]; seen = set()
        fv = [dict(base(1), op=-1)]
        for i in range(1, len(S)):
            st = S[i]; op = st['lastOp']
            if op == 'admitMergeWorkKey':
                k = st['lastArgs']['primaryId']; cols = st['mwKeyRegistry'][str(k)]['colCount'] if str(k) in st['mwKeyRegistry'] else st['mwKeyRegistry'][k]['colCount']
                s = {**base(1), 'mwColumns': cols}
                out.chain(f'{name}_mwkey{k}_s{i}', [dict(s, op=-1), dict(s, mwKeyOk=True, op=2 * 16)])
            if op == 'proveN3ForeignUnit':
                row = st['lastArgs']['primaryId']
                fv[-1] = {**fv[-1], 'cKind': 2, 'cKeyRow': 6 + row}
                n = len([k for k in nodes(st) if 61 <= k <= 62])
                fv.append({**fv[-1], 'foreignUnits': n, 'op': 7 * 16})
        out.chain(f'{name}_foreign', fv)
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
