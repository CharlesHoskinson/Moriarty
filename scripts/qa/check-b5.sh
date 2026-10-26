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
import importlib.util, json, os, pathlib, shutil, sys
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
if os.environ.get('SHERLOCK_PREFIX_PROVER_BUILD'):
    declaration = pathlib.Path(os.environ['SHERLOCK_PREFIX_PROVER_BUILD'])
    prover = json.loads(declaration.read_bytes())
    if prover['schema'] != 'sherlock-loki-build/1':
        raise SystemExit('requires a stamped isolated Loki build')
    evidence.require_digest(pathlib.Path(prover['binary']), prover['binary_sha256'])
    shutil.copy2(declaration, out / 'prover-build.json')
    shutil.copy2(prover['binary'], out / 'prover.bin')
    spec = importlib.util.spec_from_file_location('prefix', 'scripts/qa/check-prefix-evidence.py')
    prefix = importlib.util.module_from_spec(spec); spec.loader.exec_module(prefix)
    prefix.proving_build(out, build)
(out / 'workload.json').write_text(json.dumps({'schema': 'sherlock-b5-workload/1',
    'prefixes': [2, 4, 6], 'transactions_per_batch': 16, 'headers_per_batch': 50,
    'samples_per_prefix': 1, 'proof_reuse_between_prefixes': False,
    'status': 'running', 'reference_b3': str(source)}, indent=2, sort_keys=True) + '\n')
PY
binary="$run_dir/rollup.bin"
if [[ -f "$run_dir/prover.bin" ]]; then binary="$run_dir/prover.bin"; fi
srs="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
for count in 2 4 6; do
  prefix="$run_dir/prefix-$count"
  mkdir "$prefix"
  for stage in leaf parent decider verify; do
    selected="$binary"
    # Loki is never the reference verifier, including for Loki-created proofs.
    if [[ "$stage" == verify ]]; then selected="$run_dir/rollup.bin"; fi
    python3 -B scripts/qa/measure-rollup.py "$prefix/$stage.measurement.json" -- \
      "$selected" "prefix-$stage" "$run_dir/fixture/corpus" "$count" "$prefix" "$srs"
    python3 -B - "$prefix/$stage.measurement.json" <<'PY'
import importlib.util, sys
spec = importlib.util.spec_from_file_location('prefix', 'scripts/qa/check-prefix-evidence.py')
prefix = importlib.util.module_from_spec(spec); spec.loader.exec_module(prefix)
prefix.check_resources(prefix.evidence.read(sys.argv[1], 'sherlock-rollup-measurement/1'))
PY
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
