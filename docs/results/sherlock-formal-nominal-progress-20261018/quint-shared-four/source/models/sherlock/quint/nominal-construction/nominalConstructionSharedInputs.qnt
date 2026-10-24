module nominalConstructionSharedInputs {
  import nominalConstruction.* from "./nominalConstruction"
  // Mechanical input-only adapter; no expected output, prefix or work oracle.
  // Source SHA256 af670405bb1d476017cc51659ee9dd68231bbcaa023f111dcfecbd283e0a7cc2
  pure val SHARED_FUEL = 96
  // let-if-call-true
  pure val sharedCase0Program: Program = {
    expressions: Map(0 -> Let({name: 10, value: 1, body: 2}),
      1 -> Literal(List(UIntToken(4))),
      2 -> If({condition: 3, yes: 4, no: 5}),
      3 -> Literal(List(BoolToken(true))),
      4 -> Call({function: 30, arguments: List(6)}),
      5 -> Literal(List(UIntToken(5))),
      6 -> Variable(10),
      7 -> Variable(40)),
    functions: Map(30 -> {name: 30, parameters: List(40), parameterTypes: List(0), resultType: 0, body: 7}),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}), entry: 0,
  }
  pure val sharedCase0Environment: Environment = Map(10 -> List(UIntToken(9)), 20 -> List(UIntToken(7)))
  pure val sharedCase0Initial: Snapshot = {...initial(sharedCase0Program, SHARED_FUEL), environment: sharedCase0Environment}
  pure val sharedCase0Occurrences: int -> int = Map(0 -> 2000, 1 -> 2001, 2 -> 2002, 3 -> 2003, 4 -> 2004, 5 -> 2005, 6 -> 2006, 7 -> 2007)
  pure val sharedCase0RecursiveGroups: int -> int = Map(20 -> 9)
  // constructor-let-if-true
  pure val sharedCase1Program: Program = {
    expressions: Map(0 -> Constructor({typeId: 20, ctorId: 23, occurrence: 1000, fields: List(1, 9)}),
      1 -> Let({name: 10, value: 2, body: 5}),
      2 -> Constructor({typeId: 20, ctorId: 22, occurrence: 1002, fields: List(3, 4)}),
      3 -> Literal(List(UIntToken(4))),
      4 -> Constructor({typeId: 20, ctorId: 21, occurrence: 1004, fields: List()}),
      5 -> If({condition: 6, yes: 7, no: 8}),
      6 -> Literal(List(BoolToken(true))),
      7 -> Variable(10),
      8 -> Constructor({typeId: 20, ctorId: 21, occurrence: 1008, fields: List()}),
      9 -> Constructor({typeId: 20, ctorId: 22, occurrence: 1009, fields: List(10, 11)}),
      10 -> Literal(List(UIntToken(5))),
      11 -> Constructor({typeId: 20, ctorId: 21, occurrence: 1011, fields: List()})),
    functions: Map(),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}), entry: 0,
  }
  pure val sharedCase1Environment: Environment = Map(10 -> List(ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 20 -> List(UIntToken(7)))
  pure val sharedCase1Initial: Snapshot = {...initial(sharedCase1Program, SHARED_FUEL), environment: sharedCase1Environment}
  pure val sharedCase1Occurrences: int -> int = Map(0 -> 1000, 1 -> 1001, 2 -> 1002, 3 -> 1003, 4 -> 1004, 5 -> 1005, 6 -> 1006, 7 -> 1007, 8 -> 1008, 9 -> 1009, 10 -> 1010, 11 -> 1011)
  pure val sharedCase1RecursiveGroups: int -> int = Map(20 -> 9)
  // let-if-call-false
  pure val sharedCase2Program: Program = {
    expressions: Map(0 -> Let({name: 10, value: 1, body: 2}),
      1 -> Literal(List(UIntToken(4))),
      2 -> If({condition: 3, yes: 4, no: 5}),
      3 -> Literal(List(BoolToken(false))),
      4 -> Call({function: 30, arguments: List(6)}),
      5 -> Literal(List(UIntToken(5))),
      6 -> Variable(10),
      7 -> Variable(40)),
    functions: Map(30 -> {name: 30, parameters: List(40), parameterTypes: List(0), resultType: 0, body: 7}),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}), entry: 0,
  }
  pure val sharedCase2Environment: Environment = Map(10 -> List(UIntToken(9)), 20 -> List(UIntToken(7)))
  pure val sharedCase2Initial: Snapshot = {...initial(sharedCase2Program, SHARED_FUEL), environment: sharedCase2Environment}
  pure val sharedCase2Occurrences: int -> int = Map(0 -> 2000, 1 -> 2001, 2 -> 2002, 3 -> 2003, 4 -> 2004, 5 -> 2005, 6 -> 2006, 7 -> 2007)
  pure val sharedCase2RecursiveGroups: int -> int = Map(20 -> 9)
  // constructor-let-if-false
  pure val sharedCase3Program: Program = {
    expressions: Map(0 -> Constructor({typeId: 20, ctorId: 23, occurrence: 1000, fields: List(1, 9)}),
      1 -> Let({name: 10, value: 2, body: 5}),
      2 -> Constructor({typeId: 20, ctorId: 22, occurrence: 1002, fields: List(3, 4)}),
      3 -> Literal(List(UIntToken(4))),
      4 -> Constructor({typeId: 20, ctorId: 21, occurrence: 1004, fields: List()}),
      5 -> If({condition: 6, yes: 7, no: 8}),
      6 -> Literal(List(BoolToken(false))),
      7 -> Variable(10),
      8 -> Constructor({typeId: 20, ctorId: 21, occurrence: 1008, fields: List()}),
      9 -> Constructor({typeId: 20, ctorId: 22, occurrence: 1009, fields: List(10, 11)}),
      10 -> Literal(List(UIntToken(5))),
      11 -> Constructor({typeId: 20, ctorId: 21, occurrence: 1011, fields: List()})),
    functions: Map(),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}), entry: 0,
  }
  pure val sharedCase3Environment: Environment = Map(10 -> List(ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 20 -> List(UIntToken(7)))
  pure val sharedCase3Initial: Snapshot = {...initial(sharedCase3Program, SHARED_FUEL), environment: sharedCase3Environment}
  pure val sharedCase3Occurrences: int -> int = Map(0 -> 1000, 1 -> 1001, 2 -> 1002, 3 -> 1003, 4 -> 1004, 5 -> 1005, 6 -> 1006, 7 -> 1007, 8 -> 1008, 9 -> 1009, 10 -> 1010, 11 -> 1011)
  pure val sharedCase3RecursiveGroups: int -> int = Map(20 -> 9)
  pure val sharedInitials: List[Snapshot] = List(sharedCase0Initial, sharedCase1Initial, sharedCase2Initial, sharedCase3Initial)
}
