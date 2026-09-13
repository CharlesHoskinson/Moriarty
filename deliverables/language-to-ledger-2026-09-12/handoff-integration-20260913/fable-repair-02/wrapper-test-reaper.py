import ctypes,json,os,signal,sys,time
libc=ctypes.CDLL(None,use_errno=True)
if libc.prctl(36,1,0,0,0)!=0: raise OSError(ctypes.get_errno(),'PR_SET_CHILD_SUBREAPER')
child=os.fork()
if child==0: os.execv(sys.executable,[sys.executable,'/tmp/moriarty-wrapper-context-probe.py'])
end=time.monotonic()+15
status=None
while time.monotonic()<end:
 try: pid,s=os.waitpid(-1,os.WNOHANG)
 except ChildProcessError:
  if status is not None: sys.exit(status)
  raise
 if pid:
  print('REAP '+json.dumps({'pid':pid,'testProcess':pid==child,'exit':os.waitstatus_to_exitcode(s)}),flush=True)
  if pid==child: status=os.waitstatus_to_exitcode(s)
 else: time.sleep(.005)
print('HARNESS_TIMEOUT',flush=True)
sys.exit(124)
