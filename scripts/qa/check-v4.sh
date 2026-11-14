#!/usr/bin/env bash
# V.4 gate: slice mode v4 (see check-slice.sh).
set -euo pipefail
exec "$(dirname "$0")/check-slice.sh" v4
