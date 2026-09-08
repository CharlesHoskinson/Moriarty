# SP01 markets fragment (specified-only)

Group `markets`. Status `specified-only`. Not a semantic freeze, implementation, proof, or protocol-conformance claim. Pending independent GPT-6 Astra review.

## Owned files

| path | bytes | sha256 |
| --- | --- | --- |
| experiments/moriarty-language/spec/successor/financial-fragments/markets.json | 226823 | `96216e68434e03d189b29ec964ab2faf4b97bc1d1c464f46561805f720236f53` |

Input packet `6641cbbadf6505b2404ce741e4917b81eba9b3105b9de98885bef2451ea03a71` matches binding.json. Worktree `/home/charl/Moriarty/.worktrees/sp01-map-markets-grok`. Base commit `45c42cf95d0e500f8307e99214c8f23ebc701ce6`.

## Cases

Six rows, six traces, eight mutations (six preserved IDs plus two added), six verification entries.

- exact-output swap: preserved floor_div 19743; accounts alice/pool; fee 30 stays in pool; debit remaining 0; work 8; status settled.
- concentrated liquidity: toy sqrtP=tick, SY=1024; zeroForOne in 477 / out 4882 floor; reverse quote ceil 4883 / floor 432; fee growth 1; ticksVisited 105,100; reject ceil 4883 and 4-tick walk.
- iterative AMM: Ann=A*n**n=400; get_D D=1999 in 1 floor step; get_y y=850 in 5 steps; swap 50/50; nonconvergent sibling xp=[10**9,1] A=1 after 8 steps D=39020571 abs=19508019 reject.
- ordered redemption: price 2; burn 150 stable; coll 297+fee 3; T1 surplus 100; T2 debt 50 remains; T3 untouched; reject T3-first.
- shared vault: +delta increases vault inventory; transfers 10/9/1; final 0 from settlement; reject forged zero without v2; reject hook fee 2.
- margin/funding: position long 10, IM 20, funding debt 5, pnl 40 nominal; executed withdraw 75; collateral 25; reject withdraw 130.

Findings addressed as design-oracle text: R04, R07, R09, R10. Not executed by a language runtime.

## Commands actually run

Independent python3 stdlib integer arithmetic (not Moriarty evaluator). Each command is also stored on the matching verification entry.

```
python3 -c "rI,rO,aI=1000000,2000000,10000;n,d=997,1000;adj=aI*n;num=adj*rO;den=rI*d+adj;o=num//den;print(adj,num,den,o,num%den,aI-(aI*n//d),rI+aI,rO-o,50000-aI,1000+o)"
# 9970000 19940000000000 1009970000 19743 162290000 30 1010000 1980257 40000 20743

python3 -c "L,SY=1000000,1024;n=L*5;print(n//SY,(n+SY-1)//SY,n%SY,n//10500,(n+10499)//10500,n%10500,n//11550,477*3//1000,4883*3//1000,10000-477,1000+n//SY,100000+477,200000-n//SY)"
# 4882 4883 832 476 477 2000 432 1 14 9523 5882 100477 195118

python3 -c "
A,n=100,2;Ann=A*n**n;xp=[1100,900];S=sum(xp);D=S
D_P=D
for x in xp:
 D_P=D_P*D//(n*x)
D=(Ann*S+D_P*n)*D//((Ann-1)*D+(n+1)*D_P)
print('D',D,'Ann',Ann)
x_in=1150;c=D;c=c*D//(x_in*n);c=c*D//(Ann*n);b=x_in+D//Ann;y=D
for i in range(8):
 yprev=y;y=(y*y+c)//(2*y+b-D)
 if abs(y-yprev)<=1 and i>0: break
print('y',y,'c',c,'b',b,'i',i+1)
xp2=[10**9,1];A2=1;Ann2=A2*4;S2=sum(xp2);D2=S2
for i in range(8):
 DP=D2
 for x in xp2: DP=DP*D2//(2*x)
 D2=(Ann2*S2+DP*2)*D2//((Ann2-1)*D2+(3)*DP)
print('non',D2)
"
# D 1999 Ann 400
# y 850 c 4340 b 1154 i 5
# non 39020571

python3 -c "price,redeem,feeBps=2,150,100;g=redeem*price;f=g*feeBps//10000;print(g,f,g-f,100,50,100-50,300-200,280-100,980-g,150,0+20+10+30+980)"
# 300 3 297 100 50 50 100 180 680 150 1040

python3 -c "print(50-10,5+9,1000+10,1000-9-1,0+1,50+1000,5+1000,-9+-1)"
# 40 14 1010 990 1 1050 1005 -10

python3 -c "print(10*(10-6),100-20-5,100-75,10+75,10+100,20+5,130<=(100-20-5))"
# 40 75 25 85 110 25 False
```

sha256 of markets.json after those checks: `96216e68434e03d189b29ec964ab2faf4b97bc1d1c464f46561805f720236f53`.

## Limitations

- Toy profile: perActionWork 64, lifetime 1024, reserve 16, Newton cap 8, three ticks, SY=1024, A=100, price 2, fee 100 bps, IM 20. Not production limits.
- Source gaps preserved: intents.md inequality, no Uniswap V3 Q64.96, no Curve newton_D pin, no Liquity V2, no Balancer V3 behavior id, Velocity/funding-index unknown.
- Foreign observations are bounded Midnight-model assumptions, not foreign adapters.
- Design predicates are not claimed to run on a Moriarty language runtime.
- No RP01/sprint completion, no implementation, no mechanized proof.
