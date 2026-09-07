from pathlib import Path
import json,hashlib,re,inspect
from collections import Counter
from graphify.build import build_from_json
from graphify.cluster import cluster,score_all
from graphify.analyze import god_nodes,surprising_connections,suggest_questions
from graphify.diagnostics import diagnose_extraction,format_diagnostic_report
from graphify.export import to_json,to_html
from graphify.cache import save_semantic_cache
r=Path(__file__).resolve().parents[2];o=r/'deliverables/moriarty-report-plan-review-2026-09-07';plan=r/'openspec/REPORT-RECONCILIATION-2026-09-07.md'
previous=json.loads((o/'graph.json').read_text()) if (o/'graph.json').exists() else {'nodes':[]}
previous_names={n['id']:n.get('community_name') for n in previous['nodes']}
data={'nodes':[],'edges':[],'hyperedges':[],'input_tokens':0,'output_tokens':0,'usage_status':'unavailable; zero schema placeholders are not measurements'}
for name in ['intents','pcd','defi']:
 d=json.loads((o/(name+'.graph.json')).read_text())
 for n in d['nodes']:n['evidence_kind']='report claim';n['implementation_status']='not established by report'
 for k in ['nodes','edges','hyperedges']:data[k].extend(d.get(k,[]))
 save_semantic_cache(d['nodes'],d['edges'],d.get('hyperedges',[]),root=r,allowed_source_files=[n['source_file'] for n in d['nodes']],prompt_file='/home/charl/.agents/skills/graphify/references/extraction-spec.md',merge_existing=True)
ids={n['id'] for n in data['nodes']}
def addnode(id,label,file,kind='plan recommendation',location=None):
 if id not in ids:data['nodes'].append(dict(id=id,label=label,file_type='concept',source_file=str(file),source_location=location,evidence_kind=kind));ids.add(id)
def edge(a,b,relation='references',kind='INFERRED',file=plan,loc=None):
 assert a in ids and b in ids,(a,b)
 data['edges'].append(dict(source=a,target=b,relation=relation,confidence=kind,confidence_score=1.0 if kind=='EXTRACTED' else .85,source_file=str(file),source_location=loc,weight=1.0))
reg=json.loads((r/'openspec/moriarty-completion-program.json').read_text())
for p in reg['packages']:
 addnode('plan_'+p['id'].lower(),p['id']+' '+p['change'].split('-',1)[1].replace('-',' '),r/p['acceptance'],location='Report reconciliation requirement')
 for dep in p['dependencies']:edge('plan_'+p['id'].lower(),'plan_'+dep.lower(),'references','EXTRACTED',r/'openspec/moriarty-completion-program.json','packages.dependencies')
for id,label in [('rp01','Financial and intent challenge'),('rp02','Native history and ledger feasibility'),('rp03','Campaign admission consistency')]:addnode('plan_'+id,label,plan,location=id.upper())
for p in reg['packages']:
 for gate in p.get('planningPrerequisites',[]):
  gid='plan_'+re.sub('[^a-z0-9_]','_',gate.lower());addnode(gid,gate,plan,location='Gate records and completion predicates');edge('plan_'+p['id'].lower(),gid,'references','EXTRACTED',r/'openspec/moriarty-completion-program.json','planningPrerequisites')
for stage in reg['reportReconciliation']['stageAdmission']['stages']:
 addnode('admission_'+stage['id'].replace('-','_'),stage['id']+' admission stage',r/'openspec/moriarty-completion-program.json',location='reportReconciliation.stageAdmission.stages')
for stage in reg['reportReconciliation']['stageAdmission']['stages']:
 for dep in stage['requires']:edge('admission_'+stage['id'].replace('-','_'),'admission_'+dep.replace('-','_'),'references','EXTRACTED',r/'openspec/moriarty-completion-program.json','stage.requires')
 for owner in stage['owners']:edge('admission_'+stage['id'].replace('-','_'),'plan_'+owner.lower(),'references','EXTRACTED',r/'openspec/moriarty-completion-program.json','stage.owners')
addnode('roadmap_moriarty_midnight_language','Moriarty Midnight-centric language',r/'ROADMAP.md',location='Product scope')
for p in reg['packages']:edge('roadmap_moriarty_midnight_language','plan_'+p['id'].lower(),'references','EXTRACTED',r/'ROADMAP.md',p['id'])
rows=[
('R01','Midnight-centric design','pcd','midnight',['MC01','MC03','MC04'], 'RP02', 'Midnight Compact, native proofs, private state and ledger constrain design; other chains comparative only'),
('R02','Finite values and lifetime','defi','unbounded_numeric_domains_proposal',['MC01','MC06'],'RP01','Reject unbounded-domain recommendation; preserve registered bounds and conserved work'),
('R03','Mandatory PCD','intents','optional_zk',['MC03','MC05'],'RP02','Reject optional-history roadmap; deterministic subchecks cannot remove mandatory history'),
('R04','Typed financial kernel','defi','typed_open_transition_kernel',['MC01','MC07'],'RP01','Financial libraries over typed state/effects, not a primitive-count claim'),
('R05','Independent signed intent and plan','intents','plan_ir',['MC01','MC05','MC08'],'RP01','Atomic agreement grammar is initial subset; versioned outcome-intent authoring remains required'),
('R06','Nominal debt authority','intents','liability_authority',['MC05','MC07'],'RP01','Token debit caps cannot authorize new liabilities'),
('R07','Obligation preservation','defi','liability_and_obligation_record',['MC06','MC07'],'RP01','Episode closure, refinance and redemption retain live claims and identified counterparties'),
('R08','Complete financial conformance','defi','first_pass_taxonomy_crosswalk',['MC07'],'RP01','Keep 277 fixtures, 32 dispositions and 72 stable rows; normalize product/version scope'),
('R09','Model-to-source fidelity','defi','model_to_contract_fidelity',['MC04','MC07'],'RP01','Certificate must bind observation map and implementation source, not only model proof'),
('R10','Proof and property composition','pcd','composition_bindings',['MC03','MC05','MC06'],'RP02','Check predecessor/output/policy compatibility, constrained genesis and duplicate inputs'),
('R11','Private successor handoff','pcd','private_handoff',['MC03','MC06'],'RP02','Inventory serialized proof, public state, accumulator, proving data and secrets'),
('R12','Genuine branching history','pcd','dag_composition',['MC06'],'RP02','Linear fixed-state IVC does not establish private split/join'),
('R13','Composition operators','defi','asynchronous_cross_domain_composition',['MC06','MC07'],'RP01','Separate sequential/disjoint/shared-state/async semantics; model foreign facts as assumptions'),
('R14','Solvency and environment assumptions','intents','solvency',['MC05','MC07','MC08'],'RP01','Token accounting and proof validity do not establish solvency, oracle truth or custody'),
('R15','Clear semantic signing','intents','clear_signing_proof',['MC05','MC08'],'RP01','Display signed bytes, assets, recipients, limits, liabilities and assumptions'),
('R16','Finite verification versus liveness','intents','bounded_verification',['MC01','MC08'],'RP01','Bounded plan checking does not establish solver feasibility, witness availability or closure liveness')]
cross=[]
for id,label,report,suffix,packages,gate,decision in rows:
 source=f'raw_reports_unified_2026_09_07_{report}_{suffix}';nid='review_'+id.lower();addnode(nid,label,plan,location=gate);edge(nid,source,loc=gate);edge(nid,'plan_'+gate.lower(),loc=gate)
 for pkg in packages:edge(nid,'plan_'+pkg.lower(),loc=gate)
 cross.append(dict(id=id,requirement=label,reportNode=source,packages=packages,earlyGate=gate,decision=decision,evidenceKind='review inference / recommendation',status='remaining acceptance obligation'))
# Direct cross-report semantic bridges, intentionally marked as inferences.
for a,b in [('intents_obligation','defi_liability_and_obligation_record'),('intents_liability_authority','defi_authority_safety_theorem'),('pcd_composition_bindings','defi_assume_guarantee_composition_theorem'),('intents_model_fidelity','defi_model_to_contract_fidelity'),('pcd_private_handoff','intents_private_predicates_deferred'),('pcd_optional_accelerator','intents_optional_zk')]:edge('raw_reports_unified_2026_09_07_'+a,'raw_reports_unified_2026_09_07_'+b,'conceptually_related_to',loc='Decisions from the review')
observations=[('atomic_source','Implemented atomic agreement grammar','experiments/moriarty-language/spec/grammar.ebnf','R05'),('debt_gap','No explicit signed nominal-debt cap','experiments/moriarty-language/spec/semantics.md','R06'),('wrapper_gap','Complete native-to-Preview verifier unresolved','evidence/moriarty-completion-program-2026-09-07/MC04/wrapper-interface-source-02/README.md','R01'),('fixed_ivc','Fixed-instance native IVC source; unrun','experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs','R12'),('heldouts','Held-out cases require extensions','evidence/moriarty-r2b-heldouts-2026-09-06/README.md','R07')]
for id,label,file,rid in observations:addnode('observed_'+id,label,r/file,'repository observation',location='lines 1-'+str(len((r/file).read_text().splitlines())));edge('review_'+rid.lower(),'observed_'+id,file=r/file,loc='Current source/evidence scope')
(o/'crosswalk.json').write_text(json.dumps(cross,indent=2)+'\n');(o/'extraction.json').write_text(json.dumps(data,indent=2)+'\n')
# Lossless raw extraction retained. Aggregate parallel edge statements explicitly for simple-graph rendering.
agg={}
for e in data['edges']:agg.setdefault((e['source'],e['target']),[]).append(e)
view=dict(data);view['edges']=[]
for statements in agg.values():
 e=dict(statements[0]);e['statements']=statements;e['relations']=sorted({x['relation'] for x in statements});view['edges'].append(e)
health=diagnose_extraction(view,directed=True,root=r)
(o/'graph-health.json').write_text(json.dumps({'view':health,'parallelStatementsPreserved':len(data['edges'])-len(view['edges']),'losslessSource':'extraction.json'},indent=2)+'\n');print(format_diagnostic_report(health))
assert not any(health.get(k,0) for k in ['dangling_endpoint_edges','missing_endpoint_edges','self_loop_edges','directed_same_endpoint_collapsed_edges'])
G=build_from_json(view,root=r,directed=True);com=cluster(G);coh=score_all(G,com);gods=god_nodes(G);sur=surprising_connections(G,com)
labels={k:' / '.join([G.nodes[n].get('label',n) for n in sorted(ns,key=lambda n:G.degree(n),reverse=True)[:2]]) for k,ns in com.items()}
(o/'community-candidates.json').write_text(json.dumps({k:{'suggested':labels[k],'cohesion':coh[k],'nodes':[G.nodes[n].get('label',n) for n in ns]} for k,ns in com.items()},indent=2)+'\n')
labelpath=o/'community-labels.json'
for cid,ns in com.items():
 votes=Counter(previous_names[n] for n in ns if previous_names.get(n))
 if votes:labels[cid]=votes.most_common(1)[0][0]
 else:labels[cid]='Moriarty admission requirements'
labelpath.write_text(json.dumps(labels,indent=2)+'\n')
assert to_json(G,com,str(o/'graph.json'),community_labels=labels)
assert to_html(G,com,str(o/'graph.html'),community_labels=labels)
analysis={'nodes':len(G),'edges':G.number_of_edges(),'sourceStatements':len(data['edges']),'communities':len(com),'cohesion':coh,'centralConcepts':gods,'crossCommunityLinks':sur,'questions':suggest_questions(G,com,labels),'tokenUsage':None,'tokenUsageNote':'Host-agent token usage unavailable; extraction zeros are schema placeholders, not measured zero usage.'}
(o/'analysis.json').write_text(json.dumps(analysis,indent=2)+'\n')
print(json.dumps({k:analysis[k] for k in ['nodes','edges','sourceStatements','communities']}))
