I've finished the read-only work. Guarded status reports capability `SP01.6 loan-swap-subset` as blocked, lists `sp01-loan-report` as the next action and shows no pending Midnight transactions. This review is read-only and doesn't dispatch anything.

Plan mode is active and this session has no file-write tool, so I couldn't save the review to the repository or to a plan file. It is below in full. Everything here is a specified-only proposal. It isn't adopted MIL/4, and I compiled nothing, executed nothing and proved nothing.

---

# Episode/stage transition semantics — an independent review with K text

**Focus:** stage and episode transitions, complete effects, conservation, cumulative authority, pending/partial/unknown, recovery, authenticated history.
**Sources read:** `concepts/intent-language/DESIGN-MIL2.md`, `deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md` and `docs/MORIARTY-CONSOLIDATED-DESIGN.md`. I also read MIL/3 §2–§7, because MIL/4 inherits its six judgments from there. The K conventions come from `experiments/moriarty-language/formal/k/` (K 7.1.337, LLVM backend, per `toolchain.lock.json`).

## 1. Summary

1. MIL/2's episode model is ledger induction over link fields (§2, line 46), and that choice is sound. But nobody has defined how the link fields change from one stage to the next. `cumulativeGrossDebit`, `obligationRollForward` and `terminalTombstone` are named without contribution rules, so the design has no answer to "a refund resets the cap".
2. MIL/2 §7's escrow has three phases (`unfunded → pending → released|refunded`), which isn't enough. The consolidated design (line 151) and ROADMAP U3 also require the workflow to distinguish *partial*, *unknown/unresolved* and *recovering*. MIL/2 has no partial rule and no unknown state.
3. The §13 showcase `AcquireB` fails under four separate counterexamples (G2–G5 below): a gross/fee cap that allows no fee, a free option for the owner, an incomplete footprint, and budgets missing from its grants.
4. Conservation (E1) is necessary but not sufficient. Redirecting a recipient and making hidden balance writes both conserve. Acceptance needs **claimed writes to equal the effect-derived writes exactly**, and **submitted effects to equal the prepared effects exactly**. MIL/3 §4 and MIL/4 M4-C4 already point this way. The K below makes these separate rules with separate codes.
5. The episode length cap (MIL/2 §12: 8 stages) with no reserved closure slots lets ordinary progress strand funds. I propose a stage-slot **closure reserve**, the stage-count analogue of the existing K lifecycle `closureReserve`.
6. For foreign delivery, MIL/2 §7's authoring check `after(deadline) ⇒ release ∨ refund` can only be met with a time-only refund guard. MIL/3 §7 shows that guard is unsafe. A foreign escrow that claims recovery therefore needs a named verifier-liveness assumption.
7. Section 4 has the K module. It is written to kompile, but I haven't compiled it. Every action constructor either gets admission rules or an explicit rejection code. Unknown constructors reject by default. No acceptance path depends on an uninterpreted predicate. Hashes are free constructors, which makes them injective by construction; native collision resistance is a separate obligation.

## 2. Findings, gaps and counterexamples

Each finding is labeled: **[source fact]**, **[repository observation]**, **[inference]**, **[counterexample]** or **[recommendation]**.

**G1. The phase set is incomplete.** [source fact] MIL/2:203 lists `unfunded → pending → (released | refunded)`. Consolidated:151 requires partial, waiting, in-flight, unresolved and recovering states. ROADMAP:37 requires "permitted partial outcomes" and says "a committed partial stage retains its obligation". [recommendation] Keep the phase set, add counters (`delivered`, `paidB`, `releasedA`) and a foreign status `dNone | dUnknown | dObserved(q) | dClosed(q)`. Partial progress becomes counter state inside `pending`, not another phase.

**G2. Under the roadmap reading, the showcase can pay no fee.** [source fact] MIL/2:299 says `gross_debit <= 11 A; fees <= 1 A`, and MIL/2:308 funds `11 A`. ROADMAP:37 says "at most 11 A **including** at most 1 A fees", and consolidated:145 counts fees inside gross. [counterexample] If the fund is 11 A, any fee above zero makes gross 12 > 11. If fees sit outside gross, the showcase contradicts the roadmap. [recommendation] Gross includes fees. A solver-paid scenario then needs `fund ≤ 10`. This needs an explicit owner decision under M4-C5.

**G3. Direct delivery gives the owner a free option.** [source fact] MIL/2:309 has `release_when delivered(B, >= 20 B, to owner)`, so B goes straight to the owner. [counterexample] The counterparty delivers 19 B, the deadline passes, and `refund_when after(validity.end)` returns 11 A to the owner. The owner ends with 19 B and 11 A. [recommendation] For local delivery, B goes into episode custody. Release pays both sides and refund returns both sides (a delivery-versus-payment escrow). Foreign delivery can't be held in custody, so a foreign escrow must be `proRata` with `prioRelease` (code `FOREIGN_POLICY`).

**G4. The showcase footprint is incomplete, and the §9 cell vocabulary lacks an actor-relative cell.** [source fact] MIL/2:316–317 omits the counterparty A credit, the escrow custody balances and the fee recipient. MIL/3 §4 records the first omission. [inference] `complete by any solver` with a fee needs a fee-recipient cell that is known only at stage time. MIL/3 §2 says "a dynamic cell selected by a hole has a statically bounded resolution or rejects". [recommendation] Add `actorBalC(d, asset)`, which resolves to the stage actor's balance. That is a bounded resolution: exactly one cell per stage.

**G5. Grants lack budgets, and refund authority is unnamed.** [source fact] MIL/2 §6 (line 191) gives every right a budget. The showcase at line 320 gives `initiate` and `recover` none, and it names nobody who may trigger `refund`. [recommendation] A grant without a budget fails admission. Refund needs `complete` or `recover`. The budget charge is `fund` for initiate and the release amount for complete. Refund charges nothing.

**G6. `priority signed_order` has no meaning.** [source fact] MIL/2:206 lists it. It is never defined. [recommendation] This profile rejects it with `PRIORITY_UNSUPPORTED`.

**G7. Priority and partial settlement can deadlock.** [counterexample, found while designing] Suppose partial releases must settle before a refund (to keep Φ₀, meaning no division), and `prioRefund` blocks every release while the refund guard holds. Then a `proRata` + `prioRefund` escrow with unsettled pro-rata value can neither refund nor settle. [recommendation] Priority applies only to the *final* release against the refund. Partial settlement is always allowed.

**G8. Pro-rata rounding goes to a beneficiary the design hasn't chosen.** [counterexample] With fund 10 A, target 20 B and 7 B delivered, the counterparty's entitlement is floor(70/20) = 3 A and the owner gets 7 A back. The fourth whole A goes to the owner. [source fact] MIL/4 M4-C2 (line 18) names the protocol reserve as beneficiary for rounding, and a fractional remainder isn't a ledger quantity. [open question] Whether this whole-unit allocation counts as "rounding" under M4-C2 is not decided. The K below uses floor to the counterparty and marks it as a W-D4 decision.

**G9. An 8-stage cap without reserved slots strands funds.** [source fact] MIL/2:281 caps episodes at 8 stages. Consolidated:159 says "Reserve work cannot be consumed by ordinary progress". [counterexample] Under a cap of 4, fund + deliver + deliver + deliver leaves no slot for a refund, so the funds are stuck. With foreign status, repeated `dUnknown` observations can also use up the slots (griefing). [recommendation] Reserve `need(I) = 1 + [foreign] + [proRata]` slots. Only closing actions may use them: refund, recover, `observe(dClosed)`, and a release that settles exactly. Unknown→unknown observations are rejected as `NO_PROGRESS`.

**G10. A time-only authoring check is unsafe for foreign delivery.** [source fact] MIL/2:218 has the check. MIL/3 §7 (lines 125–129) and consolidated:153 say a timeout "does not prove another chain did not execute". [recommendation] A foreign refund requires `dClosed(q)`. `dUnknown` blocks refund and recovery whatever the time. Claiming recovery then requires `verifierLiveness` in the named assumptions, or `RECOVERY_UNGROUNDED`.

**G11. A signed intent can open more than one episode.** [source fact] MIL/2 §9 has a `replay(id)` cell but no rule tying the signed intent to a single episode. [counterexample] One signature opens two episodes, each funding 11 A, so the owner's exposure is 22 A. That breaks consolidated:145 ("reserve spent plus pending exposure against the same authenticated budget"). [recommendation] Put the intent itself into the replay set at `open` (`INTENT_REPLAY`).

**G12. Custody can alias across episodes.** [counterexample] A custody account keyed by the escrow name `E` is shared by two episodes that both use `E`, which mixes their funds and breaks the accounting. [recommendation] Key custody by episode id, and reject a custody endpoint from another episode (`CUSTODY_FOREIGN_EPISODE`). [open obligation] At the native level, only the episode's own contract transitions may move custody. Otherwise a donation makes the equality invariant fail and funds stick.

**G13. Conservation is weaker than exact effects.** [counterexample] (a) Redirect the release from the counterparty to the solver. It conserves, and its writes match its own effects. (b) Correct effects, but the claimed writes move 1 more A from owner to solver. That conserves too. Both must be rejected: (a) by `EFFECT_MISMATCH` and (b) by `WRITES_NOT_EXACT`. A third case, crediting the solver with no debit, is caught by `CONSERVATION`. Each hostile case reaches its own rule, as MIL/4:53 requires.

**G14. Rejection precedence must follow MIL/4:34.** [source fact] MIL/4:34: "Changing only … an unpaid duty in an otherwise feasible stage must select … `failure_p` … rather than pass through `effect_p` alone." [recommendation] So `DUTY_DROPPED` (a custody-accounting check on the *submitted* effects) runs **before** `EFFECT_MISMATCH`. A refund that returns 9 of 10 A then reports `DUTY_DROPPED`.

**G15. Revocation can strand recovery.** [source fact] Consolidated:159 says revocation cannot erase duties. [recommendation] Revoking a `recover` grant while a recovery-claiming episode is still open fails with `REVOKE_WOULD_STRAND`.

**G16. "Unknown" for a local stage.** [inference] A local stage that has been submitted but not yet observed doesn't change the K state. Every successor must name the current head, so a stage that lands late makes the competing successor `STALE_PREDECESSOR`, and nothing is applied twice. This is compare-and-swap on the head cell. It relies on the native layer actually consuming the head (the MIL/4 `history_p` obligation).

**G17. Successor episodes are undefined.** [source fact] Consolidated:172 requires successor episodes "without resetting an existing signed lifetime budget". [gap] MIL/2 and MIL/4 have no constructor for this. This profile has none either, so there is no way to continue past the cap. Section 5 gives pseudocode for one.

**G18. Existing K defaults.** [repository observation] `lifecycle-kernel.k:43` has `lcPrincipalPart(_,_,_,_) => 0 [owise]` and `:44` has `lcRound(_,D,_) => 0 requires D <=Int 0`. These are totalizing defaults, and I did not check here whether upstream admission excludes their inputs. The new module follows a stricter rule: a default may only appear where another conjunct that fails on the same input dominates it. `inProfile`, `grantsOk`, `statusStep` and `authorized` all default to `false`. `prepared` defaults to the unmatchable `unpreparable`.

**G19. K evaluates eagerly.** [inference] In the LLVM backend, the `ensure` arguments are all computed when a stage expands. Precedence therefore only selects the rejection code, so every check function must be total. The module avoids division, and the pro-rata checks use literal-coefficient cross-multiplication (Φ₀). [native obligation] `fund·delivered` can reach 2²⁵⁶, so at the native level it needs the MIL/2 §4.5 two-limb rule.

## 3. Semantic decisions this profile makes (profile `P-ESC1`)

The rest of this section is a proposal that needs the M4 review gate.

- **Gross debit:** the sum of asset-A transfer lines *from the owner to anyone else*, including fees. It accumulates across the episode, and a refund never lowers it. A line from the owner in any asset other than A is `UNBUDGETED_OUTFLOW`.
- **Fees:** only allowed on `releaseA` and `failA`. They go from the owner to the solver actor, count toward both gross and fees, and are in asset A. Because net is measured in B, "fees count against net" has no meaning across assets. For this cross-asset case, gross carries it.
- **Net:** checked only at the final release: `deliveredView ≥ netFloor`. A refund is the failure outcome and promises no net.
- **Custody invariant:** in `pending`, custody holds A = `fund − releasedA` and B = `delivered − paidB`. At a terminal phase both are 0.
- **Terminal:** `released` or `refunded`. After that, only `reconcileA`, which has no effects.
- **Arity:** a stage binds exactly one intent (MIL/2 §14 decision 4). Any other value is `ARITY_UNSUPPORTED`.
- **Not in this profile, explicit reject `ACTION_UNSUPPORTED_IN_PROFILE`:** `forkA`, `joinA`, `amendA`, and `failA` of anything other than a release.
- **Not admitted:** `mint`/`burn`, with `ISSUE_UNAUTHORIZED`, because no `rIssue` grant is admitted. `foreignCredit` gives `FOREIGN_EFFECT`.

## 4. K module

This is K source written to kompile against K 7.1.337 with the LLVM backend. I haven't compiled or run it, so expect some syntax repairs.

```k
// mil-episode-proposal.k — PROPOSAL, 2026-09-29.
// Status: K source written for review. Not kompiled, not executed, not audited,
// not adopted MIL/4. No proof, correspondence or Midnight result is claimed.
// Profile P-ESC1: one local-custody (or foreign-verified) conditional escrow per
// episode; stage arity 1; no mint/burn issue grants; fork/join/amend/successor
// rejected. Heads are free constructors (injective by construction); native
// collision resistance and public-input binding are separate obligations.

module MIL-EPISODE-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  imports BOOL-SYNTAX
  imports LIST
  imports MAP
  imports SET

  syntax Dom    ::= dom(String)                [symbol(dom)]
  syntax Asset  ::= asset(String)              [symbol(asset)]
  syntax Acct   ::= acct(String)               [symbol(acct)]
                  | custody(String)            [symbol(custody)]     // custody(epId): program-held
  syntax BalKey ::= bk(Dom, Acct, Asset)       [symbol(bk)]
  syntax DA     ::= da(Dom, Asset)             [symbol(da)]
  syntax Cell   ::= balC(Dom, Acct, Asset)     [symbol(balC)]
                  | actorBalC(Dom, Asset)      [symbol(actorBalC)]   // resolves to the stage actor
                  | escrowC(String)            [symbol(escrowC)]
                  | grantC(String)             [symbol(grantC)]
                  | headC(String)              [symbol(headC)]
                  | replayC(String)            [symbol(replayC)]

  syntax Tag    ::= "tFund" [symbol(tFund)] | "tFee" [symbol(tFee)]
                  | tDeliver(String) [symbol(tDeliver)]
                  | "tRelease" [symbol(tRelease)] | "tRefund" [symbol(tRefund)]
  syntax Effect ::= xfer(Dom, Asset, Acct, Acct, Int, Tag) [symbol(xfer)]
                  | mint(Dom, Asset, Acct, Int)            [symbol(mint)]
                  | burn(Dom, Asset, Acct, Int)            [symbol(burn)]
                  | foreignCredit(Dom, Asset, Acct, Int)   [symbol(foreignCredit)]

  syntax Right  ::= "rInitiate" | "rComplete" | "rReconcile" | "rRecover"
                  | "rDisclose" | "rAmend" | "rIssue" | "rEnforce"
  syntax Holder ::= only(Acct) | "anyone"
  syntax Expiry ::= until(Int) | "indefinite"
  syntax Grant  ::= grant(Right, Holder, String, Int, Expiry, Int, Bool) [symbol(grant)]
                    // right, holder, epId, validFrom, expiry, remaining budget (A units), revoked
                  | "noGrant"

  syntax Delivery   ::= local(Dom) | foreign(Dom, String)   // String names the verifier premise
  syntax Priority   ::= "prioRelease" | "prioRefund" | signedOrder(List)
  syntax Partial    ::= "noPartial" | "proRata"
  syntax FailPolicy ::= "noFailBranch" | retainFee(Int)
  syntax Assumption ::= "inclusion" | "actorArrival" | "witnessAvailability" | "verifierLiveness"

  syntax Budget   ::= budget(Asset, Int, Int, Asset, Int)            // A, grossCap, feeCap, B, netFloor
  syntax Escrow   ::= escrow(Acct, Int, Int, Delivery, Int, Priority, Partial)
                      // counterparty, fund(A), target(B), delivery, refundAfter, priority, partial
  syntax Recovery ::= recovery(Bool, Set, Int)                       // claims, assumptions, closure reserve
  syntax Intent   ::= intent(Acct, Dom, Int, Budget, Escrow, FailPolicy, Map, Set, Recovery, Int)
                      [symbol(intent)]
                      // owner, exec domain, validTo, budget, escrow, failure policy,
                      // authority (grantId |-> Grant), declared write footprint, recovery, stage cap

  syntax DStatus ::= "dNone" | "dUnknown" | dObserved(Int) | dClosed(Int)
  syntax Action  ::= "fundA" | deliverA(String, Int) | observeA(DStatus) | releaseA(Int)
                   | "refundA" | "recoverA" | failA(Action) | "reconcileA" | revokeA(String)
                   | forkA(List) | joinA(Head, Head) | amendA(Intent)
  syntax Head    ::= genesis(Intent)            [symbol(genesis)]
                   | ext(Head, Int, Action, List) [symbol(ext)]

  syntax Stage  ::= seed(Map)
                  | tick(Int)
                  | importEvidence(String, String, DStatus)   // epId, premise, verifier output
                  | open(String, Intent, Int)                 // epId, signed intent, arity
                  | stage(String, Head, Int, Int, Acct, String, Int, Action, List, Map)
                    // epId, predecessor, index, arity, actor, grantId, fee,
                    // action, submitted effect lines, claimed balance writes
  syntax Stages ::= List{Stage, ";"}
endmodule

module MIL-EPISODE
  imports MIL-EPISODE-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  imports LIST
  imports MAP
  imports SET
  imports K-EQUAL

  syntax EPhase   ::= "unfunded" | "pending" | "released" | "refunded"
  syntax EscRec   ::= esc(EPhase, Int, Int, Int, DStatus)
                      // phase, B delivered into custody, B paid to owner, A released, foreign status
  syntax EpRec    ::= ep(Intent, Head, Int, Int, Int, Int, EscRec, Bool)
                      // intent, head, next index, cum gross, cum fees, cum net B, escrow, terminal
  syntax Ctx      ::= ctx(String, Intent, EscRec, Acct, Int, Map, Set, Map, Bool)
                      // epId, intent, escrow, actor, now, imported, replay, grants, terminal
  syntax Applied  ::= okMap(Map) | "badApply"
  syntax Prepared ::= plist(List) | "unpreparable"
  syntax MaybeRight ::= Right | "selfAuth" | "noRight"
  syntax KItem    ::= rcpt(String, String)
  syntax Verdict  ::= accepted(String, Int) | rejected(String)
  syntax KItem    ::= ensure(Bool, String) | ensureCode(String) | failing(String) | "endStage"
                    | commitOpen(String, Intent)
                    | commitStage(String, Int, Action, List, String, EscRec, Applied)
                    | setNow(Int) | setBal(Map) | setImported(String, DStatus)

  configuration
    <k> $PGM:Stages </k>
    <now> 0 </now>
    <bal> .Map </bal>            // BalKey |-> Int  (authenticated ledger balances)
    <eps> .Map </eps>            // String |-> EpRec (authenticated episode heads)
    <grants> .Map </grants>      // String |-> Grant
    <imported> .Map </imported>  // String |-> DStatus (named verifier premise output)
    <replay> .Set </replay>      // Intent | rcpt(epId, receiptId)
    <out> .List </out>

  // ---------- sequencing: a rejected stage writes nothing ----------
  rule <k> S:Stage ; REST:Stages => S ~> endStage ~> REST ... </k>
  rule <k> .Stages => .K ... </k>
  rule <k> endStage => .K ... </k>
  rule <k> ensure(true, _) => .K ... </k>
  rule <k> ensure(false, C) => failing(C) ... </k>
  rule <k> ensureCode("") => .K ... </k>
  rule <k> ensureCode(C) => failing(C) ... </k> requires C =/=String ""
  rule <k> failing(C) ~> I:KItem => failing(C) ... </k> requires I =/=K endStage
  rule <k> failing(C) ~> endStage => .K ... </k>
       <out> ... .List => ListItem(rejected(C)) </out>

  // ---------- small total helpers ----------
  syntax Int ::= u128() [function, total] | b2i(Bool) [function, total]
               | getI(Map, KItem) [function, total]
  rule u128() => 340282366920938463463374607431768211455
  rule b2i(true) => 1
  rule b2i(false) => 0
  rule getI(M, K) => {M[K]}:>Int requires K in_keys(M) andBool isInt(M[K])
  rule getI(_, _) => 0 [owise]      // absent balance = 0; debits re-check sufficiency
  syntax String ::= chk(Bool, String) [function, total] | orc(String, String) [function, total]
  rule chk(true, _) => ""
  rule chk(false, C) => C
  rule orc("", C) => C
  rule orc(C, _) => C requires C =/=String ""
  syntax Bool ::= isPlainAcct(Acct) [function, total]
  rule isPlainAcct(acct(_)) => true
  rule isPlainAcct(custody(_)) => false

  // ---------- intent accessors ----------
  syntax Acct ::= owner(Intent) [function, total] | cpty(Intent) [function, total]
  syntax Dom  ::= exec(Intent) [function, total]
  syntax Asset ::= aA(Intent) [function, total] | aB(Intent) [function, total]
  syntax Int  ::= validTo(Intent) [function, total] | cap(Intent) [function, total]
                | grossCap(Intent) [function, total] | feeCap(Intent) [function, total]
                | netFloor(Intent) [function, total] | fundOf(Intent) [function, total]
                | target(Intent) [function, total] | refundAfter(Intent) [function, total]
                | reserve(Intent) [function, total] | need(Intent) [function, total]
  syntax Delivery ::= delivery(Intent) [function, total]
  syntax Priority ::= prio(Intent) [function, total]
  syntax Partial  ::= partial(Intent) [function, total]
  syntax FailPolicy ::= failPol(Intent) [function, total]
  syntax Map  ::= auth(Intent) [function, total]
  syntax Set  ::= footprint(Intent) [function, total] | assumptions(Intent) [function, total]
  syntax Bool ::= claims(Intent) [function, total] | isLocal(Intent) [function, total]
  rule owner(intent(O,_,_,_,_,_,_,_,_,_)) => O
  rule exec(intent(_,X,_,_,_,_,_,_,_,_)) => X
  rule validTo(intent(_,_,T,_,_,_,_,_,_,_)) => T
  rule aA(intent(_,_,_,budget(A,_,_,_,_),_,_,_,_,_,_)) => A
  rule grossCap(intent(_,_,_,budget(_,G,_,_,_),_,_,_,_,_,_)) => G
  rule feeCap(intent(_,_,_,budget(_,_,F,_,_),_,_,_,_,_,_)) => F
  rule aB(intent(_,_,_,budget(_,_,_,B,_),_,_,_,_,_,_)) => B
  rule netFloor(intent(_,_,_,budget(_,_,_,_,N),_,_,_,_,_,_)) => N
  rule cpty(intent(_,_,_,_,escrow(C,_,_,_,_,_,_),_,_,_,_,_)) => C
  rule fundOf(intent(_,_,_,_,escrow(_,F,_,_,_,_,_),_,_,_,_,_)) => F
  rule target(intent(_,_,_,_,escrow(_,_,T,_,_,_,_),_,_,_,_,_)) => T
  rule delivery(intent(_,_,_,_,escrow(_,_,_,D,_,_,_),_,_,_,_,_)) => D
  rule refundAfter(intent(_,_,_,_,escrow(_,_,_,_,R,_,_),_,_,_,_,_)) => R
  rule prio(intent(_,_,_,_,escrow(_,_,_,_,_,P,_),_,_,_,_,_)) => P
  rule partial(intent(_,_,_,_,escrow(_,_,_,_,_,_,P),_,_,_,_,_)) => P
  rule failPol(intent(_,_,_,_,_,FP,_,_,_,_)) => FP
  rule auth(intent(_,_,_,_,_,_,M,_,_,_)) => M
  rule footprint(intent(_,_,_,_,_,_,_,S,_,_)) => S
  rule claims(intent(_,_,_,_,_,_,_,_,recovery(C,_,_),_)) => C
  rule assumptions(intent(_,_,_,_,_,_,_,_,recovery(_,A,_),_)) => A
  rule reserve(intent(_,_,_,_,_,_,_,_,recovery(_,_,R),_)) => R
  rule cap(intent(_,_,_,_,_,_,_,_,_,C)) => C
  rule isLocal(I) => true  requires isLocalD(delivery(I))
  rule isLocal(I) => false requires notBool isLocalD(delivery(I))
  syntax Bool ::= isLocalD(Delivery) [function, total]
  rule isLocalD(local(_)) => true
  rule isLocalD(foreign(_, _)) => false
  rule need(I) => 1 +Int b2i(notBool isLocal(I)) +Int b2i(partial(I) ==K proRata)

  // ---------- escrow views ----------
  syntax Int ::= dview(Intent, EscRec) [function, total] | payableB(Intent, EscRec) [function, total]
               | paidView(Intent, EscRec) [function, total] | fview(DStatus) [function, total]
  rule fview(dObserved(Q)) => Q
  rule fview(dClosed(Q)) => Q
  rule fview(_) => 0 [owise]
  rule dview(I, esc(_, D, _, _, _)) => D requires isLocal(I)
  rule dview(I, esc(_, _, _, _, S)) => fview(S) requires notBool isLocal(I)
  rule payableB(I, esc(_, D, PB, _, _)) => D -Int PB requires isLocal(I)
  rule payableB(I, _) => 0 requires notBool isLocal(I)
  rule paidView(I, esc(_, _, PB, _, _)) => PB requires isLocal(I)
  rule paidView(I, esc(_, _, _, _, S)) => fview(S) requires notBool isLocal(I)
  syntax Bool ::= finalRelease(Intent, EscRec) [function, total]
                | refundEnabled(Intent, EscRec, Int) [function, total]
                | foreignClosed(Intent, EscRec) [function, total]
                | settled(Intent, EscRec) [function, total]
                | settlesAt(Intent, Int, Int) [function, total]     // released A vs paid B
  rule finalRelease(I, E) => dview(I, E) >=Int target(I)
  rule foreignClosed(I, esc(_, _, _, _, S)) => isLocal(I) orBool isClosed(S)
  syntax Bool ::= isClosed(DStatus) [function, total]
  rule isClosed(dClosed(_)) => true
  rule isClosed(_) => false [owise]
  rule refundEnabled(I, E, NOW) => NOW >Int refundAfter(I) andBool foreignClosed(I, E)
  // floor(fund*paid/target) == RA, by cross-multiplication only (Φ₀; native needs two limbs)
  rule settlesAt(I, RA, PAID) => RA *Int target(I) <=Int fundOf(I) *Int PAID
                                 andBool fundOf(I) *Int PAID <Int (RA +Int 1) *Int target(I)
  rule settled(I, E) => true requires partial(I) ==K noPartial
  rule settled(I, esc(P, D, PB, RA, S)) => settlesAt(I, RA, paidView(I, esc(P, D, PB, RA, S)))
       requires partial(I) ==K proRata

  // ---------- admission of `open` ----------
  rule <k> open(EP, I, AR)
        => ensure(AR ==Int 1, "ARITY_UNSUPPORTED")
        ~> ensure(notBool EP in_keys(EPS), "EPISODE_EXISTS")
        ~> ensure(notBool I in RP, "INTENT_REPLAY")
        ~> ensure(NOW <=Int validTo(I), "INTENT_EXPIRED")
        ~> ensure(intentShapeOk(I), "INTENT_MALFORMED")
        ~> ensure(prio(I) ==K prioRelease orBool prio(I) ==K prioRefund, "PRIORITY_UNSUPPORTED")
        ~> ensure(isLocal(I) orBool (partial(I) ==K proRata andBool prio(I) ==K prioRelease), "FOREIGN_POLICY")
        ~> ensure(grantsOk(values(auth(I)), EP), "AUTHORITY_MALFORMED")
        ~> ensure(disjointKeys(keys_list(auth(I)), GR), "GRANT_ID_REUSE")
        ~> ensure(staticCells(I, EP) <=Set footprint(I), "FOOTPRINT_INCOMPLETE")
        ~> ensure(recoveryGrounded(I), "RECOVERY_UNGROUNDED")
        ~> commitOpen(EP, I) ... </k>
       <eps> EPS </eps> <replay> RP </replay> <now> NOW </now> <grants> GR </grants>

  rule <k> commitOpen(EP, I) => .K ... </k>
       <eps> EPS => EPS[EP <- ep(I, genesis(I), 1, 0, 0, 0, esc(unfunded, 0, 0, 0, dNone), false)] </eps>
       <grants> GR => updateMap(GR, auth(I)) </grants>
       <replay> RP => RP |Set SetItem(I) </replay>
       <out> ... .List => ListItem(accepted(EP, 0)) </out>

  syntax Bool ::= intentShapeOk(Intent) [function, total] | localDomainOk(Intent) [function, total]
                | grantsOk(List, String) [function, total] | profileRight(Right) [function, total]
                | holderPlain(Holder) [function, total] | disjointKeys(List, Map) [function, total]
                | recoveryGrounded(Intent) [function, total] | hasRecoverGrant(List, Int) [function, total]
                | expiresAfter(Expiry, Int) [function, total]
  rule intentShapeOk(I) =>
         fundOf(I) >Int 0 andBool fundOf(I) <=Int u128()
         andBool target(I) >Int 0 andBool target(I) <=Int u128()
         andBool grossCap(I) >=Int 0 andBool feeCap(I) >=Int 0 andBool netFloor(I) >=Int 0
         andBool aA(I) =/=K aB(I) andBool owner(I) =/=K cpty(I)
         andBool isPlainAcct(owner(I)) andBool isPlainAcct(cpty(I))
         andBool cap(I) >=Int 1 andBool cap(I) <=Int 8
         andBool reserve(I) >=Int 0 andBool reserve(I) <Int cap(I)
         andBool localDomainOk(I)
  rule localDomainOk(I) => delivery(I) ==K local(exec(I)) requires isLocal(I)
  rule localDomainOk(intent(_,X,_,_,escrow(_,_,_,foreign(D,P),_,_,_),_,_,_,_,_))
       => D =/=K X andBool P =/=String ""
  rule grantsOk(.List, _) => true
  rule grantsOk(ListItem(grant(R, H, EPG, FROM, _, REM, REV)) L, EP)
       => profileRight(R) andBool holderPlain(H) andBool EPG ==String EP andBool FROM >=Int 0
          andBool REM >=Int 0 andBool REM <=Int u128() andBool notBool REV andBool grantsOk(L, EP)
  rule grantsOk(_, _) => false [owise]
  rule profileRight(rInitiate) => true
  rule profileRight(rComplete) => true
  rule profileRight(rRecover) => true
  rule profileRight(rReconcile) => true
  rule profileRight(_) => false [owise]          // disclose/amend/issue/enforce: not in P-ESC1
  rule holderPlain(anyone) => true
  rule holderPlain(only(X)) => isPlainAcct(X)
  rule disjointKeys(.List, _) => true
  rule disjointKeys(ListItem(K) L, GR) => notBool K in_keys(GR) andBool disjointKeys(L, GR)
  rule recoveryGrounded(I) => true requires notBool claims(I)
  rule recoveryGrounded(I) =>
         (SetItem(inclusion) |Set SetItem(actorArrival) |Set SetItem(witnessAvailability)) <=Set assumptions(I)
         andBool (isLocal(I) orBool verifierLiveness in assumptions(I))
         andBool hasRecoverGrant(values(auth(I)), refundAfter(I))
         andBool reserve(I) >=Int need(I)
       requires claims(I)
  rule hasRecoverGrant(.List, _) => false
  rule hasRecoverGrant(ListItem(grant(rRecover, _, _, _, X, _, false)) _, T) => true
       requires expiresAfter(X, T)
  rule hasRecoverGrant(ListItem(_) L, T) => hasRecoverGrant(L, T) [owise]
  rule expiresAfter(indefinite, _) => true
  rule expiresAfter(until(E), T) => E >Int T

  syntax Set ::= staticCells(Intent, String) [function, total] | grantCells(List) [function, total]
  rule staticCells(I, EP) =>
         SetItem(balC(exec(I), owner(I), aA(I))) |Set SetItem(balC(exec(I), custody(EP), aA(I)))
         |Set SetItem(balC(exec(I), cpty(I), aA(I)))
         |Set SetItem(escrowC(EP)) |Set SetItem(headC(EP)) |Set SetItem(replayC(EP))
         |Set #if feeCap(I) >Int 0 #then SetItem(actorBalC(exec(I), aA(I))) #else .Set #fi
         |Set #if isLocal(I)
              #then SetItem(balC(exec(I), cpty(I), aB(I))) |Set SetItem(balC(exec(I), custody(EP), aB(I)))
                    |Set SetItem(balC(exec(I), owner(I), aB(I)))
              #else .Set #fi
         |Set grantCells(keys_list(auth(I)))
  rule grantCells(.List) => .Set
  rule grantCells(ListItem(G:String) L) => SetItem(grantC(G)) |Set grantCells(L)
  rule grantCells(ListItem(_) L) => grantCells(L) [owise]   // dominated: grantsOk/AUTHORITY_MALFORMED

  // ---------- ledger inputs outside the episode ----------
  rule <k> seed(M) => ensure(B ==K .Map andBool EPS ==K .Map, "SEED_AFTER_START")
                   ~> ensure(seedOk(keys_list(M), M), "SEED_MALFORMED") ~> setBal(M) ... </k>
       <bal> B </bal> <eps> EPS </eps>
  rule <k> setBal(M) => .K ... </k> <bal> _ => M </bal>
  syntax Bool ::= seedOk(List, Map) [function, total]
  rule seedOk(.List, _) => true
  rule seedOk(ListItem(bk(D, acct(S), A)) L, M) =>
         isInt(M[bk(D, acct(S), A)]) andBool getI(M, bk(D, acct(S), A)) >=Int 0
         andBool getI(M, bk(D, acct(S), A)) <=Int u128() andBool seedOk(L, M)
  rule seedOk(_, _) => false [owise]              // custody or non-balance keys reject

  rule <k> tick(T) => ensure(T >=Int NOW, "CLOCK_REGRESSION") ~> setNow(T) ... </k> <now> NOW </now>
  rule <k> setNow(T) => .K ... </k> <now> _ => T </now>

  // Imported status: the named trust premise. Not a proof of the foreign fact.
  rule <k> importEvidence(EP, P, S)
        => ensure(EP in_keys(EPS), "NO_EPISODE")
        ~> ensure(premiseMatches(EPS, EP, P), "EVIDENCE_PREMISE")
        ~> ensure(statusStep(importedAt(IMP, EP), S), "EVIDENCE_REGRESSION")
        ~> setImported(EP, S) ... </k>
       <eps> EPS </eps> <imported> IMP </imported>
  rule <k> setImported(EP, S) => .K ... </k> <imported> IMP => IMP[EP <- S] </imported>
  syntax DStatus ::= importedAt(Map, String) [function, total]
  rule importedAt(IMP, EP) => {IMP[EP]}:>DStatus requires EP in_keys(IMP) andBool isDStatus(IMP[EP])
  rule importedAt(_, _) => dNone [owise]
  syntax Bool ::= premiseMatches(Map, String, String) [function, total]
                | statusStep(DStatus, DStatus) [function, total]
  rule premiseMatches(EPS, EP, P) =>
         delivery(epIntent({EPS[EP]}:>EpRec)) ==K foreign(foreignDom(epIntent({EPS[EP]}:>EpRec)), P)
         andBool P =/=String ""
       requires EP in_keys(EPS) andBool isEpRec(EPS[EP])
  rule premiseMatches(_, _, _) => false [owise]
  syntax Intent ::= epIntent(EpRec) [function, total]
  rule epIntent(ep(I, _, _, _, _, _, _, _)) => I
  syntax Dom ::= foreignDom(Intent) [function, total]
  rule foreignDom(intent(_,_,_,_,escrow(_,_,_,foreign(D,_),_,_,_),_,_,_,_,_)) => D
  rule foreignDom(I) => exec(I) [owise]          // local: can never equal foreign(...)
  // Knowledge is monotone; reorg is excluded by the named finality premise.
  rule statusStep(dNone, dUnknown) => true
  rule statusStep(dNone, dObserved(Q)) => Q >Int 0
  rule statusStep(dUnknown, dObserved(Q)) => Q >Int 0
  rule statusStep(dObserved(Q), dObserved(Q2)) => Q2 >Int Q
  rule statusStep(dNone, dClosed(Q)) => Q >=Int 0
  rule statusStep(dUnknown, dClosed(Q)) => Q >=Int 0
  rule statusStep(dObserved(Q), dClosed(Q2)) => Q2 >=Int Q
  rule statusStep(_, _) => false [owise]         // unknown->unknown, closed->*, observed->unknown

  // ---------- stage admission (fixed rejection precedence) ----------
  rule <k> stage(EP, _, _, _, _, _, _, _, _, _) => failing("NO_EPISODE") ... </k>
       <eps> EPS </eps> requires notBool EP in_keys(EPS)

  rule <k> stage(EP, PRED, IDX, AR, ACTOR, GID, FEE, ACT, SUB, WR)
        => ensure(AR ==Int 1, "ARITY_UNSUPPORTED")
        ~> ensure(inProfile(ACT), "ACTION_UNSUPPORTED_IN_PROFILE")
        ~> ensure(isPlainAcct(ACTOR), "ACTOR_MALFORMED")
        // history_p
        ~> ensure(PRED ==K H, "STALE_PREDECESSOR")
        ~> ensure(IDX ==Int N, "STAGE_INDEX")
        ~> ensure(IDX <=Int cap(I), "EPISODE_LENGTH")
        ~> ensure(notBool TERM orBool ACT ==K reconcileA, "EPISODE_TERMINAL")
        ~> ensure(IDX <=Int cap(I) -Int reserve(I)
                  orBool closing(ACT, ctx(EP, I, E, ACTOR, NOW, IMP, RP, GR, TERM)), "CLOSURE_RESERVE")
        // authority_p
        ~> ensure(authorized(ACT, grantAt(GR, GID), ctx(EP, I, E, ACTOR, NOW, IMP, RP, GR, TERM)), "UNAUTHORIZED")
        ~> ensure(charge(ACT, I) <=Int remaining(grantAt(GR, GID)), "AUTHORITY_BUDGET")
        ~> ensure(feeAdmitted(ACT, FEE, ACTOR, I), "FEE_NOT_PERMITTED")
        // intent_p / program guard (pre-state only)
        ~> ensureCode(guard(ACT, FEE, ctx(EP, I, E, ACTOR, NOW, IMP, RP, GR, TERM)))
        // effect_p: line hygiene, footprint, ledger apply, E1, exact writes
        ~> ensure(allLocal(SUB, exec(I)), "FOREIGN_EFFECT")
        ~> ensure(noSupply(SUB), "ISSUE_UNAUTHORIZED")
        ~> ensure(linesWellFormed(SUB), "MALFORMED_LINE")
        ~> ensure(custodyScoped(SUB, EP), "CUSTODY_FOREIGN_EPISODE")
        ~> ensure(ownerOutflowsBudgeted(SUB, owner(I), aA(I)), "UNBUDGETED_OUTFLOW")
        ~> ensure(derivedCells(ACT, SUB, EP, GID) <=Set resolve(Set2List(footprint(I)), ACTOR), "FOOTPRINT_INCOMPLETE")
        ~> ensure(applyEffs(B, SUB) =/=K badApply, "INSUFFICIENT_OR_OVERFLOW")
        ~> ensure(conserves(WR, B, SUB), "CONSERVATION")
        ~> ensure(WR ==K writesOf(applyEffs(B, SUB), SUB), "WRITES_NOT_EXACT")
        // failure_p before exactness, per MIL/4 line 34
        ~> ensure(custodyConsistent(applyEffs(B, SUB), EP, I, next(ACT, ctx(EP, I, E, ACTOR, NOW, IMP, RP, GR, TERM))), "DUTY_DROPPED")
        ~> ensure(plist(SUB) ==K prepared(ACT, FEE, ctx(EP, I, E, ACTOR, NOW, IMP, RP, GR, TERM)), "EFFECT_MISMATCH")
        // cumulative episode limits and outcome
        ~> ensure(G +Int grossOut(SUB, owner(I)) <=Int grossCap(I), "GROSS_CAP")
        ~> ensure(F +Int feesOf(SUB) <=Int feeCap(I), "FEE_CAP")
        ~> ensure(netOk(ACT, I, E), "NET_FLOOR")
        ~> commitStage(EP, IDX, ACT, SUB, GID, next(ACT, ctx(EP, I, E, ACTOR, NOW, IMP, RP, GR, TERM)), applyEffs(B, SUB))
        ... </k>
       <now> NOW </now> <bal> B </bal> <grants> GR </grants>
       <imported> IMP </imported> <replay> RP </replay>
       <eps> ... EP |-> ep(I, H, N, G, F, _NB, E, TERM) ... </eps>

  rule <k> commitStage(EP, IDX, ACT, SUB, GID, E2, okMap(M2)) => .K ... </k>
       <bal> _ => M2 </bal>
       <eps> ... EP |-> (ep(I, H, N, G, F, NB, _, _)
             => ep(I, ext(H, IDX, ACT, SUB), N +Int 1, G +Int grossOut(SUB, owner(I)),
                   F +Int feesOf(SUB), NB +Int creditTo(SUB, owner(I), aB(I)), E2, isTerminal(E2))) ... </eps>
       <grants> GR => nextGrants(GR, GID, ACT, I) </grants>
       <replay> RP => nextReplay(RP, EP, ACT) </replay>
       <out> ... .List => ListItem(accepted(EP, IDX)) </out>
  rule <k> commitStage(_, _, _, _, _, _, badApply) => failing("INTERNAL_APPLY") ... </k>

  // ---------- constructor admission ----------
  syntax Bool ::= inProfile(Action) [function, total]
  rule inProfile(fundA) => true
  rule inProfile(deliverA(_, _)) => true
  rule inProfile(observeA(_)) => true
  rule inProfile(releaseA(_)) => true
  rule inProfile(refundA) => true
  rule inProfile(recoverA) => true
  rule inProfile(reconcileA) => true
  rule inProfile(revokeA(_)) => true
  rule inProfile(failA(releaseA(_))) => true
  rule inProfile(_) => false [owise]             // forkA, joinA, amendA, other failA: rejected

  syntax Bool ::= closing(Action, Ctx) [function, total]
  rule closing(refundA, _) => true
  rule closing(recoverA, _) => true
  rule closing(observeA(dClosed(_)), _) => true
  rule closing(releaseA(R), ctx(_, I, esc(_, D, PB, RA, S), _, _, _, _, _, _)) =>
         (finalRelease(I, esc(pending, D, PB, RA, S)) andBool R ==Int fundOf(I) -Int RA)
         orBool settlesAt(I, RA +Int R, #if isLocal(I) #then D #else fview(S) #fi)
  rule closing(_, _) => false [owise]

  // ---------- authority ----------
  syntax MaybeRight ::= needs(Action) [function, total]
  rule needs(fundA) => rInitiate
  rule needs(releaseA(_)) => rComplete
  rule needs(refundA) => rComplete
  rule needs(observeA(_)) => rComplete
  rule needs(recoverA) => rRecover
  rule needs(reconcileA) => rReconcile
  rule needs(failA(X)) => needs(X)
  rule needs(deliverA(_, _)) => selfAuth         // counterparty spends its own balance
  rule needs(revokeA(_)) => selfAuth             // owner revokes its own grant
  rule needs(_) => noRight [owise]
  syntax Grant ::= grantAt(Map, String) [function, total]
  rule grantAt(GR, GID) => {GR[GID]}:>Grant requires GID in_keys(GR) andBool isGrant(GR[GID])
  rule grantAt(_, _) => noGrant [owise]
  syntax Bool ::= authorized(Action, Grant, Ctx) [function, total]
                | holderOk(Holder, Acct) [function, total] | notExpired(Expiry, Int) [function, total]
  rule authorized(deliverA(_, _), _, ctx(_, I, _, ACTOR, _, _, _, _, _)) => ACTOR ==K cpty(I)
  rule authorized(revokeA(_), _, ctx(_, I, _, ACTOR, _, _, _, _, _)) => ACTOR ==K owner(I)
  rule authorized(ACT, grant(R, H, EPG, FROM, X, _, REV), ctx(EP, _, _, ACTOR, NOW, _, _, _, _))
       => needs(ACT) ==K R andBool holderOk(H, ACTOR) andBool EPG ==String EP
          andBool FROM <=Int NOW andBool notExpired(X, NOW) andBool notBool REV
       requires needs(ACT) =/=K selfAuth
  rule authorized(_, _, _) => false [owise]
  rule holderOk(anyone, _) => true
  rule holderOk(only(X), A) => X ==K A
  rule notExpired(indefinite, _) => true
  rule notExpired(until(E), NOW) => NOW <=Int E
  syntax Int ::= charge(Action, Intent) [function, total] | remaining(Grant) [function, total]
  rule charge(fundA, I) => fundOf(I)
  rule charge(releaseA(R), _) => R
  rule charge(_, _) => 0 [owise]                 // refund/recover never replenish or spend budget
  rule remaining(grant(_, _, _, _, _, REM, _)) => REM
  rule remaining(noGrant) => 0
  syntax Map ::= nextGrants(Map, String, Action, Intent) [function, total]
  rule nextGrants(GR, _, revokeA(G2), _) => GR[G2 <- revoked(grantAt(GR, G2))]
  rule nextGrants(GR, GID, ACT, I) => GR[GID <- debit(grantAt(GR, GID), charge(ACT, I))]
       requires charge(ACT, I) >Int 0 andBool notBool isRevoke(ACT)
  rule nextGrants(GR, _, _, _) => GR [owise]
  syntax Bool ::= isRevoke(Action) [function, total]
  rule isRevoke(revokeA(_)) => true
  rule isRevoke(_) => false [owise]
  syntax Grant ::= debit(Grant, Int) [function, total] | revoked(Grant) [function, total]
  rule debit(grant(R, H, E, F, X, REM, V), C) => grant(R, H, E, F, X, REM -Int C, V)
  rule debit(noGrant, _) => noGrant
  rule revoked(grant(R, H, E, F, X, REM, _)) => grant(R, H, E, F, X, REM, true)
  rule revoked(noGrant) => noGrant
  syntax Bool ::= feeAdmitted(Action, Int, Acct, Intent) [function, total]
  rule feeAdmitted(_, 0, _, _) => true
  rule feeAdmitted(releaseA(_), FEE, ACTOR, I) => FEE >Int 0 andBool ACTOR =/=K owner(I)
  rule feeAdmitted(failA(_), FEE, ACTOR, I) => FEE >Int 0 andBool ACTOR =/=K owner(I)
  rule feeAdmitted(_, _, _, _) => false [owise]

  // ---------- program guards (pre-state; Reject is never a fallback) ----------
  syntax String ::= guard(Action, Int, Ctx) [function, total]
                  | refundCode(Ctx) [function, total] | foreignRefundCode(Intent, DStatus) [function, total]
  rule guard(fundA, _, ctx(_, I, esc(P, _, _, _, _), _, NOW, _, _, _, _)) =>
         orc(chk(P ==K unfunded, "ESCROW_NOT_UNFUNDED"), chk(NOW <=Int validTo(I), "INTENT_EXPIRED"))
  rule guard(deliverA(R, Q), _, ctx(EP, I, esc(P, D, _, _, _), _, _, _, RP, _, _)) =>
         orc(chk(isLocal(I), "DELIVERY_NOT_LOCAL"),
         orc(chk(P ==K pending, "ESCROW_NOT_PENDING"),
         orc(chk(Q >Int 0, "ZERO_AMOUNT"),
         orc(chk(notBool rcpt(EP, R) in RP, "RECEIPT_REPLAY"),
             chk(D +Int Q <=Int target(I), "OVER_DELIVERY")))))
  rule guard(observeA(S), _, ctx(EP, I, esc(P, _, _, _, S0), _, _, IMP, _, _, _)) =>
         orc(chk(notBool isLocal(I), "OBSERVATION_NOT_FOREIGN"),
         orc(chk(P ==K pending, "ESCROW_NOT_PENDING"),
         orc(chk(S ==K importedAt(IMP, EP), "EVIDENCE_NOT_AUTHENTICATED"),
             chk(statusStep(S0, S), "NO_PROGRESS"))))
  rule guard(releaseA(R), _, ctx(_, I, esc(P, D, PB, RA, S), _, NOW, _, _, _, _)) =>
         orc(chk(P ==K pending, "ESCROW_NOT_PENDING"),
         orc(chk(R >Int 0, "ZERO_AMOUNT"),
         #if finalRelease(I, esc(P, D, PB, RA, S))
         #then orc(chk(R ==Int fundOf(I) -Int RA, "RELEASE_NOT_EXACT"),
                   chk(notBool (prio(I) ==K prioRefund andBool refundEnabled(I, esc(P, D, PB, RA, S), NOW)),
                       "RACE_PRIORITY_REFUND"))
         #else orc(chk(partial(I) ==K proRata, "PARTIAL_NOT_PERMITTED"),
                   chk((RA +Int R) *Int target(I) <=Int fundOf(I) *Int dview(I, esc(P, D, PB, RA, S)),
                       "PRORATA_BOUND"))      // partial settlement ignores priority (G7)
         #fi))
  rule guard(refundA, _, C) => refundCode(C)
  rule guard(recoverA, _, C) => refundCode(C)
  rule refundCode(ctx(_, I, esc(P, D, PB, RA, S), _, NOW, _, _, _, _)) =>
         orc(chk(P ==K pending, "ESCROW_NOT_PENDING"),
         orc(chk(NOW >Int refundAfter(I), "REFUND_TOO_EARLY"),
         orc(foreignRefundCode(I, S),
         orc(chk(notBool (prio(I) ==K prioRelease andBool finalRelease(I, esc(P, D, PB, RA, S))),
                 "RACE_PRIORITY_RELEASE"),
             chk(settled(I, esc(P, D, PB, RA, S)), "SETTLE_PARTIAL_FIRST")))))
  rule foreignRefundCode(I, _) => "" requires isLocal(I)
  rule foreignRefundCode(I, dUnknown) => "REFUND_OUTCOME_UNKNOWN" requires notBool isLocal(I)
  rule foreignRefundCode(I, dClosed(_)) => "" requires notBool isLocal(I)
  rule foreignRefundCode(_, _) => "REFUND_NONRECEIPT_UNPROVEN" [owise]
  rule guard(failA(releaseA(_)), FEE, ctx(_, I, _, _, _, _, _, _, _)) =>
         orc(chk(failPol(I) =/=K noFailBranch, "FAILURE_BRANCH_UNSIGNED"),
             chk(FEE >Int 0 andBool FEE <=Int maxRetained(failPol(I)), "FAILURE_FEE_BOUND"))
  rule guard(reconcileA, _, _) => ""
  rule guard(revokeA(G2), _, ctx(EP, I, _, _, _, _, _, GR, TERM)) =>
         orc(chk(grantEp(grantAt(GR, G2)) ==String EP, "NO_GRANT"),
             chk(notBool (grantRight(grantAt(GR, G2)) ==K rRecover andBool claims(I) andBool notBool TERM),
                 "REVOKE_WOULD_STRAND"))
  rule guard(_, _, _) => "ACTION_UNSUPPORTED_IN_PROFILE" [owise]
  syntax Int ::= maxRetained(FailPolicy) [function, total]
  rule maxRetained(retainFee(M)) => M
  rule maxRetained(noFailBranch) => 0
  syntax String ::= grantEp(Grant) [function, total]
  rule grantEp(grant(_, _, E, _, _, _, _)) => E
  rule grantEp(noGrant) => ""
  syntax MaybeRight ::= grantRight(Grant) [function, total]
  rule grantRight(grant(R, _, _, _, _, _, _)) => R
  rule grantRight(noGrant) => noRight

  // ---------- prepared (canonical) effects ----------
  syntax List ::= nz(Effect) [function, total]
  rule nz(xfer(_, _, _, _, 0, _)) => .List
  rule nz(E) => ListItem(E) [owise]              // a negative amount survives and fails MALFORMED_LINE
  syntax Prepared ::= prepared(Action, Int, Ctx) [function, total]
  rule prepared(fundA, _, ctx(EP, I, _, _, _, _, _, _, _)) =>
         plist(ListItem(xfer(exec(I), aA(I), owner(I), custody(EP), fundOf(I), tFund)))
  rule prepared(deliverA(R, Q), _, ctx(EP, I, _, ACTOR, _, _, _, _, _)) =>
         plist(ListItem(xfer(exec(I), aB(I), ACTOR, custody(EP), Q, tDeliver(R))))
  rule prepared(releaseA(R), FEE, ctx(EP, I, E, ACTOR, _, _, _, _, _)) =>
         plist(nz(xfer(exec(I), aA(I), custody(EP), cpty(I), R, tRelease))
               nz(xfer(exec(I), aB(I), custody(EP), owner(I), payableB(I, E), tRelease))
               nz(xfer(exec(I), aA(I), owner(I), ACTOR, FEE, tFee)))
  rule prepared(refundA, _, ctx(EP, I, esc(P, D, PB, RA, S), _, _, _, _, _, _)) =>
         plist(nz(xfer(exec(I), aA(I), custody(EP), owner(I), fundOf(I) -Int RA, tRefund))
               nz(xfer(exec(I), aB(I), custody(EP), cpty(I), payableB(I, esc(P, D, PB, RA, S)), tRefund)))
  rule prepared(recoverA, FEE, C) => prepared(refundA, FEE, C)
  rule prepared(failA(_), FEE, ctx(_, I, _, ACTOR, _, _, _, _, _)) =>
         plist(nz(xfer(exec(I), aA(I), owner(I), ACTOR, FEE, tFee)))
  rule prepared(observeA(_), _, _) => plist(.List)
  rule prepared(reconcileA, _, _) => plist(.List)
  rule prepared(revokeA(_), _, _) => plist(.List)
  rule prepared(_, _, _) => unpreparable [owise]

  // ---------- next escrow record ----------
  syntax EscRec ::= next(Action, Ctx) [function, total]
  rule next(fundA, ctx(_, _, esc(_, D, PB, RA, S), _, _, _, _, _, _)) => esc(pending, D, PB, RA, S)
  rule next(deliverA(_, Q), ctx(_, _, esc(P, D, PB, RA, S), _, _, _, _, _, _)) => esc(P, D +Int Q, PB, RA, S)
  rule next(observeA(S2), ctx(_, _, esc(P, D, PB, RA, _), _, _, _, _, _, _)) => esc(P, D, PB, RA, S2)
  rule next(releaseA(R), ctx(_, I, esc(P, D, PB, RA, S), _, _, _, _, _, _)) =>
         esc(#if finalRelease(I, esc(P, D, PB, RA, S)) #then released #else P #fi,
             D, #if isLocal(I) #then D #else PB #fi, RA +Int R, S)
  rule next(refundA, ctx(_, _, esc(_, D, PB, RA, S), _, _, _, _, _, _)) => esc(refunded, D, PB, RA, S)
  rule next(recoverA, C) => next(refundA, C)
  rule next(_, ctx(_, _, E, _, _, _, _, _, _)) => E [owise]    // failA/observe-none/reconcile/revoke
  syntax Bool ::= isTerminal(EscRec) [function, total]
  rule isTerminal(esc(released, _, _, _, _)) => true
  rule isTerminal(esc(refunded, _, _, _, _)) => true
  rule isTerminal(_) => false [owise]

  // ---------- custody duty (failure_p) ----------
  syntax Bool ::= custodyConsistent(Applied, String, Intent, EscRec) [function, total]
  syntax Int  ::= expectA(EPhase, Int, Int) [function, total] | expectB(EPhase, Int, Int) [function, total]
  rule custodyConsistent(okMap(M), EP, I, esc(P, D, PB, RA, _)) =>
         getI(M, bk(exec(I), custody(EP), aA(I))) ==Int expectA(P, fundOf(I), RA)
         andBool getI(M, bk(exec(I), custody(EP), aB(I))) ==Int expectB(P, D, PB)
  rule custodyConsistent(badApply, _, _, _) => false
  rule expectA(pending, F, RA) => F -Int RA
  rule expectA(_, _, _) => 0 [owise]
  rule expectB(pending, D, PB) => D -Int PB
  rule expectB(_, _, _) => 0 [owise]

  // ---------- line hygiene, footprint, apply, conservation ----------
  syntax Bool ::= allLocal(List, Dom) [function, total] | noSupply(List) [function, total]
                | linesWellFormed(List) [function, total] | custodyScoped(List, String) [function, total]
                | ownerOutflowsBudgeted(List, Acct, Asset) [function, total]
                | custodyOk(Acct, String) [function, total]
  rule allLocal(.List, _) => true
  rule allLocal(ListItem(xfer(D, _, _, _, _, _)) L, X) => D ==K X andBool allLocal(L, X)
  rule allLocal(ListItem(mint(D, _, _, _)) L, X) => D ==K X andBool allLocal(L, X)
  rule allLocal(ListItem(burn(D, _, _, _)) L, X) => D ==K X andBool allLocal(L, X)
  rule allLocal(_, _) => false [owise]           // foreignCredit and non-effects
  rule noSupply(.List) => true
  rule noSupply(ListItem(xfer(_, _, _, _, _, _)) L) => noSupply(L)
  rule noSupply(_) => false [owise]              // mint/burn need an rIssue grant: not in P-ESC1
  rule linesWellFormed(.List) => true
  rule linesWellFormed(ListItem(xfer(_, _, F, T, Q, _)) L) =>
         Q >Int 0 andBool Q <=Int u128() andBool F =/=K T andBool linesWellFormed(L)
  rule linesWellFormed(_) => false [owise]
  rule custodyOk(acct(_), _) => true
  rule custodyOk(custody(C), EP) => C ==String EP
  rule custodyScoped(.List, _) => true
  rule custodyScoped(ListItem(xfer(_, _, F, T, _, _)) L, EP) =>
         custodyOk(F, EP) andBool custodyOk(T, EP) andBool custodyScoped(L, EP)
  rule custodyScoped(_, _) => false [owise]
  rule ownerOutflowsBudgeted(.List, _, _) => true
  rule ownerOutflowsBudgeted(ListItem(xfer(_, A, F, _, _, _)) L, O, BA) =>
         (F =/=K O orBool A ==K BA) andBool ownerOutflowsBudgeted(L, O, BA)
  rule ownerOutflowsBudgeted(_, _, _) => false [owise]

  syntax Set ::= derivedCells(Action, List, String, String) [function, total]
               | lineCells(List) [function, total] | resolve(List, Acct) [function, total]
               | touched(List) [function, total]
  rule derivedCells(ACT, SUB, EP, GID) =>
         lineCells(SUB) |Set SetItem(escrowC(EP)) |Set SetItem(headC(EP))
         |Set #if GID =/=String "" #then SetItem(grantC(GID)) #else .Set #fi
         |Set #if isDeliver(ACT) #then SetItem(replayC(EP)) #else .Set #fi
  syntax Bool ::= isDeliver(Action) [function, total]
  rule isDeliver(deliverA(_, _)) => true
  rule isDeliver(_) => false [owise]
  rule lineCells(.List) => .Set
  rule lineCells(ListItem(xfer(D, A, F, T, _, _)) L) =>
         SetItem(balC(D, F, A)) |Set SetItem(balC(D, T, A)) |Set lineCells(L)
  rule lineCells(ListItem(_) L) => lineCells(L) [owise]    // dominated by noSupply/allLocal
  rule resolve(.List, _) => .Set
  rule resolve(ListItem(actorBalC(D, A)) L, X) => SetItem(balC(D, X, A)) |Set resolve(L, X)
  rule resolve(ListItem(C) L, X) => SetItem(C) |Set resolve(L, X) [owise]
  rule touched(.List) => .Set
  rule touched(ListItem(xfer(D, A, F, T, _, _)) L) => SetItem(bk(D, F, A)) |Set SetItem(bk(D, T, A)) |Set touched(L)
  rule touched(ListItem(_) L) => touched(L) [owise]

  syntax Applied ::= applyEffs(Map, List) [function, total]
  rule applyEffs(M, .List) => okMap(M)
  rule applyEffs(M, ListItem(xfer(D, A, F, T, Q, _)) L)
    => applyEffs(M[bk(D, F, A) <- getI(M, bk(D, F, A)) -Int Q][bk(D, T, A) <- getI(M, bk(D, T, A)) +Int Q], L)
    requires Q >Int 0 andBool F =/=K T andBool getI(M, bk(D, F, A)) >=Int Q
     andBool getI(M, bk(D, T, A)) +Int Q <=Int u128()
  rule applyEffs(_, _) => badApply [owise]       // underflow, overflow, or unadmitted line kind

  syntax Map ::= writesOf(Applied, List) [function, total] | restrict(Map, List) [function, total]
  rule writesOf(okMap(M), SUB) => restrict(M, Set2List(touched(SUB)))
  rule writesOf(badApply, _) => .Map             // dominated by INSUFFICIENT_OR_OVERFLOW
  rule restrict(_, .List) => .Map
  rule restrict(M, ListItem(K) L) => restrict(M, L)[K <- getI(M, K)]

  // E1 over the CLAIMED writes: for each (d,a), Σ Δbalance = Σ mint − Σ burn.
  syntax Bool ::= conserves(Map, Map, List) [function, total]
                | balKeys(List) [function, total] | intVals(List) [function, total]
  rule conserves(WR, B, SUB) =>
         balKeys(keys_list(WR)) andBool intVals(values(WR))
         andBool dropZero(daSums(keys_list(WR), WR, B, .Map)) ==K dropZero(supplyDelta(SUB, .Map))
  rule balKeys(.List) => true
  rule balKeys(ListItem(bk(_, _, _)) L) => balKeys(L)
  rule balKeys(_) => false [owise]
  rule intVals(.List) => true
  rule intVals(ListItem(V:Int) L) => V >=Int 0 andBool V <=Int u128() andBool intVals(L)
  rule intVals(_) => false [owise]
  syntax Map ::= daSums(List, Map, Map, Map) [function, total] | supplyDelta(List, Map) [function, total]
               | dropZero(Map) [function, total] | dropZeroL(Map, List) [function, total]
  rule daSums(.List, _, _, ACC) => ACC
  rule daSums(ListItem(bk(D, X, A)) L, WR, B, ACC) =>
         daSums(L, WR, B, ACC[da(D, A) <- getI(ACC, da(D, A)) +Int getI(WR, bk(D, X, A)) -Int getI(B, bk(D, X, A))])
  rule daSums(ListItem(_) L, WR, B, ACC) => daSums(L, WR, B, ACC) [owise]   // dominated by balKeys
  rule supplyDelta(.List, ACC) => ACC
  rule supplyDelta(ListItem(mint(D, A, _, Q)) L, ACC) => supplyDelta(L, ACC[da(D, A) <- getI(ACC, da(D, A)) +Int Q])
  rule supplyDelta(ListItem(burn(D, A, _, Q)) L, ACC) => supplyDelta(L, ACC[da(D, A) <- getI(ACC, da(D, A)) -Int Q])
  rule supplyDelta(ListItem(_) L, ACC) => supplyDelta(L, ACC) [owise]
  rule dropZero(M) => dropZeroL(M, keys_list(M))
  rule dropZeroL(M, .List) => M
  rule dropZeroL(M, ListItem(K) L) => dropZeroL(M[K <- undef], L) requires getI(M, K) ==Int 0
  rule dropZeroL(M, ListItem(K) L) => dropZeroL(M, L) requires getI(M, K) =/=Int 0

  // ---------- cumulative measures ----------
  syntax Int ::= grossOut(List, Acct) [function, total] | feesOf(List) [function, total]
               | creditTo(List, Acct, Asset) [function, total]
  rule grossOut(.List, _) => 0
  rule grossOut(ListItem(xfer(_, _, F, T, Q, _)) L, O) => Q +Int grossOut(L, O) requires F ==K O andBool T =/=K O
  rule grossOut(ListItem(burn(_, _, F, Q)) L, O) => Q +Int grossOut(L, O) requires F ==K O
  rule grossOut(ListItem(_) L, O) => grossOut(L, O) [owise]   // inflows never subtract
  rule feesOf(.List) => 0
  rule feesOf(ListItem(xfer(_, _, _, _, Q, tFee)) L) => Q +Int feesOf(L)
  rule feesOf(ListItem(_) L) => feesOf(L) [owise]
  rule creditTo(.List, _, _) => 0
  rule creditTo(ListItem(xfer(_, A, _, T, Q, _)) L, O, B) => Q +Int creditTo(L, O, B) requires T ==K O andBool A ==K B
  rule creditTo(ListItem(_) L, O, B) => creditTo(L, O, B) [owise]
  syntax Bool ::= netOk(Action, Intent, EscRec) [function, total]
  rule netOk(releaseA(_), I, E) => dview(I, E) >=Int netFloor(I) requires finalRelease(I, E)
  rule netOk(_, _, _) => true [owise]            // net is promised only on successful completion
  syntax Set ::= nextReplay(Set, String, Action) [function, total]
  rule nextReplay(RP, EP, deliverA(R, _)) => RP |Set SetItem(rcpt(EP, R))
  rule nextReplay(RP, _, _) => RP [owise]
endmodule
```

### Constructor admission table

| Constructor | Status in P-ESC1 | Admission rule / rejection code |
|---|---|---|
| `seed` | Admitted once | `SEED_AFTER_START`, `SEED_MALFORMED` (no custody keys) |
| `tick` | Admitted | `CLOCK_REGRESSION` |
| `importEvidence` | Admitted for foreign escrows only | `NO_EPISODE`, `EVIDENCE_PREMISE`, `EVIDENCE_REGRESSION` |
| `open` | Admitted | `ARITY_UNSUPPORTED`, `EPISODE_EXISTS`, `INTENT_REPLAY`, `INTENT_EXPIRED`, `INTENT_MALFORMED`, `PRIORITY_UNSUPPORTED`, `FOREIGN_POLICY`, `AUTHORITY_MALFORMED`, `GRANT_ID_REUSE`, `FOOTPRINT_INCOMPLETE`, `RECOVERY_UNGROUNDED` |
| `fundA` | Admitted | Guard: `ESCROW_NOT_UNFUNDED`, `INTENT_EXPIRED`. Needs `rInitiate`, charge = fund |
| `deliverA` | Local only | Guard: `DELIVERY_NOT_LOCAL`, `ESCROW_NOT_PENDING`, `ZERO_AMOUNT`, `RECEIPT_REPLAY`, `OVER_DELIVERY`. Actor must be the counterparty |
| `observeA` | Foreign only | Guard: `OBSERVATION_NOT_FOREIGN`, `ESCROW_NOT_PENDING`, `EVIDENCE_NOT_AUTHENTICATED`, `NO_PROGRESS` |
| `releaseA` | Admitted | Guard: `RELEASE_NOT_EXACT`, `RACE_PRIORITY_REFUND`, `PARTIAL_NOT_PERMITTED`, `PRORATA_BOUND` |
| `refundA` / `recoverA` | Admitted | Guard: `REFUND_TOO_EARLY`, `REFUND_OUTCOME_UNKNOWN`, `REFUND_NONRECEIPT_UNPROVEN`, `RACE_PRIORITY_RELEASE`, `SETTLE_PARTIAL_FIRST`. Need `rComplete` / `rRecover` respectively |
| `failA(releaseA _)` | Admitted if signed | Guard: `FAILURE_BRANCH_UNSIGNED`, `FAILURE_FEE_BOUND`. Escrow unchanged; fee counts toward gross and fees |
| `reconcileA` | Admitted, including after terminal | No effects (`EFFECT_MISMATCH` otherwise). Needs `rReconcile` |
| `revokeA` | Owner only | Guard: `NO_GRANT`, `REVOKE_WOULD_STRAND` |
| `forkA`, `joinA`, `amendA`, other `failA` | **Rejected** | `ACTION_UNSUPPORTED_IN_PROFILE` |
| Effects `mint` / `burn` | **Rejected** | `ISSUE_UNAUTHORIZED` |
| Effect `foreignCredit` | **Rejected** | `FOREIGN_EFFECT` |
| Rights `rDisclose`, `rAmend`, `rIssue`, `rEnforce` | **Rejected at open** | `AUTHORITY_MALFORMED` |
| Priority `signedOrder` | **Rejected** | `PRIORITY_UNSUPPORTED` |

Stage precedence groups the checks by MIL/4 judgment: arity/profile, then `history_p`, `authority_p`, the guard, `effect_p` hygiene and E1, `failure_p` (`DUTY_DROPPED`), exactness, and finally the cumulative limits. MIL/4:34 requires this order.

## 5. Traces

I derived these expected results by hand from the K text above. I did not execute them. `$PGM` can't carry Maps as parsed text, so fixtures would need a KAST codec like the existing `codec.py`.

**Base intent `I0`.** Local domain `X`, owner `o`, counterparty `c`, solver `s`. Budget: `budget(A, 11, 1, B, 20)`. Escrow: `escrow(c, 10, 20, local(X), 100, prioRelease, noPartial)`. Failure policy: `retainFee(1)`. Grants:
- `g1 = rInitiate only(o)` until 100, budget 10
- `g2 = rComplete anyone` until 200, budget 10
- `g3 = rRecover only(o)` from 101, indefinite, budget 0

Recovery: `recovery(true, {inclusion, actorArrival, witnessAvailability}, 1)`, cap 8, footprint = `staticCells`. Seed: `o:A = 12`, `c:B = 20`.

**P1 (positive):**
1. `open(e1)` → `accepted(e1,0)`
2. `tick(10)`
3. Fund (idx 1) → accepted; gross is 10.
4. `deliverA("r1",20)` by `c` (idx 2) → accepted; custody B = 20.
5. `releaseA(10)` by `s` with fee 1 (idx 3). Lines: custody→c 10 A, custody→o 20 B, o→s 1 A. Gross 11 ≤ 11, fees 1 ≤ 1, net 20 ≥ 20, custody back to 0. Result `accepted(e1,3)`; the episode is terminal.

**Hostile cases** (each is one change to P1 unless stated):

| # | Change | Expected | Discriminates |
|---|---|---|---|
| H1 | Showcase numbers: fund 11 and seed `o:A = 12`, then step 5 with fee 1 | `GROSS_CAP` (11+1) | G2 |
| H2 | Step 5 redirects the 10 A to `s`; writes consistent | `EFFECT_MISMATCH` | G13a |
| H3 | Step 5 has correct lines, but the claimed writes move 1 more A from `o` to `s` | `WRITES_NOT_EXACT` | G13b |
| H4 | Claimed writes credit `s:A = 2` with no matching debit | `CONSERVATION` | E1 |
| H5 | No delivery; `tick(101)`; refund returns 9 A | `DUTY_DROPPED` | G14 / MIL/4:34 |
| H6 | `tick(101)`, refund at idx 2, then deliver at idx 3 | refund accepted; deliver `EPISODE_TERMINAL` | late-success race |
| H7 | Deliver 20, `tick(101)`, refund | `RACE_PRIORITY_RELEASE`; then release is accepted | MIL/2 §7 priority |
| H8 | Two idx-3 releases on the same predecessor | first accepted; second `STALE_PREDECESSOR` | head compare-and-swap (G16) |
| H9 | `open(e2, I0)` | `INTENT_REPLAY` | G11 |
| H10 | Cap 4, reserve 1: fund, deliver 5, deliver 5, then deliver 5 at idx 4 | `CLOSURE_RESERVE`; after `tick(101)`, refund at idx 4 accepted (10 A to `o`, 10 B to `c`) | G9 |
| H11 | Foreign `I0` (`proRata`, `prioRelease`, `verifierLiveness` named, reserve 3): import `dUnknown`, observe, `tick(101)`, refund | `REFUND_OUTCOME_UNKNOWN`; recover gives the same | G10 |
| H11b | Continue H11: import `dClosed(0)`, observe, refund | accepted | nonreceipt premise |
| H12 | Foreign `I0` without `verifierLiveness` | `open` → `RECOVERY_UNGROUNDED` | G10 |
| H13 | `revokeA("g3")` by `o` while pending | `REVOKE_WOULD_STRAND` | G15 |
| H14 | Arity 2, or `forkA` | `ARITY_UNSUPPORTED` / `ACTION_UNSUPPORTED_IN_PROFILE` | MIL/2 §14 d3/d4 |
| H15 | Deliver `"r1"` twice (5 each) | `RECEIPT_REPLAY` | receipt linearity |
| H16 | `proRata`: deliver 7, `releaseA(3)` accepted, `releaseA(1)` | `PRORATA_BOUND` (80 > 70); after `tick(101)`, refund returns 7 A | G1, G8 |
| H17 | `failA(releaseA(10))` fee 1 under `noFailBranch` | `FAILURE_BRANCH_UNSIGNED`; under `retainFee(1)` accepted, escrow unchanged | failure_p |
| H18 | Submitted lines include `foreignCredit` | `FOREIGN_EFFECT` | MIL/2 §10 |

## 6. Pseudocode (not K) for the constructors this profile rejects

```
fork(ep@h, branches[k ≤ 2]):             -- U3 candidate
  consume head h; each branch gets fresh epId_i with genesis' = forkOf(h, i)
  linear: receipts(ep) = ⊎ receipts_i (disjoint partition; union complete)
  affine: parent grant remaining R split into sub-grants r_i, Σ r_i ≤ R; parent remaining := 0
  cumulative gross/fees/duties copied to a shared parent-cumulative cell; every branch stage
    serializes on that cell (branches sharing an affine budget are not independent — MIL/3 §6)
join(ep1@h1, ep2@h2) with fan-in 2:
  both heads current and consumed exactly once; duties_3 = duties_1 ⊎ duties_2
  gross_3 = shared-parent-cumulative (never max/min of branches)
successor(ep@h, I'):
  I' names ep's intent digest as lifetime parent; carries gross, fees, duties, replay-set
  commitment and tombstones; its caps are the parent's remaining caps, never fresh ones
multi-signer |S| > 1: a quantifier over signed intents sharing one write set; each intent's
  cumulative cells are checked separately; post-U4.
native binding map: every EpRec field and every ensure must name its circuit constraint,
  authenticated ledger read or ledger check (consolidated:135); heads use Poseidon under a
  domain tag; the K free constructor assumes injectivity.
```

## 7. Coverage table

| Source requirement (section, line) | Where it is handled | Form | Status |
|---|---|---|---|
| MIL/2 §2 l.46: link fields `episodeId`, `predecessorHead`, `intentDigest`, cumulative gross/fees, roll-forward, tombstone | `ep(...)`, `ext(...)`, `STALE_PREDECESSOR`, `STAGE_INDEX`, `EPISODE_TERMINAL`; the intent is read from the ledger, not the stage | K | Covered for single-escrow episodes; the per-escrow tombstone is the phase |
| MIL/2 §3.4 l.100–102: lock sum ≤ balance | Custody-account model only | K (partial) | **Gap:** no general encumbrance cells |
| MIL/2 §6 l.191: rights, budget, window, revocation | `grant`, `authorized`, `charge`, `revokeA`, `profileRight` | K | 4 of 8 rights admitted; the other 4 explicitly rejected |
| MIL/2 §7 l.199–219: escrow machine, per-witness exclusivity, priority, pending legitimate, recovery viability | `guard`, `refundCode`, `recoveryGrounded`, `isTerminal` | K | Covered; `signed_order` rejected (G6); partial/unknown added (G1) |
| MIL/2 §9 l.239–250: cells, derived ⊆ declared, fork partitions | `derivedCells`, `resolve`, `actorBalC`, `staticCells` | K / pseudocode | Footprint covered; fork is pseudocode only |
| MIL/2 §10 l.254–256: local effects only; general conservation | `allLocal`, `conserves`, `supplyDelta` | K | Covered; mint/burn inactive (`ISSUE_UNAUTHORIZED`) |
| MIL/2 §10 l.258: residue never covers an effect | `EFFECT_MISMATCH` (no residue path) | K | Covered by construction |
| MIL/2 §11 l.263: digest as public input | Free-constructor heads | Pseudocode | **Native obligation** |
| MIL/2 §12 l.281–282: 8 stages, fan-in 2 | `cap ≤ 8`, reserve; join rejected | K | Covered; **reserve is new** (G9) |
| MIL/2 §13 showcase | Traces H1, G2–G5 | Counterexamples | **Showcase fails as written** |
| MIL/2 §14 decisions 2, 3, 4 | `ARITY_UNSUPPORTED`, `forkA`/`joinA` rejected | K | Covered as rejection |
| MIL/2 §17: recovery viability, derived footprint | `RECOVERY_UNGROUNDED`, `FOOTPRINT_INCOMPLETE` | K | Specified, not proved |
| MIL/4 l.20 M4-C4: delete a derived endpoint → reject | `FOOTPRINT_INCOMPLETE` | K | Covered |
| MIL/4 l.21 M4-C5: substitute recipient, revoke, replay receipt, omit duty | H2, `UNAUTHORIZED` after revoke, H15, H5 | K | Covered locally; the signature scheme is out of scope |
| MIL/4 l.23–34: six judgments and the precedence rule | Precedence groups in the stage rule | K | Covered; `DUTY_DROPPED` placed before `EFFECT_MISMATCH` |
| MIL/4 l.18 M4-C2: rounding beneficiary | Pro-rata floor | K + open question | **Conflict open** (G8) |
| MIL/4 l.50, l.61: bridge partial and nonreceipt | `dObserved`/`dClosed`, `REFUND_OUTCOME_UNKNOWN` | K | Local rules only; foreign truth stays a premise |
| Consolidated l.131–137: complete effects, prover can't pick a projection | `WRITES_NOT_EXACT`, `EFFECT_MISMATCH` | K | Covered |
| Consolidated l.143–145: gross vs net, refunds can't restore caps, pending exposure | `grossOut` (monotone), `INTENT_REPLAY` | K | Covered; cross-asset "fees against net" noted (§3) |
| Consolidated l.151–153: state distinctions; timeout isn't nonreceipt | Phase + counters + `DStatus` | K | Covered for escrow; "in flight" is local head compare-and-swap (G16) |
| Consolidated l.155, l.159: bounded stages, liveness named, reserve, recovery grant outlives ordinary authority, revocation keeps duties | Cap, `need`, `recoveryGrounded`, `g3` indefinite, `REVOKE_WOULD_STRAND` | K | Covered |
| Consolidated l.161: sequential prefixes retained | Committed stages are never undone; `failing` writes nothing | K | Covered |
| Consolidated l.168, l.172: ledger induction, successor episodes without budget reset | Head chain; successor | K / pseudocode | **Successor gap** (G17) |
| ROADMAP l.37: 11 A including 1 A fees, partial outcomes, late race | H1, H16, H6/H7 | K | Covered |

## 8. What this review does not claim

- It is not adopted MIL/4, a design vote, or an independent audit (the routing requires GPT-6 Astra and Grok 4.6 audits of frozen bytes).
- The K text was not kompiled or run. The traces are hand-derived expectations.
- Nothing here is a proof. Every property of the form "X is rejected" is by inspection of the rules, not a theorem.
- Nothing here was shown natively (ZKIR) or on Midnight Preview. In particular, head consumption, custody exclusivity, Poseidon binding and imported-verifier soundness remain open native obligations.
- I did not inspect other architects' answers.

---

The Vercel MCP server needs authorization before its tools can be used. Authorize it through `claude mcp` or `/mcp` in an interactive session. I didn't need it for this review.