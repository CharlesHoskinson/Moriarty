#!/usr/bin/env python3
"""Small deterministic algebra check; no GPU/prover timing or acceptance claim."""
import random
P = int('73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001', 16)
def peval(a,x):
    y=0
    for c in reversed(a): y=(y*x+c)%P
    return y
def dft(a,w):
    return [peval(a,pow(w,i,P)) for i in range(len(a))]
def inverse_coset(v,w,z):
    n=len(v)
    a=dft(v,pow(w,-1,P))
    return [(x*pow(n,-1,P)*pow(z,-j,P))%P for j,x in enumerate(a)]
rng=random.Random(20261026)
z=pow(7,(P-1)//3,P)
assert z!=1 and pow(z,3,P)==1
cases=rotchecks=0
for n in (4,8,16):
  for b in (2,4,8):
    N=n*b
    w=pow(7,(P-1)//N,P)
    wn=pow(w,b,P)
    assert pow(w,N,P)==1 and pow(w,N//2,P)!=1
    a=[rng.randrange(P) for _ in range(n)]
    q=[rng.randrange(P) for _ in range(N-n)]
    full=dft([(c*pow(z,j,P))%P for j,c in enumerate(a)]+[0]*(N-n),w)
    assembled=[None]*N
    divided=[None]*N
    for r in range(b):
      shift=z*pow(w,r,P)%P
      tile=dft([(c*pow(shift,j,P))%P for j,c in enumerate(a)],wn)
      denom=(pow(shift,n,P)-1)%P
      assert denom
      for j,val in enumerate(tile):
        i=r+b*j
        assembled[i]=val
        x=shift*pow(wn,j,P)%P
        assert (pow(x,n,P)-1)%P==denom
        # Synthetic numerator with known exact quotient; deg(numerator)<N.
        numerator=denom*peval(q,x)%P
        divided[i]=numerator*pow(denom,-1,P)%P
        for rotation in (-n-1,-2,-1,0,1,2,n+1):
          assert tile[(j+rotation)%n]==full[(i+rotation*b)%N]
          rotchecks+=1
    assert assembled==full
    assert inverse_coset(divided,w,z)==q+[0]*n
    # A wrong stride must be detected, not just a self-consistent round trip.
    assert any(full[(i+b)%N]!=full[(i+1)%N] for i in range(N))
    cases+=1
print('# Coset tile algebra check')
print('> Source: local:clusters/s5-quotient-ntt/wave-1/check_coset_tiles.py')
print('> Collected: 2026-10-26')
print('> Published: 2026-10-26\n')
print(f'SOURCE — PASS: {cases} domain cases; {rotchecks} exact rotated-value comparisons; all extended evaluations and known quotient coefficients equal modulo BLS12-381 scalar modulus.')
print('SOURCE — n in {4,8,16}, b in {2,4,8}; deterministic synthetic inputs; order-three shift derived from generator 7. Wrong-stride negative controls detected in every case.')
print('OPEN QUESTION — This is an algebra check only: no actual Midnight roots/ABI, CUDA execution, real constraint graph, proof, or ledger acceptance was tested; no performance measurements.')
