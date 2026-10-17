module aInputs {
  import aPure.* from "./aPure"
  pure val n01Program: Program = {
    expressions: Map(0 -> Literal(UInt(7))),
    functions: List(), entry: 0,
  }
  pure val n02Program: Program = {
    expressions: Map(0 -> Binary({op: Add, left: 1, right: 2}),
      1 -> Literal(UInt(2)),
      2 -> Literal(UInt(3))),
    functions: List(), entry: 0,
  }
  pure val n04Program: Program = {
    expressions: Map(0 -> Let({name: 0, value: 1, body: 2}),
      1 -> Literal(UInt(7)),
      2 -> Binary({op: Add, left: 3, right: 5}),
      3 -> Let({name: 0, value: 4, body: 5}),
      4 -> Literal(UInt(4)),
      5 -> Variable(0)),
    functions: List(), entry: 0,
  }
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
  pure val otherProgram: Program = {expressions: Map(0 -> Literal(UInt(9))), functions: List(), entry: 0}
}
