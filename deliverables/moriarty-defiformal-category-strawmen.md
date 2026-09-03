# Moriarty canonical DeFi application strawmen

<!-- markdownlint-disable MD013 MD060 -->

Date: 2026-09-03  
Language status: non-executable surface-syntax design sketches  
Corpus pin: `CharlesHoskinson/defiformal@8ae0bbfaa3193078d1cabf6999db1382985b7f95`

These applications test whether a bounded Moriarty language can express the
behavior behind DeFiFormal's 72 protocol rows. They are not claims that the
named protocols implement identical economics. They also are not valid input
to a current parser: syntax, typing, elaboration, and semantics remain design
work. Each sketch must eventually compile to finite Moriarty Core, readable
Compact, and ZKIR with an explicit resource and lifetime certificate.

The primary human taxonomy is M2+M3: six economic families plus Prediction over
mandatory facets. M5—actors, transitions, obligations, conservation, liveness,
and failure modes—is the internal formal profile. The twelve D-codes and M4+
remain legacy benchmark and migration views, not Core constructors. D12 maps to
two legacy patterns, yielding thirteen regression patterns for twelve old
labels.

## Primary family demonstrations and Marlowe boundary

| Family | Primary Moriarty demonstration | What Marlowe V1 can express | Required Moriarty addition | Explicitly outside Core |
|---|---|---|---|---|
| F1 Exchange | finite constant-product or bounded-batch swap | finite bilateral deposits, atomic transfer, slippage choice, refund timeout | token-indexed amounts, reusable checked pricing library, atomic-all actions | order routing, MEV policy, market discovery, persistent permissionless pool operation |
| F2 Credit | over-collateralized term loan | fixed participants, deposits, installment schedule, price choices, liquidation branch, terminal refund | typed ratios/units, fresh attestation type, bounded mandate and attributed obligations | oracle truth, variable-rate governance, pooled-liquidity discovery, liquidator willingness |
| F3 Derivatives | fully collateralized option or finite-horizon margin contract | finite option/CfD payoff, exercise choice, margin escrow, expiry | typed payoff packages, bounded schedule elaboration, explicit oracle freshness and units | matching engine, insurance fund, perpetual rollover, economic adequacy of margin |
| F4 Consensus-position claims | bounded liquid-staking receipt accounting | finite deposit, reward/slash choices, delayed payout | authority sort and typed external service capability | validator lifecycle, consensus correctness, slashing adjudication, exit availability |
| F5 Tokenized off-chain claims | capped subscription/redemption state machine | finite subscription, redemption request, issuer-controlled branches and payouts | typed obligor/authority, attestation and freeze/upgrade capability manifest | reserves, custody, register of record, bankruptcy remoteness, legal enforceability |
| F6 Delegated asset management | bounded allocation mandate | fixed deposits, choices, capped transfers and periodic settlement | first-class bounded mandate, strategy/package sort, authority scope and revocation | discretionary investment quality, adapter honesty, market execution, open-ended strategy code |
| P Prediction | finite conditional-token market | escrow, a resolution `Choice`, branch-specific payout, timeout fallback | audited split/merge accounting for outcome claims and resolution attestation | truth of the event, adjudicator independence, market making and dispute governance |

This matrix is intentionally asymmetric. “Marlowe can express” means a finite
agreement skeleton can be encoded with its existing algebra; it does not mean
Marlowe reproduces the full named protocol, proves an oracle statement, or
solves persistent shared-liquidity operation. Likewise, a Moriarty Core proof
does not discharge the last column.

## Coverage roster

| Legacy tag | Canonical pattern | Rows | Protocol/application labels |
|---|---|---:|---|
| D01 spot | `exchange.bounded_batch_swap` | 5 | Uniswap; PancakeSwap; Curve; Raydium; Fluid |
| D02 lending | `credit.collateralized_term_loan` | 6 | Aave V3; Morpho; SparkLend; JustLend V1; Maple; Compound V3 |
| D03 CDP | `money.collateralized_debt_position` | 6 | Sky; Ethena; USDD; Lista CDP; Liquity; crvUSD |
| D04 staking | `stake.delegation_receipt` | 5 | Lido; Binance staked ETH; EigenCloud/EigenLayer; ether.fi; Babylon |
| D05 perpetuals | `derivative.finite_horizon_perpetual` | 7 | Hyperliquid; ApeX; Aster; Lighter; edgeX; Jupiter Perpetual Exchange; GMX V2 Perps |
| D06 yield/vaults | `asset_management.capped_share_vault` | 8 | Pendle; Spark Savings; Convex; CIAN; Huma; Yearn; Beefy; Steakhouse Financial |
| D07 bridges | `settlement.threshold_attested_bridge` | 7 | WBTC; LayerZero V2; Coinbase Bridge/cbBTC; Hyperliquid Bridge; BTCB; Circle CCTP; Across |
| D08 intents | `execution.bounded_solver_intent` | 8 | LiquidMesh; Binance Wallet; OKX DEX; Jupiter; KyberSwap; DFlow; 1inch; CoW Swap |
| D09 RWA | `capital.tokenized_asset_subscription` | 5 | Ondo; Circle USYC; BlackRock BUIDL; Maple Finance; Centrifuge |
| D10 options | `derivative.european_option` | 5 | Derive; Rysk; Hegic; Aevo; Panoptic |
| D11 fiat stablecoins | `money.reserve_attested_stablecoin` | 5 | USDT; USDC; USD1; USDG; PYUSD |
| D12 prediction | `derivative.event_contingent_market` | 3 | Kalshi; Polymarket; Azuro |
| D12 former “other” | `asset_management.delegated_curator_vault` | 2 | Steakhouse Financial Risk Curators; Grove Finance |

The primary machine-readable row-level mapping is
`evidence/defiformal-72-protocol-family-facet-crosswalk-2026-09-02.csv`.
The historical M4+ view remains at
`evidence/defiformal-72-protocol-m4plus-crosswalk-2026-09-02.csv`.

## Shared draft conventions

`Amount<T>` is token-indexed. `Ratio<U,V>`, `Timestamp`, and `Duration` are
distinct types. `for static` and fixed vectors elaborate away before Core.
Every `await` has a finite deadline and explicit fallback. `attest` consumes a
signed, unit-typed, fresh, replay-protected statement. `capability` names an
external trust/effect boundary. `invariant` becomes a static obligation and a
runtime transition check where needed. `close` must discharge or explicitly
route every remaining balance and obligation.

## D01 — bounded batch swap

M4+: `PF-EXCH + IN-SPOT + ME-AMM`; an order-book or RFQ package can replace the
pricing library without changing settlement Core.

```moriarty
contract BoundedBatchSwap<A,B>(
  maker: Party, taker: Party,
  offered: Amount<A>, minReceived: Amount<B>,
  reserves: Pool<A,B>, expiry: Timestamp
) effects { receive, send } {
  invariant conserve(A) and conserve(B);
  await taker deposits quote.constantProduct(reserves, offered)
    and maker deposits offered
    atomic until expiry
    else refund all;
  require received<B> >= minReceived;
  pay taker offered;
  pay maker received<B>;
  close;
}
```

Core proves conservation, authorization, minimum receipt, atomicity, and
refund. Pool discovery, routing, MEV ordering, and market liquidity remain
Runtime/application concerns. Marlowe V1 can encode a finite swap, but it has no
native typed units, reusable pricing package, or pool lifecycle.

## D02 — collateralized term loan

M4+: `PF-CREDIT + IN-DEBT + ME-DEBT + ME-COLL`.

```moriarty
contract CollateralizedTermLoan<C,D,const N>(
  borrower: Party, lender: Party,
  principal: Amount<D>, collateral: Amount<C>,
  schedule: Vector<N, Instalment<D>>, price: Oracle<Ratio<D,C>>,
  maturity: Timestamp
) effects { receive, send, attest(price) } {
  invariant debt >= 0 and collateralLocked >= 0;
  atomic all {
    lender deposits principal;
    borrower deposits collateral;
  } until schedule[0].due else refund all;
  pay borrower principal;
  for static instalment in schedule {
    await ordered any {
      borrower deposits instalment.amount -> pay lender instalment.amount;
      liquidator proves unhealthy(attest price fresh instalment.maxAge)
        -> settleCollateral(lender, borrower);
    } until instalment.due + instalment.grace
      else settleCollateral(lender, borrower);
  }
  returnResidualCollateral(borrower);
  close;
}
```

Variable-rate markets, pooled lenders, flash liquidity, and governance are
separate bounded libraries/capabilities. The reference application establishes
the shared debt, collateral, liquidation, and terminal paths.

## D03 — collateralized monetary claim

M4+: `PF-CREDIT + PF-MONEY + IN-DEBT + IN-MONEY + ME-COLL + ME-ISSUE`.

```moriarty
capability StablePolicy<S> permits { mint<S>, burn<S> };

contract CollateralizedDebtPosition<C,S,const N>(
  owner: Party, collateral: Amount<C>, ceiling: Amount<S>,
  checks: Vector<N, Timestamp>, price: Oracle<Ratio<S,C>>,
  policy: StablePolicy<S>, maturity: Timestamp
) effects { receive, send, mint(policy), burn(policy), attest(price) } {
  invariant issued <= ceiling;
  invariant collateralValue(attest price) >= issued * liquidationRatio;
  await owner deposits collateral until checks[0] else refund all;
  await owner chooses mintAmount in [0, ceiling] until checks[0]
    -> mint owner mintAmount;
  for static t in checks {
    await ordered any {
      owner repays<S> -> burn repayment;
      keeper proves undercollateralized(attest price fresh maxAge)
        -> liquidate(owner);
    } until t else liquidateIfRequired(owner);
  }
  require issued == 0 before returnCollateral(owner);
  close at maturity;
}
```

The external minting policy is separately audited and does not inherit
Moriarty's guarantees. Peg stability, centralized reserves, and hedging venues
remain explicit capabilities or assumptions.

## D04 — delegation receipt with bounded exit

M4+: `PF-STAKE + IN-STAKE + ME-STAKE`.

```moriarty
capability StakingService<S,R> permits { delegate<S>, exit<S>, attestSlash<R> };

contract DelegationReceipt<S,R,const E>(
  staker: Party, stake: Amount<S>, service: StakingService<S,R>,
  epochs: Vector<E, Epoch>, exitDelay: Duration
) effects { receive, send, external(service) } {
  await staker deposits stake until epochs[0].start else close;
  external service.delegate(stake);
  for static epoch in epochs {
    let report = attest service.rewardAndSlash(epoch) fresh epoch.finality;
    applyReward(report.reward);
    applyPenaltyBounded(report.slash, max = stake + rewards);
  }
  await staker requests exit until epochs.last.end else requestExit;
  await service confirms exit until now + exitDelay else markDelayed;
  pay staker stake + rewards - penalties;
  close;
}
```

Moriarty can prove receipt accounting and penalty bounds. Validator duties,
consensus correctness, AVS adjudication, and exit availability are external.

## D05 — finite-horizon perpetual profile

M4+: `PF-EXCH + IN-DERIV:perpetual + ME-MARGIN`. A strictly finite Moriarty
contract cannot be literally perpetual; continuation requires a new, explicit
roll contract.

```moriarty
contract FiniteHorizonPerpetual<Q,const N>(
  long: Party, short: Party, margin: Amount<Q>,
  marks: Vector<N, FundingWindow>, index: Oracle<Price<Q>>
) effects { receive, send, attest(index) } {
  atomic all { long deposits margin; short deposits margin; }
    until marks[0].open else refund all;
  for static window in marks {
    let mark = attest index at window.close fresh window.maxAge;
    transferFunding(mark, window.rate);
    await ordered any {
      long addsMargin; short addsMargin;
      keeper proves maintenanceBreach(mark) -> liquidateAt(mark);
    } until window.close + window.grace else markToMarket(mark);
  }
  settleAt(attest index at marks.last.close);
  close;
}
```

Order matching, insurance funds, auto-deleveraging, and oracle publication are
separate modules. The certificate fixes `N`, maximum margin updates, and expiry.

## D06 — capped share vault

M4+: `PF-AM + IN-SHARE + ME-ALLOC`, with control facets distinguishing rules,
automation, and delegated curators.

```moriarty
capability Adapter<A> permits { deposit<A>, withdraw<A>, attestPosition<A> };

contract CappedShareVault<A,const K,const N>(
  manager: Party, members: Vector<N, Party>, adapters: Vector<K, Adapter<A>>,
  caps: Vector<K, Amount<A>>, periods: Vector<P, Timestamp>
) effects { receive, send, external(adapters) } {
  invariant totalAssets == liquidAssets + sum(attestedPositions);
  invariant sharesOutstanding >= 0;
  await members deposit<A> and receive proRataShares until periods[0];
  for static t in periods {
    await manager proposes allocation in caps until t
      -> rebalanceOnlyThrough(adapters, allocation)
      else preserveCurrentAllocation;
    publishCommittedNAV(t);
    serviceBoundedRedemptions(t);
  }
  unwind(adapters);
  redeemAllProRata;
  close;
}
```

Adapter behavior, price discovery, and discretionary investment quality remain
outside Core. The contract proves cap enforcement, authority, and share/asset
accounting under declared adapter attestations.

## D07 — threshold-attested cross-domain escrow

M4+: primarily `ME-XDOM` plus settlement, verification, finality, and
data-availability trust facets; a bridge is not automatically a financial
product.

```moriarty
capability RemoteFinality<D,M> permits { attestFinal<D,M> };

contract ThresholdAttestedBridge<A,D,const V,const T>(
  sender: Party, recipient: RemoteParty<D>, amount: Amount<A>,
  verifiers: Vector<V, RemoteFinality<D,Message>>, threshold: Nat<T>,
  expiry: Timestamp
) effects { receive, send, attest(verifiers) } {
  require 1 <= threshold and threshold <= V;
  await sender deposits amount until expiry else close;
  let message = commit(domain = D, recipient, amount, nonce, sourceState);
  await threshold distinct verifiers attestFinal(message)
    atomic until expiry
    -> releaseWrappedOrNative(recipient, amount)
    else refund sender amount;
  close;
}
```

The Core prevents replay, substitution, double release, and ambiguous timeout.
It does not prove remote consensus, verifier honesty, relayer liveness,
liquidity, or data availability.

## D08 — bounded solver intent

M4+: `ME-INTENT` with execution and order-flow facets. The underlying product
can be exchange, payment, or another family.

```moriarty
contract BoundedSolverIntent<A,B,const K>(
  user: Party, sell: Amount<A>, minBuy: Amount<B>,
  solvers: Vector<K, Party>, bidClose: Timestamp, settleBy: Timestamp
) effects { receive, send } {
  await user deposits sell until bidClose else close;
  await each solver may commit Bid<A,B> until bidClose;
  await each committed solver may reveal until bidClose + revealWindow;
  let winner = selectDeterministically(validBids, maximize buyAmount,
                                      tieBreak = lowestCommitmentHash);
  await winner deposits winner.buyAmount atomic until settleBy
    -> pay user winner.buyAmount; pay winner sell
    else slashBondAndRefund(user);
  close;
}
```

Solver discovery, private networking, routing, and censorship resistance are
Runtime properties. Core proves bid validity, deterministic selection,
minimum output, atomic settlement, and recovery.

## D09 — tokenized external-asset subscription

M4+: typically `PF-CAP` or `PF-AM` plus `IN-SHARE`/`IN-DEBT`; “RWA” becomes
reference-asset, custody, legal, identity, and jurisdiction facets.

```moriarty
capability Eligibility permits { attestEligible };
capability Custodian<A> permits { attestSubscription, attestRedemption };

contract TokenizedAssetSubscription<C,S,const N>(
  investor: Party, issuer: Party, cash: Amount<C>, shares: Amount<S>,
  eligibility: Eligibility, custodian: Custodian<C>, windows: Vector<N, Window>
) effects { receive, send, attest(eligibility), external(custodian) } {
  require attest eligibility(investor) fresh eligibilityMaxAge;
  await investor deposits cash until windows[0].close else close;
  await custodian attests subscriptionAccepted(cash, nonce)
    until windows[0].settle
    -> deliver investor shares
    else refund investor cash;
  for static window in windows.tail {
    await investor requests redemption(shares) until window.close
      -> settle only after custodian.attestRedemption(nonce)
      else continue;
  }
  close at windows.last.settle;
}
```

The circuit can keep eligibility facts private while proving policy
satisfaction. Custody, ownership law, reserve existence, and redemption
enforcement remain external and visible in the manifest.

## D10 — collateralized European option

M4+: `PF-EXCH` or `PF-AM + IN-DERIV:option`; exercise/settlement mechanism is
separate.

```moriarty
contract EuropeanCoveredCall<U,Q>(
  writer: Party, holder: Party, underlying: Amount<U>, premium: Amount<Q>,
  strike: Ratio<Q,U>, expiry: Timestamp, price: Oracle<Ratio<Q,U>>
) effects { receive, send, attest(price) } {
  atomic all { writer deposits underlying; holder deposits premium; }
    until openDeadline else refund all;
  pay writer premium;
  await ordered any {
    holder exercises with strike * underlying
      -> pay holder underlying; pay writer strike * underlying;
    timeout(expiry)
      -> pay writer underlying;
  } until expiry else pay writer underlying;
  close;
}
```

A cash-settled variant consumes a terminal-fixing attestation. Option books,
volatility surfaces, dynamic hedging, and vault strategy are outside this Core
pattern.

## D11 — reserve-attested monetary claim

M4+: `PF-MONEY + IN-MONEY:reserve_backed + ME-ISSUE` plus custody, legal,
reserve, freeze, and upgrade facets.

```moriarty
capability IssuerPolicy<S> permits { mint<S>, burn<S>, freeze };
capability Reserve<C> permits { attestReserve };

contract ReserveAttestedStablecoin<C,S,const N>(
  customer: Party, issuer: Party, cashClaim: Amount<C>,
  policy: IssuerPolicy<S>, reserve: Reserve<C>, reports: Vector<N, Timestamp>
) effects { receive, send, mint(policy), burn(policy), attest(reserve) } {
  for static t in reports {
    let proof = attest reserve at t fresh maxAge;
    invariant circulatingSupply <= proof.eligibleReserve;
    await ordered any {
      issuer confirms cashReceived(customer, cashClaim)
        -> mint customer convert(cashClaim);
      customer deposits<S> for redemption
        -> burn deposited<S>; recordCashObligation(customer);
      policy freezes credential with statedReason;
    } until t + reportWindow else suspendNewIssuance;
  }
  closeIssuanceAt(reports.last);
}
```

The contract can enforce supply rules relative to signed reports. It cannot
prove bank balances, legal redemption rights, sanctions correctness, or issuer
solvency between reports.

## D12a — event-contingent market

M4+: `PF-EXCH + IN-DERIV:event_contingent + ME-COND` plus rulebook, oracle, and
resolution facets.

```moriarty
contract EventContingentMarket<Q,const O>(
  traders: BoundedParticipants<P>, collateral: Amount<Q>, outcomes: Outcome<O>,
  resolver: Oracle<Resolution<O>>, close: Timestamp, resolveBy: Timestamp
) effects { receive, send, attest(resolver) } {
  await traders may split collateral into completeSet(outcomes) until close;
  await traders may atomically exchange outcomeClaims until close;
  let result = attest resolver once nonce eventId
                after close before resolveBy;
  require result.outcome in outcomes;
  redeemWinningClaimsProRata(result.outcome);
  refundByFallbackRule if no valid resolution by resolveBy;
  close;
}
```

Market surveillance, rulebook interpretation, and real-world fact resolution
remain outside Core. The contract proves complete-set conservation, no replay,
one resolution, and bounded payout.

## D12b — delegated curator vault

M4+: `PF-AM + IN-SHARE + ME-ALLOC` with delegated-curator control. This is a
separate product, not a prediction-market subtype.

```moriarty
contract DelegatedCuratorVault<A,const K,const N>(
  curator: Party, allocator: Party, depositors: Vector<N, Party>,
  markets: Vector<K, Capability>, caps: Vector<K, Amount<A>>, end: Timestamp
) effects { receive, send, external(markets) } {
  invariant every exposure[i] <= caps[i];
  invariant curator cannot transfer depositor assets to curator;
  await depositors deposit<A> and receive shares until fundingClose;
  await curator sets caps and allocator selects within caps until end;
  allow depositors bounded redemption windows with proRataNAV;
  revoke allocator on explicit curator action or invariant failure;
  unwind and redeemAll at end;
  close;
}
```

This pattern makes authority, caps, revocation, and accounting explicit. It
does not prove the curator's judgment or external market solvency.

## Acceptance work before these become language examples

For each pattern, the next implementation phase must provide a parsed and
formatted source file, elaborated Core, resource certificate, generated
Compact, ZKIR model/proof checks, positive and negative traces, conservation
and authorization properties, timeout and unavailable-attestation traces,
malicious Runtime transaction rejection, and a row-level residue report against
the corresponding DeFiFormal functional obligations. “Mapped” does not mean
“covered”; the present DeFiFormal construction suite itself has zero complete
verdicts at the pinned commit.
