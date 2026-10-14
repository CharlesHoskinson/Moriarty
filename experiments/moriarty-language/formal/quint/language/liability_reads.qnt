// Bounded Core statement evaluator, not a Rust/native correspondence proof.
// Two opaque declared duty identities; <=8 expression nodes, <=10 statements.
// DFS uses <=32 microsteps for an acyclic arena, fresh cache per statement.
module liability_reads {
  pure val U128_MAX = 340282366920938463463374607431768211455
  pure val W64 = 18446744073709551616
  pure val W127 = 170141183460469231731687303715884105728
  pure val DUTIES = Set(0, 1)
  pure val IDS = List(0, 1, 2, 3, 4, 5, 6, 7)
  pure val TICKS = List(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,
    16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31)
  type Value = {isBool: bool, b: bool, n: int}
  type Row = {present: bool, amount: int, identity: int}
  type Snapshot = {duties: int -> Row, x: Value, y: Value}
  type Op = UIntLit | BoolLit | Name | Present | Amount | Not | Add | Sub | Mul | Eq | And | Or
  type Expr = {op: Op, lhs: int, rhs: int, duty: int, name: str, value: Value}
  type Kind = Let | Next | Requires | Ensures | Emit | Accrue | Discharge
  type Stmt = {kind: Kind, root: int, duty: int, name: str, identity: int}
  type Case = {label: str, arena: List[Expr], program: List[Stmt], pre: Snapshot}
  type Phase = Idle | Running | Committed | Refused
  type Read = {duty: int, presence: bool, value: Value}
  type Eval = {cache: int -> Value, done: Set[int], stack: List[int], reads: List[Read], error: str}
  type Prefix = {pc: int, snapshot: Snapshot, reads: List[Read]}
  type Capture = {name: str, value: Value}
  type State = {phase: Phase, selected: Case, pc: int, current: Snapshot,
    committed: Snapshot, env: str -> Value, effects: int, committedEffects: int,
    events: List[Value], committedEvents: List[Value],
    history: List[Prefix], captures: List[Capture], executed: Set[Kind], error: str}
  var state: State
  pure def uint(n: int): Value = {isBool: false, b: false, n: n}
  pure def boolean(b: bool): Value = {isBool: true, b: b, n: 0}
  pure def row(p: bool, n: int): Row = {present: p, amount: n, identity: if (p) 10 else 0}
  pure def snapshot(p: bool, n: int): Snapshot = {
    duties: Map(0 -> row(p,n), 1 -> {present:false,amount:0,identity:0}), x:uint(0), y:uint(0)}
  pure val EMPTY = snapshot(false,0)
  pure def leaf(op: Op, duty: int, name: str, value: Value): Expr =
    {op:op,lhs:0,rhs:0,duty:duty,name:name,value:value}
  pure def lit(n: int): Expr = leaf(UIntLit,0,"",uint(n))
  pure def blit(b: bool): Expr = leaf(BoolLit,0,"",boolean(b))
  pure def amt(d: int): Expr = leaf(Amount,d,"",uint(0))
  pure def present(d: int): Expr = leaf(Present,d,"",uint(0))
  pure def name(n: str): Expr = leaf(Name,0,n,uint(0))
  pure def binary(op: Op, lhs: int, rhs: int): Expr =
    {op:op,lhs:lhs,rhs:rhs,duty:0,name:"",value:uint(0)}
  pure def stmt(kind: Kind, root: int, duty: int, name: str): Stmt =
    {kind:kind,root:root,duty:duty,name:name,identity:duty+10}
  pure def binaryOp(op: Op): bool = Set(Add,Sub,Mul,Eq,And,Or).contains(op)
  pure def u128(n: int): bool = 0 <= n and n <= U128_MAX
  pure def wellFormed(c: Case): bool = and {
    c.arena.length() > 0, c.arena.length() <= 8, c.program.length() <= 10,
    0.to(7).filter(i => i < c.arena.length()).forall(i => {
      val e = c.arena.nth(i)
      and {DUTIES.contains(e.duty),
        if (binaryOp(e.op)) e.lhs >= 0 and e.lhs < i and e.rhs >= 0 and e.rhs < i
        else if (e.op == Not) e.lhs >= 0 and e.lhs < i else true,
        if (e.op == UIntLit) u128(e.value.n) and not(e.value.isBool) else true}}),
    c.program.foldl(true,(ok,t) => ok and t.root >= 0 and t.root < c.arena.length() and DUTIES.contains(t.duty)
      and (t.kind != Accrue or t.identity == t.duty+10)
      and (t.kind != Next or Set("x","y").contains(t.name))
      and (t.kind != Let or not(Set("x","y").contains(t.name)))),
    c.program.foldl(0,(n,t) => n + (if (t.kind == Let) 1 else 0)) ==
      c.program.foldl(Set(),(names,t) => if (t.kind == Let) names.union(Set(t.name)) else names).size()}

  pure def validPre(pre: Snapshot): bool = DUTIES.forall(d => {
    val r = pre.duties.get(d)
    u128(r.amount) and (if (r.present) r.identity == d+10 else r.amount == 0 and r.identity == 0)})
  pure def binaryDefined(op: Op, a: Value, b: Value): bool =
    if (op == Eq) a.isBool == b.isBool
    else if (op == And or op == Or) a.isBool and b.isBool
    else not(a.isBool) and not(b.isBool) and (
      if (op == Add) u128(a.n+b.n) else if (op == Sub) u128(a.n-b.n)
      else if (op == Mul) u128(a.n*b.n) else false)
  pure def binaryValue(op: Op, a: Value, b: Value): Value =
    if (op == Add) uint(a.n+b.n) else if (op == Sub) uint(a.n-b.n)
    else if (op == Mul) uint(a.n*b.n) else if (op == Eq) boolean(a == b)
    else if (op == And) boolean(a.b and b.b) else boolean(a.b or b.b)
  pure def cacheValue(v: Eval, id: int, value: Value): Eval =
    {...v,cache:v.cache.set(id,value),done:v.done.union(Set(id)),stack:v.stack.tail()}
  pure def evalTick(v: Eval, arena: List[Expr], env: str -> Value, pre: Snapshot): Eval =
    if (v.stack.length() == 0 or v.error != "") v else {
      val id = v.stack.head()
      val e = arena.nth(id)
      if (v.done.contains(id)) {...v,stack:v.stack.tail()}
      else if ((binaryOp(e.op) or e.op == Not) and not(v.done.contains(e.lhs)))
        {...v,stack:List(e.lhs).concat(v.stack)}
      else if (binaryOp(e.op) and not(v.done.contains(e.rhs)))
        {...v,stack:List(e.rhs).concat(v.stack)}
      else if (binaryOp(e.op)) {
        val a = v.cache.get(e.lhs)
        val b = v.cache.get(e.rhs)
        if (binaryDefined(e.op,a,b)) cacheValue(v,id,binaryValue(e.op,a,b))
        else {...v,error:"expression"}
      } else if (e.op == Not) {
        val a = v.cache.get(e.lhs)
        if (a.isBool) cacheValue(v,id,boolean(not(a.b))) else {...v,error:"type"}
      } else if (e.op == Name) {
        if (env.keys().contains(e.name)) cacheValue(v,id,env.get(e.name)) else {...v,error:"name"}
      } else if (e.op == Present or e.op == Amount) {
        val r = pre.duties.get(e.duty)
        val value = if (e.op == Present) boolean(r.present) else uint(r.amount)
        val updated = cacheValue(v,id,value)
        {...updated,reads:v.reads.append({duty:e.duty,presence:e.op == Present,value:value})}
      } else cacheValue(v,id,e.value)
    }
  pure def evaluate(c: Case, t: Stmt, env: str -> Value, pre: Snapshot): Eval = {
    val fresh: Eval = {cache:0.to(7).mapBy(_ => uint(0)),done:Set(),stack:List(t.root),reads:List(),error:""}
    val evaluated = TICKS.foldl(fresh,(v,_) => evalTick(v,c.arena,env,pre))
    if (evaluated.error == "" and evaluated.stack.length() > 0) {...evaluated,error:"model-fuel"} else evaluated
  }
  pure def statementError(s: State, t: Stmt, v: Eval): str = {
    val value = v.cache.get(t.root)
    val r = s.current.duties.get(t.duty)
    if (v.error != "") v.error
    else if (t.kind == Requires or t.kind == Ensures) {
      if (value == boolean(true)) "" else "predicate"
    } else if (t.kind == Accrue or t.kind == Discharge) {
      if (value.isBool) "type"
      else if (t.kind == Accrue and r.present and r.identity != t.identity) "identity"
      else if (t.kind == Discharge and not(r.present)) "absent-discharge"
      else if (t.kind == Accrue and not(u128(r.amount+value.n))) "overflow"
      else if (t.kind == Discharge and not(u128(r.amount-value.n))) "underflow"
      else ""
    } else if (t.kind == Next and value.isBool) "type" else ""
  }
  pure def applyStatement(s: State, t: Stmt, v: Eval): State = {
    val value = v.cache.get(t.root)
    val r = s.current.duties.get(t.duty)
    val nextCurrent = if (t.kind == Accrue or t.kind == Discharge)
      {...s.current,duties:s.current.duties.set(t.duty,{present:true,
        amount:if (t.kind == Accrue) r.amount+value.n else r.amount-value.n,
        identity:if (t.kind == Accrue) t.identity else r.identity})}
      else if (t.kind == Next and t.name == "x") {...s.current,x:value}
      else if (t.kind == Next and t.name == "y") {...s.current,y:value} else s.current
    {...s,pc:s.pc+1,current:nextCurrent,
      env:if (t.kind == Let or t.kind == Next) s.env.put(t.name,value) else s.env,
      effects:s.effects + (if (Set(Next,Emit,Accrue,Discharge).contains(t.kind)) 1 else 0),
      events:if (t.kind == Emit) s.events.append(value) else s.events,
      captures:if (t.kind == Let) s.captures.append({name:t.name,value:value}) else s.captures,
      history:s.history.append({pc:s.pc,snapshot:s.current,reads:v.reads}),
      executed:s.executed.union(Set(t.kind))}
  }
  // The same ExprId 1 is evaluated fresh at multiple statement prefixes.
  pure val ordered: Case = {label:"ordered",pre:EMPTY,
    arena:List(lit(7),amt(0),lit(3),present(0),name("saved")),
    program:List(stmt(Accrue,0,0,""),stmt(Let,1,0,"saved"),stmt(Discharge,2,0,""),
      stmt(Next,1,0,"x"),stmt(Emit,4,0,""),stmt(Discharge,1,0,""),stmt(Emit,3,0,""),stmt(Emit,1,0,""))}
  pure val missing: Case = {label:"missing",pre:EMPTY,arena:List(present(0),amt(0)),
    program:List(stmt(Emit,0,0,""),stmt(Next,1,0,"x"))}
  pure val tombstone: Case = {...missing,label:"tombstone",pre:snapshot(true,0)}
  pure val selfRead: Case = {label:"self-read",pre:snapshot(true,7),arena:List(amt(0)),
    program:List(stmt(Accrue,0,0,""),stmt(Discharge,0,0,""))}
  pure val duplicate: Case = {label:"duplicate",pre:snapshot(true,7),
    arena:List(amt(0),binary(Add,0,0)),program:List(stmt(Next,1,0,"x"))}
  pure val twoDuties: Case = {label:"two-duties",pre:{...EMPTY,duties:Map(0 -> row(true,7),1 -> {present:true,amount:3,identity:11})},
    arena:List(amt(0),amt(1),binary(Add,0,1)),program:List(stmt(Next,2,0,"x"))}
  pure val overflow: Case = {label:"overflow",pre:EMPTY,arena:List(lit(7),lit(U128_MAX),lit(1),binary(Add,1,2)),
    program:List(stmt(Accrue,0,0,""),stmt(Let,3,0,"discarded"))}
  pure val eager: Case = {label:"eager",pre:EMPTY,
    arena:List(blit(false),lit(U128_MAX),lit(1),binary(Add,1,2),binary(Eq,3,2),binary(And,0,4)),
    program:List(stmt(Requires,5,0,""))}
  pure val absentDischarge: Case = {label:"absent-discharge",pre:EMPTY,arena:List(lit(0)),program:List(stmt(Discharge,0,0,""))}
  pure val excessiveDischarge: Case = {label:"underflow",pre:snapshot(true,3),arena:List(lit(7)),program:List(stmt(Discharge,0,0,""))}
  pure val expressionUnderflow: Case = {label:"expression-underflow",pre:EMPTY,arena:List(lit(0),lit(1),binary(Sub,0,1)),program:List(stmt(Let,2,0,"unused"))}
  pure val multiplyOverflow: Case = {label:"multiply-overflow",pre:EMPTY,arena:List(lit(W127),lit(3),binary(Mul,0,1)),program:List(stmt(Let,2,0,"unused"))}
  pure val eagerOr: Case = {...eager,label:"eager-or",arena:List(blit(true),lit(U128_MAX),lit(1),binary(Add,1,2),binary(Eq,3,2),binary(Or,0,4))}
  pure val wide: Case = {label:"wide",pre:snapshot(true,U128_MAX),arena:List(amt(0)),program:List(stmt(Next,0,0,"x"),stmt(Discharge,0,0,""),stmt(Next,0,0,"y"))}
  pure val guards: Case = {label:"guards",pre:snapshot(true,7),arena:List(present(0)),program:List(stmt(Requires,0,0,""),stmt(Ensures,0,0,""))}
  pure val forgedIdentity: Case = {...missing,label:"forged-identity",pre:{...EMPTY,duties:Map(0 -> {present:true,amount:7,identity:99},1 -> row(false,0))}}
  pure val mutationOverflow: Case = {label:"mutation-overflow",pre:snapshot(true,U128_MAX),arena:List(lit(1)),program:List(stmt(Accrue,0,0,""))}
  pure val unaryNot: Case = {label:"unary-not",pre:snapshot(true,7),
    arena:List(present(0),binary(Not,0,0),binary(Not,1,0)),
    program:List(stmt(Emit,1,0,""),stmt(Emit,2,0,""))}
  pure val equality: Case = {label:"successful-equality",pre:EMPTY,
    arena:List(lit(W127),lit(W127+1),binary(Eq,0,0),binary(Eq,0,1),
      blit(true),blit(false),binary(Eq,4,4),binary(Eq,4,5)),
    program:List(stmt(Emit,2,0,""),stmt(Emit,3,0,""),stmt(Emit,6,0,""),stmt(Emit,7,0,""))}
  pure val eventFailure: Case = {label:"event-before-failure",pre:EMPTY,
    arena:List(lit(7),lit(U128_MAX),lit(1),binary(Add,1,2)),
    program:List(stmt(Emit,0,0,""),stmt(Let,3,0,"discarded"))}
  pure val CASES = Set(ordered,missing,tombstone,selfRead,duplicate,twoDuties,overflow,eager,eagerOr,
    absentDischarge,excessiveDischarge,expressionUnderflow,multiplyOverflow,wide,guards,forgedIdentity,mutationOverflow,unaryNot,equality,eventFailure)
  pure def initial(c: Case, phase: Phase): State = {phase:phase,selected:c,pc:0,current:c.pre,
    committed:c.pre,env:Map("x" -> c.pre.x,"y" -> c.pre.y),effects:0,committedEffects:0,events:List(),committedEvents:List(),
    history:List(),captures:List(),executed:Set(),error:if (phase == Refused) "identity-admission" else ""}
  action init = state' = initial(missing,Idle)
  action begin(c: Case): bool = all {state.phase == Idle,wellFormed(c),validPre(c.pre),state' = initial(c,Running)}
  action refuseAdmission(c: Case): bool = all {state.phase == Idle,wellFormed(c),not(validPre(c.pre)),state' = initial(c,Refused)}
  action execute = if (state.phase == Running and state.pc < state.selected.program.length()) {
    val t = state.selected.program.nth(state.pc)
    val v = evaluate(state.selected,t,state.env,state.current)
    all {state.phase == Running,state.pc < state.selected.program.length(),statementError(state,t,v) == "",
      state' = applyStatement(state,t,v)}
  } else all {false,state'=state}
  action refuse = if (state.phase == Running and state.pc < state.selected.program.length()) {
    val t = state.selected.program.nth(state.pc)
    val v = evaluate(state.selected,t,state.env,state.current)
    val error = statementError(state,t,v)
    all {state.phase == Running,state.pc < state.selected.program.length(),error != "",
      state' = {...state,phase:Refused,error:error}}
  } else all {false,state'=state}
  action commit = all {state.phase == Running,state.pc == state.selected.program.length(),
    state' = {...state,phase:Committed,committed:state.current,committedEffects:state.effects,committedEvents:state.events}}
  // No terminal stutter: a finite action really finishes or refuses.
  action step = if (state.phase == Idle) {
    nondet c = CASES.oneOf()
    any {begin(c),refuseAdmission(c)}
  } else if (state.phase == Running and state.pc < state.selected.program.length()) any {execute,refuse}
    else if (state.phase == Running) commit else all {false,state'=state}
  val freshReads = state.history.foldl(true,(ok,h) => ok and h.reads.foldl(true,(readsOk,r) => readsOk and {
    val original = h.snapshot.duties.get(r.duty)
    r.value == (if (r.presence) boolean(original.present) else uint(original.amount))}))
  val capturedLets = state.captures.foldl(true,(ok,c) => ok and state.env.get(c.name) == c.value)
  val fixedIdentity = state.phase == Idle or state.error == "identity-admission" or validPre(state.current)
  val refusalIsAtomic = (state.phase != Committed) implies
    (state.committed == state.selected.pre and state.committedEffects == 0 and state.committedEvents == List())
  val safety = freshReads and capturedLets and fixedIdentity and refusalIsAtomic and state.error != "model-fuel"
  val beginWitness = state.phase == Running
  val letWitness = state.executed.contains(Let)
  val nextWitness = state.executed.contains(Next)
  val requiresWitness = state.executed.contains(Requires)
  val ensuresWitness = state.executed.contains(Ensures)
  val emitWitness = state.executed.contains(Emit)
  val accrueWitness = state.executed.contains(Accrue)
  val dischargeWitness = state.executed.contains(Discharge)
  val refusalWitness = state.phase == Refused and state.error != "identity-admission"
  val admissionRefusalWitness = state.error == "identity-admission"
  val commitWitness = state.phase == Committed
}
