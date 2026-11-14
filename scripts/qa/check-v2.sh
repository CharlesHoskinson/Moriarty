#!/usr/bin/env bash
# V.2 gate: slice mode v2 (see check-slice.sh).
set -euo pipefail
exec "$(dirname "$0")/check-slice.sh" v2
