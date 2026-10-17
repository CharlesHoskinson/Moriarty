"""Check fixed handwritten normal certificates; no AST execution or state stepper."""
from pathlib import Path
import json
P=Path(__file__).resolve().parent

def read(n):return json.loads((P/n).read_text())
def write(n,v):(P/n).write_text(json.dumps(v,indent=2)+'\n')
rows=read('normative/03-rows-supplement.json')
legacy={r['id']:r for r in rows['legacy_completion_instances']}
# Literal source-node correspondence, independently transcribed from the prepared
# source shapes. These are certificate inputs, not generated traversal results.
anchors={
 'QUOTE':{'eval.call.empty':[0],'eval.let':[1,5],'eval.binary':[2,6],
          'eval.literal.valid':[3,4,8],'eval.variable.present':[7,9],
          'return.left':[2,6],'return.right':[2,6],'primitive.value':[2,6],
          'return.let':[1,5],'return.scope':[5,1]},
 'ELIGIBILITY':{'eval.call.empty':[0],'eval.let':[1,5,6,8,13,14,16],
          'eval.binary':[2,10],'eval.if':[18],
          'eval.literal.valid':[3,4,7,9],
          'eval.variable.present':[11,12,15,17,19,20,22],
          'return.left':[2,10],'return.right':[2,10],'primitive.value':[2,10],
          'return.let':[1,6,8,5,14,16,13],
          'return.if.true':[18],'return.scope':[8,6,16,14,13,5,1]}}
# Only these fixed normal rows stage a ScheduleNext in these two specimens.
stagers={'bound.valid':'ReturnF','eval.let':'LetF','eval.binary':'LeftF','eval.if':'IfF'}
for token in stagers:
 assert any(e.get('target')=='reg.control' and e.get('value','').startswith('ScheduleNext{') for e in legacy['k.machine.'+token]['effects'])
for token in ['return.left','return.let','return.if.true']:
 r=legacy['k.machine.'+token]
 assert any(e.get('target')=='reg.control' and e.get('value','').startswith('Eval{') for e in r['effects'])
 assert not any(e.get('value','').startswith('ScheduleNext{') for e in r['effects'])
assert any(e['op']=='frameRewriteTop' for e in legacy['k.machine.return.left']['effects'])
assert any(e['op']=='frameRewriteTop' for e in legacy['k.machine.return.let']['effects'])
assert any(e['op']=='framePop' for e in legacy['k.machine.return.if.true']['effects'])
certs={}
for label,node_lists in anchors.items():
 view=read('EXPECTED-'+('QUOTE-PROGRAM-VIEW.json' if label=='QUOTE' else 'ELIGIBILITY-ALPHA-PROGRAM-VIEW.json'))
 trace=read('EXPECTED-'+label+'-B-EVENT-WORK-PROJECTION.json');plan=trace['manual_plan'];events=trace['events_projection']
 annotations=[];seen={k:0 for k in node_lists};event_start=0
 for i,token in enumerate(plan):
  expr=None
  if token in node_lists:
   expr=node_lists[token][seen[token]];seen[token]+=1
  length=len(trace['literal_helper_blocks'][token[2:]]) if token.startswith('H:') else 1
  annotations.append({'plan_index':i,'token':token,'source_expr':expr,'event_ordinal':event_start,'event_count':length});event_start+=length
 assert all(seen[k]==len(v) for k,v in node_lists.items())
 assert event_start==len(events)
 # Push must immediately follow a row whose explicit effect stages ScheduleNext.
 pushes=[]
 for a in annotations:
  if a['token']!='push.accept':continue
  prev=annotations[a['plan_index']-1];assert prev['token'] in stagers
  pushes.append({'producer_plan_index':prev['plan_index'],'push_plan_index':a['plan_index'],'frame_kind':stagers[prev['token']],'source_expr':0 if prev['token']=='bound.valid' else prev['source_expr']})
 assert len(pushes)==(5 if label=='QUOTE' else 11)
 # A rewritten/popped frame goes directly to its precise source continuation.
 continuations=[]
 for a in annotations:
  token=a['token']
  if token not in ['return.left','return.let','return.if.true']:continue
  node=a['source_expr'];payload=next(iter(view['expressions'][node].values()))
  field={'return.left':'right','return.let':'body','return.if.true':'then_expr'}[token]
  target=int(payload[field]);j=a['plan_index']+1
  while annotations[j]['token'].startswith('H:'):j+=1
  nxt=annotations[j];assert nxt['token'].startswith('eval.') and nxt['source_expr']==target,(label,a,nxt,target)
  continuations.append({'return_plan_index':a['plan_index'],'source_expr':node,'direct_eval_target':target,'next_plan_index':a['plan_index']+1,'next_token':annotations[a['plan_index']+1]['token'],'next_eval_plan_index':j,'next_eval_token':nxt['token']})
 def locate(token,expr):
  found=[a for a in annotations if a['token']==token and a['source_expr']==expr];assert len(found)==1;return found[0]
 lifetimes=[]
 for push in pushes:
  kind=push['frame_kind'];expr=push['source_expr'];push_a=annotations[push['push_plan_index']]
  if kind=='ReturnF':pop=[a for a in annotations if a['token']=='return.call.valid'][0];rewrite=None
  elif kind=='LetF':rewrite=locate('return.let',expr);pop=locate('return.scope',expr)
  elif kind=='LeftF':rewrite=locate('return.left',expr);pop=locate('return.right',expr)
  else:assert kind=='IfF';rewrite=None;pop=locate('return.if.true',expr)
  assert push_a['plan_index']<pop['plan_index']
  if rewrite:assert push_a['plan_index']<rewrite['plan_index']<pop['plan_index']
  lifetimes.append({'initial_kind':kind,'source_expr':expr,'push_event':push_a['event_ordinal'],'rewrite_event':rewrite['event_ordinal'] if rewrite else None,'rewritten_kind':'ScopeF' if kind=='LetF' else 'RightF' if kind=='LeftF' else None,'pop_event':pop['event_ordinal'],'pop_token':pop['token']})
 # Fixed frame intervals must be nested or disjoint, never crossing. This also
 # rejects an extra Scope pop, without interpreting a production frame stack.
 for a in lifetimes:
  for b in lifetimes:
   assert not a['push_event']<b['push_event']<a['pop_event']<b['pop_event']
 assert len([x for x in lifetimes if x['pop_token']=='return.scope'])==len(node_lists['eval.let'])
 depth=0;depths=[0]
 for event in events:
  delta=legacy[event['catalog_rule']]['deltas']['base'][8] if event['catalog_rule'].startswith('k.machine.') else 0
  depth+=delta;assert depth>=0
  count=sum(x['push_event']<len(depths)<=x['pop_event'] for x in lifetimes)
  assert depth==count,(label,event['ordinal'],depth,count)
  depths.append(depth)
 assert depth==0;peak=4 if label=='QUOTE' else 7;assert max(depths)==peak
 run=read('EXPECTED-'+label+'-NORMAL-RUN.json');assert run['capacity_counts']['logical_frame_peak']==peak
 certs[label]={'literal_node_correspondence':node_lists,'plan_annotations':annotations,'staged_push_certificates':pushes,'direct_eval_continuations':continuations,'frame_lifetimes':lifetimes,'prefix_logical_depth':depths,'logical_frame_peak':max(depths),'final_logical_depth':depth}
write('FIXED-NORMAL-CONTINUITY-CERTIFICATES.json',{'scope':'Two fixed normal source/path certificates only; static catalogue effects, node references and interval/depth arithmetic, not a generic machine/model or source evaluator','specimens':certs})
write('CONTINUITY-CHECKS.json',{'status':'PASS_FIXED_NORMAL_CONTROL_AND_FRAME_CERTIFICATES','checks':['Only ScheduleNext-producing predecessors have Push','ReturnLeft/ReturnLet/ReturnIfTrue go directly to their exact prepared-source continuation','Let and binary top rewrites retain their frame lifetime','If consumes its frame and creates no Scope','Every Scope pop pairs with an actual Let scope','Frame intervals are properly nested and catalogue depth deltas agree at every event prefix','Final frame depth zero; peaks Quote4 Eligibility7'],'quoted_plan_pushes':5,'eligibility_plan_pushes':11,'executed':'Inspected stdlib fixed certificate checker only; no source AST evaluation, production interpreter/compiler/transition or model executed'})
print('PASS_FIXED_NORMAL_CONTROL_AND_FRAME_CERTIFICATES')
