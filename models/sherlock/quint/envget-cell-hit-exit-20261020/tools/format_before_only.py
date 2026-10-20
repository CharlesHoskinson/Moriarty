from pathlib import Path
import json,re,hashlib,os
D=Path(__file__).parent
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
P=R/'k-EnvGetCell-hit-EnvGetExit-independent-oracle-20261020/SAFE/RELEASED-ACTUAL-ONLY-R5/ACTUAL-WINDOW-BEFORE.json'
T=R/'dedicated-quint-20261018/batch41-EnvGetCell-EnvGetExit-owner-positive-20261020/SAFE/declarations'
PINS={'q41EnvGetOwnerTypes.qnt':'7266295c7e67589251ae62e45c2fee8309495192078889e2d8d37ab30e57e7ce','c6PositiveHostTypes.qnt':'c17c3edfe85d6bc6971b093bb097f5911f4341638f6144e98e46bda1ae5d4b73','c6PositiveMetaTypes.qnt':'3d93ef1acf2ef1cc7dfdf68f24279736710cbbaa3fcdc8915e21b74203396509','c6PositiveProgramTypes.qnt':'99457551a4dcb5f899c1c5dfb58eda39ca81bf8508799adaf0210576f7d4cb07','c6PositiveContextTypes.qnt':'601ef02d3f508ff8b0a8de94cfce92772849678ca6ed645a66d01d9663c48370','c6PositiveStateTypes.qnt':'1336bbc0d38798bd6c5e4ea29b96073c7100d4e03a2c0c591e2cae880f825f06'}
sha=lambda b:hashlib.sha256(b).hexdigest()
raw=P.read_bytes();assert len(raw)==6362 and sha(raw)=='b6faa318017cc3205a663201d745424fe50397c591cf76b10d0a23f1f5b0c62d','released-before identity mismatch';native=json.loads(raw)
TOK=re.compile(r'\s+|//[^\n]*|"(?:\\.|[^"\\])*"|-?[0-9]+|[A-Za-z_][A-Za-z_0-9]*|[{}\[\](),:|=.]')
def lex(s):
 out=[];end=0
 for m in TOK.finditer(s):
  assert not s[end:m.start()].strip(),'non-DATA syntax';t=m.group();end=m.end()
  if not t.isspace() and not t.startswith('//'):out.append(t)
 assert not s[end:].strip(),'non-DATA tail';return out
class Reader:
 def __init__(self,t):self.t=t;self.i=0
 def peek(self):return self.t[self.i] if self.i<len(self.t) else None
 def pop(self,want=None):
  assert self.i<len(self.t),'truncated literal';v=self.t[self.i];self.i+=1;assert want is None or v==want,'literal syntax mismatch';return v
 def atom(self):
  if self.peek()=='{':
   self.pop();fields={}
   while self.peek()!='}':
    key=self.pop();self.pop(':');assert key not in fields;fields[key]=self.atom()
    if self.peek()!=',':break
    self.pop(',')
   self.pop('}');return ('record',fields)
  name=self.pop()
  if name=='List':self.pop('[');ty=self.atom();self.pop(']');return ('list',ty)
  return ('ref',name)
 def declaration(self):
  bar=self.peek()=='|'
  if bar:self.pop('|')
  if self.peek() in ['{','List']:return self.atom()
  name=self.pop();arg=None
  if self.peek()=='(':self.pop();arg=self.atom();self.pop(')')
  if arg is None and not bar and self.peek()!='|':return ('ref',name)
  variants={name:arg}
  while self.peek()=='|':
   self.pop();name=self.pop();arg=None
   if self.peek()=='(':self.pop();arg=self.atom();self.pop(')')
   assert name not in variants;variants[name]=arg
  return ('sum',variants)
 def value(self):
  t=self.peek()
  if t=='{':
   self.pop();v={}
   while self.peek()!='}':
    k=self.pop();self.pop(':');assert k not in v;v[k]=self.value()
    if self.peek()!=',':break
    self.pop(',')
   self.pop('}');return v
  if t=='List':
   self.pop();self.pop('(');v=[]
   while self.peek()!=')':
    v.append(self.value())
    if self.peek()!=',':break
    self.pop(',')
   self.pop(')');return v
  t=self.pop()
  if t.startswith('"'):return json.loads(t)
  if re.fullmatch('-?[0-9]+',t):return int(t)
  if t in ['true','false']:return t=='true'
  arg=None;has=False
  if self.peek()=='(':self.pop();arg=self.value();self.pop(')');has=True
  return ('variant',t,has,arg)
types={};decls=[]
for name,pin in PINS.items():
 p=T/name;b=p.read_bytes();assert sha(b)==pin,'public declaration identity mismatch';decls.append({'path':str(p),'sha256':pin,'bytes':len(b)})
 text=re.sub(r'//[^\n]*','',b.decode());matches=list(re.finditer(r'\btype\s+([A-Za-z_][A-Za-z_0-9]*)\s*=',text))
 for i,m in enumerate(matches):
  chunk=text[m.end():matches[i+1].start() if i+1<len(matches) else text.rfind('}')];r=Reader(lex(chunk));value=r.declaration();assert r.peek() is None;assert m.group(1) not in types;types[m.group(1)]=value
options={'C6Env','C6Function','C6HIndex','C6HType','C6HOptRef','C6HOptHelper','C6HOptResult','C6MOptionPair','C6MOptionWidths','C6POption','Q41OptText','Q41OptInt','Q41OptPhase'}
prefixes={'C6Ref':'C6','C6Binary':'C6','C6Unary':'C6','C6Frame':'C6','C6Binding':'C6Binding','C6Matching':'C6','C6Primitive':'C6Primitive','C6Control':'C6','C6Exhaust':'C6','C6Outcome':'C6','C6HSiteTag':'C6HSiteTag','C6HScalarMode':'C6HScalarMode','C6HScalarPhase':'C6HScalarPhase','C6HNodeKind':'C6HNodeKind','C6HBoolSelector':'C6HBoolSelector','C6HTriStatus':'C6HTriStatus','C6HBindPhase':'C6HBindPhase','C6HBindStatus':'C6HBindStatus','C6HAppendPhase':'C6HAppendPhase','C6HMatchPhase':'C6HMatchPhase','C6HPlanPhase':'C6HPlanPhase','C6HPlanAfterWalk':'C6HPlanAfterWalk','C6HBuildPhase':'C6HBuildPhase','C6HPatternTag':'C6HPatternTag','C6HTaskVisitC':'C6HTaskVisitC','C6HStaged':'C6HStaged','C6HHelper':'C6H','C6HResult':'C6HResult','C6HTask':'C6H','C6HDataRow':'C6H','C6PBinary':'C6P','C6PUnary':'C6P','C6PExpr':'C6PE','C6PPattern':'C6P','C6PPlan':'C6P','C6PConstant':'C6PK','C6MGrammar':'C6M','C6MFinance':'C6M','C6MDigest':'C6M','C6MRunInt':'C6M','Q41Phase':'Q41','Q41Locator':'Q41'}
renames={'C6MFinanceAbsent':'Absent','C6PKUInt':'U128','C6PKOutOfRange':'U128OutOfRange','C6PWildcard':'WildcardExtensionReserved','C6PPConstructor':'ConstructorExtensionReserved','C6NoMatch':'NoMatch','C6Matched':'Matched'}
def tagname(ty,tag):return renames.get(tag,tag.removeprefix(prefixes[ty]))
def byte_text(xs):
 assert all(type(z) is int and 0<=z<=255 for z in xs);return bytes(xs).decode('utf-8')
def bindfields(fields,v):assert type(v) is dict and set(v)==set(fields),'record membership mismatch'
def forward(t,v,typename=None):
 kind,x=t
 if kind=='ref':
  if x in ['str','bool','int']:
   assert type(v) is {'str':str,'bool':bool,'int':int}[x],'primitive type mismatch';return v
  if x=='C6MBytePair':assert type(v) is list and len(v)==2 and all(type(z) is str for z in v);return {'first':list(v[0].encode()),'second':list(v[1].encode())}
  if x=='C6MWidthEntry':assert type(v) is list and len(v)==2 and type(v[0]) is str and type(v[1]) is int;return {'name':list(v[0].encode()),'width':v[1]}
  if x=='C6PProgram':
   fields=types[x][1];bindfields(fields,v);return {k:([list(z.encode()) for z in a] if k=='names' else forward(fields[k],a)) for k,a in v.items()}
  if x=='C6PDecl':bindfields(types[x][1],v);return {'name':list(v['name'].encode()),'parameters':[list(z.encode()) for z in v['parameters']],'ctors':forward(types[x][1]['ctors'],v['ctors'])}
  if x=='C6PCtorDecl':bindfields(types[x][1],v);return {'name':list(v['name'].encode()),'fields':forward(types[x][1]['fields'],v['fields'])}
  if x=='C6PDeclField':bindfields(types[x][1],v);return {'name':list(v['name'].encode()),'ty':flatten_type(v['ty'])}
  return forward(types[x],v,x)
 if kind=='record':bindfields(x,v);return {k:forward(ty,v[k]) for k,ty in x.items()}
 if kind=='list':assert type(v) is list;return [forward(x,z) for z in v]
 if kind=='sum':
  if typename in options:
   if v is None:
    tags=[tag for tag,ty in x.items() if ty is None];assert len(tags)==1;return ('variant',tags[0],False,None)
   assert type(v) is dict and set(v)=={'Some'};tags=[tag for tag,ty in x.items() if ty is not None];assert len(tags)==1;tag=tags[0];return ('variant',tag,True,forward(x[tag],v['Some']))
  assert typename in prefixes
  if type(v) is str:
   tags=[tag for tag,ty in x.items() if ty is None and tagname(typename,tag)==v];assert len(tags)==1;return ('variant',tags[0],False,None)
  assert type(v) is dict and len(v)==1;name,arg=next(iter(v.items()));tags=[tag for tag,ty in x.items() if ty is not None and tagname(typename,tag)==name];assert len(tags)==1;tag=tags[0];return ('variant',tag,True,forward(x[tag],arg))
 raise AssertionError('unsupported declaration kind')
def inverse(t,v,typename=None):
 kind,x=t
 if kind=='ref':
  if x in ['str','bool','int']:assert type(v) is {'str':str,'bool':bool,'int':int}[x];return v
  if x=='C6MBytePair':return [byte_text(v['first']),byte_text(v['second'])]
  if x=='C6MWidthEntry':return [byte_text(v['name']),v['width']]
  if x=='C6PProgram':
   fields=types[x][1];bindfields(fields,v);return {k:([byte_text(z) for z in a] if k=='names' else inverse(fields[k],a)) for k,a in v.items()}
  if x=='C6PDecl':return {'name':byte_text(v['name']),'parameters':[byte_text(z) for z in v['parameters']],'ctors':inverse(types[x][1]['ctors'],v['ctors'])}
  if x=='C6PCtorDecl':return {'name':byte_text(v['name']),'fields':inverse(types[x][1]['fields'],v['fields'])}
  if x=='C6PDeclField':return {'name':byte_text(v['name']),'ty':unflatten_type(v['ty'])}
  return inverse(types[x],v,x)
 if kind=='record':bindfields(x,v);return {k:inverse(ty,v[k]) for k,ty in x.items()}
 if kind=='list':assert type(v) is list;return [inverse(x,z) for z in v]
 if kind=='sum':
  assert type(v) is tuple and v[0]=='variant';_,tag,has,arg=v;assert tag in x and has==(x[tag] is not None),'constructor membership/arity mismatch'
  if typename in options:return {'Some':inverse(x[tag],arg)} if has else None
  name=tagname(typename,tag);return {name:inverse(x[tag],arg)} if has else name
 raise AssertionError('unsupported declaration kind')
def flatten_type(v):
 if type(v) is str:assert v in ['UInt','Bool','List','Tree'];return [('variant','C6PT'+v,False,None)]
 assert type(v) is dict and len(v)==1;name,arg=next(iter(v.items()))
 if name in ['Named','Param']:assert set(arg)=={'name'};return [('variant','C6PT'+name,True,{'name':list(arg['name'].encode())})]
 assert name=='Apply' and set(arg)=={'name','args'};out=[('variant','C6PTApply',True,{'name':list(arg['name'].encode()),'arity':len(arg['args'])})]
 for a in arg['args']:out.extend(flatten_type(a))
 return out
def unflatten_type(values):
 def one(i):
  _,tag,has,arg=values[i];i+=1;name=tag.removeprefix('C6PT')
  if name in ['UInt','Bool','List','Tree']:return name,i
  if name in ['Named','Param']:return {name:{'name':byte_text(arg['name'])}},i
  assert name=='Apply';args=[]
  for _ in range(arg['arity']):a,i=one(i);args.append(a)
  return {'Apply':{'name':byte_text(arg['name']),'args':args}},i
 out,n=one(0);assert n==len(values);return out
def literal(v,depth=1):
 if isinstance(v,tuple):return v[1]+('('+literal(v[3],depth)+')' if v[2] else '')
 if type(v) is str:return json.dumps(v,ensure_ascii=False)
 if type(v) is bool:return 'true' if v else 'false'
 if type(v) is int:return str(v)
 if type(v) is list:return 'List('+','.join(literal(z,depth) for z in v)+')'
 assert type(v) is dict;pad='  '*depth;return '{\n'+',\n'.join(pad+k+': '+literal(z,depth+1) for k,z in v.items())+'\n'+'  '*(depth-1)+'}'
value=forward(('ref','C6WholeHost'),native)
imports=''.join('  import '+Path(name).stem+'.* from "./'+Path(name).stem+'"\n' for name in PINS)
head='module q41ActualOwnerBefore {\n'+imports+'\n  pure val actualBefore:C6WholeHost = '
module=head+literal(value,2)+'\n}\n'
# Independently parse the serialized literal (not an evaluator) before inverse.
r=Reader(lex(module[len(head):]));decoded=r.value();r.pop('}');assert r.peek() is None;inv=inverse(('ref','C6WholeHost'),decoded)
def exact(a,b):
 if type(a) is not type(b):return False
 if type(a) is dict:return set(a)==set(b) and all(exact(a[k],b[k]) for k in a)
 if type(a) is list:return len(a)==len(b) and all(exact(x,y) for x,y in zip(a,b))
 return a==b
assert exact(inv,native),'strict entire native inverse mismatch'
O=D/'source';O.mkdir(exist_ok=False)
for name in PINS:
 b=(T/name).read_bytes();q=O/name;q.write_bytes(b);os.chmod(q,0o400)
q=O/'q41ActualOwnerBefore.qnt';b=module.encode();q.write_bytes(b);os.chmod(q,0o400)
receipt={'status':'ACTUAL_BEFORE_ONLY_DATA_SERIALIZATION_AND_STRICT_INVERSE_PASS','rootOverride':'One released before JSON only; prior blanket actual-read prohibition overridden solely for this file.','nativeInput':{'path':str(P),'bytes':len(raw),'sha256':sha(raw)},'publicDATAOutput':{'path':str(q),'bytes':len(b),'sha256':sha(b),'module':'q41ActualOwnerBefore','export':'pure val actualBefore:C6WholeHost'},'declarations':decls,'strictInverse':{'serializedLiteralIndependentlyParsed':True,'fullNativeRecordKeysTypesValuesAndListOrderExact':True,'intVersusBoolPreserved':True,'UTF8BytesReversible':True,'noFieldsOmitted':True},'priorRoleExposure':'Same actor previously validated privately owning expected-DATA field/type/case/inverse; retain correlation. Not a fresh blind or independent semantic vote. Historical identity-only raw-read qualification remains in prior SAFE receipt; no historical body reopened here.','inputsExcluded':{'expectedOrAfterOrFullActualArtifacts':True,'privateOracleOrDiagnosticBodies':True,'producerModelCaptureKQImplementations':True},'noExpectedAsInput':True,'noSemanticFittingOrOwnerBehaviorReconstruction':True,'subjectProviderNativeGitSharedOrCanonicalActions':False,'noModelOrResultComparison':True,'sourceOnlyNoExecutionGrant':True}
rp=D/'SAFE-FORMAT-HANDOFF.json';rb=(json.dumps(receipt,indent=2)+'\n').encode();rp.write_bytes(rb);os.chmod(rp,0o400)
print(json.dumps({'output':receipt['publicDATAOutput'],'handoff':{'path':str(rp),'bytes':len(rb),'sha256':sha(rb)},'strictInversePass':True}))
