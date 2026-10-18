"""One-off JSON factoring for reviewer delivery; no semantic evaluation/imports."""
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path

D=Path(__file__).resolve().parent
R=D.parent
Q=D/'INPUT-DOCUMENTS.json'
def wire(x):return json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def pin(p):
    b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
def table(columns,objects):
    assert all(set(x)==set(columns) for x in objects)
    return {'$table':{'columns':columns,'rows':[[x[c] for c in columns] for x in objects]}}
snapshot_columns=['program','control','environment','frames_top_first','stack_limit','remaining_fuel','initial_fuel','outcome']
record_columns=['rule_start_line','charge','before_snapshot','after_snapshot']
projection=json.loads(Q.read_text())
documents=[];counts=Counter();objects={};contexts=defaultdict(set)
for member in projection['opaqueGoldensForReviewer']:
    p=Path(member['consumerLiteralInput']['path']);raw=p.read_bytes()
    assert pin(p)==member['consumerLiteralInput']
    body=json.loads(raw)
    def reject_reserved(x):
        if isinstance(x,dict):
            assert not ({'$ref','$table'} & set(x))
            for v in x.values():reject_reserved(v)
        elif isinstance(x,list):
            for v in x:reject_reserved(v)
    reject_reserved(body)
    if 'snapshots' in body:body['snapshots']=table(snapshot_columns,body['snapshots'])
    if 'intervals' in body:body['intervals']=table(['A_record_index','A_rule_line','first_B_record','B_records_exclusive','helper_rule_ids','legacy_rule_id'],body['intervals'])
    if 'records'in body:body['records']=table(record_columns,body['records'])
    documents.append({'original':pin(p),'reviewCopy':member['losslessReviewCopy'],'body':body})

def walk(x,context=''):
    if not isinstance(x,(dict,list)):return
    key=wire(x);counts[key]+=1;objects[key]=x;contexts[key].add(context)
    if isinstance(x,dict):
        if '$table'in x:
            t=x['$table'];cols=t['columns']
            for row in t['rows']:
                for col,v in zip(cols,row):walk(v,col)
        else:
            for k,v in x.items():walk(v,k)
    else:
        for v in x:walk(v,context+'Item')
for doc in documents:walk(doc['body'])

def kind(x,ctx):
    for c,n in [('program','Program'),('complete_program','Program'),('control','Control'),('environment','Environment'),('saved_environment','Environment'),('frames_top_first','FrameStack'),('frames_top_firstItem','Frame')]:
        if c in ctx:return n
    if isinstance(x,dict):
        if set(x)=={'name','value'}:return 'Binding'
        if any(k in x for k in ['Var','Literal','Binary','Call','If','Match']):return 'Expression'
        if any(k in x for k in ['Eval','Push','PrimitiveResult','Apply']):return 'Control'
        if 'saved_environment'in x:return 'SavedPayload'
        if 'body'in x and 'pattern'in x:return 'MatchArm'
        if 'functions'in x and 'entry'in x:return 'Program'
        return 'JsonObject'
    return 'JsonArray'

# Only repeated composites whose literal representation is longer than a named
# reference are factored. Scalar values and every field/list entry remain exact.
candidates={key for key,n in counts.items() if n>1 and len(key)>=30 and '$table' not in objects[key]}
prefixes={'Program':'P','Control':'C','Environment':'E','FrameStack':'S','Frame':'F','Binding':'B','Expression':'A','SavedPayload':'V','MatchArm':'M','JsonObject':'O','JsonArray':'L'}
serial=Counter();names={};types={}
for key in sorted(candidates,key=lambda k:(kind(objects[k],contexts[k]),len(k),k)):
    typ=kind(objects[key],contexts[key]);serial[typ]+=1;name=prefixes[typ]+str(serial[typ]).zfill(2);names[key]=name;types[name]=typ

def encode(x,bypass=None):
    if not isinstance(x,(dict,list)):return x
    key=wire(x)
    if key in names and key!=bypass:return {'$ref':names[key]}
    if isinstance(x,list):return [encode(v) for v in x]
    return {k:encode(v) for k,v in x.items()}
constants={names[k]:encode(objects[k],k) for k in names}
bodies=[{'original':d['original'],'reviewCopy':d['reviewCopy'],'body':encode(d['body'])} for d in documents]

# Remove definitions made unreachable by factoring their parents. Inline any
# once-used definition if doing so shortens the delivery (pure structural edit).
while True:
    refs=Counter()
    def used(x):
        if isinstance(x,dict):
            if set(x)=={'$ref'}:refs[x['$ref']]+=1
            else:
                for v in x.values():used(v)
        elif isinstance(x,list):
            for v in x:used(v)
    for d in bodies:used(d['body'])
    for v in constants.values():used(v)
    zero={n for n in constants if refs[n]==0}
    if zero:
        for n in zero:del constants[n];del types[n]
        continue
    once={n for n in constants if refs[n]==1}
    if not once:break
    def inline(x,trail=()):
        if isinstance(x,dict):
            if set(x)=={'$ref'} and x['$ref'] in once:
                n=x['$ref'];assert n not in trail;return inline(constants[n],trail+(n,))
            return {k:inline(v,trail) for k,v in x.items()}
        if isinstance(x,list):return [inline(v,trail) for v in x]
        return x
    for d in bodies:d['body']=inline(d['body'])
    constants={n:inline(v) for n,v in constants.items() if n not in once}
    types={n:v for n,v in types.items() if n not in once}

result={'format':'lossless-named-json-tables/1','meaning':'Data substitution only. $ref replaces a complete JSON value; $table replaces an ordered object array by complete named columns and every row. No inheritance, defaults, semantic projection, omitted fields or computed outcomes.','constantTypePrefixes':{v:k for k,v in prefixes.items()},'constants':constants,'documents':bodies}

def readable(x,level=0):
    # Complete table rows and each named constant occupy separate readable lines;
    # no base64/compression or ellipsis. Scalar text is ordinary JSON.
    indent='  '*level
    if isinstance(x,dict):
        if set(x)=={'$table'}:
            t=x['$table'];return '{"$table":{"columns":'+json.dumps(t['columns'],ensure_ascii=False,separators=(',',':'))+',"rows":[\n'+',\n'.join('  '*(level+1)+json.dumps(row,ensure_ascii=False,separators=(',',':')) for row in t['rows'])+'\n'+indent+']}}'
        if len(json.dumps(x,ensure_ascii=False,separators=(',',':')))<900 and level>1:
            return json.dumps(x,ensure_ascii=False,separators=(',',':'))
        return '{\n'+',\n'.join('  '*(level+1)+json.dumps(k)+': '+readable(v,level+1) for k,v in x.items())+'\n'+indent+'}'
    if isinstance(x,list):
        if len(json.dumps(x,ensure_ascii=False,separators=(',',':')))<900:return json.dumps(x,ensure_ascii=False,separators=(',',':'))
        return '[\n'+',\n'.join('  '*(level+1)+readable(v,level+1) for v in x)+'\n'+indent+']'
    return json.dumps(x,ensure_ascii=False,separators=(',',':'))
(D/'FACTORED-ORACLES.json').write_text(readable(result)+'\n')
print(json.dumps({'factoredBytes':(D/'FACTORED-ORACLES.json').stat().st_size,'constants':len(constants),'originalBytes':sum(d['original']['bytes'] for d in bodies)}))
