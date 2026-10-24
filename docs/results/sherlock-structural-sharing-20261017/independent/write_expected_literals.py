"""Transcribe fixed K-derived literals and selected typed states; no evaluator/model."""
from pathlib import Path
from copy import deepcopy
import hashlib,json,re
P=Path(__file__).resolve().parent
FUEL=512;STACK=32
RULES=[131,135,102,181,200,204,208,135,126,135,108,173,214,217,214,220,135,
117,135,131,135,108,181,200,204,208,135,126,135,108,173,214,220,135,
108,195,185,149,131,135,108,181,200,204,208,135,126,135,108,173,214,
220,135,108,195,185,153,223,195,185,80]
assert len(RULES)==61 and len(RULES)-1==60 and FUEL-60==452 and 4+4==8
# Typed-symbolic JSON is an audit mirror, NOT a claimed serde or wire ABI.
# Named ExprRef/FunctionRef/InputProgram links bind to literal definitions below.
def er(name):return {'ExprRef':name}
def uint(n):return {'UInt':str(n)}
leaf={'Tree':{'Leaf':'4'}};root={'Tree':{'Branch':[{'Leaf':'4'},{'Leaf':'4'}]}}
empty={};root_env={'11':root};branch_env={'11':root,'13':leaf,'14':leaf};leaf_env={'11':leaf};leaf_bind={'11':leaf,'12':uint(4)}
def variant(tag,*items):return {tag:list(items)}
def ret_frame(environment):return variant('Return','UInt',environment)
def scope(environment):return variant('Scope',environment)
def pending(right):return [variant('Right','Add',uint(4)) if right else variant('Left','Add',er('right-call'),branch_env),scope(root_env),ret_frame(empty)]
def callee(right):return [ret_frame(branch_env)]+pending(right)
def leaf_scopes(right):return [scope(leaf_env)]+callee(right)
def evaluate(name):return variant('Eval',er(name))
def returned(n):return variant('Return',uint(n))
def snapshot(charged,control,environment,frames,outcome='Pending'):
 return {'program':{'InputProgram':'exact input Arc for this physical variant','structural_ref':'#/definitions/program'},'control':control,'environment':environment,'frames':frames,'stack_limit':STACK,'remaining_fuel':str(FUEL-charged),'initial_fuel':str(FUEL),'outcome':outcome}
function={'name':'10','parameters':[{'name':'11','ty':'Tree'}],'return_type':'UInt','body':er('match-body')}
arm_leaf={'pattern':variant('Leaf','12'),'body':er('leaf-body')};arm_branch={'pattern':variant('Branch','13','14'),'body':er('branch-body')}
definitions={'function':function,'arms':[arm_leaf,arm_branch],'expressions':{
 'entry':variant('Call','10',[er('root-literal')]),'root-literal':variant('Literal',root),
 'match-body':variant('Match',er('parameter-t'),[arm_leaf,arm_branch]),'parameter-t':variant('Var','11'),
 'leaf-body':variant('Var','12'),'branch-body':variant('Binary','Add',er('left-call'),er('right-call')),
 'left-call':variant('Call','10',[er('left-argument')]),'left-argument':variant('Var','13'),
 'right-call':variant('Call','10',[er('right-argument')]),'right-argument':variant('Var','14')},
 'program':{'functions':[function],'entry':er('entry')}}
checkpoints=[]
def cp(event,after,control,environment,frames):
 checkpoints.append({'event_ordinal':event,'phase':'after' if after else 'before','complete_snapshot':snapshot(event+int(after),control,environment,frames)})
cp(4,False,variant('Invoke','10',[root]),empty,[])
cp(4,True,variant('Enter',{'FunctionRef':'function'},[root]),empty,[])
cp(5,True,variant('Bound',root_env,'UInt',er('match-body')),empty,[])
cp(6,True,variant('Push',ret_frame(empty),evaluate('match-body'),root_env),empty,[])
cp(7,True,evaluate('match-body'),root_env,[ret_frame(empty)])
cp(19,False,evaluate('left-call'),branch_env,pending(False))
for right,invoke,bound,stage,entered,matched,body,scope_return,return_event in [(False,23,24,25,26,31,34,35,36),(True,42,43,44,45,50,53,54,55)]:
 cp(invoke,False,variant('Invoke','10',[leaf]),branch_env,pending(right))
 cp(bound,True,variant('Bound',leaf_env,'UInt',er('match-body')),branch_env,pending(right))
 cp(stage,True,variant('Push',ret_frame(branch_env),evaluate('match-body'),leaf_env),branch_env,pending(right))
 cp(entered,True,evaluate('match-body'),leaf_env,callee(right))
 cp(matched,True,variant('CaseResult',leaf_bind,leaf,er('leaf-body'),[arm_branch],leaf_env),leaf_env,callee(right))
 cp(body,False,evaluate('leaf-body'),leaf_bind,leaf_scopes(right))
 cp(body,True,returned(4),leaf_bind,leaf_scopes(right))
 cp(scope_return,True,returned(4),leaf_env,callee(right))
 cp(return_event,False,returned(4),leaf_env,callee(right))
 cp(return_event,True,returned(4),branch_env,pending(right))
cp(37,True,evaluate('right-call'),branch_env,pending(True))
outer=[scope(root_env),ret_frame(empty)]
cp(56,True,variant('PrimitiveResult',{'Ok':uint(8)}),branch_env,outer)
cp(57,True,returned(8),branch_env,outer)
cp(58,True,returned(8),root_env,[ret_frame(empty)])
cp(59,False,returned(8),root_env,[ret_frame(empty)])
cp(59,True,returned(8),empty,[])
# Static prefix depth arithmetic is a selected counter oracle, not a state stepper.
pop_rules={153,173,181,185,195};depth=0;prefix_depths=[0]
for rule in RULES:
 depth+=(1 if rule==135 else -1 if rule in pop_rules else 0)
 assert depth>=0;prefix_depths.append(depth)
assert depth==0 and max(prefix_depths)==5
for item in checkpoints:
 n=item['event_ordinal']+(item['phase']=='after')
 assert len(item['complete_snapshot']['frames'])==prefix_depths[n]
initial=snapshot(0,evaluate('entry'),empty,[])
prepublish=snapshot(60,returned(8),empty,[])
final=snapshot(60,None,empty,[],{'Done':uint(8)})
expected={'schema':'independent-A-typed-symbolic-expected-data/1','profile':'sherlock-k-a/1','k_source_sha256':'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41','fuel':'512','stack_limit':32,'max_events':513,'physical_variants':[{'name':'shared','root':'Branch(sharedArcLeaf4,sharedArcLeaf4)','child_Arc_ptr_eq':True,'physical_tree_nodes':2,'semantic_tree_occurrences':3},{'name':'distinct','root':'Branch(ArcLeaf4a,ArcLeaf4b)','child_Arc_ptr_eq':False,'physical_tree_nodes':3,'semantic_tree_occurrences':3}],'definitions':definitions,'initial_complete_snapshot':initial,'prepublication_complete_snapshot':prepublish,'final_complete_snapshot':final,'charged_steps':'60','events':61,'remaining_fuel':'452','rule_sequence':RULES,'charges':[1]*60+[0],'prefix_remaining_fuel':[str(FUEL-min(i,60)) for i in range(62)],'prefix_depth':prefix_depths,'selected_complete_checkpoints':checkpoints,'semantic_visit_discriminators':{'invoke_ordinals':[4,23,42],'bind_ordinals':[5,24,43],'successful_return_ordinals':[36,55,59],'leaf_case_result_ordinals':[31,50],'leaf_body_eval_ordinals':[34,53],'leaf_body_values':[uint(4),uint(4)],'leaf_body_charge_each':1,'left_pending_frame':'Left(Add,right-call,branch-env)','right_pending_frame':'Right(Add,UInt4)','sum':uint(8)},'qualifications':['Typed-symbolic JSON is not a new wire ABI; executable Rust typed Snapshot literals are authoritative','All endpoint/checkpoint fields are present; Program/ExprRef/FunctionRef symbols bind only to immutable literal input definitions','Unlisted prefix control/environment/frames are NOT independently derived; complete pairwise actual trace equality is supplemental','All rule/charge/fuel/outcome/global-program/stack fields and selected prefix depth have independent fixed expectations','A pure helper rewrites and host allocation costs remain unmetered; this is no B/helper-work/native result','Two separate charged Leaf evaluations/call sequences reject pointer-cache elision of declared visits; no claim forbids an invisible optimization preserving every declared observation','Source only, subject execution pending designated-owner classification']}
(P/'EXPECTED.json').write_text(json.dumps(expected,indent=2)+'\n')
s=(P/'proposal/crates/mori-core/tests/sherlock_original_009_structural_sharing.rs').read_text()
match=re.search(r'const RULES:.*?= &\[(.*?)\];',s,re.S)
assert [int(x) for x in re.findall(r'\d+',match.group(1))]==RULES
depth_match=re.search(r'const PREFIX_DEPTH:.*?= &\[(.*?)\];',s,re.S)
assert [int(x) for x in re.findall(r'\d+',depth_match.group(1))]==prefix_depths
assert s.count('#[test]')==1
report={'status':'PASS_FIXED_LITERAL_JSON_RULE_VECTOR_ARITHMETIC_AND_DEPTH_CHECKS','charged_steps':60,'events':61,'remaining_fuel':452,'maximum_depth':5,'selected_full_checkpoint_count':len(checkpoints),'external_test_count':1,'machine_or_K_or_compiler_or_model_executed':False,'method':'Inspected stdlib fixed literal transcription, source-vector equality, integer sums and frame-count checks only. No AST evaluator or model/subject import.'}
(P/'LITERAL-CHECKS.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report))
