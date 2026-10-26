// Finite quotient of 8ffed57fc draft_plans.rs::dispatch_group and the actual
// reference.rs selector. Values/ASTs are abstracted by exactly the tags and
// predicates those selectors inspect. Independent snapshots, not whole runs.
module kResidualDispatch {
  type Control = Eval | Push | Return | Invoke | Enter | Bound | Cases | CaseResult | Primitive | Error
  type Expr = Literal | VarHit | VarMiss | Let | Binary | Unary | If | Match | CallEmpty | CallArgs
  type Frame = Empty | LetF | LeftF | RightF | UnaryF | IfF | MatchF | ArgsF | ReturnF | ScopeF
  type Condition = TrueValue | FalseValue | InvalidValue
  type Group = Publish | Scalar | ScalarCheck | Branch | DataMatch | Call | FrameReturn | Generic | Stopped
  type Input = {pending: bool, fuel: int, control: Control, expr: Expr, frame: Frame,
    flag: bool, condition: Condition, limit: int}
  type Observation = {observed: bool, input: Input, rule: int, group: Group, block: Group, seen: Set[int]}
  var observation: Observation
  pure val order = List(Publish, Scalar, ScalarCheck, Branch, DataMatch, Call, FrameReturn)
  pure val groups = order.indices().map(ix => order.nth(ix))
  pure val rules = Map(
    Publish -> Set(80, 82, 84), Scalar -> Set(102, 108, 114, 117, 120, 145, 149, 153, 157, 223),
    ScalarCheck -> Set(111, 225), Branch -> Set(123, 161, 165, 169),
    DataMatch -> Set(126, 173, 212, 214, 217, 220),
    Call -> Set(129, 131, 177, 181, 200, 202, 204, 206, 208),
    FrameReturn -> Set(135, 140, 185, 190, 195), Generic -> Set(), Stopped -> Set())
  pure val inventory = groups.fold(Set(), (allRules, group) => allRules.union(rules.get(group)))
  pure val base: Input = {pending: true, fuel: 2, control: Eval, expr: Literal,
    frame: Empty, flag: true, condition: TrueValue, limit: 7}
  pure def referenceRule(s: Input): int =
    if (not(s.pending)) -1
    else if (s.control == Return and s.frame == Empty) 80
    else if (s.control == Error) 82
    else if (s.fuel == 0) 84
    else match s.control {
      | Eval => match s.expr {
        | Literal => 102 | VarHit => 108 | VarMiss => 111 | Let => 114 | Binary => 117
        | Unary => 120 | If => 123 | Match => 126 | CallEmpty => 129 | CallArgs => 131
      }
      | Push => if (s.flag) 135 else 140
      | Return => match s.frame {
        | Empty => 80 | LetF => 145 | LeftF => 149 | RightF => 153 | UnaryF => 157
        | IfF => match s.condition { | TrueValue => 161 | FalseValue => 165 | InvalidValue => 169 }
        | MatchF => 173 | ArgsF => if (s.flag) 177 else 181
        | ReturnF => if (s.flag) 185 else 190 | ScopeF => 195
      }
      | Invoke => 200 | Enter => if (s.flag) 204 else 202 | Bound => if (s.flag) 208 else 206
      | Cases => if (s.flag) 214 else 212 | CaseResult => if (s.flag) 220 else 217
      | Primitive => if (s.flag) 223 else 225 | Error => 82
    }
  pure def dispatch(s: Input): Group =
    if (not(s.pending)) Stopped
    else if ((s.control == Return and s.frame == Empty) or s.control == Error or s.fuel == 0) Publish
    else match s.control {
      | Eval => match s.expr {
        | If => Branch | Match => DataMatch | CallEmpty => Call | CallArgs => Call
        | VarMiss => ScalarCheck | _ => Scalar
      }
      | Push => FrameReturn
      | Return => match s.frame {
        | IfF => Branch | MatchF => DataMatch | ArgsF => Call
        | ReturnF => FrameReturn | ScopeF => FrameReturn | _ => Scalar
      }
      | Invoke => Call | Enter => Call | Bound => Call | Cases => DataMatch | CaseResult => DataMatch
      | Primitive => if (s.flag) Scalar else ScalarCheck | Error => Publish
    }
  pure def kept(limit: int): int = if (limit >= 7) 7 else limit - 1
  pure def assigned(group: Group, limit: int): Group =
    if (group == Stopped) Stopped
    else if (range(0, kept(limit)).indices().exists(ix => order.nth(ix) == group)) group else Generic
  pure def observe(s: Input, seen: Set[int]): Observation = {
    val selectedRule = referenceRule(s)
    val selectedGroup = dispatch(s)
    {observed: true, input: s, rule: selectedRule, group: selectedGroup,
      block: assigned(selectedGroup, s.limit),
      seen: if (selectedRule < 0) seen else seen.union(Set(selectedRule))}
  }
  // Rule 177 evaluates the next argument directly; no intervening Push step.
  pure def afterNextArgument(s: Input): Group =
    dispatch({...s, control: Eval, fuel: s.fuel - 1})
  action init = observation' = {...observe({...base, pending: false}, Set()), observed: false}
  action sample(s: Input): bool = observation' = observe(s, observation.seen)
  action step = {
    nondet pending = Set(false, true).oneOf()
    nondet fuel = 0.to(2).oneOf()
    nondet control = Set(Eval, Push, Return, Invoke, Enter, Bound, Cases, CaseResult, Primitive, Error).oneOf()
    nondet expr = Set(Literal, VarHit, VarMiss, Let, Binary, Unary, If, Match, CallEmpty, CallArgs).oneOf()
    nondet frame = Set(Empty, LetF, LeftF, RightF, UnaryF, IfF, MatchF, ArgsF, ReturnF, ScopeF).oneOf()
    nondet flag = Set(false, true).oneOf()
    nondet condition = Set(TrueValue, FalseValue, InvalidValue).oneOf()
    nondet limit = 1.to(7).oneOf()
    sample({pending: pending, fuel: fuel, control: control, expr: expr, frame: frame,
      flag: flag, condition: condition, limit: limit})
  }
  val modelInv = and {
    if (observation.group == Stopped) observation.rule == -1 else rules.get(observation.group).contains(observation.rule),
    if (observation.block == Generic) inventory.contains(observation.rule)
      else if (observation.block == Stopped) observation.rule == -1
      else rules.get(observation.block).contains(observation.rule),
    observation.seen.subseteq(inventory),
    kept(observation.input.limit) + (if (observation.input.limit < 7) 1 else 0) <= observation.input.limit,
  }
  val sampled = observation.observed
  val genericUsed = observation.observed and observation.block == Generic
  val stopped = observation.observed and observation.group == Stopped
  val nextArgumentBranches = observation.observed and observation.rule == 177
    and afterNextArgument(observation.input) == Branch
}
