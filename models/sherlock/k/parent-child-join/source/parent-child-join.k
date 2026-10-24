requires "domains.md"

module SHERLOCK-RECURSION-PARENT-CHILD-JOIN
  imports INT
  imports BOOL
  imports STRING
  imports K-EQUAL

  // Positive mathematical lane model, not a byte receiver or proof verifier.
  // F-canonicality, pinned layout completeness, relation/key validation and
  // actual Poseidon interpretation are external. No cryptographic claim.
  syntax RJFields ::= rjNil() | rjCons(Int, RJFields)
  syntax RJContexts ::= rjContexts(Int, Int, Int, Int)
  syntax RJAtom ::= rjScalar(Int) | rjCommit(String, RJAtoms)
  syntax RJAtoms ::= rjNoAtoms() | rjAtom(RJAtom, RJAtoms)
  syntax RJResult ::= rjUnlinked() | rjLinked(RJAtoms)

  syntax Int ::= rjLength(RJFields) [function]
  rule rjLength(rjNil()) => 0
  rule rjLength(rjCons(_, Tail)) => 1 +Int rjLength(Tail)
  syntax Int ::= rjAt(Int, RJFields) [function]
  rule rjAt(0, rjCons(Head, _)) => Head
  rule rjAt(N, rjCons(_, Tail)) => rjAt(N -Int 1, Tail)
    requires N >Int 0
  syntax RJFields ::= rjDrop(Int, RJFields) [function]
  rule rjDrop(0, Fields) => Fields
  rule rjDrop(N, rjCons(_, Tail)) => rjDrop(N -Int 1, Tail)
    requires N >Int 0
  syntax RJFields ::= rjTake(Int, RJFields) [function]
  rule rjTake(0, _) => rjNil()
  rule rjTake(N, rjCons(Head, Tail))
    => rjCons(Head, rjTake(N -Int 1, Tail)) requires N >Int 0
  syntax RJAtoms ::= rjAsAtoms(RJFields) [function]
  rule rjAsAtoms(rjNil()) => rjNoAtoms()
  rule rjAsAtoms(rjCons(Head, Tail))
    => rjAtom(rjScalar(Head), rjAsAtoms(Tail))

  // link_offsets: binding0, context1..3, program3..3+P, then two S blocks,
  // then rule_tag/charge. Width and full lengths are checked before indexing.
  syntax Bool ::= rjShape(Int, Int, RJFields, RJFields) [function]
  rule rjShape(P, S, A, B)
    => P >Int 0 andBool S >Int 0
       andBool P +Int (2 *Int S) +Int 5 <=Int 16383
       andBool rjLength(A) ==Int P +Int (2 *Int S) +Int 5
       andBool rjLength(B) ==Int P +Int (2 *Int S) +Int 5
  syntax Bool ::= rjHeight(Int) [function]
  rule rjHeight(H) => H >=Int 0 andBool H <=Int 18446744073709551613

  syntax RJResult ::= rjObserve(Int, Int, Int, RJContexts, Int,
                                RJFields, RJFields) [function]
  rule rjObserve(Keys, P, S, Contexts, H, A, B)
    => rjReady(rjShape(P, S, A, B), rjHeight(H), Keys, P, S,
               Contexts, H, A, B)
  syntax RJResult ::= rjReady(Bool, Bool, Int, Int, Int, RJContexts,
                              Int, RJFields, RJFields) [function]
  rule rjReady(false, _, _, _, _, _, _, _, _) => rjUnlinked()
  rule rjReady(true, false, _, _, _, _, _, _, _) => rjUnlinked()
  rule rjReady(true, true, Keys, P, S, Contexts, H, A, B)
    => rjChoose(rjLanesMatch(P, S, Contexts, A, B), Keys, P, S,
                Contexts, H, A, B)

  // Complete ordered equality, not equality of one digest/root/header lane.
  // Bindings are intentionally not equated. Contexts are per-child pins.
  syntax Bool ::= rjLanesMatch(Int, Int, RJContexts,
                               RJFields, RJFields) [function]
  rule rjLanesMatch(P, S, rjContexts(A0, A1, B0, B1), A, B)
    => rjAt(1, A) ==Int A0 andBool rjAt(2, A) ==Int A1
       andBool rjAt(1, B) ==Int B0 andBool rjAt(2, B) ==Int B1
       andBool (rjTake(P, rjDrop(3, A)) ==K rjTake(P, rjDrop(3, B)))
       andBool (rjTake(S, rjDrop(3 +Int P +Int S, A))
                  ==K rjTake(S, rjDrop(3 +Int P, B)))
       andBool rjAt(3 +Int P +Int (2 *Int S), A) ==Int 3
       andBool rjAt(4 +Int P +Int (2 *Int S), A) ==Int 1
       andBool rjAt(3 +Int P +Int (2 *Int S), B) ==Int 0
       andBool rjAt(4 +Int P +Int (2 *Int S), B) ==Int 0

  syntax RJResult ::= rjChoose(Bool, Int, Int, Int, RJContexts,
                               Int, RJFields, RJFields) [function]
  rule rjChoose(false, _, _, _, _, _, _, _) => rjUnlinked()
  rule rjChoose(true, Keys, P, S, Contexts, H, A, B)
    => rjAssemble(Keys, Contexts,
         rjCommit("SHK_PROG", rjAsAtoms(rjTake(P, rjDrop(3, A)))),
         rjCommit("SHK_STATE", rjAsAtoms(rjTake(S, rjDrop(3 +Int P, A)))),
         rjCommit("SHK_STATE", rjAsAtoms(rjTake(S, rjDrop(3 +Int P +Int S, A)))),
         rjCommit("SHK_STATE", rjAsAtoms(rjTake(S, rjDrop(3 +Int P +Int S, B)))),
         H, rjAt(0, A), rjAt(0, B),
         rjAt(4 +Int P +Int (2 *Int S), A)
           +Int rjAt(4 +Int P +Int (2 *Int S), B))

  // rjCommit is DATA: no rule computes Poseidon. Its interpretation must
  // separately use dom(tag) and all ordered arguments in the actual field.
  syntax RJResult ::= rjAssemble(Int, RJContexts, RJAtom, RJAtom, RJAtom,
                                 RJAtom, Int, Int, Int, Int) [function]
  rule rjAssemble(Keys, Contexts, Program, Pre, Mid, Post, H, BindA, BindB, Work)
    => rjPublic(Keys, Contexts, Program, Pre, Mid, Post, H, BindA, BindB,
         Work, rjTrace(Keys, Contexts, Program, Pre, Mid, Post, H, BindA, BindB, Work))
  syntax RJAtom ::= rjTrace(Int, RJContexts, RJAtom, RJAtom, RJAtom,
                            RJAtom, Int, Int, Int, Int) [function]
  rule rjTrace(Keys, rjContexts(A0, A1, B0, B1), Program, Pre, Mid, Post,
               H, BindA, BindB, Work)
    => rjCommit("SHK_TRACE", rjAtom(rjScalar(Keys), rjAtom(rjScalar(A0), rjAtom(rjScalar(A1), rjAtom(rjScalar(B0), rjAtom(rjScalar(B1), rjAtom(Program, rjAtom(Pre, rjAtom(rjScalar(H), rjAtom(rjScalar(BindA), rjAtom(rjScalar(3), rjAtom(rjScalar(1), rjAtom(Mid, rjAtom(rjScalar(H +Int 1), rjAtom(rjScalar(BindB), rjAtom(rjScalar(0), rjAtom(rjScalar(0), rjAtom(Post, rjAtom(rjScalar(H +Int 2), rjAtom(rjScalar(2), rjAtom(rjScalar(Work), rjNoAtoms())))))))))))))))))))))
  syntax RJResult ::= rjPublic(Int, RJContexts, RJAtom, RJAtom, RJAtom,
                               RJAtom, Int, Int, Int, Int, RJAtom) [function]
  rule rjPublic(Keys, rjContexts(A0, A1, B0, B1), Program, Pre, Mid, Post,
                H, BindA, BindB, Work, Trace)
    => rjLinked(rjAtom(rjScalar(Keys), rjAtom(rjScalar(A0), rjAtom(rjScalar(A1), rjAtom(rjScalar(B0), rjAtom(rjScalar(B1), rjAtom(Program, rjAtom(Pre, rjAtom(rjScalar(H), rjAtom(Mid, rjAtom(Post, rjAtom(rjScalar(H +Int 2), rjAtom(rjScalar(BindA), rjAtom(rjScalar(BindB), rjAtom(rjScalar(2), rjAtom(rjScalar(Work), rjAtom(Trace, rjNoAtoms())))))))))))))))))
endmodule
