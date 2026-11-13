import math, sys

import importlib.util
src=open(__import__('sys').argv[1] if len(__import__('sys').argv)>1 else 'bits.py').read().split("Lp=[")[0]
exec(src)
def jc(n,D,eta):
    r=D/n; s=math.sqrt(r)
    m=max(math.ceil(s/(2*eta)),3); t=m+0.5
    mu=math.ceil(t/s)-1; h=math.ceil(t*t/(3*r))-1
    A=math.ceil((s+eta)*n)
    th=(n-D)/(A-D)
    return (2*mu-1)*h+th*(h+mu+4*D*mu*h)+(n-D-1)*mu, m
def row(mu,beta,M):
    n=2**(mu+beta); k=2**mu-1
    agree=4/3*math.sqrt(2**mu/n); eta=agree-math.sqrt(k/n)
    c,m=jc(n,k,eta); fac=1 if M==1 else 2*M
    ark=F-math.log2(fac*c)
    cand=F-math.log2(check(mu,beta,M))
    return ark,cand,m
for mu,beta,M in [(19,4,1),(8,10,1),(7,9,1),(18,3,1),(20,4,47),(19,3,511)]:
    print(mu,beta,M,"arklib %.3f cand %.3f m=%d"%row(mu,beta,M))
