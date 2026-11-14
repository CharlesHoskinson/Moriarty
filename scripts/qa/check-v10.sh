#!/usr/bin/env bash
# V.10 gate: t7_chain chain A and B with D3, then the T7 binding cases (see check-chain.sh).
set -euo pipefail
"$(dirname "$0")/check-chain.sh" chain
exec "$(dirname "$0")/check-chain.sh" cases
