# FOREMAN_REPORT

Worker: Grok 4.6 high author
Worktree: `/home/charl/Moriarty/.worktrees/sp01-map-composition-grok`
Task: SP01 composition specified-only financial fragment
Status: specified-only. Independent GPT-6 review is pending.

## Owned files

- `experiments/moriarty-language/spec/successor/financial-fragments/composition.json`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

## Commands actually run

```
python3 /tmp/sp01-composition-build/build.py
python3 -c 'stdlib Fraction/Decimal and integer AMM checks against the fragment'
sha256sum composition.json input-packet.json interrupted-theorem-ledger.json
wc -c owned files
```

No subagent, network, build, proof, install, or commit.

## Hashes

- input-packet.json: `01d8fc881934cc28e5c8ef4bd11c8229f780fbaf2f663371322f064ef3fb5a56`
- interrupted-theorem-ledger.json: `46a7b4aebf9d236c614163b341ec76b1a862c51e5947458cc7c1c40fd37868cd`
- composition.json: `5493e0c1a5216a73aa6fb5edba4b54525a857e648e89a842365d2793afeb27c6`

## What this fragment contains

Six rows and traces: NAM19 plus sequential, disjoint-parallel, shared-state-interleaving, atomic-synchronization, asynchronous-messaging.

NAM19 keeps pinned upstream prefix and remaining events. Independent 90/365 accrual is 7200/73. IPCI payoff stays 0 while notional rises. This is not micro-unit runtime evidence.

Composition oracles are funded integer toys. Sequential pays 10 then 20 USDC. Parallel moves 7 WETH and 11 USDC on disjoint frames. Shared pool swap-then-LP ends at pool WETH 92 / LP 109. Reverse order ends at WETH 96 / LP 112. Atomic borrow 50 USDC, swap to 47 WETH, repay 50 USDC plus 1 USDC fee from Alice pre-fund 51. Async locks 25 USDC, fee 1, refunds 25 at tick 100. Refund does not replenish the cap.

Invalid mutations have concrete mutated input, rejection stage, and unchanged financial state. Fees on the late-receive path stay charged.

`theoremLedger` restates all eight proposed predicates. State typing is separate from expression typing. axioms and mechanizedEvidence are empty.

`commonAcceptance` gives four closed claims and genesis/admin rejection fixtures. All rows point at those fixtures.

## Limitations

This is specified-only design. It is not implementation, not a successor freeze, not a proof, and not protocol conformance.

Predicates are not executed by a language runtime.

NAM19 remaining 26 events are pinned, not re-derived.

Toy integer AMM, work ticks, and named accounts are local profile choices, not registered production limits.

Destination finality on async is an assumed Midnight-model observation, not a proven fact and not a foreign adapter.

Full RP01 and SP10 implementation remain open.
