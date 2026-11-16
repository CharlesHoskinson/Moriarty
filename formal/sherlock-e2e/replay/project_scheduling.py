#!/usr/bin/env python3
"""Scheduling: project Quint traces (3 box profiles, prepared runs, proved-run records, early-wrap checks, joins)
onto the oracle (one box, one run). Units: memory in GB (memUnit 1), device memory in GB (MiB // 1024), leaf
time in ms. Calibration entry e = 2 * shape + dev (dev 0 GPU, 1 CPU); shapes 0 and 1 (caps 4 and 8 steps);
the model's shape 2 is outside key_root and has no oracle entry.
Entities and chains:
- runSetupOnBox b: hardware witness from the written profile (memAvail = RAM, cgroupHead = budget when below
  RAM) -> setup (0); compares memBudget, unified, gpuCount, gpuTotal, gpuFree, profCores, calMask.
- every new proved-run record (scheduleAndProveFromProfile, scheduleAndProveForcedCut): the box profile as the
  initial state (profKeyD = the profile's keysD), operator limits as witnesses -> startRun (16) [-> forceCut k
  (80+k) for the forced cut] -> prove (96); compares scheduled, calibrated, schShape, schDevice, schEst,
  schConc, runs, recShape, recDevice, recLeaves, recCalibrated.
- recordUncalibratedProfileKeyMismatch: stale profile (profKeyD = 100) -> startRun; compares calibrated.
- joinLeavesTwoBoxes -> join 0 (144); evaluateEarlyWrapGuard admitted -> earlyWrap (160) with policy 2,
  peakWrap = required MiB, starkExtra = margin = 0, wrapFree = free MiB.
Refusal checks: abortSetupBadCalProof (setup 1, setup-stopped), refuseMalformedProfile (startRun,
profile-refused), refuseUnfittableBudget (startRun at a 1 GB limit, no-shape-fits), refuseInvalidCheckFree on
shape 2 (prove, shape-no-key), refused early wrap (early-wrap-refused), refuseJoinCorruptedLeaf (join 1,
leaf-not-verified).
usage: project_scheduling.py TRACEDIR OUTDIR"""
import sys, glob, os, json
sys.path.insert(0, os.path.dirname(__file__))
from common import load, tag, Out

NEG = bool(os.environ.get('NEGATIVE_CONTROL'))
CAPS = [4, 8]
STEPS = {'PrepRunA_Short': 5, 'PrepRunB_Long': 12, 'PrepRunC_FreshSalt': 5}
BOXN = {'BoxMuninn': 0, 'BoxHuginn': 1, 'BoxOdin': 2}

def key(k):
    return json.loads(k)['tag'] if isinstance(k, str) and k.startswith('{') else tag(k)

def profs(sc):
    return {key(k): v for k, v in sc['profiles'].items()}

def gb(mb):
    return max(mb, 0) // 1024

def base():
    z4 = [0, 0, 0, 0]
    return dict(memAvail=0, cgroupHead=0, hasGpu=False, gpuReportsMem=False, hwGpuTotal=0, hwGpuFree=0, cores=0,
                proverKeyD=250, caps=list(CAPS), calT=list(z4), calHost=list(z4), calDev=list(z4), profWritten=False,
                profFormatOk=False, profFieldsOk=False, profKeyD=0, memBudget=0, unified=False, gpuCount=0, gpuTotal=0,
                gpuFree=0, profCores=0, calMask=0, setupStops=0, memUnit=1, limMemOn=False, limMem=0, gpusOff=False,
                limGpuMemOn=False, limGpuMem=0, n=0, prog=0, input=0, rP=0, salt=0, box=0, scheduled=False,
                calibrated=False, schShape=0, schDevice=0, schConc=0, schEst=0, cut=[], forced=False, policy=0,
                peakStark=0, peakWrap=0, peakCoset=0, wrapFree=0, starkExtra=0, margin=0, earlyWraps=0, runs=0,
                recBox=0, recShape=0, recDevice=0, recLeaves=0, recCalibrated=False, recKeys=0, recClaim=0, nodes=0)

def hw(p, b):
    g = p['gpuEntries'][0] if p['gpuEntries'] else None
    ram, bud = p['ramTotalGb'], p['memBudgetGb']
    return dict(memAvail=ram, cgroupHead=bud if bud < ram else 0, hasGpu=g is not None,
                gpuReportsMem=bool(g) and not g['unified'], hwGpuTotal=gb(g['vramTotalMb']) if g else 0,
                hwGpuFree=gb(g['vramFreeMb']) if g else 0, cores=p['cores'], box=BOXN[b])

def cal(p):
    t, h, d = [0] * 4, [0] * 4, [0] * 4
    for c in p['calEntries']:
        if c['shapeIdx'] > 1:
            continue
        e = 2 * c['shapeIdx'] + (1 if tag(c['devKind']) == 'DevCpu' else 0)
        t[e], h[e], d[e] = c['leafMs'], c['hostGb'], c['deviceGb']
    return dict(calT=t, calHost=h, calDev=d)

def profile_state(p, b):
    s = {**base(), **hw(p, b), **cal(p)}
    g = p['gpuEntries'][0] if p['gpuEntries'] else None
    uni = bool(g) and g['unified']
    mask = sum(1 << (2 * c['shapeIdx'] + (1 if tag(c['devKind']) == 'DevCpu' else 0)) for c in p['calEntries'] if c['shapeIdx'] <= 1)
    s.update(profWritten=p['written'], profFormatOk=tag(p['formatTag']) == 'FormatSherlockProfileV1',
             profFieldsOk=p['fieldCount'] == 11, profKeyD=p['keysD'], memBudget=p['memBudgetGb'], unified=uni,
             gpuCount=1 if g else 0, gpuTotal=0 if (uni or not g) else gb(g['vramTotalMb']),
             gpuFree=0 if (uni or not g) else gb(g['vramFreeMb']), profCores=p['cores'], calMask=mask)
    return s

def run_state(p, b, r):
    s = profile_state(p, b)
    s['n'] = r['runSteps']
    if r['hasMemLimitOverride']:
        s.update(limMemOn=True, limMem=r['memLimitOverrideGb'])
    if r['hasGpuCountOverride']:
        s.update(gpusOff=r['gpuCountOverride'] == 0)
    if r['hasGpuMemOverride']:
        s.update(limGpuMemOn=True, limGpuMem=r['gpuMemOverrideGb'])
    return s

def main(tdir, odir):
    out = Out(odir)
    for f in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(f)[:-9]; S = load(f)
        for i in range(1, len(S)):
            sc, sp = S[i]['sched'], S[i - 1]['sched']; op = S[i]['lastOp']; nm = f'{name}_s{i}'
            P, PP = profs(sc), profs(sp)
            changed = [b for b in P if P[b] != PP[b]]
            if op == 'runSetupOnBox':
                b = changed[0]; p = P[b]
                s0 = {**base(), **hw(p, b), **cal(p)}
                s1 = {**profile_state(p, b), 'profKeyD': 250 + (1 if NEG else 0)}
                out.chain(nm, [dict(s0, op=-1), dict(s1, op=0)])
            elif op == 'abortSetupBadCalProof':
                b = key(sc['lastRefusal']['value']) if isinstance(sc['lastRefusal']['value'], (str, dict)) else None
                out.refusal(nm, {**base()}, 1, ['setup-stopped'])
            elif op == 'refuseMalformedProfile':
                v = sc['lastRefusal']['value']; b = tag(v[0]); kind = tag(v[1]); p = P[b]
                s = {**profile_state(p, b), 'n': 5}
                if kind == 'BadFormatTag': s['profFormatOk'] = False
                elif kind == 'BadMissingField': s['profFieldsOk'] = False
                else: s['calT'] = [0 if (s['calMask'] >> e & 1) and e == 0 else x for e, x in enumerate(s['calT'])]
                out.refusal(nm, s, 16, ['profile-refused'])
            elif op == 'recordUncalibratedProfileKeyMismatch':
                new = [x for x in sc['uncalibratedRecords'] if x not in sp['uncalibratedRecords']]
                if not new:
                    out.skip('uncalibrated: no new record'); continue
                b = tag(new[0][0]); pr = tag(new[0][1]); p = P[b]
                s0 = {**profile_state(p, b), 'n': STEPS[pr]}
                if s0['profKeyD'] == s0['proverKeyD']:
                    out.skip('uncalibrated: profile keysD equals prover'); continue
                out.skip('uncalibrated: STM-008 record has no pick fields (calibrated covered by proved runs)')
            elif op in ('scheduleAndProveFromProfile', 'scheduleAndProveForcedCut'):
                new = [r for r in sc['provedRuns'] if r not in sp['provedRuns']]
                if not new:
                    out.skip(f'{op}: no new run'); continue
                r = new[0]; b = tag(r['boxId']); p = P[b]
                if r['shapeIdx'] > 1:
                    out.skip(f'{op}: shape outside the oracle caps'); continue
                s0 = run_state(p, b, r); dev = 1 if tag(r['devKind']) == 'DevCpu' else 0
                n, k, cap = r['runSteps'], r['cutLeafCount'], CAPS[r['shapeIdx']]
                sch = {'scheduled': True, 'calibrated': r['calibratedForKeySet'], 'schShape': r['shapeIdx'],
                       'schDevice': dev, 'schConc': min(r['concurrency'], -(-n // cap)), 'schEst': r['estWallMs']}
                # D-sched-1: the model's concurrency is the device capacity (not capped by the run's
                # leaf count); the run concurrency is min(capacity, ceil(n / S)) as in profile.rs.
                # RAW_CONC=1 projects the uncapped model value (392 startRun disagreements).
                if os.environ.get('RAW_CONC'): sch['schConc'] = r['concurrency']
                rec = {'runs': 1, 'recBox': BOXN[b], 'recShape': r['shapeIdx'], 'recDevice': dev,
                       'recLeaves': k + (1 if NEG else 0), 'recCalibrated': r['calibratedForKeySet']}
                if op == 'scheduleAndProveForcedCut':
                    # T16 forces shape, device and cut: the chain starts from the forced schedule
                    # (the oracle's startRun pick is not the forced one), then forceCut k and prove.
                    if not 0 < k < 16:
                        out.skip('forced cut k outside the op arg'); continue
                    g = -(-n // cap); st = {**s0, **sch, 'cut': [cap] * (g - 1) + [n - cap * (g - 1)]}
                    s1 = {**st, 'forced': True, 'cut': [n // k + (1 if i < n % k else 0) for i in range(k)]}
                    out.chain(nm, [dict(st, op=-1), dict(s1, op=80 + k), dict(s1, **rec, op=96)])
                else:
                    s1 = {**s0, **sch, 'cut': [cap] * (k - 1) + [n - cap * (k - 1)]}
                    out.chain(nm, [dict(s0, op=-1), dict(s1, op=16), dict(s1, **rec, op=96)])
            elif op == 'refuseUnfittableBudget':
                bs = [b for b in P if P[b]['written'] and tag(P[b]['formatTag']) == 'FormatSherlockProfileV1'
                      and P[b]['fieldCount'] == 11 and all(c['leafMs'] > 0 for c in P[b]['calEntries'])]
                if not bs:
                    out.skip('unfittable: no valid profile'); continue
                b = bs[0]; s = {**profile_state(P[b], b), 'n': 5, 'limMemOn': True, 'limMem': 1, 'gpusOff': True}
                out.refusal(nm, s, 16, ['no-shape-fits'])
            elif op == 'refuseInvalidCheckFree':
                sh = sc['lastRefusal']['value']
                if sh == 2:
                    bs = [b for b in P if P[b]['written']]
                    s = {**base(), 'profWritten': True, 'profFormatOk': True, 'profFieldsOk': True, 'n': 5,
                         'scheduled': True, 'schShape': 2, 'cut': [1, 1, 1, 1, 1]}
                    out.refusal(nm, s, 96, ['shape-no-key'])
                else:
                    out.skip('check_free refusal on shapes 0/1: the cut is not in the trace')
            elif op == 'evaluateEarlyWrapGuard':
                new = [c for c in sc['earlyWrapChecks'] if c not in sp['earlyWrapChecks']]
                if not new:
                    out.skip('early wrap: no new check'); continue
                c = new[0]
                s0 = {**base(), 'scheduled': True, 'policy': 2, 'peakWrap': c['requiredMb'], 'wrapFree': c['freeDeviceMb'],
                      'profWritten': True}
                if c['earlyWrapAdmitted']:
                    out.chain(nm, [dict(s0, op=-1), dict(s0, earlyWraps=1, op=160)])
                else:
                    out.refusal(nm, s0, 160, ['early-wrap-refused'])
            elif op == 'joinLeavesTwoBoxes':
                out.chain(nm, [dict(base(), op=-1), dict(base(), nodes=1, op=144)])
            elif op == 'refuseJoinCorruptedLeaf':
                out.refusal(nm, base(), 145, ['leaf-not-verified'])
            elif op in ('injectMalformedProfile', 'installStaleT10mProfile'):
                pass
            else:
                out.skip(f'{op}: unmapped')
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
