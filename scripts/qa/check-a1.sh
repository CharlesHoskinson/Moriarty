#!/usr/bin/env bash
# A1: native backend (mori-backend + pinned midnight crates) builds on the current source cut under the locked dependency set.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-6}"
exec cargo build --locked -p mori-backend
