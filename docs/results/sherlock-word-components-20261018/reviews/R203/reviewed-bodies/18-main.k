
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
