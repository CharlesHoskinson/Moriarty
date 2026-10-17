# Independent work and ordered-observation derivation

Authority: exact original UIREQ003/004/005/006 acceptance text and pinned K-A
main.k b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41.
All counts below are handwritten from normative rule effects. No Machine,
compiler, model, grouping function or helper evaluator supplied expected data.
The public Machine implementation was read for its constructor/capture API facts,
not used as a substitute oracle. Public typed schema is a shared dependency.

Every used nonpublication A rule costs1. Final line80 costs0 and contributes one
record. Binary literal/variable operands take the fixed seven-rule sequence
117,135,left-value,149,right-value,153,223. For the four scalar examples both value
rules are102, producing 7charged events, then80: results5,5,12,true.

A one-literal-argument named call has9 overhead rules beyond the function body:
131,135,102,181,200,204,208,135 and185. With a seven-rule Sub argument it has15
rather than9. A zero-argument call has6 overhead rules beyond its body:
129,200,204,208,135,185. Calls bind fresh declared parameters; Return restores the
saved caller environment. Pure bind/find/primitive helpers contribute no A events.

Lexical fixture: outer binding4 rules; inner ignored-value Let contributes
EvalLet/Push2 + callee12 + ReturnLet1 + caller Var1 + Scope1 =17; outer Scope1.
Total22. Callee12 = zero-arg overhead6 + local let/Var/scope6. Full literal schedule
is in EXPECTED.json. At record10 the Return-frame push enters the empty callee
parameter environment while saving caller{x=7}. Inner x=2 is read at15; local Scope
restores empty environment at16; call return17 produces2 and restores{x=7}.
Ignored binding18 adds ignored=2; caller Var19 reads7; two Scope returns restore
empty outer environment. Publication22 yields Done7.

Nested fixture: double(2) has9+7=16 charged events and returns4. increment(3) also
has16 and returns4. Outer named add uses initial Args setup2, first call16,
RetArgs1771, second call16, RetArgs1811, Invoke/Enter/Bound/Push4, body7 and Return1:
2+16+1+16+1+4+7+1=48. Ordered selected events must be double Invoke/Return, then
increment Invoke/Return, then add Invoke/Return. Exact multiplicity rules out
re-evaluation. Complete global Program and all function declaration order remain
unchanged at every recorded state.

Factorial is defined with base n==0. If selection costs10: EvalIf/Push2, Eq body7,
RetIf1. Base body adds Literal1, so B0=11. A nonbase multiplication contributes
left/setup4, recursive call with Sub argument15+B(n-1), RetRight/Primitive2.
Therefore Bn=31+B(n-1), and entry factorial(3) costs9+(11+3*31)=113. Calls descend
3,2,1,0 and returns ascend(n=0,value1),(1,1),(2,2),(3,6). No factorial-specific
machine rule is added; recursion is solely the supplied global function data.

Parity functions use the same n==0 selection. Base body11; nonbase body is
10+(15+B(n-1))=25+B(n-1). even(4) therefore costs9+11+4*25=120; odd(5) costs
9+11+5*25=145. Both descend through even(0), whose value is true, and return true
through all saved callers. Ordered names and arguments are literal in the oracle
and test source. Declared fuel512, stack32 and event budget513 exceed these small
fixed positive paths; they are test inputs, not a new host policy envelope.

Complete post-startup initial snapshots are known from supplied Program/new API:
Eval(entry), empty environment/frames, stack32, fuel512/initial512, Pending. Final
snapshots retain that entire Program and stack/initialfuel, restore empty
environment/frames, clear control, have remaining512-frozen-charge and Done(result).
All records are retained. Tests check critical intermediate typed projections and
all per-event fuel/charge/outcome; this is not a full independently specified
intermediate-image oracle, a K execution result, a general proof or full EARS exit.
