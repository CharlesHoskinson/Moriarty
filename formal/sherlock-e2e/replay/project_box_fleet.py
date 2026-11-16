#!/usr/bin/env python3
"""Box fleet: project Quint traces (5 hosts, per-box gpu.lock and link state, wrapd and STARK worker, record
sets) onto the oracle (one fleet, one gpu.lock, one daemon). Every model transition becomes one short chain (or
one refusal check) whose initial state carries the model pre-state of the box the action touches; the expected
post-state comes from the model post-state. Hosts: muninn 0, huginn 1, odin 2, fourth owned 3; the rental host
is "a host not in the roster" (host 3 with roster bit 3 clear). Lock holders: none 0, prover 1, wrapd 2,
orphan with a dead parent 3, any other holder 4. Manifest digests: 250 (T25) -> 1, 100 (T10m) -> 2, other -> 3.
Mapped actions and oracle ops:
  addFourthOwnedBoxToRoster addBox 3 (83); refuseUnownedHostJob submitJob 3 refused host-not-in-roster;
  admitAndStartBoxJob submitJob h (h) | sharedJob ceil(mem/8) (16+a) | exclusiveJob (32), then startRun (336+a);
  enqueueMuninnSlotWaiter sharedJob 1 refused no-slot; releaseMuninnSlot endShared (48) | endExclusive (64)
  [+ sharedJob 1 for the promoted waiter]; refuseSharedOverBudgetOnMuninn sharedJob ceil(mem/8) refused
  over-budget; startOrQueueGpuJobOnBox gpuAcquire 1 (97) or refused gpu-locked; finishGpuJobReleaseLock
  gpuRelease (112) [+ gpuAcquire 1 for the waiter]; stageLockHolderScenario gpuAcquire k + idle (128+m);
  observeSlowProverLeaveRunning sigterm refused holder-protected; reapOrphanLockSigterm sigterm (144) [+
  gpuRelease when the holder exits]; escalateOrphanLockSigkill sigkill (176); transferFileVerified transfer
  (192+4src+dst), two hops through muninn for a spoke pair; refuseDigestMismatchTransfer transfer with shaOk
  false refused sha256-mismatch; refuseDirectSpokeTransfer transfer 1<->2 refused not-via-muninn;
  postAndRelayBusMessage outbox (208+b); dropBoxLinkAndHandoverQueue linkDrop (224+b) with ownerAway;
  probeGpuWithoutConfCompute gpuQuery 0 (368); proveStagePipeline proveShard 0 (320); faultGpuShardOutOfMemory
  proveShard 1 (321); startDaemonsOnBox daemonStart m (256+m); refuseUnauthorizedSocketPeer request 0 refused
  peer-refused; daemonHandleUnknownOpDump request 4 (276); daemonRefuseManifestMismatch request 0 refused
  manifest-mismatch; daemonRefuseBadBlinder makeSlot 1 refused helper-slot; daemonServeCall makeSlot 0 (288)
  then request 1 (273); daemonServeStats request 2 (274); recordBoxTimingClaim speedClaim (352+4a+b).
Skipped (no oracle operation): rebootAndResumeDurableUnits, refuseStarkExportOutsideRoster, runElasticScaleDemo.
usage: project_box_fleet.py TRACEDIR OUTDIR"""
import sys, glob, os
sys.path.insert(0, os.path.dirname(__file__))
from common import load, tag, Out

NEG = bool(os.environ.get('NEGATIVE_CONTROL'))
HOST = {'HostMuninn': 0, 'HostHuginn': 1, 'HostOdin': 2, 'HostFourthOwned': 3}
MAN = {250: 1, 100: 2}

def man(x): return MAN.get(x, 3)

def roster(f):
    return sum(1 << HOST[tag(h)] for h in f['ownedRoster'] if tag(h) in HOST)

def boxes(f):
    return {tag(k) if not str(k).startswith('{') else __import__('json').loads(k)['tag']: v for k, v in f['boxes'].items()}

def holder(b):
    if not b['gpuLockHeld']: return 0
    r = tag(b['lockHolderRole'])
    return {'HolderProver': 1, 'HolderWrapd': 2}.get(r, 3 if r == 'HolderOrphanSampler' and not b['holderParentAlive'] else 4)

def base(f):
    d = f['daemon']
    return dict(roster=roster(f), slotsUsed=f['muninnSharedRunning'], exclRunning=f['muninnExclusiveRunning'],
                remoteJobs=0, gpuHolder=0, idleMin=0, termSent=False, termAgeS=0, busLog=len(f['allLogMsgIds']),
                shaOk=True, transfers=len(f['acceptedTransfers']), ownerAway=False, q1=0, q2=0,
                daemonUp=d['running'], socketMode=384 if d['running'] else 0, keysLoaded=d['running'],
                daemonManifest=man(d['loadedManifestSha']) if d['running'] else 0, reqManifest=0, reqPeerOk=True,
                reqLeaves=1, served=d['servedSeqCounter'], errors=0, statsReplies=len(f['statsReplies']),
                statsWitness=0, poolFree=0, slotsMade=d['servedSeqCounter'], slotsSpent=d['servedSeqCounter'],
                lastPath=0, cuda=True, nodeEnv=0, lEnv=0, shardsProved=0, runStopped=False, stopShard=0,
                stopDevice=0, lastAppDev=0, lastNodeDev=0, lastLDev=0, runsStarted=0, unitRuns=0, claims=0,
                gpuQueries=len(f['gpuProbeCalls']))

def lock(b):
    return dict(gpuHolder=holder(b), idleMin=b['lockStallMin'] if b['gpuLockHeld'] else 0, termSent=b['sigtermSent'],
                termAgeS=b['elapsedSinceSigtermSec'] if b['sigtermSent'] else 0)

def new(fc, fp, k):
    return [x for x in fc[k] if x not in fp[k]]

def main(tdir, odir):
    out = Out(odir)
    for fn in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(fn)[:-9]; S = load(fn)
        for i in range(1, len(S)):
            fc, fp = S[i]['fleet'], S[i - 1]['fleet']; op = S[i]['lastOp']; nm = f'{name}_s{i}'
            s0 = base(fp); s1 = base(fc); Bc, Bp = boxes(fc), boxes(fp)
            ch = None
            if op == 'addFourthOwnedBoxToRoster':
                ch = [dict(s0, op=-1), dict(s0, roster=s1['roster'], op=83)]
            elif op == 'refuseUnownedHostJob':
                out.refusal(nm, dict(s0, roster=s0['roster'] & 7), 3, ['host-not-in-roster'])
            elif op == 'admitAndStartBoxJob':
                j = new(fc, fp, 'admittedJobs')
                if not j: out.skip('admit: no new job'); continue
                j = j[0]; h = tag(j['targetHost'])
                if h not in HOST: out.skip('admit: host outside the oracle hosts'); continue
                h = HOST[h]
                if h == 0 and j['isExclusiveSlot']:
                    o1, st1 = 32, dict(s0, exclRunning=True)
                elif h == 0:
                    o1, st1 = 16 + min(15, -(-j['reqMemGb'] // 8)), dict(s0, slotsUsed=s0['slotsUsed'] + 1)
                else:
                    o1, st1 = h, dict(s0, remoteJobs=1)
                pm = j['plannedMin']; a = 4 if pm > 30 else min(3, pm // 10); u = j['startedAsSystemdUserUnit']
                st2 = dict(st1, runsStarted=1, unitRuns=1 if u else 0)
                ch = [dict(s0, op=-1), dict(st1, slotsUsed=fc['muninnSharedRunning'] if h == 0 else st1['slotsUsed'],
                                            exclRunning=fc['muninnExclusiveRunning'], op=o1), dict(st2,
                       slotsUsed=fc['muninnSharedRunning'] if h == 0 else st2['slotsUsed'], exclRunning=fc['muninnExclusiveRunning'],
                       op=336 + 2 * a + (1 if u else 0))]
            elif op == 'enqueueMuninnSlotWaiter':
                out.refusal(nm, s0, 17, ['no-slot'])
            elif op == 'releaseMuninnSlot':
                promote = fp['muninnSlotWaiters'] > 0
                if fp['muninnExclusiveRunning']:
                    mid = dict(s0, exclRunning=False); ch = [dict(s0, op=-1), dict(mid, op=64)]
                else:
                    mid = dict(s0, slotsUsed=s0['slotsUsed'] - 1); ch = [dict(s0, op=-1), dict(mid, op=48)]
                if promote:
                    ch.append(dict(mid, slotsUsed=s1['slotsUsed'], exclRunning=s1['exclRunning'], op=17))
                else:
                    ch[-1].update(slotsUsed=s1['slotsUsed'], exclRunning=s1['exclRunning'])
            elif op == 'refuseSharedOverBudgetOnMuninn':
                v = fc['lastRefusal']['value']
                out.refusal(nm, s0, 16 + min(15, -(-v // 8)), ['over-budget'])
            elif op in ('startOrQueueGpuJobOnBox', 'finishGpuJobReleaseLock', 'stageLockHolderScenario',
                        'reapOrphanLockSigterm', 'escalateOrphanLockSigkill', 'observeSlowProverLeaveRunning'):
                ch_h = [h for h in Bc if Bc[h] != Bp[h]] or [None]
                h = ch_h[0]
                if op == 'observeSlowProverLeaveRunning':
                    m = new(fc, fp, 'relayedMessages')
                    h = tag(m[0]['senderHost']) if m else None
                if h is None or h not in Bp: out.skip(f'{op}: no box changed'); continue
                bp, bc = Bp[h], Bc[h]; p = dict(s0, **lock(bp))
                if op == 'startOrQueueGpuJobOnBox':
                    if bp['gpuLockHeld']: out.refusal(nm, p, 97, ['gpu-locked']); continue
                    ch = [dict(p, op=-1), dict(p, gpuHolder=holder(bc), idleMin=0, op=97)]
                elif op == 'finishGpuJobReleaseLock':
                    if not bp['gpuLockHeld']: out.skip('finish: lock not held before (job ended by SIGTERM)'); continue
                    rel = dict(p, gpuHolder=0, idleMin=0, termSent=False, termAgeS=0)
                    ch = [dict(p, op=-1), dict(rel, op=112)]
                    if bc['gpuLockHeld']: ch.append(dict(rel, gpuHolder=holder(bc), op=97))
                    else: ch[-1]['gpuHolder'] = holder(bc)
                elif op == 'stageLockHolderScenario':
                    k = holder(bc); st = dict(p, gpuHolder=k, idleMin=0, termSent=False, termAgeS=0)
                    ch = [dict(p, op=-1), dict(st, op=96 + k)]; left = bc['lockStallMin']; acc = 0
                    while left > 0:
                        m = min(15, left); left -= m; acc += m; ch.append(dict(st, idleMin=acc, op=128 + m))
                elif op == 'observeSlowProverLeaveRunning':
                    out.refusal(nm, p, 144, ['holder-protected']); continue
                elif op == 'reapOrphanLockSigterm':
                    ev = [e for e in new(fc, fp, 'killEvents') if tag(e['sigKind']) == 'SigTerm']
                    logged = 1 if ev and ev[0]['loggedOnBusBeforeSignal'] else 0
                    st = dict(p, termSent=True, termAgeS=0, busLog=s0['busLog'] + logged)
                    ch = [dict(p, op=-1), dict(st, op=144)]
                    if not bc['gpuLockHeld']:   # the holder exits on SIGTERM: the lock is free
                        ch.append(dict(st, gpuHolder=0, idleMin=0, termSent=False, termAgeS=0, op=112))
                    else:
                        ch[-1]['termSent'] = bc['sigtermSent']
                elif op == 'escalateOrphanLockSigkill':
                    ev = [e for e in new(fc, fp, 'killEvents') if tag(e['sigKind']) == 'SigKill']
                    logged = 1 if ev and ev[0]['loggedOnBusBeforeSignal'] else 0
                    ch = [dict(p, op=-1), dict(p, gpuHolder=holder(bc), idleMin=0, termSent=bc['sigtermSent'], termAgeS=0,
                                               busLog=s0['busLog'] + logged, op=176)]
            elif op == 'transferFileVerified':
                t = new(fc, fp, 'acceptedTransfers')
                if not t: out.skip('transfer: no new record'); continue
                t = t[0]; a, b = tag(t['srcHost']), tag(t['dstHost'])
                if a not in HOST or b not in HOST: out.skip('transfer: host outside the oracle hosts'); continue
                a, b = HOST[a], HOST[b]
                hops = [(a, b)] if 0 in (a, b) else [(a, 0), (0, b)]
                ch = [dict(s0, op=-1)]; n0 = s0['transfers']
                for k, (x, y) in enumerate(hops):
                    ch.append(dict(s0, transfers=n0 + k + 1, op=192 + 4 * x + y))
            elif op == 'refuseDigestMismatchTransfer':
                a, b = [tag(x) for x in fc['lastRefusal']['value']]
                if a not in HOST or b not in HOST or a == b: out.skip('digest refusal: hosts outside the op'); continue
                out.refusal(nm, dict(s0, shaOk=False), 192 + 4 * HOST[a] + HOST[b], ['sha256-mismatch'])
            elif op == 'refuseDirectSpokeTransfer':
                a, b = [tag(x) for x in fc['lastRefusal']['value']]
                out.refusal(nm, s0, 192 + 4 * HOST[a] + HOST[b], ['not-via-muninn'])
            elif op == 'postAndRelayBusMessage':
                m = new(fc, fp, 'relayedMessages')[0]
                ch = [dict(s0, op=-1), dict(s0, busLog=s1['busLog'], op=208 + HOST[tag(m['senderHost'])])]
            elif op == 'dropBoxLinkAndHandoverQueue':
                d = [h for h in Bc if Bc[h]['linkUp'] != Bp[h]['linkUp']]
                bk = [h for h in Bc if h not in d and Bc[h]['queuedSuiteJobs'] != Bp[h]['queuedSuiteJobs']]
                if not d or HOST.get(d[0]) not in (1, 2) or (bk and HOST.get(bk[0]) not in (1, 2)):
                    out.skip('link drop: muninn dropped or muninn backup (oracle moves to the other spoke)'); continue
                st = dict(s0, ownerAway=True, q1=Bp['HostHuginn']['queuedSuiteJobs'], q2=Bp['HostOdin']['queuedSuiteJobs'])
                # D-box-2: BXF-010 moves the queue (linkDrop); the model's handover line on the bus is a
                # BXF-008 relay from muninn (outbox 0), so the chain is linkDrop then outbox 0.
                mv = dict(st, q1=Bc['HostHuginn']['queuedSuiteJobs'], q2=Bc['HostOdin']['queuedSuiteJobs'])
                ch = [dict(st, op=-1), dict(mv, op=224 + HOST[d[0]]), dict(mv, busLog=s1['busLog'], op=208)]
            elif op == 'probeGpuWithoutConfCompute':
                r = new(fc, fp, 'gpuProbeCalls')
                if not r: out.skip('probe: duplicate record'); continue
                q = 2 if r[0]['usedConfCompute'] else 0
                ch = [dict(s0, op=-1), dict(s0, gpuQueries=s1['gpuQueries'], op=368 + q)]
            elif op == 'proveStagePipeline':
                r = new(fc, fp, 'stageProofs')
                if not r: out.skip('stage: duplicate record'); continue
                r = r[0]; dv = lambda x: 1 if tag(x) == 'BackendCuda' else 0
                if r['cudaBuild'] and not r['gpuScheduled'] and not os.environ.get('RAW_GPV'):
                    # D-box-1 / F-box-fleet-Q6: the schedule picked the CPU in a cuda build; the model (and t16.rs:512)
                    # prove the app shards on the CPU, the oracle follows GPV-001 text (GPU). RAW_GPV=1 projects them.
                    out.skip('stage: cuda build, CPU schedule (D-box-1, GPV-001 text)'); continue
                st = dict(s0, cuda=r['cudaBuild'], nodeEnv=2 if r['nodeGpuSwitchOff'] else 0, lEnv=1 if r['lGpuSwitchOff'] else 0,
                          shardsProved=r['shardIdx'])
                ch = [dict(st, op=-1), dict(st, shardsProved=r['shardIdx'] + 1 + (1 if NEG else 0), lastAppDev=dv(r['appBackend']),
                                            lastNodeDev=dv(r['nodeBackend']), lastLDev=dv(r['lBackend']), op=320)]
            elif op == 'faultGpuShardOutOfMemory':
                r = new(fc, fp, 'gpuFaultRecords')
                if not r: out.skip('fault: duplicate record'); continue
                r = r[0]; st = dict(s0, shardsProved=r['shardIdx'])
                ch = [dict(st, op=-1), dict(st, runStopped=r['runStopped'], stopShard=r['shardIdx'],
                                            stopDevice=1 if r['namedShardAndDevice'] else 0, op=321)]
            elif op == 'startDaemonsOnBox':
                m = man(fc['daemon']['loadedManifestSha'])
                ch = [dict(s0, op=-1), dict(s0, daemonUp=True, socketMode=384 if fc['daemon']['socketMode'] == 600 else fc['daemon']['socketMode'],
                                            keysLoaded=fc['starkWorker']['starkKeysResident'], daemonManifest=m, op=256 + m)]
            elif op == 'refuseUnauthorizedSocketPeer':
                out.refusal(nm, dict(s0, reqPeerOk=False, reqManifest=s0['daemonManifest'], poolFree=1, slotsMade=s0['slotsMade'] + 1), 272, ['peer-refused'])
            elif op == 'daemonHandleUnknownOpDump':
                ch = [dict(s0, op=-1), dict(s0, errors=1, daemonUp=fc['daemon']['running'], op=276)]
            elif op == 'daemonRefuseManifestMismatch':
                d, r = fc['lastRefusal']['value']
                rm = man(r) if man(r) != man(d) else (man(d) % 3) + 1
                out.refusal(nm, dict(s0, reqManifest=rm, poolFree=1, slotsMade=s0['slotsMade'] + 1), 272, ['manifest-mismatch'])
            elif op == 'daemonRefuseBadBlinder':
                out.refusal(nm, s0, 289, ['helper-slot'])
            elif op == 'daemonServeCall':
                w = new(fc, fp, 'servedWrapProofs')
                if not w: out.skip('serve: no new proof'); continue
                w = w[0]
                st = dict(s0, reqManifest=man(w['reqManifestSha']), reqLeaves=w['leafCount'])
                mk = dict(st, poolFree=1, slotsMade=st['slotsMade'] + 1)
                ch = [dict(st, op=-1), dict(mk, op=288),
                      dict(mk, poolFree=0, served=fc['daemon']['servedSeqCounter'], slotsSpent=st['slotsSpent'] + 1,
                           lastPath=3 if w['usedPathC'] else 1, op=273)]
            elif op == 'daemonServeStats':
                r = new(fc, fp, 'statsReplies')
                if not r: out.skip('stats: duplicate record'); continue
                ch = [dict(s0, op=-1), dict(s0, statsReplies=s1['statsReplies'], statsWitness=r[0]['leakedSecretWords'], op=274)]
            elif op == 'recordBoxTimingClaim':
                r = new(fc, fp, 'timingRecords')
                if not r: out.skip('timing: duplicate record'); continue
                a, b = HOST[tag(r[0]['measuredHost'])], HOST[tag(r[0]['baselineHost'])]
                ch = [dict(s0, op=-1), dict(s0, claims=1, op=352 + 4 * a + b)]
            elif op in ('rebootAndResumeDurableUnits', 'refuseStarkExportOutsideRoster', 'runElasticScaleDemo'):
                out.skip(f'{op}: no oracle operation')
            else:
                out.skip(f'{op}: unmapped')
            if ch: out.chain(nm, ch)
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
