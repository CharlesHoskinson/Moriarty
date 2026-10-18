"""Fail-closed publication classes and strict public-event extraction (stdlib)."""
import hashlib,json,re,os,stat
from pathlib import Path
from urllib.parse import urlsplit,urlunsplit
from provenance_gate import permits as public_source_permits,profile_sha256,gate_sha256,context as public_source_context
SECRET=re.compile(rb'-----BEGIN [A-Z ]*PRIVATE KEY-----|gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{35,}|sk-(?:proj-)?[A-Za-z0-9_-]{35,}|(?:api[_-]?key|access[_-]?token|auth[_-]?token|password|mnemonic)\s*[=:]\s*["\x27]?[A-Za-z0-9_+/-]{20,}',re.I)
SIGNING=re.compile(rb'\\?"type\\?"\s*:\s*\\?"(?:Payment|Stake|Genesis|Byron)[^"\\]*SigningKey|(?:ed25519_sk|addr_sk|stake_sk)1[a-z0-9]{40,}',re.I)
EVENT=re.compile(rb'"(?:payload_type|reasoning|reasoning_content|chain_of_thought|thoughts|choices)"\s*:|"type"\s*:\s*"(?:thought|thinking|tool_call|tool_result|run\.[^"]*|turn\.[^"]*|lifecycle[^"]*)"|"type"\s*:\s*"text"\s*,\s*"data"\s*:',re.I)
EMBEDDED=re.compile(rb'\\?"payload_type\\?"\s*:\s*\\?"(?:run\.(?:thought|tool|lifecycle|model)|turn\.input|task\.lifecycle)|\\?"type\\?"\s*:\s*\\?"(?:turn\.[^"\\]*|lifecycle\.[^"\\]*|session\.[^"\\]*|thought|thinking|tool(?:[._-](?:call|result|request|response))?)\\?"',re.I)
CACHES={'.git','node_modules','__pycache__','.venv','venv','.cache','.npm','.yarn','.pnpm-store','.pytest_cache','.mypy_cache','.ruff_cache','.gradle','.m2','.cargo-cache','.foreman'}
ARCHIVES=('.tar','.tar.gz','.tar.xz','.tar.zst','.tar.bz2','.tgz','.zip','.deb','.rpm','.appimage','.exe','.jar')
def filename_reason(path):
 p=Path(path);n=p.name.lower();parts={x.lower() for x in p.parts}
 if parts&CACHES:return 'private Git/session/dependency/cache class; manifests retained'
 if n.startswith('.env.') and n not in ['.env.example','.env.sample','.env.template']:return 'environment/authentication configuration'
 if n in ['.env','.netrc','.npmrc','.pypirc','.git-credentials'] or '.ssh' in parts:return 'environment/authentication configuration'
 if (parts&{'wallets','keystore','key-store','keys'} and p.suffix.lower() not in ['.py','.js','.mjs','.ts','.rs','.md','.sh']) or n.endswith(('.db','.sqlite','.sqlite3')):return 'wallet/key store or runtime database class'
 if n.endswith(('.skey','.key','.pem','.p12','.pfx','.session')) or any(x in n for x in ['mnemonic','walletstore','credentials','auth.json','secrets']):return 'key/credential/session filename class'
 if n.endswith(ARCHIVES) and any(re.search(r'runtime|download|tooling|toolchain|binar',x) for x in parts):return 'downloaded runtime archive; pin metadata retained'
 return None
def public_typed_schema(e,graph=False):
 t=e.get('type')
 if isinstance(t,str) and t in ['module','commonjs'] and isinstance(e.get('name'),str) and ('version' in e or 'private' in e):return True
 schema_types={'object','array','string','integer','number','boolean','null'}
 schema_keys={'type','$schema','$id','$ref','$defs','definitions','properties','items','required','description','title','default','enum','const','anyOf','oneOf','allOf','not','additionalProperties','minimum','maximum','minItems','maxItems','minLength','maxLength','pattern','format','nullable','examples','example','readOnly','writeOnly','deprecated','discriminator','x-order'}
 if (isinstance(t,str) and t in schema_types or isinstance(t,list) and all(x in schema_types for x in t)) and set(e)<=schema_keys:return True
 if isinstance(t,str) and (t in ['PlutusScriptV1','PlutusScriptV2','PlutusScriptV3'] or 'VerificationKey' in t) and set(e)<={'type','description','cborHex','contentHash','sha256','bytes','name'}:return True
 if isinstance(t,str) and t in ['I','B','Constr','List','Map'] and set(e)<={'type','data','value','fields','index'}:return True
 if graph and 'id' in e and 'label' in e and not set(e)&{'payload_type','record_type','event','data','payload','content','messages','delta'}:return True
 return False
def provider_records(x,graph=False,schema_map=False):
 if isinstance(x,dict):
  if schema_map:return any(provider_records(v,graph) for v in x.values())
  if any(k in x for k in ['payload_type','record_type','event']) or 'type' in x and not public_typed_schema(x,graph):return True
  if not graph and any(k in x for k in ['reasoning','reasoning_content','chain_of_thought','thoughts','choices']):return True
  return any(provider_records(v,graph,public_typed_schema(x,graph) and k in ['properties','definitions','$defs']) for k,v in x.items())
 if isinstance(x,list):return any(provider_records(v,graph) for v in x)
 if isinstance(x,str):
  if x.lstrip().startswith(('{','[')):
   try:return provider_records(json.loads(x),False)
   except ValueError:return bool(EMBEDDED.search(x.encode()))
  return bool(EMBEDDED.search(x.encode()))
 return False
def sensitive_value(name,value):
 n=name.lower().replace('_','').replace('-','')
 if not isinstance(value,str):return False
 if n=='clientsecret':return bool(value) and not (value.startswith('<') and value.endswith('>')) and not value.startswith('${')
 if n=='privatekey':return bool(re.fullmatch(r'[0-9a-fA-F]{64,256}|[A-Za-z0-9+/=_-]{40,}',value))
 if n in ['mnemonic','seedphrase']:
  words=value.split();return len(words) in [12,15,18,21,24] and all(w.isalpha() for w in words)
 return False
SENSITIVE_FIELD=re.compile(rb'(?:(?:\\*)["\\]?)?(private[_-]?key|client[_-]?secret|mnemonic|seed[_-]?phrase)(?:\\*)["\\]?\s*[:=]\s*(?:\\*)["\\]([^"\\\r\n]{1,1024})(?:\\*)["\\]\s*(?=[,};\]\r\n]|$)',re.I)
CBOR_FIELD=re.compile(rb'cborHex(?:\\*)["\\]?\s*:\s*(?:\\*)["\\](58(?:20[0-9a-f]{64}|40[0-9a-f]{128}|60[0-9a-f]{192}|80[0-9a-f]{256}))(?:\\*)["\\]',re.I)
def private_cbor(value):
 return isinstance(value,str) and bool(re.fullmatch(r'58(?:20[0-9a-fA-F]{64}|40[0-9a-fA-F]{128}|60[0-9a-fA-F]{192}|80[0-9a-fA-F]{256})',value))
def secret_tree(x):
 if isinstance(x,dict):
  public_crypto=isinstance(x.get('type'),str) and (x['type'] in ['PlutusScriptV1','PlutusScriptV2','PlutusScriptV3'] or 'VerificationKey' in x['type']) and 'SigningKey' not in x['type']
  for key,value in x.items():
   if sensitive_value(key,value):return True
   if key.lower()=='cborhex' and private_cbor(value) and not public_crypto:return True
   if secret_tree(value):return True
 elif isinstance(x,list):return any(secret_tree(v) for v in x)
 elif isinstance(x,str) and x.lstrip().startswith(('{','[')):
  try:return secret_tree(json.loads(x))
  except ValueError:return False
 return False
def inspect(path,allow_unit=(),class_path=None):
 actual=Path(path);p=Path(class_path or path);reason=filename_reason(p);h=hashlib.sha256();tail=b'';magic=b'';event=False;event_type=False;event_data=False;embedded=False;unsafe=False;sign=False;sensitive=False;cbor_field=False;small=bytearray();size=0
 with actual.open('rb') as f:
  while b:=f.read(1024*1024):
   h.update(b);size+=len(b)
   if not magic:magic=b[:8]
   if len(small)<=1024:small.extend(b[:1025-len(small)])
   if not reason:
    z=tail+b;unsafe|=bool(SECRET.search(z));sensitive|=any(sensitive_value(m.group(1).decode(),m.group(2).decode(errors='replace')) for m in SENSITIVE_FIELD.finditer(z));cbor_field|=bool(CBOR_FIELD.search(z));sign|=bool(SIGNING.search(z));event|=bool(EVENT.search(z));event_type|=bool(re.search(rb'"(?:type|event|record_type)"\s*:',z));event_data|=bool(re.search(rb'"(?:data|payload|delta|messages|text)"\s*:',z));embedded|=bool(EMBEDDED.search(z));tail=b[-4096:]
 digest=h.hexdigest()
 if magic.startswith((b'\x7fELF',b'MZ',b'\xcf\xfa\xed\xfe',b'\xfe\xed\xfa')):reason='executable runtime binary; pin metadata retained'
 if size<=1024 and p.suffix not in ['.sha256','.sha1','.md'] and re.fullmatch(rb'(?:[0-9a-fA-F]{64,512}|(?:5820|5840)[0-9a-fA-F]{64,128})\s*',bytes(small)):reason='bare key/CBOR-shaped material; fail closed'
 if 'key' in p.name.lower() and (p.suffix.lower() not in ['.py','.js','.mjs','.ts','.rs','.md','.sh'] or any(x>126 for x in small)) and magic[:2] in [b'\x58\x20',b'\x58\x40'] and size==magic[1]+2:reason='key-named binary CBOR material regardless of byte values; fail closed'
 if size in [32,34,64,66,96,98,128,130] and any(x<9 or 13<x<32 for x in small):reason='binary key/CBOR-shaped material regardless of suffix; fail closed'
 if magic[:2] in [b'\x58\x20',b'\x58\x40',b'\x58\x60',b'\x58\x80'] and size==magic[1]+2:reason='unlabelled binary CBOR key-sized material; fail closed'
 if sensitive:reason='concrete private-key/client-secret/mnemonic value'
 if unsafe:reason='credential/private-key header content signature'
 if sign and digest not in allow_unit:reason='unapproved signing-key content signature'
 structured=bytes(small).lstrip().startswith((b'{',b'['))
 event=event or structured and event_type
 provider=(p.suffix.lower() in ['.jsonl','.ndjson'] or event and (structured or p.suffix.lower() not in ['.py','.js','.mjs','.ts','.md','.html','.lean','.rs','.sh']) or any(x in p.name.lower() for x in ['private-events','raw-provider','raw-model','model-trace']))
 graph_schema=bool(re.match(rb'\s*\{\s*"(?:directed|nodes|hyperedges)"\s*:',bytes(small)))
 parser_only_candidate=False
 if structured:
  try:
   with actual.open() as f:data=json.load(f)
   if secret_tree(data):reason='concrete nested credential or unlabelled CBOR key material'
   provider=provider_records(data,graph_schema)
  except RecursionError:
   # Nesting beyond the decoder or traversal depth cannot be classified; withhold without raising the recursion limit.
   reason=reason or 'structured JSON nesting exceeds bounded inspection depth; fail closed';provider=True
  except (ValueError,UnicodeError):
   provider=True
   if cbor_field and digest not in allow_unit:reason='unlabelled CBOR key-sized field in non-JSON content'
   source_formats={'.hs','.hs-boot','.lhs','.agda','.toml','.md','.nix','.rtf','.dhall','.jq','.awk','.sh','.py','.js','.mjs','.ts','.rs','.lean','.html'}
   prefix=bytes(small).lstrip();rest=prefix[1:].lstrip()
   json_prefix=(prefix.startswith(b'{') and rest.startswith((b'\"',b'}'))) or (prefix.startswith(b'[') and bool(re.match(rb'(?:[\"{\[\]0-9-]|true\b|false\b|null\b)',rest)))
   raw_name=any(x in p.name.lower() for x in ['private-events','raw-provider','raw-model','model-trace'])
   parser_only_candidate=(p.suffix.lower() in source_formats and not json_prefix and not event and not event_type and not embedded and not raw_name)
   if parser_only_candidate and public_source_permits(digest,p):provider=False
 if not structured and cbor_field and digest not in allow_unit:reason='unlabelled CBOR key-sized field in non-JSON content'
 if embedded:provider=True
 raw_name=any(x in p.name.lower() for x in ['private-events','raw-provider','raw-model','model-trace'])
 if raw_name:provider=True
 skill_source=any(marker in '/'+p.as_posix() for marker in ['/.claude/skills/','/.claude/commands/','/.codex/skills/'])
 if skill_source and not reason and not public_source_permits(digest,p):reason='exact public provenance required for skill or command source'
 return {'parser_only_candidate':parser_only_candidate,'skill_source':skill_source,'graph_schema':graph_schema,'provider_marker':event,'embedded_private_envelope':embedded,'structured':structured,'bytes':size,'sha256':digest,'reason':reason,'provider_capture':provider}
def strict_text(e):
 if not isinstance(e,dict):return None
 if 'payload_type' not in e and 'record_type' not in e and e.get('type')=='text' and isinstance(e.get('data'),str):return e['data']
 if e.get('payload_type') in ['run.output.delta','run.terminal.completed'] and isinstance(e.get('payload'),dict) and isinstance(e['payload'].get('text'),str):return e['payload']['text']
 return None
def extraction(path):
 texts=[];unknown=0;error=None
 try:
  # First support JSON/array capture without treating nested tool data as events.
  if Path(path).stat().st_size<=64*1024*1024:
   try:
    data=json.loads(Path(path).read_text());events=data if isinstance(data,list) else [data]
   except (ValueError,UnicodeError):events=None
  else:events=None
  if events is None:
   events=[]
   with Path(path).open() as f:
    for line in f:
     if not line.strip():continue
     try:events.append(json.loads(line))
     except ValueError:unknown+=1
  for e in events:
   t=strict_text(e)
   if t is not None:texts.append(t)
   else:unknown+=1
 except (OSError,UnicodeError,ValueError,RecursionError) as ex:error=type(ex).__name__
 body='\n'.join(texts).encode()
 status='public-text' if body else 'empty-or-unclassified'
 if SECRET.search(body) or SIGNING.search(body) or any(sensitive_value(m.group(1).decode(),m.group(2).decode(errors='replace')) for m in SENSITIVE_FIELD.finditer(body)) or CBOR_FIELD.search(body):body=b'';status='withheld-unsafe-public-text'
 if error:status='unparseable-capture';body=b''
 return body,{'status':status,'omitted_or_unknown_events':unknown,'parse_error':error,'rule':'Grok e.type==text and string e.data; Muse e.payload_type in run.output.delta/run.terminal.completed and string e.payload.text only; no nested/tool payload traversal'}
def redact_remote(remote):
 h=hashlib.sha256(remote.encode()).hexdigest();changed=False
 if '://' in remote:
  u=urlsplit(remote);host=u.hostname or '';port=':'+str(u.port) if u.port else '';changed=bool(u.username or u.password or u.query or u.fragment);clean=urlunsplit((u.scheme,host+port,u.path,'',''))
 elif '@' in remote:
  hostpath=remote.rsplit('@',1)[1];clean='ssh://'+hostpath.replace(':','/',1);changed=True
 else:clean=remote
 return {'remote':clean,'remote_userinfo_or_query_redacted':changed,'original_remote_sha256':h}
def confined(path,base,sources=()):
 p=Path(os.path.abspath(path));base=Path(base).resolve()
 for c in [p,*p.parents]:
  if c.is_symlink():raise ValueError('publication destination symlink refused')
 resolved=p.resolve()
 if not resolved.is_relative_to(base) or any(resolved.is_relative_to(Path(s).resolve()) for s in sources):raise ValueError('publication destination escape/source alias')
 return p


def classification_context(path):
 p=Path(path)
 return (p.suffix,filename_reason(p),'key' in p.name.lower(),any(x in p.name.lower() for x in ['private-events','raw-provider','raw-model','model-trace']),any(m in '/'+p.as_posix() for m in ['/.claude/skills/','/.claude/commands/','/.codex/skills/']),public_source_context(p))
