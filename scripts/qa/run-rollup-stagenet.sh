#!/usr/bin/env bash
# Three explicit T3 transactions. Invoke only after READY T3 exact-command + coordinator GO.
# Run this orchestrator outside heavy. Proof callbacks acquire their own heavy
# slots; wallet sync, submission and indexer waits never hold those slots.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
[[ $# == 2 ]] || { echo 'usage: run-rollup-stagenet.sh LOCAL_READY_JSON NEW_OUTPUT' >&2; exit 2; }
mapfile -t config < <(python3 -B - "$1" "$2" <<'PY'
import hashlib,json,pathlib,sys
ready=json.loads(pathlib.Path(sys.argv[1]).read_bytes())
assert ready['schema']=='sherlock-b4-local-preparation/1' and not ready['submitted']
assert ready['decider_key_bytes_match_b3'] and ready['local']['contractProofAccepted'] and ready['local']['applied']
assert ready['local']['proofVerification']=='native/proof-verifying'
assert ready['local']['settlementProofAccepted'] and ready['local']['settlementApplied'] and ready['local']['completeSettledEffectsMatch']
for field,pin in [('rollup_call_binary','rollup_call_sha256'),('native_validator','native_validator_sha256'),('compiled_call_binary','compiled_call_sha256')]:
    with pathlib.Path(ready[field]).open('rb') as stream:assert hashlib.file_digest(stream,'sha256').hexdigest()==ready[pin]
out=pathlib.Path(sys.argv[2]).resolve()
assert out.is_relative_to(pathlib.Path('target').resolve()) and not out.exists()
assert out.parent==pathlib.Path('target/sherlock-b4').resolve() and out.name.startswith('stagenet-')
for name in ('sdk','build','b3','key','deployment_preparation','srs','rollup_call_binary','native_validator','compiled_call_binary'):
    value=ready[name]; assert isinstance(value,str) and '\n' not in value; print(value)
print(out)
PY
)
[[ ${#config[@]} == 10 ]] || exit 1
sdk="${config[0]}"; build="${config[1]}"; b3="${config[2]}"; key="${config[3]}"; preparation="${config[4]}"
export MORIARTY_PARENT_SRS="${config[5]}" ROLLUP_CALL_BINARY="${config[6]}" SHERLOCK_NATIVE_VALIDATOR="${config[7]}"
export COMPILED_CALL_BINARY="${config[8]}"
export CARGO_BUILD_JOBS=4 RAYON_NUM_THREADS=4
export MALLOC_MMAP_THRESHOLD_=131072 MALLOC_TRIM_THRESHOLD_=131072 MALLOC_ARENA_MAX=1
run_dir="${config[9]}"
mkdir "$run_dir"
cp "$1" "$run_dir/ready.json"
python3 -B - "$run_dir" <<'PY'
import datetime,json,pathlib,sys
path=pathlib.Path(sys.argv[1])/'attempt.json'
path.write_text(json.dumps({'schema':'sherlock-b4-attempt/1','started_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'status':'running','transactions':16,'headers':50},indent=2)+'\n')
PY
finish_rollup_attempt() {
  rollup_exit=$?
  python3 -B - "$run_dir/attempt.json" "$rollup_exit" <<'PY'
import datetime,json,pathlib,sys
path=pathlib.Path(sys.argv[1]);doc=json.loads(path.read_bytes())
doc.update(exit=int(sys.argv[2]),status='completed' if sys.argv[2]=='0' else 'failed',
           finished_at=datetime.datetime.now(datetime.timezone.utc).isoformat())
path.write_text(json.dumps(doc,indent=2)+'\n')
PY
}
trap finish_rollup_attempt EXIT
node tools/sherlock-sdk/rollup-deploy.mjs "$sdk" "$build" "$preparation" "$run_dir/deploy" --submit
for entry in acceptValidity settleValidity; do
  node tools/sherlock-sdk/rollup-call.mjs "$sdk" "$build" "$build" "$b3" "$key" \
    "$run_dir/deploy/receipt.json" "$run_dir/$entry" "$entry" --submit
done
node tools/sherlock-sdk/rollup-readback.mjs "$run_dir" "$run_dir/readback.json"
