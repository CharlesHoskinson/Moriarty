module nominalPublicSimulation {
  // Separate finite semantic profile, not new K/B rule tags or native ABI.
  // Closed admitted expressions; source occurrence is distinct from nominal identity.
  type Token = UIntToken(int) | BoolToken(bool)
    | ConstructorToken({typeId: int, ctorId: int, arity: int})
  type Value = List[Token]
  type Environment = int -> Value
  type Layout = {typeId: int, ctorId: int, fields: List[int]}
  // Source patterns are finite preorder tokens, not recursive Quint types/functions.
  type PatternToken = PatternWildcard | PatternBind(int)
    | PatternConstructor({typeId: int, ctorId: int, arity: int})
  type Pattern = Bind(int) | ConstructorPattern({typeId: int, ctorId: int, binders: List[int]})
    | NestedPattern(List[PatternToken])
  type PatternMatch = {matched: bool, bindings: Environment}
  type Arm = {pattern: Pattern, body: int}
  type Op = Add | Sub | Equal
  type Expression = Literal(Value) | Variable(int) | BooleanNot(int)
    | Let({name: int, value: int, body: int})
    | Binary({op: Op, left: int, right: int})
    | If({condition: int, yes: int, no: int})
    | Constructor({typeId: int, ctorId: int, occurrence: int, fields: List[int]})
    | Call({function: int, arguments: List[int]})
    | Match({value: int, arms: List[Arm]})
  type Function = {name: int, parameters: List[int], parameterTypes: List[int], resultType: int, body: int}
  type Program = {expressions: int -> Expression, functions: int -> Function, layouts: (int, int) -> Layout, entry: int}
  type Construction = {typeId: int, ctorId: int, occurrence: int, source: int, completed: List[Value], remaining: List[int], saved: Environment}
  type Frame = ConstructorFields(Construction)
    | LetFrame({name: int, body: int, saved: Environment})
    | ScopeFrame(Environment)
    | LeftFrame({op: Op, right: int, saved: Environment})
    | RightFrame({op: Op, left: Value})
    | NotFrame
    | IfFrame({yes: int, no: int, saved: Environment})
    | ArgumentsFrame({function: int, completed: List[Value], remaining: List[int], saved: Environment})
    | ReturnFrame({resultType: int, saved: Environment})
    | MatchFrame({arms: List[Arm], saved: Environment, source: int})
  type Control = Eval(int) | Return(Value) | Build(Construction)
    | Invoke({function: int, arguments: List[Value]})
    | Cases({value: Value, remaining: List[Arm], saved: Environment, source: int, index: int})
    | Stopped
  type Outcome = Pending | Done(Value) | Exhausted | OutsideDomain
  type Event = Start | LiteralEvent | VariableEvent | LetEvent | BinaryEvent | UnaryEvent | IfEvent
    | ConstructorIntroduce | ConstructorFieldReturn | ConstructorBuild
    | CallEvent | ArgumentReturn | CallEnter | CallReturn | ScopeReturn
    | MatchEvent | MatchReturn | MatchMiss | MatchSelect | PrimitiveReturn | BranchSelect
    | TerminalPublication | FuelStop | DomainStop
  type BuildObservation = {occurrence: int, typeId: int, ctorId: int, fields: List[Value], value: Value}
  type Snapshot = {
    program: Program, control: Control, environment: Environment, frames: List[Frame],
    initialFuel: int, remaining: int, consumed: int, helperWork: int,
    constructorNodes: int, constructorFieldSlots: int, outcome: Outcome,
    source: int, event: Event, builds: List[BuildObservation], selectedArms: List[int],
    enteredArguments: List[List[Value]], maxDepth: int,
  }
  var machine: Snapshot
  pure val PROFILE = "sherlock-quint-public-nominal-source-cases/1"
  pure val TERMINAL_PUBLICATION_CHARGE = 1
  pure val FROZEN_K_A_TERMINAL_PUBLICATION_CHARGE = 0
  pure val MAX_SCALAR = 255
  pure val FRAME_LIMIT = 32

  pure def uint(n: int): Value = List(UIntToken(n))
  pure def rootType(v: Value): int = if (v.length() == 0) -1 else match v.head() {
    | UIntToken(_) => 0
    | BoolToken(_) => 1
    | ConstructorToken(c) => c.typeId
  }
  pure def tokenArity(t: Token): int = match t {
    | ConstructorToken(c) => c.arity
    | _ => 0
  }
  // A value is a structural prefix token sequence, never an arena address.
  pure def extent(tokens: Value): int = {
    val scan = tokens.foldl({pending: 1, count: 0}, (acc, token) =>
      if (acc.pending <= 0) acc
      else {pending: acc.pending - 1 + tokenArity(token), count: acc.count + 1})
    scan.count
  }
  pure def tokenFields(p: Program, t: Token): List[int] = match t {
    | ConstructorToken(c) => if (p.layouts.keys().contains((c.typeId, c.ctorId)))
        p.layouts.get((c.typeId, c.ctorId)).fields else List()
    | _ => List()
  }
  pure def tokenValid(p: Program, t: Token, expected: int): bool = match t {
    | UIntToken(n) => expected == 0 and n >= 0 and n <= MAX_SCALAR
    | BoolToken(_) => expected == 1
    | ConstructorToken(c) => and {
        expected == c.typeId,
        p.layouts.keys().contains((c.typeId, c.ctorId)),
        c.arity >= 0,
        c.arity == tokenFields(p, t).length(),
      }
  }
  pure def validValue(p: Program, v: Value): bool = {
    val scan = v.foldl({expected: List(rootType(v)), ok: v.length() > 0}, (acc, token) =>
      if (acc.expected.length() == 0) {expected: List(), ok: false}
      else {expected: tokenFields(p, token).concat(acc.expected.tail()),
        ok: acc.ok and tokenValid(p, token, acc.expected.head())})
    scan.ok and scan.expected.length() == 0
  }
  pure def fieldsOf(v: Value): List[Value] = {
    val tokens = v.tail()
    val scan = range(0, tokens.length()).foldl({cursor: 0, fields: List()}, (acc, ignored) =>
      if (acc.cursor >= tokens.length()) acc
      else {
        val width = extent(tokens.slice(acc.cursor, tokens.length()))
        {cursor: acc.cursor + width, fields: acc.fields.append(tokens.slice(acc.cursor, acc.cursor + width))}
      })
    scan.fields
  }
  pure def distinct(names: List[int]): bool =
    names.foldl(Set(), (seen, name) => seen.union(Set(name))).size() == names.length()
  pure def bindNames(names: List[int], values: List[Value], saved: Environment): Environment =
    range(0, names.length()).foldl(saved, (env, index) => env.put(names.nth(index), values.nth(index)))
  // Positive domain: finite declaration-typed values and well-formed typed patterns,
  // with linear bind names per arm. Source admission/type checking is external.
  // This pure traversal has no new charged transition/helper/native-work claim.
  pure def nestedPatternMatch(nestedTokens: List[PatternToken], subject: Value, saved: Environment): PatternMatch = {
    val scan = nestedTokens.foldl({pending: List(subject), bindings: saved, matched: true}, (acc, patToken) =>
      if (not(acc.matched) or acc.pending.length() == 0) {...acc, matched: false}
      else {
        val subjectValue = acc.pending.head()
        val remainingValues = acc.pending.tail()
        match patToken {
          | PatternWildcard => {...acc, pending: remainingValues}
          | PatternBind(name) => {...acc, pending: remainingValues, bindings: acc.bindings.put(name, subjectValue)}
          | PatternConstructor(expectedCtor) => if (subjectValue.length() == 0) {...acc, matched: false}
            else match subjectValue.head() {
              | ConstructorToken(actualCtor) => if (and {
                  actualCtor.typeId == expectedCtor.typeId,
                  actualCtor.ctorId == expectedCtor.ctorId,
                  actualCtor.arity == expectedCtor.arity,
                }) {...acc, pending: fieldsOf(subjectValue).concat(remainingValues)}
                else {...acc, matched: false}
              | _ => {...acc, matched: false}
            }
        }
      })
    val isMatch = scan.matched and scan.pending.length() == 0
    {matched: isMatch, bindings: if (isMatch) scan.bindings else saved}
  }
  pure def constructorMatches(pattern: Pattern, value: Value): bool = match pattern {
    | NestedPattern(nestedTokens) => nestedPatternMatch(nestedTokens, value, Map()).matched
    | Bind(_) => true
    | ConstructorPattern(c) => if (value.length() == 0) false else match value.head() {
        | ConstructorToken(t) => and {
            c.typeId == t.typeId, c.ctorId == t.ctorId,
            c.binders.length() == t.arity, distinct(c.binders),
          }
        | _ => false
      }
  }
  pure def armEnvironment(pattern: Pattern, value: Value, saved: Environment): Environment = match pattern {
    | NestedPattern(nestedTokens) => nestedPatternMatch(nestedTokens, value, saved).bindings
    | Bind(name) => saved.put(name, value)
    | ConstructorPattern(c) => bindNames(c.binders, fieldsOf(value), saved)
  }
  pure def stopDomain(s: Snapshot): Snapshot = {...s, outcome: OutsideDomain, control: Stopped, event: DomainStop}
  pure def emit(s: Snapshot, event: Event, control: Control): Snapshot = {...s, event: event, control: control}
  pure def push(s: Snapshot, frame: Frame, expression: int, env: Environment, event: Event): Snapshot = {
    val depth = s.frames.length() + 1
    if (depth > FRAME_LIMIT) stopDomain(s)
    else {...s, control: Eval(expression), environment: env, frames: List(frame).concat(s.frames),
      event: event, maxDepth: if (depth > s.maxDepth) depth else s.maxDepth}
  }
  pure def construction(s: Snapshot, id: int, c: Construction): Snapshot =
    if (c.remaining.length() == 0) emit(s, ConstructorIntroduce, Build(c))
    else push(s, ConstructorFields({...c, remaining: c.remaining.tail()}), c.remaining.head(), c.saved, ConstructorIntroduce)
  pure def evaluate(s: Snapshot, id: int): Snapshot = {
    val marked = {...s, source: id}
    if (not(s.program.expressions.keys().contains(id))) stopDomain(marked)
    else match s.program.expressions.get(id) {
      | Literal(v) => if (validValue(s.program, v)) emit(marked, LiteralEvent, Return(v)) else stopDomain(marked)
      | Variable(name) => if (s.environment.keys().contains(name))
          emit(marked, VariableEvent, Return(s.environment.get(name))) else stopDomain(marked)
      | BooleanNot(operand) => push(marked, NotFrame, operand, s.environment, UnaryEvent)
      | Let(e) => push(marked, LetFrame({name: e.name, body: e.body, saved: s.environment}), e.value, s.environment, LetEvent)
      | Binary(e) => push(marked, LeftFrame({op: e.op, right: e.right, saved: s.environment}), e.left, s.environment, BinaryEvent)
      | If(e) => push(marked, IfFrame({yes: e.yes, no: e.no, saved: s.environment}), e.condition, s.environment, IfEvent)
      | Constructor(e) => construction(marked, id, {typeId: e.typeId, ctorId: e.ctorId,
          occurrence: e.occurrence, source: id, completed: List(), remaining: e.fields, saved: s.environment})
      | Call(e) => if (e.arguments.length() == 0) emit(marked, CallEvent, Invoke({function: e.function, arguments: List()}))
          else push(marked, ArgumentsFrame({function: e.function, completed: List(), remaining: e.arguments.tail(), saved: s.environment}),
            e.arguments.head(), s.environment, CallEvent)
      | Match(e) => push(marked, MatchFrame({arms: e.arms, saved: s.environment, source: id}), e.value, s.environment, MatchEvent)
    }
  }
  pure def scalar(v: Value): int = match v.head() { | UIntToken(n) => n | _ => -1 }
  pure def primitive(s: Snapshot, op: Op, left: Value, right: Value): Snapshot =
    if (op == Equal) emit(s, PrimitiveReturn, Return(List(BoolToken(left == right))))
    else if (rootType(left) != 0 or rootType(right) != 0) stopDomain(s)
    else {
      val n = if (op == Add) scalar(left) + scalar(right) else scalar(left) - scalar(right)
      if (n < 0 or n > MAX_SCALAR) stopDomain(s) else emit(s, PrimitiveReturn, Return(uint(n)))
    }
  // General ordinary positive Bool negation; uses the same return/frame path.
  // Other values remain outside this finite typed profile, not a Core refusal.
  pure def booleanNot(s: Snapshot, value: Value): Snapshot =
    if (value == List(BoolToken(true))) emit(s, PrimitiveReturn, Return(List(BoolToken(false))))
    else if (value == List(BoolToken(false))) emit(s, PrimitiveReturn, Return(List(BoolToken(true))))
    else stopDomain(s)
  pure def returnField(s: Snapshot, c: Construction, value: Value): Snapshot = {
    val complete = {...c, completed: c.completed.append(value)}
    if (c.remaining.length() == 0)
      {...s, environment: c.saved, control: Build(complete), event: ConstructorFieldReturn}
    else push({...s, environment: c.saved}, ConstructorFields({...complete, remaining: c.remaining.tail()}),
      c.remaining.head(), c.saved, ConstructorFieldReturn)
  }
  pure def returnValue(s: Snapshot, value: Value): Snapshot =
    if (s.frames.length() == 0) {...s, control: Stopped, outcome: Done(value), event: TerminalPublication}
    else {
      val popped = {...s, frames: s.frames.tail()}
      match s.frames.head() {
        | ConstructorFields(c) => returnField(popped, c, value)
        | LetFrame(f) => push(popped, ScopeFrame(f.saved), f.body, f.saved.put(f.name, value), ScopeReturn)
        | ScopeFrame(saved) => {...popped, environment: saved, control: Return(value), event: ScopeReturn}
        | LeftFrame(f) => push(popped, RightFrame({op: f.op, left: value}), f.right, f.saved, BinaryEvent)
        | RightFrame(f) => primitive(popped, f.op, f.left, value)
        | NotFrame => booleanNot(popped, value)
        | IfFrame(f) => if (value == List(BoolToken(true)))
            {...popped, environment: f.saved, control: Eval(f.yes), event: BranchSelect}
          else if (value == List(BoolToken(false)))
            {...popped, environment: f.saved, control: Eval(f.no), event: BranchSelect}
          else stopDomain(s)
        | ArgumentsFrame(f) => {
            val supplied = f.completed.append(value)
            if (f.remaining.length() == 0) {...popped, environment: f.saved,
              control: Invoke({function: f.function, arguments: supplied}), event: ArgumentReturn}
            else push(popped, ArgumentsFrame({...f, completed: supplied, remaining: f.remaining.tail()}),
              f.remaining.head(), f.saved, ArgumentReturn)
          }
        | ReturnFrame(f) => if (rootType(value) == f.resultType)
            {...popped, environment: f.saved, control: Return(value), event: CallReturn}
          else stopDomain(s)
        | MatchFrame(f) => {...popped, environment: f.saved, event: MatchReturn,
            control: Cases({value: value, remaining: f.arms, saved: f.saved, source: f.source, index: 0})}
      }
    }
  pure def invoke(s: Snapshot, name: int, arguments: List[Value]): Snapshot =
    if (not(s.program.functions.keys().contains(name))) stopDomain(s)
    else {
      val f = s.program.functions.get(name)
      val shape = f.parameters.length() == arguments.length() and f.parameters.length() == f.parameterTypes.length()
      if (not(shape) or not(distinct(f.parameters))) stopDomain(s)
      else if (not(range(0, arguments.length()).foldl(true, (ok, index) =>
        ok and rootType(arguments.nth(index)) == f.parameterTypes.nth(index)))) stopDomain(s)
      else push({...s, enteredArguments: s.enteredArguments.append(arguments)},
        ReturnFrame({resultType: f.resultType, saved: s.environment}), f.body,
        bindNames(f.parameters, arguments, Map()), CallEnter)
    }
  pure def build(s: Snapshot, c: Construction): Snapshot = {
    val key = (c.typeId, c.ctorId)
    if (not(s.program.layouts.keys().contains(key))) stopDomain(s)
    else {
      val fieldTypes = s.program.layouts.get(key).fields
      if (c.completed.length() != fieldTypes.length() or c.remaining.length() != 0) stopDomain(s)
      else if (not(range(0, fieldTypes.length()).foldl(true, (ok, index) =>
        ok and rootType(c.completed.nth(index)) == fieldTypes.nth(index)))) stopDomain(s)
      else {
        val value = c.completed.foldl(List(ConstructorToken({typeId: c.typeId, ctorId: c.ctorId, arity: fieldTypes.length()})),
          (tokens, fieldValue) => tokens.concat(fieldValue))
        {...s, control: Return(value), environment: c.saved, source: c.source, event: ConstructorBuild,
          constructorNodes: s.constructorNodes + 1, constructorFieldSlots: s.constructorFieldSlots + fieldTypes.length(),
          builds: s.builds.append({occurrence: c.occurrence, typeId: c.typeId, ctorId: c.ctorId, fields: c.completed, value: value})}
      }
    }
  }
  pure def cases(s: Snapshot, value: Value, arms: List[Arm], saved: Environment, source: int, index: int): Snapshot =
    if (arms.length() == 0) stopDomain(s)
    else {
      val arm = arms.head()
      if (constructorMatches(arm.pattern, value))
        push({...s, selectedArms: s.selectedArms.append(index), source: source}, ScopeFrame(saved), arm.body,
          armEnvironment(arm.pattern, value, saved), MatchSelect)
      else emit(s, MatchMiss, Cases({value: value, remaining: arms.tail(), saved: saved, source: source, index: index + 1}))
    }
  pure def ordinary(s: Snapshot): Snapshot = match s.control {
    | Eval(id) => evaluate(s, id)
    | Return(value) => returnValue(s, value)
    | Build(c) => build(s, c)
    | Invoke(c) => invoke(s, c.function, c.arguments)
    | Cases(c) => cases(s, c.value, c.remaining, c.saved, c.source, c.index)
    | Stopped => stopDomain(s)
  }
  pure def advance(s: Snapshot): Snapshot =
    if (s.outcome != Pending) s
    else if (s.remaining == 0) {...s, outcome: Exhausted, control: Stopped, event: FuelStop}
    else ordinary({...s, remaining: s.remaining - 1, consumed: s.consumed + 1})
  pure def initial(p: Program, fuel: int): Snapshot = {
    program: p, control: Eval(p.entry), environment: Map(), frames: List(),
    initialFuel: fuel, remaining: fuel, consumed: 0, helperWork: 0,
    constructorNodes: 0, constructorFieldSlots: 0, outcome: Pending,
    source: p.entry, event: Start, builds: List(), selectedArms: List(), enteredArguments: List(), maxDepth: 0,
  }
  // Root/test supplies a finite concrete initialization. No uninstantiated const.
  action initProgram(p: Program, fuel: int): bool = machine' = initial(p, fuel)
  action tick: bool = all {machine.outcome == Pending, machine' = advance(machine)}
  action terminalStutter: bool = all {machine.outcome != Pending, machine' = machine}
  action step: bool = any {tick, terminalStutter}
  val accountingInv = and {
    machine.remaining >= 0, machine.consumed >= 0,
    machine.consumed + machine.remaining == machine.initialFuel,
    machine.helperWork == 0,
    machine.frames.length() <= FRAME_LIMIT, machine.maxDepth <= FRAME_LIMIT,
    machine.constructorNodes == machine.builds.length(),
    machine.constructorFieldSlots == machine.builds.foldl(0, (n, b) => n + b.fields.length()),
  }
  val valueInv = and {
    machine.environment.keys().forall(name => validValue(machine.program, machine.environment.get(name))),
    machine.builds.foldl(true, (ok, b) => ok and validValue(machine.program, b.value)),
  }
  val allInv = accountingInv and valueInv
  val recursiveFirstFieldWitness = machine.frames.foldl(false, (seen, frame) => seen or match frame {
    | ConstructorFields(c) => c.typeId == 20 and c.completed.length() == 0 and c.remaining.length() == 1
    | _ => false
  }) and machine.enteredArguments.length() >= 2
  val recursiveLastFieldWitness = machine.frames.foldl(false, (seen, frame) => seen or match frame {
    | ConstructorFields(c) => c.typeId == 10 and c.completed == List(uint(2)) and c.remaining.length() == 0
    | _ => false
  }) and machine.enteredArguments.length() >= 2
  val orderedArmWitness = machine.selectedArms == List(1)
  val completedWitness = machine.outcome == Done(uint(18))
}
