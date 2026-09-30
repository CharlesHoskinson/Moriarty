# Independent review of `sol-defi.k` (MIL/4 DeFi profile, specified only)

**Scope:** I reviewed the text only. I did not run `kompile`, prove anything or check it against the real source, Core or native settlement. SP01.6 is still blocked. Nothing below implies adoption.

## Compile findings vs. semantic gaps

**Compile:** Reading it, I found no definite compile defect. The arities of all 10 request constructors, 3 evidence constructors and `m4dSeal` match their rule patterns. Operator precedence is either explicit or correct. `minInt`/`maxInt` come from `INT`. `[owise]` is used per function. There are four lint issues:
- `m4dFamilyCheck(m4dLater(_))` and `m4dSealOf(m4dLater(_))` can never run, because `m4dEval` intercepts `m4dLater` first.
- `m4dPending` is declared but never produced.
- The function rules for `m4dEval` overlap and are separated only by `requires`. The `[total]` attribute is missing.
- The `symbol(...)` attribute form needs a recent K version.

Until `kompile` passes, "compiles" must not be claimed.

**Semantics:** The gaps below are logical, not syntactic. Most come from one pattern: financial quantities are unauthenticated Ints that sit beside an evidence string, and nothing ties them to it.

## Semantic defects

**D1. `m4dBound`: the seal checks itself (cross-cutting).**
- **Rule:** It requires only `NH ==String DH` and `EF ==String CE`, and both values are supplied by the caller. Nothing links `EF` to the request's quantities.
- **Counterexample:** Take one seal `m4dSeal("p","b","p","b","h","h","x","x","e","e",1)`. `m4dBridgeReceive` accepts with `CREDIT=10` and again with `CREDIT=50`. Both produce the same `m4dAccept("bridge-pair/1","x","e",…)`.
- **Repair:** Add `m4dFx(M4DReq) → String`, the canonical digest of the family transition tuple. Require `EF ==String m4dFx(R)` and `NH ==String m4dStep(AH, m4dFx(R))` inside `m4dEval`. Also add the effect-bearing quantities to `m4dAccept`.

**D2. `m4dAMM`: reserves and fee aren't tied to the pool evidence.**
- **Rule:** `XP`, `YP`, `F` and `N` are free Ints. The evidence `HEAD` is only matched against the request's `HEAD`, and neither is compared with the seal's `AH`.
- **Counterexample:** The true reserves are (10⁶, 10⁶). The caller submits `XP=1, YP=10⁶, F=0, N=1, DX=1`. That gives `DY = floor(10⁶·1/2) = 500000`, and the request is accepted.
- **Repair:** Change the pattern to `m4dLocal("pool", DOM, PID, EPOCH, m4dPoolDigest(PID, XP, YP, F, N))` and add `HEAD ==String m4dSealAH(S)`.
- **Second issue:** `MIN` may be ≤ 0, so the pool can take `DX` and pay `DY=0` (for example `XP=10⁶, YP=1, DX=1`). Add `MIN >=Int 1`.

**D3. `m4dRepay`: there is no loan evidence at all.**
- **Rule:** The constructor has no `M4DEvidence`. `P` and `I` are claims made by the caller.
- **Counterexample:** The real loan has `P=1000, I=50`. The caller submits `P=10, I=0, N=10`. That yields `NP=0, NI=0`, and the request is accepted as a full discharge under duty `OID`.
- **Repair:** Add an evidence argument that must match `m4dLocal("loan", DOM, OID, EPOCH, m4dLoanDigest(DEBTOR, CREDITOR, P, I))`, and require the post head to commit to `(NP, NI)`.

**D4. `m4dCDP`: price is unbound, freshness is missing, the ratio is set by the caller, and the duty is per pair.**
- **Rule:** `PX` isn't derived from `OBS`, and `ROUND >=Int 0` is the only freshness check. `NUM` and `DEN` are free, so `NUM < DEN` is allowed. The duty `"debt:"+PAIR` merges every position on that pair. Health checks only the new `LOCK` against the new `MINT` and ignores the existing position.
- **Counterexamples:**
  - `NUM=1, DEN=100` accepts a mint backed by 1% collateral.
  - Two positions A and B on ETH/USD share one duty identity.
  - An old round with an inflated `PX` is accepted.
- **Repairs:**
  - Require `OBS ==String m4dPxDigest(PAIR, ROUND, PX)`.
  - Reuse the `obs/1` round-selection and freshness predicate.
  - Bind `NUM` and `DEN` to policy evidence and require `NUM >=Int DEN`.
  - Add position fields `POS, C0, D0`, require `(C0+LOCK)·PX·DEN ≥ (D0+MINT)·NUM`, and use duty `"debt:"+PAIR+":"+POS`.
  - Add `m4dProduct(LOCK *Int PX, DEN)`.

**D5. `m4dOption`: the one-shot claim, strike and fixing are self-attested.**
- **Rule:**
  - `CLAIM ==Int 1` is a caller Int. Nothing checks that the claim is unconsumed in the pre-state.
  - `STRIKE` has no lower bound.
  - `FIX` isn't tied to `OBS`.
  - There is no expiry or fixing-time check.
- **Counterexamples:**
  - Two exercises under seals with different `AH` both accept, which double-drains `RESERVE`.
  - `STRIKE=-100, FIX=0` pays out when the option is worthless.
- **Repairs:**
  - Require `OBS ==String m4dFixDigest(IID, ROUND, FIX)`.
  - Add `STRIKE >=Int 0`.
  - Add claim-state evidence `m4dLocal("claim", DOM, IID, EPOCH, "live")`.
  - Add `ROUND ==Int` a maturity round bound in the instrument digest.

**D6. `m4dObserve`: "selected round" and "now" come from the caller.**
- **Rule:** `SELECTED`, `NOW`, `OBSERVED` and `MAXAGE` are all free Ints.
- **Counterexample:** The stale round 5 is submitted with `SELECTED=5, NOW=OBSERVED=0, MAXAGE=0`, and it is accepted. The draft's own comment says this must not happen.
- **Repair:**
  - Take `SELECTED` and `NOW` from authenticated seal or state fields.
  - Parse `OBSERVED` from `PAYLOAD`: `PAYLOAD ==String m4dObsDigest(PAIR, ROUND, OBSERVED, value)`.
  - Bind `MAXAGE` to policy.

**D7. `m4dGovern`: the threshold is chosen by the caller and duties can be dropped.**
- **Rule:**
  - The evidence payload is the action digest, not an approval set.
  - `DISTINCT`, `THRESH`, `LIVE` and `PAUSED` are caller Ints.
  - `DUTIES >=Int 0` doesn't enforce the claim that duties are preserved.
- **Counterexample:** The policy requires 3-of-5. The caller submits `THRESH=1, DISTINCT=1`, and it is accepted. A queued action that erases outstanding bridge duties is also accepted.
- **Repairs:**
  - Get `THRESH` and `LIVE` from policy evidence.
  - Compute `DISTINCT` with a dedup function over an explicit approver list that is checked against the grant set.
  - Require `postDuties ==String preDuties`, unless the action is in a whitelisted duty-transition class.

**D8. Bridge (`m4dBridgeLock`, `m4dBridgeReceive`, `m4dBridgeRefund`): no conservation and no mutual exclusion.**
- **Rule:**
  - Lock replay protection is `USED ==Int 0`, a caller Int.
  - Receive and refund never mention the original `LOCK` or `FEEBOUND`.
  - `VER =/=String ""` accepts any verifier.
  - `COUNT ==Int 0` contradicts the comment that installments are allowed.
  - The three steps emit different duty ids (`pending:`, `remaining:`, `terminal:`), so nothing links them.
- **Counterexamples:**
  - `LOCK=100` on the source, then `m4dBridgeReceive` with `REMAIN=1000, CREDIT=900` is accepted.
  - A refund is accepted, and a later receive on the same `CLAIM` is also accepted, so the funds are paid twice.
- **Repairs:**
  - Use a single duty id, `"bridge:"+CLAIM`, with a phase field (`Pending`, `Partial`, `Terminal`).
  - Every step carries `LOCK, FEEBOUND, DELIVERED, FEESPENT, REMAIN` from lock-record evidence, with the invariant `DELIVERED + FEESPENT + REMAIN == LOCK` and `FEESPENT ≤ FEEBOUND`.
  - Receive requires phase ≠ `Terminal`, and the post-state sets `REMAIN' = REMAIN − CREDIT − FEE`.
  - Refund sets the phase to `Terminal`.
  - Require `VER ==String` the authenticated verifier-set id for `(DOM, EPOCH)`.
  - Lock requires absence evidence for `CLAIM`.

**D9. `m4dVault`: there is no vault-state evidence, and `CUSTODY` checks nothing.**
- **Rule:** `MANAGED`, `SUPPLY`, `VA` and `VS` come from the caller. `CUSTODY == MANAGED + SURPLUS` with a free `SURPLUS ≥ 0` is always satisfiable.
- **Counterexample:** The real state is `MANAGED=10⁶, SUPPLY=10⁶`. The caller submits `MANAGED=0, SUPPLY=10⁶, VA=VS=1, DEPOSIT=1`, which mints about 10⁶ shares.
- **Repair:**
  - Add evidence `m4dLocal("vault", DOM, POOL, EPOCH, m4dVaultDigest(ASSET, CLASS, MANAGED, SUPPLY, CUSTODY, VA, VS))`.
  - Bind `VA` and `VS` to immutable policy.
  - Add `m4dU(CUSTODY)` and `m4dProduct(SHARES, MANAGED +Int VA)`.

## What this module can validly claim

- It is a specified-only, unkompiled K draft of acceptance predicates for 9 DeFi families.
- These arithmetic facts hold, and only if the supplied Ints are true:
  - The AMM quote is exact floor division.
  - Loan repayment discharges interest first.
  - The option payoff is floored and its reserve is ceiling-rounded.
  - Vault share issuance is floored in the vault's favour.
  - The listed quantities are range-checked against u128.
- `m4dLater` is rejected, and bridge refund without typed nonreceipt evidence fails closed with `UNKNOWN_NONRECEIPT`.
- `m4dAccept` is a prepared relation, not a commit.

## What must remain explicitly rejected

These claims must stay rejected:
- **Authentication or soundness of any seal, head, or effect digest.** D1 shows the seal only checks that the caller supplied matching strings.
- **Binding of any quantity to authenticated state.** D2–D7 and D9 show every family trusts caller Ints.
- **Bridge conservation or exclusion of double payment** (D8).
- **Oracle freshness or round selection** (D4, D6).
- **Governance quorum or duty preservation** (D7).
- **One-shot option exercise** (D5).
- **Undercollateralisation safety** (D4).
- **Correspondence with source, Core, or native execution.**
- **Proof, adoption, or native settlement.**
- **Compiled status**, until `kompile` is run and its output recorded.