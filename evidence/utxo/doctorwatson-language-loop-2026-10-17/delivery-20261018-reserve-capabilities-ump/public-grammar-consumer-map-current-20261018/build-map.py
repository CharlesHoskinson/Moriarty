"""Source-only inventory construction; does not import/execute any product code."""
import csv, hashlib, json, pathlib, re, subprocess
O=pathlib.Path(__file__).parent
R=O.parent
REPO='/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018'
B='e4d7a2b5709caa18a94f5256d03a0302364b8235'
D='5acfebe651af47e8edd6741bba298d7336a03b0e'
inv=json.loads((O/'SOURCE-INVENTORY.json').read_text())
known={(e['commit'],e['path']) for e in inv['entries']}
paths=subprocess.check_output(['git','-C',REPO,'ls-tree','-r','--name-only',B],text=True).splitlines()
extra=[p for p in paths if p.startswith(('crates/mori-lexer/src/','crates/mori-ast/src/')) or (p.startswith(('crates/fixtures/core2-','crates/fixtures/financial-report-contract/','crates/mori-source6/tests/fixtures/','crates/mori/tests/fixtures/source6-expansion/','experiments/moriarty-language/regressions/fixtures/source6-transfer/')) and p.endswith(('.mori','.json','.source6','.md','.sha256'))) or p in ['docs/audits/2026-10-14/AUDIT-PLAN.md','docs/CORE-STAGED-TESTS.md','experiments/moriarty-language/spec/successor/lexical.md','experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts']]
for p in extra:
 if (B,p) in known:continue
 data=subprocess.check_output(['git','-C',REPO,'show',B+':'+p]);dest=O/'source-base'/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
 blob=subprocess.check_output(['git','-C',REPO,'rev-parse',B+':'+p],text=True).strip()
 inv['entries'].append(dict(path=p,commit=B,git_blob=blob,sha256=hashlib.sha256(data).hexdigest(),bytes=len(data),draft_blob_same=blob==subprocess.check_output(['git','-C',REPO,'rev-parse',D+':'+p],text=True).strip()))
(O/'SOURCE-INVENTORY.json').write_text(json.dumps(inv,indent=2)+'\n')
def read(p,draft=False):return (O/('source-draft' if draft else 'source-base')/p).read_text()
def ref(p,needle,draft=False):
 s=read(p,draft);at=s.find(needle)
 if at<0:raise ValueError((p,needle))
 return ('source-draft/' if draft else 'source-base/')+p+':'+str(s[:at].count('\n')+1)
def ref_after(p,needle,after):
 s=read(p);at=s.find(needle,s.index(after))
 if at<0:raise ValueError((p,needle,after))
 return 'source-base/'+p+':'+str(s[:at].count('\n')+1)
def write(n,x): (O/n).write_text(json.dumps(x,indent=2)+'\n')
C='crates/mori-core/src/';K='crates/mori-checker/src/';P='crates/mori-parser/src/';L='crates/mori-backend/src/';CLI='crates/mori/src/commands/';S='crates/mori-source6/src/';T='experiments/moriarty-language/src/successor/';N='packages/moriarty-beta/src/'
rows=[]
def row(id,profile,rule,parse,check,lower,cli,node,tests=[],observations=[],limits=''):
 rows.append(dict(id=id,profile=profile,rule=rule,parser=parse,checker=check,lowering=lower,public_cli=cli,real_node_consumer=node,test_source=tests,observed_receipt_ids=observations,limits=limits or 'No claim beyond the named source branch and linked finite observations.'))
corecli=[ref(CLI+'check.rs','pub fn run'),ref(CLI+'compile.rs','pub fn run'),ref(CLI+'simulate.rs','pub fn run'),ref(CLI+'test.rs','pub fn run')]
corelower=[ref(L+'lib.rs','fn lower_action_segment'),ref(L+'stage/lower.rs','pub fn')]
corenode='Not admitted by experimental Source/6 parser or public beta Node frontend; no Core/1 or Core/2 Node compiler route identified in these entrypoints.'
coretype=ref(C+'runtime.rs','fn type_cached')
for name,needle,check in [('header-agreement','parser.expect("language")?', 'pub fn validate'),('state','if parser.eat("state")','pub fn validate'),('asset','parser.eat("asset")','pub fn validate'),('action-signature','parser.eat("action")','pub fn validate'),('leading-body-input','parser.eat("input")','pub fn validate'),('UInt-type','"UInt" =>','pub fn validate'),('Bool-type','"Bool" =>','pub fn validate'),('uint-literal','let raw = self.peek().text.replace','fn type_cached'),('bool-literal','if self.eat("true")','fn type_cached'),('parenthesized-expression','self.eat("(")','fn type_cached'),('name-expression','Expr::Name { name }','fn type_cached'),('not-expression','self.eat("not")','fn type_cached'),('liability-present','"liability_present" |','fn type_cached'),('liability-amount','"liability_present" |','fn type_cached')]:
 row('C-'+name,'Core/1 and Core/2',name,ref(C+'parser.rs',needle),[ref(C+'runtime.rs',check),ref(C+'stage/validate.rs','pub(crate) fn validate_v2')],corelower,corecli,corenode,limits='Core admitted and lowered subject to profile, type, finite bounds, target/domain and source-wide closed liability-registry checks. Asset scale is 0..38. Backend liability reads require a registry identity; Core scenario evaluator can have broader external duties. Parenthesized syntax uses no extra arena node.')
for op,s in [('Or','("or",'),('And','("and",'),('Eq','("=", Some("="))'),('Ne','("not", Some("="))'),('Le','("<", Some("="))'),('Ge','(">", Some("="))'),('Lt','("<", _)'),('Gt','(">", _)'),('Add','("+", _)'),('Sub','("-", _)'),('Mul','("*", _)')]:
 row('C-op-'+op,'Core/1 and Core/2','binary '+op,ref(C+'parser.rs',s),coretype,ref(L+'lib.rs','BinaryOp::'+op+' =>') if op not in ['Add','Sub','Mul'] else ref(L+'lib.rs','BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul'),corecli,corenode,limits=('Actual source Ne spelling is `not =`; UNI-018 M01 explicitly does not canonize that undocumented spelling. No != support inferred.' if op=='Ne' else 'UInt uses checked u128 arithmetic; Bool operators evaluate both operands in reference execution. Source-wide compiler limits still apply.'))
stmtparse={'Let':'"let"=>','Next':'"next"=>','Requires':'"requires"=>','Ensures':'"ensures"=>','Emit':'"emit"=>','Transfer':'"transfer"|"fee"=>','Fee':'"transfer"|"fee"=>','Liability':'"liability"=>','Discharge':'"discharge"=>'}
for name,s in stmtparse.items():
 row('C-stmt-'+name,'Core/1 and Core/2',name,ref(C+'parser.rs',s),[ref(C+'runtime.rs','Statement::'+name),ref(C+'stage/validate.rs','Statement::'+name)],ref_after(L+'lib.rs','Statement::'+name,'let mut effect = EffectDescriptor'),corecli,corenode,tests=[ref('crates/mori-backend/tests/financial_observation_relation.rs','fn invoice_relation') if name in ['Transfer','Fee','Liability','Discharge'] else ref('crates/mori/tests/core2_escrow_complete.rs','fn escrow_public_load')],observations=['FINANCIAL-RELATION'] if name in ['Transfer','Fee','Liability','Discharge'] else [],limits='Source test occurrence is not every-rule conformance. Requires/Ensures execute at their written position; Next updates environment immediately. Registry-backed compiler admits at most16 source-wide duty identities and refuses unresolved discharge-only external identities. Financial-relation receipt is the exact two-program test only, not all Core constructs.')
for id,rule,p,c,l in [('await','await label(binders) when Bool guard',ref(C+'parser.rs','"await" if'),ref(C+'stage/validate.rs','Statement::Await'),ref(L+'stage/lower.rs','pub fn')),('event-bindings','UInt/Bool event binders and prefix captures',ref(C+'parser.rs','binders.push'),ref(C+'stage/validate.rs','Await binder cannot shadow'),ref(C+'stage/admit.rs','pub fn resume')),('false-wait','false guard retains Waiting; true resumes',ref(C+'parser.rs','Statement::Await { label'),ref(C+'stage/validate.rs','Await guard must'),ref(C+'stage/admit.rs','pub fn resume'))]:
 row('C2-'+id,'Core/2',rule,p,c,l,[ref(CLI+'check.rs','mori_core::stage::load'),ref(CLI+'simulate.rs','fn run_resume'),ref(CLI+'compile.rs','mori_backend::stage::compile_staged')],corenode,tests=[ref('crates/mori/tests/core2_escrow_complete.rs','fn escrow_cli'),ref('crates/mori/tests/core2_multi_condition_complete.rs','fn multi_condition_cli'),ref('crates/mori/tests/core2_partial_delivery_complete.rs','fn partial_delivery_cli')],observations=['C2-ESCROW','C2-MULTI','C2-PARTIAL'],limits='Core/1 refuses Await; Core/2 bounds16 awaits/action,16 binders/site,256 captures,64KiB event,4MiB continuation. Unsigned staged artifact is separate /4; signed Core/2 and Core/1 ledger entrypoints refuse. No general persistent/recovery/native result follows.')
betacli=ref(CLI+'check.rs','mori_parser::parse')
betaemit=ref(S+'transfer.rs','fn extract')
betanode=[ref(N+'frontend.ts','class Parser'),ref(N+'bridge.ts','export function expand'),ref(N+'cli.ts','async function main')]
# Every beta parser primary and declaration node; inventory further enumerates schemas below.
for name,needle,checker in [('file','pub fn run','pub fn check'),('declaration','fn declaration','pub fn check'),('action-uses','fn action','pub fn check'),('type-annotation','fn ty','pub fn check'),('string','TokenKind::String =>','pub(crate) fn eval'),('number-scalar','fn number','pub(crate) fn eval'),('quantity-decimal','fn number','fn quantity'),('bool','TokenKind::KwTrue |','pub(crate) fn eval'),('None-SuccessOnly','TokenKind::KwNone |','pub(crate) fn eval'),('parentheses','TokenKind::LParen =>','pub(crate) fn eval'),('array','fn array','pub(crate) fn eval'),('record','TokenKind::LBrace =>','pub(crate) fn eval'),('reference','fn name_or_call','pub(crate) fn eval'),('named-call','fn name_or_call','fn call'),('named-fields','fn fields','fn fields'),('add','TokenKind::Plus =>','fn arithmetic'),('subtract','TokenKind::Minus =>','fn arithmetic'),('multiply','TokenKind::Star =>','fn arithmetic')]:
 cf='checker.rs' if checker=='pub fn check' else 'eval.rs'
 row('B-'+name,'Rust beta/1',name,ref(P+'parser.rs',needle),ref(K+cf,checker),betaemit,betacli,betanode,limits='Beta parse/check admits this value in its typed context; only a selected LocalS0 transfer reaches current Rust Source6 emitter. Values are evaluated constants, not runtime Core AST expressions. Node beta frontend is a distinct implementation; matching surface is not established correspondence.')
# Declarations are extracted from complete production rule table, preserving each field role.
ds=read(K+'declarations.rs'); blocks=list(re.finditer(r'kind: TokenKind::Kw(\w+),\s*fields: &(\[.*?\]),',ds,re.S))
decls=[]
for m in blocks:
 fields=[dict(name=f.group(2),required=f.group(1)=='required',role=f.group(3)) for f in re.finditer(r'(required|optional)\("([^"]+)", (?:R|Role)::(\w+)\)',m.group(2))]
 decls.append(dict(kind=m.group(1),fields=fields,source=ref(K+'declarations.rs',m.group(0))))
 row('B-decl-'+m.group(1),'Rust beta/1','declaration '+m.group(1),ref(P+'parser.rs','fn declaration'),ref(K+'declarations.rs',m.group(0)),betaemit,betacli,betanode,limits='Exact required/optional roles preserved in BETA-SCHEMAS.json; extra relation checks also apply. Source6 emitter projects selected transfer fields only. Other declarations do not imply corresponding executable financial constructors.')
for name in ['const','intent']:
 row('B-decl-'+name,'Rust beta/1',name,ref(P+'parser.rs','fn declaration'),ref(K+'checker.rs','TokenKind::KwIntent =>') if name=='intent' else ref(K+'checker.rs','keyword != TokenKind::KwConst'),betaemit,betacli,betanode,limits='Const resolves checked values; Intent validates the operation, caps and profile. Only LocalS0 transfer is selected by Rust emitter. Full source retained.')
cs=read(K+'calls.rs');calls=[]
for m in re.finditer(r'name: "([^"]+)",\s*args: &(\[.*?\]),',cs,re.S):
 name=m.group(1);args=[dict(name=a,role=r) for a,r in re.findall(r'\("([^"]+)", (?:R|Role)::(\w+)\)',m.group(2))];calls.append(dict(name=name,args=args,source=ref(K+'calls.rs',m.group(0))))
 if '.' in name:status='SpecifiedOnly authoring call; Rust emitter refuses; Node beta bridge refuses as non-LocalS0; no Source6 S0 production for this financial family.'
 elif name=='repay':status='LocalS0 checker-admitted; Rust extract explicitly refuses non-transfer; separate Node beta bridge and TS Source6 RepayAccrualFirst support it. Rust-to-Node positive missing at the Rust emission boundary.'
 else:status='Constant/metadata helper or LocalS0 transfer; may feed selected transfer values. No general backend Artifact is emitted.'
 obs=['S6-BASE'] if name in ['transfer','rounds'] else ['S6-QMIN-RUST','S6-QMIN-NODE'] if name=='min' else []
 row('B-call-'+name,'Rust beta/1',name,ref(P+'parser.rs','fn name_or_call'),[ref(K+'calls.rs',m.group(0)),ref(K+'eval.rs','fn call'),ref(K+'intents.rs','Support::SpecifiedOnly')],betaemit,betacli,betanode,tests=[ref('crates/mori/tests/source6_expand.rs','fn source_action_selection')] if name=='repay' else [],observations=obs,limits=status+' Same-kind Qty min/max and both Scalar×Qty orders are admitted; Qty×Qty, fractional excess precision and underflow/overflow refuse.')
# Enumerate every annotation family, with exact inventory links.
for name in ['Domain','Account','Asset','Obligation','Pool','Instrument','Observation','Policy','Grant','Stage','Episode','Party','ShareClass','Scalar','UInt128','String','Bool','Qty']:
 row('B-type-'+name,'Rust beta/1','annotation '+name,ref(P+'parser.rs','fn ty'),ref(K+'types.rs','pub(crate) fn check_annotation'),betaemit,betacli,betanode,limits='Only checker-permitted arguments and nominal identity/domain relationships apply; parser generic syntax alone does not admit arbitrary type constructors.')
write('BETA-SCHEMAS.json',dict(declarations=decls,calls=calls,note='Direct production-table extraction. Remaining intent, type and relational checks are preserved as full source; schemas alone are not admission.'))
# All closed Source6 EBNF productions, plus constructor refinements hidden by optional EBNF.
gp='experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf';fs=T+'financial-agreement-source-v6-frontend.ts';prep=T+'mil4-s0-source-v6.ts'
needles={'identifier':'private id()', 'integerToken':'private uint(', 'stringToken':'private string()', 'transfer':'private action()', 'repay':'private action()', 'effectVector':'private effects(', 'transferEffects':'private effects(', 'repayEffects':'private effects(', 'debit':'const debit =', 'credit':'const credit =', 'setObligation':"this.effectTag('set_obligation')",'useAllowance':"this.effectTag('use_allowance')",'useReplay':"this.effectTag('use_replay')",'advanceHead':"this.effectTag('advance_head')",'intent':"this.take('intent')",'authenticated':"this.take('authenticated')",'obligation':"if (signedAction.kind === 'Repay')",'submitted':"this.take('submit')",'domain':"this.take('domain')",'settlement':"this.take('settlement')",'selected':"this.take('selected')"}
for m in re.finditer(r'^([a-zA-Z][a-zA-Z0-9]*)\s*=',read(gp),re.M):
 name=m.group(1);repay=name in ['repay','repayEffects','setObligation','obligation']
 row('S6-'+name,'Source/6 S0',name,[ref(gp,m.group(0)),ref(fs,needles.get(name,'parse(): Source6Ast'))],ref(fs,'const expectedActionId'),[ref(fs,'export function lowerSource6'),ref(prep,'export function prepareSource6S0Unqualified')],('Rust --emit-source6-transfer cannot produce repayment/obligation; public Node beta expand can.' if repay else 'Rust --emit-source6-transfer emits transfer projection; Node beta expand returns JSON wrapper. Neither is `mori compile`.'),[ref(fs,'export function parseAndLowerSource6'),ref(prep,'export function prepareSource6S0Unqualified')],tests=[ref('experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs',"test('R-30") if repay else ref('experiments/moriarty-language/regressions/rust-source6-transfer.mjs','for (const expected')],observations=[] if repay else ['S6-BASE','S6-QMIN-NODE'],limits='Repay TS tests are source-present only in this packet; no linked Rust-to-Node positive receipt.' if repay else 'Linked finite transfer receipts exercise this production only in their exact specimens. Every array/order/value remains constrained by actual constructor shape; does not establish all alternatives or lexemes.')
for name,n,limit,obs in [
 ('transfer-cells3',"const balanceCount",'Transfer requires exactly3 ordered distinct owner/recipient/fee-recipient balance cells, including zero fee.', ['S6-BASE','S6-QMIN-NODE']),
 ('repay-cells2-duty',"if (signedAction.kind === 'Repay')",'Repay requires exactly2 ordered balance cells and exactly1 matching outstanding obligation; Rust transfer emitter cannot produce it.', []),
 ('fee-positive',"action.kind === 'Transfer' && this.here.text === 'credit'",'Positive fee includes third Credit; reference financial proposal checks effects. Existing Invoice/Workshop have positive fee.', ['S6-BASE']),
 ('fee-zero',"action.kind === 'Transfer' && this.here.text === 'credit'",'Zero fee omits fee Credit and retains third opening cell; both capped min actions already observed.', ['S6-QMIN-NODE']),
 ('repay-outstanding',"const status = this.here.text === 'settled'",'Partial repayment keeps Outstanding; TS source-only ordinary tests R-30/R-near-bound.', []),
 ('repay-settled',"const status = this.here.text === 'settled'",'Full repayment permits Settled result; TS source-only full-repay test.', []),
 ('replay-tuple',"JSON.stringify([ast.domain",'Source replay string becomes Core JSON tuple domain/signer/nonce; bare Source6 nonce is not Core key.', ['S6-BASE','S6-QMIN-NODE']),
 ('bounds',"this.uint(18n)",'Scale0..18; nominal action/caps/duties <=2^127-1; cells/counters/work use u128 and bounded sums. No arbitrary Expr grammar in Source6.', ['S6-BASE','S6-QMIN-NODE'])]:
 row('S6-refinement-'+name,'Source/6 S0',name,ref(fs,n),ref(fs,'const expectedActionId'),[ref(fs,'export function lowerSource6'),ref(prep,'export function prepareSource6S0Unqualified')],'Same explicit transfer CLI or separate Node beta bridge boundary as above.',ref(prep,'export function prepareSource6S0Unqualified'),observations=obs,limits=limit)
write('GRAMMAR-CONSUMER-MATRIX.json',dict(format='public-grammar-consumer-map/1',base=B,draft=D,predicate='For each enumerated admitted syntax/constructor branch, identify exact parser, checker, lowerer, public entrypoint, real Node consumer and named finite observed evidence or explicit gap.',rows=rows,interpretation='Rows are source inventory units, not requirements, denominator or acceptance. observed_receipt_ids link finite enclosing test scopes; a production occurrence does not qualify all its values or alternatives.'))
with (O/'GRAMMAR-CONSUMER-MATRIX.csv').open('w') as f:
 fields=list(rows[0]);w=csv.DictWriter(f,fields);w.writeheader();w.writerows({k:json.dumps(v) if isinstance(v,(list,dict)) else v for k,v in r.items()} for r in rows)
# Import immutable previous receipts and actual logs only, without rerun.
receiptpaths={
 'S6-BASE':'source6-public-consumer-check-cycle/RESULT.json',
 'S6-QMIN-RUST':'source6-capped-remittance-check-cycle/RESULT.json',
 'S6-QMIN-NODE':'source6-capped-remittance-node-check-cycle/RESULT.json',
 'C2-ESCROW':'core2-complete-report-cycle/attempt-1/RESULT.json',
 'C2-PARTIAL':'core2-partial-delivery-check-cycle/attempt-1/RESULT.json',
 'C2-MULTI':'core2-multi-condition-check-cycle/RESULT.json',
 'FINANCIAL-RELATION':'financial-relation-check-cycle/attempt-1/RESULT.json'}
evidence=[]
for id,p in receiptpaths.items():
 src=R/p;data=src.read_bytes();dest=O/'receipts'/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
 e=dict(id=id,original=str(src),copy=str(dest.relative_to(O)),sha256=hashlib.sha256(data).hexdigest(),receipt=json.loads(data),rerun=False)
 e['production_blob_comparison']='Recorded source commit can differ from current base; compare pinned file objects, not historical whole-tree acceptance.'
 e['logs']=[]
 for log in ['STDOUT.log','STDERR.log','START.json']:
  q=src.parent/log
  if q.exists():
   z=q.read_bytes(); target=dest.parent/log;target.write_bytes(z);e['logs'].append(dict(path=str(target.relative_to(O)),sha256=hashlib.sha256(z).hexdigest()))
 evidence.append(e)
write('OBSERVED-EVIDENCE.json',evidence)
# Historical source/test blobs against current exact tree: equality, never rerun.
comparisons=[]
files_by_id={
'S6-BASE':['crates/mori-source6/src/transfer.rs','crates/mori-source6/tests/transfer.rs','crates/mori/tests/source6_expand.rs',CLI+'expand.rs',CLI+'expand/source6.rs','experiments/moriarty-language/regressions/rust-source6-transfer.mjs',fs,prep],
'S6-QMIN-RUST':['crates/mori/tests/source6_expand.rs',S+'transfer.rs'],
'S6-QMIN-NODE':['experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs',fs,prep,S+'transfer.rs'],
'C2-ESCROW':['crates/mori/tests/core2_escrow_complete.rs',C+'parser.rs',C+'runtime.rs',C+'stage/validate.rs'],
'C2-PARTIAL':['crates/mori/tests/core2_partial_delivery_complete.rs',C+'parser.rs',C+'runtime.rs',C+'stage/validate.rs'],
'C2-MULTI':['crates/mori/tests/core2_multi_condition_complete.rs',C+'parser.rs',C+'runtime.rs',C+'stage/validate.rs'],
'FINANCIAL-RELATION':['crates/mori-backend/tests/financial_observation_relation.rs',L+'lib.rs',C+'parser.rs']}
for e in evidence:
 r=e['receipt'];commit=r.get('sourceCommit',r.get('sourceCommitAfter')); target=D if e['id'].startswith('S6-QMIN') else B
 for p in files_by_id[e['id']]:
  a=subprocess.check_output(['git','-C',REPO,'rev-parse',commit+':'+p],text=True).strip();b=subprocess.check_output(['git','-C',REPO,'rev-parse',target+':'+p],text=True).strip();comparisons.append(dict(evidence=e['id'],path=p,observed_commit=commit,observed_blob=a,current_commit=target,current_blob=b,identical=a==b))
write('HISTORICAL-BLOB-COMPARISON.json',comparisons)
# Test inventory is lexical, not test execution or positive/hostile classification.
tests=[]
for namespace in ['source-base','source-draft']:
 for p in sorted((O/namespace).rglob('*')):
  if p.is_file() and ('/tests/' in str(p) or '/regressions/' in str(p)) and p.suffix in ['.rs','.mjs']:
   s=p.read_text();names=[]
   if p.suffix=='.rs':
    names=[dict(name=m.group(1),line=s[:m.start()].count('\n')+1) for m in re.finditer(r'#\[test\]\s*(?:#\[[^\]]*\]\s*)*(?:async\s+)?fn\s+(\w+)',s)]
   else:names=[dict(name=m.group(1),line=s[:m.start()].count('\n')+1) for m in re.finditer(r'\btest\(([\s\S]{0,180}?),\s*(?:async\s*)?\(',s)]
   if names:tests.append(dict(path=str(p.relative_to(O)),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),source_present_only=True,names=names))
write('TEST-SOURCE-INVENTORY.json',tests)
print(json.dumps(dict(rows=len(rows),declaration_schemas=len(decls),call_schemas=len(calls),source_files=len(inv['entries']),historical_blob_comparisons=len(comparisons),same=sum(c['identical'] for c in comparisons))))
