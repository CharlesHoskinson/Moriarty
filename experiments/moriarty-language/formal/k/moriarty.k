// Bounded two-action funded repayment projection. No source/Core/proof claim.
module MORIARTY-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  syntax Balance ::= balance(String, String, Int) [symbol(balance)]
  syntax Allowance ::= allowance(String, String, Int, Int) [symbol(allowance)]
  syntax Obligation ::= obligation(String, String, String, String, String, Int, Int, Int, String, String) [symbol(obligation)]
  syntax Work ::= work(Int, Int, Int) [symbol(work)]
  syntax Transfer ::= transfer(String, String, String, String, Int) [symbol(transfer)]
  syntax Repay ::= repay(String, String, String, String, Int) [symbol(repay)]
  syntax Packet ::= packet(Balance, Balance, Allowance, Obligation, Work, Transfer, Repay, String) [symbol(packet)]
endmodule

module MORIARTY
  imports MORIARTY-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  syntax Result ::= "pending" [symbol(pending)]
                  | rejected(String, String, Int) [symbol(rejected)]
                  | prepared(String, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, String, Int, Int, Int, Int) [symbol(prepared)]
  syntax KItem ::= inspect(Packet, Int, Int) | finish(Packet, Int, Int)
                 | ensure(String, Bool, String, Int)
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
  syntax String ::= statusOf(Int) [function]
  rule statusOf(0) => "Settled"
  rule statusOf(O) => "Outstanding" requires O >Int 0

  rule <k> packet(B0, B1, A, O, W, transfer(TID, FROM, TO, TA, T), R, H)
        => inspect(packet(B0, B1, A, O, W, transfer(TID, FROM, TO, TA, T), R, H),
                   indexOf(FROM, TA, B0, B1), indexOf(TO, TA, B0, B1)) </k>
  rule <k> ensure(_, true, _, _) => .K ... </k>
  rule <k> ensure(H, false, CODE, IDX) ~> _REST:K => .K </k>
       <out> pending => rejected(H, CODE, IDX) </out>

  rule <k> inspect(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI)
        => ensure(H, (notBool (BP0 ==String BP1 andBool BA0 ==String BA1)), "DUPLICATE", -1)
        ~> ensure(H, (AR +Int AS <=Int maxU()), "INVARIANT", -1)
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
        ~> ensure(H, (N <=Int T), "INSUFFICIENT_UNALLOCATED", 1)
        ~> finish(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI) </k>

  rule <k> finish(packet(balance(BP0, BA0, B0), balance(BP1, BA1, B1), allowance(AP, AA, AR, AS), obligation(OID, DEBTOR, CREDITOR, DENOM, OA, P, I, O, RULE, STATUS), work(WR, WS, RES), transfer(TID, FROM, TO, TA, T), repay(AID, RTID, ROID, PAYER, N), H), SI, RI) => .K </k>
       <out> pending => prepared(H, moved(0, SI, RI, B0, T), moved(1, SI, RI, B1, T), appended(RI, T),
           AR -Int T, AS +Int T, WR -Int 2, WS +Int 2, P -Int principalPart(RULE, N, P, I),
           I -Int (N -Int principalPart(RULE, N, P, I)), O -Int N, statusOf(O -Int N), N,
           principalPart(RULE, N, P, I), N -Int principalPart(RULE, N, P, I), RI) </out>
endmodule
