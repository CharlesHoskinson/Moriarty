#!/usr/bin/env python3
"""Key Management: project Quint traces (record `km`, 3 boxes, 2 programs, up to 2 keygens) onto the oracle
(one global state per trace). Witness fields (listRows, rowsCommitted, treeHoldsAll, listD, d3 bits) are set on
the pre-state from the model before each op; expected values come from the model state (keygenEpoch, rekeyPhase,
gatesPassedCount, statusMark, installed manifest per box) or from counted model events (proofs, keyRoot).
Stage map: startAndRunKeygen -> keygen (0); runGatesAndD3Pass -> gatePass g (16+g) for each missing gate, then
complete (32); runGatesFailX7 -> gatePass of the 11 other gates, then a refusal check of complete
(gates-incomplete); runD3FailGrindable100Point9 -> refusal check of complete (d3-below-bound);
distributeKeysToBox b -> 48+b; startProofOnBox b -> 64+b; runSetupModeClean b -> 80+b; buildProgramKeyRoot p
-> 96+p; passSecurityGateAllChecks -> 112; appendRowAfterKeygen -> changeList (128+d, d = sha step - 1).
Refusals: refuseRekey* -> keygen refusal checks; refuseCollidingRslLeafIdx -> keygen with rslKeyIdx = keydagIdx;
refuseStartProofManifestMismatch b -> startProof refusal (manifest-mismatch).
usage: project_key_management.py TRACEDIR OUTDIR"""
import sys, glob, os, json
sys.path.insert(0, os.path.dirname(__file__))
from common import load, tag, Out

ROWS = ['RowFp3', 'RowAnchorSlot15', 'RowCanonicalRange', 'RowRslStmtBind', 'RowRslKeyLeafIdx', 'RowEraCheck']
BOX = {'BoxMuninn': 0, 'BoxHuginn': 1, 'BoxOdin': 2}
PROG = {'ProgT02Increment': 0, 'ProgD04Withdraw': 1}
PHASE = {'PhaseRekeyIdle': 0, 'PhaseKeygenComplete': 1, 'PhaseRekeyGateFailed': 1, 'PhaseRekeyDone': 2}
X7 = 6
NEG = bool(os.environ.get('NEGATIVE_CONTROL'))
NO_OP = {'abortSetupOnKeyTamper', 'verifyAndSettleFullTree', 'verifyAndSettlePathC', 'refuseUnlistedChildKey',
         'refuseFreezeBeforeGate', 'freezeUniversalKeys', 'markManifestQualifiedPrematurely'}

def tagged(v):
    return tag(v) if isinstance(v, dict) else v

def boxes(km):
    out = {}
    for k, v in km['boxes'].items():
        name = tagged(k) if isinstance(k, dict) else (json.loads(k)['tag'] if isinstance(k, str) and k.startswith('{') else k)
        out[BOX[name]] = v
    return out

def witness(km):
    rows = {tagged(r) for r in km['currentListRows']}
    unc = km['uncommittedRows']
    return dict(listRows=sum(1 << i for i, r in enumerate(ROWS) if r in rows), rowsCommitted=len(unc) == 0,
                treeHoldsAll=km['treeDescendantOfAllRows'], listD=km['currentListSha'],
                d3MinBits=km['d3MinTermDeciBits'] // 10, d3MinGrindBits=km['d3MinGrindableDeciBits'] // 10)

def base(km):
    return dict(witness(km), rslKeyIdx=4, keydagIdx=3, manifestD=0, manifestListD=0, keygens=0, phase=0, gatesPass=0,
                qualified=False, box0=0, box1=0, box2=0, proofs=0, setupRuns=0, keyRoot=0, programs=0)

def main(tdir, odir):
    out = Out(odir)
    for f in sorted(glob.glob(os.path.join(tdir, '*.itf.json'))):
        name = os.path.basename(f)[:-9]; S = load(f)
        km0 = S[0]['km']; cur = base(km0); chain = [dict(cur, op=-1)]
        epoch_of = {100: 0}            # manifest sha -> oracle manifest id
        seg = [0]
        def pre(km):
            # witness fields change only between oracle ops: a change after an op starts a new chain segment
            nonlocal cur, chain
            w = witness(km)
            if all(cur[k] == v for k, v in w.items()):
                return
            cur = {**cur, **w}
            if len(chain) > 1:
                out.chain(f'{name}_seg{seg[0]}', chain); seg[0] += 1
                chain = [dict(cur, op=-1)]
            else:
                chain[-1] = dict(cur, op=-1)
        def push(op, **upd):
            nonlocal cur
            cur = {**cur, **upd}; chain.append(dict(cur, op=op))
        for i in range(1, len(S)):
            km, kp = S[i]['km'], S[i - 1]['km']; op = S[i]['lastOp']; nm = f'{name}/s{i}/{op}'
            if op in ('dropListRowBeforeRekey', 'markRowUncommitted', 'checkoutNonDescendantTree', 'prepareValidListAndTree'):
                pre(km)
            elif op in ('refuseRekeyMissingRows', 'refuseRekeyUncommittedRow', 'refuseRekeyNonDescendantTree'):
                code = {'refuseRekeyMissingRows': 'list-row-missing', 'refuseRekeyUncommittedRow': 'row-uncommitted',
                        'refuseRekeyNonDescendantTree': 'tree-not-descendant'}[op]
                pre(kp); out.refusal(nm, {**cur}, 0, [code])
            elif op == 'refuseCollidingRslLeafIdx':
                b = {**base(km0), 'rslKeyIdx': 3}; out.refusal(nm, b, 0, ['key-leaf-collision'])
            elif op == 'startAndRunKeygen':
                pre(kp); e = km['keygenEpoch']; epoch_of[km['manifest']['manifestSha256']] = e
                push(0, keygens=e, manifestD=e, manifestListD=km['manifest']['listSha256'],
                     phase=PHASE[tag(km['rekeyPhase'])], gatesPass=0, qualified=tag(km['manifest']['statusMark']) == 'MarkQualified')
                cur['listD'] = km['currentListSha']
            elif op in ('runGatesAndD3Pass', 'runGatesFailX7'):
                pre(km)
                for g in range(12):
                    if g == X7 and op == 'runGatesFailX7':
                        continue
                    if not cur['gatesPass'] >> g & 1:
                        push(16 + g, gatesPass=cur['gatesPass'] | 1 << g)
                if op == 'runGatesAndD3Pass':
                    push(32, phase=PHASE[tag(km['rekeyPhase'])])
                elif cur['phase'] == 1:
                    out.refusal(nm, {**cur}, 32, ['gates-incomplete'])
            elif op == 'runD3FailGrindable100Point9':
                if cur['phase'] == 1:
                    out.refusal(nm, {**cur, **witness(km), 'gatesPass': 4095}, 32, ['d3-below-bound'])
                else:
                    out.skip('d3 fail outside phase 1')
            elif op in ('distributeKeysToBox', 'runSetupModeClean', 'startProofOnBox', 'refuseStartProofManifestMismatch'):
                bp, bn = boxes(kp), boxes(km)
                b = [x for x in range(3) if bp[x] != bn[x]] or [None]
                b = b[0]
                if op == 'refuseStartProofManifestMismatch':
                    ref = km['lastRefusal']['value']; b = BOX[tagged(ref[0])]
                    if cur['phase'] == 2:
                        out.refusal(nm, {**cur}, 64 + b, ['manifest-mismatch'])
                    else:
                        out.refusal(nm, {**cur}, 64 + b, ['wrong-phase'])
                    continue
                if b is None:
                    out.skip(f'{op}: no box changed'); continue
                if op == 'distributeKeysToBox':
                    push(48 + b, **{f'box{b}': epoch_of[bn[b]['installedManifestSha']] + (1 if NEG else 0)})
                elif op == 'runSetupModeClean':
                    push(80 + b, setupRuns=cur['setupRuns'] + 1)
                else:
                    push(64 + b, proofs=cur['proofs'] + 1)
            elif op == 'buildProgramKeyRoot':
                new = [k for k, v in km['programKeyRoots'].items() if v != kp['programKeyRoots'].get(k)]
                if not new:
                    out.skip('buildProgramKeyRoot: no program changed'); continue
                p = PROG[json.loads(new[0])['tag'] if isinstance(new[0], str) else tagged(new[0])]
                push(96 + p, keyRoot=cur['keyRoot'] * 16 + p + 1, programs=cur['programs'] + 1)
            elif op == 'passSecurityGateAllChecks':
                push(112, qualified=tag(km['manifest']['statusMark']) == 'MarkQualified')
            elif op == 'appendRowAfterKeygen':
                left = km['currentListSha'] - kp['currentListSha']
                if left <= 0:
                    pre(km); out.skip('appendRowAfterKeygen: list sha did not grow'); continue
                while left > 0:
                    d = min(left, 16) - 1; left -= d + 1
                    push(128 + d, listD=cur['listD'] + d + 1)
            elif op in NO_OP:
                out.skip(f'{op}: no oracle operation')
            else:
                out.skip(f'{op}: unmapped')
        out.chain(f'{name}_seg{seg[0]}', chain)
    out.close()

if __name__ == '__main__': main(*sys.argv[1:3])
