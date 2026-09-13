#!/usr/bin/env python3
"""Test-only: candidate11's exact bwrap envelope around harmless Python code."""
import hashlib,importlib.util,json,os,sys
from pathlib import Path
O=Path(__file__).resolve().parent
sp=importlib.util.spec_from_file_location('snapshot11',O/'candidate-source-11.py');m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
snapshot=json.loads((O/'snapshot-inventory-draft-03.json').read_text())
expected=hashlib.sha256(json.dumps(sorted((e['path'],e['sizeBytes'],e['sha256'],e['mode'] & 0o555) for e in snapshot['files']),separators=(',',':')).encode()).hexdigest()
check='''import hashlib,json,os,pwd,resource
from pathlib import Path
root=Path(ROOT_LITERAL)
items=sorted((str(p),p.stat().st_size,hashlib.sha256(p.read_bytes()).hexdigest(),p.stat().st_mode & 0o7777) for p in root.rglob('*') if p.is_file())
assert len(items)==506,len(items)
assert hashlib.sha256(json.dumps(items,separators=(',',':')).encode()).hexdigest()==EXPECTED_LITERAL
assert (os.getuid(),os.geteuid(),os.getgid(),os.getegid())==(1000,1000,1000,1000)
assert os.environ['HOME']==str(Path.home())==pwd.getpwuid(os.getuid()).pw_dir=='/home/charl'
assert os.getcwd()=='/home/charl/Moriarty'
assert resource.getrlimit(resource.RLIMIT_STACK)[0]==8388608
for p in [Path('/home/charl/should-not-write'),Path(items[0][0])]:
 try:
  with p.open('ab') as f:f.write(b'x')
 except OSError:pass
 else:raise AssertionError('private snapshot is writable')
memfds=[]
for p in Path('/proc/self/fd').iterdir():
 try:target=os.readlink(p)
 except FileNotFoundError:continue
 if 'memfd:moriarty-snap' in target:memfds.append(target)
assert not memfds,memfds
os.write(2,b'SNAPSHOT506_VERIFIED_HOME_UID_CWD_STACK_READONLY_FDS\\n')
'''.replace('ROOT_LITERAL',repr(snapshot['root'])).replace('EXPECTED_LITERAL',repr(expected))
fds=m.seal_snapshot(snapshot)
try:
 argv=m.build_bwrap_argv(snapshot,fds,['/usr/bin/python3.14','-I','-B','-c',check+'\n'+sys.argv[1]])
 m._inherit_snapshot_fds(fds)
 os.execve(m.BWRAP_PATH,argv,{'PATH':'/usr/bin:/bin','HOME':'/home/charl','LANG':'C.UTF-8','LC_ALL':'C.UTF-8','PYTHONDONTWRITEBYTECODE':'1','PYTHONNOUSERSITE':'1','PYTHONSAFEPATH':'1'})
finally:m._close_fds(fds)
