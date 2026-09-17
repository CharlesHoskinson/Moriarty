#!/usr/bin/env python3
"""One reviewed stack discrimination using the existing K command executor."""
import hashlib, importlib.util, json, os, sys, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
HERE=ROOT/'experiments/moriarty-language/formal/k'
sys.path.insert(0,str(HERE))
import run
run.BUILD=HERE/'.build-expression-v1'
run.DEFINITION=run.BUILD/'expression-v1-kompiled'
run.DEADLINE=time.monotonic()+25
binding=json.loads((run.BUILD/'binding.json').read_text())
assert binding['sources']=={n:run.sha(HERE/n) for n in binding['sources']}, 'SOURCE_STALE'
assert binding['artifacts']==run.artifacts(), 'ARTIFACT_STALE'
lock=json.loads((HERE/'expression-toolchain.lock.json').read_text())
assert all(run.sha(Path(n))==v for n,v in lock['files'].items()), 'TOOLCHAIN_STALE'
assert all(not Path(n).exists() for n in lock['absentFiles']), 'STARTUP_STALE'
assert all(not list(Path(n).iterdir()) for n in lock['emptyDirectories']), 'STARTUP_STALE'
record=Path(__file__).parent
program=record/'trace106.input'
gdb=record/'diagnostic.gdb'
assert run.sha(program)=='6c29dc49a828956f5056605b672b519a515129383c7dd46b0dc34bba174c3c1b'
assert run.sha(gdb)==lock['gdbCommandSha256']
with (run.BUILD/'diagnostic02-attempt.json').open('x') as f:
    json.dump({'started':time.time(),'cumulativeCompiles':10,'cumulativeKrun':221,'stackBytes':67108864,'inputSha256':run.sha(program)},f)
try:
    run.command(['/usr/bin/krun',str(program),'--definition',str(run.DEFINITION),'--output','json','--parser',str(HERE/'expression-parser.py'),'--no-expand-macros','--debugger-command',str(gdb),'--debugger-batch'],20,'diagnostic02')
    print(json.dumps({'status':'DiagnosticFinished','semanticAcceptance':False}))
except run.RunnerError as exc:
    print(json.dumps({'status':'HarnessError','code':str(exc)}));sys.exit(2)
