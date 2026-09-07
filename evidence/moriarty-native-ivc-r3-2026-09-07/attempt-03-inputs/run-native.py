#!/usr/bin/env python3
"""One reviewed native attempt in a cgroup. No retries, raised limits or hidden commands."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import selectors
import subprocess
import sys
import time

PIN = '695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7'
SRS_HASH = '4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74'
MEMORY = 8 * 1024**3
SECONDS = 1200
LOG_LIMIT = 8 * 1024**2
OUTPUT_LIMIT = 256 * 1024**2


def digest(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(1024**2), b''):
            h.update(block)
    return h.hexdigest()


def capture(args):
    return subprocess.check_output(args, text=True).strip()


def limits():
    line = next(x for x in Path('/proc/self/cgroup').read_text().splitlines() if x.startswith('0::'))
    group = Path('/sys/fs/cgroup') / line[3:].lstrip('/')
    values = {name: (group / name).read_text().strip() for name in ['memory.max', 'memory.swap.max', 'memory.oom.group', 'pids.max']}
    if values['memory.max'] != str(MEMORY) or values['memory.swap.max'] != '0' or values['memory.oom.group'] != '1':
        raise RuntimeError('Required cgroup memory controls are absent; native execution refused')
    return group, values


def inner(repo, srs, output, seconds):
    group, enforced = limits()
    (output/'enforced-limits.json').write_text(json.dumps({'cgroup':str(group), **enforced}, indent=2)+'\n')
    env = os.environ.copy()
    env.update(MORIARTY_R3_SRS=str(srs), MORIARTY_R3_OUTPUT=str(output), CARGO_BUILD_JOBS='2', RAYON_NUM_THREADS='2', RUSTUP_TOOLCHAIN='1.90.0')
    command = [str(Path.home()/'.cargo/bin/cargo'), 'run', '--locked', '--release', '--jobs', '2', '-p', 'midnight-aggregation', '--features', 'truncated-challenges', '--example', 'moriarty_loan_r3']
    # Child inherits the verified cgroup; process-group termination is additional cleanup.
    start = time.monotonic()
    proc = subprocess.Popen(command, cwd=repo, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True)
    selector = selectors.DefaultSelector(); selector.register(proc.stdout, selectors.EVENT_READ)
    count = 0; reason = 'process-exit'
    with (output/'native.stdout.txt').open('xb') as log:
        while selector.get_map():
            elapsed = time.monotonic()-start
            retained = sum(p.stat().st_size for p in output.rglob('*') if p.is_file())
            if elapsed > seconds-10 or retained >= OUTPUT_LIMIT-LOG_LIMIT or count >= LOG_LIMIT:
                reason = 'wall-limit' if elapsed > seconds-10 else 'output-limit'
                subprocess.run(['kill', '-TERM', '--', '-'+str(proc.pid)], check=False)
                try: proc.wait(timeout=3)
                except subprocess.TimeoutExpired: subprocess.run(['kill', '-KILL', '--', '-'+str(proc.pid)], check=False)
                break
            for key,_ in selector.select(timeout=0.5):
                data = os.read(key.fd, min(65536, LOG_LIMIT-count))
                if not data: selector.unregister(key.fileobj)
                else: log.write(data);log.flush();count+=len(data)
        code=proc.wait()
    usage = {name:(group/name).read_text().strip() for name in ['memory.peak','memory.events','cpu.stat']}
    (output/'native-process.json').write_text(json.dumps({'command':command,'exitCode':code,'reason':reason,'elapsedSeconds':time.monotonic()-start,'logBytes':count,'usage':usage},indent=2)+'\n')
    return code if code != 0 else (0 if reason=='process-exit' else 124)


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--srs', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--inside-cgroup', action='store_true', help=argparse.SUPPRESS)
    parser.add_argument('--budget-seconds', type=int, default=SECONDS, help=argparse.SUPPRESS)
    parser.add_argument('--correction', type=Path, help='Explicit changed hypothesis tied to a prior failed receipt; never adds runtime budget')
    args=parser.parse_args()
    repo,srs,output=(p.resolve() for p in [args.repo,args.srs,args.output])
    if not 10 < args.budget_seconds <= SECONDS:raise RuntimeError('Invalid runtime budget')
    if args.inside_cgroup:return inner(repo,srs,output,args.budget_seconds)
    if capture(['git','-C',str(repo),'rev-parse','HEAD']) != PIN:raise RuntimeError('Wrong native source pin')
    tracked_diff=subprocess.check_output(['git','-C',str(repo),'diff','HEAD','--'])
    if tracked_diff:
        approved=json.loads(args.correction.read_text()) if args.correction else {}
        changed=capture(['git','-C',str(repo),'diff','--name-only','HEAD','--'])
        if changed!='aggregation/src/ivc/error.rs' or hashlib.sha256(tracked_diff).hexdigest()!=approved.get('diagnosticPatchSha256'):
            raise RuntimeError('Tracked native source differs from reviewed pin or explicit diagnostic patch')
    if srs.stat().st_size != 25166212 or digest(srs) != SRS_HASH:raise RuntimeError('SRS catalog identity mismatch')
    if output.exists():raise RuntimeError('Output already exists: refusing to overwrite or automatically retry')
    for name in ['aggregation/examples/moriarty_loan_r3.rs','aggregation/examples/moriarty_r3/episode.rs']:
        if not (repo/name).is_file():raise RuntimeError('Missing reviewed harness: '+name)
    attempt_record=repo.parent/'r3-native-attempt.json'
    prior_seconds=0;correction=None
    if attempt_record.exists():
        if args.correction is None:raise RuntimeError('Attempt consumed; no automatic retry')
        previous=json.loads(attempt_record.read_text());correction=json.loads(args.correction.read_text())
        prior_output=Path(previous['output']);prior_result=prior_output/'result.json'
        if correction['previousOutput']!=str(prior_output) or correction['previousResultSha256']!=digest(prior_result):raise RuntimeError('Correction does not bind prior terminal receipt')
        terminal=json.loads(prior_result.read_text())
        if terminal['exitCode']==0 or not correction['changedHypothesis'].strip():raise RuntimeError('Continuation needs a failed result and changed hypothesis')
        harness=repo/'aggregation/examples/moriarty_loan_r3.rs'
        if correction['newHarnessSha256']!=digest(harness):raise RuntimeError('Corrected harness identity mismatch')
        old=json.loads((prior_output/'invocation.json').read_text())['hashes']['aggregation/examples/moriarty_loan_r3.rs']
        if old==digest(harness) and not tracked_diff:raise RuntimeError('Harness or diagnostic hypothesis did not change')
        prior_seconds=previous.get('priorSeconds',0)+math.ceil(terminal['elapsedSeconds'])
    seconds=SECONDS-prior_seconds
    if seconds<=10:raise RuntimeError('Original cumulative native runtime budget exhausted')
    attempt_record.write_text(json.dumps({'output':str(output),'nativePin':PIN,'priorSeconds':prior_seconds,'status':'attempt-consumed-no-automatic-retry'},indent=2)+'\n')
    output.mkdir(parents=True)
    unit='moriarty-r3-native-'+time.strftime('%Y%m%d%H%M%S',time.gmtime())
    command=['systemd-run','--user','--wait','--pipe','--unit='+unit,
      '-p','MemoryMax='+str(MEMORY),'-p','MemorySwapMax=0',
      '-p','RuntimeMaxSec='+str(seconds),'-p','KillMode=control-group','-p','OOMPolicy=kill',
      '-p','TasksMax=128','-p','CPUQuota=200%','-p','LimitCORE=0',
      sys.executable,str(Path(__file__).resolve()),'--inside-cgroup','--budget-seconds',str(seconds),'--repo',str(repo),'--srs',str(srs),'--output',str(output)]
    manifest={'nativePin':PIN,'srsSha256':SRS_HASH,'limits':{'seconds':seconds,'cumulativeSeconds':SECONDS,'priorSecondsCharged':prior_seconds,'memoryBytes':MEMORY,'buildJobs':2,'k':17,'positiveProvingSteps':2,'retainedOutputBytes':OUTPUT_LIMIT},'correction':correction,'command':command,'hashes':{name:digest(repo/name) for name in ['Cargo.lock','aggregation/examples/moriarty_loan_r3.rs','aggregation/examples/moriarty_r3/episode.rs']},'startedAt':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),'scope':'fixed financial native experiment; no ledger acceptance','storageBoundary':'Retained evidence capped; disposable Cargo target and dependency caches excluded' }
    (output/'invocation.json').write_text(json.dumps(manifest,indent=2)+'\n')
    start=time.monotonic()
    with (output/'outer.stdout.txt').open('xb') as log:
        result=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT)
    status=subprocess.run(['systemctl','--user','show',unit,'-p','Result','-p','ExecMainCode','-p','ExecMainStatus','-p','MemoryPeak','-p','CPUUsageNSec'],capture_output=True,text=True)
    receipt={'exitCode':result.returncode,'elapsedSeconds':time.monotonic()-start,'systemd':status.stdout,'retainedOutputBytes':sum(p.stat().st_size for p in output.rglob('*') if p.is_file()),'completedAt':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())}
    (output/'result.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(receipt));return result.returncode

if __name__=='__main__':
    sys.exit(main())
