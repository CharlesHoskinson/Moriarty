"""Independent simulated process checks; never invoke K or strace."""
import importlib.util, pathlib, json, sys, hashlib, tracemalloc, io, tempfile, time
sys.dont_write_bytecode=True
root=pathlib.Path('/home/charl/Moriarty/.worktrees/lifecycle-k')
spec=importlib.util.spec_from_file_location('adapter',root/'.moriarty-dev/k-macro05-trace106-diagnostic.py')
a=importlib.util.module_from_spec(spec);spec.loader.exec_module(a)
checks={}
pins=a.load_pins(a.DEFAULT_PINS)
checks['verifyOnly']=a.run_diagnostic(verify_only=True,stderr_write=lambda x:None)
trace=b'[pid 10] execve("/retained/interpreter", [], []) = 0\n[pid 10] +++ exited with 0 +++\n[pid 20] execve("/parser", [], []) = 0\n[pid 20] --- SIGSEGV {si_signo=SIGSEGV} ---\n[pid 20] +++ killed by SIGSEGV +++\n > native_stack_site+0x1\n'
checks['wrongPidAttribution']={'fixture':'Interpreter PID10 exits 0; parser PID20 killed by SIGSEGV','actual':a.parse_process_evidence(trace)}
stdout=io.BytesIO();stderr=io.BytesIO()
r=a.run_diagnostic(pins=pins,spawn=lambda *args,**kw: {'returncode':113,'stdout':b'','stderr':trace,'timedOut':False},getrlimit=lambda _: (8388608,-1),setrlimit=lambda *args:None,stdout_write=stdout.write,stderr_write=stderr.write)
checks['retention']={'fixture':'Injected simulated child output; no native spawn','nativeStderrBytes':len(trace),'retainedOutputContainsRawTrace':trace in stderr.getvalue(),'retainedOutputContainsStackSite':b'native_stack_site' in stderr.getvalue(),'result':r}
tracemalloc.start()
n=a._native_spawn([sys.executable,'-c','import os; os.write(1,b"x"*(4*1024*1024))'],child_seconds=2,output_limit=1024*1024)
_,peak=tracemalloc.get_traced_memory();tracemalloc.stop()
checks['captureMemory']={'fixture':'One harmless Python child writes 4MiB then exits','peakPythonAllocationBytes':peak,'observed':{k:v for k,v in n.items() if k not in ('stdout','stderr')},'retainedStdoutBytes':len(n['stdout'])}
# Detached grandchild closes naturally after 0.6 seconds; no indefinite process.
code='import subprocess,sys,time; subprocess.Popen([sys.executable,"-c","import time; time.sleep(0.6)"],start_new_session=True); time.sleep(2)'
n=a._native_spawn([sys.executable,'-c',code],child_seconds=0.1,output_limit=1024)
checks['deadline']={'fixture':'Harmless Python child spawns detached Python grandchild which holds pipes for 0.6s; parent killed at 0.1s','observed':{k:v for k,v in n.items() if k not in ('stdout','stderr')}}
checks['scope']={'nativeKInvoked':False,'straceInvoked':False,'compilations':0,'accountingMutated':False,'classification':'simulated process checks, not K reproduction'}
print(json.dumps(checks,indent=2))
