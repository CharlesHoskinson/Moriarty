module kProfileMachineTest {
  import kProfileMachine.* from "./kProfileMachine"

  // Independent literal source: ../oracles/normal-programs.json and normal-prefixes.json.
  // Structural transcription of frozen literals, never candidate evaluator output.
  // Frames are top first. ExprIds name immutable syntax; tree tokens encode values.

  // N01: Literal UInt128 seven; exact charged work 1.
  pure val n01Program: Program = {
    expressions: Map(0 -> Literal(UInt(7))),
    functions: List(), entry: 0,
  }
  pure val n01State0: Snapshot = {program: n01Program, control: Eval(0), environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 1, outcome: Pending}
  pure val n01State1: Snapshot = {program: n01Program, control: Return(UInt(7)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Pending}
  pure val n01Published: Snapshot = {program: n01Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Done(UInt(7))}

  run n01ExactTraceTest = initProgram(n01Program, 1, 8)
    .expect(machine == n01State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n01State1 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n01Published and lastRule == 80 and lastCharge == 0)

  run n01Fuel0PrefixTest = initProgram(n01Program, 0, 8)
    .then(step)
    .expect(machine == {program: n01Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n01Fuel1PrefixTest = initProgram(n01Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n01Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Done(UInt(7))} and lastRule == 80 and lastCharge == 0)

  run n01Fuel2PrefixTest = initProgram(n01Program, 2, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n01Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 2, outcome: Done(UInt(7))} and lastRule == 80 and lastCharge == 0)

  // N02: Checked UInt128 add in source order; exact charged work 7.
  pure val n02Program: Program = {
    expressions: Map(0 -> Binary({op: Add, left: 1, right: 2}),
      1 -> Literal(UInt(2)),
      2 -> Literal(UInt(3))),
    functions: List(), entry: 0,
  }
  pure val n02State0: Snapshot = {program: n02Program, control: Eval(0), environment: Map(), frames: List(), stackLimit: 8, remaining: 7, initial: 7, outcome: Pending}
  pure val n02State1: Snapshot = {program: n02Program, control: Push({frame: LeftFrame({op: Add, right: 2, environment: Map()}), expression: 1, environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 6, initial: 7, outcome: Pending}
  pure val n02State2: Snapshot = {program: n02Program, control: Eval(1), environment: Map(), frames: List(LeftFrame({op: Add, right: 2, environment: Map()})), stackLimit: 8, remaining: 5, initial: 7, outcome: Pending}
  pure val n02State3: Snapshot = {program: n02Program, control: Return(UInt(2)), environment: Map(), frames: List(LeftFrame({op: Add, right: 2, environment: Map()})), stackLimit: 8, remaining: 4, initial: 7, outcome: Pending}
  pure val n02State4: Snapshot = {program: n02Program, control: Eval(2), environment: Map(), frames: List(RightFrame({op: Add, left: UInt(2)})), stackLimit: 8, remaining: 3, initial: 7, outcome: Pending}
  pure val n02State5: Snapshot = {program: n02Program, control: Return(UInt(3)), environment: Map(), frames: List(RightFrame({op: Add, left: UInt(2)})), stackLimit: 8, remaining: 2, initial: 7, outcome: Pending}
  pure val n02State6: Snapshot = {program: n02Program, control: PrimitiveResult(PrimitiveValue(UInt(5))), environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 7, outcome: Pending}
  pure val n02State7: Snapshot = {program: n02Program, control: Return(UInt(5)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 7, outcome: Pending}
  pure val n02Published: Snapshot = {program: n02Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 7, outcome: Done(UInt(5))}

  run n02ExactTraceTest = initProgram(n02Program, 7, 8)
    .expect(machine == n02State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n02State1 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n02State2 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n02State3 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n02State4 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n02State5 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n02State6 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n02State7 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n02Published and lastRule == 80 and lastCharge == 0)

  run n02Fuel0PrefixTest = initProgram(n02Program, 0, 8)
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n02Fuel1PrefixTest = initProgram(n02Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n02Fuel2PrefixTest = initProgram(n02Program, 2, 8)
    .then(2.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(LeftFrame({op: Add, right: 2, environment: Map()})), stackLimit: 8, remaining: 0, initial: 2, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n02Fuel3PrefixTest = initProgram(n02Program, 3, 8)
    .then(3.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(LeftFrame({op: Add, right: 2, environment: Map()})), stackLimit: 8, remaining: 0, initial: 3, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n02Fuel4PrefixTest = initProgram(n02Program, 4, 8)
    .then(4.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(RightFrame({op: Add, left: UInt(2)})), stackLimit: 8, remaining: 0, initial: 4, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n02Fuel5PrefixTest = initProgram(n02Program, 5, 8)
    .then(5.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(RightFrame({op: Add, left: UInt(2)})), stackLimit: 8, remaining: 0, initial: 5, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n02Fuel6PrefixTest = initProgram(n02Program, 6, 8)
    .then(6.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 6, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n02Fuel7PrefixTest = initProgram(n02Program, 7, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 7, outcome: Done(UInt(5))} and lastRule == 80 and lastCharge == 0)

  run n02Fuel8PrefixTest = initProgram(n02Program, 8, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n02Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 8, outcome: Done(UInt(5))} and lastRule == 80 and lastCharge == 0)

  // N03: Not selects only the true conditional arm; exact charged work 9.
  pure val n03Program: Program = {
    expressions: Map(0 -> If({condition: 1, yes: 3, no: 4}),
      1 -> Unary({op: NotOp, argument: 2}),
      2 -> Literal(BoolValue(false)),
      3 -> Literal(UInt(9)),
      4 -> Literal(UInt(4))),
    functions: List(), entry: 0,
  }
  pure val n03State0: Snapshot = {program: n03Program, control: Eval(0), environment: Map(), frames: List(), stackLimit: 8, remaining: 9, initial: 9, outcome: Pending}
  pure val n03State1: Snapshot = {program: n03Program, control: Push({frame: IfFrame({yes: 3, no: 4, environment: Map()}), expression: 1, environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 8, initial: 9, outcome: Pending}
  pure val n03State2: Snapshot = {program: n03Program, control: Eval(1), environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 7, initial: 9, outcome: Pending}
  pure val n03State3: Snapshot = {program: n03Program, control: Push({frame: UnaryFrame(NotOp), expression: 2, environment: Map()}), environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 6, initial: 9, outcome: Pending}
  pure val n03State4: Snapshot = {program: n03Program, control: Eval(2), environment: Map(), frames: List(UnaryFrame(NotOp), IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 5, initial: 9, outcome: Pending}
  pure val n03State5: Snapshot = {program: n03Program, control: Return(BoolValue(false)), environment: Map(), frames: List(UnaryFrame(NotOp), IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 4, initial: 9, outcome: Pending}
  pure val n03State6: Snapshot = {program: n03Program, control: PrimitiveResult(PrimitiveValue(BoolValue(true))), environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 3, initial: 9, outcome: Pending}
  pure val n03State7: Snapshot = {program: n03Program, control: Return(BoolValue(true)), environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 2, initial: 9, outcome: Pending}
  pure val n03State8: Snapshot = {program: n03Program, control: Eval(3), environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 9, outcome: Pending}
  pure val n03State9: Snapshot = {program: n03Program, control: Return(UInt(9)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 9, outcome: Pending}
  pure val n03Published: Snapshot = {program: n03Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 9, outcome: Done(UInt(9))}

  run n03ExactTraceTest = initProgram(n03Program, 9, 8)
    .expect(machine == n03State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n03State1 and lastRule == 123 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State2 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State3 and lastRule == 120 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State4 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State5 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State6 and lastRule == 157 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State7 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State8 and lastRule == 161 and lastCharge == 1)
    .then(step)
    .expect(machine == n03State9 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n03Published and lastRule == 80 and lastCharge == 0)

  run n03Fuel0PrefixTest = initProgram(n03Program, 0, 8)
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel1PrefixTest = initProgram(n03Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel2PrefixTest = initProgram(n03Program, 2, 8)
    .then(2.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 0, initial: 2, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel3PrefixTest = initProgram(n03Program, 3, 8)
    .then(3.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 0, initial: 3, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel4PrefixTest = initProgram(n03Program, 4, 8)
    .then(4.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(UnaryFrame(NotOp), IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 0, initial: 4, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel5PrefixTest = initProgram(n03Program, 5, 8)
    .then(5.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(UnaryFrame(NotOp), IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 0, initial: 5, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel6PrefixTest = initProgram(n03Program, 6, 8)
    .then(6.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 0, initial: 6, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel7PrefixTest = initProgram(n03Program, 7, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(IfFrame({yes: 3, no: 4, environment: Map()})), stackLimit: 8, remaining: 0, initial: 7, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel8PrefixTest = initProgram(n03Program, 8, 8)
    .then(8.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 8, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n03Fuel9PrefixTest = initProgram(n03Program, 9, 8)
    .then(9.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 9, outcome: Done(UInt(9))} and lastRule == 80 and lastCharge == 0)

  run n03Fuel10PrefixTest = initProgram(n03Program, 10, 8)
    .then(9.reps(i => step))
    .then(step)
    .expect(machine == {program: n03Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 10, outcome: Done(UInt(9))} and lastRule == 80 and lastCharge == 0)

  // N04: D22 nested shadowing and lexical restoration; exact charged work 17.
  pure val n04Program: Program = {
    expressions: Map(0 -> Let({name: 0, value: 1, body: 2}),
      1 -> Literal(UInt(7)),
      2 -> Binary({op: Add, left: 3, right: 5}),
      3 -> Let({name: 0, value: 4, body: 5}),
      4 -> Literal(UInt(4)),
      5 -> Variable(0)),
    functions: List(), entry: 0,
  }
  pure val n04State0: Snapshot = {program: n04Program, control: Eval(0), environment: Map(), frames: List(), stackLimit: 8, remaining: 17, initial: 17, outcome: Pending}
  pure val n04State1: Snapshot = {program: n04Program, control: Push({frame: LetFrame({name: 0, body: 2, environment: Map()}), expression: 1, environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 16, initial: 17, outcome: Pending}
  pure val n04State2: Snapshot = {program: n04Program, control: Eval(1), environment: Map(), frames: List(LetFrame({name: 0, body: 2, environment: Map()})), stackLimit: 8, remaining: 15, initial: 17, outcome: Pending}
  pure val n04State3: Snapshot = {program: n04Program, control: Return(UInt(7)), environment: Map(), frames: List(LetFrame({name: 0, body: 2, environment: Map()})), stackLimit: 8, remaining: 14, initial: 17, outcome: Pending}
  pure val n04State4: Snapshot = {program: n04Program, control: Eval(2), environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 13, initial: 17, outcome: Pending}
  pure val n04State5: Snapshot = {program: n04Program, control: Push({frame: LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), expression: 3, environment: Map(0 -> UInt(7))}), environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 12, initial: 17, outcome: Pending}
  pure val n04State6: Snapshot = {program: n04Program, control: Eval(3), environment: Map(0 -> UInt(7)), frames: List(LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 11, initial: 17, outcome: Pending}
  pure val n04State7: Snapshot = {program: n04Program, control: Push({frame: LetFrame({name: 0, body: 5, environment: Map(0 -> UInt(7))}), expression: 4, environment: Map(0 -> UInt(7))}), environment: Map(0 -> UInt(7)), frames: List(LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 10, initial: 17, outcome: Pending}
  pure val n04State8: Snapshot = {program: n04Program, control: Eval(4), environment: Map(0 -> UInt(7)), frames: List(LetFrame({name: 0, body: 5, environment: Map(0 -> UInt(7))}), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 9, initial: 17, outcome: Pending}
  pure val n04State9: Snapshot = {program: n04Program, control: Return(UInt(4)), environment: Map(0 -> UInt(7)), frames: List(LetFrame({name: 0, body: 5, environment: Map(0 -> UInt(7))}), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 8, initial: 17, outcome: Pending}
  pure val n04State10: Snapshot = {program: n04Program, control: Eval(5), environment: Map(0 -> UInt(4)), frames: List(ScopeFrame(Map(0 -> UInt(7))), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 7, initial: 17, outcome: Pending}
  pure val n04State11: Snapshot = {program: n04Program, control: Return(UInt(4)), environment: Map(0 -> UInt(4)), frames: List(ScopeFrame(Map(0 -> UInt(7))), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 6, initial: 17, outcome: Pending}
  pure val n04State12: Snapshot = {program: n04Program, control: Return(UInt(4)), environment: Map(0 -> UInt(7)), frames: List(LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 5, initial: 17, outcome: Pending}
  pure val n04State13: Snapshot = {program: n04Program, control: Eval(5), environment: Map(0 -> UInt(7)), frames: List(RightFrame({op: Add, left: UInt(4)}), ScopeFrame(Map())), stackLimit: 8, remaining: 4, initial: 17, outcome: Pending}
  pure val n04State14: Snapshot = {program: n04Program, control: Return(UInt(7)), environment: Map(0 -> UInt(7)), frames: List(RightFrame({op: Add, left: UInt(4)}), ScopeFrame(Map())), stackLimit: 8, remaining: 3, initial: 17, outcome: Pending}
  pure val n04State15: Snapshot = {program: n04Program, control: PrimitiveResult(PrimitiveValue(UInt(11))), environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 2, initial: 17, outcome: Pending}
  pure val n04State16: Snapshot = {program: n04Program, control: Return(UInt(11)), environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 1, initial: 17, outcome: Pending}
  pure val n04State17: Snapshot = {program: n04Program, control: Return(UInt(11)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 17, outcome: Pending}
  pure val n04Published: Snapshot = {program: n04Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 17, outcome: Done(UInt(11))}

  run n04ExactTraceTest = initProgram(n04Program, 17, 8)
    .expect(machine == n04State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n04State1 and lastRule == 114 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State2 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State3 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State4 and lastRule == 145 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State5 and lastRule == 117 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State6 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State7 and lastRule == 114 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State8 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State9 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State10 and lastRule == 145 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State11 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State12 and lastRule == 195 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State13 and lastRule == 149 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State14 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State15 and lastRule == 153 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State16 and lastRule == 223 and lastCharge == 1)
    .then(step)
    .expect(machine == n04State17 and lastRule == 195 and lastCharge == 1)
    .then(step)
    .expect(machine == n04Published and lastRule == 80 and lastCharge == 0)

  run n04Fuel0PrefixTest = initProgram(n04Program, 0, 8)
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel1PrefixTest = initProgram(n04Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel2PrefixTest = initProgram(n04Program, 2, 8)
    .then(2.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(), frames: List(LetFrame({name: 0, body: 2, environment: Map()})), stackLimit: 8, remaining: 0, initial: 2, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel3PrefixTest = initProgram(n04Program, 3, 8)
    .then(3.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(), frames: List(LetFrame({name: 0, body: 2, environment: Map()})), stackLimit: 8, remaining: 0, initial: 3, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel4PrefixTest = initProgram(n04Program, 4, 8)
    .then(4.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 4, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel5PrefixTest = initProgram(n04Program, 5, 8)
    .then(5.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 5, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel6PrefixTest = initProgram(n04Program, 6, 8)
    .then(6.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 6, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel7PrefixTest = initProgram(n04Program, 7, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 7, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel8PrefixTest = initProgram(n04Program, 8, 8)
    .then(8.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(LetFrame({name: 0, body: 5, environment: Map(0 -> UInt(7))}), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 8, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel9PrefixTest = initProgram(n04Program, 9, 8)
    .then(9.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(LetFrame({name: 0, body: 5, environment: Map(0 -> UInt(7))}), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 9, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel10PrefixTest = initProgram(n04Program, 10, 8)
    .then(10.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(4)), frames: List(ScopeFrame(Map(0 -> UInt(7))), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 10, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel11PrefixTest = initProgram(n04Program, 11, 8)
    .then(11.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(4)), frames: List(ScopeFrame(Map(0 -> UInt(7))), LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 11, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel12PrefixTest = initProgram(n04Program, 12, 8)
    .then(12.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(LeftFrame({op: Add, right: 5, environment: Map(0 -> UInt(7))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 12, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel13PrefixTest = initProgram(n04Program, 13, 8)
    .then(13.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(RightFrame({op: Add, left: UInt(4)}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 13, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel14PrefixTest = initProgram(n04Program, 14, 8)
    .then(14.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(RightFrame({op: Add, left: UInt(4)}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 14, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel15PrefixTest = initProgram(n04Program, 15, 8)
    .then(15.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 15, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel16PrefixTest = initProgram(n04Program, 16, 8)
    .then(16.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(0 -> UInt(7)), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 16, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n04Fuel17PrefixTest = initProgram(n04Program, 17, 8)
    .then(17.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 17, outcome: Done(UInt(11))} and lastRule == 80 and lastCharge == 0)

  run n04Fuel18PrefixTest = initProgram(n04Program, 18, 8)
    .then(17.reps(i => step))
    .then(step)
    .expect(machine == {program: n04Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 18, outcome: Done(UInt(11))} and lastRule == 80 and lastCharge == 0)

  // N05: Named call and actual one-step self recursion; exact charged work 27.
  pure val n05Program: Program = {
    expressions: Map(0 -> If({condition: 1, yes: 2, no: 4}),
      1 -> Variable(0),
      2 -> Call({name: 11, arguments: List(3)}),
      3 -> Literal(BoolValue(false)),
      4 -> Literal(UInt(6)),
      5 -> Call({name: 11, arguments: List(6)}),
      6 -> Literal(BoolValue(true))),
    functions: List({name: 11, parameters: List({name: 0, ty: BoolType}), returnType: UIntType, body: 0}), entry: 5,
  }
  pure val n05State0: Snapshot = {program: n05Program, control: Eval(5), environment: Map(), frames: List(), stackLimit: 8, remaining: 27, initial: 27, outcome: Pending}
  pure val n05State1: Snapshot = {program: n05Program, control: Push({frame: ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map()}), expression: 6, environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 26, initial: 27, outcome: Pending}
  pure val n05State2: Snapshot = {program: n05Program, control: Eval(6), environment: Map(), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 25, initial: 27, outcome: Pending}
  pure val n05State3: Snapshot = {program: n05Program, control: Return(BoolValue(true)), environment: Map(), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 24, initial: 27, outcome: Pending}
  pure val n05State4: Snapshot = {program: n05Program, control: Invoke({name: 11, values: List(BoolValue(true))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 23, initial: 27, outcome: Pending}
  pure val n05State5: Snapshot = {program: n05Program, control: Enter({found: FoundFunction({name: 11, parameters: List({name: 0, ty: BoolType}), returnType: UIntType, body: 0}), values: List(BoolValue(true))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 22, initial: 27, outcome: Pending}
  pure val n05State6: Snapshot = {program: n05Program, control: Bound({bindings: Bindings(Map(0 -> BoolValue(true))), ty: UIntType, expression: 0}), environment: Map(), frames: List(), stackLimit: 8, remaining: 21, initial: 27, outcome: Pending}
  pure val n05State7: Snapshot = {program: n05Program, control: Push({frame: ReturnFrame({ty: UIntType, environment: Map()}), expression: 0, environment: Map(0 -> BoolValue(true))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 20, initial: 27, outcome: Pending}
  pure val n05State8: Snapshot = {program: n05Program, control: Eval(0), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 19, initial: 27, outcome: Pending}
  pure val n05State9: Snapshot = {program: n05Program, control: Push({frame: IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(true))}), expression: 1, environment: Map(0 -> BoolValue(true))}), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 18, initial: 27, outcome: Pending}
  pure val n05State10: Snapshot = {program: n05Program, control: Eval(1), environment: Map(0 -> BoolValue(true)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 17, initial: 27, outcome: Pending}
  pure val n05State11: Snapshot = {program: n05Program, control: Return(BoolValue(true)), environment: Map(0 -> BoolValue(true)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 16, initial: 27, outcome: Pending}
  pure val n05State12: Snapshot = {program: n05Program, control: Eval(2), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 15, initial: 27, outcome: Pending}
  pure val n05State13: Snapshot = {program: n05Program, control: Push({frame: ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map(0 -> BoolValue(true))}), expression: 3, environment: Map(0 -> BoolValue(true))}), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 14, initial: 27, outcome: Pending}
  pure val n05State14: Snapshot = {program: n05Program, control: Eval(3), environment: Map(0 -> BoolValue(true)), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 13, initial: 27, outcome: Pending}
  pure val n05State15: Snapshot = {program: n05Program, control: Return(BoolValue(false)), environment: Map(0 -> BoolValue(true)), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 12, initial: 27, outcome: Pending}
  pure val n05State16: Snapshot = {program: n05Program, control: Invoke({name: 11, values: List(BoolValue(false))}), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 11, initial: 27, outcome: Pending}
  pure val n05State17: Snapshot = {program: n05Program, control: Enter({found: FoundFunction({name: 11, parameters: List({name: 0, ty: BoolType}), returnType: UIntType, body: 0}), values: List(BoolValue(false))}), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 10, initial: 27, outcome: Pending}
  pure val n05State18: Snapshot = {program: n05Program, control: Bound({bindings: Bindings(Map(0 -> BoolValue(false))), ty: UIntType, expression: 0}), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 9, initial: 27, outcome: Pending}
  pure val n05State19: Snapshot = {program: n05Program, control: Push({frame: ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), expression: 0, environment: Map(0 -> BoolValue(false))}), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 8, initial: 27, outcome: Pending}
  pure val n05State20: Snapshot = {program: n05Program, control: Eval(0), environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 7, initial: 27, outcome: Pending}
  pure val n05State21: Snapshot = {program: n05Program, control: Push({frame: IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(false))}), expression: 1, environment: Map(0 -> BoolValue(false))}), environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 6, initial: 27, outcome: Pending}
  pure val n05State22: Snapshot = {program: n05Program, control: Eval(1), environment: Map(0 -> BoolValue(false)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(false))}), ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 5, initial: 27, outcome: Pending}
  pure val n05State23: Snapshot = {program: n05Program, control: Return(BoolValue(false)), environment: Map(0 -> BoolValue(false)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(false))}), ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 4, initial: 27, outcome: Pending}
  pure val n05State24: Snapshot = {program: n05Program, control: Eval(4), environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 3, initial: 27, outcome: Pending}
  pure val n05State25: Snapshot = {program: n05Program, control: Return(UInt(6)), environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 2, initial: 27, outcome: Pending}
  pure val n05State26: Snapshot = {program: n05Program, control: Return(UInt(6)), environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 1, initial: 27, outcome: Pending}
  pure val n05State27: Snapshot = {program: n05Program, control: Return(UInt(6)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 27, outcome: Pending}
  pure val n05Published: Snapshot = {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 27, outcome: Done(UInt(6))}

  run n05ExactTraceTest = initProgram(n05Program, 27, 8)
    .expect(machine == n05State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n05State1 and lastRule == 131 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State2 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State3 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State4 and lastRule == 181 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State5 and lastRule == 200 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State6 and lastRule == 204 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State7 and lastRule == 208 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State8 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State9 and lastRule == 123 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State10 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State11 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State12 and lastRule == 161 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State13 and lastRule == 131 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State14 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State15 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State16 and lastRule == 181 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State17 and lastRule == 200 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State18 and lastRule == 204 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State19 and lastRule == 208 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State20 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State21 and lastRule == 123 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State22 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State23 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State24 and lastRule == 165 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State25 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State26 and lastRule == 185 and lastCharge == 1)
    .then(step)
    .expect(machine == n05State27 and lastRule == 185 and lastCharge == 1)
    .then(step)
    .expect(machine == n05Published and lastRule == 80 and lastCharge == 0)

  run n05Fuel0PrefixTest = initProgram(n05Program, 0, 8)
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel1PrefixTest = initProgram(n05Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel2PrefixTest = initProgram(n05Program, 2, 8)
    .then(2.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 0, initial: 2, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel3PrefixTest = initProgram(n05Program, 3, 8)
    .then(3.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map()})), stackLimit: 8, remaining: 0, initial: 3, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel4PrefixTest = initProgram(n05Program, 4, 8)
    .then(4.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 4, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel5PrefixTest = initProgram(n05Program, 5, 8)
    .then(5.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 5, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel6PrefixTest = initProgram(n05Program, 6, 8)
    .then(6.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 6, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel7PrefixTest = initProgram(n05Program, 7, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 7, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel8PrefixTest = initProgram(n05Program, 8, 8)
    .then(8.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 8, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel9PrefixTest = initProgram(n05Program, 9, 8)
    .then(9.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 9, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel10PrefixTest = initProgram(n05Program, 10, 8)
    .then(10.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 10, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel11PrefixTest = initProgram(n05Program, 11, 8)
    .then(11.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 11, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel12PrefixTest = initProgram(n05Program, 12, 8)
    .then(12.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 12, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel13PrefixTest = initProgram(n05Program, 13, 8)
    .then(13.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 13, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel14PrefixTest = initProgram(n05Program, 14, 8)
    .then(14.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 14, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel15PrefixTest = initProgram(n05Program, 15, 8)
    .then(15.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ArgsFrame({name: 11, values: List(), remaining: List(), environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 15, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel16PrefixTest = initProgram(n05Program, 16, 8)
    .then(16.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 16, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel17PrefixTest = initProgram(n05Program, 17, 8)
    .then(17.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 17, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel18PrefixTest = initProgram(n05Program, 18, 8)
    .then(18.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 18, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel19PrefixTest = initProgram(n05Program, 19, 8)
    .then(19.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 19, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel20PrefixTest = initProgram(n05Program, 20, 8)
    .then(20.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 20, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel21PrefixTest = initProgram(n05Program, 21, 8)
    .then(21.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 21, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel22PrefixTest = initProgram(n05Program, 22, 8)
    .then(22.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(false)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(false))}), ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 22, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel23PrefixTest = initProgram(n05Program, 23, 8)
    .then(23.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(false)), frames: List(IfFrame({yes: 2, no: 4, environment: Map(0 -> BoolValue(false))}), ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 23, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel24PrefixTest = initProgram(n05Program, 24, 8)
    .then(24.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 24, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel25PrefixTest = initProgram(n05Program, 25, 8)
    .then(25.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(false)), frames: List(ReturnFrame({ty: UIntType, environment: Map(0 -> BoolValue(true))}), ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 25, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel26PrefixTest = initProgram(n05Program, 26, 8)
    .then(26.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(0 -> BoolValue(true)), frames: List(ReturnFrame({ty: UIntType, environment: Map()})), stackLimit: 8, remaining: 0, initial: 26, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n05Fuel27PrefixTest = initProgram(n05Program, 27, 8)
    .then(27.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 27, outcome: Done(UInt(6))} and lastRule == 80 and lastCharge == 0)

  run n05Fuel28PrefixTest = initProgram(n05Program, 28, 8)
    .then(27.reps(i => step))
    .then(step)
    .expect(machine == {program: n05Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 28, outcome: Done(UInt(6))} and lastRule == 80 and lastCharge == 0)

  // N06: Ordered list then tree match, first source arm wins; exact charged work 19.
  pure val n06Program: Program = {
    expressions: Map(0 -> Match({value: 1, arms: List({pattern: PNil, body: 2}, {pattern: PCons({head: 1, tail: 2}), body: 3})}),
      1 -> Literal(UIntList(List(4))),
      2 -> Literal(UInt(0)),
      3 -> Match({value: 4, arms: List({pattern: PVar(3), body: 5}, {pattern: PLeaf(4), body: 6})}),
      4 -> Literal(UIntTree(List(TreeLeaf(9)))),
      5 -> Variable(1),
      6 -> Literal(UInt(99))),
    functions: List(), entry: 0,
  }
  pure val n06State0: Snapshot = {program: n06Program, control: Eval(0), environment: Map(), frames: List(), stackLimit: 8, remaining: 19, initial: 19, outcome: Pending}
  pure val n06State1: Snapshot = {program: n06Program, control: Push({frame: MatchFrame({arms: List({pattern: PNil, body: 2}, {pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()}), expression: 1, environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 18, initial: 19, outcome: Pending}
  pure val n06State2: Snapshot = {program: n06Program, control: Eval(1), environment: Map(), frames: List(MatchFrame({arms: List({pattern: PNil, body: 2}, {pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()})), stackLimit: 8, remaining: 17, initial: 19, outcome: Pending}
  pure val n06State3: Snapshot = {program: n06Program, control: Return(UIntList(List(4))), environment: Map(), frames: List(MatchFrame({arms: List({pattern: PNil, body: 2}, {pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()})), stackLimit: 8, remaining: 16, initial: 19, outcome: Pending}
  pure val n06State4: Snapshot = {program: n06Program, control: Cases({value: UIntList(List(4)), arms: List({pattern: PNil, body: 2}, {pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 15, initial: 19, outcome: Pending}
  pure val n06State5: Snapshot = {program: n06Program, control: CaseResult({matching: NoMatch, value: UIntList(List(4)), expression: 2, remaining: List({pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 14, initial: 19, outcome: Pending}
  pure val n06State6: Snapshot = {program: n06Program, control: Cases({value: UIntList(List(4)), arms: List({pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 13, initial: 19, outcome: Pending}
  pure val n06State7: Snapshot = {program: n06Program, control: CaseResult({matching: Matched(Map(1 -> UInt(4), 2 -> UIntList(List()))), value: UIntList(List(4)), expression: 3, remaining: List(), environment: Map()}), environment: Map(), frames: List(), stackLimit: 8, remaining: 12, initial: 19, outcome: Pending}
  pure val n06State8: Snapshot = {program: n06Program, control: Push({frame: ScopeFrame(Map()), expression: 3, environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), environment: Map(), frames: List(), stackLimit: 8, remaining: 11, initial: 19, outcome: Pending}
  pure val n06State9: Snapshot = {program: n06Program, control: Eval(3), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 10, initial: 19, outcome: Pending}
  pure val n06State10: Snapshot = {program: n06Program, control: Push({frame: MatchFrame({arms: List({pattern: PVar(3), body: 5}, {pattern: PLeaf(4), body: 6}), environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), expression: 4, environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 9, initial: 19, outcome: Pending}
  pure val n06State11: Snapshot = {program: n06Program, control: Eval(4), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(MatchFrame({arms: List({pattern: PVar(3), body: 5}, {pattern: PLeaf(4), body: 6}), environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), ScopeFrame(Map())), stackLimit: 8, remaining: 8, initial: 19, outcome: Pending}
  pure val n06State12: Snapshot = {program: n06Program, control: Return(UIntTree(List(TreeLeaf(9)))), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(MatchFrame({arms: List({pattern: PVar(3), body: 5}, {pattern: PLeaf(4), body: 6}), environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), ScopeFrame(Map())), stackLimit: 8, remaining: 7, initial: 19, outcome: Pending}
  pure val n06State13: Snapshot = {program: n06Program, control: Cases({value: UIntTree(List(TreeLeaf(9))), arms: List({pattern: PVar(3), body: 5}, {pattern: PLeaf(4), body: 6}), environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 6, initial: 19, outcome: Pending}
  pure val n06State14: Snapshot = {program: n06Program, control: CaseResult({matching: Matched(Map(1 -> UInt(4), 2 -> UIntList(List()), 3 -> UIntTree(List(TreeLeaf(9))))), value: UIntTree(List(TreeLeaf(9))), expression: 5, remaining: List({pattern: PLeaf(4), body: 6}), environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 5, initial: 19, outcome: Pending}
  pure val n06State15: Snapshot = {program: n06Program, control: Push({frame: ScopeFrame(Map(1 -> UInt(4), 2 -> UIntList(List()))), expression: 5, environment: Map(1 -> UInt(4), 2 -> UIntList(List()), 3 -> UIntTree(List(TreeLeaf(9))))}), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 4, initial: 19, outcome: Pending}
  pure val n06State16: Snapshot = {program: n06Program, control: Eval(5), environment: Map(1 -> UInt(4), 2 -> UIntList(List()), 3 -> UIntTree(List(TreeLeaf(9)))), frames: List(ScopeFrame(Map(1 -> UInt(4), 2 -> UIntList(List()))), ScopeFrame(Map())), stackLimit: 8, remaining: 3, initial: 19, outcome: Pending}
  pure val n06State17: Snapshot = {program: n06Program, control: Return(UInt(4)), environment: Map(1 -> UInt(4), 2 -> UIntList(List()), 3 -> UIntTree(List(TreeLeaf(9)))), frames: List(ScopeFrame(Map(1 -> UInt(4), 2 -> UIntList(List()))), ScopeFrame(Map())), stackLimit: 8, remaining: 2, initial: 19, outcome: Pending}
  pure val n06State18: Snapshot = {program: n06Program, control: Return(UInt(4)), environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 1, initial: 19, outcome: Pending}
  pure val n06State19: Snapshot = {program: n06Program, control: Return(UInt(4)), environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 19, outcome: Pending}
  pure val n06Published: Snapshot = {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 19, outcome: Done(UInt(4))}

  run n06ExactTraceTest = initProgram(n06Program, 19, 8)
    .expect(machine == n06State0 and lastRule == 74 and lastCharge == 0)
    .then(step)
    .expect(machine == n06State1 and lastRule == 126 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State2 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State3 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State4 and lastRule == 173 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State5 and lastRule == 214 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State6 and lastRule == 217 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State7 and lastRule == 214 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State8 and lastRule == 220 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State9 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State10 and lastRule == 126 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State11 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State12 and lastRule == 102 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State13 and lastRule == 173 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State14 and lastRule == 214 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State15 and lastRule == 220 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State16 and lastRule == 135 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State17 and lastRule == 108 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State18 and lastRule == 195 and lastCharge == 1)
    .then(step)
    .expect(machine == n06State19 and lastRule == 195 and lastCharge == 1)
    .then(step)
    .expect(machine == n06Published and lastRule == 80 and lastCharge == 0)

  run n06Fuel0PrefixTest = initProgram(n06Program, 0, 8)
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 0, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel1PrefixTest = initProgram(n06Program, 1, 8)
    .then(1.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 1, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel2PrefixTest = initProgram(n06Program, 2, 8)
    .then(2.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(MatchFrame({arms: List({pattern: PNil, body: 2}, {pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()})), stackLimit: 8, remaining: 0, initial: 2, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel3PrefixTest = initProgram(n06Program, 3, 8)
    .then(3.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(MatchFrame({arms: List({pattern: PNil, body: 2}, {pattern: PCons({head: 1, tail: 2}), body: 3}), environment: Map()})), stackLimit: 8, remaining: 0, initial: 3, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel4PrefixTest = initProgram(n06Program, 4, 8)
    .then(4.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 4, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel5PrefixTest = initProgram(n06Program, 5, 8)
    .then(5.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 5, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel6PrefixTest = initProgram(n06Program, 6, 8)
    .then(6.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 6, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel7PrefixTest = initProgram(n06Program, 7, 8)
    .then(7.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 7, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel8PrefixTest = initProgram(n06Program, 8, 8)
    .then(8.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 8, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel9PrefixTest = initProgram(n06Program, 9, 8)
    .then(9.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 9, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel10PrefixTest = initProgram(n06Program, 10, 8)
    .then(10.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 10, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel11PrefixTest = initProgram(n06Program, 11, 8)
    .then(11.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(MatchFrame({arms: List({pattern: PVar(3), body: 5}, {pattern: PLeaf(4), body: 6}), environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 11, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel12PrefixTest = initProgram(n06Program, 12, 8)
    .then(12.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(MatchFrame({arms: List({pattern: PVar(3), body: 5}, {pattern: PLeaf(4), body: 6}), environment: Map(1 -> UInt(4), 2 -> UIntList(List()))}), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 12, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel13PrefixTest = initProgram(n06Program, 13, 8)
    .then(13.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 13, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel14PrefixTest = initProgram(n06Program, 14, 8)
    .then(14.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 14, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel15PrefixTest = initProgram(n06Program, 15, 8)
    .then(15.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 15, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel16PrefixTest = initProgram(n06Program, 16, 8)
    .then(16.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List()), 3 -> UIntTree(List(TreeLeaf(9)))), frames: List(ScopeFrame(Map(1 -> UInt(4), 2 -> UIntList(List()))), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 16, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel17PrefixTest = initProgram(n06Program, 17, 8)
    .then(17.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List()), 3 -> UIntTree(List(TreeLeaf(9)))), frames: List(ScopeFrame(Map(1 -> UInt(4), 2 -> UIntList(List()))), ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 17, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel18PrefixTest = initProgram(n06Program, 18, 8)
    .then(18.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(1 -> UInt(4), 2 -> UIntList(List())), frames: List(ScopeFrame(Map())), stackLimit: 8, remaining: 0, initial: 18, outcome: Exhausted} and lastRule == 84 and lastCharge == 0)

  run n06Fuel19PrefixTest = initProgram(n06Program, 19, 8)
    .then(19.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 0, initial: 19, outcome: Done(UInt(4))} and lastRule == 80 and lastCharge == 0)

  run n06Fuel20PrefixTest = initProgram(n06Program, 20, 8)
    .then(19.reps(i => step))
    .then(step)
    .expect(machine == {program: n06Program, control: EmptyControl, environment: Map(), frames: List(), stackLimit: 8, remaining: 1, initial: 20, outcome: Done(UInt(4))} and lastRule == 80 and lastCharge == 0)

}

