# B4 evidence check
> Source: file:///home/charl/code/Moriarty-perf/scripts/qa/check-b4.sh
> Collected: 2026-10-26
> Published: Unknown

#!/usr/bin/env bash
# Read-only T3 acceptance check. Never opens a wallet or sends a transaction.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export RAYON_NUM_THREADS=4
mapfile -t inputs < <(python3 -B - <<'PY'
import importlib.util,json,pathlib
spec=importlib.util.spec_from_file_location('e','scripts/qa/check-rollup-evidence.py')
e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
attempts=list(pathlib.Path('target/sherlock-b4').glob('stagenet-*/attempt.json'))
if not attempts:raise SystemExit('B4 requires an actual coordinated stagenet attempt')
attempt=max(attempts,key=lambda p:json.loads(p.read_bytes())['started_at'])
if not all((attempt.parent/name/'receipt.json').is_file() for name in ('deploy','acceptValidity','settleValidity')):
    raise SystemExit('B4 requires finalized deployment, acceptance and settlement receipts')
ready=json.loads((attempt.parent/'ready.json').read_bytes());assert ready['schema']=='sherlock-b4-local-preparation/1'
b3=pathlib.Path(ready['b3']);e.check('B3',b3)
print(attempt.parent.resolve());print(b3);print(ready['srs'])
PY
)
[[ ${#inputs[@]} == 3 ]] || exit 1
run_dir="${inputs[0]}"; b3="${inputs[1]}"; srs="${inputs[2]}"
mkdir -p target/sherlock-b4/checks
check_dir="$(mktemp -d "$PWD/target/sherlock-b4/checks/run-XXXXXXXX")"
# Preserve B3's original verifier receipt/measurement; reverify copies here.
for layer in leaf parent decider; do
  for suffix in vk proof relation instance.json; do
    cp "$b3/$layer.$suffix" "$check_dir/$layer.$suffix"
  done
done
scripts/doctorwatson heavy --id B4-stock-verify --budget b4-full-batch-stagenet \
  --hypothesis 'Reverify retained stock proofs under heavy; release the slot before network readback' -- \
  python3 -B scripts/qa/measure-rollup.py "$check_dir/stock-verify.measurement.json" -- \
    "$b3/rollup.bin" verify "$b3/fixture/corpus" 1 "$check_dir" "$srs"
node tools/sherlock-sdk/rollup-readback.mjs "$run_dir" "$check_dir/readback.json"
python3 -B - "$check_dir" "$run_dir" <<'PY'
import json,pathlib,sys
root=pathlib.Path(sys.argv[1]);readback=json.loads((root/'readback.json').read_bytes())
assert readback['schema']=='sherlock-b4-readback/1' and readback['completeEffectsMatch'] and readback['nativeProofsVerified']
assert readback['transactions']==16 and readback['headers']==50
doc={'schema':'sherlock-b4-results/1','predicate':'B4','run':sys.argv[2],
     'readback':readback,'independent_stock_verification':json.loads((root/'verified.json').read_bytes())}
(root/'B4.json').write_text(json.dumps(doc,indent=2,sort_keys=True)+'\n')
print('PASS B4: exact16-transaction batch accepted and settled on stagenet; fresh complete readback; stock proofs reverified')
PY
