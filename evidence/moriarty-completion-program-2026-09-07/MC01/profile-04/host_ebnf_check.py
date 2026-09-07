"""Independent EBNF recognizer for source-profile checks, not the Moriarty frontend."""
import re,json,sys,pathlib,functools

def grammar(text):
 text=re.sub(r'\(\*.*?\*\)','',text,flags=re.S)
 toks=re.findall(r'"(?:\\.|[^"\\])*"|\?[^?]*\?|[A-Za-z_][A-Za-z_0-9]*|[=;|,{}\[\]()]',text)
 at=0;rules={}
 def eat(v):
  nonlocal at
  assert toks[at]==v,(at,toks[at],v);at+=1
 def alt():
  nonlocal at
  xs=[seq()]
  while at<len(toks) and toks[at]=='|':at+=1;xs.append(seq())
  return ('alt',tuple(xs))
 def seq():
  nonlocal at
  xs=[atom()]
  while at<len(toks) and toks[at]==',':at+=1;xs.append(atom())
  return ('seq',tuple(xs))
 def atom():
  nonlocal at
  t=toks[at];at+=1
  if t in ['{','[','(']:
   n=alt();eat({'{':'}','[':']','(':')'}[t]);return ({'{':'rep','[':'opt','(':'group'}[t],n)
  if t.startswith('"'):return ('lit',json.loads(t))
  if t.startswith('?'):return ('special',t)
  return ('ref',t)
 while at<len(toks):
  name=toks[at];at+=1;eat('=');rules[name]=alt();eat(';')
 return rules

def check(rules,source):
 token=re.compile(r'\s+|"(?:\\.|[^"\\])*"|[A-Za-z_][A-Za-z_0-9]*|[0-9]+|==|<=|>=|[{}():;,.<>+*=\-]')
 ts=[];p=0
 while p<len(source):
  m=token.match(source,p)
  if not m:return {'accepted':False,'lexical_error_char_offset':p}
  if not m[0].isspace():ts.append(m[0])
  p=m.end()
 def literals(n):
  if n[0]=='lit':return {n[1]}
  if n[0] in ['seq','alt']:return set().union(*(literals(x) for x in n[1]))
  if n[0] in ['rep','opt','group']:return literals(n[1])
  return set()
 keywords=literals(rules['keyword']);reserved=literals(rules['reserved_identifier']);far=0
 @functools.lru_cache(None)
 def run(n,p):
  nonlocal far
  far=max(far,p);tag,x=n
  if tag=='lit':return frozenset([p+1]) if p<len(ts) and ts[p]==x else frozenset()
  if tag=='ref':
   if x=='identifier':ok=p<len(ts) and re.fullmatch(r'[A-Za-z][A-Za-z0-9_]{0,63}',ts[p]) and ts[p] not in keywords|reserved;return frozenset([p+1]) if ok else frozenset()
   if x=='uint_token':ok=p<len(ts) and re.fullmatch(r'0|[1-9][0-9]*',ts[p]);return frozenset([p+1]) if ok else frozenset()
   if x=='json_string':
    if p>=len(ts) or not ts[p].startswith('"'):return frozenset()
    try:v=json.loads(ts[p]);v.encode('utf-8')
    except (ValueError,UnicodeError):return frozenset()
    return frozenset([p+1])
   return run(rules[x],p)
  if tag=='alt':return frozenset().union(*(run(n,p) for n in x))
  if tag=='seq':
   states=frozenset([p])
   for n in x:states=frozenset().union(*(run(n,q) for q in states))
   return states
  if tag=='group':return run(x,p)
  if tag=='opt':return run(x,p)|{p}
  if tag=='rep':
   states={p};front={p}
   while front:
    new=set().union(*(run(x,q) for q in front))-states;states|=new;front=new
   return frozenset(states)
  raise ValueError('Unimplemented EBNF special '+str(n))
 ends=run(rules['agreement'],0)
 return {'accepted':len(ts) in ends,'tokens':len(ts),'furthest_token':far,'near':ts[max(0,far-3):far+4]}

if __name__=='__main__':
 rules=grammar(pathlib.Path(sys.argv[1]).read_text());out={str(p):check(rules,pathlib.Path(p).read_text()) for p in sys.argv[2:]};print(json.dumps(out,indent=2));sys.exit(0 if all(x['accepted'] for x in out.values()) else 1)
