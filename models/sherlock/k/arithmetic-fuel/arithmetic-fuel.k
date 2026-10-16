requires "domains.md"

module P4-ARITHMETIC-FUEL-SYNTAX
  imports DOMAINS-SYNTAX

  // Byte vectors and diagnostic carry vectors share a structural list type.
  // Domain predicates below, not the grammar alone, impose their exact ranges.
  syntax AFSeq ::= afNil() | afCons(Int, AFSeq)
  syntax AFChain ::= afChain(AFSeq, AFSeq)
  syntax AFOutcome ::= afUIntResult(AFSeq) | afBoolResult(Bool) | afErrorResult(Int)
  // outcome, ADD.r/c[0..16], SUB.r/bo[0..16], MUL.r/h/c[0..32], hSum, hZero
  // Endpoint-inclusive diagnostic projection, NOT aux/2 serialized columns.
  syntax AFArith ::= afArith(AFOutcome, AFSeq, AFSeq, AFSeq, AFSeq,
                             AFSeq, AFSeq, AFSeq, Int, Int)
  // Opaque List/Tree payload tokens are preserved only by terminal publication;
  // token equality is not semantic value equality. UInt has all sixteen bytes.
  syntax AFValue ::= afUInt(AFSeq) | afBool(Bool) | afOpaque(Int, Int)
  // Every current-A control tag is represented. Nonterminal payloads are erased.
  syntax AFControl ::= afEval() | afReturn(AFValue) | afPush() | afInvoke()
                     | afEnter() | afBound() | afCases() | afCaseResult()
                     | afPrimitiveResult() | afError(Int)
  syntax AFRegime ::= afNoEvent() | afDone(AFValue) | afRejected(Int)
                    | afExhausted() | afCharged()
  // regime, charge, post.remainingFuel, preserved initialFuel, z[0..15],
  // zp[0..16], bw[0..16]. Endpoint 0 is diagnostic, not a new wire lane.
  syntax AFFuel ::= afFuel(AFRegime, Int, AFSeq, AFSeq, AFSeq, AFSeq, AFSeq)
  // op tag (0/1/2/4), left type, sixteen left bytes, right type, sixteen right bytes
  syntax AFRequest ::= afArithmeticRequest(Int, Int, AFSeq, Int, AFSeq)
                     // outcome tag, control length, control view, depth, fuel, initialFuel
                     | afFuelRequest(Int, Int, AFControl, Int, AFSeq, AFSeq)
  syntax KItem ::= AFRequest | AFArith | AFFuel
endmodule

module P4-ARITHMETIC-FUEL-FUNCTIONS
  imports P4-ARITHMETIC-FUEL-SYNTAX
  imports DOMAINS

  syntax Int ::= afLength(AFSeq) [function]
               | afBit(Bool) [function]
               | afLast(AFSeq) [function]
               | afSum(AFSeq) [function]
  rule afLength(afNil()) => 0
  rule afLength(afCons(_, T)) => 1 +Int afLength(T)
  rule afBit(true) => 1
  rule afBit(false) => 0
  rule afLast(afCons(X, afNil())) => X
  rule afLast(afCons(_, afCons(H, T))) => afLast(afCons(H, T))
  rule afSum(afNil()) => 0
  rule afSum(afCons(H, T)) => H +Int afSum(T)

  syntax AFSeq ::= afZeros(Int) [function]
                 | afTake(AFSeq, Int) [function]
                 | afDrop(AFSeq, Int) [function]
  rule afZeros(0) => afNil()
  rule afZeros(N) => afCons(0, afZeros(N -Int 1)) requires N >Int 0
  rule afTake(_, 0) => afNil()
  rule afTake(afCons(H, T), N) => afCons(H, afTake(T, N -Int 1)) requires N >Int 0
  rule afDrop(T, 0) => T
  rule afDrop(afCons(_, T), N) => afDrop(T, N -Int 1) requires N >Int 0

  syntax Bool ::= afBytes(AFSeq) [function]
                | afWord(AFSeq) [function]
  rule afBytes(afNil()) => true
  rule afBytes(afCons(H, T)) => H >=Int 0 andBool H <Int 256 andBool afBytes(T)
  rule afWord(S) => afLength(S) ==Int 16 andBool afBytes(S)

  // Explicit Euclidean division helpers on nonnegative integers. These use
  // only locally grounded Int +,-,<,>= syntax; no external helper is assumed.
  // This reference definition has unmeasured K evaluation cost (not a native op).
  syntax Int ::= afQuot256(Int) [function] | afRem256(Int) [function]
  rule afQuot256(N) => 0 requires N >=Int 0 andBool N <Int 256
  rule afQuot256(N) => 1 +Int afQuot256(N -Int 256) requires N >=Int 256
  rule afRem256(N) => N requires N >=Int 0 andBool N <Int 256
  rule afRem256(N) => afRem256(N -Int 256) requires N >=Int 256

  syntax AFChain ::= afPrepend(Int, Int, AFChain) [function]
                   | afAdd(AFSeq, AFSeq, Int) [function]
                   | afSub(AFSeq, AFSeq, Int) [function]
  rule afPrepend(R, C, afChain(Rs, Cs)) => afChain(afCons(R, Rs), afCons(C, Cs))
  rule afAdd(afNil(), afNil(), C) => afChain(afNil(), afCons(C, afNil()))
  rule afAdd(afCons(A, As), afCons(B, Bs), C)
    => afPrepend(afRem256(A +Int B +Int C), C,
         afAdd(As, Bs, afQuot256(A +Int B +Int C)))

  // Subtraction's signed local difference is in [-256,255] on the domain.
  // Add 256 exactly when it is negative: no signed remainder convention used.
  syntax Int ::= afSubDigit(Int) [function]
  rule afSubDigit(T) => T requires T >=Int 0
  rule afSubDigit(T) => T +Int 256 requires T <Int 0
  rule afSub(afNil(), afNil(), B) => afChain(afNil(), afCons(B, afNil()))
  rule afSub(afCons(A, As), afCons(B, Bs), C)
    => afPrepend(afSubDigit(A -Int B -Int C), C,
         afSub(As, Bs, afBit(A -Int B -Int C <Int 0)))

  syntax Int ::= afAt(AFSeq, Int) [function]
               | afByteOrZero(AFSeq, Int) [function]
               | afColumn(AFSeq, AFSeq, Int, Int) [function]
  rule afAt(afCons(H, _), 0) => H
  rule afAt(afCons(_, T), I) => afAt(T, I -Int 1) requires I >Int 0
  rule afByteOrZero(_, I) => 0 requires I <Int 0 orBool I >=Int 16
  rule afByteOrZero(B, I) => afAt(B, I) requires I >=Int 0 andBool I <Int 16
  // Sum all i=0..15 with j=k-i in 0..15. In particular S_31=0.
  rule afColumn(afNil(), _, _, _) => 0
  rule afColumn(afCons(A, As), B, K, I)
    => A *Int afByteOrZero(B, K -Int I) +Int afColumn(As, B, K, I +Int 1)

  syntax AFChain ::= afMul(AFSeq, AFSeq, Int, Int) [function]
  rule afMul(_, _, 32, C) => afChain(afNil(), afCons(C, afNil()))
  rule afMul(A, B, K, C)
    => afPrepend(afRem256(afColumn(A, B, K, 0) +Int C), C,
         afMul(A, B, K +Int 1, afQuot256(afColumn(A, B, K, 0) +Int C)))
    requires K >=Int 0 andBool K <Int 32

  syntax AFOutcome ::= afChecked(AFSeq, Int) [function]
  rule afChecked(R, 0) => afUIntResult(R)
  rule afChecked(_, C) => afErrorResult(10) requires C >Int 0

  syntax AFArith ::= afBlank(AFOutcome) [function]
                   | afAddOutput(AFChain) [function]
                   | afSubOutput(AFChain) [function]
                   | afLtOutput(AFChain) [function]
                   | afMulOutput(AFChain) [function]
  rule afBlank(O) => afArith(O, afZeros(16), afZeros(17), afZeros(16), afZeros(17),
                            afZeros(16), afZeros(16), afZeros(33), 0, 0)
  rule afAddOutput(afChain(R, C))
    => afArith(afChecked(R, afLast(C)), R, C, afZeros(16), afZeros(17),
               afZeros(16), afZeros(16), afZeros(33), 0, 0)
  rule afSubOutput(afChain(R, B))
    => afArith(afChecked(R, afLast(B)), afZeros(16), afZeros(17), R, B,
               afZeros(16), afZeros(16), afZeros(33), 0, 0)
  rule afLtOutput(afChain(R, B))
    => afArith(afBoolResult(afLast(B) ==Int 1), afZeros(16), afZeros(17), R, B,
               afZeros(16), afZeros(16), afZeros(33), 0, 0)
  rule afMulOutput(afChain(P, C))
    => afArith(afChecked(afTake(P,16), afSum(afDrop(P,16))),
               afZeros(16), afZeros(17), afZeros(16), afZeros(17),
               afTake(P,16), afDrop(P,16), C,
               afSum(afDrop(P,16)), afBit(afSum(afDrop(P,16)) ==Int 0))

  // Operand bytes/type IDs are an already-fetched semantic view. Fetch, enabled
  // dispatch, sort consistency and all StateBlock append/masks are NOT modeled.
  syntax Bool ::= afArithDomain(Int, Int, AFSeq, Int, AFSeq) [function]
  rule afArithDomain(Op, L, A, R, B)
    => (Op ==Int 0 orBool Op ==Int 1 orBool Op ==Int 2 orBool Op ==Int 4)
       andBool L >=Int 0 andBool L <=Int 3 andBool R >=Int 0 andBool R <=Int 3
       andBool afWord(A) andBool afWord(B)
  syntax AFArith ::= afArithmetic(Int, Int, AFSeq, Int, AFSeq) [function]
  rule afArithmetic(0, 0, A, 0, B) => afAddOutput(afAdd(A,B,0))
    requires afWord(A) andBool afWord(B)
  rule afArithmetic(1, 0, A, 0, B) => afSubOutput(afSub(A,B,0))
    requires afWord(A) andBool afWord(B)
  rule afArithmetic(2, 0, A, 0, B) => afMulOutput(afMul(A,B,0,0))
    requires afWord(A) andBool afWord(B)
  rule afArithmetic(4, 0, A, 0, B) => afLtOutput(afSub(A,B,0))
    requires afWord(A) andBool afWord(B)
  rule afArithmetic(Op, L, A, R, B) => afBlank(afErrorResult(11))
    requires afArithDomain(Op,L,A,R,B) andBool (L =/=Int 0 orBool R =/=Int 0)

  syntax Bool ::= afValueDomain(AFValue) [function]
                | afControlDomain(AFControl) [function]
                | afTerminal(AFControl, Int) [function]
  rule afValueDomain(afUInt(V)) => afWord(V)
  rule afValueDomain(afBool(_)) => true
  rule afValueDomain(afOpaque(T, I))
    => (T ==Int 2 orBool T ==Int 3) andBool I >=Int 0
  rule afControlDomain(afReturn(V)) => afValueDomain(V)
  rule afControlDomain(afError(E)) => E >=Int 0 andBool E <Int 256
  rule afControlDomain(afEval()) => true
  rule afControlDomain(afPush()) => true
  rule afControlDomain(afInvoke()) => true
  rule afControlDomain(afEnter()) => true
  rule afControlDomain(afBound()) => true
  rule afControlDomain(afCases()) => true
  rule afControlDomain(afCaseResult()) => true
  rule afControlDomain(afPrimitiveResult()) => true
  rule afTerminal(afReturn(_), D) => D ==Int 0
  rule afTerminal(afError(_), _) => true
  rule afTerminal(afEval(), _) => false
  rule afTerminal(afPush(), _) => false
  rule afTerminal(afInvoke(), _) => false
  rule afTerminal(afEnter(), _) => false
  rule afTerminal(afBound(), _) => false
  rule afTerminal(afCases(), _) => false
  rule afTerminal(afCaseResult(), _) => false
  rule afTerminal(afPrimitiveResult(), _) => false

  syntax Int ::= afEvent(Int, Int) [function]
  rule afEvent(O,N) => afBit(O ==Int 0 andBool N >Int 0)
  syntax AFSeq ::= afProbe(Int, AFSeq) [function]
                 | afPrefix(Int, AFSeq) [function]
                 | afFuelBytes(AFSeq, AFSeq) [function]
  rule afProbe(_, afNil()) => afNil()
  rule afProbe(E, afCons(F, Fs)) => afCons(E *Int afBit(F ==Int 0), afProbe(E,Fs))
  rule afPrefix(B, afNil()) => afCons(B, afNil())
  rule afPrefix(B, afCons(Z, Zs)) => afCons(B, afPrefix(B *Int Z, Zs))
  rule afFuelBytes(afNil(), afCons(_, afNil())) => afNil()
  rule afFuelBytes(afCons(F, Fs), afCons(B, afCons(N, Ns)))
    => afCons(F -Int B +Int 256 *Int N, afFuelBytes(Fs, afCons(N,Ns)))

  syntax AFRegime ::= afRegime(Int, AFControl, Int, Int) [function]
  rule afRegime(0, _, _, _) => afNoEvent()
  rule afRegime(1, afReturn(V), 0, _) => afDone(V)
  rule afRegime(1, afError(C), _, _) => afRejected(C)
  rule afRegime(1, C, D, 1) => afExhausted() requires notBool afTerminal(C,D)
  rule afRegime(1, C, D, 0) => afCharged() requires notBool afTerminal(C,D)
  syntax Int ::= afCharge(AFRegime) [function]
  rule afCharge(afNoEvent()) => 0
  rule afCharge(afDone(_)) => 0
  rule afCharge(afRejected(_)) => 0
  rule afCharge(afExhausted()) => 0
  rule afCharge(afCharged()) => 1

  syntax Bool ::= afFuelDomain(Int, Int, AFControl, Int, AFSeq, AFSeq) [function]
  rule afFuelDomain(O,N,C,D,F,I)
    => O >=Int 0 andBool O <=Int 3 andBool N >=Int 0 andBool D >=Int 0
       andBool afControlDomain(C) andBool afWord(F) andBool afWord(I)
  syntax AFFuel ::= afFuelAttempt(Int, Int, AFControl, Int, AFSeq, AFSeq) [function]
                  | afFuelStage(Int, AFControl, Int, AFSeq, AFSeq, AFSeq, AFSeq) [function]
                  | afFuelOutput(AFRegime, AFSeq, AFSeq, AFSeq, AFSeq) [function]
  rule afFuelAttempt(O,N,C,D,F,I)
    => afFuelStage(afEvent(O,N), C,D,F,I, afProbe(afEvent(O,N),F),
                   afPrefix(afEvent(O,N),afProbe(afEvent(O,N),F)))
    requires afFuelDomain(O,N,C,D,F,I)
  rule afFuelStage(E,C,D,F,I,Z,P)
    => afFuelOutput(afRegime(E,C,D,afLast(P)),F,I,Z,P)
  rule afFuelOutput(R,F,I,Z,P)
    => afFuel(R,afCharge(R),afFuelBytes(F,afPrefix(afCharge(R),Z)),I,Z,P,
               afPrefix(afCharge(R),Z))
endmodule

module P4-ARITHMETIC-FUEL
  imports P4-ARITHMETIC-FUEL-FUNCTIONS
  configuration <k> $PGM:AFRequest </k>
  // These are model request rewrites, not accepted A-machine steps.
  rule <k> afArithmeticRequest(Op,L,A,R,B) => afArithmetic(Op,L,A,R,B) </k>
    requires afArithDomain(Op,L,A,R,B)
  rule <k> afFuelRequest(O,N,C,D,F,I) => afFuelAttempt(O,N,C,D,F,I) </k>
    requires afFuelDomain(O,N,C,D,F,I)
endmodule
