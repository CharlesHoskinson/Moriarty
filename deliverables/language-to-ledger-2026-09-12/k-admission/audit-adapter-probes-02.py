import importlib.util,json,tempfile,hashlib,sys,tracemalloc,time,os
from pathlib import Path
W=Path('/home/charl/Moriarty/.worktrees/lifecycle-k'); D=Path('/home/charl/Moriarty/deliverables/language-to-ledger-2026-09-12/k-admission')
def load(n,p):
 s=importlib.util.spec_from_file_location(n,p); m=importlib.util.module_from_spec(s); s.loader.exec_module(m); return m
A=load('a',W/'.moriarty-dev/k-macro05-trace106-diagnostic.py'); T=load('t',W/'.moriarty-dev/test_k_macro05_trace106_diagnostic.py')
files=json.loads((D/'author-adapter-ready-02.json').read_text())['files']
def freeze(): return {n:hashlib.sha256((W/n).read_bytes()).hexdigest() for n in files}
R={'before':freeze(),'verifyOnly':A.run_diagnostic(verify_only=True,stderr_write=lambda _:None)}
traces={
 'paddedWrongPid':b'[pid    10] execve("/retained/interpreter", []) = 0\n[pid    10] +++ exited with 0 +++\n[pid    20] execve("/parser", []) = 0\n[pid    20] +++ killed by SIGSEGV +++\n',
 'reexecSamePid':b'[pid 10] execve("/retained/interpreter", []) = 0\n[pid 10] execve("/parser", []) = 0\n[pid 10] +++ killed by SIGSEGV +++\n',
 'ordinaryStderr':b'debug execve("/retained/interpreter", []) = 0\nerror: child killed by SIGSEGV\n',
 'splitExec':b'[pid 10] execve("/retained/interpreter", [] <unfinished ...>\n[pid 10] <... execve resumed>) = 0\n[pid 10] +++ killed by SIGSEGV +++\n',
 'rootPidMigration':b'execve("/bin/krun", []) = 0\n[pid 10000] execve("/bin-unwrapped/krun", []) = 0\n',
}
R['attribution']={n:{'trace':v.decode(),'actual':A.parse_process_evidence(v)} for n,v in traces.items()}
raw=b'[pid 12345] execve("/retained/interpreter", []) = 0\n[pid 12345] +++ killed by SIGSEGV +++\n > native_stack_site+0x1\n\xff\x00\n'
with tempfile.TemporaryDirectory() as d:
 p=Path(d); pins=T.synthetic_pins(p)
 result=A.run_diagnostic(pins=pins,spawn=lambda *a,**k:{'returncode':113,'stdout':b'','stderr':raw},getrlimit=lambda *_:(8388608,-1),setrlimit=lambda *_:None,stdout_write=lambda _:None,stderr_write=lambda _:None,output_dir=p/'capture')
 recovered=(p/'capture/stderr.raw').read_bytes(); manifest=json.loads((p/'capture/manifest.json').read_text())
 R['retention']={'exactBytesRecovered':recovered==raw,'stackRecovered':b'native_stack_site' in recovered,'result':result,'manifest':manifest}
tracemalloc.start(); o=A._native_spawn([sys.executable,'-c','import os; os.write(1,b"x"*(4*1024*1024))'],child_seconds=2); _,peak=tracemalloc.get_traced_memory();tracemalloc.stop()
R['boundedCapture']={**{k:v for k,v in o.items() if not isinstance(v,bytes)},'retainedBytes':len(o['stdout']),'peakAllocationBytes':peak}
with tempfile.TemporaryDirectory() as d:
 marker=Path(d)/'done'; code='import subprocess,sys,time; subprocess.Popen([sys.executable,"-c",'+repr('import time; from pathlib import Path; time.sleep(0.4); Path('+repr(str(marker))+').write_text("survived")')+'],start_new_session=True); time.sleep(2)'
 o=A._native_spawn([sys.executable,'-c',code],child_seconds=.1,output_limit=1024)
 time.sleep(.5)
 R['cleanup']={**{k:v for k,v in o.items() if not isinstance(v,bytes)},'detachedChildSurvived':marker.exists()}
R['after']=freeze(); R['unchanged']=R['before']==R['after'];R['scope']='Offline synthetic trace and bounded Python children only; no K/strace/compilation/accounting; temporary capture destinations.'
(D/'audit-adapter-probes-02.json').write_text(json.dumps(R,indent=2)+'\n')
print(json.dumps({'unchanged':R['unchanged'],'verify':R['verifyOnly']['status'],'attribution':{k:v['actual'] for k,v in R['attribution'].items()},'retentionExact':R['retention']['exactBytesRecovered'],'manifestComplete':manifest['rawRetentionComplete'],'resultComplete':result['rawRetentionComplete'],'capture':R['boundedCapture'],'cleanup':R['cleanup']},indent=2))
