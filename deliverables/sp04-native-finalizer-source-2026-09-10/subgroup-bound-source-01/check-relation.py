#!/usr/bin/env python3
"""Public mathematical model and source counts, NOT circuit/native execution."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import tarfile

here = Path(__file__).resolve().parent
prior = here.parent
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(prior/'proposal.json') == '6e979f88697d354660599d572ffcf5fb60cc278a45c761e58a77c341f7a18589'
assert sha(prior/'schedule-supplement-01/manifest.json') == 'e03b044388140a3c222ec8b90a27b066bf6f8a173506ddede2b917ded263a354'
for name in ['proposal.json', 'schedule-supplement-01/manifest.json']:
    m = prior/name
    for f, h in json.loads(m.read_text())['files'].items():
        assert sha(m.parent/f) == h
e = json.loads((here/'source-evidence.json').read_text())
assert sha(Path(e['archive']['path'])) == e['archive']['archiveSha256']
with tarfile.open(e['archive']['path']) as t:
    for m in e['members']:
        raw = t.extractfile(m['member']).read()
        assert hashlib.sha256(raw).hexdigest() == m['sha256']
        ls = raw.decode().splitlines()
        for x in m['excerpts']:
            assert '\n'.join(ls[x['start']-1:x['end']]) == x['text']

O = (0, 0, True)
expected_ops = Counter(mul=5, linear=9, zero=3, select=9,
                       inverseWitness=1, equalFixed=1,
                       bitNot=3, bitAnd=1, bitOr=1, bitSelect=2)

def valid(P, p):
    x, y, b = P
    return (0 <= x < p and 0 <= y < p and isinstance(b, bool)
            and ((x == y == 0) if b else (y*y-x*x*x-4) % p == 0))

def complete(P, Q, p):
    """Straight-line field specification; every unused denominator becomes1."""
    assert valid(P, p) and valid(Q, p)
    c = Counter()
    def add(a,b): c['linear']+=1; return (a+b)%p
    def sub(a,b): c['linear']+=1; return (a-b)%p
    def mul(a,b): c['mul']+=1; return a*b%p
    def zero(a): c['zero']+=1; return a==0
    def sel(b,a,d): c['select']+=1; return a if b else d
    def bn(b): c['bitNot']+=1; return not b
    def ba(b,d): c['bitAnd']+=1; return b and d
    def bo(b,d): c['bitOr']+=1; return b or d
    def bs(b,a,d): c['bitSelect']+=1; return a if b else d
    px,py,bp=P; qx,qy,bq=Q
    dx=sub(qx,px); dy=sub(qy,py); yy=add(py,py)
    xx=mul(px,px); xxx=add(add(xx,xx),xx)
    ex=zero(dx); ey=zero(dy); vy=zero(yy)
    active=bo(bn(ex),ba(ey,bn(vy)))
    numerator=sel(ex,xxx,dy); denominator=sel(ex,yy,dx)
    safe_denominator=sel(active,denominator,1)
    assert safe_denominator != 0
    c['inverseWitness']+=1; witness=pow(safe_denominator,-1,p)
    inverse_product=mul(safe_denominator,witness)
    c['equalFixed']+=1; assert inverse_product==1
    slope=mul(numerator,witness)
    rx=sub(mul(slope,slope),add(px,qx))
    ry=sub(mul(slope,sub(px,rx)),py)
    rx=sel(active,rx,0); ry=sel(active,ry,0); rb=bn(active)
    # Q identity returns P; P identity takes precedence and returns Q.
    rx=sel(bq,px,rx); ry=sel(bq,py,ry); rb=bs(bq,bp,rb)
    rx=sel(bp,qx,rx); ry=sel(bp,qy,ry); rb=bs(bp,bq,rb)
    R=(rx,ry,rb)
    assert c==expected_ops and valid(R,p)
    return R

def reference(P,Q,p):
    """Separate conventional branch-based mathematical group law."""
    if P[2]: return Q
    if Q[2]: return P
    x,y,_=P; u,v,_=Q
    if x==u and (y+v)%p==0: return O
    if x==u: slope=3*x*x*pow(2*y,-1,p)%p
    else: slope=(v-y)*pow(u-x,-1,p)%p
    a=(slope*slope-x-u)%p
    return (a,(slope*(x-a)-y)%p,False)

def scalar(n,P,p,law):
    assert n>0
    A=P
    for bit in bin(n)[3:]:
        A=law(A,A,p)
        if bit=='1': A=law(A,P,p)
    return A

toy=[]
for p in [5,7,11,13,17,19,23,31]:
    points=[O]+[(x,y,False) for x in range(p) for y in range(p)
                if (y*y-x*x*x-4)%p==0]
    for P in points:
        for Q in points:
            assert complete(P,Q,p)==reference(P,Q,p)
    toy.append({'prime':p,'points':len(points),'pairsChecked':len(points)**2})

constants=json.loads((prior/'public-constant-results.json').read_text())
p=int(constants['baseModulusHex'],16);r=int(constants['scalarModulusHex'],16)
h=0x396c8c005555e1568c00aaab0000aaab
assert h*r==p+0xd201000000010000
T=(0,2,False)
assert scalar(3,T,p,complete)==O
assert r%3==1 and scalar(r,T,p,complete)==T
assert scalar(r,O,p,complete)==O
# Deterministic public integer point, cofactor-projected using the separate
# complete host reference law. Not a native P3 fixture or proof verification.
for x in range(1,100):
    y=pow((x**3+4)%p,(p+1)//4,p)
    if (y*y-x*x*x-4)%p: continue
    Q=scalar(h,(x,y,False),p,reference)
    if not Q[2]: break
else: raise AssertionError('public mathematical point not found')
assert scalar(r,Q,p,reference)==O and scalar(r,Q,p,complete)==O

doubles=r.bit_length()-1; adds=r.bit_count()-1; calls=doubles+adds
assert (doubles,adds,calls)==(254,133,387)
# Prior source supports45/mul,56/normalized linear,7/field select,
#14/field witness and14/fixed field equality. Normalized Fp zero detection:
#7 native fixed equalities*(2 arithmetic+fixed-zero+copy) +6 AND*(fixed-one+mul).
zero_rows=7*4+6*2
bit_rows=3*2+1*2+1+2
per_add=5*45+9*56+9*7+3*zero_rows+14+14+bit_rows
assert zero_rows==40 and bit_rows==11 and per_add==951
# Input assignment93 from prior tail +2 conditional zero-coordinate assertions:
#2 field selections(14) +2 fixed field equalities(28). Constants0/1 cost14.
input_rows=93+14+28
subgroup_rows=input_rows+14+calls*per_add+2  # final true bit fixed/copy
assert subgroup_rows==368188
print(json.dumps({'status':'PASS_PUBLIC_RELATION_AND_COUNT_MODEL_REVIEW_REQUIRED',
 'sourceOpsPerCompleteAdd':dict(expected_ops), 'toyCurves':toy,
 'actualFieldMath':{'order3PointRejectedByLiteralR':True,'identityAccepted':True,
                   'publicProjectedNonidentityPointAccepted':True,
                   'scope':'Python field arithmetic; no native fixture/circuit/proof'},
 'literalIntegerR':{'bits':r.bit_length(),'ones':r.bit_count(),
                   'doubles':doubles,'adds':adds,'completeAddCalls':calls,
                   'mustNotReduceIntoFq':True},
 'conditionalRegionBounds':{'fpZero':zero_rows,'completeAdd':per_add,
                            'inputOnCurveAndCanonicalIdentity':input_rows,
                            'onePointMembership':subgroup_rows},
 'completeVerifierRowBound':None,'nativeOperations':0,
 'scope':'Unimplemented field-level subgroup relation specification. No design approval, F0 go, synthesis/domain fit, backend, SRS or resource choice.'},indent=2))
