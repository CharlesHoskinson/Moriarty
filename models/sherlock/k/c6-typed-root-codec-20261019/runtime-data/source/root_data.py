"""Positive typed-root data carrier/consumer. No lane encoder or K model in Python.
Parser/framing/pin/JSON policy is reused byte-exact from approved existing APIs.
"""
import argparse
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[1]
MODEL=ROOT.parent
BASE=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-logical-constructor-origin-counterpart-20261019')
sys.path.insert(0,str(BASE/'source'))
import c6_data as base
INPUT_PIN={'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/SAFE/INPUTS.json', 'bytes': 726918, 'sha256': '6f029ff29655ff4eb73d0718b0471fce048664ad77d1bbe18b4e711e899dbc20'}
REGISTRY_PIN={'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/SAFE/PUBLIC-REGISTRY.json', 'bytes': 11263, 'sha256': 'cef0dc4bdb59a5b982593629587a7c44551b1bdfb66cead06459710642abdcf7'}
LABEL_MAP_PIN={'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-typed-root-outcome-codec-20261019/runtime-qa-postcompile-01/context/COMPILED-KLABEL-MAP.json', 'bytes': 87648, 'sha256': 'ed7c647f36e5acbce191554472023a23d303f4c1a790f178d952cc3a11f3a53c'}
EXPECTED_PIN={'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-actual-c6-programview-interface-oracle-20261019/K-C6-typed-root-codec-20261019/private/EXPECTED-COMPLETE-ANSWERS.json', 'bytes': 613495, 'sha256': '4ca1534806e3d002fa5dfde6653883782af9aaa5de953af14712192221752688'}
OUTPUT_SCHEMA='sherlock-c6-typed-root-positive-codec-wire/1'
PROFILE='logical-C6-typed-root-positive-codec/1'
PUBLIC=base.loads(base.checked(REGISTRY_PIN))
DATA_REG={r['name']:(r['sort'],tuple(r['argument_sorts'])) for r in PUBLIC['constructors']}
ENTRY=PUBLIC['function_entry']
ENTRY_REG={ENTRY['name']:(ENTRY['sort'],tuple(ENTRY['argument_sorts']))}
base.codec.REG=dict(DATA_REG)

def keys(value,want):
    if type(value)is not dict or set(value)!=set(want):raise ValueError('exact JSON carrier keyset differs')
    return value

def kore_label(raw):
    # Restricted declared label alphabet, as evidenced by ModuleToKORE/StringUtil.
    codes={'(':'LPar',')':'RPar',',':'Comm','_':'Unds'}
    result='Lbl';quoted=False
    for char in raw:
        if char.isascii() and (char.isalnum() or char=='-'):
            if quoted:result+="'";quoted=False
            result+=char
        else:
            if char not in codes:raise ValueError('outside declared compiled-label alphabet')
            if not quoted:result+="'";quoted=True
            result+=codes[char]
    return result+("'" if quoted else '')

def validate_label_map(data):
    keys(data,('status','definition','constructor_count','constructors','surface_registry_scope','method'))
    if type(data['constructors'])is not list or type(data['constructor_count'])is not int or data['constructor_count']!=len(base.codec.REG):
        raise ValueError('complete compiled label map count required')
    lines=base.checked(data['definition']).decode('utf-8').splitlines()
    forward={};reverse={}
    for row in data['constructors']:
        keys(row,('surface_constructor','sort','argument_sorts','declaring_module','declaration_source',
                  'compiled_klabel','kore_symbol','declaration_line','complete_symbol_declaration'))
        name=row['surface_constructor'];raw=row['compiled_klabel']
        if type(name)is not str or name not in base.codec.REG or name in forward or type(raw)is not str or raw in reverse:
            raise ValueError('unknown/duplicate compiled label declaration')
        sort,sorts=base.codec.REG[name]
        if row['sort']!=sort or type(row['argument_sorts'])is not list or tuple(row['argument_sorts'])!=sorts:
            raise ValueError('compiled label sort/arguments differ')
        # No surface alias fallback. The owner module is part of this monomorphic label.
        module=row['declaring_module']
        if type(module)is not str or not module:raise ValueError('declaring module required')
        declared=name+'('+','.join('_' for _ in sorts)+')_'+module+'_'+sort+''.join('_'+s for s in sorts)
        if raw!=declared or row['kore_symbol']!=kore_label(raw):raise ValueError('compiled raw/symbol mapping differs')
        line=row['declaration_line'];body=row['complete_symbol_declaration']
        if (type(line)is not int or not 1<=line<=len(lines) or type(body)is not str or lines[line-1]!=body
                or not body.startswith('  symbol '+row['kore_symbol']+'{}(')):
            raise ValueError('actual compiled symbol declaration membership differs')
        signature='('+', '.join('Sort'+s+'{}' for s in sorts)+') : Sort'+sort+'{} '
        if '{}'+signature not in body:raise ValueError('actual compiled symbol signature differs')
        base.checked(row['declaration_source'])
        forward[name]=raw;reverse[raw]=name
    if set(forward)!=set(base.codec.REG):raise ValueError('compiled label map missing registry entry')
    return forward,reverse


def validated_labels():
    # Same existing label validator, with the new closed67-member declaration set.
    saved=base.codec.REG
    try:
        base.codec.REG={**DATA_REG,**ENTRY_REG}
        return validate_label_map(base.loads(base.checked(LABEL_MAP_PIN)))
    finally:base.codec.REG=saved


LABELS,REVERSE_LABELS=validated_labels()

def from_tree(tree):
    if type(tree)is not dict or type(tree.get('sort'))is not str:raise ValueError('typed tree required')
    sort=tree['sort']
    if sort in ('Int','Bool','String'):
        keys(tree,('sort','value'))
        return base.encode(tree['value'],sort)
    keys(tree,('sort','constructor','arguments'))
    name,args=tree['constructor'],tree['arguments']
    if type(name)is not str or name not in base.codec.REG or type(args)is not list:raise ValueError('unknown typed constructor')
    actual,sorts=base.codec.REG[name]
    if sort!=actual or len(args)!=len(sorts):raise ValueError('typed tree sort/arity differs')
    children=[from_tree(x) for x in args]
    if tuple(x.sort for x in children)!=sorts:raise ValueError('child typed sorts differ')
    return base.codec.C(name,*children)

def encode_node(term):
    if term.name in ('@Int','@Bool','@String'):
        expected={'Int':int,'Bool':bool,'String':str}
        if term.sort not in expected or type(term.args[0])is not expected[term.sort]:raise ValueError('scalar kind differs')
        return {'node':'KToken','sort':{'node':'KSort','name':term.sort,'params':[]},'token':base.codec.render(term)}
    if term.name not in base.codec.REG:raise ValueError('outside closed carrier profile (including Map/Ref/symbolic)')
    sort,sorts=base.codec.REG[term.name]
    if term.sort!=sort or tuple(x.sort for x in term.args)!=sorts:raise ValueError('typed constructor differs')
    if term.name not in LABELS:raise ValueError('constructor lacks compiled label mapping')
    return {'node':'KApply','label':{'node':'KLabel','name':LABELS[term.name],'params':[]},
            'arity':len(term.args),'args':[encode_node(x) for x in term.args]}

def envelope(term):return {'format':'KAST','version':4,'term':encode_node(term)}

def decode_node(value,sort):
    if type(value)is not dict:raise ValueError('K node object required')
    if value.get('node')=='KToken':
        keys(value,('node','sort','token'));s=keys(value['sort'],('node','name','params'))
        if s!={'node':'KSort','name':sort,'params':[]} or sort not in ('Int','Bool','String') or type(value['token'])is not str:
            raise ValueError('exact primitive KSort/token required')
        term=base.codec.parse_printed(value['token'],sort)
        if type(term.args[0])is not {'Int':int,'Bool':bool,'String':str}[sort] or base.codec.render(term)!=value['token']:
            raise ValueError('noncanonical scalar lexeme')
        return term
    keys(value,('node','label','arity','args'))
    if value['node']!='KApply':raise ValueError('outside KApply/KToken carrier profile')
    label=keys(value['label'],('node','name','params'))
    if label['node']!='KLabel' or label['params']!=[] or type(label['name'])is not str or label['name'] not in REVERSE_LABELS:
        raise ValueError('unknown or parametric label')
    name=REVERSE_LABELS[label['name']]
    actual,sorts=base.codec.REG[name]
    if actual!=sort or type(value['arity'])is not int or value['arity']!=len(sorts) or type(value['args'])is not list or len(value['args'])!=len(sorts):
        raise ValueError('exact KApply sort/arity/argument count differs')
    return base.codec.C(name,*(decode_node(x,s) for x,s in zip(value['args'],sorts)))

def decode_document(value,sort):
    keys(value,('format','version','term'))
    if value['format']!='KAST' or type(value['version'])is not int or value['version']!=4:raise ValueError('KAST JSONv4 required')
    return decode_node(value['term'],sort)


def queries(document):
    keys(document,('schema','profile','admission','cases'))
    if document['schema']!='sherlock-c6-typed-root-codec-input/1' or document['profile']!=PROFILE:
        raise ValueError('source profile/schema differs')
    names=set();tail=base.codec.C('ctNoQueries')
    for row in reversed(document['cases']):
        keys(row,('case_id','root'))
        if type(row['case_id'])is not str or row['case_id']in names:raise ValueError('case identity differs or duplicates')
        names.add(row['case_id']);root=from_tree(row['root'])
        if root.sort!='CTRoot':raise ValueError('full typed root input required')
        tail=base.codec.C('ctQuery',base.codec.atom('String',row['case_id']),root,tail)
    return tail


def carrier(document):
    # Entry is permitted only while making/inverting the source query. Actual
    # answer decode has only DATA_REG, so unreduced entry/functions cannot pass.
    saved=base.codec.REG
    try:
        base.codec.REG={**DATA_REG,**ENTRY_REG}
        term=base.codec.C(ENTRY['name'],queries(document));wire=envelope(term)
        if not base.same(base.tree(term),base.tree(decode_document(wire,'CTAnswers'))):
            raise ValueError('whole source query/KJSON typed inverse differs')
        return term,wire
    finally:base.codec.REG=saved


def same_whole(a, b):
    stack = [(a, b)]
    while stack:
        x, y = stack.pop()
        if type(x) is not type(y): return False
        if type(x) is dict:
            if x.keys() != y.keys(): return False
            stack.extend((x[k], y[k]) for k in x)
        elif type(x) is list:
            if len(x) != len(y): return False
            stack.extend(zip(x, y))
        elif x != y: return False
    return True


def render(manifest,output,report):
    m=base.load(manifest);keys(m,('source_input','compiled_definition'))
    if not same_whole(m['source_input'],INPUT_PIN):raise ValueError('source input operand differs')
    source=base.loads(base.checked(m['source_input']))
    definition=base.loads(base.checked(LABEL_MAP_PIN))['definition']
    if m['compiled_definition']!=definition:raise ValueError('compiled definition operand differs')
    base.checked(definition)
    _,wire=carrier(source)
    product=base.write(output,(json.dumps(wire,separators=(',',':'),allow_nan=False)+'\n').encode())
    result={'status':'PASS_LOSSLESS_TYPED_ROOT_QUERY_CARRIER','source_input':m['source_input'],
        'compiled_definition':definition,'compiled_label_map':LABEL_MAP_PIN,'actual_label_count':len(LABELS),
        'case_count':len(source['cases']),'same_whole_typed_query':True,'product':product,
        'expected_body_reads':0,'no_model_or_lane_evaluation_in_Python':True}
    report_pin=base.write(report,(json.dumps(result,indent=2)+'\n').encode())
    print(json.dumps({'status':result['status'],'product':product,'report':report_pin}))


def decode(manifest,output,report):
    m=base.load(manifest);keys(m,('source_input','printed'))
    if not same_whole(m['source_input'],INPUT_PIN):raise ValueError('source input operand differs')
    source=base.loads(base.checked(m['source_input']));queries(source)
    raw=base.checked(m['printed']).decode('utf-8')
    parsed=base.codec.parse_printed(base.complete_term(raw),'CTAnswers')
    answers=[];at=parsed
    while at.name=='ctAnswerRow':answers.append(at.args[0]);at=at.args[1]
    if at.name!='ctNoAnswers' or len(answers)!=len(source['cases']):raise ValueError('complete answer count differs')
    for answer,row in zip(answers,source['cases']):
        if answer.name!='ctAnswer' or answer.args[0].args[0]!=row['case_id'] or not base.same(base.tree(answer.args[1]),row['root']):
            raise ValueError('answer case/order/full original source root differs')
    # Entire actual encoded lane list AND decoded root are retained. Their
    # semantic truth is judged only by the later independent WHOLE comparison.
    actual={'schema':OUTPUT_SCHEMA,'profile':PROFILE,'answers':[base.tree(a) for a in answers]}
    product=base.write(output,(json.dumps(actual,indent=2,allow_nan=False)+'\n').encode())
    result={'status':'PASS_COMPLETE_TYPED_ROOT_ANSWER_CAPTURE','source_input':m['source_input'],
        'printed':m['printed'],'product':product,'case_count':len(answers),'all_input_roots_retained':True,
        'all_actual_encoded_lanes_and_decoded_roots_retained':True,'expected_body_reads':0,
        'semantic_equality':'PENDING independent whole answer comparison; no Python lane calculation'}
    rp=base.write(report,(json.dumps(result,indent=2)+'\n').encode())
    print(json.dumps({'status':result['status'],'product':product,'report':rp}))


def compare(manifest):
    # Exact existing c6_data compare ordering/API policy, with profile status only.
    m=base.load(manifest);keys(m,('actual','expected','durable_freeze'))
    # Hash-only opaque operand binding; precedes every freeze/actual/expected body read.
    if not same_whole(m['expected'],EXPECTED_PIN):raise ValueError('opaque expected operand differs')
    barrier=base.loads(base.checked(m['durable_freeze']));base.validate_barrier(barrier,m['actual'])
    actual=base.loads(base.checked(m['actual']))
    expected=base.loads(base.checked(m['expected']))
    equal=same_whole(actual,expected)
    print(json.dumps({'status':'PASS_WHOLE_TYPED_C6_ROOT_CODEC' if equal else 'FAIL',
        'whole_typed_equal':equal,'actual':m['actual'],'expected':m['expected'],
        'expected_body_reads':1,'scope':'UNADMITTED positive codec, no interpreter/native/refinement claim'}))
    raise SystemExit(0 if equal else 1)


def main():
    cli=argparse.ArgumentParser(description=__doc__);sub=cli.add_subparsers(dest='action',required=True)
    for name in ('render','decode'):
        p=sub.add_parser(name);p.add_argument('manifest');p.add_argument('output');p.add_argument('report')
    p=sub.add_parser('compare');p.add_argument('manifest');a=cli.parse_args()
    if a.action=='render':render(a.manifest,a.output,a.report)
    elif a.action=='decode':decode(a.manifest,a.output,a.report)
    else:compare(a.manifest)


if __name__=='__main__':main()
