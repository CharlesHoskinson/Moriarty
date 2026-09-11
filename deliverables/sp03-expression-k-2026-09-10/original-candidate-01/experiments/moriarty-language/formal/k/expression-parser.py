#!/usr/bin/env python3
"""The krun --parser boundary: installed kast JSON-to-KORE, no evaluation."""
import os,sys,json,hashlib
from pathlib import Path
H=Path(__file__).resolve().parent
if len(sys.argv)!=2:raise SystemExit(2)
lock=json.loads((H/'expression-toolchain.lock.json').read_text())['kast']
p=Path(lock['path'])
if hashlib.sha256(p.read_bytes()).hexdigest()!=lock['sha256']:raise SystemExit('TOOLCHAIN_STALE')
os.execv(str(p),[str(p),'--input','json','--output','kore','--definition',str(H/'.build-expression-v1/expression-v1-kompiled'),sys.argv[1]])
