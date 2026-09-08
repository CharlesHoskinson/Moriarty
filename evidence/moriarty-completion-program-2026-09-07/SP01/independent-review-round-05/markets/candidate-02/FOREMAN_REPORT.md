# SP01 markets fragment (specified-only correction)

Group `markets`. Status `specified-only`. Not a semantic freeze, implementation, proof, runtime, network, or full-map acceptance. Candidate-01 identities preserved. Pending independent GPT-6 Astra review.

## Owned files

| path | bytes | sha256 |
| --- | --- | --- |
| experiments/moriarty-language/spec/successor/financial-fragments/markets.json | 382432 | `89186944596d148b0183ec87638792c7c60b09299fbfd35831ad4dbd0becbf02` |

Worktree `/home/charl/Moriarty/.worktrees/sp01-map-markets-grok`. Base `45c42cf95d0e500f8307e99214c8f23ebc701ce6`. Input packet `6641cbbadf6505b2404ce741e4917b81eba9b3105b9de98885bef2451ea03a71`.

## Counts

Six rows, six traces, eleven mutations (eight preserved IDs plus three added controls). Source pins, owner packages, closure sprint/task unchanged.

## M01-M07 (specified-only, not label-only)

- M01: typed-patch-inheritance/1 with exact preState refs. Every positive postState materializes all live fields. Redemption principal 0/50/100, accrual 0. Margin formula 25-20-5=0, controller trader retained. Hook permission retained.
- M02: four claims bind complete input/intent/plan/state/effects/authorities/work/duties/history plus funded genesis/admin contents and current-head unique consumption. Added work-reset and stale-currentness mutations.
- M03: footprints list actual reads/writes at named-record granularity. Bounds come from those sets. Frame is writes, not read-union-writes.
- M04: margin sibling withdraw 90 with explicit cap 90 (not inherited 75). PnL-as-cash 115 would accept; spendable 75 fails. Redemption burn 150 with T3-100 then T1-50; fees/collateral pass; order fails first.
- M05: tick unit is segments=len(ticksVisited)-1. Unknown-tick mutation kept (3<=3). New over-cap path 5 initialized entries / 4 segments.
- M06: toy inventory at 105 is token0=432 token1=4882. Positive 105->100 is a touch, not a cross. Bounded-cross control crosses 100: activeLiquidity 0, feeGrowthOutside0[100]=1. Tick 105 is a current-tick marker, not a liquidity bound. Not Uniswap.
- M07: numeric original/cumulative/remaining for trader caps and outgoing pool/vault/collateral/hook debits; separate nominal debt. Rejection consumes ordinary work, not reserve or caps. burnSink is irrevocably nonspendable; spendable supply 150->0.

Cash oracles unchanged: 19743; 477/4882; D=1999 y=850; burn 150 coll 297+3; vault 10/9/1; withdraw 75.

## Commands actually run

Independent python3 stdlib. Exit 0 on all. Not Moriarty evaluator.

```
python3 -c "rI,rO,aI=1000000,2000000,10000;n,d=997,1000;adj=aI*n;num=adj*rO;den=rI*d+adj;o=num//den;print(adj,num,den,o,num%den,aI-(aI*n//d),rI+aI,rO-o,50000-aI,1000+o)"
# 9970000 19940000000000 1009970000 19743 162290000 30 1010000 1980257 40000 20743

python3 -c "L,SY=1000000,1024;n=L*5;print(n//SY,(n+SY-1)//SY,n%SY,n//10500,(n+10499)//10500,n%10500,n//11550,477*3//1000,4883*3//1000,10000-477,1000+n//SY,100000+477,200000-n//SY)"
# 4882 4883 832 476 477 2000 432 1 14 9523 5882 100477 195118

python3 -c "L,SY=1000000,1024;print('inv105', L*(110-105)//(105*110), L*(105-100)//SY); print('inv100', L*(110-100)//(100*110), L*(100-100)//SY); print('crossL', 1000000-1000000, 1-0)"
# inv105 432 4882 / inv100 909 0 / crossL 0 1

python3 -c "<newton get_D/get_y and 8-step nonconv>"
# D 1999 Ann 400 / y 850 c 4340 b 1154 i 5 / non 39020571 / abs@8 19508019

python3 -c "price,redeem,feeBps=2,150,100;g=redeem*price;f=g*feeBps//10000;print(g,f,g-f,100,50,100-50,300-200,280-100,980-g,150,0+20+10+30+980)"
# 300 3 297 100 50 50 100 180 680 150 1040

python3 -c "print(50-10,5+9,1000+10,1000-9-1,0+1,50+1000,5+1000,-9+-1)"
# 40 14 1010 990 1 1050 1005 -10

python3 -c "print(10*(10-6),100-20-5,100-75,10+75,10+100,20+5); print('post',25-20-5); print('w90',90<=100,90<=75,90<=75+40)"
# 40 75 25 85 110 25 / post 0 / w90 True False True

python3 -c "print(len(['105','100'])-1, len(['105','100','95','90'])-1, len(['105','100','95','90','85'])-1, 1024-40)"
# 1 3 4 984
```

## Limitations

Toy profile only. Source gaps unchanged. No runtime, proof, Uniswap/Curve/Liquity/Balancer/Velocity conformance, or sprint completion.
