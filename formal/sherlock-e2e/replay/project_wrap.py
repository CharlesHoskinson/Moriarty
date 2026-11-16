#!/usr/bin/env python3
"""Wrap: project Quint traces (keygen attempts, one CPU wrap key, L_Fr call events, proof-run records,
deployments, T25 LW qualification) onto the oracle (one ledger call through the phase machine
askK -> cost -> keygen -> expect -> decode -> prove, then deploy / gpuProve). Ops: c * 10^9 + a.
Chains:
- every new keygen attempt (keygenWrap, refuseKeygenAboveK21, refuseKeygenRowGate, refuseVkTooLarge): from
  init: askK k, cost rows, keygen 1 bytes, as far as the model got (a stop ends the chain).
- every new proof-run record (proveWrapCpu, proveWrapGpu, proveSeededDetWrap, refuseExpectVkMismatch): from the
  keyed state of the model's CPU key (phase 3): expect (same unless E6_EXPECT_VK pins another key), decode
  (lane 6, lane 15, mode 0 ORD / 1 chunk, template = chunk write prologue), prove (library verifier) [, gpuProve
  the GPU key]; draws = the model's freshBlinderSeq; testSeed from the build.
- refuseInvalidWrapWitness: the slot forgeries (lane 6 or 15 not 15, ORD with the write template) stop at decode;
  the other forged L_Fr witnesses stop at the library verifier (prove false).
- recordT25LwQualificationPasses: qualify i for LW-003, LW-004, LW-005, LW-006, LW-014 (i = 0..4).
- deployProgramLedger: from a proved call (phase 6) with the model's record bits: deploy.
- refuseIncompleteT25Qualification (not the all-NOP bench): deploy refused (lastRefusal 8).
Skipped (no oracle operation): LW-014 keygen refusal, GPU and E6_EXPECT_VK configuration, call receipt,
rekey measurement, T25 close, seeded-blinder refusal in production (a build property).
usage: project_wrap.py TRACEDIR OUTDIR"""
import sys, glob, os
sys.path.insert(0, os.path.dirname(__file__))
from common import load, tag, Out

NEG = bool(os.environ.get('NEGATIVE_CONTROL'))
B = 10 ** 9
LW = ['LW-003', 'LW-004', 'LW-005', 'LW-006', 'LW-014']

def init(ts=False):
    return dict(phase=0, k=0, rows=0, vkBytes=0, vkId=0, keygenRan=False, vkFileWritten=False, files=0, proofs=0, stop=0,
                lastRefusal=0, anchorRead=0, chunkWrite=0, mode=0, template=False, testSeed=ts, draws=0, lastBlinder=0,
                prevBlinder=0, qual=0, eraVk=0, keysDeployed=0, deployed=0, gpuVk=0)

def qual(w):
    return sum(1 << i for i, n in enumerate(LW) if n in w['t25PassedChecks'])

def keyed(w, ts=False):
    k = w['cpuWrapKey']; d = w['freshBlinderSeq']
    return dict(init(ts), phase=3, k=k['circuitK'], rows=k['measuredRows'], vkBytes=k['vkBytes'], vkId=k['vkId'], keygenRan=True,
                draws=d, lastBlinder=0 if ts else d, prevBlinder=0 if ts else max(d - 1, 0), qual=qual(w))

def stop(s, c, **kw): return dict(s, phase=7, stop=c, **kw)

def new(c, p, f): return [x for x in c[f] if x not in p[f]]

def call_chain(s, ev, mode, accepted):
    """expect -> decode -> prove from keyed state s (expect already applied by the caller)."""
    r, wk, m, t = ev['lane6ReadKey'], ev['lane15WriteKey'], 0 if tag(ev['callMode']) == 'OrdMode' else 1, ev['hasChunkWritePrologue']
    s2 = dict(s, anchorRead=r, chunkWrite=wk, mode=m, template=t)
    op = 5 * B + r + 100 * wk + 10000 * m + 100000 * (1 if t else 0)
    if not (r == 15 and wk == 15 and not (m == 0 and t)):
        return [dict(stop(s2, 5), op=op)], None
    s2 = dict(s2, phase=5); out = [dict(s2, op=op)]
    d = s2['draws'] + 1
    s3 = dict(s2, proofs=1, draws=d, prevBlinder=s2['lastBlinder'], lastBlinder=0 if s2['testSeed'] else d)
    if accepted:
        s3 = dict(s3, phase=6, files=3 + (1 if NEG else 0), vkFileWritten=True)
    else:
        s3 = stop(s3, 6)
    out.append(dict(s3, op=6 * B + (1 if accepted else 0)))
    return out, s3

def main(tdir, odir):
    out = Out(odir)
    for fn in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(fn)[:-9]; S = load(fn)
        for i in range(1, len(S)):
            c, p = S[i]['wrp'], S[i - 1]['wrp']; op = S[i]['lastOp']; nm = f'{name}_s{i}'
            if op in ('keygenWrap', 'refuseKeygenAboveK21', 'refuseKeygenRowGate', 'refuseVkTooLarge'):
                a = new(c, p, 'keygenAttempts')
                if not a: out.skip('keygen: duplicate record'); continue
                a = a[0]; s = init(); ch = [dict(s, op=-1)]
                k, rows, vb = a['requestedK'], a['measuredRows'], a['vkBytes']
                if op == 'refuseKeygenAboveK21':
                    ch.append(dict(stop(s, 1, k=k), op=1 * B + k))
                else:
                    s = dict(s, phase=1, k=k); ch.append(dict(s, op=1 * B + k))
                    if op == 'refuseKeygenRowGate':
                        ch.append(dict(stop(s, 2, rows=rows), op=2 * B + rows))
                    else:
                        s = dict(s, phase=2, rows=rows); ch.append(dict(s, op=2 * B + rows))
                        if vb >= 100000: out.skip('keygen: VK bytes outside the op arg'); continue
                        s = dict(s, keygenRan=True, vkBytes=vb, vkId=1)
                        ch.append(dict(stop(s, 3) if op == 'refuseVkTooLarge' else dict(s, phase=3), op=3 * B + 100000 + vb))
                out.chain(nm, ch)
            elif op in ('proveWrapCpu', 'proveWrapGpu', 'proveSeededDetWrap', 'refuseExpectVkMismatch'):
                r = new(c, p, 'proofRuns')
                if not r: out.skip(f'{op}: duplicate record'); continue
                r = r[0]; ts = tag(r['buildMode']) == 'TestSeedBuild'
                s = keyed(p, ts); ch = [dict(s, op=-1)]
                same = not p['expectVkSet'] or p['cpuWrapKey']['vkId'] == p['expectedVkId']
                if op == 'refuseExpectVkMismatch' or not same:
                    ch.append(dict(stop(s, 4), op=4 * B)); out.chain(nm, ch); continue
                s = dict(s, phase=4); ch.append(dict(s, op=4 * B + 1))
                cc, s3 = call_chain(s, r, None, r['libraryVerified'])
                ch += cc
                if s3 and tag(r['device']) == 'GpuDevice' and s3['phase'] == 6:
                    v = r['vkId']
                    ch.append(dict(s3, gpuVk=v, op=9 * B + v) if v == s3['vkId'] else dict(s3, lastRefusal=10, op=9 * B + v))
                out.chain(nm, ch)
            elif op == 'refuseInvalidWrapWitness':
                cid = [x for x in p['knownCallIds']]
                evs = p['lfrEvents']; bad = None
                for k_, ev in evs.items():
                    ok = (ev['lfrValidInFr'] and ev['lfrVkMatchesFrozen'] and not ev['leaksPrivateWord'] and ev['fieldLogValid']
                          and ev['dagXCanonAndEqCached'] and not ev['anchorHoldsPlainProgD'] and tag(ev['aInTag']) == 'TagProgc'
                          and ev['lane6ReadKey'] == 15 and (ev['hasChunkWritePrologue'] == (tag(ev['callMode']) == 'ChunkMode'))
                          and (not ev['hasChunkWritePrologue'] or ev['lane15WriteKey'] == 15))
                    if not ok: bad = ev; break
                if bad is None: out.skip('witness refusal: no forged event found'); continue
                s = dict(keyed(p), phase=4); ch = [dict(s, op=-1)]
                cc, _ = call_chain(s, bad, None, False); ch += cc
                out.chain(nm, ch)
            elif op == 'recordT25LwQualificationPasses':
                s = keyed(p); ch = [dict(s, op=-1)]; q = s['qual']
                for idx, n in enumerate(LW):
                    if n in c['t25PassedChecks'] and not q >> idx & 1:
                        q |= 1 << idx; ch.append(dict(s, qual=q, op=7 * B + idx))
                if len(ch) < 2: out.skip('qualify: nothing new'); continue
                out.chain(nm, ch)
            elif op in ('deployProgramLedger', 'refuseIncompleteT25Qualification'):
                if op == 'refuseIncompleteT25Qualification' and c['lastRefusalReason'] == 'all_nop_bench_qualifies_no_gate':
                    out.skip('all-NOP bench: no oracle input'); continue
                s = dict(keyed(p), phase=6, proofs=1, files=3, vkFileWritten=True, anchorRead=15, chunkWrite=15, mode=1, template=True)
                era = sorted(p['eraWrapVkSet']); s = dict(s, eraVk=era[0] if era else 0, keysDeployed=1 if era else 0,
                                                          deployed=len(p['deployments']))
                if s['deployed'] and not s['eraVk']: out.skip('deploy: deployments without an era key'); continue
                if op == 'refuseIncompleteT25Qualification':
                    out.chain(nm, [dict(s, op=-1), dict(s, lastRefusal=8, op=8 * B)]); continue
                d = new(c, p, 'deployments')
                if not d: out.skip('deploy: no new record'); continue
                ok = d[0]['installedWriteVkId'] == s['vkId']
                post = dict(s, deployed=s['deployed'] + 1, keysDeployed=1, eraVk=s['vkId']) if ok and s['qual'] == 31 else \
                       dict(s, lastRefusal=8 if s['qual'] != 31 else 9)
                if ok and s['qual'] != 31:
                    post = dict(s, deployed=s['deployed'] + 1, keysDeployed=1, eraVk=s['vkId'])  # model deploys; oracle per WRP-012
                out.chain(nm, [dict(s, op=-1), dict(post, op=8 * B)])
            else:
                out.skip(f'{op}: no oracle operation')
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
