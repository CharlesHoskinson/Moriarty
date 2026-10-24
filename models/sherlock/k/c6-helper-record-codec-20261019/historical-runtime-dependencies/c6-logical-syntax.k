requires "nominal-product-syntax.k"

module SHERLOCK-C6-LOGICAL-SYNTAX
  imports SHERLOCK-NOMINAL-PRODUCT-SYNTAX
  syntax C6Raw ::= c6Raw(String)
  syntax C6Env ::= c6None() | c6Some(Int)
  syntax C6Ref ::= c6Const(Int) | c6Data(Int)
  syntax C6Slice ::= c6Slice(Int, Int)
  syntax C6Frame ::= c6LetF(Int, Int, C6Env, Int)
                   | c6LeftF(String, Int, C6Env, Int)
                   | c6RightF(String, C6Ref, Int)
                   | c6UnaryF(String, Int)
                   | c6IfF(Int, Int, C6Env, Int)
                   | c6MatchF(C6Slice, C6Env, Int)
                   | c6ArgsF(Int, C6Slice, C6Slice, C6Env, Int)
                   | c6ReturnF(Int, C6Env, Int)
                   | c6ScopeF(C6Env, Int)
                   | c6ConstructorFieldsF(Int, Int, Int, Int, Bool, C6Slice,
                                          C6Slice, C6Env, Int)
  // List display order is Core bottom-to-top; the final element is active.
  syntax C6Frames ::= c6NoFrames() | c6Frame(C6Frame, C6Frames)
  syntax C6Binding ::= c6BindingBad() | c6BindingOk(C6Env)
  syntax C6Matching ::= c6NoMatch() | c6Matched(C6Env)
  syntax C6Primitive ::= c6PrimValue(C6Ref) | c6PrimError(Int)
  syntax C6Control ::= c6Eval(Int) | c6Return(C6Ref)
                     | c6ScheduleNext(C6Frame, Int, C6Env)
                     | c6Invoke(Int, C6Slice) | c6Enter(C6Env, C6Slice)
                     | c6Bound(C6Binding, Int, Int)
                     | c6Cases(C6Ref, C6Slice, C6Env)
                     | c6CaseResult(C6Matching, C6Ref, Int, C6Slice, C6Env)
                     | c6PrimResult(C6Primitive) | c6Error(Int)
                     | c6Materialize(Int, Int, C6Slice)
  syntax C6Result ::= c6NoResult() | c6ScalarTypeTest(Bool) | c6AppendDone(C6Slice)
  // Full raw program; control; source ExprId; current env; raw-order frames;
  // completed helper result; retained raw residual (NOT post-step counters).
  syntax C6Boundary ::= c6Boundary(C6Raw, C6Control, Int, C6Env, C6Frames,
                                   C6Result, C6Raw)
  // Owner-checked next child, next field type, arity; final arity.
  syntax C6Read ::= c6NoRead() | c6NextRead(Int, Int, Int) | c6FinalRead(Int)
  syntax C6Answer ::= c6Applied(String, C6Boundary)
                    | c6Unsupported(String, C6Boundary) | c6Kept(C6Boundary)
                    | c6MatchedPattern(NGMatching) | c6TypeAnswer(Bool)
  syntax C6Query ::= c6EffectQuery(C6Boundary, C6Read) | c6KeepQuery(C6Boundary)
                   | c6PatternQuery(NGPattern, NGValue, Int, Int)
                   | c6TypeQuery(Int, NGValue)
  syntax C6Queries ::= c6NoQueries() | c6QueryRow(String, C6Query, C6Queries)
  syntax C6Answers ::= c6NoAnswers() | c6AnswerRow(String, C6Answer, C6Answers)
endmodule
