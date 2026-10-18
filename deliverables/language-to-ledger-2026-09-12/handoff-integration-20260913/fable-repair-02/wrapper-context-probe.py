import json,os,sys,time
from pathlib import Path
import pytest

def stat(pid):
 p=Path('/proc')/str(pid)
 try:
  data=(p/'stat').read_text(); end=data.rindex(')'); fs=data[end+2:].split()
  status=(p/'status').read_text().splitlines()
  return {'pid':pid,'comm':data[data.index('(')+1:end],'state':fs[0],'ppid':int(fs[1]),'startTicks':fs[19],'pidNamespace':os.readlink(p/'ns/pid') if (p/'ns/pid').exists() else None,'nsPid':[x for x in status if x.startswith('NSpid:')]}
 except (FileNotFoundError,ProcessLookupError): return {'pid':pid,'absent':True}

class Plugin:
 def pytest_collection_modifyitems(self,items):
  print('CONTEXT '+json.dumps({'self':stat(os.getpid()),'parent':stat(os.getppid()),'init':stat(1),'procSelf':os.readlink('/proc/self')}),flush=True)
  mods=set()
  for item in items:
   m=item.module
   if m in mods: continue
   mods.add(m); original=m.assert_pid_gone
   def checked(pid,timeout=2,_original=original):
    try: _original(pid,timeout)
    except AssertionError:
     child=stat(pid);parent=stat(child['ppid']) if 'ppid' in child else None
     print('PID_FAILURE '+json.dumps({'observed':child,'parent':parent,'self':stat(os.getpid())}),flush=True)
     raise
    else: print('PID_GONE '+json.dumps({'pid':pid}),flush=True)
   m.assert_pid_gone=checked

sys.exit(pytest.main(['-q','-s','-p','no:cacheprovider','tests/test_prover_lifetime_wrapper.py::test_control_deadline_kills_hanging_child_and_is_independent','tests/test_prover_lifetime_wrapper.py::test_dead_external_launcher_cannot_disable_wrapper_deadline'],plugins=[Plugin()]))
