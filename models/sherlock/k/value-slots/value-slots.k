requires "domains.md"

module P3-VALUE-SLOTS-SYNTAX
  imports DOMAINS-SYNTAX

  // Logical references only: 0 = Constant, 1 = Data. No semantic Eq.
  syntax VSRef ::= vsRef(Int, Int)
  syntax VSRefs ::= vsNil() | vsCons(VSRef, VSRefs)
  syntax VSSlice ::= vsSlice(Int, Int)
  // capacity, activeLength, complete padded table, holder0, holder1
  syntax VSState ::= vsState(Int, Int, VSRefs, VSSlice, VSSlice)
  // Immutable reference-domain parameters, not additional StateBlock columns.
  syntax VSBounds ::= vsBounds(Int, Int)
  syntax VSBranch ::= vsEXT() | vsCOPY()
  // A tooling relation result, never a MachineOutcome or accepted StepRecord.
  syntax VSResult ::= vsAppended(VSBranch, VSState)
                    | vsCapacity(Int, Int, Int)
  // bounds, preState, acting holder (0 or 1), reference to append
  syntax VSRequest ::= vsRequest(VSBounds, VSState, Int, VSRef)
  syntax KItem ::= VSRequest | VSResult
endmodule

module P3-VALUE-SLOTS-FUNCTIONS
  imports P3-VALUE-SLOTS-SYNTAX
  imports DOMAINS

  syntax Int ::= vsLength(VSRefs) [function]
  rule vsLength(vsNil()) => 0
  rule vsLength(vsCons(_, T)) => 1 +Int vsLength(T)

  syntax Bool ::= vsRefValid(VSRef, VSBounds) [function]
  rule vsRefValid(vsRef(S, I), vsBounds(K, D))
    => I >=Int 0 andBool ((S ==Int 0 andBool I <Int K)
                          orBool (S ==Int 1 andBool I <Int D))

  syntax Bool ::= vsZeros(VSRefs) [function]
  rule vsZeros(vsNil()) => true
  rule vsZeros(vsCons(vsRef(S, I), T))
    => S ==Int 0 andBool I ==Int 0 andBool vsZeros(T)

  syntax Bool ::= vsRowsValid(VSRefs, Int, VSBounds) [function]
  rule vsRowsValid(T, 0, _) => vsZeros(T)
  rule vsRowsValid(vsCons(R, T), A, B)
    => vsRefValid(R, B) andBool vsRowsValid(T, A -Int 1, B)
    requires A >Int 0
  rule vsRowsValid(vsNil(), A, _) => false requires A >Int 0
  rule vsRowsValid(_, A, _) => false requires A <Int 0

  syntax Bool ::= vsSliceValid(VSSlice, Int) [function]
  rule vsSliceValid(vsSlice(S, M), A)
    => (M ==Int 0 andBool S ==Int 0)
       orBool (M >Int 0 andBool S >=Int 0 andBool S +Int M <=Int A)

  syntax Bool ::= vsDomain(VSBounds, VSState, Int, VSRef) [function]
  rule vsDomain(vsBounds(K, D), vsState(C, A, T, H0, H1), I, R)
    => K >=Int 0 andBool D >=Int 0 andBool C >=Int 0
       andBool A >=Int 0 andBool A <=Int C andBool vsLength(T) ==Int C
       andBool vsRowsValid(T, A, vsBounds(K, D))
       andBool vsSliceValid(H0, A) andBool vsSliceValid(H1, A)
       andBool (I ==Int 0 orBool I ==Int 1)
       andBool vsRefValid(R, vsBounds(K, D))

  // Source sequence oracle: structural append, no physical selector/update call.
  syntax VSRefs ::= vsSourceAppend(VSRefs, VSRef) [function]
  rule vsSourceAppend(vsNil(), R) => vsCons(R, vsNil())
  rule vsSourceAppend(vsCons(H, T), R) => vsCons(H, vsSourceAppend(T, R))

  // Observation helpers use prefix/drop, independently of indexed row updates.
  syntax VSRefs ::= vsTake(VSRefs, Int) [function]
                 | vsDrop(VSRefs, Int) [function]
                 | vsMaterialize(VSRefs, VSSlice) [function]
  rule vsTake(_, 0) => vsNil()
  rule vsTake(vsCons(H, T), N) => vsCons(H, vsTake(T, N -Int 1))
    requires N >Int 0
  rule vsDrop(T, 0) => T
  rule vsDrop(vsCons(_, T), N) => vsDrop(T, N -Int 1) requires N >Int 0
  rule vsMaterialize(T, vsSlice(S, M)) => vsTake(vsDrop(T, S), M)

  syntax VSSlice ::= vsHolder(VSState, Int) [function]
  rule vsHolder(vsState(_, _, _, H0, _), 0) => H0
  rule vsHolder(vsState(_, _, _, _, H1), 1) => H1

  syntax VSBranch ::= vsSelect(VSSlice, Int) [function]
  rule vsSelect(vsSlice(S, M), A) => vsEXT()
    requires M >Int 0 andBool S +Int M ==Int A
  rule vsSelect(vsSlice(S, M), A) => vsCOPY()
    requires notBool (M >Int 0 andBool S +Int M ==Int A)

  syntax Int ::= vsDelta(VSBranch, VSSlice) [function]
  rule vsDelta(vsEXT(), _) => 1
  rule vsDelta(vsCOPY(), vsSlice(_, M)) => M +Int 1
  syntax VSSlice ::= vsNextSlice(VSBranch, VSSlice, Int) [function]
  rule vsNextSlice(vsEXT(), vsSlice(S, M), _) => vsSlice(S, M +Int 1)
  rule vsNextSlice(vsCOPY(), vsSlice(_, M), A) => vsSlice(A, M +Int 1)

  syntax VSRef ::= vsAt(VSRefs, Int) [function]
  rule vsAt(vsCons(R, _), 0) => R
  rule vsAt(vsCons(_, T), N) => vsAt(T, N -Int 1) requires N >Int 0
  syntax VSRefs ::= vsSet(VSRefs, Int, VSRef) [function]
  rule vsSet(vsCons(_, T), 0, R) => vsCons(R, T)
  rule vsSet(vsCons(H, T), N, R) => vsCons(H, vsSet(T, N -Int 1, R))
    requires N >Int 0

  // Read all copied rows from Original, never from a partially updated table.
  // Arguments: Original, destination table, source index, rows left, dest index, R.
  syntax VSRefs ::= vsCopyRows(VSRefs, VSRefs, Int, Int, Int, VSRef) [function]
  rule vsCopyRows(_, T, _, 0, N, R) => vsSet(T, N, R)
  rule vsCopyRows(Original, T, S, M, N, R)
    => vsCopyRows(Original, vsSet(T, N, vsAt(Original, S)),
                  S +Int 1, M -Int 1, N +Int 1, R)
    requires M >Int 0
  syntax VSRefs ::= vsWrite(VSBranch, VSRefs, VSSlice, Int, VSRef) [function]
  rule vsWrite(vsEXT(), T, _, A, R) => vsSet(T, A, R)
  rule vsWrite(vsCOPY(), T, vsSlice(S, M), A, R) => vsCopyRows(T, T, S, M, A, R)

  syntax VSState ::= vsReplace(VSState, Int, Int, VSRefs, VSSlice) [function]
  rule vsReplace(vsState(C, _, _, _, H1), 0, A, T, H)
    => vsState(C, A, T, H, H1)
  rule vsReplace(vsState(C, _, _, H0, _), 1, A, T, H)
    => vsState(C, A, T, H0, H)

  syntax VSResult ::= vsAttempt(VSBounds, VSState, Int, VSRef) [function]
                    | vsPlan(VSState, Int, VSRef, VSSlice, VSBranch) [function]
  rule vsAttempt(B, vsState(C, A, T, H0, H1), I, R)
    => vsPlan(vsState(C, A, T, H0, H1), I, R,
              vsHolder(vsState(C, A, T, H0, H1), I),
              vsSelect(vsHolder(vsState(C, A, T, H0, H1), I), A))
    requires vsDomain(B, vsState(C, A, T, H0, H1), I, R)
  rule vsPlan(vsState(C, A, T, H0, H1), I, R, H, Branch)
    => vsAppended(Branch,
         vsReplace(vsState(C, A, T, H0, H1), I, A +Int vsDelta(Branch, H),
                   vsWrite(Branch, T, H, A, R), vsNextSlice(Branch, H, A)))
    requires A +Int vsDelta(Branch, H) <=Int C
  // needed, limit, first overflowing ValueSlots index. No post-image constructor.
  rule vsPlan(vsState(C, A, _, _, _), _, _, H, Branch)
    => vsCapacity(A +Int vsDelta(Branch, H), C, C)
    requires A +Int vsDelta(Branch, H) >Int C
endmodule

module P3-VALUE-SLOTS
  imports P3-VALUE-SLOTS-FUNCTIONS
  configuration <k> $PGM:VSRequest </k>

  // The only transition rule. Invalid-domain requests have no transition here.
  // This tooling rewrite does not consume fuel or produce an A machine event.
  rule <k> vsRequest(B, S, I, R) => vsAttempt(B, S, I, R) </k>
    requires vsDomain(B, S, I, R)
endmodule
