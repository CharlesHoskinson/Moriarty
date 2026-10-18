"""Fail-closed publication classes and strict public-event extraction (stdlib)."""
import hashlib,json,re,os,stat
from pathlib import Path
from urllib.parse import urlsplit,urlunsplit
SECRET=re.compile(rb'-----BEGIN [A-Z ]*PRIVATE KEY-----|gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{35,}|sk-(?:proj-)?[A-Za-z0-9_-]{35,}|(?:api[_-]?key|access[_-]?token|auth[_-]?token|password|mnemonic)\s*[=:]\s*["\x27]?[A-Za-z0-9_+/-]{20,}',re.I)
SIGNING=re.compile(rb'\\?"type\\?"\s*:\s*\\?"(?:Payment|Stake|Genesis|Byron)[^"\\]*SigningKey|(?:ed25519_sk|addr_sk|stake_sk)1[a-z0-9]{40,}',re.I)
EVENT=re.compile(rb'"(?:payload_type|reasoning|reasoning_content|chain_of_thought|thoughts|choices)"\s*:|"type"\s*:\s*"(?:thought|thinking|tool_call|tool_result|run\.[^"]*|turn\.[^"]*|lifecycle[^"]*)"|"type"\s*:\s*"text"\s*,\s*"data"\s*:',re.I)
EMBEDDED=re.compile(rb'\\?"payload_type\\?"\s*:\s*\\?"(?:run\.(?:thought|tool|lifecycle|model)|turn\.input|task\.lifecycle)|\\?"type\\?"\s*:\s*\\?"(?:thought|thinking|tool(?:[._-](?:call|result|request|response))?)\\?"',re.I)
CACHES={'.git','node_modules','__pycache__','.venv','venv','.cache','.npm','.yarn','.pnpm-store','.pytest_cache','.mypy_cache','.ruff_cache','.foreman'}
ARCHIVES=('.tar','.tar.gz','.tar.xz','.tar.zst','.tar.bz2','.tgz','.zip','.deb','.rpm','.appimage','.exe','.jar')
def filename_reason(path):
 p=Path(path);n=p.name.lower();parts={x.lower() for x in p.parts}
 if parts&CACHES:return 'private Git/session/dependency/cache class; manifests retained'
 if n in ['.env','.netrc','.npmrc','.pypirc','.git-credentials'] or '.ssh' in parts:return 'environment/authentication configuration'
 if (parts&{'wallets','keystore','key-store','keys'} and p.suffix.lower() not in ['.py','.js','.mjs','.ts','.rs','.md','.sh']) or n.endswith(('.db','.sqlite','.sqlite3')):return 'wallet/key store or runtime database class'
 if n.endswith(('.skey','.key','.pem','.p12','.pfx','.session')) or any(x in n for x in ['mnemonic','walletstore','credentials','auth.json','secrets']):return 'key/credential/session filename class'
 if n.endswith(ARCHIVES) and any(re.search(r'runtime|download|tooling|toolchain|binar',x) for x in parts):return 'downloaded runtime archive; pin metadata retained'
 return None
def inspect(path,allow_unit=(),class_path=None):
 actual=Path(path);p=Path(class_path or path);reason=filename_reason(p);h=hashlib.sha256();tail=b'';magic=b'';event=False;event_type=False;event_data=False;embedded=False;unsafe=False;sign=False;small=bytearray();size=0
 with actual.open('rb') as f:
  while b:=f.read(1024*1024):
   h.update(b);size+=len(b)
   if not magic:magic=b[:8]
   if len(small)<=1024:small.extend(b[:1025-len(small)])
   if not reason:
    z=tail+b;unsafe|=bool(SECRET.search(z));sign|=bool(SIGNING.search(z));event|=bool(EVENT.search(z));event_type|=bool(re.search(rb'"(?:type|event|record_type)"\s*:',z));event_data|=bool(re.search(rb'"(?:data|payload|delta|messages|text)"\s*:',z));embedded|=bool(EMBEDDED.search(z));tail=b[-4096:]
 digest=h.hexdigest()
 if magic.startswith((b'\x7fELF',b'MZ',b'\xcf\xfa\xed\xfe',b'\xfe\xed\xfa')):reason='executable runtime binary; pin metadata retained'
 if size<=1024 and p.suffix not in ['.sha256','.sha1','.md'] and re.fullmatch(rb'(?:[0-9a-fA-F]{64,512}|(?:5820|5840)[0-9a-fA-F]{64,128})\s*',bytes(small)):reason='bare key/CBOR-shaped material; fail closed'
 if size in [32,64] and not p.suffix and any(x<9 or 13<x<32 for x in small):reason='extensionless binary key-shaped material; fail closed'
 if unsafe:reason='credential/private-key header content signature'
 if sign and digest not in allow_unit:reason='unapproved signing-key content signature'
 structured=bytes(small).lstrip().startswith((b'{',b'['))
 event=event or structured and event_type and event_data
 provider=(p.suffix.lower() in ['.jsonl','.ndjson'] or event and (structured or p.suffix.lower() not in ['.py','.js','.mjs','.ts','.md','.html','.lean','.rs','.sh']) or any(x in p.name.lower() for x in ['private-events','raw-provider','raw-model','model-trace']))
 graph_schema=bool(re.match(rb'\s*\{\s*"(?:directed|nodes|hyperedges)"\s*:',bytes(small)))
 if graph_schema:provider=embedded
 return {'graph_schema':graph_schema,'provider_marker':event,'embedded_private_envelope':embedded,'structured':structured,'bytes':size,'sha256':digest,'reason':reason,'provider_capture':provider}
def strict_text(e):
 if not isinstance(e,dict):return None
 if e.get('type')=='text' and isinstance(e.get('data'),str):return e['data']
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
 except (OSError,UnicodeError,ValueError) as ex:error=type(ex).__name__
 body='\n'.join(texts).encode()
 status='public-text' if body else 'empty-or-unclassified'
 if SECRET.search(body) or SIGNING.search(body):body=b'';status='withheld-unsafe-public-text'
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
