// Finite first-profile cell/effect schemas for MIL/4. Importable proposal only.
// The master stage must authenticate pre-values, authority, evidence and effects.
module M4E-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX

  syntax M4ECell ::= m4eBalance(String, String, String) [symbol(m4eBalance)]
                   | m4eReserve(String, String, String) [symbol(m4eReserve)]
                   | m4ePoolHead(String, String) [symbol(m4ePoolHead)]
                   | m4eDebt(String, String) [symbol(m4eDebt)]
                   | m4eLock(String, String) [symbol(m4eLock)]
                   | m4eSupply(String, String) [symbol(m4eSupply)]
                   | m4eAggregate(String, String) [symbol(m4eAggregate)]
                   | m4eGrant(String, String) [symbol(m4eGrant)]
                   | m4eReplay(String, String) [symbol(m4eReplay)]
                   | m4eObservation(String, String) [symbol(m4eObservation)]
                   | m4ePolicy(String, String) [symbol(m4ePolicy)]
                   | m4eQueue(String, String) [symbol(m4eQueue)]
                   | m4eDuty(String, String) [symbol(m4eDuty)]
                   | m4eClaim(String, String) [symbol(m4eClaim)]
                   | m4eReceipt(String, String) [symbol(m4eReceipt)]
                   | m4eNullifier(String, String) [symbol(m4eNullifier)]
                   | m4eManaged(String, String) [symbol(m4eManaged)]
                   | m4eSurplus(String, String) [symbol(m4eSurplus)]
                   | m4eCustody(String, String, String) [symbol(m4eCustody)]
                   | m4eShareSupply(String, String) [symbol(m4eShareSupply)]
                   | m4eShareBalance(String, String, String) [symbol(m4eShareBalance)]
                   | m4eBudget(String, String) [symbol(m4eBudget)]
  syntax M4ECells ::= m4eCNil() [symbol(m4eCNil())]
                    | m4eC(M4ECell, M4ECells) [symbol(m4eC)]
  syntax M4EEffect ::= m4eAdd(M4ECell, Int) [symbol(m4eAdd)]
                     | m4eSet(M4ECell, String, String) [symbol(m4eSet)]
                     | m4eConsume(M4ECell) [symbol(m4eConsume)]
                     | m4eKeep(M4ECell) [symbol(m4eKeep)]
  syntax M4EEffects ::= m4eENil() [symbol(m4eENil())]
                      | m4eE(M4EEffect, M4EEffects) [symbol(m4eE)]

  // Each request names the exact operation whose read, write and effect lists
  // are checked. String IDs carry the domain/asset/owner identity, not authority.
  syntax M4EArgs ::= m4eAmm(String, String, String, String, String, Int, Int, String, String) [symbol(m4eAmm)]
                   | m4eRepay(String, String, String, String, String, Int, Int, Int) [symbol(m4eRepay)]
                   | m4eCdp(String, String, String, String, String, String, Int, Int) [symbol(m4eCdp)]
                   | m4eOption(String, String, String, String, String, Int, Int) [symbol(m4eOption)]
                   | m4eObs(String, String, String, String, String) [symbol(m4eObs)]
                   | m4eGov(String, String, String, String, String, String) [symbol(m4eGov)]
                   | m4eBridgeLock(String, String, String, String, String, Int) [symbol(m4eBridgeLock)]
                   | m4eBridgeReceive(String, String, String, String, String, Int, Int) [symbol(m4eBridgeReceive)]
                   | m4eBridgeRefund(String, String, String, String, Int) [symbol(m4eBridgeRefund)]
                   | m4eVault(String, String, String, String, String, Int, Int) [symbol(m4eVault)]
                   | m4eLater(String) [symbol(m4eLater)]
  syntax M4EPacket ::= m4ePacket(M4EArgs, M4ECells, M4ECells, M4EEffects) [symbol(m4ePacket)]
endmodule

module M4E-EFFECTS
  imports M4E-SYNTAX
  imports INT
  imports BOOL
  imports STRING

  syntax M4EResult ::= m4ePrepared(String, M4ECells, M4EEffects) [symbol(m4ePrepared)]
                     | m4eRejected(String, String) [symbol(m4eRejected)]
                     | m4eCheck(M4EPacket) [function]

  // Exact-input AMM, rational fee retained in pool. Four account lines and two
  // reserve lines must be present, together with budget, replay and head writes.
  rule m4eCheck(m4ePacket(
       m4eAmm(D, P, T, A, B, DX, DY, H0, H1),
       m4eC(m4ePoolHead(D,P), m4eC(m4eReserve(D,P,A), m4eC(m4eReserve(D,P,B),
       m4eC(m4eBalance(D,A,T), m4eC(m4eBalance(D,B,T),
       m4eC(m4eBalance(D,A,P), m4eC(m4eBalance(D,B,P),
       m4eC(m4eBudget(D,T), m4eC(m4eReplay(D,T), m4eCNil()))))))))),
       m4eC(m4ePoolHead(D,P), m4eC(m4eReserve(D,P,A), m4eC(m4eReserve(D,P,B),
       m4eC(m4eBalance(D,A,T), m4eC(m4eBalance(D,B,T),
       m4eC(m4eBalance(D,A,P), m4eC(m4eBalance(D,B,P),
       m4eC(m4eBudget(D,T), m4eC(m4eReplay(D,T), m4eCNil()))))))))),
       m4eE(m4eAdd(m4eBalance(D,A,T), NDXTR),
       m4eE(m4eAdd(m4eBalance(D,A,P), DX),
       m4eE(m4eAdd(m4eBalance(D,B,P), NDYPOOL),
       m4eE(m4eAdd(m4eBalance(D,B,T), DY),
       m4eE(m4eAdd(m4eReserve(D,P,A), DX),
       m4eE(m4eAdd(m4eReserve(D,P,B), NDYRES),
       m4eE(m4eAdd(m4eBudget(D,T), NDXBUD),
       m4eE(m4eConsume(m4eReplay(D,T)),
       m4eE(m4eSet(m4ePoolHead(D,P), H0, H1), m4eENil())))))))))))
       => m4ePrepared("amm-cp/1", m4eC(m4ePoolHead(D,P), m4eCNil()),
                      m4eE(m4eSet(m4ePoolHead(D,P), H0, H1), m4eENil()))
       requires D =/=String "" andBool A =/=String B andBool T =/=String P
           andBool DX >Int 0 andBool DY >Int 0 andBool H0 =/=String H1
           andBool NDXTR ==Int 0 -Int DX andBool NDYPOOL ==Int 0 -Int DY
           andBool NDYRES ==Int 0 -Int DY andBool NDXBUD ==Int 0 -Int DX

  // S0 repayment: gross debtor debit and creditor credit are paired with exact
  // AccrualFirst debt writes. Default/impairment never discharges the claim.
  rule m4eCheck(m4ePacket(
       m4eRepay(D, O, PAYER, CREDITOR, A, PRINCIPAL, ACCRUED, N),
       m4eC(m4eDebt(D,O), m4eC(m4eBalance(D,A,PAYER),
       m4eC(m4eBalance(D,A,CREDITOR), m4eC(m4eBudget(D,PAYER),
       m4eC(m4eReplay(D,O), m4eCNil()))))),
       m4eC(m4eDebt(D,O), m4eC(m4eBalance(D,A,PAYER),
       m4eC(m4eBalance(D,A,CREDITOR), m4eC(m4eBudget(D,PAYER),
       m4eC(m4eReplay(D,O), m4eCNil()))))),
       m4eE(m4eAdd(m4eBalance(D,A,PAYER), NDEBIT),
       m4eE(m4eAdd(m4eBalance(D,A,CREDITOR), N),
       m4eE(m4eAdd(m4eDebt(D,O), NDEBT),
       m4eE(m4eAdd(m4eBudget(D,PAYER), NBUD),
       m4eE(m4eConsume(m4eReplay(D,O)), m4eENil())))))))
       => m4ePrepared("loan-s0/1", m4eC(m4eDebt(D,O), m4eCNil()),
                      m4eE(m4eAdd(m4eDebt(D,O), 0 -Int N), m4eENil()))
       requires PAYER =/=String CREDITOR andBool N >Int 0
           andBool PRINCIPAL >=Int 0 andBool ACCRUED >=Int 0
           andBool N <=Int PRINCIPAL +Int ACCRUED
           andBool NDEBIT ==Int 0 -Int N andBool NDEBT ==Int 0 -Int N
           andBool NBUD ==Int 0 -Int N

  // CDP one-line mint: collateral custody and lock rise; debt, aggregate debt,
  // supply and holder balance rise in the same stage. Grant quota is consumed.
  rule m4eCheck(m4ePacket(
       m4eCdp(D, ID, OWNER, COLL, SYN, ISSUER, LOCK, MINT),
       m4eC(m4eBalance(D,COLL,OWNER), m4eC(m4eCustody(D,ID,COLL),
       m4eC(m4eLock(D,ID), m4eC(m4eDebt(D,ID),
       m4eC(m4eAggregate(D,SYN), m4eC(m4eSupply(D,SYN),
       m4eC(m4eBalance(D,SYN,OWNER), m4eC(m4eGrant(D,ISSUER),
       m4eC(m4eReplay(D,ID), m4eCNil()))))))))),
       m4eC(m4eBalance(D,COLL,OWNER), m4eC(m4eCustody(D,ID,COLL),
       m4eC(m4eLock(D,ID), m4eC(m4eDebt(D,ID),
       m4eC(m4eAggregate(D,SYN), m4eC(m4eSupply(D,SYN),
       m4eC(m4eBalance(D,SYN,OWNER), m4eC(m4eGrant(D,ISSUER),
       m4eC(m4eReplay(D,ID), m4eCNil()))))))))),
       m4eE(m4eAdd(m4eBalance(D,COLL,OWNER), NLOCK),
       m4eE(m4eAdd(m4eCustody(D,ID,COLL), LOCK),
       m4eE(m4eAdd(m4eLock(D,ID), LOCK),
       m4eE(m4eAdd(m4eDebt(D,ID), MINT),
       m4eE(m4eAdd(m4eAggregate(D,SYN), MINT),
       m4eE(m4eAdd(m4eSupply(D,SYN), MINT),
       m4eE(m4eAdd(m4eBalance(D,SYN,OWNER), MINT),
       m4eE(m4eAdd(m4eGrant(D,ISSUER), NGRANT),
       m4eE(m4eConsume(m4eReplay(D,ID)), m4eENil())))))))))))
       => m4ePrepared("cdp/1", m4eC(m4eDebt(D,ID), m4eCNil()),
                      m4eE(m4eAdd(m4eDebt(D,ID), MINT), m4eENil()))
       requires COLL =/=String SYN andBool LOCK >Int 0 andBool MINT >Int 0
           andBool NLOCK ==Int 0 -Int LOCK andBool NGRANT ==Int 0 -Int MINT

  // Option exercise of an already funded reserve; premium and creation have a
  // separate open policy. One-shot claim consumption survives partial payment.
  rule m4eCheck(m4ePacket(
       m4eOption(D, ID, HOLDER, WRITER, A, PAY, REM),
       m4eC(m4eCustody(D,ID,A), m4eC(m4eBalance(D,A,HOLDER),
       m4eC(m4eClaim(D,ID), m4eC(m4eReplay(D,ID), m4eCNil())))),
       m4eC(m4eCustody(D,ID,A), m4eC(m4eBalance(D,A,HOLDER),
       m4eC(m4eClaim(D,ID), m4eC(m4eReplay(D,ID), m4eCNil())))),
       m4eE(m4eAdd(m4eCustody(D,ID,A), NPAY),
       m4eE(m4eAdd(m4eBalance(D,A,HOLDER), PAY),
       m4eE(m4eConsume(m4eClaim(D,ID)),
       m4eE(m4eConsume(m4eReplay(D,ID)), m4eENil()))))))
       => m4ePrepared("opt-capped/1", m4eC(m4eCustody(D,ID,A), m4eCNil()),
                      m4eE(m4eKeep(m4eCustody(D,ID,A)), m4eENil()))
       requires HOLDER =/=String WRITER andBool PAY >=Int 0 andBool REM >=Int 0
           andBool NPAY ==Int 0 -Int PAY

  // Oracle admission changes only the selected observation and round marker.
  rule m4eCheck(m4ePacket(m4eObs(D, PAIR, OLD, NEW, ISSUER),
       m4eC(m4eObservation(D,PAIR), m4eC(m4ePolicy(D,PAIR),
       m4eC(m4eGrant(D,ISSUER), m4eC(m4eReplay(D,PAIR), m4eCNil())))),
       m4eC(m4eObservation(D,PAIR), m4eC(m4eReplay(D,PAIR), m4eCNil())),
       m4eE(m4eSet(m4eObservation(D,PAIR), OLD, NEW),
       m4eE(m4eConsume(m4eReplay(D,PAIR)), m4eENil()))))
       => m4ePrepared("obs/1", m4eC(m4eObservation(D,PAIR), m4eCNil()),
                      m4eE(m4eSet(m4eObservation(D,PAIR), OLD, NEW), m4eENil()))
       requires OLD =/=String NEW andBool ISSUER =/=String ""

  // Governance execute under current head: queue and approvals are consumed;
  // existing duty is read and retained. Queue/cancel are distinct later rules.
  rule m4eCheck(m4ePacket(m4eGov(D, ID, OLD, NEW, QUEUE, DUTY),
       m4eC(m4ePolicy(D,ID), m4eC(m4eQueue(D,QUEUE),
       m4eC(m4eGrant(D,ID), m4eC(m4eDuty(D,DUTY),
       m4eC(m4eReplay(D,QUEUE), m4eCNil()))))),
       m4eC(m4ePolicy(D,ID), m4eC(m4eQueue(D,QUEUE),
       m4eC(m4eReplay(D,QUEUE), m4eCNil()))),
       m4eE(m4eSet(m4ePolicy(D,ID), OLD, NEW),
       m4eE(m4eConsume(m4eQueue(D,QUEUE)),
       m4eE(m4eConsume(m4eReplay(D,QUEUE)),
       m4eE(m4eKeep(m4eDuty(D,DUTY)), m4eENil()))))))
       => m4ePrepared("gov/1", m4eC(m4ePolicy(D,ID), m4eCNil()),
                      m4eE(m4eKeep(m4eDuty(D,DUTY)), m4eENil()))
       requires OLD =/=String NEW andBool DUTY =/=String ""

  // Bridge source lock. Only source-domain custody changes; destination effects
  // require a later verified receipt, never a timeout observation alone.
  rule m4eCheck(m4ePacket(m4eBridgeLock(SRC, DST, CLAIM, OWNER, A, Q),
       m4eC(m4eBalance(SRC,A,OWNER), m4eC(m4eCustody(SRC,CLAIM,A),
       m4eC(m4eClaim(SRC,CLAIM), m4eC(m4eReplay(SRC,CLAIM), m4eCNil())))),
       m4eC(m4eBalance(SRC,A,OWNER), m4eC(m4eCustody(SRC,CLAIM,A),
       m4eC(m4eClaim(SRC,CLAIM), m4eC(m4eReplay(SRC,CLAIM), m4eCNil())))),
       m4eE(m4eAdd(m4eBalance(SRC,A,OWNER), NQ),
       m4eE(m4eAdd(m4eCustody(SRC,CLAIM,A), Q),
       m4eE(m4eAdd(m4eClaim(SRC,CLAIM), Q),
       m4eE(m4eConsume(m4eReplay(SRC,CLAIM)), m4eENil()))))))
       => m4ePrepared("bridge-pair/1", m4eC(m4eClaim(SRC,CLAIM), m4eCNil()),
                      m4eE(m4eKeep(m4eClaim(SRC,CLAIM)), m4eENil()))
       requires SRC =/=String DST andBool Q >Int 0 andBool NQ ==Int 0 -Int Q

  // One destination installment. FEE stays an outstanding source entitlement
  // pending the versioned fee-beneficiary decision; no implicit fee transfer.
  rule m4eCheck(m4ePacket(m4eBridgeReceive(DST, CLAIM, OWNER, A, MSG, CREDIT, FEE),
       m4eC(m4eClaim(DST,CLAIM), m4eC(m4eBalance(DST,A,OWNER),
       m4eC(m4eReceipt(DST,MSG), m4eC(m4eNullifier(DST,MSG), m4eCNil())))),
       m4eC(m4eClaim(DST,CLAIM), m4eC(m4eBalance(DST,A,OWNER),
       m4eC(m4eReceipt(DST,MSG), m4eC(m4eNullifier(DST,MSG), m4eCNil())))),
       m4eE(m4eAdd(m4eClaim(DST,CLAIM), NCREDIT),
       m4eE(m4eAdd(m4eBalance(DST,A,OWNER), CREDIT),
       m4eE(m4eSet(m4eReceipt(DST,MSG), "absent", "delivered"),
       m4eE(m4eConsume(m4eNullifier(DST,MSG)), m4eENil()))))))
       => m4ePrepared("bridge-pair/1", m4eC(m4eClaim(DST,CLAIM), m4eCNil()),
                      m4eE(m4eKeep(m4eClaim(DST,CLAIM)), m4eENil()))
       requires CREDIT >Int 0 andBool FEE >=Int 0
           andBool NCREDIT ==Int 0 -Int CREDIT

  // Refund is intentionally unsupported until terminal foreign nonreceipt and
  // a source/destination claim correspondence are made exact. A timeout cannot
  // become a balance effect through this module.
  rule m4eCheck(m4ePacket(m4eBridgeRefund(_, _, _, _, _), _, _, _))
       => m4eRejected("bridge-pair/1", "TERMINAL_NONRECEIPT_POLICY_OPEN")

  // One-asset vault deposit. Share issue and custody are explicit. The product
  // remainder is metadata, never a free-standing asset or share effect.
  rule m4eCheck(m4ePacket(m4eVault(D, P, OWNER, A, CLASS, DEPOSIT, SHARES),
       m4eC(m4eBalance(D,A,OWNER), m4eC(m4eCustody(D,P,A),
       m4eC(m4eManaged(D,P), m4eC(m4eSurplus(D,P),
       m4eC(m4eShareSupply(D,CLASS), m4eC(m4eShareBalance(D,CLASS,OWNER),
       m4eC(m4eGrant(D,CLASS), m4eC(m4eReplay(D,P), m4eCNil())))))))),
       m4eC(m4eBalance(D,A,OWNER), m4eC(m4eCustody(D,P,A),
       m4eC(m4eManaged(D,P), m4eC(m4eShareSupply(D,CLASS),
       m4eC(m4eShareBalance(D,CLASS,OWNER), m4eC(m4eGrant(D,CLASS),
       m4eC(m4eReplay(D,P), m4eCNil()))))))),
       m4eE(m4eAdd(m4eBalance(D,A,OWNER), NDEP),
       m4eE(m4eAdd(m4eCustody(D,P,A), DEPOSIT),
       m4eE(m4eAdd(m4eManaged(D,P), DEPOSIT),
       m4eE(m4eAdd(m4eShareSupply(D,CLASS), SHARES),
       m4eE(m4eAdd(m4eShareBalance(D,CLASS,OWNER), SHARES),
       m4eE(m4eAdd(m4eGrant(D,CLASS), NSHARES),
       m4eE(m4eConsume(m4eReplay(D,P)), m4eENil())))))))))
       => m4ePrepared("vault/1", m4eC(m4eManaged(D,P), m4eCNil()),
                      m4eE(m4eAdd(m4eManaged(D,P), DEPOSIT), m4eENil()))
       requires DEPOSIT >Int 0 andBool SHARES >Int 0
           andBool NDEP ==Int 0 -Int DEPOSIT andBool NSHARES ==Int 0 -Int SHARES

  // The exact list patterns above reject missing, reordered or extra entries.
  // Each later family is closed until a separately admitted profile is supplied.
  rule m4eCheck(m4ePacket(m4eLater(F), _, _, _))
       => m4eRejected(F, "LATER_PROFILE_UNSUPPORTED")
  rule m4eCheck(m4ePacket(A, _, _, _))
       => m4eRejected(m4eName(A), "CELL_OR_EFFECT_SCHEMA") [owise]

  syntax String ::= m4eName(M4EArgs) [function]
  rule m4eName(m4eAmm(_,_,_,_,_,_,_,_,_)) => "amm-cp/1"
  rule m4eName(m4eRepay(_,_,_,_,_,_,_,_)) => "loan-s0/1"
  rule m4eName(m4eCdp(_,_,_,_,_,_,_,_)) => "cdp/1"
  rule m4eName(m4eOption(_,_,_,_,_,_,_)) => "opt-capped/1"
  rule m4eName(m4eObs(_,_,_,_,_)) => "obs/1"
  rule m4eName(m4eGov(_,_,_,_,_,_)) => "gov/1"
  rule m4eName(m4eBridgeLock(_,_,_,_,_,_)) => "bridge-pair/1"
  rule m4eName(m4eBridgeReceive(_,_,_,_,_,_,_)) => "bridge-pair/1"
  rule m4eName(m4eBridgeRefund(_,_,_,_,_)) => "bridge-pair/1"
  rule m4eName(m4eVault(_,_,_,_,_,_,_)) => "vault/1"
  rule m4eName(m4eLater(F)) => F
endmodule
