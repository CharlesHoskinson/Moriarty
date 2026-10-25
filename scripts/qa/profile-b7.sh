#!/usr/bin/env bash
# Run through DoctorWatson heavy. Reuse verified prefixes; change only allocator retention.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export RAYON_NUM_THREADS=4
export MALLOC_MMAP_THRESHOLD_=131072
export MALLOC_TRIM_THRESHOLD_=131072
mkdir -p target/sherlock-b7
run_dir="$(mktemp -d "$PWD/target/sherlock-b7/run-XXXXXXXX")"
python3 -B - "$run_dir" <<'PY'
import hashlib, json, pathlib, shutil, sys
destination = pathlib.Path(sys.argv[1])
candidates = list(pathlib.Path('target/sherlock-b3').glob('run-*/verified.json'))
if not candidates:
    raise SystemExit('B7 profile requires a separately verified B3 run')
source = max(candidates, key=lambda p: p.stat().st_mtime_ns).parent.resolve()
for name in ('mock.measurement.json', 'build.json', 'rollup.bin'):
    shutil.copy2(source / name, destination / name)
for layer in ('leaf', 'parent'):
    for suffix in ('vk', 'proof', 'relation', 'instance.json', 'stats.json', 'measurement.json'):
        shutil.copy2(source / f'{layer}.{suffix}', destination / f'{layer}.{suffix}')
shutil.copytree(source / 'fixture', destination / 'fixture')
reference = destination / 'reference'
reference.mkdir()
for suffix in ('vk', 'proof', 'relation', 'instance.json', 'stats.json', 'measurement.json'):
    shutil.copy2(source / f'decider.{suffix}', reference / f'decider.{suffix}')
shutil.copy2(source / 'verified.json', reference / 'verified.json')
doc = {'schema': 'sherlock-b7-profile/1', 'reference': str(source),
       'reuse': ['mock', 'leaf', 'parent'], 'reproved': ['decider'],
       'hypothesis': 'dynamic allocator thresholds retain temporary model/keygen buffers into proving',
       'allocator': {'MALLOC_MMAP_THRESHOLD_': '131072', 'MALLOC_TRIM_THRESHOLD_': '131072'},
       'unchanged': 'same retained executable, relation bytes, verifier keys, inputs and stock 0.8.3 prover'}
(destination / 'profile.json').write_text(json.dumps(doc, indent=2, sort_keys=True) + '\n')
build = json.loads((destination / 'build.json').read_bytes())
if hashlib.file_digest((destination / 'rollup.bin').open('rb'), 'sha256').hexdigest() != build['binary_sha256']:
    raise SystemExit('retained binary differs from measured B3 build')
PY
binary="$run_dir/rollup.bin"
srs="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
for stage in decider verify; do
  python3 -B scripts/qa/measure-rollup.py "$run_dir/$stage.measurement.json" -- \
    "$binary" "$stage" "$run_dir/fixture/corpus" 1 "$run_dir" "$srs"
done
cmp "$run_dir/decider.vk" "$run_dir/reference/decider.vk"
cmp "$run_dir/decider.relation" "$run_dir/reference/decider.relation"
cmp "$run_dir/decider.instance.json" "$run_dir/reference/decider.instance.json"
python3 -B scripts/qa/check-rollup-evidence.py B7 "$run_dir"
