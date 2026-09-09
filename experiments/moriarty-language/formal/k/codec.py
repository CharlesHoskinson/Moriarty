"""Closed projection codec. Financial predicates and changed numbers come only from K."""
import copy
import hashlib
import json
import re

MAX = 2**128 - 1
LIMIT = 65536
VERSION = 'moriarty-funded-repayment/0'
class CodecError(ValueError): pass

def error(code): raise CodecError(code)
def closed(o, keys):
    if type(o) is not dict or set(o) != set(keys.split()): error('MALFORMED_INPUT')
def ident(v):
    if type(v) is not str or re.fullmatch(r'[A-Za-z][A-Za-z0-9_]{0,63}',v) is None: error('MALFORMED_INPUT')
def uint(v):
    if type(v) is not str or len(v)>39 or re.fullmatch(r'0|[1-9][0-9]*',v) is None or int(v)>MAX: error('MALFORMED_INPUT')
def load(text):
    if type(text) is not str or len(text)>LIMIT: error('MALFORMED_INPUT')
    try:
        if len(text.encode('utf8'))>LIMIT: error('MALFORMED_INPUT')
        def pairs(ps):
            d={}
            for k,v in ps:
                if k in d: error('MALFORMED_INPUT')
                d[k]=v
            return d
        return json.loads(text,object_pairs_hook=pairs,parse_constant=lambda _:error('MALFORMED_INPUT'))
    except (ValueError,UnicodeError,RecursionError): error('MALFORMED_INPUT')

def admit(text):
    p=load(text); closed(p,'schemaVersion state actions')
    if p['schemaVersion']!=VERSION: error('MALFORMED_INPUT')
    s=p['state']; closed(s,'balances allowances obligations usedTransferIds usedAllocationIds work')
    for name in ['balances','allowances','obligations','usedTransferIds','usedAllocationIds']:
        if type(s[name]) is not list: error('MALFORMED_INPUT')
    if type(p['actions']) is not list: error('MALFORMED_INPUT')
    for b in s['balances']:
        closed(b,'party asset amount'); ident(b['party']);ident(b['asset']);uint(b['amount'])
    for a in s['allowances']:
        closed(a,'party asset remaining spent');ident(a['party']);ident(a['asset']);uint(a['remaining']);uint(a['spent'])
    for o in s['obligations']:
        closed(o,'id debtor creditor denomination settlementAsset principal accrued outstanding allocationRule conversion status')
        for k in ['id','debtor','creditor','denomination','settlementAsset']: ident(o[k])
        for k in ['principal','accrued','outstanding']: uint(o[k])
        if o['allocationRule'] not in ['AccrualFirst','PrincipalFirst','ProRata'] or o['status'] not in ['Outstanding','Settled']: error('MALFORMED_INPUT')
        c=o['conversion'];closed(c,'mantissa scale rounding');uint(c['mantissa']);uint(c['scale'])
        if c['rounding'] not in ['none','floor','ceil']:error('MALFORMED_INPUT')
    w=s['work'];closed(w,'remaining spent closureReserve')
    for v in w.values():uint(v)
    for k in ['usedTransferIds','usedAllocationIds']:
        for v in s[k]:ident(v)
    for a in p['actions']:
        if type(a) is not dict: error('MALFORMED_INPUT')
        if a.get('kind')=='Transfer':
            closed(a,'kind id from to asset amount')
            for k in ['id','from','to','asset']:ident(a[k])
            uint(a['amount'])
        elif a.get('kind')=='Repay':
            closed(a,'kind allocationId transferId obligationId payer nominalAmount')
            for k in ['allocationId','transferId','obligationId','payer']:ident(a[k])
            uint(a['nominalAmount'])
        else:error('MALFORMED_INPUT')
    if len(s['balances'])!=2 or len(s['allowances'])!=1 or len(s['obligations'])!=1 or s['usedTransferIds'] or s['usedAllocationIds']:error('UNSUPPORTED_PROJECTION')
    if [a['kind'] for a in p['actions']]!=['Transfer','Repay']:error('UNSUPPORTED_PROJECTION')
    o=s['obligations'][0]
    if o['allocationRule']=='ProRata' or o['conversion']!={'mantissa':'1','scale':'0','rounding':'none'}:error('UNSUPPORTED_PROJECTION')
    return p

def digest(p):return hashlib.sha256(json.dumps(p,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def encode(p):
    s=p['state'];o=s['obligations'][0];a=s['allowances'][0];w=s['work'];t,r=p['actions']
    def term(name,values,numeric=()):
        return name+'('+', '.join(v if i in numeric else json.dumps(v) for i,v in enumerate(values))+')'
    bs=[term('balance',[b['party'],b['asset'],b['amount']],(2,)) for b in s['balances']]
    return 'packet('+', '.join(bs+[
        term('allowance',[a[k] for k in ['party','asset','remaining','spent']],(2,3)),
        term('obligation',[o[k] for k in ['id','debtor','creditor','denomination','settlementAsset','principal','accrued','outstanding','allocationRule','status']],(5,6,7)),
        term('work',[w[k] for k in ['remaining','spent','closureReserve']],(0,1,2)),
        term('transfer',[t[k] for k in ['id','from','to','asset','amount']],(4,)),
        term('repay',[r[k] for k in ['allocationId','transferId','obligationId','payer','nominalAmount']],(4,)),json.dumps(digest(p))])+')'

REJECTIONS={-1:{'DUPLICATE','INVARIANT','INSUFFICIENT_WORK','OVERFLOW'},0:{'ZERO_AMOUNT','SELF_TRANSFER','MISSING_BALANCE','INSUFFICIENT_BALANCE','MISSING_ALLOWANCE','INSUFFICIENT_ALLOWANCE','OVERFLOW'},1:{'ZERO_AMOUNT','MISSING_OBLIGATION','NOT_OUTSTANDING','EXCEEDS_OUTSTANDING','TRANSFER_NOT_IN_STEP','TRANSFER_MISMATCH','INSUFFICIENT_UNALLOCATED'}}
def decode(text,p):
    try:
        if len(text)>1024*1024: error('K_OUTPUT')
        def unique_pairs(pairs):
            result={}
            for key,value in pairs:
                if key in result:error('K_OUTPUT')
                result[key]=value
            return result
        data=json.loads(text,object_pairs_hook=unique_pairs,parse_constant=lambda _:error('K_OUTPUT'))
        if set(data)!= {'format','version','term'} or data['format']!='KAST' or type(data['version']) is not int or data['version']!=4: error('K_OUTPUT')
        def named(n,kind):
            if type(n) is not dict or set(n)!={'node','name','params'} or n['node']!=kind or type(n['name']) is not str or n['params']!=[]:error('K_OUTPUT')
            return n['name']
        found=[]
        def walk(n):
            if type(n) is not dict:error('K_OUTPUT')
            if n.get('node')=='KApply':
                if set(n)!={'node','label','arity','args'} or type(n['args']) is not list or type(n['arity']) is not int or n['arity']!=len(n['args']):error('K_OUTPUT')
                label=named(n['label'],'KLabel')
                if label=='<out>':
                    if n['arity']!=1:error('K_OUTPUT')
                    found.append(n['args'][0]);return
                if label not in ['<generatedTop>','<k>','<generatedCounter>']:error('K_OUTPUT')
                for child in n['args']:walk(child)
            elif n.get('node')=='KSequence':
                if set(n)!={'node','arity','items'} or type(n.get('arity')) is not int or n.get('arity')!=0 or n.get('items')!=[]:error('K_OUTPUT')
            elif n.get('node')=='KToken':
                if set(n)!={'node','sort','token'} or named(n['sort'],'KSort')!='Int' or n['token']!='0':error('K_OUTPUT')
            else:error('K_OUTPUT')
        walk(data['term'])
        if len(found)!=1:error('K_OUTPUT')
        out=found[0]
        if set(out)!={'node','label','arity','args'} or out['node']!='KApply' or type(out['arity']) is not int or out['arity']!=len(out['args']):error('K_OUTPUT')
        def tok(n,sort):
            if set(n)!={'node','sort','token'} or n['node']!='KToken' or named(n['sort'],'KSort')!=sort or type(n['token']) is not str:error('K_OUTPUT')
            return json.loads(n['token']) if sort=='String' else n['token']
        label=named(out['label'],'KLabel')
        args=out['args']
        if not args or tok(args[0],'String')!=digest(p):error('K_OUTPUT_BINDING')
        if label=='rejected' and len(args)==3:
            code=tok(args[1],'String');idx=tok(args[2],'Int')
            if idx not in ['-1','0','1'] or code not in REJECTIONS[int(idx)]:error('K_OUTPUT')
            return {'status':'Rejected','code':code,'actionIndex':None if idx=='-1' else int(idx)}
        if label!='prepared' or len(args)!=16:error('K_OUTPUT')
        nums={}
        for idx in list(range(1,11))+[12,13,14]:
            v=tok(args[idx],'Int');uint(v);nums[idx]=v
        status=tok(args[11],'String');receiver=tok(args[15],'Int')
        if status not in ['Outstanding','Settled'] or receiver not in ['-1','0','1']:error('K_OUTPUT')
        transfer=p['actions'][0]
        expected_receiver=next((str(i) for i,b in enumerate(p['state']['balances']) if b['party']==transfer['to'] and b['asset']==transfer['asset']),'-1')
        if receiver!=expected_receiver:error('K_OUTPUT_BINDING')
        if receiver!='-1' and nums[3]!='0':error('K_OUTPUT')
        post=copy.deepcopy(p['state']);t,r=p['actions'];o=post['obligations'][0]
        for idx in [0,1]:post['balances'][idx]['amount']=nums[idx+1]
        if receiver=='-1':post['balances'].append({'party':t['to'],'asset':t['asset'],'amount':nums[3]})
        post['allowances'][0].update(remaining=nums[4],spent=nums[5])
        post['work'].update(remaining=nums[6],spent=nums[7])
        o.update(principal=nums[8],accrued=nums[9],outstanding=nums[10],status=status)
        post['usedTransferIds'].append(t['id']);post['usedAllocationIds'].append(r['allocationId'])
        effect={'kind':'Repayment',**{k:r[k] for k in ['allocationId','transferId','obligationId','payer']},'creditor':o['creditor'],'denomination':o['denomination'],'settlementAsset':o['settlementAsset'],'nominalAmount':r['nominalAmount'],'settlementAmount':nums[12],'principalDischarged':nums[13],'accruedDischarged':nums[14],'remainingOutstanding':nums[10]}
        return {'status':'Prepared','schemaVersion':VERSION,'post':post,'effects':[copy.deepcopy(t),effect]}
    except (KeyError,TypeError,ValueError,RecursionError) as exc:
        if isinstance(exc,CodecError):raise
        error('K_OUTPUT')
