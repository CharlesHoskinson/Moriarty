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

def literal(v,depth=1):
 if isinstance(v,tuple):return v[1]+('('+literal(v[3],depth)+')' if v[2] else '')
 if type(v) is str:return json.dumps(v,ensure_ascii=False)
 if type(v) is bool:return 'true' if v else 'false'
 if type(v) is int:return str(v)
 if type(v) is list:return 'List('+','.join(literal(z,depth) for z in v)+')'
 assert type(v) is dict;pad='  '*depth;return '{\n'+',\n'.join(pad+k+': '+literal(z,depth+1) for k,z in v.items())+'\n'+'  '*(depth-1)+'}'

def exact(a,b):
 if type(a) is not type(b):return False
 if type(a) is dict:return set(a)==set(b) and all(exact(a[k],b[k]) for k in a)
 if type(a) is list:return len(a)==len(b) and all(exact(x,y) for x,y in zip(a,b))
 return a==b
