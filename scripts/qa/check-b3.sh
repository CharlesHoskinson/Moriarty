#!/usr/bin/env bash
# B3: actual v3 corpus -> transition child -> recursive parent -> terminal D.
# Invoke through doctorwatson check B3 (one shared heavy slot).
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS=4
export RAYON_NUM_THREADS=4
export MALLOC_MMAP_THRESHOLD_=131072
export MALLOC_TRIM_THRESHOLD_=131072
export MALLOC_ARENA_MAX=1
unset SHERLOCK_PROVER_ID
# First projectfenrir checkout that exists: this host (after the 2026-10-26 rebuild), then the old layout.
if [[ -z "${FENRIR_ROOT:-}" ]]; then
  for d in "$HOME/code/projectfenrir" "$HOME/repos/projectfenrir-main"; do [[ -d $d ]] && { FENRIR_ROOT=$d; break; }; done
fi
export FENRIR_ROOT="${FENRIR_ROOT:?no projectfenrir checkout found; set FENRIR_ROOT}"
export FRTF_V3_ROOT="${FRTF_V3_ROOT:-$FENRIR_ROOT/recursion/rollup-test-format/v3}"
export MORIARTY_PARENT_SRS="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
python3 -B scripts/qa/fetch-sherlock-srs.py "$MORIARTY_PARENT_SRS"
python3 -B -m unittest discover -s scripts/qa/tests -p 'test_rollup_*.py'
cargo test --release --locked -p mori-recursion --features sherlock-rollup-prefix \
  --test sherlock_rollup -- --nocapture --test-threads=1
cargo test --release --locked -p mori-recursion --features sherlock-rollup-prefix \
  --test sherlock_rollup_uniform --test sherlock_rollup_trace -- --include-ignored --nocapture --test-threads=1
cargo build --release --locked -p mori-recursion --features sherlock-rollup-prefix --example rollup
mkdir -p target/sherlock-b3
run_dir="$(mktemp -d "$PWD/target/sherlock-b3/run-XXXXXXXX")"
cut="${FSYN_DESIGN_CUT:-$PWD/openspec/changes/sherlock-synthetic-rollup-data/design-cut.json}"
if [[ -z "${FSYN_SOURCE_MAP:-}" ]]; then
  FSYN_SOURCE_MAP="$(python3 -B models/sherlock/rollup-synth/sources.py \
    --cut "$cut" --fenrir-root "$FENRIR_ROOT" --cache "$PWD/target/fsyn-sources")"
fi
export FSYN_SOURCE_MAP
python3 -B models/sherlock/rollup-synth/fsyn.py generate --cut "$cut" \
  --golden-root "$FRTF_V3_ROOT" --out "$run_dir/fixture"
FRTF_V3_ROOT="$run_dir/fixture/corpus"
export FRTF_V3_ROOT
binary="${CARGO_TARGET_DIR:-$PWD/target}/release/examples/rollup"
cp "$binary" "$run_dir/rollup.bin"
binary="$run_dir/rollup.bin"
"$binary" identity
python3 -B - "$binary" "$run_dir/build.json" <<'PY'
import hashlib, json, pathlib, subprocess, sys
binary, output = map(pathlib.Path, sys.argv[1:])
def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()
doc = {'schema': 'sherlock-b3-build/1', 'binary_sha256': sha(binary),
       'cargo_lock_sha256': sha(pathlib.Path('Cargo.lock')), 'features': ['sherlock-rollup-prefix'],
       'adapter_version': 'sherlock-v3-batch/2',
       'source_sha256': {str(path): sha(path) for path in sorted(pathlib.Path('crates/mori-recursion').rglob('*'))
                         if path.is_file() and path.suffix in ('.rs', '.toml', '.json')},
       'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
       'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
       'dirty': bool(subprocess.check_output(['git', 'status', '--porcelain'], text=True)),
       'prover': 'midnight-proofs 0.8.3 registry stock'}
output.write_text(json.dumps(doc, indent=2, sort_keys=True) + '\n')
PY
for stage in mock leaf parent decider verify; do
  python3 -B scripts/qa/measure-rollup.py "$run_dir/$stage.measurement.json" -- \
    "$binary" "$stage" "$FRTF_V3_ROOT" 1 "$run_dir" "$MORIARTY_PARENT_SRS"
done
python3 -B scripts/qa/check-rollup-evidence.py B3 "$run_dir"
