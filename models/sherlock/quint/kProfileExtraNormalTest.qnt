module kProfileExtraNormalTest {
  import kProfileMachine.* from "./kProfileMachine"

  // Additive literal transcription of extra-normal-programs/prefixes.json only.
  // Clock: sherlock-k-a/1; charged machine rules, free startup/publication.
  // A helpers are unmetered; helper visits/work are neither measured nor zero.
  // Immutable ExprIds share structurally equal syntax; recursive calls retain names.
  // Nine frame forms belong to the unchanged imported model; frames are top first.
  // Source-only normal examples: no B, financial, theorem, native or target claim.

  // N07: Three source-ordered arguments with asymmetric decimal weights.
  pure val n07Program: Program = {expressions: Map(0 -> Call({name: 17, arguments: List(1, 2, 3)}),
      1 -> Literal(UInt(1)),
      2 -> Literal(UInt(2)),
      3 -> Literal(UInt(3)),
      4 -> Binary({op: Add, left: 5, right: 10}),
      5 -> Binary({op: Add, left: 6, right: 7}),
      6 -> Variable(0),
      7 -> Binary({op: Mul, left: 8, right: 9}),
      8 -> Literal(UInt(10)),
      9 -> Variable(1),
      10 -> Binary({op: Mul, left: 11, right: 12}),
      11 -> Literal(UInt(100)),
      12 -> Variable(2)), functions: List({name: 17, parameters: List({name: 0, ty: UIntType}, {name: 1, ty: UIntType}, {name: 2, ty: UIntType}), returnType: UIntType, body: 4}), entry: 0}
  pure val n07State0: Snapshot = {program: n07Program, control: Eval(0), environment: Map(), frames: List(), stackLimit: 8, remaining: 38, initial: 38, outcome: Pending}
  pure val n07State1: Snapshot = {program: n07Program, control: Push({frame: ArgsFrame({name: 17, values: List(), remaining: List(2, 3), environment: Map()}), expression: 1, environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 37, initial: 38, outcome: Pending}
  pure val n07State2: Snapshot = {program: n07Program, control: Eval(1), environment: Map(), frames: List(ArgsFrame({name: 17, values: List(), remaining: List(2, 3), environment: Map()})), stackLimit: 8, remaining: 36, initial: 38, outcome: Pending}
  pure val n07State3: Snapshot = {program: n07Program, control: Return(UInt(1)), environment: Map(), frames: List(ArgsFrame({name: 17, values: List(), remaining: List(2, 3), environment: Map()})), stackLimit: 8, remaining: 35, initial: 38, outcome: Pending}
  pure val n07State4: Snapshot = {program: n07Program, control: Eval(2), environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1)), remaining: List(3), environment: Map()})), stackLimit: 8, remaining: 34, initial: 38, outcome: Pending}
  pure val n07State5: Snapshot = {program: n07Program, control: Return(UInt(2)), environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1)), remaining: List(3), environment: Map()})), stackLimit: 8, remaining: 33, initial: 38, outcome: Pending}
  pure val n07State6: Snapshot = {program: n07Program, control: Eval(3), environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1), UInt(2)), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 32, initial: 38, outcome: Pending}
  pure val n07State7: Snapshot = {program: n07Program, control: Return(UInt(3)), environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1), UInt(2)), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 31, initial: 38, outcome: Pending}
  pure val n07State8: Snapshot = {program: n07Program, control: Invoke({name: 17, values: List(UInt(1), UInt(2), UInt(3))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 30, initial: 38, outcome: Pending}
  pure val n07State9: Snapshot = {program: n07Program, control: Enter({found: FoundFunction({name: 17, parameters: List({name: 0, ty: UIntType}, {name: 1, ty: UIntType}, {name: 2, ty: UIntType}), returnType: UIntType, body: 4}), values: List(UInt(1), UInt(2), UInt(3))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 29, initial: 38, outcome: Pending}
  pure val n07State10: Snapshot = {program: n07Program, control: Bound({bindings: Bindings(Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))), ty: UIntType, expression: 4}), environment: Map(), frames: List(), stackLimit: 8, remaining: 28, initial: 38, outcome: Pending}
  pure val n07State11: Snapshot = {program: n07Program, control: Push({frame: ReturnFrame({ty: UIntType, environment: Map()}), expression: 4, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 27, initial: 38, outcome: Pending}
  pure val n07State12: Snapshot = {program: n07Program, control: Eval(4), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 26, initial: 38, outcome: Pending}
  pure val n07State13: Snapshot = {program: n07Program, control: Push({frame: LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), expression: 5, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 25, initial: 38, outcome: Pending}
  pure val n07State14: Snapshot = {program: n07Program, control: Eval(5), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 24, initial: 38, outcome: Pending}
  pure val n07State15: Snapshot = {program: n07Program, control: Push({frame: LeftFrame({op: Add, right: 7, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), expression: 6, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 23, initial: 38, outcome: Pending}
  pure val n07State16: Snapshot = {program: n07Program, control: Eval(6), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 7, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 22, initial: 38, outcome: Pending}
  pure val n07State17: Snapshot = {program: n07Program, control: Return(UInt(1)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 7, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 21, initial: 38, outcome: Pending}
  pure val n07State18: Snapshot = {program: n07Program, control: Eval(7), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 20, initial: 38, outcome: Pending}
  pure val n07State19: Snapshot = {program: n07Program, control: Push({frame: LeftFrame({op: Mul, right: 9, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), expression: 8, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 19, initial: 38, outcome: Pending}
  pure val n07State20: Snapshot = {program: n07Program, control: Eval(8), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 9, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 18, initial: 38, outcome: Pending}
  pure val n07State21: Snapshot = {program: n07Program, control: Return(UInt(10)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 9, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 17, initial: 38, outcome: Pending}
  pure val n07State22: Snapshot = {program: n07Program, control: Eval(9), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(10)}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 16, initial: 38, outcome: Pending}
  pure val n07State23: Snapshot = {program: n07Program, control: Return(UInt(2)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(10)}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 15, initial: 38, outcome: Pending}
  pure val n07State24: Snapshot = {program: n07Program, control: PrimitiveResult(PrimitiveValue(UInt(20))), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 14, initial: 38, outcome: Pending}
  pure val n07State25: Snapshot = {program: n07Program, control: Return(UInt(20)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 13, initial: 38, outcome: Pending}
  pure val n07State26: Snapshot = {program: n07Program, control: PrimitiveResult(PrimitiveValue(UInt(21))), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 12, initial: 38, outcome: Pending}
  pure val n07State27: Snapshot = {program: n07Program, control: Return(UInt(21)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 11, initial: 38, outcome: Pending}
  pure val n07State28: Snapshot = {program: n07Program, control: Eval(10), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 10, initial: 38, outcome: Pending}
  pure val n07State29: Snapshot = {program: n07Program, control: Push({frame: LeftFrame({op: Mul, right: 12, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), expression: 11, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 9, initial: 38, outcome: Pending}
  pure val n07State30: Snapshot = {program: n07Program, control: Eval(11), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 12, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 8, initial: 38, outcome: Pending}
  pure val n07State31: Snapshot = {program: n07Program, control: Return(UInt(100)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 12, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 7, initial: 38, outcome: Pending}
  pure val n07State32: Snapshot = {program: n07Program, control: Eval(12), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(100)}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 6, initial: 38, outcome: Pending}
  pure val n07State33: Snapshot = {program: n07Program, control: Return(UInt(3)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(100)}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 5, initial: 38, outcome: Pending}
  pure val n07State34: Snapshot = {program: n07Program, control: PrimitiveResult(PrimitiveValue(UInt(300))), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 4, initial: 38, outcome: Pending}
  pure val n07State35: Snapshot = {program: n07Program, control: Return(UInt(300)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 3, initial: 38, outcome: Pending}
  pure val n07State36: Snapshot = {program: n07Program, control: PrimitiveResult(PrimitiveValue(UInt(321))), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 2, initial: 38, outcome: Pending}
  pure val n07State37: Snapshot = {program: n07Program, control: Return(UInt(321)), environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 1, initial: 38, outcome: Pending}
  pure val n07State38: Snapshot = {program: n07Program, control: Return(UInt(321)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 38, outcome: Pending}
  pure val n07Published: Snapshot = {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 38, outcome: Done(UInt(321))}

  run n07ExactTraceTest = initProgram(n07Program, 38, 8)
    .expect(machine == n07State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n07State1 and lastRule == 131 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State2 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State3 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State4 and lastRule == 177 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State5 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State6 and lastRule == 177 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State7 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State8 and lastRule == 181 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State9 and lastRule == 200 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State10 and lastRule == 204 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State11 and lastRule == 208 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State12 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State13 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State14 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State15 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State16 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State17 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State18 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State19 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State20 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State21 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State22 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State23 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State24 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State25 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State26 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State27 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State28 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State29 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State30 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State31 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State32 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State33 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State34 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State35 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State36 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State37 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n07State38 and lastRule == 185 and lastCharge == 1)
    .then(step)
    .expect(machine == n07Published and lastRule == 80 and lastCharge == 0)

  run n07Fuel0PrefixTest = initProgram(n07Program, 0, 8)
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel1PrefixTest = initProgram(n07Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel2PrefixTest = initProgram(n07Program, 2, 8)
    .then(2.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 17, values: List(), remaining: List(2, 3), environment: Map()})), stackLimit: 8, remaining: 0, initial: 2, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel3PrefixTest = initProgram(n07Program, 3, 8)
    .then(3.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 17, values: List(), remaining: List(2, 3), environment: Map()})), stackLimit: 8, remaining: 0, initial: 3, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel4PrefixTest = initProgram(n07Program, 4, 8)
    .then(4.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1)), remaining: List(3), environment: Map()})), stackLimit: 8, remaining: 0, initial: 4, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel5PrefixTest = initProgram(n07Program, 5, 8)
    .then(5.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1)), remaining: List(3), environment: Map()})), stackLimit: 8, remaining: 0, initial: 5, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel6PrefixTest = initProgram(n07Program, 6, 8)
    .then(6.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1), UInt(2)), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 0, initial: 6, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel7PrefixTest = initProgram(n07Program, 7, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 17, values: List(UInt(1), UInt(2)), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 0, initial: 7, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel8PrefixTest = initProgram(n07Program, 8, 8)
    .then(8.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 8, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel9PrefixTest = initProgram(n07Program, 9, 8)
    .then(9.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 9, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel10PrefixTest = initProgram(n07Program, 10, 8)
    .then(10.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 10, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel11PrefixTest = initProgram(n07Program, 11, 8)
    .then(11.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 11, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel12PrefixTest = initProgram(n07Program, 12, 8)
    .then(12.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 12, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel13PrefixTest = initProgram(n07Program, 13, 8)
    .then(13.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 13, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel14PrefixTest = initProgram(n07Program, 14, 8)
    .then(14.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 14, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel15PrefixTest = initProgram(n07Program, 15, 8)
    .then(15.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 15, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel16PrefixTest = initProgram(n07Program, 16, 8)
    .then(16.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 7, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 16, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel17PrefixTest = initProgram(n07Program, 17, 8)
    .then(17.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 7, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 17, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel18PrefixTest = initProgram(n07Program, 18, 8)
    .then(18.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 18, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel19PrefixTest = initProgram(n07Program, 19, 8)
    .then(19.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 19, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel20PrefixTest = initProgram(n07Program, 20, 8)
    .then(20.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 9, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 20, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel21PrefixTest = initProgram(n07Program, 21, 8)
    .then(21.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 9, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 21, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel22PrefixTest = initProgram(n07Program, 22, 8)
    .then(22.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(10)}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 22, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel23PrefixTest = initProgram(n07Program, 23, 8)
    .then(23.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(10)}), RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 23, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel24PrefixTest = initProgram(n07Program, 24, 8)
    .then(24.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 24, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel25PrefixTest = initProgram(n07Program, 25, 8)
    .then(25.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(1)}), LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 25, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel26PrefixTest = initProgram(n07Program, 26, 8)
    .then(26.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 26, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel27PrefixTest = initProgram(n07Program, 27, 8)
    .then(27.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Add, right: 10, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 27, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel28PrefixTest = initProgram(n07Program, 28, 8)
    .then(28.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 28, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel29PrefixTest = initProgram(n07Program, 29, 8)
    .then(29.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 29, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel30PrefixTest = initProgram(n07Program, 30, 8)
    .then(30.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 12, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 30, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel31PrefixTest = initProgram(n07Program, 31, 8)
    .then(31.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(LeftFrame({op: Mul, right: 12, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3))}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 31, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel32PrefixTest = initProgram(n07Program, 32, 8)
    .then(32.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(100)}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 32, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel33PrefixTest = initProgram(n07Program, 33, 8)
    .then(33.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Mul, left: UInt(100)}), RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 33, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel34PrefixTest = initProgram(n07Program, 34, 8)
    .then(34.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 34, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel35PrefixTest = initProgram(n07Program, 35, 8)
    .then(35.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(RightFrame({op: Add, left: UInt(21)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 35, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel36PrefixTest = initProgram(n07Program, 36, 8)
    .then(36.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 36, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel37PrefixTest = initProgram(n07Program, 37, 8)
    .then(37.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(0 -> UInt(1), 1 -> UInt(2), 2 -> UInt(3)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 37, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n07Fuel38PrefixTest = initProgram(n07Program, 38, 8)
    .then(38.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 38, outcome: Done(UInt(321))} and lastRule == 80 and lastCharge == 0)

  run n07Fuel39PrefixTest = initProgram(n07Program, 39, 8)
    .then(38.reps(i => step))
    .then(step)
    .expect(machine == {program: n07Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 39, outcome: Done(UInt(321))} and lastRule == 80 and lastCharge == 0)

  // N08: Named self-recursive factorial two with suspended multiplication.
  pure val n08Program: Program = {expressions: Map(0 -> Call({name: 18, arguments: List(1)}),
      1 -> Literal(UInt(2)),
      2 -> If({condition: 3, yes: 6, no: 7}),
      3 -> Binary({op: Eq, left: 4, right: 5}),
      4 -> Variable(0),
      5 -> Literal(UInt(0)),
      6 -> Literal(UInt(1)),
      7 -> Binary({op: Mul, left: 4, right: 8}),
      8 -> Call({name: 18, arguments: List(9)}),
      9 -> Binary({op: Sub, left: 4, right: 6})), functions: List({name: 18, parameters: List({name: 0, ty: UIntType}), returnType: UIntType, body: 2}), entry: 0}
  pure val n08State0: Snapshot = {program: n08Program, control: Eval(0), environment: Map(), frames: List(), stackLimit: 8, remaining: 82, initial: 82, outcome: Pending}
  pure val n08State1: Snapshot = {program: n08Program, control: Push({frame: ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map()}), expression: 1, environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 81, initial: 82, outcome: Pending}
  pure val n08State2: Snapshot = {program: n08Program, control: Eval(1), environment: Map(), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 80, initial: 82, outcome: Pending}
  pure val n08State3: Snapshot = {program: n08Program, control: Return(UInt(2)), environment: Map(), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 79, initial: 82, outcome: Pending}
  pure val n08State4: Snapshot = {program: n08Program, control: Invoke({name: 18, values: List(UInt(2))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 78, initial: 82, outcome: Pending}
  pure val n08State5: Snapshot = {program: n08Program, control: Enter({found: FoundFunction({name: 18, parameters: List({name: 0, ty: UIntType}), returnType: UIntType, body: 2}), values: List(UInt(2))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 77, initial: 82, outcome: Pending}
  pure val n08State6: Snapshot = {program: n08Program, control: Bound({bindings: Bindings(Map(0 -> UInt(2))), ty: UIntType, expression: 2}), environment: Map(), frames: List(), stackLimit: 8, remaining: 76, initial: 82, outcome: Pending}
  pure val n08State7: Snapshot = {program: n08Program, control: Push({frame: ReturnFrame({ty: UIntType, environment: Map()}), expression: 2, environment: Map(0 -> UInt(2))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 75, initial: 82, outcome: Pending}
  pure val n08State8: Snapshot = {program: n08Program, control: Eval(2), environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 74, initial: 82, outcome: Pending}
  pure val n08State9: Snapshot = {program: n08Program, control: Push({frame: IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), expression: 3, environment: Map(0 -> UInt(2))}), environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 73, initial: 82, outcome: Pending}
  pure val n08State10: Snapshot = {program: n08Program, control: Eval(3), environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 72, initial: 82, outcome: Pending}
  pure val n08State11: Snapshot = {program: n08Program, control: Push({frame: LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(2))}), expression: 4, environment: Map(0 -> UInt(2))}), environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 71, initial: 82, outcome: Pending}
  pure val n08State12: Snapshot = {program: n08Program, control: Eval(4), environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(2))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 70, initial: 82, outcome: Pending}
  pure val n08State13: Snapshot = {program: n08Program, control: Return(UInt(2)), environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(2))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 69, initial: 82, outcome: Pending}
  pure val n08State14: Snapshot = {program: n08Program, control: Eval(5), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Eq, left: UInt(2)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 68, initial: 82, outcome: Pending}
  pure val n08State15: Snapshot = {program: n08Program, control: Return(UInt(0)), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Eq, left: UInt(2)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 67, initial: 82, outcome: Pending}
  pure val n08State16: Snapshot = {program: n08Program, control: PrimitiveResult(PrimitiveValue(BoolValue(false))), environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 66, initial: 82, outcome: Pending}
  pure val n08State17: Snapshot = {program: n08Program, control: Return(BoolValue(false)), environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 65, initial: 82, outcome: Pending}
  pure val n08State18: Snapshot = {program: n08Program, control: Eval(7), environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 64, initial: 82, outcome: Pending}
  pure val n08State19: Snapshot = {program: n08Program, control: Push({frame: LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(2))}), expression: 4, environment: Map(0 -> UInt(2))}), environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 63, initial: 82, outcome: Pending}
  pure val n08State20: Snapshot = {program: n08Program, control: Eval(4), environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 62, initial: 82, outcome: Pending}
  pure val n08State21: Snapshot = {program: n08Program, control: Return(UInt(2)), environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 61, initial: 82, outcome: Pending}
  pure val n08State22: Snapshot = {program: n08Program, control: Eval(8), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 60, initial: 82, outcome: Pending}
  pure val n08State23: Snapshot = {program: n08Program, control: Push({frame: ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), expression: 9, environment: Map(0 -> UInt(2))}), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 59, initial: 82, outcome: Pending}
  pure val n08State24: Snapshot = {program: n08Program, control: Eval(9), environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 58, initial: 82, outcome: Pending}
  pure val n08State25: Snapshot = {program: n08Program, control: Push({frame: LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(2))}), expression: 4, environment: Map(0 -> UInt(2))}), environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 57, initial: 82, outcome: Pending}
  pure val n08State26: Snapshot = {program: n08Program, control: Eval(4), environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(2))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 56, initial: 82, outcome: Pending}
  pure val n08State27: Snapshot = {program: n08Program, control: Return(UInt(2)), environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(2))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 55, initial: 82, outcome: Pending}
  pure val n08State28: Snapshot = {program: n08Program, control: Eval(6), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Sub, left: UInt(2)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 54, initial: 82, outcome: Pending}
  pure val n08State29: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Sub, left: UInt(2)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 53, initial: 82, outcome: Pending}
  pure val n08State30: Snapshot = {program: n08Program, control: PrimitiveResult(PrimitiveValue(UInt(1))), environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 52, initial: 82, outcome: Pending}
  pure val n08State31: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 51, initial: 82, outcome: Pending}
  pure val n08State32: Snapshot = {program: n08Program, control: Invoke({name: 18, values: List(UInt(1))}), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 50, initial: 82, outcome: Pending}
  pure val n08State33: Snapshot = {program: n08Program, control: Enter({found: FoundFunction({name: 18, parameters: List({name: 0, ty: UIntType}), returnType: UIntType, body: 2}), values: List(UInt(1))}), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 49, initial: 82, outcome: Pending}
  pure val n08State34: Snapshot = {program: n08Program, control: Bound({bindings: Bindings(Map(0 -> UInt(1))), ty: UIntType, expression: 2}), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 48, initial: 82, outcome: Pending}
  pure val n08State35: Snapshot = {program: n08Program, control: Push({frame: ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), expression: 2, environment: Map(0 -> UInt(1))}), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 47, initial: 82, outcome: Pending}
  pure val n08State36: Snapshot = {program: n08Program, control: Eval(2), environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 46, initial: 82, outcome: Pending}
  pure val n08State37: Snapshot = {program: n08Program, control: Push({frame: IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), expression: 3, environment: Map(0 -> UInt(1))}), environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 45, initial: 82, outcome: Pending}
  pure val n08State38: Snapshot = {program: n08Program, control: Eval(3), environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 44, initial: 82, outcome: Pending}
  pure val n08State39: Snapshot = {program: n08Program, control: Push({frame: LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(1))}), expression: 4, environment: Map(0 -> UInt(1))}), environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 43, initial: 82, outcome: Pending}
  pure val n08State40: Snapshot = {program: n08Program, control: Eval(4), environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(1))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 42, initial: 82, outcome: Pending}
  pure val n08State41: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(1))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 41, initial: 82, outcome: Pending}
  pure val n08State42: Snapshot = {program: n08Program, control: Eval(5), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Eq, left: UInt(1)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 40, initial: 82, outcome: Pending}
  pure val n08State43: Snapshot = {program: n08Program, control: Return(UInt(0)), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Eq, left: UInt(1)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 39, initial: 82, outcome: Pending}
  pure val n08State44: Snapshot = {program: n08Program, control: PrimitiveResult(PrimitiveValue(BoolValue(false))), environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 38, initial: 82, outcome: Pending}
  pure val n08State45: Snapshot = {program: n08Program, control: Return(BoolValue(false)), environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 37, initial: 82, outcome: Pending}
  pure val n08State46: Snapshot = {program: n08Program, control: Eval(7), environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 36, initial: 82, outcome: Pending}
  pure val n08State47: Snapshot = {program: n08Program, control: Push({frame: LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(1))}), expression: 4, environment: Map(0 -> UInt(1))}), environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 35, initial: 82, outcome: Pending}
  pure val n08State48: Snapshot = {program: n08Program, control: Eval(4), environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 34, initial: 82, outcome: Pending}
  pure val n08State49: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 33, initial: 82, outcome: Pending}
  pure val n08State50: Snapshot = {program: n08Program, control: Eval(8), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 32, initial: 82, outcome: Pending}
  pure val n08State51: Snapshot = {program: n08Program, control: Push({frame: ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), expression: 9, environment: Map(0 -> UInt(1))}), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 31, initial: 82, outcome: Pending}
  pure val n08State52: Snapshot = {program: n08Program, control: Eval(9), environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 30, initial: 82, outcome: Pending}
  pure val n08State53: Snapshot = {program: n08Program, control: Push({frame: LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(1))}), expression: 4, environment: Map(0 -> UInt(1))}), environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 29, initial: 82, outcome: Pending}
  pure val n08State54: Snapshot = {program: n08Program, control: Eval(4), environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(1))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 28, initial: 82, outcome: Pending}
  pure val n08State55: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(1))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 27, initial: 82, outcome: Pending}
  pure val n08State56: Snapshot = {program: n08Program, control: Eval(6), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Sub, left: UInt(1)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 26, initial: 82, outcome: Pending}
  pure val n08State57: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Sub, left: UInt(1)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 25, initial: 82, outcome: Pending}
  pure val n08State58: Snapshot = {program: n08Program, control: PrimitiveResult(PrimitiveValue(UInt(0))), environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 24, initial: 82, outcome: Pending}
  pure val n08State59: Snapshot = {program: n08Program, control: Return(UInt(0)), environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 23, initial: 82, outcome: Pending}
  pure val n08State60: Snapshot = {program: n08Program, control: Invoke({name: 18, values: List(UInt(0))}), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 22, initial: 82, outcome: Pending}
  pure val n08State61: Snapshot = {program: n08Program, control: Enter({found: FoundFunction({name: 18, parameters: List({name: 0, ty: UIntType}), returnType: UIntType, body: 2}), values: List(UInt(0))}), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 21, initial: 82, outcome: Pending}
  pure val n08State62: Snapshot = {program: n08Program, control: Bound({bindings: Bindings(Map(0 -> UInt(0))), ty: UIntType, expression: 2}), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 20, initial: 82, outcome: Pending}
  pure val n08State63: Snapshot = {program: n08Program, control: Push({frame: ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), expression: 2, environment: Map(0 -> UInt(0))}), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 19, initial: 82, outcome: Pending}
  pure val n08State64: Snapshot = {program: n08Program, control: Eval(2), environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 18, initial: 82, outcome: Pending}
  pure val n08State65: Snapshot = {program: n08Program, control: Push({frame: IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), expression: 3, environment: Map(0 -> UInt(0))}), environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 17, initial: 82, outcome: Pending}
  pure val n08State66: Snapshot = {program: n08Program, control: Eval(3), environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 16, initial: 82, outcome: Pending}
  pure val n08State67: Snapshot = {program: n08Program, control: Push({frame: LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(0))}), expression: 4, environment: Map(0 -> UInt(0))}), environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 15, initial: 82, outcome: Pending}
  pure val n08State68: Snapshot = {program: n08Program, control: Eval(4), environment: Map(0 -> UInt(0)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(0))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 14, initial: 82, outcome: Pending}
  pure val n08State69: Snapshot = {program: n08Program, control: Return(UInt(0)), environment: Map(0 -> UInt(0)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(0))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 13, initial: 82, outcome: Pending}
  pure val n08State70: Snapshot = {program: n08Program, control: Eval(5), environment: Map(0 -> UInt(0)), frames: List(RightFrame({op: Eq, left: UInt(0)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 12, initial: 82, outcome: Pending}
  pure val n08State71: Snapshot = {program: n08Program, control: Return(UInt(0)), environment: Map(0 -> UInt(0)), frames: List(RightFrame({op: Eq, left: UInt(0)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 11, initial: 82, outcome: Pending}
  pure val n08State72: Snapshot = {program: n08Program, control: PrimitiveResult(PrimitiveValue(BoolValue(true))), environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 10, initial: 82, outcome: Pending}
  pure val n08State73: Snapshot = {program: n08Program, control: Return(BoolValue(true)), environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 9, initial: 82, outcome: Pending}
  pure val n08State74: Snapshot = {program: n08Program, control: Eval(6), environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 8, initial: 82, outcome: Pending}
  pure val n08State75: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 7, initial: 82, outcome: Pending}
  pure val n08State76: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 6, initial: 82, outcome: Pending}
  pure val n08State77: Snapshot = {program: n08Program, control: PrimitiveResult(PrimitiveValue(UInt(1))), environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 5, initial: 82, outcome: Pending}
  pure val n08State78: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 4, initial: 82, outcome: Pending}
  pure val n08State79: Snapshot = {program: n08Program, control: Return(UInt(1)), environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 3, initial: 82, outcome: Pending}
  pure val n08State80: Snapshot = {program: n08Program, control: PrimitiveResult(PrimitiveValue(UInt(2))), environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 2, initial: 82, outcome: Pending}
  pure val n08State81: Snapshot = {program: n08Program, control: Return(UInt(2)), environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 1, initial: 82, outcome: Pending}
  pure val n08State82: Snapshot = {program: n08Program, control: Return(UInt(2)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 82, outcome: Pending}
  pure val n08Published: Snapshot = {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 82, outcome: Done(UInt(2))}

  run n08ExactTraceTest = initProgram(n08Program, 82, 8)
    .expect(machine == n08State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n08State1 and lastRule == 131 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State2 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State3 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State4 and lastRule == 181 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State5 and lastRule == 200 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State6 and lastRule == 204 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State7 and lastRule == 208 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State8 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State9 and lastRule == 123 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State10 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State11 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State12 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State13 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State14 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State15 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State16 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State17 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State18 and lastRule == 165 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State19 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State20 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State21 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State22 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State23 and lastRule == 131 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State24 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State25 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State26 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State27 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State28 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State29 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State30 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State31 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State32 and lastRule == 181 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State33 and lastRule == 200 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State34 and lastRule == 204 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State35 and lastRule == 208 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State36 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State37 and lastRule == 123 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State38 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State39 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State40 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State41 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State42 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State43 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State44 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State45 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State46 and lastRule == 165 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State47 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State48 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State49 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State50 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State51 and lastRule == 131 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State52 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State53 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State54 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State55 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State56 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State57 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State58 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State59 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State60 and lastRule == 181 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State61 and lastRule == 200 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State62 and lastRule == 204 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State63 and lastRule == 208 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State64 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State65 and lastRule == 123 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State66 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State67 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State68 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State69 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State70 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State71 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State72 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State73 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State74 and lastRule == 161 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State75 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State76 and lastRule == 185 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State77 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State78 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State79 and lastRule == 185 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State80 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State81 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n08State82 and lastRule == 185 and lastCharge == 1)
    .then(step)
    .expect(machine == n08Published and lastRule == 80 and lastCharge == 0)

  run n08Fuel0PrefixTest = initProgram(n08Program, 0, 8)
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel1PrefixTest = initProgram(n08Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel2PrefixTest = initProgram(n08Program, 2, 8)
    .then(2.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 0, initial: 2, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel3PrefixTest = initProgram(n08Program, 3, 8)
    .then(3.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 0, initial: 3, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel4PrefixTest = initProgram(n08Program, 4, 8)
    .then(4.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 4, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel5PrefixTest = initProgram(n08Program, 5, 8)
    .then(5.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 5, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel6PrefixTest = initProgram(n08Program, 6, 8)
    .then(6.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 6, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel7PrefixTest = initProgram(n08Program, 7, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 7, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel8PrefixTest = initProgram(n08Program, 8, 8)
    .then(8.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 8, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel9PrefixTest = initProgram(n08Program, 9, 8)
    .then(9.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 9, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel10PrefixTest = initProgram(n08Program, 10, 8)
    .then(10.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 10, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel11PrefixTest = initProgram(n08Program, 11, 8)
    .then(11.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 11, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel12PrefixTest = initProgram(n08Program, 12, 8)
    .then(12.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(2))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 12, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel13PrefixTest = initProgram(n08Program, 13, 8)
    .then(13.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(2))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 13, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel14PrefixTest = initProgram(n08Program, 14, 8)
    .then(14.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Eq, left: UInt(2)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 14, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel15PrefixTest = initProgram(n08Program, 15, 8)
    .then(15.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Eq, left: UInt(2)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 15, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel16PrefixTest = initProgram(n08Program, 16, 8)
    .then(16.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 16, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel17PrefixTest = initProgram(n08Program, 17, 8)
    .then(17.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 17, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel18PrefixTest = initProgram(n08Program, 18, 8)
    .then(18.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 18, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel19PrefixTest = initProgram(n08Program, 19, 8)
    .then(19.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 19, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel20PrefixTest = initProgram(n08Program, 20, 8)
    .then(20.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 20, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel21PrefixTest = initProgram(n08Program, 21, 8)
    .then(21.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(2))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 21, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel22PrefixTest = initProgram(n08Program, 22, 8)
    .then(22.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 22, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel23PrefixTest = initProgram(n08Program, 23, 8)
    .then(23.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 23, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel24PrefixTest = initProgram(n08Program, 24, 8)
    .then(24.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 24, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel25PrefixTest = initProgram(n08Program, 25, 8)
    .then(25.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 25, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel26PrefixTest = initProgram(n08Program, 26, 8)
    .then(26.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(2))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 26, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel27PrefixTest = initProgram(n08Program, 27, 8)
    .then(27.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(2))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 27, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel28PrefixTest = initProgram(n08Program, 28, 8)
    .then(28.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Sub, left: UInt(2)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 28, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel29PrefixTest = initProgram(n08Program, 29, 8)
    .then(29.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Sub, left: UInt(2)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 29, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel30PrefixTest = initProgram(n08Program, 30, 8)
    .then(30.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 30, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel31PrefixTest = initProgram(n08Program, 31, 8)
    .then(31.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 31, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel32PrefixTest = initProgram(n08Program, 32, 8)
    .then(32.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 32, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel33PrefixTest = initProgram(n08Program, 33, 8)
    .then(33.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 33, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel34PrefixTest = initProgram(n08Program, 34, 8)
    .then(34.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 34, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel35PrefixTest = initProgram(n08Program, 35, 8)
    .then(35.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 35, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel36PrefixTest = initProgram(n08Program, 36, 8)
    .then(36.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 36, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel37PrefixTest = initProgram(n08Program, 37, 8)
    .then(37.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 37, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel38PrefixTest = initProgram(n08Program, 38, 8)
    .then(38.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 38, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel39PrefixTest = initProgram(n08Program, 39, 8)
    .then(39.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 39, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel40PrefixTest = initProgram(n08Program, 40, 8)
    .then(40.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(1))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 40, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel41PrefixTest = initProgram(n08Program, 41, 8)
    .then(41.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(1))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 41, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel42PrefixTest = initProgram(n08Program, 42, 8)
    .then(42.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Eq, left: UInt(1)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 42, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel43PrefixTest = initProgram(n08Program, 43, 8)
    .then(43.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Eq, left: UInt(1)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 43, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel44PrefixTest = initProgram(n08Program, 44, 8)
    .then(44.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 44, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel45PrefixTest = initProgram(n08Program, 45, 8)
    .then(45.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 45, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel46PrefixTest = initProgram(n08Program, 46, 8)
    .then(46.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 46, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel47PrefixTest = initProgram(n08Program, 47, 8)
    .then(47.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 47, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel48PrefixTest = initProgram(n08Program, 48, 8)
    .then(48.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 48, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel49PrefixTest = initProgram(n08Program, 49, 8)
    .then(49.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Mul, right: 8, environment: Map(0 -> UInt(1))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 49, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel50PrefixTest = initProgram(n08Program, 50, 8)
    .then(50.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 50, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel51PrefixTest = initProgram(n08Program, 51, 8)
    .then(51.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 51, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel52PrefixTest = initProgram(n08Program, 52, 8)
    .then(52.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 52, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel53PrefixTest = initProgram(n08Program, 53, 8)
    .then(53.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 53, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel54PrefixTest = initProgram(n08Program, 54, 8)
    .then(54.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(1))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 54, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel55PrefixTest = initProgram(n08Program, 55, 8)
    .then(55.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(LeftFrame({op: Sub, right: 6, environment: Map(0 -> UInt(1))}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 55, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel56PrefixTest = initProgram(n08Program, 56, 8)
    .then(56.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Sub, left: UInt(1)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 56, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel57PrefixTest = initProgram(n08Program, 57, 8)
    .then(57.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Sub, left: UInt(1)}), ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 57, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel58PrefixTest = initProgram(n08Program, 58, 8)
    .then(58.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 58, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel59PrefixTest = initProgram(n08Program, 59, 8)
    .then(59.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ArgsFrame({name: 18, values: List(), remaining: List(), environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 59, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel60PrefixTest = initProgram(n08Program, 60, 8)
    .then(60.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 60, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel61PrefixTest = initProgram(n08Program, 61, 8)
    .then(61.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 61, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel62PrefixTest = initProgram(n08Program, 62, 8)
    .then(62.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 62, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel63PrefixTest = initProgram(n08Program, 63, 8)
    .then(63.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 63, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel64PrefixTest = initProgram(n08Program, 64, 8)
    .then(64.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 64, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel65PrefixTest = initProgram(n08Program, 65, 8)
    .then(65.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 65, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel66PrefixTest = initProgram(n08Program, 66, 8)
    .then(66.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 66, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel67PrefixTest = initProgram(n08Program, 67, 8)
    .then(67.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 67, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel68PrefixTest = initProgram(n08Program, 68, 8)
    .then(68.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(0))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 68, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel69PrefixTest = initProgram(n08Program, 69, 8)
    .then(69.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(LeftFrame({op: Eq, right: 5, environment: Map(0 -> UInt(0))}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 69, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel70PrefixTest = initProgram(n08Program, 70, 8)
    .then(70.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(RightFrame({op: Eq, left: UInt(0)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 70, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel71PrefixTest = initProgram(n08Program, 71, 8)
    .then(71.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(RightFrame({op: Eq, left: UInt(0)}), IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 71, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel72PrefixTest = initProgram(n08Program, 72, 8)
    .then(72.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 72, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel73PrefixTest = initProgram(n08Program, 73, 8)
    .then(73.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(IfFrame({yes: 6, no: 7, environment: Map(0 -> UInt(0))}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 73, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel74PrefixTest = initProgram(n08Program, 74, 8)
    .then(74.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 74, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel75PrefixTest = initProgram(n08Program, 75, 8)
    .then(75.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(0)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(1))}), RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 75, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel76PrefixTest = initProgram(n08Program, 76, 8)
    .then(76.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(RightFrame({op: Mul, left: UInt(1)}), ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 76, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel77PrefixTest = initProgram(n08Program, 77, 8)
    .then(77.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 77, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel78PrefixTest = initProgram(n08Program, 78, 8)
    .then(78.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(1)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> UInt(2))}), RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 78, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel79PrefixTest = initProgram(n08Program, 79, 8)
    .then(79.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(RightFrame({op: Mul, left: UInt(2)}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 79, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel80PrefixTest = initProgram(n08Program, 80, 8)
    .then(80.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 80, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel81PrefixTest = initProgram(n08Program, 81, 8)
    .then(81.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(0 -> UInt(2)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 81, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n08Fuel82PrefixTest = initProgram(n08Program, 82, 8)
    .then(82.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 82, outcome: Done(UInt(2))} and lastRule == 80 and lastCharge == 0)

  run n08Fuel83PrefixTest = initProgram(n08Program, 83, 8)
    .then(82.reps(i => step))
    .then(step)
    .expect(machine == {program: n08Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 83, outcome: Done(UInt(2))} and lastRule == 80 and lastCharge == 0)

}
