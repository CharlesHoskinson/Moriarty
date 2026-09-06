"""Complete a generated preimage for the crate's test_*_proof programs: every
private_input register that is compared (constrain_eq / test_eq) against a
computed register receives that register's off-circuit encoding in the
private transcript. Values come from the K memory of the
generated preimage (all arithmetic precedes the private inputs); the result
is then re-run through the K semantics. Usage: fix_private.py PROG PRE OUT [--ext]"""
import json, subprocess, sys, tempfile
from pathlib import Path
sys.path.insert(0, '/home/charl/Moriarty/.worktrees/zkir-k-iter3/experiments/zkir-k/tools')
import zkir_kast
from zkir_run import Runner
prog, pre_path, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]); ext = '--ext' in sys.argv
oracle = '/home/charl/Moriarty/repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle' if ext else '/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle'
doc = json.load(open(prog)); pre = json.load(open(pre_path))
runner = Runner(None, ext=ext)
program = zkir_kast.load_program(prog, ext=ext)
mem = runner.run(program, pre)['memory']  # K keeps the memory of a failed run; the Rust oracle does not
pairs = {}
for ins in doc['instructions']:
    if ins['op'] in ('constrain_eq', 'test_eq'):
        pairs[ins['b']] = ins['a']; pairs[ins['a']] = ins['b']
priv = []
for ins in doc['instructions']:
    if ins['op'] == 'private_input':
        src = pairs.get(ins['output'])
        if src is None or src not in mem:
            raise SystemExit(f"no computed partner for {ins['output']}")
        priv += mem[src]['encoded']
pre['private_transcript'] = priv
json.dump(pre, open(out, 'w'))
res = runner.run(program, pre)
print(json.dumps({'program': prog.name, 'private_transcript': len(priv), 'k_status': res['status'], 'k_error': res.get('error'), 'violations': len(res.get('violations', []))}))
