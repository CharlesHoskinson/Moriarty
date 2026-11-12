#!/bin/bash
# usage: run.sh <stage> <mem> <dir> cmd...   (logs to ~/.cache/e2-lfr/logs/<stage>.log)
set -uo pipefail
N=$1; M=$2; D=$3; shift 3
export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=$HOME/.cache/e2-lfr/target RUSTFLAGS="-C target-cpu=native"
L=$HOME/.cache/cost-opt/measure/heavy.lock; HR=/home/hoskinson/.local/bin/heavy-run
cd $D
echo "# $(date -Is) $N start (waiting for lock) head $(git rev-parse --short HEAD) load $(cat /proc/loadavg)" | tee -a $HOME/.cache/e2-lfr/logs/runs.log
flock $L $HR --mem $M --name e2-$N -- "$@" > $HOME/.cache/e2-lfr/logs/$N.log 2>&1
rc=$?
echo "# $(date -Is) $N end rc=$rc load $(cat /proc/loadavg)" | tee -a $HOME/.cache/e2-lfr/logs/runs.log
exit $rc
