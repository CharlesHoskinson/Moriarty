"""K7 public verifier-record export. Source-only; importing does no I/O.
Future use only under a fresh exact root AUTH after K6 actual adoption
and a fresh non-author review of the populated AUTH. One fd-pinned
protected setup-record read; no PK path/stat/hash/read/copy/delete,
no subprocess, network, native, SDK, RNG, wallet, signing or ledger.
Exports the already-public 3926-byte setup record only."""
import hashlib,json,os,stat,sys
BASE="/home/charl/moriarty-eutxo-validation-2026-10-12"
_T6=BASE+"/current-cardano-k2-k6-proof-roundtrip-20261016"
_SRC=_T6+"/w/in/invoice-partial.setup.json"
_M7=BASE+"/current-cardano-k2-k7-public-verifier-export-materialized-20261016"
_SELF=_M7+"/operator.py"
_RR7=BASE+"/current-cardano-k2-k7-public-verifier-export-root-records-20261016"
_AUTH=_RR7+"/AUTH.json"; _ADOPT=BASE+"/current-cardano-k2-k6-integrated-operator-root-records-20261016/K6-ACTUAL-ADOPTION.json"
_LAUNCH=BASE+"/current-cardano-k2-k6-root-records-20261016/LAUNCH-RECORD.json"
_PROOF=_T6+"/receipts/invoice-partial.proof.json"
_DEST=BASE+"/current-cardano-k2-k7-public-verifier-record-20261016"
_RDIR=_DEST+"/receipts"; _REC=_RDIR+"/invoice-partial.setup.json"
_RECEIPT=_RDIR+"/EXPORT-RECEIPT.json"
_SRC_BYTES=3926; _PROOF_BYTES=5800
_SRC_SHA="86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b"
_PROOF_SHA="930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b"
_VK_DIGEST="d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd"
_PK_BYTES=98715456; _VK_BYTES=488
_SCOPE="one public invoice-partial setup-record export only; not a crypto PASS, not ledger/Preprod"
_FMT="moriarty-k2-groth16-setup/1"; _PROFILE="single-party-test-setup/1"
_NOTICE="TEST-ONLY single-party Groth16 setup sampled from OsRng; not a ceremony. Whoever controlled the setup process could forge proofs for this relation. VK provenance is declared by this record; no verifier can check that the VK was generated from r1csSha256."
_CURVE="BLS12-381"; _SNARK="Groth16 (ark-groth16 =0.4.0, LibsnarkReduction)"
_ENCODING="arkworks 0.4 compressed points (Zcash BLS12-381 layout); VK 488 bytes, proof 192 bytes"
_EFFECTS={"attempts":1,"nativeCalls":0,"networkCalls":0,"pkByteReads":0,"pkHashes":0,"pkMetadataReads":0,"protectedRecordReads":1,"retries":0,"rngCalls":0,"subprocessCalls":0}
_CAPS={"asBytes":268435456,"coreBytes":0,"cpuHardSeconds":15,"cpuSoftSeconds":10,"wallSeconds":30}
_AUTH_FIELDS=frozenset(("authFormat","caps","destinationPath","effects","executionAuthorized","k6ActualAdoptionPath","k6ActualAdoptionSha256","k6LaunchRecordPath","k6LaunchRecordSha256","k6ProofBytes","k6ProofPath","k6ProofSha256","operatorPath","operatorSha256","scope","sourceBytes","sourcePath","sourceSha256"))
_RECORD_FIELDS=frozenset(("curve","driver","encoding","format","notice","provingKeyBytes","provingKeySha256","relation","snark","trustProfile","verifyingKeyHex","vkDigestBlake2b256"))
_REL_FIELDS=frozenset(("abi2ShapeBytesHex","abi2ShapeDigest","action","actionIndex","adapter","bodyLen","compilerBuildDigest","compilerSourceManifestSha256","constraints","coreClosureSha256","familyIdHex","fieldModulusLeHex","headerLen","instanceVariables","language","logicalCostProfile","logicalWork","predicateDigest","profileDigest","programHash","publicInputCount","r1csSha256","relationFormat","relationShapeDigest","relationVersion","semantics","sourceSha256","statementTagHex","witnessVariables"))
_HEX=frozenset("0123456789abcdef"); _DEPTH=len([c for c in BASE.split("/") if c])
class _Fail(Exception): pass
def _fail(m): raise _Fail(m)
def _is_sha(v): return isinstance(v,str) and len(v)==64 and all(c in _HEX for c in v)
def _need_sha(v,m):
 if not _is_sha(v): _fail(m)
 return v
def _ident(st): return (st.st_dev,st.st_ino,st.st_nlink,stat.S_IMODE(st.st_mode),st.st_size,st.st_mtime_ns,st.st_ctime_ns,st.st_uid,st.st_gid)
def _json(data,label):
 def hook(pairs):
  o={}
  for k,v in pairs:
   if k in o: _fail(label+" duplicate key")
   o[k]=v
  return o
 try: return json.loads(data.decode("ascii"),object_pairs_hook=hook)
 except _Fail: raise
 except Exception: _fail(label+" not strict ASCII JSON")
def _parts(path):
 if not isinstance(path,str) or (path!=BASE and not path.startswith(BASE+"/")): _fail("path outside BASE")
 p=[c for c in path.split("/") if c]
 if any(c in (".","..") for c in p): _fail("dot component")
 return p
def _open_abs(path,directory=False):
 parts=_parts(path); fd=None
 try:
  fd=os.open("/",os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW|os.O_CLOEXEC)
  for i,c in enumerate(parts):
   last=i==len(parts)-1
   want_dir=directory if last else True
   nf=os.open(c,os.O_RDONLY|(os.O_DIRECTORY if want_dir else 0)|os.O_NOFOLLOW|os.O_CLOEXEC,dir_fd=fd)
   os.close(fd); fd=nf; st=os.fstat(fd)
   if i>=_DEPTH-1 and st.st_uid!=os.getuid(): _fail("component not operator-owned")
   if want_dir and not stat.S_ISDIR(st.st_mode): _fail("not a directory")
   if last and not directory and not stat.S_ISREG(st.st_mode): _fail("not a regular file")
  return fd
 except _Fail:
  if fd is not None:
   try: os.close(fd)
   except OSError: pass
  raise
 except OSError as e:
  if fd is not None:
   try: os.close(fd)
   except OSError: pass
  _fail("open: "+str(e))
def _read_fixed(path,cap,label,exp_sha=None,exp_bytes=None):
 fd=_open_abs(path)
 try:
  pre=os.fstat(fd)
  if not stat.S_ISREG(pre.st_mode) or pre.st_nlink!=1 or pre.st_uid!=os.getuid(): _fail(label+" not regular nlink1 operator-owned")
  if exp_bytes is not None and pre.st_size!=exp_bytes: _fail(label+" size mismatch")
  if pre.st_size>cap: _fail(label+" over cap")
  data=b""
  while len(data)<=cap:
   b=os.read(fd,min(65536,cap+1-len(data)))
   if not b: break
   data+=b
  if len(data)>cap: _fail(label+" read overflow")
  post=os.fstat(fd)
  if _ident(pre)!=_ident(post): _fail(label+" changed while reading")
  if exp_bytes is not None and len(data)!=exp_bytes: _fail(label+" short read")
  if exp_sha is not None and hashlib.sha256(data).hexdigest()!=exp_sha: _fail(label+" sha mismatch")
  return data
 finally: os.close(fd)
def _write_all(fd,data):
 v=memoryview(data); off=0
 while off<len(v):
  w=os.write(fd,v[off:])
  if w<=0: _fail("zero write")
  off+=w
def _check_auth(raw,expected_sha):
 if hashlib.sha256(raw).hexdigest()!=expected_sha: _fail("AUTH sha mismatch")
 a=_json(raw,"AUTH")
 if not isinstance(a,dict) or frozenset(a)!=_AUTH_FIELDS: _fail("AUTH field set mismatch")
 if a["authFormat"]!="k7-public-verifier-record-export-auth/1": _fail("AUTH format")
 if a["executionAuthorized"] is not True: _fail("AUTH not executionAuthorized")
 if a["scope"]!=_SCOPE: _fail("AUTH scope")
 if a["operatorPath"]!=_SELF or a["sourcePath"]!=_SRC or a["destinationPath"]!=_DEST: _fail("AUTH path mismatch")
 if a["k6LaunchRecordPath"]!=_LAUNCH or a["k6ProofPath"]!=_PROOF or a["k6ActualAdoptionPath"]!=_ADOPT: _fail("AUTH public path mismatch")
 if a["sourceBytes"]!=_SRC_BYTES or a["sourceSha256"]!=_SRC_SHA: _fail("AUTH source identity")
 if a["k6ProofBytes"]!=_PROOF_BYTES or a["k6ProofSha256"]!=_PROOF_SHA: _fail("AUTH proof identity")
 _need_sha(a["operatorSha256"],"AUTH operatorSha256"); _need_sha(a["k6LaunchRecordSha256"],"AUTH launch sha"); _need_sha(a["k6ActualAdoptionSha256"],"AUTH adoption sha")
 if a["effects"]!=_EFFECTS or a["caps"]!=_CAPS: _fail("AUTH effects/caps mismatch")
 return a
def _check_public(auth):
 launch=_json(_read_fixed(_LAUNCH,8192,"launch record",auth["k6LaunchRecordSha256"]),"launch record")
 if not isinstance(launch,dict) or launch.get("format")!="root-k6-launch-record/1" or launch.get("status")!="PASS" or launch.get("parentExit")!=0: _fail("launch record not PASS/parentExit 0")
 proof=_json(_read_fixed(_PROOF,16384,"public proof",_PROOF_SHA,_PROOF_BYTES),"public proof")
 if not isinstance(proof,dict) or proof.get("format")!="moriarty-k2-groth16-proof/1": _fail("proof format")
 if proof.get("setupRecordSha256")!=_SRC_SHA or proof.get("vkDigestBlake2b256")!=_VK_DIGEST: _fail("proof record/VK binding")
 if proof.get("action")!="partial" or proof.get("actionIndex")!=1: _fail("proof action")
 ph=proof.get("proofHex")
 if not isinstance(ph,str) or len(ph)!=384 or any(c not in _HEX for c in ph): _fail("proofHex not 192-byte lowercase hex")
 for k in ("programHash","sourceSha256","compilerBuildDigest","r1csSha256","predicateDigest","abi2ShapeDigest","statementDigestBlake2b256"): _need_sha(proof.get(k),"proof "+k)
 adopt=_json(_read_fixed(_ADOPT,8192,"K6 adoption",auth["k6ActualAdoptionSha256"]),"K6 adoption")
 if not isinstance(adopt,dict): _fail("adoption not an object")
 if adopt.get("format")!="root-K6-native-roundtrip-actual-adoption/1": _fail("adoption format")
 if adopt.get("nativeRoundtripPrerequisiteClosed") is not True or adopt.get("actualResultAdopted") is not True: _fail("adoption gate not closed")
 if adopt.get("executionAuthorityGranted") is not False: _fail("adoption grants no execution")
 obs=adopt.get("acceptedObservations")
 if not isinstance(obs,dict) or obs.get("proofExportSha256")!=_PROOF_SHA or obs.get("setupRecordSha256")!=_SRC_SHA: _fail("adoption accepted-observations binding")
 return proof
def _check_record(data,proof):
 if hashlib.sha256(data).hexdigest()!=_SRC_SHA: _fail("setup record sha mismatch")
 r=_json(data,"setup record")
 if not isinstance(r,dict) or frozenset(r)!=_RECORD_FIELDS: _fail("setup record field set mismatch")
 fixed=(("format",_FMT),("trustProfile",_PROFILE),("notice",_NOTICE),("curve",_CURVE),("snark",_SNARK),("encoding",_ENCODING))
 for k,v in fixed:
  if r[k]!=v: _fail("setup record "+k)
 if not isinstance(r["driver"],str) or not r["driver"]: _fail("setup record driver")
 rel=r["relation"]
 if not isinstance(rel,dict) or frozenset(rel)!=_REL_FIELDS: _fail("relation field set mismatch")
 pairs=(("programHash","programHash"),("sourceSha256","sourceSha256"),("action","action"),("actionIndex","actionIndex"),("compilerBuildDigest","compilerBuildDigest"),("r1csSha256","r1csSha256"),("predicateDigest","predicateDigest"),("abi2ShapeDigest","abi2ShapeDigest"))
 for rk,pk in pairs:
  if rel[rk]!=proof[pk]: _fail("relation/proof mismatch "+rk)
 vk=r["verifyingKeyHex"]
 if not isinstance(vk,str) or len(vk)!=2*_VK_BYTES or any(c not in _HEX for c in vk): _fail("verifyingKeyHex not 488-byte lowercase hex")
 vkb=bytes.fromhex(vk)
 if len(vkb)!=_VK_BYTES or hashlib.blake2b(vkb,digest_size=32).hexdigest()!=_VK_DIGEST: _fail("VK Blake2b-256 mismatch")
 if r["vkDigestBlake2b256"]!=_VK_DIGEST: _fail("record vkDigest mismatch")
 _need_sha(r["provingKeySha256"],"provingKeySha256 syntax")
 if type(r["provingKeyBytes"]) is not int or r["provingKeyBytes"]!=_PK_BYTES: _fail("provingKeyBytes metadata mismatch")
 return r
def _export(data,proof,auth,auth_sha):
 bfd=_open_abs(BASE,True)
 try:
  try: os.mkdir("current-cardano-k2-k7-public-verifier-record-20261016",0o755,dir_fd=bfd)
  except OSError as e: _fail("destination create: "+str(e))
  os.fsync(bfd)
 finally: os.close(bfd)
 dfd=_open_abs(_DEST,True); rfd=None
 try:
  try: os.mkdir("receipts",0o755,dir_fd=dfd)
  except OSError as e: _fail("receipts create: "+str(e))
  os.fsync(dfd); rfd=_open_abs(_RDIR,True)
  fd=os.open("invoice-partial.setup.json",os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW|os.O_CLOEXEC,0o600,dir_fd=rfd)
  try: _write_all(fd,data); os.fsync(fd); os.fchmod(fd,0o444)
  finally: os.close(fd)
  receipt={"action":proof["action"],"actionIndex":proof["actionIndex"],"authSha256":auth_sha,"compilerBuildDigest":proof["compilerBuildDigest"],"destinationPath":_DEST,"effects":dict(_EFFECTS),"format":"k7-public-verifier-record-export-receipt/1","k6ActualAdoptionSha256":auth["k6ActualAdoptionSha256"],"k6LaunchRecordSha256":auth["k6LaunchRecordSha256"],"k6ProofBytes":_PROOF_BYTES,"k6ProofSha256":_PROOF_SHA,"operatorSha256":auth["operatorSha256"],"predicateDigest":proof["predicateDigest"],"programHash":proof["programHash"],"r1csSha256":proof["r1csSha256"],"setupRecordBytes":_SRC_BYTES,"setupRecordSha256":_SRC_SHA,"sourceSha256":proof["sourceSha256"],"status":"PUBLIC_VERIFIER_RECORD_EXPORTED","verifyingKeyBytes":_VK_BYTES,"vkDigestBlake2b256":_VK_DIGEST}
  body=(json.dumps(receipt,sort_keys=True,indent=1)+"\n").encode("ascii")
  fd=os.open("EXPORT-RECEIPT.json",os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW|os.O_CLOEXEC,0o600,dir_fd=rfd)
  try: _write_all(fd,body); os.fsync(fd); os.fchmod(fd,0o444)
  finally: os.close(fd)
  os.fsync(rfd); return receipt
 finally:
  if rfd is not None: os.close(rfd)
  os.close(dfd)
def _main(argv):
 if len(argv)!=3 or argv[1]!="--expected-auth-sha256" or not _is_sha(argv[2]): _fail("usage: operator.py --expected-auth-sha256 HEX")
 auth=_check_auth(_read_fixed(_AUTH,16384,"AUTH",argv[2]),argv[2])
 _read_fixed(_SELF,65536,"own source",auth["operatorSha256"])
 proof=_check_public(auth)
 data=_read_fixed(_SRC,_SRC_BYTES+1,"protected setup record",_SRC_SHA,_SRC_BYTES)
 _check_record(data,proof)
 receipt=_export(data,proof,auth,argv[2])
 sys.stdout.write(json.dumps(receipt,sort_keys=True)+"\n"); return 0
if __name__=="__main__":
 try: sys.exit(_main(sys.argv))
 except _Fail as e: sys.stdout.write("FAIL: "+str(e)+"\n"); sys.exit(1)
 except OSError as e: sys.stdout.write("FAIL: "+str(e)+"\n"); sys.exit(1)