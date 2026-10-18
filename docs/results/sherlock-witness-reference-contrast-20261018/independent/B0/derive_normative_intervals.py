"""One-off data annotation of independently frozen A39 rows with R4 B intervals.

Not a program evaluator. No candidate, runtime, projector, DAG or trace import.
The A39 rows are pre-existing independent source-rule expectations. The small
positive helper schedules below are derived from the standalone normative R4
rows, with persistent semantic binding chains used only for visit accounting.
"""
import copy,json,hashlib,re
from pathlib import Path
D=Path(__file__).resolve().parent
R=D.parent
W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
NORM=W/'docs/audits/2026-10-16/sherlock-b-core-host-source/source-basis/r4/rows-supplement.json'
SCHEMA=W/'models/sherlock/joined09-host-source-20261016/basis/schema-supplement.json'
norm=json.loads(NORM.read_text());schema=json.loads(SCHEMA.read_text())
rowdefs={x['id']:x for group in ['legacy_completion_instances','helper_events','bookkeeping_events','administrative_boundaries'] for x in norm[group]}
byline={}
for x in norm['legacy_completion_instances']:
    for s in x.get('sources',[]):
        m=re.fullmatch(r'main\.k:(\d+)(?:-\d+)?',s)
        if m:byline[int(m[1])]=x['id']
def pin(p):
    b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
def h(kind,*names):return ['b.helper.'+kind+'.'+n for n in names]
def visible(chain):
    found={}
    for name,value in chain:
        if name not in found:found[name]=value
    return [{'name':n,'value':found[n]} for n in sorted(found)]
def validate_shape(value):
    # Pure walk of explicitly supplied literal DATA, not source evaluation.
    if 'UInt'in value or 'Bool'in value:return h('Validate','occurrence')
    if 'List'in value:
        xs=value['List']
        if not xs:return h('Validate','occurrence')
        return h('Validate','occurrence','field')+validate_shape({'UInt':xs[0]})+h('Validate','field')+validate_shape({'List':xs[1:]})
    tree=value['Tree']
    if 'Leaf'in tree:return h('Validate','occurrence','field')+validate_shape({'UInt':tree['Leaf']})
    left,right=tree['Branch']
    return h('Validate','occurrence','field')+validate_shape({'Tree':left})+h('Validate','field')+validate_shape({'Tree':right})
def kind(v):
    if 'List'in v:return 'Nil' if not v['List'] else 'Cons'
    if 'Tree'in v:return next(iter(v['Tree']))
    return next(iter(v))
def match_fields(pattern,value):
    tag,p=next(iter(pattern.items()))
    if tag=='Var':return True,[(p,value)],h('Match','arm','binding','selected','exit')
    if tag!=kind(value):return False,[],h('Match','arm','occurrence','next','failure')
    if tag=='Nil':return True,[],h('Match','arm','occurrence','selected','exit')
    if tag=='Leaf':return True,[(p,{'UInt':value['Tree']['Leaf']})],h('Match','arm','occurrence','field','binding','selected','exit')
    assert tag in ['Cons','Branch'] and p[0]!=p[1]
    values=([{'UInt':value['List'][0]},{'List':value['List'][1:]}] if tag=='Cons' else [{'Tree':v} for v in value['Tree']['Branch']])
    return True,list(zip(p,values)),h('Match','arm','occurrence','distinct','field','binding','field','binding','selected','exit')

paths=[R/'UIREQ035-independent-reference-prefix-oracle-20261018/expectations'/name for name in ['factorial3-complete-A39-prefix.json','list2-5-complete-A39-prefix.json','tree4-1-3-complete-A39-prefix.json','factorial2-normal-program-contrast-A39-prefix.json']]
summaries=[]
for path in paths:
    a=json.loads(path.read_text());chain=[];saved=[];pending_push=None;bound=None;case_saved=None;case_bound=None
    work=helper=visits=cursor=0;rows=[];intervals=[]
    # Numeric clock columns describe every B prefix, including the free terminal
    # publication. Semantic contents are bound to the complete independent A
    # snapshot by an explicit index. No physical helper/image payload is supplied.
    rows.append([None,None,0,0,0,0,4096,0])
    for index,event in enumerate(a['records']):
        before=a['snapshots'][index];after=a['snapshots'][index+1]
        assert visible(chain)==before['environment'],(path.name,index,'before-env')
        assert len(saved)==len(before['frames_top_first'])
        control=before['control'];tag,payload=next(iter(control.items()))
        top=before['frames_top_first'][0] if before['frames_top_first'] else None
        line=event['rule_start_line'];sequence=[];start=len(rows)-1
        if tag=='Eval' and 'Literal'in payload:sequence=h('Validate','entry')+validate_shape(payload['Literal'])+h('Validate','empty','exit')
        elif tag=='Eval' and 'Var'in payload:
            name=payload['Var'];position=next(i for i,(n,v) in enumerate(chain) if n==name)
            sequence=h('EnvGet','entry')+h('EnvGet',*(['cell']*(position+1)))+h('EnvGet','exit')
        elif line==145:sequence=h('EnvPut','entry','insert','exit')
        elif line==153:
            op=top['Right']['op']
            if op=='Eq':
                lhs=top['Right']['left'];rhs=payload
                assert 'UInt'in lhs and 'UInt'in rhs
                sequence=h('Equal','entry','pair','empty','exit') if lhs==rhs else h('Equal','entry','different','exit')
            else:
                assert op in ['Add','Sub','Mul','Lt']
                sequence=h('Scalar','entry','tag','arithmetic','exit')
        elif line in [161,165,185]:sequence=h('Scalar','entry','tag','exit')
        elif line==157:
            assert top['Unary']['op']=='Leaf'
            sequence=h('Scalar','entry','construct','exit')
        elif line in [177,181]:
            prefix=top['Args']['completed_values'];assert prefix==[]
            sequence=h('Append','entry','empty','tail','exit')
        elif line==200:
            assert len(a['complete_program']['functions'])==1
            sequence=h('Find','entry','row','exit')
        elif line==204:
            function=payload['function'];values=payload['values'];params=function['parameters']
            assert len(params)==len(values)==1
            sequence=h('Bind','entry','pair','empty','type','insert','exit')
            bound=[(params[0]['name'],values[0])]
        elif line==214:
            pattern=payload['arms'][0]['pattern'];ok,bindings,tail=match_fields(pattern,payload['value'])
            sequence=h('Match','entry')+tail
            assert (after['control']['CaseResult']['bindings'] is not None)==ok
            if ok:
                case_bound=list(case_saved)
                for binding in bindings:case_bound=[binding]+case_bound
                assert visible(case_bound)==after['control']['CaseResult']['bindings']

        for rule in sequence:
            definition=rowdefs[rule];assert definition['status']=='Enabled'
            delta=definition['deltas']['base']
            assert delta[0:3]==[-1,1,1]
            work+=1;helper+=1;visits+=definition['helperVisits']
            rows.append([index,rule,work,helper,visits,cursor,4096-work,index])
        rule='k.admin.publish.value' if line==80 else byline[line]
        if line!=80:
            work+=1;cursor+=1
            assert rowdefs[rule]['deltas']['base'][:5]==[-1,1,0,0,1]
        rows.append([index,rule,work,helper,visits,cursor,4096-work,index+1])
        assert work-helper==cursor
        intervals.append({'A_record_index':index,'A_rule_line':line,'first_B_record':start,'B_records_exclusive':len(rows)-1,'helper_rule_ids':sequence,'legacy_rule_id':rule})

        # Persist only semantic binding-chain order dictated by R4. This does not
        # evaluate source expressions or invent raw physical row IDs.
        if line==145:
            name=top['Let']['name'];chain=[(name,payload)]+list(saved[0])
        elif line in [149,161,165,173,177,181,185,195]:
            if line!=149 or saved[0] is not None:chain=list(saved[0])
        if line==173:case_saved=list(chain)
        if line==135:
            assert pending_push is not None
            frame_saved,newenv=pending_push;saved.insert(0,frame_saved);chain=list(newenv);pending_push=None
        elif line==149:saved[0]=None
        elif line in [153,157,161,165,173,181,185,195]:saved.pop(0)
        if after['control'] is not None and 'Push'in after['control']:
            cp=after['control']['Push'];fr=cp['frame'];frame_saved=None
            if any(x in fr for x in ['Let','Left','If','Match','Args','Return','Scope']):frame_saved=list(chain)
            newenv=bound if line==208 else case_bound if line==220 else chain
            if line==220:frame_saved=list(case_saved)
            pending_push=(frame_saved,list(newenv))
            assert visible(newenv)==cp['environment']
        assert visible(chain)==after['environment'],(path.name,index,'after-env')
        assert len(saved)==len(after['frames_top_first'])
        assert work<=4096
    out={'status':'INDEPENDENT_NORMATIVE_B0_CLOCK_SCHEDULE_SPECIFIED_ONLY','profile':'sherlock-helper-microstep/1','grammarDomain':'DedicatedSorts','normativeInputs':{'R4Rows':pin(NORM),'materializedSchema':pin(SCHEMA),'independentA39Prefix':pin(path)},'initialFuel':4096,'clockPrefixColumns':['A_record_index_or_null','preceding_B_rule_id_or_null','consumed_work','helper_work','helper_visits','reference_cursor','remaining_work','erased_semantic_A_snapshot_index'],'clockPrefixes':rows,'intervals':intervals,'semanticErasureBinding':'Each helper prefix refers to the complete independent legacy pre-snapshot; a completion refers to its complete post-snapshot. This is the declared eraseHelpers target on these normal uncut intervals; it is not raw MachineImage equality. A virtual fuel remains the A oracle field, separately from remaining_work.','unbound':['Raw physical data/env/frame/task row indices and complete helper payload/cursor snapshots','Current implementation transition validity and exact normative-row realization','Any cache/helper rewrite preserving occurrence/counter details outside this selected schedule','Allocation/physical capacity history and actual metrics; helper_visits remains a diagnostic distinct from charged helper_work'],'execution':'No subject/model/compiler/provider or actual trace read; source A rows already independently frozen. This file is not a claim that actual runtime realizes these clocks.'}
    dest=D/(path.stem+'-B0-clocks.json');dest.write_text(json.dumps(out,separators=(',',':'))+'\n')
    summaries.append({'oracle':pin(dest),'B_records':len(rows)-1,'consumed_work':work,'helper_work':helper,'helper_visits':visits,'reference_cursor':cursor,'remaining_work':4096-work,'sourceAWork':a['charged_A_work'],'SI04_numeric_each_prefix':True})
(D/'NUMERIC-SUMMARY.json').write_text(json.dumps({'status':'DERIVED_SPECIFIED_ONLY_NOT_SUBJECT_OBSERVED','cases':summaries},indent=2)+'\n')
print(json.dumps({'derivedCases':len(summaries),'subjectExecution':False}))
