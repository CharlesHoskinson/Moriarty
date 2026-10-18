from pathlib import Path
import re,json,hashlib,subprocess
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');A=R/'financial-source-check-cases-terminal-review-20261018';P=A/'packet';D=R/'concurrency-increase-main238/financial-review-fidelity';K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
patch=(P/'author-freeze/ORIGINAL-e355.patch').read_text();files=[]
for chunk in re.split(r'(?m)^--- ',patch)[1:]:
 lines=chunk.splitlines(keepends=True);path=lines[1].strip().removeprefix('+++ b/');basefile=P/'base'/path;old=basefile.read_text().splitlines(keepends=True)if basefile.exists()else[];new=[];pos=0;i=1
 while i<len(lines):
  if not lines[i].startswith('@@ '):i+=1;continue
  h=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i]);start=int(h[1]);oldcount=int(h[2] or 1);newcount=int(h[4] or 1);at=max(0,start-1);new+=old[pos:at];pos=at;i+=1;oc=nc=0
  while i<len(lines)and not lines[i].startswith('@@ '):
   line=lines[i];i+=1
   if line.startswith(' '):assert old[pos]==line[1:];new.append(line[1:]);pos+=1;oc+=1;nc+=1
   elif line.startswith('-'):assert old[pos]==line[1:];pos+=1;oc+=1
   elif line.startswith('+'):new.append(line[1:]);nc+=1
   elif line.startswith('\\ No newline'):new[-1]=new[-1].removesuffix('\n')
   else:raise AssertionError((path,line))
  assert(oc,nc)==(oldcount,newcount),(path,oc,nc,oldcount,newcount)
 new+=old[pos:];raw=''.join(new).encode();current=(P/'head'/path).read_bytes();is_test=path=='crates/mori/tests/check_source_dispatch.rs'
 assert raw==(current+b'\n'if is_test else current),path
 files.append({'path':path,'originalPatchReconstructedSha256':hashlib.sha256(raw).hexdigest(),'currentSha256':hashlib.sha256(current).hexdigest(),'delta':'one extra final newline in original'if is_test else'byte-identical'})
assert len(files)==15
actual=json.loads((P/'actual-receipts/SOURCE-PINS.json').read_text());head=actual['head'];payload=''.join(head+':'+f['path']+'\n'for f in actual['files']).encode();out=subprocess.check_output(['git','cat-file','--batch'],cwd=K,input=payload);at=0
for f in actual['files']:
 e=out.index(b'\n',at);header=out[at:e].decode().split();assert header[1]=='blob';size=int(header[2]);raw=out[e+1:e+1+size];assert hashlib.sha256(raw).hexdigest()==f['sha256'],f['path'];at=e+size+2
assert at==len(out)
def objects(path):
 text=path.read_text();dec=json.JSONDecoder();at=1;parts=[]
 while True:
  while text[at].isspace()or text[at]==',':at+=1
  if text[at]==']':return parts
  _,end=dec.raw_decode(text,at);parts.append(text[at:end]);at=end
assert objects(P/'base/crates/fixtures/check-source/cases.json')==objects(P/'head/crates/fixtures/check-source/cases.json')[:8]
x={'originalPatchSha256Verified':sha(P/'author-freeze/ORIGINAL-e355.patch')=='e355f2381c42e7ab3c4de5fb8ea7ce975bd084879b64d2d3e62b6a905174a92f','all15OriginalPatchBodiesReconstructed':files,'all873RecordedPublicSourcePinHashesCheckedAgainstExactHeadGitBlobs':len(actual['files']),'scopeOf873':'Recorded driver-selected files under crates plus Cargo.toml/Cargo.lock/rust-toolchain.toml/docs/NATIVE-CORE.md, not semantic inspection of all source dependencies','originalEightManifestRawObjectBodiesUnchanged':True,'wholeOpinionStillUnchanged':(A/'job/TERMINAL-REVIEW.json').read_bytes()==(D/'WHOLE-OPINION.json').read_bytes(),'runtimeOrGitMutation':False}
(D/'SUPPLEMENTAL-VALIDATION.json').write_text(json.dumps(x,indent=2)+'\n')
manifest={'files':[{'path':str(p.relative_to(D)),'bytes':p.stat().st_size,'sha256':sha(p)}for p in sorted(D.rglob('*'))if p.is_file()and p.name!='OUTPUT-MANIFEST.json']};(D/'OUTPUT-MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
for f in manifest['files']:assert sha(D/f['path'])==f['sha256']
print(json.dumps({'patchBodies':len(files),'actualSourcePinsMechanicallyVerified':len(actual['files']),'outputManifestSha256':sha(D/'OUTPUT-MANIFEST.json'),'outputFiles':len(manifest['files']),'reportSha256':sha(D/'REPORT.md')},indent=2))
