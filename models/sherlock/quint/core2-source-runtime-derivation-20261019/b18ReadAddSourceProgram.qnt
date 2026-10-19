module b18ReadAddSourceProgram {
  import financialBoundary.* from "./financialBoundary"
  import sourceCore2ReadAdd.* from "./sourceCore2ReadAdd"
  // Source-only projection of fixed B18 advance and the complete source ExprId arena.
  // IDs follow the adopted parser's append-after-operands allocation. No result,
  // snapshot, effect, attempted-work total or RuntimeEvidence is stored here.
  pure val selectedSourceProgram: SourceProgram = {
    semantics: "moriarty-core/2", actionName: "advance",
    expressions: List(SourceRead("marker"), SourceRead("ready"),
      SourceRead("retained"), SourceRead("delta"), SourceUIntAdd({leftId: 2, rightId: 3}),
      SourceRead("ready"), SourceRead("marker"), SourceRead("accepted"),
      SourceLiteral(UIntValue(0)), SourceLiteral(UIntValue(0))),
    statements: List(LetStatement({name: "retained", value: 0}),
      AwaitStatement({label: "gate", binders: List({name: "ready", ty: BoolType}), guard: 1}),
      NextStatement({state: "marker", value: 4}),
      NextStatement({state: "flag", value: 5}),
      LetStatement({name: "seen", value: 6}),
      AwaitStatement({label: "finish", binders: List({name: "accepted", ty: BoolType}), guard: 7})),
  }
}
