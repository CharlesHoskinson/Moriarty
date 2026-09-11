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
 try:return json.dumps(value,ensure_ascii=False,sort_keys=True,separators=(',',':'),allow_nan=False)
 except (ValueError,TypeError,RecursionError):_fail()

def _apply(label,*args):return {'node':'KApply','label':label,'args':list(args),'arity':len(args),'variable':False}
def _token(sort,value):return {'node':'KToken','sort':sort,'token':value}
def _string(value):return _token('String',json.dumps(value,ensure_ascii=True))
def _list(values):
 result=_apply('ejNil')
 for value in reversed(values):result=_apply('ejCons',value,result)
 return result

def _encode(value):
 if value is None:return _apply('ejNull')
 if type(value) is bool:return _apply('ejBool',_token('Bool','true' if value else 'false'))
 if type(value) is int:return _apply('ejNumber',_token('Int',str(value)))
 if type(value) is str:return _apply('ejString',_string(value))
 if type(value) is list:return _apply('ejArray',_list([_encode(v) for v in value]))
 if type(value) is dict:return _apply('ejObject',_list([_apply('ejPair',_string(k),_encode(v)) for k,v in value.items()]))
 _fail()

def encode_json(text):
 try:
  if type(text) is not str or len(text.encode('utf-8'))>2_000_000:_fail()
  value=json.loads(text,object_pairs_hook=_object,parse_constant=lambda _: _fail(),parse_float=lambda _: _fail())
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
  if type(t) is not dict or set(t)!={'node','sort','token'} or t['node']!='KToken' or t['sort']!=sort or type(t['token']) is not str:_fail()
  s=t['token']
  if sort=='String':
   try:v=json.loads(s)
   except (ValueError,TypeError):_fail()
   if type(v) is not str:_fail()
   v.encode('utf-8');return v
  if sort=='Bool':
   if s not in ('true','false'):_fail()
   return s=='true'
  try:n=int(s)
  except ValueError:_fail()
  if str(n)!=s:_fail()
  return n
 def application(t):
  if type(t) is not dict or set(t)!={'node','label','args','arity','variable'} or t['node']!='KApply' or t['variable'] is not False or type(t['args']) is not list or type(t['arity']) is not int or t['arity']!=len(t['args']):_fail()
  return t['label'],t['args']
 def items(t):
  out=[]
  while True:
   label,args=application(t)
   if label=='ejNil' and not args:return out
   if label!='ejCons' or len(args)!=2:_fail()
   out.append(args[0]);t=args[1]
   if len(out)>2_000_000:_fail()
 def walk(t):
  label,a=application(t)
  if label=='ejNull' and not a:return None
  if label in ('ejString','ejNumber','ejBool') and len(a)==1:return token(a[0],{'ejString':'String','ejNumber':'Int','ejBool':'Bool'}[label])
  if label=='ejArray' and len(a)==1:return [walk(x) for x in items(a[0])]
  if label=='ejObject' and len(a)==1:
   pairs=[]
   for x in items(a[0]):
    l,fields=application(x)
    if l!='ejPair' or len(fields)!=2:_fail()
    pairs.append((token(fields[0],'String'),walk(fields[1])))
   return _object(pairs)
  _fail()
 try:return walk(term)
 except (UnicodeError,RecursionError,TypeError,KeyError):_fail()
