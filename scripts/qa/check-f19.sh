#!/usr/bin/env bash
# F19: C0 profile harness ready for the GP node chain (task 2.7, RPP task 5.3, SRC-011, RPP-007).
# Code checks only: no proof, no keygen, no SRS. Steps:
#   1. prepare --check-only: vendor archive, patch and file hashes; isolation; the profile
#      workspace pins the root's midnight-proofs/circuits/zk-stdlib/curves and locks them;
#   2. the production-isolation check;
#   3. the profile unit tests (GP workload plan, C11-C14 decide rows, reducer, identity);
#   4. cargo check of tools/profile (profile-gp, profile-a2) with and without the feature,
#      offline and --locked, so a stale tools/profile/Cargo.lock fails.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
[ -f tools/profile/src/bin/profile-gp.rs ] || { echo "F19: tools/profile/src/bin/profile-gp.rs missing"; exit 1; }

# profile-recursion.py needs jsonschema; use the offline uv cache when the system python lacks it.
py=(python3)
if ! python3 -c 'import jsonschema' 2>/dev/null; then
  command -v uv >/dev/null || { echo "F19: python3 has no jsonschema and uv is absent"; exit 1; }
  py=(uv run --offline --no-project -q --with jsonschema python3)
fi

echo "F19: prepare --check-only"
"${py[@]}" -B scripts/qa/profile-recursion.py prepare --check-only
echo "F19: isolation"
"${py[@]}" -B scripts/qa/check-profile-isolation.py
echo "F19: unittest"
"${py[@]}" -B -m unittest discover -s scripts/qa/tests -p 'test_profile_*.py'

target="${F19_TARGET_DIR:-tools/profile/target/check}"
for features in "" "--features recursion-phase-profile"; do
  echo "F19: cargo check --bins ${features:-(feature off)}"
  # shellcheck disable=SC2086
  env RUSTC_WRAPPER= CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}" CARGO_TARGET_DIR="$target" \
    cargo check --offline --locked --quiet --manifest-path tools/profile/Cargo.toml --bins $features
done
echo "F19 PASS: profile harness re-pinned, GP workload and C11-C14 rows tested, profile-gp checks on and off"
