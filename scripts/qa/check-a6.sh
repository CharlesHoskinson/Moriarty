#!/usr/bin/env bash
# Native ledger-9 contract rehearsal; fee/DUST checks are retained as a separate profile.
# Run through doctorwatson check A6, which owns the shared heavy slot and budget.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
mapfile -t artifacts < <(python3 - <<'PY'
from pathlib import Path
for pattern in ['target/sherlock-a3/run-*/artifacts/parent/w.proof',
                'target/sherlock-a4/run-*/artifacts/key/acceptValidity.verifier']:
    files = list(Path('.').glob(pattern))
    if not files:
        raise SystemExit('A6 requires completed public artifacts: ' + pattern)
    print(max(files, key=lambda p: p.stat().st_mtime_ns).parent.resolve())
PY
)
[[ ${#artifacts[@]} == 2 ]] || exit 1
mkdir -p target/sherlock-a6
run_dir="$(mktemp -d "$PWD/target/sherlock-a6/run-XXXXXXXX")"
python3 scripts/qa/fetch-sherlock-contract.py "$run_dir/contract-state.bin"
cargo build --release --locked -p mori-recursion --features sherlock-ledger-validation \
  --example validate_call
sdk="${MORIARTY_SDK_ROOT:-$HOME/.midnight/stagenet-wallet}"
build="${MORIARTY_ANCHOR_BUILD:-$PWD/target/a4-anchor/deploy-run3/build}"
keyed="${MORIARTY_ANCHOR_KEYED_BUILD:-$PWD/target/a4-anchor/keyed}"
node tools/sherlock-sdk/check-local-call.mjs "$sdk" "$build" "$keyed" \
  "$run_dir/contract-state.bin" "${artifacts[0]}" "${artifacts[1]}" "$run_dir/negative" --reject-offline-proof
node tools/sherlock-sdk/check-local-call.mjs "$sdk" "$build" "$keyed" \
  "$run_dir/contract-state.bin" "${artifacts[0]}" "${artifacts[1]}" "$run_dir/positive" --prove
python3 - "$run_dir" <<'PY'
import hashlib, json, sys
from pathlib import Path
root = Path(sys.argv[1])
pos = json.loads((root / 'positive/ledger-result.json').read_text())
neg = json.loads((root / 'negative/negative-ledger-result.json').read_text())['native']
for key in ['contractProofAccepted', 'applied', 'validityStatementPresent', 'rootHeightBatchUnchanged']:
    assert pos[key] is True, key
assert pos['proofVerification'] == neg['proofVerification'] == 'native/proof-verifying'
assert neg['contractProofsSignaturesLimitsAccepted'] is False and neg['applied'] is False
assert 'proof' in neg['contractProfileFailure'].lower()
pins = json.loads(Path('crates/mori-recursion/tests/fixtures/stagenet-verifier.json').read_text())
receipt = dict(schema='sherlock-a6-local-rehearsal/1', sourceContract=pins['address'],
               installedStateSHA256=hashlib.sha256((root/'contract-state.bin').read_bytes()).hexdigest(),
               positive=pos, negative=neg, submitted=False)
(root / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print('PASS A6: actual-bound D accepted and applied by native ledger; complete SDK/native post-state matches; reused-proof negative refused')
print('Retained all-on fee/DUST outcome:', pos['allOn'], pos['allOnFailure'])
print('Public receipt:', root / 'receipt.json')
PY
