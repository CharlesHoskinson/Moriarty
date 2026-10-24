requires "nominal-product-syntax.k"

// Positive admitted-domain reference only. New uncharged microsteps; no A39,
// B helper clock, product diagnostic, capacity/guard or financial policy.
module SHERLOCK-NOMINAL-PRODUCT
  imports SHERLOCK-NOMINAL-PRODUCT-SYNTAX
  imports DOMAINS

  syntax NGNodes ::= ngNodes(NGProgram) [function]
  rule ngNodes(ngProgram(_, Ns, _, _)) => Ns
  syntax NGFunctions ::= ngFunctions(NGProgram) [function]
  rule ngFunctions(ngProgram(_, _, Fs, _)) => Fs
  syntax NGDecls ::= ngDeclarations(NGProgram) [function]
  rule ngDeclarations(ngProgram(Ds, _, _, _)) => Ds
  syntax NGExprLookup ::= ngFindExpr(Int, NGNodes) [function]
  rule ngFindExpr(_, ngNoNodes()) => ngMissingExpr()
  rule ngFindExpr(E, ngNode(E, X, _)) => ngFoundExpr(X)
  rule ngFindExpr(E, ngNode(F, _, Rest)) => ngFindExpr(E, Rest)
    requires E =/=Int F
  syntax Int ::= ngOccurrence(NGExprLookup) [function]
  rule ngOccurrence(ngFoundExpr(ngLiteral(O, _))) => O
  rule ngOccurrence(ngFoundExpr(ngVariable(O, _))) => O
  rule ngOccurrence(ngFoundExpr(ngAdd(O, _, _))) => O
  rule ngOccurrence(ngFoundExpr(ngConstruct(O, _, _, _))) => O
  rule ngOccurrence(ngFoundExpr(ngCall(O, _, _))) => O
  rule ngOccurrence(ngFoundExpr(ngMatch(O, _, _))) => O
  rule ngOccurrence(ngFoundExpr(ngLet(O, _, _, _))) => O
  rule ngOccurrence(ngFoundExpr(ngIf(O, _, _, _))) => O
  syntax NGFunctionLookup ::= ngFindFunction(Int, NGFunctions) [function]
  rule ngFindFunction(_, ngNoFunctions()) => ngMissingFunction()
  rule ngFindFunction(F, ngFunctionRow(ngFunction(F, Ps, T, B), _))
    => ngFoundFunction(ngFunction(F, Ps, T, B))
  rule ngFindFunction(F, ngFunctionRow(ngFunction(G, _, _, _), Rest))
    => ngFindFunction(F, Rest) requires F =/=Int G
  syntax NGCtorLookup ::= ngFindCtor(Int, Int, NGDecls) [function]
                       | ngFindCtorRow(Int, Int, NGCtors) [function]
  rule ngFindCtor(_, _, ngNoDecls()) => ngMissingCtor()
  rule ngFindCtor(T, C, ngDecl(T, _, Cs, _)) => ngFindCtorRow(T, C, Cs)
  rule ngFindCtor(T, C, ngDecl(U, _, _, Rest)) => ngFindCtor(T, C, Rest)
    requires T =/=Int U
  rule ngFindCtorRow(_, _, ngNoCtors()) => ngMissingCtor()
  rule ngFindCtorRow(T, C, ngCtorRow(ngCtor(T, C, S, Ts), _))
    => ngFoundCtor(ngCtor(T, C, S, Ts))
  rule ngFindCtorRow(T, C, ngCtorRow(ngCtor(U, D, _, _), Rest))
    => ngFindCtorRow(T, C, Rest) requires T =/=Int U orBool C =/=Int D
  syntax Int ::= ngTypeOf(NGValue) [function]
  rule ngTypeOf(ngUInt(_)) => 0
  rule ngTypeOf(ngBool(_)) => 1
  rule ngTypeOf(ngNominal(T, _, _)) => T
  syntax Bool ::= ngIsUInt(NGValue) [function]
  rule ngIsUInt(ngUInt(_)) => true
  rule ngIsUInt(ngBool(_)) => false
  rule ngIsUInt(ngNominal(_, _, _)) => false
  syntax Bool ::= ngIsBool(NGValue) [function]
  rule ngIsBool(ngBool(_)) => true
  rule ngIsBool(ngUInt(_)) => false
  rule ngIsBool(ngNominal(_, _, _)) => false
  syntax Int ::= ngTypeCount(NGTypes) [function]
               | ngExprCount(NGExprIds) [function]
               | ngTypeAt(NGTypes, Int) [function]
  rule ngTypeCount(ngNoTypes()) => 0
  rule ngTypeCount(ngType(_, Ts)) => 1 +Int ngTypeCount(Ts)
  rule ngExprCount(ngNoExprs()) => 0
  rule ngExprCount(ngExprId(_, Es)) => 1 +Int ngExprCount(Es)
  rule ngTypeAt(ngType(T, _), 0) => T
  rule ngTypeAt(ngType(_, Ts), I) => ngTypeAt(Ts, I -Int 1) requires I >Int 0
  syntax NGValues ::= ngAppendValue(NGValues, NGValue) [function]
  rule ngAppendValue(ngNoValues(), V) => ngValue(V, ngNoValues())
  rule ngAppendValue(ngValue(H, T), V) => ngValue(H, ngAppendValue(T, V))
  syntax NGEvents ::= ngAppendEvent(NGEvents, NGEvent) [function]
  rule ngAppendEvent(ngNoEvents(), E) => ngEvent(E, ngNoEvents())
  rule ngAppendEvent(ngEvent(H, T), E) => ngEvent(H, ngAppendEvent(T, E))

  // Parameter bindings have only parameters, never the caller environment.
  // Unique names, closed code and binder-linear patterns remain input premises.
  syntax NGEnvironment ::= ngBindParameters(NGParams, NGValues) [function]
                         | ngBindParameter(Int, Int, NGValue, NGEnvironment) [function]
  rule ngBindParameters(ngNoParams(), ngNoValues()) => ngBoundEnvironment(.Map)
  rule ngBindParameters(ngParam(N, T, Ps), ngValue(V, Vs))
    => ngBindParameter(N, T, V, ngBindParameters(Ps, Vs))
  rule ngBindParameters(ngNoParams(), ngValue(_, _)) => ngBadBindings()
  rule ngBindParameters(ngParam(_, _, _), ngNoValues()) => ngBadBindings()
  rule ngBindParameter(N, T, V, ngBoundEnvironment(Rho))
    => ngBoundEnvironment(Rho[N <- V])
    requires ngTypeOf(V) ==Int T andBool notBool N in_keys(Rho)
  rule ngBindParameter(_, _, _, ngBadBindings()) => ngBadBindings()
  rule ngBindParameter(N, T, V, ngBoundEnvironment(Rho)) => ngBadBindings()
    requires ngTypeOf(V) =/=Int T orBool N in_keys(Rho)

  syntax NGPath ::= ngChildPath(NGPath, Int) [function]
  rule ngChildPath(ngRoot(), I) => ngAt(I, ngRoot())
  rule ngChildPath(ngAt(H, T), I) => ngAt(H, ngChildPath(T, I))
  syntax NGBindings ::= ngJoinBindings(NGBindings, NGBindings) [function]
  rule ngJoinBindings(ngNoBindings(), Bs) => Bs
  rule ngJoinBindings(ngBinding(N, M, A, P, V, Rest), Bs)
    => ngBinding(N, M, A, P, V, ngJoinBindings(Rest, Bs))
  syntax NGMatching ::= ngCombineMatches(NGMatching, NGMatching) [function]
  rule ngCombineMatches(ngMatched(A), ngMatched(B)) => ngMatched(ngJoinBindings(A, B))
  rule ngCombineMatches(ngMiss(), _) => ngMiss()
  rule ngCombineMatches(ngMatched(_), ngMiss()) => ngMiss()
  syntax NGMatching ::= ngMatchPattern(NGPattern, NGValue, Int, Int, NGPath) [function]
                      | ngMatchFields(NGPatterns, NGValues, Int, Int, NGPath, Int) [function]
  rule ngMatchPattern(ngWildcard(), _, _, _, _) => ngMatched(ngNoBindings())
  rule ngMatchPattern(ngBind(N), V, M, A, Path)
    => ngMatched(ngBinding(N, M, A, Path, V, ngNoBindings()))
  rule ngMatchPattern(ngConstructorPattern(T, C, Ps), ngNominal(T, C, Vs), M, A, Path)
    => ngMatchFields(Ps, Vs, M, A, Path, 0)
  rule ngMatchPattern(ngConstructorPattern(T, C, _), ngNominal(U, D, _), _, _, _)
    => ngMiss() requires T =/=Int U orBool C =/=Int D
  rule ngMatchPattern(ngConstructorPattern(_, _, _), ngUInt(_), _, _, _) => ngMiss()
  rule ngMatchPattern(ngConstructorPattern(_, _, _), ngBool(_), _, _, _) => ngMiss()
  rule ngMatchFields(ngNoPatterns(), ngNoValues(), _, _, _, _) => ngMatched(ngNoBindings())
  rule ngMatchFields(ngPattern(P, Ps), ngValue(V, Vs), M, A, Path, I)
    => ngCombineMatches(ngMatchPattern(P, V, M, A, ngChildPath(Path, I)),
                         ngMatchFields(Ps, Vs, M, A, Path, I +Int 1))
  rule ngMatchFields(ngNoPatterns(), ngValue(_, _), _, _, _, _) => ngMiss()
  rule ngMatchFields(ngPattern(_, _), ngNoValues(), _, _, _, _) => ngMiss()
  syntax Map ::= ngExtendEnvironment(NGBindings, Map) [function]
  rule ngExtendEnvironment(ngNoBindings(), Rho) => Rho
  rule ngExtendEnvironment(ngBinding(N, _, _, _, V, Bs), Rho)
    => ngExtendEnvironment(Bs, Rho[N <- V])

  syntax NGState ::= ngInitial(NGProgram, Map) [function]
                  | ngStep(NGState) [function]
                  | ngAdvance(NGState, Int) [function]
                  | ngOutside(NGState, String) [function]
  rule ngInitial(ngProgram(Ds, Ns, Fs, E), Rho)
    => ngState(ngProgram(Ds, Ns, Fs, E), ngEval(E), Rho, ngNoFrames(), ngNoEvents())
  rule ngOutside(ngState(P, _, Rho, Fs, Es), Reason)
    => ngState(P, ngOutsideModel(Reason), Rho, Fs,
               ngAppendEvent(Es, ngOutsideEvent(Reason)))
  rule ngStep(ngState(P, ngHalted(V), Rho, Fs, Es))
    => ngState(P, ngHalted(V), Rho, Fs, Es)
  rule ngStep(ngState(P, ngOutsideModel(S), Rho, Fs, Es))
    => ngState(P, ngOutsideModel(S), Rho, Fs, Es)
  // Observer cuts preserve pending controls/frames/events; no exhaustion/fuel.
  rule ngAdvance(S, 0) => S
  rule ngAdvance(ngState(P, ngHalted(V), Rho, Fs, Es), N)
    => ngState(P, ngHalted(V), Rho, Fs, Es) requires N >Int 0
  rule ngAdvance(ngState(P, ngOutsideModel(S), Rho, Fs, Es), N)
    => ngState(P, ngOutsideModel(S), Rho, Fs, Es) requires N >Int 0
  syntax Bool ::= ngTerminal(NGControl) [function]
  rule ngTerminal(ngHalted(_)) => true
  rule ngTerminal(ngOutsideModel(_)) => true
  rule ngTerminal(ngEval(_)) => false
  rule ngTerminal(ngReturn(_)) => false
  rule ngTerminal(ngApply(_, _, _, _)) => false
  rule ngTerminal(ngChoose(_, _, _, _, _)) => false
  rule ngTerminal(ngMaterialize(_, _, _, _, _)) => false
  rule ngAdvance(ngState(P, C, Rho, Fs, Es), N)
    => ngAdvance(ngStep(ngState(P, C, Rho, Fs, Es)), N -Int 1)
    requires N >Int 0 andBool notBool ngTerminal(C)

  syntax NGState ::= ngEvaluate(NGExprLookup, Int, NGProgram, Map, NGFrames, NGEvents) [function]
  rule ngStep(ngState(P, ngEval(E), Rho, Fs, Es))
    => ngEvaluate(ngFindExpr(E, ngNodes(P)), E, P, Rho, Fs, Es)
  rule ngEvaluate(ngMissingExpr(), E, P, Rho, Fs, Es)
    => ngOutside(ngState(P, ngEval(E), Rho, Fs, Es), "missing_expression")
  rule ngEvaluate(ngFoundExpr(ngLiteral(O, V)), E, P, Rho, Fs, Es)
    => ngState(P, ngReturn(V), Rho, Fs,
               ngAppendEvent(Es, ngLiteralEvaluated(E, O, V)))
  rule ngEvaluate(ngFoundExpr(ngVariable(O, X)), E, P,
                  (X |-> V:NGValue) Rho:Map, Fs, Es)
    => ngState(P, ngReturn(V), (X |-> V) Rho, Fs,
               ngAppendEvent(Es, ngVariableRead(E, O, X, V)))
  rule ngEvaluate(ngFoundExpr(ngVariable(O, X)), E, P, Rho, Fs, Es)
    => ngOutside(ngState(P, ngEval(E), Rho, Fs, Es), "unbound_name")
    requires notBool X in_keys(Rho)
  rule ngEvaluate(ngFoundExpr(ngAdd(O, L, R)), E, P, Rho, Fs, Es)
    => ngState(P, ngEval(L), Rho, ngFrame(ngAddLeft(E, O, R, Rho), Fs),
               ngAppendEvent(Es, ngAddStarted(E, O, L, R, Rho)))
  rule ngEvaluate(ngFoundExpr(ngLet(O, X, Bound, Body)), E, P, Rho, Fs, Es)
    => ngState(P, ngEval(Bound), Rho,
         ngFrame(ngLetValue(E, O, X, Bound, Body, Rho), Fs),
         ngAppendEvent(Es, ngLetStarted(E, O, X, Bound, Body, Rho)))
  rule ngEvaluate(ngFoundExpr(ngIf(O, Condition, Yes, No)), E, P, Rho, Fs, Es)
    => ngState(P, ngEval(Condition), Rho,
         ngFrame(ngCondition(E, O, Condition, Yes, No, Rho), Fs),
         ngAppendEvent(Es, ngIfStarted(E, O, Condition, Yes, No, Rho)))
  rule ngEvaluate(ngFoundExpr(ngCall(O, F, ngNoExprs())), E, P, Rho, Fs, Es)
    => ngState(P, ngApply(E, O, F, ngNoValues()), Rho, Fs,
               ngAppendEvent(Es, ngCallStarted(E, O, F, ngNoExprs(), Rho)))
  rule ngEvaluate(ngFoundExpr(ngCall(O, F, ngExprId(A, Rest))), E, P, Rho, Fs, Es)
    => ngState(P, ngEval(A), Rho,
         ngFrame(ngArguments(E, O, F, 0, A, ngNoValues(), Rest, Rho), Fs),
         ngAppendEvent(Es, ngCallStarted(E, O, F, ngExprId(A, Rest), Rho)))
  rule ngEvaluate(ngFoundExpr(ngMatch(O, X, As)), E, P, Rho, Fs, Es)
    => ngState(P, ngEval(X), Rho, ngFrame(ngScrutinee(E, O, As, Rho), Fs),
               ngAppendEvent(Es, ngMatchStarted(E, O, X, As, Rho)))
  rule ngEvaluate(ngFoundExpr(ngConstruct(O, T, C, Xs)), E, P, Rho, Fs, Es)
    => ngStartConstructor(ngFindCtor(T, C, ngDeclarations(P)), E, O, T, C, Xs,
                           P, Rho, Fs, Es)

  syntax NGState ::= ngStartConstructor(NGCtorLookup, Int, Int, Int, Int, NGExprIds,
                                        NGProgram, Map, NGFrames, NGEvents) [function]
  rule ngStartConstructor(ngMissingCtor(), E, _, _, _, _, P, Rho, Fs, Es)
    => ngOutside(ngState(P, ngEval(E), Rho, Fs, Es), "constructor_not_declared")
  rule ngStartConstructor(ngFoundCtor(ngCtor(T, C, _, ngNoTypes())), E, O, T, C,
                           ngNoExprs(), P, Rho, Fs, Es)
    => ngState(P, ngMaterialize(E, O, T, C, ngNoValues()), Rho, Fs,
         ngAppendEvent(Es, ngConstructorStarted(E, O, T, C, ngNoExprs(), ngNoTypes(), Rho)))
  rule ngStartConstructor(ngFoundCtor(ngCtor(T, C, _, Ts)), E, O, T, C,
                           ngExprId(X, Rest), P, Rho, Fs, Es)
    => ngState(P, ngEval(X), Rho,
         ngFrame(ngConstructorFields(E, O, T, C, Ts, 0, X, ngNoValues(), Rest, Rho), Fs),
         ngAppendEvent(Es, ngConstructorStarted(E, O, T, C, ngExprId(X, Rest), Ts, Rho)))
    requires ngTypeCount(Ts) ==Int ngExprCount(ngExprId(X, Rest))
  rule ngStartConstructor(ngFoundCtor(ngCtor(_, _, _, Ts)), E, _, _, _, Xs, P, Rho, Fs, Es)
    => ngOutside(ngState(P, ngEval(E), Rho, Fs, Es), "constructor_arity_outside_model")
    requires ngTypeCount(Ts) =/=Int ngExprCount(Xs)

  rule ngStep(ngState(P, ngReturn(V), Rho, ngNoFrames(), Es))
    => ngState(P, ngHalted(V), Rho, ngNoFrames(), ngAppendEvent(Es, ngFinished(V)))
  // The bound expression is evaluated in its original environment. Its result
  // enters a new lexical scope; the original binding is restored on body return.
  rule ngStep(ngState(P, ngReturn(V), _,
         ngFrame(ngLetValue(E, O, X, Bound, Body, Saved), Fs), Es))
    => ngState(P, ngEval(Body), Saved[X <- V],
         ngFrame(ngLetScope(E, O, X, Saved), Fs),
         ngAppendEvent(Es, ngLetBound(E, O, X, Bound,
           ngOccurrence(ngFindExpr(Bound, ngNodes(P))), V, Body, Saved, Saved[X <- V])))
  rule ngStep(ngState(P, ngReturn(V), _,
         ngFrame(ngLetScope(E, O, X, Saved), Fs), Es))
    => ngState(P, ngReturn(V), Saved, Fs,
         ngAppendEvent(Es, ngLetBodyReturned(E, O, X, V, Saved)))
  // Only the selected branch is requested, after one condition evaluation.
  syntax NGState ::= ngChooseBranch(NGValue, Int, Int, Int, Int, Int,
                                     NGProgram, Map, NGFrames, NGEvents) [function]
  rule ngStep(ngState(P, ngReturn(V), _,
         ngFrame(ngCondition(E, O, Condition, Yes, No, Saved), Fs), Es))
    => ngChooseBranch(V, E, O, Condition, Yes, No, P, Saved, Fs, Es)
  rule ngChooseBranch(ngBool(true), E, O, Condition, Yes, _, P, Saved, Fs, Es)
    => ngState(P, ngEval(Yes), Saved, Fs,
         ngAppendEvent(Es, ngBranchSelected(E, O, Condition,
           ngOccurrence(ngFindExpr(Condition, ngNodes(P))), true, Yes, Saved)))
  rule ngChooseBranch(ngBool(false), E, O, Condition, _, No, P, Saved, Fs, Es)
    => ngState(P, ngEval(No), Saved, Fs,
         ngAppendEvent(Es, ngBranchSelected(E, O, Condition,
           ngOccurrence(ngFindExpr(Condition, ngNodes(P))), false, No, Saved)))
  rule ngChooseBranch(V, E, O, Condition, Yes, No, P, Saved, Fs, Es)
    => ngOutside(ngState(P, ngReturn(V), Saved,
         ngFrame(ngCondition(E, O, Condition, Yes, No, Saved), Fs), Es),
         "condition_type_outside_model") requires notBool ngIsBool(V)
  rule ngStep(ngState(P, ngReturn(V), _, ngFrame(ngAddLeft(E, O, R, Saved), Fs), Es))
    => ngState(P, ngEval(R), Saved, ngFrame(ngAddRight(E, O, V, Saved), Fs),
               ngAppendEvent(Es, ngAddRightRequested(E, O, V, R, Saved)))
  syntax NGState ::= ngFinishAdd(NGValue, NGValue, Int, Int, NGProgram, Map,
                                 NGFrames, NGEvents) [function]
  rule ngStep(ngState(P, ngReturn(V), _, ngFrame(ngAddRight(E, O, L, Saved), Fs), Es))
    => ngFinishAdd(L, V, E, O, P, Saved, Fs, Es)
  rule ngFinishAdd(ngUInt(L), ngUInt(R), E, O, P, Saved, Fs, Es)
    => ngState(P, ngReturn(ngUInt(L +Int R)), Saved, Fs,
               ngAppendEvent(Es, ngAdded(E, O, ngUInt(L), ngUInt(R), ngUInt(L +Int R))))
    requires L >=Int 0 andBool R >=Int 0
      andBool L +Int R <Int 340282366920938463463374607431768211456
  rule ngFinishAdd(ngUInt(L), ngUInt(R), E, O, P, Saved, Fs, Es)
    => ngOutside(ngState(P, ngReturn(ngUInt(R)), Saved,
                         ngFrame(ngAddRight(E, O, ngUInt(L), Saved), Fs), Es),
                  "addition_outside_UInt128_normal_domain")
    requires L <Int 0 orBool R <Int 0
      orBool L +Int R >=Int 340282366920938463463374607431768211456
  rule ngFinishAdd(L, R, E, O, P, Saved, Fs, Es)
    => ngOutside(ngState(P, ngReturn(R), Saved,
                         ngFrame(ngAddRight(E, O, L, Saved), Fs), Es), "addition_type_outside_model")
    requires notBool (ngIsUInt(L) andBool ngIsUInt(R))

  rule ngStep(ngState(P, ngReturn(V), _,
         ngFrame(ngArguments(E, O, F, I, Active, Prefix, ngExprId(X, Rest), Saved), Fs), Es))
    => ngState(P, ngEval(X), Saved,
         ngFrame(ngArguments(E, O, F, I +Int 1, X, ngAppendValue(Prefix, V), Rest, Saved), Fs),
         ngAppendEvent(Es, ngArgumentReturned(E, O, I, Active, V,
                                               ngAppendValue(Prefix, V), Rest, Saved)))
  rule ngStep(ngState(P, ngReturn(V), _,
         ngFrame(ngArguments(E, O, F, I, Active, Prefix, ngNoExprs(), Saved), Fs), Es))
    => ngState(P, ngApply(E, O, F, ngAppendValue(Prefix, V)), Saved, Fs,
         ngAppendEvent(Es, ngArgumentReturned(E, O, I, Active, V,
                                               ngAppendValue(Prefix, V), ngNoExprs(), Saved)))
  syntax NGState ::= ngEnterFunction(NGFunctionLookup, Int, Int, Int, NGValues,
                                      NGProgram, Map, NGFrames, NGEvents) [function]
                   | ngEnterBindings(NGEnvironment, Int, Int, Int, Int, Int, NGValues,
                                      NGProgram, Map, NGFrames, NGEvents) [function]
  rule ngStep(ngState(P, ngApply(E, O, F, Vs), Rho, Fs, Es))
    => ngEnterFunction(ngFindFunction(F, ngFunctions(P)), E, O, F, Vs, P, Rho, Fs, Es)
  rule ngEnterFunction(ngMissingFunction(), E, O, F, Vs, P, Rho, Fs, Es)
    => ngOutside(ngState(P, ngApply(E, O, F, Vs), Rho, Fs, Es), "missing_function")
  rule ngEnterFunction(ngFoundFunction(ngFunction(F, Ps, T, Body)), E, O, F, Vs,
                        P, Rho, Fs, Es)
    => ngEnterBindings(ngBindParameters(Ps, Vs), E, O, F, T, Body, Vs, P, Rho, Fs, Es)
  rule ngEnterBindings(ngBoundEnvironment(New), E, O, F, T, Body, Vs, P, Rho, Fs, Es)
    => ngState(P, ngEval(Body), New,
         ngFrame(ngFunctionReturn(E, O, F, T, Rho), Fs),
         ngAppendEvent(Es, ngFunctionEntered(E, O, F, Vs, New, T)))
  rule ngEnterBindings(ngBadBindings(), E, O, F, _, _, Vs, P, Rho, Fs, Es)
    => ngOutside(ngState(P, ngApply(E, O, F, Vs), Rho, Fs, Es), "function_binding_outside_model")
  rule ngStep(ngState(P, ngReturn(V), _,
                      ngFrame(ngFunctionReturn(E, O, F, T, Saved), Fs), Es))
    => ngState(P, ngReturn(V), Saved, Fs,
               ngAppendEvent(Es, ngFunctionReturned(E, O, F, V, Saved)))
    requires ngTypeOf(V) ==Int T
  rule ngStep(ngState(P, ngReturn(V), Rho,
                      ngFrame(ngFunctionReturn(E, O, F, T, Saved), Fs), Es))
    => ngOutside(ngState(P, ngReturn(V), Rho,
                         ngFrame(ngFunctionReturn(E, O, F, T, Saved), Fs), Es),
                  "function_return_type_outside_model") requires ngTypeOf(V) =/=Int T

  // The active expression belongs to the caller's field source. Nested callee
  // returns cannot replace its identity with the last callee body expression.
  rule ngStep(ngState(P, ngReturn(V), _,
         ngFrame(ngConstructorFields(E, O, T, C, Ts, I, Active, Prefix,
                                      ngExprId(X, Rest), Saved), Fs), Es))
    => ngState(P, ngEval(X), Saved,
         ngFrame(ngConstructorFields(E, O, T, C, Ts, I +Int 1, X,
                                      ngAppendValue(Prefix, V), Rest, Saved), Fs),
         ngAppendEvent(Es, ngConstructorFieldReturned(E, O, I, Active,
           ngOccurrence(ngFindExpr(Active, ngNodes(P))), V,
           ngAppendValue(Prefix, V), Rest, Saved)))
    requires ngTypeOf(V) ==Int ngTypeAt(Ts, I)
  rule ngStep(ngState(P, ngReturn(V), _,
         ngFrame(ngConstructorFields(E, O, T, C, Ts, I, Active, Prefix,
                                      ngNoExprs(), Saved), Fs), Es))
    => ngState(P, ngMaterialize(E, O, T, C, ngAppendValue(Prefix, V)), Saved, Fs,
         ngAppendEvent(Es, ngConstructorFieldReturned(E, O, I, Active,
           ngOccurrence(ngFindExpr(Active, ngNodes(P))), V,
           ngAppendValue(Prefix, V), ngNoExprs(), Saved)))
    requires ngTypeOf(V) ==Int ngTypeAt(Ts, I)
  rule ngStep(ngState(P, ngReturn(V), Rho,
         ngFrame(ngConstructorFields(E, O, T, C, Ts, I, Active, Prefix, Rest, Saved), Fs), Es))
    => ngOutside(ngState(P, ngReturn(V), Rho,
         ngFrame(ngConstructorFields(E, O, T, C, Ts, I, Active, Prefix, Rest, Saved), Fs), Es),
                  "constructor_field_type_outside_model")
    requires ngTypeOf(V) =/=Int ngTypeAt(Ts, I)
  rule ngStep(ngState(P, ngMaterialize(E, O, T, C, Vs), Rho, Fs, Es))
    => ngState(P, ngReturn(ngNominal(T, C, Vs)), Rho, Fs,
         ngAppendEvent(Es, ngConstructorBuilt(E, O, T, C, ngNominal(T, C, Vs), Rho)))

  rule ngStep(ngState(P, ngReturn(V), _, ngFrame(ngScrutinee(E, O, As, Saved), Fs), Es))
    => ngState(P, ngChoose(E, O, V, As, Saved), Saved, Fs,
               ngAppendEvent(Es, ngScrutineeReturned(E, O, V, As, Saved)))
  syntax NGState ::= ngSelectArm(NGMatching, Int, Int, NGValue, Int, Int, NGArms,
                                  NGProgram, Map, NGFrames, NGEvents) [function]
  rule ngStep(ngState(P, ngChoose(E, O, V, ngArm(A, Pat, Body, Rest), Saved), _, Fs, Es))
    => ngSelectArm(ngMatchPattern(Pat, V, E, A, ngRoot()), E, O, V, A, Body, Rest,
                     P, Saved, Fs, Es)
  rule ngSelectArm(ngMiss(), E, O, V, A, _, Rest, P, Saved, Fs, Es)
    => ngState(P, ngChoose(E, O, V, Rest, Saved), Saved, Fs,
               ngAppendEvent(Es, ngArmMiss(E, O, A, V)))
  rule ngSelectArm(ngMatched(Bs), E, O, _, A, Body, _, P, Saved, Fs, Es)
    => ngState(P, ngEval(Body), ngExtendEnvironment(Bs, Saved),
         ngFrame(ngArmScope(E, O, A, Saved), Fs),
         ngAppendEvent(Es, ngArmSelected(E, O, A, Bs, Body, ngExtendEnvironment(Bs, Saved))))
  rule ngStep(ngState(P, ngChoose(E, O, V, ngNoArms(), Saved), Rho, Fs, Es))
    => ngOutside(ngState(P, ngChoose(E, O, V, ngNoArms(), Saved), Rho, Fs, Es),
                  "no_matching_arm_in_selected_normal_model")
  rule ngStep(ngState(P, ngReturn(V), _, ngFrame(ngArmScope(E, O, A, Saved), Fs), Es))
    => ngState(P, ngReturn(V), Saved, Fs,
               ngAppendEvent(Es, ngArmBodyReturned(E, O, A, V, Saved)))
endmodule
