#!/usr/bin/env bash
# Three independently executed prefixes. Each stage retains one PK and proves
# every batch in that prefix; no previous prefix's proof is reused.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export RAYON_NUM_THREADS=4
export MALLOC_MMAP_THRESHOLD_=131072 MALLOC_TRIM_THRESHOLD_=131072 MALLOC_ARENA_MAX=1
mkdir -p target/sherlock-b5
run_dir="$(mktemp -d "$PWD/target/sherlock-b5/run-XXXXXXXX")"
python3 -B - "$run_dir" <<'PY'
import importlib.util, json, pathlib, shutil, sys
spec = importlib.util.spec_from_file_location('evidence', 'scripts/qa/check-rollup-evidence.py')
evidence = importlib.util.module_from_spec(spec)
spec.loader.exec_module(evidence)
source = evidence.latest_run('B3').resolve()
evidence.check('B3', source)
build = json.loads((source / 'build.json').read_bytes())
if build['adapter_version'] != 'sherlock-v3-batch/2' or (source / 'leaf.relation').read_bytes() != b'FSYNB002':
    raise SystemExit('B5 needs the stock-verified fixed-key B3 v2 baseline')
out = pathlib.Path(sys.argv[1])
for name in ('build.json', 'rollup.bin'):
    shutil.copy2(source / name, out / name)
shutil.copytree(source / 'fixture', out / 'fixture')
(out / 'workload.json').write_text(json.dumps({'schema': 'sherlock-b5-workload/1',
    'prefixes': [2, 4, 6], 'transactions_per_batch': 16, 'headers_per_batch': 50,
    'samples_per_prefix': 1, 'proof_reuse_between_prefixes': False,
    'status': 'running', 'reference_b3': str(source)}, indent=2, sort_keys=True) + '\n')
PY
binary="$run_dir/rollup.bin"
srs="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
for count in 2 4 6; do
  prefix="$run_dir/prefix-$count"
  mkdir "$prefix"
  for stage in leaf parent decider verify; do
    python3 -B scripts/qa/measure-rollup.py "$prefix/$stage.measurement.json" -- \
      "$binary" "prefix-$stage" "$run_dir/fixture/corpus" "$count" "$prefix" "$srs"
  done
done
python3 -B - "$run_dir/workload.json" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
doc = json.loads(path.read_bytes())
doc['status'] = 'completed'
path.write_text(json.dumps(doc, indent=2, sort_keys=True) + '\n')
PY
python3 -B scripts/qa/check-prefix-evidence.py "$run_dir"
