import pathlib, subprocess, sys, json
root=pathlib.Path('/tmp/moriarty-production-persistence-diagnosis-r1-work')
sys.path.insert(0,str(root/'plugins/moriarty-dev/tests'))
import test_records as tr
frozen=json.loads(pathlib.Path('/tmp/moriarty-production-round1-evidence/source-manifest.json').read_text())['files']
loaded={}
def fetch(rel):
 if not rel or rel in frozen:return
 out=subprocess.run(['git','-C','/home/charl/Moriarty','show','f7f40ce5e945fd9d7d2bd7a3c05ee3798331ff8e:'+rel],capture_output=True)
 if out.returncode:return
 path=root/rel;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(out.stdout)
 import hashlib
 loaded[rel]=hashlib.sha256(out.stdout).hexdigest()
for key in ('PROGRAM_SRC','SPRINTS_SRC','LOAN_BINDING_SRC','LOAN_RESOURCE_SRC','LOAN_FREEZE_SRC','LOAN_ACCEPT_SRC','LOAN_VERIFY3_SRC','ATOMIC_BINDING_SRC','ATOMIC_RESOURCE_SRC'):
 fetch(str(getattr(tr,key).relative_to(root)))
# Fetch exactly the fixture's references, preserving its existing path semantics.
original=tr.copy_relative
def materialize(dest,relative):
 fetch(relative)
 return original(dest,relative)
tr.copy_relative=materialize
import tempfile
with tempfile.TemporaryDirectory() as tmp:tr.copy_genuine_registers(pathlib.Path(tmp))
pathlib.Path('/tmp/moriarty-production-persistence-r1-base-inputs.json').write_text(json.dumps(loaded,indent=2)+'\n')
print('Materialized base inputs:',len(loaded))
