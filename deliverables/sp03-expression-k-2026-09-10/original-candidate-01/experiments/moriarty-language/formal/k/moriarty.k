// Bounded Transfer-only or two-action funded repayment projection. No source/Core/proof claim.
module MORIARTY-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  syntax Balance ::= balance(String, String, Int) [symbol(balance)]
  syntax Allowance ::= allowance(String, String, Int, Int) [symbol(allowance)]
  syntax Conversion ::= conversion(Int, Int, String) [symbol(conversion)]
  syntax Obligation ::= obligation(String, String, String, String, String, Int, Int, Int, String, String, Conversion) [symbol(obligation)]
  syntax Work ::= work(Int, Int, Int) [symbol(work)]
  syntax Transfer ::= transfer(String, String, String, String, Int) [symbol(transfer)]
  syntax Repay ::= repay(String, String, String, String, Int) [symbol(repay)]
  syntax Packet ::= packet(Balance, Balance, Allowance, Obligation, Work, Transfer, Repay, String) [symbol(packet)]
                  | transferPacket(Balance, Balance, Allowance, Obligation, Work, Transfer, String) [symbol(transferPacket)]
endmodule

module MORIARTY
  imports MORIARTY-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  syntax Result ::= "pending" [symbol(pending)]
                  | rejected(String, String, Int) [symbol(rejected)]
                  | prepared(String, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, String, Int, Int, Int, Int) [symbol(prepared)]
                  | preparedTransfer(String, Int, Int, Int, Int, Int, Int, Int, Int) [symbol(preparedTransfer)]
  syntax KItem ::= inspect(Packet, Int, Int) | finish(Packet, Int, Int)
                 | ensure(String, Bool, String, Int)
                 | convertRepay(Packet, Int, Int) | divideRepay(Packet, Int, Int, Int)
                 | roundRepay(Packet, Int, Int, Int, Int) | fundRepay(Packet, Int, Int, Int)
                 | allocateRepay(Packet, Int, Int, Int) | splitRepay(Packet, Int, Int, Int)
                 | finishRepay(Packet, Int, Int, Int, Int)
  configuration <k> $PGM:Packet </k> <out> pending </out>
  syntax Int ::= maxU() [function]
  rule maxU() => 340282366920938463463374607431768211455
  syntax Int ::= indexOf(String, String, Balance, Balance) [function]
  rule indexOf(P, A, balance(P0, A0, _), _) => 0
    requires P ==String P0 andBool A ==String A0
  rule indexOf(P, A, balance(P0, A0, _), balance(P1, A1, _)) => 1
    requires notBool (P ==String P0 andBool A ==String A0) andBool P ==String P1 andBool A ==String A1
  rule indexOf(P, A, balance(P0, A0, _), balance(P1, A1, _)) => -1
    requires notBool (P ==String P0 andBool A ==String A0) andBool notBool (P ==String P1 andBool A ==String A1)
  syntax Int ::= at(Int, Int, Int) [function]
  rule at(0, B0, _) => B0
  rule at(1, _, B1) => B1
  rule at(-1, _, _) => 0
  syntax Int ::= moved(Int, Int, Int, Int, Int) [function]
  rule moved(ROW, SI, _, B, T) => B -Int T requires ROW ==Int SI
  rule moved(ROW, SI, RI, B, T) => B +Int T requires ROW =/=Int SI andBool ROW ==Int RI
  rule moved(ROW, SI, RI, B, _) => B requires ROW =/=Int SI andBool ROW =/=Int RI
  syntax Int ::= appended(Int, Int) [function]
  rule appended(-1, T) => T
  rule appended(RI, _) => 0 requires RI >=Int 0
  syntax Int ::= principalPart(String, Int, Int, Int) [function]
  rule principalPart("AccrualFirst", N, _, I) => N -Int minInt(N, I)
  rule principalPart("PrincipalFirst", N, P, _) => minInt(N, P)
  rule principalPart("ProRata", N, P, I) => (N *Int P) /Int (P +Int I)
    requires P +Int I >Int 0 andBool N *Int P <=Int maxU()
  syntax Int ::= rounded(String, Int, Int) [function]
  rule rounded("none", Q, _) => Q
  rule rounded("floor", Q, _) => Q
  rule rounded("ceil", Q, 0) => Q
  rule rounded("ceil", Q, R) => Q +Int 1 requires R >Int 0
  syntax String ::= statusOf(Int) [function]
  rule statusOf(0) => "Settled"
  rule statusOf(O) => "Outstanding" requires O >Int 0

  rule <k> transferPacket(B0, B1, A, O, W, transfer(TID, FROM, TO, TA, T), H)
        => inspect(transferPacket(B0, B1, A, O, W, transfer(TID, FROM, TO, TA, T), H),
                   indexOf(FROM, TA, B0, B1), indexOf(TO, TA, B0, B1)) </k>
  rule <k> packet(B0, B1, A, O, W, transfer(TID, FROM, TO, TA, T), R, H)
        => inspect(packet(B0, B1, A, O, W, transfer(TID, FROM, TO, TA, T), R, H),
                   indexOf(FROM, TA, B0, B1), indexOf(TO, TA, B0, B1)) </k>
  rule <k> ensure(_, true, _, _) => .K ... </k>
  rule <k> ensure(H, false, CODE, IDX) ~> _REST:K => .K </k>
       <out> pending => rejected(H, CODE, IDX) </out>

  rule <k> inspect(transferPacket(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), H), SI, RI)
        => ensure(H, (notBool (BP0 ==String BP1 andBool BA0 ==String BA1)), "DUPLICATE", -1)
        ~> ensure(H, (AR +Int AS <=Int maxU()), "INVARIANT", -1)
        ~> ensure(H, (M >Int 0 andBool SCALE <=Int 18), "INVARIANT", -1)
        ~> ensure(H, ((P +Int I <=Int maxU()) andBool (P +Int I ==Int O) andBool (((STATUS ==String "Outstanding") andBool (O >Int 0)) orBool ((STATUS ==String "Settled") andBool (O ==Int 0)))), "INVARIANT", -1)
        ~> ensure(H, (WR +Int WS +Int RES <=Int maxU()), "INVARIANT", -1)
        ~> ensure(H, (WR >=Int 1), "INSUFFICIENT_WORK", -1)
        ~> ensure(H, (WS +Int 1 <=Int maxU()), "OVERFLOW", -1)
        ~> ensure(H, (T >Int 0), "ZERO_AMOUNT", 0)
        ~> ensure(H, (FROM =/=String TO), "SELF_TRANSFER", 0)
        ~> ensure(H, (SI >=Int 0), "MISSING_BALANCE", 0)
        ~> ensure(H, (at(SI, B0, B1) >=Int T), "INSUFFICIENT_BALANCE", 0)
        ~> ensure(H, (AP ==String FROM andBool AA ==String TA), "MISSING_ALLOWANCE", 0)
        ~> ensure(H, (AR >=Int T), "INSUFFICIENT_ALLOWANCE", 0)
        ~> ensure(H, (AS +Int T <=Int maxU()), "OVERFLOW", 0)
        ~> ensure(H, (at(RI, B0, B1) +Int T <=Int maxU()), "OVERFLOW", 0)
        ~> finish(transferPacket(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), H), SI, RI) </k>

  rule <k> inspect(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI)
        => ensure(H, (notBool (BP0 ==String BP1 andBool BA0 ==String BA1)), "DUPLICATE", -1)
        ~> ensure(H, (AR +Int AS <=Int maxU()), "INVARIANT", -1)
        ~> ensure(H, (M >Int 0 andBool SCALE <=Int 18), "INVARIANT", -1)
        ~> ensure(H, ((P +Int I <=Int maxU()) andBool (P +Int I ==Int O) andBool (((STATUS ==String "Outstanding") andBool (O >Int 0)) orBool ((STATUS ==String "Settled") andBool (O ==Int 0)))), "INVARIANT", -1)
        ~> ensure(H, (WR +Int WS +Int RES <=Int maxU()), "INVARIANT", -1)
        ~> ensure(H, (WR >=Int 2), "INSUFFICIENT_WORK", -1)
        ~> ensure(H, (WS +Int 2 <=Int maxU()), "OVERFLOW", -1)
        ~> ensure(H, (T >Int 0), "ZERO_AMOUNT", 0)
        ~> ensure(H, (FROM =/=String TO), "SELF_TRANSFER", 0)
        ~> ensure(H, (SI >=Int 0), "MISSING_BALANCE", 0)
        ~> ensure(H, (at(SI, B0, B1) >=Int T), "INSUFFICIENT_BALANCE", 0)
        ~> ensure(H, (AP ==String FROM andBool AA ==String TA), "MISSING_ALLOWANCE", 0)
        ~> ensure(H, (AR >=Int T), "INSUFFICIENT_ALLOWANCE", 0)
        ~> ensure(H, (AS +Int T <=Int maxU()), "OVERFLOW", 0)
        ~> ensure(H, (at(RI, B0, B1) +Int T <=Int maxU()), "OVERFLOW", 0)
        ~> ensure(H, (N >Int 0), "ZERO_AMOUNT", 1)
        ~> ensure(H, (ROID ==String OID), "MISSING_OBLIGATION", 1)
        ~> ensure(H, (STATUS ==String "Outstanding" andBool O >Int 0), "NOT_OUTSTANDING", 1)
        ~> ensure(H, (N <=Int O), "EXCEEDS_OUTSTANDING", 1)
        ~> ensure(H, (RTID ==String TID), "TRANSFER_NOT_IN_STEP", 1)
        ~> ensure(H, (FROM ==String PAYER andBool TO ==String CREDITOR andBool TA ==String OA), "TRANSFER_MISMATCH", 1)
        ~> convertRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI) </k>

  rule <k> finish(transferPacket(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), H), SI, RI) => .K </k>
       <out> pending => preparedTransfer(H, moved(0, SI, RI, B0, T), moved(1, SI, RI, B1, T), appended(RI, T),
           AR -Int T, AS +Int T, WR -Int 1, WS +Int 1, RI) </out>


  rule <k> convertRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI)
        => ensure(H, (N *Int M <=Int maxU()), "OVERFLOW", 1)
        ~> divideRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, N *Int M) </k>

  rule <k> divideRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, PRODUCT)
        => roundRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, PRODUCT /Int (10 ^Int SCALE), PRODUCT %Int (10 ^Int SCALE)) </k>

  rule <k> roundRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, Q, REM)
        => ensure(H, (ROUND =/=String "none" orBool REM ==Int 0), "INEXACT_CONVERSION", 1)
        ~> fundRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, rounded(ROUND, Q, REM)) </k>

  rule <k> fundRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, CASH)
        => ensure(H, (CASH <=Int maxU()), "OVERFLOW", 1)
        ~> ensure(H, (CASH >Int 0), "DUST", 1)
        ~> ensure(H, (CASH <=Int T), "INSUFFICIENT_UNALLOCATED", 1)
        ~> allocateRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, CASH) </k>

  rule <k> allocateRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, CASH)
        => ensure(H, (RULE =/=String "ProRata" orBool N *Int P <=Int maxU()), "OVERFLOW", 1)
        ~> splitRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, CASH) </k>

  rule <k> splitRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, CASH)
        => ensure(H, (principalPart(RULE, N, P, I) <=Int P andBool N -Int principalPart(RULE, N, P, I) <=Int I), "ALLOCATION_COMPONENT", 1)
        ~> finishRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, CASH, principalPart(RULE, N, P, I)) </k>

  rule <k> finishRepay(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS, conversion(M, SCALE, ROUND)), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI, CASH, DP) => .K </k>
       <out> pending => prepared(H, moved(0, SI, RI, B0, T), moved(1, SI, RI, B1, T), appended(RI, T),
           AR -Int T, AS +Int T, WR -Int 2, WS +Int 2, P -Int DP,
           I -Int (N -Int DP), O -Int N, statusOf(O -Int N), CASH, DP, N -Int DP, RI) </out>
endmodule
