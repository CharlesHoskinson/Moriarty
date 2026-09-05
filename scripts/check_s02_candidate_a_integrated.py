from __future__ import annotations
import argparse
import hashlib
import inspect
import json
import re
from pathlib import Path,PurePosixPath
import sys
if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from scripts.a4_carrier import R,M,V,Invalid,require,loads,decode,encode,typed
from scripts.a4_inventory import descriptors,history
from scripts.a4_agreement import core

ROOT=Path(__file__).resolve().parents[1]
FROZEN_EXTRA={
 'moriarty/__init__.py':'6d6b58f9875c357d5d16858e55ce81edd1e56ecb1c5b28476c7a911cad2e1127',
 'specs/quint/s02/consumption.qnt':'3dd07f3c5f274aff4fce4de9c245aa0b1f1fe7f68e29e5de95a0b1c42a00f93b',
 'specs/quint/s02/policies.qnt':'0886619203c3de16f2326e27e9d4ae2ddfda8c54742c231dd4e824397c5fa9fe',
 'specs/quint/s02/authorization.qnt':'b59779d5e2e7f1bf0a14952dfe1cd3ab70bcfe813210d690abbb3e3205a6df61',
 'specs/quint/s02/execution.qnt':'cf52f55ad1810548b7c45e32552b977a30fc8dd937dfbd01d35829b6beec8928',
 'specs/quint/s02/candidate_a_authority_adapter.qnt':'3f3b093f4c718dd08eda38e610de700d0a24138beb82fd7b2b12dcf9d300bda8',
 'specs/quint/s02/candidate_a_authority_boundary.qnt':'95ddf75ef32b432acaec5384c89826d0dc12245b80f78fd8a60ff78d0da8164b',
 'specs/quint/s02/candidate_a_authority_swap.qnt':'294633d4213d44075df76085f67b9bd24fab23d07bc863fff8b4f22e79d10024',
 'specs/quint/s02/candidate_a_authority_swap_fixtures.qnt':'fb407555584e167ae0fddc3e59fbf6d64ecd79a00d2ad03d02f159da8429ed15',
 'specs/quint/s02/candidate_a_authority_swap_harness.qnt':'bb8991e439153af69f3c8ec52df1771786a0ce0d5360f975189958b978555e29',
 'specs/quint/s02/candidate_a_authority_swap_test.qnt':'5aed1719310ef472f8fdd5a19e8cc5188a47c7c136f7fc3158bb5a12943636c5',
 'specs/quint/s02/candidate_a_authority_installment.qnt':'f12d91938098d48a313baf7cb5218f84b6bd840da8c518d54f195e0f7fa1e4cd',
 'specs/quint/s02/candidate_a_authority_installment_fixtures.qnt':'22d975d6d615e1f8e80c453115ded79fa68f783880a036f73470814221bf1a92',
 'specs/quint/s02/candidate_a_authority_installment_harness.qnt':'9a49b2a76d0c9e27a0d06b942b4f6a3338bf54a1e005cbba6d6388e91dcf82ca',
 'specs/quint/s02/candidate_a_authority_installment_test.qnt':'b1d9c21c5285105b382525df8df2ebaa41dbf10979f400d975b40500c3f7c3d1',
 'scripts/check_s02_candidate_a_correspondence.py':'c6923d0e08ec0206dfddad70eaff5a3a9a2f4a160328802e0e69be965be90aec',
}
ENTRIES={'installment':'specs/quint/s02/candidate_a_integrated_installment_export.qnt',
         'swap':'specs/quint/s02/candidate_a_integrated_swap_export.qnt'}
PYTHON_SOURCES=frozenset(('moriarty/__init__.py','moriarty/core.py','moriarty/swap.py','scripts/check_s02_candidate_a_correspondence.py',
    'scripts/a4_carrier.py','scripts/a4_agreement.py','scripts/a4_authority.py','scripts/a4_cases.py','scripts/a4_inventory.py',
    'scripts/check_s02_candidate_a_integrated.py','scripts/s02_candidate_a_integrated_inventory.py',
    'scripts/export_s02_candidate_a_integrated.py','scripts/record_s02_candidate_a_integrated.py',
    'tests/test_s02_candidate_a_integrated.py','tests/test_s02_candidate_a_integrated_export.py'))
QNT_TESTS=('specs/quint/s02/candidate_a_integrated_export_test.qnt',
    'specs/quint/s02/candidate_a_authority_installment_test.qnt','specs/quint/s02/candidate_a_authority_swap_test.qnt')
RAW_NAMES=frozenset(('installment.itf.json','swap.itf.json'))
VARS=('authorityState','latestEvent','caseIndex','cursor')
EVENT_FIELDS=frozenset(('case_id','profile','sequence','kind','arguments','observed_guard','computations','before','after','provenance'))
RENAME={'case_id':'caseId','profile':'profile','sequence':'sequence','kind':'kind','arguments':'arguments','observed_guard':'observedGuard','computations':'computations'}

def canonical(value): return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def digest(data): return hashlib.sha256(data).hexdigest()
def fields(value,names,label): require(type(value) is dict and set(value)==set(names),label+' fields')
def same(a,b): return canonical(encode(a))==canonical(encode(b))
def raw_same(a,b): return canonical(a)==canonical(b)
def safe_path(root,name):
    require(type(name) is str and name!='' and '\\' not in name,'relative path string')
    path=PurePosixPath(name)
    require(not path.is_absolute() and all(p not in ('','..','.') for p in name.split('/')) and str(path)==name,'canonical relative path')
    resolved=root.resolve(); current=resolved
    for component in path.parts:
        current=current/component; require(not current.is_symlink(),'symlink forbidden')
    require(current.resolve().is_relative_to(resolved) and current.is_file(),'file containment')
    return current
def pin_bytes(root,pins):
    require(type(pins) is dict and pins,'nonempty pin map')
    out={}
    for name,sha in pins.items():
        require(type(sha) is str and re.fullmatch('[0-9a-f]{64}',sha),'SHA256 format')
        data=safe_path(root,name).read_bytes(); require(digest(data)==sha,'hash mismatch '+name); out[name]=data
    return out
def qnt_closure(root,entries):
    found=set(); todo=list(entries)
    while todo:
        name=todo.pop()
        if name in found: continue
        source=safe_path(root,name).read_text(); found.add(name)
        # Frozen files use relative from clauses. Refuse unsupported absolute imports.
        for suffix in re.findall(r'\bfrom\s+"([^"]+)"',source):
            require(suffix.startswith('./'),'unsupported Quint import path')
            child=str(PurePosixPath(name).parent/(suffix[2:]+'.qnt'))
            todo.append(child)
    return found
def inventory():
    return {'schema_version':2,'cases':[d|{'event_count':len(history(d))} for d in descriptors()]}

def shape(raw,expected):
    # Decode equality alone conflates Python bool/int and ITF tuple/list; reject both.
    encoded=encode(expected)
    if type(encoded) in (int,bool,str):
        if type(encoded) is int and type(raw) is dict and set(raw)=={'#bigint'}:
            require(type(decode(raw)) is int,'integer carrier'); return
        require(type(raw) is type(encoded),'scalar carrier'); return
    if isinstance(encoded,list):
        require(type(raw) is list and len(raw)==len(encoded),'ordered-list carrier')
        for a,b in zip(raw,expected): shape(a,b)
        return
    require(type(raw) is dict and set(raw)==set(encoded),'record/carrier keys')
    if isinstance(expected,M):
        require(type(raw['#map']) is list and len(raw['#map'])==len(expected.pairs),'map carrier')
        values={canonical(encode(k)):(k,v) for k,v in expected.pairs}
        for pair in raw['#map']:
            require(type(pair) is list and len(pair)==2,'map pair')
            k=decode(pair[0]); key=canonical(encode(k)); require(key in values,'map key set')
            ek,ev=values[key]
            if type(ek) is tuple:
                require(type(pair[0]) is dict and set(pair[0])=={'#tup'} and len(pair[0]['#tup'])==len(ek),'tuple map key')
                for rawkey,expectedkey in zip(pair[0]['#tup'],ek): shape(rawkey,expectedkey)
            else: shape(pair[0],ek)
            shape(pair[1],ev)
        return
    if isinstance(expected,frozenset):
        require(type(raw['#set']) is list and len(raw['#set'])==len(expected),'set carrier')
        values={canonical(encode(v)):v for v in expected}
        for x in raw['#set']:
            k=canonical(encode(decode(x))); require(k in values,'set member'); shape(x,values[k])
        return
    if isinstance(expected,V):
        require(type(raw['tag']) is str,'tag scalar')
        if expected.value==() and expected.tag!='EffectsExtractedA': require(raw['value']=={'#tup':[]},'unit carrier')
        else: shape(raw['value'],expected.value)
        return
    require(isinstance(expected,R),'record carrier')
    for name,value in expected.fields: shape(raw[name],value)

def compare_case(case,descriptor):
    fields(case,set(descriptor)|{'events'},'case')
    require(all(type(case[k]) is str and case[k]==v for k,v in descriptor.items()),'case descriptor')
    expected=history(descriptor); require(type(case['events']) is list and len(case['events'])==len(expected),'event count')
    for supplied,want in zip(case['events'],expected):
        fields(supplied,EVENT_FIELDS,'event')
        require(type(supplied['sequence']) is int,'exported sequence integer')
        for public,internal in RENAME.items():
            value=want['latest'][internal]; shape(supplied[public],value)
            require(same(decode(supplied[public]),value),'semantic event '+public)
        for boundary in ('before','after'):
            actual=typed(decode(supplied[boundary]),'Execution')
            # case-start reset before is checked against adjacent raw state by bind_event.
            if boundary=='before' and supplied['kind']=='case-start': continue
            shape(supplied[boundary],want[boundary])
            require(same(actual,want[boundary]),'semantic state '+boundary)
    return expected

def bind_event(event,raw,index,input_name,input_sha):
    p=event['provenance']; fields(p,('input_path','input_sha256','before_index','after_index'),'provenance')
    require(p['input_path']==input_name and p['input_sha256']==input_sha,'input pin linkage')
    require(type(p['before_index']) is int and type(p['after_index']) is int,'integer provenance index')
    require(p['after_index']==index and p['before_index']==max(0,index-1),'adjacent provenance indices')
    current=raw['states'][index]; previous=raw['states'][max(0,index-1)]
    for name,internal in RENAME.items():
        if name=='sequence':
            raw_sequence=decode(current['latestEvent'][internal])
            require(type(event[name]) is int and type(raw_sequence) is int and event[name]==raw_sequence,'raw provenance sequence')
            continue
        exported_raw=canonical(event[name]); raw_field=canonical(current['latestEvent'][internal])
        require(exported_raw == raw_field, 'raw provenance field '+name)
    require(raw_same(event['before'],previous['authorityState']) and raw_same(event['after'],current['authorityState']),'raw authority state linkage')

def check_document(document,admission,source_root,input_root,inventory_bytes):
    fields(document,('schema_version','inventory_sha256','source_pins','input_pins','receipt_pins','cases'),'document')
    fields(admission,('schema_version','inventory_sha256','source_pins','input_pins','receipt_pins','entries'),'admission')
    require(type(document['schema_version']) is int and document['schema_version']==2 and type(admission['schema_version']) is int and admission['schema_version']==2,'schema2')
    inv=inventory(); supplied_inventory=loads(inventory_bytes)
    require(raw_same(supplied_inventory,inv),'independent inventory mismatch')
    invhash=digest(canonical(inv).encode())
    require(document['inventory_sha256']==admission['inventory_sha256']==invhash,'inventory hash')
    require(admission['entries']==ENTRIES,'admitted entry modules')
    for group in ('source_pins','input_pins','receipt_pins'):
        require(raw_same(document[group],admission[group]),'admitted pin map '+group)
    sourcepins=document['source_pins']; expected_sources=qnt_closure(source_root,tuple(ENTRIES.values())+QNT_TESTS)|PYTHON_SOURCES
    require(set(sourcepins)==expected_sources,'complete source/tooling closure')
    pin_bytes(source_root,sourcepins)
    for name,pin in (core.PINNED|FROZEN_EXTRA).items(): require(sourcepins.get(name)==pin,'frozen source pin '+name)
    for module in (core,):
        name='scripts/check_s02_candidate_a_correspondence.py'; origin=inspect.getsourcefile(module)
        require(origin is not None and digest(Path(origin).read_bytes())==sourcepins[name],'imported Core bridge origin')
    for name in ('a4_carrier','a4_agreement','a4_authority','a4_cases','a4_inventory'):
        module=sys.modules['scripts.'+name]; origin=inspect.getsourcefile(module)
        require(origin is not None and digest(Path(origin).read_bytes())==sourcepins['scripts/'+name+'.py'],'imported checker origin '+name)
    require(digest(Path(__file__).read_bytes())==sourcepins['scripts/check_s02_candidate_a_integrated.py'],'executed checker origin')
    for fn in (core.compute_transaction,core.reduce_to_quiescence):
        origin=inspect.getsourcefile(fn); require(origin is not None and digest(Path(origin).read_bytes())==core.PINNED['moriarty/core.py'],'imported frozen Core origin')
    require(set(document['input_pins'])==RAW_NAMES,'complete raw input inventory')
    inputs=pin_bytes(input_root,document['input_pins']); pin_bytes(input_root,document['receipt_pins'])
    require(not (set(document['input_pins'])&set(document['receipt_pins'])),'receipt/input separation')
    ds=descriptors(); require(type(document['cases']) is list and len(document['cases'])==78,'complete case count')
    by_loop={loop:[] for loop in ENTRIES}
    for case,descriptor in zip(document['cases'],ds):
        compare_case(case,descriptor); by_loop[descriptor['lifecycle']].append(case)
    for loop,cases in by_loop.items():
        name=loop+'.itf.json'; raw=loads(inputs[name]); fields(raw,('#meta','vars','states'),'ITF')
        fields(raw['#meta'],('format','format-description','source','status','description','timestamp'),'ITF metadata')
        require(raw['#meta']['format']=='ITF' and raw['#meta']['source']==ENTRIES[loop],'ITF source/format metadata')
        require(raw['#meta']['format-description']=='https://apalache-mc.org/docs/adr/015adr-trace.html','ITF format description')
        require(type(raw['#meta']['timestamp']) is int and all(type(raw['#meta'][k]) is str for k in ('status','description')),'ITF metadata types')
        require(raw['vars']==list(VARS),'exact raw vars and order')
        require(type(raw['states']) is list,'ITF states')
        events=[event for case in cases for event in case['events']]
        require(len(raw['states'])==len(events),'all raw states accounted')
        index=0
        for case_index,case in enumerate(cases):
            for sequence,event in enumerate(case['events']):
                state=raw['states'][index]; fields(state,('#meta',*VARS),'ITF state')
                fields(state['#meta'],('index',),'ITF state metadata')
                require(state['#meta']['index']==index and type(state['#meta']['index']) is int,'raw state metadata index')
                require(type(decode(state['caseIndex'])) is int and decode(state['caseIndex'])==case_index,'raw case index')
                require(type(decode(state['cursor'])) is int and decode(state['cursor'])==sequence,'raw cursor')
                fields(state['latestEvent'],RENAME.values(),'latest event')
                bind_event(event,raw,index,name,document['input_pins'][name])
                if sequence==0:
                    require(event['kind']=='case-start' and (index==0 or events[index-1]['kind']=='case-end'),'reset boundary')
                    if index==0: require(raw_same(event['before'],event['after']),'initial before after identity')
                else:
                    require(event['kind']!='case-start','hidden reset')
                    require(raw_same(event['before'],events[index-1]['after']),'history continuity')
                index+=1
    return {'ok':True,'scope':'complete-inventory finite-record agreement-and-authority replay','cases':78,'events':1557,
            'symbolic_premises':['EvidenceValid','finite signature token abstraction'],
            'limitations':['not model checking','not cryptographic verification','root admission is not authenticated generator execution']}

def main():
    parser=argparse.ArgumentParser()
    for name in ('cases','admission','inventory','source-root','input-root'): parser.add_argument('--'+name,type=Path,required=True)
    parser.add_argument('--report',type=Path)
    args=parser.parse_args()
    try: report=check_document(loads(args.cases.read_bytes()),loads(args.admission.read_bytes()),args.source_root,args.input_root,args.inventory.read_bytes())
    except (Invalid,core.DecodeError,ValueError,TypeError,KeyError,IndexError,OSError) as error:
        report={'ok':False,'scope':'complete-inventory','differences':[str(error)]}
    if args.report is not None: args.report.write_text(json.dumps(report,sort_keys=True,indent=2)+'\n')
    print(json.dumps(report,sort_keys=True)); return 0 if report['ok'] else 1

if __name__=='__main__': raise SystemExit(main())
