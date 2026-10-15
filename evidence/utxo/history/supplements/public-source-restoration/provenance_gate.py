"""Verify exact public-source provenance; no caller Boolean bypass or cache."""
import hashlib,json,os,re,subprocess
from pathlib import Path
from urllib.parse import urlsplit

CONFIG=os.environ.get('UTXO_PUBLIC_SOURCE_PROFILE')
EXPECTED=os.environ.get('UTXO_PUBLIC_SOURCE_PROFILE_SHA256')
PROFILE_BYTES=Path(CONFIG).read_bytes() if CONFIG else None
if CONFIG and (not EXPECTED or not re.fullmatch('[0-9a-f]{64}',EXPECTED) or hashlib.sha256(PROFILE_BYTES).hexdigest()!=EXPECTED):raise ValueError('independent curator profile SHA pin required')
PROFILE=json.loads(PROFILE_BYTES) if PROFILE_BYTES else None
if PROFILE is not None and PROFILE.get('schema')!='moriarty-public-source-profile/1':raise ValueError('unknown public-source profile')
GATE_BYTES=Path(__file__).read_bytes()
def gate_sha256():
 if Path(__file__).read_bytes()!=GATE_BYTES:raise ValueError('provenance gate changed during run')
 return hashlib.sha256(GATE_BYTES).hexdigest()
def profile_sha256():
 if not CONFIG:return None
 if Path(CONFIG).read_bytes()!=PROFILE_BYTES:raise ValueError('public-source profile changed during run')
 return hashlib.sha256(PROFILE_BYTES).hexdigest()
def plain_path(value):
 p=Path(value)
 if not p.is_absolute() or any(c.is_symlink() for c in [p,*p.parents]):raise ValueError('provenance source symlink/nonabsolute refused')
 return p
def public_url(value):
 u=urlsplit(value)
 if u.scheme!='https' or not u.hostname or u.username or u.password or u.query or u.fragment:raise ValueError('nonpublic credential-bearing provenance URL')
 return value
def proof_document(entry):
 p=plain_path(entry['proof_path']);body=p.read_bytes()
 if hashlib.sha256(body).hexdigest()!=entry['proof_sha256']:raise ValueError('public provenance manifest changed')
 return json.loads(body)
def verify(entry):
 if not re.fullmatch('[0-9a-f]{64}',entry['sha256']):raise ValueError('invalid source digest')
 if type(entry['bytes']) is not int or entry['bytes']<0:raise ValueError('invalid source byte count')
 proof=proof_document(entry)
 if entry['kind']=='git-commit-blob':
  match=[r for r in proof if r.get('root')==entry['root'] and r.get('path')==entry['repository_original_path'] and r.get('commit')==entry['commit'] and r.get('remote')==entry['remote']]
  if len(match)!=1 or not re.fullmatch('[0-9a-f]{40}',entry['commit']):raise ValueError('public commit not pinned by collection provenance')
  public_url(entry['remote']);repo=plain_path(entry['repository_path']);rel=Path(entry['repository_file'])
  if rel.is_absolute() or '..' in rel.parts:raise ValueError('Git source path escape')
  remote=subprocess.check_output(['git','-C',str(repo),'config','--get','remote.origin.url'],text=True,stderr=subprocess.DEVNULL).strip()
  if remote!=entry['remote']:raise ValueError('repository remote differs from public collection')
  body=subprocess.check_output(['git','-C',str(repo),'cat-file','blob',entry['commit']+':'+rel.as_posix()],stderr=subprocess.DEVNULL)
 elif entry['kind']=='public-document-manifest':
  public_url(entry['url']);matches=[r for r in proof if r.get('path')==entry['document_manifest_path'] and r.get('sha256')==entry['sha256'] and r.get('bytes')==entry['bytes'] and r.get('official_url')==entry['url']]
  if len(matches)!=1:raise ValueError('document URL/hash not pinned by supplied public provenance')
  body=plain_path(entry['source_path']).read_bytes()
 else:raise ValueError('unsupported public-source provenance')
 if len(body)!=entry['bytes'] or hashlib.sha256(body).hexdigest()!=entry['sha256']:raise ValueError('exact public source bytes differ from pinned proof')
 return True
def context(path):
 if PROFILE is None:return None
 p=Path(os.path.abspath(path));matches=[]
 for root,base in PROFILE['publication_roots'].items():
  if root not in ['research','validation']:raise ValueError('unknown publication root')
  base=plain_path(base)
  if p.is_relative_to(base):matches.append((root,p.relative_to(base).as_posix()))
 if len(matches)!=1:return None
 return matches[0]
def permits(digest,path):
 if PROFILE is None:return False
 profile_sha256()
 ctx=context(path)
 if ctx is None:return False
 matches=[e for e in PROFILE['entries'] if e.get('sha256')==digest and (e.get('root'),e.get('original_path'))==ctx]
 if not matches:return False
 for entry in matches:
  try:
   if verify(entry):return True
  except (ValueError,OSError,KeyError,TypeError,subprocess.CalledProcessError):continue
 return False
