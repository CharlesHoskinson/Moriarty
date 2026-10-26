// U033 c41f2da11 evaluator contract: blocks INCLUDING generic fallback fit
// the positive size limit. This is a requirements model, pending Rust audit.
// Reserve a generic block before specializing. Multiple rules map to that one
// block but retain their individual reference charges (terminal rules are free).
module residualBlockBudget {
  type Phase = Unconfigured | Building | Ready | Refused
  type Runtime = {fuel: int, depth: int, value: int, outcome: int}
  type Compiler = {phase: Phase, limit: int, todo: Set[int], specialized: Set[int],
    generic: Set[int], block: int -> int, charge: int -> int, before: Runtime, after: Runtime}
  var compiler: Compiler
  pure val rules = 0.to(5)
  pure val charges = Map(0 -> 1, 1 -> 1, 2 -> 1, 3 -> 1, 4 -> 0, 5 -> 0)
  pure val runtime: Runtime = {fuel: 0, depth: 3, value: 6, outcome: 0}
  pure def start(limit: int): Compiler = {
    phase: if (limit > 0) Building else Refused, limit: limit,
    todo: if (limit > 0) rules else Set(), specialized: Set(), generic: Set(),
    block: rules.mapBy(_ => -1), charge: rules.mapBy(_ => -1), before: runtime, after: runtime}
  pure def process(s: Compiler, key: int): Compiler = {
    val specialize = s.specialized.size() < s.limit - 1
    {...s, todo: s.todo.exclude(Set(key)),
      specialized: if (specialize) s.specialized.union(Set(key)) else s.specialized,
      generic: if (specialize) s.generic else s.generic.union(Set(key)),
      block: s.block.set(key, if (specialize) s.specialized.size() + 1 else 0),
      charge: s.charge.set(key, charges.get(key))}
  }
  pure def blockCount(s: Compiler): int = s.specialized.size() + (if (s.generic.size() > 0) 1 else 0)
  action init = compiler' = {...start(0), phase: Unconfigured}
  action configure(limit: int): bool = all {compiler.phase == Unconfigured, compiler' = start(limit)}
  action visit(key: int): bool = all {
    compiler.phase == Building, compiler.todo.contains(key), compiler' = process(compiler, key)
  }
  action finish = all {compiler.phase == Building, compiler.todo == Set(), compiler' = {...compiler, phase: Ready}}
  action chooseConfiguration = {
    nondet limit = 0.to(6).oneOf()
    configure(limit)
  }
  action chooseRule = {
    nondet key = compiler.todo.oneOf()
    visit(key)
  }
  action step = any {chooseConfiguration, chooseRule, finish}
  val configured = compiler.phase != Unconfigured
  val compiled = compiler.phase == Ready
  val genericUsed = compiler.generic.size() > 0
  val invalidRefused = compiler.phase == Refused
  val modelInv = {
    val seen = compiler.specialized.union(compiler.generic)
    and {
      compiler.after == compiler.before,
      compiler.specialized.intersect(compiler.generic) == Set(),
      seen.intersect(compiler.todo) == Set(),
      seen.forall(key => compiler.charge.get(key) == charges.get(key)),
      compiler.generic.forall(key => compiler.block.get(key) == 0),
      compiler.specialized.forall(key => compiler.block.get(key) > 0),
      compiler.specialized.forall(key => compiler.specialized.forall(other =>
        key == other or compiler.block.get(key) != compiler.block.get(other))),
      if (compiler.limit > 0) blockCount(compiler) <= compiler.limit else blockCount(compiler) == 0,
      if (compiled) seen == rules and compiler.limit > 0 else true,
      if (invalidRefused) compiler.limit <= 0 and seen == Set() else true,
    }
  }
}
