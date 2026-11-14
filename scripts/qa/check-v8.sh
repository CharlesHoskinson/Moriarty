#!/usr/bin/env bash
# V.8 gate: slice mode v8 (see check-slice.sh).
set -euo pipefail
exec "$(dirname "$0")/check-slice.sh" v8
