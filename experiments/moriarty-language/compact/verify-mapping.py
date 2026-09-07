#!/usr/bin/env python3
"""Bounded compile/runtime evidence for restricted kernels. Never builds keys."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

p=argparse.ArgumentParser()
p.add_argument('--runtime-node-modules',type=Path,required=True)
p.add_argument('--output',type=Path,required=True)
a=p.parse_args()
here=Path(__file__).resolve().parent
out=a.output.resolve();out.mkdir(parents=True,exist_ok=True)
modules=a.runtime_node_modules.resolve(strict=True)
assert json.loads((modules/'@midnight-ntwrk/compact-runtime/package.json').read_text())['version']=='0.16.0'
commands=[]
def run(command,env=None):
 start=time.monotonic()
 result=subprocess.run(command,cwd=here,env=env,capture_output=True,text=True,timeout=90)
 commands.append({'command':command,'exitCode':result.returncode,'elapsedSeconds':time.monotonic()-start,'stdout':result.stdout,'stderr':result.stderr})
 (out/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
 if result.returncode:raise RuntimeError(result.stderr or result.stdout)
 return result.stdout.strip()
assert run(['compact','compile','--version'])=='0.31.1'
assert run(['compact','compile','--language-version'])=='0.23.0'
assert run(['compact','compile','--runtime-version'])=='0.16.0'
run(['node',str(here/'materialize-mapping.mjs')])
compiled=[]
with tempfile.TemporaryDirectory(prefix='moriarty-compact-mapping-') as temp:
 for name in ['loan','swap']:
  for kind in ['kernel','harness']:
   target=Path(temp)/name/kind
   source=here/'generated'/name/(kind+'.compact')
   run(['compact','compile','--skip-zk','--compact-path',str(here),str(source),str(target)])
   assert not list(target.rglob('*.prover')) and not list(target.rglob('*.verifier'))
   for artifact in sorted(target.rglob('*')):
    if artifact.is_file():
     relative=Path('compiled')/name/kind/artifact.relative_to(target)
     retained=out/relative;retained.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(artifact,retained)
     compiled.append({'path':str(relative),'bytes':retained.stat().st_size,'sha256':hashlib.sha256(retained.read_bytes()).hexdigest()})
 env=dict(os.environ,MORIARTY_RUNTIME_NODE_MODULES=str(modules))
 run(['node','--test',str(here/'../tests/lowering.test.mjs')],env)
 run(['tsc','--noEmit','--project',str(here/'../tsconfig.json')])
files=[here/'../src/lower-compact.ts',here/'../tests/lowering.test.mjs',here/'materialize-mapping.mjs',here/'verify-mapping.py',here/'arithmetic.compact',here/'../spec/bounds.json',here/'../spec/examples/loan.moriarty',here/'../spec/examples/swap.moriarty']
receipt={'schemaVersion':'moriarty-compact-mapping-evidence/1','status':'experimental-tests-pass','scope':'Compiled restricted Core kernels, full numeric state/effect operand runtime comparisons and test-only ZKIR snapshot wrappers; no keys, proofs, asset settlement, PCD or full compiler/ledger correspondence','compiler':'0.31.1','language':'0.23.0','runtime':'0.16.0','tests':6,'keysGenerated':False,'proofsGenerated':False,'sources':[{'path':str(f.relative_to(here.parent)) if f.is_relative_to(here.parent) else str(f),'sha256':hashlib.sha256(f.read_bytes()).hexdigest()} for f in files],'compiled':compiled,'commands':'commands.json'}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'status':'experimental-tests-pass','tests':6,'compiledArtifacts':len(compiled),'keysGenerated':False,'proofsGenerated':False}))
