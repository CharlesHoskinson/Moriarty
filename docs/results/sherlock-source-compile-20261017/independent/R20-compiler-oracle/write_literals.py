"""Independent fixed B04 literals + checked arithmetic/JSON; no subject imports/calls."""
from pathlib import Path
from copy import deepcopy
import json,hashlib
P=Path(__file__).resolve().parent

def read(n):return json.loads((P/n).read_text())
def write(n,d):(P/n).write_text(json.dumps(d,indent=2)+'\n')
sources={
 'fixed-quote.mori':'language "moriarty/1";\nagreement FixedQuote {\n  action quote() {\n    let base = 7 * 11;\n    let total = base + 5;\n    emit "quoted"(total);\n  }\n}\n',
 'fixed-eligibility.mori':'language "moriarty/1";\nagreement FixedEligibility {\n  action inspect() {\n    let low = 3 < 9;\n    let high = 9 > 3;\n    let eligible = low and high;\n    emit "eligible"(eligible);\n  }\n}\n'}
(P/'inputs').mkdir(exist_ok=True)
for n,text in sources.items():(P/'inputs'/n).write_text(text)
envelope=read('normative/05-PUBLIC-NORMAL-ENVELOPE.json')
def strings(v):
 if type(v)is int:return str(v)
 if type(v)is dict:return {k:strings(x) for k,x in v.items()}
 if type(v)is list:return [strings(x) for x in v]
 return v
config={'format':'sherlock-host-config/1','profile':'sherlock-helper-microstep/1','grammar':'DedicatedSorts','finance':'Absent','bounds':strings(envelope['bounds_c']),'layout':{'layout_version':envelope['layout_schema_c']['layout_version'],'field_id':None,'widths':None,'scalar_layout_id':None,'capacities':strings(envelope['layout_capacities_c']),'boundary_capacities_present':False},'fuel':{'kind':'Nat','value':'1000'},'stack':{'kind':'Nat','value':'32'},'host':strings(envelope['source_host_bounds'])}
write('inputs/execution-config.json',config);write('inputs/empty-input-map.json',{})
# Exactly chosen typed normal input values; no changed guard/profile/numerical policy.
write('INPUTS.json',{'scope':'Two new parameter-free Core/1 normal inputs using one public selected B compiler route','fixtures':[{'source':'inputs/'+n,'source_sha256':hashlib.sha256((P/'inputs'/n).read_bytes()).hexdigest(),'action':action,'action_index':'0','agreement':agreement} for n,action,agreement in [('fixed-quote.mori','quote','FixedQuote'),('fixed-eligibility.mori','inspect','FixedEligibility')]],'execution_profile':'sherlock-helper-microstep/1','execution_config':'inputs/execution-config.json','runtime_inputs':'inputs/empty-input-map.json','parameter_policy':'No source action parameters; declared/supplied input vectors[]; supplied BTreeMap{}','target_descriptor':'CALLER_INPUT_NOT_SUPPLIED: Root must select/read/hash actual --target descriptor before subject execution; no native/network identity invented','compile_only':'--observe omitted; evaluation_performed=false; no run/event image generated','observe':'Separate existing owner/H1 scope; no execution performed or authorized by these data','numeric_input_lineage':'Exact old public input-only normal envelope values, transformed to required decimal-string/kind wire schema, not a new default/profile/cap','layout_bounds_mapping':'The single config.bounds object supplies both ConsumerConfig.bounds and LayoutSchemaC.bounds; no duplicate wire bounds'} )
# Fixed typed ProgramView rows, authored directly from the two source shapes.
def variant(tag,**fields):return {tag:fields}
def dec(n):return str(n)
def ref(n):return {'Data':dec(n)}
def const(n):return {'Const':dec(n)}
def sl(start=0,length=0):return {'start':dec(start),'len':dec(length)}
qnames=['base','quote','total'];qb={'base':'0','quote':'1','total':'2'}
qrows=[variant('Call',function_name='1',args=sl()),variant('Let',binder='0',value='2',body='5'),variant('Binary',op='Mul',left='3',right='4'),variant('Literal',constant='0'),variant('Literal',constant='1'),variant('Let',binder='2',value='6',body='9'),variant('Binary',op='Add',left='7',right='8'),variant('Var',binder='0'),variant('Literal',constant='2'),variant('Var',binder='2')]
qpaths=[[],[],[0],[0,0],[0,1],[1],[1,0],[1,0,0],[1,0,1],[1,1]]
# These are alpha-canonical placeholders, NEVER claimed actual compiler strings.
alpha_names=['$alpha.and.left','$alpha.and.right','$alpha.gt.left','$alpha.gt.right','eligible','high','inspect','low']
aid={n:dec(i) for i,n in enumerate(alpha_names)}
e=aid
erecords=[variant('Call',function_name=e['inspect'],args=sl()),variant('Let',binder=e['low'],value='2',body='5'),variant('Binary',op='Lt',left='3',right='4'),variant('Literal',constant='0'),variant('Literal',constant='1'),variant('Let',binder=e['high'],value='6',body='13'),variant('Let',binder=e['$alpha.gt.left'],value='7',body='8'),variant('Literal',constant='2'),variant('Let',binder=e['$alpha.gt.right'],value='9',body='10'),variant('Literal',constant='3'),variant('Binary',op='Lt',left='11',right='12'),variant('Var',binder=e['$alpha.gt.right']),variant('Var',binder=e['$alpha.gt.left']),variant('Let',binder=e['eligible'],value='14',body='22'),variant('Let',binder=e['$alpha.and.left'],value='15',body='16'),variant('Var',binder=e['low']),variant('Let',binder=e['$alpha.and.right'],value='17',body='18'),variant('Var',binder=e['high']),variant('If',cond='19',then_expr='20',else_expr='21'),variant('Var',binder=e['$alpha.and.left']),variant('Var',binder=e['$alpha.and.right']),variant('Literal',constant='4'),variant('Var',binder=e['eligible'])]
epaths=[[],[],[0],[0,0],[0,1],[1],[1,0],[1,0,0],[1,0,1],[1,0,1,0],[1,0,1,1],[1,0,1,1,0],[1,0,1,1,1],[1,1],[1,1,0],[1,1,0,0],[1,1,0,1],[1,1,0,1,0],[1,1,0,1,1],[1,1,0,1,1,0],[1,1,0,1,1,1],[1,1,0,1,1,2],[1,1,1]]
def view(rows,paths,names,function_name,sort,constants):
 slots=[];occ=[]
 for i,path in enumerate(paths):
  start=len(slots) if path else 0;slots.extend(path)
  occ.append({'owner_is_function':i!=0,'function':'0','path':sl(start,len(path))})
 return {'entry':'0','expressions':rows,'occurrences':occ,'occurrence_paths':[dec(x) for x in slots],'children':[],'functions':[{'function_name':function_name,'params':sl(),'return_type':sort,'body':'1'}],'parameters':[],'arms':[],'patterns':[],'constants':constants,'constant_fields':[],'names':names}
qview=view(qrows,qpaths,qnames,'1','UInt',[{'U128':{'marker_free':True,'n':str(n)}} for n in [7,11,5]])
eview=view(erecords,epaths,alpha_names,e['inspect'],'Bool',[{'U128':{'marker_free':True,'n':str(n)}} for n in [3,9,9,3]]+[{'Bool':{'marker_free':True,'b':False}}])
assert len(qview['occurrence_paths'])==16 and len(eview['occurrence_paths'])==76
write('EXPECTED-QUOTE-PROGRAM-VIEW.json',qview)
write('EXPECTED-ELIGIBILITY-ALPHA-PROGRAM-VIEW.json',eview)
write('EXPECTED-ALPHA-ADAPTER.json',{'status':'EXPLICIT_BIJECTIVE_COMPARISON_ADAPTER;NO_ACTUAL_FRESH_SPELLING_OR_RAW_ID_GUESSED','FixedQuote':'Identity adapter; names exactlybase,quote,total in UTF8 order','FixedEligibility':{'fresh_roles':[{'role':'gt.left','binding_expr':'6','normalized_name':'$alpha.gt.left','normalized_id':'2'},{'role':'gt.right','binding_expr':'8','normalized_name':'$alpha.gt.right','normalized_id':'3'},{'role':'and.left','binding_expr':'14','normalized_name':'$alpha.and.left','normalized_id':'0'},{'role':'and.right','binding_expr':'16','normalized_name':'$alpha.and.right','normalized_id':'1'}],'fixed_source_names':['eligible','high','inspect','low'],'rules':['Read actual binder NameIds from the four prescribed Let rows; retain each original actualId/actualName unchanged in a separate mapping record','Freshactualnames/IDs must form a bijection across the4roles and be distinct from all source identifiers; same binder/use references must remain consistent','Actualnames table has exactly4source names plus4fresh names, distinct and strictly sorted by UTF8 bytes; do not guess its numeric rawIDs or fresh text','Rename the4fresh symbols to their explicitalpha placeholders, then re-sort the normalizednames table and rewrite everyNameId consistently in expressions/functions/parameters and image EnvCell binderfields','Never renumber ExprIds, occurrences, paths, constant IDs, function indices, value references or event ordinals','Keep rawactualProgramView/image/report alongside normalized projection; do not replace rawfields with guessed zeros/names','Compare the complete normalized12fieldProgramView to EXPECTED-ELIGIBILITY-ALPHA-PROGRAM-VIEW.json']},'lineage':'Gt/And eager structural templates are author-owned existing public structural tests/docs; mathematical values/work derived separately below','limitation':'No exact rawfreshspellings, rawNameIds or full source-report node_kind/CoreExprId equality claim'})
identity={'profile_id':2,'grammar_domain':'DedicatedSorts','finance':'Absent','layout_version':config['layout']['layout_version'],'layout_digest':'Unselected','bounds_digest':'Unselected','field_id':None,'program_membership_digest':'Unselected'}
run_id={'identity':identity,'fuel_input':{'Nat':'1000'},'stack_limit_input':{'Nat':'32'}}
zero_active={'data':'0','data_fields':'0','env':'0','value_slots':'0'}
zero_alloc={'data_nodes':'0','data_fields':'0','environment_cells':'0'}
seg={'active':deepcopy(zero_active),'alloc':deepcopy(zero_alloc),'consumed_work':'0'}
genesis={'run_id':run_id,'control':{'Eval':{'expr':'0'}},'control_origin':'0','env_root':None,'stack_limit':'32','outcome':'Running','initial_fuel':'1000','remaining_work':'1000','consumed_work':'0','helper_work':'0','helper_visits':'0','reference_cursor':'0','alloc':zero_alloc,'active':zero_active,'helper':None,'result':None,'seg_base':seg,'data':[],'data_fields':[],'env':[],'frames':[],'tasks':[],'value_slots':[]}
write('EXPECTED-IDENTITY-LAYOUT-GENESIS.json',{'identity':identity,'identity_meaning':'Normative HostOnly DigestC fields Unselected, host equality structural; this is not an authentic constant digest','layout':{'layout_version':config['layout']['layout_version'],'field_id':None,'widths':None,'scalar_layout_id':None,'bounds':config['bounds'],'capacities':config['layout']['capacities'],'boundary_capacities_present':False},'genesis_active_projection':genesis,'scope':'Complete typed image registers and active slices. Raw host storage/padding tails are not fixed by normative lazy-storage permission; retained separately, not fabricated.'})
# Manual fixed traces: shorthand blocks are literal event lists, NOT an evaluator.
helpers={
 'validate':['Validate.entry','Validate.occurrence','Validate.empty','Validate.exit'],
 'arith':['Scalar.entry','Scalar.tag','Scalar.arithmetic','Scalar.exit'],
 'get1':['EnvGet.entry','EnvGet.cell','EnvGet.exit'],
 'get2':['EnvGet.entry','EnvGet.cell','EnvGet.cell','EnvGet.exit'],
 'put':['EnvPut.entry','EnvPut.insert','EnvPut.exit'],
 'find':['Find.entry','Find.row','Find.exit'],
 'bind0':['Bind.entry','Bind.empty','Bind.exit'],
 'type':['Scalar.entry','Scalar.tag','Scalar.exit'],
 'testbool':['Scalar.entry','Scalar.tag','Scalar.exit']}
prefix=['eval.call.empty','H:find','invoke.find','H:bind0','enter.bind','bound.valid','push.accept']
quote=prefix+['eval.let','push.accept','eval.binary','push.accept','H:validate','eval.literal.valid','return.left','push.accept','H:validate','eval.literal.valid','H:arith','return.right','primitive.value','H:put','return.let','push.accept','eval.let','push.accept','eval.binary','push.accept','H:get1','eval.variable.present','return.left','push.accept','H:validate','eval.literal.valid','H:arith','return.right','primitive.value','H:put','return.let','push.accept','H:get1','eval.variable.present','return.scope','return.scope','H:type','return.call.valid','PUBLISH']
eligibility=prefix+[
 'eval.let','push.accept','eval.binary','push.accept','H:validate','eval.literal.valid','return.left','push.accept','H:validate','eval.literal.valid','H:arith','return.right','primitive.value','H:put','return.let','push.accept',
 'eval.let','push.accept',
 'eval.let','push.accept','H:validate','eval.literal.valid','H:put','return.let','push.accept',
 'eval.let','push.accept','H:validate','eval.literal.valid','H:put','return.let','push.accept',
 'eval.binary','push.accept','H:get1','eval.variable.present','return.left','push.accept','H:get2','eval.variable.present','H:arith','return.right','primitive.value','return.scope','return.scope','H:put','return.let','push.accept',
 'eval.let','push.accept',
 'eval.let','push.accept','H:get2','eval.variable.present','H:put','return.let','push.accept',
 'eval.let','push.accept','H:get2','eval.variable.present','H:put','return.let','push.accept',
 'eval.if','push.accept','H:get2','eval.variable.present','H:testbool','return.if.true','push.accept','H:get1','eval.variable.present',
 'return.scope','return.scope','return.scope','H:put','return.let','push.accept','H:get1','eval.variable.present','return.scope','return.scope','return.scope','H:type','return.call.valid','PUBLISH']
rows=read('normative/03-rows-supplement.json');hrows={x['id']:x for x in rows['helper_events']};lrows={x['id']:x for x in rows['legacy_completion_instances']}
rule_names={'eval.call.empty':'KMachineEvalCallEmpty','invoke.find':'KMachineInvokeFind','enter.bind':'KMachineEnterBind','bound.valid':'KMachineBoundValid','push.accept':'KMachinePushAccept','eval.let':'KMachineEvalLet','eval.binary':'KMachineEvalBinary','eval.literal.valid':'KMachineEvalLiteralValid','return.left':'KMachineReturnLeft','return.right':'KMachineReturnRight','primitive.value':'KMachinePrimitiveValue','return.let':'KMachineReturnLet','eval.variable.present':'KMachineEvalVariablePresent','return.scope':'KMachineReturnScope','return.call.valid':'KMachineReturnCallValid','eval.if':'KMachineEvalIf','return.if.true':'KMachineReturnIfTrue'}
def schedule(plan):
 events=[];totals={'consumed_work':0,'helper_work':0,'helper_visits':0,'reference_cursor':0,'data_nodes':0,'environment_cells':0};prefixes=[dict(totals)]
 for token in plan:
  if token=='PUBLISH':items=[('k.admin.publish.value',None,None)]
  elif token.startswith('H:'):
   key=token[2:];items=[('b.helper.'+x,key,i) for i,x in enumerate(helpers[key])]
  else:items=[('k.machine.'+token,None,None)]
  for catalog,key,i in items:
   helper=key is not None;publish=catalog=='k.admin.publish.value';kind='Publish' if publish else hrows[catalog]['eventKind'] if helper else 'LegacyComplete'
   visits=int(hrows[catalog]['helperVisits']) if helper else 0
   data=1 if key=='arith' and catalog.endswith('Scalar.exit') else 0
   env=1 if catalog=='b.helper.EnvPut.insert' else 0
   if helper:
    a,b=catalog[len('b.helper.'):].split('.');rule='BHelper'+a+b.title()
   elif publish:rule='KAdminPublishValue'
   else:rule=rule_names[catalog[len('k.machine.'):]]
   work=0 if publish else 1
   event={'ordinal':str(len(events)),'catalog_rule':catalog,'rule_id':rule,'event_kind':kind,'disposition':'Published' if publish else 'Applied','work_delta':work,'helper_work_delta':1 if helper else 0,'helper_visits_delta':visits,'reference_cursor_delta':1 if not helper and not publish else 0,'alloc_deltas':{'data_nodes':str(data),'data_fields':'0','environment_cells':str(env)},'value_slot_writes':0,'outcome_tag':'Halted' if publish else 'Running','boundary':'Terminal' if publish else 'None'}
   events.append(event)
   for key_,delta in [('consumed_work',work),('helper_work',1 if helper else 0),('helper_visits',visits),('reference_cursor',1 if not helper and not publish else 0),('data_nodes',data),('environment_cells',env)]:totals[key_]+=delta
   prefixes.append(dict(totals))
 return {'manual_plan':plan,'literal_helper_blocks':helpers,'events_projection':events,'prefix_counters':[strings(x) for x in prefixes],'totals':{**totals,'events':len(events)}}
qtrace=schedule(quote);etrace=schedule(eligibility)
assert qtrace['totals']=={'consumed_work':74,'helper_work':41,'helper_visits':13,'reference_cursor':33,'data_nodes':2,'environment_cells':2,'events':75},qtrace['totals']
assert etrace['totals']=={'consumed_work':151,'helper_work':82,'helper_visits':29,'reference_cursor':69,'data_nodes':2,'environment_cells':7,'events':152},etrace['totals']
for label,t in [('QUOTE',qtrace),('ELIGIBILITY',etrace)]:
 t['scope']='Exact fixed normal event-rule/work/allocation projection under pinned revised rows; not complete StepEventC peripheral fields or per-prefix full image equality'
 t['not_expected_from_actual_execution']=True
 write('EXPECTED-'+label+'-B-EVENT-WORK-PROJECTION.json',t)
def envcell(binder,value,previous):return {'binder':binder,'value':value,'previous':previous}
qenv=[envcell('0',ref(0),None),envcell('2',ref(1),'0')]
eenv=[envcell(e['low'],ref(0),None),envcell(e['$alpha.gt.left'],const(2),'0'),envcell(e['$alpha.gt.right'],const(3),'1'),envcell(e['high'],ref(1),'0'),envcell(e['$alpha.and.left'],ref(0),'3'),envcell(e['$alpha.and.right'],ref(1),'4'),envcell(e['eligible'],ref(1),'3')]
for label,trace,envrows,data in [('QUOTE',qtrace,qenv,[{'U128':{'n':'77'}},{'U128':{'n':'82'}}]),('ELIGIBILITY',etrace,eenv,[{'Bool':{'b':True}},{'Bool':{'b':True}}])]:
 totals=trace['totals'];image=deepcopy(genesis);image.update({'control':{'Return':{'value':ref(1)}},'control_origin':'0','outcome':{'Halted':{'value':ref(1)}},'remaining_work':str(1000-totals['consumed_work']),'consumed_work':str(totals['consumed_work']),'helper_work':str(totals['helper_work']),'helper_visits':str(totals['helper_visits']),'reference_cursor':str(totals['reference_cursor']),'alloc':{'data_nodes':'2','data_fields':'0','environment_cells':str(len(envrows))},'active':{'data':'2','data_fields':'0','env':str(len(envrows)),'value_slots':'0'},'data':data,'env':envrows})
 summary={'outcome':{'Halted':{'value':ref(1)}},'events':str(totals['events']),'consumed_work':str(totals['consumed_work']),'helper_work':str(totals['helper_work']),'helper_visits':str(totals['helper_visits']),'reference_cursor':str(totals['reference_cursor'])}
 write('EXPECTED-'+label+'-NORMAL-RUN.json',{'mathematical_value':{'UInt':'82'} if label=='QUOTE' else {'Bool':True},'summary':summary,'events_forwarded':str(totals['events']),'final_image_active_projection':image,'scope':'Uncut completed normal run under pinned revisedBprofile and exact supplied operational inputs; all23typed image registers/active-slice fields. Eligibility NameIds alpha-normalized. Does not assert raw storage tails or full per-prefix image/event field comparison.','not_financial_effect':'Final emit value projected only; quoted/eligible Core effect is not executed here','capacity_counts':{'data_nodes':2,'data_fields':0,'environment_cells':len(envrows),'helper_task_peak':1,'logical_frame_peak':4 if label=='QUOTE' else 7,'value_slots':0}})
# Source report fields never reinterpreted as decimal owner record wire.
for label,action,emit,sort,paths,lets,bodycount in [('QUOTE','quote','quoted','UInt',qpaths[1:],['base','total'],9),('ELIGIBILITY','inspect','eligible','Bool',epaths[1:],['low','high','eligible'],22)]:
 write('EXPECTED-'+label+'-SOURCE-REPORT-PROJECTION.json',{'action_index':0,'action':action,'inputs':[],'lets':[{'statement_index':i,'name':name,'expression':{'NAMED_GAP':'CORE-PARSER-EXPR-ID-MAPPING'}} for i,name in enumerate(lets)],'emit':{'statement_index':len(lets),'name':emit,'expression':{'NAMED_GAP':'CORE-PARSER-EXPR-ID-MAPPING'},'role':'value-projection; ordered emit effect remains in Core'},'occurrences_body_path_order':paths,'occurrence_count':bodycount,'return_type':sort,'generated_function_name':action,'symbol_domain':{'NAMED_GAP':'SOURCE-REPORT-SYMBOL-DOMAIN-SPELLING'},'occurrence_other_fields':{'action_index':0,'statement_index':'Bound to original source statement, preserving eager/synthetic origin','expression':'CORE-PARSER-EXPR-ID-MAPPING','source_expression':'CoreExpr structural origin known; exact referenced Core IDs require bijective raw source-id mapping','node_kind':'SOURCE-REPORT-NODE-KIND-SPELLING: preserve actual; synthetic nodes require documented lowering- classification, not guessed text'},'scope':'Selected unchanged ScalarActionReport fields/provenance path projection only, not an invented full serde report'})
write('EXPECTED-COMPILE-ONLY.json',{'format':'sherlock-host-artifact/1','qualification':'host-only-scalar-value-projection; no effects/constraints/native/target qualification','execution_profile':'sherlock-helper-microstep/1','input_policy':{'kind':'parameter-free','declared':[],'supplied':[]},'admission':{'status':'admitted','representation':'HostOnly','identity_ref':'EXPECTED-IDENTITY-LAYOUT-GENESIS.json#/identity','program_views':['EXPECTED-QUOTE-PROGRAM-VIEW.json','EXPECTED-ELIGIBILITY-ALPHA-PROGRAM-VIEW.json'],'layout_ref':'EXPECTED-IDENTITY-LAYOUT-GENESIS.json#/layout'},'plans':{'witness':{'status':'not-implemented','artifact':None,'reason':'no exported WitnessPlan in this batch'},'constraint':{'status':'not-implemented','artifact':None,'reason':'no Sherlock constraint emission in this batch'},'export':{'status':'unsupported','artifact':None,'reason':'HostOnly source route has no native exporter'}},'observation':{'status':'not-requested','evaluation_performed':False},'evidence':{'source_preparation':'Contract requires source-preparation evidence; exact new wire spelling not supplied','host_observation':'Not requested; exact new evidence-string spelling not supplied','native':'unrun','target':'unrun','acceptance':'none'},'scope':'Compile-only expected predicates after existing guards/admission; this packet performs none. No legacyartifact.json/.zkir or SDK/native work. Source/config SHA256 metadata is not an authentic native identity.'})
write('EXPECTED-COVERAGE-AND-LIMITS.json',{'primary_comparison':'Compile-only: exact FixedQuote ProgramView, complete bijective-alpha FixedEligibility ProgramView, full caller config/layout/typedHostOnlyidentity and genesisactiveprojection; selected source-report fields; no evaluation','secondary_conditional_comparison':'Manual normal B rule/work/alloc prefix projection and complete final typed active-image/summary projections, not full rawStepEvent or full-prefix-image equality','known_counts':{'quote':{'program_expressions':10,'body_occurrences':9,'paths':16,'constants':3,'names':3,'Bwork':74,'helpers':41,'visits':13,'reference_cursor':33,'events':75},'eligibility':{'program_expressions':23,'body_occurrences':22,'paths':76,'constants':5,'names':8,'Bwork':151,'helpers':82,'visits':29,'reference_cursor':69,'events':152}},'explicit_limits':[{'id':'FRESH-TEXT-RAW-NAME-ID','fields':'Eligibility raw names/NameIds','rule':'Use explicit bijectivealphaadapter; preserve rawdata, no guessed spelling/ID'}, {'id':'CORE-PARSER-EXPR-ID-MAPPING','fields':'preparation.lets[].expression/emit.expression/occurrences.expression/source_expression numeric references','rule':'Source structural provenance paths known; raw Core numbering not normatively fixed here; retain actual and require bijective source mapping'}, {'id':'SOURCE-REPORT-NODE-KIND-SPELLING','fields':'preparation.occurrences[].node_kind, symbol_domain','rule':'No exactnewcase spelling inferred from author-owned generic structural examples'}, {'id':'FULL-STEP-EVENT-PERIPHERAL-FIELDS','fields':'StepEventC step_id.run_id/site/helper_kind/logical_depth/task/helperphase/helpercursor/resulttag/originreference/sourceexpr/sourcelocator/exhausted_kind/outcome_payload','rule':'Eventrule/work/allocprojection is frozen; no fullStepEvent equality or complete-prefix coverage claimed'}, {'id':'FULL-PREFIX-IMAGES','fields':'Every per-event complete ImageObservation state','rule':'Not derived here; no finalvalue/schedule-to-fullimageclosure inference'}, {'id':'HOST-STORAGE-TAILS','fields':'ImageObservation data/data_fields/env/frames/tasks/value_slots raw tails above active lengths','rule':'Norm permits lazyhoststorage; exact activeprefix frozen, actual rawvectors/tails retained independently, no fabricatedtailrows'}, {'id':'RESOURCE-REPORT-UNMEASURED-TEXT','fields':'ExistingResourceReport unmeasured reasons/metrics and host-dependent current_slices','rule':'Keep existing serializer/actualfields, no new measurement or reason/text invention'}, {'id':'TARGET-DESCRIPTOR','fields':'Caller --target exactbytes/digest','rule':'Not supplied by contracttask; Rootmustselectexistingdescriptor, no target/nativecompatibilityclaim'}, {'id':'CONFIG-SOURCE-ARTIFACT-METADATA-WIRE','fields':'New source/config wrapper-specific metadata key spellings/types beyond frozencontract','rule':'Independent semantic values known; no R19wireimplementationreador outputagreementclaim'}],'identity':'DigestC Unselected is an explicit normativeHostOnly value; never replace it with a guessed selected/authentic digest','acceptance':'No generalcompiler/financial/native/target/interpretergate closure; no guard/securitypolicy or hostile-test design'})
assert 7*11+5==82 and (3<9) and (9>3)
assert len(qview)==len(eview)==12 and len(genesis)==23
assert all(config['host'][k]!='0' for k in config['host'])
write('LITERAL-CHECKS.json',{'status':'PASS_FIXED_LITERAL_ARITHMETIC_TABLE_PATH_COUNT_AND_SCHEDULE_SUMS','quote_math':82,'eligibility_math':True,'quote_table_rows':10,'eligibility_table_rows':23,'quote_path_slots':16,'eligibility_path_slots':76,'run_totals':{'quote':qtrace['totals'],'eligibility':etrace['totals']},'program_view_fields_each':12,'image_observation_fields_excluding_sealed':23,'subject_or_compiler_or_interpreter_or_model_execution':'NONE; staticliteralexpansion/countarithmeticonly'})
print(json.dumps({'status':'PASS_LITERAL_CHECKS','quote_totals':qtrace['totals'],'eligibility_totals':etrace['totals'],'table_fields':12,'image_fields':23}))
