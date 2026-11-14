#!/usr/bin/env bash
# V.3 gate: slice mode v3 (see check-slice.sh).
set -euo pipefail
exec "$(dirname "$0")/check-slice.sh" v3
