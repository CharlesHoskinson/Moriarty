// Independent intended lowering INPUTS ONLY, not an interpreter or adapter.
// Imported K-A semantics unchanged. All model/type/test execution UNRUN.
module coreOperatorQaTest {
  import kProfileMachine.* from "./imports/kProfileMachine"
  type DerivedOp = NeSource | LeSource | GtSource | GeSource | AndSource | OrSource
  type Fragment = {entries: ExprId -> Expr, entry: ExprId}
  pure def attach(table: ExprId -> Expr, fragment: Fragment): ExprId -> Expr =
    fragment.entries.keys().fold(table, (acc, key) => acc.put(key, fragment.entries.get(key)))

  // Chosen independent equations: let l=LEFT in let r=RIGHT in BODY(l,r).
  // Fresh names/ExprIds are declared per finite instance. This constructs syntax
  // only; it does not evaluate values or claim the Rust adapter emits this AST.
  pure def lowered(op: DerivedOp, left: ExprId, right: ExprId, base: int): Fragment = {
    val leftName = -1000 - base * 2
    val rightName = -1001 - base * 2
    val body = if (op == GtSource or op == AndSource or op == OrSource) base + 4 else base + 5
    val core: Expr = if (op == NeSource) Binary({op: Eq, left: base + 2, right: base + 3})
      else if (op == LeSource or op == GtSource) Binary({op: Lt, left: base + 3, right: base + 2})
      else if (op == GeSource) Binary({op: Lt, left: base + 2, right: base + 3})
      else if (op == AndSource) If({condition: base + 2, yes: base + 3, no: base + 8})
      else If({condition: base + 2, yes: base + 8, no: base + 3})
    {entry: base + 7, entries: Map(
      base + 2 -> Variable(leftName), base + 3 -> Variable(rightName), base + 4 -> core,
      base + 5 -> Unary({op: NotOp, argument: base + 4}),
      base + 6 -> Let({name: rightName, value: right, body: body}),
      base + 7 -> Let({name: leftName, value: left, body: base + 6}),
      base + 8 -> Literal(BoolValue(op == OrSource)))}
  }
  pure def unitProgram(op: DerivedOp, left: Value, right: Value): Program = {
    expressions: attach(Map(0 -> Literal(left), 1 -> Literal(right)), lowered(op, 0, 1, 0)),
    functions: List(), entry: 7}
  pure val leAscending = unitProgram(LeSource, UInt(7), UInt(11))
  pure val andFalse = unitProgram(AndSource, BoolValue(false), BoolValue(true))
  pure val orTrue = unitProgram(OrSource, BoolValue(true), BoolValue(false))

  // Exact public fixture inputs, with emit represented ONLY by returned scalar.
  // Source input names:10=left,11=right. Local names20..26 are source lets.
  pure def comparisonFixture(equalCase: bool): Program = {
    val vars: ExprId -> Expr = Map(
      9000 -> Variable(10), 9001 -> Variable(11), 9002 -> Variable(20),
      9003 -> Variable(21), 9004 -> Variable(22), 9005 -> Variable(23),
      9006 -> Variable(24), 9007 -> Variable(25), 9008 -> Variable(26),
      9200 -> Literal(UInt(if (equalCase) 13 else 7)),
      9201 -> Literal(UInt(if (equalCase) 13 else 11)),
      9100 -> Let({name: 10, value: 9200, body: 9101}),
      9101 -> Let({name: 11, value: 9201, body: 9102}),
      9102 -> Let({name: 20, value: 107, body: 9103}),
      9103 -> Let({name: 21, value: 207, body: 9104}),
      9104 -> Let({name: 22, value: 307, body: 9105}),
      9105 -> Let({name: 23, value: 407, body: 9106}),
      9106 -> Let({name: 24, value: 507, body: 9107}),
      9107 -> Let({name: 25, value: 607, body: 9108}),
      9108 -> Let({name: 26, value: 707, body: 9008}))
    val t1 = attach(vars, lowered(NeSource, 9000, 9001, 100))
    val t2 = attach(t1, lowered(LeSource, 9000, 9001, 200))
    val t3 = attach(t2, lowered(GtSource, if (equalCase) 9000 else 9001,
      if (equalCase) 9001 else 9000, 300))
    val t4 = attach(t3, lowered(GeSource, 9000, 9001, 400))
    val t5 = attach(t4, lowered(AndSource, 9002, 9003, 500))
    val t6 = attach(t5, lowered(OrSource, 9004, 9005, 600))
    val t7 = if (equalCase) attach(t6, lowered(AndSource, 9004, 9005, 800)) else t6
    val t8 = attach(t7, lowered(if (equalCase) OrSource else AndSource,
      9006, if (equalCase) 807 else 9007, 700))
    {expressions: t8, functions: List(), entry: 9100}
  }
  pure val ascendingFixture = comparisonFixture(false)
  pure val equalFixture = comparisonFixture(true)
  pure def subtractFixture(minuend: int, subtrahend: int): Program = {
    expressions: Map(
      0 -> Literal(UInt(minuend)), 1 -> Literal(UInt(subtrahend)),
      2 -> Variable(10), 3 -> Variable(11),
      4 -> Binary({op: Sub, left: 2, right: 3}),
      5 -> Variable(20), 6 -> Literal(UInt(0)), 7 -> Binary({op: Eq, left: 5, right: 6}),
      8 -> Let({name: 20, value: 4, body: 7}),
      9 -> Let({name: 11, value: 1, body: 8}),
      10 -> Let({name: 10, value: 0, body: 9})), functions: List(), entry: 10}
  pure val subtractPositiveFixture = subtractFixture(11, 7)
  pure val subtractZeroFixture = subtractFixture(13, 13)
  pure def directBinary(op: BinaryOp, left: Value, right: Value): Program = {
    expressions: Map(0 -> Literal(left), 1 -> Literal(right),
      2 -> Binary({op: op, left: 0, right: 1})), functions: List(), entry: 2}
  pure val directNot: Program = {expressions: Map(0 -> Literal(BoolValue(true)),
    1 -> Unary({op: NotOp, argument: 0})), functions: List(), entry: 1}

  // Independently hand-declared complete expected states. This constructor only
  // assembles literal fields; it never invokes advance/evaluate/ordinaryStep.
  pure def expected(p: Program, fuel: int, used: int, cc: Control,
    env: Environment, ff: List[Frame], oo: Outcome): Snapshot = {
    program: p, control: cc, environment: env, frames: ff, stackLimit: 16,
    remaining: fuel - used, initial: fuel, outcome: oo}
  pure val andL: Environment = Map(-1000 -> BoolValue(false))
  pure val andLR: Environment = Map(-1000 -> BoolValue(false), -1001 -> BoolValue(true))
  pure val leL: Environment = Map(-1000 -> UInt(7))
  pure val leLR: Environment = Map(-1000 -> UInt(7), -1001 -> UInt(11))
  pure val andScope0: Frame = ScopeFrame(Map())
  pure val andScopeL: Frame = ScopeFrame(andL)
  pure val leScope0: Frame = ScopeFrame(Map())
  pure val leScopeL: Frame = ScopeFrame(leL)
  pure val andLetL: Frame = LetFrame({name: -1000, body: 6, environment: Map()})
  pure val andLetR: Frame = LetFrame({name: -1001, body: 4, environment: andL})
  pure val andIf: Frame = IfFrame({yes: 3, no: 8, environment: andLR})
  pure val leLetL: Frame = LetFrame({name: -1000, body: 6, environment: Map()})
  pure val leLetR: Frame = LetFrame({name: -1001, body: 5, environment: leL})
  pure val leUnary: Frame = UnaryFrame(NotOp)
  pure val leLeft: Frame = LeftFrame({op: Lt, right: 2, environment: leLR})
  pure val leRight: Frame = RightFrame({op: Lt, left: UInt(11)})

  pure val andSnapshots: List[Snapshot] = List(
    expected(andFalse, 15, 0, Eval(7), Map(), List(), Pending),
    expected(andFalse, 15, 1, Push({frame: andLetL, expression: 0, environment: Map()}), Map(), List(), Pending),
    expected(andFalse, 15, 2, Eval(0), Map(), List(andLetL), Pending),
    expected(andFalse, 15, 3, Return(BoolValue(false)), Map(), List(andLetL), Pending),
    expected(andFalse, 15, 4, Eval(6), andL, List(andScope0), Pending),
    expected(andFalse, 15, 5, Push({frame: andLetR, expression: 1, environment: andL}), andL, List(andScope0), Pending),
    expected(andFalse, 15, 6, Eval(1), andL, List(andLetR, andScope0), Pending),
    expected(andFalse, 15, 7, Return(BoolValue(true)), andL, List(andLetR, andScope0), Pending),
    expected(andFalse, 15, 8, Eval(4), andLR, List(andScopeL, andScope0), Pending),
    expected(andFalse, 15, 9, Push({frame: andIf, expression: 2, environment: andLR}), andLR, List(andScopeL, andScope0), Pending),
    expected(andFalse, 15, 10, Eval(2), andLR, List(andIf, andScopeL, andScope0), Pending),
    expected(andFalse, 15, 11, Return(BoolValue(false)), andLR, List(andIf, andScopeL, andScope0), Pending),
    expected(andFalse, 15, 12, Eval(8), andLR, List(andScopeL, andScope0), Pending),
    expected(andFalse, 15, 13, Return(BoolValue(false)), andLR, List(andScopeL, andScope0), Pending),
    expected(andFalse, 15, 14, Return(BoolValue(false)), andL, List(andScope0), Pending),
    expected(andFalse, 15, 15, Return(BoolValue(false)), Map(), List(), Pending),
    expected(andFalse, 15, 15, EmptyControl, Map(), List(), Done(BoolValue(false))))
  pure val andRules: List[int] = List(74, 114, 135, 102, 145, 114, 135, 102, 145, 123, 135, 108, 165, 102, 195, 195, 80)
  pure val andCharges: List[int] = List(0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0)
  run andCompleteTraceTest = initProgram(andFalse, 15, 16)
    .expect(machine == andSnapshots.nth(0) and lastRule == 74 and lastCharge == 0)
    .then(step).expect(machine == andSnapshots.nth(1) and lastRule == 114 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(2) and lastRule == 135 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(3) and lastRule == 102 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(4) and lastRule == 145 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(5) and lastRule == 114 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(6) and lastRule == 135 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(7) and lastRule == 102 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(8) and lastRule == 145 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(9) and lastRule == 123 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(10) and lastRule == 135 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(11) and lastRule == 108 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(12) and lastRule == 165 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(13) and lastRule == 102 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(14) and lastRule == 195 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(15) and lastRule == 195 and lastCharge == 1)
    .then(step).expect(machine == andSnapshots.nth(16) and lastRule == 80 and lastCharge == 0)
  pure def andFuelPrefix(fuel: int): Snapshot =
    if (fuel == 15) andSnapshots.nth(16)
    else {...andSnapshots.nth(fuel), control: EmptyControl, outcome: Exhausted,
      remaining: 0, initial: fuel}
  run andAllFuelPrefixesTest = {
    nondet fuel = 0.to(15).oneOf()
    initProgram(andFalse, fuel, 16).then((fuel + 1).reps(_ => step))
      .expect(machine == andFuelPrefix(fuel) and
        lastRule == (if (fuel == 15) 80 else 84) and lastCharge == 0)
  }

  pure val leSnapshots: List[Snapshot] = List(
    expected(leAscending, 21, 0, Eval(7), Map(), List(), Pending),
    expected(leAscending, 21, 1, Push({frame: leLetL, expression: 0, environment: Map()}), Map(), List(), Pending),
    expected(leAscending, 21, 2, Eval(0), Map(), List(leLetL), Pending),
    expected(leAscending, 21, 3, Return(UInt(7)), Map(), List(leLetL), Pending),
    expected(leAscending, 21, 4, Eval(6), leL, List(leScope0), Pending),
    expected(leAscending, 21, 5, Push({frame: leLetR, expression: 1, environment: leL}), leL, List(leScope0), Pending),
    expected(leAscending, 21, 6, Eval(1), leL, List(leLetR, leScope0), Pending),
    expected(leAscending, 21, 7, Return(UInt(11)), leL, List(leLetR, leScope0), Pending),
    expected(leAscending, 21, 8, Eval(5), leLR, List(leScopeL, leScope0), Pending),
    expected(leAscending, 21, 9, Push({frame: leUnary, expression: 4, environment: leLR}), leLR, List(leScopeL, leScope0), Pending),
    expected(leAscending, 21, 10, Eval(4), leLR, List(leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 11, Push({frame: leLeft, expression: 3, environment: leLR}), leLR, List(leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 12, Eval(3), leLR, List(leLeft, leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 13, Return(UInt(11)), leLR, List(leLeft, leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 14, Eval(2), leLR, List(leRight, leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 15, Return(UInt(7)), leLR, List(leRight, leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 16, PrimitiveResult(PrimitiveValue(BoolValue(false))), leLR, List(leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 17, Return(BoolValue(false)), leLR, List(leUnary, leScopeL, leScope0), Pending),
    expected(leAscending, 21, 18, PrimitiveResult(PrimitiveValue(BoolValue(true))), leLR, List(leScopeL, leScope0), Pending),
    expected(leAscending, 21, 19, Return(BoolValue(true)), leLR, List(leScopeL, leScope0), Pending),
    expected(leAscending, 21, 20, Return(BoolValue(true)), leL, List(leScope0), Pending),
    expected(leAscending, 21, 21, Return(BoolValue(true)), Map(), List(), Pending),
    expected(leAscending, 21, 21, EmptyControl, Map(), List(), Done(BoolValue(true))))
  pure val leRules: List[int] = List(74, 114, 135, 102, 145, 114, 135, 102, 145, 120, 135, 117, 135, 108, 149, 108, 153, 223, 157, 223, 195, 195, 80)
  pure val leCharges: List[int] = List(0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0)
  run leCompleteTraceTest = initProgram(leAscending, 21, 16)
    .expect(machine == leSnapshots.nth(0) and lastRule == 74 and lastCharge == 0)
    .then(step).expect(machine == leSnapshots.nth(1) and lastRule == 114 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(2) and lastRule == 135 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(3) and lastRule == 102 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(4) and lastRule == 145 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(5) and lastRule == 114 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(6) and lastRule == 135 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(7) and lastRule == 102 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(8) and lastRule == 145 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(9) and lastRule == 120 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(10) and lastRule == 135 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(11) and lastRule == 117 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(12) and lastRule == 135 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(13) and lastRule == 108 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(14) and lastRule == 149 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(15) and lastRule == 108 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(16) and lastRule == 153 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(17) and lastRule == 223 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(18) and lastRule == 157 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(19) and lastRule == 223 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(20) and lastRule == 195 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(21) and lastRule == 195 and lastCharge == 1)
    .then(step).expect(machine == leSnapshots.nth(22) and lastRule == 80 and lastCharge == 0)
  pure def leFuelPrefix(fuel: int): Snapshot =
    if (fuel == 21) leSnapshots.nth(22)
    else {...leSnapshots.nth(fuel), control: EmptyControl, outcome: Exhausted,
      remaining: 0, initial: fuel}
  run leAllFuelPrefixesTest = {
    nondet fuel = 0.to(21).oneOf()
    initProgram(leAscending, fuel, 16).then((fuel + 1).reps(_ => step))
      .expect(machine == leFuelPrefix(fuel) and
        lastRule == (if (fuel == 21) 80 else 84) and lastCharge == 0)
  }

  run neDistinctMeaningTest = initProgram(unitProgram(NeSource, UInt(7), UInt(11)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(NeSource, UInt(7), UInt(11)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run neEqualMeaningTest = initProgram(unitProgram(NeSource, UInt(13), UInt(13)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(NeSource, UInt(13), UInt(13)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run leAscendingMeaningTest = initProgram(unitProgram(LeSource, UInt(7), UInt(11)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(LeSource, UInt(7), UInt(11)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run leEqualMeaningTest = initProgram(unitProgram(LeSource, UInt(13), UInt(13)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(LeSource, UInt(13), UInt(13)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run leDescendingMeaningTest = initProgram(unitProgram(LeSource, UInt(11), UInt(7)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(LeSource, UInt(11), UInt(7)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run gtAscendingMeaningTest = initProgram(unitProgram(GtSource, UInt(7), UInt(11)), 64, 16)
    .then(18.reps(_ => step)).expect(and {
      machine == expected(unitProgram(GtSource, UInt(7), UInt(11)), 64, 17, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run gtDescendingMeaningTest = initProgram(unitProgram(GtSource, UInt(11), UInt(7)), 64, 16)
    .then(18.reps(_ => step)).expect(and {
      machine == expected(unitProgram(GtSource, UInt(11), UInt(7)), 64, 17, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run gtEqualMeaningTest = initProgram(unitProgram(GtSource, UInt(13), UInt(13)), 64, 16)
    .then(18.reps(_ => step)).expect(and {
      machine == expected(unitProgram(GtSource, UInt(13), UInt(13)), 64, 17, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run geAscendingMeaningTest = initProgram(unitProgram(GeSource, UInt(7), UInt(11)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(GeSource, UInt(7), UInt(11)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run geEqualMeaningTest = initProgram(unitProgram(GeSource, UInt(13), UInt(13)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(GeSource, UInt(13), UInt(13)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run geDescendingMeaningTest = initProgram(unitProgram(GeSource, UInt(11), UInt(7)), 64, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == expected(unitProgram(GeSource, UInt(11), UInt(7)), 64, 21, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run andFalseMeaningTest = initProgram(unitProgram(AndSource, BoolValue(false), BoolValue(true)), 64, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == expected(unitProgram(AndSource, BoolValue(false), BoolValue(true)), 64, 15, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run andTrueFalseMeaningTest = initProgram(unitProgram(AndSource, BoolValue(true), BoolValue(false)), 64, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == expected(unitProgram(AndSource, BoolValue(true), BoolValue(false)), 64, 15, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run andTrueMeaningTest = initProgram(unitProgram(AndSource, BoolValue(true), BoolValue(true)), 64, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == expected(unitProgram(AndSource, BoolValue(true), BoolValue(true)), 64, 15, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run orTrueMeaningTest = initProgram(unitProgram(OrSource, BoolValue(true), BoolValue(false)), 64, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == expected(unitProgram(OrSource, BoolValue(true), BoolValue(false)), 64, 15, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run orFalseTrueMeaningTest = initProgram(unitProgram(OrSource, BoolValue(false), BoolValue(true)), 64, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == expected(unitProgram(OrSource, BoolValue(false), BoolValue(true)), 64, 15, EmptyControl, Map(), List(), Done(BoolValue(true))),
      lastRule == 80, lastCharge == 0,
    })

  run orFalseMeaningTest = initProgram(unitProgram(OrSource, BoolValue(false), BoolValue(false)), 64, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == expected(unitProgram(OrSource, BoolValue(false), BoolValue(false)), 64, 15, EmptyControl, Map(), List(), Done(BoolValue(false))),
      lastRule == 80, lastCharge == 0,
    })

  run directAddBaselineTest = initProgram(directBinary(Add, UInt(7), UInt(11)), 64, 16)
    .then(8.reps(_ => step)).expect(machine == expected(directBinary(Add, UInt(7), UInt(11)), 64, 7,
      EmptyControl, Map(), List(), Done(UInt(18))) and lastRule == 80 and lastCharge == 0)

  run directSubBaselineTest = initProgram(directBinary(Sub, UInt(11), UInt(7)), 64, 16)
    .then(8.reps(_ => step)).expect(machine == expected(directBinary(Sub, UInt(11), UInt(7)), 64, 7,
      EmptyControl, Map(), List(), Done(UInt(4))) and lastRule == 80 and lastCharge == 0)

  run directMulBaselineTest = initProgram(directBinary(Mul, UInt(7), UInt(11)), 64, 16)
    .then(8.reps(_ => step)).expect(machine == expected(directBinary(Mul, UInt(7), UInt(11)), 64, 7,
      EmptyControl, Map(), List(), Done(UInt(77))) and lastRule == 80 and lastCharge == 0)

  run directEqBaselineTest = initProgram(directBinary(Eq, UInt(13), UInt(13)), 64, 16)
    .then(8.reps(_ => step)).expect(machine == expected(directBinary(Eq, UInt(13), UInt(13)), 64, 7,
      EmptyControl, Map(), List(), Done(BoolValue(true))) and lastRule == 80 and lastCharge == 0)

  run directLtBaselineTest = initProgram(directBinary(Lt, UInt(7), UInt(11)), 64, 16)
    .then(8.reps(_ => step)).expect(machine == expected(directBinary(Lt, UInt(7), UInt(11)), 64, 7,
      EmptyControl, Map(), List(), Done(BoolValue(true))) and lastRule == 80 and lastCharge == 0)

  run notBaselineBaselineTest = initProgram(directNot, 64, 16)
    .then(6.reps(_ => step)).expect(machine == expected(directNot, 64, 5,
      EmptyControl, Map(), List(), Done(BoolValue(false))) and lastRule == 80 and lastCharge == 0)

  run ascendingPublicFixtureTest = initProgram(ascendingFixture, 512, 16)
    .then(165.reps(_ => step)).expect(machine == expected(ascendingFixture, 512, 164,
      EmptyControl, Map(), List(), Done(BoolValue(true))) and lastRule == 80 and lastCharge == 0)

  run equalPublicFixtureTest = initProgram(equalFixture, 512, 16)
    .then(179.reps(_ => step)).expect(machine == expected(equalFixture, 512, 178,
      EmptyControl, Map(), List(), Done(BoolValue(false))) and lastRule == 80 and lastCharge == 0)

  run subtractPositivePublicFixtureTest = initProgram(subtractPositiveFixture, 512, 16)
    .then(29.reps(_ => step)).expect(machine == expected(subtractPositiveFixture, 512, 28,
      EmptyControl, Map(), List(), Done(BoolValue(false))) and lastRule == 80 and lastCharge == 0)

  run subtractZeroPublicFixtureTest = initProgram(subtractZeroFixture, 512, 16)
    .then(29.reps(_ => step)).expect(machine == expected(subtractZeroFixture, 512, 28,
      EmptyControl, Map(), List(), Done(BoolValue(true))) and lastRule == 80 and lastCharge == 0)

  // Version02 coverage extension: deterministic singleton cases reuse the
  // independently frozen prefix functions above. No expectations are changed.
  // The two original nondeterministic families remain sampled families.
  run andFuel00DeterministicTest = initProgram(andFalse, 0, 16)
    .then(1.reps(_ => step)).expect(and {
      machine == andFuelPrefix(0), lastRule == 84, lastCharge == 0,
    })
  run andFuel01DeterministicTest = initProgram(andFalse, 1, 16)
    .then(2.reps(_ => step)).expect(and {
      machine == andFuelPrefix(1), lastRule == 84, lastCharge == 0,
    })
  run andFuel02DeterministicTest = initProgram(andFalse, 2, 16)
    .then(3.reps(_ => step)).expect(and {
      machine == andFuelPrefix(2), lastRule == 84, lastCharge == 0,
    })
  run andFuel03DeterministicTest = initProgram(andFalse, 3, 16)
    .then(4.reps(_ => step)).expect(and {
      machine == andFuelPrefix(3), lastRule == 84, lastCharge == 0,
    })
  run andFuel04DeterministicTest = initProgram(andFalse, 4, 16)
    .then(5.reps(_ => step)).expect(and {
      machine == andFuelPrefix(4), lastRule == 84, lastCharge == 0,
    })
  run andFuel05DeterministicTest = initProgram(andFalse, 5, 16)
    .then(6.reps(_ => step)).expect(and {
      machine == andFuelPrefix(5), lastRule == 84, lastCharge == 0,
    })
  run andFuel06DeterministicTest = initProgram(andFalse, 6, 16)
    .then(7.reps(_ => step)).expect(and {
      machine == andFuelPrefix(6), lastRule == 84, lastCharge == 0,
    })
  run andFuel07DeterministicTest = initProgram(andFalse, 7, 16)
    .then(8.reps(_ => step)).expect(and {
      machine == andFuelPrefix(7), lastRule == 84, lastCharge == 0,
    })
  run andFuel08DeterministicTest = initProgram(andFalse, 8, 16)
    .then(9.reps(_ => step)).expect(and {
      machine == andFuelPrefix(8), lastRule == 84, lastCharge == 0,
    })
  run andFuel09DeterministicTest = initProgram(andFalse, 9, 16)
    .then(10.reps(_ => step)).expect(and {
      machine == andFuelPrefix(9), lastRule == 84, lastCharge == 0,
    })
  run andFuel10DeterministicTest = initProgram(andFalse, 10, 16)
    .then(11.reps(_ => step)).expect(and {
      machine == andFuelPrefix(10), lastRule == 84, lastCharge == 0,
    })
  run andFuel11DeterministicTest = initProgram(andFalse, 11, 16)
    .then(12.reps(_ => step)).expect(and {
      machine == andFuelPrefix(11), lastRule == 84, lastCharge == 0,
    })
  run andFuel12DeterministicTest = initProgram(andFalse, 12, 16)
    .then(13.reps(_ => step)).expect(and {
      machine == andFuelPrefix(12), lastRule == 84, lastCharge == 0,
    })
  run andFuel13DeterministicTest = initProgram(andFalse, 13, 16)
    .then(14.reps(_ => step)).expect(and {
      machine == andFuelPrefix(13), lastRule == 84, lastCharge == 0,
    })
  run andFuel14DeterministicTest = initProgram(andFalse, 14, 16)
    .then(15.reps(_ => step)).expect(and {
      machine == andFuelPrefix(14), lastRule == 84, lastCharge == 0,
    })
  run andFuel15DeterministicTest = initProgram(andFalse, 15, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == andFuelPrefix(15), lastRule == 80, lastCharge == 0,
    })
  run leFuel00DeterministicTest = initProgram(leAscending, 0, 16)
    .then(1.reps(_ => step)).expect(and {
      machine == leFuelPrefix(0), lastRule == 84, lastCharge == 0,
    })
  run leFuel01DeterministicTest = initProgram(leAscending, 1, 16)
    .then(2.reps(_ => step)).expect(and {
      machine == leFuelPrefix(1), lastRule == 84, lastCharge == 0,
    })
  run leFuel02DeterministicTest = initProgram(leAscending, 2, 16)
    .then(3.reps(_ => step)).expect(and {
      machine == leFuelPrefix(2), lastRule == 84, lastCharge == 0,
    })
  run leFuel03DeterministicTest = initProgram(leAscending, 3, 16)
    .then(4.reps(_ => step)).expect(and {
      machine == leFuelPrefix(3), lastRule == 84, lastCharge == 0,
    })
  run leFuel04DeterministicTest = initProgram(leAscending, 4, 16)
    .then(5.reps(_ => step)).expect(and {
      machine == leFuelPrefix(4), lastRule == 84, lastCharge == 0,
    })
  run leFuel05DeterministicTest = initProgram(leAscending, 5, 16)
    .then(6.reps(_ => step)).expect(and {
      machine == leFuelPrefix(5), lastRule == 84, lastCharge == 0,
    })
  run leFuel06DeterministicTest = initProgram(leAscending, 6, 16)
    .then(7.reps(_ => step)).expect(and {
      machine == leFuelPrefix(6), lastRule == 84, lastCharge == 0,
    })
  run leFuel07DeterministicTest = initProgram(leAscending, 7, 16)
    .then(8.reps(_ => step)).expect(and {
      machine == leFuelPrefix(7), lastRule == 84, lastCharge == 0,
    })
  run leFuel08DeterministicTest = initProgram(leAscending, 8, 16)
    .then(9.reps(_ => step)).expect(and {
      machine == leFuelPrefix(8), lastRule == 84, lastCharge == 0,
    })
  run leFuel09DeterministicTest = initProgram(leAscending, 9, 16)
    .then(10.reps(_ => step)).expect(and {
      machine == leFuelPrefix(9), lastRule == 84, lastCharge == 0,
    })
  run leFuel10DeterministicTest = initProgram(leAscending, 10, 16)
    .then(11.reps(_ => step)).expect(and {
      machine == leFuelPrefix(10), lastRule == 84, lastCharge == 0,
    })
  run leFuel11DeterministicTest = initProgram(leAscending, 11, 16)
    .then(12.reps(_ => step)).expect(and {
      machine == leFuelPrefix(11), lastRule == 84, lastCharge == 0,
    })
  run leFuel12DeterministicTest = initProgram(leAscending, 12, 16)
    .then(13.reps(_ => step)).expect(and {
      machine == leFuelPrefix(12), lastRule == 84, lastCharge == 0,
    })
  run leFuel13DeterministicTest = initProgram(leAscending, 13, 16)
    .then(14.reps(_ => step)).expect(and {
      machine == leFuelPrefix(13), lastRule == 84, lastCharge == 0,
    })
  run leFuel14DeterministicTest = initProgram(leAscending, 14, 16)
    .then(15.reps(_ => step)).expect(and {
      machine == leFuelPrefix(14), lastRule == 84, lastCharge == 0,
    })
  run leFuel15DeterministicTest = initProgram(leAscending, 15, 16)
    .then(16.reps(_ => step)).expect(and {
      machine == leFuelPrefix(15), lastRule == 84, lastCharge == 0,
    })
  run leFuel16DeterministicTest = initProgram(leAscending, 16, 16)
    .then(17.reps(_ => step)).expect(and {
      machine == leFuelPrefix(16), lastRule == 84, lastCharge == 0,
    })
  run leFuel17DeterministicTest = initProgram(leAscending, 17, 16)
    .then(18.reps(_ => step)).expect(and {
      machine == leFuelPrefix(17), lastRule == 84, lastCharge == 0,
    })
  run leFuel18DeterministicTest = initProgram(leAscending, 18, 16)
    .then(19.reps(_ => step)).expect(and {
      machine == leFuelPrefix(18), lastRule == 84, lastCharge == 0,
    })
  run leFuel19DeterministicTest = initProgram(leAscending, 19, 16)
    .then(20.reps(_ => step)).expect(and {
      machine == leFuelPrefix(19), lastRule == 84, lastCharge == 0,
    })
  run leFuel20DeterministicTest = initProgram(leAscending, 20, 16)
    .then(21.reps(_ => step)).expect(and {
      machine == leFuelPrefix(20), lastRule == 84, lastCharge == 0,
    })
  run leFuel21DeterministicTest = initProgram(leAscending, 21, 16)
    .then(22.reps(_ => step)).expect(and {
      machine == leFuelPrefix(21), lastRule == 80, lastCharge == 0,
    })
}
