requires "value-slots.k"

module P3-VALUE-SLOTS-NORMAL-CHECKS
  imports P3-VALUE-SLOTS-FUNCTIONS

  // Checks are unexecuted source. Equality here is exact reference/row equality,
  // never equality of the values denoted by Constant/Data references.
  syntax Bool ::= vsAgreement(VSBounds, VSState, Int, VSRef, VSResult) [function]
  rule vsAgreement(B, vsState(C, A, Pre, H0, H1), 0, R,
                   vsAppended(_, vsState(C, N, Post, Next, H1)))
    => vsTake(Pre, A) ==K vsTake(Post, A)
       andBool vsMaterialize(Pre, H1) ==K vsMaterialize(Post, H1)
       andBool vsSourceAppend(vsMaterialize(Pre, H0), R) ==K vsMaterialize(Post, Next)
       andBool vsDomain(B, vsState(C, N, Post, Next, H1), 0, R)
  rule vsAgreement(B, vsState(C, A, Pre, H0, H1), 1, R,
                   vsAppended(_, vsState(C, N, Post, H0, Next)))
    => vsTake(Pre, A) ==K vsTake(Post, A)
       andBool vsMaterialize(Pre, H0) ==K vsMaterialize(Post, H0)
       andBool vsSourceAppend(vsMaterialize(Pre, H1), R) ==K vsMaterialize(Post, Next)
       andBool vsDomain(B, vsState(C, N, Post, H0, Next), 1, R)

  // empty-zero-origin; hand-derived expected.json case 0.
  syntax Bool ::= vsNormal0() [function]
  rule vsNormal0()
    => (vsAttempt(vsBounds(4,4), vsState(1, 0, vsCons(vsRef(0,0), vsNil()), vsSlice(0,0), vsSlice(0,0)), 0, vsRef(0,2)) ==K vsAppended(vsCOPY(), vsState(1, 1, vsCons(vsRef(0,2), vsNil()), vsSlice(0,1), vsSlice(0,0))))
       andBool vsAgreement(vsBounds(4,4), vsState(1, 0, vsCons(vsRef(0,0), vsNil()), vsSlice(0,0), vsSlice(0,0)), 0, vsRef(0,2), vsAttempt(vsBounds(4,4), vsState(1, 0, vsCons(vsRef(0,0), vsNil()), vsSlice(0,0), vsSlice(0,0)), 0, vsRef(0,2)))

  // ex4-1-outer-empty-copy; hand-derived expected.json case 1.
  syntax Bool ::= vsNormal1() [function]
  rule vsNormal1()
    => (vsAttempt(vsBounds(4,4), vsState(9, 4, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(0,0), vsSlice(0,0)), 0, vsRef(0,2)) ==K vsAppended(vsCOPY(), vsState(9, 5, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(0,0))))
       andBool vsAgreement(vsBounds(4,4), vsState(9, 4, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(0,0), vsSlice(0,0)), 0, vsRef(0,2), vsAttempt(vsBounds(4,4), vsState(9, 4, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(0,0), vsSlice(0,0)), 0, vsRef(0,2)))

  // ex4-2-inner-empty-copy; hand-derived expected.json case 2.
  syntax Bool ::= vsNormal2() [function]
  rule vsNormal2()
    => (vsAttempt(vsBounds(4,4), vsState(9, 5, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(0,0)), 1, vsRef(1,2)) ==K vsAppended(vsCOPY(), vsState(9, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(5,1))))
       andBool vsAgreement(vsBounds(4,4), vsState(9, 5, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(0,0)), 1, vsRef(1,2), vsAttempt(vsBounds(4,4), vsState(9, 5, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(0,0)), 1, vsRef(1,2)))

  // ex4-3-outer-copy-after-inner; hand-derived expected.json case 3.
  syntax Bool ::= vsNormal3() [function]
  rule vsNormal3()
    => (vsAttempt(vsBounds(4,4), vsState(9, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(5,1)), 0, vsRef(1,3)) ==K vsAppended(vsCOPY(), vsState(9, 8, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(6,2), vsSlice(5,1))))
       andBool vsAgreement(vsBounds(4,4), vsState(9, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(5,1)), 0, vsRef(1,3), vsAttempt(vsBounds(4,4), vsState(9, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(4,1), vsSlice(5,1)), 0, vsRef(1,3)))

  // ex4-4-outer-tail-extend; hand-derived expected.json case 4.
  syntax Bool ::= vsNormal4() [function]
  rule vsNormal4()
    => (vsAttempt(vsBounds(4,4), vsState(9, 8, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(6,2), vsSlice(5,1)), 0, vsRef(0,3)) ==K vsAppended(vsEXT(), vsState(9, 9, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,3), vsNil()))))))))), vsSlice(6,3), vsSlice(5,1))))
       andBool vsAgreement(vsBounds(4,4), vsState(9, 8, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(6,2), vsSlice(5,1)), 0, vsRef(0,3), vsAttempt(vsBounds(4,4), vsState(9, 8, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsNil()))))))))), vsSlice(6,2), vsSlice(5,1)), 0, vsRef(0,3)))

  // ex4-3-copy-capacity-refusal; hand-derived expected.json case 5.
  syntax Bool ::= vsNormal5() [function]
  rule vsNormal5()
    => (vsAttempt(vsBounds(4,4), vsState(7, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,0), vsNil()))))))), vsSlice(4,1), vsSlice(5,1)), 0, vsRef(1,3)) ==K vsCapacity(8, 7, 7))

  // ex4-4-extend-capacity-refusal; hand-derived expected.json case 6.
  syntax Bool ::= vsNormal6() [function]
  rule vsNormal6()
    => (vsAttempt(vsBounds(4,4), vsState(8, 8, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsNil())))))))), vsSlice(6,2), vsSlice(5,1)), 0, vsRef(0,3)) ==K vsCapacity(9, 8, 8))

  // coholder-first-extends; hand-derived expected.json case 7.
  syntax Bool ::= vsNormal7() [function]
  rule vsNormal7()
    => (vsAttempt(vsBounds(4,4), vsState(8, 5, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil())))))))), vsSlice(4,1), vsSlice(4,1)), 0, vsRef(1,3)) ==K vsAppended(vsEXT(), vsState(8, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil())))))))), vsSlice(4,2), vsSlice(4,1))))
       andBool vsAgreement(vsBounds(4,4), vsState(8, 5, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil())))))))), vsSlice(4,1), vsSlice(4,1)), 0, vsRef(1,3), vsAttempt(vsBounds(4,4), vsState(8, 5, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil())))))))), vsSlice(4,1), vsSlice(4,1)), 0, vsRef(1,3)))

  // coholder-second-copies; hand-derived expected.json case 8.
  syntax Bool ::= vsNormal8() [function]
  rule vsNormal8()
    => (vsAttempt(vsBounds(4,4), vsState(8, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil())))))))), vsSlice(4,2), vsSlice(4,1)), 1, vsRef(1,2)) ==K vsAppended(vsCOPY(), vsState(8, 8, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,2), vsCons(vsRef(1,2), vsNil())))))))), vsSlice(4,2), vsSlice(6,2))))
       andBool vsAgreement(vsBounds(4,4), vsState(8, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil())))))))), vsSlice(4,2), vsSlice(4,1)), 1, vsRef(1,2), vsAttempt(vsBounds(4,4), vsState(8, 6, vsCons(vsRef(0,0), vsCons(vsRef(1,0), vsCons(vsRef(0,1), vsCons(vsRef(1,1), vsCons(vsRef(0,2), vsCons(vsRef(1,3), vsCons(vsRef(0,0), vsCons(vsRef(0,0), vsNil())))))))), vsSlice(4,2), vsSlice(4,1)), 1, vsRef(1,2)))

  syntax Bool ::= vsAllNormal() [function]
  rule vsAllNormal() => vsNormal0() andBool vsNormal1() andBool vsNormal2() andBool vsNormal3() andBool vsNormal4() andBool vsNormal5() andBool vsNormal6() andBool vsNormal7() andBool vsNormal8()
endmodule
