module patternFrontierInputs {
  import nominalConstruction.* from "./nominalConstruction"
  import patternFrontier as F from "./patternFrontier"
  // late-nested-miss-discards-earlier-binding
  pure val frontierInput0: F::FrontierInput = {
    sourceMatchId: 701,
    arms: List({sourceArmId: 7010, bodyExprId: 900, patternRoot: 0}, {sourceArmId: 7011, bodyExprId: 901, patternRoot: 5}, {sourceArmId: 7012, bodyExprId: 902, patternRoot: 8}),
    valueRoot: 5,
    valueRows: Map(0 -> {token: UIntToken(4), children: {start: 0, len: 0}}, 1 -> {token: UIntToken(5), children: {start: 0, len: 0}}, 2 -> {token: ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), children: {start: 0, len: 0}}, 3 -> {token: ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 4 -> {token: ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), children: {start: 2, len: 2}}, 5 -> {token: ConstructorToken({typeId: 20, ctorId: 23, arity: 2}), children: {start: 4, len: 2}}),
    patternRows: Map(0 -> {token: PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), children: {start: 2, len: 2}}, 1 -> {token: PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 2 -> {token: PatternBind(11), children: {start: 0, len: 0}}, 3 -> {token: PatternWildcard, children: {start: 0, len: 0}}, 4 -> {token: PatternConstructor({typeId: 20, ctorId: 21, arity: 0}), children: {start: 2, len: 0}}, 5 -> {token: PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), children: {start: 4, len: 2}}, 6 -> {token: PatternWildcard, children: {start: 4, len: 0}}, 7 -> {token: PatternBind(12), children: {start: 4, len: 0}}, 8 -> {token: PatternBind(13), children: {start: 6, len: 0}}),
    valueFields: List(0, 2, 1, 2, 3, 4),
    patternFields: List(2, 3, 1, 4, 6, 7),
    decodedValues: Map(0 -> List(UIntToken(4)), 1 -> List(UIntToken(5)), 2 -> List(ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 3 -> List(ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 4 -> List(ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(5), ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 5 -> List(ConstructorToken({typeId: 20, ctorId: 23, arity: 2}), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(5), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}))),
    encodedPatterns: Map(2 -> List(PatternBind(11)), 3 -> List(PatternWildcard), 1 -> List(PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(11), PatternWildcard), 4 -> List(PatternConstructor({typeId: 20, ctorId: 21, arity: 0})), 0 -> List(PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(11), PatternWildcard, PatternConstructor({typeId: 20, ctorId: 21, arity: 0})), 6 -> List(PatternWildcard), 7 -> List(PatternBind(12)), 5 -> List(PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), PatternWildcard, PatternBind(12)), 8 -> List(PatternBind(13))),
    sourcePaths: List(List(List(), List(0), List(0, 0), List(0, 1), List(1)), List(List(), List(0), List(1)), List(List())),
    binderOrigins: List(List({name: 11, path: List(0, 0)}), List({name: 12, path: List(1)}), List({name: 13, path: List()})),
    savedEnvironment: Map(10 -> List(UIntToken(11)), 11 -> List(UIntToken(77)), 20 -> List(UIntToken(7))),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}),
    analysisBounds: {advances: 32, maxTasks: 12, maxPath: 4}
  }
  // shared-subvalue-two-source-occurrence-paths
  pure val frontierInput1: F::FrontierInput = {
    sourceMatchId: 702,
    arms: List({sourceArmId: 7020, bodyExprId: 900, patternRoot: 0}, {sourceArmId: 7021, bodyExprId: 901, patternRoot: 7}, {sourceArmId: 7022, bodyExprId: 902, patternRoot: 10}),
    valueRoot: 3,
    valueRows: Map(0 -> {token: UIntToken(4), children: {start: 0, len: 0}}, 1 -> {token: ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), children: {start: 0, len: 0}}, 2 -> {token: ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 3 -> {token: ConstructorToken({typeId: 20, ctorId: 23, arity: 2}), children: {start: 2, len: 2}}),
    patternRows: Map(0 -> {token: PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), children: {start: 4, len: 2}}, 1 -> {token: PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 2 -> {token: PatternBind(11), children: {start: 0, len: 0}}, 3 -> {token: PatternWildcard, children: {start: 0, len: 0}}, 4 -> {token: PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), children: {start: 2, len: 2}}, 5 -> {token: PatternBind(12), children: {start: 2, len: 0}}, 6 -> {token: PatternWildcard, children: {start: 2, len: 0}}, 7 -> {token: PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), children: {start: 6, len: 2}}, 8 -> {token: PatternWildcard, children: {start: 6, len: 0}}, 9 -> {token: PatternBind(12), children: {start: 6, len: 0}}, 10 -> {token: PatternBind(13), children: {start: 8, len: 0}}),
    valueFields: List(0, 1, 2, 2),
    patternFields: List(2, 3, 5, 6, 1, 4, 8, 9),
    decodedValues: Map(0 -> List(UIntToken(4)), 1 -> List(ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 2 -> List(ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 3 -> List(ConstructorToken({typeId: 20, ctorId: 23, arity: 2}), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}))),
    encodedPatterns: Map(2 -> List(PatternBind(11)), 3 -> List(PatternWildcard), 1 -> List(PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(11), PatternWildcard), 5 -> List(PatternBind(12)), 6 -> List(PatternWildcard), 4 -> List(PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(12), PatternWildcard), 0 -> List(PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(11), PatternWildcard, PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(12), PatternWildcard), 8 -> List(PatternWildcard), 9 -> List(PatternBind(12)), 7 -> List(PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), PatternWildcard, PatternBind(12)), 10 -> List(PatternBind(13))),
    sourcePaths: List(List(List(), List(0), List(0, 0), List(0, 1), List(1), List(1, 0), List(1, 1)), List(List(), List(0), List(1)), List(List())),
    binderOrigins: List(List({name: 11, path: List(0, 0)}, {name: 12, path: List(1, 0)}), List({name: 12, path: List(1)}), List({name: 13, path: List()})),
    savedEnvironment: Map(10 -> List(UIntToken(11)), 11 -> List(UIntToken(77)), 20 -> List(UIntToken(7))),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}),
    analysisBounds: {advances: 32, maxTasks: 12, maxPath: 4}
  }
  // wildcard-skips-complete-deep-left-subtree
  pure val frontierInput2: F::FrontierInput = {
    sourceMatchId: 703,
    arms: List({sourceArmId: 7030, bodyExprId: 900, patternRoot: 0}, {sourceArmId: 7031, bodyExprId: 901, patternRoot: 3}, {sourceArmId: 7032, bodyExprId: 902, patternRoot: 6}),
    valueRoot: 7,
    valueRows: Map(0 -> {token: UIntToken(4), children: {start: 0, len: 0}}, 1 -> {token: UIntToken(5), children: {start: 0, len: 0}}, 2 -> {token: UIntToken(6), children: {start: 0, len: 0}}, 3 -> {token: ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), children: {start: 0, len: 0}}, 4 -> {token: ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 5 -> {token: ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), children: {start: 2, len: 2}}, 6 -> {token: ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), children: {start: 4, len: 2}}, 7 -> {token: ConstructorToken({typeId: 20, ctorId: 23, arity: 2}), children: {start: 6, len: 2}}),
    patternRows: Map(0 -> {token: PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), children: {start: 0, len: 2}}, 1 -> {token: PatternWildcard, children: {start: 0, len: 0}}, 2 -> {token: PatternBind(12), children: {start: 0, len: 0}}, 3 -> {token: PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), children: {start: 2, len: 2}}, 4 -> {token: PatternBind(11), children: {start: 2, len: 0}}, 5 -> {token: PatternWildcard, children: {start: 2, len: 0}}, 6 -> {token: PatternBind(13), children: {start: 4, len: 0}}),
    valueFields: List(1, 3, 0, 4, 2, 3, 5, 6),
    patternFields: List(1, 2, 4, 5),
    decodedValues: Map(0 -> List(UIntToken(4)), 1 -> List(UIntToken(5)), 2 -> List(UIntToken(6)), 3 -> List(ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 4 -> List(ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(5), ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 5 -> List(ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(5), ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 6 -> List(ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(6), ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 7 -> List(ConstructorToken({typeId: 20, ctorId: 23, arity: 2}), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(5), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(6), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}))),
    encodedPatterns: Map(1 -> List(PatternWildcard), 2 -> List(PatternBind(12)), 0 -> List(PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), PatternWildcard, PatternBind(12)), 4 -> List(PatternBind(11)), 5 -> List(PatternWildcard), 3 -> List(PatternConstructor({typeId: 20, ctorId: 23, arity: 2}), PatternBind(11), PatternWildcard), 6 -> List(PatternBind(13))),
    sourcePaths: List(List(List(), List(0), List(1)), List(List(), List(0), List(1)), List(List())),
    binderOrigins: List(List({name: 12, path: List(1)}), List({name: 11, path: List(0)}), List({name: 13, path: List()})),
    savedEnvironment: Map(10 -> List(UIntToken(11)), 11 -> List(UIntToken(77)), 20 -> List(UIntToken(7))),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}),
    analysisBounds: {advances: 32, maxTasks: 12, maxPath: 4}
  }
  // nullary-after-normal-constructor-miss
  pure val frontierInput3: F::FrontierInput = {
    sourceMatchId: 704,
    arms: List({sourceArmId: 7040, bodyExprId: 900, patternRoot: 0}, {sourceArmId: 7041, bodyExprId: 901, patternRoot: 3}, {sourceArmId: 7042, bodyExprId: 902, patternRoot: 4}),
    valueRoot: 0,
    valueRows: Map(0 -> {token: ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), children: {start: 0, len: 0}}),
    patternRows: Map(0 -> {token: PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 1 -> {token: PatternBind(11), children: {start: 0, len: 0}}, 2 -> {token: PatternWildcard, children: {start: 0, len: 0}}, 3 -> {token: PatternConstructor({typeId: 20, ctorId: 21, arity: 0}), children: {start: 2, len: 0}}, 4 -> {token: PatternBind(13), children: {start: 2, len: 0}}),
    valueFields: List(),
    patternFields: List(1, 2),
    decodedValues: Map(0 -> List(ConstructorToken({typeId: 20, ctorId: 21, arity: 0}))),
    encodedPatterns: Map(1 -> List(PatternBind(11)), 2 -> List(PatternWildcard), 0 -> List(PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(11), PatternWildcard), 3 -> List(PatternConstructor({typeId: 20, ctorId: 21, arity: 0})), 4 -> List(PatternBind(13))),
    sourcePaths: List(List(List(), List(0), List(1)), List(List()), List(List())),
    binderOrigins: List(List({name: 11, path: List(0)}), List(), List({name: 13, path: List()})),
    savedEnvironment: Map(10 -> List(UIntToken(11)), 11 -> List(UIntToken(77)), 20 -> List(UIntToken(7))),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}),
    analysisBounds: {advances: 32, maxTasks: 12, maxPath: 4}
  }
  // three-field-nominal-parent-retains-cursor
  pure val frontierInput4: F::FrontierInput = {
    sourceMatchId: 705,
    arms: List({sourceArmId: 7050, bodyExprId: 900, patternRoot: 0}, {sourceArmId: 7051, bodyExprId: 901, patternRoot: 6}),
    valueRoot: 4,
    valueRows: Map(0 -> {token: UIntToken(4), children: {start: 0, len: 0}}, 1 -> {token: UIntToken(7), children: {start: 0, len: 0}}, 2 -> {token: ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), children: {start: 0, len: 0}}, 3 -> {token: ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 4 -> {token: ConstructorToken({typeId: 30, ctorId: 300, arity: 3}), children: {start: 2, len: 3}}),
    patternRows: Map(0 -> {token: PatternConstructor({typeId: 30, ctorId: 300, arity: 3}), children: {start: 2, len: 3}}, 1 -> {token: PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), children: {start: 0, len: 2}}, 2 -> {token: PatternBind(11), children: {start: 0, len: 0}}, 3 -> {token: PatternWildcard, children: {start: 0, len: 0}}, 4 -> {token: PatternBind(12), children: {start: 2, len: 0}}, 5 -> {token: PatternConstructor({typeId: 20, ctorId: 21, arity: 0}), children: {start: 2, len: 0}}, 6 -> {token: PatternBind(13), children: {start: 5, len: 0}}),
    valueFields: List(0, 2, 3, 1, 2),
    patternFields: List(2, 3, 1, 4, 5),
    decodedValues: Map(0 -> List(UIntToken(4)), 1 -> List(UIntToken(7)), 2 -> List(ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 3 -> List(ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 21, arity: 0})), 4 -> List(ConstructorToken({typeId: 30, ctorId: 300, arity: 3}), ConstructorToken({typeId: 20, ctorId: 22, arity: 2}), UIntToken(4), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}), UIntToken(7), ConstructorToken({typeId: 20, ctorId: 21, arity: 0}))),
    encodedPatterns: Map(2 -> List(PatternBind(11)), 3 -> List(PatternWildcard), 1 -> List(PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(11), PatternWildcard), 4 -> List(PatternBind(12)), 5 -> List(PatternConstructor({typeId: 20, ctorId: 21, arity: 0})), 0 -> List(PatternConstructor({typeId: 30, ctorId: 300, arity: 3}), PatternConstructor({typeId: 20, ctorId: 22, arity: 2}), PatternBind(11), PatternWildcard, PatternBind(12), PatternConstructor({typeId: 20, ctorId: 21, arity: 0})), 6 -> List(PatternBind(13))),
    sourcePaths: List(List(List(), List(0), List(0, 0), List(0, 1), List(1), List(2)), List(List())),
    binderOrigins: List(List({name: 11, path: List(0, 0)}, {name: 12, path: List(1)}), List({name: 13, path: List()})),
    savedEnvironment: Map(10 -> List(UIntToken(11)), 11 -> List(UIntToken(77)), 20 -> List(UIntToken(7))),
    layouts: Map((20, 21) -> {typeId: 20, ctorId: 21, fields: List()}, (20, 22) -> {typeId: 20, ctorId: 22, fields: List(0, 20)}, (20, 23) -> {typeId: 20, ctorId: 23, fields: List(20, 20)}, (30, 300) -> {typeId: 30, ctorId: 300, fields: List(20, 0, 20)}),
    analysisBounds: {advances: 32, maxTasks: 12, maxPath: 4}
  }
  pure val frontierCohort: List[F::FrontierInput] = List(frontierInput0, frontierInput1, frontierInput2, frontierInput3, frontierInput4)
}
