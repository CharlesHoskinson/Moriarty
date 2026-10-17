requires "domains.md"

module MORIARTY-UI-SYNTAX
  imports DOMAINS-SYNTAX

  syntax UIUInt ::= uiU(Int)
  syntax UIListValue ::= uiNil() | uiCons(UIUInt, UIListValue)
  syntax UITreeValue ::= uiLeaf(UIUInt) | uiBranch(UITreeValue, UITreeValue)
  syntax UIValue ::= UIUInt | UIListValue | UITreeValue | uiB(Bool)
  syntax UIType ::= uiUIntT() | uiBoolT() | uiListT() | uiTreeT()
  syntax UIOp ::= uiAdd() | uiSub() | uiMul() | uiEq() | uiLt()
                | uiConsOp() | uiBranchOp()
  syntax UIUnaryOp ::= uiLeafOp() | uiNotOp()
  syntax UIExpr ::= uiLiteral(UIValue)
                  | uiVar(Int)
                  | uiLet(Int, UIExpr, UIExpr)
                  | uiBinary(UIOp, UIExpr, UIExpr)
                  | uiUnary(UIUnaryOp, UIExpr)
                  | uiIf(UIExpr, UIExpr, UIExpr)
                  | uiCall(Int, UIExprs)
                  | uiMatch(UIExpr, UICases)
  syntax UIExprs ::= uiNoExprs() | uiExpr(UIExpr, UIExprs)
  syntax UIValues ::= uiNoValues() | uiValue(UIValue, UIValues)
  syntax UIPattern ::= uiPVar(Int) | uiPNil() | uiPCons(Int, Int)
                     | uiPLeaf(Int) | uiPBranch(Int, Int)
  syntax UICases ::= uiNoCases() | uiCase(UIPattern, UIExpr, UICases)
  syntax UIParams ::= uiNoParams() | uiParam(Int, UIType, UIParams)
  syntax UIFun ::= uiFun(Int, UIParams, UIType, UIExpr)
  syntax UIFuns ::= uiNoFuns() | uiFunction(UIFun, UIFuns)
  syntax UIRun ::= uiRun(UIFuns, UIExpr, Int, Int)
  syntax KItem ::= UIRun
endmodule

module MORIARTY-UI
  imports MORIARTY-UI-SYNTAX
  imports DOMAINS

  syntax UIControl ::= uiEval(UIExpr) | uiRet(UIValue)
                     | uiPush(UIFrame, UIControl, Map)
                     | uiInvoke(Int, UIValues)
                     | uiEnter(UIFunLookup, UIValues)
                     | uiBound(UIBinding, UIType, UIExpr)
                     | uiCases(UIValue, UICases, Map)
                     | uiCaseResult(UIMatching, UIValue, UIExpr, UICases, Map)
                     | uiPrimResult(UIPrimitive)
                     | uiError(Int)
  syntax KItem ::= UIControl
  syntax UIFrame ::= uiLetF(Int, UIExpr, Map)
                   | uiLeftF(UIOp, UIExpr, Map)
                   | uiRightF(UIOp, UIValue)
                   | uiUnaryF(UIUnaryOp)
                   | uiIfF(UIExpr, UIExpr, Map)
                   | uiMatchF(UICases, Map)
                   | uiArgsF(Int, UIValues, UIExprs, Map)
                   | uiReturnF(UIType, Map) | uiScopeF(Map)
  syntax UIFrames ::= uiNoFrames() | uiFrame(UIFrame, UIFrames)
  syntax UIOutput ::= uiPending() | uiDone(UIValue)
                    | uiRejected(Int) | uiExhausted()
  syntax UIFunLookup ::= UIFun | uiMissingFun()
  syntax UIBinding ::= uiBindings(Map) | uiBadBinding()
  syntax UIMatching ::= uiMatched(Map) | uiNoMatch()
  syntax UIPrimitive ::= uiPrimValue(UIValue) | uiPrimError(Int)

  configuration
    <k> $PGM:UIRun </k>
    <env> .Map </env>
    <stack> uiNoFrames() </stack>
    <depth> 0 </depth> <stackLimit> 0 </stackLimit>
    <code> uiNoFuns() </code>
    <fuel> 0 </fuel> <initialFuel> 0 </initialFuel>
    <out> uiPending() </out>

  // Startup and terminal publication are administrative, with no fuel charge.
  rule <k> uiRun(P, E, F, L) => uiEval(E) </k>
       <code> _ => P </code> <fuel> _ => F </fuel>
       <initialFuel> _ => F </initialFuel> <stackLimit> _ => L </stackLimit>
    requires F >=Int 0 andBool L >=Int 0
  rule <k> uiRun(_, _, F, L) => uiError(1) </k>
    requires F <Int 0 orBool L <Int 0
  rule <k> uiRet(V) => .K </k> <stack> uiNoFrames() </stack>
       <out> uiPending() => uiDone(V) </out>
  rule <k> uiError(C) => .K </k>
       <out> uiPending() => uiRejected(C) </out>
  rule <k> C:UIControl => .K </k> <stack> S </stack> <fuel> 0 </fuel>
       <out> uiPending() => uiExhausted() </out>
    requires notBool uiTerminal(C, S)

  syntax Bool ::= uiTerminal(UIControl, UIFrames) [function]
  rule uiTerminal(uiRet(_), uiNoFrames()) => true
  rule uiTerminal(uiError(_), _) => true
  rule uiTerminal(uiEval(_), _) => false
  rule uiTerminal(uiPush(_, _, _), _) => false
  rule uiTerminal(uiInvoke(_, _), _) => false
  rule uiTerminal(uiEnter(_, _), _) => false
  rule uiTerminal(uiBound(_, _, _), _) => false
  rule uiTerminal(uiCases(_, _, _), _) => false
  rule uiTerminal(uiCaseResult(_, _, _, _, _), _) => false
  rule uiTerminal(uiPrimResult(_), _) => false
  rule uiTerminal(uiRet(_), uiFrame(_, _)) => false

  // Every rule below in the machine section consumes one semantic fuel unit.
  rule <k> uiEval(uiLiteral(V)) => uiRet(V) </k>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool uiValid(V)
  rule <k> uiEval(uiLiteral(V)) => uiError(2) </k>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool notBool uiValid(V)
  rule <k> uiEval(uiVar(X)) => uiRet(V) </k>
       <env> ... X |-> V:UIValue ... </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEval(uiVar(X)) => uiError(3) </k> <env> Rho </env>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool notBool X in_keys(Rho)
  rule <k> uiEval(uiLet(X, A, B))
        => uiPush(uiLetF(X, B, Rho), uiEval(A), Rho) </k> <env> Rho </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEval(uiBinary(Op, A, B))
        => uiPush(uiLeftF(Op, B, Rho), uiEval(A), Rho) </k> <env> Rho </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEval(uiUnary(Op, A))
        => uiPush(uiUnaryF(Op), uiEval(A), Rho) </k> <env> Rho </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEval(uiIf(C, A, B))
        => uiPush(uiIfF(A, B, Rho), uiEval(C), Rho) </k> <env> Rho </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEval(uiMatch(E, Cs))
        => uiPush(uiMatchF(Cs, Rho), uiEval(E), Rho) </k> <env> Rho </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEval(uiCall(N, uiNoExprs())) => uiInvoke(N, uiNoValues()) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEval(uiCall(N, uiExpr(A, Rest)))
        => uiPush(uiArgsF(N, uiNoValues(), Rest, Rho), uiEval(A), Rho) </k>
       <env> Rho </env> <fuel> F => F -Int 1 </fuel> requires F >Int 0

  rule <k> uiPush(Fr, C, Rho) => C </k> <env> _ => Rho </env>
       <stack> S => uiFrame(Fr, S) </stack>
       <depth> D => D +Int 1 </depth> <stackLimit> L </stackLimit>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool D <Int L
  rule <k> uiPush(_, _, _) => uiError(4) </k>
       <depth> D </depth> <stackLimit> L </stackLimit>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool D >=Int L

  rule <k> uiRet(V) => uiEval(B) </k>
       <stack> uiFrame(uiLetF(X, B, Rho), S) => uiFrame(uiScopeF(Rho), S) </stack>
       <env> _ => Rho[X <- V] </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiRet(V) => uiEval(B) </k>
       <stack> uiFrame(uiLeftF(Op, B, Rho), S)
             => uiFrame(uiRightF(Op, V), S) </stack>
       <env> _ => Rho </env> <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiRet(V) => uiPrimResult(uiBinaryResult(Op, A, V)) </k>
       <stack> uiFrame(uiRightF(Op, A), S) => S </stack>
       <depth> D => D -Int 1 </depth> <fuel> F => F -Int 1 </fuel>
    requires F >Int 0
  rule <k> uiRet(V) => uiPrimResult(uiUnaryResult(Op, V)) </k>
       <stack> uiFrame(uiUnaryF(Op), S) => S </stack>
       <depth> D => D -Int 1 </depth> <fuel> F => F -Int 1 </fuel>
    requires F >Int 0
  rule <k> uiRet(uiB(true)) => uiEval(A) </k>
       <stack> uiFrame(uiIfF(A, _, Rho), S) => S </stack>
       <env> _ => Rho </env> <depth> D => D -Int 1 </depth>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiRet(uiB(false)) => uiEval(B) </k>
       <stack> uiFrame(uiIfF(_, B, Rho), S) => S </stack>
       <env> _ => Rho </env> <depth> D => D -Int 1 </depth>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiRet(V) => uiError(5) </k>
       <stack> uiFrame(uiIfF(_, _, _), _) </stack>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool notBool uiHasType(V, uiBoolT())
  rule <k> uiRet(V) => uiCases(V, Cs, Rho) </k>
       <stack> uiFrame(uiMatchF(Cs, Rho), S) => S </stack>
       <env> _ => Rho </env> <depth> D => D -Int 1 </depth>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiRet(V) => uiEval(A) </k>
       <stack> uiFrame(uiArgsF(N, Vs, uiExpr(A, Rest), Rho), S)
             => uiFrame(uiArgsF(N, uiAppend(Vs, V), Rest, Rho), S) </stack>
       <env> _ => Rho </env> <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiRet(V) => uiInvoke(N, uiAppend(Vs, V)) </k>
       <stack> uiFrame(uiArgsF(N, Vs, uiNoExprs(), Rho), S) => S </stack>
       <env> _ => Rho </env> <depth> D => D -Int 1 </depth> <fuel> F => F -Int 1 </fuel>
    requires F >Int 0
  rule <k> uiRet(V) => uiRet(V) </k>
       <stack> uiFrame(uiReturnF(T, Rho), S) => S </stack>
       <env> _ => Rho </env> <depth> D => D -Int 1 </depth>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool uiHasType(V, T)
  rule <k> uiRet(V) => uiError(6) </k>
       <stack> uiFrame(uiReturnF(T, _), _) </stack>
       <fuel> F => F -Int 1 </fuel>
    requires F >Int 0 andBool notBool uiHasType(V, T)

  rule <k> uiRet(V) => uiRet(V) </k>
       <stack> uiFrame(uiScopeF(Rho), S) => S </stack>
       <env> _ => Rho </env> <depth> D => D -Int 1 </depth>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0

  rule <k> uiInvoke(N, Vs) => uiEnter(uiFind(N, P), Vs) </k> <code> P </code>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEnter(uiMissingFun(), _) => uiError(7) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiEnter(uiFun(_, Ps, T, E), Vs) => uiBound(uiBind(Ps, Vs), T, E) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiBound(uiBadBinding(), _, _) => uiError(8) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiBound(uiBindings(New), T, E)
        => uiPush(uiReturnF(T, Old), uiEval(E), New) </k> <env> Old </env>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0

  rule <k> uiCases(_, uiNoCases(), _) => uiError(9) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiCases(V, uiCase(P, E, Rest), Rho)
        => uiCaseResult(uiMatching(P, V, Rho), V, E, Rest, Rho) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiCaseResult(uiNoMatch(), V, _, Rest, Rho)
        => uiCases(V, Rest, Rho) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiCaseResult(uiMatched(Bs), _, E, _, Rho)
        => uiPush(uiScopeF(Rho), uiEval(E), Bs) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiPrimResult(uiPrimValue(V)) => uiRet(V) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0
  rule <k> uiPrimResult(uiPrimError(C)) => uiError(C) </k>
       <fuel> F => F -Int 1 </fuel> requires F >Int 0

  // Pure, structurally terminating helpers: their rewrites are not fuel events.
  syntax Bool ::= uiValid(UIValue) [function]
  rule uiValid(uiU(N)) => 0 <=Int N andBool N <Int 340282366920938463463374607431768211456
  rule uiValid(uiB(_)) => true
  rule uiValid(uiNil()) => true
  rule uiValid(uiCons(H, T)) => uiValid(H) andBool uiValid(T)
  rule uiValid(uiLeaf(V)) => uiValid(V)
  rule uiValid(uiBranch(L, R)) => uiValid(L) andBool uiValid(R)

  syntax Bool ::= uiHasType(UIValue, UIType) [function]
  rule uiHasType(_:UIUInt, uiUIntT()) => true
  rule uiHasType(uiB(_), uiBoolT()) => true
  rule uiHasType(_:UIListValue, uiListT()) => true
  rule uiHasType(_:UITreeValue, uiTreeT()) => true
  rule uiHasType(_:UIUInt, T) => false requires T =/=K uiUIntT()
  rule uiHasType(uiB(_), T) => false requires T =/=K uiBoolT()
  rule uiHasType(_:UIListValue, T) => false requires T =/=K uiListT()
  rule uiHasType(_:UITreeValue, T) => false requires T =/=K uiTreeT()

  syntax UIPrimitive ::= uiChecked(Int) [function]
                       | uiBinaryResult(UIOp, UIValue, UIValue) [function]
                       | uiUnaryResult(UIUnaryOp, UIValue) [function]
  rule uiChecked(N) => uiPrimValue(uiU(N))
    requires 0 <=Int N andBool N <Int 340282366920938463463374607431768211456
  rule uiChecked(N) => uiPrimError(10)
    requires N <Int 0 orBool N >=Int 340282366920938463463374607431768211456
  rule uiBinaryResult(uiAdd(), uiU(A), uiU(B)) => uiChecked(A +Int B)
  rule uiBinaryResult(uiSub(), uiU(A), uiU(B)) => uiChecked(A -Int B)
  rule uiBinaryResult(uiMul(), uiU(A), uiU(B)) => uiChecked(A *Int B)
  rule uiBinaryResult(uiLt(), uiU(A), uiU(B)) => uiPrimValue(uiB(A <Int B))
  rule uiBinaryResult(uiEq(), A, B) => uiPrimValue(uiB(A ==K B))
  rule uiBinaryResult(uiConsOp(), H:UIUInt, T:UIListValue) => uiPrimValue(uiCons(H, T))
  rule uiBinaryResult(uiBranchOp(), L:UITreeValue, R:UITreeValue) => uiPrimValue(uiBranch(L, R))
  rule uiBinaryResult(uiAdd(), A, B) => uiPrimError(11)
    requires notBool (uiHasType(A, uiUIntT()) andBool uiHasType(B, uiUIntT()))
  rule uiBinaryResult(uiSub(), A, B) => uiPrimError(11)
    requires notBool (uiHasType(A, uiUIntT()) andBool uiHasType(B, uiUIntT()))
  rule uiBinaryResult(uiMul(), A, B) => uiPrimError(11)
    requires notBool (uiHasType(A, uiUIntT()) andBool uiHasType(B, uiUIntT()))
  rule uiBinaryResult(uiLt(), A, B) => uiPrimError(11)
    requires notBool (uiHasType(A, uiUIntT()) andBool uiHasType(B, uiUIntT()))
  rule uiBinaryResult(uiConsOp(), A, B) => uiPrimError(11)
    requires notBool (uiHasType(A, uiUIntT()) andBool uiHasType(B, uiListT()))
  rule uiBinaryResult(uiBranchOp(), A, B) => uiPrimError(11)
    requires notBool (uiHasType(A, uiTreeT()) andBool uiHasType(B, uiTreeT()))
  rule uiUnaryResult(uiLeafOp(), V:UIUInt) => uiPrimValue(uiLeaf(V))
  rule uiUnaryResult(uiNotOp(), uiB(B)) => uiPrimValue(uiB(notBool B))
  rule uiUnaryResult(uiLeafOp(), V) => uiPrimError(11)
    requires notBool uiHasType(V, uiUIntT())
  rule uiUnaryResult(uiNotOp(), V) => uiPrimError(11)
    requires notBool uiHasType(V, uiBoolT())

  syntax UIFunLookup ::= uiFind(Int, UIFuns) [function]
  rule uiFind(_, uiNoFuns()) => uiMissingFun()
  rule uiFind(N, uiFunction(uiFun(N, Ps, T, E), _)) => uiFun(N, Ps, T, E)
  rule uiFind(N, uiFunction(uiFun(M, _, _, _), Rest)) => uiFind(N, Rest)
    requires N =/=Int M
  syntax UIValues ::= uiAppend(UIValues, UIValue) [function]
  rule uiAppend(uiNoValues(), V) => uiValue(V, uiNoValues())
  rule uiAppend(uiValue(H, T), V) => uiValue(H, uiAppend(T, V))
  syntax UIBinding ::= uiBind(UIParams, UIValues) [function]
                     | uiBindOne(Int, UIType, UIValue, UIBinding) [function]
  rule uiBind(uiNoParams(), uiNoValues()) => uiBindings(.Map)
  rule uiBind(uiParam(X, T, Ps), uiValue(V, Vs)) => uiBindOne(X, T, V, uiBind(Ps, Vs))
  rule uiBind(uiNoParams(), uiValue(_, _)) => uiBadBinding()
  rule uiBind(uiParam(_, _, _), uiNoValues()) => uiBadBinding()
  rule uiBindOne(X, T, V, uiBindings(M)) => uiBindings(M[X <- V])
    requires uiHasType(V, T) andBool notBool X in_keys(M)
  rule uiBindOne(_, _, _, uiBadBinding()) => uiBadBinding()
  rule uiBindOne(X, T, V, uiBindings(M)) => uiBadBinding()
    requires notBool uiHasType(V, T) orBool X in_keys(M)
  syntax UIMatching ::= uiMatching(UIPattern, UIValue, Map) [function]
  rule uiMatching(uiPVar(X), V, Rho) => uiMatched(Rho[X <- V])
  rule uiMatching(uiPNil(), uiNil(), Rho) => uiMatched(Rho)
  rule uiMatching(uiPCons(X, Y), uiCons(H, T), Rho) => uiMatched(Rho[X <- H][Y <- T])
    requires X =/=Int Y
  rule uiMatching(uiPLeaf(X), uiLeaf(V), Rho) => uiMatched(Rho[X <- V])
  rule uiMatching(uiPBranch(X, Y), uiBranch(L, R), Rho) => uiMatched(Rho[X <- L][Y <- R])
    requires X =/=Int Y
  rule uiMatching(uiPNil(), V, _) => uiNoMatch() requires uiKind(V) =/=Int 2
  rule uiMatching(uiPCons(X, Y), V, _) => uiNoMatch()
    requires uiKind(V) =/=Int 3 orBool X ==Int Y
  rule uiMatching(uiPLeaf(_), V, _) => uiNoMatch() requires uiKind(V) =/=Int 4
  rule uiMatching(uiPBranch(X, Y), V, _) => uiNoMatch()
    requires uiKind(V) =/=Int 5 orBool X ==Int Y
  syntax Int ::= uiKind(UIValue) [function]
  rule uiKind(uiU(_)) => 0
  rule uiKind(uiB(_)) => 1
  rule uiKind(uiNil()) => 2
  rule uiKind(uiCons(_, _)) => 3
  rule uiKind(uiLeaf(_)) => 4
  rule uiKind(uiBranch(_, _)) => 5
endmodule
