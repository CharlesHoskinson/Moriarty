"""K6 custody helpers. Source-only; importing does no I/O.
Only a future authorized operator may call these under a new exact
AUTH. PK bytes are never opened, read, copied or hashed; PK custody
is metadata-only. Protected byte-reads, in order, are the binary copy
then the setup-record copy, last in prepare_inputs after all public
identity checks. No subprocess, network, RNG, keys, wallet, signing
or ledger. No arbitrary-path read API. No snapshot claim stronger
than fd-pinned traversal under no concurrent writer by same operator.
"""
import hashlib
import json
import os
import stat
BASE="/home/charl/moriarty-eutxo-validation-2026-10-12"
_BASE_DEPTH=len([c for c in BASE.split("/") if c])
T5=BASE+"/current-cardano-k2-k5-single-party-test-setup-20261016"
T6=BASE+"/current-cardano-k2-k6-proof-roundtrip-20261016"
M5=BASE+"/current-cardano-k2-k5-setup-operator-materialized-20261016"
_G1=BASE+"/current-cardano-k2-g1-materialized-candidate-20261015"
_K4H=BASE+"/current-cardano-k2-k4h-hostile-materialized-20261016"
T6_W=T6+"/w"; T6_BIN=T6_W+"/bin"; T6_IN=T6_W+"/in"
T6_OUT=T6_W+"/out"; T6_STAGE=T6_W+"/stage"; T6_LOGS=T6+"/logs"
T6_JOURNAL=T6+"/journal"; T6_RECEIPTS=T6+"/receipts"
T6_NEGATIVES=T6_RECEIPTS+"/negatives"
KEYS_DIR=T5+"/w/keys"; PK_PATH=KEYS_DIR+"/invoice-partial.pk"
PK_NAME="invoice-partial.pk"; PK_BYTES=98715456; PK_MODE=0o600
KEYS_DIR_MODE=0o700; RECORD_BYTES=3926
RECORD_SHA256="86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"
RECORD_SRC=T5+"/w/out/invoice-partial.setup.json"
RECORD_DST=T6_IN+"/invoice-partial.setup.json"
BINARY_BYTES=3941352
BINARY_SHA256="d446ab0c0dececae719fe6583532f6c2a4285f67fbc89893d61251adff190902"
BINARY_SRC=T5+"/w/bin/mori-k2"; BINARY_DST=T6_BIN+"/mori-k2"
GOLD_BYTES=39353
GOLD_SHA256="65c90d3d13912b762b4977e08ea86212b2b83a16bb2b3ddcaa7a21c905075e46"
VK_DIGEST="d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd"
PROOF_OUT=T6_OUT+"/invoice-partial.proof.json"
PROOF_RECEIPT=T6_RECEIPTS+"/invoice-partial.proof.json"
FREEZE_MANIFEST=T6_RECEIPTS+"/FREEZE-MANIFEST.json"
FREEZE_MANIFEST_NAME="FREEZE-MANIFEST.json"
LAYOUT_LOG=T6_LOGS+"/custody-layout.log"
LAYOUT_STATUS=T6+"/LAYOUT-STATUS.json"
LAYOUT_STATUS_NAME="LAYOUT-STATUS.json"
K5M_PATH=M5+"/operator/request/steps/K5M.sh"; K5M_BYTES=7395
K5M_SHA256="649548a07de4d58d6129cfcd14cd468c030106b9428fa60fb31a6e0954688fe5"
ROOT26_PATH=M5+"/ROOT-SOURCE-MANIFEST.json"; ROOT26_BYTES=3902
ROOT26_SHA256="0110969fb47287949fd22e738086a9c51b2240c6fc46f63403cf6f96fee2f062"
ROOT26_COUNT=26; K5G_PATH=T5+"/receipts/K5g.json"; K5G_BYTES=3724
K5G_SHA256="71b4ee8f2cbb57dc9a756ae712ae3334a077efe701acd9daf590f89671f35890"
EXPORT_HEADROOM=4096; MAX_PUBLIC_BYTES=GOLD_BYTES+EXPORT_HEADROOM
MAX_BINARY_BYTES=BINARY_BYTES; MAX_RECORD_BYTES=RECORD_BYTES
MAX_VARIANT_BYTES=RECORD_BYTES+EXPORT_HEADROOM
MAX_PROOF_BYTES=GOLD_BYTES+EXPORT_HEADROOM
MAX_FREEZE_FILE_BYTES=BINARY_BYTES+EXPORT_HEADROOM
MAX_FREEZE_FILES=256; MAX_FREEZE_AGGREGATE_BYTES=10*BINARY_BYTES
HOST_STEP_CPU_SECONDS=300; HOST_STEP_WALL_SECONDS=600; _CHUNK=65536
_SHA=frozenset("0123456789abcdef")
PUBLIC_FIXTURES={
 "installment-invoice.mori":(_G1+"/r1cs/fixtures/reused/installment-invoice.mori",1497,"105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9"),
 "conditional-reserve.mori":(_G1+"/r1cs/fixtures/reused/conditional-reserve.mori",1415,"395e647534160fde45325232e5f1650a8bd48ce1455a68b28af203bf8bdbe357"),
 "invoice-partial.selector.json":(_G1+"/k2/requests/invoice-partial.selector.json",252,"1f011dec179e394591a69859d06c999337ba45133d63627140a89c30fa591ee4"),
 "invoice-partial.request.json":(_G1+"/k2/requests/invoice-partial.request.json",3008,"aa1703e7b3aa2e0dd6288668cc871996fc1ee49ecc90861a3c4b40f02107f3cd"),
 "K4H-GOLD.json":(_K4H+"/patches/k2/gold/K4H-GOLD.json",GOLD_BYTES,GOLD_SHA256)}
FIXTURE_ORDER=("installment-invoice.mori","conditional-reserve.mori","invoice-partial.selector.json","invoice-partial.request.json","K4H-GOLD.json")
PROTECTED_READS=(BINARY_SRC,RECORD_SRC)
T6_LAYOUT=(("logs",0o755),("journal",0o755),("receipts",0o755),("receipts/negatives",0o755),("w",0o755),("w/bin",0o755),("w/in",0o755),("w/out",0o755),("w/stage",0o755))
NEW_MODULE_OUTPUTS=(FREEZE_MANIFEST,LAYOUT_LOG,LAYOUT_STATUS)
NEGATIVE_PROOF_FILES={"N1":"n1-body-rederived.proof.json","N2":"n2-context-rederived.proof.json","N3":"n3-proof-ac-swapped.proof.json","N4":"n4-x-swapped.proof.json","N5":"n5-statement-stale-digest.proof.json","N6":"n6-proof-truncated.proof.json","N7":"n7-proof-identity.proof.json"}
NEGATIVE_REQUEST_FILES={"N10":"n10-finish-selector.request.json"}
RECORD_VARIANT_FILES={"N8":"n8-record-vk-digest.setup.json","N11":"n11-key-size.setup.json"}
RECORD_VARIANT_EDITS={"N8":{"pointer":"/vkDigestBlake2b256","old":VK_DIGEST},"N11":{"pointer":"/provingKeyBytes","old":98715456,"new":98715457}}
EXPECTED_EXPORT_SET=frozenset(("N1","N2","N3","N4","N5","N6","N7","N8","N10","N11"))
POSTCHECK_STAGES=("Q6P","P6","N11","Z61")
_FREEZE_ROOTS=("logs","journal","receipts")
_FORBIDDEN_COMP=frozenset(("keys","keys-ro","w"))
_FORBIDDEN_TOK=(".pk","provingkey","proving-key","proving_key","setup.json","witness","srs","wallet","credential","secret")
_K5M_LITERALS=("--out-record /task/out/invoice-partial.setup.json","--out-proving-key /task/keys/invoice-partial.pk",'--bind "$W/out" /task/out','--bind "$W/keys" /task/keys')
class CustodyError(Exception):
 def __init__(self,category,message):
  self.category=category; super().__init__("BLOCKED(%s): %s"%(category,message))
def _fail(c,m): raise CustodyError(c,m)
def _mode(st): return stat.S_IMODE(st.st_mode)
def _check_sha(v,label):
 if not isinstance(v,str) or len(v)!=64 or any(c not in _SHA for c in v): _fail("identity",label+" must be 64 lowercase hex")
 return v
def _ident(st): return (st.st_dev,st.st_ino,st.st_nlink,_mode(st),st.st_size,st.st_mtime_ns,st.st_ctime_ns,st.st_uid,st.st_gid)
def _dict(st): return {"dev":st.st_dev,"ino":st.st_ino,"size":st.st_size,"mode":_mode(st),"nlink":st.st_nlink,"uid":st.st_uid,"gid":st.st_gid,"mtime_ns":st.st_mtime_ns,"ctime_ns":st.st_ctime_ns}
def _parts(path):
 if not isinstance(path,str) or not path.startswith(BASE+"/"): _fail("path","path outside pinned BASE")
 p=[c for c in path.split("/") if c]
 if any(c in (".","..") for c in p): _fail("path","dot component")
 return p
def _open_abs(path,directory=False):
 parts=_parts(path); fd=None
 try:
  fd=os.open("/",os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW|os.O_CLOEXEC)
  for i,c in enumerate(parts):
   last=i==len(parts)-1
   if last and not directory: nf=os.open(c,os.O_RDONLY|os.O_NOFOLLOW|os.O_CLOEXEC,dir_fd=fd)
   else: nf=os.open(c,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW|os.O_CLOEXEC,dir_fd=fd)
   os.close(fd); fd=nf
   st=os.fstat(fd)
   if not last and not stat.S_ISDIR(st.st_mode): _fail("type","not directory: "+path)
   if i>=_BASE_DEPTH-1 and st.st_uid!=os.getuid(): _fail("owner","component not operator-owned: "+path)
   if last and directory and not stat.S_ISDIR(st.st_mode): _fail("type","not directory: "+path)
   if last and not directory and not stat.S_ISREG(st.st_mode): _fail("type","not regular file: "+path)
  return fd
 except CustodyError:
  if fd is not None:
   try: os.close(fd)
   except OSError: pass
  raise
 except OSError as e:
  if fd is not None:
   try: os.close(fd)
   except OSError: pass
  _fail("open",str(e))
def _open_parent(path):
 parts=_parts(path)
 if len(parts)<2: _fail("path","no parent")
 parent="/"+"/".join(parts[:-1])
 # parent may be BASE itself, which _parts rejects; handle via walk
 try: fd=_open_abs(parent,True) if parent.startswith(BASE+"/") else _open_base_parent(parent)
 except CustodyError: raise
 return fd,parts[-1]
def _open_base_parent(parent):
 fd=os.open("/",os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW|os.O_CLOEXEC)
 try:
  comps=[x for x in parent.split("/") if x]
  for i,c in enumerate(comps):
   nf=os.open(c,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW|os.O_CLOEXEC,dir_fd=fd); os.close(fd); fd=nf
   st=os.fstat(fd)
   if not stat.S_ISDIR(st.st_mode): _fail("type","parent component not directory")
   if i>=_BASE_DEPTH-1 and st.st_uid!=os.getuid(): _fail("owner","parent not operator-owned")
  return fd
 except CustodyError:
  os.close(fd); raise
 except OSError as e:
  os.close(fd); _fail("open",str(e))
def _hash_fd(fd,limit):
 os.lseek(fd,0,os.SEEK_SET); h=hashlib.sha256(); n=0
 while True:
  b=os.read(fd,_CHUNK)
  if not b: break
  n+=len(b)
  if n>limit: _fail("cap","size cap exceeded")
  h.update(b)
 return n,h.hexdigest()
def _verify_fd(fd,exp_bytes,exp_sha,label,exp_mode=None):
 _check_sha(exp_sha,label)
 if not isinstance(exp_bytes,int) or exp_bytes<0: _fail("identity",label+" bad bytes")
 pre=os.fstat(fd)
 if not stat.S_ISREG(pre.st_mode) or pre.st_nlink!=1: _fail("type",label+" not regular nlink1")
 if pre.st_uid!=os.getuid(): _fail("owner",label+" not operator-owned")
 if exp_mode is not None and _mode(pre)!=exp_mode: _fail("mode",label+" mode mismatch")
 if pre.st_size!=exp_bytes: _fail("identity",label+" size mismatch")
 n,digest=_hash_fd(fd,exp_bytes)
 post=os.fstat(fd)
 if _ident(pre)!=_ident(post): _fail("stable",label+" changed while reading")
 if n!=exp_bytes or digest!=exp_sha: _fail("identity",label+" sha mismatch")
 return _dict(pre)
def _read_pinned(path,exp_bytes,exp_sha,label,limit):
 if type(exp_bytes) is not int or type(limit) is not int or exp_bytes<0 or limit<0 or exp_bytes>limit: _fail("cap",label+" exceeds read cap")
 fd=_open_abs(path)
 try:
  meta=_verify_fd(fd,exp_bytes,exp_sha,label)
  os.lseek(fd,0,os.SEEK_SET); data=b""
  while len(data)<exp_bytes:
   b=os.read(fd,min(_CHUNK,exp_bytes-len(data),limit-len(data)))
   if not b: _fail("read",label+" short read")
   data+=b
   if len(data)>limit: _fail("cap",label+" read cap exceeded")
  return data,meta
 finally: os.close(fd)
def _write_all(fd,data):
 v=memoryview(data); off=0
 while off<len(v):
  w=os.write(fd,v[off:])
  if w<=0: _fail("write","zero write")
  off+=w
def _copy(src,dst,exp_bytes,exp_sha,final_mode,limit):
 _check_sha(exp_sha,"copy sha")
 if exp_bytes>limit: _fail("cap","copy exceeds cap")
 sfd=_open_abs(src); pfd=None; dfd=None; created=None
 try:
  _verify_fd(sfd,exp_bytes,exp_sha,"source "+src)
  os.lseek(sfd,0,os.SEEK_SET)
  pfd,name=_open_parent(dst)
  try: dfd=os.open(name,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW|os.O_CLOEXEC,0o600,dir_fd=pfd)
  except OSError as e: _fail("create","destination exists or create failed: "+str(e))
  created=_dict(os.fstat(dfd))
  h=hashlib.sha256(); n=0
  while True:
   b=os.read(sfd,_CHUNK)
   if not b: break
   n+=len(b)
   if n>limit or n>exp_bytes: _fail("cap","copy size cap")
   h.update(b); _write_all(dfd,b)
  if n!=exp_bytes or h.hexdigest()!=exp_sha: _fail("identity","copy sha/size mismatch")
  os.fsync(dfd); os.fchmod(dfd,final_mode)
  dstmeta=_dict(os.fstat(dfd))
  if dstmeta["size"]!=exp_bytes or dstmeta["mode"]!=final_mode: _fail("verify","destination mismatch")
  post=os.fstat(sfd)
  os.close(dfd); dfd=None; os.fsync(pfd)
  return dstmeta
 except BaseException:
  if dfd is not None:
   try:
    now=os.fstat(dfd)
    if created and (now.st_dev,now.st_ino)==(created["dev"],created["ino"]): os.unlink(name,dir_fd=pfd); os.fsync(pfd)
   except OSError: pass
   try: os.close(dfd)
   except OSError: pass
  if isinstance(_exc(),CustodyError): raise
  _fail("copy",str(_exc()))
 finally:
  os.close(sfd)
  if pfd is not None: os.close(pfd)
def _exc():
 import sys; return sys.exc_info()[1]
def _layout_check():
 for rel,mode in (("",0o755),)+T6_LAYOUT:
  path=T6 if rel=="" else T6+"/"+rel
  fd=_open_abs(path,True)
  try:
   st=os.fstat(fd)
   if _mode(st)!=mode or st.st_uid!=os.getuid(): _fail("layout","bad layout dir "+path)
  finally: os.close(fd)
def _write_layout_status(made,error):
 try: tfd=_open_abs(T6,True)
 except CustodyError: return False
 try:
  body=json.dumps({"status":"BLOCKED","made":list(made),"error":str(error)},sort_keys=True).encode("ascii")
  fd=None
  try:
   fd=os.open(LAYOUT_STATUS_NAME,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW|os.O_CLOEXEC,0o600,dir_fd=tfd); _write_all(fd,body); os.fsync(fd); os.fchmod(fd,0o444)
  finally:
   if fd is not None: os.close(fd)
  os.fsync(tfd); return True
 except Exception: return False
 finally: os.close(tfd)
def create_t6_layout():
 if os.path.lexists(T6): _fail("layout","T6 exists; fresh-only, preserved")
 created=False
 bfd=_open_base_parent(BASE)
 try:
  try: os.mkdir("current-cardano-k2-k6-proof-roundtrip-20261016",0o755,dir_fd=bfd)
  except OSError as e: _fail("layout","create T6 failed: "+str(e))
  created=True
  os.fsync(bfd)
 finally: os.close(bfd)
 made=[]
 try:
  rfd=_open_abs(T6,True)
  try:
   os.fchmod(rfd,0o755); rst=os.fstat(rfd)
   if _mode(rst)!=0o755 or rst.st_uid!=os.getuid(): _fail("layout","T6 root mode/owner mismatch")
   os.fsync(rfd)
  finally: os.close(rfd)
  for rel,mode in T6_LAYOUT:
   parent,_,name= (T6+"/"+rel).rpartition("/")
   pfd=_open_abs(parent,True) if parent!=T6 else _open_abs(T6,True)
   try:
    try: os.mkdir(name,mode,dir_fd=pfd)
    except OSError as e: _fail("layout","mkdir failed "+rel+": "+str(e))
    dfd=os.open(name,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW,dir_fd=pfd); os.fchmod(dfd,mode); os.fsync(dfd); os.close(dfd); os.fsync(pfd)
   finally: os.close(pfd)
   made.append(rel)
  _layout_check()
  body=("\n".join(made)+"\n").encode("ascii")
  pfd=_open_abs(T6_LOGS,True); dfd=None
  try:
   dfd=os.open("custody-layout.log",os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=pfd); _write_all(dfd,body); os.fsync(dfd); os.fchmod(dfd,0o444); os.close(dfd); dfd=None; os.fsync(pfd)
  finally:
   if dfd is not None: os.close(dfd)
   os.close(pfd)
  return {"layout":made,"log":LAYOUT_LOG}
 except CustodyError:
  if created: _write_layout_status(made,_exc())
  raise
 except OSError as e:
  if created: _write_layout_status(made,e)
  _fail("layout",str(e))
def verify_runtime_preconditions():
 root,_=_read_pinned(ROOT26_PATH,ROOT26_BYTES,ROOT26_SHA256,"ROOT26",MAX_PUBLIC_BYTES)
 doc=_strict_json(root,"ROOT26")
 if not isinstance(doc,dict) or doc.get("format")!="root-K5-source-materialization/1": _fail("identity","ROOT26 format mismatch")
 files=doc.get("files")
 if not isinstance(files,list) or len(files)!=ROOT26_COUNT: _fail("identity","ROOT26 count mismatch")
 seen=set(); k5m_match=0
 for f in files:
  if not isinstance(f,dict) or set(f)!={"path","sha256"}: _fail("identity","ROOT26 entry keys mismatch")
  if not isinstance(f["path"],str) or f["path"] in seen: _fail("identity","ROOT26 duplicate path")
  seen.add(f["path"]); _check_sha(f["sha256"],"ROOT26 entry sha")
  if f["path"]=="operator/request/steps/K5M.sh" and f["sha256"]==K5M_SHA256: k5m_match+=1
 if k5m_match!=1: _fail("identity","ROOT26 lacks exact K5M path/sha")
 k5m,_=_read_pinned(K5M_PATH,K5M_BYTES,K5M_SHA256,"K5M",MAX_PUBLIC_BYTES)
 text=k5m.decode("ascii")
 for lit in _K5M_LITERALS:
  if lit not in text: _fail("identity","K5M literal missing")
 gdata,_=_read_pinned(K5G_PATH,K5G_BYTES,K5G_SHA256,"K5g",MAX_PUBLIC_BYTES)
 g=_strict_json(gdata,"K5g")
 if not isinstance(g,dict) or g.get("format")!="k5g-gate/1" or g.get("verdict")!="PASS": _fail("identity","K5g format/verdict mismatch")
 recs=g.get("records")
 if not isinstance(recs,dict): _fail("identity","K5g records missing")
 sr=recs.get("setupRecord")
 if not isinstance(sr,dict): _fail("identity","K5g setupRecord missing")
 if type(sr.get("bytes")) is not int or sr.get("bytes")!=RECORD_BYTES: _fail("identity","K5g setupRecord bytes mismatch")
 _check_sha(sr.get("sha256"),"K5g setupRecord sha")
 if sr.get("sha256")!=RECORD_SHA256: _fail("identity","K5g setupRecord sha mismatch")
 return {"root26":ROOT26_SHA256,"k5m":K5M_SHA256,"k5g":K5G_SHA256}
def fixture_metadata(name):
 if name not in PUBLIC_FIXTURES: _fail("allowlist","unknown fixture")
 path,n,sha=PUBLIC_FIXTURES[name]; fd=_open_abs(path)
 try: return _verify_fd(fd,n,sha,name)
 finally: os.close(fd)
def read_public_fixture(name):
 if name not in PUBLIC_FIXTURES: _fail("allowlist","unknown fixture")
 path,n,sha=PUBLIC_FIXTURES[name]
 if n>MAX_PUBLIC_BYTES: _fail("cap","fixture cap")
 data,_=_read_pinned(path,n,sha,name,MAX_PUBLIC_BYTES); return data
def prepare_inputs():
 verify_runtime_preconditions(); _layout_check()
 for name in FIXTURE_ORDER: fixture_metadata(name)
 out={}
 for name in FIXTURE_ORDER:
  path,n,sha=PUBLIC_FIXTURES[name]; out[name]=_copy(path,T6_IN+"/"+name,n,sha,0o444,MAX_PUBLIC_BYTES)
 out["binary"]=_copy(BINARY_SRC,BINARY_DST,BINARY_BYTES,BINARY_SHA256,0o555,MAX_BINARY_BYTES)
 out["record"]=_copy(RECORD_SRC,RECORD_DST,RECORD_BYTES,RECORD_SHA256,0o444,MAX_RECORD_BYTES)
 return out
def key_precheck():
 fd=_open_abs(KEYS_DIR,True)
 try:
  dst=os.fstat(fd)
  if _mode(dst)!=KEYS_DIR_MODE or dst.st_uid!=os.getuid(): _fail("keys","keys dir mode/owner")
  names=os.listdir(fd)
  if names!=[PK_NAME]: _fail("keys","keys dir must contain exactly PK")
  pst=os.stat(PK_NAME,dir_fd=fd,follow_symlinks=False)
  if not stat.S_ISREG(pst.st_mode) or _mode(pst)!=PK_MODE or pst.st_nlink!=1 or pst.st_size!=PK_BYTES or pst.st_uid!=os.getuid(): _fail("keys","PK metadata mismatch")
  return {"dir":_dict(dst),"pk":_dict(pst)}
 finally: os.close(fd)
def key_postcheck(expected,stage):
 if stage not in POSTCHECK_STAGES: _fail("stage","bad postcheck stage")
 if not isinstance(expected,dict): _fail("identity","expected metadata required")
 now=key_precheck()
 if now!=expected: _fail("stable","PK metadata changed at "+stage)
 return now
def public_proof_export(expected_sha,expected_bytes):
 _check_sha(expected_sha,"proof sha")
 if not isinstance(expected_bytes,int) or expected_bytes<=0 or expected_bytes>MAX_PROOF_BYTES: _fail("cap","proof bytes cap")
 _layout_check(); return _copy(PROOF_OUT,PROOF_RECEIPT,expected_bytes,expected_sha,0o444,MAX_PROOF_BYTES)
def _strict_json(data,label):
 def hook(pairs):
  o={}
  for k,v in pairs:
   if k in o: _fail("json",label+" duplicate key")
   o[k]=v
  return o
 try: return json.loads(data.decode("ascii"),object_pairs_hook=hook)
 except CustodyError: raise
 except Exception: _fail("json",label+" not strict ASCII JSON")
def _variant_file(variant): return T6_STAGE+"/"+RECORD_VARIANT_FILES[variant]
def record_variant_metadata(variant,meta):
 if variant not in RECORD_VARIANT_FILES: _fail("allowlist","unknown variant")
 if not isinstance(meta,dict) or "sha256" not in meta or "bytes" not in meta: _fail("identity","variant sha/bytes required")
 if any(k in meta for k in ("pointer","old","new")): _fail("identity","caller may not override edit")
 _check_sha(meta["sha256"],"variant sha")
 if type(meta["bytes"]) is not int or meta["bytes"]<0 or meta["bytes"]>MAX_VARIANT_BYTES: _fail("cap","variant bytes cap")
 rec,_=_read_pinned(RECORD_DST,RECORD_BYTES,RECORD_SHA256,"staged record",MAX_RECORD_BYTES)
 vpath=_variant_file(variant); vfd=_open_abs(vpath)
 try: vmeta=_verify_fd(vfd,meta["bytes"],meta["sha256"],variant,0o444)
 finally: os.close(vfd)
 vdata,_=_read_pinned(vpath,meta["bytes"],meta["sha256"],variant,MAX_VARIANT_BYTES)
 a=_strict_json(rec,"record"); b=_strict_json(vdata,variant)
 if not isinstance(a,dict) or not isinstance(b,dict) or set(a)!=set(b): _fail("identity","variant key set differs")
 spec=RECORD_VARIANT_EDITS[variant]
 if not spec["pointer"].startswith("/") or "/" in spec["pointer"][1:] or "~" in spec["pointer"]: _fail("identity","variant pointer not single JSON pointer")
 key=spec["pointer"][1:]
 diff=[k for k in a if type(a[k]) is not type(b[k]) or a[k]!=b[k]]
 if diff!=[key]: _fail("identity","variant not exact single edit")
 if type(a.get(key)) is not type(spec["old"]) or a.get(key)!=spec["old"]: _fail("identity","variant old value mismatch")
 if variant=="N11" and (type(b.get(key)) is not int or b.get(key)!=spec["new"]): _fail("identity","N11 new value mismatch")
 if variant=="N8":
  _check_sha(b.get(key),"N8 new digest")
  if b[key]==spec["old"] or sum(1 for x,y in zip(b[key],spec["old"]) if x!=y)!=1: _fail("identity","N8 digest must differ in exactly one character")
 return {"variant":variant,"pointer":spec["pointer"],"old":spec["old"],"new":b[key],"bytes":vmeta["size"],"sha256":meta["sha256"],"mode":vmeta["mode"]}
def _expect(exp,label):
 if not isinstance(exp,dict): _fail("identity",label+" expected dict required")
 out={}
 for k,v in exp.items():
  if not isinstance(v,dict) or "sha256" not in v or "bytes" not in v: _fail("identity",label+" sha/bytes required per item")
  _check_sha(v["sha256"],label); out[k]=(v["sha256"],v["bytes"])
 return out
def export_negative_variants(expected):
 exp=_expect(expected,"negatives")
 if frozenset(exp)!=EXPECTED_EXPORT_SET: _fail("allowlist","exact N1-N7,N8,N10,N11 set required")
 _layout_check(); out={}
 for nid in ("N1","N2","N3","N4","N5","N6","N7"):
  sha,n=exp[nid]; out[nid]=_copy(T6_STAGE+"/"+NEGATIVE_PROOF_FILES[nid],T6_NEGATIVES+"/"+NEGATIVE_PROOF_FILES[nid],n,sha,0o444,MAX_PROOF_BYTES)
 sha,n=exp["N10"]; out["N10"]=_copy(T6_STAGE+"/"+NEGATIVE_REQUEST_FILES["N10"],T6_NEGATIVES+"/"+NEGATIVE_REQUEST_FILES["N10"],n,sha,0o444,MAX_PUBLIC_BYTES)
 for nid in ("N8","N11"):
  sha,n=exp[nid]; out[nid]=record_variant_metadata(nid,{"sha256":sha,"bytes":n})
 return out
def _freeze_walk():
 files={}; dirs=set()
 for root in _FREEZE_ROOTS:
  stack=[root]
  while stack:
   rel=stack.pop(); dirs.add(rel); fd=_open_abs(T6+"/"+rel,True)
   try:
    for name in os.listdir(fd):
     if name==FREEZE_MANIFEST_NAME and rel=="receipts": _fail("freeze","manifest must not be allowlisted")
     child=rel+"/"+name; st=os.stat(name,dir_fd=fd,follow_symlinks=False)
     low=child.lower()
     if any(t in low for t in _FORBIDDEN_TOK) or any(c in _FORBIDDEN_COMP for c in child.split("/")): _fail("freeze","forbidden path "+child)
     if stat.S_ISLNK(st.st_mode): _fail("freeze","link "+child)
     if stat.S_ISDIR(st.st_mode): stack.append(child)
     elif stat.S_ISREG(st.st_mode):
      if st.st_nlink!=1 or st.st_uid!=os.getuid(): _fail("freeze","bad file "+child)
      files[child]=st
     else: _fail("freeze","non-regular "+child)
   finally: os.close(fd)
 return files,dirs
def freeze_public_results(expected_allowlist):
 if not isinstance(expected_allowlist,tuple): _fail("freeze","allowlist must be immutable tuple")
 exp={}
 for item in expected_allowlist:
  if not isinstance(item,tuple) or len(item)!=3: _fail("freeze","allowlist item (path,bytes,sha256)")
  rel,n,sha=item; _check_sha(sha,"freeze sha")
  if not isinstance(rel,str) or rel.startswith("/") or ".." in rel.split("/") or rel.split("/")[0] not in _FREEZE_ROOTS: _fail("freeze","bad freeze path")
  if rel in exp: _fail("freeze","duplicate freeze path")
  if not isinstance(n,int) or n<0 or n>MAX_FREEZE_FILE_BYTES: _fail("freeze","freeze file cap")
  exp[rel]=(n,sha)
 if len(exp)>MAX_FREEZE_FILES: _fail("freeze","freeze count cap")
 if sum(n for n,_ in exp.values())!=sum(n for n,_ in exp.values()) or sum(n for n,_ in exp.values())>MAX_FREEZE_AGGREGATE_BYTES: _fail("freeze","freeze aggregate cap")
 # A failure archive must freeze logs/journal/receipts honestly even when no proof was ever produced; never require or fabricate a proof receipt here. The caller records the run status separately and a missing proof keeps the run BLOCKED/FAILED/INCOMPLETE.
 _layout_check(); files,dirs=_freeze_walk()
 expected_dirs=set(_FREEZE_ROOTS)
 expected_dirs.add("receipts/negatives")
 for rel in exp:
  d=rel
  while "/" in d:
   d=d.rsplit("/",1)[0]; expected_dirs.add(d)
 if dirs!=expected_dirs: _fail("freeze","unexpected or missing directory")
 if set(files)!=set(exp): _fail("freeze","unexpected or missing file")
 rows=[]; by_rel={}
 for rel in sorted(exp):
  n,sha=exp[rel]; fd=_open_abs(T6+"/"+rel)
  try: meta=_verify_fd(fd,n,sha,rel)
  finally: os.close(fd)
  row={"path":rel,"bytes":n,"sha256":sha,"mode_before":meta["mode"],"mode_after":meta["mode"]}; rows.append(row); by_rel[rel]=row
 manifest={"status":"FROZEN","files":rows}
 try:
  for rel in sorted(exp):
   pfd,name=_open_parent(T6+"/"+rel); ffd=None
   try:
    ffd=os.open(name,os.O_RDONLY|os.O_NOFOLLOW|os.O_CLOEXEC,dir_fd=pfd); os.fchmod(ffd,0o444); by_rel[rel]["mode_after"]=_mode(os.fstat(ffd))
   finally:
    if ffd is not None: os.close(ffd)
    os.close(pfd)
   if by_rel[rel]["mode_after"]!=0o444: _fail("freeze","chmod verify failed "+rel)
 except Exception as e:
  blocked={"status":"BLOCKED","files":rows,"error":str(e)}
  try:
   pfd=_open_abs(T6_RECEIPTS,True); mfd=None
   try:
    mfd=os.open(FREEZE_MANIFEST_NAME,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=pfd); _write_all(mfd,json.dumps(blocked,sort_keys=True).encode("ascii")); os.fsync(mfd); os.fchmod(mfd,0o444)
   finally:
    if mfd is not None: os.close(mfd)
    os.close(pfd)
  except Exception: pass
  if isinstance(e,CustodyError): raise
  _fail("freeze","partial freeze BLOCKED: "+str(e))
 body=json.dumps(manifest,sort_keys=True,indent=1).encode("ascii")
 pfd=_open_abs(T6_RECEIPTS,True)
 try:
  if FREEZE_MANIFEST_NAME in os.listdir(pfd): _fail("freeze","manifest exists")
  fd=os.open(FREEZE_MANIFEST_NAME,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=pfd); _write_all(fd,body); os.fsync(fd); os.fchmod(fd,0o444); os.close(fd); os.fsync(pfd)
 finally: os.close(pfd)
 return manifest
