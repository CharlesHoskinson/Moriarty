module kProfileMachine {
  // TEST-FIRST RED STUB. Tests were written before this model source.
  // Scope: post-startup sherlock-k-a/1 states, pinned main.k digest in README.
  // The Eval transition deliberately returns UInt(0), including Literal(UInt(7)).
  // This source must remain RED until the integrator observes the behavioral failure.

  type Name = int
  type ExprId = int
  type TreeToken = TreeLeaf(int) | TreeBranch
  // Prefix tree tokens are structural values, never heap addresses or pointers.
  type Value = UInt(int) | BoolValue(bool) | UIntList(List[int]) | UIntTree(List[TreeToken])
  type ValueType = UIntType | BoolType | ListType | TreeType
  type BinaryOp = Add | Sub | Mul | Eq | Lt | ConsOp | BranchOp
  type UnaryOp = LeafOp | NotOp
  type Environment = Name -> Value
  type Pattern = PVar(Name) | PNil | PCons({head: Name, tail: Name})
    | PLeaf(Name) | PBranch({left: Name, right: Name})
  type Arm = {pattern: Pattern, body: ExprId}
  type Expr = Literal(Value) | Variable(Name)
    | Let({name: Name, value: ExprId, body: ExprId})
    | Binary({op: BinaryOp, left: ExprId, right: ExprId})
    | Unary({op: UnaryOp, argument: ExprId})
    | If({condition: ExprId, yes: ExprId, no: ExprId})
    | Call({name: Name, arguments: List[ExprId]})
    | Match({value: ExprId, arms: List[Arm]})
  type Parameter = {name: Name, ty: ValueType}
  type Function = {
    name: Name, parameters: List[Parameter], returnType: ValueType, body: ExprId,
  }
  // Keep source order. Future lookup must select the first matching function.
  // Calls hold names, so recursive calls reuse this immutable syntax table.
  type Program = {expressions: ExprId -> Expr, functions: List[Function], entry: ExprId}
  type Frame = LetFrame({name: Name, body: ExprId, environment: Environment})
    | LeftFrame({op: BinaryOp, right: ExprId, environment: Environment})
    | RightFrame({op: BinaryOp, left: Value}) | UnaryFrame(UnaryOp)
    | IfFrame({yes: ExprId, no: ExprId, environment: Environment})
    | MatchFrame({arms: List[Arm], environment: Environment})
    | ArgsFrame({name: Name, values: List[Value], remaining: List[ExprId], environment: Environment})
    | ReturnFrame({ty: ValueType, environment: Environment}) | ScopeFrame(Environment)
  type FunctionLookup = MissingFunction | FoundFunction(Function)
  type Binding = BadBinding | Bindings(Environment)
  type Matching = NoMatch | Matched(Environment)
  type Primitive = PrimitiveValue(Value) | PrimitiveError(int)
  // In frozen K-A every uiPush continuation is uiEval(E); no recursive Quint type.
  type Control = EmptyControl | Eval(ExprId) | Return(Value)
    | Push({frame: Frame, expression: ExprId, environment: Environment})
    | Invoke({name: Name, values: List[Value]})
    | Enter({found: FunctionLookup, values: List[Value]})
    | Bound({bindings: Binding, ty: ValueType, expression: ExprId})
    | Cases({value: Value, arms: List[Arm], environment: Environment})
    | CaseResult({matching: Matching, value: Value, expression: ExprId, remaining: List[Arm], environment: Environment})
    | PrimitiveResult(Primitive) | Error(int)
  type Outcome = Pending | Done(Value) | Rejected(int) | Exhausted
  type Snapshot = {
    program: Program, control: Control, environment: Environment, frames: List[Frame],
    stackLimit: int, remaining: int, initial: int, outcome: Outcome,
  }
  type Transition = {state: Snapshot, rule: int, charge: int}

  pure val PROFILE = "sherlock-k-a/1"
  pure val MAX_UINT128 = 340282366920938463463374607431768211455
  // Exact mathematical integer endpoints, not the historical scalar-15 instance.
  pure def inUInt128(n: int): bool = 0 <= n and n <= MAX_UINT128

  var machine: Snapshot
  var lastRule: int
  var lastCharge: int

  pure def initialSnapshot(p: Program, fuel: int, limit: int): Snapshot = {
    program: p, control: Eval(p.entry), environment: Map(), frames: List(),
    stackLimit: limit, remaining: fuel, initial: fuel, outcome: Pending,
  }

  action initProgram(p: Program, fuel: int, limit: int): bool = all {
    fuel >= 0, limit >= 0,
    machine' = initialSnapshot(p, fuel, limit), lastRule' = 74, lastCharge' = 0,
  }

  // Terminal publication is separate and free, before the zero-fuel check.
  // Only the deliberately wrong ordinary transition below is implemented.
  pure def stubStep(s: Snapshot): Transition =
    if (s.outcome != Pending) {state: s, rule: 0, charge: 0}
    else match s.control {
      | Return(value) =>
        if (s.frames.length() == 0)
          {state: {...s, control: EmptyControl, outcome: Done(value)}, rule: 80, charge: 0}
        else if (s.remaining == 0)
          {state: {...s, control: EmptyControl, outcome: Exhausted}, rule: 84, charge: 0}
        else {state: s, rule: 0, charge: 0}
      | Error(code) =>
        {state: {...s, control: EmptyControl, outcome: Rejected(code)}, rule: 82, charge: 0}
      | _ =>
        if (s.remaining == 0)
          {state: {...s, control: EmptyControl, outcome: Exhausted}, rule: 84, charge: 0}
        else {
          state: {...s, control: Return(UInt(0)), remaining: s.remaining - 1},
          rule: 102, charge: 1,
        }
    }

  action step = {
    val transition = stubStep(machine)
    all {machine' = transition.state, lastRule' = transition.rule, lastCharge' = transition.charge}
  }

  // Concrete, deterministic entry point; tests supply independent program tables.
  action init = initProgram({expressions: Map(0 -> Literal(UInt(7))), functions: List(), entry: 0}, 1, 8)
}
