#!/usr/bin/env bash
# V.10 gate: slice_chain chain and D3 (see check-chain.sh).
set -euo pipefail
exec "$(dirname "$0")/check-chain.sh" chain
