#!/usr/bin/env bash
# V.9 gate: slice_chain omit (see check-chain.sh).
set -euo pipefail
exec "$(dirname "$0")/check-chain.sh" omit
