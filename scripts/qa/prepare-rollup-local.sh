#!/usr/bin/env bash
# Offline B4 rehearsal: real V3 keys and SDK-bound stock D, no wallet/submission.
# Run through DoctorWatson heavy with budget b4-local-rollup.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS=4 RAYON_NUM_THREADS=4
export MALLOC_MMAP_THRESHOLD_=131072 MALLOC_TRIM_THRESHOLD_=131072 MALLOC_ARENA_MAX=1
unset SHERLOCK_PROVER_ID
sdk="${MORIARTY_SDK_ROOT:-$HOME/.midnight/stagenet-wallet}"
compiler="${COMPACT_TOOLCHAIN:-$HOME/.compact/versions/0.34.0/x86_64-unknown-linux-musl}"
srs="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
export MORIARTY_PARENT_SRS="$srs"
b3="$(python3 -B - <<'PY'
import importlib.util, json
spec=importlib.util.spec_from_file_location('e','scripts/qa/check-rollup-evidence.py')
e=importlib.util.module_from_spec(spec); spec.loader.exec_module(e)
p=e.latest_run('B3').resolve(); e.check('B3',p)
if json.loads((p/'build.json').read_bytes())['adapter_version']!='sherlock-v3-batch/2':
    raise SystemExit('requires the independently stock-verified fixed-key v2 baseline')
print(p)
PY
)"
mkdir -p target/sherlock-b4
run_dir="$(mktemp -d "$PWD/target/sherlock-b4/local-XXXXXXXX")"
cargo build --release --locked -p mori-recursion --features sherlock-rollup-prefix,sherlock-ledger-validation,sherlock-compiled-prover \
  --example rollup_call --example validate_call --example compiled_call
cp "${CARGO_TARGET_DIR:-$PWD/target}/release/examples/rollup_call" "$run_dir/rollup_call.bin"
cp "${CARGO_TARGET_DIR:-$PWD/target}/release/examples/validate_call" "$run_dir/validate_call.bin"
cp "${CARGO_TARGET_DIR:-$PWD/target}/release/examples/compiled_call" "$run_dir/compiled_call.bin"
export ROLLUP_CALL_BINARY="$run_dir/rollup_call.bin" SHERLOCK_NATIVE_VALIDATOR="$run_dir/validate_call.bin" COMPILED_CALL_BINARY="$run_dir/compiled_call.bin"
"$compiler/compactc" --skip-zk --feature-zkir-v3 tools/sherlock-sdk/rollup-anchor.compact "$run_dir/build"
python3 -B - "$run_dir/build" "$sdk" <<'PY'
import json,pathlib,sys
build=pathlib.Path(sys.argv[1]); sdk=pathlib.Path(sys.argv[2]).resolve()
(build/'package.json').write_text('{"type":"module"}\n')
(build/'node_modules').symlink_to(sdk/'node_modules',target_is_directory=True)
for entry in ('acceptValidity','settleValidity'):
    assert json.loads((build/f'zkir/{entry}.zkir').read_bytes())['version']['major']==3
PY
node tools/sherlock-sdk/rollup-template.mjs "$sdk" "$run_dir/build" "$run_dir/template.json"
cmp "$run_dir/template.json" crates/mori-recursion/tests/fixtures/rollup-template.json
mkdir "$run_dir/build/keys"
for entry in acceptValidity settleValidity; do
  python3 -B scripts/qa/measure-rollup.py "$run_dir/$entry-keygen.measurement.json" -- \
    "$compiler/zkir-v3" compile "$run_dir/build/zkir/$entry.zkir" \
    "$run_dir/build/keys/$entry.prover" "$run_dir/build/keys/$entry.verifier"
done
id="$(python3 -B - <<'PY'
import secrets
q=0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
print((secrets.randbelow(q-1)+1).to_bytes(32,'little').hex())
PY
)"
python3 -B scripts/qa/measure-rollup.py "$run_dir/decider-keygen.measurement.json" -- \
  "$ROLLUP_CALL_BINARY" prepare "$b3" "$run_dir/template.json" "$run_dir/key" "$run_dir/key-output" "$id" "$srs"
cmp "$run_dir/key/decider.vk" "$b3/decider.vk"
cmp "$run_dir/key/decider.relation" "$b3/decider.relation"
node tools/sherlock-sdk/rollup-prepare.mjs "$sdk" "$run_dir/build" "$b3" "$run_dir/key" \
  "$run_dir/build/keys/settleValidity.verifier" "$run_dir/deployment"
node tools/sherlock-sdk/rollup-local-call.mjs "$sdk" "$run_dir/build" "$run_dir/build" \
  "$run_dir/deployment/contract-state.bin" "$b3" "$run_dir/key" "$run_dir/call-check"
python3 -B scripts/qa/measure-rollup.py "$run_dir/local-call.measurement.json" -- \
  node tools/sherlock-sdk/rollup-local-call.mjs "$sdk" "$run_dir/build" "$run_dir/build" \
    "$run_dir/deployment/contract-state.bin" "$b3" "$run_dir/key" "$run_dir/call-proof" --prove
python3 -B - "$run_dir" "$b3" "$sdk" "$srs" <<'PY'
import hashlib,json,pathlib,sys
root=pathlib.Path(sys.argv[1]); positive=json.loads((root/'call-proof/ledger-result.json').read_bytes())
assert positive['contractProofAccepted'] and positive['applied'] and positive['validityStatementPresent']
assert positive['proofVerification']=='native/proof-verifying'
assert positive['settlementProofAccepted'] and positive['settlementApplied'] and positive['completeSettledEffectsMatch']
def digest(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
doc={'schema':'sherlock-b4-local-preparation/1','b3':sys.argv[2],'sdk':sys.argv[3],'srs':sys.argv[4],
     'build':str(root/'build'),'key':str(root/'key'),'deployment_preparation':str(root/'deployment/preparation.json'),
     'rollup_call_binary':str(root/'rollup_call.bin'),'native_validator':str(root/'validate_call.bin'),
     'compiled_call_binary':str(root/'compiled_call.bin'),'compiled_call_sha256':digest(root/'compiled_call.bin'),
     'rollup_call_sha256':digest(root/'rollup_call.bin'),'native_validator_sha256':digest(root/'validate_call.bin'),
     'decider_key_bytes_match_b3':True,'local':positive,'submitted':False}
(root/'ready.json').write_text(json.dumps(doc,indent=2,sort_keys=True)+'\n')
print('PASS B4 offline full V3/SDK/native call preparation:',root/'ready.json')
PY
