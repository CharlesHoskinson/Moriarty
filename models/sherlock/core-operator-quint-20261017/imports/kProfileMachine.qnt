module kProfileMachine {
  // Generic frozen K-A interpreter. Independent tests preceded implementation.
  // Scope: post-startup sherlock-k-a/1 states, pinned main.k digest in README.
  // The integrator observed the deliberate UInt(0) RED before this implementation.
  // This candidate is source-only until its own ordinary model checks are captured.

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
  // Keep source order. Lookup selects the first matching function.
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

  // These folds normalize A helpers inside one machine rule. They are not B steps.
  // A prefix tree starts with one pending child; a branch adds one pending child.
  pure def validTree(tokens: List[TreeToken]): bool = {
    val scan = tokens.foldl({pending: 1, valid: true}, (acc, token) =>
      if (acc.pending <= 0) {pending: acc.pending, valid: false}
      else match token {
        | TreeLeaf(number) => {pending: acc.pending - 1, valid: acc.valid and inUInt128(number)}
        | TreeBranch => {pending: acc.pending + 1, valid: acc.valid}
      })
    scan.valid and scan.pending == 0
  }

  pure def validValue(value: Value): bool = match value {
    | UInt(number) => inUInt128(number)
    | BoolValue(_) => true
    | UIntList(elements) => elements.foldl(true, (ok, number) => ok and inUInt128(number))
    | UIntTree(tokens) => validTree(tokens)
  }

  // K uiHasType checks the sort only, separately from uiValid.
  pure def hasType(value: Value, ty: ValueType): bool = match value {
    | UInt(_) => ty == UIntType
    | BoolValue(_) => ty == BoolType
    | UIntList(_) => ty == ListType
    | UIntTree(_) => ty == TreeType
  }

  // Payload selectors are used only behind their matching sort guards below.
  pure def uintPayload(value: Value): int = match value {
    | UInt(number) => number
    | _ => 0
  }
  pure def listPayload(value: Value): List[int] = match value {
    | UIntList(elements) => elements
    | _ => List()
  }
  pure def treePayload(value: Value): List[TreeToken] = match value {
    | UIntTree(tokens) => tokens
    | _ => List()
  }

  // Extent of the first complete subtree in a nonempty, well-formed prefix.
  // Ignore later siblings once the first subtree's pending count reaches zero.
  pure def treeExtent(tokens: List[TreeToken]): int = {
    val scan = tokens.foldl({pending: 1, extent: 0}, (acc, token) =>
      if (acc.pending == 0) acc
      else match token {
        | TreeLeaf(_) => {pending: acc.pending - 1, extent: acc.extent + 1}
        | TreeBranch => {pending: acc.pending + 1, extent: acc.extent + 1}
      })
    scan.extent
  }

  pure def checkedUInt(number: int): Primitive =
    if (inUInt128(number)) PrimitiveValue(UInt(number)) else PrimitiveError(10)

  pure def binaryResult(op: BinaryOp, leftValue: Value, rightValue: Value): Primitive =
    match op {
      | Eq => PrimitiveValue(BoolValue(leftValue == rightValue))
      | Add =>
        if (hasType(leftValue, UIntType) and hasType(rightValue, UIntType))
          checkedUInt(uintPayload(leftValue) + uintPayload(rightValue))
        else PrimitiveError(11)
      | Sub =>
        if (hasType(leftValue, UIntType) and hasType(rightValue, UIntType))
          checkedUInt(uintPayload(leftValue) - uintPayload(rightValue))
        else PrimitiveError(11)
      | Mul =>
        if (hasType(leftValue, UIntType) and hasType(rightValue, UIntType))
          checkedUInt(uintPayload(leftValue) * uintPayload(rightValue))
        else PrimitiveError(11)
      | Lt =>
        if (hasType(leftValue, UIntType) and hasType(rightValue, UIntType))
          PrimitiveValue(BoolValue(uintPayload(leftValue) < uintPayload(rightValue)))
        else PrimitiveError(11)
      | ConsOp =>
        if (hasType(leftValue, UIntType) and hasType(rightValue, ListType))
          PrimitiveValue(UIntList(List(uintPayload(leftValue)).concat(listPayload(rightValue))))
        else PrimitiveError(11)
      | BranchOp =>
        if (hasType(leftValue, TreeType) and hasType(rightValue, TreeType))
          PrimitiveValue(UIntTree(List(TreeBranch).concat(treePayload(leftValue)).concat(treePayload(rightValue))))
        else PrimitiveError(11)
    }

  pure def unaryResult(op: UnaryOp, value: Value): Primitive = match op {
    | LeafOp => match value {
        | UInt(number) => PrimitiveValue(UIntTree(List(TreeLeaf(number))))
        | _ => PrimitiveError(11)
      }
    | NotOp => match value {
        | BoolValue(condition) => PrimitiveValue(BoolValue(not(condition)))
        | _ => PrimitiveError(11)
      }
  }

  // Ordered fold, preserving the first match even when a later name repeats.
  pure def findFunction(functions: List[Function], target: Name): FunctionLookup =
    functions.foldl(MissingFunction, (found, function) => match found {
      | FoundFunction(_) => found
      | MissingFunction => if (function.name == target) FoundFunction(function) else found
    })

  // uiBind normalizes the parameter tail first. Bindings begin empty and caller
  // locals are restored only by the explicit ReturnFrame, never inherited here.
  pure def bindParameters(parameters: List[Parameter], values: List[Value]): Binding =
    if (parameters.length() != values.length()) BadBinding
    else range(0, parameters.length()).foldr(Bindings(Map()), (index, result) =>
      match result {
        | BadBinding => BadBinding
        | Bindings(environment) => {
            val parameter = parameters.nth(index)
            val value = values.nth(index)
            if (hasType(value, parameter.ty) and not(environment.keys().contains(parameter.name)))
              Bindings(environment.put(parameter.name, value))
            else BadBinding
          }
      })

  pure def matchPattern(pattern: Pattern, value: Value, saved: Environment): Matching =
    match pattern {
      | PVar(name) => Matched(saved.put(name, value))
      | PNil =>
        if (hasType(value, ListType) and listPayload(value).length() == 0)
          Matched(saved) else NoMatch
      | PCons(names) =>
        if (names.head == names.tail or not(hasType(value, ListType))) NoMatch
        else {
          val elements = listPayload(value)
          if (elements.length() == 0) NoMatch
          else Matched(saved.put(names.head, UInt(elements.head())).put(names.tail, UIntList(elements.tail())))
        }
      | PLeaf(name) =>
        if (not(hasType(value, TreeType))) NoMatch
        else {
          val tokens = treePayload(value)
          if (tokens.length() == 0) NoMatch
          else match tokens.head() {
            | TreeLeaf(number) => Matched(saved.put(name, UInt(number)))
            | TreeBranch => NoMatch
          }
        }
      | PBranch(names) =>
        if (names.left == names.right or not(hasType(value, TreeType))) NoMatch
        else {
          val tokens = treePayload(value)
          if (tokens.length() == 0) NoMatch
          else match tokens.head() {
            | TreeLeaf(_) => NoMatch
            | TreeBranch => {
                // On the declared tree domain, siblings contains two nonempty
                // trees and cut lies strictly between zero and its length.
                val siblings = tokens.tail()
                val cut = treeExtent(siblings)
                val leftTree = UIntTree(siblings.slice(0, cut))
                val rightTree = UIntTree(siblings.slice(cut, siblings.length()))
                Matched(saved.put(names.left, leftTree).put(names.right, rightTree))
              }
          }
        }
    }

  pure def machineRule(s: Snapshot, control: Control, line: int): Transition = {
    state: {...s, control: control, remaining: s.remaining - 1}, rule: line, charge: 1,
  }
  pure def pushControl(frame: Frame, expression: ExprId, environment: Environment): Control =
    Push({frame: frame, expression: expression, environment: environment})

  // ExprIds are looked up only in the declared finite, closed syntax-table domain.
  // A missing ExprId has no invented K error code or admission meaning here.
  pure def evaluate(s: Snapshot, expression: ExprId): Transition =
    match s.program.expressions.get(expression) {
      | Literal(value) =>
        if (validValue(value)) machineRule(s, Return(value), 102)
        else machineRule(s, Error(2), 105)
      | Variable(name) =>
        if (s.environment.keys().contains(name)) machineRule(s, Return(s.environment.get(name)), 108)
        else machineRule(s, Error(3), 111)
      | Let(expr) => machineRule(s,
          pushControl(LetFrame({name: expr.name, body: expr.body, environment: s.environment}), expr.value, s.environment), 114)
      | Binary(expr) => machineRule(s,
          pushControl(LeftFrame({op: expr.op, right: expr.right, environment: s.environment}), expr.left, s.environment), 117)
      | Unary(expr) => machineRule(s,
          pushControl(UnaryFrame(expr.op), expr.argument, s.environment), 120)
      | If(expr) => machineRule(s,
          pushControl(IfFrame({yes: expr.yes, no: expr.no, environment: s.environment}), expr.condition, s.environment), 123)
      | Match(expr) => machineRule(s,
          pushControl(MatchFrame({arms: expr.arms, environment: s.environment}), expr.value, s.environment), 126)
      | Call(expr) =>
        if (expr.arguments.length() == 0) machineRule(s, Invoke({name: expr.name, values: List()}), 129)
        else machineRule(s, pushControl(
          ArgsFrame({name: expr.name, values: List(), remaining: expr.arguments.tail(), environment: s.environment}),
          expr.arguments.head(), s.environment), 131)
    }

  // Terminal returns have already been published before this helper is called.
  pure def returnValue(s: Snapshot, value: Value): Transition = {
    val rest = s.frames.tail()
    match s.frames.head() {
      | LetFrame(frame) => machineRule({
          ...s, frames: List(ScopeFrame(frame.environment)).concat(rest),
          environment: frame.environment.put(frame.name, value),
        }, Eval(frame.body), 145)
      | LeftFrame(frame) => machineRule({
          ...s, frames: List(RightFrame({op: frame.op, left: value})).concat(rest), environment: frame.environment,
        }, Eval(frame.right), 149)
      | RightFrame(frame) => machineRule({...s, frames: rest},
          PrimitiveResult(binaryResult(frame.op, frame.left, value)), 153)
      | UnaryFrame(op) => machineRule({...s, frames: rest}, PrimitiveResult(unaryResult(op, value)), 157)
      | IfFrame(frame) => match value {
          | BoolValue(condition) => machineRule({...s, frames: rest, environment: frame.environment},
              Eval(if (condition) frame.yes else frame.no), if (condition) 161 else 165)
          // Failed condition/type checks retain the entire original frame/env.
          | _ => machineRule(s, Error(5), 169)
        }
      | MatchFrame(frame) => machineRule({...s, frames: rest, environment: frame.environment},
          Cases({value: value, arms: frame.arms, environment: frame.environment}), 173)
      | ArgsFrame(frame) => {
          val supplied = frame.values.append(value)
          if (frame.remaining.length() == 0)
            machineRule({...s, frames: rest, environment: frame.environment}, Invoke({name: frame.name, values: supplied}), 181)
          else machineRule({
            ...s, environment: frame.environment,
            frames: List(ArgsFrame({name: frame.name, values: supplied, remaining: frame.remaining.tail(), environment: frame.environment})).concat(rest),
          }, Eval(frame.remaining.head()), 177)
        }
      | ReturnFrame(frame) =>
        if (hasType(value, frame.ty)) machineRule({...s, frames: rest, environment: frame.environment}, Return(value), 185)
        else machineRule(s, Error(6), 190)
      | ScopeFrame(environment) => machineRule({...s, frames: rest, environment: environment}, Return(value), 195)
    }
  }

  // One source machine rule. Pure helper folds create no additional A events.
  pure def ordinaryStep(s: Snapshot): Transition = match s.control {
    | Eval(expression) => evaluate(s, expression)
    | Return(value) => returnValue(s, value)
    | Push(pendingPush) =>
      if (s.frames.length() < s.stackLimit) machineRule({
          ...s, frames: List(pendingPush.frame).concat(s.frames), environment: pendingPush.environment,
        }, Eval(pendingPush.expression), 135)
      else machineRule(s, Error(4), 140)
    | Invoke(call) => machineRule(s, Enter({found: findFunction(s.program.functions, call.name), values: call.values}), 200)
    | Enter(call) => match call.found {
        | MissingFunction => machineRule(s, Error(7), 202)
        | FoundFunction(function) => machineRule(s, Bound({
            bindings: bindParameters(function.parameters, call.values), ty: function.returnType, expression: function.body,
          }), 204)
      }
    | Bound(bound) => match bound.bindings {
        | BadBinding => machineRule(s, Error(8), 206)
        | Bindings(environment) => machineRule(s,
            pushControl(ReturnFrame({ty: bound.ty, environment: s.environment}), bound.expression, environment), 208)
      }
    | Cases(cases) =>
      if (cases.arms.length() == 0) machineRule(s, Error(9), 212)
      else {
        val arm = cases.arms.head()
        machineRule(s, CaseResult({
          matching: matchPattern(arm.pattern, cases.value, cases.environment), value: cases.value,
          expression: arm.body, remaining: cases.arms.tail(), environment: cases.environment,
        }), 214)
      }
    | CaseResult(result) => match result.matching {
        | NoMatch => machineRule(s, Cases({value: result.value, arms: result.remaining, environment: result.environment}), 217)
        | Matched(environment) => machineRule(s,
            pushControl(ScopeFrame(result.environment), result.expression, environment), 220)
      }
    | PrimitiveResult(result) => match result {
        | PrimitiveValue(value) => machineRule(s, Return(value), 223)
        | PrimitiveError(code) => machineRule(s, Error(code), 225)
      }
    // Empty control and errors do not select charged rules on initialized traces.
    | EmptyControl => {state: s, rule: 0, charge: 0}
    | Error(_) => {state: s, rule: 0, charge: 0}
  }

  pure def exhaust(s: Snapshot): Transition =
    {state: {...s, control: EmptyControl, outcome: Exhausted}, rule: 84, charge: 0}

  // K terminal publication is free and precedes the zero-fuel check.
  pure def advance(s: Snapshot): Transition =
    if (s.outcome != Pending) {state: s, rule: 0, charge: 0}
    else match s.control {
      | Return(value) =>
        if (s.frames.length() == 0)
          {state: {...s, control: EmptyControl, outcome: Done(value)}, rule: 80, charge: 0}
        else if (s.remaining == 0) exhaust(s) else ordinaryStep(s)
      | Error(code) =>
        {state: {...s, control: EmptyControl, outcome: Rejected(code)}, rule: 82, charge: 0}
      | _ => if (s.remaining == 0) exhaust(s) else ordinaryStep(s)
    }

  pure def initialSnapshot(p: Program, fuel: int, limit: int): Snapshot = {
    program: p, control: Eval(p.entry), environment: Map(), frames: List(),
    stackLimit: limit, remaining: fuel, initial: fuel, outcome: Pending,
  }

  var machine: Snapshot
  var lastRule: int
  var lastCharge: int

  action initProgram(p: Program, fuel: int, limit: int): bool = all {
    fuel >= 0, limit >= 0,
    machine' = initialSnapshot(p, fuel, limit), lastRule' = 74, lastCharge' = 0,
  }
  action step = {
    val transition = advance(machine)
    all {machine' = transition.state, lastRule' = transition.rule, lastCharge' = transition.charge}
  }

  // Concrete deterministic entry point; independent tests supply program tables.
  action init = initProgram({expressions: Map(0 -> Literal(UInt(7))), functions: List(), entry: 0}, 1, 8)
}
