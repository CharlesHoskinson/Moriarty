#!/usr/bin/env bash
# V.6 gate: slice mode v6 (see check-slice.sh).
set -euo pipefail
exec "$(dirname "$0")/check-slice.sh" v6
