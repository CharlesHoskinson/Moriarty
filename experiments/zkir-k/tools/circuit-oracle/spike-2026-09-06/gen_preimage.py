"""Build a preimage for PROGRAM.zkir with the K generation run, replicating
diff_test.py's attempt loop (rng seeded '2026:<file>', small inputs, manifest
test_preimage on attempt 0) and stopping at the first preimage the K
semantics accepts. Usage: gen_preimage.py PROGRAM.zkir OUT.json [--ext]"""
import json, random, sys
from pathlib import Path
sys.path.insert(0, '/home/charl/Moriarty/.worktrees/zkir-k-iter3/experiments/zkir-k/tools')
import zkir_kast
from zkir_run import Runner
from diff_test import build_preimage
prog = Path(sys.argv[1]); out = Path(sys.argv[2]); ext = '--ext' in sys.argv
doc = json.load(open(prog))
runner = Runner(None, ext=ext)
program = zkir_kast.load_program(prog, ext=ext)
rng = random.Random(f'2026:{prog.name}')
seed_pre = None
manifest = prog.parent / 'manifest.json'
if manifest.exists():
    for entry in json.load(open(manifest)).get('programs', []):
        if entry['file'] == prog.name:
            seed_pre = entry.get('test_preimage')
for attempt in range(8):
    pre, passes = build_preimage(runner, program, doc, rng, small=True, seed_pre=seed_pre if attempt == 0 else None)
    res = runner.run(program, pre)
    print(json.dumps({'attempt': attempt, 'passes': passes, 'k_status': res['status'], 'k_error': res.get('error'), 'pis': len(res['pis'])}))
    json.dump(pre, open(out, 'w'))  # the last attempt is kept even when K rejects it
    if res['status'] == 'ok':
        break
