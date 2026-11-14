#!/usr/bin/env bash
# V.5 gate: slice mode v5 (see check-slice.sh).
set -euo pipefail
exec "$(dirname "$0")/check-slice.sh" v5
