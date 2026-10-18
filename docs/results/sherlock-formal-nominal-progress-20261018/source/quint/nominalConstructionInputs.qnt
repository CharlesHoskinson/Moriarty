module nominalConstructionInputs {
  import nominalConstruction.* from "./nominalConstruction"
  // Inputs are ordinary AST tables. No scenario identifier enters the evaluator.
  pure val layouts: (int, int) -> Layout = Map(
    (10, 0) -> {typeId: 10, ctorId: 0, fields: List()},
    (10, 1) -> {typeId: 10, ctorId: 1, fields: List(0, 10)},
    (20, 0) -> {typeId: 20, ctorId: 0, fields: List(10, 0)},
    (20, 1) -> {typeId: 20, ctorId: 1, fields: List(0)},
    (30, 0) -> {typeId: 30, ctorId: 0, fields: List(31)},
    (30, 1) -> {typeId: 30, ctorId: 1, fields: List()},
    (31, 0) -> {typeId: 31, ctorId: 0, fields: List(30)},
    (40, 0) -> {typeId: 40, ctorId: 0, fields: List(0)},
    (41, 0) -> {typeId: 41, ctorId: 0, fields: List(40, 40)}
  )
  pure val mainExpressions: int -> Expression = Map(
    0 -> Let({name: 1, value: 1, body: 2}),
    1 -> Literal(uint(9)),
    2 -> Let({name: 2, value: 3, body: 4}),
    3 -> Constructor({typeId: 20, ctorId: 0, occurrence: 200, fields: List(5, 6)}),
    4 -> Binary({op: Add, left: 8, right: 9}),
    5 -> Call({function: 10, arguments: List(7)}),
    6 -> Variable(1),
    7 -> Literal(uint(2)),
    8 -> Call({function: 11, arguments: List(10)}),
    9 -> Variable(1),
    10 -> Variable(2),
    20 -> If({condition: 21, yes: 24, no: 25}),
    21 -> Binary({op: Equal, left: 22, right: 23}),
    22 -> Variable(3),
    23 -> Literal(uint(0)),
    24 -> Constructor({typeId: 10, ctorId: 0, occurrence: 101, fields: List()}),
    25 -> Constructor({typeId: 10, ctorId: 1, occurrence: 102, fields: List(26, 27)}),
    26 -> Variable(3),
    27 -> Call({function: 10, arguments: List(28)}),
    28 -> Binary({op: Sub, left: 29, right: 30}),
    29 -> Variable(3),
    30 -> Literal(uint(1)),
    40 -> Match({value: 41, arms: List(
      {pattern: ConstructorPattern({typeId: 20, ctorId: 1, binders: List(4)}), body: 42},
      {pattern: ConstructorPattern({typeId: 20, ctorId: 0, binders: List(5, 1)}), body: 43},
      {pattern: ConstructorPattern({typeId: 20, ctorId: 0, binders: List(5, 1)}), body: 42},
      {pattern: Bind(6), body: 44})}),
    41 -> Variable(1),
    42 -> Literal(uint(77)),
    43 -> Variable(1),
    44 -> Literal(uint(88))
  )
  pure val mainFunctions: int -> Function = Map(
    10 -> {name: 10, parameters: List(3), parameterTypes: List(0), resultType: 10, body: 20},
    11 -> {name: 11, parameters: List(1), parameterTypes: List(20), resultType: 0, body: 40}
  )
  pure val mainProgram: Program = {expressions: mainExpressions, functions: mainFunctions, layouts: layouts, entry: 0}
  pure val repeatedExpressions: int -> Expression = Map(
    0 -> Let({name: 2, value: 1, body: 8}),
    1 -> Constructor({typeId: 41, ctorId: 0, occurrence: 303, fields: List(2, 3)}),
    2 -> Constructor({typeId: 40, ctorId: 0, occurrence: 301, fields: List(4)}),
    3 -> Constructor({typeId: 40, ctorId: 0, occurrence: 302, fields: List(5)}),
    4 -> Literal(uint(4)), 5 -> Literal(uint(4)),
    8 -> Match({value: 9, arms: List({pattern: ConstructorPattern({typeId: 41, ctorId: 0, binders: List(6, 7)}), body: 10})}),
    9 -> Variable(2),
    10 -> Binary({op: Add, left: 11, right: 12}),
    11 -> Call({function: 12, arguments: List(13)}),
    12 -> Call({function: 12, arguments: List(14)}),
    13 -> Variable(6), 14 -> Variable(7),
    20 -> Match({value: 21, arms: List({pattern: ConstructorPattern({typeId: 40, ctorId: 0, binders: List(1)}), body: 22})}),
    21 -> Variable(1), 22 -> Variable(1)
  )
  pure val repeatedProgram: Program = {
    expressions: repeatedExpressions,
    functions: Map(12 -> {name: 12, parameters: List(1), parameterTypes: List(40), resultType: 0, body: 20}),
    layouts: layouts, entry: 0,
  }
  pure val mutualProgram: Program = {
    expressions: Map(
      0 -> Constructor({typeId: 30, ctorId: 0, occurrence: 403, fields: List(1)}),
      1 -> Constructor({typeId: 31, ctorId: 0, occurrence: 402, fields: List(2)}),
      2 -> Constructor({typeId: 30, ctorId: 1, occurrence: 401, fields: List()})
    ), functions: Map(), layouts: layouts, entry: 0,
  }
}
