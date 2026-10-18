from pathlib import Path
import datetime, hashlib, json, subprocess
K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
prior=R/'defi-floor-division-current-source-20261018'
out=R/'floor-division-consumer-impact-20261018'
def sha(raw): return hashlib.sha256(raw).hexdigest()
def pin(p): return {'path':str(p),'sha256':sha(p.read_bytes()),'bytes':p.stat().st_size}
def inventory(p): return {x.relative_to(p).as_posix():sha(x.read_bytes()) for x in p.rglob('*') if x.is_file()}
prior_before=inventory(prior)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=K,text=True).strip()
cache={}
def source(path):
    if path not in cache:
        raw=(K/path).read_bytes()
        committed=subprocess.check_output(['git','show',head+':'+path],cwd=K)
        cache[path]={'raw':raw,'text':raw.decode(),'pin':{'root':'K','path':path,'sha256':sha(raw),'bytes':len(raw),'matchesCapturedHead':raw==committed}}
    return cache[path]
def span(text,anchor):
    start=text.index(anchor)
    brace=text.index('{',start)
    level=1; end=brace+1
    while level:
        level += (text[end]=='{')-(text[end]=='}')
        end+=1
    body=text[start:end]
    return {'startLine':text[:start].count('\n')+1,'endLine':text[:end].count('\n')+1,'sha256':sha(body.encode()),'text':body}
E=[
('E01','crates/mori-core/src/runtime.rs','type_cached','let result = match node {','Expr::FloorDiv','Explicit UInt operand/result typing.'),
('E02','crates/mori-core/src/runtime.rs','eval_cached','let result = match &p.expressions[i] {','Expr::FloorDiv','Explicit checked_div evaluation with current cache and meter.'),
('E03','crates/mori-core/src/stage/cost.rs','expression_nodes','match node {','Expr::FloorDiv','Explicit profile refusal; no staged cost support.'),
('E04','crates/mori-core/src/sherlock/source_adapter.rs','map_expression node-kind label','let kind = match source {','CoreExpr::FloorDiv','Explicit CoreExpr floor-div label.'),
('E05','crates/mori-core/src/sherlock/source_adapter.rs','map_expression translation','let mapped = match source {','CoreExpr::FloorDiv','Explicit unsupported-projection; Sherlock enum unchanged.'),
('E06','crates/mori-backend/src/lib.rs','Builder::expression value construction','let wire = match expression {','Expression is outside this compiler profile.','Generic unsupported-expression fallback only, no division lowering.'),
('E07','crates/mori-ledger/src/relation/compiler.rs','Compiler::expr value construction','let w = match self','expression-not-lowered','Generic expression-not-lowered fallback only, no division lowering.'),
('E08','crates/mori/src/commands/inspect.rs','enforcement expression rule','let rule = match node {','expression outside the current inspection profile','Generic unsupported display fallback after every existing variant.')]
exhaustive=[]
for id,path,symbol,anchor,token,detail in E:
    s=source(path); c=prior/'candidate'/path
    current=span(s['text'],anchor); candidate=span(c.read_text(),anchor)
    assert token in candidate['text'],id
    exhaustive.append({'id':id,'path':path,'symbol':symbol,'current':{'source':s['pin'],'match':current},'candidate':{'source':pin(c),'match':candidate},'disposition':detail,'status':'new variant accounted for by source inspection; compile unrun','additionalCorrectionRequired':False})
W=[
('W01','crates/mori-core/src/runtime.rs','validate depth/reference walk','let refs = match e {','Candidate adds explicit FloorDiv children and ordinary-profile refusal.'),
('W02','crates/mori-core/src/stage/validate.rs','validate_v2 depth/reference walk','let refs = match e {','Existing wildcard. Stage source load does not construct the new node; StagedProgram has no Deserialize/public constructor. No staged support is inferred.'),
('W03','crates/mori-core/src/sherlock/bounded_scalar_source.rs','bounded source tree-size/depth walk','let (children, fanout): ([crate::ExprId; 2], usize) = match expression {','Existing wildcard. crate::validate precedes this walk and proposed default validation refuses the draft. No projection support is inferred.'),
('W04','crates/mori-backend/src/lib.rs','Builder::expression child scheduling','match expression {','Existing wildcard. Final value-construction match has the candidate unsupported fallback; native division stays unresolved.'),
('W05','crates/mori-ledger/src/relation/compiler.rs','Compiler::names','match self\n            .p','Existing wildcard. Candidate expr value construction remains unsupported; native division stays unresolved.'),
('W06','crates/mori-ledger/src/ops.rs','lower_ops_inner prefix-state references','match expr {','Existing wildcard ignores unknown shapes. This is not a FloorDiv consumer and is not qualified by the reference alternative; future projection remains separate.'),
('W07','crates/mori/src/commands/inspect.rs','reachable expression IDs','match &program.expressions[id as usize] {','Existing wildcard. Current CLI/default parser does not route the draft; any future CLI support must separately address child reachability.')]
walks=[]
for id,path,symbol,anchor,detail in W:
    s=source(path);c=prior/'candidate'/path
    walks.append({'id':id,'path':path,'symbol':symbol,'current':{'source':s['pin'],'match':span(s['text'],anchor)},'candidateMatch':span(c.read_text() if c.exists() else s['text'],anchor),'status':'nonexhaustive walk; no enum compile gap','detail':detail,'additionalCorrectionRequired':False})
selective=[]
for path,symbol in [('crates/mori-core/src/runtime.rs','validate liability-read identity checks'),('crates/mori-backend/src/liabilities.rs','registry liability-read checks')]:
    s=source(path);at=s['text'].index('if let Expr::LiabilityPresent')
    selective.append({'path':path,'symbol':symbol,'line':s['text'][:at].count('\n')+1,'source':s['pin'],'disposition':'Selective if-let over flat arena nodes; no exhaustive arm required.'})
compat=[]
for p in sorted((prior/'before').rglob('*')):
    if p.is_file():
        path=p.relative_to(prior/'before').as_posix();s=source(path)
        compat.append({'path':path,'currentSha256':s['pin']['sha256'],'priorBeforeSha256':sha(p.read_bytes()),'equal':s['raw']==p.read_bytes()})
assert len(compat)==10 and all(r['equal'] for r in compat)
for path in ['crates/mori-core/src/ir.rs','crates/mori-core/src/stage/mod.rs','crates/mori-core/src/stage/decode.rs','crates/mori-core/src/parser.rs','crates/mori-core/src/lib.rs','crates/mori-core/src/logical_cost.rs','crates/mori-core/src/replay.rs','crates/mori-core/src/stage/profile.rs','Cargo.toml','experiments/moriarty-cardano-rust/k2/Cargo.toml','experiments/moriarty-cardano-rust/r1cs/source/mori-r1cs/Cargo.toml']: source(path)
wire=[
{'id':'D01','path':'crates/mori-core/src/ir.rs','symbols':['shared Expr derive Deserialize','v1::Program.expressions: Vec<Expr>','decode_via_v1!(Statement, Action, Program, Step, Execution)'],'featureOff':'The opt-in variant is absent; this specific enum change does not alter the derive variant set. Runtime compatibility was not measured.','featureOn':'Shared Expr derive recognizes FloorDiv; v1::Program reuses it, so Program deserialization recognizes the new variant before validation and independently of the language string.','candidateValidation':'Default validate explicitly refuses the new node in its whole-arena walk. Draft validation separately checks the draft language/semantics. Decode and validation are distinct.','disposition':'Feature-on frozen wire isolation/diagnostics is UNADOPTED. No codec policy correction is made.'},
{'id':'D02','path':'crates/mori-core/src/ir.rs','symbols':['v1::Statement','v1::Step','v1::Execution'],'detail':'Step/Execution contain statement ExprIds, values, snapshots/effects and no expression arena. The new Expr kind does not itself add a Statement kind or Step/Execution field. An enclosing Program retains D01.','disposition':'No extra hand-written exhaustive decoder arm is missing; this is source inference, not a decode execution.'},
{'id':'D03','path':'crates/mori-core/src/stage/mod.rs','symbols':['StagedProgram','load','Continuation','StageEvent'],'detail':'StagedProgram is constructed only by load and has no Deserialize/public constructor. Continuation/Event carry snapshots/named scalar values, not Expr/Program arenas. No external staged Program decoder is invented.','disposition':'No compile-only codec patch necessary; staged division stays unsupported.'}]
for r in wire:r['source']=source(r['path'])['pin']
excluded=[
{'family':'Sherlock Expr/BinaryOp visitors','reason':'Different enums. Candidate changes neither; Core-to-Sherlock map explicitly reports unsupported projection.'},
{'family':'mori-ast NodeTag and beta parser/checker/formatter','reason':'Different AST/frontend; no shared Core Expr exhaustive import/spelling was identified.'},
{'family':'captured fixture basis/ir-public-dto.rs copies','reason':'Immutable captured DTO source, not the production definition; no rewrite.'},
{'family':'standalone Cardano K2/R1CS Core copy','reason':'Cargo paths select experiments/moriarty-cardano-rust/moriarty/crates/mori-core rather than workspace crates/mori-core. No candidate is installed there.'},
{'family':'ordinary test/example constructors, matches! and if-let','reason':'Partial construction/pattern sites require no new exhaustive arm. No test design, test additions or execution in this lane.'}]
assert prior_before==inventory(prior)
scope=json.loads((prior/'PRESERVED-SCOPE.json').read_text())
result={'format':'moriarty.floor-division-consumer-impact/1','createdAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'currentRoot':str(K),'capturedHead':head,'priorCandidate':str(prior),'priorPreservedUnchanged':True,'finding':'All eight identified hand-written exhaustive production shared-Core Expr matches are already accounted for. No additional accidental exhaustive-match compile gap was established by source inspection.','qualification':'Source inspection/byte comparison only; no compile/build/run/test/model/SDK/provider/Git/shared write. Not ordinary runtime, compiler, proof or security acceptance.','exhaustiveConsumers':exhaustive,'partialReferenceWalks':walks,'selectiveReaders':selective,'wireDecodeCompatibility':wire,'currentToPriorPatchBase':compat,'excludedTypeFamilies':excluded,'sourceFiles':[v['pin'] for k,v in sorted(cache.items())],'retainedScope':{'source':pin(prior/'PRESERVED-SCOPE.json'),'rows':len(scope['rows']),'DA01to24':len(scope['actions']),'heldouts':len(scope['heldouts']),'intents':len(scope['intentCases']),'regressionClasses':len(scope['regressionClasses']),'composition':len(scope['composition']),'acceptance':'unchanged/open'}}
(out/'CONSUMER-IMPACT.json').write_text(json.dumps(result,indent=2)+'\n')
(out/'PATCH-CORRECTIONS.json').write_text(json.dumps({'format':'moriarty.floor-division-compile-gap-corrections/1','patches':[],'patchCount':0,'reason':'The preserved candidate already covers the eight exhaustive matches. No further accidental compile gap was established. New lowerings, traversal support, codec decisions, default routing or cost policy would exceed a compile-only correction.','baseCompatibility':'Ten modified-file bases equal current K bytes.','buildOrSyntaxVerified':False,'previousCandidateChanged':False,'sharedSourceChanged':False},indent=2)+'\n')
print(json.dumps({'head':head,'exhaustiveSites':len(exhaustive),'partialWalks':len(walks),'selectiveReaders':len(selective),'wireGroups':len(wire),'matchingPatchBases':len(compat),'newCorrections':0,'priorUnchanged':True,'sourceFilesPinned':len(cache)},indent=2))
