from pathlib import Path
import json,re,hashlib,os
D=Path(__file__).parent
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
I=R/'dedicated-quint-20261018/batch41-EnvGetCell-EnvGetExit-owner-positive-20261020/actual-before-runtime01/proposed-itf/actual_owner_window_0.itf.json'
T=R/'dedicated-quint-20261018/batch41-EnvGetCell-EnvGetExit-owner-positive-20261020/SAFE/declarations'
SCHEMA=Path('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/itf.d.ts')
OBS=R/'k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/revision02-complete-public-declarations/SAFE/SAFE-OBSERVATION-CONTRACT.json'
sha=lambda b:hashlib.sha256(b).hexdigest()
PINS={'q41EnvGetOwnerTypes.qnt': '7266295c7e67589251ae62e45c2fee8309495192078889e2d8d37ab30e57e7ce', 'c6PositiveHostTypes.qnt': 'c17c3edfe85d6bc6971b093bb097f5911f4341638f6144e98e46bda1ae5d4b73', 'c6PositiveMetaTypes.qnt': '3d93ef1acf2ef1cc7dfdf68f24279736710cbbaa3fcdc8915e21b74203396509', 'c6PositiveProgramTypes.qnt': '99457551a4dcb5f899c1c5dfb58eda39ca81bf8508799adaf0210576f7d4cb07', 'c6PositiveContextTypes.qnt': '601ef02d3f508ff8b0a8de94cfce92772849678ca6ed645a66d01d9663c48370', 'c6PositiveStateTypes.qnt': '1336bbc0d38798bd6c5e4ea29b96073c7100d4e03a2c0c591e2cae880f825f06'}
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

# ITF type-directed DATA extraction only; no Quint runtime/producer/model is imported.
def decode_itf(t,v):
 kind,x=t
 if kind=='ref':
  if x=='int':
   assert type(v) is dict and set(v)=={'#bigint'} and type(v['#bigint']) is str and re.fullmatch(r'-?(0|[1-9][0-9]*)',v['#bigint']),'GAP: unsupported/noncanonical ITF integer representation'
   return int(v['#bigint'])
  if x=='bool':assert type(v) is bool,'GAP: ITF bool type';return v
  if x=='str':assert type(v) is str,'GAP: ITF str type';return v
  assert x in types,'GAP: undeclared DATA type';return decode_itf(types[x],v)
 if kind=='record':
  assert type(v) is dict and set(v)==set(x),'GAP: complete record field membership'
  return {k:decode_itf(ty,v[k]) for k,ty in x.items()}
 if kind=='list':assert type(v) is list,'GAP: ITF List representation';return [decode_itf(x,a) for a in v]
 if kind=='sum':
  assert type(v) is dict and set(v)=={'tag','value'} and type(v['tag']) is str and v['tag'] in x,'GAP: DATA constructor membership'
  tag=v['tag'];ty=x[tag]
  if ty is None:assert v['value']=={'#tup':[]},'GAP: unit constructor payload';return ('variant',tag,False,None)
  return ('variant',tag,True,decode_itf(ty,v['value']))
 raise AssertionError('GAP: unsupported DATA declaration')
def encode_itf(t,v):
 kind,x=t
 if kind=='ref':
  if x=='int':assert type(v) is int;return {'#bigint':str(v)}
  if x=='bool':assert type(v) is bool;return v
  if x=='str':assert type(v) is str;return v
  return encode_itf(types[x],v)
 if kind=='record':bindfields(x,v);return {k:encode_itf(ty,v[k]) for k,ty in x.items()}
 if kind=='list':assert type(v) is list;return [encode_itf(x,a) for a in v]
 if kind=='sum':
  _,tag,has,arg=v;assert tag in x and has==(x[tag] is not None)
  return {'tag':tag,'value':encode_itf(x[tag],arg) if has else {'#tup':[]}}
 raise AssertionError('unsupported DATA declaration')
def exact(a,b):
 if type(a) is not type(b):return False
 if type(a) is dict:return set(a)==set(b) and all(exact(a[k],b[k]) for k in a)
 if type(a) is list:return len(a)==len(b) and all(exact(x,y) for x,y in zip(a,b))
 return a==b
F=json.loads((D/'SOURCE-AND-INPUT-FREEZE.json').read_text())
for pin in F['files']:
 p=Path(pin['path']);b=p.read_bytes();assert len(b)==pin['bytes'] and sha(b)==pin['sha256'],'frozen source/input changed'
raw=I.read_bytes();assert len(raw)==42853 and sha(raw)=='10394b0e4cf9ddf9bb26d20026d0a700b9a67aff4fa6d619c15e27044e03a9d6'
trace=json.loads(raw);assert trace['vars']==['machine'] and len(trace['states'])==2,'GAP: selected public trace schema'
state=trace['states'][1];assert set(state)=={'#meta','machine'},'GAP: final trace state fields'
machine=state['machine'];assert set(machine)=={'input_before','computed_window','computed_once'},'GAP: selected machine API'
assert type(machine['computed_once']) is bool and machine['computed_once'] is True,'GAP: final computation status'
wrapped=machine['computed_window'];assert set(wrapped)=={'tag','value'} and wrapped['tag']=='Q41ComputedWindow','GAP: selected computed-window wrapper'
window=wrapped['value'];assert set(window)=={'before','after_cell','after_exit','events'},'GAP: complete selected Q41Window'
# Public contract/declarations establish structural arities only; no behavior or expected values inferred.
contract=json.loads(OBS.read_text());assert set(contract['window_fields'])==set(types['Q41Window'][1]);assert set(contract['admission_fields'])==set(types['C6HostAdmission'][1]);assert set(contract['image_fields'])==set(types['C6HostImage'][1]);assert set(contract['layout_fields'])==set(types['C6MLayout'][1])
typed=decode_itf(('ref','Q41Window'),window)
native_window=inverse(('ref','Q41Window'),typed)
# Strict inverse returns the ENTIRE selected ITF window, not metadata or another cohort.
retyped=forward(('ref','Q41Window'),native_window);roundtrip=encode_itf(('ref','Q41Window'),retyped);assert exact(roundtrip,window),'strict whole selected-window inverse mismatch'
output=D/'COMPUTED-QUINT-NATIVE-WINDOW.json';payload=(json.dumps(native_window,ensure_ascii=False,indent=2)+'\n').encode();output.write_bytes(payload);os.chmod(output,0o400)
inverse_output=D/'STRICT-INVERSE-SELECTED-ITF-WINDOW.json';inverse_bytes=(json.dumps(roundtrip,ensure_ascii=False,indent=2)+'\n').encode();inverse_output.write_bytes(inverse_bytes);os.chmod(inverse_output,0o400)
canon=lambda x:json.dumps(x,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode()
proof={'status':'COMPLETE_SELECTED_WINDOW_PROJECTION_AND_STRICT_INVERSE_PASS','inputITFSha256':sha(raw),'selectedPath':'states[1].machine.computed_window.value','wholeSelectedTypedWindowCanonicalSha256':sha(canon(window)),'wholeInverseTypedWindowCanonicalSha256':sha(canon(roundtrip)),'wholeSelectedDataEqualityWithTypesKeysValuesAndListOrder':True,'nativeWindow':{'path':str(output),'bytes':len(payload),'sha256':sha(payload)},'strictInverseWindow':{'path':str(inverse_output),'bytes':len(inverse_bytes),'sha256':sha(inverse_bytes)},'structuralPublicArities':{'hostFieldsInWindow':['before','after_cell','after_exit'],'eventDeclarationFields':list(types['Q41StepEvent'][1]),'programViewFields':list(types['C6PProgram'][1]),'imageFields':list(types['C6HostImage'][1])},'scope':'Computed Quint final selected window only, not genuine Rust after; no full native root/prefix or Rust/expected comparison. Outer ITF metadata and initial NoWindow are preserved in immutable input, outside selected native window.','exactRepresentations':'Bool distinct Int; every int parsed from decimal #bigint with no float; constructor tag/arity exact; None=null and Some={Some:...} including zero; UTF8 byte projection strict inverse; no record fields/defaults omitted and all array ordering retained.','priorExposure':'Same FORMAT role previously saw expected-DATA structural transport and released actual-before; correlation retained, no fresh blind semantic vote. Earlier historical identity-only raw-read qualification retained; no historical body reopened.','inputIsolation':'Sole frozen computed ITF, six exact public DATA declarations, native observation contract and installed ITF declaration schema; no expected, genuine Rust-after/fullnative, private opinions/diagnostics, producer/model implementation inputs.','noSubjectProviderNativeGitPWOrSharedActions':True}
p=D/'STRICT-INVERSE-PROOF.json';b=(json.dumps(proof,indent=2)+'\n').encode();p.write_bytes(b);os.chmod(p,0o400)
post=[]
for pin in F['files']:
 p=Path(pin['path']);b=p.read_bytes();post.append({'path':str(p),'match':len(b)==pin['bytes'] and sha(b)==pin['sha256']})
assert all(x['match'] for x in post);(D/'POST-FORMAT-PIN-CHECK.json').write_text(json.dumps({'allMatch':True,'files':post},indent=2)+'\n')
print(json.dumps({'nativeWindow':proof['nativeWindow'],'strictInverseProof':{'path':str(D/'STRICT-INVERSE-PROOF.json'),'bytes':(D/'STRICT-INVERSE-PROOF.json').stat().st_size,'sha256':sha((D/'STRICT-INVERSE-PROOF.json').read_bytes())},'status':proof['status']}))
