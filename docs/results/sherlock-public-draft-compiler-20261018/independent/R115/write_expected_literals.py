"""Fixed manually derived rows/spans; no parser/checker/bridge/owner imports/calls."""
from pathlib import Path
import json,hashlib
P=Path(__file__).resolve().parent

def write(name,value):(P/name).write_text(json.dumps(value,indent=2)+'\n')
def span(start,end):return {'start':start,'end':end}
def loc(owner,path):return {'function_index':None if owner is None else str(owner),'child_path':[str(i) for i in path]}
def sl(start=0,length=0):return {'start':str(start if length else 0),'len':str(length)}
def row(tag,**fields):return {tag:fields}
# Every expression row and semantic child reference is explicitly transcribed.
chain_rows=[row('Call',function_name='1',args=sl(0,1)),row('Binary',op='Cons',left='2',right='3'),row('Literal',constant='0'),row('Binary',op='Cons',left='4',right='5'),row('Literal',constant='1'),row('Literal',constant='2'),row('Match',scrutinee='7',arms=sl(0,2)),row('Var',binder='3'),row('Literal',constant='3'),row('Binary',op='Add',left='10',right='11'),row('Var',binder='0'),row('Call',function_name='1',args=sl(1,1)),row('Var',binder='2')]
branch_rows=[row('Call',function_name='0',args=sl(0,1)),row('Binary',op='Branch',left='2',right='4'),row('Unary',op='Leaf',operand='3'),row('Literal',constant='0'),row('Binary',op='Branch',left='5',right='7'),row('Unary',op='Leaf',operand='6'),row('Literal',constant='1'),row('Unary',op='Leaf',operand='8'),row('Literal',constant='2'),row('Match',scrutinee='10',arms=sl(0,2)),row('Var',binder='4'),row('Var',binder='2'),row('Binary',op='Add',left='13',right='15'),row('Call',function_name='0',args=sl(1,1)),row('Var',binder='1'),row('Call',function_name='0',args=sl(2,1)),row('Var',binder='3')]
# (function owner, child path, kind, start,end, exact source bytes, independently
# checked sort). This is fixed provenance data, not a parsed/evaluated AST.
chain_sites=[
(None,[],'call',149,173,'sum(cons(2,cons(5,nil)))','UInt'),
(None,[0],'cons',153,172,'cons(2,cons(5,nil))','List'),
(None,[0,0],'uint',158,159,'2','UInt'),
(None,[0,1],'cons',160,171,'cons(5,nil)','List'),
(None,[0,1,0],'uint',165,166,'5','UInt'),
(None,[0,1,1],'nil',167,170,'nil','List'),
(0,[],'match',90,136,'match xs { nil => 0, cons(h,t) => h + sum(t) }','UInt'),
(0,[0],'var',96,98,'xs','List'),
(0,[1],'uint',108,109,'0','UInt'),
(0,[2],'add',124,134,'h + sum(t)','UInt'),
(0,[2,0],'var',124,125,'h','UInt'),
(0,[2,1],'call',128,134,'sum(t)','UInt'),
(0,[2,1,0],'var',132,133,'t','List')]
branch_sites=[
(None,[],'call',162,207,'fold(branch(leaf(4),branch(leaf(1),leaf(3))))','UInt'),
(None,[0],'branch',167,206,'branch(leaf(4),branch(leaf(1),leaf(3)))','Tree'),
(None,[0,0],'leaf',174,181,'leaf(4)','Tree'),
(None,[0,0,0],'uint',179,180,'4','UInt'),
(None,[0,1],'branch',182,205,'branch(leaf(1),leaf(3))','Tree'),
(None,[0,1,0],'leaf',189,196,'leaf(1)','Tree'),
(None,[0,1,0,0],'uint',194,195,'1','UInt'),
(None,[0,1,1],'leaf',197,204,'leaf(3)','Tree'),
(None,[0,1,1,0],'uint',202,203,'3','UInt'),
(0,[],'match',91,149,'match t { leaf(n) => n, branch(l,r) => fold(l) + fold(r) }','UInt'),
(0,[0],'var',97,98,'t','Tree'),
(0,[1],'var',112,113,'n','UInt'),
(0,[2],'add',130,147,'fold(l) + fold(r)','UInt'),
(0,[2,0],'call',130,137,'fold(l)','UInt'),
(0,[2,0,0],'var',135,136,'l','Tree'),
(0,[2,1],'call',140,147,'fold(r)','UInt'),
(0,[2,1,0],'var',145,146,'r','Tree')]

def base_view(rows,sites,names,function_name,param_name,param_sort,body,children,arms,patterns,constants):
    paths=[];occ=[]
    for owner,path,*_ in sites:
        start=len(paths);paths.extend(str(x) for x in path)
        occ.append({'owner_is_function':owner is not None,'function':'0','path':sl(start,len(path))})
    return {'entry':'0','expressions':rows,'occurrences':occ,'occurrence_paths':paths,'children':[str(i) for i in children],'functions':[{'function_name':str(names.index(function_name)),'params':sl(0,1),'return_type':'UInt','body':str(body)}],'parameters':[{'binder':str(names.index(param_name)),'sort':param_sort}],'arms':arms,'patterns':patterns,'constants':constants,'constant_fields':[],'names':names}
chain_view=base_view(chain_rows,chain_sites,['h','sum','t','xs'],'sum','xs','List',6,[1,12],[{'pattern':'0','body':'8'},{'pattern':'1','body':'9'}],['PNil',row('PCons',x='0',y='2')],[row('U128',marker_free=True,n='2'),row('U128',marker_free=True,n='5'),row('Nil',marker_free=True),row('U128',marker_free=True,n='0')])
branch_view=base_view(branch_rows,branch_sites,['fold','l','n','r','t'],'fold','t','Tree',9,[1,14,16],[{'pattern':'0','body':'11'},{'pattern':'1','body':'12'}],[row('PLeaf',x='2'),row('PBranch',x='1',y='3')],[row('U128',marker_free=True,n='4'),row('U128',marker_free=True,n='1'),row('U128',marker_free=True,n='3')])

def source_preparation(filename,program_name,function_name,param_name,param_sort,sites,fn_span,fn_name_span,param_span,param_type_span,return_span,program_span,pattern_specs,view):
    data=(P/'inputs'/filename).read_bytes()
    digest=hashlib.sha256(data).hexdigest()
    assert data[9:36]==b'"moriarty-sherlock-draft/1"'
    assert data[slice(*program_span)].decode()==program_name
    assert data[slice(*fn_name_span)].decode()==function_name
    assert data[slice(*param_span)].decode()==param_name
    assert data[slice(*param_type_span)].decode()==param_sort
    assert data[slice(*return_span)]==b'UInt'
    exprs=[];joins=[]
    for i,(owner,path,kind,start,end,text,sort) in enumerate(sites):
        assert data[start:end].decode()==text,(filename,i,start,end,text,data[start:end])
        exprs.append({'source':loc(owner,path),'span':span(start,end),'kind':kind})
        joins.append({'expression_id':str(i),'source':loc(owner,path),'kind':kind,'sort':sort,'span':span(start,end),'owner_occurrence':view['occurrences'][i]})
    patterns=[]
    for arm,start,end,text,binders in pattern_specs:
        assert data[start:end].decode()==text
        binder_records=[]
        for name,a,b in binders:
            assert data[a:b].decode()==name
            binder_records.append({'name':name,'span':span(a,b)})
        patterns.append({'match_source':loc(0,[]),'arm_index':str(arm),'span':span(start,end),'binders':binder_records})
    provenance={'language_span':span(9,36),'program_name_span':span(*program_span),'functions':[{'index':'0','span':span(*fn_span),'name_span':span(*fn_name_span),'parameters':[{'binder':{'name':param_name,'span':span(*param_span)},'type_span':span(*param_type_span)}],'return_type_span':span(*return_span)}],'expressions':exprs,'patterns':patterns}
    source={'language':'moriarty-sherlock-draft/1','semantics':'moriarty-sherlock-dedicated/1','program':program_name,'selection':{'kind':'entry'},'source_digest':digest}
    prep={'kind':'dedicated-source','functions':[{'name':function_name,'parameters':[{'name':param_name,'sort':param_sort}],'return_sort':'UInt'}],'provenance':provenance,'types':{'function_returns':['UInt'],'entry_type':'UInt'},'owner_mapping':{'program_view_pointer':'/admission/program_view','source_order':'entry-preorder-then-original-function-body-order','join':'source function_index/child_path to owner occurrence owner_is_function/function/path','correspondence':'unestablished-until-independent-comparison'}}
    return {'source_digest':digest,'source':source,'preparation':prep,'program_view':view},joins
chain,chain_join=source_preparation('chain-total.mori','ChainTotal','sum','xs','List',chain_sites,(61,140),(64,67),(68,70),(72,76),(79,83),(46,56),[(0,101,104,'nil',[]),(1,111,120,'cons(h,t)',[('h',116,117),('t',118,119)])],chain_view)
branch,branch_join=source_preparation('branch-total.mori','BranchTotal','fold','t','Tree',branch_sites,(62,153),(65,69),(70,71),(73,77),(80,84),(46,57),[(0,101,108,'leaf(n)',[('n',106,107)]),(1,115,126,'branch(l,r)',[('l',122,123),('r',124,125)])],branch_view)
assert len(chain_view['occurrence_paths'])==21 and len(branch_view['occurrence_paths'])==35
assert len(chain_view)==len(branch_view)==12
assert 2+(5+0)==7 and 4+(1+3)==8
write('EXPECTED.json',{'chain_total':chain,'branch_total':branch})
write('TYPE-AND-JOIN-ACCOUNTING.json',{'chain_total':chain_join,'branch_total':branch_join,'independent_mathematical_values':{'chain_total':{'UInt':'7'},'branch_total':{'UInt':'8'}},'runtime':'UNRUN; these mathematical values are not B execution/work/summary expectations','join_status':'Every source occurrence and owner row explicitly enumerated; observed correspondence unestablished until independent actual comparison'})
write('LITERAL-CHECKS.json',{'status':'PASS_MANUAL_LITERAL_SOURCE_SLICES_TABLE_COUNTS_AND_ARITHMETIC','chain':{'expressions':13,'paths':21,'call_children':2,'patterns':2,'arms':2,'constants':4,'names':4},'branch':{'expressions':17,'paths':35,'call_children':3,'patterns':2,'arms':2,'constants':3,'names':5},'all_program_view_fields':12,'source_span_slice_checks':'Every expression/pattern/binder/type/name/header range equals explicitly specified source bytes','source_or_parser_or_checker_or_bridge_or_owner_or_compiler_executed':False,'method':'Inspected Pythonstdlib literal transcription and file-byte slicing/hash/integer checks only; no subject AST evaluation or table generator called'})
print('PASS_MANUAL_LITERAL_SOURCE_SLICES_TABLE_COUNTS_AND_ARITHMETIC')
