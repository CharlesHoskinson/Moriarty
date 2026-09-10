"""Authorized bounded diagnostic: one harmless true unit, no financial/runtime services."""
import sys,pathlib,subprocess,time,json,os
from terminal_predicate import require_exit_zero

def main():
    if sys.argv[1:] != ['--run-once']:raise SystemExit('PROPOSED_ONLY_REQUIRE_RUN_ONCE')
    out=pathlib.Path(__file__).resolve().parent/'smoke-attempt-02.jsonl'
    fd=os.open(out,os.O_WRONLY|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600)
    stop=time.monotonic()+10;unit='moriarty-sp05-exit-retention-smoke-01.service';attempted=False;record={'status':'CONSUMED','financialSubmissions':0,'networkCalls':0,'rawObservations':[]}
    def call(argv):
        left=stop-time.monotonic()
        if left<=0:raise TimeoutError('SMOKE_DEADLINE')
        return subprocess.run(argv,capture_output=True,text=True,check=True,timeout=min(3,left)).stdout
    try:
        assert call(['/usr/bin/systemctl','--user','show',unit,'--property=LoadState','--value']).strip()=='not-found'
        argv=['/usr/bin/systemd-run','--user','--unit='+unit,'--property=Type=exec','--property=RemainAfterExit=yes','--property=Restart=no','--property=MemoryMax=33554432','--property=MemorySwapMax=0','--property=CPUQuota=10%','--property=RuntimeMaxSec=5s','--property=TimeoutStartSec=2s','--property=TimeoutStopSec=1s','/usr/bin/true']
        slice_group=call(['/usr/bin/systemctl','--user','show','app.slice','--property=ControlGroup','--value']).strip();assert slice_group.startswith('/') and '..' not in slice_group
        cgroup=pathlib.Path('/sys/fs/cgroup'+slice_group)/unit;assert not cgroup.exists();record['cgroupPath']=str(cgroup)
        record['argv']=argv;os.write(fd,(json.dumps(record)+'\n').encode());os.fsync(fd);attempted=True;call(argv)
        properties='LoadState,ActiveState,SubState,Type,RemainAfterExit,Transient,MainPID,Result,ExecMainCode,ExecMainStatus,InvocationID'
        raw=call(['/usr/bin/systemctl','--user','show',unit,'--property='+properties]);record['rawObservations'].append(raw)
        fields=dict(line.split('=',1) for line in raw.splitlines());record['observation']=require_exit_zero(fields,fields.get('InvocationID'))
    except BaseException as error:record['errorClass']=type(error).__name__
    finally:
        if attempted:
            try:subprocess.run(['/usr/bin/systemctl','--user','stop',unit],check=True,capture_output=True,text=True,timeout=3);record['explicitStopCompleted']=True;record['cgroupAbsent']=not cgroup.exists();record['afterStop']=subprocess.run(['/usr/bin/systemctl','--user','show',unit,'--property=LoadState,ActiveState,SubState,MainPID,ControlGroup'],check=True,capture_output=True,text=True,timeout=2).stdout
            except BaseException as error:record['stopErrorClass']=type(error).__name__
        os.write(fd,(json.dumps(record)+'\n').encode());os.fsync(fd);os.close(fd)
        directory=os.open(out.parent,os.O_RDONLY|os.O_DIRECTORY)
        try:os.fsync(directory)
        finally:os.close(directory)
    return 0 if record.get('observation',{}).get('exitCode')==0 and record.get('explicitStopCompleted') and record.get('cgroupAbsent') else 1
if __name__=='__main__':raise SystemExit(main())
