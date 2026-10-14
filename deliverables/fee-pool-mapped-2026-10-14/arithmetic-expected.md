# Independent FeePool arithmetic and semantic expectations

Status: source facts plus independent integer oracle; implementation and native execution **not performed**. Reference is Project Fenrir commit `ec8da3be532f29e5e92ff7b1804b7dff0bc89b15`; reviewed Moriarty worktree HEAD `dc31345850b5800687b16b26bfa20f1d66d94211`. Startup skill/AGENTS/status read; existing admission blockers do not bar this read-only review. No source/build/model/shared-session change. Companion JSON contains decimal exact vectors, source digests, and 50 specified cases.

## Source facts and exact domain

Pinned `research/fenrir-v2/fenrir-contracts/src/fee_pool.rs` lines 20–26, 105–160, 203–215, 324–350, 592–745 use **U64 monetary ledger values, aggregate weights, quotients, counters and times; U128 products**. Requiring U64 inputs is the complete reference domain, not a toy cap. Quint `research/fenrir-v2/quint-spec/fee_pool.qnt` lines 17, 37–38 uses conservative `MAXU=2^63-1` because of its evaluator; it is not a full native U64 boundary oracle. Signal-step-down is explicitly excluded from Quint because it does not move funds (line18).

All metering is normalized into one configured service-fee asset. TIA/DUST are costs converted into that asset, not separate FeePool income assets. No capacity-exchange/burn intake. Imported header/cost/roster truth remains authenticated importer responsibility; native foreign proof verifier and withdrawal adapter remain absent in reference header.

## Existing Core feasibility

Actual reviewed paths: `crates/mori-core/src/ir.rs`, `parser.rs`, `runtime.rs`; native producer references `crates/mori-ledger/src/relation/{arithmetic,compiler}.rs`. Core `UInt(pub u128)` checks every +,-,* intermediate. No Div, modulo, functions, arrays/maps, conditional expression, dynamic parties, or cryptographic commitment primitive. Grammar supports `requires`, `let`, `next`, `ensures`, fixed-string `transfer/fee/liability/discharge`, and Bool expressions. Program has 8192 expressions, 1024 statements/action, 256 states/assets/actions and at most256 inputs/action. Boolean evaluation is eager: a guarded disjunction does not protect an overflowing/underflowing arithmetic child.

For each exact-reference quotient, untrusted q and r inputs are safe with sequential guards **before any multiplication**:

```mori
requires n <= 18446744073709551615;
requires w <= 18446744073709551615;
requires d <= 18446744073709551615;
requires q <= 18446744073709551615;
requires r <= 18446744073709551615;
requires d > 0;
requires r < d;
requires q * d + r == n * w;
```

`n,w,d` must bind to actual stored surplus/share/effective seat/original total weight, never caller replacements. For percentages w is a source literal55,10,5,15 and d100. Bound all referenced state before arithmetic as well as inputs; guard U64 sums for ledger writes even though Core permits U128. This equation accepts nondivisible inputs and uniquely enforces floor. Let B=2^64: `(B-1)^2+(B-1)=B²-B < 2^128`, so even malicious in-range q,d,r cannot overflow this expression. Reference valid numerator max is `(B-1)^2`. Neither exact-divisible-only restriction nor host success flag is needed. Existing native checked multiplication decomposes64-bit factors and enforces no overflow; no G1 arithmetic file change is indicated. This is source-level feasibility, not a native cost or proof acceptance result.

Zero aggregate reward weight must constrain d1; positive aggregate constrains d to the immutable original aggregate. Use `(weight == 0 and d == 1) or (weight > 0 and d == weight)` after range guards. Valid zero aggregate implies all effective seats0 and q0 r0. Never divide by remaining weight or alter total weights during allocation.

Backstop witness equations can avoid dangerous eager subtraction: constrain a U64 gap via `(before < target and before + gap == target) or (before >= target and gap == 0)`, then require `(amount <= gap and refill == amount) or (amount > gap and refill == gap)` and `remainder+refill==amount`; bind nextbackstop=before+refill and payout=remainder. All additions here remain below2^65 under U64 caps. Never evaluate `target-before` when before may exceed target.

## Optional generic full UInt128 extension (not needed for reference)

Direct `q*d+r==n*w` rejects legitimate U128 operands when their products exceed U128. A complete extension is feasible in existing source grammar through **explicit 64-bit limbs and carries**, with no unchecked field equality. This extension is specified only and should not enlarge the reference library's financial domain silently.

For each x in n,w,d,q,r, witness x0,x1, require each <=B-1 first, then require `x == x1*B+x0`. Every recomposition is <=U128max. For each 2x2 multiplication a*b, four partial products `pij=ai*bj` fit128. Witness lows lij/highs hij, both<B, with `pij==hij*B+lij`. Product output limbs: y0=l00; constrain `h00+l01+l10 == y1+c1*B` with y1<B and c1<=2; constrain `h01+h10+l11+c1 == y2+c2*B` with y2<B and c2<=2; constrain `h11+c2==y3` and y3<B. These small column sums are <3B+2, never129-bit sums of full partial products. Apply twice to n*w and q*d.

Then add r to q*d in baseB: `qd0+r0 == z0+e0*B`; `qd1+r1+e0 == z1+e1*B`; `qd2+e1 == z2+e2*B`; `qd3+e2==z3`; each zi<B, ei<=1, equality zi==nw_i. No fifth carry is allowed. Require d>0 and r<d. Constrain **all** limb/high/carry ranges independently before equations; omit no top limb. Malicious high limbs/carries then cannot hide a multiple of2^256 or a native field modulus. Canonical equations imply integer equality, not modular equivalence. If mathematical quotient exceedsU128, no representable q exists and reject appropriately. This recipe uses fewer than100 extra witness scalars and150 statements for one generic quotient; actual parser expression budgets across the whole expanded agreement must still be measured. It requires no new primitive in principle, but exact-reference U64 equations are smaller and easier to audit.

## Required lifecycle and identities

1. Initialize once; nonzero authority, asset and treasury commitments; federated mode only. Zero initial backstop/watcher pool. Runtime authenticated authority is not an untrusted Bool success flag.
2. Record closed epoch once: globally unique nonzero canonical header; `(epoch+1)*86400` must fit U64; gross U64. Capacity proceeds/wrong service asset cannot enter.
3. Register unique epoch/role/seat before settle. Roles0 OP,1 DA,2 watcher. OP supplied weight<=86400 even if unattested; effective=attested?weight:0. U64 aggregate adds; watcher count<=limit. Only OP/DA increment reward seat count; repeated recipient seats allowed but not duplicate stipend entitlement.
4. Meter fresh global receipt before settle; positive U64 amount, kinds0..6, cumulative<=gross; immutable recipient/epoch/receipt/kind/amount. AUDIT5 and INSURANCE6 require L2 treasury. Stipend3 requires registered positive effective watcher, recipient equality, <=stipend limit, once per epoch+recipient. Create reimbursement entitlement, unlock epoch close, but claim still requires settlement.
5. Settle once: reimburse full metered amount first; s=gross-metered; OP=floor55s/100, DA=floor10s/100, heldWatcher=floor5s/100, L2=floor15s/100; Midnight=s-OP-DA-watcher-L2. Midnight lies floor15s/100 through +4, never independently floored. Add watcher credit with U64 overflow guard. Initial L2 fills min(L2,max(0,target-backstop)); remainder is L2 treasury entitlement. OP/DA remaining initialized; watcher remaining0. Create fresh treasury keys0/1 and immutable settled membership.
6. Permissionless allocate each OP/DA seat once while settled/notfinished: q=floor(original category total*effective seat weight / max(1,original category total weight)). Immutable registered recipient, original denominators, category remaining subtraction, incrementallocated<=seats. Unlock `(epoch+1)*86400+2592000` checked U64. Zero amount still allocated but unclaimable. Watcher allocation rejects.
7. Permissionless finish once only allocated==rewardseats. Sum OP/DA remaining (watcherremaining0), refill current backstop target then L2 treasury key2 residual. Set all remaining0 and finished. Empty categories/no seats sweep. This does not revoke pending earlier entitlements.
8. Authenticated watcher reserve spend after epoch settlement: fresh globally shared meter receipt, recipient nonzero, kind7 bounty or8 defender inclusion only, positiveamount<=held; debit and create epoch-close-unlocked entitlement. Not a stipend route.
9. Claim existing positive entitlement, settled original epoch, bound recipient secret/identity, now>=unlock, unclaimed; consume once. No claim expiry and no finished-epoch rejection. Reference returns recipient+amount but does not transfer; source library adapter must atomically transfer configured asset from pool to immutable beneficiary and record claimed. Failed debit/credit/identity/time checks must leave all state unchanged.
10. Authenticated set_backstop_target: U64; a lowered target does not claw back existing funds. Signal-step-down is separate nonfunds transition: unique window_end/report, window_end>=2592000 and closed, cost>0, 2revenue>=3cost using U128, return_bps>4000; records divisor2. An M11 funds-only bounded profile can state its exclusion explicitly.

Hash identity in reference includes fixed namespace tag, pool self address, epoch, and role/seat or receipt; stipend includes epoch+recipient. For Core fixed literal accounts, materialize a finite source profile with fixed configured beneficiaries and unique slots. Caller-provided numeric recipient labels do not authenticate secrets or control dynamic transfer destinations. Preserve full asset tuple network/issuer/reference/scale, debtor=pool and creditor=recipient for entitlements, and frompool->recipient direction for claim transfers. Conservation of **entitlements** is not an already executed payout. Keep source accounting and settlement observations distinct.

## Independent concrete oracles and blockers

JSON supplies exact splits at0,1,7,99,101,U64max and separately labeled U128max extension. It also supplies full weighted-product boundary and nondivisible q/r values, complete lifecycle111/1100 vectors, zero/noactivity/duplicate/wrongstream/identity/overflow/lateclaim cases. These are expected results, not executed test results.

Core lacks unbounded maps/sets, B32 cryptographic imports, dynamic parties and trusted block-time reads. A bounded source library needs explicitly fixed epoch/seat/receipt capacity, immutable static parties and asset identity, plus stated authenticated observation/clock/import adapters. They are exact blocked data if arbitrary-reference coverage is claimed: canonical header/finality binding, normalized cost provenance and asset conversion, attested seat roster and effective weights, authoritative time, recipient secret commitment proof, settled immutable membership and global receipt uniqueness across all slots, atomic withdrawal linkage, and proof/native resource measurements. Caller Bool `auth/host_ok` cannot supply these obligations. No new Div is blocked for the exact arithmetic. Unlimited reference-map coverage is not expressible directly by present Core; finite scope must be stated and checked, not described as full unbounded FeePool equivalence.
