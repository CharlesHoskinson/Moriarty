#!/usr/bin/env python3
"""Assurance: project Quint traces (per-box deploy secrets, prepared runs, wrap proofs, settled calls, gate
state, circuit changes) onto the oracle (one deployment). Every model transition becomes one short chain or
one refusal check whose initial state carries the model pre-state; the expected post-state comes from the
model post-state. Hosts: muninn 0, huginn 1, odin 2, external helper 3 (roster bits of ownedRoster). Devices:
WrapCpu 0, WrapGpu 1. Blinders: the model's proofSeq of every OsRng proof (0 for a seeded proof).
Mapped actions and oracle ops:
  prepareRunAndLog prepareRun (16) then writeRunRecord (160) [SSG-007 claim];
  refuseConstantRpExit4 prepareDeployment (0) with constRpPath, release build: seeded-without-feature;
  refuseEnvSeedInReleaseWrapExit4 wrapProve 0 (48) with seedEnv, release build: seeded-without-feature;
  refuseMissingDeploySecret prepareRun without deploy.secret: secret-missing;
  transferSecretFileToBox transfer dst (80+h); refuseSecretTransferOutsideRoster, refuseCaseBUnownedHost
  transfer h refused host-not-in-roster (host-outside-roster);
  makeWrapProof useSlot slot (96+b) then wrapProve d (48+d); refuseReplayedBlindingSlot useSlot refused
  slot-reused; settleWrapProofAndScan submit (64); refuseSettleTestSeedProof submit refused test-seed-binary;
  completeT12ChecksAndFixes fixDefect (224) x n, verifyStep (240) x m, gateCheck c (112+c) for the checks
  that pass; refuseSecurityGateEvaluation writeRecord (128) refused confirmed-defect / g5-old-params /
  step-unverified; writePassingGateRecordAndQualify writeRecord (128); landAndAuditCircuitChange landChange
  (176) then fileVerdict (192).
Skipped (no oracle operation): stage/refuse deploy-secret mode, app key D3 checks, crate and wrap-K audit,
wrap key fixing the L transcript, G3 leg check, gate failure streak, loop-row T15 evaluation.
usage: project_assurance.py TRACEDIR OUTDIR"""
import sys, glob, os, json
sys.path.insert(0, os.path.dirname(__file__))
from common import load, tag, Out

NEG = bool(os.environ.get('NEGATIVE_CONTROL'))
HOST = {'HostMuninn': 0, 'HostHuginn': 1, 'HostOdin': 2, 'HostExternalHelper': 3}

def k(x): return json.loads(x)['tag'] if isinstance(x, str) and x.startswith('{') else tag(x)

def new(c, p, f): return [x for x in c[f] if x not in p[f]]

def proofs(a):
    # OsRng proofs only (the k-th draw is blinder k), newest first; a seeded proof belongs to a test-seed
    # binary, so it enters the lists only as the proof the current chain makes.
    w = sorted([r for r in a['wrapProofs'] if not r['envSeedPassed']], key=lambda r: r['proofSeq'])
    return [i + 1 for i in range(len(w))][::-1], [1 if tag(r['device']) == 'WrapGpu' else 0 for r in w][::-1]

def gatebits(a):
    b = 0
    if a['g1g2g4Fp8LedgerPassed']: b |= 1 | 2 | 8 | 32
    if a['g3Passed']: b |= 4
    if a['g5MappedManifestSha'] == a['activeManifestSha']: b |= 16
    return 63 if a['gateRecordWritten'] else b

def base(a):
    bl, dv = proofs(a); n = len(a['preparedRuns'])
    return dict(testSeedBuild=False, seedEnv=False, constRpPath=False, rpDraws=1, deploySecret=True, saltDraws=n, salt=n,
                blinderDraws=len(bl), blinders=bl, devices=dv, submitted=len(a['settledCalls']),
                roster=sum(1 << HOST[tag(h)] for h in a['ownedRoster']), transfers=len(a['admittedSecretTransfers']),
                usedSlots=sum(1 << s for s in a['usedBlindingSlots'] if s < 16), gateBits=gatebits(a),
                unverified=a['unverifiedRunbookSteps'], confirmedNoFix=a['unfixedConfirmedCxCount'],
                notAttempted=len(a['notAttemptedCxIds']), g5Current=a['g5MappedManifestSha'] == a['activeManifestSha'],
                gateRecord=a['gateRecordWritten'], recordManifest=a['activeManifestSha'] if a['gateRecordWritten'] else 0,
                recordOpen=len(a['gateRecordOpenCxIds']), manifestD=a['activeManifestSha'],
                qualified=tag(a['manifestStatus']) == 'KeyQualified', runRecords=n,
                lastRunQualified=False, changes=len(a['circuitChanges']), changeRows=len(a['circuitChanges']),
                verdicts=sum(1 for c in a['circuitChanges'] if c['leadVerdictFiled']), doneRows=0)

def main(tdir, odir):
    out = Out(odir)
    for fn in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(fn)[:-9]; S = load(fn)
        for i in range(1, len(S)):
            c, p = S[i]['assure'], S[i - 1]['assure']; op = S[i]['lastOp']; nm = f'{name}_s{i}'
            s0, s1 = base(p), base(c); ch = None
            if op == 'prepareRunAndLog':
                r = new(c, p, 'preparedRuns')
                if not r: out.skip('prepare: no new run'); continue
                r = r[0]; st = dict(s0, testSeedBuild=r['builtWithTestSeed'], constRpPath=tag(r['rPSource']) == 'RandConstant',
                                    deploySecret=r['deploySecretWasPresent'], lastRunQualified=False)
                m1 = dict(st, saltDraws=s1['saltDraws'], salt=s1['saltDraws'])
                q = tag(r['securityClaimInRecord']) == 'SecQualified'
                ch = [dict(st, op=-1), dict(m1, op=16), dict(m1, runRecords=st['runRecords'] + 1 + (1 if NEG else 0), lastRunQualified=q, op=160)]
            elif op == 'refuseConstantRpExit4':
                out.refusal(nm, dict(s0, constRpPath=True, rpDraws=0, deploySecret=False), 0, ['seeded-without-feature'])
            elif op == 'refuseEnvSeedInReleaseWrapExit4':
                out.refusal(nm, dict(s0, seedEnv=True, salt=max(1, s0['salt']), saltDraws=max(1, s0['saltDraws'])), 48, ['seeded-without-feature'])
            elif op == 'refuseMissingDeploySecret':
                out.refusal(nm, dict(s0, deploySecret=False), 16, ['secret-missing'])
            elif op == 'transferSecretFileToBox':
                t = new(c, p, 'admittedSecretTransfers')
                if not t: out.skip('transfer: duplicate record'); continue
                ch = [dict(s0, op=-1), dict(s0, transfers=s1['transfers'], op=80 + HOST[tag(t[0]['dstHost'])])]
            elif op in ('refuseSecretTransferOutsideRoster', 'refuseCaseBUnownedHost'):
                v = c['lastRefusal']['value']; h = tag(v[0]) if isinstance(v, list) else tag(v)
                out.refusal(nm, s0, 80 + HOST[h], ['host-outside-roster'])
            elif op == 'makeWrapProof':
                w = new(c, p, 'wrapProofs')
                if not w: out.skip('wrap: no new proof'); continue
                w = w[0]; sl = w['blindingSlotId']
                st = dict(s0, testSeedBuild=w['builtWithTestSeed'], seedEnv=w['envSeedPassed'])
                if st['salt'] == 0:
                    out.skip('wrap: no prepared run yet (D-assure-1: the model proves without a run)'); continue
                ch = [dict(st, op=-1)]
                if sl < 16:
                    st = dict(st, usedSlots=st['usedSlots'] | (1 << sl)); ch.append(dict(st, op=96 + sl))
                d = 1 if tag(w['device']) == 'WrapGpu' else 0
                if w['envSeedPassed']: bl, dv, nd = [0] + st['blinders'], [d] + st['devices'], st['blinderDraws']
                else: bl, dv, nd = s1['blinders'], s1['devices'], s1['blinderDraws']
                ch.append(dict(st, blinders=bl, devices=dv, blinderDraws=nd,
                               op=48 + (1 if tag(w['device']) == 'WrapGpu' else 0)))
            elif op == 'refuseReplayedBlindingSlot':
                sl = c['lastRefusal']['value']
                if sl >= 16: out.skip('slot id outside the op arg'); continue
                out.refusal(nm, s0, 96 + sl, ['slot-reused'])
            elif op == 'settleWrapProofAndScan':
                sc = new(c, p, 'settledCalls')
                if not sc or not s0['blinders']: out.skip('settle: duplicate record'); continue
                ch = [dict(s0, op=-1), dict(s0, submitted=s1['submitted'], op=64)]
            elif op == 'refuseSettleTestSeedProof':
                if not s0['blinders']: out.skip('settle refusal: no proof'); continue
                out.refusal(nm, dict(s0, testSeedBuild=True), 64, ['test-seed-binary'])
            elif op == 'completeT12ChecksAndFixes':
                st = dict(s0); ch = [dict(st, op=-1)]
                for _ in range(s0['confirmedNoFix'] - s1['confirmedNoFix']):
                    st = dict(st, confirmedNoFix=st['confirmedNoFix'] - 1); ch.append(dict(st, op=224))
                for _ in range(s0['unverified'] - s1['unverified']):
                    st = dict(st, unverified=st['unverified'] - 1); ch.append(dict(st, op=240))
                for b in range(6):
                    if s1['gateBits'] >> b & 1 and not st['gateBits'] >> b & 1 and not st['gateRecord']:
                        st = dict(st, gateBits=st['gateBits'] | (1 << b)); ch.append(dict(st, op=112 + b))
                ch[-1].update(g5Current=s1['g5Current'])
                if len(ch) < 2: out.skip('T12: nothing changes'); continue
            elif op == 'refuseSecurityGateEvaluation':
                v = tag(c['lastRefusal'])
                code = {'AssureRefusalGateBlockedConfirmedCx': 'confirmed-defect', 'AssureRefusalGateBlockedStaleG5': 'g5-old-params',
                        'AssureRefusalGateBlockedUnverifiedStep': 'step-unverified'}[v]
                out.refusal(nm, dict(s0, gateBits=63, gateRecord=False, qualified=False, recordManifest=0), 128, [code])
            elif op == 'writePassingGateRecordAndQualify':
                st = dict(s0, gateBits=63)
                ch = [dict(st, op=-1), dict(st, gateRecord=True, recordManifest=s1['manifestD'], recordOpen=s1['recordOpen'],
                                            qualified=s1['qualified'], op=128)]
            elif op == 'landAndAuditCircuitChange':
                cc = new(c, p, 'circuitChanges')
                if not cc: out.skip('change: duplicate record'); continue
                m = dict(s0, changes=s0['changes'] + 1, changeRows=s0['changeRows'] + 1)
                ch = [dict(s0, op=-1), dict(m, op=176)]
                if cc[0]['leadVerdictFiled']: ch.append(dict(m, verdicts=s1['verdicts'], op=192))
            else:
                out.skip(f'{op}: no oracle operation')
            if ch: out.chain(nm, ch)
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
