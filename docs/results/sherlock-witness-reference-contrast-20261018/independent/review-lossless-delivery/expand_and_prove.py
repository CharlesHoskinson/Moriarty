"""Data-only expansion and comparison, with no evaluator/model/source imports."""
import hashlib
import json
from pathlib import Path

D=Path(__file__).resolve().parent
F=D/'FACTORED-ORACLES.json'
def canonical(x):return json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
def pin(p):
    b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
factored=json.loads(F.read_text())
assert factored['format']=='lossless-named-json-tables/1'
constants=factored['constants']
assert all(name[0] in factored['constantTypePrefixes'] and name[1:].isdigit() for name in constants)
memo={}
def expand(x,trail=()):
    if isinstance(x,dict):
        if '$ref'in x:
            assert set(x)=={'$ref'} and isinstance(x['$ref'],str)
            name=x['$ref'];assert name in constants and name not in trail
            if name not in memo:memo[name]=expand(constants[name],trail+(name,))
            return memo[name]
        if '$table'in x:
            assert set(x)=={'$table'} and set(x['$table'])=={'columns','rows'}
            t=x['$table'];cols=t['columns'];assert len(set(cols))==len(cols) and all(isinstance(k,str) for k in cols)
            out=[]
            for row in t['rows']:
                assert isinstance(row,list) and len(row)==len(cols)
                out.append({k:expand(v,trail) for k,v in zip(cols,row)})
            return out
        return {k:expand(v,trail) for k,v in x.items()}
    if isinstance(x,list):return [expand(v,trail) for v in x]
    return x
proof=[]
for document in factored['documents']:
    original=document['original'];p=Path(original['path']);before=pin(p);assert before==original
    copy=document['reviewCopy'];assert pin(Path(copy['path']))==copy
    assert Path(copy['path']).read_bytes()==p.read_bytes()
    source=json.loads(p.read_text());expanded=expand(document['body'])
    a=canonical(source);b=canonical(expanded)
    assert source==expanded and a==b
    assert pin(p)==before
    proof.append({'original':original,'reviewCopy':copy,'originalCanonicalBytes':len(a),'expandedCanonicalBytes':len(b),'originalCanonicalSHA256':hashlib.sha256(a).hexdigest(),'expandedCanonicalSHA256':hashlib.sha256(b).hexdigest(),'allJSONFieldsAndArrayOrderEqual':True,'exactRawSourceAndReviewCopyPinsUnchanged':True,'snapshots':len(source.get('snapshots',[])),'records':len(source.get('records',[]))})
for name in constants:expand({'$ref':name})
result={'status':'PASS_LOSSLESS_DATA_ROUNDTRIP','factored':pin(F),'format':'lossless-named-json-tables/1','definitionCount':len(constants),'documents':proof,'scope':'JSON data expansion/comparison only; no semantic projection, evaluator, subject source, model, compiler, test or provider execution; original files unchanged.'}
(D/'ROUNDTRIP-PROOF.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'status':result['status'],'documents':len(proof),'definitions':len(constants),'factoredBytes':F.stat().st_size}))
