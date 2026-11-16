#!/usr/bin/env python3
"""Bridge: project Quint traces (one rekey, 2 contracts, 5 calls, 6 proof slots) onto the oracle (one call or chunk).
Entities: proof slot p (1..6), one oracle run per slot. The witness fields come from the model catalogs:
the admitted L key's columns and interactions, slots 15, the call's mode, stated k and slice count.
Stage map (model -> oracle op = code*16+arg): startRekeyAndAdmitKeys -> keygenL (0); planCallChunks -> plan (16),
sent once the key exists (the model plans before the rekey; the oracle plan needs a key, projection convention);
proveLCallOrChunk -> proveL f (32+f, f=0 Path C, 1 full tree); proveAndVerifyLfr -> proveLfr (48) + stockVerify (64);
publishLfrReady -> publish (80); verifyTransferAndWrap -> transfer c (96+c, c=0 when the muninn digest matched) +
wrapStart (112); bridgeCallEndToEnd -> every missing stage. buildLfrMachine has no oracle op (its checks are the
witnesses of proveLfr). Model refusals become refusal checks on a state built from the spec scenario.
usage: project_bridge.py TRACEDIR OUTDIR"""
import sys, glob, os
sys.path.insert(0, os.path.dirname(__file__))
from common import load, Out

CALL = {1: (0, 0, 1, 1), 2: (0, 1, 1, 1), 3: (1, 1, 2, 2), 4: (1, 1, 2, 2), 5: (1, 1, 3, 3)}  # callId -> (mode, full, k, slices)
SLOT = {1: 1, 2: 2, 3: 3, 4: 3, 5: 4, 6: 4}                                                  # pSpecId -> callId
KEY = {1: (2071, 99), 2: (2200, 100), 3: (2201, 99), 4: (1995, 101)}                         # kId -> (cols, interactions)
NEG = bool(os.environ.get('NEGATIVE_CONTROL'))
NO_ORACLE_OP = {'compileAndDeployContract', 'buildLfrMachine', 'refuseInvalidWhirShape', 'refuseCollidingSlot15Contract',
                'refuseWrapWrongSlotPin', 'refuseMode0AnchorTemplateInWrap'}

def base(mode=1, k=2, slices=2, cols=2071, inter=99):
    return dict(lCols=cols, lInter=inter, slotIface=15, slotWrap=15, lKeyId=0, keygens=0, mode=mode, k=k, slices=slices,
                laneOk=True, tplWrite=False, lockOk=True, tapeExposed=True, tapeTyped=True, transcriptOk=True,
                stockAccepts=True, planned=False, phase=0, keyPathC=0, keyFull=0, verifierRefused=False,
                proofSha=0, recordSha=0, movedSha=0, records=0, wraps=0)

def keyed(**kw):
    return {**base(**kw), 'lKeyId': 1, 'keygens': 1}

def at_phase(ph, **kw):
    s = {**keyed(**kw), 'planned': True, 'phase': ph}
    if ph >= 1: s['keyFull' if s['mode'] == 1 else 'keyPathC'] = 1
    if ph >= 2: s['proofSha'] = 1
    if ph >= 4: s.update(records=1, recordSha=1)
    return s

def refusal(out, name, op, args):
    if op == 'refuseRekeySlotMismatch':
        out.refusal(name, {**base(), 'slotWrap': 0}, 0, ['slot-mismatch'])
    elif op == 'refuseLKeyOverBounds':
        c, n = KEY[args['primaryId']]; out.refusal(name, base(cols=c, inter=n), 0, ['l-key-bounds'])
    elif op == 'refuseCallOverKMax':
        m, _, k, sl = CALL[args['primaryId']]; out.refusal(name, keyed(mode=m, k=k, slices=sl), 16, ['k-max'])
    elif op == 'refuseShiftedEffectLaneWrite':
        out.refusal(name, {**at_phase(0), 'laneOk': False}, 33, ['lane-unbound'])
    elif op == 'refuseWrongChunkCountInL':
        out.refusal(name, at_phase(0, k=1, slices=2), 33, ['k-mismatch'])
    elif op == 'refuseMode0AnchorTemplateOnIface':
        out.refusal(name, {**at_phase(0, mode=0, k=1, slices=1), 'tplWrite': True}, 32, ['ordinary-anchor-write'])
    elif op == 'refuseTapeWithoutExpose':
        out.refusal(name, {**at_phase(1), 'tapeExposed': False}, 48, ['tape-not-exposed'])
    elif op in ('refuseIllTypedTapeInMachineNew', 'refuseMemBusTypeMismatchInCircuit'):
        out.refusal(name, {**at_phase(1), 'tapeTyped': False}, 48, ['ill-typed-tape'])
    elif op == 'refuseTamperedTranscriptChallenge':
        out.refusal(name, {**at_phase(1), 'transcriptOk': False}, 48, ['transcript-mismatch'])
    elif op == 'refuseSplicedOrAlteredChunkLock':
        out.refusal(name, {**at_phase(1), 'lockOk': False}, 48, ['lock-mismatch'])
    elif op == 'refuseForgedAnchorLfrNoRecord':
        s = {**at_phase(2), 'stockAccepts': False}; t = {**s, 'verifierRefused': not NEG}
        out.chain(name.replace('/', '_') + '_verify', [dict(s, op=-1), dict(t, op=64)])
        out.refusal(name, t, 80, ['not-verified', 'wrong-phase'])
    elif op == 'refuseCorruptedProofTransfer':
        s = at_phase(4); t = {**s, 'movedSha': 2}
        out.chain(name.replace('/', '_') + '_transfer', [dict(s, op=-1), dict(t, op=97)])
        out.refusal(name, t, 112, ['digest-mismatch'])
    else:
        return False
    return True

def main(tdir, odir):
    out = Out(odir)
    for f in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(f)[:-9]; S = load(f)
        last = S[-1]['rekeyState']
        cols, inter = KEY[last['admittedLKeyId']] if last['started'] else KEY[1]
        for i in range(1, len(S)):
            op = S[i]['lastOp']
            if op.startswith('refuse'):
                if not refusal(out, f'{name}/s{i}/{op}', op, S[i]['lastArgs']): out.skip(f'{op}: no oracle operation')
            elif op in NO_ORACLE_OP:
                out.skip(f'{op}: no oracle operation')
        for p, c in SLOT.items():
            mode, full, k, sl = CALL[c]
            cur = base(mode, k, sl, cols, inter); chain = [dict(cur, op=-1)]
            def push(op, **upd):
                nonlocal cur
                cur = {**cur, **upd}; chain.append(dict(cur, op=op))
            for i in range(1, len(S)):
                st = S[i]
                if st['rekeyState']['started'] and cur['lKeyId'] == 0:
                    push(0, lKeyId=1, keygens=1)
                if c in st['chunkPlans'] and cur['lKeyId'] and not cur['planned']:
                    push(16, planned=True)
                if p in st['lProofs'] and cur['phase'] == 0:
                    push(32 + full, phase=1, **({'keyFull': 1} if full else {'keyPathC': 1}))
                if p in st['lfrProofs'] and cur['phase'] == 1:
                    lfr = st['lfrProofs'][p]
                    push(48, phase=2, proofSha=1)
                    push(64, **({'phase': 3} if lfr['stockVerified'] else {'verifierRefused': True}))
                if p in st['readyRecords'] and cur['phase'] == 3:
                    rd = st['readyRecords'][p]
                    push(80, phase=4, records=1, recordSha=1 if rd['proofSha256'] == st['lfrProofs'][p]['proofSha256'] else 0)
                if p in st['wrapAcceptances'] and cur['wraps'] == 0:
                    rd = st['readyRecords'][p]; cc = 0 if rd['movedBytesSha256'] == rd['proofSha256'] else 1
                    push(96 + cc, movedSha=cur['recordSha'] + cc + (1 if NEG else 0))
                    push(112, wraps=1)
            out.chain(f'{name}_p{p}', chain)
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
