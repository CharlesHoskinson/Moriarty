module f03ListsumObservationTest {
  // Literal input only. SOURCE_ONLY / UNRUN pending Root selection and review.
  // Independent fixture-normal-programs.json F03 input SHA256:
  // 832f1c6521477d052b1021cb7cff25c2a7da3b807e6374243479e7b3126e4100
  // No expected states/results, evaluator, recursive unrolling or model changes.
  import kProfileMachine.* from "../kProfileMachine"

  pure val f03Program: Program = {
    expressions: Map(
      0 -> Call({name: 3, arguments: List(1)}),
      1 -> Literal(UIntList(List(1, 2, 3))),
      2 -> Match({value: 3, arms: List(
        {pattern: PNil, body: 4},
        {pattern: PCons({head: 1, tail: 2}), body: 5})}),
      3 -> Variable(0),
      4 -> Literal(UInt(0)),
      5 -> Binary({op: Add, left: 6, right: 8}),
      6 -> Variable(1),
      8 -> Call({name: 3, arguments: List(9)}),
      9 -> Variable(2)),
    functions: List({name: 3, parameters: List({name: 0, ty: ListType}),
                     returnType: UIntType, body: 2}),
    entry: 0,
  }

  //93 charged transitions plus the free terminal publication. Startup is init.
  run f03Fuel93ObservationTest = initProgram(f03Program, 93, 64)
    .then(94.reps(i => step))
}
