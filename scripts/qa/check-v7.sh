#!/usr/bin/env bash
# V.7 gate: slice mode v7 (see check-slice.sh).
set -euo pipefail
exec "$(dirname "$0")/check-slice.sh" v7
