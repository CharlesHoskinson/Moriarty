#!/bin/bash
# T10 Midnight Preview step (owner approval held by the lead; the lead runs this script).
# Deploys the final SherlockLedger with the final wrap key, then one acceptRun of program A and the
# refusals R1 (changed outcome), R2 (flipped proof bit), R4 (program B against A's anchor) and R3 (replay).
# The wallet is OP-PV-WALLET, read in-process by tools/sherlock-sdk/t10a-ledger-run.mjs (T10_NET=preview) as
# preview-session.mjs reads it; no seed, phrase or key is printed or written. Public receipts go to
# research/sherlock-t10-bound-20261114/preview/receipts.json. Needs the final run's outputs under ~/.cache/t10m.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
export T10_HOME=${T10_HOME:-$HOME/.cache/t10m}
export T10_OVM=${T10_OVM:-$HOME/.cache/m7-wrap/src/openvm-t10}
export T10_MOR=${T10_MOR:-$(cd "$here/../.." && pwd)}
exec "$here/chain.sh" preview
