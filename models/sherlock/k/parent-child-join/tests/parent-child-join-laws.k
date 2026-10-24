requires "../source/parent-child-join.k"

module SHERLOCK-RECURSION-PARENT-CHILD-JOIN-LAWS
  imports SHERLOCK-RECURSION-PARENT-CHILD-JOIN
  // Handcomputed full outputs frozen before producer; UNRUN.

  // complete-ordered-link
  syntax Bool ::= rjLaw1() [function]
  rule rjLaw1() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjLinked(rjAtom(rjScalar(55), rjAtom(rjScalar(101), rjAtom(rjScalar(102), rjAtom(rjScalar(201), rjAtom(rjScalar(202), rjAtom(rjCommit("SHK_PROG", rjAtom(rjScalar(7), rjAtom(rjScalar(8), rjNoAtoms()))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(1), rjAtom(rjScalar(2), rjAtom(rjScalar(3), rjNoAtoms())))), rjAtom(rjScalar(5), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(4), rjAtom(rjScalar(5), rjAtom(rjScalar(6), rjNoAtoms())))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(9), rjAtom(rjScalar(10), rjAtom(rjScalar(11), rjNoAtoms())))), rjAtom(rjScalar(7), rjAtom(rjScalar(11), rjAtom(rjScalar(22), rjAtom(rjScalar(2), rjAtom(rjScalar(1), rjAtom(rjCommit("SHK_TRACE", rjAtom(rjScalar(55), rjAtom(rjScalar(101), rjAtom(rjScalar(102), rjAtom(rjScalar(201), rjAtom(rjScalar(202), rjAtom(rjCommit("SHK_PROG", rjAtom(rjScalar(7), rjAtom(rjScalar(8), rjNoAtoms()))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(1), rjAtom(rjScalar(2), rjAtom(rjScalar(3), rjNoAtoms())))), rjAtom(rjScalar(5), rjAtom(rjScalar(11), rjAtom(rjScalar(3), rjAtom(rjScalar(1), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(4), rjAtom(rjScalar(5), rjAtom(rjScalar(6), rjNoAtoms())))), rjAtom(rjScalar(6), rjAtom(rjScalar(22), rjAtom(rjScalar(0), rjAtom(rjScalar(0), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(9), rjAtom(rjScalar(10), rjAtom(rjScalar(11), rjNoAtoms())))), rjAtom(rjScalar(7), rjAtom(rjScalar(2), rjAtom(rjScalar(1), rjNoAtoms()))))))))))))))))))))), rjNoAtoms())))))))))))))))))

  // different-final-program-lane
  syntax Bool ::= rjLaw2() [function]
  rule rjLaw2() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(99, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()

  // different-final-shared-state-lane
  syntax Bool ::= rjLaw3() [function]
  rule rjLaw3() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(60, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()

  // wrong-child-context
  syntax Bool ::= rjLaw4() [function]
  rule rjLaw4() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(101, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()

  // source-line-is-not-wire-rule-tag
  syntax Bool ::= rjLaw5() [function]
  rule rjLaw5() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(102, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()

  // wrong-a80-charge
  syntax Bool ::= rjLaw6() [function]
  rule rjLaw6() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(1, rjNil())))))))))))))) ==K rjUnlinked()

  // reversed-child-order
  syntax Bool ::= rjLaw7() [function]
  rule rjLaw7() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil()))))))))))))), rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil())))))))))))))) ==K rjUnlinked()

  // short-child
  syntax Bool ::= rjLaw8() [function]
  rule rjLaw8() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjNil())))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()

  // extra-child-lane
  syntax Bool ::= rjLaw9() [function]
  rule rjLaw9() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjCons(0, rjNil()))))))))))))))) ==K rjUnlinked()

  // zero-program-width
  syntax Bool ::= rjLaw10() [function]
  rule rjLaw10() => rjObserve(55, 0, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()

  // height-overflow
  syntax Bool ::= rjLaw11() [function]
  rule rjLaw11() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 18446744073709551614, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()

  // last-permitted-height
  syntax Bool ::= rjLaw12() [function]
  rule rjLaw12() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 18446744073709551613, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjLinked(rjAtom(rjScalar(55), rjAtom(rjScalar(101), rjAtom(rjScalar(102), rjAtom(rjScalar(201), rjAtom(rjScalar(202), rjAtom(rjCommit("SHK_PROG", rjAtom(rjScalar(7), rjAtom(rjScalar(8), rjNoAtoms()))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(1), rjAtom(rjScalar(2), rjAtom(rjScalar(3), rjNoAtoms())))), rjAtom(rjScalar(18446744073709551613), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(4), rjAtom(rjScalar(5), rjAtom(rjScalar(6), rjNoAtoms())))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(9), rjAtom(rjScalar(10), rjAtom(rjScalar(11), rjNoAtoms())))), rjAtom(rjScalar(18446744073709551615), rjAtom(rjScalar(11), rjAtom(rjScalar(22), rjAtom(rjScalar(2), rjAtom(rjScalar(1), rjAtom(rjCommit("SHK_TRACE", rjAtom(rjScalar(55), rjAtom(rjScalar(101), rjAtom(rjScalar(102), rjAtom(rjScalar(201), rjAtom(rjScalar(202), rjAtom(rjCommit("SHK_PROG", rjAtom(rjScalar(7), rjAtom(rjScalar(8), rjNoAtoms()))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(1), rjAtom(rjScalar(2), rjAtom(rjScalar(3), rjNoAtoms())))), rjAtom(rjScalar(18446744073709551613), rjAtom(rjScalar(11), rjAtom(rjScalar(3), rjAtom(rjScalar(1), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(4), rjAtom(rjScalar(5), rjAtom(rjScalar(6), rjNoAtoms())))), rjAtom(rjScalar(18446744073709551614), rjAtom(rjScalar(22), rjAtom(rjScalar(0), rjAtom(rjScalar(0), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(9), rjAtom(rjScalar(10), rjAtom(rjScalar(11), rjNoAtoms())))), rjAtom(rjScalar(18446744073709551615), rjAtom(rjScalar(2), rjAtom(rjScalar(1), rjNoAtoms()))))))))))))))))))))), rjNoAtoms())))))))))))))))))

  // distinct-binding-retained
  syntax Bool ::= rjLaw13() [function]
  rule rjLaw13() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), 5, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(33, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjLinked(rjAtom(rjScalar(55), rjAtom(rjScalar(101), rjAtom(rjScalar(102), rjAtom(rjScalar(201), rjAtom(rjScalar(202), rjAtom(rjCommit("SHK_PROG", rjAtom(rjScalar(7), rjAtom(rjScalar(8), rjNoAtoms()))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(1), rjAtom(rjScalar(2), rjAtom(rjScalar(3), rjNoAtoms())))), rjAtom(rjScalar(5), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(4), rjAtom(rjScalar(5), rjAtom(rjScalar(6), rjNoAtoms())))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(9), rjAtom(rjScalar(10), rjAtom(rjScalar(11), rjNoAtoms())))), rjAtom(rjScalar(7), rjAtom(rjScalar(11), rjAtom(rjScalar(33), rjAtom(rjScalar(2), rjAtom(rjScalar(1), rjAtom(rjCommit("SHK_TRACE", rjAtom(rjScalar(55), rjAtom(rjScalar(101), rjAtom(rjScalar(102), rjAtom(rjScalar(201), rjAtom(rjScalar(202), rjAtom(rjCommit("SHK_PROG", rjAtom(rjScalar(7), rjAtom(rjScalar(8), rjNoAtoms()))), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(1), rjAtom(rjScalar(2), rjAtom(rjScalar(3), rjNoAtoms())))), rjAtom(rjScalar(5), rjAtom(rjScalar(11), rjAtom(rjScalar(3), rjAtom(rjScalar(1), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(4), rjAtom(rjScalar(5), rjAtom(rjScalar(6), rjNoAtoms())))), rjAtom(rjScalar(6), rjAtom(rjScalar(33), rjAtom(rjScalar(0), rjAtom(rjScalar(0), rjAtom(rjCommit("SHK_STATE", rjAtom(rjScalar(9), rjAtom(rjScalar(10), rjAtom(rjScalar(11), rjNoAtoms())))), rjAtom(rjScalar(7), rjAtom(rjScalar(2), rjAtom(rjScalar(1), rjNoAtoms()))))))))))))))))))))), rjNoAtoms())))))))))))))))))

  // negative-logical-height
  syntax Bool ::= rjLaw14() [function]
  rule rjLaw14() => rjObserve(55, 2, 3, rjContexts(101, 102, 201, 202), -1, rjCons(11, rjCons(101, rjCons(102, rjCons(7, rjCons(8, rjCons(1, rjCons(2, rjCons(3, rjCons(4, rjCons(5, rjCons(6, rjCons(3, rjCons(1, rjNil()))))))))))))), rjCons(22, rjCons(201, rjCons(202, rjCons(7, rjCons(8, rjCons(4, rjCons(5, rjCons(6, rjCons(9, rjCons(10, rjCons(11, rjCons(0, rjCons(0, rjNil())))))))))))))) ==K rjUnlinked()
endmodule
