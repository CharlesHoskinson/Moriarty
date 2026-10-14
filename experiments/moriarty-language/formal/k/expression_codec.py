"""Lossless canonical JSON transport for the expression K definition.

This module does not infer types, evaluate constructors, validate Sigma or supply
prechecked flags. Schema/request semantic rejection belongs to K. Only transport
syntax, canonical encoding and a finite envelope are checked here.
"""
import json

class CodecError(ValueError):pass

def _fail():raise CodecError('EXPRESSION_TRANSPORT')
def _object(pairs):
 out={}
 for k,v in pairs:
  if k in out:_fail()
  out[k]=v
 return out

def _canonical(value):
 out=[];pending=[('value',value)]
 while pending:
  kind,v=pending.pop()
  if kind=='text':out.append(v);continue
  if type(v) in (str,bool,int) or v is None:
   if type(v) is str:v.encode('utf-8')
   out.append(json.dumps(v,ensure_ascii=False,separators=(',',':'),allow_nan=False));continue
  if type(v) is list:
   out.append('[');pending.append(('text',']'));parts=[]
   for i,x in enumerate(v):
    if i:parts.append(('text',','))
    parts.append(('value',x))
   pending.extend(reversed(parts));continue
  if type(v) is dict:
   out.append('{');pending.append(('text','}'));parts=[]
   for i,k in enumerate(sorted(v)):
    if type(k) is not str:_fail()
    if i:parts.append(('text',','))
    parts.extend([('value',k),('text',':'),('value',v[k])])
   pending.extend(reversed(parts));continue
  _fail()
 return ''.join(out)

def _parse(text,allow_space=False):
 # Standard scalar string decoding is nonrecursive; container grammar is ours.
 decoder=json.JSONDecoder();i=0;stack=[];root=[]
 def attach(value):
  if not stack:
   if root:_fail()
   root.append(value);return
  frame=stack[-1]
  if frame[0]=='array' and frame[2] in ('first','value'):
   frame[1].append(value);frame[2]='comma';return
  if frame[0]=='object' and frame[2]=='value':
   frame[1][frame[3]]=value;frame[2]='comma';return
  _fail()
 while i<len(text):
  c=text[i]
  if c in ' \t\r\n':
   if not allow_space:_fail()
   i+=1;continue
  if stack:
   f=stack[-1]
   if c in '}]':
    if (c=='}' and f[0]!='object') or (c==']' and f[0]!='array') or f[2] not in ('first','comma'):_fail()
    stack.pop();i+=1;continue
   if f[2]=='comma':
    if c!=',':_fail()
    f[2]='key' if f[0]=='object' else 'value';i+=1;continue
   if f[2]=='colon':
    if c!=':':_fail()
    f[2]='value';i+=1;continue
   if f[0]=='object' and f[2] in ('first','key'):
    if c!='"':_fail()
    key,i=decoder.raw_decode(text,i)
    if key in f[1]:_fail()
    f[3]=key;f[2]='colon';continue
  if c=='{':
   v={};attach(v);stack.append(['object',v,'first',None]);i+=1
  elif c=='[':
   v=[];attach(v);stack.append(['array',v,'first',None]);i+=1
  elif c=='"':
   v,i=decoder.raw_decode(text,i);attach(v)
  else:
   end=i
   while end<len(text) and text[end] not in ',]} \t\r\n':end+=1
   token=text[i:end]
   if token=='true':v=True
   elif token=='false':v=False
   elif token=='null':v=None
   else:
    try:v=int(token)
    except ValueError:_fail()
    if str(v)!=token:_fail()
   attach(v);i=end
 if stack or len(root)!=1:_fail()
 return root[0]

def _apply(label,*args):return {'node':'KApply','label':{'node':'KLabel','name':label,'params':[]},'args':list(args),'arity':len(args)}
def _token(sort,value):return {'node':'KToken','sort':{'node':'KSort','name':sort,'params':[]},'token':value}
def _quote_k_string(value):
 # Pinned StringUtil.enquoteKString/getUnicodeEscape, Unicode scalar domain.
 if type(value) is not str:_fail()
 out=['"'];short={'"':'\\"','\\':'\\\\','\n':'\\n','\r':'\\r','\t':'\\t','\f':'\\f'}
 for char in value:
  n=ord(char)
  if 0xd800<=n<=0xdfff:_fail()
  if char in short:out.append(short[char])
  elif 32<=n<127:out.append(char)
  elif n<=255:out.append('\\x'+format(n,'02x'))
  elif n<=65535:out.append('\\u'+format(n,'04x'))
  else:out.append('\\U'+format(n,'08x'))
 out.append('"');return ''.join(out)

def _unquote_k_string(token):
 if type(token) is not str or len(token)<2 or token[0]!='"' or token[-1]!='"':_fail()
 out=[];i=1;end=len(token)-1;short={'"':'"','\\':'\\','n':'\n','r':'\r','t':'\t','f':'\f'}
 while i<end:
  char=token[i];i+=1
  if char=='\\':
   if i>=end:_fail()
   escape=token[i];i+=1
   if escape in short:char=short[escape]
   elif escape in ('x','u','U'):
    width={'x':2,'u':4,'U':8}[escape]
    digits=token[i:i+width]
    if len(digits)!=width or i+width>end or any(c not in '0123456789abcdefABCDEF' for c in digits):_fail()
    n=int(digits,16)
    if n>0x10ffff or 0xd800<=n<=0xdfff:_fail()
    char=chr(n);i+=width
   else:_fail()
  elif char in ('"','\n','\r'):_fail()
  if 0xd800<=ord(char)<=0xdfff:_fail()
  out.append(char)
 return ''.join(out)

def _string(value):return _token('String',_quote_k_string(value))
def _list(values):
 result=_apply('ejNil')
 for value in reversed(values):result=_apply('ejCons',value,result)
 return result

def _encode(value):
 pending=[('visit',value)];results=[]
 while pending:
  kind,v=pending.pop()
  if kind=='array':
   children=results[-v:] if v else []
   if v:del results[-v:]
   results.append(_apply('ejArray',_list(children)));continue
  if kind=='object':
   n=len(v);children=results[-n:] if n else []
   if n:del results[-n:]
   results.append(_apply('ejObject',_list([_apply('ejPair',_string(k),x) for k,x in zip(v,children)])));continue
  if v is None:results.append(_apply('ejNull'))
  elif type(v) is bool:results.append(_apply('ejBool',_token('Bool','true' if v else 'false')))
  elif type(v) is int:results.append(_apply('ejNumber',_token('Int',str(v))))
  elif type(v) is str:results.append(_apply('ejString',_string(v)))
  elif type(v) is list:
   pending.append(('array',len(v)));pending.extend(('visit',x) for x in reversed(v))
  elif type(v) is dict:
   pending.append(('object',list(v)));pending.extend(('visit',x) for x in reversed(list(v.values())))
  else:_fail()
 return results[0]

def encode_json(text):
 try:
  if type(text) is not str or len(text.encode('utf-8'))>2_000_000:_fail()
  value=_parse(text)
  if _canonical(value)!=text:_fail()
  # Unpaired Unicode surrogates are forbidden even when escaped in input.
  _canonical(value).encode('utf-8')
  return _encode(value)
 except (UnicodeError,RecursionError,ValueError,TypeError) as e:
  if isinstance(e,CodecError):raise
  _fail()

def encode_request(schema_text,request_text):
 return _apply('expressionRequest',encode_json(schema_text),encode_json(request_text))

def decode_term(term):
 """Decode only our structural term vocabulary, never an arbitrary K label."""
 def token(t,sort):
  if type(t) is not dict or set(t)!={'node','sort','token'} or t['node']!='KToken' or t['sort']!={'node':'KSort','name':sort,'params':[]} or type(t['token']) is not str:_fail()
  s=t['token']
  if sort=='String':return _unquote_k_string(s)
  if sort=='Bool':
   if s not in ('true','false'):_fail()
   return s=='true'
  try:n=int(s)
  except ValueError:_fail()
  if str(n)!=s:_fail()
  return n
 def application(t):
  if type(t) is not dict or set(t)!={'node','label','args','arity'} or t['node']!='KApply' or type(t['args']) is not list or type(t['arity']) is not int or t['arity']!=len(t['args']):_fail()
  label=t['label']
  if type(label) is not dict or set(label)!={'node','name','params'} or label['node']!='KLabel' or label['params']!=[] or type(label['name']) is not str:_fail()
  return label['name'],t['args']
 def items(t):
  out=[]
  while True:
   label,args=application(t)
   if label=='ejNil' and not args:return out
   if label!='ejCons' or len(args)!=2:_fail()
   out.append(args[0]);t=args[1]
   if len(out)>2_000_000:_fail()
 def walk(t):
  pending=[('visit',t)];values=[]
  while pending:
   kind,t=pending.pop()
   if kind in ('array','object'):
    n=t if kind=='array' else len(t);children=values[-n:] if n else []
    if n:del values[-n:]
    values.append(children if kind=='array' else _object(zip(t,children)));continue
   label,a=application(t)
   if label=='ejNull' and not a:values.append(None)
   elif label in ('ejString','ejNumber','ejBool') and len(a)==1:values.append(token(a[0],{'ejString':'String','ejNumber':'Int','ejBool':'Bool'}[label]))
   elif label=='ejArray' and len(a)==1:
    children=items(a[0]);pending.append(('array',len(children)));pending.extend(('visit',x) for x in reversed(children))
   elif label=='ejObject' and len(a)==1:
    keys=[];children=[]
    for x in items(a[0]):
     l,fields=application(x)
     if l!='ejPair' or len(fields)!=2:_fail()
     keys.append(token(fields[0],'String'));children.append(fields[1])
    pending.append(('object',keys));pending.extend(('visit',x) for x in reversed(children))
   else:_fail()
  return values[0]
 try:return walk(term)
 except (UnicodeError,RecursionError,TypeError,KeyError):_fail()

def serialize_term(term):
 """Iterative KAST serialization; does not invoke K or a semantic evaluator."""
 try:return _canonical(term)
 except (UnicodeError,TypeError,ValueError):_fail()

ERROR_CODES=frozenset('INPUT_SCHEMA INPUT_BOUND INPUT_SPAN TYPE_CONSTRUCTOR TYPE_NAME TYPE_LITERAL TYPE_SCHEMA_CYCLE TYPE_COLLECTION_BOUND TYPE_STATEMENT_PLACEMENT TYPE_DUPLICATE_BINDER TYPE_FINANCIAL_WRITE TYPE_DUPLICATE_WRITE TYPE_DUPLICATE_FIELD TYPE_RECORD_FIELDS TYPE_NEXT_READ TYPE_POST_SCOPE TYPE_MISMATCH TYPE_ENUM_MEMBER TYPE_QUANTITY_DOMAIN INPUT_VALUE WORK_EXHAUSTED INDEX_RANGE ARITH_DENOMINATOR ARITH_RANGE GUARD_FAILED ENSURES_FAILED VALUE_BOUND DESCRIPTOR_BOUND'.split())

def decode_result(raw,schema_text='{}',request_text='{}'):
 """Closed KAST4 result and immutable-input binding; no state reconstruction."""
 if type(raw) is not str or len(raw.encode('utf-8'))>64*1024*1024:_fail()
 try:data=_parse(raw,allow_space=True)
 except (ValueError,UnicodeError):_fail()
 if type(data) is not dict or set(data)!={'format','version','term'} or data['format']!='KAST' or type(data['version']) is not int or data['version']!=4:_fail()
 def app(t):
  if type(t) is not dict or set(t)!={'node','label','arity','args'} or t['node']!='KApply' or type(t['args']) is not list or type(t['arity']) is not int or t['arity']!=len(t['args']):_fail()
  n=t['label']
  if type(n) is not dict or set(n)!={'node','name','params'} or n['node']!='KLabel' or n['params']!=[] or type(n['name']) is not str:_fail()
  return n['name'],t['args']
 def tok(t,sort):
  return decode_term(_apply({'Int':'ejNumber','String':'ejString','Bool':'ejBool'}[sort],t))
 label,children=app(data['term'])
 if label!='<generatedTop>':_fail()
 allowed={'<k>','<out>','<sigma>','<request>','<pre>','<args>','<obs>','<localTypes>','<locals>','<written>','<writes>','<descriptors>','<sourceBytes>','<initialWork>','<work>','<reducing>','<generatedCounter>'}
 cells={}
 for child in children:
  name,a=app(child)
  if name not in allowed or name in cells or len(a)!=1:_fail()
  cells[name]=a[0]
 if not {'<k>','<out>','<sigma>','<request>'}<=set(cells):_fail()
 if cells['<k>']!={'node':'KSequence','arity':0,'items':[]}:_fail()
 for name,text in [('<sigma>',schema_text),('<request>',request_text)]:
  if _canonical(decode_term(cells[name]))!=text:_fail()
 label,a=app(cells['<out>'])
 if label=='exValue' and len(a)==3:
  remaining=tok(a[2],'Int')
  if not 0<=remaining<=65536:_fail()
  return {'judgmentResult':'ExpressionValue','type':decode_term(a[0]),'value':decode_term(a[1]),'workRemaining':str(remaining)}
 if label=='exPrepared' and len(a)==3:
  remaining=tok(a[2],'Int')
  if not 0<=remaining<=65536:_fail()
  return {'status':'ExpressionPrepared','post':decode_term(a[0]),'descriptors':decode_term(_apply('ejArray',a[1])),'workRemaining':str(remaining)}
 if label=='exRejected' and len(a)==4:
  code=tok(a[0],'String');span=decode_term(a[1]);path=decode_term(_apply('ejArray',a[2]));work=tok(a[3],'Int')
  if code not in ERROR_CODES or not 0<=work<=65536 or type(span) is not dict or set(span)!={'kind','start','end'} or span['kind'] not in ('source','synthetic') or type(path) is not list:_fail()
  for v in [span['start'],span['end'],*path]:
   if type(v) is not str or not v.isascii() or not v.isdecimal() or str(int(v))!=v:_fail()
  if int(span['start'])>int(span['end']) or (span['kind']=='synthetic' and (span['start']!='0' or span['end']!='0')):_fail()
  return {'status':'Rejected','code':code,'span':span,'nodePath':path,'workUsed':str(work)}
 _fail()
