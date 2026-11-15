import re, json, pathlib, sys, hashlib
root = pathlib.Path('/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/openspec')
own = "leaf-proving l-transcript universal-keys node-aggregation gp-hooks call-binding elastic-proving setup-mode box-fleet gpu-proving prover-daemon zero-knowledge stark-security".split()
ctx_of_own = {'leaf-proving':'leaf-proving','node-aggregation':'aggregation','gp-hooks':'aggregation','l-transcript':'bridge','call-binding':'bridge','universal-keys':'key-management','setup-mode':'scheduling','elastic-proving':'scheduling','box-fleet':'box-fleet','gpu-proving':'box-fleet','prover-daemon':'box-fleet','zero-knowledge':'assurance','stark-security':'assurance'}
# reused families -> context
fam_ctx = {'RSL':'leaf-proving','CPR':None,'SPL':None,'NODE':'aggregation','SHP':'aggregation','CLM':'aggregation','CONT':'aggregation','FREG':'aggregation','FVR':'aggregation','GP':'aggregation','LW':'bridge','FLG':'bridge','HDM':'bridge','ANC':'bridge','RKY':'key-management','LPC':None,'PRD':'box-fleet','GPW':'box-fleet','ZK':'assurance','SQG':'assurance','DG':'assurance','FPR':'assurance'}
special = {'CPR-002':'leaf-proving','CPR-003':'key-management','SPL-006':'leaf-proving','SPL-004':'key-management','SPL-005':'key-management','SPL-011':'scheduling',
 'LPC-004':'assurance','LPC-016':'assurance','LPC-005':'box-fleet','LPC-006':'scheduling','LPC-007':'scheduling','LPC-020':'box-fleet','LPC-018':'box-fleet'}
# which reused IDs are in scope
def in_scope(i):
    fam, n = i.rsplit('-',1); n=int(n)
    if fam=='RSL': return n<=8
    if fam=='RKY': return n in (1,2,5)
    if fam=='GP': return n<=4
    if fam=='CPR' or fam=='SPL': return True
    if fam=='LPC': return n<=7 or n in (16,18,20)
    if fam=='GPW': return n==7
    return fam in fam_ctx
out = {}
pat = re.compile(r'^### Requirement: (([A-Z]+)-(\d+))\b.*?\n(.*?)(?=^### Requirement:|\Z)', re.S|re.M)
files = sorted(root.glob('changes/*/specs/*/spec.md'))
for f in files:
    txt = f.read_text()
    chg = f.parts[-4]; cap = f.parts[-2]
    for m in pat.finditer(txt):
        i, fam = m.group(1), m.group(2)
        body = m.group(4)
        sm = re.search(r'^'+re.escape(i)+r':\s*(.*?)\n\s*\n', body, re.S|re.M)
        sent = sm.group(1).strip() if sm else body.strip().split('\n\n')[0].strip()
        if chg=='sherlock-e2e-prototype-20261115':
            c = cap.replace('sherlock-e2e-','')
            if c not in ctx_of_own: continue
            ctx = ctx_of_own[c]; kind='own'
        else:
            if fam not in fam_ctx or not in_scope(i): continue
            ctx = special.get(i, fam_ctx[fam]); kind='reused'
            if ctx is None: ctx = {'CPR':'leaf-proving','SPL':'scheduling','LPC':'assurance'}[fam]
        if i in out:
            out[i]['dups'] = out[i].get('dups',[])+[f"{chg}/{cap}"]
            # later change wins? keep note; completion overrides processor-final
            if 'completion' in chg: out[i].update(sentence=sent, src=f"{chg}/{cap}")
            continue
        out[i] = dict(id=i, fam=fam, ctx=ctx, kind=kind, sentence=sent, src=f"{chg}/{cap}", sha=hashlib.sha256(f.read_bytes()).hexdigest()[:16])
json.dump(out, open('reqs.json','w'), indent=1)
from collections import Counter
print(len(out), Counter(v['ctx'] for v in out.values()), Counter(v['kind'] for v in out.values()))
print([k for k,v in out.items() if 'dups' in v])
