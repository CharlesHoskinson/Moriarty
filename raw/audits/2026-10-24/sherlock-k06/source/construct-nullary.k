requires "dependencies/c6-logical-syntax.k"

// Additive current-Core counterpart, not the older ngFinStub evaluator.
// Positional rows are a declared projection of the complete retained raw view.
// No host validity flag, record provenance assertion or physical decoder here.
module SHERLOCK-CONSTRUCT-NULLARY
  imports SHERLOCK-C6-LOGICAL-SYNTAX
  imports DOMAINS

  syntax CNRow ::= cnMissing()
                 | cnLet(Int, Int, Int) | cnVar(Int)
                 | cnConstruct(Int, Int, C6Slice, C6Env)
                 | cnType(Int, Int) | cnCtor(Int, Int, Int)
                 | cnConstant(Bool, Int, Int, C6Slice)
                 | cnOther(C6Raw)
  syntax CNRows ::= cnNoRows() | cnRow(CNRow, CNRows)
  syntax CNView ::= cnView(CNRows, CNRows, CNRows, CNRows, Int)
  syntax CNProfile ::= cnProfile(Int, String, String)
  // Each cnZeroFrame denotes all thirteen canonical inactive frame lanes zero.
  // The actual physical codec and its binding are external adapter obligations.
  syntax CNPad ::= cnNoPad() | cnZeroFrame(CNPad) | cnBadPad(C6Raw, CNPad)
  // initial, remaining, consumed, helper work, helper visits, reference cursor.
  syntax CNClock ::= cnClock(Int, Int, Int, Int, Int, Int)
  // clock, stack limit, frame capacity, helper pending, live tasks, Running, pad.
  syntax CNRuntime ::= cnRuntime(CNClock, Int, Int, Bool, Int, Bool, CNPad)
  syntax CNState ::= cnState(CNProfile, CNView, C6Boundary, CNRuntime)
  syntax CNAnswer ::= cnApplied(String, CNState) | cnOutside(String, CNState)

  syntax CNRow ::= cnAt(CNRows, Int) [function]
  rule cnAt(cnNoRows(), _) => cnMissing()
  rule cnAt(cnRow(R, _), 0) => R
  rule cnAt(cnRow(_, Rs), I) => cnAt(Rs, I -Int 1) requires I >Int 0
  rule cnAt(cnRow(_, _), I) => cnMissing() requires I <Int 0
  syntax CNRow ::= cnExpr(CNView, Int) [function]
                 | cnTypeAt(CNView, Int) [function]
                 | cnCtorAt(CNView, Int) [function]
                 | cnConstAt(CNView, Int) [function]
  rule cnExpr(cnView(Es,_,_,_,_), I) => cnAt(Es,I)
  rule cnTypeAt(cnView(_,Ts,_,_,_), I) => cnAt(Ts,I)
  rule cnCtorAt(cnView(_,_,Cs,_,_), I) => cnAt(Cs,I)
  rule cnConstAt(cnView(_,_,_,Cs,_), I) => cnAt(Cs,I)
  syntax Int ::= cnFieldCount(CNView) [function]
  rule cnFieldCount(cnView(_,_,_,_,N)) => N

  syntax Bool ::= cnU128(Int) [function] | cnUsize(Int) [function]
  rule cnU128(I) => I >=Int 0 andBool I <Int 340282366920938463463374607431768211456
  rule cnUsize(I) => I >=Int 0 andBool I <Int 18446744073709551616
  syntax Bool ::= cnEnvOk(C6Env) [function] | cnExprExists(CNRow) [function]
  rule cnEnvOk(c6None()) => true
  rule cnEnvOk(c6Some(I)) => cnUsize(I)
  rule cnExprExists(cnLet(_,_,_)) => true
  rule cnExprExists(cnVar(_)) => true
  rule cnExprExists(cnConstruct(_,_,_,_)) => true
  rule cnExprExists(_) => false [owise]
  syntax Bool ::= cnProfileOk(CNProfile) [function]
  rule cnProfileOk(cnProfile(2,"GenericNominalExtensionReserved","Absent")) => true
  rule cnProfileOk(_) => false [owise]

  syntax Int ::= cnDepth(C6Frames) [function] | cnPadDepth(CNPad) [function]
  rule cnDepth(c6NoFrames()) => 0
  rule cnDepth(c6Frame(_,Fs)) => 1 +Int cnDepth(Fs)
  rule cnPadDepth(cnNoPad()) => 0
  rule cnPadDepth(cnZeroFrame(P)) => 1 +Int cnPadDepth(P)
  rule cnPadDepth(cnBadPad(_,P)) => 1 +Int cnPadDepth(P)
  syntax Bool ::= cnPadOk(CNPad) [function]
  rule cnPadOk(cnNoPad()) => true
  rule cnPadOk(cnZeroFrame(P)) => cnPadOk(P)
  rule cnPadOk(cnBadPad(_,_)) => false
  syntax C6Frames ::= cnSnoc(C6Frames, C6Frame) [function]
  rule cnSnoc(c6NoFrames(),F) => c6Frame(F,c6NoFrames())
  rule cnSnoc(c6Frame(F,Fs),G) => c6Frame(F,cnSnoc(Fs,G))
  syntax Bool ::= cnLetShape(CNView, C6Frame, CNRow) [function]
                 | cnFrameOk(CNView, C6Frame) [function]
                 | cnFramesOk(CNView, C6Frames) [function]
  rule cnFrameOk(V,c6LetF(B,D,E,O)) => cnLetShape(V,c6LetF(B,D,E,O),cnExpr(V,O))
  rule cnFrameOk(_,_) => false [owise]
  rule cnLetShape(V,c6LetF(B,D,E,O),cnLet(B,X,D))
    => cnUsize(B) andBool cnUsize(D) andBool cnUsize(O) andBool cnEnvOk(E)
       andBool cnExprExists(cnExpr(V,X)) andBool cnExprExists(cnExpr(V,D))
  rule cnLetShape(_,_,_) => false [owise]
  rule cnFramesOk(_,c6NoFrames()) => true
  rule cnFramesOk(V,c6Frame(F,Fs)) => cnFrameOk(V,F) andBool cnFramesOk(V,Fs)
  // Current let profile requires the active (last) LetF to await this child.
  // This deliberately does not claim Core rejects all other continuations.
  syntax Bool ::= cnLetFor(CNView, C6Frames, Int) [function]
                 | cnLetChild(CNRow, Int) [function]
  rule cnLetFor(_,c6NoFrames(),_) => false
  rule cnLetFor(V,c6Frame(c6LetF(_,_,_,O),c6NoFrames()),I) => cnLetChild(cnExpr(V,O),I)
  rule cnLetFor(V,c6Frame(_,c6Frame(F,Fs)),I) => cnLetFor(V,c6Frame(F,Fs),I)
  rule cnLetFor(_,_,_) => false [owise]
  rule cnLetChild(cnLet(_,I,_),I) => true
  rule cnLetChild(_,_) => false [owise]

  syntax Bool ::= cnClockOk(CNClock) [function]
  rule cnClockOk(cnClock(I,R,W,H,V,C))
    => cnU128(I) andBool cnU128(R) andBool cnU128(W) andBool cnU128(H)
       andBool cnU128(V) andBool cnU128(C) andBool I ==Int R +Int W
       andBool R >Int 0 andBool cnU128(W +Int 1) andBool cnU128(C +Int 1)
  syntax CNClock ::= cnCharge(CNClock) [function]
  rule cnCharge(cnClock(I,R,W,H,V,C)) => cnClock(I,R -Int 1,W +Int 1,H,V,C +Int 1)
  syntax Bool ::= cnReady(CNState) [function]
  rule cnReady(cnState(P,V,c6Boundary(_,_,O,E,Fs,c6NoResult(),_),
                       cnRuntime(K,L,N,false,0,true,Pad)))
    => cnProfileOk(P) andBool cnUsize(O) andBool cnEnvOk(E) andBool cnClockOk(K)
       andBool cnUsize(L) andBool cnUsize(N) andBool L <=Int N andBool cnFramesOk(V,Fs)
       andBool cnDepth(Fs) <=Int L andBool cnDepth(Fs) +Int cnPadDepth(Pad) ==Int N
       andBool cnPadOk(Pad)
  rule cnReady(_) => false [owise]

  // checked_ctor_arity: type>=4, positional existence, owner, type interval,
  // checked field interval. For arity zero, checked_add(start,0) is start.
  syntax Bool ::= cnCtorOk(CNView, Int, Int) [function]
                 | cnCtorRows(CNRow, CNRow, Int, Int, Int) [function]
                 | cnConstOk(CNView, Int, Int, Int) [function]
                 | cnConstRow(CNRow, Int, Int) [function]
  rule cnCtorOk(V,T,C) => cnUsize(T) andBool T >=Int 4 andBool cnUsize(C)
                         andBool cnCtorRows(cnTypeAt(V,T),cnCtorAt(V,C),T,C,cnFieldCount(V))
  rule cnCtorRows(cnType(S,N),cnCtor(T,F,0),T,C,Total)
    => cnUsize(S) andBool cnUsize(N) andBool cnUsize(F) andBool cnUsize(Total)
       andBool C >=Int S andBool C -Int S <Int N andBool F <=Int Total
  rule cnCtorRows(_,_,_,_,_) => false [owise]
  rule cnConstOk(V,T,C,I) => cnUsize(I) andBool cnConstRow(cnConstAt(V,I),T,C)
  rule cnConstRow(cnConstant(true,T,C,c6Slice(S,0)),T,C) => cnUsize(S)
  rule cnConstRow(_,_,_) => false [owise]

  syntax CNAnswer ::= cnNullary(CNState) [function]
                    | cnNullarySelected(CNState, CNRow) [function]
                    | cnStep(CNState) [function]
                    | cnDispatch(CNState, C6Control, CNRow) [function]
  rule cnNullary(S:CNState) => cnNullarySelected(S,cnSelectedExpr(S)) requires cnReady(S)
  rule cnNullary(S:CNState) => cnOutside("outside current let/admission/clock profile",S)
    requires notBool cnReady(S)
  syntax CNRow ::= cnSelectedExpr(CNState) [function]
  rule cnSelectedExpr(cnState(_,V,c6Boundary(_,c6Eval(I),_,_,_,_,_),_)) => cnExpr(V,I)
  rule cnSelectedExpr(_) => cnMissing() [owise]
  rule cnNullarySelected(
      cnState(P,V,c6Boundary(Raw,c6Eval(I),I,E,Fs,c6NoResult(),Rest),
                  cnRuntime(K,L,N,false,0,true,Pad)),
      cnConstruct(T,C,c6Slice(S,0),c6Some(Cid)))
    => cnApplied("XAdtEvalConstructNullary",
         cnState(P,V,c6Boundary(Raw,c6Return(c6Const(Cid)),I,E,Fs,c6NoResult(),Rest),
                     cnRuntime(cnCharge(K),L,N,false,0,true,Pad)))
    requires cnUsize(S) andBool cnCtorOk(V,T,C) andBool cnConstOk(V,T,C,Cid)
             andBool cnLetFor(V,Fs,I)
  rule cnNullarySelected(S,_) => cnOutside("not selected checked nullary in current let profile",S) [owise]

  // Exactly the two ordinary prefix row kinds needed by the independent AST.
  // No generic new interpreter, scheduler, proof or additional rule identity.
  rule cnStep(cnState(P,V,c6Boundary(Raw,C,O,E,Fs,H,Rest),Rt))
    => cnDispatch(cnState(P,V,c6Boundary(Raw,C,O,E,Fs,H,Rest),Rt),C,
                  cnSelectedExpr(cnState(P,V,c6Boundary(Raw,C,O,E,Fs,H,Rest),Rt)))
    requires cnReady(cnState(P,V,c6Boundary(Raw,C,O,E,Fs,H,Rest),Rt))
  rule cnStep(S:CNState) => cnOutside("outside current let/admission/clock profile",S)
    requires notBool cnReady(S)
  rule cnDispatch(
      cnState(P,V,c6Boundary(Raw,c6Eval(I),I,E,Fs,c6NoResult(),Rest),
                  cnRuntime(K,L,N,false,0,true,Pad)),c6Eval(I),cnLet(B,X,D))
    => cnApplied("KMachineEvalLet",
         cnState(P,V,c6Boundary(Raw,c6ScheduleNext(c6LetF(B,D,E,I),X,E),I,E,Fs,c6NoResult(),Rest),
                     cnRuntime(cnCharge(K),L,N,false,0,true,Pad)))
    requires cnUsize(B) andBool cnUsize(X) andBool cnUsize(D)
             andBool cnExprExists(cnExpr(V,X)) andBool cnExprExists(cnExpr(V,D))
  rule cnDispatch(
      cnState(P,V,c6Boundary(Raw,c6ScheduleNext(F,X,E),O,E,Fs,c6NoResult(),Rest),
                  cnRuntime(K,L,N,false,0,true,cnZeroFrame(Pad))),c6ScheduleNext(F,X,E),_)
    => cnApplied("KMachinePushAccept",
         cnState(P,V,c6Boundary(Raw,c6Eval(X),X,E,cnSnoc(Fs,F),c6NoResult(),Rest),
                     cnRuntime(cnCharge(K),L,N,false,0,true,Pad)))
    requires cnFrameOk(V,F) andBool cnExprExists(cnExpr(V,X)) andBool cnUsize(X)
             andBool cnDepth(Fs) <Int L andBool cnScheduledChild(V,F,X,O)
  rule cnDispatch(S:CNState,c6Eval(_),cnConstruct(_,_,_,_)) => cnNullary(S)
  rule cnDispatch(S,_,_) => cnOutside("outside three selected row kinds",S) [owise]
  syntax Bool ::= cnScheduledChild(CNView, C6Frame, Int, Int) [function]
  rule cnScheduledChild(V,c6LetF(_,_,_,O),X,O) => cnLetChild(cnExpr(V,O),X)
  rule cnScheduledChild(_,_,_,_) => false [owise]

  // Logical applied identity plus complete post state; not event provenance.
  syntax Bool ::= cnMatches(CNAnswer, String, CNState) [function]
                 | cnRelation(Int, CNState, CNState) [function]
                 | cnIsOutside(CNAnswer) [function]
                 | cnOutsideMatches(CNAnswer, CNState) [function]
  rule cnMatches(cnApplied(R,S),R,T) => S ==K T
  rule cnMatches(_,_,_) => false [owise]
  rule cnOutsideMatches(cnOutside(_,S),T) => S ==K T
  rule cnOutsideMatches(cnApplied(_,_),_) => false
  rule cnRelation(0,S,T) => S ==K T
  rule cnRelation(1,S,T) => cnMatches(cnNullary(S),"XAdtEvalConstructNullary",T)
  rule cnRelation(I,_,_) => false requires I =/=Int 0 andBool I =/=Int 1
  rule cnIsOutside(cnOutside(_,_)) => true
  rule cnIsOutside(cnApplied(_,_)) => false

  // Source slice bounds for an explicitly supplied table length only.
  // This does not decode C6Raw or establish raw/projection consistency.
  // Empty slices may have nonzero starts when they fit the table.
  syntax Bool ::= cnSliceFits(C6Slice, Int) [function]
  rule cnSliceFits(c6Slice(S,L),N)
    => cnUsize(S) andBool cnUsize(L) andBool cnUsize(N)
       andBool S <=Int N andBool L <=Int N -Int S
endmodule
